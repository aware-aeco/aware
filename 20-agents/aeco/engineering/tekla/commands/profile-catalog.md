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

## Parametric shapes (`parametric: true`)

```powershell
'{"parametric":true}' | aware-tekla profile-catalog --json-stdin
```

The result also carries every parametric shape the environment defines, built-in and
user-defined (`CatalogHandler.GetParametricProfileItems`, still read-only):

```json
{
  "environment": { "name": "default", "profile_database": "C:\\ProgramData\\Trimble\\Tekla Structures\\2026.0\\\\environments\\default\\General\\Shared\\Profil\\" },
  "parametric_item_count": 284,
  "parametric_count": 244,
  "parametric": [
    {
      "prefixes": ["HI", "WI"],
      "parameter_string": "h-s-t*b",
      "type": "PROFILE_I", "subtype": "PROFILE_I_WELDED_SYMMETRICAL", "family": "i-section",
      "built_in": true, "sketched": false,
      "parameters": [
        { "symbol": "h", "label": "Height", "label_key": "albl_Height", "property": "HEIGHT",
          "unit": "mm", "unit_type": "INPUT_DIMENSION_UNIT", "default": 300, "in_name": true, "cross_section": 1 },
        { "symbol": "r", "label": "Rounding radius", "label_key": "albl_Rounding_radius", "property": "ROUNDING_RADIUS",
          "unit": "mm", "unit_type": "INPUT_DIMENSION_UNIT", "default": 20, "in_name": false, "cross_section": 1 }
      ]
    }
  ]
}
```

- One entry per shape: prefixes with the same type, subtype, parameter string and parameters are
  aliases and are listed together in `prefixes` (`parametric_item_count` counts Tekla's items,
  `parametric_count` the entries). An empty-string prefix is kept as Tekla reports it.
- `parameter_string` is Tekla's name order, verbatim. It is not always positional: Tekla reads
  `PL20*200` as `h = 200, b = 20` for the order `h*b` (plates take the larger number as the height).
  Use `section-measure` to see what Tekla made of a name.
- `parameters` are in Tekla's order. `label` is the plain-English reading of `label_key` (`albl_`
  dropped, underscores as spaces); `property` is Tekla's own field. `unit` is `mm`, `deg`,
  `integer`, `string`, `boolean`, `ratio`, `factor` or `other`, with the raw `unit_type` beside it. `in_name`
  says whether the symbol appears in `parameter_string`; a parameter that is not in the name (a
  rounding radius) takes Tekla's default.
- `built_in` is false for `PROFILE_USER_PARAMETRIC` (user and sketched shapes); `sketched` is
  Tekla's `IsSketchedUserParametric`. `family` uses the same mapping as the library catalog.
- `environment.name` is the folder after `\environments\` in `XS_PROFDB`.
- Entries are sorted by ordinal comparison of their first prefix and key, so the same
  environment gives the same bytes on every run.

Refusals are the same as `exec`: exactly one reachable Tekla instance is required.
