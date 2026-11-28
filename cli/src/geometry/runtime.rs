use std::{
    env,
    fs::{self, File, OpenOptions},
    io::{self, Read},
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

use fs2::FileExt;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::error::AwareError;

const ABI: &str = "cp312-win_amd64";
const LOCK: &[u8] = include_bytes!("../../../geometry-worker/win-amd64-cp312.requirements.lock");
const MANIFEST: &[u8] = include_bytes!("../../../geometry-worker/wheel-manifest.json");
const LICENSES: &[(&str, &[u8])] = &[
    (
        "LICENSE_LGPL_21.txt",
        include_bytes!("../../../geometry-worker/licenses/LICENSE_LGPL_21.txt"),
    ),
    (
        "OCCT_LGPL_EXCEPTION.txt",
        include_bytes!("../../../geometry-worker/licenses/OCCT_LGPL_EXCEPTION.txt"),
    ),
    (
        "OCP-LICENSE.txt",
        include_bytes!("../../../geometry-worker/licenses/OCP-LICENSE.txt"),
    ),
    (
        "VTK-LICENSE.txt",
        include_bytes!("../../../geometry-worker/licenses/VTK-LICENSE.txt"),
    ),
];
const BASE_PROBE: &str = r#"
import json, platform, struct, sys
assert sys.implementation.name == 'cpython' and sys.version_info[:2] == (3, 12)
assert sys.platform == 'win32' and platform.machine().upper() == 'AMD64' and struct.calcsize('P') == 8
assert sys.prefix == sys.base_prefix
print(json.dumps({'path': sys.executable, 'version': platform.python_version()}))
"#;
const NATIVE_PROBE: &str = r#"
import json, platform, sys
from importlib.metadata import version
import OCP, vtk
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.Precision import Precision
assert sys.prefix != sys.base_prefix
assert version('cadquery-ocp') == '8.0.1.1.0' and OCP.__version__ == '8.0.1.1'
assert version('vtk') == vtk.vtkVersion.GetVTKVersion() == '9.6.2'
cut = BRepAlgoAPI_Cut()
cut.SetFuzzyValue(0)
assert cut.FuzzyValue() == Precision.Confusion_s() == 1e-7
print(json.dumps({'python': platform.python_version(), 'ocp': version('cadquery-ocp'),
    'occt': '8.0.1', 'vtk': version('vtk'), 'native_fuzzy_floor_mm': cut.FuzzyValue()}))
"#;

#[derive(Deserialize)]
struct Manifest {
    abi: String,
    wheels: Vec<Wheel>,
}

#[derive(Deserialize)]
struct Wheel {
    filename: String,
    size: u64,
    url: String,
    sha256: String,
}

#[derive(Deserialize, Serialize)]
struct Base {
    path: PathBuf,
    version: String,
}

#[derive(Deserialize, Serialize)]
pub struct Installation {
    path: PathBuf,
    abi: String,
    lock_sha256: String,
    manifest_sha256: String,
    base: Base,
    base_exe_sha256: String,
    base_dll_sha256: String,
    venv_exe_sha256: String,
    probe: serde_json::Value,
}

fn invalid(message: impl Into<String>) -> AwareError {
    AwareError::Validation(message.into())
}

fn hash(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

fn hash_file(path: &Path) -> io::Result<String> {
    let mut sha = Sha256::new();
    io::copy(&mut File::open(path)?, &mut sha)?;
    Ok(format!("{:x}", sha.finalize()))
}

fn parent(path: &Path) -> Result<&Path, AwareError> {
    path.parent()
        .ok_or_else(|| invalid("Python base path has no parent"))
}

fn base_dll(path: &Path) -> Result<PathBuf, AwareError> {
    Ok(parent(path)?.join("python312.dll"))
}

/// Start with an empty environment and restore only the OS and temporary paths
/// needed by the venv launcher and pip. NUL disables pip config files.
pub fn configure_python(command: &mut Command) {
    command.env_clear();
    for key in ["SystemRoot", "SystemDrive", "WINDIR", "TEMP", "TMP"] {
        if let Some(value) = env::var_os(key) {
            command.env(key, value);
        }
    }
    command.env("PIP_CONFIG_FILE", "nul").stdin(Stdio::null());
}

fn python(path: &Path) -> Command {
    let mut command = Command::new(path);
    configure_python(&mut command);
    command.arg("-I");
    command
}

fn successful(command: &mut Command, stage: &str) -> Result<Vec<u8>, AwareError> {
    let output = command.output()?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(invalid(format!(
            "{stage} failed: {}",
            stderr.chars().take(512).collect::<String>()
        )));
    }
    Ok(output.stdout)
}

fn native_probe(venv_exe: &Path) -> Result<serde_json::Value, AwareError> {
    Ok(serde_json::from_slice(&successful(
        python(venv_exe).args(["-c", NATIVE_PROBE]),
        "OCCT native probe",
    )?)?)
}

fn verify_signatures(exe: &Path, dll: &Path) -> Result<(), AwareError> {
    let system = env::var_os("SystemRoot").ok_or_else(|| invalid("SystemRoot is unavailable"))?;
    let windows_powershell =
        PathBuf::from(system).join("System32/WindowsPowerShell/v1.0/powershell.exe");
    let mut command = Command::new(windows_powershell);
    configure_python(&mut command);
    command
        .env("AWARE_PROBE_EXE", exe)
        .env("AWARE_PROBE_DLL", dll);
    command.args(["-NoProfile", "-NonInteractive", "-Command", r#"
$ErrorActionPreference = 'Stop'
foreach ($path in $env:AWARE_PROBE_EXE, $env:AWARE_PROBE_DLL) {
    $signature = Get-AuthenticodeSignature -LiteralPath $path
    if ($signature.Status -ne 'Valid' -or $signature.SignerCertificate.Subject -notmatch '(^|, )CN=Python Software Foundation(,|$)') {
        throw 'Expected a trusted Python Software Foundation signature'
    }
}
"#]);
    successful(&mut command, "Python provenance check")?;
    Ok(())
}

fn platform_root(home: &Path) -> PathBuf {
    home.join("runtimes/occt").join(ABI)
}

fn installed_path(home: &Path) -> PathBuf {
    platform_root(home).join(hash(LOCK))
}

fn check_installed(path: &Path) -> Result<Installation, AwareError> {
    let marker: Installation =
        serde_json::from_reader(File::open(path.join("complete.json"))?.take(64 * 1024))?;
    let config = fs::read_to_string(path.join("pyvenv.cfg"))?;
    let entry = |key: &str| {
        config
            .lines()
            .filter_map(|line| line.split_once(" = "))
            .find_map(|(name, value)| (name == key).then_some(value))
    };
    let configured_home = entry("home")
        .map(Path::new)
        .ok_or_else(|| invalid("venv is missing its base path"))?;
    let configured_exe = entry("executable")
        .map(Path::new)
        .ok_or_else(|| invalid("venv is missing its base executable"))?;
    if marker.abi != ABI
        || marker.path != path
        || marker.lock_sha256 != hash(LOCK)
        || marker.manifest_sha256 != hash(MANIFEST)
        || entry("include-system-site-packages") != Some("false")
        || entry("version") != Some(&marker.base.version)
        || configured_exe.canonicalize()? != marker.base.path.canonicalize()?
        || configured_home.canonicalize()? != parent(&marker.base.path)?.canonicalize()?
        || marker.base_exe_sha256 != hash_file(&marker.base.path)?
        || marker.base_dll_sha256 != hash_file(&base_dll(&marker.base.path)?)?
        || marker.venv_exe_sha256 != hash_file(&path.join("Scripts/python.exe"))?
    {
        return Err(invalid(
            "OCCT runtime completion marker or base identity has changed",
        ));
    }
    Ok(marker)
}

pub struct RuntimeLease {
    pub python: PathBuf,
    _lock: File,
}

pub fn installed_runtime(home: &Path) -> Option<RuntimeLease> {
    let home = std::path::absolute(home).ok()?;
    let lock = OpenOptions::new()
        .read(true)
        .write(true)
        .open(platform_root(&home).join("install.lock"))
        .ok()?;
    FileExt::try_lock_shared(&lock).ok()?;
    let path = installed_path(&home);
    check_installed(&path).ok()?;
    Some(RuntimeLease {
        python: path.join("Scripts/python.exe"),
        _lock: lock,
    })
}

fn stage_wheel(wheel: &Wheel, wheelhouse: Option<&Path>, dest: &Path) -> Result<(), AwareError> {
    let target = dest.join(&wheel.filename);
    let mut input: Box<dyn Read> = if let Some(source) = wheelhouse {
        Box::new(File::open(source.join(&wheel.filename))?)
    } else {
        let response = ureq::AgentBuilder::new()
            .timeout(std::time::Duration::from_secs(300))
            .build()
            .get(&wheel.url)
            .call()
            .map_err(|error| AwareError::Network(format!("wheel download failed: {error}")))?;
        response.into_reader()
    };
    let size = io::copy(
        &mut input.by_ref().take(wheel.size + 1),
        &mut File::create(&target)?,
    )?;
    if size != wheel.size || hash_file(&target)? != wheel.sha256 {
        return Err(invalid(format!(
            "wheel digest mismatch: {}",
            wheel.filename
        )));
    }
    Ok(())
}

pub fn install(
    home: &Path,
    base_exe: &Path,
    wheelhouse: Option<&Path>,
) -> Result<Installation, AwareError> {
    if !base_exe.is_absolute()
        || !base_exe
            .file_name()
            .is_some_and(|name| name.eq_ignore_ascii_case("python.exe"))
    {
        return Err(invalid(
            "--python requires an absolute CPython python.exe path",
        ));
    }
    let base = base_exe.canonicalize()?;
    let dll = base_dll(&base)?;
    verify_signatures(&base, &dll)?;
    let base_exe_sha256 = hash_file(&base)?;
    let base_dll_sha256 = hash_file(&dll)?;
    let base: Base = serde_json::from_slice(&successful(
        python(base_exe).args(["-c", BASE_PROBE]),
        "Python ABI probe",
    )?)?;
    if base.path.canonicalize()? != base_exe.canonicalize()? {
        return Err(invalid(
            "Python base executable identity differs from --python",
        ));
    }

    let home = std::path::absolute(home)?;
    let platform = platform_root(&home);
    fs::create_dir_all(&platform)?;
    let install_lock = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .open(platform.join("install.lock"))?;
    FileExt::lock_exclusive(&install_lock)?;
    let root = installed_path(&home);

    let manifest: Manifest = serde_json::from_slice(MANIFEST)?;
    if manifest.abi != ABI || manifest.wheels.is_empty() {
        return Err(invalid("embedded OCCT wheel manifest has the wrong ABI"));
    }
    let wheels = tempfile::tempdir_in(&platform)?;
    for wheel in &manifest.wheels {
        stage_wheel(wheel, wheelhouse, wheels.path())?;
    }
    let lock_file = wheels.path().join("requirements.lock");
    fs::write(&lock_file, LOCK)?;
    let venv_exe = root.join("Scripts/python.exe");
    let licenses = root.join("licenses");
    if let Ok(existing) = check_installed(&root)
        && existing.base_exe_sha256 == base_exe_sha256
        && existing.base_dll_sha256 == base_dll_sha256
        && existing.base.path.canonicalize()? == base.path.canonicalize()?
        && LICENSES.iter().all(|(name, expected)| {
            fs::read(licenses.join(name)).is_ok_and(|bytes| bytes == *expected)
        })
        && native_probe(&venv_exe).is_ok()
    {
        return Ok(existing);
    }

    if let Ok(resolved) = root.canonicalize() {
        if resolved != platform.canonicalize()?.join(hash(LOCK)) {
            return Err(invalid(
                "OCCT runtime directory resolves outside the locked path",
            ));
        }
        fs::remove_dir_all(&root)?;
    }
    successful(
        python(&base.path).args(["-m", "venv"]).arg(&root),
        "create OCCT virtual environment",
    )?;
    successful(
        python(&venv_exe)
            .args([
                "-m",
                "pip",
                "--isolated",
                "install",
                "--require-hashes",
                "--no-deps",
                "--only-binary=:all:",
                "--no-index",
                "--find-links",
            ])
            .arg(wheels.path())
            .arg("-r")
            .arg(&lock_file),
        "install locked OCCT wheels",
    )?;
    let probe = native_probe(&venv_exe)?;
    if base_exe_sha256 != hash_file(&base.path)?
        || base_dll_sha256 != hash_file(&base_dll(&base.path)?)?
    {
        return Err(invalid("base Python changed during installation"));
    }
    fs::create_dir_all(&licenses)?;
    for (name, notice) in LICENSES {
        fs::write(licenses.join(name), notice)?;
    }
    fs::write(root.join("wheel-manifest.json"), MANIFEST)?;
    fs::write(root.join("requirements.lock"), LOCK)?;
    let marker = Installation {
        abi: ABI.into(),
        lock_sha256: hash(LOCK),
        manifest_sha256: hash(MANIFEST),
        base,
        base_exe_sha256,
        base_dll_sha256,
        venv_exe_sha256: hash_file(&venv_exe)?,
        path: root.clone(),
        probe,
    };
    let file = tempfile::NamedTempFile::new_in(&root)?;
    serde_json::to_writer(file.as_file(), &marker)?;
    file.as_file().sync_all()?;
    file.persist(root.join("complete.json"))
        .map_err(|failure| failure.error)?;
    Ok(marker)
}
