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
