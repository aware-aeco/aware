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

    The user-site path insert is NOT optional. Blender's bundled Python runs with
    `site.ENABLE_USER_SITE = False` and ignores PYTHONPATH, while its own
    site-packages under Program Files is not writable -- so `pip install
    ifcopenshell` silently falls back to a --user install that nothing ever adds
    to sys.path. Without this, a correctly installed ifcopenshell still raises
    ImportError on every run.
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
    ifcopenshell already converts, so 1.0 is correct for normal input.

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
