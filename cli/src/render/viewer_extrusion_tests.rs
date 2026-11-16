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
    // The reflective z-up conversion is handled by turning triangles round, never by dropping the normal.
    assert!(html.contains("const flip=U.clone().cross(V).dot(N)<0"));
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
    for element in [bad_frame, bad_depth, bad_hole, bad_type, too_many] {
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
