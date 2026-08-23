# probe

`probe` returns a bounded summary of the same canonical snapshot `read-model` publishes: Z-up
millimetre bounds, entity/property/relationship counts, exact join coverage, source hash, full provider
fingerprint and canonical-request hash. A cold probe is intentionally expensive because exact geometry
and joins cannot be established without conversion. A warm probe revalidates the signed cache.

The source is copied into a private immutable staging file and hashed on both sides. The provider sees
only that staged path. A source or executable change at any bracket refuses the run. Execution is
`local` with `destination: null`; remote destinations and external GLB resources are unsupported.

Bounds come from normalized active-scene geometry, not from names or metadata. Unclaimed geometry is
reported in coverage rather than assigned by guesswork.
