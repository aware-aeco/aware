# Runtime Artifact Streaming Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Let a one-shot agent materialize large output as a run-owned artifact and return a small, stable handle instead of embedding the payload in a JSONL trace record.

**Architecture:** The runtime allocates a per-run artifact directory and passes it only to CLI transports via `AWARE_ARTIFACT_DIR`. A producer may atomically write a JSON artifact there and return an `$aware-artifact` descriptor. The descriptor is normal node output, so trace, DAG propagation, and terminal output stay bounded; `aware app artifact` resolves the opaque handle into a user-chosen file.

**Tech Stack:** Rust 2024 / Tokio / Serde; Node.js bridge / web-ifc.

---

### Task 1: Define the runtime artifact contract

**Files:**
- Modify: `10-core/cli-spec.md`
- Modify: `cli/src/runtime/provenance.rs`
- Test: `cli/src/runtime/provenance.rs`

**Step 1:** Specify `$aware-artifact` fields: run-scoped `id`, media type, byte count, and item count; document that it replaces inline large data and is resolved through `aware app artifact`.

**Step 2:** Add artifact-directory and safe-id helpers, with tests rejecting traversal and resolving only files inside a run directory.

### Task 2: Make CLI invocation artifact-capable

**Files:**
- Modify: `cli/src/runtime/invoker.rs`
- Modify: `cli/src/commands/app.rs`
- Test: `cli/src/runtime/invoker.rs`

**Step 1:** Write a failing invocation test asserting that a CLI transport receives a new, run-owned artifact directory environment variable.

**Step 2:** Thread the directory from `app run` through `DispatchInvoker` to `CliInvoker`, creating it before dispatch and preserving it for nested app execution.

**Step 3:** Run the targeted test and commit the runtime plumbing.

### Task 3: Expose artifact retrieval

**Files:**
- Modify: `cli/src/commands/app.rs`
- Modify: `cli/tests/app_run.rs`

**Step 1:** Add a failing end-to-end test for `aware app artifact <app> <id> --run-id ... --output ...` that copies the resolved JSON without reading it into memory.

**Step 2:** Implement the command using the provenance helper, requiring an explicit output path and refusing unsafe or absent handles.

**Step 3:** Run the focused integration test and commit.

### Task 4: Stream IFC geometry into the artifact

**Files:**
- Modify: `cli-connection-reader/index.mjs`
- Modify: `cli-connection-reader/read-model.test.mjs`
- Modify: `20-agents/aeco/engineering/ifc-reference-reader/manifest.yaml`
- Modify: `20-agents/aeco/engineering/ifc-reference-reader/commands/read-model.md`

**Step 1:** Write a CLI-level test proving `read-model` writes a valid artifact incrementally, prints only its descriptor, and does not put `objects` in stdout.

**Step 2:** When `AWARE_ARTIFACT_DIR` is present, write the old response envelope incrementally to a temporary file, rename it atomically, then print the descriptor. Keep the existing inline response as the compatibility fallback outside AWARE.

**Step 3:** Update the agent contract to describe the artifact result and required consumer behavior.

### Task 5: Verify and ship

**Files:** all changed files

**Step 1:** Run Node bridge tests, the focused Rust tests, formatting, clippy, and the full Rust suite.

**Step 2:** Perform a real temp-`AWARE_HOME` run using the IFC fixture; verify the trace has only a descriptor and `aware app artifact` produces valid geometry JSON.

**Step 3:** Run `codex exec review --base main`, address findings, commit, open a `Refs #402` PR, wait for green CI, merge, label the issue `qa-ready`, release, and verify npm.
