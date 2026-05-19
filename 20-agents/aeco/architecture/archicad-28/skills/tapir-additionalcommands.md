---
name: archicad-tapir-additionalcommands
description: This skill encodes the archicad 28.0 surface of the Tapir.AdditionalCommands namespace — 14 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: ApplicationCommands, ProjectCommands, ElementCommands, ElementgroupingCommands, FavoritesCommands, PropertyCommands, AttributeCommands, LibraryCommands, and 6 more types.
---

# Tapir.AdditionalCommands

Auto-generated from vendor docs for archicad 28.0. 14 types in this namespace.

## ApplicationCommands (static-class)

Tapir Add-On 'Application Commands' command group — 5 JSON-RPC commands.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/)

### Methods
#### `ChangeWindow(windowType: WindowType, databaseId?: DatabaseId): ExecutionResult`

Changes the current (active) window to the given window.

**Parameters:**
- `windowType` (WindowType)
- `databaseId` (DatabaseId)

**Returns:** `ExecutionResult` — Returns a `ExecutionResult`.

[Docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#ChangeWindow)

#### `GetAddOnVersion(): object`

Retrieves the version of the Tapir Additional JSON Commands Add-On.

**Returns:** `object` — Returns an object with properties: version.

[Docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#GetAddOnVersion)

#### `GetArchicadLocation(): object`

Retrieves the location of the currently running Archicad executable.

**Returns:** `object` — Returns an object with properties: archicadLocation.

[Docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#GetArchicadLocation)

#### `GetCurrentWindowType(): object`

Returns the type of the current (active) window.

**Returns:** `object` — Returns an object with properties: currentWindowType.

[Docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#GetCurrentWindowType)

#### `QuitArchicad(): ExecutionResult`

Performs a quit operation on the currently running Archicad instance.

**Returns:** `ExecutionResult` — Returns a `ExecutionResult`.

[Docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#QuitArchicad)

## AttributeCommands (static-class)

Tapir Add-On 'Attribute Commands' command group — 8 JSON-RPC commands.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/)

### Methods
#### `CreateBuildingMaterials(buildingMaterialDataArray: object[], overwriteExisting?: boolean): object`

Creates or overwrites Building Material attributes based on the given parameters.

**Parameters:**
- `buildingMaterialDataArray` (object[]) — Array of data to create new Building Materials.
- `overwriteExisting` (boolean) — Overwrite the Building Material if exists with the same name, or if index is given with the same index. The default is false.

**Returns:** `object` — Returns an object with properties: attributeIds.

[Docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#CreateBuildingMaterials)

#### `CreateComposites(compositeDataArray: object[], overwriteExisting?: boolean): object`

Creates or overwrites Composite attributes based on the given parameters.

**Parameters:**
- `compositeDataArray` (object[]) — Array of data to create Composites.
- `overwriteExisting` (boolean) — Overwrite the Composite if exists with the same name, or if index is given with the same index. The default is false.

**Returns:** `object` — Returns an object with properties: attributeIds.

[Docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#CreateComposites)

#### `CreateLayerCombinations(layerCombinationDataArray: object[], overwriteExisting?: boolean): object`

Creates or overwrites Layer Combination attributes based on the given parameters.

**Parameters:**
- `layerCombinationDataArray` (object[]) — Array of data to create new Layer Combinations.
- `overwriteExisting` (boolean) — Overwrite the Layer Combination if exists with the same guid/index/name. The default is false.

**Returns:** `object` — Returns an object with properties: attributeIds.

[Docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#CreateLayerCombinations)

#### `CreateLayers(layerDataArray: object[], overwriteExisting?: boolean): object`

Creates or overwrites Layer attributes based on the given parameters.

**Parameters:**
- `layerDataArray` (object[]) — Array of data to create new Layers.
- `overwriteExisting` (boolean) — Overwrite the Layer if exists with the same name, or if index is given with the same index. The default is false.

**Returns:** `object` — Returns an object with properties: attributeIds.

[Docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#CreateLayers)

#### `CreateSurfaces(surfaceDataArray: object[], overwriteExisting?: boolean): object`

Creates or overwrites Surface attributes based on the given parameters.

**Parameters:**
- `surfaceDataArray` (object[]) — Array of data to create new surfaces.
- `overwriteExisting` (boolean) — Overwrite the Surface if exists with the same name, or if index is given with the same index. The default is false.

**Returns:** `object` — Returns an object with properties: attributeIds.

[Docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#CreateSurfaces)

#### `GetAttributesByType(attributeType: AttributeType): AttributeHeadersOrError`

Returns the details of every attribute of the given type.

**Parameters:**
- `attributeType` (AttributeType)

**Returns:** `AttributeHeadersOrError` — Returns a `AttributeHeadersOrError`.

[Docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#GetAttributesByType)

#### `GetBuildingMaterialPhysicalProperties(attributeIds: AttributeIds): object`

Retrieves the physical properties of the given Building Materials.

**Parameters:**
- `attributeIds` (AttributeIds)

**Returns:** `object` — Returns an object with properties: properties.

[Docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#GetBuildingMaterialPhysicalProperties)

#### `GetLayerCombinations(attributes: AttributeIds): object`

Returns the details of layer combination attributes.

**Parameters:**
- `attributes` (AttributeIds)

**Returns:** `object` — Returns an object with properties: layerCombinations.

[Docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#GetLayerCombinations)

## DesignOptionsCommands (static-class)

Tapir Add-On 'Design Options Commands' command group — 3 JSON-RPC commands.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/)

### Methods
#### `GetDesignOptionCombinations(): object`

Retrieves information about existing design option combinations.

**Returns:** `object` — Returns an object with properties: designOptionCombinations.

[Docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#GetDesignOptionCombinations)

#### `GetDesignOptionSets(): object`

Retrieves information about existing design option sets. Available from Archicad 29.

**Returns:** `object` — Returns an object with properties: designOptionSets.

[Docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#GetDesignOptionSets)

#### `GetDesignOptions(): object`

Retrieves information about existing design options. Available from Archicad 29.

**Returns:** `object` — Returns an object with properties: designOptions.

[Docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#GetDesignOptions)

## DeveloperCommands (static-class)

Tapir Add-On 'Developer Commands' command group — 1 JSON-RPC commands.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/)

### Methods
#### `GenerateDocumentation(destinationFolder: string): ExecutionResult`

Generates files for the documentation. Used by Tapir developers only.

**Parameters:**
- `destinationFolder` (string) — Destination folder for the generated documentation files.

**Returns:** `ExecutionResult` — Returns a `ExecutionResult`.

[Docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#GenerateDocumentation)

## ElementCommands (static-class)

Tapir Add-On 'Element Commands' command group — 48 JSON-RPC commands.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/)

### Methods
#### `ChangeSelectionOfElements(addElementsToSelection?: Elements, removeElementsFromSelection?: Elements): object`

Adds/removes a number of elements to/from the current selection.

**Parameters:**
- `addElementsToSelection` (Elements)
- `removeElementsFromSelection` (Elements)

**Returns:** `object` — Returns an object with properties: executionResultsOfAddToSelection, executionResultsOfRemoveFromSelection.

[Docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#ChangeSelectionOfElements)

#### `CreateAssociativeDimensions(dimensionsData: object[]): object`

Creates associative linear dimensions from explicit witness point references.

**Parameters:**
- `dimensionsData` (object[])

**Returns:** `object` — Returns an object with properties: elements.

[Docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#CreateAssociativeDimensions)

#### `CreateAssociativeDimensionsOnSection(dimensionsData: object[]): object`

Creates associative linear dimensions on section elements using common wall, slab, beam, column and opening presets.

**Parameters:**
- `dimensionsData` (object[])

**Returns:** `object` — Returns an object with properties: elements.

[Docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#CreateAssociativeDimensionsOnSection)

#### `CreateBeams(beamsData: object[]): object`

Creates Beam elements based on the given parameters.

**Parameters:**
- `beamsData` (object[])

**Returns:** `object` — Returns an object with properties: elements.

[Docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#CreateBeams)

#### `CreateColumns(columnsData: object[]): object`

Creates Column elements based on the given parameters.

**Parameters:**
- `columnsData` (object[]) — Array of data to create Columns.

**Returns:** `object` — Returns an object with properties: elements.

[Docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#CreateColumns)

#### `CreateDoors(doorsData: object[]): object`

Creates Door elements in host walls based on the given parameters.

**Parameters:**
- `doorsData` (object[])

**Returns:** `object` — Returns an object with properties: elements.

[Docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#CreateDoors)

#### `CreateLabels(labelsData: object[]): object`

Creates Label elements based on the given parameters.

**Parameters:**
- `labelsData` (object[]) — Array of data to create Labels.

**Returns:** `object` — Returns an object with properties: elements.

[Docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#CreateLabels)

#### `CreateMeshes(meshesData: object[]): object`

Creates Mesh elements based on the given parameters.

**Parameters:**
- `meshesData` (object[]) — Array of data to create Meshes.

**Returns:** `object` — Returns an object with properties: elements.

[Docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#CreateMeshes)

#### `CreateMorphs(morphsData: object[]): object`

Creates Morph elements from simple box definitions.

**Parameters:**
- `morphsData` (object[])

**Returns:** `object` — Returns an object with properties: elements.

[Docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#CreateMorphs)

#### `CreateObjects(objectsData: object[]): object`

Creates Object elements based on the given parameters.

**Parameters:**
- `objectsData` (object[]) — Array of data to create Objects.

**Returns:** `object` — Returns an object with properties: elements.

[Docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#CreateObjects)

#### `CreateOpenings(openingsData: object[]): object`

Creates Opening elements in the given host elements.

**Parameters:**
- `openingsData` (object[])

**Returns:** `object` — Returns an object with properties: elements.

[Docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#CreateOpenings)

#### `CreatePolylines(polylinesData: object[]): object`

Creates Polyline elements based on the given parameters.

**Parameters:**
- `polylinesData` (object[]) — Array of data to create Polylines.

**Returns:** `object` — Returns an object with properties: elements.

[Docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#CreatePolylines)

#### `CreateRoofs(roofsData: object[]): object`

Creates multi-plane Roof elements based on footprint, level and roof profile data.

**Parameters:**
- `roofsData` (object[])

**Returns:** `object` — Returns an object with properties: elements.

[Docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#CreateRoofs)

#### `CreateSlabs(slabsData: object[]): object`

Creates Slab elements based on the given parameters.

**Parameters:**
- `slabsData` (object[]) — Array of data to create Slabs.

**Returns:** `object` — Returns an object with properties: elements.

[Docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#CreateSlabs)

#### `CreateWallThicknessDimensions(dimensionsData: object[]): object`

Creates associative wall thickness dimensions for the given walls.

**Parameters:**
- `dimensionsData` (object[])

**Returns:** `object` — Returns an object with properties: elements.

[Docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#CreateWallThicknessDimensions)

#### `CreateWalls(wallsData: object[]): object`

Creates Wall elements based on the given parameters.

**Parameters:**
- `wallsData` (object[])

**Returns:** `object` — Returns an object with properties: elements.

[Docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#CreateWalls)

#### `CreateWindows(windowsData: object[]): object`

Creates Window elements in host walls based on the given parameters.

**Parameters:**
- `windowsData` (object[])

**Returns:** `object` — Returns an object with properties: elements.

[Docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#CreateWindows)

#### `CreateZones(zonesData: object[]): object`

Creates Zone elements based on the given parameters.

**Parameters:**
- `zonesData` (object[]) — Array of data to create Zones.

**Returns:** `object` — Returns an object with properties: elements.

[Docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#CreateZones)

#### `DeleteElements(elements: Elements): ExecutionResult`

Deletes elements.

**Parameters:**
- `elements` (Elements)

**Returns:** `ExecutionResult` — Returns a `ExecutionResult`.

[Docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#DeleteElements)

#### `FilterElements(elements: Elements, filters?: ElementFilter[]): object`

Tests an elements by the given criterias.

**Parameters:**
- `elements` (Elements)
- `filters` (ElementFilter[])

**Returns:** `object` — Returns an object with properties: elements.

[Docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#FilterElements)

#### `Get3DBoundingBoxes(elements: Elements): object`

Get the 3D bounding box of elements. The bounding box is calculated from the global origin in the 3D view. The output is the array of the bounding boxes respective to the input array of elements.

**Parameters:**
- `elements` (Elements)

**Returns:** `object` — Returns an object with properties: boundingBoxes3D.

[Docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#Get3DBoundingBoxes)

#### `GetAllElements(filters?: ElementFilter[], databases?: Databases): ElementsWithExecutionResultsOrError`

Returns the identifier of all elements on the plan. Use the optional filter parameter for filtering.

**Parameters:**
- `filters` (ElementFilter[])
- `databases` (Databases)

**Returns:** `ElementsWithExecutionResultsOrError` — Returns a `ElementsWithExecutionResultsOrError`.

[Docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#GetAllElements)

#### `GetClassificationsOfElements(elements: Elements, classificationSystemIds: ClassificationSystemIds): object`

Returns the classification of the given elements in the given classification systems. It works for subelements of hierarchal elements also.

**Parameters:**
- `elements` (Elements)
- `classificationSystemIds` (ClassificationSystemIds)

**Returns:** `object` — Returns an object with properties: elementClassifications.

[Docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#GetClassificationsOfElements)

#### `GetCollisions(elementsGroup1: Elements, elementsGroup2: Elements, settings?: object): object`

Detect collisions between the given two groups of elements.

**Parameters:**
- `elementsGroup1` (Elements)
- `elementsGroup2` (Elements)
- `settings` (object)

**Returns:** `object` — Returns an object with properties: collisions.

[Docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#GetCollisions)

#### `GetConnectedElements(elements: Elements, connectedElementType: ElementType): ConnectedElementsOrError`

Gets connected elements of the given elements.

**Parameters:**
- `elements` (Elements)
- `connectedElementType` (ElementType)

**Returns:** `ConnectedElementsOrError` — Returns a `ConnectedElementsOrError`.

[Docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#GetConnectedElements)

#### `GetDetailsOfElements(elements: Elements): object`

Gets the details of the given elements (geometry parameters etc).

**Parameters:**
- `elements` (Elements)

**Returns:** `object` — Returns an object with properties: detailsOfElements.

[Docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#GetDetailsOfElements)

#### `GetElementPreviewImage(elementId: ElementId, imageType?: string, format?: string, width?: integer, height?: integer): object`

Returns the preview image of the given element.

**Parameters:**
- `elementId` (ElementId)
- `imageType` (string) — The type of the preview image. Default is 3D.
- `format` (string) — The image format. Default is png.
- `width` (integer) — The width of the preview image in pixels. Default is 128.
- `height` (integer) — The height of the preview image in pixels. Default is 128.

**Returns:** `object` — Returns an object with properties: previewImage.

[Docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#GetElementPreviewImage)

#### `GetElementsByType(elementType: ElementType, filters?: ElementFilter[], databases?: Databases): ElementsWithExecutionResultsOrError`

Returns the identifier of every element of the given type on the plan. It works for any type. Use the optional filter parameter for filtering.

**Parameters:**
- `elementType` (ElementType)
- `filters` (ElementFilter[])
- `databases` (Databases)

**Returns:** `ElementsWithExecutionResultsOrError` — Returns a `ElementsWithExecutionResultsOrError`.

[Docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#GetElementsByType)

#### `GetGDLParametersOfElements(elements: Elements): object`

Gets all the GDL parameters (name, type, value) of the given elements.

**Parameters:**
- `elements` (Elements)

**Returns:** `object` — Returns an object with properties: gdlParametersOfElements.

[Docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#GetGDLParametersOfElements)

#### `GetRoomImage(zoneId: ElementId, format?: string, width?: integer, height?: integer, offset?: number, scale?: number, backgroundColor?: ColorRGB): object`

Returns the room image of the given zone.

**Parameters:**
- `zoneId` (ElementId)
- `format` (string) — The image format. Default is png.
- `width` (integer) — The width of the preview image in pixels. Default is 256.
- `height` (integer) — The height of the preview image in pixels. Default is 256.
- `offset` (number) — Offset of the clip polygon from the edge of the zone. Default is 0.001.
- `scale` (number) — Scale of the view (e.g. 0.005 for 1:200). Default is 0.005.
- `backgroundColor` (ColorRGB) — Background color of the generated image. Default is white (1.0, 1.0, 1.0).

**Returns:** `object` — Returns an object with properties: roomImage.

[Docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#GetRoomImage)

#### `GetSelectedElements(): object`

Gets the list of the currently selected elements.

**Returns:** `object` — Returns an object with properties: elements.

[Docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#GetSelectedElements)

#### `GetSubelementsOfHierarchicalElements(elements: Elements): object`

Gets the subelements of the given hierarchical elements.

**Parameters:**
- `elements` (Elements)

**Returns:** `object` — Returns an object with properties: subelements.

[Docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#GetSubelementsOfHierarchicalElements)

#### `GetZoneBoundaries(zoneElementId: ElementId): ZoneBoundariesOrError`

Gets the boundaries of the given Zone (connected elements, neighbour zones, etc.).

**Parameters:**
- `zoneElementId` (ElementId)

**Returns:** `ZoneBoundariesOrError` — Returns a `ZoneBoundariesOrError`.

[Docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#GetZoneBoundaries)

#### `HighlightElements(elements: Elements, highlightedColors: integer[][], wireframe3D?: boolean, nonHighlightedColor?: integer[]): ExecutionResult`

Highlights the elements given in the elements array. In case of empty elements array removes all previously set highlights.

**Parameters:**
- `elements` (Elements)
- `highlightedColors` (integer[][]) — A list of colors to highlight elements.
- `wireframe3D` (boolean) — Optional parameter. Switch non highlighted elements in the 3D window to wireframe.
- `nonHighlightedColor` (integer[]) — Optional parameter. Color of the non highlighted elements as an [r, g, b, a] array. Each component must be in the 0-255 range.

**Returns:** `ExecutionResult` — Returns a `ExecutionResult`.

[Docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#HighlightElements)

#### `ModifyBeams(beamsWithDetails: object[]): object`

Modifies Beam elements based on the given parameters.

**Parameters:**
- `beamsWithDetails` (object[])

**Returns:** `object` — Returns an object with properties: executionResults.

[Docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#ModifyBeams)

#### `ModifyColumns(columnsWithDetails: object[]): object`

Modifies Column elements based on the given parameters.

**Parameters:**
- `columnsWithDetails` (object[])

**Returns:** `object` — Returns an object with properties: executionResults.

[Docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#ModifyColumns)

#### `ModifyDoors(doorsWithDetails: object[]): object`

Modifies Door elements based on the given parameters.

**Parameters:**
- `doorsWithDetails` (object[])

**Returns:** `object` — Returns an object with properties: executionResults.

[Docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#ModifyDoors)

#### `ModifyMorphs(morphsWithDetails: object[]): object`

Modifies Morph elements based on the given parameters.

**Parameters:**
- `morphsWithDetails` (object[])

**Returns:** `object` — Returns an object with properties: executionResults.

[Docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#ModifyMorphs)

#### `ModifyRoofs(roofsWithDetails: object[]): object`

Modifies multi-plane Roof elements based on the given parameters.

**Parameters:**
- `roofsWithDetails` (object[])

**Returns:** `object` — Returns an object with properties: executionResults.

[Docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#ModifyRoofs)

#### `ModifySlabs(slabsWithDetails: object[]): object`

Modifies Slab elements based on the given parameters.

**Parameters:**
- `slabsWithDetails` (object[])

**Returns:** `object` — Returns an object with properties: executionResults.

[Docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#ModifySlabs)

#### `ModifyWalls(wallsWithDetails: object[]): object`

Modifies Wall elements based on the given parameters.

**Parameters:**
- `wallsWithDetails` (object[])

**Returns:** `object` — Returns an object with properties: executionResults.

[Docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#ModifyWalls)

#### `ModifyWindows(windowsWithDetails: object[]): object`

Modifies Window elements based on the given parameters.

**Parameters:**
- `windowsWithDetails` (object[])

**Returns:** `object` — Returns an object with properties: executionResults.

[Docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#ModifyWindows)

#### `MoveElements(elementsWithMoveVectors: object[]): object`

Moves elements with a given vector.

**Parameters:**
- `elementsWithMoveVectors` (object[]) — The elements with move vector pairs.

**Returns:** `object` — Returns an object with properties: executionResults.

[Docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#MoveElements)

#### `RemoveElementNotificationClient(host?: string, port: integer): ExecutionResult`

Removes an element notification client.

**Parameters:**
- `host` (string) — The host address of the notification client. If not provided, localhost is used.
- `port` (integer) — The port number of the notification client.

**Returns:** `ExecutionResult` — Returns a `ExecutionResult`.

[Docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#RemoveElementNotificationClient)

#### `SetClassificationsOfElements(elementClassifications: ElementClassifications): object`

Sets the classifications of elements. In order to set the classification of an element to unclassified, omit the classificationItemId field. It works for subelements of hierarchal elements also.

**Parameters:**
- `elementClassifications` (ElementClassifications)

**Returns:** `object` — Returns an object with properties: executionResults.

[Docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#SetClassificationsOfElements)

#### `SetDetailsOfElements(elementsWithDetails: object[]): object`

Sets the details of the given elements (floor, layer, order etc).

**Parameters:**
- `elementsWithDetails` (object[]) — The elements with parameters.

**Returns:** `object` — Returns an object with properties: executionResults.

[Docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#SetDetailsOfElements)

#### `SetElementNotificationClient(host?: string, port: integer, notifyOnNewElement?: boolean, notifyOnModificationOfAnElement?: boolean, notifyOnReservationChanges?: boolean): ExecutionResult`

Sets up a new notification client to receive element events.

**Parameters:**
- `host` (string) — The host address of the notification client. If not provided, localhost is used.
- `port` (integer) — The port number of the notification client.
- `notifyOnNewElement` (boolean) — Notify on creation of a new element. Optional parameter, by default true.
- `notifyOnModificationOfAnElement` (boolean) — Notify on modification/deletion of an element. Optional parameter, by default true.
- `notifyOnReservationChanges` (boolean) — Notify on reservation changes of an element. Optional parameter, by default true.

**Returns:** `ExecutionResult` — Returns a `ExecutionResult`.

[Docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#SetElementNotificationClient)

#### `SetGDLParametersOfElements(elementsWithGDLParameters: object[]): object`

Sets the given GDL parameters of the given elements.

**Parameters:**
- `elementsWithGDLParameters` (object[]) — The elements with GDL parameters dictionary pairs.

**Returns:** `object` — Returns an object with properties: executionResults.

[Docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#SetGDLParametersOfElements)

## ElementgroupingCommands (static-class)

Tapir Add-On 'Element grouping Commands' command group — 1 JSON-RPC commands.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/)

### Methods
#### `CreateGroups(elementGroups: ElementGroupParameters[]): object`

Creates groups of the passed elements

**Parameters:**
- `elementGroups` (ElementGroupParameters[]) — A list of element groups to create.

**Returns:** `object` — Returns an object with properties: groupGuids.

[Docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#CreateGroups)

## FavoritesCommands (static-class)

Tapir Add-On 'Favorites Commands' command group — 4 JSON-RPC commands.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/)

### Methods
#### `ApplyFavoritesToElementDefaults(favorites: Favorites): object`

Apply the given favorites to element defaults.

**Parameters:**
- `favorites` (Favorites)

**Returns:** `object` — Returns an object with properties: executionResults.

[Docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#ApplyFavoritesToElementDefaults)

#### `CreateFavoritesFromElements(favoritesFromElements: object[]): object`

Create favorites from the given elements.

**Parameters:**
- `favoritesFromElements` (object[])

**Returns:** `object` — Returns an object with properties: executionResults.

[Docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#CreateFavoritesFromElements)

#### `GetFavoritePreviewImage(favorite: string, imageType?: string, format?: string, width?: integer, height?: integer): object`

Returns the preview image of the given favorite.

**Parameters:**
- `favorite` (string) — The name of the favorite.
- `imageType` (string) — The type of the preview image. Default is 3D.
- `format` (string) — The image format. Default is png.
- `width` (integer) — The width of the preview image in pixels. Default is 128.
- `height` (integer) — The height of the preview image in pixels. Default is 128.

**Returns:** `object` — Returns an object with properties: previewImage.

[Docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#GetFavoritePreviewImage)

#### `GetFavoritesByType(elementType: ElementType): FavoritesOrError`

Returns a list of the names of all favorites with the given element type

**Parameters:**
- `elementType` (ElementType)

**Returns:** `FavoritesOrError` — Returns a `FavoritesOrError`.

[Docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#GetFavoritesByType)

## IssueManagementCommands (static-class)

Tapir Add-On 'Issue Management Commands' command group — 10 JSON-RPC commands.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/)

### Methods
#### `AddCommentToIssue(issueId: IssueId, author?: string, status?: IssueCommentStatus, text: string): ExecutionResult`

Adds a new comment to the specified issue.

**Parameters:**
- `issueId` (IssueId)
- `author` (string) — The author of the new comment.
- `status` (IssueCommentStatus)
- `text` (string) — Comment text to add.

**Returns:** `ExecutionResult` — Returns a `ExecutionResult`.

[Docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#AddCommentToIssue)

#### `AttachElementsToIssue(issueId: IssueId, elements: Elements, type: IssueElementType): ExecutionResult`

Attaches elements to the specified issue.

**Parameters:**
- `issueId` (IssueId)
- `elements` (Elements)
- `type` (IssueElementType)

**Returns:** `ExecutionResult` — Returns a `ExecutionResult`.

[Docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#AttachElementsToIssue)

#### `CreateIssue(name: string, parentIssueId?: IssueId, tagText?: string): object`

Creates a new issue.

**Parameters:**
- `name` (string) — The name of the issue.
- `parentIssueId` (IssueId)
- `tagText` (string) — Tag text of the issue, optional.

**Returns:** `object` — Returns an object with properties: issueId.

[Docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#CreateIssue)

#### `DeleteIssue(issueId: IssueId, acceptAllElements?: boolean): ExecutionResult`

Deletes the specified issue.

**Parameters:**
- `issueId` (IssueId)
- `acceptAllElements` (boolean) — Accept all creation/deletion/modification of the deleted issue. By default false.

**Returns:** `ExecutionResult` — Returns a `ExecutionResult`.

[Docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#DeleteIssue)

#### `DetachElementsFromIssue(issueId: IssueId, elements: Elements): ExecutionResult`

Detaches elements from the specified issue.

**Parameters:**
- `issueId` (IssueId)
- `elements` (Elements)

**Returns:** `ExecutionResult` — Returns a `ExecutionResult`.

[Docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#DetachElementsFromIssue)

#### `ExportIssuesToBCF(issues?: Issues, exportPath: string, useExternalId: boolean, alignBySurveyPoint: boolean): ExecutionResult`

Exports specified issues to a BCF file.

**Parameters:**
- `issues` (Issues) — Leave it empty to export all issues.
- `exportPath` (string) — The os path to the bcf file, including it's name.
- `useExternalId` (boolean) — Use external IFC ID or Archicad IFC ID as referenced in BCF topics.
- `alignBySurveyPoint` (boolean) — Align BCF views by Archicad Survey Point or Archicad Project Origin.

**Returns:** `ExecutionResult` — Returns a `ExecutionResult`.

[Docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#ExportIssuesToBCF)

#### `GetCommentsFromIssue(issueId: IssueId): object`

Retrieves comments information from the specified issue.

**Parameters:**
- `issueId` (IssueId)

**Returns:** `object` — Returns an object with properties: comments.

[Docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#GetCommentsFromIssue)

#### `GetElementsAttachedToIssue(issueId: IssueId, type: IssueElementType): object`

Retrieves attached elements of the specified issue, filtered by attachment type.

**Parameters:**
- `issueId` (IssueId)
- `type` (IssueElementType)

**Returns:** `object` — Returns an object with properties: elements.

[Docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#GetElementsAttachedToIssue)

#### `GetIssues(): object`

Retrieves information about existing issues.

**Returns:** `object` — Returns an object with properties: issues.

[Docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#GetIssues)

#### `ImportIssuesFromBCF(importPath: string, alignBySurveyPoint: boolean): ExecutionResult`

Imports issues from the specified BCF file.

**Parameters:**
- `importPath` (string) — The os path to the bcf file, including it's name.
- `alignBySurveyPoint` (boolean) — Align BCF views by Archicad Survey Point or Archicad Project Origin.

**Returns:** `ExecutionResult` — Returns a `ExecutionResult`.

[Docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#ImportIssuesFromBCF)

## LibraryCommands (static-class)

Tapir Add-On 'Library Commands' command group — 3 JSON-RPC commands.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/)

### Methods
#### `AddFilesToEmbeddedLibrary(files: LibraryFileAdditions): object`

Adds the given files into the embedded library.

**Parameters:**
- `files` (LibraryFileAdditions)

**Returns:** `object` — Returns an object with properties: executionResults.

[Docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#AddFilesToEmbeddedLibrary)

#### `GetLibraries(): object`

Gets the list of loaded libraries.

**Returns:** `object` — Returns an object with properties: libraries.

[Docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#GetLibraries)

#### `ReloadLibraries(): ExecutionResult`

Executes the reload libraries command.

**Returns:** `ExecutionResult` — Returns a `ExecutionResult`.

[Docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#ReloadLibraries)

## NavigatorCommands (static-class)

Tapir Add-On 'Navigator Commands' command group — 14 JSON-RPC commands.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/)

### Methods
#### `CreateDetails(detailsData: object[]): object`

Creates independent Detail databases.

**Parameters:**
- `detailsData` (object[])

**Returns:** `object` — Returns an object with properties: databases.

[Docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#CreateDetails)

#### `CreateDrawings(drawingsData: object[]): object`

Creates Drawing elements on the specified or active layout from navigator items.

**Parameters:**
- `drawingsData` (object[])

**Returns:** `object` — Returns an object with properties: elements.

[Docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#CreateDrawings)

#### `CreateLayouts(layoutsData: object[]): object`

Creates Layouts and their backing master layouts.

**Parameters:**
- `layoutsData` (object[])

**Returns:** `object` — Returns an object with properties: databases.

[Docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#CreateLayouts)

#### `CreateSubsets(subsetsData: object[]): object`

Creates Layout Book subsets.

**Parameters:**
- `subsetsData` (object[])

**Returns:** `object` — Returns an object with properties: executionResults.

[Docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#CreateSubsets)

#### `CreateWorksheets(worksheetsData: object[]): object`

Creates independent Worksheet databases.

**Parameters:**
- `worksheetsData` (object[])

**Returns:** `object` — Returns an object with properties: databases.

[Docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#CreateWorksheets)

#### `FitInWindow(elements?: Elements): ExecutionResult`

Zooms to the given elements or fits everything in the window.

**Parameters:**
- `elements` (Elements)

**Returns:** `ExecutionResult` — Returns a `ExecutionResult`.

[Docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#FitInWindow)

#### `GetDatabaseIdFromNavigatorItemId(navigatorItemIds: NavigatorItemIds): object`

Gets the ID of the database associated with the supplied navigator item id

**Parameters:**
- `navigatorItemIds` (NavigatorItemIds)

**Returns:** `object` — Returns an object with properties: databases.

[Docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#GetDatabaseIdFromNavigatorItemId)

#### `GetModelViewOptions(): object`

Gets all model view options

**Returns:** `object` — Returns an object with properties: modelViewOptions.

[Docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#GetModelViewOptions)

#### `GetView2DTransformations(databases?: Databases): object`

Get zoom and rotation of 2D views

**Parameters:**
- `databases` (Databases)

**Returns:** `object` — Returns an object with properties: transformations.

[Docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#GetView2DTransformations)

#### `GetViewSettings(navigatorItemIds: NavigatorItemIds): object`

Gets the view settings of navigator items

**Parameters:**
- `navigatorItemIds` (NavigatorItemIds)

**Returns:** `object` — Returns an object with properties: viewSettings.

[Docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#GetViewSettings)

#### `PublishPublisherSet(publisherSetName: string, outputPath?: string, selectedNavigatorItemIds?: NavigatorItemIds): void`

Performs a publish operation on the currently opened project. Only the given publisher set will be published.

**Parameters:**
- `publisherSetName` (string) — The name of the publisher set.
- `outputPath` (string) — Full local or LAN path for publishing. Optional, by default the path set in the settings of the publisher set will be used.
- `selectedNavigatorItemIds` (NavigatorItemIds) — Optional publisher-tree navigator items to publish instead of the whole publisher set.

[Docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#PublishPublisherSet)

#### `Set3DCutPlanes(cutPlanes?: object[]): ExecutionResult`

Sets the 3D cut planes.

**Parameters:**
- `cutPlanes` (object[])

**Returns:** `ExecutionResult` — Returns a `ExecutionResult`.

[Docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#Set3DCutPlanes)

#### `SetViewSettings(navigatorItemIdsWithViewSettings: object[]): object`

Sets the view settings of navigator items

**Parameters:**
- `navigatorItemIdsWithViewSettings` (object[])

**Returns:** `object` — Returns an object with properties: executionResults.

[Docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#SetViewSettings)

#### `UpdateDrawings(elements: Elements): ExecutionResult`

Performs a drawing update on the given elements.

**Parameters:**
- `elements` (Elements)

**Returns:** `ExecutionResult` — Returns a `ExecutionResult`.

[Docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#UpdateDrawings)

## ProjectCommands (static-class)

Tapir Add-On 'Project Commands' command group — 14 JSON-RPC commands.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/)

### Methods
#### `CloseProject(): ExecutionResult`

Closes the currently opened project.

**Returns:** `ExecutionResult` — Returns a `ExecutionResult`.

[Docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#CloseProject)

#### `GetCalculationUnits(): object`

Gets the project calculation units.

**Returns:** `object` — Returns an object with properties: length, area, volume, angle.

[Docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#GetCalculationUnits)

#### `GetGeoLocation(): object`

Gets the project location details.

**Returns:** `object` — Returns an object with properties: projectLocation, surveyPoint.

[Docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#GetGeoLocation)

#### `GetHotlinks(): object`

Gets the file system locations (path) of the hotlink modules. The hotlinks can have tree hierarchy in the project.

**Returns:** `object` — Returns an object with properties: hotlinks.

[Docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#GetHotlinks)

#### `GetProjectInfo(): object`

Retrieves information about the currently loaded project.

**Returns:** `object` — Returns an object with properties: isUntitled, isTeamwork, projectLocation, projectPath, projectName.

[Docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#GetProjectInfo)

#### `GetProjectInfoFields(): object`

Retrieves the names and values of all project info fields.

**Returns:** `object` — Returns an object with properties: fields.

[Docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#GetProjectInfoFields)

#### `GetStories(): object`

Retrieves information about the story sructure of the currently loaded project.

**Returns:** `object` — Returns an object with properties: firstStory, lastStory, actStory, skipNullFloor, stories.

[Docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#GetStories)

#### `IFCFileOperation(method: string, ifcFilePath: string, fileType?: string): ExecutionResult`

Executes an IFC file operation.

**Parameters:**
- `method` (string) — The file operation method to use.
- `ifcFilePath` (string) — The target IFC file to use.
- `fileType` (string) — The type of the IFC file. The default is 'ifc'.

**Returns:** `ExecutionResult` — Returns a `ExecutionResult`.

[Docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#IFCFileOperation)

#### `OpenProject(projectFilePath: string): ExecutionResult`

Opens the given project.

**Parameters:**
- `projectFilePath` (string) — The target project file to open.

**Returns:** `ExecutionResult` — Returns a `ExecutionResult`.

[Docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#OpenProject)

#### `PrintView(grid?: boolean, fixText?: boolean, scale?: integer, printArea?: string): ExecutionResult`

Prints from the current view.

**Parameters:**
- `grid` (boolean) — Print the grid. The default is false.
- `fixText` (boolean) — Use fixed text size. The default is false.
- `scale` (integer) — Print scale. The default is 100.
- `printArea` (string) — The area to print. The default is 'currentView'.

**Returns:** `ExecutionResult` — Returns a `ExecutionResult`.

[Docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#PrintView)

#### `SaveProject(): ExecutionResult`

Saves the currently opened project.

**Returns:** `ExecutionResult` — Returns a `ExecutionResult`.

[Docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#SaveProject)

#### `SetGeoLocation(projectLocation?: object, surveyPoint?: object): ExecutionResult`

Sets the project location details.

**Parameters:**
- `projectLocation` (object)
- `surveyPoint` (object)

**Returns:** `ExecutionResult` — Returns a `ExecutionResult`.

[Docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#SetGeoLocation)

#### `SetProjectInfoField(projectInfoId: string, projectInfoValue: string): void`

Sets the value of a project info field.

**Parameters:**
- `projectInfoId` (string) — The id of the project info field.
- `projectInfoValue` (string) — The new value of the project info field.

[Docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#SetProjectInfoField)

#### `SetStories(stories: StoriesSettings): ExecutionResult`

Sets the story sructure of the currently loaded project.

**Parameters:**
- `stories` (StoriesSettings)

**Returns:** `ExecutionResult` — Returns a `ExecutionResult`.

[Docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#SetStories)

## PropertyCommands (static-class)

Tapir Add-On 'Property Commands' command group — 9 JSON-RPC commands.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/)

### Methods
#### `CreatePropertyDefinitions(propertyDefinitions: PropertyDefinitionArrayItem[]): object`

Creates Custom Property Definitions based on the given parameters.

**Parameters:**
- `propertyDefinitions` (PropertyDefinitionArrayItem[]) — The parameters of the new properties.

**Returns:** `object` — Returns an object with properties: propertyIds.

[Docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#CreatePropertyDefinitions)

#### `CreatePropertyGroups(propertyGroups: PropertyGroupArrayItem[]): object`

Creates Property Groups based on the given parameters.

**Parameters:**
- `propertyGroups` (PropertyGroupArrayItem[]) — The parameters of the new property groups.

**Returns:** `object` — Returns an object with properties: propertyGroupIds.

[Docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#CreatePropertyGroups)

#### `DeletePropertyDefinitions(propertyIds: PropertyIdArrayItem[]): object`

Deletes the given Custom Property Definitions.

**Parameters:**
- `propertyIds` (PropertyIdArrayItem[]) — The identifiers of properties to delete.

**Returns:** `object` — Returns an object with properties: executionResults.

[Docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#DeletePropertyDefinitions)

#### `DeletePropertyGroups(propertyGroupIds: PropertyGroupIdArrayItem[]): object`

Deletes the given Custom Property Groups.

**Parameters:**
- `propertyGroupIds` (PropertyGroupIdArrayItem[]) — The identifiers of property groups to delete.

**Returns:** `object` — Returns an object with properties: executionResults.

[Docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#DeletePropertyGroups)

#### `GetAllProperties(): object`

Returns all user defined and built-in properties.

**Returns:** `object` — Returns an object with properties: properties.

[Docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#GetAllProperties)

#### `GetPropertyValuesOfAttributes(attributeIds: AttributeIds, properties: PropertyIds): object`

Returns the property values of the attributes for the given property.

**Parameters:**
- `attributeIds` (AttributeIds)
- `properties` (PropertyIds)

**Returns:** `object` — Returns an object with properties: propertyValuesForAttributes.

[Docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#GetPropertyValuesOfAttributes)

#### `GetPropertyValuesOfElements(elements: Elements, properties: PropertyIds): object`

Returns the property values of the elements for the given property. It works for subelements of hierarchal elements also.

**Parameters:**
- `elements` (Elements)
- `properties` (PropertyIds)

**Returns:** `object` — Returns an object with properties: propertyValuesForElements.

[Docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#GetPropertyValuesOfElements)

#### `SetPropertyValuesOfAttributes(attributePropertyValues: AttributePropertyValues): object`

Sets the property values of attributes.

**Parameters:**
- `attributePropertyValues` (AttributePropertyValues)

**Returns:** `object` — Returns an object with properties: executionResults.

[Docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#SetPropertyValuesOfAttributes)

#### `SetPropertyValuesOfElements(elementPropertyValues: ElementPropertyValues): object`

Sets the property values of elements. It works for subelements of hierarchal elements also.

**Parameters:**
- `elementPropertyValues` (ElementPropertyValues)

**Returns:** `object` — Returns an object with properties: executionResults.

[Docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#SetPropertyValuesOfElements)

## RevisionManagementCommands (static-class)

Tapir Add-On 'Revision Management Commands' command group — 5 JSON-RPC commands.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/)

### Methods
#### `GetCurrentRevisionChangesOfLayouts(layoutDatabaseIds: Databases): object`

Retrieves all changes belong to the last revision of the given layouts.

**Parameters:**
- `layoutDatabaseIds` (Databases)

**Returns:** `object` — Returns an object with properties: currentRevisionChangesOfLayouts.

[Docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#GetCurrentRevisionChangesOfLayouts)

#### `GetDocumentRevisions(): object`

Retrieves all document revisions.

**Returns:** `object` — Returns an object with properties: documentRevisions.

[Docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#GetDocumentRevisions)

#### `GetRevisionChanges(): object`

Retrieves all changes.

**Returns:** `object` — Returns an object with properties: revisionChanges.

[Docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#GetRevisionChanges)

#### `GetRevisionChangesOfElements(elements: Elements): object`

Retrieves the changes belong to the given elements.

**Parameters:**
- `elements` (Elements)

**Returns:** `object` — Returns an object with properties: revisionChangesOfElements.

[Docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#GetRevisionChangesOfElements)

#### `GetRevisionIssues(): object`

Retrieves all issues.

**Returns:** `object` — Returns an object with properties: revisionIssues.

[Docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#GetRevisionIssues)

## TeamworkCommands (static-class)

Tapir Add-On 'Teamwork Commands' command group — 4 JSON-RPC commands.

[Vendor docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/)

### Methods
#### `ReleaseElements(elements: Elements): ExecutionResult`

Releases elements in Teamwork mode.

**Parameters:**
- `elements` (Elements)

**Returns:** `ExecutionResult` — Returns a `ExecutionResult`.

[Docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#ReleaseElements)

#### `ReserveElements(elements: Elements): object`

Reserves elements in Teamwork mode.

**Parameters:**
- `elements` (Elements)

**Returns:** `object` — Returns an object with properties: executionResult, conflicts.

[Docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#ReserveElements)

#### `TeamworkReceive(): ExecutionResult`

Performs a receive operation on the currently opened Teamwork project.

**Returns:** `ExecutionResult` — Returns a `ExecutionResult`.

[Docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#TeamworkReceive)

#### `TeamworkSend(): ExecutionResult`

Performs a send operation on the currently opened Teamwork project.

**Returns:** `ExecutionResult` — Returns a `ExecutionResult`.

[Docs](https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#TeamworkSend)

