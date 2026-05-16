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

    pub fn config_path(&self) -> PathBuf {
        self.aware_home.join("config.yaml")
    }

    pub fn cache_dir(&self) -> PathBuf {
        self.aware_home.join("cache")
    }

    #[allow(dead_code)] // consumed by v0.4 (aware connect)
    pub fn credentials_dir(&self) -> PathBuf {
        self.aware_home.join("credentials")
    }

    #[allow(dead_code)] // consumed by Task 13 (aware app run)
    pub fn logs_dir(&self) -> PathBuf {
        self.aware_home.join("logs")
    }

    #[allow(dead_code)] // consumed by Task 14 (aware app run long-running)
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
    fn app_instance_dir_nests_correctly() {
        assert_eq!(
            p("/x").app_instance_dir("myapp", "prod"),
            PathBuf::from("/x/apps/myapp/instances/prod"),
        );
    }
}
