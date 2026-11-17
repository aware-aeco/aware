//! #845: a version-less `agent install` / `agent update` takes the newest registry release the
//! running CLI can run, and says which newer release it held back. `<id>@<version>` is unchanged.
//!
//! Every fixture judges releases against the real running CLI, so "runs here" is a floor of
//! `0.1.0` and "needs a newer AWARE" is a floor of `999.0.0`, exactly as the existing
//! runtime-gate tests in this module do.

use super::*;
use crate::validate::CURRENT_CLI_VERSION;
use std::io::Write;

const AGENT: &str = "floor-probe";
const RUNS_HERE: &str = "0.1.0";
const TOO_NEW: &str = "999.0.0";

/// One release of the fixture agent: its manifest version, the CLI floor its manifest declares,
/// and whether its manifest lists a skill file the payload does not carry (a broken release).
#[derive(Clone, Copy)]
struct Payload {
    version: &'static str,
    manifest_floor: Option<&'static str>,
    broken: bool,
}

fn payload(version: &'static str, manifest_floor: Option<&'static str>) -> Payload {
    Payload {
        version,
        manifest_floor,
        broken: false,
    }
}

fn manifest_text(p: Payload) -> String {
    let mut text = format!(
        "agent: {AGENT}\nversion: {}\ndescription: Runtime floor fixture.\nstateful: false\nlicense: MIT\n",
        p.version
    );
    if let Some(floor) = p.manifest_floor {
        text.push_str(&format!(
            "status: requires-runtime\nminimum-cli-version: {floor}\n"
        ));
    }
    text.push_str(
        "transport:\n  cli:\n    binary: floor-probe\ncommands:\n  ping:\n    lifecycle: single\n    description: Ping.\n",
    );
    if p.broken {
        text.push_str("skills:\n  - absent.md\n");
    }
    text
}

fn subdir(version: &str) -> String {
    format!("aware-main/20-agents/{AGENT}-{version}")
}

/// One archive holding every given release under its own subdir, as the substrate's
/// repository archive holds many agents.
fn write_archive(path: &Path, payloads: &[Payload]) {
    let enc = flate2::write::GzEncoder::new(
        std::fs::File::create(path).unwrap(),
        flate2::Compression::default(),
    );
    let mut tar = tar::Builder::new(enc);
    for p in payloads {
        let text = manifest_text(*p);
        let mut h = tar::Header::new_gnu();
        h.set_size(text.len() as u64);
        h.set_mode(0o644);
        h.set_cksum();
        tar.append_data(
            &mut h,
            format!("{}/manifest.yaml", subdir(p.version)),
            text.as_bytes(),
        )
        .unwrap();
    }
    let mut file = tar.into_inner().unwrap().finish().unwrap();
    file.flush().unwrap();
}

/// One index release: its registry version, the archive it is fetched from, and the CLI floor
/// the INDEX records for it (`None` = a legacy or custom entry that records none).
struct Release<'a> {
    version: &'a str,
    tarball: &'a Path,
    index_floor: Option<&'a str>,
}

fn index_of(releases: &[Release<'_>]) -> Index {
    let mut versions = serde_json::Map::new();
    for r in releases {
        let mut entry = serde_json::json!({
            "tarball": format!("file://{}", r.tarball.display()),
            "subdir": subdir(r.version),
            "manifest-agent": AGENT,
            "manifest-version": r.version,
        });
        if let Some(floor) = r.index_floor {
            entry["minimum-cli-version"] = floor.into();
        }
        versions.insert(r.version.to_string(), entry);
    }
    let raw = serde_json::to_vec(&serde_json::json!({
        "version": "1.0",
        "updated-at": "2026-11-17T00:00:00Z",
        "agents": { AGENT: { "versions": versions } },
        "bundles": {}
    }))
    .unwrap();
    Index::parse(raw.as_slice()).unwrap()
}

fn home(tmp: &Path) -> Paths {
    Paths {
        aware_home: tmp.join("aware"),
    }
}

fn install(paths: &Paths, index: &Index, pin: Option<&str>) -> Result<RegistryOutcome, AwareError> {
    install_agent_from_registry(
        AGENT,
        pin,
        paths,
        index,
        &crate::agent_store::open(paths).unwrap(),
    )
}

fn update(paths: &Paths, index: &Index, pin: Option<&str>) -> Result<RegistryOutcome, AwareError> {
    update_agent_from_registry(
        AGENT,
        pin,
        false,
        paths,
        index,
        &crate::agent_store::open(paths).unwrap(),
    )
}

fn installed_version(paths: &Paths) -> String {
    let manifest = paths.agents_dir().join(AGENT).join("manifest.yaml");
    load_agent(&manifest).unwrap().version
}

fn held(version: &str, minimum: &str) -> Option<HeldBack> {
    Some(HeldBack {
        key: AGENT.into(),
        version: version.into(),
        minimum_cli_version: minimum.into(),
    })
}

#[test]
fn a_bare_install_holds_back_a_release_whose_index_floor_is_above_this_cli_without_fetching_it() {
    let tmp = tempfile::tempdir().unwrap();
    let archive = tmp.path().join("old.tar.gz");
    write_archive(&archive, &[payload("0.1.0", Some(RUNS_HERE))]);
    // The too-new release's archive does not exist: fetching it at all would fail the install.
    let never_fetched = tmp.path().join("never-fetched.tar.gz");
    let index = index_of(&[
        Release {
            version: "0.1.0",
            tarball: &archive,
            index_floor: Some(RUNS_HERE),
        },
        Release {
            version: "0.2.0",
            tarball: &never_fetched,
            index_floor: Some(TOO_NEW),
        },
    ]);
    let paths = home(tmp.path());

    let outcome = install(&paths, &index, None).unwrap();

    assert_eq!(outcome.agent, AGENT);
    assert_eq!(outcome.registry_version, "0.1.0");
    assert_eq!(outcome.held_back, held("0.2.0", TOO_NEW));
    assert_eq!(installed_version(&paths), "0.1.0");
    assert_eq!(
        outcome.held_back.unwrap().to_string(),
        format!("{AGENT} 0.2.0 needs AWARE {TOO_NEW} or newer")
    );
}

#[test]
fn a_bare_install_holds_back_a_release_whose_manifest_alone_names_a_newer_floor() {
    // A custom or legacy index records no floor; the payload's manifest is the only judge.
    let tmp = tempfile::tempdir().unwrap();
    let archive = tmp.path().join("repo.tar.gz");
    write_archive(
        &archive,
        &[
            payload("0.1.0", Some(RUNS_HERE)),
            payload("0.2.0", Some(TOO_NEW)),
        ],
    );
    let index = index_of(&[
        Release {
            version: "0.1.0",
            tarball: &archive,
            index_floor: None,
        },
        Release {
            version: "0.2.0",
            tarball: &archive,
            index_floor: None,
        },
    ]);
    let paths = home(tmp.path());

    let outcome = install(&paths, &index, None).unwrap();

    assert_eq!(outcome.registry_version, "0.1.0");
    assert_eq!(outcome.held_back, held("0.2.0", TOO_NEW));
    assert_eq!(installed_version(&paths), "0.1.0");
}

#[test]
fn a_bare_install_with_nothing_newer_held_back_reports_none() {
    let tmp = tempfile::tempdir().unwrap();
    let archive = tmp.path().join("repo.tar.gz");
    write_archive(
        &archive,
        &[payload("0.1.0", None), payload("0.2.0", Some(RUNS_HERE))],
    );
    let index = index_of(&[
        Release {
            version: "0.1.0",
            tarball: &archive,
            index_floor: None,
        },
        Release {
            version: "0.2.0",
            tarball: &archive,
            index_floor: Some(RUNS_HERE),
        },
    ]);
    let paths = home(tmp.path());

    let outcome = install(&paths, &index, None).unwrap();

    assert_eq!(outcome.registry_version, "0.2.0");
    assert_eq!(outcome.held_back, None);
}

#[test]
fn a_bare_install_refuses_before_any_download_when_no_release_runs_here() {
    let tmp = tempfile::tempdir().unwrap();
    let never_fetched = tmp.path().join("never-fetched.tar.gz");
    let index = index_of(&[Release {
        version: "0.2.0",
        tarball: &never_fetched,
        index_floor: Some(TOO_NEW),
    }]);
    let paths = home(tmp.path());

    let message = install(&paths, &index, None).unwrap_err().to_string();

    assert!(message.contains("E_AGENT_RUNTIME_TOO_OLD"), "{message}");
    assert!(message.contains(TOO_NEW), "{message}");
    assert!(message.contains(CURRENT_CLI_VERSION), "{message}");
    assert!(!paths.agents_dir().join(AGENT).exists());
}

#[test]
fn the_refusal_names_the_lowest_floor_any_release_needs_not_the_oldest_releases() {
    // Floors need not rise with versions: the OLDER release here needs the newer CLI.
    let tmp = tempfile::tempdir().unwrap();
    let never_fetched = tmp.path().join("never-fetched.tar.gz");
    let index = index_of(&[
        Release {
            version: "1.0.0",
            tarball: &never_fetched,
            index_floor: Some("999.5.0"),
        },
        Release {
            version: "2.0.0",
            tarball: &never_fetched,
            index_floor: Some(TOO_NEW),
        },
    ]);
    let paths = home(tmp.path());

    let message = install(&paths, &index, None).unwrap_err().to_string();

    assert!(message.contains("E_AGENT_RUNTIME_TOO_OLD"), "{message}");
    assert!(
        message.contains(&format!("{TOO_NEW} or newer")),
        "the lowest floor any release needs is what an upgrade must reach: {message}"
    );
    assert!(
        !message.contains("999.5.0 or newer"),
        "the older release's higher floor is not the upgrade to name: {message}"
    );
}

#[test]
fn a_bare_install_still_refuses_when_only_the_manifest_says_it_is_too_new() {
    let tmp = tempfile::tempdir().unwrap();
    let archive = tmp.path().join("repo.tar.gz");
    write_archive(&archive, &[payload("0.2.0", Some(TOO_NEW))]);
    let index = index_of(&[Release {
        version: "0.2.0",
        tarball: &archive,
        index_floor: None,
    }]);
    let paths = home(tmp.path());

    let message = install(&paths, &index, None).unwrap_err().to_string();

    assert!(message.contains("E_AGENT_RUNTIME_TOO_OLD"), "{message}");
    assert!(!paths.agents_dir().join(AGENT).exists());
}

#[test]
fn a_bare_install_never_skips_past_a_broken_release() {
    // The newest release runs here but is broken: that is reported, not quietly replaced by 0.1.0.
    let tmp = tempfile::tempdir().unwrap();
    let archive = tmp.path().join("repo.tar.gz");
    let mut broken = payload("0.2.0", Some(RUNS_HERE));
    broken.broken = true;
    write_archive(&archive, &[payload("0.1.0", Some(RUNS_HERE)), broken]);
    let index = index_of(&[
        Release {
            version: "0.1.0",
            tarball: &archive,
            index_floor: Some(RUNS_HERE),
        },
        Release {
            version: "0.2.0",
            tarball: &archive,
            index_floor: Some(RUNS_HERE),
        },
    ]);
    let paths = home(tmp.path());

    let message = install(&paths, &index, None).unwrap_err().to_string();

    assert!(message.contains("absent.md"), "{message}");
    assert!(!paths.agents_dir().join(AGENT).exists());
}

#[test]
fn a_too_new_release_that_is_also_broken_is_reported_not_held_back() {
    // Holding back is for a release whose ONLY problem is the CLI floor.
    let tmp = tempfile::tempdir().unwrap();
    let archive = tmp.path().join("repo.tar.gz");
    let mut broken = payload("0.2.0", Some(TOO_NEW));
    broken.broken = true;
    write_archive(&archive, &[payload("0.1.0", Some(RUNS_HERE)), broken]);
    let index = index_of(&[
        Release {
            version: "0.1.0",
            tarball: &archive,
            index_floor: None,
        },
        Release {
            version: "0.2.0",
            tarball: &archive,
            index_floor: None,
        },
    ]);
    let paths = home(tmp.path());

    let message = install(&paths, &index, None).unwrap_err().to_string();

    assert!(message.contains("absent.md"), "{message}");
    assert!(!paths.agents_dir().join(AGENT).exists());
}

#[test]
fn a_pinned_install_of_a_release_this_cli_cannot_run_still_refuses() {
    let tmp = tempfile::tempdir().unwrap();
    let archive = tmp.path().join("repo.tar.gz");
    write_archive(
        &archive,
        &[
            payload("0.1.0", Some(RUNS_HERE)),
            payload("0.2.0", Some(TOO_NEW)),
        ],
    );
    let index = index_of(&[
        Release {
            version: "0.1.0",
            tarball: &archive,
            index_floor: Some(RUNS_HERE),
        },
        Release {
            version: "0.2.0",
            tarball: &archive,
            index_floor: Some(TOO_NEW),
        },
    ]);
    let paths = home(tmp.path());

    let message = install(&paths, &index, Some("0.2.0"))
        .unwrap_err()
        .to_string();

    assert!(message.contains("E_AGENT_RUNTIME_TOO_OLD"), "{message}");
    assert!(!paths.agents_dir().join(AGENT).exists());

    let outcome = install(&paths, &index, Some("0.1.0")).unwrap();
    assert_eq!(outcome.registry_version, "0.1.0");
    assert_eq!(
        outcome.held_back, None,
        "a pinned install holds nothing back"
    );
}

#[test]
fn the_index_floor_must_match_the_floor_the_payload_declares() {
    let tmp = tempfile::tempdir().unwrap();
    let archive = tmp.path().join("repo.tar.gz");
    write_archive(&archive, &[payload("0.3.0", Some("0.0.2"))]);
    let index = index_of(&[Release {
        version: "0.3.0",
        tarball: &archive,
        index_floor: Some(RUNS_HERE),
    }]);
    let paths = home(tmp.path());

    for pin in [None, Some("0.3.0")] {
        let message = install(&paths, &index, pin).unwrap_err().to_string();
        assert!(message.contains("floor-probe@0.3.0"), "{message}");
        assert!(
            message.contains(&format!("records minimum-cli-version {RUNS_HERE}")),
            "{message}"
        );
        assert!(message.contains("declares \"0.0.2\""), "{message}");
        assert!(!paths.agents_dir().join(AGENT).exists());
    }
}

#[test]
fn an_index_floor_the_payload_does_not_declare_is_a_mismatch() {
    let tmp = tempfile::tempdir().unwrap();
    let archive = tmp.path().join("repo.tar.gz");
    write_archive(&archive, &[payload("0.1.0", None)]);
    let index = index_of(&[Release {
        version: "0.1.0",
        tarball: &archive,
        index_floor: Some(RUNS_HERE),
    }]);
    let paths = home(tmp.path());

    let message = install(&paths, &index, None).unwrap_err().to_string();

    assert!(message.contains("minimum-cli-version"), "{message}");
    assert!(message.contains("declares none"), "{message}");
    assert!(!paths.agents_dir().join(AGENT).exists());
}

/// The next patch release after the running CLI: the smallest floor it does not meet.
fn next_patch_after_current() -> String {
    let parts: Vec<u64> = CURRENT_CLI_VERSION
        .split('.')
        .map(|part| part.parse().unwrap())
        .collect();
    format!("{}.{}.{}", parts[0], parts[1], parts[2] + 1)
}

#[test]
fn a_floor_equal_to_the_running_cli_runs_here_and_the_next_patch_does_not() {
    let tmp = tempfile::tempdir().unwrap();
    let archive = tmp.path().join("repo.tar.gz");
    write_archive(
        &archive,
        &[
            payload("0.1.0", Some(RUNS_HERE)),
            payload("0.2.0", Some(CURRENT_CLI_VERSION)),
        ],
    );
    let never_fetched = tmp.path().join("never-fetched.tar.gz");
    let next = next_patch_after_current();
    let index = index_of(&[
        Release {
            version: "0.1.0",
            tarball: &archive,
            index_floor: Some(RUNS_HERE),
        },
        Release {
            version: "0.2.0",
            tarball: &archive,
            index_floor: Some(CURRENT_CLI_VERSION),
        },
        Release {
            version: "0.3.0",
            tarball: &never_fetched,
            index_floor: Some(&next),
        },
    ]);
    let paths = home(tmp.path());

    let outcome = install(&paths, &index, None).unwrap();

    assert_eq!(
        outcome.registry_version, "0.2.0",
        "a met floor is met exactly"
    );
    assert_eq!(outcome.held_back, held("0.3.0", &next));
}

#[test]
fn the_success_line_names_the_newest_release_held_back_and_the_refusal_the_lowest_floor() {
    let tmp = tempfile::tempdir().unwrap();
    let archive = tmp.path().join("repo.tar.gz");
    write_archive(&archive, &[payload("0.1.0", Some(RUNS_HERE))]);
    let never_fetched = tmp.path().join("never-fetched.tar.gz");
    // Floors deliberately out of version order: newest 0.3.0 needs the most, 0.2.0 the least.
    let too_new = [
        ("0.3.0", "999.5.0"),
        ("0.2.0", TOO_NEW),
        ("0.1.5", "999.2.0"),
    ];
    let mut releases: Vec<Release<'_>> = too_new
        .iter()
        .map(|(version, floor)| Release {
            version,
            tarball: &never_fetched,
            index_floor: Some(floor),
        })
        .collect();
    let paths = home(tmp.path());

    let message = install(&paths, &index_of(&releases), None)
        .unwrap_err()
        .to_string();
    assert!(
        message.contains(&format!("{TOO_NEW} or newer")),
        "{message}"
    );
    assert!(message.contains("floor-probe@0.2.0"), "{message}");
    assert!(!message.contains("999.5.0 or newer"), "{message}");
    assert!(!message.contains("999.2.0 or newer"), "{message}");

    releases.push(Release {
        version: "0.1.0",
        tarball: &archive,
        index_floor: Some(RUNS_HERE),
    });
    let outcome = install(&paths, &index_of(&releases), None).unwrap();
    assert_eq!(outcome.registry_version, "0.1.0");
    assert_eq!(
        outcome.held_back,
        held("0.3.0", "999.5.0"),
        "the success line names the newest release passed over"
    );
}

#[test]
fn a_key_that_is_not_semver_is_never_what_a_bare_install_falls_back_to() {
    // The `nightly` key is a sound, runnable release (its manifest is 0.0.1); only its key is
    // not SemVer.
    let tmp = tempfile::tempdir().unwrap();
    let archive = tmp.path().join("repo.tar.gz");
    write_archive(&archive, &[payload("0.0.1", Some(RUNS_HERE))]);
    let never_fetched = tmp.path().join("never-fetched.tar.gz");
    let nightly = serde_json::json!({
        "tarball": format!("file://{}", archive.display()),
        "subdir": subdir("0.0.1"),
        "manifest-agent": AGENT,
        "manifest-version": "0.0.1",
        "minimum-cli-version": RUNS_HERE,
    });
    let index_with = |versions: serde_json::Value| {
        let raw = serde_json::to_vec(&serde_json::json!({
            "version": "1.0", "updated-at": "2026-11-17T00:00:00Z",
            "agents": { AGENT: { "versions": versions } }, "bundles": {}
        }))
        .unwrap();
        Index::parse(raw.as_slice()).unwrap()
    };
    let paths = home(tmp.path());

    let mixed = index_with(serde_json::json!({
        "1.0.0": {
            "tarball": format!("file://{}", never_fetched.display()),
            "subdir": subdir("1.0.0"),
            "manifest-agent": AGENT,
            "manifest-version": "1.0.0",
            "minimum-cli-version": TOO_NEW,
        },
        "nightly": nightly.clone(),
    }));
    let message = install(&paths, &mixed, None).unwrap_err().to_string();
    assert!(message.contains("E_AGENT_RUNTIME_TOO_OLD"), "{message}");
    assert!(!paths.agents_dir().join(AGENT).exists());

    // With no SemVer key at all there is no ranking to appeal to, as `resolve` has always had it.
    let only_odd = index_with(serde_json::json!({ "nightly": nightly }));
    assert_eq!(
        install(&paths, &only_odd, None).unwrap().registry_version,
        "nightly"
    );
}

/// A manifest a CLI this old cannot parse: a status value from the future, with a floor.
fn write_unparseable_too_new(archive: &Path, also: &[Payload]) {
    let enc = flate2::write::GzEncoder::new(
        std::fs::File::create(archive).unwrap(),
        flate2::Compression::default(),
    );
    let mut tar = tar::Builder::new(enc);
    let mut add = |path: String, text: String| {
        let mut h = tar::Header::new_gnu();
        h.set_size(text.len() as u64);
        h.set_mode(0o644);
        h.set_cksum();
        tar.append_data(&mut h, path, text.as_bytes()).unwrap();
    };
    for p in also {
        add(
            format!("{}/manifest.yaml", subdir(p.version)),
            manifest_text(*p),
        );
    }
    let future = manifest_text(payload("0.2.0", Some(TOO_NEW)))
        .replace("status: requires-runtime", "status: from-a-future-aware");
    add(format!("{}/manifest.yaml", subdir("0.2.0")), future);
    let mut file = tar.into_inner().unwrap().finish().unwrap();
    file.flush().unwrap();
}

#[test]
fn a_too_new_release_this_cli_cannot_even_parse_is_held_back_by_its_raw_floor() {
    // A registry that records no floors, serving a release whose manifest uses syntax only a
    // newer AWARE knows: its raw floor still holds it back instead of failing the install.
    let tmp = tempfile::tempdir().unwrap();
    let archive = tmp.path().join("repo.tar.gz");
    write_unparseable_too_new(&archive, &[payload("0.1.0", Some(RUNS_HERE))]);
    let index = index_of(&[
        Release {
            version: "0.1.0",
            tarball: &archive,
            index_floor: None,
        },
        Release {
            version: "0.2.0",
            tarball: &archive,
            index_floor: None,
        },
    ]);
    let paths = home(tmp.path());

    let outcome = install(&paths, &index, None).unwrap();
    assert_eq!(outcome.registry_version, "0.1.0");
    assert_eq!(outcome.held_back, held("0.2.0", TOO_NEW));

    // Named explicitly it is still refused, by the manifest this CLI cannot read.
    let other = home(&tmp.path().join("other"));
    let message = install(&other, &index, Some("0.2.0"))
        .unwrap_err()
        .to_string();
    assert!(message.contains("from-a-future-aware"), "{message}");
    assert!(!other.agents_dir().join(AGENT).exists());
}

#[test]
fn a_pinned_update_to_a_release_this_cli_cannot_run_refuses_and_keeps_the_install() {
    let tmp = tempfile::tempdir().unwrap();
    let archive = tmp.path().join("repo.tar.gz");
    write_archive(
        &archive,
        &[
            payload("0.1.0", Some(RUNS_HERE)),
            payload("0.2.0", Some(TOO_NEW)),
        ],
    );
    let index = index_of(&[
        Release {
            version: "0.1.0",
            tarball: &archive,
            index_floor: None,
        },
        Release {
            version: "0.2.0",
            tarball: &archive,
            index_floor: None,
        },
    ]);
    let paths = home(tmp.path());
    install(&paths, &index, Some("0.1.0")).unwrap();
    let sentinel = paths.agents_dir().join(AGENT).join("keep-me.txt");
    std::fs::write(&sentinel, "the installed 0.1.0").unwrap();

    let message = update(&paths, &index, Some("0.2.0"))
        .unwrap_err()
        .to_string();

    assert!(message.contains("E_AGENT_RUNTIME_TOO_OLD"), "{message}");
    assert_eq!(installed_version(&paths), "0.1.0");
    assert_eq!(
        std::fs::read_to_string(&sentinel).unwrap(),
        "the installed 0.1.0"
    );
    assert!(crate::install::swap::leftover_txn_dirs(&paths).is_empty());
}

#[test]
fn store_only_still_refuses_a_release_this_cli_cannot_run_or_whose_floor_drifted() {
    let tmp = tempfile::tempdir().unwrap();
    let archive = tmp.path().join("repo.tar.gz");
    write_archive(
        &archive,
        &[
            payload("0.2.0", Some(TOO_NEW)),
            payload("0.3.0", Some("0.0.2")),
        ],
    );
    let index = index_of(&[
        Release {
            version: "0.2.0",
            tarball: &archive,
            index_floor: None,
        },
        Release {
            version: "0.3.0",
            tarball: &archive,
            index_floor: Some(RUNS_HERE),
        },
    ]);
    let paths = home(tmp.path());

    let too_new =
        crate::install::store_only::store_agent_from_registry(AGENT, "0.2.0", &paths, &index)
            .unwrap_err()
            .to_string();
    assert!(too_new.contains("E_AGENT_RUNTIME_TOO_OLD"), "{too_new}");

    let drifted =
        crate::install::store_only::store_agent_from_registry(AGENT, "0.3.0", &paths, &index)
            .unwrap_err()
            .to_string();
    assert!(drifted.contains("records minimum-cli-version"), "{drifted}");
}

/// An index whose releases carry `bundle-digest`s, read as the fresh official registry.
fn official_index(releases: &[(&str, &Path, &str)]) -> Index {
    let mut versions = serde_json::Map::new();
    for (version, tarball, digest) in releases {
        versions.insert(
            (*version).to_string(),
            serde_json::json!({
                "tarball": format!("file://{}", tarball.display()),
                "subdir": subdir(version),
                "manifest-agent": AGENT,
                "manifest-version": version,
                "bundle-digest": digest,
            }),
        );
    }
    let raw = serde_json::to_vec(&serde_json::json!({
        "version": "1.0", "updated-at": "2026-11-17T00:00:00Z",
        "agents": { AGENT: { "versions": versions } }, "bundles": {}
    }))
    .unwrap();
    let mut index = Index::parse(raw.as_slice()).unwrap();
    index.trust = RegistryTrust::FreshOfficial;
    index
}

/// The `bundle-digest` of a payload holding only `p`'s manifest.
fn digest_of(tmp: &Path, p: Payload) -> String {
    let dir = tmp.join(format!("digest-{}", p.version));
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(dir.join("manifest.yaml"), manifest_text(p)).unwrap();
    crate::install::integrity::tree_digest(&dir).unwrap()
}

#[test]
fn official_trust_a_tampered_newer_release_is_reported_not_passed_over() {
    let tmp = tempfile::tempdir().unwrap();
    let archive = tmp.path().join("repo.tar.gz");
    let old = payload("0.1.0", Some(RUNS_HERE));
    let new = payload("0.2.0", Some(TOO_NEW));
    write_archive(&archive, &[old, new]);
    // 0.2.0's recorded digest is not what its archive holds (and no floor is recorded, so the
    // payload is fetched and judged).
    let index = official_index(&[
        ("0.1.0", &archive, &digest_of(tmp.path(), old)),
        ("0.2.0", &archive, &digest_of(tmp.path(), old)),
    ]);
    let paths = home(tmp.path());

    let message = install(&paths, &index, None).unwrap_err().to_string();

    assert!(message.contains("bundle digest mismatch"), "{message}");
    assert!(message.contains("floor-probe@0.2.0"), "{message}");
    assert!(!paths.agents_dir().join(AGENT).exists());
}

#[test]
fn official_trust_the_receipt_names_the_release_that_was_installed_not_the_one_held_back() {
    let tmp = tempfile::tempdir().unwrap();
    let archive = tmp.path().join("repo.tar.gz");
    let old = payload("0.1.0", Some(RUNS_HERE));
    let new = payload("0.2.0", Some(TOO_NEW));
    write_archive(&archive, &[old, new]);
    let old_digest = digest_of(tmp.path(), old);
    let index = official_index(&[
        ("0.1.0", &archive, &old_digest),
        ("0.2.0", &archive, &digest_of(tmp.path(), new)),
    ]);
    let paths = home(tmp.path());

    let outcome = install(&paths, &index, None).unwrap();

    assert_eq!(outcome.held_back, held("0.2.0", TOO_NEW));
    match crate::install::provenance::read(&paths.agents_dir().join(AGENT)) {
        Some(crate::install::provenance::InstallSource::Registry {
            version,
            manifest_version,
            entry_digest,
            official_source,
            ..
        }) => {
            assert_eq!(version, "0.1.0");
            assert_eq!(manifest_version.as_deref(), Some("0.1.0"));
            assert_eq!(entry_digest.as_deref(), Some(old_digest.as_str()));
            assert!(official_source);
        }
        other => panic!("expected a registry receipt, got {other:?}"),
    }
}

#[test]
fn an_index_floor_that_is_not_strict_semver_is_refused_before_download() {
    let tmp = tempfile::tempdir().unwrap();
    let never_fetched = tmp.path().join("never-fetched.tar.gz");
    let index = index_of(&[Release {
        version: "0.1.0",
        tarball: &never_fetched,
        index_floor: Some("0.180"),
    }]);
    let paths = home(tmp.path());

    let message = install(&paths, &index, None).unwrap_err().to_string();

    assert!(message.contains("floor-probe@0.1.0"), "{message}");
    assert!(message.contains("minimum-cli-version"), "{message}");
    assert!(message.contains("0.180"), "{message}");
}

#[test]
fn a_bare_update_moves_to_the_newest_release_this_cli_runs_and_names_what_it_held_back() {
    let tmp = tempfile::tempdir().unwrap();
    let archive = tmp.path().join("repo.tar.gz");
    write_archive(
        &archive,
        &[
            payload("0.1.0", Some(RUNS_HERE)),
            payload("0.1.5", Some(RUNS_HERE)),
        ],
    );
    let never_fetched = tmp.path().join("never-fetched.tar.gz");
    let index = index_of(&[
        Release {
            version: "0.1.0",
            tarball: &archive,
            index_floor: Some(RUNS_HERE),
        },
        Release {
            version: "0.1.5",
            tarball: &archive,
            index_floor: Some(RUNS_HERE),
        },
        Release {
            version: "0.2.0",
            tarball: &never_fetched,
            index_floor: Some(TOO_NEW),
        },
    ]);
    let paths = home(tmp.path());
    install(&paths, &index, Some("0.1.0")).unwrap();

    let outcome = update(&paths, &index, None).unwrap();

    assert_eq!(outcome.agent, AGENT);
    assert_eq!(outcome.registry_version, "0.1.5");
    assert_eq!(outcome.held_back, held("0.2.0", TOO_NEW));
    assert_eq!(installed_version(&paths), "0.1.5");
}

#[test]
fn a_bare_update_never_moves_an_agent_backwards_to_hold_a_release_back() {
    // 0.2.0 was installed when it still ran here (the CLI has since gone back a release):
    // the newest release this CLI runs is OLDER than the installed one, so `update` leaves it.
    let tmp = tempfile::tempdir().unwrap();
    let archive = tmp.path().join("repo.tar.gz");
    write_archive(
        &archive,
        &[payload("0.1.0", Some(RUNS_HERE)), payload("0.2.0", None)],
    );
    let paths = home(tmp.path());
    let first = index_of(&[Release {
        version: "0.2.0",
        tarball: &archive,
        index_floor: None,
    }]);
    install(&paths, &first, None).unwrap();
    let sentinel = paths.agents_dir().join(AGENT).join("keep-me.txt");
    std::fs::write(&sentinel, "the installed 0.2.0").unwrap();

    let never_fetched = tmp.path().join("never-fetched.tar.gz");
    let now = index_of(&[
        Release {
            version: "0.1.0",
            tarball: &archive,
            index_floor: Some(RUNS_HERE),
        },
        Release {
            version: "0.2.0",
            tarball: &never_fetched,
            index_floor: Some(TOO_NEW),
        },
    ]);
    let message = update(&paths, &now, None).unwrap_err().to_string();

    assert!(
        message.contains("E_AGENT_UPDATE_WOULD_DOWNGRADE"),
        "{message}"
    );
    assert!(message.contains("0.2.0 is installed"), "{message}");
    assert!(message.contains("0.1.0"), "{message}");
    assert!(
        message.contains("aware agent update floor-probe@0.1.0"),
        "the refusal names the exact pinned route: {message}"
    );
    assert_eq!(
        std::fs::read_to_string(&sentinel).unwrap(),
        "the installed 0.2.0"
    );
    assert_eq!(installed_version(&paths), "0.2.0");
    assert!(crate::install::swap::leftover_txn_dirs(&paths).is_empty());

    // The pinned route it names does move it, deliberately.
    let outcome = update(&paths, &now, Some("0.1.0")).unwrap();
    assert_eq!(outcome.held_back, None);
    assert_eq!(installed_version(&paths), "0.1.0");
}

#[test]
fn a_bundle_keeps_its_specs_and_reports_what_each_bare_member_held_back() {
    let tmp = tempfile::tempdir().unwrap();
    let archive = tmp.path().join("old.tar.gz");
    write_archive(&archive, &[payload("0.1.0", Some(RUNS_HERE))]);
    let never_fetched = tmp.path().join("never-fetched.tar.gz");
    let mut index = index_of(&[
        Release {
            version: "0.1.0",
            tarball: &archive,
            index_floor: Some(RUNS_HERE),
        },
        Release {
            version: "0.2.0",
            tarball: &never_fetched,
            index_floor: Some(TOO_NEW),
        },
    ]);
    index.bundles.insert(
        "floor-bundle".into(),
        crate::registry::index::BundleEntry {
            description: "One bare member".into(),
            agents: vec![AGENT.into()],
        },
    );
    let paths = home(tmp.path());

    let report = crate::install::install_bundle(
        "floor-bundle",
        &paths,
        &index,
        &crate::agent_store::open(&paths).unwrap(),
    )
    .unwrap();

    assert_eq!(report.installed, vec![AGENT.to_string()]);
    assert!(report.failed.is_empty(), "{:?}", report.failed);
    assert_eq!(
        report.held_back,
        vec![(
            AGENT.to_string(),
            RegistryOutcome {
                agent: AGENT.into(),
                registry_version: "0.1.0".into(),
                held_back: held("0.2.0", TOO_NEW),
            }
        )]
    );
}

#[test]
fn a_bare_update_judges_a_copy_it_cannot_parse_by_its_receipt_and_refuses_without_one() {
    // 0.2.0 was installed by a newer AWARE, in manifest syntax this CLI cannot parse: exactly
    // the copy a step back would replace without the receipt saying which version it is.
    let tmp = tempfile::tempdir().unwrap();
    let archive = tmp.path().join("repo.tar.gz");
    write_archive(
        &archive,
        &[payload("0.1.0", Some(RUNS_HERE)), payload("0.2.0", None)],
    );
    let paths = home(tmp.path());
    let first = index_of(&[Release {
        version: "0.2.0",
        tarball: &archive,
        index_floor: None,
    }]);
    install(&paths, &first, None).unwrap();
    let root = paths.agents_dir().join(AGENT);
    let future = manifest_text(payload("0.2.0", Some(TOO_NEW)))
        .replace("status: requires-runtime", "status: from-a-future-aware");
    std::fs::write(root.join("manifest.yaml"), &future).unwrap();

    let never_fetched = tmp.path().join("never-fetched.tar.gz");
    let now = index_of(&[
        Release {
            version: "0.1.0",
            tarball: &archive,
            index_floor: Some(RUNS_HERE),
        },
        Release {
            version: "0.2.0",
            tarball: &never_fetched,
            index_floor: Some(TOO_NEW),
        },
    ]);

    // The receipt still names 0.2.0.
    let message = update(&paths, &now, None).unwrap_err().to_string();
    assert!(
        message.contains("E_AGENT_UPDATE_WOULD_DOWNGRADE"),
        "{message}"
    );
    assert!(message.contains("0.2.0 is installed"), "{message}");
    assert_eq!(
        std::fs::read_to_string(root.join("manifest.yaml")).unwrap(),
        future
    );

    // With no receipt there is nothing to judge it by: refused rather than guessed (`--force`
    // gets past the local-install guard, not this one).
    std::fs::remove_file(root.join(crate::install::provenance::FILE)).unwrap();
    let message = update_agent_from_registry(
        AGENT,
        None,
        true,
        &paths,
        &now,
        &crate::agent_store::open(&paths).unwrap(),
    )
    .unwrap_err()
    .to_string();
    assert!(
        message.contains("E_AGENT_UPDATE_WOULD_DOWNGRADE"),
        "{message}"
    );
    assert!(message.contains("cannot read"), "{message}");
    assert_eq!(
        std::fs::read_to_string(root.join("manifest.yaml")).unwrap(),
        future
    );
    assert!(crate::install::swap::leftover_txn_dirs(&paths).is_empty());
}
