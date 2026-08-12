# Progressive large-geometry delivery (aware-aeco/aware#405)

**Goal:** let a consumer of a long `single`-lifecycle command render the first geometry while the
rest is still being produced, without putting a mesh into the trace or IFC knowledge into the
runtime.

**The gap #402 left.** #402 moved a large result out of the trace and into a run-owned artifact, so
the trace stayed bounded and memory stopped blowing up. It changed nothing about *latency*: a
`single` command has one output, emitted after the process exits, so on PALM.ifc (66 MiB, 2,993
objects, ~305 MB of geometry) the consumer waits ~43 s for the reader, then retrieves the artifact,
then parses 305 MB, and only then draws. IFC Lite shows first geometry on the same file in ~2 s
because it delivers in batches.

**Architecture.** A second, write-only channel per invocation, mirrored into the trace:

```
producer                         runtime                          consumer
────────                         ───────                          ────────
writes segment  ──► rename ──►   run-owned artifact dir      ──►  aware app artifact <id>
appends record  ──────────────►  tails AWARE_PROGRESS_FILE
                                 mirrors → node-progress     ──►  aware app logs --tail
returns descriptor            ►  node-output (unchanged)
```

Nothing about the mechanism is geometry-specific: the runtime moves bounded records and opaque
artifact ids. The producer decides what a batch is.

**Tech stack:** Rust 2024 / Tokio / Serde (runtime), Node.js + web-ifc (the reference producer).

---

### Task 1: the contract

**Files:** `10-core/cli-spec.md`, `cli/src/runtime/progress.rs`, `cli/src/runtime/provenance.rs`

- Specify `AWARE_PROGRESS_FILE`, the `{"$aware-progress": {...}}` record, the 8 KiB cap, the
  `phase` ladder, segment descriptors, and the ordering / durability / cancellation / no-resume
  semantics.
- `RunEvent::NodeProgress`; `progress::parse_record` (bounded, phase-required, segment ids fenced
  with the artifact rule) and `progress::ProgressTail` (offset cursor, holds back partial lines).

### Task 2: runtime plumbing

**Files:** `cli/src/runtime/invoker.rs`, `cli/src/runtime/orchestrator.rs`, `cli/src/commands/app.rs`

- `AgentInvoker::invoke_single_progress`, defaulted to `invoke_single` — only the CLI transport has
  a second channel to publish on.
- `CliInvoker` allocates the channel inside the run's artifact directory, sets the env var only when
  a listener exists, and relays records while waiting for the child (final drain after exit).
- The orchestrator selects over the call and the channel, writing each record as a flushed
  `node-progress` event — so ordering in the trace proves the records were live.
- `aware app logs` renders progress lines.

### Task 3: the reference producer

**Files:** `cli-connection-reader/progress.mjs`, `cli-connection-reader/index.mjs`,
`20-agents/aeco/engineering/ifc-reference-reader/**`

- `read-model` gains opt-in `batch-size`: every N objects, write a self-contained
  `{frame, seq, objects}` segment (tmp + rename), announce it after the rename, and publish the
  `parse` / `tessellate` / `batch` / `complete` phases. The whole artifact is still produced.
- Agent manifest 1.3.0 → 1.4.0 (additive), command doc, `aware agent reindex`.

### Task 4: evidence

**Files:** `cli-connection-reader/bench-progressive.mjs`, tests

- Benchmark: same file, both paths, reporting time-to-first-render (announcement + the segment
  parse a renderer pays) against time-to-complete. Takes `--ifc` for PALM; synthesises a model
  otherwise, so it runs anywhere.
- `cli/tests/app_run.rs`: drive the real `aware app run` as a live child, fetch an announced segment
  **while it is still running**, assert phases and that progress precedes `node-output`.
- Reader tests: ordered segments, the final flush, opt-in-ness, malformed `batch-size`, silence with
  no listener.

### Task 5: verify and ship

- `cargo fmt --check`, `cargo clippy -D warnings`, `cargo test`; `node --test` in
  `cli-connection-reader`.
- Real temp-`AWARE_HOME` run of the real reader over a real IFC with `batch-size`, tailing the trace
  and fetching a segment mid-run.
- `codex exec review --base main`, address findings, PR `Refs #405`, merge, label `qa-ready`.

## Measured (12,000-object / 46 MiB synthetic model, `--batch-size 100`)

| | first object drawable | complete |
|---|---|---|
| whole-artifact path (#402) | 1,418 ms | 1,418 ms |
| progressive (#405) | 354 ms | 1,627 ms |

Time-to-first depends on the batch, not the model: 302 ms at 3,000 objects, 354 ms at 12,000, while
complete went 552 ms → 1,627 ms. The 22-34% added to time-to-complete is the second copy of the
geometry, which is why segments are opt-in.
