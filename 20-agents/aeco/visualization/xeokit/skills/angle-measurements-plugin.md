---
name: xeokit-sdk-angle-measurements-plugin
description: AngleMeasurementsPlugin declarations from xeokit-sdk
---

# AngleMeasurementsPlugin

## Methods

- `createMeasurement()`
- `destroyMeasurement(id: string)`
- `clear()`
- `on(event: "mouseOver", callback: (event: AngleMeasurementMouseOverEvent)=> void)`
- `on(event: "mouseLeave", callback: (event: AngleMeasurementMouseLeaveEvent)=> void)`
- `on(event: "contextMenu", callback: (measurement: AngleMeasurement)=> void)`
- `on(event: "measurementStart", callback: (measurement: AngleMeasurement)=> void)`
- `on(event: "measurementCreated", callback: (measurement: AngleMeasurement)=> void)`
- `on(event: "measurementDestroyed", callback: (measurement: AngleMeasurement)=> void)`
