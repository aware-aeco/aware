//! Install every agent in a registry bundle. Sequential, best-effort.
//!
//! Consumed by Task 11 (aware agent install <bundle>).

use crate::error::AwareError;
use crate::install::registry::install_agent_from_registry;
use crate::paths::Paths;
use crate::registry::Index;

#[derive(Debug)]
pub struct BundleReport {
    pub bundle: String,
    pub installed: Vec<String>,
    pub failed: Vec<(String, String)>, // (agent_spec, error message)
}

pub fn install_bundle(
    name: &str,
    paths: &Paths,
    index: &Index,
) -> Result<BundleReport, AwareError> {
    let bundle = index
        .bundles
        .get(name)
        .ok_or_else(|| AwareError::NotFound(format!("bundle {name} not in registry")))?;

    let mut installed = Vec::new();
    let mut failed = Vec::new();

    for spec in &bundle.agents {
        let (id, version_pin) = match spec.split_once('@') {
            Some((i, v)) => (i, Some(v)),
            None => (spec.as_str(), None),
        };
        match install_agent_from_registry(id, version_pin, paths, index) {
            Ok(_) => installed.push(spec.clone()),
            Err(e) => failed.push((spec.clone(), e.to_string())),
        }
    }

    Ok(BundleReport {
        bundle: name.to_string(),
        installed,
        failed,
    })
}
