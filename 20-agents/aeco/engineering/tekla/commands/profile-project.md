# profile-project

The Tekla name for each canonical section, in the environment of the open Tekla, with proof of
what the name selects. It is the projection a caller uses to export a member on a parametric
section to Tekla: the caller sends the section in AWARE's scene `xsection` vocabulary
(millimetres), and AWARE answers how this Tekla environment writes it.

```powershell
'{"sections":[{"id":"m1","canonical":{"shape":"i","d":406.4,"bf":203.2,"tw":9.525,"tf":12.7}},
              {"id":"m2","canonical":{"shape":"rect","w":200,"d":20}}],
  "prefixes":{"i":"WI"}}' |
  aware-tekla profile-project --json-stdin
```

Read-only: it asks Tekla's catalogue, inserts nothing and commits nothing. A malformed request is
refused before Tekla is touched (exit 2); a section that cannot be named gets its own `status` and
the batch continues.

## Shapes

| canonical | Tekla shape (type / subtype, name order) | default prefix | how the sizes go in |
|---|---|---|---|
| `rect {w, d}` | `PROFILE_PL` / `PROFILE_PL_DEFAULT`, `h*b` | PL | h = the larger size, b = the smaller |
| `i {d, bf, tw, tf}` | `PROFILE_I` / `PROFILE_I_WELDED_SYMMETRICAL`, `h-s-t*b` | HI | h = d, s = tw, t = tf, b = bf |
| `tee {d, bf, tw, tf}` | `PROFILE_T` / `PROFILE_T_PARAMETRIC`, `h-s-t-b` | T | h = d, s = tw, t = tf, b = bf |
| `angle {d, b, t}` | `PROFILE_FPL` / `PROFILE_L_COLD_ROLLED`, `h*b*t` | L | h = d, b = b, t = t |
| `channel {d, bf, tw, tf}` | `PROFILE_U` / `PROFILE_U_COLD_ROLLED`, `h*b*t` | U | only when tw = tf: h = d, b = bf, t = tw |
| `rhs {d, b, t}` | `PROFILE_P` / `PROFILE_P_RECTANGULAR`, `h*b*t` | P | h = d, b = b, t = t (written `h*b*t` even when square) |
| `chs {od, t}` | `PROFILE_PD` / `PROFILE_PD_CIRCULAR`, `d*t` | PD | d = od, t = t |
| `solid-circle {diameter}` | `PROFILE_D` / `PROFILE_D_CIRCULAR`, `d` | D | d = diameter |

Each size must be a finite number from 0.000001 to 1000000000 mm, and the shape must be able to exist
(i and channel: 2·tf < d and tw < bf; tee: tf < d and tw < bf; angle: t < d and t < b; rhs: 2·t < d
and 2·t < b; chs: 2·t < od); otherwise `invalid`. A channel whose web and flanges differ in thickness
is `no-shape`, because Tekla's parametric channel has one thickness. A double angle is `no-shape`: it
is two Tekla members, not one profile.

**The prefix.** The default prefix when this environment lists it, otherwise the first alias the
catalogue lists for that shape. `prefixes` picks another alias per shape (for example
`{"i": "WI", "rect": "FLT"}`); one the environment does not list is `prefix-not-available`, and the
message names the aliases it does list.

**A wide plate.** Tekla's plate always takes the larger number as its height. So a `rect` wider than
it is deep is written as Tekla's tall plate and carries `section_transform`:
`{"from": "tekla-section", "to": "canonical", "matrix": [[0,-1],[1,0]], "origin": [0,0]}`, meaning
canonical = matrix × Tekla point, about the section origin (the centre of its bounding box): a
quarter turn counter-clockwise. The plate is symmetric, so the other quarter turn gives the same
outline; place the member with a rotation that turns its section by 90 degrees.

## How the name is made, and proved

The name is composed by AWARE, never by Tekla's `CreateProfileString()`, which rounds every size to
two decimals (`0.001` becomes `PL0*100`, a zero plate Tekla then accepts). Sizes are written in
millimetres in every environment, imperial included (Tekla's Open API reads profile strings in mm
there too; inch text such as `D1"` is read as **1 mm**), with a dot decimal whatever the machine's
culture, at full precision (the fewest digits that read back as the same number) and never with an
exponent. The order is Tekla's own name order, by symbol: Tekla lists an HI's parameters as
h, b, s, t, r while its name is h-s-t*b.

Then it is proved, in this order:

1. **No library profile has that name** (`LibraryProfileItem.Select`). Tekla selects a library
   profile before a parametric one, so a collision would give a different section:
   `library-collision`.
2. **Tekla's parametric read-back** (`ParametricProfileItem.Select`) reports the chosen prefix, the
   intended type, subtype and name order, every named size exactly once as a length and **bit for
   bit** equal to the size asked for, and no parameter outside the shape's own. Anything else is
   `read-back-mismatch`, with both sides in `read_back` and the message.

Only then is the section `named`. Sizes Tekla derives and does not write in the name are reported in
`hidden_parameters`: the rounding radius `r` of L, U and P (1.5 × t, 1.5 × t and 2 × t as measured)
and of HI and T (0.7 × s). They are why those Tekla shapes never equal a sharp-cornered section.

## Geometry is measured by `section-measure`, not here

This verb does not compare outlines. `measure` is a ready request for
[`section-measure`](section-measure.md), listing every name that was `named` (left out when none
was). That verb measures each name's section, its reference point and its placement on real
temporary members; the caller compares them with its own section, faces and reference point. A
`section-measure` result can be `status: ok` with `reference_point: null` or empty `section_maps`, so
a caller checks those fields before it claims the reference point or placement agree.

## Input

| field | required | meaning |
|---|---|---|
| `sections` | yes | 1 to 5000 sections. Each is `{id, canonical, definition?, sizes?, faces?, reference_point?}`; `id` is unique text. |
| `sections[].canonical` | yes | `{shape, …sizes in mm}`, the scene `xsection` vocabulary. |
| `sections[].definition`, `sizes`, `faces`, `reference_point` | no | The caller's own payload (for FloLess: the definition, its sizes, its evaluated faces and reference point). Echoed untouched; nothing here reads them. |
| `prefixes` | no | `{shape: prefix}` for the shapes above. |
| `version`, `pid` | no | Which Tekla, as for every verb. |

## Output

`{ok, result, host, host_version, host_pid, verb, delivered_at}` with

```text
result: {
  model_path, environment: {name, profile_database}, named_count,
  sections: [{
    id, status: named | invalid | no-shape | prefix-not-available | library-collision | read-back-mismatch | error,
    message?, canonical, definition?, sizes?, faces?, reference_point?,
    name?,                                      // e.g. "WI406.4-9.525-12.7*203.2"
    tekla?: {type, subtype, prefix, aliases, parameter_string},
    parameters?: [{symbol, value, from}],       // in name order; `from` is the canonical size it came from
    hidden_parameters?: [{symbol, value, unit_type}],
    read_back?: {prefix, type, subtype, parameter_string, parameters: [{symbol, value, unit_type}]},
    section_transform?: {from, to, matrix, origin, note}
  }],
  measure?: {profiles: [name, ...]}
}
```

Live samples (UK and USA imperial, Tekla 2026.0) are in `cli-tekla/samples/`.
