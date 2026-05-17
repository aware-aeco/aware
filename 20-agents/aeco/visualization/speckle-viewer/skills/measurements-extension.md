---
name: viewer-measurements-extension
description: MeasurementsExtension declarations from viewer
---

# MeasurementsExtension

## Methods

- `on(eventType: T, listener: (arg: MeasurementEventPayload[T]) => void)`
- `onLateUpdate()`
- `onResize()`
- `onPointerMove()`
- `onPointerClick()`
- `onPointerDoubleClick()`
- `autoLazerMeasure(data: Vector2)`
- `startMeasurement()`
- `cancelMeasurement()`
- `finishMeasurement()`
- `pushMeasurement(measurement: Measurement)`
- `findMeasurementFromData(measurementData: MeasurementData)`
- `removeMeasurement(measurementData?: MeasurementData)`
- `setMeasurements(measurements: MeasurementData[])`
- `clearMeasurements()`
- `flashMeasurement()`
- `pickMeasurement(data: Vector2)`
- `selectMeasurement(measurement: Measurement, value: boolean)`
- `snap(intersection: ExtendedMeshIntersection, outPoint: Vector3, outNormal: Vector3)`
- `updateClippingPlanes(planes: Plane[])`
- `applyOptions()`
- `addMeasurement(measurementData: MeasurementData)`
- `toMeasurementData()`
