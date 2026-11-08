# profile-catalog

Read-only. Asks the open Tekla model which profile names it accepts and, on request,
lists the library profiles with their shape family and dimensions. It exists so a caller
can find out BEFORE a `bake-scene` which profiles the model lacks, and suggest a match
for each, instead of discovering it as a failed batch.

```powershell
'{"profiles":["KK200/120/5","HEA140"],"catalog":true}' |
  aware-tekla profile-catalog --json-stdin
```

Nothing is inserted, modified, committed or saved. The embedded script only calls
`LibraryProfileItem.Select`, `ParametricProfileItem.Select` and (for `catalog: true`)
`CatalogHandler.GetLibraryProfileItems`, plus `GetInfo()` for the model path.

## Input

| Field | Type | Description |
|---|---|---|
| `profiles` | string[] | Names to check. Duplicates and blanks are ignored. |
| `catalog` | boolean | Also return every library profile. |
| `version`, `pid` | | As for `exec`. |

## Output (`result`)

```json
{
  "model_path": "C:\\TeklaStructuresModels\\FloLess EU",
  "resolved": [
    { "profile": "KK200/120/5", "status": "missing" },
    { "profile": "HEA140", "status": "library" },
    { "profile": "RHS160*100*4", "status": "parametric" }
  ],
  "catalog_count": 1968,
  "catalog": [
    { "name": "CFRHS200X120X5", "family": "rectangular-hollow", "dims": { "h": 200, "b": 120, "t": 5, "r": 10 } }
  ]
}
```

`library` or `parametric` is exactly "`Insert()` would accept this name"; `missing` is
what `bake-scene` turns into a profile-less line. `family` is derived from Tekla's own
`ProfileItemType`/`ProfileItemSubType`: `rectangular-hollow`, `circular-hollow`,
`i-section`, `channel`, `angle`, `tee`, `z-section`, `flat`, `other`. `dims` carries the
profile's numeric dimension parameters (millimetres, invariant-culture numbers).

Refusals are the same as `exec`: exactly one reachable Tekla instance is required.
