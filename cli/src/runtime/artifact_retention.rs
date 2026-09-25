//! Exact-run artifact retirement for report reservations. A lease is published
//! before the reservation marker, so an interrupted in-process writer can be
//! distinguished from a still-running one without trusting a PID.

use std::fs::File;
use std::io::Write;
use std::path::Path;

use cap_fs_ext::DirExt;
#[cfg(windows)]
use cap_fs_ext::OsMetadataExt;
use cap_std::ambient_authority;
use cap_std::fs::{Dir, OpenOptions};
use fs2::FileExt;
use serde::{Deserialize, Serialize};

use crate::error::AwareError;
use crate::manifest::app::{App, Node};
use crate::paths::Paths;
use crate::runtime::invoker::{TransportKind, effective_transport};
use crate::runtime::provenance::validate_artifact_component;
use crate::runtime::report_reservation::{LeaseEvidence, inspect};

const LEASE_SCHEMA: &str = "aware.report-artifact-lease/v1";

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum WriterClass {
    InProcess,
    ExternalPossible,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct LeaseRecord {
    schema_version: String,
    reservation_id: String,
    app: String,
    instance: String,
    run_id: String,
    writer_class: WriterClass,
}

pub struct RunLease {
    file: File,
    pub evidence: LeaseEvidence,
}

impl RunLease {
    pub fn in_process(&self) -> bool {
        self.evidence.writer_class == WriterClass::InProcess
    }
}

impl Drop for RunLease {
    fn drop(&mut self) {
        let _ = FileExt::unlock(&self.file);
    }
}

fn lease_name(run_id: &str) -> String {
    format!("{run_id}.artifact-lease")
}

fn require_component(label: &str, value: &str) -> Result<(), AwareError> {
    validate_artifact_component(value, label)
}

fn create_child(parent: &Dir, name: &str) -> Result<Dir, AwareError> {
    if let Err(error) = parent.create_dir(name)
        && error.kind() != std::io::ErrorKind::AlreadyExists
    {
        return Err(error.into());
    }
    Ok(parent.open_dir_nofollow(name)?)
}

#[cfg(unix)]
fn sync_dir(dir: &Dir, _path: &Path) -> Result<(), AwareError> {
    dir.try_clone()?.into_std_file().sync_all()?;
    Ok(())
}

#[cfg(windows)]
fn sync_dir(dir: &Dir, path: &Path) -> Result<(), AwareError> {
    use std::os::windows::fs::OpenOptionsExt;
    use windows_sys::Win32::Storage::FileSystem::{
        FILE_FLAG_BACKUP_SEMANTICS, FILE_FLAG_OPEN_REPARSE_POINT,
    };
    let file = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .custom_flags(FILE_FLAG_BACKUP_SEMANTICS | FILE_FLAG_OPEN_REPARSE_POINT)
        .open(path)?;
    if file_identity(&file)? != file_identity(&dir.try_clone()?.into_std_file())? {
        return Err(AwareError::Validation(
            "report artifact directory changed before durable sync".into(),
        ));
    }
    file.sync_all()?;
    Ok(())
}

fn root(paths: &Paths) -> Result<Dir, AwareError> {
    Ok(Dir::open_ambient_dir(
        &paths.aware_home,
        ambient_authority(),
    )?)
}

fn instance_for_new_run(paths: &Paths, app: &str, instance: &str) -> Result<Dir, AwareError> {
    let home = root(paths)?;
    let logs = create_child(&home, "logs")?;
    let app_dir = create_child(&logs, app)?;
    let instance_dir = create_child(&app_dir, instance)?;
    let logs_path = paths.aware_home.join("logs");
    let app_path = logs_path.join(app);
    let instance_path = app_path.join(instance);
    sync_dir(&instance_dir, &instance_path)?;
    sync_dir(&app_dir, &app_path)?;
    sync_dir(&logs, &logs_path)?;
    sync_dir(&home, &paths.aware_home)?;
    Ok(instance_dir)
}

fn instance_for_existing_run(paths: &Paths, app: &str, instance: &str) -> Result<Dir, AwareError> {
    let home = root(paths)?;
    let logs = home.open_dir_nofollow("logs")?;
    let app_dir = logs.open_dir_nofollow(app)?;
    Ok(app_dir.open_dir_nofollow(instance)?)
}

#[cfg(unix)]
fn file_identity(file: &File) -> Result<String, AwareError> {
    use std::os::unix::fs::MetadataExt;
    let meta = file.metadata()?;
    Ok(format!("unix:{:x}:{:x}", meta.dev(), meta.ino()))
}

#[cfg(windows)]
fn file_identity(file: &File) -> Result<String, AwareError> {
    use std::os::windows::io::AsRawHandle;
    use windows_sys::Win32::Storage::FileSystem::{
        BY_HANDLE_FILE_INFORMATION, GetFileInformationByHandle,
    };
    let mut info = std::mem::MaybeUninit::<BY_HANDLE_FILE_INFORMATION>::uninit();
    // SAFETY: the raw handle belongs to the open File and stays live through
    // the call; the OS fills the output structure only on success.
    let ok = unsafe { GetFileInformationByHandle(file.as_raw_handle(), info.as_mut_ptr()) };
    if ok == 0 {
        return Err(std::io::Error::last_os_error().into());
    }
    // SAFETY: GetFileInformationByHandle returned success and initialized info.
    let info = unsafe { info.assume_init() };
    Ok(format!(
        "windows:{:x}:{:x}:{:x}",
        info.dwVolumeSerialNumber, info.nFileIndexHigh, info.nFileIndexLow
    ))
}

fn plain_file(dir: &Dir, name: &str) -> Result<(), AwareError> {
    let meta = dir.symlink_metadata(name)?;
    if !meta.is_file() || meta.file_type().is_symlink() {
        return Err(AwareError::Validation(
            "run lease is not a plain file".into(),
        ));
    }
    #[cfg(windows)]
    {
        if meta.file_attributes() & 0x400 != 0 {
            return Err(AwareError::Validation(
                "run lease is a reparse point".into(),
            ));
        }
    }
    Ok(())
}

fn classify_nodes(paths: &Paths, nodes: &[Node]) -> Result<WriterClass, AwareError> {
    for node in nodes {
        if node.frozen.is_some() {
            continue;
        }
        if let Some(agent) = &node.agent {
            let manifest = crate::manifest::loader::load_agent_by_id(&paths.agents_dir(), agent)?;
            let transport = effective_transport(&manifest, agent)?;
            let safe = transport == TransportKind::Rest
                || (transport == TransportKind::Builtin
                    && agent == "html-report-stream"
                    && node.command.as_deref() == Some("render-stream"));
            if !safe {
                return Ok(WriterClass::ExternalPossible);
            }
        }
        if let Some(body) = &node.do_
            && classify_nodes(paths, body)? != WriterClass::InProcess
        {
            return Ok(WriterClass::ExternalPossible);
        }
    }
    Ok(WriterClass::InProcess)
}

pub fn begin_if_reserved(
    paths: &Paths,
    app: &str,
    instance: &str,
    run_id: &str,
    graph: &App,
) -> Result<Option<RunLease>, AwareError> {
    let Some(reservation_id) = std::env::var_os("AWARE_REPORT_RESERVATION_ID") else {
        return Ok(None);
    };
    let reservation_id = reservation_id
        .to_str()
        .ok_or_else(|| AwareError::Validation("report reservation ID must be plain text".into()))?;
    for (label, value) in [
        ("app", app),
        ("instance", instance),
        ("run id", run_id),
        ("reservation id", reservation_id),
    ] {
        require_component(label, value)?;
    }
    match inspect(&paths.logs_dir(), reservation_id) {
        Ok(_) => {
            return Err(AwareError::Conflict(
                "report reservation was already used".into(),
            ));
        }
        Err(AwareError::NotFound(_)) => {}
        Err(error) => return Err(error),
    }
    let instance_dir = instance_for_new_run(paths, app, instance)?;
    let name = lease_name(run_id);
    let mut options = OpenOptions::new();
    options.read(true).write(true).create_new(true);
    let mut file = instance_dir.open_with(&name, &options)?.into_std();
    let writer_class = classify_nodes(paths, &graph.nodes)?;
    let record = LeaseRecord {
        schema_version: LEASE_SCHEMA.into(),
        reservation_id: reservation_id.into(),
        app: app.into(),
        instance: instance.into(),
        run_id: run_id.into(),
        writer_class,
    };
    file.write_all(&serde_json::to_vec(&record)?)?;
    file.sync_all()?;
    FileExt::try_lock_shared(&file)
        .map_err(|_| AwareError::Conflict("report run artifact lease is already owned".into()))?;
    let evidence = LeaseEvidence {
        file_identity: file_identity(&file)?,
        writer_class,
    };
    sync_dir(
        &instance_dir,
        &paths.aware_home.join("logs").join(app).join(instance),
    )?;
    Ok(Some(RunLease { file, evidence }))
}

pub struct ReaderLease(File);

impl Drop for ReaderLease {
    fn drop(&mut self) {
        let _ = FileExt::unlock(&self.0);
    }
}

pub fn reader_lease(
    paths: &Paths,
    app: &str,
    instance: &str,
    run_id: &str,
) -> Result<Option<ReaderLease>, AwareError> {
    let instance_dir = match instance_for_existing_run(paths, app, instance) {
        Ok(dir) => dir,
        Err(AwareError::Io(error)) if error.kind() == std::io::ErrorKind::NotFound => {
            return Ok(None);
        }
        Err(error) => return Err(error),
    };
    let name = lease_name(run_id);
    match instance_dir.symlink_metadata(&name) {
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(error) => return Err(error.into()),
        Ok(_) => {}
    }
    plain_file(&instance_dir, &name)?;
    let mut options = OpenOptions::new();
    options.read(true).write(true);
    let file = instance_dir.open_with(&name, &options)?.into_std();
    FileExt::try_lock_shared(&file)
        .map_err(|_| AwareError::Conflict("report artifacts are being retired".into()))?;
    Ok(Some(ReaderLease(file)))
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PruneResult<'a> {
    pub app: &'a str,
    pub instance: &'a str,
    pub run_id: &'a str,
    pub pruned: bool,
    pub bytes: u64,
    pub files: u64,
}

pub fn prune<'a>(
    paths: &Paths,
    app: &'a str,
    instance: &'a str,
    run_id: &'a str,
    reservation_id: &str,
) -> Result<PruneResult<'a>, AwareError> {
    for (label, value) in [
        ("app", app),
        ("instance", instance),
        ("run id", run_id),
        ("reservation id", reservation_id),
    ] {
        require_component(label, value)?;
    }
    let owner = inspect(&paths.logs_dir(), reservation_id)?;
    if owner.app != app || owner.instance != instance || owner.run_id != run_id {
        return Err(AwareError::Validation(
            "report reservation does not own this exact run".into(),
        ));
    }
    let evidence = owner.writer_lease.ok_or_else(|| {
        AwareError::Validation(
            "this report predates safe artifact retirement; keep it for support review".into(),
        )
    })?;
    if evidence.writer_class != WriterClass::InProcess {
        return Err(AwareError::Validation(
            "this report may have an external writer; its artifacts cannot be retired automatically"
                .into(),
        ));
    }
    let instance_dir = instance_for_existing_run(paths, app, instance)?;
    let name = lease_name(run_id);
    plain_file(&instance_dir, &name)?;
    let mut options = OpenOptions::new();
    options.read(true).write(true);
    let file = instance_dir.open_with(&name, &options)?.into_std();
    if file_identity(&file)? != evidence.file_identity {
        return Err(AwareError::Validation(
            "report writer lease changed; refusing artifact retirement".into(),
        ));
    }
    FileExt::try_lock_exclusive(&file).map_err(|_| {
        AwareError::Conflict("report writer or artifact reader is still active".into())
    })?;
    if file_identity(&file)? != evidence.file_identity {
        return Err(AwareError::Validation(
            "report writer lease changed; refusing artifact retirement".into(),
        ));
    }
    let artifact_name = format!("{run_id}.artifacts");
    let artifact_dir = match instance_dir.open_dir_nofollow(&artifact_name) {
        Ok(dir) => dir,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            return Ok(PruneResult {
                app,
                instance,
                run_id,
                pruned: true,
                bytes: 0,
                files: 0,
            });
        }
        Err(error) => return Err(error.into()),
    };
    let mut entries = Vec::new();
    let mut bytes = 0u64;
    for entry in artifact_dir.read_dir(".")? {
        let entry = entry?;
        let name = entry.file_name();
        let meta = artifact_dir.symlink_metadata(&name)?;
        if !meta.is_file() || meta.file_type().is_symlink() {
            return Err(AwareError::Validation(
                "run artifacts contain a linked or non-file entry".into(),
            ));
        }
        #[cfg(windows)]
        {
            if meta.file_attributes() & 0x400 != 0 {
                return Err(AwareError::Validation(
                    "run artifacts contain a reparse point".into(),
                ));
            }
        }
        bytes = bytes
            .checked_add(meta.len())
            .ok_or_else(|| AwareError::Validation("run artifact size overflow".into()))?;
        entries.push(name);
    }
    let files = u64::try_from(entries.len())
        .map_err(|_| AwareError::Validation("run artifact count overflow".into()))?;
    for name in entries {
        artifact_dir.remove_file(name)?;
    }
    let instance_path = paths.aware_home.join("logs").join(app).join(instance);
    sync_dir(&artifact_dir, &instance_path.join(&artifact_name))?;
    sync_dir(&instance_dir, &instance_path)?;
    Ok(PruneResult {
        app,
        instance,
        run_id,
        pruned: true,
        bytes,
        files,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_graph_has_no_external_writer() {
        let root = tempfile::tempdir().unwrap();
        let paths = Paths {
            aware_home: root.path().into(),
        };
        assert_eq!(classify_nodes(&paths, &[]).unwrap(), WriterClass::InProcess);
    }
}
