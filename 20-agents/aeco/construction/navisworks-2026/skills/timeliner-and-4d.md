---
name: navisworks-timeliner-and-4d
description: This skill should be used when composing snippets that work with Navisworks's TimeLiner — the 4D-simulation engine that links construction tasks to model items. Encodes the Task / Activity / Resource hierarchy, the data-source vs internal-task distinction, the Selection / Search Set binding to tasks, the simulation/playback API, and the gotcha that TimeLiner state is not part of the main document save — it lives in a separate TimeLiner XML appended to the .nwf.
---

# Navisworks TimeLiner and 4D

TimeLiner is Navisworks's 4D-simulation engine: it links scheduled tasks (from MS Project, Primavera, CSV, or hand-typed) to selection sets of model items, then plays back construction over time with colored visualization.

## The data model

| Concept | What | Source |
|---|---|---|
| Task | One row of work: name + start + end + type + bound items | Imported from a data source OR hand-entered |
| Task Type | Construct / Demolish / Temporary | Affects coloring during simulation |
| Activity | Internal Navisworks alias for Task | Same as Task in modern API |
| Resource | Cost / labor / equipment hooked to a task | Mostly used for cost-loaded schedules |
| Data Source | Connection to an external scheduling tool | MS Project file, Primavera DB, CSV, or "internal" |
| Selection | The model items bound to a task | Selection Set OR Search Set OR explicit GUID list |

## The data-source binding

A TimeLiner task EITHER:
- Has bound items via `task.Selection` (Selection/Search Set reference OR GUID list) — links task to model
- OR is "unattached" — just a timeline entry with no model link (rare)

Most production TimeLiner setups bind tasks to Search Sets that match by an "Activity ID" property on the model items. The data source (MS Project) supplies the Activity ID; a Search Set rule matches model items by that property; binding the Search Set to the task links the data row to the geometry.

## Listing tasks

```csharp
using Autodesk.Navisworks.Api.Interop.LcOpTimelineApi;   // older COM-backed API

var doc = Autodesk.Navisworks.Api.Application.ActiveDocument;

// Get the TimeLiner manager
var timelinerPlugin = doc.GetPlugins().FirstOrDefault(p => p.Name == "TimeLiner");
// ... walk down to the tasks collection (API path varies by Navisworks version) ...
```

The TimeLiner API has changed substantially across Navisworks versions; the modern .NET surface is partial. For reliable cross-version code, use the COM bridge.

## Task properties

| Property | Type | Notes |
|---|---|---|
| `DisplayName` | string | Visible in the TimeLiner panel |
| `PlannedStart` / `PlannedEnd` | DateTime | The baseline dates |
| `ActualStart` / `ActualEnd` | DateTime? | Recorded actuals (null = not yet recorded) |
| `TaskType` | enum | Construct / Demolish / Temporary |
| `ActivityID` | string | The link key against the data source |
| `Selection` | SelectionSetReference | What the task touches |
| `UserName` | string | Who's responsible |

## Refreshing from data source

When the MS Project file is updated, click "Refresh" in the TimeLiner panel — or invoke `timelinerPlugin.Refresh()` programmatically. This rules-merges: new tasks from the file are added; existing tasks update their dates; removed tasks become "obsolete" but aren't auto-deleted (so you don't lose bindings).

## Simulation playback

Programmatic simulation control is limited; most automation runs simulation via UI commands. Useful read API:

```csharp
// Read the current simulation date
var currentDate = timelinerPlugin.SimulationCurrentDate;

// Set a specific date
timelinerPlugin.SimulationCurrentDate = new DateTime(2026, 8, 15);
doc.Refresh();    // pump UI to apply
```

The simulation runs through the visualizer; capturing simulation frames as images requires `doc.WriteImage(...)` per simulation date.

## Common gotchas

- **TimeLiner state lives in a separate .xml-like blob inside the .nwf.** Saving the .nwf saves TimeLiner; saving as .nwd embeds TimeLiner into the cache. Loading a .nwd opens TimeLiner read-only.
- **Task GUIDs are NOT stable across MS Project refresh.** Match by ActivityID, not internal GUID.
- **Refreshing the data source overwrites bindings.** If a manual binding was added on a task that the refresh treats as new, the binding is lost. Establish bindings via Search Set rules, not per-task manual selection.
- **Simulation playback runs on Navisworks's main thread.** Blocking the thread (long-running scripts) freezes the visualization. Yield via `doc.Refresh()` between operations.

## Standard pattern

```csharp
var doc = Application.ActiveDocument;
var timeliner = doc.GetPlugins().FirstOrDefault(p => p.Name == "TimeLiner");

// (API varies — pseudocode)
var tasks = timeliner.GetTasks();
var rows = tasks.Select(t => new {
    name = t.DisplayName,
    activity_id = t.ActivityID,
    planned_start = t.PlannedStart.ToString("o"),
    planned_end = t.PlannedEnd.ToString("o"),
    task_type = t.TaskType.ToString(),
    item_count = t.Selection?.Items?.Count() ?? 0,
}).ToList();

return new { task_count = rows.Count, tasks = rows };
```

## See also

- [selection-and-search-sets](./selection-and-search-sets.md) — what tasks bind against
- [clash-detection-workflow](./clash-detection-workflow.md) — clashes flagged for 4D-aware rescheduling
