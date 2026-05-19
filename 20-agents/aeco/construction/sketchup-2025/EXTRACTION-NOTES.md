# sketchup-2025 EXTRACTION NOTES

This agent is generated from a coverage IR scraped from the SketchUp Ruby API
documentation (https://ruby.sketchup.com/). These notes capture **how** the IR
was produced so the extraction can be reproduced.

## Source

- **Site:** `https://ruby.sketchup.com/`
- **Type-index root:** `https://ruby.sketchup.com/_index.html`
- **Doc generator:** YARD (https://yardoc.org)
- **Source kind:** `scrape` (HttpClient + AngleSharp)
- **Extraction date:** 2026-05-19

## Why this URL — version availability

Trimble's SketchUp Extensibility Team publishes a **single unified YARD-generated
site** at https://ruby.sketchup.com/ for the SketchUp Ruby API. There is no
version selector: the site always reflects the latest shipping SketchUp version
(2026.1 as of 2026-05-19). Each method's introduction version is tagged inline
via a `<ul class="version">` block (`SketchUp 6.0`, `Sketchup 2025.0`,
`Sketchup 2026.0`, etc.), but the docs site itself is not versioned.

Per the v0.30 Phase B brief, we produce **two IRs (2025.0 + 2026.0) from the same
source**, stamping `host_version` accordingly. The two IRs have identical content
because the docs are unified. This is the agreed convention for unified-doc
vendors and is the same approach the brief specifies for any vendor where 2025
and 2026 share doc trees.

## Reproduction

```bash
echo '{"op":"coverage-extract","args":{"vendor":"sketchup","version":"2025.0","out_path":"cli-sidecar/Ingest/Output/sketchup-2025.0.ir.json"}}' \
  | cli-sidecar/bin/Release/net9.0/win-x64/publish/aware-sidecar.exe

# Regenerate the agent:
AWARE_SIDECAR=cli-sidecar/bin/Release/net9.0/win-x64/publish/aware-sidecar.exe \
  cli/target/release/aware.exe coverage generate sketchup 2025.0 \
  --from-ir cli-sidecar/Ingest/Output/sketchup-2025.0.ir.json \
  --vendor trimble --vertical construction
```

The extractor is at `cli-sidecar/Ingest/Extractors/SketchUp/` (Extractor.cs +
PageParser.cs + MemberPageParser.cs). The sidecar verb is `coverage-extract`.

## C SDK explicitly out of scope

The SketchUp **C SDK** (file-format reading/writing outside SketchUp) is NOT
included in this agent. Different surface, different license (EULA + login
gate). Per project memory `project_sketchup_csdk_skipped`, the C SDK is
intentionally not shipped in the AWARE registry. Future SketchUp-flavor
agents that need the .SKP file format would route through the Ruby API's
`Sketchup::Skp` module (which CAN read/write SKP files from inside SketchUp)
or via Trimble Connect's REST API. The Ruby API is the only piece of the
SketchUp surface we cover here.

## Crawl strategy

YARD bundles every class/module's full member list onto **one HTML page per
type** (unlike Sandcastle, which spreads each member across its own
per-member detail page). The crawl is therefore single-pass:

1. **Phase 1 — type-URL discovery (sequential):**
   Fetch `_index.html`, then extract every `<span class="object_link">
   <a href="X.html">` anchor. Filter out the YARD-internal pages
   (`_index.html`, `class_list.html`, `method_list.html`,
   `top-level-namespace.html`, `frames.html`) and the file-doc pages
   (`file.*.html` — ReleaseNotes, LayOut overview, etc.). Yields ~155
   type URLs.

2. **Phase 2 — type-page crawl (4-way concurrent):**
   For each URL, fetch the page and parse it into a complete `TypeInfo`
   including: top-level summary, base type, all constructors, class methods,
   instance methods, attr-style getter/setter property pairs, constants
   (folded as read-only properties), and inheritance metadata.

3. **No enrichment pass needed** — YARD packs everything onto the type page
   already.

## Selectors / parser rules

| What | Selector |
|---|---|
| Title / kind | `h1` → `"Class: Foo::Bar"` / `"Module: Foo::Bar"` / `"Exception: Foo::Bar"` |
| Type-level docstring | `div.docstring > div.discussion` (top-level only — not inside `div.method_details`) |
| Inheritance | `div.box_info > dl > dt:contains("Inherits:") + dd > span.inheritName` |
| Constructor block | `div#constructor_details > div.method_details` |
| Class method blocks | `div#class_method_details > div.method_details` |
| Instance method blocks | `div#instance_method_details > div.method_details` |
| Method signature | `h3.signature` — `#methodname(args) ⇒ ReturnType` (with `.` prefix for class methods) |
| Method params | `div.tags > ul.param > li > span.name` + `span.type` |
| Method return | `div.tags > ul.return > li > span.type` |
| Method raises | `div.tags > ul.raise > li > span.type` |
| Constants | `dl.constants > dt[id$="-constant"]` |
| Overloaded signatures | `h3.signature > span.overload` (joined with `" \| "` in IR) |

### Title-suffix note spans

YARD appends visual marker spans inside `<h1>`:
- `<span class="abstract note title">Abstract</span>` for abstract classes
- `<span class="deprecated note title">Deprecated</span>` for deprecated classes

The parser strips ANY `span.note.title` descendant of the h1 before reading
the title text, so the catalog name stays the clean type identifier
(`PagesObserver`, not `PagesObserver Abstract`).

### Exception classes

YARD distinguishes exception classes with `<h1>Exception: ...</h1>`. The
parser treats these as `kind: "class"` since they're still Ruby classes; the
exception-hood is signalled via the `@base` chain (typically
`StandardError`/`ArgumentError`/etc).

### Overloaded signatures

When YARD sees `@overload` tags it wraps each signature in
`<span class="overload">` inside the `<h3>`. The parser detects this case
and joins all overload signatures with `" | "` in the IR's `signature`
field. The catalog's `params[]` and `returns` reflect the FIRST overload
(downstream tooling reads the joined signature for the full picture).

### Constants folded into properties

Ruby constants are public read-only values by language convention. SketchUp
uses them extensively as enum surrogates (e.g.
`Sketchup::Model::VERSION_2020`, `Sketchup::Layer::TYPE_BASIC`). The parser
folds every constant into `properties[]` with `getter=true, setter=false`,
inferring a best-effort type from the literal value (`Integer`, `String`,
`Symbol`, `Boolean`, `Float`, `NilClass`, `Array`, `Hash`, or `Object`).

### attr-style getter/setter pair detection

Ruby has no `attr_*` distinction at the API surface — users call methods.
YARD documents `attr_accessor :foo` as a pair of methods (`#foo` getter +
`#foo=` setter). The parser detects pairs and collapses them into a single
`PropertyInfo` with both getter and setter true. Standalone setters (no
companion getter) become write-only properties. Standalone getters stay as
methods (they may be plain accessors or behavior methods — preserving them
as methods is truer to Ruby semantics).

### Observer callbacks stay as methods

Ruby observer classes (e.g. `Sketchup::ModelObserver`) expose callback hooks
named `on...` (`onSaveModel`, `onTransactionStart`, etc.). The IR schema's
`events[]` slot is shaped for .NET-style events (delegate type, threading
model, fires_when) — Ruby callbacks don't fit. We leave `events[] = []` and
keep these callbacks in `methods[]`. Downstream tooling can detect them by
name pattern (`on*`) or by the parent class's inheritance (most descend from
`Sketchup::EntityObserver`, `Sketchup::ModelObserver`, etc.).

## IR / catalog counts

- **Vendor-published types:** 155 (per `_index.html` alphabetic listing)
- **Types in IR:** 155 (1:1 with vendor enumeration; 0 skipped)
- **Types in generated catalog:** 155 (1:1 with IR)
- **Page count (HTTP fetches):** 156 (1 root + 155 type pages)

## Member counts (sketchup-2025.0.ir.json)

| field | count |
|-------|------:|
| Methods (class + instance) | 1432 |
| Properties (paired + constants) | 760 |
| Constructors | 39 |
| Events | 0 (Ruby/SketchUp has no .NET-style events) |
| Enum values | 0 (Ruby has no enums) |

## Concurrency tuning

Trimble's CDN serves the SketchUp Ruby API docs (~155 types, ~600KB total)
without throttling at 4-way concurrent fetches. Full extraction completes in
~30 seconds on a fresh run. No fresh-per-fetch HttpClient discipline is
needed (the corpus is small enough for a single shared connection pool).

## Strict-verify caveat

`cli-sidecar/Ingest/Output/verify-types-strict.ps1` is hard-coded for the
Sandcastle title shape (`Foo Class` / `Bar Interface`). YARD titles read
`Class: Foo` / `Module: Bar`, so the strict verifier reports
`kind-word-missing` for every YARD-sourced type. This is a script-vs-vendor
shape mismatch, not an IR defect. The codex-rescue reviewer (per
`docs/superpowers/specs/host-coverage-review-protocol.md`) applies vendor-
aware checks that read the YARD title format correctly. Constants with
inferred `type=Object` (the default for non-literal constant values that
aren't a string/symbol/bool/numeric) also trip the verify-types-strict
property-placeholder rule; these are legitimate `Object`-typed values per
the Ruby contract (per the protocol's known-legitimate-exemptions section).

## Extraction errors log

Path: `cli-sidecar/Ingest/Output/sketchup-2025-errors.log` (lazily created
on first error). The 2026-05-19 run produced zero errors — all 155 type
pages parsed cleanly on the first try.

## Known limitations / follow-up work

- **Method introduction-version tags not currently captured.** YARD's
  `Version: SketchUp X.Y` block (`<ul class="version">`) on each method is
  ignored by the parser. A future enrichment could populate the IR's
  optional `since` field per method, enabling filter-by-version downstream
  (e.g. "show only members available in SketchUp 2025"). Not in scope for
  v0.30 B4 — the two IRs (2025.0 + 2026.0) carry identical surfaces today.
- **C SDK separate agent.** A second SketchUp agent covering the C SDK
  (file-format I/O) is intentionally NOT shipped in the registry. See the
  "C SDK explicitly out of scope" section above.
