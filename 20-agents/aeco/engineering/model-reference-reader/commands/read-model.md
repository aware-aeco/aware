# read-model

`read-model` publishes five run-owned artifacts. Retrieve each opaque `id` with `aware app artifact`:

- `geometry` — binary GLB v2, canonical right-handed Z-up millimetres;
- `entities` — stable Revit element identities, exact Category/Family/Type/Level/class and appearance joins;
- `properties` — ordered parameter groups and typed values, including null, empty and unreadable states;
- `relationships` — explicit provider relationships with validated endpoints and hierarchy;
- `manifest` — source/request/provider/signer provenance, hashes, frame and reconciled coverage.

Supply the preflight provider fingerprint and an out-of-band trusted signer fingerprint on every call.
The reader refuses provider or signing-key rotation before cache access or conversion.

The GLB is never JSON-encoded. Entity meaning is never inferred from node names or geometry. One entity
may own several appearance nodes; every claimed node has exactly one owner, and unclaimed nodes remain
explicit in coverage. `IfcGUID` is comparable only when the authoritative exact parameter is present
and unique; duplicated values make every duplicate uncomparable.

The approved artifact is still FloLess/AWARE consumer state, not this cache entry. This reader produces
deterministic authenticated bytes; it does not assign a project UUID, generation, approval or mutable
"latest" handle.
