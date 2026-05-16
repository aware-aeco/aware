//! `lockfile.yaml` written under `~/.aware/apps/<app>/`.
//! Pins exact agent versions resolved at install time.
//!
//! Consumed by Task 15 (aware app install with lockfile).

#![allow(dead_code)]

use std::collections::BTreeMap;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::error::AwareError;

#[derive(Debug, Serialize, Deserialize)]
pub struct Lockfile {
    pub app: String,
    pub version: String,
    #[serde(rename = "resolved-at")]
    pub resolved_at: String,
    #[serde(rename = "resolved-agents")]
    pub resolved_agents: BTreeMap<String, String>,
}

pub fn write(
    app_id: &str,
    app_version: &str,
    resolved: BTreeMap<String, String>,
    target: &Path,
) -> Result<(), AwareError> {
    let lock = Lockfile {
        app: app_id.to_string(),
        version: app_version.to_string(),
        resolved_at: chrono::Utc::now().to_rfc3339(),
        resolved_agents: resolved,
    };
    let yaml = serde_yaml::to_string(&lock)
        .map_err(|e| AwareError::Internal(format!("serialize lockfile: {e}")))?;
    std::fs::write(target, yaml)?;
    Ok(())
}

pub fn read(path: &Path) -> Result<Lockfile, AwareError> {
    let text = std::fs::read_to_string(path)?;
    serde_yaml::from_str(&text)
        .map_err(|e| AwareError::Validation(format!("{}: {e}", path.display())))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trips() {
        let tmp = tempfile::tempdir().unwrap();
        let target = tmp.path().join("lockfile.yaml");
        let mut resolved = BTreeMap::new();
        resolved.insert("tekla".into(), "2025.0.1".into());
        resolved.insert("trimble-connect".into(), "2.4.0".into());
        write("welded-to-tc", "0.3.1", resolved.clone(), &target).unwrap();

        let read_back = read(&target).unwrap();
        assert_eq!(read_back.app, "welded-to-tc");
        assert_eq!(read_back.version, "0.3.1");
        assert_eq!(read_back.resolved_agents, resolved);
        assert!(!read_back.resolved_at.is_empty());
    }
}
