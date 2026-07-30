# SketchUp bridge: main-thread pump (fixes #330)

Supersedes the threading half of `2026-05-19-v034-sketchup-exec-design.md`. The wire protocol,
the request/response shapes and the discovery mechanism are unchanged.

## The bug (#330)

The `aware-sketchup` Ruby bridge answers the first request of a session and then appears to wedge
permanently: later requests hang until the client gives up, the listening port stays in `Listen`,
new connections are accepted by the OS and pile up in `CloseWait`, and `list-instances` keeps
reporting the session healthy because it only reads the discovery file.

## Root cause — measured, not inferred

SketchUp hands the Ruby GVL to a **background** Ruby thread only about four times a second. Measured
inside SketchUp 26.1.189 (Ruby 3.2.2) by running a background thread that appends a timestamp and
then `sleep 0.01`:

```
samples: 88 over 20.07 s (asked for 10 ms sleeps)
gap ms: min=46.1  max=265  mean=230.7
```

So each background-thread wakeup costs ~250 ms, whatever the thread asked for.

The v0.34 bridge routed **every** step of a request through background threads:

```
accept thread: server.accept  →  Thread.new(worker)
worker thread: read header → read body → push to inbox
main thread  : UI.start_timer → drain_inbox → eval → push response
worker thread: wake on response → write frame → close socket
```

That is 5-7 sequential background-thread wakeups per request. An instrumented bridge (diagnostics
added to the accept loop, the worker and the timer, then a real session driven with `1+1`) shows
every one of those hops taking ~1 s of wall clock while the main-thread timer ticks happily at its
configured ~60 ms:

```
16:39:46  ACCEPT got conn #1
16:39:47  ACCEPT spawned worker #1
16:39:48  W1 frame read
16:39:49  W1 enqueued  →  MAIN process_one pushed response  (main thread: instant)
16:39:51  W1 response sent
16:39:52  W1 socket closed
```

Seven seconds for `1+1`. The pump was never dead; the *threads* were starved. Two consequences:

1. Latency grows through a session, because every lingering worker and every 90 s watchdog thread
   competes for the same ~4/s handoff. Once the round trip exceeds the client's patience every
   request "fails", which is what reads as a permanent wedge.
2. The `CloseWait` pile-up is just connections the starved threads had not got to yet — the OS
   completed the handshake, the client gave up and sent FIN, and Ruby closed them seconds later.

## The fix

**Do the whole request on the main thread inside the existing `UI.start_timer` pump, with
non-blocking socket I/O. No background threads at all.**

The main thread is the one thread SketchUp schedules reliably (~60 ms), and it is the only thread
allowed to touch the model — so it was already on the critical path for the eval. Moving accept,
read, write and close onto it removes every starved hop.

```
UI.start_timer(0.05, repeat) { pump }

pump:
  return if already pumping          # re-entrancy guard, see below
  accept_pending                     # server.accept_nonblock until :wait_readable
  service_connections                # per connection, one non-blocking step:
                                     #   drain readable bytes into inbuf
                                     #   complete frame? → eval inline → encode into outbuf
                                     #   flush outbuf with write_nonblock; when empty → close
                                     #   EOF / error / idle past the cap → drop
```

- **Re-entrancy guard.** SketchUp pumps its message loop during long model operations, so the timer
  can re-enter `pump` while a bake is still running. Without a guard the same buffered frame could
  be serviced twice — a duplicated bake. The guard makes a re-entrant tick a no-op.
- **Partial reads/writes** stay buffered per connection and resume on the next tick, so no socket
  call can ever block the SketchUp UI.
- **One request per connection**, then the bridge closes — unchanged from v0.34, and what the C#
  client already does (a fresh `TcpClient` per request).

### The 90 s watchdog is deleted

It protected nothing. The eval always ran inline on the main thread, so a runaway script hangs
SketchUp whether or not a watchdog answers; all the watchdog did was release the *client* with a
"timed out" reply while the script kept running — and it burned one starved background thread per
request to do it. The honest failure is now the client's own timeout.

`bake-scene` had a receipt branch keyed on that watchdog reply ("the materializer may still be
running…"). That caveat is still true, but it now belongs on the **client** timeout path, which is
the only timeout left.

## Making a stale running bridge visible

Installing the fixed bridge is not enough: SketchUp loads plugins once at startup, so a running
session keeps the old bridge until it restarts. Today nothing can tell — `--bridge-status` only
compares the *file* on disk with the *file* shipped beside the exe, and the discovery file carries
no bridge version. A user who installs the fix and does not restart sees the identical symptom.

- The discovery payload gains `bridge_version` (the loader's `BRIDGE_VERSION`).
- `SketchupInstance` gains a nullable `BridgeVersion` (older discovery files still parse), and
  `list-instances` reports it.
- When a request times out or the socket fails, `exec` / `bake-scene` append a restart hint if the
  *running* bridge is older than (or missing next to) the bridge packaged with the sidecar.

`BRIDGE_VERSION` goes 0.34.0 → 0.35.0.

## Verification (run against SketchUp 26.1.189, Ruby 3.2.2)

1. **Soak.** 25 consecutive raw `exec` round trips on one fresh session: 25/25 ok, 53-63 ms each
   (one pump tick) after a 193 ms first request — against 5-7 s, then outright failure, before.
   20 more through the real `aware-sketchup exec` sidecar: 20/20 ok. Afterwards the bridge port
   holds only its `Listen` socket: no `CloseWait` at all.
2. **Protocol state machine**, driven with adversarial clients against the live bridge: header and
   body in separate writes; a frame trickled one byte at a time; an absurd declared length (refused
   without allocating, connection closed, bridge survives); connect-and-close; a truncated request;
   10 concurrent connections (10/10 answered); a 4 MB request (979 ms) and a 4 MB response (44 ms),
   both spanning many ticks; malformed JSON and an unknown type (both answered with a clean error
   frame instead of a silently dropped socket). Normal service continues after every one.
3. **Bake.** A real `bake-scene` of 14 members into the live session: ok, 14 groups materialized
   with the right ownership stamps and a 200x200x3000 mm bounding box; 10 further requests
   afterwards all ok at ~125 ms. Retiring them by re-baking an empty scene under the same source
   id removed exactly those 14.
4. **Product level.** A whole-model send from floless.app (`POST /api/contract/steel-model/
   export-host/sketchup`, floless 0.113.6 on AWARE 0.108.0) returned 200 and materialized all
   187 groups of the demo project in the live session. The bridge's own log shows that request
   as 394,890 bytes in / 22,318 bytes out, 294 ms of main-thread work, with further requests
   served immediately afterwards.
5. `cargo fmt` / `clippy -D warnings` / `cargo test`, plus `dotnet build` + `dotnet test` for
   `cli-sketchup` (103 tests).

Every one of these was re-run against the FINAL bridge after each review round changed it — the
artifact that was verified is the artifact that ships (the installed `core.rb` was diffed against
the committed one).
