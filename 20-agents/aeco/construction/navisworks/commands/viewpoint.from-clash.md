# viewpoint.from-clash

Capture a viewpoint focused on a specific clash result, with the two clashing elements highlighted and other items dimmed — the per-issue screenshot in a BCF export.

Read-only.

## Inputs

| Field | Type | Required | Description |
|---|---|---|---|
| `document-id` | string | yes | The open federation. |
| `test-name` | string | yes | The `ClashTest` containing the result. |
| `clash-result-name` | string | yes | The `ClashResult` to frame. |
| `output-path` | string | yes | PNG output path. |

## Output

```yaml
path:   "C:/issue/clash-007.png"
camera:
  eye:    [12.4, -3.1, 5.0]
  target: [12.4, 0.0, 4.2]
  up:     [0, 0, 1]
```

## Implementation (Navisworks .NET API · Navisworks Manage)

There is **no single API call** that reproduces Clash Detective's "two highlighted, others dimmed" framing — you compose it:

1. Find the `ClashResult` (in `test.Children`); read `Item1`/`Item2` (the two `ModelItem`s) and `Center` / `BoundingBox` / `ViewBounds`.
2. Frame the camera on it: set `Application.ActiveDocument.CurrentViewpoint` (e.g. `Viewpoint.ZoomBox(result.BoundingBox)`), or use the COM `InwOclTestResult.GetSuitableViewPoint()` which returns a viewpoint pre-framed on the two items.
3. Highlight + dim: select `Item1`/`Item2` (`CurrentSelection.CopyFrom(...)`) and dim the rest via `Models.OverridePermanentColor(others, dimColor)` (or hide them).
4. Capture per `viewpoint.capture-current` (`ActiveView.GenerateImage(...)`, 2021+).

## Gotchas

- **The "highlight + dim" rendering is composed, not a single call** — selection + colour/visibility overrides + a framed viewpoint, assembled by hand. It is unverified that any one method reproduces the Clash Detective's exact highlight styling.
- **COM `GetSuitableViewPoint()` is the cleanest framing** for a clash; the .NET `ViewBounds`/`BoundingBox` is the pure-.NET alternative.
- **Same constraints as `viewpoint.capture-current`** — live UI-thread view, `GenerateImage` 2021+.
- **Restore state afterward** — undo the colour/visibility overrides so later nodes see a clean model.
- **Manage-only** (depends on a clash result).

Sources: [xiaodongliang sample — `ClashResult.Center/BoundingBox/ViewBounds` + COM `GetSuitableViewPoint`](https://github.com/xiaodongliang/Forge-Navisworks-ClashTest/blob/master/Navisworks%20Plugin/Class1.cs) · [Viewpoint part 2 — `CurrentViewpoint`/`CopyFrom`/`ZoomBox` (twentytwo)](https://twentytwo.space/2021/06/02/navisworks-api-viewpoint-part-2/) · [AU 2012 handout — `OverridePermanentColor` / `CurrentSelection.CopyFrom`](https://static.au-uw2-prd.autodesk.com/CP2170_handout_2170_au_2012_class_navisworks_simon_bee.pdf)

## See also

- [clash.run-test](./clash.run-test.md) — produces the results this frames
- [clash.export](./clash.export.md) — BCF topics that embed these snapshots
- [viewpoint.capture-current](./viewpoint.capture-current.md) · [viewpoints-and-markup](../skills/viewpoints-and-markup.md)
