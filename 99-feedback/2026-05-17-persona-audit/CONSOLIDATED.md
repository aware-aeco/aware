# AWARE — Consolidated Cross-Persona Audit · 2026-05-17

Five practitioners audited AWARE end-to-end against the same brief. This document distils what they agreed on, what they disagreed on, and what to do about it.

**Aggregate verdict (across all five):** the substrate's bones are sound — text-based composition, decalog discipline, sensible CLI, hand-curated Tekla agent as proof-of-concept — but the **catalogue is ~25% done relative to the manifesto's pitch**, the **architecture-side agents are reflection-grade not workflow-grade**, the **runtime has no safety contract** for live-model writes, and the **fabricator-downstream half of AECO** (EPM, NC controllers, render queues, spec authoring) is invisible. None of the five would put AWARE in front of a paying client today. All five would pilot it next month with the right gaps closed.

> Verdict by persona:
> - **BIM manager:** Watch but don't adopt.
> - **Designer:** Near-zero adoption — viz side missing.
> - **Architect:** Adopt read-only; live writes need a safety contract first.
> - **Structural engineer:** Won't sign a calc — close the audit envelope first.
> - **Detailer:** Pilot on one job if EPM + signed MSI + UDA audit stamping land.

## What they unanimously called out

These ten findings appeared in **all five** audits, often word-for-word.

1. **Auto-generated agents are unusable as shipped.** Revit-2026 has 7,647 commands; Rhino-8 has 6,954; AutoCAD-2026 13k+ lines; Allplan, Dynamo, SketchUp similarly bloated. Searching is broken; mental models are impossible; AI compositions hallucinate against this surface. *The hand-curated Tekla agent (31 craft skills) is the existence proof that curation works — it just hasn't been done for any other agent.*

2. **Live-model writes are sacred, AWARE doesn't yet act like it.** No `--dry-run`, no rollback / undo, no transaction-group as a first-class composition primitive, no worksharing pre-flight, no snapshot-before-write contract, no per-element audit trail back to a UDA on the model. Five out of five refused live writes.

3. **The `microsoft-365` agent (4 commands) is a stub.** No Teams post, no Outlook send, no calendar event, no Planner task, no SharePoint list query, no Excel range read. This is the **shared comms layer** every persona needs — they all cited it.

4. **The `acc-issues` agent (14 commands) is too thin.** No BCF viewpoint attach, no file attach, no input schemas, no examples. Doesn't reflect how ACC Issues is actually used.

5. **Inline glue is JavaScript-ish lambdas in YAML.** Five out of five flagged `e => e.type == "Welded"` as "not no-code." Real no-code needs named, reusable filter/map/group atoms — not arrow functions a non-programmer can't audit.

6. **Workflow verbs missing.** Every persona reached for a verb that didn't exist: `issue-drawing-pack`, `nc-export-phase`, `sheet-status-rollup`, `peer-review-delta`, `monday-audit`, `capture-named-view`. The substrate has nodes-at-API-method grain, not nodes-at-workflow grain.

7. **No `for-each` / `sweep` / `compare` / `approve` / `schedule` / `abort-if` / `snapshot` primitives.** Detailer, structural engineer, BIM manager, architect all listed these. The DAG model is event-stream-shaped; everyday AECO work needs iteration, comparison, scheduled triggers, and human-in-the-loop gates.

8. **No signed Windows binary.** Three of five flagged this as a hard IT blocker. SmartScreen "unknown publisher" warning kills enterprise rollout dead.

9. **No first-hour onboarding per persona.** The 60-second demo is a fabricator pitch (Tekla → Trimble Connect). Designer / architect / BIM manager / structural engineer have nothing equivalent that runs end-to-end on their tools.

10. **`think-node` / `smart-node` as AI-as-runtime for safety-critical operations is a dealbreaker.** Structural engineer + BIM manager both said this independently. *The LLM composes the plan; deterministic code is the plan.* Decalog should explicitly call this out.

## Where they disagreed

| Topic | Position |
|---|---|
| **Severity of doc style** | Engineer + detailer say docs are honest + disciplined; designer + BIM manager say manifesto reads Silicon Valley |
| **Adoption today, read-only** | Architect + engineer would do this; designer flatly says no; detailer wants signed MSI first |
| **`exposes-as-agent` permission UX** | BIM manager called it out as confusing; others didn't raise it |
| **Aesthetic determinism** (color profile, white balance) | Only designer raised this — but it's important for them specifically |
| **Engineering reproducibility envelope** (code revision, NA, solver hash) | Engineer made this their #1; nobody else surfaced it |
| **EPM agent priority** | Detailer made this their dealbreaker; BIM manager mentioned ACC Docs which is the architect-side equivalent; engineer + designer + architect didn't list it |

## Cross-cutting tools they want — ranked

Aggregated demand. Numbers in `()` = how many of 5 personas explicitly named it.

| Rank | Tool | Cited by | Category |
|---|---|---|---|
| 1 | **Navisworks Manage 2026** | BIM mgr + architect | Coordination |
| 2 | **Bluebeam Revu / Studio** | Architect + BIM mgr + designer | PDF / markup |
| 3 | **Solibri Office** | BIM mgr + architect | IFC validation |
| 4 | **BCF 3.0 file format** (read+write) | BIM mgr + architect | Coordination exchange |
| 5 | **IFC validator / inspector** (IfcOpenShell-style) | BIM mgr + architect + detailer | Format-level |
| 6 | **Twinmotion 2025 / Enscape 4 / V-Ray 7** | Designer | Real-time viz |
| 7 | **Tekla EPM / FabSuite / Stratus / Hera** | Detailer | Fabricator ERP |
| 8 | **Peddinghaus / Voortman / Trumpf controllers** | Detailer | Shop-floor CNC |
| 9 | **NBS Chorus / Avitru / SpecLink** | Architect | Specs |
| 10 | **Newforma Project Center** | Architect | Doc control |
| 11 | **Procore (architect side)** | Architect + BIM mgr | Doc control |
| 12 | **Aconex** | Architect + BIM mgr | Doc control |
| 13 | **ACC Docs** (vs Issues only today) | BIM mgr | CDE — files side |
| 14 | **RAM Structural System** | Engineer | FEA |
| 15 | **SCIA Engineer** | Engineer | FEA |
| 16 | **Autodesk Robot (RSA / ARSAP)** | Engineer | FEA |
| 17 | **RFEM / RSTAB** (Dlubal) | Engineer | FEA |
| 18 | **RISA-3D / midas / Strand7** | Engineer | FEA |
| 19 | **Ladybug / Honeybee / ClimateStudio** | Designer | Sustainability |
| 20 | **RhinoInside.Revit** | Designer | Interop |
| 21 | **Hilti PROFIS Engineering** | Engineer | Anchor design |
| 22 | **PLAXIS / gINT / Slide2** | Engineer | Geotechnical |
| 23 | **BIM 360 / Build Field Management** | Architect + BIM mgr | Field |
| 24 | **Advance Steel / SDS/2 / ProSteel** | Detailer | Detailing alternatives |
| 25 | **InDesign / Affinity Publisher** | Designer + architect | Presentation |

## Cross-cutting workflow primitives they want

Aggregated. Numbers in `()` = how many personas listed it.

| Primitive | Citations | Notes |
|---|---|---|
| **`for-each`** over a static list | 4 / 5 | Stream-based DAG doesn't cover this |
| **`compare` / `diff`** two snapshots | 4 / 5 | Heart of design review + peer review |
| **`approve` gate** (human-in-the-loop) | 4 / 5 | Slack/Teams button, wait for response, branch |
| **`schedule` / `cron`** trigger | 3 / 5 | Monday 7am report flows |
| **`abort-if` / `assert`** | 3 / 5 | Pre-flight gates on writes |
| **`snapshot` / `freeze`** model state | 3 / 5 | Peer review + reproducibility |
| **`sweep`** parameters → table | 2 / 5 | Engineer + designer |
| **`screenshot` / `capture-named-view`** | 3 / 5 | Universal — every Teams/Slack post wants one |
| **`pdf-stamp.batch`** (date, revision, project) | 3 / 5 | Architect + detailer + BIM mgr |
| **`model-lock` / write-budget** | 2 / 5 | Detailer + architect |
| **`undo-token` from destructive commands** | 2 / 5 | Detailer + architect |
| **`signed-output`** (digital seal chain-of-custody) | 1 / 5 | Engineer — but blocks PI insurance |
| **`unit-discipline`** (typed numbers) | 1 / 5 | Engineer — but prevents `4.0` vs `4.0 kN/m²` |
| **`bcf-round-trip`** | 1 / 5 | BIM manager — but it's the IFC of coordination |
| **`idempotency`** contract per command | 2 / 5 | Trimble Connect has it; others don't |
| **Named reusable predicates** (vs inline JS) | 2 / 5 | Designer + detailer |

## The killer apps they each named

Each persona named one "next-week app" they would ship if their top gaps closed. These are the **adoption wedges** for each professional segment:

| Persona | Killer app | Estimated value |
|---|---|---|
| BIM manager | **Monday model audit** — 4 Revit centrals, full health report to Teams + Excel + PDF | Replaces Naviate + ½-day Monday ritual |
| Designer | **Show Me** — diff this week's Rhino changes, 3 consistent renders, delta map, Teams post | Replaces Sunday-night "what did I change" panic |
| Architect | **Cross-project sheet+RFI ball-in-court** — 7 active projects, single Monday 7am Teams card | "$400/month subscription to delete a 4hr/week task" |
| Engineer | **Peer-review-delta** — TSD model vs snapshot, member-by-member delta report | 100 engineer-hours/year reclaimed |
| Detailer | **Issue-drawing-pack** — Phase + Rev → PDF + DWG + NC + bolt list to TC, all stamped + logged | Wednesday-afternoon panic → 1 command |

**All five killer apps are read-mostly.** None requires write-back to a live model. All five are blocked today by missing curated workflow verbs + a missing comms primitive (Teams post / ACC Issues create-with-attachment / PDF stamp). **This is where to start.**

---

## Prioritised roadmap proposal

Based on the audit, here's what to ship next. Phases sized for ~2-3 week each (matching past CLI roadmap velocity).

### P0 — Adoption blockers (no AECO firm adopts without these)

| Phase | Scope | Why |
|---|---|---|
| **v0.10 — Workflow-verb curation pass** | Take Revit-2026 from 7,647 commands → 50 curated `sheet.*`, `view.*`, `schedule.*`, `family.*`, `parameter.*`, `link.*` verbs with worked examples. Same for Rhino-8, AutoCAD-2026, Allplan-2025, Tekla curated agent. Promote skill content to commands. | Unanimous gap #1. Catalogue catches up with manifesto. |
| **v0.11 — Safety contract** | App-spec mandates `safety:` block on write-mode nodes — transaction-group, snapshot path, rollback strategy, worksharing pre-flight, UDA audit stamp. CLI refuses to run write nodes without it. Plus a `--dry-run` mode and `aware app explain <name>`. | Unanimous gap #2. Unblocks all live-model adoption. |
| **v0.12 — Microsoft-365 depth** | Teams post (with screenshot/file), Outlook send-mail, Planner task, Calendar event, SharePoint list, Excel range read. Mirror for Google Workspace. | Every Monday-morning report needs this. Universal across personas. |
| **v0.13 — Signed Windows MSI + SSO-friendly auth** | Authenticode-signed MSI (paid cert), MDM-deployable, OAuth via existing tenant SSO not dev-console tokens. | IT departments block unsigned MSI; SSO is non-optional. |

### P1 — Coverage gaps (close the manifesto promise)

| Phase | Scope | Why |
|---|---|---|
| **v0.14 — Coordination toolchain** | Navisworks Manage 2026 + Solibri Office + BCF 3.0 file-format agent (read+write) + IFC inspector (IfcOpenShell-style) | BIM-manager dealbreaker. Half of AECO work is coordination. |
| **v0.15 — Document control** | Bluebeam Revu Studio + Procore (architect side) + Aconex + ACC Docs (not just Issues) | Architect + BIM manager dealbreakers. |
| **v0.16 — Fabricator downstream** | Tekla EPM (PowerFab) agent + Peddinghaus Direct Tools / Peddimat translator + NC-export-phase verb on Tekla agent | Detailer dealbreaker. Unlocks fabrication half of AECO. |
| **v0.17 — Visualization output** | Enscape 4 + Twinmotion 2025 OR honest manifesto: "AWARE handles model prep + frame routing; render outside." Plus `capture-named-view` + `render-queue` primitives. | Designer dealbreaker. |
| **v0.18 — Spec authoring** | NBS Chorus / Avitru + CSI MasterFormat primitives | Architect gap. Half the deliverable for US firms. |

### P2 — Substrate primitives (the app-spec gets more powerful)

| Phase | Scope | Why |
|---|---|---|
| **v0.19 — Workflow primitives** | `for-each`, `compare`/`diff`, `sweep`, `approve` (Slack/Teams gate), `schedule` (cron), `abort-if`/`assert`, `snapshot`/`freeze`, `model-lock`, `pdf-stamp.batch`, `screenshot` | All 5 personas listed multiple of these. App-spec extension. |
| **v0.20 — Named reusable atoms** | Library of `is-ready-for-issue.predicate`, `for-each-named-view`, `with-camera`, `at-resolution`, etc. Replace inline JS glue. | "No-code" claim becomes honest. |
| **v0.21 — Engineering envelope** | Code-revision pinning (EC3:2022+UK NA, ASCE 7-22, IS 800:2007). Solver-build-hash in provenance. Model-schema-version pin. `signed-output` + digital seal hooks. `unit-discipline` typed numbers. | Engineer dealbreaker. Unlocks PE-stamped deliverables. |

### P3 — Adoption / onboarding

| Phase | Scope | Why |
|---|---|---|
| **v0.22 — Persona-specific reference apps + first-hour guides** | One worked end-to-end app per persona: `bim-monday-audit`, `designer-monday-shots`, `architect-sheet-status`, `engineer-peer-review-delta`, `detailer-issue-pack`. Plus a 60-second demo per persona in the manifesto. | The 60-second pitch today is fabricator-only. Designer + architect + BIM mgr have nothing equivalent. |
| **v0.23 — Decalog amendment** | Add an 11th truth: "**AI composes the plan; deterministic code is the plan.**" Restrict `think-node`/`smart-node` from validation, approval, or code-check paths in safety-critical apps. | Engineer + BIM manager dealbreaker. Currently implied; should be explicit. |

---

## The thesis check

Two personas (engineer, BIM manager) independently observed the same architectural truth:

> The decalog tells us *what the substrate is* (text, AI runtime, open source). It doesn't tell us *what the substrate guarantees about vendor data structures*. An AI runtime that writes to load-bearing data (Revit Central, Tekla model, IDEA `.ideacon`, TSD project) without modelling that vendor's transactional, worksharing, code-revision, and audit semantics — is training its own lawsuit.
>
> The Tekla curated agent's skills (`event-threading.md`, `drawing-identity.md`) encode these gotchas. The Revit / Rhino / AutoCAD / TSD / IDEA reflected agents do not. The Tekla approach is the existence proof. The work is replicating that across the catalogue.

That's the engineering bet for the next 6 months. The substrate is right. The catalogue is the work.

## What's already strong (don't break)

Worth recording — the audits also surfaced what's working:

- **Text-based composition.** All 5 read a `.flo` file and understood the shape. Decalog #1 holds.
- **Decalog discipline.** Repeatedly cited as the right shape of project document. "The first time the open-source argument has been made in a way that's structural to the format, not licensed-on-top."
- **Tekla curated agent.** Held up as the gold standard. 31 craft skills, real gotchas, the right grain.
- **`aware report substrate`** + the playground force-graph. Architect called the report "credible-looking shipping cadence." Substrate visualisation tools have done their job — practitioners can see the surface.
- **CLI ergonomics.** All 5 found `aware doctor`, `aware tree`, `aware search` discoverable. The Rust binary is solid.
- **NuGet → agent generator.** A reasonable starting point — produces a catalogue at all. The work is layering curation **on top of** the reflected manifests, not replacing them.

---

**Action for the founder:** v0.10 (workflow-verb curation pass) and v0.11 (safety contract) are the unblocking phases. Ship them as the next two PRs and the audit verdicts move from "Watch" / "Adopt read-only" to "Pilot on a real job." Everything else queues behind these two.
