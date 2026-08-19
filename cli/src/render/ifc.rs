//! `ifc.write` — write a generic 3D **scene** to a universal **IFC4** file (builtin transport).
//!
//! The file-writing sibling of `viewer-3d.render`: it consumes the SAME domain-agnostic scene
//! (members as `from`->`to` axes with an optional parametric cross-section + a `group`) and emits an
//! IFC4 STEP (SPF) document — IfcColumn/IfcBeam/IfcMember as extruded profiles placed on the member
//! axis, under an IfcProject -> IfcSite -> IfcBuilding -> IfcBuildingStorey spine. Each element may
//! carry an `xsection` (i/channel/angle/rhs/chs/rect/tee/double-angle) → the matching IfcProfileDef; a
//! neutral `role` → the element type; a `material` → an IfcMaterial association; and its `group`'s
//! colour → an IfcStyledItem. A `kind:"mesh"` element (tessellated `positions`+`indices`) is written
//! as an IfcTriangulatedFaceSet on an IfcBuildingElementProxy. The writer stays GENERIC: it applies
//! whatever descriptor the scene carries (the domain — e.g. floless steel — owns the meaning) and
//! falls back to a rectangle for a missing/invalid `xsection`.
//!
//! MVD: parametric profile defs are Design Transfer View content, so the header declares
//! `DesignTransferView_V1.0`.
//!
//! Determinism: identical `scene` input -> identical IFC bytes. GlobalIds come from an entity counter
//! (no randomness), shared colour/material entities are emitted in sorted (BTreeMap) order before the
//! elements, reals use a fixed invariant format, and the SPF stamp is fixed — no clock, no environment.

use crate::error::AwareError;
use crate::json::type_name as json_type;
use crate::render::geom::{
    Vec2, Vec3, add3, cross3, distance3, dot3, length3, normalized3, point_in_polygon,
    point_segment_distance, polygon_edges, polygon_is_simple_nonzero, scale3,
};
use crate::render::scene_roll::{SceneUp, ZeroFrameSource, member_frame, member_roll, scene_up};
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;

/// Base-64-ish charset for IFC GlobalIds (valid IFC GUID alphabet).
const B64: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz_$";

/// A deterministic 22-char IFC GlobalId from an integer counter (no randomness, no clock).
fn guid(n: i64) -> String {
    let mut v = n + 1;
    let mut out = [b'0'; 22];
    for slot in out.iter_mut().rev() {
        *slot = B64[(v & 63) as usize];
        v >>= 6;
    }
    // ASCII by construction.
    String::from_utf8(out.to_vec()).unwrap_or_default()
}

/// A stable IFC GlobalId derived from a scene record id. Unlike the legacy entity-counter ids,
/// this survives unrelated scene insertions and re-ordering. The four leading pad bits required by
/// the 22-character IFC encoding keep the first character in `0..3`.
fn record_guid(namespace: &str, id: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(namespace.as_bytes());
    hasher.update([0]);
    hasher.update(id.as_bytes());
    let digest = hasher.finalize();
    let mut bytes = [0u8; 16];
    bytes.copy_from_slice(&digest[..16]);
    let value = u128::from_be_bytes(bytes);
    let mut out = String::with_capacity(22);
    out.push(B64[((value >> 126) & 0x03) as usize] as char);
    for shift in (0..=120).rev().step_by(6) {
        out.push(B64[((value >> shift) & 0x3f) as usize] as char);
    }
    out
}

/// An IFC (ISO 10303-21) string literal: single-quoted, with Part 21 escaping. Apostrophe and
/// backslash (the Part 21 escape introducer) are doubled; any control or non-ASCII character is
/// emitted as a `\X2\<utf-16 hex…>\X0\` block so the SPF stays valid for strict parsers even when
/// names carry an em-dash, accented letters, etc.
fn s_lit(x: &str) -> String {
    /// Flush a run of accumulated UTF-16 code units as one `\X2\…\X0\` escape block.
    fn flush(out: &mut String, u16s: &mut Vec<u16>) {
        if u16s.is_empty() {
            return;
        }
        out.push_str("\\X2\\");
        for u in u16s.iter() {
            let _ = write!(out, "{u:04X}");
        }
        out.push_str("\\X0\\");
        u16s.clear();
    }

    let mut out = String::from("'");
    let mut u16s: Vec<u16> = Vec::new();
    for ch in x.chars() {
        if ch == ' ' || ch.is_ascii_graphic() {
            flush(&mut out, &mut u16s); // close any pending non-ASCII run first
            match ch {
                '\'' => out.push_str("''"),
                '\\' => out.push_str("\\\\"),
                _ => out.push(ch),
            }
        } else {
            // control or non-ASCII: batch the UTF-16 code unit(s) into the current escape run
            let mut buf = [0u16; 2];
            for u in ch.encode_utf16(&mut buf) {
                u16s.push(*u);
            }
        }
    }
    flush(&mut out, &mut u16s);
    out.push('\'');
    out
}

/// An IFC real: invariant, always with a decimal point, trailing zeros trimmed (but keep one).
fn r(d: f64) -> String {
    let d = if d == 0.0 { 0.0 } else { d }; // normalize -0.0 -> 0.0
    let mut s = format!("{d:.11}");
    if s.contains('.') {
        while s.ends_with('0') {
            s.pop();
        }
        if s.ends_with('.') {
            s.push('0');
        }
    }
    s
}

/// A number from an optional JSON value (int or float), defaulting to 0.0.
fn num(v: Option<&Value>) -> f64 {
    v.and_then(Value::as_f64).unwrap_or(0.0)
}

/// A finite, strictly-positive f64 from a JSON field, else None (guards profile-def validity).
fn pos(v: Option<&Value>) -> Option<f64> {
    v.and_then(Value::as_f64)
        .filter(|x| x.is_finite() && *x > 0.0)
}

/// A deterministic, scene-derived ascii filename for the SPF `FILE_NAME` metadata field.
fn file_name_meta(name: &str) -> String {
    let base = match name.find('(') {
        Some(i) => &name[..i],
        None => name,
    };
    let mut out = String::new();
    for ch in base.chars() {
        if ch.is_ascii_alphanumeric() {
            out.push(ch.to_ascii_lowercase());
        } else if !out.is_empty() && !out.ends_with('-') {
            out.push('-');
        }
    }
    let mut out = out.trim_matches('-').to_string();
    if out.len() > 60 {
        out.truncate(60);
        out = out.trim_matches('-').to_string();
    }
    if out.is_empty() {
        out.push_str("model");
    }
    format!("{out}.ifc")
}

/// Parse a `#RRGGBB` hex colour to r,g,b in 0..1. Only 6-digit hex is accepted (anything else → None,
/// i.e. no style rather than a fabricated colour).
fn parse_hex(s: &str) -> Option<(f64, f64, f64)> {
    let h = s.trim().strip_prefix('#')?;
    if h.len() != 6 || !h.bytes().all(|b| b.is_ascii_hexdigit()) {
        return None;
    }
    let c = |a: usize| {
        u8::from_str_radix(&h[a..a + 2], 16)
            .ok()
            .map(|v| v as f64 / 255.0)
    };
    Some((c(0)?, c(2)?, c(4)?))
}

/// Resolve an element's colour via its `group` → the scene `groups` colour map. Returns the
/// normalized hex key (upper) + the rgb, or None when the group has no valid `#RRGGBB` colour.
fn resolve_color(
    el: &Value,
    group_colors: &BTreeMap<String, String>,
) -> Option<(String, (f64, f64, f64))> {
    let g = el.get("group").and_then(Value::as_str)?;
    let hex = group_colors.get(g)?;
    let rgb = parse_hex(hex)?;
    Some((hex.trim().to_uppercase(), rgb))
}

/// Resolve an element's material to (dedupe key, display name), or None when empty. Dedupe on a
/// trim+upper key; keep the trimmed original as the display name.
fn resolve_material(el: &Value) -> Option<(String, String)> {
    let m = el.get("material").and_then(Value::as_str)?.trim();
    if m.is_empty() {
        return None;
    }
    Some((m.to_uppercase(), m.to_string()))
}

/// SPF entity emitter: assigns sequential `#ids` and accumulates the DATA section.
struct Spf {
    id: i64,
    buf: String,
}

impl Spf {
    fn new() -> Self {
        Spf {
            id: 0,
            buf: String::new(),
        }
    }

    /// Emit `#<n>=<line>;` and return the new entity's id.
    fn emit(&mut self, line: &str) -> i64 {
        self.id += 1;
        let _ = writeln!(self.buf, "#{}={};", self.id, line);
        self.id
    }
}

fn double_angle_outlines(
    d: f64,
    b: f64,
    t: f64,
    gap: f64,
    orientation: &str,
) -> Option<[Vec<Vec2>; 2]> {
    if ![d, b, t, gap].iter().all(|value| value.is_finite())
        || d <= 0.0
        || b <= 0.0
        || t <= 0.0
        || gap < 0.0
        || t >= d.min(b)
    {
        return None;
    }
    let (back, outstanding) = match orientation {
        "llbb" => (d, b),
        "slbb" => (b, d),
        _ => return None,
    };
    let half_back = back / 2.0;
    let half_gap = gap / 2.0;
    let mut left = vec![
        [-half_gap, -half_back],
        [-half_gap, half_back],
        [-half_gap - outstanding, half_back],
        [-half_gap - outstanding, half_back - t],
        [-half_gap - t, half_back - t],
        [-half_gap - t, -half_back],
    ];
    let mut right = left
        .iter()
        .rev()
        .map(|point| [-point[0], point[1]])
        .collect::<Vec<_>>();
    // IFC outer boundaries are counter-clockwise when viewed along the extrusion direction.
    for outline in [&mut left, &mut right] {
        let signed_area = outline
            .iter()
            .zip(outline.iter().cycle().skip(1))
            .take(outline.len())
            .map(|(a, b)| a[0] * b[1] - b[0] * a[1])
            .sum::<f64>();
        if signed_area < 0.0 {
            outline.reverse();
        }
    }
    Some([left, right])
}

fn rotate_outline(outline: &[Vec2], degrees: f64) -> Vec<Vec2> {
    let radians = degrees.to_radians();
    let (sin, cos) = radians.sin_cos();
    outline
        .iter()
        .map(|point| {
            [
                point[0] * cos - point[1] * sin,
                point[0] * sin + point[1] * cos,
            ]
        })
        .collect()
}

/// Emit the profile for a member's `xsection`, positioned at `pos2d`. Falls back to an
/// `IfcRectangleProfileDef(w,d)` for a missing / unknown / IFC-WHERE-invalid descriptor. Returns the
/// profile entity id + an optional neutral warning reason (set only when a *present* xsection was
/// rejected). The generic writer never emits a schema-invalid profile.
fn emit_profile(
    spf: &mut Spf,
    xsection: Option<&Value>,
    w: f64,
    d: f64,
    name: &str,
    pos2d: i64,
    profile_rotation_degrees: f64,
) -> (i64, Option<String>) {
    let rect = |spf: &mut Spf, ww: f64, dd: f64| {
        spf.emit(&format!(
            "IFCRECTANGLEPROFILEDEF(.AREA.,{},#{pos2d},{},{})",
            s_lit(name),
            r(ww),
            r(dd)
        ))
    };
    let bad = |reason: &str| Some(format!("{name}: {reason} — rectangle fallback"));
    let xs = match xsection {
        // Absent (or explicit null) → today's rectangle, silently (backward compat).
        None | Some(Value::Null) => return (rect(spf, w, d), None),
        Some(Value::Object(o)) => o,
        // Present but not an object (string/array/number) → rectangle WITH a warning.
        Some(_) => return (rect(spf, w, d), bad("xsection is not an object")),
    };
    let f = |k: &str| pos(xs.get(k));
    let envelope = |ww: f64, dd: f64| {
        let tolerance = 1.0e-6_f64.max(1.0e-9 * ww.abs().max(dd.abs()));
        (w - ww).abs() <= tolerance && (d - dd).abs() <= tolerance
    };
    match xs.get("shape").and_then(Value::as_str).unwrap_or("") {
        "i" | "channel" => {
            let (d0, bf, tw, tf) = (f("d"), f("bf"), f("tw"), f("tf"));
            if let (Some(d0), Some(bf), Some(tw), Some(tf)) = (d0, bf, tw, tf)
                && 2.0 * tf < d0
                && tw < bf
            {
                let ent = if xs.get("shape").and_then(Value::as_str) == Some("i") {
                    // IfcIShapeProfileDef(_,_,Position,OverallWidth,OverallDepth,WebThk,FlangeThk,Fillet,FlangeEdge,FlangeSlope)
                    format!(
                        "IFCISHAPEPROFILEDEF(.AREA.,{},#{pos2d},{},{},{},{},$,$,$)",
                        s_lit(name),
                        r(bf),
                        r(d0),
                        r(tw),
                        r(tf)
                    )
                } else {
                    // IfcUShapeProfileDef(_,_,Position,Depth,FlangeWidth,WebThk,FlangeThk,Fillet,EdgeRadius,FlangeSlope)
                    format!(
                        "IFCUSHAPEPROFILEDEF(.AREA.,{},#{pos2d},{},{},{},{},$,$,$)",
                        s_lit(name),
                        r(d0),
                        r(bf),
                        r(tw),
                        r(tf)
                    )
                };
                return (spf.emit(&ent), None);
            }
            (rect(spf, w, d), bad("invalid I/channel dims"))
        }
        "angle" => {
            if let (Some(d0), Some(b), Some(t)) = (f("d"), f("b"), f("t"))
                && t < d0
                && t < b
            {
                // IfcLShapeProfileDef(_,_,Position,Depth,Width,Thickness,Fillet,EdgeRadius,LegSlope)
                return (
                    spf.emit(&format!(
                        "IFCLSHAPEPROFILEDEF(.AREA.,{},#{pos2d},{},{},{},$,$,$)",
                        s_lit(name),
                        r(d0),
                        r(b),
                        r(t)
                    )),
                    None,
                );
            }
            (rect(spf, w, d), bad("invalid angle dims"))
        }
        "rhs" => {
            if let (Some(d0), Some(b), Some(t)) = (f("d"), f("b"), f("t"))
                && t < 0.5 * d0.min(b)
            {
                // IfcRectangleHollowProfileDef(_,_,Position,XDim,YDim,WallThickness,InnerFillet,OuterFillet)
                return (
                    spf.emit(&format!(
                        "IFCRECTANGLEHOLLOWPROFILEDEF(.AREA.,{},#{pos2d},{},{},{},$,$)",
                        s_lit(name),
                        r(b),
                        r(d0),
                        r(t)
                    )),
                    None,
                );
            }
            (rect(spf, w, d), bad("invalid rhs dims"))
        }
        "chs" => {
            if let (Some(od), Some(t)) = (f("od"), f("t")) {
                let radius = od / 2.0;
                if t < radius {
                    // IfcCircleHollowProfileDef(_,_,Position,Radius,WallThickness)
                    return (
                        spf.emit(&format!(
                            "IFCCIRCLEHOLLOWPROFILEDEF(.AREA.,{},#{pos2d},{},{})",
                            s_lit(name),
                            r(radius),
                            r(t)
                        )),
                        None,
                    );
                }
            }
            (rect(spf, w, d), bad("invalid chs dims"))
        }
        "rect" => {
            let ww = f("w").unwrap_or(w);
            let dd = f("d").unwrap_or(d);
            (rect(spf, ww, dd), None)
        }
        "tee" => {
            if let (Some(d0), Some(bf), Some(tw), Some(tf)) = (f("d"), f("bf"), f("tw"), f("tf"))
                && tw < bf
                && tf < d0
                && envelope(bf, d0)
            {
                // IfcTShapeProfileDef(_,_,Position,Depth,FlangeWidth,WebThickness,FlangeThickness,
                // Fillet,FlangeEdge,WebEdge,WebSlope,FlangeSlope)
                return (
                    spf.emit(&format!(
                        "IFCTSHAPEPROFILEDEF(.AREA.,{},#{pos2d},{},{},{},{},$,$,$,$,$)",
                        s_lit(name),
                        r(d0),
                        r(bf),
                        r(tw),
                        r(tf)
                    )),
                    None,
                );
            }
            (rect(spf, w, d), bad("invalid tee dims or envelope"))
        }
        "double-angle" => {
            let d0 = f("d");
            let b = f("b");
            let t = f("t");
            let gap = xs
                .get("gap")
                .and_then(Value::as_f64)
                .filter(|value| value.is_finite() && *value >= 0.0);
            let orientation = xs.get("orientation").and_then(Value::as_str);
            if let (Some(d0), Some(b), Some(t), Some(gap), Some(orientation)) =
                (d0, b, t, gap, orientation)
            {
                let expected = match orientation {
                    "llbb" => envelope(2.0 * b + gap, d0),
                    "slbb" => envelope(2.0 * d0 + gap, b),
                    _ => false,
                };
                if expected
                    && let Some(outlines) = double_angle_outlines(d0, b, t, gap, orientation)
                {
                    let mut profiles = Vec::with_capacity(2);
                    for (index, outline) in outlines.iter().enumerate() {
                        let rotated = rotate_outline(outline, profile_rotation_degrees);
                        let curve = emit_polyline2(spf, &rotated, true);
                        profiles.push(spf.emit(&format!(
                            "IFCARBITRARYCLOSEDPROFILEDEF(.AREA.,{},#{curve})",
                            s_lit(&format!("{name} angle {}", index + 1))
                        )));
                    }
                    return (
                        spf.emit(&format!(
                            "IFCCOMPOSITEPROFILEDEF(.AREA.,{},(#{},#{}),$)",
                            s_lit(name),
                            profiles[0],
                            profiles[1]
                        )),
                        None,
                    );
                }
            }
            (
                rect(spf, w, d),
                bad("invalid double-angle dims, orientation, or envelope"),
            )
        }
        other => (
            rect(spf, w, d),
            bad(&format!("unknown xsection shape '{other}'")),
        ),
    }
}

/// Emit a tessellated `kind:"mesh"` element as an IfcTriangulatedFaceSet on an
/// IfcBuildingElementProxy at `place` (identity local placement → absolute world mm). Attaches an
/// `IfcStyledItem` to the face set when `style` resolves. Returns the proxy id, or None when
/// positions/indices are missing/malformed.
fn emit_mesh(spf: &mut Spf, el: &Value, place: i64, ctx: i64, style: Option<i64>) -> Option<i64> {
    let posarr = el.get("positions").and_then(Value::as_array)?;
    let idx = el.get("indices").and_then(Value::as_array)?;
    let npts = posarr.len() / 3;
    if npts < 3 || posarr.len() % 3 != 0 || idx.len() < 3 || idx.len() % 3 != 0 {
        return None;
    }
    let mut pts = String::new();
    for i in 0..npts {
        if i > 0 {
            pts.push(',');
        }
        let _ = write!(
            pts,
            "({},{},{})",
            r(num(posarr.get(i * 3))),
            r(num(posarr.get(i * 3 + 1))),
            r(num(posarr.get(i * 3 + 2)))
        );
    }
    let mut tris = String::new();
    for t in 0..(idx.len() / 3) {
        let a = idx.get(t * 3).and_then(Value::as_i64)?;
        let b = idx.get(t * 3 + 1).and_then(Value::as_i64)?;
        let c = idx.get(t * 3 + 2).and_then(Value::as_i64)?;
        if a < 0 || b < 0 || c < 0 || a as usize >= npts || b as usize >= npts || c as usize >= npts
        {
            return None; // an out-of-range index would make the SPF invalid — reject the element
        }
        if t > 0 {
            tris.push(',');
        }
        let _ = write!(tris, "({},{},{})", a + 1, b + 1, c + 1); // IFC CoordIndex is 1-based
    }
    let plist = spf.emit(&format!("IFCCARTESIANPOINTLIST3D(({pts}))"));
    let tfs = spf.emit(&format!("IFCTRIANGULATEDFACESET(#{plist},$,$,({tris}),$)"));
    if let Some(st) = style {
        spf.emit(&format!("IFCSTYLEDITEM(#{tfs},(#{st}),$)"));
    }
    let shape = spf.emit(&format!(
        "IFCSHAPEREPRESENTATION(#{ctx},'Body','Tessellation',(#{tfs}))"
    ));
    let pds = spf.emit(&format!("IFCPRODUCTDEFINITIONSHAPE($,$,(#{shape}))"));
    let name = el.get("id").and_then(Value::as_str).unwrap_or("Mesh");
    let g = record_guid("element", name);
    Some(spf.emit(&format!(
        "IFCBUILDINGELEMENTPROXY({},$,{},$,$,#{place},#{pds},$,$)",
        s_lit(&g),
        s_lit(name)
    )))
}

fn vec2(v: Option<&Value>) -> Option<Vec2> {
    let a = v?.as_array()?;
    if a.len() != 2 {
        return None;
    }
    let p = [a.first()?.as_f64()?, a.get(1)?.as_f64()?];
    (p[0].is_finite() && p[1].is_finite()).then_some(p)
}

fn vec3(v: Option<&Value>) -> Option<Vec3> {
    let a = v?.as_array()?;
    if a.len() != 3 {
        return None;
    }
    let p = [
        a.first()?.as_f64()?,
        a.get(1)?.as_f64()?,
        a.get(2)?.as_f64()?,
    ];
    (p[0].is_finite() && p[1].is_finite() && p[2].is_finite()).then_some(p)
}

fn label(el: &Value) -> &str {
    el.get("meta")
        .and_then(Value::as_object)
        .and_then(|m| m.get("label"))
        .and_then(Value::as_str)
        .or_else(|| el.get("name").and_then(Value::as_str))
        .or_else(|| el.get("id").and_then(Value::as_str))
        .unwrap_or("Element")
}

fn emit_axis_placement(spf: &mut Spf, origin: Vec3, axis: Vec3, ref_dir: Vec3, parent: i64) -> i64 {
    let a = emit_axis2_placement3d(spf, origin, axis, ref_dir);
    spf.emit(&format!("IFCLOCALPLACEMENT(#{parent},#{a})"))
}

fn emit_axis2_placement3d(spf: &mut Spf, origin: Vec3, axis: Vec3, ref_dir: Vec3) -> i64 {
    let p = spf.emit(&format!(
        "IFCCARTESIANPOINT(({},{},{}))",
        r(origin[0]),
        r(origin[1]),
        r(origin[2])
    ));
    let z = spf.emit(&format!(
        "IFCDIRECTION(({},{},{}))",
        r(axis[0]),
        r(axis[1]),
        r(axis[2])
    ));
    let x = spf.emit(&format!(
        "IFCDIRECTION(({},{},{}))",
        r(ref_dir[0]),
        r(ref_dir[1]),
        r(ref_dir[2])
    ));
    spf.emit(&format!("IFCAXIS2PLACEMENT3D(#{p},#{z},#{x})"))
}

fn axis_ref_dir(axis: Vec3) -> Vec3 {
    let seed = if axis[2].abs() < 0.9 {
        [0.0, 0.0, 1.0]
    } else {
        [1.0, 0.0, 0.0]
    };
    normalized3(cross3(seed, axis)).unwrap_or([1.0, 0.0, 0.0])
}

fn emit_polyline2(spf: &mut Spf, points: &[Vec2], close: bool) -> i64 {
    let mut ids = Vec::with_capacity(points.len() + 1);
    for &[x, y] in points {
        ids.push(spf.emit(&format!("IFCCARTESIANPOINT(({},{}))", r(x), r(y))));
    }
    if close && points.first() != points.last() && !ids.is_empty() {
        ids.push(ids[0]);
    }
    let refs = ids
        .iter()
        .map(|id| format!("#{id}"))
        .collect::<Vec<_>>()
        .join(",");
    spf.emit(&format!("IFCPOLYLINE(({refs}))"))
}

fn emit_body(
    spf: &mut Spf,
    profile: i64,
    length: f64,
    ctx: i64,
    extrude_pos: i64,
    dir_z: i64,
    style: Option<i64>,
) -> i64 {
    let solid = spf.emit(&format!(
        "IFCEXTRUDEDAREASOLID(#{profile},#{extrude_pos},#{dir_z},{})",
        r(length)
    ));
    if let Some(st) = style {
        spf.emit(&format!("IFCSTYLEDITEM(#{solid},(#{st}),$)"));
    }
    let shape = spf.emit(&format!(
        "IFCSHAPEREPRESENTATION(#{ctx},'Body','SweptSolid',(#{solid}))"
    ));
    spf.emit(&format!("IFCPRODUCTDEFINITIONSHAPE($,$,(#{shape}))"))
}

fn emit_polyline3(spf: &mut Spf, points: &[Vec3]) -> i64 {
    let ids = points
        .iter()
        .map(|p| {
            spf.emit(&format!(
                "IFCCARTESIANPOINT(({},{},{}))",
                r(p[0]),
                r(p[1]),
                r(p[2])
            ))
        })
        .collect::<Vec<_>>();
    let refs = ids
        .iter()
        .map(|id| format!("#{id}"))
        .collect::<Vec<_>>()
        .join(",");
    spf.emit(&format!("IFCPOLYLINE(({refs}))"))
}

/// The 2D cut profile of a `box` Boolean tool in its own u-v plane: a `2hu × 2hv` rectangle centred at the
/// origin, with the single re-entrant corner (at +u, on the `vsign` side) rounded by `rad` when given — so a
/// cope removes the true filleted notch, never a square over-cut. Returns an `IfcArbitraryClosedProfileDef`
/// (fillet, via an `IfcIndexedPolyCurve` with explicit line + arc segments) or an `IfcRectangleProfileDef`
/// (square). (Codex plan-review R2 #9 / R3 #9.)
fn emit_box_cut_profile(
    spf: &mut Spf,
    pos2d: i64,
    hu: f64,
    hv: f64,
    reentrant: Option<(f64, i64)>,
    name: &str,
) -> i64 {
    if let Some((rad, vsign)) = reentrant
        && rad > 0.0
    {
        let sy = if vsign >= 0 { 1.0 } else { -1.0 };
        let vy = sy * hv; // the rounded corner's y (top or bottom on the +u edge)
        let cx = hu - rad; // arc centre
        let cy = vy - sy * rad;
        let frac = std::f64::consts::FRAC_1_SQRT_2;
        let mid = (cx + rad * frac, cy + sy * rad * frac); // arc midpoint (45° toward the corner)
        let far_bl = (-hu, -hv);
        let far_tl = (-hu, hv);
        // Walk the outline CCW; the arc replaces exactly the rounded corner, with the tangent points on
        // the +u edge and the horizontal (vy) edge.
        let (pts, segs): (Vec<(f64, f64)>, &str) = if sy > 0.0 {
            (
                vec![
                    far_bl,
                    (hu, -hv),
                    (hu, vy - rad),
                    mid,
                    (hu - rad, vy),
                    far_tl,
                ],
                "IFCLINEINDEX((1,2)),IFCLINEINDEX((2,3)),IFCARCINDEX((3,4,5)),IFCLINEINDEX((5,6)),IFCLINEINDEX((6,1))",
            )
        } else {
            (
                vec![
                    far_bl,
                    (hu - rad, -hv),
                    mid,
                    (hu, -hv + rad),
                    (hu, hv),
                    far_tl,
                ],
                "IFCLINEINDEX((1,2)),IFCARCINDEX((2,3,4)),IFCLINEINDEX((4,5)),IFCLINEINDEX((5,6)),IFCLINEINDEX((6,1))",
            )
        };
        let coords = pts
            .iter()
            .map(|(x, y)| format!("({},{})", r(*x), r(*y)))
            .collect::<Vec<_>>()
            .join(",");
        let plist = spf.emit(&format!("IFCCARTESIANPOINTLIST2D(({coords}))"));
        let curve = spf.emit(&format!("IFCINDEXEDPOLYCURVE(#{plist},({segs}),.F.)"));
        return spf.emit(&format!(
            "IFCARBITRARYCLOSEDPROFILEDEF(.AREA.,{},#{curve})",
            s_lit(name)
        ));
    }
    spf.emit(&format!(
        "IFCRECTANGLEPROFILEDEF(.AREA.,{},#{pos2d},{},{})",
        s_lit(name),
        r(2.0 * hu),
        r(2.0 * hv)
    ))
}

struct FastenerSweep {
    start: Vec3,
    axis: Vec3,
    length: f64,
    bounding_diagonal: f64,
    profile: i64,
}

fn emit_fastener_sweep(spf: &mut Spf, el: &Value, pos2d: i64) -> Option<FastenerSweep> {
    let kind = el.get("kind").and_then(Value::as_str)?;
    let (start, axis, length, cross_section_diameter, profile) =
        if matches!(kind, "rod" | "bolt-shank") {
            let axis_obj = el.get("axis")?.as_object()?;
            let from = vec3(axis_obj.get("from"))?;
            let to = vec3(axis_obj.get("to"))?;
            let delta = [to[0] - from[0], to[1] - from[1], to[2] - from[2]];
            let axis = normalized3(delta)?;
            let length = length3(delta);
            let diameter = pos(el.get("diameterMm"))?;
            let circle = spf.emit(&format!("IFCCIRCLE(#{pos2d},{})", r(diameter / 2.0)));
            let profile = spf.emit(&format!(
                "IFCARBITRARYCLOSEDPROFILEDEF(.AREA.,{},#{circle})",
                s_lit(label(el))
            ));
            (from, axis, length, diameter, profile)
        } else {
            let center = vec3(el.get("center"))?;
            let axis = vec3(el.get("axis")).and_then(normalized3)?;
            let length = pos(el.get("thicknessMm"))?;
            let start = add3(center, scale3(axis, -length / 2.0));
            if kind == "washer" {
                let outer_d = pos(el.get("outerDiameterMm"))?;
                let inner_d = pos(el.get("innerDiameterMm"))?;
                if inner_d >= outer_d {
                    return None;
                }
                let outer = spf.emit(&format!("IFCCIRCLE(#{pos2d},{})", r(outer_d / 2.0)));
                let inner = spf.emit(&format!("IFCCIRCLE(#{pos2d},{})", r(inner_d / 2.0)));
                let profile = spf.emit(&format!(
                    "IFCARBITRARYPROFILEDEFWITHVOIDS(.AREA.,{},#{outer},(#{inner}))",
                    s_lit(label(el))
                ));
                (start, axis, length, outer_d, profile)
            } else {
                let af = pos(el.get("acrossFlatsMm"))?;
                let phase = el.get("phaseRad").and_then(Value::as_f64).unwrap_or(0.0);
                let radius = af / 3.0_f64.sqrt();
                let points = (0..6)
                    .map(|i| {
                        let a = phase + i as f64 * std::f64::consts::PI / 3.0;
                        [radius * a.cos(), radius * a.sin()]
                    })
                    .collect::<Vec<_>>();
                let outer = emit_polyline2(spf, &points, true);
                let profile = spf.emit(&format!(
                    "IFCARBITRARYCLOSEDPROFILEDEF(.AREA.,{},#{outer})",
                    s_lit(label(el))
                ));
                (start, axis, length, radius * 2.0, profile)
            }
        };
    Some(FastenerSweep {
        start,
        axis,
        length,
        bounding_diagonal: (length.powi(2) + 2.0 * cross_section_diameter.powi(2)).sqrt(),
        profile,
    })
}

fn mesh_bounding_diagonal(el: &Value) -> Option<f64> {
    let positions = el.get("positions")?.as_array()?;
    let mut min = [f64::INFINITY; 3];
    let mut max = [f64::NEG_INFINITY; 3];
    for point in positions.chunks_exact(3) {
        for axis in 0..3 {
            let value = point[axis].as_f64()?;
            min[axis] = min[axis].min(value);
            max[axis] = max[axis].max(value);
        }
    }
    Some(
        (0..3)
            .map(|axis| (max[axis] - min[axis]).powi(2))
            .sum::<f64>()
            .sqrt(),
    )
}

fn emit_positioned_sweep(
    spf: &mut Spf,
    sweep: &FastenerSweep,
    dir_z: i64,
    style: Option<i64>,
) -> i64 {
    let position = emit_axis2_placement3d(spf, sweep.start, sweep.axis, axis_ref_dir(sweep.axis));
    let solid = spf.emit(&format!(
        "IFCEXTRUDEDAREASOLID(#{},#{position},#{dir_z},{})",
        sweep.profile,
        r(sweep.length)
    ));
    if let Some(st) = style {
        spf.emit(&format!("IFCSTYLEDITEM(#{solid},(#{st}),$)"));
    }
    solid
}

fn receipt_row(id: &str, kind: &str, status: &str) -> serde_json::Map<String, Value> {
    let mut row = serde_json::Map::new();
    row.insert("id".into(), Value::String(id.to_string()));
    row.insert("kind".into(), Value::String(kind.to_string()));
    row.insert("status".into(), Value::String(status.to_string()));
    row
}

fn bolt_component_ids(instance: &Value) -> Vec<&str> {
    let mut ids = Vec::new();
    for field in ["shankId", "headId"] {
        if let Some(id) = instance.get(field).and_then(Value::as_str) {
            ids.push(id);
        }
    }
    for field in ["nutIds", "washerIds"] {
        if let Some(values) = instance.get(field).and_then(Value::as_array) {
            ids.extend(values.iter().filter_map(Value::as_str));
        }
    }
    ids
}

fn mechanical_semantics(el: &Value) -> (String, &'static str) {
    let role = el
        .get("fastener")
        .or_else(|| el.get("fastenerSemantics"))
        .or_else(|| el.get("profile"))
        .and_then(Value::as_object)
        .and_then(|o| o.get("role"))
        .and_then(Value::as_str)
        .unwrap_or("")
        .to_ascii_lowercase();
    let kind = el.get("kind").and_then(Value::as_str).unwrap_or("");
    match kind {
        "washer" => ("Washer".into(), ".USERDEFINED."),
        "bolt-head" => ("Bolt head".into(), ".USERDEFINED."),
        "nut" => match role.as_str() {
            "leveling-nut" | "leveling nut" => ("Leveling nut".into(), ".USERDEFINED."),
            "anchor-nut" | "anchor nut" => ("Anchor nut".into(), ".USERDEFINED."),
            "nut" => ("Nut".into(), ".USERDEFINED."),
            _ => ("Hex fastener".into(), ".USERDEFINED."),
        },
        "rod" | "bolt-shank" => match role.as_str() {
            "anchor" | "anchor-rod" | "anchor-bolt" | "anchorbolt" => {
                ("Anchor bolt".into(), ".ANCHORBOLT.")
            }
            "bolt" | "shear-bolt" | "shear bolt" | "bolt-shank" => ("Bolt".into(), ".BOLT."),
            _ if kind == "bolt-shank" => ("Bolt".into(), ".BOLT."),
            _ => ("Round fastener".into(), ".NOTDEFINED."),
        },
        _ => ("Mechanical fastener".into(), ".NOTDEFINED."),
    }
}

/// The result of building an IFC document (pure — no IO).
struct BuildResult {
    doc: String,
    members: usize,
    columns: i64,
    beams: i64,
    meshes: i64,
    profiles: BTreeMap<String, i64>,
    /// Neutral, deterministic (scene-order) warnings for elements whose PRESENT `xsection` was
    /// rejected and fell back to a rectangle. Empty for generic/rect scenes.
    warnings: Vec<(String, String)>,
    emitted: Vec<Value>,
    failed: Vec<Value>,
    unsupported: Vec<Value>,
}

/// Build the full IFC4 SPF document from a scene object. Pure (no IO) so it is directly unit-testable.
fn try_build_ifc(scene: &Value) -> Result<BuildResult, AwareError> {
    let meta = scene.get("meta").and_then(Value::as_object);
    let proj_name = meta
        .and_then(|m| m.get("name"))
        .and_then(Value::as_str)
        .unwrap_or("Model");
    // Bind the artifact to the exact exported scene so a stale IFC with matching ids cannot pass
    // reconciliation — FloLess re-checks these against the scene it sent. (Codex plan-review R4 #1.)
    let source_id = meta
        .and_then(|m| m.get("sourceId"))
        .and_then(Value::as_str)
        .unwrap_or("");
    let scene_hash = meta
        .and_then(|m| m.get("sceneHash"))
        .and_then(Value::as_str)
        .unwrap_or("");

    let mut spf = Spf::new();

    // ── Phase 1: shared geometry primitives + the spatial spine ──
    let origin3d = spf.emit("IFCCARTESIANPOINT((0.,0.,0.))");
    let dir_z = spf.emit("IFCDIRECTION((0.,0.,1.))");
    let dir_x = spf.emit("IFCDIRECTION((1.,0.,0.))");
    let world_axes = spf.emit(&format!(
        "IFCAXIS2PLACEMENT3D(#{origin3d},#{dir_z},#{dir_x})"
    ));
    let ctx = spf.emit(&format!(
        "IFCGEOMETRICREPRESENTATIONCONTEXT($,'Model',3,1.0E-5,#{world_axes},$)"
    ));
    let origin2d = spf.emit("IFCCARTESIANPOINT((0.,0.))");
    let pos2d = spf.emit(&format!("IFCAXIS2PLACEMENT2D(#{origin2d},$)"));
    let extrude_pos = spf.emit(&format!("IFCAXIS2PLACEMENT3D(#{origin3d},$,$)"));
    let unit_l = spf.emit("IFCSIUNIT(*,.LENGTHUNIT.,.MILLI.,.METRE.)");
    let unit_a = spf.emit("IFCSIUNIT(*,.AREAUNIT.,$,.SQUARE_METRE.)");
    let unit_v = spf.emit("IFCSIUNIT(*,.VOLUMEUNIT.,$,.CUBIC_METRE.)");
    let units = spf.emit(&format!(
        "IFCUNITASSIGNMENT((#{unit_l},#{unit_a},#{unit_v}))"
    ));
    let world_place = spf.emit(&format!("IFCLOCALPLACEMENT($,#{world_axes})"));
    let g = guid(spf.id);
    let project = spf.emit(&format!(
        "IFCPROJECT({},$,{},$,$,$,$,(#{ctx}),#{units})",
        s_lit(&g),
        s_lit(proj_name)
    ));
    if !source_id.is_empty() || !scene_hash.is_empty() {
        let p_src = spf.emit(&format!(
            "IFCPROPERTYSINGLEVALUE('sourceId',$,IFCIDENTIFIER({}),$)",
            s_lit(source_id)
        ));
        let p_scn = spf.emit(&format!(
            "IFCPROPERTYSINGLEVALUE('sceneHash',$,IFCIDENTIFIER({}),$)",
            s_lit(scene_hash)
        ));
        let g = guid(spf.id);
        let pset = spf.emit(&format!(
            "IFCPROPERTYSET({},$,'Pset_AWARE_ExportIdentity',$,(#{p_src},#{p_scn}))",
            s_lit(&g)
        ));
        let g = guid(spf.id);
        spf.emit(&format!(
            "IFCRELDEFINESBYPROPERTIES({},$,$,$,(#{project}),#{pset})",
            s_lit(&g)
        ));
    }
    let g = guid(spf.id);
    let site = spf.emit(&format!(
        "IFCSITE({},$,'Site',$,$,#{world_place},$,$,.ELEMENT.,$,$,$,$,$)",
        s_lit(&g)
    ));
    let site_place = spf.emit(&format!("IFCLOCALPLACEMENT(#{world_place},#{world_axes})"));
    let g = guid(spf.id);
    let building = spf.emit(&format!(
        "IFCBUILDING({},$,'Building',$,$,#{site_place},$,$,.ELEMENT.,$,$,$)",
        s_lit(&g)
    ));
    let bldg_place = spf.emit(&format!("IFCLOCALPLACEMENT(#{site_place},#{world_axes})"));
    let g = guid(spf.id);
    let storey = spf.emit(&format!(
        "IFCBUILDINGSTOREY({},$,'Storey',$,$,#{bldg_place},$,$,.ELEMENT.,0.)",
        s_lit(&g)
    ));
    let g = guid(spf.id);
    spf.emit(&format!(
        "IFCRELAGGREGATES({},$,$,$,#{project},(#{site}))",
        s_lit(&g)
    ));
    let g = guid(spf.id);
    spf.emit(&format!(
        "IFCRELAGGREGATES({},$,$,$,#{site},(#{building}))",
        s_lit(&g)
    ));
    let g = guid(spf.id);
    spf.emit(&format!(
        "IFCRELAGGREGATES({},$,$,$,#{building},(#{storey}))",
        s_lit(&g)
    ));

    // group -> colour (first-seen wins on a duplicate key).
    let mut group_colors: BTreeMap<String, String> = BTreeMap::new();
    if let Some(groups) = scene.get("groups").and_then(Value::as_array) {
        for go in groups {
            if let (Some(k), Some(c)) = (
                go.get("key").and_then(Value::as_str),
                go.get("color").and_then(Value::as_str),
            ) {
                group_colors
                    .entry(k.to_string())
                    .or_insert_with(|| c.to_string());
            }
        }
    }

    let empty: Vec<Value> = Vec::new();
    let elements = scene
        .get("elements")
        .and_then(Value::as_array)
        .unwrap_or(&empty);
    let element_by_id = elements
        .iter()
        .filter_map(|element| {
            element
                .get("id")
                .and_then(Value::as_str)
                .map(|id| (id.to_string(), element))
        })
        .collect::<BTreeMap<_, _>>();
    let mut grouped_fastener_component_ids = BTreeSet::new();
    if let Some(operations) = scene.get("operations").and_then(Value::as_array) {
        for op in operations
            .iter()
            .filter(|op| op.get("kind").and_then(Value::as_str) == Some("bolt-array"))
        {
            if let Some(instances) = op.get("instances").and_then(Value::as_array) {
                for instance in instances {
                    grouped_fastener_component_ids.extend(
                        bolt_component_ids(instance)
                            .into_iter()
                            .map(ToOwned::to_owned),
                    );
                }
            }
        }
    }

    // Pre-scan for the distinct colours + materials actually used (deterministic BTreeMap order).
    let mut colors: BTreeMap<String, (f64, f64, f64)> = BTreeMap::new();
    let mut materials: BTreeMap<String, String> = BTreeMap::new();
    for el in elements {
        if let Some((hk, rgb)) = resolve_color(el, &group_colors) {
            colors.entry(hk).or_insert(rgb);
        }
        if let Some((mk, disp)) = resolve_material(el) {
            materials.entry(mk).or_insert(disp);
        }
    }

    // ── Phase 2a: shared, deduped colour styles ──
    let mut color_style: BTreeMap<String, i64> = BTreeMap::new();
    for (hk, (cr, cg, cb)) in &colors {
        let rgb = spf.emit(&format!("IFCCOLOURRGB($,{},{},{})", r(*cr), r(*cg), r(*cb)));
        let shade = spf.emit(&format!("IFCSURFACESTYLESHADING(#{rgb},$)"));
        let style = spf.emit(&format!("IFCSURFACESTYLE($,.BOTH.,(#{shade}))"));
        color_style.insert(hk.clone(), style);
    }
    // ── Phase 2b: shared, deduped materials ──
    let mut mat_id: BTreeMap<String, i64> = BTreeMap::new();
    for (mk, disp) in &materials {
        let mid = spf.emit(&format!("IFCMATERIAL({},$,$)", s_lit(disp)));
        mat_id.insert(mk.clone(), mid);
    }

    // ── Phase 3: one IFC element per scene element ──
    let mut elem_ids: Vec<i64> = Vec::new();
    let mut columns = 0i64;
    let mut beams = 0i64;
    let mut meshes = 0i64;
    let mut profile_counts: BTreeMap<String, i64> = BTreeMap::new();
    let mut mat_members: BTreeMap<String, Vec<i64>> = BTreeMap::new();
    let mut warnings: Vec<(String, String)> = Vec::new();
    let mut emitted: Vec<Value> = Vec::new();
    let mut failed: Vec<Value> = Vec::new();
    let mut unsupported: Vec<Value> = Vec::new();
    let mut product_by_id: BTreeMap<String, i64> = BTreeMap::new();
    // Conservative maximum distance from an authored hole centre to any point
    // on the target. Opening solids use twice this span so oblique axes still
    // pass completely through the product.
    let mut product_half_span_by_id: BTreeMap<String, f64> = BTreeMap::new();

    for el in elements {
        let elid = el
            .get("id")
            .and_then(Value::as_str)
            .unwrap_or("")
            .to_string();
        let style =
            resolve_color(el, &group_colors).and_then(|(hk, _)| color_style.get(&hk).copied());
        let matkey = resolve_material(el).map(|(mk, _)| mk);

        let kind = el.get("kind").and_then(Value::as_str).unwrap_or("member");
        if grouped_fastener_component_ids.contains(&elid) {
            continue;
        }
        if !matches!(
            kind,
            "member"
                | "line"
                | "box"
                | "plate"
                | "rod"
                | "bolt-shank"
                | "washer"
                | "nut"
                | "bolt-head"
                | "mesh"
        ) {
            let mut row = receipt_row(&elid, kind, "unsupported");
            row.insert(
                "code".into(),
                Value::String("unsupported-element-kind".into()),
            );
            row.insert(
                "message".into(),
                Value::String(format!(
                    "element kind '{kind}' is not supported by ifc.write"
                )),
            );
            unsupported.push(Value::Object(row));
            continue;
        }
        if kind == "plate" {
            let Some(frame) = el.get("frame").and_then(Value::as_object) else {
                continue;
            };
            let Some(origin) = vec3(frame.get("origin")) else {
                continue;
            };
            let Some(u) = vec3(frame.get("uDir")).and_then(normalized3) else {
                continue;
            };
            let Some(_v) = vec3(frame.get("vDir")).and_then(normalized3) else {
                continue;
            };
            let Some(normal) = vec3(frame.get("normal")).and_then(normalized3) else {
                continue;
            };
            let Some(thickness) = pos(el.get("thicknessMm")) else {
                continue;
            };
            let Some(outline) = el.get("outline").and_then(Value::as_array) else {
                continue;
            };
            let outline = outline
                .iter()
                .filter_map(|point| vec2(Some(point)))
                .collect::<Vec<_>>();
            if outline.len() < 3 {
                continue;
            }
            let outer = emit_polyline2(&mut spf, &outline, true);
            let mut inner_curves = Vec::new();
            if let Some(holes) = el.get("holes").and_then(Value::as_array) {
                for hole in holes {
                    let Some([x, y]) = vec2(hole.get("center")) else {
                        continue;
                    };
                    let Some(diameter) = pos(hole.get("diameterMm")) else {
                        continue;
                    };
                    let hp = spf.emit(&format!("IFCCARTESIANPOINT(({},{}))", r(x), r(y)));
                    let ha = spf.emit(&format!("IFCAXIS2PLACEMENT2D(#{hp},$)"));
                    inner_curves.push(spf.emit(&format!("IFCCIRCLE(#{ha},{})", r(diameter / 2.0))));
                }
            }
            let profile_name = format!("{} profile", label(el));
            let profile = if inner_curves.is_empty() {
                spf.emit(&format!(
                    "IFCARBITRARYCLOSEDPROFILEDEF(.AREA.,{},#{outer})",
                    s_lit(&profile_name)
                ))
            } else {
                let refs = inner_curves
                    .iter()
                    .map(|id| format!("#{id}"))
                    .collect::<Vec<_>>()
                    .join(",");
                spf.emit(&format!(
                    "IFCARBITRARYPROFILEDEFWITHVOIDS(.AREA.,{},#{outer},({refs}))",
                    s_lit(&profile_name)
                ))
            };
            let start = add3(origin, scale3(normal, -thickness / 2.0));
            let place = emit_axis_placement(&mut spf, start, normal, u, bldg_place);
            let pds = emit_body(&mut spf, profile, thickness, ctx, extrude_pos, dir_z, style);
            let gid = record_guid("element", &elid);
            let elem = spf.emit(&format!(
                "IFCPLATE({},$,{},$,$,#{place},#{pds},$,$)",
                s_lit(&gid),
                s_lit(label(el))
            ));
            elem_ids.push(elem);
            product_by_id.insert(elid.clone(), elem);
            let (min_x, max_x, min_y, max_y) = outline.iter().fold(
                (
                    f64::INFINITY,
                    f64::NEG_INFINITY,
                    f64::INFINITY,
                    f64::NEG_INFINITY,
                ),
                |(min_x, max_x, min_y, max_y), [x, y]| {
                    (min_x.min(*x), max_x.max(*x), min_y.min(*y), max_y.max(*y))
                },
            );
            product_half_span_by_id.insert(
                elid.clone(),
                ((max_x - min_x).powi(2) + (max_y - min_y).powi(2) + thickness.powi(2)).sqrt(),
            );
            *profile_counts.entry("PLATE".into()).or_insert(0) += 1;
            if let Some(mk) = matkey {
                mat_members.entry(mk).or_default().push(elem);
            }
            let mut row = receipt_row(&elid, kind, "emitted");
            row.insert("ifcGuid".into(), Value::String(gid.clone()));
            row.insert("entityType".into(), Value::String("IfcPlate".into()));
            emitted.push(Value::Object(row));
            if let Some(holes) = el.get("holes").and_then(Value::as_array) {
                for hole in holes {
                    let hid = hole.get("id").and_then(Value::as_str).unwrap_or("");
                    let mut row = receipt_row(hid, "hole", "emitted");
                    row.insert("realizedBy".into(), Value::String(elid.clone()));
                    row.insert("ifcGuid".into(), Value::String(gid.clone()));
                    emitted.push(Value::Object(row));
                }
            }
            continue;
        }

        if matches!(kind, "rod" | "bolt-shank" | "washer" | "nut" | "bolt-head") {
            let Some(sweep) = emit_fastener_sweep(&mut spf, el, pos2d) else {
                continue;
            };
            let place = emit_axis_placement(
                &mut spf,
                sweep.start,
                sweep.axis,
                axis_ref_dir(sweep.axis),
                bldg_place,
            );
            let pds = emit_body(
                &mut spf,
                sweep.profile,
                sweep.length,
                ctx,
                extrude_pos,
                dir_z,
                style,
            );
            let (object_type, predefined) = mechanical_semantics(el);
            let gid = record_guid("element", &elid);
            let elem = spf.emit(&format!(
                "IFCMECHANICALFASTENER({},$,{},$,{},#{place},#{pds},$,$,$,{predefined})",
                s_lit(&gid),
                s_lit(label(el)),
                s_lit(&object_type)
            ));
            elem_ids.push(elem);
            product_by_id.insert(elid.clone(), elem);
            product_half_span_by_id.insert(elid.clone(), sweep.bounding_diagonal.max(1.0));
            *profile_counts.entry(kind.to_ascii_uppercase()).or_insert(0) += 1;
            if let Some(mk) = matkey {
                mat_members.entry(mk).or_default().push(elem);
            }
            let mut row = receipt_row(&elid, kind, "emitted");
            row.insert("ifcGuid".into(), Value::String(gid));
            row.insert(
                "entityType".into(),
                Value::String("IfcMechanicalFastener".into()),
            );
            emitted.push(Value::Object(row));
            continue;
        }

        // A tessellated mesh element (e.g. an imported connection).
        let is_mesh =
            el.get("kind").and_then(Value::as_str) == Some("mesh") || el.get("positions").is_some();
        if is_mesh {
            if let Some(elem) = emit_mesh(&mut spf, el, bldg_place, ctx, style) {
                elem_ids.push(elem);
                product_by_id.insert(elid.clone(), elem);
                if let Some(diagonal) = mesh_bounding_diagonal(el) {
                    product_half_span_by_id.insert(elid.clone(), diagonal.max(1.0));
                }
                meshes += 1;
                let profile = el
                    .get("meta")
                    .and_then(Value::as_object)
                    .and_then(|m| m.get("profile"))
                    .and_then(Value::as_str)
                    .unwrap_or("MESH")
                    .to_string();
                *profile_counts.entry(profile).or_insert(0) += 1;
                if let Some(mk) = matkey {
                    mat_members.entry(mk).or_default().push(elem);
                }
                let gid = record_guid("element", &elid);
                let mut row = receipt_row(&elid, "mesh", "emitted");
                row.insert("ifcGuid".into(), Value::String(gid));
                row.insert(
                    "entityType".into(),
                    Value::String("IfcBuildingElementProxy".into()),
                );
                emitted.push(Value::Object(row));
            }
            continue;
        }

        let from = el.get("from").and_then(Value::as_array);
        let to = el.get("to").and_then(Value::as_array);
        let (from, to) = match (from, to) {
            (Some(f), Some(t)) if f.len() >= 3 && t.len() >= 3 => (f, t),
            _ => continue,
        };
        let (x1, y1, z1) = (num(from.first()), num(from.get(1)), num(from.get(2)));
        let (x2, y2, z2) = (num(to.first()), num(to.get(1)), num(to.get(2)));
        let (dx, dy, dz) = (x2 - x1, y2 - y1, z2 - z1);
        let len = (dx * dx + dy * dy + dz * dz).sqrt();
        if len < 1e-6 {
            continue; // degenerate member
        }
        let rot = member_roll(el.get("rot"), "scene element rot", "ifc write")?;
        let frame = member_frame([x1, y1, z1], [x2, y2, z2], rot, SceneUp::Z)?;
        debug_assert!(dot3(frame.axis, frame.zero_y).abs() <= 1e-10);
        debug_assert!(dot3(frame.axis, frame.rolled_x).abs() <= 1e-10);
        debug_assert!(dot3(frame.axis, frame.rolled_y).abs() <= 1e-10);
        let [zx, zy, zz] = frame.axis;
        let [xx, xy, xz] = frame.zero_x;

        let sec = el.get("section").and_then(Value::as_object);
        let mut w = sec
            .and_then(|s| s.get("w"))
            .and_then(Value::as_f64)
            .unwrap_or(100.0);
        let mut d = sec
            .and_then(|s| s.get("d"))
            .and_then(Value::as_f64)
            .unwrap_or(100.0);
        if w <= 0.0 {
            w = 100.0;
        }
        if d <= 0.0 {
            d = 100.0;
        }
        let profile = el
            .get("meta")
            .and_then(Value::as_object)
            .and_then(|m| m.get("profile"))
            .and_then(Value::as_str)
            .unwrap_or("RECT")
            .to_string();
        let group = el
            .get("group")
            .and_then(Value::as_str)
            .unwrap_or("")
            .to_lowercase();
        let role = el
            .get("role")
            .and_then(Value::as_str)
            .unwrap_or("")
            .to_lowercase();
        let name = el
            .get("id")
            .and_then(Value::as_str)
            .unwrap_or(&profile)
            .to_string();

        let p1 = spf.emit(&format!(
            "IFCCARTESIANPOINT(({},{},{}))",
            r(x1),
            r(y1),
            r(z1)
        ));
        let az = spf.emit(&format!("IFCDIRECTION(({},{},{}))", r(zx), r(zy), r(zz)));
        let ax = spf.emit(&format!("IFCDIRECTION(({},{},{}))", r(xx), r(xy), r(xz)));
        let a2p = spf.emit(&format!("IFCAXIS2PLACEMENT3D(#{p1},#{az},#{ax})"));
        let place = spf.emit(&format!("IFCLOCALPLACEMENT(#{bldg_place},#{a2p})"));

        // Section roll: `rot` is scene-space degrees (already reflection-corrected upstream). Apply
        // ONCE, positive, via the profile's 2D RefDirection. rot 0/absent → the shared identity pos2d.
        let rot = frame.normalized_degrees;
        let prof_pos = if rot.abs() > 1e-9 {
            let th = rot.to_radians();
            let rd = spf.emit(&format!("IFCDIRECTION(({},{}))", r(th.cos()), r(th.sin())));
            spf.emit(&format!("IFCAXIS2PLACEMENT2D(#{origin2d},#{rd})"))
        } else {
            pos2d
        };

        let (prof, warn) =
            emit_profile(&mut spf, el.get("xsection"), w, d, &profile, prof_pos, rot);
        if let Some(reason) = warn {
            warnings.push((elid.clone(), reason));
        }
        let solid = spf.emit(&format!(
            "IFCEXTRUDEDAREASOLID(#{prof},#{extrude_pos},#{dir_z},{})",
            r(len)
        ));
        if let Some(st) = style {
            spf.emit(&format!("IFCSTYLEDITEM(#{solid},(#{st}),$)"));
        }
        let shape = spf.emit(&format!(
            "IFCSHAPEREPRESENTATION(#{ctx},'Body','SweptSolid',(#{solid}))"
        ));
        let pds = spf.emit(&format!("IFCPRODUCTDEFINITIONSHAPE($,$,(#{shape}))"));
        let g = record_guid("element", &elid);
        // Element type from the neutral `role`; fall back to the legacy `group` when role is absent.
        let is_col = if role.is_empty() {
            group == "column"
        } else {
            role == "column"
        };
        let is_brace = if role.is_empty() {
            group == "brace"
        } else {
            role == "brace"
        };
        let (ifc_type, pdt) = if is_col {
            columns += 1;
            ("IFCCOLUMN", ".COLUMN.")
        } else if is_brace {
            ("IFCMEMBER", ".BRACE.")
        } else {
            beams += 1;
            ("IFCBEAM", ".BEAM.")
        };
        *profile_counts.entry(profile.clone()).or_insert(0) += 1;
        let elem = spf.emit(&format!(
            "{ifc_type}({},$,{},$,{},#{place},#{pds},$,{pdt})",
            s_lit(&g),
            s_lit(&name),
            s_lit(&profile)
        ));
        elem_ids.push(elem);
        product_by_id.insert(elid.clone(), elem);
        product_half_span_by_id.insert(elid.clone(), (len * len + w * w + d * d).sqrt());
        if let Some(mk) = matkey {
            mat_members.entry(mk).or_default().push(elem);
        }
        let mut row = receipt_row(&elid, kind, "emitted");
        row.insert("ifcGuid".into(), Value::String(g));
        row.insert(
            "entityType".into(),
            Value::String(
                match ifc_type {
                    "IFCCOLUMN" => "IfcColumn",
                    "IFCMEMBER" => "IfcMember",
                    _ => "IfcBeam",
                }
                .into(),
            ),
        );
        // Present only on members whose zero frame came from the vertical seed, where the
        // section's facing is fixed by convention rather than derived from the member's
        // own geometry (`scene_roll::member_frame` documents why that band exists and why
        // it cannot be removed). Absent means projected-up, so the field is additive and
        // an existing reader is unaffected. A consumer reconciling this file against
        // another toolchain reads it to see which members are the ones whose facing two
        // conforming implementations can legitimately disagree about — issue #432.
        if frame.zero_frame_source == ZeroFrameSource::VerticalSeed {
            row.insert("zeroFrame".into(), Value::String("vertical-seed".into()));
        }
        emitted.push(Value::Object(row));
    }

    // ── Phase 4: spatial containment ──
    // Native-operation intent creates one selectable IFC product per bolt array. The authored
    // heads, shanks, washers, and nuts remain exact solids inside that shared product.
    if let Some(operations) = scene.get("operations").and_then(Value::as_array) {
        for op in operations {
            let op_id = op.get("id").and_then(Value::as_str).unwrap_or("");
            let op_kind = op.get("kind").and_then(Value::as_str).unwrap_or("unknown");

            // A finite Boolean cut (a cope or a drilled opening) becomes an IfcVoidingFeature voided from its
            // target, mirroring the proven bolt-hole-effect void pattern (building-relative placement + an
            // IfcRelVoidsElement). Box -> .NOTCH. with the true filleted profile; cylinder -> .HOLE.
            if op_kind == "boolean-cut" {
                let target_id = op.get("targetId").and_then(Value::as_str).unwrap_or("");
                let tool = op.get("tool").and_then(Value::as_object);
                let kind = tool
                    .and_then(|t| t.get("kind"))
                    .and_then(Value::as_str)
                    .unwrap_or("");
                let target = product_by_id.get(target_id).copied();
                let built: Option<(i64, i64)> = (|| {
                    let tool = tool?;
                    if kind == "box" {
                        let frame = tool.get("frame").and_then(Value::as_object)?;
                        let origin = vec3(frame.get("origin"))?;
                        let u = vec3(frame.get("uDir")).and_then(normalized3)?;
                        let normal = vec3(frame.get("normal")).and_then(normalized3)?;
                        let he = tool.get("halfExtents")?.as_array()?;
                        let hu = he.first().and_then(Value::as_f64)?;
                        let hv = he.get(1).and_then(Value::as_f64)?;
                        let hn = he.get(2).and_then(Value::as_f64)?;
                        if !(hu > 0.0 && hv > 0.0 && hn > 0.0) {
                            return None;
                        }
                        let reentrant =
                            tool.get("reentrant")
                                .and_then(Value::as_object)
                                .and_then(|re| {
                                    Some((
                                        re.get("radiusMm").and_then(Value::as_f64)?,
                                        re.get("vSign").and_then(Value::as_f64)? as i64,
                                    ))
                                });
                        let profile =
                            emit_box_cut_profile(&mut spf, pos2d, hu, hv, reentrant, op_id);
                        let base = add3(origin, scale3(normal, -hn)); // base at origin - normal*hn (Codex R3 #6)
                        let place = emit_axis_placement(&mut spf, base, normal, u, bldg_place);
                        let pds =
                            emit_body(&mut spf, profile, 2.0 * hn, ctx, extrude_pos, dir_z, None);
                        Some((place, pds))
                    } else if kind == "cylinder" {
                        let axis = tool.get("axis").and_then(Value::as_object)?;
                        let from = vec3(axis.get("from"))?;
                        let to = vec3(axis.get("to"))?;
                        let delta = [to[0] - from[0], to[1] - from[1], to[2] - from[2]];
                        let dir = normalized3(delta)?;
                        let len = length3(delta);
                        let dia = pos(tool.get("diameterMm"))?;
                        // Deliberately negated rather than `len <= 0.0`: this must also reject a
                        // NaN length, and every comparison against NaN is false, so `!(len > 0.0)`
                        // catches it while `len <= 0.0` would wave it through into the geometry.
                        #[allow(clippy::neg_cmp_op_on_partial_ord)]
                        if !(len > 0.0) {
                            return None;
                        }
                        let circle = spf.emit(&format!("IFCCIRCLE(#{pos2d},{})", r(dia / 2.0)));
                        let profile = spf.emit(&format!(
                            "IFCARBITRARYCLOSEDPROFILEDEF(.AREA.,{},#{circle})",
                            s_lit(op_id)
                        ));
                        let place =
                            emit_axis_placement(&mut spf, from, dir, axis_ref_dir(dir), bldg_place);
                        let pds = emit_body(&mut spf, profile, len, ctx, extrude_pos, dir_z, None);
                        Some((place, pds))
                    } else {
                        None
                    }
                })();
                match (target, built) {
                    (Some(target), Some((place, pds))) => {
                        let feat_type = if kind == "cylinder" {
                            ".HOLE."
                        } else {
                            ".NOTCH."
                        };
                        let gid = record_guid("voiding-feature", op_id);
                        let feature = spf.emit(&format!(
                            "IFCVOIDINGFEATURE({},$,{},$,$,#{place},#{pds},$,{feat_type})",
                            s_lit(&gid),
                            s_lit(op_id)
                        ));
                        let rel_gid =
                            record_guid("void-relation", &format!("{target_id}\0{op_id}"));
                        spf.emit(&format!(
                            "IFCRELVOIDSELEMENT({},$,$,$,#{target},#{feature})",
                            s_lit(&rel_gid)
                        ));
                        let mut row = receipt_row(op_id, op_kind, "emitted");
                        row.insert("ifcGuid".into(), Value::String(gid));
                        row.insert(
                            "entityType".into(),
                            Value::String("IfcVoidingFeature".into()),
                        );
                        row.insert("voids".into(), Value::String(target_id.to_string()));
                        row.insert("relationshipGuid".into(), Value::String(rel_gid));
                        row.insert(
                            "relationshipType".into(),
                            Value::String("IfcRelVoidsElement".into()),
                        );
                        emitted.push(Value::Object(row));
                    }
                    _ => {
                        let mut row = receipt_row(op_id, op_kind, "failed");
                        row.insert(
                            "code".into(),
                            Value::String("boolean-cut-materialization-failed".into()),
                        );
                        row.insert(
                            "message".into(),
                            Value::String(format!(
                                "boolean-cut '{op_id}' could not resolve its target or tool geometry"
                            )),
                        );
                        failed.push(Value::Object(row));
                    }
                }
                continue;
            }

            // A fillet weld becomes an IfcFastener(.WELD.) with an Axis (Curve3D) polyline, the standard
            // Pset_FastenerWeld leg (z), a custom pset for around/shop, and an IfcRelConnectsWithRealizingElements
            // realizing the connection between the two authored members. v1 carries no bead solid (the contract
            // has no weld face-normal to orient one truthfully). (Codex plan-review R2 #10/#11, R3 #9/#10, R4 #8.)
            if op_kind == "weld" {
                let main_id = op.get("mainId").and_then(Value::as_str).unwrap_or("");
                let sec_id = op.get("secondaryId").and_then(Value::as_str).unwrap_or("");
                let size = pos(op.get("sizeMm"));
                let around = op.get("around").and_then(Value::as_bool).unwrap_or(false);
                let shop = op.get("shop").and_then(Value::as_bool).unwrap_or(false);
                let mut pts: Vec<Vec3> = op
                    .get("path")
                    .and_then(Value::as_array)
                    .map(|a| a.iter().filter_map(|p| vec3(Some(p))).collect())
                    .unwrap_or_default();
                if around
                    && let (Some(&first), Some(&last)) = (pts.first(), pts.last())
                    && first != last
                {
                    pts.push(first); // close an around-weld only when the authored path is open (R4 #8)
                }
                let seg_len = |w: &[Vec3]| {
                    let d = (w[1][0] - w[0][0], w[1][1] - w[0][1], w[1][2] - w[0][2]);
                    (d.0 * d.0 + d.1 * d.1 + d.2 * d.2).sqrt()
                };
                let seg_ok = pts.windows(2).all(|w| seg_len(w) > 1e-6);
                let total: f64 = pts.windows(2).map(seg_len).sum();
                let main = product_by_id.get(main_id).copied();
                let sec = product_by_id.get(sec_id).copied();
                match (main, sec, size) {
                    (Some(main), Some(sec), Some(size))
                        if main_id != sec_id
                            && main != sec
                            && pts.len() >= 2
                            && seg_ok
                            && total > 1e-6 =>
                    {
                        let first = pts[0];
                        let local: Vec<Vec3> = pts
                            .iter()
                            .map(|p| [p[0] - first[0], p[1] - first[1], p[2] - first[2]])
                            .collect();
                        let place = emit_axis_placement(
                            &mut spf,
                            first,
                            [0.0, 0.0, 1.0],
                            [1.0, 0.0, 0.0],
                            bldg_place,
                        );
                        let axis_line = emit_polyline3(&mut spf, &local);
                        let shape = spf.emit(&format!(
                            "IFCSHAPEREPRESENTATION(#{ctx},'Axis','Curve3D',(#{axis_line}))"
                        ));
                        let pds = spf.emit(&format!("IFCPRODUCTDEFINITIONSHAPE($,$,(#{shape}))"));
                        let gid = record_guid("fastener-weld", op_id);
                        let fastener = spf.emit(&format!(
                            "IFCFASTENER({},$,{},$,$,#{place},#{pds},$,.WELD.)",
                            s_lit(&gid),
                            s_lit(&format!("Fillet weld {op_id}"))
                        ));
                        let p_z = spf.emit(&format!(
                            "IFCPROPERTYSINGLEVALUE('z',$,IFCPOSITIVELENGTHMEASURE({}),$)",
                            r(size)
                        ));
                        let g = guid(spf.id);
                        let pset = spf.emit(&format!(
                            "IFCPROPERTYSET({},$,'Pset_FastenerWeld',$,(#{p_z}))",
                            s_lit(&g)
                        ));
                        let g = guid(spf.id);
                        spf.emit(&format!(
                            "IFCRELDEFINESBYPROPERTIES({},$,$,$,(#{fastener}),#{pset})",
                            s_lit(&g)
                        ));
                        let p_around = spf.emit(&format!(
                            "IFCPROPERTYSINGLEVALUE('around',$,IFCBOOLEAN(.{}.),$)",
                            if around { "T" } else { "F" }
                        ));
                        let p_shop = spf.emit(&format!(
                            "IFCPROPERTYSINGLEVALUE('shop',$,IFCBOOLEAN(.{}.),$)",
                            if shop { "T" } else { "F" }
                        ));
                        let g = guid(spf.id);
                        let pset2 = spf.emit(&format!(
                            "IFCPROPERTYSET({},$,'Pset_AWARE_Weld',$,(#{p_around},#{p_shop}))",
                            s_lit(&g)
                        ));
                        let g = guid(spf.id);
                        spf.emit(&format!(
                            "IFCRELDEFINESBYPROPERTIES({},$,$,$,(#{fastener}),#{pset2})",
                            s_lit(&g)
                        ));
                        let rel_gid = record_guid("weld-realizes", op_id);
                        spf.emit(&format!(
                            "IFCRELCONNECTSWITHREALIZINGELEMENTS({},$,$,$,$,#{main},#{sec},(#{fastener}),$)",
                            s_lit(&rel_gid)
                        ));
                        let mut row = receipt_row(op_id, op_kind, "emitted");
                        row.insert("ifcGuid".into(), Value::String(gid));
                        row.insert("entityType".into(), Value::String("IfcFastener".into()));
                        row.insert(
                            "realizes".into(),
                            Value::Array(vec![
                                Value::String(main_id.to_string()),
                                Value::String(sec_id.to_string()),
                            ]),
                        );
                        row.insert("relationshipGuid".into(), Value::String(rel_gid));
                        row.insert(
                            "relationshipType".into(),
                            Value::String("IfcRelConnectsWithRealizingElements".into()),
                        );
                        emitted.push(Value::Object(row));
                    }
                    _ => {
                        let mut row = receipt_row(op_id, op_kind, "failed");
                        row.insert(
                            "code".into(),
                            Value::String("weld-materialization-failed".into()),
                        );
                        row.insert(
                            "message".into(),
                            Value::String(format!(
                                "weld '{op_id}' has unresolved participants or a degenerate path"
                            )),
                        );
                        failed.push(Value::Object(row));
                    }
                }
                continue;
            }

            if op_kind != "bolt-array" {
                let mut row = receipt_row(op_id, op_kind, "unsupported");
                row.insert(
                    "code".into(),
                    Value::String("ifc-relationship-unsupported".into()),
                );
                row.insert(
                    "message".into(),
                    Value::String(format!(
                        "IFC4 mapping for {op_kind} relationships is not supported"
                    )),
                );
                unsupported.push(Value::Object(row));
                continue;
            }

            let group_gid = record_guid("bolt-array", op_id);
            let mut solid_ids = Vec::new();
            let mut component_rows = Vec::new();
            let mut group_material = None;
            let mut shank_length = None;
            if let Some(instances) = op.get("instances").and_then(Value::as_array) {
                for instance in instances {
                    for component_id in bolt_component_ids(instance) {
                        let Some(component) = element_by_id.get(component_id).copied() else {
                            continue;
                        };
                        let style = resolve_color(component, &group_colors)
                            .and_then(|(key, _)| color_style.get(&key).copied());
                        let Some(sweep) = emit_fastener_sweep(&mut spf, component, pos2d) else {
                            continue;
                        };
                        if matches!(
                            component.get("kind").and_then(Value::as_str),
                            Some("rod" | "bolt-shank")
                        ) && shank_length.is_none()
                        {
                            shank_length = Some(sweep.length);
                        }
                        solid_ids.push(emit_positioned_sweep(&mut spf, &sweep, dir_z, style));
                        if group_material.is_none() {
                            group_material = resolve_material(component).map(|(key, _)| key);
                        }
                        let component_kind = component
                            .get("kind")
                            .and_then(Value::as_str)
                            .unwrap_or("mechanical-fastener");
                        *profile_counts
                            .entry(component_kind.to_ascii_uppercase())
                            .or_insert(0) += 1;
                        let mut row = receipt_row(component_id, component_kind, "emitted");
                        row.insert("ifcGuid".into(), Value::String(group_gid.clone()));
                        row.insert("realizedBy".into(), Value::String(op_id.to_string()));
                        row.insert(
                            "entityType".into(),
                            Value::String("IfcMechanicalFastener".into()),
                        );
                        component_rows.push(Value::Object(row));
                    }
                }
            }
            let solid_refs = solid_ids
                .iter()
                .map(|id| format!("#{id}"))
                .collect::<Vec<_>>()
                .join(",");
            let shape = spf.emit(&format!(
                "IFCSHAPEREPRESENTATION(#{ctx},'Body','SweptSolid',({solid_refs}))"
            ));
            let pds = spf.emit(&format!("IFCPRODUCTDEFINITIONSHAPE($,$,(#{shape}))"));
            let group_place = emit_axis_placement(
                &mut spf,
                [0.0, 0.0, 0.0],
                [0.0, 0.0, 1.0],
                [1.0, 0.0, 0.0],
                bldg_place,
            );
            let standard = op.get("standard").and_then(Value::as_str).unwrap_or("");
            let name = op
                .get("name")
                .and_then(Value::as_str)
                .map(ToOwned::to_owned)
                .unwrap_or_else(|| {
                    if standard.is_empty() {
                        format!("Bolt array {op_id}")
                    } else {
                        format!("Bolt array {op_id} - {standard}")
                    }
                });
            let diameter = pos(op.get("diameterMm"))
                .map(r)
                .unwrap_or_else(|| "$".into());
            let nominal_length = shank_length.map(r).unwrap_or_else(|| "$".into());
            let fastener = spf.emit(&format!(
                "IFCMECHANICALFASTENER({},$,{},$,'Bolt array',#{group_place},#{pds},$,{diameter},{nominal_length},.BOLT.)",
                s_lit(&group_gid),
                s_lit(&name)
            ));
            elem_ids.push(fastener);
            product_by_id.insert(op_id.to_string(), fastener);
            if let Some(material) = group_material {
                mat_members.entry(material).or_default().push(fastener);
            }
            let mut op_row = receipt_row(op_id, op_kind, "emitted");
            op_row.insert("ifcGuid".into(), Value::String(group_gid.clone()));
            op_row.insert(
                "entityType".into(),
                Value::String("IfcMechanicalFastener".into()),
            );
            emitted.push(Value::Object(op_row));
            emitted.extend(component_rows);
            if let Some(instances) = op.get("instances").and_then(Value::as_array) {
                for instance in instances {
                    let instance_id = instance.get("id").and_then(Value::as_str).unwrap_or("");
                    let mut instance_row = receipt_row(instance_id, "bolt-instance", "emitted");
                    instance_row.insert("ifcGuid".into(), Value::String(group_gid.clone()));
                    instance_row.insert("realizedBy".into(), Value::String(op_id.to_string()));
                    emitted.push(Value::Object(instance_row));
                    if let Some(effects) = instance.get("holeEffects").and_then(Value::as_array) {
                        for effect in effects {
                            let effect_id = effect.get("id").and_then(Value::as_str).unwrap_or("");
                            let target_id =
                                effect.get("targetId").and_then(Value::as_str).unwrap_or("");
                            let Some(target) = product_by_id.get(target_id).copied() else {
                                continue;
                            };
                            let Some(center) = vec3(effect.get("center")) else {
                                continue;
                            };
                            let Some(axis) = vec3(effect.get("axis")).and_then(normalized3) else {
                                continue;
                            };
                            let Some(diameter) = pos(effect.get("diameterMm")) else {
                                continue;
                            };
                            let depth = product_half_span_by_id
                                .get(target_id)
                                .copied()
                                .unwrap_or(diameter * 4.0)
                                * 2.0
                                + 2.0;
                            let circle =
                                spf.emit(&format!("IFCCIRCLE(#{pos2d},{})", r(diameter / 2.0)));
                            let profile = spf.emit(&format!(
                                "IFCARBITRARYCLOSEDPROFILEDEF(.AREA.,{},#{circle})",
                                s_lit(effect_id)
                            ));
                            let start = add3(center, scale3(axis, -depth / 2.0));
                            let place = emit_axis_placement(
                                &mut spf,
                                start,
                                axis,
                                axis_ref_dir(axis),
                                bldg_place,
                            );
                            let pds =
                                emit_body(&mut spf, profile, depth, ctx, extrude_pos, dir_z, None);
                            let gid = record_guid("opening", effect_id);
                            let opening = spf.emit(&format!(
                                "IFCOPENINGELEMENT({},$,{},$,$,#{place},#{pds},$,.OPENING.)",
                                s_lit(&gid),
                                s_lit(effect_id)
                            ));
                            let rel_gid =
                                record_guid("void-relation", &format!("{target_id}\0{effect_id}"));
                            spf.emit(&format!(
                                "IFCRELVOIDSELEMENT({},$,$,$,#{target},#{opening})",
                                s_lit(&rel_gid)
                            ));
                            let mut row = receipt_row(effect_id, "hole-effect", "emitted");
                            row.insert("ifcGuid".into(), Value::String(gid));
                            row.insert("realizedBy".into(), Value::String(op_id.to_string()));
                            row.insert("targetId".into(), Value::String(target_id.to_string()));
                            row.insert(
                                "entityType".into(),
                                Value::String("IfcOpeningElement".into()),
                            );
                            emitted.push(Value::Object(row));
                        }
                    }
                }
            }
        }
    }
    let scene_element_count = elem_ids.len();

    // Structural reference systems: IfcGrid/IfcGridAxis for plan axes and a rooted IfcAnnotation
    // with curve geometry for every Z datum (never a fabricated building storey).
    if let Some(reference_systems) = scene.get("referenceSystems").and_then(Value::as_array) {
        for grid in reference_systems {
            let grid_id = grid.get("id").and_then(Value::as_str).unwrap_or("");
            let grid_kind = grid
                .get("kind")
                .and_then(Value::as_str)
                .unwrap_or("unknown");
            if grid_kind != "structural-grid" {
                let mut row = receipt_row(grid_id, grid_kind, "unsupported");
                row.insert(
                    "code".into(),
                    Value::String("unsupported-reference-system".into()),
                );
                row.insert(
                    "message".into(),
                    Value::String(format!(
                        "reference-system kind '{grid_kind}' is not supported"
                    )),
                );
                unsupported.push(Value::Object(row));
                for (collection, child_kind) in
                    [("axes", "grid-axis"), ("levels", "elevation-datum")]
                {
                    if let Some(children) = grid.get(collection).and_then(Value::as_array) {
                        for child in children {
                            let child_id = child.get("id").and_then(Value::as_str).unwrap_or("");
                            let mut child_row = receipt_row(child_id, child_kind, "unsupported");
                            child_row
                                .insert("code".into(), Value::String("unsupported-parent".into()));
                            child_row.insert("parentId".into(), Value::String(grid_id.into()));
                            child_row.insert(
                                "message".into(),
                                Value::String(format!(
                                    "parent reference-system kind '{grid_kind}' is not supported"
                                )),
                            );
                            unsupported.push(Value::Object(child_row));
                        }
                    }
                }
                continue;
            }
            let Some(origin) = vec3(grid.get("origin")) else {
                continue;
            };
            let Some(bounds) = grid.get("bounds").and_then(Value::as_object) else {
                continue;
            };
            let (min_x, max_x, min_y, max_y) = (
                num(bounds.get("minX")),
                num(bounds.get("maxX")),
                num(bounds.get("minY")),
                num(bounds.get("maxY")),
            );
            let grid_place = emit_axis_placement(
                &mut spf,
                origin,
                [0.0, 0.0, 1.0],
                [1.0, 0.0, 0.0],
                bldg_place,
            );
            let mut u_axes = Vec::new();
            let mut v_axes = Vec::new();
            let mut axis_rows = Vec::new();
            if let Some(axes) = grid.get("axes").and_then(Value::as_array) {
                for axis in axes {
                    let axis_id = axis.get("id").and_then(Value::as_str).unwrap_or("");
                    let direction = axis.get("direction").and_then(Value::as_str).unwrap_or("");
                    let offset = num(axis.get("offsetMm"));
                    let start = axis.get("startMm").and_then(Value::as_f64);
                    let end = axis.get("endMm").and_then(Value::as_f64);
                    let points = if direction == "x" {
                        vec![
                            [offset, start.unwrap_or(min_y)],
                            [offset, end.unwrap_or(max_y)],
                        ]
                    } else {
                        vec![
                            [start.unwrap_or(min_x), offset],
                            [end.unwrap_or(max_x), offset],
                        ]
                    };
                    let curve = emit_polyline2(&mut spf, &points, false);
                    let tag = axis.get("label").and_then(Value::as_str).unwrap_or(axis_id);
                    let axis_ent = spf.emit(&format!("IFCGRIDAXIS({},#{curve},.T.)", s_lit(tag)));
                    if direction == "x" {
                        u_axes.push(axis_ent);
                    } else {
                        v_axes.push(axis_ent);
                    }
                    let mut row = receipt_row(axis_id, "grid-axis", "emitted");
                    row.insert("realizedBy".into(), Value::String(grid_id.to_string()));
                    row.insert("entityType".into(), Value::String("IfcGridAxis".into()));
                    axis_rows.push(Value::Object(row));
                }
            }
            let entity_refs = |ids: &[i64]| {
                ids.iter()
                    .map(|id| format!("#{id}"))
                    .collect::<Vec<_>>()
                    .join(",")
            };
            let grid_gid = record_guid("reference-system", grid_id);
            let grid_name = grid.get("name").and_then(Value::as_str).unwrap_or(grid_id);
            let grid_ent = spf.emit(&format!(
                "IFCGRID({},$,{},$,$,#{grid_place},$,({}),({}),$,$)",
                s_lit(&grid_gid),
                s_lit(grid_name),
                entity_refs(&u_axes),
                entity_refs(&v_axes)
            ));
            elem_ids.push(grid_ent);
            let mut grid_row = receipt_row(grid_id, grid_kind, "emitted");
            grid_row.insert("ifcGuid".into(), Value::String(grid_gid));
            grid_row.insert("entityType".into(), Value::String("IfcGrid".into()));
            emitted.push(Value::Object(grid_row));
            emitted.extend(axis_rows);

            if let Some(levels) = grid.get("levels").and_then(Value::as_array) {
                for level in levels {
                    let level_id = level.get("id").and_then(Value::as_str).unwrap_or("");
                    let elevation = num(level.get("elevationMm"));
                    let level_name = level
                        .get("label")
                        .and_then(Value::as_str)
                        .unwrap_or(level_id);
                    let level_place = emit_axis_placement(
                        &mut spf,
                        [origin[0], origin[1], elevation],
                        [0.0, 0.0, 1.0],
                        [1.0, 0.0, 0.0],
                        bldg_place,
                    );
                    let p1 = spf.emit(&format!(
                        "IFCCARTESIANPOINT(({},{},0.))",
                        r(min_x),
                        r((min_y + max_y) / 2.0)
                    ));
                    let p2 = spf.emit(&format!(
                        "IFCCARTESIANPOINT(({},{},0.))",
                        r(max_x),
                        r((min_y + max_y) / 2.0)
                    ));
                    let p3 = spf.emit(&format!(
                        "IFCCARTESIANPOINT(({},{},0.))",
                        r((min_x + max_x) / 2.0),
                        r(min_y)
                    ));
                    let p4 = spf.emit(&format!(
                        "IFCCARTESIANPOINT(({},{},0.))",
                        r((min_x + max_x) / 2.0),
                        r(max_y)
                    ));
                    let line_x = spf.emit(&format!("IFCPOLYLINE((#{p1},#{p2}))"));
                    let line_y = spf.emit(&format!("IFCPOLYLINE((#{p3},#{p4}))"));
                    let curves = spf.emit(&format!("IFCGEOMETRICCURVESET((#{line_x},#{line_y}))"));
                    let rep = spf.emit(&format!(
                        "IFCSHAPEREPRESENTATION(#{ctx},'Annotation','GeometricCurveSet',(#{curves}))"
                    ));
                    let pds = spf.emit(&format!("IFCPRODUCTDEFINITIONSHAPE($,$,(#{rep}))"));
                    let level_gid = record_guid("level", level_id);
                    let annotation = spf.emit(&format!(
                        "IFCANNOTATION({},$,{},$,'Elevation datum',#{level_place},#{pds})",
                        s_lit(&level_gid),
                        s_lit(level_name)
                    ));
                    elem_ids.push(annotation);
                    let mut row = receipt_row(level_id, "elevation-datum", "emitted");
                    row.insert("ifcGuid".into(), Value::String(level_gid));
                    row.insert("realizedBy".into(), Value::String(grid_id.to_string()));
                    row.insert("entityType".into(), Value::String("IfcAnnotation".into()));
                    emitted.push(Value::Object(row));
                }
            }
        }
    }

    if !elem_ids.is_empty() {
        let refs = elem_ids
            .iter()
            .map(|i| format!("#{i}"))
            .collect::<Vec<_>>()
            .join(",");
        let g = guid(spf.id);
        spf.emit(&format!(
            "IFCRELCONTAINEDINSPATIALSTRUCTURE({},$,$,$,({refs}),#{storey})",
            s_lit(&g)
        ));
    }

    // ── Phase 5: material associations (from emitted products only; never an empty RelatedObjects) ──
    for (mk, members) in &mat_members {
        if members.is_empty() {
            continue;
        }
        let Some(mid) = mat_id.get(mk) else { continue };
        let refs = members
            .iter()
            .map(|i| format!("#{i}"))
            .collect::<Vec<_>>()
            .join(",");
        let g = guid(spf.id);
        spf.emit(&format!(
            "IFCRELASSOCIATESMATERIAL({},$,$,$,({refs}),#{mid})",
            s_lit(&g)
        ));
    }

    // ── assemble the SPF document ──
    let fname = file_name_meta(proj_name);
    let mut doc = String::new();
    doc.push_str("ISO-10303-21;\n");
    doc.push_str("HEADER;\n");
    // DesignTransferView: the profile defs are parametric geometry (outside Reference View).
    doc.push_str("FILE_DESCRIPTION(('ViewDefinition [DesignTransferView_V1.0]'),'2;1');\n");
    let _ = writeln!(
        doc,
        "FILE_NAME('{fname}','1970-01-01T00:00:00',(''),(''),'AWARE ifc','AWARE','');"
    );
    doc.push_str("FILE_SCHEMA(('IFC4'));\n");
    doc.push_str("ENDSEC;\n");
    doc.push_str("DATA;\n");
    doc.push_str(&spf.buf);
    doc.push_str("ENDSEC;\n");
    doc.push_str("END-ISO-10303-21;\n");

    Ok(BuildResult {
        doc,
        members: scene_element_count,
        columns,
        beams,
        meshes,
        profiles: profile_counts,
        warnings,
        emitted,
        failed,
        unsupported,
    })
}

/// `ifc.write` — write a generic 3D scene to an IFC4 file. Mirrors `viewer-3d.render`'s contract:
/// `{ path?, bytes, members, columns, beams, meshes, profiles, warnings }`, with the `output-path`
/// write gated to a real run.
fn require_id<'a>(
    record: &'a Value,
    path: &str,
    seen: &mut BTreeSet<String>,
) -> Result<&'a str, AwareError> {
    let raw_id = record
        .get("id")
        .and_then(Value::as_str)
        .ok_or_else(|| AwareError::Validation(format!("ifc write: `{path}.id` is required")))?;
    if raw_id != raw_id.trim() {
        return Err(AwareError::Validation(format!(
            "ifc write: `{path}.id` must not have leading or trailing whitespace"
        )));
    }
    let id = raw_id;
    if id.is_empty() || id.len() > 256 || id.chars().any(|ch| (ch as u32) <= 0x1f || ch == '\u{7f}')
    {
        return Err(AwareError::Validation(format!(
            "ifc write: `{path}.id` must be 1..256 UTF-8 bytes with no C0/DEL controls"
        )));
    }
    if !seen.insert(id.to_string()) {
        return Err(AwareError::Validation(format!(
            "ifc write: duplicate scene record id `{id}`"
        )));
    }
    Ok(id)
}

fn require_positive(record: &Value, field: &str, path: &str) -> Result<f64, AwareError> {
    pos(record.get(field)).ok_or_else(|| {
        AwareError::Validation(format!(
            "ifc write: `{path}.{field}` must be a finite positive number"
        ))
    })
}

fn optional_nonnegative(record: &Value, field: &str, path: &str) -> Result<f64, AwareError> {
    match record.get(field) {
        None | Some(Value::Null) => Ok(0.0),
        Some(value) => value
            .as_f64()
            .filter(|number| number.is_finite() && *number >= 0.0)
            .ok_or_else(|| {
                AwareError::Validation(format!(
                    "ifc write: `{path}.{field}` must be a finite nonnegative number"
                ))
            }),
    }
}

fn is_physical_kind(kind: &str) -> bool {
    matches!(
        kind,
        "member"
            | "line"
            | "box"
            | "plate"
            | "rod"
            | "bolt-shank"
            | "washer"
            | "nut"
            | "bolt-head"
            | "mesh"
    )
}

/// One shared tolerance for every box-frame legality test (matches FloLess `BOX_TOOL_TOL`), so the
/// producer and all sinks accept/reject the SAME frames. (Codex plan-review R3 #5.)
const BOX_TOOL_TOL: f64 = 1e-6;

/// Validate a `box` Boolean tool's frame: finite origin, three positive half-extents, a unit orthonormal
/// right-handed (uDir, vDir, normal) frame with `normal == uDir×vDir`, and an in-range re-entrant fillet.
fn validate_box_tool(tool: &serde_json::Map<String, Value>, path: &str) -> Result<(), AwareError> {
    let frame = tool
        .get("frame")
        .and_then(Value::as_object)
        .ok_or_else(|| AwareError::Validation(format!("ifc write: `{path}.frame` is required")))?;
    let (Some(_origin), Some(u), Some(v), Some(n)) = (
        vec3(frame.get("origin")),
        vec3(frame.get("uDir")),
        vec3(frame.get("vDir")),
        vec3(frame.get("normal")),
    ) else {
        return Err(AwareError::Validation(format!(
            "ifc write: `{path}.frame` needs finite origin/uDir/vDir/normal Vec3"
        )));
    };
    let he = tool
        .get("halfExtents")
        .and_then(Value::as_array)
        .ok_or_else(|| {
            AwareError::Validation(format!("ifc write: `{path}.halfExtents` is required"))
        })?;
    if he.len() != 3
        || !he
            .iter()
            .all(|h| h.as_f64().is_some_and(|x| x.is_finite() && x > 0.0))
    {
        return Err(AwareError::Validation(format!(
            "ifc write: `{path}.halfExtents` must be three positive numbers"
        )));
    }
    let len = |a: Vec3| (a[0] * a[0] + a[1] * a[1] + a[2] * a[2]).sqrt();
    let dot = |a: Vec3, b: Vec3| a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
    for (name, vv) in [("uDir", u), ("vDir", v), ("normal", n)] {
        if (len(vv) - 1.0).abs() > BOX_TOOL_TOL {
            return Err(AwareError::Validation(format!(
                "ifc write: `{path}.frame.{name}` must be unit length"
            )));
        }
    }
    if dot(u, v).abs() > BOX_TOOL_TOL
        || dot(u, n).abs() > BOX_TOOL_TOL
        || dot(v, n).abs() > BOX_TOOL_TOL
    {
        return Err(AwareError::Validation(format!(
            "ifc write: `{path}.frame` axes must be mutually orthogonal"
        )));
    }
    let rh = cross3(u, v);
    if (rh[0] - n[0]).abs() > BOX_TOOL_TOL
        || (rh[1] - n[1]).abs() > BOX_TOOL_TOL
        || (rh[2] - n[2]).abs() > BOX_TOOL_TOL
    {
        return Err(AwareError::Validation(format!(
            "ifc write: `{path}.frame` must be right-handed (normal == uDir×vDir)"
        )));
    }
    if let Some(re) = tool.get("reentrant").and_then(Value::as_object) {
        let hu = he[0].as_f64().unwrap_or(0.0);
        let hv = he[1].as_f64().unwrap_or(0.0);
        if !matches!(re.get("radiusMm").and_then(Value::as_f64), Some(r) if r.is_finite() && r > 0.0 && r < (2.0 * hu).min(2.0 * hv))
        {
            return Err(AwareError::Validation(format!(
                "ifc write: `{path}.reentrant.radiusMm` must be finite and 0 < r < min(2hu,2hv)"
            )));
        }
        if !matches!(re.get("vSign").and_then(Value::as_f64), Some(s) if s == 1.0 || s == -1.0) {
            return Err(AwareError::Validation(format!(
                "ifc write: `{path}.reentrant.vSign` must be -1 or 1"
            )));
        }
    }
    Ok(())
}

fn validate_scene(scene: &Value) -> Result<(), AwareError> {
    scene_up(scene, false, "ifc write")?;
    if let Some(units) = scene
        .get("meta")
        .and_then(Value::as_object)
        .and_then(|meta| meta.get("units"))
    {
        let units = units.as_str().ok_or_else(|| {
            AwareError::Validation(
                "ifc write: scene `meta.units` must be the string `mm` when present".into(),
            )
        })?;
        if units != "mm" {
            return Err(AwareError::Validation(format!(
                "ifc write: unsupported scene units `{units}`; only `mm` is accepted"
            )));
        }
    }

    let mut seen = BTreeSet::new();
    let mut element_ids = BTreeSet::new();
    let mut element_kinds = BTreeMap::new();
    let mut element_by_id = BTreeMap::new();
    if let Some(value) = scene.get("elements")
        && !value.is_array()
    {
        return Err(AwareError::Validation(
            "ifc write: `scene.elements` must be an array".into(),
        ));
    }
    if let Some(elements) = scene.get("elements").and_then(Value::as_array) {
        for (index, el) in elements.iter().enumerate() {
            let path = format!("scene.elements[{index}]");
            let id = require_id(el, &path, &mut seen)?;
            element_ids.insert(id.to_string());
            element_by_id.insert(id.to_string(), el);
            let kind = el.get("kind").and_then(Value::as_str).unwrap_or_else(|| {
                if el.get("positions").is_some() {
                    "mesh"
                } else {
                    "member"
                }
            });
            if matches!(
                kind,
                "plate" | "rod" | "bolt-shank" | "washer" | "nut" | "bolt-head" | "mesh"
            ) && el.get("rot").is_some()
            {
                return Err(AwareError::Validation(format!(
                    "ifc write: `{path}.rot` is applicable only to physical member, line, and box records"
                )));
            }
            element_kinds.insert(id.to_string(), kind.to_string());
            match kind {
                "plate" => {
                    let frame = el.get("frame").and_then(Value::as_object).ok_or_else(|| {
                        AwareError::Validation(format!("ifc write: `{path}.frame` is required"))
                    })?;
                    if vec3(frame.get("origin")).is_none() {
                        return Err(AwareError::Validation(format!(
                            "ifc write: `{path}.frame.origin` must be Vec3"
                        )));
                    }
                    let u = vec3(frame.get("uDir"))
                        .and_then(normalized3)
                        .ok_or_else(|| {
                            AwareError::Validation(format!(
                                "ifc write: `{path}.frame.uDir` must be a nonzero Vec3"
                            ))
                        })?;
                    let v = vec3(frame.get("vDir"))
                        .and_then(normalized3)
                        .ok_or_else(|| {
                            AwareError::Validation(format!(
                                "ifc write: `{path}.frame.vDir` must be a nonzero Vec3"
                            ))
                        })?;
                    let normal =
                        vec3(frame.get("normal"))
                            .and_then(normalized3)
                            .ok_or_else(|| {
                                AwareError::Validation(format!(
                                    "ifc write: `{path}.frame.normal` must be a nonzero Vec3"
                                ))
                            })?;
                    let uv = normalized3(cross3(u, v)).ok_or_else(|| {
                        AwareError::Validation(format!(
                            "ifc write: `{path}.frame` axes are collinear"
                        ))
                    })?;
                    let agreement = dot3(uv, normal);
                    if dot3(u, v).abs() > 1.0e-6 || agreement < 1.0 - 1.0e-6 {
                        return Err(AwareError::Validation(format!(
                            "ifc write: `{path}.frame` must be right-handed and orthonormal"
                        )));
                    }
                    require_positive(el, "thicknessMm", &path)?;
                    let outline = el.get("outline").and_then(Value::as_array).ok_or_else(|| {
                        AwareError::Validation(format!("ifc write: `{path}.outline` is required"))
                    })?;
                    let polygon = outline
                        .iter()
                        .map(|point| vec2(Some(point)))
                        .collect::<Option<Vec<_>>>();
                    let Some(polygon) =
                        polygon.filter(|polygon| polygon_is_simple_nonzero(polygon))
                    else {
                        return Err(AwareError::Validation(format!(
                            "ifc write: `{path}.outline` must be a finite, nonzero, simple polygon"
                        )));
                    };
                    if el
                        .get("holes")
                        .is_some_and(|holes| !holes.is_null() && !holes.is_array())
                    {
                        return Err(AwareError::Validation(format!(
                            "ifc write: `{path}.holes` must be an array"
                        )));
                    }
                    if let Some(holes) = el.get("holes").and_then(Value::as_array) {
                        let mut circles: Vec<(Vec2, f64)> = Vec::with_capacity(holes.len());
                        for (hole_index, hole) in holes.iter().enumerate() {
                            let hole_path = format!("{path}.holes[{hole_index}]");
                            require_id(hole, &hole_path, &mut seen)?;
                            let Some(center) = vec2(hole.get("center")) else {
                                return Err(AwareError::Validation(format!(
                                    "ifc write: `{hole_path}.center` must be Vec2"
                                )));
                            };
                            let diameter = require_positive(hole, "diameterMm", &hole_path)?;
                            if !point_in_polygon(center, &polygon)
                                || polygon_edges(&polygon).any(|(a, b)| {
                                    point_segment_distance(center, a, b) + 1.0e-9 < diameter / 2.0
                                })
                            {
                                return Err(AwareError::Validation(format!(
                                    "ifc write: `{hole_path}` must lie wholly inside the plate outline"
                                )));
                            }
                            if circles.iter().any(|(other, other_diameter)| {
                                let distance = ((center[0] - other[0]).powi(2)
                                    + (center[1] - other[1]).powi(2))
                                .sqrt();
                                distance <= (diameter + *other_diameter) / 2.0 + 1.0e-9
                            }) {
                                return Err(AwareError::Validation(format!(
                                    "ifc write: `{hole_path}` must not overlap or touch another hole"
                                )));
                            }
                            circles.push((center, diameter));
                        }
                    }
                }
                "member" | "line" | "box" => {
                    let from = vec3(el.get("from"));
                    let to = vec3(el.get("to"));
                    if !matches!((from, to), (Some(a), Some(b)) if normalized3([b[0]-a[0],b[1]-a[1],b[2]-a[2]]).is_some())
                    {
                        return Err(AwareError::Validation(format!(
                            "ifc write: `{path}` requires distinct finite from/to Vec3 points"
                        )));
                    }
                    member_roll(el.get("rot"), &format!("{path}.rot"), "ifc write")?;
                }
                "rod" | "bolt-shank" => {
                    let axis = el.get("axis").and_then(Value::as_object).ok_or_else(|| {
                        AwareError::Validation(format!("ifc write: `{path}.axis` is required"))
                    })?;
                    let from = vec3(axis.get("from"));
                    let to = vec3(axis.get("to"));
                    let valid_axis = match (from, to) {
                        (Some(from), Some(to)) => {
                            normalized3([to[0] - from[0], to[1] - from[1], to[2] - from[2]])
                                .is_some()
                        }
                        _ => false,
                    };
                    if !valid_axis {
                        return Err(AwareError::Validation(format!(
                            "ifc write: `{path}.axis` must contain distinct finite from/to Vec3 points"
                        )));
                    }
                    require_positive(el, "diameterMm", &path)?;
                }
                "washer" => {
                    if vec3(el.get("center")).is_none()
                        || vec3(el.get("axis")).and_then(normalized3).is_none()
                    {
                        return Err(AwareError::Validation(format!(
                            "ifc write: `{path}` requires finite center and nonzero axis Vec3"
                        )));
                    }
                    let outer = require_positive(el, "outerDiameterMm", &path)?;
                    let inner = require_positive(el, "innerDiameterMm", &path)?;
                    require_positive(el, "thicknessMm", &path)?;
                    if inner >= outer {
                        return Err(AwareError::Validation(format!(
                            "ifc write: `{path}.innerDiameterMm` must be smaller than outerDiameterMm"
                        )));
                    }
                }
                "nut" | "bolt-head" => {
                    if vec3(el.get("center")).is_none()
                        || vec3(el.get("axis")).and_then(normalized3).is_none()
                    {
                        return Err(AwareError::Validation(format!(
                            "ifc write: `{path}` requires finite center and nonzero axis Vec3"
                        )));
                    }
                    require_positive(el, "acrossFlatsMm", &path)?;
                    require_positive(el, "thicknessMm", &path)?;
                    if el
                        .get("phaseRad")
                        .is_some_and(|v| v.as_f64().is_none_or(|phase| !phase.is_finite()))
                    {
                        return Err(AwareError::Validation(format!(
                            "ifc write: `{path}.phaseRad` must be finite"
                        )));
                    }
                }
                "mesh" => {
                    let positions =
                        el.get("positions")
                            .and_then(Value::as_array)
                            .ok_or_else(|| {
                                AwareError::Validation(format!(
                                    "ifc write: `{path}.positions` must be an array"
                                ))
                            })?;
                    if positions.len() < 9
                        || positions.len() % 3 != 0
                        || positions
                            .iter()
                            .any(|value| value.as_f64().is_none_or(|number| !number.is_finite()))
                    {
                        return Err(AwareError::Validation(format!(
                            "ifc write: `{path}.positions` must contain finite xyz triples for at least one triangle"
                        )));
                    }
                    let indices = el.get("indices").and_then(Value::as_array).ok_or_else(|| {
                        AwareError::Validation(format!(
                            "ifc write: `{path}.indices` must be an array"
                        ))
                    })?;
                    let vertex_count = positions.len() / 3;
                    if indices.is_empty()
                        || indices.len() % 3 != 0
                        || indices.iter().any(|value| {
                            value
                                .as_u64()
                                .is_none_or(|index| index as usize >= vertex_count)
                        })
                    {
                        return Err(AwareError::Validation(format!(
                            "ifc write: `{path}.indices` must contain valid triangle index triples"
                        )));
                    }
                }
                _ => {}
            }
        }
    }

    if let Some(value) = scene.get("operations")
        && !value.is_array()
    {
        return Err(AwareError::Validation(
            "ifc write: `scene.operations` must be an array".into(),
        ));
    }
    let realized_bolt_components = scene
        .get("operations")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter(|op| op.get("kind").and_then(Value::as_str) == Some("bolt-array"))
        .filter_map(|op| op.get("instances").and_then(Value::as_array))
        .flatten()
        .flat_map(|instance| {
            let mut ids = Vec::new();
            for field in ["shankId", "headId"] {
                if let Some(id) = instance.get(field).and_then(Value::as_str) {
                    ids.push(id);
                }
            }
            for field in ["nutIds", "washerIds"] {
                if let Some(children) = instance.get(field).and_then(Value::as_array) {
                    ids.extend(children.iter().filter_map(Value::as_str));
                }
            }
            ids
        })
        .collect::<BTreeSet<_>>();
    let mut claimed_bolt_components = BTreeSet::new();
    if let Some(operations) = scene.get("operations").and_then(Value::as_array) {
        for (op_index, op) in operations.iter().enumerate() {
            let path = format!("scene.operations[{op_index}]");
            require_id(op, &path, &mut seen)?;
            let op_kind = op.get("kind").and_then(Value::as_str).unwrap_or("");
            if op_kind == "bolt-array" {
                let mut participants = Vec::with_capacity(2);
                for field in ["partToBoltTo", "partToBeBolted"] {
                    let target = op.get(field).and_then(Value::as_str).unwrap_or("");
                    if !element_kinds
                        .get(target)
                        .is_some_and(|kind| is_physical_kind(kind))
                    {
                        return Err(AwareError::Validation(format!(
                            "ifc write: `{path}.{field}` must reference a physical element (got `{target}`)"
                        )));
                    }
                    if realized_bolt_components.contains(target) {
                        return Err(AwareError::Validation(format!(
                            "ifc write: `{path}.{field}` must reference an independently materialized physical element, not bolt-array component `{target}`"
                        )));
                    }
                    participants.push(target);
                }
                if participants[0] == participants[1] {
                    return Err(AwareError::Validation(format!(
                        "ifc write: `{path}` bolt participants must be distinct elements"
                    )));
                }
                let frame = op.get("frame").and_then(Value::as_object).ok_or_else(|| {
                    AwareError::Validation(format!("ifc write: `{path}.frame` is required"))
                })?;
                // Parse each frame field once, at the point of use, so the
                // validation and the extraction cannot drift apart.
                let frame_vec3 = |field: &str| {
                    vec3(frame.get(field)).ok_or_else(|| {
                        AwareError::Validation(format!(
                            "ifc write: `{path}.frame.{field}` must be a valid Vec3"
                        ))
                    })
                };
                let frame_direction = |field: &str| {
                    normalized3(frame_vec3(field)?).ok_or_else(|| {
                        AwareError::Validation(format!(
                            "ifc write: `{path}.frame.{field}` must be a valid Vec3"
                        ))
                    })
                };
                let origin = frame_vec3("origin")?;
                let u = frame_direction("uDir")?;
                let v = frame_direction("vDir")?;
                let normal = frame_direction("normal")?;
                // A zero-length cross product means `uDir` and `vDir` are
                // parallel — the same "not orthonormal" failure reported here.
                let right_handed = normalized3(cross3(u, v))
                    .is_some_and(|cross| dot3(cross, normal) >= 1.0 - 1.0e-6);
                if dot3(u, v).abs() > 1.0e-6 || !right_handed {
                    return Err(AwareError::Validation(format!(
                        "ifc write: `{path}.frame` must be right-handed and orthonormal"
                    )));
                }
                let mut offsets = Vec::with_capacity(2);
                for field in ["uOffsetsMm", "vOffsetsMm"] {
                    let values = op.get(field).and_then(Value::as_array).ok_or_else(|| {
                        AwareError::Validation(format!(
                            "ifc write: `{path}.{field}` must be an array"
                        ))
                    })?;
                    // One pass: reject a non-finite entry and keep the parsed
                    // value, instead of checking and then re-reading it.
                    let parsed = values
                        .iter()
                        .map(|value| value.as_f64().filter(|number| number.is_finite()))
                        .collect::<Option<Vec<_>>>()
                        .filter(|parsed: &Vec<f64>| !parsed.is_empty())
                        .ok_or_else(|| {
                            AwareError::Validation(format!(
                                "ifc write: `{path}.{field}` must contain finite offsets"
                            ))
                        })?;
                    if parsed
                        .iter()
                        .enumerate()
                        .any(|(index, value)| parsed[..index].contains(value))
                    {
                        return Err(AwareError::Validation(format!(
                            "ifc write: `{path}.{field}` must contain unique offsets"
                        )));
                    }
                    offsets.push(parsed);
                }
                let diameter = require_positive(op, "diameterMm", &path)?;
                if op
                    .get("standard")
                    .and_then(Value::as_str)
                    .is_none_or(|standard| standard.trim().is_empty())
                {
                    return Err(AwareError::Validation(format!(
                        "ifc write: `{path}.standard` must be a non-empty string"
                    )));
                }
                let tolerance = optional_nonnegative(op, "toleranceMm", &path)?;
                if !matches!(
                    op.get("boltType").and_then(Value::as_str),
                    Some("shop" | "site")
                ) {
                    return Err(AwareError::Validation(format!(
                        "ifc write: `{path}.boltType` must be `shop` or `site`"
                    )));
                }
                let components =
                    op.get("components")
                        .and_then(Value::as_object)
                        .ok_or_else(|| {
                            AwareError::Validation(format!(
                                "ifc write: `{path}.components` must be an object"
                            ))
                        })?;
                for field in ["bolt", "nut", "washer"] {
                    if components
                        .get(field)
                        .is_some_and(|value| !value.is_boolean())
                    {
                        return Err(AwareError::Validation(format!(
                            "ifc write: `{path}.components.{field}` must be boolean"
                        )));
                    }
                }
                if components.get("bolt").and_then(Value::as_bool) == Some(false) {
                    return Err(AwareError::Validation(format!(
                        "ifc write: `{path}.components.bolt` must be true when instances author shankId and headId"
                    )));
                }
                let instances = op
                    .get("instances")
                    .and_then(Value::as_array)
                    .ok_or_else(|| {
                        AwareError::Validation(format!("ifc write: `{path}.instances` is required"))
                    })?;
                let mut bolt_materials = BTreeSet::new();
                if instances.len() != offsets[0].len() * offsets[1].len() {
                    return Err(AwareError::Validation(format!(
                        "ifc write: `{path}.instances` must match the Cartesian offset count"
                    )));
                }
                let mut expected_points = offsets[0]
                    .iter()
                    .flat_map(|u_offset| {
                        offsets[1].iter().map(move |v_offset| {
                            add3(add3(origin, scale3(u, *u_offset)), scale3(v, *v_offset))
                        })
                    })
                    .collect::<Vec<_>>();
                for (instance_index, instance) in instances.iter().enumerate() {
                    let instance_path = format!("{path}.instances[{instance_index}]");
                    require_id(instance, &instance_path, &mut seen)?;
                    let Some(instance_point) = vec3(instance.get("point")) else {
                        return Err(AwareError::Validation(format!(
                            "ifc write: `{instance_path}.point` must be Vec3"
                        )));
                    };
                    let Some(point_index) = expected_points
                        .iter()
                        .position(|expected| distance3(*expected, instance_point) <= 0.1)
                    else {
                        return Err(AwareError::Validation(format!(
                            "ifc write: `{instance_path}.point` must uniquely match an authored Cartesian offset position"
                        )));
                    };
                    expected_points.remove(point_index);
                    for (field, expected) in [
                        ("shankId", &["rod", "bolt-shank"][..]),
                        ("headId", &["bolt-head"][..]),
                    ] {
                        let child = instance.get(field).and_then(Value::as_str).unwrap_or("");
                        if !element_kinds
                            .get(child)
                            .is_some_and(|kind| expected.contains(&kind.as_str()))
                        {
                            return Err(AwareError::Validation(format!(
                                "ifc write: `{instance_path}.{field}` references wrong or unknown physical kind `{child}`"
                            )));
                        }
                        if !claimed_bolt_components.insert(child.to_string()) {
                            return Err(AwareError::Validation(format!(
                                "ifc write: `{instance_path}.{field}` reuses bolt component `{child}`"
                            )));
                        }
                        let child_element = element_by_id[child];
                        if let Some((material, _)) = resolve_material(child_element) {
                            bolt_materials.insert(material);
                        }
                        if field == "shankId" {
                            let axis = child_element
                                .get("axis")
                                .and_then(Value::as_object)
                                .ok_or_else(|| {
                                    AwareError::Validation(format!(
                                        "ifc write: `{instance_path}.{field}` requires an axis object"
                                    ))
                                })?;
                            let axis_endpoint = |end: &str| {
                                vec3(axis.get(end)).ok_or_else(|| {
                                    AwareError::Validation(format!(
                                        "ifc write: `{instance_path}.{field}` axis `{end}` must be a valid Vec3"
                                    ))
                                })
                            };
                            let from = axis_endpoint("from")?;
                            let to = axis_endpoint("to")?;
                            let direction =
                                normalized3([to[0] - from[0], to[1] - from[1], to[2] - from[2]])
                                    .ok_or_else(|| {
                                        AwareError::Validation(format!(
                                            "ifc write: `{instance_path}.{field}` shank axis must have nonzero length"
                                        ))
                                    })?;
                            let offset = [
                                instance_point[0] - from[0],
                                instance_point[1] - from[1],
                                instance_point[2] - from[2],
                            ];
                            let child_diameter = child_element
                                .get("diameterMm")
                                .and_then(Value::as_f64)
                                .ok_or_else(|| {
                                    AwareError::Validation(format!(
                                        "ifc write: `{instance_path}.{field}` requires a numeric `diameterMm`"
                                    ))
                                })?;
                            if (dot3(direction, normal).abs() - 1.0).abs() > 1.0e-6
                                || length3(cross3(offset, direction)) > 0.1
                                || (child_diameter - diameter).abs() > 0.1
                            {
                                return Err(AwareError::Validation(format!(
                                    "ifc write: `{instance_path}.{field}` shank axis and diameter must match the bolt instance"
                                )));
                            }
                        } else {
                            let center = vec3(child_element.get("center")).ok_or_else(|| {
                                AwareError::Validation(format!(
                                    "ifc write: `{instance_path}.{field}` requires a valid `center`"
                                ))
                            })?;
                            let child_axis = vec3(child_element.get("axis"))
                                .and_then(normalized3)
                                .ok_or_else(|| {
                                AwareError::Validation(format!(
                                    "ifc write: `{instance_path}.{field}` requires a nonzero `axis`"
                                ))
                            })?;
                            let offset = [
                                center[0] - instance_point[0],
                                center[1] - instance_point[1],
                                center[2] - instance_point[2],
                            ];
                            if (dot3(child_axis, normal).abs() - 1.0).abs() > 1.0e-6
                                || length3(cross3(offset, normal)) > 0.1
                            {
                                return Err(AwareError::Validation(format!(
                                    "ifc write: `{instance_path}.{field}` must be centered and aligned on the bolt instance axis"
                                )));
                            }
                        }
                    }
                    for (field, expected) in [("nutIds", "nut"), ("washerIds", "washer")] {
                        let children =
                            instance
                                .get(field)
                                .and_then(Value::as_array)
                                .ok_or_else(|| {
                                    AwareError::Validation(format!(
                                        "ifc write: `{instance_path}.{field}` must be an array"
                                    ))
                                })?;
                        for child in children {
                            let child = child.as_str().unwrap_or("");
                            if element_kinds.get(child).map(String::as_str) != Some(expected) {
                                return Err(AwareError::Validation(format!(
                                    "ifc write: `{instance_path}.{field}` references wrong or unknown physical kind `{child}`"
                                )));
                            }
                            if !claimed_bolt_components.insert(child.to_string()) {
                                return Err(AwareError::Validation(format!(
                                    "ifc write: `{instance_path}.{field}` reuses bolt component `{child}`"
                                )));
                            }
                            let child_element = element_by_id[child];
                            if let Some((material, _)) = resolve_material(child_element) {
                                bolt_materials.insert(material);
                            }
                            let center = vec3(child_element.get("center")).ok_or_else(|| {
                                AwareError::Validation(format!(
                                    "ifc write: `{instance_path}.{field}` requires a valid `center`"
                                ))
                            })?;
                            let child_axis = vec3(child_element.get("axis"))
                                .and_then(normalized3)
                                .ok_or_else(|| {
                                AwareError::Validation(format!(
                                    "ifc write: `{instance_path}.{field}` requires a nonzero `axis`"
                                ))
                            })?;
                            let offset = [
                                center[0] - instance_point[0],
                                center[1] - instance_point[1],
                                center[2] - instance_point[2],
                            ];
                            if (dot3(child_axis, normal).abs() - 1.0).abs() > 1.0e-6
                                || length3(cross3(offset, normal)) > 0.1
                            {
                                return Err(AwareError::Validation(format!(
                                    "ifc write: `{instance_path}.{field}` child `{child}` must be centered and aligned on the bolt instance axis"
                                )));
                            }
                        }
                    }
                    let effects = instance
                        .get("holeEffects")
                        .and_then(Value::as_array)
                        .ok_or_else(|| {
                            AwareError::Validation(format!(
                                "ifc write: `{instance_path}.holeEffects` must be an array"
                            ))
                        })?;
                    // ONE EFFECT PER PLY THE BOLT PASSES THROUGH — the declared pair at minimum, and more
                    // when the joint is genuinely multi-ply. A double-sided gusset bolts plate + angle leg
                    // + plate: three plies, three holes, one bolt in double shear, and that is an ordinary
                    // detail rather than an exotic one. Pinning the count at two made every such joint
                    // unexportable and refused the whole scene over it, so the effect list IS the ply stack
                    // and `partToBoltTo`/`partToBeBolted` name the primary pair within it (the pair the
                    // native hosts bind first). The emit pass already drills every effect it is given.
                    if effects.len() < participants.len() {
                        return Err(AwareError::Validation(format!(
                            "ifc write: `{instance_path}.holeEffects` must contain at least one effect per bolt participant"
                        )));
                    }
                    let mut effect_targets = BTreeSet::new();
                    for (effect_index, effect) in effects.iter().enumerate() {
                        let effect_path = format!("{instance_path}.holeEffects[{effect_index}]");
                        require_id(effect, &effect_path, &mut seen)?;
                        let target = effect.get("targetId").and_then(Value::as_str).unwrap_or("");
                        // A ply beyond the declared pair is held to exactly the bar the pair is held to.
                        // It has to be: the emit pass SKIPS a target it cannot resolve, so an unknown or
                        // mistyped id would validate here and then quietly not be drilled — a hole missing
                        // from a fabricated ply, with the receipt still reading clean.
                        if !element_kinds
                            .get(target)
                            .is_some_and(|kind| is_physical_kind(kind))
                        {
                            return Err(AwareError::Validation(format!(
                                "ifc write: `{effect_path}.targetId` must reference a physical element (got `{target}`)"
                            )));
                        }
                        if realized_bolt_components.contains(target) {
                            return Err(AwareError::Validation(format!(
                                "ifc write: `{effect_path}.targetId` must reference an independently materialized physical element, not bolt-array component `{target}`"
                            )));
                        }
                        if !effect_targets.insert(target) {
                            return Err(AwareError::Validation(format!(
                                "ifc write: `{effect_path}.targetId` must not hole the same ply twice"
                            )));
                        }
                        let Some(center) = vec3(effect.get("center")) else {
                            return Err(AwareError::Validation(format!(
                                "ifc write: `{effect_path}.center` must be a finite Vec3"
                            )));
                        };
                        if distance3(center, instance_point) > 0.1 {
                            return Err(AwareError::Validation(format!(
                                "ifc write: `{effect_path}.center` must match the bolt instance point"
                            )));
                        }
                        let Some(effect_axis) = vec3(effect.get("axis")).and_then(normalized3)
                        else {
                            return Err(AwareError::Validation(format!(
                                "ifc write: `{effect_path}.axis` must be a nonzero Vec3"
                            )));
                        };
                        if (dot3(effect_axis, normal).abs() - 1.0).abs() > 1.0e-6 {
                            return Err(AwareError::Validation(format!(
                                "ifc write: `{effect_path}.axis` must align with the bolt frame normal"
                            )));
                        }
                        let effect_diameter = require_positive(effect, "diameterMm", &effect_path)?;
                        if (effect_diameter - (diameter + tolerance)).abs() > 0.1 {
                            return Err(AwareError::Validation(format!(
                                "ifc write: `{effect_path}.diameterMm` must equal bolt diameter plus tolerance"
                            )));
                        }
                    }
                    // Extra plies are allowed; the declared pair is still mandatory. Counting effects
                    // against participants would let a 3-ply stack that MISSES one of the pair pass on
                    // arithmetic alone, so name the one that is absent instead.
                    if let Some(missing) = participants
                        .iter()
                        .find(|participant| !effect_targets.contains(*participant))
                    {
                        return Err(AwareError::Validation(format!(
                            "ifc write: `{instance_path}.holeEffects` must cover bolt participant `{missing}`"
                        )));
                    }
                }
                if !expected_points.is_empty() {
                    return Err(AwareError::Validation(format!(
                        "ifc write: `{path}.instances` must exhaust the authored Cartesian offset positions"
                    )));
                }
                if bolt_materials.len() > 1 {
                    return Err(AwareError::Validation(format!(
                        "ifc write: `{path}.instances` bolt components must use one normalized material because the grouped IfcMechanicalFastener has one material association"
                    )));
                }
            } else if op_kind == "weld" {
                for field in ["mainId", "secondaryId"] {
                    let target = op.get(field).and_then(Value::as_str).unwrap_or("");
                    if !element_kinds
                        .get(target)
                        .is_some_and(|kind| is_physical_kind(kind))
                    {
                        return Err(AwareError::Validation(format!(
                            "ifc write: `{path}.{field}` must reference a physical element"
                        )));
                    }
                }
                let weld_path = op.get("path").and_then(Value::as_array).ok_or_else(|| {
                    AwareError::Validation(format!("ifc write: `{path}.path` must be an array"))
                })?;
                if weld_path.len() < 2 || weld_path.iter().any(|point| vec3(Some(point)).is_none())
                {
                    return Err(AwareError::Validation(format!(
                        "ifc write: `{path}.path` must contain at least two Vec3 points"
                    )));
                }
                if op.get("weldType").and_then(Value::as_str) != Some("fillet") {
                    return Err(AwareError::Validation(format!(
                        "ifc write: `{path}.weldType` must be `fillet`"
                    )));
                }
                require_positive(op, "sizeMm", &path)?;
                for field in ["around", "shop"] {
                    if !op.get(field).is_some_and(Value::is_boolean) {
                        return Err(AwareError::Validation(format!(
                            "ifc write: `{path}.{field}` must be boolean"
                        )));
                    }
                }
            } else if op_kind == "boolean-cut" {
                let target = op.get("targetId").and_then(Value::as_str).unwrap_or("");
                if !element_kinds
                    .get(target)
                    .is_some_and(|kind| is_physical_kind(kind))
                {
                    return Err(AwareError::Validation(format!(
                        "ifc write: `{path}.targetId` must reference a physical element"
                    )));
                }
                let tool = op.get("tool").and_then(Value::as_object).ok_or_else(|| {
                    AwareError::Validation(format!("ifc write: `{path}.tool` is required"))
                })?;
                match tool.get("kind").and_then(Value::as_str) {
                    Some("cylinder") => {
                        let axis =
                            tool.get("axis").and_then(Value::as_object).ok_or_else(|| {
                                AwareError::Validation(format!(
                                    "ifc write: `{path}.tool.axis` is required"
                                ))
                            })?;
                        let from = vec3(axis.get("from"));
                        let to = vec3(axis.get("to"));
                        if !matches!((from, to), (Some(from), Some(to)) if normalized3([to[0]-from[0],to[1]-from[1],to[2]-from[2]]).is_some())
                        {
                            return Err(AwareError::Validation(format!(
                                "ifc write: `{path}.tool.axis` must contain distinct from/to Vec3 points"
                            )));
                        }
                        require_positive(
                            &Value::Object(tool.clone()),
                            "diameterMm",
                            &format!("{path}.tool"),
                        )?;
                    }
                    Some("box") => validate_box_tool(tool, &format!("{path}.tool"))?,
                    _ => {
                        return Err(AwareError::Validation(format!(
                            "ifc write: `{path}.tool.kind` must be `cylinder` or `box`"
                        )));
                    }
                }
            }
        }
    }

    if let Some(value) = scene.get("referenceSystems")
        && !value.is_array()
    {
        return Err(AwareError::Validation(
            "ifc write: `scene.referenceSystems` must be an array".into(),
        ));
    }
    if let Some(grids) = scene.get("referenceSystems").and_then(Value::as_array) {
        for (grid_index, grid) in grids.iter().enumerate() {
            let path = format!("scene.referenceSystems[{grid_index}]");
            require_id(grid, &path, &mut seen)?;
            if grid.get("kind").and_then(Value::as_str) != Some("structural-grid") {
                for collection in ["axes", "levels"] {
                    if let Some(children) = grid.get(collection).and_then(Value::as_array) {
                        for (child_index, child) in children.iter().enumerate() {
                            require_id(
                                child,
                                &format!("{path}.{collection}[{child_index}]"),
                                &mut seen,
                            )?;
                        }
                    }
                }
                continue;
            }
            if vec3(grid.get("origin")).is_none() {
                return Err(AwareError::Validation(format!(
                    "ifc write: `{path}.origin` must be Vec3"
                )));
            }
            let bounds = grid
                .get("bounds")
                .and_then(Value::as_object)
                .ok_or_else(|| {
                    AwareError::Validation(format!("ifc write: `{path}.bounds` is required"))
                })?;
            let finite_bound = |field: &str| {
                bounds
                    .get(field)
                    .and_then(Value::as_f64)
                    .filter(|value| value.is_finite())
                    .ok_or_else(|| {
                        AwareError::Validation(format!(
                            "ifc write: `{path}.bounds.{field}` must be finite"
                        ))
                    })
            };
            let (min_x, max_x, min_y, max_y) = (
                finite_bound("minX")?,
                finite_bound("maxX")?,
                finite_bound("minY")?,
                finite_bound("maxY")?,
            );
            if min_x >= max_x || min_y >= max_y {
                return Err(AwareError::Validation(format!(
                    "ifc write: `{path}.bounds` must have positive X and Y spans"
                )));
            }
            let axes = grid.get("axes").and_then(Value::as_array).ok_or_else(|| {
                AwareError::Validation(format!("ifc write: `{path}.axes` must be an array"))
            })?;
            let mut has_x = false;
            let mut has_y = false;
            for (axis_index, axis) in axes.iter().enumerate() {
                let axis_path = format!("{path}.axes[{axis_index}]");
                require_id(axis, &axis_path, &mut seen)?;
                if axis.get("label").and_then(Value::as_str).is_none() {
                    return Err(AwareError::Validation(format!(
                        "ifc write: `{axis_path}.label` must be a string"
                    )));
                }
                match axis.get("direction").and_then(Value::as_str) {
                    Some("x") => has_x = true,
                    Some("y") => has_y = true,
                    _ => {
                        return Err(AwareError::Validation(format!(
                            "ifc write: `{axis_path}.direction` must be `x` or `y`"
                        )));
                    }
                }
                if axis
                    .get("offsetMm")
                    .and_then(Value::as_f64)
                    .is_none_or(|value| !value.is_finite())
                {
                    return Err(AwareError::Validation(format!(
                        "ifc write: `{axis_path}.offsetMm` must be finite"
                    )));
                }
                for field in ["startMm", "endMm"] {
                    if axis.get(field).is_some_and(|value| {
                        value.as_f64().is_none_or(|number| !number.is_finite())
                    }) {
                        return Err(AwareError::Validation(format!(
                            "ifc write: `{axis_path}.{field}` must be finite when present"
                        )));
                    }
                }
            }
            if !has_x || !has_y {
                return Err(AwareError::Validation(format!(
                    "ifc write: `{path}.axes` must include both x and y families"
                )));
            }
            let levels = grid
                .get("levels")
                .and_then(Value::as_array)
                .ok_or_else(|| {
                    AwareError::Validation(format!("ifc write: `{path}.levels` must be an array"))
                })?;
            if levels.is_empty() {
                return Err(AwareError::Validation(format!(
                    "ifc write: `{path}.levels` must contain at least one elevation datum"
                )));
            }
            for (level_index, level) in levels.iter().enumerate() {
                let level_path = format!("{path}.levels[{level_index}]");
                require_id(level, &level_path, &mut seen)?;
                if level.get("label").and_then(Value::as_str).is_none() {
                    return Err(AwareError::Validation(format!(
                        "ifc write: `{level_path}.label` must be a string"
                    )));
                }
                if level
                    .get("elevationMm")
                    .and_then(Value::as_f64)
                    .is_none_or(|value| !value.is_finite())
                {
                    return Err(AwareError::Validation(format!(
                        "ifc write: `{level_path}.elevationMm` must be finite"
                    )));
                }
            }
        }
    }

    Ok(())
}

pub fn ifc_write(args: &Value, dry_run: bool) -> Result<Value, AwareError> {
    let scene = match args.get("scene") {
        Some(v @ Value::Object(_)) => v,
        None | Some(Value::Null) => {
            return Err(AwareError::Validation(
                "ifc write: `scene` is required (an object with `elements`)".into(),
            ));
        }
        Some(other) => {
            return Err(AwareError::Validation(format!(
                "ifc write: `scene` must be an object (got {})",
                json_type(other)
            )));
        }
    };

    validate_scene(scene)?;
    let built = try_build_ifc(scene)?;

    let mut out = serde_json::Map::new();
    out.insert("ok".into(), Value::Bool(true));
    out.insert("bytes".into(), Value::from(built.doc.len() as u64));
    out.insert("members".into(), Value::from(built.members as u64));
    out.insert("columns".into(), Value::from(built.columns));
    out.insert("beams".into(), Value::from(built.beams));
    out.insert("meshes".into(), Value::from(built.meshes));
    out.insert(
        "profiles".into(),
        Value::Object(
            built
                .profiles
                .into_iter()
                .map(|(k, v)| (k, Value::from(v)))
                .collect(),
        ),
    );
    out.insert("emitted".into(), Value::Array(built.emitted));
    out.insert("failed".into(), Value::Array(built.failed));
    out.insert("unsupported".into(), Value::Array(built.unsupported));
    out.insert(
        "warnings".into(),
        Value::Array(
            built
                .warnings
                .into_iter()
                .map(|(id, reason)| {
                    let mut m = serde_json::Map::new();
                    m.insert("id".into(), Value::String(id));
                    m.insert("reason".into(), Value::String(reason));
                    m.insert("status".into(), Value::String("warning".into()));
                    m.insert("kind".into(), Value::String("element".into()));
                    m.insert("code".into(), Value::String("profile-fallback".into()));
                    Value::Object(m)
                })
                .collect(),
        ),
    );

    if let Some(path) = args
        .get("output-path")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
    {
        if !dry_run {
            if let Some(parent) = std::path::Path::new(path).parent()
                && !parent.as_os_str().is_empty()
            {
                std::fs::create_dir_all(parent).map_err(|e| {
                    AwareError::Internal(format!("ifc: create {}: {e}", parent.display()))
                })?;
            }
            std::fs::write(path, built.doc.as_bytes())
                .map_err(|e| AwareError::Internal(format!("ifc: write {path}: {e}")))?;
        }
        out.insert("output-path".into(), Value::String(path.to_string()));
        out.insert("path".into(), Value::String(path.to_string()));
    }

    Ok(Value::Object(out))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn build_ifc(scene: &Value) -> BuildResult {
        try_build_ifc(scene).unwrap()
    }

    fn sample_scene() -> Value {
        json!({
            "meta": { "name": "Test frame (sample)", "units": "mm", "up": "z" },
            "elements": [
                { "id": "C1", "group": "column", "kind": "box",
                  "from": [0,0,0], "to": [0,0,3000], "section": { "w": 300, "d": 300 },
                  "meta": { "profile": "UC305x305x97" } },
                { "id": "B1", "group": "beam", "kind": "box",
                  "from": [0,0,3000], "to": [6000,0,3000], "section": { "w": 150, "d": 250 },
                  "meta": { "profile": "W10x33" } }
            ]
        })
    }

    #[test]
    fn rejects_y_up_and_malformed_roll_instead_of_coercing_to_zero() {
        let mut scene = sample_scene();
        scene["meta"]["up"] = json!("y");
        assert!(
            validate_scene(&scene)
                .unwrap_err()
                .to_string()
                .contains("unsupported")
        );

        for invalid in [Value::Null, json!("82.7"), json!(true)] {
            let mut scene = sample_scene();
            scene["elements"][0]["rot"] = invalid;
            let error = validate_scene(&scene).unwrap_err().to_string();
            assert!(error.contains("finite JSON number"), "{error}");
        }

        let mut non_member = sample_scene();
        non_member["elements"][0]["kind"] = json!("mesh");
        non_member["elements"][0]["positions"] = json!([0, 0, 0, 1, 0, 0, 0, 1, 0]);
        non_member["elements"][0]["indices"] = json!([0, 1, 2]);
        non_member["elements"][0]["rot"] = json!(10);
        assert!(
            validate_scene(&non_member)
                .unwrap_err()
                .to_string()
                .contains("applicable only")
        );
    }

    #[test]
    fn equivalent_normalized_rolls_emit_identical_ifc_geometry() {
        let mut first = sample_scene();
        first["elements"][0]["rot"] = json!(82.7);
        let mut wrapped = sample_scene();
        wrapped["elements"][0]["rot"] = json!(442.7);
        let rolled = build_ifc(&first).doc;
        assert_eq!(rolled, build_ifc(&wrapped).doc);

        // Golden 2D RefDirection: prove the authored roll reaches IFC geometry rather than only
        // being normalized and carried alongside an unchanged zero-roll profile placement.
        assert!(
            rolled.contains("IFCDIRECTION((0.1270646086,0.99189444259))"),
            "82.7-degree profile RefDirection was not emitted"
        );
        let mut zero = sample_scene();
        zero["elements"][0]["rot"] = json!(0.0);
        assert_ne!(rolled, build_ifc(&zero).doc);
    }

    #[test]
    fn writes_ifc4_with_a_column_and_a_beam() {
        let b = build_ifc(&sample_scene());
        assert_eq!(b.members, 2);
        assert_eq!(b.columns, 1);
        assert_eq!(b.beams, 1);
        assert_eq!(b.meshes, 0);
        assert_eq!(b.profiles.get("UC305x305x97"), Some(&1));
        assert_eq!(b.profiles.get("W10x33"), Some(&1));
        assert!(b.doc.starts_with("ISO-10303-21;"));
        assert!(b.doc.contains("FILE_SCHEMA(('IFC4'));"));
        assert!(b.doc.contains("DesignTransferView_V1.0"));
        assert_eq!(b.doc.matches("IFCCOLUMN(").count(), 1);
        assert_eq!(b.doc.matches("IFCBEAM(").count(), 1);
        // No xsection on the sample → rectangle fallback, no warning.
        assert_eq!(b.doc.matches("IFCRECTANGLEPROFILEDEF(").count(), 2);
        assert!(b.warnings.is_empty());
        assert_eq!(
            b.doc.matches("IFCRELCONTAINEDINSPATIALSTRUCTURE(").count(),
            1
        );
        assert!(b.doc.trim_end().ends_with("END-ISO-10303-21;"));
    }

    #[test]
    fn is_deterministic() {
        assert_eq!(
            build_ifc(&sample_scene()).doc,
            build_ifc(&sample_scene()).doc
        );
    }

    #[test]
    fn deterministic_with_materials_and_colors() {
        // Repeated + distinct materials/colours: iteration must be sorted, GlobalIds stable.
        let scene = json!({
            "meta": { "name": "m" },
            "groups": [
                { "key": "W16X26", "color": "#3b82f6" },
                { "key": "HSS6X6", "color": "#f59e0b" }
            ],
            "elements": [
                { "id": "b1", "group": "W16X26", "role": "beam", "material": "A992",
                  "from": [0,0,0], "to": [3000,0,0], "section": { "w": 140, "d": 400 },
                  "meta": { "profile": "W16X26" },
                  "xsection": { "shape": "i", "d": 400, "bf": 140, "tw": 6, "tf": 9 } },
                { "id": "b2", "group": "W16X26", "role": "beam", "material": "a992",
                  "from": [0,500,0], "to": [3000,500,0], "section": { "w": 140, "d": 400 },
                  "meta": { "profile": "W16X26" },
                  "xsection": { "shape": "i", "d": 400, "bf": 140, "tw": 6, "tf": 9 } },
                { "id": "c1", "group": "HSS6X6", "role": "column", "material": "A500-GR.B",
                  "from": [0,0,0], "to": [0,0,3000], "section": { "w": 152, "d": 152 },
                  "meta": { "profile": "HSS6X6X3/8" },
                  "xsection": { "shape": "rhs", "d": 152, "b": 152, "t": 9 } }
            ]
        });
        let a = build_ifc(&scene);
        let b = build_ifc(&scene);
        assert_eq!(a.doc, b.doc, "identical scene → identical bytes");
        // "A992" and "a992" normalize to ONE material; A500-GR.B is a second.
        assert_eq!(a.doc.matches("IFCMATERIAL(").count(), 2);
        assert_eq!(a.doc.matches("IFCRELASSOCIATESMATERIAL(").count(), 2);
        // Two distinct colours → two surface styles; three elements → three styled items.
        assert_eq!(a.doc.matches("IFCSURFACESTYLE(").count(), 2);
        assert_eq!(a.doc.matches("IFCSTYLEDITEM(").count(), 3);
        assert_eq!(a.doc.matches("IFCISHAPEPROFILEDEF(").count(), 2);
        assert_eq!(a.doc.matches("IFCRECTANGLEHOLLOWPROFILEDEF(").count(), 1);
    }

    #[test]
    fn real_profiles_by_shape() {
        let mk = |xs: Value, prof: &str| {
            json!({ "meta": { "name": "x" }, "elements": [
                { "id": "e", "group": "beam", "role": "beam",
                  "from": [0,0,0], "to": [3000,0,0], "section": { "w": 100, "d": 300 },
                  "meta": { "profile": prof }, "xsection": xs }
            ]})
        };
        assert!(
            build_ifc(&mk(
                json!({"shape":"i","d":400,"bf":140,"tw":6,"tf":9}),
                "W16X26"
            ))
            .doc
            .contains("IFCISHAPEPROFILEDEF(")
        );
        assert!(
            build_ifc(&mk(
                json!({"shape":"channel","d":152,"bf":48,"tw":5,"tf":8}),
                "C6X8.2"
            ))
            .doc
            .contains("IFCUSHAPEPROFILEDEF(")
        );
        assert!(
            build_ifc(&mk(
                json!({"shape":"angle","d":102,"b":102,"t":9}),
                "L4X4X3/8"
            ))
            .doc
            .contains("IFCLSHAPEPROFILEDEF(")
        );
        assert!(
            build_ifc(&mk(json!({"shape":"rhs","d":152,"b":152,"t":9}), "HSS6X6"))
                .doc
                .contains("IFCRECTANGLEHOLLOWPROFILEDEF(")
        );
        assert!(
            build_ifc(&mk(json!({"shape":"chs","od":168,"t":7}), "HSS6.625"))
                .doc
                .contains("IFCCIRCLEHOLLOWPROFILEDEF(")
        );
        let tee = json!({ "meta": { "name": "x" }, "elements": [{
            "id":"tee", "from":[0,0,0], "to":[3000,0,0],
            "section":{"w":200,"d":300}, "xsection":{"shape":"tee","d":300,"bf":200,"tw":10,"tf":20}
        }]});
        assert!(build_ifc(&tee).doc.contains("IFCTSHAPEPROFILEDEF("));
    }

    #[test]
    fn double_angle_materializes_as_two_exact_composite_profiles() {
        for (orientation, w, d) in [("llbb", 212, 150), ("slbb", 312, 100)] {
            let scene = json!({ "meta": { "name": "x" }, "elements": [{
                "id":"2L", "from":[0,0,0], "to":[3000,0,0],
                "section":{"w":w,"d":d}, "xsection":{
                    "shape":"double-angle","d":150,"b":100,"t":10,"gap":12,"orientation":orientation
                }
            }]});
            let result = build_ifc(&scene);
            assert!(result.doc.contains("IFCCOMPOSITEPROFILEDEF("));
            assert_eq!(
                result.doc.matches("IFCARBITRARYCLOSEDPROFILEDEF(").count(),
                2
            );
            assert!(result.warnings.is_empty());
        }
    }

    #[test]
    fn double_angle_outlines_preserve_exact_gap_and_never_overlap() {
        for (orientation, expected_width, expected_depth) in
            [("llbb", 212.0, 150.0), ("slbb", 312.0, 100.0)]
        {
            let [left, right] = double_angle_outlines(150.0, 100.0, 10.0, 12.0, orientation)
                .expect("valid double angle");
            let left_max = left.iter().map(|p| p[0]).fold(f64::NEG_INFINITY, f64::max);
            let right_min = right.iter().map(|p| p[0]).fold(f64::INFINITY, f64::min);
            let min_x = left
                .iter()
                .chain(&right)
                .map(|p| p[0])
                .fold(f64::INFINITY, f64::min);
            let max_x = left
                .iter()
                .chain(&right)
                .map(|p| p[0])
                .fold(f64::NEG_INFINITY, f64::max);
            let min_y = left
                .iter()
                .chain(&right)
                .map(|p| p[1])
                .fold(f64::INFINITY, f64::min);
            let max_y = left
                .iter()
                .chain(&right)
                .map(|p| p[1])
                .fold(f64::NEG_INFINITY, f64::max);
            assert_eq!(right_min - left_max, 12.0);
            assert_eq!(max_x - min_x, expected_width);
            assert_eq!(max_y - min_y, expected_depth);
        }

        let [left, right] =
            double_angle_outlines(150.0, 100.0, 10.0, 0.0, "slbb").expect("touching");
        assert_eq!(
            right.iter().map(|p| p[0]).fold(f64::INFINITY, f64::min)
                - left.iter().map(|p| p[0]).fold(f64::NEG_INFINITY, f64::max),
            0.0
        );
        assert!(double_angle_outlines(150.0, 100.0, 10.0, -0.001, "slbb").is_none());
    }

    #[test]
    fn invalid_xsection_falls_back_to_rectangle_with_warning() {
        // tf too large (2·tf >= d) → invalid I-shape → rectangle + one warning.
        let scene = json!({ "meta": { "name": "x" }, "elements": [
            { "id": "bad", "group": "beam", "role": "beam",
              "from": [0,0,0], "to": [3000,0,0], "section": { "w": 100, "d": 200 },
              "meta": { "profile": "W" }, "xsection": { "shape": "i", "d": 200, "bf": 100, "tw": 6, "tf": 120 } }
        ]});
        let b = build_ifc(&scene);
        assert!(b.doc.contains("IFCRECTANGLEPROFILEDEF("));
        assert!(!b.doc.contains("IFCISHAPEPROFILEDEF("));
        assert_eq!(b.warnings.len(), 1);
        assert_eq!(b.warnings[0].0, "bad");
    }

    #[test]
    fn malformed_xsection_non_object_warns_absent_does_not() {
        // A present-but-non-object xsection → rectangle + warning; an absent one → rectangle, no warning.
        let el = |xs: Value| {
            json!({ "meta": { "name": "x" }, "elements": [
                { "id": "m", "group": "beam", "role": "beam", "from": [0,0,0], "to": [3000,0,0],
                  "section": { "w": 100, "d": 200 }, "meta": { "profile": "W" }, "xsection": xs }
            ]})
        };
        let bad = build_ifc(&el(json!("not-an-object")));
        assert_eq!(bad.warnings.len(), 1);
        assert_eq!(bad.warnings[0].0, "m");
        assert!(bad.doc.contains("IFCRECTANGLEPROFILEDEF("));
        // Absent xsection → no warning (backward compat).
        let absent = build_ifc(&json!({ "meta": { "name": "x" }, "elements": [
            { "id": "m", "group": "beam", "from": [0,0,0], "to": [3000,0,0], "section": { "w": 100, "d": 200 } }
        ]}));
        assert!(absent.warnings.is_empty());
    }

    #[test]
    fn role_types_brace_as_member_and_column_as_column() {
        let scene = json!({ "meta": { "name": "x" }, "elements": [
            { "id": "br", "group": "W12X19", "role": "brace",
              "from": [0,0,0], "to": [3000,0,3000], "section": { "w": 100, "d": 300 },
              "meta": { "profile": "W12X19" } },
            { "id": "co", "group": "W14X193", "role": "column",
              "from": [0,0,0], "to": [0,0,3000], "section": { "w": 400, "d": 400 },
              "meta": { "profile": "W14X193" } }
        ]});
        let b = build_ifc(&scene);
        assert_eq!(b.doc.matches("IFCMEMBER(").count(), 1); // brace
        assert_eq!(b.doc.matches("IFCCOLUMN(").count(), 1);
        assert_eq!(b.columns, 1);
        assert_eq!(b.beams, 0);
    }

    #[test]
    fn the_receipt_marks_only_members_whose_facing_is_convention() {
        // Issue #432: inside the vertical band the zero frame is seeded from scene `+X`
        // and the section's facing is convention, not geometry. The receipt says so per
        // member, and says nothing for the ordinary case, so a reader reconciling this
        // file against another toolchain can find the members the two may legitimately
        // disagree about. `lean` is one step past the threshold, `plumb` a true column.
        let scene = json!({ "meta": { "name": "x" }, "elements": [
            { "id": "plumb", "group": "g", "from": [0,0,0], "to": [0,0,3000],
              "section": { "w": 400, "d": 400 } },
            { "id": "lean", "group": "g", "from": [0,0,0], "to": [3.003,0,3000],
              "section": { "w": 400, "d": 400 } },
            { "id": "beam", "group": "g", "from": [0,0,0], "to": [3000,0,0],
              "section": { "w": 400, "d": 400 } }
        ]});
        let zero_frame_of = |id: &str| {
            build_ifc(&scene)
                .emitted
                .iter()
                .find(|row| row.get("id").and_then(Value::as_str) == Some(id))
                .and_then(|row| row.get("zeroFrame").and_then(Value::as_str))
                .map(str::to_string)
        };
        assert_eq!(zero_frame_of("plumb").as_deref(), Some("vertical-seed"));
        assert_eq!(zero_frame_of("lean"), None);
        assert_eq!(zero_frame_of("beam"), None);
    }

    #[test]
    fn dry_run_returns_counts_and_writes_nothing() {
        let out = ifc_write(&json!({ "scene": sample_scene() }), true).unwrap();
        assert_eq!(out["members"].as_u64().unwrap(), 2);
        assert_eq!(out["columns"].as_u64().unwrap(), 1);
        assert_eq!(out["beams"].as_u64().unwrap(), 1);
        assert!(out["bytes"].as_u64().unwrap() > 0);
        assert!(out["warnings"].as_array().unwrap().is_empty());
        assert!(out.get("path").is_none());
    }

    #[test]
    fn brace_group_fallback_when_no_role() {
        let scene = json!({
            "meta": { "name": "x" },
            "elements": [
                { "id": "BR1", "group": "brace", "from": [0,0,0], "to": [3000,0,3000],
                  "section": { "w": 90, "d": 90 }, "meta": { "profile": "L90" } }
            ]
        });
        let b = build_ifc(&scene);
        assert_eq!(b.members, 1);
        assert_eq!(b.columns, 0);
        assert_eq!(b.beams, 0);
        assert_eq!(b.doc.matches("IFCMEMBER(").count(), 1);
        assert_eq!(b.profiles.get("L90"), Some(&1));
    }

    #[test]
    fn degenerate_and_malformed_elements_are_skipped() {
        let scene = json!({
            "meta": { "name": "x" },
            "elements": [
                { "id": "ZERO", "group": "beam", "from": [0,0,0], "to": [0,0,0] },
                { "id": "SHORT", "group": "beam", "from": [0,0], "to": [1,1] },
                { "id": "OK", "group": "beam", "from": [0,0,0], "to": [1000,0,0] }
            ]
        });
        let b = build_ifc(&scene);
        assert_eq!(b.members, 1);
        assert_eq!(b.beams, 1);
    }

    #[test]
    fn writes_a_triangulated_mesh_element() {
        let scene = json!({
            "meta": { "name": "conn" },
            "elements": [
                { "id": "PL-1", "kind": "mesh", "group": "connection",
                  "positions": [0.0,0.0,0.0, 100.0,0.0,0.0, 100.0,100.0,0.0, 0.0,100.0,0.0],
                  "indices": [0,1,2, 0,2,3],
                  "meta": { "profile": "PLATE" } }
            ]
        });
        let b = build_ifc(&scene);
        assert_eq!(b.members, 1);
        assert_eq!(b.meshes, 1);
        assert_eq!(b.profiles.get("PLATE"), Some(&1));
        assert_eq!(b.doc.matches("IFCTRIANGULATEDFACESET(").count(), 1);
        assert!(b.doc.contains("((1,2,3),(1,3,4))"));
        assert!(b.doc.contains("'Tessellation'"));
    }

    #[test]
    fn styled_and_unstyled_mesh() {
        // A mesh with a valid group colour gets a styled item; one without gets none.
        let styled = json!({ "meta": { "name": "x" }, "groups": [{ "key": "c", "color": "#abcdef" }], "elements": [
            { "id": "m", "kind": "mesh", "group": "c",
              "positions": [0,0,0, 1,0,0, 1,1,0], "indices": [0,1,2] }
        ]});
        let plain = json!({ "meta": { "name": "x" }, "elements": [
            { "id": "m", "kind": "mesh", "group": "c",
              "positions": [0,0,0, 1,0,0, 1,1,0], "indices": [0,1,2] }
        ]});
        assert_eq!(build_ifc(&styled).doc.matches("IFCSTYLEDITEM(").count(), 1);
        assert_eq!(build_ifc(&plain).doc.matches("IFCSTYLEDITEM(").count(), 0);
    }

    #[test]
    fn missing_scene_is_a_validation_error() {
        let err = ifc_write(&json!({}), true).unwrap_err();
        assert!(matches!(err, AwareError::Validation(_)));
    }

    #[test]
    fn non_object_scene_is_a_validation_error() {
        let err = ifc_write(&json!({ "scene": "nope" }), true).unwrap_err();
        assert!(matches!(err, AwareError::Validation(_)));
    }

    #[test]
    fn s_lit_applies_part21_escaping() {
        assert_eq!(s_lit("plain text"), "'plain text'");
        assert_eq!(s_lit("a'b"), "'a''b'");
        assert_eq!(s_lit("a\\b"), "'a\\\\b'");
        assert_eq!(s_lit("a\u{2014}b"), "'a\\X2\\2014\\X0\\b'");
    }

    #[test]
    fn polyline_emitter_only_closes_profile_boundaries() {
        let mut open = Spf::new();
        emit_polyline2(&mut open, &[[0.0, 0.0], [1.0, 0.0]], false);
        assert!(open.buf.contains("IFCPOLYLINE((#1,#2))"));

        let mut closed = Spf::new();
        emit_polyline2(&mut closed, &[[0.0, 0.0], [1.0, 0.0]], true);
        assert!(closed.buf.contains("IFCPOLYLINE((#1,#2,#1))"));
    }

    fn connection_scene() -> Value {
        json!({
            "meta": { "name": "Connection and grid", "units": "mm", "sourceId": "takeoff-1", "sceneHash": "sha256:test" },
            "elements": [
                { "id": "BEAM-1", "kind": "member", "role": "beam",
                  "from": [0,0,0], "to": [3000,0,0], "section": { "w": 200, "d": 400 } },
                { "id": "PL-1", "kind": "plate", "material": "S355",
                  "frame": { "origin": [100,0,0], "uDir": [1,0,0], "vDir": [0,1,0], "normal": [0,0,1] },
                  "outline": [[-100,-150],[100,-150],[100,150],[-100,150]], "thicknessMm": 12,
                  "holes": [{ "id": "PL-1-H-INDEPENDENT", "center": [0,100], "diameterMm": 22 }] },
                { "id": "SHANK-1", "kind": "bolt-shank", "axis": { "from": [100,0,-20], "to": [100,0,20] },
                  "diameterMm": 20, "fastener": { "role": "shear-bolt" } },
                { "id": "HEAD-1", "kind": "bolt-head", "center": [100,0,24], "axis": [0,0,1],
                  "acrossFlatsMm": 30, "thicknessMm": 8 },
                { "id": "NUT-1", "kind": "nut", "center": [100,0,-24], "axis": [0,0,1],
                  "acrossFlatsMm": 30, "thicknessMm": 10, "fastener": { "role": "nut" } },
                { "id": "WASHER-1", "kind": "washer", "center": [100,0,-16], "axis": [0,0,1],
                  "outerDiameterMm": 36, "innerDiameterMm": 22, "thicknessMm": 3 }
            ],
            "operations": [
                { "id": "BA-1", "kind": "bolt-array", "partToBoltTo": "PL-1", "partToBeBolted": "BEAM-1",
                  "frame": { "origin": [100,0,0], "uDir": [1,0,0], "vDir": [0,1,0], "normal": [0,0,1] },
                  "uOffsetsMm": [0], "vOffsetsMm": [0], "diameterMm": 20, "standard": "A325N", "toleranceMm": 2,
                  "boltType": "shop", "components": { "bolt": true, "nut": true, "washer": true },
                  "instances": [{ "id": "BA-1-I1", "point": [100,0,0], "shankId": "SHANK-1", "headId": "HEAD-1",
                    "nutIds": ["NUT-1"], "washerIds": ["WASHER-1"],
                    "holeEffects": [
                      { "id": "BA-1-H-PL", "targetId": "PL-1", "center": [100,0,0], "axis": [0,0,1], "diameterMm": 22 },
                      { "id": "BA-1-H-BEAM", "targetId": "BEAM-1", "center": [100,0,0], "axis": [0,0,1], "diameterMm": 22 }
                    ] }] },
                { "id": "WELD-1", "kind": "weld", "mainId": "BEAM-1", "secondaryId": "PL-1",
                  "path": [[0,0,0],[0,100,0]], "weldType": "fillet", "sizeMm": 6, "around": false, "shop": true },
                { "id": "CUT-1", "kind": "boolean-cut", "targetId": "BEAM-1",
                  "tool": { "kind": "cylinder", "axis": { "from": [0,0,-10], "to": [0,0,10] }, "diameterMm": 20 } }
            ],
            "referenceSystems": [{
                "id": "GRID-1", "kind": "structural-grid", "name": "Main grid", "origin": [0,0,0],
                "axes": [
                  { "id": "GRID-X-A", "direction": "x", "offsetMm": 0, "label": "A" },
                  { "id": "GRID-Y-1", "direction": "y", "offsetMm": 0, "label": "1" }
                ],
                "levels": [{ "id": "LEVEL-0", "elevationMm": 0, "label": "TOS 0" }],
                "bounds": { "minX": -1000, "maxX": 4000, "minY": -2000, "maxY": 2000 }
            }]
        })
    }

    #[test]
    fn writes_connection_products_voids_grid_and_exhaustive_receipt() {
        let scene = connection_scene();
        validate_scene(&scene).unwrap();
        let built = build_ifc(&scene);
        assert_eq!(built.doc.matches("IFCPLATE(").count(), 1);
        assert_eq!(built.doc.matches("IFCMECHANICALFASTENER(").count(), 1);
        assert_eq!(built.doc.matches("IFCOPENINGELEMENT(").count(), 2);
        // Two bolt-hole-effect openings + one cylinder cut (CUT-1) now voided as an IfcVoidingFeature(.HOLE.).
        assert_eq!(built.doc.matches("IFCRELVOIDSELEMENT(").count(), 3);
        assert_eq!(built.doc.matches("IFCVOIDINGFEATURE(").count(), 1);
        assert!(
            built.doc.contains(".HOLE.)"),
            "the cylinder cut is a HOLE voiding feature"
        );
        // WELD-1 is materialized as an IfcFastener(.WELD.) realizing the member↔plate connection.
        assert_eq!(built.doc.matches("IFCFASTENER(").count(), 1);
        assert_eq!(
            built
                .doc
                .matches("IFCRELCONNECTSWITHREALIZINGELEMENTS(")
                .count(),
            1
        );
        assert!(
            built.doc.contains("'Pset_FastenerWeld'")
                && built.doc.contains("IFCPOSITIVELENGTHMEASURE(6.0)")
        );
        // The artifact is bound to the exact exported scene identity.
        assert!(
            built.doc.contains("'Pset_AWARE_ExportIdentity'")
                && built.doc.contains("IFCIDENTIFIER('takeoff-1')")
        );
        assert_eq!(built.doc.matches("IFCGRID(").count(), 1);
        assert_eq!(built.doc.matches("IFCGRIDAXIS(").count(), 2);
        assert_eq!(built.doc.matches("IFCANNOTATION(").count(), 1);
        assert_eq!(built.profiles.get("BOLT-SHANK"), Some(&1));
        assert_eq!(built.profiles.get("BOLT-HEAD"), Some(&1));
        assert_eq!(built.profiles.get("NUT"), Some(&1));
        assert_eq!(built.profiles.get("WASHER"), Some(&1));
        assert!(built.doc.contains("'Elevation datum'"));
        assert!(built.doc.contains(".BOLT."));
        let fastener_line = built
            .doc
            .lines()
            .find(|line| line.contains("IFCMECHANICALFASTENER("))
            .unwrap();
        assert!(
            fastener_line.contains(",$,20.0,40.0,.BOLT.);"),
            "the grouped array carries bolt semantics and nominal dimensions"
        );
        let grouped_shape_line = built
            .doc
            .lines()
            .find(|line| {
                line.contains("IFCSHAPEREPRESENTATION(")
                    && line.contains("'Body','SweptSolid'")
                    && line.matches('#').count() == 6
            })
            .expect("one grouped body shape must contain four fastener solids");
        assert_eq!(
            grouped_shape_line.matches('#').count(),
            6,
            "entity id, context, and four physical component solids"
        );
        // Welds and copes are now materialized, so nothing is left unsupported.
        assert_eq!(built.unsupported.len(), 0);
        assert_eq!(built.emitted.len(), 17);
        assert!(built.failed.is_empty());
        let ids = built
            .emitted
            .iter()
            .filter_map(|row| row.get("id").and_then(Value::as_str))
            .collect::<BTreeSet<_>>();
        assert_eq!(
            ids.len(),
            built.emitted.len(),
            "every receipt row is unique"
        );
        let receipt_ids = built
            .emitted
            .iter()
            .filter_map(|row| row.get("id").and_then(Value::as_str))
            .collect::<Vec<_>>();
        let position = |id| {
            receipt_ids
                .iter()
                .position(|candidate| *candidate == id)
                .unwrap()
        };
        assert!(position("GRID-1") < position("GRID-X-A"));
        assert!(position("GRID-X-A") < position("GRID-Y-1"));
        assert!(position("GRID-Y-1") < position("LEVEL-0"));
        assert!(
            built
                .doc
                .contains("IFCCARTESIANPOINT((17.32050807569,0.0))"),
            "phaseRad=0 places the first hex vertex on local +u"
        );
        let group_guid = record_guid("bolt-array", "BA-1");
        for id in ["BA-1", "BA-1-I1", "SHANK-1", "HEAD-1", "NUT-1", "WASHER-1"] {
            let row = built
                .emitted
                .iter()
                .find(|row| row.get("id").and_then(Value::as_str) == Some(id))
                .unwrap();
            assert_eq!(
                row.get("ifcGuid").and_then(Value::as_str),
                Some(group_guid.as_str()),
                "{id} must resolve to the selectable bolt-array product"
            );
        }
    }

    #[test]
    fn record_global_id_is_stable_and_pinned() {
        assert_eq!(record_guid("element", "PL-1"), "0YATHppfuyHQeKy4B2inuk");
        let before = build_ifc(&connection_scene()).doc;
        let mut after_scene = connection_scene();
        after_scene["elements"].as_array_mut().unwrap().insert(
            0,
            json!({ "id": "UNRELATED", "kind": "member", "from": [0,0,0], "to": [0,0,1000] }),
        );
        let after = build_ifc(&after_scene).doc;
        let gid = record_guid("element", "PL-1");
        assert!(before.contains(&s_lit(&gid)));
        assert!(after.contains(&s_lit(&gid)));
    }

    #[test]
    fn rejects_duplicate_ids_and_explicit_non_mm_units() {
        let mut duplicate = connection_scene();
        duplicate["referenceSystems"][0]["levels"][0]["id"] = json!("PL-1");
        let err = ifc_write(&json!({ "scene": duplicate }), true).unwrap_err();
        assert!(err.to_string().contains("duplicate scene record id `PL-1`"));

        let mut metres = connection_scene();
        metres["meta"]["units"] = json!("m");
        let err = ifc_write(&json!({ "scene": metres }), true).unwrap_err();
        assert!(err.to_string().contains("only `mm` is accepted"));

        let mut numeric = connection_scene();
        numeric["meta"]["units"] = json!(1);
        let err = ifc_write(&json!({ "scene": numeric }), true).unwrap_err();
        assert!(err.to_string().contains("must be the string `mm`"));
    }

    #[test]
    fn connection_validation_accepts_zero_tolerance_and_line_participants() {
        let mut scene = connection_scene();
        scene["elements"][0]["kind"] = json!("line");
        scene["operations"].as_array_mut().unwrap().remove(2);
        scene["operations"][0]["toleranceMm"] = json!(0);
        for effect in scene["operations"][0]["instances"][0]["holeEffects"]
            .as_array_mut()
            .unwrap()
        {
            effect["diameterMm"] = json!(20);
        }

        validate_scene(&scene).unwrap();
        let built = build_ifc(&scene);
        assert_eq!(built.doc.matches("IFCOPENINGELEMENT(").count(), 2);
        assert!(built.failed.is_empty());
    }

    #[test]
    fn connection_validation_rejects_non_cartesian_instance_counts() {
        let mut scene = connection_scene();
        scene["operations"][0]["uOffsetsMm"] = json!([0, 100]);
        let err = validate_scene(&scene).unwrap_err();
        assert!(err.to_string().contains("Cartesian offset count"));

        let mut duplicate = connection_scene();
        duplicate["operations"][0]["uOffsetsMm"] = json!([0, 0]);
        let err = validate_scene(&duplicate).unwrap_err();
        assert!(err.to_string().contains("unique offsets"));
    }

    #[test]
    fn connection_validation_rejects_disabled_or_non_boolean_bolt_components() {
        let mut disabled = connection_scene();
        disabled["operations"][0]["components"]["bolt"] = json!(false);
        let err = validate_scene(&disabled).unwrap_err();
        assert!(err.to_string().contains("components.bolt` must be true"));

        let mut non_boolean = connection_scene();
        non_boolean["operations"][0]["components"]["washer"] = json!("yes");
        let err = validate_scene(&non_boolean).unwrap_err();
        assert!(
            err.to_string()
                .contains("components.washer` must be boolean")
        );
    }

    #[test]
    fn connection_validation_requires_bolt_standard_and_grid_labels_and_levels() {
        let mut no_standard = connection_scene();
        no_standard["operations"][0]
            .as_object_mut()
            .unwrap()
            .remove("standard");
        let err = validate_scene(&no_standard).unwrap_err();
        assert!(err.to_string().contains("standard"));

        let mut no_axis_label = connection_scene();
        no_axis_label["referenceSystems"][0]["axes"][0]
            .as_object_mut()
            .unwrap()
            .remove("label");
        let err = validate_scene(&no_axis_label).unwrap_err();
        assert!(err.to_string().contains("axes[0].label"));

        let mut no_level_label = connection_scene();
        no_level_label["referenceSystems"][0]["levels"][0]
            .as_object_mut()
            .unwrap()
            .remove("label");
        let err = validate_scene(&no_level_label).unwrap_err();
        assert!(err.to_string().contains("levels[0].label"));

        let mut empty_levels = connection_scene();
        empty_levels["referenceSystems"][0]["levels"] = json!([]);
        let err = validate_scene(&empty_levels).unwrap_err();
        assert!(err.to_string().contains("at least one elevation datum"));
    }

    #[test]
    fn connection_validation_rejects_plate_holes_outside_or_overlapping() {
        let mut outside = connection_scene();
        outside["elements"][1]["holes"][0]["center"] = json!([95, 100]);
        let err = validate_scene(&outside).unwrap_err();
        assert!(err.to_string().contains("wholly inside the plate outline"));

        let mut overlap = connection_scene();
        overlap["elements"][1]["holes"]
            .as_array_mut()
            .unwrap()
            .push(json!({ "id": "PL-1-H-OVERLAP", "center": [0,110], "diameterMm": 22 }));
        let err = validate_scene(&overlap).unwrap_err();
        assert!(err.to_string().contains("must not overlap or touch"));

        let mut bowtie = connection_scene();
        bowtie["elements"][1]["outline"] =
            json!([[-100, -150], [100, 150], [-100, 150], [100, -150]]);
        let err = validate_scene(&bowtie).unwrap_err();
        assert!(err.to_string().contains("nonzero, simple polygon"));
    }

    #[test]
    fn connection_openings_use_a_conservative_through_product_span() {
        let built = build_ifc(&connection_scene());
        let conservative_depth =
            2.0 * (200.0_f64.powi(2) + 300.0_f64.powi(2) + 12.0_f64.powi(2)).sqrt() + 2.0;
        assert!(
            built.doc.contains(&format!(",{});", r(conservative_depth))),
            "plate opening must span oblique axes, not only plate thickness"
        );
    }

    #[test]
    fn connection_openings_cover_fastener_and_mesh_participants() {
        let mut fastener = connection_scene();
        fastener["elements"].as_array_mut().unwrap().push(json!({
            "id": "TARGET-ROD",
            "kind": "rod",
            "axis": { "from": [95,0,0], "to": [105,0,0] },
            "diameterMm": 1000
        }));
        fastener["operations"][0]["partToBeBolted"] = json!("TARGET-ROD");
        fastener["operations"][0]["instances"][0]["holeEffects"][1]["targetId"] =
            json!("TARGET-ROD");
        validate_scene(&fastener).unwrap();
        let built = build_ifc(&fastener);
        let fastener_depth = 2.0 * (10.0_f64.powi(2) + 2.0 * 1000.0_f64.powi(2)).sqrt() + 2.0;
        assert!(
            built.doc.contains(&format!(",{});", r(fastener_depth))),
            "transverse openings must span the complete standalone fastener"
        );

        let mut mesh = connection_scene();
        mesh["elements"][0] = json!({
            "id": "BEAM-1",
            "kind": "mesh",
            "positions": [0,0,-500, 1000,0,500, 0,1000,500],
            "indices": [0,1,2]
        });
        validate_scene(&mesh).unwrap();
        let built = build_ifc(&mesh);
        let mesh_depth =
            2.0 * (1000.0_f64.powi(2) + 1000.0_f64.powi(2) + 1000.0_f64.powi(2)).sqrt() + 2.0;
        assert!(
            built.doc.contains(&format!(",{});", r(mesh_depth))),
            "mesh openings must span the complete authored mesh bounds"
        );
    }

    #[test]
    fn grouped_bolt_array_rejects_mixed_authored_materials() {
        let mut scene = connection_scene();
        scene["elements"][2]["material"] = json!("A325");
        scene["elements"][3]["material"] = json!("A490");

        let err = validate_scene(&scene).unwrap_err();
        assert!(
            err.to_string()
                .contains("bolt components must use one normalized material"),
            "{err}"
        );
    }

    #[test]
    fn connection_preflight_rejects_degenerate_legacy_and_mesh_geometry() {
        let mut member = connection_scene();
        member["elements"][0]["to"] = member["elements"][0]["from"].clone();
        let err = validate_scene(&member).unwrap_err();
        assert!(err.to_string().contains("distinct finite from/to"), "{err}");

        let mut mesh = connection_scene();
        mesh["elements"].as_array_mut().unwrap().push(json!({
            "id": "BAD-MESH", "kind": "mesh",
            "positions": [0,0,0, 1,0,0, 1,1,0], "indices": [0,1,3]
        }));
        let err = validate_scene(&mesh).unwrap_err();
        assert!(err.to_string().contains("valid triangle index triples"));

        let mut skewed_plate = connection_scene();
        skewed_plate["elements"][1]["frame"]["vDir"] = json!([0.5, 1, 0]);
        let err = validate_scene(&skewed_plate).unwrap_err();
        assert!(err.to_string().contains("right-handed and orthonormal"));
    }

    #[test]
    fn connection_preflight_preserves_implicit_mesh_detection() {
        let scene = json!({
            "elements": [{
                "id": "LEGACY-MESH",
                "positions": [0,0,0, 1,0,0, 0,1,0],
                "indices": [0,1,2]
            }]
        });

        validate_scene(&scene).unwrap();
        let built = build_ifc(&scene);
        assert_eq!(built.meshes, 1);
        assert!(built.emitted.iter().any(|row| row["id"] == "LEGACY-MESH"));
    }

    #[test]
    fn bolt_participants_must_be_independently_materialized_products() {
        let mut scene = connection_scene();
        scene["operations"][0]["partToBoltTo"] = json!("SHANK-1");

        let err = validate_scene(&scene).unwrap_err();
        assert!(
            err.to_string()
                .contains("independently materialized physical element")
        );
    }

    /// A double-sided gusset: plate + member + plate clamped by ONE bolt, i.e. double shear. Three
    /// plies, three holes. The commonest multi-ply detail there is, and the shape the old two-effect
    /// cap refused outright — taking the whole scene down with it.
    fn double_shear_scene() -> Value {
        let mut scene = connection_scene();
        scene["elements"].as_array_mut().unwrap().push(json!({
            "id": "PL-2", "kind": "plate", "material": "S355",
            "frame": { "origin": [100,0,-40], "uDir": [1,0,0], "vDir": [0,1,0], "normal": [0,0,1] },
            "outline": [[-100,-150],[100,-150],[100,150],[-100,150]], "thicknessMm": 12
        }));
        scene["operations"][0]["instances"][0]["holeEffects"]
            .as_array_mut()
            .unwrap()
            .push(json!({ "id": "BA-1-H-PL2", "targetId": "PL-2",
                          "center": [100,0,0], "axis": [0,0,1], "diameterMm": 22 }));
        scene
    }

    /// Pins the orthonormality rejection across this module's frame-parsing
    /// rewrite (validation and extraction merged into `frame_vec3` /
    /// `frame_direction`).
    ///
    /// Unlike the viewer's equivalent, this is *not* a panic regression test —
    /// the `normalized3(cross3(u, v)).unwrap()` it replaced was unreachable, and
    /// a mutation check confirms it: `dot3(u, v).abs() > 1.0e-6` short-circuits
    /// first for a parallel frame, and for a genuinely perpendicular pair of
    /// unit vectors the cross product has length 1, never 0. This module's
    /// `normalized3` also guards `n.is_finite()`, so the overflowing-direction
    /// route that *does* reach the viewer's panic is caught here as an invalid
    /// `Vec3` well before the frame check.
    #[test]
    fn parallel_bolt_frame_is_rejected() {
        let mut scene = double_shear_scene();
        scene["operations"][0]["frame"]["vDir"] = json!([1, 0, 0]);
        let error = validate_scene(&scene).unwrap_err();
        assert!(
            error.to_string().contains("right-handed and orthonormal"),
            "expected an orthonormality error, got: {error}"
        );
    }

    #[test]
    fn a_bolt_may_hole_more_plies_than_the_declared_pair() {
        let scene = double_shear_scene();
        validate_scene(&scene).unwrap();

        // Accepting the scene is only half of it: the far plate must actually be VOIDED. A relaxation
        // that validated the third effect and then dropped it would ship a plate fabricated with no
        // hole in it, which is worse than the refusal it replaced.
        let built = build_ifc(&scene);
        let row = built
            .emitted
            .iter()
            .find(|row| row["id"] == "BA-1-H-PL2")
            .expect("the third ply's hole effect is emitted");
        assert_eq!(row["targetId"], json!("PL-2"));
        assert_eq!(row["entityType"], json!("IfcOpeningElement"));
        assert_eq!(
            built.doc.matches("IFCRELVOIDSELEMENT(").count(),
            4,
            "three bolt holes (plate, member, far plate) plus the plate's own independent hole"
        );
    }

    #[test]
    fn an_extra_ply_must_still_be_a_holeable_element() {
        // Unknown id: the emit pass skips a target it cannot resolve, so without this guard the hole
        // would validate and then silently not be drilled.
        let mut unknown = double_shear_scene();
        unknown["operations"][0]["instances"][0]["holeEffects"][2]["targetId"] = json!("PL-NOPE");
        assert!(
            validate_scene(&unknown)
                .unwrap_err()
                .to_string()
                .contains("must reference a physical element")
        );

        // …and a bolt does not drill itself: an extra ply may not be one of the array's own components.
        let mut component = double_shear_scene();
        component["operations"][0]["instances"][0]["holeEffects"][2]["targetId"] =
            json!("WASHER-1");
        assert!(
            validate_scene(&component)
                .unwrap_err()
                .to_string()
                .contains("not bolt-array component")
        );
    }

    #[test]
    fn one_ply_may_not_be_holed_twice_by_the_same_bolt() {
        let mut twice = double_shear_scene();
        twice["operations"][0]["instances"][0]["holeEffects"][2]["targetId"] = json!("PL-1");
        assert!(
            validate_scene(&twice)
                .unwrap_err()
                .to_string()
                .contains("must not hole the same ply twice")
        );
    }

    #[test]
    fn extra_plies_do_not_excuse_a_missing_declared_participant() {
        // Three effects, but the MEMBER is not among them. A count test would pass this on arithmetic
        // alone (3 effects ≥ 2 participants) — the bolt would clamp two plates and miss the beam.
        let mut scene = double_shear_scene();
        scene["operations"][0]["instances"][0]["holeEffects"][1]["targetId"] = json!("PL-2");
        scene["operations"][0]["instances"][0]["holeEffects"][2]["targetId"] = json!("PL-2");
        let err = validate_scene(&scene).unwrap_err().to_string();
        assert!(
            err.contains("must not hole the same ply twice"),
            "got: {err}"
        );

        // The same shape without the duplicate: two distinct plates, no member.
        let mut scene = double_shear_scene();
        scene["operations"][0]["instances"][0]["holeEffects"]
            .as_array_mut()
            .unwrap()
            .remove(1);
        let err = validate_scene(&scene).unwrap_err().to_string();
        assert!(
            err.contains("must cover bolt participant `BEAM-1`"),
            "got: {err}"
        );
    }

    #[test]
    fn a_single_hole_effect_is_still_refused() {
        let mut scene = connection_scene();
        scene["operations"][0]["instances"][0]["holeEffects"]
            .as_array_mut()
            .unwrap()
            .truncate(1);
        let err = validate_scene(&scene).unwrap_err().to_string();
        assert!(
            err.contains("at least one effect per bolt participant"),
            "got: {err}"
        );
    }

    #[test]
    fn bolt_children_must_match_their_authored_instance_geometry() {
        let mut displaced = connection_scene();
        displaced["elements"][2]["axis"]["from"] = json!([200, 0, -20]);
        displaced["elements"][2]["axis"]["to"] = json!([200, 0, 20]);
        let err = validate_scene(&displaced).unwrap_err();
        assert!(err.to_string().contains("shank axis and diameter"));

        let mut wrong_diameter = connection_scene();
        wrong_diameter["elements"][2]["diameterMm"] = json!(16);
        let err = validate_scene(&wrong_diameter).unwrap_err();
        assert!(err.to_string().contains("shank axis and diameter"));

        let mut displaced_nut = connection_scene();
        displaced_nut["elements"][4]["center"] = json!([200, 0, -24]);
        let err = validate_scene(&displaced_nut).unwrap_err();
        assert!(err.to_string().contains("centered and aligned"));
    }

    #[test]
    fn grid_axis_optional_extents_must_be_finite_when_present() {
        let mut scene = connection_scene();
        scene["referenceSystems"][0]["axes"][0]["startMm"] = json!("not-a-number");

        let err = validate_scene(&scene).unwrap_err();
        assert!(err.to_string().contains("startMm` must be finite"));
    }

    #[test]
    fn boolean_cut_preflight_accepts_every_physical_target_kind() {
        // A cut may target any independently-materialized product (here a plate, not just a beam).
        let mut scene = connection_scene();
        scene["operations"][2]["targetId"] = json!("PL-1");
        validate_scene(&scene).unwrap();
        let built = build_ifc(&scene);
        let row = built
            .emitted
            .iter()
            .find(|row| row["id"] == "CUT-1")
            .expect("the cylinder cut is materialized as a voiding feature");
        assert_eq!(row["entityType"], "IfcVoidingFeature");
        assert_eq!(row["voids"], "PL-1");
        assert!(built.failed.is_empty());
    }

    #[test]
    fn box_cope_voids_the_target_as_a_filleted_notch() {
        let mut scene = connection_scene();
        // Replace the cylinder cut with an oriented box cope carrying a re-entrant fillet.
        scene["operations"][2] = json!({
            "id": "COPE-1", "kind": "boolean-cut", "targetId": "BEAM-1",
            "tool": { "kind": "box",
                      "frame": { "origin": [50,0,180], "uDir": [1,0,0], "vDir": [0,0,1], "normal": [0,-1,0] },
                      "halfExtents": [60, 20, 110],
                      "reentrant": { "radiusMm": 12.7, "vSign": -1 } }
        });
        validate_scene(&scene).unwrap();
        let built = build_ifc(&scene);
        assert_eq!(built.doc.matches("IFCVOIDINGFEATURE(").count(), 1);
        assert!(built.doc.contains(".NOTCH.)"), "a box cope is a NOTCH");
        assert_eq!(
            built.doc.matches("IFCINDEXEDPOLYCURVE(").count(),
            1,
            "the fillet uses an indexed poly-curve with an arc"
        );
        assert!(built.doc.contains("IFCARCINDEX("));
        let row = built
            .emitted
            .iter()
            .find(|row| row["id"] == "COPE-1")
            .unwrap();
        assert_eq!(row["entityType"], "IfcVoidingFeature");
        assert_eq!(row["voids"], "BEAM-1");
        assert_eq!(row["relationshipType"], "IfcRelVoidsElement");
        assert!(built.failed.is_empty() && built.unsupported.is_empty());

        // A square cope (no fillet) is still voided, but with no fillet arc.
        let mut square = scene.clone();
        square["operations"][2]["tool"]
            .as_object_mut()
            .unwrap()
            .remove("reentrant");
        let built = build_ifc(&square);
        assert_eq!(
            built.doc.matches("IFCINDEXEDPOLYCURVE(").count(),
            0,
            "a square cope has no fillet arc"
        );
        assert!(
            built
                .emitted
                .iter()
                .any(|row| row["id"] == "COPE-1" && row["entityType"] == "IfcVoidingFeature")
        );
    }

    #[test]
    fn weld_realizes_the_connection_with_a_fastener_and_size_property() {
        let built = build_ifc(&connection_scene());
        let row = built
            .emitted
            .iter()
            .find(|row| row["id"] == "WELD-1")
            .expect("the weld is materialized");
        assert_eq!(row["entityType"], "IfcFastener");
        assert_eq!(
            row["relationshipType"],
            "IfcRelConnectsWithRealizingElements"
        );
        assert_eq!(row["realizes"], json!(["BEAM-1", "PL-1"]));
        assert!(built.doc.contains(".WELD.)"));
        assert!(built.doc.contains("'Pset_AWARE_Weld'") && built.doc.contains("IFCBOOLEAN(.T.)"));
    }

    #[test]
    fn unsupported_element_kinds_are_exhaustively_reported() {
        let mut scene = connection_scene();
        scene["elements"].as_array_mut().unwrap().push(json!({
            "id": "FUTURE-1", "kind": "future-native-part", "rot": 82.7
        }));
        scene["referenceSystems"]
            .as_array_mut()
            .unwrap()
            .push(json!({
                "id": "FUTURE-GRID", "kind": "future-reference",
                "axes": [{"id":"FUTURE-AXIS"}], "levels": [{"id":"FUTURE-LEVEL"}]
            }));
        validate_scene(&scene).unwrap();
        let built = build_ifc(&scene);
        let row = built
            .unsupported
            .iter()
            .find(|row| row["id"] == "FUTURE-1")
            .expect("unsupported record must be receipted");
        assert_eq!(row["code"], "unsupported-element-kind");
        for id in ["FUTURE-GRID", "FUTURE-AXIS", "FUTURE-LEVEL"] {
            assert!(built.unsupported.iter().any(|row| row["id"] == id));
        }
        assert_eq!(
            built
                .unsupported
                .iter()
                .find(|row| row["id"] == "FUTURE-AXIS")
                .unwrap()["code"],
            "unsupported-parent"
        );
    }
}
