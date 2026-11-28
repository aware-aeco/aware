use std::{
    os::windows::process::CommandExt,
    process::{Command, ExitStatus, Stdio},
    time::Duration,
};

use tokio::{
    io::{AsyncRead, AsyncReadExt, AsyncWriteExt},
    process::{Child, ChildStdin},
    time::{self, Instant},
};
use win32job::Job;
use windows_sys::Win32::{
    Foundation::HANDLE,
    System::JobObjects::{
        JOBOBJECT_BASIC_ACCOUNTING_INFORMATION, JobObjectBasicAccountingInformation,
        QueryInformationJobObject, TerminateJobObject,
    },
};

use super::wire::LIMIT;
use crate::commands::model_reader_host::{provider_creation_flags, provider_job, resume_provider};

const STDERR_LIMIT: usize = 64 * 1024;
const CLEANUP: Duration = Duration::from_secs(3);

#[derive(Debug, PartialEq, Eq)]
pub enum WorkerError {
    Launch,
    Pipe,
    OutputLimit,
    Timeout,
    Exit,
    Cleanup,
}

async fn pipe<R: AsyncRead + Unpin>(reader: &mut R, limit: usize) -> Result<Vec<u8>, WorkerError> {
    let mut bytes = Vec::new();
    reader
        .take(limit as u64 + 1)
        .read_to_end(&mut bytes)
        .await
        .map_err(|_| WorkerError::Pipe)?;
    if bytes.len() > limit {
        return Err(WorkerError::OutputLimit);
    }
    Ok(bytes)
}

async fn write_input(stdin: &mut Option<ChildStdin>, bytes: &[u8]) -> Result<(), WorkerError> {
    let Some(mut handle) = stdin.take() else {
        return Err(WorkerError::Pipe);
    };
    handle
        .write_all(bytes)
        .await
        .map_err(|_| WorkerError::Pipe)?;
    handle.shutdown().await.map_err(|_| WorkerError::Pipe)
}

async fn reap(child: &mut Option<Child>) -> Result<Option<ExitStatus>, WorkerError> {
    let Some(process) = child else {
        return Ok(None);
    };
    let status = process.wait().await.map_err(|_| WorkerError::Pipe)?;
    // Job accounting can lag while the supervisor still holds a process handle.
    drop(child.take());
    Ok(Some(status))
}

async fn drain(job: &Job) -> Result<(), WorkerError> {
    loop {
        let mut info = JOBOBJECT_BASIC_ACCOUNTING_INFORMATION::default();
        // SAFETY: `job` retains its handle and `info` is a writable buffer of
        // the size required by JobObjectBasicAccountingInformation.
        let queried = unsafe {
            QueryInformationJobObject(
                job.handle() as HANDLE,
                JobObjectBasicAccountingInformation,
                (&mut info as *mut JOBOBJECT_BASIC_ACCOUNTING_INFORMATION).cast(),
                std::mem::size_of_val(&info) as u32,
                std::ptr::null_mut(),
            )
        };
        if queried == 0 {
            return Err(WorkerError::Cleanup);
        }
        if info.ActiveProcesses == 0 {
            return Ok(());
        }
        time::sleep(Duration::from_millis(10)).await;
    }
}

fn terminate(job: &Job) -> Result<(), WorkerError> {
    // SAFETY: `job` owns a live Windows Job Object handle for this call.
    if unsafe { TerminateJobObject(job.handle() as HANDLE, 1) } == 0 {
        return Err(WorkerError::Cleanup);
    }
    Ok(())
}

async fn discard<R: AsyncRead + Unpin>(pipe: &mut R) -> Result<u64, WorkerError> {
    tokio::io::copy(pipe, &mut tokio::io::sink())
        .await
        .map_err(|_| WorkerError::Cleanup)
}

async fn kill_before_start(mut child: Child) -> WorkerError {
    if child.start_kill().is_err() {
        return WorkerError::Cleanup;
    }
    match time::timeout(CLEANUP, child.wait()).await {
        Ok(Ok(_)) => WorkerError::Launch,
        _ => WorkerError::Cleanup,
    }
}

pub async fn supervise(
    mut command: Command,
    request: &[u8],
    budget: Duration,
) -> Result<Vec<u8>, WorkerError> {
    #[cfg(feature = "geometry-test-faults")]
    let inject_cleanup_failure = command
        .get_envs()
        .any(|(name, value)| name == "AWARE_GEOMETRY_INJECT_CLEANUP_FAILURE" && value.is_some());
    command
        .creation_flags(provider_creation_flags())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    let mut command = tokio::process::Command::from(command);
    command.kill_on_drop(true);
    let deadline = Instant::now() + budget;
    let mut process = command.spawn().map_err(|_| WorkerError::Launch)?;
    let job = match provider_job(&process) {
        Ok(job) => job,
        Err(_) => return Err(kill_before_start(process).await),
    };
    let mut stdin = process.stdin.take();
    let (Some(mut stdout), Some(mut stderr)) = (process.stdout.take(), process.stderr.take())
    else {
        return Err(kill_before_start(process).await);
    };
    if resume_provider(&process).is_err() {
        return Err(kill_before_start(process).await);
    }

    let mut child = Some(process);
    let ordinary = time::timeout_at(deadline, async {
        let (_, output, _diagnostics, status, ()) = tokio::try_join!(
            write_input(&mut stdin, request),
            pipe(&mut stdout, LIMIT),
            pipe(&mut stderr, STDERR_LIMIT),
            reap(&mut child),
            drain(&job),
        )?;
        if status.is_some_and(|status| status.success()) {
            Ok(output)
        } else {
            Err(WorkerError::Exit)
        }
    })
    .await;
    let reason = match ordinary {
        Ok(Ok(output)) => {
            // Simulate failed confirmation after reaping and draining the test worker.
            #[cfg(feature = "geometry-test-faults")]
            if inject_cleanup_failure {
                return Err(WorkerError::Cleanup);
            }
            return Ok(output);
        }
        Ok(Err(WorkerError::Exit)) => return Err(WorkerError::Exit),
        Ok(Err(reason)) => reason,
        Err(_) => WorkerError::Timeout,
    };

    drop(stdin.take());
    terminate(&job)?;
    let cleanup = time::timeout(CLEANUP, async {
        tokio::try_join!(
            reap(&mut child),
            drain(&job),
            discard(&mut stdout),
            discard(&mut stderr)
        )
    })
    .await;
    if !matches!(cleanup, Ok(Ok(_))) {
        return Err(WorkerError::Cleanup);
    }
    Err(reason)
}
