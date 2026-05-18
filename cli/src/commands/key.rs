//! `aware key ...` — manage ed25519 signing keys for Stamped Receipts (v0.27).
//!
//! Keys live at `~/.aware/keys/<operator-id>.{pub,sec}`. The `.sec`
//! (private) file is chmod 0600 on Unix; on Windows AWARE relies on the
//! existing user-profile ACL.

use clap::Subcommand;

use crate::context::Context;
use crate::error::AwareError;

#[derive(Subcommand, Debug)]
pub enum KeyCommand {
    /// Generate a fresh ed25519 keypair at `~/.aware/keys/<operator-id>.{pub,sec}`.
    Generate { operator_id: String },
    /// List installed keypairs.
    List,
    /// Print the public key for an operator (for sharing with verifiers).
    Show { operator_id: String },
}

pub fn dispatch(cmd: KeyCommand, ctx: &Context) -> Result<(), AwareError> {
    match cmd {
        KeyCommand::Generate { operator_id } => generate(ctx, &operator_id),
        KeyCommand::List => list(ctx),
        KeyCommand::Show { operator_id } => show(ctx, &operator_id),
    }
}

fn keys_dir(ctx: &Context) -> std::path::PathBuf {
    ctx.paths.aware_home.join("keys")
}

fn generate(ctx: &Context, operator_id: &str) -> Result<(), AwareError> {
    let kp = crate::receipt::generate_keypair(&keys_dir(ctx), operator_id)?;
    println!("\u{2713} generated keypair for {operator_id}");
    println!("  secret: {}", kp.sec_path.display());
    println!("  public: {}", kp.pub_path.display());
    println!();
    println!("Share the .pub file with verifiers (insurers, building-control,");
    println!("peer reviewers). NEVER share the .sec file.");
    Ok(())
}

fn list(ctx: &Context) -> Result<(), AwareError> {
    let dir = keys_dir(ctx);
    if !dir.is_dir() {
        println!("(no keys installed)");
        println!();
        println!("Generate one with:  aware key generate <operator-id>");
        return Ok(());
    }
    let mut operators: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
    if let Ok(entries) = std::fs::read_dir(&dir) {
        for e in entries.flatten() {
            let p = e.path();
            if let (Some(stem), Some(ext)) = (
                p.file_stem().and_then(|s| s.to_str()),
                p.extension().and_then(|s| s.to_str()),
            ) {
                if ext == "pub" || ext == "sec" {
                    operators.insert(stem.to_string());
                }
            }
        }
    }
    if operators.is_empty() {
        println!("(no keys installed)");
        return Ok(());
    }
    println!("OPERATOR-ID                              FILES");
    for op in &operators {
        let pub_p = dir.join(format!("{op}.pub"));
        let sec_p = dir.join(format!("{op}.sec"));
        let mut parts = Vec::new();
        if pub_p.is_file() {
            parts.push("pub");
        }
        if sec_p.is_file() {
            parts.push("sec");
        }
        println!("{op:<40} {}", parts.join(" + "));
    }
    Ok(())
}

fn show(ctx: &Context, operator_id: &str) -> Result<(), AwareError> {
    let pub_path = keys_dir(ctx).join(format!("{operator_id}.pub"));
    let text = std::fs::read_to_string(&pub_path)
        .map_err(|e| AwareError::NotFound(format!("{}: {e}", pub_path.display())))?;
    println!("{text}");
    Ok(())
}
