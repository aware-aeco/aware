//! `aware key ...` and `aware receipt ...` — the Stamped Receipt command surface.
//!
//! Both modules were at 0% coverage: `src/commands/key.rs` and
//! `src/commands/receipt_cli.rs` had no test of any kind, unit or integration,
//! so nothing exercised the argument wiring, the `AWARE_HOME`-relative key
//! lookup, the exit codes, or a single line of the printed output. The crypto
//! underneath them is unit-tested in `src/receipt.rs`; this file covers the
//! commands an operator, insurer or building-control officer actually types.
//!
//! Exit codes are asserted rather than bare success/failure: `cli-spec.md`
//! makes 3 (validation) and 7 (not-found) part of the contract, and a test that
//! only checked `.failure()` would not notice the two being swapped.

use assert_cmd::Command;
use predicates::prelude::*;

/// An `AWARE_HOME` under a fresh temp dir. Returned `TempDir` keeps it alive.
fn home() -> (tempfile::TempDir, std::path::PathBuf) {
    let tmp = tempfile::tempdir().unwrap();
    let home = tmp.path().join("home");
    std::fs::create_dir_all(&home).unwrap();
    (tmp, home)
}

fn aware(home: &std::path::Path) -> Command {
    let mut c = Command::cargo_bin("aware").unwrap();
    c.env("AWARE_HOME", home);
    c
}

fn generate(home: &std::path::Path, operator: &str) {
    aware(home)
        .args(["key", "generate", operator])
        .assert()
        .success();
}

/// Write a receipt and sign it as `operator`, returning the receipt path.
fn signed_receipt(
    home: &std::path::Path,
    dir: &std::path::Path,
    operator: &str,
    body: &str,
) -> std::path::PathBuf {
    let receipt = dir.join("run.jsonl");
    std::fs::write(&receipt, body).unwrap();
    aware(home)
        .args(["receipt", "sign"])
        .arg(&receipt)
        .args(["--operator", operator])
        .assert()
        .success();
    receipt
}

// ── aware key ────────────────────────────────────────────────────────────────

#[test]
fn key_generate_writes_both_halves_under_aware_home() {
    let (_tmp, home) = home();
    aware(&home)
        .args(["key", "generate", "acme-structures"])
        .assert()
        .success()
        .stdout(predicate::str::contains("acme-structures"));

    // The point of the command is the files, not the message.
    let sec = home.join("keys/acme-structures.sec");
    let pub_ = home.join("keys/acme-structures.pub");
    assert!(sec.is_file(), "no secret key at {}", sec.display());
    assert!(pub_.is_file(), "no public key at {}", pub_.display());
    assert!(
        std::fs::read_to_string(&sec)
            .unwrap()
            .starts_with("ed25519-secret-key-v1 ")
    );
    assert!(
        std::fs::read_to_string(&pub_)
            .unwrap()
            .starts_with("ed25519-public-key-v1 ")
    );
}

#[test]
fn key_generate_refuses_to_clobber_an_existing_operator() {
    let (_tmp, home) = home();
    generate(&home, "acme-structures");
    let before = std::fs::read_to_string(home.join("keys/acme-structures.sec")).unwrap();

    aware(&home)
        .args(["key", "generate", "acme-structures"])
        .assert()
        .failure()
        .code(3);

    // The refusal has to be the whole story: a message plus a rewritten key
    // would be worse than no guard at all.
    let after = std::fs::read_to_string(home.join("keys/acme-structures.sec")).unwrap();
    assert_eq!(before, after, "the existing secret key was rewritten");
}

#[test]
fn key_list_says_so_when_nothing_is_installed() {
    let (_tmp, home) = home();
    aware(&home)
        .args(["key", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("(no keys installed)"));
}

#[test]
fn key_list_reports_which_halves_of_each_pair_survive() {
    let (_tmp, home) = home();
    generate(&home, "beta-operator");
    generate(&home, "alpha-operator");
    // An operator who published a key and destroyed the secret: still listed,
    // but only as `pub`. This is the branch that decides whether they can still
    // sign, so it is the one worth rendering correctly.
    std::fs::remove_file(home.join("keys/beta-operator.sec")).unwrap();

    let out = aware(&home).args(["key", "list"]).assert().success();
    let stdout = String::from_utf8(out.get_output().stdout.clone()).unwrap();

    let alpha = stdout
        .lines()
        .find(|l| l.starts_with("alpha-operator"))
        .unwrap_or_else(|| panic!("alpha-operator missing from:\n{stdout}"));
    let beta = stdout
        .lines()
        .find(|l| l.starts_with("beta-operator"))
        .unwrap_or_else(|| panic!("beta-operator missing from:\n{stdout}"));
    assert!(alpha.contains("pub + sec"), "got {alpha:?}");
    assert!(beta.contains("pub"), "got {beta:?}");
    assert!(
        !beta.contains("sec"),
        "beta has no .sec but claims one: {beta:?}"
    );

    // Listed in sorted order, not readdir order, so successive runs agree.
    assert!(
        stdout.find("alpha-operator") < stdout.find("beta-operator"),
        "operators are not sorted:\n{stdout}"
    );
}

#[test]
fn key_show_prints_the_public_half_and_never_the_secret() {
    let (_tmp, home) = home();
    generate(&home, "acme-structures");
    let secret_body = std::fs::read_to_string(home.join("keys/acme-structures.sec")).unwrap();
    let secret_b64 = secret_body.split_whitespace().nth(1).unwrap().to_string();

    let out = aware(&home)
        .args(["key", "show", "acme-structures"])
        .assert()
        .success();
    let stdout = String::from_utf8(out.get_output().stdout.clone()).unwrap();

    assert!(
        stdout.contains("ed25519-public-key-v1"),
        "no public key in:\n{stdout}"
    );
    // `show` exists to be pasted into an email to a verifier. Leaking the
    // private half there would be unrecoverable.
    assert!(
        !stdout.contains(&secret_b64),
        "the SECRET key was printed by `key show`"
    );
}

#[test]
fn key_show_on_an_unknown_operator_is_not_found() {
    let (_tmp, home) = home();
    generate(&home, "acme-structures");
    aware(&home)
        .args(["key", "show", "nobody"])
        .assert()
        .failure()
        .code(7);
}

// ── aware receipt ────────────────────────────────────────────────────────────

#[test]
fn receipt_sign_then_verify_round_trips_through_the_cli() {
    let (tmp, home) = home();
    generate(&home, "acme-structures");
    let receipt = signed_receipt(
        &home,
        tmp.path(),
        "acme-structures",
        "{\"verdict\":\"PASS\"}\n",
    );

    assert!(
        tmp.path().join("run.jsonl.sig").is_file(),
        "no sidecar beside the receipt"
    );
    // No `--sig`: this also pins that `receipt verify` derives the same sidecar
    // path that `receipt sign` wrote to. The two are computed by different code
    // in different modules and nothing else checks that they agree.
    aware(&home)
        .args(["receipt", "verify"])
        .arg(&receipt)
        .assert()
        .success()
        .stdout(predicate::str::contains("signature verifies"));
}

#[test]
fn receipt_verify_rejects_a_receipt_edited_after_signing() {
    let (tmp, home) = home();
    generate(&home, "acme-structures");
    let receipt = signed_receipt(
        &home,
        tmp.path(),
        "acme-structures",
        "{\"verdict\":\"PASS\",\"udl-kn-m\":12.5}\n",
    );

    // The single edit the whole subsystem exists to catch.
    std::fs::write(&receipt, "{\"verdict\":\"PASS\",\"udl-kn-m\":99.9}\n").unwrap();

    aware(&home)
        .args(["receipt", "verify"])
        .arg(&receipt)
        .assert()
        .failure()
        .code(3);
}

#[test]
fn receipt_verify_honours_an_explicit_sig_path() {
    let (tmp, home) = home();
    generate(&home, "acme-structures");
    let receipt = signed_receipt(
        &home,
        tmp.path(),
        "acme-structures",
        "{\"verdict\":\"PASS\"}\n",
    );

    // Move the sidecar somewhere `default_sig_path` would never look, so a
    // `--sig` that was parsed but ignored fails instead of passing by luck.
    let moved = tmp.path().join("elsewhere.sig");
    std::fs::rename(tmp.path().join("run.jsonl.sig"), &moved).unwrap();

    aware(&home)
        .args(["receipt", "verify"])
        .arg(&receipt)
        .assert()
        .failure()
        .code(7);

    aware(&home)
        .args(["receipt", "verify"])
        .arg(&receipt)
        .arg("--sig")
        .arg(&moved)
        .assert()
        .success()
        .stdout(predicate::str::contains("signature verifies"));
}

#[test]
fn receipt_sign_with_an_unknown_operator_names_the_key_it_wanted() {
    let (tmp, home) = home();
    let receipt = tmp.path().join("run.jsonl");
    std::fs::write(&receipt, "{}\n").unwrap();

    let out = aware(&home)
        .args(["receipt", "sign"])
        .arg(&receipt)
        .args(["--operator", "nobody"])
        .assert()
        .failure()
        .code(7);
    let stderr = String::from_utf8(out.get_output().stderr.clone()).unwrap();
    // Naming the path is the whole remedy — it tells the operator which key to
    // generate, and where AWARE looked.
    assert!(stderr.contains("nobody.sec"), "got {stderr}");
    assert!(
        !tmp.path().join("run.jsonl.sig").exists(),
        "a sidecar was written despite the failure"
    );
}

#[test]
fn receipt_show_prints_the_receipt_and_its_sidecar() {
    let (tmp, home) = home();
    generate(&home, "acme-structures");
    let receipt = signed_receipt(
        &home,
        tmp.path(),
        "acme-structures",
        "{\"panel\":\"structural\",\"verdict\":\"PASS\"}\n",
    );

    aware(&home)
        .args(["receipt", "show"])
        .arg(&receipt)
        .assert()
        .success()
        .stdout(predicate::str::contains("\"panel\":\"structural\""))
        .stdout(predicate::str::contains("ed25519-signature-v1"))
        .stdout(predicate::str::contains("over-sha256-of: run.jsonl"));
}

#[test]
fn receipt_show_says_when_a_receipt_is_unsigned() {
    let (tmp, home) = home();
    let receipt = tmp.path().join("run.jsonl");
    std::fs::write(&receipt, "{\"verdict\":\"PASS\"}\n").unwrap();

    // An unsigned receipt is not an error — but the absence has to be stated,
    // or an unsigned artefact reads exactly like a signed one.
    aware(&home)
        .args(["receipt", "show"])
        .arg(&receipt)
        .assert()
        .success()
        .stdout(predicate::str::contains("no .sig sidecar"));
}

#[test]
fn receipt_show_on_a_missing_file_is_not_found() {
    let (tmp, home) = home();
    aware(&home)
        .args(["receipt", "show"])
        .arg(tmp.path().join("absent.jsonl"))
        .assert()
        .failure()
        .code(7);
}

// ── characterisation: two gaps this sweep found ──────────────────────────────
//
// Both tests below pin behaviour that is WRONG. They are here because the
// behaviour is undefended either way today, and a silent hole is worse than a
// documented one: if you are here because a test went red, you have fixed one
// of these, and the fix is the change — rewrite the test to match it.

#[test]
fn characterisation_verify_trusts_whatever_key_the_sidecar_carries() {
    // `verify_receipt` reads the public key OUT OF the sidecar it is checking,
    // and `aware receipt verify` never prints or pins that key. So anyone
    // holding any keypair can edit a receipt, re-sign it, and the command
    // prints exactly the same unqualified "✓ signature verifies" — the
    // verification an insurer or building-control officer is relying on cannot
    // distinguish the signer. Fixing it needs a trusted-key input (`--operator`
    // / `--pub`) or, at minimum, printing the signing key so it can be compared
    // against the published one. That is a product decision, so this sweep only
    // records it.
    let (tmp, home) = home();
    generate(&home, "honest-engineer");
    generate(&home, "forger");
    let receipt = signed_receipt(
        &home,
        tmp.path(),
        "honest-engineer",
        "{\"verdict\":\"PASS\",\"udl-kn-m\":12.5}\n",
    );

    let honest = aware(&home)
        .args(["receipt", "verify"])
        .arg(&receipt)
        .assert()
        .success();
    let honest_stdout = String::from_utf8(honest.get_output().stdout.clone()).unwrap();

    // Tamper, then re-sign under a key the verifier has never heard of.
    std::fs::write(&receipt, "{\"verdict\":\"PASS\",\"udl-kn-m\":99.9}\n").unwrap();
    aware(&home)
        .args(["receipt", "sign"])
        .arg(&receipt)
        .args(["--operator", "forger"])
        .assert()
        .success();

    let forged = aware(&home)
        .args(["receipt", "verify"])
        .arg(&receipt)
        .assert()
        .success();
    let forged_stdout = String::from_utf8(forged.get_output().stdout.clone()).unwrap();

    assert_eq!(
        honest_stdout, forged_stdout,
        "`receipt verify` now distinguishes the signer — good; update this test"
    );
}

#[test]
fn characterisation_key_generate_does_not_fence_the_operator_id() {
    // An operator id is joined onto `<AWARE_HOME>/keys/` unchecked, so it can
    // traverse out of it — here the PRIVATE key lands two levels above
    // AWARE_HOME entirely. The repo already has the fence for exactly this
    // shape (`manifest::loader::is_safe_segment`, guarded by
    // `tests/agent_id_joins_are_fenced.rs` and `tests/app_id_is_a_segment.rs`);
    // `key` and `receipt --operator` were never routed through it.
    let (tmp, home) = home();
    aware(&home)
        .args(["key", "generate", "../../escaped"])
        .assert()
        .success();

    let escaped = tmp.path().join("escaped.sec");
    assert!(
        escaped.is_file(),
        "the id is fenced now — good; update this test"
    );
    assert!(
        !home.join("keys").join("escaped.sec").exists(),
        "the key stayed inside the keys directory"
    );
}
