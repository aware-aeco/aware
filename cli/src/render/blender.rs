//! `blender.*` — the builtin transport for the headless Blender visualization agent.
//!
//! The `blender` agent is IFC in, rendered PNG / MP4 out, fully unattended. The work itself lives in
//! the `bpy` scripts shipped with the agent (`<AWARE_HOME>/agents/blender/scripts/`) — five fixed
//! recipes plus `exec.py`, which runs author-time code pinned in the approved lock (#692); this
//! module is the launcher that runs them:
//!
//! ```text
//! blender -b -P <scripts-dir>/<script>.py -- @<temp-inputs>.json
//! ```
//!
//! ## Why a builtin and not a bridge
//!
//! Every other host in the substrate (`aware-tekla`, `aware-rhino`, …) ships a bridge binary because
//! the work must happen *inside* a vendor runtime with its SDK loaded. Blender needs none of that —
//! it is a standalone executable driven from the command line — so a bridge would be a pure
//! process-launcher, and shipping one would cost a `BRIDGES` registration, `release.yml` staging, the
//! npm payload contract and both install scripts for no benefit. `vision.extract` is the precedent:
//! a builtin that depends on an external system (a pinned model over HTTP) and can fail on it.
//! Builtins are not required to be pure.
//!
//! ## The result protocol
//!
//! Blender floods stdout with progress, warnings and render statistics, so a bare `print(json…)`
//! from the script is unrecoverable. Every script instead frames its payload between
//! [`RESULT_BEGIN`] / [`RESULT_END`] sentinels (defined by the agent's `scripts/_result.py`) and this
//! module slices between them. A *named failure* is framed too — `{"ok": false, "code", "message",
//! "hint"}` — so the caller always gets structured data, never a bare Python traceback.
//!
//! **The sentinel, not the exit code, is the signal.** Blender can exit 0 with a script that never
//! ran, so a missing frame is its own failure mode and is reported with both output tails.
//!
//! ## The two named errors this module owns
//!
//! Per the design doc (`docs/superpowers/specs/2026-07-22-blender-visualization-agent-design.md`,
//! "Error handling"):
//!
//! - **No Blender executable** → an actionable error naming the search order and the `AWARE_BLENDER`
//!   override, never a raw `NotFound` io error.
//! - **Timeout** → per-call configurable via the `timeout-seconds` input; Blender's whole process
//!   tree is killed so a wedged Blender is never orphaned.
//! - **Ctrl-C** → the same tree kill, then an "interrupted" error. On Unix Blender leads its own
//!   process group, so the terminal's SIGINT does not reach it directly; the wait loop listens for
//!   Ctrl-C itself so a one-shot run cannot exit and leave Blender rendering.
//!
//! All three are [`AwareError::Network`] — the CLI's "agent runtime" class (exit 4 in `cli-spec.md`),
//! which is what `CliInvoker` already uses for a transport binary that will not spawn.
//!
//! ## Progress (#405 parity, #692)
//!
//! A render can run for hours, so the scripts append `{"$aware-progress": …}` lines to
//! `AWARE_PROGRESS_FILE` (`scripts/_progress.py`). On a run, this module owns that channel: it
//! drives the shared bounded [`Relay`] pump from INSIDE its own wait loop — beside the concurrent
//! pipe drains, the deadline and the process-tree kill — and drains it once more after an exit or
//! a timeout kill. See [`BlenderProgress`].

use crate::error::AwareError;
use crate::runtime::progress::Relay;
use crate::which::find_on_path;
use serde_json::{Map, Value};
use std::path::{Path, PathBuf};
use std::time::Duration;
use tokio::sync::mpsc;

/// The framing sentinels, mirrored from the agent's `scripts/_result.py`. Renaming one there is a
/// breaking change to this transport.
const RESULT_BEGIN: &str = "<<<AWARE_RESULT>>>";
const RESULT_END: &str = "<<<AWARE_RESULT_END>>>";

/// The environment override that always wins over discovery: point it at the Blender executable
/// itself (not its install directory).
const BLENDER_ENV: &str = "AWARE_BLENDER";

/// The executable's file name per platform.
const BLENDER_EXE: &str = if cfg!(windows) {
    "blender.exe"
} else {
    "blender"
};

/// How much of each output stream travels in a "no framed result" error. Matches the agent's own
/// smoke harness (`tests/run_smoke.py`) so a crash reads identically from either side.
const TAIL_CHARS: usize = 2000;

/// Longest budget a caller may ask for. Past a day a timeout is indistinguishable from having none,
/// and `Duration::from_secs_f64` panics on absurd values — so the cap is a guard, not a policy.
const MAX_TIMEOUT_SECONDS: f64 = 86_400.0;

/// Map an agent command to the `bpy` script that implements it. The single source of truth for the
/// command surface — an unknown command is named here rather than falling through to the generic
/// builtin-dispatch error, so the caller sees what *is* supported.
fn script_for_command(command: &str) -> Option<&'static str> {
    match command {
        "scene.import" => Some("scene_import.py"),
        "scene.info" => Some("scene_info.py"),
        "scene.apply-look" => Some("scene_apply_look.py"),
        "render.still" => Some("render_still.py"),
        "render.turntable" => Some("render_turntable.py"),
        "exec" => Some("exec.py"),
        _ => None,
    }
}

/// Every command this transport serves, in the order they compose (import → look → render).
const COMMANDS: &[&str] = &[
    "scene.import",
    "scene.info",
    "scene.apply-look",
    "render.still",
    "render.turntable",
    "exec",
];

/// The default wall-clock budget for a command, in seconds.
///
/// Renders are the long pole: a `production` (Cycles) still at high samples, or a turntable that is
/// hundreds of EEVEE frames, can legitimately run for many minutes on a modest machine — so those
/// get 30 minutes while the import / inspect commands (bounded by `ifcopenshell` tessellation of the
/// model) get 10. The default exists to stop a *wedged* Blender from hanging a workflow forever, not
/// to cap honest work; a caller who knows their scene overrides it with `timeout-seconds`.
fn default_timeout_seconds(command: &str) -> u64 {
    match command {
        // `exec` is render-class: author code that renders an animation is the reason it exists.
        "render.still" | "render.turntable" | "exec" => 1_800,
        _ => 600,
    }
}

/// The caller's budget for this run: the `timeout-seconds` input when given, else the per-command
/// default. `null` counts as absent (a templated-but-unset workflow input resolves to null).
fn timeout_for(command: &str, args: &Value) -> Result<Duration, AwareError> {
    let raw = match args.get("timeout-seconds") {
        None | Some(Value::Null) => {
            return Ok(Duration::from_secs(default_timeout_seconds(command)));
        }
        Some(v) => v,
    };
    let seconds = raw.as_f64().ok_or_else(|| {
        AwareError::Validation(format!(
            "blender {command}: `timeout-seconds` must be a number of seconds, got {raw}"
        ))
    })?;
    if !seconds.is_finite() || seconds <= 0.0 {
        return Err(AwareError::Validation(format!(
            "blender {command}: `timeout-seconds` must be a positive number of seconds, got {seconds}"
        )));
    }
    Ok(Duration::from_secs_f64(seconds.min(MAX_TIMEOUT_SECONDS)))
}

// ─────────────────────────────── binary discovery ───────────────────────────────

/// The `Blender Foundation` parents to sweep on Windows. Read from the environment (rather than
/// hard-coded) so a non-`C:` or localized install root still resolves.
fn windows_blender_roots() -> Vec<PathBuf> {
    let mut roots = Vec::new();
    for var in ["ProgramFiles", "ProgramW6432", "ProgramFiles(x86)"] {
        if let Some(value) = std::env::var_os(var) {
            let root = PathBuf::from(value).join("Blender Foundation");
            if !roots.contains(&root) {
                roots.push(root);
            }
        }
    }
    roots
}

/// Parse the version out of a `Blender <ver>` install-directory name into a comparable key —
/// `"Blender 5.2"` → `[5, 2]`. Each dot-separated component contributes its leading digits, so a
/// `"Blender 4.2 LTS"` folder still compares as `[4, 2]`. `None` for anything not shaped like an
/// install directory, which then simply does not participate in the choice.
fn blender_dir_version(name: &str) -> Option<Vec<u32>> {
    let lower = name.to_ascii_lowercase();
    let rest = lower.strip_prefix("blender ")?;
    let parts: Vec<u32> = rest
        .split('.')
        .map(|part| {
            part.trim_start()
                .chars()
                .take_while(char::is_ascii_digit)
                .collect::<String>()
        })
        .map_while(|digits| digits.parse::<u32>().ok())
        .collect();
    (!parts.is_empty()).then_some(parts)
}

/// Every `<root>/Blender <ver>/blender.exe` that exists, **highest version first**.
///
/// Several Blender versions commonly coexist under `Program Files`, so the pick must be
/// deterministic rather than whatever the directory listing happened to yield: candidates are sorted
/// by the parsed numeric version descending, with the directory name as a tie-break, and the first
/// existing executable wins. Pure over `roots` so it is testable on any platform.
fn windows_blender_candidates(roots: &[PathBuf]) -> Vec<PathBuf> {
    let mut found: Vec<(Vec<u32>, String, PathBuf)> = Vec::new();
    for root in roots {
        let Ok(entries) = std::fs::read_dir(root) else {
            continue;
        };
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().into_owned();
            let Some(version) = blender_dir_version(&name) else {
                continue;
            };
            let exe = entry.path().join(BLENDER_EXE);
            if exe.is_file() {
                found.push((version, name, exe));
            }
        }
    }
    // Descending by (version, name): newest install wins, ties resolve by name — never by listing order.
    found.sort_by(|a, b| b.0.cmp(&a.0).then_with(|| b.1.cmp(&a.1)));
    found.into_iter().map(|(_, _, exe)| exe).collect()
}

/// Well-known install locations to try after `PATH`, in preference order. Every branch is compiled
/// on every platform (`cfg!`, not `#[cfg]`) so nothing here rots on a host that never takes it.
fn platform_default_candidates() -> Vec<PathBuf> {
    if cfg!(windows) {
        windows_blender_candidates(&windows_blender_roots())
    } else if cfg!(target_os = "macos") {
        let mut candidates = vec![PathBuf::from(
            "/Applications/Blender.app/Contents/MacOS/Blender",
        )];
        if let Some(home) = dirs::home_dir() {
            candidates.push(home.join("Applications/Blender.app/Contents/MacOS/Blender"));
        }
        candidates
    } else {
        // Linux: PATH is normally enough; these cover a distro / snap install that shadowed it.
        vec![
            PathBuf::from("/usr/bin/blender"),
            PathBuf::from("/usr/local/bin/blender"),
            PathBuf::from("/snap/bin/blender"),
        ]
    }
}

/// Where discovery *looked*, in words, for the not-found error.
///
/// Deliberately not `platform_default_candidates()`: on Windows that list is what actually EXISTS,
/// which is empty in precisely the case this message is written for ("no Blender installed") — so
/// echoing it would print `()`. The reader needs the pattern that was swept, not the empty result.
fn platform_search_locations() -> Vec<String> {
    if cfg!(windows) {
        let roots = windows_blender_roots();
        if roots.is_empty() {
            return vec![format!(
                "%ProgramFiles%\\Blender Foundation\\Blender <version>\\{BLENDER_EXE}"
            )];
        }
        return roots
            .iter()
            .map(|r| format!("{}\\Blender <version>\\{BLENDER_EXE}", r.display()))
            .collect();
    }
    platform_default_candidates()
        .iter()
        .map(|p| p.display().to_string())
        .collect()
}

/// The named error for "nothing resolved" — the design doc's required missing-binary failure. Names
/// the whole search order *and* the override, so the reader can act without guessing.
fn blender_not_found_error() -> AwareError {
    AwareError::Network(format!(
        "blender: no Blender executable found. The `blender` agent drives a headless \
         Blender 4.2+ (`blender -b -P …`) and there is nothing to drive.\n  \
         searched: ${BLENDER_ENV} (unset), `blender` on PATH, then {}\n  \
         hint: install Blender from https://www.blender.org/download/ (or your package \
         manager), or point {BLENDER_ENV} at an existing executable — e.g. \
         {BLENDER_ENV}=\"C:\\Program Files\\Blender Foundation\\Blender 5.2\\blender.exe\" \
         on Windows, {BLENDER_ENV}=/Applications/Blender.app/Contents/MacOS/Blender on macOS",
        platform_search_locations().join(", ")
    ))
}

/// The named error for "the override points at nothing" — a set-but-wrong `AWARE_BLENDER` must not
/// silently fall through to discovery, or the user's explicit instruction is ignored.
fn blender_override_missing_error(value: &str) -> AwareError {
    AwareError::Network(format!(
        "blender: {BLENDER_ENV} is set to `{value}` but no executable is there.\n  \
         hint: point it at the Blender executable itself — \
         `…\\Blender Foundation\\Blender 5.2\\blender.exe` on Windows, \
         `/Applications/Blender.app/Contents/MacOS/Blender` on macOS — or unset \
         {BLENDER_ENV} to fall back to PATH and the default install locations"
    ))
}

/// Resolve the Blender executable: the `AWARE_BLENDER` override, then `blender` on `PATH`, then the
/// platform's default install locations. Pure over `override_value` so discovery is testable without
/// mutating process-global environment (which would race the rest of the test binary).
fn resolve_blender_binary(override_value: Option<&str>) -> Result<PathBuf, AwareError> {
    // An explicit override always wins, and a wrong one is an error rather than a fall-through.
    if let Some(raw) = override_value {
        let value = raw.trim();
        if !value.is_empty() {
            let direct = PathBuf::from(value);
            if direct.is_file() {
                return Ok(direct);
            }
            // Tolerate a bare command name in the override (`AWARE_BLENDER=blender-4.2`).
            if let Some(found) = find_on_path(value) {
                return Ok(found);
            }
            return Err(blender_override_missing_error(value));
        }
    }
    if let Some(found) = find_on_path("blender") {
        return Ok(found);
    }
    platform_default_candidates()
        .into_iter()
        .find(|c| c.is_file())
        .ok_or_else(blender_not_found_error)
}

/// The Blender executable for this run, honouring `AWARE_BLENDER`.
fn find_blender() -> Result<PathBuf, AwareError> {
    resolve_blender_binary(std::env::var("AWARE_BLENDER").ok().as_deref())
}

// ─────────────────────────────── script resolution ───────────────────────────────

/// The agent's `scripts/` directory. On a run, `agent_root` is the `blender` agent's resolved
/// store package (#626), so the scripts executed are the approved bytes; a resolved run that
/// lacks it refuses before reaching here. It is `None` only outside a resolved run (`aware agent
/// invoke`, `--simulate`), where the installed copy is used, resolved from the same
/// env-driven source the rest of the CLI uses (mirroring `invoker::bridges_dir`). A missing
/// directory is an *installation* error, not a path error.
fn scripts_dir(agent_root: Option<&Path>) -> Result<PathBuf, AwareError> {
    let root = match agent_root {
        Some(root) => root.to_path_buf(),
        None => crate::paths::Paths::from_env()?
            .agents_dir()
            .join("blender"),
    };
    let dir = root.join("scripts");
    if !dir.is_dir() {
        return Err(AwareError::NotFound(format!(
            "the `blender` agent is not installed (no {}) — run \
             `aware agent install blender` first",
            dir.display()
        )));
    }
    Ok(dir)
}

/// The script file for `command` inside an installed agent. A present agent whose script is missing
/// is a *damaged* install and says so — the fix (`agent update`) differs from "never installed".
fn script_path(command: &str, agent_root: Option<&Path>) -> Result<PathBuf, AwareError> {
    let script = script_for_command(command).ok_or_else(|| {
        AwareError::Validation(format!(
            "blender: no command `{command}` (available: {})",
            COMMANDS.join(", ")
        ))
    })?;
    let path = scripts_dir(agent_root)?.join(script);
    if !path.is_file() {
        return Err(AwareError::NotFound(format!(
            "the installed `blender` agent is damaged: {} is missing (the `{command}` command \
             script) — run `aware agent update blender` to restore it",
            path.display()
        )));
    }
    Ok(path)
}

// ─────────────────────────────── inputs plumbing ───────────────────────────────

/// A temp inputs file that deletes itself on drop, so an early `?` return — or a timeout — still
/// cleans up. The scripts accept `@<path>` precisely because a scene payload can exceed the OS
/// command-line length limit, so this is the normal path, not a fallback.
struct ScratchInputs(PathBuf);

impl ScratchInputs {
    fn write(args: &Value) -> Result<Self, AwareError> {
        use std::sync::atomic::{AtomicU64, Ordering};
        static SEQ: AtomicU64 = AtomicU64::new(0);
        let seq = SEQ.fetch_add(1, Ordering::Relaxed);
        let path =
            std::env::temp_dir().join(format!("aware-blender-{}-{seq}.json", std::process::id()));
        let text = serde_json::to_string(args)?;
        std::fs::write(&path, text.as_bytes()).map_err(|e| {
            AwareError::Internal(format!(
                "blender: cannot stage inputs at {}: {e}",
                path.display()
            ))
        })?;
        Ok(Self(path))
    }

    /// The `@<path>` argument form the scripts' `_result.parse_args` understands.
    fn arg(&self) -> String {
        format!("@{}", self.0.display())
    }
}

impl Drop for ScratchInputs {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.0);
    }
}

// ─────────────────────────────── result framing ───────────────────────────────

/// Slice the framed payload out of Blender's stdout, or `None` when the frame never appeared.
///
/// First `RESULT_BEGIN`, then the first `RESULT_END` after it — the same slice the agent's own smoke
/// harness takes. A script emits exactly one frame and exits immediately, so "first" is also "only";
/// anchoring on the first marker keeps a payload that happens to *contain* the sentinel text from
/// re-anchoring the slice.
fn extract_framed_payload(stdout: &str) -> Option<&str> {
    let start = stdout.find(RESULT_BEGIN)? + RESULT_BEGIN.len();
    let rest = &stdout[start..];
    let end = rest.find(RESULT_END)?;
    Some(rest[..end].trim())
}

/// The last `TAIL_CHARS` characters of `text` (char-boundary safe), for embedding in an error.
fn tail(text: &str) -> String {
    let count = text.chars().count();
    if count <= TAIL_CHARS {
        return text.trim_end().to_string();
    }
    text.chars()
        .skip(count - TAIL_CHARS)
        .collect::<String>()
        .trim_end()
        .to_string()
}

/// Turn a script's `{"ok": false, …}` payload into an `AwareError` that keeps the named error
/// intact — the whole point of the framing protocol is that `code` reaches the user, so it is never
/// flattened into "command failed".
///
/// `invalid-inputs` is the caller's mistake and maps to the validation class (exit 3); every other
/// code (`ifcopenshell-missing`, `ifc-unreadable`, `ifc-empty`, `blend-unreadable`, `render-failed`,
/// `unexpected-error`) is an agent-runtime failure (exit 4).
fn error_from_payload(command: &str, payload: &Value) -> AwareError {
    let field = |k: &str| payload.get(k).and_then(Value::as_str).unwrap_or("").trim();
    let code = field("code");
    let message = field("message");
    let hint = field("hint");

    let code_label = if code.is_empty() { "unknown" } else { code };
    let message_label = if message.is_empty() {
        "(no message)"
    } else {
        message
    };
    let mut text = format!("blender {command} failed [{code_label}]: {message_label}");
    if !hint.is_empty() {
        text.push_str(&format!("\n  hint: {hint}"));
    }
    // A traceback only rides along on `unexpected-error`, where it is the only diagnostic there is.
    if let Some(tb) = payload.get("traceback").and_then(Value::as_str) {
        text.push_str(&format!("\n  traceback: {}", tail(tb)));
    }
    if code == "invalid-inputs" {
        AwareError::Validation(text)
    } else {
        AwareError::Network(text)
    }
}

/// The "no frame" error: Blender crashed, the script died before `_result.run` installed its
/// handler, or the wrong file was executed. Carries BOTH output tails and the exit status — a bare
/// "no result" is the worst possible message, and the exit code alone cannot be trusted (Blender
/// exits 0 for a script that failed to import).
fn no_result_error(
    command: &str,
    blender: &Path,
    script: &Path,
    status: Option<i32>,
    stdout: &str,
    stderr: &str,
) -> AwareError {
    let code = match status {
        Some(c) => c.to_string(),
        None => "signal".to_string(),
    };
    AwareError::Network(format!(
        "blender {command}: no framed result in Blender's output (exit {code}). The script \
         never reached its result handler — Blender crashed, or failed before it could report. \
         Note the exit code is not the signal here; the missing `{RESULT_BEGIN}` frame is.\n  \
         blender: {}\n  script:  {}\n\
         --- stdout tail (last {TAIL_CHARS} chars) ---\n{}\n\
         --- stderr tail (last {TAIL_CHARS} chars) ---\n{}",
        blender.display(),
        script.display(),
        tail(stdout),
        tail(stderr)
    ))
}

// ─────────────────────────────── dry run ───────────────────────────────

// `abs_path` moved to `render::mod`; imported so this file's call sites (and
// their existing bare `abs_path(...)` calls) still resolve unchanged.
use super::abs_path;

/// A required, non-empty string input, named the way the script would name it.
fn require_str(command: &str, args: &Value, key: &str) -> Result<String, AwareError> {
    match args.get(key).and_then(Value::as_str).map(str::trim) {
        Some(v) if !v.is_empty() => Ok(v.to_string()),
        _ => Err(AwareError::Validation(format!(
            "blender {command}: required input `{key}` is missing"
        ))),
    }
}

/// `exec`'s pre-spawn checks: the program is present, the scene source is unambiguous, and there
/// is somewhere to write. Each is an `invalid-inputs` failure (exit 3) raised BEFORE Blender is
/// launched — a Blender start costs seconds, and a miswired node should not pay that to learn it.
fn validate_exec_inputs(args: &Value) -> Result<(), AwareError> {
    let invalid = |message: String| {
        AwareError::Validation(format!("blender exec failed [invalid-inputs]: {message}"))
    };
    match args.get("code") {
        Some(Value::String(code)) if !code.trim().is_empty() => {}
        Some(Value::String(_)) | None | Some(Value::Null) => {
            return Err(invalid(
                "required input `code` is missing or empty — it must be Python source that \
                 defines `run(ctx)`"
                    .into(),
            ));
        }
        Some(other) => {
            return Err(invalid(format!(
                "`code` must be a string of Python source, got {}",
                json_kind(other)
            )));
        }
    }
    let given = |key: &str| {
        args.get(key)
            .and_then(Value::as_str)
            .is_some_and(|v| !v.trim().is_empty())
    };
    if given("ifc-path") && given("blend-path") {
        return Err(invalid(
            "`ifc-path` and `blend-path` are mutually exclusive; give at most one".into(),
        ));
    }
    if !given("output-dir") {
        return Err(invalid(
            "required input `output-dir` is missing — the directory the code writes its \
             renders into"
                .into(),
        ));
    }
    if let Some(a) = args.get("args").filter(|a| !a.is_null() && !a.is_object()) {
        return Err(invalid(format!(
            "`args` must be an object, got {}",
            json_kind(a)
        )));
    }
    Ok(())
}

/// A JSON value's kind, for an error message.
fn json_kind(value: &Value) -> &'static str {
    match value {
        Value::Null => "null",
        Value::Bool(_) => "a boolean",
        Value::Number(_) => "a number",
        Value::String(_) => "a string",
        Value::Array(_) => "an array",
        Value::Object(_) => "an object",
    }
}

/// The `--dry-run` / `--simulate` posture: never spawn Blender, but still return the shape a
/// downstream node resolves against — above all the would-be output path — without writing anything.
///
/// Mirrors `render::file`'s posture in both directions: the *contract* keys are present with empty
/// values, and the inputs that decide those keys are validated up front, so a missing `output-path`
/// fails the same way whether previewing or running rather than being masked by the stub.
fn dry_run_result(command: &str, args: &Value) -> Result<Value, AwareError> {
    let mut out = Map::new();
    match command {
        "scene.import" => {
            let blend = abs_path(&require_str(command, args, "blend-path")?);
            out.insert("path".into(), Value::from(blend.clone()));
            out.insert("blend-path".into(), Value::from(blend));
        }
        "scene.apply-look" => {
            let blend = require_str(command, args, "blend-path")?;
            let output = args
                .get("output-path")
                .and_then(Value::as_str)
                .map(str::trim)
                .filter(|v| !v.is_empty())
                .unwrap_or(&blend);
            let output = abs_path(output);
            out.insert("path".into(), Value::from(output.clone()));
            out.insert("blend-path".into(), Value::from(output));
        }
        "render.still" | "render.turntable" => {
            let output = abs_path(&require_str(command, args, "output-path")?);
            out.insert("path".into(), Value::from(output.clone()));
            out.insert("output-path".into(), Value::from(output));
            out.insert("size-bytes".into(), Value::from(0u64));
        }
        "exec" => {
            // Validated exactly as a real run validates, so a preview fails the same way. The
            // echo carries the code's SIZE rather than the code: a preview record must not turn
            // into a second copy of the program.
            validate_exec_inputs(args)?;
            let output_dir = abs_path(&require_str(command, args, "output-dir")?);
            let code_bytes = args.get("code").and_then(Value::as_str).map_or(0, str::len);
            let mut echo = Map::new();
            echo.insert("code-bytes".into(), Value::from(code_bytes as u64));
            for key in [
                "args",
                "ifc-path",
                "blend-path",
                "preset",
                "timeout-seconds",
            ] {
                if let Some(v) = args.get(key).filter(|v| !v.is_null()) {
                    echo.insert(key.into(), v.clone());
                }
            }
            out.insert("output-dir".into(), Value::from(output_dir));
            out.insert("inputs".into(), Value::Object(echo));
            out.insert("result".into(), Value::Null);
            out.insert("outputs".into(), Value::Array(Vec::new()));
            out.insert("frames".into(), Value::from(0u64));
        }
        "scene.info" => {
            // Read-only: no output path to report, so the value here is the inventory contract —
            // and the same either/or the script enforces, so a preview catches the miswiring.
            let blend = args.get("blend-path").and_then(Value::as_str);
            let ifc = args.get("ifc-path").and_then(Value::as_str);
            let source = match (blend, ifc) {
                (Some(_), Some(_)) => {
                    return Err(AwareError::Validation(format!(
                        "blender {command}: pass exactly one of `blend-path` or `ifc-path`, not both"
                    )));
                }
                (Some(_), None) => "blend-path",
                (None, Some(_)) => "ifc-path",
                (None, None) => {
                    return Err(AwareError::Validation(format!(
                        "blender {command}: one of `blend-path` or `ifc-path` is required"
                    )));
                }
            };
            out.insert("count".into(), Value::from(0u64));
            out.insert("by-class".into(), Value::Object(Map::new()));
            out.insert("by-material".into(), Value::Object(Map::new()));
            out.insert("by-storey".into(), Value::Object(Map::new()));
            out.insert("elements".into(), Value::Array(Vec::new()));
            out.insert("source".into(), Value::from(source));
        }
        other => {
            return Err(AwareError::Validation(format!(
                "blender: no command `{other}` (available: {})",
                COMMANDS.join(", ")
            )));
        }
    }
    out.insert("ok".into(), Value::Bool(true));
    out.insert("dry-run".into(), Value::Bool(true));
    Ok(Value::Object(out))
}

// ─────────────────────────────── the launcher ───────────────────────────────

/// Drain a child pipe to EOF, tolerating a read error (whatever arrived still travels into the
/// error message, which is the only place it is used).
async fn read_to_end<R: tokio::io::AsyncRead + Unpin>(pipe: Option<R>) -> Vec<u8> {
    use tokio::io::AsyncReadExt;
    let Some(mut pipe) = pipe else {
        return Vec::new();
    };
    let mut buf = Vec::new();
    let _ = pipe.read_to_end(&mut buf).await;
    buf
}

/// Kill Blender and its whole process tree, then reap it — the timeout path.
///
/// The kill is the guard's own [`kill_tree`] (taking its pid disarms it), so a timeout and an
/// abandoned call end the same tree the same way: `Child::kill` alone leaves a grandchild running
/// — on Unix, anything authored code spawned — and a leaked headless Blender holds a GPU context
/// and the output file open. The tree dies BEFORE the reap, while the pid still names it.
async fn kill_blender(child: &mut tokio::process::Child, guard: &mut TreeKillGuard) {
    if let Some(pid) = guard.pid.take() {
        kill_tree(pid);
    }
    let _ = child.start_kill();
    let _ = child.wait().await;
}

/// Kills Blender's whole process tree when the call is ABANDONED rather than finished (#692
/// review). The run path can drop this future mid-render — a failed trace write returns early
/// through `?`, a cancelled run drops the call — and without this a headless Blender kept
/// rendering, GPU context and output file held, for up to `timeout-seconds`.
///
/// `kill_on_drop` alone ends only the direct child, which is why the tree kill mirrors
/// [`kill_blender`]: `taskkill /T /F` on Windows, the child's own process group elsewhere (it is
/// spawned as a group leader). The guard is declared AFTER the child so it drops first, while
/// the tree is still intact, and it is disarmed the moment the child is reaped — after that the
/// pid may already belong to someone else.
struct TreeKillGuard {
    pid: Option<u32>,
}

impl TreeKillGuard {
    fn disarm(&mut self) {
        self.pid = None;
    }
}

impl Drop for TreeKillGuard {
    fn drop(&mut self) {
        if let Some(pid) = self.pid.take() {
            kill_tree(pid);
        }
    }
}

/// The command that kills `pid` AND everything it started: `taskkill /T /F` on Windows, a
/// SIGKILL to the process group `pid` leads elsewhere (Blender is spawned as a group leader).
/// `windows` is a parameter rather than a `cfg!` inside, so both branches are tested everywhere.
fn tree_kill_command(pid: u32, windows: bool) -> std::process::Command {
    let mut kill = if windows {
        let mut c = std::process::Command::new("taskkill");
        c.args(["/PID", &pid.to_string(), "/T", "/F"]);
        c
    } else {
        let mut c = std::process::Command::new("kill");
        c.args(["-KILL", &format!("-{pid}")]);
        c
    };
    kill.stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null());
    kill
}

#[cfg(test)]
thread_local! {
    /// Every pid this thread asked [`kill_tree`] to kill — how a test sees WHICH kill a path used.
    static TREE_KILLS: std::cell::RefCell<Vec<u32>> = const { std::cell::RefCell::new(Vec::new()) };
}

/// Kill Blender's whole process tree — the one tree kill, shared by the timeout path and the
/// drop guard, so neither can quietly fall back to killing the direct child alone.
fn kill_tree(pid: u32) {
    #[cfg(test)]
    TREE_KILLS.with(|kills| kills.borrow_mut().push(pid));
    let _ = tree_kill_command(pid, cfg!(windows)).status();
}

/// How the wait on Blender ended.
enum Waited {
    /// Blender exited on its own (or waiting on it failed).
    Exited(std::io::Result<std::process::ExitStatus>),
    /// The `timeout-seconds` budget ran out.
    TimedOut,
    /// The user pressed Ctrl-C.
    Interrupted,
}

/// How long the progress drain after a timeout kill may take before the timeout is reported anyway.
const TIMEOUT_DRAIN_GRACE: Duration = Duration::from_secs(2);

/// The environment variable a producer appends `{"$aware-progress": …}` lines to (#405).
const PROGRESS_ENV: &str = "AWARE_PROGRESS_FILE";

/// Where a Blender child's progress records go (#405 parity for the builtin transport, #692).
pub enum BlenderProgress {
    /// No listener: the variable is REMOVED from the child's environment, so a value inherited
    /// from whoever launched `aware` cannot make Blender write somewhere nobody asked it to.
    Off,
    /// `aware agent invoke` under a caller that set `AWARE_PROGRESS_FILE` itself (FloLess's Render
    /// card): the caller's path is handed to Blender unchanged and the caller tails it.
    Passthrough(PathBuf),
    /// A workflow run: AWARE owns the channel file, tails it while Blender works, and forwards
    /// each record to `tx`, which the orchestrator writes into the trace as `node-progress`.
    Relay {
        path: PathBuf,
        tx: mpsc::Sender<Value>,
    },
}

impl BlenderProgress {
    /// The caller's own channel, when `aware agent invoke` was launched with one.
    pub fn from_caller_env() -> Self {
        match std::env::var_os(PROGRESS_ENV) {
            Some(path) if !path.is_empty() => Self::Passthrough(PathBuf::from(path)),
            _ => Self::Off,
        }
    }
}

/// Resolves when the user presses Ctrl-C (SIGINT on Unix, the console Ctrl-C/Ctrl-Break event on
/// Windows). Never resolves when the handler cannot be installed, so a platform without one simply
/// runs on to the timeout as before rather than failing the call.
async fn ctrl_c_pressed() {
    if tokio::signal::ctrl_c().await.is_err() {
        std::future::pending::<()>().await;
    }
}

/// Run one `blender` agent command: spawn headless Blender on the command's script, recover the
/// framed payload, and surface the script's named errors intact.
///
/// This is the entry point `BuiltinInvoker::invoke_single` dispatches `("blender", …)` to, and the
/// one the run path calls with a [`BlenderProgress::Relay`] so a long render reports while it works.
pub async fn run_blender_command(
    command: &str,
    args: &Value,
    dry_run: bool,
    agent_root: Option<&Path>,
    progress: BlenderProgress,
) -> Result<Value, AwareError> {
    // Reject an unknown command before anything else, so a typo never reports as "agent not
    // installed" just because the scripts directory is also missing.
    if script_for_command(command).is_none() {
        return Err(AwareError::Validation(format!(
            "blender: no command `{command}` (available: {})",
            COMMANDS.join(", ")
        )));
    }
    // The budget is validated on both postures so a bad value fails identically when previewing.
    let budget = timeout_for(command, args)?;
    if command == "exec" {
        validate_exec_inputs(args)?;
    }
    if dry_run {
        return dry_run_result(command, args);
    }

    let script = script_path(command, agent_root)?;
    let blender = find_blender()?;
    run_script(
        command,
        &blender,
        &script,
        args,
        budget,
        progress,
        ctrl_c_pressed(),
    )
    .await
}

/// Spawn `blender -b -P <script> -- @<inputs>` and turn what it prints into the command's result.
/// Split from [`run_blender_command`] so the process handling is testable with a stand-in
/// executable and a stand-in script, without an installed agent or a real Blender.
async fn run_script(
    command: &str,
    blender: &Path,
    script: &Path,
    args: &Value,
    budget: Duration,
    progress: BlenderProgress,
    interrupt: impl std::future::Future<Output = ()>,
) -> Result<Value, AwareError> {
    let inputs = ScratchInputs::write(args)?;

    // Exactly the invocation Phase 1 verified against Blender 5.2 — no extra flags (not even
    // `--factory-startup`), because deviating from the proven command line is how a working script
    // starts failing for reasons that have nothing to do with this transport.
    let mut launch = tokio::process::Command::new(blender);
    crate::private_rest_header::scrub_tokio_child(&mut launch);
    launch
        .arg("-b")
        .arg("-P")
        .arg(script)
        .arg("--")
        .arg(inputs.arg())
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        // The direct child dies with the call even if the guard below never runs.
        .kill_on_drop(true);
    // A group leader, so an abandoned call can take the whole tree down (see `TreeKillGuard`).
    #[cfg(unix)]
    launch.process_group(0);
    // Set ONLY when somebody reads the channel — a producer takes the variable's presence to mean
    // "the runtime is listening", exactly as on the CLI transport.
    let mut relay = match progress {
        BlenderProgress::Off => {
            launch.env_remove(PROGRESS_ENV);
            None
        }
        BlenderProgress::Passthrough(path) => {
            launch.env(PROGRESS_ENV, path);
            None
        }
        BlenderProgress::Relay { path, tx } => {
            launch.env(PROGRESS_ENV, &path);
            Some(Relay::new(path, tx))
        }
    };
    let mut child = launch.spawn().map_err(|e| {
        AwareError::Network(format!(
            "blender {command}: cannot spawn {}: {e}\n  hint: point {BLENDER_ENV} at a \
             working Blender executable",
            blender.display()
        ))
    })?;

    let mut tree_guard = TreeKillGuard { pid: child.id() };

    // Drain both pipes concurrently with the wait: Blender is chatty enough to fill a pipe buffer
    // and deadlock against a wait-then-read.
    let stdout_task = tokio::spawn(read_to_end(child.stdout.take()));
    let stderr_task = tokio::spawn(read_to_end(child.stderr.take()));

    // One loop owns the wait, the deadline AND the progress pump, so the relay runs inside the
    // same select that drains pipes and enforces the timeout — it never replaces that runner
    // (which is what `wait_with_output` would have done, losing the tree kill). `Child::wait` is
    // cancel-safe, so re-polling it every interval loses nothing.
    //
    // It also waits on `interrupt` (Ctrl-C in production). On Unix Blender leads its own process
    // group, so a terminal's SIGINT no longer reaches it, and a one-shot `aware app run` /
    // `aware agent invoke` has no handler of its own: without this branch SIGINT ended `aware`
    // without the tree kill and Blender rendered on. Pinned once, outside the loop, so the handler
    // registers on the first poll and stays registered for the whole call.
    let deadline = tokio::time::Instant::now() + budget;
    let mut interrupt = std::pin::pin!(interrupt);
    let waited = loop {
        tokio::select! {
            status = child.wait() => break Waited::Exited(status),
            _ = tokio::time::sleep_until(deadline) => break Waited::TimedOut,
            _ = &mut interrupt => break Waited::Interrupted,
            _ = tokio::time::sleep(crate::runtime::progress::POLL_INTERVAL), if relay.is_some() => {
                // The deadline and the interrupt stay live WHILE pumping: a pump blocked on a
                // full channel (a slow trace writer) must not carry the call past either.
                if let Some(relay) = relay.as_mut() {
                    tokio::select! {
                        pumped = tokio::time::timeout_at(deadline, relay.pump()) => {
                            if pumped.is_err() {
                                break Waited::TimedOut;
                            }
                        }
                        _ = &mut interrupt => break Waited::Interrupted,
                    }
                }
            }
        }
    };

    let status = match waited {
        Waited::Interrupted => {
            // Same order as the timeout: the tree dies first, then what it reported is delivered
            // (bounded), then the call fails so `aware` exits non-zero.
            kill_blender(&mut child, &mut tree_guard).await;
            if let Some(relay) = relay.as_mut() {
                let _ = tokio::time::timeout(TIMEOUT_DRAIN_GRACE, relay.drain_to_end()).await;
            }
            stdout_task.abort();
            stderr_task.abort();
            return Err(AwareError::Network(format!(
                "blender {command}: interrupted (Ctrl-C) and the Blender process was killed.
                   blender: {}
  script:  {}",
                blender.display(),
                script.display(),
            )));
        }
        Waited::Exited(status) => {
            let status = status.map_err(|e| {
                AwareError::Network(format!("blender {command}: waiting on Blender failed: {e}"))
            })?;
            // Reaped: its pid is free for reuse, so the guard must not fire on it any more.
            tree_guard.disarm();
            status
        }
        Waited::TimedOut => {
            // The design doc's other required named error. Kill first, report second — a wedged
            // Blender must not outlive the call.
            kill_blender(&mut child, &mut tree_guard).await;
            // The records written before the kill are most of the diagnosis ("it reached frame
            // 140 of 240"), so they are delivered before the error, not lost with the process.
            // Bounded, though: the call is already failing, and a consumer that stopped reading
            // must not hold the timeout error hostage.
            if let Some(relay) = relay.as_mut() {
                let _ = tokio::time::timeout(TIMEOUT_DRAIN_GRACE, relay.drain_to_end()).await;
            }
            stdout_task.abort();
            stderr_task.abort();
            return Err(AwareError::Network(format!(
                "blender {command}: timed out after {:.0}s and the Blender process was killed.\n  \
                 blender: {}\n  script:  {}\n  \
                 hint: raise the budget with the `timeout-seconds` input (default {}s for this \
                 command), or lower `samples` / `width-pixels` / `duration-seconds`",
                budget.as_secs_f64(),
                blender.display(),
                script.display(),
                default_timeout_seconds(command)
            )));
        }
    };
    // A producer that writes its last record and exits in the same breath would otherwise lose it
    // to the poll interval.
    if let Some(relay) = relay.as_mut() {
        relay.drain_to_end().await;
    }
    drop(relay);

    let stdout = String::from_utf8_lossy(&stdout_task.await.unwrap_or_default()).into_owned();
    let stderr = String::from_utf8_lossy(&stderr_task.await.unwrap_or_default()).into_owned();

    // The sentinel — NOT the exit code — is the signal: Blender can exit 0 having never run the
    // script at all.
    let Some(body) = extract_framed_payload(&stdout) else {
        return Err(no_result_error(
            command,
            blender,
            script,
            status.code(),
            &stdout,
            &stderr,
        ));
    };

    let payload: Value = serde_json::from_str(body).map_err(|e| {
        AwareError::Network(format!(
            "blender {command}: the framed result was not valid JSON: {e}\n  \
             --- framed body ---\n{}",
            tail(body)
        ))
    })?;
    if !payload.is_object() {
        return Err(AwareError::Network(format!(
            "blender {command}: the framed result was not a JSON object: {}",
            tail(body)
        )));
    }
    if payload.get("ok") == Some(&Value::Bool(false)) {
        return Err(error_from_payload(command, &payload));
    }
    // Success: the script's payload travels back verbatim (`path`, `output-path`, counts, framing…)
    // so downstream nodes read exactly what the agent's command contract documents.
    Ok(payload)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    // ── command → script mapping ──

    #[test]
    fn every_command_maps_to_its_script() {
        assert_eq!(script_for_command("scene.import"), Some("scene_import.py"));
        assert_eq!(script_for_command("scene.info"), Some("scene_info.py"));
        assert_eq!(
            script_for_command("scene.apply-look"),
            Some("scene_apply_look.py")
        );
        assert_eq!(script_for_command("render.still"), Some("render_still.py"));
        assert_eq!(
            script_for_command("render.turntable"),
            Some("render_turntable.py")
        );
        assert_eq!(script_for_command("exec"), Some("exec.py"));
        assert!(COMMANDS.contains(&"exec"), "exec must be advertised");
    }

    #[test]
    fn the_command_list_and_the_script_table_agree() {
        for command in COMMANDS {
            assert!(
                script_for_command(command).is_some(),
                "{command} is advertised but has no script"
            );
        }
    }

    #[test]
    fn an_unknown_command_has_no_script() {
        assert_eq!(script_for_command("render.video"), None);
        assert_eq!(script_for_command("scene"), None);
        assert_eq!(script_for_command(""), None);
    }

    #[tokio::test]
    async fn an_unknown_command_names_what_is_available() {
        let err = run_blender_command(
            "render.video",
            &json!({}),
            false,
            None,
            BlenderProgress::Off,
        )
        .await
        .unwrap_err();
        let text = err.to_string();
        assert!(matches!(err, AwareError::Validation(_)), "{text}");
        assert!(text.contains("no command `render.video`"), "{text}");
        assert!(text.contains("render.turntable"), "{text}");
    }

    // ── sentinel parsing ──

    #[test]
    fn framed_payload_is_sliced_out_of_the_noise() {
        let stdout = format!(
            "Blender 5.2\nreading …\n{RESULT_BEGIN}\n{{\n  \"ok\": true\n}}\n{RESULT_END}\nBlender quit\n"
        );
        assert_eq!(
            extract_framed_payload(&stdout),
            Some("{\n  \"ok\": true\n}")
        );
    }

    #[test]
    fn a_missing_sentinel_yields_no_payload() {
        assert_eq!(extract_framed_payload("Segmentation fault\n"), None);
        // An opening marker with no close is just as unusable as no marker at all.
        assert_eq!(
            extract_framed_payload(&format!("{RESULT_BEGIN}\n{{}}")),
            None
        );
    }

    #[tokio::test]
    async fn a_failed_payload_surfaces_its_code_message_and_hint() {
        let payload = json!({
            "ok": false,
            "code": "ifcopenshell-missing",
            "message": "ifcopenshell is not installed in Blender's Python",
            "hint": "blender -b --python-expr \"…pip install ifcopenshell…\"",
        });
        let err = error_from_payload("scene.import", &payload);
        let text = err.to_string();
        assert!(matches!(err, AwareError::Network(_)), "{text}");
        assert!(text.contains("[ifcopenshell-missing]"), "{text}");
        assert!(text.contains("not installed in Blender's Python"), "{text}");
        assert!(text.contains("pip install ifcopenshell"), "{text}");
    }

    #[test]
    fn invalid_inputs_is_a_validation_failure_not_a_runtime_one() {
        let err = error_from_payload(
            "render.still",
            &json!({ "ok": false, "code": "invalid-inputs", "message": "required input `output-path` is missing" }),
        );
        assert_eq!(err.exit_code(), 3, "{err}");
        assert!(err.to_string().contains("[invalid-inputs]"), "{err}");
    }

    #[test]
    fn a_render_failure_is_an_agent_runtime_failure() {
        let err = error_from_payload(
            "render.still",
            &json!({ "ok": false, "code": "render-failed", "message": "no camera" }),
        );
        assert_eq!(err.exit_code(), 4, "{err}");
    }

    #[test]
    fn a_codeless_failure_still_produces_a_readable_error() {
        let err = error_from_payload("scene.info", &json!({ "ok": false }));
        let text = err.to_string();
        assert!(text.contains("[unknown]"), "{text}");
        assert!(text.contains("(no message)"), "{text}");
    }

    #[test]
    fn the_no_result_error_carries_both_output_tails() {
        let err = no_result_error(
            "render.still",
            Path::new("/opt/blender"),
            Path::new("/agents/blender/scripts/render_still.py"),
            Some(0),
            "…progress…\nMEMORY-ERROR-ON-STDOUT",
            "Fatal Python error: THE-STDERR-DETAIL",
        );
        let text = err.to_string();
        assert!(text.contains("MEMORY-ERROR-ON-STDOUT"), "{text}");
        assert!(text.contains("THE-STDERR-DETAIL"), "{text}");
        assert!(text.contains("/opt/blender"), "{text}");
        assert!(text.contains("render_still.py"), "{text}");
        // Exit 0 with no frame is the trap this message exists to defuse.
        assert!(text.contains("exit 0"), "{text}");
    }

    #[test]
    fn tails_are_capped_and_char_boundary_safe() {
        let noisy = "é".repeat(TAIL_CHARS * 2);
        let cut = tail(&noisy);
        assert_eq!(cut.chars().count(), TAIL_CHARS);
    }

    // ── binary discovery ──

    #[test]
    fn discovery_honours_the_environment_override() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let exe = tmp.path().join("my-blender");
        std::fs::write(&exe, b"#!/bin/sh\n").expect("write");
        let found =
            resolve_blender_binary(Some(&exe.to_string_lossy())).expect("override should resolve");
        assert_eq!(found, exe);
    }

    #[test]
    fn a_blank_override_falls_through_to_discovery() {
        // Empty / whitespace is "unset", not "set to nothing" — it must not hard-fail.
        let outcome = resolve_blender_binary(Some("   "));
        // Either a real Blender was discovered on this host, or the named not-found error fired;
        // what must NOT happen is the override-missing error for a blank value.
        if let Err(err) = outcome {
            let text = err.to_string();
            assert!(text.contains("no Blender executable found"), "{text}");
        }
    }

    #[test]
    fn a_wrong_override_is_an_error_with_a_hint_not_a_fallthrough() {
        let err = resolve_blender_binary(Some("/definitely/not/a/blender/binary"))
            .expect_err("a set-but-wrong override must fail");
        let text = err.to_string();
        assert!(text.contains("AWARE_BLENDER"), "{text}");
        assert!(text.contains("/definitely/not/a/blender/binary"), "{text}");
        assert!(text.contains("hint:"), "{text}");
    }

    #[test]
    fn the_missing_binary_error_carries_the_install_hint_and_the_override() {
        let text = blender_not_found_error().to_string();
        assert!(text.contains("no Blender executable found"), "{text}");
        assert!(text.contains("AWARE_BLENDER"), "{text}");
        assert!(text.contains("blender.org/download"), "{text}");
        assert!(text.contains("PATH"), "{text}");
        // Agent-runtime class → exit 4, not a raw io error.
        assert_eq!(blender_not_found_error().exit_code(), 4);
    }

    #[test]
    fn the_missing_binary_error_names_where_it_looked_even_with_nothing_installed() {
        // The regression this guards: echoing the *found* candidates prints an empty `()` in
        // exactly the case the message exists for — a host with no Blender at all.
        let locations = platform_search_locations();
        assert!(!locations.is_empty(), "must always name a search location");
        for location in &locations {
            assert!(!location.trim().is_empty(), "{locations:?}");
        }
        let text = blender_not_found_error().to_string();
        assert!(text.contains(&locations[0]), "{text}");
    }

    #[test]
    fn install_directory_versions_parse_and_order() {
        assert_eq!(blender_dir_version("Blender 5.2"), Some(vec![5, 2]));
        assert_eq!(blender_dir_version("blender 4.2 LTS"), Some(vec![4, 2]));
        assert_eq!(blender_dir_version("Blender 4"), Some(vec![4]));
        assert_eq!(blender_dir_version("Blender Foundation"), None);
        assert_eq!(blender_dir_version("Notepad++"), None);
        assert!(vec![5u32, 2] > vec![4u32, 20]);
    }

    #[test]
    fn the_newest_installed_version_is_picked_deterministically() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let root = tmp.path().join("Blender Foundation");
        for version in ["Blender 4.2", "Blender 5.2", "Blender 4.11"] {
            let dir = root.join(version);
            std::fs::create_dir_all(&dir).expect("mkdir");
            std::fs::write(dir.join(BLENDER_EXE), b"x").expect("write");
        }
        // A directory that isn't an install must not participate.
        std::fs::create_dir_all(root.join("Blender Launcher")).expect("mkdir");

        let found = windows_blender_candidates(std::slice::from_ref(&root));
        assert_eq!(found.len(), 3, "{found:?}");
        assert!(
            found[0].to_string_lossy().contains("Blender 5.2"),
            "{found:?}"
        );
        // 4.11 sorts above 4.2 numerically — not lexically.
        assert!(
            found[1].to_string_lossy().contains("Blender 4.11"),
            "{found:?}"
        );
    }

    // ── timeouts ──

    #[test]
    fn renders_get_a_longer_default_budget_than_imports() {
        assert_eq!(default_timeout_seconds("render.still"), 1_800);
        assert_eq!(default_timeout_seconds("render.turntable"), 1_800);
        assert_eq!(default_timeout_seconds("exec"), 1_800);
        assert_eq!(default_timeout_seconds("scene.import"), 600);
        assert_eq!(default_timeout_seconds("scene.info"), 600);
    }

    #[test]
    fn the_budget_is_overridable_per_call() {
        let budget = timeout_for("render.still", &json!({ "timeout-seconds": 5 })).expect("ok");
        assert_eq!(budget, Duration::from_secs(5));
        let fractional =
            timeout_for("render.still", &json!({ "timeout-seconds": 0.25 })).expect("ok");
        assert_eq!(fractional, Duration::from_millis(250));
    }

    #[test]
    fn an_absent_or_null_budget_takes_the_default() {
        assert_eq!(
            timeout_for("scene.info", &json!({})).expect("ok"),
            Duration::from_secs(600)
        );
        assert_eq!(
            timeout_for("scene.info", &json!({ "timeout-seconds": null })).expect("ok"),
            Duration::from_secs(600)
        );
    }

    #[test]
    fn a_nonsense_budget_is_rejected_rather_than_panicking() {
        for bad in [json!("soon"), json!(0), json!(-3)] {
            let err = timeout_for("render.still", &json!({ "timeout-seconds": bad }))
                .expect_err("must reject");
            assert!(matches!(err, AwareError::Validation(_)), "{err}");
        }
        // Absurd budgets clamp instead of panicking inside `Duration::from_secs_f64`.
        let clamped =
            timeout_for("render.still", &json!({ "timeout-seconds": 1e30 })).expect("clamped");
        assert_eq!(clamped, Duration::from_secs_f64(MAX_TIMEOUT_SECONDS));
    }

    // ── dry run ──

    #[tokio::test]
    async fn a_dry_run_reports_the_would_be_output_path_without_spawning() {
        // No AWARE_HOME agent install and no Blender needed: a preview must never reach either.
        let out = run_blender_command(
            "render.still",
            &json!({ "blend-path": "m.blend", "output-path": "out/hero.png" }),
            true,
            None,
            BlenderProgress::Off,
        )
        .await
        .expect("dry run");
        assert_eq!(out["dry-run"], json!(true));
        assert_eq!(out["ok"], json!(true));
        let path = out["output-path"].as_str().expect("output-path");
        assert!(path.ends_with("hero.png"), "{path}");
        assert_eq!(out["path"], out["output-path"]);
        assert!(!std::path::Path::new(path).exists(), "nothing is written");
    }

    #[tokio::test]
    async fn a_dry_run_import_reports_the_blend_path() {
        let out = run_blender_command(
            "scene.import",
            &json!({ "ifc-path": "m.ifc", "blend-path": "staged/m.blend" }),
            true,
            None,
            BlenderProgress::Off,
        )
        .await
        .expect("dry run");
        assert_eq!(out["path"], out["blend-path"]);
        assert!(
            out["blend-path"]
                .as_str()
                .unwrap_or_default()
                .ends_with("m.blend")
        );
    }

    #[tokio::test]
    async fn a_dry_run_apply_look_defaults_the_output_to_the_input_blend() {
        let out = run_blender_command(
            "scene.apply-look",
            &json!({ "blend-path": "m.blend" }),
            true,
            None,
            BlenderProgress::Off,
        )
        .await
        .expect("dry run");
        assert!(
            out["blend-path"]
                .as_str()
                .unwrap_or_default()
                .ends_with("m.blend")
        );
        let redirected = run_blender_command(
            "scene.apply-look",
            &json!({ "blend-path": "m.blend", "output-path": "look/m2.blend" }),
            true,
            None,
            BlenderProgress::Off,
        )
        .await
        .expect("dry run");
        assert!(
            redirected["blend-path"]
                .as_str()
                .unwrap_or_default()
                .ends_with("m2.blend")
        );
    }

    #[tokio::test]
    async fn a_dry_run_info_returns_the_inventory_contract() {
        let out = run_blender_command(
            "scene.info",
            &json!({ "ifc-path": "m.ifc" }),
            true,
            None,
            BlenderProgress::Off,
        )
        .await
        .expect("dry run");
        assert_eq!(out["count"], json!(0));
        assert_eq!(out["source"], json!("ifc-path"));
        assert!(out["elements"].as_array().expect("elements").is_empty());
    }

    #[tokio::test]
    async fn a_dry_run_fails_the_same_way_a_real_run_would_on_missing_inputs() {
        for (command, args) in [
            ("render.still", json!({})),
            ("scene.import", json!({ "ifc-path": "m.ifc" })),
            ("scene.info", json!({})),
            (
                "scene.info",
                json!({ "ifc-path": "m.ifc", "blend-path": "m.blend" }),
            ),
        ] {
            let err = run_blender_command(command, &args, true, None, BlenderProgress::Off)
                .await
                .expect_err("preview must not mask a miswired node");
            assert!(matches!(err, AwareError::Validation(_)), "{err}");
        }
    }

    // ── exec (#692) ──

    fn exec_args() -> Value {
        json!({
            "code": "def run(ctx):\n    return {'n': 1}\n",
            "args": { "storeys": 3 },
            "output-dir": "out/anim",
        })
    }

    #[tokio::test]
    async fn exec_dry_run_echoes_the_inputs_without_spawning_or_copying_the_code() {
        let out = run_blender_command("exec", &exec_args(), true, None, BlenderProgress::Off)
            .await
            .expect("dry run");
        assert_eq!(out["dry-run"], json!(true));
        assert_eq!(out["frames"], json!(0));
        assert_eq!(out["outputs"], json!([]));
        assert!(
            out["output-dir"]
                .as_str()
                .unwrap_or_default()
                .ends_with("anim"),
            "{out}"
        );
        assert_eq!(out["inputs"]["args"], json!({ "storeys": 3 }));
        // The SIZE of the program travels, never the program itself.
        assert!(
            out["inputs"]["code-bytes"].as_u64().unwrap_or(0) > 0,
            "{out}"
        );
        assert!(!out.to_string().contains("def run"), "{out}");
    }

    #[tokio::test]
    async fn exec_inputs_are_validated_before_blender_is_ever_looked_for() {
        // `real` (dry_run = false) with a broken input must fail on the INPUT, exit 3, and never
        // reach script resolution or binary discovery — the agent is not installed here, so
        // reaching either would surface a different error.
        let mut cases = Vec::new();
        let mut no_code = exec_args();
        no_code.as_object_mut().unwrap().remove("code");
        cases.push((no_code, "`code`"));
        let mut blank = exec_args();
        blank["code"] = json!("   \n");
        cases.push((blank, "`code`"));
        let mut not_text = exec_args();
        not_text["code"] = json!({ "run": 1 });
        cases.push((not_text, "`code` must be a string"));
        let mut both = exec_args();
        both["ifc-path"] = json!("m.ifc");
        both["blend-path"] = json!("m.blend");
        cases.push((both, "mutually exclusive"));
        let mut no_dir = exec_args();
        no_dir.as_object_mut().unwrap().remove("output-dir");
        cases.push((no_dir, "`output-dir`"));
        let mut bad_args = exec_args();
        bad_args["args"] = json!([1, 2]);
        cases.push((bad_args, "`args` must be an object"));

        for (args, needle) in cases {
            for dry_run in [false, true] {
                let err = run_blender_command("exec", &args, dry_run, None, BlenderProgress::Off)
                    .await
                    .expect_err("a miswired exec must not run");
                let text = err.to_string();
                assert_eq!(err.exit_code(), 3, "{text}");
                assert!(text.contains("[invalid-inputs]"), "{text}");
                assert!(text.contains(needle), "{needle}: {text}");
            }
        }
    }

    /// A stand-in Blender: a script that writes `lines` to the progress channel, sleeping
    /// `pause_seconds` after the first, then prints `framed` between the result sentinels.
    /// `hang_seconds` > 0 makes it wedge instead of finishing. Returns the executable's path.
    fn fake_blender(
        dir: &Path,
        first: &str,
        later: &str,
        pause_seconds: u32,
        hang_seconds: u32,
        framed: &str,
    ) -> PathBuf {
        if cfg!(windows) {
            let path = dir.join("fake-blender.cmd");
            let mut body = String::from("@echo off\r\n");
            body.push_str(&format!(">>\"%AWARE_PROGRESS_FILE%\" echo {first}\r\n"));
            body.push_str(&format!("ping -n {} 127.0.0.1 >nul\r\n", pause_seconds + 1));
            if hang_seconds > 0 {
                // A wedged Blender never gets further. Exiting here (rather than writing the later
                // record) keeps a kill that reaches `ping` a moment before `cmd` from racing the
                // script on to its next line under load.
                body.push_str(&format!(
                    "ping -n {} 127.0.0.1 >nul\r\nexit /b 1\r\n",
                    hang_seconds + 1
                ));
            }
            body.push_str(&format!(">>\"%AWARE_PROGRESS_FILE%\" echo {later}\r\n"));
            body.push_str("echo Blender quit noise\r\n");
            body.push_str("echo ^<^<^<AWARE_RESULT^>^>^>\r\n");
            body.push_str(&format!("echo {framed}\r\n"));
            body.push_str("echo ^<^<^<AWARE_RESULT_END^>^>^>\r\n");
            std::fs::write(&path, body).expect("write fake blender");
            path
        } else {
            let path = dir.join("fake-blender");
            let mut body = String::from("#!/bin/sh\n");
            body.push_str(&format!(
                "printf '%s\\n' '{first}' >> \"$AWARE_PROGRESS_FILE\"\n"
            ));
            body.push_str(&format!("sleep {pause_seconds}\n"));
            if hang_seconds > 0 {
                body.push_str(&format!("sleep {hang_seconds}\nexit 1\n"));
            }
            body.push_str(&format!(
                "printf '%s\\n' '{later}' >> \"$AWARE_PROGRESS_FILE\"\n"
            ));
            body.push_str("echo 'Blender quit noise'\n");
            body.push_str("echo '<<<AWARE_RESULT>>>'\n");
            body.push_str(&format!("echo '{framed}'\n"));
            body.push_str("echo '<<<AWARE_RESULT_END>>>'\n");
            std::fs::write(&path, body).expect("write fake blender");
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o755))
                    .expect("chmod fake blender");
            }
            path
        }
    }

    /// Collect relayed records with the moment each arrived.
    fn collect(
        mut rx: mpsc::Receiver<Value>,
    ) -> tokio::task::JoinHandle<Vec<(std::time::Instant, Value)>> {
        tokio::spawn(async move {
            let mut seen = Vec::new();
            while let Some(record) = rx.recv().await {
                seen.push((std::time::Instant::now(), record));
            }
            seen
        })
    }

    #[tokio::test]
    async fn builtin_progress_is_relayed_while_blender_runs_and_drained_after_it_exits() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let blender = fake_blender(
            tmp.path(),
            r#"{"$aware-progress":{"phase":"render","done":1,"total":2,"message":"pass 1 of ?"}}"#,
            r#"{"$aware-progress":{"phase":"render","done":2,"total":2,"message":"pass 1 of ?"}}"#,
            2,
            0,
            r#"{"ok": true, "frames": 2, "outputs": [], "result": null}"#,
        );
        let channel = tmp.path().join("progress-1.jsonl");
        let (tx, rx) = mpsc::channel::<Value>(64);
        let collector = collect(rx);

        let started = std::time::Instant::now();
        let out = run_script(
            "exec",
            &blender,
            &tmp.path().join("exec.py"),
            &exec_args(),
            Duration::from_secs(60),
            BlenderProgress::Relay {
                path: channel.clone(),
                tx,
            },
            std::future::pending(),
        )
        .await
        .expect("the fake Blender frames a successful result");
        let returned = std::time::Instant::now();
        assert_eq!(out["frames"], json!(2), "{out}");

        let seen = collector.await.expect("collector");
        let done: Vec<u64> = seen
            .iter()
            .map(|(_, r)| r["done"].as_u64().unwrap_or(0))
            .collect();
        assert_eq!(done, vec![1, 2], "{seen:?}");
        assert_eq!(seen[1].1["total"], json!(2));
        // LIVE, not replayed at the end: the first record arrived while the producer still had a
        // two-second pause ahead of it.
        let lead = returned.duration_since(seen[0].0);
        assert!(
            lead >= Duration::from_millis(1_000),
            "first record arrived only {lead:?} before the call returned (started {:?} earlier)",
            returned.duration_since(started)
        );
        // The last record, written in the same breath as the exit, still arrived (final drain).
        assert!(seen[1].0 <= returned + Duration::from_millis(5));
    }

    #[tokio::test]
    async fn a_hung_blender_is_killed_on_timeout_and_its_progress_still_drains() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let blender = fake_blender(
            tmp.path(),
            r#"{"$aware-progress":{"phase":"render","done":3,"total":240}}"#,
            r#"{"$aware-progress":{"phase":"render","done":4,"total":240}}"#,
            0,
            120,
            r#"{"ok": true}"#,
        );
        let (tx, rx) = mpsc::channel::<Value>(64);
        let collector = collect(rx);

        let started = std::time::Instant::now();
        let err = run_script(
            "exec",
            &blender,
            &tmp.path().join("exec.py"),
            &exec_args(),
            Duration::from_millis(2_500),
            BlenderProgress::Relay {
                path: tmp.path().join("progress-2.jsonl"),
                tx,
            },
            std::future::pending(),
        )
        .await
        .expect_err("a wedged Blender must time out");
        let elapsed = started.elapsed();
        let text = err.to_string();
        assert!(text.contains("timed out"), "{text}");
        assert_eq!(err.exit_code(), 4, "{text}");
        assert!(
            elapsed < Duration::from_secs(30),
            "the timeout did not fire promptly: {elapsed:?}"
        );

        let seen = collector.await.expect("collector");
        assert_eq!(seen.len(), 1, "{seen:?}");
        assert_eq!(seen[0].1["done"], json!(3), "{seen:?}");
    }

    /// A stand-in Blender that starts a grandchild ticking into `ticks` every second, then hangs.
    /// Killing only the direct child leaves the grandchild ticking.
    fn fake_blender_with_ticking_child(dir: &Path, ticks: &Path) -> PathBuf {
        if cfg!(windows) {
            let ticker = dir.join("ticker.cmd");
            std::fs::write(
                &ticker,
                format!(
                    "@echo off\r\n:loop\r\n>>\"{}\" echo tick\r\nping -n 2 127.0.0.1 >nul\r\ngoto loop\r\n",
                    ticks.display()
                ),
            )
            .expect("write ticker");
            let path = dir.join("fake-blender.cmd");
            std::fs::write(
                &path,
                format!(
                    "@echo off\r\nstart \"\" /b cmd /c \"{}\"\r\nping -n 120 127.0.0.1 >nul\r\nexit /b 1\r\n",
                    ticker.display()
                ),
            )
            .expect("write fake");
            path
        } else {
            let path = dir.join("fake-blender");
            std::fs::write(
                &path,
                format!(
                    "#!/bin/sh\n(while true; do echo tick >> '{}'; sleep 1; done) &\nsleep 120\nexit 1\n",
                    ticks.display()
                ),
            )
            .expect("write fake");
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o755))
                    .expect("chmod fake blender");
            }
            path
        }
    }

    /// Require `ticks` to stop growing: whatever was ticking is dead.
    async fn assert_ticking_stopped(ticks: &Path, what: &str) {
        tokio::time::sleep(Duration::from_millis(2_500)).await;
        let settled = std::fs::read_to_string(ticks).unwrap_or_default().len();
        tokio::time::sleep(Duration::from_millis(3_500)).await;
        let later = std::fs::read_to_string(ticks).unwrap_or_default().len();
        assert_eq!(
            settled, later,
            "{what}: Blender's process tree kept running"
        );
    }

    /// Codex round 2: the TIMEOUT path must kill the whole tree through the same helper the drop
    /// guard uses. On Unix the old path killed Blender's pid alone (the group survived); here the
    /// helper records every pid it is asked to kill, so the path that was taken is observable on
    /// any platform — and the grandchild must stop ticking.
    #[tokio::test(flavor = "current_thread")]
    async fn the_timeout_path_kills_the_whole_tree_through_the_shared_helper() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let ticks = tmp.path().join("ticks.txt");
        let blender = fake_blender_with_ticking_child(tmp.path(), &ticks);
        TREE_KILLS.with(|kills| kills.borrow_mut().clear());

        let err = run_script(
            "exec",
            &blender,
            &tmp.path().join("exec.py"),
            &exec_args(),
            Duration::from_millis(3_000),
            BlenderProgress::Off,
            std::future::pending(),
        )
        .await
        .expect_err("the hanging stand-in must time out");
        assert!(err.to_string().contains("timed out"), "{err}");
        let kills = TREE_KILLS.with(|kills| kills.borrow().clone());
        assert_eq!(
            kills.len(),
            1,
            "the timeout must kill through the shared tree kill exactly once: {kills:?}"
        );
        assert!(ticks.exists(), "the grandchild never started");
        assert_ticking_stopped(&ticks, "after a timeout").await;
    }

    /// #692 review: on Unix Blender leads its own process group, so a terminal Ctrl-C no longer
    /// reaches it, and a one-shot run has no Ctrl-C handler of its own — SIGINT killed `aware`
    /// without running the tree kill and Blender rendered on. The wait loop therefore also waits on
    /// the interrupt: it must end the call promptly with an "interrupted" error (non-zero exit),
    /// kill through the shared tree kill exactly once, and leave nothing ticking. The interrupt is
    /// injected (production passes `ctrl_c_pressed()`), so this runs without sending a signal.
    #[tokio::test(flavor = "current_thread")]
    async fn an_interrupt_kills_the_whole_tree_and_fails_the_call() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let ticks = tmp.path().join("ticks.txt");
        let blender = fake_blender_with_ticking_child(tmp.path(), &ticks);
        TREE_KILLS.with(|kills| kills.borrow_mut().clear());

        let ticks_for_interrupt = ticks.clone();
        let interrupt = async move {
            while !ticks_for_interrupt.exists() {
                tokio::time::sleep(Duration::from_millis(100)).await;
            }
            tokio::time::sleep(Duration::from_millis(1_000)).await;
        };
        let started = std::time::Instant::now();
        let outcome = tokio::time::timeout(
            Duration::from_secs(25),
            run_script(
                "exec",
                &blender,
                &tmp.path().join("exec.py"),
                &exec_args(),
                Duration::from_secs(300),
                BlenderProgress::Off,
                interrupt,
            ),
        )
        .await;
        let err = outcome
            .expect("an interrupt must end the call, not leave it waiting on Blender")
            .expect_err("an interrupted call must fail");
        let text = err.to_string();
        assert!(text.contains("interrupted"), "{text}");
        assert_ne!(err.exit_code(), 0, "{text}");
        assert!(
            started.elapsed() < Duration::from_secs(20),
            "took {:?}",
            started.elapsed()
        );
        let kills = TREE_KILLS.with(|kills| kills.borrow().clone());
        assert_eq!(
            kills.len(),
            1,
            "the interrupt must kill through the shared tree kill exactly once: {kills:?}"
        );
        assert_ticking_stopped(&ticks, "after an interrupt").await;
    }

    #[test]
    fn the_tree_kill_targets_the_tree_on_both_platforms() {
        let windows = tree_kill_command(4242, true);
        assert_eq!(windows.get_program(), "taskkill");
        let args: Vec<_> = windows.get_args().collect();
        assert_eq!(args, ["/PID", "4242", "/T", "/F"]);

        // The leading `-` makes `kill` signal the PROCESS GROUP Blender leads, not one pid.
        let unix = tree_kill_command(4242, false);
        assert_eq!(unix.get_program(), "kill");
        let args: Vec<_> = unix.get_args().collect();
        assert_eq!(args, ["-KILL", "-4242"]);
    }

    /// #692 review: an ABANDONED call (the run path drops the future — a failed trace write, a
    /// cancelled run) must take Blender's whole process tree with it. The stand-in starts a
    /// grandchild that ticks into a file every second, then hangs; killing only the direct child
    /// would leave the grandchild ticking, so a file that keeps growing after the drop is the
    /// failure this guards.
    #[tokio::test]
    async fn dropping_the_call_kills_the_whole_blender_tree() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let ticks = tmp.path().join("ticks.txt");
        let blender = if cfg!(windows) {
            let ticker = tmp.path().join("ticker.cmd");
            std::fs::write(
                &ticker,
                format!(
                    "@echo off
:loop
>>\"{}\" echo tick
ping -n 2 127.0.0.1 >nul
goto loop
",
                    ticks.display()
                ),
            )
            .expect("write ticker");
            let path = tmp.path().join("fake-blender.cmd");
            std::fs::write(
                &path,
                format!(
                    "@echo off
start \"\" /b cmd /c \"{}\"
ping -n 120 127.0.0.1 >nul
",
                    ticker.display()
                ),
            )
            .expect("write fake");
            path
        } else {
            let path = tmp.path().join("fake-blender");
            std::fs::write(
                &path,
                format!(
                    "#!/bin/sh
(while true; do echo tick >> '{}'; sleep 1; done) &
sleep 120
",
                    ticks.display()
                ),
            )
            .expect("write fake");
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o755))
                    .expect("chmod fake blender");
            }
            path
        };

        let script = tmp.path().join("exec.py");
        let args = exec_args();
        let call = run_script(
            "exec",
            &blender,
            &script,
            &args,
            Duration::from_secs(300),
            BlenderProgress::Off,
            std::future::pending(),
        );
        // Abandon the call once the grandchild is demonstrably running.
        let ticks_for_wait = ticks.clone();
        let abandoned = tokio::time::timeout(Duration::from_secs(20), async move {
            tokio::select! {
                _ = call => panic!("the hanging stand-in must not finish"),
                _ = async {
                    while !ticks_for_wait.exists() {
                        tokio::time::sleep(Duration::from_millis(100)).await;
                    }
                    tokio::time::sleep(Duration::from_millis(1_500)).await;
                } => {}
            }
        })
        .await;
        assert!(abandoned.is_ok(), "the stand-in never started ticking");

        // Let any in-flight write land, then require the file to stop growing.
        tokio::time::sleep(Duration::from_millis(2_500)).await;
        let settled = std::fs::read_to_string(&ticks).unwrap_or_default().len();
        tokio::time::sleep(Duration::from_millis(3_500)).await;
        let later = std::fs::read_to_string(&ticks).unwrap_or_default().len();
        assert_eq!(
            settled, later,
            "Blender's process tree kept running after the call was dropped"
        );
    }

    /// Codex review: the deadline must hold while the relay is pumping. A consumer that has
    /// stopped reading (capacity-1 channel, receiver never polled) blocks `pump()` on `send`;
    /// the timeout must still fire on time and the process tree still die.
    #[tokio::test]
    async fn the_timeout_fires_even_while_a_full_channel_blocks_the_pump() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let blender = fake_blender(
            tmp.path(),
            r#"{"$aware-progress":{"phase":"render","done":1,"total":9}}"#,
            r#"{"$aware-progress":{"phase":"render","done":9,"total":9}}"#,
            0,
            120,
            r#"{"ok": true}"#,
        );
        // Several records up front, so the second `send` blocks on the full channel.
        let channel = tmp.path().join("progress-full.jsonl");
        let mut burst = String::new();
        for done in 1..=5 {
            burst.push_str(&format!(
                "{{\"$aware-progress\":{{\"phase\":\"render\",\"done\":{done},\"total\":9}}}}
"
            ));
        }
        std::fs::write(&channel, burst).expect("pre-seed channel");
        let (tx, _rx_never_read) = mpsc::channel::<Value>(1);

        let started = std::time::Instant::now();
        let outcome = tokio::time::timeout(
            Duration::from_secs(20),
            run_script(
                "exec",
                &blender,
                &tmp.path().join("exec.py"),
                &exec_args(),
                Duration::from_millis(1_500),
                BlenderProgress::Relay { path: channel, tx },
                std::future::pending(),
            ),
        )
        .await;
        let err = outcome
            .expect("a blocked pump must not hold the call past its timeout")
            .expect_err("the wedged stand-in must time out");
        assert!(err.to_string().contains("timed out"), "{err}");
        assert!(
            started.elapsed() < Duration::from_secs(10),
            "took {:?}",
            started.elapsed()
        );
    }

    #[tokio::test]
    async fn a_caller_channel_is_passed_through_to_blender() {
        // Passthrough (`aware agent invoke` under FloLess): Blender writes to the CALLER's file
        // and nothing is relayed by us.
        let tmp = tempfile::tempdir().expect("tempdir");
        let blender = fake_blender(
            tmp.path(),
            r#"{"$aware-progress":{"phase":"render","done":1,"total":1}}"#,
            r#"{"$aware-progress":{"phase":"render","done":1,"total":1}}"#,
            0,
            0,
            r#"{"ok": true}"#,
        );
        let callers = tmp.path().join("callers-progress.jsonl");
        run_script(
            "exec",
            &blender,
            &tmp.path().join("exec.py"),
            &exec_args(),
            Duration::from_secs(60),
            BlenderProgress::Passthrough(callers.clone()),
            std::future::pending(),
        )
        .await
        .expect("passthrough run");
        let written = std::fs::read_to_string(&callers).expect("the caller's channel was written");
        assert_eq!(written.lines().count(), 2, "{written}");
    }

    // ── staged inputs ──

    #[test]
    fn staged_inputs_use_the_at_path_form_and_clean_up() {
        let path;
        {
            let scratch = ScratchInputs::write(&json!({ "ifc-path": "m.ifc" })).expect("staged");
            path = scratch.0.clone();
            assert!(path.is_file(), "the inputs file is staged");
            assert!(scratch.arg().starts_with('@'), "{}", scratch.arg());
            let text = std::fs::read_to_string(&path).expect("read");
            assert_eq!(
                serde_json::from_str::<Value>(&text).expect("json"),
                json!({ "ifc-path": "m.ifc" })
            );
        }
        assert!(!path.exists(), "the inputs file is removed on drop");
    }

    // ── end-to-end (opt-in) ──

    /// A real headless render through a real Blender. Ignored by default because CI has no Blender
    /// and no installed agent; run it locally with an installed agent and:
    ///
    /// ```text
    /// AWARE_HOME=<home with the blender agent installed> \
    ///   cargo test -p aware render::blender::tests::end_to_end -- --ignored --nocapture
    /// ```
    #[tokio::test]
    #[ignore = "requires a local Blender install and the blender agent installed under AWARE_HOME"]
    async fn end_to_end_scene_info_against_a_real_blender() {
        let ifc = std::env::var("AWARE_BLENDER_TEST_IFC")
            .expect("set AWARE_BLENDER_TEST_IFC to a fixture .ifc");
        let out = run_blender_command(
            "scene.info",
            &json!({ "ifc-path": ifc }),
            false,
            None,
            BlenderProgress::Off,
        )
        .await
        .expect("scene.info");
        assert!(out["count"].as_u64().unwrap_or(0) > 0, "{out}");
    }
}
