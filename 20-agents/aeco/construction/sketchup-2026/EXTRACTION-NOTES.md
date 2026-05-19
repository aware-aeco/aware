# sketchup-2026 EXTRACTION NOTES

This agent is generated from the same coverage IR scrape as `sketchup-2025`,
with `host_version` stamped to `2026.0`. See `../sketchup-2025/EXTRACTION-NOTES.md`
for the full extraction story — the SketchUp Ruby API docs at
https://ruby.sketchup.com/ are a **unified doc site** with no version selector,
so 2025 and 2026 share content.

## Source

- **Site:** `https://ruby.sketchup.com/`
- **Type-index root:** `https://ruby.sketchup.com/_index.html`
- **Doc generator:** YARD (https://yardoc.org)
- **Source kind:** `scrape` (HttpClient + AngleSharp)
- **Extraction date:** 2026-05-19

## Reproduction

```bash
echo '{"op":"coverage-extract","args":{"vendor":"sketchup","version":"2026.0","out_path":"cli-sidecar/Ingest/Output/sketchup-2026.0.ir.json"}}' \
  | cli-sidecar/bin/Release/net9.0/win-x64/publish/aware-sidecar.exe

# Regenerate the agent:
AWARE_SIDECAR=cli-sidecar/bin/Release/net9.0/win-x64/publish/aware-sidecar.exe \
  cli/target/release/aware.exe coverage generate sketchup 2026.0 \
  --from-ir cli-sidecar/Ingest/Output/sketchup-2026.0.ir.json \
  --vendor trimble --vertical construction
```

## Why version-stamped, not version-filtered

YARD tags each method with its introduction version (`<ul class="version">`
inside the method-detail block — typical entries: "SketchUp 6.0",
"Sketchup 2025.0", "Sketchup 2026.0"). The site itself always reflects the
LATEST release; older versions of the docs are not separately hosted. We
produce two IRs (sketchup-2025.0 + sketchup-2026.0) from the same source per
the v0.30 Phase B brief's instructions for unified-doc vendors. The two IRs
have identical content; the version field stamps `host_version`. Downstream
agents that need to filter by introduction-version can read the per-method
`since` field once it's enriched (currently empty — not in scope for v0.30
B4; see Known limitations in `../sketchup-2025/EXTRACTION-NOTES.md`).

## IR / catalog counts

Identical to sketchup-2025 (same source):

- **Vendor-published types:** 155
- **Types in IR:** 155
- **Types in generated catalog:** 155
- **Page count (HTTP fetches):** 156

| field | count |
|-------|------:|
| Methods (class + instance) | 1432 |
| Properties (paired + constants) | 760 |
| Constructors | 39 |
| Events | 0 |
| Enum values | 0 |

See `../sketchup-2025/EXTRACTION-NOTES.md` for the full selectors, parser
rules, and architectural decisions.

## Extraction errors log

Path: `cli-sidecar/Ingest/Output/sketchup-2026-errors.log` (lazily created
on first error). The 2026-05-19 run produced zero errors.
