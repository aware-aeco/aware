//! The legacy store import (#627-b, plan §11–§12).

use super::*;
use crate::agent_store::{NO_RECEIPT, open, snapshot};

fn home() -> (tempfile::TempDir, Paths) {
    let tmp = tempfile::tempdir().unwrap();
    let paths = Paths {
        aware_home: tmp.path().to_path_buf(),
    };
    (tmp, paths)
}

fn write_agent(paths: &Paths, id: &str, version: &str) -> PathBuf {
    let dir = paths.agents_dir().join(id);
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(
        dir.join("manifest.yaml"),
        format!(
            "agent: {id}\nversion: {version}\ndescription: x\nstateful: false\nlicense: MIT\n\
             transport:\n  cli:\n    binary: aware-test\ncommands: {{}}\n"
        ),
    )
    .unwrap();
    dir
}

/// Make a package the way AWARE 0.149-0.151 did: snapshot it (into v2), then
/// move the whole v2 store to the legacy name, as if an older CLI wrote it.
fn legacy_package(paths: &Paths, id: &str, version: &str) -> crate::agent_store::StoredPackage {
    // Snapshot in a scratch home (through `open`, the only door to a guard;
    // a scratch home has no legacy store, so nothing is imported there), then
    // plant the package in this home's legacy store, as an older CLI left it.
    let tmp = tempfile::tempdir().unwrap();
    let scratch = Paths {
        aware_home: tmp.path().to_path_buf(),
    };
    let dir = write_agent(&scratch, id, version);
    let package = snapshot(&scratch, &dir, &open(&scratch).unwrap()).unwrap();
    let rel = package
        .root
        .strip_prefix(scratch.agent_store_dir())
        .unwrap()
        .to_path_buf();
    let target = paths.legacy_agent_store_dir().join(&rel);
    std::fs::create_dir_all(target.parent().unwrap()).unwrap();
    crate::fs::copy_dir_recursive(&package.root, &target).unwrap();
    crate::agent_store::StoredPackage {
        root: target,
        ..package
    }
}

/// Every file under `dir` with its bytes — to prove the legacy store is never
/// written.
fn tree(dir: &Path) -> BTreeMap<String, Vec<u8>> {
    crate::fs::plain_files_under(dir, "tree")
        .unwrap()
        .into_iter()
        .map(|(rel, path)| (rel, std::fs::read(path).unwrap()))
        .collect()
}

#[test]
fn a_legacy_package_is_imported_verified_and_the_legacy_store_is_never_written() {
    let (_tmp, paths) = home();
    let old = legacy_package(&paths, "tekla", "0.1.5");
    let before = tree(&paths.legacy_agent_store_dir());
    assert!(needed(&paths).unwrap());

    drop(open(&paths).unwrap());

    let imported = paths
        .agent_store_dir()
        .join("tekla")
        .join(digest_hex(&old.digest).unwrap())
        .join(NO_RECEIPT);
    let package = crate::agent_store::verify_package(&imported, "tekla", &old.digest, NO_RECEIPT)
        .expect("the imported package verifies");
    assert_eq!(package.version, "0.1.5");
    // The original record, byte for byte (its snapshotted-at is kept).
    assert_eq!(
        std::fs::read(imported.join(PACKAGE_FILE)).unwrap(),
        std::fs::read(old.root.join(PACKAGE_FILE)).unwrap()
    );
    assert_eq!(
        tree(&paths.legacy_agent_store_dir()),
        before,
        "legacy untouched"
    );
    assert!(!needed(&paths).unwrap(), "recorded");
    let record = read_record(&paths);
    assert_eq!(record.imported.len(), 1);

    // Idempotent: nothing more to do, and v2 is not copied over again.
    let v2_before = tree(&paths.agent_store_dir());
    drop(open(&paths).unwrap());
    assert_eq!(tree(&paths.agent_store_dir()), v2_before);
    assert_eq!(tree(&paths.legacy_agent_store_dir()), before);
}

#[test]
fn a_package_an_older_cli_adds_later_is_picked_up_by_the_next_command() {
    let (_tmp, paths) = home();
    drop(open(&paths).unwrap());
    assert!(!needed(&paths).unwrap());
    let later = legacy_package(&paths, "alpha", "2.0.0");
    assert!(needed(&paths).unwrap(), "the listing changed");
    drop(open(&paths).unwrap());
    let container = paths
        .agent_store_dir()
        .join("alpha")
        .join(digest_hex(&later.digest).unwrap());
    assert!(container.join(NO_RECEIPT).is_dir());
}

#[test]
fn a_legacy_package_that_does_not_verify_is_left_alone_and_reported() {
    let (_tmp, paths) = home();
    let bad = legacy_package(&paths, "tekla", "0.1.5");
    std::fs::write(
        bad.root.join("manifest.yaml"),
        "agent: tekla\nversion: 9.9.9\n",
    )
    .unwrap();
    let before = tree(&paths.legacy_agent_store_dir());
    drop(open(&paths).unwrap());
    assert!(
        !paths.agent_store_dir().join("tekla").exists(),
        "never imported"
    );
    let record = read_record(&paths);
    assert_eq!(record.skipped.len(), 1, "{record:?}");
    assert!(
        record.skipped.values().next().unwrap().contains("hash"),
        "{record:?}"
    );
    assert_eq!(tree(&paths.legacy_agent_store_dir()), before);
    // Recorded as seen: not retried on every command.
    assert!(!needed(&paths).unwrap());
}

#[test]
fn opening_the_store_while_this_thread_holds_a_guard_never_upgrades_it() {
    let (_tmp, paths) = home();
    let held = open(&paths).unwrap();
    legacy_package_with_guard_held(&paths);
    // A nested open on the same thread must not ask for the exclusive lock.
    let nested = open(&paths).unwrap();
    assert!(needed(&paths).unwrap(), "skipped while a guard is held");
    drop(nested);
    drop(held);
    drop(open(&paths).unwrap());
    assert!(!needed(&paths).unwrap(), "imported by the next open");
}

/// A legacy package written while this thread already holds a guard (the
/// helper above takes its own; here the bytes are planted by hand).
fn legacy_package_with_guard_held(paths: &Paths) {
    let other = paths.clone();
    std::thread::spawn(move || {
        let tmp = tempfile::tempdir().unwrap();
        let scratch = Paths {
            aware_home: tmp.path().to_path_buf(),
        };
        let made = legacy_package(&scratch, "tekla", "0.1.5");
        let rel = made
            .root
            .strip_prefix(scratch.legacy_agent_store_dir())
            .unwrap()
            .to_path_buf();
        let target = other.legacy_agent_store_dir().join(rel);
        std::fs::create_dir_all(target.parent().unwrap()).unwrap();
        crate::fs::copy_dir_recursive(&made.root, &target).unwrap();
    })
    .join()
    .unwrap();
}

#[test]
fn a_v2_store_that_is_the_legacy_store_is_refused() {
    let (_tmp, paths) = home();
    legacy_package(&paths, "tekla", "0.1.5");
    let before = tree(&paths.legacy_agent_store_dir());
    let v2 = paths.agent_store_dir();
    let legacy = paths.legacy_agent_store_dir();
    #[cfg(windows)]
    {
        let status = std::process::Command::new("cmd.exe")
            .args(["/C", "mklink", "/J"])
            .arg(&v2)
            .arg(&legacy)
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status()
            .unwrap();
        assert!(status.success(), "failed to create test junction");
    }
    #[cfg(unix)]
    std::os::unix::fs::symlink(&legacy, &v2).unwrap();
    let error = open(&paths).unwrap_err().to_string();
    assert!(error.contains("E_AGENT_STORE_ALIASED"), "{error}");
    assert_eq!(
        tree(&legacy),
        before,
        "nothing was written through the alias"
    );
}

#[test]
fn the_listing_ignores_what_is_not_a_package() {
    let (_tmp, paths) = home();
    let root = paths.legacy_agent_store_dir();
    std::fs::create_dir_all(root.join("tekla").join("a".repeat(64)).join(".tmp-x")).unwrap();
    std::fs::create_dir_all(root.join("tekla").join("not-hex")).unwrap();
    std::fs::write(root.join("stray.txt"), "x").unwrap();
    assert!(legacy_listing(&paths).unwrap().is_empty());
}
