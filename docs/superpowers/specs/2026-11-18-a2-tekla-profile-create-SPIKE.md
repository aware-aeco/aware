# A2 spike plan: creating a parametric profile in the user's Tekla catalog

Issue: aware#858 (floless.app epic #3382, slice A2; spec `docs/superpowers/specs/2026-11-10-parametric-profile-catalog-EPIC.md` §5.4, §6, open decision §11.4).
Status: **plan and spike code only. Nothing in this document has been run against a live Tekla.** Everything marked "found" was read from files on disk (the Tekla 2026.0 Open API XML docs, the environment folders, the existing bridge code). Everything marked "to prove" waits for Tekla time.

## 1. What the existing commands teach (patterns A2 must follow)

Read from `cli-tekla/` and `20-agents/aeco/engineering/tekla/commands/` at `main` (db0072a63).

| Pattern | Where | What A2 reuses |
|---|---|---|
| A verb is a Roslyn script embedded as a `const string` (`Usings` + helpers + `Body`), run by `ExecuteResolvedScript(verb, code, version, pid, argsNode, ScriptCommitPolicy.ScriptOwned)`. Pure helpers are separate constants so unit tests execute them; the Tekla-facing half is pinned on the source and proved live. | `ProfileCatalogScript.cs`, `SectionMeasureScript.cs`, `ProfileProjectScript.cs`, tests `SectionMeasureTests.cs`, `ProfileProjectTests.cs` | Same shape for `ProfileCreateScript`. A script that writes uses `ScriptOwned`, never the automatic `CommitChanges`. |
| Destination identity = `model.GetInfo().ModelPath` plus the environment name taken from the `\environments\<name>\` segment of `TeklaStructuresSettings.GetAdvancedOption("XS_PROFDB")`, returned as `model_path` and `environment {name, profile_database}`. | `section-measure.md` output, `SectionMeasureScript.cs:427-440` | A2's target identity is exactly these three values plus the Tekla version. It re-reads them at write time and compares. |
| Refusals before Tekla is touched are exit 2 with the field named; a Tekla-side refusal is a structured receipt; exactly one reachable Tekla and an open model are required. | `Program.cs` `SectionMeasure`, `ExecuteResolvedScript`, `TryReadPid` | Same envelope and exit codes. |
| Digests: `bake-scene` stamps `AWARE_SRC_V1:`+sha256 into UDAs and keys receipts on them. A0p composes names itself and **proves them by read-back** (`LibraryProfileItem.Select` must be false, `ParametricProfileItem.Select` must report the intended type, order and every size bit for bit). | `BakeSceneScript.cs:89-95`, `profile-project.md` | A2's digest is the caller's (the FloLess definition digest); AWARE recomputes it from the definition it is given and refuses a mismatch. Reuse-identical is decided by digest plus read-back, never by name alone. |
| Receipts keep what is outstanding: `section-measure` writes a per-run ledger of temporary members and reports leftovers of earlier runs, never deleting them. | `section-measure.md` | A2 keeps a per-run ledger of every catalog file it changed (path, SHA-256 before and after, backup path) and reports it, including on partial failure. |
| The verb surface is guarded: `cli/tests/tekla_verb_parity.rs` requires `dispatched == declared − planned`, `manifest.yaml` declares each verb with `mode`, and a `mode: write` verb needs a `safety:` block. | `tekla_verb_parity.rs`, `manifest.yaml` | `profile-create` is added in step 2 (not in this spike): dispatch case, manifest entry (`mode: write`), `commands/profile-create.md`, CHANGELOG. The spike adds **no** verb. |
| Geometry oracle: `section-measure` returns Tekla's evaluated outer and hole loops, area and bounds for any profile name, and the reference point. | `section-measure.md` | The verified receipt (E6) calls it for sample sizes. |

## 2. Candidate creation mechanisms (Tekla Open API 2026.0, `Tekla.Structures.Catalogs.xml`)

Facts below were read from the XML docs shipped in `C:\Program Files\Tekla Structures\2026.0\bin\` and the reflected catalog JSON under `~/.aware/agents/tekla-2025/catalog/`.

### 2.1 What the API does and does not have

- **No `Insert` exists** on `LibraryProfileItem`, `ParametricProfileItem` or the base `ProfileItem`. The profile classes offer `Select*`, `Modify`, `Copy(newName)` and `Delete` (library only), `ModifyProfileItemParameter`, `AddCrossSection`/`ModifyCrossSection`/`DeleteCrossSection`, `Export`, `GetCrossSection`/`GetHighAccuracyCrossSection`.
- **`ParametricProfileItem` has no `Delete`.** The API cannot remove a parametric shape that was added. Rollback of a parametric write is therefore a file restore, never an API call.
- `CatalogHandler.ImportParametricProfileItems(folder)` imports `*.uel` (sketched profiles); `ImportLibraryProfileItems(folder)` imports `*.lis`. Both say: *"Import overrides the existing profiles in the catalog without any warnings."* A silent-overwrite check must therefore run **before** the call (decision 4 of the contract).
- `CatalogHandler.SaveProfileDatabase()` exists, so whether an import is persisted on its own is itself a spike question.
- `ProfileItem.Export(ref filename)` / `CatalogHandler.ExportProfileItems(names, path, fileName)` write `.lis` (library), `.uel` (sketch) and `.clb` (clb profiles). *"If path is not given profile is exported to model folder"*: the spike always passes an explicit path.
- `ProfileItem` flags: `IsSketchedUserParametric`, `IsCLBProfile`, `IsHardcodedProfile`, `IsMultiCrossSectionUserParametric`. `ProfileItemParameter` exposes symbol, property, unit type, value: **not the formulas.** The API cannot read a sketched shape's expressions back; only the exported `.uel`/`.clb` text (or A0m's evaluated section) can.
- `ShapeItem`, `ImportShapeItems`, `ImportShapeItemsFromGeometryFiles` are the shape (Item) catalog. That is slice **A5**, not A2.

### 2.2 File formats seen on this machine (read-only inspection)

- **`.clb`** (text, in `…\Environments\common\profil\*.clb`): `library_id "X"` then `section_type { name; profitab "PREFIX ! USER ! 0 ! ! 2 ! 2 !LIB.SECTION !h*b"; base_attribute { name, description "albl_…", type dimension, default }; expression { name, type y|z|dimension, formula … }; geometry { name "default"; face { index 0; point 0 y1 z2 … } } }`. Corner points name expressions; formulas use `+ - * /`, `sin cos`, `PI`. This is the shape FloLess kind-2 maps onto most directly: one `base_attribute` per named size, one `expression` per corner coordinate, one `face` per loop. A hollow is **one face that runs out to the hole and back** (see the epic §3), so FloLess hole loops must be bridged on the way out.
- **`profitab.inp`** (text, `…\Profil\profitab.inp`): one row per prefix, e.g. `IRR_F ! USER ! 0 !   !  2 !  2 !TEKLAINC.XCHAMFER !a*b`. The `Lib.Section` column names the user shape; the last column is the name grammar. The `.clb`'s own `profitab` line is this row.
- **`profdb.bin`** (gzip of a binary record database; 540 KB in the default environment, 1.7 KB in `blank_project`, 790 KB in an old model folder). Not text, not documented. **Writing it directly is rejected** as a mechanism: no API, no schema, and a wrong byte corrupts a person's whole catalog.
- **`.uel`** (gzip of a text container: `<FileInfo>` headers followed by the files; `NOTCH.uel` in an old model folder holds a custom-component dialog). Whether a *sketched profile* `.uel` is a text container we can template is **unknown** until E1 exports one.
- Model folders contain `ParametricProfiles\` and `ExportedProfiles\` sub-folders (empty on every current model here), and an old model (`Take_5_2014-11-27`) carries its own `profdb.bin`, so a model-local catalogue exists in some form. Whether Tekla 2026 still reads or writes one is **unknown** (E0, E2).

### 2.3 The mechanisms

| # | Mechanism | Can express a kind-2 definition (formula loops over named sizes, holes)? | Where the write lands (file) | Reversible by API? | Read-back |
|---|---|---|---|---|---|
| **M1** | `LibraryProfileItem.Copy(newName)` + `ModifyProfileItemParameter` + `Modify()` (fixed-dimension library profile). No Insert, so it needs an existing library profile to copy. | **No.** It makes another fixed-size entry of an *existing* shape; the outline comes from the shape it was copied from. Useful only to create a fixed instance of a shape that already exists. | The effective `profdb.bin` (the `XS_PROFDB` folder's, or a model-folder copy if Tekla 2026 uses one). **E0/E2 name it.** Library rows need no `profitab.inp` row (to prove). | **Yes**: `LibraryProfileItem.Delete()`. The bytes of `profdb.bin` will probably not return to the original after a delete, so the hash proof is by file restore; the API delete is proved by `Select == false`. | `LibraryProfileItem.Select`, `GetCrossSection`, then A0m. |
| **M2** | Sketched profile through `CatalogHandler.ImportParametricProfileItems(folder-of-.uel)`, with the `.uel` generated from a template learned by exporting an existing sketch (E1). | **Probably yes** (a sketch has variables, expressions and faces, like the `.clb`), **if** the `.uel` is a text container we can write. Holes only as the bridged single face. To prove. | The effective `profdb.bin`, and maybe `profitab.inp` for the prefix row. **E2 names which.** | **No**: no parametric Delete. File restore only (Tekla closed). | `ParametricProfileItem.SelectByPrefix/Select` (type, order, sizes), `CreateProfileString`, then A0m. Formulas cannot be read back through the API; compare the re-exported `.uel` text. |
| **M3** | `.clb` file plus its `profitab` line placed where Tekla loads clb libraries, then Tekla reloads the catalog. This is how Tekla's own environments distribute sketched shapes. | **Yes**, one-to-one with kind-2 (see 2.2). | Candidates: a `.clb` in the `XS_PROFDB` folder, `profitab.inp` (the row), and the compiled `profdb.bin`. **E3 names which, and whether the API alone can trigger the load or only the Profile catalog dialog's Import can.** | **No** by API. File restore only. | As M2. |
| M4 | Write `profdb.bin` / `profitab.inp` ourselves. | n/a | n/a | n/a | **Rejected**: undocumented binary, no API. |

**Recommendation (to be confirmed by E2/E3):** the shape that suits FloLess is a sketched user parametric shape, created by **M2 if E1 shows the `.uel` is writable text, otherwise M3**. M1 is only the fallback for "a fixed instance of a shape the environment already has" and is the only mechanism with a clean API rollback. Neither M2 nor M3 can express a definition whose loops are not polygons of formulas (arcs, tapered faces) until the `.clb` grammar for those is checked; the first slice is the polygon case, as in the epic's first-delivery note.

## 3. Where the write lands: open decision §11.4

Hypotheses, to be settled by E0 (paths) and E2/E3 (observed byte changes), never by assumption:

| | Hypothesis | If true |
|---|---|---|
| H1 | The write goes to the `profdb.bin`/`profitab.inp` in the **`XS_PROFDB` folder of the environment** (here `…\environments\default\General\Shared\Profil\`, a ProgramData path shared by every model and every user of that environment). | A2 changes a shared, possibly firm-level catalogue. The consent card must name the folder; **needs the master's OK before any write there.** |
| H2 | The write goes to a catalogue **in the model folder** (a model-local `profdb.bin`/`profitab.inp`, or `ParametricProfiles\`). | Safest: scoped to one model. A2 can promise "this model only". |
| H3 | It lands in the **firm or project folder** (`XS_FIRM`, `XS_PROJECT`). | Shared across a firm. **Needs the master's OK.** |

Decision rule for §11.4 once observed: H2 means "the model folder only" and the destination identity is the model; H1/H3 mean the destination identity is the **environment folder** (its path and the pre-write hashes of its catalogue files), the proposal card says it is shared, and floless.app must treat the creation as a catalog-level change, not a model-level one.

**Per-mechanism landing file, as the night-shift master asked:**

| Mechanism | File(s) that can change | How the spike finds out | Notes |
|---|---|---|---|
| M1 library copy | `<effective catalogue folder>\profdb.bin` | SHA-256 of every candidate file before and after (snapshot script) | Not `profitab.inp` (to confirm). |
| M2 `.uel` import | `profdb.bin`; possibly `profitab.inp` | same | The import folder is read, never changed. |
| M3 `.clb` + profitab row | the `.clb` copy placed by us, `profitab.inp`, `profdb.bin` | same | We place the `.clb` only inside the scratch environment during the spike. |
| Export (E1) | only the explicit export path under the scratch root | the script lists the model folder before and after to catch a stray write | Never call `Export` without a path. |

**If any experiment's snapshot diff shows a file changed outside the scratch root (the scratch model folder and the scratch environment copy), stop at once, restore, and ask the master before another write.**

## 4. Isolation, backup, restore (applies to every write)

The scratch model alone does **not** isolate a catalogue write, because the catalogue lives in the environment (and possibly the firm folder), not the model. So:

**4.1 Preferred: a scratch environment copy, so a write cannot reach the shared catalogue at all.**
1. Create `C:\A2SPIKE\` (outside `C:\TeklaStructuresModels`, so no model there is touched). Copy the environment's `Profil` folder (the value of `XS_PROFDB`, about 3.5 MB of files) to `C:\A2SPIKE\env\Profil\`.
2. Start Tekla with an environment whose `XS_PROFDB` points at that copy (a scratch `.ini`, via Tekla's `-i` start-up ini option; to confirm on 2026.0). This is the live-session owner's step and goes through the `floless-app-tekla-launch` skill; the spike scripts never launch Tekla. E0 then **proves** the isolation: `advanced_options.XS_PROFDB.value` must be under `C:\A2SPIKE\`, and `import.csx` re-checks it on every write and refuses otherwise.
3. Create a NEW model `C:\A2SPIKE\models\A2SPIKE-<yyyymmdd>-<hhmm>` (never "FloLess UK", never any existing model). All scripts refuse a model whose folder name does not start with `A2SPIKE`.

**4.2 Fallback: the real environment, only with the master's OK**, written into the master's hands as: the folder, the files, the hashes, the reason isolation failed. `import.csx` then needs `shared_catalog_ok` equal to `MASTER-OK:<model folder name>`.

**4.3 Backup before the first write (`snapshot-catalog.ps1`).**
- `snapshot-catalog.ps1 -Mode Snapshot -Folders <XS_PROFDB>,<XS_FIRM>,<XS_PROJECT>,<model folder> -BackupDir C:\A2SPIKE\backup-<n> -Manifest C:\A2SPIKE\backup-<n>.json` hashes (SHA-256) and copies `profdb.bin`, `profitab.inp`, `profile_properties.ail`, `*.clb`, `*.uel`, `*.lis` in each folder (and the model's `ParametricProfiles`, `ExportedProfiles`, `profil`), refuses a non-empty backup folder, and verifies each copy against its original.
- `run-spike.ps1 -Script import -Write -BackupManifest …` feeds the manifest's hashes into `import.csx`, which **re-hashes every catalogue file the write can reach and refuses if any differs**, so a write can only follow a backup of exactly that state.

**4.4 Undo, and proof by hash.**
- An API delete exists only for M1 (`LibraryProfileItem.Delete`). Use it first and check `Select == false`.
- For everything else (and to prove M1's too): **close Tekla**, run `snapshot-catalog.ps1 -Mode Restore -Manifest …` (puts the backed-up bytes back, removes files added since, then verifies), and require the last line `VERIFIED: N files are byte-identical (SHA-256) to the snapshot`. `-Mode Verify` alone prints each `MODIFIED`/`ADDED`/`REMOVED` file with both hashes and exits 1 on any difference. The restore path of the script was exercised on a temporary folder while writing this plan (modified, added and removed files all detected and restored, exit 0).
- **A restore must succeed after every write experiment**, before the next one starts. A mismatch ends the spike.

**4.5 How the restore is reported (to the master and into #858).** Each experiment ends with one block pasted in the spike log (§9 of this file) and the #858 thread: experiment id; files changed (path, hash before, hash after); the exact `VERIFIED` or `NOT IDENTICAL` output of `snapshot-catalog.ps1`; whether Tekla was closed for the restore; and, when not identical, the full difference list and **no further write**. A restore that cannot be proved is reported as a problem to the master immediately, not at the end.

## 5. The experiments, in order

Shared rules: new throwaway model and scratch environment (§4.1); `A2SPIKE` guard in every script; nothing is run against "FloLess UK", "FloLess EU" or any existing model; only the live-session owner launches or closes Tekla; no keystrokes to desktop apps from this tooling. Scripts live in `cli-tekla/spike/profile-create/`; run with `run-spike.ps1`.

| # | Experiment | Writes? | What it proves | Pass | Fail / stop |
|---|---|---|---|---|---|
| **E0** | `probe.csx`: model path, `XS_PROFDB`/`XS_FIRM`/`XS_PROJECT`/`XS_SYSTEM`/… values, every catalogue file with size and SHA-256, counts of user, sketched and clb shapes, and the existence of the planned `AWZ*` names (`prefix_exists`, `library_exists`, `parametric_string_selects`). Also check ACLs on the catalogue folders by hand (`Get-Acl`). | No | Which folders are in play (answers half of §11.4), that the isolation holds, that the test names are free. | `XS_PROFDB` is under `C:\A2SPIKE\`; names free; model is `A2SPIKE*`. | `XS_PROFDB` still the real path and no master's OK: do not write; report. |
| **E1** | `export.csx` on one existing sketched shape (e.g. `TRIANGLE`, or the first from E0's list) into `C:\A2SPIKE\export\`. Repeat for one clb profile if E0 lists one. | Only the export file | What `.uel`/`.clb` Tekla writes for a sketch; whether `.uel` is a text container with the variables, expressions and faces; **whether the formulas are in it** (they are not readable through the API). | A text container whose expressions and faces correspond to the source shape; no change anywhere outside `out_dir` (`model_folder_files_changed_by_export` is empty). | Binary/opaque `.uel`: M2 is dead, go to E3. A stray file in the model folder: note the behaviour, clean it, report. |
| **E2** | M2: hand-build a `.uel` for a trivial kind-2 shape (`AWZ1`, a rectangle `h`,`b`) by editing the E1 export; then `import.csx -action import-uel` (snapshot first, §4.3), then `save` if the catalogue has not changed on disk. | **Yes (catalogue)** | Whether the import works at all; which file changed (answers §11.4); whether `Select`/`SelectByPrefix` sees the new prefix **without restarting Tekla**; whether it is persisted without `SaveProfileDatabase`; behaviour on a second import of the same prefix (expected: silent overwrite, to confirm). | Import returns true; `SelectByPrefix("AWZ1")` true; exactly the expected file(s) differ in the snapshot diff, all under the scratch root. | Import false or throws: record the exception text. A file changed outside the scratch root: stop, restore, ask the master. Then **close Tekla, restore, require VERIFIED.** |
| **E3** | M3: place a `.clb` with a `profitab` line for `AWZ2` (copied from the `TeklaInc.clb` pattern, rectangle) in the scratch `XS_PROFDB`, then see whether (a) Tekla sees it after reopening the model, (b) `ImportParametricProfileItems`/`ImportLibraryProfileItems` accept a folder holding the `.clb` (probe: they are documented for `.uel`/`.lis`), (c) it needs the Profile catalog dialog's Import (a **manual** step by the live-session owner; record exactly which). | **Yes (catalogue)** | Whether `.clb` can be created by API alone; what the dialog route changes on disk; whether the `profitab` row is also needed. | Same checks as E2. | Only the dialog works: record it; M3 then cannot be an AWARE command and A2 falls back to M2 or to M1. Restore and verify as in E2. |
| **E4** | M1: `import.csx -action library-copy` of an existing library profile to `AWZ3`, then `library-delete`. | **Yes (catalogue)** | Which file a library copy changes; that API delete leaves `Select == false`; whether `profdb.bin` returns to its original bytes after delete (expected no). | Copy true, `Select("AWZ3")` true; delete true, `Select` false. | Any file changed outside the scratch root: stop. Then restore and require VERIFIED. |
| **E5** | Name behaviour on the survivor of E2/E3 (rolled back and re-created if needed): longest accepted prefix (the `profitab.inp` prefix column is 9 wide in the files seen), allowed characters, whether a prefix derived from a digest (`AW`+hash) is accepted, whether prefix collisions with a library name or an existing prefix are refused or silently shadow, and the effect of importing the same prefix with a **different** definition (confirms the overwrite hazard and fixes the conflict rule). | Yes (catalogue) | The deterministic-name rule and the conflict rule the contract (§6) needs. | A rule that makes "same digest, same name" and "different digest, never the same name" hold. | No safe derived name: the command must refuse to proceed on a name clash and ask for another prefix. |
| **E6** | Read-back with A0m: `profile-project`-style name check (`CreateProfileString`) and `section-measure` for 3 sample size sets of the survivor shape; compare Tekla's outer and hole loops, area, bounds and reference point with FloLess's evaluation of the same definition. Include one definition with a hole and one with an asymmetric section. | No | That the created shape evaluates to the definition, vertex for vertex, in Tekla's own section; that a hole survives the bridged face; the reference point. | Max vertex deviation ≤ 0.001 mm and area equal within A0m's tolerance for every sample. | Any deviation: report with both loops; never hide it. |
| **E7** | Failure behaviours: import with a malformed definition, a duplicate prefix inside one folder, a folder with two shapes where the second is bad (partial import), and Tekla closed mid-way is **not** tested. Record what exists afterwards by `profile-catalog parametric:true` and the file diff. | Yes (catalogue) | What "partial failure" looks like so the command can report exactly what exists. | Behaviour documented; each outcome is distinguishable by read-back. | An outcome that cannot be told from success: the command must write one shape per call, and report. |
| **E8** | Final restore: close Tekla, `snapshot-catalog.ps1 -Mode Restore`, require `VERIFIED` on the first (pre-E2) snapshot; delete `C:\A2SPIKE\` model and scratch environment. | Restores | The shared/original catalogue is byte-identical to before the spike. | `VERIFIED: N files are byte-identical`. | Not identical: report to the master with the list; no further action in that environment. |

**Order and gates.** E0 and E1 first (no catalogue write). E2 only after E0 proves isolation (or the master's OK is in hand). Each write experiment ends with a restore and a `VERIFIED` before the next. E3 may be skipped if E2 works and the `.uel` route is clean; E4 can run any time after E0 and is the cheapest write.

**Needs from a live Tekla:** about 3 to 4 hours of exclusive use of one Tekla 2026 instance (exactly one reachable instance, as the bridge requires), a throwaway model in `C:\A2SPIKE\models`, a scratch environment copy (or the master's written OK to use the real one), Tekla closed and reopened at least three times for restores, and the live-session owner for launching, the model creation, and the one possible manual dialog step in E3. A 2025 repeat of E0 and E2 is another hour. No admin rights are expected (the `ProgramData\Trimble` files here are owned by the user); E0 checks.

## 6. Draft contract: `aware-tekla profile-create` (step 2)

Not built here. The spike's findings decide the `write_level`, the mechanism and the name rule; the rest stands.

```powershell
'{"definition":{...},"definition_digest":"sha256:…",
  "target":{"model_path":"C:\\…","environment":{"name":"default","profile_database":"…\\Profil\\"},"tekla_version":"2026.0"},
  "prefix":"AWFL…","sample_sizes":[{"h":300,"b":150}], "mode":"plan"}' |
  aware-tekla profile-create --json-stdin
```

**Input.** `definition` is the neutral FloLess `floless.section-profile` kind-2 payload as P0 fixes it (parameters in mm, outer and hole loops with formulas, reference point, name grammar); `definition_digest` the caller's digest of it; `target` the identity the person saw when they accepted the proposal (the three values `section-measure` already reports, plus the Tekla version); `prefix` optional (otherwise derived from the digest, per E5); `sample_sizes` the sizes to verify (1 to 8); `mode` is `plan` (no write) or `create`. `create` needs `consent: {"source": "ui-click" | "chat-proposal-card"}`: the verb is never reached from a bake.

**Checks before any write, in this order, each refusal named and exit 2 before Tekla is touched where it can be:**
1. Definition validity (finite sizes, closed non-self-intersecting loops, expressions in the allowed grammar) → `invalid-definition`.
2. `definition_digest` recomputed by AWARE over the canonical form of `definition` equals the one sent → else `digest-mismatch`.
3. Exactly one reachable Tekla, a model open → as every verb.
4. **Target identity re-check:** the open model path, environment name, `XS_PROFDB` and Tekla version equal `target` → else `target-changed` (nothing written).
5. **Catalog level:** the folder the write will land in is the one E0/E2 showed, and it is reported (`write_level`: `model` | `environment`). If it is not under the model folder, the receipt says it is shared.
6. **Pre-write snapshot:** SHA-256 of every catalogue file the mechanism can change; copies to `%LOCALAPPDATA%\aware\tekla-profile-create\<model key>\<run id>\backup\`; refuse if a copy does not verify (`backup-failed`).
7. **Existing entry under the name:** select it. If its re-exported definition text and A0m section at the sample sizes equal this definition's → `reused-identical` (nothing written, success). If it differs → `conflict` (nothing written, both sides reported; **never overwritten silently**, because Tekla's import would). If it exists as a library profile or built-in prefix → `name-taken`.

**Write.** One shape per call, by the mechanism the spike chooses; no `CommitChanges` is needed (catalogue, not model); every changed file is recorded in a per-run ledger as it changes.

**Verified receipt.** After the write, `ParametricProfileItem.SelectByPrefix`/`Select` read-back (type, name order, each size), then A0m (`section-measure` logic in-process) for every `sample_sizes` entry, compared with the loops FloLess evaluates from the same definition. Result per sample: `max_vertex_deviation_mm`, `area_delta`, `holes_match`, `reference_point`. Any deviation above tolerance gives `read-back-mismatch` with both outlines, and the entry stays (the person decides; the receipt never claims a match it does not have).

**Output (`result`)**, always the same keys:
```json
{ "status": "created | reused-identical | conflict | name-taken | target-changed | digest-mismatch | invalid-definition | backup-failed | read-back-mismatch | partial | failed",
  "target": { "model_path": "…", "environment": { "name": "…", "profile_database": "…" }, "tekla_version": "2026.0", "write_level": "model | environment" },
  "profile": { "prefix": "…", "profile_string": "…", "mechanism": "uel-import | clb | library-copy" },
  "files": [ { "path": "…", "sha256_before": "…", "sha256_after": "…", "backup": "…" } ],
  "exists_afterwards": [ "…every shape/profile Tekla reports for this call, by read-back, not by intention…" ],
  "verification": { "samples": [ { "sizes": {…}, "max_vertex_deviation_mm": 0, "area_delta": 0, "holes_match": true, "reference_point": [0,0] } ] },
  "undo": "Close Tekla and restore the files listed under `files` from `backup`; Tekla's API has no delete for a parametric profile." }
```

**Partial failure.** Any exception after the first changed file sets `status: partial` and lists `exists_afterwards` from a fresh read-back plus the ledger, with the backup paths; the command never retries and never deletes what it cannot prove it made.

**Guard rails in the manifest:** `mode: write`, a `safety:` block saying it changes the person's Tekla catalogue (the folder named in `write_level`), declared `idempotent` only for the `reused-identical` path; exit codes follow `profile-project` (0 ok, 2 refused before Tekla, 4 host-unreachable).

## 7. Recommendation and what is still undetermined

- **Recommended mechanism (provisional):** a sketched user parametric shape (M2 via `.uel`, else M3 via `.clb`), because only a sketch carries named sizes and formula loops, which is exactly the FloLess kind-2 definition; M1 only for fixed instances of an existing shape. The A2 command must write **one shape per call** and verify with A0m.
- **Run 3 results (2026-11-19):** the API alone cannot create a usable parametric profile from a `.clb`. `ImportParametricProfileItems` registers a placeholder prefix with no section even when the `.clb` is also in the scratch `XS_INP` folder, and a running Tekla does not rescan `XS_INP`, the model folder or `model\profil`. M3 therefore means "place the `.clb` in an `XS_INP` folder and restart Tekla (or use the dialog)"; it cannot be a one-call AWARE command unless the restart read shows otherwise. The `profitab` prefix column decides the prefix, not the shape name; allow `[A-Z0-9_-]`, at most 8 characters (9 and above gave a broken `PROFILE_UNKNOWN` half-entry that `SelectByPrefix` cannot see, so a read-back must enumerate). `ProfileString` plus `section-measure` read-back is proven on an existing clb shape (exact loops and area), so the A0m verification step holds once a section exists.
- **Undetermined until live:** whether a sketched `.uel` is a writable text container; whether the API alone can load a `.clb`; which file each write changes and therefore whether the write is model-level or environment-level (§11.4); whether Tekla 2026 reads a model-local catalogue; whether a new prefix is visible without restarting Tekla; prefix length and character rules; how holes survive the bridged face; behaviour of tapered (multi-face) shapes; whether the scratch-ini start (`-i`) isolates `XS_PROFDB`.
- **Known risks:** `Import` overwrites silently (mitigated by the pre-write select); no parametric delete through the API (mitigated by backup and file restore, which needs Tekla closed); the catalogue may be shared by a firm.

## 8. Files on this branch

- `cli-tekla/spike/profile-create/probe.csx`, `export.csx`, `import.csx`: the experimental scripts (run through the existing `exec` verb, so no new verb and no manifest or parity change).
- `cli-tekla/spike/profile-create/run-spike.ps1`: wraps a script into an `exec` request; `import` needs `-Write` and a backup manifest.
- `cli-tekla/spike/profile-create/snapshot-catalog.ps1`: `Snapshot`, `Verify`, `Restore` with SHA-256 proof.
- `cli-tekla/Tests/ProfileCreateSpikeTests.cs`: parses each script and pins every guard on the source. The three scripts were also compiled against the Tekla 2026.0 assemblies from the install folder (metadata only, Tekla not started) with the bridge's own script options; no errors.

## 9. Spike log

2026-11-18 (evening): **not started, blocked before E0.** A Tekla 2026 instance (PID 32968, started 13:30, command line without a model, window title "FloLess UK") was already running, so the spike's own Tekla could not be started under the rules: `tekla-launch.ps1` refuses a second Tekla (exit 4), and the bridge needs exactly one reachable instance, so a second instance could make Open API calls bind to the one holding the "FloLess UK" model. Nothing was launched, probed or written. Done safely: the real environment's `Profil` folder (87 files) was hashed read-only into `C:\A2SPIKEeal-profil-before.json` (`profdb.bin` 540222 bytes sha256 767a38504ba8e44452bfad5df59032041ae6ac9aa5d83d5c8591e33c5ae221ba is the abbreviated console view; the full hashes are in that file). Awaiting the master's decision.

### Run 2 (evening 2026-11-18): live results so far

Setup: Tekla 2026.0 started by `tekla-launch.ps1` with a bypass ini (`XS_DEFAULT_LICENSE=PARTNER`, documented in Tekla's own `Bypass.ini`), environment ini = copy of the default environment ini with only `XS_PROFDB` pointed at `C:\A2SPIKE\env\Profil\`, model `C:\A2SPIKE\models\A2SPIKE-20261118-2100` (a copy of an empty "New model 4"). Raw outputs are in `C:\A2SPIKE\E*.json` (not committed).

- **E0 (probe), PASS.** `XS_PROFDB = C:\A2SPIKE\env\Profil\`, `XS_FIRM` and `XS_PROJECT` empty. 284 parametric shapes: 64 built-in, 220 user, of which 218 are clb profiles and 2 sketched. **`XS_INP` also lists the shared `...\2026.0\environments\common\profil\`, where the clb shapes are read from**: that folder is a second write target to isolate (scratch copy under `C:\A2SPIKE\env\common-profil\`, `XS_INP` overridden in the scratch ini).
- **E1 (export), PASS.** `Export` of the clb shape `TRIANGLE` to an explicit path wrote a plain-text `.clb` (654 bytes, sha256 4c1b8357...7ff2): `library_id`, `section_type { name; profitab "..."; base_attribute...; expression { name; type y|z; formula }; geometry { face { index 0; point 0 y z } } }`, the same grammar as the shipped `.clb` files. No file was written to the model folder. `TRIANGLE` reports `IsCLBProfile = true`, `IsSketchedUserParametric = false`.
- **E4 (library copy and delete), PASS.** `LibraryProfileItem.Copy("HEA200" -> "AWZ3")` returned true and `Select("AWZ3")` became true in memory with **no file changed**. `CatalogHandler.SaveProfileDatabase()` then **created `profdb.bin` in the MODEL folder** (540207 bytes, sha256 3111279e...4057); the `XS_PROFDB` `profdb.bin` (540222 bytes, 767a3850...21ba) and `profitab.inp` stayed byte-identical. `LibraryProfileItem.Delete()` returned true and `Select` became false; after another save the model's `profdb.bin` was 540076 bytes (b1e44577...de6b), i.e. **not the original bytes** (the hash proof is a file restore). **Answer to §11.4 for M1: the write lands in the model folder's own `profdb.bin`.**
- **E3b (`ImportParametricProfileItems` on a folder holding a `.clb`), PARTIAL.** The call returned true and `ParametricProfileItem.SelectByPrefix("AWZ2")` and `Select("AWZ2")` became true with no file changed, but the item is a placeholder (`PROFILE_USER_PARAMETRIC`, subtype 999001, parameters h, h2, t, w, w2 with defaults that are not the `.clb`'s), `AWZ2300*150` stays `missing`, and `section-measure` reports `no-section`. So an import of a `.clb` folder registers a prefix without its section; the definition has to be read from the `.clb` library in an `XS_INP` folder. Nothing was saved; Tekla was ended and the restore verified.
- **Restore after the first Tekla.** Tekla 8208 was ended by stopping that PID (its Exit save prompt cannot be answered without a click; the model is a throwaway). `snapshot-catalog.ps1 -Mode Restore` removed the model's `profdb.bin`, then `Verify` printed `VERIFIED: 9 files are byte-identical (SHA-256)` against snap-0 (taken before any write), re-checked after the process had gone.
- **Operational notes.** The dead PID 8208 keeps showing in `Get-CimInstance Win32_Process`, so `tekla-launch.ps1` refuses (exit 4, "not a release the bridge can read"); the second Tekla (PID 19344) was started by hand with the identical command line. A model left locked raises Tekla's "Open ... not properly closed" notice, which needs one OK click.

Pending in Run 2: M3 proper (a `.clb` placed in the scratch `XS_INP` folder `C:\A2SPIKE\env\common-profil\AWFL.clb`, plus a model-folder `AWFL2.clb` and `profil\AWFL3.clb` to see which folders Tekla reads), the hole shape `AWZ7`, `ProfileString` and `section-measure` read-back, prefix rules, and the final restore and the real-folder proofs.

### Run 2 stopped (2026-11-19, 03:00)

- **Stopped before M3 ran.** The second Tekla (PID 19344, started by hand at 22:20) opened on Tekla's "Open ... not properly closed" notice, which needs one OK click; computer-use was not granted, so nothing ran after 22:20. The night-shift master told me to stop. PID 19344 was ended by its own PID (scratch model, nothing to save). The three M3 probe files (`C:\A2SPIKE\env\common-profil\AWFL.clb`, model-folder `AWFL2.clb`, `profil\AWFL3.clb`) had been placed but **never read by Tekla**, so M3 is still undetermined.
- **Scratch restore.** `snapshot-catalog.ps1 -Mode Restore` against snap-3 removed the three added `.clb` files and printed `VERIFIED: 64 files are byte-identical (SHA-256)` (snap-3, 21:18 UTC, scratch `Profil`, scratch `common-profil`, the model). `Verify` against snap-0 (before any write): `VERIFIED: 9 files are byte-identical`.
- **Real folders untouched.** Every file in each baseline re-hashed equal, and no file was added: default `Profil` 87 files (`profdb.bin` 540222 bytes, 767a3850...21ba), UK `profil` 30 files (`profdb.bin` 1866487 bytes, f0496a68...b479), shared `common\profil` 59 files. Baseline manifests: `real-profil-before.json` sha256 727685713afe..., `real-uk-profil-before.json` d30c0a9bbaf2..., `real-common-profil-before.json` 26e97c0ca13c....
- **Next window:** create a fresh `A2SPIKE-*` model per run (no recovery notice), close Tekla only through its own exit, and run M3, the hole shape `AWZ7`, `ProfileString` / A0m read-back and the prefix rules.

### Run 3 (2026-11-19, morning)

Setup: Tekla 2026.0 PID 28392 (started by the live-session owner, not by this run), model `C:\A2SPIKE\models\A2SPIKE-20261119-0642` (fresh copy of "FloLess UK"), scratch ini as in Run 2. The installed `~\.aware\bin\aware-tekla.exe` has no `exec` verb, so every call used the worktree build `cli-tekla\bin\Debug\net48\aware-tekla.exe`. Baseline snapshot `snap-r3-0`, then `snap-r3-1` to `snap-r3-5` before each write step. Note: `snapshot-catalog.ps1 -Folders` must be given through `pwsh -Command "& ./snapshot-catalog.ps1 ..."`; with `-File` the comma list arrives as one string and the snapshot holds 0 files. Raw outputs: `C:\A2SPIKE\R3-*.json` (not committed). Free memory stayed above 10 GB.

- **E0 (probe, `R3-E0.json`), PASS.** `XS_PROFDB = C:\A2SPIKE\env\Profil\`, `XS_FIRM` and `XS_PROJECT` empty, `XS_INP` = real `common\inp`, **scratch `C:\A2SPIKE\env\common-profil\`**, real `common\collaboration\ifc`, install `Env\Common\inp`. 282 parametric shapes, 218 clb, **0 sketched** in this model (Run 2's 2 sketched ones came from the other model), so there is no sketched shape here to export. `AWZ1/2/3/7`, `AWFL`, `AWFL2`, `AWFL3` were all free.
- **M3 proper, which folders does Tekla read (`R3-M3a-live.json`): NOT read by the running Tekla.** `AWFL.clb` (common-profil, the scratch `XS_INP` folder), `AWFL2.clb` (model folder) and `profil\AWFL3.clb` (model sub-folder) were placed, each a rectangle `h`,`b` with an inline `profitab` line in the grammar of the E1 export. A probe afterwards: `SelectByPrefix` and `Select` false for all three, `parametric_total` still 282. So **Tekla does not rescan any of the three folders while running.** Whether it reads any of them at start is **undetermined and needs a Tekla restart** (blocker below). `AWZ7.clb` (hole shape: outer `h` x `b`, hole inset by `t`, face index 1) was also placed in common-profil, not imported, so a restarted Tekla tests M3 and the hole in one go.
- **M3b: `ImportParametricProfileItems` with the `.clb` also present in the scratch `XS_INP` folder (`R3-M3b-import-with-xsinp-clb.json`, `R3-M3b-measure-AWZ4.json`, `R3-M3b-save.json`).** Import of a folder holding `AWZ4.clb` (copy also in common-profil) returned true; `SelectByPrefix("AWZ4")` and `Select("AWZ4")` became true, but it is **the same placeholder as Run 2's AWZ2**: type `PROFILE_USER_PARAMETRIC`, subtype 999001, `IsCLBProfile` true, parameters h=232, h2=112, t=2, w=165, w2=65, empty `ParameterString`, `CreateProfileString()` = `AWZ4`, `section-measure` of `AWZ4300*150` = `missing` and of `AWZ4` = `no-section`. Having the library file in `XS_INP` does not give the placeholder its section: **the API import of a `.clb` registers only a prefix.** Files: the `XS_PROFDB` `profdb.bin` (767a3850...21ba) and `profitab.inp` (d4163ec4...63aef) stayed identical. **The call created `models\A2SPIKE-20261119-0642\profdb.bin` (540076 bytes, sha256 b1e44577eaa7dbe1db59a3978a1ac72a67ea8be96471652c3ee9542fac9dde6b, created at the import, not at a save)**; a later `SaveProfileDatabase()` rewrote it with identical bytes. That is the same hash as Run 2's E4 file after copy, delete and save, so the placeholder prefix is **not stored in that `profdb.bin`**: the registration lives in memory or in a file this snapshot does not cover, and whether it survives a restart is for the restart to show.
- **Prefix rules (`R3-prefix-rules.json`, `R3-prefix-probe.json`, `R3-list-aw.json`), measured on the same import route.** The prefix comes from the **`profitab` prefix column, not from the `section_type` name or the `library_id`**: a file with `section_type` name `AWZ8X` and profitab prefix `AWZ8` registered `AWZ8`, and `AWZ8X` does not exist. Accepted: `-` and `_` (`AWZ6-X`, `AWZ6_Y`) and digits (`AWZ12345`, 8 characters). **A prefix longer than 9 characters is broken**: `AWZ5LONGPREFIXNAME` was registered as `AWZ5LONGP` (cut at 9) with type `PROFILE_UNKNOWN`, no subtype, and `Select` and `SelectByPrefix` are false for it and for every shorter variant tested. That is a silent half-registration that `SelectByPrefix` cannot reveal; only the enumerator shows it, so a read-back must enumerate. Rule for the command: prefix characters `[A-Z0-9_-]`, at most 8 (9 exactly not tested). Lowercase is blocked by the spike's `AWZ` guard, so not tested. Parametric total went 282 to 288 (the four good ones, `AWZ4`, and the broken `AWZ5LONGP`). No `Profil` file changed.
- **ProfileString and A0m read-back (`R3-profilestring.json`, `R3-readback-existing-clb.json`, `R3-readback-hole-existing.json`).** For a **shape Tekla loaded at start** the pipeline works: `CreateProfileString()` for `TRIANGLE` = `TRIANGLE381*508` (default sizes joined with `*`), for `OBLVAR_C` = `OBLVAR_C30-50-20-60-100-40-0-0` (the `-` separator of its `profitab` parameter string), `REC_E` = `REC_E30-50-10`. `section-measure TRIANGLE400*200` is `ok` with outer loop `[[-100,200],[-100,-200],[100,-200]]`, area 40000, bounds exact and reference point [0,0]: the clb `y`/`z` map to section x = z, y = y (point `y1 z2` with h=400, b=200 gives (-100, 200)), so the loop equals the formula evaluation exactly. A bare prefix with no sizes gives `no-section`, so the check needs a full name. `OBLVAR_C` evaluated with no hole at the two sizes tried, so **hole survival is still untested** (needs `AWZ7` after a restart). For the placeholders (`AWZ4`, `AWZ6-X`) `CreateProfileString()` returns just the prefix and `section-measure` says `no-section`.
- **Not run, and why.** `AWZ7` evaluation (needs a restart), the sketched `.uel` route (this model has no sketched shape to export and hand-edit, so E2 stays open), a second import with a different definition (the placeholder carries no definition, so it would show nothing), and a dialog Import (a click).

Files created or changed in Run 3 (all under `C:\A2SPIKE\`):

| Path | State | sha256 (after) |
|---|---|---|
| `env\common-profil\AWFL.clb` | added | dbc6bdc514883ec5ffc4803f06c5a868ce9bab307817a1673050677611306311 |
| `env\common-profil\AWZ4.clb` | added | 0367f520c3431c44ff697dbd83ef3e50e69ba9b1e6560242a0e3fb63168bbd3a |
| `env\common-profil\AWZ7.clb` | added | 3fd4508dc0c02a9f908c66cccbac4ca1272d81ba375125bf75d82925104cae33 |
| `models\A2SPIKE-20261119-0642\AWFL2.clb` | added | 1456df27a891eff5f045a3611141df4264ec5aabddfa4e308d59c13cedf7b9d0 |
| `models\A2SPIKE-20261119-0642\profil\AWFL3.clb` (the `profil` folder is new too) | added | a6f39cf725ea0707eec1e34b757d917a19e3b2a4cf65bc2f9af660486741d5e3 |
| `models\A2SPIKE-20261119-0642\profdb.bin` | added by the import (before: absent) | b1e44577eaa7dbe1db59a3978a1ac72a67ea8be96471652c3ee9542fac9dde6b |

No existing catalogue file changed: `Verify` against snap-r3-0 lists exactly those 6 additions, and `Profil\profdb.bin` (767a3850...21ba) and `profitab.inp` (d4163ec4...63aef) are unchanged. Nothing was written under the real, default or UK environments. Also created, not catalogue: `stage\`, `import-r3\`, `import-r3b\`, `backup-r3-1..5\`, `snap-r3-1..5.json`, `req-*.json`, `R3-*.json`; Tekla also rewrote its own model files. **Restore:** with Tekla closed, `snapshot-catalog.ps1 -Mode Restore -Manifest C:\A2SPIKE\snap-r3-0.json` removes the 6 added files; the empty `profil` folder in the model folder stays and can be deleted by hand.

**Blocker for M3:** the running Tekla never read the three folders. To settle it, Tekla has to be restarted on the scratch ini by the master with the files now in place, then run `probe.csx` with prefixes `AWFL,AWFL2,AWFL3,AWZ7` and `section-measure` of `AWFL300*150` and `AWZ7300*200*20`. The restart also shows whether the Run 3 placeholder prefixes survive. New read-only scripts from this run: `reflect.csx`, `list-aw.csx`, `profile-string.csx`.

### Run 3, restarts (2026-11-19, 07:13-08:55)

- **Restart with the `.clb` files in place, no `profitab.inp` row:** Tekla loaded **none** of them. `AWFL` in the scratch `XS_INP` folder, `AWFL2` in the model folder, `AWFL3` in `model\profil` and `AWZ7` in `XS_INP` all stayed absent (AW count 0 of 282, `R3r-list-aw.json`). The Run 3 API placeholder `AWZ4` did not survive the restart either. A `.clb` file alone, in any of these folders, is not read.
- **Why:** Tekla's own clb shapes are registered by a row in `profitab.inp` in the `XS_PROFDB` folder that names `LIBRARY.SECTION` (e.g. `TRI_D ! USER ! … !TEKLAINC.TRIANGLE !h*b`). The `profitab` line inside the `.clb` is not enough.
- **Restart with `AWFL` and `AWZ7` rows appended to the scratch `profitab.inp`** (snapshot `snap-r3-6` first): both prefixes now register (284 shapes, `PROFILE_USER_PARAMETRIC`, `clb: true`, `R3p-list-aw.json`), and `SelectByPrefix` finds them. **But the section is still empty.** `section-measure` of `AWFL300*150`, `AWFL400*200`, `AWZ7300*200*20` and `AWZ7400*250*15` gives `no-section` (`R3p-measure.json`). The listed parameters (`h, h2, t, w, w2`, defaults 232/112/2/165/65, identical for both) are a generic placeholder, not the clb's. Control in the same session: `TRIANGLE400*200` and `TRI_D400*200` measure `ok` (`R3p-measure-control.json`), so measurement works and a `profitab` row pointing at a library Tekla has loaded works. **Open:** why Tekla does not load the new `AWFL`/`AWZ7` libraries from `XS_INP`. The files differ from `TeklaInc.clb` only in layout (indentation, a `default` inside each `expression`, attribute order). The next discriminating experiments: (a) a `profitab` row pointing a new prefix at an existing section (`AWTRI … !TEKLAINC.TRIANGLE`); (b) the new section appended inside a copy of `TeklaInc.clb`; (c) the `.clb` in the `XS_PROFDB` folder.
- **Operational:** a Tekla closed from its own Exit *without* saving leaves `*.db1_bimst`-style autosave files, and the next start stops on "Autosaved data exists". Deleting those files in the throwaway copy before a start avoids it (done once, by PID-stopping the stuck Tekla this session had started, which never opened the model). The clean exit route without a person: `new ModelHandler().Save()` through `exec`, then `Operation.RunMacro(@"C:\A2SPIKE\macros\A2Exit.cs")` calling `akit.Callback("acmd_exit_program", "", "main_frame")`. Tekla still asks "save before exit?" (and about keeping the licence). That one prompt was answered **Save** with screen control, which was granted this time.
- **Restore:** with Tekla closed, `Restore` against `snap-r3-0` removed the six Run 3 additions (`AWFL.clb`, `AWZ4.clb`, `AWZ7.clb`, `AWFL2.clb`, model `profdb.bin`, `profil\AWFL3.clb`) and restored `profitab.inp`; `VERIFIED: 64 files`. `Verify` against `snap-0`: 9 files identical. The empty `model\profil` folder was removed. The three real folders re-hashed equal: default `Profil` 87 files, UK `profil` 30, `common\profil` 59, nothing added.

### Run 3, discriminators (2026-11-19, 09:00)

One Tekla session, all files in the scratch environment (snapshot `snap-r3-7` first; the installed `TeklaInc.clb` and the real folders untouched). Raw: `R3d-list-aw.json`, `R3d-measure.json`.

| Route | Registers | Measures a section | Sizes honoured |
|---|---|---|---|
| (a) `profitab.inp` row `AWTRI → TEKLAINC.TRIANGLE` (new prefix, existing section) | yes, `parameter_string h*b` | yes | **yes**: `AWTRI400*200` = `TRIANGLE400*200` exactly (area 40000) |
| (b) new `section_type AWTK` appended inside the scratch copy of `TeklaInc.clb` + row `AWTK → TEKLAINC.AWTK` | yes, `parameter_string ""` | yes | **no**: `AWTK300*150` and `AWTK400*200` both give the 150×300 default rectangle (area 45000) |
| (c) new library `AWPD.clb` in the scratch `XS_PROFDB` folder + row `AWPD → AWPD.AWPD` | placeholder only (subtype 999001) | no (`no-section`) | — |

**Reading.** Tekla 2026 loads clb sections only from libraries it already knows, and `TEKLAINC` is one of them. A new library file, in `XS_INP` (Run 3 restarts) or in `XS_PROFDB` (c), is never loaded. A section added to a known library loads its geometry (b), but its size parameters do not bind: the `parameter_string` is empty, so every size evaluates the defaults. The likely causes are our section text: the `default` lines inside each `expression`, absent in Tekla's own sections, or `base_attribute` order `b, h` against the `h*b` parameter string. That is the next single experiment. **Only route (a), a row aliasing an existing section, registers and measures correctly sized sections today.** Route (b) is the candidate for genuinely new shapes once the size binding is fixed. Writing into a known library means the destination's catalog file changes (a Tekla-owned file), so the command needs the backup/receipt discipline in §6.

Exit: `ModelHandler.Save()` + `RunMacro(A2Exit.cs)`, with the "save before exit?" prompt answered **Save** under screen control; no autosave files left. Restore against `snap-r3-0`: AWPD.clb removed, `TeklaInc.clb` and `profitab.inp` restored, `VERIFIED: 64 files`; `snap-0` 9 files verified; real folders re-hashed equal (87 / 30 / 59, nothing added).

### Run 3, size binding (2026-11-19, 09:10), inconclusive

Four sections appended to the scratch `TeklaInc.clb` copy, each with its `profitab.inp` row (snapshot `snap-r3-8`; raw `R3s-list.json`, `R3s-measure.json`):
- `AWB1`: our rectangle without the `default` inside each `expression`.
- `AWB2`: the same with `base_attribute` order `h, b`.
- `AWB3`: an exact copy of Tekla's `TRIANGLE` section, as a control.
- `AWB4`: our rectangle with order `h, b`, defaults kept.

**All four registered only as placeholders** (subtype 999001, `no-section`), the `TRIANGLE` copy included. In the same session the built-in `TRIANGLE`, `TRPBL` and `PCTRI` measured `ok`, and the file was well formed: CRLF throughout, braces balanced. So this run says nothing about size binding.

Its one clear difference from the discriminator run, where `AWTK` loaded, is the **name**: every name that has loaded (`AWTK` and Tekla's own seven) is letters only, and `AWB1`–`AWB4` contain a digit. A digit in a section name may be refused, or `AWB1300*150` read as prefix `AWB` plus sizes. **Next:** repeat with letter-only names (`AWBA` no defaults, `AWBB` no defaults with order `h,b`, `AWBC` TRIANGLE copy, `AWBD` our original text), one restart.

Restore: `snap-r3-0` VERIFIED 64 files, `snap-0` 9 files; real folders 87 / 30 / 59 unchanged. The exit was clean (API save, `RunMacro`, Save clicked), and no autosave files are left.

### Run 3, letter-only names (2026-11-19, 09:40)

The same four variants renamed `AWBA`–`AWBD` (snapshot `snap-r3-9`; raw `R3L-list.json`, `R3L-measure.json`). **All four now register as real sections** (subtypes 999338–999341) and measure `ok`. The name rule is therefore: **a clb section name must be letters only.** A digit in the name leaves a placeholder; the size digits after it parse fine (`AWBA300*150`).

**But sizes never bind for an appended section.** Every size gives the section's own defaults:
- `AWBA`, `AWBB` and `AWBD`, our rectangle with or without expression defaults and with either attribute order, all give 150×300 (area 45000);
- `AWBC`, Tekla's own `TRIANGLE` text copied verbatim, gives 381×508 for both `400*200` and `300*100`.

The `parameter_string` stays empty. The section text is not the cause, because the verbatim copy fails the same way. Something tied to the original registration (only an alias of an existing section binds, see route (a)) carries the parameters. A section added to a library therefore cannot carry sizes, which by itself rules route (b) out.

## Owner ruling 2026-11-19: routes for a FloLess profile going to Tekla

Pawel, word for word: *"Any truly new shape we can add as an item. And the item is easy to add in Tekla. So there is this shape and shape geometry XML files, I believe, in a model folder. So we should follow this for any shape that Tekla doesn't have for some reason. This should be our fallback."*

1. **Alias.** A `profitab.inp` row pointing a FloLess prefix at an existing Tekla section, when one matches. Proven (route (a)): registers, measures, sizes honoured. This is the A2 `profile-create` scope.
2. **Fallback for any shape Tekla does not have:** a Tekla **Item** whose shape goes into the model's own shape catalog (`Shapes\`, `ShapeGeometries\` in the model folder). This is the aware#859 route, with the shared-model confirm card and the per-length count approved earlier today.
3. **No writes into `TeklaInc.clb` or any Tekla-owned library.** Route (b) is dropped. It also fails on its own terms, since sizes never bind.

Restore after this session: see the A5 run in aware#859, which used the same Tekla session.
