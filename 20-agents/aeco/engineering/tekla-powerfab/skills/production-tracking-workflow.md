# Production tracking workflow

The fabricator's daily shop-floor loop, now scriptable.

## Stations

EPM models the production line as a sequence of stations. AWARE
exposes them as an enum on `production.advance`:

| Station | What happens |
|---|---|
| `received` | Material lands in yard from mill |
| `sawing` | Cut to length on the saw line (Peddinghaus etc.) |
| `drilling` | Holes drilled (Peddinghaus drill line) |
| `welding` | Assembly welding |
| `painting` | Paint shop / galv / fireproofing |
| `qc` | Quality control inspection |
| `shipping` | Loaded onto truck |
| `shipped` | Out the door |

## Tablet flow

Shop tablets typically scan piece barcodes and call:

```yaml
- id: scan
  agent: tekla-powerfab
  command: production.advance
  inputs:
    job-id: '{{ secrets.active-job-id }}'
    piece-marks: ['A-100-1', 'A-100-2', 'A-100-3']
    to-station: welding
  safety:
    transaction-group: shop-tablet
    snapshot: false  # EPM is the source-of-truth; this IS the state-write
```

The transport emits one `production.advance` per scan; EPM's audit log
captures `who / when / which station`.

## Bottleneck detection

Daily 7am the production manager wants: "which pieces are stuck at one
station for > 24 hours?"

```yaml
- id: status
  agent: tekla-powerfab
  command: production.status
  inputs:
    job-id: '{{ secrets.active-job-id }}'

- id: stuck
  inline:
    kind: map
    description: Filter pieces stuck > 24h at current station
    code: |
      pieces => pieces.filter(p =>
        (Date.now() - new Date(p['updated-at']).getTime()) > 86400000)

- id: post
  agent: microsoft-365
  command: teams.channel.post-with-card
  inputs:
    team-id: 'Acme Fab'
    channel-id: 'shop-floor'
    title: 'Production bottlenecks — {{ run.date }}'
    rows: '{{ stuck | groupBy: station }}'
```

## Common pitfalls

- **Out-of-order advances**: EPM accepts skip-ahead (e.g. `received` → `welding`) but logs a warning. Configure EPM's *Settings → Station ordering enforcement* if your shop policy requires strict sequence.
- **Skipped stations on rework**: a returning piece from `qc` back to `welding` is a valid state — the transport doesn't block it. EPM's audit captures the loop.
- **Concurrent writes**: two tablets advancing the same piece-mark race; later-write wins by timestamp. For high-throughput stations, batch via `piece-marks` array (atomic) rather than one call per piece.
