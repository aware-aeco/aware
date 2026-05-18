---
name: tekla-tekla-structures-analysis
description: This skill encodes the tekla 2026.0 surface of the Tekla.Structures.Analysis namespace — 121 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: Analysis, AnalysisArea, AnalysisAreaPolygon, AnalysisBar, AnalysisConnectivity, AnalysisBeamEnd, AnalysisCompositeBeam, AnalysisCrossSection, and 113 more types.
---

# Tekla.Structures.Analysis

Auto-generated from vendor docs for tekla 2026.0. 121 types in this namespace.

## Analysis (class)

The Analysis class is a class from which the user can query analysis instances.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/47cb0854-5cb1-9863-234f-acf7ca8ae087)

### Constructors
- `public Analysis()` — Initializes a new instance of the Analysis class.

### Methods
#### `public AnalysisObjectSelector GetAnalysisObjectSelector()`

Returns the analysis object selector.

**Returns:** `AnalysisObjectSelector` — The analysis object selector on success.

[Docs](https://developer.tekla.com/topic/en/18/47/d6520431-c9b4-c5f5-f4d5-b1d04b4e3bc8)

#### `public bool GetConnectionStatus()`

Checks the connection status of the Tekla Structures process.

**Returns:** `Boolean` — True if a proper connection to the Tekla Structures process has been established. Currently there's no way to re-establish the connection.

[Docs](https://developer.tekla.com/topic/en/18/47/acbd9e7c-d498-507f-1f7a-2a09269dfca8)

## AnalysisArea (class)

The AnalysisArea class contains information related to analysis areas.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/fe26f692-3c0c-caac-b3ad-08ae4f6ba405)

### Constructors
- `public AnalysisArea()` — Initializes a new instance of the AnalysisArea class.

### Methods
#### `public void Delete()`

Deletes an analysis area. Method is not implemented yet.

[Docs](https://developer.tekla.com/topic/en/18/47/afc5eb62-d1b4-41e2-f375-2a911ce2aa08)

#### `public AnalysisObjectEnumerator GetAreaHoles()`

Gets the area holes of the analysis area.

**Returns:** `AnalysisObjectEnumerator` — The area holes of the analysis area.

[Docs](https://developer.tekla.com/topic/en/18/47/64f5bd46-62ea-5754-ae19-db73cfc9e27c)

#### `public AnalysisObjectEnumerator GetInnerPositions()`

Gets the inner positions of the analysis area.

**Returns:** `AnalysisObjectEnumerator` — The inner positions of the analysis area.

[Docs](https://developer.tekla.com/topic/en/18/47/f0447753-dfec-1cab-f755-3015aacddfa6)

#### `public void Modify()`

Modifies an analysis area. Method is not implemented yet.

[Docs](https://developer.tekla.com/topic/en/18/47/bac7832b-0fb2-0f4a-8ad3-974f53791682)

#### `public override void Select()`

Selects an analysis area. The AnalysisModelName and ID have to be set.

[Docs](https://developer.tekla.com/topic/en/18/47/d8032a19-efb2-cd48-99d5-f0bb7895330b)

### Properties
- `AnalysisDesignCode` (AnalysisDesignCode, get/set) — Gets or sets the design code of the analysis area.
- `AnalysisModelName` (String, get/set) — Gets or sets the analysis model name.
- `AnalysisObjectType` (AnalysisObject.AnalysisObjectEnum, get/set) — Gets or sets the type of the analysis object.
- `AnalysisType` (AnalysisPart.AnalysisTypeEnum, get/set) — Gets or sets the analysis type of the analysis area.
- `Area` (AnalysisAreaPolygon, get/set) — Gets or sets the area polygon of the analysis area.
- `AreaHoles` (List<AnalysisAreaPolygon>, get/set) — Gets or sets the area holes of the analysis area.
- `FatherObject2ID` (Identifier, get/set) — Gets or sets the identifier of the father part.
- `FatherObject2Type` (AnalysisObject.AnalysisObjectEnum, get/set) — Gets or sets the type of the father object.
- `FatherObjectID` (Identifier, get/set) — Gets or sets the identifier of the father part.
- `FatherObjectType` (AnalysisObject.AnalysisObjectEnum, get/set) — Gets or sets the type of the father object.
- `ID` (Identifier, get/set) — Gets or sets the identifier of the analysis object.
- `InnerPositions` (List<AnalysisPosition>, get/set) — Gets or sets the inner positions of the analysis area.
- `Label` (AnalysisLabel, get/set) — Gets or sets the label of the analysis area.
- `Material` (AnalysisMaterial, get/set) — Gets or sets the material of the analysis area.
- `Profile` (AnalysisCrossSection, get/set) — Gets or sets the profile of the analysis area.
- `RigidDiaphragm` (AnalysisRigidDiaphragm, get/set) — Gets or sets the rigid diaphragm of the analysis area.
- `Thickness` (Double, get/set) — Gets or sets the thickness of the analysis area.

## AnalysisAreaPolygon (class)

The AnalysisAreaPolygon class contains information related to analysis area polygons.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/9a4c35fa-8955-e019-2812-a8737c70355e)

### Constructors
- `public AnalysisAreaPolygon(AnalysisObject.AnalysisObjectEnum ObjectType)` — Creates a new instance of the AnalysisAreaPolygon class.

### Methods
#### `public void Add(AnalysisPosition AnalysisPosition)`

Adds a position to the analysis area polygon. Also adds an edge.

**Parameters:**
- `AnalysisPosition` (Tekla.Structures.Analysis.AnalysisPosition) — The position to be added.

[Docs](https://developer.tekla.com/topic/en/18/47/dcd56480-7cc2-7ec3-1bcd-bd9cb1cd6baf)

#### `public AnalysisObjectEnumerator GetEdges()`

Gets the edges of the area polygon.

**Returns:** `AnalysisObjectEnumerator` — The edges of the area polygon.

[Docs](https://developer.tekla.com/topic/en/18/47/4a17d0fa-4e43-7502-a6dd-fd2cefede0fc)

#### `public AnalysisObjectEnumerator GetPositions()`

Gets the positions of the area polygon.

**Returns:** `AnalysisObjectEnumerator` — The positions of the area polygon.

[Docs](https://developer.tekla.com/topic/en/18/47/431e508a-512d-97ac-648e-5564b1adb941)

#### `public void Insert()`

Inserts an analysis area polygon. The AnalysisModelName has to be set.

[Docs](https://developer.tekla.com/topic/en/18/47/5af081d1-cafd-db09-5aab-aaea9159158a)

#### `public override void Select()`

Selects an analysis area polygon. The AnalysisObjectType has to be set.

[Docs](https://developer.tekla.com/topic/en/18/47/08537d0f-a7c1-bdb6-243b-9fa55b88f3b6)

### Properties
- `AnalysisModelName` (String, get/set) — Gets or sets the analysis model name.
- `AnalysisObjectType` (AnalysisObject.AnalysisObjectEnum, get/set) — Gets or sets the type of the analysis object.
- `Edges` (List<AnalysisEdge>, get/set) — The edges of the area polygon.
- `FatherObject2ID` (Identifier, get/set) — Gets or sets the identifier of the father part.
- `FatherObject2Type` (AnalysisObject.AnalysisObjectEnum, get/set) — Gets or sets the type of the father object.
- `FatherObjectID` (Identifier, get/set) — Gets or sets the identifier of the father part.
- `FatherObjectType` (AnalysisObject.AnalysisObjectEnum, get/set) — Gets or sets the type of the father object.
- `ID` (Identifier, get/set) — Gets or sets the identifier of the analysis object.
- `Positions` (List<AnalysisPosition>, get/set) — The positions of the area polygon.

## AnalysisBar (class)

The AnalysisBar class contains information related to analysis bars.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/3ff27579-557f-2457-d76c-ea70b2c3101e)

### Constructors
- `public AnalysisBar()` — Initializes a new instance of the AnalysisBar class.

### Methods
#### `public void Add(AnalysisPosition AnalysisPosition)`

Adds a position to the analysis bar.

**Parameters:**
- `AnalysisPosition` (Tekla.Structures.Analysis.AnalysisPosition) — The position to be added.

[Docs](https://developer.tekla.com/topic/en/18/47/94840bd0-1058-900f-d0d7-e1ce12084405)

#### `public void Add(AnalysisPosition position, AnalysisMember member)`

Adds a position and member to the analysis bar.

**Parameters:**
- `position` (Tekla.Structures.Analysis.AnalysisPosition) — The position to be added.
- `member` (Tekla.Structures.Analysis.AnalysisMember) — The member to be added.

[Docs](https://developer.tekla.com/topic/en/18/47/7b09f522-a98f-884b-899c-df298172e861)

#### `public void Delete()`

Deletes an analysis bar. The method is not implemented.

[Docs](https://developer.tekla.com/topic/en/18/47/bf807987-579b-264b-fe75-744322dcbcfc)

#### `public AnalysisObjectEnumerator GetMembers()`

Gets the members of the analysis bar.

**Returns:** `AnalysisObjectEnumerator` — The members of the analysis bar.

[Docs](https://developer.tekla.com/topic/en/18/47/6e0db972-47fa-4b46-4ecd-74d9ca57cc92)

#### `public AnalysisObjectEnumerator GetPositions()`

Gets the positions of the analysis bar.

**Returns:** `AnalysisObjectEnumerator` — The positions of the analysis bar.

[Docs](https://developer.tekla.com/topic/en/18/47/ca833661-fac3-9807-2b22-01cd84711b6f)

#### `public void Modify()`

Modifies an analysis bar. The method is not implemented.

[Docs](https://developer.tekla.com/topic/en/18/47/3d7699e4-55da-aac0-c137-f8670077b40b)

#### `public override void Select()`

Selects an analysis bar. The AnalysisModelName and ID have to be set.

[Docs](https://developer.tekla.com/topic/en/18/47/3a0291d9-eb64-9cba-5173-51025d0d1f36)

### Properties
- `AnalysisDesignCode` (AnalysisDesignCode, get/set) — Gets or sets the analysis design code of the analysis bar.
- `AnalysisModelName` (String, get/set) — Gets or sets the analysis model name.
- `AnalysisObjectType` (AnalysisObject.AnalysisObjectEnum, get/set) — Gets or sets the type of the analysis object.
- `AnalysisType` (AnalysisPart.AnalysisTypeEnum, get/set) — Gets or sets the analysis type of the analysis bar.
- `CardinalPoint` (AnalysisBar.CardinalPointEnum, get/set) — Gets or sets the cardinal point of the analysis bar.
- `Composite` (AnalysisCompositeBeam, get/set) — Gets or sets the composite beam of the analysis bar.
- `Curvature` (AnalysisCurvature, get/set) — Gets or sets the curvature of the analysis bar.
- `FatherObject2ID` (Identifier, get/set) — Gets or sets the identifier of the father part.
- `FatherObject2Type` (AnalysisObject.AnalysisObjectEnum, get/set) — Gets or sets the type of the father object.
- `FatherObjectID` (Identifier, get/set) — Gets or sets the identifier of the father part.
- `FatherObjectType` (AnalysisObject.AnalysisObjectEnum, get/set) — Gets or sets the type of the father object.
- `ID` (Identifier, get/set) — Gets or sets the identifier of the analysis object.
- `Label` (AnalysisLabel, get/set) — Gets or sets the label of the analysis bar.
- `Material` (AnalysisMaterial, get/set) — Gets or sets the material of the analysis bar.
- `Members` (List<AnalysisMember>, get/set) — Gets the members of the analysis bar.
- `Positions` (List<AnalysisPosition>, get/set) — Gets the positions of the analysis bar.
- `ProfileEnd` (AnalysisCrossSection, get/set) — Gets or sets the end profile of the analysis bar.
- `ProfileStart` (AnalysisCrossSection, get/set) — Gets or sets the start profile of the analysis bar.

## AnalysisBar.CardinalPointEnum (enum)

The cardinal point types of the analysis bar.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/acd4ff4b-1095-b5ce-e3b3-871095db92d4)

### Values
- `CARDINAL_POINT_CENTROID` = `0` — The cardinal point is centroid.
- `CARDINAL_POINT_TOP_LEFT` = `1` — The cardinal point is top left.
- `CARDINAL_POINT_TOP_CENTER` = `2` — The cardinal point is top center.
- `CARDINAL_POINT_TOP_RIGHT` = `3` — The cardinal point is top right.
- `CARDINAL_POINT_MIDDLE_LEFT` = `4` — The cardinal point is middle left.
- `CARDINAL_POINT_MIDDLE_CENTER` = `5` — The cardinal point is middle center.
- `CARDINAL_POINT_MIDDLE_RIGHT` = `6` — The cardinal point is middle right.
- `CARDINAL_POINT_BOTTOM_LEFT` = `7` — The cardinal point is bottom left.
- `CARDINAL_POINT_BOTTOM_CENTER` = `8` — The cardinal point is bottom center.
- `CARDINAL_POINT_BOTTOM_RIGHT` = `9` — The cardinal point is bottom right.

## AnalysisBeamEnd (class)

The AnalysisBeamEnd class contains information related to analysis beam ends.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/4cb46c46-20b2-25d2-ded2-1d065a7fd2b0)

### Constructors
- `public AnalysisBeamEnd()` — Initializes a new instance of the AnalysisBeamEnd class.

### Properties
- `Connectivity` (AnalysisConnectivity, get/set) — Gets or sets the connectivity at the beam end.
- `Eccentricity` (Vector, get/set) — Gets or sets the eccentricity at the beam end (in the member's local coordinate system).
- `Profile` (AnalysisCrossSection, get/set) — Gets or sets the profile at the beam end.

## AnalysisCompositeBeam (class)

The AnalysisCompositeBeam class contains information related to analysis composite beams.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/0841f981-87ab-b1ee-55e4-2ad372f7a37e)

### Constructors
- `public AnalysisCompositeBeam()` — Initializes a new instance of the AnalysisCompositeBeam class

### Properties
- `Composite` (Boolean, get/set) — Indicates whether the beam is composite.
- `ConcreteStrength` (Double, get/set) — The strength of the concrete of the analysis composite beam.
- `RibDirection` (Vector, get/set) — The direction of the rib of the analysis composite beam (in the global coordinate system).
- `RibHeight` (Double, get/set) — The height of the rib of the analysis composite beam.
- `RibWidth` (Double, get/set) — The width of the rib of the analysis composite beam.
- `SlabMaterial` (String, get/set) — The slab material of the analysis composite beam.
- `SlabThickness` (Double, get/set) — The thickness of the slab of the analysis composite beam.
- `SlabWidthLeft` (Double, get/set) — The left width of the slab of the analysis composite beam.
- `SlabWidthRight` (Double, get/set) — The right width of the slab of the analysis composite beam.
- `StudDiameter` (Double, get/set) — The diameter of the stud of the analysis composite beam.
- `StudLength` (Double, get/set) — The length of the stud of the analysis composite beam.

## AnalysisConnectivity (class)

The AnalysisConnectivity class contains information related to analysis connectivities.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/08abc530-b7e9-5a6b-1e33-1ef3014d58f6)

### Constructors
- `public AnalysisConnectivity()` — Initializes a new instance of the AnalysisConnectivity class

### Properties
- `Connectivity` (.AnalysisConnectivity.ConnectivityEnum., get/set) — The connectivity type of the analysis connectivity.
- `Spring` (.Double., get/set) — The spring values of the analysis connectivity.

## AnalysisConnectivity.CombinedTypeEnum (enum)

The combined types.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/6fcb495a-b35d-a303-9998-c8f9ade7379c)

### Values
- `COMBINED_TYPE_FIXED` = `0` — The combined type is fixed.
- `COMBINED_TYPE_PINNED` = `1` — The combined type is pinned.
- `COMBINED_TYPE_OTHER` = `2` — The combined type is other.

## AnalysisConnectivity.ConnectivityEnum (enum)

The connectivity types.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/47869ce9-2303-0618-6f64-16f5cff42e3e)

### Values
- `CONNECTIVITY_FIXED` = `0` — The connectivity is fixed.
- `CONNECTIVITY_FREE` = `1` — The connectivity is free.
- `CONNECTIVITY_SPRING` = `2` — The connectivity is a spring.
- `CONNECTIVITY_PARTIAL_RELEASE` = `3` — The connectivity is partially released.

## AnalysisCrossSection (class)

The AnalysisCrossSection class contains information related to analysis cross sections.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/5f39479f-2ea4-9799-a226-6ed9360a9da4)

### Constructors
- `public AnalysisCrossSection()` — Creates a new instance of an analysis cross section.

### Methods
#### `public void Delete()`

Deletes an analysis cross section. The AnalysisModelName and ID have to be set.

[Docs](https://developer.tekla.com/topic/en/18/47/2dda6dcb-28dc-50c6-bba0-e0681d0e086a)

#### `public List<AnalysisCrossSection> GetCrossSectionGroup()`

Gets the cross section group.

**Returns:** `List<AnalysisCrossSection>` — The cross section group.

[Docs](https://developer.tekla.com/topic/en/18/47/907ce96f-4947-c62d-fe9d-acad76f92dad)

#### `public void Insert()`

Inserts an analysis cross section. The AnalysisModelName has to be set.

[Docs](https://developer.tekla.com/topic/en/18/47/ec7fbfb2-4c6f-0125-d408-12fd89973166)

#### `public override void Select()`

Selects an analysis cross section. The AnalysisModelName and ID/Name have to be set.

[Docs](https://developer.tekla.com/topic/en/18/47/3fb513b9-840d-175c-a250-9e1f2f642a4a)

### Properties
- `A` (Double, get/set) — The cross section area (m2) of the analysis cross section.
- `AnalysisModelName` (String, get/set) — Gets or sets the analysis model name.
- `AnalysisObjectType` (AnalysisObject.AnalysisObjectEnum, get/set) — Gets or sets the type of the analysis object.
- `AnalysisSubSections` (List<AnalysisSubSection>, get/set) — The subsections of the analysis cross section.
- `Avy` (Double, get/set) — The shear area for loads parallel to the Y-axis (m2) of the analysis cross section.
- `Avz` (Double, get/set) — The shear area for loads parallel to the Z-axis (m2) of the analysis cross section.
- `Calculated` (Boolean, get/set) — Indicates whether section values have been calculated.
- `FatherObject2ID` (Identifier, get/set) — Gets or sets the identifier of the father part.
- `FatherObject2Type` (AnalysisObject.AnalysisObjectEnum, get/set) — Gets or sets the type of the father object.
- `FatherObjectID` (Identifier, get/set) — Gets or sets the identifier of the father part.
- `FatherObjectType` (AnalysisObject.AnalysisObjectEnum, get/set) — Gets or sets the type of the father object.
- `G` (Double, get/set) — The mass per unit of length (kg/m) of the analysis cross section.
- `ID` (Identifier, get/set) — Gets or sets the identifier of the analysis object.
- `It` (Double, get/set) — The torsion moment (m4) of the analysis cross section.
- `Iw` (Double, get/set) — The warping constant (m6) of the analysis cross section.
- `Iy` (Double, get/set) — The radius of gyration about the Y-axis (m) of the analysis cross section.
- `Iyy` (Double, get/set) — The moment of inertia about the Y-axis (m4) of the analysis cross section.
- `Iz` (Double, get/set) — The radius of gyration about the Z-axis (m) of the analysis cross section.
- `Izz` (Double, get/set) — The moment of inertia about the Z-axis (m4) of the analysis cross section.
- `Name` (String, get/set) — The name of the analysis cross section.
- `Wplyy` (Double, get/set) — The plastic modulus about the Y-axis (m3) of the analysis cross section.
- `Wplzz` (Double, get/set) — The plastic modulus about the Z-axis (m3) of the analysis cross section.
- `Wyy` (Double, get/set) — The elastic modulus about the Y-axis (m3) of the analysis cross section.
- `Wzz` (Double, get/set) — The elastic modulus about the Z-axis (m3) of the analysis cross section.

## AnalysisCurvature (class)

The AnalysisCurvature class contains information related to analysis curvatures.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/b108b86e-c2e8-1ffd-c04b-c142bdc96cd2)

### Constructors
- `public AnalysisCurvature()` — Initializes a new instance of the AnalysisCurvature class

### Properties
- `Radius` (Double, get/set) — The radius of the beam curvature.
- `RadiusDirection` (Vector, get/set) — The radius direction of the beam curvature.

## AnalysisDecomposedAreaLoad (class)

The AnalysisDecomposedAreaLoad class contains decomposed area load information.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/9b6f3d15-7357-85f8-7ef6-183af0a7631f)

### Constructors
- `public AnalysisDecomposedAreaLoad()` — Initializes a new instance of the AnalysisDecomposedAreaLoad class

### Methods
#### `public override void Select()`

Selects an analysis decomposed area load.

[Docs](https://developer.tekla.com/topic/en/18/47/1c9fd463-f53d-dfe4-3f32-60b68fc9de8b)

### Properties
- `AnalysisLoadType` (AnalysisDecomposedLoad.AnalysisLoadTypeEnum, get/set) — Gets or sets the type of the analysis decomposed load.
- `AnalysisModelName` (String, get/set) — Gets or sets the analysis model name.
- `AnalysisObjectType` (AnalysisObject.AnalysisObjectEnum, get/set) — Gets or sets the type of the analysis object.
- `Direction` (AnalysisDecomposedLoad.AnalysisDecomposedLoadDirectionEnum, get) — The direction of the decomposed area load.
- `FatherObject2ID` (Identifier, get/set) — Gets or sets the identifier of the father part.
- `FatherObject2Type` (AnalysisObject.AnalysisObjectEnum, get/set) — Gets or sets the type of the father object.
- `FatherObjectID` (Identifier, get/set) — Gets or sets the identifier of the father part.
- `FatherObjectType` (AnalysisObject.AnalysisObjectEnum, get/set) — Gets or sets the type of the father object.
- `ID` (Identifier, get/set) — Gets or sets the identifier of the analysis object.
- `Magnitudes` (List<Double>, get) — The magnitudes of the decomposed area load.
- `ModelLoadGroupID` (Identifier, get/set) — Gets or sets the identifier of the load group.
- `ModelLoadID` (Identifier, get/set) — Gets or sets the identifier of the model load.
- `ObjectAttachedToID` (Identifier, get/set) — Gets or sets the identifier of the object the load is attached to.
- `ObjectAttachedToType` (AnalysisObject.AnalysisObjectEnum, get/set) — Gets or sets the type of the object the load is attached to.
- `Positions` (List<Point>, get) — The positions of the decomposed area load.

## AnalysisDecomposedBarLoad (class)

The AnalysisDecomposedBarLoad class contains information related to decomposed bar loads.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/d62778b8-087d-2b97-4faa-b858fbb50b6e)

### Constructors
- `public AnalysisDecomposedBarLoad()` — Initializes a new instance of the AnalysisDecomposedBarLoad class

### Methods
#### `public override void Select()`

Selects an analysis decomposed bar load.

[Docs](https://developer.tekla.com/topic/en/18/47/8394bba1-db92-0c3a-9b53-032465b48ed5)

### Properties
- `AnalysisLoadType` (AnalysisDecomposedLoad.AnalysisLoadTypeEnum, get/set) — Gets or sets the type of the analysis decomposed load.
- `AnalysisModelName` (String, get/set) — Gets or sets the analysis model name.
- `AnalysisObjectType` (AnalysisObject.AnalysisObjectEnum, get/set) — Gets or sets the type of the analysis object.
- `CSCode` (AnalysisDecomposedLoad.LoadCoordinateEnum, get) — The CS code of the decomposed bar load.
- `EndDistance` (Double, get) — The end distance of the decomposed bar load.
- `EndValue` (Double, get) — The end value of the decomposed bar load.
- `FatherObject2ID` (Identifier, get/set) — Gets or sets the identifier of the father part.
- `FatherObject2Type` (AnalysisObject.AnalysisObjectEnum, get/set) — Gets or sets the type of the father object.
- `FatherObjectID` (Identifier, get/set) — Gets or sets the identifier of the father part.
- `FatherObjectType` (AnalysisObject.AnalysisObjectEnum, get/set) — Gets or sets the type of the father object.
- `ID` (Identifier, get/set) — Gets or sets the identifier of the analysis object.
- `ModelLoadGroupID` (Identifier, get/set) — Gets or sets the identifier of the load group.
- `ModelLoadID` (Identifier, get/set) — Gets or sets the identifier of the model load.
- `ObjectAttachedToID` (Identifier, get/set) — Gets or sets the identifier of the object the load is attached to.
- `ObjectAttachedToType` (AnalysisObject.AnalysisObjectEnum, get/set) — Gets or sets the type of the object the load is attached to.
- `StartDistance` (Double, get) — The start distance of the decomposed bar load.
- `StartValue` (Double, get) — The start value of the decomposed bar load.

## AnalysisDecomposedLoad (class)

The AnalysisDecomposedLoad class contains information related to decomposed loads.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/22170c59-65ac-2f10-37d7-71a9a4c7305b)

### Constructors
- `public AnalysisDecomposedLoad()` — Initializes a new instance of the AnalysisDecomposedLoad class.

### Methods
#### `public abstract void Select()`

Selects an analysis object from the database.

[Docs](https://developer.tekla.com/topic/en/18/47/838ad5fe-ed14-8f58-d432-efd635e6c7ea)

### Properties
- `AnalysisLoadType` (AnalysisDecomposedLoad.AnalysisLoadTypeEnum, get/set) — Gets or sets the type of the analysis decomposed load.
- `AnalysisModelName` (String, get/set) — Gets or sets the analysis model name.
- `AnalysisObjectType` (AnalysisObject.AnalysisObjectEnum, get/set) — Gets or sets the type of the analysis object.
- `FatherObject2ID` (Identifier, get/set) — Gets or sets the identifier of the father part.
- `FatherObject2Type` (AnalysisObject.AnalysisObjectEnum, get/set) — Gets or sets the type of the father object.
- `FatherObjectID` (Identifier, get/set) — Gets or sets the identifier of the father part.
- `FatherObjectType` (AnalysisObject.AnalysisObjectEnum, get/set) — Gets or sets the type of the father object.
- `ID` (Identifier, get/set) — Gets or sets the identifier of the analysis object.
- `ModelLoadGroupID` (Identifier, get/set) — Gets or sets the identifier of the load group.
- `ModelLoadID` (Identifier, get/set) — Gets or sets the identifier of the model load.
- `ObjectAttachedToID` (Identifier, get/set) — Gets or sets the identifier of the object the load is attached to.
- `ObjectAttachedToType` (AnalysisObject.AnalysisObjectEnum, get/set) — Gets or sets the type of the object the load is attached to.

## AnalysisDecomposedLoad.AnalysisDecomposedLoadDirectionEnum (enum)

Defines the analysis decomposed load directions.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/ce99bc57-d5a5-eaa3-b501-aef9f163635e)

### Values
- `X` = `1` — The analysis decomposed load direction is in the direction of the X-axis.
- `Y` = `2` — The analysis decomposed load direction is in the direction of the Y-axis.
- `Z` = `3` — The analysis decomposed load direction is in the direction of the Z-axis.
- `RX` = `4` — The analysis decomposed load direction is in the rotation of the X-axis.
- `RY` = `5` — The analysis decomposed load direction is in the rotation of the Y-axis.
- `RZ` = `6` — The analysis decomposed load direction is in the rotation of the Z-axis.

## AnalysisDecomposedLoad.AnalysisLoadTypeEnum (enum)

Defines the analysis load types.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/568646ff-3247-512a-4629-433b141d7459)

### Values
- `NOT_DEFINED` = `0` — The analysis load type is not defined.
- `ANALYSIS_NODAL_LOAD` = `1001` — The analysis nodal load.
- `ANALYSIS_MEMBER_LINE_LOADX` = `1` — The analysis member line load in the X direction.
- `ANALYSIS_MEMBER_LINE_LOADY` = `2` — The analysis member line load in the Y direction.
- `ANALYSIS_MEMBER_LINE_LOADZ` = `3` — The analysis member line load in the Z direction.
- `ANALYSIS_MEMBER_LINE_MOMENTX` = `4` — The analysis member line moment load in the X direction.
- `ANALYSIS_MEMBER_LINE_MOMENTY` = `5` — The analysis member line moment load in the Y direction.
- `ANALYSIS_MEMBER_LINE_MOMENTZ` = `6` — The analysis member line moment load in the Z direction.
- `ANALYSIS_MEMBER_POINT_LOADX` = `7` — The analysis member point load in the X direction.
- `ANALYSIS_MEMBER_POINT_LOADY` = `8` — The analysis member point load in the Y direction.
- `ANALYSIS_MEMBER_POINT_LOADZ` = `9` — The analysis member point load in the Z direction.
- `ANALYSIS_MEMBER_POINT_MOMENTX` = `10` — The analysis member point moment load in the X direction.
- `ANALYSIS_MEMBER_POINT_MOMENTY` = `11` — The analysis member point moment load in the Y direction.
- `ANALYSIS_MEMBER_POINT_MOMENTZ` = `12` — The analysis member point moment load in the Z direction.
- `ANALYSIS_MEMBER_TEMPERATURE_LOAD` = `13` — The analysis member temperature load.
- `ANALYSIS_PLATE_POINT_LOAD` = `14` — The analysis area point load.
- `ANALYSIS_PLATE_LINE_LOAD` = `15` — The analysis area line load.
- `ANALYSIS_PLATE_PRESSURE_LOAD` = `16` — The analysis area pressure load.
- `ANALYSIS_BAR_LINE_LOADX` = `21` — The analysis bar line load in the X direction.
- `ANALYSIS_BAR_LINE_LOADY` = `22` — The analysis bar line load in the Y direction.
- `ANALYSIS_BAR_LINE_LOADZ` = `23` — The analysis bar line load in the Z direction.
- `ANALYSIS_BAR_LINE_MOMENTX` = `24` — The analysis bar line moment load in the X direction.
- `ANALYSIS_BAR_LINE_MOMENTY` = `25` — The analysis bar line moment load in the Y direction.
- `ANALYSIS_BAR_LINE_MOMENTZ` = `26` — The analysis bar line moment load in the Z direction.
- `ANALYSIS_BAR_POINT_LOADX` = `27` — The analysis bar point load in the X direction.
- `ANALYSIS_BAR_POINT_LOADY` = `28` — The analysis bar point load in the Y direction.
- `ANALYSIS_BAR_POINT_LOADZ` = `29` — The analysis bar point load in the Z direction.
- `ANALYSIS_BAR_POINT_MOMENTX` = `30` — The analysis bar point moment load in the X direction.
- `ANALYSIS_BAR_POINT_MOMENTY` = `31` — The analysis bar point moment load in the Y direction.
- `ANALYSIS_BAR_POINT_MOMENTZ` = `32` — The analysis bar point moment load in the Z direction.
- `ANALYSIS_BAR_TEMPERATURE_LOAD` = `33` — The analysis bar temperature load.

## AnalysisDecomposedLoad.LoadCoordinateEnum (enum)

This defines whether local or global coordinates are used for loading the coordinate system.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/3238e019-d3c8-abff-6066-9fc11a80623d)

### Values
- `USE_LOCAL_COORDINATES` = `0` — Local coordinates are used.
- `USE_GLOBAL_COORDINATES` = `1` — Global coordinates are used.

## AnalysisDecomposedMemberLoad (class)

The AnalysisDecomposedMemberLoad class contains information related to decomposed member loads.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/fb83ca48-74bb-aad8-d98f-5c5e5682ec9d)

### Constructors
- `public AnalysisDecomposedMemberLoad()` — Initializes a new instance of the AnalysisDecomposedMemberLoad class.

### Methods
#### `public override void Select()`

Selects the analysis decomposed member load.

[Docs](https://developer.tekla.com/topic/en/18/47/9018b6b7-89b2-b1cc-8a51-d9dafcb7c0d6)

### Properties
- `AnalysisLoadType` (AnalysisDecomposedLoad.AnalysisLoadTypeEnum, get/set) — Gets or sets the type of the analysis decomposed load.
- `AnalysisModelName` (String, get/set) — Gets or sets the analysis model name.
- `AnalysisObjectType` (AnalysisObject.AnalysisObjectEnum, get/set) — Gets or sets the type of the analysis object.
- `CSCode` (AnalysisDecomposedLoad.LoadCoordinateEnum, get/set) — Gets or sets the CS code of the decomposed member load.
- `EndDistance` (Double, get/set) — Gets or sets the end distance of the decomposed member load.
- `EndValue` (Double, get/set) — Gets or sets the end value of the decomposed member load.
- `FatherObject2ID` (Identifier, get/set) — Gets or sets the identifier of the father part.
- `FatherObject2Type` (AnalysisObject.AnalysisObjectEnum, get/set) — Gets or sets the type of the father object.
- `FatherObjectID` (Identifier, get/set) — Gets or sets the identifier of the father part.
- `FatherObjectType` (AnalysisObject.AnalysisObjectEnum, get/set) — Gets or sets the type of the father object.
- `ID` (Identifier, get/set) — Gets or sets the identifier of the analysis object.
- `ModelLoadGroupID` (Identifier, get/set) — Gets or sets the identifier of the load group.
- `ModelLoadID` (Identifier, get/set) — Gets or sets the identifier of the model load.
- `ObjectAttachedToID` (Identifier, get/set) — Gets or sets the identifier of the object the load is attached to.
- `ObjectAttachedToType` (AnalysisObject.AnalysisObjectEnum, get/set) — Gets or sets the type of the object the load is attached to.
- `StartDistance` (Double, get/set) — Gets or sets the start distance of the decomposed member load.
- `StartValue` (Double, get/set) — Gets or sets the start value of the decomposed member load.

## AnalysisDecomposedNodeLoad (class)

The AnalysisDecomposedNodeLoad class contains information related to decomposed node loads.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/011ed77b-258f-8ac0-d366-64b030d82715)

### Constructors
- `public AnalysisDecomposedNodeLoad()` — Initializes a new instance of the AnalysisDecomposedNodeLoad class.

### Methods
#### `public override void Select()`

Selects an analysis decomposed node load.

[Docs](https://developer.tekla.com/topic/en/18/47/1163b258-2c7a-0a1b-51dd-3a610ac0f40d)

### Properties
- `AnalysisLoadType` (AnalysisDecomposedLoad.AnalysisLoadTypeEnum, get/set) — Gets or sets the type of the analysis decomposed load.
- `AnalysisModelName` (String, get/set) — Gets or sets the analysis model name.
- `AnalysisObjectType` (AnalysisObject.AnalysisObjectEnum, get/set) — Gets or sets the type of the analysis object.
- `F` (Vector, get/set) — Gets or sets the force of the analysis decomposed node load.
- `FatherObject2ID` (Identifier, get/set) — Gets or sets the identifier of the father part.
- `FatherObject2Type` (AnalysisObject.AnalysisObjectEnum, get/set) — Gets or sets the type of the father object.
- `FatherObjectID` (Identifier, get/set) — Gets or sets the identifier of the father part.
- `FatherObjectType` (AnalysisObject.AnalysisObjectEnum, get/set) — Gets or sets the type of the father object.
- `ID` (Identifier, get/set) — Gets or sets the identifier of the analysis object.
- `M` (Vector, get/set) — Gets or sets the moment of the analysis decomposed node load.
- `ModelLoadGroupID` (Identifier, get/set) — Gets or sets the identifier of the load group.
- `ModelLoadID` (Identifier, get/set) — Gets or sets the identifier of the model load.
- `ObjectAttachedToID` (Identifier, get/set) — Gets or sets the identifier of the object the load is attached to.
- `ObjectAttachedToType` (AnalysisObject.AnalysisObjectEnum, get/set) — Gets or sets the type of the object the load is attached to.

## AnalysisDecomposedSelfweightLoad (class)

The AnalysisDecomposedSelfweightLoad class contains information of decomposed selfweight loads.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/d957d7ca-bf9f-ae3c-2a91-d4e9543f8337)

### Constructors
- `public AnalysisDecomposedSelfweightLoad()` — Initializes a new instance of the AnalysisDecomposedSelfweightLoad class

### Methods
#### `public override void Select()`

Selects an analysis decomposed selfweight load.

[Docs](https://developer.tekla.com/topic/en/18/47/0979f81f-5085-6ac2-4fa9-71b328f7e938)

### Properties
- `AnalysisLoadType` (AnalysisDecomposedLoad.AnalysisLoadTypeEnum, get/set) — Gets or sets the type of the analysis decomposed load.
- `AnalysisModelName` (String, get/set) — Gets or sets the analysis model name.
- `AnalysisObjectType` (AnalysisObject.AnalysisObjectEnum, get/set) — Gets or sets the type of the analysis object.
- `Directions` (List<AnalysisDecomposedLoad.AnalysisDecomposedLoadDirectionEnum>, get) — The directions of the decomposed selfweight load.
- `Factors` (List<Double>, get) — The factors of the decomposed selfweight load.
- `FatherObject2ID` (Identifier, get/set) — Gets or sets the identifier of the father part.
- `FatherObject2Type` (AnalysisObject.AnalysisObjectEnum, get/set) — Gets or sets the type of the father object.
- `FatherObjectID` (Identifier, get/set) — Gets or sets the identifier of the father part.
- `FatherObjectType` (AnalysisObject.AnalysisObjectEnum, get/set) — Gets or sets the type of the father object.
- `ID` (Identifier, get/set) — Gets or sets the identifier of the analysis object.
- `ModelLoadGroupID` (Identifier, get/set) — Gets or sets the identifier of the load group.
- `ModelLoadID` (Identifier, get/set) — Gets or sets the identifier of the model load.
- `ObjectAttachedToID` (Identifier, get/set) — Gets or sets the identifier of the object the load is attached to.
- `ObjectAttachedToType` (AnalysisObject.AnalysisObjectEnum, get/set) — Gets or sets the type of the object the load is attached to.

## AnalysisDecomposedTemperatureLoad (class)

The AnalysisDecomposedTemperatureLoad class contains information related to decomposed temperature loads.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/60970561-01bc-66b7-8aa5-691afb9cd4db)

### Constructors
- `public AnalysisDecomposedTemperatureLoad()` — Initializes a new instance of the AnalysisDecomposedTemperatureLoad class

### Methods
#### `public override void Select()`

Selects an analysis decomposed temperature load.

[Docs](https://developer.tekla.com/topic/en/18/47/589d0e8d-a12f-70e0-0d38-6eaa5b6e4367)

### Properties
- `AnalysisLoadType` (AnalysisDecomposedLoad.AnalysisLoadTypeEnum, get/set) — Gets or sets the type of the analysis decomposed load.
- `AnalysisModelName` (String, get/set) — Gets or sets the analysis model name.
- `AnalysisObjectType` (AnalysisObject.AnalysisObjectEnum, get/set) — Gets or sets the type of the analysis object.
- `AxialTemperatureDifference` (Double, get) — The axial temperature difference of the decomposed temperature load.
- `FatherObject2ID` (Identifier, get/set) — Gets or sets the identifier of the father part.
- `FatherObject2Type` (AnalysisObject.AnalysisObjectEnum, get/set) — Gets or sets the type of the father object.
- `FatherObjectID` (Identifier, get/set) — Gets or sets the identifier of the father part.
- `FatherObjectType` (AnalysisObject.AnalysisObjectEnum, get/set) — Gets or sets the type of the father object.
- `HorizontalTemperatureDifference` (Double, get) — The horizontal temperature difference of the decomposed temperature load.
- `ID` (Identifier, get/set) — Gets or sets the identifier of the analysis object.
- `ModelLoadGroupID` (Identifier, get/set) — Gets or sets the identifier of the load group.
- `ModelLoadID` (Identifier, get/set) — Gets or sets the identifier of the model load.
- `ObjectAttachedToID` (Identifier, get/set) — Gets or sets the identifier of the object the load is attached to.
- `ObjectAttachedToType` (AnalysisObject.AnalysisObjectEnum, get/set) — Gets or sets the type of the object the load is attached to.
- `Strain` (Double, get) — The strain of the decomposed temperature load.
- `VerticalTemperatureDifference` (Double, get) — The vertical temperature difference of the decomposed temperature load.

## AnalysisDeltaZ (class)

The AnalysisDeltaZ class contains delta Z information for an analysis model.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/d8870681-dfba-d8e5-2e0f-f970fb7b0c35)

### Constructors
- `public AnalysisDeltaZ()` — Creates a new instance of an analysis delta Z.

### Properties
- `Bottom` (Double, get) — The delta Z at the bottom.
- `Top` (Double, get) — The delta Z at the top.

## AnalysisDesignCode (class)

The AnalysisDesignCode class contains information related to analysis design codes.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/eb046690-d5ab-6cfe-0853-e7e2cbeb80d7)

### Constructors
- `public AnalysisDesignCode()` — Initializes a new instance of the AnalysisDesignCode class

### Properties
- `DesignParameters` (AnalysisObjectEnumerator, get/set) — Gets or sets the design parameters of the analysis design code.
- `ID` (Identifier, get/set) — Gets or sets the identifier of the analysis design code.
- `MaterialType` (AnalysisMaterial.MaterialTypeEnum, get) — Gets the material type of the analysis analysis design code.
- `Name` (String, get) — Gets the name of the analysis design code.

## AnalysisDesignParameter (class)

The AnalysisDesignParameter class contains information related to analysis design parameters.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/038a7c51-a979-0b8a-8f4e-1b601f05e0f9)

### Constructors
- `public AnalysisDesignParameter(int materialType)` — Initializes a new instance of the AnalysisDesignParameter class.
- `public AnalysisDesignParameter(string analysisModelName, int parameterId, Identifier partId, double value)` — Initializes a new instance of the AnalysisDesignParameter class.

### Methods
#### `public override void Modify()`

Modifies an analysis design parameter. The ID has to be set.

[Docs](https://developer.tekla.com/topic/en/18/47/39293609-a0f3-aa6b-1f08-fd1f8d0122fd)

#### `public override void Select()`

Selects an analysis design parameter.

[Docs](https://developer.tekla.com/topic/en/18/47/fe470643-e424-a5c1-ab58-43510bd03610)

### Properties
- `AnalysisModelName` (String, get/set) — Gets or sets the analysis model name.
- `AnalysisObjectType` (AnalysisObject.AnalysisObjectEnum, get/set) — Gets or sets the type of the analysis object.
- `FatherObject2ID` (Identifier, get/set) — Gets or sets the identifier of the father part.
- `FatherObject2Type` (AnalysisObject.AnalysisObjectEnum, get/set) — Gets or sets the type of the father object.
- `FatherObjectID` (Identifier, get/set) — Gets or sets the identifier of the father part.
- `FatherObjectType` (AnalysisObject.AnalysisObjectEnum, get/set) — Gets or sets the type of the father object.
- `ID` (Identifier, get/set) — Gets or sets the identifier of the analysis object.
- `IsGeneral` (Boolean, get/set) — Gets or sets a value indicating whether the design parameter is general.
- `Name` (String, get/set) — Gets or sets the name of the analysis design parameter.
- `Unit` (Int32, get/set) — Gets or sets the unit of the analysis design parameter.
- `Value` (Double, get/set) — Gets or sets the value of the analysis design parameter.

## AnalysisDesignParameterBase (class)

The AnalysisDesignParameterBase class contains base information related to analysis design parameters.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/eedee4c6-7724-7b18-f12c-64f2462a82b8)

### Methods
#### `public abstract void Modify()`

Modifies an analysis design parameter at the database. Inserts if parameter does not exist.

[Docs](https://developer.tekla.com/topic/en/18/47/ba96bb8e-a7b4-4531-8f46-acb145de22d0)

#### `public abstract void Select()`

Selects an analysis object from the database.

[Docs](https://developer.tekla.com/topic/en/18/47/838ad5fe-ed14-8f58-d432-efd635e6c7ea)

### Properties
- `AnalysisModelName` (String, get/set) — Gets or sets the analysis model name.
- `AnalysisObjectType` (AnalysisObject.AnalysisObjectEnum, get/set) — Gets or sets the type of the analysis object.
- `FatherObject2ID` (Identifier, get/set) — Gets or sets the identifier of the father part.
- `FatherObject2Type` (AnalysisObject.AnalysisObjectEnum, get/set) — Gets or sets the type of the father object.
- `FatherObjectID` (Identifier, get/set) — Gets or sets the identifier of the father part.
- `FatherObjectType` (AnalysisObject.AnalysisObjectEnum, get/set) — Gets or sets the type of the father object.
- `ID` (Identifier, get/set) — Gets or sets the identifier of the analysis object.
- `IsGeneral` (Boolean, get/set) — Gets or sets a value indicating whether the design parameter is general.
- `Name` (String, get/set) — Gets or sets the name of the analysis design parameter.
- `Unit` (Int32, get/set) — Gets or sets the unit of the analysis design parameter.

## AnalysisDesignParameterMulti (class)

The AnalysisDesignParameterMulti class contains information related to analysis design parameters with multi values.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/e5c963ca-08c8-2ebf-1042-fc8738da262e)

### Constructors
- `public AnalysisDesignParameterMulti(int materialType)` — Initializes a new instance of the AnalysisDesignParameterMulti class.
- `public AnalysisDesignParameterMulti(string analysisModelName, int parameterId, Identifier partId)` — Initializes a new instance of the AnalysisDesignParameterMulti class.

### Methods
#### `public double GetValue(int index)`

Gets the value specified by the index of the analysis design parameter.

**Parameters:**
- `index` (System.Int32) — The index of the value.

**Returns:** `Double` — The value at the index.

[Docs](https://developer.tekla.com/topic/en/18/47/f6895b51-f553-4e17-24d9-95812e97ab6a)

#### `public override void Modify()`

Modifies an analysis design parameter. The ID has to be set.

[Docs](https://developer.tekla.com/topic/en/18/47/4ebae5d9-5c9e-9955-fcfb-fcce8f7cfefe)

#### `public override void Select()`

Selects an analysis design parameter.

[Docs](https://developer.tekla.com/topic/en/18/47/7de57939-b5ee-bd2b-a373-26c1832e4753)

#### `public void SetValue(int index, double value)`

Sets the value specified by the index of the analysis design parameter.

**Parameters:**
- `index` (System.Int32) — The index of the value.
- `value` (System.Double) — The value.

[Docs](https://developer.tekla.com/topic/en/18/47/2b53c57a-62a1-2098-6d42-915962610188)

### Properties
- `AnalysisModelName` (String, get/set) — Gets or sets the analysis model name.
- `AnalysisObjectType` (AnalysisObject.AnalysisObjectEnum, get/set) — Gets or sets the type of the analysis object.
- `FatherObject2ID` (Identifier, get/set) — Gets or sets the identifier of the father part.
- `FatherObject2Type` (AnalysisObject.AnalysisObjectEnum, get/set) — Gets or sets the type of the father object.
- `FatherObjectID` (Identifier, get/set) — Gets or sets the identifier of the father part.
- `FatherObjectType` (AnalysisObject.AnalysisObjectEnum, get/set) — Gets or sets the type of the father object.
- `ID` (Identifier, get/set) — Gets or sets the identifier of the analysis object.
- `IsGeneral` (Boolean, get/set) — Gets or sets a value indicating whether the design parameter is general.
- `MultiValueCount` (Int32, get) — Gets the multi value count of the analysis design parameter.
- `Name` (String, get/set) — Gets or sets the name of the analysis design parameter.
- `Unit` (Int32, get/set) — Gets or sets the unit of the analysis design parameter.

## AnalysisDesignParameterMulti.DesignParIndex (enum)

Multi value design parameter indexes

[Vendor docs](https://developer.tekla.com/topic/en/18/47/d19ca947-d9d1-c906-e5ee-6c75bac378ff)

### Values
- `Ky` = `88` — VAM_DESIGNPARINDEX_MultiKy => 88.
- `Kz` = `89` — VAM_DESIGNPARINDEX_MultiKz => 89.
- `Ly` = `90` — VAM_DESIGNPARINDEX_MultiLy => 90.
- `Lz` = `91` — VAM_DESIGNPARINDEX_MultiLz => 91.

## AnalysisDesignSettings (class)

The AnalysisDesignSettings class contains information about analysis design settings.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/bc513219-786a-654a-ca22-1c340ddca845)

### Constructors
- `public AnalysisDesignSettings()` — Creates a new analysis design settings instance.

### Properties
- `ConcreteDesignCode` (String, get) — The concrete design code.
- `ConcreteDesignMethod` (String, get) — The concrete design method.
- `SteelDesignCode` (String, get) — The steel design code.
- `SteelDesignMethod` (String, get) — The steel design method.
- `TimberDesignCode` (String, get) — The timber design code.
- `TimberDesignMethod` (String, get) — The timber design method.

## AnalysisEccentricity (class)

The AnalysisEccentricity class contains eccentricity information for an analysis model.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/57b5d953-4d56-f84f-a67b-ac58260e690e)

### Constructors
- `public AnalysisEccentricity()` — Creates a new analysis eccentricity instance.

### Properties
- `End` (Vector, get) — The eccentricity at the end.
- `Start` (Vector, get) — The eccentricity at the start.

## AnalysisEdge (class)

The AnalysisEdge class contains information related to analysis edges.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/d8425917-6ede-07de-d448-2967ae5abea4)

### Constructors
- `public AnalysisEdge(AnalysisObject.AnalysisObjectEnum ObjectType)` — Creates a new instance of the AnalysisEdge class.

### Methods
#### `public void AddSplit(AnalysisPosition AnalysisPosition)`

Adds a split position to the analysis edge.

**Parameters:**
- `AnalysisPosition` (Tekla.Structures.Analysis.AnalysisPosition) — The split position to be added.

[Docs](https://developer.tekla.com/topic/en/18/47/97fb70fe-2bf3-1123-3eaf-933faa33cd11)

#### `public void CopyData(AnalysisEdge Analysisedge)`

Copies data from another analysis edge.

**Parameters:**
- `Analysisedge` (Tekla.Structures.Analysis.AnalysisEdge) — The analysis edge to copy data from.

[Docs](https://developer.tekla.com/topic/en/18/47/bd114c63-7989-e9dc-7120-19315857a5a3)

#### `public AnalysisObjectEnumerator GetSplitPositions()`

Gets the split positions of the analysis edge.

**Returns:** `AnalysisObjectEnumerator` — The split positions of the analysis edge.

[Docs](https://developer.tekla.com/topic/en/18/47/0dfbec6e-354f-1b34-b69e-451f583d2be9)

#### `public void Modify()`

Modifies an analysis edge. The AnalysisModelName has to be set.

[Docs](https://developer.tekla.com/topic/en/18/47/6c457e77-05cc-2d55-e9bc-13da787aa80f)

#### `public override void Select()`

Selects an analysis edge.

[Docs](https://developer.tekla.com/topic/en/18/47/2b38b971-27ff-c814-1587-f036332eb619)

### Properties
- `AnalysisModelName` (String, get/set) — Gets or sets the analysis model name.
- `AnalysisObjectType` (AnalysisObject.AnalysisObjectEnum, get/set) — Gets or sets the type of the analysis object.
- `Connectivity` (AnalysisConnectivity, get/set) — The connectivity of the analysis edge.
- `FatherObject2ID` (Identifier, get/set) — Gets or sets the identifier of the father part.
- `FatherObject2Type` (AnalysisObject.AnalysisObjectEnum, get/set) — Gets or sets the type of the father object.
- `FatherObjectID` (Identifier, get/set) — Gets or sets the identifier of the father part.
- `FatherObjectType` (AnalysisObject.AnalysisObjectEnum, get/set) — Gets or sets the type of the father object.
- `ID` (Identifier, get/set) — Gets or sets the identifier of the analysis object.
- `Radius` (Double, get/set) — The radius of the analysis edge.
- `SplitPositions` (List<AnalysisPosition>, get) — The split positions of the analysis edge.

## AnalysisFace (class)

The AnalysisFace class contains information related to analysis faces.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/ff6b2584-c4e1-7eea-04b2-7995b08c06a5)

### Constructors
- `public AnalysisFace()` — Initializes a new instance of the AnalysisFace class

### Methods
#### `public override void Select()`

Selects an analysis face.

[Docs](https://developer.tekla.com/topic/en/18/47/f9ddd3e3-8588-869a-d472-67c50612874b)

### Properties
- `AnalysisModelName` (String, get/set) — Gets or sets the analysis model name.
- `AnalysisObjectType` (AnalysisObject.AnalysisObjectEnum, get/set) — Gets or sets the type of the analysis object.
- `FatherObject2ID` (Identifier, get/set) — Gets or sets the identifier of the father part.
- `FatherObject2Type` (AnalysisObject.AnalysisObjectEnum, get/set) — Gets or sets the type of the father object.
- `FatherObjectID` (Identifier, get/set) — Gets or sets the identifier of the father part.
- `FatherObjectType` (AnalysisObject.AnalysisObjectEnum, get/set) — Gets or sets the type of the father object.
- `ID` (Identifier, get/set) — Gets or sets the identifier of the analysis object.
- `Positions` (AnalysisObjectEnumerator, get/set) — The positions of the analysis face.

## AnalysisLabel (class)

The AnalysisLabel class contains information related to analysis labels.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/b21f9ac8-644c-e9c7-9f10-eef6a5a7e270)

### Constructors
- `public AnalysisLabel()` — Initializes a new instance of the AnalysisLabel class

### Properties
- `Number` (Int32, get/set) — The number of the analysis label.
- `Prefix` (String, get/set) — The prefix of the analysis label.

## AnalysisLoadCombination (class)

The AnalysisLoadCombination class contains information related to analysis load combinations.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/3398e50a-555b-78be-e25a-f8c7c92aa83d)

### Constructors
- `public AnalysisLoadCombination()` — Initializes a new instance of the AnalysisLoadCombination class.

### Methods
#### `public void Delete()`

Deletes an analysis load combination. The AnalysisModelName and ID have to be set.

[Docs](https://developer.tekla.com/topic/en/18/47/943ade7c-b4ff-bc04-439f-bc5a62bda9b0)

#### `public void Insert()`

Inserts an analysis load combination. The AnalysisModelName has to be set.

[Docs](https://developer.tekla.com/topic/en/18/47/2fb0708b-a2a9-740b-b25e-8cab93c8f0e8)

#### `public void Modify()`

Modifies an analysis load combination. The AnalysisModelName and ID have to be set.

[Docs](https://developer.tekla.com/topic/en/18/47/5b5a160e-31f8-767c-0533-ac0a0caf0854)

#### `public override void Select()`

Selects an analysis load combination. The AnalysisModelName and ID have to be set.

[Docs](https://developer.tekla.com/topic/en/18/47/a035c2cd-3fbc-3158-6575-808a2262cbde)

#### `public void SetLoads(List<Identifier> groupIDs, List<double> scalingFactors, List<double> reductionFactors, List<double> partialSafetyFactors)`

Sets the loads.

**Parameters:**
- `groupIDs` (System.Collections.Generic.List<Identifier>) — The group identifiers.
- `scalingFactors` (System.Collections.Generic.List<Double>) — The scaling factors.
- `reductionFactors` (System.Collections.Generic.List<Double>) — The reduction factors.
- `partialSafetyFactors` (System.Collections.Generic.List<Double>) — The partial safety factors.

[Docs](https://developer.tekla.com/topic/en/18/47/2da09cbf-b58c-9aab-7c3a-e2a4797d2d8a)

### Properties
- `AnalysisModelName` (String, get/set) — Gets or sets the analysis model name.
- `AnalysisObjectType` (AnalysisObject.AnalysisObjectEnum, get/set) — Gets or sets the type of the analysis object.
- `FatherObject2ID` (Identifier, get/set) — Gets or sets the identifier of the father part.
- `FatherObject2Type` (AnalysisObject.AnalysisObjectEnum, get/set) — Gets or sets the type of the father object.
- `FatherObjectID` (Identifier, get/set) — Gets or sets the identifier of the father part.
- `FatherObjectType` (AnalysisObject.AnalysisObjectEnum, get/set) — Gets or sets the type of the father object.
- `GroupIDs` (List<Identifier>, get/set) — Gets or sets the identifiers of the groups of the analysis load combination.
- `ID` (Identifier, get/set) — Gets or sets the identifier of the analysis object.
- `Name` (String, get/set) — Gets or sets the name of the analysis load combination.
- `PartialSafetyFactors` (List<Double>, get/set) — Gets or sets the partial safety factors of the analysis load combination.
- `ReductionFactors` (List<Double>, get/set) — Gets or sets the reduction factors of the analysis load combination.
- `ScalingFactors` (List<Double>, get/set) — Gets or sets the scaling factors of the analysis load combination.
- `Type` (AnalysisLoadCombination.LoadCombinationEnum, get/set) — Gets or sets the combination type of the analysis load combination.

## AnalysisLoadCombination.LoadCombinationEnum (enum)

The load combination types.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/4e32fda4-81f6-2f3c-b147-125b3902f8d2)

### Values
- `NOT_DEFINED` = `0` — The load combination type has not been defined.
- `LG` = `1` — The LG load combination type.
- `ULS` = `2` — The ULS load combination type.
- `SLSRC` = `3` — The SLSRC load combination type.
- `SLSQP` = `4` — The SLSQP load combination type.
- `ULSAC` = `5` — The ULSAC load combination type.
- `USLEQ` = `6` — The USLEQ load combination type.
- `SLS` = `7` — The SLS load combination type.
- `SLSFC` = `8` — The SLSFC load combination type.
- `ULSNL` = `9` — The ULSNL load combination type.
- `ULSEL` = `10` — The ULSEL load combination type.
- `SLSD` = `11` — The SLSD load combination type.
- `ACC` = `12` — The ACC load combination type.
- `SLSNL` = `13` — The SLSNL load combination type.
- `ULSUL` = `14` — The ULSUL load combiantion type.
- `ACCUL` = `15` — The ACCUL load combination type.
- `LFPSS` = `16` — The LFPSS load combination type.
- `LFPSSWS` = `17` — The LFPSSWS load combination type.
- `LFSS` = `18` — The LFSS load combination type.
- `LFSSWS` = `19` — The LFSSWS load combination type.
- `LFPCS` = `20` — The LFPCS load combination type.
- `LFPCSWS` = `21` — The LFPCSWS load combination type.
- `LFCS` = `22` — The LFCS load combination type.
- `LFCSWS` = `23` — The LFCSWS load combination type.
- `LFPS` = `24` — The LFPS load combination type.
- `LFPSWS` = `25` — The LFPSWS load combination type.
- `LFNPS` = `26` — The LFNPS load combination type.
- `LFNPSWS` = `27` — The LFNPSWS load combination type.
- `ACITABLE1` = `28` — The ACITABLE1 load combination type.
- `ACITABLE2` = `29` — The ACITABLE2 load combination type.
- `ACITABLE3` = `30` — The ACITABLE3 load combination type.
- `ACITABLE4` = `31` — The ACITABLE4 load combination type.
- `ACITABLE5` = `32` — The ACITABLE5 load combination type.
- `ACITABLE6` = `33` — The ACITABLE6 load combination type.
- `ACITABLE7` = `34` — The ACITABLE7 load combination type.
- `ACITABLE8` = `35` — The ACITABLE8 load combination type.

## AnalysisLoadGroup (class)

The AnalysisLoadGroup class contains information related to analysis load groups.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/24cabe5a-7528-384d-0a1b-e9c8ad887b43)

### Constructors
- `public AnalysisLoadGroup()` — Initializes a new instance of the AnalysisLoadGroup class.

### Methods
#### `public static string GetTypeName(AnalysisLoadGroup.LoadGroupTypeEnum type)`

Get load group type name, translated to current language.

**Parameters:**
- `type` (Tekla.Structures.Analysis.AnalysisLoadGroup.LoadGroupTypeEnum) — Load group type.

**Returns:** `String` — The load group type name, translated to current language.

[Docs](https://developer.tekla.com/topic/en/18/47/c8df68e9-6555-bd7d-de0b-0798e23adf1d)

#### `public bool IsEmpty()`

Check if load group is empty (no loading exists).

**Returns:** `Boolean` — True if load group is empty (no loading exists).

[Docs](https://developer.tekla.com/topic/en/18/47/bea58da2-9fb8-b013-3c35-22e720c6e37d)

#### `public bool IsSelfWeight()`

Check if load group is self weight load group.

**Returns:** `Boolean` — True if load group is self weight load group.

[Docs](https://developer.tekla.com/topic/en/18/47/40c25a63-12da-aee2-c68a-d932b7ce09a1)

#### `public override void Select()`

Selects an AnalysisLoadGroup. The ID has to be set.

[Docs](https://developer.tekla.com/topic/en/18/47/79cc408a-009f-71df-c73e-0762d0e7b95c)

### Properties
- `AnalysisModelName` (String, get/set) — Gets or sets the analysis model name.
- `AnalysisObjectType` (AnalysisObject.AnalysisObjectEnum, get/set) — Gets or sets the type of the analysis object.
- `Compatible` (Boolean, get/set) — Gets or sets a value indicating whether the load group is compatible.
- `CompatibleNumber` (Int32, get/set) — Gets or sets the load group compatible number.
- `Direction` (AnalysisLoadGroup.LoadGroupDirectionEnum, get/set) — Gets or sets the direction of the load group.
- `FatherObject2ID` (Identifier, get/set) — Gets or sets the identifier of the father part.
- `FatherObject2Type` (AnalysisObject.AnalysisObjectEnum, get/set) — Gets or sets the type of the father object.
- `FatherObjectID` (Identifier, get/set) — Gets or sets the identifier of the father part.
- `FatherObjectType` (AnalysisObject.AnalysisObjectEnum, get/set) — Gets or sets the type of the father object.
- `ID` (Identifier, get/set) — Gets or sets the identifier of the analysis object.
- `Incompatible` (Boolean, get/set) — Gets or sets a value indicating whether the load group is incompatible.
- `IncompatibleNumber` (Int32, get/set) — Gets or sets the load group incompatible number.
- `LoadGroupID` (Identifier, get/set) — Gets or sets the identifier of the load group.
- `Name` (String, get/set) — Gets or sets the name of the load group.
- `Type` (AnalysisLoadGroup.LoadGroupTypeEnum, get/set) — Gets or sets the type of the load group.

## AnalysisLoadGroup.LoadGroupDirectionEnum (enum)

The directions of a load group.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/134c41eb-842f-c9f0-a7ce-8165fb7fe744)

### Values
- `DIRECTION_X` = `1` — The X direction.
- `DIRECTION_Y` = `2` — The Y direction.
- `DIRECTION_Z` = `3` — The Z direction.
- `DIRECTION_NEGATIVE_X` = `4` — The negative X direction.
- `DIRECTION_NEGATIVE_Y` = `5` — The negative Y direction.
- `DIRECTION_NEGATIVE_Z` = `6` — The negative Z direction.

## AnalysisLoadGroup.LoadGroupTypeEnum (enum)

The types of load groups.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/35b03eb4-d4e1-c307-377e-c524513588fc)

### Values
- `NOT_DEFINED` = `0` — The load group has not been defined.
- `LOADM_DEFAULT_GROUP` = `1000` — The default load group.
- `LOADM_SELF_WEIGHT_EC` = `1001` — The Eurocode self weight type.
- `LOADM_PERMANENT_LOAD_EC` = `1002` — The Eurocode permanent load type.
- `LOADM_PRE_STRESS_LOAD_EC` = `1003` — The Eurocode pre-stress load type.
- `LOADM_LIVE_LOAD_HOUSING_EC` = `1004` — The Eurocode live load housing type.
- `LOADM_LIVE_LOAD_PUBLIC_BUILDINGS_EC` = `1005` — The Eurocode live load public buildings type.
- `LOADM_LIVE_LOAD_THEATRES_RESTAURANTS_EC` = `1006` — The Eurocode live load theatres, restaurants type.
- `LOADM_LIVE_LOAD_WAREHOUSES_EC` = `1007` — The Eurocode live load warehouses type.
- `LOADM_LIVE_LOAD_STORAGE_BUILDINGS_EC` = `1008` — The Eurocode live load storage buildings type.
- `LOADM_TRAFFIC_LOAD_LIGHT_VEHICLES_EC` = `1009` — The Eurocode traffic load light vehicles type.
- `LOADM_TRAFFIC_LOAD_HEAVY_VEHICLES_EC` = `1010` — The Eurocode traffic load heavy vehicles type.
- `LOADM_TRAFFIC_LOAD_ROOFS_EC` = `1011` — The Eurocode traffic load roofs type.
- `LOADM_SNOW_LOAD_EC` = `1012` — The Eurocode snow load type.
- `LOADM_WIND_LOAD_EC` = `1013` — The Eurocode wind load type.
- `LOADM_FORCES_DUE_TO_TEMPERATURE_EFFECTS_EC` = `1014` — The Eurocode forces due to temperature effects type.
- `LOADM_IMPERFECTION_LOAD_DUE_TO_DEAD_LOADS_EC` = `1015` — The Eurocode imperfection load due to dead loads type.
- `LOADM_IMPERFECTION_LOAD_DUE_TO_LIVE_LOADS_EC` = `1016` — The Eurocode imperfection load due to live loads type.
- `LOADM_IMPERFECTION_LOAD_DUE_TO_SNOW_LOADS_EC` = `1017` — The Eurocode imperfection load due to snow loads type.
- `LOADM_ACCIDENTAL_LOAD_EC` = `1018` — The Eurocode accidental load type.
- `LOADM_EARTHQUAKE_LOAD_EC` = `1019` — The Eurocode earthquake load type.
- `LOADM_DEAD_LOAD_ALONE_BS` = `2001` — The British code dead load alone type.
- `LOADM_DEADLOAD_BS` = `2002` — The British code dead load type.
- `LOADM_IMPOSED_LOAD_BS` = `2003` — The British code imposed load type.
- `LOADM_WIND_LOAD_BS` = `2004` — The British code wind load type.
- `LOADM_TEMPERATURE_LOAD_BS` = `2005` — The British code temperature load type.
- `LOADM_VERTICAL_CRANE_LOAD_BS` = `2006` — The British code vertical crane load type.
- `LOADM_HORIZONTAL_CRANE_LOAD_BS` = `2007` — The British code horizontal crane load type.
- `LOADM_DEAD_LOAD_ALONE_AISC` = `3001` — The AISC dead load alone type.
- `LOADM_DEAD_LOAD_AISC` = `3002` — The AISC dead load type.
- `LOADM_LIVE_LOAD_AISC` = `3003` — The AISC live load type.
- `LOADM_ROOF_LIVE_LOAD_AISC` = `3004` — The AISC roof live load type.
- `LOADM_WIND_LOAD_AISC` = `3005` — The AISC wind load type.
- `LOADM_SNOW_LOAD_AISC` = `3006` — The AISC snow load type.
- `LOADM_RAINWATER_LOAD_AISC` = `3007` — The AISC rainwater load type.
- `LOADM_EARTHQUAKE_LOAD_AISC` = `3008` — The AISC earthquake load type.
- `LOADM_PERMANENT_LOAD_FR_CM66` = `6001` — The CM66 permanent load type.
- `LOADM_EXPLOITATION_LOAD_FR_CM66` = `6002` — The CM66 exploitation load type.
- `LOADM_TEMPERATURE_LOAD_FR_CM66` = `6003` — The CM66 temperature load type.
- `LOADM_WIND_LOAD_FR_CM66` = `6004` — The CM66 wind load type.
- `LOADM_SNOW_LOAD_FR_CM66` = `6005` — The CM66 snow load type.
- `LOADM_SEISMIC_LOAD_HORIZONTAL_FR_CM66` = `6006` — The CM66 seismic load horizontal type.
- `LOADM_SEISMIC_LOAD_VERTICAL_FR_CM66` = `6007` — The CM66 seismic load vertical type.
- `LOADM_PERMANENT_LOAD_FR_BAEL91` = `7001` — The BAEL91 permanent load type.
- `LOADM_EXPLOITATION_LOAD_FR_BAEL91` = `7002` — The BAEL91 exploitation load type.
- `LOADM_TEMPERATURE_LOAD_FR_BAEL91` = `7003` — The BAEL91 temperature load type.
- `LOADM_WIND_LOAD_FR_BAEL91` = `7004` — The BAEL91 wind load type.
- `LOADM_SNOW_LOAD_FR_BAEL91` = `7005` — The BAEL91 snow load type.
- `LOADM_SEISMIC_LOAD_FR_BAEL91` = `7006` — The BAEL91 seismic load type.
- `LOADM_ACCIDENTAL_LOAD_FR_BAEL91` = `7007` — The BAEL91 accidental load type.
- `LOADM_DEAD_LOAD_UBC` = `8001` — The UBC dead load type.
- `LOADM_LIVE_LOAD_UBC` = `8002` — The UBC live load type.
- `LOADM_ROOF_LIVE_LOAD_UBC` = `8003` — The UBC roof live load type.
- `LOADM_WIND_LOAD_UBC` = `8004` — The UBC wind load type.
- `LOADM_SNOW_LOAD_UBC` = `8005` — The UBC snow load type.
- `LOADM_TEMPERATURE_LOAD_UBC` = `8006` — The UBC temperature load type.
- `LOADM_FLUIDS_LOAD_UBC` = `8007` — The UBC fluids load type.
- `LOADM_SOIL_LOAD_UBC` = `8008` — The UBC soil load type.
- `LOADM_PONDING_LOAD_UBC` = `8009` — The UBC ponding load type.
- `LOADM_SEISMIC_LOAD_UBC` = `8010` — The UBC seismic load type.
- `LOADM_DEAD_LOAD_IBC` = `9001` — The IBC dead load type.
- `LOADM_LIVE_LOAD_IBC` = `9002` — The IBC live load type.
- `LOADM_ROOF_LIVE_LOAD_IBC` = `9003` — The IBC roof live load type.
- `LOADM_WIND_LOAD_IBC` = `9004` — The IBC wind load type.
- `LOADM_SNOW_LOAD_IBC` = `9005` — The IBC snow load type.
- `LOADM_TEMPERATURE_LOAD_IBC` = `9006` — The IBC temperature load type.
- `LOADM_FLUIDS_LOAD_IBC` = `9007` — The IBC fluids load type.
- `LOADM_SOIL_LOAD_IBC` = `9008` — The IBC soil load type.
- `LOADM_RAIN_LOAD_IBC` = `9009` — The IBC rain load type.
- `LOADM_PONDING_LOAD_IBC` = `9010` — The IBC ponding load type.
- `LOADM_SEISMIC_LOAD_IBC` = `9011` — The IBC seismic load type.
- `LOADM_DEAD_LOAD_ACI` = `9101` — The ACI dead load type.
- `LOADM_LIVE_LOAD_ACI` = `9102` — The ACI live load type.
- `LOADM_ROOF_LIVE_LOAD_ACI` = `9103` — The ACI roof live load type.
- `LOADM_WIND_LOAD_ACI` = `9104` — The ACI wind load type.
- `LOADM_SEISMIC_LOAD_ACI` = `9105` — The ACI seismic load type.
- `LOADM_SNOW_LOAD_ACI` = `9106` — The ACI snow load type.
- `LOADM_FLUIDS_LOAD_ACI` = `9107` — The ACI fluids load type.
- `LOADM_SOIL_LOAD_ACI` = `9108` — The ACI soil load type.
- `LOADM_RAIN_LOAD_ACI` = `9109` — The ACI rain load type.
- `LOADM_TEMPERATURE_LOAD_ACI` = `9110` — The ACI temperature load type.

## AnalysisLocalCoordinateSystem (class)

The AnalysisLocalCoordinateSystem class contains local coordinate system information for an analysis model.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/7dd7ab45-ce94-0f2f-0fb3-cbb823608016)

### Constructors
- `public AnalysisLocalCoordinateSystem()` — Creates a new analysis local coordinate system instance.

### Properties
- `LocalX` (Vector, get) — The local X direction.
- `LocalY` (Vector, get) — The local Y direction.

## AnalysisMaterial (class)

The AnalysisMaterial class contains the material information.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/ef408345-da5e-30c3-daae-ed0ba09e502f)

### Constructors
- `public AnalysisMaterial()` — Initializes a new instance of the AnalysisMaterial class.

### Methods
#### `public override void Select()`

Selects an analysis material. The AnalysisModelName and Name have to be set.

[Docs](https://developer.tekla.com/topic/en/18/47/33e681ec-38e0-1b5a-c4ae-a85d39a266d3)

### Properties
- `AnalysisModelName` (String, get/set) — Gets or sets the analysis model name.
- `AnalysisObjectType` (AnalysisObject.AnalysisObjectEnum, get/set) — Gets or sets the type of the analysis object.
- `DampingFactor` (Double, get/set) — Gets or sets the damping factor of the material.
- `Density` (Double, get/set) — Gets or sets the density of the material.
- `FatherObject2ID` (Identifier, get/set) — Gets or sets the identifier of the father part.
- `FatherObject2Type` (AnalysisObject.AnalysisObjectEnum, get/set) — Gets or sets the type of the father object.
- `FatherObjectID` (Identifier, get/set) — Gets or sets the identifier of the father part.
- `FatherObjectType` (AnalysisObject.AnalysisObjectEnum, get/set) — Gets or sets the type of the father object.
- `ID` (Identifier, get/set) — Gets or sets the identifier of the analysis object.
- `MaterialType` (AnalysisMaterial.MaterialTypeEnum, get/set) — Gets or sets tThe type of the material.
- `ModulusOfElasticity` (Double, get/set) — Gets or sets the modulus of elasticity of the analysis material.
- `Name` (String, get/set) — Gets or sets the name of the analysis material.
- `PoissonsRatio` (Double, get/set) — Gets or sets the Poisson's ratio of the analysis material.
- `ThermalDilatation` (Double, get/set) — Gets or sets the thermal dilatation of the analysis material.

## AnalysisMaterial.MaterialTypeEnum (enum)

The material types.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/5430e7b6-0a98-1465-cc66-cfee387e5957)

### Values
- `STEEL` = `1` — The steel material type.
- `CONCRETE` = `2` — The concrete material type.
- `TIMBER` = `5` — The timber material type.

## AnalysisMember (class)

The AnalysisMember class contains information related to analysis members.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/714466d6-06c3-35b3-2a23-0895a5394913)

### Constructors
- `public AnalysisMember()` — Initializes a new instance of the AnalysisMember class.

### Methods
#### `public void CopyData(AnalysisMember AnalysisMember)`

Copies data from another analysis member.

**Parameters:**
- `AnalysisMember` (Tekla.Structures.Analysis.AnalysisMember) — The analysis member to copy data from.

[Docs](https://developer.tekla.com/topic/en/18/47/7b9bab44-4378-ccac-812d-feabc828a0ba)

#### `public void Modify()`

Modifies an analysis member. The AnalysisModelName has to be set.

[Docs](https://developer.tekla.com/topic/en/18/47/bea6cbda-7622-5b8d-4acf-2a6883299bc8)

#### `public override void Select()`

Selects an analysis member. The AnalysisModelName and ID have to be set.

[Docs](https://developer.tekla.com/topic/en/18/47/2145077e-6a7d-dc86-b945-0929c497d700)

### Properties
- `AnalysisDesignCode` (AnalysisDesignCode, get/set) — Gets or sets the analysis design code of the member.
- `AnalysisModelName` (String, get/set) — Gets or sets the analysis model name.
- `AnalysisObjectType` (AnalysisObject.AnalysisObjectEnum, get/set) — Gets or sets the type of the analysis object.
- `Composite` (AnalysisCompositeBeam, get/set) — Gets or sets the composite settings of the member.
- `Curvature` (AnalysisCurvature, get/set) — Gets or sets the beam curvature of the member.
- `End` (AnalysisBeamEnd, get/set) — Gets or sets the analysis beam end settings at the end of the member.
- `FatherObject2ID` (Identifier, get/set) — Gets or sets the identifier of the father part.
- `FatherObject2Type` (AnalysisObject.AnalysisObjectEnum, get/set) — Gets or sets the type of the father object.
- `FatherObjectID` (Identifier, get/set) — Gets or sets the identifier of the father part.
- `FatherObjectType` (AnalysisObject.AnalysisObjectEnum, get/set) — Gets or sets the type of the father object.
- `ID` (Identifier, get/set) — Gets or sets the identifier of the analysis object.
- `Label` (AnalysisLabel, get/set) — Gets or sets the label of the analysis member.
- `Material` (AnalysisMaterial, get/set) — Gets or sets the material of the analysis member.
- `Start` (AnalysisBeamEnd, get/set) — Gets or sets the analysis beam end settings at the start of the member.

## AnalysisModel (class)

The AnalysisModel class contains information related to analysis models.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/8186bfc0-ae67-5ac2-8c87-c9ba59724faf)

### Constructors
- `public AnalysisModel()` — Initializes a new instance of the AnalysisModel class.

### Methods
#### `public void Delete()`

Deletes an analysis model. The AnalysisModelName has to be set.

[Docs](https://developer.tekla.com/topic/en/18/47/0d04a451-e528-f758-6971-4f844133eef7)

#### `public List<AnalysisModelIssue> GetIssues()`

Get analysis model issues.

**Returns:** `List<AnalysisModelIssue>` — The analysis model issues.

[Docs](https://developer.tekla.com/topic/en/18/47/8c9e3dbc-63d7-69b2-e63e-0192196652ae)

#### `public void Insert()`

Inserts a new analysis model into the Tekla Structures model. The AnalysisModelName has to be set.

[Docs](https://developer.tekla.com/topic/en/18/47/cf8651fe-b4fe-9219-a849-978ef567254c)

#### `public void Modify()`

Modifies an analysis model. The AnalysisModelName has to be set.

[Docs](https://developer.tekla.com/topic/en/18/47/70a4a6f1-39f2-6e34-d1e5-fea6d37dc33f)

#### `public bool Select(bool update)`

Selects an analysis model. The AnalysisModelName or ID has to be set.

**Parameters:**
- `update` (System.Boolean) — Indicates whether analysis model is updated.

**Returns:** `Boolean` — True if successful.

[Docs](https://developer.tekla.com/topic/en/18/47/6f0dc619-b143-ca2b-7fec-9f75294ba493)

#### `public override void Select()`

Selects an analysis model. Updates the model. The AnalysisModelName or ID has to be set.

[Docs](https://developer.tekla.com/topic/en/18/47/754a6dbd-1581-b4bf-98cc-f4334db31c84)

### Properties
- `AnalysisConnectivityRules` (AnalysisObjectEnumerator, get/set) — Gets or sets the analysis model connectivity rules.
- `AnalysisEngine` (String, get/set) — Gets or sets the analysis engine of the analysis model.
- `AnalysisLoadCombinations` (AnalysisObjectEnumerator, get/set) — Gets the analysis load combinations of the analysis model.
- `AnalysisLoadGroups` (AnalysisObjectEnumerator, get/set) — Gets the analysis load groups of the analysis model.
- `AnalysisMethod` (AnalysisModel.AnalysisMethodEnum, get/set) — Gets or sets the analysis method of the analysis model.
- `AnalysisModelDesignProperties` (AnalysisModelDesignProperties, get/set) — Gets or sets the analysis model design properties.
- `AnalysisModelJobProperties` (AnalysisModelJobProperties, get/set) — Gets or sets the analysis model job properties.
- `AnalysisModelModalAnalysisProperties` (AnalysisModelModalAnalysisProperties, get/set) — Gets or sets the analysis model modal analysis properties.
- `AnalysisModelName` (String, get/set) — Gets or sets the analysis model name.
- `AnalysisModelOutputProperties` (AnalysisModelOutputProperties, get/set) — Gets or sets the analysis model output properties.
- `AnalysisModelSeismicProperties` (AnalysisModelSeismicProperties, get/set) — Gets or sets the analysis model seismic properties.
- `AnalysisModelSpectrumProperties` (AnalysisModelSpectrumProperties, get/set) — Gets or sets the analysis model spectrum properties
- `AnalysisNodeLinks` (AnalysisObjectEnumerator, get) — Gets the analysis node links of the analysis model.
- `AnalysisNodes` (AnalysisObjectEnumerator, get) — Gets the analysis nodes of the analysis model.
- `AnalysisObjectType` (AnalysisObject.AnalysisObjectEnum, get/set) — Gets or sets the type of the analysis object.
- `AnalysisParts` (AnalysisObjectEnumerator, get) — Gets the analysis parts of the analysis model.
- `AnalysisRigidDiaphragms` (AnalysisObjectEnumerator, get/set) — Gets the analysis rigid diaphragms of the analysis model.
- `AutodetectSecondaries` (Boolean, get/set) — Gets or sets a value indicating whether bracing members are detected automatically.
- `BracingFilterName` (String, get/set) — Gets or sets the bracing filter name.
- `ConstructionMethod` (AnalysisModel.ConstructionMethodEnum, get/set) — Gets or sets the construction method of the analysis model.
- `FatherObject2ID` (Identifier, get/set) — Gets or sets the identifier of the father part.
- `FatherObject2Type` (AnalysisObject.AnalysisObjectEnum, get/set) — Gets or sets the type of the father object.
- `FatherObjectID` (Identifier, get/set) — Gets or sets the identifier of the father part.
- `FatherObjectType` (AnalysisObject.AnalysisObjectEnum, get/set) — Gets or sets the type of the father object.
- `FilterName` (String, get/set) — Gets or sets the filter name of the analysis model.
- `ID` (Identifier, get/set) — Gets or sets the identifier of the analysis object.
- `IncludeImperfections` (Boolean, get/set) — Gets or sets a value indicating whether imperfections are to be included.
- `MemberAxislocation` (AnalysisModel.AxisLocationEnum, get/set) — Gets or sets the axis location of the members.
- `MemberEndReleaseMethodByJoint` (Boolean, get/set) — Gets or sets a value indicating whether the end releases are set by joint properties.
- `ModelUpdateModificationIdentifier` (Identifier, get/set) — Gets or sets the identifier of the model update modification.
- `NodePositionTolerance` (Double, get/set) — Gets or sets the node position tolerance of the analysis model.
- `NumberOfIterations` (Int32, get/set) — Gets or sets the number of iterations of the analysis model.
- `PhysicalFreezed` (Boolean, get/set) — Gets or sets a value indicating whether the physical objects are frozen.
- `PhysicalFrozen` (Boolean, get/set) — Gets or sets a value indicating whether the physical objects are frozen.
- `RelativeAccuracy` (Double, get/set) — Gets or sets the relative accuracy of the analysis model.
- `ResultsFreezed` (Boolean, get/set) — Gets or sets a value indicating whether the results are frozen.
- `ResultsFrozen` (Boolean, get/set) — Gets or sets a value indicating whether the results are frozen.
- `ResultsModificationIdentifier` (Identifier, get/set) — Gets or sets the results modification identifier.
- `ResultsTime` (String, get/set) — Gets or sets the results time.
- `SecondaryFilterName` (String, get/set) — Gets or sets the bracing filter name.
- `SecondaryKeepAxis` (AnalysisModel.KeepAxisEnum, get/set) — Gets or sets the secondary keep axis settings. Not used, bracing/secondary members always have KEEP_AXIS_NO initially.
- `SecondaryMemberFilterName` (String, get/set) — Gets or sets the secondary member filter name.
- `SnapDistance` (Double, get/set) — Gets or sets the snap distance of the analysis model.
- `UseModelMerge` (Boolean, get/set) — Gets or sets a value indicating whether model merge is used.
- `UseTrueCurvedMembers` (Boolean, get/set) — Gets or sets a value indicating whether members are true curved.
- `UseTwinProfiles` (Boolean, get/set) — Gets or sets a value indicating whether twin profiles are used.

## AnalysisModel.AnalysisMethodEnum (enum)

The analysis methods.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/35e76a03-c6d3-de4e-d51f-a540469de1f1)

### Values
- `FIRST_ORDER` = `1` — The first order analysis method.
- `SECOND_ORDER` = `2` — The second order analysis method.
- `P_DELTA` = `3` — The P Delta analysis method.

## AnalysisModel.AxisLocationEnum (enum)

The axis location.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/bcbb854f-2310-7356-8706-f5eb9389867e)

### Values
- `REFERENCE_AXIS_WITH_ECCENTRICITY` = `0` — The axis location is the reference axis with eccentricity.
- `REFERENCE_AXIS` = `1` — The axis location is the reference axis.
- `MODEL_PART` = `2` — The axis location setting of each part is used.
- `NEUTRAL_AXIS` = `3` — The axis location is the neutral axis.

## AnalysisModel.ConstructionMethodEnum (enum)

The construction methods.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/b06c0307-de43-92c7-81b1-41fac45c89ee)

### Values
- `FULL` = `1` — The construction method is "full".
- `BOX` = `2` — The construction method is "box".
- `SELECTED_PARTS` = `3` — The construction method is "with selected parts".
- `SELECTED_PARTS_AND_LOADS` = `4` — The construction method is "with selected parts and loads".
- `FLOOR_BY_SELECTED_PARTS_AND_LOADS` = `11` — The construction method is "floor by selected parts and loads".

## AnalysisModel.KeepAxisEnum (enum)

The keep axis settings.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/cd949ee2-54f6-c2a9-9c00-ccb7000fda70)

### Values
- `KEEP_AXIS_NO` = `0` — The keep axis setting is "no".
- `KEEP_AXIS_LIMITED` = `1` — The keep axis setting is "limited".
- `KEEP_AXIS_YES` = `2` — The keep axis setting is "yes".
- `KEEP_AXIS_FULL` = `3` — The keep axis setting is "full".

## AnalysisModelAnalysisProperties (class)

The AnalysisModelAnalysisProperties class contains the analysis property information for an analysis model.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/df9bd9d1-a56b-8056-735f-a3a523c4dab3)

### Constructors
- `public AnalysisModelAnalysisProperties()` — Creates a new analysis model analysis properties instance.

### Properties
- `AccuracyOfIterations` (Double, get/set) — The accuracy of the iterations.
- `AnalysisMethod` (AnalysisModelAnalysisProperties.AnalysisMethodEnum, get/set) — The analysis method.
- `MaxIterations` (Int32, get/set) — The maximum number of iterations.

## AnalysisModelAnalysisProperties.AnalysisMethodEnum (enum)

Defines the analysis method.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/5f51756b-a4ea-585c-f137-e0fb47cbda85)

### Values
- `FIRST_ORDER` = `1` — The first order calculation method.
- `P_DELTA` = `2` — The P-delta calculation method.
- `NON_LINEAR` = `3` — The non-linear calculation method.

## AnalysisModelConnectivityRule (class)

The AnalysisModelConnectivityRule class contains information related to analysis model connectivity rules.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/d6c5fe24-23bd-5e24-35d5-cfe63d1f4195)

### Constructors
- `public AnalysisModelConnectivityRule()` — Initializes a new instance of the AnalysisModelConnectivityRule class

### Methods
#### `public override void Select()`

Selects an analysis connectivity rule. The ID has to be set.

[Docs](https://developer.tekla.com/topic/en/18/47/2f473acf-e09d-0c80-08d2-3225114ddc7b)

### Properties
- `AnalysisModelName` (String, get/set) — Gets or sets the analysis model name.
- `AnalysisObjectType` (AnalysisObject.AnalysisObjectEnum, get/set) — Gets or sets the type of the analysis object.
- `FatherObject2ID` (Identifier, get/set) — Gets or sets the identifier of the father part.
- `FatherObject2Type` (AnalysisObject.AnalysisObjectEnum, get/set) — Gets or sets the type of the father object.
- `FatherObjectID` (Identifier, get/set) — Gets or sets the identifier of the father part.
- `FatherObjectType` (AnalysisObject.AnalysisObjectEnum, get/set) — Gets or sets the type of the father object.
- `Group1` (String, get/set) — The group name 1 of the connectivity rule.
- `Group2` (String, get/set) — The group name 2 of the connectivity rule.
- `ID` (Identifier, get/set) — Gets or sets the identifier of the analysis object.
- `Linkage1` (Int32, get/set) — The linkage 1 of the connectivity rule.
- `Linkage2` (Int32, get/set) — The linkage 2 of the connectivity rule.
- `MagneticDistance` (Double, get/set) — The magnetic distance of the connectivity rule.
- `Status` (Int32, get/set) — The status of the connectivity rule.

## AnalysisModelDesignProperties (class)

The AnalysisModelDesignProperties class contains information related to analysis model design properties.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/7cd03721-f7d8-15f8-052e-a25114c9da1a)

### Constructors
- `public AnalysisModelDesignProperties()` — Initializes a new instance of the AnalysisModelDesignProperties class

### Properties
- `ConcreteDesign` (AnalysisDesignCode, get/set) — The concrete code used.
- `ConcreteDesignMethod` (AnalysisModelDesignProperties.DesignMethodEnum, get/set) — The concrete design method.
- `SteelDesign` (AnalysisDesignCode, get/set) — The steel code used.
- `SteelDesignMethod` (AnalysisModelDesignProperties.DesignMethodEnum, get/set) — The steel design method.
- `TimberDesign` (AnalysisDesignCode, get/set) — The timber code used.
- `TimberDesignMethod` (AnalysisModelDesignProperties.DesignMethodEnum, get/set) — The timber design method.

## AnalysisModelDesignProperties.DesignMethodEnum (enum)

The design methods.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/815f1c81-f8d4-c17f-b993-121c5846c569)

### Values
- `NONE` = `1` — The design method is none.
- `CHECK_DESIGN` = `2` — The design method is check design.
- `OPTIMISE` = `3` — The design method is optimise.

## AnalysisModelGeneralProperties (class)

The AnalysisModelGeneralProperties class contains the general property information for an analysis model.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/535258bb-120b-fa8a-590c-32e2395a6ead)

### Constructors
- `public AnalysisModelGeneralProperties()` — Creates a new analysis model general properties instance.

### Properties
- `AnalysisEngineName` (String, get/set) — The name of the analysis engine.
- `ExtendedClashCheck` (Boolean, get/set) — Defines whether the extended clash check is used.
- `FilterName` (String, get/set) — The name of the filter.
- `MemberAxisLocation` (AnalysisModelGeneralProperties.MemberAxisLocationEnum, get/set) — The location of the member axis.
- `MemberEndReleaseMethodByConnection` (Boolean, get/set) — Defines whether the support conditions of connections or parts are used.
- `ModalAnalysisModel` (Boolean, get/set) — Defines whether the modal analysis properties are used instead of the static load combinations.
- `ModelCreationType` (AnalysisModelGeneralProperties.ModelCreationTypeEnum, get/set) — The model creation type.
- `ModelMerging` (Boolean, get/set) — Defines whether the model merging is used.
- `ModelName` (String, get/set) — The name of the model.
- `NodeDefinition` (Boolean, get/set) — Defines how the nodes and node positions are defined.

## AnalysisModelGeneralProperties.MemberAxisLocationEnum (enum)

Defines the member axis location.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/85038f7d-1ecb-5133-52b4-b9d67d860fe9)

### Values
- `MODEL_DEFAULT` = `0` — The member axis is defined individually based on each part's properties.
- `REFERENCE_AXIS_ECCENTRICITY_BY_NEUTRAL_AXIS` = `1` — The reference axis is the member axis. The location of the neutral axis defines the axis eccentricity.
- `REFERENCE_AXIS` = `2` — The reference axis is the member axis.
- `NEUTRAL_AXIS` = `3` — The neutral axis is the member axis.

## AnalysisModelGeneralProperties.ModelCreationTypeEnum (enum)

Defines the model creation type.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/1b93f9f4-65f6-8560-291a-bef55c181055)

### Values
- `FULL_MODEL` = `0` — Creates an analysis model using the full model.
- `BY_WORK_AREA` = `1` — Creates an analysis model using the selected area.
- `BY_SELECTED_PARTS` = `2` — Creates an analysis model using the selected parts.
- `BY_SELECTED_PARTS_AND_LOADS` = `3` — Creates an analysis model using the selected parts and loads.
- `FLOOR_MODEL_BY_SELECTED_PARTS_AND_LOADS` = `4` — Creates a floor model using the selected parts and loads.

## AnalysisModelHandler (class)

The AnalysisModelHandler class contains information related to analysis models.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/81f3cd95-1301-fdf4-3894-e4cea5d79aca)

### Constructors
- `public AnalysisModelHandler()` — Initializes a new instance of the AnalysisModelHandler class

### Methods
#### `public void AddLoad(AnalysisModel AnalysisModel, Load ModelLoad)`

Adds a load to the analysis model. The ConstructionMethod must be SELECTED_PARTS, SELECTED_PARTS_AND_LOADS or FLOOR_BY_SELECTED_PARTS_AND_LOADS.

**Parameters:**
- `AnalysisModel` (Tekla.Structures.Analysis.AnalysisModel) — The analysis model to be used.
- `ModelLoad` (Tekla.Structures.Model.Load) — The model load to be used.

[Docs](https://developer.tekla.com/topic/en/18/47/5546fc34-2f62-dfcf-c3c1-d4a36a0d9c11)

#### `public void AddPart(AnalysisModel AnalysisModel, Part ModelPart)`

Adds a part to the analysis model. The ConstructionMethod must be SELECTED_PARTS, SELECTED_PARTS_AND_LOADS or FLOOR_BY_SELECTED_PARTS_AND_LOADS.

**Parameters:**
- `AnalysisModel` (Tekla.Structures.Analysis.AnalysisModel) — The analysis model to be used.
- `ModelPart` (Tekla.Structures.Model.Part) — The model part to be used.

[Docs](https://developer.tekla.com/topic/en/18/47/7ef1b329-8542-0e1c-e32e-f6da38518297)

#### `public AnalysisModel GetActiveModel()`

Gets the active model.

**Returns:** `AnalysisModel` — The active model.

[Docs](https://developer.tekla.com/topic/en/18/47/9fba4ad4-3677-279c-e06e-6d986d05892e)

#### `public void RemoveLoad(AnalysisModel AnalysisModel, Load ModelLoad)`

Removes a load from the analysis model. The ConstructionMethod must be SELECTED_PARTS, SELECTED_PARTS_AND_LOADS or FLOOR_BY_SELECTED_PARTS_AND_LOADS.

**Parameters:**
- `AnalysisModel` (Tekla.Structures.Analysis.AnalysisModel) — The analysis model to be used.
- `ModelLoad` (Tekla.Structures.Model.Load) — The model load to be used.

[Docs](https://developer.tekla.com/topic/en/18/47/3f0b05fa-cd06-7e36-6871-6f21ee51e7e5)

#### `public void RemovePart(AnalysisModel AnalysisModel, Part ModelPart)`

Removes a part from the analysis model. The ConstructionMethod must be SELECTED_PARTS, SELECTED_PARTS_AND_LOADS or FLOOR_BY_SELECTED_PARTS_AND_LOADS.

**Parameters:**
- `AnalysisModel` (Tekla.Structures.Analysis.AnalysisModel) — The analysis model to be used.
- `ModelPart` (Tekla.Structures.Model.Part) — The model part to be used.

[Docs](https://developer.tekla.com/topic/en/18/47/3155a7d5-d251-6247-0273-21bcd6bd7b52)

#### `public void SetActiveModel(AnalysisModel AnalysisModel)`

Sets the active model. The AnalysisModelName or ID has to be set.

**Parameters:**
- `AnalysisModel` (Tekla.Structures.Analysis.AnalysisModel) — The analysis model to be used.

[Docs](https://developer.tekla.com/topic/en/18/47/6d4c76b2-b00d-fb79-e6e9-817f80757055)

## AnalysisModelIssue (class)

Analysis model issue (error, warning, information)

[Vendor docs](https://developer.tekla.com/topic/en/18/47/99efdba4-1ad6-54f6-0635-643d0b7563e1)

### Constructors
- `public AnalysisModelIssue()` — Initializes a new instance of the AnalysisModelIssue class

### Properties
- `AnalysisObjects` (List<AnalysisObject>, get/set) — Gets or sets the analysis objects of the issue
- `Description` (String, get/set) — Gets or sets the description of the issue, translated to the current language
- `Details` (List<String>, get/set) — Gets or sets the details of the issue, translated to the current language
- `ModelObjects` (List<ModelObject>, get/set) — Gets or sets the model objects of the issue
- `Positions` (List<Point>, get/set) — Gets or sets the positions of the issue
- `Severity` (AnalysisModelIssue.SeverityEnum, get/set) — Gets or sets the issue severity
- `Type` (AnalysisModelIssue.TypeEnum, get/set) — Gets or sets the issue type

## AnalysisModelIssue.SeverityEnum (enum)

Issue severity.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/a30423e9-4da3-a409-7a10-a4d98390af29)

### Values
- `Error` = `1` — Error
- `Warning` = `0` — Warning
- `Information` = `2` — Information

## AnalysisModelIssue.TypeEnum (enum)

Issue type.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/2b03f57c-3255-d26d-1dfe-dc5deaa567dd)

### Values
- `CrossSectionValues` = `1301` — Cross section values issue.
- `ShortRigidLink` = `1403` — Short rigid link issue.
- `ShortMember` = `1404` — Short member issue.
- `ProfileOverride` = `1405` — Part profile override issue.
- `NearNodes` = `1407` — Near nodes issue.
- `RequiredRigidLinkDisabled` = `1408` — Required rigid link disabled issue.
- `TooCloseNodesForRigidLink` = `1409` — Too close nodes for rigid link issue.
- `MemberDirectionDifference` = `1410` — Member direction difference issue.
- `BarDirectionDifference` = `1411` — Bar direction difference issue.
- `BarNotConnected` = `1412` — Bar not connected issue.
- `BarOverlapping` = `1413` — Bar overlapping issue.
- `AreaNotConnected` = `1414` — Area not connected issue.
- `AreaNotPlanar` = `1415` — Area not planar issue.
- `LoadDistribution` = `1501` — Load distribution issue.
- `DuplicateLoad` = `1502` — Duplicate load issue.
- `LoadGroupNotInCombinations` = `1503` — Load group not in load combinations issue.
- `KmodeMustNePhysicalMember` = `1416` — Buckling kmode issue.
- `DesignParameterValueCount` = `1417` — Design parameter value count issue.
- `DesignParameterValue` = `1419` — Design parameter value issue.

## AnalysisModelJobProperties (class)

The AnalysisModelJobProperties class contains information related to analysis model job properties.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/20c301d6-c98c-adcb-321e-d6997db05993)

### Constructors
- `public AnalysisModelJobProperties()` — Initializes a new instance of the AnalysisModelJobProperties class

### Properties
- `ApprovedDate` (DateTime, get/set) — The date of the approval.
- `ApprovedName` (String, get/set) — The name of approver.
- `CheckerDate` (DateTime, get/set) — The date of the checking.
- `CheckerName` (String, get/set) — The name of the checker.
- `ClientName` (String, get/set) — The name of the client.
- `EngineerDate` (DateTime, get/set) — The date of the engineering.
- `EngineerName` (String, get/set) — The name of the engineer.
- `JobComment` (String, get/set) — The comment of the job.
- `JobName` (String, get/set) — The name of the job.
- `JobNumber` (String, get/set) — The number of the job.
- `PartNumber` (String, get/set) — The number of the part.
- `ReferenceNumber` (String, get/set) — The reference number.

## AnalysisModelLoad (class)

The AnalysisModelLoad class contains information related to the model loads that are included in the analysis model. The identifier is the identifier of the Tekla.Structures.Model object.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/c16103d8-1215-79ae-2240-b355f8915be0)

### Constructors
- `public AnalysisModelLoad()` — Initializes a new instance of the AnalysisModelLoad class

### Methods
#### `public override void Select()`

Selects an analysis node. The ID has to be set.

[Docs](https://developer.tekla.com/topic/en/18/47/a493355c-c2ee-eb6b-64cc-94dfff937e34)

### Properties
- `AnalysisModelName` (String, get/set) — Gets or sets the analysis model name.
- `AnalysisObjectType` (AnalysisObject.AnalysisObjectEnum, get/set) — Gets or sets the type of the analysis object.
- `FatherObject2ID` (Identifier, get/set) — Gets or sets the identifier of the father part.
- `FatherObject2Type` (AnalysisObject.AnalysisObjectEnum, get/set) — Gets or sets the type of the father object.
- `FatherObjectID` (Identifier, get/set) — Gets or sets the identifier of the father part.
- `FatherObjectType` (AnalysisObject.AnalysisObjectEnum, get/set) — Gets or sets the type of the father object.
- `ID` (Identifier, get/set) — Gets or sets the identifier of the analysis object.

## AnalysisModelModalAnalysisProperties (class)

The AnalysisModelModalAnalysisProperties class contains information related to analysis model modal analysis properties.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/715e3969-7cde-65df-fc37-09f24d0b442e)

### Constructors
- `public AnalysisModelModalAnalysisProperties()` — Initializes a new instance of the AnalysisModelModalAnalysisProperties class

### Properties
- `ModalAnalysisIncludeSelfWeightMass` (Boolean, get/set) — Indicates whether selfweight mass is included in the modal analysis.
- `ModalAnalysisMassLoadGroupDirections` (List<Int32>, get/set) — The mass load group directions of the modal analysis.
- `ModalAnalysisMassLoadGroupFactors` (List<Double>, get/set) — The mass load group factors of the modal analysis.
- `ModalAnalysisMassLoadGroupIds` (List<Identifier>, get/set) — The mass load group indentifier of the modal analysis.
- `ModalAnalysisMaxFrequency` (Double, get/set) — The modal analysis maximum frequence.
- `ModalAnalysisModeCount` (Int32, get/set) — The modal analysis mode count.
- `ModalAnalysisModel` (Boolean, get/set) — Indicates whether a modal analysis model is used.

## AnalysisModelOutputProperties (class)

The AnalysisModelOutputProperties class contains information related to analysis model output properties.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/995eb0dd-5cef-88a8-e391-a1c2f6f6f75d)

### Constructors
- `public AnalysisModelOutputProperties()` — Initializes a new instance of the AnalysisModelOutputProperties class

### Properties
- `OutputAnalysisResults` (Boolean, get/set) — Indicates whether the analysis results are created.
- `OutputElementForces` (Boolean, get/set) — Indicates whether the element forces are created.
- `OutputElementInformation` (Boolean, get/set) — Indicates whether the element information is created.
- `OutputElementJointStresses` (Boolean, get/set) — Indicates whether the element joint stresses are created.
- `OutputElementJointStressesSolid` (Boolean, get/set) — Indicates whether the element joint stresses solid is created.
- `OutputEntireSteelTable` (Boolean, get/set) — Indicates whether the entire steel table is created.
- `OutputJointCoordinates` (Boolean, get/set) — Indicates whether the joint coordinates are created.
- `OutputJointDisplacements` (Boolean, get/set) — Indicates whether the joint displacements are created.
- `OutputMaterialProperties` (Boolean, get/set) — Indicates whether the material properties are created.
- `OutputMemberForces` (Boolean, get/set) — Indicates whether the member forces are created.
- `OutputMemberInformation` (Boolean, get/set) — Indicates whether the member information is created.
- `OutputMemberProperties` (Boolean, get/set) — Indicates whether the member properties are created.
- `OutputMemberSectionForces` (Boolean, get/set) — Indicates whether the member section forces are created.
- `OutputMemberStresses` (Boolean, get/set) — Indicates whether the member stresses are created.
- `OutputModeShapes` (Boolean, get/set) — Indicates whether the mode shapes are created.
- `OutputStoryDrift` (Boolean, get/set) — Indicates whether the story drift is created.
- `OutputSupportInformation` (Boolean, get/set) — Indicates whether the support information is created.
- `OutputSupportReactions` (Boolean, get/set) — Indicates whether the supported reactions are created.

## AnalysisModelSeismicProperties (class)

The AnalysisModelSeismicProperties class contains information related to analysis model seismic properties.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/6262ad0b-b2cc-6daa-334b-64ae23fd6c44)

### Constructors
- `public AnalysisModelSeismicProperties()` — Initializes a new instance of the AnalysisModelSeismicProperties class

### Properties
- `SeismicIncludeSelfWeightMass` (Boolean, get/set) — Indicates whether the selfweight is included.
- `SeismicMassLoadGroupFactors` (List<Double>, get/set) — The factors of the mass load group.
- `SeismicMassLoadGroupIds` (List<Identifier>, get/set) — The identifier of the mass load group.
- `SeismicParameters` (List<Double>, get/set) — The seismic parameters.

## AnalysisModelSpectrumProperties (class)

The AnalysisModelSpectrumProperties class contains information related to analysis model spectrum properties.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/25c416a5-cb07-6343-4334-8e85a092f6d5)

### Constructors
- `public AnalysisModelSpectrumProperties()` — Initializes a new instance of the AnalysisModelSpectrumProperties class

### Properties
- `SpectrumAcceleration` (List<Double>, get/set) — The list of the accelerations of the spectrum.
- `SpectrumASCEf1` (Double, get/set) — The ASCEf1 value of the spectrum.
- `SpectrumASCEf2` (Double, get/set) — The ASCEf2 value of the spectrum.
- `SpectrumCombination` (Int32, get/set) — The spectrum combination.
- `SpectrumCompositeDampingRatioX` (List<Double>, get/set) — The list of the composite damping rations in the X direction of the spectrum.
- `SpectrumCompositeDampingRatioY` (List<Double>, get/set) — The list of the composite damping rations in the Y direction of the spectrum.
- `SpectrumCompositeDampingRatioZ` (List<Double>, get/set) — The list of the composite damping rations in the Z direction of the spectrum.
- `SpectrumDampingFactor` (Double, get/set) — The damping factor of the spectrum.
- `SpectrumDampingPointID` (List<Identifier>, get/set) — The list of the damping point identifiers of the spectrum.
- `SpectrumDampingType` (Int32, get/set) — The type of the spectrum damping.
- `SpectrumInterpolation` (Int32, get/set) — The interpolation of the spectrum.
- `SpectrumMaximumDampingRatio` (Double, get/set) — The maximum damping ratio of the spectrum.
- `SpectrumMinimumDampingRatio` (Double, get/set) — The minimum damping ratio of the spectrum.
- `SpectrumMissingMass` (Double, get/set) — The missing mass of the spectrum.
- `SpectrumMissingMassEnable` (Boolean, get/set) — Indicates whether the missing mass of the spectrum is enabled.
- `SpectrumModalDampingRatio` (List<Double>, get/set) — The list of the modal damping ratios of the spectrum.
- `SpectrumModalDampingRatioEnable` (Boolean, get/set) — Indicates whether the damping ratio of the spectrum is enabled.
- `SpectrumModeCount` (Int32, get/set) — The count of the spectrum mode.
- `SpectrumPeriod` (List<Double>, get/set) — The list of the periods of the spectrum.
- `SpectrumScale` (Double, get/set) — The scale of the spectrum.
- `SpectrumType` (Int32, get/set) — The type of the spectrum.
- `SpectrumXdirection` (Double, get/set) — The X direction of the spectrum.
- `SpectrumXdirectionEnable` (Boolean, get/set) — Indicates whether the X direction of the spectrum is enabled.
- `SpectrumYdirection` (Double, get/set) — The Y direction of the spectrum.
- `SpectrumYdirectionEnable` (Boolean, get/set) — Indicates whether the Y direction of the spectrum is enabled.
- `SpectrumZdirection` (Double, get/set) — The Z direction of the spectrum.
- `SpectrumZdirectionEnable` (Boolean, get/set) — Indicates whether the Z direction of the spectrum is enabled.

## AnalysisNode (class)

The AnalysisNode class contains information related to analysis nodes.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/ec555e9b-0b3d-d1e4-efe5-f3340b40f0c0)

### Constructors
- `public AnalysisNode()` — Initializes a new instance of the AnalysisNode class.

### Methods
#### `public void Delete()`

Deletes an analysis node. The ID has to be set.

[Docs](https://developer.tekla.com/topic/en/18/47/7e14c3f5-c191-c336-7c80-370d3e1ee686)

#### `public AnalysisObjectEnumerator GetAnalysisNodeLinks()`

Get analysis nodelinks connected to node.

**Returns:** `AnalysisObjectEnumerator` — Enumerator of analysis nodelinks connected to node.

[Docs](https://developer.tekla.com/topic/en/18/47/9e08ff95-60a7-6ae3-4404-5661aff5d2e4)

#### `public AnalysisObjectEnumerator GetAnalysisParts()`

Get analysis parts connected to node.

**Returns:** `AnalysisObjectEnumerator` — Enumerator of analysis parts connected to node.

[Docs](https://developer.tekla.com/topic/en/18/47/ff69f4e7-472f-acac-5ee8-1841d45eb90d)

#### `public void Insert()`

Inserts an analysis node. The AnalysisModelName has to be set.

[Docs](https://developer.tekla.com/topic/en/18/47/3364bda8-1d4c-4308-3e31-744335f93688)

#### `public void Modify()`

Modifies an analysis node. The ID has to be set.

[Docs](https://developer.tekla.com/topic/en/18/47/c2c1f970-d078-3fc8-f139-3c8ef67bfd81)

#### `public override void Select()`

Selects an analysis node. The ID has to be set.

[Docs](https://developer.tekla.com/topic/en/18/47/4c6487ef-01ee-e6d1-4dd6-7aececbc8f5c)

### Properties
- `AnalysisModelName` (String, get/set) — Gets or sets the analysis model name.
- `AnalysisObjectType` (AnalysisObject.AnalysisObjectEnum, get/set) — Gets or sets the type of the analysis object.
- `CoordinateSystem` (CoordinateSystem, get/set) — The coordinate system of the analysis node.
- `FatherObject2ID` (Identifier, get/set) — Gets or sets the identifier of the father part.
- `FatherObject2Type` (AnalysisObject.AnalysisObjectEnum, get/set) — Gets or sets the type of the father object.
- `FatherObjectID` (Identifier, get/set) — Gets or sets the identifier of the father part.
- `FatherObjectType` (AnalysisObject.AnalysisObjectEnum, get/set) — Gets or sets the type of the father object.
- `ID` (Identifier, get/set) — Gets or sets the identifier of the analysis object.
- `IsUserNode` (Boolean, get/set) — Indicates whether the analysis node is a user node.
- `IsUserSupport` (Boolean, get/set) — Indicates whether the analysis node has a user node.
- `Label` (AnalysisLabel, get/set) — The label of the analysis node.
- `Position` (Vector, get/set) — The position of the analysis node (in the global coordinate system).
- `Support` (AnalysisSupport, get/set) — The support of the analysis node.

## AnalysisNodeLink (class)

The AnalysisNodeLink class contains information related to analysis node links.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/d633193f-9412-06da-c274-9b02cac9186f)

### Constructors
- `public AnalysisNodeLink()` — Initializes a new instance of the AnalysisNodeLink class.

### Methods
#### `public void Delete()`

Deletes a user analysis node link. The ID has to be set.

[Docs](https://developer.tekla.com/topic/en/18/47/fad6764e-4f78-b6e3-335a-11d701e5ac43)

#### `public void Insert()`

Inserts a user analysis node link. The AnalysisModelName has to be set. The StartNodeID has to be set. The EndNodeID has to be set.

[Docs](https://developer.tekla.com/topic/en/18/47/a6f7596c-cf99-b230-4184-fdd8692c8522)

#### `public void Modify()`

Modifies an analysis node link. The ID has to be set.

[Docs](https://developer.tekla.com/topic/en/18/47/6ea9ed3b-10e2-94bf-43ee-4d899bdb8371)

#### `public override void Select()`

Selects an analysis node link. The ID has to be set.

[Docs](https://developer.tekla.com/topic/en/18/47/6ff1a838-95ce-325d-1630-0783a93ecd60)

### Properties
- `AnalysisModelName` (String, get/set) — Gets or sets the analysis model name.
- `AnalysisObjectType` (AnalysisObject.AnalysisObjectEnum, get/set) — Gets or sets the type of the analysis object.
- `CoordinateSystem` (CoordinateSystem, get/set) — The coordinate system of the analysis node link.
- `Disabled` (Boolean, get/set) — Indicates whether the analysis node link is disabled.
- `EndConnectivity` (AnalysisConnectivity, get/set) — The end connectivity of the analysis node link.
- `EndNode` (AnalysisNode, get/set) — The end node.
- `FatherObject2ID` (Identifier, get/set) — Gets or sets the identifier of the father part.
- `FatherObject2Type` (AnalysisObject.AnalysisObjectEnum, get/set) — Gets or sets the type of the father object.
- `FatherObjectID` (Identifier, get/set) — Gets or sets the identifier of the father part.
- `FatherObjectType` (AnalysisObject.AnalysisObjectEnum, get/set) — Gets or sets the type of the father object.
- `ID` (Identifier, get/set) — Gets or sets the identifier of the analysis object.
- `IsUserConnectivity` (Boolean, get/set) — Indicates whether the analysis node has a user defined connectivity.
- `IsUserNodeLink` (Boolean, get/set) — Indicates whether the analysis node is a user node link.
- `OwnerPartIDs` (List<Identifier>, get/set) — The list of the identifiers of the owner parts.
- `StartConnectivity` (AnalysisConnectivity, get/set) — The start connectivity of the analysis node link.
- `StartNode` (AnalysisNode, get/set) — The start node.

## AnalysisObject (class)

The AnalysisObject class is an abstract base class for all analysis objects.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/1a5c5655-7804-dc7a-64d6-c356c382be27)

### Constructors
- `public AnalysisObject()` — Initializes a new instance of the AnalysisObject class.

### Methods
#### `public abstract void Select()`

Selects an analysis object from the database.

[Docs](https://developer.tekla.com/topic/en/18/47/838ad5fe-ed14-8f58-d432-efd635e6c7ea)

### Properties
- `AnalysisModelName` (String, get/set) — Gets or sets the analysis model name.
- `AnalysisObjectType` (AnalysisObject.AnalysisObjectEnum, get/set) — Gets or sets the type of the analysis object.
- `FatherObject2ID` (Identifier, get/set) — Gets or sets the identifier of the father part.
- `FatherObject2Type` (AnalysisObject.AnalysisObjectEnum, get/set) — Gets or sets the type of the father object.
- `FatherObjectID` (Identifier, get/set) — Gets or sets the identifier of the father part.
- `FatherObjectType` (AnalysisObject.AnalysisObjectEnum, get/set) — Gets or sets the type of the father object.
- `ID` (Identifier, get/set) — Gets or sets the identifier of the analysis object.

## AnalysisObject.AnalysisObjectEnum (enum)

All subclasses of analysis objects are defined here. This enumeration can be used to select certain types of analysis objects.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/e1b54325-7dfc-88e4-92bb-830979fdb059)

### Values
- `ANALYSIS_NOT_DEFINED` = `0` — The analysis object type is not defined.
- `ANALYSIS_MODEL` = `1` — This type is for selecting analysis models from the current Tekla Structures model.
- `ANALYSIS_PART` = `2` — This type is for selecting analysis parts from the current analysis model.
- `ANALYSIS_BAR` = `3` — This type is for selecting analysis bars from the current analysis model.
- `ANALYSIS_AREA` = `4` — This type is for selecting analysis areas from the current analysis model.
- `ANALYSIS_VOLUME` = `5` — This type is for selecting analysis volumes from the current analysis model.
- `ANALYSIS_MEMBER` = `6` — This type is for selecting analysis members from the current analysis model.
- `ANALYSIS_DECOMPOSED_LOAD_NODE` = `7` — This type is for selecting decomposed node loads from the current analysis model.
- `ANALYSIS_DECOMPOSED_LOAD_BAR` = `8` — This type is for selecting decomposed bar loads from the current analysis model.
- `ANALYSIS_DECOMPOSED_LOAD_AREA` = `9` — This type is for selecting decomposed area loads from the current analysis model.
- `ANALYSIS_DECOMPOSED_LOAD_MEMBER` = `10` — This type is for selecting decomposed member loads from the current analysis model.
- `ANALYSIS_DECOMPOSED_LOAD_SELFWEIGHT` = `11` — This type is for selecting decomposed selfweight loads from the current analysis model.
- `ANALYSIS_DECOMPOSED_LOAD_TEMPERATURE` = `12` — This type is for selecting decomposed temperature loads from the current analysis model.
- `ANALYSIS_AREA_POLYGON` = `13` — This type is for selecting area polygons from the current analysis model.
- `ANALYSIS_AREA_HOLE` = `14` — This type is for selecting area holes from the current analysis model.
- `ANALYSIS_EDGE` = `15` — This type is for selecting area edges from the current analysis model.
- `ANALYSIS_EDGE_HOLE` = `16` — This type is for selecting area hole edges from the current analysis model.
- `ANALYSIS_POSITION_BAR` = `17` — This type is for selecting bar positions from the current analysis model.
- `ANALYSIS_POSITION_AREA` = `18` — This type is for selecting area polygon positions from the current analysis model.
- `ANALYSIS_POSITION_AREA_INNER` = `19` — This type is for selecting area inner positions from the current analysis model.
- `ANALYSIS_POSITION_AREA_HOLE` = `20` — This type is for selecting area hole positions from the current analysis model.
- `ANALYSIS_POSITION_EDGE_SPLIT` = `21` — This type is for selecting edge split positions from the current analysis model.
- `ANALYSIS_POSITION_EDGE_SPLIT_HOLE` = `22` — This type is for selecting edge split hole positions from the current analysis model.
- `ANALYSIS_POSITION_FACE` = `23` — This type is for selecting face positions from the current analysis model.
- `ANALYSIS_NODE` = `24` — This type is for selecting analysis nodes from the current analysis model.
- `ANALYSIS_NODE_LINK` = `25` — This type is for selecting analysis node links from the current analysis model.
- `ANALYSIS_DESIGN_PARAMETER` = `26` — This type is for selecting analysis design parameters from the current analysis model.
- `ANALYSIS_DESIGN_PARAMETER_MULTI` = `27` — This type is for selecting analysis design multi parameters from the current analysis model.
- `ANALYSIS_MODEL_CONNECTIVITY_RULE` = `28` — This type is for selecting analysis connectivity rules from the current analysis model.
- `ANALYSIS_FACE` = `29` — This type is for selecting analysis faces from the current analysis model.
- `ANALYSIS_VOLUME_ITEM` = `30` — This type is for selecting analysis volume items from the current analysis model.
- `ANALYSIS_CROSS_SECTION` = `31` — This type is for selecting analysis cross sections from the current analysis model.
- `ANALYSIS_SUB_SECTION` = `32` — This type is for selecting analysis subsections from the current analysis model.
- `ANALYSIS_MATERIAL` = `33` — This type is for selecting analysis materials from the current analysis model.
- `ANALYSIS_CROSS_SECTION_GROUP` = `34` — This type is for selecting analysis cross section groups from the current analysis model.
- `ANALYSIS_RIGID_DIAPHRAGM` = `35` — This type is for selecting analysis rigid diaphragms from the current analysis model.
- `ANALYSIS_LOAD_GROUP` = `36` — This type is for selecting analysis load groups from the current analysis model.
- `ANALYSIS_LOAD_COMBINATION` = `37` — This type is for selecting analysis load combinations from the current analysis model.
- `ANALYSIS_MODEL_LOAD` = `38` — This type is for selecting model loads from the current analysis model.

## AnalysisObjectEnumerator (class)

The AnalysisObjectEnumerator class provides the means to iterate through analysis object instances.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/aef284e6-9106-6341-067c-fd6fa330dcc1)

### Methods
#### `public IEnumerator GetEnumerator()`

Returns a pointer to the existing instance of the class. This allows the usage of the foreach statement with AnalysisObjectEnumerator.

**Returns:** `IEnumerator` — The pointer to the existing instance of the class.

[Docs](https://developer.tekla.com/topic/en/18/47/a339d39c-900f-230d-fab8-3ba7e3ec4ee9)

#### `public int GetSize()`

Returns the total amout of items.

**Returns:** `Int32` — The total amount of items.

[Docs](https://developer.tekla.com/topic/en/18/47/e76342ea-c2b1-59eb-96c5-86d4471533bc)

#### `public bool MoveNext()`

Moves to the next item in the enumerator.

**Returns:** `Boolean` — True if succeeded to move to the next item.

[Docs](https://developer.tekla.com/topic/en/18/47/c924d65b-4a72-8d8b-2ff9-072c75e0684d)

#### `public void Refresh()`

Refreshes the enumerator with latest data at database.

[Docs](https://developer.tekla.com/topic/en/18/47/f6837180-aefb-6c1f-a2ef-a2a97595aadb)

#### `public void Reset()`

Resets the enumerator to the beginning.

[Docs](https://developer.tekla.com/topic/en/18/47/31822a54-63fc-8431-ebe3-1201c2dfafc6)

### Properties
- `Current` (AnalysisObject, get) — Gets the current analysis object in the enumerator.
- `CurrentItem` (Int32, get/set) — Gets or sets the index of the current item.
- `CurrentSelected` (AnalysisObject, get/set) — Gets or sets the currently selected item.
- `SelectInstances` (Boolean, get/set) — Gets or sets a value indicating whether the instance's Select() is called when the "Current" item is asked from the enumerator. The user can set this to "false" if no members are ever asked from the instance. This is the case when e.g. asking only a report property from this identifier. WARNING: normally the user should not change this value.

## AnalysisObjectEnumerator.AnalysisEnumeratorTypeEnum (enum)

The available enumerator types.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/c0802039-8a74-6b35-75d9-e387925ec948)

### Values
- `ALL_SELECTED` = `1` — This selects all the user selected objects.
- `ALL_OBJECTS` = `2` — This selects all the objects.
- `ALL_OBJECTS_WITH_TYPE` = `3` — This selects all the objects with the given type as the "subtype".
- `SUB_ANALYSIS_PARTS_OF_ANALYSIS_PART` = `4` — Sub analysis parts of analysis part.
- `MAIN_ANALYSIS_PARTS_OF_ANALYSIS_PART` = `5` — Main analysis parts of analysis part.
- `ANALYSIS_PARTS_BY_NODE` = `6` — Analysis parts connected to node.
- `ANALYSIS_NODELINKS_BY_NODE` = `7` — Analysis node links connected to node.

## AnalysisObjectSelector (class)

The AnalysisObjectSelector class contains methods for getting objects.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/302cf92f-a39e-a869-a38b-1b6683f77dac)

### Methods
#### `public AnalysisObjectEnumerator GetAllObjects(string AnalysisModelName)`

Returns an enumerator of all the model objects in the current model. NOT YET IMPLEMENTED.

**Parameters:**
- `AnalysisModelName` (System.String) — The name of the analysis model.

**Returns:** `AnalysisObjectEnumerator` — A model object enumerator of all the model objects.

[Docs](https://developer.tekla.com/topic/en/18/47/2d75a66e-dc18-0739-17a1-342d7ff82771)

#### `public AnalysisObjectEnumerator GetAllObjectsWithType(AnalysisObject.AnalysisObjectEnum ObjectToSelectType, string AnalysisModelName)`

Returns an enumerator of all the model objects in the current model with the given type.

**Parameters:**
- `ObjectToSelectType` (Tekla.Structures.Analysis.AnalysisObject.AnalysisObjectEnum) — The type of the objects to return.
- `AnalysisModelName` (System.String) — The name of the analysis model.

**Returns:** `AnalysisObjectEnumerator` — A model object enumerator of all the model objects with the given type.

[Docs](https://developer.tekla.com/topic/en/18/47/c9afcf88-f1ec-9cf1-0280-66a70e47e66a)

#### `public AnalysisObjectEnumerator GetAnalysisModels()`

Returns an enumerator of all the analysis model objects.

**Returns:** `AnalysisObjectEnumerator` — A model object enumerator of all the analysis model objects.

[Docs](https://developer.tekla.com/topic/en/18/47/327fdf82-56ca-888e-a9ff-bf12bc757028)

#### `public AnalysisObjectEnumerator GetSelectedObjects()`

Returns an enumerator of all the selected model objects in the current model. NOT YET IMPLEMENTED.

**Returns:** `AnalysisObjectEnumerator` — A model object enumerator of all the selected model objects.

[Docs](https://developer.tekla.com/topic/en/18/47/82197e0a-7acf-41b1-05d7-b1c4c95f8e3d)

## AnalysisPart (class)

The AnalysisPart class contains information related to analysis parts.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/79fa39f8-7f9f-4f98-8ae6-20c6efc31c1f)

### Constructors
- `public AnalysisPart()` — Initializes a new instance of the AnalysisPart class.

### Methods
#### `public void Add(AnalysisArea AnalysisArea)`

Adds an analysis area to the analysis part.

**Parameters:**
- `AnalysisArea` (Tekla.Structures.Analysis.AnalysisArea) — The analysis area to be added.

[Docs](https://developer.tekla.com/topic/en/18/47/593e179d-1ef5-f69c-cc04-aa77832c2a64)

#### `public void Add(AnalysisBar AnalysisBar)`

Adds an analysis bar to the analysis part.

**Parameters:**
- `AnalysisBar` (Tekla.Structures.Analysis.AnalysisBar) — The analysis bar to be added.

[Docs](https://developer.tekla.com/topic/en/18/47/8ee05e2c-7864-0c43-7b1d-c0b84e753f03)

#### `public void Delete()`

Deletes an analysis part. The method is not implemented.

[Docs](https://developer.tekla.com/topic/en/18/47/913d5fc8-12a5-3842-401e-1fa9c3b457f9)

#### `public bool DeleteAnalysisPart()`

Deletes the analysis part by ID.

**Returns:** `Boolean` — True if successful.

[Docs](https://developer.tekla.com/topic/en/18/47/4f473d30-9ec6-d949-746c-0c8ef34b245d)

#### `public AnalysisObjectEnumerator GetAnalysisAreas()`

Gets the analysis areas of the analysis part.

**Returns:** `AnalysisObjectEnumerator` — The analysis areas of the analysis part.

[Docs](https://developer.tekla.com/topic/en/18/47/ab93e725-314c-4b9a-f203-056d6d64702b)

#### `public AnalysisObjectEnumerator GetAnalysisBars()`

Gets the analysis bars of the analysis part.

**Returns:** `AnalysisObjectEnumerator` — The analysis bars of the analysis part.

[Docs](https://developer.tekla.com/topic/en/18/47/0da734f3-344d-69b2-6b60-44d0982272ec)

#### `public AnalysisObjectEnumerator GetAnalysisVolumes()`

Gets the analysis volumes of the analysis part.

**Returns:** `AnalysisObjectEnumerator` — The analysis volumes of the analysis part.

[Docs](https://developer.tekla.com/topic/en/18/47/ea25e3ca-8245-726d-53e8-3a727fb3c931)

#### `public AnalysisObjectEnumerator GetMainParts()`

Get the main analysis parts of this analysis part. There are no main analysis parts if this is not a sub analysis part.

**Returns:** `AnalysisObjectEnumerator` — The the main parts of this analysis part.

[Docs](https://developer.tekla.com/topic/en/18/47/497f3914-70d0-aad9-e950-effe84eaa9d1)

#### `public AnalysisObjectEnumerator GetSubParts()`

Get the sub analysis parts of this analysis part. There are no sub analysis parts if this is not a main analysis part.

**Returns:** `AnalysisObjectEnumerator` — The sub analysis parts of this analysis part.

[Docs](https://developer.tekla.com/topic/en/18/47/470af607-1d81-d92c-5897-a316ec333ef5)

#### `public void Insert()`

Inserts a new analysis part into the Tekla Structures model. The AnalysisModelName has to be set.

[Docs](https://developer.tekla.com/topic/en/18/47/22f458e3-89e3-9026-c42e-d9056e5a4f43)

#### `public bool IsMainPart()`

Check if this analysis part is a main analysis part (has sub analysis parts).

**Returns:** `Boolean` — True if this analysis part is a main analysis part.

[Docs](https://developer.tekla.com/topic/en/18/47/4bf5cd08-99b9-bbae-bce7-6bc34d71a0c9)

#### `public bool IsSubPart()`

Check if this analysis part is a sub analysis part.

**Returns:** `Boolean` — True if this analysis part is a sub part.

[Docs](https://developer.tekla.com/topic/en/18/47/779dd4a8-54b9-a77a-d4fc-dfeed6e88a84)

#### `public void Modify()`

Modifies an analysis part. The ID, or the PartID and the AnalysisModelName, has to be set.

[Docs](https://developer.tekla.com/topic/en/18/47/f5f4208a-209f-80c4-7d27-95ce16fe0c83)

#### `public bool MovePosition(AnalysisPosition position, Vector move)`

Move analysis position.

**Parameters:**
- `position` (Tekla.Structures.Analysis.AnalysisPosition) — The analysis position.
- `move` (Tekla.Structures.Geometry3d.Vector) — The move.

**Returns:** `Boolean` — True if successful.

[Docs](https://developer.tekla.com/topic/en/18/47/82b2187a-548d-dc58-4dc3-0a4654c17480)

#### `public override void Select()`

Selects an analysis part. The ID, or the PartID and the AnalysisModelName, has to be set.

[Docs](https://developer.tekla.com/topic/en/18/47/0f61b68a-9046-e767-7a57-a7e0b3283537)

### Properties
- `AnalysisAreas` (List<AnalysisArea>, get) — The analysis areas of the analysis part.
- `AnalysisBars` (List<AnalysisBar>, get) — The analysis bars of the analysis part.
- `AnalysisDesignCode` (AnalysisDesignCode, get/set) — The analysis design code of the analysis part.
- `AnalysisModelName` (String, get/set) — Gets or sets the analysis model name.
- `AnalysisObjectType` (AnalysisObject.AnalysisObjectEnum, get/set) — Gets or sets the type of the analysis object.
- `AnalysisPartAnalysisProperties` (AnalysisPartAnalysisProperties, get/set) — The analysis part analysis properties.
- `AnalysisPartAreaAttributes` (AnalysisPartAreaAttributes, get/set) — The analysis part area attributes.
- `AnalysisPartBarAttributes` (AnalysisPartBarAttributes, get/set) — The analysis part bar attributes.
- `AnalysisPartCompositeProperties` (AnalysisPartCompositeProperties, get/set) — The analysis part composite properties.
- `AnalysisPartLoadingProperties` (AnalysisPartLoadingProperties, get/set) — The analysis part loading properties.
- `AnalysisPartPositionProperties` (AnalysisPartPositionProperties, get/set) — The analysis part position properties.
- `AnalysisPartSpanningProperties` (AnalysisPartSpanningProperties, get/set) — The analysis part spanning properties.
- `AnalysisType` (AnalysisPart.AnalysisTypeEnum, get/set) — The analysis type of the analysis part.
- `AnalysisVolumes` (List<AnalysisVolume>, get) — The analysis volumes of the analysis part.
- `CoordinateSystem` (CoordinateSystem, get/set) — The coordinate system of the analysis part.
- `Curvature` (AnalysisCurvature, get/set) — The curvature of the analysis part.
- `FatherObject2ID` (Identifier, get/set) — Gets or sets the identifier of the father part.
- `FatherObject2Type` (AnalysisObject.AnalysisObjectEnum, get/set) — Gets or sets the type of the father object.
- `FatherObjectID` (Identifier, get/set) — Gets or sets the identifier of the father part.
- `FatherObjectType` (AnalysisObject.AnalysisObjectEnum, get/set) — Gets or sets the type of the father object.
- `ID` (Identifier, get/set) — Gets or sets the identifier of the analysis object.
- `LoadBearingOrder` (Int32, get/set) — The load bearing order.
- `Material` (AnalysisMaterial, get/set) — The material of the analysis part.
- `OverrideProfile` (AnalysisCrossSection, get/set) — The override profile of the analysis part. In case of tapered profile, only the name is available, and more details are available in the start/end properties of the analysis members.
- `PartID` (Identifier, get/set) — The identifier of the physical part.
- `PartName` (String, get/set) — The name of the physical part.
- `PartType` (AnalysisPart.PartTypeEnum, get/set) — The type of the physical part.
- `Profile` (AnalysisCrossSection, get/set) — The profile of the analysis part. Note: OverrideProfile may exist to replace this. In case of tapered profile, only the name is available, ID is 0 and values are not returned. More details are available in the start/end properties of the analysis members.
- `SnapDistance` (Double, get/set) — The snap distance. Use 0.0 for manual connectivity, positive value for automatic connectivity.
- `UpDirection` (Vector, get/set) — The up direction of the analysis part (in the global coordinate system). The axis vector (connecting member end nodes) and the up direction vector define the local XY plane of the member. The local Z can be calculated as localZ = localX x UpDirection (cross product), and after this localY = localZ x localX (cross product).

## AnalysisPart.AnalysisTypeEnum (enum)

The analysis types.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/5bbef527-d904-3e64-2c56-9d7a1c007fe7)

### Values
- `ANALYSIS_TYPE_IGNORE` = `1` — The ignore analysis type.
- `ANALYSIS_TYPE_TRUSS` = `2` — The truss analysis type.
- `ANALYSIS_TYPE_RIGID_DIAPHRAGM` = `3` — The rigid diaphragm analysis type.
- `ANALYSIS_TYPE_TENSION_ONLY` = `4` — The tension only analysis type.
- `ANALYSIS_TYPE_COMPRESSION_ONLY` = `5` — The compression only analysis type.
- `ANALYSIS_TYPE_SHELL` = `6` — The shell analysis type.
- `ANALYSIS_TYPE_SHEAR_WALL` = `7` — The wall analysis type.
- `ANALYSIS_TYPE_BEAM` = `8` — The beam analysis type.
- `ANALYSIS_TYPE_MEMBRANE` = `9` — The membrane analysis type.
- `ANALYSIS_TYPE_PLATE` = `10` — The plate analysis type.
- `ANALYSIS_TYPE_MAT_FOUNDATION` = `11` — The mat foundation analysis type.

## AnalysisPart.ItemTypeEnum (enum)

The item types.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/c1ab8b10-0725-0315-e221-8ccdb3e779ad)

### Values
- `ITEM_TYPE_UNKNOWN` = `0` — The item type is unknown.
- `ITEM_TYPE_BAR` = `1` — The bar item type.
- `ITEM_TYPE_AREA` = `2` — The area item type.
- `ITEM_TYPE_VOLUME` = `3` — The volume item type.

## AnalysisPart.PartTypeEnum (enum)

The types of the physical model part.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/131441ba-c644-82f7-e390-379049b7d307)

### Values
- `PART` = `0` — The part is a normal part.
- `BOOLEAN_ADD_PART` = `1` — The part is a boolean add part.

## AnalysisPartAnalysisProperties (class)

The AnalysisPartAnalysisProperties class contains information related to analysis part analysis properties.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/4f9a542f-63fa-730d-3e5b-7b292c6aa99f)

### Constructors
- `public AnalysisPartAnalysisProperties()` — Initializes a new instance of the AnalysisPartAnalysisProperties class

### Properties
- `AnalysisClass` (AnalysisPartAnalysisProperties.AnalysisClassEnum, get/set) — The analysis class of the analysis part.
- `BuiltupMode` (AnalysisPartAnalysisProperties.BuiltupEnum, get/set) — The builtup mode of the analysis part.
- `DesignGroup` (String, get/set) — The design group of the analysis part.
- `PhysicalFreezed` (Boolean, get) — Indicates whether the physical part has been freezed.

## AnalysisPartAnalysisProperties.AnalysisClassEnum (enum)

The analysis classes.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/4cbde6d4-92ff-295c-6aba-7c2a92af06ac)

### Values
- `ANALYSIS_CLASS_COLUMN` = `1` — The column analysis class.
- `ANALYSIS_CLASS_BEAM` = `2` — The beam analysis class.
- `ANALYSIS_CLASS_WALL` = `3` — The wall analysis class.
- `ANALYSIS_CLASS_SLAB` = `4` — The slab analysis class.
- `ANALYSIS_CLASS_CONTOUR_PLATE` = `5` — The contour plate analysis class.
- `ANALYSIS_CLASS_SECONDARY` = `99` — The old secondary object analysis class. Named now as bracing.
- `ANALYSIS_CLASS_BRACING` = `99` — The bracing object analysis class.
- `ANALYSIS_CLASS_SECONDARY_MEMBER` = `100` — The secondary object analysis class.

## AnalysisPartAnalysisProperties.BuiltupEnum (enum)

The builtup settings.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/1cb79936-3ab2-3121-88b4-0bac9e9c6e22)

### Values
- `BUILTUP_AUTO` = `9` — The "auto" builtup setting.
- `BUILTUP_NONE` = `0` — The "none" builtup setting.
- `BUILTUP_MAIN_PART` = `1` — The "main part" builtup setting.
- `BUILTUP_SUB_PART` = `2` — The "sub part" builtup setting.
- `BUILTUP_SUB_PART_BEAM` = `3` — The "part beam" builtup setting.
- `BUILTUP_SUB_PART_COLUMN` = `4` — The "sub part column" builtup setting.

## AnalysisPartAreaAttributes (class)

The AnalysisPartAreaAttributes class contains information related to analysis part area attributes.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/413b0561-05af-f353-df9e-be7a291c50af)

### Constructors
- `public AnalysisPartAreaAttributes()` — Initializes a new instance of the AnalysisPartAreaAttributes class

### Properties
- `AreaElementSizeHoles` (Double, get/set) — Gets or sets the area element size holes.
- `AreaElementSizeX` (Double, get/set) — Gets or sets the area element size X.
- `AreaElementSizeY` (Double, get/set) — Gets or sets the area element size Y.
- `AreaElementType` (AnalysisPartAreaAttributes.AreaElementTypeEnum, get/set) — Gets or sets the area element type.
- `AreaRotationXY` (Double, get/set) — Gets or sets the area rotation XY.
- `AreaStartNumber` (Int32, get/set) — Gets or sets the start number of the area.
- `AreaSupport` (AnalysisPartAreaAttributes.AreaSupportEnum, get/set) — Gets or sets the area support.
- `AreaSupportType` (AnalysisPartAreaAttributes.AreaSupportTypeEnum, get/set) — Gets or sets the area support type.
- `MinimumAreaHoleSize` (Double, get/set) — Gets or sets the minimum area hole size.
- `RigidDiaphragmFilterName` (String, get/set) — Gets or sets the filter name of the rigid diaphragm.
- `UseSimpleArea` (Boolean, get/set) — Gets or sets a value indicating whether simple areas are used.

## AnalysisPartAreaAttributes.AreaElementTypeEnum (enum)

The area element types.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/1b857ade-0ac9-4dbc-20a0-97b26afcc5f8)

### Values
- `AREA_ELEMENT_TYPE_TRIANGULAR` = `0` — The triangular area element type.
- `AREA_ELEMENT_TYPE_QUADRILATERAL` = `1` — The quadrilateral area element type.

## AnalysisPartAreaAttributes.AreaSupportEnum (enum)

The area support options.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/d61599db-50f4-3fdb-43a3-690dc7dcde2e)

### Values
- `AREA_SUPPORT_NONE` = `0` — The "none" area support option.
- `AREA_SUPPORT_WALL_BOTTOM_PINNED` = `1` — The area support simple.
- `AREA_SUPPORT_WALL_BOTTOM_FIXED` = `2` — The wall bottom fixed area support.
- `AREA_SUPPORT_ALL_PINNED` = `3` — The all pinned area support.
- `AREA_SUPPORT_ALL_FIXED` = `4` — The all fixed area support.
- `AREA_SUPPORT_OUTLINE_PINNED` = `5` — The outline pinned area support.
- `AREA_SUPPORT_OUTLINE_FIXED` = `6` — The outline fixed area support.

## AnalysisPartAreaAttributes.AreaSupportTypeEnum (enum)

The new area support options.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/84d42166-d983-6890-310c-01acac51ca9a)

### Values
- `AREA_SUPPORT_NO` = `0` — The "no" area support option.
- `AREA_SUPPORT_SIMPLE` = `1` — The area support simple.
- `AREA_SUPPORT_FULL` = `2` — The area support full.

## AnalysisPartBarAttributes (class)

The AnalysisPartBarAttributes class contains information related to analysis part bar attributes.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/f62c3da6-9858-b482-c809-35c91b0b59ed)

### Constructors
- `public AnalysisPartBarAttributes()` — Initializes a new instance of the AnalysisPartBarAttributes class

### Properties
- `BarEndCondition` (AnalysisPartEnd, get/set) — The bar end condition.
- `BarStartCondition` (AnalysisPartEnd, get/set) — The bar start condition.
- `BarStartNumber` (Int32, get/set) — The start number of the bar.
- `ForceStraightSegments` (AnalysisPartBarAttributes.ForceStraightSegmentsEnum, get/set) — The straight force segments.
- `MemberStartNumber` (Int32, get/set) — The start number of the member.
- `OverrideProfileName` (String, get/set) — The override profile name.
- `SplitDistances` (List<Double>, get/set) — The split distances.
- `SplitNodeCount` (Int32, get/set) — The count of the split node.

## AnalysisPartBarAttributes.ForceStraightSegmentsEnum (enum)

The straight force segments.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/e7208b24-34c6-77b4-601a-52869683712d)

### Values
- `FORCE_STRAIGHT_SEGMENTS_NOT_SET` = `9` — The straight force segments setting "not set".
- `FORCE_STRAIGHT_SEGMENTS_YES` = `0` — The straight force segments setting "yes".
- `FORCE_STRAIGHT_SEGMENTS_NO` = `1` — The straight force segments setting "no".

## AnalysisPartCompositeProperties (class)

The AnalysisPartCompositeProperties class contains information related to analysis part composite properties.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/fca7e2a4-a548-78fa-dc81-d2d64f7c25db)

### Constructors
- `public AnalysisPartCompositeProperties()` — Initializes a new instance of the AnalysisPartCompositeProperties class

### Properties
- `CompositeBeamSlabMaterial` (String, get/set) — The composite beam slab material.
- `CompositeBeamSlabThickness` (Double, get/set) — The composite beam slab thickness.
- `CompositeBeamType` (AnalysisPartCompositeProperties.CompositeBeamEnum, get/set) — The composite beam type.
- `CompositeBeamWidthFromSpanFactor` (Double, get/set) — The composite beam width from span factor.
- `CompositeBeamWidthLeft` (Double, get/set) — The composite beam width left.
- `CompositeBeamWidthModeLeft` (AnalysisPartCompositeProperties.CompositeBeamWidthEnum, get/set) — The composite beam width mode left.
- `CompositeBeamWidthModeRight` (AnalysisPartCompositeProperties.CompositeBeamWidthEnum, get/set) — The composite beam width mode right.
- `CompositeBeamWidthRight` (Double, get/set) — The composite beam width right.

## AnalysisPartCompositeProperties.CompositeBeamEnum (enum)

The composite beam options.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/135caca2-4966-9151-b833-8609654aafc7)

### Values
- `COMPOSITE_BEAM_NOT` = `0` — The composite beam setting "not".
- `COMPOSITE_BEAM_YES` = `1` — The composite beam setting "yes".
- `COMPOSITE_BEAM_AUTOMATIC` = `2` — The composite beam setting "automatic".

## AnalysisPartCompositeProperties.CompositeBeamWidthEnum (enum)

The composite beam width options.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/2f09c75a-32ca-7d83-409f-ae6c575e1e94)

### Values
- `COMPOSITE_BEAM_WIDTH_MANUAL` = `0` — The composite beam width is set manually.
- `COMPOSITE_BEAM_WIDTH_AUTOMATIC` = `1` — The composite beam width is set automatically.

## AnalysisPartEnd (class)

The AnalysisPartEnd class contains information related to analysis part ends.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/e9e24f96-5487-123d-4b75-1bf34e51f390)

### Constructors
- `public AnalysisPartEnd()` — Initializes a new instance of the AnalysisPartEnd class

### Properties
- `Condition` (AnalysisPartEnd.ConditionEnum, get/set) — The condition of the part end.
- `Connectivity` (AnalysisConnectivity, get/set) — The connectivity of the part end.
- `Eccentricity` (Vector, get/set) — The eccentricity of the part end (in the local coordinate system).
- `EccentricityMode` (AnalysisPartEnd.EccentricityModeEnum, get/set) — The eccentricity mode of the part end.
- `Support` (AnalysisSupport, get/set) — The support of the part end.
- `SupportCoordSystem` (CoordinateSystem, get/set) — The coordinate system of the support.

## AnalysisPartEnd.ConditionEnum (enum)

The conditions of the part end.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/07a0eda9-b467-7e54-c958-12e48edf7e92)

### Values
- `CONDITION_CONNECTED` = `0` — The connected condition.
- `CONDITION_SUPPORTED` = `1` — The supported condition.

## AnalysisPartEnd.EccentricityModeEnum (enum)

The eccentricity modes of the part end.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/feb2738e-06b6-ceed-ebf5-1bb155e41e74)

### Values
- `ECCENTRICITY_MODE_MANUAL` = `0` — The manual eccentricity mode.
- `ECCENTRICITY_MODE_AUTOMATIC` = `1` — The automatic eccentricity mode.

## AnalysisPartLoadingProperties (class)

The AnalysisPartLoadingProperties class contains information related to analysis part loading properties.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/4f3a0d61-c97c-03ad-35a5-2209874b0ff9)

### Constructors
- `public AnalysisPartLoadingProperties()` — Initializes a new instance of the AnalysisPartLoadingProperties class

### Properties
- `LoadGenerationAutoCreateFixedSupports` (Boolean, get/set) — Indicates whether fixed supports are automatically created.
- `LoadGenerationGroupIdentifierFactors` (Dictionary<Int32,Double>, get/set) — The load generation group identifier factors.
- `LoadGenerationPartNames` (String, get/set) — The load generation part names.
- `LoadGenerationPartNamesInclude` (Boolean, get/set) — Indicates whether part names are included in the load generation.
- `LoadGenerationSelfWeight` (Boolean, get/set) — Indicates whether the selfweight load is generated.

## AnalysisPartPositionProperties (class)

The AnalysisPartPositionProperties class contains information related to analysis part position properties.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/078f86fd-4ad5-70dd-5624-9249b39ab2e6)

### Constructors
- `public AnalysisPartPositionProperties()` — Initializes a new instance of the AnalysisPartPositionProperties class

### Properties
- `AxisCoordinateX` (Double, get/set) — The axis coordinate in the X direction.
- `AxisCoordinateY` (Double, get/set) — The axis coordinate in the Y direction.
- `AxisCoordinateZ` (Double, get/set) — The axis coordinate in the Z direction.
- `AxisLocation` (AnalysisPartPositionProperties.AxisLocationEnum, get/set) — The axis location of the analysis part.
- `AxisModifierX` (AnalysisPartPositionProperties.AxisModifierEnum, get/set) — The axis modifier in the X direction.
- `AxisModifierY` (AnalysisPartPositionProperties.AxisModifierEnum, get/set) — The axis modifier in the Y direction.
- `AxisModifierZ` (AnalysisPartPositionProperties.AxisModifierEnum, get/set) — The axis modifier in the Z direction.
- `AxisOffset` (Vector, get/set) — The axis offset of the analysis part (in the global coordinate system).
- `KeepAxis` (AnalysisModel.KeepAxisEnum, get/set) — The keep axis setting of the analysis part.
- `LongitudinalOffsetMode` (AnalysisPartPositionProperties.LongitudinalOffsetEnum, get/set) — The longitudinal offset mode.

## AnalysisPartPositionProperties.AxisLocationEnum (enum)

The axis location.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/1abe87fa-f529-9083-4045-afa77b219693)

### Values
- `REFERENCE_AXIS_WITH_ECCENTRICITY` = `0` — The axis location is the reference axis with eccentricity.
- `REFERENCE_AXIS` = `1` — The axis location is the reference axis.
- `NEUTRAL_AXIS` = `3` — The axis location is the neutral axis.
- `TOP_PLANE` = `4` — The axis location is the top plane.
- `MIDDLE_PLANE` = `5` — The axis location is the middle plane.
- `BOTTOM_PLANE` = `6` — The axis location is the bottom plane.
- `LEFT_PLANE` = `7` — The axis location is the left plane.
- `RIGHT_PLANE` = `8` — The axis location is the right plane.
- `MIDDLE_PLANE_BY_LEFT_RIGHT` = `9` — The axis location is the middle plane by the left point.
- `TOP_LEFT` = `10` — The axis location is at the top left corner.
- `TOP_CENTER` = `11` — The axis location is at top center.
- `TOP_RIGHT` = `12` — The axis location is at the top right corner.
- `MIDDLE_LEFT` = `13` — The axis location is at the middle left corner.
- `MIDDLE_CENTER` = `14` — The axis location is at middle center.
- `MIDDLE_RIGHT` = `15` — The axis location is at the middle right corner.
- `BOTTOM_LEFT` = `16` — The axis location is at the bottom left corner.
- `BOTTOM_CENTER` = `17` — The axis location is at bottom center.
- `BOTTOM_RIGHT` = `18` — The axis location is at the bottom right corner.

## AnalysisPartPositionProperties.AxisModifierEnum (enum)

The axis modifiers.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/43932d24-9b30-d746-131f-0c19730c88ad)

### Values
- `AXIS_MODIFIER_NONE` = `0` — The axis modifier is "none".
- `AXIS_MODIFIER_FIXED_COORDINATE` = `1` — The axis modifier is "fixed coordinate".
- `AXIS_MODIFIER_NEAREST_GRID` = `2` — The axis modifier is "nearest grid".

## AnalysisPartPositionProperties.LongitudinalOffsetEnum (enum)

The longitudinal offsets.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/a84274ce-172a-f056-3d8a-73a96ef7235c)

### Values
- `LONGITUDINAL_OFFSET_NONE` = `0` — The longitudinal offset is "none".
- `LONGITUDINAL_OFFSET_EXTEND_ONLY` = `1` — The longitudinal offset is "extend only".
- `LONGITUDINAL_OFFSET_ALWAYS` = `2` — The longitudinal offset is "always".

## AnalysisPartSpanningProperties (class)

The AnalysisPartSpanningProperties class contains information related to analysis part spanning properties.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/f3750ff1-4fd4-8586-6ebf-cdd7cc8156ef)

### Constructors
- `public AnalysisPartSpanningProperties()` — Initializes a new instance of the AnalysisPartSpanningProperties class

### Properties
- `SpanningDirection` (Vector, get/set) — The spanning direction (in the global coordinate system).
- `SpanningType` (AnalysisPartSpanningProperties.SpanningTypeEnum, get/set) — The spanning type.

## AnalysisPartSpanningProperties.SpanningTypeEnum (enum)

The spanning types.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/f61bcc98-8df1-fb60-e56d-9c1b51bcb940)

### Values
- `SPANNING_TYPE_SINGLE` = `1` — The single spanning type.
- `SPANNING_TYPE_DOUBLE` = `2` — The double spanning type.

## AnalysisPosition (class)

The AnalysisPosition class contains information related to analysis positions.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/f38a9da6-b305-8662-72e4-06bf8c3e5a02)

### Constructors
- `public AnalysisPosition()` — Initializes a new instance of the AnalysisPosition class

### Methods
#### `public override void Select()`

Selects an analysis position.

[Docs](https://developer.tekla.com/topic/en/18/47/365ea44a-0b65-d4ce-9977-bf4dd7bfe713)

### Properties
- `AnalysisModelName` (String, get/set) — Gets or sets the analysis model name.
- `AnalysisObjectType` (AnalysisObject.AnalysisObjectEnum, get/set) — Gets or sets the type of the analysis object.
- `FatherObject2ID` (Identifier, get/set) — Gets or sets the identifier of the father part.
- `FatherObject2Type` (AnalysisObject.AnalysisObjectEnum, get/set) — Gets or sets the type of the father object.
- `FatherObjectID` (Identifier, get/set) — Gets or sets the identifier of the father part.
- `FatherObjectType` (AnalysisObject.AnalysisObjectEnum, get/set) — Gets or sets the type of the father object.
- `ID` (Identifier, get/set) — Gets or sets the identifier of the analysis object.
- `Node` (AnalysisNode, get/set) — The node of the position.
- `Type` (AnalysisPosition.TypeEnum, get/set) — The type of the position.

## AnalysisPosition.TypeEnum (enum)

The position types.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/6b37b8a9-fc3d-ad14-e3cd-ca5d0f84ee57)

### Values
- `TYPE_NORMAL` = `0` — The normal type.
- `TYPE_SPLIT` = `1` — The split type.
- `TYPE_USER_SPLIT` = `9` — The user split type.

## AnalysisRestraintData (class)

The AnalysisRestraintData class contains the restraint data for an analysis part.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/3d4a69be-8813-ff6a-3d3b-9489ea90bd39)

### Constructors
- `public AnalysisRestraintData()` — Creates a new instance of an analysis restraint data.

### Properties
- `LocalCoordinateSystem` (AnalysisLocalCoordinateSystem, get/set) — The local coordinate system.
- `ReleaseType` (AnalysisRestraintData.ReleaseTypeEnum, get/set) — The release type.
- `RxSpring` (Double, get/set) — The rotation in the X-axis direction (Nmm/rad).
- `RxType` (AnalysisRestraintData.RestraintTypeEnum, get/set) — The restraint type for the rotation in the X-axis direction.
- `RySpring` (Double, get/set) — The rotation in the Y-axis direction (Nmm/rad).
- `RyType` (AnalysisRestraintData.RestraintTypeEnum, get/set) — The restraint type for the rotation in the Y-axis direction.
- `RzSpring` (Double, get/set) — The rotation in the Z-axis direction (Nmm/rad).
- `RzType` (AnalysisRestraintData.RestraintTypeEnum, get/set) — The restraint type for the rotation in the Z-axis direction.
- `Supported` (AnalysisRestraintData.SupportedEnum, get/set) — The support condition.
- `UxSpring` (Double, get/set) — The displacement in the X-axis direction (N/mm).
- `UxType` (AnalysisRestraintData.RestraintTypeEnum, get/set) — The restraint type for the displacement in the X-axis direction.
- `UySpring` (Double, get/set) — The displacement in the Y-axis direction (N/mm).
- `UyType` (AnalysisRestraintData.RestraintTypeEnum, get/set) — The restraint type for the displacement in the Y-axis direction.
- `UzSpring` (Double, get/set) — The displacement in the Z-axis direction (N/mm).
- `UzType` (AnalysisRestraintData.RestraintTypeEnum, get/set) — The restraint type for the displacement in the Z-axis direction.

## AnalysisRestraintData.ReleaseTypeEnum (enum)

Defines the release type.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/c4969558-6118-4b46-caa3-8633b1ab1df6)

### Values
- `FIXED_SUPPORTED` = `0` — The fixed and supported type.
- `PINNED_SUPPORTED` = `1` — The pinned and supported type.
- `FIXED_CONNECTED` = `2` — The fixed and connected type.
- `PINNED_CONNECTED` = `3` — The pinned and connected type.
- `USER_DEFINED` = `4` — The user defined type.

## AnalysisRestraintData.RestraintTypeEnum (enum)

Defines all the restraint types.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/4fde30d0-24b7-ba1b-7a4d-fd30e9d15113)

### Values
- `NOT_DEFINED` = `-3` — The restraint type is not defined.
- `PARTIAL_RELEASE` = `-2` — The partial release type.
- `FIXED` = `-1` — The fixed release type.
- `FREE` = `0` — The free release type.
- `SPRING` = `1` — The spring release type.

## AnalysisRestraintData.SupportedEnum (enum)

Defines how the part is supported.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/6ecc5af2-b81e-eb95-6670-da5178419279)

### Values
- `CONNECTED` = `0` — The part is connected.
- `SUPPORTED` = `1` — The part is supported.

## AnalysisRestraints (class)

The AnalysisRestraints class contains analysis restraints information for an analysis part.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/12b522fa-9fb8-a5b7-3fb2-b487f9a8e76e)

### Constructors
- `public AnalysisRestraints()` — Creates a new analysis restraints instance.

### Properties
- `End` (AnalysisRestraintData, get/set) — Data related to the second part end.
- `Start` (AnalysisRestraintData, get/set) — Data related to the first part end.

## AnalysisResult (class)

The AnalysisResult class contains analysis result information.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/9c245008-64d8-2846-76e1-b1ec8c6eb6b1)

### Constructors
- `public AnalysisResult()` — Creates a new analysis result instance.

### Methods
#### `public static bool Delete(AnalysisResult.ObjectTypeEnum objectType, int objectId)`

Delete analysis results by the given key.

**Parameters:**
- `objectType` (Tekla.Structures.Analysis.AnalysisResult.ObjectTypeEnum) — Object type.
- `objectId` (System.Int32) — Object id.

**Returns:** `Boolean` — True if successful.

[Docs](https://developer.tekla.com/topic/en/18/47/aef9329d-e4b2-886a-f21e-f9fac02c5aeb)

#### `public static bool DeleteAll()`

Delete all analysis results.

**Returns:** `Boolean` — True if successful.

[Docs](https://developer.tekla.com/topic/en/18/47/1a48788a-c8f2-8b6c-c545-6ec68430cc0e)

#### `public static bool DeleteLoadcase(int loadcaseNumber, string loadcaseName)`

Delete analysis results by load case.

**Parameters:**
- `loadcaseNumber` (System.Int32) — Object type.
- `loadcaseName` (System.String) — Object id.

**Returns:** `Boolean` — True if successful.

[Docs](https://developer.tekla.com/topic/en/18/47/a75b4887-0b6b-005d-59e6-ca8d03dc3240)

#### `public static List<int> GetResultObjectIds(AnalysisResult.ObjectTypeEnum objectType)`

Get ID numbers of objects with results.

**Parameters:**
- `objectType` (Tekla.Structures.Analysis.AnalysisResult.ObjectTypeEnum) — The object type.

**Returns:** `List<Int32>` — The ID numbers of objects with results.

[Docs](https://developer.tekla.com/topic/en/18/47/d985ddea-02c1-6c83-3801-3e013b31965e)

#### `public static List<AnalysisResult> GetResults(AnalysisResult.ObjectTypeEnum? objectType, int? objectId, AnalysisResult.ResultPositionEnum? position, Point point, AnalysisResult.ValueTypeEnum? valueType, bool sort)`

Get analysis results.

**Parameters:**
- `objectType` (System.Nullable<AnalysisResult.ObjectTypeEnum>) — The object type.
- `objectId` (System.Nullable<Int32>) — The object id or number (also object type must be given).
- `position` (System.Nullable<AnalysisResult.ResultPositionEnum>) — The position.
- `point` (Tekla.Structures.Geometry3d.Point) — The position point (when position is POSITION_BY_COORDINATES).
- `valueType` (System.Nullable<AnalysisResult.ValueTypeEnum>) — The value type.
- `sort` (System.Boolean) — True if values to be sorted.

**Returns:** `List<AnalysisResult>` — The results.

[Docs](https://developer.tekla.com/topic/en/18/47/dfecc942-baf1-3d44-c192-046868786c11)

#### `public bool Insert()`

Insert analysis result. Results are always stored to physical parts. When OBJECT_TYPE_MEMBER or OBJECT_TYPE_AREA is used the analysis model must be active. All results for given part must be inserted within one commit (CommitChanges), older results of this part are removed automatically.

**Returns:** `Boolean` — True if successful.

[Docs](https://developer.tekla.com/topic/en/18/47/110a28e3-09a8-5646-20a4-ac3a2252de80)

#### `public bool QueryAnalysisResult()`

Queries an analysis result.

**Returns:** `Boolean` — True if successfully queried an analysis result.

[Docs](https://developer.tekla.com/topic/en/18/47/792d80e9-e3ce-cbf2-626a-a3d9cad719f6)

### Properties
- `LoadCase` (Int32, get/set) — The load case. The real load cases have positive values 1, 2, 3, ... For min/max queries LoadCaseForMinMaxEnum can be used.
- `LoadCaseInfo` (String, get/set) — The load case info (optional, max length 128).
- `LoadCaseName` (String, get/set) — The load case name (max length 64).
- `LoadCaseType` (AnalysisResult.LoadCaseTypeEnum, get/set) — The load case type.
- `ModId` (Int32, get) — The time stamp of the result.
- `ObjectId` (Int32, get/set) — The identifier of the object. If ObjectType is OBJECT_TYPE_PART, ObjectId is (physical) part ID. If ObjectType is OBJECT_TYPE_MEMBER, ObjectId is the bar number when analysis bars are used in integration, otherwise ObjectId is the member number. If ObjectType is OBJECT_TYPE_AREA, ObjectId is the area number.
- `ObjectType` (AnalysisResult.ObjectTypeEnum, get/set) — The object type. In result query the only supported type is the physical member (the part).
- `Position` (AnalysisResult.ResultPositionEnum, get/set) — The position on the part's START/END/...
- `PositionCoordX` (Double, get/set) — Global coordinate X (mm) when inserting result. Local (part) coordinate X (mm) when getting results.
- `PositionCoordY` (Double, get/set) — Global coordinate Y (mm) when inserting result. Local (part) coordinate Y (mm) when getting results.
- `PositionCoordZ` (Double, get/set) — Global coordinate Z (mm) when inserting result. Local (part) coordinate Z (mm) when getting results.
- `PositionTolerance` (Double, get/set) — The position's coordinate tolerance.
- `Value` (Double, get/set) — The result value.
- `ValueType` (AnalysisResult.ValueTypeEnum, get/set) — The result value type.

## AnalysisResult.LoadCaseForMinMaxEnum (enum)

Special load case numbers for min/max queries.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/63b98a3d-878d-ce75-ffeb-dfa9212b67f0)

### Values
- `LOADCASE_WITH_MIN_VALUE` = `-1` — The load case with the minimum signed value of the queried value type.
- `LOADCASE_WITH_MAX_VALUE` = `-2` — The load case with the maximum signed value of the queried value type.
- `LOADCASE_WITH_MAX_ABSOLUTE_VALUE` = `-3` — The load case with the maximum absolute value of the queried value type.
- `LOADCASE_WITH_MAX_FX_VALUE` = `-11` — The load case with the maximum axial (FX) value.
- `LOADCASE_WITH_MIN_FX_VALUE` = `-12` — The load case with the minimum axial (FX) value.
- `LOADCASE_WITH_MAX_FY_VALUE` = `-13` — The load case with the maximum shear Y (FY) value.
- `LOADCASE_WITH_MIN_FY_VALUE` = `-14` — The load case with the minimum shear Y (FY) value.
- `LOADCASE_WITH_MAX_FZ_VALUE` = `-15` — The load case with the maximum shear Z (FZ) value.
- `LOADCASE_WITH_MIN_FZ_VALUE` = `-16` — The load case with the minimum shear Z (FZ) value.
- `LOADCASE_WITH_MAX_MX_VALUE` = `-17` — The load case with the maximum torsional moment (MX) value.
- `LOADCASE_WITH_MIN_MX_VALUE` = `-18` — The load case with the minimum torsional moment (MX) value.
- `LOADCASE_WITH_MAX_MY_VALUE` = `-19` — The load case with the maximum bending moment Y (MY) value.
- `LOADCASE_WITH_MIN_MY_VALUE` = `-20` — The load case with the minimum bending moment Y (MY) value.
- `LOADCASE_WITH_MAX_MZ_VALUE` = `-21` — The load case with the maximum bending moment Z (MZ) value.
- `LOADCASE_WITH_MIN_MZ_VALUE` = `-22` — The load case with the minimum bending moment Z (MZ) value.

## AnalysisResult.LoadCaseTypeEnum (enum)

The load case type.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/6ec3a114-0a91-d863-b137-45953d5d4c16)

### Values
- `CASE` = `1` — Load case (not a combination to be considered in design).
- `COMBINATION_ULS` = `2` — Ultimate limit state combination.
- `COMBINATION_SLS` = `3` — Service limit state combination.
- `COMBINATION` = `4` — Combination (not limit state).
- `ENVELOPE` = `5` — Envelope (from group of combinations).

## AnalysisResult.ObjectTypeEnum (enum)

The object type.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/6650f196-d4ff-c2de-60bd-d69a666f5331)

### Values
- `OBJECT_TYPE_MODEL` = `1` — The model object type.
- `OBJECT_TYPE_MEMBER` = `2` — The analytical member object type.
- `OBJECT_TYPE_NODE` = `3` — The node object type.
- `OBJECT_TYPE_PART` = `4` — The physical member object type.
- `OBJECT_TYPE_AREA` = `5` — The analytical area object type.

## AnalysisResult.ResultPositionEnum (enum)

The result position.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/a85864c1-c111-0d43-86d4-fc708b681cae)

### Values
- `POSITION_BY_COORDINATES` = `0` — The position given by the coordinates X, Y and Z.
- `POSITION_BY_COORDINATES_MINUS` = `6` — The position given by the coordinates X, Y and Z (minus side).
- `POSITION_BY_COORDINATES_PLUS` = `7` — The position given by the coordinates X, Y and Z (plus side).
- `POSITION_BEAM_START_POINT` = `1` — The member start position.
- `POSITION_BEAM_END_POINT` = `2` — The member end position.
- `POSITION_BEAM_MID_POINT` = `3` — The member middle position.
- `POSITION_BEAM_MID_POINT_MINUS` = `4` — The member middle position (minus side).
- `POSITION_BEAM_MID_POINT_PLUS` = `5` — The member middle position (plus side).
- `POSITION_MAX_NO_COORDINATES` = `201` — No position specified, gets the maximum.
- `POSITION_MIN_NO_COORDINATES` = `203` — No position specified, gets the minimum.

## AnalysisResult.ValueTypeEnum (enum)

The value type.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/e35fbe0b-b7da-a269-dbfc-8705df64e4f4)

### Values
- `VALUETYPE_AXIAL` = `1` — The axial (FX) (N) value type.
- `VALUETYPE_SHEAR_Y` = `2` — The shear Y (FY) (N) value type.
- `VALUETYPE_SHEAR_Z` = `3` — The shear Z (FZ) (N) value type.
- `VALUETYPE_TORSION_MOMENT` = `4` — The torsion moment (MX) (Nm) value type.
- `VALUETYPE_MOMENT_Y` = `5` — The bending moment Y (MY) (Nm) value type.
- `VALUETYPE_MOMENT_Z` = `6` — The bending moment Z (MZ) (Nm) value type.
- `VALUETYPE_SHEAR_YZ` = `51` — The shear resultant (positive) (N) value type.
- `VALUETYPE_MOMENT_YZ` = `52` — The moment resultant (positive) (Nm) value type.
- `VALUETYPE_ECC_MOMENT_Y` = `53` — The eccentricity moment Y (Nm) value type.
- `VALUETYPE_ECC_MOMENT_Z` = `54` — The eccentricity moment Z (Nm) value type.
- `VALUETYPE_STEEL_UTILITY_RATIO` = `101` — The utility ratio value type.
- `VALUETYPE_GLOBAL_REACTION_X` = `201` — The reaction in global X (N) value type.
- `VALUETYPE_GLOBAL_REACTION_Y` = `202` — The reaction in global Y (N) value type.
- `VALUETYPE_GLOBAL_REACTION_Z` = `203` — The reaction in global Z (N) value type.
- `VALUETYPE_DISPLACEMENT_X` = `301` — The displacement in global X (mm) value type.
- `VALUETYPE_DISPLACEMENT_Y` = `302` — The displacement in global Y (mm) value type.
- `VALUETYPE_DISPLACEMENT_Z` = `303` — The displacement in global Z (mm) value type.
- `VALUETYPE_ROTATION_X` = `304` — The rotation around global X (rad) value type.
- `VALUETYPE_ROTATION_Y` = `305` — The rotation around global Y (rad) value type.
- `VALUETYPE_ROTATION_Z` = `306` — The rotation around global Z (rad) value type.
- `VALUETYPE_REINFORCEMENT_AREA` = `400` — The reinforcement area (total) (mm2) value type.
- `VALUETYPE_REINFORCEMENT_BOTTOM` = `401` — The bottom reinforcement area (mm2) value type.
- `VALUETYPE_REINFORCEMENT_TOP` = `402` — The top reinforcement area (mm2) value type.
- `VALUETYPE_REINFORCEMENT_SHEAR` = `403` — The shear reinforcement area (mm2/m) value type.
- `VALUETYPE_SLAB_MAIN_DIRECTION_X` = `421` — The slab main direction cosine, X component, value type.
- `VALUETYPE_SLAB_MAIN_DIRECTION_Y` = `422` — The slab main direction cosine, Y component, value type.
- `VALUETYPE_SLAB_MAIN_DIRECTION_Z` = `423` — The slab main direction cosine, Z component, value type.
- `VALUETYPE_SLAB_SECONDARY_DIRECTION_X` = `424` — The slab secondary direction cosine, X component, value type.
- `VALUETYPE_SLAB_SECONDARY_MAIN_DIRECTION_Y` = `425` — The slab secondary direction cosine, Y component, value type.
- `VALUETYPE_SLAB_SECONDARY_MAIN_DIRECTION_Z` = `426` — The slab secondary direction cosine, Z component, value type.
- `VALUETYPE_SLAB_MAIN_REINFORCEMENT_BOTTOM` = `427` — The slab main reinforcement, bottom (mm2/m), value type.
- `VALUETYPE_SLAB_MAIN_REINFORCEMENT_TOP` = `428` — The slab main reinforcement, top (mm2/m), value type.
- `VALUETYPE_SLAB_SEC_REINFORCEMENT_BOTTOM` = `429` — The slab secondary reinforcement, bottom (mm2/m), value type.
- `VALUETYPE_SLAB_SEC_REINFORCEMENT_TOP` = `430` — The slab secondary reinforcement, top (mm2/m), value type.
- `VALUETYPE_SLAB_SHEAR_REINFORCEMENT` = `431` — The slab shear reinforcement (mm2/m) value type.
- `VALUETYPE_AXIAL_MINUS` = `11` — The axial (FX), minus side (N), value type.
- `VALUETYPE_SHEAR_Y_MINUS` = `12` — The shear Y (FY), minus side (N), value type.
- `VALUETYPE_SHEAR_Z_MINUS` = `13` — The shear Z (FZ), minus side (N), value type.
- `VALUETYPE_TORSION_MOMENT_MINUS` = `14` — The torsion moment (MX), minus side (Nm), value type.
- `VALUETYPE_MOMENT_Y_MINUS` = `15` — The bending moment Y (MY), minus side (Nm), value type.
- `VALUETYPE_MOMENT_Z_MINUS` = `16` — The bending moment Z (MZ), minus side (Nm), value type.
- `VALUETYPE_AXIAL_PLUS` = `21` — The axial (FX), plus side (N), value type.
- `VALUETYPE_SHEAR_Y_PLUS` = `22` — The shear Y (FY), plus side (N), value type.
- `VALUETYPE_SHEAR_Z_PLUS` = `23` — The shear Z (FZ), plus side (N), value type.
- `VALUETYPE_TORSION_MOMENT_PLUS` = `24` — The torsion moment (MX), plus side (Nm), value type.
- `VALUETYPE_MOMENT_Y_PLUS` = `25` — The bending moment Y (MY), plus side (Nm), value type.
- `VALUETYPE_MOMENT_Z_PLUS` = `26` — The bending moment Z (MZ), plus side (Nm), value type.

## AnalysisResultPositions (class)

The AnalysisResultPositions class contains information about analysis result positions.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/c7f892eb-bdaa-bff3-e743-d9977b9c252b)

### Constructors
- `public AnalysisResultPositions()` — Creates a new analysis result positions instance.

### Methods
#### `public bool GetResultPositions()`

Gets the analysis result positions.

**Returns:** `Boolean` — True if successfully got the analysis result positions.

[Docs](https://developer.tekla.com/topic/en/18/47/0e17cde9-3d30-1d38-2cdb-11721904ffcf)

### Properties
- `aPositions` (ArrayList, get) — The position types.
- `aPositionsX` (ArrayList, get) — The X coordinates of the positions.
- `aPositionsY` (ArrayList, get) — The Y coordinates of the positions.
- `aPositionsZ` (ArrayList, get) — The Z coordinates of the positions.
- `nPositions` (Int32, get) — The number of returned positions.
- `ObjectId` (Int32, get/set) — The identifier of the object.
- `ObjectType` (AnalysisResult.ObjectTypeEnum, get/set) — The object type. In result query the only supported type is the physical member (the part).

## AnalysisRigidDiaphragm (class)

The AnalysisRigidDiaphragm class contains information related to analysis rigid diaphragms.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/504098c8-62bf-74ad-0484-83dbd264e7e2)

### Constructors
- `public AnalysisRigidDiaphragm()` — Initializes a new instance of the AnalysisRigidDiaphragm class

### Methods
#### `public override void Select()`

Selects an analysis rigid diaphragm. The ID has to be set.

[Docs](https://developer.tekla.com/topic/en/18/47/d45af225-559c-3250-3bac-6dbe620a437f)

### Properties
- `AnalysisModelName` (String, get/set) — Gets or sets the analysis model name.
- `AnalysisObjectType` (AnalysisObject.AnalysisObjectEnum, get/set) — Gets or sets the type of the analysis object.
- `ExtremaMaximum` (Vector, get/set) — The maximum of the extrema (in the global coordinate system).
- `ExtremaMinimum` (Vector, get/set) — The minimum of the extrema (in the global coordinate system).
- `FatherObject2ID` (Identifier, get/set) — Gets or sets the identifier of the father part.
- `FatherObject2Type` (AnalysisObject.AnalysisObjectEnum, get/set) — Gets or sets the type of the father object.
- `FatherObjectID` (Identifier, get/set) — Gets or sets the identifier of the father part.
- `FatherObjectType` (AnalysisObject.AnalysisObjectEnum, get/set) — Gets or sets the type of the father object.
- `ID` (Identifier, get/set) — Gets or sets the identifier of the analysis object.
- `MasterNodeID` (Identifier, get/set) — The identifier of the master node.
- `PartIDs` (List<Identifier>, get/set) — The part identifier.
- `SlaveNodeIDs` (List<Identifier>, get/set) — The slave node identifier.

## AnalysisSubSection (class)

The AnalysisSubSection class contains information related to analysis subsections.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/4f52c2c7-2e95-c2d5-7fe1-4c7ebdb275bc)

### Constructors
- `public AnalysisSubSection()` — Initializes a new instance of the AnalysisSubSection class

### Methods
#### `public override void Select()`

Selects an analysis subsection. The analysis subsection identifier must be set.

[Docs](https://developer.tekla.com/topic/en/18/47/83c541c6-3c34-8a0b-1672-ac36de2181dc)

### Properties
- `AnalysisModelName` (String, get/set) — Gets or sets the analysis model name.
- `AnalysisObjectType` (AnalysisObject.AnalysisObjectEnum, get/set) — Gets or sets the type of the analysis object.
- `B` (Double, get/set) — The B value.
- `B1` (Double, get/set) — The B1 value.
- `B2` (Double, get/set) — The B2 value.
- `B3` (Double, get/set) — The B3 value.
- `B4` (Double, get/set) — The B4 value.
- `FatherObject2ID` (Identifier, get/set) — Gets or sets the identifier of the father part.
- `FatherObject2Type` (AnalysisObject.AnalysisObjectEnum, get/set) — Gets or sets the type of the father object.
- `FatherObjectID` (Identifier, get/set) — Gets or sets the identifier of the father part.
- `FatherObjectType` (AnalysisObject.AnalysisObjectEnum, get/set) — Gets or sets the type of the father object.
- `H` (Double, get/set) — The H value.
- `H1` (Double, get/set) — The H1 value.
- `H2` (Double, get/set) — The H2 value.
- `ID` (Identifier, get/set) — Gets or sets the identifier of the analysis object.
- `IsHole` (Boolean, get/set) — Defines whether the subsection is a hole.
- `Material` (AnalysisMaterial, get/set) — The material of the subsection.
- `OffsetAngle` (Double, get/set) — The offset angle.
- `OffsetY` (Double, get/set) — The offset in the direction of the Y-axis.
- `OffsetZ` (Double, get/set) — The offset in the direction of the Z-axis.
- `Points` (List<Point>, get/set) — The points of the subsection.
- `R1` (Double, get/set) — The R1 value.
- `R2` (Double, get/set) — The R2 value.
- `SubType` (AnalysisSubSection.AnalysisSubSectionSubTypeEnum, get/set) — The subtype of the subsection.
- `T1` (Double, get/set) — The T1 value.
- `T2` (Double, get/set) — The T2 value.
- `T3` (Double, get/set) — The T3 value.
- `T4` (Double, get/set) — The T4 value.
- `TF` (Double, get/set) — The TF value.
- `TW` (Double, get/set) — The TW value.
- `Type` (AnalysisSubSection.AnalysisSubSectionTypeEnum, get/set) — The type of the subsection.

## AnalysisSubSection.AnalysisSubSectionSubTypeEnum (enum)

Defines the subsection subtypes.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/7a6c4915-7fbc-2ebd-9d47-6200f3d9f91b)

### Values
- `UNKNOWN_SUBTYPE` = `0` — The subsection subtype is unknown.
- `I_HOT_ROLLED` = `1001` — The hot rolled I profile.
- `I_WELDED_SYMMETRICAL` = `1002` — The first symmetrically welded I profile.
- `I_WELDED_UNSYMMETRICAL` = `1003` — The first unsymmetrically welded I profile.
- `I_WELDED_SYMMETRICAL2` = `1004` — The second symmetrically welded I profile.
- `I_WELDED_UNSYMMETRICAL2` = `1005` — The second unsymmetrically welded I profile.
- `L_HOT_ROLLED` = `2001` — The hot rolled L profile.
- `L_COLD_ROLLED` = `2002` — The cold rolled L profile.
- `Z_COLD_ROLLED` = `3001` — The cold rolled Z profile.
- `U_HOT_ROLLED` = `4001` — The hot rolled U profile.
- `U_COLD_ROLLED` = `4002` — The cold rolled U profile.
- `PL_DEFAULT` = `5001` — The default PL profile.
- `D_CIRCULAR` = `6001` — The circular D profile.
- `D_ELLIPTICAL` = `6002` — The elliptical D profile.
- `PD_CIRCULAR` = `7001` — The circular PD profile.
- `PD_ELLIPTICAL` = `7002` — The elliptical PD profile.
- `PD_CIRCULAR_TAPERED` = `7003` — The circular tapered PD profile.
- `P_SQUARE` = `8001` — The square P profile.
- `P_RECTANGULAR` = `8002` — The rectangular P profile.
- `P_ALTERING_HEIGHT` = `8003` — The altering height P profile.
- `C_HOT_ROLLED` = `9001` — The hot rolled C profile.
- `C_COLD_ROLLED` = `9002` — The cold rolled C profile.
- `T_HOT_ROLLED` = `10001` — The hot rolled T profile.
- `T_PARAMETRIC` = `10002` — The parametric T profile.
- `HK_SYMMETRICAL` = `11001` — The symmetrical HK profile.
- `HK_UNSYMMETRICAL` = `11002` — The unsymmetrical HK profile.
- `HQ_CENTERED` = `13001` — The centered HQ profile.
- `HQ_NOT_CENTERED` = `13002` — The not centered HQ profile.
- `ZZ_SYMMETRICAL` = `15001` — The symmetrical ZZ profile.
- `ZZ_NOT_SYMMETRICAL` = `15002` — The unsymmetrical ZZ profile.
- `CC_SYMMETRICAL` = `16001` — The symmetrical CC profile.
- `CC_NOT_SYMMETRICAL` = `16002` — The unsymmetrical CC profile.
- `CW_SYMMETRICAL` = `17001` — The symmetrical CW profile.
- `CW_UNSYMMETRICAL` = `17002` — The unsymmetrical CW profile.
- `CU_SYMMETRICAL` = `18001` — The symmetrical CU profile.
- `CU_NOT_SYMMETRICAL` = `18002` — The unsymmetrical CU profile.
- `EB_SYMMETRICAL` = `19001` — The symmetrical EB profile.
- `EB_NOT_SYMMETRICAL` = `19002` — The unsymmetrical EB profile.
- `BF_DEFAULT` = `20001` — The default BF profile.
- `SPD_CIRCULAR` = `21001` — The circular SPD profile.
- `SPD_ELLIPTICAL` = `21002` — The elliptical SPD profile.
- `SPD_CIRCULAR_TAPERED` = `21003` — The circular tapered SPD profile.
- `EC_SYMMETRICAL` = `22001` — The symmetrical EC profile.
- `EC_NOT_SYMMETRICAL` = `22002` — The unsymmetrical EC profile.
- `ED_DEFAULT` = `23001` — The default ED profile.
- `EE_DEFAULT` = `24001` — The default EE profile.
- `EF_DEFAULT` = `25001` — The default EF profile.
- `EZ_DEFAULT` = `26001` — The default EZ profile.
- `EW_DEFAULT` = `27001` — The default EW profile.
- `RCDL_SYMMETRICAL` = `102001` — The symmetrical RCDL profile.
- `RCDL_UNSYMMETRICAL` = `102002` — The unsymmetrical RCDL profile.
- `RCXX_DEFAULT` = `103001` — The default RCXX profile.
- `RCL_DEFAULT` = `104001` — The default RCL profile.
- `RCDX_SYMMETRICAL` = `105001` — The symmetrical RCDX profile.
- `RCDX_UNSYMMETRICAL` = `105002` — The first unsymmetrical RCDX profile.
- `RCDX_UNSYMMETRICAL2` = `105003` — The second unsymmetrical RCDX profile.
- `RCX_DEFAULT` = `106001` — The default RCX profile.
- `COMPONENT_FIRST` = `999001` — The component first type.

## AnalysisSubSection.AnalysisSubSectionTypeEnum (enum)

Defines the subsection types.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/bc60fc0a-d228-41aa-af7d-702b9b32ca28)

### Values
- `UNKNOWN_TYPE` = `0` — The subsection subtype is unknown.
- `I` = `4` — The profile type I.
- `UNSYMMETRICAL_I` = `5` — The profile type unsymmetrical I.
- `L` = `6` — The profile type L.
- `Z` = `7` — The profile type Z.
- `U` = `8` — The profile type U.
- `PL` = `1` — The profile type PL.
- `D` = `10` — The profile type D.
- `PD` = `11` — The profile type PD.
- `P` = `9` — The profile type P.
- `C` = `8` — The profile type C.
- `T` = `2` — The profile type T.
- `T_MIRRORED` = `3` — The profile type T mirrored.
- `USER_DEFINED` = `12` — The profile type user defined.
- `TWIN_L_1` = `13` — The profile type twin L 1.
- `TWIN_L_2` = `14` — The profile type twin L 2.
- `TWIN_U_1` = `15` — The profile type twin U 1.
- `TWIN_U_2` = `16` — The profile type twin U 2.

## AnalysisSupport (class)

The AnalysisSupport class contains information related to analysis supports.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/5ce94939-b93c-f0fa-d7ac-5e15f77661bb)

### Constructors
- `public AnalysisSupport()` — Initializes a new instance of the AnalysisSupport class

### Properties
- `Spring` (.Double., get/set) — The spring values of the connectivity.
- `Support` (.AnalysisSupport.SupportEnum., get/set) — The connectivity type of the connectivity.

## AnalysisSupport.SupportEnum (enum)

The connectivity types.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/2952a91b-17a5-30fe-344b-162c691c522b)

### Values
- `SUPPORT_FREE` = `0` — The support is free.
- `SUPPORT_FIXED` = `1` — The support is fixed.
- `SUPPORT_SPRING` = `2` — The support is a spring.

## AnalysisVolume (class)

The AnalysisVolume class contains information related to analysis volumes.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/4782bb1f-4628-07bb-aa4f-99094c3629cb)

### Constructors
- `public AnalysisVolume()` — Initializes a new instance of the AnalysisVolume class

### Methods
#### `public override void Select()`

Selects an analysis volume. The ID has to be set.

[Docs](https://developer.tekla.com/topic/en/18/47/7303bff4-75d9-7040-a1a9-b07d136174d2)

### Properties
- `AnalysisModelName` (String, get/set) — Gets or sets the analysis model name.
- `AnalysisObjectType` (AnalysisObject.AnalysisObjectEnum, get/set) — Gets or sets the type of the analysis object.
- `FatherObject2ID` (Identifier, get/set) — Gets or sets the identifier of the father part.
- `FatherObject2Type` (AnalysisObject.AnalysisObjectEnum, get/set) — Gets or sets the type of the father object.
- `FatherObjectID` (Identifier, get/set) — Gets or sets the identifier of the father part.
- `FatherObjectType` (AnalysisObject.AnalysisObjectEnum, get/set) — Gets or sets the type of the father object.
- `ID` (Identifier, get/set) — Gets or sets the identifier of the analysis object.
- `Material` (AnalysisMaterial, get) — The material of the analysis volume.
- `Profile` (AnalysisCrossSection, get) — The profile of the analysis volume.
- `Volume` (AnalysisVolumeItem, get) — The volume item.
- `VolumeHoles` (List<AnalysisVolumeItem>, get) — The volume holes.

## AnalysisVolumeItem (class)

The AnalysisVolumeItem class contains information related to analysis volume items.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/58d014f3-6271-0881-a461-46fde0f2514e)

### Constructors
- `public AnalysisVolumeItem()` — Initializes a new instance of the AnalysisVolumeItem class

### Methods
#### `public override void Select()`

Selects an analysis volume item. The ID has to be set.

[Docs](https://developer.tekla.com/topic/en/18/47/300d85e1-7dc5-a134-95ad-d7fc147c0fbe)

### Properties
- `AnalysisModelName` (String, get/set) — Gets or sets the analysis model name.
- `AnalysisObjectType` (AnalysisObject.AnalysisObjectEnum, get/set) — Gets or sets the type of the analysis object.
- `Faces` (AnalysisObjectEnumerator, get/set) — The faces of the analysis volume item.
- `FatherObject2ID` (Identifier, get/set) — Gets or sets the identifier of the father part.
- `FatherObject2Type` (AnalysisObject.AnalysisObjectEnum, get/set) — Gets or sets the type of the father object.
- `FatherObjectID` (Identifier, get/set) — Gets or sets the identifier of the father part.
- `FatherObjectType` (AnalysisObject.AnalysisObjectEnum, get/set) — Gets or sets the type of the father object.
- `ID` (Identifier, get/set) — Gets or sets the identifier of the analysis object.

## DotAnalysisModelObjectOperation (struct)

The serialization structure for analysis model object operations.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/6723a0df-9ef7-71db-b04c-7beda2734bdd)

