---
name: tekla-tekla-structures-model
description: This skill encodes the tekla 2025.0 surface of the Tekla.Structures.Model namespace — 280 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: BaseComponent, Assembly, BasePoint, BaseRebarGroup, BaseWeld, BaseRebarModifier, BendSurface, Beam, and 272 more types.
---

# Tekla.Structures.Model

Auto-generated from vendor docs for tekla 2025.0. 280 types in this namespace.

## Assembly (class)

The Assembly class defines a single manufacturing unit: objects that are bolted or welded together in the workshop. An assembly has a main part and secondary assemblables attached to it. The number of secondaries is limited to 2048. Hierarchical assemblies can also have subassemblies attached to them and they can have a parent assembly instance.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/05895e96-9746-fce3-380b-7892400ca1b8)

### Constructors
- `Assembly(...)` — Creates a new assembly instance.

### Methods
#### `Add(ArrayList)(...)`

Adds an array list of given assemblable instances to the assembly. For assemblies use Add(Assembly Assembly). Notice that adding other than assembly instances will actually assume that you are adding objects to a cast unit assembly.

[Docs](https://developer.tekla.com/topic/en/18/43/14444602-aeba-c719-f883-59fc8432805f)

#### `Add(Assembly)(...)`

Adds a new subassembly for the assembly instance.

[Docs](https://developer.tekla.com/topic/en/18/43/07ace982-ded4-7674-5bbc-b6e3f5823e0e)

#### `Add(IAssemblable)(...)`

Adds the given assemblable instance to the assembly. Notice that adding other than assembly instances will assume that you are adding objects to a cast unit assembly.

[Docs](https://developer.tekla.com/topic/en/18/43/2a844691-427f-aded-4ec7-ae07ae95db63)

#### `CompareTo(Assembly)(...)`

Compares the instantiated assembly with another one.

[Docs](https://developer.tekla.com/topic/en/18/43/22871c43-1a15-bb3b-862c-d8a2e5e5f7bd)

#### `CompareTo(Object)(...)`

Compares Identifiers of model objects.

[Docs](https://developer.tekla.com/topic/en/18/43/79cca356-0c8a-e8fb-463a-34ad4141bec7)

#### `Delete(...)`

Explodes the assembly instance.

[Docs](https://developer.tekla.com/topic/en/18/43/0be6c8ba-616e-71b2-8a95-41b257c9c319)

#### `DeleteUserDefinedCoordSys(...)`

Deletes the assembly's user defined coordinate system, if it has one.

[Docs](https://developer.tekla.com/topic/en/18/43/c20117e0-d7a8-1d6a-bb9a-f7c879cff4d5)

#### `Equals(...)`

Check if Identifiers of model objects are same.

[Docs](https://developer.tekla.com/topic/en/18/43/e4324265-2490-00de-4139-63a8f7acb492)

#### `GetAllReportProperties(...)`

Retrieves all properties for the model object in one hashtable. Type for the returned value must be checked using type casting.

[Docs](https://developer.tekla.com/topic/en/18/43/0708f404-ce77-28e1-4ebe-fa906f68bff8)

#### `GetAllUserProperties(...)`

Retrieves all properties for the model object in one hashtable. Type for the returned value must be checked using type casting.

[Docs](https://developer.tekla.com/topic/en/18/43/63b853a0-0af5-af9c-3ce7-05299664d7f8)

#### `GetAssembly(...)`

Returns the assembly instance the assembly belongs to (null if none).

[Docs](https://developer.tekla.com/topic/en/18/43/cd9afea6-4dab-a394-bb22-8e55c9a937f0)

#### `GetAssemblyType(...)`

Returns the type of the assembly.

[Docs](https://developer.tekla.com/topic/en/18/43/7b884d13-eb60-326a-616a-525342ded4f3)

#### `GetChildren(...)`

Returns an enumerator of all the children model objects.

[Docs](https://developer.tekla.com/topic/en/18/43/ce51ec40-310c-6067-a89f-f8d029aab747)

#### `GetCoordinateSystem(...)`

Returns the coordinate system for the given model object.

[Docs](https://developer.tekla.com/topic/en/18/43/8adfe9cd-f6e5-6474-1297-33c6270a7b0a)

#### `GetCustomObjectType(...)`

Gets custom object type name from an existing object.

[Docs](https://developer.tekla.com/topic/en/18/43/d3be2ba3-8533-7165-f2d3-a5aa9df556b8)

#### `GetDoubleReportProperties(...)`

Retrieves multiple double report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/f86f6c96-f738-49ca-16ba-f5634e862ba2)

#### `GetDoubleUserProperties(...)`

Retrieves all double properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/6ead7a63-5f76-0631-d480-d46c27e882aa)

#### `GetDynamicStringProperty(...)`

Gets a dynamic string property from the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/415e86d9-7de6-ea53-ce68-20b3e67b8655)

#### `GetFatherComponent(...)`

Returns the father component of the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/a2c28f3f-52ff-50a8-1b32-10d18db5c31d)

#### `GetFatherPour(...)`

Returns the pour that the assembly is directly associated to.

[Docs](https://developer.tekla.com/topic/en/18/43/d8cf90fe-74f8-5694-9426-fea41881ab1c)

#### `GetFatherPourUnit(...)`

Returns the pour unit that the bolt group is associated to.

[Docs](https://developer.tekla.com/topic/en/18/43/ca75149c-7019-d1dd-1340-cf5a95af04fb)

#### `GetHierarchicObjects(...)`

Returns an enumerator of all the connected hierarchic objects.

[Docs](https://developer.tekla.com/topic/en/18/43/cc228ea0-267b-7d90-7441-c9efc1779b08)

#### `GetIntegerReportProperties(...)`

Retrieves multiple integer report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/2623a9c0-2a9c-e233-9daa-59f92ed51445)

#### `GetIntegerUserProperties(...)`

Retrieves all integer properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/1b3c6f06-bc25-a54e-2a88-b60dc4a3c85e)

#### `GetMainObject(...)`

Selects and returns the main object of the assembly or null if none.

[Docs](https://developer.tekla.com/topic/en/18/43/3e9629aa-22a8-3a1d-7e46-467636f8e125)

#### `GetMainPart(...)`

Selects and returns the main part of the assembly or null if none.

[Docs](https://developer.tekla.com/topic/en/18/43/68febdc3-73bc-7733-d10a-ee8f29a60a25)

#### `GetPhase(...)`

Retrieves the phase of the model object (the phase number, the phase name, the phase comment and whether the phase is the current one or not).

[Docs](https://developer.tekla.com/topic/en/18/43/310f8b7a-c120-753a-35c9-f7c73a9806c9)

#### `GetReportProperty(String, Double.)(...)`

Retrieves a double property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/1c05aa56-5f42-be91-a59e-8d59a5549f45)

#### `GetReportProperty(String, Int32.)(...)`

Retrieves an integer property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/ce8e4ca0-3750-3c9b-5ca8-27e48b45e0bd)

#### `GetReportProperty(String, String.)(...)`

Retrieves a string property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/70f212bb-ee70-e9e9-7ab6-d6a9d46f8de6)

#### `GetSecondaries(...)`

Selects and returns all the secondary assemblables belonging to the assembly. The method replaces the old GetObjects() method. Notice that the number of secondaries is limited to 2048.

[Docs](https://developer.tekla.com/topic/en/18/43/91373524-2bb2-12c2-a32a-6708b195bc6d)

#### `GetStringReportProperties(...)`

Retrieves multiple string report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/5ff72da1-7382-f5d2-eb1d-c99b46239d42)

#### `GetStringUserProperties(...)`

Retrieves all string properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/67120e84-c820-9d52-12ae-c167c595842c)

#### `GetSubAssemblies(...)`

Selects and returns a list of subassemblies belonging to the assembly.

[Docs](https://developer.tekla.com/topic/en/18/43/259ed63a-3eb8-96a7-d5b5-6bc5a26f77f8)

#### `GetUserProperty(String, Double.)(...)`

Retrieves a double property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/17cc8074-7955-32b5-2b4b-b169c45d4ee0)

#### `GetUserProperty(String, Int32.)(...)`

Retrieves an integer property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/d9364c1d-5b57-6a31-4c13-01124058cfb2)

#### `GetUserProperty(String, String.)(...)`

Retrieves a string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/214a886c-c18c-9b66-d7c9-fe84260962f2)

#### `HasUserDefinedCoordSys(...)`

Returns whether the assembly has a user defined coordinate system.

[Docs](https://developer.tekla.com/topic/en/18/43/e6a5faaa-31e1-6978-4d97-e1d53ddb47ea)

#### `Insert(...)`

Inserts the Assembly instance in the model. At the moment only rebar assemblies can be inserted. The main object of the assembly must be a Reinforcement or RebarSet. If the main rebar is not set (null), the main rebar will be automatically set to the secondary rebar with the biggest weight.

[Docs](https://developer.tekla.com/topic/en/18/43/267fc42c-acb4-f8ab-686a-da9747e9e12f)

#### `Modify(...)`

Modifies the assembly instance in the model.

[Docs](https://developer.tekla.com/topic/en/18/43/a1dcbb64-dc3b-ad05-9541-ad0f9d4055bf)

#### `Remove(...)`

Removes the given instance from the assembly. Before removing steel parts from an assembly all connecting workshop welds and bolts must be deleted.

[Docs](https://developer.tekla.com/topic/en/18/43/f2b5be55-4710-68f1-81d2-cc2651a52c26)

#### `Select(...)`

Selects the assembly instance from the model. The identifier of the instance must be set.

[Docs](https://developer.tekla.com/topic/en/18/43/3127bb46-b578-c2b8-14aa-cd30a76a1c6e)

#### `SetCustomObjectType(...)`

Sets a custom object type name for an existing object.

[Docs](https://developer.tekla.com/topic/en/18/43/3d6fb91b-48a9-35ca-d498-526514344089)

#### `SetDynamicStringProperty(...)`

Sets a dynamic string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/4dab0254-09e5-5e61-c28b-1c12afab71a0)

#### `SetLabel(...)`

Sets a label for an object when a new instance is created, this method must be called before Insert. The label is used in plug-ins for identifying the changed object in modification.

[Docs](https://developer.tekla.com/topic/en/18/43/3efcfc44-613a-e66b-9ebd-9f86df4f4036)

#### `SetMainObject(...)`

Sets the main object of the assembly.

[Docs](https://developer.tekla.com/topic/en/18/43/da28460a-4a0b-4c18-9bb1-3421095b2a3c)

#### `SetMainPart(...)`

Sets the main part of the assembly.

[Docs](https://developer.tekla.com/topic/en/18/43/e7ff7520-2344-30f2-79cd-9abd6317e297)

#### `SetPhase(...)`

Sets the phase of the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/ed80abcc-0ff7-d081-2229-e651f858a727)

#### `SetUserDefinedCoordSys(...)`

Set user defined assembly coordinate system. If set, GetCoordinateSystem will return this value.

[Docs](https://developer.tekla.com/topic/en/18/43/b9037408-029a-e8f6-fa92-1dc8ef153ce6)

#### `SetUserProperties(...)`

Sets multiple properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/8659da37-d571-f03f-c666-5c9fa3b113f5)

#### `SetUserProperty(String, Double)(...)`

Sets a double property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/5c585b8b-5c2e-7b75-2242-758f17429396)

#### `SetUserProperty(String, Int32)(...)`

Sets an integer property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/dd5b828d-3a87-bb58-5ac9-e1c3b4c2fe4b)

#### `SetUserProperty(String, String)(...)`

Sets a string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/80bf0348-b651-3d4a-862b-49c85bc924ab)

### Properties
- `AssemblyNumber` (object, get/set) — Defines the numbering in the assembly sense. If the members are empty and zero, the main part attributes are used.
- `Identifier` (object, get/set) — The identifier of the object.
- `IsUpToDate` (object, get/set) — Gets if the object does not have a modification which is not shared.
- `ModificationTime` (object, get/set) — Gets latest time of the object was modified or created.
- `Name` (object, get/set) — The name of the assembly.

## Assembly.AssemblyTypeEnum (enum)

The assembly types.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/b5783436-5017-c594-0f5f-3c437e0e1108)

### Values
- `STEEL_ASSEMBLY` = `0` — The steel assembly type.
- `PRECAST_ASSEMBLY` = `1` — The precast assembly type.
- `IN_SITU_ASSEMBLY` = `2` — The in-situ assembly type.
- `TIMBER_ASSEMBLY` = `3` — The timber assembly type.
- `UNKNOWN_ASSEMBLY` = `4` — The unknown assembly type.
- `REBAR_ASSEMBLY` = `5` — The rebar assembly type.

## BaseComponent (class)

The BaseComponent class is an abstract base class that represents generic components. The generic components derived from the base component are the component, the connection, the detail and the seam.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/3f5c4af6-b37c-5f30-9ce4-ea5f2bf028fe)

### Constructors
- `BaseComponent(...)` — Creates a base component instance with default attributes.

### Methods
#### `CompareTo(...)`

Compares Identifiers of model objects.

[Docs](https://developer.tekla.com/topic/en/18/43/79cca356-0c8a-e8fb-463a-34ad4141bec7)

#### `Delete(...)`

Deletes the instance from the model database.

[Docs](https://developer.tekla.com/topic/en/18/43/0b922776-ccee-4b5e-4a12-4e5306802e90)

#### `Equals(...)`

Check if Identifiers of model objects are same.

[Docs](https://developer.tekla.com/topic/en/18/43/e4324265-2490-00de-4139-63a8f7acb492)

#### `GetAllReportProperties(...)`

Retrieves all properties for the model object in one hashtable. Type for the returned value must be checked using type casting.

[Docs](https://developer.tekla.com/topic/en/18/43/0708f404-ce77-28e1-4ebe-fa906f68bff8)

#### `GetAllUserProperties(...)`

Retrieves all properties for the model object in one hashtable. Type for the returned value must be checked using type casting.

[Docs](https://developer.tekla.com/topic/en/18/43/63b853a0-0af5-af9c-3ce7-05299664d7f8)

#### `GetAttribute(String, Double.)(...)`

Retrieves the attribute with the given name.

[Docs](https://developer.tekla.com/topic/en/18/43/a347ce9e-865f-554d-140f-9a65728b24a3)

#### `GetAttribute(String, Int32.)(...)`

Retrieves the attribute with the given name.

[Docs](https://developer.tekla.com/topic/en/18/43/2c48112c-5580-99f3-6c35-70fd5f538b09)

#### `GetAttribute(String, String.)(...)`

Retrieves the attribute with the given name.

[Docs](https://developer.tekla.com/topic/en/18/43/c8652aa1-9069-88f2-b821-3afdd68e7ca5)

#### `GetChildren(...)`

Returns an enumerator of all the children model objects.

[Docs](https://developer.tekla.com/topic/en/18/43/ce51ec40-310c-6067-a89f-f8d029aab747)

#### `GetCoordinateSystem(...)`

Returns the coordinate system for the given model object.

[Docs](https://developer.tekla.com/topic/en/18/43/8adfe9cd-f6e5-6474-1297-33c6270a7b0a)

#### `GetCustomObjectType(...)`

Gets custom object type name from an existing object.

[Docs](https://developer.tekla.com/topic/en/18/43/d3be2ba3-8533-7165-f2d3-a5aa9df556b8)

#### `GetDoubleReportProperties(...)`

Retrieves multiple double report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/f86f6c96-f738-49ca-16ba-f5634e862ba2)

#### `GetDoubleUserProperties(...)`

Retrieves all double properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/6ead7a63-5f76-0631-d480-d46c27e882aa)

#### `GetDynamicStringProperty(...)`

Gets a dynamic string property from the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/415e86d9-7de6-ea53-ce68-20b3e67b8655)

#### `GetFatherComponent(...)`

Returns the father component of the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/a2c28f3f-52ff-50a8-1b32-10d18db5c31d)

#### `GetHierarchicObjects(...)`

Returns an enumerator of all the connected hierarchic objects.

[Docs](https://developer.tekla.com/topic/en/18/43/cc228ea0-267b-7d90-7441-c9efc1779b08)

#### `GetIntegerReportProperties(...)`

Retrieves multiple integer report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/2623a9c0-2a9c-e233-9daa-59f92ed51445)

#### `GetIntegerUserProperties(...)`

Retrieves all integer properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/1b3c6f06-bc25-a54e-2a88-b60dc4a3c85e)

#### `GetPhase(...)`

Retrieves the phase of the model object (the phase number, the phase name, the phase comment and whether the phase is the current one or not).

[Docs](https://developer.tekla.com/topic/en/18/43/310f8b7a-c120-753a-35c9-f7c73a9806c9)

#### `GetReportProperty(String, Double.)(...)`

Retrieves a double property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/1c05aa56-5f42-be91-a59e-8d59a5549f45)

#### `GetReportProperty(String, Int32.)(...)`

Retrieves an integer property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/ce8e4ca0-3750-3c9b-5ca8-27e48b45e0bd)

#### `GetReportProperty(String, String.)(...)`

Retrieves a string property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/70f212bb-ee70-e9e9-7ab6-d6a9d46f8de6)

#### `GetStringReportProperties(...)`

Retrieves multiple string report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/5ff72da1-7382-f5d2-eb1d-c99b46239d42)

#### `GetStringUserProperties(...)`

Retrieves all string properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/67120e84-c820-9d52-12ae-c167c595842c)

#### `GetUserProperty(String, Double.)(...)`

Retrieves a double property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/17cc8074-7955-32b5-2b4b-b169c45d4ee0)

#### `GetUserProperty(String, Int32.)(...)`

Retrieves an integer property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/d9364c1d-5b57-6a31-4c13-01124058cfb2)

#### `GetUserProperty(String, String.)(...)`

Retrieves a string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/214a886c-c18c-9b66-d7c9-fe84260962f2)

#### `Insert(...)`

Inserts the model object instance into the model database.

[Docs](https://developer.tekla.com/topic/en/18/43/f17ed840-1d16-b843-4b0c-270875de1d35)

#### `LoadAttributesFromFile(...)`

Loads the attributes for the component from the given file. These attributes will be loaded before all the attributes that have been set with the SetAttribute methods, so any attributes that are set with SetAttribute will override those loaded from the given standard file.

[Docs](https://developer.tekla.com/topic/en/18/43/acc9930a-aa6f-667c-0878-7ea80ce416b8)

#### `Modify(...)`

Modifies the model instance in the model database.

[Docs](https://developer.tekla.com/topic/en/18/43/186833b8-4837-ed77-b314-3ebf9dc32d6a)

#### `Select(...)`

Selects the model object instance from the model database.

[Docs](https://developer.tekla.com/topic/en/18/43/f662ab10-be15-6280-d0aa-fc259dfd7c07)

#### `SetAttribute(String, Double)(...)`

Sets the attribute's value to the given value.

[Docs](https://developer.tekla.com/topic/en/18/43/785d0216-df12-53fc-344b-a2e67001cf3c)

#### `SetAttribute(String, Int32)(...)`

Sets the attribute's value to the given value.

[Docs](https://developer.tekla.com/topic/en/18/43/797240ce-567c-bff9-2a60-6359cc909ce0)

#### `SetAttribute(String, String)(...)`

Sets the attribute's value to the given value.

[Docs](https://developer.tekla.com/topic/en/18/43/6f0f7c8f-c5d2-8012-9faf-f68991966ff5)

#### `SetCustomObjectType(...)`

Sets a custom object type name for an existing object.

[Docs](https://developer.tekla.com/topic/en/18/43/3d6fb91b-48a9-35ca-d498-526514344089)

#### `SetDynamicStringProperty(...)`

Sets a dynamic string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/4dab0254-09e5-5e61-c28b-1c12afab71a0)

#### `SetLabel(...)`

Sets a label for an object when a new instance is created, this method must be called before Insert. The label is used in plug-ins for identifying the changed object in modification.

[Docs](https://developer.tekla.com/topic/en/18/43/3efcfc44-613a-e66b-9ebd-9f86df4f4036)

#### `SetPhase(...)`

Sets the phase of the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/ed80abcc-0ff7-d081-2229-e651f858a727)

#### `SetUserProperties(...)`

Sets multiple properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/8659da37-d571-f03f-c666-5c9fa3b113f5)

#### `SetUserProperty(String, Double)(...)`

Sets a double property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/5c585b8b-5c2e-7b75-2242-758f17429396)

#### `SetUserProperty(String, Int32)(...)`

Sets an integer property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/dd5b828d-3a87-bb58-5ac9-e1c3b4c2fe4b)

#### `SetUserProperty(String, String)(...)`

Sets a string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/80bf0348-b651-3d4a-862b-49c85bc924ab)

### Properties
- `Identifier` (object, get/set) — The identifier of the object.
- `IsUpToDate` (object, get/set) — Gets if the object does not have a modification which is not shared.
- `ModificationTime` (object, get/set) — Gets latest time of the object was modified or created.
- `Name` (object, get/set) — The name of the component. The name identifies custom components or plug-ins.
- `Number` (object, get/set) — The number of the component. A number greater than zero identifies system components, for custom components the number is CUSTOM_OBJECT_NUMBER, and for plug-ins the number is PLUGIN_OBJECT_NUMBER.

## BasePoint (class)

The BasePoint class provides base point related functionalities. Base points can be retrieved using ProjectInfo class.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/5c1b45f5-9a0b-cd81-ec51-dd4e21ff20ce)

### Constructors
- `BasePoint(...)` — Initializes a new instance of the BasePoint class.
- `BasePoint(...)` — Initializes a new instance of the BasePoint class.

### Methods
#### `ConvertFromBasePoint(BasePoint, Point)(...)`

Converts the given base point point to local point.

[Docs](https://developer.tekla.com/topic/en/18/43/405c560b-8772-065c-7a63-e474d21592c2)

#### `ConvertFromBasePoint(Point)(...)`

Converts the given point from this base point to local point.

[Docs](https://developer.tekla.com/topic/en/18/43/69941ebe-3291-9807-b9ea-24339052ba16)

#### `ConvertToBasePoint(BasePoint, Point)(...)`

Converts the given local point to given base point.

[Docs](https://developer.tekla.com/topic/en/18/43/1ff21f4d-0da5-7345-9494-dd1bddcd9816)

#### `ConvertToBasePoint(Point)(...)`

Converts the given local point to this base point.

[Docs](https://developer.tekla.com/topic/en/18/43/07421afb-80e7-8279-e0b8-aa31206d6ff9)

#### `Delete(...)`

Deletes the base point from the model database.

[Docs](https://developer.tekla.com/topic/en/18/43/59b13ff7-fc18-dfed-6c0d-34efd5302110)

#### `GetCompoundPlaneAngleLatitude(...)`

Gets the Latitude as a compound plane angle.

[Docs](https://developer.tekla.com/topic/en/18/43/fb80f506-4793-911c-ed0f-32e7ddf5da12)

#### `GetCompoundPlaneAngleLongitude(...)`

Gets the Longitude as a compound plane angle.

[Docs](https://developer.tekla.com/topic/en/18/43/303a3cf2-8ce2-2cb7-84c4-77cdce557ca8)

#### `GetCoordinateSystem(...)`

Gets the coordinate system of the base point.

[Docs](https://developer.tekla.com/topic/en/18/43/230d2dc0-2288-59b4-4175-93eae8271817)

#### `Insert(...)`

Inserts the base point into the model database.

[Docs](https://developer.tekla.com/topic/en/18/43/74097064-a8ad-b2bc-4fd1-7ed932b55ba3)

#### `Modify(...)`

Modifies the base point in the model database.

[Docs](https://developer.tekla.com/topic/en/18/43/a90914f1-1429-f8e1-bd91-94506e890222)

#### `SetAsCurrent(...)`

Sets this base point as current base point point until the token is disposed of. Can be used for example when retrieving report values according to this base point.

[Docs](https://developer.tekla.com/topic/en/18/43/35ba04bd-51ac-d3ed-cd44-0c518bbb97a4)

### Properties
- `AngleToNorth` (object, get/set) — Gets or sets the angle to north in radians.
- `CoordinateSystem` (object, get/set) — Gets or sets the coordinate system.
- `Description` (object, get/set) — Gets or sets the description.
- `EastWest` (object, get/set) — Gets or sets the east-west.
- `Elevation` (object, get/set) — Gets or sets the elevation.
- `Guid` (object, get/set) — Gets or sets the guid.
- `Id` (object, get/set) — Gets or sets the id.
- `InitialGuid` (object, get/set) — Gets or sets the initial guid. A metadata field to store for example some external applications base point guid. Not used in itself in any functionality.
- `IsCurrentBasePoint` (object, get/set) — Gets or sets the value indicating if this base point is the current base point.
- `IsLocked` (object, get/set) — Gets or sets the value indicating if this base point is locked.
- `IsProjectBasePoint` (object, get/set) — Gets or sets the value indicating if this base point is the project base point.
- `IsScopedCurrentBasePoint` (object, get/set) — Gets the value indicating whether this base point is the scoped current base point. That means, a temporary base point set with the SetAsCurrent method, and changed back when returned token is disposed of.
- `Latitude` (object, get/set) — Gets or sets the latitude.
- `LocationInModelX` (object, get/set) — Gets or sets the location in model x.
- `LocationInModelY` (object, get/set) — Gets or sets the location in model y.
- `LocationInModelZ` (object, get/set) — Gets or sets the location in model z.
- `Longitude` (object, get/set) — Gets or sets the longitude.
- `Name` (object, get/set) — Gets or sets the name.
- `NorthSouth` (object, get/set) — Gets or sets the north-south.

## BasePoint.CoordinateSystemType (enum)

The type of the base points coordinate system.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/f91ad4ef-a27d-ecfe-2d60-5cc8fcdfbdd3)

### Values
- `GLOBAL` = `1` — Coordinate system in global.
- `WORKPLANE` = `2` — Coordinate system in work plane.

## BaseRebarGroup (class)

The BaseRebarGroup class is an abstract base class that represents reinforcing bar group classes. The reinforcing bar groups derived from the base group are the rebar group, the circled rebar group and the curved rebar group.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/2102980d-739e-ab9a-1311-c4c494dc6d42)

### Methods
#### `CompareTo(...)`

Compares Identifiers of model objects.

[Docs](https://developer.tekla.com/topic/en/18/43/79cca356-0c8a-e8fb-463a-34ad4141bec7)

#### `Delete(...)`

Deletes the instance from the model database.

[Docs](https://developer.tekla.com/topic/en/18/43/0b922776-ccee-4b5e-4a12-4e5306802e90)

#### `Equals(...)`

Check if Identifiers of model objects are same.

[Docs](https://developer.tekla.com/topic/en/18/43/e4324265-2490-00de-4139-63a8f7acb492)

#### `GetAllReportProperties(...)`

Retrieves all properties for the model object in one hashtable. Type for the returned value must be checked using type casting.

[Docs](https://developer.tekla.com/topic/en/18/43/0708f404-ce77-28e1-4ebe-fa906f68bff8)

#### `GetAllUserProperties(...)`

Retrieves all properties for the model object in one hashtable. Type for the returned value must be checked using type casting.

[Docs](https://developer.tekla.com/topic/en/18/43/63b853a0-0af5-af9c-3ce7-05299664d7f8)

#### `GetAssembly(...)`

Returns the assembly that the reinforcement belongs to.

[Docs](https://developer.tekla.com/topic/en/18/43/322f932e-f09b-df0d-c1c8-3de07d79afeb)

#### `GetChildren(...)`

Returns an enumerator of all the children model objects.

[Docs](https://developer.tekla.com/topic/en/18/43/ce51ec40-310c-6067-a89f-f8d029aab747)

#### `GetCoordinateSystem(...)`

Returns the coordinate system for the given model object.

[Docs](https://developer.tekla.com/topic/en/18/43/8adfe9cd-f6e5-6474-1297-33c6270a7b0a)

#### `GetCustomObjectType(...)`

Gets custom object type name from an existing object.

[Docs](https://developer.tekla.com/topic/en/18/43/d3be2ba3-8533-7165-f2d3-a5aa9df556b8)

#### `GetDoubleReportProperties(...)`

Retrieves multiple double report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/f86f6c96-f738-49ca-16ba-f5634e862ba2)

#### `GetDoubleUserProperties(...)`

Retrieves all double properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/6ead7a63-5f76-0631-d480-d46c27e882aa)

#### `GetDynamicStringProperty(...)`

Gets a dynamic string property from the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/415e86d9-7de6-ea53-ce68-20b3e67b8655)

#### `GetFatherComponent(...)`

Returns the father component of the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/a2c28f3f-52ff-50a8-1b32-10d18db5c31d)

#### `GetFatherPour(...)`

Returns the pour that the rebar is associated to.

[Docs](https://developer.tekla.com/topic/en/18/43/0725e80c-89cf-d6d7-ac40-afdf75f6e695)

#### `GetFatherPourUnit(...)`

Returns the pour unit that the rebar is associated to.

[Docs](https://developer.tekla.com/topic/en/18/43/135a51ac-86cb-ef86-cfda-d2bc53a0d8b8)

#### `GetHierarchicObjects(...)`

Returns an enumerator of all the connected hierarchic objects.

[Docs](https://developer.tekla.com/topic/en/18/43/cc228ea0-267b-7d90-7441-c9efc1779b08)

#### `GetIntegerReportProperties(...)`

Retrieves multiple integer report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/2623a9c0-2a9c-e233-9daa-59f92ed51445)

#### `GetIntegerUserProperties(...)`

Retrieves all integer properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/1b3c6f06-bc25-a54e-2a88-b60dc4a3c85e)

#### `GetNumberOfRebars(...)`

Returns the number of rebars in the reinforcing group.

[Docs](https://developer.tekla.com/topic/en/18/43/6b02fd6f-4f84-6550-69c4-bd2f1117b922)

#### `GetPhase(...)`

Retrieves the phase of the model object (the phase number, the phase name, the phase comment and whether the phase is the current one or not).

[Docs](https://developer.tekla.com/topic/en/18/43/310f8b7a-c120-753a-35c9-f7c73a9806c9)

#### `GetRebarComplexGeometries(Boolean, Boolean, Boolean)(...)`

Retrieves a list of physical reinforcing bars (of type RebarComplexGeometry). These objects contain physical curves in the 3D space of each reinforcing bar as shown in model view. RebarComplexGeometry only differs from RebarGeometry by containing a polycurve rather than a polyline.

[Docs](https://developer.tekla.com/topic/en/18/43/c796aafb-3520-fe06-12e3-a07a87c8f2f9)

#### `GetRebarComplexGeometries(Boolean, Boolean, Boolean, Reinforcement.RebarGeometrySimplificationTypeEnum)(...)`

Retrieves a list of physical reinforcing bars (of type RebarComplexGeometry). These objects contain physical curves in the 3D space of each reinforcing bar as shown in model view. RebarComplexGeometry only differs from RebarGeometry by containing a polycurve (of Arc and LineSegment objects) rather than a polyline. The polycurve only contains arcs if the "simplified" parameter is RATIONALIZED or FABRICATION. If "simplified" is NONE, any arcs will be returned as a series of line segments.

[Docs](https://developer.tekla.com/topic/en/18/43/84a3a38b-d7eb-97df-7fba-a34d13de6610)

#### `GetRebarGeometries(Boolean)(...)`

Retrieves a list of physical reinforcing bars (of type RebarGeometry). These objects contain physical points in the 3D space of each reinforcing bar.

[Docs](https://developer.tekla.com/topic/en/18/43/75649a4c-6252-09a1-b509-cd815e718c0f)

#### `GetRebarGeometries(Reinforcement.RebarGeometryOptionEnum)(...)`

Retrieves a list of physical reinforcing bars (of type RebarGeometry). These objects contain physical points in the 3D space of each reinforcing bar.

[Docs](https://developer.tekla.com/topic/en/18/43/bc45309d-fc4b-9bf9-c640-e71bb1ade771)

#### `GetRebarGeometriesWithoutClashes(...)`

Retrieves a list of physical reinforcing bars (of type RebarGeometry). These objects contain physical points in the 3D space of each reinforcing bar as shown in model view. In case rebar polygon clashes with itself, physical points are moved to avoid clashing.

[Docs](https://developer.tekla.com/topic/en/18/43/470acf1f-1beb-f30a-a1c2-2fcab066ab4a)

#### `GetReportProperty(String, Double.)(...)`

Retrieves a double property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/1c05aa56-5f42-be91-a59e-8d59a5549f45)

#### `GetReportProperty(String, Int32.)(...)`

Retrieves an integer property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/ce8e4ca0-3750-3c9b-5ca8-27e48b45e0bd)

#### `GetReportProperty(String, String.)(...)`

Retrieves a string property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/70f212bb-ee70-e9e9-7ab6-d6a9d46f8de6)

#### `GetSingleRebar(...)`

Returns a single rebar inside the rebar group located by the given index. The indexing starts from the start point. The rebar represents a physical reinforcing bar and contains physical points in the 3D space of the bar. The method returns null on error, for example if given an erroneous index. The number of rebars in the group can be requested with GetNumberOfRebars().

[Docs](https://developer.tekla.com/topic/en/18/43/41181569-65f8-f4ba-7722-3cd890c7a3d4)

#### `GetSingleRebarWithoutClash(...)`

Returns a single rebar inside the rebar group located by the given index. The indexing starts from the start point. The rebar represents a physical reinforcing bar and contains physical, non-clashing points in the 3D space of the bar. The method returns null on error, for example if given an erroneous index. The number of rebars in the group can be requested with GetNumberOfRebars().

[Docs](https://developer.tekla.com/topic/en/18/43/8fd2e21a-3d2f-6211-92ba-9d6e652ac100)

#### `GetSolid(...)`

Method for getting the solid information of the reinforcement.

[Docs](https://developer.tekla.com/topic/en/18/43/6717d119-97da-5554-f359-49ee55e65a63)

#### `GetStringReportProperties(...)`

Retrieves multiple string report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/5ff72da1-7382-f5d2-eb1d-c99b46239d42)

#### `GetStringUserProperties(...)`

Retrieves all string properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/67120e84-c820-9d52-12ae-c167c595842c)

#### `GetUserProperty(String, Double.)(...)`

Retrieves a double property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/17cc8074-7955-32b5-2b4b-b169c45d4ee0)

#### `GetUserProperty(String, Int32.)(...)`

Retrieves an integer property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/d9364c1d-5b57-6a31-4c13-01124058cfb2)

#### `GetUserProperty(String, String.)(...)`

Retrieves a string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/214a886c-c18c-9b66-d7c9-fe84260962f2)

#### `Insert(...)`

Inserts the model object instance into the model database.

[Docs](https://developer.tekla.com/topic/en/18/43/f17ed840-1d16-b843-4b0c-270875de1d35)

#### `IsGeometryValid(...)`

Tells whether the geometry of a reinforcement object is valid or not.

[Docs](https://developer.tekla.com/topic/en/18/43/5699f6a6-4e39-b784-d84f-45eb7b1d0e37)

#### `Modify(...)`

Modifies the model instance in the model database.

[Docs](https://developer.tekla.com/topic/en/18/43/186833b8-4837-ed77-b314-3ebf9dc32d6a)

#### `Select(...)`

Selects the model object instance from the model database.

[Docs](https://developer.tekla.com/topic/en/18/43/f662ab10-be15-6280-d0aa-fc259dfd7c07)

#### `SetCustomObjectType(...)`

Sets a custom object type name for an existing object.

[Docs](https://developer.tekla.com/topic/en/18/43/3d6fb91b-48a9-35ca-d498-526514344089)

#### `SetDynamicStringProperty(...)`

Sets a dynamic string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/4dab0254-09e5-5e61-c28b-1c12afab71a0)

#### `SetLabel(...)`

Sets a label for an object when a new instance is created, this method must be called before Insert. The label is used in plug-ins for identifying the changed object in modification.

[Docs](https://developer.tekla.com/topic/en/18/43/3efcfc44-613a-e66b-9ebd-9f86df4f4036)

#### `SetPhase(...)`

Sets the phase of the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/ed80abcc-0ff7-d081-2229-e651f858a727)

#### `SetUserProperties(...)`

Sets multiple properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/8659da37-d571-f03f-c666-5c9fa3b113f5)

#### `SetUserProperty(String, Double)(...)`

Sets a double property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/5c585b8b-5c2e-7b75-2242-758f17429396)

#### `SetUserProperty(String, Int32)(...)`

Sets an integer property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/dd5b828d-3a87-bb58-5ac9-e1c3b4c2fe4b)

#### `SetUserProperty(String, String)(...)`

Sets a string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/80bf0348-b651-3d4a-862b-49c85bc924ab)

### Properties
- `Class` (object, get/set) — Gets or sets the class of the reinforcement. The class is used to group reinforcements.
- `EndFromPlaneOffset` (object, get/set) — The end offset value from the part surface.
- `EndHook` (object, get/set) — The hook at the end of the reinforcing bar.
- `EndPoint` (object, get/set) — The end point of the direction in which the bars are distributed.
- `EndPointOffsetType` (object, get/set) — Gets or sets the type of the end point offset. The options are: OFFSET_TYPE_LEG_LENGTHOFFSET_TYPE_COVER_THICKNESS
- `EndPointOffsetValue` (object, get/set) — Gets or sets the concrete cover thickness or leg length at the second end of the bar.
- `ExcludeType` (object, get/set) — Defines which bars to omit from the group. The options are: EXCLUDE_TYPE_NONEEXCLUDE_TYPE_FIRSTEXCLUDE_TYPE_LASTEXCLUDE_TYPE_BOTH
- `Father` (object, get/set) — Gets or sets the father object of the reinforcement; the model object instance to operate on.
- `FromPlaneOffset` (object, get/set) — The offset value from the part surface applied in both sides.
- `Grade` (object, get/set) — Gets or sets the steel grade of the reinforcing bar. The grade indicates the strength of the steel used in reinforcing bars. It can also indicate other factors, such as the weldability or surface deformations of the bar.
- `Identifier` (object, get/set) — The identifier of the object.
- `InputPointDeformingState` (object, get/set) — Gets or sets the reinforcement input point deforming state.
- `IsUpToDate` (object, get/set) — Gets if the object does not have a modification which is not shared.
- `ModificationTime` (object, get/set) — Gets latest time of the object was modified or created.
- `Name` (object, get/set) — Gets or sets the name of the reinforcement.
- `NumberingSeries` (object, get/set) — Gets or sets the numbering series of the reinforcement.
- `OnPlaneOffsets` (object, get/set) — Gets or sets the double offset value for each leg on the same plane as the bar.
- `RadiusValues` (object, get/set) — Gets or sets the radius value(s) of the bends in the bar.
- `Size` (object, get/set) — The size of the reinforcement.
- `Spacings` (object, get/set) — The spacing value(s). If the type of the spacing is SPACING_TYPE_EXACT_NUMBER Spacings has only one value that defines the number of the reinforcing bars.
- `SpacingType` (object, get/set) — The type of spacing. The options are (BaseRebarGroup.RebarGroupSpacingTypeEnum.): SPACING_TYPE_UNDEFINEDSPACING_TYPE_EXACT_SPACINGSSPACING_TYPE_EXACT_NUMBERSPACING_TYPE_TARGET_SPACESPACING_TYPE_EXACT_SPACE_FLEX_AT_STARTSPACING_TYPE_EXACT_SPACE_FLEX_AT_ENDSPACING_TYPE_EXACT_SPACE_FLEX_AT_BOTHSPACING_TYPE_EXACT_SPACE_FLEX_AT_MIDDLE
- `StartFromPlaneOffset` (object, get/set) — The start offset value from the part surface.
- `StartHook` (object, get/set) — The hook at the beginning of the reinforcing bar.
- `StartPoint` (object, get/set) — The start point of the direction in which the bars are distributed.
- `StartPointOffsetType` (object, get/set) — Gets or sets the type of the start point offset is either OFFSET_TYPE_LEG_LENGTH or OFFSET_TYPE_COVER_THICKNESS.
- `StartPointOffsetValue` (object, get/set) — Gets or sets the concrete cover thickness or leg length at the first end of the bar.

## BaseRebarGroup.ExcludeTypeEnum (enum)

The different ways to exclude reinforcing bars from the group.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/46465095-11b7-c672-27b6-5cf5578ea1fc)

### Values
- `EXCLUDE_TYPE_NONE` = `1` — All the reinforcing bars are included.
- `EXCLUDE_TYPE_FIRST` = `2` — The first reinforcing bar is not to be created to the group.
- `EXCLUDE_TYPE_LAST` = `3` — The last reinforcing bar is not to be created to the group.
- `EXCLUDE_TYPE_BOTH` = `4` — The first and last reinforcing bars are not to be created to the group.

## BaseRebarGroup.RebarGroupSpacingTypeEnum (enum)

The ways to distribute the reinforcing bars with different spacings.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/30e7f555-8816-41d5-f79f-e902c423bc81)

### Values
- `SPACING_TYPE_UNDEFINED` = `0` — The undefined spacing type.
- `SPACING_TYPE_EXACT_SPACINGS` = `1` — The bars are distributed using the spacing values.
- `SPACING_TYPE_EXACT_NUMBER` = `2` — Equal distribution by the number of reinforcing bars.
- `SPACING_TYPE_TARGET_SPACE` = `3` — Equal distribution by the target spacing value.
- `SPACING_TYPE_EXACT_SPACE_FLEX_AT_START` = `4` — Distribution by the exact spacing value with a flexible first space.
- `SPACING_TYPE_EXACT_SPACE_FLEX_AT_END` = `5` — Distribution by the exact spacing value with a flexible last space.
- `SPACING_TYPE_EXACT_SPACE_FLEX_AT_BOTH` = `6` — Distribution by the exact spacing value with flexible first and last spaces.
- `SPACING_TYPE_EXACT_SPACE_FLEX_AT_MIDDLE` = `7` — Distribution by the exact spacing value with a flexible middle space.

## BaseRebarModifier (class)

Represents a modifier that can modify the characteristics of RebarSet reinforcing bars that pass through it.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/f2d8320f-58b5-3669-bde6-a6bdf71d6062)

### Methods
#### `CompareTo(...)`

Compares Identifiers of model objects.

[Docs](https://developer.tekla.com/topic/en/18/43/79cca356-0c8a-e8fb-463a-34ad4141bec7)

#### `Delete(...)`

Deletes the rebar modifier from the model database.

[Docs](https://developer.tekla.com/topic/en/18/43/251dbf0c-4b13-fcc8-f4da-0a53618c9248)

#### `Equals(...)`

Check if Identifiers of model objects are same.

[Docs](https://developer.tekla.com/topic/en/18/43/e4324265-2490-00de-4139-63a8f7acb492)

#### `GetAllReportProperties(...)`

Retrieves all properties for the model object in one hashtable. Type for the returned value must be checked using type casting.

[Docs](https://developer.tekla.com/topic/en/18/43/0708f404-ce77-28e1-4ebe-fa906f68bff8)

#### `GetAllUserProperties(...)`

Retrieves all properties for the model object in one hashtable. Type for the returned value must be checked using type casting.

[Docs](https://developer.tekla.com/topic/en/18/43/63b853a0-0af5-af9c-3ce7-05299664d7f8)

#### `GetChildren(...)`

Returns an enumerator of all the children model objects.

[Docs](https://developer.tekla.com/topic/en/18/43/ce51ec40-310c-6067-a89f-f8d029aab747)

#### `GetCoordinateSystem(...)`

Returns the coordinate system for the given model object.

[Docs](https://developer.tekla.com/topic/en/18/43/8adfe9cd-f6e5-6474-1297-33c6270a7b0a)

#### `GetCustomObjectType(...)`

Gets custom object type name from an existing object.

[Docs](https://developer.tekla.com/topic/en/18/43/d3be2ba3-8533-7165-f2d3-a5aa9df556b8)

#### `GetDoubleReportProperties(...)`

Retrieves multiple double report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/f86f6c96-f738-49ca-16ba-f5634e862ba2)

#### `GetDoubleUserProperties(...)`

Retrieves all double properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/6ead7a63-5f76-0631-d480-d46c27e882aa)

#### `GetDynamicStringProperty(...)`

Gets a dynamic string property from the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/415e86d9-7de6-ea53-ce68-20b3e67b8655)

#### `GetFatherComponent(...)`

Returns the father component of the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/a2c28f3f-52ff-50a8-1b32-10d18db5c31d)

#### `GetHierarchicObjects(...)`

Returns an enumerator of all the connected hierarchic objects.

[Docs](https://developer.tekla.com/topic/en/18/43/cc228ea0-267b-7d90-7441-c9efc1779b08)

#### `GetIntegerReportProperties(...)`

Retrieves multiple integer report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/2623a9c0-2a9c-e233-9daa-59f92ed51445)

#### `GetIntegerUserProperties(...)`

Retrieves all integer properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/1b3c6f06-bc25-a54e-2a88-b60dc4a3c85e)

#### `GetPhase(...)`

Retrieves the phase of the model object (the phase number, the phase name, the phase comment and whether the phase is the current one or not).

[Docs](https://developer.tekla.com/topic/en/18/43/310f8b7a-c120-753a-35c9-f7c73a9806c9)

#### `GetReportProperty(String, Double.)(...)`

Retrieves a double property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/1c05aa56-5f42-be91-a59e-8d59a5549f45)

#### `GetReportProperty(String, Int32.)(...)`

Retrieves an integer property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/ce8e4ca0-3750-3c9b-5ca8-27e48b45e0bd)

#### `GetReportProperty(String, String.)(...)`

Retrieves a string property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/70f212bb-ee70-e9e9-7ab6-d6a9d46f8de6)

#### `GetStringReportProperties(...)`

Retrieves multiple string report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/5ff72da1-7382-f5d2-eb1d-c99b46239d42)

#### `GetStringUserProperties(...)`

Retrieves all string properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/67120e84-c820-9d52-12ae-c167c595842c)

#### `GetUserProperty(String, Double.)(...)`

Retrieves a double property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/17cc8074-7955-32b5-2b4b-b169c45d4ee0)

#### `GetUserProperty(String, Int32.)(...)`

Retrieves an integer property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/d9364c1d-5b57-6a31-4c13-01124058cfb2)

#### `GetUserProperty(String, String.)(...)`

Retrieves a string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/214a886c-c18c-9b66-d7c9-fe84260962f2)

#### `Insert(...)`

Inserts the rebar modifier into the model database.

[Docs](https://developer.tekla.com/topic/en/18/43/a9c11ac6-779a-e098-7069-b0757f11600e)

#### `Modify(...)`

Modifies the existing rebar modifier in the model database to match the current one.

[Docs](https://developer.tekla.com/topic/en/18/43/19951732-1fda-3890-9d60-60be06346c83)

#### `Select(...)`

Selects the rebar modifier from the model database. The identifier must be set.

[Docs](https://developer.tekla.com/topic/en/18/43/3798b590-971f-97f2-8884-72b1eefab6c6)

#### `SetCustomObjectType(...)`

Sets a custom object type name for an existing object.

[Docs](https://developer.tekla.com/topic/en/18/43/3d6fb91b-48a9-35ca-d498-526514344089)

#### `SetDynamicStringProperty(...)`

Sets a dynamic string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/4dab0254-09e5-5e61-c28b-1c12afab71a0)

#### `SetLabel(...)`

Sets a label for an object when a new instance is created, this method must be called before Insert. The label is used in plug-ins for identifying the changed object in modification.

[Docs](https://developer.tekla.com/topic/en/18/43/3efcfc44-613a-e66b-9ebd-9f86df4f4036)

#### `SetPhase(...)`

Sets the phase of the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/ed80abcc-0ff7-d081-2229-e651f858a727)

#### `SetUserProperties(...)`

Sets multiple properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/8659da37-d571-f03f-c666-5c9fa3b113f5)

#### `SetUserProperty(String, Double)(...)`

Sets a double property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/5c585b8b-5c2e-7b75-2242-758f17429396)

#### `SetUserProperty(String, Int32)(...)`

Sets an integer property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/dd5b828d-3a87-bb58-5ac9-e1c3b4c2fe4b)

#### `SetUserProperty(String, String)(...)`

Sets a string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/80bf0348-b651-3d4a-862b-49c85bc924ab)

### Properties
- `BarsAffected` (object, get/set) — Gets or sets the reinforcing bars affected.
- `Curve` (object, get/set) — Gets or sets the geometric Contour of the rebar modifier.
- `Father` (object, get/set) — Gets or sets the RebarSet to which the modifier belongs. Only reinforcing bars generated by this RebarSet can be modified by this rebar modifier.
- `FirstAffectedBar` (object, get/set) — Gets or sets the first affected bar. If set to zero, the first affected bar will be chosen automatically.
- `FollowEdges` (object, get/set) — Gets or sets a flag that indicates whether the modifier should attempt to follow the leg face edges located between its end points.
- `Identifier` (object, get/set) — The identifier of the object.
- `IsUpToDate` (object, get/set) — Gets if the object does not have a modification which is not shared.
- `ModificationTime` (object, get/set) — Gets latest time of the object was modified or created.

## BaseRebarModifier.AffectedRebarEnum (enum)

This enum allows the caller to specify which end of the bar is of interest when calling the function GetAffectedBars.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/240f68e0-8f7f-6483-bfc6-d46137879691)

### Values
- `START` = `0` — Gets bars that are affected at their start.
- `END` = `1` — Gets bars that affected at their end.
- `EITHER` = `2` — Gets all bars affected by the modifier.

## BaseRebarModifier.BarsAffectedEnum (enum)

The reinforcing bars affected.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/68f81d64-7286-6118-c2e4-f659b15a0c06)

### Values
- `ALL_BARS` = `0` — All reinforcing bars.
- `EVERY_SECOND_BAR` = `1` — Every second reinforcing bar.
- `EVERY_THIRD_BAR` = `2` — Every third reinforcing bar.
- `EVERY_FOURTH_BAR` = `3` — Every fourth reinforcing bar.

## BaseWeld (class)

The BaseWeld abstract class defines a weld between two model objects.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/173a061a-4957-ea9e-e87e-3949c3c7a653)

### Methods
#### `CompareTo(...)`

Compares Identifiers of model objects.

[Docs](https://developer.tekla.com/topic/en/18/43/79cca356-0c8a-e8fb-463a-34ad4141bec7)

#### `Delete(...)`

Deletes the instance from the model database.

[Docs](https://developer.tekla.com/topic/en/18/43/0b922776-ccee-4b5e-4a12-4e5306802e90)

#### `Equals(...)`

Check if Identifiers of model objects are same.

[Docs](https://developer.tekla.com/topic/en/18/43/e4324265-2490-00de-4139-63a8f7acb492)

#### `GetAllReportProperties(...)`

Retrieves all properties for the model object in one hashtable. Type for the returned value must be checked using type casting.

[Docs](https://developer.tekla.com/topic/en/18/43/0708f404-ce77-28e1-4ebe-fa906f68bff8)

#### `GetAllUserProperties(...)`

Retrieves all properties for the model object in one hashtable. Type for the returned value must be checked using type casting.

[Docs](https://developer.tekla.com/topic/en/18/43/63b853a0-0af5-af9c-3ce7-05299664d7f8)

#### `GetChildren(...)`

Returns an enumerator of all the children model objects.

[Docs](https://developer.tekla.com/topic/en/18/43/ce51ec40-310c-6067-a89f-f8d029aab747)

#### `GetCoordinateSystem(...)`

Returns the coordinate system for the given model object.

[Docs](https://developer.tekla.com/topic/en/18/43/8adfe9cd-f6e5-6474-1297-33c6270a7b0a)

#### `GetCustomObjectType(...)`

Gets custom object type name from an existing object.

[Docs](https://developer.tekla.com/topic/en/18/43/d3be2ba3-8533-7165-f2d3-a5aa9df556b8)

#### `GetDoubleReportProperties(...)`

Retrieves multiple double report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/f86f6c96-f738-49ca-16ba-f5634e862ba2)

#### `GetDoubleUserProperties(...)`

Retrieves all double properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/6ead7a63-5f76-0631-d480-d46c27e882aa)

#### `GetDynamicStringProperty(...)`

Gets a dynamic string property from the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/415e86d9-7de6-ea53-ce68-20b3e67b8655)

#### `GetFatherComponent(...)`

Returns the father component of the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/a2c28f3f-52ff-50a8-1b32-10d18db5c31d)

#### `GetHierarchicObjects(...)`

Returns an enumerator of all the connected hierarchic objects.

[Docs](https://developer.tekla.com/topic/en/18/43/cc228ea0-267b-7d90-7441-c9efc1779b08)

#### `GetIntegerReportProperties(...)`

Retrieves multiple integer report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/2623a9c0-2a9c-e233-9daa-59f92ed51445)

#### `GetIntegerUserProperties(...)`

Retrieves all integer properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/1b3c6f06-bc25-a54e-2a88-b60dc4a3c85e)

#### `GetPhase(...)`

Retrieves the phase of the model object (the phase number, the phase name, the phase comment and whether the phase is the current one or not).

[Docs](https://developer.tekla.com/topic/en/18/43/310f8b7a-c120-753a-35c9-f7c73a9806c9)

#### `GetReportProperty(String, Double.)(...)`

Retrieves a double property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/1c05aa56-5f42-be91-a59e-8d59a5549f45)

#### `GetReportProperty(String, Int32.)(...)`

Retrieves an integer property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/ce8e4ca0-3750-3c9b-5ca8-27e48b45e0bd)

#### `GetReportProperty(String, String.)(...)`

Retrieves a string property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/70f212bb-ee70-e9e9-7ab6-d6a9d46f8de6)

#### `GetSolid(...)`

Method for getting the weld solid.

[Docs](https://developer.tekla.com/topic/en/18/43/319baa05-e7f2-e9ff-1f4f-c2a403dd71d1)

#### `GetStringReportProperties(...)`

Retrieves multiple string report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/5ff72da1-7382-f5d2-eb1d-c99b46239d42)

#### `GetStringUserProperties(...)`

Retrieves all string properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/67120e84-c820-9d52-12ae-c167c595842c)

#### `GetUserProperty(String, Double.)(...)`

Retrieves a double property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/17cc8074-7955-32b5-2b4b-b169c45d4ee0)

#### `GetUserProperty(String, Int32.)(...)`

Retrieves an integer property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/d9364c1d-5b57-6a31-4c13-01124058cfb2)

#### `GetUserProperty(String, String.)(...)`

Retrieves a string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/214a886c-c18c-9b66-d7c9-fe84260962f2)

#### `GetWeldGeometries(...)`

Method for getting weld seam geometries. Every result represents one individual seam geometry in current weld.

[Docs](https://developer.tekla.com/topic/en/18/43/335a1753-ab21-bc4a-f02b-2dfa5e951c7a)

#### `Insert(...)`

Inserts the model object instance into the model database.

[Docs](https://developer.tekla.com/topic/en/18/43/f17ed840-1d16-b843-4b0c-270875de1d35)

#### `Modify(...)`

Modifies the model instance in the model database.

[Docs](https://developer.tekla.com/topic/en/18/43/186833b8-4837-ed77-b314-3ebf9dc32d6a)

#### `Select(...)`

Selects the model object instance from the model database.

[Docs](https://developer.tekla.com/topic/en/18/43/f662ab10-be15-6280-d0aa-fc259dfd7c07)

#### `SetCustomObjectType(...)`

Sets a custom object type name for an existing object.

[Docs](https://developer.tekla.com/topic/en/18/43/3d6fb91b-48a9-35ca-d498-526514344089)

#### `SetDynamicStringProperty(...)`

Sets a dynamic string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/4dab0254-09e5-5e61-c28b-1c12afab71a0)

#### `SetLabel(...)`

Sets a label for an object when a new instance is created, this method must be called before Insert. The label is used in plug-ins for identifying the changed object in modification.

[Docs](https://developer.tekla.com/topic/en/18/43/3efcfc44-613a-e66b-9ebd-9f86df4f4036)

#### `SetPhase(...)`

Sets the phase of the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/ed80abcc-0ff7-d081-2229-e651f858a727)

#### `SetUserProperties(...)`

Sets multiple properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/8659da37-d571-f03f-c666-5c9fa3b113f5)

#### `SetUserProperty(String, Double)(...)`

Sets a double property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/5c585b8b-5c2e-7b75-2242-758f17429396)

#### `SetUserProperty(String, Int32)(...)`

Sets an integer property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/dd5b828d-3a87-bb58-5ac9-e1c3b4c2fe4b)

#### `SetUserProperty(String, String)(...)`

Sets a string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/80bf0348-b651-3d4a-862b-49c85bc924ab)

### Properties
- `AdditionalSizeAbove` (object, get/set) — Gets or sets the additional size above for combination welds.
- `AdditionalSizeBelow` (object, get/set) — Gets or sets the additional size below for combination welds.
- `AngleAbove` (object, get/set) — Gets or sets the angle above.
- `AngleBelow` (object, get/set) — Gets or sets the angle below.
- `AroundWeld` (object, get/set) — Gets or sets a value indicating whether the weld is an around weld (true) or an edge weld (false).
- `ConnectAssemblies` (object, get/set) — Gets or sets a value indicating whether to connect a part or an assembly as a secondary part (false) or as a sub-assembly (true).
- `ContourAbove` (object, get/set) — Gets or sets the contour above.
- `ContourBelow` (object, get/set) — Gets or sets the contour below.
- `EffectiveThroatAbove` (object, get/set) — Gets or sets the effective throat above.
- `EffectiveThroatBelow` (object, get/set) — Gets or sets the effective throat below.
- `ElectrodeClassification` (object, get/set) — Gets or sets the weld electrode classification.
- `ElectrodeCoefficient` (object, get/set) — Gets or sets the electrode strength coefficient.
- `ElectrodeStrength` (object, get/set) — Gets or sets the electrode strength.
- `FinishAbove` (object, get/set) — Gets or sets the finish above.
- `FinishBelow` (object, get/set) — Gets or sets the finish below.
- `Identifier` (object, get/set) — The identifier of the object.
- `IncrementAmountAbove` (object, get/set) — Gets or sets the increment amount above.
- `IncrementAmountBelow` (object, get/set) — Gets or sets the increment amount below.
- `IntermittentType` (object, get/set) — Gets or sets the weld intermittent type.
- `IsUpToDate` (object, get/set) — Gets if the object does not have a modification which is not shared.
- `LengthAbove` (object, get/set) — Gets or sets the length above.
- `LengthBelow` (object, get/set) — Gets or sets the length below.
- `MainObject` (object, get/set) — Gets or sets the main part of the weld.
- `ModificationTime` (object, get/set) — Gets latest time of the object was modified or created.
- `NDTInspection` (object, get/set) — Gets or sets the NDT inspection level.
- `PitchAbove` (object, get/set) — Gets or sets the pitch above.
- `PitchBelow` (object, get/set) — Gets or sets the pitch below.
- `Placement` (object, get/set) — Gets or sets the weld placement.
- `PrefixAboveLine` (object, get/set) — Gets or sets the size prefix above the line.
- `PrefixBelowLine` (object, get/set) — Gets or sets the size prefix below the line.
- `Preparation` (object, get/set) — Gets or sets the weld preparation.
- `ProcessType` (object, get/set) — Gets or sets the process type.
- `ReferenceText` (object, get/set) — Gets or sets the reference text.
- `RootFaceAbove` (object, get/set) — Gets or sets the root face above.
- `RootFaceBelow` (object, get/set) — Gets or sets the root face below.
- `RootOpeningAbove` (object, get/set) — Gets or sets the root opening above.
- `RootOpeningBelow` (object, get/set) — Gets or sets the root opening below.
- `SecondaryObject` (object, get/set) — Gets or sets the secondary part of the weld.
- `ShopWeld` (object, get/set) — Gets or sets a value indicating whether the weld is a shop weld (true) or a site weld (false).
- `SizeAbove` (object, get/set) — Gets or sets the size above.
- `SizeBelow` (object, get/set) — Gets or sets the size below.
- `Standard` (object, get/set) — Gets or sets the weld detail/standard.
- `StitchWeld` (object, get/set) — Gets or sets a value indicating whether the weld is stitched (true) or not stitched (false).
- `TypeAbove` (object, get/set) — Gets or sets the type above.
- `TypeBelow` (object, get/set) — Gets or sets the type below.
- `WeldNumber` (object, get/set) — Gets the weld number.
- `WeldNumberPrefix` (object, get/set) — Gets or sets the weld number prefix.

## BaseWeld.WeldContourEnum (enum)

The weld contour.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/45f9adf5-15ab-ec1a-ee0f-d2630e1b191c)

### Values
- `WELD_CONTOUR_NONE` = `0` — No contour.
- `WELD_CONTOUR_FLUSH` = `1` — The flush contour.
- `WELD_CONTOUR_CONVEX` = `2` — The convex contour.
- `WELD_CONTOUR_CONCAVE` = `3` — The concave contour.

## BaseWeld.WeldElectrodeClassificationEnum (enum)

The weld electrode classification.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/30523b14-c433-6f00-03bb-ff542912152d)

### Values
- `WELD_ELECTRODE_CLASSIFICATION_NONE` = `0` — No electrode classification.
- `WELD_ELECTRODE_CLASSIFICATION_35` = `1` — The electrode classification 35.
- `WELD_ELECTRODE_CLASSIFICATION_42` = `2` — The electrode classification 42.
- `WELD_ELECTRODE_CLASSIFICATION_50` = `3` — The electrode classification 50.
- `WELD_ELECTRODE_CLASSIFICATION_E60XX` = `4` — The electrode classification E60XX.
- `WELD_ELECTRODE_CLASSIFICATION_E70XX` = `5` — The electrode classification E70XX.
- `WELD_ELECTRODE_CLASSIFICATION_E80XX` = `6` — The electrode classification E80XX.
- `WELD_ELECTRODE_CLASSIFICATION_E90XX` = `7` — The electrode classification E90XX.

## BaseWeld.WeldFinishEnum (enum)

The weld finish.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/703b6027-cf2b-26da-c3b9-32edb1fba8ce)

### Values
- `WELD_FINISH_NONE` = `0` — No weld finish.
- `WELD_FINISH_GRIND` = `1` — The grind finish.
- `WELD_FINISH_MACHINE` = `2` — The machine finish.
- `WELD_FINISH_CHIP` = `3` — The chip finish.
- `WELD_FINISH_FINISHED_WELD` = `4` — The normally treated finish.
- `WELS_FINISH_SMOOTH_TRANSITION` = `5` — The smooth transition finish.

## BaseWeld.WeldIntermittentTypeEnum (enum)

Weld intermittent types.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/fb0677a7-651a-7a60-af8b-8c9be1e73b29)

### Values
- `CONTINUOUS` = `0` — Continuous.
- `CHAIN_INTERMITTENT` = `1` — Chain intermittent.
- `STAGGERED_INTERMITTENT` = `2` — Staggered intermittent.

## BaseWeld.WeldNDTInspectionEnum (enum)

The weld NDT inspection level.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/6d6eec0c-e0d5-cdca-9d63-08477735be8b)

### Values
- `WELD_NDT_INSPECTION_NONE` = `0` — No NDT inspection level.
- `WELD_NDT_INSPECTION_A` = `1` — The NDT inspection level A.
- `WELD_NDT_INSPECTION_B` = `2` — The NDT inspection level B.
- `WELD_NDT_INSPECTION_C` = `3` — The NDT inspection level C.
- `WELD_NDT_INSPECTION_D` = `4` — The NDT inspection level D.
- `WELD_NDT_INSPECTION_E` = `5` — The NDT inspection level E.

## BaseWeld.WeldPlacementTypeEnum (enum)

Weld placement types.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/209c02ef-b1d1-5629-05ad-7f988523ce36)

### Values
- `PLACEMENT_AUTO` = `0` — Auto placement.
- `PLACEMENT_MAIN` = `1` — Main part placement.
- `PLACEMENT_SECONDARY` = `2` — Secondary part placement.

## BaseWeld.WeldPreparationTypeEnum (enum)

Weld preparation types.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/13526ea5-f367-ea94-529d-03e7bd7427ca)

### Values
- `PREPARATION_NONE` = `0` — No preparation.
- `PREPARATION_AUTO` = `1` — Auto preparation.
- `PREPARATION_MAIN` = `2` — Main part preparation.
- `PREPARATION_SECONDARY` = `3` — Secondary part preparation.

## BaseWeld.WeldProcessTypeEnum (enum)

The weld process type.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/81695cfc-b575-4541-c469-e4e20507b795)

### Values
- `WELD_PROCESS_NONE` = `0` — No process type.
- `WELD_PROCESS_SMAW` = `1` — The shielded metal arc (SMAW) process type.
- `WELD_PROCESS_SAW` = `2` — The submerged arc (SAW) process type.
- `WELD_PROCESS_GMAW` = `3` — The gas metal arc (GMAW) process type.
- `WELD_PROCESS_FCAW` = `4` — The flux cored arc (FCAW) process type.
- `WELD_PROCESS_ESW` = `5` — The electroslag (ESW) process type.
- `WELD_PROCESS_EGW` = `6` — The electrogas (EGW) process type.

## BaseWeld.WeldTypeEnum (enum)

The weld types.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/4371747e-9838-b687-e75b-06d27cbffb1e)

### Values
- `WELD_TYPE_NONE` = `0` — No weld type.
- `WELD_TYPE_EDGE_FLANGE` = `1` — The edge-flange weld.
- `WELD_TYPE_SQUARE_GROOVE_SQUARE_BUTT` = `2` — The square-groove (square butt) weld.
- `WELD_TYPE_BEVEL_GROOVE_SINGLE_V_BUTT` = `3` — The bevel-groove (single-V butt) weld.
- `WELD_TYPE_BEVEL_GROOVE_SINGLE_BEVEL_BUTT` = `4` — The bevel-groove (single-bevel butt) weld.
- `WELD_TYPE_SINGLE_V_BUTT_WITH_BROAD_ROOT_FACE` = `5` — The single-V butt weld with broad root face.
- `WELD_TYPE_SINGLE_BEVEL_BUTT_WITH_BROAD_ROOT_FACE` = `6` — The single-bevel butt weld with broad root face.
- `WELD_TYPE_U_GROOVE_SINGLE_U_BUTT` = `7` — The U-groove (single U-butt) weld.
- `WELD_TYPE_J_GROOVE_J_BUTT` = `8` — The J-groove (single J-butt) weld.
- `WELD_TYPE_BEVEL_BACKING` = `9` — The bevel backing weld.
- `WELD_TYPE_FILLET` = `10` — The fillet weld.
- `WELD_TYPE_PLUG` = `11` — The plug weld.
- `WELD_TYPE_SPOT` = `12` — The spot weld.
- `WELD_TYPE_SEAM` = `13` — The seam weld.
- `WELD_TYPE_SLOT` = `14` — The slot weld.
- `WELD_TYPE_FLARE_BEVEL_GROOVE` = `15` — The flare-bevel-groove weld.
- `WELD_TYPE_FLARE_V_GROOVE` = `16` — The flare V-groove weld.
- `WELD_TYPE_CORNER_FLANGE` = `17` — The corner-flange weld.
- `WELD_TYPE_PARTIAL_PENETRATION_SINGLE_BEVEL_BUTT_PLUS_FILLET` = `18` — The partial penetration (single-bevel butt + fillet) weld.
- `WELD_TYPE_PARTIAL_PENETRATION_SQUARE_GROOVE_PLUS_FILLET` = `19` — The partial penetration (square groove + fillet) weld.
- `WELD_TYPE_MELT_THROUGH` = `20` — The melt-through weld.
- `STEEP_FLANKED_BEVEL_GROOVE_SINGLE_V_BUTT` = `21` — The steep flanked bevel groove single-V butt weld.
- `STEEP_FLANKED_BEVEL_GROOVE_SINGLE_BEVEL_BUTT` = `22` — The steep flanked bevel groove single-bevel butt weld.
- `WELD_TYPE_EDGE` = `23` — The edge weld.
- `WELD_TYPE_ISO_SURFACING` = `24` — The ISO surfacing weld.
- `WELD_TYPE_FOLD` = `25` — The fold weld.
- `WELD_TYPE_INCLINED` = `26` — The inclined weld.

## Beam (class)

The Beam class represents a single beam in the model. A beam has a single start and end point.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/8621e658-a496-c1b9-49d0-4076c20779f7)

### Constructors
- `Beam(...)` — Initializes a new instance of the Beam class. The start and end points are in the origin.
- `Beam(...)` — Initializes a new instance of the Beam class by using the defined type.
- `Beam(...)` — Initializes a new instance of the Beam class with the given start and end points.

### Methods
#### `CompareTo(Object)(...)`

Compares Identifiers of model objects.

[Docs](https://developer.tekla.com/topic/en/18/43/79cca356-0c8a-e8fb-463a-34ad4141bec7)

#### `CompareTo(Part)(...)`

Compares the instantiated part with another one.

[Docs](https://developer.tekla.com/topic/en/18/43/e8632c0d-89cf-9aab-6532-f62f9873067b)

#### `Delete(...)`

Deletes the beam instance with the given identifier from the model database.

[Docs](https://developer.tekla.com/topic/en/18/43/5ed4ec9d-c075-7e42-af78-cbfb01556efa)

#### `Equals(...)`

Check if Identifiers of model objects are same.

[Docs](https://developer.tekla.com/topic/en/18/43/e4324265-2490-00de-4139-63a8f7acb492)

#### `GetAllReportProperties(...)`

Retrieves all properties for the model object in one hashtable. Type for the returned value must be checked using type casting.

[Docs](https://developer.tekla.com/topic/en/18/43/0708f404-ce77-28e1-4ebe-fa906f68bff8)

#### `GetAllUserProperties(...)`

Retrieves all properties for the model object in one hashtable. Type for the returned value must be checked using type casting.

[Docs](https://developer.tekla.com/topic/en/18/43/63b853a0-0af5-af9c-3ce7-05299664d7f8)

#### `GetAssembly(...)`

Returns the assembly that the part belongs to.

[Docs](https://developer.tekla.com/topic/en/18/43/bda18562-45a1-1355-42ed-6044af0b18cb)

#### `GetBolts(...)`

Returns an enumerator of all the connected bolts.

[Docs](https://developer.tekla.com/topic/en/18/43/dbc6b475-7461-7ec4-e8ec-7b7d3d8ff429)

#### `GetBooleans(...)`

Returns an enumerator of all the connected boolean objects.

[Docs](https://developer.tekla.com/topic/en/18/43/ae180e43-88e2-d759-0ae0-567d52681a2a)

#### `GetCenterLine(...)`

Returns the center line for the given part.

[Docs](https://developer.tekla.com/topic/en/18/43/bdc886b8-af7a-722c-dcb8-fddb2f250a83)

#### `GetChildren(...)`

Returns an enumerator of all the children model objects.

[Docs](https://developer.tekla.com/topic/en/18/43/ce51ec40-310c-6067-a89f-f8d029aab747)

#### `GetComponents(...)`

Returns an enumerator of all the connected components, connections, seams and details inherited from the base component.

[Docs](https://developer.tekla.com/topic/en/18/43/ed57941c-2cd8-3b78-6a6e-848b8a920298)

#### `GetCoordinateSystem(...)`

Returns the coordinate system for the given model object.

[Docs](https://developer.tekla.com/topic/en/18/43/8adfe9cd-f6e5-6474-1297-33c6270a7b0a)

#### `GetCustomObjectType(...)`

Gets custom object type name from an existing object.

[Docs](https://developer.tekla.com/topic/en/18/43/d3be2ba3-8533-7165-f2d3-a5aa9df556b8)

#### `GetDSTVCoordinateSystem(...)`

Get DSTV coordinate system.

[Docs](https://developer.tekla.com/topic/en/18/43/9e70005f-c701-ab99-8fa0-4fce47b1ab05)

#### `GetDoubleReportProperties(...)`

Retrieves multiple double report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/f86f6c96-f738-49ca-16ba-f5634e862ba2)

#### `GetDoubleUserProperties(...)`

Retrieves all double properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/6ead7a63-5f76-0631-d480-d46c27e882aa)

#### `GetDynamicStringProperty(...)`

Gets a dynamic string property from the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/415e86d9-7de6-ea53-ce68-20b3e67b8655)

#### `GetFatherComponent(...)`

Returns the father component of the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/a2c28f3f-52ff-50a8-1b32-10d18db5c31d)

#### `GetHierarchicObjects(...)`

Returns an enumerator of all the connected hierarchic objects.

[Docs](https://developer.tekla.com/topic/en/18/43/cc228ea0-267b-7d90-7441-c9efc1779b08)

#### `GetIntegerReportProperties(...)`

Retrieves multiple integer report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/2623a9c0-2a9c-e233-9daa-59f92ed51445)

#### `GetIntegerUserProperties(...)`

Retrieves all integer properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/1b3c6f06-bc25-a54e-2a88-b60dc4a3c85e)

#### `GetPartMark(...)`

Returns the part mark.

[Docs](https://developer.tekla.com/topic/en/18/43/9978ba28-6304-714d-815c-fe4a6bb77076)

#### `GetPhase(...)`

Retrieves the phase of the model object (the phase number, the phase name, the phase comment and whether the phase is the current one or not).

[Docs](https://developer.tekla.com/topic/en/18/43/310f8b7a-c120-753a-35c9-f7c73a9806c9)

#### `GetPours(...)`

Returns an enumerator of all the pours that the part belongs to.

[Docs](https://developer.tekla.com/topic/en/18/43/1cc67ebb-329a-4675-7857-af663602d21d)

#### `GetReferenceLine(...)`

Returns the reference line for the given part.

[Docs](https://developer.tekla.com/topic/en/18/43/3e0a42c9-f10c-4225-bcd6-2b9b61ecc743)

#### `GetReinforcements(...)`

Returns an enumerator of all the connected reinforcements.

[Docs](https://developer.tekla.com/topic/en/18/43/510c913a-0653-6777-1841-c4f7df4fee47)

#### `GetReportProperty(String, Double.)(...)`

Retrieves a double property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/1c05aa56-5f42-be91-a59e-8d59a5549f45)

#### `GetReportProperty(String, Int32.)(...)`

Retrieves an integer property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/ce8e4ca0-3750-3c9b-5ca8-27e48b45e0bd)

#### `GetReportProperty(String, String.)(...)`

Retrieves a string property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/70f212bb-ee70-e9e9-7ab6-d6a9d46f8de6)

#### `GetSolid(FormingStates)(...)`

Returns the solid of the part.

[Docs](https://developer.tekla.com/topic/en/18/43/1e4d454b-d3a0-4219-b4e2-7f661b402c4d)

#### `GetSolid(Solid.SolidCreationTypeEnum)(...)`

Returns the solid of the part.

[Docs](https://developer.tekla.com/topic/en/18/43/25f58541-37fe-c1ca-f384-065ecbe38e55)

#### `GetSolid.(...)`

Returns the solid of the part.

[Docs](https://developer.tekla.com/topic/en/18/43/7bd70e97-7c8b-823c-c7e1-da2ebe3dadaf)

#### `GetStringReportProperties(...)`

Retrieves multiple string report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/5ff72da1-7382-f5d2-eb1d-c99b46239d42)

#### `GetStringUserProperties(...)`

Retrieves all string properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/67120e84-c820-9d52-12ae-c167c595842c)

#### `GetSurfaceObjects(...)`

Returns an enumerator of all the connected surface objects.

[Docs](https://developer.tekla.com/topic/en/18/43/36268c4e-9006-1c9d-57bd-331f907da6ce)

#### `GetSurfaceTreatments(...)`

Returns an enumerator of all the connected surface treatments.

[Docs](https://developer.tekla.com/topic/en/18/43/26c1c1a5-e5db-9850-f739-c58bc05c963e)

#### `GetUserProperty(String, Double.)(...)`

Retrieves a double property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/17cc8074-7955-32b5-2b4b-b169c45d4ee0)

#### `GetUserProperty(String, Int32.)(...)`

Retrieves an integer property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/d9364c1d-5b57-6a31-4c13-01124058cfb2)

#### `GetUserProperty(String, String.)(...)`

Retrieves a string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/214a886c-c18c-9b66-d7c9-fe84260962f2)

#### `GetWelds(...)`

Returns an enumerator of all the connected welds.

[Docs](https://developer.tekla.com/topic/en/18/43/722de753-413b-f810-f09c-07388feb0afa)

#### `Insert(...)`

Inserts the beam into the model database. All the attributes must be set.

[Docs](https://developer.tekla.com/topic/en/18/43/ed57e866-7dad-e2d4-b656-c7aa27a0a602)

#### `Modify(...)`

Modifies the existing beam in the model database to match the current one.

[Docs](https://developer.tekla.com/topic/en/18/43/6fa9ceba-2444-c91d-cbbb-4320b4353edf)

#### `Select(...)`

Selects a beam from the model database. The beam identifier must be set.

[Docs](https://developer.tekla.com/topic/en/18/43/567849b2-1c58-6850-89ef-49ed11fdadde)

#### `SetCustomObjectType(...)`

Sets a custom object type name for an existing object.

[Docs](https://developer.tekla.com/topic/en/18/43/3d6fb91b-48a9-35ca-d498-526514344089)

#### `SetDynamicStringProperty(...)`

Sets a dynamic string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/4dab0254-09e5-5e61-c28b-1c12afab71a0)

#### `SetLabel(...)`

Sets a label for an object when a new instance is created, this method must be called before Insert. The label is used in plug-ins for identifying the changed object in modification.

[Docs](https://developer.tekla.com/topic/en/18/43/3efcfc44-613a-e66b-9ebd-9f86df4f4036)

#### `SetPhase(...)`

Sets the phase of the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/ed80abcc-0ff7-d081-2229-e651f858a727)

#### `SetUserProperties(...)`

Sets multiple properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/8659da37-d571-f03f-c666-5c9fa3b113f5)

#### `SetUserProperty(String, Double)(...)`

Sets a double property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/5c585b8b-5c2e-7b75-2242-758f17429396)

#### `SetUserProperty(String, Int32)(...)`

Sets an integer property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/dd5b828d-3a87-bb58-5ac9-e1c3b4c2fe4b)

#### `SetUserProperty(String, String)(...)`

Sets a string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/80bf0348-b651-3d4a-862b-49c85bc924ab)

### Properties
- `AssemblyNumber` (object, get/set) — Gets or sets the assembly number. Defines the numbering in the assembly sense.
- `CastUnitType` (object, get/set) — Gets or sets the cast unit type of the part.
- `Class` (object, get/set) — Gets or sets the class of the part.
- `DeformingData` (object, get/set) — Gets or sets the deforming data of the part.
- `EndPoint` (object, get/set) — Gets or sets the end point of the beam.
- `EndPointOffset` (object, get/set) — Gets or sets the beam's end point offset.
- `Finish` (object, get/set) — Gets or sets the finish of the part.
- `Identifier` (object, get/set) — The identifier of the object.
- `IsUpToDate` (object, get/set) — Gets if the object does not have a modification which is not shared.
- `Material` (object, get/set) — Gets or sets the material the part is made of.
- `ModificationTime` (object, get/set) — Gets latest time of the object was modified or created.
- `Name` (object, get/set) — Gets or sets the name of the part.
- `PartNumber` (object, get/set) — Gets or sets the part number. Defines the numbering in the part sense.
- `Position` (object, get/set) — Gets or sets the part position. Defines the way the part is positioned in the model.
- `PourPhase` (object, get/set) — Gets or sets the pour phase of the part.
- `Profile` (object, get/set) — Gets or sets the profile of the part.
- `StartPoint` (object, get/set) — Gets or sets the start point of the beam.
- `StartPointOffset` (object, get/set) — Gets or sets the beam's start point offset.
- `Type` (object, get/set) — Gets the read-only type of the beam. The default type for a new beam instance is beam.

## Beam.BeamTypeEnum (enum)

The beam types. The strip and pad footings are valid only if the material type of the beam is concrete.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/6151749e-0ba3-8d75-6127-b84f300dd30f)

### Values
- `BEAM` = `0` — The beam.
- `PANEL` = `1` — The panel.
- `STRIP_FOOTING` = `2` — The concrete strip footing.
- `PAD_FOOTING` = `3` — The concrete pad footing.
- `COLUMN` = `4` — The column.

## BendSurface (class)

The BendSurface class defines a surface contour that can be used as a bend between two plates

[Vendor docs](https://developer.tekla.com/topic/en/18/43/4b8c0f69-2f54-749d-5dcc-753991f7481f)

### Constructors
- `BendSurface(...)` — Initializes a new instance of the BendSurface class from a base arc, a height and a radius at the top. This is the recommended constructor if no intermediate points are desired along the lateral boundaries of the bend surface.
- `BendSurface(...)` — Initializes a new instance of the BendSurface class from the cone lateral boundaries and the center line. This is the recommended constructor if fine grained control is desired over the lateral boundaries of the surface
- `BendSurface(...)` — Initializes a new instance of the BendSurface class with given parameters. The boundaries are defined by the lateral boundaries (i.e. the points on the sides of the curved part of the surface). The side boundaries are defined by the two first points and the two last points of the lateral boundaries.
- `BendSurface(...)` — Initializes a new instance of the BendSurface class with given parameters. The boundaries are defined by the side boundaries (i.e. generator lines of the surface).

### Properties
- `CenterLine` (object, get/set) — Gets the center line of the bend surface (i.e. the line that crosses every circular cross section of the bend at their center point)
- `EndFaceNormal1` (object, get/set) — Gets the first end face normal.
- `EndFaceNormal2` (object, get/set) — Gets the second end face normal.
- `IntersectionLine` (object, get/set) — Gets intersection line.
- `InwardCurved` (object, get/set) — Gets the direction of the curve - true if the curve is oriented towards the intersection line, false otherwise.
- `LateralBoundary1` (object, get/set) — Gets or sets the first lateral boundary
- `LateralBoundary2` (object, get/set) — Gets or sets the second lateral boundary
- `RotationAxis` (object, get/set) — Gets the unit vector that defines the rotation axis (for counter clockwise rotations) of the lateral boundaries of the surface. This axis is always parallel to the direction of the center line.
- `SideBoundary1` (object, get/set) — Gets or sets the first side boundary.
- `SideBoundary2` (object, get/set) — Gets or sets the second side boundary.

## BendSurfaceNode (class)

The BendSurfaceNode class represents a bend surface geometry tree node.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/587d9a1d-e196-e65c-eaf5-130c137c8b60)

### Constructors
- `BendSurfaceNode(...)` — Initializes a new instance of the BendSurfaceNode class.

### Methods
#### `AcceptVisitor(...)`

Entry method for the visitor pattern

[Docs](https://developer.tekla.com/topic/en/18/43/df878108-a5a3-e0d1-8617-f114f2490b6d)

#### `Clone(...)`

Creates a new object that is a copy of the current instance.

[Docs](https://developer.tekla.com/topic/en/18/43/5cb016ba-46b5-d41d-8e6a-760ba327a50f)

### Properties
- `IsAutomatic` (object, get/set) — Gets a value indicating whether this geometry node was automatically generated (returns false if it was originally a user-defined part)
- `Surface` (object, get/set) — Gets the bend surface geometry.

## BentPlate (class)

A class for the bent plate

[Vendor docs](https://developer.tekla.com/topic/en/18/43/a75e430c-a12d-1753-65b0-f93472716a4a)

### Constructors
- `BentPlate(...)` — Initializes a new instance of the BentPlate class.

### Methods
#### `CompareTo(Object)(...)`

Compares Identifiers of model objects.

[Docs](https://developer.tekla.com/topic/en/18/43/79cca356-0c8a-e8fb-463a-34ad4141bec7)

#### `CompareTo(Part)(...)`

Compares the instantiated part with another one.

[Docs](https://developer.tekla.com/topic/en/18/43/e8632c0d-89cf-9aab-6532-f62f9873067b)

#### `Delete(...)`

Deletes the bent plate with this instance identifier from the model database.

[Docs](https://developer.tekla.com/topic/en/18/43/280f4717-b474-1518-63d7-399be45cbe7a)

#### `Equals(...)`

Check if Identifiers of model objects are same.

[Docs](https://developer.tekla.com/topic/en/18/43/e4324265-2490-00de-4139-63a8f7acb492)

#### `GetAllReportProperties(...)`

Retrieves all properties for the model object in one hashtable. Type for the returned value must be checked using type casting.

[Docs](https://developer.tekla.com/topic/en/18/43/0708f404-ce77-28e1-4ebe-fa906f68bff8)

#### `GetAllUserProperties(...)`

Retrieves all properties for the model object in one hashtable. Type for the returned value must be checked using type casting.

[Docs](https://developer.tekla.com/topic/en/18/43/63b853a0-0af5-af9c-3ce7-05299664d7f8)

#### `GetAssembly(...)`

Returns the assembly that the part belongs to.

[Docs](https://developer.tekla.com/topic/en/18/43/bda18562-45a1-1355-42ed-6044af0b18cb)

#### `GetBolts(...)`

Returns an enumerator of all the connected bolts.

[Docs](https://developer.tekla.com/topic/en/18/43/dbc6b475-7461-7ec4-e8ec-7b7d3d8ff429)

#### `GetBooleans(...)`

Returns an enumerator of all the connected boolean objects.

[Docs](https://developer.tekla.com/topic/en/18/43/ae180e43-88e2-d759-0ae0-567d52681a2a)

#### `GetCenterLine(...)`

Returns the center line for the given part.

[Docs](https://developer.tekla.com/topic/en/18/43/bdc886b8-af7a-722c-dcb8-fddb2f250a83)

#### `GetChildren(...)`

Returns an enumerator of all the children model objects.

[Docs](https://developer.tekla.com/topic/en/18/43/ce51ec40-310c-6067-a89f-f8d029aab747)

#### `GetComponents(...)`

Returns an enumerator of all the connected components, connections, seams and details inherited from the base component.

[Docs](https://developer.tekla.com/topic/en/18/43/ed57941c-2cd8-3b78-6a6e-848b8a920298)

#### `GetCoordinateSystem(...)`

Returns the coordinate system for the given model object.

[Docs](https://developer.tekla.com/topic/en/18/43/8adfe9cd-f6e5-6474-1297-33c6270a7b0a)

#### `GetCustomObjectType(...)`

Gets custom object type name from an existing object.

[Docs](https://developer.tekla.com/topic/en/18/43/d3be2ba3-8533-7165-f2d3-a5aa9df556b8)

#### `GetDSTVCoordinateSystem(...)`

Get DSTV coordinate system.

[Docs](https://developer.tekla.com/topic/en/18/43/9e70005f-c701-ab99-8fa0-4fce47b1ab05)

#### `GetDoubleReportProperties(...)`

Retrieves multiple double report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/f86f6c96-f738-49ca-16ba-f5634e862ba2)

#### `GetDoubleUserProperties(...)`

Retrieves all double properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/6ead7a63-5f76-0631-d480-d46c27e882aa)

#### `GetDynamicStringProperty(...)`

Gets a dynamic string property from the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/415e86d9-7de6-ea53-ce68-20b3e67b8655)

#### `GetFatherComponent(...)`

Returns the father component of the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/a2c28f3f-52ff-50a8-1b32-10d18db5c31d)

#### `GetHierarchicObjects(...)`

Returns an enumerator of all the connected hierarchic objects.

[Docs](https://developer.tekla.com/topic/en/18/43/cc228ea0-267b-7d90-7441-c9efc1779b08)

#### `GetIntegerReportProperties(...)`

Retrieves multiple integer report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/2623a9c0-2a9c-e233-9daa-59f92ed51445)

#### `GetIntegerUserProperties(...)`

Retrieves all integer properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/1b3c6f06-bc25-a54e-2a88-b60dc4a3c85e)

#### `GetPartMark(...)`

Returns the part mark.

[Docs](https://developer.tekla.com/topic/en/18/43/9978ba28-6304-714d-815c-fe4a6bb77076)

#### `GetPhase(...)`

Retrieves the phase of the model object (the phase number, the phase name, the phase comment and whether the phase is the current one or not).

[Docs](https://developer.tekla.com/topic/en/18/43/310f8b7a-c120-753a-35c9-f7c73a9806c9)

#### `GetPours(...)`

Returns an enumerator of all the pours that the part belongs to.

[Docs](https://developer.tekla.com/topic/en/18/43/1cc67ebb-329a-4675-7857-af663602d21d)

#### `GetReferenceLine(...)`

Returns the reference line for the given part.

[Docs](https://developer.tekla.com/topic/en/18/43/3e0a42c9-f10c-4225-bcd6-2b9b61ecc743)

#### `GetReinforcements(...)`

Returns an enumerator of all the connected reinforcements.

[Docs](https://developer.tekla.com/topic/en/18/43/510c913a-0653-6777-1841-c4f7df4fee47)

#### `GetReportProperty(String, Double.)(...)`

Retrieves a double property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/1c05aa56-5f42-be91-a59e-8d59a5549f45)

#### `GetReportProperty(String, Int32.)(...)`

Retrieves an integer property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/ce8e4ca0-3750-3c9b-5ca8-27e48b45e0bd)

#### `GetReportProperty(String, String.)(...)`

Retrieves a string property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/70f212bb-ee70-e9e9-7ab6-d6a9d46f8de6)

#### `GetSolid(FormingStates)(...)`

Returns the solid of the part.

[Docs](https://developer.tekla.com/topic/en/18/43/1e4d454b-d3a0-4219-b4e2-7f661b402c4d)

#### `GetSolid(Solid.SolidCreationTypeEnum)(...)`

Returns the solid of the part.

[Docs](https://developer.tekla.com/topic/en/18/43/25f58541-37fe-c1ca-f384-065ecbe38e55)

#### `GetSolid.(...)`

Returns the solid of the part.

[Docs](https://developer.tekla.com/topic/en/18/43/7bd70e97-7c8b-823c-c7e1-da2ebe3dadaf)

#### `GetStringReportProperties(...)`

Retrieves multiple string report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/5ff72da1-7382-f5d2-eb1d-c99b46239d42)

#### `GetStringUserProperties(...)`

Retrieves all string properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/67120e84-c820-9d52-12ae-c167c595842c)

#### `GetSurfaceObjects(...)`

Returns an enumerator of all the connected surface objects.

[Docs](https://developer.tekla.com/topic/en/18/43/36268c4e-9006-1c9d-57bd-331f907da6ce)

#### `GetSurfaceTreatments(...)`

Returns an enumerator of all the connected surface treatments.

[Docs](https://developer.tekla.com/topic/en/18/43/26c1c1a5-e5db-9850-f739-c58bc05c963e)

#### `GetUserProperty(String, Double.)(...)`

Retrieves a double property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/17cc8074-7955-32b5-2b4b-b169c45d4ee0)

#### `GetUserProperty(String, Int32.)(...)`

Retrieves an integer property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/d9364c1d-5b57-6a31-4c13-01124058cfb2)

#### `GetUserProperty(String, String.)(...)`

Retrieves a string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/214a886c-c18c-9b66-d7c9-fe84260962f2)

#### `GetWelds(...)`

Returns an enumerator of all the connected welds.

[Docs](https://developer.tekla.com/topic/en/18/43/722de753-413b-f810-f09c-07388feb0afa)

#### `Insert(...)`

Inserts the bent plate into the model database. All the attributes must be set and geometry should have at least one bend.

[Docs](https://developer.tekla.com/topic/en/18/43/cadf8e15-919e-7684-c67f-b1ced59d4695)

#### `Modify(...)`

Modifies the bent plate object values in the database.

[Docs](https://developer.tekla.com/topic/en/18/43/51152350-e0c8-0eef-4cff-fb918104fe06)

#### `Select(...)`

Selects a bent plate object from the database.

[Docs](https://developer.tekla.com/topic/en/18/43/4661012d-d390-dc54-272a-dcefca781148)

#### `SetCustomObjectType(...)`

Sets a custom object type name for an existing object.

[Docs](https://developer.tekla.com/topic/en/18/43/3d6fb91b-48a9-35ca-d498-526514344089)

#### `SetDynamicStringProperty(...)`

Sets a dynamic string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/4dab0254-09e5-5e61-c28b-1c12afab71a0)

#### `SetLabel(...)`

Sets a label for an object when a new instance is created, this method must be called before Insert. The label is used in plug-ins for identifying the changed object in modification.

[Docs](https://developer.tekla.com/topic/en/18/43/3efcfc44-613a-e66b-9ebd-9f86df4f4036)

#### `SetPhase(...)`

Sets the phase of the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/ed80abcc-0ff7-d081-2229-e651f858a727)

#### `SetUserProperties(...)`

Sets multiple properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/8659da37-d571-f03f-c666-5c9fa3b113f5)

#### `SetUserProperty(String, Double)(...)`

Sets a double property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/5c585b8b-5c2e-7b75-2242-758f17429396)

#### `SetUserProperty(String, Int32)(...)`

Sets an integer property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/dd5b828d-3a87-bb58-5ac9-e1c3b4c2fe4b)

#### `SetUserProperty(String, String)(...)`

Sets a string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/80bf0348-b651-3d4a-862b-49c85bc924ab)

### Properties
- `AssemblyNumber` (object, get/set) — Gets or sets the assembly number. Defines the numbering in the assembly sense.
- `CastUnitType` (object, get/set) — Gets or sets the cast unit type of the part.
- `Class` (object, get/set) — Gets or sets the class of the part.
- `DeformingData` (object, get/set) — Gets or sets the deforming data of the part.
- `Finish` (object, get/set) — Gets or sets the finish of the part.
- `Geometry` (object, get/set) — Gets or sets the geometry of the bent plate.
- `Identifier` (object, get/set) — The identifier of the object.
- `IsUpToDate` (object, get/set) — Gets if the object does not have a modification which is not shared.
- `Material` (object, get/set) — Gets or sets the material the part is made of.
- `ModificationTime` (object, get/set) — Gets latest time of the object was modified or created.
- `Name` (object, get/set) — Gets or sets the name of the part.
- `PartNumber` (object, get/set) — Gets or sets the part number. Defines the numbering in the part sense.
- `Position` (object, get/set) — Gets or sets the part position. Defines the way the part is positioned in the model.
- `PourPhase` (object, get/set) — Gets or sets the pour phase of the part.
- `Profile` (object, get/set) — Gets or sets the profile of the part.
- `Thickness` (object, get/set) — Gets thickness of the bent plate.

## BentPlate.BendShape (enum)

Enumerator to define different possibilities for bend shapes.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/b5343f3c-931a-194b-ccc5-e1c29895b276)

### Values
- `Cylindrical` = `0` — Cylindrical bend.
- `Conical` = `1` — Conical bend.

## BentPlateGeometrySolver (class)

BentPlateGeometrySolver is a solver class to handle ConnectiveGeometry modification related computations.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/6868795d-e6b6-4c94-a705-5e4972e535c9)

### Constructors
- `BentPlateGeometrySolver(...)` — Initializes a new instance of the BentPlateGeometrySolver class.

### Methods
#### `AddLeg(ConnectiveGeometry, Contour)(...)`

Appends a polygon section to ConnectiveGeometry, using the maximal radius for the cylindrical connection.

[Docs](https://developer.tekla.com/topic/en/18/43/a2759dfe-8879-387a-43c8-a30e76609c81)

#### `AddLeg(ConnectiveGeometry, Contour, BentPlate.BendShape)(...)`

Appends a polygon section to ConnectiveGeometry, using the maximal radius for the cylindrical connection, or the maximum cone aperture for conical connection.

[Docs](https://developer.tekla.com/topic/en/18/43/01139736-16e5-b503-b77c-2a989302924e)

#### `AddLeg(ConnectiveGeometry, Contour, Double)(...)`

Appends a polygon section to this ConnectiveGeometry, using the specified radius for the cylindrical connection.

[Docs](https://developer.tekla.com/topic/en/18/43/be684308-bd46-4b43-ab67-885df4db987f)

#### `AddLeg(ConnectiveGeometry, Contour, Double, Double)(...)`

Joins a polygon section to this ConnectiveGeometry, using the specified radius and aperture for the conical connection.

[Docs](https://developer.tekla.com/topic/en/18/43/005a09fb-64f4-0150-fdee-40ba1d2e009e)

#### `AddLeg(ConnectiveGeometry, LineSegment, Contour, LineSegment)(...)`

Appends a polygon section to this ConnectiveGeometry, using the specified connection points and maximal radius for the cylindrical connection.

[Docs](https://developer.tekla.com/topic/en/18/43/fe3ef7c1-95d0-b903-029f-3327db769d09)

#### `AddLeg(ConnectiveGeometry, LineSegment, Contour, LineSegment, BentPlate.BendShape)(...)`

Appends a polygon section to this ConnectiveGeometry, using the specified connection points and maximal radius for the cylindrical connection, or maximum aperture for conical connection.

[Docs](https://developer.tekla.com/topic/en/18/43/2ae3af3f-6bf8-1f24-e666-c55aca86f5c4)

#### `AddLeg(ConnectiveGeometry, LineSegment, Contour, LineSegment, Double)(...)`

Appends a polygon section to this ConnectiveGeometry, using the specified connection points and radius for the cylindrical connection.

[Docs](https://developer.tekla.com/topic/en/18/43/3381e0db-8efa-f82b-8322-f4a48ed1a265)

#### `AddLeg(ConnectiveGeometry, LineSegment, Contour, LineSegment, Double, Double)(...)`

Joins a polygon section to this ConnectiveGeometry, using the specified connection points and radius and aperture for the conical connection.

[Docs](https://developer.tekla.com/topic/en/18/43/253cf055-768d-0624-e258-2039cd78494d)

#### `ModifyBendSurface(...)`

Modifies the shape of a bend surface section, and updates related polygon sections accordingly.

[Docs](https://developer.tekla.com/topic/en/18/43/2d82eb90-c4a4-d473-b872-08e8522fa827)

#### `ModifyConicalRadiuses(...)`

Modify the radiuses of a conical section, preserving the length of the side boundaries. The order of the provided radiuses is not relevant, as the cone vertex will be located on the same side of the plates as the original cone section.

[Docs](https://developer.tekla.com/topic/en/18/43/e2a50415-93e4-cc52-e702-0bab551f0a2d)

#### `ModifyCylindricalSurface(...)`

Modifies the shape of a cylindrical surface section, and updates related polygon sections accordingly.

[Docs](https://developer.tekla.com/topic/en/18/43/9b20a6c3-4976-912f-a940-660056f80e3d)

#### `ModifyPolygon(...)`

Modify the shape of a polygon section, and updates related cylindrical sections accordingly.

[Docs](https://developer.tekla.com/topic/en/18/43/124f6069-da5d-82e5-0762-9266e5b30aed)

#### `ModifyRadius(...)`

Modify the radius of a cylindrical section.

[Docs](https://developer.tekla.com/topic/en/18/43/332d9017-9164-8a8b-5a1e-dc772f98674f)

#### `RemoveLeg(...)`

Removes a polygon section which is the end section, from this ConnectiveGeometry including extensions and cylindrical surface.

[Docs](https://developer.tekla.com/topic/en/18/43/5426185f-4351-d659-7842-8a1e6256eb25)

#### `ScaleConeSection(...)`

Scale the shape of a conical section.

[Docs](https://developer.tekla.com/topic/en/18/43/ca3720e0-be3e-6960-ca2f-151a15227447)

#### `SetBendAngle(...)`

Sets angle as the angle of curved section sectionToSetAngle and modifies geometry starting from sectionToMove.

[Docs](https://developer.tekla.com/topic/en/18/43/6480f6a8-c58d-c973-1e50-e90c49132b08)

#### `SetMainSection(...)`

Sets newMainSection as a main section of geometry.

[Docs](https://developer.tekla.com/topic/en/18/43/a9b78007-72d0-cdc5-7461-ffd5fc97c0d5)

#### `Split(...)`

Returns the resulting geometry or geometries when a section and the related automatically created sections are removed from this geometry.

[Docs](https://developer.tekla.com/topic/en/18/43/5cac6160-6cf1-1bfb-b4da-d9c72fff026a)

## BentPlateGeometrySolver.OperationStatus (enum)

Define possible statuses for geometry creation/modification commands.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/6e3965fc-cced-c0c2-3025-63df350c92bd)

### Values
- `Success` = `0` — Status on success creation.
- `Failure` = `1` — General failure mode.
- `FacePerpendicularToIntersectionLine` = `2` — Face perpendicular to intersection line.
- `PlateIntersectsWithIntersectionLine` = `3` — Plate polygon extends beyond intersection line.
- `ExtensionIntersectsWithPlate` = `4` — Extension plate clashes with plates.
- `FacesTooNearEachOther` = `5` — Faces to be connected overlap.
- `FacesAtAnObtuseAngle` = `6` — Faces to be connected are too steep of an angle.
- `UndefinedCurveDirection` = `7` — Can't make inward/outward curve check.
- `UnsupportedChamfer` = `8` — Plates containing unsupported chamfer information.
- `InvalidRadius` = `9` — Cannot connect parts with a cylindrical section having the provided radius.
- `InvalidFacePoints` = `10` — Cannot create geometry from requested face points.

## BoltArray (class)

The BoltArray class defines a bolt group with an array shape.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/caa3aee3-d7ae-6c96-bce9-eaab89a1cad5)

### Constructors
- `BoltArray(...)` — Creates a new bolt array instance.

### Methods
#### `AddBoltDistX(...)`

Adds an X distance to the bolt array.

[Docs](https://developer.tekla.com/topic/en/18/43/d42a61ed-0cfb-094d-c7af-dc55b8a32373)

#### `AddBoltDistY(...)`

Adds a Y distance to the bolt array.

[Docs](https://developer.tekla.com/topic/en/18/43/37e5f9a4-7c3e-82df-8d22-afb6a0b45357)

#### `AddOtherPartToBolt(...)`

Adds the given part to the list of the other parts that should be bolted.

[Docs](https://developer.tekla.com/topic/en/18/43/342a0099-f3a9-d0f2-c02f-9df1901682e3)

#### `CompareTo(...)`

Compares Identifiers of model objects.

[Docs](https://developer.tekla.com/topic/en/18/43/79cca356-0c8a-e8fb-463a-34ad4141bec7)

#### `Delete(...)`

Deletes the bolt array instance with the given ID from the model database.

[Docs](https://developer.tekla.com/topic/en/18/43/b879ced4-71d9-beb2-8de2-32ac4a205886)

#### `Equals(...)`

Check if Identifiers of model objects are same.

[Docs](https://developer.tekla.com/topic/en/18/43/e4324265-2490-00de-4139-63a8f7acb492)

#### `GetAllReportProperties(...)`

Retrieves all properties for the model object in one hashtable. Type for the returned value must be checked using type casting.

[Docs](https://developer.tekla.com/topic/en/18/43/0708f404-ce77-28e1-4ebe-fa906f68bff8)

#### `GetAllUserProperties(...)`

Retrieves all properties for the model object in one hashtable. Type for the returned value must be checked using type casting.

[Docs](https://developer.tekla.com/topic/en/18/43/63b853a0-0af5-af9c-3ce7-05299664d7f8)

#### `GetBoltDistX(...)`

Returns the X distance defined by the argument.

[Docs](https://developer.tekla.com/topic/en/18/43/58abe51f-c1aa-2707-0471-28b2320bd300)

#### `GetBoltDistXCount(...)`

Returns the amount of X distances in the bolt array.

[Docs](https://developer.tekla.com/topic/en/18/43/c2895d2f-3620-3d3b-856d-c9ee42ce7e30)

#### `GetBoltDistY(...)`

Returns the Y distance defined by the argument.

[Docs](https://developer.tekla.com/topic/en/18/43/d64395de-1003-2609-ba02-6276dd0f6aa9)

#### `GetBoltDistYCount(...)`

Returns the amount of Y distances in the bolt array.

[Docs](https://developer.tekla.com/topic/en/18/43/8808662a-0a53-3b51-3df0-6db0b53d9b09)

#### `GetChildren(...)`

Returns an enumerator of all the children model objects.

[Docs](https://developer.tekla.com/topic/en/18/43/ce51ec40-310c-6067-a89f-f8d029aab747)

#### `GetCoordinateSystem(...)`

Returns the coordinate system for the given model object.

[Docs](https://developer.tekla.com/topic/en/18/43/8adfe9cd-f6e5-6474-1297-33c6270a7b0a)

#### `GetCustomObjectType(...)`

Gets custom object type name from an existing object.

[Docs](https://developer.tekla.com/topic/en/18/43/d3be2ba3-8533-7165-f2d3-a5aa9df556b8)

#### `GetDoubleReportProperties(...)`

Retrieves multiple double report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/f86f6c96-f738-49ca-16ba-f5634e862ba2)

#### `GetDoubleUserProperties(...)`

Retrieves all double properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/6ead7a63-5f76-0631-d480-d46c27e882aa)

#### `GetDynamicStringProperty(...)`

Gets a dynamic string property from the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/415e86d9-7de6-ea53-ce68-20b3e67b8655)

#### `GetFatherComponent(...)`

Returns the father component of the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/a2c28f3f-52ff-50a8-1b32-10d18db5c31d)

#### `GetFatherPour(...)`

Returns the pour that the bolt group is associated to.

[Docs](https://developer.tekla.com/topic/en/18/43/47ce2460-d162-281a-9154-da7df10209f3)

#### `GetFatherPourUnit(...)`

Returns the pour unit that the bolt group is associated to.

[Docs](https://developer.tekla.com/topic/en/18/43/9dd833c7-8aaf-a67f-6687-b59b481be994)

#### `GetHierarchicObjects(...)`

Returns an enumerator of all the connected hierarchic objects.

[Docs](https://developer.tekla.com/topic/en/18/43/cc228ea0-267b-7d90-7441-c9efc1779b08)

#### `GetIntegerReportProperties(...)`

Retrieves multiple integer report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/2623a9c0-2a9c-e233-9daa-59f92ed51445)

#### `GetIntegerUserProperties(...)`

Retrieves all integer properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/1b3c6f06-bc25-a54e-2a88-b60dc4a3c85e)

#### `GetOtherPartsToBolt(...)`

Returns an array list containing all the other parts that are bolted.

[Docs](https://developer.tekla.com/topic/en/18/43/8c9b1d6d-45d5-0007-b428-104a4945b343)

#### `GetPhase(...)`

Retrieves the phase of the model object (the phase number, the phase name, the phase comment and whether the phase is the current one or not).

[Docs](https://developer.tekla.com/topic/en/18/43/310f8b7a-c120-753a-35c9-f7c73a9806c9)

#### `GetReportProperty(String, Double.)(...)`

Retrieves a double property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/1c05aa56-5f42-be91-a59e-8d59a5549f45)

#### `GetReportProperty(String, Int32.)(...)`

Retrieves an integer property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/ce8e4ca0-3750-3c9b-5ca8-27e48b45e0bd)

#### `GetReportProperty(String, String.)(...)`

Retrieves a string property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/70f212bb-ee70-e9e9-7ab6-d6a9d46f8de6)

#### `GetSolid(Boolean)(...)`

Method for getting the solid information of the bolt group.

[Docs](https://developer.tekla.com/topic/en/18/43/32e49ef9-ec57-ece5-011e-d4ebcd8afe6b)

#### `GetSolid.(...)`

Method for getting the solid information of the bolt group.

[Docs](https://developer.tekla.com/topic/en/18/43/8db794a9-0fab-8338-811d-5a70711839d1)

#### `GetStringReportProperties(...)`

Retrieves multiple string report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/5ff72da1-7382-f5d2-eb1d-c99b46239d42)

#### `GetStringUserProperties(...)`

Retrieves all string properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/67120e84-c820-9d52-12ae-c167c595842c)

#### `GetUserProperty(String, Double.)(...)`

Retrieves a double property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/17cc8074-7955-32b5-2b4b-b169c45d4ee0)

#### `GetUserProperty(String, Int32.)(...)`

Retrieves an integer property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/d9364c1d-5b57-6a31-4c13-01124058cfb2)

#### `GetUserProperty(String, String.)(...)`

Retrieves a string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/214a886c-c18c-9b66-d7c9-fe84260962f2)

#### `Insert(...)`

Inserts the bolt array into the model database.

[Docs](https://developer.tekla.com/topic/en/18/43/af3ee62e-fab7-9a46-021a-8f4db0204d84)

#### `Modify(...)`

Modifies the existing bolt array in the model database to match the current one. Note, the IDs of Part To Bolt To, Part To Be Bolted and OtherParts cannot be modified.

[Docs](https://developer.tekla.com/topic/en/18/43/8e6cf242-f7e0-7d8d-0353-57389423b258)

#### `RemoveBoltDistX(...)`

Removes an X distance from the bolt array in the index position.

[Docs](https://developer.tekla.com/topic/en/18/43/d07f9461-ccef-722c-ad84-624b7f9da45d)

#### `RemoveBoltDistY(...)`

Removes a Y distance from the bolt array in the index position.

[Docs](https://developer.tekla.com/topic/en/18/43/ef298124-c755-5cf7-4c86-f3593737249b)

#### `RemoveOtherPartToBolt(...)`

Removes the given part from the list of the other bolted parts.

[Docs](https://developer.tekla.com/topic/en/18/43/93b65a83-3e7f-fee6-8392-3d22a1be58d6)

#### `Select(...)`

Selects a bolt array from the model database. The ID must be set.

[Docs](https://developer.tekla.com/topic/en/18/43/2ecfe059-1043-8dbf-b665-279c8956cbcb)

#### `SetBoltDistX(...)`

Sets the X distance at the index defined by the argument.

[Docs](https://developer.tekla.com/topic/en/18/43/2d6d155a-2aa6-3bd4-6c42-d27905f314b3)

#### `SetBoltDistY(...)`

Sets the Y distance at the index defined by the argument.

[Docs](https://developer.tekla.com/topic/en/18/43/5a314c16-c04c-6481-0a78-cf81caa2f2dc)

#### `SetCustomObjectType(...)`

Sets a custom object type name for an existing object.

[Docs](https://developer.tekla.com/topic/en/18/43/3d6fb91b-48a9-35ca-d498-526514344089)

#### `SetDynamicStringProperty(...)`

Sets a dynamic string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/4dab0254-09e5-5e61-c28b-1c12afab71a0)

#### `SetLabel(...)`

Sets a label for an object when a new instance is created, this method must be called before Insert. The label is used in plug-ins for identifying the changed object in modification.

[Docs](https://developer.tekla.com/topic/en/18/43/3efcfc44-613a-e66b-9ebd-9f86df4f4036)

#### `SetPhase(...)`

Sets the phase of the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/ed80abcc-0ff7-d081-2229-e651f858a727)

#### `SetUserProperties(...)`

Sets multiple properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/8659da37-d571-f03f-c666-5c9fa3b113f5)

#### `SetUserProperty(String, Double)(...)`

Sets a double property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/5c585b8b-5c2e-7b75-2242-758f17429396)

#### `SetUserProperty(String, Int32)(...)`

Sets an integer property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/dd5b828d-3a87-bb58-5ac9-e1c3b4c2fe4b)

#### `SetUserProperty(String, String)(...)`

Sets a string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/80bf0348-b651-3d4a-862b-49c85bc924ab)

### Properties
- `BlindHoleDepth` (object, get/set) — Gets or sets the blind hole depth.
- `Bolt` (object, get/set) — Indicates whether the instance is a bolt or just a hole.
- `BoltHolesAttributes` (object, get/set) — The bolt holes attributes.
- `BoltPositions` (object, get/set) — Gets the single bolt positions. All the bolt positions are in the XY-plane defined by the bolt group coordinate system. The given positions are in relation to the transformation plane in which the bolt group was selected.
- `BoltSize` (object, get/set) — The size of the bolts in the group.
- `BoltStandard` (object, get/set) — The standard of the bolt group.
- `BoltType` (object, get/set) — The type of the bolt group.
- `ConnectAssemblies` (object, get/set) — Defines whether to connect a part or an assembly as a secondary part (false) or as a sub-assembly (true).
- `CutLength` (object, get/set) — The cut length.
- `EndPointOffset` (object, get/set) — The end point offset values of the bolt group.
- `ExtraLength` (object, get/set) — The extra length for the bolts.
- `FirstPosition` (object, get/set) — The first position point.
- `Hole1` (object, get/set) — Indicates whether the hole 1 is used.
- `Hole2` (object, get/set) — Indicates whether the hole 2 is used.
- `Hole3` (object, get/set) — Indicates whether the hole 3 is used.
- `Hole4` (object, get/set) — Indicates whether the hole 4 is used.
- `Hole5` (object, get/set) — Indicates whether the hole 5 is used.
- `HoleType` (object, get/set) — The special hole type: oversized, slotted, blind or no hole.
- `Identifier` (object, get/set) — The identifier of the object.
- `IsUpToDate` (object, get/set) — Gets if the object does not have a modification which is not shared.
- `Length` (object, get/set) — The length. An extra variable that is only set when the bolt group is made of studs instead of bolts.
- `ModificationTime` (object, get/set) — Gets latest time of the object was modified or created.
- `Nut1` (object, get/set) — Indicates whether the nut 1 is used in the assembly.
- `Nut2` (object, get/set) — Indicates whether the nut 2 is used in the assembly.
- `OtherPartsToBolt` (object, get/set) — The other parts to be bolted (read-only).
- `PartToBeBolted` (object, get/set) — The part to be bolted.
- `PartToBoltTo` (object, get/set) — The part to bolt to.
- `PlainHoleType` (object, get/set) — Gets or sets the plain hole type.
- `Position` (object, get/set) — The position attributes.
- `RotateSlots` (object, get/set) — Gets or sets the rotation of the slots: odd, even or parallel.
- `SecondPosition` (object, get/set) — The second position point.
- `SlotOffsetX` (object, get/set) — Gets or sets the slotted hole offset X.
- `SlotOffsetY` (object, get/set) — Gets or sets the slotted hole offset Y.
- `SlottedHoleX` (object, get/set) — Gets or sets the X allowance of the slotted hole.
- `SlottedHoleY` (object, get/set) — Gets or sets the Y allowance of the slotted hole.
- `StartPointOffset` (object, get/set) — The start point offset values of the bolt group.
- `ThreadInMaterial` (object, get/set) — Whether the thread goes inside the materal or not.
- `Tolerance` (object, get/set) — The hole tolerance.
- `Washer1` (object, get/set) — Indicates whether the washer 1 is used in the assembly.
- `Washer2` (object, get/set) — Indicates whether the washer 2 is used in the assembly.
- `Washer3` (object, get/set) — Indicates whether the washer 3 is used in the assembly.

## BoltCircle (class)

The BoltCircle class defines a bolt group with a circle shape.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/344a2e05-14de-2b76-e846-28e68b0c68ae)

### Constructors
- `BoltCircle(...)` — Creates a new bolt circle instance.

### Methods
#### `AddOtherPartToBolt(...)`

Adds the given part to the list of the other parts that should be bolted.

[Docs](https://developer.tekla.com/topic/en/18/43/342a0099-f3a9-d0f2-c02f-9df1901682e3)

#### `CompareTo(...)`

Compares Identifiers of model objects.

[Docs](https://developer.tekla.com/topic/en/18/43/79cca356-0c8a-e8fb-463a-34ad4141bec7)

#### `Delete(...)`

Deletes the bolt circle instance with the given ID from the model database.

[Docs](https://developer.tekla.com/topic/en/18/43/eea10901-f0b1-e016-ca7b-2a40e66b22cd)

#### `Equals(...)`

Check if Identifiers of model objects are same.

[Docs](https://developer.tekla.com/topic/en/18/43/e4324265-2490-00de-4139-63a8f7acb492)

#### `GetAllReportProperties(...)`

Retrieves all properties for the model object in one hashtable. Type for the returned value must be checked using type casting.

[Docs](https://developer.tekla.com/topic/en/18/43/0708f404-ce77-28e1-4ebe-fa906f68bff8)

#### `GetAllUserProperties(...)`

Retrieves all properties for the model object in one hashtable. Type for the returned value must be checked using type casting.

[Docs](https://developer.tekla.com/topic/en/18/43/63b853a0-0af5-af9c-3ce7-05299664d7f8)

#### `GetChildren(...)`

Returns an enumerator of all the children model objects.

[Docs](https://developer.tekla.com/topic/en/18/43/ce51ec40-310c-6067-a89f-f8d029aab747)

#### `GetCoordinateSystem(...)`

Returns the coordinate system for the given model object.

[Docs](https://developer.tekla.com/topic/en/18/43/8adfe9cd-f6e5-6474-1297-33c6270a7b0a)

#### `GetCustomObjectType(...)`

Gets custom object type name from an existing object.

[Docs](https://developer.tekla.com/topic/en/18/43/d3be2ba3-8533-7165-f2d3-a5aa9df556b8)

#### `GetDoubleReportProperties(...)`

Retrieves multiple double report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/f86f6c96-f738-49ca-16ba-f5634e862ba2)

#### `GetDoubleUserProperties(...)`

Retrieves all double properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/6ead7a63-5f76-0631-d480-d46c27e882aa)

#### `GetDynamicStringProperty(...)`

Gets a dynamic string property from the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/415e86d9-7de6-ea53-ce68-20b3e67b8655)

#### `GetFatherComponent(...)`

Returns the father component of the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/a2c28f3f-52ff-50a8-1b32-10d18db5c31d)

#### `GetFatherPour(...)`

Returns the pour that the bolt group is associated to.

[Docs](https://developer.tekla.com/topic/en/18/43/47ce2460-d162-281a-9154-da7df10209f3)

#### `GetFatherPourUnit(...)`

Returns the pour unit that the bolt group is associated to.

[Docs](https://developer.tekla.com/topic/en/18/43/9dd833c7-8aaf-a67f-6687-b59b481be994)

#### `GetHierarchicObjects(...)`

Returns an enumerator of all the connected hierarchic objects.

[Docs](https://developer.tekla.com/topic/en/18/43/cc228ea0-267b-7d90-7441-c9efc1779b08)

#### `GetIntegerReportProperties(...)`

Retrieves multiple integer report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/2623a9c0-2a9c-e233-9daa-59f92ed51445)

#### `GetIntegerUserProperties(...)`

Retrieves all integer properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/1b3c6f06-bc25-a54e-2a88-b60dc4a3c85e)

#### `GetOtherPartsToBolt(...)`

Returns an array list containing all the other parts that are bolted.

[Docs](https://developer.tekla.com/topic/en/18/43/8c9b1d6d-45d5-0007-b428-104a4945b343)

#### `GetPhase(...)`

Retrieves the phase of the model object (the phase number, the phase name, the phase comment and whether the phase is the current one or not).

[Docs](https://developer.tekla.com/topic/en/18/43/310f8b7a-c120-753a-35c9-f7c73a9806c9)

#### `GetReportProperty(String, Double.)(...)`

Retrieves a double property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/1c05aa56-5f42-be91-a59e-8d59a5549f45)

#### `GetReportProperty(String, Int32.)(...)`

Retrieves an integer property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/ce8e4ca0-3750-3c9b-5ca8-27e48b45e0bd)

#### `GetReportProperty(String, String.)(...)`

Retrieves a string property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/70f212bb-ee70-e9e9-7ab6-d6a9d46f8de6)

#### `GetSolid(Boolean)(...)`

Method for getting the solid information of the bolt group.

[Docs](https://developer.tekla.com/topic/en/18/43/32e49ef9-ec57-ece5-011e-d4ebcd8afe6b)

#### `GetSolid.(...)`

Method for getting the solid information of the bolt group.

[Docs](https://developer.tekla.com/topic/en/18/43/8db794a9-0fab-8338-811d-5a70711839d1)

#### `GetStringReportProperties(...)`

Retrieves multiple string report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/5ff72da1-7382-f5d2-eb1d-c99b46239d42)

#### `GetStringUserProperties(...)`

Retrieves all string properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/67120e84-c820-9d52-12ae-c167c595842c)

#### `GetUserProperty(String, Double.)(...)`

Retrieves a double property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/17cc8074-7955-32b5-2b4b-b169c45d4ee0)

#### `GetUserProperty(String, Int32.)(...)`

Retrieves an integer property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/d9364c1d-5b57-6a31-4c13-01124058cfb2)

#### `GetUserProperty(String, String.)(...)`

Retrieves a string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/214a886c-c18c-9b66-d7c9-fe84260962f2)

#### `Insert(...)`

Inserts the bolt circle into the model database. All the attributes must be set.

[Docs](https://developer.tekla.com/topic/en/18/43/e43b806f-ccf1-e61c-797d-cb0d48aafc48)

#### `Modify(...)`

Modifies the existing bolt circle in the model database to match the current one. Note, the IDs of Part To Bolt To, Part To Be Bolted and OtherParts cannot be modified.

[Docs](https://developer.tekla.com/topic/en/18/43/2a0ef0ac-b655-a45e-06da-ebdaca210e6f)

#### `RemoveOtherPartToBolt(...)`

Removes the given part from the list of the other bolted parts.

[Docs](https://developer.tekla.com/topic/en/18/43/93b65a83-3e7f-fee6-8392-3d22a1be58d6)

#### `Select(...)`

Selects a bolt circle from the model database. The ID must be set.

[Docs](https://developer.tekla.com/topic/en/18/43/37cd55b7-c1af-52d7-78de-4f08e1d809cd)

#### `SetCustomObjectType(...)`

Sets a custom object type name for an existing object.

[Docs](https://developer.tekla.com/topic/en/18/43/3d6fb91b-48a9-35ca-d498-526514344089)

#### `SetDynamicStringProperty(...)`

Sets a dynamic string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/4dab0254-09e5-5e61-c28b-1c12afab71a0)

#### `SetLabel(...)`

Sets a label for an object when a new instance is created, this method must be called before Insert. The label is used in plug-ins for identifying the changed object in modification.

[Docs](https://developer.tekla.com/topic/en/18/43/3efcfc44-613a-e66b-9ebd-9f86df4f4036)

#### `SetPhase(...)`

Sets the phase of the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/ed80abcc-0ff7-d081-2229-e651f858a727)

#### `SetUserProperties(...)`

Sets multiple properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/8659da37-d571-f03f-c666-5c9fa3b113f5)

#### `SetUserProperty(String, Double)(...)`

Sets a double property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/5c585b8b-5c2e-7b75-2242-758f17429396)

#### `SetUserProperty(String, Int32)(...)`

Sets an integer property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/dd5b828d-3a87-bb58-5ac9-e1c3b4c2fe4b)

#### `SetUserProperty(String, String)(...)`

Sets a string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/80bf0348-b651-3d4a-862b-49c85bc924ab)

### Properties
- `BlindHoleDepth` (object, get/set) — Gets or sets the blind hole depth.
- `Bolt` (object, get/set) — Indicates whether the instance is a bolt or just a hole.
- `BoltHolesAttributes` (object, get/set) — The bolt holes attributes.
- `BoltPositions` (object, get/set) — Gets the single bolt positions. All the bolt positions are in the XY-plane defined by the bolt group coordinate system. The given positions are in relation to the transformation plane in which the bolt group was selected.
- `BoltSize` (object, get/set) — The size of the bolts in the group.
- `BoltStandard` (object, get/set) — The standard of the bolt group.
- `BoltType` (object, get/set) — The type of the bolt group.
- `ConnectAssemblies` (object, get/set) — Defines whether to connect a part or an assembly as a secondary part (false) or as a sub-assembly (true).
- `CutLength` (object, get/set) — The cut length.
- `Diameter` (object, get/set) — The diameter of the circle.
- `EndPointOffset` (object, get/set) — The end point offset values of the bolt group.
- `ExtraLength` (object, get/set) — The extra length for the bolts.
- `FirstPosition` (object, get/set) — The first position point.
- `Hole1` (object, get/set) — Indicates whether the hole 1 is used.
- `Hole2` (object, get/set) — Indicates whether the hole 2 is used.
- `Hole3` (object, get/set) — Indicates whether the hole 3 is used.
- `Hole4` (object, get/set) — Indicates whether the hole 4 is used.
- `Hole5` (object, get/set) — Indicates whether the hole 5 is used.
- `HoleType` (object, get/set) — The special hole type: oversized, slotted, blind or no hole.
- `Identifier` (object, get/set) — The identifier of the object.
- `IsUpToDate` (object, get/set) — Gets if the object does not have a modification which is not shared.
- `Length` (object, get/set) — The length. An extra variable that is only set when the bolt group is made of studs instead of bolts.
- `ModificationTime` (object, get/set) — Gets latest time of the object was modified or created.
- `NumberOfBolts` (object, get/set) — The number of bolts in the circle.
- `Nut1` (object, get/set) — Indicates whether the nut 1 is used in the assembly.
- `Nut2` (object, get/set) — Indicates whether the nut 2 is used in the assembly.
- `OtherPartsToBolt` (object, get/set) — The other parts to be bolted (read-only).
- `PartToBeBolted` (object, get/set) — The part to be bolted.
- `PartToBoltTo` (object, get/set) — The part to bolt to.
- `PlainHoleType` (object, get/set) — Gets or sets the plain hole type.
- `Position` (object, get/set) — The position attributes.
- `RotateSlots` (object, get/set) — Gets or sets the rotation of the slots: odd, even or parallel.
- `SecondPosition` (object, get/set) — The second position point.
- `SlotOffsetX` (object, get/set) — Gets or sets the slotted hole offset X.
- `SlotOffsetY` (object, get/set) — Gets or sets the slotted hole offset Y.
- `SlottedHoleX` (object, get/set) — Gets or sets the X allowance of the slotted hole.
- `SlottedHoleY` (object, get/set) — Gets or sets the Y allowance of the slotted hole.
- `StartPointOffset` (object, get/set) — The start point offset values of the bolt group.
- `ThreadInMaterial` (object, get/set) — Whether the thread goes inside the materal or not.
- `Tolerance` (object, get/set) — The hole tolerance.
- `Washer1` (object, get/set) — Indicates whether the washer 1 is used in the assembly.
- `Washer2` (object, get/set) — Indicates whether the washer 2 is used in the assembly.
- `Washer3` (object, get/set) — Indicates whether the washer 3 is used in the assembly.

## BoltGroup (class)

The BoltGroup class is an abstract base class for all bolt shapes, such as array, circle and XY list. See the Tekla Structures help file for further information about the attributes.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/3647beae-3e71-72e2-03ee-df67381da5db)

### Constructors
- `BoltGroup(...)` — Creates a new bolt group instance with the given shape.

### Methods
#### `AddOtherPartToBolt(...)`

Adds the given part to the list of the other parts that should be bolted.

[Docs](https://developer.tekla.com/topic/en/18/43/342a0099-f3a9-d0f2-c02f-9df1901682e3)

#### `CompareTo(...)`

Compares Identifiers of model objects.

[Docs](https://developer.tekla.com/topic/en/18/43/79cca356-0c8a-e8fb-463a-34ad4141bec7)

#### `Delete(...)`

Deletes the instance from the model database.

[Docs](https://developer.tekla.com/topic/en/18/43/0b922776-ccee-4b5e-4a12-4e5306802e90)

#### `Equals(...)`

Check if Identifiers of model objects are same.

[Docs](https://developer.tekla.com/topic/en/18/43/e4324265-2490-00de-4139-63a8f7acb492)

#### `GetAllReportProperties(...)`

Retrieves all properties for the model object in one hashtable. Type for the returned value must be checked using type casting.

[Docs](https://developer.tekla.com/topic/en/18/43/0708f404-ce77-28e1-4ebe-fa906f68bff8)

#### `GetAllUserProperties(...)`

Retrieves all properties for the model object in one hashtable. Type for the returned value must be checked using type casting.

[Docs](https://developer.tekla.com/topic/en/18/43/63b853a0-0af5-af9c-3ce7-05299664d7f8)

#### `GetChildren(...)`

Returns an enumerator of all the children model objects.

[Docs](https://developer.tekla.com/topic/en/18/43/ce51ec40-310c-6067-a89f-f8d029aab747)

#### `GetCoordinateSystem(...)`

Returns the coordinate system for the given model object.

[Docs](https://developer.tekla.com/topic/en/18/43/8adfe9cd-f6e5-6474-1297-33c6270a7b0a)

#### `GetCustomObjectType(...)`

Gets custom object type name from an existing object.

[Docs](https://developer.tekla.com/topic/en/18/43/d3be2ba3-8533-7165-f2d3-a5aa9df556b8)

#### `GetDoubleReportProperties(...)`

Retrieves multiple double report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/f86f6c96-f738-49ca-16ba-f5634e862ba2)

#### `GetDoubleUserProperties(...)`

Retrieves all double properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/6ead7a63-5f76-0631-d480-d46c27e882aa)

#### `GetDynamicStringProperty(...)`

Gets a dynamic string property from the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/415e86d9-7de6-ea53-ce68-20b3e67b8655)

#### `GetFatherComponent(...)`

Returns the father component of the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/a2c28f3f-52ff-50a8-1b32-10d18db5c31d)

#### `GetFatherPour(...)`

Returns the pour that the bolt group is associated to.

[Docs](https://developer.tekla.com/topic/en/18/43/47ce2460-d162-281a-9154-da7df10209f3)

#### `GetFatherPourUnit(...)`

Returns the pour unit that the bolt group is associated to.

[Docs](https://developer.tekla.com/topic/en/18/43/9dd833c7-8aaf-a67f-6687-b59b481be994)

#### `GetHierarchicObjects(...)`

Returns an enumerator of all the connected hierarchic objects.

[Docs](https://developer.tekla.com/topic/en/18/43/cc228ea0-267b-7d90-7441-c9efc1779b08)

#### `GetIntegerReportProperties(...)`

Retrieves multiple integer report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/2623a9c0-2a9c-e233-9daa-59f92ed51445)

#### `GetIntegerUserProperties(...)`

Retrieves all integer properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/1b3c6f06-bc25-a54e-2a88-b60dc4a3c85e)

#### `GetOtherPartsToBolt(...)`

Returns an array list containing all the other parts that are bolted.

[Docs](https://developer.tekla.com/topic/en/18/43/8c9b1d6d-45d5-0007-b428-104a4945b343)

#### `GetPhase(...)`

Retrieves the phase of the model object (the phase number, the phase name, the phase comment and whether the phase is the current one or not).

[Docs](https://developer.tekla.com/topic/en/18/43/310f8b7a-c120-753a-35c9-f7c73a9806c9)

#### `GetReportProperty(String, Double.)(...)`

Retrieves a double property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/1c05aa56-5f42-be91-a59e-8d59a5549f45)

#### `GetReportProperty(String, Int32.)(...)`

Retrieves an integer property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/ce8e4ca0-3750-3c9b-5ca8-27e48b45e0bd)

#### `GetReportProperty(String, String.)(...)`

Retrieves a string property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/70f212bb-ee70-e9e9-7ab6-d6a9d46f8de6)

#### `GetSolid(Boolean)(...)`

Method for getting the solid information of the bolt group.

[Docs](https://developer.tekla.com/topic/en/18/43/32e49ef9-ec57-ece5-011e-d4ebcd8afe6b)

#### `GetSolid.(...)`

Method for getting the solid information of the bolt group.

[Docs](https://developer.tekla.com/topic/en/18/43/8db794a9-0fab-8338-811d-5a70711839d1)

#### `GetStringReportProperties(...)`

Retrieves multiple string report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/5ff72da1-7382-f5d2-eb1d-c99b46239d42)

#### `GetStringUserProperties(...)`

Retrieves all string properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/67120e84-c820-9d52-12ae-c167c595842c)

#### `GetUserProperty(String, Double.)(...)`

Retrieves a double property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/17cc8074-7955-32b5-2b4b-b169c45d4ee0)

#### `GetUserProperty(String, Int32.)(...)`

Retrieves an integer property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/d9364c1d-5b57-6a31-4c13-01124058cfb2)

#### `GetUserProperty(String, String.)(...)`

Retrieves a string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/214a886c-c18c-9b66-d7c9-fe84260962f2)

#### `Insert(...)`

Inserts the model object instance into the model database.

[Docs](https://developer.tekla.com/topic/en/18/43/f17ed840-1d16-b843-4b0c-270875de1d35)

#### `Modify(...)`

Modifies the model instance in the model database.

[Docs](https://developer.tekla.com/topic/en/18/43/186833b8-4837-ed77-b314-3ebf9dc32d6a)

#### `RemoveOtherPartToBolt(...)`

Removes the given part from the list of the other bolted parts.

[Docs](https://developer.tekla.com/topic/en/18/43/93b65a83-3e7f-fee6-8392-3d22a1be58d6)

#### `Select(...)`

Selects the model object instance from the model database.

[Docs](https://developer.tekla.com/topic/en/18/43/f662ab10-be15-6280-d0aa-fc259dfd7c07)

#### `SetCustomObjectType(...)`

Sets a custom object type name for an existing object.

[Docs](https://developer.tekla.com/topic/en/18/43/3d6fb91b-48a9-35ca-d498-526514344089)

#### `SetDynamicStringProperty(...)`

Sets a dynamic string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/4dab0254-09e5-5e61-c28b-1c12afab71a0)

#### `SetLabel(...)`

Sets a label for an object when a new instance is created, this method must be called before Insert. The label is used in plug-ins for identifying the changed object in modification.

[Docs](https://developer.tekla.com/topic/en/18/43/3efcfc44-613a-e66b-9ebd-9f86df4f4036)

#### `SetPhase(...)`

Sets the phase of the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/ed80abcc-0ff7-d081-2229-e651f858a727)

#### `SetUserProperties(...)`

Sets multiple properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/8659da37-d571-f03f-c666-5c9fa3b113f5)

#### `SetUserProperty(String, Double)(...)`

Sets a double property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/5c585b8b-5c2e-7b75-2242-758f17429396)

#### `SetUserProperty(String, Int32)(...)`

Sets an integer property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/dd5b828d-3a87-bb58-5ac9-e1c3b4c2fe4b)

#### `SetUserProperty(String, String)(...)`

Sets a string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/80bf0348-b651-3d4a-862b-49c85bc924ab)

### Properties
- `BlindHoleDepth` (object, get/set) — Gets or sets the blind hole depth.
- `Bolt` (object, get/set) — Indicates whether the instance is a bolt or just a hole.
- `BoltHolesAttributes` (object, get/set) — The bolt holes attributes.
- `BoltPositions` (object, get/set) — Gets the single bolt positions. All the bolt positions are in the XY-plane defined by the bolt group coordinate system. The given positions are in relation to the transformation plane in which the bolt group was selected.
- `BoltSize` (object, get/set) — The size of the bolts in the group.
- `BoltStandard` (object, get/set) — The standard of the bolt group.
- `BoltType` (object, get/set) — The type of the bolt group.
- `ConnectAssemblies` (object, get/set) — Defines whether to connect a part or an assembly as a secondary part (false) or as a sub-assembly (true).
- `CutLength` (object, get/set) — The cut length.
- `EndPointOffset` (object, get/set) — The end point offset values of the bolt group.
- `ExtraLength` (object, get/set) — The extra length for the bolts.
- `FirstPosition` (object, get/set) — The first position point.
- `Hole1` (object, get/set) — Indicates whether the hole 1 is used.
- `Hole2` (object, get/set) — Indicates whether the hole 2 is used.
- `Hole3` (object, get/set) — Indicates whether the hole 3 is used.
- `Hole4` (object, get/set) — Indicates whether the hole 4 is used.
- `Hole5` (object, get/set) — Indicates whether the hole 5 is used.
- `HoleType` (object, get/set) — The special hole type: oversized, slotted, blind or no hole.
- `Identifier` (object, get/set) — The identifier of the object.
- `IsUpToDate` (object, get/set) — Gets if the object does not have a modification which is not shared.
- `Length` (object, get/set) — The length. An extra variable that is only set when the bolt group is made of studs instead of bolts.
- `ModificationTime` (object, get/set) — Gets latest time of the object was modified or created.
- `Nut1` (object, get/set) — Indicates whether the nut 1 is used in the assembly.
- `Nut2` (object, get/set) — Indicates whether the nut 2 is used in the assembly.
- `OtherPartsToBolt` (object, get/set) — The other parts to be bolted (read-only).
- `PartToBeBolted` (object, get/set) — The part to be bolted.
- `PartToBoltTo` (object, get/set) — The part to bolt to.
- `PlainHoleType` (object, get/set) — Gets or sets the plain hole type.
- `Position` (object, get/set) — The position attributes.
- `RotateSlots` (object, get/set) — Gets or sets the rotation of the slots: odd, even or parallel.
- `SecondPosition` (object, get/set) — The second position point.
- `SlotOffsetX` (object, get/set) — Gets or sets the slotted hole offset X.
- `SlotOffsetY` (object, get/set) — Gets or sets the slotted hole offset Y.
- `SlottedHoleX` (object, get/set) — Gets or sets the X allowance of the slotted hole.
- `SlottedHoleY` (object, get/set) — Gets or sets the Y allowance of the slotted hole.
- `StartPointOffset` (object, get/set) — The start point offset values of the bolt group.
- `ThreadInMaterial` (object, get/set) — Whether the thread goes inside the materal or not.
- `Tolerance` (object, get/set) — The hole tolerance.
- `Washer1` (object, get/set) — Indicates whether the washer 1 is used in the assembly.
- `Washer2` (object, get/set) — Indicates whether the washer 2 is used in the assembly.
- `Washer3` (object, get/set) — Indicates whether the washer 3 is used in the assembly.

## BoltGroup.BoltHoleTypeEnum (enum)

The special hole type.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/4875b00c-24a3-bd15-5b05-76b3c541f18c)

### Values
- `HOLE_TYPE_OVERSIZED` = `0` — The oversized hole type.
- `HOLE_TYPE_SLOTTED` = `1` — The slotted hole type.
- `HOLE_TYPE_NO_HOLE` = `2` — The no hole type.
- `HOLE_TYPE_TAPPED` = `3` — The tapped hole type.

## BoltGroup.BoltPlainHoleTypeEnum (enum)

The plain hole type.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/d1716b64-f135-3612-eaca-36a14f2341ec)

### Values
- `HOLE_TYPE_THROUGH` = `0` — The through hole type.
- `HOLE_TYPE_BLIND` = `1` — The blind hole type.

## BoltGroup.BoltRotateSlotsEnum (enum)

The rotation of the slots (the holes).

[Vendor docs](https://developer.tekla.com/topic/en/18/43/b1a3f6e5-d6af-9968-310d-4de51286c668)

### Values
- `ROTATE_SLOTS_ODD` = `0` — The slots are crossed to odd parts.
- `ROTATE_SLOTS_EVEN` = `1` — The slots are crossed to even parts.
- `ROTATE_SLOTS_PARALLEL` = `2` — The slots are rotated to be parallel.

## BoltGroup.BoltThreadInMaterialEnum (enum)

The thread in the material.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/3db6886a-d653-ddc4-6543-8e645a77f6da)

### Values
- `THREAD_IN_MATERIAL_NO` = `0` — No thread inside the material (the part).
- `THREAD_IN_MATERIAL_YES` = `1` — Thread inside the material (the part).

## BoltGroup.BoltTypeEnum (enum)

The type of the bolt.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/6983381d-847d-9348-a456-338ed39eeb56)

### Values
- `BOLT_TYPE_SITE` = `0` — The site bolt type.
- `BOLT_TYPE_WORKSHOP` = `1` — The workshop bolt type.

## BoltGroup.ErrorStatus (enum)

Define possible statuses for geometry creation/modification commands.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/2c68335c-fae5-a806-5e6b-40bbb9a29738)

### Values
- `CommonSettingFromDiffPropertiesException` = `0` — Exception thrown when the common setting cannot be computed from the individual settings.
- `SlottedHoleOffsetsTooBigException` = `1` — Exception thrown when the slotted hole offsets move hole too far from bolt origin.

## BoltHoleAttributes (class)

The bolt hole attributes.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/422ddb23-4ec9-3422-853d-aac6c307683d)

### Constructors
- `BoltHoleAttributes(...)` — Creates a new bolt hole attributes instance.

### Properties
- `HoleType` (object, get/set) — The special hole type: oversized, tapped, slotted, blind or no hole.
- `LongHoleX` (object, get/set) — The long hole X value.
- `LongHoleY` (object, get/set) — The long hole Y value.
- `SlotOffsetX` (object, get/set) — The slot offset X value.
- `SlotOffsetY` (object, get/set) — The slot offset Y value.

## BoltXYList (class)

The BoltXYList class defines a bolt group with an XY list shape. The bolt group is the most generic one of the bolt groups.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/46d4ad9b-0db9-51b1-caa4-74a439cd54fc)

### Constructors
- `BoltXYList(...)` — Creates a new bolt XY list instance.

### Methods
#### `AddBoltDistX(...)`

Adds an X distance to the bolt XY list.

[Docs](https://developer.tekla.com/topic/en/18/43/ca8277c3-163b-fa35-0b6d-4f596d04e468)

#### `AddBoltDistY(...)`

Adds a Y distance to the bolt XY list.

[Docs](https://developer.tekla.com/topic/en/18/43/00e7bb7d-be9e-7e5a-fe8d-1ff64e43bf46)

#### `AddOtherPartToBolt(...)`

Adds the given part to the list of the other parts that should be bolted.

[Docs](https://developer.tekla.com/topic/en/18/43/342a0099-f3a9-d0f2-c02f-9df1901682e3)

#### `CompareTo(...)`

Compares Identifiers of model objects.

[Docs](https://developer.tekla.com/topic/en/18/43/79cca356-0c8a-e8fb-463a-34ad4141bec7)

#### `Delete(...)`

Deletes the bolt XY list instance with the given ID from the model database.

[Docs](https://developer.tekla.com/topic/en/18/43/5854d2fa-18cd-8b9e-28ca-99ade34b3262)

#### `Equals(...)`

Check if Identifiers of model objects are same.

[Docs](https://developer.tekla.com/topic/en/18/43/e4324265-2490-00de-4139-63a8f7acb492)

#### `GetAllReportProperties(...)`

Retrieves all properties for the model object in one hashtable. Type for the returned value must be checked using type casting.

[Docs](https://developer.tekla.com/topic/en/18/43/0708f404-ce77-28e1-4ebe-fa906f68bff8)

#### `GetAllUserProperties(...)`

Retrieves all properties for the model object in one hashtable. Type for the returned value must be checked using type casting.

[Docs](https://developer.tekla.com/topic/en/18/43/63b853a0-0af5-af9c-3ce7-05299664d7f8)

#### `GetBoltDistX(...)`

Returns the X distance defined by the argument.

[Docs](https://developer.tekla.com/topic/en/18/43/12f38251-40f0-1130-babd-a8924e20c6b9)

#### `GetBoltDistXCount(...)`

Returns the amount of X distances in the bolt XY list.

[Docs](https://developer.tekla.com/topic/en/18/43/3f72d10e-f721-be6f-df49-f30ad920c788)

#### `GetBoltDistY(...)`

Returns the Y distance defined by the argument.

[Docs](https://developer.tekla.com/topic/en/18/43/76c3acd4-1282-54dd-f311-b293268e3437)

#### `GetBoltDistYCount(...)`

Returns the amount of Y distances in the bolt XY list.

[Docs](https://developer.tekla.com/topic/en/18/43/ac680802-d7a6-2cfd-6f60-7e1873d5d2f8)

#### `GetChildren(...)`

Returns an enumerator of all the children model objects.

[Docs](https://developer.tekla.com/topic/en/18/43/ce51ec40-310c-6067-a89f-f8d029aab747)

#### `GetCoordinateSystem(...)`

Returns the coordinate system for the given model object.

[Docs](https://developer.tekla.com/topic/en/18/43/8adfe9cd-f6e5-6474-1297-33c6270a7b0a)

#### `GetCustomObjectType(...)`

Gets custom object type name from an existing object.

[Docs](https://developer.tekla.com/topic/en/18/43/d3be2ba3-8533-7165-f2d3-a5aa9df556b8)

#### `GetDoubleReportProperties(...)`

Retrieves multiple double report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/f86f6c96-f738-49ca-16ba-f5634e862ba2)

#### `GetDoubleUserProperties(...)`

Retrieves all double properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/6ead7a63-5f76-0631-d480-d46c27e882aa)

#### `GetDynamicStringProperty(...)`

Gets a dynamic string property from the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/415e86d9-7de6-ea53-ce68-20b3e67b8655)

#### `GetFatherComponent(...)`

Returns the father component of the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/a2c28f3f-52ff-50a8-1b32-10d18db5c31d)

#### `GetFatherPour(...)`

Returns the pour that the bolt group is associated to.

[Docs](https://developer.tekla.com/topic/en/18/43/47ce2460-d162-281a-9154-da7df10209f3)

#### `GetFatherPourUnit(...)`

Returns the pour unit that the bolt group is associated to.

[Docs](https://developer.tekla.com/topic/en/18/43/9dd833c7-8aaf-a67f-6687-b59b481be994)

#### `GetHierarchicObjects(...)`

Returns an enumerator of all the connected hierarchic objects.

[Docs](https://developer.tekla.com/topic/en/18/43/cc228ea0-267b-7d90-7441-c9efc1779b08)

#### `GetIntegerReportProperties(...)`

Retrieves multiple integer report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/2623a9c0-2a9c-e233-9daa-59f92ed51445)

#### `GetIntegerUserProperties(...)`

Retrieves all integer properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/1b3c6f06-bc25-a54e-2a88-b60dc4a3c85e)

#### `GetOtherPartsToBolt(...)`

Returns an array list containing all the other parts that are bolted.

[Docs](https://developer.tekla.com/topic/en/18/43/8c9b1d6d-45d5-0007-b428-104a4945b343)

#### `GetPhase(...)`

Retrieves the phase of the model object (the phase number, the phase name, the phase comment and whether the phase is the current one or not).

[Docs](https://developer.tekla.com/topic/en/18/43/310f8b7a-c120-753a-35c9-f7c73a9806c9)

#### `GetReportProperty(String, Double.)(...)`

Retrieves a double property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/1c05aa56-5f42-be91-a59e-8d59a5549f45)

#### `GetReportProperty(String, Int32.)(...)`

Retrieves an integer property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/ce8e4ca0-3750-3c9b-5ca8-27e48b45e0bd)

#### `GetReportProperty(String, String.)(...)`

Retrieves a string property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/70f212bb-ee70-e9e9-7ab6-d6a9d46f8de6)

#### `GetSolid(Boolean)(...)`

Method for getting the solid information of the bolt group.

[Docs](https://developer.tekla.com/topic/en/18/43/32e49ef9-ec57-ece5-011e-d4ebcd8afe6b)

#### `GetSolid.(...)`

Method for getting the solid information of the bolt group.

[Docs](https://developer.tekla.com/topic/en/18/43/8db794a9-0fab-8338-811d-5a70711839d1)

#### `GetStringReportProperties(...)`

Retrieves multiple string report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/5ff72da1-7382-f5d2-eb1d-c99b46239d42)

#### `GetStringUserProperties(...)`

Retrieves all string properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/67120e84-c820-9d52-12ae-c167c595842c)

#### `GetUserProperty(String, Double.)(...)`

Retrieves a double property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/17cc8074-7955-32b5-2b4b-b169c45d4ee0)

#### `GetUserProperty(String, Int32.)(...)`

Retrieves an integer property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/d9364c1d-5b57-6a31-4c13-01124058cfb2)

#### `GetUserProperty(String, String.)(...)`

Retrieves a string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/214a886c-c18c-9b66-d7c9-fe84260962f2)

#### `Insert(...)`

Inserts the bolt XY list into the model database. All the attributes must be set.

[Docs](https://developer.tekla.com/topic/en/18/43/98c0e186-8495-8977-bf92-812cfa79fcb6)

#### `Modify(...)`

Modifies the existing bolt XY list in the model database to match the current one. Note, the IDs of Part To Bolt To, Part To Be Bolted and OtherParts cannot be modified.

[Docs](https://developer.tekla.com/topic/en/18/43/a008e002-2003-d284-dab1-dd6410c8c3d0)

#### `RemoveOtherPartToBolt(...)`

Removes the given part from the list of the other bolted parts.

[Docs](https://developer.tekla.com/topic/en/18/43/93b65a83-3e7f-fee6-8392-3d22a1be58d6)

#### `Select(...)`

Selects a bolt XY list from the model database. The ID must be set.

[Docs](https://developer.tekla.com/topic/en/18/43/b595c613-2fec-0e81-517d-56eba35a11c0)

#### `SetCustomObjectType(...)`

Sets a custom object type name for an existing object.

[Docs](https://developer.tekla.com/topic/en/18/43/3d6fb91b-48a9-35ca-d498-526514344089)

#### `SetDynamicStringProperty(...)`

Sets a dynamic string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/4dab0254-09e5-5e61-c28b-1c12afab71a0)

#### `SetLabel(...)`

Sets a label for an object when a new instance is created, this method must be called before Insert. The label is used in plug-ins for identifying the changed object in modification.

[Docs](https://developer.tekla.com/topic/en/18/43/3efcfc44-613a-e66b-9ebd-9f86df4f4036)

#### `SetPhase(...)`

Sets the phase of the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/ed80abcc-0ff7-d081-2229-e651f858a727)

#### `SetUserProperties(...)`

Sets multiple properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/8659da37-d571-f03f-c666-5c9fa3b113f5)

#### `SetUserProperty(String, Double)(...)`

Sets a double property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/5c585b8b-5c2e-7b75-2242-758f17429396)

#### `SetUserProperty(String, Int32)(...)`

Sets an integer property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/dd5b828d-3a87-bb58-5ac9-e1c3b4c2fe4b)

#### `SetUserProperty(String, String)(...)`

Sets a string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/80bf0348-b651-3d4a-862b-49c85bc924ab)

### Properties
- `BlindHoleDepth` (object, get/set) — Gets or sets the blind hole depth.
- `Bolt` (object, get/set) — Indicates whether the instance is a bolt or just a hole.
- `BoltHolesAttributes` (object, get/set) — The bolt holes attributes.
- `BoltPositions` (object, get/set) — Gets the single bolt positions. All the bolt positions are in the XY-plane defined by the bolt group coordinate system. The given positions are in relation to the transformation plane in which the bolt group was selected.
- `BoltSize` (object, get/set) — The size of the bolts in the group.
- `BoltStandard` (object, get/set) — The standard of the bolt group.
- `BoltType` (object, get/set) — The type of the bolt group.
- `ConnectAssemblies` (object, get/set) — Defines whether to connect a part or an assembly as a secondary part (false) or as a sub-assembly (true).
- `CutLength` (object, get/set) — The cut length.
- `EndPointOffset` (object, get/set) — The end point offset values of the bolt group.
- `ExtraLength` (object, get/set) — The extra length for the bolts.
- `FirstPosition` (object, get/set) — The first position point.
- `Hole1` (object, get/set) — Indicates whether the hole 1 is used.
- `Hole2` (object, get/set) — Indicates whether the hole 2 is used.
- `Hole3` (object, get/set) — Indicates whether the hole 3 is used.
- `Hole4` (object, get/set) — Indicates whether the hole 4 is used.
- `Hole5` (object, get/set) — Indicates whether the hole 5 is used.
- `HoleType` (object, get/set) — The special hole type: oversized, slotted, blind or no hole.
- `Identifier` (object, get/set) — The identifier of the object.
- `IsUpToDate` (object, get/set) — Gets if the object does not have a modification which is not shared.
- `Length` (object, get/set) — The length. An extra variable that is only set when the bolt group is made of studs instead of bolts.
- `ModificationTime` (object, get/set) — Gets latest time of the object was modified or created.
- `Nut1` (object, get/set) — Indicates whether the nut 1 is used in the assembly.
- `Nut2` (object, get/set) — Indicates whether the nut 2 is used in the assembly.
- `OtherPartsToBolt` (object, get/set) — The other parts to be bolted (read-only).
- `PartToBeBolted` (object, get/set) — The part to be bolted.
- `PartToBoltTo` (object, get/set) — The part to bolt to.
- `PlainHoleType` (object, get/set) — Gets or sets the plain hole type.
- `Position` (object, get/set) — The position attributes.
- `RotateSlots` (object, get/set) — Gets or sets the rotation of the slots: odd, even or parallel.
- `SecondPosition` (object, get/set) — The second position point.
- `SlotOffsetX` (object, get/set) — Gets or sets the slotted hole offset X.
- `SlotOffsetY` (object, get/set) — Gets or sets the slotted hole offset Y.
- `SlottedHoleX` (object, get/set) — Gets or sets the X allowance of the slotted hole.
- `SlottedHoleY` (object, get/set) — Gets or sets the Y allowance of the slotted hole.
- `StartPointOffset` (object, get/set) — The start point offset values of the bolt group.
- `ThreadInMaterial` (object, get/set) — Whether the thread goes inside the materal or not.
- `Tolerance` (object, get/set) — The hole tolerance.
- `Washer1` (object, get/set) — Indicates whether the washer 1 is used in the assembly.
- `Washer2` (object, get/set) — Indicates whether the washer 2 is used in the assembly.
- `Washer3` (object, get/set) — Indicates whether the washer 3 is used in the assembly.

## Boolean (class)

The Boolean class is an abstract base class for boolean operations such as a part cut, an add, a fitting or a cut plane.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/49be33d1-5e49-b9cf-a30f-6292868e03a5)

### Constructors
- `Boolean(...)` — Constructs an empty boolean.

### Methods
#### `CompareTo(...)`

Compares Identifiers of model objects.

[Docs](https://developer.tekla.com/topic/en/18/43/79cca356-0c8a-e8fb-463a-34ad4141bec7)

#### `Delete(...)`

Deletes the instance from the model database.

[Docs](https://developer.tekla.com/topic/en/18/43/0b922776-ccee-4b5e-4a12-4e5306802e90)

#### `Equals(...)`

Check if Identifiers of model objects are same.

[Docs](https://developer.tekla.com/topic/en/18/43/e4324265-2490-00de-4139-63a8f7acb492)

#### `GetAllReportProperties(...)`

Retrieves all properties for the model object in one hashtable. Type for the returned value must be checked using type casting.

[Docs](https://developer.tekla.com/topic/en/18/43/0708f404-ce77-28e1-4ebe-fa906f68bff8)

#### `GetAllUserProperties(...)`

Retrieves all properties for the model object in one hashtable. Type for the returned value must be checked using type casting.

[Docs](https://developer.tekla.com/topic/en/18/43/63b853a0-0af5-af9c-3ce7-05299664d7f8)

#### `GetChildren(...)`

Returns an enumerator of all the children model objects.

[Docs](https://developer.tekla.com/topic/en/18/43/ce51ec40-310c-6067-a89f-f8d029aab747)

#### `GetCoordinateSystem(...)`

Returns the coordinate system for the given model object.

[Docs](https://developer.tekla.com/topic/en/18/43/8adfe9cd-f6e5-6474-1297-33c6270a7b0a)

#### `GetCustomObjectType(...)`

Gets custom object type name from an existing object.

[Docs](https://developer.tekla.com/topic/en/18/43/d3be2ba3-8533-7165-f2d3-a5aa9df556b8)

#### `GetDoubleReportProperties(...)`

Retrieves multiple double report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/f86f6c96-f738-49ca-16ba-f5634e862ba2)

#### `GetDoubleUserProperties(...)`

Retrieves all double properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/6ead7a63-5f76-0631-d480-d46c27e882aa)

#### `GetDynamicStringProperty(...)`

Gets a dynamic string property from the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/415e86d9-7de6-ea53-ce68-20b3e67b8655)

#### `GetFatherComponent(...)`

Returns the father component of the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/a2c28f3f-52ff-50a8-1b32-10d18db5c31d)

#### `GetHierarchicObjects(...)`

Returns an enumerator of all the connected hierarchic objects.

[Docs](https://developer.tekla.com/topic/en/18/43/cc228ea0-267b-7d90-7441-c9efc1779b08)

#### `GetIntegerReportProperties(...)`

Retrieves multiple integer report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/2623a9c0-2a9c-e233-9daa-59f92ed51445)

#### `GetIntegerUserProperties(...)`

Retrieves all integer properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/1b3c6f06-bc25-a54e-2a88-b60dc4a3c85e)

#### `GetPhase(...)`

Retrieves the phase of the model object (the phase number, the phase name, the phase comment and whether the phase is the current one or not).

[Docs](https://developer.tekla.com/topic/en/18/43/310f8b7a-c120-753a-35c9-f7c73a9806c9)

#### `GetReportProperty(String, Double.)(...)`

Retrieves a double property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/1c05aa56-5f42-be91-a59e-8d59a5549f45)

#### `GetReportProperty(String, Int32.)(...)`

Retrieves an integer property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/ce8e4ca0-3750-3c9b-5ca8-27e48b45e0bd)

#### `GetReportProperty(String, String.)(...)`

Retrieves a string property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/70f212bb-ee70-e9e9-7ab6-d6a9d46f8de6)

#### `GetStringReportProperties(...)`

Retrieves multiple string report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/5ff72da1-7382-f5d2-eb1d-c99b46239d42)

#### `GetStringUserProperties(...)`

Retrieves all string properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/67120e84-c820-9d52-12ae-c167c595842c)

#### `GetUserProperty(String, Double.)(...)`

Retrieves a double property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/17cc8074-7955-32b5-2b4b-b169c45d4ee0)

#### `GetUserProperty(String, Int32.)(...)`

Retrieves an integer property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/d9364c1d-5b57-6a31-4c13-01124058cfb2)

#### `GetUserProperty(String, String.)(...)`

Retrieves a string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/214a886c-c18c-9b66-d7c9-fe84260962f2)

#### `Insert(...)`

Inserts the model object instance into the model database.

[Docs](https://developer.tekla.com/topic/en/18/43/f17ed840-1d16-b843-4b0c-270875de1d35)

#### `Modify(...)`

Modifies the model instance in the model database.

[Docs](https://developer.tekla.com/topic/en/18/43/186833b8-4837-ed77-b314-3ebf9dc32d6a)

#### `Select(...)`

Selects the model object instance from the model database.

[Docs](https://developer.tekla.com/topic/en/18/43/f662ab10-be15-6280-d0aa-fc259dfd7c07)

#### `SetCustomObjectType(...)`

Sets a custom object type name for an existing object.

[Docs](https://developer.tekla.com/topic/en/18/43/3d6fb91b-48a9-35ca-d498-526514344089)

#### `SetDynamicStringProperty(...)`

Sets a dynamic string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/4dab0254-09e5-5e61-c28b-1c12afab71a0)

#### `SetLabel(...)`

Sets a label for an object when a new instance is created, this method must be called before Insert. The label is used in plug-ins for identifying the changed object in modification.

[Docs](https://developer.tekla.com/topic/en/18/43/3efcfc44-613a-e66b-9ebd-9f86df4f4036)

#### `SetPhase(...)`

Sets the phase of the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/ed80abcc-0ff7-d081-2229-e651f858a727)

#### `SetUserProperties(...)`

Sets multiple properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/8659da37-d571-f03f-c666-5c9fa3b113f5)

#### `SetUserProperty(String, Double)(...)`

Sets a double property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/5c585b8b-5c2e-7b75-2242-758f17429396)

#### `SetUserProperty(String, Int32)(...)`

Sets an integer property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/dd5b828d-3a87-bb58-5ac9-e1c3b4c2fe4b)

#### `SetUserProperty(String, String)(...)`

Sets a string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/80bf0348-b651-3d4a-862b-49c85bc924ab)

### Properties
- `Father` (object, get/set) — The father object of the boolean operation; the model object instance to operate on.
- `Identifier` (object, get/set) — The identifier of the object.
- `IsUpToDate` (object, get/set) — Gets if the object does not have a modification which is not shared.
- `ModificationTime` (object, get/set) — Gets latest time of the object was modified or created.

## BooleanPart (class)

The BooleanPart class represents a part cut or an add. This means that a model object is, for example, cut with a part instance to create a void to the father part. Typically the operative part is deleted after the boolean cut operation. In case of boolean add, operative part is deleted automatically.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/2fbf5949-ba94-3e0d-a738-1ed7ba1bec1d)

### Constructors
- `BooleanPart(...)` — Instantiates an empty cut operation.

### Methods
#### `CompareTo(...)`

Compares Identifiers of model objects.

[Docs](https://developer.tekla.com/topic/en/18/43/79cca356-0c8a-e8fb-463a-34ad4141bec7)

#### `Delete(...)`

Deletes the boolean part instance with the given ID from the model database.

[Docs](https://developer.tekla.com/topic/en/18/43/be9477b8-b8b3-92d4-f32a-44daaddd03e3)

#### `Equals(...)`

Check if Identifiers of model objects are same.

[Docs](https://developer.tekla.com/topic/en/18/43/e4324265-2490-00de-4139-63a8f7acb492)

#### `GetAllReportProperties(...)`

Retrieves all properties for the model object in one hashtable. Type for the returned value must be checked using type casting.

[Docs](https://developer.tekla.com/topic/en/18/43/0708f404-ce77-28e1-4ebe-fa906f68bff8)

#### `GetAllUserProperties(...)`

Retrieves all properties for the model object in one hashtable. Type for the returned value must be checked using type casting.

[Docs](https://developer.tekla.com/topic/en/18/43/63b853a0-0af5-af9c-3ce7-05299664d7f8)

#### `GetChildren(...)`

Returns an enumerator of all the children model objects.

[Docs](https://developer.tekla.com/topic/en/18/43/ce51ec40-310c-6067-a89f-f8d029aab747)

#### `GetCoordinateSystem(...)`

Returns the coordinate system for the given model object.

[Docs](https://developer.tekla.com/topic/en/18/43/8adfe9cd-f6e5-6474-1297-33c6270a7b0a)

#### `GetCustomObjectType(...)`

Gets custom object type name from an existing object.

[Docs](https://developer.tekla.com/topic/en/18/43/d3be2ba3-8533-7165-f2d3-a5aa9df556b8)

#### `GetDoubleReportProperties(...)`

Retrieves multiple double report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/f86f6c96-f738-49ca-16ba-f5634e862ba2)

#### `GetDoubleUserProperties(...)`

Retrieves all double properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/6ead7a63-5f76-0631-d480-d46c27e882aa)

#### `GetDynamicStringProperty(...)`

Gets a dynamic string property from the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/415e86d9-7de6-ea53-ce68-20b3e67b8655)

#### `GetFatherComponent(...)`

Returns the father component of the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/a2c28f3f-52ff-50a8-1b32-10d18db5c31d)

#### `GetHierarchicObjects(...)`

Returns an enumerator of all the connected hierarchic objects.

[Docs](https://developer.tekla.com/topic/en/18/43/cc228ea0-267b-7d90-7441-c9efc1779b08)

#### `GetIntegerReportProperties(...)`

Retrieves multiple integer report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/2623a9c0-2a9c-e233-9daa-59f92ed51445)

#### `GetIntegerUserProperties(...)`

Retrieves all integer properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/1b3c6f06-bc25-a54e-2a88-b60dc4a3c85e)

#### `GetPhase(...)`

Retrieves the phase of the model object (the phase number, the phase name, the phase comment and whether the phase is the current one or not).

[Docs](https://developer.tekla.com/topic/en/18/43/310f8b7a-c120-753a-35c9-f7c73a9806c9)

#### `GetReportProperty(String, Double.)(...)`

Retrieves a double property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/1c05aa56-5f42-be91-a59e-8d59a5549f45)

#### `GetReportProperty(String, Int32.)(...)`

Retrieves an integer property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/ce8e4ca0-3750-3c9b-5ca8-27e48b45e0bd)

#### `GetReportProperty(String, String.)(...)`

Retrieves a string property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/70f212bb-ee70-e9e9-7ab6-d6a9d46f8de6)

#### `GetStringReportProperties(...)`

Retrieves multiple string report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/5ff72da1-7382-f5d2-eb1d-c99b46239d42)

#### `GetStringUserProperties(...)`

Retrieves all string properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/67120e84-c820-9d52-12ae-c167c595842c)

#### `GetUserProperty(String, Double.)(...)`

Retrieves a double property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/17cc8074-7955-32b5-2b4b-b169c45d4ee0)

#### `GetUserProperty(String, Int32.)(...)`

Retrieves an integer property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/d9364c1d-5b57-6a31-4c13-01124058cfb2)

#### `GetUserProperty(String, String.)(...)`

Retrieves a string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/214a886c-c18c-9b66-d7c9-fe84260962f2)

#### `Insert(...)`

Inserts the boolean part into the model database. All the attributes must be set. The operative part is overwritten by a copy of the original part.

[Docs](https://developer.tekla.com/topic/en/18/43/429e4a44-dd59-97ae-3b93-b5921e272d3a)

#### `Modify(...)`

Currently it is not possible to modify the boolean part as there is nothing that can be modified for it. Calling this function will actually call modify for the operative part member.

[Docs](https://developer.tekla.com/topic/en/18/43/e257d9f9-d22b-9189-ece5-3a4d1e6bf35f)

#### `Select(...)`

Selects a boolean part from the model database. The part ID must be set.

[Docs](https://developer.tekla.com/topic/en/18/43/6fef0b3f-6159-9689-c0ee-9c90a7b4f170)

#### `SetCustomObjectType(...)`

Sets a custom object type name for an existing object.

[Docs](https://developer.tekla.com/topic/en/18/43/3d6fb91b-48a9-35ca-d498-526514344089)

#### `SetDynamicStringProperty(...)`

Sets a dynamic string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/4dab0254-09e5-5e61-c28b-1c12afab71a0)

#### `SetLabel(...)`

Sets a label for an object when a new instance is created, this method must be called before Insert. The label is used in plug-ins for identifying the changed object in modification.

[Docs](https://developer.tekla.com/topic/en/18/43/3efcfc44-613a-e66b-9ebd-9f86df4f4036)

#### `SetOperativePart(...)`

Sets the part to operate with. The operative part's class member must be defined to be BooleanOperativeClass.

[Docs](https://developer.tekla.com/topic/en/18/43/c70ac6e9-d127-bbbc-28c0-0d22545aa923)

#### `SetPhase(...)`

Sets the phase of the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/ed80abcc-0ff7-d081-2229-e651f858a727)

#### `SetUserProperties(...)`

Sets multiple properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/8659da37-d571-f03f-c666-5c9fa3b113f5)

#### `SetUserProperty(String, Double)(...)`

Sets a double property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/5c585b8b-5c2e-7b75-2242-758f17429396)

#### `SetUserProperty(String, Int32)(...)`

Sets an integer property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/dd5b828d-3a87-bb58-5ac9-e1c3b4c2fe4b)

#### `SetUserProperty(String, String)(...)`

Sets a string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/80bf0348-b651-3d4a-862b-49c85bc924ab)

### Properties
- `Father` (object, get/set) — The father object of the boolean operation; the model object instance to operate on.
- `Identifier` (object, get/set) — The identifier of the object.
- `IsUpToDate` (object, get/set) — Gets if the object does not have a modification which is not shared.
- `ModificationTime` (object, get/set) — Gets latest time of the object was modified or created.
- `OperativePart` (object, get/set) — The part that does the boolean operation. When Insert() is called, this field is overwritten with a new part instance which is a copy of the original operative part. This part has the same identifier as the boolean part. Modifications (such as profile change) have to be made through the part instance and not the boolean part instance. The operative part's class must be BooleanOperativeClass.
- `Type` (object, get/set) — Defines the boolean operation: an add or a cut.

## BooleanPart.BooleanTypeEnum (enum)

Defines the boolean operation: an add or a cut.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/a103fb75-b005-bb97-51cc-c4f5ff5763af)

### Values
- `BOOLEAN_ADD` = `1` — Defines the add part. Typically the add part type is not used.
- `BOOLEAN_CUT` = `2` — Defines the cut part.
- `BOOLEAN_WELDPREP` = `3` — Defines the weld prep part.

## Brep (class)

The Brep class represents a single brep in the model. A brep has a single start and end point.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/77e7ce8e-76cd-c1d8-1814-1b02750fa9bc)

### Constructors
- `Brep(...)` — Initializes a new instance of the Brep class. The start and end points are in the origin.
- `Brep(...)` — Initializes a new instance of the Brep class with the given start and end points.

### Methods
#### `CompareTo(Object)(...)`

Compares Identifiers of model objects.

[Docs](https://developer.tekla.com/topic/en/18/43/79cca356-0c8a-e8fb-463a-34ad4141bec7)

#### `CompareTo(Part)(...)`

Compares the instantiated part with another one.

[Docs](https://developer.tekla.com/topic/en/18/43/e8632c0d-89cf-9aab-6532-f62f9873067b)

#### `Delete(...)`

Deletes the brep instance with the given identifier from the model database.

[Docs](https://developer.tekla.com/topic/en/18/43/171e636e-f6b0-3f4e-7014-63ebef8a99a6)

#### `Equals(...)`

Check if Identifiers of model objects are same.

[Docs](https://developer.tekla.com/topic/en/18/43/e4324265-2490-00de-4139-63a8f7acb492)

#### `GetAllReportProperties(...)`

Retrieves all properties for the model object in one hashtable. Type for the returned value must be checked using type casting.

[Docs](https://developer.tekla.com/topic/en/18/43/0708f404-ce77-28e1-4ebe-fa906f68bff8)

#### `GetAllUserProperties(...)`

Retrieves all properties for the model object in one hashtable. Type for the returned value must be checked using type casting.

[Docs](https://developer.tekla.com/topic/en/18/43/63b853a0-0af5-af9c-3ce7-05299664d7f8)

#### `GetAssembly(...)`

Returns the assembly that the part belongs to.

[Docs](https://developer.tekla.com/topic/en/18/43/bda18562-45a1-1355-42ed-6044af0b18cb)

#### `GetBolts(...)`

Returns an enumerator of all the connected bolts.

[Docs](https://developer.tekla.com/topic/en/18/43/dbc6b475-7461-7ec4-e8ec-7b7d3d8ff429)

#### `GetBooleans(...)`

Returns an enumerator of all the connected boolean objects.

[Docs](https://developer.tekla.com/topic/en/18/43/ae180e43-88e2-d759-0ae0-567d52681a2a)

#### `GetCenterLine(...)`

Returns the center line for the given part.

[Docs](https://developer.tekla.com/topic/en/18/43/bdc886b8-af7a-722c-dcb8-fddb2f250a83)

#### `GetChildren(...)`

Returns an enumerator of all the children model objects.

[Docs](https://developer.tekla.com/topic/en/18/43/ce51ec40-310c-6067-a89f-f8d029aab747)

#### `GetComponents(...)`

Returns an enumerator of all the connected components, connections, seams and details inherited from the base component.

[Docs](https://developer.tekla.com/topic/en/18/43/ed57941c-2cd8-3b78-6a6e-848b8a920298)

#### `GetCoordinateSystem(...)`

Returns the coordinate system for the given model object.

[Docs](https://developer.tekla.com/topic/en/18/43/8adfe9cd-f6e5-6474-1297-33c6270a7b0a)

#### `GetCustomObjectType(...)`

Gets custom object type name from an existing object.

[Docs](https://developer.tekla.com/topic/en/18/43/d3be2ba3-8533-7165-f2d3-a5aa9df556b8)

#### `GetDSTVCoordinateSystem(...)`

Get DSTV coordinate system.

[Docs](https://developer.tekla.com/topic/en/18/43/9e70005f-c701-ab99-8fa0-4fce47b1ab05)

#### `GetDoubleReportProperties(...)`

Retrieves multiple double report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/f86f6c96-f738-49ca-16ba-f5634e862ba2)

#### `GetDoubleUserProperties(...)`

Retrieves all double properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/6ead7a63-5f76-0631-d480-d46c27e882aa)

#### `GetDynamicStringProperty(...)`

Gets a dynamic string property from the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/415e86d9-7de6-ea53-ce68-20b3e67b8655)

#### `GetFatherComponent(...)`

Returns the father component of the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/a2c28f3f-52ff-50a8-1b32-10d18db5c31d)

#### `GetHierarchicObjects(...)`

Returns an enumerator of all the connected hierarchic objects.

[Docs](https://developer.tekla.com/topic/en/18/43/cc228ea0-267b-7d90-7441-c9efc1779b08)

#### `GetIntegerReportProperties(...)`

Retrieves multiple integer report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/2623a9c0-2a9c-e233-9daa-59f92ed51445)

#### `GetIntegerUserProperties(...)`

Retrieves all integer properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/1b3c6f06-bc25-a54e-2a88-b60dc4a3c85e)

#### `GetPartMark(...)`

Returns the part mark.

[Docs](https://developer.tekla.com/topic/en/18/43/9978ba28-6304-714d-815c-fe4a6bb77076)

#### `GetPhase(...)`

Retrieves the phase of the model object (the phase number, the phase name, the phase comment and whether the phase is the current one or not).

[Docs](https://developer.tekla.com/topic/en/18/43/310f8b7a-c120-753a-35c9-f7c73a9806c9)

#### `GetPours(...)`

Returns an enumerator of all the pours that the part belongs to.

[Docs](https://developer.tekla.com/topic/en/18/43/1cc67ebb-329a-4675-7857-af663602d21d)

#### `GetReferenceLine(...)`

Returns the reference line for the given part.

[Docs](https://developer.tekla.com/topic/en/18/43/3e0a42c9-f10c-4225-bcd6-2b9b61ecc743)

#### `GetReinforcements(...)`

Returns an enumerator of all the connected reinforcements.

[Docs](https://developer.tekla.com/topic/en/18/43/510c913a-0653-6777-1841-c4f7df4fee47)

#### `GetReportProperty(String, Double.)(...)`

Retrieves a double property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/1c05aa56-5f42-be91-a59e-8d59a5549f45)

#### `GetReportProperty(String, Int32.)(...)`

Retrieves an integer property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/ce8e4ca0-3750-3c9b-5ca8-27e48b45e0bd)

#### `GetReportProperty(String, String.)(...)`

Retrieves a string property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/70f212bb-ee70-e9e9-7ab6-d6a9d46f8de6)

#### `GetSolid(FormingStates)(...)`

Returns the solid of the part.

[Docs](https://developer.tekla.com/topic/en/18/43/1e4d454b-d3a0-4219-b4e2-7f661b402c4d)

#### `GetSolid(Solid.SolidCreationTypeEnum)(...)`

Returns the solid of the part.

[Docs](https://developer.tekla.com/topic/en/18/43/25f58541-37fe-c1ca-f384-065ecbe38e55)

#### `GetSolid.(...)`

Returns the solid of the part.

[Docs](https://developer.tekla.com/topic/en/18/43/7bd70e97-7c8b-823c-c7e1-da2ebe3dadaf)

#### `GetStringReportProperties(...)`

Retrieves multiple string report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/5ff72da1-7382-f5d2-eb1d-c99b46239d42)

#### `GetStringUserProperties(...)`

Retrieves all string properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/67120e84-c820-9d52-12ae-c167c595842c)

#### `GetSurfaceObjects(...)`

Returns an enumerator of all the connected surface objects.

[Docs](https://developer.tekla.com/topic/en/18/43/36268c4e-9006-1c9d-57bd-331f907da6ce)

#### `GetSurfaceTreatments(...)`

Returns an enumerator of all the connected surface treatments.

[Docs](https://developer.tekla.com/topic/en/18/43/26c1c1a5-e5db-9850-f739-c58bc05c963e)

#### `GetUserProperty(String, Double.)(...)`

Retrieves a double property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/17cc8074-7955-32b5-2b4b-b169c45d4ee0)

#### `GetUserProperty(String, Int32.)(...)`

Retrieves an integer property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/d9364c1d-5b57-6a31-4c13-01124058cfb2)

#### `GetUserProperty(String, String.)(...)`

Retrieves a string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/214a886c-c18c-9b66-d7c9-fe84260962f2)

#### `GetWelds(...)`

Returns an enumerator of all the connected welds.

[Docs](https://developer.tekla.com/topic/en/18/43/722de753-413b-f810-f09c-07388feb0afa)

#### `Insert(...)`

Inserts the brep into the model database. All the attributes must be set.

[Docs](https://developer.tekla.com/topic/en/18/43/22acf248-df5a-2418-0621-37d0c7557d52)

#### `Modify(...)`

Modifies the existing brep in the model database to match the current one.

[Docs](https://developer.tekla.com/topic/en/18/43/fa6fa4a0-044c-8cd3-439e-e2093b81d1a8)

#### `Select(...)`

Selects a brep from the model database. The brep identifier must be set.

[Docs](https://developer.tekla.com/topic/en/18/43/146e528b-e4e4-da5d-2f4f-315bfac92cf6)

#### `SetCustomObjectType(...)`

Sets a custom object type name for an existing object.

[Docs](https://developer.tekla.com/topic/en/18/43/3d6fb91b-48a9-35ca-d498-526514344089)

#### `SetDynamicStringProperty(...)`

Sets a dynamic string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/4dab0254-09e5-5e61-c28b-1c12afab71a0)

#### `SetLabel(...)`

Sets a label for an object when a new instance is created, this method must be called before Insert. The label is used in plug-ins for identifying the changed object in modification.

[Docs](https://developer.tekla.com/topic/en/18/43/3efcfc44-613a-e66b-9ebd-9f86df4f4036)

#### `SetPhase(...)`

Sets the phase of the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/ed80abcc-0ff7-d081-2229-e651f858a727)

#### `SetUserProperties(...)`

Sets multiple properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/8659da37-d571-f03f-c666-5c9fa3b113f5)

#### `SetUserProperty(String, Double)(...)`

Sets a double property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/5c585b8b-5c2e-7b75-2242-758f17429396)

#### `SetUserProperty(String, Int32)(...)`

Sets an integer property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/dd5b828d-3a87-bb58-5ac9-e1c3b4c2fe4b)

#### `SetUserProperty(String, String)(...)`

Sets a string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/80bf0348-b651-3d4a-862b-49c85bc924ab)

### Properties
- `AssemblyNumber` (object, get/set) — Gets or sets the assembly number. Defines the numbering in the assembly sense.
- `CastUnitType` (object, get/set) — Gets or sets the cast unit type of the part.
- `Class` (object, get/set) — Gets or sets the class of the part.
- `DeformingData` (object, get/set) — Gets or sets the deforming data of the part.
- `EndPoint` (object, get/set) — Gets or sets the end point of the brep.
- `EndPointOffset` (object, get/set) — Gets or sets the beam's end point offset.
- `Finish` (object, get/set) — Gets or sets the finish of the part.
- `Identifier` (object, get/set) — The identifier of the object.
- `IsUpToDate` (object, get/set) — Gets if the object does not have a modification which is not shared.
- `Material` (object, get/set) — Gets or sets the material the part is made of.
- `ModificationTime` (object, get/set) — Gets latest time of the object was modified or created.
- `Name` (object, get/set) — Gets or sets the name of the part.
- `PartNumber` (object, get/set) — Gets or sets the part number. Defines the numbering in the part sense.
- `Position` (object, get/set) — Gets or sets the part position. Defines the way the part is positioned in the model.
- `PourPhase` (object, get/set) — Gets or sets the pour phase of the part.
- `Profile` (object, get/set) — Gets or sets the profile of the part.
- `StartPoint` (object, get/set) — Gets or sets the start point of the brep.
- `StartPointOffset` (object, get/set) — Gets or sets the beam's start point offset.

## Chamfer (class)

The Chamfer class defines how the corners of a polybeam or a contour plate are rounded.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/3064a4cf-a8fc-6b76-b9e1-01173be467df)

### Constructors
- `Chamfer(...)` — Creates a chamfer instance with the basic shape type CHAMFER_NONE.
- `Chamfer(...)` — Instantiates a chamfer with the given values.

### Properties
- `DZ1` (object, get/set) — The thickness above the offset. Note that this does nothing when used with the PolyBeam.
- `DZ2` (object, get/set) — The thickness below the offset. Note that this does nothing when used with the PolyBeam.
- `Type` (object, get/set) — The basic shape type of the chamfer.
- `X` (object, get/set) — The dimension of the chamfer.
- `Y` (object, get/set) — The second dimension of straight chamfers.

## Chamfer.ChamferTypeEnum (enum)

The basic shape type of the chamfer.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/e17f4130-3566-bee7-b126-0ddfa6a77eea)

### Values
- `CHAMFER_NONE` = `0` — No chamfer type.
- `CHAMFER_LINE` = `1` — The line type chamfer.
- `CHAMFER_ROUNDING` = `2` — The rounding type chamfer.
- `CHAMFER_ARC` = `3` — The arc type chamfer.
- `CHAMFER_ARC_POINT` = `4` — The arc point type chamfer.
- `CHAMFER_SQUARE` = `5` — The square type chamfer.
- `CHAMFER_SQUARE_PARALLEL` = `6` — The square parallel type chamfer.
- `CHAMFER_LINE_AND_ARC` = `7` — The line and arc type chamfer.

## ChangeData (class)

The ChangeData class presents information about the detected change of the modelobject in the model. It is instantiated inside the connected method.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/71408208-1126-6f3d-f4a1-8e524f7875da)

### Properties
- `Object` (object, get/set) — Gets the changed object.
- `Source` (object, get/set) — Gets the change source type.
- `Type` (object, get/set) — Gets the change type.

## ChangeData.ChangeSourceTypeEnum (enum)

The source types of changed event.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/fe62fb09-f708-2f31-f3d4-57c7300fe40e)

### Values
- `COMMIT` = `0` — Normal commit
- `UNDO_REDO` = `1` — Undo or redo pressed
- `ROLLBACK` = `2` — Rollback of database

## ChangeData.ChangeTypeEnum (enum)

The types of changed event.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/64a75766-987a-6ed2-0a2d-553e8507a5fc)

### Values
- `OBJECT_INSERT` = `0` — Object created
- `OBJECT_MODIFY` = `1` — Object modified
- `OBJECT_DELETE` = `2` — Object deleted
- `USERPROPERTY_CHANGED` = `3` — User property of object changed

## CircleRebarGroup (class)

The CircleRebarGroup class represents a group of reinforcing bars which have a circular shape.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/8b1ad479-c856-307c-b1c3-87b6ad423bec)

### Constructors
- `CircleRebarGroup(...)` — Initializes a new circle rebar group instance with empty attributes.

### Methods
#### `CompareTo(...)`

Compares Identifiers of model objects.

[Docs](https://developer.tekla.com/topic/en/18/43/79cca356-0c8a-e8fb-463a-34ad4141bec7)

#### `Delete(...)`

Deletes the circle rebar group instance with the given identifier from the model database.

[Docs](https://developer.tekla.com/topic/en/18/43/96cbbcf2-fa63-ff32-f70b-12c9475071a2)

#### `Equals(...)`

Check if Identifiers of model objects are same.

[Docs](https://developer.tekla.com/topic/en/18/43/e4324265-2490-00de-4139-63a8f7acb492)

#### `GetAllReportProperties(...)`

Retrieves all properties for the model object in one hashtable. Type for the returned value must be checked using type casting.

[Docs](https://developer.tekla.com/topic/en/18/43/0708f404-ce77-28e1-4ebe-fa906f68bff8)

#### `GetAllUserProperties(...)`

Retrieves all properties for the model object in one hashtable. Type for the returned value must be checked using type casting.

[Docs](https://developer.tekla.com/topic/en/18/43/63b853a0-0af5-af9c-3ce7-05299664d7f8)

#### `GetAssembly(...)`

Returns the assembly that the reinforcement belongs to.

[Docs](https://developer.tekla.com/topic/en/18/43/322f932e-f09b-df0d-c1c8-3de07d79afeb)

#### `GetChildren(...)`

Returns an enumerator of all the children model objects.

[Docs](https://developer.tekla.com/topic/en/18/43/ce51ec40-310c-6067-a89f-f8d029aab747)

#### `GetCoordinateSystem(...)`

Returns the coordinate system for the given model object.

[Docs](https://developer.tekla.com/topic/en/18/43/8adfe9cd-f6e5-6474-1297-33c6270a7b0a)

#### `GetCustomObjectType(...)`

Gets custom object type name from an existing object.

[Docs](https://developer.tekla.com/topic/en/18/43/d3be2ba3-8533-7165-f2d3-a5aa9df556b8)

#### `GetDoubleReportProperties(...)`

Retrieves multiple double report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/f86f6c96-f738-49ca-16ba-f5634e862ba2)

#### `GetDoubleUserProperties(...)`

Retrieves all double properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/6ead7a63-5f76-0631-d480-d46c27e882aa)

#### `GetDynamicStringProperty(...)`

Gets a dynamic string property from the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/415e86d9-7de6-ea53-ce68-20b3e67b8655)

#### `GetFatherComponent(...)`

Returns the father component of the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/a2c28f3f-52ff-50a8-1b32-10d18db5c31d)

#### `GetFatherPour(...)`

Returns the pour that the rebar is associated to.

[Docs](https://developer.tekla.com/topic/en/18/43/0725e80c-89cf-d6d7-ac40-afdf75f6e695)

#### `GetFatherPourUnit(...)`

Returns the pour unit that the rebar is associated to.

[Docs](https://developer.tekla.com/topic/en/18/43/135a51ac-86cb-ef86-cfda-d2bc53a0d8b8)

#### `GetHierarchicObjects(...)`

Returns an enumerator of all the connected hierarchic objects.

[Docs](https://developer.tekla.com/topic/en/18/43/cc228ea0-267b-7d90-7441-c9efc1779b08)

#### `GetIntegerReportProperties(...)`

Retrieves multiple integer report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/2623a9c0-2a9c-e233-9daa-59f92ed51445)

#### `GetIntegerUserProperties(...)`

Retrieves all integer properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/1b3c6f06-bc25-a54e-2a88-b60dc4a3c85e)

#### `GetNumberOfRebars(...)`

Returns the number of rebars in the reinforcing group.

[Docs](https://developer.tekla.com/topic/en/18/43/6b02fd6f-4f84-6550-69c4-bd2f1117b922)

#### `GetPhase(...)`

Retrieves the phase of the model object (the phase number, the phase name, the phase comment and whether the phase is the current one or not).

[Docs](https://developer.tekla.com/topic/en/18/43/310f8b7a-c120-753a-35c9-f7c73a9806c9)

#### `GetRebarComplexGeometries(Boolean, Boolean, Boolean)(...)`

Retrieves a list of physical reinforcing bars (of type RebarComplexGeometry). These objects contain physical curves in the 3D space of each reinforcing bar as shown in model view. RebarComplexGeometry only differs from RebarGeometry by containing a polycurve rather than a polyline.

[Docs](https://developer.tekla.com/topic/en/18/43/c796aafb-3520-fe06-12e3-a07a87c8f2f9)

#### `GetRebarComplexGeometries(Boolean, Boolean, Boolean, Reinforcement.RebarGeometrySimplificationTypeEnum)(...)`

Retrieves a list of physical reinforcing bars (of type RebarComplexGeometry). These objects contain physical curves in the 3D space of each reinforcing bar as shown in model view. RebarComplexGeometry only differs from RebarGeometry by containing a polycurve (of Arc and LineSegment objects) rather than a polyline. The polycurve only contains arcs if the "simplified" parameter is RATIONALIZED or FABRICATION. If "simplified" is NONE, any arcs will be returned as a series of line segments.

[Docs](https://developer.tekla.com/topic/en/18/43/84a3a38b-d7eb-97df-7fba-a34d13de6610)

#### `GetRebarGeometries(Boolean)(...)`

Retrieves a list of physical reinforcing bars (of type RebarGeometry). These objects contain physical points in the 3D space of each reinforcing bar.

[Docs](https://developer.tekla.com/topic/en/18/43/75649a4c-6252-09a1-b509-cd815e718c0f)

#### `GetRebarGeometries(Reinforcement.RebarGeometryOptionEnum)(...)`

Retrieves a list of physical reinforcing bars (of type RebarGeometry). These objects contain physical points in the 3D space of each reinforcing bar.

[Docs](https://developer.tekla.com/topic/en/18/43/bc45309d-fc4b-9bf9-c640-e71bb1ade771)

#### `GetRebarGeometriesWithoutClashes(...)`

Retrieves a list of physical reinforcing bars (of type RebarGeometry). These objects contain physical points in the 3D space of each reinforcing bar as shown in model view. In case rebar polygon clashes with itself, physical points are moved to avoid clashing.

[Docs](https://developer.tekla.com/topic/en/18/43/470acf1f-1beb-f30a-a1c2-2fcab066ab4a)

#### `GetReportProperty(String, Double.)(...)`

Retrieves a double property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/1c05aa56-5f42-be91-a59e-8d59a5549f45)

#### `GetReportProperty(String, Int32.)(...)`

Retrieves an integer property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/ce8e4ca0-3750-3c9b-5ca8-27e48b45e0bd)

#### `GetReportProperty(String, String.)(...)`

Retrieves a string property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/70f212bb-ee70-e9e9-7ab6-d6a9d46f8de6)

#### `GetSingleRebar(...)`

Returns a single rebar inside the rebar group located by the given index. The indexing starts from the start point. The rebar represents a physical reinforcing bar and contains physical points in the 3D space of the bar. The method returns null on error, for example if given an erroneous index. The number of rebars in the group can be requested with GetNumberOfRebars().

[Docs](https://developer.tekla.com/topic/en/18/43/41181569-65f8-f4ba-7722-3cd890c7a3d4)

#### `GetSingleRebarWithoutClash(...)`

Returns a single rebar inside the rebar group located by the given index. The indexing starts from the start point. The rebar represents a physical reinforcing bar and contains physical, non-clashing points in the 3D space of the bar. The method returns null on error, for example if given an erroneous index. The number of rebars in the group can be requested with GetNumberOfRebars().

[Docs](https://developer.tekla.com/topic/en/18/43/8fd2e21a-3d2f-6211-92ba-9d6e652ac100)

#### `GetSolid(...)`

Method for getting the solid information of the reinforcement.

[Docs](https://developer.tekla.com/topic/en/18/43/6717d119-97da-5554-f359-49ee55e65a63)

#### `GetStringReportProperties(...)`

Retrieves multiple string report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/5ff72da1-7382-f5d2-eb1d-c99b46239d42)

#### `GetStringUserProperties(...)`

Retrieves all string properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/67120e84-c820-9d52-12ae-c167c595842c)

#### `GetUserProperty(String, Double.)(...)`

Retrieves a double property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/17cc8074-7955-32b5-2b4b-b169c45d4ee0)

#### `GetUserProperty(String, Int32.)(...)`

Retrieves an integer property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/d9364c1d-5b57-6a31-4c13-01124058cfb2)

#### `GetUserProperty(String, String.)(...)`

Retrieves a string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/214a886c-c18c-9b66-d7c9-fe84260962f2)

#### `Insert(...)`

Inserts the circle rebar group into the model database. All the attributes must be set.

[Docs](https://developer.tekla.com/topic/en/18/43/bbc15f10-8acb-3a04-ace8-a9b2042141f1)

#### `IsGeometryValid(...)`

Tells whether the geometry of a reinforcement object is valid or not.

[Docs](https://developer.tekla.com/topic/en/18/43/5699f6a6-4e39-b784-d84f-45eb7b1d0e37)

#### `Modify(...)`

Modifies the existing circle rebar group in the model database to match the current one.

[Docs](https://developer.tekla.com/topic/en/18/43/48bf80ea-6ea1-6f7e-4bcb-f4cd9ffbed44)

#### `Select(...)`

Selects a circle rebar group from the model database. The identifier of the circle rebar group must be set.

[Docs](https://developer.tekla.com/topic/en/18/43/978e4b4d-3dc8-2230-03c8-e801ff97f464)

#### `SetCustomObjectType(...)`

Sets a custom object type name for an existing object.

[Docs](https://developer.tekla.com/topic/en/18/43/3d6fb91b-48a9-35ca-d498-526514344089)

#### `SetDynamicStringProperty(...)`

Sets a dynamic string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/4dab0254-09e5-5e61-c28b-1c12afab71a0)

#### `SetLabel(...)`

Sets a label for an object when a new instance is created, this method must be called before Insert. The label is used in plug-ins for identifying the changed object in modification.

[Docs](https://developer.tekla.com/topic/en/18/43/3efcfc44-613a-e66b-9ebd-9f86df4f4036)

#### `SetPhase(...)`

Sets the phase of the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/ed80abcc-0ff7-d081-2229-e651f858a727)

#### `SetUserProperties(...)`

Sets multiple properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/8659da37-d571-f03f-c666-5c9fa3b113f5)

#### `SetUserProperty(String, Double)(...)`

Sets a double property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/5c585b8b-5c2e-7b75-2242-758f17429396)

#### `SetUserProperty(String, Int32)(...)`

Sets an integer property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/dd5b828d-3a87-bb58-5ac9-e1c3b4c2fe4b)

#### `SetUserProperty(String, String)(...)`

Sets a string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/80bf0348-b651-3d4a-862b-49c85bc924ab)

### Properties
- `Class` (object, get/set) — Gets or sets the class of the reinforcement. The class is used to group reinforcements.
- `EndFromPlaneOffset` (object, get/set) — The end offset value from the part surface.
- `EndHook` (object, get/set) — The hook at the end of the reinforcing bar.
- `EndPoint` (object, get/set) — The end point of the direction in which the bars are distributed.
- `EndPointOffsetType` (object, get/set) — Gets or sets the type of the end point offset. The options are: OFFSET_TYPE_LEG_LENGTHOFFSET_TYPE_COVER_THICKNESS
- `EndPointOffsetValue` (object, get/set) — Gets or sets the concrete cover thickness or leg length at the second end of the bar.
- `ExcludeType` (object, get/set) — Defines which bars to omit from the group. The options are: EXCLUDE_TYPE_NONEEXCLUDE_TYPE_FIRSTEXCLUDE_TYPE_LASTEXCLUDE_TYPE_BOTH
- `Father` (object, get/set) — Gets or sets the father object of the reinforcement; the model object instance to operate on.
- `FromPlaneOffset` (object, get/set) — The offset value from the part surface applied in both sides.
- `Grade` (object, get/set) — Gets or sets the steel grade of the reinforcing bar. The grade indicates the strength of the steel used in reinforcing bars. It can also indicate other factors, such as the weldability or surface deformations of the bar.
- `Identifier` (object, get/set) — The identifier of the object.
- `InputPointDeformingState` (object, get/set) — Gets or sets the reinforcement input point deforming state.
- `IsUpToDate` (object, get/set) — Gets if the object does not have a modification which is not shared.
- `ModificationTime` (object, get/set) — Gets latest time of the object was modified or created.
- `Name` (object, get/set) — Gets or sets the name of the reinforcement.
- `NumberingSeries` (object, get/set) — Gets or sets the numbering series of the reinforcement.
- `OnPlaneOffsets` (object, get/set) — Gets or sets the double offset value for each leg on the same plane as the bar.
- `Polygon` (object, get/set) — A polygon definition for the circle reinforcing bar group shape. The polygon must have 3 points.
- `RadiusValues` (object, get/set) — Gets or sets the radius value(s) of the bends in the bar.
- `Size` (object, get/set) — The size of the reinforcement.
- `Spacings` (object, get/set) — The spacing value(s). If the type of the spacing is SPACING_TYPE_EXACT_NUMBER Spacings has only one value that defines the number of the reinforcing bars.
- `SpacingType` (object, get/set) — The type of spacing. The options are (BaseRebarGroup.RebarGroupSpacingTypeEnum.): SPACING_TYPE_UNDEFINEDSPACING_TYPE_EXACT_SPACINGSSPACING_TYPE_EXACT_NUMBERSPACING_TYPE_TARGET_SPACESPACING_TYPE_EXACT_SPACE_FLEX_AT_STARTSPACING_TYPE_EXACT_SPACE_FLEX_AT_ENDSPACING_TYPE_EXACT_SPACE_FLEX_AT_BOTHSPACING_TYPE_EXACT_SPACE_FLEX_AT_MIDDLE
- `StartFromPlaneOffset` (object, get/set) — The start offset value from the part surface.
- `StartHook` (object, get/set) — The hook at the beginning of the reinforcing bar.
- `StartPoint` (object, get/set) — The start point of the direction in which the bars are distributed.
- `StartPointOffsetType` (object, get/set) — Gets or sets the type of the start point offset is either OFFSET_TYPE_LEG_LENGTH or OFFSET_TYPE_COVER_THICKNESS.
- `StartPointOffsetValue` (object, get/set) — Gets or sets the concrete cover thickness or leg length at the first end of the bar.
- `StirrupType` (object, get/set) — The type of the stirrup. The options are: STIRRUP_TYPE_CIRCLESTIRRUP_TYPE_SPIRAL

## CircleRebarGroup.CircleRebarGroupStirrupTypeEnum (enum)

The different stirrup types.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/1053de24-3e25-9b11-8b1c-30adc1a537b7)

### Values
- `STIRRUP_TYPE_CIRCLE` = `0` — The circle type of stirrup.
- `STIRRUP_TYPE_SPIRAL` = `1` — The spiral type of stirrup.

## ClashCheckData (class)

The ClashCheckData class presents information about the detected clash. It is instantiated inside the onClashDetected method. The objects store the identifiers and type of the clash to be passed on to event subscribers.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/2e113655-9825-9801-b873-2dc75b1549f1)

### Constructors
- `ClashCheckData(...)` — Initializes a new instance of the ClashCheckData class

### Properties
- `Object1` (object, get/set) — Gets the first clashing object.
- `Object2` (object, get/set) — Gets the second clashing object.
- `Overlap` (object, get/set) — Gets the overlap of the clash. Only applies to clashes of type CLASH_TYPE_CLASH.
- `Type` (object, get/set) — Gets the type of the clash.

## ClashCheckData.ClashTypeEnum (enum)

An enumeration for the different clash types.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/ef3402da-08ed-9713-6f0e-fcd6e9bfe78f)

### Values
- `CLASH_TYPE_INVALID` = `0` — The invalid clash type.
- `CLASH_TYPE_ISINSIDE` = `1` — One object is inside the other object.
- `CLASH_TYPE_EXACTMATCH` = `2` — The exact match clash type.
- `CLASH_TYPE_CLASH` = `3` — The unidentified clash type.
- `CLASH_TYPE_MINDISTANCE` = `4` — The minimum distance clash type.
- `CLASH_TYPE_FAILEDSOLID` = `5` — The failed solid clash type.
- `CLASH_TYPE_CUTTHROUGH` = `6` — The cut through clash type.
- `CLASH_TYPE_COMPLEXCLASH` = `7` — The complex clash type.
- `CLASH_TYPE_FAILEDTEST` = `8` — The failed test clash type.

## ClashCheckHandler (class)

The ClashCheckHandler class contains clash check methods.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/788112d7-7e99-4d1c-c631-4c1a23366696)

### Methods
#### `GetIntersectionBoundingBoxes(...)`

Get a list of bounding boxes (AABB) of the clashing volumes of two objects.

[Docs](https://developer.tekla.com/topic/en/18/43/73b07f50-4b4c-cfe3-1e89-d723db38ac7e)

#### `RunClashCheck(...)`

Starts the clash checking. Uses advanced options sa option values.

[Docs](https://developer.tekla.com/topic/en/18/43/866f01a1-9f7b-64ff-5131-476f1162d5db)

#### `RunClashCheckWithOptions(Boolean, Boolean, Boolean, Double, Boolean)(...)`

Starts the clash checking with options.

[Docs](https://developer.tekla.com/topic/en/18/43/69286754-a3b8-f783-0f47-ba35a9d72ff9)

#### `RunClashCheckWithOptions(Boolean, Boolean, Double, Boolean)(...)`

Starts the clash checking with options.

[Docs](https://developer.tekla.com/topic/en/18/43/efdedd8f-706d-e77e-0f49-3647b6a78ebe)

#### `StopClashCheck(...)`

Stops the clash checking.

[Docs](https://developer.tekla.com/topic/en/18/43/a392ba3f-5f87-3d4d-e65e-15c1960e947a)

## Component (class)

The Component class represents a component. A component is a modelling tool that typically assembles multiple parts to build some type of a structure, for example a hall macro or at a smaller scale a reinforcement component (a macro). Also, a custom part is a component.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/63586110-d60f-f5a1-2bce-c5138003cbad)

### Constructors
- `Component(...)` — Creates a new component instance.
- `Component(...)` — Creates a new component instance with the given component input.

### Methods
#### `CompareTo(...)`

Compares Identifiers of model objects.

[Docs](https://developer.tekla.com/topic/en/18/43/79cca356-0c8a-e8fb-463a-34ad4141bec7)

#### `Delete(...)`

Deletes the component instance with the given ID from the model database.

[Docs](https://developer.tekla.com/topic/en/18/43/d72b85cf-584f-15a4-063d-c20f2d4772fa)

#### `Equals(...)`

Check if Identifiers of model objects are same.

[Docs](https://developer.tekla.com/topic/en/18/43/e4324265-2490-00de-4139-63a8f7acb492)

#### `GetAllReportProperties(...)`

Retrieves all properties for the model object in one hashtable. Type for the returned value must be checked using type casting.

[Docs](https://developer.tekla.com/topic/en/18/43/0708f404-ce77-28e1-4ebe-fa906f68bff8)

#### `GetAllUserProperties(...)`

Retrieves all properties for the model object in one hashtable. Type for the returned value must be checked using type casting.

[Docs](https://developer.tekla.com/topic/en/18/43/63b853a0-0af5-af9c-3ce7-05299664d7f8)

#### `GetAssembly(...)`

Returns the assembly that the component belongs to.

[Docs](https://developer.tekla.com/topic/en/18/43/651b7a6d-7887-dedc-ca17-23ea603f1703)

#### `GetAttribute(String, Double.)(...)`

Retrieves the attribute with the given name.

[Docs](https://developer.tekla.com/topic/en/18/43/a347ce9e-865f-554d-140f-9a65728b24a3)

#### `GetAttribute(String, Int32.)(...)`

Retrieves the attribute with the given name.

[Docs](https://developer.tekla.com/topic/en/18/43/2c48112c-5580-99f3-6c35-70fd5f538b09)

#### `GetAttribute(String, String.)(...)`

Retrieves the attribute with the given name.

[Docs](https://developer.tekla.com/topic/en/18/43/c8652aa1-9069-88f2-b821-3afdd68e7ca5)

#### `GetBooleans(...)`

Returns an enumerator of all the connected boolean objects.

[Docs](https://developer.tekla.com/topic/en/18/43/6aff754f-edce-a4c9-f8d8-8bfef0badaa4)

#### `GetChildren(...)`

Returns an enumerator of all the children model objects.

[Docs](https://developer.tekla.com/topic/en/18/43/ce51ec40-310c-6067-a89f-f8d029aab747)

#### `GetComponentInput(...)`

Returns the component input object.

[Docs](https://developer.tekla.com/topic/en/18/43/9ace2477-8ba2-a8d6-555b-40eaeeb6b009)

#### `GetComponents(...)`

Returns an enumerator of all the connected components.

[Docs](https://developer.tekla.com/topic/en/18/43/c459a16c-07b7-cf83-0c71-8a053abd4865)

#### `GetCoordinateSystem(...)`

Returns the coordinate system for the given model object.

[Docs](https://developer.tekla.com/topic/en/18/43/8adfe9cd-f6e5-6474-1297-33c6270a7b0a)

#### `GetCustomObjectType(...)`

Gets custom object type name from an existing object.

[Docs](https://developer.tekla.com/topic/en/18/43/d3be2ba3-8533-7165-f2d3-a5aa9df556b8)

#### `GetDoubleReportProperties(...)`

Retrieves multiple double report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/f86f6c96-f738-49ca-16ba-f5634e862ba2)

#### `GetDoubleUserProperties(...)`

Retrieves all double properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/6ead7a63-5f76-0631-d480-d46c27e882aa)

#### `GetDynamicStringProperty(...)`

Gets a dynamic string property from the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/415e86d9-7de6-ea53-ce68-20b3e67b8655)

#### `GetFatherComponent(...)`

Returns the father component of the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/a2c28f3f-52ff-50a8-1b32-10d18db5c31d)

#### `GetHierarchicObjects(...)`

Returns an enumerator of all the connected hierarchic objects.

[Docs](https://developer.tekla.com/topic/en/18/43/cc228ea0-267b-7d90-7441-c9efc1779b08)

#### `GetIntegerReportProperties(...)`

Retrieves multiple integer report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/2623a9c0-2a9c-e233-9daa-59f92ed51445)

#### `GetIntegerUserProperties(...)`

Retrieves all integer properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/1b3c6f06-bc25-a54e-2a88-b60dc4a3c85e)

#### `GetPhase(...)`

Retrieves the phase of the model object (the phase number, the phase name, the phase comment and whether the phase is the current one or not).

[Docs](https://developer.tekla.com/topic/en/18/43/310f8b7a-c120-753a-35c9-f7c73a9806c9)

#### `GetReportProperty(String, Double.)(...)`

Retrieves a double property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/1c05aa56-5f42-be91-a59e-8d59a5549f45)

#### `GetReportProperty(String, Int32.)(...)`

Retrieves an integer property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/ce8e4ca0-3750-3c9b-5ca8-27e48b45e0bd)

#### `GetReportProperty(String, String.)(...)`

Retrieves a string property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/70f212bb-ee70-e9e9-7ab6-d6a9d46f8de6)

#### `GetStringReportProperties(...)`

Retrieves multiple string report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/5ff72da1-7382-f5d2-eb1d-c99b46239d42)

#### `GetStringUserProperties(...)`

Retrieves all string properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/67120e84-c820-9d52-12ae-c167c595842c)

#### `GetUserProperty(String, Double.)(...)`

Retrieves a double property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/17cc8074-7955-32b5-2b4b-b169c45d4ee0)

#### `GetUserProperty(String, Int32.)(...)`

Retrieves an integer property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/d9364c1d-5b57-6a31-4c13-01124058cfb2)

#### `GetUserProperty(String, String.)(...)`

Retrieves a string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/214a886c-c18c-9b66-d7c9-fe84260962f2)

#### `Insert(...)`

Inserts the component into the model database. The component input needs to be set.

[Docs](https://developer.tekla.com/topic/en/18/43/f8f93ae2-830e-5687-0823-7c591cd7a774)

#### `LoadAttributesFromFile(...)`

Loads the attributes for the component from the given file. These attributes will be loaded before all the attributes that have been set with the SetAttribute methods, so any attributes that are set with SetAttribute will override those loaded from the given standard file.

[Docs](https://developer.tekla.com/topic/en/18/43/acc9930a-aa6f-667c-0878-7ea80ce416b8)

#### `Modify(...)`

Modifies the existing component in the model database to match the current one.

[Docs](https://developer.tekla.com/topic/en/18/43/77e8ce39-2b89-47bc-709d-089efe911f44)

#### `Select(...)`

Selects a component from the model database. The component ID must be set.

[Docs](https://developer.tekla.com/topic/en/18/43/ff572395-ed37-fc34-9737-1f4f9dfbed63)

#### `SetAttribute(String, Double)(...)`

Sets the attribute's value to the given value.

[Docs](https://developer.tekla.com/topic/en/18/43/785d0216-df12-53fc-344b-a2e67001cf3c)

#### `SetAttribute(String, Int32)(...)`

Sets the attribute's value to the given value.

[Docs](https://developer.tekla.com/topic/en/18/43/797240ce-567c-bff9-2a60-6359cc909ce0)

#### `SetAttribute(String, String)(...)`

Sets the attribute's value to the given value.

[Docs](https://developer.tekla.com/topic/en/18/43/6f0f7c8f-c5d2-8012-9faf-f68991966ff5)

#### `SetComponentInput(...)`

Sets the component input object for the component. The component input object contains all the input objects and positions that are needed for the component creation.

[Docs](https://developer.tekla.com/topic/en/18/43/c5d1f598-8bf5-9246-f43a-51b1e089459b)

#### `SetCustomObjectType(...)`

Sets a custom object type name for an existing object.

[Docs](https://developer.tekla.com/topic/en/18/43/3d6fb91b-48a9-35ca-d498-526514344089)

#### `SetDynamicStringProperty(...)`

Sets a dynamic string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/4dab0254-09e5-5e61-c28b-1c12afab71a0)

#### `SetLabel(...)`

Sets a label for an object when a new instance is created, this method must be called before Insert. The label is used in plug-ins for identifying the changed object in modification.

[Docs](https://developer.tekla.com/topic/en/18/43/3efcfc44-613a-e66b-9ebd-9f86df4f4036)

#### `SetPhase(...)`

Sets the phase of the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/ed80abcc-0ff7-d081-2229-e651f858a727)

#### `SetUserProperties(...)`

Sets multiple properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/8659da37-d571-f03f-c666-5c9fa3b113f5)

#### `SetUserProperty(String, Double)(...)`

Sets a double property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/5c585b8b-5c2e-7b75-2242-758f17429396)

#### `SetUserProperty(String, Int32)(...)`

Sets an integer property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/dd5b828d-3a87-bb58-5ac9-e1c3b4c2fe4b)

#### `SetUserProperty(String, String)(...)`

Sets a string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/80bf0348-b651-3d4a-862b-49c85bc924ab)

### Properties
- `Identifier` (object, get/set) — The identifier of the object.
- `IsUpToDate` (object, get/set) — Gets if the object does not have a modification which is not shared.
- `ModificationTime` (object, get/set) — Gets latest time of the object was modified or created.
- `Name` (object, get/set) — The name of the component. The name identifies custom components or plug-ins.
- `Number` (object, get/set) — The number of the component. A number greater than zero identifies system components, for custom components the number is CUSTOM_OBJECT_NUMBER, and for plug-ins the number is PLUGIN_OBJECT_NUMBER.

## ComponentInput (class)

The ComponentInput class handles the input of component objects and positions. A component has one component input. The component input must include all the needed input objects and points in the correct order. For example, an input with a point and a part is different from an input of a part and a point. The maximum number of different input items in the collection is 10.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/2e678a2a-118b-ca18-2776-91cd29d4540c)

### Constructors
- `ComponentInput(...)` — Creates a new component input object.

### Methods
#### `AddInputObject(...)`

Adds a model object to the component input.

[Docs](https://developer.tekla.com/topic/en/18/43/a5d879e1-4952-2600-40db-fdab2637adae)

#### `AddInputObjects(...)`

Adds an array list of model objects to the component input.

[Docs](https://developer.tekla.com/topic/en/18/43/f7711bd2-4bf6-9b33-dd62-dc5702e62a2b)

#### `AddInputPolygon(...)`

Adds a polygon to the component input.

[Docs](https://developer.tekla.com/topic/en/18/43/d50f8295-7d7b-90ce-e1a2-4a38b0a05826)

#### `AddOneInputPosition(...)`

Adds an input position to the component input.

[Docs](https://developer.tekla.com/topic/en/18/43/8aca8df7-49a3-0a9c-b4cd-66ba8b611345)

#### `AddTwoInputPositions(...)`

Adds two input positions to the component input.

[Docs](https://developer.tekla.com/topic/en/18/43/63f0c2d2-025a-9c81-915f-fd8df69f744a)

#### `CopyTo(...)`

Copies the elements of the ICollection to an Array, starting at a particular Array index.

[Docs](https://developer.tekla.com/topic/en/18/43/42454497-9e17-cd6b-cda6-b7ec6decbe1f)

#### `GetEnumerator(...)`

Returns an enumerator that iterates through a collection.

[Docs](https://developer.tekla.com/topic/en/18/43/c3e1a0fc-7e10-3647-deff-3a18bc3fb8c1)

### Properties
- `Count` (object, get/set) — Gets the number of elements contained in the ICollection.
- `IsSynchronized` (object, get/set) — Gets a value indicating whether access to the ICollection is synchronized (thread safe).
- `SyncRoot` (object, get/set) — Gets an object that can be used to synchronize access to the ICollection.

## ConicalSurface (class)

The ConicalSurface class defines a conical surface contour.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/782c39eb-4018-ceec-cbf2-c8502d926da0)

### Constructors
- `ConicalSurface(...)` — Initializes a new instance of the ConicalSurface class from a base arc, a height and a radius at the top. This is the recommended constructor if no intermediate points are desired along the lateral boundaries of the bend surface.
- `ConicalSurface(...)` — Initializes a new instance of the ConicalSurface class from the cone lateral boundaries and the center line. This is the recommended constructor if fine grained control is desired over the lateral boundaries of the surface.
- `ConicalSurface(...)` — Initializes a new instance of the ConicalSurface class with given parameters. The boundaries are defined by the lateral boundaries (i.e. the points on the sides of the curved part of the cone). The side boundaries are defined by the two first points and the two last points of the lateral boundaries.
- `ConicalSurface(...)` — Initializes a new instance of the ConicalSurface class with given parameters. The boundaries are defined by the side boundaries (i.e. generator lines of the cone).

### Properties
- `Apex` (object, get/set) — Gets the apex of the cone. If it does not exist, it returns null
- `CenterLine` (object, get/set) — Gets the center line of the bend surface (i.e. the line that crosses every circular cross section of the bend at their center point)
- `EndFaceNormal1` (object, get/set) — Gets the first end face normal.
- `EndFaceNormal2` (object, get/set) — Gets the second end face normal.
- `IntersectionLine` (object, get/set) — Gets intersection line.
- `InwardCurved` (object, get/set) — Gets the direction of the curve - true if the curve is oriented towards the intersection line, false otherwise.
- `LateralBoundary1` (object, get/set) — Gets or sets the first lateral boundary
- `LateralBoundary2` (object, get/set) — Gets or sets the second lateral boundary
- `Radiuses` (object, get/set) — Gets radiuses of the conical surface. The first radius corresponds to the section located in the first point of the side boundary 1, and the second radius corresponds to the section located in the second point of the side boundary 1.
- `RotationAxis` (object, get/set) — Gets the unit vector that defines the rotation axis (for counter clockwise rotations) of the lateral boundaries of the surface. This axis is always parallel to the direction of the center line.
- `SideBoundary1` (object, get/set) — Gets or sets the first side boundary.
- `SideBoundary2` (object, get/set) — Gets or sets the second side boundary.

## ConicalSurfaceNode (class)

The ConicalSurfaceNode class represents conical surface geometry tree node.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/73922bd2-0f50-6751-da49-df7948f18e6a)

### Constructors
- `ConicalSurfaceNode(...)` — Initializes a new instance of the ConicalSurfaceNode class.

### Methods
#### `AcceptVisitor(...)`

Entry method for the visitor pattern.

[Docs](https://developer.tekla.com/topic/en/18/43/c4b3e931-c16e-9154-96b5-9c12cb81fd42)

#### `Clone(...)`

Creates a new object that is a copy of the current instance.

[Docs](https://developer.tekla.com/topic/en/18/43/3c51f2ca-57da-3ecc-8a08-bba604b4c6b0)

### Properties
- `IsAutomatic` (object, get/set) — Gets a value indicating whether this geometry node was automatically generated (returns false if it was originally a user-defined part)
- `Surface` (object, get/set) — Gets the conical surface geometry.

## Connection (class)

The Connection class represents a connection. A connection is something that connects two or more parts together.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/41a1ee09-b531-3160-137a-37aefe3f6947)

### Constructors
- `Connection(...)` — Creates a new connection instance.

### Methods
#### `CompareTo(...)`

Compares Identifiers of model objects.

[Docs](https://developer.tekla.com/topic/en/18/43/79cca356-0c8a-e8fb-463a-34ad4141bec7)

#### `Delete(...)`

Deletes the connection instance with the given ID from the model database.

[Docs](https://developer.tekla.com/topic/en/18/43/b95f35af-c9b3-0b67-86f4-99d303a632b4)

#### `Equals(...)`

Check if Identifiers of model objects are same.

[Docs](https://developer.tekla.com/topic/en/18/43/e4324265-2490-00de-4139-63a8f7acb492)

#### `GetAllReportProperties(...)`

Retrieves all properties for the model object in one hashtable. Type for the returned value must be checked using type casting.

[Docs](https://developer.tekla.com/topic/en/18/43/0708f404-ce77-28e1-4ebe-fa906f68bff8)

#### `GetAllUserProperties(...)`

Retrieves all properties for the model object in one hashtable. Type for the returned value must be checked using type casting.

[Docs](https://developer.tekla.com/topic/en/18/43/63b853a0-0af5-af9c-3ce7-05299664d7f8)

#### `GetAttribute(String, Double.)(...)`

Retrieves the attribute with the given name.

[Docs](https://developer.tekla.com/topic/en/18/43/a347ce9e-865f-554d-140f-9a65728b24a3)

#### `GetAttribute(String, Int32.)(...)`

Retrieves the attribute with the given name.

[Docs](https://developer.tekla.com/topic/en/18/43/2c48112c-5580-99f3-6c35-70fd5f538b09)

#### `GetAttribute(String, String.)(...)`

Retrieves the attribute with the given name.

[Docs](https://developer.tekla.com/topic/en/18/43/c8652aa1-9069-88f2-b821-3afdd68e7ca5)

#### `GetChildren(...)`

Returns an enumerator of all the children model objects.

[Docs](https://developer.tekla.com/topic/en/18/43/ce51ec40-310c-6067-a89f-f8d029aab747)

#### `GetCoordinateSystem(...)`

Returns the coordinate system for the given model object.

[Docs](https://developer.tekla.com/topic/en/18/43/8adfe9cd-f6e5-6474-1297-33c6270a7b0a)

#### `GetCustomObjectType(...)`

Gets custom object type name from an existing object.

[Docs](https://developer.tekla.com/topic/en/18/43/d3be2ba3-8533-7165-f2d3-a5aa9df556b8)

#### `GetDoubleReportProperties(...)`

Retrieves multiple double report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/f86f6c96-f738-49ca-16ba-f5634e862ba2)

#### `GetDoubleUserProperties(...)`

Retrieves all double properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/6ead7a63-5f76-0631-d480-d46c27e882aa)

#### `GetDynamicStringProperty(...)`

Gets a dynamic string property from the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/415e86d9-7de6-ea53-ce68-20b3e67b8655)

#### `GetFatherComponent(...)`

Returns the father component of the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/a2c28f3f-52ff-50a8-1b32-10d18db5c31d)

#### `GetHierarchicObjects(...)`

Returns an enumerator of all the connected hierarchic objects.

[Docs](https://developer.tekla.com/topic/en/18/43/cc228ea0-267b-7d90-7441-c9efc1779b08)

#### `GetIntegerReportProperties(...)`

Retrieves multiple integer report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/2623a9c0-2a9c-e233-9daa-59f92ed51445)

#### `GetIntegerUserProperties(...)`

Retrieves all integer properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/1b3c6f06-bc25-a54e-2a88-b60dc4a3c85e)

#### `GetPhase(...)`

Retrieves the phase of the model object (the phase number, the phase name, the phase comment and whether the phase is the current one or not).

[Docs](https://developer.tekla.com/topic/en/18/43/310f8b7a-c120-753a-35c9-f7c73a9806c9)

#### `GetPrimaryObject(...)`

Returns the primary object of the connection.

[Docs](https://developer.tekla.com/topic/en/18/43/1a5c0bb2-f44c-c3d0-39b5-c6c45da17304)

#### `GetReportProperty(String, Double.)(...)`

Retrieves a double property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/1c05aa56-5f42-be91-a59e-8d59a5549f45)

#### `GetReportProperty(String, Int32.)(...)`

Retrieves an integer property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/ce8e4ca0-3750-3c9b-5ca8-27e48b45e0bd)

#### `GetReportProperty(String, String.)(...)`

Retrieves a string property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/70f212bb-ee70-e9e9-7ab6-d6a9d46f8de6)

#### `GetSecondaryObjects(...)`

Returns the secondary objects.

[Docs](https://developer.tekla.com/topic/en/18/43/10c1bece-ac84-8975-141f-bd263ca9f5d5)

#### `GetStringReportProperties(...)`

Retrieves multiple string report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/5ff72da1-7382-f5d2-eb1d-c99b46239d42)

#### `GetStringUserProperties(...)`

Retrieves all string properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/67120e84-c820-9d52-12ae-c167c595842c)

#### `GetUserProperty(String, Double.)(...)`

Retrieves a double property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/17cc8074-7955-32b5-2b4b-b169c45d4ee0)

#### `GetUserProperty(String, Int32.)(...)`

Retrieves an integer property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/d9364c1d-5b57-6a31-4c13-01124058cfb2)

#### `GetUserProperty(String, String.)(...)`

Retrieves a string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/214a886c-c18c-9b66-d7c9-fe84260962f2)

#### `Insert(...)`

Inserts the connection into the model database. All the attributes must be set.

[Docs](https://developer.tekla.com/topic/en/18/43/5dee6835-18b6-486b-5c21-1eb0b3506398)

#### `LoadAttributesFromFile(...)`

Loads the attributes for the component from the given file. These attributes will be loaded before all the attributes that have been set with the SetAttribute methods, so any attributes that are set with SetAttribute will override those loaded from the given standard file.

[Docs](https://developer.tekla.com/topic/en/18/43/acc9930a-aa6f-667c-0878-7ea80ce416b8)

#### `Modify(...)`

Modifies the existing connection in the model database to match the current one.

[Docs](https://developer.tekla.com/topic/en/18/43/a3e7e509-d85b-17f7-6dc0-9a5cdf22d1c4)

#### `Select(...)`

Selects a connection from the model database. The connection ID must be set.

[Docs](https://developer.tekla.com/topic/en/18/43/2bd7cde8-69b2-bdd3-d6cd-136d434b56ab)

#### `SetAttribute(String, Double)(...)`

Sets the attribute's value to the given value.

[Docs](https://developer.tekla.com/topic/en/18/43/785d0216-df12-53fc-344b-a2e67001cf3c)

#### `SetAttribute(String, Int32)(...)`

Sets the attribute's value to the given value.

[Docs](https://developer.tekla.com/topic/en/18/43/797240ce-567c-bff9-2a60-6359cc909ce0)

#### `SetAttribute(String, String)(...)`

Sets the attribute's value to the given value.

[Docs](https://developer.tekla.com/topic/en/18/43/6f0f7c8f-c5d2-8012-9faf-f68991966ff5)

#### `SetCustomObjectType(...)`

Sets a custom object type name for an existing object.

[Docs](https://developer.tekla.com/topic/en/18/43/3d6fb91b-48a9-35ca-d498-526514344089)

#### `SetDynamicStringProperty(...)`

Sets a dynamic string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/4dab0254-09e5-5e61-c28b-1c12afab71a0)

#### `SetLabel(...)`

Sets a label for an object when a new instance is created, this method must be called before Insert. The label is used in plug-ins for identifying the changed object in modification.

[Docs](https://developer.tekla.com/topic/en/18/43/3efcfc44-613a-e66b-9ebd-9f86df4f4036)

#### `SetPhase(...)`

Sets the phase of the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/ed80abcc-0ff7-d081-2229-e651f858a727)

#### `SetPrimaryObject(...)`

Sets the primary object of the connection.

[Docs](https://developer.tekla.com/topic/en/18/43/d06340d6-3e52-08d5-279d-e2d7dc218db0)

#### `SetSecondaryObject(...)`

Sets the secondary object of the connection. Use this method if you wish to add only one secondary object to the connection.

[Docs](https://developer.tekla.com/topic/en/18/43/51a262d8-a359-8e02-39d4-bdd7d5c3f654)

#### `SetSecondaryObjects(...)`

Sets an array list of model objects as the secondary objects of the connection.

[Docs](https://developer.tekla.com/topic/en/18/43/75665aab-0c32-9fbd-28f7-362d018c52f7)

#### `SetUserProperties(...)`

Sets multiple properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/8659da37-d571-f03f-c666-5c9fa3b113f5)

#### `SetUserProperty(String, Double)(...)`

Sets a double property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/5c585b8b-5c2e-7b75-2242-758f17429396)

#### `SetUserProperty(String, Int32)(...)`

Sets an integer property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/dd5b828d-3a87-bb58-5ac9-e1c3b4c2fe4b)

#### `SetUserProperty(String, String)(...)`

Sets a string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/80bf0348-b651-3d4a-862b-49c85bc924ab)

### Properties
- `AutoDirectionType` (object, get/set) — The auto direction type. Default value is AUTODIR_BASIC. Value should be AUTODIR_NA to set direction with UpVector property. Value should be AUTODIR_FROM_ATTRIBUTE_FILE to set direction with connection attribute file.
- `Class` (object, get/set) — The class of the connection.
- `Code` (object, get/set) — The code of the connection. The code can be used to classify the connection. The code of the connection can be reported and shown in drawings. The maximum length of the string is 20 characters.
- `Identifier` (object, get/set) — The identifier of the object.
- `IsUpToDate` (object, get/set) — Gets if the object does not have a modification which is not shared.
- `ModificationTime` (object, get/set) — Gets latest time of the object was modified or created.
- `Name` (object, get/set) — The name of the component. The name identifies custom components or plug-ins.
- `Number` (object, get/set) — The number of the component. A number greater than zero identifies system components, for custom components the number is CUSTOM_OBJECT_NUMBER, and for plug-ins the number is PLUGIN_OBJECT_NUMBER.
- `PositionType` (object, get/set) — The position type. When creating custom connections this attribute is not applied, instead the position type used when creating the custom connection is used.
- `Status` (object, get/set) — The read only status of the connection. The status can be reported and shown in drawings. The color of the connection symbol in the model indicates the status of the connection.
- `UpVector` (object, get/set) — A vector indicating which direction is considered the up direction for the connection.

## ConnectiveGeometry (class)

A class for geometry formed of several geometry sections. Geometry is initialized with polygon geometry and then can be extended by calling one of AddLeg methods using BentPlateGeometrySolver class. In the case of success, two polygon geometries will be connected by cylindrical surface geometry possibly with help of additional polygon geometries. Resulted geometry, then, can be set to bent plate.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/f376b309-e3fe-c8f6-d8cd-805695550b9d)

### Constructors
- `ConnectiveGeometry(...)` — Initializes a new instance of the ConnectiveGeometry class for a stand-alone bend creation.
- `ConnectiveGeometry(...)` — Initializes a new instance of the ConnectiveGeometry class.

### Methods
#### `GetConnection(...)`

Returns 2 connecting line segments between geometrySection1 and geometrySection2 sections, if sections are connected, otherwise returns empty list.

[Docs](https://developer.tekla.com/topic/en/18/43/97fdc8ce-5c44-0d5b-0cf1-0477b2bf7ee7)

#### `GetGeometryEnumerator(...)`

Gets a new geometry sections enumerator for enumerating through the geometry sections.

[Docs](https://developer.tekla.com/topic/en/18/43/4b345f2f-7977-8d1d-bfe8-df07f99c4c91)

#### `GetGeometryLegSections(...)`

Gets geometry end GeometrySections which are possible to remove.

[Docs](https://developer.tekla.com/topic/en/18/43/68035fba-42ff-c225-5b85-2fd9b6664c21)

#### `GetNeighborSections(...)`

Gets neighbor sections of the geometrySection inside this ConnectiveGeometry.

[Docs](https://developer.tekla.com/topic/en/18/43/7ffd60f3-2ecb-c6fe-beba-c9e01964d2db)

#### `IsEmpty(...)`

Checks whether the geometry is degenerate

[Docs](https://developer.tekla.com/topic/en/18/43/a5b27561-22c1-e67a-ba34-971391059a30)

## ConnectiveGeometryException (class)

The ConnectiveGeometryException class represents an error that occurred during the ConnectiveGeometry creation or modification.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/bb701f1a-d0bc-4b22-23ab-8484219793af)

### Constructors
- `ConnectiveGeometryException(...)` — Initializes a new instance of the ConnectiveGeometryException class with general connective geometry error message.
- `ConnectiveGeometryException(...)` — Initializes a new instance of the ConnectiveGeometryException class with connective geometry error message based on status.

## Contour (class)

The Contour class defines a possibly chamfered contour. Contours must not have loops.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/903abf10-4c92-e708-533a-88c7cebfd501)

### Constructors
- `Contour(...)` — Instantiates an empty contour.

### Methods
#### `AddContourPoint(...)`

Adds a new contour point to the contour. Do not add the same contour point twice.

[Docs](https://developer.tekla.com/topic/en/18/43/758831d8-d612-1ccd-aa64-e3c0d07baa1d)

#### `CalculatePolygon(...)`

Calculate a polygon approximating the contour with chamfers evaluated.

[Docs](https://developer.tekla.com/topic/en/18/43/f5c142bc-8a2b-dee2-5b19-26863e8410a0)

#### `GetPolycurve(...)`

Gets the contour geometry as Polycurve

[Docs](https://developer.tekla.com/topic/en/18/43/1a21df7a-cb44-dada-7422-69a6052fad72)

### Properties
- `ContourPoints` (object, get/set) — The contour points belonging to the contour.

## ContourPlate (class)

The ContourPlate class represents a part made with a contour, such as, a concrete slab.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/ec399c5b-d8e9-a69a-57c8-d85f70ac3c51)

### Constructors
- `ContourPlate(...)` — Initializes a new instance of the ContourPlate class.

### Methods
#### `AddContourPoint(...)`

Adds a contour point to the contour plate.

[Docs](https://developer.tekla.com/topic/en/18/43/e76c9697-5ba1-7d4a-69d0-62f148d12938)

#### `CompareTo(Object)(...)`

Compares Identifiers of model objects.

[Docs](https://developer.tekla.com/topic/en/18/43/79cca356-0c8a-e8fb-463a-34ad4141bec7)

#### `CompareTo(Part)(...)`

Compares the instantiated part with another one.

[Docs](https://developer.tekla.com/topic/en/18/43/e8632c0d-89cf-9aab-6532-f62f9873067b)

#### `Delete(...)`

Deletes the contour plate instance with the given identifier from the model database.

[Docs](https://developer.tekla.com/topic/en/18/43/dde078ae-650d-2dee-1446-ab354c04dbda)

#### `Equals(...)`

Check if Identifiers of model objects are same.

[Docs](https://developer.tekla.com/topic/en/18/43/e4324265-2490-00de-4139-63a8f7acb492)

#### `GetAllReportProperties(...)`

Retrieves all properties for the model object in one hashtable. Type for the returned value must be checked using type casting.

[Docs](https://developer.tekla.com/topic/en/18/43/0708f404-ce77-28e1-4ebe-fa906f68bff8)

#### `GetAllUserProperties(...)`

Retrieves all properties for the model object in one hashtable. Type for the returned value must be checked using type casting.

[Docs](https://developer.tekla.com/topic/en/18/43/63b853a0-0af5-af9c-3ce7-05299664d7f8)

#### `GetAssembly(...)`

Returns the assembly that the part belongs to.

[Docs](https://developer.tekla.com/topic/en/18/43/bda18562-45a1-1355-42ed-6044af0b18cb)

#### `GetBolts(...)`

Returns an enumerator of all the connected bolts.

[Docs](https://developer.tekla.com/topic/en/18/43/dbc6b475-7461-7ec4-e8ec-7b7d3d8ff429)

#### `GetBooleans(...)`

Returns an enumerator of all the connected boolean objects.

[Docs](https://developer.tekla.com/topic/en/18/43/ae180e43-88e2-d759-0ae0-567d52681a2a)

#### `GetCenterLine(...)`

Returns the center line for the given part.

[Docs](https://developer.tekla.com/topic/en/18/43/bdc886b8-af7a-722c-dcb8-fddb2f250a83)

#### `GetChildren(...)`

Returns an enumerator of all the children model objects.

[Docs](https://developer.tekla.com/topic/en/18/43/ce51ec40-310c-6067-a89f-f8d029aab747)

#### `GetComponents(...)`

Returns an enumerator of all the connected components, connections, seams and details inherited from the base component.

[Docs](https://developer.tekla.com/topic/en/18/43/ed57941c-2cd8-3b78-6a6e-848b8a920298)

#### `GetContourPolycurve(...)`

Get contour as polycurve

[Docs](https://developer.tekla.com/topic/en/18/43/e0fcc8a4-baaf-b32e-bb1b-b5432b52ab6e)

#### `GetCoordinateSystem(...)`

Returns the coordinate system for the given model object.

[Docs](https://developer.tekla.com/topic/en/18/43/8adfe9cd-f6e5-6474-1297-33c6270a7b0a)

#### `GetCustomObjectType(...)`

Gets custom object type name from an existing object.

[Docs](https://developer.tekla.com/topic/en/18/43/d3be2ba3-8533-7165-f2d3-a5aa9df556b8)

#### `GetDSTVCoordinateSystem(...)`

Get DSTV coordinate system.

[Docs](https://developer.tekla.com/topic/en/18/43/9e70005f-c701-ab99-8fa0-4fce47b1ab05)

#### `GetDoubleReportProperties(...)`

Retrieves multiple double report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/f86f6c96-f738-49ca-16ba-f5634e862ba2)

#### `GetDoubleUserProperties(...)`

Retrieves all double properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/6ead7a63-5f76-0631-d480-d46c27e882aa)

#### `GetDynamicStringProperty(...)`

Gets a dynamic string property from the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/415e86d9-7de6-ea53-ce68-20b3e67b8655)

#### `GetFatherComponent(...)`

Returns the father component of the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/a2c28f3f-52ff-50a8-1b32-10d18db5c31d)

#### `GetHierarchicObjects(...)`

Returns an enumerator of all the connected hierarchic objects.

[Docs](https://developer.tekla.com/topic/en/18/43/cc228ea0-267b-7d90-7441-c9efc1779b08)

#### `GetIntegerReportProperties(...)`

Retrieves multiple integer report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/2623a9c0-2a9c-e233-9daa-59f92ed51445)

#### `GetIntegerUserProperties(...)`

Retrieves all integer properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/1b3c6f06-bc25-a54e-2a88-b60dc4a3c85e)

#### `GetPartMark(...)`

Returns the part mark.

[Docs](https://developer.tekla.com/topic/en/18/43/9978ba28-6304-714d-815c-fe4a6bb77076)

#### `GetPhase(...)`

Retrieves the phase of the model object (the phase number, the phase name, the phase comment and whether the phase is the current one or not).

[Docs](https://developer.tekla.com/topic/en/18/43/310f8b7a-c120-753a-35c9-f7c73a9806c9)

#### `GetPours(...)`

Returns an enumerator of all the pours that the part belongs to.

[Docs](https://developer.tekla.com/topic/en/18/43/1cc67ebb-329a-4675-7857-af663602d21d)

#### `GetReferenceLine(...)`

Returns the reference line for the given part.

[Docs](https://developer.tekla.com/topic/en/18/43/3e0a42c9-f10c-4225-bcd6-2b9b61ecc743)

#### `GetReinforcements(...)`

Returns an enumerator of all the connected reinforcements.

[Docs](https://developer.tekla.com/topic/en/18/43/510c913a-0653-6777-1841-c4f7df4fee47)

#### `GetReportProperty(String, Double.)(...)`

Retrieves a double property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/1c05aa56-5f42-be91-a59e-8d59a5549f45)

#### `GetReportProperty(String, Int32.)(...)`

Retrieves an integer property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/ce8e4ca0-3750-3c9b-5ca8-27e48b45e0bd)

#### `GetReportProperty(String, String.)(...)`

Retrieves a string property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/70f212bb-ee70-e9e9-7ab6-d6a9d46f8de6)

#### `GetSolid(FormingStates)(...)`

Returns the solid of the part.

[Docs](https://developer.tekla.com/topic/en/18/43/1e4d454b-d3a0-4219-b4e2-7f661b402c4d)

#### `GetSolid(Solid.SolidCreationTypeEnum)(...)`

Returns the solid of the part.

[Docs](https://developer.tekla.com/topic/en/18/43/25f58541-37fe-c1ca-f384-065ecbe38e55)

#### `GetSolid.(...)`

Returns the solid of the part.

[Docs](https://developer.tekla.com/topic/en/18/43/7bd70e97-7c8b-823c-c7e1-da2ebe3dadaf)

#### `GetStringReportProperties(...)`

Retrieves multiple string report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/5ff72da1-7382-f5d2-eb1d-c99b46239d42)

#### `GetStringUserProperties(...)`

Retrieves all string properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/67120e84-c820-9d52-12ae-c167c595842c)

#### `GetSurfaceObjects(...)`

Returns an enumerator of all the connected surface objects.

[Docs](https://developer.tekla.com/topic/en/18/43/36268c4e-9006-1c9d-57bd-331f907da6ce)

#### `GetSurfaceTreatments(...)`

Returns an enumerator of all the connected surface treatments.

[Docs](https://developer.tekla.com/topic/en/18/43/26c1c1a5-e5db-9850-f739-c58bc05c963e)

#### `GetUserProperty(String, Double.)(...)`

Retrieves a double property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/17cc8074-7955-32b5-2b4b-b169c45d4ee0)

#### `GetUserProperty(String, Int32.)(...)`

Retrieves an integer property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/d9364c1d-5b57-6a31-4c13-01124058cfb2)

#### `GetUserProperty(String, String.)(...)`

Retrieves a string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/214a886c-c18c-9b66-d7c9-fe84260962f2)

#### `GetWelds(...)`

Returns an enumerator of all the connected welds.

[Docs](https://developer.tekla.com/topic/en/18/43/722de753-413b-f810-f09c-07388feb0afa)

#### `Insert(...)`

Inserts the contour plate into the model database. All the attributes must be set.

[Docs](https://developer.tekla.com/topic/en/18/43/c02b023b-a8be-925c-f2bf-7f3cee59c03d)

#### `Modify(...)`

Modifies the existing contour plate in the model database to match the current one. The identifier must be set.

[Docs](https://developer.tekla.com/topic/en/18/43/15700480-e315-b734-7e0d-129e39990368)

#### `Select(...)`

Selects a contour plate from the model database. The identifier of the contour plate must be set.

[Docs](https://developer.tekla.com/topic/en/18/43/cc2b528a-c78d-8237-3171-8ae0c6650913)

#### `SetCustomObjectType(...)`

Sets a custom object type name for an existing object.

[Docs](https://developer.tekla.com/topic/en/18/43/3d6fb91b-48a9-35ca-d498-526514344089)

#### `SetDynamicStringProperty(...)`

Sets a dynamic string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/4dab0254-09e5-5e61-c28b-1c12afab71a0)

#### `SetLabel(...)`

Sets a label for an object when a new instance is created, this method must be called before Insert. The label is used in plug-ins for identifying the changed object in modification.

[Docs](https://developer.tekla.com/topic/en/18/43/3efcfc44-613a-e66b-9ebd-9f86df4f4036)

#### `SetPhase(...)`

Sets the phase of the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/ed80abcc-0ff7-d081-2229-e651f858a727)

#### `SetUserProperties(...)`

Sets multiple properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/8659da37-d571-f03f-c666-5c9fa3b113f5)

#### `SetUserProperty(String, Double)(...)`

Sets a double property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/5c585b8b-5c2e-7b75-2242-758f17429396)

#### `SetUserProperty(String, Int32)(...)`

Sets an integer property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/dd5b828d-3a87-bb58-5ac9-e1c3b4c2fe4b)

#### `SetUserProperty(String, String)(...)`

Sets a string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/80bf0348-b651-3d4a-862b-49c85bc924ab)

### Properties
- `AssemblyNumber` (object, get/set) — Gets or sets the assembly number. Defines the numbering in the assembly sense.
- `CastUnitType` (object, get/set) — Gets or sets the cast unit type of the part.
- `Class` (object, get/set) — Gets or sets the class of the part.
- `Contour` (object, get/set) — Gets or sets the contour for the contour plate.
- `DeformingData` (object, get/set) — Gets or sets the deforming data of the part.
- `Finish` (object, get/set) — Gets or sets the finish of the part.
- `Identifier` (object, get/set) — The identifier of the object.
- `IsUpToDate` (object, get/set) — Gets if the object does not have a modification which is not shared.
- `Material` (object, get/set) — Gets or sets the material the part is made of.
- `ModificationTime` (object, get/set) — Gets latest time of the object was modified or created.
- `Name` (object, get/set) — Gets or sets the name of the part.
- `PartNumber` (object, get/set) — Gets or sets the part number. Defines the numbering in the part sense.
- `Position` (object, get/set) — Gets or sets the part position. Defines the way the part is positioned in the model.
- `PourPhase` (object, get/set) — Gets or sets the pour phase of the part.
- `Profile` (object, get/set) — Gets or sets the profile of the part.
- `Type` (object, get/set) — Gets the read only type of the contour plate. In creation, the type is defined based on the material. If the material is concrete, the type will be slab, otherwise plate.

## ContourPlate.ContourPlateTypeEnum (enum)

The contour plate types.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/22f8144f-78df-e35b-1d18-c9999523d61b)

### Values
- `UNKNOWN` = `0` — The unknown type.
- `PLATE` = `1` — The plate-like contour plate.
- `SLAB` = `2` — The slab-like contour plate.

## ContourPoint (class)

The ContourPoint class defines a point with possible chamfering information, used, for example, to define a contour plate.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/1f158311-3d42-5ee3-4074-7cb13d653826)

### Constructors
- `ContourPoint(...)` — Creates a new contour point instance.
- `ContourPoint(...)` — Creates a new contour point using the given point and chamfer.

### Methods
#### `CompareTo(...)`

Compares two points. To use binarysearch somekind of sorting should be used.

[Docs](https://developer.tekla.com/topic/en/18/43/2f202828-05fd-220f-92b1-5b64372a4f7e)

#### `Equals(...)`

Returns true if the current object and the given object are equal.

[Docs](https://developer.tekla.com/topic/en/18/43/dbca17f2-6730-e2c4-583c-90c74c72e0b0)

#### `GetHashCode(...)`

Returns a hash code for the point. Notice, in extremely rare cases, you might not get the same hash code for two points even though they are considered equal! This should, however, happen only in extremely rare cases!

[Docs](https://developer.tekla.com/topic/en/18/43/9f84f024-170b-5181-770d-5ed2f56332cd)

#### `SetPoint(...)`

Sets the point's coordinates for the contour point.

[Docs](https://developer.tekla.com/topic/en/18/43/caa63223-3fa6-7f07-5e3f-1d8e7b7386d0)

#### `ToString(...)`

Formats the point into a string.

[Docs](https://developer.tekla.com/topic/en/18/43/63db29a6-32f7-6696-31eb-a6084dcb62cd)

#### `Translate(...)`

Translates the point using the given vector.

[Docs](https://developer.tekla.com/topic/en/18/43/7ad7ecb9-acf3-7ed2-f2bc-aa52b99ceae6)

#### `Zero(...)`

Zeros all the members of the point.

[Docs](https://developer.tekla.com/topic/en/18/43/9bf63e0c-fe71-40ae-e467-306f0a41f315)

### Properties
- `Chamfer` (object, get/set) — The chamfer for the contour point.

## ControlArc (class)

The ControlArc class defines a user defined arc helping in modeling work.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/5b51428a-00f9-155b-333c-83bd261a0b88)

### Constructors
- `ControlArc(...)` — Creates a new control arc instance. The default values are as follows: CenterPoint = Point(0.0, 0.0, 0.0);StartPoint = Point(0.0, 1000.0, 0.0);EndPoint = Point(1000.0, 0.0, 0.0);Color = ControlObjectColorEnum.YELLOW;
- `ControlArc(...)` — Creates a new control arc instance taking the 3 points.

### Methods
#### `CompareTo(...)`

Compares Identifiers of model objects.

[Docs](https://developer.tekla.com/topic/en/18/43/79cca356-0c8a-e8fb-463a-34ad4141bec7)

#### `Delete(...)`

Deletes the control arc with the given Identifier.

[Docs](https://developer.tekla.com/topic/en/18/43/c3d8544a-0173-f022-2522-94ac3c9a9d8c)

#### `Equals(...)`

Check if Identifiers of model objects are same.

[Docs](https://developer.tekla.com/topic/en/18/43/e4324265-2490-00de-4139-63a8f7acb492)

#### `GetAllReportProperties(...)`

Retrieves all properties for the model object in one hashtable. Type for the returned value must be checked using type casting.

[Docs](https://developer.tekla.com/topic/en/18/43/0708f404-ce77-28e1-4ebe-fa906f68bff8)

#### `GetAllUserProperties(...)`

Retrieves all properties for the model object in one hashtable. Type for the returned value must be checked using type casting.

[Docs](https://developer.tekla.com/topic/en/18/43/63b853a0-0af5-af9c-3ce7-05299664d7f8)

#### `GetChildren(...)`

Returns an enumerator of all the children model objects.

[Docs](https://developer.tekla.com/topic/en/18/43/ce51ec40-310c-6067-a89f-f8d029aab747)

#### `GetCoordinateSystem(...)`

Returns the coordinate system for the given model object.

[Docs](https://developer.tekla.com/topic/en/18/43/8adfe9cd-f6e5-6474-1297-33c6270a7b0a)

#### `GetCustomObjectType(...)`

Gets custom object type name from an existing object.

[Docs](https://developer.tekla.com/topic/en/18/43/d3be2ba3-8533-7165-f2d3-a5aa9df556b8)

#### `GetDoubleReportProperties(...)`

Retrieves multiple double report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/f86f6c96-f738-49ca-16ba-f5634e862ba2)

#### `GetDoubleUserProperties(...)`

Retrieves all double properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/6ead7a63-5f76-0631-d480-d46c27e882aa)

#### `GetDynamicStringProperty(...)`

Gets a dynamic string property from the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/415e86d9-7de6-ea53-ce68-20b3e67b8655)

#### `GetFatherComponent(...)`

Returns the father component of the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/a2c28f3f-52ff-50a8-1b32-10d18db5c31d)

#### `GetHierarchicObjects(...)`

Returns an enumerator of all the connected hierarchic objects.

[Docs](https://developer.tekla.com/topic/en/18/43/cc228ea0-267b-7d90-7441-c9efc1779b08)

#### `GetIntegerReportProperties(...)`

Retrieves multiple integer report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/2623a9c0-2a9c-e233-9daa-59f92ed51445)

#### `GetIntegerUserProperties(...)`

Retrieves all integer properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/1b3c6f06-bc25-a54e-2a88-b60dc4a3c85e)

#### `GetPhase(...)`

Retrieves the phase of the model object (the phase number, the phase name, the phase comment and whether the phase is the current one or not).

[Docs](https://developer.tekla.com/topic/en/18/43/310f8b7a-c120-753a-35c9-f7c73a9806c9)

#### `GetReportProperty(String, Double.)(...)`

Retrieves a double property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/1c05aa56-5f42-be91-a59e-8d59a5549f45)

#### `GetReportProperty(String, Int32.)(...)`

Retrieves an integer property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/ce8e4ca0-3750-3c9b-5ca8-27e48b45e0bd)

#### `GetReportProperty(String, String.)(...)`

Retrieves a string property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/70f212bb-ee70-e9e9-7ab6-d6a9d46f8de6)

#### `GetStringReportProperties(...)`

Retrieves multiple string report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/5ff72da1-7382-f5d2-eb1d-c99b46239d42)

#### `GetStringUserProperties(...)`

Retrieves all string properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/67120e84-c820-9d52-12ae-c167c595842c)

#### `GetUserProperty(String, Double.)(...)`

Retrieves a double property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/17cc8074-7955-32b5-2b4b-b169c45d4ee0)

#### `GetUserProperty(String, Int32.)(...)`

Retrieves an integer property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/d9364c1d-5b57-6a31-4c13-01124058cfb2)

#### `GetUserProperty(String, String.)(...)`

Retrieves a string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/214a886c-c18c-9b66-d7c9-fe84260962f2)

#### `Insert(...)`

Inserts the control arc into the model.

[Docs](https://developer.tekla.com/topic/en/18/43/ba029045-86bb-fd04-2632-bed8f123ec3b)

#### `Modify(...)`

Modifies the control arc with the given Identifier.

[Docs](https://developer.tekla.com/topic/en/18/43/23a76778-3c4b-66af-8edf-0889dc461873)

#### `Select(...)`

Selects the control arc with the given Identifier.

[Docs](https://developer.tekla.com/topic/en/18/43/8d69f80d-ae14-380a-5250-06043cfc68b2)

#### `SetCustomObjectType(...)`

Sets a custom object type name for an existing object.

[Docs](https://developer.tekla.com/topic/en/18/43/3d6fb91b-48a9-35ca-d498-526514344089)

#### `SetDynamicStringProperty(...)`

Sets a dynamic string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/4dab0254-09e5-5e61-c28b-1c12afab71a0)

#### `SetLabel(...)`

Sets a label for an object when a new instance is created, this method must be called before Insert. The label is used in plug-ins for identifying the changed object in modification.

[Docs](https://developer.tekla.com/topic/en/18/43/3efcfc44-613a-e66b-9ebd-9f86df4f4036)

#### `SetPhase(...)`

Sets the phase of the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/ed80abcc-0ff7-d081-2229-e651f858a727)

#### `SetUserProperties(...)`

Sets multiple properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/8659da37-d571-f03f-c666-5c9fa3b113f5)

#### `SetUserProperty(String, Double)(...)`

Sets a double property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/5c585b8b-5c2e-7b75-2242-758f17429396)

#### `SetUserProperty(String, Int32)(...)`

Sets an integer property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/dd5b828d-3a87-bb58-5ac9-e1c3b4c2fe4b)

#### `SetUserProperty(String, String)(...)`

Sets a string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/80bf0348-b651-3d4a-862b-49c85bc924ab)

### Properties
- `Color` (object, get/set) — Gets or sets color of the control arc.
- `Geometry` (object, get/set) — Gets or sets the arc geometry. The arc must not represent a complete circle (i.e. the arc angle must be less than 360 degrees).
- `Identifier` (object, get/set) — The identifier of the object.
- `IsUpToDate` (object, get/set) — Gets if the object does not have a modification which is not shared.
- `LineType` (object, get/set) — The line type of the control arc.
- `ModificationTime` (object, get/set) — Gets latest time of the object was modified or created.

## ControlCircle (class)

The ControlCircle class defines a user defined (not magnetic) circle helping in modeling work. If there are duplicated input points or 3 input points are in a line, ControlCircle.Insert() will return false. ControlCircle.Select() will return the center point of the circle to Point1.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/9763b8ff-1f52-2d69-7549-a6d8733e945c)

### Constructors
- `ControlCircle(...)` — Initializes a new instance of the ControlCircle class. The default values are as follows: Color = ControlCircleColorEnum.YELLOW; Extension = 0.0;
- `ControlCircle(...)` — Initializes a new instance of the ControlCircle class. The control circle is not magnetic.

### Methods
#### `CompareTo(...)`

Compares Identifiers of model objects.

[Docs](https://developer.tekla.com/topic/en/18/43/79cca356-0c8a-e8fb-463a-34ad4141bec7)

#### `Delete(...)`

Deletes the control circle with the given Identifier.

[Docs](https://developer.tekla.com/topic/en/18/43/338ebade-5719-4097-0f39-04125037b1fb)

#### `Equals(...)`

Check if Identifiers of model objects are same.

[Docs](https://developer.tekla.com/topic/en/18/43/e4324265-2490-00de-4139-63a8f7acb492)

#### `GetAllReportProperties(...)`

Retrieves all properties for the model object in one hashtable. Type for the returned value must be checked using type casting.

[Docs](https://developer.tekla.com/topic/en/18/43/0708f404-ce77-28e1-4ebe-fa906f68bff8)

#### `GetAllUserProperties(...)`

Retrieves all properties for the model object in one hashtable. Type for the returned value must be checked using type casting.

[Docs](https://developer.tekla.com/topic/en/18/43/63b853a0-0af5-af9c-3ce7-05299664d7f8)

#### `GetChildren(...)`

Returns an enumerator of all the children model objects.

[Docs](https://developer.tekla.com/topic/en/18/43/ce51ec40-310c-6067-a89f-f8d029aab747)

#### `GetCoordinateSystem(...)`

Returns the coordinate system for the given model object.

[Docs](https://developer.tekla.com/topic/en/18/43/8adfe9cd-f6e5-6474-1297-33c6270a7b0a)

#### `GetCustomObjectType(...)`

Gets custom object type name from an existing object.

[Docs](https://developer.tekla.com/topic/en/18/43/d3be2ba3-8533-7165-f2d3-a5aa9df556b8)

#### `GetDoubleReportProperties(...)`

Retrieves multiple double report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/f86f6c96-f738-49ca-16ba-f5634e862ba2)

#### `GetDoubleUserProperties(...)`

Retrieves all double properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/6ead7a63-5f76-0631-d480-d46c27e882aa)

#### `GetDynamicStringProperty(...)`

Gets a dynamic string property from the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/415e86d9-7de6-ea53-ce68-20b3e67b8655)

#### `GetFatherComponent(...)`

Returns the father component of the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/a2c28f3f-52ff-50a8-1b32-10d18db5c31d)

#### `GetHierarchicObjects(...)`

Returns an enumerator of all the connected hierarchic objects.

[Docs](https://developer.tekla.com/topic/en/18/43/cc228ea0-267b-7d90-7441-c9efc1779b08)

#### `GetIntegerReportProperties(...)`

Retrieves multiple integer report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/2623a9c0-2a9c-e233-9daa-59f92ed51445)

#### `GetIntegerUserProperties(...)`

Retrieves all integer properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/1b3c6f06-bc25-a54e-2a88-b60dc4a3c85e)

#### `GetPhase(...)`

Retrieves the phase of the model object (the phase number, the phase name, the phase comment and whether the phase is the current one or not).

[Docs](https://developer.tekla.com/topic/en/18/43/310f8b7a-c120-753a-35c9-f7c73a9806c9)

#### `GetReportProperty(String, Double.)(...)`

Retrieves a double property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/1c05aa56-5f42-be91-a59e-8d59a5549f45)

#### `GetReportProperty(String, Int32.)(...)`

Retrieves an integer property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/ce8e4ca0-3750-3c9b-5ca8-27e48b45e0bd)

#### `GetReportProperty(String, String.)(...)`

Retrieves a string property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/70f212bb-ee70-e9e9-7ab6-d6a9d46f8de6)

#### `GetStringReportProperties(...)`

Retrieves multiple string report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/5ff72da1-7382-f5d2-eb1d-c99b46239d42)

#### `GetStringUserProperties(...)`

Retrieves all string properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/67120e84-c820-9d52-12ae-c167c595842c)

#### `GetUserProperty(String, Double.)(...)`

Retrieves a double property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/17cc8074-7955-32b5-2b4b-b169c45d4ee0)

#### `GetUserProperty(String, Int32.)(...)`

Retrieves an integer property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/d9364c1d-5b57-6a31-4c13-01124058cfb2)

#### `GetUserProperty(String, String.)(...)`

Retrieves a string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/214a886c-c18c-9b66-d7c9-fe84260962f2)

#### `Insert(...)`

Inserts the control circle into the model.

[Docs](https://developer.tekla.com/topic/en/18/43/05ddc648-b70b-21f1-d905-9e3e0cac2908)

#### `Modify(...)`

Modifies the control circle with the given Identifier.

[Docs](https://developer.tekla.com/topic/en/18/43/2d5f9007-bc4e-e954-f2bf-61d01e4af8a8)

#### `Select(...)`

Selects the control circle with the given Identifier.

[Docs](https://developer.tekla.com/topic/en/18/43/3c136bef-e668-4372-784a-b6169df29570)

#### `SetCustomObjectType(...)`

Sets a custom object type name for an existing object.

[Docs](https://developer.tekla.com/topic/en/18/43/3d6fb91b-48a9-35ca-d498-526514344089)

#### `SetDynamicStringProperty(...)`

Sets a dynamic string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/4dab0254-09e5-5e61-c28b-1c12afab71a0)

#### `SetLabel(...)`

Sets a label for an object when a new instance is created, this method must be called before Insert. The label is used in plug-ins for identifying the changed object in modification.

[Docs](https://developer.tekla.com/topic/en/18/43/3efcfc44-613a-e66b-9ebd-9f86df4f4036)

#### `SetPhase(...)`

Sets the phase of the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/ed80abcc-0ff7-d081-2229-e651f858a727)

#### `SetUserProperties(...)`

Sets multiple properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/8659da37-d571-f03f-c666-5c9fa3b113f5)

#### `SetUserProperty(String, Double)(...)`

Sets a double property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/5c585b8b-5c2e-7b75-2242-758f17429396)

#### `SetUserProperty(String, Int32)(...)`

Sets an integer property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/dd5b828d-3a87-bb58-5ac9-e1c3b4c2fe4b)

#### `SetUserProperty(String, String)(...)`

Sets a string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/80bf0348-b651-3d4a-862b-49c85bc924ab)

### Properties
- `Color` (object, get/set) — Gets or sets the color of the control circle.
- `Extension` (object, get/set) — Gets or sets the extension of the control circle.
- `Identifier` (object, get/set) — The identifier of the object.
- `IsUpToDate` (object, get/set) — Gets if the object does not have a modification which is not shared.
- `LineType` (object, get/set) — The line type of the control circle.
- `ModificationTime` (object, get/set) — Gets latest time of the object was modified or created.
- `Point1` (object, get/set) — Gets or sets the first point. Point1 becomes the center point of the circle after selecting the control circle.
- `Point2` (object, get/set) — Gets or sets the second point. Point2 contains the original input Point1 after selecting the control circle.
- `Point3` (object, get/set) — Gets or sets the third point. Point3 contains the calculated result from the orthogonal point and the center point after selecting the control circle.

## ControlCircle.ControlCircleColorEnum (enum)

The different circle colors.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/20466415-4abb-34c8-b445-f7a9e03810da)

### Values
- `YELLOW_RED` = `-1` — The yellow/red color.
- `BLACK` = `0` — The black color.
- `WHITE` = `1` — The white color.
- `RED` = `2` — The red color.
- `GREEN` = `3` — The green color.
- `BLUE` = `4` — The blue color.
- `CYAN` = `5` — The cyan color.
- `YELLOW` = `6` — The yellow color.
- `MAGENTA` = `7` — The magenta color.

## ControlLine (class)

The ControlLine class defines a user defined (possibly magnetic) line helping in modeling work.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/8f08f0af-9bc7-d718-4bba-cd6f23c4b0e1)

### Constructors
- `ControlLine(...)` — Creates a new control line instance. The default values are as follows: Line = new LineSegment();IsMagnetic = false;Color = ControlLineColorEnum.YELLOW;Extension = 0.0;
- `ControlLine(...)` — Creates a new control line instance taking the line segment and magnetism as input.

### Methods
#### `CompareTo(...)`

Compares Identifiers of model objects.

[Docs](https://developer.tekla.com/topic/en/18/43/79cca356-0c8a-e8fb-463a-34ad4141bec7)

#### `Delete(...)`

Deletes the control line with the given Identifier.

[Docs](https://developer.tekla.com/topic/en/18/43/77a8b233-a564-bd1c-5755-38863bc77006)

#### `Equals(...)`

Check if Identifiers of model objects are same.

[Docs](https://developer.tekla.com/topic/en/18/43/e4324265-2490-00de-4139-63a8f7acb492)

#### `GetAllReportProperties(...)`

Retrieves all properties for the model object in one hashtable. Type for the returned value must be checked using type casting.

[Docs](https://developer.tekla.com/topic/en/18/43/0708f404-ce77-28e1-4ebe-fa906f68bff8)

#### `GetAllUserProperties(...)`

Retrieves all properties for the model object in one hashtable. Type for the returned value must be checked using type casting.

[Docs](https://developer.tekla.com/topic/en/18/43/63b853a0-0af5-af9c-3ce7-05299664d7f8)

#### `GetChildren(...)`

Returns an enumerator of all the children model objects.

[Docs](https://developer.tekla.com/topic/en/18/43/ce51ec40-310c-6067-a89f-f8d029aab747)

#### `GetCoordinateSystem(...)`

Returns the coordinate system for the given model object.

[Docs](https://developer.tekla.com/topic/en/18/43/8adfe9cd-f6e5-6474-1297-33c6270a7b0a)

#### `GetCustomObjectType(...)`

Gets custom object type name from an existing object.

[Docs](https://developer.tekla.com/topic/en/18/43/d3be2ba3-8533-7165-f2d3-a5aa9df556b8)

#### `GetDoubleReportProperties(...)`

Retrieves multiple double report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/f86f6c96-f738-49ca-16ba-f5634e862ba2)

#### `GetDoubleUserProperties(...)`

Retrieves all double properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/6ead7a63-5f76-0631-d480-d46c27e882aa)

#### `GetDynamicStringProperty(...)`

Gets a dynamic string property from the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/415e86d9-7de6-ea53-ce68-20b3e67b8655)

#### `GetFatherComponent(...)`

Returns the father component of the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/a2c28f3f-52ff-50a8-1b32-10d18db5c31d)

#### `GetHierarchicObjects(...)`

Returns an enumerator of all the connected hierarchic objects.

[Docs](https://developer.tekla.com/topic/en/18/43/cc228ea0-267b-7d90-7441-c9efc1779b08)

#### `GetIntegerReportProperties(...)`

Retrieves multiple integer report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/2623a9c0-2a9c-e233-9daa-59f92ed51445)

#### `GetIntegerUserProperties(...)`

Retrieves all integer properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/1b3c6f06-bc25-a54e-2a88-b60dc4a3c85e)

#### `GetPhase(...)`

Retrieves the phase of the model object (the phase number, the phase name, the phase comment and whether the phase is the current one or not).

[Docs](https://developer.tekla.com/topic/en/18/43/310f8b7a-c120-753a-35c9-f7c73a9806c9)

#### `GetReportProperty(String, Double.)(...)`

Retrieves a double property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/1c05aa56-5f42-be91-a59e-8d59a5549f45)

#### `GetReportProperty(String, Int32.)(...)`

Retrieves an integer property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/ce8e4ca0-3750-3c9b-5ca8-27e48b45e0bd)

#### `GetReportProperty(String, String.)(...)`

Retrieves a string property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/70f212bb-ee70-e9e9-7ab6-d6a9d46f8de6)

#### `GetStringReportProperties(...)`

Retrieves multiple string report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/5ff72da1-7382-f5d2-eb1d-c99b46239d42)

#### `GetStringUserProperties(...)`

Retrieves all string properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/67120e84-c820-9d52-12ae-c167c595842c)

#### `GetUserProperty(String, Double.)(...)`

Retrieves a double property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/17cc8074-7955-32b5-2b4b-b169c45d4ee0)

#### `GetUserProperty(String, Int32.)(...)`

Retrieves an integer property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/d9364c1d-5b57-6a31-4c13-01124058cfb2)

#### `GetUserProperty(String, String.)(...)`

Retrieves a string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/214a886c-c18c-9b66-d7c9-fe84260962f2)

#### `Insert(...)`

Inserts the control line into the model.

[Docs](https://developer.tekla.com/topic/en/18/43/79eef866-f844-f120-e9a3-e0db3c754f30)

#### `Modify(...)`

Modifies the control line with the given Identifier.

[Docs](https://developer.tekla.com/topic/en/18/43/27068a1e-eb21-91ba-8655-9e59b5fca7ad)

#### `Select(...)`

Selects the control line with the given Identifier.

[Docs](https://developer.tekla.com/topic/en/18/43/79acb4f3-e6a0-1728-40f3-12073ff62a5d)

#### `SetCustomObjectType(...)`

Sets a custom object type name for an existing object.

[Docs](https://developer.tekla.com/topic/en/18/43/3d6fb91b-48a9-35ca-d498-526514344089)

#### `SetDynamicStringProperty(...)`

Sets a dynamic string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/4dab0254-09e5-5e61-c28b-1c12afab71a0)

#### `SetLabel(...)`

Sets a label for an object when a new instance is created, this method must be called before Insert. The label is used in plug-ins for identifying the changed object in modification.

[Docs](https://developer.tekla.com/topic/en/18/43/3efcfc44-613a-e66b-9ebd-9f86df4f4036)

#### `SetPhase(...)`

Sets the phase of the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/ed80abcc-0ff7-d081-2229-e651f858a727)

#### `SetUserProperties(...)`

Sets multiple properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/8659da37-d571-f03f-c666-5c9fa3b113f5)

#### `SetUserProperty(String, Double)(...)`

Sets a double property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/5c585b8b-5c2e-7b75-2242-758f17429396)

#### `SetUserProperty(String, Int32)(...)`

Sets an integer property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/dd5b828d-3a87-bb58-5ac9-e1c3b4c2fe4b)

#### `SetUserProperty(String, String)(...)`

Sets a string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/80bf0348-b651-3d4a-862b-49c85bc924ab)

### Properties
- `Color` (object, get/set) — The color of the control line.
- `Extension` (object, get/set) — The extension of the control line.
- `Identifier` (object, get/set) — The identifier of the object.
- `IsMagnetic` (object, get/set) — Determines whether the line is magnetic or not.
- `IsUpToDate` (object, get/set) — Gets if the object does not have a modification which is not shared.
- `Line` (object, get/set) — The line segment defining the position of the control line.
- `LineType` (object, get/set) — The line type of the control line.
- `ModificationTime` (object, get/set) — Gets latest time of the object was modified or created.

## ControlLine.ControlLineColorEnum (enum)

The different line colors.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/5be6efb4-cfe5-6f0b-1ebc-be8609d0bc05)

### Values
- `YELLOW_RED` = `-1` — The yellow/red color.
- `BLACK` = `0` — The black color.
- `WHITE` = `1` — The white color.
- `RED` = `2` — The red color.
- `GREEN` = `3` — The green color.
- `BLUE` = `4` — The blue color.
- `CYAN` = `5` — The cyan color.
- `YELLOW` = `6` — The yellow color.
- `MAGENTA` = `7` — The magenta color.

## ControlObjectColorEnum (enum)

The different colors used for control objects (ControlLine and ControlCircle have their own).

[Vendor docs](https://developer.tekla.com/topic/en/18/43/de19d580-60b4-961b-dfb1-107560ed1ed2)

### Values
- `YELLOW_RED` = `-1` — The yellow/red color.
- `BLACK` = `0` — The black color.
- `WHITE` = `1` — The white color.
- `RED` = `2` — The red color.
- `GREEN` = `3` — The green color.
- `BLUE` = `4` — The blue color.
- `CYAN` = `5` — The cyan color.
- `YELLOW` = `6` — The yellow color.
- `MAGENTA` = `7` — The magenta color.

## ControlObjectLineType (enum)

The line types (the presentation of the line).

[Vendor docs](https://developer.tekla.com/topic/en/18/43/1e738aca-83ee-237a-9285-eceb80bc6c51)

### Values
- `SolidLine` = `1` — The solid line (default).
- `DashedLine` = `2` — The dashed line (-----).
- `SlashedLine` = `3` — The slashed line (- - -).
- `DashDot` = `4` — The dash dotted line (--.--.).
- `DottedLine` = `5` — The dotted line (.....).

## ControlPlane (class)

The ControlPlane class defines a user defined (possibly magnetic) plane helping in modeling work.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/561ea178-df9a-d196-a846-e3223a8b202f)

### Constructors
- `ControlPlane(...)` — Creates a new control plane instance. The default values are as follows: Plane = new Plane();IsMagnetic = false;Name = "Plane";
- `ControlPlane(...)` — Creates a new control plane instance taking the plane and magnetism as input.

### Methods
#### `CompareTo(...)`

Compares Identifiers of model objects.

[Docs](https://developer.tekla.com/topic/en/18/43/79cca356-0c8a-e8fb-463a-34ad4141bec7)

#### `Delete(...)`

Deletes the control plane with the given ID.

[Docs](https://developer.tekla.com/topic/en/18/43/0ba8433f-1e70-f117-c88a-224a4413c080)

#### `Equals(...)`

Check if Identifiers of model objects are same.

[Docs](https://developer.tekla.com/topic/en/18/43/e4324265-2490-00de-4139-63a8f7acb492)

#### `GetAllReportProperties(...)`

Retrieves all properties for the model object in one hashtable. Type for the returned value must be checked using type casting.

[Docs](https://developer.tekla.com/topic/en/18/43/0708f404-ce77-28e1-4ebe-fa906f68bff8)

#### `GetAllUserProperties(...)`

Retrieves all properties for the model object in one hashtable. Type for the returned value must be checked using type casting.

[Docs](https://developer.tekla.com/topic/en/18/43/63b853a0-0af5-af9c-3ce7-05299664d7f8)

#### `GetChildren(...)`

Returns an enumerator of all the children model objects.

[Docs](https://developer.tekla.com/topic/en/18/43/ce51ec40-310c-6067-a89f-f8d029aab747)

#### `GetCoordinateSystem(...)`

Returns the coordinate system for the given model object.

[Docs](https://developer.tekla.com/topic/en/18/43/8adfe9cd-f6e5-6474-1297-33c6270a7b0a)

#### `GetCustomObjectType(...)`

Gets custom object type name from an existing object.

[Docs](https://developer.tekla.com/topic/en/18/43/d3be2ba3-8533-7165-f2d3-a5aa9df556b8)

#### `GetDoubleReportProperties(...)`

Retrieves multiple double report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/f86f6c96-f738-49ca-16ba-f5634e862ba2)

#### `GetDoubleUserProperties(...)`

Retrieves all double properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/6ead7a63-5f76-0631-d480-d46c27e882aa)

#### `GetDynamicStringProperty(...)`

Gets a dynamic string property from the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/415e86d9-7de6-ea53-ce68-20b3e67b8655)

#### `GetFatherComponent(...)`

Returns the father component of the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/a2c28f3f-52ff-50a8-1b32-10d18db5c31d)

#### `GetHierarchicObjects(...)`

Returns an enumerator of all the connected hierarchic objects.

[Docs](https://developer.tekla.com/topic/en/18/43/cc228ea0-267b-7d90-7441-c9efc1779b08)

#### `GetIntegerReportProperties(...)`

Retrieves multiple integer report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/2623a9c0-2a9c-e233-9daa-59f92ed51445)

#### `GetIntegerUserProperties(...)`

Retrieves all integer properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/1b3c6f06-bc25-a54e-2a88-b60dc4a3c85e)

#### `GetPhase(...)`

Retrieves the phase of the model object (the phase number, the phase name, the phase comment and whether the phase is the current one or not).

[Docs](https://developer.tekla.com/topic/en/18/43/310f8b7a-c120-753a-35c9-f7c73a9806c9)

#### `GetReportProperty(String, Double.)(...)`

Retrieves a double property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/1c05aa56-5f42-be91-a59e-8d59a5549f45)

#### `GetReportProperty(String, Int32.)(...)`

Retrieves an integer property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/ce8e4ca0-3750-3c9b-5ca8-27e48b45e0bd)

#### `GetReportProperty(String, String.)(...)`

Retrieves a string property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/70f212bb-ee70-e9e9-7ab6-d6a9d46f8de6)

#### `GetStringReportProperties(...)`

Retrieves multiple string report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/5ff72da1-7382-f5d2-eb1d-c99b46239d42)

#### `GetStringUserProperties(...)`

Retrieves all string properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/67120e84-c820-9d52-12ae-c167c595842c)

#### `GetUserProperty(String, Double.)(...)`

Retrieves a double property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/17cc8074-7955-32b5-2b4b-b169c45d4ee0)

#### `GetUserProperty(String, Int32.)(...)`

Retrieves an integer property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/d9364c1d-5b57-6a31-4c13-01124058cfb2)

#### `GetUserProperty(String, String.)(...)`

Retrieves a string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/214a886c-c18c-9b66-d7c9-fe84260962f2)

#### `Insert(...)`

Inserts the control plane into the model.

[Docs](https://developer.tekla.com/topic/en/18/43/aed12852-a0c6-a0f2-be15-dd1088f3c56d)

#### `Modify(...)`

Modifies the control plane with the given ID.

[Docs](https://developer.tekla.com/topic/en/18/43/0d9e2b86-6f86-38ff-7b74-db12308004cd)

#### `Select(...)`

Selects the control plane with the given ID.

[Docs](https://developer.tekla.com/topic/en/18/43/8d3f44b4-7bae-0b00-dffe-019fe217ab19)

#### `SetCustomObjectType(...)`

Sets a custom object type name for an existing object.

[Docs](https://developer.tekla.com/topic/en/18/43/3d6fb91b-48a9-35ca-d498-526514344089)

#### `SetDynamicStringProperty(...)`

Sets a dynamic string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/4dab0254-09e5-5e61-c28b-1c12afab71a0)

#### `SetLabel(...)`

Sets a label for an object when a new instance is created, this method must be called before Insert. The label is used in plug-ins for identifying the changed object in modification.

[Docs](https://developer.tekla.com/topic/en/18/43/3efcfc44-613a-e66b-9ebd-9f86df4f4036)

#### `SetPhase(...)`

Sets the phase of the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/ed80abcc-0ff7-d081-2229-e651f858a727)

#### `SetUserProperties(...)`

Sets multiple properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/8659da37-d571-f03f-c666-5c9fa3b113f5)

#### `SetUserProperty(String, Double)(...)`

Sets a double property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/5c585b8b-5c2e-7b75-2242-758f17429396)

#### `SetUserProperty(String, Int32)(...)`

Sets an integer property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/dd5b828d-3a87-bb58-5ac9-e1c3b4c2fe4b)

#### `SetUserProperty(String, String)(...)`

Sets a string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/80bf0348-b651-3d4a-862b-49c85bc924ab)

### Properties
- `Identifier` (object, get/set) — The identifier of the object.
- `IsMagnetic` (object, get/set) — Determines whether the plane is magnetic or not.
- `IsUpToDate` (object, get/set) — Gets if the object does not have a modification which is not shared.
- `ModificationTime` (object, get/set) — Gets latest time of the object was modified or created.
- `Name` (object, get/set) — The name of the control plane.
- `Plane` (object, get/set) — The plane defining the position of the control plane.

## ControlPoint (class)

The ControlPoint class defines a user defined point helping in modeling work.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/38b3f184-d532-5d3e-d59e-2d16a7e0f2d9)

### Constructors
- `ControlPoint(...)` — Initializes a new instance of the ControlPoint class with zero members.
- `ControlPoint(...)` — Initializes a new instance of the ControlPoint class with an existing point.

### Methods
#### `CompareTo(...)`

Compares Identifiers of model objects.

[Docs](https://developer.tekla.com/topic/en/18/43/79cca356-0c8a-e8fb-463a-34ad4141bec7)

#### `Delete(...)`

Deletes the control point with the given Identifier.

[Docs](https://developer.tekla.com/topic/en/18/43/61c44644-1092-b21f-c29b-842107382419)

#### `Equals(...)`

Check if Identifiers of model objects are same.

[Docs](https://developer.tekla.com/topic/en/18/43/e4324265-2490-00de-4139-63a8f7acb492)

#### `GetAllReportProperties(...)`

Retrieves all properties for the model object in one hashtable. Type for the returned value must be checked using type casting.

[Docs](https://developer.tekla.com/topic/en/18/43/0708f404-ce77-28e1-4ebe-fa906f68bff8)

#### `GetAllUserProperties(...)`

Retrieves all properties for the model object in one hashtable. Type for the returned value must be checked using type casting.

[Docs](https://developer.tekla.com/topic/en/18/43/63b853a0-0af5-af9c-3ce7-05299664d7f8)

#### `GetChildren(...)`

Returns an enumerator of all the children model objects.

[Docs](https://developer.tekla.com/topic/en/18/43/ce51ec40-310c-6067-a89f-f8d029aab747)

#### `GetCoordinateSystem(...)`

Returns the coordinate system for the given model object.

[Docs](https://developer.tekla.com/topic/en/18/43/8adfe9cd-f6e5-6474-1297-33c6270a7b0a)

#### `GetCustomObjectType(...)`

Gets custom object type name from an existing object.

[Docs](https://developer.tekla.com/topic/en/18/43/d3be2ba3-8533-7165-f2d3-a5aa9df556b8)

#### `GetDoubleReportProperties(...)`

Retrieves multiple double report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/f86f6c96-f738-49ca-16ba-f5634e862ba2)

#### `GetDoubleUserProperties(...)`

Retrieves all double properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/6ead7a63-5f76-0631-d480-d46c27e882aa)

#### `GetDynamicStringProperty(...)`

Gets a dynamic string property from the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/415e86d9-7de6-ea53-ce68-20b3e67b8655)

#### `GetFatherComponent(...)`

Returns the father component of the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/a2c28f3f-52ff-50a8-1b32-10d18db5c31d)

#### `GetHierarchicObjects(...)`

Returns an enumerator of all the connected hierarchic objects.

[Docs](https://developer.tekla.com/topic/en/18/43/cc228ea0-267b-7d90-7441-c9efc1779b08)

#### `GetIntegerReportProperties(...)`

Retrieves multiple integer report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/2623a9c0-2a9c-e233-9daa-59f92ed51445)

#### `GetIntegerUserProperties(...)`

Retrieves all integer properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/1b3c6f06-bc25-a54e-2a88-b60dc4a3c85e)

#### `GetPhase(...)`

Retrieves the phase of the model object (the phase number, the phase name, the phase comment and whether the phase is the current one or not).

[Docs](https://developer.tekla.com/topic/en/18/43/310f8b7a-c120-753a-35c9-f7c73a9806c9)

#### `GetReportProperty(String, Double.)(...)`

Retrieves a double property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/1c05aa56-5f42-be91-a59e-8d59a5549f45)

#### `GetReportProperty(String, Int32.)(...)`

Retrieves an integer property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/ce8e4ca0-3750-3c9b-5ca8-27e48b45e0bd)

#### `GetReportProperty(String, String.)(...)`

Retrieves a string property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/70f212bb-ee70-e9e9-7ab6-d6a9d46f8de6)

#### `GetStringReportProperties(...)`

Retrieves multiple string report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/5ff72da1-7382-f5d2-eb1d-c99b46239d42)

#### `GetStringUserProperties(...)`

Retrieves all string properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/67120e84-c820-9d52-12ae-c167c595842c)

#### `GetUserProperty(String, Double.)(...)`

Retrieves a double property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/17cc8074-7955-32b5-2b4b-b169c45d4ee0)

#### `GetUserProperty(String, Int32.)(...)`

Retrieves an integer property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/d9364c1d-5b57-6a31-4c13-01124058cfb2)

#### `GetUserProperty(String, String.)(...)`

Retrieves a string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/214a886c-c18c-9b66-d7c9-fe84260962f2)

#### `Insert(...)`

Inserts the control point into the model.

[Docs](https://developer.tekla.com/topic/en/18/43/233ceb2f-d19d-12d6-a1a1-9c211ea23887)

#### `Modify(...)`

Modifies the control point with the given Identifier.

[Docs](https://developer.tekla.com/topic/en/18/43/bacabcef-ffdf-0fed-53ac-a8b78db8053e)

#### `Select(...)`

Selects the control point with the given Identifier.

[Docs](https://developer.tekla.com/topic/en/18/43/16c6d05d-8e3c-3340-f0ec-93a3139db4db)

#### `SetCustomObjectType(...)`

Sets a custom object type name for an existing object.

[Docs](https://developer.tekla.com/topic/en/18/43/3d6fb91b-48a9-35ca-d498-526514344089)

#### `SetDynamicStringProperty(...)`

Sets a dynamic string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/4dab0254-09e5-5e61-c28b-1c12afab71a0)

#### `SetLabel(...)`

Sets a label for an object when a new instance is created, this method must be called before Insert. The label is used in plug-ins for identifying the changed object in modification.

[Docs](https://developer.tekla.com/topic/en/18/43/3efcfc44-613a-e66b-9ebd-9f86df4f4036)

#### `SetPhase(...)`

Sets the phase of the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/ed80abcc-0ff7-d081-2229-e651f858a727)

#### `SetUserProperties(...)`

Sets multiple properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/8659da37-d571-f03f-c666-5c9fa3b113f5)

#### `SetUserProperty(String, Double)(...)`

Sets a double property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/5c585b8b-5c2e-7b75-2242-758f17429396)

#### `SetUserProperty(String, Int32)(...)`

Sets an integer property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/dd5b828d-3a87-bb58-5ac9-e1c3b4c2fe4b)

#### `SetUserProperty(String, String)(...)`

Sets a string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/80bf0348-b651-3d4a-862b-49c85bc924ab)

### Properties
- `Identifier` (object, get/set) — The identifier of the object.
- `IsUpToDate` (object, get/set) — Gets if the object does not have a modification which is not shared.
- `ModificationTime` (object, get/set) — Gets latest time of the object was modified or created.
- `Point` (object, get/set) — Gets or sets the position of the control point.

## ControlPolycurve (class)

The ControlPolycurve class defines a user defined polycurve object which contains a chain of line and arc geometries.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/cd91f00f-1681-cfc5-3616-ff8e229a29bc)

### Constructors
- `ControlPolycurve(...)` — Initializes a new instance of the ControlPolycurve class. The default values are as follows: Color = ControlObjectColorEnum.BLUE;LineType = LineType.DashDot;
- `ControlPolycurve(...)` — Initializes a new instance of the ControlPolycurve class.

### Methods
#### `CompareTo(...)`

Compares Identifiers of model objects.

[Docs](https://developer.tekla.com/topic/en/18/43/79cca356-0c8a-e8fb-463a-34ad4141bec7)

#### `Delete(...)`

Deletes a polycurve with given Identifier.

[Docs](https://developer.tekla.com/topic/en/18/43/25be89ff-124b-f9c5-92af-04d9cef33f37)

#### `Equals(...)`

Check if Identifiers of model objects are same.

[Docs](https://developer.tekla.com/topic/en/18/43/e4324265-2490-00de-4139-63a8f7acb492)

#### `GetAllReportProperties(...)`

Retrieves all properties for the model object in one hashtable. Type for the returned value must be checked using type casting.

[Docs](https://developer.tekla.com/topic/en/18/43/0708f404-ce77-28e1-4ebe-fa906f68bff8)

#### `GetAllUserProperties(...)`

Retrieves all properties for the model object in one hashtable. Type for the returned value must be checked using type casting.

[Docs](https://developer.tekla.com/topic/en/18/43/63b853a0-0af5-af9c-3ce7-05299664d7f8)

#### `GetChildren(...)`

Returns an enumerator of all the children model objects.

[Docs](https://developer.tekla.com/topic/en/18/43/ce51ec40-310c-6067-a89f-f8d029aab747)

#### `GetCoordinateSystem(...)`

Returns the coordinate system for the given model object.

[Docs](https://developer.tekla.com/topic/en/18/43/8adfe9cd-f6e5-6474-1297-33c6270a7b0a)

#### `GetCustomObjectType(...)`

Gets custom object type name from an existing object.

[Docs](https://developer.tekla.com/topic/en/18/43/d3be2ba3-8533-7165-f2d3-a5aa9df556b8)

#### `GetDoubleReportProperties(...)`

Retrieves multiple double report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/f86f6c96-f738-49ca-16ba-f5634e862ba2)

#### `GetDoubleUserProperties(...)`

Retrieves all double properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/6ead7a63-5f76-0631-d480-d46c27e882aa)

#### `GetDynamicStringProperty(...)`

Gets a dynamic string property from the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/415e86d9-7de6-ea53-ce68-20b3e67b8655)

#### `GetFatherComponent(...)`

Returns the father component of the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/a2c28f3f-52ff-50a8-1b32-10d18db5c31d)

#### `GetHierarchicObjects(...)`

Returns an enumerator of all the connected hierarchic objects.

[Docs](https://developer.tekla.com/topic/en/18/43/cc228ea0-267b-7d90-7441-c9efc1779b08)

#### `GetIntegerReportProperties(...)`

Retrieves multiple integer report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/2623a9c0-2a9c-e233-9daa-59f92ed51445)

#### `GetIntegerUserProperties(...)`

Retrieves all integer properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/1b3c6f06-bc25-a54e-2a88-b60dc4a3c85e)

#### `GetPhase(...)`

Retrieves the phase of the model object (the phase number, the phase name, the phase comment and whether the phase is the current one or not).

[Docs](https://developer.tekla.com/topic/en/18/43/310f8b7a-c120-753a-35c9-f7c73a9806c9)

#### `GetReportProperty(String, Double.)(...)`

Retrieves a double property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/1c05aa56-5f42-be91-a59e-8d59a5549f45)

#### `GetReportProperty(String, Int32.)(...)`

Retrieves an integer property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/ce8e4ca0-3750-3c9b-5ca8-27e48b45e0bd)

#### `GetReportProperty(String, String.)(...)`

Retrieves a string property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/70f212bb-ee70-e9e9-7ab6-d6a9d46f8de6)

#### `GetStringReportProperties(...)`

Retrieves multiple string report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/5ff72da1-7382-f5d2-eb1d-c99b46239d42)

#### `GetStringUserProperties(...)`

Retrieves all string properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/67120e84-c820-9d52-12ae-c167c595842c)

#### `GetUserProperty(String, Double.)(...)`

Retrieves a double property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/17cc8074-7955-32b5-2b4b-b169c45d4ee0)

#### `GetUserProperty(String, Int32.)(...)`

Retrieves an integer property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/d9364c1d-5b57-6a31-4c13-01124058cfb2)

#### `GetUserProperty(String, String.)(...)`

Retrieves a string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/214a886c-c18c-9b66-d7c9-fe84260962f2)

#### `Insert(...)`

Inserts a polycurve into the model.

[Docs](https://developer.tekla.com/topic/en/18/43/1d08b3cc-4256-d750-193d-f7c83ad1c646)

#### `Modify(...)`

Modifies a polycurve with the given Identifier.

[Docs](https://developer.tekla.com/topic/en/18/43/d64ec560-e0df-e658-7f06-4187b19c36a9)

#### `Select(...)`

Selects a polycurve with the given Identifier.

[Docs](https://developer.tekla.com/topic/en/18/43/98ecd64a-606e-23c3-1ffb-5d2dae51d6cc)

#### `SetCustomObjectType(...)`

Sets a custom object type name for an existing object.

[Docs](https://developer.tekla.com/topic/en/18/43/3d6fb91b-48a9-35ca-d498-526514344089)

#### `SetDynamicStringProperty(...)`

Sets a dynamic string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/4dab0254-09e5-5e61-c28b-1c12afab71a0)

#### `SetLabel(...)`

Sets a label for an object when a new instance is created, this method must be called before Insert. The label is used in plug-ins for identifying the changed object in modification.

[Docs](https://developer.tekla.com/topic/en/18/43/3efcfc44-613a-e66b-9ebd-9f86df4f4036)

#### `SetPhase(...)`

Sets the phase of the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/ed80abcc-0ff7-d081-2229-e651f858a727)

#### `SetUserProperties(...)`

Sets multiple properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/8659da37-d571-f03f-c666-5c9fa3b113f5)

#### `SetUserProperty(String, Double)(...)`

Sets a double property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/5c585b8b-5c2e-7b75-2242-758f17429396)

#### `SetUserProperty(String, Int32)(...)`

Sets an integer property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/dd5b828d-3a87-bb58-5ac9-e1c3b4c2fe4b)

#### `SetUserProperty(String, String)(...)`

Sets a string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/80bf0348-b651-3d4a-862b-49c85bc924ab)

### Properties
- `Color` (object, get/set) — Gets or sets color of the Polycurve.
- `Geometry` (object, get/set) — Gets or sets geometry list that defines geometry of the polycurve.
- `Identifier` (object, get/set) — The identifier of the object.
- `IsUpToDate` (object, get/set) — Gets if the object does not have a modification which is not shared.
- `LineType` (object, get/set) — Gets or sets the line type of the polycurve.
- `ModificationTime` (object, get/set) — Gets latest time of the object was modified or created.

## CurvedRebarGroup (class)

The CurvedRebarGroup class represents a group of reinforcing bars which have a curved shape.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/9875d65f-111f-f5bb-6ba0-e8cb263099ab)

### Constructors
- `CurvedRebarGroup(...)` — Initializes a new curved rebar group instance with empty attributes.

### Methods
#### `CompareTo(...)`

Compares Identifiers of model objects.

[Docs](https://developer.tekla.com/topic/en/18/43/79cca356-0c8a-e8fb-463a-34ad4141bec7)

#### `Delete(...)`

Deletes the curved rebar group instance with the given identifier from the model database.

[Docs](https://developer.tekla.com/topic/en/18/43/420c6fa9-2bd2-be45-a81b-38f946878c24)

#### `Equals(...)`

Check if Identifiers of model objects are same.

[Docs](https://developer.tekla.com/topic/en/18/43/e4324265-2490-00de-4139-63a8f7acb492)

#### `GetAllReportProperties(...)`

Retrieves all properties for the model object in one hashtable. Type for the returned value must be checked using type casting.

[Docs](https://developer.tekla.com/topic/en/18/43/0708f404-ce77-28e1-4ebe-fa906f68bff8)

#### `GetAllUserProperties(...)`

Retrieves all properties for the model object in one hashtable. Type for the returned value must be checked using type casting.

[Docs](https://developer.tekla.com/topic/en/18/43/63b853a0-0af5-af9c-3ce7-05299664d7f8)

#### `GetAssembly(...)`

Returns the assembly that the reinforcement belongs to.

[Docs](https://developer.tekla.com/topic/en/18/43/322f932e-f09b-df0d-c1c8-3de07d79afeb)

#### `GetChildren(...)`

Returns an enumerator of all the children model objects.

[Docs](https://developer.tekla.com/topic/en/18/43/ce51ec40-310c-6067-a89f-f8d029aab747)

#### `GetCoordinateSystem(...)`

Returns the coordinate system for the given model object.

[Docs](https://developer.tekla.com/topic/en/18/43/8adfe9cd-f6e5-6474-1297-33c6270a7b0a)

#### `GetCustomObjectType(...)`

Gets custom object type name from an existing object.

[Docs](https://developer.tekla.com/topic/en/18/43/d3be2ba3-8533-7165-f2d3-a5aa9df556b8)

#### `GetDoubleReportProperties(...)`

Retrieves multiple double report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/f86f6c96-f738-49ca-16ba-f5634e862ba2)

#### `GetDoubleUserProperties(...)`

Retrieves all double properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/6ead7a63-5f76-0631-d480-d46c27e882aa)

#### `GetDynamicStringProperty(...)`

Gets a dynamic string property from the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/415e86d9-7de6-ea53-ce68-20b3e67b8655)

#### `GetFatherComponent(...)`

Returns the father component of the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/a2c28f3f-52ff-50a8-1b32-10d18db5c31d)

#### `GetFatherPour(...)`

Returns the pour that the rebar is associated to.

[Docs](https://developer.tekla.com/topic/en/18/43/0725e80c-89cf-d6d7-ac40-afdf75f6e695)

#### `GetFatherPourUnit(...)`

Returns the pour unit that the rebar is associated to.

[Docs](https://developer.tekla.com/topic/en/18/43/135a51ac-86cb-ef86-cfda-d2bc53a0d8b8)

#### `GetHierarchicObjects(...)`

Returns an enumerator of all the connected hierarchic objects.

[Docs](https://developer.tekla.com/topic/en/18/43/cc228ea0-267b-7d90-7441-c9efc1779b08)

#### `GetIntegerReportProperties(...)`

Retrieves multiple integer report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/2623a9c0-2a9c-e233-9daa-59f92ed51445)

#### `GetIntegerUserProperties(...)`

Retrieves all integer properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/1b3c6f06-bc25-a54e-2a88-b60dc4a3c85e)

#### `GetNumberOfRebars(...)`

Returns the number of rebars in the reinforcing group.

[Docs](https://developer.tekla.com/topic/en/18/43/6b02fd6f-4f84-6550-69c4-bd2f1117b922)

#### `GetPhase(...)`

Retrieves the phase of the model object (the phase number, the phase name, the phase comment and whether the phase is the current one or not).

[Docs](https://developer.tekla.com/topic/en/18/43/310f8b7a-c120-753a-35c9-f7c73a9806c9)

#### `GetRebarComplexGeometries(Boolean, Boolean, Boolean)(...)`

Retrieves a list of physical reinforcing bars (of type RebarComplexGeometry). These objects contain physical curves in the 3D space of each reinforcing bar as shown in model view. RebarComplexGeometry only differs from RebarGeometry by containing a polycurve rather than a polyline.

[Docs](https://developer.tekla.com/topic/en/18/43/c796aafb-3520-fe06-12e3-a07a87c8f2f9)

#### `GetRebarComplexGeometries(Boolean, Boolean, Boolean, Reinforcement.RebarGeometrySimplificationTypeEnum)(...)`

Retrieves a list of physical reinforcing bars (of type RebarComplexGeometry). These objects contain physical curves in the 3D space of each reinforcing bar as shown in model view. RebarComplexGeometry only differs from RebarGeometry by containing a polycurve (of Arc and LineSegment objects) rather than a polyline. The polycurve only contains arcs if the "simplified" parameter is RATIONALIZED or FABRICATION. If "simplified" is NONE, any arcs will be returned as a series of line segments.

[Docs](https://developer.tekla.com/topic/en/18/43/84a3a38b-d7eb-97df-7fba-a34d13de6610)

#### `GetRebarGeometries(Boolean)(...)`

Retrieves a list of physical reinforcing bars (of type RebarGeometry). These objects contain physical points in the 3D space of each reinforcing bar.

[Docs](https://developer.tekla.com/topic/en/18/43/75649a4c-6252-09a1-b509-cd815e718c0f)

#### `GetRebarGeometries(Reinforcement.RebarGeometryOptionEnum)(...)`

Retrieves a list of physical reinforcing bars (of type RebarGeometry). These objects contain physical points in the 3D space of each reinforcing bar.

[Docs](https://developer.tekla.com/topic/en/18/43/bc45309d-fc4b-9bf9-c640-e71bb1ade771)

#### `GetRebarGeometriesWithoutClashes(...)`

Retrieves a list of physical reinforcing bars (of type RebarGeometry). These objects contain physical points in the 3D space of each reinforcing bar as shown in model view. In case rebar polygon clashes with itself, physical points are moved to avoid clashing.

[Docs](https://developer.tekla.com/topic/en/18/43/470acf1f-1beb-f30a-a1c2-2fcab066ab4a)

#### `GetReportProperty(String, Double.)(...)`

Retrieves a double property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/1c05aa56-5f42-be91-a59e-8d59a5549f45)

#### `GetReportProperty(String, Int32.)(...)`

Retrieves an integer property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/ce8e4ca0-3750-3c9b-5ca8-27e48b45e0bd)

#### `GetReportProperty(String, String.)(...)`

Retrieves a string property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/70f212bb-ee70-e9e9-7ab6-d6a9d46f8de6)

#### `GetSingleRebar(...)`

Returns a single rebar inside the rebar group located by the given index. The indexing starts from the start point. The rebar represents a physical reinforcing bar and contains physical points in the 3D space of the bar. The method returns null on error, for example if given an erroneous index. The number of rebars in the group can be requested with GetNumberOfRebars().

[Docs](https://developer.tekla.com/topic/en/18/43/41181569-65f8-f4ba-7722-3cd890c7a3d4)

#### `GetSingleRebarWithoutClash(...)`

Returns a single rebar inside the rebar group located by the given index. The indexing starts from the start point. The rebar represents a physical reinforcing bar and contains physical, non-clashing points in the 3D space of the bar. The method returns null on error, for example if given an erroneous index. The number of rebars in the group can be requested with GetNumberOfRebars().

[Docs](https://developer.tekla.com/topic/en/18/43/8fd2e21a-3d2f-6211-92ba-9d6e652ac100)

#### `GetSolid(...)`

Method for getting the solid information of the reinforcement.

[Docs](https://developer.tekla.com/topic/en/18/43/6717d119-97da-5554-f359-49ee55e65a63)

#### `GetStringReportProperties(...)`

Retrieves multiple string report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/5ff72da1-7382-f5d2-eb1d-c99b46239d42)

#### `GetStringUserProperties(...)`

Retrieves all string properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/67120e84-c820-9d52-12ae-c167c595842c)

#### `GetUserProperty(String, Double.)(...)`

Retrieves a double property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/17cc8074-7955-32b5-2b4b-b169c45d4ee0)

#### `GetUserProperty(String, Int32.)(...)`

Retrieves an integer property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/d9364c1d-5b57-6a31-4c13-01124058cfb2)

#### `GetUserProperty(String, String.)(...)`

Retrieves a string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/214a886c-c18c-9b66-d7c9-fe84260962f2)

#### `Insert(...)`

Inserts the curved rebar group into the model database. All the attributes must be set.

[Docs](https://developer.tekla.com/topic/en/18/43/e498a9a5-cffc-ac33-5477-0a5f98166899)

#### `IsGeometryValid(...)`

Tells whether the geometry of a reinforcement object is valid or not.

[Docs](https://developer.tekla.com/topic/en/18/43/5699f6a6-4e39-b784-d84f-45eb7b1d0e37)

#### `Modify(...)`

Modifies the existing curved rebar group in the model database to match the current one.

[Docs](https://developer.tekla.com/topic/en/18/43/362ad782-e00c-27e0-4f5a-4697ad94a20f)

#### `Select(...)`

Selects a curved rebar group from the model database. The identifier of the curved rebar group must be set.

[Docs](https://developer.tekla.com/topic/en/18/43/32af0cdc-1999-0cf9-5942-af31245b82cc)

#### `SetCustomObjectType(...)`

Sets a custom object type name for an existing object.

[Docs](https://developer.tekla.com/topic/en/18/43/3d6fb91b-48a9-35ca-d498-526514344089)

#### `SetDynamicStringProperty(...)`

Sets a dynamic string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/4dab0254-09e5-5e61-c28b-1c12afab71a0)

#### `SetLabel(...)`

Sets a label for an object when a new instance is created, this method must be called before Insert. The label is used in plug-ins for identifying the changed object in modification.

[Docs](https://developer.tekla.com/topic/en/18/43/3efcfc44-613a-e66b-9ebd-9f86df4f4036)

#### `SetPhase(...)`

Sets the phase of the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/ed80abcc-0ff7-d081-2229-e651f858a727)

#### `SetUserProperties(...)`

Sets multiple properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/8659da37-d571-f03f-c666-5c9fa3b113f5)

#### `SetUserProperty(String, Double)(...)`

Sets a double property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/5c585b8b-5c2e-7b75-2242-758f17429396)

#### `SetUserProperty(String, Int32)(...)`

Sets an integer property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/dd5b828d-3a87-bb58-5ac9-e1c3b4c2fe4b)

#### `SetUserProperty(String, String)(...)`

Sets a string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/80bf0348-b651-3d4a-862b-49c85bc924ab)

### Properties
- `Class` (object, get/set) — Gets or sets the class of the reinforcement. The class is used to group reinforcements.
- `EndFromPlaneOffset` (object, get/set) — The end offset value from the part surface.
- `EndHook` (object, get/set) — The hook at the end of the reinforcing bar.
- `EndPoint` (object, get/set) — The end point of the direction in which the bars are distributed.
- `EndPointOffsetType` (object, get/set) — Gets or sets the type of the end point offset. The options are: OFFSET_TYPE_LEG_LENGTHOFFSET_TYPE_COVER_THICKNESS
- `EndPointOffsetValue` (object, get/set) — Gets or sets the concrete cover thickness or leg length at the second end of the bar.
- `ExcludeType` (object, get/set) — Defines which bars to omit from the group. The options are: EXCLUDE_TYPE_NONEEXCLUDE_TYPE_FIRSTEXCLUDE_TYPE_LASTEXCLUDE_TYPE_BOTH
- `Father` (object, get/set) — Gets or sets the father object of the reinforcement; the model object instance to operate on.
- `FromPlaneOffset` (object, get/set) — The offset value from the part surface applied in both sides.
- `Grade` (object, get/set) — Gets or sets the steel grade of the reinforcing bar. The grade indicates the strength of the steel used in reinforcing bars. It can also indicate other factors, such as the weldability or surface deformations of the bar.
- `Identifier` (object, get/set) — The identifier of the object.
- `InputPointDeformingState` (object, get/set) — Gets or sets the reinforcement input point deforming state.
- `IsUpToDate` (object, get/set) — Gets if the object does not have a modification which is not shared.
- `ModificationTime` (object, get/set) — Gets latest time of the object was modified or created.
- `Name` (object, get/set) — Gets or sets the name of the reinforcement.
- `NumberingSeries` (object, get/set) — Gets or sets the numbering series of the reinforcement.
- `OnPlaneOffsets` (object, get/set) — Gets or sets the double offset value for each leg on the same plane as the bar.
- `Polygon` (object, get/set) — A polygon definition for the curved reinforcing bar group shape. The polygon must have 3 points.
- `RadiusValues` (object, get/set) — Gets or sets the radius value(s) of the bends in the bar.
- `Size` (object, get/set) — The size of the reinforcement.
- `Spacings` (object, get/set) — The spacing value(s). If the type of the spacing is SPACING_TYPE_EXACT_NUMBER Spacings has only one value that defines the number of the reinforcing bars.
- `SpacingType` (object, get/set) — The type of spacing. The options are (BaseRebarGroup.RebarGroupSpacingTypeEnum.): SPACING_TYPE_UNDEFINEDSPACING_TYPE_EXACT_SPACINGSSPACING_TYPE_EXACT_NUMBERSPACING_TYPE_TARGET_SPACESPACING_TYPE_EXACT_SPACE_FLEX_AT_STARTSPACING_TYPE_EXACT_SPACE_FLEX_AT_ENDSPACING_TYPE_EXACT_SPACE_FLEX_AT_BOTHSPACING_TYPE_EXACT_SPACE_FLEX_AT_MIDDLE
- `StartFromPlaneOffset` (object, get/set) — The start offset value from the part surface.
- `StartHook` (object, get/set) — The hook at the beginning of the reinforcing bar.
- `StartPoint` (object, get/set) — The start point of the direction in which the bars are distributed.
- `StartPointOffsetType` (object, get/set) — Gets or sets the type of the start point offset is either OFFSET_TYPE_LEG_LENGTH or OFFSET_TYPE_COVER_THICKNESS.
- `StartPointOffsetValue` (object, get/set) — Gets or sets the concrete cover thickness or leg length at the first end of the bar.

## CustomPart (class)

The CustomPart class represents a custom part object in the Tekla Structures model. A custom part is a modeling tool that typically assembles multiple parts to build some type of a beam-like structure, for example a tapered beam or a sandwich panel.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/d49ee520-e614-1f88-32b0-3bd442cfc84c)

### Constructors
- `CustomPart(...)` — Creates a new custom part instance.
- `CustomPart(...)` — Creates a new custom part instance with the given input.

### Methods
#### `CompareTo(...)`

Compares Identifiers of model objects.

[Docs](https://developer.tekla.com/topic/en/18/43/79cca356-0c8a-e8fb-463a-34ad4141bec7)

#### `Delete(...)`

Deletes the custom part instance with the given identifier from the model database.

[Docs](https://developer.tekla.com/topic/en/18/43/0a9b2f2a-8420-5e3b-c265-85cbdd4d2074)

#### `Equals(...)`

Check if Identifiers of model objects are same.

[Docs](https://developer.tekla.com/topic/en/18/43/e4324265-2490-00de-4139-63a8f7acb492)

#### `GetAllReportProperties(...)`

Retrieves all properties for the model object in one hashtable. Type for the returned value must be checked using type casting.

[Docs](https://developer.tekla.com/topic/en/18/43/0708f404-ce77-28e1-4ebe-fa906f68bff8)

#### `GetAllUserProperties(...)`

Retrieves all properties for the model object in one hashtable. Type for the returned value must be checked using type casting.

[Docs](https://developer.tekla.com/topic/en/18/43/63b853a0-0af5-af9c-3ce7-05299664d7f8)

#### `GetAssembly(...)`

Returns the assembly that the custom part belongs to.

[Docs](https://developer.tekla.com/topic/en/18/43/80c030d6-66a2-c153-a0bf-0f940e3c74f8)

#### `GetAttribute(String, Double.)(...)`

Retrieves the attribute with the given name.

[Docs](https://developer.tekla.com/topic/en/18/43/a347ce9e-865f-554d-140f-9a65728b24a3)

#### `GetAttribute(String, Int32.)(...)`

Retrieves the attribute with the given name.

[Docs](https://developer.tekla.com/topic/en/18/43/2c48112c-5580-99f3-6c35-70fd5f538b09)

#### `GetAttribute(String, String.)(...)`

Retrieves the attribute with the given name.

[Docs](https://developer.tekla.com/topic/en/18/43/c8652aa1-9069-88f2-b821-3afdd68e7ca5)

#### `GetBooleans(...)`

Returns an enumerator of all the connected boolean objects.

[Docs](https://developer.tekla.com/topic/en/18/43/cd27e725-0b11-fe7b-ff7d-94fa4e4d93c5)

#### `GetChildren(...)`

Returns an enumerator of all the children model objects.

[Docs](https://developer.tekla.com/topic/en/18/43/ce51ec40-310c-6067-a89f-f8d029aab747)

#### `GetComponents(...)`

Returns an enumerator of all the connected components.

[Docs](https://developer.tekla.com/topic/en/18/43/e798c1b7-0103-df32-534a-42160f26a167)

#### `GetCoordinateSystem(...)`

Returns the coordinate system for the given model object.

[Docs](https://developer.tekla.com/topic/en/18/43/8adfe9cd-f6e5-6474-1297-33c6270a7b0a)

#### `GetCustomObjectType(...)`

Gets custom object type name from an existing object.

[Docs](https://developer.tekla.com/topic/en/18/43/d3be2ba3-8533-7165-f2d3-a5aa9df556b8)

#### `GetDoubleReportProperties(...)`

Retrieves multiple double report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/f86f6c96-f738-49ca-16ba-f5634e862ba2)

#### `GetDoubleUserProperties(...)`

Retrieves all double properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/6ead7a63-5f76-0631-d480-d46c27e882aa)

#### `GetDynamicStringProperty(...)`

Gets a dynamic string property from the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/415e86d9-7de6-ea53-ce68-20b3e67b8655)

#### `GetFatherComponent(...)`

Returns the father component of the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/a2c28f3f-52ff-50a8-1b32-10d18db5c31d)

#### `GetHierarchicObjects(...)`

Returns an enumerator of all the connected hierarchic objects.

[Docs](https://developer.tekla.com/topic/en/18/43/cc228ea0-267b-7d90-7441-c9efc1779b08)

#### `GetIntegerReportProperties(...)`

Retrieves multiple integer report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/2623a9c0-2a9c-e233-9daa-59f92ed51445)

#### `GetIntegerUserProperties(...)`

Retrieves all integer properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/1b3c6f06-bc25-a54e-2a88-b60dc4a3c85e)

#### `GetPhase(...)`

Retrieves the phase of the model object (the phase number, the phase name, the phase comment and whether the phase is the current one or not).

[Docs](https://developer.tekla.com/topic/en/18/43/310f8b7a-c120-753a-35c9-f7c73a9806c9)

#### `GetReportProperty(String, Double.)(...)`

Retrieves a double property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/1c05aa56-5f42-be91-a59e-8d59a5549f45)

#### `GetReportProperty(String, Int32.)(...)`

Retrieves an integer property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/ce8e4ca0-3750-3c9b-5ca8-27e48b45e0bd)

#### `GetReportProperty(String, String.)(...)`

Retrieves a string property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/70f212bb-ee70-e9e9-7ab6-d6a9d46f8de6)

#### `GetStartAndEndPositions(...)`

Returns the start and end points of the custom part.

[Docs](https://developer.tekla.com/topic/en/18/43/7aa159fb-14cc-a60a-c767-9f19fa93cf62)

#### `GetStringReportProperties(...)`

Retrieves multiple string report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/5ff72da1-7382-f5d2-eb1d-c99b46239d42)

#### `GetStringUserProperties(...)`

Retrieves all string properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/67120e84-c820-9d52-12ae-c167c595842c)

#### `GetUserProperty(String, Double.)(...)`

Retrieves a double property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/17cc8074-7955-32b5-2b4b-b169c45d4ee0)

#### `GetUserProperty(String, Int32.)(...)`

Retrieves an integer property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/d9364c1d-5b57-6a31-4c13-01124058cfb2)

#### `GetUserProperty(String, String.)(...)`

Retrieves a string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/214a886c-c18c-9b66-d7c9-fe84260962f2)

#### `Insert(...)`

Inserts the custom part into the model database.

[Docs](https://developer.tekla.com/topic/en/18/43/5dc9a7bb-dedf-11dd-a22c-b2abc37cf688)

#### `LoadAttributesFromFile(...)`

Loads the attributes for the component from the given file. These attributes will be loaded before all the attributes that have been set with the SetAttribute methods, so any attributes that are set with SetAttribute will override those loaded from the given standard file.

[Docs](https://developer.tekla.com/topic/en/18/43/acc9930a-aa6f-667c-0878-7ea80ce416b8)

#### `Modify(...)`

Modifies the existing custom part in the model database to match the current one.

[Docs](https://developer.tekla.com/topic/en/18/43/b996076d-5a6c-3d28-8b48-e44dd518f3f9)

#### `Select(...)`

Selects a custom part from the model database. The custom part identifier must be set.

[Docs](https://developer.tekla.com/topic/en/18/43/8f178f6c-4035-2182-652b-8772c0ab5f73)

#### `SetAttribute(String, Double)(...)`

Sets the attribute's value to the given value.

[Docs](https://developer.tekla.com/topic/en/18/43/785d0216-df12-53fc-344b-a2e67001cf3c)

#### `SetAttribute(String, Int32)(...)`

Sets the attribute's value to the given value.

[Docs](https://developer.tekla.com/topic/en/18/43/797240ce-567c-bff9-2a60-6359cc909ce0)

#### `SetAttribute(String, String)(...)`

Sets the attribute's value to the given value.

[Docs](https://developer.tekla.com/topic/en/18/43/6f0f7c8f-c5d2-8012-9faf-f68991966ff5)

#### `SetCustomObjectType(...)`

Sets a custom object type name for an existing object.

[Docs](https://developer.tekla.com/topic/en/18/43/3d6fb91b-48a9-35ca-d498-526514344089)

#### `SetDynamicStringProperty(...)`

Sets a dynamic string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/4dab0254-09e5-5e61-c28b-1c12afab71a0)

#### `SetInputPositions(...)`

Sets the start and end point of the custom part.

[Docs](https://developer.tekla.com/topic/en/18/43/548e98d5-bb40-4c52-59d7-26a74b5ee88e)

#### `SetLabel(...)`

Sets a label for an object when a new instance is created, this method must be called before Insert. The label is used in plug-ins for identifying the changed object in modification.

[Docs](https://developer.tekla.com/topic/en/18/43/3efcfc44-613a-e66b-9ebd-9f86df4f4036)

#### `SetPhase(...)`

Sets the phase of the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/ed80abcc-0ff7-d081-2229-e651f858a727)

#### `SetUserProperties(...)`

Sets multiple properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/8659da37-d571-f03f-c666-5c9fa3b113f5)

#### `SetUserProperty(String, Double)(...)`

Sets a double property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/5c585b8b-5c2e-7b75-2242-758f17429396)

#### `SetUserProperty(String, Int32)(...)`

Sets an integer property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/dd5b828d-3a87-bb58-5ac9-e1c3b4c2fe4b)

#### `SetUserProperty(String, String)(...)`

Sets a string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/80bf0348-b651-3d4a-862b-49c85bc924ab)

### Properties
- `Identifier` (object, get/set) — The identifier of the object.
- `IsUpToDate` (object, get/set) — Gets if the object does not have a modification which is not shared.
- `ModificationTime` (object, get/set) — Gets latest time of the object was modified or created.
- `Name` (object, get/set) — The name of the component. The name identifies custom components or plug-ins.
- `Number` (object, get/set) — The number of the component. A number greater than zero identifies system components, for custom components the number is CUSTOM_OBJECT_NUMBER, and for plug-ins the number is PLUGIN_OBJECT_NUMBER.
- `Position` (object, get/set) — The way the custom part is positioned in the model.

## CutPlane (class)

The CutPlane class defines how the end of a part is cut with a plane. A cut plane differs from a fitting because a cut can never extend the boundaries of the original part.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/9ffbb991-b111-ad28-f0c0-5b9484c1002a)

### Constructors
- `CutPlane(...)` — Initializes a new instance of the CutPlane class with empty attributes.

### Methods
#### `CompareTo(...)`

Compares Identifiers of model objects.

[Docs](https://developer.tekla.com/topic/en/18/43/79cca356-0c8a-e8fb-463a-34ad4141bec7)

#### `Delete(...)`

Deletes the cut plane instance with the given ID from the model database.

[Docs](https://developer.tekla.com/topic/en/18/43/639f58b3-5a2d-2ae1-fcec-81644576764c)

#### `Equals(...)`

Check if Identifiers of model objects are same.

[Docs](https://developer.tekla.com/topic/en/18/43/e4324265-2490-00de-4139-63a8f7acb492)

#### `GetAllReportProperties(...)`

Retrieves all properties for the model object in one hashtable. Type for the returned value must be checked using type casting.

[Docs](https://developer.tekla.com/topic/en/18/43/0708f404-ce77-28e1-4ebe-fa906f68bff8)

#### `GetAllUserProperties(...)`

Retrieves all properties for the model object in one hashtable. Type for the returned value must be checked using type casting.

[Docs](https://developer.tekla.com/topic/en/18/43/63b853a0-0af5-af9c-3ce7-05299664d7f8)

#### `GetChildren(...)`

Returns an enumerator of all the children model objects.

[Docs](https://developer.tekla.com/topic/en/18/43/ce51ec40-310c-6067-a89f-f8d029aab747)

#### `GetCoordinateSystem(...)`

Returns the coordinate system for the given model object.

[Docs](https://developer.tekla.com/topic/en/18/43/8adfe9cd-f6e5-6474-1297-33c6270a7b0a)

#### `GetCustomObjectType(...)`

Gets custom object type name from an existing object.

[Docs](https://developer.tekla.com/topic/en/18/43/d3be2ba3-8533-7165-f2d3-a5aa9df556b8)

#### `GetDoubleReportProperties(...)`

Retrieves multiple double report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/f86f6c96-f738-49ca-16ba-f5634e862ba2)

#### `GetDoubleUserProperties(...)`

Retrieves all double properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/6ead7a63-5f76-0631-d480-d46c27e882aa)

#### `GetDynamicStringProperty(...)`

Gets a dynamic string property from the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/415e86d9-7de6-ea53-ce68-20b3e67b8655)

#### `GetFatherComponent(...)`

Returns the father component of the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/a2c28f3f-52ff-50a8-1b32-10d18db5c31d)

#### `GetHierarchicObjects(...)`

Returns an enumerator of all the connected hierarchic objects.

[Docs](https://developer.tekla.com/topic/en/18/43/cc228ea0-267b-7d90-7441-c9efc1779b08)

#### `GetIntegerReportProperties(...)`

Retrieves multiple integer report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/2623a9c0-2a9c-e233-9daa-59f92ed51445)

#### `GetIntegerUserProperties(...)`

Retrieves all integer properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/1b3c6f06-bc25-a54e-2a88-b60dc4a3c85e)

#### `GetPhase(...)`

Retrieves the phase of the model object (the phase number, the phase name, the phase comment and whether the phase is the current one or not).

[Docs](https://developer.tekla.com/topic/en/18/43/310f8b7a-c120-753a-35c9-f7c73a9806c9)

#### `GetReportProperty(String, Double.)(...)`

Retrieves a double property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/1c05aa56-5f42-be91-a59e-8d59a5549f45)

#### `GetReportProperty(String, Int32.)(...)`

Retrieves an integer property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/ce8e4ca0-3750-3c9b-5ca8-27e48b45e0bd)

#### `GetReportProperty(String, String.)(...)`

Retrieves a string property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/70f212bb-ee70-e9e9-7ab6-d6a9d46f8de6)

#### `GetStringReportProperties(...)`

Retrieves multiple string report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/5ff72da1-7382-f5d2-eb1d-c99b46239d42)

#### `GetStringUserProperties(...)`

Retrieves all string properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/67120e84-c820-9d52-12ae-c167c595842c)

#### `GetUserProperty(String, Double.)(...)`

Retrieves a double property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/17cc8074-7955-32b5-2b4b-b169c45d4ee0)

#### `GetUserProperty(String, Int32.)(...)`

Retrieves an integer property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/d9364c1d-5b57-6a31-4c13-01124058cfb2)

#### `GetUserProperty(String, String.)(...)`

Retrieves a string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/214a886c-c18c-9b66-d7c9-fe84260962f2)

#### `Insert(...)`

Inserts the cut plane into the model database. All the attributes must be set.

[Docs](https://developer.tekla.com/topic/en/18/43/a270c562-3f8e-b717-5fdb-382f080ccc47)

#### `Modify(...)`

Modifies the existing cut plane in the model database to match the current one.

[Docs](https://developer.tekla.com/topic/en/18/43/875c87b3-bc31-5473-87b1-9e013c31e9de)

#### `Select(...)`

Selects a cut plane from the model database. The part ID must be set.

[Docs](https://developer.tekla.com/topic/en/18/43/80e49c8d-b895-2e51-1ef8-3a627ecffebc)

#### `SetCustomObjectType(...)`

Sets a custom object type name for an existing object.

[Docs](https://developer.tekla.com/topic/en/18/43/3d6fb91b-48a9-35ca-d498-526514344089)

#### `SetDynamicStringProperty(...)`

Sets a dynamic string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/4dab0254-09e5-5e61-c28b-1c12afab71a0)

#### `SetLabel(...)`

Sets a label for an object when a new instance is created, this method must be called before Insert. The label is used in plug-ins for identifying the changed object in modification.

[Docs](https://developer.tekla.com/topic/en/18/43/3efcfc44-613a-e66b-9ebd-9f86df4f4036)

#### `SetPhase(...)`

Sets the phase of the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/ed80abcc-0ff7-d081-2229-e651f858a727)

#### `SetUserProperties(...)`

Sets multiple properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/8659da37-d571-f03f-c666-5c9fa3b113f5)

#### `SetUserProperty(String, Double)(...)`

Sets a double property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/5c585b8b-5c2e-7b75-2242-758f17429396)

#### `SetUserProperty(String, Int32)(...)`

Sets an integer property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/dd5b828d-3a87-bb58-5ac9-e1c3b4c2fe4b)

#### `SetUserProperty(String, String)(...)`

Sets a string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/80bf0348-b651-3d4a-862b-49c85bc924ab)

### Properties
- `Father` (object, get/set) — The father object of the boolean operation; the model object instance to operate on.
- `Identifier` (object, get/set) — The identifier of the object.
- `IsUpToDate` (object, get/set) — Gets if the object does not have a modification which is not shared.
- `ModificationTime` (object, get/set) — Gets latest time of the object was modified or created.
- `Plane` (object, get/set) — The plane that cuts the part.

## CylindricalSurface (class)

The CylindricalSurface class defines a cylindrical surface contour.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/79e12062-9cfd-f132-b719-45e61c809615)

### Constructors
- `CylindricalSurface(...)` — Initializes a new instance of the CylindricalSurface class from a base arc and a height. This is the recommended constructor if no intermediate points are desired along the lateral boundaries of the bend surface.
- `CylindricalSurface(...)` — Initializes a new instance of the CylindricalSurface class from the cone lateral boundaries and the center line. This is the recommended constructor if fine grained control is desired over the lateral boundaries of the surface.
- `CylindricalSurface(...)` — Initializes a new instance of the CylindricalSurface class with given parameters. The boundaries are defined by the lateral boundaries (i.e. the points on the sides of the curved part of the cylinder). The side boundaries are defined by the two first points and the two last points of the lateral boundaries.
- `CylindricalSurface(...)` — Initializes a new instance of the CylindricalSurface class with given parameters. The boundaries are defined by the side boundaries (i.e. generator lines of the cylinder).

### Properties
- `CenterLine` (object, get/set) — Gets the center line of the bend surface (i.e. the line that crosses every circular cross section of the bend at their center point)
- `EndFaceNormal1` (object, get/set) — Gets the first end face normal.
- `EndFaceNormal2` (object, get/set) — Gets the second end face normal.
- `IntersectionLine` (object, get/set) — Gets intersection line.
- `InwardCurved` (object, get/set) — Gets the direction of the curve - true if the curve is oriented towards the intersection line, false otherwise.
- `LateralBoundary1` (object, get/set) — Gets or sets the first lateral boundary
- `LateralBoundary2` (object, get/set) — Gets or sets the second lateral boundary
- `Radius` (object, get/set) — Gets radius of the cylindrical surface.
- `RotationAxis` (object, get/set) — Gets the unit vector that defines the rotation axis (for counter clockwise rotations) of the lateral boundaries of the surface. This axis is always parallel to the direction of the center line.
- `SideBoundary1` (object, get/set) — Gets or sets the first side boundary.
- `SideBoundary2` (object, get/set) — Gets or sets the second side boundary.

## CylindricalSurfaceNode (class)

The CylindricalSurfaceNode class represents cylindrical surface geometry tree node.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/647c9194-6cfd-7361-719f-9107ecae25a6)

### Constructors
- `CylindricalSurfaceNode(...)` — Initializes a new instance of the CylindricalSurfaceNode class.

### Methods
#### `AcceptVisitor(...)`

Entry method for the visitor pattern

[Docs](https://developer.tekla.com/topic/en/18/43/2868c6b0-a860-931e-fa9b-47dae98b5b3c)

#### `Clone(...)`

Creates a new object that is a copy of the current instance.

[Docs](https://developer.tekla.com/topic/en/18/43/10b89604-e30a-248d-d676-57e9e21c0baf)

### Properties
- `IsAutomatic` (object, get/set) — Gets a value indicating whether this geometry node was automatically generated (returns false if it was originally a user-defined part)
- `Surface` (object, get/set) — Gets the cylindrical surface geometry.

## DeformingData (class)

The DeformingData class holds information about the deforming of parts. Deformed parts are parts that have been warped, cambered or shortened.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/d344d31d-6c1a-e537-ddf0-57d0a2c0a48f)

### Constructors
- `DeformingData(...)` — Instantiates a deforming data instance.

### Properties
- `Angle` (object, get/set) — The angle of the part at its start point, relative to the part reference line. For example, if the beam is warped 45 degrees at the end point, in the Angle field the value is 0, and the value is 45 in the Angle2 field.
- `Angle2` (object, get/set) — The angle of the part at its end point, relative to the part reference line. For example, if the beam is warped 45 degrees at the end point, in the Angle field the value is 0, and the value is 45 in the Angle2 field.
- `Cambering` (object, get/set) — The degree of cambering.
- `Shortening` (object, get/set) — The degree of shortening.

## Detail (class)

The Detail class represents a detail. A detail is different from a connection since the detail only connects to one part.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/5e5bcfd3-7bef-21c3-0c09-059814ebdebb)

### Constructors
- `Detail(...)` — Creates a new detail instance.

### Methods
#### `CompareTo(...)`

Compares Identifiers of model objects.

[Docs](https://developer.tekla.com/topic/en/18/43/79cca356-0c8a-e8fb-463a-34ad4141bec7)

#### `Delete(...)`

Deletes the detail instance with the given ID from the model database.

[Docs](https://developer.tekla.com/topic/en/18/43/ad2e3ca8-a3ab-dffd-1319-a8c9a8bc0a1e)

#### `Equals(...)`

Check if Identifiers of model objects are same.

[Docs](https://developer.tekla.com/topic/en/18/43/e4324265-2490-00de-4139-63a8f7acb492)

#### `GetAllReportProperties(...)`

Retrieves all properties for the model object in one hashtable. Type for the returned value must be checked using type casting.

[Docs](https://developer.tekla.com/topic/en/18/43/0708f404-ce77-28e1-4ebe-fa906f68bff8)

#### `GetAllUserProperties(...)`

Retrieves all properties for the model object in one hashtable. Type for the returned value must be checked using type casting.

[Docs](https://developer.tekla.com/topic/en/18/43/63b853a0-0af5-af9c-3ce7-05299664d7f8)

#### `GetAttribute(String, Double.)(...)`

Retrieves the attribute with the given name.

[Docs](https://developer.tekla.com/topic/en/18/43/a347ce9e-865f-554d-140f-9a65728b24a3)

#### `GetAttribute(String, Int32.)(...)`

Retrieves the attribute with the given name.

[Docs](https://developer.tekla.com/topic/en/18/43/2c48112c-5580-99f3-6c35-70fd5f538b09)

#### `GetAttribute(String, String.)(...)`

Retrieves the attribute with the given name.

[Docs](https://developer.tekla.com/topic/en/18/43/c8652aa1-9069-88f2-b821-3afdd68e7ca5)

#### `GetChildren(...)`

Returns an enumerator of all the children model objects.

[Docs](https://developer.tekla.com/topic/en/18/43/ce51ec40-310c-6067-a89f-f8d029aab747)

#### `GetCoordinateSystem(...)`

Returns the coordinate system for the given model object.

[Docs](https://developer.tekla.com/topic/en/18/43/8adfe9cd-f6e5-6474-1297-33c6270a7b0a)

#### `GetCustomObjectType(...)`

Gets custom object type name from an existing object.

[Docs](https://developer.tekla.com/topic/en/18/43/d3be2ba3-8533-7165-f2d3-a5aa9df556b8)

#### `GetDoubleReportProperties(...)`

Retrieves multiple double report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/f86f6c96-f738-49ca-16ba-f5634e862ba2)

#### `GetDoubleUserProperties(...)`

Retrieves all double properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/6ead7a63-5f76-0631-d480-d46c27e882aa)

#### `GetDynamicStringProperty(...)`

Gets a dynamic string property from the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/415e86d9-7de6-ea53-ce68-20b3e67b8655)

#### `GetFatherComponent(...)`

Returns the father component of the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/a2c28f3f-52ff-50a8-1b32-10d18db5c31d)

#### `GetHierarchicObjects(...)`

Returns an enumerator of all the connected hierarchic objects.

[Docs](https://developer.tekla.com/topic/en/18/43/cc228ea0-267b-7d90-7441-c9efc1779b08)

#### `GetIntegerReportProperties(...)`

Retrieves multiple integer report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/2623a9c0-2a9c-e233-9daa-59f92ed51445)

#### `GetIntegerUserProperties(...)`

Retrieves all integer properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/1b3c6f06-bc25-a54e-2a88-b60dc4a3c85e)

#### `GetPhase(...)`

Retrieves the phase of the model object (the phase number, the phase name, the phase comment and whether the phase is the current one or not).

[Docs](https://developer.tekla.com/topic/en/18/43/310f8b7a-c120-753a-35c9-f7c73a9806c9)

#### `GetPrimaryObject(...)`

Returns the primary object.

[Docs](https://developer.tekla.com/topic/en/18/43/6b4f82a2-1d05-361b-d6fe-2748137fe38a)

#### `GetReferencePoint(...)`

Returns the reference point.

[Docs](https://developer.tekla.com/topic/en/18/43/118c71b4-7a7d-a7c9-200f-75e8b768ec01)

#### `GetReportProperty(String, Double.)(...)`

Retrieves a double property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/1c05aa56-5f42-be91-a59e-8d59a5549f45)

#### `GetReportProperty(String, Int32.)(...)`

Retrieves an integer property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/ce8e4ca0-3750-3c9b-5ca8-27e48b45e0bd)

#### `GetReportProperty(String, String.)(...)`

Retrieves a string property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/70f212bb-ee70-e9e9-7ab6-d6a9d46f8de6)

#### `GetStringReportProperties(...)`

Retrieves multiple string report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/5ff72da1-7382-f5d2-eb1d-c99b46239d42)

#### `GetStringUserProperties(...)`

Retrieves all string properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/67120e84-c820-9d52-12ae-c167c595842c)

#### `GetUserProperty(String, Double.)(...)`

Retrieves a double property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/17cc8074-7955-32b5-2b4b-b169c45d4ee0)

#### `GetUserProperty(String, Int32.)(...)`

Retrieves an integer property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/d9364c1d-5b57-6a31-4c13-01124058cfb2)

#### `GetUserProperty(String, String.)(...)`

Retrieves a string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/214a886c-c18c-9b66-d7c9-fe84260962f2)

#### `Insert(...)`

Inserts the detail into the model database. All the attributes must be set.

[Docs](https://developer.tekla.com/topic/en/18/43/3f3361f6-b718-15c1-554d-23afadedcd33)

#### `LoadAttributesFromFile(...)`

Loads the attributes for the component from the given file. These attributes will be loaded before all the attributes that have been set with the SetAttribute methods, so any attributes that are set with SetAttribute will override those loaded from the given standard file.

[Docs](https://developer.tekla.com/topic/en/18/43/acc9930a-aa6f-667c-0878-7ea80ce416b8)

#### `Modify(...)`

Modifies the existing detail in the model database to match the current one.

[Docs](https://developer.tekla.com/topic/en/18/43/a86a0b10-fb7d-cbcb-cbc6-fd0076571f34)

#### `Select(...)`

Selects a detail from the model database. The detail ID must be set.

[Docs](https://developer.tekla.com/topic/en/18/43/ca36ecc0-4139-6a75-bcf9-2091a96093b6)

#### `SetAttribute(String, Double)(...)`

Sets the attribute's value to the given value.

[Docs](https://developer.tekla.com/topic/en/18/43/785d0216-df12-53fc-344b-a2e67001cf3c)

#### `SetAttribute(String, Int32)(...)`

Sets the attribute's value to the given value.

[Docs](https://developer.tekla.com/topic/en/18/43/797240ce-567c-bff9-2a60-6359cc909ce0)

#### `SetAttribute(String, String)(...)`

Sets the attribute's value to the given value.

[Docs](https://developer.tekla.com/topic/en/18/43/6f0f7c8f-c5d2-8012-9faf-f68991966ff5)

#### `SetCustomObjectType(...)`

Sets a custom object type name for an existing object.

[Docs](https://developer.tekla.com/topic/en/18/43/3d6fb91b-48a9-35ca-d498-526514344089)

#### `SetDynamicStringProperty(...)`

Sets a dynamic string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/4dab0254-09e5-5e61-c28b-1c12afab71a0)

#### `SetLabel(...)`

Sets a label for an object when a new instance is created, this method must be called before Insert. The label is used in plug-ins for identifying the changed object in modification.

[Docs](https://developer.tekla.com/topic/en/18/43/3efcfc44-613a-e66b-9ebd-9f86df4f4036)

#### `SetPhase(...)`

Sets the phase of the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/ed80abcc-0ff7-d081-2229-e651f858a727)

#### `SetPrimaryObject(...)`

Sets the detail's primary object.

[Docs](https://developer.tekla.com/topic/en/18/43/849229b7-7246-bc17-0bb5-72a0860fa200)

#### `SetReferencePoint(...)`

Sets the reference point; the point the user would pick for the detail to appear to.

[Docs](https://developer.tekla.com/topic/en/18/43/73e6af33-275c-0305-b8c9-a4084bc96ae7)

#### `SetUserProperties(...)`

Sets multiple properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/8659da37-d571-f03f-c666-5c9fa3b113f5)

#### `SetUserProperty(String, Double)(...)`

Sets a double property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/5c585b8b-5c2e-7b75-2242-758f17429396)

#### `SetUserProperty(String, Int32)(...)`

Sets an integer property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/dd5b828d-3a87-bb58-5ac9-e1c3b4c2fe4b)

#### `SetUserProperty(String, String)(...)`

Sets a string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/80bf0348-b651-3d4a-862b-49c85bc924ab)

### Properties
- `AutoDirectionType` (object, get/set) — The auto direction type. Default value is AUTODIR_DETAIL. Value should be AUTODIR_NA to set direction with UpVector property. Value should be AUTODIR_FROM_ATTRIBUTE_FILE to set direction with detail attribute file.
- `Class` (object, get/set) — The class of the detail.
- `Code` (object, get/set) — The code of the detail. The code can be used to classify the detail. The code can be reported and shown in drawings. The maximum length of the string is 20 characters.
- `DetailType` (object, get/set) — The detail type.
- `Identifier` (object, get/set) — The identifier of the object.
- `IsUpToDate` (object, get/set) — Gets if the object does not have a modification which is not shared.
- `ModificationTime` (object, get/set) — Gets latest time of the object was modified or created.
- `Name` (object, get/set) — The name of the component. The name identifies custom components or plug-ins.
- `Number` (object, get/set) — The number of the component. A number greater than zero identifies system components, for custom components the number is CUSTOM_OBJECT_NUMBER, and for plug-ins the number is PLUGIN_OBJECT_NUMBER.
- `PositionType` (object, get/set) — The position type.
- `Status` (object, get/set) — The read only status of the detail. The status can be reported and shown in drawings. The color of the detail symbol in the model indicates the status of the detail.
- `UpVector` (object, get/set) — A vector indicating which direction is considered the up direction.

## DisposableToken (class)

IDisposable delegate adapter.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/6fceaf49-8ef1-a334-d72d-a109a1a627d9)

### Constructors
- `DisposableToken(...)` — Initializes a new instance of the DisposableToken class.

### Methods
#### `Dispose(...)`

Invokes the callback.

[Docs](https://developer.tekla.com/topic/en/18/43/0b2e9549-6519-5ab3-d049-137eacefe337)

## EdgeChamfer (class)

The EdgeChamfer class defines how the part edge is chamfered.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/803ad6ff-2940-2bff-01b5-0241125e38b5)

### Constructors
- `EdgeChamfer(...)` — Initializes a new instance of the EdgeChamfer class with empty attributes.
- `EdgeChamfer(...)` — Initializes a new instance of the EdgeChamfer class.

### Methods
#### `CompareTo(...)`

Compares Identifiers of model objects.

[Docs](https://developer.tekla.com/topic/en/18/43/79cca356-0c8a-e8fb-463a-34ad4141bec7)

#### `Delete(...)`

Deletes the edge chamfer instance with the given ID from the model database.

[Docs](https://developer.tekla.com/topic/en/18/43/c1d17010-214c-5928-8699-be0bb90f4b79)

#### `Equals(...)`

Check if Identifiers of model objects are same.

[Docs](https://developer.tekla.com/topic/en/18/43/e4324265-2490-00de-4139-63a8f7acb492)

#### `GetAllReportProperties(...)`

Retrieves all properties for the model object in one hashtable. Type for the returned value must be checked using type casting.

[Docs](https://developer.tekla.com/topic/en/18/43/0708f404-ce77-28e1-4ebe-fa906f68bff8)

#### `GetAllUserProperties(...)`

Retrieves all properties for the model object in one hashtable. Type for the returned value must be checked using type casting.

[Docs](https://developer.tekla.com/topic/en/18/43/63b853a0-0af5-af9c-3ce7-05299664d7f8)

#### `GetChildren(...)`

Returns an enumerator of all the children model objects.

[Docs](https://developer.tekla.com/topic/en/18/43/ce51ec40-310c-6067-a89f-f8d029aab747)

#### `GetCoordinateSystem(...)`

Returns the coordinate system for the given model object.

[Docs](https://developer.tekla.com/topic/en/18/43/8adfe9cd-f6e5-6474-1297-33c6270a7b0a)

#### `GetCustomObjectType(...)`

Gets custom object type name from an existing object.

[Docs](https://developer.tekla.com/topic/en/18/43/d3be2ba3-8533-7165-f2d3-a5aa9df556b8)

#### `GetDoubleReportProperties(...)`

Retrieves multiple double report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/f86f6c96-f738-49ca-16ba-f5634e862ba2)

#### `GetDoubleUserProperties(...)`

Retrieves all double properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/6ead7a63-5f76-0631-d480-d46c27e882aa)

#### `GetDynamicStringProperty(...)`

Gets a dynamic string property from the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/415e86d9-7de6-ea53-ce68-20b3e67b8655)

#### `GetFatherComponent(...)`

Returns the father component of the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/a2c28f3f-52ff-50a8-1b32-10d18db5c31d)

#### `GetHierarchicObjects(...)`

Returns an enumerator of all the connected hierarchic objects.

[Docs](https://developer.tekla.com/topic/en/18/43/cc228ea0-267b-7d90-7441-c9efc1779b08)

#### `GetIntegerReportProperties(...)`

Retrieves multiple integer report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/2623a9c0-2a9c-e233-9daa-59f92ed51445)

#### `GetIntegerUserProperties(...)`

Retrieves all integer properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/1b3c6f06-bc25-a54e-2a88-b60dc4a3c85e)

#### `GetPhase(...)`

Retrieves the phase of the model object (the phase number, the phase name, the phase comment and whether the phase is the current one or not).

[Docs](https://developer.tekla.com/topic/en/18/43/310f8b7a-c120-753a-35c9-f7c73a9806c9)

#### `GetReportProperty(String, Double.)(...)`

Retrieves a double property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/1c05aa56-5f42-be91-a59e-8d59a5549f45)

#### `GetReportProperty(String, Int32.)(...)`

Retrieves an integer property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/ce8e4ca0-3750-3c9b-5ca8-27e48b45e0bd)

#### `GetReportProperty(String, String.)(...)`

Retrieves a string property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/70f212bb-ee70-e9e9-7ab6-d6a9d46f8de6)

#### `GetStringReportProperties(...)`

Retrieves multiple string report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/5ff72da1-7382-f5d2-eb1d-c99b46239d42)

#### `GetStringUserProperties(...)`

Retrieves all string properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/67120e84-c820-9d52-12ae-c167c595842c)

#### `GetUserProperty(String, Double.)(...)`

Retrieves a double property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/17cc8074-7955-32b5-2b4b-b169c45d4ee0)

#### `GetUserProperty(String, Int32.)(...)`

Retrieves an integer property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/d9364c1d-5b57-6a31-4c13-01124058cfb2)

#### `GetUserProperty(String, String.)(...)`

Retrieves a string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/214a886c-c18c-9b66-d7c9-fe84260962f2)

#### `Insert(...)`

Inserts the edge chamfer into the model database. All the attributes must be set.

[Docs](https://developer.tekla.com/topic/en/18/43/f3ac876a-95ee-05a1-5053-97077f2bbe35)

#### `Modify(...)`

Modifies the existing edge chamfer in the model database to match the current one.

[Docs](https://developer.tekla.com/topic/en/18/43/07af06b7-f3d6-a881-0284-36ca54229913)

#### `Select(...)`

Selects an edge chamfer from the model database. The edge chamfer ID must be set.

[Docs](https://developer.tekla.com/topic/en/18/43/89a09a62-2b25-f15d-101d-342d5b6f3e72)

#### `SetCustomObjectType(...)`

Sets a custom object type name for an existing object.

[Docs](https://developer.tekla.com/topic/en/18/43/3d6fb91b-48a9-35ca-d498-526514344089)

#### `SetDynamicStringProperty(...)`

Sets a dynamic string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/4dab0254-09e5-5e61-c28b-1c12afab71a0)

#### `SetLabel(...)`

Sets a label for an object when a new instance is created, this method must be called before Insert. The label is used in plug-ins for identifying the changed object in modification.

[Docs](https://developer.tekla.com/topic/en/18/43/3efcfc44-613a-e66b-9ebd-9f86df4f4036)

#### `SetPhase(...)`

Sets the phase of the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/ed80abcc-0ff7-d081-2229-e651f858a727)

#### `SetUserProperties(...)`

Sets multiple properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/8659da37-d571-f03f-c666-5c9fa3b113f5)

#### `SetUserProperty(String, Double)(...)`

Sets a double property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/5c585b8b-5c2e-7b75-2242-758f17429396)

#### `SetUserProperty(String, Int32)(...)`

Sets an integer property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/dd5b828d-3a87-bb58-5ac9-e1c3b4c2fe4b)

#### `SetUserProperty(String, String)(...)`

Sets a string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/80bf0348-b651-3d4a-862b-49c85bc924ab)

### Properties
- `Chamfer` (object, get/set) — The chamfer values for the EdgeChamfer.
- `Father` (object, get/set) — The father object of the boolean operation; the model object instance to operate on.
- `FirstBevelDimension` (object, get/set) — The bevel dimension if the first end is bevelled.
- `FirstChamferEndType` (object, get/set) — The ChamferEndTypeEnum of the first end.
- `FirstEnd` (object, get/set) — The start point of the EdgeChamfer.
- `Identifier` (object, get/set) — The identifier of the object.
- `IsUpToDate` (object, get/set) — Gets if the object does not have a modification which is not shared.
- `ModificationTime` (object, get/set) — Gets latest time of the object was modified or created.
- `Name` (object, get/set) — The name of the EdgeChamfer instance.
- `SecondBevelDimension` (object, get/set) — The bevel dimension if the second end is bevelled.
- `SecondChamferEndType` (object, get/set) — The ChamferEndTypeEnum of the second end.
- `SecondEnd` (object, get/set) — The end point of the EdgeChamfer.

## EdgeChamfer.ChamferEndTypeEnum (enum)

The end type of the edge chamfer.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/bb703a65-2763-a1f7-5315-8973f2a01dd0)

### Values
- `FULL` = `0` — The chamfer continues to the end of the edge.
- `STRAIGHT` = `1` — The chamfer end is straight at the given point.
- `BEVELLED` = `2` — The chamfer end is bevelled at the given point.

## Events (class)

The Events class allows the user to register event listeners for model events.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/8718989a-ebeb-cbd0-b920-08fb7a8ad3e4)

### Constructors
- `Events(...)` — Creates an empty events instance.

### Methods
#### `Dispose(...)`

Disposes the instance.

[Docs](https://developer.tekla.com/topic/en/18/43/98294faa-f7c3-4f2b-fca7-1b1cc6d5e7c3)

#### `InitializeLifetimeService(...)`

Initializes the lifetime service.

[Docs](https://developer.tekla.com/topic/en/18/43/46f8cdad-822e-e7ea-d76a-f495a0c83fb3)

#### `OnInterrupted(...)`

Called when uesr interrupts.

[Docs](https://developer.tekla.com/topic/en/18/43/5228be94-0f54-214b-47eb-60b574f92c9c)

#### `OnModelSaveInfo(...)`

Called when model is saved.

[Docs](https://developer.tekla.com/topic/en/18/43/bed20299-72dc-0d5f-6325-1ba70626082f)

#### `OnUndoClicked(...)`

Called when user clicks undo.

[Docs](https://developer.tekla.com/topic/en/18/43/4f8999bd-b4af-b02b-fd08-250ae8b0a4a7)

#### `Register(...)`

Registers the instance to listen to the specified events. More event delegates should not be added without calling UnRegister first.

[Docs](https://developer.tekla.com/topic/en/18/43/e324b229-c0fc-ee66-a7f9-5277a17c0858)

#### `UnRegister(...)`

Unregisters the instance from listening to events.

[Docs](https://developer.tekla.com/topic/en/18/43/89ad49e2-c123-7b99-5484-b4efe9a1dbde)

### Events
#### `AnnotationSelectionChange` (`EventHandler`)

**Signature:** `event EventHandler AnnotationSelectionChange`

The SelectionChange event is raised when the user changes the current selection of an annotation inside Tekla Structures.

[Docs](https://developer.tekla.com/topic/en/18/43/f799ee63-e2d3-224e-b824-b2c96c76d405)

#### `ClashCheckDone` (`EventHandler`)

**Signature:** `event EventHandler ClashCheckDone`

The ClashCheckDone event is raised just after clash check is completed.

[Docs](https://developer.tekla.com/topic/en/18/43/97cb0c22-0ee7-37d5-7908-c1fc2f036ede)

#### `ClashDetected` (`EventHandler`)

**Signature:** `event EventHandler ClashDetected`

The ClashDetected event is raised just after the detection of each clash. The clash information is saved before the event is triggered.

[Docs](https://developer.tekla.com/topic/en/18/43/ffc4e371-805a-71c7-2486-4eccb4b2365d)

#### `ClipPlaneChanged` (`EventHandler`)

**Signature:** `event EventHandler ClipPlaneChanged`

Occurs when user deletes a clip plane.

[Docs](https://developer.tekla.com/topic/en/18/43/721df2a3-9657-41e7-7c48-1cf0a7b93dd7)

#### `CommandStatusChange` (`EventHandler`)

**Signature:** `event EventHandler CommandStatusChange`

The CommandStatusChange event is raised when a command is started/ended.

[Docs](https://developer.tekla.com/topic/en/18/43/0cd7e991-c25d-a837-a622-e21c45c7708d)

#### `HiddenObjectsChanged` (`EventHandler`)

**Signature:** `event EventHandler HiddenObjectsChanged`

Occurs when user hides an object or view is redrawn

[Docs](https://developer.tekla.com/topic/en/18/43/c3b53935-2927-bd06-0052-dde7c01c71ea)

#### `Interrupted` (`EventHandler`)

**Signature:** `event EventHandler Interrupted`

Occurs when user interrupts.

[Docs](https://developer.tekla.com/topic/en/18/43/1e0d303b-03bc-32a3-19a0-2569b7f9c8fd)

#### `ModelChanged` (`EventHandler`)

**Signature:** `event EventHandler ModelChanged`

The ModelChanged event is raised just after some changes have been made to the model. Note, this event is however not triggered when UNDO or REDO are performed.

[Docs](https://developer.tekla.com/topic/en/18/43/89629a3e-cdc4-fb06-b16b-8f5d9c1e3e03)

#### `ModelLoad` (`EventHandler`)

**Signature:** `event EventHandler ModelLoad`

The ModelLoad event is raised just after a model has been loaded.

[Docs](https://developer.tekla.com/topic/en/18/43/99d9f5c4-b182-f022-ef7d-3dd16b9fbae3)

#### `ModelLoadInfo` (`EventHandler`)

**Signature:** `event EventHandler ModelLoadInfo`

The ModelLoadInfo event is raised just after a model has been loaded with the model load information as a string parameter.

[Docs](https://developer.tekla.com/topic/en/18/43/7d76d21d-6e26-fe29-cb58-5ca31f59fed2)

#### `ModelObjectChanged` (`EventHandler`)

**Signature:** `event EventHandler ModelObjectChanged`

The ModelObjectChanged event is raised just after some changes have been made to the model objects. Note, this event is triggered when UNDO or REDO are performed!!

[Docs](https://developer.tekla.com/topic/en/18/43/17f71c02-7595-85e1-ae0f-0636d4fd1072)

#### `ModelObjectNumbered` (`EventHandler`)

**Signature:** `event EventHandler ModelObjectNumbered`

The ModelObjectNumbered event is raised when model object is numbered.

[Docs](https://developer.tekla.com/topic/en/18/43/dd75c7d0-ee11-6903-9d2c-90c3001823e6)

#### `ModelSave` (`EventHandler`)

**Signature:** `event EventHandler ModelSave`

The ModelSave event is raised just after a model has been saved.

[Docs](https://developer.tekla.com/topic/en/18/43/104ed49c-cd7f-62af-c9a8-30d24863879a)

#### `ModelSaveAs` (`EventHandler`)

**Signature:** `event EventHandler ModelSaveAs`

The ModelSaveAs event is raised just after a model has been saved using save as.

[Docs](https://developer.tekla.com/topic/en/18/43/0f11e599-85e9-5710-e19d-6da7c9dc72eb)

#### `ModelSaveInfo` (`EventHandler`)

**Signature:** `event EventHandler ModelSaveInfo`

The ModelSave event is raised just after a model has been saved with the model save information as a string parameter.

[Docs](https://developer.tekla.com/topic/en/18/43/99e7c0e7-8836-e54f-bf97-565f74542a29)

#### `ModelUnloading` (`EventHandler`)

**Signature:** `event EventHandler ModelUnloading`

The ModelUnloading event is raised just before the model is unloaded.

[Docs](https://developer.tekla.com/topic/en/18/43/3e887bc9-d6ca-66b0-4a5e-cdde949f19c9)

#### `ModelUnloadingSync` (`EventHandler`)

**Signature:** `event EventHandler ModelUnloadingSync`

The ModelUnloadingSync event is raised synchronously on the main thread just before the model is unloaded. Use this event only when core resources need be cleaned up before the model unloading proceeds. All UI-only related resource cleanup should be done in the async ModelUnloading event. OnModelUnloadingSync(String, .Object.)

[Docs](https://developer.tekla.com/topic/en/18/43/03f82a07-1d6c-a7df-323c-311d71808b8a)

#### `Numbering` (`EventHandler`)

**Signature:** `event EventHandler Numbering`

The Numbering event is raised just after a model has been numbered.

[Docs](https://developer.tekla.com/topic/en/18/43/7b8f33de-a5eb-7ae5-98c3-124486c7d38e)

#### `ProjectInfoChanged` (`EventHandler`)

**Signature:** `event EventHandler ProjectInfoChanged`

The ProjectInfoChanged event is raised just after some changes have been made to the project info. Note, this event is however not triggered when UNDO or REDO are performed.

[Docs](https://developer.tekla.com/topic/en/18/43/23fbc11c-197c-a56e-e715-20787b29ea6b)

#### `SelectionChange` (`EventHandler`)

**Signature:** `event EventHandler SelectionChange`

The SelectionChange event is raised when the user changes the current selection inside Tekla Structures.

[Docs](https://developer.tekla.com/topic/en/18/43/27f86a1c-8d6f-89db-e24b-403ee8d7461f)

#### `TeklaStructuresExit` (`EventHandler`)

**Signature:** `event EventHandler TeklaStructuresExit`

The TeklaStructuresExit event is raised just before Tekla Structures exits. After this event has been called the user will no longer be able to perform any calls to Tekla Structures.

[Docs](https://developer.tekla.com/topic/en/18/43/7ed403c9-e659-d8df-dbc8-b028c59ecfa2)

#### `TemporaryStatesChanged` (`EventHandler`)

**Signature:** `event EventHandler TemporaryStatesChanged`

Occurs when temporary states are changed.

[Docs](https://developer.tekla.com/topic/en/18/43/8cd318d9-780d-6b36-53f0-314840e2fcad)

#### `UndoClicked` (`EventHandler`)

**Signature:** `event EventHandler UndoClicked`

Occurs when user clicks undo.

[Docs](https://developer.tekla.com/topic/en/18/43/ad04c5ac-2094-95cf-2ca4-082797bc7850)

#### `ViewCameraChanged` (`EventHandler`)

**Signature:** `event EventHandler ViewCameraChanged`

Occurs when view camera is changed.

[Docs](https://developer.tekla.com/topic/en/18/43/019d91d7-f1b7-54a7-48d9-6a5c6ba428a7)

#### `ViewClosed` (`EventHandler`)

**Signature:** `event EventHandler ViewClosed`

Occurs when view is closed.

[Docs](https://developer.tekla.com/topic/en/18/43/b0716378-227a-dc10-bca8-f5080354050d)

## Events.ClashCheckDoneDelegate (delegate)

The delegate to use for clash check completion.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/7beeecd1-ba63-d100-5429-958333a736e9)

## Events.ClashDetectedDelegate (delegate)

The delegate to use for clash detection. Encapsulates any method that takes a clash check data object as a parameter.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/3a5c3f17-6601-d4e1-6e10-5358c1978eb6)

## Events.ClipPlaneChangedDelegate (delegate)

The delegate to use for clip plane events.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/44f3eba6-9931-7ac8-c2dc-947c5f894488)

## Events.CommandStatusChangeDelegate (delegate)

The delegate to use for command status change.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/a18d14a2-bf19-91d7-d00f-e5815b76e55f)

## Events.HiddenObjectsChangedDelegate (delegate)

The delegate to use when objects are hidden or view is redrawn

[Vendor docs](https://developer.tekla.com/topic/en/18/43/4625ea6b-72a9-6f0d-7bb0-a8971b1e7c7c)

## Events.InterruptedDelegate (delegate)

The delegate to use for interrupted event

[Vendor docs](https://developer.tekla.com/topic/en/18/43/055e98a3-bab4-394b-2407-c89799aec5d4)

## Events.ModelChangedDelegate (delegate)

The delegate to use for database commit.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/da2bba46-3d2c-0843-4e56-750a766f331c)

## Events.ModelLoadDelegate (delegate)

The delegate to use for model load.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/5ff47b5e-6907-32b6-2927-2d88b765bb73)

## Events.ModelLoadInfoDelegate (delegate)

The delegate to use for extended model load.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/aecd1bab-ce8a-ec50-1752-8338234d5baa)

## Events.ModelObjectChangedDelegate (delegate)

The delegate to use for changed model objects. ModelObject inside ChangeData contains only identifier information. If properties need to be checked, Select() for object is required.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/0fb5c5db-fdf7-5bf8-6b41-dad52cb01933)

## Events.ModelObjectNumberedDelegate (delegate)

The delegate to use for model objects which are numbered. ModelObjects contain only identifier information. If properties need to be checked, Select() for object is required.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/845f67a0-ae66-a0e9-f511-a7bf3fd6898a)

## Events.ModelSaveAsDelegate (delegate)

The delegate to use for model save as.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/1d7ece5c-9948-e002-d6f9-f171c73b88a1)

## Events.ModelSaveDelegate (delegate)

The delegate to use for model save.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/145a428e-581b-8c98-df8f-2bee359d9595)

## Events.ModelSaveInfoDelegate (delegate)

The delegate to use for model save with context information.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/bf917674-1611-c405-e798-55e4d1afa4e9)

## Events.NumberingDelegate (delegate)

The delegate to use for numbering.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/e7ad68f1-6cba-7616-c9c7-ffe1140a0f13)

## Events.ProjectInfoChangedDelegate (delegate)

The delegate to use for change in project info.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/f7c4a1ff-5621-bb91-19d9-dc984645810f)

## Events.SelectionChangeDelegate (delegate)

The delegate to use for selection change.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/036c3eef-bcb7-3a75-86bf-3854c482d889)

## Events.TeklaStructuresExitDelegate (delegate)

The delegate to use for Tekla Structures exit.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/6adc90d0-84db-6db6-d8d3-fffbd87aaca8)

## Events.TemporaryStatesChangedDelegate (delegate)

The delegate to use when objects temporary states are changed.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/71901e95-3384-d85e-852f-352fe6639b50)

## Events.UndoClickedDelegate (delegate)

The delegate to use for undo clicked event

[Vendor docs](https://developer.tekla.com/topic/en/18/43/6fa8018b-5e6b-55b2-489e-1a216a2538e0)

## Events.ViewCameraChangedDelegate (delegate)

The delegate to use for view camera changed event

[Vendor docs](https://developer.tekla.com/topic/en/18/43/42ddeb84-7691-757c-8885-2d307dfa0933)

## Events.ViewClosedDelegate (delegate)

The delegate to use for view closed event.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/400204c0-a733-8832-fe35-8963bf394fa0)

## ExtensionIntersectsWithPlateException (class)

The ExtensionIntersectsWithPlateException class represents an error when extension plate clashes with plates.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/1e7f5097-9730-c720-48e0-e816e25bef98)

### Constructors
- `ExtensionIntersectsWithPlateException(...)` — Initializes a new instance of the ExtensionIntersectsWithPlateException class with "extension plate clashes with plates" error message.

## FacePerpendicularToIntersectionLineException (class)

The FacePerpendicularToIntersectionLineException class represents an error when, face perpendicular to intersection line.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/935fde75-f2c5-1d01-623f-d7a6b4c48c34)

### Constructors
- `FacePerpendicularToIntersectionLineException(...)` — Initializes a new instance of the FacePerpendicularToIntersectionLineException class with "face perpendicular to intersection line" error message.

## FacesAtAnObtuseAngleException (class)

The FacesAtAnObtuseAngleException class represents an error when, faces to be connected are too steep of an angle.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/5a2bc09b-f29d-62ef-c502-50bfc1f8aa86)

### Constructors
- `FacesAtAnObtuseAngleException(...)` — Initializes a new instance of the FacesAtAnObtuseAngleException class with "Faces to be connected are too steep of an angle" error message.

## FacesTooNearEachOtherException (class)

The FacesTooNearEachOtherException class represents an error when, faces to be connected overlap.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/3e31a78f-75c9-796d-7463-c0d60e28e905)

### Constructors
- `FacesTooNearEachOtherException(...)` — Initializes a new instance of the ExtensionIntersectsWithPlateException class with "faces to be connected overlap" error message.

## Fitting (class)

The Fitting class defines how the part end fits to a plane. A fitting can make the part either longer or shorter.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/9bd2b3ed-c087-cb0c-f1ed-2c11a3e4c533)

### Constructors
- `Fitting(...)` — Initializes a new instance of the Fitting class with empty attributes.

### Methods
#### `CompareTo(...)`

Compares Identifiers of model objects.

[Docs](https://developer.tekla.com/topic/en/18/43/79cca356-0c8a-e8fb-463a-34ad4141bec7)

#### `Delete(...)`

Deletes the fitting instance with the given ID from the model database.

[Docs](https://developer.tekla.com/topic/en/18/43/bb537d5c-2a27-22da-ed13-b5287ec63022)

#### `Equals(...)`

Check if Identifiers of model objects are same.

[Docs](https://developer.tekla.com/topic/en/18/43/e4324265-2490-00de-4139-63a8f7acb492)

#### `GetAllReportProperties(...)`

Retrieves all properties for the model object in one hashtable. Type for the returned value must be checked using type casting.

[Docs](https://developer.tekla.com/topic/en/18/43/0708f404-ce77-28e1-4ebe-fa906f68bff8)

#### `GetAllUserProperties(...)`

Retrieves all properties for the model object in one hashtable. Type for the returned value must be checked using type casting.

[Docs](https://developer.tekla.com/topic/en/18/43/63b853a0-0af5-af9c-3ce7-05299664d7f8)

#### `GetChildren(...)`

Returns an enumerator of all the children model objects.

[Docs](https://developer.tekla.com/topic/en/18/43/ce51ec40-310c-6067-a89f-f8d029aab747)

#### `GetCoordinateSystem(...)`

Returns the coordinate system for the given model object.

[Docs](https://developer.tekla.com/topic/en/18/43/8adfe9cd-f6e5-6474-1297-33c6270a7b0a)

#### `GetCustomObjectType(...)`

Gets custom object type name from an existing object.

[Docs](https://developer.tekla.com/topic/en/18/43/d3be2ba3-8533-7165-f2d3-a5aa9df556b8)

#### `GetDoubleReportProperties(...)`

Retrieves multiple double report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/f86f6c96-f738-49ca-16ba-f5634e862ba2)

#### `GetDoubleUserProperties(...)`

Retrieves all double properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/6ead7a63-5f76-0631-d480-d46c27e882aa)

#### `GetDynamicStringProperty(...)`

Gets a dynamic string property from the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/415e86d9-7de6-ea53-ce68-20b3e67b8655)

#### `GetFatherComponent(...)`

Returns the father component of the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/a2c28f3f-52ff-50a8-1b32-10d18db5c31d)

#### `GetHierarchicObjects(...)`

Returns an enumerator of all the connected hierarchic objects.

[Docs](https://developer.tekla.com/topic/en/18/43/cc228ea0-267b-7d90-7441-c9efc1779b08)

#### `GetIntegerReportProperties(...)`

Retrieves multiple integer report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/2623a9c0-2a9c-e233-9daa-59f92ed51445)

#### `GetIntegerUserProperties(...)`

Retrieves all integer properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/1b3c6f06-bc25-a54e-2a88-b60dc4a3c85e)

#### `GetPhase(...)`

Retrieves the phase of the model object (the phase number, the phase name, the phase comment and whether the phase is the current one or not).

[Docs](https://developer.tekla.com/topic/en/18/43/310f8b7a-c120-753a-35c9-f7c73a9806c9)

#### `GetReportProperty(String, Double.)(...)`

Retrieves a double property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/1c05aa56-5f42-be91-a59e-8d59a5549f45)

#### `GetReportProperty(String, Int32.)(...)`

Retrieves an integer property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/ce8e4ca0-3750-3c9b-5ca8-27e48b45e0bd)

#### `GetReportProperty(String, String.)(...)`

Retrieves a string property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/70f212bb-ee70-e9e9-7ab6-d6a9d46f8de6)

#### `GetStringReportProperties(...)`

Retrieves multiple string report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/5ff72da1-7382-f5d2-eb1d-c99b46239d42)

#### `GetStringUserProperties(...)`

Retrieves all string properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/67120e84-c820-9d52-12ae-c167c595842c)

#### `GetUserProperty(String, Double.)(...)`

Retrieves a double property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/17cc8074-7955-32b5-2b4b-b169c45d4ee0)

#### `GetUserProperty(String, Int32.)(...)`

Retrieves an integer property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/d9364c1d-5b57-6a31-4c13-01124058cfb2)

#### `GetUserProperty(String, String.)(...)`

Retrieves a string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/214a886c-c18c-9b66-d7c9-fe84260962f2)

#### `Insert(...)`

Inserts the fitting into the model database. All the attributes must be set.

[Docs](https://developer.tekla.com/topic/en/18/43/1cfaae52-4503-d6d1-a472-f41b371cb64c)

#### `Modify(...)`

Modifies the existing fitting in the model database to match the current one.

[Docs](https://developer.tekla.com/topic/en/18/43/7d55da03-10b0-3726-0479-d8eda7d50f6e)

#### `Select(...)`

Selects a fitting from the model database. The fitting ID must be set.

[Docs](https://developer.tekla.com/topic/en/18/43/94bf1da2-b4b7-c792-49c6-cc5dfd8b5b2f)

#### `SetCustomObjectType(...)`

Sets a custom object type name for an existing object.

[Docs](https://developer.tekla.com/topic/en/18/43/3d6fb91b-48a9-35ca-d498-526514344089)

#### `SetDynamicStringProperty(...)`

Sets a dynamic string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/4dab0254-09e5-5e61-c28b-1c12afab71a0)

#### `SetLabel(...)`

Sets a label for an object when a new instance is created, this method must be called before Insert. The label is used in plug-ins for identifying the changed object in modification.

[Docs](https://developer.tekla.com/topic/en/18/43/3efcfc44-613a-e66b-9ebd-9f86df4f4036)

#### `SetPhase(...)`

Sets the phase of the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/ed80abcc-0ff7-d081-2229-e651f858a727)

#### `SetUserProperties(...)`

Sets multiple properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/8659da37-d571-f03f-c666-5c9fa3b113f5)

#### `SetUserProperty(String, Double)(...)`

Sets a double property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/5c585b8b-5c2e-7b75-2242-758f17429396)

#### `SetUserProperty(String, Int32)(...)`

Sets an integer property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/dd5b828d-3a87-bb58-5ac9-e1c3b4c2fe4b)

#### `SetUserProperty(String, String)(...)`

Sets a string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/80bf0348-b651-3d4a-862b-49c85bc924ab)

### Properties
- `Father` (object, get/set) — The father object of the boolean operation; the model object instance to operate on.
- `Identifier` (object, get/set) — The identifier of the object.
- `IsUpToDate` (object, get/set) — Gets if the object does not have a modification which is not shared.
- `ModificationTime` (object, get/set) — Gets latest time of the object was modified or created.
- `Plane` (object, get/set) — The plane that fits the part.

## GeneralConnectiveGeometryException (class)

The GeneralConnectiveGeometryException class represents a general error that occurred during the ConnectiveGeometry creation or modification.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/9a296802-ce5e-ef31-9f8a-37c708592065)

### Constructors
- `GeneralConnectiveGeometryException(...)` — Initializes a new instance of the GeneralConnectiveGeometryException class with "Failed to create or modify connective geometry" message.

## GeometrySection (class)

The GeometrySection class represents geometry node with its identifier.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/ebc2368c-cb3d-5533-8e3e-af3d02c7466d)

### Properties
- `GeometryNode` (object, get/set) — Gets the geometry node
- `Index` (object, get/set) — Gets the index of the node

## GeometrySectionEnumerator (class)

The GeometrySectionEnumerator class is used to enumerate the geometry sections of a bent plate geometry.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/fabaac2a-bb59-e3d1-b60d-38f638e7bf6e)

### Methods
#### `MoveNext(...)`

Moves to the next item in the enumerator.

[Docs](https://developer.tekla.com/topic/en/18/43/e3a5139a-f5d1-c283-d58a-21d2403b9624)

#### `Reset(...)`

Resets the enumerator to the beginning.

[Docs](https://developer.tekla.com/topic/en/18/43/2735ca0c-35d3-991b-c53b-d3ea666301b8)

### Properties
- `Current` (object, get/set) — Gets the current geometry section.

## Grid (class)

The Grid class defines a user defined (possibly magnetic) set of planes helping in modeling work. A grid has grid plane instances as children.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/82142833-2bb4-61e8-88d3-e8b43a38ade2)

### Constructors
- `Grid(...)` — Creates a new grid instance. The default values are as follows: Name = "Grid";CoordinateX = "0.00 4*3000.00";CoordinateY = "0.00 5*6000.00";CoordinateZ = "0.00 6000.00 8000.00 9000.00";LabelX = "A B C D E";LabelY = "1 2 3 4 5 6";LabelZ = "+0 +6000 +8000 +9000";ExtensionLeftX = 2000.0;ExtensionLeftY = 2000.0;ExtensionLeftZ = 2000.0;ExtensionRightX = 2000.0;ExtensionRightY = 2000.0;ExtensionRightZ = 2000.0;IsMagnetic = false;

### Methods
#### `CompareTo(...)`

Compares Identifiers of model objects.

[Docs](https://developer.tekla.com/topic/en/18/43/79cca356-0c8a-e8fb-463a-34ad4141bec7)

#### `Delete(...)`

Deletes the grid with the given ID.

[Docs](https://developer.tekla.com/topic/en/18/43/c1feb648-a138-5987-7bce-1375f684653a)

#### `Equals(...)`

Check if Identifiers of model objects are same.

[Docs](https://developer.tekla.com/topic/en/18/43/e4324265-2490-00de-4139-63a8f7acb492)

#### `GetAllReportProperties(...)`

Retrieves all properties for the model object in one hashtable. Type for the returned value must be checked using type casting.

[Docs](https://developer.tekla.com/topic/en/18/43/0708f404-ce77-28e1-4ebe-fa906f68bff8)

#### `GetAllUserProperties(...)`

Retrieves all properties for the model object in one hashtable. Type for the returned value must be checked using type casting.

[Docs](https://developer.tekla.com/topic/en/18/43/63b853a0-0af5-af9c-3ce7-05299664d7f8)

#### `GetChildren(...)`

Returns an enumerator of all the children model objects.

[Docs](https://developer.tekla.com/topic/en/18/43/ce51ec40-310c-6067-a89f-f8d029aab747)

#### `GetCoordinateSystem(...)`

Returns the coordinate system for the given model object.

[Docs](https://developer.tekla.com/topic/en/18/43/8adfe9cd-f6e5-6474-1297-33c6270a7b0a)

#### `GetCustomObjectType(...)`

Gets custom object type name from an existing object.

[Docs](https://developer.tekla.com/topic/en/18/43/d3be2ba3-8533-7165-f2d3-a5aa9df556b8)

#### `GetDoubleReportProperties(...)`

Retrieves multiple double report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/f86f6c96-f738-49ca-16ba-f5634e862ba2)

#### `GetDoubleUserProperties(...)`

Retrieves all double properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/6ead7a63-5f76-0631-d480-d46c27e882aa)

#### `GetDynamicStringProperty(...)`

Gets a dynamic string property from the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/415e86d9-7de6-ea53-ce68-20b3e67b8655)

#### `GetFatherComponent(...)`

Returns the father component of the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/a2c28f3f-52ff-50a8-1b32-10d18db5c31d)

#### `GetHierarchicObjects(...)`

Returns an enumerator of all the connected hierarchic objects.

[Docs](https://developer.tekla.com/topic/en/18/43/cc228ea0-267b-7d90-7441-c9efc1779b08)

#### `GetIntegerReportProperties(...)`

Retrieves multiple integer report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/2623a9c0-2a9c-e233-9daa-59f92ed51445)

#### `GetIntegerUserProperties(...)`

Retrieves all integer properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/1b3c6f06-bc25-a54e-2a88-b60dc4a3c85e)

#### `GetPhase(...)`

Retrieves the phase of the model object (the phase number, the phase name, the phase comment and whether the phase is the current one or not).

[Docs](https://developer.tekla.com/topic/en/18/43/310f8b7a-c120-753a-35c9-f7c73a9806c9)

#### `GetReportProperty(String, Double.)(...)`

Retrieves a double property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/1c05aa56-5f42-be91-a59e-8d59a5549f45)

#### `GetReportProperty(String, Int32.)(...)`

Retrieves an integer property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/ce8e4ca0-3750-3c9b-5ca8-27e48b45e0bd)

#### `GetReportProperty(String, String.)(...)`

Retrieves a string property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/70f212bb-ee70-e9e9-7ab6-d6a9d46f8de6)

#### `GetStringReportProperties(...)`

Retrieves multiple string report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/5ff72da1-7382-f5d2-eb1d-c99b46239d42)

#### `GetStringUserProperties(...)`

Retrieves all string properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/67120e84-c820-9d52-12ae-c167c595842c)

#### `GetUserProperty(String, Double.)(...)`

Retrieves a double property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/17cc8074-7955-32b5-2b4b-b169c45d4ee0)

#### `GetUserProperty(String, Int32.)(...)`

Retrieves an integer property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/d9364c1d-5b57-6a31-4c13-01124058cfb2)

#### `GetUserProperty(String, String.)(...)`

Retrieves a string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/214a886c-c18c-9b66-d7c9-fe84260962f2)

#### `Insert(...)`

Inserts the grid into the model.

[Docs](https://developer.tekla.com/topic/en/18/43/ee9fce8c-a744-1147-a10e-be0e5a353436)

#### `Modify(...)`

Modifies the grid with the given ID.

[Docs](https://developer.tekla.com/topic/en/18/43/1ac56a24-5d36-2715-7dcc-9101041de4e9)

#### `Select(...)`

Selects the grid with the given ID.

[Docs](https://developer.tekla.com/topic/en/18/43/4a5978f8-bc16-99b9-be9a-69f02353d725)

#### `SetCustomObjectType(...)`

Sets a custom object type name for an existing object.

[Docs](https://developer.tekla.com/topic/en/18/43/3d6fb91b-48a9-35ca-d498-526514344089)

#### `SetDynamicStringProperty(...)`

Sets a dynamic string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/4dab0254-09e5-5e61-c28b-1c12afab71a0)

#### `SetLabel(...)`

Sets a label for an object when a new instance is created, this method must be called before Insert. The label is used in plug-ins for identifying the changed object in modification.

[Docs](https://developer.tekla.com/topic/en/18/43/3efcfc44-613a-e66b-9ebd-9f86df4f4036)

#### `SetPhase(...)`

Sets the phase of the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/ed80abcc-0ff7-d081-2229-e651f858a727)

#### `SetUserProperties(...)`

Sets multiple properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/8659da37-d571-f03f-c666-5c9fa3b113f5)

#### `SetUserProperty(String, Double)(...)`

Sets a double property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/5c585b8b-5c2e-7b75-2242-758f17429396)

#### `SetUserProperty(String, Int32)(...)`

Sets an integer property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/dd5b828d-3a87-bb58-5ac9-e1c3b4c2fe4b)

#### `SetUserProperty(String, String)(...)`

Sets a string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/80bf0348-b651-3d4a-862b-49c85bc924ab)

### Properties
- `Color` (object, get/set) — Gets or sets the color of the grid.
- `CoordinateX` (object, get/set) — The coordinates for the X-axis.
- `CoordinateY` (object, get/set) — The coordinates for the Y-axis.
- `CoordinateZ` (object, get/set) — The coordinates for the Z-axis.
- `ExtensionForMagneticArea` (object, get/set) — The extension for the magnetic area.
- `ExtensionLeftX` (object, get/set) — The extension to the left X-axis.
- `ExtensionLeftY` (object, get/set) — The extension to the left Y-axis.
- `ExtensionLeftZ` (object, get/set) — The extension to the left Z-axis.
- `ExtensionRightX` (object, get/set) — The extension to the right X-axis.
- `ExtensionRightY` (object, get/set) — The extension to the right Y-axis.
- `ExtensionRightZ` (object, get/set) — The extension to the right Z-axis.
- `FontColor` (object, get/set) — Gets or sets font color for the grid labels
- `FontSize` (object, get/set) — Gets or sets font size for the grid labels
- `Identifier` (object, get/set) — The identifier of the object.
- `IsMagnetic` (object, get/set) — Gets or sets whether the grid is magnetic or not.
- `IsUpToDate` (object, get/set) — Gets if the object does not have a modification which is not shared.
- `LabelX` (object, get/set) — The labels for the X-axis.
- `LabelY` (object, get/set) — The labels for the Y-axis.
- `LabelZ` (object, get/set) — The labels for the Z-axis.
- `ModificationTime` (object, get/set) — Gets latest time of the object was modified or created.
- `Name` (object, get/set) — Gets or sets the name of the grid.
- `Origin` (object, get/set) — Gets or sets the origin of the grid

## GridBase (class)

The GridBase class defines the base class for grids.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/6cd5d756-5f8e-15d5-37b7-728c940c30ed)

### Constructors
- `GridBase(...)` — Creates a new grid instance. The default values are as follows: Name = "Grid";CoordinateX = "0.00 4*3000.00";CoordinateY = "0.00 5*6000.00";CoordinateZ = "0.00 6000.00 8000.00 9000.00";LabelX = "A B C D E";LabelY = "1 2 3 4 5 6";LabelZ = "+0 +6000 +8000 +9000";ExtensionLeftX = 2000.0;ExtensionLeftY = 2000.0;ExtensionLeftZ = 2000.0;ExtensionRightX = 2000.0;ExtensionRightY = 2000.0;ExtensionRightZ = 2000.0;IsMagnetic = false;

### Methods
#### `CompareTo(...)`

Compares Identifiers of model objects.

[Docs](https://developer.tekla.com/topic/en/18/43/79cca356-0c8a-e8fb-463a-34ad4141bec7)

#### `Delete(...)`

Deletes the grid with the given ID.

[Docs](https://developer.tekla.com/topic/en/18/43/c1feb648-a138-5987-7bce-1375f684653a)

#### `Equals(...)`

Check if Identifiers of model objects are same.

[Docs](https://developer.tekla.com/topic/en/18/43/e4324265-2490-00de-4139-63a8f7acb492)

#### `GetAllReportProperties(...)`

Retrieves all properties for the model object in one hashtable. Type for the returned value must be checked using type casting.

[Docs](https://developer.tekla.com/topic/en/18/43/0708f404-ce77-28e1-4ebe-fa906f68bff8)

#### `GetAllUserProperties(...)`

Retrieves all properties for the model object in one hashtable. Type for the returned value must be checked using type casting.

[Docs](https://developer.tekla.com/topic/en/18/43/63b853a0-0af5-af9c-3ce7-05299664d7f8)

#### `GetChildren(...)`

Returns an enumerator of all the children model objects.

[Docs](https://developer.tekla.com/topic/en/18/43/ce51ec40-310c-6067-a89f-f8d029aab747)

#### `GetCoordinateSystem(...)`

Returns the coordinate system for the given model object.

[Docs](https://developer.tekla.com/topic/en/18/43/8adfe9cd-f6e5-6474-1297-33c6270a7b0a)

#### `GetCustomObjectType(...)`

Gets custom object type name from an existing object.

[Docs](https://developer.tekla.com/topic/en/18/43/d3be2ba3-8533-7165-f2d3-a5aa9df556b8)

#### `GetDoubleReportProperties(...)`

Retrieves multiple double report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/f86f6c96-f738-49ca-16ba-f5634e862ba2)

#### `GetDoubleUserProperties(...)`

Retrieves all double properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/6ead7a63-5f76-0631-d480-d46c27e882aa)

#### `GetDynamicStringProperty(...)`

Gets a dynamic string property from the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/415e86d9-7de6-ea53-ce68-20b3e67b8655)

#### `GetFatherComponent(...)`

Returns the father component of the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/a2c28f3f-52ff-50a8-1b32-10d18db5c31d)

#### `GetHierarchicObjects(...)`

Returns an enumerator of all the connected hierarchic objects.

[Docs](https://developer.tekla.com/topic/en/18/43/cc228ea0-267b-7d90-7441-c9efc1779b08)

#### `GetIntegerReportProperties(...)`

Retrieves multiple integer report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/2623a9c0-2a9c-e233-9daa-59f92ed51445)

#### `GetIntegerUserProperties(...)`

Retrieves all integer properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/1b3c6f06-bc25-a54e-2a88-b60dc4a3c85e)

#### `GetPhase(...)`

Retrieves the phase of the model object (the phase number, the phase name, the phase comment and whether the phase is the current one or not).

[Docs](https://developer.tekla.com/topic/en/18/43/310f8b7a-c120-753a-35c9-f7c73a9806c9)

#### `GetReportProperty(String, Double.)(...)`

Retrieves a double property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/1c05aa56-5f42-be91-a59e-8d59a5549f45)

#### `GetReportProperty(String, Int32.)(...)`

Retrieves an integer property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/ce8e4ca0-3750-3c9b-5ca8-27e48b45e0bd)

#### `GetReportProperty(String, String.)(...)`

Retrieves a string property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/70f212bb-ee70-e9e9-7ab6-d6a9d46f8de6)

#### `GetStringReportProperties(...)`

Retrieves multiple string report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/5ff72da1-7382-f5d2-eb1d-c99b46239d42)

#### `GetStringUserProperties(...)`

Retrieves all string properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/67120e84-c820-9d52-12ae-c167c595842c)

#### `GetUserProperty(String, Double.)(...)`

Retrieves a double property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/17cc8074-7955-32b5-2b4b-b169c45d4ee0)

#### `GetUserProperty(String, Int32.)(...)`

Retrieves an integer property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/d9364c1d-5b57-6a31-4c13-01124058cfb2)

#### `GetUserProperty(String, String.)(...)`

Retrieves a string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/214a886c-c18c-9b66-d7c9-fe84260962f2)

#### `Insert(...)`

Inserts the grid into the model.

[Docs](https://developer.tekla.com/topic/en/18/43/ee9fce8c-a744-1147-a10e-be0e5a353436)

#### `Modify(...)`

Modifies the grid with the given ID.

[Docs](https://developer.tekla.com/topic/en/18/43/1ac56a24-5d36-2715-7dcc-9101041de4e9)

#### `Select(...)`

Selects the grid with the given ID.

[Docs](https://developer.tekla.com/topic/en/18/43/4a5978f8-bc16-99b9-be9a-69f02353d725)

#### `SetCustomObjectType(...)`

Sets a custom object type name for an existing object.

[Docs](https://developer.tekla.com/topic/en/18/43/3d6fb91b-48a9-35ca-d498-526514344089)

#### `SetDynamicStringProperty(...)`

Sets a dynamic string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/4dab0254-09e5-5e61-c28b-1c12afab71a0)

#### `SetLabel(...)`

Sets a label for an object when a new instance is created, this method must be called before Insert. The label is used in plug-ins for identifying the changed object in modification.

[Docs](https://developer.tekla.com/topic/en/18/43/3efcfc44-613a-e66b-9ebd-9f86df4f4036)

#### `SetPhase(...)`

Sets the phase of the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/ed80abcc-0ff7-d081-2229-e651f858a727)

#### `SetUserProperties(...)`

Sets multiple properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/8659da37-d571-f03f-c666-5c9fa3b113f5)

#### `SetUserProperty(String, Double)(...)`

Sets a double property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/5c585b8b-5c2e-7b75-2242-758f17429396)

#### `SetUserProperty(String, Int32)(...)`

Sets an integer property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/dd5b828d-3a87-bb58-5ac9-e1c3b4c2fe4b)

#### `SetUserProperty(String, String)(...)`

Sets a string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/80bf0348-b651-3d4a-862b-49c85bc924ab)

### Properties
- `FontColor` (object, get/set) — Gets or sets font color for the grid labels
- `FontSize` (object, get/set) — Gets or sets font size for the grid labels
- `Identifier` (object, get/set) — The identifier of the object.
- `IsMagnetic` (object, get/set) — Gets or sets whether the grid is magnetic or not.
- `IsUpToDate` (object, get/set) — Gets if the object does not have a modification which is not shared.
- `ModificationTime` (object, get/set) — Gets latest time of the object was modified or created.
- `Name` (object, get/set) — Gets or sets the name of the grid.
- `Origin` (object, get/set) — Gets or sets the origin of the grid

## GridCylindricalSurface (class)

The GridCylindricalSurface class defines a user defined (possibly magnetic) cylindrical surface helping in modeling work. A grid cylindrical surface must always belong to a parent grid instance.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/49862649-da7c-2c72-554e-78cc2a2d7f6b)

### Constructors
- `GridCylindricalSurface(...)` — Creates a new grid plane instance. The default values are as follows: CylinderBase = new Arc(new Point(0, 0, 0), new Point(6000, 0, 0), new Vector(0, 0, 1), Math.PI);CylinderHeight = 6000.0;Label = "Default";IsMagnetic = false;ExtensionLeft = 2000.0;ExtensionRight = 2000.0;ExtensionBelow = 2000.0;ExtensionAbove = 2000.0;DrawingVisibility = false;
- `GridCylindricalSurface(...)` — Creates a new grid plane instance taking the plane and label as input.

### Methods
#### `CompareTo(...)`

Compares Identifiers of model objects.

[Docs](https://developer.tekla.com/topic/en/18/43/79cca356-0c8a-e8fb-463a-34ad4141bec7)

#### `Delete(...)`

Deletes the grid surface with the given ID.

[Docs](https://developer.tekla.com/topic/en/18/43/8e5f9bdd-e3b2-0982-3a72-c9974947d0eb)

#### `Equals(...)`

Check if Identifiers of model objects are same.

[Docs](https://developer.tekla.com/topic/en/18/43/e4324265-2490-00de-4139-63a8f7acb492)

#### `GetAllReportProperties(...)`

Retrieves all properties for the model object in one hashtable. Type for the returned value must be checked using type casting.

[Docs](https://developer.tekla.com/topic/en/18/43/0708f404-ce77-28e1-4ebe-fa906f68bff8)

#### `GetAllUserProperties(...)`

Retrieves all properties for the model object in one hashtable. Type for the returned value must be checked using type casting.

[Docs](https://developer.tekla.com/topic/en/18/43/63b853a0-0af5-af9c-3ce7-05299664d7f8)

#### `GetChildren(...)`

Returns an enumerator of all the children model objects.

[Docs](https://developer.tekla.com/topic/en/18/43/ce51ec40-310c-6067-a89f-f8d029aab747)

#### `GetCoordinateSystem(...)`

Returns the coordinate system for the given model object.

[Docs](https://developer.tekla.com/topic/en/18/43/8adfe9cd-f6e5-6474-1297-33c6270a7b0a)

#### `GetCustomObjectType(...)`

Gets custom object type name from an existing object.

[Docs](https://developer.tekla.com/topic/en/18/43/d3be2ba3-8533-7165-f2d3-a5aa9df556b8)

#### `GetDoubleReportProperties(...)`

Retrieves multiple double report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/f86f6c96-f738-49ca-16ba-f5634e862ba2)

#### `GetDoubleUserProperties(...)`

Retrieves all double properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/6ead7a63-5f76-0631-d480-d46c27e882aa)

#### `GetDynamicStringProperty(...)`

Gets a dynamic string property from the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/415e86d9-7de6-ea53-ce68-20b3e67b8655)

#### `GetFatherComponent(...)`

Returns the father component of the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/a2c28f3f-52ff-50a8-1b32-10d18db5c31d)

#### `GetHierarchicObjects(...)`

Returns an enumerator of all the connected hierarchic objects.

[Docs](https://developer.tekla.com/topic/en/18/43/cc228ea0-267b-7d90-7441-c9efc1779b08)

#### `GetIntegerReportProperties(...)`

Retrieves multiple integer report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/2623a9c0-2a9c-e233-9daa-59f92ed51445)

#### `GetIntegerUserProperties(...)`

Retrieves all integer properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/1b3c6f06-bc25-a54e-2a88-b60dc4a3c85e)

#### `GetPhase(...)`

Retrieves the phase of the model object (the phase number, the phase name, the phase comment and whether the phase is the current one or not).

[Docs](https://developer.tekla.com/topic/en/18/43/310f8b7a-c120-753a-35c9-f7c73a9806c9)

#### `GetReportProperty(String, Double.)(...)`

Retrieves a double property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/1c05aa56-5f42-be91-a59e-8d59a5549f45)

#### `GetReportProperty(String, Int32.)(...)`

Retrieves an integer property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/ce8e4ca0-3750-3c9b-5ca8-27e48b45e0bd)

#### `GetReportProperty(String, String.)(...)`

Retrieves a string property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/70f212bb-ee70-e9e9-7ab6-d6a9d46f8de6)

#### `GetStringReportProperties(...)`

Retrieves multiple string report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/5ff72da1-7382-f5d2-eb1d-c99b46239d42)

#### `GetStringUserProperties(...)`

Retrieves all string properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/67120e84-c820-9d52-12ae-c167c595842c)

#### `GetUserProperty(String, Double.)(...)`

Retrieves a double property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/17cc8074-7955-32b5-2b4b-b169c45d4ee0)

#### `GetUserProperty(String, Int32.)(...)`

Retrieves an integer property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/d9364c1d-5b57-6a31-4c13-01124058cfb2)

#### `GetUserProperty(String, String.)(...)`

Retrieves a string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/214a886c-c18c-9b66-d7c9-fe84260962f2)

#### `Insert(...)`

Inserts the grid surface into the model.

[Docs](https://developer.tekla.com/topic/en/18/43/12a6eef8-ddab-096a-ecce-1f8950b897a5)

#### `Modify(...)`

Modifies the grid plane with the given ID.

[Docs](https://developer.tekla.com/topic/en/18/43/19a168a8-cfb6-7d50-8b54-1774a09a66c5)

#### `Select(...)`

Selects the grid surface with the given ID.

[Docs](https://developer.tekla.com/topic/en/18/43/e59d762e-e145-c980-9d49-9bc2c07cdd46)

#### `SetCustomObjectType(...)`

Sets a custom object type name for an existing object.

[Docs](https://developer.tekla.com/topic/en/18/43/3d6fb91b-48a9-35ca-d498-526514344089)

#### `SetDynamicStringProperty(...)`

Sets a dynamic string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/4dab0254-09e5-5e61-c28b-1c12afab71a0)

#### `SetLabel(...)`

Sets a label for an object when a new instance is created, this method must be called before Insert. The label is used in plug-ins for identifying the changed object in modification.

[Docs](https://developer.tekla.com/topic/en/18/43/3efcfc44-613a-e66b-9ebd-9f86df4f4036)

#### `SetPhase(...)`

Sets the phase of the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/ed80abcc-0ff7-d081-2229-e651f858a727)

#### `SetUserProperties(...)`

Sets multiple properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/8659da37-d571-f03f-c666-5c9fa3b113f5)

#### `SetUserProperty(String, Double)(...)`

Sets a double property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/5c585b8b-5c2e-7b75-2242-758f17429396)

#### `SetUserProperty(String, Int32)(...)`

Sets an integer property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/dd5b828d-3a87-bb58-5ac9-e1c3b4c2fe4b)

#### `SetUserProperty(String, String)(...)`

Sets a string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/80bf0348-b651-3d4a-862b-49c85bc924ab)

### Properties
- `CylinderBase` (object, get/set) — The arc that represents the base of the cylindrical surface. The cylinder is then extruded along the normal direction of the arc, with the height given by CylinderHeight"
- `CylinderHeight` (object, get/set) — Height of the cylinder along the normal direction of the CylinderBase arc.
- `DrawingVisibility` (object, get/set) — The visibility of the grid surface in drawings.
- `ExtensionAbove` (object, get/set) — The line extension above.
- `ExtensionBelow` (object, get/set) — The line extension below.
- `ExtensionLeft` (object, get/set) — The line extension on the left.
- `ExtensionRight` (object, get/set) — The line extension on the right.
- `Identifier` (object, get/set) — The identifier of the object.
- `IsMagnetic` (object, get/set) — Whether the grid surface is magnetic or not.
- `IsManual` (object, get/set) — Gets or sets if the line was manually created.
- `IsUpToDate` (object, get/set) — Gets if the object does not have a modification which is not shared.
- `Label` (object, get/set) — The label for the grid surface.
- `ModificationTime` (object, get/set) — Gets latest time of the object was modified or created.
- `Parent` (object, get/set) — The parent grid of the grid surface.

## GridPlane (class)

The GridPlane class defines a user defined (possibly magnetic) plane helping in modeling work. A grid plane must always belong to a father grid instance.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/d46eb5c5-4e0e-d205-47a5-61dd7d1bb1f9)

### Constructors
- `GridPlane(...)` — Creates a new grid plane instance. The default values are as follows: Plane = new Plane();Plane.Origin = new Point(500.0, 500.0, 0.0);Plane.AxisX = new Vector(2000.0, 0.0, 0.0);Plane.AxisY = new Vector(0.0, 0.0, 2000.0);Label = "Default";IsMagnetic = false;ExtensionLeft = 2000.0;ExtensionRight = 2000.0;ExtensionBelow = 2000.0;ExtensionAbove = 2000.0;DrawingVisibility = false;
- `GridPlane(...)` — Creates a new grid plane instance taking the plane and label as input.

### Methods
#### `CompareTo(...)`

Compares Identifiers of model objects.

[Docs](https://developer.tekla.com/topic/en/18/43/79cca356-0c8a-e8fb-463a-34ad4141bec7)

#### `Delete(...)`

Deletes the grid surface with the given ID.

[Docs](https://developer.tekla.com/topic/en/18/43/8bf0748d-5972-7268-dc71-fb6dd375088a)

#### `Equals(...)`

Check if Identifiers of model objects are same.

[Docs](https://developer.tekla.com/topic/en/18/43/e4324265-2490-00de-4139-63a8f7acb492)

#### `GetAllReportProperties(...)`

Retrieves all properties for the model object in one hashtable. Type for the returned value must be checked using type casting.

[Docs](https://developer.tekla.com/topic/en/18/43/0708f404-ce77-28e1-4ebe-fa906f68bff8)

#### `GetAllUserProperties(...)`

Retrieves all properties for the model object in one hashtable. Type for the returned value must be checked using type casting.

[Docs](https://developer.tekla.com/topic/en/18/43/63b853a0-0af5-af9c-3ce7-05299664d7f8)

#### `GetChildren(...)`

Returns an enumerator of all the children model objects.

[Docs](https://developer.tekla.com/topic/en/18/43/ce51ec40-310c-6067-a89f-f8d029aab747)

#### `GetCoordinateSystem(...)`

Returns the coordinate system for the given model object.

[Docs](https://developer.tekla.com/topic/en/18/43/8adfe9cd-f6e5-6474-1297-33c6270a7b0a)

#### `GetCustomObjectType(...)`

Gets custom object type name from an existing object.

[Docs](https://developer.tekla.com/topic/en/18/43/d3be2ba3-8533-7165-f2d3-a5aa9df556b8)

#### `GetDoubleReportProperties(...)`

Retrieves multiple double report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/f86f6c96-f738-49ca-16ba-f5634e862ba2)

#### `GetDoubleUserProperties(...)`

Retrieves all double properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/6ead7a63-5f76-0631-d480-d46c27e882aa)

#### `GetDynamicStringProperty(...)`

Gets a dynamic string property from the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/415e86d9-7de6-ea53-ce68-20b3e67b8655)

#### `GetFatherComponent(...)`

Returns the father component of the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/a2c28f3f-52ff-50a8-1b32-10d18db5c31d)

#### `GetHierarchicObjects(...)`

Returns an enumerator of all the connected hierarchic objects.

[Docs](https://developer.tekla.com/topic/en/18/43/cc228ea0-267b-7d90-7441-c9efc1779b08)

#### `GetIntegerReportProperties(...)`

Retrieves multiple integer report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/2623a9c0-2a9c-e233-9daa-59f92ed51445)

#### `GetIntegerUserProperties(...)`

Retrieves all integer properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/1b3c6f06-bc25-a54e-2a88-b60dc4a3c85e)

#### `GetPhase(...)`

Retrieves the phase of the model object (the phase number, the phase name, the phase comment and whether the phase is the current one or not).

[Docs](https://developer.tekla.com/topic/en/18/43/310f8b7a-c120-753a-35c9-f7c73a9806c9)

#### `GetReportProperty(String, Double.)(...)`

Retrieves a double property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/1c05aa56-5f42-be91-a59e-8d59a5549f45)

#### `GetReportProperty(String, Int32.)(...)`

Retrieves an integer property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/ce8e4ca0-3750-3c9b-5ca8-27e48b45e0bd)

#### `GetReportProperty(String, String.)(...)`

Retrieves a string property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/70f212bb-ee70-e9e9-7ab6-d6a9d46f8de6)

#### `GetStringReportProperties(...)`

Retrieves multiple string report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/5ff72da1-7382-f5d2-eb1d-c99b46239d42)

#### `GetStringUserProperties(...)`

Retrieves all string properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/67120e84-c820-9d52-12ae-c167c595842c)

#### `GetUserProperty(String, Double.)(...)`

Retrieves a double property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/17cc8074-7955-32b5-2b4b-b169c45d4ee0)

#### `GetUserProperty(String, Int32.)(...)`

Retrieves an integer property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/d9364c1d-5b57-6a31-4c13-01124058cfb2)

#### `GetUserProperty(String, String.)(...)`

Retrieves a string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/214a886c-c18c-9b66-d7c9-fe84260962f2)

#### `Insert(...)`

Inserts the grid surface into the model.

[Docs](https://developer.tekla.com/topic/en/18/43/f6519bc0-effc-9b06-6a91-282e60b19763)

#### `Modify(...)`

Modifies the grid plane with the given ID.

[Docs](https://developer.tekla.com/topic/en/18/43/35320513-93f4-e092-6b89-dea44df988da)

#### `Select(...)`

Selects the grid surface with the given ID.

[Docs](https://developer.tekla.com/topic/en/18/43/a823571d-ced2-8df9-2dd9-34a5a2886266)

#### `SetCustomObjectType(...)`

Sets a custom object type name for an existing object.

[Docs](https://developer.tekla.com/topic/en/18/43/3d6fb91b-48a9-35ca-d498-526514344089)

#### `SetDynamicStringProperty(...)`

Sets a dynamic string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/4dab0254-09e5-5e61-c28b-1c12afab71a0)

#### `SetLabel(...)`

Sets a label for an object when a new instance is created, this method must be called before Insert. The label is used in plug-ins for identifying the changed object in modification.

[Docs](https://developer.tekla.com/topic/en/18/43/3efcfc44-613a-e66b-9ebd-9f86df4f4036)

#### `SetPhase(...)`

Sets the phase of the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/ed80abcc-0ff7-d081-2229-e651f858a727)

#### `SetUserProperties(...)`

Sets multiple properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/8659da37-d571-f03f-c666-5c9fa3b113f5)

#### `SetUserProperty(String, Double)(...)`

Sets a double property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/5c585b8b-5c2e-7b75-2242-758f17429396)

#### `SetUserProperty(String, Int32)(...)`

Sets an integer property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/dd5b828d-3a87-bb58-5ac9-e1c3b4c2fe4b)

#### `SetUserProperty(String, String)(...)`

Sets a string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/80bf0348-b651-3d4a-862b-49c85bc924ab)

### Properties
- `Color` (object, get/set) — The color of the grid plane. This works only in wireframe views. The color of grid planes in rendered views can be changed using the advanced option XS_GRID_COLOR.
- `DrawingVisibility` (object, get/set) — The visibility of the grid surface in drawings.
- `ExtensionAbove` (object, get/set) — The line extension above.
- `ExtensionBelow` (object, get/set) — The line extension below.
- `ExtensionForMagneticArea` (object, get/set) — The extension for the magnetic area.
- `ExtensionLeft` (object, get/set) — The line extension on the left.
- `ExtensionRight` (object, get/set) — The line extension on the right.
- `Father` (object, get/set) — The father grid of the grid plane.
- `Identifier` (object, get/set) — The identifier of the object.
- `IsMagnetic` (object, get/set) — Whether the grid surface is magnetic or not.
- `IsManual` (object, get/set) — Gets or sets if the line was manually created.
- `IsUpToDate` (object, get/set) — Gets if the object does not have a modification which is not shared.
- `Label` (object, get/set) — The label for the grid surface.
- `ModificationTime` (object, get/set) — Gets latest time of the object was modified or created.
- `Parent` (object, get/set) — The parent grid of the grid surface.
- `Plane` (object, get/set) — The plane defining the position of the grid plane.

## GridSurface (class)

The GridSurface class defines a user defined (possibly magnetic) surface helping in modeling work. A grid surface must always belong to a parent GridBase instance.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/99d2617a-fb93-5d6e-c62d-bc84e0ee76c7)

### Constructors
- `GridSurface(...)` — Creates a new grid surface instance. The default values are as follows: Label = "Default";IsMagnetic = false;ExtensionLeft = 2000.0;ExtensionRight = 2000.0;ExtensionBelow = 2000.0;ExtensionAbove = 2000.0;DrawingVisibility = false;
- `GridSurface(...)` — Creates a new grid surface instance taking the label as input.

### Methods
#### `CompareTo(...)`

Compares Identifiers of model objects.

[Docs](https://developer.tekla.com/topic/en/18/43/79cca356-0c8a-e8fb-463a-34ad4141bec7)

#### `Delete(...)`

Deletes the grid surface with the given ID.

[Docs](https://developer.tekla.com/topic/en/18/43/8e5f9bdd-e3b2-0982-3a72-c9974947d0eb)

#### `Equals(...)`

Check if Identifiers of model objects are same.

[Docs](https://developer.tekla.com/topic/en/18/43/e4324265-2490-00de-4139-63a8f7acb492)

#### `GetAllReportProperties(...)`

Retrieves all properties for the model object in one hashtable. Type for the returned value must be checked using type casting.

[Docs](https://developer.tekla.com/topic/en/18/43/0708f404-ce77-28e1-4ebe-fa906f68bff8)

#### `GetAllUserProperties(...)`

Retrieves all properties for the model object in one hashtable. Type for the returned value must be checked using type casting.

[Docs](https://developer.tekla.com/topic/en/18/43/63b853a0-0af5-af9c-3ce7-05299664d7f8)

#### `GetChildren(...)`

Returns an enumerator of all the children model objects.

[Docs](https://developer.tekla.com/topic/en/18/43/ce51ec40-310c-6067-a89f-f8d029aab747)

#### `GetCoordinateSystem(...)`

Returns the coordinate system for the given model object.

[Docs](https://developer.tekla.com/topic/en/18/43/8adfe9cd-f6e5-6474-1297-33c6270a7b0a)

#### `GetCustomObjectType(...)`

Gets custom object type name from an existing object.

[Docs](https://developer.tekla.com/topic/en/18/43/d3be2ba3-8533-7165-f2d3-a5aa9df556b8)

#### `GetDoubleReportProperties(...)`

Retrieves multiple double report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/f86f6c96-f738-49ca-16ba-f5634e862ba2)

#### `GetDoubleUserProperties(...)`

Retrieves all double properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/6ead7a63-5f76-0631-d480-d46c27e882aa)

#### `GetDynamicStringProperty(...)`

Gets a dynamic string property from the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/415e86d9-7de6-ea53-ce68-20b3e67b8655)

#### `GetFatherComponent(...)`

Returns the father component of the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/a2c28f3f-52ff-50a8-1b32-10d18db5c31d)

#### `GetHierarchicObjects(...)`

Returns an enumerator of all the connected hierarchic objects.

[Docs](https://developer.tekla.com/topic/en/18/43/cc228ea0-267b-7d90-7441-c9efc1779b08)

#### `GetIntegerReportProperties(...)`

Retrieves multiple integer report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/2623a9c0-2a9c-e233-9daa-59f92ed51445)

#### `GetIntegerUserProperties(...)`

Retrieves all integer properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/1b3c6f06-bc25-a54e-2a88-b60dc4a3c85e)

#### `GetPhase(...)`

Retrieves the phase of the model object (the phase number, the phase name, the phase comment and whether the phase is the current one or not).

[Docs](https://developer.tekla.com/topic/en/18/43/310f8b7a-c120-753a-35c9-f7c73a9806c9)

#### `GetReportProperty(String, Double.)(...)`

Retrieves a double property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/1c05aa56-5f42-be91-a59e-8d59a5549f45)

#### `GetReportProperty(String, Int32.)(...)`

Retrieves an integer property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/ce8e4ca0-3750-3c9b-5ca8-27e48b45e0bd)

#### `GetReportProperty(String, String.)(...)`

Retrieves a string property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/70f212bb-ee70-e9e9-7ab6-d6a9d46f8de6)

#### `GetStringReportProperties(...)`

Retrieves multiple string report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/5ff72da1-7382-f5d2-eb1d-c99b46239d42)

#### `GetStringUserProperties(...)`

Retrieves all string properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/67120e84-c820-9d52-12ae-c167c595842c)

#### `GetUserProperty(String, Double.)(...)`

Retrieves a double property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/17cc8074-7955-32b5-2b4b-b169c45d4ee0)

#### `GetUserProperty(String, Int32.)(...)`

Retrieves an integer property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/d9364c1d-5b57-6a31-4c13-01124058cfb2)

#### `GetUserProperty(String, String.)(...)`

Retrieves a string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/214a886c-c18c-9b66-d7c9-fe84260962f2)

#### `Insert(...)`

Inserts the grid surface into the model.

[Docs](https://developer.tekla.com/topic/en/18/43/12a6eef8-ddab-096a-ecce-1f8950b897a5)

#### `Modify(...)`

Modifies the grid surface with the given ID.

[Docs](https://developer.tekla.com/topic/en/18/43/5261c8f7-5a0a-f0dd-2d84-526eb3751bee)

#### `Select(...)`

Selects the grid surface with the given ID.

[Docs](https://developer.tekla.com/topic/en/18/43/e59d762e-e145-c980-9d49-9bc2c07cdd46)

#### `SetCustomObjectType(...)`

Sets a custom object type name for an existing object.

[Docs](https://developer.tekla.com/topic/en/18/43/3d6fb91b-48a9-35ca-d498-526514344089)

#### `SetDynamicStringProperty(...)`

Sets a dynamic string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/4dab0254-09e5-5e61-c28b-1c12afab71a0)

#### `SetLabel(...)`

Sets a label for an object when a new instance is created, this method must be called before Insert. The label is used in plug-ins for identifying the changed object in modification.

[Docs](https://developer.tekla.com/topic/en/18/43/3efcfc44-613a-e66b-9ebd-9f86df4f4036)

#### `SetPhase(...)`

Sets the phase of the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/ed80abcc-0ff7-d081-2229-e651f858a727)

#### `SetUserProperties(...)`

Sets multiple properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/8659da37-d571-f03f-c666-5c9fa3b113f5)

#### `SetUserProperty(String, Double)(...)`

Sets a double property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/5c585b8b-5c2e-7b75-2242-758f17429396)

#### `SetUserProperty(String, Int32)(...)`

Sets an integer property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/dd5b828d-3a87-bb58-5ac9-e1c3b4c2fe4b)

#### `SetUserProperty(String, String)(...)`

Sets a string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/80bf0348-b651-3d4a-862b-49c85bc924ab)

### Properties
- `DrawingVisibility` (object, get/set) — The visibility of the grid surface in drawings.
- `ExtensionAbove` (object, get/set) — The line extension above.
- `ExtensionBelow` (object, get/set) — The line extension below.
- `ExtensionLeft` (object, get/set) — The line extension on the left.
- `ExtensionRight` (object, get/set) — The line extension on the right.
- `Identifier` (object, get/set) — The identifier of the object.
- `IsMagnetic` (object, get/set) — Whether the grid surface is magnetic or not.
- `IsManual` (object, get/set) — Gets or sets if the line was manually created.
- `IsUpToDate` (object, get/set) — Gets if the object does not have a modification which is not shared.
- `Label` (object, get/set) — The label for the grid surface.
- `ModificationTime` (object, get/set) — Gets latest time of the object was modified or created.
- `Parent` (object, get/set) — The parent grid of the grid surface.

## HierarchicDefinition (class)

The HierarchicDefinition class defines a hierarchy structure. This hierarchy structure can be instantiated and extended by hierarchical objects.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/3b2e2665-3864-af76-83c7-ac2221f81a59)

### Constructors
- `HierarchicDefinition(...)` — Creates a new hierarchical definition instance.
- `HierarchicDefinition(...)` — Instantiates a hierarchical definition with a known identifier. The instantiation has to be done before the hierarchical definition instance can be selected.

### Methods
#### `AddObjects(...)`

Associates hierarchical objects with the hierarchical definition. Fails if the hierarchical definition doesn't exist in the model.

[Docs](https://developer.tekla.com/topic/en/18/43/2f575edf-4a3f-3f2a-cae4-b9ffa5382024)

#### `CompareTo(...)`

Compares Identifiers of model objects.

[Docs](https://developer.tekla.com/topic/en/18/43/79cca356-0c8a-e8fb-463a-34ad4141bec7)

#### `Delete(...)`

Deletes the hierarchical definition instance from the model. The ID must be set.

[Docs](https://developer.tekla.com/topic/en/18/43/883b4c3b-2e12-fe69-af39-c2c2f5e7d63f)

#### `Equals(...)`

Check if Identifiers of model objects are same.

[Docs](https://developer.tekla.com/topic/en/18/43/e4324265-2490-00de-4139-63a8f7acb492)

#### `GetAllReportProperties(...)`

Retrieves all properties for the model object in one hashtable. Type for the returned value must be checked using type casting.

[Docs](https://developer.tekla.com/topic/en/18/43/0708f404-ce77-28e1-4ebe-fa906f68bff8)

#### `GetAllUserProperties(...)`

Retrieves all properties for the model object in one hashtable. Type for the returned value must be checked using type casting.

[Docs](https://developer.tekla.com/topic/en/18/43/63b853a0-0af5-af9c-3ce7-05299664d7f8)

#### `GetChildren(...)`

Returns an enumerator of all the children model objects.

[Docs](https://developer.tekla.com/topic/en/18/43/ce51ec40-310c-6067-a89f-f8d029aab747)

#### `GetCoordinateSystem(...)`

Returns the coordinate system for the given model object.

[Docs](https://developer.tekla.com/topic/en/18/43/8adfe9cd-f6e5-6474-1297-33c6270a7b0a)

#### `GetCustomObjectType(...)`

Gets custom object type name from an existing object.

[Docs](https://developer.tekla.com/topic/en/18/43/d3be2ba3-8533-7165-f2d3-a5aa9df556b8)

#### `GetDoubleReportProperties(...)`

Retrieves multiple double report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/f86f6c96-f738-49ca-16ba-f5634e862ba2)

#### `GetDoubleUserProperties(...)`

Retrieves all double properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/6ead7a63-5f76-0631-d480-d46c27e882aa)

#### `GetDynamicStringProperty(...)`

Gets a dynamic string property from the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/415e86d9-7de6-ea53-ce68-20b3e67b8655)

#### `GetFatherComponent(...)`

Returns the father component of the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/a2c28f3f-52ff-50a8-1b32-10d18db5c31d)

#### `GetHierarchicObjects(...)`

Returns an enumerator of all the connected hierarchic objects.

[Docs](https://developer.tekla.com/topic/en/18/43/cc228ea0-267b-7d90-7441-c9efc1779b08)

#### `GetIntegerReportProperties(...)`

Retrieves multiple integer report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/2623a9c0-2a9c-e233-9daa-59f92ed51445)

#### `GetIntegerUserProperties(...)`

Retrieves all integer properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/1b3c6f06-bc25-a54e-2a88-b60dc4a3c85e)

#### `GetPhase(...)`

Retrieves the phase of the model object (the phase number, the phase name, the phase comment and whether the phase is the current one or not).

[Docs](https://developer.tekla.com/topic/en/18/43/310f8b7a-c120-753a-35c9-f7c73a9806c9)

#### `GetReportProperty(String, Double.)(...)`

Retrieves a double property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/1c05aa56-5f42-be91-a59e-8d59a5549f45)

#### `GetReportProperty(String, Int32.)(...)`

Retrieves an integer property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/ce8e4ca0-3750-3c9b-5ca8-27e48b45e0bd)

#### `GetReportProperty(String, String.)(...)`

Retrieves a string property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/70f212bb-ee70-e9e9-7ab6-d6a9d46f8de6)

#### `GetStringReportProperties(...)`

Retrieves multiple string report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/5ff72da1-7382-f5d2-eb1d-c99b46239d42)

#### `GetStringUserProperties(...)`

Retrieves all string properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/67120e84-c820-9d52-12ae-c167c595842c)

#### `GetUserProperty(String, Double.)(...)`

Retrieves a double property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/17cc8074-7955-32b5-2b4b-b169c45d4ee0)

#### `GetUserProperty(String, Int32.)(...)`

Retrieves an integer property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/d9364c1d-5b57-6a31-4c13-01124058cfb2)

#### `GetUserProperty(String, String.)(...)`

Retrieves a string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/214a886c-c18c-9b66-d7c9-fe84260962f2)

#### `Insert(...)`

Inserts the hierarchical definition instance in the model.

[Docs](https://developer.tekla.com/topic/en/18/43/a2ed914d-c426-03ff-540e-3a51e34d739a)

#### `Modify(...)`

Modifies the hierarchical definition instance in the model. The ID must be set.

[Docs](https://developer.tekla.com/topic/en/18/43/b0f7f8df-aadf-e2b6-82f0-aa5ee3220614)

#### `RemoveObjects(...)`

Removes associations to the given list of hierarchical objects. Fails if the hierarchical definition doesn't exist in the model.

[Docs](https://developer.tekla.com/topic/en/18/43/6a6c45e1-31c2-00e6-f6e5-7e88e2a3f265)

#### `Select(...)`

Selects the hierarchical definition instance from the model. The ID must be set.

[Docs](https://developer.tekla.com/topic/en/18/43/53adca26-d427-4a62-4b0e-75cc663cf0d5)

#### `SetCustomObjectType(...)`

Sets a custom object type name for an existing object.

[Docs](https://developer.tekla.com/topic/en/18/43/3d6fb91b-48a9-35ca-d498-526514344089)

#### `SetDynamicStringProperty(...)`

Sets a dynamic string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/4dab0254-09e5-5e61-c28b-1c12afab71a0)

#### `SetLabel(...)`

Sets a label for an object when a new instance is created, this method must be called before Insert. The label is used in plug-ins for identifying the changed object in modification.

[Docs](https://developer.tekla.com/topic/en/18/43/3efcfc44-613a-e66b-9ebd-9f86df4f4036)

#### `SetPhase(...)`

Sets the phase of the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/ed80abcc-0ff7-d081-2229-e651f858a727)

#### `SetUserProperties(...)`

Sets multiple properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/8659da37-d571-f03f-c666-5c9fa3b113f5)

#### `SetUserProperty(String, Double)(...)`

Sets a double property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/5c585b8b-5c2e-7b75-2242-758f17429396)

#### `SetUserProperty(String, Int32)(...)`

Sets an integer property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/dd5b828d-3a87-bb58-5ac9-e1c3b4c2fe4b)

#### `SetUserProperty(String, String)(...)`

Sets a string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/80bf0348-b651-3d4a-862b-49c85bc924ab)

### Properties
- `CustomType` (object, get/set) — The custom type of the hierarchical definition.
- `Drawable` (object, get/set) — Defines whether the hierarchical definition is a drawable area in the model.
- `Father` (object, get/set) — The father of the hierarchical definition.
- `HierarchicChildren` (object, get/set) — An array list that contains the hierarchical definition's children.
- `HierarchyIdentifier` (object, get/set) — The GUID of the hierarchical definition.
- `HierarchyType` (object, get/set) — The type of the hierarchical definition.
- `Identifier` (object, get/set) — The identifier of the object.
- `IsUpToDate` (object, get/set) — Gets if the object does not have a modification which is not shared.
- `ModificationTime` (object, get/set) — Gets latest time of the object was modified or created.
- `Name` (object, get/set) — The name of the hierarchical definition.

## HierarchicDefinitionTypeEnum (enum)

Defines the different types of classification hierarchies.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/07fc35c8-a364-8d68-e9cb-4798407bf2b7)

### Values
- `DOT_HIERARCHIC_CUSTOM_TYPE` = `0` — The custom type classification hierarchy.
- `DOT_HIERARCHIC_LOGICAL_BUILDING_AREA` = `1` — The logical building area classification hierarchy.
- `DOT_HIERARCHIC_OBJECT_TYPE` = `2` — The object type classification hierarchy.
- `DOT_HIERARCHIC_TASK_WORK_TYPE` = `3` — The task work type classification hierarchy.
- `DOT_HIERARCHIC_TASK_SCENARIO` = `4` — The task scenario classification hierarchy.

## HierarchicObject (class)

The HierarchicObject class defines an object in a tree-structured hierarchy. The hierarchy structure is defined by hierarchical definitions which the hierarchical objects instantiate. Hierarchical objects may contain other hierarchical objects, thus forming a sub-hierarchy, or model objects.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/c62fa47a-7ee7-9f7c-7151-313d6b9aad25)

### Constructors
- `HierarchicObject(...)` — Creates a new hierarchical object instance.
- `HierarchicObject(...)` — Instantiates a hierarchical object with a known identifier. The instantiation has to be done before the hierarchical definition instance can be selected.

### Methods
#### `AddObjects(...)`

Associates model objects with the hierarchical object. Fails if the hierarchical definition doesn't exist in the model.

[Docs](https://developer.tekla.com/topic/en/18/43/09b467a0-069b-a7d9-2723-fc47c289ad73)

#### `CompareTo(...)`

Compares Identifiers of model objects.

[Docs](https://developer.tekla.com/topic/en/18/43/79cca356-0c8a-e8fb-463a-34ad4141bec7)

#### `Delete(...)`

Deletes the hierarchical object instance from the model. The ID must be set.

[Docs](https://developer.tekla.com/topic/en/18/43/4ebdb86c-1334-bd9d-efcf-dd5fa7e3eb9a)

#### `Equals(...)`

Check if Identifiers of model objects are same.

[Docs](https://developer.tekla.com/topic/en/18/43/e4324265-2490-00de-4139-63a8f7acb492)

#### `GetAllReportProperties(...)`

Retrieves all properties for the model object in one hashtable. Type for the returned value must be checked using type casting.

[Docs](https://developer.tekla.com/topic/en/18/43/0708f404-ce77-28e1-4ebe-fa906f68bff8)

#### `GetAllUserProperties(...)`

Retrieves all properties for the model object in one hashtable. Type for the returned value must be checked using type casting.

[Docs](https://developer.tekla.com/topic/en/18/43/63b853a0-0af5-af9c-3ce7-05299664d7f8)

#### `GetChildren(...)`

Returns an enumerator of all the children model objects.

[Docs](https://developer.tekla.com/topic/en/18/43/ce51ec40-310c-6067-a89f-f8d029aab747)

#### `GetCoordinateSystem(...)`

Returns the coordinate system for the given model object.

[Docs](https://developer.tekla.com/topic/en/18/43/8adfe9cd-f6e5-6474-1297-33c6270a7b0a)

#### `GetCustomObjectType(...)`

Gets custom object type name from an existing object.

[Docs](https://developer.tekla.com/topic/en/18/43/d3be2ba3-8533-7165-f2d3-a5aa9df556b8)

#### `GetDoubleReportProperties(...)`

Retrieves multiple double report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/f86f6c96-f738-49ca-16ba-f5634e862ba2)

#### `GetDoubleUserProperties(...)`

Retrieves all double properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/6ead7a63-5f76-0631-d480-d46c27e882aa)

#### `GetDynamicStringProperty(...)`

Gets a dynamic string property from the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/415e86d9-7de6-ea53-ce68-20b3e67b8655)

#### `GetFatherComponent(...)`

Returns the father component of the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/a2c28f3f-52ff-50a8-1b32-10d18db5c31d)

#### `GetHierarchicObjects(...)`

Returns an enumerator of all the connected hierarchic objects.

[Docs](https://developer.tekla.com/topic/en/18/43/cc228ea0-267b-7d90-7441-c9efc1779b08)

#### `GetIntegerReportProperties(...)`

Retrieves multiple integer report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/2623a9c0-2a9c-e233-9daa-59f92ed51445)

#### `GetIntegerUserProperties(...)`

Retrieves all integer properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/1b3c6f06-bc25-a54e-2a88-b60dc4a3c85e)

#### `GetPhase(...)`

Retrieves the phase of the model object (the phase number, the phase name, the phase comment and whether the phase is the current one or not).

[Docs](https://developer.tekla.com/topic/en/18/43/310f8b7a-c120-753a-35c9-f7c73a9806c9)

#### `GetReportProperty(String, Double.)(...)`

Retrieves a double property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/1c05aa56-5f42-be91-a59e-8d59a5549f45)

#### `GetReportProperty(String, Int32.)(...)`

Retrieves an integer property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/ce8e4ca0-3750-3c9b-5ca8-27e48b45e0bd)

#### `GetReportProperty(String, String.)(...)`

Retrieves a string property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/70f212bb-ee70-e9e9-7ab6-d6a9d46f8de6)

#### `GetStringReportProperties(...)`

Retrieves multiple string report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/5ff72da1-7382-f5d2-eb1d-c99b46239d42)

#### `GetStringUserProperties(...)`

Retrieves all string properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/67120e84-c820-9d52-12ae-c167c595842c)

#### `GetUserProperty(String, Double.)(...)`

Retrieves a double property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/17cc8074-7955-32b5-2b4b-b169c45d4ee0)

#### `GetUserProperty(String, Int32.)(...)`

Retrieves an integer property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/d9364c1d-5b57-6a31-4c13-01124058cfb2)

#### `GetUserProperty(String, String.)(...)`

Retrieves a string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/214a886c-c18c-9b66-d7c9-fe84260962f2)

#### `Insert(...)`

Inserts the hierarchical object instance in the model.

[Docs](https://developer.tekla.com/topic/en/18/43/9d526073-306e-9dfe-d016-1871cdf97a02)

#### `Modify(...)`

Modifies the hierarchical object instance in the model. The ID must be set.

[Docs](https://developer.tekla.com/topic/en/18/43/072c41ba-af7f-2820-8380-6e511d979162)

#### `RemoveObjects(...)`

Removes associations to the given list of model objects. Fails if the hierarchical definition doesn't exist in the model.

[Docs](https://developer.tekla.com/topic/en/18/43/27d6506b-cd2e-8d4d-5faa-c7d5cd051add)

#### `Select(...)`

Selects the hierarchical object instance from the model. The ID must be set.

[Docs](https://developer.tekla.com/topic/en/18/43/99994843-03a6-09b3-fe30-5c6a2bcd19a6)

#### `SetCustomObjectType(...)`

Sets a custom object type name for an existing object.

[Docs](https://developer.tekla.com/topic/en/18/43/3d6fb91b-48a9-35ca-d498-526514344089)

#### `SetDynamicStringProperty(...)`

Sets a dynamic string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/4dab0254-09e5-5e61-c28b-1c12afab71a0)

#### `SetLabel(...)`

Sets a label for an object when a new instance is created, this method must be called before Insert. The label is used in plug-ins for identifying the changed object in modification.

[Docs](https://developer.tekla.com/topic/en/18/43/3efcfc44-613a-e66b-9ebd-9f86df4f4036)

#### `SetPhase(...)`

Sets the phase of the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/ed80abcc-0ff7-d081-2229-e651f858a727)

#### `SetUserProperties(...)`

Sets multiple properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/8659da37-d571-f03f-c666-5c9fa3b113f5)

#### `SetUserProperty(String, Double)(...)`

Sets a double property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/5c585b8b-5c2e-7b75-2242-758f17429396)

#### `SetUserProperty(String, Int32)(...)`

Sets an integer property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/dd5b828d-3a87-bb58-5ac9-e1c3b4c2fe4b)

#### `SetUserProperty(String, String)(...)`

Sets a string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/80bf0348-b651-3d4a-862b-49c85bc924ab)

### Properties
- `Definition` (object, get/set) — The hierarchical definition which the hierarchical object is associated with.
- `Father` (object, get/set) — The father of the hierarchical object.
- `HierarchicChildren` (object, get/set) — An array list that contains the hierarchical object's hierarchical children (of the type hierarchical object).
- `Identifier` (object, get/set) — The identifier of the object.
- `IsUpToDate` (object, get/set) — Gets if the object does not have a modification which is not shared.
- `ModificationTime` (object, get/set) — Gets latest time of the object was modified or created.
- `Name` (object, get/set) — The name of the hierarchical object.

## IAssemblable (interface)

The IAssemblable interface dictates which model objects are capabable of belonging to an assembly at the leaf level.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/c937f46b-c437-8d96-0545-6067119dfea2)

### Methods
#### `GetAssembly(...)`

Returns the assembly that the assemblable belongs to.

[Docs](https://developer.tekla.com/topic/en/18/43/5d79c5f5-a401-ab6b-3566-ae4ae54054a4)

## IEvents (interface)

Interface for the Events class allows the user to register event listeners for Model events. This interface allows the Marshaling of the Events class with Trimble.Remoting

[Vendor docs](https://developer.tekla.com/topic/en/18/43/54bfb640-73cf-907f-a432-96246eca23ca)

### Methods
#### `OnClipPlaneChanged(...)`

Called when user inserts, updates or deletes a clip plane.

[Docs](https://developer.tekla.com/topic/en/18/43/c37bfe9c-c675-7019-fce7-92e7e00ac0d3)

#### `OnHiddenObjectsChanged(...)`

Called when objects are set to hidden or view is redrawn

[Docs](https://developer.tekla.com/topic/en/18/43/98996a2a-e743-b61f-672f-24d68eb39832)

#### `OnInterrupted(...)`

Called when user interrupts.

[Docs](https://developer.tekla.com/topic/en/18/43/adb85435-860d-1fe0-0075-337112cfa0ce)

#### `OnModelSaveInfo(...)`

Called when model is saved.

[Docs](https://developer.tekla.com/topic/en/18/43/a583e0b1-10d5-7a6c-f68f-43a06553236d)

#### `OnTemporaryStatesChanged(...)`

Called when the temporary states are changed.

[Docs](https://developer.tekla.com/topic/en/18/43/c5c53a9a-e57f-7f37-0322-047d8aef2e1c)

#### `OnUndoClicked(...)`

Called when user clicks undo.

[Docs](https://developer.tekla.com/topic/en/18/43/69c17382-2e77-f635-a3a8-5765de998eda)

#### `OnViewClosed(...)`

Called when a view is closed.

[Docs](https://developer.tekla.com/topic/en/18/43/1400ea41-9ffe-d039-2935-970c210d98b0)

#### `Register(...)`

Registers the instance to listen to the specified events. More event delegates should not be added without calling UnRegister first.

[Docs](https://developer.tekla.com/topic/en/18/43/d0730bc4-a833-068f-d632-6916bd501832)

#### `UnRegister(...)`

Unregisters the instance from listening to events.

[Docs](https://developer.tekla.com/topic/en/18/43/f3c67e22-daa9-f10a-68d7-eeda79d6c10f)

### Events
#### `ClashCheckDone` (`EventHandler`)

**Signature:** `event EventHandler ClashCheckDone`

The ClashCheckDone event is raised just after clash check is completed.

[Docs](https://developer.tekla.com/topic/en/18/43/e0dd04f8-a536-e2c0-a8d0-e7527e7dd829)

#### `ClashDetected` (`EventHandler`)

**Signature:** `event EventHandler ClashDetected`

The ClashDetected event is raised just after the detection of each clash. The clash information is saved before the event is triggered.

[Docs](https://developer.tekla.com/topic/en/18/43/410049d9-af0c-4049-52e6-f558b577d34b)

#### `ClipPlaneChanged` (`EventHandler`)

**Signature:** `event EventHandler ClipPlaneChanged`

Occurs when clip plane is changed.

[Docs](https://developer.tekla.com/topic/en/18/43/ca0c7199-f018-3c28-5b9f-332caebb9e4b)

#### `CommandStatusChange` (`EventHandler`)

**Signature:** `event EventHandler CommandStatusChange`

The CommandStatusChange event is raised when a command is started/ended.

[Docs](https://developer.tekla.com/topic/en/18/43/e9291cd7-830b-a215-9520-917294174dc7)

#### `Interrupted` (`EventHandler`)

**Signature:** `event EventHandler Interrupted`

Occurs when user interrupts.

[Docs](https://developer.tekla.com/topic/en/18/43/3a0a0ed7-6297-8aba-3edb-6d1e09dfa429)

#### `ModelChanged` (`EventHandler`)

**Signature:** `event EventHandler ModelChanged`

The ModelChanged event is raised just after some changes have been made to the model. Note, this event is however not triggered when UNDO or REDO are performed.

[Docs](https://developer.tekla.com/topic/en/18/43/00930987-8efe-41ab-e124-6d9d5c00fe12)

#### `ModelLoad` (`EventHandler`)

**Signature:** `event EventHandler ModelLoad`

The ModelLoad event is raised just after a model has been loaded.

[Docs](https://developer.tekla.com/topic/en/18/43/040b1b98-1abb-f840-d794-a834e9cece9b)

#### `ModelLoadInfo` (`EventHandler`)

**Signature:** `event EventHandler ModelLoadInfo`

The ModelLoadInfo event is raised just after a model has been loaded with the model load information as a string parameter.

[Docs](https://developer.tekla.com/topic/en/18/43/eeef8696-14ac-ed0f-2f32-f517de721303)

#### `ModelObjectChanged` (`EventHandler`)

**Signature:** `event EventHandler ModelObjectChanged`

The ModelObjectChanged event is raised just after some changes have been made to the model objects. Note, this event is triggered when UNDO or REDO are performed!!

[Docs](https://developer.tekla.com/topic/en/18/43/b3deb34d-bf92-ac3c-7ad7-44741c2d565a)

#### `ModelObjectNumbered` (`EventHandler`)

**Signature:** `event EventHandler ModelObjectNumbered`

The ModelObjectNumbered event is raised when model object is numbered.

[Docs](https://developer.tekla.com/topic/en/18/43/14649407-f014-ce82-2c3c-112c81bb0a34)

#### `ModelSave` (`EventHandler`)

**Signature:** `event EventHandler ModelSave`

The ModelSave event is raised just after a model has been saved.

[Docs](https://developer.tekla.com/topic/en/18/43/8238325b-a4e0-e7df-d735-2c2baf3f94b0)

#### `ModelSaveAs` (`EventHandler`)

**Signature:** `event EventHandler ModelSaveAs`

The ModelSaveAs event is raised just after a model has been saved using save as.

[Docs](https://developer.tekla.com/topic/en/18/43/5673f9ff-d163-61b1-6ac9-385d67ab8487)

#### `ModelSaveInfo` (`EventHandler`)

**Signature:** `event EventHandler ModelSaveInfo`

The ModelSave event is raised just after a model has been saved with the model save information as a string parameter.

[Docs](https://developer.tekla.com/topic/en/18/43/f3aa5f94-d3d4-a4da-b57b-b7d948c0e1a9)

#### `ModelUnloading` (`EventHandler`)

**Signature:** `event EventHandler ModelUnloading`

The ModelUnloading event is raised just before the model is unloaded.

[Docs](https://developer.tekla.com/topic/en/18/43/b560ae15-4784-0898-3193-d8da77b8e09e)

#### `Numbering` (`EventHandler`)

**Signature:** `event EventHandler Numbering`

The Numbering event is raised just after a model has been numbered.

[Docs](https://developer.tekla.com/topic/en/18/43/eef0a75c-c885-2b43-b979-6eaf5df12a5d)

#### `ProjectInfoChanged` (`EventHandler`)

**Signature:** `event EventHandler ProjectInfoChanged`

The ProjectInfoChanged event is raised just after some changes have been made to the project info. Note, this event is however not triggered when UNDO or REDO are performed.

[Docs](https://developer.tekla.com/topic/en/18/43/c788e409-8eb6-b735-061d-15ffebe8e047)

#### `SelectionChange` (`EventHandler`)

**Signature:** `event EventHandler SelectionChange`

The SelectionChange event is raised when the user changes the current selection inside Tekla Structures.

[Docs](https://developer.tekla.com/topic/en/18/43/b7d4c817-4945-bc48-d7f0-4214aaf36ab2)

#### `TeklaStructuresExit` (`EventHandler`)

**Signature:** `event EventHandler TeklaStructuresExit`

The TeklaStructuresExit event is raised just before Tekla Structures exits. After this event has been called the user will no longer be able to perform any calls to Tekla Structures.

[Docs](https://developer.tekla.com/topic/en/18/43/7c70898a-bda9-cb2b-63a6-4dec85d477bd)

#### `ViewCameraChanged` (`EventHandler`)

**Signature:** `event EventHandler ViewCameraChanged`

Occurs when view camera is changed.

[Docs](https://developer.tekla.com/topic/en/18/43/dd9dc833-4465-0ef8-5ed2-ac785c96eb4e)

## IGeometryNode (interface)

The IGeometryNode interface represents geometry tree node. This is implemented by concrete classes for the various types of geometry nodes

[Vendor docs](https://developer.tekla.com/topic/en/18/43/d79a60dd-f84b-ce24-8bfa-a7a2f1cb3912)

### Methods
#### `AcceptVisitor(...)`

Entry method for the visitor pattern

[Docs](https://developer.tekla.com/topic/en/18/43/e4a23ca4-afba-4ac6-574b-9133549c4515)

#### `Clone(...)`

Creates a new object that is a copy of the current instance.

[Docs](https://developer.tekla.com/topic/en/18/43/e01da20e-a19e-e9c9-dfb4-c1aa981b41b4)

### Properties
- `IsAutomatic` (object, get/set) — Gets a value indicating whether this geometry node was automatically generated (i.e. false if it was originally a user-defined part)

## IGeometryNodeVisitor (interface)

The IGeometryNodeVisitor interface is implemented by classes visiting geometry nodes

[Vendor docs](https://developer.tekla.com/topic/en/18/43/526199de-8d7e-02d0-0e3a-2a1a0793cf8d)

### Methods
#### `Visit(CylindricalSurfaceNode)(...)`

Visits a node that is an instance of CylindricalSurfaceNode

[Docs](https://developer.tekla.com/topic/en/18/43/3958d1f8-7497-672d-80c9-0bbf60724b1c)

#### `Visit(PolygonNode)(...)`

Visits a node that is an instance of PolygonNode node

[Docs](https://developer.tekla.com/topic/en/18/43/458e978f-9b0c-1bfd-0807-acbddaa02233)

#### `Visit(SpiralNode)(...)`

Visits a node that is an instance of HelixNode

[Docs](https://developer.tekla.com/topic/en/18/43/7043fa01-8442-e7a4-7c63-b3486b1ff65f)

## InputItem (class)

The InputItem class is used to store input objects and positions.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/cc7681b2-4148-059a-f0f0-641b7c2d2769)

### Methods
#### `GetData(...)`

Returns the data of the input item. See also: InputItem.InputTypeEnum.

[Docs](https://developer.tekla.com/topic/en/18/43/d588fef0-4022-bf7b-a92f-b3528419fe35)

#### `GetInputType(...)`

Returns the type of the input item.

[Docs](https://developer.tekla.com/topic/en/18/43/9929646f-ef1d-03c3-f4d1-3d123818108d)

## InputItem.InputTypeEnum (enum)

Defines the type of the input.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/a0433c89-2537-7cbc-0224-d7f60ed163d3)

### Values
- `INPUT_1_POINT` = `0` — The input is one point. The data is returned as a point.
- `INPUT_2_POINTS` = `1` — The input is two points. The data is returned as an array list of points.
- `INPUT_POLYGON` = `2` — The input is any number of points (a polygon). The data is returned as an array list of points.
- `INPUT_1_OBJECT` = `3` — The input is one model object. The data is returned as a model object.
- `INPUT_N_OBJECTS` = `4` — The input is any number of model objects. The data is returned as an array list of model objects.

## InvalidCurveCombinationException (class)

This class represent an exception thrown when the combination of base curves is not allowed in a lofted plate operation

[Vendor docs](https://developer.tekla.com/topic/en/18/43/aee41b14-a9b8-3cd3-84ff-3b08d60f94e6)

### Constructors
- `InvalidCurveCombinationException(...)` — Constructs an instance of the class

## InvalidFacePointsException (class)

The InvalidFacePointsException class represents an error when, cannot create geometry from requested face points.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/aec29813-439a-ea77-34ae-d2c1c3db7ef7)

### Constructors
- `InvalidFacePointsException(...)` — Initializes a new instance of the InvalidFacePointsException class with "Cannot create geometry from requested face points" error message.

## InvalidRadiusException (class)

The InvalidRadiusException class represents an error when, cannot connect parts with a cylindrical section having the provided radius.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/9f0fd914-78f1-fb5d-af93-9a736d9462d4)

### Constructors
- `InvalidRadiusException(...)` — Initializes a new instance of the InvalidRadiusException class with "Cannot connect parts with a cylindrical section having the provided radius" error message.

## Load (class)

The Load class is an abstract base class for all loads.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/d4147471-17dc-0a4b-f579-3f9f50fa93aa)

### Constructors
- `Load(...)` — Creates a new load instance.

### Methods
#### `CompareTo(...)`

Compares Identifiers of model objects.

[Docs](https://developer.tekla.com/topic/en/18/43/79cca356-0c8a-e8fb-463a-34ad4141bec7)

#### `Delete(...)`

Deletes the instance from the model database.

[Docs](https://developer.tekla.com/topic/en/18/43/0b922776-ccee-4b5e-4a12-4e5306802e90)

#### `Equals(...)`

Check if Identifiers of model objects are same.

[Docs](https://developer.tekla.com/topic/en/18/43/e4324265-2490-00de-4139-63a8f7acb492)

#### `GetAllReportProperties(...)`

Retrieves all properties for the model object in one hashtable. Type for the returned value must be checked using type casting.

[Docs](https://developer.tekla.com/topic/en/18/43/0708f404-ce77-28e1-4ebe-fa906f68bff8)

#### `GetAllUserProperties(...)`

Retrieves all properties for the model object in one hashtable. Type for the returned value must be checked using type casting.

[Docs](https://developer.tekla.com/topic/en/18/43/63b853a0-0af5-af9c-3ce7-05299664d7f8)

#### `GetChildren(...)`

Returns an enumerator of all the children model objects.

[Docs](https://developer.tekla.com/topic/en/18/43/ce51ec40-310c-6067-a89f-f8d029aab747)

#### `GetCoordinateSystem(...)`

Returns the coordinate system for the given model object.

[Docs](https://developer.tekla.com/topic/en/18/43/8adfe9cd-f6e5-6474-1297-33c6270a7b0a)

#### `GetCustomObjectType(...)`

Gets custom object type name from an existing object.

[Docs](https://developer.tekla.com/topic/en/18/43/d3be2ba3-8533-7165-f2d3-a5aa9df556b8)

#### `GetDoubleReportProperties(...)`

Retrieves multiple double report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/f86f6c96-f738-49ca-16ba-f5634e862ba2)

#### `GetDoubleUserProperties(...)`

Retrieves all double properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/6ead7a63-5f76-0631-d480-d46c27e882aa)

#### `GetDynamicStringProperty(...)`

Gets a dynamic string property from the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/415e86d9-7de6-ea53-ce68-20b3e67b8655)

#### `GetFatherComponent(...)`

Returns the father component of the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/a2c28f3f-52ff-50a8-1b32-10d18db5c31d)

#### `GetHierarchicObjects(...)`

Returns an enumerator of all the connected hierarchic objects.

[Docs](https://developer.tekla.com/topic/en/18/43/cc228ea0-267b-7d90-7441-c9efc1779b08)

#### `GetIntegerReportProperties(...)`

Retrieves multiple integer report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/2623a9c0-2a9c-e233-9daa-59f92ed51445)

#### `GetIntegerUserProperties(...)`

Retrieves all integer properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/1b3c6f06-bc25-a54e-2a88-b60dc4a3c85e)

#### `GetPhase(...)`

Retrieves the phase of the model object (the phase number, the phase name, the phase comment and whether the phase is the current one or not).

[Docs](https://developer.tekla.com/topic/en/18/43/310f8b7a-c120-753a-35c9-f7c73a9806c9)

#### `GetReportProperty(String, Double.)(...)`

Retrieves a double property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/1c05aa56-5f42-be91-a59e-8d59a5549f45)

#### `GetReportProperty(String, Int32.)(...)`

Retrieves an integer property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/ce8e4ca0-3750-3c9b-5ca8-27e48b45e0bd)

#### `GetReportProperty(String, String.)(...)`

Retrieves a string property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/70f212bb-ee70-e9e9-7ab6-d6a9d46f8de6)

#### `GetStringReportProperties(...)`

Retrieves multiple string report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/5ff72da1-7382-f5d2-eb1d-c99b46239d42)

#### `GetStringUserProperties(...)`

Retrieves all string properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/67120e84-c820-9d52-12ae-c167c595842c)

#### `GetUserProperty(String, Double.)(...)`

Retrieves a double property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/17cc8074-7955-32b5-2b4b-b169c45d4ee0)

#### `GetUserProperty(String, Int32.)(...)`

Retrieves an integer property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/d9364c1d-5b57-6a31-4c13-01124058cfb2)

#### `GetUserProperty(String, String.)(...)`

Retrieves a string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/214a886c-c18c-9b66-d7c9-fe84260962f2)

#### `Insert(...)`

Inserts the model object instance into the model database.

[Docs](https://developer.tekla.com/topic/en/18/43/f17ed840-1d16-b843-4b0c-270875de1d35)

#### `Modify(...)`

Modifies the model instance in the model database.

[Docs](https://developer.tekla.com/topic/en/18/43/186833b8-4837-ed77-b314-3ebf9dc32d6a)

#### `Select(...)`

Selects the model object instance from the model database.

[Docs](https://developer.tekla.com/topic/en/18/43/f662ab10-be15-6280-d0aa-fc259dfd7c07)

#### `SetCustomObjectType(...)`

Sets a custom object type name for an existing object.

[Docs](https://developer.tekla.com/topic/en/18/43/3d6fb91b-48a9-35ca-d498-526514344089)

#### `SetDynamicStringProperty(...)`

Sets a dynamic string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/4dab0254-09e5-5e61-c28b-1c12afab71a0)

#### `SetLabel(...)`

Sets a label for an object when a new instance is created, this method must be called before Insert. The label is used in plug-ins for identifying the changed object in modification.

[Docs](https://developer.tekla.com/topic/en/18/43/3efcfc44-613a-e66b-9ebd-9f86df4f4036)

#### `SetPhase(...)`

Sets the phase of the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/ed80abcc-0ff7-d081-2229-e651f858a727)

#### `SetUserProperties(...)`

Sets multiple properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/8659da37-d571-f03f-c666-5c9fa3b113f5)

#### `SetUserProperty(String, Double)(...)`

Sets a double property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/5c585b8b-5c2e-7b75-2242-758f17429396)

#### `SetUserProperty(String, Int32)(...)`

Sets an integer property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/dd5b828d-3a87-bb58-5ac9-e1c3b4c2fe4b)

#### `SetUserProperty(String, String)(...)`

Sets a string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/80bf0348-b651-3d4a-862b-49c85bc924ab)

### Properties
- `AutomaticPrimaryAxisWeight` (object, get/set) — The automatic primary axis weight.
- `BoundingBoxDx` (object, get/set) — The dimension of the bounding box in the X direction.
- `BoundingBoxDy` (object, get/set) — The dimension of the bounding box in the Y direction.
- `BoundingBoxDz` (object, get/set) — The dimension of the bounding box in the Z direction.
- `CreateFixedSupportConditionsAutomatically` (object, get/set) — Indicates whether fixed support conditions are created automatically.
- `FatherId` (object, get/set) — The identifier of the father object that the load is attached to.
- `Group` (object, get/set) — The load group object for the load.
- `Identifier` (object, get/set) — The identifier of the object.
- `IsUpToDate` (object, get/set) — Gets if the object does not have a modification which is not shared.
- `LoadAttachment` (object, get/set) — The load attachment.
- `LoadDispersionAngle` (object, get/set) — The load dispersion angle.
- `ModificationTime` (object, get/set) — Gets latest time of the object was modified or created.
- `PartFilter` (object, get/set) — The part filter.
- `PartNames` (object, get/set) — The part names.
- `PrimaryAxisDirection` (object, get/set) — The vector for the primary axis direction.
- `Spanning` (object, get/set) — The load spanning.
- `Weight` (object, get/set) — The weight.

## Load.LoadAttachmentEnum (enum)

The load attachment.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/a2ef5acf-8fd5-795e-e10a-3158afc3acfa)

### Values
- `LOAD_ATTACHMENT_ATTACH_TO_MEMBER` = `4001` — Attach to member.
- `LOAD_ATTACHMENT_DONT_ATTACH` = `4003` — Don't attach.

## Load.LoadPartNamesEnum (enum)

The load part names.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/1fac12d5-3ea1-bf0c-4173-ad9376ec2d9c)

### Values
- `LOAD_PART_NAMES_EXCLUDE` = `0` — Exclude parts by name.
- `LOAD_PART_NAMES_INCLUDE` = `1` — Include parts by name.

## Load.LoadSpanningEnum (enum)

The load spanning.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/2d4b6439-a7f7-2f14-8a1a-37e5ff6ca95a)

### Values
- `LOAD_SPANNING_SINGLE` = `1` — Single spanning.
- `LOAD_SPANNING_DOUBLE` = `2` — Double spanning.

## LoadArea (class)

The LoadArea class defines a linearly-distributed force bound by a triangle or a quadrangle.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/0c92809d-6aa4-549b-023e-7879db4d7d61)

### Constructors
- `LoadArea(...)` — Creates a new load area instance.

### Methods
#### `CompareTo(...)`

Compares Identifiers of model objects.

[Docs](https://developer.tekla.com/topic/en/18/43/79cca356-0c8a-e8fb-463a-34ad4141bec7)

#### `Delete(...)`

Deletes the load area instance with the given ID from the model database.

[Docs](https://developer.tekla.com/topic/en/18/43/bb3faf00-2d66-bcf0-f64b-ce07dea84e61)

#### `Equals(...)`

Check if Identifiers of model objects are same.

[Docs](https://developer.tekla.com/topic/en/18/43/e4324265-2490-00de-4139-63a8f7acb492)

#### `GetAllReportProperties(...)`

Retrieves all properties for the model object in one hashtable. Type for the returned value must be checked using type casting.

[Docs](https://developer.tekla.com/topic/en/18/43/0708f404-ce77-28e1-4ebe-fa906f68bff8)

#### `GetAllUserProperties(...)`

Retrieves all properties for the model object in one hashtable. Type for the returned value must be checked using type casting.

[Docs](https://developer.tekla.com/topic/en/18/43/63b853a0-0af5-af9c-3ce7-05299664d7f8)

#### `GetChildren(...)`

Returns an enumerator of all the children model objects.

[Docs](https://developer.tekla.com/topic/en/18/43/ce51ec40-310c-6067-a89f-f8d029aab747)

#### `GetCoordinateSystem(...)`

Returns the coordinate system for the given model object.

[Docs](https://developer.tekla.com/topic/en/18/43/8adfe9cd-f6e5-6474-1297-33c6270a7b0a)

#### `GetCustomObjectType(...)`

Gets custom object type name from an existing object.

[Docs](https://developer.tekla.com/topic/en/18/43/d3be2ba3-8533-7165-f2d3-a5aa9df556b8)

#### `GetDoubleReportProperties(...)`

Retrieves multiple double report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/f86f6c96-f738-49ca-16ba-f5634e862ba2)

#### `GetDoubleUserProperties(...)`

Retrieves all double properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/6ead7a63-5f76-0631-d480-d46c27e882aa)

#### `GetDynamicStringProperty(...)`

Gets a dynamic string property from the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/415e86d9-7de6-ea53-ce68-20b3e67b8655)

#### `GetFatherComponent(...)`

Returns the father component of the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/a2c28f3f-52ff-50a8-1b32-10d18db5c31d)

#### `GetHierarchicObjects(...)`

Returns an enumerator of all the connected hierarchic objects.

[Docs](https://developer.tekla.com/topic/en/18/43/cc228ea0-267b-7d90-7441-c9efc1779b08)

#### `GetIntegerReportProperties(...)`

Retrieves multiple integer report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/2623a9c0-2a9c-e233-9daa-59f92ed51445)

#### `GetIntegerUserProperties(...)`

Retrieves all integer properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/1b3c6f06-bc25-a54e-2a88-b60dc4a3c85e)

#### `GetPhase(...)`

Retrieves the phase of the model object (the phase number, the phase name, the phase comment and whether the phase is the current one or not).

[Docs](https://developer.tekla.com/topic/en/18/43/310f8b7a-c120-753a-35c9-f7c73a9806c9)

#### `GetReportProperty(String, Double.)(...)`

Retrieves a double property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/1c05aa56-5f42-be91-a59e-8d59a5549f45)

#### `GetReportProperty(String, Int32.)(...)`

Retrieves an integer property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/ce8e4ca0-3750-3c9b-5ca8-27e48b45e0bd)

#### `GetReportProperty(String, String.)(...)`

Retrieves a string property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/70f212bb-ee70-e9e9-7ab6-d6a9d46f8de6)

#### `GetStringReportProperties(...)`

Retrieves multiple string report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/5ff72da1-7382-f5d2-eb1d-c99b46239d42)

#### `GetStringUserProperties(...)`

Retrieves all string properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/67120e84-c820-9d52-12ae-c167c595842c)

#### `GetUserProperty(String, Double.)(...)`

Retrieves a double property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/17cc8074-7955-32b5-2b4b-b169c45d4ee0)

#### `GetUserProperty(String, Int32.)(...)`

Retrieves an integer property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/d9364c1d-5b57-6a31-4c13-01124058cfb2)

#### `GetUserProperty(String, String.)(...)`

Retrieves a string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/214a886c-c18c-9b66-d7c9-fe84260962f2)

#### `Insert(...)`

Inserts the load area into the model database.

[Docs](https://developer.tekla.com/topic/en/18/43/6f5284b4-b7a8-d81f-cdb5-d5c8088fc989)

#### `Modify(...)`

Modifies the existing load area in the model database to match the current one. At the moment it is not possible to change the load attachment or the father.

[Docs](https://developer.tekla.com/topic/en/18/43/6f0c6414-0f0d-6b47-fdfe-c2a3fbdbda95)

#### `Select(...)`

Selects a load area from the model database. The ID must be set.

[Docs](https://developer.tekla.com/topic/en/18/43/fc2cfeae-a59e-bdd0-6b87-b9227efd7ca1)

#### `SetCustomObjectType(...)`

Sets a custom object type name for an existing object.

[Docs](https://developer.tekla.com/topic/en/18/43/3d6fb91b-48a9-35ca-d498-526514344089)

#### `SetDynamicStringProperty(...)`

Sets a dynamic string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/4dab0254-09e5-5e61-c28b-1c12afab71a0)

#### `SetLabel(...)`

Sets a label for an object when a new instance is created, this method must be called before Insert. The label is used in plug-ins for identifying the changed object in modification.

[Docs](https://developer.tekla.com/topic/en/18/43/3efcfc44-613a-e66b-9ebd-9f86df4f4036)

#### `SetPhase(...)`

Sets the phase of the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/ed80abcc-0ff7-d081-2229-e651f858a727)

#### `SetUserProperties(...)`

Sets multiple properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/8659da37-d571-f03f-c666-5c9fa3b113f5)

#### `SetUserProperty(String, Double)(...)`

Sets a double property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/5c585b8b-5c2e-7b75-2242-758f17429396)

#### `SetUserProperty(String, Int32)(...)`

Sets an integer property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/dd5b828d-3a87-bb58-5ac9-e1c3b4c2fe4b)

#### `SetUserProperty(String, String)(...)`

Sets a string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/80bf0348-b651-3d4a-862b-49c85bc924ab)

### Properties
- `AutomaticPrimaryAxisWeight` (object, get/set) — The automatic primary axis weight.
- `BoundingBoxDx` (object, get/set) — The dimension of the bounding box in the X direction.
- `BoundingBoxDy` (object, get/set) — The dimension of the bounding box in the Y direction.
- `BoundingBoxDz` (object, get/set) — The dimension of the bounding box in the Z direction.
- `CreateFixedSupportConditionsAutomatically` (object, get/set) — Indicates whether fixed support conditions are created automatically.
- `DistanceA` (object, get/set) — The distance a (for enlarging or reducing the loaded area).
- `FatherId` (object, get/set) — The identifier of the father object that the load is attached to.
- `Group` (object, get/set) — The load group object for the load.
- `Identifier` (object, get/set) — The identifier of the object.
- `IsUpToDate` (object, get/set) — Gets if the object does not have a modification which is not shared.
- `LoadAttachment` (object, get/set) — The load attachment.
- `LoadDispersionAngle` (object, get/set) — The load dispersion angle.
- `LoadForm` (object, get/set) — The load form.
- `ModificationTime` (object, get/set) — Gets latest time of the object was modified or created.
- `P1` (object, get/set) — The load magnitude vector P1. The values are given in the local coordinate system of the load (not in the current coordinate system).
- `P2` (object, get/set) — The load magnitude vector P2. The values are given in the local coordinate system of the load (not in the current coordinate system).
- `P3` (object, get/set) — The load magnitude vector P3. The values are given in the local coordinate system of the load (not in the current coordinate system).
- `P4` (object, get/set) — The load magnitude vector P4. This one cannot be set. The values are given in the local coordinate system of the load (not in the current coordinate system).
- `PartFilter` (object, get/set) — The part filter.
- `PartNames` (object, get/set) — The part names.
- `Position1` (object, get/set) — The first position point.
- `Position2` (object, get/set) — The second position point.
- `Position3` (object, get/set) — The third position point.
- `PrimaryAxisDirection` (object, get/set) — The vector for the primary axis direction.
- `Spanning` (object, get/set) — The load spanning.
- `Weight` (object, get/set) — The weight.

## LoadArea.AreaLoadFormEnum (enum)

The area load form.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/38b0eec5-e459-c5fd-12d3-806d0256c95e)

### Values
- `LOAD_FORM_AREA_PARALLELOGRAM` = `1002` — The area load form is a parallelogram.
- `LOAD_FORM_AREA_TRIANGLE` = `1003` — The area load form is a triangle.

## LoadGroup (class)

The LoadGroup class implements grouping functionality related to all loads. Each load should be assigned to a load group. Each load group contains loads caused by the same action and to which you want to refer collectively. Tekla Structures assumes that all loads in a group Have the same partial safety and other combination factorsHave the same action directionOccur at the same time and all together You may create new load groups, modify existing ones and delete load groups via the .NET interface. You may also change the assigned load group of a load and list all the load groups found in the model via the ModelObjectSelector interface. The model always has at least one load group.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/c33bc83e-1690-718a-c3e9-0a588cbd1943)

### Constructors
- `LoadGroup(...)` — Creates a new load group instance.

### Methods
#### `CompareTo(...)`

Compares Identifiers of model objects.

[Docs](https://developer.tekla.com/topic/en/18/43/79cca356-0c8a-e8fb-463a-34ad4141bec7)

#### `Delete(...)`

Deletes the load group instance with the given ID from the model database.

[Docs](https://developer.tekla.com/topic/en/18/43/fee30bea-0e10-ff61-e973-30bb6b0874bc)

#### `Equals(...)`

Check if Identifiers of model objects are same.

[Docs](https://developer.tekla.com/topic/en/18/43/e4324265-2490-00de-4139-63a8f7acb492)

#### `GetAllReportProperties(...)`

Retrieves all properties for the model object in one hashtable. Type for the returned value must be checked using type casting.

[Docs](https://developer.tekla.com/topic/en/18/43/0708f404-ce77-28e1-4ebe-fa906f68bff8)

#### `GetAllUserProperties(...)`

Retrieves all properties for the model object in one hashtable. Type for the returned value must be checked using type casting.

[Docs](https://developer.tekla.com/topic/en/18/43/63b853a0-0af5-af9c-3ce7-05299664d7f8)

#### `GetChildren(...)`

Returns an enumerator of all the children model objects.

[Docs](https://developer.tekla.com/topic/en/18/43/ce51ec40-310c-6067-a89f-f8d029aab747)

#### `GetCoordinateSystem(...)`

Returns the coordinate system for the given model object.

[Docs](https://developer.tekla.com/topic/en/18/43/8adfe9cd-f6e5-6474-1297-33c6270a7b0a)

#### `GetCustomObjectType(...)`

Gets custom object type name from an existing object.

[Docs](https://developer.tekla.com/topic/en/18/43/d3be2ba3-8533-7165-f2d3-a5aa9df556b8)

#### `GetDoubleReportProperties(...)`

Retrieves multiple double report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/f86f6c96-f738-49ca-16ba-f5634e862ba2)

#### `GetDoubleUserProperties(...)`

Retrieves all double properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/6ead7a63-5f76-0631-d480-d46c27e882aa)

#### `GetDynamicStringProperty(...)`

Gets a dynamic string property from the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/415e86d9-7de6-ea53-ce68-20b3e67b8655)

#### `GetFatherComponent(...)`

Returns the father component of the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/a2c28f3f-52ff-50a8-1b32-10d18db5c31d)

#### `GetHierarchicObjects(...)`

Returns an enumerator of all the connected hierarchic objects.

[Docs](https://developer.tekla.com/topic/en/18/43/cc228ea0-267b-7d90-7441-c9efc1779b08)

#### `GetIntegerReportProperties(...)`

Retrieves multiple integer report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/2623a9c0-2a9c-e233-9daa-59f92ed51445)

#### `GetIntegerUserProperties(...)`

Retrieves all integer properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/1b3c6f06-bc25-a54e-2a88-b60dc4a3c85e)

#### `GetPhase(...)`

Retrieves the phase of the model object (the phase number, the phase name, the phase comment and whether the phase is the current one or not).

[Docs](https://developer.tekla.com/topic/en/18/43/310f8b7a-c120-753a-35c9-f7c73a9806c9)

#### `GetReportProperty(String, Double.)(...)`

Retrieves a double property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/1c05aa56-5f42-be91-a59e-8d59a5549f45)

#### `GetReportProperty(String, Int32.)(...)`

Retrieves an integer property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/ce8e4ca0-3750-3c9b-5ca8-27e48b45e0bd)

#### `GetReportProperty(String, String.)(...)`

Retrieves a string property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/70f212bb-ee70-e9e9-7ab6-d6a9d46f8de6)

#### `GetStringReportProperties(...)`

Retrieves multiple string report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/5ff72da1-7382-f5d2-eb1d-c99b46239d42)

#### `GetStringUserProperties(...)`

Retrieves all string properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/67120e84-c820-9d52-12ae-c167c595842c)

#### `GetUserProperty(String, Double.)(...)`

Retrieves a double property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/17cc8074-7955-32b5-2b4b-b169c45d4ee0)

#### `GetUserProperty(String, Int32.)(...)`

Retrieves an integer property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/d9364c1d-5b57-6a31-4c13-01124058cfb2)

#### `GetUserProperty(String, String.)(...)`

Retrieves a string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/214a886c-c18c-9b66-d7c9-fe84260962f2)

#### `Insert(...)`

Inserts the load group into the model database.

[Docs](https://developer.tekla.com/topic/en/18/43/0f18143b-9736-4bd5-d9ea-fca9ac1ba064)

#### `Modify(...)`

Modifies the existing load group in the model database to match the current one. At the moment it is not possible to change the load attachment or the father.

[Docs](https://developer.tekla.com/topic/en/18/43/26785d2f-fba5-f6ad-ab7c-580d33958791)

#### `Select(...)`

Selects a load group from the model database. The ID must be set.

[Docs](https://developer.tekla.com/topic/en/18/43/478851ec-ca39-10df-20cb-b8bb70e6b4a2)

#### `SetCustomObjectType(...)`

Sets a custom object type name for an existing object.

[Docs](https://developer.tekla.com/topic/en/18/43/3d6fb91b-48a9-35ca-d498-526514344089)

#### `SetDynamicStringProperty(...)`

Sets a dynamic string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/4dab0254-09e5-5e61-c28b-1c12afab71a0)

#### `SetLabel(...)`

Sets a label for an object when a new instance is created, this method must be called before Insert. The label is used in plug-ins for identifying the changed object in modification.

[Docs](https://developer.tekla.com/topic/en/18/43/3efcfc44-613a-e66b-9ebd-9f86df4f4036)

#### `SetPhase(...)`

Sets the phase of the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/ed80abcc-0ff7-d081-2229-e651f858a727)

#### `SetUserProperties(...)`

Sets multiple properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/8659da37-d571-f03f-c666-5c9fa3b113f5)

#### `SetUserProperty(String, Double)(...)`

Sets a double property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/5c585b8b-5c2e-7b75-2242-758f17429396)

#### `SetUserProperty(String, Int32)(...)`

Sets an integer property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/dd5b828d-3a87-bb58-5ac9-e1c3b4c2fe4b)

#### `SetUserProperty(String, String)(...)`

Sets a string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/80bf0348-b651-3d4a-862b-49c85bc924ab)

### Properties
- `Color` (object, get/set) — The load group color. This variable indicates the color with which all loads that belong to this same group are drawn in Tekla Structures views.
- `Compatible` (object, get/set) — This variable is used to store information on which groups are compatible with each other in structural analysis load combination. Load combination is a process in which some simultaneously acting load groups are multiplied by their partial safety factors and combined with each other according to specific rules. To accurately combine loads which have the same load group type, you need to identify which load groups can occur at the same time, i.e. are compatible. This variable stores one integer, and all groups who have this same integer are known to be compatible with each other. Depending on the analysis needs, they may be calculated together, in smaller combinations, or separately.
- `Direction` (object, get/set) — The load group direction.
- `GroupName` (object, get/set) — The load group name. The maximum size is 126 characters.
- `GroupType` (object, get/set) — The load group type.
- `Identifier` (object, get/set) — The identifier of the object.
- `Incompatible` (object, get/set) — This variable is used to store information on which groups exclude each other in structural analysis load combination. Load combination is a process in which some simultaneously acting load groups are multiplied by their partial safety factors and combined with each other according to specific rules. To accurately combine loads which have the same load group type, you need to identify which load groups exclude each other, i.e. are incompatible. This variable stores one integer, and all groups who have this same integer are known to be incompatible with each other.
- `IsUpToDate` (object, get/set) — Gets if the object does not have a modification which is not shared.
- `ModificationTime` (object, get/set) — Gets latest time of the object was modified or created.

## LoadGroup.Colors (enum)

The possible display colors for loads in a load group.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/6f2a89bd-8ba4-973c-a2f5-f7cef79ba412)

### Values
- `BLACK` = `0` — The black display color.
- `WHITE` = `1` — The white display color.
- `RED` = `2` — The red display color.
- `GREEN` = `3` — The green display color.
- `BLUE` = `4` — The blue display color.
- `CYAN` = `5` — The cyan display color.
- `YELLOW` = `6` — The yellow display color.
- `MAGENTA` = `7` — The magenta display color.

## LoadGroup.LoadGroupDirection (enum)

The possible directions.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/b0c729c6-cfde-f097-72c0-1ca150c956e9)

### Values
- `DIR_X` = `1` — The X direction.
- `DIR_Y` = `2` — The Y direction.
- `DIR_Z` = `3` — The Z direction.
- `DIR_NEG_X` = `4` — The negative X direction.
- `DIR_NEG_Y` = `5` — The negative Y direction.
- `DIR_NEG_Z` = `6` — The negative Z direction.

## LoadGroup.LoadGroupType (enum)

The possible types for load groups.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/5bf85407-b90c-398a-2192-3072566c32fd)

### Values
- `EUROCODE_DEFAULT_GROUP` = `1000` — The Eurocode default group type.
- `EUROCODE_SELF_WEIGHT` = `1001` — The Eurocode self weight type.
- `EUROCODE_PERMANENT_LOAD` = `1002` — The Eurocode permanent load type.
- `EUROCODE_PRE_STRESS_LOAD` = `1003` — The Eurocode pre-stress load type.
- `EUROCODE_LIVE_LOAD_HOUSING` = `1004` — The Eurocode live load housing type.
- `EUROCODE_LIVE_LOAD_PUBLIC_BUILDINGS` = `1005` — The Eurocode live load public buildings type.
- `EUROCODE_LIVE_LOAD_THEATRES_RESTAURANTS` = `1006` — The Eurocode live load theatres, restaurants type.
- `EUROCODE_LIVE_LOAD_WAREHOUSES` = `1007` — The Eurocode live load warehouses type.
- `EUROCODE_LIVE_LOAD_STORAGE_BUILDINGS` = `1008` — The Eurocode live load storage buildings type.
- `EUROCODE_TRAFFIC_LOAD_LIGHT_VEHICLES` = `1009` — The Eurocode traffic load light vehicles type.
- `EUROCODE_TRAFFIC_LOAD_HEAVY_VEHICLES` = `1010` — The Eurocode traffic load heavy vehicles type.
- `EUROCODE_TRAFFIC_LOAD_ROOFS` = `1011` — The Eurocode traffic load roofs type.
- `EUROCODE_SNOW_LOAD` = `1012` — The Eurocode snow load type.
- `EUROCODE_WIND_LOAD` = `1013` — The Eurocode wind load type.
- `EUROCODE_FORCES_DUE_TO_TEMPERATURE_EFFECTS` = `1014` — The Eurocode forces due to temperature effects type.
- `EUROCODE_IMPERFECTION_LOAD_DUE_TO_DEAD_LOADS` = `1015` — The Eurocode imperfection load due to dead loads type.
- `EUROCODE_IMPERFECTION_LOAD_DUE_TO_LIVE_LOADS` = `1016` — The Eurocode imperfection load due to live loads type.
- `EUROCODE_IMPERFECTION_LOAD_DUE_TO_SNOW_LOADS` = `1017` — The Eurocode imperfection load due to snow loads type.
- `EUROCODE_ACCIDENTAL_LOAD` = `1018` — The Eurocode accidental load type.
- `EUROCODE_EARTHQUAKE_LOAD` = `1019` — The Eurocode earthquake load type.
- `BRITISH_CODE_DEAD_LOAD_ALONE` = `2001` — The British code dead load alone type.
- `BRITISH_CODE_DEADLOAD` = `2002` — The British code dead load type.
- `BRITISH_CODE_IMPOSED_LOAD` = `2003` — The British code imposed load type.
- `BRITISH_CODE_WIND_LOAD` = `2004` — The British code wind load type.
- `BRITISH_CODE_TEMPERATURE_LOAD` = `2005` — The British code temperature load type.
- `BRITISH_CODE_VERTICAL_CRANE_LOAD` = `2006` — The British code vertical crane load type.
- `BRITISH_CODE_HORIZONTAL_CRANE_LOAD` = `2007` — The British code horizontal crane load type.
- `AISC_DEAD_LOAD_ALONE` = `3001` — The AISC dead load alone type.
- `AISC_DEAD_LOAD` = `3002` — The AISC dead load type.
- `AISC_LIVE_LOAD` = `3003` — The AISC live load type.
- `AISC_ROOF_LIVE_LOAD` = `3004` — The AISC roof live load type.
- `AISC_WIND_LOAD` = `3005` — The AISC wind load type.
- `AISC_SNOW_LOAD` = `3006` — The AISC snow load type.
- `AISC_RAINWATER_LOAD` = `3007` — The AISC rainwater load type.
- `AISC_EARTHQUAKE_LOAD` = `3008` — The AISC earthquake load type.
- `CM66_PERMANENT_LOAD_FR` = `6001` — The CM66 permanent load type.
- `CM66_EXPLOITATION_LOAD_FR` = `6002` — The CM66 exploitation load type.
- `CM66_TEMPERATURE_LOAD_FR` = `6003` — The CM66 temperature load type.
- `CM66_WIND_LOAD_FR` = `6004` — The CM66 wind load type.
- `CM66_SNOW_LOAD_FR` = `6005` — The CM66 snow load type.
- `CM66_SEISMIC_LOAD_HORIZONTAL_FR` = `6006` — The CM66 seismic load horizontal type.
- `CM66_SEISMIC_LOAD_VERTICAL_FR` = `6007` — The CM66 seismic load vertical type.
- `BAEL91_PERMANENT_LOAD_FR` = `7001` — The BAEL91 permanent load type.
- `BAEL91_EXPLOITATION_LOAD_FR` = `7002` — The BAEL91 exploitation load type.
- `BAEL91_TEMPERATURE_LOAD_FR` = `7003` — The BAEL91 temperature load type.
- `BAEL91_WIND_LOAD_FR` = `7004` — The BAEL91 wind load type.
- `BAEL91_SNOW_LOAD_FR` = `7005` — The BAEL91 snow load type.
- `BAEL91_SEISMIC_LOAD_FR` = `7006` — The BAEL91 seismic load type.
- `BAEL91_ACCIDENTAL_LOAD_FR` = `7007` — The BAEL91 accidental load type.
- `UBC_DEAD_LOAD` = `8001` — The UBC dead load type.
- `UBC_LIVE_LOAD` = `8002` — The UBC live load type.
- `UBC_ROOF_LIVE_LOAD` = `8003` — The UBC roof live load type.
- `UBC_WIND_LOAD` = `8004` — The UBC wind load type.
- `UBC_SNOW_LOAD` = `8005` — The UBC snow load type.
- `UBC_TEMPERATURE_LOAD` = `8006` — The UBC temperature load type.
- `UBC_FLUIDS_LOAD` = `8007` — The UBC fluids load type.
- `UBC_SOIL_LOAD` = `8008` — The UBC soil load type.
- `UBC_PONDING_LOAD` = `8009` — The UBC ponding load type.
- `UBC_SEISMIC_LOAD` = `8010` — The UBC seismic load type.
- `IBC_DEAD_LOAD` = `9001` — The IBC dead load type.
- `IBC_LIVE_LOAD` = `9002` — The IBC live load type.
- `IBC_ROOF_LIVE_LOAD` = `9003` — The IBC roof live load type.
- `IBC_WIND_LOAD` = `9004` — The IBC wind load type.
- `IBC_SNOW_LOAD` = `9005` — The IBC snow load type.
- `IBC_TEMPERATURE_LOAD` = `9006` — The IBC temperature load type.
- `IBC_FLUIDS_LOAD` = `9007` — The IBC fluids load type.
- `IBC_SOIL_LOAD` = `9008` — The IBC soil load type.
- `IBC_RAIN_LOAD` = `9009` — The IBC rain load type.
- `IBC_PONDING_LOAD` = `9010` — The IBC ponding load type.
- `IBC_SEISMIC_LOAD` = `9011` — The IBC seismic load type.
- `ACI_DEAD_LOAD` = `9101` — The ACI dead load type.
- `ACI_LIVE_LOAD` = `9102` — The ACI live load type.
- `ACI_ROOF_LIVE_LOAD` = `9103` — The ACI roof live load type.
- `ACI_WIND_LOAD` = `9104` — The ACI wind load type.
- `ACI_SEISMIC_LOAD` = `9105` — The ACI seismic load type.
- `ACI_SNOW_LOAD` = `9106` — The ACI snow load type.
- `ACI_FLUIDS_LOAD` = `9107` — The ACI fluids load type.
- `ACI_SOIL_LOAD` = `9108` — The ACI soil load type.
- `ACI_RAIN_LOAD` = `9109` — The ACI rain load type.
- `ACI_TEMPERATURE_LOAD` = `9110` — The ACI temperature load type.

## LoadLine (class)

The LoadLine class defines a linearly-distributed force or torsion.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/6c2c4fcd-3944-0df3-0dfb-d042662860d1)

### Constructors
- `LoadLine(...)` — Creates a new load line instance.

### Methods
#### `CompareTo(...)`

Compares Identifiers of model objects.

[Docs](https://developer.tekla.com/topic/en/18/43/79cca356-0c8a-e8fb-463a-34ad4141bec7)

#### `Delete(...)`

Deletes the load line instance with the given ID from the model database.

[Docs](https://developer.tekla.com/topic/en/18/43/e99dacfc-d419-8098-143c-3db0f454221f)

#### `Equals(...)`

Check if Identifiers of model objects are same.

[Docs](https://developer.tekla.com/topic/en/18/43/e4324265-2490-00de-4139-63a8f7acb492)

#### `GetAllReportProperties(...)`

Retrieves all properties for the model object in one hashtable. Type for the returned value must be checked using type casting.

[Docs](https://developer.tekla.com/topic/en/18/43/0708f404-ce77-28e1-4ebe-fa906f68bff8)

#### `GetAllUserProperties(...)`

Retrieves all properties for the model object in one hashtable. Type for the returned value must be checked using type casting.

[Docs](https://developer.tekla.com/topic/en/18/43/63b853a0-0af5-af9c-3ce7-05299664d7f8)

#### `GetChildren(...)`

Returns an enumerator of all the children model objects.

[Docs](https://developer.tekla.com/topic/en/18/43/ce51ec40-310c-6067-a89f-f8d029aab747)

#### `GetCoordinateSystem(...)`

Returns the coordinate system for the given model object.

[Docs](https://developer.tekla.com/topic/en/18/43/8adfe9cd-f6e5-6474-1297-33c6270a7b0a)

#### `GetCustomObjectType(...)`

Gets custom object type name from an existing object.

[Docs](https://developer.tekla.com/topic/en/18/43/d3be2ba3-8533-7165-f2d3-a5aa9df556b8)

#### `GetDoubleReportProperties(...)`

Retrieves multiple double report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/f86f6c96-f738-49ca-16ba-f5634e862ba2)

#### `GetDoubleUserProperties(...)`

Retrieves all double properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/6ead7a63-5f76-0631-d480-d46c27e882aa)

#### `GetDynamicStringProperty(...)`

Gets a dynamic string property from the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/415e86d9-7de6-ea53-ce68-20b3e67b8655)

#### `GetFatherComponent(...)`

Returns the father component of the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/a2c28f3f-52ff-50a8-1b32-10d18db5c31d)

#### `GetHierarchicObjects(...)`

Returns an enumerator of all the connected hierarchic objects.

[Docs](https://developer.tekla.com/topic/en/18/43/cc228ea0-267b-7d90-7441-c9efc1779b08)

#### `GetIntegerReportProperties(...)`

Retrieves multiple integer report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/2623a9c0-2a9c-e233-9daa-59f92ed51445)

#### `GetIntegerUserProperties(...)`

Retrieves all integer properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/1b3c6f06-bc25-a54e-2a88-b60dc4a3c85e)

#### `GetPhase(...)`

Retrieves the phase of the model object (the phase number, the phase name, the phase comment and whether the phase is the current one or not).

[Docs](https://developer.tekla.com/topic/en/18/43/310f8b7a-c120-753a-35c9-f7c73a9806c9)

#### `GetReportProperty(String, Double.)(...)`

Retrieves a double property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/1c05aa56-5f42-be91-a59e-8d59a5549f45)

#### `GetReportProperty(String, Int32.)(...)`

Retrieves an integer property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/ce8e4ca0-3750-3c9b-5ca8-27e48b45e0bd)

#### `GetReportProperty(String, String.)(...)`

Retrieves a string property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/70f212bb-ee70-e9e9-7ab6-d6a9d46f8de6)

#### `GetStringReportProperties(...)`

Retrieves multiple string report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/5ff72da1-7382-f5d2-eb1d-c99b46239d42)

#### `GetStringUserProperties(...)`

Retrieves all string properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/67120e84-c820-9d52-12ae-c167c595842c)

#### `GetUserProperty(String, Double.)(...)`

Retrieves a double property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/17cc8074-7955-32b5-2b4b-b169c45d4ee0)

#### `GetUserProperty(String, Int32.)(...)`

Retrieves an integer property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/d9364c1d-5b57-6a31-4c13-01124058cfb2)

#### `GetUserProperty(String, String.)(...)`

Retrieves a string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/214a886c-c18c-9b66-d7c9-fe84260962f2)

#### `Insert(...)`

Inserts the load line into the model database.

[Docs](https://developer.tekla.com/topic/en/18/43/aa00c4dc-0874-7830-b834-0f442ba9bc6b)

#### `Modify(...)`

Modifies the existing load line in the model database to match the current one. At the moment it is not possible to change the load attachment or the father.

[Docs](https://developer.tekla.com/topic/en/18/43/6c276991-260e-37b4-7e83-26c620053e42)

#### `Select(...)`

Selects a load line from the model database. The ID must be set.

[Docs](https://developer.tekla.com/topic/en/18/43/d3ead8dd-68a2-f871-ca48-50c1f7784fed)

#### `SetCustomObjectType(...)`

Sets a custom object type name for an existing object.

[Docs](https://developer.tekla.com/topic/en/18/43/3d6fb91b-48a9-35ca-d498-526514344089)

#### `SetDynamicStringProperty(...)`

Sets a dynamic string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/4dab0254-09e5-5e61-c28b-1c12afab71a0)

#### `SetLabel(...)`

Sets a label for an object when a new instance is created, this method must be called before Insert. The label is used in plug-ins for identifying the changed object in modification.

[Docs](https://developer.tekla.com/topic/en/18/43/3efcfc44-613a-e66b-9ebd-9f86df4f4036)

#### `SetPhase(...)`

Sets the phase of the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/ed80abcc-0ff7-d081-2229-e651f858a727)

#### `SetUserProperties(...)`

Sets multiple properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/8659da37-d571-f03f-c666-5c9fa3b113f5)

#### `SetUserProperty(String, Double)(...)`

Sets a double property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/5c585b8b-5c2e-7b75-2242-758f17429396)

#### `SetUserProperty(String, Int32)(...)`

Sets an integer property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/dd5b828d-3a87-bb58-5ac9-e1c3b4c2fe4b)

#### `SetUserProperty(String, String)(...)`

Sets a string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/80bf0348-b651-3d4a-862b-49c85bc924ab)

### Properties
- `AutomaticPrimaryAxisWeight` (object, get/set) — The automatic primary axis weight.
- `BoundingBoxDx` (object, get/set) — The dimension of the bounding box in the X direction.
- `BoundingBoxDy` (object, get/set) — The dimension of the bounding box in the Y direction.
- `BoundingBoxDz` (object, get/set) — The dimension of the bounding box in the Z direction.
- `CreateFixedSupportConditionsAutomatically` (object, get/set) — Indicates whether fixed support conditions are created automatically.
- `DistanceA` (object, get/set) — The distance a (for shortening or dividing the length of a line load).
- `DistanceB` (object, get/set) — The distance b (for shortening or dividing the length of a line load).
- `FatherId` (object, get/set) — The identifier of the father object that the load is attached to.
- `Group` (object, get/set) — The load group object for the load.
- `Identifier` (object, get/set) — The identifier of the object.
- `IsUpToDate` (object, get/set) — Gets if the object does not have a modification which is not shared.
- `LoadAttachment` (object, get/set) — The load attachment.
- `LoadDispersionAngle` (object, get/set) — The load dispersion angle.
- `LoadForm` (object, get/set) — The load form.
- `ModificationTime` (object, get/set) — Gets latest time of the object was modified or created.
- `P1` (object, get/set) — The load magnitude vector at the start position. The values are given in the local coordinate system of the load (not in the current coordinate system).
- `P2` (object, get/set) — The load magnitude vector at the end position. The values are given in the local coordinate system of the load (not in the current coordinate system).
- `PartFilter` (object, get/set) — The part filter.
- `PartNames` (object, get/set) — The part names.
- `Position1` (object, get/set) — The first input position point.
- `Position2` (object, get/set) — The second input position point.
- `PrimaryAxisDirection` (object, get/set) — The vector for the primary axis direction.
- `Spanning` (object, get/set) — The load spanning.
- `Torsion1` (object, get/set) — The first torsion.
- `Torsion2` (object, get/set) — The second torsion.
- `Weight` (object, get/set) — The weight.

## LoadLine.LineLoadFormEnum (enum)

The line load form.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/73f495f9-6e26-28a8-1ef6-82e1c7b2b808)

### Values
- `LOAD_FORM_LINE_1` = `1001` — The load magnitude is uniform across the loaded length.
- `LOAD_FORM_LINE_2` = `1002` — The load has different magnitudes at the ends of the loaded length. The magnitude changes linearly between the ends.
- `LOAD_FORM_LINE_3` = `1003` — The load magnitude changes linearly, from zero at the ends of the loaded length, to a fixed value in the middle of the loaded length.
- `LOAD_FORM_LINE_4` = `1004` — The load magnitude changes linearly, from zero at one end of the loaded length, through two (different) values, back to zero at the other end.

## LoadPoint (class)

The LoadPoint class defines a concentrated force or a bending moment.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/2d81d728-67f5-5d87-579e-5b33cdeb9b83)

### Constructors
- `LoadPoint(...)` — Creates a new load point instance.

### Methods
#### `CompareTo(...)`

Compares Identifiers of model objects.

[Docs](https://developer.tekla.com/topic/en/18/43/79cca356-0c8a-e8fb-463a-34ad4141bec7)

#### `Delete(...)`

Deletes the load point instance with the given ID from the model database.

[Docs](https://developer.tekla.com/topic/en/18/43/8a29640f-e0e9-58e9-729a-b10a226dbb4e)

#### `Equals(...)`

Check if Identifiers of model objects are same.

[Docs](https://developer.tekla.com/topic/en/18/43/e4324265-2490-00de-4139-63a8f7acb492)

#### `GetAllReportProperties(...)`

Retrieves all properties for the model object in one hashtable. Type for the returned value must be checked using type casting.

[Docs](https://developer.tekla.com/topic/en/18/43/0708f404-ce77-28e1-4ebe-fa906f68bff8)

#### `GetAllUserProperties(...)`

Retrieves all properties for the model object in one hashtable. Type for the returned value must be checked using type casting.

[Docs](https://developer.tekla.com/topic/en/18/43/63b853a0-0af5-af9c-3ce7-05299664d7f8)

#### `GetChildren(...)`

Returns an enumerator of all the children model objects.

[Docs](https://developer.tekla.com/topic/en/18/43/ce51ec40-310c-6067-a89f-f8d029aab747)

#### `GetCoordinateSystem(...)`

Returns the coordinate system for the given model object.

[Docs](https://developer.tekla.com/topic/en/18/43/8adfe9cd-f6e5-6474-1297-33c6270a7b0a)

#### `GetCustomObjectType(...)`

Gets custom object type name from an existing object.

[Docs](https://developer.tekla.com/topic/en/18/43/d3be2ba3-8533-7165-f2d3-a5aa9df556b8)

#### `GetDoubleReportProperties(...)`

Retrieves multiple double report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/f86f6c96-f738-49ca-16ba-f5634e862ba2)

#### `GetDoubleUserProperties(...)`

Retrieves all double properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/6ead7a63-5f76-0631-d480-d46c27e882aa)

#### `GetDynamicStringProperty(...)`

Gets a dynamic string property from the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/415e86d9-7de6-ea53-ce68-20b3e67b8655)

#### `GetFatherComponent(...)`

Returns the father component of the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/a2c28f3f-52ff-50a8-1b32-10d18db5c31d)

#### `GetHierarchicObjects(...)`

Returns an enumerator of all the connected hierarchic objects.

[Docs](https://developer.tekla.com/topic/en/18/43/cc228ea0-267b-7d90-7441-c9efc1779b08)

#### `GetIntegerReportProperties(...)`

Retrieves multiple integer report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/2623a9c0-2a9c-e233-9daa-59f92ed51445)

#### `GetIntegerUserProperties(...)`

Retrieves all integer properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/1b3c6f06-bc25-a54e-2a88-b60dc4a3c85e)

#### `GetPhase(...)`

Retrieves the phase of the model object (the phase number, the phase name, the phase comment and whether the phase is the current one or not).

[Docs](https://developer.tekla.com/topic/en/18/43/310f8b7a-c120-753a-35c9-f7c73a9806c9)

#### `GetReportProperty(String, Double.)(...)`

Retrieves a double property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/1c05aa56-5f42-be91-a59e-8d59a5549f45)

#### `GetReportProperty(String, Int32.)(...)`

Retrieves an integer property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/ce8e4ca0-3750-3c9b-5ca8-27e48b45e0bd)

#### `GetReportProperty(String, String.)(...)`

Retrieves a string property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/70f212bb-ee70-e9e9-7ab6-d6a9d46f8de6)

#### `GetStringReportProperties(...)`

Retrieves multiple string report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/5ff72da1-7382-f5d2-eb1d-c99b46239d42)

#### `GetStringUserProperties(...)`

Retrieves all string properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/67120e84-c820-9d52-12ae-c167c595842c)

#### `GetUserProperty(String, Double.)(...)`

Retrieves a double property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/17cc8074-7955-32b5-2b4b-b169c45d4ee0)

#### `GetUserProperty(String, Int32.)(...)`

Retrieves an integer property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/d9364c1d-5b57-6a31-4c13-01124058cfb2)

#### `GetUserProperty(String, String.)(...)`

Retrieves a string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/214a886c-c18c-9b66-d7c9-fe84260962f2)

#### `Insert(...)`

Inserts the load point into the model database.

[Docs](https://developer.tekla.com/topic/en/18/43/09a45fdb-41a9-4aa6-2b1d-d5b6500d6e6a)

#### `Modify(...)`

Modifies the existing load point in the model database to match the current one. At the moment it is not possible to change the load attachment or the father.

[Docs](https://developer.tekla.com/topic/en/18/43/bae65474-9443-17c8-6c12-7eb6956e38ed)

#### `Select(...)`

Selects a load point from the model database. The ID must be set.

[Docs](https://developer.tekla.com/topic/en/18/43/2c61bdf7-57e7-9e70-f592-192e8d3f9037)

#### `SetCustomObjectType(...)`

Sets a custom object type name for an existing object.

[Docs](https://developer.tekla.com/topic/en/18/43/3d6fb91b-48a9-35ca-d498-526514344089)

#### `SetDynamicStringProperty(...)`

Sets a dynamic string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/4dab0254-09e5-5e61-c28b-1c12afab71a0)

#### `SetLabel(...)`

Sets a label for an object when a new instance is created, this method must be called before Insert. The label is used in plug-ins for identifying the changed object in modification.

[Docs](https://developer.tekla.com/topic/en/18/43/3efcfc44-613a-e66b-9ebd-9f86df4f4036)

#### `SetPhase(...)`

Sets the phase of the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/ed80abcc-0ff7-d081-2229-e651f858a727)

#### `SetUserProperties(...)`

Sets multiple properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/8659da37-d571-f03f-c666-5c9fa3b113f5)

#### `SetUserProperty(String, Double)(...)`

Sets a double property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/5c585b8b-5c2e-7b75-2242-758f17429396)

#### `SetUserProperty(String, Int32)(...)`

Sets an integer property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/dd5b828d-3a87-bb58-5ac9-e1c3b4c2fe4b)

#### `SetUserProperty(String, String)(...)`

Sets a string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/80bf0348-b651-3d4a-862b-49c85bc924ab)

### Properties
- `AutomaticPrimaryAxisWeight` (object, get/set) — The automatic primary axis weight.
- `BoundingBoxDx` (object, get/set) — The dimension of the bounding box in the X direction.
- `BoundingBoxDy` (object, get/set) — The dimension of the bounding box in the Y direction.
- `BoundingBoxDz` (object, get/set) — The dimension of the bounding box in the Z direction.
- `CreateFixedSupportConditionsAutomatically` (object, get/set) — Indicates whether fixed support conditions are created automatically.
- `FatherId` (object, get/set) — The identifier of the father object that the load is attached to.
- `Group` (object, get/set) — The load group object for the load.
- `Identifier` (object, get/set) — The identifier of the object.
- `IsUpToDate` (object, get/set) — Gets if the object does not have a modification which is not shared.
- `LoadAttachment` (object, get/set) — The load attachment.
- `LoadDispersionAngle` (object, get/set) — The load dispersion angle.
- `ModificationTime` (object, get/set) — Gets latest time of the object was modified or created.
- `Moment` (object, get/set) — The moment vector. The values are given in the local coordinate system of the load (not in the current coordinate system).
- `P` (object, get/set) — The load vector. The values are given in the local coordinate system of the load (not in the current coordinate system).
- `PartFilter` (object, get/set) — The part filter.
- `PartNames` (object, get/set) — The part names.
- `Position` (object, get/set) — The position.
- `PrimaryAxisDirection` (object, get/set) — The vector for the primary axis direction.
- `Spanning` (object, get/set) — The load spanning.
- `Weight` (object, get/set) — The weight.

## LoadTemperature (class)

The LoadTemperature class defines a temperature change in a part, or a temperature difference between two part surfaces.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/65ff2768-3dca-366e-b304-deef114b05f8)

### Constructors
- `LoadTemperature(...)` — Initializes a new instance of the LoadTemperature class.

### Methods
#### `CompareTo(...)`

Compares Identifiers of model objects.

[Docs](https://developer.tekla.com/topic/en/18/43/79cca356-0c8a-e8fb-463a-34ad4141bec7)

#### `Delete(...)`

Deletes the load temperature instance with the given ID from the model database.

[Docs](https://developer.tekla.com/topic/en/18/43/86fd0ebe-4e5a-a137-7202-aa66517436bf)

#### `Equals(...)`

Check if Identifiers of model objects are same.

[Docs](https://developer.tekla.com/topic/en/18/43/e4324265-2490-00de-4139-63a8f7acb492)

#### `GetAllReportProperties(...)`

Retrieves all properties for the model object in one hashtable. Type for the returned value must be checked using type casting.

[Docs](https://developer.tekla.com/topic/en/18/43/0708f404-ce77-28e1-4ebe-fa906f68bff8)

#### `GetAllUserProperties(...)`

Retrieves all properties for the model object in one hashtable. Type for the returned value must be checked using type casting.

[Docs](https://developer.tekla.com/topic/en/18/43/63b853a0-0af5-af9c-3ce7-05299664d7f8)

#### `GetChildren(...)`

Returns an enumerator of all the children model objects.

[Docs](https://developer.tekla.com/topic/en/18/43/ce51ec40-310c-6067-a89f-f8d029aab747)

#### `GetCoordinateSystem(...)`

Returns the coordinate system for the given model object.

[Docs](https://developer.tekla.com/topic/en/18/43/8adfe9cd-f6e5-6474-1297-33c6270a7b0a)

#### `GetCustomObjectType(...)`

Gets custom object type name from an existing object.

[Docs](https://developer.tekla.com/topic/en/18/43/d3be2ba3-8533-7165-f2d3-a5aa9df556b8)

#### `GetDoubleReportProperties(...)`

Retrieves multiple double report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/f86f6c96-f738-49ca-16ba-f5634e862ba2)

#### `GetDoubleUserProperties(...)`

Retrieves all double properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/6ead7a63-5f76-0631-d480-d46c27e882aa)

#### `GetDynamicStringProperty(...)`

Gets a dynamic string property from the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/415e86d9-7de6-ea53-ce68-20b3e67b8655)

#### `GetFatherComponent(...)`

Returns the father component of the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/a2c28f3f-52ff-50a8-1b32-10d18db5c31d)

#### `GetHierarchicObjects(...)`

Returns an enumerator of all the connected hierarchic objects.

[Docs](https://developer.tekla.com/topic/en/18/43/cc228ea0-267b-7d90-7441-c9efc1779b08)

#### `GetIntegerReportProperties(...)`

Retrieves multiple integer report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/2623a9c0-2a9c-e233-9daa-59f92ed51445)

#### `GetIntegerUserProperties(...)`

Retrieves all integer properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/1b3c6f06-bc25-a54e-2a88-b60dc4a3c85e)

#### `GetPhase(...)`

Retrieves the phase of the model object (the phase number, the phase name, the phase comment and whether the phase is the current one or not).

[Docs](https://developer.tekla.com/topic/en/18/43/310f8b7a-c120-753a-35c9-f7c73a9806c9)

#### `GetReportProperty(String, Double.)(...)`

Retrieves a double property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/1c05aa56-5f42-be91-a59e-8d59a5549f45)

#### `GetReportProperty(String, Int32.)(...)`

Retrieves an integer property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/ce8e4ca0-3750-3c9b-5ca8-27e48b45e0bd)

#### `GetReportProperty(String, String.)(...)`

Retrieves a string property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/70f212bb-ee70-e9e9-7ab6-d6a9d46f8de6)

#### `GetStringReportProperties(...)`

Retrieves multiple string report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/5ff72da1-7382-f5d2-eb1d-c99b46239d42)

#### `GetStringUserProperties(...)`

Retrieves all string properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/67120e84-c820-9d52-12ae-c167c595842c)

#### `GetUserProperty(String, Double.)(...)`

Retrieves a double property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/17cc8074-7955-32b5-2b4b-b169c45d4ee0)

#### `GetUserProperty(String, Int32.)(...)`

Retrieves an integer property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/d9364c1d-5b57-6a31-4c13-01124058cfb2)

#### `GetUserProperty(String, String.)(...)`

Retrieves a string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/214a886c-c18c-9b66-d7c9-fe84260962f2)

#### `Insert(...)`

Inserts the load temperature into the model database.

[Docs](https://developer.tekla.com/topic/en/18/43/9e0802de-2abb-c07c-b6cb-ac1858472d3c)

#### `Modify(...)`

Modifies the existing load temperature in the model database to match the current one. At the moment it is not possible to change the load attachment or the father.

[Docs](https://developer.tekla.com/topic/en/18/43/5d3aee68-fbf0-4c0a-9bfc-c6d8c338a613)

#### `Select(...)`

Selects a load temperature from the model database. The ID must be set.

[Docs](https://developer.tekla.com/topic/en/18/43/eb429123-7a5a-ac6f-e0ee-2dc8abd79108)

#### `SetCustomObjectType(...)`

Sets a custom object type name for an existing object.

[Docs](https://developer.tekla.com/topic/en/18/43/3d6fb91b-48a9-35ca-d498-526514344089)

#### `SetDynamicStringProperty(...)`

Sets a dynamic string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/4dab0254-09e5-5e61-c28b-1c12afab71a0)

#### `SetLabel(...)`

Sets a label for an object when a new instance is created, this method must be called before Insert. The label is used in plug-ins for identifying the changed object in modification.

[Docs](https://developer.tekla.com/topic/en/18/43/3efcfc44-613a-e66b-9ebd-9f86df4f4036)

#### `SetPhase(...)`

Sets the phase of the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/ed80abcc-0ff7-d081-2229-e651f858a727)

#### `SetUserProperties(...)`

Sets multiple properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/8659da37-d571-f03f-c666-5c9fa3b113f5)

#### `SetUserProperty(String, Double)(...)`

Sets a double property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/5c585b8b-5c2e-7b75-2242-758f17429396)

#### `SetUserProperty(String, Int32)(...)`

Sets an integer property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/dd5b828d-3a87-bb58-5ac9-e1c3b4c2fe4b)

#### `SetUserProperty(String, String)(...)`

Sets a string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/80bf0348-b651-3d4a-862b-49c85bc924ab)

### Properties
- `AutomaticPrimaryAxisWeight` (object, get/set) — Property not supported in LoadTemperature class.
- `BoundingBoxDx` (object, get/set) — The dimension of the bounding box in the X direction.
- `BoundingBoxDy` (object, get/set) — The dimension of the bounding box in the Y direction.
- `BoundingBoxDz` (object, get/set) — The dimension of the bounding box in the Z direction.
- `CreateFixedSupportConditionsAutomatically` (object, get/set) — Property not supported in LoadTemperature class.
- `FatherId` (object, get/set) — The identifier of the father object that the load is attached to.
- `Group` (object, get/set) — The load group object for the load.
- `Identifier` (object, get/set) — The identifier of the object.
- `InitialAxialElongation` (object, get/set) — Gets or sets the initial axial elongation.
- `IsUpToDate` (object, get/set) — Gets if the object does not have a modification which is not shared.
- `LoadAttachment` (object, get/set) — The load attachment.
- `LoadDispersionAngle` (object, get/set) — Property not supported in LoadTemperature class.
- `ModificationTime` (object, get/set) — Gets latest time of the object was modified or created.
- `PartFilter` (object, get/set) — The part filter.
- `PartNames` (object, get/set) — The part names.
- `Position1` (object, get/set) — Gets or sets the first input position point.
- `Position2` (object, get/set) — Gets or sets the second input position point.
- `PrimaryAxisDirection` (object, get/set) — Property not supported in LoadTemperature class.
- `Spanning` (object, get/set) — Property not supported in LoadTemperature class.
- `TemperatureChangeForAxialElongation` (object, get/set) — Gets or sets the temperature change for axial elongation.
- `TemperatureDifferentialSideToSide` (object, get/set) — Gets or sets the temperature differential from side to side.
- `TemperatureDifferentialTopToBottom` (object, get/set) — Gets or sets the temperature differential from top to bottom.
- `Weight` (object, get/set) — Property not supported in LoadTemperature class.

## LoadUniform (class)

The LoadUniform class defines a uniformly-distributed force bounded by a polygon.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/f2fcb15d-f861-c351-a847-901b47057015)

### Constructors
- `LoadUniform(...)` — Creates a new load uniform instance.

### Methods
#### `CompareTo(...)`

Compares Identifiers of model objects.

[Docs](https://developer.tekla.com/topic/en/18/43/79cca356-0c8a-e8fb-463a-34ad4141bec7)

#### `Delete(...)`

Deletes the load uniform instance with the given ID from the model database.

[Docs](https://developer.tekla.com/topic/en/18/43/d7d9ce37-2030-7445-e7ee-7df91162a114)

#### `Equals(...)`

Check if Identifiers of model objects are same.

[Docs](https://developer.tekla.com/topic/en/18/43/e4324265-2490-00de-4139-63a8f7acb492)

#### `GetAllReportProperties(...)`

Retrieves all properties for the model object in one hashtable. Type for the returned value must be checked using type casting.

[Docs](https://developer.tekla.com/topic/en/18/43/0708f404-ce77-28e1-4ebe-fa906f68bff8)

#### `GetAllUserProperties(...)`

Retrieves all properties for the model object in one hashtable. Type for the returned value must be checked using type casting.

[Docs](https://developer.tekla.com/topic/en/18/43/63b853a0-0af5-af9c-3ce7-05299664d7f8)

#### `GetChildren(...)`

Returns an enumerator of all the children model objects.

[Docs](https://developer.tekla.com/topic/en/18/43/ce51ec40-310c-6067-a89f-f8d029aab747)

#### `GetCoordinateSystem(...)`

Returns the coordinate system for the given model object.

[Docs](https://developer.tekla.com/topic/en/18/43/8adfe9cd-f6e5-6474-1297-33c6270a7b0a)

#### `GetCustomObjectType(...)`

Gets custom object type name from an existing object.

[Docs](https://developer.tekla.com/topic/en/18/43/d3be2ba3-8533-7165-f2d3-a5aa9df556b8)

#### `GetDoubleReportProperties(...)`

Retrieves multiple double report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/f86f6c96-f738-49ca-16ba-f5634e862ba2)

#### `GetDoubleUserProperties(...)`

Retrieves all double properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/6ead7a63-5f76-0631-d480-d46c27e882aa)

#### `GetDynamicStringProperty(...)`

Gets a dynamic string property from the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/415e86d9-7de6-ea53-ce68-20b3e67b8655)

#### `GetFatherComponent(...)`

Returns the father component of the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/a2c28f3f-52ff-50a8-1b32-10d18db5c31d)

#### `GetHierarchicObjects(...)`

Returns an enumerator of all the connected hierarchic objects.

[Docs](https://developer.tekla.com/topic/en/18/43/cc228ea0-267b-7d90-7441-c9efc1779b08)

#### `GetIntegerReportProperties(...)`

Retrieves multiple integer report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/2623a9c0-2a9c-e233-9daa-59f92ed51445)

#### `GetIntegerUserProperties(...)`

Retrieves all integer properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/1b3c6f06-bc25-a54e-2a88-b60dc4a3c85e)

#### `GetPhase(...)`

Retrieves the phase of the model object (the phase number, the phase name, the phase comment and whether the phase is the current one or not).

[Docs](https://developer.tekla.com/topic/en/18/43/310f8b7a-c120-753a-35c9-f7c73a9806c9)

#### `GetReportProperty(String, Double.)(...)`

Retrieves a double property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/1c05aa56-5f42-be91-a59e-8d59a5549f45)

#### `GetReportProperty(String, Int32.)(...)`

Retrieves an integer property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/ce8e4ca0-3750-3c9b-5ca8-27e48b45e0bd)

#### `GetReportProperty(String, String.)(...)`

Retrieves a string property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/70f212bb-ee70-e9e9-7ab6-d6a9d46f8de6)

#### `GetStringReportProperties(...)`

Retrieves multiple string report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/5ff72da1-7382-f5d2-eb1d-c99b46239d42)

#### `GetStringUserProperties(...)`

Retrieves all string properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/67120e84-c820-9d52-12ae-c167c595842c)

#### `GetUserProperty(String, Double.)(...)`

Retrieves a double property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/17cc8074-7955-32b5-2b4b-b169c45d4ee0)

#### `GetUserProperty(String, Int32.)(...)`

Retrieves an integer property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/d9364c1d-5b57-6a31-4c13-01124058cfb2)

#### `GetUserProperty(String, String.)(...)`

Retrieves a string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/214a886c-c18c-9b66-d7c9-fe84260962f2)

#### `Insert(...)`

Inserts the load uniform into the model database.

[Docs](https://developer.tekla.com/topic/en/18/43/b9a1abbe-be5f-c71f-26ae-adb571b6c02d)

#### `Modify(...)`

Modifies the existing load uniform in the model database to match the current one. At the moment it is not possible to change the load attachment or the father.

[Docs](https://developer.tekla.com/topic/en/18/43/983c8b6a-f91a-4659-843a-e561c2c3ab8d)

#### `Select(...)`

Selects a load uniform from the model database. The ID must be set.

[Docs](https://developer.tekla.com/topic/en/18/43/351f4286-0bee-4c47-5eaf-f66161531f61)

#### `SetCustomObjectType(...)`

Sets a custom object type name for an existing object.

[Docs](https://developer.tekla.com/topic/en/18/43/3d6fb91b-48a9-35ca-d498-526514344089)

#### `SetDynamicStringProperty(...)`

Sets a dynamic string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/4dab0254-09e5-5e61-c28b-1c12afab71a0)

#### `SetLabel(...)`

Sets a label for an object when a new instance is created, this method must be called before Insert. The label is used in plug-ins for identifying the changed object in modification.

[Docs](https://developer.tekla.com/topic/en/18/43/3efcfc44-613a-e66b-9ebd-9f86df4f4036)

#### `SetPhase(...)`

Sets the phase of the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/ed80abcc-0ff7-d081-2229-e651f858a727)

#### `SetUserProperties(...)`

Sets multiple properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/8659da37-d571-f03f-c666-5c9fa3b113f5)

#### `SetUserProperty(String, Double)(...)`

Sets a double property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/5c585b8b-5c2e-7b75-2242-758f17429396)

#### `SetUserProperty(String, Int32)(...)`

Sets an integer property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/dd5b828d-3a87-bb58-5ac9-e1c3b4c2fe4b)

#### `SetUserProperty(String, String)(...)`

Sets a string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/80bf0348-b651-3d4a-862b-49c85bc924ab)

### Properties
- `AutomaticPrimaryAxisWeight` (object, get/set) — The automatic primary axis weight.
- `BoundingBoxDx` (object, get/set) — The dimension of the bounding box in the X direction.
- `BoundingBoxDy` (object, get/set) — The dimension of the bounding box in the Y direction.
- `BoundingBoxDz` (object, get/set) — The dimension of the bounding box in the Z direction.
- `CreateFixedSupportConditionsAutomatically` (object, get/set) — Indicates whether fixed support conditions are created automatically.
- `DistanceA` (object, get/set) — The distance a (for enlarging or reducing the loaded area).
- `FatherId` (object, get/set) — The identifier of the father object that the load is attached to.
- `Group` (object, get/set) — The load group object for the load.
- `Identifier` (object, get/set) — The identifier of the object.
- `IsUpToDate` (object, get/set) — Gets if the object does not have a modification which is not shared.
- `LoadAttachment` (object, get/set) — The load attachment.
- `LoadDispersionAngle` (object, get/set) — The load dispersion angle.
- `ModificationTime` (object, get/set) — Gets latest time of the object was modified or created.
- `P1` (object, get/set) — The load magnitude vector. The values are given in the local coordinate system of the load (not in the current coordinate system).
- `PartFilter` (object, get/set) — The part filter.
- `PartNames` (object, get/set) — The part names.
- `Polygon` (object, get/set) — The position points for the uniform load.
- `PrimaryAxisDirection` (object, get/set) — The vector for the primary axis direction.
- `Spanning` (object, get/set) — The load spanning.
- `Weight` (object, get/set) — The weight.

## LoftedPlate (class)

This class represents a lofted plate

[Vendor docs](https://developer.tekla.com/topic/en/18/43/a0265b09-ae24-4aa3-6d43-d03a19f8f923)

### Constructors
- `LoftedPlate(...)` — Initializes a new instance of the LoftedPlate class.

### Methods
#### `CompareTo(Object)(...)`

Compares Identifiers of model objects.

[Docs](https://developer.tekla.com/topic/en/18/43/79cca356-0c8a-e8fb-463a-34ad4141bec7)

#### `CompareTo(Part)(...)`

Compares the instantiated part with another one.

[Docs](https://developer.tekla.com/topic/en/18/43/e8632c0d-89cf-9aab-6532-f62f9873067b)

#### `Delete(...)`

Deletes the lofted plate with this instance identifier from the model database.

[Docs](https://developer.tekla.com/topic/en/18/43/fa454111-c160-4da5-c76f-2c20e76691de)

#### `Equals(...)`

Check if Identifiers of model objects are same.

[Docs](https://developer.tekla.com/topic/en/18/43/e4324265-2490-00de-4139-63a8f7acb492)

#### `GetAllReportProperties(...)`

Retrieves all properties for the model object in one hashtable. Type for the returned value must be checked using type casting.

[Docs](https://developer.tekla.com/topic/en/18/43/0708f404-ce77-28e1-4ebe-fa906f68bff8)

#### `GetAllUserProperties(...)`

Retrieves all properties for the model object in one hashtable. Type for the returned value must be checked using type casting.

[Docs](https://developer.tekla.com/topic/en/18/43/63b853a0-0af5-af9c-3ce7-05299664d7f8)

#### `GetAssembly(...)`

Returns the assembly that the part belongs to.

[Docs](https://developer.tekla.com/topic/en/18/43/bda18562-45a1-1355-42ed-6044af0b18cb)

#### `GetBolts(...)`

Returns an enumerator of all the connected bolts.

[Docs](https://developer.tekla.com/topic/en/18/43/dbc6b475-7461-7ec4-e8ec-7b7d3d8ff429)

#### `GetBooleans(...)`

Returns an enumerator of all the connected boolean objects.

[Docs](https://developer.tekla.com/topic/en/18/43/ae180e43-88e2-d759-0ae0-567d52681a2a)

#### `GetCenterLine(...)`

Returns the center line for the given part.

[Docs](https://developer.tekla.com/topic/en/18/43/bdc886b8-af7a-722c-dcb8-fddb2f250a83)

#### `GetChildren(...)`

Returns an enumerator of all the children model objects.

[Docs](https://developer.tekla.com/topic/en/18/43/ce51ec40-310c-6067-a89f-f8d029aab747)

#### `GetComponents(...)`

Returns an enumerator of all the connected components, connections, seams and details inherited from the base component.

[Docs](https://developer.tekla.com/topic/en/18/43/ed57941c-2cd8-3b78-6a6e-848b8a920298)

#### `GetCoordinateSystem(...)`

Returns the coordinate system for the given model object.

[Docs](https://developer.tekla.com/topic/en/18/43/8adfe9cd-f6e5-6474-1297-33c6270a7b0a)

#### `GetCustomObjectType(...)`

Gets custom object type name from an existing object.

[Docs](https://developer.tekla.com/topic/en/18/43/d3be2ba3-8533-7165-f2d3-a5aa9df556b8)

#### `GetDSTVCoordinateSystem(...)`

Get DSTV coordinate system.

[Docs](https://developer.tekla.com/topic/en/18/43/9e70005f-c701-ab99-8fa0-4fce47b1ab05)

#### `GetDoubleReportProperties(...)`

Retrieves multiple double report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/f86f6c96-f738-49ca-16ba-f5634e862ba2)

#### `GetDoubleUserProperties(...)`

Retrieves all double properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/6ead7a63-5f76-0631-d480-d46c27e882aa)

#### `GetDynamicStringProperty(...)`

Gets a dynamic string property from the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/415e86d9-7de6-ea53-ce68-20b3e67b8655)

#### `GetFatherComponent(...)`

Returns the father component of the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/a2c28f3f-52ff-50a8-1b32-10d18db5c31d)

#### `GetHierarchicObjects(...)`

Returns an enumerator of all the connected hierarchic objects.

[Docs](https://developer.tekla.com/topic/en/18/43/cc228ea0-267b-7d90-7441-c9efc1779b08)

#### `GetIntegerReportProperties(...)`

Retrieves multiple integer report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/2623a9c0-2a9c-e233-9daa-59f92ed51445)

#### `GetIntegerUserProperties(...)`

Retrieves all integer properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/1b3c6f06-bc25-a54e-2a88-b60dc4a3c85e)

#### `GetPartMark(...)`

Returns the part mark.

[Docs](https://developer.tekla.com/topic/en/18/43/9978ba28-6304-714d-815c-fe4a6bb77076)

#### `GetPhase(...)`

Retrieves the phase of the model object (the phase number, the phase name, the phase comment and whether the phase is the current one or not).

[Docs](https://developer.tekla.com/topic/en/18/43/310f8b7a-c120-753a-35c9-f7c73a9806c9)

#### `GetPours(...)`

Returns an enumerator of all the pours that the part belongs to.

[Docs](https://developer.tekla.com/topic/en/18/43/1cc67ebb-329a-4675-7857-af663602d21d)

#### `GetReferenceLine(...)`

Returns the reference line for the given part.

[Docs](https://developer.tekla.com/topic/en/18/43/3e0a42c9-f10c-4225-bcd6-2b9b61ecc743)

#### `GetReinforcements(...)`

Returns an enumerator of all the connected reinforcements.

[Docs](https://developer.tekla.com/topic/en/18/43/510c913a-0653-6777-1841-c4f7df4fee47)

#### `GetReportProperty(String, Double.)(...)`

Retrieves a double property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/1c05aa56-5f42-be91-a59e-8d59a5549f45)

#### `GetReportProperty(String, Int32.)(...)`

Retrieves an integer property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/ce8e4ca0-3750-3c9b-5ca8-27e48b45e0bd)

#### `GetReportProperty(String, String.)(...)`

Retrieves a string property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/70f212bb-ee70-e9e9-7ab6-d6a9d46f8de6)

#### `GetSolid(FormingStates)(...)`

Returns the solid of the part.

[Docs](https://developer.tekla.com/topic/en/18/43/1e4d454b-d3a0-4219-b4e2-7f661b402c4d)

#### `GetSolid(Solid.SolidCreationTypeEnum)(...)`

Returns the solid of the part.

[Docs](https://developer.tekla.com/topic/en/18/43/25f58541-37fe-c1ca-f384-065ecbe38e55)

#### `GetSolid.(...)`

Returns the solid of the part.

[Docs](https://developer.tekla.com/topic/en/18/43/7bd70e97-7c8b-823c-c7e1-da2ebe3dadaf)

#### `GetStringReportProperties(...)`

Retrieves multiple string report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/5ff72da1-7382-f5d2-eb1d-c99b46239d42)

#### `GetStringUserProperties(...)`

Retrieves all string properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/67120e84-c820-9d52-12ae-c167c595842c)

#### `GetSurfaceObjects(...)`

Returns an enumerator of all the connected surface objects.

[Docs](https://developer.tekla.com/topic/en/18/43/36268c4e-9006-1c9d-57bd-331f907da6ce)

#### `GetSurfaceTreatments(...)`

Returns an enumerator of all the connected surface treatments.

[Docs](https://developer.tekla.com/topic/en/18/43/26c1c1a5-e5db-9850-f739-c58bc05c963e)

#### `GetUserProperty(String, Double.)(...)`

Retrieves a double property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/17cc8074-7955-32b5-2b4b-b169c45d4ee0)

#### `GetUserProperty(String, Int32.)(...)`

Retrieves an integer property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/d9364c1d-5b57-6a31-4c13-01124058cfb2)

#### `GetUserProperty(String, String.)(...)`

Retrieves a string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/214a886c-c18c-9b66-d7c9-fe84260962f2)

#### `GetWelds(...)`

Returns an enumerator of all the connected welds.

[Docs](https://developer.tekla.com/topic/en/18/43/722de753-413b-f810-f09c-07388feb0afa)

#### `Insert(...)`

Inserts the lofted plate into the model database. All the attributes must be set and there must be at minimum 2 base curves.

[Docs](https://developer.tekla.com/topic/en/18/43/6f89a610-eb42-1d83-2a11-e67376da4085)

#### `Modify(...)`

Modifies the lofted plate object values in the database.

[Docs](https://developer.tekla.com/topic/en/18/43/1273a8d6-0231-7f5f-8038-3b38c4ce521d)

#### `Select(...)`

Selects a lofted plate object from the database.

[Docs](https://developer.tekla.com/topic/en/18/43/8d64db7e-585d-afe2-da7d-3b970ff999ee)

#### `SetCustomObjectType(...)`

Sets a custom object type name for an existing object.

[Docs](https://developer.tekla.com/topic/en/18/43/3d6fb91b-48a9-35ca-d498-526514344089)

#### `SetDynamicStringProperty(...)`

Sets a dynamic string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/4dab0254-09e5-5e61-c28b-1c12afab71a0)

#### `SetLabel(...)`

Sets a label for an object when a new instance is created, this method must be called before Insert. The label is used in plug-ins for identifying the changed object in modification.

[Docs](https://developer.tekla.com/topic/en/18/43/3efcfc44-613a-e66b-9ebd-9f86df4f4036)

#### `SetPhase(...)`

Sets the phase of the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/ed80abcc-0ff7-d081-2229-e651f858a727)

#### `SetUserProperties(...)`

Sets multiple properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/8659da37-d571-f03f-c666-5c9fa3b113f5)

#### `SetUserProperty(String, Double)(...)`

Sets a double property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/5c585b8b-5c2e-7b75-2242-758f17429396)

#### `SetUserProperty(String, Int32)(...)`

Sets an integer property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/dd5b828d-3a87-bb58-5ac9-e1c3b4c2fe4b)

#### `SetUserProperty(String, String)(...)`

Sets a string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/80bf0348-b651-3d4a-862b-49c85bc924ab)

### Properties
- `AssemblyNumber` (object, get/set) — Gets or sets the assembly number. Defines the numbering in the assembly sense.
- `BaseCurves` (object, get/set) — The curves that define the directrix lines of the lofted plate. A lofted plate is defined by at least to base curves (non empty if they are Polycurve(s)).
- `CastUnitType` (object, get/set) — Gets or sets the cast unit type of the part.
- `Class` (object, get/set) — Gets or sets the class of the part.
- `DeformingData` (object, get/set) — Gets or sets the deforming data of the part.
- `FaceType` (object, get/set) — Gets or sets what type of solid faces will be used to create lofted plate solid.
- `Finish` (object, get/set) — Gets or sets the finish of the part.
- `Identifier` (object, get/set) — The identifier of the object.
- `IsUpToDate` (object, get/set) — Gets if the object does not have a modification which is not shared.
- `Material` (object, get/set) — Gets or sets the material the part is made of.
- `ModificationTime` (object, get/set) — Gets latest time of the object was modified or created.
- `Name` (object, get/set) — Gets or sets the name of the part.
- `PartNumber` (object, get/set) — Gets or sets the part number. Defines the numbering in the part sense.
- `Position` (object, get/set) — Gets or sets the part position. Defines the way the part is positioned in the model.
- `PourPhase` (object, get/set) — Gets or sets the pour phase of the part.
- `Profile` (object, get/set) — Gets or sets the profile of the part.

## LoftedPlate.LoftedPlateFaceTypeEnum (enum)

Lofted plate solid creation face types.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/f535a70c-d17d-cb0d-60a6-f472d5ec5664)

### Values
- `Perpendicular` = `0` — Faces are perpendicular to the thickness direction.
- `BoundedByCurvePlanes` = `1` — Faces following each curve plane.

## LoftedPlateOperationException (class)

This empty class serves as a base (marker class) for all the exception thrown in lofted plate operations

[Vendor docs](https://developer.tekla.com/topic/en/18/43/de853339-6173-37a9-4b73-fc8db446420f)

### Constructors
- `LoftedPlateOperationException(...)` — Constructs an instance of the class

## LogicalWeld (class)

The LogicalWeld class represents a group of welds in the model. A logical weld contains a list of children welds. The primary properties need to be the same in all the children welds.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/fc9878e2-e8b0-356f-5c7c-d06b71bd3057)

### Constructors
- `LogicalWeld(...)` — Creates a new logical weld instance.

### Methods
#### `AddWeld(...)`

Adds a weld to the logical weld.

[Docs](https://developer.tekla.com/topic/en/18/43/ef69a589-32c7-e352-0a9c-488305acb6f5)

#### `CompareTo(...)`

Compares Identifiers of model objects.

[Docs](https://developer.tekla.com/topic/en/18/43/79cca356-0c8a-e8fb-463a-34ad4141bec7)

#### `Delete(...)`

Deletes the logical weld instance with the given identifier from the model database and also the children welds.

[Docs](https://developer.tekla.com/topic/en/18/43/240d305f-c09e-ee5a-dc8c-99df3f1583e7)

#### `Equals(...)`

Check if Identifiers of model objects are same.

[Docs](https://developer.tekla.com/topic/en/18/43/e4324265-2490-00de-4139-63a8f7acb492)

#### `Explode(...)`

Removes the logical weld from the model but not the children welds.

[Docs](https://developer.tekla.com/topic/en/18/43/d016df9f-df30-cb0f-c9eb-bb011f353030)

#### `GetAllReportProperties(...)`

Retrieves all properties for the model object in one hashtable. Type for the returned value must be checked using type casting.

[Docs](https://developer.tekla.com/topic/en/18/43/0708f404-ce77-28e1-4ebe-fa906f68bff8)

#### `GetAllUserProperties(...)`

Retrieves all properties for the model object in one hashtable. Type for the returned value must be checked using type casting.

[Docs](https://developer.tekla.com/topic/en/18/43/63b853a0-0af5-af9c-3ce7-05299664d7f8)

#### `GetChildren(...)`

Returns an enumerator of all the children model objects.

[Docs](https://developer.tekla.com/topic/en/18/43/ce51ec40-310c-6067-a89f-f8d029aab747)

#### `GetCoordinateSystem(...)`

Returns the coordinate system for the given model object.

[Docs](https://developer.tekla.com/topic/en/18/43/8adfe9cd-f6e5-6474-1297-33c6270a7b0a)

#### `GetCustomObjectType(...)`

Gets custom object type name from an existing object.

[Docs](https://developer.tekla.com/topic/en/18/43/d3be2ba3-8533-7165-f2d3-a5aa9df556b8)

#### `GetDoubleReportProperties(...)`

Retrieves multiple double report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/f86f6c96-f738-49ca-16ba-f5634e862ba2)

#### `GetDoubleUserProperties(...)`

Retrieves all double properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/6ead7a63-5f76-0631-d480-d46c27e882aa)

#### `GetDynamicStringProperty(...)`

Gets a dynamic string property from the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/415e86d9-7de6-ea53-ce68-20b3e67b8655)

#### `GetFatherComponent(...)`

Returns the father component of the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/a2c28f3f-52ff-50a8-1b32-10d18db5c31d)

#### `GetHierarchicObjects(...)`

Returns an enumerator of all the connected hierarchic objects.

[Docs](https://developer.tekla.com/topic/en/18/43/cc228ea0-267b-7d90-7441-c9efc1779b08)

#### `GetIntegerReportProperties(...)`

Retrieves multiple integer report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/2623a9c0-2a9c-e233-9daa-59f92ed51445)

#### `GetIntegerUserProperties(...)`

Retrieves all integer properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/1b3c6f06-bc25-a54e-2a88-b60dc4a3c85e)

#### `GetMainWeld(...)`

Returns the main weld of the logical weld.

[Docs](https://developer.tekla.com/topic/en/18/43/d3829427-28e0-3bff-60a4-25ee7ee40c95)

#### `GetPhase(...)`

Retrieves the phase of the model object (the phase number, the phase name, the phase comment and whether the phase is the current one or not).

[Docs](https://developer.tekla.com/topic/en/18/43/310f8b7a-c120-753a-35c9-f7c73a9806c9)

#### `GetReportProperty(String, Double.)(...)`

Retrieves a double property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/1c05aa56-5f42-be91-a59e-8d59a5549f45)

#### `GetReportProperty(String, Int32.)(...)`

Retrieves an integer property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/ce8e4ca0-3750-3c9b-5ca8-27e48b45e0bd)

#### `GetReportProperty(String, String.)(...)`

Retrieves a string property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/70f212bb-ee70-e9e9-7ab6-d6a9d46f8de6)

#### `GetSolid(...)`

Method for getting the weld solid.

[Docs](https://developer.tekla.com/topic/en/18/43/319baa05-e7f2-e9ff-1f4f-c2a403dd71d1)

#### `GetStringReportProperties(...)`

Retrieves multiple string report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/5ff72da1-7382-f5d2-eb1d-c99b46239d42)

#### `GetStringUserProperties(...)`

Retrieves all string properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/67120e84-c820-9d52-12ae-c167c595842c)

#### `GetUserProperty(String, Double.)(...)`

Retrieves a double property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/17cc8074-7955-32b5-2b4b-b169c45d4ee0)

#### `GetUserProperty(String, Int32.)(...)`

Retrieves an integer property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/d9364c1d-5b57-6a31-4c13-01124058cfb2)

#### `GetUserProperty(String, String.)(...)`

Retrieves a string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/214a886c-c18c-9b66-d7c9-fe84260962f2)

#### `GetWeldGeometries(...)`

Method for getting weld seam geometries. Every result represents one individual seam geometry in current weld.

[Docs](https://developer.tekla.com/topic/en/18/43/335a1753-ab21-bc4a-f02b-2dfa5e951c7a)

#### `Insert(...)`

Inserts the logical weld into the model database. All the attributes must be set.

[Docs](https://developer.tekla.com/topic/en/18/43/6b3492fb-9680-cf6c-b510-8288abc5932f)

#### `Modify(...)`

Modifies the existing logical weld in the model database to match the current one.

[Docs](https://developer.tekla.com/topic/en/18/43/fcdc94bf-e306-5d39-87e1-85800c1c3d02)

#### `RemoveWeld(...)`

Removes a weld from the logical weld.

[Docs](https://developer.tekla.com/topic/en/18/43/f4110460-c769-2e5f-6e93-b5393091c033)

#### `Select(BaseWeld)(...)`

Selects the logical weld that a child weld belongs to.

[Docs](https://developer.tekla.com/topic/en/18/43/d483a81d-0da9-3473-7c51-d56982578e6a)

#### `Select.(...)`

Selects a logical weld from the model database. The logical weld identifier must be set.

[Docs](https://developer.tekla.com/topic/en/18/43/81e09766-4092-c4ac-03db-3bcaaaa6c3cd)

#### `SetCustomObjectType(...)`

Sets a custom object type name for an existing object.

[Docs](https://developer.tekla.com/topic/en/18/43/3d6fb91b-48a9-35ca-d498-526514344089)

#### `SetDynamicStringProperty(...)`

Sets a dynamic string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/4dab0254-09e5-5e61-c28b-1c12afab71a0)

#### `SetLabel(...)`

Sets a label for an object when a new instance is created, this method must be called before Insert. The label is used in plug-ins for identifying the changed object in modification.

[Docs](https://developer.tekla.com/topic/en/18/43/3efcfc44-613a-e66b-9ebd-9f86df4f4036)

#### `SetMainWeld(...)`

Sets the main weld for the logical weld.

[Docs](https://developer.tekla.com/topic/en/18/43/855f0f29-503f-fea4-2e11-9a87a9ecd764)

#### `SetPhase(...)`

Sets the phase of the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/ed80abcc-0ff7-d081-2229-e651f858a727)

#### `SetUserProperties(...)`

Sets multiple properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/8659da37-d571-f03f-c666-5c9fa3b113f5)

#### `SetUserProperty(String, Double)(...)`

Sets a double property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/5c585b8b-5c2e-7b75-2242-758f17429396)

#### `SetUserProperty(String, Int32)(...)`

Sets an integer property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/dd5b828d-3a87-bb58-5ac9-e1c3b4c2fe4b)

#### `SetUserProperty(String, String)(...)`

Sets a string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/80bf0348-b651-3d4a-862b-49c85bc924ab)

### Properties
- `AdditionalSizeAbove` (object, get/set) — Gets or sets the additional size above for combination welds.
- `AdditionalSizeBelow` (object, get/set) — Gets or sets the additional size below for combination welds.
- `AngleAbove` (object, get/set) — Gets or sets the angle above.
- `AngleBelow` (object, get/set) — Gets or sets the angle below.
- `AroundWeld` (object, get/set) — Gets or sets a value indicating whether the weld is an around weld (true) or an edge weld (false).
- `ConnectAssemblies` (object, get/set) — Gets or sets a value indicating whether to connect a part or an assembly as a secondary part (false) or as a sub-assembly (true).
- `ContourAbove` (object, get/set) — Gets or sets the contour above.
- `ContourBelow` (object, get/set) — Gets or sets the contour below.
- `EffectiveThroatAbove` (object, get/set) — Gets or sets the effective throat above.
- `EffectiveThroatBelow` (object, get/set) — Gets or sets the effective throat below.
- `ElectrodeClassification` (object, get/set) — Gets or sets the weld electrode classification.
- `ElectrodeCoefficient` (object, get/set) — Gets or sets the electrode strength coefficient.
- `ElectrodeStrength` (object, get/set) — Gets or sets the electrode strength.
- `FinishAbove` (object, get/set) — Gets or sets the finish above.
- `FinishBelow` (object, get/set) — Gets or sets the finish below.
- `Identifier` (object, get/set) — The identifier of the object.
- `IncrementAmountAbove` (object, get/set) — Gets or sets the increment amount above.
- `IncrementAmountBelow` (object, get/set) — Gets or sets the increment amount below.
- `IntermittentType` (object, get/set) — Gets or sets the weld intermittent type.
- `IsUpToDate` (object, get/set) — Gets if the object does not have a modification which is not shared.
- `LengthAbove` (object, get/set) — Gets or sets the length above.
- `LengthBelow` (object, get/set) — Gets or sets the length below.
- `MainObject` (object, get/set) — Gets or sets the main part of the weld.
- `ModificationTime` (object, get/set) — Gets latest time of the object was modified or created.
- `NDTInspection` (object, get/set) — Gets or sets the NDT inspection level.
- `PitchAbove` (object, get/set) — Gets or sets the pitch above.
- `PitchBelow` (object, get/set) — Gets or sets the pitch below.
- `Placement` (object, get/set) — Gets or sets the weld placement.
- `PrefixAboveLine` (object, get/set) — Gets or sets the size prefix above the line.
- `PrefixBelowLine` (object, get/set) — Gets or sets the size prefix below the line.
- `Preparation` (object, get/set) — Gets or sets the weld preparation.
- `ProcessType` (object, get/set) — Gets or sets the process type.
- `ReferenceText` (object, get/set) — Gets or sets the reference text.
- `RootFaceAbove` (object, get/set) — Gets or sets the root face above.
- `RootFaceBelow` (object, get/set) — Gets or sets the root face below.
- `RootOpeningAbove` (object, get/set) — Gets or sets the root opening above.
- `RootOpeningBelow` (object, get/set) — Gets or sets the root opening below.
- `SecondaryObject` (object, get/set) — Gets or sets the secondary part of the weld.
- `ShopWeld` (object, get/set) — Gets or sets a value indicating whether the weld is a shop weld (true) or a site weld (false).
- `SizeAbove` (object, get/set) — Gets or sets the size above.
- `SizeBelow` (object, get/set) — Gets or sets the size below.
- `Standard` (object, get/set) — Gets or sets the weld detail/standard.
- `StitchWeld` (object, get/set) — Gets or sets a value indicating whether the weld is stitched (true) or not stitched (false).
- `TypeAbove` (object, get/set) — Gets or sets the type above.
- `TypeBelow` (object, get/set) — Gets or sets the type below.
- `WeldNumber` (object, get/set) — Gets the weld number.
- `WeldNumberPrefix` (object, get/set) — Gets or sets the weld number prefix.

## Material (class)

The Material class represents a single material that parts can be made of.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/b76cd4d2-f863-0e66-174a-99dda6afc8d6)

### Constructors
- `Material(...)` — Instantiates an empty material instance.

### Properties
- `MaterialString` (object, get/set) — Identifies the material in a string format.

## Model (class)

The Model class represents a single model open in Tekla Structures. Before interaction with the model, the user will have to create one instance of this class.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/01eab525-e3d6-22d8-b31a-e4c733e03d43)

### Constructors
- `Model(...)` — Creates a "handle" to the currently open model.

### Methods
#### `CommitChanges(String)(...)`

Commits the changes made to the model database so far. One commit is something that a user can later on undo with the undo command. A commit also launches the drawing of the changed product model to the visible views. A plug-in should never call CommitChanges, since this would make undo very difficult for the user to do.

[Docs](https://developer.tekla.com/topic/en/18/43/6b589509-80c1-8a7d-4c6d-718ccf01b9d3)

#### `CommitChanges.(...)`

Commits the changes made to the model database so far. One commit is something that a user can later on undo with the undo command. A commit also launches the drawing of the changed product model to the visible views. A dependent plug-in should never call CommitChanges, since this would make undo very difficult for the user to do. However, non-dependent plug-ins require a separate commit if new objects are created.

[Docs](https://developer.tekla.com/topic/en/18/43/06690b14-dafd-82b5-ee4c-51292a3e3055)

#### `FetchModelObjects(List.Identifier., Boolean)(...)`

Fetches a list of modelobjects based on given identifier list of objects, and optionally selects objects before returning them.

[Docs](https://developer.tekla.com/topic/en/18/43/a1d2ba4a-7713-0fb2-5e0c-671708b98b58)

#### `FetchModelObjects(List.String., Boolean)(...)`

Fetches a list of modelobjects based on given guid list of objects, checks if guid is native or external (for reference model object) and optionally selects objects before returning them.

[Docs](https://developer.tekla.com/topic/en/18/43/fb4054d2-dfad-6bb6-c07d-0298ea5e1833)

#### `GetClashCheckHandler(...)`

Returns a new clash check handler.

[Docs](https://developer.tekla.com/topic/en/18/43/e27b5954-d2a6-55e5-3b08-c0d3c8e8fba6)

#### `GetConnectionStatus(...)`

Returns true if a proper connection to the Tekla Structures process has been established. Currently, there's no way to re-establish the connection.

[Docs](https://developer.tekla.com/topic/en/18/43/aea8fbbf-cf8f-7128-f6e9-e3ef045497d4)

#### `GetGUIDByIdentifier(...)`

Returns the GUID of the given identifier instance.

[Docs](https://developer.tekla.com/topic/en/18/43/01994be1-6039-ca3b-98fb-1deb2aeba040)

#### `GetIdentifierByGUID(...)`

Returns an identifier instance that has the given GUID in the model.

[Docs](https://developer.tekla.com/topic/en/18/43/3fb69e75-03a4-e79b-4f2d-8738407ffabe)

#### `GetInfo(...)`

Returns information about the currently open model.

[Docs](https://developer.tekla.com/topic/en/18/43/de955ec1-7513-ad43-07ea-95d11ab3c34c)

#### `GetModelObjectSelector(...)`

Returns a ModelObjectSelector instance from which different kind of selections can be made.

[Docs](https://developer.tekla.com/topic/en/18/43/fb9935eb-3299-ce0b-7415-c08eed8a8244)

#### `GetPhases(...)`

Returns information about the current model's phases.

[Docs](https://developer.tekla.com/topic/en/18/43/4bc73a5f-a7d0-72d1-1e9d-87587dd37139)

#### `GetProjectInfo(...)`

Returns information about the current model's project.

[Docs](https://developer.tekla.com/topic/en/18/43/dc119752-ab2b-b228-b92e-2e7d08a6f599)

#### `GetReportPropertyDouble(...)`

Get a list of property values, where the property is of type double, for the given identifiers.

[Docs](https://developer.tekla.com/topic/en/18/43/0c8df735-b799-f799-b733-fd15cc8efd16)

#### `GetReportPropertyInt(...)`

Get a list of property values, where the property is of type interger, for the given identifiers.

[Docs](https://developer.tekla.com/topic/en/18/43/a72cac03-20d1-216f-471f-7a8b491ba1f3)

#### `GetReportPropertyStr(...)`

Get a list of property values, where the property is of type string, for the given identifiers.

[Docs](https://developer.tekla.com/topic/en/18/43/3387b364-3010-d135-79c6-8f45d935e25f)

#### `GetWorkPlaneHandler(...)`

Returns a new work plane handler for the model. The work plane handler can be used to manipulate the current work plane in the model.

[Docs](https://developer.tekla.com/topic/en/18/43/f0559338-2fb2-eb85-3986-9717b9637c73)

#### `SelectModelObject(...)`

Takes as input an identifier to an object and then checks its type and instantiates and selects it before returning it.

[Docs](https://developer.tekla.com/topic/en/18/43/42921956-058f-e5b1-1765-02ad74c57f55)

## ModelHandler (class)

The ModelHandler class provides information about the currently open Tekla Structures model.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/4a2739ce-3c44-53ae-9246-2d6ce85d750b)

### Constructors
- `ModelHandler(...)` — Creates a new ModelHandler instance.

### Methods
#### `Close(...)`

Closes current model.

[Docs](https://developer.tekla.com/topic/en/18/43/8f0e93e3-83f2-50b9-9c60-5a6ac54c5764)

#### `CreateNewMultiUserModel(...)`

Creates a new multi-user model.

[Docs](https://developer.tekla.com/topic/en/18/43/26fdf9dc-2f34-5ea8-af1c-af891809822b)

#### `CreateNewSingleUserModel(...)`

Creates a new single user model.

[Docs](https://developer.tekla.com/topic/en/18/43/4a87383d-506b-847e-0259-27e1e9b1dfb0)

#### `IsModelAutoSaved(...)`

Tells whether a model has auto saved information.

[Docs](https://developer.tekla.com/topic/en/18/43/df868bbd-a070-1cc4-349f-e2ef21aa408c)

#### `IsModelSaved(...)`

Tells whether current model has been saved.

[Docs](https://developer.tekla.com/topic/en/18/43/c2a7a29e-4c4b-f75a-1116-d3c0e276fd25)

#### `Open(...)`

Opens a new model to Tekla Structures without saving changes to current model.

[Docs](https://developer.tekla.com/topic/en/18/43/3c82285d-8ad8-bd71-cc74-874ea0ef0e16)

#### `Save(String, String)(...)`

Saves current model with comment and user information.

[Docs](https://developer.tekla.com/topic/en/18/43/c828c8fa-25b4-eb78-f9dc-0d465498e3d0)

#### `Save(String, String, String)(...)`

Saves current model with comment and user information.

[Docs](https://developer.tekla.com/topic/en/18/43/7373ac01-ea88-2ffa-ddae-01a37ed87e2c)

## ModelInfo (class)

The ModelInfo class provides information about the currently open Tekla Structures model.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/cc2d593e-65f0-07c0-e4ab-17c60fe4f221)

### Properties
- `CurrentPhase` (object, get/set) — The number of the Tekla Structures model's current phase.
- `ModelName` (object, get/set) — The name of the Tekla Structures model.
- `ModelPath` (object, get/set) — The path to the Tekla Structures model.
- `NorthDirection` (object, get/set) — The north direction of the current Tekla Structures model.
- `SharedModel` (object, get/set) — The read-only property of the Tekla Structures model's sharing status.
- `SingleUserModel` (object, get/set) — The read-only property if the Tekla Structures model is a single user model.

## ModelObject (class)

The ModelObject class is an abstract base class for all model objects, such as parts, bolts, welds and reinforcements.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/a90863bf-5ccc-e808-566a-a2ef79a9224d)

### Methods
#### `CompareTo(...)`

Compares Identifiers of model objects.

[Docs](https://developer.tekla.com/topic/en/18/43/79cca356-0c8a-e8fb-463a-34ad4141bec7)

#### `Delete(...)`

Deletes the instance from the model database.

[Docs](https://developer.tekla.com/topic/en/18/43/0b922776-ccee-4b5e-4a12-4e5306802e90)

#### `Equals(...)`

Check if Identifiers of model objects are same.

[Docs](https://developer.tekla.com/topic/en/18/43/e4324265-2490-00de-4139-63a8f7acb492)

#### `GetAllReportProperties(...)`

Retrieves all properties for the model object in one hashtable. Type for the returned value must be checked using type casting.

[Docs](https://developer.tekla.com/topic/en/18/43/0708f404-ce77-28e1-4ebe-fa906f68bff8)

#### `GetAllUserProperties(...)`

Retrieves all properties for the model object in one hashtable. Type for the returned value must be checked using type casting.

[Docs](https://developer.tekla.com/topic/en/18/43/63b853a0-0af5-af9c-3ce7-05299664d7f8)

#### `GetChildren(...)`

Returns an enumerator of all the children model objects.

[Docs](https://developer.tekla.com/topic/en/18/43/ce51ec40-310c-6067-a89f-f8d029aab747)

#### `GetCoordinateSystem(...)`

Returns the coordinate system for the given model object.

[Docs](https://developer.tekla.com/topic/en/18/43/8adfe9cd-f6e5-6474-1297-33c6270a7b0a)

#### `GetCustomObjectType(...)`

Gets custom object type name from an existing object.

[Docs](https://developer.tekla.com/topic/en/18/43/d3be2ba3-8533-7165-f2d3-a5aa9df556b8)

#### `GetDoubleReportProperties(...)`

Retrieves multiple double report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/f86f6c96-f738-49ca-16ba-f5634e862ba2)

#### `GetDoubleUserProperties(...)`

Retrieves all double properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/6ead7a63-5f76-0631-d480-d46c27e882aa)

#### `GetDynamicStringProperty(...)`

Gets a dynamic string property from the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/415e86d9-7de6-ea53-ce68-20b3e67b8655)

#### `GetFatherComponent(...)`

Returns the father component of the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/a2c28f3f-52ff-50a8-1b32-10d18db5c31d)

#### `GetHierarchicObjects(...)`

Returns an enumerator of all the connected hierarchic objects.

[Docs](https://developer.tekla.com/topic/en/18/43/cc228ea0-267b-7d90-7441-c9efc1779b08)

#### `GetIntegerReportProperties(...)`

Retrieves multiple integer report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/2623a9c0-2a9c-e233-9daa-59f92ed51445)

#### `GetIntegerUserProperties(...)`

Retrieves all integer properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/1b3c6f06-bc25-a54e-2a88-b60dc4a3c85e)

#### `GetPhase(...)`

Retrieves the phase of the model object (the phase number, the phase name, the phase comment and whether the phase is the current one or not).

[Docs](https://developer.tekla.com/topic/en/18/43/310f8b7a-c120-753a-35c9-f7c73a9806c9)

#### `GetReportProperty(String, Double.)(...)`

Retrieves a double property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/1c05aa56-5f42-be91-a59e-8d59a5549f45)

#### `GetReportProperty(String, Int32.)(...)`

Retrieves an integer property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/ce8e4ca0-3750-3c9b-5ca8-27e48b45e0bd)

#### `GetReportProperty(String, String.)(...)`

Retrieves a string property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/70f212bb-ee70-e9e9-7ab6-d6a9d46f8de6)

#### `GetStringReportProperties(...)`

Retrieves multiple string report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/5ff72da1-7382-f5d2-eb1d-c99b46239d42)

#### `GetStringUserProperties(...)`

Retrieves all string properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/67120e84-c820-9d52-12ae-c167c595842c)

#### `GetUserProperty(String, Double.)(...)`

Retrieves a double property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/17cc8074-7955-32b5-2b4b-b169c45d4ee0)

#### `GetUserProperty(String, Int32.)(...)`

Retrieves an integer property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/d9364c1d-5b57-6a31-4c13-01124058cfb2)

#### `GetUserProperty(String, String.)(...)`

Retrieves a string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/214a886c-c18c-9b66-d7c9-fe84260962f2)

#### `Insert(...)`

Inserts the model object instance into the model database.

[Docs](https://developer.tekla.com/topic/en/18/43/f17ed840-1d16-b843-4b0c-270875de1d35)

#### `Modify(...)`

Modifies the model instance in the model database.

[Docs](https://developer.tekla.com/topic/en/18/43/186833b8-4837-ed77-b314-3ebf9dc32d6a)

#### `Select(...)`

Selects the model object instance from the model database.

[Docs](https://developer.tekla.com/topic/en/18/43/f662ab10-be15-6280-d0aa-fc259dfd7c07)

#### `SetCustomObjectType(...)`

Sets a custom object type name for an existing object.

[Docs](https://developer.tekla.com/topic/en/18/43/3d6fb91b-48a9-35ca-d498-526514344089)

#### `SetDynamicStringProperty(...)`

Sets a dynamic string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/4dab0254-09e5-5e61-c28b-1c12afab71a0)

#### `SetLabel(...)`

Sets a label for an object when a new instance is created, this method must be called before Insert. The label is used in plug-ins for identifying the changed object in modification.

[Docs](https://developer.tekla.com/topic/en/18/43/3efcfc44-613a-e66b-9ebd-9f86df4f4036)

#### `SetPhase(...)`

Sets the phase of the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/ed80abcc-0ff7-d081-2229-e651f858a727)

#### `SetUserProperties(...)`

Sets multiple properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/8659da37-d571-f03f-c666-5c9fa3b113f5)

#### `SetUserProperty(String, Double)(...)`

Sets a double property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/5c585b8b-5c2e-7b75-2242-758f17429396)

#### `SetUserProperty(String, Int32)(...)`

Sets an integer property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/dd5b828d-3a87-bb58-5ac9-e1c3b4c2fe4b)

#### `SetUserProperty(String, String)(...)`

Sets a string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/80bf0348-b651-3d4a-862b-49c85bc924ab)

### Properties
- `Identifier` (object, get/set) — The identifier of the object.
- `IsUpToDate` (object, get/set) — Gets if the object does not have a modification which is not shared.
- `ModificationTime` (object, get/set) — Gets latest time of the object was modified or created.

## ModelObject.ModelObjectEnum (enum)

All possible concrete model object subclasses are defined here. This enumeration can be used to fetch certain types of objects from the Tekla Structures model.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/40af6269-5d48-d7a0-3b34-95ef2f744c3e)

### Values
- `UNKNOWN` = `0` — The unknown model object.
- `BEAM` = `1` — The beam.
- `POLYBEAM` = `2` — The polybeam.
- `CONTOURPLATE` = `3` — The contour plate.
- `BOOLEANPART` = `4` — The boolean part.
- `FITTING` = `5` — The fitting.
- `CUTPLANE` = `6` — The cutplane.
- `SURFACE_TREATMENT` = `7` — The surface treatment.
- `WELD` = `8` — The weld.
- `ASSEMBLY` = `9` — The assembly.
- `SINGLEREBAR` = `10` — The single rebar.
- `REBARGROUP` = `11` — The rebar group.
- `REBARMESH` = `12` — The rebar mesh.
- `REBARSTRAND` = `13` — The rebar strand.
- `CONTROL_PLANE` = `14` — The control plane.
- `BOLT_ARRAY` = `15` — The bolt array.
- `BOLT_CIRCLE` = `16` — The bolt circle.
- `BOLT_XYLIST` = `17` — The bolt XY list.
- `LOAD_POINT` = `18` — The point load.
- `LOAD_LINE` = `19` — The line load.
- `LOAD_AREA` = `20` — The area load.
- `LOAD_UNIFORM` = `21` — The uniform load.
- `GRID` = `22` — The grid.
- `GRIDPLANE` = `23` — The grid plane.
- `CONNECTION` = `24` — The connection.
- `COMPONENT` = `25` — The component.
- `SEAM` = `26` — The seam.
- `DETAIL` = `27` — The detail.
- `REFERENCE_MODEL` = `28` — The reference model.
- `REBAR_SPLICE` = `29` — The rebar splice.
- `LOAD_GROUP` = `30` — The load group.
- `TASK` = `31` — The task.
- `TASK_DEPENDENCY` = `32` — The task dependency.
- `TASK_WORKTYPE` = `34` — The task worktype.
- `POLYGON_WELD` = `35` — The polygon weld.
- `LOGICAL_WELD` = `36` — The logical weld.
- `CIRCLEREBAR` = `37` — The circle rebar.
- `HIERARCHIC_DEFINITION` = `38` — The hierarchic definition.
- `HIERARCHIC_OBJECT` = `39` — The hierarchic object.
- `ANALYSIS_GEOMETRY` = `40` — The analysis geometry.
- `ANALYSIS_PART` = `41` — The analysis part.
- `REFERENCE_MODEL_OBJECT` = `42` — The reference model object.
- `CUSTOM_PART` = `43` — The custom part object.
- `CIRCLE_REBARGROUP` = `44` — The circle rebar group.
- `CURVED_REBARGROUP` = `45` — The curved rebar group.
- `EDGE_CHAMFER` = `46` — The edge chamfer.
- `POUR_OBJECT` = `47` — The pour object.
- `POUR_BREAK` = `48` — The pour break.
- `CONTROL_LINE` = `50` — The control line.
- `LOAD_TEMPERATURE` = `51` — The temperature load.
- `BREP` = `52` — The Brep part instance.
- `CONTROL_CIRCLE` = `53` — The control circle.
- `CONTROL_POINT` = `54` — The control point.
- `REBAR_SET` = `55` — The rebar set.
- `REBAR_SET_ADDITION` = `56` — The rebar set addition.
- `REBAR_PROPERTY_MODIFIER` = `57` — The rebar property modifier.
- `REBAR_END_DETAIL_MODIFIER` = `58` — The rebar end-detail modifier.
- `REBAR_SPLITTER` = `59` — The rebar splitter.
- `CONTROL_ARC` = `64` — Construction arc.
- `CONTROL_SPLINE` = `65` — Construction spline
- `CONTROL_POLYCURVE` = `66` — Construction polycurve
- `RADIAL_GRID` = `67` — Radial grid
- `GRID_CYLINDRICAL_SURFACE` = `68` — Cylindrical surfaces of radial grids
- `LOFTED_PLATE` = `69` — The lofted plate
- `LEG_SURFACE_OBJECT` = `70` — The leg surface object.
- `STOREY` = `71` — The storey object.
- `SPACE` = `72` — The building space object.
- `BUILDING` = `73` — The building space object.
- `BUILDING_SITE` = `74` — The site object.

## ModelObjectEnumerator (class)

The ModelObjectEnumerator class provides the means to iterate through model object instances in the current model.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/dfa49e10-8481-338b-9448-aa086d991eb4)

### Methods
#### `GetEnumerator(...)`

Gets the enumerator.

[Docs](https://developer.tekla.com/topic/en/18/43/b9b8dfa0-1aff-df12-b15d-657934d702eb)

#### `GetSize(...)`

Returns the total amount of items.

[Docs](https://developer.tekla.com/topic/en/18/43/554f6b86-812b-dce3-6f18-238f1bdfba05)

#### `MoveNext(...)`

Moves to the next item in the enumerator.

[Docs](https://developer.tekla.com/topic/en/18/43/f77c087e-87ac-dad4-049a-4d8d6d340d4d)

#### `Reset(...)`

Resets the enumerator to the beginning.

[Docs](https://developer.tekla.com/topic/en/18/43/b62cffa6-b709-1ca1-e52f-64702d76aef6)

### Properties
- `AutoFetch` (object, get/set) — Indicates that the objects are fetched from the model when the enumerator is created. Object information is therefore not anymore fetched when 'Current' item is asked from the enumerator. Warning: changing of TransformationPlane after creation of enumerator or during the enumeration requires a separate selection of object for refreshing the values. Property value is used for all enumerators in application
- `Current` (object, get/set) — The current model object instance active in the enumerator. The value is null if there are no more objects left.
- `SelectInstances` (object, get/set) — Indicates that the instance Select() is called when the 'Current' item is asked from the enumerator. The user can set this to 'false' if no members are ever asked from the instance. This is the case when, for example, asking only a report property from the identifier. Warning: normally the user should not change this value.

## ModelObjectEnumerator.EnumeratorTypeEnum (enum)

The types of enumerator available.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/d68984b3-09aa-57bf-5ecb-e89993e7a61c)

### Values
- `ALL_SELECTED` = `1` — Selects all user selected objects from the model.
- `ALL_OBJECTS` = `2` — Selects all objects from the model.
- `MODELOBJECT_CHILDREN` = `3` — Selects children of specified type of given ModelObject
- `BY_FILTER_NAME` = `4` — Selects all the ModelObjects for a given filter name
- `CONNECTED_COMPONENTS` = `5` — Selects all connected components of given ModelObject
- `BY_BOUNDING_BOX` = `6` — Selects all objects intersecting with given bounding box
- `ALL_OBJECTS_WITH_TYPE` = `7` — All Objects with type given in type in the "subtype".
- `REFERENCE_MODEL_OBJECTS` = `8` — Selects sub objects of given ReferenceModel.
- `MODELOBJECT_FATHERS` = `9` — Selects fathers of specified type of given ModelObject (Tasks mainly)
- `CONNECTED_DEPENDENCIES` = `10` — Selects all connected dependencies of given ModelObject (Tasks mainly)
- `TASKS_OF_SELECTED_OBJECTS` = `11` — Selects all tasks of selected objects
- `MODIFIED_OBJECTS_AFTER_STAMP` = `12` — Selects all modified and created objects
- `FILTERED_OBJECTS_WITH_TYPE` = `13` — Filtered objects with type given in type in the "subtype".
- `DELETED_OBJECTS_AFTER_STAMP` = `14` — Selects all modified objects
- `ALL_PRESELECTED` = `15` — Selects all preselected objects.
- `BY_BOUNDING_BOX_AND_VIEW` = `16` — Selects all objects that intersect with the given bounding box and that are visible in the given view.
- `CONVERTED_FROM_REFERENCE_MODEL` = `17` — Selects objects that were converted from a given reference model (if they weren't unlinked)
- `ATTRIBUTE_MODIFIED_OBJECTS_AFTER_STAMP` = `18` — Selects all objects of which attributes (UDA) have been modified after the stamp.
- `NUMBERING_MODIFIED_OBJECTS_AFTER_STAMP` = `19` — Selects all objects of which numbering has been modified after the stamp.
- `CREATED_OBJECTS_AFTER_STAMP` = `20` — Selects all created objects
- `MODIFIED_OBJECTS_BY_FILTER_NAME` = `21` — Selects all objects which are new, modified, numbered or which phase has changed after the stamp and pass the filter of given name.
- `PHASE_MODIFIED_OBJECTS_AFTER_STAMP` = `22` — Selects all objects of which phase has been modified after the stamp.
- `OBJECTS_WITH_ANY_MODIFICATION_AFTER_STAMP` = `23` — Selects all objects with any type of modification or are created after the stamp.
- `BY_MODEL_OBJECT_TYPE` = `25` — Selects all model objects with the specified type.

## ModelObjectSelector (class)

The ModelObjectSelector class can be used to make different model object selections from the current model.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/50af1768-22fb-080e-f3d1-08b0da8199da)

### Methods
#### `GetAllObjects(...)`

Returns an enumerator of all the model objects in the current model.

[Docs](https://developer.tekla.com/topic/en/18/43/10561eb7-f852-d0f1-a34e-8cab6c2e3459)

#### `GetAllObjectsWithType(.Type.)(...)`

Returns an enumerator of all the model objects in the current model with the given base type.

[Docs](https://developer.tekla.com/topic/en/18/43/71548129-b401-0376-634f-c3b2add9f38a)

#### `GetAllObjectsWithType(ModelObject.ModelObjectEnum)(...)`

Returns an enumerator of all the model objects in the current model with the given type.

[Docs](https://developer.tekla.com/topic/en/18/43/d6c4b8b6-3636-3060-9458-a196720c6ba9)

#### `GetEnumerator(...)`

Returns an enumerator of all the model objects in the current model.

[Docs](https://developer.tekla.com/topic/en/18/43/cef5b6c6-2855-343a-7ccd-3e3c16e3cf47)

#### `GetFilteredObjectsWithType(...)`

Returns an enumerator of the model objects in the current model with the given type and selected by the filter.

[Docs](https://developer.tekla.com/topic/en/18/43/1c569914-2d33-622e-1aae-8105c059ea09)

#### `GetObjectsByBoundingBox(...)`

Returns an enumerator of the model objects in the current model colliding with the given geometrical bounding box. Note that this method uses approximate bounding boxes and thus is NOT EXACT, and may return objects not necessarily colliding with the given box but only being somewhere near to it.

[Docs](https://developer.tekla.com/topic/en/18/43/0947a69e-18cb-ca41-cb95-c36c19b1c85e)

#### `GetObjectsByFilter(...)`

Returns an enumerator of the model objects in the current model selected by the given selection filter definition.

[Docs](https://developer.tekla.com/topic/en/18/43/84095767-c855-1698-b819-6f73c55e19b6)

#### `GetObjectsByFilterName(...)`

Returns an enumerator of the model objects in the current model selected by the given selection filter.

[Docs](https://developer.tekla.com/topic/en/18/43/b51f9696-1097-13a6-7fac-f79264fabcbd)

## NullRulingException (class)

This class represent an exception thrown when a ruling of the lofted plate (i.e. the lines that swipe the plate surface between the base curves) would become zero because of a lofted plate operation

[Vendor docs](https://developer.tekla.com/topic/en/18/43/6a6c5f35-ba15-bf0c-ebff-7e205ead76fd)

### Constructors
- `NullRulingException(...)` — Constructs an instance of the class

## NumberingSeries (class)

The NumberingSeries class describes how an object is to be numbered.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/8788e92a-0d78-4c24-a3cc-039b96c08789)

### Constructors
- `NumberingSeries(...)` — Instantiates a default numbering.
- `NumberingSeries(...)` — Instantiates a numbering series with the given arguments.

### Properties
- `Prefix` (object, get/set) — The prefix in numbering.
- `StartNumber` (object, get/set) — The start number in numbering.

## NumberingSeriesNullable (class)

The NumberingSeriesNullable class describes how an object is to be numbered.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/e2273fc8-03bf-259b-a74a-ae85efeca9e6)

### Constructors
- `NumberingSeriesNullable(...)` — Initializes a new instance of the NumberingSeriesNullable class.
- `NumberingSeriesNullable(...)` — Initializes a new instance of the NumberingSeriesNullable class with the given arguments.

### Properties
- `Prefix` (object, get/set) — Gets or sets the numbering prefix.
- `StartNumber` (object, get/set) — Gets or sets the numbering start number.

## Object (class)

The Object class is an abstract base class for all the objects Tekla Structures has.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/9e5dc86b-68fe-395d-5ad0-5d4ca084ab36)

### Properties
- `Identifier` (object, get/set) — The identifier of the object.

## Offset (class)

The Offset class defines the offset value of start points and end points. The offset is the distance from the current work plane in the three global directions.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/288e8450-b840-d318-343e-9098448754b4)

### Constructors
- `Offset(...)` — Instantiates an offset with zero distance.

### Properties
- `Dx` (object, get/set) — The current object's Dx offset. This value is defined using the object's coordinate system.
- `Dy` (object, get/set) — The current object's Dy offset. This value is defined using the current work plane.
- `Dz` (object, get/set) — The current object's Dz offset. This value is defined using the current work plane.

## Part (class)

The Part class represents a part in the model. A part can be either a beam, a polybeam or a contour plate.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/ac720ea3-a9ae-235b-d627-c9c37585b6ec)

### Constructors
- `Part(...)` — Initializes a new instance of the Part class with default attributes. The default values are set as follows: Position = new Position();Profile = new Profile();Material = new Material();DeformingData = new DeformingData();Name = "";Class = "";Finish = "";PartNumber = new NumberingSeries("P", 1);AssemblyNumber = new NumberingSeries("A", 1);CastUnitType = CastUnitTypeEnum.PRECAST;

### Methods
#### `CompareTo(Object)(...)`

Compares Identifiers of model objects.

[Docs](https://developer.tekla.com/topic/en/18/43/79cca356-0c8a-e8fb-463a-34ad4141bec7)

#### `CompareTo(Part)(...)`

Compares the instantiated part with another one.

[Docs](https://developer.tekla.com/topic/en/18/43/e8632c0d-89cf-9aab-6532-f62f9873067b)

#### `Delete(...)`

Deletes the instance from the model database.

[Docs](https://developer.tekla.com/topic/en/18/43/0b922776-ccee-4b5e-4a12-4e5306802e90)

#### `Equals(...)`

Check if Identifiers of model objects are same.

[Docs](https://developer.tekla.com/topic/en/18/43/e4324265-2490-00de-4139-63a8f7acb492)

#### `GetAllReportProperties(...)`

Retrieves all properties for the model object in one hashtable. Type for the returned value must be checked using type casting.

[Docs](https://developer.tekla.com/topic/en/18/43/0708f404-ce77-28e1-4ebe-fa906f68bff8)

#### `GetAllUserProperties(...)`

Retrieves all properties for the model object in one hashtable. Type for the returned value must be checked using type casting.

[Docs](https://developer.tekla.com/topic/en/18/43/63b853a0-0af5-af9c-3ce7-05299664d7f8)

#### `GetAssembly(...)`

Returns the assembly that the part belongs to.

[Docs](https://developer.tekla.com/topic/en/18/43/bda18562-45a1-1355-42ed-6044af0b18cb)

#### `GetBolts(...)`

Returns an enumerator of all the connected bolts.

[Docs](https://developer.tekla.com/topic/en/18/43/dbc6b475-7461-7ec4-e8ec-7b7d3d8ff429)

#### `GetBooleans(...)`

Returns an enumerator of all the connected boolean objects.

[Docs](https://developer.tekla.com/topic/en/18/43/ae180e43-88e2-d759-0ae0-567d52681a2a)

#### `GetCenterLine(...)`

Returns the center line for the given part.

[Docs](https://developer.tekla.com/topic/en/18/43/bdc886b8-af7a-722c-dcb8-fddb2f250a83)

#### `GetChildren(...)`

Returns an enumerator of all the children model objects.

[Docs](https://developer.tekla.com/topic/en/18/43/ce51ec40-310c-6067-a89f-f8d029aab747)

#### `GetComponents(...)`

Returns an enumerator of all the connected components, connections, seams and details inherited from the base component.

[Docs](https://developer.tekla.com/topic/en/18/43/ed57941c-2cd8-3b78-6a6e-848b8a920298)

#### `GetCoordinateSystem(...)`

Returns the coordinate system for the given model object.

[Docs](https://developer.tekla.com/topic/en/18/43/8adfe9cd-f6e5-6474-1297-33c6270a7b0a)

#### `GetCustomObjectType(...)`

Gets custom object type name from an existing object.

[Docs](https://developer.tekla.com/topic/en/18/43/d3be2ba3-8533-7165-f2d3-a5aa9df556b8)

#### `GetDSTVCoordinateSystem(...)`

Get DSTV coordinate system.

[Docs](https://developer.tekla.com/topic/en/18/43/9e70005f-c701-ab99-8fa0-4fce47b1ab05)

#### `GetDoubleReportProperties(...)`

Retrieves multiple double report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/f86f6c96-f738-49ca-16ba-f5634e862ba2)

#### `GetDoubleUserProperties(...)`

Retrieves all double properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/6ead7a63-5f76-0631-d480-d46c27e882aa)

#### `GetDynamicStringProperty(...)`

Gets a dynamic string property from the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/415e86d9-7de6-ea53-ce68-20b3e67b8655)

#### `GetFatherComponent(...)`

Returns the father component of the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/a2c28f3f-52ff-50a8-1b32-10d18db5c31d)

#### `GetHierarchicObjects(...)`

Returns an enumerator of all the connected hierarchic objects.

[Docs](https://developer.tekla.com/topic/en/18/43/cc228ea0-267b-7d90-7441-c9efc1779b08)

#### `GetIntegerReportProperties(...)`

Retrieves multiple integer report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/2623a9c0-2a9c-e233-9daa-59f92ed51445)

#### `GetIntegerUserProperties(...)`

Retrieves all integer properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/1b3c6f06-bc25-a54e-2a88-b60dc4a3c85e)

#### `GetPartMark(...)`

Returns the part mark.

[Docs](https://developer.tekla.com/topic/en/18/43/9978ba28-6304-714d-815c-fe4a6bb77076)

#### `GetPhase(...)`

Retrieves the phase of the model object (the phase number, the phase name, the phase comment and whether the phase is the current one or not).

[Docs](https://developer.tekla.com/topic/en/18/43/310f8b7a-c120-753a-35c9-f7c73a9806c9)

#### `GetPours(...)`

Returns an enumerator of all the pours that the part belongs to.

[Docs](https://developer.tekla.com/topic/en/18/43/1cc67ebb-329a-4675-7857-af663602d21d)

#### `GetReferenceLine(...)`

Returns the reference line for the given part.

[Docs](https://developer.tekla.com/topic/en/18/43/3e0a42c9-f10c-4225-bcd6-2b9b61ecc743)

#### `GetReinforcements(...)`

Returns an enumerator of all the connected reinforcements.

[Docs](https://developer.tekla.com/topic/en/18/43/510c913a-0653-6777-1841-c4f7df4fee47)

#### `GetReportProperty(String, Double.)(...)`

Retrieves a double property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/1c05aa56-5f42-be91-a59e-8d59a5549f45)

#### `GetReportProperty(String, Int32.)(...)`

Retrieves an integer property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/ce8e4ca0-3750-3c9b-5ca8-27e48b45e0bd)

#### `GetReportProperty(String, String.)(...)`

Retrieves a string property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/70f212bb-ee70-e9e9-7ab6-d6a9d46f8de6)

#### `GetSolid(FormingStates)(...)`

Returns the solid of the part.

[Docs](https://developer.tekla.com/topic/en/18/43/1e4d454b-d3a0-4219-b4e2-7f661b402c4d)

#### `GetSolid(Solid.SolidCreationTypeEnum)(...)`

Returns the solid of the part.

[Docs](https://developer.tekla.com/topic/en/18/43/25f58541-37fe-c1ca-f384-065ecbe38e55)

#### `GetSolid.(...)`

Returns the solid of the part.

[Docs](https://developer.tekla.com/topic/en/18/43/7bd70e97-7c8b-823c-c7e1-da2ebe3dadaf)

#### `GetStringReportProperties(...)`

Retrieves multiple string report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/5ff72da1-7382-f5d2-eb1d-c99b46239d42)

#### `GetStringUserProperties(...)`

Retrieves all string properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/67120e84-c820-9d52-12ae-c167c595842c)

#### `GetSurfaceObjects(...)`

Returns an enumerator of all the connected surface objects.

[Docs](https://developer.tekla.com/topic/en/18/43/36268c4e-9006-1c9d-57bd-331f907da6ce)

#### `GetSurfaceTreatments(...)`

Returns an enumerator of all the connected surface treatments.

[Docs](https://developer.tekla.com/topic/en/18/43/26c1c1a5-e5db-9850-f739-c58bc05c963e)

#### `GetUserProperty(String, Double.)(...)`

Retrieves a double property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/17cc8074-7955-32b5-2b4b-b169c45d4ee0)

#### `GetUserProperty(String, Int32.)(...)`

Retrieves an integer property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/d9364c1d-5b57-6a31-4c13-01124058cfb2)

#### `GetUserProperty(String, String.)(...)`

Retrieves a string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/214a886c-c18c-9b66-d7c9-fe84260962f2)

#### `GetWelds(...)`

Returns an enumerator of all the connected welds.

[Docs](https://developer.tekla.com/topic/en/18/43/722de753-413b-f810-f09c-07388feb0afa)

#### `Insert(...)`

Inserts the model object instance into the model database.

[Docs](https://developer.tekla.com/topic/en/18/43/f17ed840-1d16-b843-4b0c-270875de1d35)

#### `Modify(...)`

Modifies the model instance in the model database.

[Docs](https://developer.tekla.com/topic/en/18/43/186833b8-4837-ed77-b314-3ebf9dc32d6a)

#### `Select(...)`

Selects the model object instance from the model database.

[Docs](https://developer.tekla.com/topic/en/18/43/f662ab10-be15-6280-d0aa-fc259dfd7c07)

#### `SetCustomObjectType(...)`

Sets a custom object type name for an existing object.

[Docs](https://developer.tekla.com/topic/en/18/43/3d6fb91b-48a9-35ca-d498-526514344089)

#### `SetDynamicStringProperty(...)`

Sets a dynamic string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/4dab0254-09e5-5e61-c28b-1c12afab71a0)

#### `SetLabel(...)`

Sets a label for an object when a new instance is created, this method must be called before Insert. The label is used in plug-ins for identifying the changed object in modification.

[Docs](https://developer.tekla.com/topic/en/18/43/3efcfc44-613a-e66b-9ebd-9f86df4f4036)

#### `SetPhase(...)`

Sets the phase of the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/ed80abcc-0ff7-d081-2229-e651f858a727)

#### `SetUserProperties(...)`

Sets multiple properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/8659da37-d571-f03f-c666-5c9fa3b113f5)

#### `SetUserProperty(String, Double)(...)`

Sets a double property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/5c585b8b-5c2e-7b75-2242-758f17429396)

#### `SetUserProperty(String, Int32)(...)`

Sets an integer property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/dd5b828d-3a87-bb58-5ac9-e1c3b4c2fe4b)

#### `SetUserProperty(String, String)(...)`

Sets a string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/80bf0348-b651-3d4a-862b-49c85bc924ab)

### Properties
- `AssemblyNumber` (object, get/set) — Gets or sets the assembly number. Defines the numbering in the assembly sense.
- `CastUnitType` (object, get/set) — Gets or sets the cast unit type of the part.
- `Class` (object, get/set) — Gets or sets the class of the part.
- `DeformingData` (object, get/set) — Gets or sets the deforming data of the part.
- `Finish` (object, get/set) — Gets or sets the finish of the part.
- `Identifier` (object, get/set) — The identifier of the object.
- `IsUpToDate` (object, get/set) — Gets if the object does not have a modification which is not shared.
- `Material` (object, get/set) — Gets or sets the material the part is made of.
- `ModificationTime` (object, get/set) — Gets latest time of the object was modified or created.
- `Name` (object, get/set) — Gets or sets the name of the part.
- `PartNumber` (object, get/set) — Gets or sets the part number. Defines the numbering in the part sense.
- `Position` (object, get/set) — Gets or sets the part position. Defines the way the part is positioned in the model.
- `PourPhase` (object, get/set) — Gets or sets the pour phase of the part.
- `Profile` (object, get/set) — Gets or sets the profile of the part.

## Part.CastUnitTypeEnum (enum)

The cast unit types.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/d8d763ec-4bed-04c2-a930-17d0fb53de92)

### Values
- `PRECAST` = `0` — The precast cast unit type.
- `CAST_IN_PLACE` = `1` — The cast in place cast unit type.

## Phase (class)

The Phase class defines a model object phase.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/2d4746c9-dc38-07d8-c644-b450af02d77d)

### Constructors
- `Phase(...)` — Instantiates a new phase with the phase number 0.
- `Phase(...)` — Instantiates a new phase with the given phase number.
- `Phase(...)` — Instantiates a new phase with the given parameters.

### Methods
#### `Delete(...)`

Deletes a phase. The current phase cannot be deleted.

[Docs](https://developer.tekla.com/topic/en/18/43/908ebf19-ac2e-5ea5-089d-8729f93c2569)

#### `GetUserProperty(String, Double.)(...)`

Retrieves a double property for the phase.

[Docs](https://developer.tekla.com/topic/en/18/43/88f635f5-af5a-f8de-142d-1f5aafe13cda)

#### `GetUserProperty(String, Int32.)(...)`

Retrieves an integer property for the phase.

[Docs](https://developer.tekla.com/topic/en/18/43/1637afd3-9740-d429-3636-4c665df77bd1)

#### `GetUserProperty(String, String.)(...)`

Retrieves a string property for the phase.

[Docs](https://developer.tekla.com/topic/en/18/43/3e94df7a-eb8d-986c-afd6-dd42c8d5cf16)

#### `Insert(...)`

Creates a new phase.

[Docs](https://developer.tekla.com/topic/en/18/43/3016a6b5-374c-abdc-a3c2-936850c22d67)

#### `Modify(...)`

Modifies a phase.

[Docs](https://developer.tekla.com/topic/en/18/43/1ca533f6-1cc6-47bb-9537-4812b55e976d)

#### `Select(...)`

Selects a phase.

[Docs](https://developer.tekla.com/topic/en/18/43/d107679d-0cab-291b-450e-82917ee51d77)

#### `SetUserProperty(String, Double)(...)`

Sets a double property for the phase.

[Docs](https://developer.tekla.com/topic/en/18/43/1b34bb94-be9d-b762-c6aa-c07f23a9aafe)

#### `SetUserProperty(String, Int32)(...)`

Sets an integer property for the phase.

[Docs](https://developer.tekla.com/topic/en/18/43/0fdc9fd6-07ce-feb9-4448-91f3279ec822)

#### `SetUserProperty(String, String)(...)`

Sets a string property for the phase.

[Docs](https://developer.tekla.com/topic/en/18/43/b6766490-3722-2bce-5ed3-f7c91182df8e)

### Properties
- `IsCurrentPhase` (object, get/set) — The value is 1 if the phase is the current phase, 0 otherwise.
- `PhaseComment` (object, get/set) — The phase comment.
- `PhaseName` (object, get/set) — The phase name.
- `PhaseNumber` (object, get/set) — The phase number.

## PhaseCollection (class)

The PhaseCollection class handles the collection of the model phases.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/8501ccf0-5464-8ee1-0155-52e30d1742a4)

### Methods
#### `CopyTo(...)`

Copies the elements of the ICollection to an Array, starting at a particular Array index.

[Docs](https://developer.tekla.com/topic/en/18/43/7a2ef0eb-1767-2b9a-13a9-1b9f88cafa46)

#### `GetEnumerator(...)`

Returns an enumerator that iterates through a collection.

[Docs](https://developer.tekla.com/topic/en/18/43/d9061c80-73fd-e533-e1c5-68d47d4873dc)

### Properties
- `Count` (object, get/set) — Gets the number of elements contained in the ICollection.
- `IsSynchronized` (object, get/set) — Gets a value indicating whether access to the ICollection is synchronized (thread safe).
- `SyncRoot` (object, get/set) — Gets an object that can be used to synchronize access to the ICollection.

## Plane (class)

The Plane class defines a plane in space using an origin, an X-axis and a Y-axis.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/37ad1799-48c1-2b90-a52c-fd0e84b4ec46)

### Constructors
- `Plane(...)` — Initializes a new plane instance.

### Properties
- `AxisX` (object, get/set) — The X-axis of the plane.
- `AxisY` (object, get/set) — The Y-axis of the plane.
- `Origin` (object, get/set) — The origin of the plane.

## PlateIntersectsWithIntersectionLineException (class)

The PlateIntersectsWithIntersectionLineException class represents an error when, plate polygon extends beyond intersection line.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/092874f5-d392-7d82-429f-f7a9167ca364)

### Constructors
- `PlateIntersectsWithIntersectionLineException(...)` — Initializes a new instance of the PlateIntersectsWithIntersectionLineException class with "Plate polygon extends beyond intersection line" error message.

## PointCloud (class)

Initializes a new instance of the PointCloud class.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/1b398fae-616f-2cfa-8940-23f4d30e11c5)

### Constructors
- `PointCloud(...)` — Creates a new point cloud instance using the given parameters.

### Methods
#### `Attach(...)`

Attach this point cloud to model. Can select only Potree type point clouds immediately after attach. Others need to be selected manually after attach has been completed. Use AttachComplete() to find out, if attach process has completed.

[Docs](https://developer.tekla.com/topic/en/18/43/959ad250-6b61-2aa2-7fbb-d8517edebe11)

#### `AttachComplete(...)`

Checks if the point cloud attach is complete. Needed when attaching non-Potree point cloud file, which needs conversion.

[Docs](https://developer.tekla.com/topic/en/18/43/75b49d08-b9b4-82d7-15c4-006d7bb75c77)

#### `Detach(...)`

Detach this point cloud from model.

[Docs](https://developer.tekla.com/topic/en/18/43/d78949bd-39ae-6564-b318-5079c4f245cc)

#### `GetPointClouds(...)`

Utility function: Gets available point clouds.

[Docs](https://developer.tekla.com/topic/en/18/43/1fea88c0-bef7-2351-4388-781f1da03780)

#### `GetVisibleInViews(...)`

Gets the views in which this point cloud is visible.

[Docs](https://developer.tekla.com/topic/en/18/43/ea50246a-4f69-ea97-6a67-c7734d15b73a)

#### `Select(...)`

Select point cloud by Guid, by Name, by OriginalPath or by Url.

[Docs](https://developer.tekla.com/topic/en/18/43/1cf3df03-e26a-0776-d572-596b4756c9a1)

#### `SetVisibility(...)`

Set point cloud visibility in given views.

[Docs](https://developer.tekla.com/topic/en/18/43/8f10f866-0f6e-a6ba-0e8a-9918e1ed65f8)

### Properties
- `BoundingBox` (object, get/set) — Gets or sets the bounding box.
- `Guid` (object, get/set) — Gets or sets the guid.
- `LocationBy` (object, get/set) — Gets or sets the location by value: Empty for model origin, base point guid for base point.
- `Name` (object, get/set) — Gets or sets the point clouds name. Abbreviated from the point cloud file name. Cannot be given by user.
- `OffsetX` (object, get/set) — Gets or sets the offset X.
- `OffsetY` (object, get/set) — Gets or sets the offset Y.
- `OffsetZ` (object, get/set) — Gets or sets the offset Z.
- `OriginalPath` (object, get/set) — Gets or sets the point clouds path (where it originally located).
- `RotationZ` (object, get/set) — Gets or sets the rotation Z.
- `Scale` (object, get/set) — Gets or sets the scale.
- `Url` (object, get/set) — Gets or sets the point clouds URL without a file name.
- `UseAutoCreatedBasePoint` (object, get/set) — Gets or sets a value indicating whether an auto-created base point should be used. If set to true, LocationBy value is ignored.

## PolyBeam (class)

The PolyBeam class represents a continuous beam with a contour as input.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/d879e653-76ab-ce1d-7b09-7abcdd6fa22e)

### Constructors
- `PolyBeam(...)` — Initializes a new instance of the PolyBeam class.
- `PolyBeam(...)` — Initializes a new instance of the PolyBeam class by using the defined type.

### Methods
#### `AddContourPoint(...)`

Adds a contour point to the polybeam.

[Docs](https://developer.tekla.com/topic/en/18/43/13797e11-3193-3c07-f07d-ba1bfc5d9880)

#### `CompareTo(Object)(...)`

Compares Identifiers of model objects.

[Docs](https://developer.tekla.com/topic/en/18/43/79cca356-0c8a-e8fb-463a-34ad4141bec7)

#### `CompareTo(Part)(...)`

Compares the instantiated part with another one.

[Docs](https://developer.tekla.com/topic/en/18/43/e8632c0d-89cf-9aab-6532-f62f9873067b)

#### `Delete(...)`

Deletes the polybeam instance with the given identifier from the model database.

[Docs](https://developer.tekla.com/topic/en/18/43/c78d6895-073c-8b36-f71a-6385c5aaaac7)

#### `Equals(...)`

Check if Identifiers of model objects are same.

[Docs](https://developer.tekla.com/topic/en/18/43/e4324265-2490-00de-4139-63a8f7acb492)

#### `GetAllReportProperties(...)`

Retrieves all properties for the model object in one hashtable. Type for the returned value must be checked using type casting.

[Docs](https://developer.tekla.com/topic/en/18/43/0708f404-ce77-28e1-4ebe-fa906f68bff8)

#### `GetAllUserProperties(...)`

Retrieves all properties for the model object in one hashtable. Type for the returned value must be checked using type casting.

[Docs](https://developer.tekla.com/topic/en/18/43/63b853a0-0af5-af9c-3ce7-05299664d7f8)

#### `GetAssembly(...)`

Returns the assembly that the part belongs to.

[Docs](https://developer.tekla.com/topic/en/18/43/bda18562-45a1-1355-42ed-6044af0b18cb)

#### `GetBolts(...)`

Returns an enumerator of all the connected bolts.

[Docs](https://developer.tekla.com/topic/en/18/43/dbc6b475-7461-7ec4-e8ec-7b7d3d8ff429)

#### `GetBooleans(...)`

Returns an enumerator of all the connected boolean objects.

[Docs](https://developer.tekla.com/topic/en/18/43/ae180e43-88e2-d759-0ae0-567d52681a2a)

#### `GetCenterLine(...)`

Returns the center line for the given part.

[Docs](https://developer.tekla.com/topic/en/18/43/bdc886b8-af7a-722c-dcb8-fddb2f250a83)

#### `GetCenterLinePolycurve(...)`

Get centerline as polycurve

[Docs](https://developer.tekla.com/topic/en/18/43/14c4b16d-226b-5359-d0ad-74a20db501b4)

#### `GetChildren(...)`

Returns an enumerator of all the children model objects.

[Docs](https://developer.tekla.com/topic/en/18/43/ce51ec40-310c-6067-a89f-f8d029aab747)

#### `GetComponents(...)`

Returns an enumerator of all the connected components, connections, seams and details inherited from the base component.

[Docs](https://developer.tekla.com/topic/en/18/43/ed57941c-2cd8-3b78-6a6e-848b8a920298)

#### `GetCoordinateSystem(...)`

Returns the coordinate system for the given model object.

[Docs](https://developer.tekla.com/topic/en/18/43/8adfe9cd-f6e5-6474-1297-33c6270a7b0a)

#### `GetCustomObjectType(...)`

Gets custom object type name from an existing object.

[Docs](https://developer.tekla.com/topic/en/18/43/d3be2ba3-8533-7165-f2d3-a5aa9df556b8)

#### `GetDSTVCoordinateSystem(...)`

Get DSTV coordinate system.

[Docs](https://developer.tekla.com/topic/en/18/43/9e70005f-c701-ab99-8fa0-4fce47b1ab05)

#### `GetDoubleReportProperties(...)`

Retrieves multiple double report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/f86f6c96-f738-49ca-16ba-f5634e862ba2)

#### `GetDoubleUserProperties(...)`

Retrieves all double properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/6ead7a63-5f76-0631-d480-d46c27e882aa)

#### `GetDynamicStringProperty(...)`

Gets a dynamic string property from the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/415e86d9-7de6-ea53-ce68-20b3e67b8655)

#### `GetFatherComponent(...)`

Returns the father component of the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/a2c28f3f-52ff-50a8-1b32-10d18db5c31d)

#### `GetHierarchicObjects(...)`

Returns an enumerator of all the connected hierarchic objects.

[Docs](https://developer.tekla.com/topic/en/18/43/cc228ea0-267b-7d90-7441-c9efc1779b08)

#### `GetIntegerReportProperties(...)`

Retrieves multiple integer report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/2623a9c0-2a9c-e233-9daa-59f92ed51445)

#### `GetIntegerUserProperties(...)`

Retrieves all integer properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/1b3c6f06-bc25-a54e-2a88-b60dc4a3c85e)

#### `GetPartMark(...)`

Returns the part mark.

[Docs](https://developer.tekla.com/topic/en/18/43/9978ba28-6304-714d-815c-fe4a6bb77076)

#### `GetPhase(...)`

Retrieves the phase of the model object (the phase number, the phase name, the phase comment and whether the phase is the current one or not).

[Docs](https://developer.tekla.com/topic/en/18/43/310f8b7a-c120-753a-35c9-f7c73a9806c9)

#### `GetPolybeamCoordinateSystems(...)`

Returns a list of coordinate systems for the polybeam segments.

[Docs](https://developer.tekla.com/topic/en/18/43/b3c1f127-9fce-4fa2-df7d-e8fac3eed715)

#### `GetPours(...)`

Returns an enumerator of all the pours that the part belongs to.

[Docs](https://developer.tekla.com/topic/en/18/43/1cc67ebb-329a-4675-7857-af663602d21d)

#### `GetReferenceLine(...)`

Returns the reference line for the given part.

[Docs](https://developer.tekla.com/topic/en/18/43/3e0a42c9-f10c-4225-bcd6-2b9b61ecc743)

#### `GetReinforcements(...)`

Returns an enumerator of all the connected reinforcements.

[Docs](https://developer.tekla.com/topic/en/18/43/510c913a-0653-6777-1841-c4f7df4fee47)

#### `GetReportProperty(String, Double.)(...)`

Retrieves a double property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/1c05aa56-5f42-be91-a59e-8d59a5549f45)

#### `GetReportProperty(String, Int32.)(...)`

Retrieves an integer property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/ce8e4ca0-3750-3c9b-5ca8-27e48b45e0bd)

#### `GetReportProperty(String, String.)(...)`

Retrieves a string property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/70f212bb-ee70-e9e9-7ab6-d6a9d46f8de6)

#### `GetSolid(FormingStates)(...)`

Returns the solid of the part.

[Docs](https://developer.tekla.com/topic/en/18/43/1e4d454b-d3a0-4219-b4e2-7f661b402c4d)

#### `GetSolid(Solid.SolidCreationTypeEnum)(...)`

Returns the solid of the part.

[Docs](https://developer.tekla.com/topic/en/18/43/25f58541-37fe-c1ca-f384-065ecbe38e55)

#### `GetSolid.(...)`

Returns the solid of the part.

[Docs](https://developer.tekla.com/topic/en/18/43/7bd70e97-7c8b-823c-c7e1-da2ebe3dadaf)

#### `GetStringReportProperties(...)`

Retrieves multiple string report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/5ff72da1-7382-f5d2-eb1d-c99b46239d42)

#### `GetStringUserProperties(...)`

Retrieves all string properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/67120e84-c820-9d52-12ae-c167c595842c)

#### `GetSurfaceObjects(...)`

Returns an enumerator of all the connected surface objects.

[Docs](https://developer.tekla.com/topic/en/18/43/36268c4e-9006-1c9d-57bd-331f907da6ce)

#### `GetSurfaceTreatments(...)`

Returns an enumerator of all the connected surface treatments.

[Docs](https://developer.tekla.com/topic/en/18/43/26c1c1a5-e5db-9850-f739-c58bc05c963e)

#### `GetUserProperty(String, Double.)(...)`

Retrieves a double property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/17cc8074-7955-32b5-2b4b-b169c45d4ee0)

#### `GetUserProperty(String, Int32.)(...)`

Retrieves an integer property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/d9364c1d-5b57-6a31-4c13-01124058cfb2)

#### `GetUserProperty(String, String.)(...)`

Retrieves a string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/214a886c-c18c-9b66-d7c9-fe84260962f2)

#### `GetWelds(...)`

Returns an enumerator of all the connected welds.

[Docs](https://developer.tekla.com/topic/en/18/43/722de753-413b-f810-f09c-07388feb0afa)

#### `Insert(...)`

Inserts the polybeam into the model database. All the attributes must be set.

[Docs](https://developer.tekla.com/topic/en/18/43/e040f76e-a5db-291e-75b2-a6280353f7ff)

#### `Modify(...)`

Modifies the existing polybeam in the model database to match the current one.

[Docs](https://developer.tekla.com/topic/en/18/43/592c5a0d-0f5c-6b05-04d6-982f17b6ed4d)

#### `Select(...)`

Selects a polybeam from the model database. The identifier of the polybeam must be set.

[Docs](https://developer.tekla.com/topic/en/18/43/a025a432-349a-8bbb-62e7-c03aa3d6565d)

#### `SetCustomObjectType(...)`

Sets a custom object type name for an existing object.

[Docs](https://developer.tekla.com/topic/en/18/43/3d6fb91b-48a9-35ca-d498-526514344089)

#### `SetDynamicStringProperty(...)`

Sets a dynamic string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/4dab0254-09e5-5e61-c28b-1c12afab71a0)

#### `SetLabel(...)`

Sets a label for an object when a new instance is created, this method must be called before Insert. The label is used in plug-ins for identifying the changed object in modification.

[Docs](https://developer.tekla.com/topic/en/18/43/3efcfc44-613a-e66b-9ebd-9f86df4f4036)

#### `SetPhase(...)`

Sets the phase of the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/ed80abcc-0ff7-d081-2229-e651f858a727)

#### `SetUserProperties(...)`

Sets multiple properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/8659da37-d571-f03f-c666-5c9fa3b113f5)

#### `SetUserProperty(String, Double)(...)`

Sets a double property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/5c585b8b-5c2e-7b75-2242-758f17429396)

#### `SetUserProperty(String, Int32)(...)`

Sets an integer property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/dd5b828d-3a87-bb58-5ac9-e1c3b4c2fe4b)

#### `SetUserProperty(String, String)(...)`

Sets a string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/80bf0348-b651-3d4a-862b-49c85bc924ab)

### Properties
- `AssemblyNumber` (object, get/set) — Gets or sets the assembly number. Defines the numbering in the assembly sense.
- `CastUnitType` (object, get/set) — Gets or sets the cast unit type of the part.
- `Class` (object, get/set) — Gets or sets the class of the part.
- `Contour` (object, get/set) — Gets or sets the contour for the polybeam.
- `DeformingData` (object, get/set) — Gets or sets the deforming data of the part.
- `Finish` (object, get/set) — Gets or sets the finish of the part.
- `Identifier` (object, get/set) — The identifier of the object.
- `IsUpToDate` (object, get/set) — Gets if the object does not have a modification which is not shared.
- `Material` (object, get/set) — Gets or sets the material the part is made of.
- `ModificationTime` (object, get/set) — Gets latest time of the object was modified or created.
- `Name` (object, get/set) — Gets or sets the name of the part.
- `PartNumber` (object, get/set) — Gets or sets the part number. Defines the numbering in the part sense.
- `Position` (object, get/set) — Gets or sets the part position. Defines the way the part is positioned in the model.
- `PourPhase` (object, get/set) — Gets or sets the pour phase of the part.
- `Profile` (object, get/set) — Gets or sets the profile of the part.
- `Type` (object, get/set) — Gets the read only type of the polybeam. The default type for a new polybeam instance is beam.

## PolyBeam.PolyBeamTypeEnum (enum)

The polybeam types.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/252fe607-40e6-7326-862c-944f6c6e4513)

### Values
- `BEAM` = `0` — The polybeam.
- `PANEL` = `1` — The panel.
- `STRIP_FOOTING` = `2` — The strip footing.
- `COLUMN` = `3` — The column.

## Polygon (class)

The Polygon class represents a polygon object that has its corners at the given points.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/b08e87a3-a223-a38f-882e-91c412b7db5e)

### Constructors
- `Polygon(...)` — Initializes a new polygon instance with empty attributes.

### Properties
- `Points` (object, get/set) — The corners of the polygon.

## PolygonNode (class)

The PolygonNode class represents flat geometry tree node.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/8b8b4b86-6354-8422-b25e-3cb648755976)

### Constructors
- `PolygonNode(...)` — Initializes a new instance of the PolygonNode class.

### Methods
#### `AcceptVisitor(...)`

Entry method for the visitor pattern

[Docs](https://developer.tekla.com/topic/en/18/43/ea91906f-dfb0-e192-f2d5-c912453e9568)

#### `Clone(...)`

Creates a new object that is a copy of the current instance.

[Docs](https://developer.tekla.com/topic/en/18/43/947896a5-05f7-c3e7-e570-703dc743fdf1)

### Properties
- `Contour` (object, get/set) — Gets or sets the contour points of flat geometry.
- `IsAutomatic` (object, get/set) — Gets a value indicating whether this geometry node was automatically generated (returns false if it was originally a user-defined part)

## PolygonWeld (class)

The PolygonWeld class represents a polygon weld in the model. A polygon weld has a main part and a secondary part.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/4b43dd4d-bd93-e2a8-7bf6-03cd05b834d6)

### Constructors
- `PolygonWeld(...)` — Creates a new polygon weld instance.

### Methods
#### `CompareTo(...)`

Compares Identifiers of model objects.

[Docs](https://developer.tekla.com/topic/en/18/43/79cca356-0c8a-e8fb-463a-34ad4141bec7)

#### `Delete(...)`

Deletes the polygon weld instance with the given identifier from the model database.

[Docs](https://developer.tekla.com/topic/en/18/43/84cb8cb2-ac67-ebb8-cc31-d258d43ed9fd)

#### `Equals(...)`

Check if Identifiers of model objects are same.

[Docs](https://developer.tekla.com/topic/en/18/43/e4324265-2490-00de-4139-63a8f7acb492)

#### `GetAllReportProperties(...)`

Retrieves all properties for the model object in one hashtable. Type for the returned value must be checked using type casting.

[Docs](https://developer.tekla.com/topic/en/18/43/0708f404-ce77-28e1-4ebe-fa906f68bff8)

#### `GetAllUserProperties(...)`

Retrieves all properties for the model object in one hashtable. Type for the returned value must be checked using type casting.

[Docs](https://developer.tekla.com/topic/en/18/43/63b853a0-0af5-af9c-3ce7-05299664d7f8)

#### `GetChildren(...)`

Returns an enumerator of all the children model objects.

[Docs](https://developer.tekla.com/topic/en/18/43/ce51ec40-310c-6067-a89f-f8d029aab747)

#### `GetCoordinateSystem(...)`

Returns the coordinate system for the given model object.

[Docs](https://developer.tekla.com/topic/en/18/43/8adfe9cd-f6e5-6474-1297-33c6270a7b0a)

#### `GetCustomObjectType(...)`

Gets custom object type name from an existing object.

[Docs](https://developer.tekla.com/topic/en/18/43/d3be2ba3-8533-7165-f2d3-a5aa9df556b8)

#### `GetDoubleReportProperties(...)`

Retrieves multiple double report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/f86f6c96-f738-49ca-16ba-f5634e862ba2)

#### `GetDoubleUserProperties(...)`

Retrieves all double properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/6ead7a63-5f76-0631-d480-d46c27e882aa)

#### `GetDynamicStringProperty(...)`

Gets a dynamic string property from the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/415e86d9-7de6-ea53-ce68-20b3e67b8655)

#### `GetFatherComponent(...)`

Returns the father component of the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/a2c28f3f-52ff-50a8-1b32-10d18db5c31d)

#### `GetHierarchicObjects(...)`

Returns an enumerator of all the connected hierarchic objects.

[Docs](https://developer.tekla.com/topic/en/18/43/cc228ea0-267b-7d90-7441-c9efc1779b08)

#### `GetIntegerReportProperties(...)`

Retrieves multiple integer report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/2623a9c0-2a9c-e233-9daa-59f92ed51445)

#### `GetIntegerUserProperties(...)`

Retrieves all integer properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/1b3c6f06-bc25-a54e-2a88-b60dc4a3c85e)

#### `GetLogicalWeld(...)`

Gets the logical weld where the weld belongs. Returns false if the weld does not belong to any logical weld.

[Docs](https://developer.tekla.com/topic/en/18/43/167aa52e-fd36-2fd1-1caf-fe1484531ce9)

#### `GetPhase(...)`

Retrieves the phase of the model object (the phase number, the phase name, the phase comment and whether the phase is the current one or not).

[Docs](https://developer.tekla.com/topic/en/18/43/310f8b7a-c120-753a-35c9-f7c73a9806c9)

#### `GetReportProperty(String, Double.)(...)`

Retrieves a double property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/1c05aa56-5f42-be91-a59e-8d59a5549f45)

#### `GetReportProperty(String, Int32.)(...)`

Retrieves an integer property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/ce8e4ca0-3750-3c9b-5ca8-27e48b45e0bd)

#### `GetReportProperty(String, String.)(...)`

Retrieves a string property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/70f212bb-ee70-e9e9-7ab6-d6a9d46f8de6)

#### `GetSolid(...)`

Method for getting the weld solid.

[Docs](https://developer.tekla.com/topic/en/18/43/319baa05-e7f2-e9ff-1f4f-c2a403dd71d1)

#### `GetStringReportProperties(...)`

Retrieves multiple string report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/5ff72da1-7382-f5d2-eb1d-c99b46239d42)

#### `GetStringUserProperties(...)`

Retrieves all string properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/67120e84-c820-9d52-12ae-c167c595842c)

#### `GetUserProperty(String, Double.)(...)`

Retrieves a double property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/17cc8074-7955-32b5-2b4b-b169c45d4ee0)

#### `GetUserProperty(String, Int32.)(...)`

Retrieves an integer property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/d9364c1d-5b57-6a31-4c13-01124058cfb2)

#### `GetUserProperty(String, String.)(...)`

Retrieves a string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/214a886c-c18c-9b66-d7c9-fe84260962f2)

#### `GetWeldGeometries(...)`

Method for getting weld seam geometries. Every result represents one individual seam geometry in current weld.

[Docs](https://developer.tekla.com/topic/en/18/43/335a1753-ab21-bc4a-f02b-2dfa5e951c7a)

#### `Insert(...)`

Inserts the polygon weld into the model database. All the attributes must be set.

[Docs](https://developer.tekla.com/topic/en/18/43/a42589d9-34bb-2dd2-3bd1-05d9849bc791)

#### `Modify(...)`

Modifies the existing polygon weld in the model database to match the current one. The modification cannot be done if the polygon weld is a part of a logical weld.

[Docs](https://developer.tekla.com/topic/en/18/43/af8c89e8-d15f-7612-d031-9ba767083386)

#### `Select(...)`

Selects a polygon weld from the model database. The polygon weld identifier must be set.

[Docs](https://developer.tekla.com/topic/en/18/43/b576ddee-2abc-cde3-bac6-e265b8d9194d)

#### `SetCustomObjectType(...)`

Sets a custom object type name for an existing object.

[Docs](https://developer.tekla.com/topic/en/18/43/3d6fb91b-48a9-35ca-d498-526514344089)

#### `SetDynamicStringProperty(...)`

Sets a dynamic string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/4dab0254-09e5-5e61-c28b-1c12afab71a0)

#### `SetLabel(...)`

Sets a label for an object when a new instance is created, this method must be called before Insert. The label is used in plug-ins for identifying the changed object in modification.

[Docs](https://developer.tekla.com/topic/en/18/43/3efcfc44-613a-e66b-9ebd-9f86df4f4036)

#### `SetPhase(...)`

Sets the phase of the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/ed80abcc-0ff7-d081-2229-e651f858a727)

#### `SetUserProperties(...)`

Sets multiple properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/8659da37-d571-f03f-c666-5c9fa3b113f5)

#### `SetUserProperty(String, Double)(...)`

Sets a double property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/5c585b8b-5c2e-7b75-2242-758f17429396)

#### `SetUserProperty(String, Int32)(...)`

Sets an integer property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/dd5b828d-3a87-bb58-5ac9-e1c3b4c2fe4b)

#### `SetUserProperty(String, String)(...)`

Sets a string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/80bf0348-b651-3d4a-862b-49c85bc924ab)

### Properties
- `AdditionalSizeAbove` (object, get/set) — Gets or sets the additional size above for combination welds.
- `AdditionalSizeBelow` (object, get/set) — Gets or sets the additional size below for combination welds.
- `AngleAbove` (object, get/set) — Gets or sets the angle above.
- `AngleBelow` (object, get/set) — Gets or sets the angle below.
- `AroundWeld` (object, get/set) — Gets or sets a value indicating whether the weld is an around weld (true) or an edge weld (false).
- `ConnectAssemblies` (object, get/set) — Gets or sets a value indicating whether to connect a part or an assembly as a secondary part (false) or as a sub-assembly (true).
- `ContourAbove` (object, get/set) — Gets or sets the contour above.
- `ContourBelow` (object, get/set) — Gets or sets the contour below.
- `EffectiveThroatAbove` (object, get/set) — Gets or sets the effective throat above.
- `EffectiveThroatBelow` (object, get/set) — Gets or sets the effective throat below.
- `ElectrodeClassification` (object, get/set) — Gets or sets the weld electrode classification.
- `ElectrodeCoefficient` (object, get/set) — Gets or sets the electrode strength coefficient.
- `ElectrodeStrength` (object, get/set) — Gets or sets the electrode strength.
- `FinishAbove` (object, get/set) — Gets or sets the finish above.
- `FinishBelow` (object, get/set) — Gets or sets the finish below.
- `Identifier` (object, get/set) — The identifier of the object.
- `IncrementAmountAbove` (object, get/set) — Gets or sets the increment amount above.
- `IncrementAmountBelow` (object, get/set) — Gets or sets the increment amount below.
- `IntermittentType` (object, get/set) — Gets or sets the weld intermittent type.
- `IsUpToDate` (object, get/set) — Gets if the object does not have a modification which is not shared.
- `LengthAbove` (object, get/set) — Gets or sets the length above.
- `LengthBelow` (object, get/set) — Gets or sets the length below.
- `MainObject` (object, get/set) — Gets or sets the main part of the weld.
- `ModificationTime` (object, get/set) — Gets latest time of the object was modified or created.
- `NDTInspection` (object, get/set) — Gets or sets the NDT inspection level.
- `PitchAbove` (object, get/set) — Gets or sets the pitch above.
- `PitchBelow` (object, get/set) — Gets or sets the pitch below.
- `Placement` (object, get/set) — Gets or sets the weld placement.
- `Polygon` (object, get/set) — The polygon for the polygon weld.
- `PrefixAboveLine` (object, get/set) — Gets or sets the size prefix above the line.
- `PrefixBelowLine` (object, get/set) — Gets or sets the size prefix below the line.
- `Preparation` (object, get/set) — Gets or sets the weld preparation.
- `ProcessType` (object, get/set) — Gets or sets the process type.
- `ReferenceText` (object, get/set) — Gets or sets the reference text.
- `RootFaceAbove` (object, get/set) — Gets or sets the root face above.
- `RootFaceBelow` (object, get/set) — Gets or sets the root face below.
- `RootOpeningAbove` (object, get/set) — Gets or sets the root opening above.
- `RootOpeningBelow` (object, get/set) — Gets or sets the root opening below.
- `SecondaryObject` (object, get/set) — Gets or sets the secondary part of the weld.
- `ShopWeld` (object, get/set) — Gets or sets a value indicating whether the weld is a shop weld (true) or a site weld (false).
- `SizeAbove` (object, get/set) — Gets or sets the size above.
- `SizeBelow` (object, get/set) — Gets or sets the size below.
- `Standard` (object, get/set) — Gets or sets the weld detail/standard.
- `StitchWeld` (object, get/set) — Gets or sets a value indicating whether the weld is stitched (true) or not stitched (false).
- `TypeAbove` (object, get/set) — Gets or sets the type above.
- `TypeBelow` (object, get/set) — Gets or sets the type below.
- `WeldNumber` (object, get/set) — Gets the weld number.
- `WeldNumberPrefix` (object, get/set) — Gets or sets the weld number prefix.

## Polymesh (class)

The Polygon class represents a polygon object that has its corners at the given points.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/eef01483-89f4-d8bc-17e6-420d6156db89)

### Constructors
- `Polymesh(...)` — Initializes a new instance of the Polymesh class

### Methods
#### `CompareFingerprints(...)`

Compares two fingerprints

[Docs](https://developer.tekla.com/topic/en/18/43/7d5c6766-b766-9e11-da23-5877d24d23ba)

#### `Convert(...)`

Converts a struct of polymeshes to a list of CopyFacetedBrep.

[Docs](https://developer.tekla.com/topic/en/18/43/32e5d693-cecf-b650-a701-11c119f1fbd1)

#### `ConvertInvalidInfoFromStruct(...)`

Convert a struct of invalid info to a list of pairs of outer loop index and error code. Read indexes and error codes from the integer list.

[Docs](https://developer.tekla.com/topic/en/18/43/e8630e24-1d44-86b8-3ed7-95417f96318e)

#### `ConvertToStruct(...)`

Converts a CopyFacetedBrep to struct dot polymesh.

[Docs](https://developer.tekla.com/topic/en/18/43/56c98044-7935-f0a3-bd35-adfae51376ec)

#### `Fingerprint(...)`

Gets fingerprint information for input brep

[Docs](https://developer.tekla.com/topic/en/18/43/5cb24120-b307-defd-1efd-157082e55a18)

#### `FromStruct(...)`

Converts a struct of polymesh to a CopyFacetedBrep.

[Docs](https://developer.tekla.com/topic/en/18/43/fa3e031a-f57d-6525-c6d4-d3cbb8680918)

#### `GetSolidBrep(...)`

Gets the cleaned Brep

[Docs](https://developer.tekla.com/topic/en/18/43/9b11cd0e-703b-8a52-2192-f31f5be5e55b)

#### `ToStruct(...)`

Converts a CopyFacetedBrep to dotnet polymesh struct.

[Docs](https://developer.tekla.com/topic/en/18/43/c98e4037-98cc-3ff2-a650-7b0bfa910cdc)

#### `Validate(...)`

Converts a CopyFacetedBrep to a polymesh and validates.

[Docs](https://developer.tekla.com/topic/en/18/43/972f3ac2-c678-16b5-c9f6-110fb4d6a7cd)

### Properties
- `Brep` (object, get/set) — Gets or sets the polymesh that forms the object geometry.

## Polymesh.PolymeshCheckerFlags (enum)

Polymesh check flags, please refer to PolymeshHealthCheckEnum for the corresponding result values.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/9f06f9d1-a8fb-41f2-aa21-b962d632fcdb)

### Values
- `None` = `0` — Perform no check
- `LoopVertexUnique` = `2048` — To check that no two consecutive vertex index may be the same.
- `VerticesOnSameEdge` = `1` — To check outerloop vertices on same edge, distance tolerance 0.01.
- `OuterloopPlanarity` = `2` — To check outerloop's planarity, IN_PLANE_EPSILON 1.0e-3.
- `OuterloopSelfIntersection` = `4` — To check if outerloop intersects itself, EPSILON 1.E-4
- `InnerloopPlanarity` = `8` — To check innterloop's planarity, IN_PLANE_EPSILON 1.0e-3.
- `InnerloopSelfIntersection` = `16` — To check if innerloop intersects itself, EPSILON 1.E-4.
- `InnerloopInsideOuterloop` = `256` — To check if innerloop inside outerloop, EPSILON 1.E-4.
- `LoopNormalValidity` = `32` — The inner loop is oriented clockwise, the opposite of the outer loop (counterclockwise).
- `FaceEdgeOrientation` = `64` — To check face's edge orientation.
- `Multishellness` = `128` — To check are there more than one topologically connected components in the polymesh. This checks only through vertex indexes.
- `MultishellnessGeometrical` = `16384` — To check are there more than one geometrically connected components in the polymesh. Check with all vertex edge face position containment (includes partial edge touching cases). This check can be slow, the behavior is O(n^2).
- `VerticesOnSamePosition` = `512` — To check if there are any vertices that are in the same location, EPSILON 0.01.
- `UnusedVertices` = `1024` — To check if there are any unused vertices in the vertex array.
- `NonManifoldEdges` = `4096` — Are there non-manifold edges (indirect edge vertices are found for more than 2 faces).
- `NullFaces` = `8192` — To check if all vertices of the face are linearly dependent, or having less than 3 vertices, or smaller than allowed edges.
- `All` = `511` — To check all criteria.

## Polymesh.PolymeshHealthCheckEnum (enum)

Polymesh check error result codes. See the PolymeshCheckerFlags to see how to set the various checks.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/d3116a3b-bd01-49c1-ee54-56b2e9a2fb30)

### Values
- `PolymeshLevelError` = `-2` — The vertex array contains vertices that are geometrically equivalent or the polymesh contains unused vertices
- `PolymeshUndefined` = `-1` — Polymesh is null, the creation failed or unknown reasons.
- `PolymeshOk` = `0` — Valid polymesh
- `NumOfVerticesOuterloop` = `1` — Number of vertices in outer loop too small
- `NumOfVerticesInnerloop` = `2` — Number of vertices in innerloop too small
- `VertexArrayOuterloop` = `3` — Check of indices of vertices in outer loop failed
- `VertexArrayInnerloop` = `4` — Check of indices of vertices in the innerloop failed
- `InnerloopNotInsideOuterloop` = `5` — Inner loop is not inside the outer loop
- `VertexArrayForListOfEdges` = `6` — Check of indices of vertices for edge list failed
- `NonPlanarOuterloop` = `7` — Outer loop points are not in the same plane with each other
- `NonPlanarInnerloop` = `8` — Inner loop points are not in the same plane with each other or with outer loop
- `SelfIntersectingOuterloop` = `9` — Outer loop is self-intersecting
- `SelfIntersectingInnerloop` = `10` — Inner loop is self-intersecting
- `LoopNormalsInvalid` = `11` — Inner loop normal direction should be opposite to outer loop
- `FaceEdgesInvalidOrientation` = `12` — Two neighbor faces with common edge have wrong orientation
- `FaceEdgesPartlyInvalidOrientation` = `13` — Two neighbor faces with partly common edge have wrong orientation
- `MultishellPolymesh` = `14` — Polymesh consists topologically of multiple shells
- `MultishellPolymeshByGeometry` = `21` — Polymesh consists geometrically of multiple shells
- `NumberOfFaces` = `15` — Number of faces too small
- `VertexArrayUniqueness` = `16` — The vertex array contains vertices that are geometrically equivalent
- `VertexDuplicatesOnPolygon` = `17` — The same vertex index appears at least twice for a face
- `VertexCollinearOnPolygon` = `18` — The face has at least one vertex that is collinear with others on an edge
- `NonManifoldEdges` = `19` — The polymesh contains non-manifold edges
- `UnusedVertices` = `20` — The polymesh contains unused vertices

## PolymeshEnumerator (class)

The PolymeshEnumerator class is used to enumerate the polymesh of a pour's merged faces.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/03636ab7-e0d2-62e7-0be0-0ec9962228c9)

### Methods
#### `MoveNext(...)`

Moves to the next item in the enumerator.

[Docs](https://developer.tekla.com/topic/en/18/43/82dc58b4-4e6b-925a-725d-545c3b19448f)

#### `Reset(...)`

Resets the enumerator to the beginning.

[Docs](https://developer.tekla.com/topic/en/18/43/0b194b30-9327-5016-a078-7e7ca52171c2)

### Properties
- `Current` (object, get/set) — Gets the current polymesh.

## Position (class)

The Position class defines how a part is positioned relative to the input.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/4fd3fb0d-31ad-371e-d499-4b623d412b61)

### Constructors
- `Position(...)` — Creates a new position instance.

### Properties
- `Depth` (object, get/set) — Indicates the positioning in the depth sense.
- `DepthOffset` (object, get/set) — The offset away from the Depth value.
- `Plane` (object, get/set) — Indicates the positioning in the plane sense.
- `PlaneOffset` (object, get/set) — The offset away from the Plane value.
- `Rotation` (object, get/set) — Indicates the positioning in the rotational sense.
- `RotationOffset` (object, get/set) — The offset away from the Rotation value.

## Position.DepthEnum (enum)

The position in the depth sense.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/c532ad57-9a3c-8754-bccc-797db0151372)

### Values
- `MIDDLE` = `0` — The middle option.
- `FRONT` = `1` — The front option.
- `BEHIND` = `2` — The behind option.

## Position.PlaneEnum (enum)

The position in the plane sense.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/c599df35-04b2-a919-1c2a-92c3b8be2a94)

### Values
- `MIDDLE` = `0` — The middle option.
- `LEFT` = `1` — The left option.
- `RIGHT` = `2` — The right option.

## Position.RotationEnum (enum)

The position in the rotational sense.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/fae3579c-28af-3001-0871-db7105a8daa3)

### Values
- `FRONT` = `0` — The front option.
- `TOP` = `1` — The top option.
- `BACK` = `2` — The back option.
- `BELOW` = `3` — The below option.

## PourBreak (class)

The PourBreak class represents a pour break in a model.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/d794b218-b226-027e-7d13-52c04efc8734)

### Constructors
- `PourBreak(...)` — Initializes a new instance of the PourBreak class

### Methods
#### `CompareTo(...)`

Compares Identifiers of model objects.

[Docs](https://developer.tekla.com/topic/en/18/43/79cca356-0c8a-e8fb-463a-34ad4141bec7)

#### `Delete(...)`

Deletes the model object from the model database.

[Docs](https://developer.tekla.com/topic/en/18/43/a401e498-6485-ed97-e7ec-56ab3eb9177c)

#### `Equals(...)`

Check if Identifiers of model objects are same.

[Docs](https://developer.tekla.com/topic/en/18/43/e4324265-2490-00de-4139-63a8f7acb492)

#### `GetAllReportProperties(...)`

Retrieves all properties for the model object in one hashtable. Type for the returned value must be checked using type casting.

[Docs](https://developer.tekla.com/topic/en/18/43/0708f404-ce77-28e1-4ebe-fa906f68bff8)

#### `GetAllUserProperties(...)`

Retrieves all properties for the model object in one hashtable. Type for the returned value must be checked using type casting.

[Docs](https://developer.tekla.com/topic/en/18/43/63b853a0-0af5-af9c-3ce7-05299664d7f8)

#### `GetChildren(...)`

Returns an enumerator of all the children model objects.

[Docs](https://developer.tekla.com/topic/en/18/43/ce51ec40-310c-6067-a89f-f8d029aab747)

#### `GetCoordinateSystem(...)`

Returns the coordinate system for the given model object.

[Docs](https://developer.tekla.com/topic/en/18/43/8adfe9cd-f6e5-6474-1297-33c6270a7b0a)

#### `GetCustomObjectType(...)`

Gets custom object type name from an existing object.

[Docs](https://developer.tekla.com/topic/en/18/43/d3be2ba3-8533-7165-f2d3-a5aa9df556b8)

#### `GetDoubleReportProperties(...)`

Retrieves multiple double report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/f86f6c96-f738-49ca-16ba-f5634e862ba2)

#### `GetDoubleUserProperties(...)`

Retrieves all double properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/6ead7a63-5f76-0631-d480-d46c27e882aa)

#### `GetDynamicStringProperty(...)`

Gets a dynamic string property from the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/415e86d9-7de6-ea53-ce68-20b3e67b8655)

#### `GetFatherComponent(...)`

Returns the father component of the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/a2c28f3f-52ff-50a8-1b32-10d18db5c31d)

#### `GetHierarchicObjects(...)`

Returns an enumerator of all the connected hierarchic objects.

[Docs](https://developer.tekla.com/topic/en/18/43/cc228ea0-267b-7d90-7441-c9efc1779b08)

#### `GetIntegerReportProperties(...)`

Retrieves multiple integer report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/2623a9c0-2a9c-e233-9daa-59f92ed51445)

#### `GetIntegerUserProperties(...)`

Retrieves all integer properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/1b3c6f06-bc25-a54e-2a88-b60dc4a3c85e)

#### `GetPhase(...)`

Retrieves the phase of the model object (the phase number, the phase name, the phase comment and whether the phase is the current one or not).

[Docs](https://developer.tekla.com/topic/en/18/43/310f8b7a-c120-753a-35c9-f7c73a9806c9)

#### `GetReportProperty(String, Double.)(...)`

Retrieves a double property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/1c05aa56-5f42-be91-a59e-8d59a5549f45)

#### `GetReportProperty(String, Int32.)(...)`

Retrieves an integer property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/ce8e4ca0-3750-3c9b-5ca8-27e48b45e0bd)

#### `GetReportProperty(String, String.)(...)`

Retrieves a string property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/70f212bb-ee70-e9e9-7ab6-d6a9d46f8de6)

#### `GetStringReportProperties(...)`

Retrieves multiple string report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/5ff72da1-7382-f5d2-eb1d-c99b46239d42)

#### `GetStringUserProperties(...)`

Retrieves all string properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/67120e84-c820-9d52-12ae-c167c595842c)

#### `GetUserProperty(String, Double.)(...)`

Retrieves a double property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/17cc8074-7955-32b5-2b4b-b169c45d4ee0)

#### `GetUserProperty(String, Int32.)(...)`

Retrieves an integer property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/d9364c1d-5b57-6a31-4c13-01124058cfb2)

#### `GetUserProperty(String, String.)(...)`

Retrieves a string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/214a886c-c18c-9b66-d7c9-fe84260962f2)

#### `Insert(...)`

Inserts a new model object. The geometry of the object needs to be set by using the public Polymesh function, before calling insert.

[Docs](https://developer.tekla.com/topic/en/18/43/1ada69ff-53a0-51a4-16b0-81b6023df7d9)

#### `Modify(...)`

Modifies the existing model object in the model database to match the current one.

[Docs](https://developer.tekla.com/topic/en/18/43/5a86cef4-be24-5c16-5765-dff715441e5e)

#### `Select(...)`

Selects the model object from the model by the identifier of this instance.

[Docs](https://developer.tekla.com/topic/en/18/43/99bc1a18-6182-77f4-6f95-ed29f804cfde)

#### `SetCustomObjectType(...)`

Sets a custom object type name for an existing object.

[Docs](https://developer.tekla.com/topic/en/18/43/3d6fb91b-48a9-35ca-d498-526514344089)

#### `SetDynamicStringProperty(...)`

Sets a dynamic string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/4dab0254-09e5-5e61-c28b-1c12afab71a0)

#### `SetLabel(...)`

Sets a label for an object when a new instance is created, this method must be called before Insert. The label is used in plug-ins for identifying the changed object in modification.

[Docs](https://developer.tekla.com/topic/en/18/43/3efcfc44-613a-e66b-9ebd-9f86df4f4036)

#### `SetPhase(...)`

Sets the phase of the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/ed80abcc-0ff7-d081-2229-e651f858a727)

#### `SetUserProperties(...)`

Sets multiple properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/8659da37-d571-f03f-c666-5c9fa3b113f5)

#### `SetUserProperty(String, Double)(...)`

Sets a double property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/5c585b8b-5c2e-7b75-2242-758f17429396)

#### `SetUserProperty(String, Int32)(...)`

Sets an integer property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/dd5b828d-3a87-bb58-5ac9-e1c3b4c2fe4b)

#### `SetUserProperty(String, String)(...)`

Sets a string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/80bf0348-b651-3d4a-862b-49c85bc924ab)

### Properties
- `Identifier` (object, get/set) — The identifier of the object.
- `IsUpToDate` (object, get/set) — Gets if the object does not have a modification which is not shared.
- `ModificationTime` (object, get/set) — Gets latest time of the object was modified or created.
- `Polymesh` (object, get/set) — Gets or sets the polymesh that forms the object geometry.

## PourObject (class)

The PourObject class represents a pour object in the model.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/6cc031f5-71e3-2274-74ef-5b12f9954ff7)

### Constructors
- `PourObject(...)` — Initializes a new instance of the PourObject class.

### Methods
#### `CompareTo(...)`

Compares Identifiers of model objects.

[Docs](https://developer.tekla.com/topic/en/18/43/79cca356-0c8a-e8fb-463a-34ad4141bec7)

#### `Delete(...)`

At the moment pour objects cannot be deleted.

[Docs](https://developer.tekla.com/topic/en/18/43/5d1f198b-8929-ff55-24a9-e81cabfd0a1f)

#### `Equals(...)`

Check if Identifiers of model objects are same.

[Docs](https://developer.tekla.com/topic/en/18/43/e4324265-2490-00de-4139-63a8f7acb492)

#### `GetAllReportProperties(...)`

Retrieves all properties for the model object in one hashtable. Type for the returned value must be checked using type casting.

[Docs](https://developer.tekla.com/topic/en/18/43/0708f404-ce77-28e1-4ebe-fa906f68bff8)

#### `GetAllUserProperties(...)`

Retrieves all properties for the model object in one hashtable. Type for the returned value must be checked using type casting.

[Docs](https://developer.tekla.com/topic/en/18/43/63b853a0-0af5-af9c-3ce7-05299664d7f8)

#### `GetAssembly(...)`

Returns the assembly that the pour object belongs to.

[Docs](https://developer.tekla.com/topic/en/18/43/9c2695c0-48e8-2846-b587-3b686358ae8c)

#### `GetChildren(...)`

Returns an enumerator of all the children model objects.

[Docs](https://developer.tekla.com/topic/en/18/43/ce51ec40-310c-6067-a89f-f8d029aab747)

#### `GetCoordinateSystem(...)`

Returns the coordinate system for the given model object.

[Docs](https://developer.tekla.com/topic/en/18/43/8adfe9cd-f6e5-6474-1297-33c6270a7b0a)

#### `GetCustomObjectType(...)`

Gets custom object type name from an existing object.

[Docs](https://developer.tekla.com/topic/en/18/43/d3be2ba3-8533-7165-f2d3-a5aa9df556b8)

#### `GetDoubleReportProperties(...)`

Retrieves multiple double report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/f86f6c96-f738-49ca-16ba-f5634e862ba2)

#### `GetDoubleUserProperties(...)`

Retrieves all double properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/6ead7a63-5f76-0631-d480-d46c27e882aa)

#### `GetDynamicStringProperty(...)`

Gets a dynamic string property from the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/415e86d9-7de6-ea53-ce68-20b3e67b8655)

#### `GetFatherComponent(...)`

Returns the father component of the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/a2c28f3f-52ff-50a8-1b32-10d18db5c31d)

#### `GetFatherPourUnit(...)`

Returns the pour unit that the pour object belongs to.

[Docs](https://developer.tekla.com/topic/en/18/43/b7f94efa-d237-6e4f-99d8-fa0ab99b2ff6)

#### `GetHierarchicObjects(...)`

Returns an enumerator of all the connected hierarchic objects.

[Docs](https://developer.tekla.com/topic/en/18/43/cc228ea0-267b-7d90-7441-c9efc1779b08)

#### `GetIntegerReportProperties(...)`

Retrieves multiple integer report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/2623a9c0-2a9c-e233-9daa-59f92ed51445)

#### `GetIntegerUserProperties(...)`

Retrieves all integer properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/1b3c6f06-bc25-a54e-2a88-b60dc4a3c85e)

#### `GetObjects(...)`

Returns an enumerator of all objects related to pour.

[Docs](https://developer.tekla.com/topic/en/18/43/e8e903c5-9aa9-be68-8f96-6c9a6d532f7d)

#### `GetParts(...)`

Returns an enumerator of all parts related to pour.

[Docs](https://developer.tekla.com/topic/en/18/43/39ed13a1-e41d-23b6-8e13-9b197554ef9e)

#### `GetPhase(...)`

Retrieves the phase of the model object (the phase number, the phase name, the phase comment and whether the phase is the current one or not).

[Docs](https://developer.tekla.com/topic/en/18/43/310f8b7a-c120-753a-35c9-f7c73a9806c9)

#### `GetPourPolymeshes(...)`

Returns a new polymesh enumerator based on merged faces of the pour object in the current plane.

[Docs](https://developer.tekla.com/topic/en/18/43/dcba2f4e-58df-edb0-5341-1739f822aeac)

#### `GetReportProperty(String, Double.)(...)`

Retrieves a double property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/1c05aa56-5f42-be91-a59e-8d59a5549f45)

#### `GetReportProperty(String, Int32.)(...)`

Retrieves an integer property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/ce8e4ca0-3750-3c9b-5ca8-27e48b45e0bd)

#### `GetReportProperty(String, String.)(...)`

Retrieves a string property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/70f212bb-ee70-e9e9-7ab6-d6a9d46f8de6)

#### `GetSolid(...)`

Returns the solid of the pour object.

[Docs](https://developer.tekla.com/topic/en/18/43/e5d83cf6-3c1f-8668-433e-78c42d5a5bfc)

#### `GetStringReportProperties(...)`

Retrieves multiple string report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/5ff72da1-7382-f5d2-eb1d-c99b46239d42)

#### `GetStringUserProperties(...)`

Retrieves all string properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/67120e84-c820-9d52-12ae-c167c595842c)

#### `GetSurfaceObjects(...)`

Returns an enumerator of all the connected surface objects.

[Docs](https://developer.tekla.com/topic/en/18/43/e76afd02-af38-11ed-1048-eeff7503d071)

#### `GetUserProperty(String, Double.)(...)`

Retrieves a double property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/17cc8074-7955-32b5-2b4b-b169c45d4ee0)

#### `GetUserProperty(String, Int32.)(...)`

Retrieves an integer property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/d9364c1d-5b57-6a31-4c13-01124058cfb2)

#### `GetUserProperty(String, String.)(...)`

Retrieves a string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/214a886c-c18c-9b66-d7c9-fe84260962f2)

#### `Insert(...)`

At the moment pour objects cannot be inserted.

[Docs](https://developer.tekla.com/topic/en/18/43/27499f63-911d-1069-2ea2-8a25aa4b8c63)

#### `Modify(...)`

Modifies the pour object with the given identifier.

[Docs](https://developer.tekla.com/topic/en/18/43/c0e528f0-cd8e-ca60-23b8-2a8ca6b990e2)

#### `Select(...)`

Selects the pour object instance from the model. The identifier of the instance must be set.

[Docs](https://developer.tekla.com/topic/en/18/43/e9e760b9-5e6f-8d6c-af0f-0eb678755ef7)

#### `SetCustomObjectType(...)`

Sets a custom object type name for an existing object.

[Docs](https://developer.tekla.com/topic/en/18/43/3d6fb91b-48a9-35ca-d498-526514344089)

#### `SetDynamicStringProperty(...)`

Sets a dynamic string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/4dab0254-09e5-5e61-c28b-1c12afab71a0)

#### `SetLabel(...)`

Sets a label for an object when a new instance is created, this method must be called before Insert. The label is used in plug-ins for identifying the changed object in modification.

[Docs](https://developer.tekla.com/topic/en/18/43/3efcfc44-613a-e66b-9ebd-9f86df4f4036)

#### `SetPhase(...)`

Sets the phase of the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/ed80abcc-0ff7-d081-2229-e651f858a727)

#### `SetUserProperties(...)`

Sets multiple properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/8659da37-d571-f03f-c666-5c9fa3b113f5)

#### `SetUserProperty(String, Double)(...)`

Sets a double property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/5c585b8b-5c2e-7b75-2242-758f17429396)

#### `SetUserProperty(String, Int32)(...)`

Sets an integer property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/dd5b828d-3a87-bb58-5ac9-e1c3b4c2fe4b)

#### `SetUserProperty(String, String)(...)`

Sets a string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/80bf0348-b651-3d4a-862b-49c85bc924ab)

### Properties
- `Class` (object, get/set) — Gets or sets the class of the pour object.
- `ConcreteMixture` (object, get/set) — Gets or sets the concrete mixture of the pour.
- `Identifier` (object, get/set) — The identifier of the object.
- `IsUpToDate` (object, get/set) — Gets if the object does not have a modification which is not shared.
- `ModificationTime` (object, get/set) — Gets latest time of the object was modified or created.
- `PourNumber` (object, get/set) — Gets or sets the pour number of the pour.
- `PourType` (object, get/set) — Gets or sets the pour type of the pour.

## PourUnit (class)

The PourUnit class represents a pour unit in the model.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/ce3eec91-2dd7-6b90-ecd5-97f5017102c8)

### Constructors
- `PourUnit(...)` — Initializes a new instance of the PourUnit class

### Methods
#### `CompareTo(...)`

Compares Identifiers of model objects.

[Docs](https://developer.tekla.com/topic/en/18/43/79cca356-0c8a-e8fb-463a-34ad4141bec7)

#### `Delete(...)`

At the moment pour units cannot be deleted.

[Docs](https://developer.tekla.com/topic/en/18/43/a53286b7-89aa-4940-1be5-7bf224571171)

#### `Equals(...)`

Check if Identifiers of model objects are same.

[Docs](https://developer.tekla.com/topic/en/18/43/e4324265-2490-00de-4139-63a8f7acb492)

#### `GetAllReportProperties(...)`

Retrieves all properties for the model object in one hashtable. Type for the returned value must be checked using type casting.

[Docs](https://developer.tekla.com/topic/en/18/43/0708f404-ce77-28e1-4ebe-fa906f68bff8)

#### `GetAllUserProperties(...)`

Retrieves all properties for the model object in one hashtable. Type for the returned value must be checked using type casting.

[Docs](https://developer.tekla.com/topic/en/18/43/63b853a0-0af5-af9c-3ce7-05299664d7f8)

#### `GetChildren(...)`

Returns an enumerator of all the children model objects.

[Docs](https://developer.tekla.com/topic/en/18/43/ce51ec40-310c-6067-a89f-f8d029aab747)

#### `GetCoordinateSystem(...)`

Returns the coordinate system for the given model object.

[Docs](https://developer.tekla.com/topic/en/18/43/8adfe9cd-f6e5-6474-1297-33c6270a7b0a)

#### `GetCustomObjectType(...)`

Gets custom object type name from an existing object.

[Docs](https://developer.tekla.com/topic/en/18/43/d3be2ba3-8533-7165-f2d3-a5aa9df556b8)

#### `GetDoubleReportProperties(...)`

Retrieves multiple double report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/f86f6c96-f738-49ca-16ba-f5634e862ba2)

#### `GetDoubleUserProperties(...)`

Retrieves all double properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/6ead7a63-5f76-0631-d480-d46c27e882aa)

#### `GetDynamicStringProperty(...)`

Gets a dynamic string property from the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/415e86d9-7de6-ea53-ce68-20b3e67b8655)

#### `GetFatherComponent(...)`

Returns the father component of the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/a2c28f3f-52ff-50a8-1b32-10d18db5c31d)

#### `GetHierarchicObjects(...)`

Returns an enumerator of all the connected hierarchic objects.

[Docs](https://developer.tekla.com/topic/en/18/43/cc228ea0-267b-7d90-7441-c9efc1779b08)

#### `GetIntegerReportProperties(...)`

Retrieves multiple integer report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/2623a9c0-2a9c-e233-9daa-59f92ed51445)

#### `GetIntegerUserProperties(...)`

Retrieves all integer properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/1b3c6f06-bc25-a54e-2a88-b60dc4a3c85e)

#### `GetObjects(...)`

Returns all objects related to pour unit, pour object not included.

[Docs](https://developer.tekla.com/topic/en/18/43/08fa2d0b-72ee-b10d-f083-e3ba88408d74)

#### `GetPhase(...)`

Retrieves the phase of the model object (the phase number, the phase name, the phase comment and whether the phase is the current one or not).

[Docs](https://developer.tekla.com/topic/en/18/43/310f8b7a-c120-753a-35c9-f7c73a9806c9)

#### `GetPourObject(...)`

Returns the pour object in pour unit.

[Docs](https://developer.tekla.com/topic/en/18/43/17f70cea-48ae-889e-3955-be5aa6c82a74)

#### `GetReportProperty(String, Double.)(...)`

Retrieves a double property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/1c05aa56-5f42-be91-a59e-8d59a5549f45)

#### `GetReportProperty(String, Int32.)(...)`

Retrieves an integer property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/ce8e4ca0-3750-3c9b-5ca8-27e48b45e0bd)

#### `GetReportProperty(String, String.)(...)`

Retrieves a string property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/70f212bb-ee70-e9e9-7ab6-d6a9d46f8de6)

#### `GetStringReportProperties(...)`

Retrieves multiple string report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/5ff72da1-7382-f5d2-eb1d-c99b46239d42)

#### `GetStringUserProperties(...)`

Retrieves all string properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/67120e84-c820-9d52-12ae-c167c595842c)

#### `GetUserProperty(String, Double.)(...)`

Retrieves a double property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/17cc8074-7955-32b5-2b4b-b169c45d4ee0)

#### `GetUserProperty(String, Int32.)(...)`

Retrieves an integer property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/d9364c1d-5b57-6a31-4c13-01124058cfb2)

#### `GetUserProperty(String, String.)(...)`

Retrieves a string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/214a886c-c18c-9b66-d7c9-fe84260962f2)

#### `Insert(...)`

At the moment pour units cannot be inserted.

[Docs](https://developer.tekla.com/topic/en/18/43/9dba525f-a455-f61a-9257-0cbff60bed39)

#### `Modify(...)`

Modifies the pour unit with the given identifier.

[Docs](https://developer.tekla.com/topic/en/18/43/8c48f740-099e-3a7c-ccd7-06bdf1859448)

#### `Select(...)`

Selects the pour unit instance from the model. The identifier of the instance must be set.

[Docs](https://developer.tekla.com/topic/en/18/43/eeefa6b8-dbd9-ed8a-5ca6-1ea0c2fda367)

#### `SetCustomObjectType(...)`

Sets a custom object type name for an existing object.

[Docs](https://developer.tekla.com/topic/en/18/43/3d6fb91b-48a9-35ca-d498-526514344089)

#### `SetDynamicStringProperty(...)`

Sets a dynamic string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/4dab0254-09e5-5e61-c28b-1c12afab71a0)

#### `SetLabel(...)`

Sets a label for an object when a new instance is created, this method must be called before Insert. The label is used in plug-ins for identifying the changed object in modification.

[Docs](https://developer.tekla.com/topic/en/18/43/3efcfc44-613a-e66b-9ebd-9f86df4f4036)

#### `SetPhase(...)`

Sets the phase of the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/ed80abcc-0ff7-d081-2229-e651f858a727)

#### `SetUserProperties(...)`

Sets multiple properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/8659da37-d571-f03f-c666-5c9fa3b113f5)

#### `SetUserProperty(String, Double)(...)`

Sets a double property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/5c585b8b-5c2e-7b75-2242-758f17429396)

#### `SetUserProperty(String, Int32)(...)`

Sets an integer property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/dd5b828d-3a87-bb58-5ac9-e1c3b4c2fe4b)

#### `SetUserProperty(String, String)(...)`

Sets a string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/80bf0348-b651-3d4a-862b-49c85bc924ab)

### Properties
- `Identifier` (object, get/set) — The identifier of the object.
- `IsUpToDate` (object, get/set) — Gets if the object does not have a modification which is not shared.
- `ModificationTime` (object, get/set) — Gets latest time of the object was modified or created.
- `Name` (object, get/set) — Gets or sets the name of the pour unit.

## Profile (class)

The Profile class defines the form of a cross section.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/ee78af8a-1ea0-65e3-6b75-57945a009dae)

### Constructors
- `Profile(...)` — Instantiates an empty profile.

### Methods
#### `FormatProfileString(...)`

Formats profile string

[Docs](https://developer.tekla.com/topic/en/18/43/ca9d6a66-2d49-6f5e-ef81-d2688f631207)

#### `ParseProfileString(...)`

Parses profile string

[Docs](https://developer.tekla.com/topic/en/18/43/fb92ae16-8054-c8ac-9469-ef01e23ffe0b)

### Properties
- `ProfileString` (object, get/set) — The profile in human-readable form.

## ProjectInfo (class)

The ProjectInfo class provides project information about the currently open Tekla Structures model.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/e7a57d92-16ad-f456-ecda-fc014e592028)

### Methods
#### `GetBasePointByGuid(...)`

Gets a base point by given guid.

[Docs](https://developer.tekla.com/topic/en/18/43/c5e4596a-269a-8f7c-042b-f506eae10f75)

#### `GetBasePointByName(...)`

Gets a base point by given name.

[Docs](https://developer.tekla.com/topic/en/18/43/8a2ec01e-ffdf-f922-a016-de0b9f23416e)

#### `GetBasePoints(...)`

Gets all available base points as a list.

[Docs](https://developer.tekla.com/topic/en/18/43/871a85de-6e86-1a99-cf44-e90705150737)

#### `GetCurrentCoordsysBasePoint(...)`

Gets the current base point, if set.

[Docs](https://developer.tekla.com/topic/en/18/43/13fbb64f-969a-841e-a222-4b3db6e9f71e)

#### `GetDoubleUserProperties(...)`

Retrieves all double properties for the ProjectInfo.

[Docs](https://developer.tekla.com/topic/en/18/43/ebe59860-a0f1-b2c4-e57a-3f6bbce2b456)

#### `GetDynamicStringProperty(...)`

Gets a dynamic string property for the ProjectInfo.

[Docs](https://developer.tekla.com/topic/en/18/43/dbc7cb81-a36f-9692-9645-d9a48de7db31)

#### `GetIntegerUserProperties(...)`

Retrieves all integer properties for the ProjectInfo.

[Docs](https://developer.tekla.com/topic/en/18/43/425be37f-a8f7-d04a-2397-51912ff18b08)

#### `GetProjectBasePoint(...)`

Gets the project base point, if any set.

[Docs](https://developer.tekla.com/topic/en/18/43/9f95fde2-4622-8461-5844-f0995494662f)

#### `GetStringUserProperties(...)`

Retrieves all string properties for the ProjectInfo.

[Docs](https://developer.tekla.com/topic/en/18/43/816242b1-4315-c23e-0790-e4925c6a66f2)

#### `GetUserProperty(String, Double.)(...)`

Retrieves a double property for the ProjectInfo.

[Docs](https://developer.tekla.com/topic/en/18/43/2f18fcc6-1609-a5dc-73d7-361a14b1a5a9)

#### `GetUserProperty(String, Int32.)(...)`

Retrieves an integer property for the ProjectInfo.

[Docs](https://developer.tekla.com/topic/en/18/43/275e26b2-9fc5-8756-7f67-c90664ae5e3b)

#### `GetUserProperty(String, String.)(...)`

Retrieves a string property for the ProjectInfo.

[Docs](https://developer.tekla.com/topic/en/18/43/2d1103e9-6a84-8f5e-9790-24e51a651b4c)

#### `Modify(...)`

Modifies the current project information.

[Docs](https://developer.tekla.com/topic/en/18/43/61a78724-f119-4cf7-b39d-79a9aeabc575)

#### `SetCurrentCoordsysToBasePoint(...)`

Sets the current base point.

[Docs](https://developer.tekla.com/topic/en/18/43/3137b89c-5c12-7f57-0f7d-77e97ac58b7b)

#### `SetDynamicStringProperty(...)`

Sets a dynamic string property for the ProjectInfo.

[Docs](https://developer.tekla.com/topic/en/18/43/72527889-8db9-9984-1ec5-1f3d33a7055c)

#### `SetUserProperty(String, Double)(...)`

Sets a double property for the ProjectInfo.

[Docs](https://developer.tekla.com/topic/en/18/43/4da75f17-48d2-4a94-1242-40e1e6315de7)

#### `SetUserProperty(String, Int32)(...)`

Sets an integer property for the ProjectInfo.

[Docs](https://developer.tekla.com/topic/en/18/43/a7590f0f-f1a3-b931-2a79-359f7933c0ea)

#### `SetUserProperty(String, String)(...)`

Sets a string property for the ProjectInfo.

[Docs](https://developer.tekla.com/topic/en/18/43/c6a33fba-1e5b-23d9-7955-f7ea4248d1f7)

### Properties
- `Address` (object, get/set) — The address information of the ProjectInfo.
- `Builder` (object, get/set) — The builder information of the ProjectInfo.
- `Country` (object, get/set) — The address country information of the ProjectInfo.
- `Description` (object, get/set) — The description of the ProjectInfo.
- `Designer` (object, get/set) — The designer information of the ProjectInfo.
- `EndDate` (object, get/set) — The end date information of the ProjectInfo.
- `GUID` (object, get/set) — The GUID field of the ProjectInfo.
- `Info1` (object, get/set) — The Info 1 field of the ProjectInfo.
- `Info2` (object, get/set) — The Info 2 field of the ProjectInfo.
- `Location` (object, get/set) — The address location information of the ProjectInfo.
- `ModelSharingLocalPath` (object, get/set) — The local folder for model sharing. Is null if not defined for the model.
- `ModelSharingServerPath` (object, get/set) — The server path for model sharing. Is null if not defined for the model.
- `Name` (object, get/set) — The name information of the ProjectInfo.
- `Object` (object, get/set) — The object information of the ProjectInfo.
- `PostalBox` (object, get/set) — The address postal box information of the ProjectInfo.
- `PostalCode` (object, get/set) — The address postal code information of the ProjectInfo.
- `ProjectNumber` (object, get/set) — The project number information of the ProjectInfo.
- `Region` (object, get/set) — The address region information of the ProjectInfo.
- `StartDate` (object, get/set) — The start date information of the ProjectInfo.
- `Town` (object, get/set) — The address town information of the ProjectInfo.

## RadialGrid (class)

The RadialGrid class defines a user defined (possibly magnetic) set; radial grid has grid planes and cylindrical grid planes as children

[Vendor docs](https://developer.tekla.com/topic/en/18/43/3a0e7bec-0713-864c-4214-a7925c5df74e)

### Constructors
- `RadialGrid(...)` — Creates a new radial grid instance. Overrides the default creation parameters by forcing the angular coordinates to be "0.0 6*15.0"

### Methods
#### `CompareTo(...)`

Compares Identifiers of model objects.

[Docs](https://developer.tekla.com/topic/en/18/43/79cca356-0c8a-e8fb-463a-34ad4141bec7)

#### `Delete(...)`

Deletes the grid with the given ID.

[Docs](https://developer.tekla.com/topic/en/18/43/c1feb648-a138-5987-7bce-1375f684653a)

#### `Equals(...)`

Check if Identifiers of model objects are same.

[Docs](https://developer.tekla.com/topic/en/18/43/e4324265-2490-00de-4139-63a8f7acb492)

#### `GetAllReportProperties(...)`

Retrieves all properties for the model object in one hashtable. Type for the returned value must be checked using type casting.

[Docs](https://developer.tekla.com/topic/en/18/43/0708f404-ce77-28e1-4ebe-fa906f68bff8)

#### `GetAllUserProperties(...)`

Retrieves all properties for the model object in one hashtable. Type for the returned value must be checked using type casting.

[Docs](https://developer.tekla.com/topic/en/18/43/63b853a0-0af5-af9c-3ce7-05299664d7f8)

#### `GetChildren(...)`

Returns an enumerator of all the children model objects.

[Docs](https://developer.tekla.com/topic/en/18/43/ce51ec40-310c-6067-a89f-f8d029aab747)

#### `GetCoordinateSystem(...)`

Returns the coordinate system for the given model object.

[Docs](https://developer.tekla.com/topic/en/18/43/8adfe9cd-f6e5-6474-1297-33c6270a7b0a)

#### `GetCustomObjectType(...)`

Gets custom object type name from an existing object.

[Docs](https://developer.tekla.com/topic/en/18/43/d3be2ba3-8533-7165-f2d3-a5aa9df556b8)

#### `GetDoubleReportProperties(...)`

Retrieves multiple double report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/f86f6c96-f738-49ca-16ba-f5634e862ba2)

#### `GetDoubleUserProperties(...)`

Retrieves all double properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/6ead7a63-5f76-0631-d480-d46c27e882aa)

#### `GetDynamicStringProperty(...)`

Gets a dynamic string property from the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/415e86d9-7de6-ea53-ce68-20b3e67b8655)

#### `GetFatherComponent(...)`

Returns the father component of the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/a2c28f3f-52ff-50a8-1b32-10d18db5c31d)

#### `GetHierarchicObjects(...)`

Returns an enumerator of all the connected hierarchic objects.

[Docs](https://developer.tekla.com/topic/en/18/43/cc228ea0-267b-7d90-7441-c9efc1779b08)

#### `GetIntegerReportProperties(...)`

Retrieves multiple integer report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/2623a9c0-2a9c-e233-9daa-59f92ed51445)

#### `GetIntegerUserProperties(...)`

Retrieves all integer properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/1b3c6f06-bc25-a54e-2a88-b60dc4a3c85e)

#### `GetPhase(...)`

Retrieves the phase of the model object (the phase number, the phase name, the phase comment and whether the phase is the current one or not).

[Docs](https://developer.tekla.com/topic/en/18/43/310f8b7a-c120-753a-35c9-f7c73a9806c9)

#### `GetReportProperty(String, Double.)(...)`

Retrieves a double property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/1c05aa56-5f42-be91-a59e-8d59a5549f45)

#### `GetReportProperty(String, Int32.)(...)`

Retrieves an integer property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/ce8e4ca0-3750-3c9b-5ca8-27e48b45e0bd)

#### `GetReportProperty(String, String.)(...)`

Retrieves a string property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/70f212bb-ee70-e9e9-7ab6-d6a9d46f8de6)

#### `GetStringReportProperties(...)`

Retrieves multiple string report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/5ff72da1-7382-f5d2-eb1d-c99b46239d42)

#### `GetStringUserProperties(...)`

Retrieves all string properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/67120e84-c820-9d52-12ae-c167c595842c)

#### `GetUserProperty(String, Double.)(...)`

Retrieves a double property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/17cc8074-7955-32b5-2b4b-b169c45d4ee0)

#### `GetUserProperty(String, Int32.)(...)`

Retrieves an integer property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/d9364c1d-5b57-6a31-4c13-01124058cfb2)

#### `GetUserProperty(String, String.)(...)`

Retrieves a string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/214a886c-c18c-9b66-d7c9-fe84260962f2)

#### `Insert(...)`

Inserts the grid into the model.

[Docs](https://developer.tekla.com/topic/en/18/43/ee9fce8c-a744-1147-a10e-be0e5a353436)

#### `Modify(...)`

Modifies the grid with the given ID.

[Docs](https://developer.tekla.com/topic/en/18/43/1ac56a24-5d36-2715-7dcc-9101041de4e9)

#### `Select(...)`

Selects the grid with the given ID.

[Docs](https://developer.tekla.com/topic/en/18/43/4a5978f8-bc16-99b9-be9a-69f02353d725)

#### `SetCustomObjectType(...)`

Sets a custom object type name for an existing object.

[Docs](https://developer.tekla.com/topic/en/18/43/3d6fb91b-48a9-35ca-d498-526514344089)

#### `SetDynamicStringProperty(...)`

Sets a dynamic string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/4dab0254-09e5-5e61-c28b-1c12afab71a0)

#### `SetLabel(...)`

Sets a label for an object when a new instance is created, this method must be called before Insert. The label is used in plug-ins for identifying the changed object in modification.

[Docs](https://developer.tekla.com/topic/en/18/43/3efcfc44-613a-e66b-9ebd-9f86df4f4036)

#### `SetPhase(...)`

Sets the phase of the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/ed80abcc-0ff7-d081-2229-e651f858a727)

#### `SetUserProperties(...)`

Sets multiple properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/8659da37-d571-f03f-c666-5c9fa3b113f5)

#### `SetUserProperty(String, Double)(...)`

Sets a double property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/5c585b8b-5c2e-7b75-2242-758f17429396)

#### `SetUserProperty(String, Int32)(...)`

Sets an integer property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/dd5b828d-3a87-bb58-5ac9-e1c3b4c2fe4b)

#### `SetUserProperty(String, String)(...)`

Sets a string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/80bf0348-b651-3d4a-862b-49c85bc924ab)

### Properties
- `AngularCoordinates` (object, get/set) — The angular coordinates of the grid lines
- `AngularLabels` (object, get/set) — The labels for the angular grid lines.
- `AngularLinesEndExtension` (object, get/set) — The extension of the angular grid lines at their end point.
- `AngularLinesStartExtension` (object, get/set) — The extension of the angular grid lines at their start point.
- `ArcEndExtension` (object, get/set) — The extension of the grid arcs at their end point.
- `ArcStartExtension` (object, get/set) — The extension of the grid arcs at their start point.
- `Color` (object, get/set) — Gets or sets the color of the grid.
- `CoordinateZ` (object, get/set) — The coordinates for the Z-axis.
- `ExtensionAboveZ` (object, get/set) — The extension above Z-axis.
- `ExtensionBelowZ` (object, get/set) — The extension below the Z-axis.
- `FontColor` (object, get/set) — Gets or sets font color for the grid labels
- `FontSize` (object, get/set) — Gets or sets font size for the grid labels
- `Identifier` (object, get/set) — The identifier of the object.
- `IsMagnetic` (object, get/set) — Gets or sets whether the grid is magnetic or not
- `IsUpToDate` (object, get/set) — Gets if the object does not have a modification which is not shared.
- `LabelZ` (object, get/set) — The labels for the Z-axis.
- `ModificationTime` (object, get/set) — Gets latest time of the object was modified or created.
- `Name` (object, get/set) — Gets or sets the name of the grid.
- `Origin` (object, get/set) — Gets or sets the origin of the grid
- `RadialCoordinates` (object, get/set) — The distances between the radii of the arcs
- `RadialLabels` (object, get/set) — The labels for the grid arcs.

## RebarComplexGeometry (class)

The ReinforcingBar class represents a single physical reinforcing bar. It contains the curves and bendings that define the geometry of the bar.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/4fdf4193-1664-f040-0c9c-789b5e9443cf)

### Properties
- `BendingRadiuses` (object, get/set) — The bending radiuses of the wire.
- `Diameter` (object, get/set) — Gets the diameter of the wire.
- `Legs` (object, get/set) — Gets the list of RebarLegs of the bar.

## RebarCranking (class)

The RebarCranking class represents the cranking of reinforcing bars at a RebarSplitter.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/87bcf96b-78c0-5086-7639-26f9e1fefe60)

### Constructors
- `RebarCranking(...)` — Initializes a new instance of the RebarCranking class.

### Properties
- `CrankedDistance` (object, get/set) — Gets or sets the cranked distance.
- `CrankedLengthType` (object, get/set) — Gets or sets the cranked length type.
- `CrankedOffset` (object, get/set) — Gets or sets the cranked offset.
- `CrankedRatio` (object, get/set) — Gets or sets the cranked ratio.
- `CrankingType` (object, get/set) — Gets or sets the cranking type.
- `CrankRotation` (object, get/set) — Gets or sets the crank rotation.
- `CrankSide` (object, get/set) — Gets or sets the crank side.
- `CrankStraightLength` (object, get/set) — Gets or sets the crank straight length.

## RebarCranking.CrankSideEnum (enum)

The crank side.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/59095750-aed2-11db-e365-1bbd36c7edb6)

### Values
- `CRANK_LEFT` = `0` — Crank left.
- `CRANK_RIGHT` = `1` — Crank right.

## RebarCranking.CrankedLengthTypeEnum (enum)

The cranked length type.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/390ab6ef-4ab1-2619-5da5-a82f4bfba90f)

### Values
- `DIAGONAL_RATIO` = `0` — Diagonal ratio.
- `DIAGONAL_DISTANCE` = `1` — Diagonal distance.
- `HORIZONTAL_RATIO` = `2` — Horizontal ratio.
- `HORIZONTAL_DISTANCE` = `3` — Horizontal distance.

## RebarCranking.CrankingTypeEnum (enum)

The cranking type.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/416eb182-2d11-0104-67c9-68a6a368a264)

### Values
- `CUSTOM_CRANKING` = `0` — Custom cranking.
- `STANDARD_CRANKING` = `1` — Standard cranking.

## RebarCrankingNullable (class)

The RebarCrankingNullable class represents the cranking of reinforcing bars at a RebarEndDetailModifier.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/103b736f-aa8a-4005-0b09-ea6808049d23)

### Constructors
- `RebarCrankingNullable(...)` — Initializes a new instance of the RebarCrankingNullable class.

### Properties
- `CrankedDistance` (object, get/set) — Gets or sets the cranked distance.
- `CrankedLengthType` (object, get/set) — Gets or sets the cranked length type.
- `CrankedOffset` (object, get/set) — Gets or sets the cranked offset.
- `CrankedRatio` (object, get/set) — Gets or sets the cranked ratio.
- `CrankingType` (object, get/set) — Gets or sets the cranking type.
- `CrankRotation` (object, get/set) — Gets or sets the crank rotation.
- `CrankStraightLength` (object, get/set) — Gets or sets the crank straight length.

## RebarCrankingNullable.EndCrankingTypeEnum (enum)

The end cranking type.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/db0dacf9-363d-f0b3-37c1-4a9eb2eaba1c)

### Values
- `NO_CRANKING` = `0` — No cranking.
- `CUSTOM_CRANKING` = `1` — Custom cranking.
- `STANDARD_CRANKING` = `2` — Standard cranking.

## RebarEndDetailModifier (class)

The RebarEndDetailModifier class represents a modifier that can modify the characteristics of RebarSet reinforcing bars that pass through it. It can modify reinforcing bar end properties and will therefore affect the end of the reinforcing bars it is nearest to.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/a9416b22-067a-33ec-7328-88b946f665d2)

### Constructors
- `RebarEndDetailModifier(...)` — Initializes a new instance of the RebarEndDetailModifier class.

### Methods
#### `CompareTo(...)`

Compares Identifiers of model objects.

[Docs](https://developer.tekla.com/topic/en/18/43/79cca356-0c8a-e8fb-463a-34ad4141bec7)

#### `Delete(...)`

Deletes the rebar modifier from the model database.

[Docs](https://developer.tekla.com/topic/en/18/43/251dbf0c-4b13-fcc8-f4da-0a53618c9248)

#### `Equals(...)`

Check if Identifiers of model objects are same.

[Docs](https://developer.tekla.com/topic/en/18/43/e4324265-2490-00de-4139-63a8f7acb492)

#### `GetAffectedBars(...)`

Gets the virtual bars affected by this modifier.

[Docs](https://developer.tekla.com/topic/en/18/43/f24881c3-eb48-a5b0-0b77-023bab1a4efe)

#### `GetAllReportProperties(...)`

Retrieves all properties for the model object in one hashtable. Type for the returned value must be checked using type casting.

[Docs](https://developer.tekla.com/topic/en/18/43/0708f404-ce77-28e1-4ebe-fa906f68bff8)

#### `GetAllUserProperties(...)`

Retrieves all properties for the model object in one hashtable. Type for the returned value must be checked using type casting.

[Docs](https://developer.tekla.com/topic/en/18/43/63b853a0-0af5-af9c-3ce7-05299664d7f8)

#### `GetChildren(...)`

Returns an enumerator of all the children model objects.

[Docs](https://developer.tekla.com/topic/en/18/43/ce51ec40-310c-6067-a89f-f8d029aab747)

#### `GetCoordinateSystem(...)`

Returns the coordinate system for the given model object.

[Docs](https://developer.tekla.com/topic/en/18/43/8adfe9cd-f6e5-6474-1297-33c6270a7b0a)

#### `GetCustomObjectType(...)`

Gets custom object type name from an existing object.

[Docs](https://developer.tekla.com/topic/en/18/43/d3be2ba3-8533-7165-f2d3-a5aa9df556b8)

#### `GetDoubleReportProperties(...)`

Retrieves multiple double report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/f86f6c96-f738-49ca-16ba-f5634e862ba2)

#### `GetDoubleUserProperties(...)`

Retrieves all double properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/6ead7a63-5f76-0631-d480-d46c27e882aa)

#### `GetDynamicStringProperty(...)`

Gets a dynamic string property from the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/415e86d9-7de6-ea53-ce68-20b3e67b8655)

#### `GetFatherComponent(...)`

Returns the father component of the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/a2c28f3f-52ff-50a8-1b32-10d18db5c31d)

#### `GetHierarchicObjects(...)`

Returns an enumerator of all the connected hierarchic objects.

[Docs](https://developer.tekla.com/topic/en/18/43/cc228ea0-267b-7d90-7441-c9efc1779b08)

#### `GetIntegerReportProperties(...)`

Retrieves multiple integer report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/2623a9c0-2a9c-e233-9daa-59f92ed51445)

#### `GetIntegerUserProperties(...)`

Retrieves all integer properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/1b3c6f06-bc25-a54e-2a88-b60dc4a3c85e)

#### `GetPhase(...)`

Retrieves the phase of the model object (the phase number, the phase name, the phase comment and whether the phase is the current one or not).

[Docs](https://developer.tekla.com/topic/en/18/43/310f8b7a-c120-753a-35c9-f7c73a9806c9)

#### `GetReportProperty(String, Double.)(...)`

Retrieves a double property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/1c05aa56-5f42-be91-a59e-8d59a5549f45)

#### `GetReportProperty(String, Int32.)(...)`

Retrieves an integer property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/ce8e4ca0-3750-3c9b-5ca8-27e48b45e0bd)

#### `GetReportProperty(String, String.)(...)`

Retrieves a string property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/70f212bb-ee70-e9e9-7ab6-d6a9d46f8de6)

#### `GetStringReportProperties(...)`

Retrieves multiple string report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/5ff72da1-7382-f5d2-eb1d-c99b46239d42)

#### `GetStringUserProperties(...)`

Retrieves all string properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/67120e84-c820-9d52-12ae-c167c595842c)

#### `GetUserProperty(String, Double.)(...)`

Retrieves a double property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/17cc8074-7955-32b5-2b4b-b169c45d4ee0)

#### `GetUserProperty(String, Int32.)(...)`

Retrieves an integer property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/d9364c1d-5b57-6a31-4c13-01124058cfb2)

#### `GetUserProperty(String, String.)(...)`

Retrieves a string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/214a886c-c18c-9b66-d7c9-fe84260962f2)

#### `Insert(...)`

Inserts the rebar modifier into the model database.

[Docs](https://developer.tekla.com/topic/en/18/43/a9c11ac6-779a-e098-7069-b0757f11600e)

#### `Modify(...)`

Modifies the existing rebar modifier in the model database to match the current one.

[Docs](https://developer.tekla.com/topic/en/18/43/19951732-1fda-3890-9d60-60be06346c83)

#### `Select(...)`

Selects the rebar modifier from the model database. The identifier must be set.

[Docs](https://developer.tekla.com/topic/en/18/43/3798b590-971f-97f2-8884-72b1eefab6c6)

#### `SetCustomObjectType(...)`

Sets a custom object type name for an existing object.

[Docs](https://developer.tekla.com/topic/en/18/43/3d6fb91b-48a9-35ca-d498-526514344089)

#### `SetDynamicStringProperty(...)`

Sets a dynamic string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/4dab0254-09e5-5e61-c28b-1c12afab71a0)

#### `SetLabel(...)`

Sets a label for an object when a new instance is created, this method must be called before Insert. The label is used in plug-ins for identifying the changed object in modification.

[Docs](https://developer.tekla.com/topic/en/18/43/3efcfc44-613a-e66b-9ebd-9f86df4f4036)

#### `SetPhase(...)`

Sets the phase of the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/ed80abcc-0ff7-d081-2229-e651f858a727)

#### `SetUserProperties(...)`

Sets multiple properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/8659da37-d571-f03f-c666-5c9fa3b113f5)

#### `SetUserProperty(String, Double)(...)`

Sets a double property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/5c585b8b-5c2e-7b75-2242-758f17429396)

#### `SetUserProperty(String, Int32)(...)`

Sets an integer property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/dd5b828d-3a87-bb58-5ac9-e1c3b4c2fe4b)

#### `SetUserProperty(String, String)(...)`

Sets a string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/80bf0348-b651-3d4a-862b-49c85bc924ab)

### Properties
- `BarsAffected` (object, get/set) — Gets or sets the reinforcing bars affected.
- `Curve` (object, get/set) — Gets or sets the geometric Contour of the rebar modifier.
- `EndType` (object, get/set) — Gets or sets the end type.
- `Father` (object, get/set) — Gets or sets the RebarSet to which the modifier belongs. Only reinforcing bars generated by this RebarSet can be modified by this rebar modifier.
- `FirstAffectedBar` (object, get/set) — Gets or sets the first affected bar. If set to zero, the first affected bar will be chosen automatically.
- `FollowEdges` (object, get/set) — Gets or sets a flag that indicates whether the modifier should attempt to follow the leg face edges located between its end points.
- `Identifier` (object, get/set) — The identifier of the object.
- `IsUpToDate` (object, get/set) — Gets if the object does not have a modification which is not shared.
- `ModificationTime` (object, get/set) — Gets latest time of the object was modified or created.
- `RebarCranking` (object, get/set) — Gets or sets the rebar cranking.
- `RebarHook` (object, get/set) — Gets or sets the reinforcing bar hook data.
- `RebarLengthAdjustment` (object, get/set) — Gets or sets the reinforcing bar length adjustment data.
- `RebarThreading` (object, get/set) — Gets or sets the reinforcing bar threading data.

## RebarEndDetailModifier.EndTypeEnum (enum)

The end type.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/7c2ac690-db81-fc87-b89b-d393d28171e8)

### Values
- `HOOK` = `0` — Hook.
- `CRANKING` = `1` — Cranking.

## RebarGeometry (class)

The RebarGeometry class represents a single physical reinforcing bar - the building block of a mesh, a rebar group and even a single rebar. It contains the physical reinforcing bar points in 3D space.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/fbcee9dc-d383-f540-0cf3-ec991d935127)

### Properties
- `BendingRadiuses` (object, get/set) — The bending radiuses of the wire.
- `Diameter` (object, get/set) — The diameter of the wire.
- `Shape` (object, get/set) — The physical points of the reinforcing bar.

## RebarGroup (class)

The RebarGroup class represents a group of reinforcing bars.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/d55cb006-1a87-bda0-7cf6-0a879b3c6c6b)

### Constructors
- `RebarGroup(...)` — Initializes a new rebar group instance with empty attributes.

### Methods
#### `CompareTo(...)`

Compares Identifiers of model objects.

[Docs](https://developer.tekla.com/topic/en/18/43/79cca356-0c8a-e8fb-463a-34ad4141bec7)

#### `Delete(...)`

Deletes the rebar group instance with the given identifier from the model database.

[Docs](https://developer.tekla.com/topic/en/18/43/d941ac31-cc45-2f1f-88ee-ad3d563b262c)

#### `Equals(...)`

Check if Identifiers of model objects are same.

[Docs](https://developer.tekla.com/topic/en/18/43/e4324265-2490-00de-4139-63a8f7acb492)

#### `GetAllReportProperties(...)`

Retrieves all properties for the model object in one hashtable. Type for the returned value must be checked using type casting.

[Docs](https://developer.tekla.com/topic/en/18/43/0708f404-ce77-28e1-4ebe-fa906f68bff8)

#### `GetAllUserProperties(...)`

Retrieves all properties for the model object in one hashtable. Type for the returned value must be checked using type casting.

[Docs](https://developer.tekla.com/topic/en/18/43/63b853a0-0af5-af9c-3ce7-05299664d7f8)

#### `GetAssembly(...)`

Returns the assembly that the reinforcement belongs to.

[Docs](https://developer.tekla.com/topic/en/18/43/322f932e-f09b-df0d-c1c8-3de07d79afeb)

#### `GetChildren(...)`

Returns an enumerator of all the children model objects.

[Docs](https://developer.tekla.com/topic/en/18/43/ce51ec40-310c-6067-a89f-f8d029aab747)

#### `GetCoordinateSystem(...)`

Returns the coordinate system for the given model object.

[Docs](https://developer.tekla.com/topic/en/18/43/8adfe9cd-f6e5-6474-1297-33c6270a7b0a)

#### `GetCustomObjectType(...)`

Gets custom object type name from an existing object.

[Docs](https://developer.tekla.com/topic/en/18/43/d3be2ba3-8533-7165-f2d3-a5aa9df556b8)

#### `GetDoubleReportProperties(...)`

Retrieves multiple double report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/f86f6c96-f738-49ca-16ba-f5634e862ba2)

#### `GetDoubleUserProperties(...)`

Retrieves all double properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/6ead7a63-5f76-0631-d480-d46c27e882aa)

#### `GetDynamicStringProperty(...)`

Gets a dynamic string property from the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/415e86d9-7de6-ea53-ce68-20b3e67b8655)

#### `GetFatherComponent(...)`

Returns the father component of the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/a2c28f3f-52ff-50a8-1b32-10d18db5c31d)

#### `GetFatherPour(...)`

Returns the pour that the rebar is associated to.

[Docs](https://developer.tekla.com/topic/en/18/43/0725e80c-89cf-d6d7-ac40-afdf75f6e695)

#### `GetFatherPourUnit(...)`

Returns the pour unit that the rebar is associated to.

[Docs](https://developer.tekla.com/topic/en/18/43/135a51ac-86cb-ef86-cfda-d2bc53a0d8b8)

#### `GetHierarchicObjects(...)`

Returns an enumerator of all the connected hierarchic objects.

[Docs](https://developer.tekla.com/topic/en/18/43/cc228ea0-267b-7d90-7441-c9efc1779b08)

#### `GetIntegerReportProperties(...)`

Retrieves multiple integer report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/2623a9c0-2a9c-e233-9daa-59f92ed51445)

#### `GetIntegerUserProperties(...)`

Retrieves all integer properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/1b3c6f06-bc25-a54e-2a88-b60dc4a3c85e)

#### `GetNumberOfRebars(...)`

Returns the number of rebars in the reinforcing group.

[Docs](https://developer.tekla.com/topic/en/18/43/6b02fd6f-4f84-6550-69c4-bd2f1117b922)

#### `GetPhase(...)`

Retrieves the phase of the model object (the phase number, the phase name, the phase comment and whether the phase is the current one or not).

[Docs](https://developer.tekla.com/topic/en/18/43/310f8b7a-c120-753a-35c9-f7c73a9806c9)

#### `GetRebarComplexGeometries(Boolean, Boolean, Boolean)(...)`

Retrieves a list of physical reinforcing bars (of type RebarComplexGeometry). These objects contain physical curves in the 3D space of each reinforcing bar as shown in model view. RebarComplexGeometry only differs from RebarGeometry by containing a polycurve rather than a polyline.

[Docs](https://developer.tekla.com/topic/en/18/43/c796aafb-3520-fe06-12e3-a07a87c8f2f9)

#### `GetRebarComplexGeometries(Boolean, Boolean, Boolean, Reinforcement.RebarGeometrySimplificationTypeEnum)(...)`

Retrieves a list of physical reinforcing bars (of type RebarComplexGeometry). These objects contain physical curves in the 3D space of each reinforcing bar as shown in model view. RebarComplexGeometry only differs from RebarGeometry by containing a polycurve (of Arc and LineSegment objects) rather than a polyline. The polycurve only contains arcs if the "simplified" parameter is RATIONALIZED or FABRICATION. If "simplified" is NONE, any arcs will be returned as a series of line segments.

[Docs](https://developer.tekla.com/topic/en/18/43/84a3a38b-d7eb-97df-7fba-a34d13de6610)

#### `GetRebarGeometries(Boolean)(...)`

Retrieves a list of physical reinforcing bars (of type RebarGeometry). These objects contain physical points in the 3D space of each reinforcing bar.

[Docs](https://developer.tekla.com/topic/en/18/43/75649a4c-6252-09a1-b509-cd815e718c0f)

#### `GetRebarGeometries(Reinforcement.RebarGeometryOptionEnum)(...)`

Retrieves a list of physical reinforcing bars (of type RebarGeometry). These objects contain physical points in the 3D space of each reinforcing bar.

[Docs](https://developer.tekla.com/topic/en/18/43/bc45309d-fc4b-9bf9-c640-e71bb1ade771)

#### `GetRebarGeometriesWithoutClashes(...)`

Retrieves a list of physical reinforcing bars (of type RebarGeometry). These objects contain physical points in the 3D space of each reinforcing bar as shown in model view. In case rebar polygon clashes with itself, physical points are moved to avoid clashing.

[Docs](https://developer.tekla.com/topic/en/18/43/470acf1f-1beb-f30a-a1c2-2fcab066ab4a)

#### `GetReportProperty(String, Double.)(...)`

Retrieves a double property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/1c05aa56-5f42-be91-a59e-8d59a5549f45)

#### `GetReportProperty(String, Int32.)(...)`

Retrieves an integer property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/ce8e4ca0-3750-3c9b-5ca8-27e48b45e0bd)

#### `GetReportProperty(String, String.)(...)`

Retrieves a string property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/70f212bb-ee70-e9e9-7ab6-d6a9d46f8de6)

#### `GetSingleRebar(...)`

Returns a single rebar inside the rebar group located by the given index. The indexing starts from the start point. The rebar represents a physical reinforcing bar and contains physical points in the 3D space of the bar. The method returns null on error, for example if given an erroneous index. The number of rebars in the group can be requested with GetNumberOfRebars().

[Docs](https://developer.tekla.com/topic/en/18/43/41181569-65f8-f4ba-7722-3cd890c7a3d4)

#### `GetSingleRebarWithoutClash(...)`

Returns a single rebar inside the rebar group located by the given index. The indexing starts from the start point. The rebar represents a physical reinforcing bar and contains physical, non-clashing points in the 3D space of the bar. The method returns null on error, for example if given an erroneous index. The number of rebars in the group can be requested with GetNumberOfRebars().

[Docs](https://developer.tekla.com/topic/en/18/43/8fd2e21a-3d2f-6211-92ba-9d6e652ac100)

#### `GetSolid(...)`

Method for getting the solid information of the reinforcement.

[Docs](https://developer.tekla.com/topic/en/18/43/6717d119-97da-5554-f359-49ee55e65a63)

#### `GetStringReportProperties(...)`

Retrieves multiple string report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/5ff72da1-7382-f5d2-eb1d-c99b46239d42)

#### `GetStringUserProperties(...)`

Retrieves all string properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/67120e84-c820-9d52-12ae-c167c595842c)

#### `GetUserProperty(String, Double.)(...)`

Retrieves a double property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/17cc8074-7955-32b5-2b4b-b169c45d4ee0)

#### `GetUserProperty(String, Int32.)(...)`

Retrieves an integer property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/d9364c1d-5b57-6a31-4c13-01124058cfb2)

#### `GetUserProperty(String, String.)(...)`

Retrieves a string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/214a886c-c18c-9b66-d7c9-fe84260962f2)

#### `Insert(...)`

Inserts the rebar group into the model database. All the attributes must be set.

[Docs](https://developer.tekla.com/topic/en/18/43/7f18fe92-71a4-9768-b613-30b5855dbab2)

#### `IsGeometryValid(...)`

Tells whether the geometry of a reinforcement object is valid or not.

[Docs](https://developer.tekla.com/topic/en/18/43/5699f6a6-4e39-b784-d84f-45eb7b1d0e37)

#### `Modify(...)`

Modifies the existing rebar group in the model database to match the current one.

[Docs](https://developer.tekla.com/topic/en/18/43/17e5a63d-7d42-5776-c57e-2f86bb8c8ef3)

#### `Select(...)`

Selects a rebar group from the model database. The reinforcement identifier must be set.

[Docs](https://developer.tekla.com/topic/en/18/43/f1ede400-2d4e-9aaf-4743-b3b4e914fda6)

#### `SetCustomObjectType(...)`

Sets a custom object type name for an existing object.

[Docs](https://developer.tekla.com/topic/en/18/43/3d6fb91b-48a9-35ca-d498-526514344089)

#### `SetDynamicStringProperty(...)`

Sets a dynamic string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/4dab0254-09e5-5e61-c28b-1c12afab71a0)

#### `SetLabel(...)`

Sets a label for an object when a new instance is created, this method must be called before Insert. The label is used in plug-ins for identifying the changed object in modification.

[Docs](https://developer.tekla.com/topic/en/18/43/3efcfc44-613a-e66b-9ebd-9f86df4f4036)

#### `SetPhase(...)`

Sets the phase of the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/ed80abcc-0ff7-d081-2229-e651f858a727)

#### `SetUserProperties(...)`

Sets multiple properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/8659da37-d571-f03f-c666-5c9fa3b113f5)

#### `SetUserProperty(String, Double)(...)`

Sets a double property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/5c585b8b-5c2e-7b75-2242-758f17429396)

#### `SetUserProperty(String, Int32)(...)`

Sets an integer property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/dd5b828d-3a87-bb58-5ac9-e1c3b4c2fe4b)

#### `SetUserProperty(String, String)(...)`

Sets a string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/80bf0348-b651-3d4a-862b-49c85bc924ab)

### Properties
- `Class` (object, get/set) — Gets or sets the class of the reinforcement. The class is used to group reinforcements.
- `EndFromPlaneOffset` (object, get/set) — The end offset value from the part surface.
- `EndHook` (object, get/set) — The hook at the end of the reinforcing bar.
- `EndPoint` (object, get/set) — The end point of the direction in which the bars are distributed.
- `EndPointOffsetType` (object, get/set) — Gets or sets the type of the end point offset. The options are: OFFSET_TYPE_LEG_LENGTHOFFSET_TYPE_COVER_THICKNESS
- `EndPointOffsetValue` (object, get/set) — Gets or sets the concrete cover thickness or leg length at the second end of the bar.
- `ExcludeType` (object, get/set) — Defines which bars to omit from the group. The options are: EXCLUDE_TYPE_NONEEXCLUDE_TYPE_FIRSTEXCLUDE_TYPE_LASTEXCLUDE_TYPE_BOTH
- `Father` (object, get/set) — Gets or sets the father object of the reinforcement; the model object instance to operate on.
- `FromPlaneOffset` (object, get/set) — The offset value from the part surface applied in both sides.
- `Grade` (object, get/set) — Gets or sets the steel grade of the reinforcing bar. The grade indicates the strength of the steel used in reinforcing bars. It can also indicate other factors, such as the weldability or surface deformations of the bar.
- `Identifier` (object, get/set) — The identifier of the object.
- `InputPointDeformingState` (object, get/set) — Gets or sets the reinforcement input point deforming state.
- `IsUpToDate` (object, get/set) — Gets if the object does not have a modification which is not shared.
- `ModificationTime` (object, get/set) — Gets latest time of the object was modified or created.
- `Name` (object, get/set) — Gets or sets the name of the reinforcement.
- `NumberingSeries` (object, get/set) — Gets or sets the numbering series of the reinforcement.
- `OnPlaneOffsets` (object, get/set) — Gets or sets the double offset value for each leg on the same plane as the bar.
- `Polygons` (object, get/set) — An array list of polygons to define the reinforcing bar group shape. Use 1 polygon when defining non-tapered or spiral rebar groups. Use 2 to 99 polygons when defining tapered rebar groups. Each polygon must have an equal number of points.
- `RadiusValues` (object, get/set) — Gets or sets the radius value(s) of the bends in the bar.
- `Size` (object, get/set) — The size of the reinforcement.
- `Spacings` (object, get/set) — The spacing value(s). If the type of the spacing is SPACING_TYPE_EXACT_NUMBER Spacings has only one value that defines the number of the reinforcing bars.
- `SpacingType` (object, get/set) — The type of spacing. The options are (BaseRebarGroup.RebarGroupSpacingTypeEnum.): SPACING_TYPE_UNDEFINEDSPACING_TYPE_EXACT_SPACINGSSPACING_TYPE_EXACT_NUMBERSPACING_TYPE_TARGET_SPACESPACING_TYPE_EXACT_SPACE_FLEX_AT_STARTSPACING_TYPE_EXACT_SPACE_FLEX_AT_ENDSPACING_TYPE_EXACT_SPACE_FLEX_AT_BOTHSPACING_TYPE_EXACT_SPACE_FLEX_AT_MIDDLE
- `StartFromPlaneOffset` (object, get/set) — The start offset value from the part surface.
- `StartHook` (object, get/set) — The hook at the beginning of the reinforcing bar.
- `StartPoint` (object, get/set) — The start point of the direction in which the bars are distributed.
- `StartPointOffsetType` (object, get/set) — Gets or sets the type of the start point offset is either OFFSET_TYPE_LEG_LENGTH or OFFSET_TYPE_COVER_THICKNESS.
- `StartPointOffsetValue` (object, get/set) — Gets or sets the concrete cover thickness or leg length at the first end of the bar.
- `StirrupType` (object, get/set) — The type of the stirrup. The options are: STIRRUP_TYPE_POLYGONALSTIRRUP_TYPE_SPIRALSTIRRUP_TYPE_TAPERED_CURVED

## RebarGroup.RebarGroupStirrupTypeEnum (enum)

The different stirrup types.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/74460d20-c421-c309-9d21-aa5fc3feb0a0)

### Values
- `STIRRUP_TYPE_POLYGONAL` = `0` — The polygonal type of stirrup.
- `STIRRUP_TYPE_SPIRAL` = `1` — The spiral type of stirrup.
- `STIRRUP_TYPE_TAPERED_CURVED` = `2` — The tapered curved type of stirrup. Three polygons have to be defined.

## RebarGuideline (class)

Defines a rebar guideline that is used in a RebarSet. A RebarSet has a minimum of one rebar guideline, the primary guideline.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/39c4ee42-992a-871b-bed2-b3665b791270)

### Constructors
- `RebarGuideline(...)` — Initializes a new instance of the RebarGuideline class.

### Properties
- `ChamferType` (object, get/set) — Gets or sets the chamfer type of the guideline. To create longitudinal bars on the top/bottom face of a flat polybeam, create a guideline at each corner, and use the ARC_POINT chamfer type at "curved" corners, and NONE otherwise.
- `Curve` (object, get/set) — Gets or sets the geometric Contour of the rebar guideline.
- `FollowEdges` (object, get/set) — Gets or sets a flag that indicates whether the guideline should attempt to follow the leg face edges located between its end points.
- `Id` (object, get/set) — Gets or sets an identifier that may be used to identify instances of the same rebar guideline.
- `Spacing` (object, get/set) — Gets or sets the RebarSpacing along the rebar guideline.

## RebarGuideline.GuidelineChamferTypeEnum (enum)

The guideline chamfer types. For use when creating longitudinal bars on the top/bottom face of a flat polybeam.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/582e089b-4a60-eb91-5b5c-43415b433900)

### Values
- `NONE` = `0` — No chamfer.
- `ARC_POINT` = `1` — Arc point chamfer.

## RebarHookData (class)

The RebarHookData class defines the hook at the end of a reinforcing bar. Hooks are used for anchoring purposes.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/d0a79869-e12f-9686-d440-b2fe47008803)

### Constructors
- `RebarHookData(...)` — Instantiates a rebar hook data instance with empty members.

### Properties
- `Angle` (object, get/set) — The angle of the hook. The value is between -270 to +270 degrees.
- `Length` (object, get/set) — The length of the straight part of the hook.
- `Radius` (object, get/set) — The bending radius of the hook.
- `Shape` (object, get/set) — Defines the shape of the hook. The options are: NO_HOOK. Sets the angle, radius and length to 0.HOOK_90_DEGREES.HOOK_135_DEGREES.HOOK_180_DEGREES.CUSTOM_HOOK. The angle, radius and length values have to be set manually.

## RebarHookData.RebarHookShapeEnum (enum)

The different hook shapes.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/b689fb8e-0a78-1764-54db-a24472dbe63e)

### Values
- `NO_HOOK` = `0` — No hook.
- `HOOK_90_DEGREES` = `1` — The standard 90-degree hook.
- `HOOK_135_DEGREES` = `2` — The standard 135-degree hook.
- `HOOK_180_DEGREES` = `3` — The standard 180-degree hook.
- `CUSTOM_HOOK` = `4` — The custom hook shape to manually define the angle, radius and length of a hook.

## RebarHookDataNullable (class)

The RebarHookDataNullable class defines the hook at the end of a reinforcing bar. Hooks are used for anchoring purposes.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/e74062d0-32e0-6406-59ca-959ba934408d)

### Constructors
- `RebarHookDataNullable(...)` — Initializes a new instance of the RebarHookDataNullable class.

### Properties
- `Angle` (object, get/set) — Gets or sets the angle of the hook. The value is between -270 to +270 degrees.
- `Length` (object, get/set) — Gets or sets the length of the straight part of the hook.
- `Radius` (object, get/set) — Gets or sets the bending radius of the hook.
- `Rotation` (object, get/set) — Gets or sets the rotation of the hook.
- `Shape` (object, get/set) — Gets or sets the shape of the hook. The options are: NO_HOOK. Sets the angle, radius and length to 0.HOOK_90_DEGREES.HOOK_135_DEGREES.HOOK_180_DEGREES.CUSTOM_HOOK. The angle, radius and length values have to be set manually.

## RebarLapping (class)

The RebarLapping class represents the lapping of reinforcing bars at a RebarSplitter.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/148c7454-629c-faca-e0c8-956fb85a4ad2)

### Constructors
- `RebarLapping(...)` — Initializes a new instance of the RebarLapping class.

### Properties
- `LapLength` (object, get/set) — Gets or sets the lap length. Negative values will define a gap rather than a lap.
- `LappingType` (object, get/set) — Gets or sets the lapping type.
- `LapPlacement` (object, get/set) — Gets or sets the placement of the lap relative to the associated RebarLegFace.
- `LapSide` (object, get/set) — Gets or sets the side of the split that the lap is located.

## RebarLapping.LapPlacementEnum (enum)

The placement of the lap relative to the associated RebarLegFace.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/11808979-1497-c22c-6dba-ba217bd0bf64)

### Values
- `ON_LEG_FACE` = `0` — On leg face.
- `PERPENDICULAR_TO_LEG_FACE` = `1` — Perpendicular to leg face.

## RebarLapping.LapSideEnum (enum)

The side of the split that the lap is located.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/36e35f69-31fb-a0d7-8168-eeda986ce68d)

### Values
- `LAP_LEFT` = `0` — Lap left.
- `LAP_RIGHT` = `1` — Lap right.
- `LAP_MIDDLE` = `2` — Lap middle.

## RebarLapping.LappingTypeEnum (enum)

The lapping type.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/1ac21790-4121-b56e-1d33-6be04b81dfe7)

### Values
- `CUSTOM_LAPPING` = `0` — Custom lapping.
- `STANDARD_LAPPING` = `1` — Standard lapping.

## RebarLeg (class)

This represents a leg of a rebar and is used in the RebarComplexGeometry class.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/6d6067fc-12b2-75b5-12b6-55671ccce55f)

### Properties
- `Curve` (object, get/set) — Gets the curve of the leg
- `Origin` (object, get/set) — Gets the origin of the leg

## RebarLeg.OriginEnum (enum)

Enumerator for different leg origins.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/3060efb6-4306-f18f-573a-867ae166bfd2)

### Values
- `NORMAL` = `0` — The leg is from a face
- `HOOK` = `1` — The leg is on a hook.
- `CRANK` = `2` — The leg is on a crank.

## RebarLegFace (class)

Defines a rebar leg face that is used in a RebarSet. Rebar leg faces control how the RebarSet generates the reinforcing bars.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/dbf1e225-bc2c-43a3-2487-427fa0d12224)

### Constructors
- `RebarLegFace(...)` — Initializes a new instance of the RebarLegFace class.
- `RebarLegFace(...)` — Initializes a new instance of the RebarLegFace class using the specified Contour.

### Properties
- `AdditonalOffset` (object, get/set) — Gets or sets the user defined offset of the rebars from the rebar leg face.
- `Contour` (object, get/set) — Gets or sets the outer Contour of the rebar leg face.
- `Id` (object, get/set) — Gets or sets an identifier that may be used to identify instances of the same rebar leg face.
- `LayerOrderNumber` (object, get/set) — Gets or sets the layer order number of the rebar leg face.
- `Reversed` (object, get/set) — Gets or sets a value indicating whether the cover thickness is opposite to the coordinate system Z direction for the rebar leg face.

## RebarLegSurfaceObject (class)

The RebarLegSurfaceObject class represents a polymesh surface for generating rebar legs in a RebarSet.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/091450f3-68ea-a337-38cd-1b32ff9c2a00)

### Constructors
- `RebarLegSurfaceObject(...)` — Initializes a new instance of the RebarLegSurfaceObject class.

### Methods
#### `CompareTo(...)`

Compares Identifiers of model objects.

[Docs](https://developer.tekla.com/topic/en/18/43/79cca356-0c8a-e8fb-463a-34ad4141bec7)

#### `Delete(...)`

Deletes the model object from the model database.

[Docs](https://developer.tekla.com/topic/en/18/43/67b92503-1679-fcb5-036a-503a581495a5)

#### `Equals(...)`

Check if Identifiers of model objects are same.

[Docs](https://developer.tekla.com/topic/en/18/43/e4324265-2490-00de-4139-63a8f7acb492)

#### `GetAllReportProperties(...)`

Retrieves all properties for the model object in one hashtable. Type for the returned value must be checked using type casting.

[Docs](https://developer.tekla.com/topic/en/18/43/0708f404-ce77-28e1-4ebe-fa906f68bff8)

#### `GetAllUserProperties(...)`

Retrieves all properties for the model object in one hashtable. Type for the returned value must be checked using type casting.

[Docs](https://developer.tekla.com/topic/en/18/43/63b853a0-0af5-af9c-3ce7-05299664d7f8)

#### `GetChildren(...)`

Returns an enumerator of all the children model objects.

[Docs](https://developer.tekla.com/topic/en/18/43/ce51ec40-310c-6067-a89f-f8d029aab747)

#### `GetCoordinateSystem(...)`

Returns the coordinate system for the given model object.

[Docs](https://developer.tekla.com/topic/en/18/43/8adfe9cd-f6e5-6474-1297-33c6270a7b0a)

#### `GetCustomObjectType(...)`

Gets custom object type name from an existing object.

[Docs](https://developer.tekla.com/topic/en/18/43/d3be2ba3-8533-7165-f2d3-a5aa9df556b8)

#### `GetDoubleReportProperties(...)`

Retrieves multiple double report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/f86f6c96-f738-49ca-16ba-f5634e862ba2)

#### `GetDoubleUserProperties(...)`

Retrieves all double properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/6ead7a63-5f76-0631-d480-d46c27e882aa)

#### `GetDynamicStringProperty(...)`

Gets a dynamic string property from the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/415e86d9-7de6-ea53-ce68-20b3e67b8655)

#### `GetFatherComponent(...)`

Returns the father component of the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/a2c28f3f-52ff-50a8-1b32-10d18db5c31d)

#### `GetHierarchicObjects(...)`

Returns an enumerator of all the connected hierarchic objects.

[Docs](https://developer.tekla.com/topic/en/18/43/cc228ea0-267b-7d90-7441-c9efc1779b08)

#### `GetIntegerReportProperties(...)`

Retrieves multiple integer report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/2623a9c0-2a9c-e233-9daa-59f92ed51445)

#### `GetIntegerUserProperties(...)`

Retrieves all integer properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/1b3c6f06-bc25-a54e-2a88-b60dc4a3c85e)

#### `GetPhase(...)`

Retrieves the phase of the model object (the phase number, the phase name, the phase comment and whether the phase is the current one or not).

[Docs](https://developer.tekla.com/topic/en/18/43/310f8b7a-c120-753a-35c9-f7c73a9806c9)

#### `GetReportProperty(String, Double.)(...)`

Retrieves a double property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/1c05aa56-5f42-be91-a59e-8d59a5549f45)

#### `GetReportProperty(String, Int32.)(...)`

Retrieves an integer property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/ce8e4ca0-3750-3c9b-5ca8-27e48b45e0bd)

#### `GetReportProperty(String, String.)(...)`

Retrieves a string property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/70f212bb-ee70-e9e9-7ab6-d6a9d46f8de6)

#### `GetStringReportProperties(...)`

Retrieves multiple string report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/5ff72da1-7382-f5d2-eb1d-c99b46239d42)

#### `GetStringUserProperties(...)`

Retrieves all string properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/67120e84-c820-9d52-12ae-c167c595842c)

#### `GetUserProperty(String, Double.)(...)`

Retrieves a double property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/17cc8074-7955-32b5-2b4b-b169c45d4ee0)

#### `GetUserProperty(String, Int32.)(...)`

Retrieves an integer property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/d9364c1d-5b57-6a31-4c13-01124058cfb2)

#### `GetUserProperty(String, String.)(...)`

Retrieves a string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/214a886c-c18c-9b66-d7c9-fe84260962f2)

#### `Insert(...)`

Inserts the leg surface object into the model database.

[Docs](https://developer.tekla.com/topic/en/18/43/afd1ef63-e101-de50-6924-24233faee019)

#### `Modify(...)`

Modifies the existing leg surface object in the model database to match the current one.

[Docs](https://developer.tekla.com/topic/en/18/43/aeaf0c95-87ef-8005-30ea-cdd53e7121c0)

#### `Select(...)`

Selects the leg surface object from the model database. The identifier must be set.

[Docs](https://developer.tekla.com/topic/en/18/43/3506c9e0-8f2b-afea-17e0-9896e389c1f7)

#### `SetCustomObjectType(...)`

Sets a custom object type name for an existing object.

[Docs](https://developer.tekla.com/topic/en/18/43/3d6fb91b-48a9-35ca-d498-526514344089)

#### `SetDynamicStringProperty(...)`

Sets a dynamic string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/4dab0254-09e5-5e61-c28b-1c12afab71a0)

#### `SetLabel(...)`

Sets a label for an object when a new instance is created, this method must be called before Insert. The label is used in plug-ins for identifying the changed object in modification.

[Docs](https://developer.tekla.com/topic/en/18/43/3efcfc44-613a-e66b-9ebd-9f86df4f4036)

#### `SetPhase(...)`

Sets the phase of the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/ed80abcc-0ff7-d081-2229-e651f858a727)

#### `SetUserProperties(...)`

Sets multiple properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/8659da37-d571-f03f-c666-5c9fa3b113f5)

#### `SetUserProperty(String, Double)(...)`

Sets a double property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/5c585b8b-5c2e-7b75-2242-758f17429396)

#### `SetUserProperty(String, Int32)(...)`

Sets an integer property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/dd5b828d-3a87-bb58-5ac9-e1c3b4c2fe4b)

#### `SetUserProperty(String, String)(...)`

Sets a string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/80bf0348-b651-3d4a-862b-49c85bc924ab)

### Properties
- `AdditionalOffset` (object, get/set) — Gets or sets the user defined offset of the rebar legs from the leg surface object.
- `Class` (object, get/set) — Gets or sets the class of the surface object.
- `CreateHoles` (object, get/set) — Gets or sets a value indicating whether CreateHoles parameter of the surface object.
- `Father` (object, get/set) — Gets or sets the father object of the surface object. The father can be set only before the Insert(), otherwise exception is thrown.
- `Identifier` (object, get/set) — The identifier of the object.
- `IsUpToDate` (object, get/set) — Gets if the object does not have a modification which is not shared.
- `LayerNumber` (object, get/set) — Gets or sets the layer number of the leg surface object.
- `ModificationTime` (object, get/set) — Gets latest time of the object was modified or created.
- `Name` (object, get/set) — Gets or sets the name of the surface object.
- `Polymesh` (object, get/set) — Gets or sets the surface geometry brep.
- `RebarSet` (object, get/set) — Gets or sets the RebarSet to which the leg surface object belongs.
- `Type` (object, get/set) — Gets or sets the type of the surface object.

## RebarLengthAdjustmentDataNullable (class)

The RebarLengthAdjustmentDataNullable class defines the length adjustment at the end of a reinforcing bar affected by a RebarEndDetailModifier. If a length adjustment property is null then that property will not be applied to the RebarSet generated reinforcing bars.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/f01e81f5-5d30-9ee4-1d10-be6f2ad7cafa)

### Constructors
- `RebarLengthAdjustmentDataNullable(...)` — Initializes a new instance of the RebarLengthAdjustmentDataNullable class.

### Properties
- `AdjustmentLength` (object, get/set) — Gets or sets the adjustment length.
- `AdjustmentType` (object, get/set) — Gets or sets the adjustment type.

## RebarLengthAdjustmentDataNullable.LengthAdjustmentTypeEnum (enum)

The length adjustment type.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/615c6899-28ab-0be7-6a9c-be73d79184eb)

### Values
- `NO_ADJUSTMENT` = `0` — No adjustment.
- `END_OFFSET` = `1` — End offset.
- `LEG_LENGTH` = `2` — Leg length.

## RebarMesh (class)

The RebarMesh class represents a reinforcement mesh.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/a385b67a-fd0e-e6fb-d1ef-db2d770dfc8a)

### Constructors
- `RebarMesh(...)` — Initializes a new mesh instance with empty attributes.

### Methods
#### `CompareTo(...)`

Compares Identifiers of model objects.

[Docs](https://developer.tekla.com/topic/en/18/43/79cca356-0c8a-e8fb-463a-34ad4141bec7)

#### `Delete(...)`

Deletes the reinforcement mesh instance with the given identifier from the model database.

[Docs](https://developer.tekla.com/topic/en/18/43/42ff3544-4b2a-dd95-1558-e160ebe90ef5)

#### `Equals(...)`

Check if Identifiers of model objects are same.

[Docs](https://developer.tekla.com/topic/en/18/43/e4324265-2490-00de-4139-63a8f7acb492)

#### `GetAllReportProperties(...)`

Retrieves all properties for the model object in one hashtable. Type for the returned value must be checked using type casting.

[Docs](https://developer.tekla.com/topic/en/18/43/0708f404-ce77-28e1-4ebe-fa906f68bff8)

#### `GetAllUserProperties(...)`

Retrieves all properties for the model object in one hashtable. Type for the returned value must be checked using type casting.

[Docs](https://developer.tekla.com/topic/en/18/43/63b853a0-0af5-af9c-3ce7-05299664d7f8)

#### `GetAssembly(...)`

Returns the assembly that the reinforcement belongs to.

[Docs](https://developer.tekla.com/topic/en/18/43/322f932e-f09b-df0d-c1c8-3de07d79afeb)

#### `GetChildren(...)`

Returns an enumerator of all the children model objects.

[Docs](https://developer.tekla.com/topic/en/18/43/ce51ec40-310c-6067-a89f-f8d029aab747)

#### `GetCoordinateSystem(...)`

Returns the coordinate system for the given model object.

[Docs](https://developer.tekla.com/topic/en/18/43/8adfe9cd-f6e5-6474-1297-33c6270a7b0a)

#### `GetCustomObjectType(...)`

Gets custom object type name from an existing object.

[Docs](https://developer.tekla.com/topic/en/18/43/d3be2ba3-8533-7165-f2d3-a5aa9df556b8)

#### `GetDoubleReportProperties(...)`

Retrieves multiple double report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/f86f6c96-f738-49ca-16ba-f5634e862ba2)

#### `GetDoubleUserProperties(...)`

Retrieves all double properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/6ead7a63-5f76-0631-d480-d46c27e882aa)

#### `GetDynamicStringProperty(...)`

Gets a dynamic string property from the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/415e86d9-7de6-ea53-ce68-20b3e67b8655)

#### `GetFatherComponent(...)`

Returns the father component of the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/a2c28f3f-52ff-50a8-1b32-10d18db5c31d)

#### `GetFatherPour(...)`

Returns the pour that the rebar is associated to.

[Docs](https://developer.tekla.com/topic/en/18/43/0725e80c-89cf-d6d7-ac40-afdf75f6e695)

#### `GetFatherPourUnit(...)`

Returns the pour unit that the rebar is associated to.

[Docs](https://developer.tekla.com/topic/en/18/43/135a51ac-86cb-ef86-cfda-d2bc53a0d8b8)

#### `GetHierarchicObjects(...)`

Returns an enumerator of all the connected hierarchic objects.

[Docs](https://developer.tekla.com/topic/en/18/43/cc228ea0-267b-7d90-7441-c9efc1779b08)

#### `GetIntegerReportProperties(...)`

Retrieves multiple integer report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/2623a9c0-2a9c-e233-9daa-59f92ed51445)

#### `GetIntegerUserProperties(...)`

Retrieves all integer properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/1b3c6f06-bc25-a54e-2a88-b60dc4a3c85e)

#### `GetNumberOfRebars(...)`

Returns the number of rebars in the reinforcing group.

[Docs](https://developer.tekla.com/topic/en/18/43/6b02fd6f-4f84-6550-69c4-bd2f1117b922)

#### `GetPhase(...)`

Retrieves the phase of the model object (the phase number, the phase name, the phase comment and whether the phase is the current one or not).

[Docs](https://developer.tekla.com/topic/en/18/43/310f8b7a-c120-753a-35c9-f7c73a9806c9)

#### `GetRebarComplexGeometries(Boolean, Boolean, Boolean)(...)`

Retrieves a list of physical reinforcing bars (of type RebarComplexGeometry). These objects contain physical curves in the 3D space of each reinforcing bar as shown in model view. RebarComplexGeometry only differs from RebarGeometry by containing a polycurve rather than a polyline.

[Docs](https://developer.tekla.com/topic/en/18/43/c796aafb-3520-fe06-12e3-a07a87c8f2f9)

#### `GetRebarComplexGeometries(Boolean, Boolean, Boolean, Reinforcement.RebarGeometrySimplificationTypeEnum)(...)`

Retrieves a list of physical reinforcing bars (of type RebarComplexGeometry). These objects contain physical curves in the 3D space of each reinforcing bar as shown in model view. RebarComplexGeometry only differs from RebarGeometry by containing a polycurve (of Arc and LineSegment objects) rather than a polyline. The polycurve only contains arcs if the "simplified" parameter is RATIONALIZED or FABRICATION. If "simplified" is NONE, any arcs will be returned as a series of line segments.

[Docs](https://developer.tekla.com/topic/en/18/43/84a3a38b-d7eb-97df-7fba-a34d13de6610)

#### `GetRebarGeometries(Boolean)(...)`

Retrieves a list of physical reinforcing bars (of type RebarGeometry). These objects contain physical points in the 3D space of each reinforcing bar.

[Docs](https://developer.tekla.com/topic/en/18/43/75649a4c-6252-09a1-b509-cd815e718c0f)

#### `GetRebarGeometries(Reinforcement.RebarGeometryOptionEnum)(...)`

Retrieves a list of physical reinforcing bars (of type RebarGeometry). These objects contain physical points in the 3D space of each reinforcing bar.

[Docs](https://developer.tekla.com/topic/en/18/43/bc45309d-fc4b-9bf9-c640-e71bb1ade771)

#### `GetRebarGeometriesWithoutClashes(...)`

Retrieves a list of physical reinforcing bars (of type RebarGeometry). These objects contain physical points in the 3D space of each reinforcing bar as shown in model view. In case rebar polygon clashes with itself, physical points are moved to avoid clashing.

[Docs](https://developer.tekla.com/topic/en/18/43/470acf1f-1beb-f30a-a1c2-2fcab066ab4a)

#### `GetReportProperty(String, Double.)(...)`

Retrieves a double property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/1c05aa56-5f42-be91-a59e-8d59a5549f45)

#### `GetReportProperty(String, Int32.)(...)`

Retrieves an integer property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/ce8e4ca0-3750-3c9b-5ca8-27e48b45e0bd)

#### `GetReportProperty(String, String.)(...)`

Retrieves a string property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/70f212bb-ee70-e9e9-7ab6-d6a9d46f8de6)

#### `GetSingleRebar(...)`

Returns a single rebar inside the rebar group located by the given index. The indexing starts from the start point. The rebar represents a physical reinforcing bar and contains physical points in the 3D space of the bar. The method returns null on error, for example if given an erroneous index. The number of rebars in the group can be requested with GetNumberOfRebars().

[Docs](https://developer.tekla.com/topic/en/18/43/41181569-65f8-f4ba-7722-3cd890c7a3d4)

#### `GetSingleRebarWithoutClash(...)`

Returns a single rebar inside the rebar group located by the given index. The indexing starts from the start point. The rebar represents a physical reinforcing bar and contains physical, non-clashing points in the 3D space of the bar. The method returns null on error, for example if given an erroneous index. The number of rebars in the group can be requested with GetNumberOfRebars().

[Docs](https://developer.tekla.com/topic/en/18/43/8fd2e21a-3d2f-6211-92ba-9d6e652ac100)

#### `GetSolid(...)`

Method for getting the solid information of the reinforcement.

[Docs](https://developer.tekla.com/topic/en/18/43/6717d119-97da-5554-f359-49ee55e65a63)

#### `GetStringReportProperties(...)`

Retrieves multiple string report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/5ff72da1-7382-f5d2-eb1d-c99b46239d42)

#### `GetStringUserProperties(...)`

Retrieves all string properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/67120e84-c820-9d52-12ae-c167c595842c)

#### `GetUserProperty(String, Double.)(...)`

Retrieves a double property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/17cc8074-7955-32b5-2b4b-b169c45d4ee0)

#### `GetUserProperty(String, Int32.)(...)`

Retrieves an integer property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/d9364c1d-5b57-6a31-4c13-01124058cfb2)

#### `GetUserProperty(String, String.)(...)`

Retrieves a string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/214a886c-c18c-9b66-d7c9-fe84260962f2)

#### `Insert(...)`

Inserts the reinforcement mesh into the model database. All the attributes must be set.

[Docs](https://developer.tekla.com/topic/en/18/43/59c0d778-ca7e-f658-9e95-bf48316bb3b9)

#### `IsGeometryValid(...)`

Tells whether the geometry of a reinforcement object is valid or not.

[Docs](https://developer.tekla.com/topic/en/18/43/5699f6a6-4e39-b784-d84f-45eb7b1d0e37)

#### `Modify(...)`

Modifies the existing reinforcement mesh in the model database to match the current one.

[Docs](https://developer.tekla.com/topic/en/18/43/77b54aba-0b79-5280-a938-c09818ce8155)

#### `Select(...)`

Selects a reinforcement mesh from the model database. The reinforcement identifier must be set.

[Docs](https://developer.tekla.com/topic/en/18/43/84e24608-76e8-41fd-f6d7-7594ab52c7c0)

#### `SetCustomObjectType(...)`

Sets a custom object type name for an existing object.

[Docs](https://developer.tekla.com/topic/en/18/43/3d6fb91b-48a9-35ca-d498-526514344089)

#### `SetDynamicStringProperty(...)`

Sets a dynamic string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/4dab0254-09e5-5e61-c28b-1c12afab71a0)

#### `SetLabel(...)`

Sets a label for an object when a new instance is created, this method must be called before Insert. The label is used in plug-ins for identifying the changed object in modification.

[Docs](https://developer.tekla.com/topic/en/18/43/3efcfc44-613a-e66b-9ebd-9f86df4f4036)

#### `SetPhase(...)`

Sets the phase of the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/ed80abcc-0ff7-d081-2229-e651f858a727)

#### `SetUserProperties(...)`

Sets multiple properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/8659da37-d571-f03f-c666-5c9fa3b113f5)

#### `SetUserProperty(String, Double)(...)`

Sets a double property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/5c585b8b-5c2e-7b75-2242-758f17429396)

#### `SetUserProperty(String, Int32)(...)`

Sets an integer property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/dd5b828d-3a87-bb58-5ac9-e1c3b4c2fe4b)

#### `SetUserProperty(String, String)(...)`

Sets a string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/80bf0348-b651-3d4a-862b-49c85bc924ab)

### Properties
- `CatalogName` (object, get/set) — The name of the rebar mesh catalog.
- `Class` (object, get/set) — Gets or sets the class of the reinforcement. The class is used to group reinforcements.
- `CrossBarLocation` (object, get/set) — Defines the location of the crossing bars.
- `CrossDistances` (object, get/set) — The crossing spacing double value(s).
- `CrossSize` (object, get/set) — The size of the crossing rebar.
- `CutByFatherPartCuts` (object, get/set) — Defines whether the rebar mesh is cut by father part cuts or not.
- `EndFromPlaneOffset` (object, get/set) — The end offset value from the part surface.
- `EndHook` (object, get/set) — The hook at the end of the reinforcing.
- `EndPoint` (object, get/set) — The end point of the direction in which the longitudinal bars are distributed.
- `EndPointOffsetType` (object, get/set) — Gets or sets the type of the end point offset. The options are: OFFSET_TYPE_LEG_LENGTHOFFSET_TYPE_COVER_THICKNESS
- `EndPointOffsetValue` (object, get/set) — Gets or sets the concrete cover thickness or leg length at the second end of the bar.
- `Father` (object, get/set) — Gets or sets the father object of the reinforcement; the model object instance to operate on.
- `FromPlaneOffset` (object, get/set) — The offset value from the part surface applied in both sides.
- `Grade` (object, get/set) — Gets or sets the steel grade of the reinforcing bar. The grade indicates the strength of the steel used in reinforcing bars. It can also indicate other factors, such as the weldability or surface deformations of the bar.
- `Identifier` (object, get/set) — The identifier of the object.
- `InputPointDeformingState` (object, get/set) — Gets or sets the reinforcement input point deforming state.
- `IsUpToDate` (object, get/set) — Gets if the object does not have a modification which is not shared.
- `LeftOverhangCross` (object, get/set) — The overhang of the crossing part's left side.
- `LeftOverhangLongitudinal` (object, get/set) — The overhang of the longitudinal bar's left side.
- `Length` (object, get/set) — The length of the rectangle rebar mesh.
- `LongitudinalDistances` (object, get/set) — The longitudinal spacing double value(s).
- `LongitudinalSize` (object, get/set) — The size of the longitudinal rebar.
- `LongitudinalSpacingMethod` (object, get/set) — The type of spacing between rebar mesh bars. Used both for longitudinal and crossing bars. The options are (RebarMesh.RebarMeshSpacingMethodEnum.): SPACING_TYPE_SAME_DISTANCESPACING_TYPE_MULTIPLE_VARYING_DISTANCES
- `MeshType` (object, get/set) — The type of the reinforcement mesh. The options are: RECTANGULAR_MESHPOLYGON_MESHBENT_MESH
- `ModificationTime` (object, get/set) — Gets latest time of the object was modified or created.
- `Name` (object, get/set) — Gets or sets the name of the reinforcement.
- `NumberingSeries` (object, get/set) — Gets or sets the numbering series of the reinforcement.
- `OnPlaneOffsets` (object, get/set) — Gets or sets the double offset value for each leg on the same plane as the bar.
- `Polygon` (object, get/set) — The polygon of the mesh.
- `RadiusValues` (object, get/set) — Gets or sets the radius value(s) of the bends in the bar.
- `RightOverhangCross` (object, get/set) — The right overhang of the crossing bar.
- `RightOverhangLongitudinal` (object, get/set) — The right overhang of the longitudinal bar.
- `StartFromPlaneOffset` (object, get/set) — The start offset value from the part surface.
- `StartHook` (object, get/set) — The hook at the beginning of the reinforcing.
- `StartPoint` (object, get/set) — The start point of the direction in which the longitudinal bars are distributed.
- `StartPointOffsetType` (object, get/set) — Gets or sets the type of the start point offset is either OFFSET_TYPE_LEG_LENGTH or OFFSET_TYPE_COVER_THICKNESS.
- `StartPointOffsetValue` (object, get/set) — Gets or sets the concrete cover thickness or leg length at the first end of the bar.
- `Width` (object, get/set) — The width of the rectangle rebar mesh.

## RebarMesh.RebarMeshCrossBarLocationEnum (enum)

The ways to locate the crossing bars.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/4ae1fcc1-01e0-0245-8025-8cd06a8711c8)

### Values
- `LOCATION_TYPE_UNDEFINED` = `-1` — The undefined location type.
- `LOCATION_TYPE_ABOVE` = `0` — The location of the crossing bars is above the longitudinal bars.
- `LOCATION_TYPE_BELOW` = `1` — The location of the crossing bars is below the longitudinal bars.

## RebarMesh.RebarMeshSpacingMethodEnum (enum)

The ways to distribute the reinforcing bars with different spacings.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/202198a8-2a54-900b-26a8-3b25c55db19f)

### Values
- `SPACING_TYPE_UNDEFINED` = `0` — The undefined spacing type.
- `SPACING_TYPE_SAME_DISTANCE` = `1` — Distributes the bars using the same distance for all bars.
- `SPACING_TYPE_MULTIPLE_VARYING_DISTANCES` = `2` — Varying distribution of the bars.

## RebarMesh.RebarMeshTypeEnum (enum)

The different types of reinforcement meshes.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/a9f722d1-5248-ab18-8d02-39e78dee4e05)

### Values
- `UNKNOWN_MESH` = `0` — The type of the mesh is unknown.
- `RECTANGULAR_MESH` = `1` — The shape of the mesh is rectangular.
- `POLYGON_MESH` = `2` — The shape of the mesh is a polygon. Create rectangular meshes also as polygon meshes.
- `BENT_MESH` = `3` — The shape of the mesh is bent.

## RebarProperties (class)

The RebarProperties class defines the default properties of the reinforcing bars in a RebarSet.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/131721c2-c623-c3df-1f06-ff6a0ba3302a)

### Constructors
- `RebarProperties(...)` — Initializes a new instance of the RebarProperties class.

### Properties
- `BendingRadius` (object, get/set) — Gets or sets the bending radius of the reinforcement.
- `Class` (object, get/set) — Gets or sets the class of the reinforcement. The class is used to group reinforcements.
- `Grade` (object, get/set) — Gets or sets the steel grade of the reinforcing bar. The grade indicates the strength of the steel used in reinforcing bars. It can also indicate other factors, such as the weldability or surface deformations of the bar.
- `Name` (object, get/set) — Gets or sets the name of the reinforcement.
- `NumberingSeries` (object, get/set) — Gets or sets the numbering series of the reinforcement.
- `Size` (object, get/set) — Gets or sets the size of the reinforcement.

## RebarPropertiesNullable (class)

The RebarPropertiesNullable class defines the properties of the reinforcing bars affected by a RebarPropertyModifier.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/c532391c-a2bf-3795-52e9-e6bbea31ad1c)

### Constructors
- `RebarPropertiesNullable(...)` — Initializes a new instance of the RebarPropertiesNullable class.

### Properties
- `BendingRadius` (object, get/set) — Gets or sets the bending radius radius of the reinforcement.
- `Class` (object, get/set) — Gets or sets the class of the reinforcement. The class is used to group reinforcements.
- `Grade` (object, get/set) — Gets or sets the steel grade of the reinforcing bar. The grade indicates the strength of the steel used in reinforcing bars. It can also indicate other factors, such as the weldability or surface deformations of the bar.
- `Name` (object, get/set) — Gets or sets the name of the reinforcement.
- `NumberingSeries` (object, get/set) — Gets or sets the numbering series of the reinforcement.
- `Size` (object, get/set) — Gets or sets the size of the reinforcement.

## RebarPropertyModifier (class)

The RebarPropertyModifier class represents a modifier that can modify the characteristics of RebarSet reinforcing bars that pass through it.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/fccc6e86-8989-4082-2190-75f4fd3b4552)

### Constructors
- `RebarPropertyModifier(...)` — Initializes a new instance of the RebarPropertyModifier class.

### Methods
#### `CompareTo(...)`

Compares Identifiers of model objects.

[Docs](https://developer.tekla.com/topic/en/18/43/79cca356-0c8a-e8fb-463a-34ad4141bec7)

#### `Delete(...)`

Deletes the rebar modifier from the model database.

[Docs](https://developer.tekla.com/topic/en/18/43/251dbf0c-4b13-fcc8-f4da-0a53618c9248)

#### `Equals(...)`

Check if Identifiers of model objects are same.

[Docs](https://developer.tekla.com/topic/en/18/43/e4324265-2490-00de-4139-63a8f7acb492)

#### `GetAffectedBars(...)`

Gets the virtual bars affected by this modifier.

[Docs](https://developer.tekla.com/topic/en/18/43/d05c1076-fd7b-c50a-017c-936951773918)

#### `GetAllReportProperties(...)`

Retrieves all properties for the model object in one hashtable. Type for the returned value must be checked using type casting.

[Docs](https://developer.tekla.com/topic/en/18/43/0708f404-ce77-28e1-4ebe-fa906f68bff8)

#### `GetAllUserProperties(...)`

Retrieves all properties for the model object in one hashtable. Type for the returned value must be checked using type casting.

[Docs](https://developer.tekla.com/topic/en/18/43/63b853a0-0af5-af9c-3ce7-05299664d7f8)

#### `GetChildren(...)`

Returns an enumerator of all the children model objects.

[Docs](https://developer.tekla.com/topic/en/18/43/ce51ec40-310c-6067-a89f-f8d029aab747)

#### `GetCoordinateSystem(...)`

Returns the coordinate system for the given model object.

[Docs](https://developer.tekla.com/topic/en/18/43/8adfe9cd-f6e5-6474-1297-33c6270a7b0a)

#### `GetCustomObjectType(...)`

Gets custom object type name from an existing object.

[Docs](https://developer.tekla.com/topic/en/18/43/d3be2ba3-8533-7165-f2d3-a5aa9df556b8)

#### `GetDoubleReportProperties(...)`

Retrieves multiple double report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/f86f6c96-f738-49ca-16ba-f5634e862ba2)

#### `GetDoubleUserProperties(...)`

Retrieves all double properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/6ead7a63-5f76-0631-d480-d46c27e882aa)

#### `GetDynamicStringProperty(...)`

Gets a dynamic string property from the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/415e86d9-7de6-ea53-ce68-20b3e67b8655)

#### `GetFatherComponent(...)`

Returns the father component of the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/a2c28f3f-52ff-50a8-1b32-10d18db5c31d)

#### `GetHierarchicObjects(...)`

Returns an enumerator of all the connected hierarchic objects.

[Docs](https://developer.tekla.com/topic/en/18/43/cc228ea0-267b-7d90-7441-c9efc1779b08)

#### `GetIntegerReportProperties(...)`

Retrieves multiple integer report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/2623a9c0-2a9c-e233-9daa-59f92ed51445)

#### `GetIntegerUserProperties(...)`

Retrieves all integer properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/1b3c6f06-bc25-a54e-2a88-b60dc4a3c85e)

#### `GetPhase(...)`

Retrieves the phase of the model object (the phase number, the phase name, the phase comment and whether the phase is the current one or not).

[Docs](https://developer.tekla.com/topic/en/18/43/310f8b7a-c120-753a-35c9-f7c73a9806c9)

#### `GetReportProperty(String, Double.)(...)`

Retrieves a double property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/1c05aa56-5f42-be91-a59e-8d59a5549f45)

#### `GetReportProperty(String, Int32.)(...)`

Retrieves an integer property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/ce8e4ca0-3750-3c9b-5ca8-27e48b45e0bd)

#### `GetReportProperty(String, String.)(...)`

Retrieves a string property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/70f212bb-ee70-e9e9-7ab6-d6a9d46f8de6)

#### `GetStringReportProperties(...)`

Retrieves multiple string report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/5ff72da1-7382-f5d2-eb1d-c99b46239d42)

#### `GetStringUserProperties(...)`

Retrieves all string properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/67120e84-c820-9d52-12ae-c167c595842c)

#### `GetUserProperty(String, Double.)(...)`

Retrieves a double property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/17cc8074-7955-32b5-2b4b-b169c45d4ee0)

#### `GetUserProperty(String, Int32.)(...)`

Retrieves an integer property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/d9364c1d-5b57-6a31-4c13-01124058cfb2)

#### `GetUserProperty(String, String.)(...)`

Retrieves a string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/214a886c-c18c-9b66-d7c9-fe84260962f2)

#### `Insert(...)`

Inserts the rebar modifier into the model database.

[Docs](https://developer.tekla.com/topic/en/18/43/a9c11ac6-779a-e098-7069-b0757f11600e)

#### `Modify(...)`

Modifies the existing rebar modifier in the model database to match the current one.

[Docs](https://developer.tekla.com/topic/en/18/43/19951732-1fda-3890-9d60-60be06346c83)

#### `Select(...)`

Selects the rebar modifier from the model database. The identifier must be set.

[Docs](https://developer.tekla.com/topic/en/18/43/3798b590-971f-97f2-8884-72b1eefab6c6)

#### `SetCustomObjectType(...)`

Sets a custom object type name for an existing object.

[Docs](https://developer.tekla.com/topic/en/18/43/3d6fb91b-48a9-35ca-d498-526514344089)

#### `SetDynamicStringProperty(...)`

Sets a dynamic string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/4dab0254-09e5-5e61-c28b-1c12afab71a0)

#### `SetLabel(...)`

Sets a label for an object when a new instance is created, this method must be called before Insert. The label is used in plug-ins for identifying the changed object in modification.

[Docs](https://developer.tekla.com/topic/en/18/43/3efcfc44-613a-e66b-9ebd-9f86df4f4036)

#### `SetPhase(...)`

Sets the phase of the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/ed80abcc-0ff7-d081-2229-e651f858a727)

#### `SetUserProperties(...)`

Sets multiple properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/8659da37-d571-f03f-c666-5c9fa3b113f5)

#### `SetUserProperty(String, Double)(...)`

Sets a double property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/5c585b8b-5c2e-7b75-2242-758f17429396)

#### `SetUserProperty(String, Int32)(...)`

Sets an integer property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/dd5b828d-3a87-bb58-5ac9-e1c3b4c2fe4b)

#### `SetUserProperty(String, String)(...)`

Sets a string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/80bf0348-b651-3d4a-862b-49c85bc924ab)

### Properties
- `AffectsWholeBarPlane` (object, get/set) — Gets or sets a value indicating whether the modifier affects all bars in the plane or just those that it touches.
- `BarsAffected` (object, get/set) — Gets or sets the reinforcing bars affected.
- `Curve` (object, get/set) — Gets or sets the geometric Contour of the rebar modifier.
- `Father` (object, get/set) — Gets or sets the RebarSet to which the modifier belongs. Only reinforcing bars generated by this RebarSet can be modified by this rebar modifier.
- `FatherPart` (object, get/set) — Gets or sets the father part for all bars touching the property modifier.
- `FirstAffectedBar` (object, get/set) — Gets or sets the first affected bar. If set to zero, the first affected bar will be chosen automatically.
- `FollowEdges` (object, get/set) — Gets or sets a flag that indicates whether the modifier should attempt to follow the leg face edges located between its end points.
- `GroupingType` (object, get/set) — Gets or sets the grouping type.
- `Identifier` (object, get/set) — The identifier of the object.
- `IsUpToDate` (object, get/set) — Gets if the object does not have a modification which is not shared.
- `ModificationTime` (object, get/set) — Gets latest time of the object was modified or created.
- `RebarProperties` (object, get/set) — Gets or sets the reinforcing bar properties.
- `SpacingOverride` (object, get/set) — Gets or sets the spacing override.

## RebarPropertyModifier.GroupingTypeEnum (enum)

The grouping type.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/c78cea4b-aa31-e9b6-1295-e07dfa3f8679)

### Values
- `AUTOMATIC` = `0` — Automatic grouping.
- `MANUAL` = `1` — Manual grouping.
- `NO_GROUPING` = `2` — No grouping.

## RebarSet (class)

The RebarSet class represents a set of reinforcing bars.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/f17a0c5b-46ec-bb51-0f38-6971aef88a48)

### Constructors
- `RebarSet(...)` — Initializes a new instance of the RebarSet class.

### Methods
#### `CompareTo(...)`

Compares Identifiers of model objects.

[Docs](https://developer.tekla.com/topic/en/18/43/79cca356-0c8a-e8fb-463a-34ad4141bec7)

#### `Delete(...)`

Deletes the rebar set instance with the given identifier from the model database.

[Docs](https://developer.tekla.com/topic/en/18/43/bd5aed03-5f48-e1f6-446e-29fc3fa363fd)

#### `Equals(...)`

Check if Identifiers of model objects are same.

[Docs](https://developer.tekla.com/topic/en/18/43/e4324265-2490-00de-4139-63a8f7acb492)

#### `GetAllReportProperties(...)`

Retrieves all properties for the model object in one hashtable. Type for the returned value must be checked using type casting.

[Docs](https://developer.tekla.com/topic/en/18/43/0708f404-ce77-28e1-4ebe-fa906f68bff8)

#### `GetAllUserProperties(...)`

Retrieves all properties for the model object in one hashtable. Type for the returned value must be checked using type casting.

[Docs](https://developer.tekla.com/topic/en/18/43/63b853a0-0af5-af9c-3ce7-05299664d7f8)

#### `GetAssembly(...)`

Returns the assembly that the rebar set belongs to.

[Docs](https://developer.tekla.com/topic/en/18/43/8335608e-cc4e-6b43-a08d-db9f5f5ab3e0)

#### `GetChildren(...)`

Returns an enumerator of all the children model objects.

[Docs](https://developer.tekla.com/topic/en/18/43/ce51ec40-310c-6067-a89f-f8d029aab747)

#### `GetCoordinateSystem(...)`

Returns the coordinate system for the given model object.

[Docs](https://developer.tekla.com/topic/en/18/43/8adfe9cd-f6e5-6474-1297-33c6270a7b0a)

#### `GetCustomObjectType(...)`

Gets custom object type name from an existing object.

[Docs](https://developer.tekla.com/topic/en/18/43/d3be2ba3-8533-7165-f2d3-a5aa9df556b8)

#### `GetDoubleReportProperties(...)`

Retrieves multiple double report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/f86f6c96-f738-49ca-16ba-f5634e862ba2)

#### `GetDoubleUserProperties(...)`

Retrieves all double properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/6ead7a63-5f76-0631-d480-d46c27e882aa)

#### `GetDynamicStringProperty(...)`

Gets a dynamic string property from the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/415e86d9-7de6-ea53-ce68-20b3e67b8655)

#### `GetFatherComponent(...)`

Returns the father component of the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/a2c28f3f-52ff-50a8-1b32-10d18db5c31d)

#### `GetHierarchicObjects(...)`

Returns an enumerator of all the connected hierarchic objects.

[Docs](https://developer.tekla.com/topic/en/18/43/cc228ea0-267b-7d90-7441-c9efc1779b08)

#### `GetIntegerReportProperties(...)`

Retrieves multiple integer report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/2623a9c0-2a9c-e233-9daa-59f92ed51445)

#### `GetIntegerUserProperties(...)`

Retrieves all integer properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/1b3c6f06-bc25-a54e-2a88-b60dc4a3c85e)

#### `GetPhase(...)`

Retrieves the phase of the model object (the phase number, the phase name, the phase comment and whether the phase is the current one or not).

[Docs](https://developer.tekla.com/topic/en/18/43/310f8b7a-c120-753a-35c9-f7c73a9806c9)

#### `GetRebarLegSurfaces(...)`

Returns an enumerator of all child RebarLegSurfaceObject objects.

[Docs](https://developer.tekla.com/topic/en/18/43/9b325c30-b21a-0a7a-c06f-5969a1edbdc1)

#### `GetRebarModifiers(...)`

Returns an enumerator of all child BaseRebarModifier objects.

[Docs](https://developer.tekla.com/topic/en/18/43/a1895b03-5a3b-5c7e-da20-1c4d4626bdf2)

#### `GetRebarSetAdditions(...)`

Returns an enumerator of all child RebarSetAddition objects.

[Docs](https://developer.tekla.com/topic/en/18/43/697fe2a9-271e-e4dc-2aa4-5b6a50d5a874)

#### `GetReinforcements(...)`

Returns an enumerator of all the connected SingleRebar objects.

[Docs](https://developer.tekla.com/topic/en/18/43/b613ff47-3a57-78b5-4001-35a303dcf564)

#### `GetReportProperty(String, Double.)(...)`

Retrieves a double property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/1c05aa56-5f42-be91-a59e-8d59a5549f45)

#### `GetReportProperty(String, Int32.)(...)`

Retrieves an integer property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/ce8e4ca0-3750-3c9b-5ca8-27e48b45e0bd)

#### `GetReportProperty(String, String.)(...)`

Retrieves a string property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/70f212bb-ee70-e9e9-7ab6-d6a9d46f8de6)

#### `GetStringReportProperties(...)`

Retrieves multiple string report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/5ff72da1-7382-f5d2-eb1d-c99b46239d42)

#### `GetStringUserProperties(...)`

Retrieves all string properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/67120e84-c820-9d52-12ae-c167c595842c)

#### `GetUserProperty(String, Double.)(...)`

Retrieves a double property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/17cc8074-7955-32b5-2b4b-b169c45d4ee0)

#### `GetUserProperty(String, Int32.)(...)`

Retrieves an integer property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/d9364c1d-5b57-6a31-4c13-01124058cfb2)

#### `GetUserProperty(String, String.)(...)`

Retrieves a string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/214a886c-c18c-9b66-d7c9-fe84260962f2)

#### `Insert(...)`

Inserts the rebar set into the model database.

[Docs](https://developer.tekla.com/topic/en/18/43/c38bbf65-e326-cdf7-5cf4-c87f6378cf23)

#### `Modify(...)`

Modifies the rebar set in the model database to match the current one.

[Docs](https://developer.tekla.com/topic/en/18/43/c2df227d-0cb6-dfad-050a-1d1b7f4070a0)

#### `Select(...)`

Selects the rebar set from the model database. The rebar set identifier must be set.

[Docs](https://developer.tekla.com/topic/en/18/43/d484a0e8-5052-eec1-c1f9-ff3df5418664)

#### `SetCustomObjectType(...)`

Sets a custom object type name for an existing object.

[Docs](https://developer.tekla.com/topic/en/18/43/3d6fb91b-48a9-35ca-d498-526514344089)

#### `SetDynamicStringProperty(...)`

Sets a dynamic string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/4dab0254-09e5-5e61-c28b-1c12afab71a0)

#### `SetLabel(...)`

Sets a label for an object when a new instance is created, this method must be called before Insert. The label is used in plug-ins for identifying the changed object in modification.

[Docs](https://developer.tekla.com/topic/en/18/43/3efcfc44-613a-e66b-9ebd-9f86df4f4036)

#### `SetPhase(...)`

Sets the phase of the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/ed80abcc-0ff7-d081-2229-e651f858a727)

#### `SetUserProperties(...)`

Sets multiple properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/8659da37-d571-f03f-c666-5c9fa3b113f5)

#### `SetUserProperty(String, Double)(...)`

Sets a double property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/5c585b8b-5c2e-7b75-2242-758f17429396)

#### `SetUserProperty(String, Int32)(...)`

Sets an integer property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/dd5b828d-3a87-bb58-5ac9-e1c3b4c2fe4b)

#### `SetUserProperty(String, String)(...)`

Sets a string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/80bf0348-b651-3d4a-862b-49c85bc924ab)

### Properties
- `BarOrientation` (object, get/set) — Gets or sets the bar orientation line. If this is non-null, then it is used to orient the bar planes such that the line is parallel to all the planes.
- `CloseShape` (object, get/set) — Gets or sets a value indicating whether disconnected bar legs should be connected where possible to form a closed shape.
- `FatherPart` (object, get/set) — Gets or sets the father part for the rebar set bars. If set, overrides the automatic bar parenting.
- `Guidelines` (object, get/set) — Gets or sets the rebar set's guidelines.
- `Identifier` (object, get/set) — The identifier of the object.
- `IsUpToDate` (object, get/set) — Gets if the object does not have a modification which is not shared.
- `LayerOrderNumber` (object, get/set) — Gets or sets the layer order number. If set to 0 this means use the creation date/time of the rebar set object to determine the layering.
- `LegFaces` (object, get/set) — Gets or sets the rebar set's leg faces.
- `ModificationTime` (object, get/set) — Gets latest time of the object was modified or created.
- `RebarProperties` (object, get/set) — Gets or sets the reinforcing bar properties for the rebar set.

## RebarSetAddition (class)

The RebarSetAddition class can be used to add RebarLegFace objects to an existing RebarSet.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/87273ccd-8487-e959-e60e-db165b5a6b8f)

### Constructors
- `RebarSetAddition(...)` — Initializes a new instance of the RebarSetAddition class with empty attributes.

### Methods
#### `CompareTo(...)`

Compares Identifiers of model objects.

[Docs](https://developer.tekla.com/topic/en/18/43/79cca356-0c8a-e8fb-463a-34ad4141bec7)

#### `Delete(...)`

Deletes the rebar set addition with the given identifier from the model database.

[Docs](https://developer.tekla.com/topic/en/18/43/06658d93-001d-ce60-1cc7-731dad4dc18d)

#### `Equals(...)`

Check if Identifiers of model objects are same.

[Docs](https://developer.tekla.com/topic/en/18/43/e4324265-2490-00de-4139-63a8f7acb492)

#### `GetAllReportProperties(...)`

Retrieves all properties for the model object in one hashtable. Type for the returned value must be checked using type casting.

[Docs](https://developer.tekla.com/topic/en/18/43/0708f404-ce77-28e1-4ebe-fa906f68bff8)

#### `GetAllUserProperties(...)`

Retrieves all properties for the model object in one hashtable. Type for the returned value must be checked using type casting.

[Docs](https://developer.tekla.com/topic/en/18/43/63b853a0-0af5-af9c-3ce7-05299664d7f8)

#### `GetChildren(...)`

Returns an enumerator of all the children model objects.

[Docs](https://developer.tekla.com/topic/en/18/43/ce51ec40-310c-6067-a89f-f8d029aab747)

#### `GetCoordinateSystem(...)`

Returns the coordinate system for the given model object.

[Docs](https://developer.tekla.com/topic/en/18/43/8adfe9cd-f6e5-6474-1297-33c6270a7b0a)

#### `GetCustomObjectType(...)`

Gets custom object type name from an existing object.

[Docs](https://developer.tekla.com/topic/en/18/43/d3be2ba3-8533-7165-f2d3-a5aa9df556b8)

#### `GetDoubleReportProperties(...)`

Retrieves multiple double report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/f86f6c96-f738-49ca-16ba-f5634e862ba2)

#### `GetDoubleUserProperties(...)`

Retrieves all double properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/6ead7a63-5f76-0631-d480-d46c27e882aa)

#### `GetDynamicStringProperty(...)`

Gets a dynamic string property from the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/415e86d9-7de6-ea53-ce68-20b3e67b8655)

#### `GetFatherComponent(...)`

Returns the father component of the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/a2c28f3f-52ff-50a8-1b32-10d18db5c31d)

#### `GetHierarchicObjects(...)`

Returns an enumerator of all the connected hierarchic objects.

[Docs](https://developer.tekla.com/topic/en/18/43/cc228ea0-267b-7d90-7441-c9efc1779b08)

#### `GetIntegerReportProperties(...)`

Retrieves multiple integer report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/2623a9c0-2a9c-e233-9daa-59f92ed51445)

#### `GetIntegerUserProperties(...)`

Retrieves all integer properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/1b3c6f06-bc25-a54e-2a88-b60dc4a3c85e)

#### `GetPhase(...)`

Retrieves the phase of the model object (the phase number, the phase name, the phase comment and whether the phase is the current one or not).

[Docs](https://developer.tekla.com/topic/en/18/43/310f8b7a-c120-753a-35c9-f7c73a9806c9)

#### `GetReportProperty(String, Double.)(...)`

Retrieves a double property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/1c05aa56-5f42-be91-a59e-8d59a5549f45)

#### `GetReportProperty(String, Int32.)(...)`

Retrieves an integer property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/ce8e4ca0-3750-3c9b-5ca8-27e48b45e0bd)

#### `GetReportProperty(String, String.)(...)`

Retrieves a string property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/70f212bb-ee70-e9e9-7ab6-d6a9d46f8de6)

#### `GetStringReportProperties(...)`

Retrieves multiple string report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/5ff72da1-7382-f5d2-eb1d-c99b46239d42)

#### `GetStringUserProperties(...)`

Retrieves all string properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/67120e84-c820-9d52-12ae-c167c595842c)

#### `GetUserProperty(String, Double.)(...)`

Retrieves a double property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/17cc8074-7955-32b5-2b4b-b169c45d4ee0)

#### `GetUserProperty(String, Int32.)(...)`

Retrieves an integer property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/d9364c1d-5b57-6a31-4c13-01124058cfb2)

#### `GetUserProperty(String, String.)(...)`

Retrieves a string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/214a886c-c18c-9b66-d7c9-fe84260962f2)

#### `Insert(...)`

Inserts the rebar set addition into the model database.

[Docs](https://developer.tekla.com/topic/en/18/43/36ac29b6-8e84-68e6-61a0-774884432533)

#### `Modify(...)`

Modifies the rebar set addition in the model database to match the current one.

[Docs](https://developer.tekla.com/topic/en/18/43/e4b76e36-7d9a-f3af-4163-e824389b96d0)

#### `Select(...)`

Selects the rebar set addition from the model database. The rebar set addition identifier must be set.

[Docs](https://developer.tekla.com/topic/en/18/43/df646faf-3390-60aa-de3f-2bb021a9c057)

#### `SetCustomObjectType(...)`

Sets a custom object type name for an existing object.

[Docs](https://developer.tekla.com/topic/en/18/43/3d6fb91b-48a9-35ca-d498-526514344089)

#### `SetDynamicStringProperty(...)`

Sets a dynamic string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/4dab0254-09e5-5e61-c28b-1c12afab71a0)

#### `SetLabel(...)`

Sets a label for an object when a new instance is created, this method must be called before Insert. The label is used in plug-ins for identifying the changed object in modification.

[Docs](https://developer.tekla.com/topic/en/18/43/3efcfc44-613a-e66b-9ebd-9f86df4f4036)

#### `SetPhase(...)`

Sets the phase of the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/ed80abcc-0ff7-d081-2229-e651f858a727)

#### `SetUserProperties(...)`

Sets multiple properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/8659da37-d571-f03f-c666-5c9fa3b113f5)

#### `SetUserProperty(String, Double)(...)`

Sets a double property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/5c585b8b-5c2e-7b75-2242-758f17429396)

#### `SetUserProperty(String, Int32)(...)`

Sets an integer property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/dd5b828d-3a87-bb58-5ac9-e1c3b4c2fe4b)

#### `SetUserProperty(String, String)(...)`

Sets a string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/80bf0348-b651-3d4a-862b-49c85bc924ab)

### Properties
- `Father` (object, get/set) — Gets or sets the RebarSet to which the rebar set addition applies.
- `Identifier` (object, get/set) — The identifier of the object.
- `IsUpToDate` (object, get/set) — Gets if the object does not have a modification which is not shared.
- `LegFaces` (object, get/set) — Gets or sets the rebar set addition's RebarLegFace objects.
- `ModificationTime` (object, get/set) — Gets latest time of the object was modified or created.

## RebarSpacing (class)

The RebarSpacing class defines spacing properties for a RebarGuideline.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/05404a92-6125-9e64-38ac-784900b5a5da)

### Constructors
- `RebarSpacing(...)` — Initializes a new instance of the RebarSpacing class.

### Methods
#### `Clone(...)`

Makes a copy of this object.

[Docs](https://developer.tekla.com/topic/en/18/43/c9fd9bc4-c7ec-bd90-7e3d-d7ce3564ffee)

#### `Create(RebarSpacing.Offset, RebarSpacing.Offset, IEnumerable.RebarSpacing.ExactSpacing.Element., RebarSpacing.ExactSpacing.Validation.)(...)`

Creates an exact spacing if the given elements are valid.

[Docs](https://developer.tekla.com/topic/en/18/43/5aad86a8-235d-3196-c3c3-a2b4726808d8)

#### `Create(RebarSpacing.Offset, RebarSpacing.Offset, Int32)(...)`

Craetes a spacing defined by number of bars.

[Docs](https://developer.tekla.com/topic/en/18/43/e25e1d85-f2fd-c378-2d88-d0593f44ef8b)

#### `Create(RebarSpacing.SpacingType, RebarSpacing.Offset, RebarSpacing.Offset, Double)(...)`

Create a spacing which is defined by a single distance.

[Docs](https://developer.tekla.com/topic/en/18/43/720eb400-0689-b240-5abe-1f6de1db595d)

#### `IsExactFlexibleType(...)`

Gets whether the spacing is an exact type with flexible space(s).

[Docs](https://developer.tekla.com/topic/en/18/43/53f65e5e-ad47-7e3a-64fe-6f4654c435ee)

### Properties
- `EndOffset` (object, get/set) — Gets or sets the proposed offset to the last reinforcing bar from the end of the RebarGuideline.
- `EndOffsetIsAutomatic` (object, get/set) — Gets or sets a value indicating whether the end offset is automatic. When automatic the proposed end offset is calculated to be the concrete cover thickness plus half the reinforcing bar diameter.
- `EndOffsetType` (object, get/set) — Gets or sets a value indicating whether the end offset is exact or minimum. If the end offset is minimum the spacing algorithm may determine a larger offset.
- `ExactElements` (object, get/set) — Gets or sets the exact elements of the spacing.
- `ExactSpace` (object, get/set) — Gets or sets the exact space.
- `ExcludeType` (object, get/set) — Gets or sets the exclusion of first and last bars.
- `InheritFromPrimary` (object, get/set) — Gets or sets a value indicating whether the secondary guideline inherits all its spacing properties from the primary guideline. This property is not applicable to a primary guideline.
- `InheritOffsetsFromPrimary` (object, get/set) — Gets or sets a value indicating whether the secondary guideline inherits its start and end offsets from the primary guideline. This property is not applicable to a primary guideline.
- `NumberOfBars` (object, get/set) — Gets or sets the number of bars in the spacing object.
- `StartOffset` (object, get/set) — Gets or sets the proposed offset to the first reinforcing bar from the start of the RebarGuideline.
- `StartOffsetIsAutomatic` (object, get/set) — Gets or sets a value indicating whether the start offset is automatic. When automatic the proposed start offset is calculated to be the concrete cover thickness plus half the reinforcing bar diameter.
- `StartOffsetType` (object, get/set) — Gets or sets a value indicating whether the start offset is exact or minimum. If the start offset is minimum the spacing algorithm may determine a larger offset.
- `TargetSpace` (object, get/set) — Gets or sets the target space.
- `Type` (object, get/set) — Gets or sets the type of spacing. Does not allow change to exact spacing or number bars spacing.
- `Zones` (object, get/set) — Gets or sets the spacing zones.

## RebarSpacing.ExactSpacing (class)

This class defines objects used in the exact spacing type.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/8313a0b4-447c-2512-0509-31708a10f051)

### Constructors
- `RebarSpacing.ExactSpacing(...)` — Initializes a new instance of the RebarSpacing.ExactSpacing class

## RebarSpacing.ExactSpacing.Element (struct)

This structure is used to hold number and distance data.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/6d0a7d41-926b-8adb-0ff7-ca117e9be98d)

### Constructors
- `RebarSpacing.ExactSpacing.Element(...)` — Initializes a new instance of the RebarSpacing.ExactSpacing.Element struct.

### Properties
- `Distance` (object, get/set) — Gets or sets the spacing distance in the element.
- `Number` (object, get/set) — Gets or sets the number of spaces in the element.

## RebarSpacing.ExactSpacing.Validation (enum)

Defines values used in validation of exact spacing elements.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/da987189-57da-6c11-3fdb-a97d436b256f)

### Values
- `VALID` = `0` — The elements are valid.
- `EMPTY` = `1` — There are no elements.
- `ZERO_ELEMENTS` = `2` — Contains elements with zero number or distance.
- `NEGATIVE_ELEMENTS` = `3` — Contains elements with negative number or distance.

## RebarSpacing.ExcludeTypeEnum (enum)

The different ways to exclude reinforcing bars from the spacing.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/74ba9b18-0f68-86c7-f4dd-9342a89de5aa)

### Values
- `EXCLUDE_TYPE_NONE` = `1` — All the reinforcing bars are included.
- `EXCLUDE_TYPE_FIRST` = `2` — The first reinforcing bar is not to be created to the spacing.
- `EXCLUDE_TYPE_LAST` = `3` — The last reinforcing bar is not to be created to the spacing.
- `EXCLUDE_TYPE_BOTH` = `4` — The first and last reinforcing bars are not to be created to the spacing.

## RebarSpacing.Offset (struct)

Defines parameters associated with the start and end of the spacing.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/62642faf-3c5e-66fd-2e26-bd3811a02ee9)

### Constructors
- `RebarSpacing.Offset(...)` — Initializes a new instance of the RebarSpacing.Offset struct.

## RebarSpacing.OffsetEnum (enum)

The offset distance type.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/4adf4df1-e121-c217-8ab3-e463d84f7be9)

### Values
- `EXACT` = `0` — The offset distance is exact.
- `MINIMUM` = `1` — The offset distance is the minimum required.

## RebarSpacing.SpacingType (enum)

Defines the different spacing types.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/9caee5b6-a3fd-09dd-da56-10a049e1f45a)

### Values
- `UNDEFINED` = `0` — Undefined spacing.
- `EXACT_SPACINGS` = `1` — Explicitly defined exact spacings.
- `NUMBER_BARS` = `2` — Spacing by number of bars.
- `TARGET` = `3` — Spacing by target distance.
- `EXACT_FLEXIBLE_FIRST` = `4` — Exact spacing with flexible first space.
- `EXACT_FLEXIBLE_LAST` = `5` — Exact spacing with flexible last space.
- `EXACT_FLEXIBLE_FIRST_AND_LAST` = `6` — Exact spacing with flexible first and last space.
- `EXACT_FLEXIBLE_MIDDLE` = `7` — Exact spacing with flexible middle space.

## RebarSpacingZone (class)

The RebarSpacingZone class defines the properties of a single spacing zone. The spacing properties of number of spaces, spacing and length are proposed properties that the spacing algorithm uses as input when calculating the final reinforcing bars spacing. Depending on the RebarSpacingZone properties and any other rebar spacing zones defined on the associated RebarGuideline the spacing algorithm may determine a different final reinforcing bars spacing from the original proposed properties. However the RebarSpacingZone properties will always contain the original proposed values.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/cf189735-28cc-b497-1dac-feb92431104f)

### Constructors
- `RebarSpacingZone(...)` — Initializes a new instance of the RebarSpacingZone class.

### Properties
- `Length` (object, get/set) — Gets or sets the proposed overall spacing zone length.
- `LengthType` (object, get/set) — Gets or sets a value indicating whether the length is absolute or relative. If the length is relative the length property is treated as a proportion of the overall available spacing length (from the associated RebarGuideline). The available spacing length is the guideline length less the calculated start and end offsets and less any fixed length spacing zones. An example of how relative length values are handled: With two relative length zones with proposed length values of 10 and 20 respectively the final calculated lengths will normally be 'available spacing length'*10/(10+20) and 'available spacing length'*20/(10+20) though the spacing algorithm may determine a different result in some situations.
- `NumberOfSpaces` (object, get/set) — Gets or sets the proposed number of spaces in the spacing zone.
- `NumberOfSpacesType` (object, get/set) — Gets or sets a value indicating whether the number of spaces is exact or target. If the number of spaces is target the spacing algorithm may determine a different number of spaces to use.
- `Spacing` (object, get/set) — Gets or sets the proposed spacing (the space between each reinforcing bar centerline) in the spacing zone.
- `SpacingType` (object, get/set) — Gets or sets a value indicating whether the spacing is exact or target. If the spacing is target the spacing algorithm may determine a different spacing to use.

## RebarSpacingZone.LengthEnum (enum)

The length type.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/223a779e-0ae1-dcad-ff8c-52bcd3fe46f3)

### Values
- `ABSOLUTE` = `0` — The length is an absolute value.
- `RELATIVE` = `1` — The length is a relative value.

## RebarSpacingZone.SpacingEnum (enum)

The spacing type.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/bb47f856-0f41-2b58-5a0c-0d59c8c0c37a)

### Values
- `EXACT` = `0` — The spacing is exact.
- `TARGET` = `1` — The spacing is a target value.

## RebarSplice (class)

The RebarSplice class represents a splice between two reinforcements.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/7989d6f9-a7df-43e3-a504-cd4c0b361b02)

### Constructors
- `RebarSplice(...)` — Initializes a new rebar splice instance.
- `RebarSplice(...)` — Initializes a new rebar splice instance with rebar groups to be joined.

### Methods
#### `CompareTo(...)`

Compares Identifiers of model objects.

[Docs](https://developer.tekla.com/topic/en/18/43/79cca356-0c8a-e8fb-463a-34ad4141bec7)

#### `Delete(...)`

Deletes the rebar splice instance with the given identifier from the model database.

[Docs](https://developer.tekla.com/topic/en/18/43/b951cba4-152f-e986-abdc-01f160265c7f)

#### `Equals(...)`

Check if Identifiers of model objects are same.

[Docs](https://developer.tekla.com/topic/en/18/43/e4324265-2490-00de-4139-63a8f7acb492)

#### `GetAllReportProperties(...)`

Retrieves all properties for the model object in one hashtable. Type for the returned value must be checked using type casting.

[Docs](https://developer.tekla.com/topic/en/18/43/0708f404-ce77-28e1-4ebe-fa906f68bff8)

#### `GetAllUserProperties(...)`

Retrieves all properties for the model object in one hashtable. Type for the returned value must be checked using type casting.

[Docs](https://developer.tekla.com/topic/en/18/43/63b853a0-0af5-af9c-3ce7-05299664d7f8)

#### `GetChildren(...)`

Returns an enumerator of all the children model objects.

[Docs](https://developer.tekla.com/topic/en/18/43/ce51ec40-310c-6067-a89f-f8d029aab747)

#### `GetCoordinateSystem(...)`

Returns the coordinate system for the given model object.

[Docs](https://developer.tekla.com/topic/en/18/43/8adfe9cd-f6e5-6474-1297-33c6270a7b0a)

#### `GetCustomObjectType(...)`

Gets custom object type name from an existing object.

[Docs](https://developer.tekla.com/topic/en/18/43/d3be2ba3-8533-7165-f2d3-a5aa9df556b8)

#### `GetDoubleReportProperties(...)`

Retrieves multiple double report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/f86f6c96-f738-49ca-16ba-f5634e862ba2)

#### `GetDoubleUserProperties(...)`

Retrieves all double properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/6ead7a63-5f76-0631-d480-d46c27e882aa)

#### `GetDynamicStringProperty(...)`

Gets a dynamic string property from the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/415e86d9-7de6-ea53-ce68-20b3e67b8655)

#### `GetFatherComponent(...)`

Returns the father component of the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/a2c28f3f-52ff-50a8-1b32-10d18db5c31d)

#### `GetHierarchicObjects(...)`

Returns an enumerator of all the connected hierarchic objects.

[Docs](https://developer.tekla.com/topic/en/18/43/cc228ea0-267b-7d90-7441-c9efc1779b08)

#### `GetIntegerReportProperties(...)`

Retrieves multiple integer report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/2623a9c0-2a9c-e233-9daa-59f92ed51445)

#### `GetIntegerUserProperties(...)`

Retrieves all integer properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/1b3c6f06-bc25-a54e-2a88-b60dc4a3c85e)

#### `GetPhase(...)`

Retrieves the phase of the model object (the phase number, the phase name, the phase comment and whether the phase is the current one or not).

[Docs](https://developer.tekla.com/topic/en/18/43/310f8b7a-c120-753a-35c9-f7c73a9806c9)

#### `GetReportProperty(String, Double.)(...)`

Retrieves a double property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/1c05aa56-5f42-be91-a59e-8d59a5549f45)

#### `GetReportProperty(String, Int32.)(...)`

Retrieves an integer property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/ce8e4ca0-3750-3c9b-5ca8-27e48b45e0bd)

#### `GetReportProperty(String, String.)(...)`

Retrieves a string property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/70f212bb-ee70-e9e9-7ab6-d6a9d46f8de6)

#### `GetStringReportProperties(...)`

Retrieves multiple string report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/5ff72da1-7382-f5d2-eb1d-c99b46239d42)

#### `GetStringUserProperties(...)`

Retrieves all string properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/67120e84-c820-9d52-12ae-c167c595842c)

#### `GetUserProperty(String, Double.)(...)`

Retrieves a double property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/17cc8074-7955-32b5-2b4b-b169c45d4ee0)

#### `GetUserProperty(String, Int32.)(...)`

Retrieves an integer property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/d9364c1d-5b57-6a31-4c13-01124058cfb2)

#### `GetUserProperty(String, String.)(...)`

Retrieves a string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/214a886c-c18c-9b66-d7c9-fe84260962f2)

#### `Insert(...)`

Inserts the rebar splice into the model database. All the attributes must be set.

[Docs](https://developer.tekla.com/topic/en/18/43/e6f5b1d8-5349-b255-5ad7-6680c1c01e3e)

#### `Modify(...)`

Modifies the existing rebar splice in the model database to match the current one.

[Docs](https://developer.tekla.com/topic/en/18/43/7509b971-eb94-4674-7def-489f7da5577f)

#### `Select(...)`

Selects a rebar splice from the model database. The splice identifier must be set.

[Docs](https://developer.tekla.com/topic/en/18/43/bd0b5d77-0ca7-d9a5-37b9-29c8cbddee3e)

#### `SetCustomObjectType(...)`

Sets a custom object type name for an existing object.

[Docs](https://developer.tekla.com/topic/en/18/43/3d6fb91b-48a9-35ca-d498-526514344089)

#### `SetDynamicStringProperty(...)`

Sets a dynamic string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/4dab0254-09e5-5e61-c28b-1c12afab71a0)

#### `SetLabel(...)`

Sets a label for an object when a new instance is created, this method must be called before Insert. The label is used in plug-ins for identifying the changed object in modification.

[Docs](https://developer.tekla.com/topic/en/18/43/3efcfc44-613a-e66b-9ebd-9f86df4f4036)

#### `SetPhase(...)`

Sets the phase of the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/ed80abcc-0ff7-d081-2229-e651f858a727)

#### `SetUserProperties(...)`

Sets multiple properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/8659da37-d571-f03f-c666-5c9fa3b113f5)

#### `SetUserProperty(String, Double)(...)`

Sets a double property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/5c585b8b-5c2e-7b75-2242-758f17429396)

#### `SetUserProperty(String, Int32)(...)`

Sets an integer property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/dd5b828d-3a87-bb58-5ac9-e1c3b4c2fe4b)

#### `SetUserProperty(String, String)(...)`

Sets a string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/80bf0348-b651-3d4a-862b-49c85bc924ab)

### Properties
- `BarPositions` (object, get/set) — Value 0: Joined bars on top of each other in the case of the lap type splice. Value 1: Joined bars parallel to each other in the case of the lap type splice.
- `Clearance` (object, get/set) — The clearance between joined bars in the case of the lap type splice.
- `Identifier` (object, get/set) — The identifier of the object.
- `IsUpToDate` (object, get/set) — Gets if the object does not have a modification which is not shared.
- `LapLength` (object, get/set) — The lap length of the lap type splice.
- `ModificationTime` (object, get/set) — Gets latest time of the object was modified or created.
- `Offset` (object, get/set) — The distance from the end point of the first bar along the bar length.
- `RebarGroup1` (object, get/set) — The first one of the joined reinforcements (picked first).
- `RebarGroup2` (object, get/set) — The second one of the joined reinforcements (picked second).
- `Type` (object, get/set) — The type of the splice (can be a lap, weld or muff splice).

## RebarSplice.RebarSpliceBarPositionsEnum (enum)

The bar group positions in the splice.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/cdbe3eee-707a-0ae2-9131-b8237bc02d51)

### Values
- `SPLICE_BAR_ON_TOP` = `0` — Joined bars on top of each other.
- `SPLICE_BAR_PARALLEL` = `1` — Joined bars parallel to each other.

## RebarSplice.RebarSpliceTypeEnum (enum)

The splice types.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/b63b4f78-1efe-c97c-e733-e7fee3435d99)

### Values
- `SPLICE_TYPE_LAP_RIGHT` = `0` — The first (left) bar extends over the second (right) bar.
- `SPLICE_TYPE_LAP_LEFT` = `1` — The second (right) bar extends over the first (left) bar.
- `SPLICE_TYPE_LAP_BOTH` = `2` — Both bars extend over each other.
- `SPLICE_TYPE_MUFF` = `3` — The bar ends are joined with a muff.
- `SPLICE_TYPE_WELD` = `4` — The bar ends are joined with a weld.

## RebarSplitter (class)

The RebarSplitter class represents a modifier that can split the RebarSet reinforcing bars that pass through it.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/e862d1ea-64b1-1fe4-26a2-a92aa34bbd83)

### Constructors
- `RebarSplitter(...)` — Initializes a new instance of the RebarSplitter class.

### Methods
#### `CompareTo(...)`

Compares Identifiers of model objects.

[Docs](https://developer.tekla.com/topic/en/18/43/79cca356-0c8a-e8fb-463a-34ad4141bec7)

#### `Delete(...)`

Deletes the rebar modifier from the model database.

[Docs](https://developer.tekla.com/topic/en/18/43/251dbf0c-4b13-fcc8-f4da-0a53618c9248)

#### `Equals(...)`

Check if Identifiers of model objects are same.

[Docs](https://developer.tekla.com/topic/en/18/43/e4324265-2490-00de-4139-63a8f7acb492)

#### `GetAffectedBars(...)`

Gets the virtual bars affected by this splitter.

[Docs](https://developer.tekla.com/topic/en/18/43/db6cf285-a582-9c25-7733-224a2c2b34e9)

#### `GetAllReportProperties(...)`

Retrieves all properties for the model object in one hashtable. Type for the returned value must be checked using type casting.

[Docs](https://developer.tekla.com/topic/en/18/43/0708f404-ce77-28e1-4ebe-fa906f68bff8)

#### `GetAllUserProperties(...)`

Retrieves all properties for the model object in one hashtable. Type for the returned value must be checked using type casting.

[Docs](https://developer.tekla.com/topic/en/18/43/63b853a0-0af5-af9c-3ce7-05299664d7f8)

#### `GetChildren(...)`

Returns an enumerator of all the children model objects.

[Docs](https://developer.tekla.com/topic/en/18/43/ce51ec40-310c-6067-a89f-f8d029aab747)

#### `GetCoordinateSystem(...)`

Returns the coordinate system for the given model object.

[Docs](https://developer.tekla.com/topic/en/18/43/8adfe9cd-f6e5-6474-1297-33c6270a7b0a)

#### `GetCustomObjectType(...)`

Gets custom object type name from an existing object.

[Docs](https://developer.tekla.com/topic/en/18/43/d3be2ba3-8533-7165-f2d3-a5aa9df556b8)

#### `GetDoubleReportProperties(...)`

Retrieves multiple double report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/f86f6c96-f738-49ca-16ba-f5634e862ba2)

#### `GetDoubleUserProperties(...)`

Retrieves all double properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/6ead7a63-5f76-0631-d480-d46c27e882aa)

#### `GetDynamicStringProperty(...)`

Gets a dynamic string property from the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/415e86d9-7de6-ea53-ce68-20b3e67b8655)

#### `GetFatherComponent(...)`

Returns the father component of the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/a2c28f3f-52ff-50a8-1b32-10d18db5c31d)

#### `GetHierarchicObjects(...)`

Returns an enumerator of all the connected hierarchic objects.

[Docs](https://developer.tekla.com/topic/en/18/43/cc228ea0-267b-7d90-7441-c9efc1779b08)

#### `GetIntegerReportProperties(...)`

Retrieves multiple integer report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/2623a9c0-2a9c-e233-9daa-59f92ed51445)

#### `GetIntegerUserProperties(...)`

Retrieves all integer properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/1b3c6f06-bc25-a54e-2a88-b60dc4a3c85e)

#### `GetPhase(...)`

Retrieves the phase of the model object (the phase number, the phase name, the phase comment and whether the phase is the current one or not).

[Docs](https://developer.tekla.com/topic/en/18/43/310f8b7a-c120-753a-35c9-f7c73a9806c9)

#### `GetReportProperty(String, Double.)(...)`

Retrieves a double property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/1c05aa56-5f42-be91-a59e-8d59a5549f45)

#### `GetReportProperty(String, Int32.)(...)`

Retrieves an integer property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/ce8e4ca0-3750-3c9b-5ca8-27e48b45e0bd)

#### `GetReportProperty(String, String.)(...)`

Retrieves a string property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/70f212bb-ee70-e9e9-7ab6-d6a9d46f8de6)

#### `GetStringReportProperties(...)`

Retrieves multiple string report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/5ff72da1-7382-f5d2-eb1d-c99b46239d42)

#### `GetStringUserProperties(...)`

Retrieves all string properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/67120e84-c820-9d52-12ae-c167c595842c)

#### `GetUserProperty(String, Double.)(...)`

Retrieves a double property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/17cc8074-7955-32b5-2b4b-b169c45d4ee0)

#### `GetUserProperty(String, Int32.)(...)`

Retrieves an integer property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/d9364c1d-5b57-6a31-4c13-01124058cfb2)

#### `GetUserProperty(String, String.)(...)`

Retrieves a string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/214a886c-c18c-9b66-d7c9-fe84260962f2)

#### `Insert(...)`

Inserts the rebar modifier into the model database.

[Docs](https://developer.tekla.com/topic/en/18/43/a9c11ac6-779a-e098-7069-b0757f11600e)

#### `Modify(...)`

Modifies the existing rebar modifier in the model database to match the current one.

[Docs](https://developer.tekla.com/topic/en/18/43/19951732-1fda-3890-9d60-60be06346c83)

#### `Select(...)`

Selects the rebar modifier from the model database. The identifier must be set.

[Docs](https://developer.tekla.com/topic/en/18/43/3798b590-971f-97f2-8884-72b1eefab6c6)

#### `SetCustomObjectType(...)`

Sets a custom object type name for an existing object.

[Docs](https://developer.tekla.com/topic/en/18/43/3d6fb91b-48a9-35ca-d498-526514344089)

#### `SetDynamicStringProperty(...)`

Sets a dynamic string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/4dab0254-09e5-5e61-c28b-1c12afab71a0)

#### `SetLabel(...)`

Sets a label for an object when a new instance is created, this method must be called before Insert. The label is used in plug-ins for identifying the changed object in modification.

[Docs](https://developer.tekla.com/topic/en/18/43/3efcfc44-613a-e66b-9ebd-9f86df4f4036)

#### `SetPhase(...)`

Sets the phase of the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/ed80abcc-0ff7-d081-2229-e651f858a727)

#### `SetUserProperties(...)`

Sets multiple properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/8659da37-d571-f03f-c666-5c9fa3b113f5)

#### `SetUserProperty(String, Double)(...)`

Sets a double property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/5c585b8b-5c2e-7b75-2242-758f17429396)

#### `SetUserProperty(String, Int32)(...)`

Sets an integer property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/dd5b828d-3a87-bb58-5ac9-e1c3b4c2fe4b)

#### `SetUserProperty(String, String)(...)`

Sets a string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/80bf0348-b651-3d4a-862b-49c85bc924ab)

### Properties
- `BarsAffected` (object, get/set) — Gets or sets the reinforcing bars affected.
- `Cranking` (object, get/set) — Gets or sets the rebar cranking.
- `Curve` (object, get/set) — Gets or sets the geometric Contour of the rebar modifier.
- `Father` (object, get/set) — Gets or sets the RebarSet to which the modifier belongs. Only reinforcing bars generated by this RebarSet can be modified by this rebar modifier.
- `FirstAffectedBar` (object, get/set) — Gets or sets the first affected bar. If set to zero, the first affected bar will be chosen automatically.
- `FollowEdges` (object, get/set) — Gets or sets a flag that indicates whether the modifier should attempt to follow the leg face edges located between its end points.
- `Identifier` (object, get/set) — The identifier of the object.
- `IsUpToDate` (object, get/set) — Gets if the object does not have a modification which is not shared.
- `Lapping` (object, get/set) — Gets or sets the rebar lapping.
- `ModificationTime` (object, get/set) — Gets latest time of the object was modified or created.
- `SplitOffset` (object, get/set) — Gets or sets the offset of the split from the location of the rebar splitter.
- `SplitType` (object, get/set) — Gets or sets the split type.
- `StaggerOffset` (object, get/set) — Gets or sets the additional offset (stagger) of the split location for every other split reinforcing bar.
- `StaggerType` (object, get/set) — Gets or sets the stagger type.

## RebarSplitter.SplitTypeEnum (enum)

The split type.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/27e2d4a2-4414-58ac-6830-edf05f2ea5e7)

### Values
- `LAPPING` = `0` — Lapping.
- `CRANKING` = `1` — Cranking.

## RebarSplitter.StaggerTypeEnum (enum)

The side of the split that the additional offset (stagger) is located.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/2c5b0cea-b4cf-ce97-321f-28514bca0952)

### Values
- `NO_STAGGER` = `0` — No stagger.
- `STAGGER_LEFT` = `1` — Stagger left.
- `STAGGER_RIGHT` = `2` — Stagger right.
- `STAGGER_MIDDLE` = `3` — Stagger middle.

## RebarStrand (class)

The RebarStrand class represents prestressed strands for concrete parts.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/8ce96935-0139-196c-6a95-ffa5c1d4ad89)

### Constructors
- `RebarStrand(...)` — Initializes a new rebar strand instance with empty attributes.

### Methods
#### `CompareTo(...)`

Compares Identifiers of model objects.

[Docs](https://developer.tekla.com/topic/en/18/43/79cca356-0c8a-e8fb-463a-34ad4141bec7)

#### `Delete(...)`

Deletes the strand instance with the given identifier from the model database.

[Docs](https://developer.tekla.com/topic/en/18/43/090475e6-81f5-98c1-ac03-0f596e72f3c0)

#### `Equals(...)`

Check if Identifiers of model objects are same.

[Docs](https://developer.tekla.com/topic/en/18/43/e4324265-2490-00de-4139-63a8f7acb492)

#### `GetAllReportProperties(...)`

Retrieves all properties for the model object in one hashtable. Type for the returned value must be checked using type casting.

[Docs](https://developer.tekla.com/topic/en/18/43/0708f404-ce77-28e1-4ebe-fa906f68bff8)

#### `GetAllUserProperties(...)`

Retrieves all properties for the model object in one hashtable. Type for the returned value must be checked using type casting.

[Docs](https://developer.tekla.com/topic/en/18/43/63b853a0-0af5-af9c-3ce7-05299664d7f8)

#### `GetAssembly(...)`

Returns the assembly that the reinforcement belongs to.

[Docs](https://developer.tekla.com/topic/en/18/43/322f932e-f09b-df0d-c1c8-3de07d79afeb)

#### `GetChildren(...)`

Returns an enumerator of all the children model objects.

[Docs](https://developer.tekla.com/topic/en/18/43/ce51ec40-310c-6067-a89f-f8d029aab747)

#### `GetCoordinateSystem(...)`

Returns the coordinate system for the given model object.

[Docs](https://developer.tekla.com/topic/en/18/43/8adfe9cd-f6e5-6474-1297-33c6270a7b0a)

#### `GetCustomObjectType(...)`

Gets custom object type name from an existing object.

[Docs](https://developer.tekla.com/topic/en/18/43/d3be2ba3-8533-7165-f2d3-a5aa9df556b8)

#### `GetDoubleReportProperties(...)`

Retrieves multiple double report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/f86f6c96-f738-49ca-16ba-f5634e862ba2)

#### `GetDoubleUserProperties(...)`

Retrieves all double properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/6ead7a63-5f76-0631-d480-d46c27e882aa)

#### `GetDynamicStringProperty(...)`

Gets a dynamic string property from the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/415e86d9-7de6-ea53-ce68-20b3e67b8655)

#### `GetFatherComponent(...)`

Returns the father component of the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/a2c28f3f-52ff-50a8-1b32-10d18db5c31d)

#### `GetFatherPour(...)`

Returns the pour that the rebar is associated to.

[Docs](https://developer.tekla.com/topic/en/18/43/0725e80c-89cf-d6d7-ac40-afdf75f6e695)

#### `GetFatherPourUnit(...)`

Returns the pour unit that the rebar is associated to.

[Docs](https://developer.tekla.com/topic/en/18/43/135a51ac-86cb-ef86-cfda-d2bc53a0d8b8)

#### `GetHierarchicObjects(...)`

Returns an enumerator of all the connected hierarchic objects.

[Docs](https://developer.tekla.com/topic/en/18/43/cc228ea0-267b-7d90-7441-c9efc1779b08)

#### `GetIntegerReportProperties(...)`

Retrieves multiple integer report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/2623a9c0-2a9c-e233-9daa-59f92ed51445)

#### `GetIntegerUserProperties(...)`

Retrieves all integer properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/1b3c6f06-bc25-a54e-2a88-b60dc4a3c85e)

#### `GetNumberOfRebars(...)`

Returns the number of rebars in the reinforcing group.

[Docs](https://developer.tekla.com/topic/en/18/43/6b02fd6f-4f84-6550-69c4-bd2f1117b922)

#### `GetPhase(...)`

Retrieves the phase of the model object (the phase number, the phase name, the phase comment and whether the phase is the current one or not).

[Docs](https://developer.tekla.com/topic/en/18/43/310f8b7a-c120-753a-35c9-f7c73a9806c9)

#### `GetRebarComplexGeometries(Boolean, Boolean, Boolean)(...)`

Retrieves a list of physical reinforcing bars (of type RebarComplexGeometry). These objects contain physical curves in the 3D space of each reinforcing bar as shown in model view. RebarComplexGeometry only differs from RebarGeometry by containing a polycurve rather than a polyline.

[Docs](https://developer.tekla.com/topic/en/18/43/c796aafb-3520-fe06-12e3-a07a87c8f2f9)

#### `GetRebarComplexGeometries(Boolean, Boolean, Boolean, Reinforcement.RebarGeometrySimplificationTypeEnum)(...)`

Retrieves a list of physical reinforcing bars (of type RebarComplexGeometry). These objects contain physical curves in the 3D space of each reinforcing bar as shown in model view. RebarComplexGeometry only differs from RebarGeometry by containing a polycurve (of Arc and LineSegment objects) rather than a polyline. The polycurve only contains arcs if the "simplified" parameter is RATIONALIZED or FABRICATION. If "simplified" is NONE, any arcs will be returned as a series of line segments.

[Docs](https://developer.tekla.com/topic/en/18/43/84a3a38b-d7eb-97df-7fba-a34d13de6610)

#### `GetRebarGeometries(Boolean)(...)`

Retrieves a list of physical reinforcing bars (of type RebarGeometry). These objects contain physical points in the 3D space of each reinforcing bar.

[Docs](https://developer.tekla.com/topic/en/18/43/75649a4c-6252-09a1-b509-cd815e718c0f)

#### `GetRebarGeometries(Reinforcement.RebarGeometryOptionEnum)(...)`

Retrieves a list of physical reinforcing bars (of type RebarGeometry). These objects contain physical points in the 3D space of each reinforcing bar.

[Docs](https://developer.tekla.com/topic/en/18/43/bc45309d-fc4b-9bf9-c640-e71bb1ade771)

#### `GetRebarGeometriesWithoutClashes(...)`

Retrieves a list of physical reinforcing bars (of type RebarGeometry). These objects contain physical points in the 3D space of each reinforcing bar as shown in model view. In case rebar polygon clashes with itself, physical points are moved to avoid clashing.

[Docs](https://developer.tekla.com/topic/en/18/43/470acf1f-1beb-f30a-a1c2-2fcab066ab4a)

#### `GetReportProperty(String, Double.)(...)`

Retrieves a double property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/1c05aa56-5f42-be91-a59e-8d59a5549f45)

#### `GetReportProperty(String, Int32.)(...)`

Retrieves an integer property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/ce8e4ca0-3750-3c9b-5ca8-27e48b45e0bd)

#### `GetReportProperty(String, String.)(...)`

Retrieves a string property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/70f212bb-ee70-e9e9-7ab6-d6a9d46f8de6)

#### `GetSingleRebar(...)`

Returns a single rebar inside the rebar group located by the given index. The indexing starts from the start point. The rebar represents a physical reinforcing bar and contains physical points in the 3D space of the bar. The method returns null on error, for example if given an erroneous index. The number of rebars in the group can be requested with GetNumberOfRebars().

[Docs](https://developer.tekla.com/topic/en/18/43/41181569-65f8-f4ba-7722-3cd890c7a3d4)

#### `GetSingleRebarWithoutClash(...)`

Returns a single rebar inside the rebar group located by the given index. The indexing starts from the start point. The rebar represents a physical reinforcing bar and contains physical, non-clashing points in the 3D space of the bar. The method returns null on error, for example if given an erroneous index. The number of rebars in the group can be requested with GetNumberOfRebars().

[Docs](https://developer.tekla.com/topic/en/18/43/8fd2e21a-3d2f-6211-92ba-9d6e652ac100)

#### `GetSolid(...)`

Method for getting the solid information of the reinforcement.

[Docs](https://developer.tekla.com/topic/en/18/43/6717d119-97da-5554-f359-49ee55e65a63)

#### `GetStringReportProperties(...)`

Retrieves multiple string report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/5ff72da1-7382-f5d2-eb1d-c99b46239d42)

#### `GetStringUserProperties(...)`

Retrieves all string properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/67120e84-c820-9d52-12ae-c167c595842c)

#### `GetUserProperty(String, Double.)(...)`

Retrieves a double property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/17cc8074-7955-32b5-2b4b-b169c45d4ee0)

#### `GetUserProperty(String, Int32.)(...)`

Retrieves an integer property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/d9364c1d-5b57-6a31-4c13-01124058cfb2)

#### `GetUserProperty(String, String.)(...)`

Retrieves a string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/214a886c-c18c-9b66-d7c9-fe84260962f2)

#### `Insert(...)`

Inserts the strand into the model database. All the attributes must be set.

[Docs](https://developer.tekla.com/topic/en/18/43/0822342e-298c-7229-075a-0b46a3a7ecd4)

#### `IsGeometryValid(...)`

Tells whether the geometry of a reinforcement object is valid or not.

[Docs](https://developer.tekla.com/topic/en/18/43/5699f6a6-4e39-b784-d84f-45eb7b1d0e37)

#### `Modify(...)`

Modifies the existing strand in the model database to match the current one.

[Docs](https://developer.tekla.com/topic/en/18/43/ea8ee6ce-2093-f5fc-6541-0a42bb13e5ef)

#### `Select(...)`

Selects the strand from the model database. The reinforcement identifier must be set.

[Docs](https://developer.tekla.com/topic/en/18/43/df6b3c81-2dbf-8794-ccba-5166785cfe41)

#### `SetCustomObjectType(...)`

Sets a custom object type name for an existing object.

[Docs](https://developer.tekla.com/topic/en/18/43/3d6fb91b-48a9-35ca-d498-526514344089)

#### `SetDynamicStringProperty(...)`

Sets a dynamic string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/4dab0254-09e5-5e61-c28b-1c12afab71a0)

#### `SetLabel(...)`

Sets a label for an object when a new instance is created, this method must be called before Insert. The label is used in plug-ins for identifying the changed object in modification.

[Docs](https://developer.tekla.com/topic/en/18/43/3efcfc44-613a-e66b-9ebd-9f86df4f4036)

#### `SetPhase(...)`

Sets the phase of the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/ed80abcc-0ff7-d081-2229-e651f858a727)

#### `SetUserProperties(...)`

Sets multiple properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/8659da37-d571-f03f-c666-5c9fa3b113f5)

#### `SetUserProperty(String, Double)(...)`

Sets a double property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/5c585b8b-5c2e-7b75-2242-758f17429396)

#### `SetUserProperty(String, Int32)(...)`

Sets an integer property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/dd5b828d-3a87-bb58-5ac9-e1c3b4c2fe4b)

#### `SetUserProperty(String, String)(...)`

Sets a string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/80bf0348-b651-3d4a-862b-49c85bc924ab)

### Properties
- `Class` (object, get/set) — Gets or sets the class of the reinforcement. The class is used to group reinforcements.
- `EndPoint` (object, get/set) — The end point of the single strand (the start point and end point define the length of the strand).
- `EndPointOffsetType` (object, get/set) — Gets or sets the type of the end point offset. The options are: OFFSET_TYPE_LEG_LENGTHOFFSET_TYPE_COVER_THICKNESS
- `EndPointOffsetValue` (object, get/set) — Gets or sets the concrete cover thickness or leg length at the second end of the bar.
- `Father` (object, get/set) — Gets or sets the father object of the reinforcement; the model object instance to operate on.
- `FromPlaneOffset` (object, get/set) — Gets or sets the offset value from the part surface.
- `Grade` (object, get/set) — Gets or sets the steel grade of the reinforcing bar. The grade indicates the strength of the steel used in reinforcing bars. It can also indicate other factors, such as the weldability or surface deformations of the bar.
- `Identifier` (object, get/set) — The identifier of the object.
- `InputPointDeformingState` (object, get/set) — Gets or sets the reinforcement input point deforming state.
- `IsUpToDate` (object, get/set) — Gets if the object does not have a modification which is not shared.
- `ModificationTime` (object, get/set) — Gets latest time of the object was modified or created.
- `Name` (object, get/set) — Gets or sets the name of the reinforcement.
- `NumberingSeries` (object, get/set) — Gets or sets the numbering series of the reinforcement.
- `OnPlaneOffsets` (object, get/set) — Not supported.
- `Patterns` (object, get/set) — An array list of polygons that represents the patterns of the rebar strand. Give the strand positions in the same order as for the first pattern. The maximum number of patterns is 20.
- `PullPerStrand` (object, get/set) — The pull per strand (N).
- `RadiusValues` (object, get/set) — Gets or sets the radius value(s) of the bends in the bar.
- `Size` (object, get/set) — The size of the reinforcement.
- `StartPoint` (object, get/set) — The start point of the single strand (the start point and end point define the length of the strand).
- `StartPointOffsetType` (object, get/set) — Gets or sets the type of the start point offset is either OFFSET_TYPE_LEG_LENGTH or OFFSET_TYPE_COVER_THICKNESS.
- `StartPointOffsetValue` (object, get/set) — Gets or sets the concrete cover thickness or leg length at the first end of the bar.
- `Unbondings` (object, get/set) — An array list of StrandUnbondingData items.

## RebarThreadingDataNullable (class)

The RebarThreadingDataNullable class defines the threading properties at the end of a reinforcing bar affected by a RebarEndDetailModifier. If a threading property is null then that property will not be applied to the RebarSet generated reinforcing bars.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/57d87a1e-cd45-eccf-35f1-73a6e5fba633)

### Constructors
- `RebarThreadingDataNullable(...)` — Initializes a new instance of the RebarThreadingDataNullable class.

### Properties
- `ExtraFabricationLength` (object, get/set) — Gets or sets the extra fabrication length.
- `Length` (object, get/set) — Gets or sets the threading length.
- `ThreadingType` (object, get/set) — Gets or sets the threading type. Null indicates that the threading type is not specified.

## ReferenceModel (class)

The ReferenceModel class contains methods related to reference models. A reference model is a reference of an external model.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/a1ca3d98-d440-e04c-ed85-901efed0470c)

### Constructors
- `ReferenceModel(...)` — Initializes a new instance of the ReferenceModel class. The default values are as follows: Filename = string.Empty; ActiveFilePath = string.Empty; ProjectGUID = Guid.Empty; ModelGUID = Guid.Empty; VersionGUID = Guid.Empty; Position = new Point(); Scale = 1.0; Visibility = VisibilityEnum.VISIBLE; BasePointGuid = string.Empty;
- `ReferenceModel(...)` — Initializes a new instance of the ReferenceModel class. Creates a new reference model instance using the given filename, position and scale.

### Methods
#### `CompareTo(...)`

Compares Identifiers of model objects.

[Docs](https://developer.tekla.com/topic/en/18/43/79cca356-0c8a-e8fb-463a-34ad4141bec7)

#### `Delete(...)`

Deletes the reference model with the given identifier.

[Docs](https://developer.tekla.com/topic/en/18/43/532a3f22-e1de-20e4-93f1-034f6702119c)

#### `Equals(...)`

Check if Identifiers of model objects are same.

[Docs](https://developer.tekla.com/topic/en/18/43/e4324265-2490-00de-4139-63a8f7acb492)

#### `GetAllReportProperties(...)`

Retrieves all properties for the model object in one hashtable. Type for the returned value must be checked using type casting.

[Docs](https://developer.tekla.com/topic/en/18/43/0708f404-ce77-28e1-4ebe-fa906f68bff8)

#### `GetAllUserProperties(...)`

Retrieves all properties for the model object in one hashtable. Type for the returned value must be checked using type casting.

[Docs](https://developer.tekla.com/topic/en/18/43/63b853a0-0af5-af9c-3ce7-05299664d7f8)

#### `GetChildren(...)`

Returns an enumerator of all the children reference model objects.

[Docs](https://developer.tekla.com/topic/en/18/43/729b427b-2ab5-8f20-1262-ec3ca0fae970)

#### `GetConvertedObjects(...)`

Returns an enumerator of model object converted from the reference model.

[Docs](https://developer.tekla.com/topic/en/18/43/c4709708-9407-9313-d363-47fe73e1626d)

#### `GetCoordinateSystem(...)`

Returns the coordinate system for the given model object.

[Docs](https://developer.tekla.com/topic/en/18/43/8adfe9cd-f6e5-6474-1297-33c6270a7b0a)

#### `GetCurrentRevision(...)`

Gets the current revision.

[Docs](https://developer.tekla.com/topic/en/18/43/e644c72a-529a-8dc7-f3d8-4f420743222a)

#### `GetCustomObjectType(...)`

Gets custom object type name from an existing object.

[Docs](https://developer.tekla.com/topic/en/18/43/d3be2ba3-8533-7165-f2d3-a5aa9df556b8)

#### `GetDoubleReportProperties(...)`

Retrieves multiple double report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/f86f6c96-f738-49ca-16ba-f5634e862ba2)

#### `GetDoubleUserProperties(...)`

Retrieves all double properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/6ead7a63-5f76-0631-d480-d46c27e882aa)

#### `GetDynamicStringProperty(...)`

Gets a dynamic string property from the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/415e86d9-7de6-ea53-ce68-20b3e67b8655)

#### `GetFatherComponent(...)`

Returns the father component of the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/a2c28f3f-52ff-50a8-1b32-10d18db5c31d)

#### `GetHierarchicObjects(...)`

Returns an enumerator of all the connected hierarchic objects.

[Docs](https://developer.tekla.com/topic/en/18/43/cc228ea0-267b-7d90-7441-c9efc1779b08)

#### `GetIntegerReportProperties(...)`

Retrieves multiple integer report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/2623a9c0-2a9c-e233-9daa-59f92ed51445)

#### `GetIntegerUserProperties(...)`

Retrieves all integer properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/1b3c6f06-bc25-a54e-2a88-b60dc4a3c85e)

#### `GetPhase(...)`

Retrieves the phase of the model object (the phase number, the phase name, the phase comment and whether the phase is the current one or not).

[Docs](https://developer.tekla.com/topic/en/18/43/310f8b7a-c120-753a-35c9-f7c73a9806c9)

#### `GetReferenceModelObjectByExternalGuid(...)`

Gets reference model object by external (IFC) guid from this reference model.

[Docs](https://developer.tekla.com/topic/en/18/43/6dcd0806-25ad-bcad-46ea-dd1d5a9214c1)

#### `GetReferenceModelObjectGuidsByExternalGuids(...)`

Gets reference model object guids by external guids.

[Docs](https://developer.tekla.com/topic/en/18/43/0a693785-9471-0c5e-a915-f09d0d1887fd)

#### `GetReportProperty(String, Double.)(...)`

Retrieves a double property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/1c05aa56-5f42-be91-a59e-8d59a5549f45)

#### `GetReportProperty(String, Int32.)(...)`

Retrieves an integer property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/ce8e4ca0-3750-3c9b-5ca8-27e48b45e0bd)

#### `GetReportProperty(String, String.)(...)`

Retrieves a string property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/70f212bb-ee70-e9e9-7ab6-d6a9d46f8de6)

#### `GetRevisions(...)`

Gets the reference models available revisions.

[Docs](https://developer.tekla.com/topic/en/18/43/f1b36d75-032f-63b7-9d83-11ab4894bbd6)

#### `GetStringReportProperties(...)`

Retrieves multiple string report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/5ff72da1-7382-f5d2-eb1d-c99b46239d42)

#### `GetStringUserProperties(...)`

Retrieves all string properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/67120e84-c820-9d52-12ae-c167c595842c)

#### `GetUserProperty(String, Double.)(...)`

Retrieves a double property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/17cc8074-7955-32b5-2b4b-b169c45d4ee0)

#### `GetUserProperty(String, Int32.)(...)`

Retrieves an integer property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/d9364c1d-5b57-6a31-4c13-01124058cfb2)

#### `GetUserProperty(String, String.)(...)`

Retrieves a string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/214a886c-c18c-9b66-d7c9-fe84260962f2)

#### `Insert(...)`

Inserts the reference model into the model.

[Docs](https://developer.tekla.com/topic/en/18/43/cf21c60e-c8c7-d1e0-48a0-185f609601cc)

#### `Modify(...)`

Modifies the reference model with the given identifier.

[Docs](https://developer.tekla.com/topic/en/18/43/db7b2238-5e56-7adb-e06a-cc91f71d21fe)

#### `RefreshFile(...)`

Reload the reference file from the location specified by Filename. If the file has changed, a new revision will be created and taken into use. Note that if a new revision was successfully created, the ActiveFilePath is changed.

[Docs](https://developer.tekla.com/topic/en/18/43/13291a3d-b362-4520-34c3-418563d3e06f)

#### `RemoveRevision(...)`

Removes the given revision from the reference model.

[Docs](https://developer.tekla.com/topic/en/18/43/8eff93f8-cfed-5f17-8767-00bd2fe1c375)

#### `Select(...)`

Selects the reference model with the given identifier.

[Docs](https://developer.tekla.com/topic/en/18/43/48df3abf-5d96-6af7-0921-a76530456bcd)

#### `SetAsCurrentRevision(Int32, Int32)(...)`

Sets the given revision as current revision.

[Docs](https://developer.tekla.com/topic/en/18/43/194aca9c-df74-e1ad-be42-3eacbe3538d2)

#### `SetAsCurrentRevision(ReferenceModel.Revision)(...)`

Sets the given revision as current revision.

[Docs](https://developer.tekla.com/topic/en/18/43/e311b023-4d53-2224-ef9a-b6134bba271e)

#### `SetCustomObjectType(...)`

Sets a custom object type name for an existing object.

[Docs](https://developer.tekla.com/topic/en/18/43/3d6fb91b-48a9-35ca-d498-526514344089)

#### `SetDynamicStringProperty(...)`

Sets a dynamic string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/4dab0254-09e5-5e61-c28b-1c12afab71a0)

#### `SetLabel(...)`

Sets a label for an object when a new instance is created, this method must be called before Insert. The label is used in plug-ins for identifying the changed object in modification.

[Docs](https://developer.tekla.com/topic/en/18/43/3efcfc44-613a-e66b-9ebd-9f86df4f4036)

#### `SetPhase(...)`

Sets the phase of the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/ed80abcc-0ff7-d081-2229-e651f858a727)

#### `SetUserProperties(...)`

Sets multiple properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/8659da37-d571-f03f-c666-5c9fa3b113f5)

#### `SetUserProperty(String, Double)(...)`

Sets a double property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/5c585b8b-5c2e-7b75-2242-758f17429396)

#### `SetUserProperty(String, Int32)(...)`

Sets an integer property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/dd5b828d-3a87-bb58-5ac9-e1c3b4c2fe4b)

#### `SetUserProperty(String, String)(...)`

Sets a string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/80bf0348-b651-3d4a-862b-49c85bc924ab)

### Properties
- `ActiveFilePath` (object, get/set) — Gets the path to the local copy of the current revision of the reference model.
- `BasePointGuid` (object, get/set) — The GUID of the used base point. Empty if base point is not used.
- `Filename` (object, get/set) — The path to the original location of the reference file.
- `Identifier` (object, get/set) — The identifier of the object.
- `IsUpToDate` (object, get/set) — Gets if the object does not have a modification which is not shared.
- `ModelGUID` (object, get/set) — The ModelGUID of the reference model in the Project Center.
- `ModificationTime` (object, get/set) — Gets latest time of the object was modified or created.
- `Position` (object, get/set) — The position of the reference model.
- `ProjectGUID` (object, get/set) — The ProjectGUID of the reference model in the Project Center.
- `Rotation` (object, get/set) — Gets or sets value of rotation around Z axis (degrees).
- `Rotation3D` (object, get/set) — Gets or sets value of 3d rotation.
- `Scale` (object, get/set) — The scale of the reference model.
- `Title` (object, get/set) — The reference model name(title).
- `UseWorkplane` (object, get/set) — Sets whether workplane is used.
- `VersionGUID` (object, get/set) — The VersionGUID of the reference model in the Project Center.
- `Visibility` (object, get/set) — The visibility of the reference model in the model view.

## ReferenceModel.Revision (struct)

Reference model revision.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/2d3e9160-2d8e-2de7-7c4c-978d447e3d92)

### Constructors
- `ReferenceModel.Revision(...)` — Parameterized constructor.

### Properties
- `FileName` (object, get/set) — Gets the revision file name.
- `Hash` (object, get/set) — Gets the revision hash identifier.
- `Id` (object, get/set) — Gets the revision id.
- `IsCurrentRevision` (object, get/set) — Gets the value indicating whether this revision is the current revision or not.
- `ReferenceModelId` (object, get/set) — Gets or sets the parent reference model id.
- `Time` (object, get/set) — Gets the revision time.

## ReferenceModel.VisibilityEnum (enum)

The visibility type.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/2af7cd9d-3835-c661-a69c-a9f2cde183b7)

### Values
- `HIDDEN` = `0` — The reference model is invisible in the model.
- `VISIBLE` = `1` — The reference model is visible in the model.

## ReferenceModelObject (class)

The ReferenceModelObject class contains methods related to reference model objects. A reference model object is a subobject of a reference model.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/98ca59de-9a53-e939-e397-12158eb2f840)

### Constructors
- `ReferenceModelObject(...)` — Creates a new reference model object instance.
- `ReferenceModelObject(...)` — Creates a new reference model object instance using the given parameters.

### Methods
#### `CompareTo(...)`

Compares Identifiers of model objects.

[Docs](https://developer.tekla.com/topic/en/18/43/79cca356-0c8a-e8fb-463a-34ad4141bec7)

#### `Delete(...)`

At the moment reference objects cannot be deleted.

[Docs](https://developer.tekla.com/topic/en/18/43/e8ede2d8-73d3-e7bf-0348-a0ecda17896b)

#### `Equals(...)`

Check if Identifiers of model objects are same.

[Docs](https://developer.tekla.com/topic/en/18/43/e4324265-2490-00de-4139-63a8f7acb492)

#### `GetAllReportProperties(...)`

Retrieves all properties for the model object in one hashtable. Type for the returned value must be checked using type casting.

[Docs](https://developer.tekla.com/topic/en/18/43/0708f404-ce77-28e1-4ebe-fa906f68bff8)

#### `GetAllUserProperties(...)`

Retrieves all properties for the model object in one hashtable. Type for the returned value must be checked using type casting.

[Docs](https://developer.tekla.com/topic/en/18/43/63b853a0-0af5-af9c-3ce7-05299664d7f8)

#### `GetChildren(...)`

Returns an enumerator of all the children model objects.

[Docs](https://developer.tekla.com/topic/en/18/43/ce51ec40-310c-6067-a89f-f8d029aab747)

#### `GetCoordinateSystem(...)`

Returns the coordinate system for the given model object.

[Docs](https://developer.tekla.com/topic/en/18/43/8adfe9cd-f6e5-6474-1297-33c6270a7b0a)

#### `GetCustomObjectType(...)`

Gets custom object type name from an existing object.

[Docs](https://developer.tekla.com/topic/en/18/43/d3be2ba3-8533-7165-f2d3-a5aa9df556b8)

#### `GetDoubleReportProperties(...)`

Retrieves multiple double report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/f86f6c96-f738-49ca-16ba-f5634e862ba2)

#### `GetDoubleUserProperties(...)`

Retrieves all double properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/6ead7a63-5f76-0631-d480-d46c27e882aa)

#### `GetDynamicStringProperty(...)`

Gets a dynamic string property from the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/415e86d9-7de6-ea53-ce68-20b3e67b8655)

#### `GetFather(...)`

Returns the reference hierarchy father.

[Docs](https://developer.tekla.com/topic/en/18/43/d8450c4f-ed78-8d13-b5db-496b921b82ca)

#### `GetFatherComponent(...)`

Returns the father component of the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/a2c28f3f-52ff-50a8-1b32-10d18db5c31d)

#### `GetHierarchicObjects(...)`

Returns an enumerator of all the connected hierarchic objects.

[Docs](https://developer.tekla.com/topic/en/18/43/cc228ea0-267b-7d90-7441-c9efc1779b08)

#### `GetIntegerReportProperties(...)`

Retrieves multiple integer report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/2623a9c0-2a9c-e233-9daa-59f92ed51445)

#### `GetIntegerUserProperties(...)`

Retrieves all integer properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/1b3c6f06-bc25-a54e-2a88-b60dc4a3c85e)

#### `GetPhase(...)`

Retrieves the phase of the model object (the phase number, the phase name, the phase comment and whether the phase is the current one or not).

[Docs](https://developer.tekla.com/topic/en/18/43/310f8b7a-c120-753a-35c9-f7c73a9806c9)

#### `GetReferenceModel(...)`

Gets the reference model that the reference object belongs to.

[Docs](https://developer.tekla.com/topic/en/18/43/268449d0-4c2f-fbc9-2282-9f5a3d5de7bf)

#### `GetReportProperty(String, Double.)(...)`

Retrieves a double property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/1c05aa56-5f42-be91-a59e-8d59a5549f45)

#### `GetReportProperty(String, Int32.)(...)`

Retrieves an integer property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/ce8e4ca0-3750-3c9b-5ca8-27e48b45e0bd)

#### `GetReportProperty(String, String.)(...)`

Retrieves a string property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/70f212bb-ee70-e9e9-7ab6-d6a9d46f8de6)

#### `GetStringReportProperties(...)`

Retrieves multiple string report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/5ff72da1-7382-f5d2-eb1d-c99b46239d42)

#### `GetStringUserProperties(...)`

Retrieves all string properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/67120e84-c820-9d52-12ae-c167c595842c)

#### `GetUserProperty(String, Double.)(...)`

Retrieves a double property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/17cc8074-7955-32b5-2b4b-b169c45d4ee0)

#### `GetUserProperty(String, Int32.)(...)`

Retrieves an integer property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/d9364c1d-5b57-6a31-4c13-01124058cfb2)

#### `GetUserProperty(String, String.)(...)`

Retrieves a string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/214a886c-c18c-9b66-d7c9-fe84260962f2)

#### `Insert(...)`

At the moment reference objects cannot be inserted.

[Docs](https://developer.tekla.com/topic/en/18/43/5883c0c3-902a-0a21-a690-e74c7866ec9f)

#### `Modify(...)`

At the moment reference objects cannot be modified.

[Docs](https://developer.tekla.com/topic/en/18/43/921fd452-6b1d-b9ba-114b-8fe4bbe78091)

#### `Select(...)`

Selects the reference model with the given identifier.

[Docs](https://developer.tekla.com/topic/en/18/43/76108732-4c6d-e72d-74a5-ea07c6820b2f)

#### `SetCustomObjectType(...)`

Sets a custom object type name for an existing object.

[Docs](https://developer.tekla.com/topic/en/18/43/3d6fb91b-48a9-35ca-d498-526514344089)

#### `SetDynamicStringProperty(...)`

Sets a dynamic string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/4dab0254-09e5-5e61-c28b-1c12afab71a0)

#### `SetLabel(...)`

Sets a label for an object when a new instance is created, this method must be called before Insert. The label is used in plug-ins for identifying the changed object in modification.

[Docs](https://developer.tekla.com/topic/en/18/43/3efcfc44-613a-e66b-9ebd-9f86df4f4036)

#### `SetPhase(...)`

Sets the phase of the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/ed80abcc-0ff7-d081-2229-e651f858a727)

#### `SetUserProperties(...)`

Sets multiple properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/8659da37-d571-f03f-c666-5c9fa3b113f5)

#### `SetUserProperty(String, Double)(...)`

Sets a double property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/5c585b8b-5c2e-7b75-2242-758f17429396)

#### `SetUserProperty(String, Int32)(...)`

Sets an integer property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/dd5b828d-3a87-bb58-5ac9-e1c3b4c2fe4b)

#### `SetUserProperty(String, String)(...)`

Sets a string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/80bf0348-b651-3d4a-862b-49c85bc924ab)

### Properties
- `Identifier` (object, get/set) — The identifier of the object.
- `IsUpToDate` (object, get/set) — Gets if the object does not have a modification which is not shared.
- `ModificationTime` (object, get/set) — Gets latest time of the object was modified or created.

## Reinforcement (class)

The Reinforcement class represents a reinforcement in the model. A reinforcement can either be a mesh, a single rebar, a rebar group or a strand.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/c3d81ba0-8bd9-6e21-d36d-e650c6a0ef17)

### Methods
#### `CompareTo(...)`

Compares Identifiers of model objects.

[Docs](https://developer.tekla.com/topic/en/18/43/79cca356-0c8a-e8fb-463a-34ad4141bec7)

#### `Delete(...)`

Deletes the instance from the model database.

[Docs](https://developer.tekla.com/topic/en/18/43/0b922776-ccee-4b5e-4a12-4e5306802e90)

#### `Equals(...)`

Check if Identifiers of model objects are same.

[Docs](https://developer.tekla.com/topic/en/18/43/e4324265-2490-00de-4139-63a8f7acb492)

#### `GetAllReportProperties(...)`

Retrieves all properties for the model object in one hashtable. Type for the returned value must be checked using type casting.

[Docs](https://developer.tekla.com/topic/en/18/43/0708f404-ce77-28e1-4ebe-fa906f68bff8)

#### `GetAllUserProperties(...)`

Retrieves all properties for the model object in one hashtable. Type for the returned value must be checked using type casting.

[Docs](https://developer.tekla.com/topic/en/18/43/63b853a0-0af5-af9c-3ce7-05299664d7f8)

#### `GetAssembly(...)`

Returns the assembly that the reinforcement belongs to.

[Docs](https://developer.tekla.com/topic/en/18/43/322f932e-f09b-df0d-c1c8-3de07d79afeb)

#### `GetChildren(...)`

Returns an enumerator of all the children model objects.

[Docs](https://developer.tekla.com/topic/en/18/43/ce51ec40-310c-6067-a89f-f8d029aab747)

#### `GetCoordinateSystem(...)`

Returns the coordinate system for the given model object.

[Docs](https://developer.tekla.com/topic/en/18/43/8adfe9cd-f6e5-6474-1297-33c6270a7b0a)

#### `GetCustomObjectType(...)`

Gets custom object type name from an existing object.

[Docs](https://developer.tekla.com/topic/en/18/43/d3be2ba3-8533-7165-f2d3-a5aa9df556b8)

#### `GetDoubleReportProperties(...)`

Retrieves multiple double report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/f86f6c96-f738-49ca-16ba-f5634e862ba2)

#### `GetDoubleUserProperties(...)`

Retrieves all double properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/6ead7a63-5f76-0631-d480-d46c27e882aa)

#### `GetDynamicStringProperty(...)`

Gets a dynamic string property from the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/415e86d9-7de6-ea53-ce68-20b3e67b8655)

#### `GetFatherComponent(...)`

Returns the father component of the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/a2c28f3f-52ff-50a8-1b32-10d18db5c31d)

#### `GetFatherPour(...)`

Returns the pour that the rebar is associated to.

[Docs](https://developer.tekla.com/topic/en/18/43/0725e80c-89cf-d6d7-ac40-afdf75f6e695)

#### `GetFatherPourUnit(...)`

Returns the pour unit that the rebar is associated to.

[Docs](https://developer.tekla.com/topic/en/18/43/135a51ac-86cb-ef86-cfda-d2bc53a0d8b8)

#### `GetHierarchicObjects(...)`

Returns an enumerator of all the connected hierarchic objects.

[Docs](https://developer.tekla.com/topic/en/18/43/cc228ea0-267b-7d90-7441-c9efc1779b08)

#### `GetIntegerReportProperties(...)`

Retrieves multiple integer report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/2623a9c0-2a9c-e233-9daa-59f92ed51445)

#### `GetIntegerUserProperties(...)`

Retrieves all integer properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/1b3c6f06-bc25-a54e-2a88-b60dc4a3c85e)

#### `GetNumberOfRebars(...)`

Returns the number of rebars in the reinforcing group.

[Docs](https://developer.tekla.com/topic/en/18/43/6b02fd6f-4f84-6550-69c4-bd2f1117b922)

#### `GetPhase(...)`

Retrieves the phase of the model object (the phase number, the phase name, the phase comment and whether the phase is the current one or not).

[Docs](https://developer.tekla.com/topic/en/18/43/310f8b7a-c120-753a-35c9-f7c73a9806c9)

#### `GetRebarComplexGeometries(Boolean, Boolean, Boolean)(...)`

Retrieves a list of physical reinforcing bars (of type RebarComplexGeometry). These objects contain physical curves in the 3D space of each reinforcing bar as shown in model view. RebarComplexGeometry only differs from RebarGeometry by containing a polycurve rather than a polyline.

[Docs](https://developer.tekla.com/topic/en/18/43/c796aafb-3520-fe06-12e3-a07a87c8f2f9)

#### `GetRebarComplexGeometries(Boolean, Boolean, Boolean, Reinforcement.RebarGeometrySimplificationTypeEnum)(...)`

Retrieves a list of physical reinforcing bars (of type RebarComplexGeometry). These objects contain physical curves in the 3D space of each reinforcing bar as shown in model view. RebarComplexGeometry only differs from RebarGeometry by containing a polycurve (of Arc and LineSegment objects) rather than a polyline. The polycurve only contains arcs if the "simplified" parameter is RATIONALIZED or FABRICATION. If "simplified" is NONE, any arcs will be returned as a series of line segments.

[Docs](https://developer.tekla.com/topic/en/18/43/84a3a38b-d7eb-97df-7fba-a34d13de6610)

#### `GetRebarGeometries(Boolean)(...)`

Retrieves a list of physical reinforcing bars (of type RebarGeometry). These objects contain physical points in the 3D space of each reinforcing bar.

[Docs](https://developer.tekla.com/topic/en/18/43/75649a4c-6252-09a1-b509-cd815e718c0f)

#### `GetRebarGeometries(Reinforcement.RebarGeometryOptionEnum)(...)`

Retrieves a list of physical reinforcing bars (of type RebarGeometry). These objects contain physical points in the 3D space of each reinforcing bar.

[Docs](https://developer.tekla.com/topic/en/18/43/bc45309d-fc4b-9bf9-c640-e71bb1ade771)

#### `GetRebarGeometriesWithoutClashes(...)`

Retrieves a list of physical reinforcing bars (of type RebarGeometry). These objects contain physical points in the 3D space of each reinforcing bar as shown in model view. In case rebar polygon clashes with itself, physical points are moved to avoid clashing.

[Docs](https://developer.tekla.com/topic/en/18/43/470acf1f-1beb-f30a-a1c2-2fcab066ab4a)

#### `GetReportProperty(String, Double.)(...)`

Retrieves a double property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/1c05aa56-5f42-be91-a59e-8d59a5549f45)

#### `GetReportProperty(String, Int32.)(...)`

Retrieves an integer property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/ce8e4ca0-3750-3c9b-5ca8-27e48b45e0bd)

#### `GetReportProperty(String, String.)(...)`

Retrieves a string property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/70f212bb-ee70-e9e9-7ab6-d6a9d46f8de6)

#### `GetSingleRebar(...)`

Returns a single rebar inside the rebar group located by the given index. The indexing starts from the start point. The rebar represents a physical reinforcing bar and contains physical points in the 3D space of the bar. The method returns null on error, for example if given an erroneous index. The number of rebars in the group can be requested with GetNumberOfRebars().

[Docs](https://developer.tekla.com/topic/en/18/43/41181569-65f8-f4ba-7722-3cd890c7a3d4)

#### `GetSingleRebarWithoutClash(...)`

Returns a single rebar inside the rebar group located by the given index. The indexing starts from the start point. The rebar represents a physical reinforcing bar and contains physical, non-clashing points in the 3D space of the bar. The method returns null on error, for example if given an erroneous index. The number of rebars in the group can be requested with GetNumberOfRebars().

[Docs](https://developer.tekla.com/topic/en/18/43/8fd2e21a-3d2f-6211-92ba-9d6e652ac100)

#### `GetSolid(...)`

Method for getting the solid information of the reinforcement.

[Docs](https://developer.tekla.com/topic/en/18/43/6717d119-97da-5554-f359-49ee55e65a63)

#### `GetStringReportProperties(...)`

Retrieves multiple string report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/5ff72da1-7382-f5d2-eb1d-c99b46239d42)

#### `GetStringUserProperties(...)`

Retrieves all string properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/67120e84-c820-9d52-12ae-c167c595842c)

#### `GetUserProperty(String, Double.)(...)`

Retrieves a double property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/17cc8074-7955-32b5-2b4b-b169c45d4ee0)

#### `GetUserProperty(String, Int32.)(...)`

Retrieves an integer property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/d9364c1d-5b57-6a31-4c13-01124058cfb2)

#### `GetUserProperty(String, String.)(...)`

Retrieves a string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/214a886c-c18c-9b66-d7c9-fe84260962f2)

#### `Insert(...)`

Inserts the model object instance into the model database.

[Docs](https://developer.tekla.com/topic/en/18/43/f17ed840-1d16-b843-4b0c-270875de1d35)

#### `IsGeometryValid(...)`

Tells whether the geometry of a reinforcement object is valid or not.

[Docs](https://developer.tekla.com/topic/en/18/43/5699f6a6-4e39-b784-d84f-45eb7b1d0e37)

#### `Modify(...)`

Modifies the model instance in the model database.

[Docs](https://developer.tekla.com/topic/en/18/43/186833b8-4837-ed77-b314-3ebf9dc32d6a)

#### `Select(...)`

Selects the model object instance from the model database.

[Docs](https://developer.tekla.com/topic/en/18/43/f662ab10-be15-6280-d0aa-fc259dfd7c07)

#### `SetCustomObjectType(...)`

Sets a custom object type name for an existing object.

[Docs](https://developer.tekla.com/topic/en/18/43/3d6fb91b-48a9-35ca-d498-526514344089)

#### `SetDynamicStringProperty(...)`

Sets a dynamic string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/4dab0254-09e5-5e61-c28b-1c12afab71a0)

#### `SetLabel(...)`

Sets a label for an object when a new instance is created, this method must be called before Insert. The label is used in plug-ins for identifying the changed object in modification.

[Docs](https://developer.tekla.com/topic/en/18/43/3efcfc44-613a-e66b-9ebd-9f86df4f4036)

#### `SetPhase(...)`

Sets the phase of the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/ed80abcc-0ff7-d081-2229-e651f858a727)

#### `SetUserProperties(...)`

Sets multiple properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/8659da37-d571-f03f-c666-5c9fa3b113f5)

#### `SetUserProperty(String, Double)(...)`

Sets a double property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/5c585b8b-5c2e-7b75-2242-758f17429396)

#### `SetUserProperty(String, Int32)(...)`

Sets an integer property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/dd5b828d-3a87-bb58-5ac9-e1c3b4c2fe4b)

#### `SetUserProperty(String, String)(...)`

Sets a string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/80bf0348-b651-3d4a-862b-49c85bc924ab)

### Properties
- `Class` (object, get/set) — Gets or sets the class of the reinforcement. The class is used to group reinforcements.
- `EndPointOffsetType` (object, get/set) — Gets or sets the type of the end point offset. The options are: OFFSET_TYPE_LEG_LENGTHOFFSET_TYPE_COVER_THICKNESS
- `EndPointOffsetValue` (object, get/set) — Gets or sets the concrete cover thickness or leg length at the second end of the bar.
- `Father` (object, get/set) — Gets or sets the father object of the reinforcement; the model object instance to operate on.
- `FromPlaneOffset` (object, get/set) — Gets or sets the offset value from the part surface.
- `Grade` (object, get/set) — Gets or sets the steel grade of the reinforcing bar. The grade indicates the strength of the steel used in reinforcing bars. It can also indicate other factors, such as the weldability or surface deformations of the bar.
- `Identifier` (object, get/set) — The identifier of the object.
- `InputPointDeformingState` (object, get/set) — Gets or sets the reinforcement input point deforming state.
- `IsUpToDate` (object, get/set) — Gets if the object does not have a modification which is not shared.
- `ModificationTime` (object, get/set) — Gets latest time of the object was modified or created.
- `Name` (object, get/set) — Gets or sets the name of the reinforcement.
- `NumberingSeries` (object, get/set) — Gets or sets the numbering series of the reinforcement.
- `OnPlaneOffsets` (object, get/set) — Gets or sets the double offset value for each leg on the same plane as the bar.
- `RadiusValues` (object, get/set) — Gets or sets the radius value(s) of the bends in the bar.
- `StartPointOffsetType` (object, get/set) — Gets or sets the type of the start point offset is either OFFSET_TYPE_LEG_LENGTH or OFFSET_TYPE_COVER_THICKNESS.
- `StartPointOffsetValue` (object, get/set) — Gets or sets the concrete cover thickness or leg length at the first end of the bar.

## Reinforcement.RebarGeometryOptionEnum (enum)

Options for the function GetRebarGeometries.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/7108e0fe-54a6-1bb5-2c14-3a461eff6bda)

### Values
- `NONE` = `0` — No hooks or clash avoidance or length adjustment.
- `HOOKS` = `1` — Hooks.
- `AVOID_CLASH` = `2` — In case rebar polygon clashes with itself, physical points are moved to avoid clashing.
- `LENGTH_ADJUSTMENTS` = `4` — Length adjustments

## Reinforcement.RebarGeometrySimplificationTypeEnum (enum)

The rebar geometry simplification type.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/9138fa19-1a9b-02bc-4710-a8856a76edc3)

### Values
- `NONE` = `0` — No simplification.
- `RATIONALIZED` = `1` — Rationalized geometry - includes segmented arcs converted to true arcs by Rebar Shape Recognition.
- `FABRICATION` = `2` — Fabrication geometry - includes arcs recognized as straight by Rebar Shape Recognition.

## Reinforcement.RebarOffsetTypeEnum (enum)

The different types of offsets.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/a0e850ad-1bda-6e5d-09b5-e64010191033)

### Values
- `OFFSET_TYPE_COVER_THICKNESS` = `0` — The concrete cover thickness.
- `OFFSET_TYPE_LEG_LENGTH` = `1` — The leg length.

## Seam (class)

The Seam class represents a seam. A seam is something that connects a main part to other parts along an edge.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/99d98d23-1040-9c9e-cab2-4d89c1e43f1e)

### Constructors
- `Seam(...)` — Creates a new seam instance.

### Methods
#### `CompareTo(...)`

Compares Identifiers of model objects.

[Docs](https://developer.tekla.com/topic/en/18/43/79cca356-0c8a-e8fb-463a-34ad4141bec7)

#### `Delete(...)`

Deletes the seam instance with the given ID from the model database.

[Docs](https://developer.tekla.com/topic/en/18/43/7db02b19-2b09-df66-8376-19b030000fd7)

#### `Equals(...)`

Check if Identifiers of model objects are same.

[Docs](https://developer.tekla.com/topic/en/18/43/e4324265-2490-00de-4139-63a8f7acb492)

#### `GetAllReportProperties(...)`

Retrieves all properties for the model object in one hashtable. Type for the returned value must be checked using type casting.

[Docs](https://developer.tekla.com/topic/en/18/43/0708f404-ce77-28e1-4ebe-fa906f68bff8)

#### `GetAllUserProperties(...)`

Retrieves all properties for the model object in one hashtable. Type for the returned value must be checked using type casting.

[Docs](https://developer.tekla.com/topic/en/18/43/63b853a0-0af5-af9c-3ce7-05299664d7f8)

#### `GetAttribute(String, Double.)(...)`

Retrieves the attribute with the given name.

[Docs](https://developer.tekla.com/topic/en/18/43/a347ce9e-865f-554d-140f-9a65728b24a3)

#### `GetAttribute(String, Int32.)(...)`

Retrieves the attribute with the given name.

[Docs](https://developer.tekla.com/topic/en/18/43/2c48112c-5580-99f3-6c35-70fd5f538b09)

#### `GetAttribute(String, String.)(...)`

Retrieves the attribute with the given name.

[Docs](https://developer.tekla.com/topic/en/18/43/c8652aa1-9069-88f2-b821-3afdd68e7ca5)

#### `GetChildren(...)`

Returns an enumerator of all the children model objects.

[Docs](https://developer.tekla.com/topic/en/18/43/ce51ec40-310c-6067-a89f-f8d029aab747)

#### `GetCoordinateSystem(...)`

Returns the coordinate system for the given model object.

[Docs](https://developer.tekla.com/topic/en/18/43/8adfe9cd-f6e5-6474-1297-33c6270a7b0a)

#### `GetCustomObjectType(...)`

Gets custom object type name from an existing object.

[Docs](https://developer.tekla.com/topic/en/18/43/d3be2ba3-8533-7165-f2d3-a5aa9df556b8)

#### `GetDoubleReportProperties(...)`

Retrieves multiple double report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/f86f6c96-f738-49ca-16ba-f5634e862ba2)

#### `GetDoubleUserProperties(...)`

Retrieves all double properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/6ead7a63-5f76-0631-d480-d46c27e882aa)

#### `GetDynamicStringProperty(...)`

Gets a dynamic string property from the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/415e86d9-7de6-ea53-ce68-20b3e67b8655)

#### `GetFatherComponent(...)`

Returns the father component of the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/a2c28f3f-52ff-50a8-1b32-10d18db5c31d)

#### `GetHierarchicObjects(...)`

Returns an enumerator of all the connected hierarchic objects.

[Docs](https://developer.tekla.com/topic/en/18/43/cc228ea0-267b-7d90-7441-c9efc1779b08)

#### `GetInputPolygon(...)`

Returns the input polygon of the seam.

[Docs](https://developer.tekla.com/topic/en/18/43/6ed2fd6f-9e42-f82d-b0c0-e0702ac27519)

#### `GetIntegerReportProperties(...)`

Retrieves multiple integer report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/2623a9c0-2a9c-e233-9daa-59f92ed51445)

#### `GetIntegerUserProperties(...)`

Retrieves all integer properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/1b3c6f06-bc25-a54e-2a88-b60dc4a3c85e)

#### `GetPhase(...)`

Retrieves the phase of the model object (the phase number, the phase name, the phase comment and whether the phase is the current one or not).

[Docs](https://developer.tekla.com/topic/en/18/43/310f8b7a-c120-753a-35c9-f7c73a9806c9)

#### `GetPrimaryObject(...)`

Returns the primary object.

[Docs](https://developer.tekla.com/topic/en/18/43/a410c572-b40d-070c-f332-91f6f5fd8925)

#### `GetReportProperty(String, Double.)(...)`

Retrieves a double property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/1c05aa56-5f42-be91-a59e-8d59a5549f45)

#### `GetReportProperty(String, Int32.)(...)`

Retrieves an integer property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/ce8e4ca0-3750-3c9b-5ca8-27e48b45e0bd)

#### `GetReportProperty(String, String.)(...)`

Retrieves a string property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/70f212bb-ee70-e9e9-7ab6-d6a9d46f8de6)

#### `GetSecondaryObjects(...)`

Returns an array list containing all the secondary objects.

[Docs](https://developer.tekla.com/topic/en/18/43/4637ebb5-1624-f867-8d6c-b0317513ce8c)

#### `GetStartAndEndPositions(...)`

Returns the start and end points of the seam.

[Docs](https://developer.tekla.com/topic/en/18/43/db1a0567-7d4c-f6e8-084a-c504f1542e16)

#### `GetStringReportProperties(...)`

Retrieves multiple string report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/5ff72da1-7382-f5d2-eb1d-c99b46239d42)

#### `GetStringUserProperties(...)`

Retrieves all string properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/67120e84-c820-9d52-12ae-c167c595842c)

#### `GetUserProperty(String, Double.)(...)`

Retrieves a double property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/17cc8074-7955-32b5-2b4b-b169c45d4ee0)

#### `GetUserProperty(String, Int32.)(...)`

Retrieves an integer property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/d9364c1d-5b57-6a31-4c13-01124058cfb2)

#### `GetUserProperty(String, String.)(...)`

Retrieves a string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/214a886c-c18c-9b66-d7c9-fe84260962f2)

#### `Insert(...)`

Inserts the seam into the model database. All the attributes must be set.

[Docs](https://developer.tekla.com/topic/en/18/43/bba02635-2aca-b896-0aa6-ada425872e8d)

#### `LoadAttributesFromFile(...)`

Loads the attributes for the component from the given file. These attributes will be loaded before all the attributes that have been set with the SetAttribute methods, so any attributes that are set with SetAttribute will override those loaded from the given standard file.

[Docs](https://developer.tekla.com/topic/en/18/43/acc9930a-aa6f-667c-0878-7ea80ce416b8)

#### `Modify(...)`

Modifies the existing seam in the model database to match the current one.

[Docs](https://developer.tekla.com/topic/en/18/43/4aec1556-5235-88c8-7683-28eaabfab252)

#### `Select(...)`

Selects a seam from the model database. The seam ID must be set.

[Docs](https://developer.tekla.com/topic/en/18/43/a1f1ae56-1b21-a475-f889-10c3e4f6e3d4)

#### `SetAttribute(String, Double)(...)`

Sets the attribute's value to the given value.

[Docs](https://developer.tekla.com/topic/en/18/43/785d0216-df12-53fc-344b-a2e67001cf3c)

#### `SetAttribute(String, Int32)(...)`

Sets the attribute's value to the given value.

[Docs](https://developer.tekla.com/topic/en/18/43/797240ce-567c-bff9-2a60-6359cc909ce0)

#### `SetAttribute(String, String)(...)`

Sets the attribute's value to the given value.

[Docs](https://developer.tekla.com/topic/en/18/43/6f0f7c8f-c5d2-8012-9faf-f68991966ff5)

#### `SetCustomObjectType(...)`

Sets a custom object type name for an existing object.

[Docs](https://developer.tekla.com/topic/en/18/43/3d6fb91b-48a9-35ca-d498-526514344089)

#### `SetDynamicStringProperty(...)`

Sets a dynamic string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/4dab0254-09e5-5e61-c28b-1c12afab71a0)

#### `SetInputPolygon(...)`

Sets the input polygon of the seam.

[Docs](https://developer.tekla.com/topic/en/18/43/50bba7e4-0230-7108-37ba-24ef6beecc84)

#### `SetInputPositions(...)`

Sets the start and end point of the seam.

[Docs](https://developer.tekla.com/topic/en/18/43/12e43c2f-be73-1f45-8c8b-fc38d6b4ef2e)

#### `SetLabel(...)`

Sets a label for an object when a new instance is created, this method must be called before Insert. The label is used in plug-ins for identifying the changed object in modification.

[Docs](https://developer.tekla.com/topic/en/18/43/3efcfc44-613a-e66b-9ebd-9f86df4f4036)

#### `SetPhase(...)`

Sets the phase of the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/ed80abcc-0ff7-d081-2229-e651f858a727)

#### `SetPrimaryObject(...)`

Sets the primary object.

[Docs](https://developer.tekla.com/topic/en/18/43/6e5d9a16-126d-9c68-df4f-df73c8f26bf1)

#### `SetSecondaryObject(...)`

Sets the secondary object of the seam. Use this method if you wish to add only one secondary object to the seam.

[Docs](https://developer.tekla.com/topic/en/18/43/a578f577-d277-ddc1-c792-b9e600789340)

#### `SetSecondaryObjects(...)`

Sets the secondary objects.

[Docs](https://developer.tekla.com/topic/en/18/43/d291129c-af81-476c-24af-88fba69e40b5)

#### `SetUserProperties(...)`

Sets multiple properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/8659da37-d571-f03f-c666-5c9fa3b113f5)

#### `SetUserProperty(String, Double)(...)`

Sets a double property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/5c585b8b-5c2e-7b75-2242-758f17429396)

#### `SetUserProperty(String, Int32)(...)`

Sets an integer property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/dd5b828d-3a87-bb58-5ac9-e1c3b4c2fe4b)

#### `SetUserProperty(String, String)(...)`

Sets a string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/80bf0348-b651-3d4a-862b-49c85bc924ab)

### Properties
- `AutoDirectionType` (object, get/set) — The auto direction type. Default value is AUTODIR_BASIC. Value should be AUTODIR_NA to set direction with UpVector property. Value should be AUTODIR_FROM_ATTRIBUTE_FILE to set direction with seam attribute file.
- `AutoPosition` (object, get/set) — The auto position type. Defines that the seam will internally evaluate the final position more accurately. The input positions are still needed for reference.
- `Class` (object, get/set) — The class of the seam.
- `Code` (object, get/set) — The code of the seam. The code can be used to classify the seam. The code can be reported and shown in drawings. The maximum length of the string is 20 characters.
- `Identifier` (object, get/set) — The identifier of the object.
- `IsUpToDate` (object, get/set) — Gets if the object does not have a modification which is not shared.
- `ModificationTime` (object, get/set) — Gets latest time of the object was modified or created.
- `Name` (object, get/set) — The name of the component. The name identifies custom components or plug-ins.
- `Number` (object, get/set) — The number of the component. A number greater than zero identifies system components, for custom components the number is CUSTOM_OBJECT_NUMBER, and for plug-ins the number is PLUGIN_OBJECT_NUMBER.
- `Status` (object, get/set) — The read only status of the seam. The status can be reported and shown in drawings. The color of the connection symbol in the model indicates the status of the seam.
- `UpVector` (object, get/set) — A vector indicating which direction is considered the up direction for the seam.

## SelfIntersectingSurfaceException (class)

This class represent an exception thrown when the an operation on a lofted plate has been determined to cause a self intersecting surface

[Vendor docs](https://developer.tekla.com/topic/en/18/43/9f7aca96-ef41-ae5f-d61a-43d8bc994304)

### Constructors
- `SelfIntersectingSurfaceException(...)` — Constructs an instance of the class

## SingleRebar (class)

The SingleRebar class represents a single reinforcing bar.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/f6ddf6c4-47c6-8017-1a14-7b6a9ddd4961)

### Constructors
- `SingleRebar(...)` — Initializes a new single rebar instance with empty attributes.

### Methods
#### `CompareTo(...)`

Compares Identifiers of model objects.

[Docs](https://developer.tekla.com/topic/en/18/43/79cca356-0c8a-e8fb-463a-34ad4141bec7)

#### `Delete(...)`

Deletes the single rebar instance with the given identifier from the model database.

[Docs](https://developer.tekla.com/topic/en/18/43/e3eb6ed3-e5db-19bb-9cd8-bf5075a1f466)

#### `Equals(...)`

Check if Identifiers of model objects are same.

[Docs](https://developer.tekla.com/topic/en/18/43/e4324265-2490-00de-4139-63a8f7acb492)

#### `GetAllReportProperties(...)`

Retrieves all properties for the model object in one hashtable. Type for the returned value must be checked using type casting.

[Docs](https://developer.tekla.com/topic/en/18/43/0708f404-ce77-28e1-4ebe-fa906f68bff8)

#### `GetAllUserProperties(...)`

Retrieves all properties for the model object in one hashtable. Type for the returned value must be checked using type casting.

[Docs](https://developer.tekla.com/topic/en/18/43/63b853a0-0af5-af9c-3ce7-05299664d7f8)

#### `GetAssembly(...)`

Returns the assembly that the reinforcement belongs to.

[Docs](https://developer.tekla.com/topic/en/18/43/322f932e-f09b-df0d-c1c8-3de07d79afeb)

#### `GetChildren(...)`

Returns an enumerator of all the children model objects.

[Docs](https://developer.tekla.com/topic/en/18/43/ce51ec40-310c-6067-a89f-f8d029aab747)

#### `GetCoordinateSystem(...)`

Returns the coordinate system for the given model object.

[Docs](https://developer.tekla.com/topic/en/18/43/8adfe9cd-f6e5-6474-1297-33c6270a7b0a)

#### `GetCustomObjectType(...)`

Gets custom object type name from an existing object.

[Docs](https://developer.tekla.com/topic/en/18/43/d3be2ba3-8533-7165-f2d3-a5aa9df556b8)

#### `GetDoubleReportProperties(...)`

Retrieves multiple double report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/f86f6c96-f738-49ca-16ba-f5634e862ba2)

#### `GetDoubleUserProperties(...)`

Retrieves all double properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/6ead7a63-5f76-0631-d480-d46c27e882aa)

#### `GetDynamicStringProperty(...)`

Gets a dynamic string property from the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/415e86d9-7de6-ea53-ce68-20b3e67b8655)

#### `GetFatherComponent(...)`

Returns the father component of the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/a2c28f3f-52ff-50a8-1b32-10d18db5c31d)

#### `GetFatherPour(...)`

Returns the pour that the rebar is associated to.

[Docs](https://developer.tekla.com/topic/en/18/43/0725e80c-89cf-d6d7-ac40-afdf75f6e695)

#### `GetFatherPourUnit(...)`

Returns the pour unit that the rebar is associated to.

[Docs](https://developer.tekla.com/topic/en/18/43/135a51ac-86cb-ef86-cfda-d2bc53a0d8b8)

#### `GetHierarchicObjects(...)`

Returns an enumerator of all the connected hierarchic objects.

[Docs](https://developer.tekla.com/topic/en/18/43/cc228ea0-267b-7d90-7441-c9efc1779b08)

#### `GetIntegerReportProperties(...)`

Retrieves multiple integer report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/2623a9c0-2a9c-e233-9daa-59f92ed51445)

#### `GetIntegerUserProperties(...)`

Retrieves all integer properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/1b3c6f06-bc25-a54e-2a88-b60dc4a3c85e)

#### `GetNumberOfRebars(...)`

Returns the number of rebars in the reinforcing group.

[Docs](https://developer.tekla.com/topic/en/18/43/6b02fd6f-4f84-6550-69c4-bd2f1117b922)

#### `GetPhase(...)`

Retrieves the phase of the model object (the phase number, the phase name, the phase comment and whether the phase is the current one or not).

[Docs](https://developer.tekla.com/topic/en/18/43/310f8b7a-c120-753a-35c9-f7c73a9806c9)

#### `GetRebarComplexGeometries(Boolean, Boolean, Boolean)(...)`

Retrieves a list of physical reinforcing bars (of type RebarComplexGeometry). These objects contain physical curves in the 3D space of each reinforcing bar as shown in model view. RebarComplexGeometry only differs from RebarGeometry by containing a polycurve rather than a polyline.

[Docs](https://developer.tekla.com/topic/en/18/43/c796aafb-3520-fe06-12e3-a07a87c8f2f9)

#### `GetRebarComplexGeometries(Boolean, Boolean, Boolean, Reinforcement.RebarGeometrySimplificationTypeEnum)(...)`

Retrieves a list of physical reinforcing bars (of type RebarComplexGeometry). These objects contain physical curves in the 3D space of each reinforcing bar as shown in model view. RebarComplexGeometry only differs from RebarGeometry by containing a polycurve (of Arc and LineSegment objects) rather than a polyline. The polycurve only contains arcs if the "simplified" parameter is RATIONALIZED or FABRICATION. If "simplified" is NONE, any arcs will be returned as a series of line segments.

[Docs](https://developer.tekla.com/topic/en/18/43/84a3a38b-d7eb-97df-7fba-a34d13de6610)

#### `GetRebarGeometries(Boolean)(...)`

Retrieves a list of physical reinforcing bars (of type RebarGeometry). These objects contain physical points in the 3D space of each reinforcing bar.

[Docs](https://developer.tekla.com/topic/en/18/43/75649a4c-6252-09a1-b509-cd815e718c0f)

#### `GetRebarGeometries(Reinforcement.RebarGeometryOptionEnum)(...)`

Retrieves a list of physical reinforcing bars (of type RebarGeometry). These objects contain physical points in the 3D space of each reinforcing bar.

[Docs](https://developer.tekla.com/topic/en/18/43/bc45309d-fc4b-9bf9-c640-e71bb1ade771)

#### `GetRebarGeometriesWithoutClashes(...)`

Retrieves a list of physical reinforcing bars (of type RebarGeometry). These objects contain physical points in the 3D space of each reinforcing bar as shown in model view. In case rebar polygon clashes with itself, physical points are moved to avoid clashing.

[Docs](https://developer.tekla.com/topic/en/18/43/470acf1f-1beb-f30a-a1c2-2fcab066ab4a)

#### `GetRebarSet(...)`

Gets the parent RebarSet of the single rebar.

[Docs](https://developer.tekla.com/topic/en/18/43/e98179a4-33b3-a938-9d57-368404e87668)

#### `GetReportProperty(String, Double.)(...)`

Retrieves a double property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/1c05aa56-5f42-be91-a59e-8d59a5549f45)

#### `GetReportProperty(String, Int32.)(...)`

Retrieves an integer property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/ce8e4ca0-3750-3c9b-5ca8-27e48b45e0bd)

#### `GetReportProperty(String, String.)(...)`

Retrieves a string property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/70f212bb-ee70-e9e9-7ab6-d6a9d46f8de6)

#### `GetSingleRebar(...)`

Returns a single rebar inside the rebar group located by the given index. The indexing starts from the start point. The rebar represents a physical reinforcing bar and contains physical points in the 3D space of the bar. The method returns null on error, for example if given an erroneous index. The number of rebars in the group can be requested with GetNumberOfRebars().

[Docs](https://developer.tekla.com/topic/en/18/43/41181569-65f8-f4ba-7722-3cd890c7a3d4)

#### `GetSingleRebarWithoutClash(...)`

Returns a single rebar inside the rebar group located by the given index. The indexing starts from the start point. The rebar represents a physical reinforcing bar and contains physical, non-clashing points in the 3D space of the bar. The method returns null on error, for example if given an erroneous index. The number of rebars in the group can be requested with GetNumberOfRebars().

[Docs](https://developer.tekla.com/topic/en/18/43/8fd2e21a-3d2f-6211-92ba-9d6e652ac100)

#### `GetSolid(...)`

Method for getting the solid information of the reinforcement.

[Docs](https://developer.tekla.com/topic/en/18/43/6717d119-97da-5554-f359-49ee55e65a63)

#### `GetStringReportProperties(...)`

Retrieves multiple string report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/5ff72da1-7382-f5d2-eb1d-c99b46239d42)

#### `GetStringUserProperties(...)`

Retrieves all string properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/67120e84-c820-9d52-12ae-c167c595842c)

#### `GetUserProperty(String, Double.)(...)`

Retrieves a double property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/17cc8074-7955-32b5-2b4b-b169c45d4ee0)

#### `GetUserProperty(String, Int32.)(...)`

Retrieves an integer property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/d9364c1d-5b57-6a31-4c13-01124058cfb2)

#### `GetUserProperty(String, String.)(...)`

Retrieves a string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/214a886c-c18c-9b66-d7c9-fe84260962f2)

#### `Insert(...)`

Inserts the single rebar into the model database. All the attributes must be set.

[Docs](https://developer.tekla.com/topic/en/18/43/2a34b54a-4157-a1f0-1813-7982b14b5bfa)

#### `IsGeometryValid(...)`

Tells whether the geometry of a reinforcement object is valid or not.

[Docs](https://developer.tekla.com/topic/en/18/43/5699f6a6-4e39-b784-d84f-45eb7b1d0e37)

#### `Modify(...)`

Modifies the existing single rebar in the model database to match the current one.

[Docs](https://developer.tekla.com/topic/en/18/43/35feafff-559c-71c6-0960-6854c7d0ef23)

#### `Select(...)`

Selects a single rebar from the model database. The reinforcement identifier must be set.

[Docs](https://developer.tekla.com/topic/en/18/43/234da88f-d0f0-47b0-1e8f-24f9555ad7e7)

#### `SetCustomObjectType(...)`

Sets a custom object type name for an existing object.

[Docs](https://developer.tekla.com/topic/en/18/43/3d6fb91b-48a9-35ca-d498-526514344089)

#### `SetDynamicStringProperty(...)`

Sets a dynamic string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/4dab0254-09e5-5e61-c28b-1c12afab71a0)

#### `SetLabel(...)`

Sets a label for an object when a new instance is created, this method must be called before Insert. The label is used in plug-ins for identifying the changed object in modification.

[Docs](https://developer.tekla.com/topic/en/18/43/3efcfc44-613a-e66b-9ebd-9f86df4f4036)

#### `SetPhase(...)`

Sets the phase of the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/ed80abcc-0ff7-d081-2229-e651f858a727)

#### `SetUserProperties(...)`

Sets multiple properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/8659da37-d571-f03f-c666-5c9fa3b113f5)

#### `SetUserProperty(String, Double)(...)`

Sets a double property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/5c585b8b-5c2e-7b75-2242-758f17429396)

#### `SetUserProperty(String, Int32)(...)`

Sets an integer property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/dd5b828d-3a87-bb58-5ac9-e1c3b4c2fe4b)

#### `SetUserProperty(String, String)(...)`

Sets a string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/80bf0348-b651-3d4a-862b-49c85bc924ab)

### Properties
- `Class` (object, get/set) — Gets or sets the class of the reinforcement. The class is used to group reinforcements.
- `EndHook` (object, get/set) — The hook at the end of the reinforcing bar.
- `EndPointOffsetType` (object, get/set) — Gets or sets the type of the end point offset. The options are: OFFSET_TYPE_LEG_LENGTHOFFSET_TYPE_COVER_THICKNESS
- `EndPointOffsetValue` (object, get/set) — Gets or sets the concrete cover thickness or leg length at the second end of the bar.
- `Father` (object, get/set) — Gets or sets the father object of the reinforcement; the model object instance to operate on.
- `FromPlaneOffset` (object, get/set) — Gets or sets the offset value from the part surface.
- `Grade` (object, get/set) — Gets or sets the steel grade of the reinforcing bar. The grade indicates the strength of the steel used in reinforcing bars. It can also indicate other factors, such as the weldability or surface deformations of the bar.
- `Identifier` (object, get/set) — The identifier of the object.
- `InputPointDeformingState` (object, get/set) — Gets or sets the reinforcement input point deforming state.
- `IsUpToDate` (object, get/set) — Gets if the object does not have a modification which is not shared.
- `ModificationTime` (object, get/set) — Gets latest time of the object was modified or created.
- `Name` (object, get/set) — Gets or sets the name of the reinforcement.
- `NumberingSeries` (object, get/set) — Gets or sets the numbering series of the reinforcement.
- `OnPlaneOffsets` (object, get/set) — Gets or sets the double offset value for each leg on the same plane as the bar.
- `Polygon` (object, get/set) — The polygon of the reinforcing bar.
- `RadiusValues` (object, get/set) — Gets or sets the radius value(s) of the bends in the bar.
- `Size` (object, get/set) — The size of the reinforcement.
- `StartHook` (object, get/set) — The hook at the beginning of the reinforcing bar.
- `StartPointOffsetType` (object, get/set) — Gets or sets the type of the start point offset is either OFFSET_TYPE_LEG_LENGTH or OFFSET_TYPE_COVER_THICKNESS.
- `StartPointOffsetValue` (object, get/set) — Gets or sets the concrete cover thickness or leg length at the first end of the bar.

## Solid (class)

The Solid class represents the physical object in the model created by a part instance. A solid instance can be used to query the actual geometry of the part and intersect that geometry with, for example, lines and planes.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/7d939b20-a22f-7370-299b-a5a9fee8e54e)

### Methods
#### `GetAllIntersectionPoints(...)`

Used to get all the intersection points between the solid and a plane. Does not arrange the points into polygons, thus a lot faster.

[Docs](https://developer.tekla.com/topic/en/18/43/75e5f4f1-9b46-fce4-7cab-b7033817cc3e)

#### `GetCutPart(...)`

Returns a shell enumerator for the solid th at results cutting this solid with the given solid.

[Docs](https://developer.tekla.com/topic/en/18/43/6df0f96a-ce73-cec1-6320-f3b7ddeae3cf)

#### `GetEdgeEnumerator(...)`

Returns a new edge enumerator in the current plane.

[Docs](https://developer.tekla.com/topic/en/18/43/222fbb93-f13f-f7af-0d51-a8b0018fe12d)

#### `GetFaceEnumerator(...)`

Returns a new face enumerator in the current plane.

[Docs](https://developer.tekla.com/topic/en/18/43/a760c670-0718-ebd9-2535-86c4df113e2c)

#### `Intersect(LineSegment)(...)`

Returns a list of line - solid intersection points.

[Docs](https://developer.tekla.com/topic/en/18/43/b60a7604-e5c4-cf74-77e0-34139eec5d9a)

#### `Intersect(Point, Point)(...)`

Returns a list of line - solid intersection points.

[Docs](https://developer.tekla.com/topic/en/18/43/829d834d-424a-034e-c903-9cccf1759304)

#### `Intersect(Point, Point, Point)(...)`

Returns a list of plane - solid intersection points. The first item of the list contains a list of the outmost intersection polygon and the rest of the items (if there are any) inner polygons.

[Docs](https://developer.tekla.com/topic/en/18/43/89e188c7-dedb-4439-db95-78e3a442bebf)

#### `IntersectAllFaces(...)`

Returns an enumerator for an array list of lists of plane - solid intersection points from all intersecting faces. The first item of one list contains points of the outmost intersection polygon and then the inner polygons (if there are any).

[Docs](https://developer.tekla.com/topic/en/18/43/223e9ea0-6f38-e3b0-f323-574838c399aa)

#### `IsValid(...)`

Returns if the solid is valid.

[Docs](https://developer.tekla.com/topic/en/18/43/4b7bd339-892a-5872-8156-d761323779cf)

### Properties
- `MaximumPoint` (object, get/set) — Gets the maximum axis-aligned point of the solid in the current plane.
- `MinimumPoint` (object, get/set) — Gets the minimum axis-aligned point of the solid in the current plane.

## Solid.SolidCreationTypeEnum (enum)

The creation type of the solid.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/91495004-3272-05f8-52b1-cdc3d386ea19)

### Values
- `RAW` = `0` — The solid is created without any boolean operations.
- `FITTED` = `1` — The solid is created with fittings.
- `NORMAL` = `2` — The solid is created including all cutting and fitting operations.
- `HIGH_ACCURACY` = `3` — The solid is created as normal with an exact profile cross section.
- `PLANECUTTED` = `4` — The solid is created with fittings and cut planes.
- `NORMAL_WITHOUT_EDGECHAMFERS` = `5` — The solid is created including all cutting and fitting operations without edge chamfers.
- `NORMAL_WITHOUT_WELDPREPS` = `6` — The solid is created including all cutting and fitting operations without weld prepping.

## SpiralBeam (class)

A class for the spiral beam part.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/2b829f38-d9b1-d5ea-f9b5-68801bbfdcd3)

### Constructors
- `SpiralBeam(...)` — Initializes a new instance of the SpiralBeam class.
- `SpiralBeam(...)` — Initializes a new instance of the SpiralBeam class with the given parameters. Rotation axis direction is current positive Z direction.
- `SpiralBeam(...)` — Initializes a new instance of the SpiralBeam class with the given parameters.
- `SpiralBeam(...)` — Initializes a new instance of the SpiralBeam class with the given parameters.

### Methods
#### `CompareTo(Object)(...)`

Compares Identifiers of model objects.

[Docs](https://developer.tekla.com/topic/en/18/43/79cca356-0c8a-e8fb-463a-34ad4141bec7)

#### `CompareTo(Part)(...)`

Compares the instantiated part with another one.

[Docs](https://developer.tekla.com/topic/en/18/43/e8632c0d-89cf-9aab-6532-f62f9873067b)

#### `Delete(...)`

Deletes the spiral beam with this instance identifier from the model database.

[Docs](https://developer.tekla.com/topic/en/18/43/334d6965-9063-474e-a941-f266d0456e6b)

#### `Equals(...)`

Check if Identifiers of model objects are same.

[Docs](https://developer.tekla.com/topic/en/18/43/e4324265-2490-00de-4139-63a8f7acb492)

#### `GetAllReportProperties(...)`

Retrieves all properties for the model object in one hashtable. Type for the returned value must be checked using type casting.

[Docs](https://developer.tekla.com/topic/en/18/43/0708f404-ce77-28e1-4ebe-fa906f68bff8)

#### `GetAllUserProperties(...)`

Retrieves all properties for the model object in one hashtable. Type for the returned value must be checked using type casting.

[Docs](https://developer.tekla.com/topic/en/18/43/63b853a0-0af5-af9c-3ce7-05299664d7f8)

#### `GetAssembly(...)`

Returns the assembly that the part belongs to.

[Docs](https://developer.tekla.com/topic/en/18/43/bda18562-45a1-1355-42ed-6044af0b18cb)

#### `GetBolts(...)`

Returns an enumerator of all the connected bolts.

[Docs](https://developer.tekla.com/topic/en/18/43/dbc6b475-7461-7ec4-e8ec-7b7d3d8ff429)

#### `GetBooleans(...)`

Returns an enumerator of all the connected boolean objects.

[Docs](https://developer.tekla.com/topic/en/18/43/ae180e43-88e2-d759-0ae0-567d52681a2a)

#### `GetCenterLine(...)`

Returns the center line for the given part.

[Docs](https://developer.tekla.com/topic/en/18/43/bdc886b8-af7a-722c-dcb8-fddb2f250a83)

#### `GetChildren(...)`

Returns an enumerator of all the children model objects.

[Docs](https://developer.tekla.com/topic/en/18/43/ce51ec40-310c-6067-a89f-f8d029aab747)

#### `GetComponents(...)`

Returns an enumerator of all the connected components, connections, seams and details inherited from the base component.

[Docs](https://developer.tekla.com/topic/en/18/43/ed57941c-2cd8-3b78-6a6e-848b8a920298)

#### `GetCoordinateSystem(...)`

Returns the coordinate system for the given model object.

[Docs](https://developer.tekla.com/topic/en/18/43/8adfe9cd-f6e5-6474-1297-33c6270a7b0a)

#### `GetCustomObjectType(...)`

Gets custom object type name from an existing object.

[Docs](https://developer.tekla.com/topic/en/18/43/d3be2ba3-8533-7165-f2d3-a5aa9df556b8)

#### `GetDSTVCoordinateSystem(...)`

Get DSTV coordinate system.

[Docs](https://developer.tekla.com/topic/en/18/43/9e70005f-c701-ab99-8fa0-4fce47b1ab05)

#### `GetDoubleReportProperties(...)`

Retrieves multiple double report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/f86f6c96-f738-49ca-16ba-f5634e862ba2)

#### `GetDoubleUserProperties(...)`

Retrieves all double properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/6ead7a63-5f76-0631-d480-d46c27e882aa)

#### `GetDynamicStringProperty(...)`

Gets a dynamic string property from the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/415e86d9-7de6-ea53-ce68-20b3e67b8655)

#### `GetFatherComponent(...)`

Returns the father component of the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/a2c28f3f-52ff-50a8-1b32-10d18db5c31d)

#### `GetHierarchicObjects(...)`

Returns an enumerator of all the connected hierarchic objects.

[Docs](https://developer.tekla.com/topic/en/18/43/cc228ea0-267b-7d90-7441-c9efc1779b08)

#### `GetIntegerReportProperties(...)`

Retrieves multiple integer report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/2623a9c0-2a9c-e233-9daa-59f92ed51445)

#### `GetIntegerUserProperties(...)`

Retrieves all integer properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/1b3c6f06-bc25-a54e-2a88-b60dc4a3c85e)

#### `GetPartMark(...)`

Returns the part mark.

[Docs](https://developer.tekla.com/topic/en/18/43/9978ba28-6304-714d-815c-fe4a6bb77076)

#### `GetPhase(...)`

Retrieves the phase of the model object (the phase number, the phase name, the phase comment and whether the phase is the current one or not).

[Docs](https://developer.tekla.com/topic/en/18/43/310f8b7a-c120-753a-35c9-f7c73a9806c9)

#### `GetPours(...)`

Returns an enumerator of all the pours that the part belongs to.

[Docs](https://developer.tekla.com/topic/en/18/43/1cc67ebb-329a-4675-7857-af663602d21d)

#### `GetReferenceLine(...)`

Returns the reference line for the given part.

[Docs](https://developer.tekla.com/topic/en/18/43/3e0a42c9-f10c-4225-bcd6-2b9b61ecc743)

#### `GetReinforcements(...)`

Returns an enumerator of all the connected reinforcements.

[Docs](https://developer.tekla.com/topic/en/18/43/510c913a-0653-6777-1841-c4f7df4fee47)

#### `GetReportProperty(String, Double.)(...)`

Retrieves a double property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/1c05aa56-5f42-be91-a59e-8d59a5549f45)

#### `GetReportProperty(String, Int32.)(...)`

Retrieves an integer property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/ce8e4ca0-3750-3c9b-5ca8-27e48b45e0bd)

#### `GetReportProperty(String, String.)(...)`

Retrieves a string property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/70f212bb-ee70-e9e9-7ab6-d6a9d46f8de6)

#### `GetSolid(FormingStates)(...)`

Returns the solid of the part.

[Docs](https://developer.tekla.com/topic/en/18/43/1e4d454b-d3a0-4219-b4e2-7f661b402c4d)

#### `GetSolid(Solid.SolidCreationTypeEnum)(...)`

Returns the solid of the part.

[Docs](https://developer.tekla.com/topic/en/18/43/25f58541-37fe-c1ca-f384-065ecbe38e55)

#### `GetSolid.(...)`

Returns the solid of the part.

[Docs](https://developer.tekla.com/topic/en/18/43/7bd70e97-7c8b-823c-c7e1-da2ebe3dadaf)

#### `GetStringReportProperties(...)`

Retrieves multiple string report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/5ff72da1-7382-f5d2-eb1d-c99b46239d42)

#### `GetStringUserProperties(...)`

Retrieves all string properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/67120e84-c820-9d52-12ae-c167c595842c)

#### `GetSurfaceObjects(...)`

Returns an enumerator of all the connected surface objects.

[Docs](https://developer.tekla.com/topic/en/18/43/36268c4e-9006-1c9d-57bd-331f907da6ce)

#### `GetSurfaceTreatments(...)`

Returns an enumerator of all the connected surface treatments.

[Docs](https://developer.tekla.com/topic/en/18/43/26c1c1a5-e5db-9850-f739-c58bc05c963e)

#### `GetUserProperty(String, Double.)(...)`

Retrieves a double property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/17cc8074-7955-32b5-2b4b-b169c45d4ee0)

#### `GetUserProperty(String, Int32.)(...)`

Retrieves an integer property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/d9364c1d-5b57-6a31-4c13-01124058cfb2)

#### `GetUserProperty(String, String.)(...)`

Retrieves a string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/214a886c-c18c-9b66-d7c9-fe84260962f2)

#### `GetWelds(...)`

Returns an enumerator of all the connected welds.

[Docs](https://developer.tekla.com/topic/en/18/43/722de753-413b-f810-f09c-07388feb0afa)

#### `Insert(...)`

Inserts the spiral beam into the model database. All the attributes must be set.

[Docs](https://developer.tekla.com/topic/en/18/43/71afc10f-d107-d193-7692-d22b4e61f055)

#### `Modify(...)`

Modifies the spiral beam object values in the database.

[Docs](https://developer.tekla.com/topic/en/18/43/67a6ee1c-3c7c-e844-f1c9-25383c00eff7)

#### `Select(...)`

Selects a spiral beam object from the database.

[Docs](https://developer.tekla.com/topic/en/18/43/dafa4073-0be8-429b-f41b-a1344d106d3c)

#### `SetCustomObjectType(...)`

Sets a custom object type name for an existing object.

[Docs](https://developer.tekla.com/topic/en/18/43/3d6fb91b-48a9-35ca-d498-526514344089)

#### `SetDynamicStringProperty(...)`

Sets a dynamic string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/4dab0254-09e5-5e61-c28b-1c12afab71a0)

#### `SetLabel(...)`

Sets a label for an object when a new instance is created, this method must be called before Insert. The label is used in plug-ins for identifying the changed object in modification.

[Docs](https://developer.tekla.com/topic/en/18/43/3efcfc44-613a-e66b-9ebd-9f86df4f4036)

#### `SetPhase(...)`

Sets the phase of the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/ed80abcc-0ff7-d081-2229-e651f858a727)

#### `SetUserProperties(...)`

Sets multiple properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/8659da37-d571-f03f-c666-5c9fa3b113f5)

#### `SetUserProperty(String, Double)(...)`

Sets a double property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/5c585b8b-5c2e-7b75-2242-758f17429396)

#### `SetUserProperty(String, Int32)(...)`

Sets an integer property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/dd5b828d-3a87-bb58-5ac9-e1c3b4c2fe4b)

#### `SetUserProperty(String, String)(...)`

Sets a string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/80bf0348-b651-3d4a-862b-49c85bc924ab)

### Properties
- `AssemblyNumber` (object, get/set) — Gets or sets the assembly number. Defines the numbering in the assembly sense.
- `CastUnitType` (object, get/set) — Gets or sets the cast unit type of the part.
- `Class` (object, get/set) — Gets or sets the class of the part.
- `DeformingData` (object, get/set) — Gets or sets the deforming data of the part.
- `EndPoint` (object, get/set) — Gets the spiral beam end point.
- `Finish` (object, get/set) — Gets or sets the finish of the part.
- `Identifier` (object, get/set) — The identifier of the object.
- `IsUpToDate` (object, get/set) — Gets if the object does not have a modification which is not shared.
- `Material` (object, get/set) — Gets or sets the material the part is made of.
- `ModificationTime` (object, get/set) — Gets latest time of the object was modified or created.
- `Name` (object, get/set) — Gets or sets the name of the part.
- `PartNumber` (object, get/set) — Gets or sets the part number. Defines the numbering in the part sense.
- `Position` (object, get/set) — Gets or sets the part position. Defines the way the part is positioned in the model.
- `PourPhase` (object, get/set) — Gets or sets the pour phase of the part.
- `Profile` (object, get/set) — Gets or sets the profile of the part.
- `RotationAngle` (object, get/set) — Gets or sets the rotation of the spiral beam in degrees. Each range of 360.0 degrees adds a full rotation.
- `RotationAxisBasePoint` (object, get/set) — Gets or sets the first definition point of the spiral beam rotation axis.
- `RotationAxisDirection` (object, get/set) — Gets the direction of the spiral beam rotation axis.
- `RotationAxisUpPoint` (object, get/set) — Gets or sets the second definition point of the spiral beam rotation axis.
- `RotationCenterPoint` (object, get/set) — Gets the geometric rotation center point at the start of the spiral beam.
- `StartPoint` (object, get/set) — Gets or sets the start point of the spiral beam.
- `TotalRise` (object, get/set) — Gets or sets the difference between the spiral beam start and end point in the direction of the rotation axis.
- `TwistAngleEnd` (object, get/set) — Gets or sets the twist angle in degrees at the end of the spiral beam.
- `TwistAngleStart` (object, get/set) — Gets or sets the twist angle in degrees at the start of the spiral beam.

## SpiralBeam.ErrorStatus (enum)

Define possible statuses for geometry creation/modification commands.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/96dd1b8d-e2ec-e21a-a908-854a9748bd21)

### Values
- `DataMissing` = `0` — Status when some part data is missing.
- `DefinitionPointsTooClose` = `1` — Status when definition points are too close.
- `DefinitionPointsCannotBeAligned` = `2` — Status when definition points are on line with sweep direction.
- `ZeroTotalRiseWithMore360Degrees` = `3` — Status when total rise is zero and rotation angle more than 360 degrees.
- `RotationAngleIs0Degrees` = `4` — Status when rotation angle is 0 degrees.
- `InvalidGeometry` = `5` — Status when helix has invalid geometry.

## SpiralBeamDataException (class)

The SpiralBeamDataException class represents an error that occurred during the spiral beam creation or modification.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/031dfd58-7798-4ded-5a3a-3e35e02069d8)

### Constructors
- `SpiralBeamDataException(...)` — Initializes a new instance of the SpiralBeamDataException class with spiral beam error message based on status.

## StrandUnbondingData (class)

The StrandUnbondingData class represents the unbonding of a rebar strand.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/b532c1be-5eec-7820-26ff-12e6061d7c05)

### Constructors
- `StrandUnbondingData(...)` — Initializes a new strand unbonding data instance with empty attributes.

### Properties
- `FromEnd` (object, get/set) — The unbonding from the end of the strand.
- `FromStart` (object, get/set) — The unbonding from the start of the strand.
- `MiddleToEnd` (object, get/set) — The unbonding from the middle of the strand to the end of the strand.
- `MiddleToStart` (object, get/set) — The unbonding from the middle of the strand to the start of the strand.
- `StrandIndex` (object, get/set) — The index of the strand.

## SurfaceObject (class)

The SurfaceObject class represents a surface object in the model. Currently this class only support dynamic surfaces whose geometries are determined by the parts or pours they associate to. It is not allowed to modify the geometry of a dynamic surface on Open API. The polymesh setter of this class throws .

[Vendor docs](https://developer.tekla.com/topic/en/18/43/03dc457c-9884-1f10-6a99-390c401463b7)

### Constructors
- `SurfaceObject(...)` — Initializes a new instance of the SurfaceObject class.

### Methods
#### `CompareTo(...)`

Compares Identifiers of model objects.

[Docs](https://developer.tekla.com/topic/en/18/43/79cca356-0c8a-e8fb-463a-34ad4141bec7)

#### `Delete(...)`

Deletes the model object from the model database.

[Docs](https://developer.tekla.com/topic/en/18/43/67b92503-1679-fcb5-036a-503a581495a5)

#### `Equals(...)`

Check if Identifiers of model objects are same.

[Docs](https://developer.tekla.com/topic/en/18/43/e4324265-2490-00de-4139-63a8f7acb492)

#### `GetAllReportProperties(...)`

Retrieves all properties for the model object in one hashtable. Type for the returned value must be checked using type casting.

[Docs](https://developer.tekla.com/topic/en/18/43/0708f404-ce77-28e1-4ebe-fa906f68bff8)

#### `GetAllUserProperties(...)`

Retrieves all properties for the model object in one hashtable. Type for the returned value must be checked using type casting.

[Docs](https://developer.tekla.com/topic/en/18/43/63b853a0-0af5-af9c-3ce7-05299664d7f8)

#### `GetChildren(...)`

Returns an enumerator of all the children model objects.

[Docs](https://developer.tekla.com/topic/en/18/43/ce51ec40-310c-6067-a89f-f8d029aab747)

#### `GetCoordinateSystem(...)`

Returns the coordinate system for the given model object.

[Docs](https://developer.tekla.com/topic/en/18/43/8adfe9cd-f6e5-6474-1297-33c6270a7b0a)

#### `GetCustomObjectType(...)`

Gets custom object type name from an existing object.

[Docs](https://developer.tekla.com/topic/en/18/43/d3be2ba3-8533-7165-f2d3-a5aa9df556b8)

#### `GetDoubleReportProperties(...)`

Retrieves multiple double report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/f86f6c96-f738-49ca-16ba-f5634e862ba2)

#### `GetDoubleUserProperties(...)`

Retrieves all double properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/6ead7a63-5f76-0631-d480-d46c27e882aa)

#### `GetDynamicStringProperty(...)`

Gets a dynamic string property from the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/415e86d9-7de6-ea53-ce68-20b3e67b8655)

#### `GetFatherComponent(...)`

Returns the father component of the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/a2c28f3f-52ff-50a8-1b32-10d18db5c31d)

#### `GetHierarchicObjects(...)`

Returns an enumerator of all the connected hierarchic objects.

[Docs](https://developer.tekla.com/topic/en/18/43/cc228ea0-267b-7d90-7441-c9efc1779b08)

#### `GetIntegerReportProperties(...)`

Retrieves multiple integer report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/2623a9c0-2a9c-e233-9daa-59f92ed51445)

#### `GetIntegerUserProperties(...)`

Retrieves all integer properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/1b3c6f06-bc25-a54e-2a88-b60dc4a3c85e)

#### `GetPhase(...)`

Retrieves the phase of the model object (the phase number, the phase name, the phase comment and whether the phase is the current one or not).

[Docs](https://developer.tekla.com/topic/en/18/43/310f8b7a-c120-753a-35c9-f7c73a9806c9)

#### `GetReportProperty(String, Double.)(...)`

Retrieves a double property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/1c05aa56-5f42-be91-a59e-8d59a5549f45)

#### `GetReportProperty(String, Int32.)(...)`

Retrieves an integer property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/ce8e4ca0-3750-3c9b-5ca8-27e48b45e0bd)

#### `GetReportProperty(String, String.)(...)`

Retrieves a string property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/70f212bb-ee70-e9e9-7ab6-d6a9d46f8de6)

#### `GetStringReportProperties(...)`

Retrieves multiple string report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/5ff72da1-7382-f5d2-eb1d-c99b46239d42)

#### `GetStringUserProperties(...)`

Retrieves all string properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/67120e84-c820-9d52-12ae-c167c595842c)

#### `GetUserProperty(String, Double.)(...)`

Retrieves a double property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/17cc8074-7955-32b5-2b4b-b169c45d4ee0)

#### `GetUserProperty(String, Int32.)(...)`

Retrieves an integer property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/d9364c1d-5b57-6a31-4c13-01124058cfb2)

#### `GetUserProperty(String, String.)(...)`

Retrieves a string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/214a886c-c18c-9b66-d7c9-fe84260962f2)

#### `Insert(...)`

Inserts a new model object. The geometry of the object needs to be set by using the public Polymesh function, before calling insert.

[Docs](https://developer.tekla.com/topic/en/18/43/a9b5ac5b-0e99-2a05-8ff7-002435eecfcb)

#### `Modify(...)`

Modifies the existing model object in the model database to match the current one.

[Docs](https://developer.tekla.com/topic/en/18/43/6f2a4e49-f428-c610-efe3-f59020ffc5cc)

#### `Select(...)`

Selects the model object from the model by the identifier of this instance.

[Docs](https://developer.tekla.com/topic/en/18/43/5f7ce37b-9bff-4264-3a44-1b6533622035)

#### `SetCustomObjectType(...)`

Sets a custom object type name for an existing object.

[Docs](https://developer.tekla.com/topic/en/18/43/3d6fb91b-48a9-35ca-d498-526514344089)

#### `SetDynamicStringProperty(...)`

Sets a dynamic string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/4dab0254-09e5-5e61-c28b-1c12afab71a0)

#### `SetLabel(...)`

Sets a label for an object when a new instance is created, this method must be called before Insert. The label is used in plug-ins for identifying the changed object in modification.

[Docs](https://developer.tekla.com/topic/en/18/43/3efcfc44-613a-e66b-9ebd-9f86df4f4036)

#### `SetPhase(...)`

Sets the phase of the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/ed80abcc-0ff7-d081-2229-e651f858a727)

#### `SetUserProperties(...)`

Sets multiple properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/8659da37-d571-f03f-c666-5c9fa3b113f5)

#### `SetUserProperty(String, Double)(...)`

Sets a double property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/5c585b8b-5c2e-7b75-2242-758f17429396)

#### `SetUserProperty(String, Int32)(...)`

Sets an integer property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/dd5b828d-3a87-bb58-5ac9-e1c3b4c2fe4b)

#### `SetUserProperty(String, String)(...)`

Sets a string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/80bf0348-b651-3d4a-862b-49c85bc924ab)

### Properties
- `Class` (object, get/set) — Gets or sets the class of the surface object.
- `CreateHoles` (object, get/set) — Gets or sets a value indicating whether CreateHoles parameter of the surface object.
- `Father` (object, get/set) — Gets or sets the father object of the surface object. The father can be set only before the Insert(), otherwise exception is thrown.
- `Identifier` (object, get/set) — The identifier of the object.
- `IsUpToDate` (object, get/set) — Gets if the object does not have a modification which is not shared.
- `ModificationTime` (object, get/set) — Gets latest time of the object was modified or created.
- `Name` (object, get/set) — Gets or sets the name of the surface object.
- `Polymesh` (object, get/set) — Gets or sets the surface geometry brep.
- `Type` (object, get/set) — Gets or sets the type of the surface object.

## SurfaceTreatment (class)

The SurfaceTreatment class defines a layer above a building element defining what a surface looks like. The surface could for example have a tiled look.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/6726c2c8-455a-71c3-8c81-3443c27018f9)

### Constructors
- `SurfaceTreatment(...)` — Creates a new surface treatment instance.

### Methods
#### `CompareTo(...)`

Compares Identifiers of model objects.

[Docs](https://developer.tekla.com/topic/en/18/43/79cca356-0c8a-e8fb-463a-34ad4141bec7)

#### `Delete(...)`

Deletes the surface treatment with the given identifier from the database.

[Docs](https://developer.tekla.com/topic/en/18/43/edbaee89-ed6e-4954-4451-bc2856c51206)

#### `Equals(...)`

Check if Identifiers of model objects are same.

[Docs](https://developer.tekla.com/topic/en/18/43/e4324265-2490-00de-4139-63a8f7acb492)

#### `GetAllReportProperties(...)`

Retrieves all properties for the model object in one hashtable. Type for the returned value must be checked using type casting.

[Docs](https://developer.tekla.com/topic/en/18/43/0708f404-ce77-28e1-4ebe-fa906f68bff8)

#### `GetAllUserProperties(...)`

Retrieves all properties for the model object in one hashtable. Type for the returned value must be checked using type casting.

[Docs](https://developer.tekla.com/topic/en/18/43/63b853a0-0af5-af9c-3ce7-05299664d7f8)

#### `GetChildren(...)`

Returns an enumerator of all the children model objects.

[Docs](https://developer.tekla.com/topic/en/18/43/ce51ec40-310c-6067-a89f-f8d029aab747)

#### `GetCoordinateSystem(...)`

Returns the coordinate system for the given model object.

[Docs](https://developer.tekla.com/topic/en/18/43/8adfe9cd-f6e5-6474-1297-33c6270a7b0a)

#### `GetCustomObjectType(...)`

Gets custom object type name from an existing object.

[Docs](https://developer.tekla.com/topic/en/18/43/d3be2ba3-8533-7165-f2d3-a5aa9df556b8)

#### `GetDoubleReportProperties(...)`

Retrieves multiple double report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/f86f6c96-f738-49ca-16ba-f5634e862ba2)

#### `GetDoubleUserProperties(...)`

Retrieves all double properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/6ead7a63-5f76-0631-d480-d46c27e882aa)

#### `GetDynamicStringProperty(...)`

Gets a dynamic string property from the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/415e86d9-7de6-ea53-ce68-20b3e67b8655)

#### `GetFatherComponent(...)`

Returns the father component of the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/a2c28f3f-52ff-50a8-1b32-10d18db5c31d)

#### `GetHierarchicObjects(...)`

Returns an enumerator of all the connected hierarchic objects.

[Docs](https://developer.tekla.com/topic/en/18/43/cc228ea0-267b-7d90-7441-c9efc1779b08)

#### `GetIntegerReportProperties(...)`

Retrieves multiple integer report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/2623a9c0-2a9c-e233-9daa-59f92ed51445)

#### `GetIntegerUserProperties(...)`

Retrieves all integer properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/1b3c6f06-bc25-a54e-2a88-b60dc4a3c85e)

#### `GetPhase(...)`

Retrieves the phase of the model object (the phase number, the phase name, the phase comment and whether the phase is the current one or not).

[Docs](https://developer.tekla.com/topic/en/18/43/310f8b7a-c120-753a-35c9-f7c73a9806c9)

#### `GetReportProperty(String, Double.)(...)`

Retrieves a double property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/1c05aa56-5f42-be91-a59e-8d59a5549f45)

#### `GetReportProperty(String, Int32.)(...)`

Retrieves an integer property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/ce8e4ca0-3750-3c9b-5ca8-27e48b45e0bd)

#### `GetReportProperty(String, String.)(...)`

Retrieves a string property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/70f212bb-ee70-e9e9-7ab6-d6a9d46f8de6)

#### `GetStringReportProperties(...)`

Retrieves multiple string report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/5ff72da1-7382-f5d2-eb1d-c99b46239d42)

#### `GetStringUserProperties(...)`

Retrieves all string properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/67120e84-c820-9d52-12ae-c167c595842c)

#### `GetUserProperty(String, Double.)(...)`

Retrieves a double property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/17cc8074-7955-32b5-2b4b-b169c45d4ee0)

#### `GetUserProperty(String, Int32.)(...)`

Retrieves an integer property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/d9364c1d-5b57-6a31-4c13-01124058cfb2)

#### `GetUserProperty(String, String.)(...)`

Retrieves a string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/214a886c-c18c-9b66-d7c9-fe84260962f2)

#### `Insert(...)`

Inserts the surface treatment to the database.

[Docs](https://developer.tekla.com/topic/en/18/43/3f077ca4-e267-e155-3b9f-102a0c17470c)

#### `Modify(...)`

Modifies the surface treatment with the given identifier in the database to match the current one.

[Docs](https://developer.tekla.com/topic/en/18/43/0148aa37-6ae6-fc45-61de-0a9b1f79c8b4)

#### `Select(...)`

Selects the surface treatment with the given identifier.

[Docs](https://developer.tekla.com/topic/en/18/43/b0c301da-c771-f7ce-2c64-0ebfceda018b)

#### `SetCustomObjectType(...)`

Sets a custom object type name for an existing object.

[Docs](https://developer.tekla.com/topic/en/18/43/3d6fb91b-48a9-35ca-d498-526514344089)

#### `SetDynamicStringProperty(...)`

Sets a dynamic string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/4dab0254-09e5-5e61-c28b-1c12afab71a0)

#### `SetLabel(...)`

Sets a label for an object when a new instance is created, this method must be called before Insert. The label is used in plug-ins for identifying the changed object in modification.

[Docs](https://developer.tekla.com/topic/en/18/43/3efcfc44-613a-e66b-9ebd-9f86df4f4036)

#### `SetPhase(...)`

Sets the phase of the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/ed80abcc-0ff7-d081-2229-e651f858a727)

#### `SetUserProperties(...)`

Sets multiple properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/8659da37-d571-f03f-c666-5c9fa3b113f5)

#### `SetUserProperty(String, Double)(...)`

Sets a double property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/5c585b8b-5c2e-7b75-2242-758f17429396)

#### `SetUserProperty(String, Int32)(...)`

Sets an integer property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/dd5b828d-3a87-bb58-5ac9-e1c3b4c2fe4b)

#### `SetUserProperty(String, String)(...)`

Sets a string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/80bf0348-b651-3d4a-862b-49c85bc924ab)

### Properties
- `Class` (object, get/set) — The class of the surface treatment.
- `Color` (object, get/set) — The color of the surfacing.
- `CutByFatherBooleans` (object, get/set) — Defines if the part is cut by the father's cuts.
- `EndPoint` (object, get/set) — Determines the direction the surfacing is filled in. The direction is from the start point towards the end point.
- `Father` (object, get/set) — Defines the father part for the surface treatment: the part that the surface treatment is created on.
- `Identifier` (object, get/set) — The identifier of the object.
- `IsUpToDate` (object, get/set) — Gets if the object does not have a modification which is not shared.
- `Material` (object, get/set) — Defines the material for the surface treatment.
- `ModificationTime` (object, get/set) — Gets latest time of the object was modified or created.
- `Name` (object, get/set) — The name of the surface treatment.
- `Polygon` (object, get/set) — Defines the area for the surface treatment.
- `Position` (object, get/set) — Defines the position for the surface treatment.
- `StartPoint` (object, get/set) — Determines the origin of the surfacing. The surfacing is then filled out from the start point towards the end point.
- `Thickness` (object, get/set) — Defines how thick the surface treatment is.
- `Type` (object, get/set) — The type of the surfacing.
- `TypeName` (object, get/set) — The name of the surface treatment type. The valid names are defined in the product_finishes.dat file.

## SurfaceTreatment.SurfaceColorEnum (enum)

The different surfacing colors.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/220357a1-749a-c75a-62cc-651b54fe5e1f)

### Values
- `WHITE` = `1` — The white surface color.
- `RED` = `2` — The red surface color.
- `GREEN` = `3` — The green surface color.
- `BLUE` = `4` — The blue surface color.
- `CYAN` = `5` — The cyan surface color.
- `YELLOW` = `6` — The yellow surface color.
- `MAGENTA` = `7` — The magenta surface color.

## SurfaceTreatment.SurfaceTypeEnum (enum)

The different surfacing types.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/3f55bbef-2e2a-3aac-7081-a9e9fd68a52e)

### Values
- `CONCRETE_FINISH` = `1` — The concrete finish surface type.
- `SPECIAL_MIX` = `2` — The special mix surface type.
- `TILE_SURFACE` = `3` — The tile surface type.
- `STEEL_FINISH` = `4` — The steel finish surface type.

## Task (class)

The Task class defines a single building site task. It may contain parts, assemblies or other tasks. Tasks may have a hierarchy between them i.e. there are other tasks as subtasks for a parent task. Tasks may also depend on each other, have resources assigned to them or have a single worktype.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/c36a21f1-71b0-e3ab-8db5-2f10414db8bb)

### Constructors
- `Task(...)` — Creates a new task instance.
- `Task(...)` — Creates a task instance with a known identifier. Select the task after the creation.

### Methods
#### `AddObjectsToTask(...)`

Adds objects to the task.

[Docs](https://developer.tekla.com/topic/en/18/43/d2f3929b-a826-934f-1336-10d85ce949ef)

#### `CompareTo(...)`

Compares Identifiers of model objects.

[Docs](https://developer.tekla.com/topic/en/18/43/79cca356-0c8a-e8fb-463a-34ad4141bec7)

#### `Delete(...)`

Deletes the task instance from the model. The identifier must be set.

[Docs](https://developer.tekla.com/topic/en/18/43/7d60cc20-1b40-93f5-0ea1-fe82a172e1b8)

#### `Equals(...)`

Check if Identifiers of model objects are same.

[Docs](https://developer.tekla.com/topic/en/18/43/e4324265-2490-00de-4139-63a8f7acb492)

#### `GetAllReportProperties(...)`

Retrieves all properties for the model object in one hashtable. Type for the returned value must be checked using type casting.

[Docs](https://developer.tekla.com/topic/en/18/43/0708f404-ce77-28e1-4ebe-fa906f68bff8)

#### `GetAllTasksOfSelectedObjects(...)`

Returns an enumerator of all the tasks related to the selected objects.

[Docs](https://developer.tekla.com/topic/en/18/43/d1155055-20c0-fd93-5e98-c6a5a0511266)

#### `GetAllUserProperties(...)`

Retrieves all properties for the model object in one hashtable. Type for the returned value must be checked using type casting.

[Docs](https://developer.tekla.com/topic/en/18/43/63b853a0-0af5-af9c-3ce7-05299664d7f8)

#### `GetChildren(...)`

Returns an enumerator of all the children model objects.

[Docs](https://developer.tekla.com/topic/en/18/43/ce51ec40-310c-6067-a89f-f8d029aab747)

#### `GetCoordinateSystem(...)`

Returns the coordinate system for the given model object.

[Docs](https://developer.tekla.com/topic/en/18/43/8adfe9cd-f6e5-6474-1297-33c6270a7b0a)

#### `GetCustomObjectType(...)`

Gets custom object type name from an existing object.

[Docs](https://developer.tekla.com/topic/en/18/43/d3be2ba3-8533-7165-f2d3-a5aa9df556b8)

#### `GetDependencies(...)`

Returns an enumerator of all the task dependency objects where the task is involved.

[Docs](https://developer.tekla.com/topic/en/18/43/cf66446e-5bfc-df55-67d8-cb61054f9725)

#### `GetDoubleReportProperties(...)`

Retrieves multiple double report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/f86f6c96-f738-49ca-16ba-f5634e862ba2)

#### `GetDoubleUserProperties(...)`

Retrieves all double properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/6ead7a63-5f76-0631-d480-d46c27e882aa)

#### `GetDynamicStringProperty(...)`

Gets a dynamic string property from the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/415e86d9-7de6-ea53-ce68-20b3e67b8655)

#### `GetFatherComponent(...)`

Returns the father component of the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/a2c28f3f-52ff-50a8-1b32-10d18db5c31d)

#### `GetFathers(...)`

Returns an enumerator of all the task type fathers for the task object.

[Docs](https://developer.tekla.com/topic/en/18/43/9bb56a6a-ceee-e866-6061-c893ff40df93)

#### `GetHierarchicObjects(...)`

Returns an enumerator of all the connected hierarchic objects.

[Docs](https://developer.tekla.com/topic/en/18/43/cc228ea0-267b-7d90-7441-c9efc1779b08)

#### `GetIntegerReportProperties(...)`

Retrieves multiple integer report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/2623a9c0-2a9c-e233-9daa-59f92ed51445)

#### `GetIntegerUserProperties(...)`

Retrieves all integer properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/1b3c6f06-bc25-a54e-2a88-b60dc4a3c85e)

#### `GetPhase(...)`

Retrieves the phase of the model object (the phase number, the phase name, the phase comment and whether the phase is the current one or not).

[Docs](https://developer.tekla.com/topic/en/18/43/310f8b7a-c120-753a-35c9-f7c73a9806c9)

#### `GetReportProperty(String, Double.)(...)`

Retrieves a double property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/1c05aa56-5f42-be91-a59e-8d59a5549f45)

#### `GetReportProperty(String, Int32.)(...)`

Retrieves an integer property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/ce8e4ca0-3750-3c9b-5ca8-27e48b45e0bd)

#### `GetReportProperty(String, String.)(...)`

Retrieves a string property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/70f212bb-ee70-e9e9-7ab6-d6a9d46f8de6)

#### `GetStringReportProperties(...)`

Retrieves multiple string report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/5ff72da1-7382-f5d2-eb1d-c99b46239d42)

#### `GetStringUserProperties(...)`

Retrieves all string properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/67120e84-c820-9d52-12ae-c167c595842c)

#### `GetUserProperty(String, Double.)(...)`

Retrieves a double property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/17cc8074-7955-32b5-2b4b-b169c45d4ee0)

#### `GetUserProperty(String, Int32.)(...)`

Retrieves an integer property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/d9364c1d-5b57-6a31-4c13-01124058cfb2)

#### `GetUserProperty(String, String.)(...)`

Retrieves a string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/214a886c-c18c-9b66-d7c9-fe84260962f2)

#### `Insert(...)`

Inserts the task instance in the model.

[Docs](https://developer.tekla.com/topic/en/18/43/f920abce-e962-3943-fe36-d59462b35867)

#### `Modify(...)`

Modifies the task instance in the model. The identifier must be set.

[Docs](https://developer.tekla.com/topic/en/18/43/0f954b0c-b15f-981d-fb0f-a181706c9e6b)

#### `RemoveObjectsFromTask(...)`

Removes objects from the task.

[Docs](https://developer.tekla.com/topic/en/18/43/61a841b1-43f7-9bc0-18ec-570766ee577e)

#### `Select(...)`

Selects the task instance from the model. The identifier must be set.

[Docs](https://developer.tekla.com/topic/en/18/43/9f8aa6f7-2504-0b38-1a09-e0971e848586)

#### `SetCustomObjectType(...)`

Sets a custom object type name for an existing object.

[Docs](https://developer.tekla.com/topic/en/18/43/3d6fb91b-48a9-35ca-d498-526514344089)

#### `SetDynamicStringProperty(...)`

Sets a dynamic string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/4dab0254-09e5-5e61-c28b-1c12afab71a0)

#### `SetLabel(...)`

Sets a label for an object when a new instance is created, this method must be called before Insert. The label is used in plug-ins for identifying the changed object in modification.

[Docs](https://developer.tekla.com/topic/en/18/43/3efcfc44-613a-e66b-9ebd-9f86df4f4036)

#### `SetPhase(...)`

Sets the phase of the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/ed80abcc-0ff7-d081-2229-e651f858a727)

#### `SetUserProperties(...)`

Sets multiple properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/8659da37-d571-f03f-c666-5c9fa3b113f5)

#### `SetUserProperty(String, Double)(...)`

Sets a double property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/5c585b8b-5c2e-7b75-2242-758f17429396)

#### `SetUserProperty(String, Int32)(...)`

Sets an integer property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/dd5b828d-3a87-bb58-5ac9-e1c3b4c2fe4b)

#### `SetUserProperty(String, String)(...)`

Sets a string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/80bf0348-b651-3d4a-862b-49c85bc924ab)

### Properties
- `ActualEndDate` (object, get/set) — The actual end date of the task.
- `ActualStartDate` (object, get/set) — The actual start date of the task.
- `ActualWorkAmount` (object, get/set) — The amount of work already used for the task.
- `Completeness` (object, get/set) — The percentage of the completeness of the task on the scale from 0 to 100.
- `Critical` (object, get/set) — The criticality of the task.
- `Description` (object, get/set) — A short textual description of the task.
- `Identifier` (object, get/set) — The identifier of the object.
- `IsUpToDate` (object, get/set) — Gets if the object does not have a modification which is not shared.
- `Local` (object, get/set) — The locality indicates if the task was created in Tekla Structures and Task Manager or imported.
- `ModificationTime` (object, get/set) — Gets latest time of the object was modified or created.
- `Name` (object, get/set) — The name of the task.
- `PlannedEndDate` (object, get/set) — The planned end date of the task.
- `PlannedStartDate` (object, get/set) — The planned start date of the task.
- `PlannedWorkAmount` (object, get/set) — The amount of work planned to be used for the task.
- `Scenario` (object, get/set) — The scenario which the task belongs to.
- `Url` (object, get/set) — A link to material or data related to the task. The link can be a hyperlink or a file system link.

## TaskDependency (class)

The TaskDependency class represents a dependency between two task objects. Each task may depend on many other tasks, but between two specific tasks only one type of a dependency may exist. There are four types of dependencies: FINISH_TO_FINISH (FF)FINISH_TO_START (FS)START_TO_FINISH (SF)START_TO_START (SS)

[Vendor docs](https://developer.tekla.com/topic/en/18/43/2a999834-8d4f-42de-83fc-d66316056fc2)

### Constructors
- `TaskDependency(...)` — Creates a new task dependency instance.
- `TaskDependency(...)` — Creates a new task dependency instance with the given primary and secondary tasks.

### Methods
#### `CompareTo(...)`

Compares Identifiers of model objects.

[Docs](https://developer.tekla.com/topic/en/18/43/79cca356-0c8a-e8fb-463a-34ad4141bec7)

#### `Delete(...)`

Deletes the task dependency instance from the model.

[Docs](https://developer.tekla.com/topic/en/18/43/7d2e148a-f4c4-0f80-7ea1-74c9bce8126f)

#### `Equals(...)`

Check if Identifiers of model objects are same.

[Docs](https://developer.tekla.com/topic/en/18/43/e4324265-2490-00de-4139-63a8f7acb492)

#### `GetAllReportProperties(...)`

Retrieves all properties for the model object in one hashtable. Type for the returned value must be checked using type casting.

[Docs](https://developer.tekla.com/topic/en/18/43/0708f404-ce77-28e1-4ebe-fa906f68bff8)

#### `GetAllUserProperties(...)`

Retrieves all properties for the model object in one hashtable. Type for the returned value must be checked using type casting.

[Docs](https://developer.tekla.com/topic/en/18/43/63b853a0-0af5-af9c-3ce7-05299664d7f8)

#### `GetChildren(...)`

Returns an enumerator of all the children model objects.

[Docs](https://developer.tekla.com/topic/en/18/43/ce51ec40-310c-6067-a89f-f8d029aab747)

#### `GetCoordinateSystem(...)`

Returns the coordinate system for the given model object.

[Docs](https://developer.tekla.com/topic/en/18/43/8adfe9cd-f6e5-6474-1297-33c6270a7b0a)

#### `GetCustomObjectType(...)`

Gets custom object type name from an existing object.

[Docs](https://developer.tekla.com/topic/en/18/43/d3be2ba3-8533-7165-f2d3-a5aa9df556b8)

#### `GetDoubleReportProperties(...)`

Retrieves multiple double report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/f86f6c96-f738-49ca-16ba-f5634e862ba2)

#### `GetDoubleUserProperties(...)`

Retrieves all double properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/6ead7a63-5f76-0631-d480-d46c27e882aa)

#### `GetDynamicStringProperty(...)`

Gets a dynamic string property from the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/415e86d9-7de6-ea53-ce68-20b3e67b8655)

#### `GetFatherComponent(...)`

Returns the father component of the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/a2c28f3f-52ff-50a8-1b32-10d18db5c31d)

#### `GetHierarchicObjects(...)`

Returns an enumerator of all the connected hierarchic objects.

[Docs](https://developer.tekla.com/topic/en/18/43/cc228ea0-267b-7d90-7441-c9efc1779b08)

#### `GetIntegerReportProperties(...)`

Retrieves multiple integer report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/2623a9c0-2a9c-e233-9daa-59f92ed51445)

#### `GetIntegerUserProperties(...)`

Retrieves all integer properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/1b3c6f06-bc25-a54e-2a88-b60dc4a3c85e)

#### `GetPhase(...)`

Retrieves the phase of the model object (the phase number, the phase name, the phase comment and whether the phase is the current one or not).

[Docs](https://developer.tekla.com/topic/en/18/43/310f8b7a-c120-753a-35c9-f7c73a9806c9)

#### `GetReportProperty(String, Double.)(...)`

Retrieves a double property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/1c05aa56-5f42-be91-a59e-8d59a5549f45)

#### `GetReportProperty(String, Int32.)(...)`

Retrieves an integer property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/ce8e4ca0-3750-3c9b-5ca8-27e48b45e0bd)

#### `GetReportProperty(String, String.)(...)`

Retrieves a string property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/70f212bb-ee70-e9e9-7ab6-d6a9d46f8de6)

#### `GetStringReportProperties(...)`

Retrieves multiple string report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/5ff72da1-7382-f5d2-eb1d-c99b46239d42)

#### `GetStringUserProperties(...)`

Retrieves all string properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/67120e84-c820-9d52-12ae-c167c595842c)

#### `GetUserProperty(String, Double.)(...)`

Retrieves a double property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/17cc8074-7955-32b5-2b4b-b169c45d4ee0)

#### `GetUserProperty(String, Int32.)(...)`

Retrieves an integer property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/d9364c1d-5b57-6a31-4c13-01124058cfb2)

#### `GetUserProperty(String, String.)(...)`

Retrieves a string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/214a886c-c18c-9b66-d7c9-fe84260962f2)

#### `Insert(...)`

Inserts the task dependency instance in the model.

[Docs](https://developer.tekla.com/topic/en/18/43/4a52527d-ca1d-724c-3443-288439aba7f0)

#### `Modify(...)`

Modifies the task dependency instance in the model. The identifier must be set.

[Docs](https://developer.tekla.com/topic/en/18/43/e50ce4fd-135b-9658-31ae-396133a565e5)

#### `Select(...)`

Selects the task dependency instance from the model. The identifier must be set.

[Docs](https://developer.tekla.com/topic/en/18/43/9a4412b2-6fb2-4620-4a0e-70a54b82fc49)

#### `SetCustomObjectType(...)`

Sets a custom object type name for an existing object.

[Docs](https://developer.tekla.com/topic/en/18/43/3d6fb91b-48a9-35ca-d498-526514344089)

#### `SetDynamicStringProperty(...)`

Sets a dynamic string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/4dab0254-09e5-5e61-c28b-1c12afab71a0)

#### `SetLabel(...)`

Sets a label for an object when a new instance is created, this method must be called before Insert. The label is used in plug-ins for identifying the changed object in modification.

[Docs](https://developer.tekla.com/topic/en/18/43/3efcfc44-613a-e66b-9ebd-9f86df4f4036)

#### `SetPhase(...)`

Sets the phase of the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/ed80abcc-0ff7-d081-2229-e651f858a727)

#### `SetUserProperties(...)`

Sets multiple properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/8659da37-d571-f03f-c666-5c9fa3b113f5)

#### `SetUserProperty(String, Double)(...)`

Sets a double property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/5c585b8b-5c2e-7b75-2242-758f17429396)

#### `SetUserProperty(String, Int32)(...)`

Sets an integer property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/dd5b828d-3a87-bb58-5ac9-e1c3b4c2fe4b)

#### `SetUserProperty(String, String)(...)`

Sets a string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/80bf0348-b651-3d4a-862b-49c85bc924ab)

### Properties
- `DependencyType` (object, get/set) — The type of the task dependency.
- `Identifier` (object, get/set) — The identifier of the object.
- `IsUpToDate` (object, get/set) — Gets if the object does not have a modification which is not shared.
- `Lag` (object, get/set) — The lag of the task dependency represented in whole hours.
- `Local` (object, get/set) — The locality indicates if the dependency was created in Tekla Structures and Task Manager or imported.
- `ModificationTime` (object, get/set) — Gets latest time of the object was modified or created.
- `Primary` (object, get/set) — The primary task of the task dependency.
- `Secondary` (object, get/set) — The secondary task of the task dependency.

## TaskDependency.DependencyTypeEnum (enum)

The task dependency types.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/51995429-984b-e28d-1b25-adbb3a83fe37)

### Values
- `FINISH_TO_FINISH` = `0` — The task dependency type finish-to-finish.
- `FINISH_TO_START` = `1` — The task dependency type finish-to-start.
- `START_TO_FINISH` = `2` — The task dependency type start-to-finish.
- `START_TO_START` = `3` — The task dependency type start-to-start.

## TaskWorktype (class)

The TaskWorktype class defines a single worktype for a task object. Each task may belong to only one worktype.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/97bac05f-2188-ce24-1011-6e2e97cc8e3a)

### Constructors
- `TaskWorktype(...)` — Creates a new task worktype instance.

### Methods
#### `CompareTo(...)`

Compares Identifiers of model objects.

[Docs](https://developer.tekla.com/topic/en/18/43/79cca356-0c8a-e8fb-463a-34ad4141bec7)

#### `Delete(...)`

Deletes the task worktype instance from the model. The identifier must be set.

[Docs](https://developer.tekla.com/topic/en/18/43/a6b773d9-9be6-1900-42f2-e82be0c12aea)

#### `Equals(...)`

Check if Identifiers of model objects are same.

[Docs](https://developer.tekla.com/topic/en/18/43/e4324265-2490-00de-4139-63a8f7acb492)

#### `GetAllReportProperties(...)`

Retrieves all properties for the model object in one hashtable. Type for the returned value must be checked using type casting.

[Docs](https://developer.tekla.com/topic/en/18/43/0708f404-ce77-28e1-4ebe-fa906f68bff8)

#### `GetAllUserProperties(...)`

Retrieves all properties for the model object in one hashtable. Type for the returned value must be checked using type casting.

[Docs](https://developer.tekla.com/topic/en/18/43/63b853a0-0af5-af9c-3ce7-05299664d7f8)

#### `GetChildren(...)`

Returns an enumerator of all the children model objects.

[Docs](https://developer.tekla.com/topic/en/18/43/ce51ec40-310c-6067-a89f-f8d029aab747)

#### `GetCoordinateSystem(...)`

Returns the coordinate system for the given model object.

[Docs](https://developer.tekla.com/topic/en/18/43/8adfe9cd-f6e5-6474-1297-33c6270a7b0a)

#### `GetCustomObjectType(...)`

Gets custom object type name from an existing object.

[Docs](https://developer.tekla.com/topic/en/18/43/d3be2ba3-8533-7165-f2d3-a5aa9df556b8)

#### `GetDoubleReportProperties(...)`

Retrieves multiple double report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/f86f6c96-f738-49ca-16ba-f5634e862ba2)

#### `GetDoubleUserProperties(...)`

Retrieves all double properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/6ead7a63-5f76-0631-d480-d46c27e882aa)

#### `GetDynamicStringProperty(...)`

Gets a dynamic string property from the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/415e86d9-7de6-ea53-ce68-20b3e67b8655)

#### `GetFatherComponent(...)`

Returns the father component of the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/a2c28f3f-52ff-50a8-1b32-10d18db5c31d)

#### `GetHierarchicObjects(...)`

Returns an enumerator of all the connected hierarchic objects.

[Docs](https://developer.tekla.com/topic/en/18/43/cc228ea0-267b-7d90-7441-c9efc1779b08)

#### `GetIntegerReportProperties(...)`

Retrieves multiple integer report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/2623a9c0-2a9c-e233-9daa-59f92ed51445)

#### `GetIntegerUserProperties(...)`

Retrieves all integer properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/1b3c6f06-bc25-a54e-2a88-b60dc4a3c85e)

#### `GetPhase(...)`

Retrieves the phase of the model object (the phase number, the phase name, the phase comment and whether the phase is the current one or not).

[Docs](https://developer.tekla.com/topic/en/18/43/310f8b7a-c120-753a-35c9-f7c73a9806c9)

#### `GetReportProperty(String, Double.)(...)`

Retrieves a double property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/1c05aa56-5f42-be91-a59e-8d59a5549f45)

#### `GetReportProperty(String, Int32.)(...)`

Retrieves an integer property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/ce8e4ca0-3750-3c9b-5ca8-27e48b45e0bd)

#### `GetReportProperty(String, String.)(...)`

Retrieves a string property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/70f212bb-ee70-e9e9-7ab6-d6a9d46f8de6)

#### `GetStringReportProperties(...)`

Retrieves multiple string report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/5ff72da1-7382-f5d2-eb1d-c99b46239d42)

#### `GetStringUserProperties(...)`

Retrieves all string properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/67120e84-c820-9d52-12ae-c167c595842c)

#### `GetUserProperty(String, Double.)(...)`

Retrieves a double property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/17cc8074-7955-32b5-2b4b-b169c45d4ee0)

#### `GetUserProperty(String, Int32.)(...)`

Retrieves an integer property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/d9364c1d-5b57-6a31-4c13-01124058cfb2)

#### `GetUserProperty(String, String.)(...)`

Retrieves a string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/214a886c-c18c-9b66-d7c9-fe84260962f2)

#### `Insert(...)`

Inserts the task worktype instance in the model.

[Docs](https://developer.tekla.com/topic/en/18/43/0b75298f-7f79-1a3e-f880-6236026a60a5)

#### `Modify(...)`

Modifies the task worktype instance in the model. The identifier must be set.

[Docs](https://developer.tekla.com/topic/en/18/43/74c6a001-0ef3-9b37-66c3-8e805a8107ab)

#### `Select(...)`

Selects the task worktype instance from the model. The identifier must be set.

[Docs](https://developer.tekla.com/topic/en/18/43/96077a89-fee0-6790-e6cd-9f7ad98d0fee)

#### `SetCustomObjectType(...)`

Sets a custom object type name for an existing object.

[Docs](https://developer.tekla.com/topic/en/18/43/3d6fb91b-48a9-35ca-d498-526514344089)

#### `SetDynamicStringProperty(...)`

Sets a dynamic string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/4dab0254-09e5-5e61-c28b-1c12afab71a0)

#### `SetLabel(...)`

Sets a label for an object when a new instance is created, this method must be called before Insert. The label is used in plug-ins for identifying the changed object in modification.

[Docs](https://developer.tekla.com/topic/en/18/43/3efcfc44-613a-e66b-9ebd-9f86df4f4036)

#### `SetPhase(...)`

Sets the phase of the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/ed80abcc-0ff7-d081-2229-e651f858a727)

#### `SetUserProperties(...)`

Sets multiple properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/8659da37-d571-f03f-c666-5c9fa3b113f5)

#### `SetUserProperty(String, Double)(...)`

Sets a double property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/5c585b8b-5c2e-7b75-2242-758f17429396)

#### `SetUserProperty(String, Int32)(...)`

Sets an integer property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/dd5b828d-3a87-bb58-5ac9-e1c3b4c2fe4b)

#### `SetUserProperty(String, String)(...)`

Sets a string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/80bf0348-b651-3d4a-862b-49c85bc924ab)

### Properties
- `Identifier` (object, get/set) — The identifier of the object.
- `IsUpToDate` (object, get/set) — Gets if the object does not have a modification which is not shared.
- `ModificationTime` (object, get/set) — Gets latest time of the object was modified or created.
- `Name` (object, get/set) — The name of the task worktype.

## TransformationPlane (class)

The TransformationPlane class describes a transformation from global model coordinates to local and back to global.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/324b1565-2e6e-8847-6c8b-88ca12e2151d)

### Constructors
- `TransformationPlane(...)` — Creates a new identity transformation using the global coordinate system.
- `TransformationPlane(...)` — Creates a new transformation plane defined by the given coordinate system.
- `TransformationPlane(...)` — Creates a new transformation plane defined by the given origin and two vectors.

### Methods
#### `ToString(...)`

Returns a string representation of the matrices that transform to local and global coordinates.

[Docs](https://developer.tekla.com/topic/en/18/43/5a396a55-b509-6d5c-71cc-d4cd1861e3a6)

### Properties
- `TransformationMatrixToGlobal` (object, get/set) — Gives a transformation matrix for converting local points in the transformation plane to global coordinates.
- `TransformationMatrixToLocal` (object, get/set) — Gives a transformation matrix for converting points in the global plane to local coordinates in the transformation plane.

## UndefinedCurveDirectionException (class)

The UndefinedCurveDirectionException class represents an error when, cannot make inward/outward curve check.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/b4d572bd-333a-26f2-458c-d688d2ca0c4e)

### Constructors
- `UndefinedCurveDirectionException(...)` — Initializes a new instance of the UndefinedCurveDirectionException class with "Cannot make inward/outward curve check" error message.

## UnknownLoftedPlateErrorException (class)

This class represent an exception thrown when an operation on a lofted plate caused an error of unknown origin

[Vendor docs](https://developer.tekla.com/topic/en/18/43/1b7dc228-870a-7be6-0bb6-cfc92a24ab32)

### Constructors
- `UnknownLoftedPlateErrorException(...)` — Constructs an instance of the class

## UnsupportedChamferException (class)

The UnsupportedChamferException class represents an error when, plates containing unsupported chamfer information.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/7ab3fa5a-0958-525d-91a2-87eaf92f798d)

### Constructors
- `UnsupportedChamferException(...)` — Initializes a new instance of the UnsupportedChamferException class with "Plates containing unsupported chamfer information" error message.

## Weld (class)

The Weld class represents a normal weld in the model. A normal weld has a main part and a secondary part.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/0acd5a34-eab5-0e9a-4869-d7f825f3a608)

### Constructors
- `Weld(...)` — Creates a new weld instance.

### Methods
#### `CompareTo(...)`

Compares Identifiers of model objects.

[Docs](https://developer.tekla.com/topic/en/18/43/79cca356-0c8a-e8fb-463a-34ad4141bec7)

#### `Delete(...)`

Deletes the weld instance with the given identifier from the model database.

[Docs](https://developer.tekla.com/topic/en/18/43/bf988072-41c7-e6d6-e3a5-4e65cd4a0522)

#### `Equals(...)`

Check if Identifiers of model objects are same.

[Docs](https://developer.tekla.com/topic/en/18/43/e4324265-2490-00de-4139-63a8f7acb492)

#### `GetAllReportProperties(...)`

Retrieves all properties for the model object in one hashtable. Type for the returned value must be checked using type casting.

[Docs](https://developer.tekla.com/topic/en/18/43/0708f404-ce77-28e1-4ebe-fa906f68bff8)

#### `GetAllUserProperties(...)`

Retrieves all properties for the model object in one hashtable. Type for the returned value must be checked using type casting.

[Docs](https://developer.tekla.com/topic/en/18/43/63b853a0-0af5-af9c-3ce7-05299664d7f8)

#### `GetChildren(...)`

Returns an enumerator of all the children model objects.

[Docs](https://developer.tekla.com/topic/en/18/43/ce51ec40-310c-6067-a89f-f8d029aab747)

#### `GetCoordinateSystem(...)`

Returns the coordinate system for the given model object.

[Docs](https://developer.tekla.com/topic/en/18/43/8adfe9cd-f6e5-6474-1297-33c6270a7b0a)

#### `GetCustomObjectType(...)`

Gets custom object type name from an existing object.

[Docs](https://developer.tekla.com/topic/en/18/43/d3be2ba3-8533-7165-f2d3-a5aa9df556b8)

#### `GetDoubleReportProperties(...)`

Retrieves multiple double report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/f86f6c96-f738-49ca-16ba-f5634e862ba2)

#### `GetDoubleUserProperties(...)`

Retrieves all double properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/6ead7a63-5f76-0631-d480-d46c27e882aa)

#### `GetDynamicStringProperty(...)`

Gets a dynamic string property from the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/415e86d9-7de6-ea53-ce68-20b3e67b8655)

#### `GetFatherComponent(...)`

Returns the father component of the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/a2c28f3f-52ff-50a8-1b32-10d18db5c31d)

#### `GetHierarchicObjects(...)`

Returns an enumerator of all the connected hierarchic objects.

[Docs](https://developer.tekla.com/topic/en/18/43/cc228ea0-267b-7d90-7441-c9efc1779b08)

#### `GetIntegerReportProperties(...)`

Retrieves multiple integer report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/2623a9c0-2a9c-e233-9daa-59f92ed51445)

#### `GetIntegerUserProperties(...)`

Retrieves all integer properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/1b3c6f06-bc25-a54e-2a88-b60dc4a3c85e)

#### `GetLogicalWeld(...)`

Gets the logical weld where the weld belongs. Returns false if the weld does not belong to any logical weld.

[Docs](https://developer.tekla.com/topic/en/18/43/5db7894d-cd3b-ee1e-7cf5-b3ead3c88e04)

#### `GetPhase(...)`

Retrieves the phase of the model object (the phase number, the phase name, the phase comment and whether the phase is the current one or not).

[Docs](https://developer.tekla.com/topic/en/18/43/310f8b7a-c120-753a-35c9-f7c73a9806c9)

#### `GetReportProperty(String, Double.)(...)`

Retrieves a double property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/1c05aa56-5f42-be91-a59e-8d59a5549f45)

#### `GetReportProperty(String, Int32.)(...)`

Retrieves an integer property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/ce8e4ca0-3750-3c9b-5ca8-27e48b45e0bd)

#### `GetReportProperty(String, String.)(...)`

Retrieves a string property of the report for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/70f212bb-ee70-e9e9-7ab6-d6a9d46f8de6)

#### `GetSolid(...)`

Method for getting the weld solid.

[Docs](https://developer.tekla.com/topic/en/18/43/319baa05-e7f2-e9ff-1f4f-c2a403dd71d1)

#### `GetStringReportProperties(...)`

Retrieves multiple string report properties for the model object. It is faster to fetch multiple properties at once.

[Docs](https://developer.tekla.com/topic/en/18/43/5ff72da1-7382-f5d2-eb1d-c99b46239d42)

#### `GetStringUserProperties(...)`

Retrieves all string properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/67120e84-c820-9d52-12ae-c167c595842c)

#### `GetUserProperty(String, Double.)(...)`

Retrieves a double property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/17cc8074-7955-32b5-2b4b-b169c45d4ee0)

#### `GetUserProperty(String, Int32.)(...)`

Retrieves an integer property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/d9364c1d-5b57-6a31-4c13-01124058cfb2)

#### `GetUserProperty(String, String.)(...)`

Retrieves a string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/214a886c-c18c-9b66-d7c9-fe84260962f2)

#### `GetWeldGeometries(...)`

Method for getting weld seam geometries. Every result represents one individual seam geometry in current weld.

[Docs](https://developer.tekla.com/topic/en/18/43/335a1753-ab21-bc4a-f02b-2dfa5e951c7a)

#### `Insert(...)`

Inserts the weld into the model database. All the attributes must be set.

[Docs](https://developer.tekla.com/topic/en/18/43/2702f7e5-5442-956e-e1a4-ece27e192cc0)

#### `Modify(...)`

Modifies the existing weld in the model database to match the current one. The modification cannot be done if the weld is a part of a logical weld.

[Docs](https://developer.tekla.com/topic/en/18/43/eec46d80-3cfd-e95a-db9f-e9e86cb67c23)

#### `Select(...)`

Selects a weld from the model database. The weld identifier must be set.

[Docs](https://developer.tekla.com/topic/en/18/43/b0fc0282-7a98-b4fe-c441-fd0a076efbff)

#### `SetCustomObjectType(...)`

Sets a custom object type name for an existing object.

[Docs](https://developer.tekla.com/topic/en/18/43/3d6fb91b-48a9-35ca-d498-526514344089)

#### `SetDynamicStringProperty(...)`

Sets a dynamic string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/4dab0254-09e5-5e61-c28b-1c12afab71a0)

#### `SetLabel(...)`

Sets a label for an object when a new instance is created, this method must be called before Insert. The label is used in plug-ins for identifying the changed object in modification.

[Docs](https://developer.tekla.com/topic/en/18/43/3efcfc44-613a-e66b-9ebd-9f86df4f4036)

#### `SetPhase(...)`

Sets the phase of the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/ed80abcc-0ff7-d081-2229-e651f858a727)

#### `SetUserProperties(...)`

Sets multiple properties for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/8659da37-d571-f03f-c666-5c9fa3b113f5)

#### `SetUserProperty(String, Double)(...)`

Sets a double property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/5c585b8b-5c2e-7b75-2242-758f17429396)

#### `SetUserProperty(String, Int32)(...)`

Sets an integer property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/dd5b828d-3a87-bb58-5ac9-e1c3b4c2fe4b)

#### `SetUserProperty(String, String)(...)`

Sets a string property for the model object.

[Docs](https://developer.tekla.com/topic/en/18/43/80bf0348-b651-3d4a-862b-49c85bc924ab)

### Properties
- `AdditionalSizeAbove` (object, get/set) — Gets or sets the additional size above for combination welds.
- `AdditionalSizeBelow` (object, get/set) — Gets or sets the additional size below for combination welds.
- `AngleAbove` (object, get/set) — Gets or sets the angle above.
- `AngleBelow` (object, get/set) — Gets or sets the angle below.
- `AroundWeld` (object, get/set) — Gets or sets a value indicating whether the weld is an around weld (true) or an edge weld (false).
- `ConnectAssemblies` (object, get/set) — Gets or sets a value indicating whether to connect a part or an assembly as a secondary part (false) or as a sub-assembly (true).
- `ContourAbove` (object, get/set) — Gets or sets the contour above.
- `ContourBelow` (object, get/set) — Gets or sets the contour below.
- `Direction` (object, get/set) — Defines the Weld edge search direction normalized to length 1000.0. Overrides Position if set.
- `EffectiveThroatAbove` (object, get/set) — Gets or sets the effective throat above.
- `EffectiveThroatBelow` (object, get/set) — Gets or sets the effective throat below.
- `ElectrodeClassification` (object, get/set) — Gets or sets the weld electrode classification.
- `ElectrodeCoefficient` (object, get/set) — Gets or sets the electrode strength coefficient.
- `ElectrodeStrength` (object, get/set) — Gets or sets the electrode strength.
- `FinishAbove` (object, get/set) — Gets or sets the finish above.
- `FinishBelow` (object, get/set) — Gets or sets the finish below.
- `Identifier` (object, get/set) — The identifier of the object.
- `IncrementAmountAbove` (object, get/set) — Gets or sets the increment amount above.
- `IncrementAmountBelow` (object, get/set) — Gets or sets the increment amount below.
- `IntermittentType` (object, get/set) — Gets or sets the weld intermittent type.
- `IsUpToDate` (object, get/set) — Gets if the object does not have a modification which is not shared.
- `LengthAbove` (object, get/set) — Gets or sets the length above.
- `LengthBelow` (object, get/set) — Gets or sets the length below.
- `MainObject` (object, get/set) — Gets or sets the main part of the weld.
- `ModificationTime` (object, get/set) — Gets latest time of the object was modified or created.
- `NDTInspection` (object, get/set) — Gets or sets the NDT inspection level.
- `PitchAbove` (object, get/set) — Gets or sets the pitch above.
- `PitchBelow` (object, get/set) — Gets or sets the pitch below.
- `Placement` (object, get/set) — Gets or sets the weld placement.
- `Position` (object, get/set) — Defines the position. Use of Direction is preferred over this.
- `PrefixAboveLine` (object, get/set) — Gets or sets the size prefix above the line.
- `PrefixBelowLine` (object, get/set) — Gets or sets the size prefix below the line.
- `Preparation` (object, get/set) — Gets or sets the weld preparation.
- `ProcessType` (object, get/set) — Gets or sets the process type.
- `ReferenceText` (object, get/set) — Gets or sets the reference text.
- `RootFaceAbove` (object, get/set) — Gets or sets the root face above.
- `RootFaceBelow` (object, get/set) — Gets or sets the root face below.
- `RootOpeningAbove` (object, get/set) — Gets or sets the root opening above.
- `RootOpeningBelow` (object, get/set) — Gets or sets the root opening below.
- `SecondaryObject` (object, get/set) — Gets or sets the secondary part of the weld.
- `ShopWeld` (object, get/set) — Gets or sets a value indicating whether the weld is a shop weld (true) or a site weld (false).
- `SizeAbove` (object, get/set) — Gets or sets the size above.
- `SizeBelow` (object, get/set) — Gets or sets the size below.
- `Standard` (object, get/set) — Gets or sets the weld detail/standard.
- `StitchWeld` (object, get/set) — Gets or sets a value indicating whether the weld is stitched (true) or not stitched (false).
- `TypeAbove` (object, get/set) — Gets or sets the type above.
- `TypeBelow` (object, get/set) — Gets or sets the type below.
- `WeldNumber` (object, get/set) — Gets the weld number.
- `WeldNumberPrefix` (object, get/set) — Gets or sets the weld number prefix.

## Weld.WeldPositionEnum (enum)

The weld position.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/845f121d-e5ce-cfe1-1467-bd657060b631)

### Values
- `WELD_POSITION_PLUS_X` = `1` — The position +x.
- `WELD_POSITION_MINUS_X` = `2` — The position -x.
- `WELD_POSITION_PLUS_Y` = `3` — The position +y.
- `WELD_POSITION_MINUS_Y` = `4` — The position -y.
- `WELD_POSITION_PLUS_Z` = `5` — The position +z.
- `WELD_POSITION_MINUS_Z` = `6` — The position -z.

## WorkPlaneHandler (class)

The WorkPlaneHandler class contains methods for getting and setting the current transformation plane.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/ac187147-19d4-851c-7af5-19e217534818)

### Methods
#### `GetCurrentTransformationPlane(...)`

Gets the current transformation plane.

[Docs](https://developer.tekla.com/topic/en/18/43/74206331-60ff-3061-ce71-526788aee687)

#### `SetCurrentTransformationPlane(...)`

Sets the current transformation plane.

[Docs](https://developer.tekla.com/topic/en/18/43/e6f659a2-cc4a-9e35-fbde-78769210ab87)

