# A5 spike plan: sending a member to Tekla as an Item (a shape in the model's shape catalogue)

Issue: aware#859 (floless.app epic #3382, slice A5; spec `docs/superpowers/specs/2026-11-10-parametric-profile-catalog-EPIC.md` §5.3 "Sending a member as a Tekla Item", §6 row A5, §9).
Sister spike: A2 (aware#858), `2026-11-18-a2-tekla-profile-create-SPIKE.md`. This plan keeps A2's shape, guards and rigour.
Status: **plan and spike code only. Nothing in this document has been run against a live Tekla, and Tekla was not started, attached to or driven while writing it.** Everything marked "found" was read from files on disk: the Tekla 2026.0 Open API XML documentation, the assemblies' metadata (read as metadata only, never loaded), the shape-catalogue files of existing models, a sample `.tsc` shape file, and the existing bridge code. Everything marked "to prove" waits for Tekla time.

## 1. The answer in short

- **The route exists in the Open API, and it is a plain insert, not a file import.** `Tekla.Structures.Catalogs.ShapeItem.Insert()` takes a `FacetedBrep` (vertices plus face loops, with inner loops for holes) and adds it to the open model's shape catalogue. A `Tekla.Structures.Model.Brep` (the Open API's "Item") whose `Profile.ProfileString` is the shape's name then places that shape between a start and an end point.
- **The shape catalogue is the model's own.** Every model folder on this machine has `Shapes\<shape guid>.xml` and `ShapeGeometries\<geometry guid>.xml`. The write should land there and nowhere else: not in the environment, not in the firm folder, and not in `profdb.bin` or `profitab.inp`. That is the opposite of A2's profile catalogue (§11.4 there) and it is much better news for consent: "this model only". It is **found in the files and the documentation, and still to be proved by a snapshot diff in E1**.
- **Two file routes exist as fallbacks**: `CatalogHandler.ImportShapeItems(folder)` reads `*.tsc` (a gzip container that holds the two XML files), and `ImportShapeItemsFromGeometryFiles(folder)` reads `.skp .dxf .dwg .step .stp .iges .igs .ifc .dgn .trb` and uses the file name as the shape name. Both "override the existing shape items in the catalog without any warnings", so a silent-overwrite check has to come first, exactly as in A2.
- **Reuse has Tekla support.** `Insert()` "does not allow duplicate geometry", `InsertOrGetGuidsOfShapesWithMatchingGeometry` returns the guids of shapes that already match by Tekla's fingerprint, and `SetMetadata` lets us stamp our own digest on a shape and read it back.
- **Unlike A2, the API can undo.** `ShapeItem.Delete()` and `Brep.Delete()` exist, so what a call created can be removed by API, and `ShapeItem.GetInstanceCount()` says whether a shape is still used.
- **The biggest open question is the length.** The input is a section swept along the member, so the solid has a length. Whether Tekla stretches a shape to the item's start-to-end distance decides whether one shape serves every length or each length needs its own (§10, owner question 1). E4 answers it.

## 2. What the existing code teaches (patterns A5 must follow)

Read at `main` (db0072a63). The A2 plan's table (§1 there) applies unchanged; the points that matter here:

| Pattern | Where | What A5 reuses |
|---|---|---|
| A verb is a Roslyn script embedded as a `const string`, run by `ExecuteResolvedScript(...)`. The `exec` verb runs `ScriptCommitPolicy.Automatic` (the bridge calls `CommitChanges` after the script); shipped write verbs use `ScriptOwned`. | `Program.cs` (`exec` at the `Automatic` call, `profile-catalog`, `section-measure`, `profile-project`, `bake-scene` at `ScriptOwned`) | Spike scripts run through `exec` (no new verb, manifest or parity change). The later `item-create` verb is `ScriptOwned`, because a catalogue insert is not a model change but the item is. |
| `exec` defaults to usings for `Tekla.Structures`, `.Model`, `.Model.Operations`, `.Geometry3d`, `.Drawing`, `.Datatype`. `Catalogs` and `Solid` (where `Face` and `Loop` live) are **not** default, so each script adds them. | `Program.CreateScriptOptions` | Compile check (§11) caught exactly this. |
| Destination identity = `model.GetInfo().ModelPath` plus the environment name from `XS_PROFDB`; refusals before Tekla is touched are exit 2 with the field named; a Tekla-side refusal is a structured receipt. | `section-measure.md`, `SectionMeasureScript.cs` | The A5 target identity is the model path, the model's own GUID (in its `TeklaStructuresModel.xml`), its shared/single-user flag (`ModelInfo.SharedModel`, `.SingleUserModel`) and the Tekla version. |
| Read-back of a member's solid: `Solid.MinimumPoint/MaximumPoint`, faces and loops, `GetCoordinateSystem()`, `GetReportProperty(...)`; the bridge sets a global work plane and restores it. | `SectionMeasureScript.cs:551-626`, `BakeSceneScript.cs:370-497` | `item-place.csx` and `readback.csx` do the same, and compute the volume themselves from the solid's faces so the check does not depend on a report property that may not exist for an Item. |
| Digests stamped as UDAs (`AWARE_SRC_V1:`+sha256) and receipts keyed on them. | `BakeSceneScript.cs:89-95` | The shape carries `aware.digest` as shape metadata, the item carries UDAs (names to confirm in E3), and reuse is decided by digest plus read-back, never by name alone. |
| Receipts keep what is outstanding: a per-run ledger of every object created. | `section-measure.md` | A5 keeps a ledger of every shape and item it created (name, guid, geometry guid, file hashes). |
| The verb surface is guarded (`tekla_verb_parity.rs`): a new verb needs a dispatch case, a manifest entry (`mode: write`), a `safety:` block and `commands/<verb>.md`. | `tekla_verb_parity.rs`, `manifest.yaml` | `item-create` is added in step 2, not in this spike. |
| Existing Tekla skill pages already cover the idea (`20-agents/aeco/engineering/tekla/skills/bent-plates-and-custom-shapes.md`, `structures-catalogs-api-reference.md`) but with **two errors** found against the assemblies: the page writes `ShapeItem.ShapeUpAxis.Z_Axis` (the enum is `Tekla.Structures.Catalogs.ShapeUpAxis`, a top-level type) and `Polymesh.Validate(flags, brep)` (the real signature is `Validate(FacetedBrep, PolymeshCheckerFlags, ref List<KeyValuePair<int, PolymeshHealthCheckEnum>>)`). | skills folder | Not changed by the spike. To fix when step 2 ships, with the skill's version bump. |

## 3. The Open API route to a shape, from the 2026.0 documentation and metadata

Facts below were read from `C:\Program Files\Tekla Structures\2026.0\bin\Tekla.Structures.Catalogs.xml`, `Tekla.Structures.Model.xml`, and the assemblies' metadata tables (field constants, enum values and method names read with `System.Reflection.Metadata`, which does not load Tekla).

### 3.1 What the API has

| Member | What it does | Note |
|---|---|---|
| `ShapeItem.Insert()` | Inserts a shape "based on the shape geometry (does not allow duplicate geometry)". Needs `Name`, `ShapeFacetedBrep`, `UpAxis`. | The documented example is a 300 mm cube with `UpAxis = ShapeUpAxis.Z_Axis`. Tekla's own text says to orient shapes "x-axis on the extrusion line and Y-axis pointing up" and that `Undefined` "may throw". |
| `ShapeItem.InsertOrGetGuidsOfShapesWithMatchingGeometry(out List<string>)` | Inserts, or, "based on fingerprint", returns the guids of shapes that already have that geometry. | The reuse primitive. Throws on invalid shape data. |
| `ShapeItem.InsertUsingNormals()` / `InsertUsingNormalsAllowDuplicates()` | Same, using vertex normals to hide smooth edges. | Only useful for curved faces; a polygon prism does not need it. `InsertUsingNormalsAllowDuplicates` is the only public way to keep a duplicate geometry (the `Insert(bool)` overload is not public: the compile check rejects it). |
| `ShapeItem.Select(name)` / `Select()` | Selects by name / by guid. | |
| `ShapeItem.Delete()`, `Rename(newName)`, `Modify()`, `CleanAndModify(ref bool)` | Catalogue edits. | `Modify()` "based on the shape name or, if not found, the shape guid": the overwrite hazard (E5). |
| `ShapeItem.GetInstanceCount()`, `GetBrepInstanceCount()`, `GetSpaceInstanceCount()` | How many objects in the model use the shape. | Tells whether a shape may be cleaned up. |
| `ShapeItem.Guid`, `GeometryGuid`, `Extrema` (AABB), `Fingerprint`, `IsSolid`, `BrepType`, `HandlePoints`, `UpAxis` | Read-back. | `Guid` is the file name in `Shapes\`; `GeometryGuid` the file name in `ShapeGeometries\`. |
| `ShapeItem.SetMetadata(key, ShapeMetadataTypeEnum, value, label, description)`, `GetMetadata`, `GetAllMetadataKeys`, `DeleteMetadata` | Typed metadata on the shape. Types: `Text`, `DateTime`, `Integer`, `Double`, `Length`, `Area`, `Volume`, `Weight`. | Where the AWARE digest, sizes and length are kept. |
| `ShapeItem.Export(ref string filename)` | Writes a `.tsc` ("if path is not given, shape is exported to model folder"). | Always pass an explicit path. |
| `CatalogHandler.GetShapeItems()` | Enumerates every shape. | |
| `CatalogHandler.ImportShapeItems(folder)` | Imports `*.tsc` from a folder; "throws exception if model is not open"; overrides without warning. | Fallback route. |
| `CatalogHandler.ImportShapeItemsFromGeometryFiles(folder)` | Imports `.skp .dxf .dwg .step .iges .stp .igs .ifc .dgn .trb`; file name is the shape name; overrides without warning. | Fallback route, and FloLess already writes IFC. |
| `Polymesh.Validate(FacetedBrep, PolymeshCheckerFlags, ref List<...>)`, `Polymesh.Fingerprint(FacetedBrep)`, `Polymesh.GetSolidBrep(...)` | Tekla's own check of a brep before it is inserted; its fingerprint; a cleaned solid. | `PolymeshCheckerFlags.All` is 511: it does **not** include `VerticesOnSamePosition` (512), `UnusedVertices` (1024), `LoopVertexUnique` (2048), `NonManifoldEdges` (4096), `NullFaces` (8192) or `MultishellnessGeometrical` (16384). The spike passes `All` and adds the others as an experiment (E2). |
| `ShapeItem.MaxShapeNameLength` | Documented as the maximum name length. | The constant is `1023` in the metadata but is **not public** (the compile check rejects it), so the scripts repeat `1023`. |

No `Tekla.Structures.Catalogs` API takes a section outline and sweeps it: a `FacetedBrep` has to be built by the caller. That is straightforward for a polygon section with holes (§5 below), and is the same builder the production command will use.

### 3.2 `FacetedBrep` input

`new FacetedBrep(Vector[] vertices, int[][] outerLoops, Dictionary<int, int[][]> innerLoops)`: one vertex list, one outer loop per face (indices into the list, winding by the right-hand rule, outward normal), and for each face that has holes a list of inner loops keyed by the face's index. The documented cube winds each face counter-clockwise seen from outside. How a face's inner loops should wind relative to its outer loop is not stated; E2 tries both and lets `Polymesh.Validate` and `IsSolid` decide.

### 3.3 The shape files Tekla writes (found in an existing model, "Aware Tests")

- `Shapes\5268e8f9-…xml`: an `<ImportPart>` with `Name`, `BrepStorageId` (the geometry guid), `Guid`, `Extrema` (min and max point), `BrepGeomType`, `BrepType` (0 = building product, 1 = structural shape) and `Fingerprint` (for example `232.126.696.8-37638.051172230553`).
- `ShapeGeometries\323690fd-…xml`: a `<Polymesh>` with `<Points>`, then faces. **Coordinates are stored as 32-bit floats**: a 425 mm vertex reads back as `424.99999702178872` and a 200 mm one as `200.00000297820586`. So an exact round trip is impossible, and every comparison tolerance in this spike (1e-3 mm on a coordinate near 1 m, 1e-5 relative on volume) is chosen from that, not assumed.
- Real shapes show up as a name plus two guid-named files; the model database (`*.db1`/`*.db2`) holds the instances.

### 3.4 The `.tsc` container (found in a sample shape of another project, read-only)

A `.tsc` is gzip over a text container: `<FileInfo><Name>…</Name><Size>…</Size><Compressed>0|1</Compressed></FileInfo>` headers, each followed by that file: an `INFO` entry (shape name, shape guid, geometry guid), then `<guid>.xml` (the `ImportPart` above) and `<geometry guid>.tez` (the Polymesh XML, compressed). It is the same family as the `.uel` A2 found. Writing a `.tsc` by hand would need Tekla's own fingerprint and a compressed geometry blob, so **`Insert()` is the primary mechanism and `.tsc` is only a fallback** (E8).

### 3.5 Mechanisms ranked

| # | Mechanism | Fit | Lands in | Reversible by API | Verdict |
|---|---|---|---|---|---|
| **M1** | `ShapeItem.Insert()` from a built `FacetedBrep`, `SetMetadata`, then an Item (`Brep`) with `ProfileString` = shape name | Exact: the brep is the neutral solid | The open model's `Shapes\` and `ShapeGeometries\` (to prove, E1) | **Yes**: `ShapeItem.Delete()`, `Brep.Delete()` | **Recommended** |
| M2 | A `.tsc` file produced by us, then `ImportShapeItems(folder)` | Possible, needs Tekla's fingerprint and compressed geometry | Same | Yes | Fallback only (E8) |
| M3 | An IFC/STEP/DXF of the solid, then `ImportShapeItemsFromGeometryFiles(folder)` (name = file name) | Fine for a plain solid; Tekla re-meshes it | Same | Yes | Fallback only (E8), and the only route that needs no brep builder |
| M4 | Writing the two XML files ourselves | n/a | n/a | n/a | **Rejected**: bypasses Tekla's fingerprint and database; a wrong byte makes a model's catalogue inconsistent |

## 4. Placing the Item

- The Open API's Item is `Tekla.Structures.Model.Brep : Part : ModelObject`, with `StartPoint`, `EndPoint`, `StartPointOffset`, `EndPointOffset`, `Profile`, `Material`, `Name`, `Class`, `Finish`, `Position` (plane, depth, rotation and their offsets), `Insert/Select/Modify/Delete`, `GetSolid`, `GetCoordinateSystem`, `GetReportProperty`. The class documentation (`Brep`) and the `ShapeItem` example both use `Profile.ProfileString` as the shape name.
- A project of the owner's own (ScaffPlanDesigner) already inserts custom shapes this way, which is the strongest sign the route works in current releases (not run by us).
- **Convention the builder uses:** the shape is built with its section in the Y-Z plane and its length along +X from 0 to L, `UpAxis = Y_Axis`, the section coordinates already containing the section's reference-point offset. Tekla's own advice is exactly that orientation.
- **To prove (E3):** where the shape's origin sits relative to `StartPoint`; whether `EndPoint` only sets the direction; what `Position.Rotation` / `RotationOffset` / `Plane` / `Depth` do to a Brep Item (they are defined for beams; for an Item they may be ignored or applied about the start-to-end axis); and the rule that maps a FloLess member frame (start, end, roll) to `(StartPoint, EndPoint, Rotation, RotationOffset)`. `item-place.csx` measures this without assuming Tekla's convention: it reads Tekla's own coordinate system for the Item, moves the catalogue shape's vertices into the world with it, and requires the solid Tekla reports to coincide with them, then separately reports whether the axes agree with the requested direction and `expect_axis_y`.

## 5. The solid the scripts build

`shape-create.csx` builds the brep from `args.solid = { kind: "prism", outer: [[y, z], …], holes: [[[y, z], …], …], length }`: a section swept along X (two caps, one quad per ring edge). The outer ring is made counter-clockwise and hole rings clockwise (seen from +X); the x = 0 cap runs clockwise and the x = L cap counter-clockwise so both face outwards; each cap's inner loops wind either "opposite" the outer loop (the default) or "same" (`args.inner_winding`), so E2 can let Tekla decide. The same winding and volume arithmetic was checked outside Tekla while writing this plan (a 100 x 100 x 6 hollow section, 1000 long, and an asymmetric L 75 x 100 x 8: signed face-sum volumes equal area x length exactly, and the wrong cap winding is detectably wrong), so a failure in E2 will be Tekla's convention, not a typo in the builder. The production input is "a neutral solid"; a prism is the member case, and any other solid needs its own builder.

## 6. Where the write lands, and the decision rule (the key question, like A2 §11.4)

Hypotheses, to be settled by E0 (paths) and E1/E2 (observed byte changes in the snapshot diff), never by assumption:

| | Hypothesis | If true |
|---|---|---|
| H1 | The write goes only to the **open model folder**: one new `Shapes\<guid>.xml` and one new `ShapeGeometries\<guid>.xml` per shape, plus the model database for the Item. | **Found as the working assumption** (every model here carries these two folders; `ImportShapeItems` says "imported … to model"; ScaffPlan's `.tsc` templates sit in an *environment* folder and are imported into the model when used). The destination identity is the model; the consent card says "this model only". |
| H2 | It also writes something in the environment or firm folders (`XS_PROFDB`, `XS_FIRM`, `XS_PROJECT`). | The snapshot diff of those folders shows it. Stop, restore, ask the master; the consent card would then have to say it is shared. |
| H3 | In a **shared (multi-user) model** the shapes live on the server and every user sees them. | E-list item for a shared scratch model if one can be had; until then the command treats `ModelInfo.SharedModel == true` as "affects everyone on this model", says so on the consent card and asks. |

**If any experiment's snapshot diff shows a file changed outside the scratch root (the model folder and the scratch environment copy), stop at once, restore, and ask the master before another write.** The real folders are proved untouched at the end of every run (§7.4).

## 7. Isolation, backup, restore (A2's rules, adapted, plus two new run rules)

The shape catalogue is inside the model, so a **throwaway model inside `C:\A5SPIKE\` isolates the write**. The profile catalogue is not written by A5, but the same isolation is kept so a surprise cannot reach it.

### 7.1 Setup, in this order

1. Create `C:\A5SPIKE\` (outside `C:\TeklaStructuresModels`, so no model there is touched). Copy the environment's `Profil` folder (the value of `XS_PROFDB`) and the shared `common\profil` folder (the second `XS_INP` entry A2 found) to `C:\A5SPIKE\env\` as A2 did; this is only so a stray profile write cannot reach the real ones.
2. Start Tekla through the `floless-app-tekla-launch` skill with a scratch `-i` ini whose `XS_PROFDB` and `XS_INP` point at the scratch copies (A2's bypass ini: `XS_DEFAULT_LICENSE=PARTNER`, documented in Tekla's own `Bypass.ini`; the environment ini is a copy of the default one with only those two paths changed). The spike scripts never launch Tekla.
3. **Run rule A (new): a FRESH scratch model every run.** Create `C:\A5SPIKE\models\A5SPIKE-<yyyymmdd>-<hhmm>` from a pristine template (a model that was created and closed through Tekla's own exit and never opened since). Never reuse a model from an earlier run, never open one that Tekla did not close cleanly: A2 lost a night to Tekla's "Open … not properly closed" notice, which needs a click that this tooling cannot make. All scripts refuse a model whose folder name does not start with `A5SPIKE` and, for writes, whose folder is not under `C:\A5SPIKE\`.
4. **E0 first** (no write): it **proves isolation**: `advanced_options.XS_PROFDB.value` is under `C:\A5SPIKE\`, the model's `Shapes\` and `ShapeGeometries\` are empty, the planned `AWI*` names are free. Every write script re-checks the model path and `XS_PROFDB` and refuses otherwise.

### 7.2 Backup before every write (`snapshot-shapes.ps1`)

- `snapshot-shapes.ps1 -Mode Snapshot -Folders <scratch model>,<scratch Profil>,<scratch common-profil> -BackupDir C:\A5SPIKE\backup-<n> -Manifest C:\A5SPIKE\backup-<n>.json` hashes (SHA-256) and copies `profdb.bin`, `profitab.inp`, `profile_properties.ail`, `*.clb`, `*.uel`, `*.lis` at each folder's top level and **every file under the model's `Shapes\` and `ShapeGeometries\`**. A fresh model has none, and an empty snapshot is valid: the manifest still carries its time (`backup_id`).
- `run-spike.ps1 -Script shape-create -Write -BackupManifest …` passes the manifest's hashes and id to `shape-create.csx`, which **re-hashes every file now in the model's `Shapes\` and `ShapeGeometries\`, refuses if any is missing from the manifest or differs**, so a write can only follow a backup of exactly that state.

### 7.3 Undo, and proof by hash

- API undo exists for this spike: `shape-create.csx -action delete` (after the Item is deleted, `GetInstanceCount() == 0`) and `Brep.Delete()`. Use it, check `Select == false`, and still prove the files by hash.
- For the files: **close Tekla through its own exit** (**run rule B (new)**: never by ending its process, which leaves the model's lock and produces the "not properly closed" notice next time; the live-session owner answers Tekla's Save prompt), then `snapshot-shapes.ps1 -Mode Restore -Manifest …` (puts the backed-up bytes back, removes files added since, verifies) and require the last line `VERIFIED: N files are byte-identical (SHA-256) to the snapshot`. For a fresh scratch model the restore is also simply deleting the model folder; the **real-folder proof below is what matters**.
- **A restore must succeed after every write experiment**, before the next one starts. A mismatch ends the spike. The script's three paths (added, modified, removed files) were exercised on a temporary folder while writing this plan, including the empty-snapshot case: all detected and restored, exit 0.

### 7.4 Real folders proven untouched (every run)

Before the first Tekla action, `snapshot-shapes.ps1 -Mode Snapshot` the real catalogues read-only to `C:\A5SPIKE\real-*.json` (no copies needed: hash only; use a throwaway `-BackupDir`): the default environment `Profil`, the UK `profil` folder (and the EU one if it exists), the shared `common\profil`, and the `Shapes\` and `ShapeGeometries\` of every model under `C:\TeklaStructuresModels\` (at least "FloLess UK", "FloLess EU", "FloLess US", "FloLess", "Aware Tests"). After the last Tekla exit run `-Mode Verify` against each and report the exact `VERIFIED` lines, as A2 §9 did. Any `MODIFIED`/`ADDED`/`REMOVED` is reported to the master at once.

### 7.5 How the restore is reported

As A2 §4.5: each experiment ends with one block in the spike log (§12) and the #859 thread: experiment id; files changed (path, hash before, hash after); the exact `VERIFIED`/`NOT IDENTICAL` output; whether Tekla was closed through its own exit for the restore; when not identical, the full difference list and no further write.

## 8. The experiments, in order

Shared rules: fresh model and scratch environment (§7.1); the `A5SPIKE` and `AWI` guards; nothing runs against "FloLess UK", "FloLess EU" or any existing model; only the live-session owner launches or closes Tekla; no keystrokes to desktop apps from this tooling. Scripts are in `cli-tekla/spike/item-create/`; run with `run-spike.ps1`. Coordinates are millimetres whatever the environment's display units (the Open API works in model units).

| # | Experiment | Writes? | What it proves | Pass | Fail / stop |
|---|---|---|---|---|---|
| **E0** | `probe.csx`: model path, `XS_*` options, files in `Shapes\` and `ShapeGeometries\` (size, SHA-256), shapes Tekla lists, planned-name existence, count of Items. | No | Isolation holds; the fresh model has no shapes; names are free. | `XS_PROFDB` under `C:\A5SPIKE\`; shape folders empty; model is `A5SPIKE*`. | `XS_PROFDB` still real and no master's OK: do not write, report. |
| **E1** | `shape-create -action insert` of a plain rectangular prism 200 x 100, length 1000, name `AWI-E1-RECT`, digest set. Snapshot first (§7.2), diff after. | **Yes (model shape catalogue)** | The mechanism works at all; **where the write lands (§6)**; which files appear. | `insert` returns true; `Select` true; `extrema` equals `[0,-50,-50,1000,50,50]` and every vertex within 1e-3 mm of the sent one (float32 storage, §3.3); `is_solid`; **exactly one new file in `Shapes\` and one in `ShapeGeometries\`, nothing changed anywhere else under the scratch root, and the real-folder proof (§7.4) still holds after Tekla's exit**. | A file changed outside the model folder: stop, restore, ask the master. Insert false or throws: record the exact exception text and the `polymesh_problems`. |
| **E2** | Harder solids: a hollow section (outer 100 x 100, hole 88 x 88, length 1000) with both `inner_winding` values, an asymmetric L 75 x 100 x 8, a section with two holes, and each `UpAxis` (`Y_Axis`, `Z_Axis`) on one shape; also `Polymesh.Validate` with the extra flags (512, 1024, 2048, 4096, 8192). | Yes | Whether holes survive as inner loops of the caps; which winding Tekla accepts (`polymesh_valid`, `is_solid`); whether `UpAxis` changes the stored extrema; what the validator says about a deliberately bad brep (open shell, self-intersecting ring, zero-thickness). | The accepted winding is identical for every hollow shape; vertex and face counts read back equal the sent ones (or the difference is explained); a bad brep is **refused with a reason**, and the catalogue is unchanged (no file added). | A bad brep accepted: record, and the command must run `Polymesh.Validate` itself before every insert. |
| **E3** | `item-place` of the E1 and E2 shapes: along +X from the origin, along +Y, vertical, and a skewed axis; for each, the four `Position.Rotation` values, `RotationOffset` 0/90/45, and the default plane and depth. | **Yes (one Item per case in the scratch model)** | The placement rule: origin of the shape vs `StartPoint`; what `EndPoint` does; what rotation does to a Brep Item; the UDA names an Item accepts. | For every case: `origin_to_start_mm` ≤ 1e-3, `axis_x_dot_start_to_end` ≥ 1 − 1e-9, `max_solid_vertex_to_moved_shape_mm` and `max_moved_shape_vertex_to_solid_mm` ≤ 1e-2, solid box and `volume_from_faces` equal to the prism's (box within 1e-2 mm, volume ratio within 1e-5), `instance_count_after` grows by one. A rotation convention is **named and fits all cases** (`axis_y_dot_expected` ≥ 1 − 1e-9). | A convention that depends on the member's direction in a way that cannot be expressed: report it, and the command refuses members that do not satisfy it with a choice (§9). |
| **E4** | One shape (length 1000) placed with start-to-end distance 500, 1000 and 2000. | Yes | **Does Tekla stretch the shape to the Item's length?** (owner question 1) | Either `solid_extent_along_axis_x` equals the distance every time (stretches; the section does not scale) **or** it stays 1000 every time (fixed). Whichever it is, record `volume_from_faces` and whether the section changed. | A partial stretch (section scales too): the Item route is unusable for lengths other than the shape's own; report. |
| **E5** | Reuse and conflict behaviour: `insert` the E1 geometry again under a second name; `insert-or-match` the same; `insert-allow-duplicates`; `insert-same-name` (same name, different geometry) after placing an Item on the first; `rename`; `delete` of a shape that has an Item. | Yes | What Tekla does on duplicate geometry, on same name with different geometry (silent overwrite? refusal?), and on deleting a shape in use; what happens to existing Items when a shape of the same name changes. | The behaviour is recorded and each case is distinguishable by read-back (`returned`, `matching_guids`, `files_*`, instance counts). The reuse rule (§9 step 6) is then fixed. | Same-name insert **silently overwrites and changes the placed Item**: confirms the hazard; the command never uses an existing name for different geometry (digest in the name) and never calls `Modify`. |
| **E6** | Name behaviour: spaces, `#`, `/`, `:`, `x`, a long name of 1023 and of 1024 characters, a leading space, a different case of an existing name, a non-ASCII letter. | Yes | The deterministic-name rule (digest in the name) and which characters are safe. | A name form that holds for "same digest, same name" and "different digest, never the same name", exact length limit confirmed. | No safe form: the command names shapes `AWI-<hex digest>` only and puts the readable label in metadata. |
| **E7** | Persistence: `SetMetadata("aware.digest", …)` then `GetMetadata`; place an Item; **close Tekla through its own exit**, reopen the same scratch model, `readback.csx`. | Yes | The digest, the shape, the Item and its instance count survive a save and reload, with no "not properly closed" notice. | `readback` lists the shape (same guid, geometry guid, fingerprint, metadata) and the Item (same geometry, volume, coordinate system) after reopening. | Metadata lost: the digest has to live in the name (and a UDA) instead. |
| **E8** | Fallbacks: `export-tsc` of the E1 shape to an explicit folder, `delete` it, `import-tsc` that folder; and `import-geometry` of an IFC of the same prism named `AWI-E8-IFC.ifc` (FloLess's own IFC writer produces it). | Yes | Whether a file route is a real fallback; what the `.tsc` looks like when Tekla writes it (compare §3.4); what an IFC import does to name, extrema and volume. | Import returns true, the shape reappears with the same extrema (within 1e-3 mm) and, for the IFC, volume within 1e-4. | Either fails: M1 is the only route; record the exception text. |
| **E9** | Failure behaviours: insert refused for bad geometry (E2), an import folder with a good and a corrupt file (which shapes exist afterwards), delete of an in-use shape, an Item insert that returns false (shape missing), and Tekla refusing a duplicate. **Tekla closed mid-way is not tested.** | Yes | What "partial failure" looks like, so the command can report exactly what exists afterwards. | Each outcome is distinguishable by read-back of shapes, Items and the file diff. | An outcome that cannot be told from success: the command inserts one shape per step and verifies each. |
| **E10** | Final: close Tekla through its own exit; `snapshot-shapes.ps1 -Mode Verify` on every real baseline (§7.4); delete `C:\A5SPIKE\models\<model>`. | Restores | The real catalogues and models are byte-identical to before the spike. | `VERIFIED: N files are byte-identical` for each baseline. | Not identical: report the list to the master; no further action in that environment. |

**Order and gates.** E0 first. E1 only after E0 proves isolation. E2 and E3 after E1. E4 and E5 need E3's Item. E7 and E8 last in a window because they close Tekla. Each write experiment ends with its diff reported; a restore (or a new fresh model) before the next window.

**Needs from a live Tekla:** about 3 hours of exclusive use of one Tekla 2026 instance (exactly one reachable instance, as the bridge requires), a fresh scratch model per window, the scratch environment copy, Tekla closed through its own exit at least twice (E7, E10), and the live-session owner for launching, model creation and the Save prompt. No admin rights are expected. A 2025 repeat of E1, E3 and E7 is another half hour if 2025 must be supported.

## 9. The later destination-bound command (step 2): `aware-tekla item-create`

Not built here. The spike decides the placement rule, the length rule and the name rule; the rest stands.

```powershell
'{"members":[{"id":"M12","definition_digest":"sha256:…","sizes":{"h":300,"b":150},
   "solid":{"kind":"prism","outer":[[…]],"holes":[],"length":2400.0},
   "placement":{"start":[…],"end":[…],"roll_deg":0},"material":"S355JR","label":"…"}],
  "target":{"model_path":"C:\\…","model_guid":"…","shared_model":false,"tekla_version":"2026.0"},
  "mode":"plan","consent":null}' | aware-tekla item-create --json-stdin
```

**Input.** One call takes one or more members. Each member carries the neutral solid (the member's evaluated section, outer plus holes and reference point, swept along its length), the caller's definition digest plus sizes, the placement as FloLess has it, and what Tekla needs for an Item (material, label). `target` is what the person saw when they chose "send as an Item". `mode` is `plan` (no write, reports what it would create and reuse) or `create`; `create` needs `consent: {"source": "ui-click" | "chat-proposal-card"}`. **The verb is never reached from a bake, and an unattended run never creates Items** (owner ruling, epic §5.3): the run takes the safe default (leave the member out, labelled, question batched) before this verb is ever called. That is the owner's carve-out, not a new hard stop.

**Checks before any write, in this order; each refusal named, exit 2 before Tekla is touched where it can be:**
1. Solid validity (finite, closed, non-self-intersecting rings, positive length) → `invalid-solid`.
2. The caller's digest is recomputed by AWARE over the canonical bytes of the solid and sizes and compared → else `digest-mismatch`.
3. Exactly one reachable Tekla and an open model, as every verb.
4. **Target identity re-check:** the open model's path, model GUID, shared flag and Tekla version equal `target` → else `target-changed` (nothing written).
5. **Write level:** the model folder only (H1), reported as `write_level: "model"`; if the model is shared, `shared_model: true` is reported and the consent card has already said it affects everyone on the model.
6. **Existing shape under the name** (the name is derived from the digest, E6): select it. If its `aware.digest` metadata, extrema and read-back vertices equal this solid's → `reused-identical` (nothing written, success). If it exists with a different digest or geometry → `conflict` (nothing written, both sides reported; **never overwritten silently**, because an import or a same-name insert might). If the geometry already exists under another name (`InsertOrGetGuidsOfShapesWithMatchingGeometry`) → the existing guid is reported and `reused-by-geometry` is offered as a choice, never taken silently.
7. **Pre-write snapshot:** SHA-256 of the files in the model's `Shapes\` and `ShapeGeometries\`, copied to `%LOCALAPPDATA%\aware\tekla-item-create\<model key>\<run id>\backup\`; refuse if a copy does not verify (`backup-failed`).
8. `Polymesh.Validate` with every flag, run by the command before `Insert()`.

**Write.** One shape per distinct digest, then one Item per member; each step verified before the next. Every created shape and Item is recorded in a per-run ledger as it is created (name, guid, geometry guid, file hashes before and after).

**Verified receipt.** After each Item: Tekla's solid bounds (world, within 1e-2 mm of the input after the placement rule), the volume computed from Tekla's faces (relative 1e-5 of the input's, after allowing for 32-bit storage), the coordinate system against the requested start and direction, the shape's `GetInstanceCount()`. Any deviation gives `read-back-mismatch` with both sides, and the Item stays (the person decides; the receipt never claims a match it does not have).

**Honest labelling.** The Item's name, a UDA (`AWARE_SENT_AS_ITEM = 1` plus the digest and the FloLess custom-section id; the UDA names are confirmed in E3) and the receipt all say "sent to Tekla as an Item, not as a profile": it is a fixed solid of one size and one length, with no profile name.

**Output (`result`)**, always the same keys:
```json
{ "status": "created | reused-identical | conflict | target-changed | digest-mismatch | invalid-solid | backup-failed | read-back-mismatch | partial | failed",
  "target": { "model_path": "…", "model_guid": "…", "shared_model": false, "tekla_version": "2026.0", "write_level": "model" },
  "shapes":  [ { "name": "…", "digest": "sha256:…", "status": "created | reused-identical | conflict", "guid": "…", "geometry_guid": "…", "files": [ { "path": "…", "sha256_after": "…" } ] } ],
  "items":   [ { "member_id": "M12", "item_guid": "…", "shape": "…", "verification": { "box_max_deviation_mm": 0, "volume_ratio": 1, "origin_to_start_mm": 0, "axis_dot": 1 } } ],
  "exists_afterwards": [ "…every shape and item Tekla reports for this call, by read-back, not by intention…" ],
  "undo": "Delete the items listed, then each shape whose instance count is 0; or restore the files from `backup` with Tekla closed." }
```

**Partial failure.** Any exception after the first created object sets `status: partial` and lists `exists_afterwards` from a fresh read-back plus the ledger, with the backup path; the command never retries, and **never deletes what it cannot prove it made** (it can prove it, by the ledger's guids, and offers the undo as an explicit second call, never as an automatic rollback).

**Guard rails in the manifest:** `mode: write`, a `safety:` block saying it adds shapes to the model's shape catalogue and Items to the model (and that a shared model's users see them), declared `idempotent` only for the `reused-identical` path; exit codes follow `profile-project` (0 ok, 2 refused before Tekla, 4 host-unreachable).

## 10. Open questions

**For the live Tekla** (each is an experiment above):
1. Does the insert land only in the model folder, and which files exactly (E1)? Does the model database change too?
2. Which cap inner-loop winding does Tekla accept, and do holes survive (E2)?
3. Does `UpAxis` change the stored shape, and does the native "X along, Y up" orientation matter (E2)?
4. Where is a shape's origin relative to the Item's `StartPoint`, and does `EndPoint` stretch the shape (E3, E4)?
5. What do `Position.Rotation`, `RotationOffset`, `Plane` and `Depth` do to a Brep Item (E3)?
6. Which name characters and lengths are accepted, and is the name case-sensitive (E6)?
7. Does Tekla refuse, merge or silently overwrite on same name with other geometry and on duplicate geometry under another name (E5)?
8. Do the metadata and the Item survive a save and reload, and does closing through Tekla's own exit avoid the lock notice (E7)?
9. Is an Item's `GetReportProperty("VOLUME")` available, or is the face-sum volume the only one (E3)?
10. What does a shared (multi-user) model do with shapes (needs a shared scratch model; not planned, so the command asks)?
11. Is the 2025 behaviour the same (a short repeat)?
12. Can the Item carry UDAs and a material that FloLess maps, without the Item's own defaults overriding them (E3)?

**For the owner, in plain English, with a recommendation:**
1. **Members of the same section but different lengths.** If Tekla will not stretch a shape (E4 answers this), each distinct length has to be its own shape in the user's model, so a model with 200 differently cut beams would add up to 200 shapes. Is that acceptable if the consent screen says plainly how many shapes will be added ("this adds 14 shapes to your model's shape catalogue")? **Recommendation: yes, with the count on the consent screen**, because the issue's own rule ("reuse an identical shape") still saves anything cut to the same length, and the alternative (refusing) is the kind of hard stop the rules avoid. If Tekla does stretch a shape cleanly, there is nothing to ask.
2. **Shared (multi-user) Tekla models.** Adding shapes there changes the catalogue everyone on the model sees. **Recommendation: allowed, but the consent card names the model as shared and says it affects everyone on it**, and the choice is the person's, like every other choice in this epic.

## 11. Files on this branch, and the compile record

- `cli-tekla/spike/item-create/probe.csx` (E0), `readback.csx` (E7): read-only.
- `cli-tekla/spike/item-create/shape-create.csx` (E1, E2, E5-E9): writes the model's shape catalogue; five guards.
- `cli-tekla/spike/item-create/item-place.csx` (E3, E4): writes one Item; reads the result back.
- `cli-tekla/spike/item-create/run-spike.ps1`: wraps a script into an `exec` request; the writing scripts need `-Write`, and `shape-create` a backup manifest.
- `cli-tekla/spike/item-create/snapshot-shapes.ps1`: `Snapshot`, `Verify`, `Restore` with SHA-256 proof, adapted from A2 for `Shapes\` and `ShapeGeometries\`.
- `cli-tekla/Tests/ItemCreateSpikeTests.cs`: 17 tests that parse the scripts and pin every guard on the source, in the order the guards must run.
- The scripts add no verb, no manifest entry and no parity change.

**Compile record (2026-11-19, Tekla not started).** All four `.csx` scripts were compiled with Roslyn against the Tekla 2026.0 managed assemblies (`Tekla.Structures.*.dll` from `Net48Runtime` then `bin`, 40 references, file version 2026.0.60107.0) with the bridge's own script options (`Program.CreateScriptOptions`, `ExecGlobals` as globals), as metadata references only: no Tekla assembly was loaded for execution and no Tekla process was touched. Method: a throwaway net48 console project (kept out of the repo) that references `cli-tekla.csproj`, calls the internal `ResolveTeklaReferences` and `CreateScriptOptions` by reflection and prints the diagnostics of `CSharpScript.Create(...).Compile()`. The first compile found four kinds of real mistake (all fixed): the non-public `MaxShapeNameLength` constant, `Polymesh.Validate` taking `ref` rather than `out`, a non-public `Insert(bool)` overload, and a missing `using Tekla.Structures.Solid;` for `Face` and `Loop` in the two read-back scripts. Final result: **probe.csx, readback.csx, shape-create.csx and item-place.csx: 0 errors, 0 warnings.** A compile cannot prove behaviour; that is E0-E10.

**Tests (2026-11-19).** `dotnet test --filter ItemCreateSpikeTests`: 17 passed, 0 failed. Mutation check: removing the same-name guard from `shape-create.csx` and the work-plane restore from `item-place.csx` turned two tests red; both files were restored byte for byte. Only this test class was run.

## 12. Spike log

2026-11-19: **plan, scripts and tests written; not started on a live Tekla.** Tekla was not started, attached to or driven, and no Tekla process was queried or touched. Read-only inspection only: the 2026.0 install's XML documentation and assembly metadata, the shape-catalogue files of the existing model "Aware Tests" and a sample `.tsc` from the owner's ScaffPlanDesigner project (decompressed in memory, not modified), and the A2 worktree. No file under `C:\TeklaStructuresModels`, `C:\ProgramData\Trimble` or the A2 worktree was changed. Awaiting Tekla time and the master's decision on the two owner questions above.
