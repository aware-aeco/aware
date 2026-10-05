//! The store reference lock is held by every store-reference writer, and every
//! swap lock is taken under it (#627 plan 8 R1-1, R1-4, R1-7).
//!
//! The lock ORDER (store first, then swap locks) is enforced by the compiler:
//! `install::swap`'s lock API takes `&RefGuard` and ties every swap lock's
//! lifetime to it. What the compiler cannot see is (a) that the API keeps that
//! shape, (b) that every command that creates or relies on a store reference
//! takes the guard at all — and takes it BEFORE it reads the approval it is
//! about to rely on — and (c) that a shared guard is only ever obtained through
//! `agent_store::open`. This is a text scan of the REAL source tree, in the
//! spirit of `run_path_reads_the_resolved_catalogue.rs`, with negative controls.

use std::path::{Path, PathBuf};

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
            // Byte for byte, so the offsets after it stay valid when the
            // module holds multi-byte characters.
            let blanked: String = out[at..end]
                .chars()
                .map(|c| {
                    if c == '\n' {
                        "\n".to_string()
                    } else {
                        " ".repeat(c.len_utf8())
                    }
                })
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

/// The text from `fn <name>` up to (not including) its body's `{`: the
/// signature, generics and parameters.
fn fn_signature<'a>(code: &'a str, name: &str) -> Option<&'a str> {
    let mut from = 0;
    while let Some(rel) = code[from..].find(&format!("fn {name}")) {
        let at = from + rel;
        let after = at + 3 + name.len();
        if matches!(code[after..].chars().next(), Some('(' | '<')) {
            let brace = code[after..].find('{').map(|p| after + p)?;
            return Some(&code[at..brace]);
        }
        from = after;
    }
    None
}

fn src_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("src")
}

fn code_of(rel: &str) -> String {
    let path = src_root().join(rel);
    let text = std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("cannot read {}: {e}", path.display()));
    without_test_modules(&code_only(&text))
}

/// The commands that create or rely on a store reference (plan 8 R1-4,
/// R1-7 as far as #627-a reaches): `(file, fn, must-open-before)`. The guard
/// must be taken in that function, and BEFORE the first occurrence of the
/// third column (the read or write of the reference it relies on).
const WRITERS: &[(&str, &str, &str)] = &[
    // `app run`: before the source and its `<app>.lock` are read.
    ("commands/app.rs", "run", "load_approved_app_with_lock("),
    ("commands/app.rs", "compile_cmd", "compile_to_disk_for("),
    (
        "commands/app.rs",
        "inspect_cmd",
        "compile_to_disk_with_lock(",
    ),
    (
        "commands/app.rs",
        "recompile_after_freeze",
        "compile_to_disk(",
    ),
    ("commands/app.rs", "install", "install_app_from_path("),
    ("commands/app.rs", "rename_cmd", "rename_app("),
    ("commands/app.rs", "duplicate_cmd", "duplicate_app("),
    ("commands/app.rs", "dispatch", "uninstall_app("),
    // Every migrate verb: before any verb reads an approval.
    ("commands/app_migrate.rs", "dispatch", "_cmd("),
    ("commands/agent.rs", "dispatch", "uninstall_agent("),
    ("commands/agent.rs", "install", "install_agent_from_path("),
    (
        "commands/agent.rs",
        "update_one",
        "update_agent_from_registry(",
    ),
    (
        "commands/agent.rs",
        "update_all",
        "update_agent_from_registry(",
    ),
    ("commands/doctor.rs", "recover_agent_swaps", "recover_all("),
];

const OPEN: &str = "agent_store::open(";

/// Findings for one writer: the guard must be taken, before `read`.
fn writer_findings(label: &str, body: &str, read: &str) -> Vec<String> {
    let Some(open) = body.find(OPEN) else {
        return vec![format!(
            "{label}: never takes the store reference lock (`{OPEN}`)"
        )];
    };
    match body.find(read) {
        None => vec![format!(
            "{label}: no `{read}` - the guard list names a call this function no longer makes"
        )],
        Some(at) if at < open => vec![format!(
            "{label}: `{read}` runs before the store reference lock is taken"
        )],
        Some(_) => Vec::new(),
    }
}

fn scan_writers(sources: &dyn Fn(&str) -> String) -> Vec<String> {
    let mut out = Vec::new();
    for (file, name, read) in WRITERS {
        let code = sources(file);
        let body = fn_body(&code, name)
            .unwrap_or_else(|| panic!("{file} has no `fn {name}` - the guard list is stale"));
        out.extend(writer_findings(&format!("{file}::{name}"), body, read));
    }
    out
}

#[test]
fn every_store_reference_writer_holds_the_guard_before_it_reads() {
    let findings = scan_writers(&code_of);
    assert!(
        findings.is_empty(),
        "store-reference writers without the store reference lock (#627):\n  {}",
        findings.join("\n  ")
    );
}

/// The APIs a reference is created through demand the guard in their
/// signature, so a caller that forgets it does not compile.
const GUARDED_SIGNATURES: &[(&str, &str)] = &[
    ("agent_store.rs", "snapshot"),
    ("agent_resolution.rs", "resolve_agents"),
    ("app_lock.rs", "compile_to_disk"),
    ("app_lock.rs", "compile_to_disk_with_lock"),
    ("install/registry.rs", "install_agent_from_registry"),
    ("install/registry.rs", "update_agent_from_registry"),
    ("install/local.rs", "install_agent_from_path"),
    ("install/local.rs", "install_app_from_path"),
    ("install/local.rs", "write_synthesized_agent"),
    ("install/local.rs", "remove_synthesized_agent"),
    ("install/uninstall.rs", "uninstall_agent"),
    ("install/uninstall.rs", "uninstall_app"),
    ("install/rename.rs", "rename_app"),
    ("install/rename.rs", "duplicate_app"),
    ("install/bundle.rs", "install_bundle"),
    // The swap-lock API: every acquisition path takes the guard.
    ("install/swap.rs", "acquire"),
    ("install/swap.rs", "read_lock"),
    ("install/swap.rs", "begin"),
    ("install/swap.rs", "lock_and_recover"),
    ("install/swap.rs", "recover_all"),
];

fn signature_findings(label: &str, signature: &str) -> Vec<String> {
    if signature.contains("RefGuard") {
        Vec::new()
    } else {
        vec![format!("{label}: its signature takes no `&RefGuard`")]
    }
}

#[test]
fn reference_and_swap_lock_apis_demand_the_guard() {
    let mut findings = Vec::new();
    for (file, name) in GUARDED_SIGNATURES {
        let code = code_of(file);
        let signature = fn_signature(&code, name)
            .unwrap_or_else(|| panic!("{file} has no `fn {name}` - the guard list is stale"));
        findings.extend(signature_findings(&format!("{file}::{name}"), signature));
    }
    assert!(findings.is_empty(), "{}", findings.join("\n"));
}

/// Swap lock files are locked in exactly one place, `swap::acquire`, whose
/// signature takes the guard, and the swap locks it returns borrow it.
#[test]
fn swap_locks_are_only_taken_inside_the_guarded_acquire() {
    let code = code_of("install/swap.rs");
    let acquire = fn_body(&code, "acquire").expect("swap.rs has fn acquire");
    for call in [".lock_exclusive()", ".lock_shared()"] {
        assert_eq!(
            code.matches(call).count(),
            acquire.matches(call).count(),
            "`{call}` is used outside `swap::acquire`"
        );
        assert_eq!(acquire.matches(call).count(), 1, "`{call}` in acquire");
    }
    assert!(
        code.contains("PhantomData<&'g RefGuard>"),
        "SwapLocks must borrow the RefGuard it was taken under"
    );
    // No other file locks a swap lock or names the lock directory.
    let mut files = Vec::new();
    rust_files(&src_root(), &mut files);
    assert!(files.len() > 50, "the src walk is not reaching the tree");
    for file in files {
        let rel = file
            .strip_prefix(src_root())
            .unwrap()
            .to_string_lossy()
            .replace('\\', "/");
        if rel.starts_with("install/swap") {
            continue;
        }
        let text = std::fs::read_to_string(&file).unwrap();
        assert!(
            !text.contains("aware-swap/locks") && !text.contains("LOCKS_DIR"),
            "{rel} reaches into the swap lock directory"
        );
    }
}

/// A shared guard comes only from `agent_store::open` (R5-1: the one door a
/// future store import must run behind); `RefGuard::shared` is not public.
#[test]
fn a_shared_guard_is_only_obtainable_through_open() {
    let guard = code_of("agent_store/guard.rs");
    let signature = fn_signature(&guard, "shared").expect("guard.rs has fn shared");
    let before = &guard[..guard.find(signature).unwrap()];
    let line_start = before.rfind('\n').map_or(0, |p| p + 1);
    let declared = &guard[line_start..guard.find(signature).unwrap() + 9];
    assert!(
        declared.contains("pub(super) fn shared") || declared.trim_start().starts_with("fn shared"),
        "RefGuard::shared must stay private to agent_store: {declared:?}"
    );
    let mut files = Vec::new();
    rust_files(&src_root(), &mut files);
    let mut callers = Vec::new();
    for file in files {
        let code = without_test_modules(&code_only(&std::fs::read_to_string(&file).unwrap()));
        if code.contains("RefGuard::shared(") {
            callers.push(
                file.strip_prefix(src_root())
                    .unwrap()
                    .to_string_lossy()
                    .replace('\\', "/"),
            );
        }
    }
    assert_eq!(callers, vec!["agent_store.rs".to_string()], "{callers:?}");
    let store = code_of("agent_store.rs");
    let open = fn_body(&store, "open").expect("agent_store.rs has fn open");
    assert!(open.contains("RefGuard::shared("));
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

// -- negative controls --------------------------------------------------------

#[test]
fn the_writer_scan_trips_on_a_missing_or_late_guard() {
    let read = "load_approved_app_with_lock(";
    // Missing entirely.
    let found = writer_findings(
        "x::run",
        "{ let (app, lock) = load_approved_app_with_lock(&p)?; }",
        read,
    );
    assert_eq!(found.len(), 1, "{found:?}");
    assert!(found[0].contains("never takes"));
    // Taken, but after the approval was read.
    let found = writer_findings(
        "x::run",
        "{ let (app, lock) = load_approved_app_with_lock(&p)?; let g = crate::agent_store::open(&paths)?; }",
        read,
    );
    assert_eq!(found.len(), 1, "{found:?}");
    assert!(found[0].contains("before the store reference lock"));
    // The named read is gone: the list is stale, which must fail too.
    let found = writer_findings(
        "x::run",
        "{ let g = crate::agent_store::open(&paths)?; }",
        read,
    );
    assert_eq!(found.len(), 1, "{found:?}");
    // In order: clean.
    assert!(
        writer_findings(
            "x::run",
            "{ let g = crate::agent_store::open(&paths)?; let x = load_approved_app_with_lock(&p)?; }",
            read,
        )
        .is_empty()
    );
    // A guard mentioned only in a comment or a string does not count.
    let commented = without_test_modules(&code_only(
        "fn run() { // crate::agent_store::open(&paths)\n let s = \"agent_store::open(\"; load_approved_app_with_lock(&p); }",
    ));
    let body = fn_body(&commented, "run").unwrap();
    assert_eq!(writer_findings("x::run", body, read).len(), 1);
    // And a renamed function fails the scan instead of passing it.
    let missing =
        std::panic::catch_unwind(|| scan_writers(&|_| "fn something_else() {}".to_string()));
    assert!(missing.is_err());
}

#[test]
fn the_signature_scan_trips_on_a_guardless_api() {
    let code =
        "pub fn snapshot(paths: &Paths, root: &Path) -> Result<StoredPackage, AwareError> { x }";
    let signature = fn_signature(code, "snapshot").unwrap();
    assert_eq!(
        signature_findings("agent_store.rs::snapshot", signature).len(),
        1
    );
    let code =
        "pub fn snapshot(paths: &Paths, root: &Path, guard: &RefGuard) -> Result<(), E> { x }";
    assert!(signature_findings("s", fn_signature(code, "snapshot").unwrap()).is_empty());
}
