---
name: tekla-tekla-structures-direct-manipulation
description: This skill encodes the tekla 2026.0 Direct Manipulation API for plugins — the in-model handle/picking/graphics loop that lets a plugin be created and edited by dragging in the 3D view instead of through a dialog. Covers Tekla.Structures.Plugins.DirectManipulation.Core (PluginCreationFeatureBase, PluginManipulationFeatureBase, ManipulationContext, IHandleManager) and .Services (PickingTool, PointHandle, IGraphicsDrawer). This namespace is NOT in the scraped Open API reference tree, so it is hand-authored from the official TSDirectManipulationPluginsExample (2026 branch) + the Direct Manipulation API guide. Read when wiring live drag-handles, rubber-band creation previews, or contextual toolbars onto a plugin — the "Custom Component 2.0" manipulation layer.
---

# Tekla Direct Manipulation API (plugins)

**A plugin gets a *creation feature* (how it is first placed, with live preview) and a *manipulation feature* (in-model handles that edit it after placement). Both are attached to the plugin by name and run inside the Direct Manipulation Platform — "what you see is what you get," no dialog required.**

> **Why this doc is hand-authored.** The `Tekla.Structures.Plugins.DirectManipulation.*` namespaces ship in a separate NuGet (`Tekla.Structures.Plugins.DirectManipulation`, net48) and are **not** present in the scraped `developer.tekla.com/doc/tekla-structures/2026` reference tree that generated the rest of this agent. The surface below is verified against real working code — [TSDirectManipulationPluginsExample @ branch `2026`](https://github.com/TrimbleSolutionsCorporation/TSDirectManipulationPluginsExample/tree/master/2026_DirectManipulationPlugins.Examples) — plus the [Direct Manipulation API guide](https://developer.tekla.com/documentation/direct-manipulation-api-plugins).

## The package + namespace map

NuGet: `Tekla.Structures.Plugins.DirectManipulation` (2026.x) → depends on `Tekla.BIM.DirectManipulation` + `TeklaFusion`. **net48 only.**

| Namespace | Versioning | Key types |
|---|---|---|
| `…DirectManipulation.Core` | **Stable** — code here survives version bumps | `ManipulationContext`, `IHandleManager` |
| `…DirectManipulation.Core.Features` | Stable | `PluginCreationFeatureBase`, `PluginManipulationFeatureBase` |
| `…DirectManipulation.Services.Tools.Picking` | Evolving | `PickingTool`, `InputRange`, `InputTypes`, `ToleratedObjectEventArgs`, `InputValidationEventArgs` |
| `…DirectManipulation.Services.Handles` | Evolving | `PointHandle`, `HandleLocationType`, `HandleEffectType`, `DragEventArgs` |
| `…DirectManipulation.Services.Utilities` | Evolving | graphics/geometry helpers |
| `…DirectManipulation.*.Internal` | **Never reference** | Trimble implementation details |

**Rule:** prefer `Core` types in any code meant to last. `Services` gives you the ergonomic tools (picking, handles) but may shift between releases.

## How a plugin opts in

The plugin itself is an ordinary `PluginBase` (see [tekla-structures-plugins.md](./tekla-structures-plugins.md)). Direct Manipulation is added by writing **two feature classes in a separate assembly**, each constructed with the plugin's registered name so the DM Platform can bind them:

```csharp
public class BeamPluginCreationFeature : PluginCreationFeatureBase
{
    public BeamPluginCreationFeature() : base(BeamPlugin.PluginName) { }
}

public class BeamPluginManipulationFeature : PluginManipulationFeatureBase
{
    public BeamPluginManipulationFeature() : base(BeamPlugin.PluginName) { }
}
```

Deploy the feature DLL alongside the plugin under `environments/common/extensions/`; the plugin is launched from the **Applications & components** pane.

## 1 · Creation feature — pick + live preview + commit

`PluginCreationFeatureBase` overrides `Initialize()` (start a picking session) and `Refresh()` (reset transient state). The work happens in `PickingTool` event handlers.

```csharp
protected override void Initialize()
{
    pickingTool = CreatePickingTool(InputRange.AtMost(2), InputTypes.Point);
    pickingTool.PreviewRequested        += OnPreviewRequested;   // rubber-band redraw
    pickingTool.ObjectPicked            += OnObjectPicked;       // a point/object was committed
    pickingTool.InputValidationRequested+= OnInputValidationRequested; // keep session open?
    pickingTool.PickSessionEnded        += OnPickEnded;          // enough input → build
    pickingTool.PickUndone              += OnPickingUndone;      // user pressed undo mid-pick
    pickingTool.StartPickingSession("Pick two points.");
}
```

- **Live preview** runs every mouse-move via `PreviewRequested`. Read the in-progress attribute values with `this.Component.GetAppliedAttributes()` (a `Dictionary<string,object>`), then draw with the feature's `Graphics` drawer:
  ```csharp
  this.Graphics.Clear();
  this.Graphics.DrawProfile(profile,
      new LineSegment(lastPt, lastPt + lengthVector),
      new Vector(0, 0, -150), rotationInDegrees: 90);
  ```
  `eventArgs.HitPoint` (a `Point`) is the current cursor position; `eventArgs.IsValid` gates bad picks. (`ToleratedObjectEventArgs`.)
- **Keep the session going** until enough points: set `eventArgs.ContinueSession = true` in `InputValidationRequested` (`InputValidationEventArgs`). Note: this has no effect once the user *interrupts* the session.
- **Commit** on `PickSessionEnded` by handing points to a `ComponentInput` and calling `CommitComponentInput` — this is what actually instantiates the plugin in the model:
  ```csharp
  var input = new ComponentInput();
  input.AddInputPolygon(new Polygon { Points = new ArrayList(pickedPoints) });
  CommitComponentInput(input);
  pickingTool.StartPickingSession("Pick again to insert another.");
  ```

## 2 · Manipulation feature — handles that edit the placed plugin

`PluginManipulationFeatureBase` yields one or more `ManipulationContext` per component:

```csharp
protected override IEnumerable<ManipulationContext> AttachManipulationContexts(Component component)
{
    yield return new BeamPluginManipulationContext(component, this);
}
```

A `ManipulationContext` owns the handles. The pattern is **create handles from current input → on drag-end, write input back**:

```csharp
public sealed class BeamPluginManipulationContext : ManipulationContext
{
    private readonly IHandleManager handleManager;   // from feature.HandleManager
    private readonly List<PointHandle> pointHandles;

    public BeamPluginManipulationContext(Component component, PluginManipulationFeatureBase feature)
        : base(component, feature)
    {
        handleManager = feature.HandleManager;
        pointHandles = CreatePointHandles(component);   // one per input point
        pointHandles.ForEach(h => { h.DragOngoing += OnDragOngoing; h.DragEnded += OnDragEnded; });
    }

    public override void UpdateContext()                 // model changed elsewhere → resync handles
        => UpdatePointHandles(Component, pointHandles);

    private PointHandle MakeHandle(Point p) =>
        handleManager.CreatePointHandle(p, HandleLocationType.InputPoint, HandleEffectType.Geometry);
}
```

- A `PointHandle` exposes `.Point` and the `DragOngoing` / `DragEnded` events (`DragEventArgs`). On `DragEnded`, read every handle's `.Point` and push the new positions back into the component.
- Always `Dispose()` handles in the context's `Dispose(bool)` override, and detach the drag handlers — leaked handles paint ghosts in the view.

## 3 · The ComponentInput round-trip (the load-bearing pattern)

Both features speak `ComponentInput` / `InputItem`. To **edit** an existing plugin you must read its current input, swap the geometry, and re-apply — preserving object inputs while moving point inputs:

```csharp
foreach (InputItem item in Component.GetComponentInput())
{
    switch (item.GetInputType())            // InputItem.InputTypeEnum
    {
        case InputItem.InputTypeEnum.INPUT_1_OBJECT:  input.AddInputObject(item.GetData() as ModelObject); break;
        case InputItem.InputTypeEnum.INPUT_N_OBJECTS: input.AddInputObjects(item.GetData() as ArrayList);  break;
        case InputItem.InputTypeEnum.INPUT_1_POINT:   input.AddOneInputPosition(points[i++]);               break;
        case InputItem.InputTypeEnum.INPUT_2_POINTS:  input.AddTwoInputPositions(points[i], points[i+1]); i += 2; break;
        case InputItem.InputTypeEnum.INPUT_POLYGON:   input.AddInputPolygon(BuildPolygon(points));          break;
    }
}
ModifyComponentInput(input);   // creation uses CommitComponentInput; manipulation uses ModifyComponentInput
```

`Commit…` creates; `Modify…` re-runs the existing instance. Confusing the two is the classic "my drag created a duplicate beam" bug.

## 4 · Contextual toolbar (optional)

A small toolbar that appears when the object is selected. Opt in via the `PluginCreationFeatureBase` constructor flag `useFeatureContextualToolBar: true` and override `DefineFeatureContextualToolbar(IToolbar toolbar)`. Controls are created from `IToolbar` factory methods and all derive from `ControlBase`. Use it for discrete actions (flip, cycle profile) that don't warrant a handle. Wiring control state back to the plugin is the "Communication" part of the guide.

## Where this fits the plugin-forge

This is the **Phase 4 (manipulate)** target of the generator: from a confirmed parameter set, emit a creation feature (picking + `DrawProfile` preview) and a manipulation context (one `PointHandle` per geometric driver, `DragEnded → ModifyComponentInput`). Geometric params → handles; scalar params → contextual-toolbar controls or the WPF dialog. See [[tekla-plugin-forge]].

## Source

- Verified code: [TSDirectManipulationPluginsExample @ `2026`](https://github.com/TrimbleSolutionsCorporation/TSDirectManipulationPluginsExample/tree/master/2026_DirectManipulationPlugins.Examples) — `BasicPluginFeatures/` (BeamPlugin creation + manipulation) and `ComplexTransitionSectionPluginFeatures/` (multi-context, `DataFetcher`).
- Guide: [Direct Manipulation API for plugins](https://developer.tekla.com/documentation/direct-manipulation-api-plugins) (4 parts: creation feature, contextual toolbar, manipulation feature, communication).
- **Full reflected reference** (every public type/method/property with Trimble's own summaries) is the sibling agent **`tekla-plugin-sdk-2026`** — generated from the `Tekla.Structures.Plugins.DirectManipulation` 2026.0.3 NuGet via the `tekla-plugin-sdk` extractor. This doc is the lifecycle narrative; that agent is the exhaustive API surface.
