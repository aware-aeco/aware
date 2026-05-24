---
name: tekla-plugin-sdk-tekla-structures-plugins-directmanipulation-services-tools-textmanipulation
description: This skill encodes the tekla-plugin-sdk 2026.0 surface of the Tekla.Structures.Plugins.DirectManipulation.Services.Tools.TextManipulation namespace — 6 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: AngleTextManipulator, DimensionTextManipulator, DistanceTextManipulator, TextManipulator, TextManipulatorBase, ValueEnteredEventArgs`1.
---

# Tekla.Structures.Plugins.DirectManipulation.Services.Tools.TextManipulation

Auto-generated from vendor docs for tekla-plugin-sdk 2026.0. 6 types in this namespace.

## AngleTextManipulator (class)

The AngleTextManipulator class defines a manipulator to display and manipulate angle value text.

[Vendor docs](https://developer.tekla.com/documentation/direct-manipulation-api-plugins#T:Tekla.Structures.Plugins.DirectManipulation.Services.Tools.TextManipulation.AngleTextManipulator)

### Constructors
- `AngleTextManipulator(Tekla.Structures.Plugins.DirectManipulation.Core.ManipulationContext manipulationContext, Tekla.Structures.Geometry3d.LineSegment placement, double initialValue)` — Initializes a new instance of the AngleTextManipulator class.

### Properties
- `Value` (double, get/set) — Gets or sets the value of the angle.

### Events
#### `AngleEntered` (`System.EventHandler<Tekla.Structures.Plugins.DirectManipulation.Services.Tools.TextManipulation.ValueEnteredEventArgs<double>>`)

**Signature:** `event System.EventHandler<Tekla.Structures.Plugins.DirectManipulation.Services.Tools.TextManipulation.ValueEnteredEventArgs<double>> AngleEntered`

Invoked when an angle has been entered.

[Docs](https://developer.tekla.com/documentation/direct-manipulation-api-plugins#E%3ATekla.Structures.Plugins.DirectManipulation.Services.Tools.TextManipulation.AngleTextManipulator.AngleEntered)

## DimensionTextManipulator (class)

The DimensionTextManipulator class defines a manipulator to display and manipulate dimension value text.

[Vendor docs](https://developer.tekla.com/documentation/direct-manipulation-api-plugins#T:Tekla.Structures.Plugins.DirectManipulation.Services.Tools.TextManipulation.DimensionTextManipulator)

### Constructors
- `DimensionTextManipulator(Tekla.Structures.Plugins.DirectManipulation.Core.ManipulationContext manipulationContext, Tekla.Structures.Geometry3d.LineSegment placement, Tekla.Structures.Geometry3d.LineSegment initialDimension, Tekla.Structures.Plugins.DirectManipulation.Services.Tools.TextManipulation.DimensionTextManipulator.InputDirections inputDirection)` — Initializes a new instance of the DimensionTextManipulator class.

### Properties
- `Dimension` (Tekla.Structures.Geometry3d.LineSegment, get/set) — Gets or sets the line segment that represents the dimension.
- `InputDirectionArrows` (Tekla.Structures.Plugins.DirectManipulation.Services.Tools.TextManipulation.DimensionTextManipulator.InputDirections, get/set) — Gets or sets the input direction arrows value.

### Events
#### `DimensionEntered` (`System.EventHandler<Tekla.Structures.Plugins.DirectManipulation.Services.Tools.TextManipulation.DimensionTextManipulator.DimensionEnteredEventArgs>`)

**Signature:** `event System.EventHandler<Tekla.Structures.Plugins.DirectManipulation.Services.Tools.TextManipulation.DimensionTextManipulator.DimensionEnteredEventArgs> DimensionEntered`

Invoked when a dimension has been entered.

[Docs](https://developer.tekla.com/documentation/direct-manipulation-api-plugins#E%3ATekla.Structures.Plugins.DirectManipulation.Services.Tools.TextManipulation.DimensionTextManipulator.DimensionEntered)

## DistanceTextManipulator (class)

The DistanceTextManipulator class defines a manipulator to display and manipulate distance value text.

[Vendor docs](https://developer.tekla.com/documentation/direct-manipulation-api-plugins#T:Tekla.Structures.Plugins.DirectManipulation.Services.Tools.TextManipulation.DistanceTextManipulator)

### Constructors
- `DistanceTextManipulator(Tekla.Structures.Plugins.DirectManipulation.Core.ManipulationContext manipulationContext, Tekla.Structures.Geometry3d.LineSegment placement, double initialValue)` — Initializes a new instance of the DistanceTextManipulator class.

### Properties
- `Value` (double, get/set) — Gets or sets the value of the distance.

### Events
#### `DistanceEntered` (`System.EventHandler<Tekla.Structures.Plugins.DirectManipulation.Services.Tools.TextManipulation.ValueEnteredEventArgs<double>>`)

**Signature:** `event System.EventHandler<Tekla.Structures.Plugins.DirectManipulation.Services.Tools.TextManipulation.ValueEnteredEventArgs<double>> DistanceEntered`

Invoked when a distance has been entered.

[Docs](https://developer.tekla.com/documentation/direct-manipulation-api-plugins#E%3ATekla.Structures.Plugins.DirectManipulation.Services.Tools.TextManipulation.DistanceTextManipulator.DistanceEntered)

## TextManipulator (class)

The TextManipulator class defines a manipulator to display and manipulate any text.

[Vendor docs](https://developer.tekla.com/documentation/direct-manipulation-api-plugins#T:Tekla.Structures.Plugins.DirectManipulation.Services.Tools.TextManipulation.TextManipulator)

### Constructors
- `TextManipulator(Tekla.Structures.Plugins.DirectManipulation.Core.ManipulationContext manipulationContext, Tekla.Structures.Geometry3d.LineSegment placement, string initialText)` — Initializes a new instance of the TextManipulator class.

### Properties
- `TextValue` (string, get/set) — Gets or sets the text value.

### Events
#### `TextEntered` (`System.EventHandler<Tekla.Structures.Plugins.DirectManipulation.Services.Tools.TextManipulation.ValueEnteredEventArgs<string>>`)

**Signature:** `event System.EventHandler<Tekla.Structures.Plugins.DirectManipulation.Services.Tools.TextManipulation.ValueEnteredEventArgs<string>> TextEntered`

Invoked when a text has been entered.

[Docs](https://developer.tekla.com/documentation/direct-manipulation-api-plugins#E%3ATekla.Structures.Plugins.DirectManipulation.Services.Tools.TextManipulation.TextManipulator.TextEntered)

## TextManipulatorBase (class)

The TextManipulatorBase class defines a base class for all text manipulators.

[Vendor docs](https://developer.tekla.com/documentation/direct-manipulation-api-plugins#T:Tekla.Structures.Plugins.DirectManipulation.Services.Tools.TextManipulation.TextManipulatorBase)

### Properties
- `Placement` (Tekla.Structures.Geometry3d.LineSegment, get/set) — Gets or sets the placement of the manipulator.
- `Visible` (bool, get/set) — Gets or sets the visibility of the manipulator.

## ValueEnteredEventArgs`1 (class)

The ValueEnteredEventArgs`1 provides the event arguments for when a value has been entered.

[Vendor docs](https://developer.tekla.com/documentation/direct-manipulation-api-plugins#T:Tekla.Structures.Plugins.DirectManipulation.Services.Tools.TextManipulation.ValueEnteredEventArgs`1)

### Constructors
- `ValueEnteredEventArgs`1(TValue value)` — Initializes a new instance of ValueEnteredEventArgs`1.

### Properties
- `Value` (TValue, get) — Gets the entered value.

