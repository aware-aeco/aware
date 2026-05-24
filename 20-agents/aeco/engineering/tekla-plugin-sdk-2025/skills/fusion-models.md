---
name: tekla-plugin-sdk-fusion-models
description: This skill encodes the tekla-plugin-sdk 2025.0 surface of the Fusion.Models namespace — 11 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: Entity, EntityGroup, EntityPickedCallbackDelegate, EntityPickingCallbackDelegate, EntityState, EntityStates, EntityType, Geometry, and 3 more types.
---

# Fusion.Models

Auto-generated from vendor docs for tekla-plugin-sdk 2025.0. 11 types in this namespace.

## Entity (class)

Entity class in Fusion.Models.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.Models.Entity)

### Constructors
- `Entity(System.Guid identifier, Fusion.Models.EntityType entityType, Fusion.Models.EntityGroup entityGroup)` — Constructs a new Entity.

### Methods
#### `bool Equals(Fusion.Models.Entity other)`

Equals method of Entity.

**Parameters:**
- `other` (Fusion.Models.Entity)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Models.Entity.Equals%28Fusion.Models.Entity%29)

#### `bool Equals(object other)`

Equals method of Entity.

**Parameters:**
- `other` (object)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Models.Entity.Equals%28System.Object%29)

#### `int GetHashCode()`

GetHashCode method of Entity.

**Returns:** `int` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Models.Entity.GetHashCode)

#### `string ToString()`

ToString method of Entity.

**Returns:** `string` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Models.Entity.ToString)

### Properties
- `EntityGroup` (Fusion.Models.EntityGroup, get) — EntityGroup property of Entity.
- `EntityType` (Fusion.Models.EntityType, get) — EntityType property of Entity.
- `Geometries` (System.Collections.Generic.IEnumerable<Fusion.Models.Geometry>, get) — Geometries property of Entity.
- `Identifier` (System.Guid, get) — Identifier property of Entity.

## EntityGroup (class)

EntityGroup class in Fusion.Models.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.Models.EntityGroup)

### Constructors
- `EntityGroup(System.Guid identifier)` — Constructs a new EntityGroup.

### Methods
#### `bool Equals(Fusion.Models.EntityGroup other)`

Equals method of EntityGroup.

**Parameters:**
- `other` (Fusion.Models.EntityGroup)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Models.EntityGroup.Equals%28Fusion.Models.EntityGroup%29)

#### `bool Equals(object other)`

Equals method of EntityGroup.

**Parameters:**
- `other` (object)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Models.EntityGroup.Equals%28System.Object%29)

#### `int GetHashCode()`

GetHashCode method of EntityGroup.

**Returns:** `int` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Models.EntityGroup.GetHashCode)

#### `string ToString()`

ToString method of EntityGroup.

**Returns:** `string` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Models.EntityGroup.ToString)

### Properties
- `Entities` (System.Collections.Generic.IEnumerable<Fusion.Models.Entity>, get) — Entities property of EntityGroup.
- `Identifier` (System.Guid, get) — Identifier property of EntityGroup.

## EntityPickedCallbackDelegate (delegate)

EntityPickedCallbackDelegate delegate in Fusion.Models.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.Models.EntityPickedCallbackDelegate)

### Constructors
- `EntityPickedCallbackDelegate(object object, nint method)` — Constructs a new EntityPickedCallbackDelegate.

## EntityPickingCallbackDelegate (delegate)

EntityPickingCallbackDelegate delegate in Fusion.Models.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.Models.EntityPickingCallbackDelegate)

### Constructors
- `EntityPickingCallbackDelegate(object object, nint method)` — Constructs a new EntityPickingCallbackDelegate.

## EntityState (enum)

EntityState enum in Fusion.Models.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.Models.EntityState)

### Values
- `Normal` = `0`
- `Selected` = `1`
- `Grayed` = `2`
- `Ghosted` = `3`

## EntityStates (class)

EntityStates class in Fusion.Models.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.Models.EntityStates)

### Constructors
- `EntityStates()` — Constructs a new EntityStates.

### Methods
#### `void Clear()`

Clear method of EntityStates.

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Models.EntityStates.Clear)

### Properties
- `DefaultState` (Fusion.Models.EntityState, get/set) — DefaultState property of EntityStates.
- `Item` (Fusion.Models.EntityState, get/set) — Item property of EntityStates.

### Events
#### `EntityStatesChanged` (`System.EventHandler`)

**Signature:** `event System.EventHandler EntityStatesChanged`

EntityStatesChanged event of EntityStates.

[Docs](https://www.nuget.org/packages/TeklaFusion#E%3AFusion.Models.EntityStates.EntityStatesChanged)

## EntityType (enum)

EntityType enum in Fusion.Models.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.Models.EntityType)

### Values
- `Unknown` = `0`
- `Product` = `1`
- `Annotation` = `2`
- `Element` = `3`
- `BuildingElement` = `4`
- `Beam` = `5`
- `BuildingElementComponent` = `6`
- `BuildingElementPart` = `7`
- `ReinforcingElement` = `8`
- `ReinforcingBar` = `9`
- `ReinforcingMesh` = `10`
- `Tendon` = `11`
- `TendonAnchor` = `12`
- `BuildingElementProxy` = `13`
- `Column` = `14`
- `Covering` = `15`
- `CurtainWall` = `16`
- `Door` = `17`
- `Footing` = `18`
- `Member` = `19`
- `Pile` = `20`
- `Plate` = `21`
- `Railing` = `22`
- `Ramp` = `23`
- `RampFlight` = `24`
- `Roof` = `25`
- `Slab` = `26`
- `Stair` = `27`
- `StairFlight` = `28`
- `Wall` = `29`
- `WallStandardCase` = `30`
- `Window` = `31`
- `DistributionElement` = `32`
- `DistributionControlElement` = `33`
- `DistributionFlowElement` = `34`
- `DistributionChamberElement` = `35`
- `EnergyConversionDevice` = `36`
- `FlowController` = `37`
- `ElectricDistributionPoint` = `38`
- `FlowFitting` = `39`
- `FlowMovingDevice` = `40`
- `FlowSegment` = `41`
- `FlowStorageDevice` = `42`
- `FlowTerminal` = `43`
- `FlowTreatmentDevice` = `44`
- `ElectricalElement` = `45`
- `ElementAssembly` = `46`
- `ElementComponent` = `47`
- `DiscreteAccessory` = `48`
- `Fastener` = `49`
- `MechanicalFastener` = `50`
- `EquipmentElement` = `51`
- `FeatureElement` = `52`
- `FeatureElementAddition` = `53`
- `ProjectionElement` = `54`
- `FeatureElementSubtraction` = `55`
- `EdgeFeature` = `56`
- `ChamferEdgeFeature` = `57`
- `RoundedEdgeFeature` = `58`
- `OpeningElement` = `59`
- `FurnishingElement` = `60`
- `TransportElement` = `61`
- `VirtualElement` = `62`
- `Grid` = `63`
- `Port` = `64`
- `DistributionPort` = `65`
- `Proxy` = `66`
- `SpatialStructureElement` = `67`
- `Building` = `68`
- `BuildingStorey` = `69`
- `Site` = `70`
- `Space` = `71`
- `StructuralActivity` = `72`
- `StructuralAction` = `73`
- `StructuralLinearAction` = `74`
- `StructuralLinearActionVarying` = `75`
- `StructuralPlanarAction` = `76`
- `StructuralPlanarActionVarying` = `77`
- `StructuralPointAction` = `78`
- `StructuralReaction` = `79`
- `StructuralPointReaction` = `80`
- `StructuralItem` = `81`
- `StructuralConnection` = `82`
- `StructuralCurveConnection` = `83`
- `StructuralPointConnection` = `84`
- `StructuralSurfaceConnection` = `85`
- `StructuralMember` = `86`
- `StructuralCurveMember` = `87`
- `StructuralCurveMemberVarying` = `88`
- `StructuralSurfaceMember` = `89`
- `StructuralSurfaceMemberVarying` = `90`
- `Project` = `91`
- `Group` = `92`
- `System` = `93`
- `Zone` = `94`
- `ElectricalCircuit` = `95`

## Geometry (class)

Geometry class in Fusion.Models.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.Models.Geometry)

### Constructors
- `Geometry(Fusion.Models.Entity entity, Fusion.Models.Material material, System.Collections.Generic.IReadOnlyList<Fusion.Float3> points, System.Collections.Generic.IReadOnlyList<Fusion.UShort3> faces)` — Constructs a new Geometry.

### Methods
#### `bool Equals(Fusion.Models.Geometry other)`

Equals method of Geometry.

**Parameters:**
- `other` (Fusion.Models.Geometry)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Models.Geometry.Equals%28Fusion.Models.Geometry%29)

#### `bool Equals(object other)`

Equals method of Geometry.

**Parameters:**
- `other` (object)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Models.Geometry.Equals%28System.Object%29)

#### `int GetHashCode()`

GetHashCode method of Geometry.

**Returns:** `int` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Models.Geometry.GetHashCode)

### Properties
- `BoundingBox` (Fusion.BoundingBox, get) — BoundingBox property of Geometry.
- `Entity` (Fusion.Models.Entity, get) — Entity property of Geometry.
- `Material` (Fusion.Models.Material, get) — Material property of Geometry.
- `Normals` (System.Collections.Generic.IReadOnlyList<Fusion.Float3>, get) — Normals property of Geometry.

## IModelView (interface)

IModelView interface in Fusion.Models.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.Models.IModelView)

### Properties
- `EntityPickedCallback` (Fusion.Models.EntityPickedCallbackDelegate, get/set) — EntityPickedCallback property of IModelView.
- `EntityPickingCallback` (Fusion.Models.EntityPickingCallbackDelegate, get/set) — EntityPickingCallback property of IModelView.
- `EntityStates` (Fusion.Models.EntityStates, get/set) — EntityStates property of IModelView.
- `IsPickingEnabled` (bool, get/set) — IsPickingEnabled property of IModelView.
- `Model` (Fusion.Models.Model, get/set) — Model property of IModelView.

## Material (class)

Material class in Fusion.Models.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.Models.Material)

### Constructors
- `Material(Fusion.RGBA color, bool doubleSided, bool enableLighting)` — Constructs a new Material.

### Methods
#### `bool Equals(Fusion.Models.Material other)`

Equals method of Material.

**Parameters:**
- `other` (Fusion.Models.Material)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Models.Material.Equals%28Fusion.Models.Material%29)

#### `bool Equals(object other)`

Equals method of Material.

**Parameters:**
- `other` (object)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Models.Material.Equals%28System.Object%29)

#### `int GetHashCode()`

GetHashCode method of Material.

**Returns:** `int` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Models.Material.GetHashCode)

### Properties
- `Color` (Fusion.RGBA, get) — Color property of Material.
- `DoubleSided` (bool, get) — DoubleSided property of Material.
- `EnableLighting` (bool, get) — EnableLighting property of Material.

## Model (class)

Model class in Fusion.Models.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.Models.Model)

### Constructors
- `Model(System.Guid identifier, System.Collections.Generic.IReadOnlyList<Fusion.Models.EntityGroup> entityGroups, System.Collections.Generic.IReadOnlyList<Fusion.Models.Entity> entities, System.Collections.Generic.IReadOnlyList<Fusion.Models.Material> materials, System.Collections.Generic.IReadOnlyList<Fusion.Models.Geometry> geometries)` — Constructs a new Model.

### Methods
#### `bool Equals(Fusion.Models.Model other)`

Equals method of Model.

**Parameters:**
- `other` (Fusion.Models.Model)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Models.Model.Equals%28Fusion.Models.Model%29)

#### `bool Equals(object other)`

Equals method of Model.

**Parameters:**
- `other` (object)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Models.Model.Equals%28System.Object%29)

#### `static Fusion.Models.Model FromRawbim(System.Guid identifier, System.IO.Stream source, System.Threading.CancellationToken cancellationToken = null)`

FromRawbim method of Model.

**Parameters:**
- `identifier` (System.Guid)
- `source` (System.IO.Stream)
- `cancellationToken` (System.Threading.CancellationToken)

**Returns:** `Fusion.Models.Model` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Models.Model.FromRawbim%28System.Guid%2CSystem.IO.Stream%2CSystem.Threading.CancellationToken%29)

#### `static Fusion.Models.Model FromStream(System.Guid identifier, System.IO.Stream stream, System.Threading.CancellationToken cancellationToken = null)`

FromStream method of Model.

**Parameters:**
- `identifier` (System.Guid)
- `stream` (System.IO.Stream)
- `cancellationToken` (System.Threading.CancellationToken)

**Returns:** `Fusion.Models.Model` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Models.Model.FromStream%28System.Guid%2CSystem.IO.Stream%2CSystem.Threading.CancellationToken%29)

#### `int GetHashCode()`

GetHashCode method of Model.

**Returns:** `int` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Models.Model.GetHashCode)

#### `void ToStream(System.IO.Stream stream, System.Threading.CancellationToken cancellationToken = null)`

ToStream method of Model.

**Parameters:**
- `stream` (System.IO.Stream)
- `cancellationToken` (System.Threading.CancellationToken)

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Models.Model.ToStream%28System.IO.Stream%2CSystem.Threading.CancellationToken%29)

#### `string ToString()`

ToString method of Model.

**Returns:** `string` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Models.Model.ToString)

### Properties
- `BoundingBox` (Fusion.BoundingBox, get) — BoundingBox property of Model.
- `Entities` (System.Collections.Generic.IReadOnlyList<Fusion.Models.Entity>, get) — Entities property of Model.
- `EntityGroups` (System.Collections.Generic.IReadOnlyList<Fusion.Models.EntityGroup>, get) — EntityGroups property of Model.
- `Geometries` (System.Collections.Generic.IReadOnlyList<Fusion.Models.Geometry>, get) — Geometries property of Model.
- `Identifier` (System.Guid, get) — Identifier property of Model.
- `Materials` (System.Collections.Generic.IReadOnlyList<Fusion.Models.Material>, get) — Materials property of Model.

