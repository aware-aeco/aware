---
name: archicad-tapir-schema
description: This skill encodes the archicad 28.0 surface of the Tapir.Schema namespace — 203 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: AccuracyType, AngleType, AreaType, AttributeHeaders, AttributeHeadersOrError, AttributeHeadersWrapper, AttributeId, AttributeIdArrayItem, and 195 more types.
---

# Tapir.Schema

Auto-generated from vendor docs for archicad 28.0. 203 types in this namespace.

## AccuracyType (enum)

Methods for rounding decimal values.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#AccuracyType)

### Values
- `Off` = `Off`
- `ShowSmall5` = `ShowSmall5`
- `ShowSmall25` = `ShowSmall25`
- `ShowSmall1` = `ShowSmall1`
- `ShowSmall01` = `ShowSmall01`
- `InchCaseFractions` = `InchCaseFractions`

## AngleType (enum)

The type of the angle measurement unit.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#AngleType)

### Values
- `DecimalDegree` = `DecimalDegree`
- `DegreeMinSec` = `DegreeMinSec`
- `Grad` = `Grad`
- `Radian` = `Radian`
- `Surveyors` = `Surveyors`

## AreaType (enum)

The type of the area measurement unit.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#AreaType)

### Values
- `SquareMeter` = `SquareMeter`
- `SquareCentimeter` = `SquareCentimeter`
- `SquareMillimeter` = `SquareMillimeter`
- `SquareFoot` = `SquareFoot`
- `SquareInch` = `SquareInch`

## AttributeHeaders (class)

Details of attributes.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#AttributeHeaders)

### Properties
- `Items` (object[], get) — Array element type.

## AttributeHeadersOrError (class)

The response of the GetAttributesByType command or an error.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#AttributeHeadersOrError)

## AttributeHeadersWrapper (class)

The response of the GetAttributesByType command.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#AttributeHeadersWrapper)

### Properties
- `attributes` (AttributeHeaders, get) — Property `attributes` of type `AttributeHeaders` (required).

## AttributeId (class)

The identifier of an attribute.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#AttributeId)

### Properties
- `guid` (Guid, get) — Property `guid` of type `Guid` (required).

## AttributeIdArrayItem (class)

Shared schema type `AttributeIdArrayItem` defined in the Tapir / Graphisoft JSON Interface.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#AttributeIdArrayItem)

### Properties
- `attributeId` (AttributeId, get) — Property `attributeId` of type `AttributeId` (required).

## AttributeIds (class)

A list of attributes.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#AttributeIds)

### Properties
- `Items` (AttributeIdArrayItem[], get) — Array element type.

## AttributePropertyValue (class)

A property value with the identifiers of the property and its owner attribute.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#AttributePropertyValue)

### Properties
- `attributeId` (AttributeId, get) — Property `attributeId` of type `AttributeId` (required).
- `propertyId` (PropertyId, get) — Property `propertyId` of type `PropertyId` (required).
- `propertyValue` (PropertyValue, get) — Property `propertyValue` of type `PropertyValue` (required).

## AttributePropertyValues (class)

A list of attribute property values.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#AttributePropertyValues)

### Properties
- `Items` (AttributePropertyValue[], get) — Array element type.

## AttributeType (enum)

The type of an attribute.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#AttributeType)

### Values
- `Layer` = `Layer`
- `Line` = `Line`
- `Fill` = `Fill`
- `Composite` = `Composite`
- `Surface` = `Surface`
- `LayerCombination` = `LayerCombination`
- `ZoneCategory` = `ZoneCategory`
- `Profile` = `Profile`
- `PenTable` = `PenTable`
- `MEPSystem` = `MEPSystem`
- `OperationProfile` = `OperationProfile`
- `BuildingMaterial` = `BuildingMaterial`

## AutomaticZoneGeometry (class)

Automatic zone placement.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#AutomaticZoneGeometry)

### Properties
- `referencePosition` (Coordinate2D, get) — Reference point to automatically find zone.

## BasicDefaultValue (class)

Default value of the property in case of a basic property value (ie. not an expression).

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#BasicDefaultValue)

### Properties
- `basicDefaultValue` (PropertyValueDetails, get) — Property `basicDefaultValue` of type `PropertyValueDetails` (required).

## BeamDetails (class)

Shared schema type `BeamDetails` defined in the Tapir / Graphisoft JSON Interface.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#BeamDetails)

### Properties
- `begCoordinate` (Coordinate2D, get) — Property `begCoordinate` of type `Coordinate2D` (required).
- `endCoordinate` (Coordinate2D, get) — Property `endCoordinate` of type `Coordinate2D` (required).
- `zCoordinate` (number, get) — Property `zCoordinate` of type `number` (required).
- `level` (number, get) — base height of the beam relative to the floor level
- `offset` (number, get) — beam ref.line offset from the center
- `slantAngle` (number, get) — The slant angle of the beam in radians.
- `arcAngle` (number, get) — The arc angle of the (horizontally) curved beam in radians.
- `verticalCurveHeight` (number, get) — The height of the vertical curve of the beam.

## BoundingBox3D (class)

A 3D bounding box of an element.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#BoundingBox3D)

### Properties
- `xMin` (number, get) — The minimum X value of the bounding box.
- `yMin` (number, get) — The minimum Y value of the bounding box.
- `zMin` (number, get) — The minimum Z value of the bounding box.
- `xMax` (number, get) — The maximum X value of the bounding box.
- `yMax` (number, get) — The maximum Y value of the bounding box.
- `zMax` (number, get) — The maximum Z value of the bounding box.

## BoundingBox3DArrayItem (class)

A wrapper containing a 3D bounding box.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#BoundingBox3DArrayItem)

### Properties
- `boundingBox3D` (BoundingBox3D, get) — Property `boundingBox3D` of type `BoundingBox3D` (required).

## BoundingBox3DOrError (class)

A 3D bounding box or an error.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#BoundingBox3DOrError)

## BoundingBoxes3D (class)

A list of 3D bounding boxes.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#BoundingBoxes3D)

### Properties
- `Items` (BoundingBox3DOrError[], get) — Array element type.

## BuildingMaterialPhysicalProperties (class)

The physical properties of a single building material.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#BuildingMaterialPhysicalProperties)

### Properties
- `thermalConductivity` (number, get) — Thermal Conductivity.
- `density` (number, get) — Density.
- `heatCapacity` (number, get) — Heat Capacity.
- `embodiedEnergy` (number, get) — Embodied Energy.
- `embodiedCarbon` (number, get) — Embodied Carbon.

## BuildingMaterialPhysicalPropertiesArrayItem (class)

Shared schema type `BuildingMaterialPhysicalPropertiesArrayItem` defined in the Tapir / Graphisoft JSON Interface.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#BuildingMaterialPhysicalPropertiesArrayItem)

### Properties
- `properties` (BuildingMaterialPhysicalProperties, get) — Property `properties` of type `BuildingMaterialPhysicalProperties` (required).

## BuildingMaterialPhysicalPropertiesList (class)

A list of building material physical properties

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#BuildingMaterialPhysicalPropertiesList)

### Properties
- `Items` (BuildingMaterialPhysicalPropertiesArrayItem[], get) — Array element type.

## ClassificationId (class)

The element classification identifier.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#ClassificationId)

### Properties
- `classificationSystemId` (ClassificationSystemId, get) — Property `classificationSystemId` of type `ClassificationSystemId` (required).
- `classificationItemId` (ClassificationItemId, get/set) — The element's classification in the given system. If no value is specified here, the element is Unclassified in this system.

## ClassificationIdArrayItem (class)

A wrapper containing the classification identifier.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#ClassificationIdArrayItem)

### Properties
- `classificationId` (ClassificationId, get) — Property `classificationId` of type `ClassificationId` (required).

## ClassificationIdOrError (class)

A classification identifier or an error.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#ClassificationIdOrError)

## ClassificationIdsOrErrors (class)

A list of element classification identifiers or errors.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#ClassificationIdsOrErrors)

### Properties
- `Items` (ClassificationIdOrError[], get) — Array element type.

## ClassificationItemId (class)

The identifier of a classification item.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#ClassificationItemId)

### Properties
- `guid` (Guid, get) — Property `guid` of type `Guid` (required).

## ClassificationItemIdArrayItem (class)

Shared schema type `ClassificationItemIdArrayItem` defined in the Tapir / Graphisoft JSON Interface.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#ClassificationItemIdArrayItem)

### Properties
- `classificationItemId` (ClassificationItemId, get) — Property `classificationItemId` of type `ClassificationItemId` (required).

## ClassificationSystemId (class)

The identifier of a classification system.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#ClassificationSystemId)

### Properties
- `guid` (Guid, get) — Property `guid` of type `Guid` (required).

## ClassificationSystemIdArrayItem (class)

Shared schema type `ClassificationSystemIdArrayItem` defined in the Tapir / Graphisoft JSON Interface.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#ClassificationSystemIdArrayItem)

### Properties
- `classificationSystemId` (ClassificationSystemId, get) — Property `classificationSystemId` of type `ClassificationSystemId` (required).

## ClassificationSystemIds (class)

A list of classification system identifiers.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#ClassificationSystemIds)

### Properties
- `Items` (ClassificationSystemIdArrayItem[], get) — Array element type.

## ColorRGB (class)

RGB color.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#ColorRGB)

### Properties
- `red` (number, get) — Red value between 0.0 and 1.0
- `green` (number, get) — Green value between 0.0 and 1.0
- `blue` (number, get) — Blue value between 0.0 and 1.0

## ColumnDetails (class)

Shared schema type `ColumnDetails` defined in the Tapir / Graphisoft JSON Interface.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#ColumnDetails)

### Properties
- `origin` (Coordinate2D, get) — Property `origin` of type `Coordinate2D` (required).
- `zCoordinate` (number, get) — Property `zCoordinate` of type `number` (required).
- `height` (number, get) — height relative to bottom
- `bottomOffset` (number, get) — base level of the column relative to the floor level

## ConnectedElementsOrError (class)

The response of the GetConnectedElements command or an error.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#ConnectedElementsOrError)

## ConnectedElementsWrapper (class)

The response of the GetConnectedElements command.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#ConnectedElementsWrapper)

### Properties
- `connectedElements` (object[], get) — Property `connectedElements` of type `object[]` (required).

## Coordinate2D (class)

2D coordinate.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#Coordinate2D)

### Properties
- `x` (number, get) — X value of the coordinate.
- `y` (number, get) — Y value of the coordinate.

## Coordinate3D (class)

3D coordinate.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#Coordinate3D)

### Properties
- `x` (number, get) — X value of the coordinate.
- `y` (number, get) — Y value of the coordinate.
- `z` (number, get) — Z value of the coordinate.

## CurtainWallDetails (class)

Shared schema type `CurtainWallDetails` defined in the Tapir / Graphisoft JSON Interface.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#CurtainWallDetails)

### Properties
- `height` (number, get/set) — Property `height` of type `number` (optional).
- `angle` (number, get/set) — The rotation angle of the curtain wall in radians.

## CurtainWallFrameDetails (class)

Shared schema type `CurtainWallFrameDetails` defined in the Tapir / Graphisoft JSON Interface.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#CurtainWallFrameDetails)

### Properties
- `begCoordinate` (Coordinate3D, get) — Property `begCoordinate` of type `Coordinate3D` (required).
- `endCoordinate` (Coordinate3D, get) — Property `endCoordinate` of type `Coordinate3D` (required).
- `orientationVector` (Coordinate3D, get) — Property `orientationVector` of type `Coordinate3D` (required).
- `panelConnectionHole` (object, get) — The parameters of the panel connection hole.
- `frameContour` (object, get) — The parameters of the frame contour.
- `segmentIndex` (number, get) — The index of the curtain wall segment to which this frame belongs.
- `className` (string, get) — Property `className` of type `string` (required).
- `type` (CurtainWallFrameType, get) — Property `type` of type `CurtainWallFrameType` (required).

## CurtainWallFrameType (enum)

Enumeration of available curtain wall frame types.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#CurtainWallFrameType)

### Values
- `Deleted` = `Deleted`
- `Division` = `Division`
- `Corner` = `Corner`
- `Boundary` = `Boundary`
- `Custom` = `Custom`

## CurtainWallPanelDetails (class)

Shared schema type `CurtainWallPanelDetails` defined in the Tapir / Graphisoft JSON Interface.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#CurtainWallPanelDetails)

### Properties
- `polygonCoordinates` (Coordinate3D[], get) — The 3D coordinates of the panel polygon.
- `isHidden` (boolean, get) — Indicates if the panel is hidden (deleted panels remain in the database).
- `segmentIndex` (number, get) — The index of the curtain wall segment to which this panel belongs.
- `className` (string, get) — Property `className` of type `string` (required).
- `frames` (ElementIdArrayItem[], get) — The surrounding frames.

## CurtainWallSegmentDetails (class)

Shared schema type `CurtainWallSegmentDetails` defined in the Tapir / Graphisoft JSON Interface.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#CurtainWallSegmentDetails)

### Properties
- `begCoordinate` (Coordinate3D, get) — Property `begCoordinate` of type `Coordinate3D` (required).
- `endCoordinate` (Coordinate3D, get) — Property `endCoordinate` of type `Coordinate3D` (required).
- `extrusionVector` (Coordinate3D, get) — Property `extrusionVector` of type `Coordinate3D` (required).
- `gridOrigin` (Coordinate3D, get) — Property `gridOrigin` of type `Coordinate3D` (required).
- `gridAngle` (number, get) — The angle of the grid in radians.
- `arcOrigin` (Coordinate3D, get/set) — Property `arcOrigin` of type `Coordinate3D` (optional).
- `isNegativeArc` (boolean, get/set) — Indicates if the arc is negative.

## DatabaseId (class)

The identifier of a database

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#DatabaseId)

### Properties
- `guid` (Guid, get) — Property `guid` of type `Guid` (required).

## DatabaseIdArrayItem (class)

Shared schema type `DatabaseIdArrayItem` defined in the Tapir / Graphisoft JSON Interface.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#DatabaseIdArrayItem)

### Properties
- `databaseId` (DatabaseId, get) — Property `databaseId` of type `DatabaseId` (required).

## Databases (class)

A list of Archicad databases.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#Databases)

### Properties
- `Items` (DatabaseIdArrayItem[], get) — Array element type.

## DesignOptionIdArrayItem (class)

Shared schema type `DesignOptionIdArrayItem` defined in the Tapir / Graphisoft JSON Interface.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#DesignOptionIdArrayItem)

### Properties
- `designOptionId` (GuidId, get) — Property `designOptionId` of type `GuidId` (required).

## DetailWorksheetDetails (class)

Shared schema type `DetailWorksheetDetails` defined in the Tapir / Graphisoft JSON Interface.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#DetailWorksheetDetails)

### Properties
- `basePoint` (Coordinate2D, get) — Coordinate of the base point
- `angle` (number, get) — The rotation angle (radian) of the marker symbol
- `markerId` (ElementId, get) — Guid of the marker symbol
- `detailName` (string, get) — Name of the detail/worksheet
- `detailIdStr` (string, get) — Reference ID of the detail/worksheet
- `isHorizontalMarker` (boolean, get) — Marker symbol is always horizontal?
- `isWindowOpened` (boolean, get) — Side (detail/worksheet) window is opened?
- `clipPolygon` (Coordinate2D[], get) — The clip polygon of the detail/worksheet
- `linkData` (object, get) — The marker link data

## Dimensions3D (class)

Dimensions in 3D.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#Dimensions3D)

### Properties
- `x` (number, get) — X dimension.
- `y` (number, get) — Y dimension.
- `z` (number, get) — Z dimension.

## DisplayValueEnumId (class)

An enumeration value identifier using the displayed value.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#DisplayValueEnumId)

### Properties
- `type` (string, get) — Property `type` of type `string` (required).
- `displayValue` (string, get) — Property `displayValue` of type `string` (required).

## DocumentRevision (class)

Shared schema type `DocumentRevision` defined in the Tapir / Graphisoft JSON Interface.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#DocumentRevision)

### Properties
- `revisionId` (DocumentRevisionId, get) — Property `revisionId` of type `DocumentRevisionId` (required).
- `id` (string, get) — Property `id` of type `string` (required).
- `finalId` (string, get) — Property `finalId` of type `string` (required).
- `ownerUser` (string, get) — Property `ownerUser` of type `string` (required).
- `status` (string, get) — Property `status` of type `string` (required).
- `changes` (object[], get/set) — All changes belonging to the given document revision.
- `layoutInfo` (LayoutInfo, get) — Property `layoutInfo` of type `LayoutInfo` (required).

## DocumentRevisionId (class)

The identifier of a document revision.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#DocumentRevisionId)

### Properties
- `guid` (Guid, get) — Property `guid` of type `Guid` (required).

## DocumentRevisionReference (class)

A reference to a document revision belonging to the current issue

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#DocumentRevisionReference)

### Properties
- `revisionId` (DocumentRevisionId, get) — Property `revisionId` of type `DocumentRevisionId` (required).

## DocumentRevisionReferences (class)

All document revisions belong to the current issue.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#DocumentRevisionReferences)

### Properties
- `Items` (DocumentRevisionReference[], get) — Array element type.

## ElementClassification (class)

The classification of an element.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#ElementClassification)

### Properties
- `elementId` (ElementId, get) — Property `elementId` of type `ElementId` (required).
- `classificationId` (ClassificationId, get) — Property `classificationId` of type `ClassificationId` (required).

## ElementClassificationItemArray (class)

A wrapper containing a list of element classification identifiers or errors.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#ElementClassificationItemArray)

### Properties
- `classificationIds` (ClassificationIdsOrErrors, get) — Property `classificationIds` of type `ClassificationIdsOrErrors` (required).

## ElementClassificationOrError (class)

Element classification identifiers or errors.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#ElementClassificationOrError)

## ElementClassifications (class)

A list of element classification identifiers.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#ElementClassifications)

### Properties
- `Items` (ElementClassification[], get) — Array element type.

## ElementClassificationsOrErrors (class)

A list of element classification identifiers or errors.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#ElementClassificationsOrErrors)

### Properties
- `Items` (ElementClassificationOrError[], get) — Array element type.

## ElementFilter (enum)

A filter type for an element.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#ElementFilter)

### Values
- `IsEditable` = `IsEditable`
- `IsVisibleByLayer` = `IsVisibleByLayer`
- `IsVisibleByRenovation` = `IsVisibleByRenovation`
- `IsVisibleByStructureDisplay` = `IsVisibleByStructureDisplay`
- `IsVisibleIn3D` = `IsVisibleIn3D`
- `OnActualFloor` = `OnActualFloor`
- `OnActualLayout` = `OnActualLayout`
- `InMyWorkspace` = `InMyWorkspace`
- `IsIndependent` = `IsIndependent`
- `InCroppedView` = `InCroppedView`
- `HasAccessRight` = `HasAccessRight`
- `IsOverriddenByRenovation` = `IsOverriddenByRenovation`

## ElementGroupParameters (class)

The parameters for creating a single group

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#ElementGroupParameters)

### Properties
- `elements` (ElementOrGroupIds, get) — The elements or child groups to be grouped.
- `parentGroupId` (GroupId, get/set) — Optional parent group ID to nest this group under.

## ElementId (class)

The identifier of an element.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#ElementId)

### Properties
- `guid` (Guid, get) — Property `guid` of type `Guid` (required).

## ElementIdArrayItem (class)

Shared schema type `ElementIdArrayItem` defined in the Tapir / Graphisoft JSON Interface.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#ElementIdArrayItem)

### Properties
- `elementId` (ElementId, get) — Property `elementId` of type `ElementId` (required).

## ElementOrGroupId (class)

An identifier representing either an Element or a Group.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#ElementOrGroupId)

## ElementOrGroupIds (class)

A list of element and/or group identifiers.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#ElementOrGroupIds)

### Properties
- `Items` (ElementOrGroupId[], get) — Array element type.

## ElementPropertyValue (class)

A property value with the identifiers of the property and its owner element.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#ElementPropertyValue)

### Properties
- `elementId` (ElementId, get) — Property `elementId` of type `ElementId` (required).
- `propertyId` (PropertyId, get) — Property `propertyId` of type `PropertyId` (required).
- `propertyValue` (PropertyValue, get) — Property `propertyValue` of type `PropertyValue` (required).

## ElementPropertyValues (class)

A list of element property values.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#ElementPropertyValues)

### Properties
- `Items` (ElementPropertyValue[], get) — Array element type.

## ElementType (enum)

The type of an element.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#ElementType)

### Values
- `Wall` = `Wall`
- `Column` = `Column`
- `Beam` = `Beam`
- `Window` = `Window`
- `Door` = `Door`
- `Object` = `Object`
- `Lamp` = `Lamp`
- `Slab` = `Slab`
- `Roof` = `Roof`
- `Mesh` = `Mesh`
- `Dimension` = `Dimension`
- `RadialDimension` = `RadialDimension`
- `LevelDimension` = `LevelDimension`
- `AngleDimension` = `AngleDimension`
- `Text` = `Text`
- `Label` = `Label`
- `Zone` = `Zone`
- `Hatch` = `Hatch`
- `Line` = `Line`
- `PolyLine` = `PolyLine`
- `Arc` = `Arc`
- `Circle` = `Circle`
- `Spline` = `Spline`
- `Hotspot` = `Hotspot`
- `CutPlane` = `CutPlane`
- `Camera` = `Camera`
- `CamSet` = `CamSet`
- `Group` = `Group`
- `SectElem` = `SectElem`
- `Drawing` = `Drawing`
- `Picture` = `Picture`
- `Detail` = `Detail`
- `Elevation` = `Elevation`
- `InteriorElevation` = `InteriorElevation`
- `Worksheet` = `Worksheet`
- `Hotlink` = `Hotlink`
- `CurtainWall` = `CurtainWall`
- `CurtainWallSegment` = `CurtainWallSegment`
- `CurtainWallFrame` = `CurtainWallFrame`
- `CurtainWallPanel` = `CurtainWallPanel`
- `CurtainWallJunction` = `CurtainWallJunction`
- `CurtainWallAccessory` = `CurtainWallAccessory`
- `Shell` = `Shell`
- `Skylight` = `Skylight`
- `Morph` = `Morph`
- `ChangeMarker` = `ChangeMarker`
- `Stair` = `Stair`
- `Riser` = `Riser`
- `Tread` = `Tread`
- `StairStructure` = `StairStructure`
- `Railing` = `Railing`
- `RailingToprail` = `RailingToprail`
- `RailingHandrail` = `RailingHandrail`
- `RailingRail` = `RailingRail`
- `RailingPost` = `RailingPost`
- `RailingInnerPost` = `RailingInnerPost`
- `RailingBaluster` = `RailingBaluster`
- `RailingPanel` = `RailingPanel`
- `RailingSegment` = `RailingSegment`
- `RailingNode` = `RailingNode`
- `RailingBalusterSet` = `RailingBalusterSet`
- `RailingPattern` = `RailingPattern`
- `RailingToprailEnd` = `RailingToprailEnd`
- `RailingHandrailEnd` = `RailingHandrailEnd`
- `RailingRailEnd` = `RailingRailEnd`
- `RailingToprailConnection` = `RailingToprailConnection`
- `RailingHandrailConnection` = `RailingHandrailConnection`
- `RailingRailConnection` = `RailingRailConnection`
- `RailingEndFinish` = `RailingEndFinish`
- `BeamSegment` = `BeamSegment`
- `ColumnSegment` = `ColumnSegment`
- `Opening` = `Opening`
- `Unknown` = `Unknown`

## Elements (class)

A list of elements.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#Elements)

### Properties
- `Items` (ElementIdArrayItem[], get) — Array element type.

## ElementsWithExecutionResults (class)

The response of the GetElementsByType command.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#ElementsWithExecutionResults)

### Properties
- `elements` (Elements, get) — Property `elements` of type `Elements` (required).
- `executionResultForDatabases` (ExecutionResults, get/set) — Property `executionResultForDatabases` of type `ExecutionResults` (optional).

## ElementsWithExecutionResultsOrError (class)

The response of the GetElementsByType command or an error.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#ElementsWithExecutionResultsOrError)

## EnumValueId (class)

The identifier of a property enumeration value.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#EnumValueId)

## EnumValueIdArrayItem (class)

A wrapper containing the identifier of a property enumeration value.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#EnumValueIdArrayItem)

### Properties
- `enumValueId` (EnumValueId, get) — Property `enumValueId` of type `EnumValueId` (required).

## EnumValueIds (class)

A list of enumeration identifiers.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#EnumValueIds)

### Properties
- `Items` (EnumValueIdArrayItem[], get) — Array element type.

## Error (class)

The details of an error.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#Error)

### Properties
- `code` (integer, get) — The code of the error.
- `message` (string, get) — The error message.

## ErrorItem (class)

Shared schema type `ErrorItem` defined in the Tapir / Graphisoft JSON Interface.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#ErrorItem)

### Properties
- `error` (Error, get) — Property `error` of type `Error` (required).

## ExecutionResult (class)

The result of the execution.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#ExecutionResult)

## ExecutionResults (class)

A list of execution results.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#ExecutionResults)

### Properties
- `Items` (ExecutionResult[], get) — Array element type.

## ExpressionDefaultValue (class)

Default value of the property in case of an expression based property value.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#ExpressionDefaultValue)

### Properties
- `expressions` (string[], get) — Property `expressions` of type `string[]` (required).

## FailedExecutionResult (class)

The result of a failed execution.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#FailedExecutionResult)

### Properties
- `success` (boolean, get) — Property `success` of type `boolean` (required).
- `error` (Error, get) — The details of an execution failure.

## Favorites (class)

A list of favorite names

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#Favorites)

### Properties
- `Items` (string[], get) — Array element type.

## FavoritesOrError (class)

The response of the GetFavoritesByType command or an error.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#FavoritesOrError)

## FavoritesWrapper (class)

The response of the GetFavoritesByType command.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#FavoritesWrapper)

### Properties
- `favorites` (Favorites, get) — Property `favorites` of type `Favorites` (required).

## GDLParameterArray (class)

The list of GDL parameters.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#GDLParameterArray)

### Properties
- `Items` (GDLParameterDetails[], get) — Array element type.

## GDLParameterDetails (class)

Details of a GDL parameter.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#GDLParameterDetails)

### Properties
- `name` (string, get) — The name of the parameter.
- `displayName` (string, get) — The display name of the parameter.
- `index` (integer, get) — The index of the parameter.
- `type` (string, get) — The type of the parameter.
- `dimension1` (integer, get) — The 1st dimension of array (in case of array value).
- `dimension2` (integer, get) — The 2nd dimension of array (in case of array value).
- `value` (object, get) — The value of the parameter.
- `valueDescription` (string, get/set) — The value description for numeric parameter.
- `isLocked` (boolean, get) — The parameter is locked; i.e. the user cannot modify it
- `flags` (string[], get) — The flags of the parameter.
- `possibleValues` (oneOf, get/set) — Property `possibleValues` of type `oneOf` (optional).
- `canHaveCustomValue` (boolean, get/set) — The parameter can have a custom value.

## GDLParameterList (class)

The list of GDL parameters.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#GDLParameterList)

### Properties
- `parameters` (GDLParameterArray, get) — Property `parameters` of type `GDLParameterArray` (required).

## GroupId (class)

The identifier of a group.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#GroupId)

### Properties
- `guid` (Guid, get) — Property `guid` of type `Guid` (required).

## GroupIdArrayItem (class)

A wrapper containing the group identifier.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#GroupIdArrayItem)

### Properties
- `groupId` (GroupId, get) — Property `groupId` of type `GroupId` (required).

## GroupIdOrError (class)

A groupId or an error.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#GroupIdOrError)

## Guid (class)

A Globally Unique Identifier (or Universally Unique Identifier) in its string representation as defined in RFC 4122.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#Guid)

## GuidId (class)

Identifier.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#GuidId)

### Properties
- `guid` (Guid, get) — Property `guid` of type `Guid` (required).

## Hole2D (class)

A 2D hole in an element defined by closed polylines

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#Hole2D)

### Properties
- `polygonOutline` (Coordinate2D[], get/set) — The 2D coordinates of the edge of the hole.
- `polygonArcs` (PolyArc[], get/set) — Polygon outline arcs of the hole.

## Hole3D (class)

A 3D hole in an element defined by closed polylines

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#Hole3D)

### Properties
- `polygonCoordinates` (Coordinate3D[], get) — The 3D coordinates of the polygon of the hole.
- `polygonArcs` (PolyArc[], get/set) — Polygon outline arcs of the hole.

## Holes2D (class)

A list of 2D holes in an element defined by closed polylines

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#Holes2D)

### Properties
- `Items` (Hole2D[], get) — Array element type.

## Holes3D (class)

A list of 3D holes in an element defined by closed polylines

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#Holes3D)

### Properties
- `Items` (Hole3D[], get) — Array element type.

## Hotlink (class)

The details of a hotlink node.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#Hotlink)

### Properties
- `location` (string, get) — The path of the hotlink file.
- `children` (Hotlinks, get/set) — The children of the hotlink node if it has any.

## Hotlinks (class)

A list of hotlink nodes.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#Hotlinks)

### Properties
- `Items` (Hotlink[], get) — Array element type.

## IssueCommentStatus (enum)

The status of an issue comment.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#IssueCommentStatus)

### Values
- `Error` = `Error`
- `Warning` = `Warning`
- `Info` = `Info`
- `Unknown` = `Unknown`

## IssueElementType (enum)

The attachment type of an element component of an issue.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#IssueElementType)

### Values
- `Creation` = `Creation`
- `Highlight` = `Highlight`
- `Deletion` = `Deletion`
- `Modification` = `Modification`

## IssueId (class)

The identifier of an issue.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#IssueId)

### Properties
- `guid` (Guid, get) — Property `guid` of type `Guid` (required).

## IssueIdArrayItem (class)

Shared schema type `IssueIdArrayItem` defined in the Tapir / Graphisoft JSON Interface.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#IssueIdArrayItem)

### Properties
- `issueId` (IssueId, get) — Property `issueId` of type `IssueId` (required).

## Issues (class)

A list of Issues.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#Issues)

### Properties
- `Items` (IssueIdArrayItem[], get) — Array element type.

## LayerCombinationAttribute (class)

A layer combination attribute.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#LayerCombinationAttribute)

### Properties
- `layerCombination` (LayerCombinationAttributeDetails, get) — Property `layerCombination` of type `LayerCombinationAttributeDetails` (required).

## LayerCombinationAttributeDetails (class)

The details of the layer combination attribute.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#LayerCombinationAttributeDetails)

### Properties
- `attributeId` (AttributeId, get) — The identifier of the layer combination attribute.
- `attributeIndex` (integer, get/set) — The index identifier of the layer combination attribute.
- `name` (string, get) — The name of the layer combination.
- `layers` (LayersOfLayerCombination, get) — Property `layers` of type `LayersOfLayerCombination` (required).

## LayerCombinationAttributeOrError (class)

A layer combination attribute or an error.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#LayerCombinationAttributeOrError)

## LayersOfLayerCombination (class)

List of Layers included in the Layer Combination.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#LayersOfLayerCombination)

### Properties
- `Items` (object[], get) — Array element type.

## LayoutInfo (class)

Shared schema type `LayoutInfo` defined in the Tapir / Graphisoft JSON Interface.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#LayoutInfo)

### Properties
- `id` (string, get) — Property `id` of type `string` (required).
- `databaseId` (DatabaseId, get) — Property `databaseId` of type `DatabaseId` (required).
- `name` (string, get) — Property `name` of type `string` (required).
- `masterLayoutName` (string, get) — Property `masterLayoutName` of type `string` (required).
- `width` (number, get) — Property `width` of type `number` (required).
- `height` (number, get) — Property `height` of type `number` (required).
- `subsetId` (string, get) — Property `subsetId` of type `string` (required).
- `subsetName` (string, get) — Property `subsetName` of type `string` (required).
- `ownerUser` (string, get) — Property `ownerUser` of type `string` (required).
- `customSchemeData` (RevisionCustomSchemeData, get/set) — Property `customSchemeData` of type `RevisionCustomSchemeData` (optional).

## LengthType (enum)

The type of the length measurement unit.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#LengthType)

### Values
- `Meter` = `Meter`
- `Decimeter` = `Decimeter`
- `Centimeter` = `Centimeter`
- `Millimeter` = `Millimeter`
- `FootFracInch` = `FootFracInch`
- `FootDecInch` = `FootDecInch`
- `DecFoot` = `DecFoot`
- `FracInch` = `FracInch`
- `DecInch` = `DecInch`

## LibPartBasedElementDetails (class)

Shared schema type `LibPartBasedElementDetails` defined in the Tapir / Graphisoft JSON Interface.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#LibPartBasedElementDetails)

### Properties
- `libPart` (LibPartDetails, get) — Property `libPart` of type `LibPartDetails` (required).
- `ownerElementId` (ElementId, get/set) — Property `ownerElementId` of type `ElementId` (optional).
- `ownerElementType` (ElementType, get/set) — Property `ownerElementType` of type `ElementType` (optional).

## LibPartDetails (class)

Shared schema type `LibPartDetails` defined in the Tapir / Graphisoft JSON Interface.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#LibPartDetails)

### Properties
- `name` (string, get) — Property `name` of type `string` (required).
- `parentUnID` (LibPartUnId, get) — Property `parentUnID` of type `LibPartUnId` (required).
- `ownUnID` (LibPartUnId, get) — Property `ownUnID` of type `LibPartUnId` (required).

## LibPartUnId (class)

Shared schema type `LibPartUnId` defined in the Tapir / Graphisoft JSON Interface.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#LibPartUnId)

### Properties
- `guid` (Guid, get) — Property `guid` of type `Guid` (required).

## LibraryFileAddition (class)

Shared schema type `LibraryFileAddition` defined in the Tapir / Graphisoft JSON Interface.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#LibraryFileAddition)

### Properties
- `inputPath` (string, get) — The path to the input file.
- `outputPath` (string, get) — The relative path to the new file inside embedded library.
- `type` (LibraryPartType, get/set) — The type of the library part. By default 'Pict'.

## LibraryFileAdditions (class)

A list of library file additions to the embedded library

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#LibraryFileAdditions)

### Properties
- `Items` (LibraryFileAddition[], get) — Array element type.

## LibraryPartType (enum)

Enumeration of available library part types. 'Unknown' is the schema-valid catch-all returned for any libpart whose typeID is not one of the named values (rare ACAPI sentinels and any future SDK subtype).

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#LibraryPartType)

### Values
- `Spec` = `Spec`
- `Window` = `Window`
- `Door` = `Door`
- `Object` = `Object`
- `Lamp` = `Lamp`
- `Room` = `Room`
- `Property` = `Property`
- `PlanSign` = `PlanSign`
- `Label` = `Label`
- `Macro` = `Macro`
- `Pict` = `Pict`
- `Picture` = `Picture`
- `ListScheme` = `ListScheme`
- `Skylight` = `Skylight`
- `OpeningSymbol` = `OpeningSymbol`
- `Unknown` = `Unknown`

## ManualZoneGeometry (class)

Manual zone placement.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#ManualZoneGeometry)

### Properties
- `polygonCoordinates` (Coordinate2D[], get) — The 2D coordinates of the edge of the zone.
- `polygonArcs` (PolyArc[], get/set) — Polygon outline arcs of the zone.
- `holes` (Holes2D, get/set) — Property `holes` of type `Holes2D` (optional).

## MeshDetails (class)

Shared schema type `MeshDetails` defined in the Tapir / Graphisoft JSON Interface.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#MeshDetails)

### Properties
- `level` (number, get) — The Z reference level of coordinates.
- `skirtType` (MeshSkirtType, get) — Property `skirtType` of type `MeshSkirtType` (required).
- `skirtLevel` (number, get) — The height of the skirt.
- `polygonCoordinates` (Coordinate3D[], get) — The 3D coordinates of the outline polygon of the mesh.
- `polygonArcs` (PolyArc[], get/set) — Polygon outline arcs of the mesh.
- `holes` (Holes3D, get/set) — Property `holes` of type `Holes3D` (optional).
- `sublines` (object[], get/set) — The leveling sublines inside the polygon of the mesh.

## MeshSkirtType (enum)

The type of the skirt structure.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#MeshSkirtType)

### Values
- `SurfaceOnlyWithoutSkirt` = `SurfaceOnlyWithoutSkirt`
- `WithSkirt` = `WithSkirt`
- `SolidBodyWithSkirt` = `SolidBodyWithSkirt`

## NavigatorItemId (class)

The identifier of a navigator item.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#NavigatorItemId)

### Properties
- `guid` (Guid, get) — Property `guid` of type `Guid` (required).

## NavigatorItemIdArrayItem (class)

Shared schema type `NavigatorItemIdArrayItem` defined in the Tapir / Graphisoft JSON Interface.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#NavigatorItemIdArrayItem)

### Properties
- `navigatorItemId` (NavigatorItemId, get) — Property `navigatorItemId` of type `NavigatorItemId` (required).

## NavigatorItemIds (class)

A list of navigator item identifiers.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#NavigatorItemIds)

### Properties
- `Items` (NavigatorItemIdArrayItem[], get) — Array element type.

## NonLocalizedValueEnumId (class)

An enumeration value identifier using the nonlocalized value.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#NonLocalizedValueEnumId)

### Properties
- `type` (string, get) — Property `type` of type `string` (required).
- `nonLocalizedValue` (string, get) — Property `nonLocalizedValue` of type `string` (required).

## NormalAngleListPropertyValue (class)

An angle list property value containing angles in an array. The values are measured in SI (radians).

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#NormalAngleListPropertyValue)

### Properties
- `type` (string, get) — Property `type` of type `string` (required).
- `status` (string, get) — Property `status` of type `string` (required).
- `value` (number[], get) — Property `value` of type `number[]` (required).

## NormalAnglePropertyValue (class)

An angle property value containing a real angle. The value is measured in SI (radians).

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#NormalAnglePropertyValue)

### Properties
- `type` (string, get) — Property `type` of type `string` (required).
- `status` (string, get) — Property `status` of type `string` (required).
- `value` (number, get) — Property `value` of type `number` (required).

## NormalAreaListPropertyValue (class)

An area list property value containing areas in an array. The values are measured in SI (square meters).

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#NormalAreaListPropertyValue)

### Properties
- `type` (string, get) — Property `type` of type `string` (required).
- `status` (string, get) — Property `status` of type `string` (required).
- `value` (number[], get) — Property `value` of type `number[]` (required).

## NormalAreaPropertyValue (class)

An area property value containing a real area. The value is measured in SI (square meters).

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#NormalAreaPropertyValue)

### Properties
- `type` (string, get) — Property `type` of type `string` (required).
- `status` (string, get) — Property `status` of type `string` (required).
- `value` (number, get) — Property `value` of type `number` (required).

## NormalBooleanListPropertyValue (class)

A boolean list property value containing boolean values in an array.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#NormalBooleanListPropertyValue)

### Properties
- `type` (string, get) — Property `type` of type `string` (required).
- `status` (string, get) — Property `status` of type `string` (required).
- `value` (boolean[], get) — Property `value` of type `boolean[]` (required).

## NormalBooleanPropertyValue (class)

A boolean property value containing a valid boolean value.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#NormalBooleanPropertyValue)

### Properties
- `type` (string, get) — Property `type` of type `string` (required).
- `status` (string, get) — Property `status` of type `string` (required).
- `value` (boolean, get) — Property `value` of type `boolean` (required).

## NormalIntegerListPropertyValue (class)

An integer list property value containing integers in an array.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#NormalIntegerListPropertyValue)

### Properties
- `type` (string, get) — Property `type` of type `string` (required).
- `status` (string, get) — Property `status` of type `string` (required).
- `value` (integer[], get) — Property `value` of type `integer[]` (required).

## NormalIntegerPropertyValue (class)

An integer property value containing a valid integer number.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#NormalIntegerPropertyValue)

### Properties
- `type` (string, get) — Property `type` of type `string` (required).
- `status` (string, get) — Property `status` of type `string` (required).
- `value` (integer, get) — Property `value` of type `integer` (required).

## NormalLengthListPropertyValue (class)

A length list property value containing length values in an array. The values are measured in SI (meters).

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#NormalLengthListPropertyValue)

### Properties
- `type` (string, get) — Property `type` of type `string` (required).
- `status` (string, get) — Property `status` of type `string` (required).
- `value` (number[], get) — Property `value` of type `number[]` (required).

## NormalLengthPropertyValue (class)

A length property value containing a real length value. The value is measured in SI (meters).

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#NormalLengthPropertyValue)

### Properties
- `type` (string, get) — Property `type` of type `string` (required).
- `status` (string, get) — Property `status` of type `string` (required).
- `value` (number, get) — Property `value` of type `number` (required).

## NormalMultiEnumPropertyValue (class)

A multiple choice enumeration property value containing the IDs of the selected enum values in an array.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#NormalMultiEnumPropertyValue)

### Properties
- `type` (string, get) — Property `type` of type `string` (required).
- `status` (string, get) — Property `status` of type `string` (required).
- `value` (EnumValueIds, get) — Property `value` of type `EnumValueIds` (required).

## NormalNumberListPropertyValue (class)

A number list property value containing numbers in an array.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#NormalNumberListPropertyValue)

### Properties
- `type` (string, get) — Property `type` of type `string` (required).
- `status` (string, get) — Property `status` of type `string` (required).
- `value` (number[], get) — Property `value` of type `number[]` (required).

## NormalNumberPropertyValue (class)

A number property value containing a valid numeric value.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#NormalNumberPropertyValue)

### Properties
- `type` (string, get) — Property `type` of type `string` (required).
- `status` (string, get) — Property `status` of type `string` (required).
- `value` (number, get) — Property `value` of type `number` (required).

## NormalOrUserUndefinedPropertyValue (class)

A normal or a userUndefined property value.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#NormalOrUserUndefinedPropertyValue)

## NormalSingleEnumPropertyValue (class)

A single enumeration property value containing the ID of the selected enum value.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#NormalSingleEnumPropertyValue)

### Properties
- `type` (string, get) — Property `type` of type `string` (required).
- `status` (string, get) — Property `status` of type `string` (required).
- `value` (EnumValueId, get) — Property `value` of type `EnumValueId` (required).

## NormalStringListPropertyValue (class)

A string list property value containing strings in an array.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#NormalStringListPropertyValue)

### Properties
- `type` (string, get) — Property `type` of type `string` (required).
- `status` (string, get) — Property `status` of type `string` (required).
- `value` (string[], get) — Property `value` of type `string[]` (required).

## NormalStringPropertyValue (class)

A string property value containing a valid string.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#NormalStringPropertyValue)

### Properties
- `type` (string, get) — Property `type` of type `string` (required).
- `status` (string, get) — Property `status` of type `string` (required).
- `value` (string, get) — Property `value` of type `string` (required).

## NormalVolumeListPropertyValue (class)

A volume list property value containing volumes in an array. The values are measured in SI (cubic meters).

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#NormalVolumeListPropertyValue)

### Properties
- `type` (string, get) — Property `type` of type `string` (required).
- `status` (string, get) — Property `status` of type `string` (required).
- `value` (number[], get) — Property `value` of type `number[]` (required).

## NormalVolumePropertyValue (class)

A volume property value containing a real volume. The value is measured in SI (cubic meters).

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#NormalVolumePropertyValue)

### Properties
- `type` (string, get) — Property `type` of type `string` (required).
- `status` (string, get) — Property `status` of type `string` (required).
- `value` (number, get) — Property `value` of type `number` (required).

## NotAvailablePropertyValue (class)

A notAvailable value means that the property is not available for the property owner (and therefore it has no property value for it).

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#NotAvailablePropertyValue)

### Properties
- `type` (PropertyDataType, get) — Property `type` of type `PropertyDataType` (required).
- `status` (string, get) — Property `status` of type `string` (required).

## NotYetSupportedElementTypeDetails (class)

Shared schema type `NotYetSupportedElementTypeDetails` defined in the Tapir / Graphisoft JSON Interface.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#NotYetSupportedElementTypeDetails)

### Properties
- `error` (string, get) — Property `error` of type `string` (required).

## ObjectDetails (class)

Shared schema type `ObjectDetails` defined in the Tapir / Graphisoft JSON Interface.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#ObjectDetails)

### Properties
- `origin` (Coordinate3D, get) — Property `origin` of type `Coordinate3D` (required).
- `dimensions` (Coordinate3D, get) — Property `dimensions` of type `Coordinate3D` (required).
- `angle` (number, get) — Property `angle` of type `number` (required).

## PolyArc (class)

Representation of an arc segment of a two dimensional polygon/polyline.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#PolyArc)

### Properties
- `begIndex` (integer, get) — Node index of one end point of the arc.
- `endIndex` (integer, get) — Node index of the other end point of the arc.
- `arcAngle` (number, get) — Angle of the arc; it is positive, if the arc is on the right-hand side of the straight segment.

## PolylineDetails (class)

Shared schema type `PolylineDetails` defined in the Tapir / Graphisoft JSON Interface.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#PolylineDetails)

### Properties
- `coordinates` (Coordinate2D[], get) — Property `coordinates` of type `Coordinate2D[]` (required).
- `arcs` (PolyArc[], get/set) — The arcs of the polyline.
- `zCoordinate` (number, get) — Property `zCoordinate` of type `number` (required).

## PossibleNumericValues (class)

The possible numeric values of a GDL parameter.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#PossibleNumericValues)

### Properties
- `Items` (object[], get) — Array element type.

## PossibleStringValues (class)

The possible string values of a GDL parameter.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#PossibleStringValues)

### Properties
- `Items` (string[], get) — Array element type.

## ProjectInfoField (class)

Shared schema type `ProjectInfoField` defined in the Tapir / Graphisoft JSON Interface.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#ProjectInfoField)

### Properties
- `projectInfoId` (string, get) — The id of the project info field.
- `projectInfoName` (string, get) — The name of the project info field visible on UI.
- `projectInfoValue` (string, get) — The value of the project info field.

## ProjectInfoFields (class)

A list of project info fields.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#ProjectInfoFields)

### Properties
- `Items` (ProjectInfoField[], get) — Array element type.

## PropertyDataType (enum)

Shared schema type `PropertyDataType` defined in the Tapir / Graphisoft JSON Interface.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#PropertyDataType)

### Values
- `number` = `number`
- `integer` = `integer`
- `string` = `string`
- `boolean` = `boolean`
- `length` = `length`
- `area` = `area`
- `volume` = `volume`
- `angle` = `angle`
- `numberList` = `numberList`
- `integerList` = `integerList`
- `stringList` = `stringList`
- `booleanList` = `booleanList`
- `lengthList` = `lengthList`
- `areaList` = `areaList`
- `volumeList` = `volumeList`
- `angleList` = `angleList`
- `singleEnum` = `singleEnum`
- `multiEnum` = `multiEnum`

## PropertyDefaultValue (class)

Shared schema type `PropertyDefaultValue` defined in the Tapir / Graphisoft JSON Interface.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#PropertyDefaultValue)

## PropertyDefinition (class)

Shared schema type `PropertyDefinition` defined in the Tapir / Graphisoft JSON Interface.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#PropertyDefinition)

### Properties
- `name` (string, get) — Property `name` of type `string` (required).
- `description` (string, get) — Property `description` of type `string` (required).
- `type` (PropertyDataType, get) — Property `type` of type `PropertyDataType` (required).
- `isEditable` (boolean, get) — Property `isEditable` of type `boolean` (required).
- `defaultValue` (PropertyDefaultValue, get/set) — Property `defaultValue` of type `PropertyDefaultValue` (optional).
- `possibleEnumValues` (object[], get/set) — The possible enum values of the property when the property type is enumeration.
- `availability` (ClassificationItemIdArrayItem[], get) — The identifiers of classification items the new property is available for.
- `group` (object, get) — The property group defined by name or id. If both fields exists the id will be used.

## PropertyDefinitionArrayItem (class)

A wrapper containing a property definition

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#PropertyDefinitionArrayItem)

### Properties
- `propertyDefinition` (PropertyDefinition, get) — Property `propertyDefinition` of type `PropertyDefinition` (required).

## PropertyDetails (class)

The details of the property.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#PropertyDetails)

### Properties
- `propertyId` (PropertyId, get) — Property `propertyId` of type `PropertyId` (required).
- `propertyType` (string, get) — Property `propertyType` of type `string` (required).
- `propertyGroupName` (string, get) — Property `propertyGroupName` of type `string` (required).
- `propertyName` (string, get) — Property `propertyName` of type `string` (required).
- `propertyCollectionType` (string, get) — Property `propertyCollectionType` of type `string` (required).
- `propertyValueType` (string, get) — Property `propertyValueType` of type `string` (required).
- `propertyMeasureType` (string, get) — Property `propertyMeasureType` of type `string` (required).
- `propertyIsEditable` (boolean, get) — Property `propertyIsEditable` of type `boolean` (required).

## PropertyGroup (class)

Represents a property group.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#PropertyGroup)

### Properties
- `name` (string, get) — Property `name` of type `string` (required).
- `description` (string, get/set) — Property `description` of type `string` (optional).

## PropertyGroupArrayItem (class)

A wrapper containing a property group

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#PropertyGroupArrayItem)

### Properties
- `propertyGroup` (PropertyGroup, get) — Property `propertyGroup` of type `PropertyGroup` (required).

## PropertyGroupId (class)

The identifier of a property group.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#PropertyGroupId)

### Properties
- `guid` (Guid, get) — Property `guid` of type `Guid` (required).

## PropertyGroupIdArrayItem (class)

A wrapper containing the property group identifier.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#PropertyGroupIdArrayItem)

### Properties
- `propertyGroupId` (PropertyGroupId, get) — Property `propertyGroupId` of type `PropertyGroupId` (required).

## PropertyId (class)

The identifier of a property.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#PropertyId)

### Properties
- `guid` (Guid, get) — Property `guid` of type `Guid` (required).

## PropertyIdArrayItem (class)

A wrapper containing the property identifier.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#PropertyIdArrayItem)

### Properties
- `propertyId` (PropertyId, get) — Property `propertyId` of type `PropertyId` (required).

## PropertyIdOrError (class)

A propertyId or an error.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#PropertyIdOrError)

## PropertyIdOrErrorArray (class)

A list of property identifiers.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#PropertyIdOrErrorArray)

### Properties
- `Items` (PropertyIdOrError[], get) — Array element type.

## PropertyIds (class)

A list of property identifiers.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#PropertyIds)

### Properties
- `Items` (PropertyIdArrayItem[], get) — Array element type.

## PropertyValue (class)

The display string value of a property.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#PropertyValue)

### Properties
- `value` (string, get) — Property `value` of type `string` (required).

## PropertyValueArrayItem (class)

A wrapper containing the property value.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#PropertyValueArrayItem)

### Properties
- `propertyValue` (PropertyValue, get) — Property `propertyValue` of type `PropertyValue` (required).

## PropertyValueDetails (class)

A normal, userUndefined, notAvailable or notEvaluated property value.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#PropertyValueDetails)

## PropertyValueOrErrorItem (class)

A property value or an error

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#PropertyValueOrErrorItem)

## PropertyValues (class)

A list of property values.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#PropertyValues)

### Properties
- `Items` (PropertyValueOrErrorItem[], get) — Array element type.

## PropertyValuesArrayItem (class)

A wrapper containing the property values.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#PropertyValuesArrayItem)

### Properties
- `propertyValues` (PropertyValues, get) — Property `propertyValues` of type `PropertyValues` (required).

## PropertyValuesOrError (class)

A list of property values or an error.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#PropertyValuesOrError)

## PropertyValuesOrErrorArray (class)

A list of property value lists.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#PropertyValuesOrErrorArray)

### Properties
- `Items` (PropertyValuesOrError[], get) — Array element type.

## RevisionChange (class)

Shared schema type `RevisionChange` defined in the Tapir / Graphisoft JSON Interface.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#RevisionChange)

### Properties
- `id` (string, get) — Property `id` of type `string` (required).
- `description` (string, get) — Property `description` of type `string` (required).
- `lastModifiedTime` (string, get) — Property `lastModifiedTime` of type `string` (required).
- `modifiedByUser` (string, get) — Property `modifiedByUser` of type `string` (required).
- `isIssued` (boolean, get) — Property `isIssued` of type `boolean` (required).
- `firstRevisionIssueId` (RevisionIssueId, get/set) — The identifier of the first issue in which the given change is issued.
- `isArchived` (boolean, get) — Property `isArchived` of type `boolean` (required).
- `customSchemeData` (RevisionCustomSchemeData, get/set) — Property `customSchemeData` of type `RevisionCustomSchemeData` (optional).

## RevisionChangesArrayItem (class)

A wrapper containing an array of revision changes

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#RevisionChangesArrayItem)

### Properties
- `revisionChanges` (RevisionChange[], get/set) — Property `revisionChanges` of type `RevisionChange[]` (optional).

## RevisionChangesOfEntities (class)

Shared schema type `RevisionChangesOfEntities` defined in the Tapir / Graphisoft JSON Interface.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#RevisionChangesOfEntities)

## RevisionCustomSchemeData (class)

Shared schema type `RevisionCustomSchemeData` defined in the Tapir / Graphisoft JSON Interface.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#RevisionCustomSchemeData)

### Properties
- `Items` (object[], get) — Array element type.

## RevisionIssue (class)

Shared schema type `RevisionIssue` defined in the Tapir / Graphisoft JSON Interface.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#RevisionIssue)

### Properties
- `revisionIssueId` (RevisionIssueId, get) — Property `revisionIssueId` of type `RevisionIssueId` (required).
- `id` (string, get) — Property `id` of type `string` (required).
- `description` (string, get) — Property `description` of type `string` (required).
- `issueTime` (string, get) — Property `issueTime` of type `string` (required).
- `issuedByUser` (string, get) — Property `issuedByUser` of type `string` (required).
- `overrideRevisionIDOfAllIncludedLayouts` (boolean, get) — Property `overrideRevisionIDOfAllIncludedLayouts` of type `boolean` (required).
- `createNewRevisionInAllIncludedLayouts` (boolean, get) — Property `createNewRevisionInAllIncludedLayouts` of type `boolean` (required).
- `markersVisibleSinceIndex` (integer, get/set) — Property `markersVisibleSinceIndex` of type `integer` (optional).
- `isIssued` (boolean, get) — Property `isIssued` of type `boolean` (required).
- `documentRevisions` (DocumentRevisionReferences, get/set) — Property `documentRevisions` of type `DocumentRevisionReferences` (optional).
- `customSchemeData` (RevisionCustomSchemeData, get/set) — Property `customSchemeData` of type `RevisionCustomSchemeData` (optional).

## RevisionIssueId (class)

The identifier of a revision issue.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#RevisionIssueId)

### Properties
- `guid` (Guid, get) — Property `guid` of type `Guid` (required).

## SetGDLParameterArray (class)

The list of GDL parameters.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#SetGDLParameterArray)

### Properties
- `Items` (SetGDLParameterDetails[], get) — Array element type.

## SetGDLParameterByIndexDetails (class)

Details of a GDL parameter.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#SetGDLParameterByIndexDetails)

### Properties
- `index` (integer, get) — The index of the parameter.
- `value` (object, get) — The value of the parameter.

## SetGDLParameterByNameDetails (class)

Details of a GDL parameter.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#SetGDLParameterByNameDetails)

### Properties
- `name` (string, get) — The name of the parameter.
- `value` (object, get) — The value of the parameter.

## SetGDLParameterDetails (class)

Shared schema type `SetGDLParameterDetails` defined in the Tapir / Graphisoft JSON Interface.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#SetGDLParameterDetails)

## SlabDetails (class)

Shared schema type `SlabDetails` defined in the Tapir / Graphisoft JSON Interface.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#SlabDetails)

### Properties
- `thickness` (number, get) — Thickness of the slab.
- `level` (number, get) — Distance of the reference level of the slab from the floor level.
- `offsetFromTop` (number, get) — Vertical distance between the reference level and the top of the slab.
- `zCoordinate` (number, get) — Property `zCoordinate` of type `number` (required).
- `polygonOutline` (Coordinate2D[], get) — Polygon outline of the slab.
- `polygonArcs` (PolyArc[], get/set) — Polygon outline arcs of the slab.
- `holes` (Holes2D, get) — Property `holes` of type `Holes2D` (required).
- `structureType` (string, get/set) — Property `structureType` of type `string` (optional).
- `buildingMaterialId` (AttributeId, get/set) — Property `buildingMaterialId` of type `AttributeId` (optional).
- `compositeId` (AttributeId, get/set) — Property `compositeId` of type `AttributeId` (optional).

## StoriesParameters (class)

A list of project stories, each with their complete parameters.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#StoriesParameters)

### Properties
- `Items` (StoryParameters[], get) — Array element type.

## StoriesSettings (class)

A list of story settings, used as input for creating or modifying multiple stories.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#StoriesSettings)

### Properties
- `Items` (StorySettings[], get) — Array element type.

## StoryParameters (class)

Represents all parameters of a single project story, including its unique identifiers. Used in API responses.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#StoryParameters)

### Properties
- `index` (integer, get) — The story index.
- `floorId` (integer, get) — Unique ID of the story.
- `dispOnSections` (boolean, get) — Story level lines should appear on sections and elevations.
- `level` (number, get) — The story level.
- `name` (string, get) — The name of the story.

## StorySettings (class)

Contains the configurable settings for creating or modifying a story. Used as input in API requests.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#StorySettings)

### Properties
- `dispOnSections` (boolean, get) — Story level lines should appear on sections and elevations.
- `level` (number, get) — The story level.
- `name` (string, get) — The name of the story.

## SuccessfulExecutionResult (class)

The result of a successful execution.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#SuccessfulExecutionResult)

### Properties
- `success` (boolean, get) — Property `success` of type `boolean` (required).

## SurfaceType (enum)

The type of a surface material.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#SurfaceType)

### Values
- `General` = `General`
- `Simple` = `Simple`
- `Matte` = `Matte`
- `Metal` = `Metal`
- `Plastic` = `Plastic`
- `Glass` = `Glass`
- `Glowing` = `Glowing`
- `Constant` = `Constant`

## Texture (class)

Texture parameters

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#Texture)

### Properties
- `name` (string, get) — The filename of the texture in the library (without extension).
- `rotationAngle` (number, get/set) — Rotation angle in radians.
- `xSize` (number, get/set) — X size of the picture in model space, by default 1.
- `ySize` (number, get/set) — Y size of the picture in model space, by default 1.
- `FillRectangle` (boolean, get/set) — True, if fit the rectangle with the picture in a central position, using the natural aspect ratio of the picture.
- `FitPicture` (boolean, get/set) — True, if fit the picture in the middle of the rectangle, using the natural aspect ratio of the picture.
- `mirrorX` (boolean, get/set) — True, if the texture is mirrored in X direction.
- `mirrorY` (boolean, get/set) — True, if the texture is mirrored in Y direction.
- `useAlphaChannel` (boolean, get/set) — True, if the alpha channel of the texture is used.
- `alphaChannelChangesTransparency` (boolean, get/set) — True, if the alpha channel of the texture changes the transparency.
- `alphaChannelChangesSurfaceColor` (boolean, get/set) — True, if the alpha channel of the texture changes the surface color.
- `alphaChannelChangesAmbientColor` (boolean, get/set) — True, if the alpha channel of the texture changes the ambient color.
- `alphaChannelChangesSpecularColor` (boolean, get/set) — True, if the alpha channel of the texture changes the specular color.
- `alphaChannelChangesDiffuseColor` (boolean, get/set) — True, if the alpha channel of the texture changes the diffuse color.

## TypeSpecificDetails (class)

Represents the complete type-specific details of an element. Used as output from GET requests

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#TypeSpecificDetails)

## TypeSpecificSettings (class)

Defines the modifiable type-specific settings for an element. Used as input for SET requests.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#TypeSpecificSettings)

## UserUndefinedPropertyValue (class)

A userUndefined value means that there is no actual number/string/etc. value, but the user deliberately set an Undefined value: this is a valid value, too.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#UserUndefinedPropertyValue)

### Properties
- `type` (PropertyDataType, get) — Property `type` of type `PropertyDataType` (required).
- `status` (string, get) — Property `status` of type `string` (required).

## ViewSettings (class)

The settings of a navigator view

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#ViewSettings)

### Properties
- `modelViewOptions` (string, get/set) — The name of the model view options. If empty, the view has custom model view options.
- `layerCombination` (string, get/set) — The name of the layer combination. If empty, the view has custom layer combination.
- `dimensionStyle` (string, get/set) — The name of the dimension style. If empty, the view has custom dimension style.
- `penSetName` (string, get/set) — The name of the pen set. If empty, the view has custom pen set.
- `graphicOverrideCombination` (string, get/set) — The name of the graphic override combination. If empty, the view has custom graphic override combination.
- `drawingScale` (integer, get/set) — The drawing scale stored on the view, if enabled.
- `saveZoom` (boolean, get/set) — Whether the zoom box is stored in the view.
- `ignoreSavedZoom` (boolean, get/set) — Whether changing to the view should ignore its stored zoom.
- `zoom` (object, get/set) — Stored zoom box in model coordinates. Used only when saveZoom is true.

## ViewSettingsOrError (class)

Shared schema type `ViewSettingsOrError` defined in the Tapir / Graphisoft JSON Interface.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#ViewSettingsOrError)

## ViewTransformations (class)

Shared schema type `ViewTransformations` defined in the Tapir / Graphisoft JSON Interface.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#ViewTransformations)

### Properties
- `zoom` (object, get) — The actual zoom parameters, rectangular region of the model.
- `rotation` (number, get) — The orientation in radian.

## ViewTransformationsOrError (class)

Shared schema type `ViewTransformationsOrError` defined in the Tapir / Graphisoft JSON Interface.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#ViewTransformationsOrError)

## VolumeType (enum)

The type of the volume measurement unit.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#VolumeType)

### Values
- `CubicMeter` = `CubicMeter`
- `Liter` = `Liter`
- `CubicCentimeter` = `CubicCentimeter`
- `CubicMillimeter` = `CubicMillimeter`
- `CubicFoot` = `CubicFoot`
- `CubicInch` = `CubicInch`
- `CubicYard` = `CubicYard`
- `Gallon` = `Gallon`

## WallDetails (class)

Shared schema type `WallDetails` defined in the Tapir / Graphisoft JSON Interface.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#WallDetails)

### Properties
- `geometryType` (string, get) — Property `geometryType` of type `string` (required).
- `begCoordinate` (Coordinate2D, get) — Property `begCoordinate` of type `Coordinate2D` (required).
- `endCoordinate` (Coordinate2D, get) — Property `endCoordinate` of type `Coordinate2D` (required).
- `zCoordinate` (number, get) — Property `zCoordinate` of type `number` (required).
- `height` (number, get) — height relative to bottom
- `bottomOffset` (number, get) — base level of the wall relative to the floor level
- `offset` (number, get) — wall's base line's offset from ref. line
- `arcAngle` (number, get/set) — The arc angle of the curved wall in radians.
- `begThickness` (number, get/set) — Thickness at the beginning in case of trapezoid wall
- `endThickness` (number, get/set) — Thickness at the end in case of trapezoid wall
- `polygonOutline` (Coordinate2D[], get/set) — Polygon outline in case of polygonal wall
- `polygonArcs` (PolyArc[], get/set) — Polygon arcs in case of polygonal wall
- `structureType` (string, get/set) — Property `structureType` of type `string` (optional).
- `buildingMaterialId` (AttributeId, get/set) — Property `buildingMaterialId` of type `AttributeId` (optional).
- `compositeId` (AttributeId, get/set) — Property `compositeId` of type `AttributeId` (optional).
- `profileId` (AttributeId, get/set) — Property `profileId` of type `AttributeId` (optional).

## WallSettings (class)

Settings for modifying a wall.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#WallSettings)

### Properties
- `begCoordinate` (Coordinate2D, get/set) — Property `begCoordinate` of type `Coordinate2D` (optional).
- `endCoordinate` (Coordinate2D, get/set) — Property `endCoordinate` of type `Coordinate2D` (optional).
- `height` (number, get/set) — height relative to bottom
- `bottomOffset` (number, get/set) — base level of the wall relative to the floor level
- `offset` (number, get/set) — wall's base line's offset from ref. line
- `begThickness` (number, get/set) — Thickness at the beginning in case of trapezoid wall
- `endThickness` (number, get/set) — Thickness at the end in case of trapezoid wall

## WindowType (enum)

The type of a window.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#WindowType)

### Values
- `FloorPlan` = `FloorPlan`
- `Section` = `Section`
- `Details` = `Details`
- `3DModel` = `3DModel`
- `Layout` = `Layout`
- `Drawing` = `Drawing`
- `CustomText` = `CustomText`
- `CustomDraw` = `CustomDraw`
- `MasterLayout` = `MasterLayout`
- `Elevation` = `Elevation`
- `InteriorElevation` = `InteriorElevation`
- `Worksheet` = `Worksheet`
- `Report` = `Report`
- `3DDocument` = `3DDocument`
- `External3D` = `External3D`
- `Movie3D` = `Movie3D`
- `MovieRendering` = `MovieRendering`
- `Rendering` = `Rendering`
- `ModelCompare` = `ModelCompare`
- `Interactive Schedule` = `Interactive Schedule`
- `Unknown` = `Unknown`

## ZoneBoundariesOrError (class)

The response of the zone boundaries command or an error.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#ZoneBoundariesOrError)

## ZoneBoundariesWrapper (class)

Shared schema type `ZoneBoundariesWrapper` defined in the Tapir / Graphisoft JSON Interface.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#ZoneBoundariesWrapper)

### Properties
- `zoneBoundaries` (object[], get) — Property `zoneBoundaries` of type `object[]` (required).

## ZoneCreationGeometry (class)

Defines the geometry of a zone. Used as input for creating zones.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#ZoneCreationGeometry)

## ZoneDetails (class)

Shared schema type `ZoneDetails` defined in the Tapir / Graphisoft JSON Interface.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#ZoneDetails)

### Properties
- `name` (string, get) — Name of the zone.
- `numberStr` (string, get) — Zone number.
- `categoryAttributeId` (AttributeId, get) — The identifier of the zone category attribute.
- `stampPosition` (Coordinate2D, get) — Position of the origin of the zone stamp.
- `isManual` (boolean, get) — Is the coordinates of the zone manually placed?
- `polygonCoordinates` (Coordinate2D[], get) — The 2D coordinates of the edge of the zone.
- `polygonArcs` (PolyArc[], get/set) — Polygon outline arcs of the zone.
- `holes` (Holes2D, get/set) — Property `holes` of type `Holes2D` (optional).
- `zCoordinate` (number, get) — Property `zCoordinate` of type `number` (required).

