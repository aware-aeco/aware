# probe

`probe` returns a bounded summary of the same canonical snapshot `read-model` publishes: Z-up
millimetre bounds, entity/property/relationship counts, exact join coverage, source hash, full provider
fingerprint and canonical-request hash. A cold probe is intentionally expensive because exact geometry
and joins cannot be established without conversion. A warm probe revalidates the signed cache.

The source is copied into a private immutable staging file and hashed on both sides. The installed
adapter sees only that staged path. A source or executable change at any bracket refuses the run.
Protocol v1 requires `execution: local` and `destination: null`. Protocol v2 permits
`execution: managed-cloud` only when its exact canonical HTTPS origin is caller-pinned. External GLB
resources remain unsupported in both protocols.

The call must carry both `expected-provider-sha256` and the independently trusted
`expected-signer-sha256`. Neither pin is learned from the conversion being authenticated.

Bounds come from normalized active-scene geometry, not from names or metadata. Unclaimed geometry is
reported in coverage rather than assigned by guesswork.
