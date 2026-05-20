# clash.run-test

Run a single named `ClashTest` (already defined in the federation, typically via the Clash Detective UI) and return the result counts by status.

**WRITE-mode** (re-running mutates result statuses).

## Inputs

| Field | Type | Required | Description |
|---|---|---|---|
| `document-id` | string | yes | The open federation. |
| `test-name` | string | yes | Name of the `ClashTest`. Use `clash.list-tests` if unsure. |

## Output

```yaml
test-name: "STR-vs-MEP"
results:
  new:      12
  active:   3
  reviewed: 0
  approved: 5
  resolved: 41
```

## Implementation (Navisworks .NET API · Navisworks Manage)

Clash Detective lives in `Autodesk.Navisworks.Api.Clash` and is a **Navisworks Manage** feature.

```csharp
DocumentClash dc = Application.ActiveDocument.GetClash();
DocumentClashTests dct = dc.TestsData;
ClashTest test = dct.Tests.OfType<ClashTest>()
                    .First(t => t.DisplayName == testName);
dct.TestsRunTest(test);
```

Then tally statuses by walking the test's children: each child is either a `ClashResultGroup` (a folder — recurse) or a `ClashResult`. Read `ClashResult.Status` (enum `ClashResultStatus`) and count by value.

## Gotchas

- **Re-running mutates statuses** — new results are added, existing `New` results become `Active`, and non-reproducible results are removed. Snapshot the counts *after* the run, and treat the verb as write-mode for that reason.
- **Recurse the result tree.** `test.Children` mixes `ClashResultGroup` folders and `ClashResult` leaves — descend into groups (don't tally only the top level).
- **`ClashResultStatus` member spellings are unverified** against the SDK reference: `New`/`Active`/`Approved` are confirmed in samples; `Reviewed`/`Resolved` are inferred from the Clash Detective UI lifecycle — confirm against the shipped `.chm` before relying on exact names.
- **Manage-only.** The `Clash` assembly is present in Simulate but the run functionality requires Manage; the precise non-Manage failure mode is unverified.
- A `ClashResult` also exposes `Item1`/`Item2` (the two clashing `ModelItem`s), `Center`, `BoundingBox` — used by `viewpoint.from-clash`.

Sources: [xiaodongliang/Forge-Navisworks-ClashTest — `GetClash()`/`TestsData`/result walk/status](https://github.com/xiaodongliang/Forge-Navisworks-ClashTest/blob/master/Navisworks%20Plugin/Class1.cs) · [Run a clash test via API (Autodesk forum)](https://forums.autodesk.com/t5/navisworks-api-forum/to-run-a-clash-test-using-api/td-p/6607309) · [Change clash result status (Autodesk forum)](https://forums.autodesk.com/t5/navisworks-api/change-clash-result-properties-name-status/td-p/8762686)

## See also

- [clash.run-all](./clash.run-all.md) · [clash.list-tests](./clash.list-tests.md) · [clash.export](./clash.export.md)
- [viewpoint.from-clash](./viewpoint.from-clash.md) — screenshot a specific result
- [clash-detection-workflow](../skills/clash-detection-workflow.md)
