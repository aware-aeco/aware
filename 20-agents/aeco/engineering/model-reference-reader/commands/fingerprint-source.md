# fingerprint-source

Use `fingerprint-source` after protocol-v3 preflight and before conversion. The caller supplies an
ordered set of logical source namespaces already resolved from user-authorized local folders. AWARE
copies every admitted file into private immutable staging, then runs the exact selected provider's
`discover` operation against that staging. The provider never receives the original paths.

The provider reports which captured files it consumed, which it ignored, and which dependency roles
are absent. AWARE applies the separately admitted local dependency policy; the provider cannot decide
whether an absence is mandatory, optional or degraded. Mandatory, unknown and unsupported external
dependencies refuse. Degraded output also refuses unless `degraded-mode` is explicitly `allow`.

The result contains the canonical `model-effective-source/v2` object and its SHA-256. It binds the
consumed file receipts, exact provider/package identity, dependency policy digest, primary source and
completeness state without exposing absolute paths. AWARE re-verifies the staged bytes, provider package
and policy after discovery and removes the private capture before returning.
