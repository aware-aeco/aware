//! Stamped Receipt — v0.27.
//!
//! **ed25519-signed JSONL receipts.** Every panel-gated run produces a
//! `.jsonl` receipt + a `.sig` sidecar containing an ed25519 signature over
//! the receipt's SHA-256. The operator's keypair is loaded from
//! `~/.aware/keys/<operator-id>.{pub,sec}`. An insurer, building-control
//! officer, or PE-stamping engineer can verify the receipt independently
//! against the published public key.
//!
//! Surfaced by `aware key` + `aware receipt`.

use std::path::{Path, PathBuf};

use base64::Engine;
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use sha2::{Digest, Sha256};

use crate::error::AwareError;

// ---- ed25519 key management ----

/// Generate a fresh ed25519 keypair + persist it to
/// `~/.aware/keys/<operator-id>.{pub,sec}`. Used by `aware key generate`.
pub fn generate_keypair(keys_dir: &Path, operator_id: &str) -> Result<KeyPaths, AwareError> {
    use rand_core::OsRng;

    std::fs::create_dir_all(keys_dir)
        .map_err(|e| AwareError::Internal(format!("create {}: {e}", keys_dir.display())))?;

    let mut csprng = OsRng;
    let signing = SigningKey::generate(&mut csprng);
    let verifying = signing.verifying_key();

    let sec_path = keys_dir.join(format!("{operator_id}.sec"));
    let pub_path = keys_dir.join(format!("{operator_id}.pub"));

    if sec_path.exists() {
        return Err(AwareError::Validation(format!(
            "key already exists at {} — refusing to overwrite; remove manually first",
            sec_path.display()
        )));
    }

    // Base64-encode for portability.
    let sec_b64 = base64::engine::general_purpose::STANDARD.encode(signing.to_bytes());
    let pub_b64 = base64::engine::general_purpose::STANDARD.encode(verifying.to_bytes());

    std::fs::write(&sec_path, format!("ed25519-secret-key-v1 {sec_b64}\n"))
        .map_err(|e| AwareError::Internal(format!("write {}: {e}", sec_path.display())))?;
    std::fs::write(&pub_path, format!("ed25519-public-key-v1 {pub_b64}\n"))
        .map_err(|e| AwareError::Internal(format!("write {}: {e}", pub_path.display())))?;

    // Permissions: best-effort 0600 on Unix.
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let _ = std::fs::set_permissions(&sec_path, std::fs::Permissions::from_mode(0o600));
    }

    Ok(KeyPaths { sec_path, pub_path })
}

#[derive(Debug)]
pub struct KeyPaths {
    pub sec_path: PathBuf,
    pub pub_path: PathBuf,
}

/// Load an ed25519 signing key (private) from disk.
pub fn load_signing_key(sec_path: &Path) -> Result<SigningKey, AwareError> {
    let raw = std::fs::read_to_string(sec_path)
        .map_err(|e| AwareError::NotFound(format!("{}: {e}", sec_path.display())))?;
    let parts: Vec<&str> = raw.split_whitespace().collect();
    if parts.len() != 2 || parts[0] != "ed25519-secret-key-v1" {
        return Err(AwareError::Validation(format!(
            "{} is not a valid ed25519-secret-key-v1 file",
            sec_path.display()
        )));
    }
    let bytes = base64::engine::general_purpose::STANDARD
        .decode(parts[1])
        .map_err(|e| AwareError::Validation(format!("base64 decode: {e}")))?;
    let arr: [u8; 32] = bytes
        .as_slice()
        .try_into()
        .map_err(|_| AwareError::Validation("secret key must be 32 bytes".into()))?;
    Ok(SigningKey::from_bytes(&arr))
}

/// Load an ed25519 verifying key (public) from disk. `verify_receipt` parses the
/// key inline, so only the tests reach this today.
#[allow(dead_code)]
pub fn load_verifying_key(pub_path: &Path) -> Result<VerifyingKey, AwareError> {
    let raw = std::fs::read_to_string(pub_path)
        .map_err(|e| AwareError::NotFound(format!("{}: {e}", pub_path.display())))?;
    let parts: Vec<&str> = raw.split_whitespace().collect();
    if parts.len() != 2 || parts[0] != "ed25519-public-key-v1" {
        return Err(AwareError::Validation(format!(
            "{} is not a valid ed25519-public-key-v1 file",
            pub_path.display()
        )));
    }
    let bytes = base64::engine::general_purpose::STANDARD
        .decode(parts[1])
        .map_err(|e| AwareError::Validation(format!("base64 decode: {e}")))?;
    let arr: [u8; 32] = bytes
        .as_slice()
        .try_into()
        .map_err(|_| AwareError::Validation("public key must be 32 bytes".into()))?;
    VerifyingKey::from_bytes(&arr)
        .map_err(|e| AwareError::Validation(format!("verifying key: {e}")))
}

// ---- Receipt signing / verification ----

/// Sign a receipt file. The signature is taken over the SHA-256 of the
/// receipt bytes; the signature sidecar is written next to the receipt
/// as `<receipt>.sig`.
pub fn sign_receipt(receipt_path: &Path, signing_key: &SigningKey) -> Result<PathBuf, AwareError> {
    let bytes = std::fs::read(receipt_path)
        .map_err(|e| AwareError::NotFound(format!("{}: {e}", receipt_path.display())))?;
    let digest = Sha256::digest(&bytes);
    let signature = signing_key.sign(&digest);
    let sig_b64 = base64::engine::general_purpose::STANDARD.encode(signature.to_bytes());

    let sig_path = receipt_path.with_extension(
        format!(
            "{}.sig",
            receipt_path
                .extension()
                .and_then(|e| e.to_str())
                .unwrap_or("")
        )
        .trim_start_matches('.'),
    );
    let pub_b64 =
        base64::engine::general_purpose::STANDARD.encode(signing_key.verifying_key().to_bytes());
    let body = format!(
        "ed25519-signature-v1\nover-sha256-of: {}\nsignature: {sig_b64}\npublic-key: {pub_b64}\n",
        receipt_path
            .file_name()
            .and_then(|f| f.to_str())
            .unwrap_or("(receipt)")
    );
    std::fs::write(&sig_path, body)
        .map_err(|e| AwareError::Internal(format!("write {}: {e}", sig_path.display())))?;
    Ok(sig_path)
}

/// Verify a receipt against a `.sig` sidecar. Returns Ok(()) on a valid
/// signature; Err(Validation) otherwise.
pub fn verify_receipt(receipt_path: &Path, sig_path: &Path) -> Result<(), AwareError> {
    let receipt_bytes = std::fs::read(receipt_path)
        .map_err(|e| AwareError::NotFound(format!("{}: {e}", receipt_path.display())))?;
    let digest = Sha256::digest(&receipt_bytes);

    let sig_text = std::fs::read_to_string(sig_path)
        .map_err(|e| AwareError::NotFound(format!("{}: {e}", sig_path.display())))?;

    let mut sig_b64: Option<&str> = None;
    let mut pub_b64: Option<&str> = None;
    for line in sig_text.lines() {
        if let Some(rest) = line.strip_prefix("signature:") {
            sig_b64 = Some(rest.trim());
        } else if let Some(rest) = line.strip_prefix("public-key:") {
            pub_b64 = Some(rest.trim());
        }
    }
    let sig_b64 =
        sig_b64.ok_or_else(|| AwareError::Validation(".sig missing `signature:` line".into()))?;
    let pub_b64 =
        pub_b64.ok_or_else(|| AwareError::Validation(".sig missing `public-key:` line".into()))?;

    let sig_bytes = base64::engine::general_purpose::STANDARD
        .decode(sig_b64)
        .map_err(|e| AwareError::Validation(format!("base64 signature: {e}")))?;
    let pub_bytes = base64::engine::general_purpose::STANDARD
        .decode(pub_b64)
        .map_err(|e| AwareError::Validation(format!("base64 public-key: {e}")))?;

    let sig_arr: [u8; 64] = sig_bytes
        .as_slice()
        .try_into()
        .map_err(|_| AwareError::Validation("signature must be 64 bytes".into()))?;
    let pub_arr: [u8; 32] = pub_bytes
        .as_slice()
        .try_into()
        .map_err(|_| AwareError::Validation("public key must be 32 bytes".into()))?;
    let verifying = VerifyingKey::from_bytes(&pub_arr)
        .map_err(|e| AwareError::Validation(format!("public-key: {e}")))?;
    let signature = Signature::from_bytes(&sig_arr);

    verifying
        .verify(&digest, &signature)
        .map_err(|e| AwareError::Validation(format!("signature does not verify: {e}")))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keypair_roundtrip_signs_and_verifies() {
        let tmp = tempfile::tempdir().unwrap();
        let keys = generate_keypair(tmp.path(), "test-operator").unwrap();
        let signing = load_signing_key(&keys.sec_path).unwrap();
        let receipt = tmp.path().join("r.jsonl");
        std::fs::write(&receipt, "{\"kind\":\"test\",\"value\":42}\n").unwrap();
        let sig = sign_receipt(&receipt, &signing).unwrap();
        verify_receipt(&receipt, &sig).unwrap();
    }

    #[test]
    fn tampering_with_receipt_invalidates_signature() {
        let tmp = tempfile::tempdir().unwrap();
        let keys = generate_keypair(tmp.path(), "test-operator").unwrap();
        let signing = load_signing_key(&keys.sec_path).unwrap();
        let receipt = tmp.path().join("r.jsonl");
        std::fs::write(&receipt, "{\"value\":42}\n").unwrap();
        let sig = sign_receipt(&receipt, &signing).unwrap();
        // Tamper with the receipt.
        std::fs::write(&receipt, "{\"value\":99}\n").unwrap();
        assert!(verify_receipt(&receipt, &sig).is_err());
    }

    #[test]
    fn generate_refuses_to_overwrite_existing_key() {
        let tmp = tempfile::tempdir().unwrap();
        generate_keypair(tmp.path(), "x").unwrap();
        let err = generate_keypair(tmp.path(), "x").unwrap_err();
        match err {
            AwareError::Validation(m) => assert!(m.contains("refusing to overwrite")),
            e => panic!("expected Validation, got {e:?}"),
        }
    }

    #[test]
    fn pubkey_roundtrip() {
        let tmp = tempfile::tempdir().unwrap();
        let keys = generate_keypair(tmp.path(), "x").unwrap();
        let signing = load_signing_key(&keys.sec_path).unwrap();
        let verifying = load_verifying_key(&keys.pub_path).unwrap();
        assert_eq!(verifying.to_bytes(), signing.verifying_key().to_bytes());
    }

    // ── malformed inputs ─────────────────────────────────────────────────────
    //
    // Everything above walks the happy path. The branches below are the ones a
    // receipt actually arrives on when it has been hand-edited, truncated in
    // transit, or swapped — i.e. every case verification exists to catch. None
    // of them was reached by a test before.

    const B64: base64::engine::general_purpose::GeneralPurpose =
        base64::engine::general_purpose::STANDARD;

    /// Unwrap an `AwareError::Validation`, failing loudly on any other variant.
    /// The variant is load-bearing: it is what maps to exit code 3, so a test
    /// that accepted any `Err` would not notice a `NotFound` regression.
    fn validation_message(err: AwareError) -> String {
        match err {
            AwareError::Validation(m) => m,
            other => panic!("expected Validation, got {other:?}"),
        }
    }

    /// A fresh keypair and a signed receipt, ready to be corrupted.
    /// Returns the `TempDir` so the caller keeps the tree alive.
    fn signed_fixture() -> (tempfile::TempDir, PathBuf, PathBuf, SigningKey) {
        let tmp = tempfile::tempdir().unwrap();
        let keys = generate_keypair(tmp.path(), "operator").unwrap();
        let signing = load_signing_key(&keys.sec_path).unwrap();
        let receipt = tmp.path().join("r.jsonl");
        std::fs::write(
            &receipt,
            "{\"panel\":\"structural\",\"verdict\":\"PASS\"}\n",
        )
        .unwrap();
        let sig = sign_receipt(&receipt, &signing).unwrap();
        (tmp, receipt, sig, signing)
    }

    /// Rewrite one `key: value` line of a `.sig` sidecar, leaving the rest byte
    /// for byte. `None` drops the line entirely.
    fn rewrite_sig_line(sig_path: &Path, key: &str, value: Option<&str>) {
        let text = std::fs::read_to_string(sig_path).unwrap();
        let mut out = String::new();
        let mut seen = false;
        for line in text.lines() {
            if line.starts_with(&format!("{key}:")) {
                seen = true;
                if let Some(v) = value {
                    out.push_str(&format!("{key}: {v}\n"));
                }
            } else {
                out.push_str(line);
                out.push('\n');
            }
        }
        assert!(
            seen,
            "sidecar had no `{key}:` line to rewrite — fixture rot"
        );
        std::fs::write(sig_path, out).unwrap();
    }

    #[test]
    fn load_signing_key_rejects_a_foreign_header_tag() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("k.sec");
        // Well-formed base64 of a real 32-byte key, wrong tag: the tag is the
        // only thing standing between this and being loaded as one of ours.
        std::fs::write(&path, format!("ssh-ed25519 {}\n", B64.encode([7u8; 32]))).unwrap();
        let msg = validation_message(load_signing_key(&path).unwrap_err());
        assert!(msg.contains("ed25519-secret-key-v1"), "got {msg}");
    }

    #[test]
    fn load_signing_key_rejects_a_file_carrying_extra_tokens() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("k.sec");
        std::fs::write(
            &path,
            format!(
                "ed25519-secret-key-v1 {} trailing-junk\n",
                B64.encode([7u8; 32])
            ),
        )
        .unwrap();
        let msg = validation_message(load_signing_key(&path).unwrap_err());
        assert!(msg.contains("ed25519-secret-key-v1"), "got {msg}");
    }

    #[test]
    fn load_signing_key_rejects_a_body_that_is_not_base64() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("k.sec");
        std::fs::write(&path, "ed25519-secret-key-v1 not-!-base64\n").unwrap();
        let msg = validation_message(load_signing_key(&path).unwrap_err());
        // Named specifically: a lenient decoder that swallowed the error would
        // yield an empty key and fail the LENGTH check instead, which is a
        // different bug wearing the same `is_err()`.
        assert!(msg.contains("base64"), "got {msg}");
    }

    #[test]
    fn load_signing_key_rejects_a_key_that_is_not_32_bytes() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("k.sec");
        std::fs::write(
            &path,
            format!("ed25519-secret-key-v1 {}\n", B64.encode([7u8; 31])),
        )
        .unwrap();
        let msg = validation_message(load_signing_key(&path).unwrap_err());
        assert!(msg.contains("32 bytes"), "got {msg}");
    }

    #[test]
    fn load_signing_key_reports_a_missing_file_as_not_found() {
        let tmp = tempfile::tempdir().unwrap();
        let err = load_signing_key(&tmp.path().join("absent.sec")).unwrap_err();
        // NotFound, not Validation: exit code 7 is what tells a caller "you
        // named a key that does not exist" rather than "your key is corrupt".
        assert!(
            matches!(err, AwareError::NotFound(_)),
            "expected NotFound, got {err:?}"
        );
        assert_eq!(err.exit_code(), 7);
    }

    #[test]
    fn load_verifying_key_refuses_the_secret_half_of_the_pair() {
        // The likeliest operator slip: handing a verifier the `.sec` file. Both
        // files are 32 base64 bytes and differ only by their tag, so nothing but
        // the tag check stops a private key being treated as publishable.
        let tmp = tempfile::tempdir().unwrap();
        let keys = generate_keypair(tmp.path(), "operator").unwrap();
        let msg = validation_message(load_verifying_key(&keys.sec_path).unwrap_err());
        assert!(msg.contains("ed25519-public-key-v1"), "got {msg}");
    }

    #[test]
    fn load_verifying_key_rejects_a_key_that_is_not_32_bytes() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("k.pub");
        std::fs::write(
            &path,
            format!("ed25519-public-key-v1 {}\n", B64.encode([7u8; 33])),
        )
        .unwrap();
        let msg = validation_message(load_verifying_key(&path).unwrap_err());
        assert!(msg.contains("32 bytes"), "got {msg}");
    }

    #[test]
    fn verify_rejects_a_sidecar_with_no_signature_line() {
        let (_tmp, receipt, sig, _k) = signed_fixture();
        rewrite_sig_line(&sig, "signature", None);
        let msg = validation_message(verify_receipt(&receipt, &sig).unwrap_err());
        assert!(msg.contains("missing `signature:`"), "got {msg}");
    }

    #[test]
    fn verify_rejects_a_sidecar_with_no_public_key_line() {
        let (_tmp, receipt, sig, _k) = signed_fixture();
        rewrite_sig_line(&sig, "public-key", None);
        let msg = validation_message(verify_receipt(&receipt, &sig).unwrap_err());
        assert!(msg.contains("missing `public-key:`"), "got {msg}");
    }

    #[test]
    fn verify_rejects_a_truncated_signature() {
        let (_tmp, receipt, sig, _k) = signed_fixture();
        rewrite_sig_line(&sig, "signature", Some(&B64.encode([0u8; 63])));
        let msg = validation_message(verify_receipt(&receipt, &sig).unwrap_err());
        assert!(msg.contains("64 bytes"), "got {msg}");
    }

    #[test]
    fn verify_rejects_a_public_key_that_is_not_a_curve_point() {
        let (_tmp, receipt, sig, _k) = signed_fixture();
        // y = 2 does not decompress to a Curve25519 point, so this is refused
        // by `VerifyingKey::from_bytes` before any signature maths runs — a
        // distinct branch from "the equation was not satisfied". Most 32-byte
        // patterns are NOT this: `[0xFF; 32]` decompresses fine and falls
        // through to a verification failure instead.
        let mut off_curve = [0u8; 32];
        off_curve[0] = 2;
        rewrite_sig_line(&sig, "public-key", Some(&B64.encode(off_curve)));
        let msg = validation_message(verify_receipt(&receipt, &sig).unwrap_err());
        assert!(
            msg.contains("public-key") && !msg.contains("does not verify"),
            "expected a key-decode refusal, got {msg}"
        );
    }

    #[test]
    fn verify_rejects_a_signature_that_belongs_to_another_key() {
        // The signature and the receipt are both untouched and internally
        // consistent; only the key they are checked against changes. This is
        // what proves the ed25519 verification is actually executed rather
        // than the sidecar merely being parsed.
        let (tmp, receipt, sig, _k) = signed_fixture();
        let other = generate_keypair(tmp.path(), "someone-else").unwrap();
        let other_pub = load_verifying_key(&other.pub_path).unwrap();
        rewrite_sig_line(&sig, "public-key", Some(&B64.encode(other_pub.to_bytes())));

        let msg = validation_message(verify_receipt(&receipt, &sig).unwrap_err());
        assert!(msg.contains("does not verify"), "got {msg}");
    }

    #[test]
    fn verify_reports_a_missing_sidecar_as_not_found() {
        let (tmp, receipt, _sig, _k) = signed_fixture();
        let err = verify_receipt(&receipt, &tmp.path().join("absent.sig")).unwrap_err();
        assert!(
            matches!(err, AwareError::NotFound(_)),
            "expected NotFound, got {err:?}"
        );
    }

    #[test]
    fn signing_a_receipt_that_does_not_exist_is_not_found() {
        let tmp = tempfile::tempdir().unwrap();
        let keys = generate_keypair(tmp.path(), "operator").unwrap();
        let signing = load_signing_key(&keys.sec_path).unwrap();
        let err = sign_receipt(&tmp.path().join("absent.jsonl"), &signing).unwrap_err();
        assert!(
            matches!(err, AwareError::NotFound(_)),
            "expected NotFound, got {err:?}"
        );
    }

    #[test]
    fn the_sidecar_names_the_receipt_and_carries_the_signer_public_key() {
        // A verifier reads the sidecar alone, so both fields are contract: the
        // file name says which artefact the digest covers, and the key says who
        // to attribute it to. Cross-checked against the key loaded from disk,
        // not against a value this test computed.
        let (_tmp, _receipt, sig, signing) = signed_fixture();
        let body = std::fs::read_to_string(&sig).unwrap();
        assert!(body.contains("over-sha256-of: r.jsonl"), "got {body}");
        assert!(
            body.contains(&B64.encode(signing.verifying_key().to_bytes())),
            "sidecar does not carry the signing key's public half: {body}"
        );
    }

    #[test]
    fn the_sidecar_lands_beside_the_receipt_whatever_its_extension() {
        // `sign_receipt` derives the sidecar path through `with_extension`,
        // which REPLACES rather than appends — the surrounding `format!` is
        // what turns it back into an append. `aware receipt verify` finds the
        // sidecar by appending ".sig" itself (`receipt_cli::default_sig_path`),
        // so the two must agree for every shape of name or verification of an
        // extensionless receipt silently looks in the wrong place.
        let tmp = tempfile::tempdir().unwrap();
        let keys = generate_keypair(tmp.path(), "operator").unwrap();
        let signing = load_signing_key(&keys.sec_path).unwrap();

        for name in [
            "plain",
            "run.jsonl",
            "run.2026-08-17.jsonl",
            "archive.tar.gz",
        ] {
            let receipt = tmp.path().join(name);
            std::fs::write(&receipt, "{}\n").unwrap();
            let sig = sign_receipt(&receipt, &signing).unwrap();
            assert_eq!(
                sig,
                tmp.path().join(format!("{name}.sig")),
                "sidecar for {name} landed at {}",
                sig.display()
            );
            verify_receipt(&receipt, &sig).unwrap();
        }
    }

    #[cfg(unix)]
    #[test]
    fn the_secret_key_is_not_readable_by_anyone_else() {
        use std::os::unix::fs::PermissionsExt;
        let tmp = tempfile::tempdir().unwrap();
        let keys = generate_keypair(tmp.path(), "operator").unwrap();
        let mode = std::fs::metadata(&keys.sec_path)
            .unwrap()
            .permissions()
            .mode();
        assert_eq!(
            mode & 0o077,
            0,
            "secret key is group/world accessible: {mode:o}"
        );
    }

    #[test]
    fn generate_replaces_a_public_key_orphaned_by_a_deleted_secret() {
        // CHARACTERISATION, NOT AN ENDORSEMENT. The overwrite guard tests
        // `sec_path` alone, so a `.pub` whose `.sec` is gone is silently
        // replaced — and every receipt already signed under the old key becomes
        // unattributable, because the published half no longer matches. Left as
        // it stands because changing it is a product decision, not test
        // hygiene; if you are here because you fixed it, rewrite this test.
        let tmp = tempfile::tempdir().unwrap();
        let first = generate_keypair(tmp.path(), "operator").unwrap();
        let published = std::fs::read_to_string(&first.pub_path).unwrap();
        std::fs::remove_file(&first.sec_path).unwrap();

        generate_keypair(tmp.path(), "operator").expect("guard only covers the .sec half");
        let now = std::fs::read_to_string(&first.pub_path).unwrap();
        assert_ne!(published, now, "the orphaned public key survived");
    }
}
