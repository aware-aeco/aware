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
- **Undetermined until live:** whether a sketched `.uel` is a writable text container; whether the API alone can load a `.clb`; which file each write changes and therefore whether the write is model-level or environment-level (§11.4); whether Tekla 2026 reads a model-local catalogue; whether a new prefix is visible without restarting Tekla; prefix length and character rules; how holes survive the bridged face; behaviour of tapered (multi-face) shapes; whether the scratch-ini start (`-i`) isolates `XS_PROFDB`.
- **Known risks:** `Import` overwrites silently (mitigated by the pre-write select); no parametric delete through the API (mitigated by backup and file restore, which needs Tekla closed); the catalogue may be shared by a firm.

## 8. Files on this branch

- `cli-tekla/spike/profile-create/probe.csx`, `export.csx`, `import.csx`: the experimental scripts (run through the existing `exec` verb, so no new verb and no manifest or parity change).
- `cli-tekla/spike/profile-create/run-spike.ps1`: wraps a script into an `exec` request; `import` needs `-Write` and a backup manifest.
- `cli-tekla/spike/profile-create/snapshot-catalog.ps1`: `Snapshot`, `Verify`, `Restore` with SHA-256 proof.
- `cli-tekla/Tests/ProfileCreateSpikeTests.cs`: parses each script and pins every guard on the source. The three scripts were also compiled against the Tekla 2026.0 assemblies from the install folder (metadata only, Tekla not started) with the bridge's own script options; no errors.

## 9. Spike log

2026-11-18 (evening): **not started, blocked before E0.** A Tekla 2026 instance (PID 32968, started 13:30, command line without a model, window title "FloLess UK") was already running, so the spike's own Tekla could not be started under the rules: `tekla-launch.ps1` refuses a second Tekla (exit 4), and the bridge needs exactly one reachable instance, so a second instance could make Open API calls bind to the one holding the "FloLess UK" model. Nothing was launched, probed or written. Done safely: the real environment's `Profil` folder (87 files) was hashed read-only into `C:\A2SPIKEeal-profil-before.json` (`profdb.bin` 540222 bytes sha256 767a38504ba8e44452bfad5df59032041ae6ac9aa5d83d5c8591e33c5ae221ba is the abbreviated console view; the full hashes are in that file). Awaiting the master's decision.
