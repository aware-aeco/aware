---
name: xeokit-sdk-distance-measurements-plugin
description: DistanceMeasurementsPlugin declarations from xeokit-sdk
---

# DistanceMeasurementsPlugin

## Methods

- `createMeasurement()`
- `getAxisVisible()`
- `setAxisVisible(axisVisible: boolean)`
- `destroyMeasurement(id: string)`
- `clear()`
- `on(event: "mouseOver", callback: (event: DistanceMeasurementMouseOverEvent)=> void)`
- `on(event: "mouseLeave", callback: (event: DistanceMeasurementMouseLeaveEvent)=> void)`
- `on(event: "contextMenu", callback: (event: DistanceMeasurementMouseContextMenuEvent)=> void)`
- `on(event: "measurementCreated", callback: (measurement: DistanceMeasurement)=> void)`
- `on(event: "measurementStart", callback: (measurement: DistanceMeasurement)=> void)`
- `on(event: "measurementEnd", callback: (measurement: DistanceMeasurement)=> void)`
- `on(event: "measurementCancel", callback: (measurement: DistanceMeasurement)=> void)`
- `on(event: "measurementDestroyed", callback: (measurement: DistanceMeasurement)=> void)`
