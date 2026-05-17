# Persona Audit — Steel Detailer (Tekla / IDEA / Trimble Connect)

**3-sentence verdict.** AWARE clearly understands that *AECO* runs on Tekla, IDEA, Trimble Connect and the Office suite — the manifesto is honest, the decalog is short enough to be useful, and the curated `tekla` agent contains real Tekla gotchas (Mark vs Name, `TeklaStructures.Connect()`, `XS_MACRO_DIRECTORY`) that you only learn by losing a Saturday. But the *detailing* workflow — issue drawings, push NC to the saw line, reconcile bolt lists, propagate revision clouds, track mill orders — is essentially absent as first-class capability; what you get are 3,179 raw reflected Tekla API methods and a `RunMacro("ExportNCFiles")` reference inside a skill. That is the difference between a substrate that *could* do my job and a substrate that *does* my job, and right now it's the former.

**Top 3 gaps.**
1. **No NC / DSTV / drawing-issue commands.** The fabricator's two most-used buttons (export NC1 for every part, issue a drawing pack) are not commands on the `tekla` agent. They're hidden behind `Operation.RunMacro("ExportNCFiles")` in a skill.
2. **No fabricator-downstream coverage.** Tekla EPM (PowerFab), Stratus, Hera, FabSuite, Peddinghaus Direct Tools / Peddimat, paint-shop trackers, shipping ERP — none of it exists. Tekla is not the shop floor; the shop floor is *after* Tekla.
3. **No revision / audit-trail primitive.** The decalog talks about provenance, but there's no "supersede previous revision, stamp date, log who issued it" verb anywhere. A drawing issued without a stamp is a £40k mistake waiting.

**Next-week app.** `issue-drawing-pack` — take a list of drawing marks, set status to "Issued for Construction", stamp date + revision + issuer, export PDF + DWG + NC + bolt list at the right naming convention, upload to a Trimble Connect folder, log the operation. That is the one app that solves a Wednesday-afternoon problem.

---

## A. First-impression friction

The manifesto reads honestly. "Apps are text. AI is the runtime. Open source is what the format does automatically." I get it on first read — and the `welded-to-tc.flo` example actually makes it real: three nodes, one inline filter, a `{{ tekla-watch.mark }}` placeholder. I can read that file in Notepad. Good.

But — the manifesto's 60-second demo presents a perfectly tidy "watch welded assemblies → upload drawings" pipeline. *Welded assemblies don't upload drawings.* I issue drawings. Drawings have status flags (`IsReadyForIssue`, `IsIssued`, `IsIssuedButModified`). Drawings have revisions (A, B, C, D, E …) and the moment one revision supersedes another, every downstream consumer needs to know the prior PDF is dead. The demo skips all of that and presents an unconditional "new assembly = new PDF goes to TC" loop. In production that is a recipe for the saw line cutting against revision C while my QC team has revision E open.

The drawing-workflows skill actually documents this exact issue beautifully — `UpToDateStatus`, `IsReadyForIssue`, `IsIssued`. So the *knowledge* exists. But the *demo* and the *commands* don't reflect it. A new detailer reading the README will think AWARE handles drawings; opening the `tekla` manifest shows three commands (`watch`, `insert`, `save-attributes`) and that's it.

10-minute install + first run: I cannot judge until I've installed the binary. The README claims `npm install -g @aware-aeco/cli` works on Windows. Realistic — I have npm because I keep tripping over it as a Tekla developer. Most of my colleagues do not. The MSI is acknowledged as unsigned ("SmartScreen will warn 'unknown publisher'") — that single warning is enough to lose 40% of fabricators. IT departments don't allow unsigned MSI in our environment full stop. The roadmap says code-signing is queued. It needs to be done before pitching this to a fab shop.

## B. The "no-code" claim

The `.flo` file is readable. I can imagine copy-paste-modifying `welded-to-tc.flo` to point at a different folder. Beyond that — no. The moment I need to express "for every drawing where `IsReadyForIssue == true` and `IssuingDate < today - 7 days`, send me a reminder," I'm writing inline glue:

```yaml
- id: filter-stale
  inline:
    kind: predicate
    code: |
      d => d.isReadyForIssue && (Date.now() - d.issuingDate) > 7*86400*1000
```

That `e => ...` is JavaScript. It is code. A detailer with 18 years on Tekla and zero JavaScript will not write that. They will ask "Claude Code" or whatever AI host to write it, which is fine — but the *file they end up shipping* is no longer a thing they can audit. They have to trust an AI told them the truth about millisecond arithmetic. In our world that's where the £40k holes get drilled.

The YAML itself is fine. It's structurally similar to Tekla's own `.objects.inp` or `XS_*` advanced options files — fabricators don't fear text config. Indentation breaking the parse is the same pain as a missing semicolon in a `.tpl` file. Survivable.

What's missing for ergonomics: **named, reusable nodes**. Every time I want a "if drawing IsReadyForIssue" filter, I rewrite the inline glue. There should be a way to define `is-ready-for-issue.predicate` once in a skill folder and reference it. Right now the `.flo` is going to grow ugly fast in any real pipeline.

## C. Coverage gaps in my domain

This is where it falls down hardest.

**The curated `tekla` agent commands: `watch`, `insert`, `save-attributes`. Three.** Every other capability is buried in skills as code snippets the AI is supposed to assemble at runtime. That means every invocation re-derives "how do I export an NC file" from a skill paragraph. For a fabricator who needs to export 800 parts on a Friday afternoon, "the AI will figure out the macro path again" is not acceptable. Each one of these belongs as a first-class command with a documented contract and a test:

- `nc-export` (with format = `DSTV` | `peddimat-asc` | `dxf` | `csv-x,y` per machine)
- `drawing-export` (PDF + DWG, batched, with naming-convention template)
- `report-create` (wraps `Operation.CreateReportFromAll` / `CreateReportFromSelected` — the skill knows about it; the command doesn't expose it)
- `drawing-issue` (sets status, stamps revision, supersedes prior)
- `drawing-list` (filter by status / phase / lot / IsReadyForIssue)
- `assembly-list` / `part-list` / `bolt-list` / `weld-list` (the four bread-and-butter BoMs)
- `numbering-run` (modified objects + all objects, the macro skill mentions both)
- `ifc-export` (with reference-only vs full-model option, for MEP clash)
- `clash-check` (Tekla has it; `events-and-clashes.md` skill exists; no command)
- `udacopy` / `uda-set` / `uda-get` for user-defined attributes — without these I can't track "phase," "lot," "lift week," "paint code," "shipped"

**Auto-generated raw agents.** `tekla-2026` has 3,179 commands like `assembly-add`, `assembly-modify`, `bolt-group-modify`. That's the reflected API surface. It's useful as a backstop — if I need to do something exotic, the API method is documented. But 3,179 leaf-level commands is not a workflow. No detailer is going to compose a "issue drawings" app out of `assembly-modify` + `mark-insert` + `drawing-handler-get-drawings`. The curated agent should expose the **workflow verbs**, with the raw agent as the escape hatch.

Same problem on `idea-statica-26` — 116 commands all of which are `cross-section-factory-fill-rolled-i` / `cross-section-factory-fill-welded-box-flange`. Useful as a catalog reference. Not a workflow. The actual fabricator workflow is "import a `.ideacon` file the engineer sent me into Tekla as a connection" — that is one verb, sitting on top of forty of those calls.

**Things missing entirely:**

- **Advance Steel 2026** — no agent. Half the fabricators in the UK use Advance Steel as a second seat for jobs that started in AutoCAD.
- **SDS/2** — no agent. Big in the US.
- **ProSteel** — no agent. Bentley shops.
- **RAM Connection** — no agent. Connection design alternative to IDEA.
- **Tekla EPM / PowerFab** — no agent. This is the *fabricator's ERP*. Without an EPM agent, AWARE cannot reach the production schedule, the heat treatment tracker, the shipping schedule, the painting station, the cost roll-up. **This is the single biggest gap.** I will go further: an `aware-aeco` substrate without an EPM hook is missing 40% of the fabricator's day.
- **Peddinghaus Direct Tools / Peddimat** — no agent. The saw line and the drill line speak Peddimat (a DSTV dialect with vendor extensions). Tekla's NC export produces DSTV; the Peddinghaus controller wants Peddimat. The conversion is non-trivial. A `peddinghaus` agent that takes a folder of `.nc1` files and emits `.asc` for the saw line is one Monday-morning win.
- **Stratus / Hera / FabSuite** — no agents. Same EPM-shaped hole.
- **Trimble Quadri** — no agent. Mentioned as a wedge but nothing there.
- **Trumpf / Voortman controllers** — no agents.
- **AutoCAD** is there with 4,413 commands but no curated "open DWG, extract block attributes, save". Same problem as Tekla raw.

**Existing Tekla agent — completeness check:**

| Capability | Skill present? | Command present? |
|---|---|---|
| Drawings — read status, identity, types | yes (`drawing-workflows.md`, `drawing-identity.md`) | no |
| Drawings — issue / supersede | partial (status props documented) | no |
| Drawings — export PDF / DWG | yes (in `macro-execution.md` + `drawing-workflows.md`) | no |
| Templates (.tpl / .rpt) | yes (`rpt-report-templates.md`) | no |
| Reports — assembly list / part list | yes (`Operation.CreateReportFromAll`) | no |
| NC / DSTV export | yes (one line, `Operation.RunMacro("ExportNCFiles")`) | **no — biggest gap** |
| Bolt / weld lists | implicit via reports skill | no |
| Organiser | no | no |
| Custom components | no | no |
| Attribute files (.j*) | yes (`save-attributes.md` command exists) | yes (one) |
| Model browser ops | no | no |
| UDA read/write | implicit | no |

The skills are excellent. The commands are anaemic. The agent-spec format itself encourages this — you put "the rule" in skills and "the operation" in commands — but the curators have been mostly writing skills.

**Around the fabricator's downstream tools** — saw / drill / plasma / paint / shipping — zero coverage. This is where AWARE *wins or loses*. The whole reason I'd pay for this is to wire Tekla → EPM → Peddinghaus → paint-shop tracker → shipping in one weekend. Right now I have Tekla → Trimble Connect. That's it.

## D. Workflow primitives I wish existed

Concrete list, in priority order:

1. **`drawing-issue-pack`** — `marks: ["A-100", "A-101", ...]`, `revision: "E"`, `status: "Issued for Construction"`, `naming-template: "{project}-{phase}-{mark}-RevE.pdf"`, `target: trimble-connect-folder-id`. Stamps date + revisor, sets `IssuingDate`, writes a revision history JSON next to each file, supersedes the prior PDF by renaming-with-suffix not deleting (we keep history).
2. **`nc-export-phase`** — `phase: 2`, `formats: [DSTV, peddimat-asc]`, `group-by: [profile, length]`, `output-dir: ...`, `kerf-mm: 3`. Produces a folder structure the saw line operator expects, plus a `manifest.csv` mapping piece-mark → file → machine.
3. **`bolt-list-reconcile`** — pulls bolt schedule from the model, joins against a procurement CSV (one column per supplier), flags `over-ordered`, `under-ordered`, `wrong-grade`, `wrong-length`, outputs Excel with a coloured delta column.
4. **`weld-list-export`** — every shop-welded connection, weld type / length / throat / AWS code, grouped by assembly mark, PDF for the QC's daily round.
5. **`drawings-by-status`** — `status: in-progress | for-checking | issued | issued-but-modified`, returns the list. This is the foundation for everything else.
6. **`drawings-affected-by-revision`** — given a list of model changes (or an IFC diff against the engineer's prior submission), output the assembly/single-part drawings affected. Revision-cloud propagation.
7. **`apply-connection-batch`** — given an IDEA `.ideacon` file + a list of beam-column joint locations in the model, apply the connection to all matching joints. Right now `insert` does one at a time.
8. **`watch-engineer-ifc`** — TC folder watcher, notifies on every IFC revision from the engineer's `eng-ifc-out` folder. Includes a diff (what beams moved, what was added, what was removed).
9. **`erection-schedule-export`** — read UDA `Lift_Week` per assembly, group by week, export coloured isometric PDFs + a tabular plan.
10. **`mill-order-from-bom`** — input is the tender BoM, output is grouped by section profile + grade, accounting for kerf and standard mill lengths (12 m, 13.5 m, 15 m), output as a procurement-ready Excel.
11. **`fit-up-check`** — input is an as-built survey CSV (e.g. from Trimble RTS), output is per-connection out-of-tolerance flags + proposed mitigations (slot the hole, add a shim, cope deeper).

Half of these are already 80%-implementable from the existing skills — the AI could compose them. But "the AI could compose them" is the wrong layer. **They need to be commands** — versioned, contract-stable, testable, debuggable. The substrate keeps deferring to "AI as the runtime"; production fabrication needs deterministic, replayable, idempotent commands.

## E. Trust and production safety

**Would I let AWARE issue drawings to the saw line without a human-in-the-loop sign-off?** Today: absolutely not. The runtime spec talks about provenance logs at `~/.aware/logs/<app>/<instance>/<run-id>.jsonl` which is fine for forensics, but there's no concept of:

- **A "dry run" mode** that produces a diff: "I would issue these 47 drawings, supersede these 3, send these to TC folder X." Read-only preview before commit.
- **A signoff gate** built into the topology — a node that blocks until a human approves the staged batch.
- **An audit stamp on the model.** When AWARE runs `insert` on a connection, what UDA gets set so I can see, six months later in a fit-up dispute, *which* app run, on *which* date, by *which* user, with *what* inputs, placed that connection? Right now the contract is silent. The `provenance.jsonl` log is in the user's `~/.aware/logs/` — that doesn't help me trace from a part in the field back to its origin.

**Undo / audit trail story.** The CLI spec mentions `aware app logs --replay`. That replays the *log*, not the model state. There's no `aware app rollback`. The Tekla transaction wrapper around `insert` is good (`tx.Rollback()` on failure inside the call), but that only covers within-call atomicity. If an app run inserts 50 connections and you want to reverse them all because the engineer changed the brief, you're hand-deleting. The agent spec should mandate that destructive commands return a `undo-token` that the orchestrator can use to reverse the operation.

**File integrity on a 4 GB / 60,000-part Tekla model.** This is the one I worry about most. Tekla's database is not transactional across processes. Every `CommitChanges()` is a write to the model. The `tekla.watch` agent says it subscribes to `ModelObjectChanged` events — that's fine, read-only. But the moment I have a long-running app with `tekla.insert` firing on every event, I am driving uncontrolled writes into the model while a human detailer is also editing it. There needs to be:

- **A `model-lock` primitive** — the agent reserves the model for a write batch, the UI shows "AWARE editing — N changes pending — Pause / Continue / Cancel."
- **A write-budget per run** — refuse to fire more than N writes per second / per minute.
- **A "this run wrote these GUIDs" provenance record** stored on the model itself (as a UDA on each touched object), not just in `~/.aware/logs/`.

Without all three, I'm not pointing this at production.

## F. The killer feature

**`aware app run issue-drawing-pack --phase 2 --revision E --target tc:folder/issued-for-construction`** — and on Monday morning the saw line has its NCs, the shop floor has its drawings, the engineer has their PDF set in their TC folder, the QC inspector has a fresh weld inspection list, Slack has a single-message summary saying "Phase 2 Rev E issued — 312 drawings, 1,847 parts, 14,200 bolts, 287 welds, deltas vs Rev D listed in this thread."

That is the app where I stop using my macro library and start using AWARE. Everything else flows from it.

It is **not technically hard** with the substrate as designed — the skills already know how to do every piece. The blocker is that the curated agent doesn't expose the verbs, and the substrate doesn't have a revision/audit primitive. Both addressable.

## G. The 5-Mondays adoption test

**Monday 1: `issue-drawing-pack`** — covers ~25% of my Wednesday-afternoon panic.
Blocker: no `drawing-export` / `drawing-issue` / `report-create` commands on the `tekla` agent. Have to climb through skills and inline glue.

**Monday 2: `nc-batch-export`** — saw line for Friday.
Blocker: no `nc-export` command. No Peddinghaus dialect translation. No machine-grouping primitive.

**Monday 3: `bolt-list-reconcile`** — procurement against model.
Blocker: no `bolt-list` command. No standard Excel template. The `excel` agent (mentioned in the manifesto's v0 agent list) does not appear in the actual repo — I see `microsoft-365` and `google-workspace` in the cross-cutting folder; let me trust the README.

**Monday 4: `revision-propagate`** — IFC diff from engineer → revision cloud on affected drawings.
Blocker: no `ifc-diff` primitive. No drawing-affected-by-model-change lookup. Skills don't cover this.

**Monday 5: `erection-schedule-export`** — phase-1 lift weeks coloured + PDF set.
Blocker: no `uda-read` command. No drawing-template-with-colour-map.

By Monday 5, four of five gaps are around verbs that don't exist as commands but *are* documented in skills. The fix is a curation pass on the `tekla` agent — promote the skill paragraphs into first-class commands.

## H. The dealbreaker

**One thing that, if not addressed, will block adoption in any fabrication shop:**

**No agent for Tekla EPM (PowerFab) / FabSuite / Stratus / Hera.** The fabricator's ERP is the source of truth for the production schedule, the material yard, the heat treatment tracker, the cost roll-up, the shipping manifest. If AWARE cannot read or write to it, AWARE is a Tekla-and-TC convenience layer — not a fabrication substrate. The thesis ("AECO is the wedge, not the limit") is dramatic on the engineering side and silent on the *fabrication* side. From where I sit, the fabricator is half the AEC**O** in AECO.

Second on the list — but a dealbreaker too: **no signed Windows binary**. IT departments in fabrication firms have draconian software-allowlists. An unsigned MSI dies at the gate. The README admits this is "queued"; it needs to be on the critical path.

Third: **no revision/audit primitive in the substrate itself**. Every command that writes to the model should stamp `AWARE_RUN_ID`, `AWARE_APP`, `AWARE_REVISION`, `AWARE_OPERATOR` as UDAs on the touched objects. Without this, AWARE writes to the model anonymously, and when a fit-up dispute lands on my desk in six months I cannot trace the part back to its origin. This is the £40k-hole-pattern story — it has happened to me; I will not let software repeat it.

Fix those three and I would pilot AWARE on a real job. Skills are good. Curation needs to catch up to ambition.
