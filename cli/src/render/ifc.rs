//! `ifc.write` — write a generic 3D **scene** to a universal **IFC4** file (builtin transport).
//!
//! The file-writing sibling of `viewer-3d.render`: it consumes the SAME domain-agnostic scene
//! (members as `from`->`to` boxes with an optional cross-section + a `group`) and emits an IFC4
//! STEP (SPF) document — IfcColumn/IfcBeam/IfcMember as extruded rectangular sections placed on
//! the member axis, under an IfcProject -> IfcSite -> IfcBuilding -> IfcBuildingStorey spine.
//! A `kind:"mesh"` element (tessellated `positions`+`indices`, e.g. an imported connection) is
//! written as an IfcTriangulatedFaceSet on an IfcBuildingElementProxy — so free-form geometry
//! that has no parametric section still round-trips into the exported model.
//! Host-free (no Tekla/Revit): pure serialization + an optional file write, so any composition
//! that produces a scene can export a universal model to open in Tekla, SDS2, Revit or Navisworks.
//!
//! Output mirrors `viewer-3d.render`: `{ path?, bytes, members, columns, beams }`, with the
//! `output-path` write gated to a real run (skipped under --dry-run / --simulate). The producer
//! owns domain meaning AND the output path; the writer stays generic. (Companion to `viewer-3d`,
//! which renders the same scene to interactive HTML — one scene, two outputs.)
//!
//! Determinism: identical `scene` input -> identical IFC bytes. GlobalIds come from an entity
//! counter (no randomness), reals use a fixed invariant format, and the SPF stamp is fixed — no
//! clock, no environment. Proven against the floless `steel-to-ifc` reference (ifcopenshell loads
//! it: columns vertical, beams horizontal).

use crate::error::AwareError;
use serde_json::Value;
use std::collections::BTreeMap;
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

/// A deterministic, scene-derived ascii filename for the SPF `FILE_NAME` metadata field (NOT the
/// on-disk path — the caller owns that via `output-path`). Drops a trailing "(...)" qualifier,
/// then lowercases ascii alphanumerics joining the rest with single hyphens. Mirrors the proven
/// reference so the same scene name yields the same in-file metadata.
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

/// Emit a tessellated `kind:"mesh"` element as an IfcTriangulatedFaceSet on an
/// IfcBuildingElementProxy, placed at `place` (an identity local placement, so the mesh's
/// `positions` are absolute world mm). Returns the proxy's entity id, or `None` when
/// `positions`/`indices` are missing or malformed (flat xyz triples + whole triangles, every
/// index in range). IFC coordinate indices are **1-based**, so each 0-based input index is +1'd.
fn emit_mesh(spf: &mut Spf, el: &Value, place: i64, ctx: i64) -> Option<i64> {
    let pos = el.get("positions").and_then(Value::as_array)?;
    let idx = el.get("indices").and_then(Value::as_array)?;
    let npts = pos.len() / 3;
    if npts < 3 || pos.len() % 3 != 0 || idx.len() < 3 || idx.len() % 3 != 0 {
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
            r(num(pos.get(i * 3))),
            r(num(pos.get(i * 3 + 1))),
            r(num(pos.get(i * 3 + 2)))
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
    // IfcTriangulatedFaceSet(Coordinates, Normals, Closed, CoordIndex, PnIndex).
    let tfs = spf.emit(&format!("IFCTRIANGULATEDFACESET(#{plist},$,$,({tris}),$)"));
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

/// Build the full IFC4 SPF document from a scene object. Returns `(ifc_text, members, columns,
/// beams, meshes, profiles)`. Pure (no IO) so it is directly unit-testable. `members` is the count
/// of placed elements (mesh proxies included); `columns`/`beams` are per-group sub-counts (braces
/// count toward neither), `meshes` counts tessellated `kind:"mesh"` elements; `profiles` is a
/// profile-string -> count breakdown for the caller's report.
fn build_ifc(scene: &Value) -> (String, usize, i64, i64, i64, BTreeMap<String, i64>) {
    let proj_name = scene
        .get("meta")
        .and_then(Value::as_object)
        .and_then(|m| m.get("name"))
        .and_then(Value::as_str)
        .unwrap_or("Model");

    let mut spf = Spf::new();

    // ── shared geometry primitives + the spatial spine ──
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

    // ── one IFC element per scene element ──
    let mut elem_ids: Vec<i64> = Vec::new();
    let mut columns = 0i64;
    let mut beams = 0i64;
    let mut meshes = 0i64;
    let mut profile_counts: BTreeMap<String, i64> = BTreeMap::new();

    if let Some(elements) = scene.get("elements").and_then(Value::as_array) {
        for el in elements {
            // A tessellated mesh element (e.g. an imported connection): `positions`+`indices`, no
            // parametric section. Emit it as an IfcTriangulatedFaceSet proxy and move on — it never
            // falls through to the extruded-box path below.
            let is_mesh = el.get("kind").and_then(Value::as_str) == Some("mesh")
                || el.get("positions").is_some();
            if is_mesh {
                if let Some(elem) = emit_mesh(&mut spf, el, bldg_place, ctx) {
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
            // a stable perpendicular (local X): cross(ref, Z), ref = global Z unless ~parallel.
            let (rx, ry, rz) = if zz.abs() > 0.9 {
                (1.0, 0.0, 0.0)
            } else {
                (0.0, 0.0, 1.0)
            };
            let (mut xx, mut xy, mut xz) =
                (ry * zz - rz * zy, rz * zx - rx * zz, rx * zy - ry * zx);
            let xl = (xx * xx + xy * xy + xz * xz).sqrt();
            if xl < 1e-9 {
                xx = 1.0;
                xy = 0.0;
                xz = 0.0;
            } else {
                xx /= xl;
                xy /= xl;
                xz /= xl;
            }

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
            let prof = spf.emit(&format!(
                "IFCRECTANGLEPROFILEDEF(.AREA.,{},#{pos2d},{},{})",
                s_lit(&profile),
                r(w),
                r(d)
            ));
            let solid = spf.emit(&format!(
                "IFCEXTRUDEDAREASOLID(#{prof},#{extrude_pos},#{dir_z},{})",
                r(len)
            ));
            let shape = spf.emit(&format!(
                "IFCSHAPEREPRESENTATION(#{ctx},'Body','SweptSolid',(#{solid}))"
            ));
            let pds = spf.emit(&format!("IFCPRODUCTDEFINITIONSHAPE($,$,(#{shape}))"));
            let g = guid(spf.id);
            let (ifc_type, pdt) = if group == "column" {
                columns += 1;
                ("IFCCOLUMN", ".COLUMN.")
            } else if group == "brace" {
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

    // ── assemble the SPF document ──
    let fname = file_name_meta(proj_name);
    let mut doc = String::new();
    doc.push_str("ISO-10303-21;\n");
    doc.push_str("HEADER;\n");
    doc.push_str("FILE_DESCRIPTION(('ViewDefinition [ReferenceView_V1.2]'),'2;1');\n");
    // Fixed stamp: deterministic output (no clock in determinism-sensitive content).
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

    (doc, elem_ids.len(), columns, beams, meshes, profile_counts)
}

/// `ifc.write` — write a generic 3D scene to an IFC4 file. Mirrors `viewer-3d.render`'s contract:
/// `{ path?, bytes, members, columns, beams, profiles }`, with the `output-path` write gated to a
/// real run.
pub fn ifc_write(args: &Value, dry_run: bool) -> Result<Value, AwareError> {
    // The scene is the payload; require an object so there is something to serialize.
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

    let (ifc, members, columns, beams, meshes, profiles) = build_ifc(scene);

    let mut out = serde_json::Map::new();
    out.insert("bytes".into(), Value::from(ifc.len() as u64));
    out.insert("members".into(), Value::from(members as u64));
    out.insert("columns".into(), Value::from(columns));
    out.insert("beams".into(), Value::from(beams));
    out.insert("meshes".into(), Value::from(meshes));
    out.insert(
        "profiles".into(),
        Value::Object(
            profiles
                .into_iter()
                .map(|(k, v)| (k, Value::from(v)))
                .collect(),
        ),
    );

    if let Some(path) = args
        .get("output-path")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
    {
        // Real run only: a preview (--dry-run / --simulate) returns the counts + would-be path
        // but never touches disk (same contract as viewer-3d / html-report).
        if !dry_run {
            if let Some(parent) = std::path::Path::new(path).parent()
                && !parent.as_os_str().is_empty()
            {
                std::fs::create_dir_all(parent).map_err(|e| {
                    AwareError::Internal(format!("ifc: create {}: {e}", parent.display()))
                })?;
            }
            std::fs::write(path, ifc.as_bytes())
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
        let (ifc, members, columns, beams, meshes, profiles) = build_ifc(&sample_scene());
        assert_eq!(members, 2);
        assert_eq!(columns, 1);
        assert_eq!(beams, 1);
        assert_eq!(meshes, 0);
        assert_eq!(profiles.get("UC305x305x97"), Some(&1));
        assert_eq!(profiles.get("W10x33"), Some(&1));
        assert!(ifc.starts_with("ISO-10303-21;"));
        assert!(ifc.contains("FILE_SCHEMA(('IFC4'));"));
        assert_eq!(ifc.matches("IFCCOLUMN(").count(), 1);
        assert_eq!(ifc.matches("IFCBEAM(").count(), 1);
        assert_eq!(ifc.matches("IFCRELCONTAINEDINSPATIALSTRUCTURE(").count(), 1);
        assert!(ifc.trim_end().ends_with("END-ISO-10303-21;"));
    }

    #[test]
    fn is_deterministic() {
        let (a, ..) = build_ifc(&sample_scene());
        let (b, ..) = build_ifc(&sample_scene());
        assert_eq!(a, b, "identical scene must yield identical IFC bytes");
    }

    #[test]
    fn dry_run_returns_counts_and_writes_nothing() {
        let out = ifc_write(&json!({ "scene": sample_scene() }), true).unwrap();
        assert_eq!(out["members"].as_u64().unwrap(), 2);
        assert_eq!(out["columns"].as_u64().unwrap(), 1);
        assert_eq!(out["beams"].as_u64().unwrap(), 1);
        assert!(out["bytes"].as_u64().unwrap() > 0);
        assert!(out.get("path").is_none()); // no output-path given -> nothing written
    }

    #[test]
    fn brace_counts_toward_neither_column_nor_beam() {
        let scene = json!({
            "meta": { "name": "x" },
            "elements": [
                { "id": "BR1", "group": "brace", "from": [0,0,0], "to": [3000,0,3000],
                  "section": { "w": 90, "d": 90 }, "meta": { "profile": "L90" } }
            ]
        });
        let (ifc, members, columns, beams, _meshes, profiles) = build_ifc(&scene);
        assert_eq!(members, 1);
        assert_eq!(columns, 0);
        assert_eq!(beams, 0);
        assert_eq!(ifc.matches("IFCMEMBER(").count(), 1);
        assert_eq!(profiles.get("L90"), Some(&1));
    }

    #[test]
    fn degenerate_and_malformed_elements_are_skipped() {
        let scene = json!({
            "meta": { "name": "x" },
            "elements": [
                { "id": "ZERO", "group": "beam", "from": [0,0,0], "to": [0,0,0] }, // zero length
                { "id": "SHORT", "group": "beam", "from": [0,0], "to": [1,1] },      // <3 coords
                { "id": "OK", "group": "beam", "from": [0,0,0], "to": [1000,0,0] }
            ]
        });
        let (_ifc, members, _c, beams, _m, _profiles) = build_ifc(&scene);
        assert_eq!(members, 1);
        assert_eq!(beams, 1);
    }

    #[test]
    fn writes_a_triangulated_mesh_element() {
        // A single tetra-ish mesh: 4 points, 2 triangles (0-based indices become 1-based in IFC).
        let scene = json!({
            "meta": { "name": "conn" },
            "elements": [
                { "id": "PL-1", "kind": "mesh", "group": "connection",
                  "positions": [0.0,0.0,0.0, 100.0,0.0,0.0, 100.0,100.0,0.0, 0.0,100.0,0.0],
                  "indices": [0,1,2, 0,2,3],
                  "meta": { "profile": "PLATE" } }
            ]
        });
        let (ifc, members, columns, beams, meshes, profiles) = build_ifc(&scene);
        assert_eq!(members, 1); // the proxy is a placed element
        assert_eq!(columns, 0);
        assert_eq!(beams, 0);
        assert_eq!(meshes, 1);
        assert_eq!(profiles.get("PLATE"), Some(&1));
        assert_eq!(ifc.matches("IFCTRIANGULATEDFACESET(").count(), 1);
        assert_eq!(ifc.matches("IFCCARTESIANPOINTLIST3D(").count(), 1);
        assert_eq!(ifc.matches("IFCBUILDINGELEMENTPROXY(").count(), 1);
        // CoordIndex is 1-based: input triangle (0,1,2) -> (1,2,3), (0,2,3) -> (1,3,4).
        assert!(ifc.contains("((1,2,3),(1,3,4))"));
        assert!(ifc.contains("'Tessellation'"));
    }

    #[test]
    fn malformed_mesh_is_skipped_not_emitted() {
        let scene = json!({
            "meta": { "name": "x" },
            "elements": [
                { "id": "BAD-COUNT", "kind": "mesh", "positions": [0,0,0, 1,0,0], "indices": [0,1,2] }, // <3 pts
                { "id": "OOR", "kind": "mesh",
                  "positions": [0,0,0, 1,0,0, 1,1,0], "indices": [0,1,9] } // index 9 out of range
            ]
        });
        let (ifc, members, _c, _b, meshes, _p) = build_ifc(&scene);
        assert_eq!(members, 0);
        assert_eq!(meshes, 0);
        assert_eq!(ifc.matches("IFCTRIANGULATEDFACESET(").count(), 0);
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
        assert_eq!(s_lit("a'b"), "'a''b'"); // apostrophe doubled
        assert_eq!(s_lit("a\\b"), "'a\\\\b'"); // backslash (Part 21 escape char) doubled
        // non-ASCII (em-dash U+2014) → a \X2\<utf-16>\X0\ block
        assert_eq!(s_lit("a\u{2014}b"), "'a\\X2\\2014\\X0\\b'");
    }
}
