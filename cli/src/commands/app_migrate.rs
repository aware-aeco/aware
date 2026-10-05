//! `aware app migrate …` (#628): plan, prepare, discard, hold, unhold (PR2);
//! promote, revert and the policy verbs (PR3b).
//!
//! Only `promote` and `revert` touch `<app>.lock` (see
//! [`crate::migration::promote`]). `prepare` writes only the candidate +
//! evidence under `<source-dir>/.aware-migration/`; `hold` writes only
//! `<source-dir>/.aware-approvals/HOLD.<app>`; the policy verbs write only
//! `AWARE_HOME/migration-policies/`. Expected outcomes of `plan`/`prepare` —
//! up to date, needs a person, blocked, held — are `ok: true` data; `ok:
//! false` only when the command cannot run or a promotion is refused (each
//! refusal has its own `E_MIGRATE_…` code and changes nothing).

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::time::Instant;

use clap::Subcommand;
use serde::Serialize;

use crate::agent_resolution::PinTarget;
use crate::app_lock::approval::SuccessorKind;
use crate::context::Context;
use crate::envelope;
use crate::error::AwareError;
use crate::migration::files::{self, HoldRecord};
use crate::migration::plan::{self, PlanRow, State};
use crate::migration::policy;
use crate::migration::promote::{self, Approver, Refused};

#[derive(Subcommand, Debug)]
pub enum MigrateCommand {
    /// Report, per app, whether an agent update can carry its approval
    /// forward and who must decide. Writes nothing.
    Plan {
        /// An installed app id or a path to its source. Repeatable.
        #[arg(long = "app", conflicts_with = "all")]
        apps: Vec<String>,
        /// Every installed app (the default when no --app is given).
        #[arg(long)]
        all: bool,
        /// Move an agent to `<agent>@sha256:<hex>` or `<agent>@<version>`
        /// instead of to its installed copy. Repeatable.
        #[arg(long = "to")]
        to: Vec<String>,
    },
    /// Compile a candidate for one app against the new agent pins and write
    /// it, with its evidence, beside the source. The approved lock is never
    /// touched.
    Prepare {
        app: String,
        #[arg(long = "to")]
        to: Vec<String>,
    },
    /// Remove an app's candidate and evidence.
    Discard { app: String },
    /// Put an app on hold: it is sealed, certified or frozen and must not be
    /// carried forward.
    Hold {
        app: String,
        /// Who is holding it.
        #[arg(long)]
        actor: String,
        /// Why, in plain words.
        #[arg(long)]
        reason: Option<String>,
    },
    /// Lift a hold.
    Unhold {
        app: String,
        #[arg(long)]
        actor: String,
    },
    /// Carry an app's approval forward to its prepared candidate: replace
    /// `<app>.lock` and append a successor link. Needs a person's recorded
    /// approval (`--person --approval --front-door`) or a policy (`--policy`).
    Promote {
        app: String,
        /// The `sha256:` digest of the prepared candidate being approved.
        #[arg(long)]
        candidate: String,
        /// Who approved it (a claim the front door recorded).
        #[arg(long)]
        person: Option<String>,
        /// The front door's approval record (JSON) bound to this candidate.
        #[arg(long)]
        approval: Option<PathBuf>,
        /// Carry it forward under this person-approved policy instead.
        #[arg(long)]
        policy: Option<String>,
        /// The front door recording this promotion (required with --person).
        #[arg(long = "front-door")]
        front_door: Option<String>,
    },
    /// Move an app back to pins that were approved before: promote a
    /// candidate prepared with `--to <agent>@sha256:<earlier digest>` as a
    /// `reverted` successor. Never claims anything was undone.
    Revert {
        app: String,
        #[arg(long)]
        candidate: String,
        #[arg(long)]
        person: Option<String>,
        #[arg(long)]
        approval: Option<PathBuf>,
        /// A caller's policy reverting after a failed run (`--reason
        /// run-failed:<run-id>`).
        #[arg(long)]
        automatic: bool,
        #[arg(long)]
        reason: Option<String>,
        #[arg(long = "front-door")]
        front_door: Option<String>,
    },
    /// Carry-forward policies: record, list, revoke.
    Policy {
        #[command(subcommand)]
        cmd: PolicyCommand,
    },
}

#[derive(Subcommand, Debug)]
pub enum PolicyCommand {
    /// Record the policy a person approved, from the front door's record.
    Record {
        /// The front door's policy approval record (JSON).
        #[arg(long)]
        approval: PathBuf,
        #[arg(long = "front-door")]
        front_door: Option<String>,
    },
    /// List every policy: active, revoked, or invalid.
    List,
    /// Revoke a policy. It stays on disk, marked revoked.
    Revoke {
        id: String,
        #[arg(long)]
        actor: String,
        #[arg(long = "front-door")]
        front_door: Option<String>,
        #[arg(long)]
        reason: Option<String>,
    },
}

pub fn dispatch(cmd: MigrateCommand, ctx: &Context) -> Result<(), AwareError> {
    let started = Instant::now();
    // #627: every migrate verb reads approvals and writes or reads candidate
    // locks — store references — so each runs under the store reference lock,
    // shared, taken before anything is read (plan R1-4, R1-7). `plan` and
    // `prepare` also hash installed working copies to propose targets; they
    // take those agents' swap locks shared under this guard.
    let store_guard = crate::agent_store::open(&ctx.paths)?;
    let (name, outcome) = match cmd {
        MigrateCommand::Plan { apps, all: _, to } => {
            ("app migrate plan", plan_cmd(ctx, &apps, &to, &store_guard))
        }
        MigrateCommand::Prepare { app, to } => (
            "app migrate prepare",
            prepare_cmd(ctx, &app, &to, &store_guard),
        ),
        MigrateCommand::Discard { app } => {
            ("app migrate discard", discard_cmd(ctx, &app, &store_guard))
        }
        MigrateCommand::Hold { app, actor, reason } => (
            "app migrate hold",
            hold_cmd(ctx, &app, &actor, reason, &store_guard),
        ),
        MigrateCommand::Unhold { app, actor } => {
            ("app migrate unhold", unhold_cmd(ctx, &app, &actor))
        }
        MigrateCommand::Promote {
            app,
            candidate,
            person,
            approval,
            policy,
            front_door,
        } => (
            "app migrate promote",
            promote_cmd(
                ctx,
                &store_guard,
                &app,
                &candidate,
                ApproverArgs {
                    person,
                    approval,
                    policy,
                    automatic: false,
                    reason: None,
                    front_door,
                },
                SuccessorKind::CarriedForward,
            ),
        ),
        MigrateCommand::Revert {
            app,
            candidate,
            person,
            approval,
            automatic,
            reason,
            front_door,
        } => (
            "app migrate revert",
            promote_cmd(
                ctx,
                &store_guard,
                &app,
                &candidate,
                ApproverArgs {
                    person,
                    approval,
                    policy: None,
                    automatic,
                    reason,
                    front_door,
                },
                SuccessorKind::Reverted,
            ),
        ),
        MigrateCommand::Policy { cmd } => match cmd {
            PolicyCommand::Record {
                approval,
                front_door,
            } => (
                "app migrate policy record",
                policy_record_cmd(ctx, &approval, front_door),
            ),
            PolicyCommand::List => ("app migrate policy list", policy_list_cmd(ctx)),
            PolicyCommand::Revoke {
                id,
                actor,
                front_door,
                reason,
            } => (
                "app migrate policy revoke",
                policy_revoke_cmd(ctx, &id, &actor, front_door, reason),
            ),
        },
    };
    match outcome {
        Ok(output) => {
            if ctx.json {
                envelope::print_ok(name, &output.data, started)?;
            } else {
                print!("{}", output.text);
            }
            Ok(())
        }
        Err(failure) if ctx.json => {
            let env = envelope::Envelope::<()> {
                ok: false,
                data: None,
                error: Some(envelope::EnvelopeError {
                    code: failure.code.clone(),
                    message: failure.error.to_string(),
                    details: failure.details.clone(),
                }),
                meta: envelope::meta_for(name, started),
            };
            println!("{}", serde_json::to_string(&env)?);
            use std::io::Write;
            let _ = std::io::stdout().flush();
            std::process::exit(failure.error.exit_code());
        }
        Err(failure) => Err(*failure.error),
    }
}

struct Output {
    data: serde_json::Value,
    text: String,
}

struct Failure {
    code: String,
    error: Box<AwareError>,
    details: serde_json::Value,
}

impl From<AwareError> for Failure {
    fn from(error: AwareError) -> Self {
        let code = match &error {
            AwareError::NotFound(_) => "E_MIGRATE_APP_NOT_FOUND",
            _ => "E_MIGRATE_FAILED",
        };
        Failure {
            code: code.into(),
            error: Box::new(error),
            details: serde_json::Value::Null,
        }
    }
}

impl From<Refused> for Failure {
    fn from(refused: Refused) -> Self {
        Failure {
            code: refused.code,
            error: refused.error,
            details: refused.details,
        }
    }
}

fn to_json<T: Serialize>(value: &T) -> Result<serde_json::Value, Failure> {
    serde_json::to_value(value).map_err(|e| AwareError::Internal(e.to_string()).into())
}

/// The source file of an installed app id, or of a path to a source/app dir.
fn app_source(ctx: &Context, app: &str) -> Result<PathBuf, AwareError> {
    let path = Path::new(app);
    let exists = path.try_exists().map_err(|error| {
        std::io::Error::new(error.kind(), format!("{}: {error}", path.display()))
    })?;
    if exists {
        return crate::app_lock::find_app_source(path).ok_or_else(|| {
            AwareError::NotFound(format!(
                "no app source file (.flo / .app / .flow / .aware) at {}",
                path.display()
            ))
        });
    }
    let dir = crate::manifest::loader::resolve_app_dir(&ctx.paths, app)?;
    crate::manifest::loader::find_app_manifest(&dir)
        .ok_or_else(|| AwareError::NotFound(format!("app {app} has no .flo/.app file")))
}

fn app_id(source: &Path) -> Result<String, AwareError> {
    crate::app_lock::read_app_source(source).map(|(app, _)| app.app)
}

/// Parse `--to` values: `<agent>@sha256:<hex>` or `<agent>@<version>`.
pub fn parse_targets(values: &[String]) -> Result<BTreeMap<String, PinTarget>, AwareError> {
    let mut out = BTreeMap::new();
    for value in values {
        let Some((agent, target)) = value.split_once('@') else {
            return Err(bad_target(
                value,
                "expected <agent>@sha256:<hex> or <agent>@<version>",
            ));
        };
        if !crate::manifest::loader::is_safe_segment(agent) || target.is_empty() {
            return Err(bad_target(
                value,
                "expected <agent>@sha256:<hex> or <agent>@<version>",
            ));
        }
        let target = if target.starts_with("sha256:") {
            if crate::agent_store::digest_hex(target).is_none() {
                return Err(bad_target(
                    value,
                    "a digest is sha256: followed by 64 lowercase hex characters",
                ));
            }
            PinTarget::Digest(target.to_string())
        } else {
            PinTarget::Version(target.to_string())
        };
        if out.insert(agent.to_string(), target).is_some() {
            return Err(bad_target(value, "the agent is named twice"));
        }
    }
    Ok(out)
}

fn bad_target(value: &str, why: &str) -> AwareError {
    AwareError::Validation(format!("[E_MIGRATE_BAD_TARGET] --to {value}: {why}"))
}

fn usage_failure(error: AwareError) -> Failure {
    let code = crate::migration::reason_from_error(&error).error_code();
    Failure {
        code,
        error: Box::new(error),
        details: serde_json::Value::Null,
    }
}

// ── plan ────────────────────────────────────────────────────────────────────

#[derive(Serialize)]
struct PlanData {
    apps: Vec<PlanRow>,
}

fn plan_cmd(
    ctx: &Context,
    apps: &[String],
    to: &[String],
    guard: &crate::agent_store::RefGuard,
) -> Result<Output, Failure> {
    let targets = parse_targets(to).map_err(usage_failure)?;
    let mut sources = Vec::new();
    if apps.is_empty() {
        sources = installed_sources(ctx)?;
    } else {
        for app in apps {
            let source = app_source(ctx, app)?;
            let id = app_id(&source).unwrap_or_else(|_| app.clone());
            sources.push((id, source));
        }
    }
    let requested = (!targets.is_empty()).then_some(&targets);
    let rows = plan::plan_rows(&ctx.paths, &sources, requested, guard);
    let mut text = String::new();
    for row in &rows {
        text.push_str(&format!("{}: {}\n", row.app, state_word(row.state)));
        for target in &row.targets {
            text.push_str(&format!(
                "  {} {} -> {} ({})\n",
                target.agent, target.from.version, target.to.version, target.bump
            ));
        }
        for reason in &row.reasons {
            text.push_str(&format!("  - {}\n", reason.text));
        }
        for warning in &row.warnings {
            text.push_str(&format!("  \u{26a0} {}\n", warning.text));
        }
    }
    if rows.is_empty() {
        text.push_str("no installed apps\n");
    }
    Ok(Output {
        data: to_json(&PlanData { apps: rows })?,
        text,
    })
}

/// Every installed app's source, sorted by app id. A directory with no source
/// is skipped; one whose source cannot be read is still listed, by its
/// directory name, and reported as a blocked row.
fn installed_sources(ctx: &Context) -> Result<Vec<(String, PathBuf)>, Failure> {
    let apps_dir = ctx.paths.apps_dir();
    let entries = match std::fs::read_dir(&apps_dir) {
        Ok(entries) => entries,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(Vec::new()),
        Err(error) => {
            return Err(AwareError::from(std::io::Error::new(
                error.kind(),
                format!("{}: {error}", apps_dir.display()),
            ))
            .into());
        }
    };
    let mut out = Vec::new();
    for entry in entries {
        let entry = entry.map_err(AwareError::from)?;
        let Some(source) = crate::manifest::loader::find_app_manifest(&entry.path()) else {
            continue;
        };
        let id =
            app_id(&source).unwrap_or_else(|_| entry.file_name().to_string_lossy().into_owned());
        out.push((id, source));
    }
    out.sort();
    Ok(out)
}

fn state_word(state: State) -> &'static str {
    match state {
        State::UpToDate => "up to date",
        State::AutoUnderPolicy => "can be carried forward under policy",
        State::NeedsPerson => "needs a person",
        State::Blocked => "blocked",
        State::Held => "held",
    }
}

// ── prepare ─────────────────────────────────────────────────────────────────

#[derive(Serialize)]
#[serde(rename_all = "kebab-case")]
struct PrepareData {
    app: String,
    state: State,
    /// Whether a candidate was written. False when nothing moves or the
    /// candidate is blocked (the reasons say why).
    prepared: bool,
    candidate: Option<String>,
    evidence: Option<String>,
    candidate_digest: Option<String>,
    plan_digest: Option<String>,
    evidence_digest: Option<String>,
    /// The approved lock's digest before and after: always equal.
    lock_digest: Option<String>,
    lock_unchanged: bool,
    row: PlanRow,
}

fn prepare_cmd(
    ctx: &Context,
    app: &str,
    to: &[String],
    guard: &crate::agent_store::RefGuard,
) -> Result<Output, Failure> {
    let targets = parse_targets(to).map_err(usage_failure)?;
    let source = app_source(ctx, app)?;
    let dir = crate::fs::containing_dir(&source).to_path_buf();
    // Under the app's promotion lock: a candidate is never written while a
    // promotion of this app reads or consumes the one before it.
    let _promotion = promote::lock_app(&ctx.paths, guard, &dir, &app_id(&source)?)?;
    let requested = (!targets.is_empty()).then_some(&targets);
    if let Some(requested) = requested {
        // An explicit target the app does not dispatch is a misuse, not a state.
        let (parsed, _) = crate::app_lock::read_app_source(&source)?;
        let dispatched = crate::validate::dispatchable_agents(&parsed);
        if let Some(id) = requested
            .keys()
            .find(|id| !dispatched.contains(id.as_str()))
        {
            return Err(usage_failure(AwareError::Validation(format!(
                "[E_MIGRATE_TARGET_UNUSED] app {} does not dispatch agent {id}, so there is nothing to move it to",
                parsed.app
            ))));
        }
    }
    let mut eval = plan::evaluate(&ctx.paths, &source, requested, guard)?;
    let row = &eval.row;
    // An explicit target that cannot be resolved is a misuse too.
    if requested.is_some()
        && let Some(reason) = row
            .reasons
            .iter()
            .find(|r| r.code.starts_with("migrate-target-"))
    {
        return Err(usage_failure(AwareError::Validation(format!(
            "[{}] {}",
            reason.error_code(),
            reason.text
        ))));
    }
    // §14: v1 never prepares a candidate that moves a backing app.
    if row.backing_app_moved {
        let callers = if row.callers.is_empty() {
            plan::find_callers(&ctx.paths, &row.app)
        } else {
            row.callers.clone()
        };
        return Err(Failure {
            code: "E_MIGRATE_BACKING_APP".into(),
            error: Box::new(AwareError::Validation(format!(
                "[E_MIGRATE_BACKING_APP] {}: a workflow that other workflows run as a tool is never carried forward automatically; compile it and its callers again{}",
                row.app,
                if callers.is_empty() {
                    String::new()
                } else {
                    format!(" (callers: {})", callers.join(", "))
                }
            ))),
            details: serde_json::json!({
                "callers": callers,
                "reasons": row.reasons,
            }),
        });
    }
    let lock_path = PathBuf::from(&row.lock);
    let lock_before = eval.base_bytes.as_deref().map(crate::app_lock::lock_digest);
    let writable = matches!(row.state, State::NeedsPerson | State::AutoUnderPolicy)
        || (row.state == State::Held && !row.targets.is_empty() && !has_block(row));
    let mut data = PrepareData {
        app: row.app.clone(),
        state: row.state,
        prepared: false,
        candidate: None,
        evidence: None,
        candidate_digest: None,
        plan_digest: None,
        evidence_digest: None,
        lock_digest: lock_before.clone(),
        lock_unchanged: true,
        row: row.clone(),
    };
    if writable && let Some(candidate) = eval.candidate.take() {
        let mut row = eval.row.clone();
        row.candidate = plan::CandidateRow {
            present: true,
            candidate_digest: Some(candidate.header.candidate_digest.clone()),
            fresh: true,
        };
        let evidence = files::Evidence {
            format: files::EVIDENCE_FORMAT.into(),
            app: row.app.clone(),
            prepared_at: chrono::Utc::now().to_rfc3339(),
            cli_version: env!("CARGO_PKG_VERSION").into(),
            header: candidate.header.clone(),
            row: to_json(&row)?,
        };
        let evidence_bytes = serde_json::to_vec_pretty(&evidence)
            .map_err(|e| AwareError::Internal(format!("serialize evidence: {e}")))?;
        let (candidate_file, evidence_file) =
            files::write_candidate(&dir, &row.app, &candidate.bytes, &evidence_bytes)?;
        data.prepared = true;
        data.candidate = Some(candidate_file.display().to_string());
        data.evidence = Some(evidence_file.display().to_string());
        data.candidate_digest = Some(candidate.header.candidate_digest.clone());
        data.plan_digest = Some(candidate.header.plan_digest.clone());
        data.evidence_digest = Some(crate::app_lock::lock_digest(&evidence_bytes));
        data.row = row;
    }
    let lock_after = std::fs::read(&lock_path)
        .ok()
        .map(|bytes| crate::app_lock::lock_digest(&bytes));
    data.lock_unchanged = lock_after == lock_before;
    if !data.lock_unchanged {
        return Err(AwareError::Internal(format!(
            "{} changed while a candidate was being prepared (another process wrote it); the candidate may be stale — run prepare again",
            lock_path.display()
        ))
        .into());
    }
    let text = if data.prepared {
        format!(
            "\u{2713} candidate for {} written to {}\n  the approved {} is unchanged; a person must approve before it is used\n",
            data.app,
            data.candidate.as_deref().unwrap_or(""),
            lock_path.display()
        )
    } else {
        let mut text = format!(
            "{}: {} — no candidate written\n",
            data.app,
            state_word(data.state)
        );
        for reason in &data.row.reasons {
            text.push_str(&format!("  - {}\n", reason.text));
        }
        text
    };
    Ok(Output {
        data: to_json(&data)?,
        text,
    })
}

/// A held app's candidate is still worth recording unless something blocks it.
fn has_block(row: &PlanRow) -> bool {
    row.reasons.iter().any(|r| {
        matches!(
            r.code.as_str(),
            "needs-source-edit"
                | "agent-unavailable"
                | "no-approval"
                | "approval-invalid"
                | "source-changed"
        ) || r.code.starts_with("migrate-")
            || r.code.starts_with("app-lock-")
            || r.code == "cannot-evaluate"
    })
}

// ── discard / hold / unhold ─────────────────────────────────────────────────

fn discard_cmd(
    ctx: &Context,
    app: &str,
    guard: &crate::agent_store::RefGuard,
) -> Result<Output, Failure> {
    let source = app_source(ctx, app)?;
    let id = app_id(&source)?;
    let dir = crate::fs::containing_dir(&source);
    let _promotion = promote::lock_app(&ctx.paths, guard, dir, &id)?;
    let discarded = files::discard_candidate(dir, &id)?;
    Ok(Output {
        data: serde_json::json!({ "app": id, "discarded": discarded }),
        text: if discarded {
            format!("\u{2713} discarded the candidate for {id}\n")
        } else {
            format!("{id} has no candidate\n")
        },
    })
}

fn hold_cmd(
    ctx: &Context,
    app: &str,
    actor: &str,
    reason: Option<String>,
    guard: &crate::agent_store::RefGuard,
) -> Result<Output, Failure> {
    let actor = require_actor(actor)?;
    let source = app_source(ctx, app)?;
    let id = app_id(&source)?;
    let dir = crate::fs::containing_dir(&source);
    // Under the app's promotion lock: a promotion either finished before the
    // hold or sees it (§15.1 R3).
    let _promotion = promote::lock_app(&ctx.paths, guard, dir, &id)?;
    let record = HoldRecord {
        format: 1,
        app: id.clone(),
        held_by: actor,
        held_at: chrono::Utc::now().to_rfc3339(),
        reason,
    };
    let path = files::write_hold(dir, &record)?;
    Ok(Output {
        data: serde_json::json!({ "app": id, "held": true, "hold": record, "path": path.display().to_string() }),
        text: format!(
            "\u{2713} {id} is on hold; it will not be carried forward until someone runs `aware app migrate unhold {id}`\n"
        ),
    })
}

fn unhold_cmd(ctx: &Context, app: &str, actor: &str) -> Result<Output, Failure> {
    let actor = require_actor(actor)?;
    let source = app_source(ctx, app)?;
    let id = app_id(&source)?;
    let dir = crate::fs::containing_dir(&source);
    let was_held = files::remove_hold(dir, &id).map_err(|error| match error {
        AwareError::Validation(_) => usage_failure(error),
        other => Failure::from(other),
    })?;
    Ok(Output {
        data: serde_json::json!({ "app": id, "held": false, "was-held": was_held, "by": actor }),
        text: if was_held {
            format!("\u{2713} lifted the hold on {id}\n")
        } else {
            format!("{id} was not on hold\n")
        },
    })
}

// ── promote / revert ────────────────────────────────────────────────────────

struct ApproverArgs {
    person: Option<String>,
    approval: Option<PathBuf>,
    policy: Option<String>,
    automatic: bool,
    reason: Option<String>,
    front_door: Option<String>,
}

/// The front door that recorded a person's approval. Required: the CLI cannot
/// tell a person from a process, so a person approval exists only as a front
/// door's record (owner decision 1, pawellisowski/floless.app#1985). In an AI
/// coding session a missing front door is reported as such — an accident
/// guard, not a security boundary (plan §11 R1).
fn required_front_door(front_door: Option<String>, what: &str) -> Result<String, Failure> {
    match front_door.map(|f| f.trim().to_string()) {
        Some(front_door) if !front_door.is_empty() => Ok(front_door),
        _ => {
            let why = match promote::ai_session_marker() {
                Some(marker) => format!(
                    "this looks like an AI coding session ({marker} is set), and an AI never approves on a person's behalf: {what} must come from the front door the person used, which passes --front-door"
                ),
                None => format!("{what} must name the front door that recorded it (--front-door)"),
            };
            Err(Refused::new("E_MIGRATE_NO_FRONT_DOOR", why).into())
        }
    }
}

fn read_record(path: &Path) -> Result<Vec<u8>, Failure> {
    std::fs::read(path).map_err(|error| {
        Refused::new(
            "E_MIGRATE_APPROVAL_UNREADABLE",
            format!(
                "the approval record {} cannot be read: {error}",
                path.display()
            ),
        )
        .into()
    })
}

/// The front door of a promotion no person claims (a policy, an automatic
/// revert): the caller's name when given, else the CLI itself.
fn optional_front_door(front_door: Option<String>) -> String {
    front_door
        .map(|f| f.trim().to_string())
        .filter(|f| !f.is_empty())
        .unwrap_or_else(|| "aware-cli".to_string())
}

fn promote_cmd(
    ctx: &Context,
    guard: &crate::agent_store::RefGuard,
    app: &str,
    candidate: &str,
    args: ApproverArgs,
    kind: SuccessorKind,
) -> Result<Output, Failure> {
    if crate::agent_store::digest_hex(candidate).is_none() {
        return Err(Refused::new(
            "E_MIGRATE_BAD_CANDIDATE",
            format!(
                "--candidate {candidate:?} is not sha256: followed by 64 lowercase hex characters"
            ),
        )
        .into());
    }
    let person = args.person.is_some() || args.approval.is_some();
    let approvers =
        usize::from(person) + usize::from(args.policy.is_some()) + usize::from(args.automatic);
    if approvers == 0 {
        return Err(Refused::new(
            "E_MIGRATE_NO_APPROVER",
            match kind {
                SuccessorKind::CarriedForward => {
                    "nothing is carried forward without an approver: give a person's recorded approval (--person, --approval, --front-door) or a policy (--policy)"
                }
                SuccessorKind::Reverted => {
                    "nothing is reverted without an approver: give a person's recorded approval (--person, --approval, --front-door), or --automatic --reason run-failed:<run-id>"
                }
            },
        )
        .into());
    }
    if approvers > 1 {
        return Err(Refused::new(
            "E_MIGRATE_APPROVER_CONFLICT",
            "give exactly one approver: a person's recorded approval, a policy, or --automatic — not several",
        )
        .into());
    }
    let source = app_source(ctx, app)?;
    let official_index = std::cell::OnceCell::new();
    let official = |pin: &crate::app_lock::CandidatePin| -> Result<(), String> {
        let index = official_index
            .get_or_init(|| {
                crate::registry::fetch::fetch_fresh_official_index().map_err(|e| e.to_string())
            })
            .as_ref()
            .map_err(|e| format!("the official registry could not be fetched: {e}"))?;
        let verdict = crate::install::provenance::assess_against_index(
            &pin.root,
            &pin.agent,
            &pin.version,
            Some(index),
        );
        if verdict.verified {
            Ok(())
        } else {
            Err(verdict.reason)
        }
    };
    let actor;
    let front_door;
    let approver = if person {
        let (Some(who), Some(path)) = (&args.person, &args.approval) else {
            return Err(Refused::new(
                "E_MIGRATE_NO_APPROVER",
                "a person approval needs both --person <who> and --approval <record file>",
            )
            .into());
        };
        front_door = required_front_door(args.front_door, "a person approval")?;
        actor = who.trim().to_string();
        if actor.is_empty() {
            return Err(
                Refused::new("E_MIGRATE_NO_APPROVER", "--person must name who approved").into(),
            );
        }
        Approver::Person {
            actor: &actor,
            front_door: &front_door,
            record: read_record(path)?,
        }
    } else if let Some(id) = &args.policy {
        front_door = optional_front_door(args.front_door);
        Approver::Policy {
            id,
            front_door: &front_door,
            official: &official,
        }
    } else {
        front_door = optional_front_door(args.front_door);
        let Some(reason) = args.reason.as_deref() else {
            return Err(Refused::new(
                "E_MIGRATE_NOT_ELIGIBLE",
                "--automatic needs --reason run-failed:<run-id>, naming the run that failed",
            )
            .into());
        };
        Approver::Automatic {
            reason,
            front_door: &front_door,
        }
    };
    let promoted = promote::promote(&ctx.paths, guard, &source, candidate, approver, kind)?;
    let moves = promoted
        .to
        .iter()
        .map(|(id, pin)| {
            format!(
                "{id} {} -> {}",
                promoted
                    .from
                    .get(id)
                    .map(|p| p.version.as_str())
                    .unwrap_or("(none)"),
                pin.version
            )
        })
        .collect::<Vec<_>>()
        .join(", ");
    let mut text = format!(
        "\u{2713} {} {}: {moves}\n  {}\n",
        promoted.app,
        match kind {
            SuccessorKind::CarriedForward => "carried forward",
            SuccessorKind::Reverted => "reverted",
        },
        promoted.label
    );
    if !promoted.running_instances.is_empty() {
        text.push_str(&format!(
            "  {} run(s) already in progress keep the plan they started with\n",
            promoted.running_instances.len()
        ));
    }
    for warning in &promoted.warnings {
        text.push_str(&format!("  \u{26a0} {}\n", warning.text));
    }
    Ok(Output {
        data: to_json(&promoted)?,
        text,
    })
}

// ── policy record / list / revoke ───────────────────────────────────────────

fn policy_record_cmd(
    ctx: &Context,
    approval: &Path,
    front_door: Option<String>,
) -> Result<Output, Failure> {
    let front_door = required_front_door(front_door, "a policy a person approved")?;
    let record = read_record(approval)?;
    let (loaded, created) =
        policy::record(&ctx.paths, &record, &front_door).map_err(Refused::from)?;
    let label = policy::label(&loaded);
    Ok(Output {
        text: format!(
            "{} {label}\n",
            if created {
                "\u{2713} recorded"
            } else {
                "already recorded:"
            }
        ),
        data: serde_json::json!({
            "policy": loaded.policy.policy,
            "created": created,
            "digest": loaded.digest,
            "path": loaded.path.display().to_string(),
            "rule": loaded.policy.rule,
            "scope": loaded.policy.scope,
            "approved-by": loaded.policy.approved_by,
            "label": label,
        }),
    })
}

fn policy_list_cmd(ctx: &Context) -> Result<Output, Failure> {
    let entries = policy::list(&ctx.paths)?;
    let mut text = String::new();
    for entry in &entries {
        text.push_str(&format!(
            "{} [{}] {}\n",
            entry.policy, entry.state, entry.label
        ));
    }
    if entries.is_empty() {
        text.push_str("no carry-forward policies\n");
    }
    Ok(Output {
        data: serde_json::json!({ "policies": entries }),
        text,
    })
}

fn policy_revoke_cmd(
    ctx: &Context,
    id: &str,
    actor: &str,
    front_door: Option<String>,
    reason: Option<String>,
) -> Result<Output, Failure> {
    let actor = require_actor(actor)?;
    let front_door = optional_front_door(front_door);
    let (revocation, revoked_now) =
        policy::revoke(&ctx.paths, id, &actor, &front_door, reason).map_err(Refused::from)?;
    Ok(Output {
        text: if revoked_now {
            format!("\u{2713} policy {id} revoked; it carries nothing forward from now on\n")
        } else {
            format!(
                "policy {id} was already revoked by {}\n",
                revocation.revoked_by
            )
        },
        data: serde_json::json!({
            "policy": id,
            "revoked": true,
            "revoked-now": revoked_now,
            "revocation": revocation,
        }),
    })
}

fn require_actor(actor: &str) -> Result<String, Failure> {
    let actor = actor.trim();
    if actor.is_empty() {
        return Err(usage_failure(AwareError::Validation(
            "[E_MIGRATE_NO_ACTOR] --actor must name who is acting".into(),
        )));
    }
    Ok(actor.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn targets_parse_as_digest_or_version_and_refuse_the_rest() {
        let hex = "a".repeat(64);
        let parsed = parse_targets(&[format!("tekla@sha256:{hex}"), "gw@1.2.3".into()]).unwrap();
        assert_eq!(parsed["tekla"], PinTarget::Digest(format!("sha256:{hex}")));
        assert_eq!(parsed["gw"], PinTarget::Version("1.2.3".into()));
        for bad in [
            "tekla",
            "@1.0.0",
            "tekla@",
            "tekla@sha256:ABC",
            "../x@1.0.0",
        ] {
            let error = parse_targets(&[bad.to_string()]).unwrap_err().to_string();
            assert!(error.contains("E_MIGRATE_BAD_TARGET"), "{bad}: {error}");
        }
        let twice = parse_targets(&["a@1.0.0".into(), "a@2.0.0".into()]).unwrap_err();
        assert!(twice.to_string().contains("named twice"));
    }
}
