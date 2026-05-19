# Extraction notes — dynamo-4-1-1

**Vendor:** Autodesk (DynamoBIM)
**Version:** 4.1.1.4941 (NuGet `DynamoVisualProgramming.Core@4.1.1.4941`)
**Source kind:** `hybrid` (NuGet XML doc + .NET DLL reflection + GitHub source tree)
**Extracted at:** 2026-05-19
**GitHub ref:** `RC4.1.1_master` (release-candidate branch — see caveat #1)

## Source landscape

Same as `dynamo-4-1-0/EXTRACTION-NOTES.md` — read that first. The Dynamo project has no canonical hosted HTML type-index, so the extractor uses the NuGet payload as the canonical source and produces `doc_url`s pointing at the project's GitHub source tree.

## Version-specific notes

### Caveat #1: no GitHub tag exists for v4.1.1

Autodesk's release pipeline ships `v4.1.0` as a public GitHub tag but the `v4.1.1` patch release (NuGet `4.1.1.4941`, published 2026-05-10) has **no tag** in the public DynamoDS/Dynamo repo. The closest stable ref is the `RC4.1.1_master` release-candidate branch which Autodesk uses internally for each shipped 4.1.x build.

Doc URLs in this IR point at `github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/...`. This branch is stable per Autodesk's release process — once a build ships, the RC branch isn't rewritten — but it is NOT a tagged release. If Autodesk later promotes 4.1.1 to a public tag, the URLs should be updated.

### Caveat #2: identical XML docs with 4.1.0

The XML doc files in the 4.1.1 NuGet package are **byte-identical** with the 4.1.0 package (verified at build time via md5sum). This means:

- The public API surface, including type/method/property signatures and prose summaries, is unchanged between 4.1.0 and 4.1.1.
- The DLLs themselves differ — 4.1.1 carries bug-fix code changes — but no new public types or methods were added.
- The 4.1.0 and 4.1.1 IRs are structurally identical (same `type_count`, `method_count`, etc.) and differ ONLY in `host_version`, `source.urls`, and the GitHub-ref segment of `doc_url`s.

This makes the 4.1.1 agent a **content-equivalent variant** of the 4.1.0 agent. AWARE app authors picking between them should choose based on which NuGet version they intend to ship against — not based on API differences.

### Caveat #3: source-content scan against RC branch

The content-scan augmentation (which finds the source file declaring each type) runs against the `RC4.1.1_master` branch. Some types defined in v4.1.0 source files have been renamed or moved in the RC branch — but the public API surface is stable. The strict-verify run picks 50 random types; for 4.1.1 it gets 50/50 PASS against the RC branch with the source-code lowercase-kind-word recognition rule.

## Coverage stats

Identical to 4.1.0:

- 707 types
- 3,064 methods
- 2,370 properties
- 142 events
- 5,576 catalog files

## Crawl strategy

Same as dynamo-4-1-0 (read that EXTRACTION-NOTES.md). The only difference is the GitHub ref: `RC4.1.1_master` instead of `v4.1.0`.

If the codex-coverage reviewer can't reach the RC branch (e.g. Autodesk later deletes it), the IR's `source.urls[0]` still points at the NuGet package — the binary + XML doc set is reproducible from there for as long as NuGet preserves the package version.
