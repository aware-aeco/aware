# Private Windows runtime build inputs

This is unsigned build evidence, not a releasable installer. Signing, trusted builder
isolation, independent approvals, reservations and promotion remain separate gates.

`create-windows-internal-repro-inputs.mjs` accepts one local JSON input with exactly
`schema`, `source`, `inputs`, `tools` and `closures`. Its schema is
`aware-windows-repro-builder-inputs/v1`. `source` contains the exact Git `commit`,
`tree` and absolute source `bundle` path. Use canonical Git blob bytes for the six
input files; a checkout with line-ending conversions is not equivalent evidence.

| Input ID | File in that exact source |
| --- | --- |
| aware-cargo-lock | cli/Cargo.lock |
| reader-package-lock | cli-connection-reader/package-lock.json |
| builder-script | cli/build-windows-internal-repro.mjs |
| compiler-closure-script | cli/windows-compiler-closure.mjs |
| reader-settings-script | cli-connection-reader/repro-settings.mjs |
| compiler-audit-script | cli/windows-compiler-audit.ps1 |

`tools` has exactly six absolute files: `git`, `node`, `npm-cli`, `postject`,
`web-ifc-wasm`, and `powershell`. Node is Windows x64 24.14.0. PowerShell must be the
canonical inbox Windows PowerShell under the loader-observed System32 directory;
PowerShell 7 and copied hosts are not accepted. The builder hashes its executable
helpers, extracts the source with the authenticated Git tool and inline OS bootstrap,
then matches the helpers to those Git blobs before evaluation. JavaScript helpers
are imported from authenticated in-memory bytes; PowerShell receives the exact
authenticated script bytes over stdin, rather than re-reading a mutable script path.
Legacy `Add-Type` compilation uses a fresh, access-restricted ASCII directory
under the loader-observed Windows `Temp` directory. That parent must be canonical,
ASCII and writable; otherwise bootstrap refuses. The helper removes only its own
scratch directory, including after compilation failure. Compiler-child paths and
TEMP/TMP remain the declared private paths; no machine locale or installed files
are changed.

`closures` has exactly these eleven absolute directories. Inventory every file in
each selected component. Do not prune DLLs, headers or libraries to fit a disk quota.

| Closure ID | Selected installed component |
| --- | --- |
| npm-cache | Sealed offline npm cache |
| cargo-home | Sealed Cargo source directory containing vendor/ |
| compiler-rust-bin | Direct Rust 1.95.0 toolchain bin/; never rustup shims |
| compiler-rust-lib | The same toolchain lib/, including rustlib/ |
| compiler-msvc-bin | MSVC bin/Hostx64/x64/ |
| compiler-msvc-include | That MSVC version's include/ |
| compiler-msvc-lib | That MSVC version's lib/x64/ |
| compiler-sdk-include | Windows SDK Include/VERSION/ |
| compiler-sdk-um-lib | Windows SDK Lib/VERSION/um/x64/ |
| compiler-sdk-ucrt-lib | Windows SDK Lib/VERSION/ucrt/x64/ |
| compiler-sdk-bin | Windows SDK bin/VERSION/x64/ |

The factory emits a canonical path-free manifest and a local-only locator. The
locator has no environment or separate compiler executable paths. Roots must be
local absolute Windows drive paths of at most 200 characters, without redirection
links, ambiguous path components, controls, semicolons or equals signs. Spaces and
Unicode are supported. Retain local evidence privately: it contains physical paths.

Launch the exact digest-bound builder with the exact Node executable and an explicit
native Windows environment containing only the OS-derived `SystemRoot`. Node's
Windows child launcher can add PATH even with `env: {}`; that launch is refused.
A native ProcessStartInfo launcher must clear EnvironmentVariables, supply only
SystemRoot, preserve exact argv/cwd, and concurrently drain both redirected streams.
Do not forward the Visual Studio developer shell environment. PATH, INCLUDE, LIB,
LIBPATH and tool roles are constructed from fresh verified private copies. Cargo
and Rust receive descriptor-owned `VCINSTALLDIR` pointing at the private MSVC
directory and `VSCMD_ARG_TGT_ARCH=x64`, so MSVC lookup uses this fixed environment
instead of loading the machine's Visual Studio discovery DLL. Cargo
uses the private vendor copy and a separate empty CARGO_HOME. Existing targets,
network fallback, shared mutable compiler inputs and byte patching are not admitted.
Private npm logs are directed outside its copied cache. The cache inventory must
still match after consumption before any runtime receipt can be issued.

The Windows debugger audit observes compiler descendants and their loaded images
inside an owned non-breakaway job. Its process count must reconcile with the job,
and all images must be declared private inputs, derived Cargo outputs or files in
the explicitly protected Windows locations. It never attaches to unrelated work.
Pre/post inventories and private Rust sysroot checks are mandatory.

Every v3 audit records its effective, fixed startup policy, including an explicit
null when the MSVC inventory has no `vctip.exe`. When that exact private file is
created as a descendant, the auditor verifies its size/hash while the creation
event is paused, stops only that process before its entry point, and observes its
real exit with code `0xe0000488`. All process counts, image checks and zero-active
completion requirements still apply. No telemetry file is omitted or changed, no
machine policy is changed, and an unrelated process is never targeted. The native
Cargo fixture explicitly starts the copied helper from a build script to prove
this branch even when the linker does not request telemetry itself.

Process lifetime ordinals distinguish legitimate PID reuse from overlapping live
processes. Creation, DLL and exit events bind to that lifetime, and root identity
never follows a reused numeric PID. The event counter includes every observed debug
event; retained lifetime/image events are ordered within it, while thread, exception,
unload and debug-string events need not retain their payloads. Exited lifetimes lose
their active handle/breakpoint state. Windows owns closure of debugger-provided
handles when EXIT is continued; see Microsoft's [debugging-event contract](https://learn.microsoft.com/en-us/windows/win32/debug/debugging-events).
The v2 schema/policy is not accepted by new-source provenance validation.

Each exclusive request retains raw stdout/stderr, a combined command log and a
launch sidecar with status, signal, timeout budget and spawn error details, even
when PowerShell itself fails. Capture files refuse replacement; all writes are
attempted and execution/persistence errors are preserved together. An incomplete
audit remains diagnostic evidence and can never authorize successful provenance.

Before comparing A/B artifact inventories, call `verifyCompilerProvenance` from
the reviewed compiler helper separately for both output roots and retain the returned
evidence digests in the comparison report. This checks source/build identity, all
five required complete process audits, compiler inputs, successful exits and the
exact artifact inventory. Raw audit files stay outside the byte-compared payload.
Recheck both npm/Cargo sources and scan artifacts for original/private physical roots.

The required Windows native gate is `node cli/windows-vendor-repro.native.mjs`.
It discovers installed inputs only while constructing test manifests, copies its own
original inputs, hides those originals, and executes the production private compiler
path. It checks Rust and C bytes, SDK/CRT provenance, lib/rc/rustdoc, private-mutation
refusals and vendor-remap red controls. `AWARE_REPRO_COMPILER_ROOTS` may name a local
JSON mapping of the nine component names without the `compiler-` prefix to installed
paths; otherwise use a VS x64 developer environment. `AWARE_REPRO_TEST_EVIDENCE`
retains per-side audit files outside the test's unique temporary fixture directory.
The fixture cleans only its own newly created copies, sequentially between sides.
Its paths retain Polish characters and an emoji. Include provenance comes from
UTF-8 `/sourceDependencies` records, and `/LINKREPRO` captures actual library/object
bytes for comparison with the private inventories. Console diagnostic encoding is
not an authority: raw output is retained, while missing, foreign or altered input
records fail verification.

Reserve at least 10 GB free for a full paired runtime/package run and recheck between
stages. Compression is extra margin, not a capacity guarantee. Do not remove retained
failed-build evidence or weaken closure inventories to make the build fit.
