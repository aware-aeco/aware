//! Test helpers that write a promoted lock BY HAND — #628 PR3a ships no way to
//! create a successor (that is PR3b), so the reading side is tested against
//! locks these helpers build exactly as the plan (§5, §12, §13) lays them out:
//! the candidate compiled by the real `compile_candidate`, the outgoing lock,
//! the resulting plan, the evidence and the person approval record archived
//! content-addressed, and the chain appended.

use super::*;
use crate::agent_resolution::{PinSet, PinTarget};

/// Who carries the approval forward in a hand-built link.
pub(crate) enum By {
    Person(&'static str),
    Policy(&'static str),
}

/// What a hand promotion wrote.
pub(crate) struct Promoted {
    pub lock_path: std::path::PathBuf,
    pub lock: LockFile,
    pub bytes: Vec<u8>,
    /// Exact bytes of the lock the promotion replaced.
    pub replaced: Vec<u8>,
}

/// Write `bytes` to `.aware-approvals/<hex>.<ext>` beside `dir`; return the digest.
pub(crate) fn archive(dir: &Path, bytes: &[u8], ext: &str) -> String {
    let digest = lock_digest(bytes);
    let rel = archive_rel(&digest, ext).unwrap();
    let path = dir.join(rel);
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    std::fs::write(path, bytes).unwrap();
    digest
}

/// Promote the compiled app at `source` to `targets`, recording a successor
/// of `kind` carried forward `by`.
pub(crate) fn promote(
    paths: &Paths,
    source: &Path,
    targets: BTreeMap<String, PinTarget>,
    by: By,
    kind: SuccessorKind,
) -> Promoted {
    let dir = crate::fs::containing_dir(source).to_path_buf();
    let (app, _) = read_app_source(source).unwrap();
    let lock_path = dir.join(format!("{}.lock", app.app));
    let base_bytes = std::fs::read(&lock_path).unwrap();
    let base: LockFile = serde_yaml::from_slice(&base_bytes).unwrap();
    let pins = PinSet::from_lock(&base, targets).unwrap();
    let candidate = compile_candidate(source, paths, &base, &base_bytes, &pins).unwrap();
    assert!(candidate.blocked.is_empty(), "{:?}", candidate.blocked);

    let from_lock_digest = archive(&dir, &base_bytes, "lock");
    let resulting_lock_digest = archive(&dir, &candidate.bytes, "lock");
    assert_eq!(resulting_lock_digest, candidate.header.candidate_digest);
    let evidence = serde_json::json!({
        "format": crate::migration::files::EVIDENCE_FORMAT,
        "app": app.app,
        "prepared-at": "2026-10-05T00:00:00Z",
        "cli-version": env!("CARGO_PKG_VERSION"),
        "header": candidate.header,
        "row": {
            "effect": "declared-read-only",
            "comparison": { "status": "identical-instructions", "runs": 0 },
        },
    });
    let evidence_digest = archive(&dir, evidence.to_string().as_bytes(), "json");
    let carried_forward_by = match by {
        By::Person(actor) => {
            let record = serde_json::json!({
                "format": 1,
                "kind": "person",
                "actor": actor,
                "front-door": "floless@test",
                "approval-ref": "batch-1",
                "candidate-digest": candidate.header.candidate_digest,
                "base-lock-digest": from_lock_digest,
                "plan-digest": candidate.header.plan_digest,
                "statement-sha256": lock_digest(b"I approve"),
                "at": "2026-10-05T00:00:00Z",
            });
            CarriedForwardBy::Person {
                actor: actor.into(),
                approval_ref: "batch-1".into(),
                front_door: "floless@test".into(),
                attested: false,
                approval_record_digest: archive(&dir, record.to_string().as_bytes(), "json"),
            }
        }
        By::Policy(id) => CarriedForwardBy::Policy {
            policy_id: id.into(),
            policy_digest: lock_digest(id.as_bytes()),
            policy_approved_by: "pawel".into(),
        },
    };
    let mut chain = base.approval.clone().unwrap_or_else(|| ApprovalChain {
        format: APPROVAL_FORMAT,
        original: OriginalApproval {
            lock_digest: from_lock_digest.clone(),
            archive: archive_rel(&from_lock_digest, "lock").unwrap(),
            compiled_at: base.compiled_at.clone(),
            compiler_version: base.compiler_version.clone(),
            front_door: base.front_door.clone(),
            agent_pins: base.agent_pins.clone(),
            agent_digests: base.agent_digests.clone(),
            agent_bundle_pins: base.agent_bundle_pins.clone(),
        },
        successors: Vec::new(),
    });
    chain.successors.push(Successor {
        seq: u32::try_from(chain.successors.len()).unwrap() + 1,
        kind,
        from_lock_digest,
        to_plan_digest: candidate.header.plan_digest.clone(),
        resulting_lock_digest,
        from: pins_of(&base),
        to: pins_of(&candidate.lock),
        carried_forward_by,
        evidence: archive_rel(&evidence_digest, "json").unwrap(),
        evidence_digest,
        front_door: "floless@test".into(),
        cli_version: env!("CARGO_PKG_VERSION").into(),
        promoted_at: "2026-10-05T00:00:00Z".into(),
    });
    let mut lock = candidate.lock;
    let hex = approved_source_hash(&lock)
        .strip_prefix("sha256:")
        .unwrap()
        .to_string();
    lock.source_hash = format!("{SUCCESSOR_SOURCE_PREFIX}{hex}");
    lock.approval = Some(chain);
    let bytes = write_lock(&lock_path, &lock);
    Promoted {
        lock_path,
        lock,
        bytes,
        replaced: base_bytes,
    }
}

/// Render `lock` as a promoted lock file and write it at `path`.
pub(crate) fn write_lock(path: &Path, lock: &LockFile) -> Vec<u8> {
    let header = format!(
        "# {}.lock — approval carried forward; see approval:\n# DO NOT EDIT.\n\n",
        lock.app
    );
    let bytes = render_lock(lock, &header).unwrap();
    std::fs::write(path, &bytes).unwrap();
    bytes
}

/// Read the lock at `path`, change it with `edit`, write it back.
pub(crate) fn tamper(path: &Path, edit: impl FnOnce(&mut LockFile)) {
    let mut lock: LockFile = serde_yaml::from_slice(&std::fs::read(path).unwrap()).unwrap();
    edit(&mut lock);
    write_lock(path, &lock);
}
