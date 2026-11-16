//! The canonical scene element `{kind:"extrusion"}`: planar ring sets extruded along a frame
//! normal, with an optional building class (`wall`, `slab`, `footing`).
//!
//! One parser shared by every sink that reads it (`ifc.write`, `viewer-3d`), so an element one sink
//! accepts is an element the other accepts, and an invalid one is refused with the same reason
//! everywhere. Errors are data: [`parse_extrusion`] returns the exact rule that failed, starting
//! with the field it concerns, and never coerces, repairs or guesses a shape.
//!
//! Contract (world millimetres; areas m², volumes m³, weights kg):
//!
//! * `id`: required and unique in the scene (checked by each sink with its other records). It is
//!   the stable reference target a later record may name (a base plate bearing on a footing, a
//!   member connected to a wall); this module does not resolve references.
//! * `frame {origin, uDir, vDir, normal}`: finite; `uDir`, `vDir`, `normal` unit length and
//!   mutually orthogonal to [`FRAME_TOLERANCE`], right-handed (`normal = uDir × vDir`).
//! * `items`: 1..[`MAX_ITEMS`] solids. Each is the ring set `outline` (counter-clockwise in
//!   (u, v)) less optional `holes` (clockwise), from the plane `fromMm` along `+normal` by
//!   `depthMm` at least [`TOLERANCE`] (0.00001 mm, so a written depth is never rounded to zero). Ring rules are `section_outline`'s, with at most [`MAX_HOLES`] holes and
//!   [`MAX_POINTS_PER_RING`] points in any one ring, each coordinate within 1,000,000 mm of the
//!   frame origin.
//! * `class`: optional `wall` | `slab` | `footing`; absent is a generic shape.
//! * `predefinedType`: optional, one of the class's IFC4 enumeration (the proxy's when no class);
//!   `USERDEFINED` requires `objectType`.
//! * `axis {u0, u1}`: walls and footings only, `u0 < u1`, the centreline along `u` at `v = 0`.
//! * `loadBearing`: optional boolean; needs a class.
//! * `material {layers:[{name, thicknessMm, grade?}]}`: optional; an ordered list of layers, of which
//!   exactly ONE is accepted today (`name` and `grade` IFC labels, `thicknessMm` finite and > 0, not
//!   compared with the element). Unknown keys are refused at every level, so a second layer or a new
//!   key is an additive change, never one an older reader drops.
//! * `quantities`: optional, finite and non-negative, only the keys the class's IFC4 base quantity
//!   set has (see [`quantity_keys`]); needs a class.
//! * `meta.description` / `meta.properties`: the member label channel, validated by each sink.

use serde_json::{Map, Value};

use super::geom::{Vec3, cross3, dot3, length3};
use super::section_outline::{
    MAX_COORDINATE, OutlineSection, RingLimits, TOLERANCE, Winding, parse_rings,
};

/// Solids per element. A banded wall (FloLess S3 §3.3) needs at most five.
pub const MAX_ITEMS: usize = 8;
/// Holes per item.
pub const MAX_HOLES: usize = 64;
/// Vertices in any one ring.
pub const MAX_POINTS_PER_RING: usize = 512;
/// Unit-length and orthogonality tolerance of the frame axes.
pub const FRAME_TOLERANCE: f64 = 1.0e-6;
/// `IfcLabel` is `STRING(255)`.
const LABEL_MAX_BYTES: usize = 255;

const ITEM_LIMITS: RingLimits = RingLimits {
    max_holes: MAX_HOLES,
    max_points_total: None,
    max_points_per_ring: Some(MAX_POINTS_PER_RING),
    max_coordinate: MAX_COORDINATE,
    origin_name: "the frame origin",
    subject: "item",
    winding: Winding::Require,
};

/// The building class of an extrusion, which decides its IFC entity.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExtrusionClass {
    Wall,
    Slab,
    Footing,
}

/// The IFC4 `PredefinedType` enumeration of a class (`None` is `IfcBuildingElementProxyTypeEnum`).
pub fn predefined_types(class: Option<ExtrusionClass>) -> &'static [&'static str] {
    match class {
        Some(ExtrusionClass::Wall) => &[
            "MOVABLE",
            "PARAPET",
            "PARTITIONING",
            "PLUMBINGWALL",
            "SHEAR",
            "SOLIDWALL",
            "STANDARD",
            "POLYGONAL",
            "ELEMENTEDWALL",
            "USERDEFINED",
            "NOTDEFINED",
        ],
        Some(ExtrusionClass::Slab) => &[
            "FLOOR",
            "ROOF",
            "LANDING",
            "BASESLAB",
            "USERDEFINED",
            "NOTDEFINED",
        ],
        Some(ExtrusionClass::Footing) => &[
            "CAISSON_FOUNDATION",
            "FOOTING_BEAM",
            "PAD_FOOTING",
            "PILE_CAP",
            "STRIP_FOOTING",
            "USERDEFINED",
            "NOTDEFINED",
        ],
        None => &[
            "COMPLEX",
            "ELEMENT",
            "PARTIAL",
            "PROVISIONFORVOID",
            "PROVISIONFORSPACE",
            "USERDEFINED",
            "NOTDEFINED",
        ],
    }
}

/// The measure an IFC base quantity carries, which decides its entity and unit.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuantityMeasure {
    /// `IfcQuantityLength`, millimetres (the file's length unit).
    LengthMm,
    /// `IfcQuantityArea`, square metres (the file's area unit).
    AreaM2,
    /// `IfcQuantityVolume`, cubic metres (the file's volume unit).
    VolumeM3,
    /// `IfcQuantityWeight`, kilograms (an explicit `IfcSIUnit` on the quantity).
    WeightKg,
}

/// The scene `quantities` keys a class accepts, as `(scene key, IFC quantity name, measure)` in the
/// order the IFC base quantity set lists them. A generic shape has none.
pub fn quantity_keys(
    class: Option<ExtrusionClass>,
) -> &'static [(&'static str, &'static str, QuantityMeasure)] {
    use QuantityMeasure::*;
    match class {
        Some(ExtrusionClass::Wall) => &[
            ("lengthMm", "Length", LengthMm),
            ("widthMm", "Width", LengthMm),
            ("heightMm", "Height", LengthMm),
            ("grossSideAreaM2", "GrossSideArea", AreaM2),
            ("netSideAreaM2", "NetSideArea", AreaM2),
            ("grossVolumeM3", "GrossVolume", VolumeM3),
            ("netVolumeM3", "NetVolume", VolumeM3),
            ("grossWeightKg", "GrossWeight", WeightKg),
            ("netWeightKg", "NetWeight", WeightKg),
        ],
        Some(ExtrusionClass::Slab) => &[
            ("widthMm", "Width", LengthMm),
            ("perimeterMm", "Perimeter", LengthMm),
            ("grossAreaM2", "GrossArea", AreaM2),
            ("netAreaM2", "NetArea", AreaM2),
            ("grossVolumeM3", "GrossVolume", VolumeM3),
            ("netVolumeM3", "NetVolume", VolumeM3),
            ("grossWeightKg", "GrossWeight", WeightKg),
            ("netWeightKg", "NetWeight", WeightKg),
        ],
        Some(ExtrusionClass::Footing) => &[
            ("lengthMm", "Length", LengthMm),
            ("widthMm", "Width", LengthMm),
            ("heightMm", "Height", LengthMm),
            ("crossSectionAreaM2", "CrossSectionArea", AreaM2),
            ("outerSurfaceAreaM2", "OuterSurfaceArea", AreaM2),
            ("grossSurfaceAreaM2", "GrossSurfaceArea", AreaM2),
            ("grossVolumeM3", "GrossVolume", VolumeM3),
            ("netVolumeM3", "NetVolume", VolumeM3),
            ("grossWeightKg", "GrossWeight", WeightKg),
            ("netWeightKg", "NetWeight", WeightKg),
        ],
        None => &[],
    }
}

/// The IFC4 base quantity set of a class.
pub fn quantity_set_name(class: ExtrusionClass) -> &'static str {
    match class {
        ExtrusionClass::Wall => "Qto_WallBaseQuantities",
        ExtrusionClass::Slab => "Qto_SlabBaseQuantities",
        ExtrusionClass::Footing => "Qto_FootingBaseQuantities",
    }
}

/// The IFC4 common property set that carries `LoadBearing` for a class.
pub fn common_property_set_name(class: ExtrusionClass) -> &'static str {
    match class {
        ExtrusionClass::Wall => "Pset_WallCommon",
        ExtrusionClass::Slab => "Pset_SlabCommon",
        ExtrusionClass::Footing => "Pset_FootingCommon",
    }
}

/// A validated frame: unit, orthogonal, right-handed.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ExtrusionFrame {
    pub origin: Vec3,
    pub u: Vec3,
    pub v: Vec3,
    pub normal: Vec3,
}

/// One validated solid: its rings (outline counter-clockwise, holes clockwise, as sent) from
/// `from_mm` along the normal by `depth_mm`.
#[derive(Debug, Clone, PartialEq)]
pub struct ExtrusionItem {
    pub rings: OutlineSection,
    pub from_mm: f64,
    pub depth_mm: f64,
}

#[cfg(test)]
impl ExtrusionItem {
    /// The solid's volume in cubic millimetres (net area × depth).
    pub fn volume_mm3(&self) -> f64 {
        super::section_outline::net_area_mm2(&self.rings) * self.depth_mm
    }
}

/// Layers a material may have today. Several layers (an `IfcMaterialLayerSet`) are a later, additive
/// step; until then a second layer is refused with its rule, never dropped.
pub const MAX_MATERIAL_LAYERS: usize = 1;

/// One layer of an extrusion's material.
#[derive(Debug, Clone, PartialEq)]
pub struct MaterialLayer {
    pub name: String,
    pub thickness_mm: f64,
    /// A catalogue grade (for example a host's material catalogue entry); carried, not written to IFC.
    pub grade: Option<String>,
}

/// A validated `extrusion` element.
#[derive(Debug, Clone, PartialEq)]
pub struct Extrusion {
    pub frame: ExtrusionFrame,
    pub items: Vec<ExtrusionItem>,
    pub class: Option<ExtrusionClass>,
    pub predefined_type: Option<String>,
    pub object_type: Option<String>,
    pub axis: Option<(f64, f64)>,
    pub load_bearing: Option<bool>,
    /// The material's layers, outermost first. Exactly one today (see [`MAX_MATERIAL_LAYERS`]).
    pub material: Option<Vec<MaterialLayer>>,
    /// `(IFC quantity name, measure, value)` in the base quantity set's order.
    pub quantities: Vec<(&'static str, QuantityMeasure, f64)>,
}

#[cfg(test)]
impl Extrusion {
    /// The total solid volume in cubic metres (the items are taken as disjoint).
    pub fn volume_m3(&self) -> f64 {
        self.items
            .iter()
            .map(ExtrusionItem::volume_mm3)
            .sum::<f64>()
            / 1.0e9
    }
}

/// Whether an item's rings are still a valid ring set once moved by `(-a0, -b0)` and rounded to
/// float32, as the viewer draws them: `Err` names the rule the rounded rings break (a hole now
/// touching its outline, a sliver gone to a line), or says the net area changed by more than 0.1%.
/// The exact rings were already validated; this only asks whether the display can hold them.
pub fn display_rings_problem(item: &ExtrusionItem, a0: f64, b0: f64) -> Result<(), String> {
    let round = |ring: &[[f64; 2]]| -> Vec<[f64; 2]> {
        ring.iter()
            .map(|p| [f64::from((p[0] - a0) as f32), f64::from((p[1] - b0) as f32)])
            .collect()
    };
    let outer = round(&item.rings.outer);
    let holes: Vec<Vec<[f64; 2]>> = item.rings.holes.iter().map(|h| round(h)).collect();
    let shown = parse_rings(
        &serde_json::json!(outer),
        (!holes.is_empty())
            .then(|| serde_json::json!(holes))
            .as_ref(),
        "outline",
        "holes",
        &ITEM_LIMITS,
    )?;
    let exact = super::section_outline::net_area_mm2(&item.rings);
    let drawn = super::section_outline::net_area_mm2(&shown);
    if (drawn - exact).abs() > 1.0e-3 * exact {
        return Err(format!(
            "its area would be drawn as {drawn} mm² instead of {exact} mm²"
        ));
    }
    Ok(())
}

/// `material: {layers:[{name, thicknessMm, grade?}]}`, exactly one layer today.
fn parse_material(value: &Value) -> Result<Vec<MaterialLayer>, String> {
    let material = value
        .as_object()
        .ok_or_else(|| "material must be an object {layers}".to_string())?;
    if let Some(extra) = material.keys().find(|key| *key != "layers") {
        return Err(format!(
            "material has an unknown field `{extra}`; only `layers` is defined"
        ));
    }
    let layers = material
        .get("layers")
        .ok_or_else(|| "material.layers is required".to_string())?
        .as_array()
        .ok_or_else(|| "material.layers must be an array".to_string())?;
    if layers.len() != MAX_MATERIAL_LAYERS {
        return Err(format!(
            "material.layers must have exactly {MAX_MATERIAL_LAYERS} layer (got {})",
            layers.len()
        ));
    }
    let mut out = Vec::with_capacity(layers.len());
    for (index, layer) in layers.iter().enumerate() {
        let path = format!("material.layers[{index}]");
        let layer = layer
            .as_object()
            .ok_or_else(|| format!("{path} must be an object {{name, thicknessMm, grade?}}"))?;
        if let Some(extra) = layer
            .keys()
            .find(|key| !matches!(key.as_str(), "name" | "thicknessMm" | "grade"))
        {
            return Err(format!(
                "{path} has an unknown field `{extra}`; only `name`, `thicknessMm` and `grade` are defined"
            ));
        }
        let name = layer
            .get("name")
            .ok_or_else(|| format!("{path}.name is required"))?;
        let name = label_text(name, &format!("{path}.name"))?;
        if name.trim().is_empty() {
            // `ifc.write` names the IfcMaterial by the trimmed name; a blank one would drop the
            // association silently, so it is refused here instead.
            return Err(format!("{path}.name must not be blank"));
        }
        let thickness_mm = finite(layer.get("thicknessMm"))
            .ok_or_else(|| format!("{path}.thicknessMm must be a finite number"))?;
        if thickness_mm <= 0.0 {
            return Err(format!(
                "{path}.thicknessMm must be greater than zero (got {})",
                short(thickness_mm)
            ));
        }
        let grade = match layer.get("grade") {
            None | Some(Value::Null) => None,
            Some(grade) => {
                let grade = label_text(grade, &format!("{path}.grade"))?;
                if grade.trim().is_empty() {
                    return Err(format!("{path}.grade must not be blank"));
                }
                Some(grade)
            }
        };
        out.push(MaterialLayer {
            name,
            thickness_mm,
            grade,
        });
    }
    Ok(out)
}

fn finite(value: Option<&Value>) -> Option<f64> {
    value
        .and_then(Value::as_f64)
        .filter(|number| number.is_finite())
}

fn vec3_at(frame: &Map<String, Value>, key: &str) -> Result<Vec3, String> {
    let values = frame
        .get(key)
        .and_then(Value::as_array)
        .filter(|values| values.len() == 3)
        .ok_or_else(|| format!("frame.{key} must be an array of 3 finite numbers"))?;
    let mut out = [0.0; 3];
    for (index, value) in values.iter().enumerate() {
        out[index] = finite(Some(value))
            .ok_or_else(|| format!("frame.{key} must be an array of 3 finite numbers"))?;
    }
    Ok(out)
}

/// The element's IFC `Name` is the first string of `meta.label`, `name` and `id` (as `ifc.write`
/// resolves every element's name), and it must fit an `IfcLabel`.
fn check_name(element: &Map<String, Value>) -> Result<(), String> {
    let meta_label = element
        .get("meta")
        .and_then(Value::as_object)
        .and_then(|meta| meta.get("label"))
        .and_then(Value::as_str);
    let (field, name) = match (
        meta_label,
        element.get("name").and_then(Value::as_str),
        element.get("id").and_then(Value::as_str),
    ) {
        (Some(name), _, _) => ("meta.label", name),
        (None, Some(name), _) => ("name", name),
        (None, None, Some(name)) => ("id", name),
        (None, None, None) => return Ok(()),
    };
    if name.len() > LABEL_MAX_BYTES {
        return Err(format!(
            "{field} names the element in IFC and is {} UTF-8 bytes; an IFC label is at most {LABEL_MAX_BYTES}",
            name.len()
        ));
    }
    Ok(())
}

fn label_text(value: &Value, field: &str) -> Result<String, String> {
    value
        .as_str()
        .filter(|text| {
            !text.is_empty()
                && text.len() <= LABEL_MAX_BYTES
                && !text.chars().any(|ch| (ch as u32) <= 0x1f || ch == '\u{7f}')
        })
        .map(str::to_owned)
        .ok_or_else(|| {
            format!("{field} must be a string of 1..{LABEL_MAX_BYTES} UTF-8 bytes with no C0/DEL controls")
        })
}

fn short(value: f64) -> String {
    let text = format!("{value:.9}");
    text.trim_end_matches('0').trim_end_matches('.').to_string()
}

/// Parse and validate the frame.
fn parse_frame(element: &Map<String, Value>) -> Result<ExtrusionFrame, String> {
    let frame = element
        .get("frame")
        .and_then(Value::as_object)
        .ok_or_else(|| "frame is required: {origin, uDir, vDir, normal}".to_string())?;
    let origin = vec3_at(frame, "origin")?;
    let u = vec3_at(frame, "uDir")?;
    let v = vec3_at(frame, "vDir")?;
    let normal = vec3_at(frame, "normal")?;
    for (key, axis) in [("uDir", u), ("vDir", v), ("normal", normal)] {
        let length = length3(axis);
        if (length - 1.0).abs() > FRAME_TOLERANCE {
            return Err(format!(
                "frame.{key} must be a unit vector (its length is {})",
                short(length)
            ));
        }
    }
    for (a, b, name_a, name_b) in [
        (u, v, "uDir", "vDir"),
        (u, normal, "uDir", "normal"),
        (v, normal, "vDir", "normal"),
    ] {
        let dot = dot3(a, b);
        if dot.abs() > FRAME_TOLERANCE {
            return Err(format!(
                "frame is not orthonormal: {name_a} · {name_b} is {}, not 0",
                short(dot)
            ));
        }
    }
    let expected = cross3(u, v);
    if dot3(expected, normal) < 0.0 {
        return Err("frame is left-handed: normal must equal uDir × vDir".to_string());
    }
    if (0..3).any(|i| (expected[i] - normal[i]).abs() > FRAME_TOLERANCE) {
        return Err("frame is not right-handed: normal must equal uDir × vDir".to_string());
    }
    Ok(ExtrusionFrame {
        origin,
        u,
        v,
        normal,
    })
}

fn parse_items(element: &Map<String, Value>) -> Result<Vec<ExtrusionItem>, String> {
    let items = element
        .get("items")
        .and_then(Value::as_array)
        .ok_or_else(|| {
            "items is required: an array of {outline, holes?, fromMm, depthMm}".to_string()
        })?;
    if items.is_empty() || items.len() > MAX_ITEMS {
        return Err(format!(
            "items has {} entries, it must have 1..{MAX_ITEMS}",
            items.len()
        ));
    }
    let mut out = Vec::with_capacity(items.len());
    for (index, item) in items.iter().enumerate() {
        let path = format!("items[{index}]");
        let item = item
            .as_object()
            .ok_or_else(|| format!("{path} must be an object"))?;
        let outline_label = format!("{path}.outline");
        let holes_label = format!("{path}.holes");
        let outline = item
            .get("outline")
            .ok_or_else(|| format!("{outline_label} is required, an array of [u, v] points"))?;
        let rings = parse_rings(
            outline,
            item.get("holes"),
            &outline_label,
            &holes_label,
            &ITEM_LIMITS,
        )
        .map_err(|reason| {
            // The shared rules name a ring set generically ("item has 65 holes"); say which.
            reason.replacen("item has", &format!("{path} has"), 1)
        })?;
        let from_mm = finite(item.get("fromMm"))
            .ok_or_else(|| format!("{path}.fromMm must be a finite number"))?;
        let depth_mm = finite(item.get("depthMm"))
            .ok_or_else(|| format!("{path}.depthMm must be a finite number"))?;
        if depth_mm < TOLERANCE {
            return Err(format!(
                "{path}.depthMm must be at least 0.00001 mm (got {})",
                short(depth_mm)
            ));
        }
        out.push(ExtrusionItem {
            rings,
            from_mm,
            depth_mm,
        });
    }
    Ok(out)
}

fn parse_class(element: &Map<String, Value>) -> Result<Option<ExtrusionClass>, String> {
    match element.get("class") {
        None | Some(Value::Null) => Ok(None),
        Some(Value::String(name)) if name == "wall" => Ok(Some(ExtrusionClass::Wall)),
        Some(Value::String(name)) if name == "slab" => Ok(Some(ExtrusionClass::Slab)),
        Some(Value::String(name)) if name == "footing" => Ok(Some(ExtrusionClass::Footing)),
        Some(_) => Err("class must be \"wall\", \"slab\" or \"footing\" when present".to_string()),
    }
}

fn class_word(class: Option<ExtrusionClass>) -> &'static str {
    class.map_or("a generic shape (no class)", |class| match class {
        ExtrusionClass::Wall => "a wall",
        ExtrusionClass::Slab => "a slab",
        ExtrusionClass::Footing => "a footing",
    })
}

/// Validate an `extrusion` element (the object itself; its `id` is the caller's). `Err` is the
/// exact rule that failed, beginning with the field it concerns.
pub fn parse_extrusion(element: &Map<String, Value>) -> Result<Extrusion, String> {
    if element.contains_key("rot") {
        return Err(
            "rot does not apply to an extrusion: its frame states the orientation".to_string(),
        );
    }
    check_name(element)?;
    let frame = parse_frame(element)?;
    let items = parse_items(element)?;
    let class = parse_class(element)?;

    let object_type = match element.get("objectType") {
        None | Some(Value::Null) => None,
        Some(value) => Some(label_text(value, "objectType")?),
    };
    let predefined_type = match element.get("predefinedType") {
        None | Some(Value::Null) => None,
        Some(Value::String(name)) => {
            let allowed = predefined_types(class);
            if !allowed.contains(&name.as_str()) {
                return Err(format!(
                    "predefinedType `{name}` is not in the IFC4 enumeration for {}: {}",
                    class_word(class),
                    allowed.join(", ")
                ));
            }
            Some(name.clone())
        }
        Some(_) => return Err("predefinedType must be a string".to_string()),
    };
    if predefined_type.as_deref() == Some("USERDEFINED") && object_type.is_none() {
        return Err(
            "objectType is required when predefinedType is USERDEFINED (IFC4 CorrectPredefinedType)"
                .to_string(),
        );
    }

    let axis = match element.get("axis") {
        None | Some(Value::Null) => None,
        Some(value) => {
            if !matches!(
                class,
                Some(ExtrusionClass::Wall) | Some(ExtrusionClass::Footing)
            ) {
                return Err(format!(
                    "axis applies only to a wall or a footing, not to {}",
                    class_word(class)
                ));
            }
            let axis = value
                .as_object()
                .ok_or_else(|| "axis must be an object {u0, u1}".to_string())?;
            let u0 = finite(axis.get("u0"))
                .ok_or_else(|| "axis.u0 must be a finite number".to_string())?;
            let u1 = finite(axis.get("u1"))
                .ok_or_else(|| "axis.u1 must be a finite number".to_string())?;
            if u1 <= u0 {
                return Err(format!(
                    "axis.u1 ({}) must be greater than axis.u0 ({})",
                    short(u1),
                    short(u0)
                ));
            }
            Some((u0, u1))
        }
    };

    let load_bearing = match element.get("loadBearing") {
        None | Some(Value::Null) => None,
        Some(Value::Bool(flag)) => {
            if class.is_none() {
                return Err(
                    "loadBearing needs a class: a generic shape has no IFC4 common property set that carries it"
                        .to_string(),
                );
            }
            Some(*flag)
        }
        Some(_) => return Err("loadBearing must be true or false".to_string()),
    };

    let material = match element.get("material") {
        None | Some(Value::Null) => None,
        Some(value) => Some(parse_material(value)?),
    };

    let mut quantities = Vec::new();
    match element.get("quantities") {
        None | Some(Value::Null) => {}
        Some(Value::Object(given)) => {
            let accepted = quantity_keys(class);
            if class.is_none() && !given.is_empty() {
                return Err(
                    "quantities need a class: a generic shape has no IFC4 base quantity set"
                        .to_string(),
                );
            }
            if let Some(unknown) = given
                .keys()
                .find(|key| !accepted.iter().any(|(name, _, _)| name == key))
            {
                let names: Vec<&str> = accepted.iter().map(|(name, _, _)| *name).collect();
                return Err(format!(
                    "quantities.{unknown} does not apply to {}; its keys are {}",
                    class_word(class),
                    names.join(", ")
                ));
            }
            for (key, ifc_name, measure) in accepted {
                let Some(value) = given.get(*key) else {
                    continue;
                };
                let number = finite(Some(value))
                    .ok_or_else(|| format!("quantities.{key} must be a finite number"))?;
                if number < 0.0 {
                    return Err(format!(
                        "quantities.{key} must not be negative (got {})",
                        short(number)
                    ));
                }
                quantities.push((*ifc_name, *measure, number));
            }
        }
        Some(_) => return Err("quantities must be an object".to_string()),
    }

    Ok(Extrusion {
        frame,
        items,
        class,
        predefined_type,
        object_type,
        axis,
        load_bearing,
        material,
        quantities,
    })
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    fn wall() -> Value {
        json!({
            "id": "W-1", "kind": "extrusion", "class": "wall", "predefinedType": "SOLIDWALL",
            "frame": {"origin":[1000,2000,0],"uDir":[1,0,0],"vDir":[0,1,0],"normal":[0,0,1]},
            "items": [{"outline":[[0,-100],[6000,-100],[6000,100],[0,100]],"fromMm":0,"depthMm":3000}],
            "axis": {"u0":0,"u1":6000},
            "loadBearing": true,
            "material": {"layers":[{"name":"Concrete block","thicknessMm":200,"grade":"CMU-200"}]},
            "quantities": {"lengthMm":6000,"widthMm":200,"heightMm":3000,"grossVolumeM3":3.6}
        })
    }

    fn slab() -> Value {
        json!({
            "id": "S-1", "kind": "extrusion", "class": "slab",
            "frame": {"origin":[0,0,2850],"uDir":[1,0,0],"vDir":[0,1,0],"normal":[0,0,1]},
            "items": [{"outline":[[0,0],[8000,0],[8000,6000],[0,6000]],
                       "holes":[[[1000,1000],[1000,2000],[2000,2000],[2000,1000]],
                                [[5000,3000],[5000,4000],[6000,4000],[6000,3000]]],
                       "fromMm":0,"depthMm":150}]
        })
    }

    fn parse(value: &Value) -> Result<Extrusion, String> {
        parse_extrusion(value.as_object().unwrap())
    }

    fn err(value: &Value) -> String {
        parse(value).unwrap_err()
    }

    #[test]
    fn a_wall_and_a_slab_with_two_holes_are_accepted_as_sent() {
        let wall = parse(&wall()).unwrap();
        assert_eq!(wall.class, Some(ExtrusionClass::Wall));
        assert_eq!(wall.axis, Some((0.0, 6000.0)));
        assert_eq!(wall.load_bearing, Some(true));
        assert_eq!(
            wall.material,
            Some(vec![MaterialLayer {
                name: "Concrete block".into(),
                thickness_mm: 200.0,
                grade: Some("CMU-200".into())
            }])
        );
        assert!((wall.volume_m3() - 3.6).abs() < 1e-12);
        let slab = parse(&slab()).unwrap();
        assert_eq!(slab.items[0].rings.holes.len(), 2);
        // 48 m² less two 1 m² openings, 150 mm thick.
        assert!((slab.volume_m3() - 46.0 * 0.15).abs() < 1e-12);
        // The rings are kept as sent: nothing reversed, nothing re-centred.
        assert_eq!(slab.items[0].rings.holes[0][1], [1000.0, 2000.0]);
    }

    #[test]
    fn a_footing_pad_and_a_strip_with_an_axis_are_accepted() {
        let mut pad = slab();
        pad["class"] = json!("footing");
        pad["predefinedType"] = json!("PAD_FOOTING");
        pad["items"][0].as_object_mut().unwrap().remove("holes");
        pad["quantities"] =
            json!({"grossVolumeM3":7.2,"grossWeightKg":17280,"crossSectionAreaM2":48});
        let pad = parse(&pad).unwrap();
        assert_eq!(pad.class, Some(ExtrusionClass::Footing));
        assert_eq!(pad.quantities[0].0, "CrossSectionArea");
        assert_eq!(
            pad.quantities[2],
            ("GrossWeight", QuantityMeasure::WeightKg, 17280.0)
        );
        let mut strip = wall();
        strip["class"] = json!("footing");
        strip["predefinedType"] = json!("STRIP_FOOTING");
        strip["quantities"] = json!({"lengthMm":6000,"heightMm":600});
        assert_eq!(parse(&strip).unwrap().axis, Some((0.0, 6000.0)));
    }

    #[test]
    fn a_frame_that_is_not_orthonormal_or_is_left_handed_is_refused() {
        let mut scaled = wall();
        scaled["frame"]["uDir"] = json!([2, 0, 0]);
        assert!(err(&scaled).starts_with("frame.uDir must be a unit vector"));
        let mut skew = wall();
        let s = std::f64::consts::FRAC_1_SQRT_2;
        skew["frame"]["vDir"] = json!([s, s, 0]);
        assert!(err(&skew).contains("frame is not orthonormal: uDir · vDir"));
        let mut left = wall();
        left["frame"]["normal"] = json!([0, 0, -1]);
        assert_eq!(
            err(&left),
            "frame is left-handed: normal must equal uDir × vDir"
        );
        let mut missing = wall();
        missing["frame"].as_object_mut().unwrap().remove("normal");
        assert!(err(&missing).contains("frame.normal must be an array of 3"));
        let mut nan = wall();
        nan["frame"]["origin"] = json!([0, "x", 0]);
        assert!(err(&nan).contains("frame.origin"));
    }

    #[test]
    fn depth_must_be_positive_and_from_finite() {
        // 1e-12 is positive but would be written as `0.` (IfcPositiveLengthMeasure breaks), so it is
        // refused with the rule rather than rounded away.
        for depth in [
            json!(0),
            json!(-5),
            json!("3"),
            json!(1.0e-12),
            json!(0.000009),
        ] {
            let mut bad = wall();
            bad["items"][0]["depthMm"] = depth.clone();
            assert!(err(&bad).starts_with("items[0].depthMm"), "{depth}");
        }
        let mut bad = wall();
        bad["items"][0]["depthMm"] = json!(-5);
        assert_eq!(
            err(&bad),
            "items[0].depthMm must be at least 0.00001 mm (got -5)"
        );
        let mut bad = wall();
        bad["items"][0].as_object_mut().unwrap().remove("fromMm");
        assert_eq!(err(&bad), "items[0].fromMm must be a finite number");
    }

    #[test]
    fn bad_rings_are_refused_with_the_shared_rule_and_never_reversed() {
        let mut cw = wall();
        cw["items"][0]["outline"] = json!([[0, 100], [6000, 100], [6000, -100], [0, -100]]);
        assert_eq!(
            err(&cw),
            "items[0].outline winds clockwise; it must be counter-clockwise"
        );
        let mut hole_ccw = slab();
        hole_ccw["items"][0]["holes"][1] =
            json!([[5000, 3000], [6000, 3000], [6000, 4000], [5000, 4000]]);
        assert_eq!(
            err(&hole_ccw),
            "items[0].holes[1] winds counter-clockwise; a hole must be clockwise"
        );
        let mut bowtie = wall();
        bowtie["items"][0]["outline"] = json!([[0, 0], [10, 10], [10, 0], [0, 10]]);
        assert!(err(&bowtie).contains("items[0].outline crosses or touches itself"));
        let mut two = wall();
        two["items"][0]["outline"] = json!([[0, 0], [10, 0]]);
        assert!(err(&two).contains("items[0].outline needs at least 3 distinct points"));
        let mut missing = wall();
        missing["items"][0]
            .as_object_mut()
            .unwrap()
            .remove("outline");
        assert!(err(&missing).starts_with("items[0].outline is required"));
    }

    #[test]
    fn holes_must_lie_strictly_inside_and_apart() {
        let mut outside = slab();
        outside["items"][0]["holes"][0] =
            json!([[9000, 1000], [9000, 2000], [9500, 2000], [9500, 1000]]);
        assert_eq!(
            err(&outside),
            "items[0].holes[0] is not inside the outer boundary"
        );
        let mut touching = slab();
        touching["items"][0]["holes"][0] =
            json!([[0, 1000], [0, 2000], [1000, 2000], [1000, 1000]]);
        assert_eq!(
            err(&touching),
            "items[0].holes[0] touches or crosses the outer boundary"
        );
        let mut overlap = slab();
        overlap["items"][0]["holes"][1] =
            json!([[1500, 1500], [1500, 2500], [2500, 2500], [2500, 1500]]);
        assert_eq!(
            err(&overlap),
            "items[0].holes[0] and items[0].holes[1] touch or overlap"
        );
        let mut nested = slab();
        nested["items"][0]["holes"][1] =
            json!([[1200, 1200], [1200, 1800], [1800, 1800], [1800, 1200]]);
        assert_eq!(
            err(&nested),
            "items[0].holes[0] and items[0].holes[1] lie one inside the other"
        );
    }

    #[test]
    fn too_many_items_holes_or_points_are_refused() {
        let mut nine = wall();
        let item = nine["items"][0].clone();
        nine["items"] = json!((0..9).map(|_| item.clone()).collect::<Vec<_>>());
        assert_eq!(err(&nine), "items has 9 entries, it must have 1..8");
        let mut none = wall();
        none["items"] = json!([]);
        assert_eq!(err(&none), "items has 0 entries, it must have 1..8");
        let mut eight = wall();
        eight["items"] = json!((0..8).map(|_| item.clone()).collect::<Vec<_>>());
        assert!(parse(&eight).is_ok());

        // 64 holes fit, a 65th does not.
        let hole = |index: usize| {
            let (x, y) = (
                100.0 + 100.0 * (index % 8) as f64,
                100.0 + 100.0 * (index / 8) as f64,
            );
            json!([[x, y], [x, y + 50.0], [x + 50.0, y + 50.0], [x + 50.0, y]])
        };
        let mut many = slab();
        many["items"][0]["holes"] = json!((0..64).map(hole).collect::<Vec<_>>());
        assert!(parse(&many).is_ok(), "{:?}", parse(&many).err());
        many["items"][0]["holes"] = json!((0..65).map(hole).collect::<Vec<_>>());
        assert_eq!(
            err(&many),
            "items[0] has 65 holes, at most 64 are supported"
        );

        // 512 points in a ring fit (and a closing repeat is free), 513 do not.
        let circle = |n: usize| {
            (0..n)
                .map(|i| {
                    let a = std::f64::consts::TAU * i as f64 / n as f64;
                    json!([5000.0 * a.cos(), 5000.0 * a.sin()])
                })
                .collect::<Vec<_>>()
        };
        let mut ring = wall();
        ring["items"][0]["outline"] = json!(circle(512));
        assert!(parse(&ring).is_ok());
        let mut closed = circle(512);
        closed.push(closed[0].clone());
        ring["items"][0]["outline"] = json!(closed);
        assert!(parse(&ring).is_ok());
        ring["items"][0]["outline"] = json!(circle(513));
        assert_eq!(
            err(&ring),
            "items[0].outline has 513 points, at most 512 are supported in one ring"
        );
    }

    #[test]
    fn predefined_type_is_checked_against_its_class_enumeration() {
        let mut bad = wall();
        bad["predefinedType"] = json!("FLOOR");
        assert!(err(&bad).starts_with(
            "predefinedType `FLOOR` is not in the IFC4 enumeration for a wall: MOVABLE"
        ));
        let mut bad = slab();
        bad["predefinedType"] = json!("SOLIDWALL");
        assert!(err(&bad).contains("for a slab"));
        let mut bad = slab();
        bad["class"] = json!("footing");
        bad["predefinedType"] = json!("FLOOR");
        assert!(err(&bad).contains("for a footing: CAISSON_FOUNDATION"));
        let mut proxy = slab();
        proxy.as_object_mut().unwrap().remove("class");
        proxy["predefinedType"] = json!("ELEMENT");
        assert!(parse(&proxy).is_ok());
        proxy["predefinedType"] = json!("FLOOR");
        assert!(err(&proxy).contains("for a generic shape (no class)"));
        let mut lower = wall();
        lower["predefinedType"] = json!("solidwall");
        assert!(err(&lower).contains("`solidwall` is not in the IFC4 enumeration"));
        let mut bad = wall();
        bad["class"] = json!("column");
        assert!(err(&bad).starts_with("class must be"));
    }

    #[test]
    fn userdefined_needs_an_object_type() {
        let mut bad = wall();
        bad["predefinedType"] = json!("USERDEFINED");
        assert_eq!(
            err(&bad),
            "objectType is required when predefinedType is USERDEFINED (IFC4 CorrectPredefinedType)"
        );
        bad["objectType"] = json!("Acme CMU 200");
        assert_eq!(
            parse(&bad).unwrap().object_type.as_deref(),
            Some("Acme CMU 200")
        );
        bad["objectType"] = json!("");
        assert!(err(&bad).starts_with("objectType must be a string"));
    }

    #[test]
    fn quantities_must_be_finite_non_negative_and_belong_to_the_class() {
        let mut bad = wall();
        bad["quantities"]["netVolumeM3"] = json!(-1);
        assert_eq!(
            err(&bad),
            "quantities.netVolumeM3 must not be negative (got -1)"
        );
        let mut bad = wall();
        bad["quantities"]["heightMm"] = json!("3000");
        assert_eq!(err(&bad), "quantities.heightMm must be a finite number");
        let mut bad = wall();
        bad["quantities"]["perimeterMm"] = json!(10);
        assert!(err(&bad).starts_with("quantities.perimeterMm does not apply to a wall"));
        let mut bad = slab();
        bad["quantities"] = json!({"heightMm": 150});
        assert!(err(&bad).starts_with("quantities.heightMm does not apply to a slab"));
        let mut bad = slab();
        bad.as_object_mut().unwrap().remove("class");
        bad["quantities"] = json!({"grossVolumeM3": 1});
        assert!(err(&bad).starts_with("quantities need a class"));
        let mut zero = wall();
        zero["quantities"] = json!({"netSideAreaM2": 0});
        assert!(parse(&zero).is_ok());
        // Walls and slabs carry weights in kilograms, as footings do, and the same rules apply.
        for mut element in [wall(), slab()] {
            element["quantities"] = json!({"grossWeightKg": 8640, "netWeightKg": 8280});
            let parsed = parse(&element).unwrap();
            assert_eq!(
                parsed.quantities,
                vec![
                    ("GrossWeight", QuantityMeasure::WeightKg, 8640.0),
                    ("NetWeight", QuantityMeasure::WeightKg, 8280.0)
                ]
            );
            element["quantities"] = json!({"netWeightKg": -1});
            assert_eq!(
                err(&element),
                "quantities.netWeightKg must not be negative (got -1)"
            );
        }
    }

    #[test]
    fn axis_load_bearing_and_material_are_checked() {
        let mut bad = slab();
        bad["axis"] = json!({"u0":0,"u1":10});
        assert_eq!(
            err(&bad),
            "axis applies only to a wall or a footing, not to a slab"
        );
        let mut bad = wall();
        bad["axis"] = json!({"u0":10,"u1":10});
        assert_eq!(err(&bad), "axis.u1 (10) must be greater than axis.u0 (10)");
        let mut bad = wall();
        bad["loadBearing"] = json!("yes");
        assert_eq!(err(&bad), "loadBearing must be true or false");
        let mut bad = slab();
        bad.as_object_mut().unwrap().remove("class");
        bad["loadBearing"] = json!(true);
        assert!(err(&bad).starts_with("loadBearing needs a class"));
    }

    #[test]
    fn material_is_exactly_one_layer_with_a_positive_thickness_and_no_unknown_key() {
        let layer = |extra: Value| {
            let mut layer = json!({"name": "Concrete", "thicknessMm": 200});
            for (key, value) in extra.as_object().unwrap() {
                layer[key] = value.clone();
            }
            layer
        };
        let with = |material: Value| {
            let mut element = wall();
            element["material"] = material;
            element
        };
        // A one-layer material is accepted with or without a class, with or without a grade.
        let mut classless = slab();
        classless.as_object_mut().unwrap().remove("class");
        classless.as_object_mut().unwrap().remove("quantities");
        classless["material"] = json!({"layers": [layer(json!({}))]});
        assert_eq!(parse(&classless).unwrap().material.unwrap()[0].grade, None);
        assert!(
            parse(&with(
                json!({"layers": [layer(json!({"grade": "C30/37"}))]})
            ))
            .is_ok()
        );
        for (material, rule) in [
            (
                json!({"layers": [layer(json!({})), layer(json!({}))]}),
                "material.layers must have exactly 1 layer (got 2)",
            ),
            (
                json!({"layers": []}),
                "material.layers must have exactly 1 layer (got 0)",
            ),
            (
                json!({"layers": [layer(json!({"thicknessMm": 0}))]}),
                "material.layers[0].thicknessMm must be greater than zero (got 0)",
            ),
            (
                json!({"layers": [layer(json!({"thicknessMm": -10}))]}),
                "material.layers[0].thicknessMm must be greater than zero (got -10)",
            ),
            (
                json!({"layers": [layer(json!({"thicknessMm": "200"}))]}),
                "material.layers[0].thicknessMm must be a finite number",
            ),
            (
                json!({"layers": [{"thicknessMm": 200}]}),
                "material.layers[0].name is required",
            ),
            (
                json!({"layers": [layer(json!({"function": "core"}))]}),
                "material.layers[0] has an unknown field `function`; only `name`, `thicknessMm` and `grade` are defined",
            ),
            (
                json!({"name": "Concrete"}),
                "material has an unknown field `name`; only `layers` is defined",
            ),
            (
                json!({"layers": [layer(json!({}))], "usage": {}}),
                "material has an unknown field `usage`; only `layers` is defined",
            ),
            (json!("Concrete"), "material must be an object {layers}"),
            (json!({}), "material.layers is required"),
            (
                json!({"layers": [layer(json!({"name": "   "}))]}),
                "material.layers[0].name must not be blank",
            ),
            (
                json!({"layers": [layer(json!({"grade": " "}))]}),
                "material.layers[0].grade must not be blank",
            ),
            (json!({"layers": {}}), "material.layers must be an array"),
        ] {
            assert_eq!(err(&with(material.clone())), rule, "{material}");
        }
        let mut bad = wall();
        bad["rot"] = json!(0);
        assert!(err(&bad).starts_with("rot does not apply"));
    }
}
