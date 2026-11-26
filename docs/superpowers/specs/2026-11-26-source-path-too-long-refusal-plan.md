# Plan — `reference-source-path-too-long` provider refusal (aware-aeco/aware#883)

## Problem
A protocol-v3 model provider that cannot open a staged source file because the absolute path is
over the OS path limit (Windows `MAX_PATH` 260 without long paths) has no way to say so.
Conversion admits only `reference-model-coverage-incomplete`
(`cli-connection-reader/model-provider-conversion.mjs`, `ALLOWED_PROVIDER_REFUSALS`); discovery
(`model-provider-discovery.mjs`, `invokeDiscover`) parses no refusal at all. Both flatten the
failure into `reference-provider-failed (…, retryable=true)`, so callers retry forever.

## Scope
Issue items 1-3. Item 4 (extended-length staging root so the provider can open long paths) is
optional and separate — not in this PR.

## Design

### 1. One shared refusal parser — new `cli-connection-reader/model-provider-refusal.mjs`
Move `safeProviderRefusal` out of conversion into a module both phases import:

```
parseProviderRefusal(result, { phase, stderrLimit, capture })
  -> null | { code, message, details? }
```

- Same envelope gate as today: nonzero integer exit, empty stdout, nonempty stderr
  `<= min(4096, stderrLimit)`, strict JSON, exact canonical bytes, `retryable === false`,
  UUID `diagnosticId`, `phase === <calling phase>`, message 1-240 chars without control chars.
- Allowed object keys: `code, phase, retryable, message, diagnosticId` (required) plus
  `source` (required for the path code, forbidden for every other; any other code carrying `source` is rejected
  as malformed → generic failure, as today for unknown keys).
- Allow-list becomes `code -> { phases, genericMessage }`:
  - `reference-model-coverage-incomplete`: phases `[conversion]` (unchanged behaviour,
    `coverage-counts-v1:` payload still the only forwarded text).
  - `reference-source-path-too-long`: phases `[discovery, conversion]`, generic message
    "A source file path is longer than this system can open. No partial model was imported."
- A recognised code with a phase it is not allowed in → `null` (generic failure).

### 2. Path payload — validated, never forwarded as provider text  _(revised round 1)_
Provider message must be exactly `path-length-v1:<length>,<limit>` (canonical unsigned
decimals, no leading zeros) with `limit == 259` (round 2: the only limit AWARE accepts until the
protocol can establish another) and `259 < length <= 32767`. **`limit` is the longest
usable path excluding the terminating NUL** — 259 for classic `MAX_PATH` (260 includes the
NUL). `length` is the UTF-16 code-unit length of the absolute path, also without NUL.
**`source: { namespaceId, path }` is required** (closed object, both strings): the admitted
logical file it failed on.

AWARE validates, and **any failure falls back to the existing generic
`reference-provider-failed` (retryable)** — an uncorroborated claim earns no permanent
diagnosis:
- `source` must be a file in the capture manifest **handed to that provider call**
  (discovery: the full capture manifest; conversion: the consumed closure manifest — a file
  present only in the full capture is rejected), exact match on `namespaceId` + `path`.
- AWARE recomputes `path.join(capture.stagingRoot, 'namespaces', namespaceId,
  ...path.split('/')).length` from the same root string it sent as `capture.root` and requires
  it to equal `<length>`, and requires `<length> > <limit>`.

Forwarded error: `ModelReaderError(code, phase, false, 'path-length-v1:L,M', rawResult)` —
existing argument positions unchanged (5th = private evidence, 6th = providerCode); public
details are attached through a separate, narrow setter that accepts only the validated
fields, never a free map. Envelope `details` (already accepted by Rust
`structured_bridge_error`: ≤8 keys, ≤256-byte values; serialized by
`StructuredAgentError.details`):
`{ pathLength: "<AWARE-computed L>", pathLimit: "<provider-reported M>", sourceNamespace, sourcePath }`.
`sourceNamespace`/`sourcePath` are AWARE's manifest strings and are included only when each
is ≤256 UTF-8 bytes (longer admitted paths still forward the numbers; the spec says so).

**Rejected (round 1, item 1): keep file identity out of the public envelope.** The issue
explicitly asks for the admitted logical path ("name the exact path instead of an estimate");
the public error goes to the same local operator who owns the source, the string is
AWARE's own manifest entry (membership-proven, bounded, no provider bytes), and it is the one
fact the caller needs to act (shorten or move that file). The contract states this disclosure.

`ModelReaderError` public details: emitted by `safeErrorEnvelope` only when set, so every other
envelope is byte-identical to today. Test: a valid refusal whose raw stderr is the private
evidence → envelope contains only the four validated keys.

### 3. Diagnostic ID
The forwarded error is always a fresh `ModelReaderError`, so its `diagnosticId` is AWARE's
`randomUUID()`, never the provider's; the provider's id stays only in the private diagnostic
record's retained stderr. Tested explicitly for both phases.

### 4. Discovery wiring
In `invokeDiscover`, on a nonzero/invalid result: parse the refusal (phase `discovery`,
`capture` = the captured source). If admitted, first `verifyCapturedSource` and re-check
package identity + admitted policy (same order as conversion does before honouring a
refusal), then throw the non-retryable error with the raw result as private unsafe details.
Otherwise the existing `reference-provider-failed` (retryable) path is unchanged.

### 4b. Cleanup must not erase the typed refusal  _(added round 1)_
`fingerprintSource` (capture removal) and `runModelCommand` (`removeRunRoot` in `finally`) can
replace a primary error with a cleanup error, losing the refusal and its diagnostic ID. Follow
conversion's existing pattern: when a primary failure exists, attach the cleanup error as a
non-enumerable `cleanupError` and rethrow the primary; a cleanup failure with no primary still
throws. Test: injected cleanup failure → emitted envelope still the path refusal.

### 5. Conversion wiring
Replace the local `safeProviderRefusal` call with the shared parser
(`phase: 'conversion'`, `capture: closure`). Everything else unchanged.

### 6. Contract — `10-core/cli-spec.md`
Extend the provider-refusal paragraph: the new code, both phases, the payload grammar, the
required `source`, AWARE's recomputation, the `details` keys, and when a provider may raise
it: only for a corroborated long-path failure — Windows error 206
(`ERROR_FILENAME_EXCED_RANGE`), or error 3 (`ERROR_PATH_NOT_FOUND`) on a path whose length
exceeds the limit and which exists through a long-path-capable lookup (e.g. the `\\?\` form).
Length measured in UTF-16 code units of the absolute path formed from the request's
`capture.root`; `limit` is 259 (classic `MAX_PATH` 260 minus the terminating NUL), the only accepted value.

## Tests (red first, each with an acceptance control)
`model-provider-refusal.test.mjs` (unit) plus additions to `model-provider-conversion.test.mjs`
and `model-provider-discovery.test.mjs`:
1. Conversion: valid path refusal → code, `retryable:false`, message `path-length-v1:266,259`,
   details with numbers + source. Control: the identical stderr with code
   `reference-unknown-refusal` → `reference-provider-failed`, retryable.
2. Discovery: same, phase `discovery`. Control: same payload with `phase:"conversion"` during
   discovery → generic retryable failure.
3. Coverage code in discovery stays rejected (phase table control).
4. Provider free text never forwarded: refusals with prose message, a message with a secret
   token appended to the payload, a `source` naming a file not in the manifest, a `source.path`
   carrying traversal / a secret, a length mismatch — the envelope (JSON.stringify of
   `safeErrorEnvelope`) never contains the provider's string; message is generic, no details.
   Control: the valid payload does appear (so the assertion is not vacuous).
5. Diagnostic id: envelope `diagnosticId` ≠ provider `diagnosticId`, is a UUID.
6. Long admitted path (>256 bytes) → numbers forwarded, source dropped.
6b. Boundary: limit 259, length 259 → generic failure; length 260 → admitted; limit 1 or 260 → generic failure.
6e. Discovery integrity precedence: the same valid refusal after a staged file changes, the
    package changes, or the policy changes → that integrity error, not the refusal.

**Rejected (round 2, item 2): redact the path from persisted provenance.** The run trace lives
in the same operator-owned `AWARE_HOME`, and it is exactly where FloLess reads a failed node's
error to render the card (#4085); redacting it there would defeat the issue's goal. The
string is AWARE's manifest entry, no provider bytes.
6c. Conversion manifest boundary: two-file capture, closure consumes one; naming the consumed
    file → admitted; naming the capture-only file → generic failure.
6d. Missing `source`, unknown file, or length mismatch → generic `reference-provider-failed`.
7. Envelope with details satisfies Rust `structured_bridge_error` bounds — one Rust unit test in
   `invoker.rs` proving a `reference-source-path-too-long` envelope with `details` reaches the
   CLI typed, non-retryable, with details intact (targeted `cargo test` only).

## Round 3 additions
- The path code is admitted only when the reader runs on Windows (`platform` injectable for
  tests); elsewhere it falls back to the generic failure. The contract states the provider
  attests the native Win32 error.
- A dispatcher-level test asserts the emitted stderr envelope; the Rust test parses that exact
  shape (with `details`) and checks the node-error event keeps code, retryable, diagnostic id
  and details.
- Unresolved (logged): owner-private trace permissions — pre-existing provenance property,
  out of scope.

## Out of scope
- Item 4 (long-path staging root).
- floless.app Tekla Parser provider raising the code, and floless #4085 card switching from an
  estimate to the exact path — follow-up after an AWARE release.

## Release
The connection reader ships inside the CLI package, so FloLess needs a new AWARE release
(minor: new provider-contract code) before a provider can rely on it.
