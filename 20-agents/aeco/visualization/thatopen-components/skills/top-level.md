---
name: components-top-level
description: TopLevel declarations from components
---

# TopLevel

## Methods

- `angleDimensionMachine(state: AngleAnnotationState, event: AngleAnnotationEvent)`
- `buildAnglePositions(dim: AngleAnnotation, style: AngleAnnotationStyle)`
- `buildAnglePreviewPositions(pointA: THREE.Vector3, vertex: THREE.Vector3, pointB: THREE.Vector3, cursor: THREE.Vector3 | null, style: AngleAnnotationStyle, flipped?: boolean)`
- `buildCalloutPositions(ann: CalloutAnnotation, style: CalloutAnnotationStyle)`
- `buildCalloutPreviewPositions(kind: "awaitingRadius" | "awaitingElbow" | "awaitingExtension", center: THREE.Vector3, halfW: number, halfH: number, elbow: THREE.Vector3 | null, cursor: THREE.Vector3 | null, style: CalloutAnnotationStyle)`
- `buildDimensionPositions(dim: LinearAnnotation, style: LinearAnnotationStyle)`
- `buildDimensions(points: THREE.Vector3[], offset: number)`
- `buildLeaderPositions(ann: LeaderAnnotation, style: LeaderAnnotationStyle)`
- `buildLeaderPreviewPositions(kind: "placingElbow" | "placingExtension", arrowTip: THREE.Vector3, elbow: THREE.Vector3 | null, cursor: THREE.Vector3 | null, style: LeaderAnnotationStyle)`
- `buildPreviewPositions(kind: "placingPoints" | "positioningOffset", points: THREE.Vector3[], cursor: THREE.Vector3 | null, style: LinearAnnotationStyle)`
- `buildSlopePositions(ann: SlopeAnnotation, style: SlopeAnnotationStyle)`
- `calloutAnnotationMachine(state: CalloutAnnotationState, event: CalloutAnnotationEvent)`
- `computeAlignmentMatrix(drawingPoints: THREE.Vector3[], worldPoints: THREE.Vector3[])`
- `computeAngle(dim: AngleAnnotation)`
- `computeBisectorAngle(dim: AngleAnnotation)`
- `computeOffset(points: THREE.Vector3[], cursor: THREE.Vector3)`
- `formatSlope(slope: number, format: SlopeFormat)`
- `getAngleTickEndpoints(dim: AngleAnnotation)`
- `getDimensionTickEndpoints(dim: LinearAnnotation)`
- `getSlopeTip(ann: SlopeAnnotation, length: number)`
- `leaderAnnotationMachine(state: LeaderAnnotationState, event: LeaderAnnotationEvent)`
- `linearDimensionMachine(state: LinearAnnotationState, event: LinearAnnotationEvent)`
