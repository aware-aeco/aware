//! One-use, process-private REST header supplied by a trusted CLI launcher.
//! The value never enters an app input, lock, trace, or command line.

use std::sync::{Arc, Mutex};

use serde::Deserialize;

use crate::error::AwareError;
use crate::manifest::app::{App, Node};

pub const DESCRIPTOR_ENV: &str = "AWARE_PRIVATE_REST_HEADER";
pub const VALUE_ENV: &str = "AWARE_PRIVATE_REST_HEADER_VALUE";

/// Defense in depth for the runtime's child launchers. The synchronous
/// bootstrap already removes all spellings from the parent environment.
pub fn scrub_tokio_child(command: &mut tokio::process::Command) {
    command.env_remove(DESCRIPTOR_ENV).env_remove(VALUE_ENV);
}

pub fn scrub_std_child(command: &mut std::process::Command) {
    command.env_remove(DESCRIPTOR_ENV).env_remove(VALUE_ENV);
}

#[derive(Clone, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Descriptor {
    pub node_id: String,
    pub agent: String,
    pub command: String,
    pub method: String,
    pub origin: String,
    pub path: String,
    pub header_name: String,
}

struct State {
    descriptor: Descriptor,
    value: Option<String>,
    consumed: bool,
}

#[derive(Clone)]
pub struct PrivateRestHeader(Arc<Mutex<State>>);

fn invalid(detail: &str) -> AwareError {
    AwareError::Validation(format!("[E_APP_PRIVATE_REST_HEADER] {detail}"))
}

fn env_name(name: &std::ffi::OsStr) -> String {
    name.to_string_lossy().to_ascii_uppercase()
}

/// Must be called by synchronous `main` before Tokio creates any threads.
pub fn take_from_environment() -> Result<Option<PrivateRestHeader>, AwareError> {
    let reserved: Vec<_> = std::env::vars_os()
        .filter(|(key, _)| env_name(key).starts_with("AWARE_PRIVATE_REST_"))
        .collect();
    // Rust 2024 marks process-env mutation unsafe because another thread may
    // inspect it concurrently. main calls this before constructing Tokio or
    // starting any AWARE thread; no AWARE child can inherit these values.
    for (key, _) in &reserved {
        // SAFETY: synchronous main invokes this before Tokio or any AWARE thread exists.
        unsafe { std::env::remove_var(key) };
    }
    if reserved.is_empty() {
        return Ok(None);
    }
    if reserved.len() != 2
        || reserved.iter().any(|(key, _)| {
            let name = env_name(key);
            name != DESCRIPTOR_ENV && name != VALUE_ENV
        })
    {
        return Err(invalid(
            "private REST header environment is incomplete or ambiguous",
        ));
    }
    let find = |wanted: &str| {
        reserved
            .iter()
            .find(|(key, _)| env_name(key) == wanted)
            .map(|(_, value)| value.to_string_lossy().into_owned())
    };
    let (Some(raw_descriptor), Some(value)) = (find(DESCRIPTOR_ENV), find(VALUE_ENV)) else {
        return Err(invalid("private REST header environment is incomplete"));
    };
    if raw_descriptor.len() > 2048 || value.len() < 16 || value.len() > 512 {
        return Err(invalid(
            "private REST header descriptor or value has an invalid size",
        ));
    }
    if !value.bytes().all(|b| (0x21..=0x7e).contains(&b)) {
        return Err(invalid("private REST header value must use visible ASCII"));
    }
    let descriptor: Descriptor = serde_json::from_str(&raw_descriptor)
        .map_err(|_| invalid("private REST header descriptor is invalid"))?;
    descriptor.validate()?;
    Ok(Some(PrivateRestHeader(Arc::new(Mutex::new(State {
        descriptor,
        value: Some(value),
        consumed: false,
    })))))
}

impl Descriptor {
    fn validate(&self) -> Result<(), AwareError> {
        for part in [&self.node_id, &self.agent, &self.command] {
            if part.is_empty()
                || part.len() > 128
                || !part
                    .bytes()
                    .all(|b| b.is_ascii_alphanumeric() || matches!(b, b'-' | b'_' | b'.'))
            {
                return Err(invalid("private REST header node or command is invalid"));
            }
        }
        if !matches!(
            self.method.as_str(),
            "GET" | "POST" | "PUT" | "PATCH" | "DELETE" | "HEAD"
        ) {
            return Err(invalid("private REST header method is invalid"));
        }
        let origin = url::Url::parse(&self.origin)
            .map_err(|_| invalid("private REST header origin is invalid"))?;
        if !matches!(origin.scheme(), "http" | "https")
            || origin.host().is_none()
            || origin.username() != ""
            || origin.password().is_some()
            || origin.path() != "/"
            || origin.query().is_some()
            || origin.fragment().is_some()
            || origin.origin().ascii_serialization() != self.origin
        {
            return Err(invalid("private REST header origin is invalid"));
        }
        if !self.path.starts_with('/')
            || self.path.len() > 2048
            || self.path.contains(['?', '#'])
            || self.path.bytes().any(|b| b < 0x21 || b == 0x7f)
        {
            return Err(invalid("private REST header path is invalid"));
        }
        let name = self.header_name.as_bytes();
        let lower = self.header_name.to_ascii_lowercase();
        if name.len() < 3
            || name.len() > 128
            || !lower.starts_with("x-")
            || !name.iter().all(|b| b.is_ascii_alphanumeric() || *b == b'-')
            || lower.starts_with("x-forwarded-")
        {
            return Err(invalid(
                "private REST header name must be a safe custom header",
            ));
        }
        Ok(())
    }
}

impl PrivateRestHeader {
    pub fn descriptor(&self) -> Result<Descriptor, AwareError> {
        self.0
            .lock()
            .map(|state| state.descriptor.clone())
            .map_err(|_| invalid("private REST header state is unavailable"))
    }

    pub fn validate_app_graph(&self, app: &App) -> Result<(), AwareError> {
        let bound = self.descriptor()?;
        let matching: Vec<_> = app
            .nodes
            .iter()
            .filter(|node| node.id == bound.node_id)
            .collect();
        fn count_ids(nodes: &[Node], wanted: &str) -> usize {
            nodes
                .iter()
                .map(|node| {
                    usize::from(node.id == wanted)
                        + node
                            .do_
                            .as_deref()
                            .map_or(0, |body| count_ids(body, wanted))
                })
                .sum()
        }
        if matching.len() != 1
            || count_ids(&app.nodes, &bound.node_id) != 1
            || matching[0].agent.as_deref() != Some(bound.agent.as_str())
            || matching[0].command.as_deref() != Some(bound.command.as_str())
            || matching[0].frozen.is_some()
            || matching[0].do_.is_some()
            || matching[0].for_each.is_some()
            || matching[0].sweep.is_some()
            || app.schedule.is_some()
        {
            return Err(invalid("bound node is missing or may dispatch repeatedly"));
        }
        Ok(())
    }

    /// A matching node may claim once. A mismatched target node fails before network I/O.
    pub fn claim(
        &self,
        node_id: &str,
        agent: &str,
        command: &str,
        method: &str,
        url: &str,
        headers: &[(String, String)],
    ) -> Result<Option<(String, String)>, AwareError> {
        let mut state = self
            .0
            .lock()
            .map_err(|_| invalid("private REST header state is unavailable"))?;
        let bound = &state.descriptor;
        if node_id != bound.node_id {
            return Ok(None);
        }
        let parsed = url::Url::parse(url)
            .map_err(|_| invalid("private REST header target URL is invalid"))?;
        if agent != bound.agent
            || command != bound.command
            || method != bound.method
            || parsed.origin().ascii_serialization() != bound.origin
            || !parsed.username().is_empty()
            || parsed.password().is_some()
            || parsed.path() != bound.path
            || parsed.query().is_some()
            || parsed.fragment().is_some()
        {
            return Err(invalid("private REST header target changed"));
        }
        if headers
            .iter()
            .any(|(name, _)| name.eq_ignore_ascii_case(&bound.header_name))
        {
            return Err(invalid(
                "private REST header collides with another request header",
            ));
        }
        if state.consumed {
            return Err(invalid("private REST header was already used"));
        }
        state.consumed = true;
        let name = state.descriptor.header_name.clone();
        let value = state
            .value
            .take()
            .ok_or_else(|| invalid("private REST header was already used"))?;
        Ok(Some((name, value)))
    }

    pub fn require_consumed(&self) -> Result<(), AwareError> {
        let state = self
            .0
            .lock()
            .map_err(|_| invalid("private REST header state is unavailable"))?;
        if state.consumed {
            Ok(())
        } else {
            Err(invalid("private REST header was not used by this run"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn descriptor() -> Descriptor {
        Descriptor {
            node_id: "model".into(),
            agent: "floless-workspace".into(),
            command: "read-model-complete".into(),
            method: "POST".into(),
            origin: "http://127.0.0.1:47833".into(),
            path: "/api/agent/workspace/model/report-stream".into(),
            header_name: "X-FloLess-Report-Run".into(),
        }
    }

    fn grant() -> PrivateRestHeader {
        PrivateRestHeader(Arc::new(Mutex::new(State {
            descriptor: descriptor(),
            value: Some("a-private-capability-123456789".into()),
            consumed: false,
        })))
    }

    #[test]
    fn descriptor_rejects_bad_scope_and_reserved_names() {
        for name in [
            "Authorization",
            "Host",
            "Content-Length",
            "Connection",
            "Cookie",
            "X-Forwarded-Host",
            "X-Bad\r\nName",
        ] {
            let mut bound = descriptor();
            bound.header_name = name.into();
            assert!(bound.validate().is_err(), "{name}");
        }
        let mut bound = descriptor();
        bound.origin = "http://user:pass@127.0.0.1:47833".into();
        assert!(bound.validate().is_err());
        let mut bound = descriptor();
        bound.path = "/report?secret=1".into();
        assert!(bound.validate().is_err());
    }

    #[test]
    fn mismatch_does_not_consume_and_target_drift_fails() {
        let grant = grant();
        assert!(
            grant
                .claim(
                    "other",
                    "floless-workspace",
                    "read-model-complete",
                    "POST",
                    "http://127.0.0.1:47833/api/agent/workspace/model/report-stream",
                    &[]
                )
                .unwrap()
                .is_none()
        );
        assert!(grant.require_consumed().is_err());
        for url in [
            "http://127.0.0.1:47834/api/agent/workspace/model/report-stream",
            "http://127.0.0.1:47833/other",
            "http://127.0.0.1:47833/api/agent/workspace/model/report-stream?x=1",
        ] {
            assert!(
                grant
                    .claim(
                        "model",
                        "floless-workspace",
                        "read-model-complete",
                        "POST",
                        url,
                        &[]
                    )
                    .is_err()
            );
        }
        assert!(grant.require_consumed().is_err());
    }

    #[test]
    fn one_match_only_and_case_folded_collision_fails() {
        let grant = grant();
        let url = "http://127.0.0.1:47833/api/agent/workspace/model/report-stream";
        assert!(
            grant
                .claim(
                    "model",
                    "floless-workspace",
                    "read-model-complete",
                    "POST",
                    url,
                    &[("x-floless-report-run".into(), "other".into())]
                )
                .is_err()
        );
        let (name, value) = grant
            .claim(
                "model",
                "floless-workspace",
                "read-model-complete",
                "POST",
                url,
                &[],
            )
            .unwrap()
            .unwrap();
        assert_eq!(name, "X-FloLess-Report-Run");
        assert_eq!(value, "a-private-capability-123456789");
        assert!(grant.require_consumed().is_ok());
        assert!(
            grant
                .claim(
                    "model",
                    "floless-workspace",
                    "read-model-complete",
                    "POST",
                    url,
                    &[]
                )
                .is_err()
        );
    }

    #[test]
    fn child_scrubber_removes_the_private_environment() {
        #[cfg(windows)]
        let mut command = {
            let mut command = std::process::Command::new("cmd");
            command.args(["/C", "echo %AWARE_PRIVATE_REST_HEADER_VALUE%"]);
            command
        };
        #[cfg(not(windows))]
        let mut command = {
            let mut command = std::process::Command::new("sh");
            command.args(["-c", "printf '%s' \"$AWARE_PRIVATE_REST_HEADER_VALUE\""]);
            command
        };
        command.env(VALUE_ENV, "should-not-leak-to-child");
        scrub_std_child(&mut command);
        let output = command.output().unwrap();
        assert!(output.status.success());
        assert!(!String::from_utf8_lossy(&output.stdout).contains("should-not-leak-to-child"));
    }

    #[test]
    fn graph_binding_rejects_duplicate_frozen_and_nested_nodes() {
        let base = "app: fixture\nversion: 0.1.0\ndescription: fixture\nnodes:\n  - id: model\n    agent: floless-workspace\n    command: read-model-complete\nconnections: []\n";
        let app: App = serde_yaml::from_str(base).unwrap();
        assert!(grant().validate_app_graph(&app).is_ok());
        let duplicate = base.replace("connections: []", "  - id: model\n    agent: floless-workspace\n    command: read-model-complete\nconnections: []");
        let app: App = serde_yaml::from_str(&duplicate).unwrap();
        assert!(grant().validate_app_graph(&app).is_err());
        let frozen = base.replace(
            "    command: read-model-complete",
            "    command: read-model-complete\n    frozen: {ok: true}",
        );
        let app: App = serde_yaml::from_str(&frozen).unwrap();
        assert!(grant().validate_app_graph(&app).is_err());
        let nested = base.replace("connections: []", "  - id: repeat\n    for-each: '{{ inputs.items }}'\n    do:\n      - id: model\n        agent: floless-workspace\n        command: read-model-complete\nconnections: []");
        let app: App = serde_yaml::from_str(&nested).unwrap();
        assert!(grant().validate_app_graph(&app).is_err());
    }
}
