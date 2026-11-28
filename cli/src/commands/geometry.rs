use std::{
    fs::File,
    io::{self, Read, Write},
    path::{Path, PathBuf},
    time::Instant,
};

use clap::Subcommand;
use serde_json::json;

use crate::{
    context::Context,
    envelope::{self, Envelope},
    error::AwareError,
    geometry::{
        execute, runtime,
        wire::{Cut, LIMIT, Rejection, Request, WarningCode},
    },
};

const EXEC: &str = "geometry exec";

#[derive(Debug, Subcommand)]
pub enum GeometryCommand {
    #[command(hide = true)]
    ArtifactIdentity,
    Exec {
        request: String,
    },
    Install {
        #[arg(long, required = true)]
        occt: bool,
        #[arg(long)]
        python: PathBuf,
        #[arg(long)]
        wheelhouse: Option<PathBuf>,
    },
}

pub async fn dispatch(action: GeometryCommand, ctx: &Context) -> Result<(), AwareError> {
    if !ctx.json {
        return Err(AwareError::Validation("geometry requires --json".into()));
    }
    match action {
        GeometryCommand::ArtifactIdentity => {
            use sha2::{Digest, Sha256};
            envelope::print_ok(
                "geometry artifact-identity",
                json!({
                    "abi": "windows-x64",
                    "fault_enabled": cfg!(feature = "geometry-test-faults"),
                    "worker_sha256": format!("{:x}", Sha256::digest(execute::worker_source().as_bytes())),
                    "wheel_lock_sha256": format!("{:x}", Sha256::digest(include_bytes!(
                        "../../../geometry-worker/win-amd64-cp312.requirements.lock"
                    ))),
                }),
                Instant::now(),
            )?;
            Ok(())
        }
        GeometryCommand::Exec { request } => {
            let started = Instant::now();
            let (request, bytes) = match read_request(&request) {
                Ok(value) => value,
                Err(error) => {
                    envelope::exit_with_error(EXEC, "E_GEOMETRY_REQUEST", &error, started)
                }
            };
            let envelope = run(&ctx.paths.aware_home, &request, &bytes, started).await?;
            if envelope.len() > LIMIT {
                let error =
                    AwareError::Validation("operand ids exceed the 32 MiB response budget".into());
                envelope::exit_with_error(EXEC, "E_GEOMETRY_REQUEST", &error, started);
            }
            io::stdout().lock().write_all(&envelope)?;
            Ok(())
        }
        GeometryCommand::Install {
            python, wheelhouse, ..
        } => {
            let started = Instant::now();
            let installation =
                match runtime::install(&ctx.paths.aware_home, &python, wheelhouse.as_deref()) {
                    Ok(value) => value,
                    Err(error) => envelope::exit_with_error(
                        "geometry install",
                        "E_GEOMETRY_INSTALL",
                        &error,
                        started,
                    ),
                };
            envelope::print_ok("geometry install", installation, started)?;
            Ok(())
        }
    }
}

fn read_request(source: &str) -> Result<(Request, Vec<u8>), AwareError> {
    let mut bytes = Vec::new();
    let limit = LIMIT as u64 + 1;
    if source == "-" {
        io::stdin().lock().take(limit).read_to_end(&mut bytes)?;
    } else if let Some(path) = source.strip_prefix('@') {
        File::open(path)?.take(limit).read_to_end(&mut bytes)?;
    } else {
        return Err(AwareError::Validation("expected @request.json or -".into()));
    }
    if bytes.len() > LIMIT {
        return Err(AwareError::Validation("request exceeds 32 MiB".into()));
    }
    let utf8 = std::str::from_utf8(&bytes)
        .map_err(|_| AwareError::Validation("request must be UTF-8".into()))?;
    let request: Request = serde_json::from_str(utf8)
        .map_err(|error| AwareError::Validation(error.to_string().chars().take(512).collect()))?;
    request
        .validate()
        .map_err(|message| AwareError::Validation(message.into()))?;
    Ok((request, bytes))
}

async fn run(
    home: &Path,
    request: &Request,
    bytes: &[u8],
    started: Instant,
) -> Result<Vec<u8>, AwareError> {
    use WarningCode::*;
    let exact_unavailable = if let Some(runtime) = runtime::installed_runtime(home) {
        if request.a.brep_b64.is_none() || request.b.brep_b64.is_none() {
            BrepInputMissing
        } else {
            match execute::exact(&runtime.python, bytes).await {
                Ok(result) => {
                    let output = encode(
                        request,
                        None,
                        "validated",
                        "validated_brep",
                        None,
                        Some(&result),
                        started,
                    )?;
                    if output.len() <= LIMIT {
                        return Ok(output);
                    }
                    #[cfg(feature = "geometry-test-faults")]
                    execute::test_trace(
                        &json!({"event": "wrapped_envelope", "route": "exact", "wrapped_bytes": output.len()}),
                    );
                    return mesh_result(request, bytes, BudgetExceeded, "failed", started).await;
                }
                Err(failure) if failure.terminal => {
                    return encode(
                        request,
                        None,
                        "failed",
                        "unavailable",
                        Some(failure.rejection),
                        None,
                        started,
                    );
                }
                Err(failure) => {
                    return mesh_result(request, bytes, failure.rejection.0, "failed", started)
                        .await;
                }
            }
        }
    } else {
        OcctRuntimeMissing
    };
    mesh_result(request, bytes, exact_unavailable, "unavailable", started).await
}

async fn mesh_result(
    request: &Request,
    bytes: &[u8],
    exact_warning: WarningCode,
    exact_status: &str,
    started: Instant,
) -> Result<Vec<u8>, AwareError> {
    use WarningCode::*;
    let warning = match execute::fallback(bytes).await {
        Ok(result) => {
            let output = encode(
                request,
                Some(exact_warning),
                exact_status,
                "mesh_fallback",
                Some((ChordHeightUnverified, None)),
                Some(&result),
                started,
            )?;
            if output.len() <= LIMIT {
                return Ok(output);
            }
            #[cfg(feature = "geometry-test-faults")]
            execute::test_trace(
                &json!({"event": "wrapped_envelope", "route": "mesh", "wrapped_bytes": output.len()}),
            );
            (BudgetExceeded, None)
        }
        Err(warning) => warning,
    };
    encode(
        request,
        Some(exact_warning),
        exact_status,
        "unavailable",
        Some(warning),
        None,
        started,
    )
}

fn encode(
    request: &Request,
    exact_warning: Option<WarningCode>,
    exact_status: &str,
    status: &str,
    warning: Option<Rejection>,
    result: Option<&Cut>,
    started: Instant,
) -> Result<Vec<u8>, AwareError> {
    let warnings: Vec<_> = exact_warning
        .map(|code| (code, None))
        .into_iter()
        .chain(warning)
        .map(|(code, relation)| {
            let message = relation.map_or_else(
                || code.message().to_string(),
                |relation| format!("{} Relation {}.", code.message(), relation.name()),
            );
            json!({"code": code, "message": message})
        })
        .collect();
    let data = json!({
        "schema": "aware.geometry.exec.response/v1", "op": "boolean", "kind": "difference",
        "frame": request.frame, "status": status, "exact_status": exact_status,
        "warnings": warnings,
        "result": result, "reason_code": warning.filter(|_| result.is_none()).map(|(code, _)| code),
        "operand_ids": {"a": request.a.id, "b": request.b.id}
    });
    let mut bytes = serde_json::to_vec(&Envelope {
        ok: true,
        data: Some(data),
        error: None,
        meta: envelope::meta_for(EXEC, started),
    })?;
    bytes.push(b'\n');
    Ok(bytes)
}
