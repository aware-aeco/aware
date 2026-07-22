//! `blender.*` — the builtin transport for the headless Blender visualization agent.
//!
//! The `blender` agent is IFC in, rendered PNG / MP4 out, fully unattended. The work itself lives in
//! five `bpy` scripts shipped with the agent (`<AWARE_HOME>/agents/blender/scripts/`); this module is
//! the launcher that runs them:
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
//! - **Timeout** → per-call configurable via the `timeout-seconds` input; the process (and on
//!   Windows its whole tree) is killed so a wedged Blender is never orphaned.
//!
//! Both are [`AwareError::Network`] — the CLI's "agent runtime" class (exit 4 in `cli-spec.md`),
//! which is what `CliInvoker` already uses for a transport binary that will not spawn.

use crate::error::AwareError;
use serde_json::{Map, Value};
use std::path::{Path, PathBuf};
use std::time::Duration;

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
        "render.still" | "render.turntable" => 1_800,
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

/// Look a bare command name up on `PATH` (appending the Windows executable extensions).
fn find_on_path(name: &str) -> Option<PathBuf> {
    let path = std::env::var_os("PATH")?;
    let exts: &[&str] = if cfg!(windows) {
        &[".exe", ".cmd", ".bat"]
    } else {
        &[""]
    };
    for dir in std::env::split_paths(&path) {
        for ext in exts {
            let candidate = dir.join(format!("{name}{ext}"));
            if candidate.is_file() {
                return Some(candidate);
            }
        }
    }
    None
}

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

/// The installed agent's `scripts/` directory. `BuiltinInvoker` carries only `dry_run`, so the path
/// is resolved here from the same env-driven source the rest of the CLI uses (mirroring
/// `invoker::bridges_dir`). A missing directory is an *installation* error, not a path error.
fn scripts_dir() -> Result<PathBuf, AwareError> {
    let dir = crate::paths::Paths::from_env()?
        .agents_dir()
        .join("blender")
        .join("scripts");
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
fn script_path(command: &str) -> Result<PathBuf, AwareError> {
    let script = script_for_command(command).ok_or_else(|| {
        AwareError::Validation(format!(
            "blender: no command `{command}` (available: {})",
            COMMANDS.join(", ")
        ))
    })?;
    let path = scripts_dir()?.join(script);
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

/// Resolve a (possibly relative) path to an absolute string for the output contract — same helper
/// shape as `render::file::abs_path`; never resolves symlinks or requires existence.
fn abs_path(path: &str) -> String {
    std::path::absolute(path)
        .map(|p| p.to_string_lossy().into_owned())
        .unwrap_or_else(|_| path.to_string())
}

/// A required, non-empty string input, named the way the script would name it.
fn require_str(command: &str, args: &Value, key: &str) -> Result<String, AwareError> {
    match args.get(key).and_then(Value::as_str).map(str::trim) {
        Some(v) if !v.is_empty() => Ok(v.to_string()),
        _ => Err(AwareError::Validation(format!(
            "blender {command}: required input `{key}` is missing"
        ))),
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

/// Kill Blender and (on Windows) its whole process tree, then reap it. `taskkill /T /F` mirrors
/// `invoker::kill_process_tree`: `Child::kill` alone can leave a grandchild running, and a leaked
/// headless Blender holds a GPU context and the output file open.
async fn kill_blender(child: &mut tokio::process::Child) {
    #[cfg(windows)]
    if let Some(pid) = child.id() {
        let _ = tokio::process::Command::new("taskkill")
            .args(["/PID", &pid.to_string(), "/T", "/F"])
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status()
            .await;
    }
    let _ = child.start_kill();
    let _ = child.wait().await;
}

/// Run one `blender` agent command: spawn headless Blender on the command's script, recover the
/// framed payload, and surface the script's named errors intact.
///
/// This is the entry point `BuiltinInvoker::invoke_single` dispatches `("blender", …)` to.
pub async fn run_blender_command(
    command: &str,
    args: &Value,
    dry_run: bool,
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
    if dry_run {
        return dry_run_result(command, args);
    }

    let script = script_path(command)?;
    let blender = find_blender()?;
    let inputs = ScratchInputs::write(args)?;

    // Exactly the invocation Phase 1 verified against Blender 5.2 — no extra flags (not even
    // `--factory-startup`), because deviating from the proven command line is how a working script
    // starts failing for reasons that have nothing to do with this transport.
    let mut child = tokio::process::Command::new(&blender)
        .arg("-b")
        .arg("-P")
        .arg(&script)
        .arg("--")
        .arg(inputs.arg())
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .map_err(|e| {
            AwareError::Network(format!(
                "blender {command}: cannot spawn {}: {e}\n  hint: point {BLENDER_ENV} at a \
                 working Blender executable",
                blender.display()
            ))
        })?;

    // Drain both pipes concurrently with the wait: Blender is chatty enough to fill a pipe buffer
    // and deadlock against a wait-then-read.
    let stdout_task = tokio::spawn(read_to_end(child.stdout.take()));
    let stderr_task = tokio::spawn(read_to_end(child.stderr.take()));

    let status = match tokio::time::timeout(budget, child.wait()).await {
        Ok(status) => status.map_err(|e| {
            AwareError::Network(format!("blender {command}: waiting on Blender failed: {e}"))
        })?,
        Err(_) => {
            // The design doc's other required named error. Kill first, report second — a wedged
            // Blender must not outlive the call.
            kill_blender(&mut child).await;
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

    let stdout = String::from_utf8_lossy(&stdout_task.await.unwrap_or_default()).into_owned();
    let stderr = String::from_utf8_lossy(&stderr_task.await.unwrap_or_default()).into_owned();

    // The sentinel — NOT the exit code — is the signal: Blender can exit 0 having never run the
    // script at all.
    let Some(body) = extract_framed_payload(&stdout) else {
        return Err(no_result_error(
            command,
            &blender,
            &script,
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
        let err = run_blender_command("render.video", &json!({}), false)
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
        let out = run_blender_command("scene.info", &json!({ "ifc-path": "m.ifc" }), true)
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
            let err = run_blender_command(command, &args, true)
                .await
                .expect_err("preview must not mask a miswired node");
            assert!(matches!(err, AwareError::Validation(_)), "{err}");
        }
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
        let out = run_blender_command("scene.info", &json!({ "ifc-path": ifc }), false)
            .await
            .expect("scene.info");
        assert!(out["count"].as_u64().unwrap_or(0) > 0, "{out}");
    }
}
