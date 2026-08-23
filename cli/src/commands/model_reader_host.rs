//! Internal managed provider host for `model-reference-reader`.
//!
//! The bridge never launches a commercial provider directly. This hidden command owns provider
//! processes, drains both output pipes concurrently, and keeps cancellation live over a bounded,
//! request-correlated binary protocol. It is not a public agent surface.

use std::collections::{BTreeMap, HashMap};
use std::path::PathBuf;
use std::process::Stdio;
use std::sync::Arc;

use fs2::FileExt;
use rand::RngCore;
use serde::Deserialize;
use serde_json::json;
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};
use tokio::sync::{Mutex, oneshot};

use crate::error::AwareError;

const HEADER_BYTES: usize = 50;
const MAX_CONTROL_BYTES: usize = 1024 * 1024;
const KIND_CONTROL: u8 = 0x01;
const KIND_STDOUT: u8 = 0x02;
const KIND_STDERR: u8 = 0x03;
const KIND_STDIN: u8 = 0x04;
const FINAL: u8 = 0x01;

#[derive(Clone, Debug, PartialEq, Eq)]
struct Frame {
    kind: u8,
    request_id: u64,
    run_handle: [u8; 32],
    sequence: u32,
    flags: u8,
    payload: Vec<u8>,
}

impl Frame {
    async fn read<R: AsyncRead + Unpin>(reader: &mut R) -> std::io::Result<Option<Self>> {
        let mut header = [0u8; HEADER_BYTES];
        match reader.read_exact(&mut header).await {
            Ok(_) => {}
            Err(error) if error.kind() == std::io::ErrorKind::UnexpectedEof => return Ok(None),
            Err(error) => return Err(error),
        }
        let kind = header[0];
        if !matches!(kind, KIND_CONTROL | KIND_STDOUT | KIND_STDERR | KIND_STDIN) {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "unknown frame kind",
            ));
        }
        let mut request_id_bytes = [0u8; 8];
        request_id_bytes.copy_from_slice(&header[1..9]);
        let request_id = u64::from_be_bytes(request_id_bytes);
        let mut run_handle = [0u8; 32];
        run_handle.copy_from_slice(&header[9..41]);
        let mut sequence_bytes = [0u8; 4];
        sequence_bytes.copy_from_slice(&header[41..45]);
        let sequence = u32::from_be_bytes(sequence_bytes);
        let flags = header[45];
        let mut length_bytes = [0u8; 4];
        length_bytes.copy_from_slice(&header[46..50]);
        let length = u32::from_be_bytes(length_bytes) as usize;
        if length > MAX_CONTROL_BYTES {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "frame payload exceeds limit",
            ));
        }
        let mut payload = vec![0u8; length];
        reader.read_exact(&mut payload).await?;
        Ok(Some(Self {
            kind,
            request_id,
            run_handle,
            sequence,
            flags,
            payload,
        }))
    }

    async fn write<W: AsyncWrite + Unpin>(&self, writer: &mut W) -> std::io::Result<()> {
        let length: u32 = self.payload.len().try_into().map_err(|_| {
            std::io::Error::new(std::io::ErrorKind::InvalidInput, "frame too large")
        })?;
        let mut header = [0u8; HEADER_BYTES];
        header[0] = self.kind;
        header[1..9].copy_from_slice(&self.request_id.to_be_bytes());
        header[9..41].copy_from_slice(&self.run_handle);
        header[41..45].copy_from_slice(&self.sequence.to_be_bytes());
        header[45] = self.flags;
        header[46..50].copy_from_slice(&length.to_be_bytes());
        writer.write_all(&header).await?;
        writer.write_all(&self.payload).await?;
        writer.flush().await
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct ProviderRun {
    op: String,
    executable: PathBuf,
    operation: String,
    cwd: PathBuf,
    environment: BTreeMap<String, String>,
    stdin_length: usize,
    timeout_ms: u64,
    stdout_limit: usize,
    stderr_limit: usize,
}

struct PendingRun {
    request: ProviderRun,
    handle: [u8; 32],
}

type SharedWriter = Arc<Mutex<tokio::io::Stdout>>;
type ActiveRuns = Arc<Mutex<HashMap<[u8; 32], oneshot::Sender<()>>>>;

async fn cancel_and_join(active: &ActiveRuns, tasks: &mut Vec<tokio::task::JoinHandle<()>>) {
    {
        let mut runs = active.lock().await;
        for (_, cancel) in runs.drain() {
            let _ = cancel.send(());
        }
    }
    for task in tasks.drain(..) {
        let _ = task.await;
    }
}

async fn send_control(
    writer: &SharedWriter,
    request_id: u64,
    handle: [u8; 32],
    body: serde_json::Value,
) {
    let frame = Frame {
        kind: KIND_CONTROL,
        request_id,
        run_handle: handle,
        sequence: 0,
        flags: FINAL,
        payload: serde_json::to_vec(&body).unwrap_or_default(),
    };
    let _ = frame.write(&mut *writer.lock().await).await;
}

async fn read_bounded<R: AsyncRead + Unpin>(
    mut reader: R,
    limit: usize,
) -> std::io::Result<Vec<u8>> {
    let mut bytes = Vec::new();
    let mut chunk = [0u8; 8192];
    loop {
        let count = reader.read(&mut chunk).await?;
        if count == 0 {
            return Ok(bytes);
        }
        if bytes.len().saturating_add(count) > limit {
            return Err(std::io::Error::new(
                std::io::ErrorKind::FileTooLarge,
                "provider stream exceeds limit",
            ));
        }
        bytes.extend_from_slice(&chunk[..count]);
    }
}

fn provider_command(request: &ProviderRun) -> Result<tokio::process::Command, AwareError> {
    if !request.executable.is_absolute()
        || !request.cwd.is_absolute()
        || !matches!(request.operation.as_str(), "describe" | "convert")
    {
        return Err(AwareError::Validation(
            "model-reader host received an unsafe provider request".into(),
        ));
    }
    let mut command = tokio::process::Command::new(&request.executable);
    command
        .arg(&request.operation)
        .arg("--json-stdin")
        .current_dir(&request.cwd)
        .env_clear()
        .envs(&request.environment)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .kill_on_drop(true);
    #[cfg(unix)]
    {
        use std::os::unix::process::CommandExt;
        command.as_std_mut().process_group(0);
    }
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        command
            .as_std_mut()
            .creation_flags(provider_creation_flags());
    }
    Ok(command)
}

#[cfg(windows)]
fn provider_creation_flags() -> u32 {
    windows_sys::Win32::System::Threading::CREATE_SUSPENDED
}

async fn kill_tree(child: &mut tokio::process::Child) {
    #[cfg(windows)]
    if let Some(pid) = child.id() {
        let _ = tokio::process::Command::new("taskkill")
            .args(["/PID", &pid.to_string(), "/T", "/F"])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .await;
    }
    #[cfg(unix)]
    if let Some(pid) = child.id() {
        let _ = tokio::process::Command::new("kill")
            .args(["-TERM", &format!("-{pid}")])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .await;
    }
    let _ = child.start_kill();
    let _ = child.wait().await;
}

#[cfg(windows)]
fn provider_job(child: &tokio::process::Child) -> Result<win32job::Job, AwareError> {
    let job = win32job::Job::create()
        .map_err(|error| AwareError::Internal(format!("create provider Job Object: {error}")))?;
    let mut limits = job
        .query_extended_limit_info()
        .map_err(|error| AwareError::Internal(format!("query provider Job Object: {error}")))?;
    limits.limit_kill_on_job_close();
    job.set_extended_limit_info(&limits)
        .map_err(|error| AwareError::Internal(format!("configure provider Job Object: {error}")))?;
    let handle = child
        .raw_handle()
        .ok_or_else(|| AwareError::Internal("provider process handle is unavailable".into()))?;
    job.assign_process(handle as isize)
        .map_err(|error| AwareError::Internal(format!("assign provider Job Object: {error}")))?;
    Ok(job)
}

#[cfg(windows)]
fn resume_provider(child: &tokio::process::Child) -> Result<(), AwareError> {
    use windows_sys::Win32::Foundation::{CloseHandle, INVALID_HANDLE_VALUE};
    use windows_sys::Win32::System::Diagnostics::ToolHelp::{
        CreateToolhelp32Snapshot, TH32CS_SNAPTHREAD, THREADENTRY32, Thread32First, Thread32Next,
    };
    use windows_sys::Win32::System::Threading::{OpenThread, ResumeThread, THREAD_SUSPEND_RESUME};

    let pid = child
        .id()
        .ok_or_else(|| AwareError::Internal("provider process id is unavailable".into()))?;
    // SAFETY: every handle is checked and closed on this path; THREADENTRY32 carries the documented
    // size, and the suspended process cannot create another thread before its primary one is resumed.
    unsafe {
        let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPTHREAD, 0);
        if snapshot == INVALID_HANDLE_VALUE {
            return Err(AwareError::Internal(
                "snapshot provider threads before resume failed".into(),
            ));
        }
        let mut entry = THREADENTRY32 {
            dwSize: std::mem::size_of::<THREADENTRY32>() as u32,
            ..Default::default()
        };
        let mut found = Thread32First(snapshot, &mut entry) != 0;
        let mut resumed = false;
        while found {
            if entry.th32OwnerProcessID == pid {
                let thread = OpenThread(THREAD_SUSPEND_RESUME, 0, entry.th32ThreadID);
                if !thread.is_null() {
                    resumed = ResumeThread(thread) != u32::MAX;
                    let _ = CloseHandle(thread);
                    break;
                }
            }
            found = Thread32Next(snapshot, &mut entry) != 0;
        }
        let _ = CloseHandle(snapshot);
        if !resumed {
            return Err(AwareError::Internal(
                "resume contained provider thread failed".into(),
            ));
        }
    }
    Ok(())
}

async fn join_bounded_stream(
    task: tokio::task::JoinHandle<std::io::Result<Vec<u8>>>,
) -> std::io::Result<Vec<u8>> {
    task.await
        .map_err(|error| std::io::Error::other(format!("provider stream task failed: {error}")))?
}

#[cfg(unix)]
async fn terminate_provider_group(pid: Option<u32>) {
    if let Some(pid) = pid {
        let _ = tokio::process::Command::new("kill")
            .args(["-KILL", &format!("-{pid}")])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .await;
    }
}

async fn execute_provider(
    request_id: u64,
    handle: [u8; 32],
    request: ProviderRun,
    stdin_bytes: Vec<u8>,
    writer: SharedWriter,
    active: ActiveRuns,
    cancel_rx: oneshot::Receiver<()>,
) {
    let mut child = match provider_command(&request)
        .and_then(|mut command| command.spawn().map_err(AwareError::Io))
    {
        Ok(child) => child,
        Err(error) => {
            send_control(
                &writer,
                request_id,
                handle,
                json!({"status":"complete","exitCode":127,"hostError":error.to_string()}),
            )
            .await;
            active.lock().await.remove(&handle);
            return;
        }
    };
    #[cfg(unix)]
    let provider_pid = child.id();
    #[cfg(windows)]
    let provider_job = match provider_job(&child) {
        Ok(job) => job,
        Err(error) => {
            kill_tree(&mut child).await;
            send_control(
                &writer,
                request_id,
                handle,
                json!({"status":"complete","exitCode":127,"hostError":error.to_string()}),
            )
            .await;
            active.lock().await.remove(&handle);
            return;
        }
    };
    #[cfg(windows)]
    if let Err(error) = resume_provider(&child) {
        drop(provider_job);
        kill_tree(&mut child).await;
        send_control(
            &writer,
            request_id,
            handle,
            json!({"status":"complete","exitCode":127,"hostError":error.to_string()}),
        )
        .await;
        active.lock().await.remove(&handle);
        return;
    }
    let mut stdin = child.stdin.take();
    let stdout = child.stdout.take();
    let stderr = child.stderr.take();
    let input = tokio::spawn(async move {
        if let Some(mut stream) = stdin.take() {
            stream.write_all(&stdin_bytes).await?;
            stream.shutdown().await?;
        }
        Ok::<(), std::io::Error>(())
    });
    let (Some(stdout), Some(stderr)) = (stdout, stderr) else {
        kill_tree(&mut child).await;
        send_control(
            &writer,
            request_id,
            handle,
            json!({"status":"complete","exitCode":127,"hostError":"provider pipes unavailable"}),
        )
        .await;
        active.lock().await.remove(&handle);
        return;
    };
    let stdout_task = tokio::spawn(read_bounded(stdout, request.stdout_limit));
    let stderr_task = tokio::spawn(read_bounded(stderr, request.stderr_limit));
    let mut cancel_rx = cancel_rx;
    let outcome = tokio::select! {
        status = child.wait() => status.map(|status| (status.code().unwrap_or(1), None)),
        _ = tokio::time::sleep(std::time::Duration::from_millis(request.timeout_ms)) => { kill_tree(&mut child).await; Ok((124, Some("timeout"))) },
        _ = &mut cancel_rx => { kill_tree(&mut child).await; Ok((130, Some("cancelled"))) },
    };
    // A successful provider parent is not sufficient: descendants may still own output handles or
    // mutate the private directory. Closing the Windows Job kills the complete nested tree; Unix
    // providers run in their own process group, which is force-terminated here. Only then may pipe
    // drains finish and output validation begin.
    #[cfg(windows)]
    drop(provider_job);
    #[cfg(unix)]
    terminate_provider_group(provider_pid).await;
    let _ = input.await;
    let stdout = join_bounded_stream(stdout_task).await;
    let stderr = join_bounded_stream(stderr_task).await;
    let stream_failure = stdout.as_ref().err().or_else(|| stderr.as_ref().err());
    let stream_failure_code = stream_failure.map(|error| {
        if error.kind() == std::io::ErrorKind::FileTooLarge {
            "reference-provider-output-limit"
        } else {
            "reference-provider-host-failed"
        }
    });
    let stdout = stdout.unwrap_or_default();
    let stderr = stderr.unwrap_or_default();
    for (kind, payload) in [(KIND_STDOUT, stdout), (KIND_STDERR, stderr)] {
        let frame = Frame {
            kind,
            request_id,
            run_handle: handle,
            sequence: 0,
            flags: FINAL,
            payload,
        };
        let _ = frame.write(&mut *writer.lock().await).await;
    }
    let (mut exit_code, mut reason) = outcome.unwrap_or((1, Some("host-wait-failed")));
    if stream_failure_code.is_some() {
        exit_code = 1;
        reason = Some("provider-stream-failed");
    }
    send_control(
        &writer,
        request_id,
        handle,
        json!({"status":"complete","exitCode":exit_code,"reason":reason,"hostErrorCode":stream_failure_code}),
    )
    .await;
    active.lock().await.remove(&handle);
}

async fn protocol_loop<R: AsyncRead + Unpin>(
    reader: &mut R,
    writer: SharedWriter,
    active: ActiveRuns,
    provider_tasks: &mut Vec<tokio::task::JoinHandle<()>>,
) -> Result<(), AwareError> {
    let mut pending: HashMap<(u64, [u8; 32]), PendingRun> = HashMap::new();
    let mut locks: HashMap<[u8; 32], std::fs::File> = HashMap::new();
    let mut last_control_id = 0u64;
    while let Some(frame) = Frame::read(&mut *reader).await? {
        if frame.kind == KIND_STDIN {
            let Some(pending_run) = pending.remove(&(frame.request_id, frame.run_handle)) else {
                return Err(AwareError::Validation(
                    "model-reader host received uncorrelated stdin".into(),
                ));
            };
            if frame.sequence != 0
                || frame.flags & FINAL == 0
                || frame.payload.len() != pending_run.request.stdin_length
            {
                return Err(AwareError::Validation(
                    "model-reader host stdin length/sequence mismatch".into(),
                ));
            }
            let (cancel_tx, cancel_rx) = oneshot::channel();
            active.lock().await.insert(pending_run.handle, cancel_tx);
            provider_tasks.retain(|task| !task.is_finished());
            provider_tasks.push(tokio::spawn(execute_provider(
                frame.request_id,
                pending_run.handle,
                pending_run.request,
                frame.payload,
                writer.clone(),
                active.clone(),
                cancel_rx,
            )));
            continue;
        }
        if frame.kind != KIND_CONTROL || frame.request_id <= last_control_id {
            return Err(AwareError::Validation(
                "model-reader host control request ids must increase".into(),
            ));
        }
        last_control_id = frame.request_id;
        let control: serde_json::Value = serde_json::from_slice(&frame.payload)?;
        match control.get("op").and_then(|value| value.as_str()) {
            Some("hello") => send_control(&writer, frame.request_id, [0; 32], json!({"status":"ok","protocol":"model-reader-host/v1","build":env!("CARGO_PKG_VERSION")})).await,
            Some("provider-run") => {
                let request: ProviderRun = serde_json::from_value(control)?;
                if request.op != "provider-run" || request.stdin_length > MAX_CONTROL_BYTES { return Err(AwareError::Validation("model-reader host provider request exceeds limit".into())); }
                let mut handle = [0u8; 32]; rand::thread_rng().fill_bytes(&mut handle);
                pending.insert((frame.request_id, handle), PendingRun { request, handle });
                send_control(&writer, frame.request_id, handle, json!({"status":"accepted"})).await;
            }
            Some("provider-cancel") => {
                if let Some(cancel) = active.lock().await.remove(&frame.run_handle) { let _ = cancel.send(()); }
                send_control(&writer, frame.request_id, frame.run_handle, json!({"status":"cancel-requested"})).await;
            }
            Some("lock-acquire") => {
                let lock_path = control.get("path").and_then(|value| value.as_str()).map(PathBuf::from).filter(|value| value.is_absolute()).ok_or_else(|| AwareError::Validation("model-reader host lock path must be absolute".into()))?;
                let file = std::fs::OpenOptions::new().read(true).write(true).create(true).truncate(false).open(lock_path)?;
                if file.try_lock_exclusive().is_err() {
                    send_control(&writer, frame.request_id, [0; 32], json!({"status":"busy"})).await;
                } else {
                    let mut handle = [0u8; 32]; rand::thread_rng().fill_bytes(&mut handle); locks.insert(handle, file);
                    send_control(&writer, frame.request_id, handle, json!({"status":"acquired"})).await;
                }
            }
            Some("lock-release") => {
                let file = locks.remove(&frame.run_handle).ok_or_else(|| AwareError::Validation("model-reader host lock handle is unknown".into()))?;
                file.unlock()?; send_control(&writer, frame.request_id, frame.run_handle, json!({"status":"released"})).await;
            }
            Some("shutdown") => {
                cancel_and_join(&active, provider_tasks).await;
                send_control(&writer, frame.request_id, [0; 32], json!({"status":"bye"})).await; break;
            }
            _ => return Err(AwareError::Validation("model-reader host control operation is unknown".into())),
        }
    }
    Ok(())
}

#[cfg(unix)]
async fn termination_signal() {
    use tokio::signal::unix::{SignalKind, signal};
    let mut terminate = signal(SignalKind::terminate()).expect("install SIGTERM handler");
    tokio::select! {
        _ = tokio::signal::ctrl_c() => {}
        _ = terminate.recv() => {}
    }
}

#[cfg(not(unix))]
async fn termination_signal() {
    let _ = tokio::signal::ctrl_c().await;
}

async fn supervise_protocol<R, S>(
    reader: &mut R,
    writer: SharedWriter,
    active: ActiveRuns,
    provider_tasks: &mut Vec<tokio::task::JoinHandle<()>>,
    shutdown: S,
) -> Result<(), AwareError>
where
    R: AsyncRead + Unpin,
    S: std::future::Future<Output = ()>,
{
    tokio::pin!(shutdown);
    let result = tokio::select! {
        result = protocol_loop(reader, writer, active.clone(), provider_tasks) => result,
        _ = &mut shutdown => Ok(()),
    };
    cancel_and_join(&active, provider_tasks).await;
    result
}

pub async fn run() -> Result<(), AwareError> {
    let mut reader = tokio::io::BufReader::new(tokio::io::stdin());
    let writer: SharedWriter = Arc::new(Mutex::new(tokio::io::stdout()));
    let active: ActiveRuns = Arc::new(Mutex::new(HashMap::new()));
    let mut provider_tasks: Vec<tokio::task::JoinHandle<()>> = Vec::new();
    supervise_protocol(
        &mut reader,
        writer,
        active,
        &mut provider_tasks,
        termination_signal(),
    )
    .await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn frames_preserve_request_run_sequence_final_and_binary_payload() {
        let frame = Frame {
            kind: KIND_STDOUT,
            request_id: 42,
            run_handle: [7; 32],
            sequence: 9,
            flags: FINAL,
            payload: vec![0, 255, 1, 2],
        };
        let mut bytes = Vec::new();
        frame.write(&mut bytes).await.unwrap();
        let decoded = Frame::read(&mut bytes.as_slice()).await.unwrap().unwrap();
        assert_eq!(decoded, frame);
    }

    #[test]
    fn provider_command_has_only_protocol_argv_and_clears_ambient_environment() {
        let request = ProviderRun {
            op: "provider-run".into(),
            executable: std::env::current_exe().unwrap(),
            operation: "convert".into(),
            cwd: std::env::current_dir().unwrap(),
            environment: BTreeMap::from([("TZ".into(), "UTC".into())]),
            stdin_length: 1,
            timeout_ms: 1000,
            stdout_limit: 10,
            stderr_limit: 10,
        };
        let command = provider_command(&request).unwrap();
        let debug = format!("{command:?}");
        assert!(debug.contains("convert"));
        assert!(debug.contains("--json-stdin"));
        assert!(!debug.contains("secret.rvt"));
    }

    #[cfg(windows)]
    #[test]
    fn provider_is_created_suspended_until_its_job_is_attached() {
        assert_eq!(
            provider_creation_flags() & windows_sys::Win32::System::Threading::CREATE_SUSPENDED,
            windows_sys::Win32::System::Threading::CREATE_SUSPENDED
        );
    }

    #[tokio::test]
    async fn bounded_stream_failure_is_preserved_for_the_completion_frame() {
        let task = tokio::spawn(async {
            Err::<Vec<u8>, std::io::Error>(std::io::Error::new(
                std::io::ErrorKind::FileTooLarge,
                "provider stream exceeds limit",
            ))
        });
        let error = join_bounded_stream(task).await.unwrap_err();
        assert_eq!(error.kind(), std::io::ErrorKind::FileTooLarge);
    }

    #[tokio::test]
    async fn host_shutdown_waits_for_cancelled_provider_cleanup() {
        let active: ActiveRuns = Arc::new(Mutex::new(HashMap::new()));
        let (cancel_tx, cancel_rx) = oneshot::channel();
        active.lock().await.insert([7; 32], cancel_tx);
        let cleaned = Arc::new(std::sync::atomic::AtomicBool::new(false));
        let observed = cleaned.clone();
        let task = tokio::spawn(async move {
            let _ = cancel_rx.await;
            observed.store(true, std::sync::atomic::Ordering::SeqCst);
        });
        let mut tasks = vec![task];
        cancel_and_join(&active, &mut tasks).await;
        assert!(cleaned.load(std::sync::atomic::Ordering::SeqCst));
        assert!(tasks.is_empty());
    }

    #[tokio::test]
    async fn protocol_errors_and_termination_signals_always_cleanup_provider_tasks() {
        async fn active_provider() -> (
            ActiveRuns,
            Vec<tokio::task::JoinHandle<()>>,
            Arc<std::sync::atomic::AtomicBool>,
        ) {
            let active: ActiveRuns = Arc::new(Mutex::new(HashMap::new()));
            let (cancel_tx, cancel_rx) = oneshot::channel();
            active.lock().await.insert([7; 32], cancel_tx);
            let cleaned = Arc::new(std::sync::atomic::AtomicBool::new(false));
            let observed = cleaned.clone();
            let task = tokio::spawn(async move {
                let _ = cancel_rx.await;
                observed.store(true, std::sync::atomic::Ordering::SeqCst);
            });
            (active, vec![task], cleaned)
        }

        let invalid = Frame {
            kind: KIND_STDIN,
            request_id: 1,
            run_handle: [9; 32],
            sequence: 0,
            flags: FINAL,
            payload: Vec::new(),
        };
        let mut bytes = Vec::new();
        invalid.write(&mut bytes).await.unwrap();
        let (active, mut tasks, cleaned) = active_provider().await;
        let writer: SharedWriter = Arc::new(Mutex::new(tokio::io::stdout()));
        let result = supervise_protocol(
            &mut bytes.as_slice(),
            writer,
            active,
            &mut tasks,
            std::future::pending(),
        )
        .await;
        assert!(result.is_err());
        assert!(cleaned.load(std::sync::atomic::Ordering::SeqCst));

        let (_input, mut reader) = tokio::io::duplex(64);
        let (active, mut tasks, cleaned) = active_provider().await;
        let writer: SharedWriter = Arc::new(Mutex::new(tokio::io::stdout()));
        supervise_protocol(
            &mut reader,
            writer,
            active,
            &mut tasks,
            std::future::ready(()),
        )
        .await
        .unwrap();
        assert!(cleaned.load(std::sync::atomic::Ordering::SeqCst));
    }
}
