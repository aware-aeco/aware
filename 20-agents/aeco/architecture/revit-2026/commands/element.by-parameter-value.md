# `revit-2026.element.by-parameter-value`

Typed filter over the active document — find elements where a named parameter matches a value (or is null / not null).

## When to use

When you'd otherwise reach for `FilteredElementCollector` + `ElementParameterFilter` + a typed value rule. This is the workflow-verb wrapper around that compose-it-yourself stack. Read-only.

## Inputs

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `parameter-name` | string | yes | — | Name of the parameter to test. |
| `operator` | enum | no | `equals` | `equals` / `not-equals` / `contains` / `is-null` / `is-not-null` / `greater-than` / `less-than`. |
| `value` | string | depends | — | Required for all operators except `is-null` / `is-not-null`. |
| `category` | string | no | — | Restrict to a Revit category (e.g. `"Walls"`, `"Doors"`). |

## Output

```yaml
elements:
  - element-id: "12345"
    category: "Walls"
    type-name: "Generic - 200mm"
    instance-marker: "W17"
```

## Worked example

Find every wall missing a fire rating:

```yaml
- id: walls-missing-fire
  agent: revit-2026
  command: element.by-parameter-value
  inputs:
    parameter-name: "Fire Rating"
    operator: is-null
    category: "Walls"
```

## Implementation note

Builds a `FilteredElementCollector` + `ElementParameterFilter` with a `FilterStringRule` / `FilterIntegerRule` / `FilterDoubleRule` based on the parameter's storage type. Category restriction uses `OfCategory` for performance.

## See also

- `parameter.bulk-write` — fix the values found
- `schedule.find-rows-missing` — same QA via schedule
