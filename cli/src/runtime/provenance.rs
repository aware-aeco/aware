use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use tokio::fs::OpenOptions;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

use crate::error::AwareError;

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
    NodeError {
        ts: String,
        run_id: String,
        node: String,
        error: String,
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

pub fn run_id_now() -> String {
    uuid::Uuid::new_v4().to_string()
}

pub fn now_iso() -> String {
    chrono::Utc::now().to_rfc3339()
}

pub fn log_dir_for(logs_dir: &Path, app: &str, instance: &str) -> PathBuf {
    logs_dir.join(app).join(instance)
}

pub fn log_path_for(logs_dir: &Path, app: &str, instance: &str, run_id: &str) -> PathBuf {
    log_dir_for(logs_dir, app, instance).join(format!("{run_id}.jsonl"))
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
    fn run_id_now_is_uuid_format() {
        let id = run_id_now();
        assert_eq!(id.len(), 36);
        assert!(id.contains('-'));
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
}
