# aware_sketchup_bridge/core.rb — the in-process TCP bridge for AWARE.
#
# Spec lives in docs/superpowers/specs/2026-05-19-v034-sketchup-exec-design.md,
# amended by docs/superpowers/specs/2026-07-30-sketchup-bridge-main-thread-pump.md.
#
# Architecture
# ============
# EVERYTHING runs on SketchUp's main thread, inside one repeating UI timer, using
# non-blocking socket I/O. There are no background threads.
#
# - UI.start_timer(0.05, true) { pump } is the whole engine. Each tick accepts
#   pending connections, drains whatever bytes are readable, evaluates at most
#   one complete request against the live document, and flushes whatever the
#   kernel will take of the reply. Anything unfinished resumes on the next tick,
#   so no socket call can ever block SketchUp's UI.
# - Why not threads: SketchUp hands the Ruby GVL to a BACKGROUND thread only
#   about four times a second (measured: a background `sleep 0.01` loop resumes
#   after ~250 ms). The v0.34 design crossed thread boundaries 5-7 times per
#   request — accept thread, per-connection worker, main-thread drain, back to
#   the worker to write and close — so a trivial `1+1` took 5-7 seconds, latency
#   grew as worker and watchdog threads piled up competing for the same scarce
#   handoff, and requests eventually always outran the client's patience while
#   unserviced connections stacked up in CLOSE_WAIT (aware-aeco/aware#330). The
#   main thread is the one thread SketchUp schedules reliably, and it is the only
#   thread allowed to touch the model, so it was already on the critical path.
# - Discovery: on bind, the bridge writes %TEMP%/aware-sketchup/<pid>.json (via a
#   temp file + rename, so a reader never sees a half-written file) so the
#   sidecar can find us. We delete it at process exit (at_exit).
#
# Wire protocol (request)
# -----------------------
#   <4-byte big-endian length><utf8-json-body>
# Body: {"type":"exec","code":"<ruby>","args":{<hash>}}
#
# Wire protocol (response)
# ------------------------
#   <4-byte big-endian length><utf8-json-body>
# Body: {"ok":true,"result":<json>,"stdout_log":"<captured>"}
#    or {"ok":false,"error":"<msg>","stack":"<backtrace>","stdout_log":"<captured>"}
#
# One request per connection: the bridge answers, then closes. A peer that
# disconnects before its reply is written is dropped WITHOUT evaluating — a
# caller that has already given up must not get a surprise mutation it will
# never see a receipt for.

require 'socket'
require 'json'
require 'fileutils'
require 'tmpdir'
require 'stringio'
require 'time'

module AwareSketchupBridge
  module Core
    PORT_MIN = 8765
    PORT_MAX = 8865
    PUMP_INTERVAL_SECONDS = 0.05

    # Protocol/resource ceilings. The bridge only ever talks to a local sidecar,
    # but the pump runs on SketchUp's UI thread, so every loop it runs is bounded.
    MAX_FRAME_BYTES        = 64 * 1024 * 1024
    MAX_LIVE_CONNECTIONS   = 32
    MAX_ACCEPTS_PER_TICK   = 8
    READ_CHUNK_BYTES       = 256 * 1024
    READ_BUDGET_PER_TICK   = 8 * 1024 * 1024
    # A connection that makes no progress at all for this long is abandoned.
    CONNECTION_IDLE_LIMIT_SECONDS = 300

    # One in-flight connection. `outbuf` being non-nil means the request has been
    # evaluated and we are flushing the reply; until then we are still reading.
    Conn = Struct.new(:sock, :inbuf, :need, :outbuf, :touched_at, :id)

    @started   = false
    @port      = nil
    @server    = nil
    @conns     = []
    @timer_id  = nil
    @pumping   = false
    @evaluated_this_tick = false
    @shutdown  = false
    @served    = 0
    @accepted  = 0

    class << self
      attr_reader :port, :started

      # Entry point called by SketchupExtension's loader. SketchUp invokes this
      # once at startup; subsequent re-loads of the .rb file are no-ops.
      def start!
        return if @started

        write_diag("start! called at #{Time.now.iso8601} pid=#{Process.pid}")

        @conns = []
        @port, @server = bind_first_free_port
        if @server.nil?
          write_diag("bind failed in #{PORT_MIN}..#{PORT_MAX}")
          warn "[aware-sketchup] could not bind any port in #{PORT_MIN}..#{PORT_MAX}; bridge disabled"
          return
        end
        write_diag("bound port=#{@port}")

        # The whole engine. UI.start_timer's block runs on the main UI thread —
        # safe to touch Sketchup.active_model from inside it.
        @timer_id = UI.start_timer(PUMP_INTERVAL_SECONDS, true) { pump }
        write_diag("pump timer started (id=#{@timer_id.inspect})")

        write_discovery_file(@port)
        write_diag("discovery file written to #{discovery_file_path}")
        at_exit { shutdown! }

        @started = true
        log "started on 127.0.0.1:#{@port} (pid #{Process.pid})"
      rescue StandardError => e
        write_diag("startup CRASHED: #{e.class}: #{e.message}\n#{(e.backtrace || []).join("\n")}")
        warn "[aware-sketchup] startup failed: #{e.class}: #{e.message}"
        warn e.backtrace.join("\n") if e.backtrace
        # Never leave a half-started bridge holding a port or advertising itself.
        shutdown!
      end

      # Idempotent teardown: stop the pump, close everything, unadvertise.
      def shutdown!
        return if @shutdown

        @shutdown = true
        begin
          UI.stop_timer(@timer_id) if @timer_id && defined?(UI) && UI.respond_to?(:stop_timer)
        rescue StandardError
          # Nothing useful to do while the host is going down.
        end
        @timer_id = nil
        (@conns || []).each { |c| close_socket(c.sock) }
        @conns = []
        begin
          @server.close if @server && !@server.closed?
        rescue StandardError
          # best-effort
        end
        @server = nil
        delete_discovery_file
        @started = false
      end

      # Diagnostic: writes one line to %TEMP%\aware-sketchup\boot-<pid>.log
      # regardless of outcome. Helps diagnose silent failures — the readiness
      # probe (`list-instances`) only reads the discovery file, so this log is
      # the only place that can show whether requests are actually being served.
      def write_diag(msg)
        dir = discovery_dir
        FileUtils.mkdir_p(dir) rescue nil
        path = File.join(dir, "boot-#{Process.pid}.log")
        File.open(path, 'a') { |f| f.puts("[#{Time.now.iso8601}] #{msg}") }
      rescue StandardError
        # never let logging take us down
      end

      # --- socket binding -----------------------------------------------------

      def bind_first_free_port
        (PORT_MIN..PORT_MAX).each do |p|
          begin
            s = TCPServer.new('127.0.0.1', p)
            return [p, s]
          rescue Errno::EADDRINUSE, Errno::EACCES
            next
          end
        end
        [nil, nil]
      end

      # --- the pump (main thread, once per timer tick) ------------------------

      # SketchUp pumps its own message loop during long model operations, so this
      # timer can fire again while a bake is still running inside it. Without the
      # guard the same buffered frame could be serviced twice — a duplicated bake.
      #
      # The guard is cleared ONLY by the invocation that set it: the early return
      # deliberately sits outside the begin/ensure, because a nested tick running
      # the ensure would drop the guard while the outer tick is still inside
      # service_connections (mutating @conns during its own delete_if).
      def pump
        return if @pumping

        @pumping = true
        begin
          @evaluated_this_tick = false
          now = monotonic
          accept_pending(now)
          service_connections(now)
        rescue StandardError => e
          write_diag("pump error: #{e.class}: #{e.message}")
          warn "[aware-sketchup] pump error: #{e.class}: #{e.message}"
        ensure
          @pumping = false
        end
      end

      def accept_pending(now)
        MAX_ACCEPTS_PER_TICK.times do
          sock = @server.accept_nonblock(exception: false)
          break if sock.nil? || sock == :wait_readable

          if @conns.size >= MAX_LIVE_CONNECTIONS
            # Refuse rather than grow without bound; the sidecar retries.
            write_diag("refused connection: #{@conns.size} already live")
            close_socket(sock)
            next
          end

          @accepted += 1
          sock.sync = true if sock.respond_to?(:sync=)
          @conns << Conn.new(sock, String.new, nil, nil, now, @accepted)
        end
      rescue StandardError => e
        write_diag("accept error: #{e.class}: #{e.message}")
      end

      def service_connections(now)
        @conns.delete_if do |c|
          begin
            service(c, now)
          rescue StandardError => e
            write_diag("conn ##{c.id} dropped: #{e.class}: #{e.message}")
            close_socket(c.sock)
            true
          end
        end
      end

      # One connection, one tick. Returns true when the connection is finished
      # (socket closed) and should be forgotten.
      def service(c, now)
        # Covers both phases: a peer that never finishes sending and one that
        # never reads its reply both eventually get dropped.
        if now - c.touched_at > CONNECTION_IDLE_LIMIT_SECONDS
          write_diag("conn ##{c.id} made no progress for #{CONNECTION_IDLE_LIMIT_SECONDS}s; dropping")
          close_socket(c.sock)
          return true
        end

        return flush(c, now) unless c.outbuf.nil?

        if drain_reads(c, now) == :closed
          # Peer went away mid-request: nothing to answer, nothing to run.
          write_diag("conn ##{c.id} closed by peer before a reply was due")
          close_socket(c.sock)
          return true
        end

        # At most one user script per tick: a burst of queued requests must not
        # chain several evaluations into a single UI callback. Checked BEFORE
        # take_frame so the frame stays buffered for the next tick.
        return false if @evaluated_this_tick

        frame = take_frame(c)
        return false if frame.nil?

        @evaluated_this_tick = true
        started_at = monotonic
        response   = process_frame(frame)
        c.outbuf   = encode_frame(response)
        c.touched_at = monotonic
        @served += 1
        write_diag(format('served #%d conn #%d in=%dB out=%dB %.0fms',
                          @served, c.id, frame.bytesize, c.outbuf.bytesize,
                          (monotonic - started_at) * 1000))
        flush(c, monotonic)
      end

      # Pulls whatever the kernel already has. Returns :closed on EOF, :wait
      # otherwise. Bounded so a huge payload can't monopolise one UI tick.
      def drain_reads(c, now)
        budget = READ_BUDGET_PER_TICK
        while budget.positive?
          chunk = c.sock.read_nonblock(READ_CHUNK_BYTES, exception: false)
          return :wait   if chunk == :wait_readable
          return :closed if chunk.nil?

          c.inbuf << chunk
          c.touched_at = now
          budget -= chunk.bytesize
        end
        :wait
      end

      # Returns a complete request body, or nil while one is still arriving.
      def take_frame(c)
        if c.need.nil?
          return nil if c.inbuf.bytesize < 4

          len = c.inbuf.byteslice(0, 4).unpack1('N')
          raise "absurd frame length: #{len}" if len.nil? || len > MAX_FRAME_BYTES

          c.need  = len
          c.inbuf = c.inbuf.byteslice(4, c.inbuf.bytesize - 4)
        end
        return nil if c.inbuf.bytesize < c.need

        body    = c.inbuf.byteslice(0, c.need)
        c.inbuf = c.inbuf.byteslice(c.need, c.inbuf.bytesize - c.need)
        c.need  = nil
        body
      end

      # Hands the reply to the kernel a piece at a time. Returns true once the
      # whole reply is out and the socket is closed, false to resume next tick.
      def flush(c, now)
        until c.outbuf.empty?
          written = c.sock.write_nonblock(c.outbuf, exception: false)
          return false if written == :wait_writable
          raise "write_nonblock returned #{written.inspect}" unless written.is_a?(Integer) && written.positive?

          c.outbuf = c.outbuf.byteslice(written, c.outbuf.bytesize - written)
          c.touched_at = now
        end
        close_socket(c.sock)
        true
      end

      # --- request handling (main thread) ------------------------------------

      def process_frame(body)
        req = JSON.parse(body)
        req = {} unless req.is_a?(Hash)
        case req['type']
        when 'exec'
          run_user_script(req['code'].to_s, req['args'].is_a?(Hash) ? req['args'] : {})
        when 'ping'
          { 'ok' => true, 'result' => 'pong', 'stdout_log' => '' }
        else
          {
            'ok'    => false,
            'error' => "unknown request type: #{req['type'].inspect}",
            'stack' => '',
            'stdout_log' => '',
          }
        end
      rescue JSON::ParserError => e
        { 'ok' => false, 'error' => "malformed request JSON: #{e.message}", 'stack' => '', 'stdout_log' => '' }
      rescue StandardError => e
        {
          'ok' => false,
          'error' => "bridge failed handling request: #{e.class}: #{e.message}",
          'stack' => (e.backtrace || []).first(50).join("\n"),
          'stdout_log' => '',
        }
      end

      # Length-prefixed reply bytes. Falls back to a minimal error frame if the
      # response itself can't be serialized, so the caller always gets an answer.
      def encode_frame(response)
        body = response.to_json.encode('utf-8').b
        [body.bytesize].pack('N') + body
      rescue StandardError => e
        fallback = { 'ok' => false, 'error' => "bridge could not encode its response: #{e.class}: #{e.message}",
                     'stack' => '', 'stdout_log' => '' }.to_json.b
        [fallback.bytesize].pack('N') + fallback
      end

      # --- user-script wrapper ----------------------------------------------

      def run_user_script(code, args)
        # Capture stdout so `puts` in user code goes into the receipt rather
        # than SketchUp's Ruby Console.
        captured = String.new
        original_stdout = $stdout

        # Evaluate the user's code in a FRESH binding obtained from a sandbox
        # object so user code can't accidentally rebind our outer locals
        # (`result`, `error`, `captured`, etc.) via name reassignment. The
        # sandbox is a plain Object with no methods of ours exposed.
        sandbox_binding = Object.new.instance_eval { binding }

        outcome = { ok: nil, value: nil, error: nil, stack: '' }
        begin
          $stdout = StringIO.new(captured)
          # Wrap user code in a lambda so `return` short-circuits the lambda
          # rather than the bridge. The lambda's own `args` parameter shadows
          # any incidental references in surrounding scope.
          fn = ::Kernel.send(:eval, "->(args) { #{code}\n}", sandbox_binding, '(aware-exec)', 1)
          value = fn.call(args)
          outcome[:ok] = true
          outcome[:value] = value
        rescue Exception => e   # must catch SystemExit so user code can't kill SketchUp
          outcome[:ok] = false
          outcome[:error] = "#{e.class}: #{e.message}"
          outcome[:stack] = (e.backtrace || []).first(50).join("\n")
        ensure
          $stdout = original_stdout
        end

        if outcome[:ok]
          { 'ok' => true, 'result' => encode_json_safe(outcome[:value]), 'stdout_log' => captured }
        else
          { 'ok' => false, 'error' => outcome[:error], 'stack' => outcome[:stack], 'stdout_log' => captured }
        end
      end

      # Convert SketchUp-specific types to JSON-friendly Ruby primitives.
      # Defensive - anything we don't recognise falls back to .to_s.
      # Recursion-depth-limited so circular references can't blow the stack.
      def encode_json_safe(obj, depth = 0)
        return '(max-depth)' if depth > 8

        case obj
        when nil, true, false, Integer, Float, String
          obj
        when Symbol
          obj.to_s
        when Hash
          obj.each_with_object({}) { |(k, v), h| h[k.to_s] = encode_json_safe(v, depth + 1) }
        when Array
          obj.map { |v| encode_json_safe(v, depth + 1) }
        else
          encode_typed(obj, depth)
        end
      rescue StandardError => e
        "(encode-error #{e.class}: #{e.message})"
      end

      # Branch for Geom::* / Length / Sketchup::* — kept out of `case` because
      # those constants may not exist in stub environments (tests).
      def encode_typed(obj, depth)
        klass = obj.class.to_s

        if defined?(Geom::Point3d)     && obj.is_a?(Geom::Point3d)
          return [obj.x.to_f, obj.y.to_f, obj.z.to_f]
        end
        if defined?(Geom::Vector3d)    && obj.is_a?(Geom::Vector3d)
          return [obj.x.to_f, obj.y.to_f, obj.z.to_f]
        end
        if defined?(Geom::Point2d)     && obj.is_a?(Geom::Point2d)
          return [obj.x.to_f, obj.y.to_f]
        end
        if defined?(Geom::Vector2d)    && obj.is_a?(Geom::Vector2d)
          return [obj.x.to_f, obj.y.to_f]
        end
        if defined?(Geom::BoundingBox) && obj.is_a?(Geom::BoundingBox)
          return {
            'min'   => encode_json_safe(obj.min, depth + 1),
            'max'   => encode_json_safe(obj.max, depth + 1),
            'valid' => obj.valid?,
          }
        end
        if defined?(Geom::Transformation) && obj.is_a?(Geom::Transformation)
          return obj.to_a.map(&:to_f)
        end
        if defined?(Length) && obj.is_a?(Length)
          return obj.to_f
        end

        if klass.start_with?('Sketchup::')
          h = { '_type' => klass }
          h['persistent_id'] = obj.persistent_id.to_s if obj.respond_to?(:persistent_id)
          h['entityID']      = obj.entityID            if obj.respond_to?(:entityID)
          if obj.respond_to?(:name) && !obj.is_a?(Hash)
            begin
              h['name'] = obj.name
            rescue StandardError
              # Some entities throw on .name when deleted; skip.
            end
          end
          return h
        end

        obj.to_s
      end

      # --- discovery file ----------------------------------------------------

      def write_discovery_file(port)
        dir = discovery_dir
        FileUtils.mkdir_p(dir)
        payload = {
          'pid'            => Process.pid,
          'port'           => port,
          'version'        => sketchup_version,
          'bridge_version' => bridge_version,
          'model_path'     => safe_active_model_path,
          'started_at'     => Time.now.utc.iso8601,
        }
        # Write-then-rename: the sidecar DELETES any discovery file it fails to
        # parse, so it must never be able to catch this one half-written.
        tmp = "#{discovery_file_path}.tmp-#{Process.pid}"
        File.open(tmp, 'w') { |f| f.write(JSON.pretty_generate(payload)) }
        File.rename(tmp, discovery_file_path)
      end

      def delete_discovery_file
        File.delete(discovery_file_path) if File.exist?(discovery_file_path)
      rescue StandardError
        # best-effort
      end

      def discovery_dir
        override = ENV['AWARE_SKETCHUP_DISCOVERY_DIR']
        return override if override && !override.empty?
        File.join(Dir.tmpdir, 'aware-sketchup')
      end

      def discovery_file_path
        File.join(discovery_dir, "#{Process.pid}.json")
      end

      # Robust against SketchUp versions that move the version constant around.
      def sketchup_version
        v = Sketchup.version rescue nil
        v.to_s.empty? ? 'unknown' : v.to_s
      end

      def safe_active_model_path
        m = Sketchup.active_model rescue nil
        return nil if m.nil?
        path = m.path rescue ''
        path.to_s.empty? ? nil : path.to_s
      end

      # The loader defines BRIDGE_VERSION; report it so the sidecar can tell a
      # RUNNING bridge from the one installed on disk (a session keeps whatever
      # it loaded at startup until SketchUp restarts).
      def bridge_version
        defined?(AwareSketchupBridge::BRIDGE_VERSION) ? AwareSketchupBridge::BRIDGE_VERSION.to_s : 'unknown'
      end

      # --- small utilities ---------------------------------------------------

      def monotonic
        Process.clock_gettime(Process::CLOCK_MONOTONIC)
      end

      def close_socket(sock)
        return if sock.nil?

        # Drain anything still unread: closing a socket with unread inbound data
        # can produce an abortive reset on Windows, which would eat the reply we
        # just wrote.
        begin
          8.times do
            chunk = sock.read_nonblock(READ_CHUNK_BYTES, exception: false)
            break if chunk.nil? || chunk == :wait_readable
          end
        rescue StandardError
          # already broken; closing is all that's left
        end
        sock.close
      rescue StandardError
        # best-effort
      end

      def log(msg)
        puts "[aware-sketchup] #{msg}"
      rescue StandardError
        # never let logging take us down
      end
    end
  end

  # Kick off - runs once when the extension loads.
  Core.start!
end
