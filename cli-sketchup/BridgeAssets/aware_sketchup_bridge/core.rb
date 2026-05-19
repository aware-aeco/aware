# aware_sketchup_bridge/core.rb — the in-process TCP bridge for AWARE.
#
# Spec lives in docs/superpowers/specs/2026-05-19-v034-sketchup-exec-design.md.
#
# Architecture
# ============
# - Background accept thread binds 127.0.0.1:<first-free-port-in-8765..8865>,
#   reads length-prefixed JSON request frames, enqueues them onto a thread-
#   safe Queue.
# - A UI.start_timer(0.05, true) tick on the SketchUp main thread drains the
#   queue, evaluates each request against the live document, encodes the
#   result (or error) as JSON, sends it back to the originating socket.
# - Why the timer: SketchUp's Ruby API is single-threaded with respect to the
#   document; calls into Sketchup.active_model.entities.* from a background
#   thread crash the host. UI.start_timer is Trimble's canonical pattern for
#   marshaling work onto the main thread.
# - Discovery: on bind, the bridge writes %TEMP%/aware-sketchup/<pid>.json so
#   the sidecar can find us. We delete the file at process exit (at_exit).
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
    DRAIN_INTERVAL_SECONDS = 0.05

    @started   = false
    @port      = nil
    @server    = nil
    @accept_th = nil
    @inbox     = nil

    class << self
      attr_reader :port, :started

      # Entry point called by SketchupExtension's loader. SketchUp invokes this
      # once at startup; subsequent re-loads of the .rb file are no-ops.
      def start!
        return if @started

        write_diag("start! called at #{Time.now.iso8601} pid=#{Process.pid}")

        @inbox = Queue.new
        @port, @server = bind_first_free_port
        if @server.nil?
          write_diag("bind failed in #{PORT_MIN}..#{PORT_MAX}")
          warn "[aware-sketchup] could not bind any port in #{PORT_MIN}..#{PORT_MAX}; bridge disabled"
          return
        end
        write_diag("bound port=#{@port}")

        @accept_th = Thread.new { accept_loop(@server, @inbox) }
        @accept_th.name = 'aware-sketchup-accept' if @accept_th.respond_to?(:name=)
        write_diag("accept_th started")

        # Main-thread drain pump. UI.start_timer's block runs on the main UI
        # thread - safe to touch Sketchup.active_model from inside it.
        UI.start_timer(DRAIN_INTERVAL_SECONDS, true) { drain_inbox }
        write_diag("UI.start_timer set up")

        write_discovery_file(@port)
        write_diag("discovery file written to #{discovery_file_path}")
        at_exit { delete_discovery_file }

        @started = true
        log "started on 127.0.0.1:#{@port} (pid #{Process.pid})"
      rescue StandardError => e
        write_diag("startup CRASHED: #{e.class}: #{e.message}\n#{(e.backtrace || []).join("\n")}")
        warn "[aware-sketchup] startup failed: #{e.class}: #{e.message}"
        warn e.backtrace.join("\n") if e.backtrace
      end

      # Diagnostic: writes one line to %TEMP%\aware-sketchup\boot-<pid>.log
      # regardless of outcome. Helps diagnose silent failures.
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

      # --- accept loop (runs on background thread) ---------------------------

      def accept_loop(server, inbox)
        loop do
          conn = server.accept
          # One worker thread per connection. Reads the request, enqueues
          # {conn, request, response_q}, then blocks on response_q for the
          # main thread to drop a result in.
          Thread.new(conn) do |sock|
            handle_one(sock, inbox)
          end
        end
      rescue IOError, Errno::EBADF
        # Server socket closed - graceful exit.
      rescue StandardError => e
        warn "[aware-sketchup] accept loop crashed: #{e.class}: #{e.message}"
      end

      def handle_one(sock, inbox)
        begin
          request = read_frame(sock)
        rescue StandardError => e
          warn "[aware-sketchup] read_frame failed: #{e.message}"
          sock.close rescue nil
          return
        end

        response_q = Queue.new
        inbox << { conn: sock, request: request, response_q: response_q }

        # Block until the main thread fulfils the response. 90s ceiling so we
        # don't leak threads forever if the user code somehow never returns.
        watchdog = Thread.new do
          sleep(90)
          begin
            response_q << { 'ok' => false, 'error' => 'main-thread eval timed out after 90s', 'stack' => '', 'stdout_log' => '' }
          rescue ClosedQueueError
            # Queue may have been closed/drained already.
          end
        end
        result = response_q.pop
        watchdog.kill rescue nil
        send_frame(sock, result.to_json)
      rescue StandardError => e
        warn "[aware-sketchup] handle_one error: #{e.class}: #{e.message}"
      ensure
        sock.close rescue nil
      end

      # --- frame I/O (length-prefixed JSON) ----------------------------------

      def read_frame(sock)
        header = read_exactly(sock, 4)
        len    = header.unpack1('N')
        raise 'absurd length' if len.negative? || len > 64 * 1024 * 1024
        body = read_exactly(sock, len)
        JSON.parse(body)
      end

      def send_frame(sock, json_str)
        body = json_str.encode('utf-8')
        header = [body.bytesize].pack('N')
        sock.write(header)
        sock.write(body)
        sock.flush
      end

      def read_exactly(sock, count)
        buf = String.new(capacity: count)
        while buf.bytesize < count
          chunk = sock.read(count - buf.bytesize)
          raise EOFError, "stream closed after #{buf.bytesize}/#{count}" if chunk.nil?
          buf << chunk
        end
        buf
      end

      # --- main-thread drain -------------------------------------------------

      def drain_inbox
        # Pull every pending request and process inline on the main thread.
        until @inbox.empty?
          begin
            msg = @inbox.pop(true)  # non-blocking
          rescue ThreadError
            break  # queue drained between empty? and pop
          end
          process_one(msg)
        end
      rescue StandardError => e
        warn "[aware-sketchup] drain crashed: #{e.class}: #{e.message}"
      end

      def process_one(msg)
        req = msg[:request] || {}
        case req['type']
        when 'exec'
          response = run_user_script(req['code'].to_s, req['args'] || {})
        when 'ping'
          response = { 'ok' => true, 'result' => 'pong', 'stdout_log' => '' }
        else
          response = {
            'ok'    => false,
            'error' => "unknown request type: #{req['type'].inspect}",
            'stack' => '',
            'stdout_log' => '',
          }
        end
        msg[:response_q] << response
      rescue StandardError => e
        msg[:response_q] << {
          'ok' => false,
          'error' => "process_one crashed: #{e.class}: #{e.message}",
          'stack' => (e.backtrace || []).join("\n"),
          'stdout_log' => '',
        }
      end

      # --- user-script wrapper ----------------------------------------------

      def run_user_script(code, args)
        captured = String.new
        result = nil
        error  = nil
        backtrace = ''

        original_stdout = $stdout
        begin
          $stdout = StringIO.new(captured)
          # Wrap in a lambda so the user can use `return` to short-circuit
          # without taking down the bridge.
          fn = ::Kernel.send(:eval, "->(args) { #{code}\n}", binding, '(aware-exec)', 1)
          result = fn.call(args)
        rescue Exception => e   # must catch SystemExit so user code can't kill SketchUp
          error = "#{e.class}: #{e.message}"
          backtrace = (e.backtrace || []).first(50).join("\n")
        ensure
          $stdout = original_stdout
        end

        if error
          { 'ok' => false, 'error' => error, 'stack' => backtrace, 'stdout_log' => captured }
        else
          { 'ok' => true, 'result' => encode_json_safe(result), 'stdout_log' => captured }
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
          'pid'        => Process.pid,
          'port'       => port,
          'version'    => sketchup_version,
          'model_path' => safe_active_model_path,
          'started_at' => Time.now.utc.iso8601,
        }
        File.open(discovery_file_path, 'w') { |f| f.write(JSON.pretty_generate(payload)) }
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

      # --- small utility ----------------------------------------------------

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
