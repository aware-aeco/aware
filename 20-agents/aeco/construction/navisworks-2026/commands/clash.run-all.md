# clash.run-all

Run every `ClashTest` defined in the federation — the Monday-morning coordination pass.

**WRITE-mode** (re-running mutates result statuses).

## Inputs

| Field | Type | Required | Description |
|---|---|---|---|
| `document-id` | string | yes | The open federation. |

## Output

```yaml
tests:
  - test-name: "STR-vs-MEP"
    results: { new: 12, active: 3, reviewed: 0, approved: 5, resolved: 41 }
  - test-name: "ARCH-vs-STR"
    results: { new: 0, active: 0, reviewed: 0, approved: 2, resolved: 18 }
```

## Implementation (Navisworks .NET API · Navisworks Manage)

```csharp
DocumentClashTests dct = Application.ActiveDocument.GetClash().TestsData;
dct.TestsRunAllTests();
foreach (ClashTest test in dct.Tests.OfType<ClashTest>())
{
    // tally test.Children (recurse ClashResultGroup) by ClashResult.Status
}
```

`DocumentClashTests.TestsRunAllTests()` runs all defined tests in one call; then aggregate per-test counts exactly as in `clash.run-test`.

## Gotchas

- **Long-running and synchronous** — `TestsRunAllTests()` blocks (it can freeze the UI thread inside a plug-in for a large federation). Budget time accordingly; this is the heaviest Navisworks verb.
- **Same status-mutation caveat** as `clash.run-test` — snapshot counts after the run.
- **`TestsRunAllTests()` is confirmed by name** (forum + method-page snippet) but its exact signature/overloads are unverified against the SDK reference.
- **Manage-only.**

Sources: [`TestsRunAllTests` method (apidocs, via search)](https://apidocs.co/apps/navisworks/2018/M_Autodesk_Navisworks_Api_Clash_DocumentClashTests_TestsRunAllTests.htm) · [Run a clash test via API — `TestsData.TestsRunAllTests()` (Autodesk forum)](https://forums.autodesk.com/t5/navisworks-api-forum/to-run-a-clash-test-using-api/td-p/6607309)

## See also

- [clash.run-test](./clash.run-test.md) — run a single test
- [clash.list-tests](./clash.list-tests.md) · [clash.export](./clash.export.md)
- [clash-detection-workflow](../skills/clash-detection-workflow.md)
