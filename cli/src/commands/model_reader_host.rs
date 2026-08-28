//! Internal managed provider host for `model-reference-reader`.
//!
//! The bridge never launches a commercial provider directly. This hidden command owns provider
//! processes, drains both output pipes concurrently, and keeps cancellation live over a bounded,
//! request-correlated binary protocol. It is not a public agent surface.

use std::collections::{BTreeMap, HashMap};
use std::io::{Read, Seek, SeekFrom};
use std::path::PathBuf;
use std::process::Stdio;
use std::sync::Arc;

use fs2::FileExt;
use rand::RngCore;
use serde::Deserialize;
use serde_json::json;
use sha2::{Digest, Sha256};
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};
use tokio::sync::{Mutex, mpsc, oneshot};

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
    executable_sha256: String,
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

async fn send_empty_streams(writer: &SharedWriter, request_id: u64, handle: [u8; 32]) {
    for kind in [KIND_STDOUT, KIND_STDERR] {
        let frame = Frame {
            kind,
            request_id,
            run_handle: handle,
            sequence: 0,
            flags: FINAL,
            payload: Vec::new(),
        };
        let _ = frame.write(&mut *writer.lock().await).await;
    }
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

#[derive(Clone, Copy)]
enum ProviderIoFailure {
    Stdin,
    Stream,
}

async fn read_bounded_supervised<R: AsyncRead + Unpin>(
    reader: R,
    limit: usize,
    failures: mpsc::UnboundedSender<ProviderIoFailure>,
) -> std::io::Result<Vec<u8>> {
    let result = read_bounded(reader, limit).await;
    if result.is_err() {
        let _ = failures.send(ProviderIoFailure::Stream);
    }
    result
}

async fn write_input_supervised<W: AsyncWrite + Unpin>(
    mut stream: W,
    bytes: Vec<u8>,
    failures: mpsc::UnboundedSender<ProviderIoFailure>,
) -> std::io::Result<()> {
    let result = async {
        stream.write_all(&bytes).await?;
        stream.shutdown().await?;
        Ok(())
    }
    .await;
    if result.is_err() {
        let _ = failures.send(ProviderIoFailure::Stdin);
    }
    result
}

struct PreparedProvider {
    command: tokio::process::Command,
    image: std::fs::File,
}

fn provider_command(request: &ProviderRun) -> Result<PreparedProvider, AwareError> {
    if !request.executable.is_absolute()
        || !request.cwd.is_absolute()
        || !matches!(request.operation.as_str(), "describe" | "convert")
        || request.executable_sha256.len() != 64
        || !request
            .executable_sha256
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    {
        return Err(AwareError::Validation(
            "model-reader host received an unsafe provider request".into(),
        ));
    }
    let link_metadata = std::fs::symlink_metadata(&request.executable)?;
    if !link_metadata.is_file() || link_metadata.file_type().is_symlink() {
        return Err(AwareError::Validation(
            "model-reader host provider image must be a regular non-link file".into(),
        ));
    }
    let mut options = std::fs::OpenOptions::new();
    options.read(true);
    #[cfg(windows)]
    {
        use std::os::windows::fs::OpenOptionsExt;
        use windows_sys::Win32::Storage::FileSystem::FILE_SHARE_READ;
        // Denying write/delete sharing keeps the verified pathname bound until CreateProcess has
        // mapped the suspended image. The file remains open through spawn below.
        options.share_mode(FILE_SHARE_READ);
    }
    let mut image = options.open(&request.executable)?;
    if !image.metadata()?.is_file() {
        return Err(AwareError::Validation(
            "model-reader host provider image must be a regular file".into(),
        ));
    }
    let mut hasher = Sha256::new();
    let mut chunk = [0u8; 64 * 1024];
    loop {
        let count = image.read(&mut chunk)?;
        if count == 0 {
            break;
        }
        hasher.update(&chunk[..count]);
    }
    let actual_sha256 = format!("{:x}", hasher.finalize());
    if actual_sha256 != request.executable_sha256 {
        return Err(AwareError::Validation(
            "model-reader host provider image does not match its expected digest".into(),
        ));
    }
    image.seek(SeekFrom::Start(0))?;

    #[cfg(windows)]
    let launch_path = request.executable.clone();
    #[cfg(target_os = "linux")]
    let launch_path = {
        use std::os::fd::AsRawFd;
        PathBuf::from(format!("/proc/self/fd/{}", image.as_raw_fd()))
    };
    #[cfg(all(unix, not(target_os = "linux")))]
    let launch_path = {
        use std::os::fd::AsRawFd;
        PathBuf::from(format!("/dev/fd/{}", image.as_raw_fd()))
    };

    let mut command = tokio::process::Command::new(launch_path);
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
    Ok(PreparedProvider { command, image })
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
async fn terminate_provider_group(pid: Option<u32>) -> Result<(), AwareError> {
    if let Some(pid) = pid {
        let _ = tokio::process::Command::new("kill")
            .args(["-KILL", &format!("-{pid}")])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .await;
        let deadline = tokio::time::Instant::now() + std::time::Duration::from_secs(5);
        loop {
            let alive = tokio::process::Command::new("kill")
                .args(["-0", &format!("-{pid}")])
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .status()
                .await
                .map(|status| status.success())
                .unwrap_or(false);
            if !alive {
                break;
            }
            if tokio::time::Instant::now() >= deadline {
                return Err(AwareError::Internal(
                    "provider process group did not terminate".into(),
                ));
            }
            tokio::time::sleep(std::time::Duration::from_millis(10)).await;
        }
    }
    Ok(())
}

#[cfg(windows)]
async fn terminate_provider_job(job: win32job::Job) -> Result<(), AwareError> {
    tokio::task::spawn_blocking(move || {
        use windows_sys::Win32::Foundation::{HANDLE, WAIT_OBJECT_0};
        use windows_sys::Win32::System::JobObjects::TerminateJobObject;
        use windows_sys::Win32::System::Threading::WaitForSingleObject;
        let handle = job.handle() as HANDLE;
        // SAFETY: the Job owns `handle` for this closure's lifetime. Termination is initiated while
        // the handle is retained, then the signaled state proves the Job has zero active processes.
        let (terminated, wait_result) = unsafe {
            let terminated = TerminateJobObject(handle, 1);
            let wait_result = if terminated == 0 {
                WAIT_OBJECT_0
            } else {
                WaitForSingleObject(handle, 5_000)
            };
            (terminated, wait_result)
        };
        if terminated == 0 {
            return Err(AwareError::Internal(
                "terminate provider Job Object failed".into(),
            ));
        }
        if wait_result != WAIT_OBJECT_0 {
            return Err(AwareError::Internal(
                "provider Job Object did not reach zero active processes".into(),
            ));
        }
        Ok(())
    })
    .await
    .map_err(|error| AwareError::Internal(format!("provider Job waiter failed: {error}")))?
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
    let prepared = match provider_command(&request) {
        Ok(prepared) => prepared,
        Err(error) => {
            send_empty_streams(&writer, request_id, handle).await;
            send_control(
                &writer,
                request_id,
                handle,
                json!({"status":"complete","exitCode":127,"hostError":error.to_string(),"hostErrorCode":"reference-provider-executable-mismatch"}),
            )
            .await;
            active.lock().await.remove(&handle);
            return;
        }
    };
    let PreparedProvider { mut command, image } = prepared;
    let mut child = match command.spawn() {
        Ok(child) => child,
        Err(error) => {
            drop(image);
            send_empty_streams(&writer, request_id, handle).await;
            send_control(
                &writer,
                request_id,
                handle,
                json!({"status":"complete","exitCode":127,"hostError":error.to_string(),"hostErrorCode":"reference-provider-host-failed"}),
            )
            .await;
            active.lock().await.remove(&handle);
            return;
        }
    };
    // Windows has mapped the still-suspended image; Unix exec resolved the descriptor-backed path.
    // Only now can the verification handle be released without reopening the pathname for launch.
    drop(image);
    #[cfg(unix)]
    let provider_pid = child.id();
    #[cfg(windows)]
    let provider_job = match provider_job(&child) {
        Ok(job) => job,
        Err(error) => {
            kill_tree(&mut child).await;
            send_empty_streams(&writer, request_id, handle).await;
            send_control(
                &writer,
                request_id,
                handle,
                json!({"status":"complete","exitCode":127,"hostError":error.to_string(),"hostErrorCode":"reference-provider-host-failed"}),
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
        send_empty_streams(&writer, request_id, handle).await;
        send_control(
            &writer,
            request_id,
            handle,
            json!({"status":"complete","exitCode":127,"hostError":error.to_string(),"hostErrorCode":"reference-provider-host-failed"}),
        )
        .await;
        active.lock().await.remove(&handle);
        return;
    }
    let stdin = child.stdin.take();
    let stdout = child.stdout.take();
    let stderr = child.stderr.take();
    let (Some(stdin), Some(stdout), Some(stderr)) = (stdin, stdout, stderr) else {
        kill_tree(&mut child).await;
        send_empty_streams(&writer, request_id, handle).await;
        send_control(
            &writer,
            request_id,
            handle,
            json!({"status":"complete","exitCode":127,"hostError":"provider pipes unavailable","hostErrorCode":"reference-provider-host-failed"}),
        )
        .await;
        active.lock().await.remove(&handle);
        return;
    };
    let (io_failure_tx, mut io_failure_rx) = mpsc::unbounded_channel();
    let input = tokio::spawn(write_input_supervised(
        stdin,
        stdin_bytes,
        io_failure_tx.clone(),
    ));
    let stdout_task = tokio::spawn(read_bounded_supervised(
        stdout,
        request.stdout_limit,
        io_failure_tx.clone(),
    ));
    let stderr_task = tokio::spawn(read_bounded_supervised(
        stderr,
        request.stderr_limit,
        io_failure_tx.clone(),
    ));
    drop(io_failure_tx);
    let mut cancel_rx = cancel_rx;
    let outcome = tokio::select! {
        biased;
        Some(failure) = io_failure_rx.recv() => {
            kill_tree(&mut child).await;
            let reason = match failure {
                ProviderIoFailure::Stdin => "provider-stdin-failed",
                ProviderIoFailure::Stream => "provider-stream-failed",
            };
            Ok((1, Some(reason)))
        },
        status = child.wait() => status.map(|status| (status.code().unwrap_or(1), None)),
        _ = tokio::time::sleep(std::time::Duration::from_millis(request.timeout_ms)) => { kill_tree(&mut child).await; Ok((124, Some("timeout"))) },
        _ = &mut cancel_rx => { kill_tree(&mut child).await; Ok((130, Some("cancelled"))) },
    };
    // A successful provider parent is not sufficient: descendants may still own output handles or
    // mutate the private directory. Closing the Windows Job kills the complete nested tree; Unix
    // providers run in their own process group, which is force-terminated here. Only then may pipe
    // drains finish and output validation begin.
    #[cfg(windows)]
    let containment = terminate_provider_job(provider_job).await;
    #[cfg(unix)]
    let containment = terminate_provider_group(provider_pid).await;
    let input_failure = match input.await {
        Ok(Ok(())) => None,
        Ok(Err(error)) => Some(error),
        Err(error) => Some(std::io::Error::other(format!(
            "provider stdin task failed: {error}"
        ))),
    };
    let stdout = join_bounded_stream(stdout_task).await;
    let stderr = join_bounded_stream(stderr_task).await;
    let stream_failure = stdout.as_ref().err().or_else(|| stderr.as_ref().err());
    let mut host_failure_code = if containment.is_err() {
        Some("reference-provider-host-failed")
    } else {
        stream_failure.map(|error| {
            if error.kind() == std::io::ErrorKind::FileTooLarge {
                "reference-provider-output-limit"
            } else {
                "reference-provider-host-failed"
            }
        })
    };
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
    if host_failure_code.is_none() && input_failure.is_some() {
        host_failure_code = Some("reference-provider-host-failed");
        exit_code = 1;
        reason = Some("provider-stdin-failed");
    }
    if host_failure_code.is_some() {
        exit_code = 1;
        reason.get_or_insert("provider-io-failed");
    }
    send_control(
        &writer,
        request_id,
        handle,
        json!({"status":"complete","exitCode":exit_code,"reason":reason,"hostErrorCode":host_failure_code}),
    )
    .await;
    active.lock().await.remove(&handle);
}

async fn protocol_loop<R: AsyncRead + Unpin>(
    reader: &mut R,
    writer: SharedWriter,
    active: ActiveRuns,
    provider_tasks: &mut Vec<tokio::task::JoinHandle<()>>,
) -> Result<Option<u64>, AwareError> {
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
                return Ok(Some(frame.request_id));
            }
            _ => return Err(AwareError::Validation("model-reader host control operation is unknown".into())),
        }
    }
    Ok(None)
}

#[cfg(unix)]
async fn termination_signal() {
    use tokio::signal::unix::{SignalKind, signal};
    let Ok(mut terminate) = signal(SignalKind::terminate()) else {
        let _ = tokio::signal::ctrl_c().await;
        return;
    };
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
        result = protocol_loop(reader, writer.clone(), active.clone(), provider_tasks) => result,
        _ = &mut shutdown => Ok(None),
    };
    cancel_and_join(&active, provider_tasks).await;
    match result {
        Ok(Some(request_id)) => {
            send_control(&writer, request_id, [0; 32], json!({"status":"bye"})).await;
            Ok(())
        }
        Ok(None) => Ok(()),
        Err(error) => Err(error),
    }
}

pub async fn run() -> Result<(), AwareError> {
    // This cross-process fence is independent of the Node bridge. If the bridge crashes, EOF
    // drives `supervise_protocol` through provider cancellation/join while this lock remains held;
    // the next app run cannot admit another host until the cleanup has actually completed.
    let _cleanup_fence = match std::env::var_os("AWARE_MODEL_READER_HOST_CLEANUP_FENCE") {
        Some(value) => {
            use fs2::FileExt;
            let path = PathBuf::from(value);
            if !path.is_absolute() {
                return Err(AwareError::Validation(
                    "model-reader host cleanup fence must be absolute".into(),
                ));
            }
            let file = std::fs::OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .truncate(false)
                .open(path)?;
            file.lock_exclusive()?;
            Some(file)
        }
        None => None,
    };
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

    fn file_sha256(path: &std::path::Path) -> String {
        format!("{:x}", Sha256::digest(std::fs::read(path).unwrap()))
    }

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
        let executable = std::env::current_exe().unwrap();
        let request = ProviderRun {
            op: "provider-run".into(),
            executable_sha256: file_sha256(&executable),
            executable,
            operation: "convert".into(),
            cwd: std::env::current_dir().unwrap(),
            environment: BTreeMap::from([("TZ".into(), "UTC".into())]),
            stdin_length: 1,
            timeout_ms: 1000,
            stdout_limit: 10,
            stderr_limit: 10,
        };
        let prepared = provider_command(&request).unwrap();
        let debug = format!("{:?}", prepared.command);
        assert!(debug.contains("convert"));
        assert!(debug.contains("--json-stdin"));
        // The two halves the name claims, asserted rather than gestured at. `Command`'s Debug
        // renders an env-cleared command with a leading `env -i` and lists only the argv it will
        // pass, so both are observable — on unix, where that rendering is defined.
        #[cfg(unix)]
        {
            assert!(
                debug.contains("env -i TZ=\"UTC\""),
                "the provider gets the requested environment and nothing it inherited: {debug}"
            );
            let after_argv = debug.split_once("\"--json-stdin\"").unwrap().1;
            assert!(
                !after_argv.contains('"'),
                "no argument may follow the protocol's own two: {debug}"
            );
        }
    }

    #[test]
    fn provider_command_refuses_an_image_that_does_not_match_the_host_digest() {
        let executable = std::env::current_exe().unwrap();
        let request = ProviderRun {
            op: "provider-run".into(),
            executable,
            executable_sha256: "0".repeat(64),
            operation: "describe".into(),
            cwd: std::env::current_dir().unwrap(),
            environment: BTreeMap::new(),
            stdin_length: 0,
            timeout_ms: 1000,
            stdout_limit: 10,
            stderr_limit: 10,
        };
        assert!(provider_command(&request).is_err());
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
    async fn output_limit_notifies_supervision_before_the_provider_closes_its_pipe() {
        let (mut writer, reader) = tokio::io::duplex(64);
        let (failure_tx, mut failure_rx) = mpsc::unbounded_channel();
        let task = tokio::spawn(read_bounded_supervised(reader, 2, failure_tx));
        writer.write_all(b"three").await.unwrap();
        let failure = tokio::time::timeout(std::time::Duration::from_secs(1), failure_rx.recv())
            .await
            .expect("output-limit notification timed out")
            .expect("output reader dropped its notification channel");
        assert!(matches!(failure, ProviderIoFailure::Stream));
        assert_eq!(
            task.await.unwrap().unwrap_err().kind(),
            std::io::ErrorKind::FileTooLarge
        );
    }

    #[tokio::test]
    async fn closed_provider_stdin_is_reported_as_a_supervision_failure() {
        let (writer, reader) = tokio::io::duplex(1);
        drop(reader);
        let (failure_tx, mut failure_rx) = mpsc::unbounded_channel();
        let result = write_input_supervised(writer, vec![1, 2, 3], failure_tx).await;
        assert!(result.is_err());
        assert!(matches!(
            failure_rx.recv().await,
            Some(ProviderIoFailure::Stdin)
        ));
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

        let mut shutdown_bytes = Vec::new();
        Frame {
            kind: KIND_CONTROL,
            request_id: 1,
            run_handle: [0; 32],
            sequence: 0,
            flags: FINAL,
            payload: serde_json::to_vec(&json!({"op":"shutdown"})).unwrap(),
        }
        .write(&mut shutdown_bytes)
        .await
        .unwrap();
        let (active, mut tasks, _cleaned) = active_provider().await;
        let cancel_seen = Arc::new(tokio::sync::Notify::new());
        let observed = cancel_seen.clone();
        let (cancel_tx, cancel_rx) = oneshot::channel();
        active.lock().await.insert([8; 32], cancel_tx);
        let (release_tx, release_rx) = oneshot::channel();
        tasks.push(tokio::spawn(async move {
            let _ = cancel_rx.await;
            observed.notify_one();
            let _ = release_rx.await;
        }));
        let (signal_tx, signal_rx) = oneshot::channel();
        let writer: SharedWriter = Arc::new(Mutex::new(tokio::io::stdout()));
        let supervisor = tokio::spawn(async move {
            supervise_protocol(
                &mut shutdown_bytes.as_slice(),
                writer,
                active,
                &mut tasks,
                async {
                    let _ = signal_rx.await;
                },
            )
            .await
        });
        cancel_seen.notified().await;
        let _ = signal_tx.send(());
        tokio::time::sleep(std::time::Duration::from_millis(10)).await;
        assert!(
            !supervisor.is_finished(),
            "signal must not detach cleanup already in progress"
        );
        let _ = release_tx.send(());
        supervisor.await.unwrap().unwrap();
    }

    // ── the wire format and the decoder's refusals ───────────────────────────────────
    //
    // Everything below reads bytes the host did not write. The bridge is a separate process
    // in another language, so the 50-byte header is a contract between two implementations
    // and every field of it is reachable by a peer that is buggy, out of date, or hostile.

    /// A control frame carrying `body`, at the sequence/flags the protocol requires.
    fn control(request_id: u64, body: serde_json::Value) -> Frame {
        Frame {
            kind: KIND_CONTROL,
            request_id,
            run_handle: [0; 32],
            sequence: 0,
            flags: FINAL,
            payload: serde_json::to_vec(&body).unwrap(),
        }
    }

    /// Feed `frames` to `protocol_loop` as one byte stream and return what it made of them,
    /// joining any provider task it started so a refusal can't leave one running.
    async fn drive(frames: &[Frame]) -> Result<Option<u64>, AwareError> {
        let mut wire = Vec::new();
        for frame in frames {
            frame.write(&mut wire).await.unwrap();
        }
        let writer: SharedWriter = Arc::new(Mutex::new(tokio::io::stdout()));
        let active: ActiveRuns = Arc::new(Mutex::new(HashMap::new()));
        let mut tasks: Vec<tokio::task::JoinHandle<()>> = Vec::new();
        let result = protocol_loop(&mut wire.as_slice(), writer, active.clone(), &mut tasks).await;
        cancel_and_join(&active, &mut tasks).await;
        result
    }

    #[tokio::test]
    async fn the_frame_header_layout_is_the_documented_wire_format() {
        // Field OFFSETS are the half a round-trip test cannot see: move `sequence` and `flags`
        // in `read` and `write` together and `frames_preserve_...` stays green while every
        // bridge build in the field stops parsing. Pin the bytes themselves, both directions.
        let frame = Frame {
            kind: KIND_STDERR,
            request_id: 0x0102_0304_0506_0708,
            run_handle: [0xAB; 32],
            sequence: 0x0000_00FF,
            flags: FINAL,
            payload: vec![0xDE, 0xAD],
        };
        let mut expected = vec![KIND_STDERR]; // [0]      kind
        expected.extend_from_slice(&[1, 2, 3, 4, 5, 6, 7, 8]); // [1..9]   request id, big-endian
        expected.extend_from_slice(&[0xAB; 32]); // [9..41]  run handle
        expected.extend_from_slice(&[0, 0, 0, 0xFF]); // [41..45] sequence, big-endian
        expected.push(FINAL); // [45]     flags
        expected.extend_from_slice(&[0, 0, 0, 2]); // [46..50] payload length, big-endian
        expected.extend_from_slice(&[0xDE, 0xAD]); // payload

        let mut written = Vec::new();
        frame.write(&mut written).await.unwrap();
        assert_eq!(
            written, expected,
            "the encoder drifted from the wire format"
        );
        assert_eq!(
            Frame::read(&mut expected.as_slice())
                .await
                .unwrap()
                .unwrap(),
            frame,
            "the decoder drifted from the wire format",
        );
    }

    #[tokio::test]
    async fn the_decoder_accepts_every_protocol_kind_and_refuses_any_other() {
        for kind in [KIND_CONTROL, KIND_STDOUT, KIND_STDERR, KIND_STDIN] {
            let frame = Frame {
                kind,
                request_id: 1,
                run_handle: [0; 32],
                sequence: 0,
                flags: FINAL,
                payload: Vec::new(),
            };
            let mut wire = Vec::new();
            frame.write(&mut wire).await.unwrap();
            assert_eq!(
                Frame::read(&mut wire.as_slice()).await.unwrap().unwrap(),
                frame,
                "kind {kind:#04x} is part of the protocol and must decode",
            );
            // The same frame with an off-protocol kind byte, and nothing else changed.
            wire[0] = 0x7F;
            let error = Frame::read(&mut wire.as_slice()).await.unwrap_err();
            assert_eq!(
                error.kind(),
                std::io::ErrorKind::InvalidData,
                "an unknown kind must be rejected, not routed by whatever it resembles",
            );
        }
    }

    #[tokio::test]
    async fn the_decoder_refuses_a_declared_length_over_the_limit_before_allocating_it() {
        // A peer that declares 4 GiB must be refused on the header alone. Only 50 bytes are on
        // the wire here, so a decoder that trusted the length would fail with UnexpectedEof
        // after trying to allocate and fill the payload — a different error, and far too late.
        let mut header = [0u8; HEADER_BYTES];
        header[0] = KIND_CONTROL;
        header[46..50].copy_from_slice(&u32::MAX.to_be_bytes());
        let error = Frame::read(&mut header.as_slice()).await.unwrap_err();
        assert_eq!(error.kind(), std::io::ErrorKind::InvalidData);

        // The boundary is inclusive: a payload of exactly the limit is legal traffic.
        let frame = Frame {
            kind: KIND_CONTROL,
            request_id: 1,
            run_handle: [0; 32],
            sequence: 0,
            flags: FINAL,
            payload: vec![7; MAX_CONTROL_BYTES],
        };
        let mut wire = Vec::new();
        frame.write(&mut wire).await.unwrap();
        let decoded = Frame::read(&mut wire.as_slice()).await.unwrap().unwrap();
        assert_eq!(
            decoded.payload.len(),
            MAX_CONTROL_BYTES,
            "a payload exactly at the limit must decode",
        );
    }

    #[tokio::test]
    async fn the_decoder_separates_a_clean_end_of_stream_from_a_truncated_frame() {
        // Nothing at all is how the bridge closes; a header whose payload never arrives is a
        // half-written frame, and accepting it would hand the host a short `stdin` body as if
        // the peer had sent it in full.
        assert!(Frame::read(&mut &b""[..]).await.unwrap().is_none());

        let mut wire = Vec::new();
        Frame {
            kind: KIND_STDIN,
            request_id: 3,
            run_handle: [1; 32],
            sequence: 0,
            flags: FINAL,
            payload: vec![9; 8],
        }
        .write(&mut wire)
        .await
        .unwrap();
        wire.truncate(HEADER_BYTES + 3);
        let error = Frame::read(&mut wire.as_slice()).await.unwrap_err();
        assert_eq!(error.kind(), std::io::ErrorKind::UnexpectedEof);
    }

    // ── the control loop's refusals ──────────────────────────────────────────────────

    #[tokio::test]
    async fn control_request_ids_must_strictly_increase() {
        // The request id is what correlates a reply to its request. Re-using one would let a
        // replayed frame collect the answer to an exchange that has already completed.
        assert!(
            drive(&[
                control(1, json!({"op": "hello"})),
                control(2, json!({"op": "hello"})),
            ])
            .await
            .is_ok(),
            "increasing ids are ordinary traffic",
        );
        for repeat in [1u64, 0] {
            let error = drive(&[
                control(1, json!({"op": "hello"})),
                control(repeat, json!({"op": "hello"})),
            ])
            .await
            .unwrap_err();
            assert!(
                error
                    .to_string()
                    .contains("control request ids must increase"),
                "id {repeat} after 1 must be refused, got: {error}",
            );
        }
    }

    #[tokio::test]
    async fn the_host_refuses_unknown_ops_and_frames_only_it_may_send() {
        let error = drive(&[control(1, json!({"op": "exec"}))])
            .await
            .unwrap_err();
        assert!(
            error.to_string().contains("control operation is unknown"),
            "{error}",
        );

        // `stdout`/`stderr` travel host → bridge. Inbound they are not control frames, so the
        // loop must refuse them rather than fall through to the control dispatch.
        for kind in [KIND_STDOUT, KIND_STDERR] {
            let frame = Frame {
                kind,
                request_id: 1,
                run_handle: [0; 32],
                sequence: 0,
                flags: FINAL,
                payload: b"{\"op\":\"hello\"}".to_vec(),
            };
            assert!(
                drive(&[frame]).await.is_err(),
                "kind {kind:#04x} must not be accepted inbound",
            );
        }
    }

    #[tokio::test]
    async fn stdin_that_matches_no_pending_run_is_refused() {
        // The run handle is minted by the host and never guessable by the peer. A stdin body
        // carrying an unknown one has no run to feed, and treating it as a no-op would let a
        // peer probe for live handles without ever being told it was wrong.
        let error = drive(&[Frame {
            kind: KIND_STDIN,
            request_id: 1,
            run_handle: [5; 32],
            sequence: 0,
            flags: FINAL,
            payload: Vec::new(),
        }])
        .await
        .unwrap_err();
        assert!(error.to_string().contains("uncorrelated stdin"), "{error}");
    }

    #[tokio::test]
    async fn the_lock_ops_refuse_a_relative_path_and_a_handle_the_host_never_issued() {
        // `lock-acquire` locks whatever path it is handed. A relative one would resolve against
        // the HOST's working directory, not the bridge's, and silently guard the wrong file.
        for body in [
            json!({"op": "lock-acquire", "path": "relative/provider.lock"}),
            json!({"op": "lock-acquire"}),
        ] {
            let error = drive(&[control(1, body.clone())]).await.unwrap_err();
            assert!(
                error.to_string().contains("lock path must be absolute"),
                "{body} must be refused, got: {error}",
            );
        }

        let error = drive(&[control(1, json!({"op": "lock-release"}))])
            .await
            .unwrap_err();
        assert!(
            error.to_string().contains("lock handle is unknown"),
            "releasing an unissued handle must not be reported as a release: {error}",
        );

        // An absolute path is the accepted case, so the refusals above are not vacuous.
        let tmp = tempfile::tempdir().unwrap();
        let lock = tmp.path().join("provider.lock");
        assert!(
            drive(&[
                control(
                    1,
                    json!({"op": "lock-acquire", "path": lock.to_str().unwrap()})
                ),
                control(2, json!({"op": "shutdown"})),
            ])
            .await
            .is_ok(),
        );
        assert!(lock.is_file(), "lock-acquire creates the file it locks");
    }

    #[tokio::test]
    async fn a_provider_run_may_not_declare_more_stdin_than_one_control_frame_can_carry() {
        // `stdin_length` is checked against the frame limit HERE, before a handle is minted,
        // because the stdin body arrives in a single frame and the loop later requires its
        // payload to equal this number exactly. A larger declaration could never be satisfied.
        let request = |stdin_length: usize| {
            json!({
                "op": "provider-run",
                "executable": "/opt/provider/reader",
                "executableSha256": "0".repeat(64),
                "operation": "describe",
                "cwd": "/tmp",
                "environment": {},
                "stdinLength": stdin_length,
                "timeoutMs": 1000,
                "stdoutLimit": 16,
                "stderrLimit": 16,
            })
        };
        let error = drive(&[control(1, request(MAX_CONTROL_BYTES + 1))])
            .await
            .unwrap_err();
        assert!(
            error.to_string().contains("provider request exceeds limit"),
            "{error}",
        );
        // Exactly the limit is accepted. Staging a run launches nothing — the provider only
        // starts once its stdin frame arrives — so this stops at the guard under test.
        assert!(
            drive(&[
                control(1, request(MAX_CONTROL_BYTES)),
                control(2, json!({"op": "shutdown"})),
            ])
            .await
            .is_ok(),
            "a declaration exactly at the limit is satisfiable and must be accepted",
        );
    }

    // ── the provider-launch guard ────────────────────────────────────────────────────

    /// A well-formed request for `image`, which the caller has already written.
    fn provider_run_for(image: &std::path::Path, cwd: &std::path::Path) -> ProviderRun {
        ProviderRun {
            op: "provider-run".into(),
            executable: image.to_path_buf(),
            executable_sha256: file_sha256(image),
            operation: "describe".into(),
            cwd: cwd.to_path_buf(),
            environment: BTreeMap::new(),
            stdin_length: 0,
            timeout_ms: 1000,
            stdout_limit: 16,
            stderr_limit: 16,
        }
    }

    #[test]
    fn provider_command_refuses_a_request_outside_the_protocol_before_it_opens_the_image() {
        let tmp = tempfile::tempdir().unwrap();
        let image = tmp.path().join("provider-image");
        std::fs::write(&image, b"provider image bytes").unwrap();

        // Accepted unmodified — without this every case below could "pass" for the wrong reason.
        assert!(provider_command(&provider_run_for(&image, tmp.path())).is_ok());

        #[allow(clippy::type_complexity)]
        let cases: [(&str, Box<dyn Fn(&mut ProviderRun)>); 6] = [
            (
                "a relative executable, which would resolve against the host's cwd",
                Box::new(|r| r.executable = PathBuf::from("provider/reader")),
            ),
            (
                "a relative working directory",
                Box::new(|r| r.cwd = PathBuf::from("work")),
            ),
            (
                "an operation the host does not implement",
                Box::new(|r| r.operation = "exec".into()),
            ),
            (
                "a digest shorter than sha-256",
                Box::new(|r| r.executable_sha256.truncate(63)),
            ),
            (
                "a digest with a byte outside lower-case hex",
                Box::new(|r| r.executable_sha256.replace_range(0..1, "g")),
            ),
            (
                "an upper-case digest, which the host's own formatting never produces",
                Box::new(|r| r.executable_sha256 = r.executable_sha256.to_uppercase()),
            ),
        ];
        for (what, break_it) in cases {
            let mut request = provider_run_for(&image, tmp.path());
            break_it(&mut request);
            assert!(
                provider_command(&request).is_err(),
                "the host must refuse {what}",
            );
        }
    }

    #[cfg(unix)]
    #[test]
    fn provider_command_refuses_indirection_even_when_it_resolves_to_the_verified_image() {
        // The digest is taken from whatever the path resolves to at verification time. A symlink
        // can be repointed between that read and the launch, so the host refuses the indirection
        // outright rather than trying to win the race. Same for a directory: `symlink_metadata`
        // is what decides, so neither can reach the hashing step below it.
        let tmp = tempfile::tempdir().unwrap();
        let image = tmp.path().join("provider-image");
        std::fs::write(&image, b"provider image bytes").unwrap();
        let link = tmp.path().join("provider-link");
        std::os::unix::fs::symlink(&image, &link).unwrap();

        // The link and the image are byte-identical and carry the same digest, so the ONLY
        // thing separating the two outcomes below is the indirection itself.
        assert!(provider_command(&provider_run_for(&image, tmp.path())).is_ok());
        let mut through_link = provider_run_for(&image, tmp.path());
        through_link.executable = link;
        let Err(error) = provider_command(&through_link) else {
            panic!("a symlink to the verified image must still be refused");
        };
        assert!(
            error.to_string().contains("regular non-link file"),
            "a symlink must be refused: {error}",
        );

        let mut directory = provider_run_for(&image, tmp.path());
        directory.executable = tmp.path().to_path_buf();
        assert!(
            provider_command(&directory).is_err(),
            "a directory is not a provider image",
        );
    }

    #[tokio::test]
    async fn a_bounded_stream_accepts_exactly_its_limit_and_refuses_one_byte_more() {
        // The limit is the provider's entire output budget. One byte of slack either truncates a
        // legitimate result or lets a runaway provider fill the host's memory.
        assert_eq!(
            read_bounded(&b"1234"[..], 4).await.unwrap(),
            b"1234".to_vec()
        );
        assert_eq!(
            read_bounded(&b"12345"[..], 4).await.unwrap_err().kind(),
            std::io::ErrorKind::FileTooLarge,
        );

        // Across two reads, which is the shape a pipe actually delivers: the budget is spent by
        // the TOTAL, so neither chunk on its own exceeding it makes the stream legal.
        assert_eq!(
            read_bounded(AsyncReadExt::chain(&b"12"[..], &b"34"[..]), 4)
                .await
                .unwrap(),
            b"1234".to_vec(),
        );
        assert_eq!(
            read_bounded(AsyncReadExt::chain(&b"12"[..], &b"345"[..]), 4)
                .await
                .unwrap_err()
                .kind(),
            std::io::ErrorKind::FileTooLarge,
            "a limit checked per read instead of per stream is unbounded",
        );
    }
}
