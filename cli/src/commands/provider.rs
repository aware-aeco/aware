//! `aware provider ...` — trust and select closed model-provider packages.

use std::path::PathBuf;
use std::time::Instant;

use clap::Subcommand;

use crate::context::Context;
use crate::envelope;
use crate::error::AwareError;

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
    List {
        #[arg(long)]
        format: Option<String>,
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
            } else if listing.packages.is_empty() {
                println!("(no provider packages enrolled)");
            } else {
                println!("FORMAT  PACKAGE  VERSION  MANIFEST  SELECTED");
                for package in listing.packages {
                    println!(
                        "{}  {}  {}  {}  {}",
                        package.format_id,
                        package.package_id,
                        package.package_version,
                        package.manifest_sha256,
                        if package.selected { "yes" } else { "no" }
                    );
                }
            }
        }
    }
    Ok(())
}
