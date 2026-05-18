---
name: tekla-tekla-structures-plugins
description: This skill encodes the tekla 2026.0 surface of the Tekla.Structures.Plugins namespace — 34 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: AutoDirectionTypeAttribute, ConnectionBase, CustomPartBase, CustomPartInputTypeAttribute, DetailTypeAttribute, CustomPartPositioningTypeAttribute, DrawingPluginBase, DrawingPluginBase.InputDefinition, and 26 more types.
---

# Tekla.Structures.Plugins

Auto-generated from vendor docs for tekla 2026.0. 34 types in this namespace.

## AutoDirectionTypeAttribute (class)

The AutoDirectionTypeAttribute class is used for storing the auto direction type. Based on the type the system will then calculate the up direction of the connection/detail if the "Auto" up direction is chosen in the dialog. The attribute is initialized from the custom attribute [AutoDirectionType(AutoDirectionTypeEnum.AUTODIR_DETAIL)] in the connection/detail source.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/6319f80e-cec4-116d-6232-c66ff4de6101)

### Constructors
- `public AutoDirectionTypeAttribute(int Type)` — The custom attribute [AutoDirectionType()] uses this to store the auto direction type to the system. This is not to be used by itself.
- `public AutoDirectionTypeAttribute(AutoDirectionTypeEnum Type)` — The custom attribute [AutoDirectionType()] uses this to store the auto direction type to the system. This is not to be used by itself.

### Properties
- `Type` (AutoDirectionTypeEnum, get) — The custom attribute [AutoDirectionType()] uses this to store the auto direction type to the system. This is not to be used by itself.

## ConnectionBase (class)

The ConnectionBase class is a base class for defining connections, details and seams. These types are more specialized and restricted by the input values than the ones derived from PluginBase. The coordinate system for connections, details and seams is explained in the Tekla Structures help, in the part about the position type of custom components. The position type defines the origin of the custom component, relative to the main part.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/6109ad9b-caa0-e7bd-dd42-dafe9b7d82a9)

### Methods
#### `public bool IsDefaultValue(double Value)`

Returns true if the given value is set to the default value for this type.

**Parameters:**
- `Value` (System.Double) — The value to test.

**Returns:** `Boolean` — True if the value is set to the default.

[Docs](https://developer.tekla.com/topic/en/18/47/999607cd-78b3-27fa-2d16-477291d612e6)

#### `public bool IsDefaultValue(int Value)`

Returns true if the given value is set to the default value for this type.

**Parameters:**
- `Value` (System.Int32) — The value to test.

**Returns:** `Boolean` — True if the value is set to the default.

[Docs](https://developer.tekla.com/topic/en/18/47/8260c2f5-8b1b-fe04-715e-b5408f07da8b)

#### `public bool IsDefaultValue(string Value)`

Returns true if the given value is set to the default value (empty string).

**Parameters:**
- `Value` (System.String) — The value to test.

**Returns:** `Boolean` — True if the value is set to the default.

[Docs](https://developer.tekla.com/topic/en/18/47/ef9ef015-5d36-7c29-b229-3ae1bdd7be20)

#### `public abstract bool Run()`

The main method of the component. Inside Run the user can implement the logic based on the user given attributes and input. Inside the method input can be found from the provided properties: Primary, Secondaries and Positions.

**Returns:** `Boolean` — True on success.

[Docs](https://developer.tekla.com/topic/en/18/47/1fe2084f-e160-10b4-22a3-457d714a382b)

### Properties
- `Code` (String, get/set) — The connection code of the executable connection instance. The maximum length is 22 characters.
- `Identifier` (Identifier, get) — The identifier of the executable plug-in instance.
- `Positions` (List<Point>, get) — The positional attributes for a detail or a seam instance; one for a detail, N for a seam.
- `Primary` (Identifier, get) — The identifier that was selected as the primary object.
- `Secondaries` (List<Identifier>, get) — A list of secondary identifiers of a connection or a seam.

## ConnectionBase.InputObjectType (enum)

Defines the input object type.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/1a07ad5c-4674-2484-581f-40fd5bf24da7)

### Values
- `INPUTOBJECT_PART` = `0` — The input object is a part instance.
- `INPUTOBJECT_CUSTOMPART` = `1` — The input object can be a custom part (component).

## ConnectionBase.SeamInputType (enum)

Defines the type of the input.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/2603c888-2ddc-b32a-6f25-c65e32d1ff78)

### Values
- `INPUT_2_POINTS` = `1` — The input is two points. The data is returned as an array list of points.
- `INPUT_POLYGON` = `2` — The input is any number of points (a polygon). The data is returned as an array list of points.

## ConnectionBase.SecondaryType (enum)

Defines how many secondaries a connection can have.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/9568ad7f-6ab6-c04b-5307-4d30cc704931)

### Values
- `SECONDARYTYPE_ZERO` = `0` — The connection can have zero secondaries. Zero secondaries means that the connection is a detail.
- `SECONDARYTYPE_ONE` = `1` — The connection can have one secondary.
- `SECONDARYTYPE_TWO` = `2` — The connection can have two secondaries.
- `SECONDARYTYPE_MULTIPLE` = `3` — The connection can have multiple secondaries.

## CustomPartBase (class)

The CustomPartBase class is a base class for defining custom parts. These types are more specialized and restricted by the input values than the ones derived from PluginBase. The coordinate system for connections, details and seams is explained in the Tekla Structures help, in the part about the position type of custom components. The position type defines the origin of the custom component, relative to the main part.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/b498d9fd-0928-7653-c323-d66cfb18f678)

### Methods
#### `public bool IsDefaultValue(double Value)`

Returns true if the given value is set to the default value for this type.

**Parameters:**
- `Value` (System.Double) — The value to test.

**Returns:** `Boolean` — True if the value is set to the default.

[Docs](https://developer.tekla.com/topic/en/18/47/0d112a60-b9ba-0cdc-49a9-b85f4a0d3557)

#### `public bool IsDefaultValue(int Value)`

Returns true if the given value is set to the default value for this type.

**Parameters:**
- `Value` (System.Int32) — The value to test.

**Returns:** `Boolean` — True if the value is set to the default.

[Docs](https://developer.tekla.com/topic/en/18/47/6dbb47e9-a1c7-82ca-4b8e-d5fa5bc2a7fc)

#### `public bool IsDefaultValue(string Value)`

Returns true if the given value is set to the default value (empty string).

**Parameters:**
- `Value` (System.String) — The value to test.

**Returns:** `Boolean` — True if the value is set to the default.

[Docs](https://developer.tekla.com/topic/en/18/47/43ce7a81-3e9e-0ae0-8030-c9d8a06534ff)

#### `public abstract bool Run()`

The main method of the component. Inside Run the user can implement the logic based on the user given attributes and input. Inside the method input can be found from the provided properties: Primary, Secondaries and Positions.

**Returns:** `Boolean` — True on success.

[Docs](https://developer.tekla.com/topic/en/18/47/2785e532-af81-6a7d-ef0c-9f615039a143)

### Properties
- `Identifier` (Identifier, get) — The identifier of the executable plug-in instance.
- `Positions` (List<Point>, get) — The positional attributes for a custom part instance.

## CustomPartBase.CustomPartInputType (enum)

Defines the type of the input.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/c9fa6dad-b75e-dcee-2222-118095d0d763)

### Values
- `INPUT_2_POINTS` = `1` — The input is two points. The data is returned as an array list of points. Default value.
- `INPUT_1_POINT` = `2` — The input is one point. The data is returned as an array list of points.

## CustomPartBase.CustomPartPositioningType (enum)

Defines the type of the positioning.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/e219db13-ecfa-4073-4879-1ca003da8543)

### Values
- `POSITIONING_BY_CENTER_OF_BOUNDING_BOX` = `1` — Positioning happens based on total extrema of objects. Default value. Same as "Use center of the bounding box in positioning" value on in custom component wizard
- `POSITIONING_BY_INPUTPOINTS` = `2` — Positioning happens based on input points and extrema of objects. Similar to exact positioning in custom parts Same as "Use center of the bounding box in positioning" value off in custom component wizard

## CustomPartInputTypeAttribute (class)

The CustomPartInputTypeAttribute class is used for defining the input type. Based on the type the system will then ask for the correct number of input points in the creation. The attribute is initialized from the custom attribute [CustomPartInputType(CustomPartBase.CustomPartInputType.INPUT_1_POINT)] in the custom part source code.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/c53e4a0c-1bda-8552-dd7b-b83017d390ed)

### Constructors
- `public CustomPartInputTypeAttribute(int Type)` — The custom attribute uses this to store the input type to the system. This is not to be used by itself.
- `public CustomPartInputTypeAttribute(CustomPartBase.CustomPartInputType Type)` — The custom attribute uses this to store the input type to the system. This is not to be used by itself.

### Properties
- `Type` (CustomPartBase.CustomPartInputType, get) — The custom attribute uses this to store the input type to the system. This is not to be used by itself.

## CustomPartPositioningTypeAttribute (class)

The CustomPartInputTypeAttribute class is used for defining the input type. Based on the type the system will then position the custom part either based on calculated extrema center point or given input points. The attribute is initialized from the custom attribute i.e. [CustomPartPositioningType((CustomPartBase.CustomPartPositioningType.POSITIONING_BY_INPUTPOINTS)] in the custom part source code.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/68e9e6e3-dc82-cd2c-637d-ec7b0bb89ef2)

### Constructors
- `public CustomPartPositioningTypeAttribute(int Type)` — The custom attribute uses this to store the positioning type to the system. This is not to be used by itself.
- `public CustomPartPositioningTypeAttribute(CustomPartBase.CustomPartPositioningType Type)` — The custom attribute uses this to store the positioning type to the system. This is not to be used by itself.

### Properties
- `Type` (CustomPartBase.CustomPartPositioningType, get) — The custom attribute uses this to store the positioning type to the system. This is not to be used by itself.

## DetailTypeAttribute (class)

The DetailTypeAttribute class is used for storing the detail type. Based on the type the system will then position the detail in the creation. The attribute is initialized from the custom attribute [DetailType(DetailTypeEnum.END)] in the connection source.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/039e5970-ac37-24e7-cd99-658f3ede5126)

### Constructors
- `public DetailTypeAttribute(int Type)` — The custom attribute [DetailType()] uses this to store the type of the detail to the system. This is not to be used by itself.
- `public DetailTypeAttribute(DetailTypeEnum Type)` — The custom attribute [DetailType()] uses this to store the type of the detail to the system. This is not to be used by itself.

### Properties
- `Type` (DetailTypeEnum, get) — The custom attribute [DetailType()] uses this to store the type of the detail to the system. This is not to be used by itself.

## DrawingPluginBase (class)

The DrawingPluginBase class is an abstract base class for drawing plug-ins.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/d888ea51-3dae-c2e2-2c18-d89cbf04ce23)

### Methods
#### `public abstract List<DrawingPluginBase.InputDefinition> DefineInput()`

The method Tekla Structures calls for the plug-in to query the input. The plug-in must then return an array list of input definition instances. The plug-in will be dependent on the items it returns. Dependent means that if any of these items change, for example the user moves the points, the plug-in will be re-run with new input. DefineInput is not called during the re-run, and thus all the actual implementation should be in the Run() method. The Run() method is always called in view coordinates.

**Returns:** `List<DrawingPluginBase.InputDefinition>` — An array list of input definition instances. If the plug-in is not dependent on input, it should return an empty array list (not null).

[Docs](https://developer.tekla.com/topic/en/18/47/b566fc7a-696f-a394-7641-2bcda9c40726)

#### `public bool IsDefaultValue(double Value)`

Returns true if the given value is set to the default value for this type.

**Parameters:**
- `Value` (System.Double) — The value to test.

**Returns:** `Boolean` — True if the value is set to the default.

[Docs](https://developer.tekla.com/topic/en/18/47/7acd70f1-66e5-28b9-73ff-8cce84fdecb0)

#### `public bool IsDefaultValue(int Value)`

Returns true if the given value is set to the default value for this type.

**Parameters:**
- `Value` (System.Int32) — The value to test.

**Returns:** `Boolean` — True if the value is set to the default.

[Docs](https://developer.tekla.com/topic/en/18/47/063121cd-be98-6430-9c28-288b181b1602)

#### `public bool IsDefaultValue(string Value)`

Returns true if the given value is set to the default value (empty string).

**Parameters:**
- `Value` (System.String) — The value to test.

**Returns:** `Boolean` — True if the value is set to the default.

[Docs](https://developer.tekla.com/topic/en/18/47/00f6e4b6-60f6-68de-c128-9be9648e31b6)

#### `public abstract bool Run(List<DrawingPluginBase.InputDefinition> Input)`

The main method of the plug-in. It is called after the input has been defined with DefineInput(). This is the "main" method of the plug-in and should contain all the actual implementation.

**Parameters:**
- `Input` (System.Collections.Generic.List<DrawingPluginBase.InputDefinition>) — An array list of the same format and order as what was returned from DefineInput().

**Returns:** `Boolean` — True on success.

[Docs](https://developer.tekla.com/topic/en/18/47/758feebd-8605-e681-7498-1772eee54f02)

## DrawingPluginBase.InputDefinition (class)

The InputDefinition class is a class for defining the plug-in dependency over the input (points or identifiers). The user implemented method DefineInput() of the DrawingPluginBase interface should return an array list of input definition instances. This defines the points and identifiers the plug-in will receive as input when the Run() method is called.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/671978ea-ae03-3158-95ff-a55cbd398d67)

### Constructors
- `public InputDefinition(Identifier viewId, List<Point> points)` — Creates a new input definition with multiple points.
- `public InputDefinition(Identifier viewId, Point point)` — Creates a new input definition instance with one point.
- `public InputDefinition(Identifier viewId, Identifier ObjectId)` — Creates a new input definition instance with one identifier.
- `public InputDefinition(Identifier viewId, Point point1, Point point2)` — Creates a new input definition instance with two points.
- `public InputDefinition(Identifier viewId, Point point1, Point point2, Point point3)` — Creates a new input definition instance with three points.

### Methods
#### `public Object GetInput()`

Returns the input Tekla Structures gave to the plug-in. The input is either a point instance, an identifier instance, an array list of points or an array list of identifiers. This is based on the input format returned from the DefineInput() method.

**Returns:** `Object` — The input to use in the Run() method.

[Docs](https://developer.tekla.com/topic/en/18/47/9376bb9a-beae-f280-96ac-1b6fd0160c65)

### Properties
- `Type` (DrawingPluginBase.InputDefinition.InputTypes, get/set) — The type of the input the current instance contains.
- `ViewId` (Identifier, get/set) — The view object's identifier.

## DrawingPluginBase.InputDefinition.InputTypes (enum)

The possible input types for a drawing plug-in.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/a7c9fa37-3ea6-4acf-dce6-ce37d13c0aff)

### Values
- `INPUT_ONE_POINT` = `1` — The input has one point.
- `INPUT_TWO_POINTS` = `2` — The input has two points.
- `INPUT_THREE_POINTS` = `3` — The input has three points.
- `INPUT_N_POINTS` = `4` — The input has multiple points.
- `INPUT_ONE_OBJECT` = `5` — The input has one drawing object. The supported drawing objects are bolts, parts and rebars. Bolts, parts and rebars are drawing objects derived from model objects. The parts include the beam, the contour plate and the polybeam.

## DrawingPluginBase.UpdateMode (enum)

Defines the update mode of the drawing plug-in. The update mode tells the system when the plug-in is executed.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/2ca38da1-1e87-f979-1b85-b488ab7fc312)

### Values
- `CREATE_ONLY` = `0` — The plug-in is never updated. Plug-ins are executed from the plug-in dialog instead of the component catalog. The created objects do not have any relation to the plug-in anymore. The plug-in dialog cannot be opened from the created objects.
- `INPUT_CHANGED` = `1` — The plug-in is updated when the input is a point and the point is moved or when the input is an object and the object changes. The plug-in is executed when the input is an object and its properties are changed in the drawing editor. This mode is the default which is used if the update mode is not defined in the plug-in source.
- `DRAWING_OPENED` = `2` — The plug-in is updated also when a drawing is opened. The plug-in is executed when the input is changed or during drawing opening.

## InputObjectDependencyAttribute (class)

The InputObjectDependencyAttribute class is used for storing an input object dependency which determines when the plug-in is updated in the system. The attribute is initialized from the custom attribute [InputObjectDependency(PluginBase.InputObjectDependency Type)] in the plug-in source.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/683d0814-ff48-9371-384c-8b8a2db73b46)

### Constructors
- `public InputObjectDependencyAttribute(int Type)` — The custom attribute [InputObjectDependency(PluginBase.InputObjectDependency Type)] uses this to store a dependency attribute which determines when the plug-in is updated in the system. This is not to be used by itself.
- `public InputObjectDependencyAttribute(PluginBase.InputObjectDependency Type)` — The custom attribute [InputObjectDependency(PluginBase.InputObjectDependency Type)] uses this to store a dependency attribute which determines when the plug-in is updated in the system. This is not to be used by itself.

### Properties
- `Type` (PluginBase.InputObjectDependency, get) — Returns the type of the input object dependency.

## InputObjectTypeAttribute (class)

The InputObjectTypeAttribute class is used for storing the type of the input. Based on the type the system can then correctly ask for user input in the creation. The attribute is initialized from the custom attribute [InputObjectType(ConnectionBase.InputObjectType.INPUTOBJECT_PART)] in the connection source.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/6d2240db-156e-3bf5-fd9d-10f29c411e8b)

### Constructors
- `public InputObjectTypeAttribute(int Type)` — The custom attribute [InputObjectType()] uses this to store the input object type to the system. This is not to be used by itself.
- `public InputObjectTypeAttribute(ConnectionBase.InputObjectType Type)` — The custom attribute [InputObjectType()] uses this to store the input object type to the system. This is not to be used by itself.

### Properties
- `Type` (ConnectionBase.InputObjectType, get) — The custom attribute [InputObjectType()] uses this to store the input object type to the system. This is not to be used by itself.

## PluginAttribute (class)

The PluginAttribute class is used for storing the name of the plug-in to the system. The attribute is initialized from the custom attribute [Plugin("PluginName")] in the plug-in source.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/19308e33-a3f9-4297-8963-b9532788d8cf)

### Constructors
- `public PluginAttribute(string name)` — The custom attribute [Plugin("PluginName")] uses this to store the name to the system. This is not to be used by itself.

### Properties
- `Name` (String, get) — The name of the plug-in in the system.

## PluginBase (class)

The PluginBase class in an abstract base class for model plug-ins. Model plug-ins have to be inherited from this class. Drawing plug-ins have to be inherited from the DrawingPluginBase. A plug-in is always executed in the plug-in's local coordinate system. The origin of the plug-in's coordinate system is defined based on the first input object or point. In case the first input is an object, the origin of the plug-in's coordinate system is the first input point of the object. In case the first input is a point, the origin of the plug-in's coordinate system is the input point. The X- and Y-axes of the coordinate system are defined in the current plane.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/5bd24240-26d1-39e2-9b6b-3f4556cdcd47)

### Methods
#### `public abstract List<PluginBase.InputDefinition> DefineInput()`

The method Tekla Structures calls for the plug-in to query the input. The plug-in must then return a list of input definition instances. The plug-in will be dependent on the items it returns. Dependent means that if any of these items change, for example the user moves the points, the plug-in will be re-run with new input. DefineInput is not called during the re-run, and thus all the actual implementation should be in the Run() method. The maximum number of InputDefinitions in the List is 10.

**Returns:** `List<PluginBase.InputDefinition>` — A list of input definition instances. If the plug-in is not dependent on input, it should return an empty list (not null).

[Docs](https://developer.tekla.com/topic/en/18/47/eb486b17-396b-4b6f-689d-a957db5b3658)

#### `public bool IsDefaultValue(double Value)`

Returns true if the given value is set to the default value for this type.

**Parameters:**
- `Value` (System.Double) — The value to test.

**Returns:** `Boolean` — True if the value is set to the default.

[Docs](https://developer.tekla.com/topic/en/18/47/4a0abd49-6b0a-2bb5-a8ab-17c6e5b55c75)

#### `public bool IsDefaultValue(int Value)`

Returns true if the given value is set to the default value for this type.

**Parameters:**
- `Value` (System.Int32) — The value to test.

**Returns:** `Boolean` — True if the value is set to the default.

[Docs](https://developer.tekla.com/topic/en/18/47/79f0b6e7-7744-bb26-15ab-c97861855732)

#### `public bool IsDefaultValue(string Value)`

Returns true if the given value is set to the default value (empty string).

**Parameters:**
- `Value` (System.String) — The value to test.

**Returns:** `Boolean` — True if the value is set to the default.

[Docs](https://developer.tekla.com/topic/en/18/47/8da27f11-11c1-871b-71c1-91c93d729bc0)

#### `public abstract bool Run(List<PluginBase.InputDefinition> Input)`

The main method of the plug-in. It is called after the input has been defined with DefineInput(). This is the "main" method of the plug-in and should contain all the actual implementation.

**Parameters:**
- `Input` (System.Collections.Generic.List<PluginBase.InputDefinition>) — A list of the same format and order as what was returned from DefineInput().

**Returns:** `Boolean` — True on success.

[Docs](https://developer.tekla.com/topic/en/18/47/5c8c1a5b-438a-7950-fd69-8b4935074ec6)

### Properties
- `Identifier` (Identifier, get) — The identifier of the executable plug-in instance.

## PluginBase.CoordinateSystemType (enum)

Defines the coordinate system type for model plug-ins which are using points as an input.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/440d2b29-3abf-36bf-37a4-4d5f845580b8)

### Values
- `FROM_FIRST_POINT_AND_GLOBAL` = `0` — Plugin coordinate system is calculated from first input point defining the origin and global coordinates which defines x and y-direction.
- `FROM_FIRST_AND_SECOND_POINT` = `1` — Plugin coordinate system is calculated from first input point defining the origin and second input point which defines x-direction.

## PluginBase.InputDefinition (class)

The InputDefinition class is a class for defining the plug-in dependency over the input (points or identifiers). The user implemented method DefineInput() of the PluginBase interface should return an array list of input definition instances. This defines the points and identifiers the plug-in will receive as input when the Run() method is called.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/5a24f6b6-72e1-ef1b-1dfd-c51d9b39e460)

### Constructors
- `public InputDefinition(ArrayList _Input)` — Creates a new input definition instance with multiple point or identifier instances.
- `public InputDefinition(Identifier ID)` — Creates a new input definition instance with one identifier.
- `public InputDefinition(Point Point)` — Creates a new input definition instance with one point.
- `public InputDefinition(Point P1, Point P2)` — Creates a new input definition instance with two points.

### Methods
#### `public Object GetInput()`

Returns the input Tekla Structures gave to the plug-in. The input is either a point instance, an identifier instance, an array list of points or an array list of identifiers. This is based on the input format returned from the DefineInput() method.

**Returns:** `Object` — The input to use in the Run() method.

[Docs](https://developer.tekla.com/topic/en/18/47/ea126692-d899-a70f-46e5-14e462635aa5)

#### `public PluginBase.InputDefinition.InputTypeEnum GetInputType()`

Returns the type of the input the current instance contains.

**Returns:** `PluginBase.InputDefinition.InputTypeEnum` — The type of the input.

[Docs](https://developer.tekla.com/topic/en/18/47/4262e736-6d4a-89ba-0950-5e1d7902a34e)

## PluginBase.InputDefinition.InputTypeEnum (enum)

The possible input types for a plug-in.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/fec18db9-7adc-f90c-030c-1318e59223ff)

### Values
- `INPUT_ONE_POINT` = `1` — The input is one point (x,y,z).
- `INPUT_TWO_POINTS` = `2` — The input is two points.
- `INPUT_POLYGON` = `3` — The input is multiple points.
- `INPUT_ONE_OBJECT` = `4` — The input is one ID of any type.
- `INPUT_N_OBJECTS` = `5` — The input is N IDs of any type.

## PluginBase.InputObjectDependency (enum)

Defines the input object dependency type for model plug-ins.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/9b621f28-934e-90ec-2996-fac877e7de20)

### Values
- `NOT_DEPENDENT` = `0` — No dependency on input. Typically this type is applied to the import and export functionality. Not-dependent plug-ins are executed from the plug-in dialog instead of the component catalog. The created objects do not have any relation to the plug-in anymore. The plug-in dialog cannot be opened from the created objects. Note! The system does not automatically save a rollback point to the database for non-dependent plug-ins. If new objects are created, CommitChanges needs to be called at the end of the Run method.
- `DEPENDENT` = `1` — Dependent on input. The plug-in is executed if e.g. the definition points or the profile of the input part change. Boolean operations to the input part do not cause plug-in execution.
- `GEOMETRICALLY_DEPENDENT` = `2` — Geometrically dependent on input. The plug-in is executed if the input part geometry changes i.e. if the input part is fitted.
- `NOT_DEPENDENT_MODIFIABLE` = `3` — No dependency on input but the instance is modifiable in the model. The created objects have a relation to the plug-in. The plug-in dialog can be opened from the created objects.

## PluginBase.SymbolVisibility (enum)

Defines the symbol visibility for model plug-ins in model views.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/277a0471-12da-93d9-7307-c550a3086201)

### Values
- `DRAW_SYMBOL` = `0` — Symbol is visible in model view.
- `DO_NOT_DRAW_SYMBOL` = `1` — Symbol is not drawn to model view. Plug-in must be selected from the view using created objects only.

## PluginCoordinateSystemAttribute (class)

The PluginCoordinateSystemAttribute class is used for defining the coordinate system type for model plug-ins which are using points as an input. The attribute is initialized from the custom attribute [PluginCoordinateSystem(PluginBase.CoordinateSystemType Type)] in the plug-in source. If attribute is not defined or attribute has value PluginBase.CoordinateSystemType.FROM_FIRST_POINT_AND_GLOBAL coordinate system is calculated as earlier. The attribute is used if the first Inputdefinition object contains at least two points (defined by two points, line, face or polygon).

[Vendor docs](https://developer.tekla.com/topic/en/18/47/4e22e521-31d7-14cd-9322-b5e585d3cdee)

### Constructors
- `public PluginCoordinateSystemAttribute(int Type)` — The custom attribute used to define coordinate system of the plug-in. This is not to be used by itself.
- `public PluginCoordinateSystemAttribute(PluginBase.CoordinateSystemType Type)` — The custom attribute used to define coordinate system of the plug-in. This is not to be used by itself.

### Properties
- `Type` (PluginBase.CoordinateSystemType, get) — Returns the type of the input object dependency.

## PluginDescriptionAttribute (class)

Not supported at the moment.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/3ae73917-956a-f7d8-520c-a60ea15ce79d)

### Constructors
- `public PluginDescriptionAttribute(string language, string description)` — Not supported at the moment.

### Properties
- `Description` (String, get) — Not supported at the moment.
- `Language` (String, get) — Not supported at the moment.

## PluginNameAttribute (class)

Not supported at the moment.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/64260160-d2fd-baf5-377c-d4a0318a3def)

### Constructors
- `public PluginNameAttribute(string language, string name)` — Not supported at the moment.

### Properties
- `Language` (String, get) — Not supported at the moment.
- `Name` (String, get) — Not supported at the moment.

## PluginPropertyFileLocationAttribute (class)

Used to specify file location for a property file.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/23b0191a-e544-1e01-af19-aa2b3bb67a70)

### Constructors
- `public PluginPropertyFileLocationAttribute(string suffix, string subdirectory)` — Initializes a new instance of the PluginPropertyFileLocationAttribute class, for a file with provided filename suffix.

### Properties
- `Subdirectory` (String, get) — Gets the property file subdirectory in a model folder.
- `Suffix` (String, get) — Gets the property file suffix.

## PluginSymbolVisiblityAttribute (class)

The PluginSymbolVisiblityAttribute class is used for defining the symbol visibility for model plug-ins. The attribute is initialized from the custom attribute [PluginSymbolVisiblity(PluginBase.SymbolVisibility Type)] in the plug-in source. If attribute is not defined or attribute has value PluginBase.SymbolVisibility.DRAW_SYMBOL plug-in symbol is drawn normally.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/34a54f53-8840-15d4-9cc6-d7a746bc3f49)

### Constructors
- `public PluginSymbolVisiblityAttribute(int Type)` — The custom attribute used to define coordinate system of the plug-in. This is not to be used by itself.
- `public PluginSymbolVisiblityAttribute(PluginBase.SymbolVisibility Type)` — The custom attribute used to define coordinate system of the plug-in. This is not to be used by itself.

### Properties
- `Type` (PluginBase.SymbolVisibility, get) — Returns the type of the input object dependency.

## PluginUserInterfaceAttribute (class)

The PluginUserInterfaceAttribute class is used for storing the description of the plug-in user interface to the system. The attribute is initialized from the custom attribute [PluginUserInterface(PluginName.UserInterfaceDefinitions.Plugin1)] in the plug-in source. In this case the description string Plugin1 is a dialog written in inp format. If the plug-in dialog is inherited from the PluginFormBase the description string contains a class name of the dialog i.e. [PluginUserInterface("Model_Plug_in1.MainForm")].

[Vendor docs](https://developer.tekla.com/topic/en/18/47/48e673e5-d34c-6ed9-ce45-1a1d5a444898)

### Constructors
- `public PluginUserInterfaceAttribute(string description)` — The custom attribute [PluginUserInterface(PluginName.UserInterfaceDefinitions.Plugin1)] uses this to store the description of the plug-in user interface to the system. This is not to be used by itself.

### Properties
- `Description` (String, get) — The description of the plug-in user interface in inp format.

## PositionTypeAttribute (class)

The PositionTypeAttribute class is used for storing the position type of the connection. Based on the type the system will then position the connection in the creation. The attribute is initialized from the custom attribute [PositionType(PositionTypeEnum.COLLISION_PLANE)] in the connection source.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/85869afb-56cb-102f-dc01-3c8aa94b473c)

### Constructors
- `public PositionTypeAttribute(int Type)` — The custom attribute [PositionType()] uses this to store the position type to the system. This is not to be used by itself.
- `public PositionTypeAttribute(PositionTypeEnum Type)` — The custom attribute [PositionType()] uses this to store the position type to the system. This is not to be used by itself.

### Properties
- `Type` (PositionTypeEnum, get) — The custom attribute [PositionType()] uses this to store the position type to the system. This is not to be used by itself.

## SeamInputTypeAttribute (class)

The SeamInputTypeAttribute class is used for identifying that the connection is a seam and defining the input type. Based on the type the system will then ask for the correct number of input points in the creation. The attribute is initialized from the custom attribute [SeamInputType(ConnectionBase.SeamInputType.INPUT_POLYGON)] in the connection source.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/d4a9bbb6-876c-df62-b60a-424ef66dd3c0)

### Constructors
- `public SeamInputTypeAttribute(int Type)` — The custom attribute [SeamInputType()] uses this to store the input type to the system. This is not to be used by itself.
- `public SeamInputTypeAttribute(ConnectionBase.SeamInputType Type)` — The custom attribute [SeamInputType()] uses this to store the input type to the system. This is not to be used by itself.

### Properties
- `Type` (ConnectionBase.SeamInputType, get) — The custom attribute [SeamInputType()] uses this to store the input type to the system. This is not to be used by itself.

## SecondaryTypeAttribute (class)

The SecondaryTypeAttribute class is used for storing the number of needed secondaries in the connection. Based on the type the system can then correctly ask for user input in the creation. The attribute is initialized from the custom attribute i.e. [SecondaryType(ConnectionBase.SecondaryType.SECONDARYTYPE_ONE)] in the connection source.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/babb5ce4-8ff1-4e7a-5190-f4ea99b35406)

### Constructors
- `public SecondaryTypeAttribute(int Type)` — The custom attribute [SecondaryType()] uses this to store the secondary type to the system. This is not to be used by itself.
- `public SecondaryTypeAttribute(ConnectionBase.SecondaryType Type)` — The custom attribute [SecondaryType()] uses this to store the secondary type to the system. This is not to be used by itself.

### Properties
- `Type` (ConnectionBase.SecondaryType, get) — The custom attribute [SecondaryType()] uses this to store the secondary type to the system. This is not to be used by itself.

## StructuresFieldAttribute (class)

The StructuresFieldAttribute class is used for mapping a database attribute to a data field that the plug-in uses in execution. The attribute is initialized from the custom attribute [StructuresField(attributeName)] in the plug-in source. In the plug-in the data field must be public and the type must be double, integer or string.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/1db3e6b8-6d5a-3d89-28df-6ff41b96fa92)

### Constructors
- `public StructuresFieldAttribute(string attributeName)` — The custom attribute [StructuresField(attributeName)] uses this to map a database attribute to a data field that the plug-in uses in execution. This is not to be used by itself.

### Properties
- `AttributeName` (String, get) — The name of the attribute.

