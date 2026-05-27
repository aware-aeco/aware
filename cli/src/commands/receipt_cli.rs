//! `aware receipt ...` — inspect / verify signed JSONL receipts (v0.27).
//!
//! A receipt is a JSONL artefact emitted by panel-gated runs. Adjacent
//! to each receipt sits a `.sig` ed25519 signature sidecar. This module
//! provides verification + inspection.

use std::path::PathBuf;

use clap::{Args, Subcommand};

use crate::context::Context;
use crate::error::AwareError;

#[derive(Subcommand, Debug)]
pub enum ReceiptCommand {
    /// Verify a receipt against its `.sig` sidecar.
    Verify(VerifyArgs),
    /// Print a receipt's panel transcripts + signature metadata.
    Show { path: PathBuf },
    /// Sign an existing receipt (typically called by the runtime, but
    /// exposed here so engineers can sign external artefacts too).
    Sign(SignArgs),
}

#[derive(Args, Debug)]
pub struct VerifyArgs {
    /// Path to the receipt (`.jsonl` or any signed artefact).
    pub receipt: PathBuf,
    /// Path to the `.sig` sidecar. Defaults to `<receipt>.sig`.
    #[arg(long)]
    pub sig: Option<PathBuf>,
}

#[derive(Args, Debug)]
pub struct SignArgs {
    /// Path to the receipt to sign.
    pub receipt: PathBuf,
    /// Operator id (resolves to `~/.aware/keys/<operator-id>.sec`).
    #[arg(long)]
    pub operator: String,
}

pub fn dispatch(cmd: ReceiptCommand, ctx: &Context) -> Result<(), AwareError> {
    match cmd {
        ReceiptCommand::Verify(args) => verify(ctx, &args),
        ReceiptCommand::Show { path } => show(ctx, &path),
        ReceiptCommand::Sign(args) => sign(ctx, &args),
    }
}

fn default_sig_path(receipt: &std::path::Path) -> PathBuf {
    let mut s = receipt.as_os_str().to_owned();
    s.push(".sig");
    PathBuf::from(s)
}

fn verify(_ctx: &Context, args: &VerifyArgs) -> Result<(), AwareError> {
    let sig_path = args
        .sig
        .clone()
        .unwrap_or_else(|| default_sig_path(&args.receipt));
    crate::receipt::verify_receipt(&args.receipt, &sig_path)?;
    println!("\u{2713} signature verifies");
    println!("  receipt: {}", args.receipt.display());
    println!("  sig:     {}", sig_path.display());
    Ok(())
}

fn show(_ctx: &Context, path: &std::path::Path) -> Result<(), AwareError> {
    let text = std::fs::read_to_string(path)
        .map_err(|e| AwareError::NotFound(format!("{}: {e}", path.display())))?;
    println!("{text}");
    let sig_path = default_sig_path(path);
    if sig_path.is_file() {
        println!("--- {} ---", sig_path.display());
        if let Ok(s) = std::fs::read_to_string(&sig_path) {
            println!("{s}");
        }
    } else {
        println!();
        println!("(no .sig sidecar at {})", sig_path.display());
    }
    Ok(())
}

fn sign(ctx: &Context, args: &SignArgs) -> Result<(), AwareError> {
    let sec_path = ctx
        .paths
        .aware_home
        .join("keys")
        .join(format!("{}.sec", args.operator));
    let signing = crate::receipt::load_signing_key(&sec_path)?;
    let sig_path = crate::receipt::sign_receipt(&args.receipt, &signing)?;
    println!("\u{2713} signed");
    println!("  receipt: {}", args.receipt.display());
    println!("  sig:     {}", sig_path.display());
    Ok(())
}
