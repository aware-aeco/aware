# Persona Audit — Structural Engineer (TSD / IDEA / CSi / Tekla)

**Verdict (3 sentences).** AWARE is genuinely interesting plumbing — the substrate is honest about being substrate, the agents reflect real APIs (TSD, IDEA, CSi, Tekla, Revit at version-pinned granularity), and the manifesto thesis ("apps are text, AI is the runtime") is the first AECO automation pitch in 15 years that I can repeat at a CPD without wincing. **But it's nowhere near ready to underwrite a calculation I sign.** It is, today, an excellent way to *automate everything around* my calcs (file routing, schedule extraction, report assembly, RFI mechanics) and a *dangerous* way to do the calcs themselves, because nothing in the substrate forces — or even encourages — engineering-grade reproducibility, code-revision pinning, or stampable audit trails.

**Top 3 gaps (in priority order):**
1. **No engineering-grade audit envelope.** There is a provenance JSONL, but it captures the call graph, not the *engineering inputs* (NA, code revision, partial factors, ψ values, gauge of the section catalogue, build of the analysis kernel). I cannot hand that JSONL to a checker and have them recompute. EN 1990 §2.5 reproducibility is non-negotiable. AWARE does not enable it.
2. **The engineering agents are reflection-grade, not engineering-grade.** TSD-26 has 440 commands — and the *Design* skill is **9 lines long** (literally enumerates `DesignStatus`, `DesignSubject`, `DesignType`, `IDesignCondition`). There is no skill capturing "how do I run an EC3 check on a beam and read its utilisation"; no skill mapping "Australian Loading Category" to "what an Aussie SE wants". An AI composing against this manifest will guess.
3. **No "engineering primitives" in the app spec.** The DAG supports `predicate`, `map`, `shape`. It does not support "for-each-load-combination", "delta-against-previous-run", "fail-on-utilisation > 1.0", "iterate until convergence", or "hold a reference snapshot for peer review". Every parametric study I do needs these. Without them, every app re-implements them inline.

**One concrete next-week app I'd actually deploy** (if the gaps below were closed): **`steel-tonnage-watch`** — watch `Tekla Structures` for model save events, extract the steel BoM (`structures-model-snippets.md` already covers this), diff against the previous saved BoM, post `+0.4 t added between rev-C and rev-D — UB 305×165×40 increased on grid B/2-3` to a Teams channel. That's a 5-node `.flo` against existing agents and it would replace a daily ritual I do by hand. I'd ship it Monday morning if the runtime existed against a real Tekla.

---

## A. First-impression friction

The manifesto reads honest. *"Software has always been a thing you buy from someone else"* — yes, and we paid £50k/seat last renewal. The decalog is the right shape: short, contestable, anchored. Decalog #1 ("app = text") is the part I'd quote at any AEC tech meet — it's the first time the open-source argument has been made in a way that's *structural* to the format, not licensed-on-top. I don't have to trust someone's GPL pledge; I have to trust that `cat welded-to-tc.flo` returns text. It does.

That said: the manifesto talks like a software vendor, even while denying it is one. *"For decades that integration was either buy a $40k plugin or hire two devs for six months"* — yes, but you're skipping a third path: "or it didn't exist because the engineer who needed it spent that time doing engineering instead of writing integration code." The reason the integration didn't exist isn't *just* economic friction; it's that the engineer making the buy decision (me) does not write integration code, has no time to learn, and is professionally liable when an integration silently miscomputes a load case. The manifesto's claim that AI removes this is hand-waved — `30-apps/_examples/qa-drawings-to-tekla.flo` has an inline glue node `match-build` whose `tolerance-percent: 5` is fine for a sketch but **professionally negligent** for a real connection insertion. A 5% tolerance on what — connection eccentricity? Beam end position? Load? Whose engineer accepted that 5%? Where is it captured? It's in a `.flo` file in someone's git, and that's exactly the failure mode EN 1990 §1.3 ("Assumptions") was written to prevent.

**Could I reproduce the 60-second demo in 30 minutes?** Setting aside that the runtime against real Tekla is still being shipped (the agent transports invoke a CLI binary per-agent — `aware-tekla-2026.exe` etc. — which doesn't exist for the desktop products yet): the *manifesto* reads as if it does. That's a credibility problem. A structural engineer reading "✓ aware CLI installed / ✓ 11 AECO agents available" expects that when they paste it, it works. The `cli/README.md` is honest about shipping status (v0.6.0, all phases shipped, agent binaries on PATH still expected for hand-authored ones). The manifesto should be too — *"60 seconds once you've installed Tekla + connected Trimble Connect + the Tekla agent binary is on PATH"*. Soft sell on the 60 seconds when half my colleagues will give up at the OAuth dialog.

## B. The "no-code" claim

The `.flo` files are *readable* to me. I write MATLAB, I write VBA, I read SAP2000's `$2k` text file, I edit IDEA StatiCa `.ideacon` XML when it crashes. `welded-to-tc.flo` is YAML; I get it in 30 seconds. `qa-drawings-to-tekla.flo` is harder — the `row:` / `col:` grid coordinates are noise; the `inputs:` block under `match-build` with `keyed-by: mark` is opaque without reading the agent skills. I would read it but I wouldn't author it.

Where it breaks down for me:

- **Inline glue is JavaScript-ish.** `e => e.AssemblyType == AssemblyType.Welded` — that's a lambda. Not Python, not VBA. I don't write that idiom. I copy-paste it from somewhere, change `Welded` to `Bolted`, and pray nothing else broke. The app-spec calls it "inline glue, kind: predicate" — but `code:` is a free-text block of arrow-function syntax. There is no validation that `AssemblyType.Welded` is a real enum value at compose time. If I typo `Wellded`, the app runs forever and never matches anything, silently.
- **YAML templating is genuinely awful for engineers.** `{{ tekla-watch.mark }}.pdf` for a filename is fine. `{{ match-build.connection-type }}` for an enum value going into Tekla is *not* — I have no idea what shape `connection-type` is. Is it a string `"baseplate"`? An integer code `1041`? A Tekla GUID? `app-spec.md` says templating is resolved at composition time. So when does it tell me the type doesn't match what `tekla-insert` expects? Probably at runtime, in the JSONL log, after the model has been corrupted.
- **Parametric studies need iteration.** AWARE has no `for ... in` loop primitive. Watch nodes emit a stream; one-shot nodes run once. To re-run an analysis under 12 wind directions, I need either 12 nodes (ugly), or a "smart node" doing it internally (hides the parameter), or the `think-node` agent — which is just an LLM call. **That's not engineering iteration; that's storytelling.** I want to declare `for direction in 0..360 step 15: run-analysis; record utilisations` and get back a 24-row table.

## C. Coverage gaps in your domain

The engineering agents are *enumeratively* comprehensive and *engineering-judgment-wise* hollow. TSD-26 manifest has 440 commands; the design-related skill is the 9-liner above. Tekla-2026 has ~3,179 commands across 22 namespace skills, but the curated `tekla` agent (the hand-refined sibling) has the actual engineering content (`connections-bolts-welds.md`, `reinforcement-patterns.md`, `events-and-clashes.md`). **The auto-generated agents are reference, not knowledge.** That's fine, but the manifesto sells them as both.

**What's missing entirely:**

| Tool | Why I need it | Where AWARE stands |
|---|---|---|
| **RAM Structural System** (Bentley) | Half the US market for steel buildings | absent |
| **SCIA Engineer** (Allplan/Nemetschek) | Big in EU; the actual engineering arm of Nemetschek; you have `allplan-2025` (the CAD) but not SCIA | absent |
| **Autodesk Robot Structural Analysis** (RSA / ARSAP) | Bundled with Revit license, lots of EU offices use it for compliance | absent |
| **RFEM / RSTAB** (Dlubal) | Best EC8 seismic; best timber design; standard for German speakers | absent |
| **RISA-3D / RISAFloor** | US mid-rise standard | absent |
| **midas Civil / Gen** (MIDAS) | Bridge engineering; the Korean/Asia market | absent |
| **Strand7 / Straus7** | UK + AU bridge + nuclear | absent |
| **ADAPT-Builder** (RISA) | Post-tensioned slabs — the only credible PT slab tool | absent |
| **FEM-Design** (StruSoft) | Scandinavia; PT slabs; ETABS competitor | absent |
| **PROKON** | Southern Africa, India; cheap-and-cheerful design tool | absent |
| **Limcon, RAM Connection** | Connection design — IDEA's competitors | absent |
| **Hilti PROFIS Engineering** | Anchor design — every connection that hits concrete | absent |

The **architecture** side covers Revit, AutoCAD, Rhino, Grasshopper, Dynamo, Allplan, SketchUp — that's competent. The **engineering** side covers Tekla (detailing, not analysis), TSD (analysis + EC design — UK-centric), IDEA (connections), CSi (analysis). **Missing FEA for civil/bridge/PT entirely.** Missing geotechnical entirely (`PLAXIS`, `WALLAP`, `gINT`, `Slide2`, `Settle3`). Missing wind tunnel software entirely. Missing seismic-specific (`Perform-3D`, `SeismoStruct`).

**Per-agent thinness for TSD-26 specifically.** Look at the manifest:
- `imodel-run-analysis-async`: description = *"Performs an analysis of requested type over a requested selection of combinations and loadcases"*. That's the **interface docstring**. There is no skill telling the AI: *"Before calling `RunAnalysisAsync`, you must (1) confirm the model passes validation (`ValidateAsync`), (2) ensure the analysis type matches the code (`EuroCode` vs `Australian`), (3) check loadcases are mapped to combinations, (4) know that 3D analysis on >50k elements times out the default Remoting channel."* That knowledge is what stops a junior engineer destroying a model. AWARE doesn't have it.
- The auto-gen TSD-26 exposes load combinations and design checks **as method calls**. It does **not** expose Eurocode parameters (γ_M0, γ_M1, ψ_0, ψ_1, ψ_2), code revision (EC3:2005 vs EC3:2022), or National Annex selection. Those live inside TSD's model file. The agent treats them as opaque. **They are the single most important inputs to any structural calculation.** A `cli-spec` for an engineering agent should *force* declaration of these in the manifest's `requires:` block, not hide them.

**No code-revision agent.** Where is `eurocode@2022.uk.NA`? Where is `ASCE@7-22`? Where is `IS 800:2007`? These are versionable artifacts. AWARE versions DLLs but not codes of practice. A storey-add what-if app needs to know: *"the previous calc was done under EC3:2005+UK NA 2008 with ψ_0,snow = 0.5; the re-run is under EC3:2022+UK NA 2022 with ψ_0,snow = 0.5; deltas in member utilisation are attributable to code change, not loading change."* This is the *exact* tracing AWARE's `provenance` field should produce — but the spec doesn't surface it.

## D. Workflow primitives you wish existed

In priority of how much they'd unblock real work:

1. **`for-each` over a list.** Run a design check for every member in a filter. Run a connection check for every node where a beam meets a column. Run wind analysis for 8 directions. AWARE has fan-out for static graphs, no iteration primitive.
2. **`assert` / `fail-fast`.** Inline check that *every* member's utilisation ≤ 1.00 before proceeding. If not, halt the app, dump the failures, and refuse to write the report. EN 1990 §2.1: limit states are pass/fail.
3. **`snapshot` / `freeze`.** At end of design stage, freeze the model state into an immutable artifact. The peer reviewer downloads the snapshot, the runtime, the agent versions, and recomputes. Match → approved. Mismatch → fail. Without this AWARE is not auditable; it's just convenient.
4. **`sweep`.** Storey count from 4 to 10, beam grade S275 vs S355 vs S460, bay 6m vs 7.5m vs 9m — tabulate utilisation. That's a 2-3 line declarative block in any decent parametric framework (Grasshopper does it, Tekla Live does it). AWARE has no shape for it.
5. **`unit-discipline`.** Every numeric input is dimensioned. kN, kN/m, kN/m², MPa, mm, m. The TSD agent already speaks `IUnitConverter`; the substrate doesn't surface units at the templating layer. `{{ match-build.deviation-percent }}` is fine; `{{ floor-load }}` had better be `4.0 kN/m²` not `4.0`.
6. **`signed-output`.** End of pipeline produces a PDF; the PDF is hashed; the hash is signed with my CEng credential; the signed hash is appended to the JSONL log. *That* is the future. Today an engineer's PE/CEng stamp is a wet-ink signature on a printed PDF. The Royal Academy is already discussing digital seals. AWARE is exactly the kind of substrate that should be first in.
7. **`lock`.** Single-writer. Two engineers running this app against the same TSD model at the same time = corruption. Cap-lock the model file. (TSD already has session locks; AWARE should respect them.)

## E. Trust and reproducibility

This is where AWARE breaks for me.

**Reproducibility in 5 years.** The setup: I sign a calc package today. In 2031 a defects notification arrives. Insurer asks the checker to recompute. The checker is given:
- The `.flo` file → text, fine.
- The lockfile → version-pinned agents, fine.
- The `~/.aware/credentials/` → not transferable; checker has their own creds.
- **The agent binaries** → `aware-tsd-26.exe` v0.x.x with what underlying TSD? Was the TSD install 26.0.3 or 26.0.7?
- **The TSD installer** → Trimble has a 5-year support window. By 2031, TSD 26 is unavailable. Trimble offers TSD 32. Different solver, different EC2 revision, different NA defaults.
- **Eurocode revisions** → EC3 had four minor amendments between 2024 and 2031, two affecting LTB.

The `.flo` text reproduces. The *engineering result* does not. `app-spec.md` says exact pinning is "used for reproducibility." That pin is a string match against the agent's manifest version — `tekla@2025.0.1`. It does **not** pin the underlying TSD .exe build, the model schema version, the design-code revision, or the NA selection. **A 5-year reproducibility claim requires pinning all of these.** This is exactly what AWARE could uniquely do, because it sits between the engineer and every tool. Today it doesn't.

**Audit trail.** The `~/.aware/logs/<app>/<instance>/<run-id>.jsonl` is fine for execution provenance — what called what, what came back. It is *not* an engineering audit trail. An engineering audit trail records:
- Code of practice + revision + NA
- Materials catalogue + revision
- Section catalogue + revision (which "UB 305×165×40" — BS 4-1:2005 or the EN 10365:2017 version?)
- Solver build hash
- Inputs (loadcases, combinations, factors, ψ values)
- Outputs (utilisations, deflections, accelerations) with their tolerance/precision

None of that is in the JSONL spec. It could be — that's a `provenance: ...` block extension at the agent level — but no agent does it today and no app demands it.

**Engineer's stamp.** Where does my CEng/PE stamp go? Nowhere in the substrate. I sign printed PDFs with wet ink. AWARE has no concept of a chain-of-custody between "this app produced this output" and "this engineer is professionally liable for this output". Until that's there, my insurer (Zurich, in my case) won't underwrite a calc whose decisive step ran in `aware app run`. That is the literal sentence I will hear when I propose adopting this on a real project.

## F. The killer feature

For me — at 14 years, leading delivery on 3 live projects this month — one app converts me:

**`peer-review-delta`** — given the current TSD model and a snapshot from the last peer review checkpoint, produce a one-page report listing every member where: section changed, grade changed, length changed by > 5%, end fixity changed, applied load changed by > 10%, or utilisation crossed the 0.85 threshold (in either direction). Plus an appendix: every load combination that gained or lost a loadcase factor. Plus a heatmap of which gridlines saw the most churn.

This is what I do by hand for 4 hours every fortnight on every project. **Four hours, fortnightly, every project, every engineer.** That's ~100 hours/year per engineer. Across a 15-person team, that's an engineer-year of work. An AWARE app that does this reliably converts our whole team in one demo.

Crucially: this app does not require any of the gaps in section E to be closed. It is a *reading* app, not a *deciding* app. It reads the model, reads the snapshot, computes deltas, writes a PDF. The engineer still makes the call. AWARE handles the donkey work; I handle the liability.

That's the wedge.

## G. The 5-Mondays adoption test

**Monday 1 — `peer-review-delta`** (above). Probability I get value: high. Gaps to close: snapshot primitive, table renderer (the `html-report` agent exists; PDF output via existing tooling is fine).

**Monday 2 — `steel-tonnage-watch`** (mentioned above). Watch Tekla saves; diff steel BoM; post to Teams. Probability: high. Gaps: Tekla 2026 agent transport binary against real Tekla; the curated `tekla` agent's `model-operations.md` skill already covers what's needed.

**Monday 3 — `calc-pack-assemble`**. End-of-stage: collect TSD output, IDEA outputs for the 8 worst connections, hand-calcs from a folder, cover sheet with project metadata, ToC, signed engineer's certificate page. Probability *if E. closes*: high. Today: medium — I can stitch PDFs but the signed-certificate page is the blocker, no AWARE primitive for it.

**Monday 4 — `connection-batch`**. Top 200 worst-stressed beam-column nodes from TSD → IDEA StatiCa → utilisation table. Probability: medium. The idea-statica-26 manifest has 116 commands and the connection-results skill (`idea-rs-open-model-connection.md`) actually mentions a real engineering gotcha (`CheckResBolt`: *"Bolt identifiers are opaque internal solver identifiers... do not perform arithmetic on bolt identifiers"*). That's the right shape of skill. **Replicate that across the rest of the agents and you have something.** Gap: the TSD→IDEA SAF/CXL exchange path is not skilled anywhere; today an engineer does that by clicking export-import dialogs.

**Monday 5 — `coordination-check`**. Architect's Revit grid + level + LOD against my Tekla model; produce delta list, post to ACC as issues. Probability: low without `acc-issues` having the ability to *create* issues with attachments + the cross-grid coordinate-system reconciliation skill that does not exist. Today this is 2 days of manual work.

**Score:** 2 hard wins, 1 win with substrate gaps closed, 2 wins with significant agent build-out needed. **Worth my time at 2/5 today, 4/5 with 6 months of curation.**

## H. The dealbreaker

**There is no answer to "who is liable when your app gets this wrong."**

The decalog says (#4) "No vendor in the loop." I respect the engineering thesis. But it also means: when `aware app run storey-add` returns "all 432 members pass at the proposed 8 storeys" and three years later the building's roof deflects 90mm at 32-yr return wind because the wind-direction sweep had an off-by-one in the inline glue node, **no one is on the line.** Anthropic isn't, by design. Trimble isn't, because the TSD calc was correct given the loads passed in. The .flo author isn't, because they didn't write the code that interpreted it. The engineer who ran it? *Maybe* — but only if they understood every line of the .flo, every skill of every agent, every templating substitution, every inline-glue lambda. **No engineer will.** That's why we have professional indemnity insurance, that's why we have CEng/PE chartership, that's why we sign drawings with wet ink — because we accept *legal* responsibility for the calculation. And accept it knowing what was in the calculation.

The fix is not "don't use AWARE." The fix is that **AWARE must produce a calculation package whose every input is auditable, every code-revision pinned, every solver build hashed, every assumption flagged, and every step replayable by a third party with no access to the running engineer's machine.** Until then, an AWARE-generated calc package is a *productivity tool* that ends at the engineer's stamp, not a deliverable that *replaces* engineer-authored calcs. That distinction matters for everything from project insurance to RIBA stage gates to court-admissible evidence.

If I had to walk away today, this is the reason. If you close section E (audit envelope, code-revision pinning, signed outputs), I stay. If you don't, AWARE remains a clever toy for the parts of my job that aren't structurally load-bearing — which is genuinely a lot of value — but it doesn't earn the central role the manifesto wants.

**Verdict in one line, for the record:** ship the substrate, close the audit story, add 4 more analysis agents (RFEM, SCIA, RSA, RAM), and curate the existing ones from 9-line stubs into actual engineering skills — and a 15-person UK consultancy converts.
