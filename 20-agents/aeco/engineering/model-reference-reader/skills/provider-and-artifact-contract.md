# Provider and artifact contract

Treat the provider adapter as a separately installed local trusted dependency. Configure its absolute
regular executable path with `AWARE_MODEL_REFERENCE_PROVIDER`; never use PATH lookup, a shell command,
URL or committed binary. Protocol v1 accepts only local execution. Protocol v2 accepts managed-cloud
execution only when the caller supplies the exact canonical HTTPS origin returned by `preflight`; the
origin is part of the complete provider fingerprint. A protocol-v2 conversion also requires an absolute,
installer-enrolled `authority-store-path`; AWARE passes it to the provider only in that conversion
request and never copies credentials into its environment. Every protocol-v2 conversion request carries
a fresh `conversion-attempt-id`, and the live receipt must return that exact value before AWARE accepts
the conversion. Configure the
AWARE-format signing key locally.
Run `preflight`, pin the returned full provider fingerprint, obtain the signer fingerprint through an
independent operator trust channel, then call `probe`, `read-model` or `read-snapshot` with both pins,
the selected protocol/destination, enrolled authority-store path and the source SHA-256.

Reader v2 requires every `describe` response and conversion receipt to acknowledge
`reader-schema-version: model-reference-reader/v2`, for local protocol-v1 and managed protocol-v2
providers alike. Because that acknowledgement is part of the provider fingerprint, run `preflight`
and pin again when selecting a different reader schema. Reader v2 accepts optional
`property-expansion-limits`. Their effective defaults inherit the same
request's `limits.maxParameters` and `limits.maxComponentJsonBytes`, and the canonical request signs
the resulting row and byte ceilings. A caller may explicitly allow more expanded rows up to the hard
cap, but the property-byte ceiling never exceeds the enclosing component-byte ceiling. Selecting
reader v2 raises the default component JSON ceiling from 32 MiB to 128 MiB so larger property and
relationship shards can be admitted; this is the hard ceiling and callers may lower it explicitly.

The ordinary source admission ceiling is 256 MiB so the default profile admits at least twice the
pinned Snowdon Towers sample. This default is part of the signed canonical request: upgrading from the
earlier 150 MiB default changes the request hash, so the first read after upgrade converts again instead
of reusing an entry produced under different limits. The bounded cache's normal quota maintenance later
reclaims unmatched entries by deterministic least-recently-used eviction; it does not treat an old
request hash as current evidence.

The canonical request, provider fingerprint, source digest and signer trust anchor jointly define a
cache key. Every cache hit verifies its signature, closed receipt, complete file set and every blob
digest. A cache result is reusable conversion evidence, not approval authority.

Protocol v3 package preflight and source fingerprinting form a separate, format-neutral route. An operator first trusts a
publisher key, enrolls a signed closed package, and selects its manifest digest for an opaque format
with `aware provider`. The caller supplies that format, one declared capability and the exact selected
manifest digest. AWARE verifies the publisher, manifest signature, closed file inventory and every file
receipt before and after invoking `describe`. The caller then uses `fingerprint-source` with ordered,
user-authorized logical namespace roots. AWARE captures those roots privately, invokes `discover`,
rechecks the staged bytes and applies an operator-admitted dependency policy. The portable effective
source carries consumed receipts and explicit absences without original paths. This version does not
dispatch v3 `convert`.

`read-model` preserves the original five-descriptor compatibility response and does not expose the
private cache receipt. Use `read-snapshot` when a downstream consumer needs public authenticated source
and bounded display-package envelopes. Verify the enrolled signer and source envelope before the
package envelope; neither signature grants project approval.

Consume semantic records only through their explicit IDs and joins. In v2, `parameters[i].id` and
`parameterGroups[i].id` are the one-based decimal position (`i + 1`), while references into those
tables are zero-based array indexes. Preserve property group/order and units. A `source-storage` value
retains `readable`, `storageType`, and its source value. A `provider-display` value instead retains its
declared `valueType` and display value; it deliberately carries no storage tag and cannot establish
IfcGUID identity (`providerDisplayIdentity: excluded`). Do not derive Revit Category, Family, Type,
Level, hierarchy or stable identity from geometry or names.
