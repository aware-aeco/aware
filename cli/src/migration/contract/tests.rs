use super::*;

const BASE: &str = r#"agent: tool
version: 1.0.0
display-name: Tool
description: A tool.
stateful: false
license: MIT
keywords: [a]
skills: [one.md]
transport:
  cli:
    binary: aware-tool
commands:
  exec:
    lifecycle: single
    description: Run a script.
    mode: write
    mode-overridable: true
    inputs:
      code: { type: string }
  fetch:
    lifecycle: single
    description: Fetch.
    method: GET
    path: /things
  other:
    lifecycle: single
    description: Not called.
    mode: read
probe:
  command: fetch
  describe: proves it
"#;

struct Pkg {
    _dir: tempfile::TempDir,
    root: std::path::PathBuf,
}

fn package(manifest: &str, files: &[(&str, &str)]) -> Pkg {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path().to_path_buf();
    std::fs::write(root.join("manifest.yaml"), manifest).unwrap();
    for (rel, body) in files {
        let path = root.join(rel);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(path, body).unwrap();
    }
    Pkg { _dir: dir, root }
}

fn called(entries: &[(&str, &str)]) -> BTreeMap<String, Vec<String>> {
    let mut out: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for (command, node) in entries {
        out.entry((*command).into())
            .or_default()
            .push((*node).into());
    }
    out
}

/// Every test pins the executor to one fixed identity so a host's PATH can
/// never decide a verdict; the executor tests vary it on purpose.
fn fixed(_: &Agent) -> Executor {
    Executor::AwareCli {
        version: "test".into(),
    }
}

fn run(old: &Pkg, new: &Pkg, called: &BTreeMap<String, Vec<String>>) -> ContractDiff {
    diff_with(
        DiffInput {
            agent: "tool",
            old: PackageSide {
                root: &old.root,
                version: "1.0.0",
                digest: "sha256:old",
            },
            new: PackageSide {
                root: &new.root,
                version: "1.0.1",
                digest: "sha256:new",
            },
            called,
            plan_changes: Vec::new(),
        },
        fixed,
    )
    .unwrap()
}

fn exec_called() -> BTreeMap<String, Vec<String>> {
    called(&[("exec", "n1"), ("fetch", "n2")])
}

#[test]
fn identical_packages_are_unchanged_and_say_so_in_the_v1_shape() {
    let a = package(BASE, &[("atoms/run.cs", "x")]);
    let b = package(BASE, &[("atoms/run.cs", "x")]);
    let d = run(&a, &b, &exec_called());
    assert!(d.unchanged);
    assert_eq!(d.format, "aware.contract-diff/v1");
    let json = serde_json::to_value(&d).unwrap();
    for key in [
        "agent",
        "from",
        "to",
        "unchanged",
        "agent-level",
        "commands",
        "executable-files",
        "probe-changed",
        "executor",
        "plan-changes",
        "ignored",
    ] {
        assert!(json.get(key).is_some(), "missing {key}: {json}");
    }
    assert_eq!(json["commands"][0]["nodes"], serde_json::json!(["n1"]));
    assert!(json["ignored"].get("commands-not-called").is_some());
}

#[test]
fn ignored_agent_keys_and_command_descriptions_never_count() {
    let a = package(BASE, &[]);
    let edited = BASE
        .replace("version: 1.0.0", "version: 1.0.1")
        .replace("display-name: Tool", "display-name: Tool Two")
        .replace("description: A tool.", "description: A better tool.")
        .replace(
            "keywords: [a]",
            "keywords: [a, b]\nhomepage: https://x\nvendor: V",
        )
        .replace("license: MIT", "license: Apache-2.0")
        .replace(
            "skills: [one.md]",
            "skills: [one.md, two.md]\nprovenance: { generated-by: x }",
        )
        .replace(
            "description: Run a script.",
            "description: Run a C# script.",
        );
    let b = package(&edited, &[]);
    let d = run(&a, &b, &exec_called());
    assert!(d.unchanged, "{d:#?}");
    assert!(d.agent_level.changed.is_empty());
}

#[test]
fn a_called_command_change_is_named_down_to_the_key() {
    let a = package(BASE, &[]);
    for (from, to, key) in [
        (
            "      code: { type: string }",
            "      code: { type: string }\n      timeout: { type: integer }",
            "inputs",
        ),
        (
            "    mode-overridable: true",
            "    mode-overridable: false",
            "mode-overridable",
        ),
        ("    method: GET", "    method: POST", "method"),
        ("    path: /things", "    path: /things/{id}", "path"),
        (
            "    method: GET",
            "    method: GET\n    no-auth: true",
            "no-auth",
        ),
        (
            "    method: GET",
            "    method: GET\n    x-unknown: 1",
            "x-unknown",
        ),
    ] {
        let b = package(&BASE.replacen(from, to, 1), &[]);
        let d = run(&a, &b, &exec_called());
        assert!(!d.unchanged, "{key} must count");
        let changed: Vec<&String> = d.commands.iter().flat_map(|c| &c.changes).collect();
        assert_eq!(changed, [&key.to_string()], "{key}");
    }
}

#[test]
fn an_unknown_or_run_relevant_agent_key_counts() {
    let a = package(BASE, &[]);
    for (edit, key) in [
        (
            BASE.replace("stateful: false", "stateful: false\nx-new-runtime-key: 1"),
            "x-new-runtime-key",
        ),
        (
            BASE.replace(
                "stateful: false",
                "stateful: false\nauth: { scheme: bearer, secret: s }",
            ),
            "auth",
        ),
        (
            BASE.replace("binary: aware-tool", "binary: aware-tool2"),
            "transport",
        ),
        (
            BASE.replace("stateful: false", "stateful: true"),
            "stateful",
        ),
    ] {
        let d = run(&a, &package(&edit, &[]), &exec_called());
        assert!(!d.unchanged, "{key}");
        assert_eq!(d.agent_level.changed, [key.to_string()]);
    }
}

#[test]
fn an_uncalled_command_change_is_reported_but_not_counted() {
    let a = package(BASE, &[]);
    let b = package(&BASE.replace("    mode: read", "    mode: write"), &[]);
    let d = run(&a, &b, &exec_called());
    assert!(d.unchanged);
    assert_eq!(d.ignored.commands_not_called, ["other"]);
    // ...and counts the moment a node calls it.
    let d = run(&a, &b, &called(&[("other", "n3")]));
    assert!(!d.unchanged);
}

#[test]
fn a_command_appearing_or_vanishing_counts() {
    let a = package(BASE, &[]);
    let gone = package(&BASE.replace("  fetch:\n", "  fetch-renamed:\n"), &[]);
    let d = run(&a, &gone, &exec_called());
    let fetch = d.commands.iter().find(|c| c.command == "fetch").unwrap();
    assert_eq!(fetch.changes, ["removed"]);
    let d = run(&gone, &a, &exec_called());
    let fetch = d.commands.iter().find(|c| c.command == "fetch").unwrap();
    assert_eq!(fetch.changes, ["added"]);
}

#[test]
fn a_probe_change_is_reported_and_not_counted() {
    let a = package(BASE, &[]);
    let b = package(
        &BASE.replace("describe: proves it", "describe: proves it again"),
        &[],
    );
    let d = run(&a, &b, &exec_called());
    assert!(d.probe_changed);
    assert!(d.unchanged);
}

#[test]
fn executable_files_count_and_doc_files_do_not() {
    let a = package(
        BASE,
        &[
            ("atoms/run.cs", "one"),
            ("scripts/old.py", "x"),
            ("skills/one.md", "a"),
            ("commands/exec.md", "a"),
            ("CHANGELOG.md", "a"),
            ("README.md", "a"),
            ("LICENSE", "a"),
        ],
    );
    let docs_only = package(
        BASE,
        &[
            ("atoms/run.cs", "one"),
            ("scripts/old.py", "x"),
            ("skills/one.md", "b"),
            ("skills/two.md", "new"),
            ("commands/exec.md", "b"),
            ("CHANGELOG.md", "b"),
            ("README.md", "b"),
            ("LICENSE", "b"),
            (".aware-install.yaml", "receipt"),
        ],
    );
    let d = run(&a, &docs_only, &exec_called());
    assert!(d.unchanged, "{d:#?}");
    assert_eq!(
        d.ignored.doc_files,
        [
            ".aware-install.yaml",
            "CHANGELOG.md",
            "LICENSE",
            "README.md",
            "commands/exec.md",
            "skills/one.md",
            "skills/two.md"
        ]
    );

    let code = package(
        BASE,
        &[
            ("atoms/run.cs", "two"),
            ("scripts/new.py", "y"),
            ("commands/helper.cs", "z"),
            ("nested/README.md", "z"),
        ],
    );
    let d = run(&a, &code, &exec_called());
    assert!(!d.unchanged);
    assert_eq!(d.executable_files.changed, ["atoms/run.cs"]);
    // Only root-level README/LICENSE and commands/*.md are documentation.
    assert_eq!(
        d.executable_files.added,
        ["commands/helper.cs", "nested/README.md", "scripts/new.py"]
    );
    assert_eq!(d.executable_files.removed, ["scripts/old.py"]);
}

#[test]
fn a_different_executor_for_the_old_pin_counts() {
    let a = package(BASE, &[]);
    let b = package(
        &BASE.replace("binary: aware-tool", "binary: aware-tool-2"),
        &[],
    );
    let by_binary = |m: &Agent| Executor::Cli {
        binary: m.transport.cli.as_ref().unwrap().binary.clone(),
        program: "p".into(),
        sha256: Some("sha256:1".into()),
        detail: None,
    };
    let called = exec_called();
    let d = diff_with(
        DiffInput {
            agent: "tool",
            old: PackageSide {
                root: &a.root,
                version: "1",
                digest: "a",
            },
            new: PackageSide {
                root: &b.root,
                version: "2",
                digest: "b",
            },
            called: &called,
            plan_changes: Vec::new(),
        },
        by_binary,
    )
    .unwrap();
    assert!(d.agent_level.changed.contains(&"executor".to_string()));
    assert!(!d.unchanged);
}

#[test]
fn a_plan_change_alone_makes_the_contract_changed() {
    let a = package(BASE, &[]);
    let b = package(BASE, &[]);
    let called = exec_called();
    let d = diff_with(
        DiffInput {
            agent: "tool",
            old: PackageSide {
                root: &a.root,
                version: "1",
                digest: "a",
            },
            new: PackageSide {
                root: &b.root,
                version: "2",
                digest: "b",
            },
            called: &called,
            plan_changes: vec![PlanChange {
                node: "n1".into(),
                field: "mode".into(),
                from: serde_json::json!("read"),
                to: serde_json::json!("write"),
            }],
        },
        fixed,
    )
    .unwrap();
    assert!(!d.unchanged);
}

#[test]
fn key_order_and_yaml_spelling_do_not_count_as_change() {
    let a = package(BASE, &[]);
    let reordered = BASE.replace(
        "    method: GET\n    path: /things",
        "    path: /things\n    method: GET",
    );
    let d = run(&a, &package(&reordered, &[]), &exec_called());
    assert!(d.unchanged, "{d:#?}");
}

fn lock(nodes: &str) -> LockFile {
    serde_yaml::from_str(&format!(
        "source-hash: x\ncompiled-at: t\ncompiler-version: v\napp: a\nversion: 1.0.0\nagent-pins: {{}}\nnodes:\n{nodes}"
    ))
    .unwrap()
}

#[test]
fn plan_changes_compare_only_the_moved_agents_compiled_fields() {
    let base = lock(
        "  - { id: n1, kind: agent, agent: tool, command: exec, mode: read, output-schema: { type: single, schema: { a: string } } }\n\
         \x20 - { id: n2, kind: agent, agent: tool, command: fetch, mode: read }\n\
         \x20 - { id: x, kind: agent, agent: other, command: c, mode: read }\n",
    );
    let candidate = lock(
        "  - { id: n1, kind: agent, agent: tool, command: exec, mode: write, output-schema: { schema: { a: string }, type: single } }\n\
         \x20 - { id: n3, kind: agent, agent: tool, command: fetch, mode: read }\n\
         \x20 - { id: x, kind: agent, agent: other, command: c, mode: write }\n",
    );
    let changes = plan_changes(&base, &candidate, "tool");
    let summary: Vec<(&str, &str)> = changes
        .iter()
        .map(|c| (c.node.as_str(), c.field.as_str()))
        .collect();
    // n1: mode only (the reordered output schema is the same schema); n2 gone,
    // n3 new; node x belongs to another agent.
    assert_eq!(summary, [("n1", "mode"), ("n2", "node"), ("n3", "node")]);
    assert!(plan_changes(&base, &base, "tool").is_empty());
}

#[test]
fn called_commands_skip_frozen_subtrees_and_enter_do_bodies() {
    let app: App = serde_yaml::from_str(
        "app: a\nversion: 1.0.0\ndescription: x\nrequires: []\nnodes:\n\
         \x20 - { id: a, agent: tool, command: exec }\n\
         \x20 - { id: f, agent: tool, command: fetch, frozen: { ok: true } }\n\
         \x20 - id: loop\n    for-each: '{{ inputs.xs }}'\n    do:\n      - { id: b, agent: tool, command: exec }\n\
         \x20 - { id: c, agent: other, command: exec }\n",
    )
    .unwrap();
    let called = called_commands(&app, "tool");
    assert_eq!(called.len(), 1, "{called:?}");
    assert_eq!(called["exec"], ["a", "loop.b"]);
}

// ── The ignore-list is only honest while the run path never reads it ──────────

/// Every `.<field>` access in Rust source that names a contract-ignored
/// manifest key (`IGNORED_AGENT_KEYS`, in its Rust spelling), method calls
/// excluded, plus any call of `parse_probe`. Returns `receiver.field` strings.
fn ignored_field_reads(source: &str) -> Vec<String> {
    const FIELDS: &[&str] = &[
        "version",
        "display_name",
        "description",
        "keywords",
        "homepage",
        "vendor",
        "license",
        "provenance",
        "skills",
        "probe",
    ];
    let bytes = source.as_bytes();
    let is_ident = |b: u8| b.is_ascii_alphanumeric() || b == b'_';
    let mut hits = Vec::new();
    for (dot, _) in source.match_indices('.') {
        let start = dot + 1;
        let mut end = start;
        while end < bytes.len() && is_ident(bytes[end]) {
            end += 1;
        }
        let field = &source[start..end];
        if !FIELDS.contains(&field) {
            continue;
        }
        let rest = source[end..].trim_start();
        if rest.starts_with('(') || rest.starts_with("::") {
            continue; // a method call, not a field read
        }
        let mut begin = dot;
        while begin > 0 && is_ident(bytes[begin - 1]) {
            begin -= 1;
        }
        hits.push(format!("{}.{field}", &source[begin..dot]));
    }
    for (at, _) in source.match_indices("parse_probe(") {
        hits.push(format!("parse_probe@{at}"));
    }
    hits
}

/// `receiver.field` reads in `runtime/` that are NOT the agent manifest — each
/// must still exist, so the list cannot rot into a blanket pass.
const NOT_THE_MANIFEST: &[(&str, &str)] = &[
    // The Gmail reservation record's own schema version.
    ("google_mail.rs", "record.version"),
    // The run's provenance event writer, not the manifest's `provenance:`.
    ("orchestrator.rs", "self.provenance"),
];

fn scan_runtime(root: &Path) -> Vec<String> {
    let mut found = Vec::new();
    for (relative, path) in crate::fs::plain_files_under(root, "runtime source").unwrap() {
        // The probe is the one runtime path that reads `probe:` — and only
        // `aware agent probe` runs it, never `aware app run`.
        if !relative.ends_with(".rs") || relative == "probe.rs" || relative == "probe_tests.rs" {
            continue;
        }
        let file = relative.rsplit('/').next().unwrap_or(&relative).to_string();
        let source = std::fs::read_to_string(&path).unwrap();
        for hit in ignored_field_reads(&source) {
            if !NOT_THE_MANIFEST.contains(&(file.as_str(), hit.as_str())) {
                found.push(format!("runtime/{relative}: {hit}"));
            }
        }
    }
    found
}

#[test]
fn run_path_never_reads_a_contract_ignored_field() {
    let runtime = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/runtime");
    let found = scan_runtime(&runtime);
    assert!(
        found.is_empty(),
        "the run path reads a manifest field the contract diff ignores, so a change to it \
         would be carried forward as 'unchanged' — compare it (remove it from \
         IGNORED_AGENT_KEYS) or stop reading it: {found:#?}"
    );
    // The exceptions are real, current reads — not a blanket pass.
    for (file, read) in NOT_THE_MANIFEST {
        let source = std::fs::read_to_string(runtime.join(file)).unwrap();
        assert!(
            ignored_field_reads(&source).iter().any(|h| h == read),
            "stale exception {file}: {read}"
        );
    }
}

/// Negative control: the scanner must flag exactly the reads the guard exists
/// for, or a green guard proves nothing.
#[test]
fn the_ignored_field_scanner_flags_manifest_reads_and_spares_method_calls() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(
        dir.path().join("dispatch.rs"),
        "fn f(m: &Agent) { let v = m.version; let l = &manifest.license; \
         let s = agent.skills.len(); let d = cmd.description.clone(); \
         let x = crate::manifest::probe::parse_probe(m); }\n",
    )
    .unwrap();
    std::fs::write(
        dir.path().join("clean.rs"),
        "fn g(x: &X) { x.version(); X::probe(); let t = x.transport; }\n",
    )
    .unwrap();
    // probe.rs is the exempt file even when it reads the field.
    std::fs::write(
        dir.path().join("probe.rs"),
        "fn h(m: &Agent) { m.probe; }\n",
    )
    .unwrap();
    let found = scan_runtime(dir.path());
    assert_eq!(found.len(), 5, "{found:#?}");
    for read in [
        "m.version",
        "manifest.license",
        "agent.skills",
        "cmd.description",
    ] {
        assert!(
            found.iter().any(|f| f.ends_with(read)),
            "{read} not flagged: {found:#?}"
        );
    }
    assert!(found.iter().any(|f| f.contains("parse_probe")));
    assert!(found.iter().all(|f| f.starts_with("runtime/dispatch.rs")));
}

/// The real Tekla packages: point `AWARE_628_OLD_PACKAGE` / `AWARE_628_NEW_PACKAGE`
/// at two store packages of `tekla` (e.g. 0.1.5 and 0.1.6 from a real install)
/// and run with `--ignored --nocapture` to print the diff for an `exec` node.
#[test]
#[ignore = "needs two real stored tekla packages; see the doc comment"]
fn real_tekla_packages_contract_diff_for_exec() {
    let (Ok(old), Ok(new)) = (
        std::env::var("AWARE_628_OLD_PACKAGE"),
        std::env::var("AWARE_628_NEW_PACKAGE"),
    ) else {
        panic!("set AWARE_628_OLD_PACKAGE and AWARE_628_NEW_PACKAGE");
    };
    let record = |root: &str| -> crate::agent_store::PackageMetadata {
        serde_yaml::from_str(
            &std::fs::read_to_string(Path::new(root).join(crate::agent_store::PACKAGE_FILE))
                .unwrap(),
        )
        .unwrap()
    };
    let (a, b) = (record(&old), record(&new));
    let called = called(&[("exec", "read-model")]);
    let d = diff(DiffInput {
        agent: "tekla",
        old: PackageSide {
            root: Path::new(&old),
            version: &a.version,
            digest: &a.digest,
        },
        new: PackageSide {
            root: Path::new(&new),
            version: &b.version,
            digest: &b.digest,
        },
        called: &called,
        plan_changes: Vec::new(),
    })
    .unwrap();
    println!("{}", serde_json::to_string_pretty(&d).unwrap());
}
