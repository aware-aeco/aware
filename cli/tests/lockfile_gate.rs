//! Negative control for the lockfile gate in `.github/workflows/ci.yml`.
//!
//! `rust-toolchain.toml` pins the compiler; `Cargo.lock` pins everything the
//! compiler is handed. Only the first was ever checked. Every cargo invocation
//! in CI resolved dependencies without `--locked`, so a lockfile disagreeing
//! with its manifest was rewritten inside the runner and the job went green on a
//! tree that does not build reproducibly.
//!
//! It drifted: the v0.123.0 release commit bumped `cli/Cargo.toml` and left
//! `cli/Cargo.lock` recording `aware 0.122.0`, so `cargo build --locked` and
//! `cargo install --locked` — the reproducible install CLAUDE.md §Build / run /
//! test documents — failed on `main` while all four workflows reported success.
//! Earlier release commits carried a regenerated lock only because somebody had
//! built locally first; nothing required one.
//!
//! **This gate deliberately does not live in `cargo test`, and the reason is
//! measured below.** Cargo repairs a stale lock as a silent side effect of any
//! command that resolves dependencies — `cargo test` included — so a test that
//! reads `Cargo.lock` reads the copy cargo just rewrote and passes against the
//! exact drift it was written to catch. That was tried first, on the real
//! violation, and it passed; [`cargo_repairs_a_stale_lock_while_building`] pins
//! the behaviour that made it useless so nobody reinstates the idea.
//!
//! What is left here is what a test *can* prove:
//!   * `cargo metadata --locked` really does reject a stale lock and accept a
//!     fresh one — the mechanism CI runs, measured rather than assumed;
//!   * cargo really does erase the evidence, which is what makes the step's
//!     position in the workflow load-bearing;
//!   * `ci.yml` still carries the check, still names both crates, and still runs
//!     it before anything that would repair the lock — without this, deleting or
//!     merely *moving* the step leaves both assertions above green.

use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// Repository root — `cli/`'s parent. The workflow under test lives there.
fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap_or_else(|| panic!("{} has no parent", env!("CARGO_MANIFEST_DIR")))
        .to_path_buf()
}

/// A scratch crate with a generated lockfile, at `version`.
///
/// It declares NO dependencies, which is what makes `--offline` sound in these
/// probes: with nothing to resolve, cargo never reaches for the registry index,
/// so they cannot fail for a cold cache the way the same calls against a real
/// crate do (`--locked --offline` against `cli/` fails with *"attempting to make
/// an HTTP request"* until something has warmed the index — which is why the
/// full resolve stays in CI, where the network is).
fn probe_crate(version: &str) -> tempfile::TempDir {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path();
    std::fs::create_dir_all(root.join("src")).expect("src dir");
    std::fs::write(root.join("Cargo.toml"), manifest(version)).expect("write manifest");
    std::fs::write(root.join("src/main.rs"), "fn main() {}\n").expect("write main");

    let generated = Command::new("cargo")
        .args(["generate-lockfile", "--offline"])
        .current_dir(root)
        .output()
        .expect("run cargo generate-lockfile");
    assert!(
        generated.status.success(),
        "could not generate the probe's lockfile, so nothing below proves anything:\n{}",
        String::from_utf8_lossy(&generated.stderr)
    );
    dir
}

/// The probe's manifest, at `version`.
///
/// `edition = "2021"`, deliberately, even though CLAUDE.md §Tech stack pins this
/// repo at 2024. The probe is built by a cargo running in a TEMPDIR, and a
/// `rust-toolchain.toml` override applies only within its own directory tree —
/// so the child process gets rustup's *default* toolchain, not `cli/`'s pinned
/// 1.95. On a developer whose default is older than 1.85, an edition-2024
/// manifest fails to parse and the probe fails for a reason that has nothing to
/// do with lockfiles (Codex review, PR #416). Nothing here is edition-sensitive:
/// the probe declares no dependencies and is never compiled — `generate-lockfile`
/// and `metadata` only resolve — so the oldest widely-available edition is the
/// one that couples this test to the least.
fn manifest(version: &str) -> String {
    format!(
        "[package]\nname = \"lock_probe\"\nversion = \"{version}\"\nedition = \"2021\"\n\n[workspace]\n"
    )
}

/// The version a probe crate's lockfile records for its one package.
///
/// Scanned from the `[[package]]` header onwards, not from the top of the file:
/// a lockfile opens with its own format `version` (`version = 4`), so taking the
/// first `version =` line reads the schema number rather than the package's.
fn locked_version(root: &Path) -> String {
    let lock =
        std::fs::read_to_string(root.join("Cargo.lock")).expect("read the probe's Cargo.lock");
    lock.lines()
        .map(str::trim)
        .skip_while(|line| *line != "[[package]]")
        .find_map(|line| line.strip_prefix("version = "))
        .map(|rest| rest.trim_matches('"').to_owned())
        .unwrap_or_else(|| panic!("the probe's Cargo.lock records no package version:\n{lock}"))
}

/// Run `cargo metadata --locked --offline` in `dir`; `true` when it accepts.
fn metadata_accepts(dir: &Path) -> (bool, String) {
    let out = Command::new("cargo")
        .args(["metadata", "--locked", "--offline", "--format-version", "1"])
        .current_dir(dir)
        // On success `cargo metadata` prints the whole graph; none of it is
        // wanted, and inheriting the pipe risks filling it.
        .stdout(Stdio::null())
        .output()
        .unwrap_or_else(|e| panic!("run cargo metadata: {e}"));
    (
        out.status.success(),
        String::from_utf8_lossy(&out.stderr).into_owned(),
    )
}

/// The mechanism CI runs, driven over a crate whose lock is known-fresh and then
/// known-stale.
///
/// Both halves are needed. The rejection alone would pass just as well against a
/// `cargo metadata` that fails for any reason at all — a malformed manifest, a
/// missing toolchain — which would make a dead gate look enforced; the
/// acceptance first proves the probe is sound.
#[test]
fn locked_rejects_a_stale_lock_and_accepts_a_fresh_one() {
    let dir = probe_crate("0.2.0");
    let root = dir.path();

    // Positive control: manifest and lock agree.
    let (accepted, stderr) = metadata_accepts(root);
    assert!(
        accepted,
        "`--locked` rejected a freshly generated lockfile — the probe is broken, \
         not the gate:\n{stderr}"
    );

    // Now the drift that shipped: bump the manifest, leave the lock behind.
    std::fs::write(root.join("Cargo.toml"), manifest("0.3.0")).expect("rewrite manifest");
    let (accepted, stderr) = metadata_accepts(root);
    assert!(
        !accepted,
        "`--locked` accepted a lockfile recording a version its manifest no \
         longer declares — the gate CI relies on is not enforcing"
    );
    assert!(
        stderr.contains("--locked"),
        "`--locked` rejected the stale lock, but not for the lock — the probe may \
         be failing for an unrelated reason. Stderr:\n{stderr}"
    );
}

/// Why the check above cannot be applied to this repo's own lockfile from
/// inside `cargo test`.
///
/// Cargo silently rewrites a stale `Cargo.lock` as a side effect of resolving,
/// and building a test binary resolves. So by the time any `#[test]` runs, the
/// working tree's lock has already been repaired and a comparison against the
/// manifest reads two values that now agree — which is precisely what happened
/// when this was first written that way and run against the real v0.123.0
/// drift: green, on a tree where `cargo build --locked` failed.
///
/// Measured on a probe rather than argued, because it is the entire reason the
/// gate is a workflow step placed ahead of the cache and every build. If a
/// future cargo stops repairing locks, this fails and the design can be
/// revisited on evidence.
#[test]
fn cargo_repairs_a_stale_lock_while_building() {
    let dir = probe_crate("0.2.0");
    let root = dir.path();
    assert_eq!(
        locked_version(root),
        "0.2.0",
        "the probe did not start in the state this test needs"
    );

    // Exactly the drift that shipped: manifest moves, lock does not.
    std::fs::write(root.join("Cargo.toml"), manifest("0.3.0")).expect("rewrite manifest");

    // Any resolving command will do; `metadata` is the cheapest and is what
    // `Swatinem/rust-cache` itself shells out to before any build step runs.
    let out = Command::new("cargo")
        .args(["metadata", "--offline", "--format-version", "1"])
        .current_dir(root)
        .stdout(Stdio::null())
        .output()
        .expect("run cargo metadata");
    assert!(
        out.status.success(),
        "the probe's resolve failed, so this measures nothing:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );

    assert_eq!(
        locked_version(root),
        "0.3.0",
        "cargo no longer repairs a stale lockfile in place. That is the behaviour \
         `.github/workflows/ci.yml` orders its lockfile step around, and the \
         reason this gate is not a test — if it has genuinely changed, the step \
         could be simplified, but do not assume it from this message alone"
    );
}

/// One workflow step: its `name:` and its body with comment lines removed.
struct Step {
    name: String,
    body: String,
}

/// Split a workflow's steps in order.
///
/// Scoped to the step, not the file, and that is the point. A `contains` over
/// the whole of `ci.yml` passes for the wrong reason at least three ways here,
/// each measured against this very change: `cargo test --locked` appears in two
/// different steps, so dropping it from one leaves the string present;
/// `20-agents/…/steel-detailer-lookup` appears in the cache list and in its own
/// job, so dropping it from the lockfile loop leaves the string present; and the
/// step's own explanatory comment names `cargo metadata --locked`, so deleting
/// the command leaves the prose describing it. Comment lines are therefore
/// dropped, and every assertion below names the step it applies to.
fn steps(workflow: &str) -> Vec<Step> {
    workflow
        .split("\n      - ")
        .skip(1) // the job header, before the first step
        .map(|chunk| Step {
            name: chunk
                .lines()
                .next()
                .and_then(|first| first.strip_prefix("name: "))
                .unwrap_or_default() // a bare `uses:` step has no name
                .trim()
                .to_string(),
            body: chunk
                .lines()
                .filter(|line| !line.trim_start().starts_with('#'))
                .collect::<Vec<_>>()
                .join("\n"),
        })
        .collect()
}

/// The index of the step called `name`, or a failure naming what was searched.
fn step_index(steps: &[Step], name: &str) -> usize {
    steps
        .iter()
        .position(|step| step.name == name)
        .unwrap_or_else(|| {
            panic!(
                "ci.yml has no step named {name:?} — it was renamed or removed, and \
                 this test can no longer tell whether its gate still runs. Steps \
                 present: {:?}",
                steps.iter().map(|s| &s.name).collect::<Vec<_>>()
            )
        })
}

/// The tests above prove the mechanism works and that cargo hides the evidence.
/// Neither notices if CI stops running the check — so this anchors the step.
///
/// Both crate directories, not just this one, because the whole finding was a
/// lockfile nobody's gate reached: `steel-detailer-lookup` is the repo's other
/// Rust binary and ships in the same archive. CI is its only coverage, and this
/// is what keeps CI honest about it.
#[test]
fn ci_still_checks_both_lockfiles_before_anything_repairs_them() {
    let workflow = repo_root().join(".github/workflows/ci.yml");
    let source = std::fs::read_to_string(&workflow)
        .unwrap_or_else(|e| panic!("read {}: {e}", workflow.display()));
    let steps = steps(&source);

    // A floor, not an is-empty check: a splitter that recovered one step would
    // pass an emptiness test having silently dropped the rest, and every
    // `step_index` below would then fail for the wrong reason.
    assert!(
        steps.len() > 8,
        "parsed only {} steps out of ci.yml — the splitter is broken, so nothing \
         below means anything",
        steps.len()
    );

    let check = step_index(&steps, "lockfiles agree with their manifests");
    assert!(
        steps[check].body.contains("cargo metadata --locked"),
        "the lockfile step no longer runs `cargo metadata --locked`, so a lockfile \
         disagreeing with its manifest would be silently rewritten in the runner \
         and reported green"
    );
    for crate_dir in ["cli", "20-agents/aeco/engineering/steel-detailer-lookup"] {
        assert!(
            steps[check].body.contains(crate_dir),
            "the lockfile step no longer names {crate_dir}, so its Cargo.lock is \
             unchecked — CI is that crate's only coverage for this"
        );
    }

    // Position, not just presence. `Swatinem/rust-cache` shells out to `cargo
    // metadata` for its cache key, so a check that runs after it inspects a lock
    // cargo has already repaired — the step would still be there, still be
    // green, and prove nothing.
    assert!(
        check < step_index(&steps, "Cache cargo registry + build"),
        "the lockfile check now runs AFTER the cache step, which resolves \
         dependencies and repairs a stale lock before the check can see it. Move \
         it back above `Swatinem/rust-cache`."
    );

    // `--locked` on the build steps is the belt to that check's braces: it makes
    // the enforcement independent of step order, so a future reshuffle degrades
    // the error message rather than the gate. Named per step, because both crates
    // run a `cargo test` and only one of them matching would otherwise satisfy a
    // whole-file search.
    for (step_name, expected) in [
        (
            "cargo clippy -D warnings",
            &["cargo clippy --all-targets --locked -- -D warnings"][..],
        ),
        ("cargo test", &["cargo test --locked"][..]),
        (
            "steel-detailer-lookup — fmt + clippy + test",
            &[
                "cargo clippy --all-targets --locked -- -D warnings",
                "cargo test --locked",
            ][..],
        ),
    ] {
        let step = &steps[step_index(&steps, step_name)];
        for command in expected {
            assert!(
                step.body.contains(command),
                "ci.yml step {step_name:?} no longer runs `{command}` — dropping \
                 `--locked` lets a build step quietly resolve around a stale \
                 lockfile"
            );
        }
    }
}
