//! `ifc.write` for `kind:"extrusion"` (walls, slabs, footings; #3296 S3). Every check here reads the
//! written STEP back: the solids are reconstructed from their polylines and depths and their volume
//! is compared with the scene's `quantities`, so a test passes on what the file says, never on the
//! entity names alone.

use std::collections::BTreeMap;

use serde_json::{Value, json};

use super::{BuildResult, s_lit, try_build_ifc, validate_scene};

/// One parsed `#id=ENTITY(args);` line.
struct Entity {
    name: String,
    args: Vec<String>,
}

/// Split a STEP argument list at its top-level commas (quotes and nested parentheses respected).
fn split_args(text: &str) -> Vec<String> {
    let (mut out, mut depth, mut quoted, mut start) = (Vec::new(), 0i32, false, 0usize);
    let bytes = text.as_bytes();
    for (index, &byte) in bytes.iter().enumerate() {
        match byte {
            b'\'' => quoted = !quoted,
            b'(' if !quoted => depth += 1,
            b')' if !quoted => depth -= 1,
            b',' if !quoted && depth == 0 => {
                out.push(text[start..index].to_string());
                start = index + 1;
            }
            _ => {}
        }
    }
    out.push(text[start..].to_string());
    out
}

fn index(doc: &str) -> BTreeMap<i64, Entity> {
    doc.lines()
        .filter_map(|line| {
            let rest = line.strip_prefix('#')?;
            let (id, body) = rest.split_once('=')?;
            let open = body.find('(')?;
            let inner = body[open + 1..].strip_suffix(");")?;
            Some((
                id.parse().ok()?,
                Entity {
                    name: body[..open].to_string(),
                    args: split_args(inner),
                },
            ))
        })
        .collect()
}

fn reference(arg: &str) -> i64 {
    arg.trim_start_matches('#')
        .parse()
        .unwrap_or_else(|_| panic!("not a reference: {arg}"))
}

fn references(arg: &str) -> Vec<i64> {
    let inner = arg
        .strip_prefix('(')
        .and_then(|a| a.strip_suffix(')'))
        .unwrap_or_else(|| panic!("not a list: {arg}"));
    split_args(inner).iter().map(|a| reference(a)).collect()
}

fn numbers(arg: &str) -> Vec<f64> {
    let inner = arg.trim_start_matches('(').trim_end_matches(')');
    inner.split(',').map(|n| n.parse().unwrap()).collect()
}

fn string(arg: &str) -> Option<String> {
    (arg != "$").then(|| arg.trim_matches('\'').replace("''", "'"))
}

fn polyline_area(entities: &BTreeMap<i64, Entity>, id: i64) -> f64 {
    let line = &entities[&id];
    assert_eq!(line.name, "IFCPOLYLINE");
    let points: Vec<Vec<f64>> = references(&line.args[0])
        .iter()
        .map(|p| numbers(&entities[p].args[0]))
        .collect();
    assert_eq!(
        points.first(),
        points.last(),
        "a profile polyline is closed"
    );
    points
        .windows(2)
        .map(|w| w[0][0] * w[1][1] - w[1][0] * w[0][1])
        .sum::<f64>()
        .abs()
        / 2.0
}

/// What the file says about one wall, slab, footing or proxy.
#[derive(Debug, Default)]
struct ReadBack {
    entity: String,
    description: Option<String>,
    object_type: Option<String>,
    predefined: String,
    /// Reconstructed from the Body: Σ (outer − holes area) × depth, in m³.
    volume_m3: f64,
    solids: usize,
    holes: usize,
    /// The placement: origin, Z axis, X axis.
    placement: (Vec<f64>, Vec<f64>, Vec<f64>),
    /// Each solid's start offset along local Z (from its `Position`).
    from_mm: Vec<f64>,
    axis: Option<Vec<Vec<f64>>>,
    /// Set name -> property -> value text, for property sets and quantity sets alike.
    sets: BTreeMap<String, BTreeMap<String, String>>,
    material: Option<String>,
}

fn read_back(doc: &str) -> BTreeMap<String, ReadBack> {
    let entities = index(doc);
    let mut out = BTreeMap::new();
    for (&id, entity) in &entities {
        if !matches!(
            entity.name.as_str(),
            "IFCWALL" | "IFCSLAB" | "IFCFOOTING" | "IFCBUILDINGELEMENTPROXY"
        ) {
            continue;
        }
        let a = &entity.args;
        let mut rb = ReadBack {
            entity: entity.name.clone(),
            description: string(&a[3]),
            object_type: string(&a[4]),
            predefined: a[8].clone(),
            ..ReadBack::default()
        };
        let placement = &entities[&reference(&a[5])];
        let axes = &entities[&reference(&placement.args[1])];
        rb.placement = (
            numbers(&entities[&reference(&axes.args[0])].args[0]),
            numbers(&entities[&reference(&axes.args[1])].args[0]),
            numbers(&entities[&reference(&axes.args[2])].args[0]),
        );
        let pds = &entities[&reference(&a[6])];
        for shape in references(&pds.args[2]) {
            let shape = &entities[&shape];
            match shape.args[1].as_str() {
                "'Body'" => {
                    assert_eq!(shape.args[2], "'SweptSolid'");
                    for solid in references(&shape.args[3]) {
                        let solid = &entities[&solid];
                        assert_eq!(solid.name, "IFCEXTRUDEDAREASOLID");
                        assert_eq!(
                            numbers(&entities[&reference(&solid.args[2])].args[0]),
                            vec![0.0, 0.0, 1.0],
                            "extruded along local +Z"
                        );
                        let position = &entities[&reference(&solid.args[1])];
                        let start = numbers(&entities[&reference(&position.args[0])].args[0]);
                        rb.from_mm.push(start[2]);
                        let depth: f64 = solid.args[3].parse().unwrap();
                        let profile = &entities[&reference(&solid.args[0])];
                        let mut area = polyline_area(&entities, reference(&profile.args[2]));
                        if profile.name == "IFCARBITRARYPROFILEDEFWITHVOIDS" {
                            for hole in references(&profile.args[3]) {
                                area -= polyline_area(&entities, hole);
                                rb.holes += 1;
                            }
                        } else {
                            assert_eq!(profile.name, "IFCARBITRARYCLOSEDPROFILEDEF");
                        }
                        rb.volume_m3 += area * depth / 1.0e9;
                        rb.solids += 1;
                    }
                }
                "'Axis'" => {
                    assert_eq!(shape.args[2], "'Curve2D'");
                    let context = &entities[&reference(&shape.args[0])];
                    assert_eq!(context.name, "IFCGEOMETRICREPRESENTATIONSUBCONTEXT");
                    let line = &entities[&references(&shape.args[3])[0]];
                    rb.axis = Some(
                        references(&line.args[0])
                            .iter()
                            .map(|p| numbers(&entities[p].args[0]))
                            .collect(),
                    );
                }
                other => panic!("unexpected representation {other}"),
            }
        }
        for rel in entities.values() {
            if rel.name == "IFCRELDEFINESBYPROPERTIES" && references(&rel.args[4]).contains(&id) {
                let set = &entities[&reference(&rel.args[5])];
                let (name, members) = match set.name.as_str() {
                    "IFCPROPERTYSET" => (string(&set.args[2]).unwrap(), &set.args[4]),
                    "IFCELEMENTQUANTITY" => (string(&set.args[2]).unwrap(), &set.args[5]),
                    other => panic!("unexpected set {other}"),
                };
                let mut values = BTreeMap::new();
                for member in references(members) {
                    let member = &entities[&member];
                    let value = match member.name.as_str() {
                        "IFCPROPERTYSINGLEVALUE" => member.args[2].clone(),
                        "IFCQUANTITYWEIGHT" => {
                            let unit = &entities[&reference(&member.args[2])];
                            assert_eq!(unit.args[1..], [".MASSUNIT.", ".KILO.", ".GRAM."]);
                            member.args[3].clone()
                        }
                        _ => {
                            assert_eq!(member.args[2], "$", "length/area/volume in the file units");
                            format!("{}:{}", member.name, member.args[3])
                        }
                    };
                    values.insert(string(&member.args[0]).unwrap(), value);
                }
                rb.sets.insert(name, values);
            }
            if rel.name == "IFCRELASSOCIATESMATERIAL" && references(&rel.args[4]).contains(&id) {
                rb.material = string(&entities[&reference(&rel.args[5])].args[0]);
            }
        }
        out.insert(string(&a[2]).unwrap(), rb);
    }
    out
}

fn frame(origin: [f64; 3]) -> Value {
    json!({"origin": origin, "uDir": [1, 0, 0], "vDir": [0, 1, 0], "normal": [0, 0, 1]})
}

/// A wall with an axis, a slab with two holes, a pad footing, a strip footing with an axis, a
/// USERDEFINED wall and a generic shape: the S3 fixture set.
fn element_scene() -> Value {
    json!({
        "meta": { "name": "Walls and slabs", "units": "mm", "up": "z" },
        "groups": [{ "key": "wall", "color": "#94a3b8" }],
        "elements": [
            { "id": "W-1", "kind": "extrusion", "class": "wall", "predefinedType": "SOLIDWALL",
              "group": "wall",
              "frame": frame([1000.0, 2000.0, 0.0]),
              "items": [{ "outline": [[0,-100],[6000,-100],[6000,100],[0,100]], "fromMm": 0, "depthMm": 3000 }],
              "axis": { "u0": 0, "u1": 6000 },
              "loadBearing": true,
              "material": { "name": "Concrete block" },
              "quantities": { "lengthMm": 6000, "widthMm": 200, "heightMm": 3000,
                              "grossSideAreaM2": 18, "netSideAreaM2": 18,
                              "grossVolumeM3": 3.6, "netVolumeM3": 3.6 },
              "meta": { "description": "Wall W-1 on grid A, 200 CMU",
                        "properties": { "name": "FloLess",
                                        "properties": { "DefinitionId": "def-cmu-200",
                                                        "AddOn": "walls-slabs" } } } },
            { "id": "S-1", "kind": "extrusion", "class": "slab", "predefinedType": "FLOOR",
              "frame": frame([0.0, 0.0, 2850.0]),
              "items": [{ "outline": [[0,0],[8000,0],[8000,6000],[0,6000]],
                          "holes": [[[1000,1000],[1000,2000],[2000,2000],[2000,1000]],
                                    [[5000,3000],[5000,4000],[6000,4000],[6000,3000]]],
                          "fromMm": 0, "depthMm": 150 }],
              "loadBearing": false,
              "material": { "name": "concrete block" },
              "quantities": { "widthMm": 150, "perimeterMm": 28000, "grossAreaM2": 48,
                              "netAreaM2": 46, "grossVolumeM3": 6.9, "netVolumeM3": 6.9 } },
            { "id": "F-PAD", "kind": "extrusion", "class": "footing", "predefinedType": "PAD_FOOTING",
              "frame": frame([-1000.0, -1000.0, -600.0]),
              "items": [{ "outline": [[0,0],[2000,0],[2000,2000],[0,2000]], "fromMm": 0, "depthMm": 600 }],
              "material": { "name": "Concrete C30/37" },
              "quantities": { "lengthMm": 2000, "widthMm": 2000, "heightMm": 600,
                              "crossSectionAreaM2": 4, "grossVolumeM3": 2.4, "netVolumeM3": 2.4,
                              "grossWeightKg": 5760, "netWeightKg": 5760 } },
            { "id": "F-STRIP", "kind": "extrusion", "class": "footing", "predefinedType": "STRIP_FOOTING",
              "frame": frame([1000.0, 2000.0, -500.0]),
              "items": [{ "outline": [[0,-300],[6000,-300],[6000,300],[0,300]], "fromMm": -100, "depthMm": 500 }],
              "axis": { "u0": 0, "u1": 6000 },
              "loadBearing": true,
              "quantities": { "lengthMm": 6000, "widthMm": 600, "heightMm": 500,
                              "grossVolumeM3": 1.8 } },
            { "id": "W-UD", "kind": "extrusion", "class": "wall", "predefinedType": "USERDEFINED",
              "objectType": "Acme CMU 200",
              "frame": frame([0.0, 8000.0, 0.0]),
              "items": [
                  { "outline": [[0,-100],[3000,-100],[3000,100],[0,100]], "fromMm": 0, "depthMm": 1000 },
                  { "outline": [[0,-100],[2500,-100],[2500,100],[0,100]], "fromMm": 1000, "depthMm": 2000 }
              ],
              "quantities": { "grossVolumeM3": 1.6 } },
            { "id": "X-1", "kind": "extrusion",
              "frame": frame([0.0, 0.0, 0.0]),
              "items": [{ "outline": [[0,0],[100,0],[0,100]], "fromMm": 0, "depthMm": 10 }] }
        ]
    })
}

fn build(scene: &Value) -> BuildResult {
    validate_scene(scene).unwrap();
    try_build_ifc(scene).unwrap()
}

fn quantity(rb: &ReadBack, set: &str, key: &str) -> f64 {
    let text = &rb.sets[set][key];
    text.rsplit(':').next().unwrap().parse().unwrap()
}

#[test]
fn walls_slabs_and_footings_are_their_own_ifc_classes_with_their_predefined_types() {
    let built = build(&element_scene());
    let rb = read_back(&built.doc);
    assert_eq!(rb.len(), 6);
    assert_eq!(rb["W-1"].entity, "IFCWALL");
    assert_eq!(rb["W-1"].predefined, ".SOLIDWALL.");
    assert_eq!(rb["S-1"].entity, "IFCSLAB");
    assert_eq!(rb["S-1"].predefined, ".FLOOR.");
    assert_eq!(rb["F-PAD"].entity, "IFCFOOTING");
    assert_eq!(rb["F-PAD"].predefined, ".PAD_FOOTING.");
    assert_eq!(rb["F-STRIP"].predefined, ".STRIP_FOOTING.");
    assert_eq!(rb["X-1"].entity, "IFCBUILDINGELEMENTPROXY");
    assert_eq!(rb["X-1"].predefined, "$");
    // USERDEFINED carries its ObjectType (IFC4 CorrectPredefinedType); the others leave it unset.
    assert_eq!(rb["W-UD"].predefined, ".USERDEFINED.");
    assert_eq!(rb["W-UD"].object_type.as_deref(), Some("Acme CMU 200"));
    assert_eq!(rb["W-1"].object_type, None);
    assert_eq!(built.extrusions, 6);
    assert_eq!(built.profiles["WALL"], 2);
    assert_eq!(built.profiles["FOOTING"], 2);
    let rows: Vec<&Value> = built
        .emitted
        .iter()
        .filter(|row| row["kind"] == "extrusion")
        .collect();
    assert_eq!(rows.len(), 6);
    let row = |id: &str| *rows.iter().find(|row| row["id"] == id).unwrap();
    assert_eq!(row("W-1")["entityType"], "IfcWall");
    assert_eq!(row("F-PAD")["entityType"], "IfcFooting");
    assert_eq!(row("W-UD")["items"], 2);
    assert!(built.unsupported.is_empty() && built.warnings.is_empty());
}

#[test]
fn the_read_back_volume_of_every_solid_equals_its_scene_quantities() {
    let scene = element_scene();
    let built = build(&scene);
    let rb = read_back(&built.doc);
    for element in scene["elements"].as_array().unwrap() {
        let id = element["id"].as_str().unwrap();
        let Some(quantities) = element.get("quantities") else {
            continue;
        };
        let set = match element["class"].as_str().unwrap() {
            "wall" => "Qto_WallBaseQuantities",
            "slab" => "Qto_SlabBaseQuantities",
            _ => "Qto_FootingBaseQuantities",
        };
        let gross = quantities["grossVolumeM3"].as_f64().unwrap();
        // What the file's geometry encloses is what the scene counted...
        assert!(
            (rb[id].volume_m3 - gross).abs() < 1e-9,
            "{id}: the solids enclose {} m³, the scene says {gross}",
            rb[id].volume_m3
        );
        // ...and what the quantity set states.
        assert!(
            (quantity(&rb[id], set, "GrossVolume") - gross).abs() < 1e-12,
            "{id}"
        );
    }
    // The slab's two openings are real voids, not painted.
    assert_eq!(rb["S-1"].holes, 2);
    assert_eq!(rb["W-UD"].solids, 2);
    assert_eq!(rb["W-UD"].from_mm, vec![0.0, 1000.0]);
    assert_eq!(rb["F-STRIP"].from_mm, vec![-100.0]);
}

#[test]
fn quantities_carry_their_units_and_only_the_keys_sent() {
    let rb = read_back(&build(&element_scene()).doc);
    let wall = &rb["W-1"].sets["Qto_WallBaseQuantities"];
    assert_eq!(wall["Length"], "IFCQUANTITYLENGTH:6000.0");
    assert_eq!(wall["Width"], "IFCQUANTITYLENGTH:200.0");
    assert_eq!(wall["GrossSideArea"], "IFCQUANTITYAREA:18.0");
    assert_eq!(wall["NetVolume"], "IFCQUANTITYVOLUME:3.6");
    assert_eq!(wall.len(), 7);
    let slab = &rb["S-1"].sets["Qto_SlabBaseQuantities"];
    assert_eq!(slab["Width"], "IFCQUANTITYLENGTH:150.0");
    assert_eq!(slab["Perimeter"], "IFCQUANTITYLENGTH:28000.0");
    assert_eq!(slab["NetArea"], "IFCQUANTITYAREA:46.0");
    let pad = &rb["F-PAD"].sets["Qto_FootingBaseQuantities"];
    assert_eq!(pad["CrossSectionArea"], "IFCQUANTITYAREA:4.0");
    // Weight is in kilograms through an explicit SI unit on the quantity (checked in read_back).
    assert_eq!(pad["GrossWeight"], "5760.0");
    assert_eq!(rb["F-STRIP"].sets["Qto_FootingBaseQuantities"].len(), 4);
    assert!(!rb["X-1"].sets.contains_key("Qto_WallBaseQuantities") && rb["X-1"].sets.is_empty());
}

#[test]
fn load_bearing_goes_to_the_class_common_set_and_material_is_associated() {
    let rb = read_back(&build(&element_scene()).doc);
    assert_eq!(
        rb["W-1"].sets["Pset_WallCommon"]["LoadBearing"],
        "IFCBOOLEAN(.T.)"
    );
    assert_eq!(
        rb["S-1"].sets["Pset_SlabCommon"]["LoadBearing"],
        "IFCBOOLEAN(.F.)"
    );
    assert_eq!(
        rb["F-STRIP"].sets["Pset_FootingCommon"]["LoadBearing"],
        "IFCBOOLEAN(.T.)"
    );
    assert!(!rb["F-PAD"].sets.contains_key("Pset_FootingCommon"));
    assert_eq!(rb["W-1"].material.as_deref(), Some("Concrete block"));
    // Material names dedupe case-insensitively, as for members.
    assert_eq!(rb["S-1"].material.as_deref(), Some("Concrete block"));
    assert_eq!(rb["F-PAD"].material.as_deref(), Some("Concrete C30/37"));
    assert_eq!(rb["X-1"].material, None);
}

#[test]
fn the_placement_is_the_frame_and_the_axis_is_a_two_d_centreline() {
    let rb = read_back(&build(&element_scene()).doc);
    let wall = &rb["W-1"];
    assert_eq!(wall.placement.0, vec![1000.0, 2000.0, 0.0]);
    assert_eq!(wall.placement.1, vec![0.0, 0.0, 1.0]);
    assert_eq!(wall.placement.2, vec![1.0, 0.0, 0.0]);
    assert_eq!(
        wall.axis.as_deref(),
        Some(&[vec![0.0, 0.0], vec![6000.0, 0.0]][..])
    );
    assert!(rb["F-STRIP"].axis.is_some());
    assert!(rb["S-1"].axis.is_none() && rb["F-PAD"].axis.is_none());
}

#[test]
fn labels_go_through_the_member_label_path_and_their_removal_is_seen() {
    let scene = element_scene();
    let built = build(&scene);
    let rb = read_back(&built.doc);
    assert_eq!(
        rb["W-1"].description.as_deref(),
        Some("Wall W-1 on grid A, 200 CMU")
    );
    let floless = &rb["W-1"].sets["FloLess"];
    assert_eq!(
        floless["DefinitionId"],
        format!("IFCLABEL({})", s_lit("def-cmu-200"))
    );
    assert_eq!(
        floless["AddOn"],
        format!("IFCLABEL({})", s_lit("walls-slabs"))
    );
    let row = built.emitted.iter().find(|row| row["id"] == "W-1").unwrap();
    assert_eq!(row["description"], true);
    assert_eq!(row["propertySet"], "FloLess");

    // Negative control: the same scene without `meta.properties` writes no FloLess set, so a
    // label read-back that expects it fails.
    let mut unlabelled = scene.clone();
    unlabelled["elements"][0]["meta"]
        .as_object_mut()
        .unwrap()
        .remove("properties");
    let rb = read_back(&build(&unlabelled).doc);
    assert!(!rb["W-1"].sets.contains_key("FloLess"));
    let label_read_back = |rb: &ReadBack| -> Result<(), String> {
        let set = rb.sets.get("FloLess").ok_or("no FloLess set")?;
        (set.get("DefinitionId").map(String::as_str)
            == Some(format!("IFCLABEL({})", s_lit("def-cmu-200")).as_str()))
        .then_some(())
        .ok_or_else(|| "wrong DefinitionId".to_string())
    };
    assert!(label_read_back(&rb["W-1"]).is_err());
}

#[test]
fn a_malformed_extrusion_label_is_refused_naming_the_extrusion() {
    let mut scene = element_scene();
    scene["elements"][0]["meta"]["properties"]["name"] = json!("Pset_Mine");
    let error = validate_scene(&scene).unwrap_err().to_string();
    assert!(
        error.contains("ifc write: extrusion `W-1` `meta.properties.name`"),
        "{error}"
    );
}

#[test]
fn an_invalid_extrusion_is_refused_with_the_shared_rule_before_anything_is_written() {
    for (path, value, expected) in [
        ("/frame/normal", json!([0, 0, -1]), "frame is left-handed"),
        (
            "/items/0/depthMm",
            json!(0),
            "items[0].depthMm must be greater than zero",
        ),
        (
            "/items/0/outline",
            json!([[0, 100], [6000, 100], [6000, -100], [0, -100]]),
            "items[0].outline winds clockwise",
        ),
        (
            "/predefinedType",
            json!("FLOOR"),
            "is not in the IFC4 enumeration for a wall",
        ),
        (
            "/quantities/grossVolumeM3",
            json!(-1),
            "must not be negative",
        ),
    ] {
        let mut scene = element_scene();
        *scene["elements"][0].pointer_mut(path).unwrap() = value;
        let error = validate_scene(&scene).unwrap_err().to_string();
        assert!(
            error.contains("ifc write: extrusion `W-1` (`scene.elements[0]`): ")
                && error.contains(expected),
            "{path}: {error}"
        );
    }
    let mut scene = element_scene();
    scene["elements"][4]
        .as_object_mut()
        .unwrap()
        .remove("objectType");
    let error = validate_scene(&scene).unwrap_err().to_string();
    assert!(error.contains("objectType is required when predefinedType is USERDEFINED"));
}

#[test]
fn an_extrusion_scene_writes_identical_bytes_every_time() {
    let scene = element_scene();
    assert_eq!(build(&scene).doc, build(&scene).doc);
}

#[test]
fn a_scene_without_extrusions_has_no_axis_context_or_mass_unit() {
    let scene = json!({ "meta": { "name": "m" }, "elements": [
        { "id": "B1", "role": "beam", "from": [0,0,0], "to": [3000,0,0], "section": {"w":100,"d":200} }
    ]});
    let doc = build(&scene).doc;
    assert!(!doc.contains("IFCGEOMETRICREPRESENTATIONSUBCONTEXT"));
    assert!(!doc.contains(".MASSUNIT."));
}

#[test]
fn a_banded_wall_is_one_wall_with_one_solid_per_band() {
    let mut scene = element_scene();
    let bands: Vec<Value> = (0..5)
        .map(|band| {
            json!({ "outline": [[0,-100],[6000 - 100 * band,-100],[6000 - 100 * band,100],[0,100]],
                    "fromMm": 600 * band, "depthMm": 600 })
        })
        .collect();
    scene["elements"][0]["items"] = json!(bands);
    let volume: f64 = (0..5)
        .map(|band| (6000.0 - 100.0 * band as f64) * 200.0 * 600.0 / 1.0e9)
        .sum();
    scene["elements"][0]["quantities"]["grossVolumeM3"] = json!(volume);
    let rb = read_back(&build(&scene).doc);
    assert_eq!(rb["W-1"].solids, 5);
    assert_eq!(rb["W-1"].from_mm, vec![0.0, 600.0, 1200.0, 1800.0, 2400.0]);
    assert!((rb["W-1"].volume_m3 - volume).abs() < 1e-9);
}

#[test]
fn a_thin_band_keeps_its_true_depth() {
    let mut scene = element_scene();
    scene["elements"][0]["items"][0]["depthMm"] = json!(0.1);
    let doc = build(&scene).doc;
    let rb = read_back(&doc);
    assert!((rb["W-1"].volume_m3 - 6000.0 * 200.0 * 0.1 / 1.0e9).abs() < 1e-15);
}
