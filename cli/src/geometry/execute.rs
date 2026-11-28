use std::{
    io::Write,
    path::{Path, PathBuf},
    process::Command,
    time::Duration,
};

use super::{
    runtime,
    supervision::{WorkerError, supervise},
    wire::{Admission, Cut, NativeCandidate, Rejection, WarningCode, parse_child},
};
use WarningCode::*;

const NORMAL_WORKER: &str = include_str!("../../../geometry-worker/occt-worker.py");

pub fn worker_source() -> std::borrow::Cow<'static, str> {
    #[cfg(feature = "geometry-test-faults")]
    {
        format!(
            "_aware_fault_worker = True\n{NORMAL_WORKER}\n{}",
            include_str!("../../../geometry-worker/occt-test-faults.py")
        )
        .into()
    }
    #[cfg(not(feature = "geometry-test-faults"))]
    {
        NORMAL_WORKER.into()
    }
}

#[cfg(feature = "geometry-test-faults")]
pub fn test_trace(record: &serde_json::Value) {
    if let Some(path) = std::env::var_os("AWARE_GEOMETRY_TEST_TRACE")
        && let Ok(mut file) = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)
    {
        let _ = writeln!(file, "{record}");
    }
}
const BUDGET: Duration = Duration::from_secs(30);

pub struct Failure {
    pub rejection: Rejection,
    pub terminal: bool,
}

impl From<Rejection> for Failure {
    fn from(rejection: Rejection) -> Self {
        Self {
            terminal: rejection.0 == CoincidenceMismatch,
            rejection,
        }
    }
}

impl Failure {
    fn worker(error: WorkerError, exact: bool) -> Self {
        let code = match (&error, exact) {
            (WorkerError::Timeout, true) => OcctTimeout,
            (WorkerError::Timeout, false) => MeshTimeout,
            (WorkerError::OutputLimit, _) => BudgetExceeded,
            (_, true) => OcctWorkerFailed,
            (_, false) => MeshWorkerFailed,
        };
        Self {
            rejection: (code, None),
            terminal: error == WorkerError::Cleanup,
        }
    }
}

fn mesh_stage() -> Result<PathBuf, Failure> {
    let missing = || Failure::from((MeshWorkerFailed, None));
    let exe = std::env::current_exe().map_err(|_| missing())?;
    let stage = exe.parent().ok_or_else(missing)?.join("geometry-mesh");
    if !stage.join("node.exe").is_file() || !stage.join("geometry-mesh-dispatcher.mjs").is_file() {
        return Err(missing());
    }
    Ok(stage)
}

async fn mesh_worker(bytes: &[u8], mode: Option<&str>) -> Result<Vec<u8>, Failure> {
    let stage = mesh_stage()?;
    let mut command = Command::new(stage.join("node.exe"));
    runtime::configure_python(&mut command);
    command.current_dir(&stage);
    let dispatcher = stage.join("geometry-mesh-dispatcher.mjs");
    #[cfg(feature = "geometry-test-faults")]
    {
        let route = if mode == Some("--verify-exact") {
            "exact"
        } else {
            "mesh"
        };
        test_trace(&serde_json::json!({"event": "launch", "route": route, "mode": mode}));
        let fault_setting = std::env::var("AWARE_GEOMETRY_TEST_FAULT");
        if mode == Some("--preflight") && fault_setting.as_deref() == Ok("mesh:preflight-cleanup") {
            command.env("AWARE_GEOMETRY_INJECT_CLEANUP_FAILURE", "1");
        }
        if mode != Some("--preflight")
            && let Ok(setting) = fault_setting
            && let Some((target, fault)) = setting.split_once(':')
            && target == route
            && matches!(
                fault,
                "malformed"
                    | "nonzero"
                    | "unknown-field"
                    | "omitted-field"
                    | "envelope"
                    | "stdout"
                    | "stderr"
                    | "held-open"
                    | "blocked-stdin"
                    | "trace"
            )
            && let Some(trace) = std::env::var_os("AWARE_GEOMETRY_TEST_TRACE")
        {
            command
                .args([
                    "--input-type=module",
                    "-e",
                    include_str!("../../../geometry-worker/reply-test-faults.mjs"),
                    "--",
                    "geometry-reply-fault",
                ])
                .arg(&dispatcher)
                .args([route, fault])
                .arg(trace);
            let deadline = if matches!(fault, "held-open" | "blocked-stdin") {
                Duration::from_secs(2)
            } else {
                BUDGET
            };
            return supervise(command, bytes, deadline)
                .await
                .map_err(|error| Failure::worker(error, route == "exact"));
        }
    }
    command.arg(dispatcher).args(mode);
    supervise(command, bytes, BUDGET)
        .await
        .map_err(|error| Failure::worker(error, mode == Some("--verify-exact")))
}

pub async fn fallback(bytes: &[u8]) -> Result<Cut, Rejection> {
    let output = mesh_worker(bytes, None)
        .await
        .map_err(|failure| failure.rejection)?;
    parse_child(&output, false)
}

async fn native_worker(
    python: &Path,
    script: &Path,
    bytes: &[u8],
    mode: Option<&str>,
) -> Result<Vec<u8>, Failure> {
    let mut command = Command::new(python);
    runtime::configure_python(&mut command);
    command.arg("-I").arg(script).args(mode);
    #[cfg(feature = "geometry-test-faults")]
    {
        test_trace(&serde_json::json!({"event": "launch", "route": "native", "mode": mode}));
        for name in ["AWARE_GEOMETRY_TEST_FAULT", "AWARE_GEOMETRY_TEST_TRACE"] {
            if let Some(value) = std::env::var_os(name) {
                command.env(name, value);
            }
        }
    }
    supervise(command, bytes, BUDGET)
        .await
        .map_err(|error| Failure::worker(error, true))
}

fn require_admission(bytes: &[u8], exact: bool) -> Result<(), Failure> {
    if serde_json::from_slice::<Admission>(bytes).is_ok() {
        return Ok(());
    }
    match parse_child(bytes, exact) {
        Err(rejection) => Err(rejection.into()),
        Ok(_) => Err((
            if exact {
                OcctWorkerFailed
            } else {
                MeshWorkerFailed
            },
            None,
        )
            .into()),
    }
}

pub async fn exact(python: &Path, bytes: &[u8]) -> Result<Cut, Failure> {
    let failure = || Failure::from((OcctWorkerFailed, None));
    let mut script = tempfile::Builder::new()
        .suffix(".py")
        .tempfile()
        .map_err(|_| failure())?;
    script
        .write_all(worker_source().as_bytes())
        .map_err(|_| failure())?;
    script.flush().map_err(|_| failure())?;
    let script = script.into_temp_path();
    let reply = native_worker(python, &script, bytes, Some("--preflight")).await?;
    require_admission(&reply, true)?;
    let reply = mesh_worker(bytes, Some("--preflight")).await?;
    require_admission(&reply, false)?;
    let reply = native_worker(python, &script, bytes, None).await?;
    let candidate: NativeCandidate = match serde_json::from_slice(&reply) {
        Ok(candidate) => candidate,
        Err(_) => {
            return Err(parse_child(&reply, true)
                .err()
                .map_or_else(failure, Failure::from));
        }
    };
    let payload = candidate.verification_input(bytes)?;
    let reply = mesh_worker(&payload, Some("--verify-exact")).await?;
    parse_child(&reply, true).map_err(Failure::from)
}
