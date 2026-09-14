# Plan: Bind agent registry releases to their published manifests
_Round 0 — initial draft by Codex_

## Goal
Make `aware agent install <agent>@<release>` and `aware agent update <agent>@<release>` either install the manifest identity bound to that registry release or fail before changing the installed-agent directory. Repair `viewer-3d@0.1.0` so it resolves to immutable historical bytes whose manifest declares `viewer-3d@0.1.0`.

## Approach
1. Extend each registry version entry with explicit `manifest-agent` and `manifest-version` fields. Keep the registry release key separate from the manifest version because calendar release keys such as `tekla@2025.0.1` intentionally map to an implementation manifest such as `0.1.4`. The explicit fields remove that ambiguity.
2. Require both fields on every registry release consumed by install or update, regardless of trust. Reject missing, partial, or blank bindings before download. Validate bound and extracted agent IDs with one platform-independent allowlist: ASCII letters, digits, `.`, `_`, and `-`; first character alphanumeric; no trailing dot or space; and no Windows reserved stem. This rejects separators, prefixes, dot segments, colons, controls, and cross-platform aliases before any path join. Validate bound and extracted manifest versions as strict SemVer with the shared parser, closing the corresponding manifest-validation gap. For a rename alias, require `alias-of` to equal every release's `manifest-agent`; ordinary entries may still bind a suffixed install id explicitly.
3. After extraction and manifest validation, compare the payload's `agent` and `version` with the release entry's bindings. Perform this before destination conflict checks and before staging/promotion for install; perform it before any replacement or deletion for update. Use one helper so install and update cannot drift.
4. Add regression tests that first fail on the current code: install and update each reject mismatched manifest version and identity; install creates no destination, and update preserves the previous installed bundle. Add focused cases for missing/partial/blank bindings, unsafe bound or payload IDs, invalid manifest SemVer, a legitimate rename alias/suffixed install ID, an alias target mismatch, and transitively verify bundle install and update-all use the same guarded path. Add producer and attestation tests proving publish emits both bindings, reindex/check rejects missing or mismatched bindings, and provenance refuses a digest-equal receipt whose fresh-index semantic binding differs. Pre-download schema errors must name `<key>@<release>` and the missing or invalid field in plain English; post-extraction mismatches must additionally name the expected and actual payload values.
5. Teach registry publishing to write both bindings from the manifest being published. Make reindex/check require both bindings for every checked-in release and compare them with the manifest loaded from that release source, independent of runtime trust. This prevents CI from accepting metadata production will refuse. Extend installed-bundle provenance assessment so “verified” also requires the fresh index bindings, receipt bindings, and installed manifest identity to agree. Update the Agent and CLI specs with the mandatory fields, portable identity and SemVer rules, alias constraint, trust-independent install enforcement, and provenance cross-check.
6. Backfill checked-in entries by loading each release source directly, rather than trusting the generated catalog. For mutable entries this is the current checkout source; for existing pinned entries it is the named Git tree. Repair `viewer-3d@0.1.0` specially by pinning the introducing release commit `b6991920e952a0bc293308faa9090dfd2d9b570e`, its matching archive root, and the digest computed from that commit's viewer subtree. Pin that digest in a regression test. Regenerate the catalog and independently confirm it reports release key 0.1.0 with manifest version 0.1.0.
7. Keep the captured pre-fix real-URL reproduction as baseline evidence. Before merge, verify the fixed CLI with registry fixtures marked both official and unverified, plus the checked-in registry through `AWARE_REGISTRY`; assert exact installed manifest and receipt bindings, mismatch refusal, and no partial installation. After merge, run the required fresh real-URL smoke test and confirm verified provenance.
8. Run format, clippy, the complete Rust suite, registry reindex/check, and mandatory Codex diff review. Open a PR with `Refs #534`, wait for CI and final review, merge, then mark #534 `qa-ready` and leave it open for release testing.

## Key decisions & tradeoffs
- The registry release key and manifest version remain distinct axes. Explicit bindings are safer than assuming equality and preserve existing calendar/versioned product releases.
- Registry index parsing stays structurally backward compatible, but install and update fail closed when a release omits either binding under every trust level. This intentionally requires old custom registries to add two explicit fields before they can mutate installations.
- The installer verifies semantics in addition to digest integrity. A digest can faithfully attest the wrong payload if registry metadata was refreshed after a mutable archive moved.
- Only `viewer-3d@0.1.0` is moved to historical bytes in this issue. Existing entries receive semantic bindings, while wholesale immutable-archive migration remains separate work unless current checks prove it is required for correctness.

## Risks / open questions
- Rename aliases intentionally resolve a registry key to a differently named manifest; explicit `manifest-agent` must preserve that path.
- Updating all official entries can create a large index diff. Generate it deterministically from resolved release sources and inspect aliases and pinned historical sources.
- The historical viewer bundle may depend on older builtin runtime behavior. The issue asks for exact delivery; the introducing commit is the only unambiguous 0.1.0 release point, while ordinary app compatibility gates remain authoritative.
- Registry cache fingerprints must include the new fields automatically through serialization so changed bindings invalidate cached archives.
- Alias validation must bind the declared migration target to the payload identity; otherwise the alias label and the filesystem migration can disagree.

## Out of scope
- Releasing or tagging AWARE.
- Changing FloLess UI behavior already merged downstream.
- Collapsing registry release keys into manifest versions.
- Migrating every registry tarball to an immutable commit in this PR unless the existing official-registry contract makes that unavoidable.
- Redesigning update's existing rename/promotion transaction after semantic validation passes; #534's guarantee is that a binding mismatch is detected before that transaction begins.
