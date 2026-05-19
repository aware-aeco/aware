---
name: dynamo-dynamo-wpf-interfaces
description: This skill encodes the dynamo 4.1.0 surface of the Dynamo.Wpf.Interfaces namespace — 6 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: ILibraryViewCustomization, LayoutSpecification, LayoutIncludeInfo, LayoutElementType, LayoutElement, LayoutSection.
---

# Dynamo.Wpf.Interfaces

Auto-generated from vendor docs for dynamo 4.1.0. 6 types in this namespace.

## ILibraryViewCustomization (interface)

Provides methods to customize library view

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Interfaces/ILibraryViewCustomization.cs)

### Methods
#### `bool AddElements(System.Collections.Generic.IEnumerable<Dynamo.Wpf.Interfaces.LayoutElement> elements, string sectionText)`

Allows clients to add a list of elements to a given section. This operation will fail if the text property of any of the child elements of the given section matches with the text property of the given elements.

**Parameters:**
- `elements` (System.Collections.Generic.IEnumerable<Dynamo.Wpf.Interfaces.LayoutElement>) — List of layout elements to add
- `sectionText` (string) — The name of the section to be updated

**Returns:** `bool` — True if the operation was successful

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Interfaces/ILibraryViewCustomization.cs)

#### `bool AddIncludeInfo(System.Collections.Generic.IEnumerable<Dynamo.Wpf.Interfaces.LayoutIncludeInfo> includes, string sectionText)`

Allows clients to add a list of layout include info to a given section. This operation will fail if the path property of any of the includes of the given section conflicts with the path property of the given includes.

**Parameters:**
- `includes` (System.Collections.Generic.IEnumerable<Dynamo.Wpf.Interfaces.LayoutIncludeInfo>) — List of layout include info to add
- `sectionText` (string) — The name of the section to be updated

**Returns:** `bool` — True if the operation was successful

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Interfaces/ILibraryViewCustomization.cs)

#### `bool AddSections(System.Collections.Generic.IEnumerable<Dynamo.Wpf.Interfaces.LayoutSection> sections)`

Allows clients to add a new section to the layout specification. This operation will fail if the current specification already contains a section with the same text as of the given section. The clients must call AddElements method to add layout elements to a given section.

**Parameters:**
- `sections` (System.Collections.Generic.IEnumerable<Dynamo.Wpf.Interfaces.LayoutSection>) — A list of new sections.

**Returns:** `bool` — True if the operation was successful

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Interfaces/ILibraryViewCustomization.cs)

#### `Dynamo.Wpf.Interfaces.LayoutSpecification GetSpecification()`

Gets a copy of the current library layout specification

**Returns:** `Dynamo.Wpf.Interfaces.LayoutSpecification` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Interfaces/ILibraryViewCustomization.cs)

#### `void OnAppShutdown()`

This method can be called by host application to notify the aplication shutdown so that the customization service can do the cleanup of resources.

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Interfaces/ILibraryViewCustomization.cs)

#### `bool RegisterResourceStream(string urlpath, System.IO.Stream resource)`

Registers a given resource stream for a given url. If registered successfully the requested resource is returned from the given stream.

**Parameters:**
- `urlpath` (string) — relative path of url including extension of the resource, e.g. /myicons/xicon.png
- `resource` (System.IO.Stream) — resource data stream

**Returns:** `bool` — True if the operation was successful

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Interfaces/ILibraryViewCustomization.cs)

#### `void SetSpecification(Dynamo.Wpf.Interfaces.LayoutSpecification specification)`

Sets the given specification as the current library layout specification by overwriting the current one.

**Parameters:**
- `specification` (Dynamo.Wpf.Interfaces.LayoutSpecification) — New layout specification

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Interfaces/ILibraryViewCustomization.cs)

#### `System.IO.Stream ToJSONStream()`

Serializes the current specification to JSON stream

**Returns:** `System.IO.Stream` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Interfaces/ILibraryViewCustomization.cs)

### Events
#### `SpecificationUpdated` (`System.EventHandler`)

**Signature:** `public event System.EventHandler SpecificationUpdated`

This event is raised when the specification is updated

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Interfaces/ILibraryViewCustomization.cs)

## LayoutElement (class)

Represents a group/category

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Interfaces/LayoutSpecification.cs)

### Constructors
- `void LayoutElement(string text)` — Default constructor

### Methods
#### `System.Collections.Generic.IEnumerable<Dynamo.Wpf.Interfaces.LayoutElement> EnumerateChildren()`

Returns a flat list of all the children, including the children owned by its child elements recursively

**Returns:** `System.Collections.Generic.IEnumerable<Dynamo.Wpf.Interfaces.LayoutElement>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Interfaces/LayoutSpecification.cs)

#### `System.Collections.Generic.IEnumerable<Dynamo.Wpf.Interfaces.LayoutIncludeInfo> EnumerateIncludes()`

Returns a flat list of all the include info, including those owned by its child elements recursively.

**Returns:** `System.Collections.Generic.IEnumerable<Dynamo.Wpf.Interfaces.LayoutIncludeInfo>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Interfaces/LayoutSpecification.cs)

### Properties
- `childElements` (System.Collections.Generic.List<Dynamo.Wpf.Interfaces.LayoutElement>, get/set) — List of nested elements under this element
- `elementType` (Dynamo.Wpf.Interfaces.LayoutElementType, get/set) — The type of the element.
- `iconUrl` (string, get/set) — Relative or absolute URL of the icon for the corresponding library item
- `include` (System.Collections.Generic.List<Dynamo.Wpf.Interfaces.LayoutIncludeInfo>, get/set) — List of data types that should be included under this given library element
- `text` (string, get/set) — Name of the element

## LayoutElementType (enum)

Represents the type of the element. Possible values are section, category, group, create, action, query and none.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Interfaces/LayoutSpecification.cs)

### Values
- `action` = `4` — Elements of this type result in library items that get clubbed under the Action cluster
- `category` = `1` — Category elements represent the root library items contained in section elements.
- `classType` = `7` — classType comes directly under its parent category, it contains just text without icon
- `create` = `3` — Elements of this type result in library items that get clubbed under the Create cluster
- `group` = `2` — Groups comes directly under its parent category, it contains just text without icon
- `none` = `6` — All other expandable library items that are not categories or groups
- `query` = `5` — Elements of this type result in library items that get clubbed under the Query cluster
- `section` = `0` — Section elements represent the root items on the library view

## LayoutIncludeInfo (class)

Represents the information about the data type that needs to be included under a given LayoutElement.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Interfaces/LayoutSpecification.cs)

### Constructors
- `void LayoutIncludeInfo()` — Default constructor

### Properties
- `iconUrl` (string, get/set) — Relative or absolute URL of the icon for the corresponding library item
- `inclusive` (bool, get/set) — Checks if a new category to be created for its path. If the include path is "A.B.C" under an element say "Root" and for two nodes fully qualified name is "A.B.C.D" and "A.B.C.E", then by default inclusive is true that means there we will get a category "C" under "Root" element as parent and "D" and "E" as children. If inclusive is false then both "D" and "E" will be direct children of "Root" and there won't be any intermediate element "C".
- `path` (string, get/set) — Partial or full namespace to be included under a given layout element

## LayoutSection (class)

Represents a section element, which is the root element in the library view

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Interfaces/LayoutSpecification.cs)

### Constructors
- `void LayoutSection(string text)` — Default constructor

### Properties
- `showHeader` (bool, get/set) — Sections may have headers displayed or hidden. If the header is shown the section is collapsible, otheriwse its always expanded.

## LayoutSpecification (class)

Represents the specification for the library items layout in library view.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Interfaces/LayoutSpecification.cs)

### Constructors
- `void LayoutSpecification()` — Default constructor

### Methods
#### `Dynamo.Wpf.Interfaces.LayoutSpecification Clone()`

Performs a deep clone of this object and returns a new LayoutSpecification object

**Returns:** `Dynamo.Wpf.Interfaces.LayoutSpecification` — A cloned LayoutSpecification object

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Interfaces/LayoutSpecification.cs)

#### `Dynamo.Wpf.Interfaces.LayoutSpecification FromJSONStream(System.IO.Stream stream)`

Creates LibraryLayoutSpec object from given json stream

**Parameters:**
- `stream` (System.IO.Stream) — JSON stream

**Returns:** `Dynamo.Wpf.Interfaces.LayoutSpecification` — LibraryLayoutSpec object or null

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Interfaces/LayoutSpecification.cs)

#### `Dynamo.Wpf.Interfaces.LayoutSpecification FromJSONString(string jsonText)`

Creates LibraryLayoutSpec object from the given json text

**Parameters:**
- `jsonText` (string) — JSON text to deserialize

**Returns:** `Dynamo.Wpf.Interfaces.LayoutSpecification` — LibraryLayoutSpec object or null

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Interfaces/LayoutSpecification.cs)

#### `System.IO.Stream ToJSONStream()`

Serializes the layout specification to json stream

**Returns:** `System.IO.Stream` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Interfaces/LayoutSpecification.cs)

### Properties
- `sections` (System.Collections.Generic.List<Dynamo.Wpf.Interfaces.LayoutSection>, get/set) — List of LayoutSections

