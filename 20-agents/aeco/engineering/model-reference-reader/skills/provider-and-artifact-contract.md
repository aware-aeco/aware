# Provider and artifact contract

Treat the provider as a separately installed local trusted dependency. Configure its absolute regular
executable path with `AWARE_MODEL_REFERENCE_PROVIDER`; never use PATH lookup, a shell command, URL or
committed binary. Configure the AWARE-format signing key locally. Run `preflight`, pin the returned full
provider fingerprint, then call `probe` and `read-model` with the source SHA-256.

The canonical request, provider fingerprint, source digest and signer trust anchor jointly define a
cache key. Every cache hit verifies its signature, closed receipt, complete file set and every blob
digest. A cache result is reusable conversion evidence, not approval authority.

Consume semantic records only through their explicit IDs and joins. Preserve property group/order,
units and tagged storage values. Do not derive Revit Category, Family, Type, Level, hierarchy or stable
identity from geometry or names.
