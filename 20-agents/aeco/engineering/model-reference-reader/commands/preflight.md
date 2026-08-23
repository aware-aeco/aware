# preflight

Use `preflight` before offering an RVT create/import action. It proves that the configured executable
is a regular local file, the AWARE-format Ed25519 keypair matches, the managed host protocol is current,
and the provider describes the exact closed local-RVT contract. It does not receive an RVT path and
does not convert a model.

`ready: true` is specific to the provider and signer fingerprints returned beside it. Pin
`providerFingerprintSha256` into `probe` and `read-model`, and obtain/pin `signerFingerprintSha256`
through the operator's out-of-band trust channel. A changed executable, engine, version, build, or
signing key then refuses before conversion. A missing provider and a missing signing key are setup failures.
They are distinct from a conversion failure after readiness.

AWARE 0.126.0 has no generic secret-provisioning facility (issue #448). Provider licensing and
credentials remain a local concern of the separately installed provider. The agent contains no cloud
URL, provider binary, credential or implicit discovery.
