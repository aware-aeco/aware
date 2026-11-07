//! Contract tests for `script/run` (#777). Each rule in the module docs has a test here that
//! fails when the rule is broken; the adversarial ones run the hostile code for real.

use super::*;
use serde_json::json;

fn fixed_start() -> RunStart {
    RunStart {
        iso: "2026-11-07T09:30:00.000Z".into(),
        ms: 1_794_043_800_000,
    }
}

fn call(inputs: Value) -> Result<Value, AwareError> {
    let request = parse_request(&inputs)?;
    on_worker(request, Arc::new(AtomicBool::new(false)))
}

/// Run on a thread with the production worker's stack, as `script_run` does: the JavaScript
/// stack limit is sized for that thread, not for a test harness thread.
fn on_worker(request: Request, cancel: Arc<AtomicBool>) -> Result<Value, AwareError> {
    std::thread::Builder::new()
        .stack_size(WORKER_STACK_BYTES)
        .spawn(move || run_blocking(&request, &fixed_start(), &cancel))
        .unwrap()
        .join()
        .unwrap()
}

fn js(code: &str) -> Value {
    json!({ "language": "javascript", "code": code })
}

fn js_with(code: &str, extra: Value) -> Value {
    let mut inputs = js(code);
    if let (Some(base), Value::Object(more)) = (inputs.as_object_mut(), extra) {
        base.extend(more);
    }
    inputs
}

fn result_of(code: &str) -> Value {
    match call(js(code)) {
        Ok(out) => out["result"].clone(),
        Err(e) => panic!("script failed: {e}\ncode: {code}"),
    }
}

fn error_of(inputs: Value) -> String {
    match call(inputs) {
        Ok(out) => panic!("expected a failed node, got {out}"),
        Err(e) => {
            assert!(
                matches!(e, AwareError::Validation(_)),
                "a script failure is a Validation error: {e:?}"
            );
            e.to_string()
        }
    }
}

#[test]
fn the_return_value_is_the_result_and_the_run_is_recorded() {
    let out = call(js_with(
        "return { sum: args.a + args.b, re: /b+/.test('abbc') };",
        json!({ "args": { "a": 2, "b": 3 } }),
    ))
    .unwrap();
    assert_eq!(out["ok"], true);
    assert_eq!(out["result"], json!({ "sum": 5, "re": true }));
    assert_eq!(out["files"], json!([]));
    assert!(
        out.get("logs").is_none(),
        "no logs key without console output"
    );
    assert_eq!(out["run"]["startedAt"], "2026-11-07T09:30:00.000Z");
    assert_eq!(out["run"]["startedAtMs"], 1_794_043_800_000i64);
    assert_eq!(out["engine"]["name"], ENGINE_NAME);
    assert_eq!(out["engine"]["version"], ENGINE_VERSION);
    assert_eq!(out["engine"]["rquickjs"], RQUICKJS_VERSION);
    assert_eq!(out["engine"]["aware-cli"], env!("CARGO_PKG_VERSION"));
}

/// The recorded engine is the one the lock pins. Moving `rquickjs` moves the bundled QuickJS-ng,
/// so this trips until `ENGINE_VERSION` (read from the new `quickjs.h`) moves with it.
#[test]
fn the_recorded_engine_matches_the_lock() {
    let lock = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("Cargo.lock"),
    )
    .unwrap()
    .replace("\r\n", "\n");
    for crate_name in ["rquickjs", "rquickjs-core", "rquickjs-sys"] {
        let entry = format!("name = \"{crate_name}\"\nversion = \"{RQUICKJS_VERSION}\"");
        assert!(
            lock.contains(&entry),
            "Cargo.lock must pin {crate_name} {RQUICKJS_VERSION}"
        );
    }
    let bundled = match RQUICKJS_VERSION {
        "0.14.0" => "0.16.2",
        other => {
            panic!("rquickjs {other}: read QJS_VERSION_* from its quickjs.h into ENGINE_VERSION")
        }
    };
    assert_eq!(ENGINE_VERSION, bundled);
}

/// Text handed in as data can never become code: every route from a string to a function is
/// refused in the context the script runs in.
#[test]
fn eval_and_every_function_constructor_throw() {
    let routes = [
        "eval('1')",
        "(0, eval)('1')",
        "globalThis.eval('1')",
        "new Function('return 1')()",
        "Function('return 1')()",
        "(function () {}).constructor('return 1')()",
        "Function.prototype.constructor('return 1')()",
        "(async function () {}).constructor('return 1')",
        "(function* () {}).constructor('yield 1')",
        "(async function* () {}).constructor('yield 1')",
        "(() => {}).constructor('return 1')()",
        "Reflect.construct(Function, ['return 1'])()",
        "new (Object.getPrototypeOf(function* () {}).constructor)('yield 1')",
    ];
    for route in routes {
        let code = format!(
            "try {{ {route}; return 'ran'; }} catch (e) {{ return String(e && e.message); }}"
        );
        let answer = result_of(&code);
        assert_ne!(answer, "ran", "{route} must not run code");
        assert!(
            answer
                .as_str()
                .is_some_and(|m| m.contains("eval is not supported")
                    || m.contains("not a function")
                    || m.contains("not defined")),
            "{route}: unexpected refusal {answer}"
        );
    }
    // Uncaught, it fails the node.
    let error = error_of(js("return eval('40 + 2');"));
    assert!(error.contains("threw"), "{error}");
    // An AI answer or any other data in `args` stays data.
    let answer = result_of_with(
        "try { return Function(args.payload)(); } catch (e) { return 'refused'; }",
        json!({ "payload": "return 'executed'" }),
    );
    assert_eq!(answer, "refused");
}

fn result_of_with(code: &str, args: Value) -> Value {
    call(js_with(code, json!({ "args": args }))).unwrap()["result"].clone()
}

#[test]
fn there_is_no_ambient_clock_randomness_locale_or_host_access() {
    let answer = result_of(
        "return ['Date', 'performance', 'Intl', 'WeakRef', 'FinalizationRegistry', 'require', \
         'fetch', 'setTimeout', 'setInterval', 'process', 'std', 'os', 'print', 'Proxy']\
         .map(n => n + ':' + typeof globalThis[n]);",
    );
    for entry in answer.as_array().unwrap() {
        let entry = entry.as_str().unwrap();
        assert!(
            entry.ends_with(":undefined"),
            "{entry} must not be reachable"
        );
    }
    assert_eq!(result_of("return typeof Date;"), "undefined");
}

#[test]
fn math_random_and_the_locale_methods_throw_and_the_originals_are_unreachable() {
    let probes = [
        ("Math.random()", "Math.random is not available"),
        ("(1.5).toLocaleString()", "Number.prototype.toLocaleString"),
        ("[1, 2].toLocaleString()", "Number.prototype.toLocaleString"),
        ("(10n).toLocaleString()", "BigInt.prototype.toLocaleString"),
        ("'a'.localeCompare('b')", "String.prototype.localeCompare"),
        (
            "'i'.toLocaleUpperCase()",
            "String.prototype.toLocaleUpperCase",
        ),
        (
            "'I'.toLocaleLowerCase()",
            "String.prototype.toLocaleLowerCase",
        ),
        (
            "Object.getOwnPropertyDescriptor(Math, 'random').value.call(Math)",
            "Math.random is not available",
        ),
        (
            "Reflect.apply(Number.prototype.toLocaleString, 3, [])",
            "Number.prototype.toLocaleString",
        ),
        (
            "Object.getPrototypeOf('').localeCompare.call('a', 'b')",
            "String.prototype.localeCompare",
        ),
    ];
    for (probe, expected) in probes {
        let code =
            format!("try {{ {probe}; return 'ran'; }} catch (e) {{ return String(e.message); }}");
        let answer = result_of(&code);
        assert!(
            answer.as_str().is_some_and(|m| m.contains(expected)),
            "{probe}: expected a refusal naming {expected}, got {answer}"
        );
    }
    // Deleting the replacement exposes nothing underneath: the original is gone, not shadowed.
    assert_eq!(
        result_of("delete Math.random; return typeof Math.random;"),
        "undefined"
    );
    assert_eq!(
        result_of(
            "delete Number.prototype.toLocaleString; \
             return (1).toLocaleString === Object.prototype.toLocaleString;"
        ),
        true,
        "after delete the lookup falls through to Object.prototype, never to the original"
    );
    // The message tells the author what to do instead.
    let error = error_of(js("return Math.random();"));
    assert!(error.contains("seed in `args`"), "{error}");
}

#[test]
fn run_is_the_recorded_start_and_frozen() {
    let answer = result_of(
        "return [run.startedAt, run.startedAtMs, Object.isFrozen(run), \
         Object.keys(run).join(',')];",
    );
    assert_eq!(
        answer,
        json!([
            "2026-11-07T09:30:00.000Z",
            1_794_043_800_000i64,
            true,
            "startedAt,startedAtMs"
        ])
    );
    let start = RunStart::at(
        chrono::DateTime::parse_from_rfc3339("2026-11-07T09:30:00.123456Z")
            .unwrap()
            .with_timezone(&chrono::Utc),
    );
    assert_eq!(start.iso, "2026-11-07T09:30:00.123Z");
    assert_eq!(start.ms, 1_794_043_800_123);
}

#[test]
fn args_is_a_frozen_deep_copy() {
    let answer = result_of_with(
        "const out = [Object.isFrozen(args), Object.isFrozen(args.a), Object.isFrozen(args.a.list)];\n\
         try { args.a.b = 2; out.push('assigned'); } catch (e) { out.push(e.constructor.name); }\n\
         try { args.a.list.push(4); out.push('pushed'); } catch (e) { out.push(e.constructor.name); }\n\
         try { args = {}; out.push('rebound'); } catch (e) { out.push(e.constructor.name); }\n\
         out.push(args.a.b, args.a.list.length);\n\
         return out;",
        json!({ "a": { "b": 1, "list": [1, 2, 3] } }),
    );
    assert_eq!(
        answer,
        json!([
            true,
            true,
            true,
            "TypeError",
            "TypeError",
            "TypeError",
            1,
            3
        ])
    );
}

fn temp_file(dir: &std::path::Path, name: &str, body: &[u8]) -> String {
    let path = dir.join(name);
    std::fs::write(&path, body).unwrap();
    path.to_str().unwrap().to_owned()
}

#[test]
fn files_are_handed_over_as_text_and_lazy_json_with_receipts_only_in_the_output() {
    let dir = tempfile::tempdir().unwrap();
    let marker = "UNIQUE-CONTENT-MARKER-4f2a";
    let design = temp_file(
        dir.path(),
        "design.json",
        format!("{{\"members\": [1, 2, 3], \"note\": \"{marker}\"}}").as_bytes(),
    );
    let broken = temp_file(dir.path(), "broken.json", b"{ not json");
    let out = call(js_with(
        "return { count: files.design.json.members.length, \
                  textLength: files.design.text.length, \
                  bytes: files.design.bytes, sha: files.design.sha256, \
                  same: files.design.json === files.design.json, \
                  frozen: Object.isFrozen(files) && !Object.isExtensible(files.broken) \
                      && (() => { try { files.design.text = ''; return false; } \
                                  catch (e) { return files.design.text.length > 0; } })(), \
                  brokenText: files.broken.text.length };",
        json!({ "files": { "design": design, "broken": broken } }),
    ))
    .unwrap();
    let digest = format!("{:x}", Sha256::digest(std::fs::read(&design).unwrap()));
    assert_eq!(out["result"]["count"], 3);
    assert_eq!(
        out["result"]["same"], true,
        "json is parsed once and cached"
    );
    assert_eq!(out["result"]["frozen"], true);
    assert_eq!(out["result"]["sha"], digest.as_str());
    assert_eq!(
        out["result"]["brokenText"], 10,
        "an unparsable file is fine until its json is read"
    );
    let receipts = out["files"].as_array().unwrap();
    assert_eq!(receipts.len(), 2);
    let receipt = receipts.iter().find(|r| r["name"] == "design").unwrap();
    assert_eq!(
        receipt.as_object().unwrap().keys().collect::<Vec<_>>(),
        ["name", "path", "resolvedPath", "bytes", "sha256"],
        "a receipt carries no contents"
    );
    assert_eq!(receipt["sha256"], digest.as_str());
    assert_eq!(
        receipt["path"],
        design.as_str(),
        "the approved path, as written"
    );
    assert_eq!(
        receipt["resolvedPath"],
        crate::fs::canonicalise_readable_file(std::path::Path::new(&design))
            .unwrap()
            .path
            .display()
            .to_string()
    );
    assert!(
        !out.to_string().contains(marker),
        "file contents never reach the output unless the script returns them"
    );

    // The difference: a script that returns file-derived data on purpose gets it traced.
    let out = call(js_with(
        "return files.design.json.note;",
        json!({ "files": { "design": design } }),
    ))
    .unwrap();
    assert_eq!(out["result"], marker);

    // Reading the broken file's json fails the node, naming the file.
    let error = error_of(js_with(
        "return files.broken.json;",
        json!({ "files": { "broken": broken } }),
    ));
    assert!(error.contains("files.broken.json"), "{error}");
    assert!(error.contains("not valid JSON"), "{error}");
}

#[test]
fn a_bom_is_skipped_for_json_but_kept_in_text_and_non_utf8_is_refused() {
    let dir = tempfile::tempdir().unwrap();
    let bom = temp_file(dir.path(), "bom.json", "\u{feff}{\"a\": 1}".as_bytes());
    let out = call(js_with(
        "return [files.f.json.a, files.f.text.charCodeAt(0)];",
        json!({ "files": { "f": bom } }),
    ))
    .unwrap();
    assert_eq!(out["result"], json!([1, 0xfeff]));

    let latin1 = temp_file(dir.path(), "latin1.txt", &[0x63, 0x61, 0x66, 0xe9]);
    let error = error_of(js_with("return 1;", json!({ "files": { "f": latin1 } })));
    assert!(error.contains("not UTF-8"), "{error}");
    assert!(error.contains("\"f\""), "{error}");
}

#[test]
fn file_paths_that_are_not_literal_absolute_files_are_refused() {
    let dir = tempfile::tempdir().unwrap();
    let real = temp_file(dir.path(), "real.json", b"{}");
    let traversal = format!(
        "{}{}..{}real.json",
        dir.path().display(),
        std::path::MAIN_SEPARATOR,
        std::path::MAIN_SEPARATOR
    );
    let cases = [
        (json!("data/real.json"), "relative path"),
        (json!("{{ inputs.design_file }}"), "template"),
        (json!("{% raw %}x{% endraw %}"), "template"),
        (json!(traversal), "`..` step"),
        (json!(r"\\.\pipe\aware-script"), "device path"),
        (json!(r"\\?\C:\Windows\win.ini"), "device path"),
        (json!(42), "not given as a text path"),
        (json!(dir.path().to_str().unwrap()), "is a directory"),
        (
            json!(dir.path().join("missing.json").to_str().unwrap()),
            "does not exist",
        ),
    ];
    for (path, expected) in cases {
        let error = error_of(js_with("return 1;", json!({ "files": { "f": path } })));
        assert!(error.contains(expected), "{path}: {error}");
    }
    let error = error_of(js_with(
        "return 1;",
        json!({ "files": { "__proto__": real } }),
    ));
    assert!(error.contains("not a plain identifier"), "{error}");
    let many: Map<String, Value> = (0..17)
        .map(|i| (format!("f{i}"), Value::String(real.clone())))
        .collect();
    let error = error_of(js_with("return 1;", json!({ "files": many })));
    assert!(error.contains("at most 16"), "{error}");
    let error = error_of(js_with("return 1;", json!({ "files": ["a"] })));
    assert!(error.contains("must map each name"), "{error}");
}

#[test]
fn the_input_budget_covers_args_and_files() {
    let dir = tempfile::tempdir().unwrap();
    let big = temp_file(dir.path(), "big.txt", &vec![b'a'; 2 * 1024 * 1024]);
    let error = error_of(js_with(
        "return 1;",
        json!({ "files": { "big": big }, "max-input-mb": 1 }),
    ));
    assert!(error.contains("max-input-mb"), "{error}");
    let error = error_of(js_with(
        "return 1;",
        json!({ "args": { "blob": "x".repeat(2 * 1024 * 1024) }, "max-input-mb": 1 }),
    ));
    assert!(error.contains("`args`"), "{error}");
}

#[test]
fn a_twenty_megabyte_json_file_is_read_and_parsed() {
    let dir = tempfile::tempdir().unwrap();
    let mut body = String::from("{\"members\":[");
    let mut count = 0u64;
    while body.len() < 20 * 1024 * 1024 {
        if count > 0 {
            body.push(',');
        }
        body.push_str(&format!(
            "{{\"id\":\"M-{count}\",\"profile\":\"W12X26\",\"x\":{count}.5,\"y\":-{count},\"z\":3000,\
             \"tags\":[\"steel\",\"beam\"],\"length_mm\":6096}}"
        ));
        count += 1;
    }
    body.push_str("]}");
    let path = temp_file(dir.path(), "model.json", body.as_bytes());
    let out = call(js_with(
        "const m = files.model.json.members; let sum = 0; \
         for (const e of m) sum += e.length_mm; \
         return { count: m.length, sum, last: m[m.length - 1].id };",
        json!({ "files": { "model": path }, "timeout-seconds": 300 }),
    ))
    .unwrap();
    assert_eq!(out["result"]["count"], count);
    assert_eq!(out["result"]["sum"], count * 6096);
    assert_eq!(out["result"]["last"], format!("M-{}", count - 1));
    assert!(out["files"][0]["bytes"].as_u64().unwrap() >= 20 * 1024 * 1024);
}

#[test]
fn an_infinite_loop_is_stopped_at_the_deadline() {
    let started = Instant::now();
    let error = error_of(js_with("while (true) {}", json!({ "timeout-seconds": 1 })));
    assert!(error.contains("did not finish within 1 seconds"), "{error}");
    assert!(
        started.elapsed() < Duration::from_secs(30),
        "stopped promptly: {:?}",
        started.elapsed()
    );
    // A script cannot catch its way past the deadline.
    let error = error_of(js_with(
        "for (;;) { try { while (true) {} } catch (e) {} }",
        json!({ "timeout-seconds": 1 }),
    ));
    assert!(error.contains("did not finish"), "{error}");
}

#[test]
fn cancellation_stops_a_running_script() {
    let request = parse_request(&js_with(
        "while (true) {}",
        json!({ "timeout-seconds": 600 }),
    ))
    .unwrap();
    let cancel = Arc::new(AtomicBool::new(false));
    let flag = Arc::clone(&cancel);
    let stopper = std::thread::spawn(move || {
        std::thread::sleep(Duration::from_millis(300));
        flag.store(true, Ordering::Release);
    });
    let started = Instant::now();
    let error = on_worker(request, cancel).unwrap_err();
    stopper.join().unwrap();
    assert!(error.to_string().contains("stopped"), "{error}");
    assert!(started.elapsed() < Duration::from_secs(30));
    // Dropping the awaiting future is what raises the flag in a run.
    let flag = Arc::new(AtomicBool::new(false));
    drop(CancelOnDrop(Arc::clone(&flag)));
    assert!(flag.load(Ordering::Acquire));
}

#[test]
fn running_out_of_memory_fails_the_node_without_aborting() {
    let error = error_of(js_with(
        "const keep = []; for (let i = 0; ; i++) keep.push(new Array(100000).fill(i));",
        json!({ "memory-mb": 16 }),
    ));
    assert!(error.contains("ran out of memory"), "{error}");
    assert!(error.contains("16 MB"), "{error}");
}

#[test]
fn deep_recursion_is_a_stack_overflow_error() {
    let error = error_of(js("function f(n) { return f(n + 1) + 1; } return f(0);"));
    assert!(error.contains("recursed too deeply"), "{error}");
    // Deeply nested JSON in a file is refused the same way, not by a crashed thread.
    let dir = tempfile::tempdir().unwrap();
    let nested = temp_file(
        dir.path(),
        "nested.json",
        format!("{}{}", "[".repeat(2_000_000), "]".repeat(2_000_000)).as_bytes(),
    );
    let error = error_of(js_with(
        "return files.n.json.length;",
        json!({ "files": { "n": nested } }),
    ));
    assert!(
        error.contains("recursed too deeply") || error.contains("not valid JSON"),
        "{error}"
    );
}

#[test]
fn every_invalid_result_kind_is_refused_by_name() {
    let cases = [
        ("const nothing = 1;", "returned nothing"),
        ("return undefined;", "returned nothing"),
        ("return { a: undefined };", "`result.a` is undefined"),
        ("return { f() {} };", "`result.f` is a function"),
        ("return [Symbol('s')];", "`result[0]` is a symbol"),
        ("return { n: 10n };", "is a BigInt"),
        ("return { x: NaN };", "is NaN"),
        ("return [Infinity];", "is Infinity"),
        ("return [-Infinity];", "is -Infinity"),
        ("const o = {}; o.self = o; return o;", "a cycle"),
        ("const a = []; a.push({ back: a }); return a;", "a cycle"),
        ("return Promise.resolve(1);", "is a Promise"),
        ("return { m: new Map() };", "is not a plain object"),
        ("return /re/;", "is not a plain object"),
        (
            "class Point { constructor() { this.x = 1; } } return new Point();",
            "is not a plain object",
        ),
        ("return new Error('e');", "is an Error object"),
        (
            "return { get x() { return 1; } };",
            "getter/setter property",
        ),
        ("return [1, , 3];", "sparse array"),
        (
            "const a = [1]; a.extra = 2; return a;",
            "named (non-index) property",
        ),
        ("return { [Symbol('k')]: 1 };", "symbol-keyed property"),
        (
            "const o = {}; Object.defineProperty(o, 'h', { value: 1, enumerable: false }); return o;",
            "hidden (non-enumerable) property",
        ),
        ("return { s: '\\uD800' };", "not valid Unicode"),
        (
            "class A extends Array {} return A.from([1]);",
            "not a plain array",
        ),
    ];
    for (code, expected) in cases {
        let error = error_of(js(code));
        assert!(
            error.contains(expected),
            "{code:?}: expected {expected:?} in: {error}"
        );
    }
}

#[test]
fn a_hostile_getter_is_refused_without_being_run_and_to_json_is_never_called() {
    let started = Instant::now();
    let error = error_of(js_with(
        "return { get trap() { while (true) {} } };",
        json!({ "timeout-seconds": 600 }),
    ));
    assert!(error.contains("getter/setter"), "{error}");
    assert!(
        started.elapsed() < Duration::from_secs(30),
        "the getter never ran"
    );
    let answer = result_of(
        "Object.defineProperty(Object.prototype, 'toJSON', \
         { value() { return 'hijacked'; }, configurable: true }); return { a: 1 };",
    );
    assert_eq!(answer, json!({ "a": 1 }));
}

#[test]
fn numbers_are_written_as_json_writes_them() {
    assert_eq!(
        result_of("return [2.0, -0, 1.5, 2 ** 53, 2 ** 60, -7];"),
        json!([
            2,
            0,
            1.5,
            9_007_199_254_740_992i64,
            1_152_921_504_606_846_976.0,
            -7
        ])
    );
    assert_eq!(
        result_of("return Object.assign(Object.create(null), { a: [null, true, 'x'] });"),
        json!({ "a": [null, true, "x"] }),
        "a null-prototype object is plain"
    );
}

#[test]
fn deep_and_oversize_results_are_refused() {
    let error = error_of(js(
        "let v = 1; for (let i = 0; i < 1000; i++) v = { v }; return v;",
    ));
    assert!(
        error.contains("nested more than 256 levels deep"),
        "{error}"
    );
    let error = error_of(js_with(
        "return 'x'.repeat(2 * 1024 * 1024);",
        json!({ "max-output-mb": 1 }),
    ));
    assert!(error.contains("max-output-mb"), "{error}");
    let error = error_of(js_with(
        "const a = new Array(200000).fill('abcdefghij'); return a;",
        json!({ "max-output-mb": 1 }),
    ));
    assert!(error.contains("max-output-mb"), "{error}");
}

#[test]
fn console_output_goes_to_the_logs_capped_at_64_kb() {
    let out = call(js(
        "console.log('hello', 1, { a: [1] }, undefined); console.warn('careful'); return 0;",
    ))
    .unwrap();
    assert_eq!(
        out["logs"],
        json!(["hello 1 {\"a\":[1]} undefined", "warning: careful"])
    );
    let out = call(js(
        "for (let i = 0; i < 1000; i++) console.log('é'.repeat(100)); return 0;",
    ))
    .unwrap();
    let logs = out["logs"].as_array().unwrap();
    assert_eq!(logs.last().unwrap(), LOG_TRUNCATED_NOTE);
    let bytes: usize = logs
        .iter()
        .take(logs.len() - 1)
        .map(|l| l.as_str().unwrap().len())
        .sum();
    // Each kept line is charged its newline too.
    let charged = bytes + (logs.len() - 1);
    assert!(charged <= LOG_CAP_BYTES, "{charged}");
    assert!(charged > LOG_CAP_BYTES - 300, "{charged}");
}

/// #777 review (Codex P1): an empty line used to cost nothing, so `console.log()` in a loop grew
/// the buffer outside QuickJS's memory limit without ever reaching the cap.
#[test]
fn empty_console_lines_still_use_up_the_cap() {
    let out = call(js_with(
        "for (let i = 0; i < 1000000; i++) console.log(); return 0;",
        json!({ "memory-mb": 16 }),
    ))
    .unwrap();
    let logs = out["logs"].as_array().unwrap();
    assert_eq!(logs.last().unwrap(), LOG_TRUNCATED_NOTE);
    assert!(logs.len() <= LOG_CAP_LINES + 1, "{}", logs.len());
}

/// #777 review (Codex P1): an interrupt inside `console.log`'s formatting (a looping `toJSON`)
/// was caught and replaced by a placeholder, so the script went on and its result was accepted
/// after the deadline. The run must fail as timed out instead.
#[test]
fn a_deadline_hit_inside_console_formatting_still_fails_the_run() {
    let started = Instant::now();
    let error = error_of(js_with(
        "console.log({ toJSON() { while (true) {} } }); return 42;",
        json!({ "timeout-seconds": 1 }),
    ));
    assert!(error.contains("did not finish within 1 seconds"), "{error}");
    assert!(started.elapsed() < Duration::from_secs(30));
}

/// #777 review (Codex P2): a descriptor object inherits from `Object.prototype`, so a script
/// that defined `Object.prototype.value` made an accessor look like data — and its getter ran.
#[test]
fn a_polluted_prototype_cannot_pass_an_accessor_off_as_data() {
    let error = error_of(js(
        "const result = { get x() { throw Error('the getter ran'); } }; \
         Object.defineProperty(Object.prototype, 'value', { get() { return 7; } }); \
         return result;",
    ));
    assert!(error.contains("getter/setter"), "{error}");
    assert!(!error.contains("the getter ran"), "{error}");
}

/// #777 review (Codex P2): the script sees the approved literal path, never where it resolved,
/// so retargeting a symlink between runs cannot change the result while every input the
/// guarantee names stays the same.
#[test]
fn the_script_sees_the_approved_path_not_where_it_resolved() {
    let dir = tempfile::tempdir().unwrap();
    let real = temp_file(dir.path(), "real.json", b"{}");
    // A dotted spelling is refused by the rule, so use a link where the platform allows one.
    #[cfg(unix)]
    let approved = {
        let link = dir.path().join("current.json");
        std::os::unix::fs::symlink(&real, &link).unwrap();
        link.display().to_string()
    };
    #[cfg(not(unix))]
    let approved = real.clone();
    let out = call(js_with(
        "return files.f.path;",
        json!({ "files": { "f": approved } }),
    ))
    .unwrap();
    assert_eq!(out["result"], approved.as_str());
    assert_eq!(out["files"][0]["path"], approved.as_str());
}

#[test]
fn a_throw_or_a_syntax_error_names_what_went_wrong() {
    let error = error_of(js("const x = 1;\nthrow new RangeError('boom');"));
    assert!(error.contains("threw RangeError: boom"), "{error}");
    assert!(error.contains("at line 2"), "{error}");
    let error = error_of(js("throw 'plain text';"));
    assert!(error.contains("not an Error: plain text"), "{error}");
    let error = error_of(js("return {;"));
    assert!(error.contains("does not compile"), "{error}");
    // Escaping the function body cannot reach a module loader: there is none.
    let error = error_of(js("}\nimport x from 'fs';\nexport function y() {"));
    assert!(
        error.contains("does not compile") || error.contains("module"),
        "{error}"
    );
}

#[test]
fn the_call_contract_is_checked_before_anything_runs() {
    let error = error_of(json!({ "code": "return 1;" }));
    assert!(error.contains("`language` is required"), "{error}");
    let error = error_of(json!({ "language": "python", "code": "return 1" }));
    assert!(error.contains("not supported"), "{error}");
    let error = error_of(json!({ "language": "javascript", "code": "  " }));
    assert!(error.contains("`code` is required"), "{error}");
    let error = error_of(js_with("return 1;", json!({ "args": [1, 2] })));
    assert!(error.contains("`args` must be a mapping"), "{error}");
    for (key, value) in [
        ("timeout-seconds", json!(0)),
        ("timeout-seconds", json!(601)),
        ("timeout-seconds", json!("60")),
        ("memory-mb", json!(4097)),
        ("memory-mb", json!(8)),
        ("memory-mb", json!(1.5)),
        ("max-input-mb", json!(513)),
        ("max-output-mb", json!(129)),
    ] {
        let error = error_of(js_with("return 1;", json!({ key: value })));
        assert!(
            error.contains(&format!("`{key}`")),
            "{key}={value}: {error}"
        );
    }
}

/// Whatever a script leaves behind — pending jobs, a never-settled promise, cycles, a held
/// accessor — the runtime is torn down cleanly; a leak here is a QuickJS assertion that would
/// abort the whole CLI process, not fail one node.
#[test]
fn leftovers_never_abort_the_teardown() {
    let out = call(js("const a = {}; const b = { a }; a.b = b;\n\
         new Promise(() => {}).then(() => 1);\n\
         Promise.resolve().then(() => { while (true) {} });\n\
         const getter = Object.getOwnPropertyDescriptor(files, 'none');\n\
         globalThis.keep = [a, b, getter, () => a];\n\
         return 'done';"))
    .unwrap();
    assert_eq!(out["result"], "done");
    let dir = tempfile::tempdir().unwrap();
    let file = temp_file(dir.path(), "f.json", b"{\"x\": 1}");
    for code in [
        "globalThis.held = files.f; return 1;",
        "globalThis.held = files.f.json; return files.f.json.x;",
        "const d = Object.getOwnPropertyDescriptor(files.f, 'json'); globalThis.g = d.get; return 2;",
        "throw files.f;",
    ] {
        let _ = call(js_with(code, json!({ "files": { "f": file.clone() } })));
    }
}

#[tokio::test]
async fn the_async_entry_runs_on_its_worker_and_answers() {
    let out = script_run(js("return 6 * 7;"), Some(fixed_start()))
        .await
        .unwrap();
    assert_eq!(out["result"], 42);
    let error = script_run(js("throw new Error('nope');"), None)
        .await
        .unwrap_err();
    assert!(error.to_string().contains("nope"), "{error}");
}

/// #777 review (Codex P2): the walk charges a string as JSON writes it, so the output budget is
/// reached before anything is serialised. Pinned against `serde_json` itself, byte for byte.
#[test]
fn the_output_budget_charges_strings_as_json_writes_them() {
    let mut samples: Vec<String> = (0u8..128).map(|b| char::from(b).to_string()).collect();
    samples.push("plain é ✓ 𝄞 \"quoted\" back\\slash \u{7f}".into());
    for text in samples {
        let written = serde_json::to_string(&text).unwrap().len() - 2;
        assert_eq!(json_escaped_len(&text), written, "{text:?}");
    }
    // A string that grows six-fold when written is refused by the walk on its written size.
    let error = error_of(js_with(
        "return '\\0'.repeat(200000);",
        json!({ "max-output-mb": 1 }),
    ));
    assert!(error.contains("max-output-mb"), "{error}");
}

/// #777 review (Codex P1): every argument used to be copied out of QuickJS and joined before the
/// cap applied, so a spread of one large string asked for gigabytes outside the memory limit.
#[test]
fn a_huge_console_line_is_cut_while_it_is_built() {
    let started = Instant::now();
    let out = call(js_with(
        "const s = 'x'.repeat(1024 * 1024); console.log(...Array(10000).fill(s)); return 42;",
        json!({ "memory-mb": 64 }),
    ))
    .unwrap();
    assert_eq!(out["result"], 42);
    let logs = out["logs"].as_array().unwrap();
    assert_eq!(logs.last().unwrap(), LOG_TRUNCATED_NOTE);
    // Cut while built, this takes milliseconds; copying all 10 GB first took ~24 s here.
    assert!(
        started.elapsed() < Duration::from_secs(5),
        "{:?}",
        started.elapsed()
    );
}
