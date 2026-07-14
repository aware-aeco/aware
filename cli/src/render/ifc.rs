//! `ifc.write` — write a generic 3D **scene** to a universal **IFC4** file (builtin transport).
//!
//! The file-writing sibling of `viewer-3d.render`: it consumes the SAME domain-agnostic scene
//! (members as `from`->`to` axes with an optional parametric cross-section + a `group`) and emits an
//! IFC4 STEP (SPF) document — IfcColumn/IfcBeam/IfcMember as extruded profiles placed on the member
//! axis, under an IfcProject -> IfcSite -> IfcBuilding -> IfcBuildingStorey spine. Each element may
//! carry an `xsection` (i/channel/angle/rhs/chs/rect) → the matching parametric IfcProfileDef; a
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
use serde_json::Value;
use std::collections::BTreeMap;
use std::fmt::Write as _;

/// Base-64-ish charset for IFC GlobalIds (valid IFC GUID alphabet).
const B64: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz_$";

/// A member axis whose horizontal component is within this (squared) tolerance is treated as vertical.
const VERTICAL_EPSILON_SQ: f64 = 1e-6;

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

/// 3-vector cross product.
fn cross(a: (f64, f64, f64), b: (f64, f64, f64)) -> (f64, f64, f64) {
    (
        a.1 * b.2 - a.2 * b.1,
        a.2 * b.0 - a.0 * b.2,
        a.0 * b.1 - a.1 * b.0,
    )
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

/// Emit the parametric profile for a member's `xsection`, positioned at `pos2d`. Falls back to an
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
    let g = guid(spf.id);
    Some(spf.emit(&format!(
        "IFCBUILDINGELEMENTPROXY({},$,{},$,$,#{place},#{pds},$,$)",
        s_lit(&g),
        s_lit(name)
    )))
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
}

/// Build the full IFC4 SPF document from a scene object. Pure (no IO) so it is directly unit-testable.
fn build_ifc(scene: &Value) -> BuildResult {
    let proj_name = scene
        .get("meta")
        .and_then(Value::as_object)
        .and_then(|m| m.get("name"))
        .and_then(Value::as_str)
        .unwrap_or("Model");

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

    for el in elements {
        let elid = el
            .get("id")
            .and_then(Value::as_str)
            .unwrap_or("")
            .to_string();
        let style =
            resolve_color(el, &group_colors).and_then(|(hk, _)| color_style.get(&hk).copied());
        let matkey = resolve_material(el).map(|(mk, _)| mk);

        // A tessellated mesh element (e.g. an imported connection).
        let is_mesh =
            el.get("kind").and_then(Value::as_str) == Some("mesh") || el.get("positions").is_some();
        if is_mesh {
            if let Some(elem) = emit_mesh(&mut spf, el, bldg_place, ctx, style) {
                elem_ids.push(elem);
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
        let (zx, zy, zz) = (dx / len, dy / len, dz / len);
        // Local frame: Z = member axis; profile Y (= Z × X) = world-up, so an I-shape web is vertical
        // for beams and columns take a fixed +Y-depth orientation (matches the FloLess 3D viewer).
        let (xx, xy, xz) = if zx * zx + zy * zy <= VERTICAL_EPSILON_SQ {
            // Near-vertical: seed local X = world +X (Y = Z × X is right-handed for both +Z and −Z).
            (1.0, 0.0, 0.0)
        } else {
            // General: local Y = normalize(world-up projected onto the ⟂-Z plane); local X = Y × Z.
            let dot = zz; // up·z
            let (mut yx, mut yy, mut yz) = (-dot * zx, -dot * zy, 1.0 - dot * zz);
            let yl = (yx * yx + yy * yy + yz * yz).sqrt();
            yx /= yl;
            yy /= yl;
            yz /= yl;
            cross((yx, yy, yz), (zx, zy, zz)) // x = y × z
        };

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
        let rot = el.get("rot").and_then(Value::as_f64).unwrap_or(0.0);
        let prof_pos = if rot.abs() > 1e-9 {
            let th = rot.to_radians();
            let rd = spf.emit(&format!("IFCDIRECTION(({},{}))", r(th.cos()), r(th.sin())));
            spf.emit(&format!("IFCAXIS2PLACEMENT2D(#{origin2d},#{rd})"))
        } else {
            pos2d
        };

        let (prof, warn) = emit_profile(&mut spf, el.get("xsection"), w, d, &profile, prof_pos);
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
        let g = guid(spf.id);
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
        if let Some(mk) = matkey {
            mat_members.entry(mk).or_default().push(elem);
        }
    }

    // ── Phase 4: spatial containment ──
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

    BuildResult {
        doc,
        members: elem_ids.len(),
        columns,
        beams,
        meshes,
        profiles: profile_counts,
        warnings,
    }
}

/// `ifc.write` — write a generic 3D scene to an IFC4 file. Mirrors `viewer-3d.render`'s contract:
/// `{ path?, bytes, members, columns, beams, meshes, profiles, warnings }`, with the `output-path`
/// write gated to a real run.
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

    let built = build_ifc(scene);

    let mut out = serde_json::Map::new();
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

/// JSON type name for clear validation errors.
fn json_type(v: &Value) -> &'static str {
    match v {
        Value::Null => "null",
        Value::Bool(_) => "boolean",
        Value::Number(_) => "number",
        Value::String(_) => "string",
        Value::Array(_) => "array",
        Value::Object(_) => "object",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

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
}
