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
}
