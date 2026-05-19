---
name: archicad-archicad-officialcommands
description: This skill encodes the archicad 29.0 surface of the Archicad.OfficialCommands namespace — 12 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: AddOnCommands, AttributeCommands, BasicCommands, ClassificationCommands, ComponentCommands, ElementGeometryCommands, ElementListingCommands, ElementRelationCommands, and 4 more types.
---

# Archicad.OfficialCommands

Auto-generated from vendor docs for archicad 29.0. 12 types in this namespace.

## AddOnCommands (static-class)

Graphisoft official 'AddOn Commands' command group — 2 JSON-RPC commands (Archicad 29.0).

[Vendor docs](https://archicadapi.graphisoft.com/JSONInterfaceDocumentation/)

### Methods
#### `ExecuteAddOnCommand(addOnCommandId: AddOnCommandId, addOnCommandParameters?: AddOnCommandParameters): object`

Executes a command registered in an Add-On.

**Parameters:**
- `addOnCommandId` (AddOnCommandId)
- `addOnCommandParameters` (AddOnCommandParameters)

**Returns:** `object` — Returns an object with properties: addOnCommandResponse.

[Docs](https://archicadapi.graphisoft.com/JSONInterfaceDocumentation/#ExecuteAddOnCommand)

#### `IsAddOnCommandAvailable(addOnCommandId: AddOnCommandId): object`

Checks if the command is available or not.

**Parameters:**
- `addOnCommandId` (AddOnCommandId)

**Returns:** `object` — Returns an object with properties: available.

[Docs](https://archicadapi.graphisoft.com/JSONInterfaceDocumentation/#IsAddOnCommandAvailable)

## AttributeCommands (static-class)

Graphisoft official 'Attribute Commands' command group — 21 JSON-RPC commands (Archicad 29.0).

[Vendor docs](https://archicadapi.graphisoft.com/JSONInterfaceDocumentation/)

### Methods
#### `CreateAttributeFolders(attributeFolders: AttributeFolderCreationParameters[]): object`

Creates attribute folders. To create a folder, its full path has to be provided. The command will create all folders along the path, if they do not exist.

**Parameters:**
- `attributeFolders` (AttributeFolderCreationParameters[])

**Returns:** `object` — Returns an object with properties: executionResults.

[Docs](https://archicadapi.graphisoft.com/JSONInterfaceDocumentation/#CreateAttributeFolders)

#### `DeleteAttributeFolders(attributeFolderIds: AttributeFolderIds): object`

Deletes attribute folders and all the deletable attributes and folders it contains. To delete a folder, its identifier has to be provided.

**Parameters:**
- `attributeFolderIds` (AttributeFolderIds)

**Returns:** `object` — Returns an object with properties: executionResults.

[Docs](https://archicadapi.graphisoft.com/JSONInterfaceDocumentation/#DeleteAttributeFolders)

#### `DeleteAttributes(attributeIds: AttributeIds): object`

Deletes attributes.

**Parameters:**
- `attributeIds` (AttributeIds)

**Returns:** `object` — Returns an object with properties: executionResults.

[Docs](https://archicadapi.graphisoft.com/JSONInterfaceDocumentation/#DeleteAttributes)

#### `GetActivePenTables(): object`

Returns the model view and layout book pen table identifiers.

**Returns:** `object` — Returns an object with properties: modelViewPenTableId, layoutBookPenTableId.

[Docs](https://archicadapi.graphisoft.com/JSONInterfaceDocumentation/#GetActivePenTables)

#### `GetAttributeFolderStructure(attributeType: AttributeType, path?: AttributeFolderPath): object`

Returns the detailed folder structure for the attributes of a given type. If the path is not given, the root folder will be returned

**Parameters:**
- `attributeType` (AttributeType)
- `path` (AttributeFolderPath)

**Returns:** `object` — Returns an object with properties: attributeFolder.

[Docs](https://archicadapi.graphisoft.com/JSONInterfaceDocumentation/#GetAttributeFolderStructure)

#### `GetAttributeFolders(attributeFolderIds: AttributeFolderIds): object`

Returns the detailed attribute folders identified by their Ids.

**Parameters:**
- `attributeFolderIds` (AttributeFolderIds)

**Returns:** `object` — Returns an object with properties: attributeFolders.

[Docs](https://archicadapi.graphisoft.com/JSONInterfaceDocumentation/#GetAttributeFolders)

#### `GetAttributesByType(attributeType: AttributeType): object`

Returns the identifier of every attribute of the given type.

**Parameters:**
- `attributeType` (AttributeType)

**Returns:** `object` — Returns an object with properties: attributeIds.

[Docs](https://archicadapi.graphisoft.com/JSONInterfaceDocumentation/#GetAttributesByType)

#### `GetAttributesIndices(attributeIds: AttributeIds): object`

Returns the requested indices and guids of attributes.

**Parameters:**
- `attributeIds` (AttributeIds)

**Returns:** `object` — Returns an object with properties: attributeIndicesAndGuids.

[Docs](https://archicadapi.graphisoft.com/JSONInterfaceDocumentation/#GetAttributesIndices)

#### `GetBuildingMaterialAttributes(attributeIds: AttributeIds): object`

Returns the detailed building material attributes identified by their GUIDs.

**Parameters:**
- `attributeIds` (AttributeIds)

**Returns:** `object` — Returns an object with properties: attributes.

[Docs](https://archicadapi.graphisoft.com/JSONInterfaceDocumentation/#GetBuildingMaterialAttributes)

#### `GetCompositeAttributes(attributeIds: AttributeIds): object`

Returns the detailed composite attributes identified by their GUIDs.

**Parameters:**
- `attributeIds` (AttributeIds)

**Returns:** `object` — Returns an object with properties: attributes.

[Docs](https://archicadapi.graphisoft.com/JSONInterfaceDocumentation/#GetCompositeAttributes)

#### `GetFillAttributes(attributeIds: AttributeIds): object`

Returns the detailed fill attributes identified by their GUIDs.

**Parameters:**
- `attributeIds` (AttributeIds)

**Returns:** `object` — Returns an object with properties: attributes.

[Docs](https://archicadapi.graphisoft.com/JSONInterfaceDocumentation/#GetFillAttributes)

#### `GetLayerAttributes(attributeIds: AttributeIds): object`

Returns the detailed layer attributes identified by their GUIDs.

**Parameters:**
- `attributeIds` (AttributeIds)

**Returns:** `object` — Returns an object with properties: attributes.

[Docs](https://archicadapi.graphisoft.com/JSONInterfaceDocumentation/#GetLayerAttributes)

#### `GetLayerCombinationAttributes(attributeIds: AttributeIds): object`

Returns the detailed layer combination attributes identified by their GUIDs.

**Parameters:**
- `attributeIds` (AttributeIds)

**Returns:** `object` — Returns an object with properties: attributes.

[Docs](https://archicadapi.graphisoft.com/JSONInterfaceDocumentation/#GetLayerCombinationAttributes)

#### `GetLineAttributes(attributeIds: AttributeIds): object`

Returns the detailed line attributes identified by their GUIDs.

**Parameters:**
- `attributeIds` (AttributeIds)

**Returns:** `object` — Returns an object with properties: attributes.

[Docs](https://archicadapi.graphisoft.com/JSONInterfaceDocumentation/#GetLineAttributes)

#### `GetPenTableAttributes(attributeIds: AttributeIds): object`

Returns the detailed pen table attributes (including their pens) identified by their GUIDs.

**Parameters:**
- `attributeIds` (AttributeIds)

**Returns:** `object` — Returns an object with properties: attributes.

[Docs](https://archicadapi.graphisoft.com/JSONInterfaceDocumentation/#GetPenTableAttributes)

#### `GetProfileAttributePreview(attributeIds: AttributeIds, imageWidth: integer, imageHeight: integer, backgroundColor?: RGBColor): object`

Returns the preview image of each requested profile attribute in a base64 string format.

**Parameters:**
- `attributeIds` (AttributeIds)
- `imageWidth` (integer) — The width of the preview image.
- `imageHeight` (integer) — The height of the preview image.
- `backgroundColor` (RGBColor) — The background color of the preview image.

**Returns:** `object` — Returns an object with properties: previewImages.

[Docs](https://archicadapi.graphisoft.com/JSONInterfaceDocumentation/#GetProfileAttributePreview)

#### `GetProfileAttributes(attributeIds: AttributeIds): object`

Returns the detailed profile attributes identified by their GUIDs.

**Parameters:**
- `attributeIds` (AttributeIds)

**Returns:** `object` — Returns an object with properties: attributes.

[Docs](https://archicadapi.graphisoft.com/JSONInterfaceDocumentation/#GetProfileAttributes)

#### `GetSurfaceAttributes(attributeIds: AttributeIds): object`

Returns the detailed surface attributes identified by their GUIDs.

**Parameters:**
- `attributeIds` (AttributeIds)

**Returns:** `object` — Returns an object with properties: attributes.

[Docs](https://archicadapi.graphisoft.com/JSONInterfaceDocumentation/#GetSurfaceAttributes)

#### `GetZoneCategoryAttributes(attributeIds: AttributeIds): object`

Returns the detailed zone category attributes identified by their GUIDs.

**Parameters:**
- `attributeIds` (AttributeIds)

**Returns:** `object` — Returns an object with properties: attributes.

[Docs](https://archicadapi.graphisoft.com/JSONInterfaceDocumentation/#GetZoneCategoryAttributes)

#### `MoveAttributesAndFolders(attributeFolderIds: AttributeFolderIds, attributeIds: AttributeIds, targetFolderId: AttributeFolderId): object`

Moves attributes and attribute folders.

**Parameters:**
- `attributeFolderIds` (AttributeFolderIds)
- `attributeIds` (AttributeIds)
- `targetFolderId` (AttributeFolderId)

**Returns:** `object` — Empty result object.

[Docs](https://archicadapi.graphisoft.com/JSONInterfaceDocumentation/#MoveAttributesAndFolders)

#### `RenameAttributeFolders(attributeFolderParametersList: AttributeFolderRenameParameters[]): object`

Rename attribute folder.

**Parameters:**
- `attributeFolderParametersList` (AttributeFolderRenameParameters[])

**Returns:** `object` — Returns an object with properties: executionResults.

[Docs](https://archicadapi.graphisoft.com/JSONInterfaceDocumentation/#RenameAttributeFolders)

## BasicCommands (static-class)

Graphisoft official 'Basic Commands' command group — 2 JSON-RPC commands (Archicad 29.0).

[Vendor docs](https://archicadapi.graphisoft.com/JSONInterfaceDocumentation/)

### Methods
#### `GetProductInfo(): object`

Accesses the version information from the running Archicad.

**Returns:** `object` — Returns an object with properties: version, buildNumber, languageCode.

[Docs](https://archicadapi.graphisoft.com/JSONInterfaceDocumentation/#GetProductInfo)

#### `IsAlive(): object`

Checks if the Archicad connection is alive.

**Returns:** `object` — Returns an object with properties: isAlive.

[Docs](https://archicadapi.graphisoft.com/JSONInterfaceDocumentation/#IsAlive)

## ClassificationCommands (static-class)

Graphisoft official 'Classification Commands' command group — 8 JSON-RPC commands (Archicad 29.0).

[Vendor docs](https://archicadapi.graphisoft.com/JSONInterfaceDocumentation/)

### Methods
#### `GetAllClassificationSystems(): object`

Returns the list of available classification systems.

**Returns:** `object` — Returns an object with properties: classificationSystems.

[Docs](https://archicadapi.graphisoft.com/JSONInterfaceDocumentation/#GetAllClassificationSystems)

#### `GetAllClassificationsInSystem(classificationSystemId: ClassificationSystemId): object`

Returns the tree of classifications in the given classification system.

**Parameters:**
- `classificationSystemId` (ClassificationSystemId)

**Returns:** `object` — Returns an object with properties: classificationItems.

[Docs](https://archicadapi.graphisoft.com/JSONInterfaceDocumentation/#GetAllClassificationsInSystem)

#### `GetClassificationItemAvailability(classificationItemIds: ClassificationItemIds): object`

Returns the ids of property definitions available for a given classification item.

**Parameters:**
- `classificationItemIds` (ClassificationItemIds)

**Returns:** `object` — Returns an object with properties: classificationItemAvailabilityList.

[Docs](https://archicadapi.graphisoft.com/JSONInterfaceDocumentation/#GetClassificationItemAvailability)

#### `GetClassificationSystemIds(): object`

Returns the list of available classification systems.

**Returns:** `object` — Returns an object with properties: classificationSystemIds.

[Docs](https://archicadapi.graphisoft.com/JSONInterfaceDocumentation/#GetClassificationSystemIds)

#### `GetClassificationSystems(classificationSystemIds: ClassificationSystemIds): object`

Returns the details of classification systems identified by their GUIDs.

**Parameters:**
- `classificationSystemIds` (ClassificationSystemIds)

**Returns:** `object` — Returns an object with properties: classificationSystems.

[Docs](https://archicadapi.graphisoft.com/JSONInterfaceDocumentation/#GetClassificationSystems)

#### `GetClassificationsOfElements(elements: Elements, classificationSystemIds: ClassificationSystemIds): object`

Returns the classification of the given elements in the given classification systems.

**Parameters:**
- `elements` (Elements)
- `classificationSystemIds` (ClassificationSystemIds)

**Returns:** `object` — Returns an object with properties: elementClassifications.

[Docs](https://archicadapi.graphisoft.com/JSONInterfaceDocumentation/#GetClassificationsOfElements)

#### `GetDetailsOfClassificationItems(classificationItemIds: ClassificationItemIds): object`

Returns the details of classification items.

**Parameters:**
- `classificationItemIds` (ClassificationItemIds)

**Returns:** `object` — Returns an object with properties: classificationItems.

[Docs](https://archicadapi.graphisoft.com/JSONInterfaceDocumentation/#GetDetailsOfClassificationItems)

#### `SetClassificationsOfElements(elementClassifications: ElementClassifications): object`

Sets the classifications of elements. In order to set the classification of an element to unclassified, omit the classificationItemId field.

**Parameters:**
- `elementClassifications` (ElementClassifications)

**Returns:** `object` — Returns an object with properties: executionResults.

[Docs](https://archicadapi.graphisoft.com/JSONInterfaceDocumentation/#SetClassificationsOfElements)

## ComponentCommands (static-class)

Graphisoft official 'Component Commands' command group — 2 JSON-RPC commands (Archicad 29.0).

[Vendor docs](https://archicadapi.graphisoft.com/JSONInterfaceDocumentation/)

### Methods
#### `GetComponentsOfElements(elements: Elements): object`

Returns the identifier of every component for a list of elements. The order of the returned list is the same as the given elements.

**Parameters:**
- `elements` (Elements)

**Returns:** `object` — Returns an object with properties: componentsOfElements.

[Docs](https://archicadapi.graphisoft.com/JSONInterfaceDocumentation/#GetComponentsOfElements)

#### `GetPropertyValuesOfElementComponents(elementComponents: ElementComponents, properties: PropertyIds): object`

Returns the property values of the components for the given property.

**Parameters:**
- `elementComponents` (ElementComponents)
- `properties` (PropertyIds)

**Returns:** `object` — Returns an object with properties: propertyValuesForElementComponents.

[Docs](https://archicadapi.graphisoft.com/JSONInterfaceDocumentation/#GetPropertyValuesOfElementComponents)

## ElementGeometryCommands (static-class)

Graphisoft official 'Element Geometry Commands' command group — 2 JSON-RPC commands (Archicad 29.0).

[Vendor docs](https://archicadapi.graphisoft.com/JSONInterfaceDocumentation/)

### Methods
#### `Get2DBoundingBoxes(elements: Elements): object`

Get the 2D bounding box of elements identified by their GUIDs. The bounding box is calculated from the global origin on the floor plan view. The output is the array of the bounding boxes respective to the input GUIDs. Only works for elements detailed in <i>Element Information</i>.

**Parameters:**
- `elements` (Elements)

**Returns:** `object` — Returns an object with properties: boundingBoxes2D.

[Docs](https://archicadapi.graphisoft.com/JSONInterfaceDocumentation/#Get2DBoundingBoxes)

#### `Get3DBoundingBoxes(elements: Elements): object`

Get the 3D bounding box of elements identified by their GUIDs. The bounding box is calculated from the global origin in the 3D view. The output is the array of the bounding boxes respective to the input GUIDs. Only works for elements detailed in <i>Element Information</i>.

**Parameters:**
- `elements` (Elements)

**Returns:** `object` — Returns an object with properties: boundingBoxes3D.

[Docs](https://archicadapi.graphisoft.com/JSONInterfaceDocumentation/#Get3DBoundingBoxes)

## ElementListingCommands (static-class)

Graphisoft official 'Element Listing Commands' command group — 5 JSON-RPC commands (Archicad 29.0).

[Vendor docs](https://archicadapi.graphisoft.com/JSONInterfaceDocumentation/)

### Methods
#### `GetAllElements(): object`

Returns the identifier of every element in the current plan.

**Returns:** `object` — Returns an object with properties: elements.

[Docs](https://archicadapi.graphisoft.com/JSONInterfaceDocumentation/#GetAllElements)

#### `GetElementsByClassification(classificationItemId: ClassificationItemId): object`

Returns the identifier of every element with the given classification identifier.

**Parameters:**
- `classificationItemId` (ClassificationItemId)

**Returns:** `object` — Returns an object with properties: elements.

[Docs](https://archicadapi.graphisoft.com/JSONInterfaceDocumentation/#GetElementsByClassification)

#### `GetElementsByType(elementType: ElementType): object`

Returns the identifier of every element of the given type on the plan.

**Parameters:**
- `elementType` (ElementType)

**Returns:** `object` — Returns an object with properties: elements.

[Docs](https://archicadapi.graphisoft.com/JSONInterfaceDocumentation/#GetElementsByType)

#### `GetSelectedElements(onlyEditable?: boolean, onlySupportedTypes?: boolean): object`

Returns the identifiers of selected elements in the current plan.

**Parameters:**
- `onlyEditable` (boolean) — Optional parameter that defines whether the selection list should include only the editable elements or all of them. The default value is FALSE
- `onlySupportedTypes` (boolean) — Optional parameter. When it is set to true, only elements with types that are supported by any other JSON API command will be returned.

**Returns:** `object` — Returns an object with properties: elements.

[Docs](https://archicadapi.graphisoft.com/JSONInterfaceDocumentation/#GetSelectedElements)

#### `GetTypesOfElements(elements: Elements): object`

Returns the types of the given elements.

**Parameters:**
- `elements` (Elements)

**Returns:** `object` — Returns an object with properties: typesOfElements.

[Docs](https://archicadapi.graphisoft.com/JSONInterfaceDocumentation/#GetTypesOfElements)

## ElementRelationCommands (static-class)

Graphisoft official 'Element Relation Commands' command group — 1 JSON-RPC commands (Archicad 29.0).

[Vendor docs](https://archicadapi.graphisoft.com/JSONInterfaceDocumentation/)

### Methods
#### `GetElementsRelatedToZones(zones: Elements, elementTypes: ElementTypes): object`

Returns related elements of the given zones. The related elements will be grouped by type. If multiple zones was given, then the order of the returned list is that of the given zones.

**Parameters:**
- `zones` (Elements)
- `elementTypes` (ElementTypes)

**Returns:** `object` — Returns an object with properties: elementsRelatedToZones.

[Docs](https://archicadapi.graphisoft.com/JSONInterfaceDocumentation/#GetElementsRelatedToZones)

## LayoutBookCommands (static-class)

Graphisoft official 'Layout Book Commands' command group — 4 JSON-RPC commands (Archicad 29.0).

[Vendor docs](https://archicadapi.graphisoft.com/JSONInterfaceDocumentation/)

### Methods
#### `CreateLayout(layoutName: string, layoutParameters: LayoutParameters, masterNavigatorItemId: NavigatorItemId, parentNavigatorItemId: NavigatorItemId): object`

Creates a new layout.

**Parameters:**
- `layoutName` (string) — The name of the layout.
- `layoutParameters` (LayoutParameters)
- `masterNavigatorItemId` (NavigatorItemId)
- `parentNavigatorItemId` (NavigatorItemId)

**Returns:** `object` — Returns an object with properties: createdNavigatorItemId.

[Docs](https://archicadapi.graphisoft.com/JSONInterfaceDocumentation/#CreateLayout)

#### `CreateLayoutSubset(subsetParameters: Subset, parentNavigatorItemId: NavigatorItemId): object`

Creates a new layout subset.

**Parameters:**
- `subsetParameters` (Subset)
- `parentNavigatorItemId` (NavigatorItemId)

**Returns:** `object` — Returns an object with properties: createdSubsetId.

[Docs](https://archicadapi.graphisoft.com/JSONInterfaceDocumentation/#CreateLayoutSubset)

#### `GetLayoutSettings(layoutNavigatorItemId: NavigatorItemId): object`

Returns the parameters (settings) of the given layout.

**Parameters:**
- `layoutNavigatorItemId` (NavigatorItemId)

**Returns:** `object` — Returns an object with properties: layoutParameters.

[Docs](https://archicadapi.graphisoft.com/JSONInterfaceDocumentation/#GetLayoutSettings)

#### `SetLayoutSettings(layoutParameters: LayoutParameters, layoutNavigatorItemId: NavigatorItemId): object`

Sets the parameters (settings) of the given layout.

**Parameters:**
- `layoutParameters` (LayoutParameters)
- `layoutNavigatorItemId` (NavigatorItemId)

**Returns:** `object` — Empty result object.

[Docs](https://archicadapi.graphisoft.com/JSONInterfaceDocumentation/#SetLayoutSettings)

## NavigatorTreeCommands (static-class)

Graphisoft official 'Navigator Tree Commands' command group — 14 JSON-RPC commands (Archicad 29.0).

[Vendor docs](https://archicadapi.graphisoft.com/JSONInterfaceDocumentation/)

### Methods
#### `DeleteNavigatorItems(navigatorItemIds: NavigatorItemIds): object`

Deletes items from navigator tree.

**Parameters:**
- `navigatorItemIds` (NavigatorItemIds)

**Returns:** `object` — Returns an object with properties: executionResults.

[Docs](https://archicadapi.graphisoft.com/JSONInterfaceDocumentation/#DeleteNavigatorItems)

#### `GetBuiltInContainerNavigatorItems(navigatorItemIds: NavigatorItemIds): object`

Returns the details of the built-in container navigator items identified by their Ids.

**Parameters:**
- `navigatorItemIds` (NavigatorItemIds)

**Returns:** `object` — Returns an object with properties: navigatorItems.

[Docs](https://archicadapi.graphisoft.com/JSONInterfaceDocumentation/#GetBuiltInContainerNavigatorItems)

#### `GetDetailNavigatorItems(navigatorItemIds: NavigatorItemIds): object`

Returns the details of the detail navigator items identified by their Ids.

**Parameters:**
- `navigatorItemIds` (NavigatorItemIds)

**Returns:** `object` — Returns an object with properties: navigatorItems.

[Docs](https://archicadapi.graphisoft.com/JSONInterfaceDocumentation/#GetDetailNavigatorItems)

#### `GetDocument3DNavigatorItems(navigatorItemIds: NavigatorItemIds): object`

Returns the details of the 3D document navigator items identified by their Ids.

**Parameters:**
- `navigatorItemIds` (NavigatorItemIds)

**Returns:** `object` — Returns an object with properties: navigatorItems.

[Docs](https://archicadapi.graphisoft.com/JSONInterfaceDocumentation/#GetDocument3DNavigatorItems)

#### `GetElevationNavigatorItems(navigatorItemIds: NavigatorItemIds): object`

Returns the detailed elevation navigator items identified by their Ids.

**Parameters:**
- `navigatorItemIds` (NavigatorItemIds)

**Returns:** `object` — Returns an object with properties: navigatorItems.

[Docs](https://archicadapi.graphisoft.com/JSONInterfaceDocumentation/#GetElevationNavigatorItems)

#### `GetInteriorElevationNavigatorItems(navigatorItemIds: NavigatorItemIds): object`

Returns the details of the interior elevation navigator items identified by their Ids.

**Parameters:**
- `navigatorItemIds` (NavigatorItemIds)

**Returns:** `object` — Returns an object with properties: navigatorItems.

[Docs](https://archicadapi.graphisoft.com/JSONInterfaceDocumentation/#GetInteriorElevationNavigatorItems)

#### `GetNavigatorItemTree(navigatorTreeId: NavigatorTreeId): object`

Returns the tree of navigator items.

**Parameters:**
- `navigatorTreeId` (NavigatorTreeId)

**Returns:** `object` — Returns an object with properties: navigatorTree.

[Docs](https://archicadapi.graphisoft.com/JSONInterfaceDocumentation/#GetNavigatorItemTree)

#### `GetNavigatorItemsType(navigatorItemIds: NavigatorItemIds): object`

Returns all navigator item types based on the navigator item identifiers given. An error is returned for each identifier that is not found.

**Parameters:**
- `navigatorItemIds` (NavigatorItemIds)

**Returns:** `object` — Returns an object with properties: navigatorItemIdAndTypeList.

[Docs](https://archicadapi.graphisoft.com/JSONInterfaceDocumentation/#GetNavigatorItemsType)

#### `GetPublisherSetNames(): object`

Returns the names of available publisher sets.

**Returns:** `object` — Returns an object with properties: publisherSetNames.

[Docs](https://archicadapi.graphisoft.com/JSONInterfaceDocumentation/#GetPublisherSetNames)

#### `GetSectionNavigatorItems(navigatorItemIds: NavigatorItemIds): object`

Returns the details of the section navigator items identified by their Ids.

**Parameters:**
- `navigatorItemIds` (NavigatorItemIds)

**Returns:** `object` — Returns an object with properties: navigatorItems.

[Docs](https://archicadapi.graphisoft.com/JSONInterfaceDocumentation/#GetSectionNavigatorItems)

#### `GetStoryNavigatorItems(navigatorItemIds: NavigatorItemIds): object`

Returns the details of the story navigator items identified by their Ids.

**Parameters:**
- `navigatorItemIds` (NavigatorItemIds)

**Returns:** `object` — Returns an object with properties: navigatorItems.

[Docs](https://archicadapi.graphisoft.com/JSONInterfaceDocumentation/#GetStoryNavigatorItems)

#### `GetWorksheetNavigatorItems(navigatorItemIds: NavigatorItemIds): object`

Returns the details of the worksheet navigator items identified by their Ids.

**Parameters:**
- `navigatorItemIds` (NavigatorItemIds)

**Returns:** `object` — Returns an object with properties: navigatorItems.

[Docs](https://archicadapi.graphisoft.com/JSONInterfaceDocumentation/#GetWorksheetNavigatorItems)

#### `MoveNavigatorItem(navigatorItemIdToMove: NavigatorItemId, parentNavigatorItemId: NavigatorItemId, previousNavigatorItemId?: NavigatorItemId): object`

Moves the given navigator item under the <i>parentNavigatorItemId</i> in the navigator tree. If <i>previousNavigatorItemId</i> is not given then inserts it at the first place under the new parent. If it is given then inserts it after this navigator item.

**Parameters:**
- `navigatorItemIdToMove` (NavigatorItemId)
- `parentNavigatorItemId` (NavigatorItemId)
- `previousNavigatorItemId` (NavigatorItemId)

**Returns:** `object` — Empty result object.

[Docs](https://archicadapi.graphisoft.com/JSONInterfaceDocumentation/#MoveNavigatorItem)

#### `RenameNavigatorItem(): object`

Renames an existing navigator item by specifying either the name or the ID, or both.

**Returns:** `object` — Empty result object.

[Docs](https://archicadapi.graphisoft.com/JSONInterfaceDocumentation/#RenameNavigatorItem)

## PropertyCommands (static-class)

Graphisoft official 'Property Commands' command group — 10 JSON-RPC commands (Archicad 29.0).

[Vendor docs](https://archicadapi.graphisoft.com/JSONInterfaceDocumentation/)

### Methods
#### `GetAllPropertyGroupIds(propertyType?: PropertyType): object`

Returns the identifier of every property group in the current plan. The optional propertyType parameter can be used to filter the results based on the type of the property group (Built-in or User Defined).

**Parameters:**
- `propertyType` (PropertyType)

**Returns:** `object` — Returns an object with properties: propertyGroupIds.

[Docs](https://archicadapi.graphisoft.com/JSONInterfaceDocumentation/#GetAllPropertyGroupIds)

#### `GetAllPropertyIds(propertyType?: PropertyType): object`

Returns the identifier of every property in the current plan. The optional propertyType parameter can be used to filter the results based on the type of the property (Built-in or User Defined).

**Parameters:**
- `propertyType` (PropertyType)

**Returns:** `object` — Returns an object with properties: propertyIds.

[Docs](https://archicadapi.graphisoft.com/JSONInterfaceDocumentation/#GetAllPropertyIds)

#### `GetAllPropertyIdsOfElements(elements: Elements, propertyType?: PropertyType): object`

Returns all property identifiers of the given elements. The optional propertyType parameter can be used to filter the results based on the type of the property (Built-in or User Defined).

**Parameters:**
- `elements` (Elements)
- `propertyType` (PropertyType)

**Returns:** `object` — Returns an object with properties: propertyIdsOfElements.

[Docs](https://archicadapi.graphisoft.com/JSONInterfaceDocumentation/#GetAllPropertyIdsOfElements)

#### `GetAllPropertyNames(): object`

Returns the human-readable names of available Property definitions for debug and development purposes.

**Returns:** `object` — Returns an object with properties: properties.

[Docs](https://archicadapi.graphisoft.com/JSONInterfaceDocumentation/#GetAllPropertyNames)

#### `GetDetailsOfProperties(properties: PropertyIds): object`

Returns the details of property definitions.

**Parameters:**
- `properties` (PropertyIds)

**Returns:** `object` — Returns an object with properties: propertyDefinitions.

[Docs](https://archicadapi.graphisoft.com/JSONInterfaceDocumentation/#GetDetailsOfProperties)

#### `GetPropertyDefinitionAvailability(propertyIds: PropertyIds): object`

Returns the ids of classification items a given property definition is available for.

**Parameters:**
- `propertyIds` (PropertyIds)

**Returns:** `object` — Returns an object with properties: propertyDefinitionAvailabilityList.

[Docs](https://archicadapi.graphisoft.com/JSONInterfaceDocumentation/#GetPropertyDefinitionAvailability)

#### `GetPropertyGroups(propertyGroupIds: PropertyGroupIds): object`

Returns the details of property groups.

**Parameters:**
- `propertyGroupIds` (PropertyGroupIds)

**Returns:** `object` — Returns an object with properties: propertyGroups.

[Docs](https://archicadapi.graphisoft.com/JSONInterfaceDocumentation/#GetPropertyGroups)

#### `GetPropertyIds(properties: PropertyUserIds): object`

Returns the identifiers of property definitions for the requested property names.

**Parameters:**
- `properties` (PropertyUserIds) — List of property names whose ids are requested.

**Returns:** `object` — Returns an object with properties: properties.

[Docs](https://archicadapi.graphisoft.com/JSONInterfaceDocumentation/#GetPropertyIds)

#### `GetPropertyValuesOfElements(elements: Elements, properties: PropertyIds, onlySupportedTypes?: boolean): object`

Returns the property values of the elements for the given property.

**Parameters:**
- `elements` (Elements)
- `properties` (PropertyIds)
- `onlySupportedTypes` (boolean) — Optional parameter. When it is set to true, only elements with types that are supported by any other JSON API command will be returned.

**Returns:** `object` — Returns an object with properties: propertyValuesForElements.

[Docs](https://archicadapi.graphisoft.com/JSONInterfaceDocumentation/#GetPropertyValuesOfElements)

#### `SetPropertyValuesOfElements(elementPropertyValues: ElementPropertyValues): object`

Sets the property values of elements.

**Parameters:**
- `elementPropertyValues` (ElementPropertyValues)

**Returns:** `object` — Returns an object with properties: executionResults.

[Docs](https://archicadapi.graphisoft.com/JSONInterfaceDocumentation/#SetPropertyValuesOfElements)

## ViewMapCommands (static-class)

Graphisoft official 'View Map Commands' command group — 2 JSON-RPC commands (Archicad 29.0).

[Vendor docs](https://archicadapi.graphisoft.com/JSONInterfaceDocumentation/)

### Methods
#### `CloneProjectMapItemToViewMap(projectMapNavigatorItemId: NavigatorItemId, parentNavigatorItemId: NavigatorItemId): object`

Clones a project map item to the view map.

**Parameters:**
- `projectMapNavigatorItemId` (NavigatorItemId) — The ID of the navigator item to be cloned. Only navigator items from the project map can be cloned.
- `parentNavigatorItemId` (NavigatorItemId) — The ID of the navigator item below which the new view will be inserted. Only navigator items from the view map are allowed.

**Returns:** `object` — Returns an object with properties: createdNavigatorItemId.

[Docs](https://archicadapi.graphisoft.com/JSONInterfaceDocumentation/#CloneProjectMapItemToViewMap)

#### `CreateViewMapFolder(folderParameters: FolderParameters, parentNavigatorItemId?: NavigatorItemId, previousNavigatorItemId?: NavigatorItemId): object`

Creates a view folder item at the given position in the navigator tree.

**Parameters:**
- `folderParameters` (FolderParameters) — This is the name of the new folder.
- `parentNavigatorItemId` (NavigatorItemId) — The newly created folder will be placed under this parent item. If this parameter is not given the folder will be created as the first item in the View Map list.
- `previousNavigatorItemId` (NavigatorItemId) — The newly created folder will be placed after this sibling item. If this parameter is not given the folder will be created as the first item under the parent.

**Returns:** `object` — Returns an object with properties: createdFolderNavigatorItemId.

[Docs](https://archicadapi.graphisoft.com/JSONInterfaceDocumentation/#CreateViewMapFolder)

