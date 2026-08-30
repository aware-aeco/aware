use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use tokio::fs::OpenOptions;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

use crate::error::{AwareError, StructuredAgentError};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum RunEvent {
    RunStart {
        ts: String,
        run_id: String,
        app: String,
        instance: String,
        config: serde_json::Value,
    },
    NodeStart {
        ts: String,
        run_id: String,
        node: String,
        agent: Option<String>,
        command: Option<String>,
    },
    NodeOutput {
        ts: String,
        run_id: String,
        node: String,
        data: serde_json::Value,
    },
    /// A bounded progress record a still-running command published on its
    /// progress channel (#405). Mirrored into the trace the moment it is
    /// written, so a consumer tailing the trace learns what a long node is
    /// doing — and can retrieve the ordered artifact segments it announces —
    /// before the node's single `node-output` exists. `data` is the record's
    /// `$aware-progress` body, size-capped by
    /// [`crate::runtime::progress::MAX_RECORD_BYTES`] so a payload can never
    /// become a trace record.
    NodeProgress {
        ts: String,
        run_id: String,
        node: String,
        data: serde_json::Value,
    },
    NodeError {
        ts: String,
        run_id: String,
        node: String,
        error: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        structured: Option<StructuredAgentError>,
    },
    NodeStop {
        ts: String,
        run_id: String,
        node: String,
        reason: String,
    },
    /// Emitted in `--dry-run` mode in place of `NodeOutput` for write-mode
    /// nodes — records exactly what *would* be written, including the
    /// safety-contract block the live run would honor. See
    /// `10-core/app-spec.md § Safety contract`.
    WouldWrite {
        ts: String,
        run_id: String,
        node: String,
        agent: String,
        command: String,
        proposed_inputs: serde_json::Value,
        safety: serde_json::Value,
    },
    RunEnd {
        ts: String,
        run_id: String,
        status: String,
    },
}

impl RunEvent {
    pub fn node_error(ts: String, run_id: String, node: String, error: &AwareError) -> Self {
        Self::NodeError {
            ts,
            run_id,
            node,
            error: error.to_string(),
            structured: error.structured_agent_error(),
        }
    }
}

pub fn run_id_now() -> String {
    uuid::Uuid::new_v4().to_string()
}

// Byte-identical to the copy that used to live in `builder::mod`. Both are
// now re-exports of `crate::time::now_iso`; existing imports
// (`use crate::runtime::provenance::{..., now_iso}` in `runtime::orchestrator`)
// still resolve.
pub use crate::time::now_iso;

pub fn log_dir_for(logs_dir: &Path, app: &str, instance: &str) -> PathBuf {
    logs_dir.join(app).join(instance)
}

pub fn log_path_for(logs_dir: &Path, app: &str, instance: &str, run_id: &str) -> PathBuf {
    log_dir_for(logs_dir, app, instance).join(format!("{run_id}.jsonl"))
}

/// Files produced during one run live beside its JSONL trace, never inside it.
/// The id is deliberately a filename, rather than an absolute path, so a trace
/// can hand an opaque artifact handle to another local consumer without granting
/// it access to arbitrary files.
pub fn artifact_dir_for(logs_dir: &Path, app: &str, instance: &str, run_id: &str) -> PathBuf {
    log_dir_for(logs_dir, app, instance).join(format!("{run_id}.artifacts"))
}

pub fn artifact_path_for(
    logs_dir: &Path,
    app: &str,
    instance: &str,
    run_id: &str,
    id: &str,
) -> Result<PathBuf, AwareError> {
    for (label, value) in [
        ("app", app),
        ("instance", instance),
        ("run id", run_id),
        ("artifact id", id),
    ] {
        validate_artifact_component(value, label)?;
    }
    Ok(artifact_dir_for(logs_dir, app, instance, run_id).join(id))
}

/// Path components supplied to artifact retrieval are opaque selectors, never paths.
pub fn validate_artifact_component(value: &str, label: &str) -> Result<(), AwareError> {
    if value.is_empty()
        || matches!(value, "." | "..")
        || !value
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || matches!(b, b'.' | b'-' | b'_'))
    {
        return Err(AwareError::Validation(format!(
            "artifact {label} {value:?} must be one path-safe identifier"
        )));
    }
    Ok(())
}

pub struct ProvenanceWriter {
    file: tokio::fs::File,
}

impl ProvenanceWriter {
    pub async fn open(path: &Path) -> Result<Self, AwareError> {
        if let Some(parent) = path.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)
            .await?;
        Ok(Self { file })
    }

    pub async fn write(&mut self, event: &RunEvent) -> Result<(), AwareError> {
        let mut line = serde_json::to_string(event)
            .map_err(|e| AwareError::Internal(format!("serialize event: {e}")))?;
        line.push('\n');
        self.file.write_all(line.as_bytes()).await?;
        self.file.flush().await?;
        Ok(())
    }
}

pub async fn read_run_events(path: &Path) -> Result<Vec<RunEvent>, AwareError> {
    let file = tokio::fs::File::open(path).await?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    let mut out = Vec::new();
    while let Some(line) = lines.next_line().await? {
        if line.trim().is_empty() {
            continue;
        }
        let evt: RunEvent = serde_json::from_str(&line)
            .map_err(|e| AwareError::Validation(format!("parse run event: {e}")))?;
        out.push(evt);
    }
    Ok(out)
}

/// Find the newest `.jsonl` file under `<logs_dir>/<app>/<instance>/`.
pub fn most_recent_run_id(logs_dir: &Path, app: &str, instance: &str) -> Option<String> {
    let dir = log_dir_for(logs_dir, app, instance);
    let mut best: Option<(std::time::SystemTime, String)> = None;
    for entry in std::fs::read_dir(&dir).ok()?.flatten() {
        let p = entry.path();
        if p.extension().is_some_and(|e| e == "jsonl")
            && let Ok(meta) = entry.metadata()
            && let Ok(modified) = meta.modified()
        {
            let stem = p.file_stem()?.to_string_lossy().to_string();
            match &best {
                None => best = Some((modified, stem)),
                Some((t, _)) if modified > *t => best = Some((modified, stem)),
                _ => {}
            }
        }
    }
    best.map(|(_, id)| id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn writes_and_reads_round_trip() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("test.jsonl");

        let mut w = ProvenanceWriter::open(&path).await.unwrap();
        w.write(&RunEvent::RunStart {
            ts: "2026-05-16T00:00:00Z".into(),
            run_id: "r1".into(),
            app: "welded-to-tc".into(),
            instance: "default".into(),
            config: serde_json::json!({}),
        })
        .await
        .unwrap();
        w.write(&RunEvent::NodeStart {
            ts: "2026-05-16T00:00:01Z".into(),
            run_id: "r1".into(),
            node: "tekla-watch".into(),
            agent: Some("tekla".into()),
            command: Some("watch".into()),
        })
        .await
        .unwrap();
        drop(w);

        let events = read_run_events(&path).await.unwrap();
        assert_eq!(events.len(), 2);
        if let RunEvent::RunStart { app, .. } = &events[0] {
            assert_eq!(app, "welded-to-tc");
        } else {
            panic!("expected RunStart");
        }
    }

    #[test]
    fn run_ids_are_unique_v4_uuids() {
        // Shape alone is not the property. A run id names one run's trace file and its
        // artifact directory, so the load-bearing guarantee is that two runs never collide —
        // and a test that only counted 36 characters and a hyphen stayed green against a
        // hard-coded constant, which is the one implementation that breaks every consumer.
        let a = run_id_now();
        let b = run_id_now();
        assert_ne!(a, b, "two runs must not share a trace file");
        let parsed = uuid::Uuid::parse_str(&a).unwrap_or_else(|e| panic!("{a:?}: {e}"));
        assert_eq!(
            parsed.get_version_num(),
            4,
            "run ids are random, not time- or \
             host-derived: a v1 id would leak the MAC address of the machine that ran the app"
        );
        assert_eq!(
            a.len(),
            36,
            "hyphenated form, so the id is a safe path component"
        );
    }

    #[test]
    fn node_error_preserves_bounded_structured_agent_fields() {
        let error = AwareError::AgentStructured {
            code: "reference-provider-timeout".into(),
            phase: "convert".into(),
            retryable: true,
            message: "The model provider timed out.".into(),
            diagnostic_id: "123e4567-e89b-12d3-a456-426614174000".into(),
        };
        let event = RunEvent::node_error("now".into(), "run".into(), "reader".into(), &error);
        let value = serde_json::to_value(event).unwrap();
        assert_eq!(value["structured"]["code"], "reference-provider-timeout");
        assert_eq!(value["structured"]["phase"], "convert");
        assert_eq!(value["structured"]["retryable"], true);
        assert_eq!(
            value["structured"]["diagnosticId"],
            "123e4567-e89b-12d3-a456-426614174000"
        );
        assert!(
            value["error"]
                .as_str()
                .unwrap()
                .contains("provider-timeout")
        );

        let ordinary = RunEvent::node_error(
            "now".into(),
            "run".into(),
            "reader".into(),
            &AwareError::Validation("bad input".into()),
        );
        assert!(
            serde_json::to_value(ordinary)
                .unwrap()
                .get("structured")
                .is_none()
        );
    }

    #[test]
    fn most_recent_finds_newest() {
        let tmp = tempfile::tempdir().unwrap();
        let dir = log_dir_for(tmp.path(), "app", "default");
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("r_old.jsonl"), "").unwrap();
        std::thread::sleep(std::time::Duration::from_millis(20));
        std::fs::write(dir.join("r_new.jsonl"), "").unwrap();
        let found = most_recent_run_id(tmp.path(), "app", "default").unwrap();
        assert_eq!(found, "r_new");
    }

    #[test]
    fn artifact_paths_are_run_scoped_and_reject_traversal() {
        let logs = Path::new("/tmp/aware/logs");
        assert_eq!(
            artifact_path_for(logs, "app", "default", "r1", "read-model.json").unwrap(),
            PathBuf::from("/tmp/aware/logs/app/default/r1.artifacts/read-model.json")
        );
        assert!(artifact_path_for(logs, "app", "default", "r1", "../secret").is_err());
        assert!(artifact_path_for(logs, "../app", "default", "r1", "result.json").is_err());
        assert!(artifact_path_for(logs, "app", "../other", "r1", "result.json").is_err());
        assert!(artifact_path_for(logs, "app", "default", "..", "result.json").is_err());
    }

    fn run_start(run_id: &str) -> RunEvent {
        RunEvent::RunStart {
            ts: "2026-08-30T00:00:00Z".into(),
            run_id: run_id.into(),
            app: "welded-to-tc".into(),
            instance: "default".into(),
            config: serde_json::json!({}),
        }
    }

    #[tokio::test]
    async fn a_corrupt_trace_line_is_reported_rather_than_skipped() {
        // The trace is evidence. `aware app output` and `app logs` reconstruct what a run did
        // from these lines, so a line that cannot be parsed means the reconstruction is
        // INCOMPLETE — and silently dropping it would answer "what did the run produce?" with a
        // confident, wrong answer instead of an error the operator can act on.
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("torn.jsonl");
        let good = serde_json::to_string(&run_start("r1")).unwrap();
        tokio::fs::write(&path, format!("{good}\n{{\"kind\":\"run-st\n"))
            .await
            .unwrap();

        let err = read_run_events(&path).await.unwrap_err();
        assert!(
            matches!(err, AwareError::Validation(ref m) if m.contains("parse run event")),
            "got: {err:?}"
        );
    }

    #[tokio::test]
    async fn blank_lines_between_events_are_not_corruption() {
        // A writer that is killed between the payload and its newline, and a file that simply
        // ends with one, both leave empty lines behind. Those carry no claim, so they must not
        // be read as a torn trace — the distinction the test above depends on.
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("gappy.jsonl");
        let first = serde_json::to_string(&run_start("r1")).unwrap();
        let second = serde_json::to_string(&RunEvent::RunEnd {
            ts: "2026-08-30T00:00:01Z".into(),
            run_id: "r1".into(),
            status: "ok".into(),
        })
        .unwrap();
        tokio::fs::write(&path, format!("\n{first}\n\n   \n{second}\n\n"))
            .await
            .unwrap();

        let events = read_run_events(&path).await.unwrap();
        assert_eq!(events.len(), 2, "blank lines must not become events");
        assert!(matches!(events[0], RunEvent::RunStart { .. }));
        assert!(matches!(events[1], RunEvent::RunEnd { .. }));
    }

    #[tokio::test]
    async fn one_event_occupies_one_line_however_its_fields_are_spelled() {
        // The trace is JSONL and `read_run_events` splits on newlines, so the framing holds only
        // as long as an event's own text can never contain one. An agent's error string is the
        // realistic carrier — a bridge stack trace is multi-line — and pretty-printing the event
        // would break every reader in the same way.
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("multiline.jsonl");
        let mut w = ProvenanceWriter::open(&path).await.unwrap();
        w.write(&RunEvent::node_error(
            "2026-08-30T00:00:00Z".into(),
            "r1".into(),
            "reader".into(),
            &AwareError::Validation("first line\nsecond line".into()),
        ))
        .await
        .unwrap();
        drop(w);

        let raw = tokio::fs::read_to_string(&path).await.unwrap();
        assert_eq!(raw.lines().count(), 1, "not one line: {raw:?}");
        assert!(
            raw.ends_with('\n'),
            "records are newline-TERMINATED: {raw:?}"
        );

        let events = read_run_events(&path).await.unwrap();
        let [RunEvent::NodeError { error, .. }] = events.as_slice() else {
            panic!("expected one NodeError, got {events:?}");
        };
        assert!(
            error.contains("first line\nsecond line"),
            "the newline must survive as data, not as framing: {error:?}"
        );
    }

    #[tokio::test]
    async fn reopening_a_trace_appends_to_it_and_creates_its_directory() {
        // A run's trace is opened more than once over its lifetime (the run loop and the stop
        // path both write to it), and the directory does not exist before the first open.
        // Truncating instead of appending would silently erase everything a run had recorded up
        // to that point, which is precisely the history an operator reaches for after a failure.
        let tmp = tempfile::tempdir().unwrap();
        let path = log_path_for(tmp.path(), "welded-to-tc", "default", "r1");
        assert!(!path.parent().unwrap().exists(), "fixture must start empty");

        let mut first = ProvenanceWriter::open(&path).await.unwrap();
        first.write(&run_start("r1")).await.unwrap();
        drop(first);

        let mut second = ProvenanceWriter::open(&path).await.unwrap();
        second
            .write(&RunEvent::RunEnd {
                ts: "2026-08-30T00:00:01Z".into(),
                run_id: "r1".into(),
                status: "ok".into(),
            })
            .await
            .unwrap();
        drop(second);

        let events = read_run_events(&path).await.unwrap();
        assert_eq!(events.len(), 2, "the first open's events were lost");
        assert!(matches!(events[0], RunEvent::RunStart { .. }));
    }

    #[test]
    fn most_recent_run_id_considers_only_jsonl_traces() {
        // `app logs`/`app output`/`app artifact` all fall back to "the newest run" when no id is
        // given, and the directory holds more than traces: each run also drops a
        // `<run-id>.artifacts` directory beside its `.jsonl`, and it is written AFTER the trace
        // is opened, so it is reliably the newer entry. Returning its name would resolve every
        // default-run lookup to a run id that has no trace at all.
        let tmp = tempfile::tempdir().unwrap();
        let dir = log_dir_for(tmp.path(), "welded-to-tc", "default");
        std::fs::create_dir_all(&dir).unwrap();
        let tick = || std::thread::sleep(std::time::Duration::from_millis(20));
        std::fs::write(dir.join("r_real.jsonl"), "").unwrap();
        tick();
        std::fs::create_dir_all(dir.join("r_real.artifacts")).unwrap();
        tick();
        std::fs::write(dir.join("r_newer.jsonl.tmp"), "").unwrap();
        tick();
        std::fs::write(dir.join("notes.txt"), "").unwrap();

        assert_eq!(
            most_recent_run_id(tmp.path(), "welded-to-tc", "default").as_deref(),
            Some("r_real"),
            "a newer non-trace entry must not be mistaken for the newest run"
        );
    }

    #[test]
    fn most_recent_run_id_is_none_when_the_app_has_never_run() {
        // The no-runs-yet case is ordinary, not exceptional: `app logs` on a freshly installed
        // app reaches this before any directory exists. It must report "no run" rather than
        // fail, so the command can print its own message instead of an io error about a path.
        let tmp = tempfile::tempdir().unwrap();
        assert_eq!(most_recent_run_id(tmp.path(), "never-run", "default"), None);

        // A directory that exists but holds no trace is the same answer.
        let dir = log_dir_for(tmp.path(), "started-once", "default");
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("notes.txt"), "").unwrap();
        assert_eq!(
            most_recent_run_id(tmp.path(), "started-once", "default"),
            None
        );
    }

    #[test]
    fn artifact_components_are_identifiers_not_paths() {
        // Every one of these reaches `artifact_dir_for(..).join(id)`, so anything `Path::join`
        // reads as a separator or a root escapes the run's own directory. The allowlist is the
        // fence; these are the spellings that would walk through a gap in it.
        for ok in ["read-model.json", "seg_1", "A0", "..foo", "x"] {
            assert!(
                validate_artifact_component(ok, "id").is_ok(),
                "{ok:?} is an ordinary artifact name"
            );
        }
        for bad in [
            "",           // joins to the directory itself
            ".",          // ditto
            "..",         // the parent run's directory
            "a/b",        // a separator on every platform
            "a\\b",       // a separator on Windows
            "/etc",       // absolute: `join` DISCARDS the base it is given one
            "C:",         // drive-relative on Windows
            "a b",        // shell-fragile, and outside the allowlist
            "seg-\u{e5}", // non-ASCII: normalises differently per filesystem
        ] {
            assert!(
                validate_artifact_component(bad, "id").is_err(),
                "{bad:?} must not reach a path join"
            );
        }
    }

    #[test]
    fn a_refused_artifact_component_says_which_one_it_was() {
        // Four different components are validated for one `aware app artifact` call. Naming the
        // offending one is the difference between a fixable message and a guess.
        let err =
            artifact_path_for(Path::new("/logs"), "app", "../other", "r1", "x.json").unwrap_err();
        let text = err.to_string();
        assert!(text.contains("instance"), "{text}");
        assert!(text.contains("../other"), "{text}");
        assert!(
            !text.contains("artifact id"),
            "must not blame the component that was fine: {text}"
        );
    }

    const TS: &str = "2026-08-30T00:00:00Z";
    const RUN: &str = "r1";
    const NODE: &str = "reader";

    /// Expands one list of trace variants into the whole fixture: `Kind`, [`Kind::ALL`],
    /// [`Kind::wire`], [`Kind::sample`] and [`kind_of`].
    ///
    /// The point is that there is exactly ONE place a variant can be named. `kind_of` matches
    /// `RunEvent` without a wildcard, so adding a variant to `RunEvent` stops this file
    /// compiling; and the only repair is an entry in the list below, which cannot be written
    /// without an external name AND a sample event, and which lands the variant in `ALL` at the
    /// same time. A `Kind` with no matching `RunEvent` variant fails the same way, so the two
    /// lists are one-to-one in both directions.
    ///
    /// Four earlier spellings each enforced less than they claimed, and Codex caught every one
    /// on this PR: `assert_eq!(cases.len(), 8)` was satisfied by the fixture that produced it;
    /// a wildcard-free `RunEvent -> &str` match forced a new arm but let the sample list go
    /// unchanged; a hand-maintained `ALL` array was one Rust does not check for exhaustiveness;
    /// and — the reason the fixture is generated rather than hand-written — a hand-written
    /// `kind_of` arm could satisfy the compiler by pointing a NEW variant at an EXISTING `Kind`
    /// (`RunEvent::NodeRetry { .. } => Kind::NodeError`), leaving the new variant never
    /// constructed and its wire name never pinned. Generating both sides from one list leaves
    /// no per-variant arm to write, so that repair is not available.
    macro_rules! trace_kinds {
        ($( $variant:ident => $wire:literal, $sample:expr ; )+) => {
            #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
            enum Kind {
                $( $variant, )+
            }

            impl Kind {
                const ALL: &'static [Kind] = &[ $( Kind::$variant, )+ ];

                /// The `kind` string this variant must serialize as. Hand-written on purpose:
                /// reading it back out of serde would compare serde with itself and pin nothing.
                fn wire(self) -> &'static str {
                    match self {
                        $( Kind::$variant => $wire, )+
                    }
                }

                /// An event of this variant, so a variant cannot be given a name here without
                /// also being given something to serialize.
                fn sample(self) -> RunEvent {
                    match self {
                        $( Kind::$variant => $sample, )+
                    }
                }
            }

            /// Which variant an event is, decided by the compiler rather than by its `kind`
            /// string — so the round trip below can assert the VARIANT came back, not merely a
            /// matching tag. Exhaustive over `RunEvent` by construction.
            fn kind_of(event: &RunEvent) -> Kind {
                match event {
                    $( RunEvent::$variant { .. } => Kind::$variant, )+
                }
            }
        };
    }

    trace_kinds! {
        RunStart => "run-start", run_start(RUN);
        NodeStart => "node-start", RunEvent::NodeStart {
            ts: TS.into(),
            run_id: RUN.into(),
            node: NODE.into(),
            agent: None,
            command: None,
        };
        NodeOutput => "node-output", RunEvent::NodeOutput {
            ts: TS.into(),
            run_id: RUN.into(),
            node: NODE.into(),
            data: serde_json::json!({}),
        };
        NodeProgress => "node-progress", RunEvent::NodeProgress {
            ts: TS.into(),
            run_id: RUN.into(),
            node: NODE.into(),
            data: serde_json::json!({}),
        };
        NodeError => "node-error", RunEvent::node_error(
            TS.into(),
            RUN.into(),
            NODE.into(),
            &AwareError::Validation("x".into()),
        );
        NodeStop => "node-stop", RunEvent::NodeStop {
            ts: TS.into(),
            run_id: RUN.into(),
            node: NODE.into(),
            reason: "cancelled".into(),
        };
        WouldWrite => "would-write", RunEvent::WouldWrite {
            ts: TS.into(),
            run_id: RUN.into(),
            node: NODE.into(),
            agent: "tekla".into(),
            command: "write".into(),
            proposed_inputs: serde_json::json!({}),
            safety: serde_json::json!({}),
        };
        RunEnd => "run-end", RunEvent::RunEnd {
            ts: TS.into(),
            run_id: RUN.into(),
            status: "ok".into(),
        };
    }

    #[test]
    fn every_trace_event_carries_its_documented_kind() {
        // The `kind` strings are the trace's external contract — `aware app logs`, `app output`
        // and downstream readers switch on them, and a trace already on disk is read by a LATER
        // build of the CLI. Renaming one is a silent, unversioned break, so they are pinned here
        // rather than left to whatever `rename_all` happens to be set to.
        let wires: std::collections::BTreeSet<&str> = Kind::ALL.iter().map(|k| k.wire()).collect();
        assert_eq!(
            wires.len(),
            Kind::ALL.len(),
            "two variants claim one kind, so a reader cannot tell them apart: {wires:?}"
        );

        for &kind in Kind::ALL {
            let event = kind.sample();
            assert_eq!(
                kind_of(&event),
                kind,
                "the sample for {kind:?} is a different variant"
            );

            let value = serde_json::to_value(&event).unwrap();
            assert_eq!(
                value["kind"],
                kind.wire(),
                "wrong discriminator for {kind:?}"
            );
            // The discriminator is inline, not a wrapper object: a reader matches on
            // `.kind` and reads the payload's fields from the same map.
            assert_eq!(value["ts"], TS, "{kind:?} lost its timestamp");
            assert_eq!(value["run_id"], RUN, "{kind:?} lost its run id");

            // And the tag routes back to the SAME variant through the reader — the property a
            // later build of the CLI depends on when it opens an older trace.
            let line = serde_json::to_string(&event).unwrap();
            let back: RunEvent = serde_json::from_str(&line).unwrap();
            assert_eq!(
                kind_of(&back),
                kind,
                "{kind:?} did not survive the round trip"
            );
        }
    }
}
