//! `script.run` — a host-free runner for a workflow's data and report steps (#777).
//!
//! Runs JavaScript in embedded QuickJS (QuickJS-ng through `rquickjs`), in-process, on every
//! platform AWARE ships, with nothing to install. It exists so a step that touches no host
//! program — compare two JSON files, build an HTML report string — no longer has to borrow a
//! host's runner (`tekla/exec`, `revit/exec`, `blender/exec`) and that host's install.
//!
//! # What a script can do
//!
//! The approved `code` is the body of a function; its `return` value is the node's `result`. It
//! sees four globals and nothing else:
//!
//! - `args` — the node's templatable `args` data, as a frozen deep copy;
//! - `files` — each file the node names, read once before the script starts, as
//!   `{ text, json, path, bytes, sha256 }` (`json` is `JSON.parse(text)` on first access);
//! - `run` — `{ startedAt, startedAtMs }`, the run's start as AWARE recorded it;
//! - `console` — `log`/`info`/`warn`/`error`, into the trace, capped at 64 KB.
//!
//! No `require`, `import`, `fetch`, timers, file system, `Date`, `performance`, `WeakRef` or
//! `Intl`. `Math.random` and the locale-dependent methods (`Number.prototype.toLocaleString`,
//! `String.prototype.localeCompare` / `toLocaleUpperCase` / `toLocaleLowerCase`,
//! `BigInt.prototype.toLocaleString`) are replaced by functions that throw a plain message; the
//! originals are dropped, not hidden.
//!
//! # Code is pinned, data stays data
//!
//! The code is compiled to bytecode in a scratch context that has QuickJS's `Eval` intrinsic,
//! and that bytecode is then loaded and run in a context built **without** it. So in the context
//! the script runs in, `eval`, `new Function`, `(function(){}).constructor` and the
//! `AsyncFunction` / `GeneratorFunction` / `AsyncGeneratorFunction` constructors all throw: text
//! handed in as data — an AI step's answer in `args`, a file's contents — can never be run as
//! code. The only bytecode ever loaded is what `Module::write` produced moments earlier in the
//! same process from the approved source; bytecode from anywhere else is never accepted.
//!
//! # What is guaranteed
//!
//! The output is a function of the pinned `code`, `args`, the file bytes and the recorded run
//! start, **for a given AWARE CLI version**: a built-in's agent digest pins its manifest, not its
//! implementation, so every successful run records the CLI version and the engine version
//! alongside its result.
//!
//! # Limits
//!
//! Cooperative, and enforced at the interpreter's safe points: a wall-clock deadline through the
//! QuickJS interrupt handler (`timeout-seconds`), `Runtime::set_memory_limit` (`memory-mb`), a
//! JavaScript stack limit inside a worker thread whose own stack is larger, a byte budget over
//! `args` plus the files (`max-input-mb`, at most 16 files), and a cap on the serialised result
//! (`max-output-mb`). File reading happens before the clock starts and has its own byte budget;
//! the deadline then covers compiling, running and converting the result. Every limit ends in a
//! failed node with a plain-English error, never a process abort.
//!
//! # Failure fails the node
//!
//! A throw, a timeout, the memory or stack limit, a file problem or an invalid result is an `Err`
//! from the invoker, so downstream nodes do not run on it — deliberately unlike `tekla/exec`'s
//! `ok: false` data: a report or a write after a broken compare must not run.

use std::cell::RefCell;
use std::collections::HashSet;
use std::io::Read;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU8, AtomicUsize, Ordering};
use std::time::{Duration, Instant};

use rquickjs::allocator::{Allocator, RustAllocator};
use rquickjs::context::intrinsic;
use rquickjs::function::{Rest, This};
use rquickjs::object::{Accessor, Property};
use rquickjs::{
    Array, Coerced, Context, Ctx, Exception, FromJs, Function, Module, Object, Runtime, Type,
    Value as JsValue, WriteOptions,
};
use serde_json::{Map, Value};
use sha2::{Digest, Sha256};

use crate::error::AwareError;

/// The QuickJS engine the pinned `rquickjs` bundles: QuickJS-ng, the version its `quickjs.h`
/// declares (`QJS_VERSION_*`). Recorded with every run; `the_recorded_engine_matches_the_lock`
/// fails the build when the pin in `Cargo.lock` moves without this moving with it.
pub(crate) const ENGINE_NAME: &str = "QuickJS-ng";
pub(crate) const ENGINE_VERSION: &str = "0.16.2";
/// The `rquickjs` binding version, pinned with `=` in `Cargo.toml`.
pub(crate) const RQUICKJS_VERSION: &str = "0.14.0";

const MAX_FILES: usize = crate::manifest::agent::MAX_PATH_MAP_ENTRIES;
const LOG_CAP_BYTES: usize = 64 * 1024;
/// The most console lines kept: a cap on count as well as bytes (#777 review).
const LOG_CAP_LINES: usize = 4096;
const LOG_TRUNCATED_NOTE: &str = "[console output truncated at 64 KB or 4096 lines]";
/// How deep a result may nest. Far beyond any report or comparison, and well inside the worker's
/// stack for the recursive conversion below.
const MAX_RESULT_DEPTH: usize = 256;
/// The worker thread's own stack, and the JavaScript stack limit inside it. The JavaScript limit
/// is measured from where the runtime is created, at the top of the worker, so the gap is what
/// the result conversion, the native builtins and the C frames between QuickJS's own checks may
/// use; deep JavaScript recursion is refused as a catchable stack overflow long before the
/// thread's guard page.
const WORKER_STACK_BYTES: usize = 32 * 1024 * 1024;
const JS_STACK_BYTES: usize = 8 * 1024 * 1024;
const READ_CHUNK_BYTES: usize = 1024 * 1024;
const MIB: u64 = 1024 * 1024;

const NOT_INTERRUPTED: u8 = 0;
const TIMED_OUT: u8 = 1;
const CANCELLED: u8 = 2;

/// How long past its deadline the awaiting side waits for the worker before giving up on it.
/// Some QuickJS natives run long loops that never poll the interrupt handler (`indexOf` on two
/// huge strings is O(n*m)), so the deadline alone cannot always stop a script; this bound makes
/// the NODE fail on time even when the thread cannot be stopped (#777 review).
const DEADLINE_GRACE: Duration = Duration::from_secs(5);
/// How many workers given up on that way may still be spinning in this process before another
/// script is refused. Each holds a core and up to its `memory-mb`; there is no safe way to stop a
/// thread from outside, so the bound is on how many can pile up (a watcher re-running the same
/// script on every event would otherwise start one per event).
const MAX_ABANDONED_WORKERS: usize = 2;
/// Workers this process gave up on that have not finished yet.
static ABANDONED_WORKERS: AtomicUsize = AtomicUsize::new(0);
const WORKER_RUNNING: u8 = 0;
const WORKER_ABANDONED: u8 = 1;
const WORKER_DONE: u8 = 2;
/// How close to the JavaScript stack limit an allocation or interrupt poll counts as having hit
/// it. A stack overflow allocates its RangeError deeper than the failed check, so any frame
/// deeper than this marks the run as having reached the limit, caught or not (#777 review).
const STACK_LIMIT_MARGIN: usize = 256 * 1024;

/// The run's start, as AWARE recorded it and hands to the script as `run`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct RunStart {
    /// ISO 8601, UTC, millisecond precision (`2026-11-07T09:30:00.000Z`).
    pub(crate) iso: String,
    /// The same instant in milliseconds since the Unix epoch.
    pub(crate) ms: i64,
}

impl RunStart {
    pub(crate) fn now() -> Self {
        Self::at(chrono::Utc::now())
    }

    pub(crate) fn at(instant: chrono::DateTime<chrono::Utc>) -> Self {
        Self {
            iso: instant.to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
            ms: instant.timestamp_millis(),
        }
    }
}

/// The limits one run is held to, each from its input or its default.
#[derive(Clone, Debug)]
struct Limits {
    timeout: Duration,
    timeout_seconds: f64,
    memory_mb: u64,
    max_input_mb: u64,
    max_output_mb: u64,
}

/// A validated `script/run` call: everything is checked before a file is opened.
#[derive(Debug)]
struct Request {
    code: String,
    args: Value,
    files: Vec<(String, String)>,
    limits: Limits,
}

/// One file handed to the script, read once, before the clock starts.
struct LoadedFile {
    name: String,
    /// The path as the approved workflow wrote it: what the script sees as `files.<name>.path`.
    /// The resolved location is NOT handed to the script — a symlink retargeted between runs would
    /// otherwise change the result without any input the guarantee names changing (#777 review).
    path: String,
    /// Where `path` resolved on this run (symlinks followed): a receipt diagnostic only.
    resolved: String,
    bytes: u64,
    sha256: String,
    text: String,
}

fn invalid(message: impl Into<String>) -> AwareError {
    AwareError::Validation(format!("script run: {}", message.into()))
}

/// `script.run` — the builtin invoker's entry point. Validates the call, then runs it on a
/// dedicated worker thread with an explicit stack and awaits the answer without blocking the
/// async executor. Dropping the returned future (the run was stopped) raises the cancellation
/// flag the interrupt handler and the Rust-side loops check, so the worker stops at its next safe
/// point instead of running on unobserved.
pub(crate) async fn script_run(
    args: Value,
    run_start: Option<RunStart>,
) -> Result<Value, AwareError> {
    let request = parse_request(&args)?;
    admit(&ABANDONED_WORKERS)?;
    let limits = request.limits.clone();
    let run = run_start.unwrap_or_else(RunStart::now);
    let cancel = Arc::new(AtomicBool::new(false));
    let _cancel_on_drop = CancelOnDrop(Arc::clone(&cancel));
    let (tx, mut rx) = tokio::sync::oneshot::channel();
    let (started_tx, started_rx) = tokio::sync::oneshot::channel();
    let state = Arc::new(AtomicU8::new(WORKER_RUNNING));
    let worker_cancel = Arc::clone(&cancel);
    let worker_state = Arc::clone(&state);
    std::thread::Builder::new()
        .name("aware-script".into())
        .stack_size(WORKER_STACK_BYTES)
        .spawn(move || {
            let answer = run_blocking(&request, &run, &worker_cancel, Some(started_tx));
            if worker_state.swap(WORKER_DONE, Ordering::AcqRel) == WORKER_ABANDONED {
                ABANDONED_WORKERS.fetch_sub(1, Ordering::AcqRel);
            }
            let _ = tx.send(answer);
        })
        .map_err(|e| {
            AwareError::Internal(format!(
                "script run: could not start the script thread: {e}"
            ))
        })?;
    // Armed only now that a worker exists (a failed spawn above has nothing to count). Whichever
    // way this future ends without the worker's answer — the outer bound below, or the run being
    // stopped and the future dropped — the worker is counted as given up on until it finishes,
    // so a stopped run cannot leave an uncounted thread spinning (#777 review).
    let _give_up_on_drop = GiveUpOnDrop {
        state: Arc::clone(&state),
        counter: &ABANDONED_WORKERS,
    };
    let panicked = || {
        Err(AwareError::Internal(
            "script run: the script thread stopped without an answer (it panicked)".into(),
        ))
    };
    // Reading the files is not on the clock; the deadline starts when the worker says so.
    let deadline = tokio::select! {
        answer = &mut rx => return answer.unwrap_or_else(|_| panicked()),
        started = started_rx => match started {
            Ok(deadline) => deadline,
            Err(_) => return rx.await.unwrap_or_else(|_| panicked()),
        },
    };
    let give_up_at = tokio::time::Instant::from_std(deadline + DEADLINE_GRACE);
    match tokio::time::timeout_at(give_up_at, &mut rx).await {
        Ok(answer) => answer.unwrap_or_else(|_| panicked()),
        Err(_) => {
            cancel.store(true, Ordering::Release);
            if let Ok(answer) = rx.try_recv() {
                return answer; // it finished in the same instant
            }
            // `_give_up_on_drop` counts the worker as this function returns.
            Err(timed_out_in_native(&limits))
        }
    }
}

/// Counts the worker as given up on when the awaiting future ends without its answer.
struct GiveUpOnDrop {
    state: Arc<AtomicU8>,
    counter: &'static AtomicUsize,
}

impl Drop for GiveUpOnDrop {
    fn drop(&mut self) {
        // The usual case: the answer arrived, so the worker is already done.
        if self.state.load(Ordering::Acquire) == WORKER_DONE {
            return;
        }
        // Count FIRST, then publish the state: the worker decrements only after it sees
        // `WORKER_ABANDONED`, so the counter never dips below zero (a decrement racing ahead of
        // the increment would wrap it and refuse every script for that instant).
        self.counter.fetch_add(1, Ordering::AcqRel);
        if self
            .state
            .compare_exchange(
                WORKER_RUNNING,
                WORKER_ABANDONED,
                Ordering::AcqRel,
                Ordering::Acquire,
            )
            .is_err()
        {
            // The worker already finished: nothing is left running.
            self.counter.fetch_sub(1, Ordering::AcqRel);
        }
    }
}

/// Refuse a new script while too many given-up workers are still running in this process.
fn admit(abandoned: &AtomicUsize) -> Result<(), AwareError> {
    let spinning = abandoned.load(Ordering::Acquire);
    if spinning >= MAX_ABANDONED_WORKERS {
        return Err(AwareError::Internal(format!(
            "script run: {spinning} earlier scripts are still running past their deadline \
             inside a built-in operation that cannot be interrupted, so no new script is started \
             until they finish or AWARE is restarted. Make the long operation smaller (for \
             example, search smaller strings) and restart the run"
        )));
    }
    Ok(())
}

/// Raises the cancellation flag when the awaiting future goes away.
struct CancelOnDrop(Arc<AtomicBool>);

impl Drop for CancelOnDrop {
    fn drop(&mut self) {
        self.0.store(true, Ordering::Release);
    }
}

fn parse_request(args: &Value) -> Result<Request, AwareError> {
    let Some(obj) = args.as_object() else {
        return Err(invalid(format!(
            "the node's inputs must be a mapping (got {})",
            crate::json::type_name(args)
        )));
    };
    match obj.get("language") {
        Some(Value::String(language)) if language == "javascript" => {}
        None | Some(Value::Null) => {
            return Err(invalid(
                "`language` is required; the only language today is `javascript`",
            ));
        }
        Some(other) => {
            return Err(invalid(format!(
                "`language` {other} is not supported; the only language today is `javascript`"
            )));
        }
    }
    let code = match obj.get("code") {
        Some(Value::String(code)) if !code.trim().is_empty() => code.clone(),
        _ => {
            return Err(invalid(
                "`code` is required: the JavaScript body to run, written out in the workflow",
            ));
        }
    };
    let script_args = match obj.get("args") {
        None | Some(Value::Null) => Value::Object(Map::new()),
        Some(Value::Object(map)) => Value::Object(map.clone()),
        Some(other) => {
            return Err(invalid(format!(
                "`args` must be a mapping of names to values (got {})",
                crate::json::type_name(other)
            )));
        }
    };
    let files = match obj.get("files") {
        None | Some(Value::Null) => Vec::new(),
        Some(Value::Object(map)) => {
            if map.len() > MAX_FILES {
                return Err(invalid(format!(
                    "`files` names {} files; at most {MAX_FILES} are allowed",
                    map.len()
                )));
            }
            let mut files = Vec::with_capacity(map.len());
            for (name, path) in map {
                if let Some(problem) =
                    crate::manifest::agent::path_map_entry_problem(name, path.as_str())
                {
                    return Err(invalid(format!("`files`: {problem}")));
                }
                files.push((name.clone(), path.as_str().unwrap_or_default().to_owned()));
            }
            files
        }
        Some(other) => {
            return Err(invalid(format!(
                "`files` must map each name to the absolute path of a file (got {})",
                crate::json::type_name(other)
            )));
        }
    };
    let timeout_seconds = number_limit(obj, "timeout-seconds", 60.0, 600.0)?;
    let limits = Limits {
        timeout: Duration::from_secs_f64(timeout_seconds),
        timeout_seconds,
        memory_mb: whole_limit(obj, "memory-mb", 1024, 16, 4096)?,
        max_input_mb: whole_limit(obj, "max-input-mb", 128, 1, 512)?,
        max_output_mb: whole_limit(obj, "max-output-mb", 32, 1, 128)?,
    };
    Ok(Request {
        code,
        args: script_args,
        files,
        limits,
    })
}

/// A positive number of seconds, at most `max`. Out of range is refused, never clamped.
fn number_limit(
    obj: &Map<String, Value>,
    key: &str,
    default: f64,
    max: f64,
) -> Result<f64, AwareError> {
    match obj.get(key) {
        None | Some(Value::Null) => Ok(default),
        Some(Value::Number(n)) => match n.as_f64() {
            Some(v) if v > 0.0 && v <= max => Ok(v),
            _ => Err(invalid(format!(
                "`{key}` is {n}; it must be more than 0 and at most {max}"
            ))),
        },
        Some(other) => Err(invalid(format!(
            "`{key}` must be a number (got {})",
            crate::json::type_name(other)
        ))),
    }
}

/// A whole number of megabytes between `min` and `max`. Out of range is refused, never clamped.
fn whole_limit(
    obj: &Map<String, Value>,
    key: &str,
    default: u64,
    min: u64,
    max: u64,
) -> Result<u64, AwareError> {
    match obj.get(key) {
        None | Some(Value::Null) => Ok(default),
        Some(Value::Number(n)) => match n.as_u64() {
            Some(v) if (min..=max).contains(&v) => Ok(v),
            _ => Err(invalid(format!(
                "`{key}` is {n}; it must be a whole number from {min} to {max}"
            ))),
        },
        Some(other) => Err(invalid(format!(
            "`{key}` must be a whole number (got {})",
            crate::json::type_name(other)
        ))),
    }
}

/// The whole run, on the worker thread: read the files, then compile, run and convert under the
/// deadline, and shape the node's output.
fn run_blocking(
    request: &Request,
    run: &RunStart,
    cancel: &Arc<AtomicBool>,
    started: Option<tokio::sync::oneshot::Sender<Instant>>,
) -> Result<Value, AwareError> {
    let input_budget = request.limits.max_input_mb * MIB;
    let args_bytes = serde_json::to_vec(&request.args)?.len() as u64;
    if args_bytes > input_budget {
        return Err(invalid(format!(
            "`args` is {} bytes, more than `max-input-mb` ({} MB)",
            args_bytes, request.limits.max_input_mb
        )));
    }
    let files = read_files(&request.files, input_budget - args_bytes, request, cancel)?;
    let receipts: Vec<Value> = files
        .iter()
        .map(|file| {
            serde_json::json!({
                "name": file.name,
                "path": file.path,
                "resolvedPath": file.resolved,
                "bytes": file.bytes,
                "sha256": file.sha256,
            })
        })
        .collect();

    let deadline = Instant::now() + request.limits.timeout;
    if let Some(started) = started {
        let _ = started.send(deadline);
    }
    let (result, logs) = execute(request, files, run, deadline, cancel)?;

    let mut out = Map::new();
    out.insert("ok".into(), Value::Bool(true));
    out.insert("result".into(), result);
    if !logs.is_empty() {
        out.insert(
            "logs".into(),
            Value::Array(logs.into_iter().map(Value::String).collect()),
        );
    }
    out.insert("files".into(), Value::Array(receipts));
    out.insert(
        "run".into(),
        serde_json::json!({ "startedAt": run.iso, "startedAtMs": run.ms }),
    );
    out.insert(
        "engine".into(),
        serde_json::json!({
            "name": ENGINE_NAME,
            "version": ENGINE_VERSION,
            "rquickjs": RQUICKJS_VERSION,
            "aware-cli": env!("CARGO_PKG_VERSION"),
        }),
    );
    Ok(Value::Object(out))
}

/// Read each named file once, read-only, through the shared admission rule, within the byte
/// budget, hashing as it goes. The contents go to the script only; the caller records receipts.
fn read_files(
    files: &[(String, String)],
    budget: u64,
    request: &Request,
    cancel: &AtomicBool,
) -> Result<Vec<LoadedFile>, AwareError> {
    let mut remaining = budget;
    let mut loaded = Vec::with_capacity(files.len());
    for (name, raw) in files {
        let readable = crate::fs::canonicalise_readable_file(std::path::Path::new(raw))
            .map_err(|e| invalid(format!("file {name:?}: {e}")))?;
        let path = readable.path.display().to_string();
        let approved = raw.clone();
        if readable.len > remaining {
            return Err(over_input_budget(name, &path, request));
        }
        let mut reader = readable.file.take(remaining + 1);
        let mut bytes = Vec::with_capacity(usize::try_from(readable.len).unwrap_or(0));
        let mut hasher = Sha256::new();
        let mut chunk = vec![0u8; READ_CHUNK_BYTES];
        loop {
            if cancel.load(Ordering::Acquire) {
                return Err(cancelled());
            }
            let n = reader
                .read(&mut chunk)
                .map_err(|e| invalid(format!("file {name:?} ({path}) could not be read: {e}")))?;
            if n == 0 {
                break;
            }
            let Some(read) = chunk.get(..n) else {
                break;
            };
            hasher.update(read);
            bytes.extend_from_slice(read);
            if bytes.len() as u64 > remaining {
                return Err(over_input_budget(name, &path, request));
            }
        }
        remaining -= bytes.len() as u64;
        let size = bytes.len() as u64;
        let text = String::from_utf8(bytes).map_err(|e| {
            invalid(format!(
                "file {name:?} ({path}) is not UTF-8 text ({e}); a script reads text files only"
            ))
        })?;
        loaded.push(LoadedFile {
            name: name.clone(),
            path: approved,
            resolved: path,
            bytes: size,
            sha256: format!("{:x}", hasher.finalize()),
            text,
        });
    }
    Ok(loaded)
}

fn over_input_budget(name: &str, path: &str, request: &Request) -> AwareError {
    invalid(format!(
        "file {name:?} ({path}) takes `args` and the files past `max-input-mb` ({} MB); raise \
         `max-input-mb` (at most 512) or hand the script a smaller file",
        request.limits.max_input_mb
    ))
}

fn cancelled() -> AwareError {
    invalid("the run was stopped before the script finished")
}

/// The deadline passed while the script was inside a built-in that never yields to the
/// interrupt check. The node fails on time; the thread is left to finish on its own.
fn timed_out_in_native(limits: &Limits) -> AwareError {
    invalid(format!(
        "the script did not finish within {} seconds (`timeout-seconds`): it was inside a \
         built-in operation that cannot be interrupted (a search or split over very large \
         strings, say). The node is failed now; the operation is left to finish in the \
         background. Work on smaller pieces, or raise `timeout-seconds` (at most 600)",
        limits.timeout_seconds
    ))
}

fn stack_message() -> String {
    "the script recursed too deeply (stack overflow); make the recursion shallower or use a loop. \
     Catching the overflow does not help: how deep a script may go differs between platforms, so \
     a run that reached the limit is failed even when the script caught it"
        .into()
}

fn timed_out(limits: &Limits) -> AwareError {
    invalid(format!(
        "the script did not finish within {} seconds (`timeout-seconds`). The deadline is \
         checked while the script runs, so a loop that never ends is stopped there; raise \
         `timeout-seconds` (at most 600) if the work is genuinely long",
        limits.timeout_seconds
    ))
}

/// The intrinsics of the scratch context the approved code is compiled in. `Eval` is what makes
/// compiling possible at all; `RegExpCompiler` compiles regular-expression literals.
type CompileIntrinsics = (
    intrinsic::Eval,
    intrinsic::RegExpCompiler,
    intrinsic::RegExp,
    intrinsic::Json,
    intrinsic::MapSet,
    intrinsic::TypedArrays,
    intrinsic::Promise,
);

/// The intrinsics of the context the script runs in: **no `Eval`**, so no text can become code,
/// and no `Date`, `Performance`, `WeakRef` (with `FinalizationRegistry`), `Proxy` or
/// `DOMException`. `Promise` is required to evaluate a module at all.
type ExecIntrinsics = (
    intrinsic::Promise,
    intrinsic::RegExpCompiler,
    intrinsic::RegExp,
    intrinsic::Json,
    intrinsic::MapSet,
    intrinsic::TypedArrays,
);

/// Compile, run and convert, under the deadline. Returns the strict-JSON result and the logs.
fn execute(
    request: &Request,
    files: Vec<LoadedFile>,
    run: &RunStart,
    deadline: Instant,
    cancel: &Arc<AtomicBool>,
) -> Result<(Value, Vec<String>), AwareError> {
    let limits = &request.limits;
    let hit = Arc::new(LimitsHit::default());
    let stack_top = native_stack_pointer();
    // The memory limit is enforced by the allocator, not `set_memory_limit`, so that reaching it
    // is RECORDED: QuickJS turns a refused allocation into a catchable error, and a script that
    // caught it could otherwise return an answer that depends on how close to the limit this
    // platform's allocation sizes put it.
    let runtime = Runtime::new_with_alloc(LimitedAllocator {
        inner: RustAllocator,
        used: 0,
        limit: usize::try_from(limits.memory_mb * MIB).unwrap_or(usize::MAX),
        stack_top,
        hit: Arc::clone(&hit),
    })
    .map_err(|e| AwareError::Internal(format!("script run: could not start QuickJS: {e}")))?;
    runtime.set_max_stack_size(JS_STACK_BYTES);
    let reason = Arc::new(AtomicU8::new(NOT_INTERRUPTED));
    {
        let reason = Arc::clone(&reason);
        let cancel = Arc::clone(cancel);
        let hit = Arc::clone(&hit);
        runtime.set_interrupt_handler(Some(Box::new(move || {
            hit.note_stack(stack_top);
            if cancel.load(Ordering::Acquire) {
                reason.store(CANCELLED, Ordering::Release);
                return true;
            }
            if Instant::now() >= deadline {
                reason.store(TIMED_OUT, Ordering::Release);
                return true;
            }
            false
        })));
    }
    let interrupted = || match reason.load(Ordering::Acquire) {
        TIMED_OUT => Some(timed_out(limits)),
        CANCELLED => Some(cancelled()),
        _ => None,
    };

    // The code is the body of a function. It starts on the header's own line, so a line number
    // in an error is the line of the author's code.
    let source = format!("export default function script() {{ {}\n}}\n", request.code);
    let bytecode = {
        let compile = Context::custom::<CompileIntrinsics>(&runtime).map_err(|e| {
            AwareError::Internal(format!("script run: could not create a context: {e}"))
        })?;
        compile.with(|ctx| {
            Module::declare(ctx.clone(), "script.js", source)
                .and_then(|module| module.write(WriteOptions::default()))
                .map_err(|e| {
                    // Always described, so the pending exception is taken off the context even
                    // when the deadline is what gets reported.
                    let described = describe_failure(&ctx, e, limits);
                    interrupted().unwrap_or_else(|| {
                        invalid(format!("the code does not compile: {described}"))
                    })
                })
        })?
    };

    let logs = Rc::new(RefCell::new(LogBuffer::default()));
    let context = Context::custom::<ExecIntrinsics>(&runtime).map_err(|e| {
        AwareError::Internal(format!("script run: could not create a context: {e}"))
    })?;
    let outcome = context.with(|ctx| -> Result<Value, AwareError> {
        let fail = |e: rquickjs::Error| {
            let described = describe_failure(&ctx, e, limits);
            interrupted().unwrap_or_else(|| invalid(described))
        };
        let builtins = Builtins::capture_and_harden(&ctx).map_err(fail)?;
        install_error_watch(&ctx, &hit).map_err(fail)?;
        install_globals(&ctx, &builtins, request, files, run, &logs).map_err(fail)?;

        // SAFETY: `Module::load` reads QuickJS bytecode without validating it, so malformed bytes
        // could corrupt the runtime. The invariant that makes this sound: `bytecode` is exactly
        // the output of `Module::write` above — produced moments earlier, in this same process,
        // by this same runtime and engine build, from the approved source text — and it never
        // leaves this function, so no caller, file, input or other process can supply or alter
        // it. `bytecode` is also declared before `context`, so it outlives everything the loaded
        // module could still reference.
        let module = unsafe { Module::load(ctx.clone(), &bytecode) }.map_err(fail)?;
        let (module, evaluated) = module.eval().map_err(fail)?;
        evaluated.finish::<()>().map_err(|e| match e {
            rquickjs::Error::WouldBlock => invalid(
                "the script's top-level code waits on a promise that never settles; a script \
                 runs to completion and returns a value",
            ),
            other => fail(other),
        })?;
        let entry: Function = module.get("default").map_err(fail)?;
        let returned: JsValue = entry.call(()).map_err(fail)?;
        let mut walk = ResultWalk {
            builtins: &builtins,
            ancestors: HashSet::new(),
            path: Vec::new(),
            used: 0,
            budget: usize::try_from(limits.max_output_mb * MIB).unwrap_or(usize::MAX),
            visited: 0,
            deadline,
            cancel: cancel.as_ref(),
            limits,
        };
        let result = walk.convert(&ctx, returned).map_err(|failure| match failure {
            WalkFailure::Js(e) => fail(e),
            WalkFailure::Stop(error) => error,
            WalkFailure::Invalid(message) => invalid(format!(
                "the value the script returned cannot be the node's result: {message}. A result \
                 may hold only plain objects, arrays, strings, finite numbers, booleans and null"
            )),
        })?;
        Ok(result)
    });
    drop(context);
    // A limit reached anywhere in the run fails the node, whether or not the script caught the
    // error QuickJS raised for it.
    if hit.memory.load(Ordering::Acquire) {
        return Err(invalid(memory_message(limits)));
    }
    if hit.stack.load(Ordering::Acquire) {
        return Err(invalid(stack_message()));
    }
    let result = outcome?;
    // A success is accepted only if nothing stopped the script on the way: an interrupt the
    // script swallowed (a `toJSON` loop inside `console.log`, say) leaves its reason recorded,
    // and a deadline that passed after the last interrupt poll is caught by the clock itself.
    stopped(&reason, cancel, deadline, limits)?;
    let serialised = serde_json::to_vec(&result)?.len() as u64;
    if serialised > limits.max_output_mb * MIB {
        return Err(oversize(limits));
    }
    stopped(&reason, cancel, deadline, limits)?;
    let logs = Rc::try_unwrap(logs)
        .map(RefCell::into_inner)
        .unwrap_or_else(|shared| shared.borrow().clone())
        .finish();
    Ok((result, logs))
}

/// `Err` when the run was interrupted, cancelled or is past its deadline.
fn stopped(
    reason: &AtomicU8,
    cancel: &AtomicBool,
    deadline: Instant,
    limits: &Limits,
) -> Result<(), AwareError> {
    if cancel.load(Ordering::Acquire) || reason.load(Ordering::Acquire) == CANCELLED {
        return Err(cancelled());
    }
    if reason.load(Ordering::Acquire) == TIMED_OUT || Instant::now() >= deadline {
        return Err(timed_out(limits));
    }
    Ok(())
}

fn oversize(limits: &Limits) -> AwareError {
    invalid(format!(
        "the script's result is larger than `max-output-mb` ({} MB) once serialised; return \
         less, or raise `max-output-mb` (at most 128)",
        limits.max_output_mb
    ))
}

/// The message QuickJS gives a stack overflow, and the one it gives an out-of-memory error.
const STACK_OVERFLOW_MESSAGE: &str = "Maximum call stack size exceeded";
const OUT_OF_MEMORY_MESSAGE: &str = "out of memory";

/// Watch every error the engine gives a stack trace, through `Error.prepareStackTrace`, and
/// record a stack overflow or an out-of-memory error as a limit reached (#777 review).
///
/// The depth check in the allocator sees an overflow from deep recursion; this sees the ones it
/// cannot: QuickJS refuses a call on its PROSPECTIVE stack use (a bound function's argument
/// buffer, say), which can throw at a shallow depth, and it raises "out of memory" for a size it
/// refuses before asking the allocator (a string past its maximum length). QuickJS builds the
/// trace when the error leaves the first script frame, so this runs even when the script then
/// catches it. The hook is then locked in: `Error.prepareStackTrace` becomes a read-only
/// `undefined`, so the script can neither read nor replace it (the setter is the only way the
/// engine's hook changes). The trace it writes has QuickJS's own shape, so error messages still
/// name the line and column.
fn install_error_watch<'js>(ctx: &Ctx<'js>, hit: &Arc<LimitsHit>) -> rquickjs::Result<()> {
    let hit = Arc::clone(hit);
    let hook = Function::new(
        ctx.clone(),
        move |error: JsValue<'js>, sites: JsValue<'js>| -> rquickjs::Result<String> {
            // Only an own DATA `message` is read: an inherited one, or an own accessor, is a
            // script getter, and script code that ran here (while QuickJS is building a trace)
            // could catch an overflow the watch never sees. An error QuickJS raises always
            // carries its message as an own data property.
            if let Some(exception) = error.as_exception() {
                match own_data_message(exception.as_object()).as_deref() {
                    Some(STACK_OVERFLOW_MESSAGE) => hit.stack.store(true, Ordering::Release),
                    Some(OUT_OF_MEMORY_MESSAGE) => hit.memory.store(true, Ordering::Release),
                    _ => {}
                }
            }
            Ok(format_trace(&sites))
        },
    )?;
    hook.set_name("prepareStackTrace")?;
    let error: Object = ctx.globals().get("Error")?;
    error.set("prepareStackTrace", hook)?;
    // Lock the hook in, and close the two ways script code could run while a trace is built
    // (#777 review): `Error.stackTraceLimit` is coerced to a number there, so an object with a
    // `valueOf` would run inside it; `Error.captureStackTrace` builds a trace on any object the
    // script hands it, own accessors included. The limit stays the engine's default (10).
    error.prop(
        "prepareStackTrace",
        Property::from(JsValue::new_undefined(ctx.clone())),
    )?;
    error.prop("stackTraceLimit", Property::from(10))?;
    error.prop(
        "captureStackTrace",
        Property::from(JsValue::new_undefined(ctx.clone())),
    )?;
    Ok(())
}

/// An error object's own `message` when it is a plain data property holding a string, read
/// without running any script code (#777 review). `None` for an accessor (a script getter), a
/// missing or inherited message, or anything that is not a string.
///
/// rquickjs has no safe way to read an own property's descriptor without going through a
/// getter, so this calls the engine's `JS_GetOwnProperty`. For an ordinary object (every Error
/// is one: `as_exception` admits only the Error class, never a Proxy) that is a lookup in the
/// object's own shape. It invokes no getter and no trap.
fn own_data_message(error: &Object<'_>) -> Option<std::string::String> {
    use rquickjs::qjs;
    use std::mem::MaybeUninit;

    let ctx = error.ctx().clone();
    let raw_ctx = ctx.as_raw().as_ptr();
    // SAFETY: `raw_ctx` is the live context that owns `error` (borrowed for this call), and the
    // name is a NUL-terminated static string. The atom is freed below on every path.
    let atom = unsafe { qjs::JS_NewAtom(raw_ctx, c"message".as_ptr()) };
    if atom == qjs::JS_ATOM_NULL {
        // Creating the atom failed (out of memory); drop the pending exception it left.
        let _ = ctx.catch();
        return None;
    }
    let mut descriptor = MaybeUninit::<qjs::JSPropertyDescriptor>::uninit();
    // SAFETY: `error.as_raw()` is a live object value of this context; `descriptor` is valid
    // writable memory for one descriptor, and `atom` was created above and is still alive.
    let found =
        unsafe { qjs::JS_GetOwnProperty(raw_ctx, descriptor.as_mut_ptr(), error.as_raw(), atom) };
    // SAFETY: `atom` came from `JS_NewAtom` above and is not used after this.
    unsafe { qjs::JS_FreeAtom(raw_ctx, atom) };
    if found < 0 {
        // Only an engine failure (out of memory instantiating a lazy property) gets here.
        let _ = ctx.catch();
        return None;
    }
    if found == 0 {
        return None;
    }
    // SAFETY: `JS_GetOwnProperty` returned 1, so it wrote every field of the descriptor, each of
    // `value`, `getter` and `setter` being an owned reference (or `undefined`).
    let descriptor = unsafe { descriptor.assume_init() };
    // SAFETY: each value is an owned reference of this context handed to us by the call above;
    // wrapping it once transfers that ownership, so dropping it releases it exactly once.
    let (value, getter, setter) = unsafe {
        (
            JsValue::from_raw(ctx.clone(), descriptor.value),
            JsValue::from_raw(ctx.clone(), descriptor.getter),
            JsValue::from_raw(ctx.clone(), descriptor.setter),
        )
    };
    drop((getter, setter));
    if descriptor.flags & qjs::JS_PROP_GETSET as i32 != 0 {
        return None;
    }
    value.as_string()?.to_string().ok()
}

/// A trace in QuickJS's own format (`    at name (file:line:column)`), from the call sites.
fn format_trace(sites: &JsValue<'_>) -> String {
    let Some(sites) = sites.as_array() else {
        return String::new();
    };
    let mut out = String::new();
    for site in sites.iter::<JsValue>().flatten() {
        let Some(site) = site.as_object() else {
            continue;
        };
        let call = |method: &str| -> Option<JsValue<'_>> {
            let function: Function = site.get(method).ok()?;
            function.call((This(site.clone()),)).ok()
        };
        let text =
            |value: Option<JsValue<'_>>| -> Option<String> { value?.as_string()?.to_string().ok() };
        let number = |value: Option<JsValue<'_>>| -> Option<i64> {
            let value = value?;
            value
                .as_int()
                .map(i64::from)
                .or_else(|| value.as_float().map(|f| f as i64))
        };
        let name = text(call("getFunctionName"))
            .filter(|n| !n.is_empty())
            .unwrap_or_else(|| "<anonymous>".into());
        out.push_str("    at ");
        out.push_str(&name);
        match (
            text(call("getFileName")),
            number(call("getLineNumber")),
            number(call("getColumnNumber")),
        ) {
            (Some(file), Some(line), Some(column)) if line >= 0 => {
                out.push_str(&format!(" ({file}:{line}:{column})"));
            }
            (Some(file), _, _) => out.push_str(&format!(" ({file})")),
            _ => out.push_str(" (native)"),
        }
        out.push('\n');
    }
    out
}

/// Which of the run's hard limits it reached, caught or not.
#[derive(Default)]
struct LimitsHit {
    memory: AtomicBool,
    stack: AtomicBool,
}

impl LimitsHit {
    /// Record the stack limit as reached if this frame is within [`STACK_LIMIT_MARGIN`] of it.
    fn note_stack(&self, stack_top: usize) {
        let depth = stack_top.saturating_sub(native_stack_pointer());
        if depth + STACK_LIMIT_MARGIN >= JS_STACK_BYTES {
            self.stack.store(true, Ordering::Release);
        }
    }
}

/// The address of a local in a frame of its own: the current native stack position. Stacks grow
/// down on every platform AWARE ships, so `top - this` is how deep the thread is.
#[inline(never)]
fn native_stack_pointer() -> usize {
    let marker = 0u8;
    std::hint::black_box(std::ptr::addr_of!(marker)) as usize
}

/// QuickJS's allocator: Rust's, through rquickjs's `RustAllocator`, with the run's memory limit
/// enforced here so that reaching it is recorded, and a stack-depth check on every allocation
/// (a stack overflow allocates its error object, so this sees the overflow even when the
/// script catches it).
struct LimitedAllocator {
    inner: RustAllocator,
    used: usize,
    limit: usize,
    stack_top: usize,
    hit: Arc<LimitsHit>,
}

impl LimitedAllocator {
    /// Whether `extra` more bytes fit; records the memory limit as reached when they do not.
    fn fits(&self, extra: usize) -> bool {
        self.hit.note_stack(self.stack_top);
        if self.used.saturating_add(extra) > self.limit {
            self.hit.memory.store(true, Ordering::Release);
            return false;
        }
        true
    }
}

// SAFETY: every method delegates to `RustAllocator`, which upholds the `Allocator` contract
// (null or a `usize`-aligned region of at least the requested size, and `usable_size` for its
// own allocations). This wrapper only refuses some requests up front by returning null, which
// the contract allows, and keeps a byte count; it never alters a pointer or a size it passes on.
unsafe impl Allocator for LimitedAllocator {
    fn alloc(&mut self, size: usize) -> *mut u8 {
        if !self.fits(size) {
            return std::ptr::null_mut();
        }
        let ptr = self.inner.alloc(size);
        if !ptr.is_null() {
            // SAFETY: `ptr` was just returned, non-null, by the inner allocator.
            self.used += unsafe { RustAllocator::usable_size(ptr) };
        }
        ptr
    }

    fn calloc(&mut self, count: usize, size: usize) -> *mut u8 {
        // `RustAllocator::calloc` panics on an overflowing product; refuse it here instead, so no
        // panic can unwind into QuickJS's C frames.
        let Some(total) = count.checked_mul(size) else {
            return std::ptr::null_mut();
        };
        if !self.fits(total) {
            return std::ptr::null_mut();
        }
        let ptr = self.inner.calloc(count, size);
        if !ptr.is_null() {
            // SAFETY: `ptr` was just returned, non-null, by the inner allocator.
            self.used += unsafe { RustAllocator::usable_size(ptr) };
        }
        ptr
    }

    unsafe fn dealloc(&mut self, ptr: *mut u8) {
        // SAFETY: the caller guarantees `ptr` came from this allocator, i.e. from `inner`.
        let size = unsafe { RustAllocator::usable_size(ptr) };
        self.used = self.used.saturating_sub(size);
        // SAFETY: as above.
        unsafe { self.inner.dealloc(ptr) }
    }

    unsafe fn realloc(&mut self, ptr: *mut u8, new_size: usize) -> *mut u8 {
        // SAFETY: the caller guarantees `ptr` came from this allocator, i.e. from `inner`.
        let old = unsafe { RustAllocator::usable_size(ptr) };
        if new_size > old && !self.fits(new_size - old) {
            return std::ptr::null_mut();
        }
        // SAFETY: as above; on null the old allocation is untouched and still counted.
        let moved = unsafe { self.inner.realloc(ptr, new_size) };
        if !moved.is_null() {
            // SAFETY: `moved` was just returned, non-null, by the inner allocator.
            let now = unsafe { RustAllocator::usable_size(moved) };
            self.used = self.used.saturating_sub(old) + now;
        }
        moved
    }

    unsafe fn usable_size(ptr: *mut u8) -> usize {
        // SAFETY: the caller guarantees `ptr` came from this allocator, i.e. from `inner`.
        unsafe { RustAllocator::usable_size(ptr) }
    }
}

/// A plain-English account of a QuickJS failure: what was thrown, and where.
fn describe_failure(ctx: &Ctx<'_>, error: rquickjs::Error, limits: &Limits) -> String {
    match error {
        rquickjs::Error::Exception => describe_thrown(&ctx.catch(), limits),
        rquickjs::Error::Allocation => memory_message(limits),
        other => format!("{other}"),
    }
}

fn memory_message(limits: &Limits) -> String {
    format!(
        "the script ran out of memory: it needed more than `memory-mb` ({} MB). Raise \
         `memory-mb` (at most 4096) or have it hold less at once",
        limits.memory_mb
    )
}

fn describe_thrown(thrown: &JsValue<'_>, limits: &Limits) -> String {
    if let Some(exception) = thrown.as_exception() {
        let message = exception.message().unwrap_or_default();
        let lower = message.to_ascii_lowercase();
        if lower.contains("out of memory") {
            return memory_message(limits);
        }
        if lower.contains("stack overflow") || lower.contains("maximum call stack") {
            return "the script recursed too deeply (stack overflow); make the recursion \
                    shallower or use a loop"
                .into();
        }
        let name: String = exception
            .get::<_, Coerced<String>>("name")
            .map(|n| n.0)
            .unwrap_or_else(|_| "Error".into());
        let at = exception
            .stack()
            .and_then(|stack| first_script_location(&stack))
            .map(|(line, column)| format!(" (at line {line}, column {column})"))
            .unwrap_or_default();
        return format!("the script threw {name}: {message}{at}");
    }
    let shown = thrown
        .as_string()
        .and_then(|s| s.to_string().ok())
        .or_else(|| {
            Coerced::<String>::from_js(thrown.ctx(), thrown.clone())
                .ok()
                .map(|c| c.0)
        })
        .unwrap_or_else(|| thrown.type_name().to_owned());
    format!("the script threw a value that is not an Error: {shown}")
}

/// The first `script.js:<line>:<column>` frame of a QuickJS stack trace.
fn first_script_location(stack: &str) -> Option<(u32, u32)> {
    let index = stack.find("script.js:")?;
    let rest = stack.get(index + "script.js:".len()..)?;
    let mut parts = rest.split(|c: char| !c.is_ascii_digit());
    let line = parts.next()?.parse().ok()?;
    let column = parts.next()?.parse().ok()?;
    Some((line, column))
}

/// References to the builtins the result walk and the globals need, captured before any script
/// code runs, so nothing the script reassigns can change how its result is read.
struct Builtins<'js> {
    freeze: Function<'js>,
    prevent_extensions: Function<'js>,
    own_keys: Function<'js>,
    own_descriptor: Function<'js>,
    object_prototype: Object<'js>,
    array_prototype: Object<'js>,
}

const RANDOM_MESSAGE: &str = "Math.random is not available in a script: a script must give the \
     same answer on every run. Pass a seed in `args` and use your own generator";

/// The locale-dependent methods, with the reason each is unavailable.
const LOCALE_METHODS: [(&str, &str, &str); 5] = [
    (
        "Number",
        "toLocaleString",
        "Number.prototype.toLocaleString is not available in a script: its output depends on \
         the machine's locale. Format the number yourself, e.g. with toFixed()",
    ),
    (
        "BigInt",
        "toLocaleString",
        "BigInt.prototype.toLocaleString is not available in a script: its output depends on \
         the machine's locale. Use toString() instead",
    ),
    (
        "String",
        "localeCompare",
        "String.prototype.localeCompare is not available in a script: its order depends on the \
         machine's locale. Compare with < and > (code-unit order) instead",
    ),
    (
        "String",
        "toLocaleUpperCase",
        "String.prototype.toLocaleUpperCase is not available in a script: its output depends on \
         the machine's locale. Use toUpperCase() instead",
    ),
    (
        "String",
        "toLocaleLowerCase",
        "String.prototype.toLocaleLowerCase is not available in a script: its output depends on \
         the machine's locale. Use toLowerCase() instead",
    ),
];

impl<'js> Builtins<'js> {
    fn capture_and_harden(ctx: &Ctx<'js>) -> rquickjs::Result<Self> {
        let globals = ctx.globals();
        let object: Object = globals.get("Object")?;
        let reflect: Object = globals.get("Reflect")?;
        let array: Object = globals.get("Array")?;
        let builtins = Self {
            freeze: object.get("freeze")?,
            prevent_extensions: object.get("preventExtensions")?,
            own_keys: reflect.get("ownKeys")?,
            own_descriptor: object.get("getOwnPropertyDescriptor")?,
            object_prototype: object.get("prototype")?,
            array_prototype: array.get("prototype")?,
        };

        // No ambient clock, randomness or locale. Each original is replaced in place (its only
        // reference is the property being overwritten), so no path from script code reaches it.
        replace_with_throwing(
            ctx,
            &globals.get::<_, Object>("Math")?,
            "random",
            RANDOM_MESSAGE,
        )?;
        for (owner, method, message) in LOCALE_METHODS {
            let Ok(constructor) = globals.get::<_, Object>(owner) else {
                continue;
            };
            let prototype: Object = constructor.get("prototype")?;
            replace_with_throwing(ctx, &prototype, method, message)?;
        }
        // None of these is added by the intrinsics above; removed anyway so a future engine that
        // grows one into its base objects cannot quietly hand a script a clock or a locale.
        for ambient in [
            "Date",
            "performance",
            "Intl",
            "WeakRef",
            "FinalizationRegistry",
        ] {
            globals.remove(ambient)?;
        }
        Ok(builtins)
    }
}

fn replace_with_throwing<'js>(
    ctx: &Ctx<'js>,
    owner: &Object<'js>,
    name: &'static str,
    message: &'static str,
) -> rquickjs::Result<()> {
    let thrower = Function::new(ctx.clone(), move |ctx: Ctx<'js>| -> rquickjs::Result<()> {
        Err(Exception::throw_type(&ctx, message))
    })?;
    thrower.set_name(name)?;
    owner.prop(name, Property::from(thrower).writable().configurable())
}

/// Hand the script its globals: `args`, `files`, `run` and `console`, each read-only.
fn install_globals<'js>(
    ctx: &Ctx<'js>,
    builtins: &Builtins<'js>,
    request: &Request,
    files: Vec<LoadedFile>,
    run: &RunStart,
    logs: &Rc<RefCell<LogBuffer>>,
) -> rquickjs::Result<()> {
    let globals = ctx.globals();

    // `args`: a deep copy through JSON, every object and array frozen.
    let args_text = serde_json::to_string(&request.args)
        .map_err(|e| Exception::throw_internal(ctx, &format!("args: {e}")))?;
    let args = ctx.json_parse(args_text)?;
    deep_freeze(builtins, args.clone())?;
    globals.prop("args", Property::from(args).enumerable())?;

    // `files`: `{ text, json, path, bytes, sha256 }` per file; `json` parses on first access.
    let files_object = Object::new(ctx.clone())?;
    for file in files {
        let entry = Object::new(ctx.clone())?;
        let text = rquickjs::String::from_str(ctx.clone(), &file.text)?;
        drop(file.text);
        entry.prop("text", Property::from(text).enumerable())?;
        entry.prop("path", Property::from(file.path.as_str()).enumerable())?;
        entry.prop("bytes", Property::from(file.bytes as f64).enumerable())?;
        entry.prop("sha256", Property::from(file.sha256.as_str()).enumerable())?;
        // The getter captures no JavaScript value: a JS reference held inside a Rust closure is
        // invisible to QuickJS's cycle collector, and the cycle it would close (getter -> value
        // -> ... -> this file object -> getter) would leak past the runtime's teardown.
        let name = file.name.clone();
        let getter =
            move |ctx: Ctx<'js>, this: This<Object<'js>>| -> rquickjs::Result<JsValue<'js>> {
                let text: String = this.0.get("text")?;
                let body = text.strip_prefix('\u{feff}').unwrap_or(&text);
                let parsed = match ctx.json_parse(body) {
                    Ok(value) => value,
                    Err(rquickjs::Error::Exception) => {
                        let caught = ctx.catch();
                        let syntax = caught
                            .as_exception()
                            .and_then(|e| e.get::<_, String>("name").ok())
                            .is_some_and(|n| n == "SyntaxError");
                        if !syntax {
                            return Err(ctx.throw(caught));
                        }
                        let detail = caught
                            .as_exception()
                            .and_then(|e| e.message())
                            .unwrap_or_default();
                        return Err(Exception::throw_syntax(
                            &ctx,
                            &format!(
                                "files.{name}.json: the file {name:?} is not valid JSON ({detail})"
                            ),
                        ));
                    }
                    Err(other) => return Err(other),
                };
                // Replace the accessor with the parsed value (read-only), so every later read returns
                // the same object without parsing again.
                this.0
                    .prop("json", Property::from(parsed.clone()).enumerable())?;
                Ok(parsed)
            };
        entry.prop("json", Accessor::from(getter).configurable().enumerable())?;
        builtins
            .prevent_extensions
            .call::<_, ()>((entry.clone(),))?;
        files_object.prop(file.name.as_str(), Property::from(entry).enumerable())?;
    }
    builtins.freeze.call::<_, ()>((files_object.clone(),))?;
    globals.prop("files", Property::from(files_object).enumerable())?;

    // `run`: the recorded start, as data.
    let run_object = Object::new(ctx.clone())?;
    run_object.prop("startedAt", Property::from(run.iso.as_str()).enumerable())?;
    run_object.prop("startedAtMs", Property::from(run.ms as f64).enumerable())?;
    builtins.freeze.call::<_, ()>((run_object.clone(),))?;
    globals.prop("run", Property::from(run_object).enumerable())?;

    // `console`: into the trace, capped.
    let console = Object::new(ctx.clone())?;
    for (method, prefix) in [
        ("log", ""),
        ("info", ""),
        ("warn", "warning: "),
        ("error", "error: "),
    ] {
        let logs = Rc::clone(logs);
        let writer = Function::new(
            ctx.clone(),
            move |ctx: Ctx<'js>, values: Rest<JsValue<'js>>| -> rquickjs::Result<()> {
                if logs.borrow().truncated {
                    return Ok(());
                }
                // Build the line within what the buffer can still keep: each argument is copied
                // out of QuickJS once at most, and none after the line is already past the cap,
                // so `console.log(...Array(10000).fill(bigString))` cannot allocate gigabytes
                // outside QuickJS's memory limit (#777 review).
                let room = logs.borrow().room();
                let mut line = String::from(prefix);
                for (i, value) in values.0.iter().enumerate() {
                    if line.len() > room {
                        break;
                    }
                    if i > 0 {
                        line.push(' ');
                    }
                    line.push_str(&log_text(&ctx, value));
                }
                logs.borrow_mut().push(line);
                Ok(())
            },
        )?;
        writer.set_name(method)?;
        console.prop(method, Property::from(writer).enumerable())?;
    }
    builtins.freeze.call::<_, ()>((console.clone(),))?;
    globals.prop("console", Property::from(console).enumerable())?;
    Ok(())
}

/// Freeze `root` and every object and array beneath it. Iterative: `args` may nest deeply.
fn deep_freeze<'js>(builtins: &Builtins<'js>, root: JsValue<'js>) -> rquickjs::Result<()> {
    let mut pending = vec![root];
    while let Some(value) = pending.pop() {
        let Some(object) = value.as_object() else {
            continue;
        };
        for child in object.values::<JsValue>() {
            let child = child?;
            if child.is_object() {
                pending.push(child);
            }
        }
        builtins.freeze.call::<_, ()>((value.clone(),))?;
    }
    Ok(())
}

/// One `console` argument as text: strings as they are, other values as JSON where they can be.
fn log_text<'js>(ctx: &Ctx<'js>, value: &JsValue<'js>) -> String {
    match value.type_of() {
        Type::String => value
            .as_string()
            .and_then(|s| s.to_string().ok())
            .unwrap_or_default(),
        Type::Undefined | Type::Uninitialized => "undefined".into(),
        Type::Function | Type::Constructor => "[function]".into(),
        Type::Symbol => "[symbol]".into(),
        Type::Object | Type::Array => match ctx.json_stringify(value.clone()) {
            Ok(Some(text)) => text.to_string().unwrap_or_default(),
            Ok(None) => "undefined".into(),
            Err(_) => {
                let _ = ctx.catch();
                "[value that cannot be shown as JSON]".into()
            }
        },
        _ => Coerced::<String>::from_js(ctx, value.clone())
            .map(|c| c.0)
            .unwrap_or_else(|_| {
                let _ = ctx.catch();
                format!("[{}]", value.type_name())
            }),
    }
}

/// The script's console output, capped at [`LOG_CAP_BYTES`] and then truncated with a note.
#[derive(Default, Clone)]
struct LogBuffer {
    lines: Vec<String>,
    bytes: usize,
    truncated: bool,
}

impl LogBuffer {
    /// Bytes the buffer can still keep.
    fn room(&self) -> usize {
        LOG_CAP_BYTES.saturating_sub(self.bytes)
    }

    fn push(&mut self, line: String) {
        if self.truncated {
            return;
        }
        if self.lines.len() >= LOG_CAP_LINES {
            self.bytes = LOG_CAP_BYTES;
            self.truncated = true;
            return;
        }
        // Each line is charged its newline too, so even empty lines use up the cap.
        let remaining = LOG_CAP_BYTES.saturating_sub(self.bytes);
        if line.len() < remaining {
            self.bytes += line.len() + 1;
            self.lines.push(line);
            return;
        }
        let mut cut = remaining.saturating_sub(1); // room for its newline
        while cut > 0 && !line.is_char_boundary(cut) {
            cut -= 1;
        }
        if let Some(kept) = line.get(..cut)
            && !kept.is_empty()
        {
            self.lines.push(kept.to_owned());
        }
        self.bytes = LOG_CAP_BYTES;
        self.truncated = true;
    }

    fn finish(mut self) -> Vec<String> {
        if self.truncated {
            self.lines.push(LOG_TRUNCATED_NOTE.into());
        }
        self.lines
    }
}

/// How many bytes `text` takes once `serde_json` writes it as a JSON string, quotes excluded.
fn json_escaped_len(text: &str) -> usize {
    text.bytes()
        .map(|b| match b {
            b'"' | b'\\' | b'\n' | b'\r' | b'\t' | 0x08 | 0x0c => 2,
            0x00..=0x1f => 6,
            _ => 1,
        })
        .sum()
}

/// Why the result walk stopped.
enum WalkFailure {
    /// A QuickJS call failed (memory, or the interrupt handler).
    Js(rquickjs::Error),
    /// The deadline or the cancellation flag, checked by the walk itself.
    Stop(AwareError),
    /// The value is not strict JSON; the message names where and why.
    Invalid(String),
}

enum Segment {
    Key(String),
    Index(usize),
}

/// The strict conversion of the script's return value into the node's JSON result.
///
/// A custom walk, not `JSON.stringify`, so nothing is silently coerced or dropped and no script
/// code runs: `toJSON` is never called, and a getter is refused as an accessor property without
/// being invoked. Plain objects (prototype `Object.prototype` or `null`), dense arrays (prototype
/// `Array.prototype`), strings, finite numbers, booleans and null are accepted; everything else
/// is refused with the path to the offending value.
struct ResultWalk<'a, 'js> {
    builtins: &'a Builtins<'js>,
    ancestors: HashSet<JsValue<'js>>,
    path: Vec<Segment>,
    used: usize,
    budget: usize,
    visited: u64,
    deadline: Instant,
    cancel: &'a AtomicBool,
    limits: &'a Limits,
}

impl<'js> ResultWalk<'_, 'js> {
    fn convert(&mut self, ctx: &Ctx<'js>, value: JsValue<'js>) -> Result<Value, WalkFailure> {
        if value.is_undefined() {
            return Err(WalkFailure::Invalid(
                "the script returned nothing (undefined); end it with `return <value>`".into(),
            ));
        }
        self.value(ctx, value, 0)
    }

    fn here(&self) -> String {
        let mut text = String::from("result");
        for segment in &self.path {
            match segment {
                Segment::Index(i) => text.push_str(&format!("[{i}]")),
                Segment::Key(key)
                    if !key.is_empty()
                        && key.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
                        && !key.starts_with(|c: char| c.is_ascii_digit()) =>
                {
                    text.push('.');
                    text.push_str(key);
                }
                Segment::Key(key) => text.push_str(&format!("[{key:?}]")),
            }
        }
        format!("`{text}`")
    }

    fn invalid<T>(&self, what: &str) -> Result<T, WalkFailure> {
        Err(WalkFailure::Invalid(format!("{} {what}", self.here())))
    }

    /// Charge `bytes` of SERIALISED output. Strings are charged as JSON writes them (escapes
    /// included), so the budget is reached before the result is ever serialised: a `"\0"`
    /// string grows six-fold when written, and charging its raw length let a result far past
    /// `max-output-mb` reach the serialiser (#777 review).
    fn charge(&mut self, bytes: usize) -> Result<(), WalkFailure> {
        self.used = self.used.saturating_add(bytes);
        if self.used > self.budget {
            return Err(WalkFailure::Stop(oversize(self.limits)));
        }
        Ok(())
    }

    fn tick(&mut self) -> Result<(), WalkFailure> {
        self.visited += 1;
        if self.visited.is_multiple_of(1024) {
            if self.cancel.load(Ordering::Acquire) {
                return Err(WalkFailure::Stop(cancelled()));
            }
            if Instant::now() >= self.deadline {
                return Err(WalkFailure::Stop(timed_out(self.limits)));
            }
        }
        Ok(())
    }

    fn value(
        &mut self,
        ctx: &Ctx<'js>,
        value: JsValue<'js>,
        depth: usize,
    ) -> Result<Value, WalkFailure> {
        self.tick()?;
        match value.type_of() {
            Type::Null => {
                self.charge(4)?;
                Ok(Value::Null)
            }
            Type::Bool => {
                self.charge(5)?;
                Ok(Value::Bool(value.as_bool().unwrap_or(false)))
            }
            Type::Int => {
                self.charge(11)?;
                Ok(Value::from(value.as_int().unwrap_or(0)))
            }
            Type::Float => {
                self.charge(24)?;
                self.number(value.as_float().unwrap_or(f64::NAN))
            }
            Type::String => {
                let text = value
                    .as_string()
                    .map(|s| s.to_string())
                    .transpose()
                    .map_err(|_| ())
                    .and_then(|s| s.ok_or(()));
                match text {
                    Ok(text) => {
                        self.charge(json_escaped_len(&text) + 2)?;
                        Ok(Value::String(text))
                    }
                    Err(()) => self.invalid(
                        "holds text that is not valid Unicode (an unpaired surrogate half)",
                    ),
                }
            }
            Type::Undefined | Type::Uninitialized => self.invalid("is undefined"),
            Type::Symbol => self.invalid("is a symbol"),
            Type::BigInt => self.invalid(
                "is a BigInt, which JSON cannot carry; convert it with Number() or String() first",
            ),
            Type::Function | Type::Constructor => self.invalid("is a function"),
            Type::Promise => self.invalid(
                "is a Promise; a script returns a finished value (there is nothing to await it)",
            ),
            Type::Exception => {
                self.invalid("is an Error object; return its message (error.message) instead")
            }
            Type::Proxy => self.invalid("is a Proxy"),
            Type::Array => self.array(ctx, value, depth),
            Type::Object => self.object(ctx, value, depth),
            Type::Module | Type::Unknown => self.invalid("is not a JSON value"),
        }
    }

    fn number(&self, number: f64) -> Result<Value, WalkFailure> {
        if number.is_nan() {
            return self.invalid("is NaN, which JSON cannot carry");
        }
        if number.is_infinite() {
            return self.invalid(if number > 0.0 {
                "is Infinity, which JSON cannot carry"
            } else {
                "is -Infinity, which JSON cannot carry"
            });
        }
        // Whole numbers inside the exactly-representable range stay integers (`2`, not `2.0`),
        // and -0 is 0, as JSON writes them.
        const EXACT: f64 = 9_007_199_254_740_992.0;
        if number.fract() == 0.0 && number.abs() <= EXACT {
            return Ok(Value::from(number as i64));
        }
        match serde_json::Number::from_f64(number) {
            Some(n) => Ok(Value::Number(n)),
            None => self.invalid("is not a finite number"),
        }
    }

    /// Enter a container: refuse a cycle or too deep a nesting, and remember it as an ancestor.
    fn enter(&mut self, value: &JsValue<'js>, depth: usize) -> Result<(), WalkFailure> {
        if depth >= MAX_RESULT_DEPTH {
            return self.invalid(&format!(
                "is nested more than {MAX_RESULT_DEPTH} levels deep"
            ));
        }
        if !self.ancestors.insert(value.clone()) {
            return self.invalid("refers back to a value that contains it (a cycle)");
        }
        Ok(())
    }

    fn keys(&self, object: &Object<'js>) -> Result<Vec<JsValue<'js>>, WalkFailure> {
        let keys: Array = self
            .builtins
            .own_keys
            .call((object.clone(),))
            .map_err(WalkFailure::Js)?;
        keys.iter::<JsValue>()
            .collect::<rquickjs::Result<Vec<_>>>()
            .map_err(WalkFailure::Js)
    }

    /// The own data value at `key`, refusing an accessor or a hidden property without running it.
    fn data_property(
        &self,
        object: &Object<'js>,
        key: JsValue<'js>,
        shown: &str,
    ) -> Result<Option<JsValue<'js>>, WalkFailure> {
        let descriptor: JsValue = self
            .builtins
            .own_descriptor
            .call((object.clone(), key))
            .map_err(WalkFailure::Js)?;
        let Some(descriptor) = descriptor.into_object() else {
            return Ok(None);
        };
        // Own keys only, through the captured `Reflect.ownKeys`: a descriptor object inherits from
        // `Object.prototype`, so `"value" in descriptor` is true for an accessor once a script
        // defines `Object.prototype.value`, and reading it would run that script's getter.
        let has_value = self.keys(&descriptor)?.iter().any(|key| {
            key.as_string().and_then(|s| s.to_string().ok()).as_deref() == Some("value")
        });
        if !has_value {
            return Err(WalkFailure::Invalid(format!(
                "{} has a getter/setter property {shown}; return plain data (a getter is not \
                 called)",
                self.here()
            )));
        }
        // `enumerable` and `value` are both own data properties of the descriptor here
        // (`getOwnPropertyDescriptor` defines all four of a data descriptor's fields).
        let enumerable: bool = descriptor.get("enumerable").map_err(WalkFailure::Js)?;
        if !enumerable {
            return Err(WalkFailure::Invalid(format!(
                "{} has a hidden (non-enumerable) property {shown}, which JSON would drop",
                self.here()
            )));
        }
        descriptor.get("value").map(Some).map_err(WalkFailure::Js)
    }

    fn object(
        &mut self,
        ctx: &Ctx<'js>,
        value: JsValue<'js>,
        depth: usize,
    ) -> Result<Value, WalkFailure> {
        let Some(object) = value.as_object().cloned() else {
            return self.invalid("is not an object");
        };
        match object.get_prototype() {
            None => {}
            Some(prototype) if prototype == self.builtins.object_prototype => {}
            Some(_) => {
                return self.invalid(
                    "is not a plain object (it is an instance of a class, such as a Map, Set, \
                     RegExp or Date); copy what you need into a plain object",
                );
            }
        }
        self.enter(&value, depth)?;
        let mut out = Map::new();
        self.charge(2)?;
        for key in self.keys(&object)? {
            let Some(name) = key.as_string().and_then(|s| s.to_string().ok()) else {
                return self.invalid("has a symbol-keyed property, which JSON would drop");
            };
            let Some(child) = self.data_property(&object, key, &format!("{name:?}"))? else {
                continue;
            };
            self.charge(json_escaped_len(&name) + 4)?;
            self.path.push(Segment::Key(name.clone()));
            let converted = self.value(ctx, child, depth + 1)?;
            self.path.pop();
            out.insert(name, converted);
        }
        self.ancestors.remove(&value);
        Ok(Value::Object(out))
    }

    fn array(
        &mut self,
        ctx: &Ctx<'js>,
        value: JsValue<'js>,
        depth: usize,
    ) -> Result<Value, WalkFailure> {
        let Some(array) = value.as_array().cloned() else {
            return self.invalid("is not an array");
        };
        let object = array.as_object().clone();
        if object.get_prototype().as_ref() != Some(&self.builtins.array_prototype) {
            return self.invalid("is not a plain array (it is a subclass of Array)");
        }
        self.enter(&value, depth)?;
        let len = array.len();
        self.charge(2)?;
        let mut out = Vec::with_capacity(len.min(1 << 20));
        for index in 0..len {
            let key = JsValue::new_number(ctx.clone(), index as f64);
            let Some(child) = self.data_property(&object, key, &format!("[{index}]"))? else {
                return self.invalid(&format!(
                    "is a sparse array: element {index} is missing (a hole), which JSON would \
                     turn into null"
                ));
            };
            self.charge(1)?;
            self.path.push(Segment::Index(index));
            let converted = self.value(ctx, child, depth + 1)?;
            self.path.pop();
            out.push(converted);
        }
        // An array's own keys are its indices plus `length`; anything more is a named or
        // symbol-keyed property that JSON would silently drop.
        let keys = self.keys(&object)?;
        if keys.len() > len + 1 {
            return self.invalid(
                "has a named (non-index) property besides its elements, which JSON would drop",
            );
        }
        self.ancestors.remove(&value);
        Ok(Value::Array(out))
    }
}

#[cfg(test)]
mod tests;
