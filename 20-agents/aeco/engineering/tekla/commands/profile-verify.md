# profile-verify

The read-back half of [`profile-create`](profile-create.md). Run it **after Tekla has been restarted** (Tekla reads
`profitab.inp` only at start). It proves, from Tekla itself, that the prefix registered and measures like the shape
it aliases.

```powershell
'{"prefix":"AWTRI","reference":"TRIANGLE","samples":["400*200","300*100"]}' |
  aware-tekla profile-verify --json-stdin
```

Read-only: it enumerates the parametric catalogue and asks `CrossSection.Select`; it inserts nothing and commits
nothing. A malformed request is refused before Tekla is touched (exit 2).

| field | meaning |
|---|---|
| `prefix` | required; the prefix `profile-create` added. |
| `reference` | the aliased shape's prefix (`profile-create` returns it in `verify.request.reference`); each sample is also measured under it and the two are compared. |
| `samples` | 1 to 8 size strings, the part of a name after the prefix, e.g. `400*200`. |
| `version`, `pid` | which Tekla, as for every verb. |

Result: `status` is `verified` (registered, and every sample's outer loop, area and hole count equal the reference's
within 1e-6 mm), `mismatch`, or `not-registered` (Tekla does not list the prefix: restart Tekla, or see
`near_miss_entries`). The catalogue is **enumerated**, not only selected, because a prefix that is too long registers a
broken entry that `SelectByPrefix` cannot see. Also returned: `registered` (type, subtype, parameter string),
`parametric_total`, `select_by_prefix`, and per sample the alias and reference measurements with
`max_vertex_deviation_mm`.
