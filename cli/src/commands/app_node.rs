//! `aware app rename-node` and the `--json` forms of `app validate` / `app compile`
//! (aware#701).
//!
//! `E_APP_AMBIGUOUS_NODE_ID` refuses to write a lock but must not be a dead end: the
//! JSON forms always print ONE envelope on stdout (never a bare stderr line), carrying
//! the issue's structured `details`, and `rename-node` is the deterministic repair.

use std::time::Instant;

use crate::context::Context;
use crate::envelope;
use crate::error::AwareError;

/// Print an `ok: false` envelope with structured `details`, then exit.
pub(super) fn exit_json_error(
    command: &str,
    code: &str,
    message: String,
    details: serde_json::Value,
    exit_code: i32,
    started: Instant,
) -> ! {
    let env = envelope::Envelope::<()> {
        ok: false,
        data: None,
        error: Some(envelope::EnvelopeError {
            code: code.into(),
            message: message.clone(),
            details,
        }),
        meta: envelope::meta_for(command, started),
    };
    if let Ok(json) = serde_json::to_string(&env) {
        println!("{json}");
    }
    eprintln!("error: {message}");
    use std::io::Write;
    let _ = std::io::stdout().flush();
    std::process::exit(exit_code);
}

/// `aware app validate --json`.
pub(super) fn validate_json(ctx: &Context, path: &std::path::Path) -> Result<(), AwareError> {
    let started = Instant::now();
    let command = "app validate";
    let fail = |e: &AwareError, code: &str| -> ! {
        exit_json_error(
            command,
            code,
            e.to_string(),
            serde_json::Value::Null,
            e.exit_code(),
            started,
        )
    };
    let manifest_path = super::app::resolve_validate_target(path)
        .unwrap_or_else(|e| fail(&e, "E_APP_VALIDATE_TARGET"));
    let app = crate::manifest::loader::load_app(&manifest_path)
        .unwrap_or_else(|e| fail(&e, "E_APP_PARSE"));
    let mut issues = crate::validate::validate_app(&app);
    if let Ok(discovery) = crate::manifest::loader::discover_agents(&ctx.paths) {
        let agents = discovery
            .require(crate::validate::referenced_agents(&app))
            .unwrap_or_else(|e| fail(&e, crate::manifest::loader::agent_manifest_error_code(&e)));
        issues.extend(crate::validate::validate_app_safety(&app, &agents));
        issues.extend(crate::validate::validate_app_agents(&app, &agents));
    }
    let list: Vec<serde_json::Value> = issues.iter().map(|i| i.to_json()).collect();
    if crate::validate::has_errors(&issues) {
        exit_json_error(
            command,
            "E_APP_VALIDATION",
            "app failed validation".into(),
            serde_json::json!({ "issues": list }),
            3,
            started,
        );
    }
    if issues.is_empty()
        && let Err(e) = crate::app_lock::validate_compiles(&manifest_path, &ctx.paths)
    {
        fail(&e, "E_APP_VALIDATION");
    }
    envelope::print_ok(
        command,
        serde_json::json!({
            "valid": true,
            "path": manifest_path.display().to_string(),
            "issues": list,
        }),
        started,
    )?;
    Ok(())
}

/// `aware app compile --json`: one envelope, success or failure. A validation
/// refusal carries every error issue (with `details`) under `error.details.issues`.
pub(super) fn compile_json(
    ctx: &Context,
    path: &std::path::Path,
    front_door: Option<&str>,
) -> Result<(), AwareError> {
    let started = Instant::now();
    let command = "app compile";
    let result = (|| {
        if front_door.is_some_and(|door| door.trim().is_empty()) {
            return Err(AwareError::Validation(
                "--front-door names who asked for the compile; it cannot be empty".into(),
            ));
        }
        let source = crate::app_lock::find_app_source(path).ok_or_else(|| {
            AwareError::Validation(format!(
                "no app source file (.flo / .app / .flow / .aware) at {}",
                path.display()
            ))
        })?;
        let guard = crate::agent_store::open(&ctx.paths)?;
        let (lock_path, _) =
            crate::app_lock::compile_to_disk_for(&source, &ctx.paths, front_door, &guard)?;
        drop(guard);
        Ok((source, lock_path))
    })();
    match result {
        Ok((source, lock)) => {
            envelope::print_ok(
                command,
                serde_json::json!({
                    "source": source.display().to_string(),
                    "lock": lock.display().to_string(),
                }),
                started,
            )?;
            Ok(())
        }
        Err(e) => {
            let message = e.to_string();
            // `app failed validation: [CODE] ...` -> keep the code; enrich with the full issue
            // list (re-validating the same file) so a front end gets the structured details.
            let code = message
                .split_once("app failed validation: [")
                .and_then(|(_, rest)| rest.split_once(']'))
                .map(|(c, _)| c.to_string());
            let mut details = serde_json::Value::Null;
            if code.is_some()
                && let Some(src) = crate::app_lock::find_app_source(path)
                && let Ok(app) = crate::manifest::loader::load_app(&src)
            {
                let mut list: Vec<serde_json::Value> = crate::validate::validate_app(&app)
                    .iter()
                    .filter(|i| i.severity == crate::validate::Severity::Error)
                    .map(|i| i.to_json())
                    .collect();
                // Agent-dependent refusals (command unavailable, unsatisfied pin) come from
                // checks that need the installed agents; carry the refusal itself so the
                // list always contains the reported failure.
                if let Some(c) = &code
                    && !list.iter().any(|i| i["code"] == c.as_str())
                {
                    list.push(serde_json::json!({
                        "severity": "error", "code": c, "message": message,
                    }));
                }
                details = serde_json::json!({ "issues": list });
            }
            exit_json_error(
                command,
                code.as_deref().unwrap_or("E_APP_COMPILE"),
                message,
                details,
                e.exit_code(),
                started,
            )
        }
    }
}

/// `aware app rename-node` (aware#701). Edits the SOURCE only; never reads, writes or
/// refreshes a `.lock`.
pub(super) fn rename_node_cmd(
    ctx: &Context,
    app_or_path: &str,
    node_path: &str,
    new_id: &str,
    flags: RenameFlags,
) -> Result<(), AwareError> {
    let RenameFlags {
        dry_run,
        reformat,
        accept_unverified,
    } = flags;
    let started = Instant::now();
    let command = "app rename-node";
    let fail = |code: &str, message: String, details: serde_json::Value| -> AwareError {
        if ctx.json {
            exit_json_error(command, code, message, details, 3, started);
        }
        AwareError::Validation(format!("[{code}] {message}"))
    };

    let as_path = std::path::Path::new(app_or_path);
    let resolved = if as_path.exists() {
        super::app::resolve_validate_target(as_path)
    } else {
        crate::manifest::loader::resolve_app_dir(&ctx.paths, app_or_path).and_then(|dir| {
            crate::manifest::loader::find_app_manifest(&dir).ok_or_else(|| {
                AwareError::Validation(format!("app {app_or_path} has no .flo/.app file"))
            })
        })
    };
    let source_path = match resolved {
        Ok(p) => p,
        Err(e) if ctx.json => exit_json_error(
            command,
            "E_APP_RENAME_NODE_TARGET",
            e.to_string(),
            serde_json::Value::Null,
            e.exit_code(),
            started,
        ),
        Err(e) => return Err(e),
    };
    let source = match std::fs::read_to_string(&source_path) {
        Ok(t) => t,
        Err(e) => {
            return Err(fail(
                "E_APP_RENAME_NODE_TARGET",
                format!("read {}: {e}", source_path.display()),
                serde_json::Value::Null,
            ));
        }
    };

    let plan = match crate::node_rename::plan(&source, node_path, new_id) {
        Ok(p) => p,
        Err(e) => return Err(fail(e.code, e.message, e.details)),
    };

    let app_id = serde_yaml::from_str::<crate::manifest::app::App>(&source)
        .map(|a| a.app)
        .unwrap_or_default();
    let lock_path = source_path
        .parent()
        .map(|d| d.join(format!("{app_id}.lock")));
    let lock_present = lock_path.as_ref().is_some_and(|p| p.exists());

    let text_ok = plan.text_source.is_some();
    let shown = if text_ok {
        &plan.edits
    } else {
        &plan.value_edits
    };
    let edits: Vec<serde_json::Value> = shown.iter().map(|e| e.to_json()).collect();
    let unverified = plan.unverified.clone();

    if !dry_run {
        if !unverified.is_empty() && !accept_unverified {
            return Err(fail(
                "E_APP_RENAME_NODE_NEEDS_REVIEW",
                format!(
                    "nothing was changed: {} mention(s) could not be proven to mean `{}`: {}. Review them (--dry-run lists every edit) and re-run with --accept-unverified to rename anyway",
                    unverified.len(),
                    plan.node_path,
                    unverified.join("; ")
                ),
                serde_json::json!({ "unverified": unverified, "edits": edits }),
            ));
        }
        if !text_ok && !reformat {
            return Err(fail(
                "E_APP_RENAME_NODE_NEEDS_REFORMAT",
                format!(
                    "nothing was changed: the file's layout cannot be edited byte-for-byte ({}). Re-run with --reformat to rewrite it from the parsed result (comments and layout are lost), or edit the id by hand",
                    plan.text_edit_note.clone().unwrap_or_default()
                ),
                serde_json::json!({ "edits": edits, "reformat-available": true }),
            ));
        }
        let bytes = plan
            .text_source
            .clone()
            .unwrap_or_else(|| plan.reformatted.clone());
        match crate::app_lock::replace_atomically(&source_path, bytes.as_bytes()) {
            Ok(crate::fs::Replaced::Durable) => {}
            Ok(crate::fs::Replaced::NotDurable(e)) => eprintln!(
                "\u{26a0} {} was updated and is in effect, but making the change durable failed ({e})",
                source_path.display()
            ),
            Err(e) => {
                return Err(AwareError::Internal(format!(
                    "write {}: {e}",
                    source_path.display()
                )));
            }
        }
    }

    let next = format!("aware app compile {}", source_path.display());
    let lock_note = if lock_present {
        "a compiled lock exists next to the source and was not touched; it will no longer match the changed source, so run compile for a fresh lock, which must be approved again"
    } else {
        "no compiled lock exists next to the source; compile one with the command in `next`"
    };
    if ctx.json {
        envelope::print_ok(
            command,
            serde_json::json!({
                "app": app_id,
                "source": source_path.display().to_string(),
                "dry-run": dry_run,
                "applied": !dry_run,
                "node-path": plan.node_path,
                "old-id": plan.old_id,
                "new-id": plan.new_id,
                "edit-mode": if text_ok { "text" } else { "reformat" },
                "text-edit-note": plan.text_edit_note,
                "edits": edits,
                "unverified": unverified,
                "lock": { "path": lock_path.map(|p| p.display().to_string()), "present": lock_present, "touched": false },
                "next": next,
                "note": lock_note,
            }),
            started,
        )?;
        return Ok(());
    }
    let verb = if dry_run { "would rename" } else { "renamed" };
    println!(
        "\u{2713} {verb} `{}`: {} \u{2192} {}",
        plan.node_path, plan.old_id, plan.new_id
    );
    for e in shown {
        if e.line > 0 {
            println!(
                "  line {}: {}\n    \u{2192} {}",
                e.line,
                e.before.trim(),
                e.after.trim()
            );
        } else {
            println!(
                "  {} [{}]: {} \u{2192} {}",
                e.location,
                e.kind,
                e.before.trim(),
                e.after.trim()
            );
        }
    }
    if !text_ok {
        println!(
            "  (the file would be rewritten from the parsed result - comments and layout are lost: {})",
            plan.text_edit_note.clone().unwrap_or_default()
        );
    }
    for u in &unverified {
        println!("  \u{26a0} unverified: {u}");
    }
    println!("  {lock_note}");
    println!("  next: {next}");
    Ok(())
}

pub(super) struct RenameFlags {
    pub dry_run: bool,
    pub reformat: bool,
    pub accept_unverified: bool,
}
