---
name: tekla-tekla-structures-analysis-operations
description: This skill encodes the tekla 2025.0 surface of the Tekla.Structures.Analysis.Operations namespace — 1 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: Operation.
---

# Tekla.Structures.Analysis.Operations

Auto-generated from vendor docs for tekla 2025.0. 1 types in this namespace.

## Operation (class)

Operations of analysis objects.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/babef861-9636-cd4d-67fd-a987b73de719)

### Constructors
- `Operation(...)` — Initializes a new instance of the Operation class

### Methods
#### `MoveAnalysisPartAreaEdge(...)`

Moves the analysis part area edge by translation vector. Perpendicular offset is calculated from the translation vector.

[Docs](https://developer.tekla.com/topic/en/18/43/e21f7feb-4843-676b-29f0-76dcda52948a)

#### `MoveAnalysisPartPosition(...)`

Moves the analysis part position by translation vector. Only the handle positions can be moved.

[Docs](https://developer.tekla.com/topic/en/18/43/8d745ca6-6ff9-ab60-b76b-a4383a09315b)

#### `MoveObject(...)`

Moves the analysis object by translation vector. Supported objects are: AnalysisNode, AnalysisPart.

[Docs](https://developer.tekla.com/topic/en/18/43/2d4b3006-74d0-17d2-316f-1f10d028d0a6)

#### `ResetAnalysisPartPositions(...)`

Reset analysis part. Removes the offsets applied to node positions. The axis offset of the analysis part is not reset.

[Docs](https://developer.tekla.com/topic/en/18/43/790333a7-df91-ceaa-caf7-be3183a0f547)

