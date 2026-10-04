//! `aware provider ...` — trust and select closed model-provider packages.

use std::path::PathBuf;
use std::time::Instant;

use clap::Subcommand;

use crate::context::Context;
use crate::envelope;
use crate::error::AwareError;
use crate::provider_store::Verification;

#[derive(Subcommand, Debug)]
pub enum ProviderCommand {
    /// Trust a publisher's Ed25519 public key for later package enrollment.
    TrustPublisher {
        public_key_file: PathBuf,
        #[arg(long)]
        publisher_id: String,
    },
    /// Verify and enroll an immutable signed package directory.
    Enroll { package_directory: PathBuf },
    /// Select an enrolled package for an opaque model format.
    Select {
        format_id: String,
        manifest_sha256: String,
    },
    /// Admit an operator-controlled dependency policy for one enrolled capability.
    AdmitPolicy {
        manifest_sha256: String,
        capability_id: String,
        policy_file: PathBuf,
    },
    /// List enrolled packages, optionally restricted to an opaque format.
    ///
    /// Each format's selected package is re-verified completely (every file re-hashed); every other
    /// enrollment is re-checked without re-hashing its files and is reported with
    /// `verification: "inventory"`. `select` re-verifies a package completely before using it.
    List {
        #[arg(long)]
        format: Option<String>,
    },
    /// Retire one enrollment: remove AWARE's record of it and the dependency policies admitted
    /// for it. Refuses the active selection and its rollback history. Never touches the package
    /// directory.
    Unenroll { manifest_sha256: String },
    /// Retire every superseded enrollment of one format: each one that is neither selected, nor
    /// in the selection's rollback history, nor enrolled after the selection was made. Never
    /// touches a package directory.
    Prune {
        #[arg(long)]
        format: String,
        /// Report what would be retired without removing anything.
        #[arg(long)]
        dry_run: bool,
    },
}

pub fn dispatch(command: ProviderCommand, context: &Context) -> Result<(), AwareError> {
    let store = crate::provider_store::ProviderStore::new(context.paths.providers_dir());
    let started = Instant::now();
    match command {
        ProviderCommand::TrustPublisher {
            public_key_file,
            publisher_id,
        } => {
            let publisher = store.trust_publisher(&public_key_file, &publisher_id)?;
            if context.json {
                envelope::print_ok("provider trust-publisher", publisher, started)?;
            } else {
                println!(
                    "trusted provider publisher {} ({})",
                    publisher.publisher_id, publisher.key_fingerprint_sha256
                );
            }
        }
        ProviderCommand::Enroll { package_directory } => {
            let package = store.enroll(&package_directory)?;
            if context.json {
                envelope::print_ok("provider enroll", package.public_view(), started)?;
            } else {
                println!(
                    "enrolled provider package {} {} ({})",
                    package.manifest.package_id,
                    package.manifest.package_version,
                    package.manifest_sha256
                );
            }
        }
        ProviderCommand::Select {
            format_id,
            manifest_sha256,
        } => {
            let selection = store.select(&format_id, &manifest_sha256)?;
            if context.json {
                envelope::print_ok("provider select", selection, started)?;
            } else {
                println!(
                    "selected provider package {} for {} (generation {})",
                    selection.active_manifest_sha256, selection.format_id, selection.generation
                );
            }
        }
        ProviderCommand::AdmitPolicy {
            manifest_sha256,
            capability_id,
            policy_file,
        } => {
            let admitted =
                store.admit_dependency_policy(&manifest_sha256, &capability_id, &policy_file)?;
            if context.json {
                envelope::print_ok("provider admit-policy", admitted, started)?;
            } else {
                println!(
                    "admitted dependency policy {} ({})",
                    admitted.policy.policy_id, admitted.sha256
                );
            }
        }
        ProviderCommand::List { format } => {
            let listing = store.list(format.as_deref())?;
            if context.json {
                envelope::print_ok("provider list", listing, started)?;
            } else if listing.packages.is_empty() && listing.unavailable.is_empty() {
                println!("(no provider packages enrolled)");
            } else {
                println!("FORMAT  PACKAGE  VERSION  MANIFEST  SELECTED  VERIFIED");
                for package in listing.packages {
                    println!(
                        "{}  {}  {}  {}  {}  {}",
                        package.format_id,
                        package.package_id,
                        package.package_version,
                        package.manifest_sha256,
                        if package.selected { "yes" } else { "no" },
                        match package.verification {
                            Verification::Complete => "complete",
                            Verification::Inventory => "inventory",
                        }
                    );
                }
                for package in listing.unavailable {
                    println!(
                        "{}  {}  {}  {}  {}unavailable ({})",
                        package.format_id,
                        package.package_id,
                        package.package_version,
                        package.manifest_sha256,
                        if package.selected { "yes, " } else { "" },
                        package.reason
                    );
                }
            }
        }
        ProviderCommand::Unenroll { manifest_sha256 } => {
            let retired = store.unenroll(&manifest_sha256)?;
            if context.json {
                envelope::print_ok("provider unenroll", retired, started)?;
            } else {
                println!(
                    "unenrolled provider package {} {} ({}); removed {} dependency polic{}",
                    retired.package_id,
                    retired.package_version,
                    retired.manifest_sha256,
                    retired.dependency_policies,
                    if retired.dependency_policies == 1 {
                        "y"
                    } else {
                        "ies"
                    }
                );
            }
        }
        ProviderCommand::Prune { format, dry_run } => {
            let pruned = store.prune(&format, dry_run)?;
            if context.json {
                envelope::print_ok("provider prune", pruned, started)?;
            } else {
                let verb = if pruned.dry_run {
                    "would retire"
                } else {
                    "retired"
                };
                println!(
                    "{verb} {} superseded provider package(s) for {}; kept {}",
                    pruned.retired.len(),
                    pruned.format_id,
                    pruned.kept.len()
                );
                for package in &pruned.retired {
                    println!(
                        "  {verb}  {}  {}  {}",
                        package.package_id, package.package_version, package.manifest_sha256
                    );
                }
                for package in &pruned.kept {
                    println!(
                        "  kept ({})  {}  {}  {}",
                        package.reason,
                        package.package_id,
                        package.package_version,
                        package.manifest_sha256
                    );
                }
            }
        }
    }
    Ok(())
}
