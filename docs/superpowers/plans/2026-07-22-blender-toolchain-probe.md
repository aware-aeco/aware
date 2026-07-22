# Blender Toolchain Probe — Verified Findings

**Task:** [Task 1 of the Blender visualization agent implementation plan](./2026-07-22-blender-visualization-agent.md#task-1-prove-the-toolchain) ([design doc](../specs/2026-07-22-blender-visualization-agent-design.md)).

**Machine:** this workstation (Windows 11 Pro 10.0.26200), probed 2026-07-22.

**Method:** every command below was actually executed via `blender -b --python-expr "..."` (or `--python-expr` piped through the `aware` CLI for the fixture). Nothing here is inferred, paraphrased, or copied from docs — each block quotes the real stdout. Where a result differed from what the plan doc expected, the actual output is recorded as observed, with the investigation that explains it.

No product code shipped from this task. This document is the record; later tasks quote it instead of re-probing.

---

## Quick reference for later tasks

Substitute these directly into Tasks 4, 6, 8, 12 (see plan doc cross-references in each task):

| Symbol | Value | Source |
|---|---|---|
| `BLENDER` | `C:\Program Files\Blender Foundation\Blender 5.2\blender.exe` | already verified (plan doc "Verified starting state") |
| Bundled Python | `C:\Program Files\Blender Foundation\Blender 5.2\5.2\python\bin\python.exe`, **3.13.13** | Q1 |
| `ifcopenshell` version | **0.8.5** | Q2 |
| Settings API generation | **string-keys** — `settings.set("use-world-coords", True)` | Q3 |
| `UNIT_SCALE` | **1.0** — ifcopenshell.geom already returns metres for a millimetre-unit IFC file | Q4 |
| `EEVEE_ENGINE` | **`BLENDER_EEVEE`** (not `_NEXT`) | Q5 |
| Material association shape | Plain `IfcMaterial` directly off `RelatingMaterial`, no layer-set/profile-set wrapping, for `ifc.write`-generated files | Q6 |

**One correction to the design doc's assumption, flagged for whoever executes Task 4** (see "Additional finding 1" below): a bare `pip install ifcopenshell` inside Blender's Python is **not sufficient** to make it importable on this machine. `_import_ifcopenshell()` as drafted in the plan doc (a bare `import ifcopenshell`) will raise `ImportError` unless the caller first puts the pip `--user` install location on `sys.path`. The working fix is quoted in full below.

---

## Q1 — Blender's bundled Python

```
"$BLENDER" -b --python-expr "import sys; print('PYEXE', sys.executable); print('PYVER', sys.version)"
```

Verbatim output:

```
PYEXE C:\Program Files\Blender Foundation\Blender 5.2\5.2\python\bin\python.exe
PYVER 3.13.13 (main, May  8 2026, 12:37:03) [MSC v.1944 64 bit (AMD64)]
Blender 5.2.0 LTS (hash fbe6228777e7 built 2026-07-14 01:35:40)
```

**Answer:** Blender 5.2 bundles **Python 3.13.13** at `C:\Program Files\Blender Foundation\Blender 5.2\5.2\python\bin\python.exe`.

---

## Q2 — Install `ifcopenshell` into that Python

```
"$BLENDER" -b --python-expr "import sys,subprocess; subprocess.check_call([sys.executable,'-m','pip','install','--upgrade','pip']); subprocess.check_call([sys.executable,'-m','pip','install','ifcopenshell'])"
```

Verbatim output, full — the pip self-upgrade ran first (per the command), then the ifcopenshell install:

```
Defaulting to user installation because normal site-packages is not writeable
Requirement already satisfied: pip in C:\Program Files\Blender Foundation\Blender 5.2\5.2\python\Lib\site-packages (26.0.1)
Collecting pip
  Downloading pip-26.1.2-py3-none-any.whl.metadata (4.6 kB)
Downloading pip-26.1.2-py3-none-any.whl (1.8 MB)
   ---------------------------------------- 1.8/1.8 MB 5.1 MB/s  0:00:00
Installing collected packages: pip
  WARNING: The scripts pip.exe, pip3.13.exe and pip3.exe are installed in 'C:\Users\bimst\AppData\Roaming\Python\Python313\Scripts' which is not on PATH.
  Consider adding this directory to PATH or, if you prefer to suppress this warning, use --no-warn-script-location.
Successfully installed pip-26.1.2

[notice] A new release of pip is available: 26.0.1 -> 26.1.2
[notice] To update, run: C:\Program Files\Blender Foundation\Blender 5.2\5.2\python\bin\python.exe -m pip install --upgrade pip
Defaulting to user installation because normal site-packages is not writeable
Collecting ifcopenshell
  Using cached ifcopenshell-0.8.5-py313-none-win_amd64.whl.metadata (13 kB)
Collecting shapely (from ifcopenshell)
  Using cached shapely-2.1.2-cp313-cp313-win_amd64.whl.metadata (7.1 kB)
Requirement already satisfied: numpy in C:\Program Files\Blender Foundation\Blender 5.2\5.2\python\Lib\site-packages (from ifcopenshell) (2.3.4)
Collecting isodate (from ifcopenshell)
  Using cached isodate-0.7.2-py3-none-any.whl.metadata (11 kB)
Collecting python-dateutil (from ifcopenshell)
  Using cached python_dateutil-2.9.0.post0-py2.py3-none-any.whl.metadata (8.4 kB)
Collecting lark (from ifcopenshell)
  Using cached lark-1.3.1-py3-none-any.whl.metadata (1.8 kB)
Requirement already satisfied: typing-extensions in C:\Program Files\Blender Foundation\Blender 5.2\5.2\python\Lib\site-packages (from ifcopenshell) (4.14.1)
Collecting six>=1.5 (from python-dateutil->ifcopenshell)
  Using cached six-1.17.0-py2.py3-none-any.whl.metadata (1.7 kB)
Using cached ifcopenshell-0.8.5-py313-none-win_amd64.whl (24.5 MB)
Using cached isodate-0.7.2-py3-none-any.whl (22 kB)
Using cached lark-1.3.1-py3-none-any.whl (113 kB)
Using cached python_dateutil-2.9.0.post0-py2.py3-none-any.whl (229 kB)
Using cached six-1.17.0-py2.py3-none-any.whl (11 kB)
Using cached shapely-2.1.2-cp313-cp313-win_amd64.whl (1.7 MB)
Installing collected packages: six, shapely, lark, isodate, python-dateutil, ifcopenshell

Successfully installed ifcopenshell-0.8.5 isodate-0.7.2 lark-1.3.1 python-dateutil-2.9.0.post0 shapely-2.1.2 six-1.17.0
```

**Answer:** A wheel exists for Python 3.13 / win_amd64 and installed cleanly — `ifcopenshell-0.8.5` plus `isodate-0.7.2`, `lark-1.3.1`, `python-dateutil-2.9.0.post0`, `shapely-2.1.2`, `six-1.17.0`. **No fork triggered** — the "no wheel" escalation path in the task instructions did not apply.

However: note the first line, `Defaulting to user installation because normal site-packages is not writeable`. Blender's own bundled `site-packages` (`...\5.2\python\Lib\site-packages`) sits under `Program Files` and isn't writable without elevation, so pip silently fell back to a `--user` install at `C:\Users\bimst\AppData\Roaming\Python\Python313\site-packages`. **This has a consequence — see "Additional finding 1" below**, which is not one of the six numbered questions but is load-bearing for Task 4.

---

## Q3 — The geom settings API generation

First attempt, run immediately after Q2 with a fresh `--python-expr` (i.e. simulating what Task 4's script would do with a bare `import ifcopenshell`):

```
"$BLENDER" -b --python-expr "
import ifcopenshell, ifcopenshell.geom
print('IFCOS_VERSION', ifcopenshell.version)
..."
```

Verbatim output — **this failed**, and the failure itself is Additional Finding 1:

```
Traceback (most recent call last):
  File "<string>", line 2, in <module>
ModuleNotFoundError: No module named 'ifcopenshell'
Blender 5.2.0 LTS (hash fbe6228777e7 built 2026-07-14 01:35:40)
```

Diagnosed (see Additional Finding 1) and re-run with the fix (`sys.path.insert(0, site.getusersitepackages())` before import):

```
"$BLENDER" -b --python-expr "
import sys, site
sys.path.insert(0, site.getusersitepackages())
import ifcopenshell, ifcopenshell.geom
print('IFCOS_VERSION', ifcopenshell.version)
s = ifcopenshell.geom.settings()
try:
    s.set('use-world-coords', True); print('API_GENERATION string-keys')
except Exception as e:
    print('string-keys failed:', e)
    try:
        s.set(s.USE_WORLD_COORDS, True); print('API_GENERATION enum-attrs')
    except Exception as e2:
        print('enum-attrs failed:', e2)
"
```

Verbatim output:

```
IFCOS_VERSION 0.8.5
API_GENERATION string-keys
Blender 5.2.0 LTS (hash fbe6228777e7 built 2026-07-14 01:35:40)
```

**Answer:** `ifcopenshell.version` = **0.8.5**. API generation is **string-keys** (`settings.set("use-world-coords", True)` succeeds; the enum-attrs fallback was never exercised). This matches the plan doc's "0.8+" branch.

---

## Q4 — Geometry unit scale (the important one)

### Fixture generation

Scene JSON used (exactly as specified in the task — a minimal 2-element probe, distinct from Task 2's 9-element committed fixture), written to the session scratchpad as `probe-scene-args.json`:

```json
{
  "scene": {
    "meta": { "name": "Probe", "units": "mm", "up": "z" },
    "groups": [{ "key": "W16X26", "label": "Beams", "color": "#60a5fa", "opacity": 1 }],
    "elements": [
      { "id": "B1", "group": "W16X26", "role": "beam", "material": "A992",
        "from": [0,0,3000], "to": [6000,0,3000], "section": { "w": 140, "d": 400 },
        "meta": { "profile": "W16X26" },
        "xsection": { "shape": "i", "d": 400, "bf": 140, "tw": 6, "tf": 9 } },
      { "id": "C1", "group": "W16X26", "role": "column", "material": "A500-GR.B",
        "from": [0,0,0], "to": [0,0,3000], "section": { "w": 152, "d": 152 },
        "meta": { "profile": "HSS6X6X3/8" },
        "xsection": { "shape": "rhs", "d": 152, "b": 152, "t": 9 } }
    ]
  },
  "output-path": "<scratchpad>/probe.ifc"
}
```

```
./cli/target/debug/aware.exe agent invoke ifc write --inputs "@probe-scene-args.json"
```

Output (`ok: true`, 2 members emitted — `B1` as `IfcBeam`, `C1` as `IfcColumn`; full JSON receipt omitted here for brevity, all fields nominal, no `failed`/`unsupported`/`warnings`).

The generated `probe.ifc` declares `#9=IFCSIUNIT(*,.LENGTHUNIT.,.MILLI.,.METRE.)` and stores raw millimetre magnitudes verbatim — e.g. `#34=IFCEXTRUDEDAREASOLID(#33,#8,#2,6000.0)` for the beam's 6000 mm extrusion depth, and `#44=IFCRECTANGLEHOLLOWPROFILEDEF(.AREA.,'HSS6X6X3/8',#7,152.0,152.0,9.0,$,$)` for the column's 152×152 mm section centred on its own placement axis.

### Measurement

```
"$BLENDER" -b --python-expr "
import sys, site
sys.path.insert(0, site.getusersitepackages())
import ifcopenshell, ifcopenshell.geom
f = ifcopenshell.open(r'<probe.ifc>')
s = ifcopenshell.geom.settings()
try: s.set('use-world-coords', True)
except Exception: s.set(s.USE_WORLD_COORDS, True)
it = ifcopenshell.geom.iterator(s, f)
assert it.initialize()
xs = []
while True:
    v = it.get().geometry.verts
    xs += list(v[0::3])
    if not it.next(): break
print('X_SPAN', max(xs) - min(xs))
print('X_MIN', min(xs))
print('X_MAX', max(xs))
"
```

Verbatim output:

```
X_SPAN 6.076
X_MIN -0.076
X_MAX 6.0
```

**This is neither of the two numbers the task anticipated (`6.0` or `6000.0`).** Investigated rather than guessed — reran with a per-product breakdown instead of the pooled min/max:

```
"$BLENDER" -b --python-expr "
... (same setup) ...
while True:
    shape = it.get()
    v = shape.geometry.verts
    xs = list(v[0::3])
    print(shape.name, shape.guid, 'X_MIN', min(xs), 'X_MAX', max(xs))
    if not it.next(): break
"
```

Verbatim output:

```
B1 00jcUEeKFGJl_a2VaAqTeN X_MIN 0.0 X_MAX 6.0
C1 2np_18Z_6dpY6BDH2LRd_m X_MIN -0.076 X_MAX 0.076
```

**Root cause (confirmed, not inferred):** `B1` (the beam, the element the task's expected-value comment was written against) measures **exactly** `0.0` to `6.0` — its nominal 0–6000 mm endpoints, divided by 1000. `C1` (the column) measures **exactly** `-0.076` to `0.076` — half of its own 152 mm square section (76 mm), divided by 1000, straddling the column's centreline at global X=0 (the column's `IFCRECTANGLEHOLLOWPROFILEDEF` is centred on its placement axis, per the raw STEP file). The pooled `X_SPAN 6.076` is simply `6.0 − (−0.076)`: the whole-scene bounding box includes the column's own footprint at the origin, which the task's "known span" reasoning didn't account for (the column sits at X=0 with a physical cross-section, not a zero-width point). Both individual numbers are clean `/1000` conversions of the file's raw millimetre magnitudes — there is no rounding error or partial conversion.

**Answer:** `ifcopenshell.geom` (0.8.5, string-keys settings, `use-world-coords=True`) converts a millimetre-unit IFC file's geometry to **metres**. `UNIT_SCALE = 1.0` — Task 4's importer needs **no** manual 0.001 scale factor for files written by this repo's `ifc.write` agent, since Blender's own scene unit is also metres.

---

## Q5 — The EEVEE engine identifier

```
"$BLENDER" -b --python-expr "
import bpy
print('ENGINES', [i.identifier for i in bpy.types.RenderSettings.bl_rna.properties['engine'].enum_items])
"
```

Verbatim output:

```
ENGINES ['BLENDER_EEVEE']
```

This is shorter than the plan doc's stated expectation ("a list containing `CYCLES` and exactly one of `BLENDER_EEVEE` / `BLENDER_EEVEE_NEXT`") — `CYCLES` and `BLENDER_WORKBENCH` are both absent from this static enumeration. Rather than accept a surprising result at face value, verified whether those engines are actually usable:

```
"$BLENDER" -b --python-expr "
import bpy, addon_utils
print('ENGINES', [i.identifier for i in bpy.types.RenderSettings.bl_rna.properties['engine'].enum_items])
print('ACTIVE_ENGINE', bpy.context.scene.render.engine)
print('CYCLES_ADDON_STATE', addon_utils.check('cycles'))
try:
    bpy.context.scene.render.engine = 'CYCLES'
    print('SET_CYCLES_OK', bpy.context.scene.render.engine)
except Exception as e:
    print('SET_CYCLES_FAILED', repr(e))
try:
    bpy.context.scene.render.engine = 'BLENDER_WORKBENCH'
    print('SET_WORKBENCH_OK', bpy.context.scene.render.engine)
except Exception as e:
    print('SET_WORKBENCH_FAILED', repr(e))
"
```

Verbatim output:

```
ENGINES ['BLENDER_EEVEE']
ACTIVE_ENGINE BLENDER_EEVEE
CYCLES_ADDON_STATE (True, True)
SET_CYCLES_OK CYCLES
SET_WORKBENCH_OK BLENDER_WORKBENCH
```

**Answer:** The EEVEE identifier is **`BLENDER_EEVEE`** (Blender 5.2 reverted the 4.2-era `BLENDER_EEVEE_NEXT` rename back to `BLENDER_EEVEE`, matching the task's hint), and it's the scene's default active engine. **Additional finding 2:** `bpy.types.RenderSettings.bl_rna.properties['engine'].enum_items` under-reports the true engine roster — `CYCLES` and `BLENDER_WORKBENCH` do not appear in that static enum listing, but both are genuinely registered and usable (`addon_utils.check('cycles')` reports `(loaded=True, enabled=True)`, and direct assignment to `bpy.context.scene.render.engine` succeeds for both). **Anything that enumerates available engines via this static RNA introspection (rather than just trying to set the identifier it wants and catching the exception) will get an incomplete list on this Blender build.** Task 8's `_eevee_engine()` as drafted in the plan doc is unaffected — it only searches this same static list for `BLENDER_EEVEE_NEXT` / `BLENDER_EEVEE`, and `BLENDER_EEVEE` is present — but any future code that also wants to expose Cycles as a quality option should not gate on this enumeration; it should attempt the identifier directly.

---

## Q6 — Material association shape

```
"$BLENDER" -b --python-expr "
import sys, site
sys.path.insert(0, site.getusersitepackages())
import ifcopenshell
f = ifcopenshell.open(r'<probe.ifc>')
for p in f.by_type('IfcProduct'):
    mats = []
    for a in getattr(p,'HasAssociations',None) or ():
        if a.is_a('IfcRelAssociatesMaterial'):
            m = a.RelatingMaterial
            mats.append((m.is_a(), getattr(m,'Name',None)))
    print(p.is_a(), p.GlobalId, mats)
"
```

Verbatim output:

```
IfcBeam 00jcUEeKFGJl_a2VaAqTeN [('IfcMaterial', 'A992')]
IfcColumn 2np_18Z_6dpY6BDH2LRd_m [('IfcMaterial', 'A500-GR.B')]
IfcBuilding 000000000000000000000H []
IfcBuildingStorey 000000000000000000000J []
IfcSite 000000000000000000000F []
```

Cross-checked against the raw STEP file: `#51=IFCRELASSOCIATESMATERIAL(...,(#49),#26)` where `#26=IFCMATERIAL('A500-GR.B',$,$)` — a bare `IfcMaterial`, not wrapped in `IfcMaterialLayerSet`/`IfcMaterialLayerSetUsage`/`IfcMaterialProfileSet`/`IfcMaterialList`. Same shape for the beam's `#27=IFCMATERIAL('A992',$,$)`.

**Answer:** For IFC files produced by this repo's `ifc.write` agent, `IfcRelAssociatesMaterial.RelatingMaterial` is **always a plain, direct `IfcMaterial`** with `.Name` populated — never a layer-set or profile-set wrapper. The plan doc's draft `_material_of()` (in Task 4's `_ifc_import.py` snippet) already special-cases `material.is_a("IfcMaterial")` as its first, simplest branch and falls through to `ForLayerSet` / `Materials` / `MaterialLayers` unwrapping only for other producers' files — that first branch is confirmed to be the one that actually fires for this pipeline's own fixtures. The fallback unwrapping exists for defensiveness against other tools' IFC output, not because `ifc.write` needs it.

---

## Additional finding 1 — `pip install` succeeding is not the same as `import` working

Not one of the six numbered questions, but discovered while answering Q3 and directly relevant to Task 4 (`_import_ifcopenshell()`), so recorded here with equal rigor.

**Symptom:** immediately after Q2 printed `Successfully installed ifcopenshell-0.8.5 ...`, a fresh `blender -b --python-expr "import ifcopenshell"` raised:

```
Traceback (most recent call last):
  File "<string>", line 2, in <module>
ModuleNotFoundError: No module named 'ifcopenshell'
```

**Diagnosis (verified, not guessed):**

```
"$BLENDER" -b --python-expr "
import sys, site
print('ENABLE_USER_SITE', site.ENABLE_USER_SITE)
print('USER_SITE', site.getusersitepackages())
import os
print('USER_SITE_EXISTS', os.path.isdir(site.getusersitepackages()))
print('---SYS.PATH---')
for p in sys.path: print(p)
"
```

```
ENABLE_USER_SITE False
USER_SITE C:\Users\bimst\AppData\Roaming\Python\Python313\site-packages
USER_SITE_EXISTS True
---SYS.PATH---
C:\Program Files\Blender Foundation\Blender 5.2\5.2\scripts\startup
C:\Program Files\Blender Foundation\Blender 5.2\5.2\scripts\modules
C:\Program Files\Blender Foundation\Blender 5.2\python313.zip
C:\Program Files\Blender Foundation\Blender 5.2\5.2\python\DLLs
C:\Program Files\Blender Foundation\Blender 5.2\5.2\python\Lib
C:\Program Files\Blender Foundation\Blender 5.2
C:\Program Files\Blender Foundation\Blender 5.2\5.2\python
C:\Program Files\Blender Foundation\Blender 5.2\5.2\python\Lib\site-packages
C:\Program Files\Blender Foundation\Blender 5.2\5.2\scripts\freestyle\modules
C:\Users\bimst\AppData\Roaming\Blender Foundation\Blender\5.2\scripts\addons\modules
C:\Program Files\Blender Foundation\Blender 5.2\5.2\scripts\addons_core
```

Blender's bundled Python runs with `site.ENABLE_USER_SITE = False`, so the per-user site-packages directory pip installed into (`%APPDATA%\Roaming\Python\Python313\site-packages`, since the bundled global `site-packages` under `Program Files` isn't writable without elevation) is **not** on `sys.path` at startup.

Also tried and **failed**: setting the `PYTHONPATH` environment variable before launching Blender.

```
export PYTHONPATH="/c/Users/bimst/AppData/Roaming/Python/Python313/site-packages"
"$BLENDER" -b --python-expr "import os,sys; print('PYTHONPATH_ENV', repr(os.environ.get('PYTHONPATH'))); [print(p) for p in sys.path]"
```

The variable reached the process correctly (`PYTHONPATH_ENV 'C:/Users/bimst/AppData/Roaming/Python/Python313/site-packages'`) but never appeared in `sys.path` — Blender's bundled Python ignores `PYTHONPATH` entirely (isolated-mode startup).

**Verified working fix** — insert the user site-packages directory into `sys.path` from inside the script itself, before importing:

```python
import sys, site
sys.path.insert(0, site.getusersitepackages())
import ifcopenshell, ifcopenshell.geom
```

Confirmed: `ifcopenshell.__file__` then resolves to `C:\Users\bimst\AppData\Roaming\Python\Python313\site-packages\ifcopenshell\__init__.py`, and every later probe (Q3–Q6) used this fix successfully.

**Consequence for Task 4:** the plan doc's `_import_ifcopenshell()` snippet (`scripts/_ifc_import.py`) currently does a bare `import ifcopenshell` inside a `try/except ImportError`. On a machine provisioned the way this one was (`pip install ifcopenshell` run directly against Blender's bundled Python, no elevation), that bare import **will fail every time**, and every command will raise `ERR_IFCOPENSHELL_MISSING` even though the package is correctly installed. Task 4 should add the `site.getusersitepackages()` insertion (guarded, e.g. only if not already importable) before the `import ifcopenshell` line, and the installed error hint text should mention it — a plain "run `pip install ifcopenshell`" hint will leave an operator stuck in the exact loop this probe just walked out of. Flagging this for the person/session executing Task 4 rather than patching `_ifc_import.py` myself, since that file does not exist yet and Task 1 is scoped to probing only.

## Additional finding 2 — engine enumeration undercounts

Covered in full under Q5. Summary: `bpy.types.RenderSettings.bl_rna.properties['engine'].enum_items` returned only `['BLENDER_EEVEE']` on this build, omitting `CYCLES` and `BLENDER_WORKBENCH` despite both being registered, enabled, and directly settable. Anything that needs to discover available engines should attempt assignment and catch the exception rather than trust this enumeration.

---

## Self-review

- Every command above was actually run through the `Bash` tool against the real `blender.exe` on this machine — none of the six answers were inferred.
- The `X_SPAN 6.076` reading is quoted exactly as printed, not rounded to `6.0` or otherwise cleaned up, and the doc shows the follow-up per-product measurement used to explain — not explain away — the discrepancy.
- No URL or version number was invented. The `ifcopenshell --pre` / official-wheel fallback path was not exercised because the plain `pip install ifcopenshell` succeeded on the first attempt.
- Two findings surfaced that fall outside the six numbered questions (site-packages visibility, engine-enum undercount); both are recorded with the same evidence standard as the six, since both are load-bearing for Tasks 4 and 8.
