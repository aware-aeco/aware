//! Nothing on the run path re-reads `agents/` (#626).
//!
//! An approved app runs on the exact agent bytes its lock approved: `aware app
//! run` resolves every pinned agent ONCE, at preflight, to an immutable store
//! package (`agent_resolution::resolve_agents`), and every later read — every
//! preflight, every transport, every built-in helper, every nested app-backed
//! dispatch — must take its manifest and root from that one catalogue. A single
//! `load_agent_by_id(agents_dir, id)` left on the run path would quietly run a
//! freshly updated agent's manifest against the approved one's bytes.
//!
//! This is a text scan of the REAL source tree, in the spirit of
//! `agent_id_joins_are_fenced.rs`: it catches the call shapes below, not every
//! conceivable way to read a file. It carries a negative control, because a
//! scanner whose patterns have rotted passes exactly as quietly as a clean tree.
//!
//! Exempt, deliberately and by name:
//! * `--simulate`, which dispatches nothing and builds no catalogue —
//!   `commands/app.rs::simulate_nested_malformed_requires`, plus the one
//!   `AgentCatalogue::working_copies` construction in `run`'s simulate branch;
//! * `runtime/probe.rs` — `aware agent probe` checks an installed agent's
//!   connection; it is not lock-bound and never part of an app run;
//! * `runtime/agent_call.rs::read_installed` — ONE function, read only by the
//!   `aware agent call-capabilities` / `aware agent call` verbs (#618), which pin
//!   the installed manifest's SHA-256 a host confirmed and are never part of an
//!   app run. The same file's workflow route (`run_for_workflow*`) is given its
//!   manifest from the run's catalogue. The guard proves that every function
//!   of the file that reaches the exempt one, directly or through other
//!   functions of the file, is a named verb function, and that no other
//!   scanned file calls those verbs (`agent_call::capabilities…` /
//!   `agent_call::call…`), so a run-path caller at any depth trips it;
//! * `#[cfg(test)]` modules, which build their own fixtures.

use std::path::{Path, PathBuf};

/// Calls that read an agent's manifest or the agent catalogue from `agents/`.
const FORBIDDEN: &[&str] = &[
    "load_agent_by_id(",
    "discover_agents(",
    "discover_agents_in(",
    "agent_manifest_path(",
    "loader::load_agent(",
    // The `aware agent call-capabilities` / `call` verbs read `agents/` through
    // the one exempt function below; a run-path file calling them would too.
    "agent_call::capabilities",
    "agent_call::call",
];

/// The `commands/app.rs` functions that make up the run path.
const APP_RUN_PATH_FNS: &[&str] = &[
    "run",
    "app_uses_model_reader",
    "nested_malformed_requires",
    "resolve_run_agents",
];

/// Files on the run path that are not under `runtime/`.
const EXTRA_RUN_PATH_FILES: &[&str] = &["render/blender.rs"];

/// Files under `runtime/` that are not part of an app run.
const RUNTIME_EXEMPT: &[&str] = &["runtime/probe.rs", "runtime/probe_tests.rs"];

/// One function of a scanned runtime file that may read `agents/`, and the only
/// functions of that file allowed to reach it, directly or transitively:
/// (file, exempt fn, its verb callers). Everything else in the file is scanned
/// as usual.
const RUNTIME_FN_EXEMPT: &[(&str, &str, &[&str])] = &[(
    "runtime/agent_call.rs",
    "read_installed",
    &[
        "capabilities",
        "capabilities_with",
        "capabilities_within",
        "call",
        "call_with",
        "call_within",
    ],
)];

/// Whether `body` calls the free function `name`: `name(` not preceded by an
/// identifier character (so `call(` is not found inside `read_and_call(`), not
/// a definition, and not a method call `x.name(` — the exempt function and its
/// verb callers are free functions, and ureq's `request.call()` is not one.
fn calls(body: &str, name: &str) -> bool {
    body.match_indices(&format!("{name}(")).any(|(at, _)| {
        let before = &body[..at];
        !before
            .chars()
            .next_back()
            .is_some_and(|c| c.is_ascii_alphanumeric() || c == '_' || c == '.')
            && !before.trim_end().ends_with("fn")
    })
}

/// Every function name defined in `code`.
fn fn_names(code: &str) -> Vec<String> {
    let mut names = Vec::new();
    for (at, _) in code.match_indices("fn ") {
        let starts_word = !code[..at]
            .chars()
            .next_back()
            .is_some_and(|c| c.is_ascii_alphanumeric() || c == '_');
        let name: String = code[at + 3..]
            .chars()
            .take_while(|c| c.is_ascii_alphanumeric() || *c == '_')
            .collect();
        if starts_word && !name.is_empty() && !names.contains(&name) {
            names.push(name);
        }
    }
    names
}

/// The functions of `code` that reach `target` through calls within `code`,
/// at any depth (`target` itself excluded).
fn reaching(code: &str, target: &str) -> std::collections::BTreeSet<String> {
    let names = fn_names(code);
    let mut reach = std::collections::BTreeSet::new();
    let mut frontier = vec![target.to_string()];
    while let Some(callee) = frontier.pop() {
        for name in &names {
            if name == target || reach.contains(name) {
                continue;
            }
            if fn_body(code, name).is_some_and(|body| calls(body, &callee)) {
                reach.insert(name.clone());
                frontier.push(name.clone());
            }
        }
    }
    reach
}

/// Scan a runtime file, minus the body of its by-name exempt function, and
/// report every function that reaches the exempt one — directly or through
/// other functions of the file — but is not a named verb caller. Panics if the
/// exempt function or a named caller is missing, so a rename cannot quietly
/// widen the exemption or empty the caller check.
fn scan_file_with_fn_exemption(
    label: &str,
    src: &str,
    exempt: &str,
    callers: &[&str],
) -> Vec<String> {
    let mut code = without_test_modules(&code_only(src));
    let body = fn_body(&code, exempt)
        .unwrap_or_else(|| panic!("{label} has no `fn {exempt}` — the exemption names nothing"));
    let start = body.as_ptr() as usize - code.as_ptr() as usize;
    let end = start + body.len();
    let blanked: String = code[start..end]
        .chars()
        .map(|c| if c == '\n' { '\n' } else { ' ' })
        .collect();
    code.replace_range(start..end, &blanked);
    let mut out = findings_in(label, &code);
    for caller in callers {
        assert!(
            fn_body(&code, caller).is_some(),
            "{label} has no `fn {caller}` — the exemption's caller list is stale"
        );
    }
    for name in reaching(&code, exempt) {
        if !callers.contains(&name.as_str()) {
            out.push(format!(
                "{label}: `fn {name}` reaches the verb-only `{exempt}`"
            ));
        }
    }
    out
}

/// Blank out comments, string and char literals (keeping length and newlines),
/// so brace matching and pattern search see only code.
fn code_only(src: &str) -> String {
    let b = src.as_bytes();
    let mut out = b.to_vec();
    let mut i = 0;
    let blank = |out: &mut Vec<u8>, from: usize, to: usize| {
        for byte in out.iter_mut().take(to).skip(from) {
            if *byte != b'\n' {
                *byte = b' ';
            }
        }
    };
    while i < b.len() {
        match b[i] {
            b'/' if b.get(i + 1) == Some(&b'/') => {
                let end = b[i..]
                    .iter()
                    .position(|&c| c == b'\n')
                    .map_or(b.len(), |p| i + p);
                blank(&mut out, i, end);
                i = end;
            }
            b'/' if b.get(i + 1) == Some(&b'*') => {
                let mut depth = 0;
                let mut j = i;
                while j < b.len() {
                    if b[j] == b'/' && b.get(j + 1) == Some(&b'*') {
                        depth += 1;
                        j += 2;
                    } else if b[j] == b'*' && b.get(j + 1) == Some(&b'/') {
                        depth -= 1;
                        j += 2;
                        if depth == 0 {
                            break;
                        }
                    } else {
                        j += 1;
                    }
                }
                blank(&mut out, i, j);
                i = j;
            }
            b'r' if (b.get(i + 1) == Some(&b'"') || b.get(i + 1) == Some(&b'#'))
                && starts_token(b, i) =>
            {
                let mut j = i + 1;
                let mut hashes = 0;
                while b.get(j) == Some(&b'#') {
                    hashes += 1;
                    j += 1;
                }
                if b.get(j) != Some(&b'"') {
                    i += 1;
                    continue;
                }
                let closing: Vec<u8> = std::iter::once(b'"')
                    .chain(std::iter::repeat_n(b'#', hashes))
                    .collect();
                let body_start = j + 1;
                let end = b[body_start..]
                    .windows(closing.len())
                    .position(|w| w == closing.as_slice())
                    .map_or(b.len(), |p| body_start + p + closing.len());
                blank(&mut out, i, end);
                i = end;
            }
            b'"' => {
                let mut j = i + 1;
                while j < b.len() && b[j] != b'"' {
                    if b[j] == b'\\' {
                        j += 1;
                    }
                    j += 1;
                }
                blank(&mut out, i, (j + 1).min(b.len()));
                i = j + 1;
            }
            b'\'' => {
                // A char literal ('x', '\n', '{') — not a lifetime ('a).
                let close = if b.get(i + 1) == Some(&b'\\') {
                    b[i + 2..]
                        .iter()
                        .position(|&c| c == b'\'')
                        .map(|p| i + 2 + p)
                } else if b.get(i + 2) == Some(&b'\'') {
                    Some(i + 2)
                } else {
                    None
                };
                match close {
                    Some(end) => {
                        blank(&mut out, i, end + 1);
                        i = end + 1;
                    }
                    None => i += 1,
                }
            }
            _ => i += 1,
        }
    }
    String::from_utf8(out).expect("blanking keeps UTF-8 boundaries")
}

/// Whether the `r` at `i` begins a raw-string token (`r"`, `r#"`, `br#"`), rather
/// than the tail of an identifier.
fn starts_token(b: &[u8], i: usize) -> bool {
    let ident = |c: u8| c.is_ascii_alphanumeric() || c == b'_';
    match i.checked_sub(1).map(|p| b[p]) {
        None => true,
        Some(b'b') => i < 2 || !ident(b[i - 2]),
        Some(c) => !ident(c),
    }
}

/// The index just past the `}` that closes the `{` at `open`.
fn matching_brace(code: &str, open: usize) -> usize {
    let mut depth = 0usize;
    for (offset, c) in code[open..].char_indices() {
        match c {
            '{' => depth += 1,
            '}' => {
                depth -= 1;
                if depth == 0 {
                    return open + offset + 1;
                }
            }
            _ => {}
        }
    }
    code.len()
}

/// `code` with every `#[cfg(test)]`-gated module blanked.
fn without_test_modules(code: &str) -> String {
    let mut out = code.to_string();
    let mut from = 0;
    while let Some(rel) = out[from..].find("#[cfg(test)]") {
        let at = from + rel;
        let after = at + "#[cfg(test)]".len();
        let rest = out[after..].trim_start();
        if rest.starts_with("mod ") {
            let Some(brace) = out[after..].find('{').map(|p| after + p) else {
                break;
            };
            // `mod tests;` (an out-of-line module) has no body here.
            if out[after..brace].contains(';') {
                from = after;
                continue;
            }
            let end = matching_brace(&out, brace);
            let blanked: String = out[at..end]
                .chars()
                .map(|c| if c == '\n' { '\n' } else { ' ' })
                .collect();
            out.replace_range(at..end, &blanked);
            from = end;
        } else {
            from = after;
        }
    }
    out
}

/// The body of `fn <name>(` in `code`, or `None` when there is no such function.
fn fn_body<'a>(code: &'a str, name: &str) -> Option<&'a str> {
    let mut from = 0;
    while let Some(rel) = code[from..].find(&format!("fn {name}")) {
        let at = from + rel;
        let after = at + 3 + name.len();
        // Exactly this name: `fn run(` / `fn run<`, never `fn run_foo(`.
        if matches!(code[after..].chars().next(), Some('(' | '<')) {
            let brace = code[after..].find('{').map(|p| after + p)?;
            return Some(&code[brace..matching_brace(code, brace)]);
        }
        from = after;
    }
    None
}

fn findings_in(label: &str, code: &str) -> Vec<String> {
    let mut out = Vec::new();
    for pattern in FORBIDDEN {
        let mut from = 0;
        while let Some(rel) = code[from..].find(pattern) {
            let at = from + rel;
            // `fn load_agent_by_id(` is a definition, not a call.
            if !code[..at].trim_end().ends_with("fn") {
                let line = code[..at].matches('\n').count() + 1;
                out.push(format!("{label}:{line}: `{pattern}`"));
            }
            from = at + pattern.len();
        }
    }
    out
}

/// Scan one runtime-ish file (whole file, test modules removed).
fn scan_file(label: &str, src: &str) -> Vec<String> {
    findings_in(label, &without_test_modules(&code_only(src)))
}

/// Scan the run-path functions of `commands/app.rs`. Panics if one is missing,
/// so a rename cannot silently shrink the scan.
fn scan_app_run_path(src: &str) -> Vec<String> {
    let code = without_test_modules(&code_only(src));
    let mut out = Vec::new();
    for name in APP_RUN_PATH_FNS {
        let body = fn_body(&code, name).unwrap_or_else(|| {
            panic!("commands/app.rs has no `fn {name}` — the run-path guard is scanning nothing")
        });
        assert!(body.len() > 2, "`fn {name}` has an empty body");
        out.extend(findings_in(&format!("commands/app.rs::{name}"), body));
    }
    out
}

fn rust_files(dir: &Path, out: &mut Vec<PathBuf>) {
    for entry in std::fs::read_dir(dir).expect("read src dir").flatten() {
        let path = entry.path();
        if path.is_dir() {
            rust_files(&path, out);
        } else if path.extension().is_some_and(|ext| ext == "rs") {
            out.push(path);
        }
    }
}

fn src_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("src")
}

#[test]
fn the_run_path_takes_manifests_only_from_the_resolved_catalogue() {
    let root = src_root();
    let mut files = Vec::new();
    rust_files(&root.join("runtime"), &mut files);
    assert!(
        files.len() >= 10 && files.iter().any(|f| f.ends_with("invoker.rs")),
        "the runtime/ walk is not reaching the tree ({} files)",
        files.len()
    );
    let mut findings = Vec::new();
    let mut scanned = 0;
    for file in &files {
        let rel = file
            .strip_prefix(&root)
            .unwrap()
            .to_string_lossy()
            .replace('\\', "/");
        if RUNTIME_EXEMPT.contains(&rel.as_str()) {
            continue;
        }
        scanned += 1;
        let src = std::fs::read_to_string(file).unwrap();
        match RUNTIME_FN_EXEMPT.iter().find(|(path, _, _)| *path == rel) {
            Some((_, exempt, callers)) => {
                findings.extend(scan_file_with_fn_exemption(&rel, &src, exempt, callers));
            }
            None => findings.extend(scan_file(&rel, &src)),
        }
    }
    for rel in EXTRA_RUN_PATH_FILES {
        scanned += 1;
        findings.extend(scan_file(
            rel,
            &std::fs::read_to_string(root.join(rel)).unwrap(),
        ));
    }
    let app = std::fs::read_to_string(root.join("commands/app.rs")).unwrap();
    findings.extend(scan_app_run_path(&app));
    assert!(scanned >= 10, "only {scanned} files scanned");
    assert!(
        findings.is_empty(),
        "the run path re-reads agents/ instead of the run's resolved catalogue \
         (`AgentCatalogue::manifest` / `root`, #626):\n  {}",
        findings.join("\n  ")
    );
}

#[test]
fn only_the_simulate_branch_reads_working_copies() {
    let root = src_root();
    let app = without_test_modules(&code_only(
        &std::fs::read_to_string(root.join("commands/app.rs")).unwrap(),
    ));
    let run = fn_body(&app, "run").expect("commands/app.rs has fn run");
    assert_eq!(
        run.matches("AgentCatalogue::working_copies(").count(),
        1,
        "`run` builds a working-copy catalogue exactly once — in its --simulate branch"
    );
    let mut files = Vec::new();
    rust_files(&root.join("runtime"), &mut files);
    for file in files {
        let code = without_test_modules(&code_only(&std::fs::read_to_string(&file).unwrap()));
        assert!(
            !code.contains("AgentCatalogue::working_copies(")
                && !code.contains("::WorkingCopies {"),
            "{} builds its own working-copy catalogue; the runtime must use the one it was given",
            file.display()
        );
    }
}

#[test]
fn the_scanner_trips_on_the_shapes_it_claims_to() {
    // Negative control: each forbidden call on a run-path function is found…
    let dirty_app = r#"
        async fn run(ctx: &Context) -> Result<(), AwareError> {
            let m = crate::manifest::loader::load_agent_by_id(&ctx.paths.agents_dir(), "x")?;
            let all = discover_agents(&ctx.paths)?;
            Ok(())
        }
        fn app_uses_model_reader() { let p = agent_manifest_path(&d, id); }
        fn nested_malformed_requires() { let a = crate::manifest::loader::load_agent(&p); }
        fn resolve_run_agents() { let a = discover_agents_in(&d); }
        fn simulate_nested_malformed_requires() { load_agent_by_id(&d, id); }
    "#;
    let found = scan_app_run_path(dirty_app);
    assert_eq!(found.len(), 5, "{found:#?}");
    assert!(found.iter().all(|f| !f.contains("simulate_")), "{found:#?}");

    // …in a runtime file too, but not inside a test module, a comment or a string.
    let runtime = r#"
        fn dispatch(&self) {
            // load_agent_by_id( in a comment is not a call
            let s = "discover_agents( in a string";
            let m = crate::manifest::loader::load_agent_by_id(&self.agents_dir, agent)?;
        }
        #[cfg(test)]
        mod tests {
            fn fixture() { let m = load_agent_by_id(&d, "x"); let c = '{'; }
        }
    "#;
    let found = scan_file("runtime/x.rs", runtime);
    assert_eq!(found.len(), 1, "{found:#?}");

    // And a definition is not a call.
    assert!(scan_file("x.rs", "pub fn load_agent_by_id(d: &Path) {}").is_empty());
}

/// Negative controls for the by-name function exemption: the exempt body is
/// skipped, a forbidden read anywhere else in the file still trips, and a call
/// of the exempt function from a run-path function trips.
#[test]
fn a_function_exemption_covers_one_body_and_only_its_named_callers() {
    let clean = r#"
        fn read_installed(home: &Path) { let p = agent_manifest_path(&d, id); }
        async fn capabilities_within() { let i = read_installed(h); }
        pub(crate) async fn capabilities() { capabilities_within().await }
        async fn call_within() { let i = read_installed(h); }
        pub(crate) async fn run_for_workflow_with(manifest: &Agent) { use_it(manifest); }
        fn read_and_call() { helper(); request.call(); }
    "#;
    let callers: &[&str] = &["capabilities", "capabilities_within", "call_within"];
    assert!(
        scan_file_with_fn_exemption("runtime/agent_call.rs", clean, "read_installed", callers)
            .is_empty()
    );

    let run_path_caller = clean.replace("{ use_it(manifest); }", "{ let i = read_installed(h); }");
    let found = scan_file_with_fn_exemption(
        "runtime/agent_call.rs",
        &run_path_caller,
        "read_installed",
        callers,
    );
    assert_eq!(found.len(), 1, "{found:#?}");
    assert!(
        found[0].contains("`fn run_for_workflow_with` reaches"),
        "{found:#?}"
    );

    // Indirectly, through a verb function or a helper, at any depth (Codex).
    for indirect in [
        "{ capabilities_within().await; }",
        "{ helper(); } fn helper() { deeper() } fn deeper() { call_within() }",
    ] {
        let src = clean.replace("{ use_it(manifest); }", indirect);
        let found =
            scan_file_with_fn_exemption("runtime/agent_call.rs", &src, "read_installed", callers);
        assert!(
            found
                .iter()
                .any(|f| f.contains("`fn run_for_workflow_with` reaches")),
            "{indirect}: {found:#?}"
        );
    }

    // Another run-path file calling the verbs is a forbidden read too.
    let found = scan_file(
        "runtime/invoker.rs",
        "fn dispatch() { crate::runtime::agent_call::capabilities(h, op, None).await; }",
    );
    assert_eq!(found.len(), 1, "{found:#?}");

    let second_read = clean.replace(
        "{ use_it(manifest); }",
        "{ let m = load_agent_by_id(&d, id); }",
    );
    let found = scan_file_with_fn_exemption(
        "runtime/agent_call.rs",
        &second_read,
        "read_installed",
        callers,
    );
    assert_eq!(found.len(), 1, "{found:#?}");
    assert!(found[0].contains("load_agent_by_id("), "{found:#?}");

    for (missing, src) in [
        (
            "exempt fn",
            clean.replace("fn read_installed", "fn renamed"),
        ),
        ("caller", clean.replace("fn call_within", "fn renamed")),
    ] {
        let result = std::panic::catch_unwind(|| {
            scan_file_with_fn_exemption("x.rs", &src, "read_installed", callers)
        });
        assert!(result.is_err(), "a missing {missing} must fail the guard");
    }
}

#[test]
fn the_scanner_refuses_to_scan_a_missing_run_function() {
    let result = std::panic::catch_unwind(|| scan_app_run_path("fn something_else() {}"));
    assert!(
        result.is_err(),
        "a renamed run path must fail the guard, not pass it"
    );
}
