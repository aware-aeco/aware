//! JSON envelope per cli-spec.md § Response envelope.
//!
//! `print_ok` is consumed by Tasks 9/12/13/14 (agent list, agent show, app
//! list, app show).  `EnvelopeError` and `meta_for` are consumed when
//! error-envelope output is added in v0.2.
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
    let json = serde_json::to_string(&env).map_err(std::io::Error::other)?;
    println!("{json}");
    Ok(())
}

// No unit tests here on purpose. The one that used to live at this spot
// (`ok_envelope_round_trips`) hand-built an `Envelope`, serialised it and read
// its own literals back — it never called `print_ok` or `meta_for`, so
// inverting `ok`, blanking the command in `meta_for` or dropping the
// `cli-version` / `duration-ms` renames all left it green. The envelope frame
// is now asserted end-to-end against the real binary in
// `tests/search_filters.rs::json_output_is_wrapped_in_the_spec_envelope`, which
// goes red on each of those.
