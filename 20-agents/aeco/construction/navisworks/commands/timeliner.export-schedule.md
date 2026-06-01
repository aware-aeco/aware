# timeliner.export-schedule

Export the TimeLiner (4D) schedule as MS Project XML, CSV, or Primavera P6 XML. Read-only.

## Inputs

| Field | Type | Required | Description |
|---|---|---|---|
| `document-id` | string | yes | The open federation. |
| `format` | enum `msproject-xml\|csv\|p6-xml` | no (default `csv`) | Output format. |
| `output-path` | string | yes | Output file path. |

## Output

```yaml
path:       "C:/schedule/4d.csv"
task-count: 214
```

## Implementation (Navisworks .NET API · Navisworks Manage)

> **Important — read before composing.** The TimeLiner .NET API is **import + read/create/modify only — there is no native schedule *export*.** Across Autodesk's TimeLiner dev-blog series and the community tutorials, no export-to-MS-Project/CSV/P6 method appears; the supported integration direction is *into* Navisworks (schedules are exported from Primavera/MS Project and attached/imported to TimeLiner via a data source). So this verb **reads `TimelinerTask`s and serializes the chosen format itself** — it does not call a Navisworks exporter. Implying a native export would be fabricating an API.

1. Get the TimeLiner: `DocumentTimeliner tl = doc.GetTimeliner();` (or `(DocumentTimeliner)doc.Timeliner`).
2. Walk the tasks (`TimelinerTask`): `DisplayName`, `PlannedStartDate` / `PlannedEndDate`, `SimulationTaskTypeName`, attached `Selection`.
3. Serialize: CSV is straightforward; **MS Project XML** and **Primavera P6 XML** mean writing those schemas yourself from the task data.

## Gotchas

- **No export API — you write the file.** This is the load-bearing fact; the verb hand-rolls MS Project XML / CSV / P6 XML from `TimelinerTask` data. Treat the two XML formats as a real serialization effort (their schemas are non-trivial).
- **Integration is one-directional by design** — Navisworks *imports* schedules (via `TimelinerDataSourceProvider`); it does not export them. If round-tripping back to P6/MSProject is the goal, flag that the source-of-truth schedule should stay in the scheduling tool.
- **Date field names partly unverified** — `PlannedStartDate`/`PlannedEndDate` are the planned dates; the actual-date member names should be confirmed against the SDK reference.
- **`GetTimeliner(doc)` extension** is the simpler accessor vs the `doc.Timeliner` cast.
- **Manage-only.**

Sources: [TimeLiner API part 1 — `DocumentTimeliner`/`GetTimeliner`, read+create (Autodesk dev blog)](https://blog.autodesk.io/timeliner-api-part1/) · [TimeLiner part 2 — `TimelinerDataSourceProvider` import; no export (twentytwo)](https://twentytwo.space/2022/12/27/navisworks-api-timeliner-part-2/) · [Supported scheduling software — export FROM P6/MSProject INTO Navisworks (Autodesk help)](https://help.autodesk.com/cloudhelp/2018/ENU/Navisworks/files/GUID-1879F810-B3D3-419A-A138-0D2F02A8BFBA.htm)

## See also

- [timeliner-and-4d](../skills/timeliner-and-4d.md) — TimeLiner model, data sources, simulation
- [selection.by-property](./selection.by-property.md) — build the selections tasks attach to
