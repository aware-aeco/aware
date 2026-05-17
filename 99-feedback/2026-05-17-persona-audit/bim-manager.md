# Persona Audit — BIM Manager

**Verdict:** **Watch but don't adopt.** AWARE has good bones (text-based composition, decalog discipline, a reasonable CLI surface) but the agent catalogue I'd actually use on Monday morning is either auto-generated firehose (7,647 stub commands per Revit version) or missing entirely (no Navisworks, no Solibri, no BCF, no COBie, no IFC validator, no clash detection of any kind). I would not let it touch a live model.

**Top 3 gaps that block me, ranked:**
1. **No coordination toolchain.** Zero agents for Navisworks Manage 2026, Solibri Office, BIMcollab Zoom, or the BCF 3.0 file format. Clash detection / federation is half my job — and it's invisible to AWARE.
2. **Auto-generated agents are unusable as shipped.** The Revit 2026 manifest is a 1.1 MB file with 7,647 commands like `active-task-dialog-click-button` and `access-denied-exception-get-object-data`. There's no curated "things a BIM manager wants to do" layer on top. The hand-curated Tekla agent (31 skills) proves the team *can* do this — they just haven't, for any of the architecture stack.
3. **No safety story for write-back.** No `--dry-run`, no rollback, no per-command read/write flag, no preview, no approval gate. Decalog #4 ("no vendor in the loop") doesn't help me when AWARE itself can silently corrupt a Central model.

**One app I'd write next week if gap 1 were closed:** *Friday clash digest* — Navisworks Manage 2026 runs the federated NWF, pipes the clash XML into ACC Issues (one issue per matched clash group, ball-in-court inferred from element discipline), drops a BCF 3.0 export into the project Teams channel, posts the rollup to Slack `#coord`. That single app would justify the toolchain.

---

## A. First-impression friction (10 minutes from `git clone` to first usable run)

I read the manifesto first. The 60-second demo on lines 13–34 is misleading. It shows:

```bash
$ npm install -g @aware-aeco/cli
$ claude-code
> Watch this Tekla model. When a welded assembly appears, upload its drawing...
```

But the README confirms (line 22) that the npm package is a 2 KB shim that downloads a ~10 MB binary from a GitHub Release I have to trust. SmartScreen will warn me ("unknown publisher"). My IT team won't sign that off without a code-signed MSI — and the README itself flags the signing as a tracked follow-up. **For an enterprise BIM team, "click *More info → Run anyway*" is a non-starter.** That's the first stop.

Second stop: I run `aware connect trimble-connect` and the README tells me (line 102) to open a *browser tab*, get a token from Trimble's developer console, paste it into a password form. **I don't have a Trimble developer-console account.** Our company's Trimble Connect is provisioned through SSO via the IT department. Nobody in the BIM team has the developer credentials. The "advanced OAuth" path requires `AWARE_OAUTH_TRIMBLE_CLIENT_ID = "your-registered-client-id"` — I'd have to file a ticket with IT, get a SecOps review, register an OAuth app at the tenant level, then come back. **That's a 3-week journey, not 60 seconds.**

Third stop: there are *two* example apps. Both are toy. `qa-drawings-to-tekla.flo` invokes agents called `file`, `excel`, `think-node`, `slack` — none of which exist in `20-agents/aeco/`. I checked. The example apps would fail validation against the installed agent set. Manifesto says "11 first-party agents"; I count something like 30 installed but the two reference apps don't run on any of them as-is.

**What's missing for a non-programmer:**
- A "first hour" guide: install → connect to ACC/Trimble using your normal SSO → run one real app that produces a real PDF/email. The whole onboarding is written for someone who already understands OAuth client IDs, PKCE, and YAML pinning syntax.
- A signed binary. Until that happens, this dies at IT.
- One worked example that touches a real model and one I can actually run end-to-end with the installed agents.

## B. The "no-code" claim — does it hold?

The decalog says (#1) "App = text. Not metaphorically. Literally. The whole app is a file you can read in Notepad." Yes, that part is honest — `.flo` files are clean YAML. I can read them. I can probably edit a `folder-id` value or a Slack channel name. That's genuinely good.

Where it breaks:

- **Inline glue** (`app-spec.md` lines 152–161) is JavaScript-ish code in a `code:` block. `e => e.AssemblyType == AssemblyType.Welded` — that's a lambda. I'd be lost the moment I needed to filter on anything more complex than a single property equality. The spec calls this "no hidden logic" but inline JavaScript inside YAML *is* code; pretending otherwise is gaslighting.
- **Templating** (`{{ tekla-watch.drawing-bytes }}`) is fine for variable substitution but the syntax for fan-in inputs (`{ from: pdf-extract.drawing, keyed-by: mark }`) is dense. I'd copy-paste it; I would never author it from scratch.
- **Agent commands** I haven't built a mental model for. Looking at `revit-2026/commands/document-get-warnings.md` — the file is 3 lines, no inputs, no outputs, no example. If I want to know what params to pass, the docs say "see manifest"; the manifest is 1.1 MB and barely structured. **I cannot author against this without a programmer.**
- **`exposes-as-agent: true`** requires me to author an `exposed-commands` block with input/output schemas. That's API-design work. I do not do API design.

**Operations that require a programmer that shouldn't:**
- Authoring any predicate beyond an exact-match equality
- Adding error handling ("if upload fails, do X")
- Doing anything looped or batched (the spec is event-driven; "loop over 4 Revit files" isn't shown anywhere)
- Reading from one Excel cell vs a range
- Anything involving date math, string formatting, or grouping

I would get stuck on day one composing a new app and would file a ticket.

## C. Coverage gaps in your domain (BIM management)

**Five tools missing that I'd need on day 1:**

1. **Navisworks Manage 2026** — clash detection, model federation, the NWF/NWD format. This is the single biggest gap. There is no AECO BIM workflow without it.
2. **Solibri Office** — rule-based model checking (the only credible IFC validator that actually does code/BEP checks). My BEPs reference Solibri rulesets. Without it, "validate against BEP" is impossible.
3. **BIM 360 / ACC Docs** — the *files* side of ACC. The repo has `acc-issues` and `acc-account-admin` but no Docs (publish, version, attribute, folder permissions, transfer). That's where 90% of the actual files live.
4. **BCF (the file format)** — BCF-XML 2.1 / 3.0 read+write. Not an API — a file format. Every coordination tool emits BCF; without a BCF agent, "BCF round-trip" is not possible.
5. **IfcOpenShell or BIMServer-style IFC inspector** — `web-ifc` exists in `visualization/` but that's a renderer, not an IFC validator/extractor. I need to query IFC files: count IfcSpace, list GUIDs, check IfcSite georeferencing, extract Psets for COBie. None of that is here.

Honorable mentions also missing: Procore, BIMcollab Zoom, Solibri Anywhere, NavisWorks Simulate, Revit-server / Revit-cloud-workshare APIs, ACC Model Coordination (the cloud Navis replacement), Bluebeam Revu (the PDF tool every contractor uses), Naviate/Ideate macros, COBie spreadsheet generation. **These are not edge cases. These are core daily tools.**

**Existing agents that are too thin:**

- **`microsoft-365` — 4 commands.** No Teams post, no Outlook send-mail, no calendar event, no Planner task, no SharePoint list query, no Excel range read (separate from drive-item). I'd hit a wall in 10 minutes — half my reporting goes to Outlook + Teams, neither of which AWARE can hit.
- **`acc-issues` — 14 commands, all stub one-liners.** `create-issue.md` is literally 3 lines: title, lifecycle, "POST /construction/issues/v1/projects/{projectId}/issues —". No input schema. No example. No mention of how to attach a BCF viewpoint (which is the *whole point* of an ACC issue from coordination).
- **`revit-2026` — 7,647 commands.** Inverse problem — it's everything, indistinguishable. There is no curated "BIM-manager surface": list-sheets, list-rooms, get-warnings-count, get-file-size, get-last-saved-by. They're all *somewhere* in there as raw API methods, but I'd need to spend a week building the mental map myself. The Tekla agent (31 hand-curated skills covering `drawing-identity`, `event-threading`, `coordinate-systems`, `events-and-clashes`) shows the team knows how to do this. They just haven't done it for any of the Autodesk stack.
- **Two AutoCAD versions, two Allplan versions, two SketchUp versions, two Rhino versions, two Grasshopper, two Dynamo, three Tekla, two TSD, two iTwin, two IDEA…** — version-sprawl with no curation. Why install `autocad-2025` AND `autocad-2026` instead of one `autocad` agent with version constraints?

**Operations that should be cross-tool but aren't:**

- "Validate this IFC against our BEP" — touches IFC reader + BEP doc + reporting; no single command.
- "Check level naming consistency between Revit, Tekla, and Allplan models" — touches three CAD agents + a diff/comparison primitive that doesn't exist.
- "Audit family library across all projects" — needs Revit reader + cross-project iteration + tagging/grouping primitive.

These are workflows. AWARE prides itself on composition — but I haven't seen any *workflow* primitives, only tool wrappers.

## D. Workflow primitives you wish existed

Atomic operations I reach for daily, missing from AWARE:

- **`screenshot`** — capture a Revit/Tekla viewport, attach to a Teams/Slack/ACC message. ACC Issues without a screenshot is a half-story.
- **`approve`** — pause execution, post a "Approve / Reject" button in Slack or Teams, wait for human response, branch on outcome. Half my workflows have human-in-the-loop steps.
- **`schedule`** — "run this every Monday at 07:30 before standup." There is no cron in the app spec. `tekla.watch` is the only trigger pattern, which is event-driven. I want time-driven too.
- **`for-each`** — "for every Revit file in this folder, run X." The app spec is event/stream-based; iterating over a static collection is unidiomatic.
- **`abort-if`** — "if file size > 500 MB, abort and notify." No guard/assertion primitive.
- **`compare`** — diff two models (or two snapshots of the same model). Critical for BEP compliance audits.
- **`bcf-round-trip`** — read BCF, transform, write BCF. The format is industry standard; should be a first-class primitive.
- **`html-report`** — the `_core/html-report` agent exists but I can't tell from a single skill file whether it's a real templated report builder or a stub.

**Primitives in the app-spec I don't trust:**

- **`inline.kind: predicate`** — the app file embeds a JavaScript-ish lambda. Who interprets that? The CLI is Rust. There's no JS runtime mentioned anywhere. I would not bet my Monday pipeline on it.
- **`think-node` and `smart-node`** — both referenced in the example DAG but neither is in the agent catalog. `think-node command: extract` with a prompt is presumably an LLM call. So the app *contains a literal LLM prompt* as the validator. I don't trust an LLM to do unit conversion or tolerance math on a connection callout. That's a hallucination waiting to corrupt a fab drawing.
- **Templating order of operations** — when does `{{ secrets.X }}` resolve? Before the agent gets it, or inside the agent? If the latter, every agent re-implements the templating. If the former, secrets touch the orchestrator's address space.
- **Idempotency** — only the Trimble Connect agent documents idempotency-by-mark. Every other write-capable agent (acc-issues, m365, slack) does not. I'd uploads duplicates for weeks before noticing.

## E. Trust + safety

**Would I let AWARE write into our live production models?** Not a chance.

Reasons:

- **No `--dry-run` anywhere.** I searched. The CLI spec doesn't mention it; the app spec doesn't have a `mode: simulate` field; the README has no example. Every Revit/Tekla automation I've ever bought ships with a dry-run mode because the first run of any automation against a live Central model is terrifying. Without it I would not even unit-test against a real file.
- **No rollback / undo / checkpoint.** Once `tekla.insert` creates a part, there is no "step back." The provenance trail (`logs/<app>/<run>.jsonl`) records *what* happened, but it's not actionable. I want "replay this run in reverse" or "create a savepoint before this node."
- **Permission model is too coarse.** The agent manifest declares `requires.filesystem.read` / `network: <hosts>` / `software`. That's a *capability* declaration, not a *scope* one. Nothing says "this app may upload but not delete." Nothing says "this app may write to folder `/fab-drawings/` only, not `/contract-deliverables/`." When I approve once at install time, I'm approving forever. That's not granular enough for a CDE I'm legally responsible for.
- **No mandatory peer review for shared apps.** Decalog #6 ("ship agents, not apps") means apps are shareable text. The instant someone in my firm shares a `.flo` over Slack and a junior runs it against our ACC project — there's no signing, no audit, no provenance check beyond what they choose to read.
- **`exposes-as-agent: true` blurs trust.** I install `@detailer/connection-validator`. Internally it composes 7 agents I never approved. The spec says "an exposed app **does** inherit the union of its internal agents' `requires-permissions`. The caller approves the full union" — okay, but in practice I'd see a prompt with 30 lines and click "Allow" because the prompt is too dense.
- **Logging is JSONL only.** Fine for replay, terrible for human audit. I want a CSV export and a "who changed what in our Revit Central this week" view.

**Permission visibility:** the `aware app validate` checks for missing fields but the docs don't say it surfaces a read/write summary. I want a single command — `aware app explain <name>` — that prints: *"This app reads X, writes Y, sends to Z. Cost estimate: $0.04 in LLM calls per run."* That doesn't exist.

## F. The killer feature you want

**"Monday morning model audit" — one command, four Revit Central models, full report in Teams.**

The behavior I want:

```
aware app run monday-audit --project "Riverside Phase 2"
```

And out comes:

- A table per model: file size, last-saved-by, last-saved-when, warnings count, unplaced rooms count, sheets count, sheets-changed-this-week count.
- Cross-model checks: shared coordinates baseline match, level naming match, grid naming match, federation alignment delta.
- A "BEP compliance" pass: parameter standards, naming standards, view template standards.
- Sent as a Teams post + Excel attachment + PDF to design-lead's Outlook.

If AWARE could do that *out of the box* on day 1, I'd run it before Monday standup every week. It would replace the half-dozen Dynamo scripts and the Naviate audit panel I currently use, and I'd tell my team "we're using AWARE now."

It can't do it today because: (a) no Revit `list-rooms` / `list-sheets` / `get-warnings-count` high-level commands curated in the 7,647 raw list, (b) no IFC alternative path if Revit-API is unavailable, (c) no Teams-post in `microsoft-365`, (d) no PDF generation primitive, (e) no scheduling primitive.

## G. The 5-Mondays adoption test

**Week 1 — `welded-to-tc` running for real**, our actual fab pipeline. This is the manifesto demo. Can AWARE deliver it? **In theory yes**, but Tekla agent is one version (`tekla` hand-curated, then split into 2025/2026 stubs), Trimble Connect needs our IT to register an OAuth app or somebody to manually grab a session token from the dev console — neither happens in week one. **Realistic chance: 20%.** And even if it ships, the watch-loop runs on one workstation; nobody can monitor it remotely.

**Week 2 — IFC reference-model audit on receipt from MEP consultant.** Validate the IFC has valid IFC4 schema, expected entity counts, georeferencing, no orphan GUIDs. **Cannot deliver** — no IFC validator agent. `web-ifc` is a viewer not a validator. Blocked.

**Week 3 — Daily ACC Issues digest into Teams.** Pull yesterday's new issues, group by ball-in-court, format as a digest, post in Teams `#coordination` channel. **Cannot deliver** — `acc-issues` is 14 stub commands, `microsoft-365` doesn't have a Teams post command, and there's no scheduling primitive. Two missing pieces, both critical.

**Week 4 — Sheet status report across one Revit project.** List every sheet, owner, last-saved-by, sign-off status, dump to Excel. **Theoretically deliverable** — Revit-2026 has the API calls in the raw firehose, M365 can upload to OneDrive — but nobody on my team can navigate 7,647 commands to find the right four. Without a "BIM-friendly" curated layer this fails the no-code test.

**Week 5 — Family library cross-project audit.** Walk N project files, extract family list, group by category and age. **Cannot deliver** — no for-each primitive, no aggregation primitive, no diff primitive.

**After 5 weeks: AWARE saves ~10% of my toil.** Generously. And only if weeks 1 and 4 land. The other three weeks are flat-out blocked by missing agents and primitives. I'd still be running my Dynamo scripts, Naviate plugins, and an Outlook rule for the rest.

## H. The single thing that would make you walk away

**`think-node` and `smart-node` doing AI-as-runtime for safety-critical model operations.**

If AWARE's pitch is "AI is the runtime" (decalog #2), and that means an LLM is in the validator path for *"is this drawing's tolerance within 5%"*, then I will not adopt it under any circumstance. I cannot ship a fab drawing whose validation step is a non-deterministic LLM. PE seals and steel deliveries do not survive hallucinations. **For any AECO write-back, the runtime must be deterministic, traceable, and testable.** The LLM can compose the *plan*. It cannot *be* the plan.

Adjacent dealbreaker: **no path to enterprise distribution.** I can't roll out AWARE to 150 people via curl-pipe and SmartScreen warnings. Until there's a code-signed MSI, an MDM-deployable package, and a centrally-managed credential store, this is a hobbyist tool. Decalog #4 ("no vendor in the loop") is admirable but practically — without a signed binary and an SSO-friendly auth path — I can't deploy it past my own laptop.

---

**Closing note for the founder:** the substrate ideas are good. The decalog discipline is rare. The text-based composition is genuinely the right architecture. But you have ~95% of the documentation work done and ~25% of the agent catalogue done. The reflexive solution — "let the agent builder generate agents from NuGet packages" — produces Revit-2026 with 7,647 commands, which is *worse* than nothing because it dilutes search. The hand-curated Tekla agent is the existence proof that curation works; you need someone (a BIM manager, not a Rust dev) to do that curation for Revit, Navisworks, ACC Docs, Solibri, and IFC. **Until the catalogue catches up with the spec, no BIM team adopts this.**
