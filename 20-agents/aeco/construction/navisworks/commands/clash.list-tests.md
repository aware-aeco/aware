# clash.list-tests

List every `ClashTest` defined in the federation, with its type and last-run time. Read-only.

## Inputs

| Field | Type | Required | Description |
|---|---|---|---|
| `document-id` | string | yes | The open federation. |

## Output

```yaml
tests:
  - test-name:   "STR-vs-MEP"
    test-type:   "Hard"
    last-run-at: "2026-05-18T08:14:00Z"
  - test-name:   "Clearance-300"
    test-type:   "Clearance"
    last-run-at: "2026-05-18T08:15:30Z"
```

## Implementation (Navisworks .NET API · Navisworks Manage)

Iterate the defined tests and read each test's type:

```csharp
DocumentClashTests dct = Application.ActiveDocument.GetClash().TestsData;
foreach (ClashTest test in dct.Tests.OfType<ClashTest>())
{
    string name = test.DisplayName;
    ClashTestType type = test.TestType;   // Hard | Hard Conservative | Clearance | Duplicates | TimeBased
}
```

`ClashTest` derives from `GroupItem`; the test collection can contain folders, so recurse. `test-type` comes from the `ClashTestType` enum.

## Gotchas

- **`last-run` property name is unverified.** The Clash Detective tracks a last-run timestamp and it is almost certainly exposed on `ClashTest`, but the exact member name (`LastRun`?) was not confirmed against the SDK reference — verify against the shipped `.chm` before depending on it; emit `null` if absent.
- **`ClashTestType` member spellings partly unverified.** `Clearance` is confirmed; `Hard` / `Duplicates` / `TimeBased` match the UI but the exact enum identifiers should be checked against the reference.
- **TimeBased tests need a TimeLiner sequence** (4D) to be meaningful.
- **Recurse `GroupItem` folders** — tests can be organized in folders.
- **Manage-only.**

Sources: [`ClashTest` type + `TestType`/`GroupItem` (apidocs, via search)](https://apidocs.co/apps/navisworks/2018/T_Autodesk_Navisworks_Api_Clash_ClashTest.htm) · [Clash Detective namespace overview (apidocs, via search)](https://apidocs.co/apps/navisworks/2018/87317537-2911-4c08-b492-6496c82b3ee5.htm)

## See also

- [clash.run-test](./clash.run-test.md) · [clash.run-all](./clash.run-all.md) · [clash.export](./clash.export.md)
- [clash-detection-workflow](../skills/clash-detection-workflow.md)
