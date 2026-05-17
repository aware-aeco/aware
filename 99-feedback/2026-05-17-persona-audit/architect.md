# Persona Audit — Architect (Revit / AutoCAD / Allplan)

**Verdict (three sentences).** AWARE has a genuinely interesting thesis and a credible-looking shipping cadence, but the architecture-side agents (`revit-2026`, `autocad-2026`, `allplan-2025`, `dynamo-4-1-1`, `sketchup-2026`) are NuGet-reflection dumps — 7,600+ method-level commands wired one-to-one to API endpoints with no workflow surface above them. The "60-second app" the manifesto promises works for the Tekla→Trimble Connect example because that side has been *curated* by hand (31 craft skills); on the architecture side, all I see is a flat alphabetical list of `view-sheet-create`, `revision-cloud-create`, `view-schedule-export` with no notion of *issuing a sheet set*, *running a code check*, *bumping a revision*, *generating a transmittal*, or *posting a comment back to BIM360 Docs from a Bluebeam markup*. The substrate could absolutely be the thing — but the architecture coverage today is at the level of "we exposed the API," not "we did the work an architect does."

**Top three gaps (in order of pain):**
1. **No workflow primitives.** Every Revit-2026 command is a single API method. There is zero `sheet.issue`, `revision.bump`, `transmittal.generate`, `schedule.export-to-excel`, `pdf-set.stamp`. The architect's mental units don't exist.
2. **No Bluebeam, no Newforma, no Procore (architect side), no NBS/Avitru, no Aconex, no BIM 360 Field, no Microsoft Project.** I get an ACC Issues agent, but the tools I actually live in for redlines, document control, project transmittals, and specs are absent.
3. **No safety contract on the live model.** "Stateful: false" on the Revit agent means it has no concept of *transaction*, *worksharing*, *checked-out element*, *sync-with-central*, *undo*, or *unsaved changes*. I'm not letting that touch a project file on Thursday at 3pm when CDs go out Monday.

**One app I would build next week** (assuming gaps closed): **`sheet-status-monday`** — every Monday at 7am, open my active Revit project (read-only, no transaction), enumerate `ViewSheets`, pull `Sheet Number / Sheet Name / Current Revision / Sheet Issue Date / Drawn By / Checked By / Sheet Issued For`, group by current revision, dump a Markdown table into a Teams channel + email the PM. No writes. Pure read. I'd run that one Monday and I'd be sold.

---

## A. First-impression friction

The docs are *unusually disciplined* for a young open-source project — the **decalog** is the kind of thing I wish more software shipped with. "App = text. AI = runtime. Open source is structural." Those are tight, falsifiable claims. The manifesto leans Silicon Valley in places ("welcome to what comes after software"), but the spec docs immediately back it up with a real `.flo` file you can read in Notepad. That earns trust.

What lost me:

- **The 60-second demo is a fiction.** Look at the manifesto's terminal block — `npm install -g @aware-aeco/cli`, then a chat command, then a real Tekla→Trimble Connect integration. The `cli/README.md` says the CLI is at v0.6.0, distribution-released, all v0.1–v0.5.2 phases shipped. Fine. But there's no `aware build app` demoed end-to-end against the *Revit* agent — and that's the agent that would matter to me. The demo works *because* the Tekla agent has 31 hand-written workflow skills (`drawing-identity.md`, `drawing-workflows.md`, `event-threading.md`). Revit-2026 has 46 skill files all named like `autodesk-revit-db-architecture.md` whose entire content is "API reference for namespace X" plus a flat list of class names. That is not a craft surface; that is a Doxygen index in YAML.
- **`git clone` → run one app in 10 minutes? No.** The README's install paths (npm wrapper, MSI, curl-pipe, PowerShell) are credible. The Rust source build is fine. But "run one app" requires (a) `aware connect` to a real OAuth-protected endpoint, (b) a real `aware-revit-2026.exe` transport binary on PATH, and (c) Revit actually open with my model. The reference apps shipped in `30-apps/_examples/` are both **Tekla-flavored** (welded-to-tc, qa-drawings-to-tekla). There is **not a single Revit app** in `_examples/`. That's the gap I have to cross alone the first time.

Specific quote that made me wince: *"Decades of AECO knowledge stops dying with the people who hold it."* The pitch is right; the execution gap is that **Tekla has 31 curated skills written by someone who clearly knows Tekla,** and **Revit has 46 auto-generated namespace stubs.** Today AWARE is preserving Tekla knowledge. It's not preserving Revit knowledge. The manifesto promises both.

## B. The "no-code" claim

A `.flo` is plain text. I can read it. I cannot *write* one from scratch — and that's fine, the manifesto admits the AI composes it. So the real question is: **can I copy-paste-modify the examples to fit my workflow?**

For my workflow — no, not yet, and the YAML isn't the problem.

I looked at `welded-to-tc.flo`. The shape is fine. Three nodes, two edges, sane templating: `{{ tekla-watch.mark }}`. A registered architect can read that and roughly understand what's happening — it's no harder than reading a Dynamo graph someone built. The `inline:` glue block with `code: |  e => e.type == "Welded"` is the only programmer-friendly bit, and even there the `description:` field saves it.

The problem is the **node vocabulary I'd need to compose against doesn't exist as workflow primitives**. To copy-paste-modify into "watch my Revit project and notify me when any sheet hits revision X," I need a node that says `revit.sheets.watch-revisions` — not `view-sheet-get-current-revision` (which is one Revit API method called once on one sheet). I'd have to compose a loop, a filter, a grouping, an idempotency check, all inside the `.flo` — that's not no-code, that's writing a program in YAML.

The YAML itself is Revit-architect-friendly *only* if the nodes it references are themselves architect-friendly. Today the architect-side nodes are at the API-method level. That's programmer-friendly.

## C. Coverage gaps — architecture / docs / specs

This is the section that will determine whether AWARE ships into my office.

### Missing tools (priority ordered)

1. **Bluebeam Revu / Bluebeam Cloud (Studio Sessions).** Every redline I do, every owner markup I receive, every consultant comment on a PDF set lives here. Bluebeam has a REST API (Studio Prime) and a (limited) command-line. Not in AWARE. *This is the #1 missing tool for any working architect.*
2. **Newforma Project Center.** AIA-firm RFI/submittal/transmittal logging. Some firms run their entire CA phase off Newforma. Not in AWARE.
3. **NBS Chorus / NBS Source / Avitru / SpecLink.** Where my **specifications** live. CSI MasterFormat 50-division spec docs do not live in Revit, they live in spec authoring tools. AWARE has **nothing** for specs. The word "spec" appears in the Revit manifest only as part of method names like `revit-2026-some-thing-set-specified-foo`. There is no spec-coordination agent. (See scenario 5 below.)
4. **Procore — architect side** (Drawings, Submittals, RFIs, ASIs). ACC Issues is in AWARE but Procore isn't, and many GCs I work with run Procore, not ACC.
5. **Aconex (Oracle).** UK and international document control. Whole BD-funded UK projects run on Aconex.
6. **Microsoft Project / Smartsheet / Asana** for the design programme.
7. **Adobe InDesign / Affinity Publisher** for presentation decks. (Scenario 9.)
8. **BIM 360 Field / Autodesk Build Field Management** — punch list on tablets.
9. **AutoCAD `acad.exe` itself**, as a real automation host. The `autocad-2026` agent is 13,000+ lines of reflected ObjectARX commands — but ObjectARX is C++ and we mostly script AutoCAD via LISP, .NET, or COM. None of those are clearly mapped to invocable commands here.
10. **Deltek Vantagepoint / Ajera** — invoicing and time-tracking on the project. (Less urgent, but a real architect spends 1 day/week on this.)

### Are the Revit / AutoCAD / Allplan agents thin?

They are **wide and thin** — the worst combination. 7,647 commands in Revit-2026, 13,351 lines of AutoCAD-2026 manifest, 1,200 Allplan commands, 1,713 Dynamo commands, 1,713 SketchUp Ruby methods. Volume looks impressive. But:

- **Sheet ops:** `view-sheet-create`, `view-sheet-create-placeholder`, `view-sheet-get-current-revision`, `view-sheet-get-all-revision-cloud-ids` — yes, exposed. But "issue this sheet set to Bluebeam Studio as a stamped PDF and notify consultants" is **not** a node. It's 17 nodes you'd have to compose by hand.
- **View ops:** I see `view-schedule-export`, `view-schedule-create-sheet-list`, etc. Method-level. No `views-by-discipline` or `views-by-area` filters. No "duplicate-with-detailing-for-presentation."
- **Schedule ops:** Method-level only. No `schedule.find-rows-with-missing-fire-rating` (scenario 6). No `schedule.export-to-csi-format`. Just raw `ViewSchedule.Export` with format flags.
- **Family ops:** I checked the family-related commands — they're a wall of `FamilyInstance.*`, `FamilySymbol.*`, `Family.*`. No `family.find-unused` or `family.purge` or `family.audit-naming-against-standard`. Those are the actual ops a Revit lead does in QA week.
- **Parameter ops:** Same pattern. `parameter-set`, `parameter-get`, `shared-parameter-file-load`. No `parameters.audit-against-project-template` or `parameters.bulk-write-from-excel-keyed-by-mark`.
- **Link ops:** I see link CRUD methods but **nothing** about *coordination workflows*. Reload-all-links-and-report-status doesn't exist as a node.

The fundamental issue is **the agent-builder reflects, it doesn't curate.** The `revit-2026` manifest says it's auto-generated from `Autodesk.Revit.SDK@2026.0.0.9999`. The provenance block says `generator-version: 0.8.0`. The description even admits *"Marked stateful: false because raw reflection has no way to identify the session-start method; curators flip lifecycles when refining."* The plan is right (curate later); the curation hasn't happened.

### Specs — addressed at all?

No. Grep for `csi` returns nothing in `revit-2026/manifest.yaml` except as a substring inside random method names. There's no NBS agent, no Avitru agent, no spec-section template. For a firm that runs MasterFormat-keyed specs (every U.S. firm, basically), this is a hole the size of half the deliverable.

## D. Workflow primitives I wish existed

In order of how often I'd touch each:

| Primitive | Today it would take | Should be |
|---|---|---|
| `revit.sheet-set.list` (filter by IssuedFor / Revision / Discipline) | 5 chained API calls + a filter | one node, one config block |
| `revit.sheet-set.export-pdfs` (with PDF/A, color, embed fonts) | N/A — must go through Revit's `PrintManager` which is a 30-line dance | one node |
| `revit.sheet-set.stamp` (drop a transmittal stamp + date + revision on each page) | N/A | one node |
| `revit.revision.bump` (next revision per sheet, set per-sheet revision date, set issued status, lock revision) | 6+ method calls per sheet | one node, scoped by selection |
| `revit.transmittal.generate` (AIA G706-style transmittal sheet, log to ACC/Newforma/Bluebeam Studio) | N/A | one node |
| `revit.schedule.qa` ("find rows missing fire rating in door schedule") | scenario 6, see below | one node |
| `revit.worksharing.check-state` ("is this model checked out by me / synced / locked?") | scenario E, see below | mandatory pre-flight on every write node |
| `revit.element.by-parameter-value` ("find all walls where Fire Rating is null") | a `FilteredElementCollector` + a lambda | one node |
| `revit.area-calc.export` (NIA / GIA per the local building code) | huge | one node — different per market (RICS in UK, BOMA in US) |
| `acc.folder.permission.audit` ("who can see this folder?") | 30 endpoints in acc-account-admin, none composed | one node |
| `cde.sync` (pull latest of every link from ACC/TC/Aconex and notify the team) | N/A | one node |
| `bluebeam.studio.pull-comments` (get all markups from a Studio Session as a CSV with disposition status) | not in AWARE at all | one node |
| `pdf.stamp.batch` (date, project, revision, sheet number; PDF/A output) | not in AWARE | one node |
| `permit.bundle` (drawings + area calcs + accessibility statement + fire strategy, named per local authority's format) | scenario 10 — multi-day exercise | one node, parameterized by authority |
| **`abort-if-not-signed-off`** | not enforceable in AWARE today | a built-in **gate node** that any write app must pass through |

That last one matters. Production document control says **you do not write to the live model unless the sheet you're touching is not currently in an issued-for-construction state, OR you have an authorized variation order.** AWARE has no notion of this guard.

## E. Trust + safety

This is where AWARE either wins or dies for me.

**Would I let it write to my live Revit model the day before deadline?** Absolutely not. Here's why:

- **No transaction concept on the Revit agent.** A Revit API write must happen inside a `Transaction` and committed or rolled back. AWARE's command list shows `transaction-start`, `transaction-commit`, `transaction-rollback` as separate raw API calls — but the *composition primitive* of "wrap this whole node's effect in one undoable transaction group, and if anything in the DAG fails, rollback the lot" is not in the app-spec. The undo stack and Revit's `TransactionGroup` are the only thing standing between a script and a destroyed model. **AWARE has to make transactions a first-class composition concept**, not a leaf-API call.

- **No worksharing awareness.** The manifest has `worksharing-utils-*` methods exposed, but the spec has no rule that says *"a write-mode node MUST first check the element's checkout owner, refuse to proceed if owned by someone else, and emit a structured error to the trace."* On any real project of mine, two people are in the central file at the same time. This is non-negotiable.

- **No backup contract.** The spec doesn't say *"before a write node runs against a Revit model, eTransmit-or-snapshot the central file to `~/.aware/snapshots/<app>/<timestamp>/`"*. That's exactly what Revit's own `Save and Synchronize` workflow does implicitly via central-file backups. AWARE should require the equivalent or refuse the run.

- **No audit trail surfaced to the user.** OK, I see the spec says runs write JSONL traces to `~/.aware/logs/<app>/<instance>/<run>.jsonl`. That's good for debugging. It is **not** an AIA-defensible audit trail. When the GC says "your script changed the dimension on Sheet A-201" three months later, I need a per-element, per-user, per-transaction log that links to the Revit `ChangeHistory` and is signed. AWARE's provenance is a developer log, not a record-keeping log.

- **No "dry run" mode.** I want every write-app to support `--dry-run` that prints the proposed transaction list without committing. The spec mentions `aware app run` and `aware app logs --replay` — the latter replays a past run; it doesn't *preview* a proposed one. Different thing.

The decalog says `closed apps that wrap an LLM are training their replacements`. Fine. But **the live model is sacred.** I would happily run AWARE *read-only* against the model and let it write to PDFs, Excel, email, ACC, Bluebeam — anything *outside* the .rvt. Crossing into the model itself needs a contract I haven't seen in the docs.

## F. The killer feature

**Cross-project sheet-and-revision status with consultant ball-in-court tracking.**

I work three to seven active projects at a time. Each is a separate `.rvt` central. Each has its own ACC folder. Each has its own consultant team in their own CDE. Right now, **on Monday morning I open seven things and copy data into a Smartsheet** to answer the principal's standing question: *"What's the state of all live projects, who's holding what up, what's at risk?"*

If AWARE could run on a schedule, walk all seven projects, pull current revision per sheet, cross-reference outstanding RFIs in ACC Issues, cross-reference outstanding consultant submittals in TC / Aconex, and produce **a single Teams card every Monday at 7am** with "Project X: 4 sheets at Rev C awaiting client sign-off, 2 RFIs > 5 days old assigned to MEP consultant, schedule shows 12 days to 50% CD milestone, here's the risk list" — that is a 4-hour-per-week task I'd buy a $400/month subscription to delete.

It's also the **kind** of app AWARE was designed for: read-only against the model, multi-tool composition (Revit + ACC + TC + MS365), no destructive writes, runs locally on my workstation so no project IP leaves. The substrate's strengths line up exactly.

## G. The 5-Mondays adoption test

**Today (without gaps closed) — these mostly don't work:**

1. *Monday sheet status* — half-works. Revit-2026 has `view-sheet-get-current-revision`. Composing it across many sheets, then emailing the PM, is a YAML-coding exercise.
2. *Code-compliance pre-check* — fails. No "egress window" or "natural light" semantic. Would have to build the rules from scratch.
3. *Sheet issue* — fails. No `sheet-set.export-pdfs`, no stamp, no transmittal. Each step is an API method, the composition is gigantic.
4. *RFI generation from clash* — works only because `acc-issues.create-issue` exists. Navisworks → AWARE is missing entirely (no Navisworks agent listed).
5. *Specification round-trip* — fails. No NBS/Avitru, no spec primitives.
6. *Schedule QA* — barely works. `view-schedule-export` to CSV exists; the QA logic would be inline glue.
7. *As-built handover* — fails. Multiple CDEs, multiple file formats, multiple naming conventions per client.
8. *Code-version migration* — fails. No code-rule layer at all.
9. *Design option deck* — partial. Image export exists. PowerPoint/InDesign assembly doesn't.
10. *Permit-set assembly* — fails. Authority-specific bundling unknown.

**With gaps closed — Five Mondays I'd want to schedule:**

- Monday 1: `sheet-status-rollup` (the killer feature above)
- Monday 2: `rfi-aging-report` (pull from ACC Issues + TC topics, weekly Teams post)
- Monday 3: `consultant-ball-in-court` (cross-CDE submittal aging)
- Monday 4: `schedule-qa` (every door / window / floor-finish schedule, find missing fields, dump to Excel)
- Monday 5: `weekly-issue-bundle` (the Friday-evening tender/IFC export pack — PDFs stamped, named per project standard, dropped in the consultant folder)

If AWARE gave me three of those five reliably, I'd be using it weekly within a month.

## H. The dealbreaker

**The model is sacred and AWARE doesn't yet act like it.**

Specifically: until the app-spec mandates a first-class transaction-group primitive, a worksharing pre-flight, a snapshot-before-write contract, and a `--dry-run` mode, **no architect in their right mind will let an AI runtime touch a live `.rvt` the week of an issue**. The decalog correctly identifies the social and economic argument for opening up software. It is silent on the production-liability argument that has kept architects skeptical of every "AI for AEC" pitch for the last five years.

You don't need to solve this with policy. You solve it with **structure**: make write-mode nodes require an explicit `safety:` block declaring the transaction group, the snapshot path, and the rollback strategy — and refuse to run a write node that doesn't declare it. That single rule would make AWARE the first AI-for-AEC tool I'd recommend without caveat.

The secondary dealbreaker — and it's catching up to the first — is **the architecture-side coverage gap**. Tekla has been done properly; Revit / AutoCAD / Allplan have been reflected but not curated. The manifesto promises both. Today the substrate delivers the structural side beautifully and the architecture side as a parts catalog. If the next two roadmap phases focused on hand-curating 20–30 *workflow* skills per architecture agent (the way Tekla has been), and on adding Bluebeam + Newforma + Procore-architect + NBS as first-class agents, AWARE would close that gap fast.

Until then: read-only, against my data, in a sandbox project, on a quiet afternoon — sure. Live writes to a CD-phase model — not a chance.

---

**One closing observation for the team.** The decalog item *"Closed apps that wrap an LLM are training their replacements"* is correct, and AWARE's bet on open-text composition is the right structural answer. But there is a parallel architectural truth the manifesto hasn't quite stated: **An AI runtime that writes to a vendor's load-bearing data structures and doesn't model the vendor's transactional and worksharing semantics is training its lawsuit.** The Tekla agent's skills (`event-threading.md`, `drawing-identity.md`) get this — they encode the gotchas you only learn after a bad weekend. The Revit agent's skills do not yet. Closing that gap is the work.
