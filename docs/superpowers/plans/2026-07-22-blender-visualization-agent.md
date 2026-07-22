# Blender Visualization Agent Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Ship a `blender` agent under `20-agents/aeco/visualization/blender/` that takes an IFC file and produces production stills (PNG) and turntable videos (MP4) with no human in the loop.

**Architecture:** A set of `bpy` scripts run by `blender -b -P scripts/<command>.py -- <json>`. Geometry and semantics come from raw `ifcopenshell` (not Bonsai) reading the same file handle: the `ifcopenshell.geom` iterator tessellates into Blender meshes while IFC class / material / storey ride along as Blender custom properties. Those properties drive automatic look assignment, so the renderer never needs a human clicking materials. Results leave each script as sentinel-wrapped JSON on stdout so the CLI transport can parse them out of Blender's chatty log.

**Tech Stack:** Blender 5.2 (headless, EEVEE + Cycles), Blender's bundled Python, `ifcopenshell`, the `aware` CLI's builtin `ifc.write` for fixture generation.

**Source of truth:** [`docs/superpowers/specs/2026-07-22-blender-visualization-agent-design.md`](../specs/2026-07-22-blender-visualization-agent-design.md). Where this plan and the design doc disagree, the design doc wins.

---

## Verified starting state (checked 2026-07-22 on this machine)

These facts were established by running the commands, not by reading docs. Later tasks depend on them.

| Fact | Evidence |
|---|---|
| CLI builds clean | `cargo build` → `Finished dev profile in 1m 13s`, binary at `cli/target/debug/aware.exe` |
| `ifc.write` is a builtin, routed by agent id | [`cli/src/runtime/invoker.rs:1057`](../../../cli/src/runtime/invoker.rs) — `("ifc", "write") => crate::render::ifc::ifc_write(...)` |
| `ifc` agent is already installed in `~/.aware` | `aware agent install ./20-agents/_core/ifc` → `conflict: agent ifc already installed` |
| Invocation form works with a file arg | `aware agent invoke ifc write --inputs "@args.json"` → wrote a 6,686-byte IFC |
| Role → entity mapping is real | fixture receipt: `role:column`→`IfcColumn`, `role:beam`→`IfcBeam`, `role:brace`→`IfcMember` |
| Materials survive to IFC | fixture contains `IFCMATERIAL('A36')`, `IFCMATERIAL('A500-GR.B')`, `IFCMATERIAL('A992')` |
| Output is deterministic | header stamp is fixed: `FILE_NAME('blender-fixture-frame.ifc','1970-01-01T00:00:00',...)` |
| Blender was absent; installed for this work | `winget install --id BlenderFoundation.Blender` → **Blender 5.2.0 LTS**, build hash `fbe6228777e7`, at `C:\Program Files\Blender Foundation\Blender 5.2\blender.exe` |

**Unknowns resolved by the Task 1 probe** — measured, not assumed. Full evidence in [`2026-07-22-blender-toolchain-probe.md`](./2026-07-22-blender-toolchain-probe.md).

| Question | Answer |
|---|---|
| Blender's bundled Python | **3.13.13**, at `…\Blender 5.2\5.2\python\bin\python.exe` |
| `ifcopenshell` wheel | **Available** — `ifcopenshell-0.8.5` installed first try. The IfcConvert fork was never triggered. |
| `geom.settings()` API generation | **string-keys** — `settings.set("use-world-coords", True)` |
| Geometry unit scale | **metres — `unit-scale` stays 1.0.** The probe's 6000 mm beam measured exactly `0.0 → 6.0`. |
| EEVEE identifier | **`BLENDER_EEVEE`** (not `_NEXT`) |
| IfcMaterial association shape | **Plain `IfcMaterial` direct off `RelatingMaterial`** — no layer-set wrapping, so `_material_of`'s first branch is the live path for `ifc.write` output |

**Two probe findings that changed this plan's code — both already folded into the tasks below:**

1. **The user-site trap (Task 4).** `pip install ifcopenshell` reporting success does *not* mean `import ifcopenshell` works. Blender's bundled Python runs with `site.ENABLE_USER_SITE = False` and ignores `PYTHONPATH`, and its own `site-packages` under `Program Files` is not writable — so pip falls back to a `--user` install that nothing ever puts on `sys.path`. A bare import raises `ImportError` against a correctly installed package. `_ifc_import._import_ifcopenshell()` now inserts `site.getusersitepackages()` first. Without this, every command in Tasks 4–10 fails with a misleading `ifcopenshell-missing`.

2. **Engine enum introspection under-reports (Task 8).** `bpy.types.RenderSettings.bl_rna.properties["engine"].enum_items` returned only `['BLENDER_EEVEE']` — omitting `CYCLES` and `BLENDER_WORKBENCH` even though both are genuinely available (confirmed by direct assignment and `addon_utils.check('cycles')`). `_eevee_engine()`'s membership test still works, since `BLENDER_EEVEE` *is* in the list. But **do not add a Cycles availability check built on that enum** — it would report Cycles missing on a machine that has it. Assign the engine directly and let it fail if genuinely absent.

---

## File Structure

Everything the agent ships lives under `20-agents/aeco/visualization/blender/`.

| Path | Responsibility |
|---|---|
| `manifest.yaml` | Agent contract: requires, transport, the five commands, skills list |
| `scripts/_result.py` | Sentinel-wrapped JSON result protocol + the named-error taxonomy. No `bpy` import — importable by plain Python for testing |
| `scripts/_ifc_import.py` | `ifcopenshell` → Blender meshes, semantics as custom properties, skip-and-count for bad elements |
| `scripts/_framing.py` | Bounding-box → camera placement math. Pure math, no `bpy` state beyond the camera object it is handed |
| `scripts/_looks.py` | The semantic look mapping table (IFC class + material grade → material) |
| `scripts/scene_import.py` | Command entry: IFC → staged `.blend` |
| `scripts/scene_info.py` | Command entry: inventory readback |
| `scripts/scene_apply_look.py` | Command entry: apply a preset look |
| `scripts/render_still.py` | Command entry: framed still |
| `scripts/render_turntable.py` | Command entry: orbit MP4 |
| `skills/headless-rendering.md` | How unattended Blender rendering works and where it bites |
| `skills/ifc-import-ifcopenshell.md` | The import approach and why not Bonsai |
| `skills/look-presets.md` | The look mapping table and how to extend it |
| `tests/fixture-scene.json` | The reference scene, fed to `ifc.write` at test time (no binary committed) |
| `tests/run_smoke.py` | Headless smoke harness; skips with a clear message when Blender is absent |

Repo-level touches: `registry-index.json` (new entry), stats markers via `scripts/sync_stats.py --write`, and `30-apps/_examples/model-to-renders.app`.

**Why separate modules rather than one script:** each command is its own `-P` entry point, but import, framing, and looks are shared by three commands each. Underscore-prefixed modules are the shared half; the command scripts stay thin enough to read in one screen.

---

## Task 1: Prove the toolchain

No code ships from this task. It answers the four unknowns above and writes them down, because every later task's code depends on the answers.

**Files:**
- Create: `docs/superpowers/plans/2026-07-22-blender-toolchain-probe.md` (findings, deleted before the final commit if the facts land in the skills instead)

- [ ] **Step 1: Locate the installed Blender and record its version**

```bash
"/c/Program Files/Blender Foundation/Blender 5.2/blender.exe" --version
```

Expected: `Blender 5.2.0` plus build hash. If the path differs, find it with:

```bash
ls "/c/Program Files/Blender Foundation/"
```

Record the absolute path — every later command uses it. Export it for convenience:

```bash
export BLENDER="/c/Program Files/Blender Foundation/Blender 5.2/blender.exe"
```

- [ ] **Step 2: Find Blender's bundled Python and its version**

```bash
"$BLENDER" -b --python-expr "import sys; print('PYEXE', sys.executable); print('PYVER', sys.version)"
```

Expected: a `PYEXE` line pointing at `.../5.2/python/bin/python.exe` and a `PYVER` line. Record both.

- [ ] **Step 3: Install `ifcopenshell` into Blender's bundled Python**

```bash
"$BLENDER" -b --python-expr "import sys,subprocess; subprocess.check_call([sys.executable,'-m','pip','install','--upgrade','pip']); subprocess.check_call([sys.executable,'-m','pip','install','ifcopenshell'])"
```

Expected: pip resolves a wheel for the bundled Python version and reports `Successfully installed ifcopenshell-<ver>`.

**If no wheel exists for that Python version** — this is the one genuine fork in the plan. Do not improvise silently. In order:
1. Try `pip install ifcopenshell --pre`.
2. Try the official IfcOpenShell Blender-Python wheels from `https://docs.ifcopenshell.org/` (verify the URL resolves before trusting it).
3. If both fail, stop and report: the fallback is the design doc's documented escape hatch — IfcConvert preprocessing (IFC → glTF + JSON sidecar) — which is a **v2 scope change** and needs the user's decision, not a unilateral pivot.

- [ ] **Step 4: Determine the `ifcopenshell.geom.settings()` API generation**

```bash
"$BLENDER" -b --python-expr "
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

Expected: exactly one `API_GENERATION` line. Record which. `_ifc_import.py` will support both, but knowing the local one makes failures readable.

- [ ] **Step 5: Determine the geometry unit scale — the trap that silently ruins framing**

Generate the fixture IFC first (this also proves the fixture pipeline for Task 2):

```bash
./cli/target/debug/aware.exe agent invoke ifc write --inputs "@tests/fixture-args.json"
```

where `tests/fixture-args.json` wraps `tests/fixture-scene.json` with an `output-path`. Then measure a known span — the fixture's `B1` beam runs 0 → 6000 mm in X:

```bash
"$BLENDER" -b --python-expr "
import ifcopenshell, ifcopenshell.geom
f = ifcopenshell.open(r'FIXTURE.ifc')
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
"
```

Expected: `X_SPAN 6.0` (metres — ifcopenshell converted) or `X_SPAN 6000.0` (millimetres — raw file units). Record which. **Blender's scene unit is metres**, so if the span is 6000 the importer must scale by 0.001 or every camera frame and every render will be wrong.

- [ ] **Step 6: Determine the EEVEE engine enum**

```bash
"$BLENDER" -b --python-expr "
import bpy
print('ENGINES', [i.identifier for i in bpy.types.RenderSettings.bl_rna.properties['engine'].enum_items])
"
```

Expected: a list containing `CYCLES` and exactly one of `BLENDER_EEVEE` / `BLENDER_EEVEE_NEXT`. Record the EEVEE identifier.

- [ ] **Step 7: Write the findings down**

Record all six answers in the probe doc: Blender path + version, Python version, ifcopenshell version, API generation, unit scale, EEVEE identifier. Later tasks quote these; a future session on another machine re-runs this task rather than guessing.

- [ ] **Step 8: Commit**

```bash
git add docs/superpowers/plans/2026-07-22-blender-visualization-agent.md docs/superpowers/plans/2026-07-22-blender-toolchain-probe.md
git commit -m "docs(plans): blender agent implementation plan + verified toolchain probe"
```

---

## Task 2: The reference fixture

**Files:**
- Create: `20-agents/aeco/visualization/blender/tests/fixture-scene.json`
- Create: `20-agents/aeco/visualization/blender/tests/make_fixture.py`

The fixture is a nine-element portal frame: four HSS columns (A500-GR.B), four W-section beams (A992), one angle brace (A36). It exercises all three IFC entity types the writer emits and all three material grades the look mapper keys on. It is committed as **scene JSON**, never as an `.ifc` binary — `ifc.write` is deterministic, so the IFC is regenerated byte-identically at test time.

- [ ] **Step 1: Write the fixture scene**

Create `20-agents/aeco/visualization/blender/tests/fixture-scene.json`:

```json
{
  "meta": { "name": "Blender Fixture Frame", "units": "mm", "up": "z" },
  "groups": [
    { "key": "W16X26", "label": "Beams", "color": "#60a5fa", "opacity": 1 },
    { "key": "HSS6X6", "label": "Columns", "color": "#f59e0b", "opacity": 1 }
  ],
  "elements": [
    { "id": "C1", "group": "HSS6X6", "role": "column", "material": "A500-GR.B",
      "from": [0, 0, 0], "to": [0, 0, 3000], "section": { "w": 152, "d": 152 },
      "meta": { "profile": "HSS6X6X3/8" },
      "xsection": { "shape": "rhs", "d": 152, "b": 152, "t": 9 } },
    { "id": "C2", "group": "HSS6X6", "role": "column", "material": "A500-GR.B",
      "from": [6000, 0, 0], "to": [6000, 0, 3000], "section": { "w": 152, "d": 152 },
      "meta": { "profile": "HSS6X6X3/8" },
      "xsection": { "shape": "rhs", "d": 152, "b": 152, "t": 9 } },
    { "id": "C3", "group": "HSS6X6", "role": "column", "material": "A500-GR.B",
      "from": [0, 4000, 0], "to": [0, 4000, 3000], "section": { "w": 152, "d": 152 },
      "meta": { "profile": "HSS6X6X3/8" },
      "xsection": { "shape": "rhs", "d": 152, "b": 152, "t": 9 } },
    { "id": "C4", "group": "HSS6X6", "role": "column", "material": "A500-GR.B",
      "from": [6000, 4000, 0], "to": [6000, 4000, 3000], "section": { "w": 152, "d": 152 },
      "meta": { "profile": "HSS6X6X3/8" },
      "xsection": { "shape": "rhs", "d": 152, "b": 152, "t": 9 } },
    { "id": "B1", "group": "W16X26", "role": "beam", "material": "A992",
      "from": [0, 0, 3000], "to": [6000, 0, 3000], "section": { "w": 140, "d": 400 },
      "meta": { "profile": "W16X26" },
      "xsection": { "shape": "i", "d": 400, "bf": 140, "tw": 6, "tf": 9 } },
    { "id": "B2", "group": "W16X26", "role": "beam", "material": "A992",
      "from": [0, 4000, 3000], "to": [6000, 4000, 3000], "section": { "w": 140, "d": 400 },
      "meta": { "profile": "W16X26" },
      "xsection": { "shape": "i", "d": 400, "bf": 140, "tw": 6, "tf": 9 } },
    { "id": "B3", "group": "W16X26", "role": "beam", "material": "A992",
      "from": [0, 0, 3000], "to": [0, 4000, 3000], "section": { "w": 140, "d": 400 },
      "meta": { "profile": "W16X26" },
      "xsection": { "shape": "i", "d": 400, "bf": 140, "tw": 6, "tf": 9 } },
    { "id": "B4", "group": "W16X26", "role": "beam", "material": "A992",
      "from": [6000, 0, 3000], "to": [6000, 4000, 3000], "section": { "w": 140, "d": 400 },
      "meta": { "profile": "W16X26" },
      "xsection": { "shape": "i", "d": 400, "bf": 140, "tw": 6, "tf": 9 } },
    { "id": "BR1", "group": "W16X26", "role": "brace", "material": "A36",
      "from": [0, 0, 0], "to": [6000, 0, 3000], "section": { "w": 100, "d": 100 },
      "meta": { "profile": "L4X4X1/2" },
      "xsection": { "shape": "angle", "d": 102, "b": 102, "t": 13 } }
  ]
}
```

- [ ] **Step 2: Write the fixture generator**

Create `20-agents/aeco/visualization/blender/tests/make_fixture.py`:

```python
"""Regenerate the reference IFC from the committed scene JSON via the aware CLI.

The IFC is never committed: `ifc.write` is deterministic, so regenerating it at
test time gives a stable asset without a binary in git.
"""

import argparse
import json
import subprocess
import sys
import tempfile
from pathlib import Path

HERE = Path(__file__).resolve().parent


def build_fixture(aware_bin: str, out_ifc: Path) -> dict:
    """Run `ifc.write` over the committed scene; return the CLI's receipt."""
    scene = json.loads((HERE / "fixture-scene.json").read_text(encoding="utf-8"))
    payload = {"scene": scene, "output-path": str(out_ifc)}

    with tempfile.NamedTemporaryFile(
        "w", suffix=".json", delete=False, encoding="utf-8"
    ) as handle:
        json.dump(payload, handle)
        args_path = Path(handle.name)

    try:
        proc = subprocess.run(
            [aware_bin, "agent", "invoke", "ifc", "write", "--inputs", f"@{args_path}"],
            capture_output=True,
            text=True,
            check=False,
        )
    finally:
        args_path.unlink(missing_ok=True)

    if proc.returncode != 0:
        raise RuntimeError(f"ifc.write failed ({proc.returncode}):\n{proc.stderr}")
    if not out_ifc.exists():
        raise RuntimeError(f"ifc.write reported success but {out_ifc} is missing")
    return json.loads(proc.stdout[proc.stdout.index("{"):])


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--aware-bin", default="aware", help="path to the aware binary")
    parser.add_argument("--out", required=True, type=Path, help="IFC output path")
    opts = parser.parse_args()

    receipt = build_fixture(opts.aware_bin, opts.out)
    print(f"fixture: {opts.out} ({len(receipt.get('emitted', []))} elements)")
    return 0


if __name__ == "__main__":
    sys.exit(main())
```

- [ ] **Step 3: Run it and verify the receipt**

```bash
python 20-agents/aeco/visualization/blender/tests/make_fixture.py --aware-bin ./cli/target/debug/aware.exe --out /tmp/fixture.ifc
```

Expected: `fixture: /tmp/fixture.ifc (9 elements)`.

- [ ] **Step 4: Verify the IFC carries the semantics the look mapper needs**

```bash
grep -c "IFCCOLUMN\|IFCBEAM\|IFCMEMBER" /tmp/fixture.ifc
grep -o "IFCMATERIAL('[^']*'" /tmp/fixture.ifc | sort -u
```

Expected: count `9`, and exactly three materials — `A36`, `A500-GR.B`, `A992`.

- [ ] **Step 5: Commit**

```bash
git add 20-agents/aeco/visualization/blender/tests/fixture-scene.json 20-agents/aeco/visualization/blender/tests/make_fixture.py
git commit -m "test(blender): reference portal-frame fixture generated through ifc.write"
```

---

## Task 3: The result protocol and error taxonomy

**Files:**
- Create: `20-agents/aeco/visualization/blender/scripts/_result.py`

Blender floods stdout. The transport needs to find the payload in that noise, and the design doc requires *named* errors. This module owns both. It deliberately does not import `bpy`, so it can be exercised with plain Python.

- [ ] **Step 1: Write the module**

Create `20-agents/aeco/visualization/blender/scripts/_result.py`:

```python
"""Result and error protocol shared by every aware-blender command script.

Blender writes progress, warnings and render statistics to stdout, so a bare
`print(json.dumps(...))` is not recoverable by the caller. Every script instead
frames its payload between sentinels; the transport slices between them.

Exit codes: 0 on success, 1 on a named failure. A named failure still emits a
framed payload, so the caller always gets structured data.
"""

import json
import sys
import traceback

RESULT_BEGIN = "<<<AWARE_RESULT>>>"
RESULT_END = "<<<AWARE_RESULT_END>>>"

# The named error taxonomy. The transport surfaces `code` verbatim, so these
# strings are contract: renaming one is a breaking change.
ERR_IFCOPENSHELL_MISSING = "ifcopenshell-missing"
ERR_IFC_UNREADABLE = "ifc-unreadable"
ERR_IFC_EMPTY = "ifc-empty"
ERR_BLEND_UNREADABLE = "blend-unreadable"
ERR_INVALID_INPUTS = "invalid-inputs"
ERR_RENDER_FAILED = "render-failed"
ERR_UNEXPECTED = "unexpected-error"

# Install hint reused by the ifcopenshell guard; the design doc requires the
# exact one-liner to travel with the error.
IFCOPENSHELL_HINT = (
    "Install it into Blender's bundled Python:\n"
    '  blender -b --python-expr "import sys,subprocess; '
    "subprocess.check_call([sys.executable,'-m','pip','install','ifcopenshell'])\""
)


class AwareBlenderError(Exception):
    """A failure with a stable machine-readable code and an actionable hint."""

    def __init__(self, code: str, message: str, hint: str = "", **details):
        super().__init__(message)
        self.code = code
        self.message = message
        self.hint = hint
        self.details = details

    def payload(self) -> dict:
        out = {"ok": False, "code": self.code, "message": self.message}
        if self.hint:
            out["hint"] = self.hint
        out.update(self.details)
        return out


def emit(payload: dict) -> None:
    """Frame a payload on stdout so the transport can recover it."""
    sys.stdout.flush()
    print(RESULT_BEGIN)
    print(json.dumps(payload, indent=2, sort_keys=True))
    print(RESULT_END)
    sys.stdout.flush()


def parse_args(argv: list[str]) -> dict:
    """Read the JSON blob that follows `--` in a `blender -b -P script.py -- {...}` call.

    Accepts either inline JSON or `@path/to/file.json` for payloads that would
    otherwise hit the command-line length limit.
    """
    if "--" not in argv:
        raise AwareBlenderError(
            ERR_INVALID_INPUTS,
            "no `--` separator found; expected `blender -b -P script.py -- <json>`",
        )
    rest = argv[argv.index("--") + 1:]
    if not rest:
        raise AwareBlenderError(ERR_INVALID_INPUTS, "no JSON payload after `--`")

    raw = rest[0]
    if raw.startswith("@"):
        try:
            with open(raw[1:], encoding="utf-8") as handle:
                raw = handle.read()
        except OSError as exc:
            raise AwareBlenderError(
                ERR_INVALID_INPUTS, f"cannot read inputs file {raw[1:]}: {exc}"
            ) from exc
    try:
        parsed = json.loads(raw)
    except json.JSONDecodeError as exc:
        raise AwareBlenderError(
            ERR_INVALID_INPUTS, f"inputs are not valid JSON: {exc}"
        ) from exc
    if not isinstance(parsed, dict):
        raise AwareBlenderError(ERR_INVALID_INPUTS, "inputs must be a JSON object")
    return parsed


def require(inputs: dict, key: str) -> object:
    """Fetch a required input or raise the named invalid-inputs error."""
    if key not in inputs or inputs[key] in (None, ""):
        raise AwareBlenderError(
            ERR_INVALID_INPUTS, f"required input `{key}` is missing"
        )
    return inputs[key]


def run(main_fn) -> None:
    """Entry-point wrapper: run `main_fn(inputs) -> dict`, frame whatever happens.

    Every command script ends with `_result.run(main)`. Unexpected exceptions are
    caught and framed too, so the caller never has to scrape a Python traceback
    out of Blender's log to find out what went wrong.
    """
    try:
        inputs = parse_args(sys.argv)
        payload = main_fn(inputs)
        payload.setdefault("ok", True)
        emit(payload)
        sys.exit(0)
    except AwareBlenderError as exc:
        emit(exc.payload())
        sys.exit(1)
    except Exception as exc:  # noqa: BLE001 - last resort, must stay structured
        emit(
            {
                "ok": False,
                "code": ERR_UNEXPECTED,
                "message": f"{type(exc).__name__}: {exc}",
                "traceback": traceback.format_exc(),
            }
        )
        sys.exit(1)
```

- [ ] **Step 2: Test the protocol with plain Python (no Blender needed)**

```bash
python -c "
import sys, json
sys.path.insert(0, '20-agents/aeco/visualization/blender/scripts')
import _result

# happy path
assert _result.parse_args(['blender','-b','--','{\"a\":1}']) == {'a': 1}
# missing separator
try:
    _result.parse_args(['blender','-b']); raise SystemExit('should have raised')
except _result.AwareBlenderError as e:
    assert e.code == _result.ERR_INVALID_INPUTS, e.code
# malformed json
try:
    _result.parse_args(['--','{nope}']); raise SystemExit('should have raised')
except _result.AwareBlenderError as e:
    assert e.code == _result.ERR_INVALID_INPUTS, e.code
# non-object json
try:
    _result.parse_args(['--','[1,2]']); raise SystemExit('should have raised')
except _result.AwareBlenderError as e:
    assert e.code == _result.ERR_INVALID_INPUTS, e.code
# required input
try:
    _result.require({}, 'ifc-path'); raise SystemExit('should have raised')
except _result.AwareBlenderError as e:
    assert 'ifc-path' in e.message, e.message
print('OK _result protocol')
"
```

Expected: `OK _result protocol`.

- [ ] **Step 3: Commit**

```bash
git add 20-agents/aeco/visualization/blender/scripts/_result.py
git commit -m "feat(blender): sentinel result protocol and named error taxonomy"
```

---

## Task 4: IFC import

**Files:**
- Create: `20-agents/aeco/visualization/blender/scripts/_ifc_import.py`
- Create: `20-agents/aeco/visualization/blender/scripts/scene_import.py`

The heart of the agent. Uses the `ifcopenshell.geom` iterator to tessellate, builds one Blender mesh per IFC product, and writes class / material / storey onto each object as custom properties so downstream commands never need to reopen the IFC.

**Substitute the Task 1 findings** for `UNIT_SCALE` and the settings API before running.

- [ ] **Step 1: Write the importer**

Create `20-agents/aeco/visualization/blender/scripts/_ifc_import.py`:

```python
"""IFC -> Blender meshes, carrying IFC semantics as object custom properties.

Raw ifcopenshell, not Bonsai: Bonsai is GUI-oriented with no confirmed clean
story under `blender -b`. The geom iterator tessellates parametric profiles and
the same file handle answers the class / material / storey questions, so one
pass produces both geometry and the semantics the look mapper needs.

A product that fails to tessellate is skipped and counted, never fatal -- a
single bad element must not cost the caller the whole render.
"""

import sys

import bpy

import _result

# Custom-property keys written onto every imported object. Downstream commands
# read these instead of reopening the IFC.
PROP_GUID = "ifc_guid"
PROP_CLASS = "ifc_class"
PROP_NAME = "ifc_name"
PROP_MATERIAL = "ifc_material"
PROP_STOREY = "ifc_storey"


def _import_ifcopenshell():
    """Import ifcopenshell or raise the named error with the install one-liner.

    The user-site path insert is NOT optional (Task 1 probe finding). Blender's
    bundled Python runs with `site.ENABLE_USER_SITE = False` and ignores
    PYTHONPATH, while its own site-packages under Program Files is not writable
    -- so `pip install ifcopenshell` silently falls back to a --user install that
    nothing ever adds to sys.path. Without this, a correctly installed
    ifcopenshell still raises ImportError on every run.
    """
    import site

    user_site = site.getusersitepackages()
    if user_site and user_site not in sys.path:
        sys.path.insert(0, user_site)

    try:
        import ifcopenshell
        import ifcopenshell.geom

        return ifcopenshell
    except ImportError as exc:
        raise _result.AwareBlenderError(
            _result.ERR_IFCOPENSHELL_MISSING,
            f"ifcopenshell is not available in Blender's Python: {exc}",
            hint=_result.IFCOPENSHELL_HINT,
        ) from exc


def _make_settings(ifcopenshell):
    """Build geom settings across the 0.7 (enum attrs) / 0.8+ (string keys) split."""
    settings = ifcopenshell.geom.settings()
    try:
        settings.set("use-world-coords", True)
    except (TypeError, AttributeError, RuntimeError):
        settings.set(settings.USE_WORLD_COORDS, True)
    return settings


def _material_of(product) -> str:
    """First associated IfcMaterial name, or empty string."""
    for association in getattr(product, "HasAssociations", None) or ():
        if not association.is_a("IfcRelAssociatesMaterial"):
            continue
        material = association.RelatingMaterial
        if material.is_a("IfcMaterial"):
            return material.Name or ""
        # IfcMaterialLayerSetUsage / IfcMaterialList etc. -- take the first leaf.
        for attr in ("ForLayerSet", "Materials", "MaterialLayers"):
            nested = getattr(material, attr, None)
            if nested:
                first = nested[0] if isinstance(nested, (list, tuple)) else nested
                name = getattr(first, "Name", None)
                if name:
                    return name
    return ""


def _storey_of(product) -> str:
    """Containing IfcBuildingStorey name, or empty string."""
    for rel in getattr(product, "ContainedInStructure", None) or ():
        structure = getattr(rel, "RelatingStructure", None)
        if structure is not None and structure.is_a("IfcBuildingStorey"):
            return structure.Name or ""
    return ""


def import_ifc(ifc_path: str, unit_scale: float = 1.0) -> dict:
    """Tessellate every product in `ifc_path` into the current Blender scene.

    `unit_scale` multiplies incoming coordinates. Blender's scene unit is metres;
    set this from the Task 1 probe (1.0 when ifcopenshell already converts to
    metres, 0.001 when it hands back raw millimetres).

    Returns a receipt: counts, per-class inventory, and the skipped GUIDs.
    """
    ifcopenshell = _import_ifcopenshell()

    try:
        ifc_file = ifcopenshell.open(ifc_path)
    except Exception as exc:  # ifcopenshell raises bare Exception on parse failure
        raise _result.AwareBlenderError(
            _result.ERR_IFC_UNREADABLE,
            f"cannot open IFC {ifc_path}: {exc}",
        ) from exc

    settings = _make_settings(ifcopenshell)
    iterator = ifcopenshell.geom.iterator(settings, ifc_file)
    if not iterator.initialize():
        raise _result.AwareBlenderError(
            _result.ERR_IFC_EMPTY,
            f"IFC {ifc_path} contains no tessellatable geometry",
        )

    collection = bpy.data.collections.new("IFC")
    bpy.context.scene.collection.children.link(collection)

    imported = 0
    skipped: list[dict] = []
    by_class: dict[str, int] = {}
    by_material: dict[str, int] = {}
    by_storey: dict[str, int] = {}

    while True:
        try:
            shape = iterator.get()
            product = ifc_file.by_guid(shape.guid)

            verts = shape.geometry.verts
            faces = shape.geometry.faces
            if not verts or not faces:
                raise ValueError("empty tessellation")

            coords = [
                (
                    verts[i] * unit_scale,
                    verts[i + 1] * unit_scale,
                    verts[i + 2] * unit_scale,
                )
                for i in range(0, len(verts), 3)
            ]
            tris = [
                (faces[i], faces[i + 1], faces[i + 2])
                for i in range(0, len(faces), 3)
            ]

            mesh = bpy.data.meshes.new(shape.name or shape.guid)
            mesh.from_pydata(coords, [], tris)
            mesh.validate()
            mesh.update()

            obj = bpy.data.objects.new(shape.name or shape.guid, mesh)
            ifc_class = product.is_a()
            material = _material_of(product)
            storey = _storey_of(product)

            obj[PROP_GUID] = shape.guid
            obj[PROP_CLASS] = ifc_class
            obj[PROP_NAME] = product.Name or ""
            obj[PROP_MATERIAL] = material
            obj[PROP_STOREY] = storey

            collection.objects.link(obj)

            imported += 1
            by_class[ifc_class] = by_class.get(ifc_class, 0) + 1
            if material:
                by_material[material] = by_material.get(material, 0) + 1
            if storey:
                by_storey[storey] = by_storey.get(storey, 0) + 1

        except Exception as exc:  # noqa: BLE001 - skip-and-count is the contract
            guid = ""
            try:
                guid = iterator.get().guid
            except Exception:  # noqa: BLE001
                pass
            skipped.append({"guid": guid, "reason": f"{type(exc).__name__}: {exc}"})

        if not iterator.next():
            break

    if imported == 0:
        raise _result.AwareBlenderError(
            _result.ERR_IFC_EMPTY,
            f"no element of {ifc_path} could be imported",
            skipped=skipped,
        )

    return {
        "imported": imported,
        "skipped": skipped,
        "skipped-count": len(skipped),
        "by-class": dict(sorted(by_class.items())),
        "by-material": dict(sorted(by_material.items())),
        "by-storey": dict(sorted(by_storey.items())),
    }


def clear_scene() -> None:
    """Empty the default startup scene so an import starts from nothing."""
    bpy.ops.object.select_all(action="SELECT")
    bpy.ops.object.delete(use_global=False)
    for block in (bpy.data.meshes, bpy.data.materials, bpy.data.cameras):
        for item in list(block):
            if item.users == 0:
                block.remove(item)
```

- [ ] **Step 2: Write the `scene.import` command entry**

Create `20-agents/aeco/visualization/blender/scripts/scene_import.py`:

```python
"""`scene.import` -- IFC in, staged .blend out.

Run: blender -b -P scene_import.py -- '{"ifc-path":"m.ifc","blend-path":"m.blend"}'
"""

import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))

import bpy  # noqa: E402

import _ifc_import  # noqa: E402
import _result  # noqa: E402


def main(inputs: dict) -> dict:
    ifc_path = str(_result.require(inputs, "ifc-path"))
    blend_path = str(_result.require(inputs, "blend-path"))
    unit_scale = float(inputs.get("unit-scale", 1.0))

    if not os.path.exists(ifc_path):
        raise _result.AwareBlenderError(
            _result.ERR_IFC_UNREADABLE, f"IFC not found: {ifc_path}"
        )

    _ifc_import.clear_scene()
    receipt = _ifc_import.import_ifc(ifc_path, unit_scale=unit_scale)

    os.makedirs(os.path.dirname(os.path.abspath(blend_path)), exist_ok=True)
    bpy.ops.wm.save_as_mainfile(filepath=os.path.abspath(blend_path))

    receipt["blend-path"] = os.path.abspath(blend_path)
    receipt["path"] = receipt["blend-path"]
    return receipt


_result.run(main)
```

- [ ] **Step 3: Run it against the fixture**

```bash
"$BLENDER" -b -P 20-agents/aeco/visualization/blender/scripts/scene_import.py -- "{\"ifc-path\":\"/tmp/fixture.ifc\",\"blend-path\":\"/tmp/fixture.blend\"}"
```

Expected between the sentinels:

```json
{
  "by-class": { "IfcBeam": 4, "IfcColumn": 4, "IfcMember": 1 },
  "by-material": { "A36": 1, "A500-GR.B": 4, "A992": 4 },
  "imported": 9,
  "ok": true,
  "skipped": [],
  "skipped-count": 0
}
```

**This is the moment the design's central claim is proven or falsified**: that IFC class *and* material grade both survive import as queryable per-object data. If `by-material` is empty, `_material_of` is not walking the association the writer emitted — inspect with:

```bash
grep -n "IFCRELASSOCIATESMATERIAL" /tmp/fixture.ifc | head
```

and fix the traversal before continuing. Do not proceed with an empty material inventory; the look mapper has nothing to key on without it.

- [ ] **Step 4: Verify the geometry landed at the right scale**

```bash
"$BLENDER" -b /tmp/fixture.blend --python-expr "
import bpy
xs = [ (o.matrix_world @ v.co).x for o in bpy.data.objects if o.type=='MESH' for v in o.data.vertices ]
print('X_SPAN', round(max(xs)-min(xs), 3))
print('OBJECTS', len([o for o in bpy.data.objects if o.type=='MESH']))
"
```

Expected: `X_SPAN 6.0` (the fixture frame is 6 m long) and `OBJECTS 9`. A span of `6000.0` means `unit-scale` needs to be `0.001` — set it and re-run Step 3.

- [ ] **Step 5: Verify the error path**

```bash
"$BLENDER" -b -P 20-agents/aeco/visualization/blender/scripts/scene_import.py -- "{\"ifc-path\":\"/tmp/nope.ifc\",\"blend-path\":\"/tmp/x.blend\"}"
```

Expected: framed payload with `"ok": false` and `"code": "ifc-unreadable"`, exit code 1.

- [ ] **Step 6: Commit**

```bash
git add 20-agents/aeco/visualization/blender/scripts/_ifc_import.py 20-agents/aeco/visualization/blender/scripts/scene_import.py
git commit -m "feat(blender): IFC import via ifcopenshell with semantics as custom properties"
```

---

## Task 5: `scene.info`

**Files:**
- Create: `20-agents/aeco/visualization/blender/scripts/scene_info.py`

The read-only inventory command. It is also the plan's verification instrument — the design doc's look-mapping check is "`scene.info` output matches the fixture's known class/material inventory."

- [ ] **Step 1: Write the command**

Create `20-agents/aeco/visualization/blender/scripts/scene_info.py`:

```python
"""`scene.info` -- inventory a staged .blend (or an IFC directly) by class,
material and storey. Read-only.

Run: blender -b -P scene_info.py -- '{"blend-path":"m.blend"}'
  or blender -b -P scene_info.py -- '{"ifc-path":"m.ifc"}'
"""

import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))

import bpy  # noqa: E402

import _ifc_import  # noqa: E402
import _result  # noqa: E402


def _inventory() -> dict:
    """Tally the current scene's objects by their IFC custom properties."""
    by_class: dict[str, int] = {}
    by_material: dict[str, int] = {}
    by_storey: dict[str, int] = {}
    elements = []

    for obj in bpy.data.objects:
        if obj.type != "MESH":
            continue
        ifc_class = obj.get(_ifc_import.PROP_CLASS, "")
        material = obj.get(_ifc_import.PROP_MATERIAL, "")
        storey = obj.get(_ifc_import.PROP_STOREY, "")
        if ifc_class:
            by_class[ifc_class] = by_class.get(ifc_class, 0) + 1
        if material:
            by_material[material] = by_material.get(material, 0) + 1
        if storey:
            by_storey[storey] = by_storey.get(storey, 0) + 1
        elements.append(
            {
                "guid": obj.get(_ifc_import.PROP_GUID, ""),
                "name": obj.get(_ifc_import.PROP_NAME, "") or obj.name,
                "class": ifc_class,
                "material": material,
                "storey": storey,
            }
        )

    return {
        "count": len(elements),
        "by-class": dict(sorted(by_class.items())),
        "by-material": dict(sorted(by_material.items())),
        "by-storey": dict(sorted(by_storey.items())),
        "elements": sorted(elements, key=lambda e: (e["class"], e["name"])),
    }


def main(inputs: dict) -> dict:
    blend_path = inputs.get("blend-path")
    ifc_path = inputs.get("ifc-path")
    if not blend_path and not ifc_path:
        raise _result.AwareBlenderError(
            _result.ERR_INVALID_INPUTS,
            "one of `blend-path` or `ifc-path` is required",
        )

    if blend_path:
        if not os.path.exists(str(blend_path)):
            raise _result.AwareBlenderError(
                _result.ERR_BLEND_UNREADABLE, f".blend not found: {blend_path}"
            )
        bpy.ops.wm.open_mainfile(filepath=os.path.abspath(str(blend_path)))
    else:
        if not os.path.exists(str(ifc_path)):
            raise _result.AwareBlenderError(
                _result.ERR_IFC_UNREADABLE, f"IFC not found: {ifc_path}"
            )
        _ifc_import.clear_scene()
        _ifc_import.import_ifc(
            str(ifc_path), unit_scale=float(inputs.get("unit-scale", 1.0))
        )

    return _inventory()


_result.run(main)
```

- [ ] **Step 2: Run it and check against the fixture's known inventory**

```bash
"$BLENDER" -b -P 20-agents/aeco/visualization/blender/scripts/scene_info.py -- "{\"blend-path\":\"/tmp/fixture.blend\"}"
```

Expected: `"count": 9`, `by-class` = `{IfcBeam: 4, IfcColumn: 4, IfcMember: 1}`, `by-material` = `{A36: 1, A500-GR.B: 4, A992: 4}` — matching the fixture scene JSON exactly.

- [ ] **Step 3: Commit**

```bash
git add 20-agents/aeco/visualization/blender/scripts/scene_info.py
git commit -m "feat(blender): scene.info inventory by IFC class, material and storey"
```

---

## Task 6: Camera framing

**Files:**
- Create: `20-agents/aeco/visualization/blender/scripts/_framing.py`

`bpy.ops.view3d.camera_to_view_selected()` needs a 3D viewport context that does not exist under `-b`, so framing is computed directly. The camera is placed along a chosen direction at the distance that fits the model's bounding sphere inside the narrower of the two half-FOVs.

Bounding *sphere* rather than exact box projection is deliberate: it is rotation-invariant, so the turntable never clips as the model spins. It costs some framing tightness, which `margin` tunes.

- [ ] **Step 1: Write the module**

Create `20-agents/aeco/visualization/blender/scripts/_framing.py`:

```python
"""Headless camera framing.

`bpy.ops.view3d.camera_to_view_selected()` requires a 3D viewport that does not
exist in background mode, so the fit is computed from the scene bounds.

The fit uses the model's bounding SPHERE, not its projected box: the sphere is
rotation-invariant, so a turntable orbit cannot clip the model at some angle the
framing pass never sampled. The cost is a slightly loose crop, tuned by `margin`.
"""

import math

import bpy
from mathutils import Vector

# Named view directions, in Blender's Z-up world. Each is a unit-ish vector FROM
# the model TOWARD the camera; they are normalized before use.
DIRECTIONS = {
    "iso": Vector((1.0, -1.0, 0.7)),
    "front": Vector((0.0, -1.0, 0.0)),
    "back": Vector((0.0, 1.0, 0.0)),
    "left": Vector((-1.0, 0.0, 0.0)),
    "right": Vector((1.0, 0.0, 0.0)),
    "top": Vector((0.0, 0.0, 1.0)),
}


def scene_bounds() -> tuple[Vector, Vector]:
    """World-space min/max corners over every mesh object."""
    lo = Vector((math.inf,) * 3)
    hi = Vector((-math.inf,) * 3)
    found = False
    for obj in bpy.data.objects:
        if obj.type != "MESH":
            continue
        for corner in obj.bound_box:
            world = obj.matrix_world @ Vector(corner)
            for axis in range(3):
                lo[axis] = min(lo[axis], world[axis])
                hi[axis] = max(hi[axis], world[axis])
            found = True
    if not found:
        raise ValueError("scene contains no mesh geometry to frame")
    return lo, hi


def frame_camera(
    camera: bpy.types.Object,
    direction: str = "iso",
    margin: float = 1.10,
) -> dict:
    """Place `camera` so the whole scene fits, looking at the model centre.

    Returns the framing receipt (centre, radius, distance) for the caller's log.
    """
    lo, hi = scene_bounds()
    centre = (lo + hi) / 2.0
    radius = (hi - lo).length / 2.0
    if radius <= 0.0:
        raise ValueError("scene bounding box is degenerate")

    vector = DIRECTIONS.get(direction)
    if vector is None:
        raise ValueError(
            f"unknown direction `{direction}`; expected one of {sorted(DIRECTIONS)}"
        )

    scene = bpy.context.scene
    render = scene.render
    width = render.resolution_x * (render.pixel_aspect_x or 1.0)
    height = render.resolution_y * (render.pixel_aspect_y or 1.0)

    cam_data = camera.data
    # `angle` is the FOV across the sensor-fit axis; derive the other from aspect.
    if width >= height:
        half_x = cam_data.angle / 2.0
        half_y = math.atan(math.tan(half_x) * height / width)
    else:
        half_y = cam_data.angle / 2.0
        half_x = math.atan(math.tan(half_y) * width / height)

    half_fov = min(half_x, half_y)
    distance = (radius / math.sin(half_fov)) * margin

    camera.location = centre + vector.normalized() * distance
    look = (centre - camera.location).normalized()
    camera.rotation_euler = look.to_track_quat("-Z", "Y").to_euler()

    # Keep the model comfortably inside the clip range at any orbit angle.
    cam_data.clip_start = max(distance - radius * 4.0, distance / 1000.0)
    cam_data.clip_end = distance + radius * 4.0

    return {
        "centre": [round(v, 6) for v in centre],
        "radius": round(radius, 6),
        "distance": round(distance, 6),
        "direction": direction,
    }


def ensure_camera() -> bpy.types.Object:
    """Return the scene camera, creating one if the .blend has none."""
    scene = bpy.context.scene
    if scene.camera is not None:
        return scene.camera
    cam_data = bpy.data.cameras.new("AwareCamera")
    camera = bpy.data.objects.new("AwareCamera", cam_data)
    scene.collection.objects.link(camera)
    scene.camera = camera
    return camera
```

- [ ] **Step 2: Verify the framing math against the fixture**

```bash
"$BLENDER" -b /tmp/fixture.blend --python-expr "
import sys; sys.path.insert(0,'20-agents/aeco/visualization/blender/scripts')
import bpy, _framing
bpy.context.scene.render.resolution_x = 960
bpy.context.scene.render.resolution_y = 540
cam = _framing.ensure_camera()
print('RECEIPT', _framing.frame_camera(cam, 'iso'))
# REQUIRED: frame_camera() writes .location/.rotation_euler directly, and in
# background mode Blender does NOT synchronously refresh camera.matrix_world --
# it updates on the next depsgraph evaluation. Without this line the check below
# reads a stale identity matrix and reports EVERY vertex outside the frustum.
bpy.context.view_layer.update()
# every mesh vertex must land inside the camera frustum
from bpy_extras.object_utils import world_to_camera_view
scene = bpy.context.scene
outside = 0
for o in bpy.data.objects:
    if o.type != 'MESH': continue
    for v in o.data.vertices:
        co = world_to_camera_view(scene, cam, o.matrix_world @ v.co)
        if not (0.0 <= co.x <= 1.0 and 0.0 <= co.y <= 1.0 and co.z > 0):
            outside += 1
print('OUTSIDE_FRUSTUM', outside)
"
```

Expected: a `RECEIPT` with `radius` ≈ 4.05 (the fixture's diagonal half-span) and **`OUTSIDE_FRUSTUM 0`**. A non-zero count means the fit is wrong — raise `margin` only after confirming the half-FOV derivation matches the sensor fit, since a margin bump hides a math bug rather than fixing it.

**The `matrix_world` staleness trap** (found while verifying this task — it cost real time, so it is written down): `frame_camera()` sets `.location` and `.rotation_euler`, but in background mode `camera.matrix_world` does not refresh until the next depsgraph evaluation. Anything reading `matrix_world` — including `world_to_camera_view` — sees an identity matrix until then, which reports *every* vertex outside the frustum and looks exactly like a broken fit.

Consequences for later tasks:
- **The render path is safe as-is.** `bpy.ops.render.render()` forces its own depsgraph evaluation, so pixels are always rasterized from the correct placement. Tasks 8 and 9 need no `view_layer.update()` call to render correctly.
- **Diagnostics are not safe.** If `render_still.py` or `render_turntable.py` ever log a camera world position into their receipt *before* rendering, that value will be stale. Log the values already in `frame_camera()`'s returned dict (`centre` / `radius` / `distance` / `direction`) — they are correct with no sync — or call `bpy.context.view_layer.update()` first.

- [ ] **Step 3: Commit**

```bash
git add 20-agents/aeco/visualization/blender/scripts/_framing.py
git commit -m "feat(blender): headless camera auto-framing on the bounding sphere"
```

---

## Task 7: Look presets

**Files:**
- Create: `20-agents/aeco/visualization/blender/scripts/_looks.py`
- Create: `20-agents/aeco/visualization/blender/scripts/scene_apply_look.py`

The design doc's look-mapping table, made executable. Assignment is by IFC class + material grade, with the `IfcStyledItem` colour as a tint fallback and clay as the never-fail default.

- [ ] **Step 1: Write the look mapper**

Create `20-agents/aeco/visualization/blender/scripts/_looks.py`:

```python
"""Semantic look assignment -- the design doc's mapping table, executable.

No human clicks materials in an unattended render, so the look is deduced from
what the IFC already states: the product class and its associated material grade.
Unrecognised input never fails the render; it falls through to clay.
"""

import bpy

import _ifc_import

# Material-grade prefixes by family. Matched case-insensitively against the
# IfcMaterial name, longest-prefix-first, so "A500-GR.B" hits STEEL via "A500".
GRADE_FAMILIES = {
    "steel": ("A992", "A500", "A36", "A572", "A53", "S355", "S275", "S235", "Q345"),
    "concrete": ("C20", "C25", "C30", "C35", "C40", "C50", "CONCRETE", "BETON"),
    "glass": ("GLASS", "GLAZING", "SZKLO"),
    "timber": ("TIMBER", "WOOD", "GL24", "GL28", "C24"),
}

# IFC class -> family, used when the material grade says nothing useful.
CLASS_FAMILIES = {
    "IfcColumn": "steel",
    "IfcBeam": "steel",
    "IfcMember": "steel",
    "IfcPlate": "steel",
    "IfcMechanicalFastener": "steel",
    "IfcSlab": "concrete",
    "IfcWall": "concrete",
    "IfcWallStandardCase": "concrete",
    "IfcFooting": "concrete",
    "IfcPile": "concrete",
    "IfcWindow": "glass",
    "IfcCurtainWall": "glass",
}

# (base_colour_rgba, metallic, roughness) per family, per preset.
PALETTES = {
    "clay": {
        "steel": ((0.62, 0.60, 0.58, 1.0), 0.0, 0.62),
        "concrete": ((0.66, 0.65, 0.63, 1.0), 0.0, 0.72),
        "glass": ((0.70, 0.72, 0.74, 1.0), 0.0, 0.55),
        "timber": ((0.68, 0.62, 0.54, 1.0), 0.0, 0.68),
        "default": ((0.64, 0.63, 0.61, 1.0), 0.0, 0.65),
    },
    "realistic": {
        "steel": ((0.42, 0.45, 0.49, 1.0), 0.85, 0.38),
        "concrete": ((0.60, 0.59, 0.56, 1.0), 0.0, 0.85),
        "glass": ((0.58, 0.70, 0.76, 0.28), 0.0, 0.08),
        "timber": ((0.52, 0.38, 0.24, 1.0), 0.0, 0.62),
        "default": ((0.55, 0.55, 0.55, 1.0), 0.1, 0.60),
    },
    "section-style": {
        "steel": ((0.20, 0.28, 0.42, 1.0), 0.0, 0.95),
        "concrete": ((0.78, 0.78, 0.76, 1.0), 0.0, 0.95),
        "glass": ((0.80, 0.88, 0.92, 0.45), 0.0, 0.30),
        "timber": ((0.72, 0.60, 0.44, 1.0), 0.0, 0.95),
        "default": ((0.70, 0.70, 0.70, 1.0), 0.0, 0.95),
    },
}

PRESETS = tuple(PALETTES)


def family_for(ifc_class: str, material: str) -> tuple[str, str]:
    """Resolve (family, reason) from an IFC class and material grade."""
    grade = (material or "").strip().upper()
    if grade:
        best_family = ""
        best_len = 0
        for family, prefixes in GRADE_FAMILIES.items():
            for prefix in prefixes:
                if grade.startswith(prefix) and len(prefix) > best_len:
                    best_family, best_len = family, len(prefix)
        if best_family:
            return best_family, f"grade:{material}"

    family = CLASS_FAMILIES.get(ifc_class or "")
    if family:
        return family, f"class:{ifc_class}"
    return "default", "fallback:clay"


def _material_for(preset: str, family: str) -> bpy.types.Material:
    """Fetch or build the shared Blender material for a (preset, family)."""
    name = f"AWARE_{preset}_{family}"
    existing = bpy.data.materials.get(name)
    if existing is not None:
        return existing

    colour, metallic, roughness = PALETTES[preset][family]
    material = bpy.data.materials.new(name)
    material.use_nodes = True
    bsdf = material.node_tree.nodes.get("Principled BSDF")
    if bsdf is not None:
        bsdf.inputs["Base Color"].default_value = colour
        bsdf.inputs["Metallic"].default_value = metallic
        bsdf.inputs["Roughness"].default_value = roughness
        if colour[3] < 1.0 and "Alpha" in bsdf.inputs:
            bsdf.inputs["Alpha"].default_value = colour[3]
            material.blend_method = "BLEND"
    return material


def apply_look(preset: str) -> dict:
    """Assign a material to every mesh object from its IFC semantics."""
    if preset not in PALETTES:
        raise ValueError(f"unknown preset `{preset}`; expected one of {sorted(PRESETS)}")

    assigned: dict[str, int] = {}
    reasons: dict[str, int] = {}

    for obj in bpy.data.objects:
        if obj.type != "MESH":
            continue
        ifc_class = obj.get(_ifc_import.PROP_CLASS, "")
        grade = obj.get(_ifc_import.PROP_MATERIAL, "")
        family, reason = family_for(ifc_class, grade)

        material = _material_for(preset, family)
        obj.data.materials.clear()
        obj.data.materials.append(material)

        assigned[family] = assigned.get(family, 0) + 1
        kind = reason.split(":", 1)[0]
        reasons[kind] = reasons.get(kind, 0) + 1

    return {
        "preset": preset,
        "assigned": dict(sorted(assigned.items())),
        "by-reason": dict(sorted(reasons.items())),
    }
```

- [ ] **Step 2: Write the `scene.apply-look` command entry**

Create `20-agents/aeco/visualization/blender/scripts/scene_apply_look.py`:

```python
"""`scene.apply-look` -- assign preset materials from IFC semantics.

Run: blender -b -P scene_apply_look.py -- '{"blend-path":"m.blend","preset":"realistic"}'
"""

import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))

import bpy  # noqa: E402

import _looks  # noqa: E402
import _result  # noqa: E402


def main(inputs: dict) -> dict:
    blend_path = str(_result.require(inputs, "blend-path"))
    preset = str(inputs.get("preset", "realistic"))
    out_path = str(inputs.get("output-path") or blend_path)

    if not os.path.exists(blend_path):
        raise _result.AwareBlenderError(
            _result.ERR_BLEND_UNREADABLE, f".blend not found: {blend_path}"
        )
    if preset not in _looks.PRESETS:
        raise _result.AwareBlenderError(
            _result.ERR_INVALID_INPUTS,
            f"unknown preset `{preset}`; expected one of {sorted(_looks.PRESETS)}",
        )

    bpy.ops.wm.open_mainfile(filepath=os.path.abspath(blend_path))
    receipt = _looks.apply_look(preset)
    bpy.ops.wm.save_as_mainfile(filepath=os.path.abspath(out_path))

    receipt["blend-path"] = os.path.abspath(out_path)
    receipt["path"] = receipt["blend-path"]
    return receipt


_result.run(main)
```

- [ ] **Step 3: Verify the mapping logic without Blender**

The `family_for` resolver is pure — test it directly:

```bash
python -c "
import sys, types
sys.path.insert(0, '20-agents/aeco/visualization/blender/scripts')
# _looks imports bpy at top level AND annotates a return as `bpy.types.Material`.
# Python evaluates that annotation at import time (the module has no
# `from __future__ import annotations`), so a bare ModuleType stub raises
# AttributeError before a single assertion runs. The stub needs `.types`.
bpy_stub = types.ModuleType('bpy')
bpy_stub.types = types.SimpleNamespace(Material=object)
sys.modules['bpy'] = bpy_stub
sys.modules['_ifc_import'] = types.ModuleType('_ifc_import')
import _looks
assert _looks.family_for('IfcBeam','A992')[0] == 'steel'
assert _looks.family_for('IfcColumn','A500-GR.B')[0] == 'steel'
assert _looks.family_for('IfcMember','A36')[0] == 'steel'
assert _looks.family_for('IfcSlab','C30/37')[0] == 'concrete'
assert _looks.family_for('IfcSlab','')[0] == 'concrete'          # class fallback
assert _looks.family_for('IfcBeam','')[0] == 'steel'             # class fallback
assert _looks.family_for('','')[0] == 'default'                  # clay, never fails
assert _looks.family_for('IfcWeirdThing','Unobtainium')[0] == 'default'
assert _looks.family_for('IfcBeam','A992')[1].startswith('grade:')
assert _looks.family_for('IfcBeam','')[1].startswith('class:')
print('OK look mapping')
"
```

Expected: `OK look mapping`.

- [ ] **Step 4: Apply to the fixture in Blender**

```bash
"$BLENDER" -b -P 20-agents/aeco/visualization/blender/scripts/scene_apply_look.py -- "{\"blend-path\":\"/tmp/fixture.blend\",\"preset\":\"realistic\"}"
```

Expected: `"assigned": {"steel": 9}` and `"by-reason": {"grade": 9}` — every fixture element resolved through its material grade, none falling back.

- [ ] **Step 5: Commit**

```bash
git add 20-agents/aeco/visualization/blender/scripts/_looks.py 20-agents/aeco/visualization/blender/scripts/scene_apply_look.py
git commit -m "feat(blender): semantic look presets keyed on IFC class and material grade"
```

---

## Task 8: `render.still`

**Files:**
- Create: `20-agents/aeco/visualization/blender/scripts/render_still.py`

**Substitute the Task 1 EEVEE identifier** for `EEVEE_ENGINE` before running.

- [ ] **Step 1: Write the command**

Create `20-agents/aeco/visualization/blender/scripts/render_still.py`:

```python
"""`render.still` -- a framed PNG of a staged .blend (or an IFC directly).

Run: blender -b -P render_still.py -- '{"blend-path":"m.blend","output-path":"o.png"}'
"""

import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))

import bpy  # noqa: E402

import _framing  # noqa: E402
import _ifc_import  # noqa: E402
import _looks  # noqa: E402
import _result  # noqa: E402

# Set from the Task 1 probe: Blender 4.2+ renamed EEVEE to BLENDER_EEVEE_NEXT.
# Resolved at runtime so one script works across the 4.x/5.x split.
def _eevee_engine() -> str:
    identifiers = {
        item.identifier
        for item in bpy.types.RenderSettings.bl_rna.properties["engine"].enum_items
    }
    for candidate in ("BLENDER_EEVEE_NEXT", "BLENDER_EEVEE"):
        if candidate in identifiers:
            return candidate
    raise _result.AwareBlenderError(
        _result.ERR_RENDER_FAILED,
        f"no EEVEE engine in this Blender; available: {sorted(identifiers)}",
    )


def setup_world(strength: float = 1.0) -> None:
    """A neutral grey studio world so nothing renders pitch black."""
    world = bpy.context.scene.world
    if world is None:
        world = bpy.data.worlds.new("AwareWorld")
        bpy.context.scene.world = world
    world.use_nodes = True
    background = world.node_tree.nodes.get("Background")
    if background is not None:
        background.inputs["Color"].default_value = (0.05, 0.06, 0.08, 1.0)
        background.inputs["Strength"].default_value = strength


def setup_key_light() -> None:
    """One sun, angled to match the default `iso` camera direction."""
    if any(obj.type == "LIGHT" for obj in bpy.data.objects):
        return
    light_data = bpy.data.lights.new("AwareSun", type="SUN")
    light_data.energy = 3.0
    light_data.angle = 0.15
    light = bpy.data.objects.new("AwareSun", light_data)
    light.rotation_euler = (0.9, 0.0, -0.8)
    bpy.context.scene.collection.objects.link(light)


def load_scene(inputs: dict) -> None:
    """Open the staged .blend, or import an IFC and apply a look on the fly."""
    blend_path = inputs.get("blend-path")
    ifc_path = inputs.get("ifc-path")
    if not blend_path and not ifc_path:
        raise _result.AwareBlenderError(
            _result.ERR_INVALID_INPUTS,
            "one of `blend-path` or `ifc-path` is required",
        )

    if blend_path:
        if not os.path.exists(str(blend_path)):
            raise _result.AwareBlenderError(
                _result.ERR_BLEND_UNREADABLE, f".blend not found: {blend_path}"
            )
        bpy.ops.wm.open_mainfile(filepath=os.path.abspath(str(blend_path)))
        return

    if not os.path.exists(str(ifc_path)):
        raise _result.AwareBlenderError(
            _result.ERR_IFC_UNREADABLE, f"IFC not found: {ifc_path}"
        )
    _ifc_import.clear_scene()
    _ifc_import.import_ifc(
        str(ifc_path), unit_scale=float(inputs.get("unit-scale", 1.0))
    )
    _looks.apply_look(str(inputs.get("preset", "realistic")))


def main(inputs: dict) -> dict:
    output_path = os.path.abspath(str(_result.require(inputs, "output-path")))
    quality = str(inputs.get("quality", "draft"))
    direction = str(inputs.get("direction", "iso"))
    width = int(inputs.get("width-pixels", 1920))
    height = int(inputs.get("height-pixels", 1080))
    samples = int(inputs.get("samples", 0))

    if quality not in ("draft", "production"):
        raise _result.AwareBlenderError(
            _result.ERR_INVALID_INPUTS,
            f"unknown quality `{quality}`; expected draft or production",
        )
    if width < 1 or height < 1:
        raise _result.AwareBlenderError(
            _result.ERR_INVALID_INPUTS, "width-pixels and height-pixels must be >= 1"
        )

    load_scene(inputs)

    scene = bpy.context.scene
    scene.render.resolution_x = width
    scene.render.resolution_y = height
    scene.render.resolution_percentage = 100
    scene.render.image_settings.file_format = "PNG"
    scene.render.filepath = output_path

    if quality == "production":
        scene.render.engine = "CYCLES"
        scene.cycles.samples = samples or 128
    else:
        scene.render.engine = _eevee_engine()
        if hasattr(scene, "eevee") and hasattr(scene.eevee, "taa_render_samples"):
            scene.eevee.taa_render_samples = samples or 32

    setup_world()
    setup_key_light()

    camera = _framing.ensure_camera()
    try:
        framing = _framing.frame_camera(camera, direction)
    except ValueError as exc:
        raise _result.AwareBlenderError(_result.ERR_RENDER_FAILED, str(exc)) from exc

    os.makedirs(os.path.dirname(output_path), exist_ok=True)
    bpy.ops.render.render(write_still=True)

    if not os.path.exists(output_path):
        raise _result.AwareBlenderError(
            _result.ERR_RENDER_FAILED,
            f"render completed but {output_path} was not written",
        )

    return {
        "path": output_path,
        "output-path": output_path,
        "size-bytes": os.path.getsize(output_path),
        "width-pixels": width,
        "height-pixels": height,
        "engine": scene.render.engine,
        "quality": quality,
        "framing": framing,
    }


_result.run(main)
```

- [ ] **Step 2: Render the fixture at low resolution**

```bash
"$BLENDER" -b -P 20-agents/aeco/visualization/blender/scripts/render_still.py -- "{\"blend-path\":\"/tmp/fixture.blend\",\"output-path\":\"/tmp/still.png\",\"quality\":\"draft\",\"width-pixels\":640,\"height-pixels\":360}"
```

Expected: framed payload with `"ok": true`, a non-zero `size-bytes`, and the resolved EEVEE identifier in `engine`.

- [ ] **Step 3: Assert the PNG is real, not a flat void**

This is the design doc's headless smoke assertion. A render that silently produced a uniform grey rectangle would pass an existence check, so check the pixel variety too:

```bash
"$BLENDER" -b --python-expr "
import bpy
img = bpy.data.images.load('/tmp/still.png')
w, h = img.size
px = list(img.pixels)
print('DIMENSIONS', w, h)
distinct = len({tuple(px[i:i+3]) for i in range(0, len(px), 4)})
print('DISTINCT_COLOURS', distinct)
assert (w, h) == (640, 360), f'unexpected dimensions {w}x{h}'
assert distinct > 16, f'render looks flat: only {distinct} distinct colours'
print('OK still render')
"
```

Expected: `DIMENSIONS 640 360`, a `DISTINCT_COLOURS` count in the hundreds or more, and `OK still render`. A count of 1–2 means the camera is pointed at nothing or the scene is unlit — check the framing receipt's `OUTSIDE_FRUSTUM` from Task 6 before touching light energy.

- [ ] **Step 4: Verify the IFC-direct path (no staged .blend)**

```bash
"$BLENDER" -b -P 20-agents/aeco/visualization/blender/scripts/render_still.py -- "{\"ifc-path\":\"/tmp/fixture.ifc\",\"output-path\":\"/tmp/direct.png\",\"quality\":\"draft\",\"width-pixels\":640,\"height-pixels\":360,\"preset\":\"clay\"}"
```

Expected: `"ok": true` — one command taking IFC straight to PNG, which is the app's happy path.

- [ ] **Step 5: Commit**

```bash
git add 20-agents/aeco/visualization/blender/scripts/render_still.py
git commit -m "feat(blender): render.still with EEVEE draft and Cycles production paths"
```

---

## Task 9: `render.turntable`

**Files:**
- Create: `20-agents/aeco/visualization/blender/scripts/render_turntable.py`

The camera is parented to an empty at the model centre and the empty spins a full 360°, so the framing computed once holds for every frame.

- [ ] **Step 1: Write the command**

Create `20-agents/aeco/visualization/blender/scripts/render_turntable.py`:

```python
"""`render.turntable` -- a 360-degree orbit MP4 around the model (EEVEE).

The camera is parented to an empty at the model centre and the EMPTY rotates, so
the framing solved once holds for every frame. Rotating the camera itself would
require re-solving the fit per frame.

Run: blender -b -P render_turntable.py -- '{"blend-path":"m.blend","output-path":"o.mp4"}'
"""

import math
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))

import bpy  # noqa: E402

import _framing  # noqa: E402
import _result  # noqa: E402
import render_still  # noqa: E402  - reuse load_scene / lighting / engine resolution


def _iter_fcurves(action):
    """Yield an action's fcurves across the pre-4.4 flat and 4.4+ layered shapes.

    Blender 4.4 replaced `action.fcurves` with layered actions; on 5.2 the old
    attribute is gone entirely, so reaching for it raises AttributeError.
    """
    flat = getattr(action, "fcurves", None)
    if flat is not None:
        yield from flat
        return
    for layer in getattr(action, "layers", ()):
        for strip in getattr(layer, "strips", ()):
            for channelbag in getattr(strip, "channelbags", ()):
                yield from getattr(channelbag, "fcurves", ())


def main(inputs: dict) -> dict:
    output_path = os.path.abspath(str(_result.require(inputs, "output-path")))
    duration = float(inputs.get("duration-seconds", 8))
    fps = int(inputs.get("fps", 30))
    width = int(inputs.get("width-pixels", 1920))
    height = int(inputs.get("height-pixels", 1080))
    direction = str(inputs.get("direction", "iso"))
    samples = int(inputs.get("samples", 0))

    if duration <= 0 or fps < 1:
        raise _result.AwareBlenderError(
            _result.ERR_INVALID_INPUTS,
            "duration-seconds must be > 0 and fps must be >= 1",
        )
    frame_count = max(2, int(round(duration * fps)))

    render_still.load_scene(inputs)

    scene = bpy.context.scene
    scene.render.resolution_x = width
    scene.render.resolution_y = height
    scene.render.resolution_percentage = 100
    scene.render.fps = fps
    scene.frame_start = 1
    scene.frame_end = frame_count

    scene.render.engine = render_still._eevee_engine()
    if hasattr(scene, "eevee") and hasattr(scene.eevee, "taa_render_samples"):
        scene.eevee.taa_render_samples = samples or 16

    # Blender 5.2 gates video containers behind a media_type switch that defaults
    # to IMAGE; without this, setting file_format = "FFMPEG" raises TypeError.
    # hasattr-guarded so the script still runs on 4.x, which has no media_type.
    if hasattr(scene.render.image_settings, "media_type"):
        scene.render.image_settings.media_type = "VIDEO"
    scene.render.image_settings.file_format = "FFMPEG"
    scene.render.ffmpeg.format = "MPEG4"
    scene.render.ffmpeg.codec = "H264"
    scene.render.ffmpeg.constant_rate_factor = "MEDIUM"
    scene.render.ffmpeg.ffmpeg_preset = "GOOD"
    scene.render.filepath = output_path

    render_still.setup_world()
    render_still.setup_key_light()

    camera = _framing.ensure_camera()
    try:
        framing = _framing.frame_camera(camera, direction)
    except ValueError as exc:
        raise _result.AwareBlenderError(_result.ERR_RENDER_FAILED, str(exc)) from exc

    # Pivot at the model centre; the camera rides it, so the fit never changes.
    pivot = bpy.data.objects.new("AwareTurntablePivot", None)
    pivot.location = framing["centre"]
    scene.collection.objects.link(pivot)

    # REQUIRED: frame_camera() wrote .location/.rotation_euler, but background-mode
    # Blender has not refreshed camera.matrix_world yet. Without this sync the copy
    # below captures a stale identity matrix and the re-parented camera lands at the
    # world origin -- yielding a video of nothing, or of the inside of a beam.
    bpy.context.view_layer.update()

    world_matrix = camera.matrix_world.copy()
    camera.parent = pivot
    camera.matrix_parent_inverse = pivot.matrix_world.inverted()
    camera.matrix_world = world_matrix

    pivot.rotation_euler = (0.0, 0.0, 0.0)
    pivot.keyframe_insert(data_path="rotation_euler", frame=1)
    pivot.rotation_euler = (0.0, 0.0, 2.0 * math.pi)
    pivot.keyframe_insert(data_path="rotation_euler", frame=frame_count)

    # Linear interpolation, or the orbit eases in and out and looks broken.
    # Blender 4.4+ replaced the flat `action.fcurves` with layered actions, so the
    # curves now hang off action.layers[].strips[].channelbags[]. _iter_fcurves()
    # handles both shapes -- a bare `action.fcurves` raises AttributeError on 5.2.
    for fcurve in _iter_fcurves(pivot.animation_data.action):
        for keyframe in fcurve.keyframe_points:
            keyframe.interpolation = "LINEAR"

    os.makedirs(os.path.dirname(output_path), exist_ok=True)
    bpy.ops.render.render(animation=True)

    # Blender may append a frame range to the container name; find what landed.
    written = output_path
    if not os.path.exists(written):
        directory = os.path.dirname(output_path)
        stem = os.path.splitext(os.path.basename(output_path))[0]
        candidates = [
            os.path.join(directory, name)
            for name in os.listdir(directory)
            if name.startswith(stem) and name.lower().endswith(".mp4")
        ]
        if not candidates:
            raise _result.AwareBlenderError(
                _result.ERR_RENDER_FAILED,
                f"animation rendered but no MP4 was written near {output_path}",
            )
        written = max(candidates, key=os.path.getmtime)

    return {
        "path": written,
        "output-path": written,
        "size-bytes": os.path.getsize(written),
        "frames": frame_count,
        "fps": fps,
        "duration-seconds": round(frame_count / fps, 3),
        "width-pixels": width,
        "height-pixels": height,
        "engine": scene.render.engine,
        "framing": framing,
    }
```

Then append the entry point:

```python
_result.run(main)
```

- [ ] **Step 2: Render a short turntable**

Keep it tiny — this is a correctness check, not a beauty pass:

```bash
"$BLENDER" -b -P 20-agents/aeco/visualization/blender/scripts/render_turntable.py -- "{\"blend-path\":\"/tmp/fixture.blend\",\"output-path\":\"/tmp/turn.mp4\",\"duration-seconds\":1,\"fps\":12,\"width-pixels\":480,\"height-pixels\":270}"
```

Expected: `"ok": true`, `"frames": 12`, and a `size-bytes` in the tens of kilobytes. A few hundred bytes means FFMPEG wrote a container with no frames.

- [ ] **Step 3: Verify the MP4 is a real video**

```bash
python -c "
import pathlib
p = pathlib.Path('/tmp/turn.mp4')
data = p.read_bytes()
assert p.stat().st_size > 10_000, f'suspiciously small: {p.stat().st_size} bytes'
assert data[4:8] == b'ftyp', 'not an ISO base-media (MP4) container'
print('OK turntable', p.stat().st_size, 'bytes')
"
```

Expected: `OK turntable <size> bytes`.

- [ ] **Step 4: Commit**

```bash
git add 20-agents/aeco/visualization/blender/scripts/render_turntable.py
git commit -m "feat(blender): render.turntable orbit MP4 via a keyframed pivot"
```

---

## Task 10: The smoke harness

**Files:**
- Create: `20-agents/aeco/visualization/blender/tests/run_smoke.py`

One command that runs the whole chain, so a future session on any machine can prove the agent still works without rediscovering the invocations. CI does not run this — CI has no Blender — so it must fail loudly and legibly when run by hand.

- [ ] **Step 1: Write the harness**

Create `20-agents/aeco/visualization/blender/tests/run_smoke.py`:

```python
"""Headless smoke test for the blender agent.

Chain: fixture scene JSON -> ifc.write -> IFC -> scene.import -> scene.info
-> scene.apply-look -> render.still -> render.turntable, asserting the design
doc's verification criteria at each hop.

Not run in CI (no Blender there). Run it by hand on a Blender-equipped machine:

    python tests/run_smoke.py --blender "/c/Program Files/Blender Foundation/Blender 5.2/blender.exe" \
                              --aware-bin ./cli/target/debug/aware.exe
"""

import argparse
import json
import shutil
import subprocess
import sys
import tempfile
from pathlib import Path

HERE = Path(__file__).resolve().parent
SCRIPTS = HERE.parent / "scripts"

sys.path.insert(0, str(SCRIPTS))
import _result  # noqa: E402

# The fixture's known inventory -- the design doc's look-mapping check.
EXPECTED_CLASSES = {"IfcBeam": 4, "IfcColumn": 4, "IfcMember": 1}
EXPECTED_MATERIALS = {"A36": 1, "A500-GR.B": 4, "A992": 4}


def run_command(blender: str, script: str, inputs: dict) -> dict:
    """Run one agent command headless and recover its framed payload."""
    proc = subprocess.run(
        [blender, "-b", "-P", str(SCRIPTS / script), "--", json.dumps(inputs)],
        capture_output=True,
        text=True,
        check=False,
    )
    out = proc.stdout
    if _result.RESULT_BEGIN not in out or _result.RESULT_END not in out:
        raise AssertionError(
            f"{script}: no framed result in output.\n"
            f"--- stdout tail ---\n{out[-2000:]}\n--- stderr tail ---\n{proc.stderr[-2000:]}"
        )
    body = out.split(_result.RESULT_BEGIN, 1)[1].split(_result.RESULT_END, 1)[0]
    return json.loads(body)


def expect_ok(label: str, payload: dict) -> dict:
    if not payload.get("ok"):
        raise AssertionError(f"{label} failed: {json.dumps(payload, indent=2)}")
    print(f"  ok  {label}")
    return payload


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--blender", required=True, help="path to the blender binary")
    parser.add_argument("--aware-bin", default="aware", help="path to the aware binary")
    parser.add_argument("--keep", action="store_true", help="keep the work directory")
    opts = parser.parse_args()

    if not Path(opts.blender).exists() and shutil.which(opts.blender) is None:
        print(f"SKIP: blender not found at {opts.blender}", file=sys.stderr)
        return 77  # conventional "skipped" exit code

    work = Path(tempfile.mkdtemp(prefix="aware-blender-smoke-"))
    print(f"work dir: {work}")

    try:
        # 1. fixture IFC through the real CLI
        sys.path.insert(0, str(HERE))
        import make_fixture

        ifc = work / "fixture.ifc"
        receipt = make_fixture.build_fixture(opts.aware_bin, ifc)
        assert len(receipt["emitted"]) == 9, receipt["emitted"]
        print("  ok  ifc.write (9 elements)")

        blend = work / "fixture.blend"

        # 2. import
        imported = expect_ok(
            "scene.import",
            run_command(
                opts.blender,
                "scene_import.py",
                {"ifc-path": str(ifc), "blend-path": str(blend)},
            ),
        )
        assert imported["imported"] == 9, imported
        assert imported["by-class"] == EXPECTED_CLASSES, imported["by-class"]
        assert imported["by-material"] == EXPECTED_MATERIALS, imported["by-material"]

        # 3. inventory
        info = expect_ok(
            "scene.info",
            run_command(opts.blender, "scene_info.py", {"blend-path": str(blend)}),
        )
        assert info["by-class"] == EXPECTED_CLASSES, info["by-class"]
        assert info["by-material"] == EXPECTED_MATERIALS, info["by-material"]

        # 4. look
        look = expect_ok(
            "scene.apply-look",
            run_command(
                opts.blender,
                "scene_apply_look.py",
                {"blend-path": str(blend), "preset": "realistic"},
            ),
        )
        assert look["assigned"] == {"steel": 9}, look["assigned"]

        # 5. still
        png = work / "still.png"
        still = expect_ok(
            "render.still",
            run_command(
                opts.blender,
                "render_still.py",
                {
                    "blend-path": str(blend),
                    "output-path": str(png),
                    "quality": "draft",
                    "width-pixels": 640,
                    "height-pixels": 360,
                },
            ),
        )
        assert png.exists() and png.stat().st_size > 5_000, still
        assert png.read_bytes()[:8] == b"\x89PNG\r\n\x1a\n", "not a PNG"

        # 6. turntable
        mp4 = work / "turn.mp4"
        turn = expect_ok(
            "render.turntable",
            run_command(
                opts.blender,
                "render_turntable.py",
                {
                    "blend-path": str(blend),
                    "output-path": str(mp4),
                    "duration-seconds": 1,
                    "fps": 12,
                    "width-pixels": 480,
                    "height-pixels": 270,
                },
            ),
        )
        written = Path(turn["path"])
        assert written.exists() and written.stat().st_size > 10_000, turn
        assert written.read_bytes()[4:8] == b"ftyp", "not an MP4 container"

        print("\nSMOKE PASS")
        return 0
    finally:
        if not opts.keep:
            shutil.rmtree(work, ignore_errors=True)


if __name__ == "__main__":
    sys.exit(main())
```

- [ ] **Step 2: Run the whole chain**

```bash
python 20-agents/aeco/visualization/blender/tests/run_smoke.py --blender "$BLENDER" --aware-bin ./cli/target/debug/aware.exe
```

Expected: six `ok` lines then `SMOKE PASS`, exit code 0.

- [ ] **Step 3: Commit**

```bash
git add 20-agents/aeco/visualization/blender/tests/run_smoke.py
git commit -m "test(blender): end-to-end headless smoke harness"
```

**Phase 1 is complete when this passes.** Do not start Task 11 before it does — the design doc puts all the real risk in these scripts, and the manifest is only a description of what they already do.

---

## Task 11: The manifest

**Files:**
- Create: `20-agents/aeco/visualization/blender/manifest.yaml`

Follows `twinmotion-prep/manifest.yaml`'s structure. The description states plainly what makes this agent different from its prep-only siblings.

- [ ] **Step 1: Write the manifest**

Create `20-agents/aeco/visualization/blender/manifest.yaml`:

```yaml
agent:        blender
version:      0.1.0
display-name: Blender
description: |
  The first visualization agent in the substrate that renders with NO human in
  the loop. IFC in, production stills (PNG) and turntable videos (MP4) out.

  Its siblings `enscape-prep` and `twinmotion-prep` are honest prep-only agents:
  Enscape and Twinmotion cannot render headlessly, so AWARE stages the scene and
  a human presses Render. Blender can — `blender -b -P script.py` runs import,
  materials, camera and render entirely unattended, with two production
  renderers in the box (EEVEE for drafts and turntables, Cycles for path-traced
  hero shots).

  Materials are assigned from what the IFC already states: product class plus
  associated material grade, with the IfcStyledItem colour as a tint fallback
  and clay as a never-fail default. That is what makes the chain autonomous —
  nobody is there to click a material.

  Generic by construction: it consumes any IFC — a scene written by the `ifc`
  agent, a Tekla export, a Revit export — identically.

stateful: false

vendor:   aware-aeco
license:  Apache-2.0
homepage: https://github.com/aware-aeco/aware/tree/main/20-agents/aeco/visualization/blender
keywords: [aware, blender, rendering, visualization, headless, ifc, eevee, cycles, turntable]

provenance:
  generated-by:      hand-curated
  generator-version: 0.100.0
  refined-by:        [pawellisowski]
  generated-at:      2026-07-22T00:00:00Z

requires:
  software:
    - blender@4.2.x|5.x
  filesystem:
    - read:  '*.ifc,*.blend'
    - write: '*.blend,*.png,*.mp4'

transport:
  cli:
    binary: aware-blender

commands:

  scene.import:
    lifecycle: single
    category: curated
    mode: write
    description: |
      Import an IFC into Blender and stage it as a `.blend`. Geometry comes from
      the `ifcopenshell.geom` iterator; IFC class, material, storey, GUID and
      name ride along on each object as custom properties, so later commands
      never reopen the IFC. An element that fails to tessellate is skipped and
      counted, never fatal — its GUID is reported in `skipped`.
    inputs:
      ifc-path: string
      blend-path: string
      unit-scale:
        type: number
        default: 1.0
        description: |
          Multiplier applied to incoming coordinates. Blender's scene unit is
          metres; leave at 1.0 when ifcopenshell already converts, or set 0.001
          for a source that hands back raw millimetres.
    outputs:
      type: single
      schema:
        blend-path:    string
        path:          string
        imported:      number
        skipped-count: number
        skipped:       array
        by-class:      object
        by-material:   object
        by-storey:     object

  scene.apply-look:
    lifecycle: single
    category: curated
    mode: write
    description: |
      Assign preset materials from IFC semantics. `clay` is neutral matte for
      form studies, `realistic` is PBR steel/concrete/glass/timber, and
      `section-style` is the flat drawing-like look. Family is resolved by
      material grade first (A992/A500/A36/S355/C30…), then by IFC class, then
      falls back to clay — an unrecognised element never fails the render.
    inputs:
      blend-path: string
      preset:
        type: enum
        values: [clay, realistic, section-style]
        default: realistic
      output-path:
        type: string
        description: Optional; defaults to overwriting `blend-path`.
    outputs:
      type: single
      schema:
        preset:     string
        assigned:   object
        by-reason:  object
        blend-path: string
        path:       string

  render.still:
    lifecycle: single
    category: curated
    mode: write
    description: |
      Render a camera-framed still. Takes either a staged `blend-path` or an
      `ifc-path` directly (importing and applying a look on the fly). `draft`
      uses EEVEE, `production` uses Cycles. The camera is fitted to the model's
      bounding sphere — there is no viewport in background mode to fit against.
    inputs:
      blend-path:
        type: string
        description: Staged .blend; mutually exclusive with `ifc-path`.
      ifc-path:
        type: string
        description: IFC to import on the fly; mutually exclusive with `blend-path`.
      output-path: string
      quality:
        type: enum
        values: [draft, production]
        default: draft
      direction:
        type: enum
        values: [iso, front, back, left, right, top]
        default: iso
      width-pixels:
        type: number
        default: 1920
      height-pixels:
        type: number
        default: 1080
      samples:
        type: number
        description: Render samples; 0 selects the per-engine default.
      preset:
        type: enum
        values: [clay, realistic, section-style]
        default: realistic
        description: Look applied when importing from `ifc-path`.
    outputs:
      type: single
      schema:
        path:          string
        output-path:   string
        size-bytes:    number
        width-pixels:  number
        height-pixels: number
        engine:        string
        quality:       string
        framing:       object

  render.turntable:
    lifecycle: single
    category: curated
    mode: write
    description: |
      Render a 360-degree orbit MP4 (EEVEE, H.264). The camera is parented to a
      pivot at the model centre and the pivot spins, so the framing solved once
      holds for every frame.
    inputs:
      blend-path: string
      ifc-path: string
      output-path: string
      duration-seconds:
        type: number
        default: 8
      fps:
        type: number
        default: 30
      direction:
        type: enum
        values: [iso, front, back, left, right, top]
        default: iso
      width-pixels:
        type: number
        default: 1920
      height-pixels:
        type: number
        default: 1080
      samples:
        type: number
      preset:
        type: enum
        values: [clay, realistic, section-style]
        default: realistic
    outputs:
      type: single
      schema:
        path:             string
        output-path:      string
        size-bytes:       number
        frames:           number
        fps:              number
        duration-seconds: number
        engine:           string
        framing:          object

  scene.info:
    lifecycle: single
    category: curated
    description: |
      Inventory a staged `.blend` (or an IFC directly) by IFC class, material and
      storey. Read-only — the verification and debugging surface for everything
      above.
    inputs:
      blend-path: string
      ifc-path: string
      unit-scale:
        type: number
        default: 1.0
    outputs:
      type: single
      schema:
        count:       number
        by-class:    object
        by-material: object
        by-storey:   object
        elements:    array

skills:
  - headless-rendering.md
  - ifc-import-ifcopenshell.md
  - look-presets.md
```

- [ ] **Step 2: Validate it through the CLI**

```bash
./cli/target/debug/aware.exe agent validate ./20-agents/aeco/visualization/blender
```

Expected: a pass. Fix any schema complaint before moving on — this is the contract the runtime enforces.

- [ ] **Step 3: Commit**

```bash
git add 20-agents/aeco/visualization/blender/manifest.yaml
git commit -m "feat(blender): agent manifest for the five v1 commands"
```

---

## Task 12: The skills

**Files:**
- Create: `20-agents/aeco/visualization/blender/skills/headless-rendering.md`
- Create: `20-agents/aeco/visualization/blender/skills/ifc-import-ifcopenshell.md`
- Create: `20-agents/aeco/visualization/blender/skills/look-presets.md`

- [ ] **Step 1: Route through skill-creator**

CLAUDE.md is explicit and has no exception for "just three short files":

> **Use skill-creator for all skill work.** All skill creation, modification, or porting routes through Anthropic's `skill-creator` skill (via the `Skill` tool). No exceptions for "quick edits" or "just porting."

Invoke `Skill(skill-creator)` and author the three files through it. See [`20-agents/_core/aware-skill-builder/`](../../../20-agents/_core/aware-skill-builder/) for the AWARE-specific pipeline that wraps it, and `20-agents/aeco/visualization/twinmotion-prep/skills/datasmith-bridge.md` for the house style — prose plus tables, a "what flows cleanly / what's lossy" honesty section, and a worked example.

- [ ] **Step 2: Content each skill must carry**

Do not invent this from scratch — it is what Tasks 1–10 actually established, and the probe doc has the measured values.

`headless-rendering.md`:
- Why `blender -b -P` beats every GUI-bound renderer for unattended work, and what it costs
- The sentinel result protocol and why stdout framing is necessary (Blender's log noise)
- The named error taxonomy from `_result.py` and what each one means for a caller
- That `bpy.ops.view3d.camera_to_view_selected()` does not work in background mode, and the bounding-sphere fit used instead
- EEVEE vs Cycles: when each is the right call, and the `BLENDER_EEVEE` / `BLENDER_EEVEE_NEXT` identifier split across 4.x and 5.x
- The measured Blender version and Python version from the Task 1 probe

`ifc-import-ifcopenshell.md`:
- The three options weighed and why raw `ifcopenshell` won (design doc §"IFC import implementation")
- That Bonsai is for humans: GUI-oriented, no confirmed clean story under `-b`
- The geom-iterator pattern, with the working code from `_ifc_import.py`
- The 0.7 / 0.8 settings API split and how the code handles both
- **The unit trap**: Blender's scene unit is metres; state the measured behaviour from Task 1 Step 5 and what `unit-scale` is for
- Skip-and-count: why one unparseable element must never cost the whole render
- IfcConvert preprocessing as the documented v2 escape hatch for very large models

`look-presets.md`:
- The design doc's mapping table as shipped in `_looks.py`
- Grade prefixes per family and the longest-prefix-wins rule (why `A500-GR.B` resolves through `A500`)
- The resolution order: grade → class → clay, and why the fallback must never fail
- How to extend `GRADE_FAMILIES` / `CLASS_FAMILIES` for a new vertical, with a worked example
- The three presets and what each is for

- [ ] **Step 3: Commit**

```bash
git add 20-agents/aeco/visualization/blender/skills/
git commit -m "docs(blender): agent skills for headless rendering, IFC import and look presets"
```

---

## Task 13: Register the agent

**Files:**
- Modify: `registry-index.json`
- Modify: `CLAUDE.md`, `README.md` (via the stats script — do not hand-edit the markers)

- [ ] **Step 1: Add the registry entry**

`registry-index.json` has an `agents` object keyed by agent id. Add, in the same shape as its visualization siblings:

```json
"blender": {
  "versions": {
    "0.1.0": {
      "tarball": "https://github.com/aware-aeco/aware/archive/refs/heads/main.tar.gz",
      "subdir": "aware-main/20-agents/aeco/visualization/blender"
    }
  }
}
```

Keep the file's existing key ordering convention; check how neighbouring entries are sorted before inserting.

- [ ] **Step 2: Verify the JSON still parses and the count moved**

```bash
python -c "
import json
d = json.load(open('registry-index.json'))
print('agents:', len(d['agents']))
assert 'blender' in d['agents'], 'blender missing from registry'
print('blender:', json.dumps(d['agents']['blender'], indent=1))
"
```

Expected: `agents: 77` (up from the 76 recorded in CLAUDE.md) and the entry echoed back.

- [ ] **Step 3: Sync the stats markers**

Never hand-edit the `<!--stat:...-->` markers — a stale marker turns the `stats` CI gate red on every subsequent PR:

```bash
python scripts/sync_stats.py --write
```

Expected: the script reports the keys it updated (`agents_total` 76→77, `agents_curated` 27→28).

- [ ] **Step 4: Confirm the gate is green**

```bash
python scripts/sync_stats.py
```

Expected: `sync_stats: all N managed stats current (11 keys).`

- [ ] **Step 5: Commit**

```bash
git add registry-index.json CLAUDE.md README.md
git commit -m "feat(registry): register the blender visualization agent"
```

---

## Task 14: The example app

**Files:**
- Create: `30-apps/_examples/model-to-renders.app`

The substrate's first fully autonomous visualization chain: IFC in, hero still plus turntable out, nobody pressing Render.

- [ ] **Step 1: Write the app**

Create `30-apps/_examples/model-to-renders.app`:

```yaml
app:           model-to-renders
version:       0.1.0
display-name:  Model to Renders
description: |
  IFC in, finished renders out — the substrate's first visualization chain that
  completes with no human in the loop.

  Stages the model once, applies materials from what the IFC already states
  (product class plus material grade), then renders a hero still and a 360
  turntable from the same staged scene. Every earlier visualization example
  stopped at "…and now a designer presses Render"; this one does not.

  Point it at any IFC: one written by the `ifc` agent from a takeoff, a Tekla
  export, a Revit export. The agent is generic by construction.

exposes-as-agent: false

requires:
  - blender@0.1.x

requires-permissions:
  filesystem:
    - read:  '{{ inputs.ifc-path }}'
    - write: '{{ inputs.output-dir }}'

layout: dag

nodes:
  - id: stage
    agent: blender
    command: scene.import
    inputs:
      ifc-path:   '{{ inputs.ifc-path }}'
      blend-path: '{{ inputs.output-dir }}/staged.blend'

  - id: look
    agent: blender
    command: scene.apply-look
    inputs:
      blend-path: '{{ stage.blend-path }}'
      preset:     '{{ inputs.preset }}'

  - id: hero
    agent: blender
    command: render.still
    inputs:
      blend-path:    '{{ look.blend-path }}'
      output-path:   '{{ inputs.output-dir }}/hero.png'
      quality:       '{{ inputs.quality }}'
      direction:     iso
      width-pixels:  1920
      height-pixels: 1080

  - id: turntable
    agent: blender
    command: render.turntable
    inputs:
      blend-path:       '{{ look.blend-path }}'
      output-path:      '{{ inputs.output-dir }}/turntable.mp4'
      duration-seconds: 8
      fps:              30
      width-pixels:     1920
      height-pixels:    1080

  - id: inventory
    agent: blender
    command: scene.info
    inputs:
      blend-path: '{{ look.blend-path }}'

connections:
  - { from: stage, to: look }
  - { from: look,  to: hero }
  - { from: look,  to: turntable }
  - { from: look,  to: inventory }

inputs:
  ifc-path:
    type: string
    description: The IFC to render. Any producer — this agent is generic.
  output-dir:
    type: string
    description: Directory for the staged .blend, the still and the video.
  preset:
    type: enum
    values: [clay, realistic, section-style]
    default: realistic
  quality:
    type: enum
    values: [draft, production]
    default: draft
    description: draft renders the still in EEVEE; production uses Cycles.
```

**Before committing, check the `inputs:` block placement and syntax against a sibling** — `30-apps/_examples/designer-monday-shots.app` references `{{ inputs.* }}` throughout, so read how it declares them and match that exactly rather than trusting the shape above.

- [ ] **Step 2: Validate through the CLI**

```bash
./cli/target/debug/aware.exe app validate ./30-apps/_examples/model-to-renders.app
```

Expected: a pass. Fix any complaint before committing.

- [ ] **Step 3: Commit**

```bash
git add 30-apps/_examples/model-to-renders.app
git commit -m "feat(apps): model-to-renders — the first unattended visualization chain"
```

---

## Task 15: Final verification and PR

- [ ] **Step 1: Re-run the full smoke chain**

```bash
python 20-agents/aeco/visualization/blender/tests/run_smoke.py --blender "$BLENDER" --aware-bin ./cli/target/debug/aware.exe
```

Expected: `SMOKE PASS`.

- [ ] **Step 2: Run the repo's gates**

No Rust changed in this plan, but the gates must be green on the branch regardless:

```bash
cd cli && cargo fmt --all -- --check && cargo clippy --all-targets -- -D warnings && cargo test
```

Expected: all three clean. If `cargo test` fails on something this plan did not touch, investigate before assuming it is unrelated — a new agent directory can affect catalog/registry tests.

- [ ] **Step 3: Verify the agent installs and describes like any other**

```bash
./cli/target/debug/aware.exe agent install ./20-agents/aeco/visualization/blender
./cli/target/debug/aware.exe agent describe blender
```

Expected: `installed blender`, then a describe listing all five commands. This is the design doc's "runnable via the `aware` CLI per the commands table" criterion.

- [ ] **Step 4: Clean up session temp files**

```bash
git status --short
```

Expected: no `tmpclaude-*` files, no stray `.ifc` / `.blend` / `.png` / `.mp4` in the working tree. CLAUDE.md requires this before commit.

- [ ] **Step 5: Codex review**

```bash
codex exec review --base main
```

Address every finding, or justify in the PR why not. Fall back to `pr-review-toolkit:code-reviewer` **only** if Codex is genuinely unavailable (rate-limited, errored, not installed) — and re-check Codex next time rather than coasting on the fallback.

- [ ] **Step 6: Open the PR**

Pushing needs explicit approval per CLAUDE.md — ask before running this.

```bash
git push -u origin claude/session-63df38
```

Then open the PR against `main`, summarising: the agent, its five commands, the verified smoke chain, and the design decisions carried over (IFC-only input, raw ifcopenshell over Bonsai).

---

## Self-review against the design doc

| Design doc requirement | Where it lands |
|---|---|
| Agent at `20-agents/aeco/visualization/blender/` | Tasks 11–13 |
| `scene.import` | Task 4 |
| `scene.apply-look` with clay / realistic / section-style | Task 7 |
| `render.still` draft EEVEE / production Cycles | Task 8 |
| `render.turntable` MP4 | Task 9 |
| `scene.info` | Task 5 |
| Look mapping by class + grade, colour fallback, clay default | Task 7 (`_looks.py`) |
| Raw `ifcopenshell`, not Bonsai | Task 4 |
| `requires: blender@4.2+|5.x` + ifcopenshell install step | Tasks 1, 11 |
| Manifest follows `twinmotion-prep` | Task 11 |
| Skills: headless-rendering, ifc-import-ifcopenshell, look-presets | Task 12 |
| Registered in `registry-index.json`, stats bumped | Task 13 |
| Example app `model-to-renders.app` | Task 14 |
| Fixture: scene JSON run through `ifc.write` at test time, no binary committed | Task 2 |
| Smoke test: import → still → PNG exists, dimensions, not one flat colour | Tasks 8, 10 |
| `scene.info` matches the fixture's known inventory | Tasks 5, 10 |
| Named error: ifcopenshell missing + install one-liner | Task 3 (`ERR_IFCOPENSHELL_MISSING`, `IFCOPENSHELL_HINT`) |
| Named error: unparseable element skipped + counted, GUIDs reported | Task 4 (`skipped`) |
| Named error: missing Blender binary | Transport-level — see gap below |
| Render timeout, configurable, kill the process | Transport-level — see gap below |

### The transport gap — verified, and it is real

**Two design-doc requirements live outside the `bpy` scripts**: the *missing-Blender-binary* named error and the *render timeout*. Both belong to whatever shells out to `blender`, and by definition a `bpy` script is already inside a running Blender. The design doc names that thing in one clause — "Transport: `aware-blender` CLI that shells out to `blender -b -P …`" — and does not cost it.

What the runtime actually requires of such a binary, read from source rather than assumed:

| Fact | Evidence |
|---|---|
| The transport protocol is `<binary> <command> --json-stdin`, JSON in on stdin, JSON out on stdout | [`invoker.rs:277-283`](../../../cli/src/runtime/invoker.rs) |
| Binary resolution order: `~/.aware/bridges/` → bundled sibling (allowlist) → bare name on PATH | [`invoker.rs:149-161`](../../../cli/src/runtime/invoker.rs) |
| The bundled-sibling allowlist is explicit and currently steel-detailer only | `BUNDLED_TRANSPORTS`, [`invoker.rs:173-177`](../../../cli/src/runtime/invoker.rs) |
| Host bridges are a registered, downloadable set (`aware sidecar install <id>`) | `BRIDGES`, [`sidecar.rs:36-74`](../../../cli/src/commands/sidecar.rs) |
| A non-.NET bridge is already precedent | `aware-connection-reader` — "Node + web-ifc WASM" — in that same `BRIDGES` list |

So `aware-blender` is a **real, separate deliverable**: a bridge binary registered in `BRIDGES`, published as a release asset, speaking the `--json-stdin` protocol, locating Blender, running the scripts, parsing the sentinel payload, and owning both the missing-binary error and the timeout.

**Do not let this leak into Task 11 silently.** Two options, and they are not equivalent:

- **A — ship `aware-blender` as a host bridge** (register in `BRIDGES`, add release staging, implement the protocol). Architecturally correct: Blender is a vendor product with a host binary, which is exactly what bridges are for. Costs a new binary in the release pipeline.
- **B — add a `builtin` transport handler** in the Rust CLI (`("blender", "scene.import") => …` alongside `ifc.write`). No new binary, no release change, and the timeout lands naturally in Rust. But every current builtin (`ui`, `viewer-3d`, `ifc`, `file`, `vision`) is host-free pure computation under `_core/`; a builtin that spawns a vendor product would be the first of its kind.

This plan's Tasks 1–10 are unaffected either way — the `bpy` scripts and their contract are identical under both. **Raise the choice with the user before Task 11**, since it decides the manifest's `transport:` block and whether the release pipeline changes.
