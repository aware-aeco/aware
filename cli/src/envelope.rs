//! JSON envelope per cli-spec.md § Response envelope.
//!
//! `print_ok` is consumed by Tasks 9/12/13/14 (agent list, agent show, app
//! list, app show).  `EnvelopeError` and `meta_for` are consumed when
//! error-envelope output is added in v0.2.
#![allow(dead_code)]

use std::time::Instant;

use serde::Serialize;
use serde_json::Value;

#[derive(Serialize)]
pub struct Envelope<T: Serialize> {
    pub ok: bool,
    pub data: Option<T>,
    pub error: Option<EnvelopeError>,
    pub meta: Meta,
}

#[derive(Serialize)]
pub struct EnvelopeError {
    pub code: String,
    pub message: String,
    pub details: Value,
}

#[derive(Serialize)]
pub struct Meta {
    #[serde(rename = "cli-version")]
    pub cli_version: &'static str,
    pub command: String,
    #[serde(rename = "duration-ms")]
    pub duration_ms: u128,
}

pub fn meta_for(command: &str, started: Instant) -> Meta {
    Meta {
        cli_version: env!("CARGO_PKG_VERSION"),
        command: command.to_string(),
        duration_ms: started.elapsed().as_millis(),
    }
}

/// Print a successful envelope to stdout.
pub fn print_ok<T: Serialize>(command: &str, data: T, started: Instant) -> std::io::Result<()> {
    let env = Envelope {
        ok: true,
        data: Some(data),
        error: None,
        meta: meta_for(command, started),
    };
    println!("{}", serde_json::to_string(&env).unwrap());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ok_envelope_round_trips() {
        let env = Envelope {
            ok: true,
            data: Some(serde_json::json!({"n": 7})),
            error: None,
            meta: Meta {
                cli_version: "0.1.0",
                command: "agent list".into(),
                duration_ms: 1,
            },
        };
        let s = serde_json::to_string(&env).unwrap();
        let v: Value = serde_json::from_str(&s).unwrap();
        assert_eq!(v["ok"], true);
        assert_eq!(v["data"]["n"], 7);
        assert!(v["error"].is_null());
        assert_eq!(v["meta"]["command"], "agent list");
    }
}
