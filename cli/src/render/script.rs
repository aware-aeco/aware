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
use std::sync::atomic::{AtomicBool, AtomicU8, Ordering};
use std::time::{Duration, Instant};

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
const LOG_TRUNCATED_NOTE: &str = "[console output truncated at 64 KB]";
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
    path: String,
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
    let run = run_start.unwrap_or_else(RunStart::now);
    let cancel = Arc::new(AtomicBool::new(false));
    let _cancel_on_drop = CancelOnDrop(Arc::clone(&cancel));
    let (tx, rx) = tokio::sync::oneshot::channel();
    let worker_cancel = Arc::clone(&cancel);
    std::thread::Builder::new()
        .name("aware-script".into())
        .stack_size(WORKER_STACK_BYTES)
        .spawn(move || {
            let _ = tx.send(run_blocking(&request, &run, &worker_cancel));
        })
        .map_err(|e| {
            AwareError::Internal(format!(
                "script run: could not start the script thread: {e}"
            ))
        })?;
    rx.await.unwrap_or_else(|_| {
        Err(AwareError::Internal(
            "script run: the script thread stopped without an answer (it panicked)".into(),
        ))
    })
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
                "bytes": file.bytes,
                "sha256": file.sha256,
            })
        })
        .collect();

    let deadline = Instant::now() + request.limits.timeout;
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
            path,
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
    let runtime = Runtime::new()
        .map_err(|e| AwareError::Internal(format!("script run: could not start QuickJS: {e}")))?;
    runtime.set_memory_limit(usize::try_from(limits.memory_mb * MIB).unwrap_or(usize::MAX));
    runtime.set_max_stack_size(JS_STACK_BYTES);
    let reason = Arc::new(AtomicU8::new(NOT_INTERRUPTED));
    {
        let reason = Arc::clone(&reason);
        let cancel = Arc::clone(cancel);
        runtime.set_interrupt_handler(Some(Box::new(move || {
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
    let result = outcome?;
    let serialised = serde_json::to_vec(&result)?.len() as u64;
    if serialised > limits.max_output_mb * MIB {
        return Err(oversize(limits));
    }
    let logs = Rc::try_unwrap(logs)
        .map(RefCell::into_inner)
        .unwrap_or_else(|shared| shared.borrow().clone())
        .finish();
    Ok((result, logs))
}

fn oversize(limits: &Limits) -> AwareError {
    invalid(format!(
        "the script's result is larger than `max-output-mb` ({} MB) once serialised; return \
         less, or raise `max-output-mb` (at most 128)",
        limits.max_output_mb
    ))
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
                let line = values
                    .0
                    .iter()
                    .map(|value| log_text(&ctx, value))
                    .collect::<Vec<_>>()
                    .join(" ");
                logs.borrow_mut().push(format!("{prefix}{line}"));
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
    fn push(&mut self, line: String) {
        if self.truncated {
            return;
        }
        let remaining = LOG_CAP_BYTES.saturating_sub(self.bytes);
        if line.len() <= remaining {
            self.bytes += line.len();
            self.lines.push(line);
            return;
        }
        let mut cut = remaining;
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
                        self.charge(text.len() + 2)?;
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
        let has_value = descriptor.contains_key("value").map_err(WalkFailure::Js)?;
        if !has_value {
            return Err(WalkFailure::Invalid(format!(
                "{} has a getter/setter property {shown}; return plain data (a getter is not \
                 called)",
                self.here()
            )));
        }
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
            self.charge(name.len() + 4)?;
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
