# Host Coverage Review Protocol (v0.30)

The verification protocol every `codex-rescue` reviewer runs against a
freshly-generated raw host agent. Outcome is committed to
`20-agents/aeco/<vertical>/<host>-<version>/COVERAGE-REVIEW.md`.

This protocol is load-bearing: it is the gate between "an extractor ran"
and "the agent is shippable." A reviewer must execute the four steps in
order, with the listed pass/fail criteria, and emit the report in the
exact shape below.

## Inputs

- **The generated agent folder** — `manifest.yaml`, `skills/*.md`,
  `catalog/*.json`. Path: `20-agents/aeco/<vertical>/<host>-<version>/`.
- **The IR file the agent was generated from** —
  `cli-sidecar/Ingest/Output/<host>-<version>.ir.json`.
- **The vendor's type-index URL** — recorded in the IR's
  `source.urls[0]`.

## Step 1 — Type enumeration (FULL, not sampled)

Verify the catalog covers every type the vendor publishes — no gaps, no
silent drops.

1. Fetch the vendor's type-index page(s) from `source.urls[0]`. **Some
   vendors do not publish a single flat type-index page** (Tekla, for
   one, uses a sidebar navigation tree); in those cases the reviewer
   walks the same multi-page enumeration the extractor walked. The
   extractor's `EXTRACTION-NOTES.md` documents the actual enumeration
   pattern under "Crawl strategy" — follow it.
2. Extract every `(type_name, namespace)` pair using the vendor-specific
   CSS selector defined by the extractor.
3. Compute the set difference between the vendor's enumerated types and
   the `catalog/*.json` filenames.

   **Filename sanitization**: catalog filenames sanitize characters that
   Windows rejects in filenames. Generic types use `_of_` for `<` and
   strip `>` — vendor's `IEnumerable<T>` becomes catalog file
   `IEnumerable_of_T.json`. Comma-separated type-args render as `__`.
   The catalog file's `"name"` field always carries the canonical
   vendor name. Reviewers comparing filename → vendor name MUST either:
   (a) open each catalog JSON and use its `name` field, or
   (b) reverse the sanitization in the comparison logic.

4. **PASS** if `missing_types_count == 0`. List any missing types in the
   report.

This step is **full enumeration, not sampled** — a single missed type
is a FAIL.

## Step 2 — Deep-check sampling (N=50 random types)

Verify that each catalog entry's members match the vendor's actual API
surface for that type.

1. Seed the RNG with `sha256(IR-file-contents)` so the same IR always
   selects the same sample (deterministic re-runs).
2. Pick 50 random types from the IR's `types` array.
3. For each sampled type, fetch its `doc_url` from the catalog entry.
4. Parse the vendor page for: methods, properties, events, constructors,
   enum values.
5. Compare each member's name and signature against the corresponding
   list on the catalog entry.
6. **PASS** if `deep_check_pass_rate == 50/50`. Report every mismatch
   per type (extra members, missing members, signature drift).

### Reviewer regex helpers (Tekla LST script-substitution)

Vendors that use script-substitution markup for language-specific
delimiters (Tekla's `LST*` spans, see Tekla's `EXTRACTION-NOTES.md`)
expose multi-language tokens of the shape:

```
AddLanguageSpecificTextSet("LSTxxx_N?cs=X|vb=Y|cpp=Z|nu=W|fs=V");
```

The `cs=` key (C# rendering) can appear anywhere in the pipe-list, NOT
just first. Reviewers extracting the C# token must use a regex like
`[?|]cs=([^|"]*)` — anchoring on `?cs=` only mishandles entries where
`cpp=` or `vb=` precede `cs=`. Tekla emits ~3 of 50 sampled types per
version in this shape.

If the LST has no `cs=` key at all (e.g. `cpp=%` for a C++-only
`out`-param marker), the extractor must render the empty string. A
trailing-dot fallback contaminates the catalog (see PageParser.cs
LST handling). **0.015% of methods** in Tekla 2025/2026 hit this case.

### Clustered FAIL investigation

When Step 2 surfaces 3+ sampled types failing the same root cause (e.g.
all generic-typed methods rendering `.X.` instead of `<X>`), DO NOT
declare FAIL immediately. The cluster may be:

- A REAL parser-systemic bug — verify by spot-checking 1-2 unsampled
  types matching the pattern; if they also fail, declare FAIL.
- A REVIEWER bug — your own regex / parser may be mis-interpreting
  vendor markup. Cross-check against the vendor URL directly before
  declaring FAIL.

In practice: a Step 2 FAIL with `pass_rate < 47/50` AND a clustered
root cause is almost always one or the other. Spot-check before
writing the final report.

### Sample-size adaptation (vendors with N < 50 types)

For small-surface vendors (Tekla Tedds has 33 types, Solibri may be smaller),
the "pick 50 random types" rule degenerates. The protocol's adaptation:

- **If vendor surface < 50:** sample the FULL set. The `deep_check_pass_rate`
  threshold becomes `N/N` where N is the surface size.
- Step 3 (N=20 random methods) similarly degrades to `min(20, total_methods)`.

The seeded-RNG / Fisher-Yates approach still applies — there's just no
shuffling needed when the sample is the full set.

### DocFX-style vendor rules (Tekla Tedds, others using DocFX)

DocFX is Microsoft's open-source replacement for Sandcastle. Many vendors who
migrated off Sandcastle now use DocFX. Reviewers processing DocFX vendors:

- **Title order is reversed.** DocFX renders `<h1>Class <Name></h1>` (kind word
  leads), Sandcastle renders `<h1><Name> Class</h1>` (kind word trails). The
  shared `verify-types-strict.ps1` script accepts both orderings.
- **Alternate kind spellings.** DocFX uses `Enum` (Sandcastle: `Enumeration`)
  and `Struct` (Sandcastle: `Structure`). Reviewer's `$kindWords` set must
  accept both.
- **Constructor pages bundle all overloads.** Multiple `<h4 data-uid="...#ctor(args)">`
  blocks on one page; the parser uses the type-page row text as disambiguator.
- **Sidebar tree is the URL source of truth.** Content-hash xref URLs (the
  `/topic/...` style) 302-redirect; canonical `/doc/<vendor>/<ver>/<slug>-<id>`
  URLs live only in the sidebar tree.

### Hybrid OpenAPI/markdown vendor rules (IDEA Statica, other REST-first vendors)

Vendors that ship a REST API + SDK use OpenAPI specs as canonical type+signature
source, with markdown SDK docs supplying prose. Reviewers processing these:

- **No HTML to scrape.** Vendor source URLs in IR point at `.yaml` / `.md`
  files (often github raw URLs). The strict-verify script's
  `$isMarkdownSource` branch dispatches to YAML-aware / markdown-aware checks.
- **Tag-based API-class synthesis.** OpenAPI's `tags: [Calculation]` becomes
  the SDK's `CalculationApi.cs`. The extractor synthesizes `<Tag>Api` classes
  with the tag's operations as `Async`-suffixed methods.
- **Async-suffix mismatch.** Markdown's `## **MethodAsync**` (with Async)
  vs `<a id="method">` (no Async). Reviewers must strip the `Async` suffix
  before lookup.

### mkdocs-material / mkdocstrings vendor rules (Allplan PythonParts, other Python SDKs)

mkdocs-material (with the mkdocstrings plugin) is the Python-SDK community's
canonical docs generator. Allplan PythonParts uses it; other Python-first
vendors (Grasshopper-on-Rhino's Python API ports, FreeCAD, Blender) also
gravitate to it. Reviewers processing mkdocs-material vendors must apply these
rules:

- **Class title shape.** The class heading carries the bare type name inside
  `<span class="doc-object-name doc-class-name">TypeName</span>` — NOT
  `TypeName Class` (Sandcastle) or `Class TypeName` (DocFX). The strict-verify
  script's `$isMkdocsSource` branch reads the type name from this span; do not
  attempt to strip a kind word.
- **Two template variants in production.** mkdocstrings shipped a major DOM
  refresh between 2024 and 2025; Allplan publishes BOTH variants at
  `/2024/` and `/2025/` URLs. Reviewers must detect and handle both:
  - **2024 legacy.** Member headings use `<h3 class="doc-heading">`; the full
    signature lives in the `<code>` element inside that heading. Properties,
    returns, and raises are inside `<table>` rows. Overload groups wrap their
    variants in `<div class="doc-function-overload">` with nested
    `<div class="doc-contents doc-contents-overloads">` children.
  - **2025 current.** Member headings promote to `<h2 class="doc-heading">`
    with `<span class="doc-object-name doc-function-name">name</span>` (name
    only, NO signature). The signature lives in a sibling
    `<div class="doc-signature highlight"><pre><code>…</code></pre></div>`.
    Properties / returns / raises rendered as `<ul><li class="doc-section-item field-body">`.
    Overload variants are sibling `<div class="doc doc-object doc-function doc-function-overload">`
    elements under the same `<h2>` — there is NO outer wrapper.
- **Detection heuristic.** Presence of `<p class="doc doc-class-full-path">`
  on the class page distinguishes 2025 from 2024 (which uses a plain
  `<p>Class full path: <code>FQ.Name</code></p>` line). The strict-verify
  script branches on this signal.
- **Inner classes are promoted to top-level IR types.** mkdocstrings renders
  nested classes under an `<h2>Classes</h2>` section inside the parent's
  `<div class="doc-children">`. The extractor promotes each inner class to a
  separate IR type with `@namespace = "<parent-fq-name>"` (Allplan convention).
  Reviewers verifying Step 1 enumeration must walk the parent's class block
  for inner-class headings, NOT just the top-level class pages.
- **Strict-verify branch.** The `verify-types-strict.ps1` script needs an
  `$isMkdocsSource` branch that accepts the mkdocs-material markup forms
  above (both the 2024 and 2025 variants). The branch dispatches on the
  detection heuristic above.

### Sandcastle-style vendor rules (Rhino, Grasshopper, AutoCAD, Navisworks)

Sandcastle (the Microsoft documentation generator) is used by .NET-focused
vendors. Its DOM has conventions that differ from Tekla's. Reviewers
processing Sandcastle vendors must apply these rules:

- **Enum rows have no anchor in td[1].** `enumMemberList` rows are plain
  text (icon | name | value | summary), not `<a>` elements. A reviewer
  requiring an anchor will misread enum types as having 0 enum_values
  and produce false-positive Step 2 FAILs. Read enum row names from
  `td:nth-child(2)` text content.
- **Filter `[Missing <summary>…]` placeholders.** When the vendor's XML
  doc comment is missing, Sandcastle renders a red-styled
  `<p>[Missing &lt;summary&gt; documentation for "M:..."]</p>` inside
  `div.summary`. The extractor correctly skips these; the reviewer's
  Step 3 must mirror that filter (treat as "vendor has no prose").
- **External / inherited member filter.** Sandcastle exposes inherited
  members from `System.Object` (Equals, ToString, GetType) in the type
  listing tables with `href` pointing to `docs.microsoft.com` /
  `learn.microsoft.com`. The extractor SKIPS these. The reviewer's
  Step 1 enumeration must apply the same filter, OR over-counts the
  vendor surface by 1-6 rows per type.
- **Multiple member tables fold into one catalog list.** Sandcastle has
  `operatorList` (folds into `methods[]`), `fieldList` (folds into
  `properties[]`), `conversionList` (folds into `methods[]`). Step 2's
  deep-check must read all of these and merge per the IR convention
  before comparing against the catalog.

### Known-legitimate exemptions (false-positive whitelist)

Some property types are genuinely `object` per language contract —
flagging them as parser placeholders is a false positive. The reviewer
maintains a whitelist of these patterns:

- `.SyncRoot` properties on `.NET` `ICollection`/`IEnumerable`
  implementations: the BCL contract returns `System.Object`. Tekla
  vendor pages render these as `Type: Object`. **Not a parser bug.**
- Reflection-style properties (`.Tag`, `.UserData`, `.Item`) that the
  vendor documents as `object`-typed.

A `type="object"` property only counts as a deep-check failure when the
vendor page renders a non-`Object` type. Reviewers MUST cross-check the
specific vendor URL before flagging.

## Step 3 — Behavioral spot-check (N=20 random methods)

Verify that prose carried over from vendor docs — not just shapes.

1. From the 50 deep-checked types, pick 20 random methods across them
   (same seeded RNG continues; do not re-seed).
2. For each method, check that the catalog's `remarks` and `summary`
   are non-empty whenever the vendor page has corresponding prose.
3. **PASS** if `behavior_present_rate == 20/20`. A method whose vendor
   page genuinely has no prose counts as present (record it as such).

## Step 4 — Schema validation

Verify every artefact is well-formed.

1. Validate the **IR file** at `cli-sidecar/Ingest/Output/<host>-<version>.ir.json`
   against `cli-sidecar/Ingest/Schema/host-coverage.schema.json` (the
   root-level schema with `host`/`host_version`/`source`/`types`/`metadata`).
2. Validate **every file under `catalog/`** against
   `cli-sidecar/Ingest/Schema/host-coverage-type.schema.json` (the
   single-Type fragment schema). Each catalog file holds exactly one
   `Type` — the root IR schema does NOT match catalog files, by design.
3. Validate any other JSON artefacts the agent ships (none for v0.30).
4. **PASS** if `schema_violations == 0` across all artefacts. Report
   every offending file with the violating path and the specific
   schema-required field that was missing or malformed.

## Output: COVERAGE-REVIEW.md

Write the report exactly in this shape. The reviewer commits it as
`20-agents/aeco/<vertical>/<host>-<version>/COVERAGE-REVIEW.md`.

````markdown
# Coverage review — <agent-id>

**Verdict:** PASS | FAIL
**Reviewer:** codex-rescue, run <timestamp>
**IR source:** <path> (sha256: <hash>)

## Step 1 — Type enumeration
- Vendor index: <URL>
- Vendor type count: <N>
- Catalog type count: <N>
- **missing_types_count:** 0 ✓

## Step 2 — Deep-check (50 random types)
- Seed: <ir-sha>
- Sampled: <comma-list of 50 names>
- **deep_check_pass_rate:** 50/50 ✓

## Step 3 — Behavioral spot-check (20 random methods)
- **behavior_present_rate:** 20/20 ✓

## Step 4 — Schema validation
- Files validated: <N>
- **schema_violations:** 0 ✓

## Re-run command
`aware coverage review <agent-id>`
````

On FAIL, replace each checkmarked metric with the failing count and add
a subsection enumerating each defect (missing types, mismatched members,
empty prose for documented methods, schema-violating files).

## On failure

When any step fails, `codex-rescue` emits the FAIL report with all
missing types and mismatched signatures listed in full. The extraction
subagent (Phase B) reads the report, fixes its extractor, regenerates
the IR, regenerates the agent, and `codex-rescue` re-runs the four
steps. Loop until PASS.

The loop has no retry cap by design — the gate is correctness, not
attempt count.
