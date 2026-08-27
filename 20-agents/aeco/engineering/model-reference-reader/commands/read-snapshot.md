# read-snapshot

`read-snapshot` first runs the same deterministic conversion/cache path as `read-model`. A cache miss
invokes the pinned installed adapter once; an authenticated cache hit invokes it zero times. AWARE verifies
the private cache receipt and every source byte before it creates any public authentication object.

The command publishes:

- the unchanged five `read-model` descriptors;
- `sourceArtifactPreimage` and its domain-separated Ed25519 `sourceArtifactEnvelope`;
- one independently decodable bounded canonical GLB tile;
- bounded entity, property and relationship shards plus an entity-to-tile index;
- `packagePreimage` and its separately domain-separated Ed25519 `packageArtifactEnvelope`.

Verify the independently enrolled signer first, then the source envelope and all five source artifacts,
then the package envelope and every package artifact. The package binds the exact source preimage and
envelope digests, so a valid package cannot be attached to another conversion or run. Neither envelope
assigns FloLess project identity, generation or approval.
