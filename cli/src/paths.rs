//! Filesystem path resolution for the AWARE CLI.
//!
//! `AWARE_HOME` env var overrides the default `~/.aware/` location.
//! Tests rely on this override to avoid polluting the real home dir.

use std::path::PathBuf;

use crate::error::AwareError;

#[derive(Debug, Clone)]
pub struct Paths {
    pub aware_home: PathBuf,
}

impl Paths {
    pub fn from_env() -> Result<Self, AwareError> {
        let aware_home = match std::env::var_os("AWARE_HOME") {
            Some(p) => PathBuf::from(p),
            None => dirs::home_dir()
                .ok_or_else(|| AwareError::Internal("could not determine home directory".into()))?
                .join(".aware"),
        };
        Ok(Self { aware_home })
    }

    pub fn agents_dir(&self) -> PathBuf {
        self.aware_home.join("agents")
    }

    pub fn apps_dir(&self) -> PathBuf {
        self.aware_home.join("apps")
    }

    /// The immutable, content-addressed agent store (#626):
    /// `agent-store-v2/<id>/<tree-hex>/<receipt-key>/`. Written only by
    /// [`crate::agent_store::snapshot`] and the one-time legacy import; a
    /// package is never modified, and removed only by `aware agent gc` (#629).
    /// `-v2` since #627-b: older CLIs (0.149-0.151) use `agent-store/` and
    /// take no run leases, so only this store is safe to garbage-collect.
    pub fn agent_store_dir(&self) -> PathBuf {
        self.aware_home.join("agent-store-v2")
    }

    /// The store of AWARE 0.149-0.151 (#626 layout). Read by the one-time
    /// import into [`Self::agent_store_dir`]; never written or deleted by this
    /// CLI (#627-b).
    pub fn legacy_agent_store_dir(&self) -> PathBuf {
        self.aware_home.join("agent-store")
    }

    /// Control files of the agent store (#627): `store.flock`, the store
    /// reference lock every reference writer holds shared and GC exclusive.
    pub fn agent_store_control_dir(&self) -> PathBuf {
        self.aware_home.join("agent-store-control")
    }

    /// The swap area under `agents/` (#627): journaled install/update/uninstall
    /// transactions `<txn>/` and per-agent swap locks `locks/<id>.flock`. Same
    /// volume as `agents/<id>`, so every swap is a same-directory-tree rename;
    /// two levels deep, so agent discovery never mistakes it for an agent.
    pub fn agent_swap_dir(&self) -> PathBuf {
        self.agents_dir().join(crate::install::swap::SWAP_DIR)
    }

    /// Carry-forward policies (#628 PR3b): `<policy-id>.yaml`, immutable once
    /// written; `<policy-id>.revoked.yaml` revokes one.
    pub fn migration_policies_dir(&self) -> PathBuf {
        self.aware_home.join("migration-policies")
    }

    /// Per-app promotion locks (#628 PR3b): `<key>.flock`, held exclusive by
    /// `migrate promote|revert` and by a compile writing the same `<app>.lock`.
    pub fn migration_locks_dir(&self) -> PathBuf {
        self.aware_home.join("migration-locks")
    }

    pub fn config_path(&self) -> PathBuf {
        self.aware_home.join("config.yaml")
    }

    pub fn cache_dir(&self) -> PathBuf {
        self.aware_home.join("cache")
    }

    pub fn credentials_dir(&self) -> PathBuf {
        self.aware_home.join("credentials")
    }

    pub fn logs_dir(&self) -> PathBuf {
        self.aware_home.join("logs")
    }

    pub fn providers_dir(&self) -> PathBuf {
        self.aware_home.join("providers")
    }

    pub fn app_instance_dir(&self, app: &str, instance: &str) -> PathBuf {
        self.apps_dir().join(app).join("instances").join(instance)
    }

    pub fn diagrams_dir(&self) -> PathBuf {
        self.aware_home.join("diagrams")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn p(home: &str) -> Paths {
        Paths {
            aware_home: PathBuf::from(home),
        }
    }

    #[test]
    fn agents_dir_appends_agents() {
        assert_eq!(p("/x").agents_dir(), PathBuf::from("/x/agents"));
    }

    #[test]
    fn agent_store_dir_appends_agent_store() {
        assert_eq!(
            p("/x").agent_store_dir(),
            PathBuf::from("/x/agent-store-v2")
        );
        assert_eq!(
            p("/x").legacy_agent_store_dir(),
            PathBuf::from("/x/agent-store")
        );
    }

    #[test]
    fn apps_dir_appends_apps() {
        assert_eq!(p("/x").apps_dir(), PathBuf::from("/x/apps"));
    }

    #[test]
    fn config_path_appends_config_yaml() {
        assert_eq!(p("/x").config_path(), PathBuf::from("/x/config.yaml"));
    }

    #[test]
    fn cache_dir_appends_cache() {
        assert_eq!(p("/x").cache_dir(), PathBuf::from("/x/cache"));
    }

    #[test]
    fn credentials_dir_appends_credentials() {
        assert_eq!(p("/x").credentials_dir(), PathBuf::from("/x/credentials"));
    }

    #[test]
    fn logs_dir_appends_logs() {
        assert_eq!(p("/x").logs_dir(), PathBuf::from("/x/logs"));
    }

    #[test]
    fn providers_dir_appends_providers() {
        assert_eq!(p("/x").providers_dir(), PathBuf::from("/x/providers"));
    }

    #[test]
    fn app_instance_dir_nests_correctly() {
        assert_eq!(
            p("/x").app_instance_dir("myapp", "prod"),
            PathBuf::from("/x/apps/myapp/instances/prod"),
        );
    }
}
