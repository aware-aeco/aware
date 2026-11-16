//! `viewer-3d` for `kind:"extrusion"` (#3296 S3): drawn, receipted, and refused with exactly the
//! rule `ifc.write` names, from the one shared parser.

use serde_json::{Value, json};

use super::viewer_3d_render;
use crate::render::ifc::ifc_write;

fn scene(elements: Value) -> Value {
    json!({ "meta": { "name": "elements", "units": "mm", "up": "z" }, "elements": elements })
}

fn wall() -> Value {
    json!({ "id": "W-1", "kind": "extrusion", "class": "wall",
        "frame": {"origin":[1000,2000,0],"uDir":[1,0,0],"vDir":[0,1,0],"normal":[0,0,1]},
        "items": [{"outline":[[0,-100],[6000,-100],[6000,100],[0,100]],"fromMm":0,"depthMm":3000}],
        "axis": {"u0":0,"u1":6000} })
}

fn slab() -> Value {
    json!({ "id": "S-1", "kind": "extrusion", "class": "slab",
        "frame": {"origin":[0,0,2850],"uDir":[1,0,0],"vDir":[0,1,0],"normal":[0,0,1]},
        "items": [{"outline":[[0,0],[8000,0],[8000,6000],[0,6000]],
                   "holes":[[[1000,1000],[1000,2000],[2000,2000],[2000,1000]],
                            [[5000,3000],[5000,4000],[6000,4000],[6000,3000]]],
                   "fromMm":0,"depthMm":150}] })
}

fn footing() -> Value {
    json!({ "id": "F-1", "kind": "extrusion", "class": "footing", "predefinedType": "PAD_FOOTING",
        "frame": {"origin":[0,0,-600],"uDir":[1,0,0],"vDir":[0,1,0],"normal":[0,0,1]},
        "items": [{"outline":[[0,0],[2000,0],[2000,2000],[0,2000]],"fromMm":0,"depthMm":600}] })
}

#[test]
fn an_extrusion_is_drawn_and_receipted_as_one_object() {
    let out = viewer_3d_render(
        &json!({ "scene": scene(json!([wall(), slab(), footing()])) }),
        true,
    )
    .unwrap();
    for id in ["W-1", "S-1", "F-1"] {
        let row = out["emitted"]
            .as_array()
            .unwrap()
            .iter()
            .find(|row| row["id"] == id)
            .unwrap_or_else(|| panic!("{id} not emitted: {out}"));
        assert_eq!(row["kind"], "extrusion");
        assert_eq!(row["renderedKind"], "extrusion");
    }
    assert!(out["unsupported"].as_array().unwrap().is_empty());
    let html = out["html"].as_str().unwrap();
    assert!(html.contains("else if(kind==='extrusion') mesh=extrusionMesh(e,up,mat);"));
    assert!(html.contains("function extrusionMesh(e,up,mat)"));
    // Built in the frame's own coordinates from the first item's start plane; the frame (a reflection
    // under the z-up conversion) is the mesh matrix, never baked into float32 vertices.
    assert!(html.contains("mesh.matrix.makeBasis(U,V,N).setPosition(O);"));
    assert!(html.contains("pos.push(P.getX(i),P.getY(i),P.getZ(i)+dz)"));
    // A thin band keeps its true depth: no display floor is applied to an extrusion item.
    assert!(html.contains("{depth:it.depthMm,bevelEnabled:false,curveSegments:1}"));
}

#[test]
fn an_invalid_extrusion_is_refused_with_the_same_rule_ifc_write_names() {
    let mut bad_frame = wall();
    bad_frame["frame"]["normal"] = json!([0, 0, -1]);
    let mut bad_depth = slab();
    bad_depth["items"][0]["depthMm"] = json!(-1);
    let mut bad_hole = slab();
    bad_hole["items"][0]["holes"][0] = json!([[0, 1000], [0, 2000], [1000, 2000], [1000, 1000]]);
    let mut bad_type = footing();
    bad_type["predefinedType"] = json!("FLOOR");
    let mut too_many = wall();
    let item = too_many["items"][0].clone();
    too_many["items"] = json!((0..9).map(|_| item.clone()).collect::<Vec<_>>());
    // The IFC Name (meta.label, else name, else id) must fit an IfcLabel's 255 bytes.
    let mut long_id = slab();
    long_id["id"] = json!("S".repeat(256));
    let mut long_label = footing();
    long_label["meta"] = json!({ "label": "F".repeat(300) });
    let mut blank_material = wall();
    blank_material["material"] = json!({ "layers": [{ "name": "  ", "thicknessMm": 200 }] });
    let mut two_layers = slab();
    two_layers["material"] = json!({ "layers": [
        { "name": "Screed", "thicknessMm": 50 }, { "name": "Concrete", "thicknessMm": 100 }] });
    for element in [
        bad_frame,
        bad_depth,
        bad_hole,
        bad_type,
        too_many,
        long_id,
        long_label,
        blank_material,
        two_layers,
    ] {
        let scene = scene(json!([element]));
        let viewer = viewer_3d_render(&json!({ "scene": scene }), true)
            .unwrap_err()
            .to_string();
        let ifc = ifc_write(&json!({ "scene": scene }), true)
            .unwrap_err()
            .to_string();
        let viewer_reason = viewer
            .split_once("`elements[0]` ")
            .unwrap_or_else(|| panic!("{viewer}"))
            .1;
        let ifc_reason = ifc.split_once("): ").unwrap_or_else(|| panic!("{ifc}")).1;
        assert_eq!(viewer_reason, ifc_reason, "{viewer} / {ifc}");
    }
}

#[test]
fn malformed_extrusion_labels_are_refused_by_both_sinks_with_the_same_rule() {
    let mut bad_description = wall();
    bad_description["meta"] = json!({ "description": 123 });
    let mut reserved_name = slab();
    reserved_name["meta"] =
        json!({ "properties": { "name": "Pset_Mine", "properties": { "A": "v" } } });
    let mut unknown_field = footing();
    unknown_field["meta"] =
        json!({ "properties": { "name": "FloLess", "properties": { "A": "v" }, "x": 1 } });
    for element in [bad_description, reserved_name, unknown_field] {
        let scene = scene(json!([element]));
        let viewer = viewer_3d_render(&json!({ "scene": scene }), true)
            .unwrap_err()
            .to_string();
        let ifc = ifc_write(&json!({ "scene": scene }), true)
            .unwrap_err()
            .to_string();
        let rule = |message: &str| {
            let at = message
                .find("`meta.")
                .unwrap_or_else(|| panic!("no label rule in {message}"));
            message[at..].to_string()
        };
        assert_eq!(rule(&viewer), rule(&ifc), "{viewer} / {ifc}");
        // ...and partial mode leaves just that element out, keeping a valid one.
        let out = viewer_3d_render(
            &json!({ "scene": { "meta": scene["meta"], "elements": [scene["elements"][0], wall_with_id("W-OK")] },
                     "partial": true }),
            true,
        )
        .unwrap();
        assert_eq!(out["failed"][0]["id"], scene["elements"][0]["id"], "{out}");
        assert_eq!(out["emitted"][0]["id"], "W-OK", "{out}");
    }
    // Valid labels pass both.
    let mut labelled = wall();
    labelled["meta"] = json!({ "description": "Wall W-1",
        "properties": { "name": "FloLess", "properties": { "AddOn": "walls" } } });
    assert!(viewer_3d_render(&json!({ "scene": scene(json!([labelled])) }), true).is_ok());
}

fn wall_with_id(id: &str) -> Value {
    let mut element = wall();
    element["id"] = json!(id);
    element
}

#[test]
fn an_extrusion_material_name_picks_its_realistic_finish() {
    // `material` on an extrusion is `{name}`; the finish lookup reads its name rather than the
    // object's string form (which would always fall back to painted).
    let out = viewer_3d_render(&json!({ "scene": scene(json!([wall()])) }), true).unwrap();
    let html = out["html"].as_str().unwrap();
    assert!(html.contains(
        "const pick=v=>(v==null?'':String(typeof v==='object'?(((v.layers||[])[0]||{}).name??''):v)).trim();"
    ));
}

#[test]
fn a_band_thinner_than_the_display_resolution_is_drawn_and_warned_about() {
    // One item: exact at any accepted depth, so no warning even at the minimum depth far from origin.
    let mut thin = wall();
    thin["frame"]["origin"] = json!([500000, 500000, 900000]);
    thin["items"][0]["depthMm"] = json!(0.00001);
    let out = viewer_3d_render(&json!({ "scene": scene(json!([thin])) }), true).unwrap();
    assert_eq!(out["warnings"], json!([]), "{out}");
    // A second band 3000 mm up, 0.00001 mm deep: float32 cannot hold it there, so the receipt says so.
    let mut banded = wall();
    let band = json!({"outline":[[0,-100],[6000,-100],[6000,100],[0,100]],"fromMm":3000,"depthMm":0.00001});
    banded["items"].as_array_mut().unwrap().push(band);
    let out = viewer_3d_render(&json!({ "scene": scene(json!([banded])) }), true).unwrap();
    let warnings = out["warnings"].as_array().unwrap();
    assert_eq!(warnings.len(), 1, "{out}");
    assert_eq!(warnings[0]["code"], "extrusion-below-display-resolution");
    assert_eq!(warnings[0]["id"], "W-1");
    assert!(
        warnings[0]["message"]
            .as_str()
            .unwrap()
            .contains("items[1]")
    );
    assert_eq!(out["emitted"][0]["id"], "W-1", "still drawn");
    // A 100 mm band at the same height is well within float32 and is not warned about.
    banded["items"][1]["depthMm"] = json!(100);
    let out = viewer_3d_render(&json!({ "scene": scene(json!([banded])) }), true).unwrap();
    assert_eq!(out["warnings"], json!([]));
}

#[test]
fn a_ring_far_from_its_frame_origin_is_drawn_from_its_own_corner() {
    // A 0.001 mm wide ring 900 m along u: measured from the outlines' lowest corner it is exact, so
    // no warning, and the page rebases the rings on that same corner.
    let mut far = wall();
    far["items"][0]["outline"] = json!([
        [900000, 0],
        [900000.001, 0],
        [900000.001, 1000],
        [900000, 1000]
    ]);
    far["items"][0]["depthMm"] = json!(100);
    far.as_object_mut().unwrap().remove("axis");
    let out = viewer_3d_render(&json!({ "scene": scene(json!([far])) }), true).unwrap();
    assert_eq!(out["warnings"], json!([]), "{out}");
    let html = out["html"].as_str().unwrap();
    assert!(html.contains("const x=pts[i][0]-a0, y=pts[i][1]-b0;"));
    // A second item with a 0.001 mm edge 900 m from the first item's corner is past float32: warned.
    let mut split = wall();
    split.as_object_mut().unwrap().remove("axis");
    split["items"].as_array_mut().unwrap().push(json!({
        "outline": [[900000, 0], [900000.001, 0], [900000.001, 1000], [900000, 1000]],
        "fromMm": 0, "depthMm": 100 }));
    let out = viewer_3d_render(&json!({ "scene": scene(json!([split])) }), true).unwrap();
    let warnings = out["warnings"].as_array().unwrap();
    assert_eq!(warnings.len(), 1, "{out}");
    assert!(
        warnings[0]["message"]
            .as_str()
            .unwrap()
            .contains("items[1] has a"),
        "{out}"
    );
    // A sliver with long edges (area 4500 mm², 900 m long) is too narrow for float32 even from its
    // own corner: it is drawn and warned about, although no edge is short.
    let mut sliver = wall();
    sliver.as_object_mut().unwrap().remove("axis");
    sliver["items"][0]["outline"] = json!([[0, 0], [900000, 900000], [450000, 450000.01]]);
    sliver["items"][0]["depthMm"] = json!(100);
    let out = viewer_3d_render(&json!({ "scene": scene(json!([sliver])) }), true).unwrap();
    let warnings = out["warnings"].as_array().unwrap();
    assert_eq!(warnings.len(), 1, "{out}");
    let message = warnings[0]["message"].as_str().unwrap();
    assert!(message.contains("items[0] has a ring"), "{out}");
    // A hole that hugs its outline leaves a 0.001 mm strip of material 900 m long. Every ring on
    // its own is wide, so only the rings checked together show that float32 cannot hold the strip.
    let mut strip = wall();
    strip.as_object_mut().unwrap().remove("axis");
    strip["items"][0]["outline"] = json!([[0, 0], [900000, 0], [900000, 900000], [0, 900000]]);
    strip["items"][0]["holes"] = json!([[
        [0.001, 0.001],
        [0.001, 899999.999],
        [899999.999, 899999.999],
        [899999.999, 0.001]
    ]]);
    strip["items"][0]["depthMm"] = json!(100);
    let out = viewer_3d_render(&json!({ "scene": scene(json!([strip])) }), true).unwrap();
    let warnings = out["warnings"].as_array().unwrap();
    assert_eq!(warnings.len(), 1, "{out}");
    let message = warnings[0]["message"].as_str().unwrap();
    assert!(message.contains("items[0] has a ring"), "{out}");
    assert!(
        out["emitted"]
            .as_array()
            .unwrap()
            .iter()
            .any(|r| r["id"] == "W-1"),
        "{out}"
    );
}

#[test]
fn partial_mode_leaves_out_only_the_invalid_extrusion() {
    let mut bad = slab();
    bad["items"][0]["depthMm"] = json!(0);
    let out = viewer_3d_render(
        &json!({ "scene": scene(json!([wall(), bad])), "partial": true }),
        true,
    )
    .unwrap();
    let ids = |list: &str| -> Vec<Value> {
        out[list]
            .as_array()
            .unwrap()
            .iter()
            .map(|row| row["id"].clone())
            .collect()
    };
    assert!(ids("emitted").contains(&json!("W-1")));
    assert!(!ids("emitted").contains(&json!("S-1")));
    assert!(ids("failed").contains(&json!("S-1")), "{out}");
}
