//! The progress channel a long-running command publishes on while it works (#405).
//!
//! #402 made a large result affordable: the payload becomes a run-owned artifact and the node
//! output shrinks to a descriptor. It did nothing for *when* the payload arrives — the consumer
//! still learns nothing until the last triangle is tessellated, because a `single`-lifecycle
//! command has exactly one output and it is emitted after the process exits. On the file that
//! motivated this (PALM.ifc, 66 MiB, 2,993 objects, ~305 MiB of geometry) that is ~43 s of silence
//! before the consumer may draw anything.
//!
//! So a CLI transport is handed a second, WRITE-ONLY channel: a file path in `AWARE_PROGRESS_FILE`,
//! inside the same run-owned artifact directory. The producer appends newline-delimited
//! `{"$aware-progress": {...}}` records to it as it goes; the runtime tails the file and mirrors
//! each valid record into the run trace as a `node-progress` event, flushed immediately. A consumer
//! tailing the trace (`aware app logs <app> --tail`, with the run id `aware app run` prints before
//! the first node starts) therefore sees phases, counters, and — the point of the exercise —
//! ORDERED ARTIFACT SEGMENTS it can fetch and render while the run continues.
//!
//! A file rather than stderr, deliberately. stderr is the failure channel: `failure_detail` reports
//! it verbatim when a bridge exits non-zero, and bridges already write engine noise there (web-ifc
//! prints during tessellation). Multiplexing a machine protocol onto it would make the diagnostic
//! that a user acts on compete with the protocol, and any producer that got the framing wrong would
//! corrupt both at once. A separate path costs one env var and keeps each channel able to say one
//! thing.

use std::path::{Path, PathBuf};

use serde_json::Value;

/// The wrapper key a producer's record must carry. Deliberately the same `$aware-` namespace as
/// `$aware-artifact` (#402): both are runtime protocol inside an otherwise agent-owned document.
pub const RECORD_KEY: &str = "$aware-progress";

/// Ceiling on ONE progress record, in bytes.
///
/// This is what keeps the mechanism honest. The whole point of #402 was that a trace record must
/// never be proportional to the payload; a progress channel that mirrored arbitrary JSON into the
/// trace would hand that property straight back — a producer could stream the mesh through it one
/// "progress" record at a time and the trace would be as large as before. 8 KiB is far more than a
/// phase, a pair of counters, a message and a segment descriptor need, and far less than any
/// geometry batch. An over-long record is DROPPED rather than truncated: half a JSON document is
/// not a smaller fact, it is a different one.
pub const MAX_RECORD_BYTES: usize = 8 * 1024;

/// Longest `phase` we mirror. A phase is a label a consumer switches on, not free text.
const MAX_PHASE_CHARS: usize = 64;

/// How often the runtime looks for new records while a node runs.
///
/// The cost of a poll is one seek + read on a file that is usually unchanged; the benefit is that a
/// consumer's first batch arrives this much sooner. 50 ms is below the threshold at which a person
/// perceives the first geometry as "late" while leaving the tessellation loop essentially all of
/// the machine.
pub const POLL_INTERVAL: std::time::Duration = std::time::Duration::from_millis(50);

/// Parse one line of the progress channel into the record body to mirror.
///
/// Returns `None` for anything that is not a well-formed, bounded progress record — a blank line, a
/// partially-flushed line, engine noise a producer redirected here by mistake, or a record whose
/// segment descriptor names an id the artifact command could not safely resolve. Skipping is
/// deliberate and silent: this channel is advisory, and a malformed record must not fail a run that
/// is otherwise producing correct geometry. The authoritative result is still the node output.
pub fn parse_record(line: &str) -> Option<Value> {
    let line = line.trim();
    if line.is_empty() || line.len() > MAX_RECORD_BYTES {
        return None;
    }
    let parsed: Value = serde_json::from_str(line).ok()?;
    let body = parsed.get(RECORD_KEY)?;
    let map = body.as_object()?;

    // `phase` is required and bounded: a consumer branches on it (does this record mean "still
    // parsing" or "a batch is ready to draw"?), so a record without one carries no decision.
    let phase = map.get("phase")?.as_str()?;
    if phase.is_empty() || phase.chars().count() > MAX_PHASE_CHARS {
        return None;
    }

    // A segment descriptor is an INSTRUCTION to go and read a file, so its id is fenced here with
    // the same rule `aware app artifact` applies. Validating at the boundary means a consumer can
    // hand the id straight to the artifact command; letting an unsafe id through and rejecting it
    // later would put a traversal attempt into the trace as though the runtime had endorsed it.
    if let Some(artifact) = map.get("artifact") {
        let id = artifact.as_object()?.get("id")?.as_str()?;
        crate::runtime::provenance::validate_artifact_component(id, "id").ok()?;
    }

    Some(body.clone())
}

/// A cursor over a producer's progress file.
///
/// Reads only what is new since the last drain and holds back a trailing partial line, because the
/// producer appends with no locking and a read can land mid-record. Missing file is not an error:
/// the overwhelmingly common case is a producer that publishes no progress at all, and that must
/// cost nothing.
pub struct ProgressTail {
    path: PathBuf,
    offset: u64,
    partial: String,
}

impl ProgressTail {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self {
            path: path.into(),
            offset: 0,
            partial: String::new(),
        }
    }

    /// Every complete record written since the previous call, in the order the producer wrote them.
    pub async fn drain(&mut self) -> Vec<Value> {
        use tokio::io::{AsyncReadExt, AsyncSeekExt};

        let Ok(mut file) = tokio::fs::File::open(&self.path).await else {
            return Vec::new();
        };
        if file
            .seek(std::io::SeekFrom::Start(self.offset))
            .await
            .is_err()
        {
            return Vec::new();
        }
        let mut fresh = Vec::new();
        if file.read_to_end(&mut fresh).await.is_err() || fresh.is_empty() {
            return Vec::new();
        }
        self.offset += fresh.len() as u64;
        // Lossy on purpose: the offset counts BYTES, so a multi-byte character split across two
        // reads would otherwise desynchronise the cursor from the file. The replacement character
        // fails `parse_record` and the record is skipped, which is the right outcome for a line
        // whose bytes we only half have.
        self.partial.push_str(&String::from_utf8_lossy(&fresh));

        let mut out = Vec::new();
        // The tail after the last newline is a record still being written — keep it for next time.
        let keep_from = match self.partial.rfind('\n') {
            Some(idx) => idx + 1,
            // No newline at all: nothing is complete yet. Guard the unbounded case — a producer
            // that opened the channel and wrote megabytes without a newline is not speaking this
            // protocol, and buffering it forever would be the memory blow-up #402 removed.
            None => {
                if self.partial.len() > MAX_RECORD_BYTES {
                    self.partial.clear();
                }
                return out;
            }
        };
        let complete = self.partial[..keep_from].to_string();
        self.partial.drain(..keep_from);
        for line in complete.lines() {
            if let Some(record) = parse_record(line) {
                out.push(record);
            }
        }
        out
    }
}

/// The channel path handed to one invocation, inside its run-owned artifact directory.
///
/// One per invocation, not one per run: an app can call the same command from several nodes (and a
/// `for-each` calls it once per item), and a shared file would interleave two producers' records
/// with no way to tell whose is whose.
pub fn channel_path(artifact_dir: &Path) -> PathBuf {
    artifact_dir.join(format!("progress-{}.jsonl", uuid::Uuid::new_v4()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn accepts_a_well_formed_record_and_returns_its_body() {
        let body =
            parse_record(r#"{"$aware-progress":{"phase":"tessellate","done":12,"total":99}}"#)
                .expect("well-formed record");
        assert_eq!(body["phase"], "tessellate");
        assert_eq!(body["done"], 12);
    }

    #[test]
    fn rejects_lines_that_are_not_progress_records() {
        // Engine noise, an unrelated JSON document, and a blank line all mean "no record here".
        assert!(parse_record("web-ifc: loading geometry").is_none());
        assert!(parse_record(r#"{"objects":[]}"#).is_none());
        assert!(parse_record("   ").is_none());
        // A truncated line (the producer was mid-write) must not parse as anything.
        assert!(parse_record(r#"{"$aware-progress":{"phase":"bat"#).is_none());
    }

    #[test]
    fn requires_a_bounded_phase() {
        assert!(parse_record(r#"{"$aware-progress":{"done":1}}"#).is_none());
        assert!(parse_record(r#"{"$aware-progress":{"phase":""}}"#).is_none());
        let long = "p".repeat(MAX_PHASE_CHARS + 1);
        assert!(parse_record(&format!(r#"{{"$aware-progress":{{"phase":"{long}"}}}}"#)).is_none());
    }

    #[test]
    fn refuses_a_record_larger_than_the_cap() {
        // The guarantee this exists for: no trace record proportional to the payload. A producer
        // that tries to push geometry through the progress channel is dropped, not truncated.
        let payload = "9,".repeat(MAX_RECORD_BYTES);
        let line = json!({ RECORD_KEY: { "phase": "batch", "positions": payload } }).to_string();
        assert!(line.len() > MAX_RECORD_BYTES);
        assert!(parse_record(&line).is_none());
    }

    #[test]
    fn refuses_a_segment_descriptor_with_an_unsafe_id() {
        // The id is handed to `aware app artifact`; a traversal must be refused at the boundary
        // rather than mirrored into the trace as an endorsed instruction.
        assert!(
            parse_record(r#"{"$aware-progress":{"phase":"batch","artifact":{"id":"../secret"}}}"#)
                .is_none()
        );
        assert!(
            parse_record(r#"{"$aware-progress":{"phase":"batch","artifact":{"id":"a/b.json"}}}"#)
                .is_none()
        );
        assert!(
            parse_record(
                r#"{"$aware-progress":{"phase":"batch","artifact":{"id":"seg-1.json","seq":1}}}"#
            )
            .is_some()
        );
    }

    #[tokio::test]
    async fn tail_yields_only_new_complete_records() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("progress.jsonl");
        let mut tail = ProgressTail::new(&path);
        // Nothing written yet — and no file at all — is the common case and yields nothing.
        assert!(tail.drain().await.is_empty());

        tokio::fs::write(
            &path,
            "{\"$aware-progress\":{\"phase\":\"parse\"}}\n{\"$aware-progress\":{\"phase\":\"batch\",\"seq\":1}}\n",
        )
        .await
        .unwrap();
        let first = tail.drain().await;
        assert_eq!(first.len(), 2);
        assert_eq!(first[0]["phase"], "parse");
        assert_eq!(first[1]["seq"], 1);

        // A second drain returns only what is new, never a replay.
        assert!(tail.drain().await.is_empty());

        // A half-written line is held back until its newline arrives.
        tokio::fs::write(
            &path,
            "{\"$aware-progress\":{\"phase\":\"parse\"}}\n{\"$aware-progress\":{\"phase\":\"batch\",\"seq\":1}}\n{\"$aware-progress\":{\"phase\":\"comp",
        )
        .await
        .unwrap();
        assert!(tail.drain().await.is_empty());
        let mut file = tokio::fs::OpenOptions::new()
            .append(true)
            .open(&path)
            .await
            .unwrap();
        tokio::io::AsyncWriteExt::write_all(&mut file, b"lete\"}}\n")
            .await
            .unwrap();
        let last = tail.drain().await;
        assert_eq!(last.len(), 1);
        assert_eq!(last[0]["phase"], "complete");
    }

    #[tokio::test]
    async fn tail_drops_an_unterminated_flood_instead_of_buffering_it() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("progress.jsonl");
        let mut tail = ProgressTail::new(&path);
        tokio::fs::write(&path, "x".repeat(MAX_RECORD_BYTES * 2))
            .await
            .unwrap();
        assert!(tail.drain().await.is_empty());
        assert!(
            tail.partial.is_empty(),
            "a producer that never writes a newline must not grow the runtime's memory"
        );
    }

    #[test]
    fn channel_paths_are_unique_per_invocation() {
        let dir = Path::new("/tmp/run.artifacts");
        let a = channel_path(dir);
        let b = channel_path(dir);
        assert_ne!(a, b, "two invocations must not share one channel");
        // The channel lives in the artifact directory, so its name must satisfy the same fence the
        // artifact command applies — otherwise the file it writes is unreachable and unexplained.
        let name = a.file_name().unwrap().to_string_lossy().to_string();
        assert!(crate::runtime::provenance::validate_artifact_component(&name, "id").is_ok());
    }
}
