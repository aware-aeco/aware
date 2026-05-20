# pythonpart.execute

Instantiate a **PythonPart** (`.pyp` definition + `.py` script) at a placement `{x, y, z, rotation-deg}` in the active document, overriding named parameters. Returns whether it was placed, the created element id, and any warnings — the "drop a parametric component into the model" primitive.

**WRITE-mode.**

## Inputs

| Field | Type | Required | Description |
|---|---|---|---|
| `pyp-path` | string | yes | Absolute path to the `.pyp` definition (its paired `.py` sits alongside). |
| `placement` | object | no | `{ x, y, z, rotation-deg }` in document units (millimetres). |
| `parameters` | object | no | Map of parameter-name → value, overriding the `.pyp` defaults. |

## Output

```yaml
placed:     true
element-id: "a83f…-uuid"
warnings:   []
```

## Implementation (Allplan Python API)

Allplan runs PythonParts **in-process**. A `.pyp` is XML (palette + parameter declarations + which `.py` to run); the framework populates a `BuildingElement` from the palette and calls the script's entry point:

```python
def create_element(build_ele, doc):           # build_ele = BuildingElement, doc = DocumentAdapter
    ...
    return CreateElementResult(model_elements)
```

1. **Apply parameter overrides** onto the `BuildingElement` before building. Each declared parameter is reachable as `build_ele.<ParamName>.value`:

   ```python
   for name, val in parameters.items():
       getattr(build_ele, name).value = val      # only params declared in the .pyp exist
   ```

2. **Build the placement transform.** Rotation is about an axis by an `Angle` (radians internally — always construct from degrees):

   ```python
   import NemAll_Python_Geometry as AllplanGeo
   mat = AllplanGeo.Matrix3D()
   z_axis = AllplanGeo.Line3D(AllplanGeo.Point3D(0,0,0), AllplanGeo.Point3D(0,0,1))
   mat.SetRotation(z_axis, AllplanGeo.Angle.FromDeg(placement["rotation-deg"]))
   mat.SetTranslation(AllplanGeo.Vector3D(placement["x"], placement["y"], placement["z"]))
   ```

3. **Wrap + place.** Either transform raw geometry into a `ModelEleList` (`.append_geometry_3d(...)`) or, for a full container, `PythonPartUtil.create_pythonpart(build_ele, …)` with the placement matrix, then `return CreateElementResult(...)`. The exact `create_pythonpart` placement keyword and the fields `CreateElementResult` surfaces are **not pinned in the current docs — confirm against a `PythonPartUtil` example** before depending on a specific `element-id`/`warnings` shape.

## Gotchas

- **Angles are radians internally.** Build every rotation with `AllplanGeo.Angle.FromDeg(deg)`. Passing degrees where radians are expected is the classic silent-misplacement bug.
- **Coordinates are millimetres in model space, with a possible project offset.** `AllplanGlobalSettings.GetOffsetPoint()` returns the active project offset; an absolute `{x,y,z}` may need it applied. Don't assume `{0,0,0}` is the project origin.
- **`AxisPlacement3D` X and Z vectors must be orthogonal** or the placement is silently invalid (relevant if you use `AxisPlacement3D` instead of a `Matrix3D`).
- **Parameter names must match the `.pyp` exactly.** The `BuildingElement` only exposes parameters declared in the `.pyp`; overriding an undeclared name is a no-op (a likely source of `warnings`).
- **Standard PythonParts can't reference picked elements.** If the placement must snap to existing geometry, the `.pyp` has to be an interactor — a plain "execute at coordinates" runs as a standard part.

Sources: [PythonPart manual](https://pythonparts.allplan.com/2025/manual/features/pythonpart/) · [Matrix3D](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Geometry/Matrix3D/) · [Angle](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Geometry/Angle/) · [AxisPlacement3D](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Geometry/AxisPlacement3D/) · [PythonPartsExamples (`Cuboid.py`)](https://github.com/NemetschekAllplan/PythonPartsExamples)

## See also

- [pythonpartutil](../skills/pythonpartutil.md) — the container-creation helper
- [buildingelement](../skills/buildingelement.md) — how `.pyp` parameters surface as `build_ele.<param>.value`
- [createelementresult](../skills/createelementresult.md) — the value `create_element` returns
