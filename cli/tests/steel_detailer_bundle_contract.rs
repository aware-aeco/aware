//! The first-party `aware-aeco` bundle installs the current steel-detailer
//! releases: the ones carrying the #796 limit rows (published by #825).

use std::fs;
use std::path::{Path, PathBuf};

const DETAILERS: [(&str, &str, &str); 3] = [
    (
        "steel-detailer-us",
        "0.6.0",
        "20-agents/aeco/engineering/steel-detailer-us",
    ),
    (
        "steel-detailer-uk",
        "0.3.0",
        "20-agents/aeco/engineering/steel-detailer-uk",
    ),
    (
        "steel-detailer-eu",
        "0.4.0",
        "20-agents/aeco/engineering/steel-detailer-eu",
    ),
];

fn repository_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("cli crate must live directly below the repository root")
        .to_path_buf()
}

fn load_index() -> serde_json::Value {
    let path = repository_root().join("registry-index.json");
    serde_json::from_str(
        &fs::read_to_string(&path)
            .unwrap_or_else(|error| panic!("read {}: {error}", path.display())),
    )
    .unwrap_or_else(|error| panic!("parse {}: {error}", path.display()))
}

#[test]
fn first_party_bundle_installs_the_current_steel_detailer_releases() {
    let index = load_index();
    let agents = index["bundles"]["aware-aeco"]["agents"]
        .as_array()
        .expect("aware-aeco bundle must list agents");

    for (id, version, relative) in DETAILERS {
        let pinned: Vec<&str> = agents
            .iter()
            .filter_map(|entry| entry.as_str())
            .filter(|entry| entry.split_once('@').map(|(name, _)| name) == Some(id))
            .collect();
        assert_eq!(
            pinned,
            vec![format!("{id}@{version}").as_str()],
            "the bundle must pin {id} exactly once, at {version}"
        );

        // The pinned release exists in the index, and it is the version the
        // manifest in this checkout declares.
        assert!(
            index["agents"][id]["versions"][version].is_object(),
            "{id}@{version} must be a published release"
        );
        let manifest_path = repository_root().join(relative).join("manifest.yaml");
        let manifest: serde_yaml::Value = serde_yaml::from_str(
            &fs::read_to_string(&manifest_path)
                .unwrap_or_else(|error| panic!("read {}: {error}", manifest_path.display())),
        )
        .unwrap_or_else(|error| panic!("parse {}: {error}", manifest_path.display()));
        assert_eq!(manifest["version"].as_str(), Some(version), "{id} manifest");

        // 0.1.0 stays addressable by name but is no longer what the bundle installs.
        assert!(
            index["agents"][id]["versions"]["0.1.0"].is_object(),
            "{id}@0.1.0 must remain published"
        );
    }
}
