# Provider and artifact contract

Treat the provider adapter as a separately installed local trusted dependency. Configure its absolute
regular executable path with `AWARE_MODEL_REFERENCE_PROVIDER`; never use PATH lookup, a shell command,
URL or committed binary. Protocol v1 accepts only local execution. Protocol v2 accepts managed-cloud
execution only when the caller supplies the exact canonical HTTPS origin returned by `preflight`; the
origin is part of the complete provider fingerprint. A v2 conversion also requires an absolute,
installer-enrolled `authority-store-path`; AWARE passes it to the provider only in that conversion
request and never copies credentials into its environment. Configure the AWARE-format signing key locally.
Run `preflight`, pin the returned full provider fingerprint, obtain the signer fingerprint through an
independent operator trust channel, then call `probe`, `read-model` or `read-snapshot` with both pins,
the selected protocol/destination, enrolled authority-store path and the source SHA-256.

The canonical request, provider fingerprint, source digest and signer trust anchor jointly define a
cache key. Every cache hit verifies its signature, closed receipt, complete file set and every blob
digest. A cache result is reusable conversion evidence, not approval authority.

`read-model` preserves the original five-descriptor compatibility response and does not expose the
private cache receipt. Use `read-snapshot` when a downstream consumer needs public authenticated source
and bounded display-package envelopes. Verify the enrolled signer and source envelope before the
package envelope; neither signature grants project approval.

Consume semantic records only through their explicit IDs and joins. Preserve property group/order,
units and tagged storage values. Do not derive Revit Category, Family, Type, Level, hierarchy or stable
identity from geometry or names.
