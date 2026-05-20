# `bcf-file.convert` — translate between BCF 2.1 and 3.0

Stateless, **write-mode**. Reads a BCF in one version and writes it in the other. Used to align files before a [`merge`](./merge.md), or to downgrade a 3.0 file so an older consumer (ACC Issues, an older Revit BCF plugin) can read it. The input file is never modified.

## Lifecycle

`single` — one call, one response

## Mode

`write` — produces a new file. Input is read-only; only `output-path` is created.

## Inputs

| Field | Type | Description |
|---|---|---|
| `input-path` | string | Source `.bcfzip` or folder. |
| `output-path` | string | Where to write the converted `.bcfzip`. Overwritten if it exists. |
| `target-version` | enum `2.1` \| `3.0` | The version to emit. If the source is already this version, the file is copied through and a no-op warning is returned. |

## Outputs

```yaml
path: string
warnings:
  type: array
  items: string     # every lossy transform is reported here — read them
```

## What the conversion does

The full field-by-field behaviour lives in [bcf-21-vs-30](../skills/bcf-21-vs-30.md). Summary:

**2.1 → 3.0 (lossless-ish, additive):**
- `Labels` kept as labels
- `ReferenceLinks` upgraded from bare strings to `{ Url, Description }` objects (Description left empty)
- `ModifiedDate` populated from `CreationDate` where absent (best-effort)
- `Stage` left empty — there is no 2.1 source for it; a downstream tool fills it in

**3.0 → 2.1 (lossy — every drop is a `warnings` entry):**
- `Stage` has no 2.1 equivalent → preserved as a label `Stage:<value>` so the data isn't lost, just relocated
- `ReferenceLinks` objects flattened to their URL string only
- Extension-schema fields (custom topic types) dropped
- Per-comment images and threaded replies flattened (2.1 comments are flat)
- Viewpoint colouring alpha channel dropped to RGB

**Default outbound is 2.1.** As of 2026, ACC Issues / BIMcollab / Revizto / Trimble Connect support for 3.0 is still patchy — emit 2.1 unless you've confirmed every consumer in the chain reads 3.0. See [bcf-21-vs-30](../skills/bcf-21-vs-30.md) for the recommendation.

## Composition example — downgrade for an older consumer

```yaml
- id: down
  agent: bcf-file
  command: convert
  config:
    input-path:     "{{ inputs.solibri-30 }}"
    output-path:    "./out/for-acc-21.bcfzip"
    target-version: "2.1"
  safety: { snapshot: true }

- id: warn-if-lossy
  inline:
    kind: assert
    description: Surface the lossy conversion to the operator before upload
    code: out => out.warnings.length === 0 || `BCF downgrade dropped: ${out.warnings.join("; ")}`
```

## Failure modes

| Error | Cause | Recovery |
|---|---|---|
| `bcf.file-not-found` | `input-path` doesn't exist | Check the path |
| `bcf.unsupported-version` | Source is neither 2.1 nor 3.0 (e.g. 2.0) | 2.0 conversion is post-v0; open an issue |
| `bcf.noop-conversion` | `target-version` equals the source version | Returned as a `warnings` entry, not an error — the file is copied through unchanged |
| `bcf.output-not-writable` | `output-path` dir missing/read-only | Create dir / fix permissions |

## See also

- [bcf-21-vs-30](../skills/bcf-21-vs-30.md) — the authoritative field-by-field difference + which version to emit
- [`merge`](./merge.md) — the main reason to convert (merge requires uniform versions)
