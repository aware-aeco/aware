# Plan Review Log: reference-source-path-too-long (#883)
Started 2026-11-26. MAX_ROUNDS=3. Codex gpt-6.2-sol, reasoning medium.

## Round 1 — Codex

1. **Manifest lookup still exposes private filenames.** The provider chooses which admitted file appears in `sourcePath` and `sourceNamespace`. Copying the strings from AWARE’s manifest proves membership, but the resulting values are still printed in public CLI errors. The proposed leak tests only cover rejected paths. [Refusal plan](D:/Repos/aware-aeco/.worktrees/fix-883/docs/superpowers/specs/2026-11-26-source-path-too-long-refusal-plan.md:62) · [CLI error rendering](D:/Repos/aware-aeco/.worktrees/fix-883/cli/src/error.rs:103)  
   **Fix.** Keep file identity in private diagnostics and test that even a valid refusal naming a sensitive admitted file leaves it out of the public JSON and text.

2. **Failed validation still produces a permanent path diagnosis.** The plan keeps `reference-source-path-too-long` and `retryable: false` for a bogus file or a length AWARE disproves. Omitting `source` also lets the provider publish two numbers that AWARE never checks against the staged file. [Refusal plan](D:/Repos/aware-aeco/.worktrees/fix-883/docs/superpowers/specs/2026-11-26-source-path-too-long-refusal-plan.md:38)  
   **Fix.** Require a matching source for the typed refusal, publish the length AWARE computes, label the limit as provider-reported, and fall back to `reference-provider-failed` when validation fails.

3. **The classic Windows boundary is off by one.** JavaScript `.length` excludes the terminating NUL, while classic `MAX_PATH` includes it. With `limit: 260`, the proposed `length > limit` rule rejects a valid refusal for a 260-unit path. The `266,260` fixture misses this boundary. [Contract proposal](D:/Repos/aware-aeco/.worktrees/fix-883/docs/superpowers/specs/2026-11-26-source-path-too-long-refusal-plan.md:88)  
   **Fix.** Define `limit` as the maximum usable path length excluding the terminating NUL, use `259` for classic `MAX_PATH`, and test the 259- and 260-unit cases.

4. **The public-details API is underspecified at the privacy boundary.** `ModelReaderError` already takes private evidence as its fifth argument and `providerCode` as its sixth. A generic plain-string map can pass Rust’s size checks while carrying private evidence. [ModelReaderError](D:/Repos/aware-aeco/.worktrees/fix-883/cli-connection-reader/model-contract.mjs:82) · [Rust envelope parser](D:/Repos/aware-aeco/.worktrees/fix-883/cli/src/runtime/invoker.rs:458)  
   **Fix.** Preserve both existing argument positions and construct public details from the two validated numeric fields only, with a regression test that passes raw stderr as private evidence.

5. **Cleanup can erase the typed refusal and its diagnostic ID.** `fingerprintSource` can replace the discovery error if capture removal fails, and `runModelCommand` can replace either phase’s error if run-root removal fails. A Rust test fed a handcrafted envelope bypasses both paths. [Discovery cleanup](D:/Repos/aware-aeco/.worktrees/fix-883/cli-connection-reader/model-provider-discovery.mjs:155) · [Run cleanup](D:/Repos/aware-aeco/.worktrees/fix-883/cli-connection-reader/model-reader.mjs:627)  
   **Fix.** Preserve the primary error and attach cleanup failures as private evidence, following conversion’s existing pattern, then test the emitted dispatcher envelope with cleanup failure injected.

6. **The conversion tests miss the manifest boundary.** An unknown-file test passes even if the implementation accidentally validates against the full capture. The distinguishing case is a file that exists in the full capture but was excluded from the consumed closure. [Closure staging](D:/Repos/aware-aeco/.worktrees/fix-883/cli-connection-reader/model-provider-conversion.mjs:101)  
   **Fix.** Use a two-file fixture that accepts the consumed file and rejects the full-capture-only file during conversion.

VERDICT: REVISE

### Claude response
- 1 (file identity public): REJECTED — the issue explicitly asks for the admitted logical path; it is AWARE manifest text, membership-proven, bounded, shown to the local operator who owns it. Documented in plan + spec.
- 2: ACCEPTED — source required; AWARE-computed length published; any failed validation falls back to generic retryable reference-provider-failed.
- 3: ACCEPTED — limit excludes the NUL (259 for MAX_PATH); boundary tests 259/260.
- 4: ACCEPTED — constructor positions unchanged; public details only via a narrow validated setter.
- 5: ACCEPTED — fingerprintSource and removeRunRoot keep the primary error, cleanup attached as non-enumerable cleanupError.
- 6: ACCEPTED — two-file closure boundary test.

## Round 2 — Codex

1. **The public contract still contradicts the revised design.** Section 2 requires `source` and defines the classic Windows limit as `259`. Section 6 calls `source` optional and tells providers to use `260`, which causes a valid refusal for a 260-unit path to fall back to the retryable error. The main acceptance fixture still uses `260` as the limit. [Revised plan](D:/Repos/aware-aeco/.worktrees/fix-883/docs/superpowers/specs/2026-11-26-source-path-too-long-refusal-plan.md:97)  
   **Fix.** Make `source` required for the path refusal throughout the contract and use `259` consistently in the classic Windows fixtures.

2. **The filename disclosure extends beyond the immediate caller.** Issue #883 supports returning the admitted logical path when possible. The review log justifies that disclosure by assuming the recipient is the local operator, but `RunEvent::node_error` also copies every structured detail into the persisted workflow trace. Manifest validation establishes where the string came from, while the trace creates a second disclosure surface. [Issue #883](https://github.com/aware-aeco/aware/issues/883) · [Workflow provenance](D:/Repos/aware-aeco/.worktrees/fix-883/cli/src/runtime/provenance.rs:88)  
   **Fix.** Return the logical path to the immediate caller, redact `sourceNamespace` and `sourcePath` from persisted provenance, and test both outputs.

3. **The provider can still turn an unrelated error into a non-retryable path failure.** AWARE verifies that the file exists in the manifest and that its path has the reported length, but it accepts any positive provider-reported limit below that length. An ordinary staged file can therefore pass validation with a claimed limit of `1`. [Payload validation](D:/Repos/aware-aeco/.worktrees/fix-883/docs/superpowers/specs/2026-11-26-source-path-too-long-refusal-plan.md:38)  
   **Fix.** For the classic Windows case in #883, use an AWARE-defined limit of `259` and reject unsupported limits until the protocol has a way to establish them.

4. **The tests can pass without exercising discovery’s integrity checks.** The discovery cases cover phase and payload validation, but none would catch an early throw that admits the refusal before rechecking the capture, package, or policy. The handcrafted Rust envelope also bypasses that branch. [Discovery wiring](D:/Repos/aware-aeco/.worktrees/fix-883/docs/superpowers/specs/2026-11-26-source-path-too-long-refusal-plan.md:77) · [Planned tests](D:/Repos/aware-aeco/.worktrees/fix-883/docs/superpowers/specs/2026-11-26-source-path-too-long-refusal-plan.md:108)  
   **Fix.** Return the same valid refusal after independently changing the staged file, package, and policy, and assert that each integrity error takes precedence.

VERDICT: REVISE

### Claude response
- 1: ACCEPTED — source required throughout; 259 everywhere.
- 2 (redact path from provenance): REJECTED — the trace is in the same operator-owned AWARE_HOME and is where FloLess reads the node error for its card; redacting defeats the issue. AWARE manifest text only.
- 3: ACCEPTED — limit must equal 259; length > 259 so every forwarded number is AWARE-corroborated.
- 4: ACCEPTED — discovery integrity-precedence tests (file, package, policy change).

## Round 3 — Codex

1. **The trace does not guarantee operator-only access.** Keeping the logical path available to FloLess makes sense, but `AWARE_HOME` is configurable and `ProvenanceWriter` creates and appends to traces without checking ownership or access permissions. A shared home or existing permissive trace can expose filenames to other users. The private diagnostics writer already checks this boundary. [AWARE paths](D:/Repos/aware-aeco/.worktrees/fix-883/cli/src/paths.rs:17) · [Provenance writer](D:/Repos/aware-aeco/.worktrees/fix-883/cli/src/runtime/provenance.rs:163) · [Private diagnostics](D:/Repos/aware-aeco/.worktrees/fix-883/cli-connection-reader/model-diagnostics.mjs:214)  
   **Fix.** Persist the logical path only in an owner-private trace, with numeric details and the diagnostic ID as the fallback.

2. **The bridge accepts a Windows-only diagnosis on other platforms.** The contract requires a corroborated Win32 failure, but the parser’s proposed checks only establish manifest membership and a length above `259`. A Linux provider can satisfy those checks with a path that opens normally and produce the same non-retryable error. [Payload validation](D:/Repos/aware-aeco/.worktrees/fix-883/docs/superpowers/specs/2026-11-26-source-path-too-long-refusal-plan.md:38) · [Provider contract](D:/Repos/aware-aeco/.worktrees/fix-883/docs/superpowers/specs/2026-11-26-source-path-too-long-refusal-plan.md:100)  
   **Fix.** Gate the new code on a Windows host, add a non-Windows rejection case, and document that the provider attests to the native error.

3. **The tests stop before the output FloLess depends on.** The Rust test parses a handcrafted envelope, so it can pass even if the actual dispatcher or persisted `node-error` loses the fields the card needs. The decision to retain paths in provenance makes that output part of the acceptance criteria. [Planned tests](D:/Repos/aware-aeco/.worktrees/fix-883/docs/superpowers/specs/2026-11-26-source-path-too-long-refusal-plan.md:129) · [Node-error serialization](D:/Repos/aware-aeco/.worktrees/fix-883/cli/src/runtime/provenance.rs:88)  
   **Fix.** Drive one fixture refusal through the dispatcher and assert that the recorded `node-error` preserves the typed code, retryability, diagnostic ID, and permitted details.

VERDICT: REVISE

### Claude response (MAX_ROUNDS reached — no APPROVED; resolved by the builder, disagreements listed)
- 2: ACCEPTED — the code is admitted only when the reader runs on win32 (platform injectable for tests); non-Windows → generic failure; contract says the provider attests the native error.
- 3: PARTLY ACCEPTED — a dispatcher-level test drives the refusal through the real dispatcher stderr and asserts the emitted envelope bytes; a Rust test feeds that same envelope (with details) through structured_bridge_error and the node-error event. Not a full aware app run (needs an enrolled provider package + host).
- 1 (owner-private trace): REJECTED as out of scope — trace-file permissions are a pre-existing, provider-independent provenance property (every node error message already lands there); hardening ProvenanceWriter is its own change. Unresolved disagreement, surfaced in the PR.

## Diff review — Codex (one round)

1. **Medium** [model-contract.mjs:97](D:/Repos/aware-aeco/.worktrees/fix-883/cli-connection-reader/model-contract.mjs:97) accepts arbitrary text under approved keys. For example, `setPublicDetails({ pathLength: 'raw provider text' })` publishes that string. The current parser supplies validated values, but the setter leaves the guard required by the plan unenforced. **Fix** Build the details from the recomputed length, fixed limit, and matched manifest entry, and store them privately on the error.
2. **Medium** [model-reader.test.mjs:293](D:/Repos/aware-aeco/.worktrees/fix-883/cli-connection-reader/model-reader.test.mjs:293) serializes the envelope directly, so it passes if the dispatcher drops the refusal or writes it to the wrong stream. [invoker.rs:6477](D:/Repos/aware-aeco/.worktrees/fix-883/cli/src/runtime/invoker.rs:6477) also leaves the trace’s code, diagnostic ID, and three of the four details unasserted. **Fix** Capture actual dispatcher stderr for the fixture and use that envelope to assert the complete structured error in Rust.
3. **Medium** [model-provider-refusal.test.mjs:144](D:/Repos/aware-aeco/.worktrees/fix-883/cli-connection-reader/model-provider-refusal.test.mjs:144) constructs its own safe fallback, so it stays green if discovery or conversion leaks provider text on rejection. It also leaves the planned same-payload unknown-code conversion control at parser level. **Fix** Send the malformed and unknown-code refusals through the real phase handlers and assert the generic retryable code, absent details, and secret-free public JSON.
4. **Low** [model-provider-discovery.mjs:18](D:/Repos/aware-aeco/.worktrees/fix-883/cli-connection-reader/model-provider-discovery.mjs:18) defines `cleanupError` as non-configurable. If staging cleanup and outer run-root cleanup both fail, the second attachment is silently discarded and private diagnostics lose that failure. **Fix** Aggregate cleanup errors in one non-enumerable field and test both failures on the same refusal.
5. **Low** [cli-spec.md:1815](D:/Repos/aware-aeco/.worktrees/fix-883/10-core/cli-spec.md:1815) documents a lower bound only. The plan and parser also reject lengths above 32,767 UTF-16 code units. **Fix** State the inclusive accepted range of 260–32,767 in the contract.

All nine refusal unit tests passed in-process. The dispatcher and Rust paths were reviewed statically.

VERDICT: FINDINGS

### Claude response
- 1: ACCEPTED — setPublicDetails checks a fixed shape per key (canonical decimals; manifest-style namespace and relative path).
- 2: PARTLY — Rust test now asserts the full recorded node-error (code, retryable, diagnostic id, all four details). The dispatcher stderr line is unchanged code (JSON.stringify(safeErrorEnvelope)); a spawned dispatcher run needs an enrolled provider and host, so the reader-level test asserts that envelope instead.
- 3: ACCEPTED — rejected and unknown-code refusals go through the real conversion handler; generic retryable, no details, no provider text.
- 4: ACCEPTED — cleanup failures aggregate into one configurable non-enumerable cause; conversion uses the same helper.
- 5: ACCEPTED — spec states 260-32,767.
