---
name: tekla-tekla-structures-catalogs
description: This skill encodes the tekla 2026.0 surface of the Tekla.Structures.Catalogs namespace — 53 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: BoltItem, BoltItemEnumerator, BoltName, CatalogHandler, CatalogItemEnumeratorInitializationException, ComponentItem, ComponentItemEnumerator, CrossSection, and 45 more types.
---

# Tekla.Structures.Catalogs

Auto-generated from vendor docs for tekla 2026.0. 53 types in this namespace.

## BoltItem (class)

The BoltItem class contains information about the bolts in the Tekla Structures bolt catalog.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/e9c3c616-26e8-f827-b96f-592aa2724c60)

### Constructors
- `public BoltItem()` — Creates a new bolt item instance.

### Methods
#### `public bool ExportBoltStandard(ref string filename)`

Exports the bolt item standard + needed bolt catalog items in *.bass-format to the to given file name. If path is not given bolt standard + items are exported to model folder. If filename is empty bolt standard name is used as filename.

**Parameters:**
- `filename` (System.String.) — The export file name of the bolt assembly.

**Returns:** `Boolean` — True on success.

[Docs](https://developer.tekla.com/topic/en/18/47/742798eb-02a0-3b83-ef22-62f4ee2ef9d3)

### Properties
- `Lengths` (List<Double>, get) — The bolt item's length values.
- `Size` (Double, get/set) — The bolt item's size.
- `Standard` (String, get/set) — The bolt item's grade.
- `Type` (BoltItem.BoltItemTypeEnum, get/set) — The bolt item's type.

## BoltItem.BoltItemTypeEnum (enum)

Defines the different bolt item types.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/0b606ef0-2e8e-2527-2eac-63e2f9c94b04)

### Values
- `BOLT_UNKNOWN` = `0` — The unknown bolt item type.
- `BOLT` = `1` — The bolt type.
- `STUD` = `2` — The stud type.

## BoltItemEnumerator (class)

The BoltItemEnumerator class allows to loop through the bolt catalog items.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/1b842d05-50f4-8862-42f7-703fdaa505a0)

### Methods
#### `public int GetSize()`

Returns the total amout of items.

**Returns:** `Int32` — The total amount of items.

[Docs](https://developer.tekla.com/topic/en/18/47/e6c7329c-43d2-5c89-1c91-89a81813f37e)

#### `public bool MoveNext()`

Moves to the next item in the enumerator.

**Returns:** `Boolean` — False on failure.

[Docs](https://developer.tekla.com/topic/en/18/47/d7ef7d06-eed2-5dc8-3024-0097c5d7e8bd)

#### `public void Reset()`

Resets the enumerator to the beginning.

[Docs](https://developer.tekla.com/topic/en/18/47/a42dea55-ef04-8bc3-cefb-aa00d6c1cc52)

### Properties
- `Current` (BoltItem, get) — Returns a bolt item instance of the current element.

## BoltName (class)

The BoltName class contains the name of the bolt item.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/14ea55d1-ede5-e746-1b99-fc5debac1fcc)

### Constructors
- `public BoltName()` — Initializes a new instance of the BoltName class

### Properties
- `Name` (String, get/set) — The bolt item name.

## BrepType (enum)

Shape catalog renewal makes it possible to organize shapes in the shape catalog compared to a plain list before. Thus, it is now possible and required to distinguish shapes in such a manner that: BuildingProduct represent shapes that users can import from, for example, Tekla Warehouse and insert them into a model. These are usually provided by the manufacturer and not detailed by users themselves. StructuralShape represent shapes that get created by "Convert part to item" or from geometry editing.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/2bf93ab8-7617-e2d7-910b-3baf23087211)

### Values
- `BuildingProduct` = `0` — Default value if the flag doesn't exist for the shape. It is also assigned in case of shape import.
- `StructuralShape` = `1` — StructuralShape is assigned in case a shape is created by "Convert part to item", or on shape edit and save.

## CatalogHandler (class)

The CatalogHandler class is a class from which the user can query catalog instances.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/9335ffbe-aadb-8919-93a2-6622af8b3efa)

### Constructors
- `public CatalogHandler()` — Creates a new catalog handler instance.

### Methods
#### `public bool ExportProfileItems(IList<string> profileNames, string path, string fileName)`

Exports the profile items in the profile database to given path and file name. Currently library profiles, sketch profiles and clb profiles are supported. Library profiles are exported to *.lis format. Sketch profiles are exported to *.uel format. Clb profiles are exported to *.clb format. If profiles are not available, export fails. If path is empty, export fails. If file name is empty, export fails.

**Parameters:**
- `profileNames` (System.Collections.Generic.IList<String>)
- `path` (System.String)
- `fileName` (System.String)

**Returns:** `Boolean` — True on success.

[Docs](https://developer.tekla.com/topic/en/18/47/0d367359-7a89-53c0-5a76-33930fd78ba8)

#### `public BoltItemEnumerator GetBoltItems()`

Returns an enumerator of all bolt items.

**Returns:** `BoltItemEnumerator` — A BoltItemEnumerator of all bolt items.

[Docs](https://developer.tekla.com/topic/en/18/47/564d4405-0cb8-bd00-36fe-8dadcfd8d443)

#### `public ComponentItemEnumerator GetComponentItems()`

Returns an enumerator of all component items.

**Returns:** `ComponentItemEnumerator` — A ComponentItemEnumerator of all component items.

[Docs](https://developer.tekla.com/topic/en/18/47/5276c15a-19f5-9538-0b02-9a5f1aab35c4)

#### `public bool GetConnectionStatus()`

Returns true if a proper connection to the Tekla Structures process has been established. Currently, there's no way to re-establish the connection.

**Returns:** `Boolean` — True on success.

[Docs](https://developer.tekla.com/topic/en/18/47/c78555d3-7a00-e1ee-f7d8-13efac5e8f80)

#### `public DrawingItemEnumerator GetDrawingItems()`

Returns an enumerator of all drawing items.

**Returns:** `DrawingItemEnumerator` — A DrawingItemEnumerator of all drawing catalog items.

[Docs](https://developer.tekla.com/topic/en/18/47/2f10f848-08c7-a9ef-5b20-5d05465066c5)

#### `public ProfileItemEnumerator GetLibraryProfileItems()`

Returns an enumerator of library profile items.

**Returns:** `ProfileItemEnumerator` — A ProfileItemEnumerator of library profile items.

[Docs](https://developer.tekla.com/topic/en/18/47/5451d58b-692d-f5b5-141a-100e2751bc08)

#### `public MaterialItemEnumerator GetMaterialItems()`

Returns an enumerator of all material items.

**Returns:** `MaterialItemEnumerator` — A MaterialItemEnumerator of all material items.

[Docs](https://developer.tekla.com/topic/en/18/47/1f0b154b-381e-3cb4-8c28-c942c520b5b3)

#### `public MaterialMarketSizesItemEnumerator GetMaterialMarketSizes()`

Returns an enumerator of all material marketsizes. Data is read from XS_PROFDB\marketsizes.dat file.

**Returns:** `MaterialMarketSizesItemEnumerator` — A MaterialMarketSizesItemEnumerator of all material marketsizes.

[Docs](https://developer.tekla.com/topic/en/18/47/80ddd345-75e3-9778-89c8-4674603e5510)

#### `public MeshItemEnumerator GetMeshItems()`

Returns an enumerator of mesh items.

**Returns:** `MeshItemEnumerator` — A MeshItemEnumerator of mesh items.

[Docs](https://developer.tekla.com/topic/en/18/47/0f63844e-2f69-8886-9a11-0a7e7c5a0dde)

#### `public ProfileItemEnumerator GetParametricProfileItems()`

Returns an enumerator of parametric profile items.

**Returns:** `ProfileItemEnumerator` — A ProfileItemEnumerator of parametric profile items.

[Docs](https://developer.tekla.com/topic/en/18/47/80fe7547-568c-5ccb-62f4-cd419ba5727b)

#### `public PrinterItemEnumerator GetPrinterItems()`

Returns an enumerator of all printer items.

**Returns:** `PrinterItemEnumerator` — A PrinterItemEnumerator of all printer items.

[Docs](https://developer.tekla.com/topic/en/18/47/db6c490a-4382-88c7-b00b-cebdd305a780)

#### `public ProfileItemEnumerator GetProfileItems()`

Returns an enumerator of all profile items.

**Returns:** `ProfileItemEnumerator` — A ProfileItemEnumerator of all profile items.

[Docs](https://developer.tekla.com/topic/en/18/47/bcb35b79-bdf0-dcf1-3f3a-c7a0b297743b)

#### `public RebarItemEnumerator GetRebarItems()`

Returns an enumerator of rebar items.

**Returns:** `RebarItemEnumerator` — A RebarItemEnumerator of rebar items.

[Docs](https://developer.tekla.com/topic/en/18/47/1eed9080-8844-eb0b-1952-943d0af5b563)

#### `public ShapeItemEnumerator GetShapeItems()`

Returns an enumerator of all shape items.

**Returns:** `ShapeItemEnumerator` — A ShapeItemEnumerator of all shape items.

[Docs](https://developer.tekla.com/topic/en/18/47/3b509288-e37f-d537-5416-6cbb7b9b60d6)

#### `public static IEnumerable<FinishItem> GetSteelFinishItems()`

Returns the collection of all steel finish items.

**Returns:** `IEnumerable<FinishItem>` — The collection of all steel finish items.

[Docs](https://developer.tekla.com/topic/en/18/47/992a8408-b4c2-c50f-895a-ea876c4c7333)

#### `public UserPropertyItemEnumerator GetUserPropertyItems()`

Returns an enumerator of all user property items.

**Returns:** `UserPropertyItemEnumerator` — A UserPropertyItemEnumerator of all user property items.

[Docs](https://developer.tekla.com/topic/en/18/47/e982db4c-daf4-1161-db5e-6a4688c724db)

#### `public UserPropertyItemEnumerator GetUserPropertyItems(CatalogObjectTypeEnum objectType)`

Returns an enumerator of user property items of the given object type.

**Parameters:**
- `objectType` (Tekla.Structures.Catalogs.CatalogObjectTypeEnum) — The object type to be used.

**Returns:** `UserPropertyItemEnumerator` — A UserPropertyItemEnumerator of user property items of the given object type.

[Docs](https://developer.tekla.com/topic/en/18/47/17a6222e-f487-de7e-aa78-37d6283209bb)

#### `public bool ImportBoltItems(string path)`

Imports bolt items and their assembly information (*.bass) from folder to bolt assembly catalog and bolt catalog. Import overrides the existing bolts in the catalog without any warnings.

**Parameters:**
- `path` (System.String) — The path to be used for import.

**Returns:** `Boolean` — true if successful.

[Docs](https://developer.tekla.com/topic/en/18/47/8b3e21a7-4d9e-b8d5-c524-df1235bac4c8)

#### `public bool ImportCustomComponentItems(string path)`

Imports custom component items (*.uel) from path to component catalog. Import overrides the existing custom components in the catalog without any warnings.

**Parameters:**
- `path` (System.String) — The path to be used for import. This can be a uel file or a folder containing one or more uel files.

**Returns:** `Boolean` — true if successful.

[Docs](https://developer.tekla.com/topic/en/18/47/bd6a584e-e1b5-ca68-8032-00d23479b3fc)

#### `public bool ImportDrawingItems(string path)`

Imports drawing setting items (*.tsds) from specified folder to model folder. Import throws exception if model is not open. Import overrides the existing drawing items in the catalog without any warnings.

**Parameters:**
- `path` (System.String) — The path to be used for import.

**Returns:** `Boolean` — true if successful.

[Docs](https://developer.tekla.com/topic/en/18/47/830a96bd-0a99-3085-ec1c-af48386c4ea7)

#### `public bool ImportLibraryProfileItems(string path)`

Imports library profile items (*.lis) from folder to profile catalog. Import overrides the existing profiles in the catalog without any warnings.

**Parameters:**
- `path` (System.String) — The path to be used for import.

**Returns:** `Boolean` — true if successful.

[Docs](https://developer.tekla.com/topic/en/18/47/78798d1e-e2a8-84cc-f7e5-16c62fe595b8)

#### `public bool ImportMaterialItems(string path)`

Imports material items (*.lis) from folder to material catalog. Import overrides the existing materials in the catalog without any warnings.

**Parameters:**
- `path` (System.String) — The path to be used for import.

**Returns:** `Boolean` — true if successful.

[Docs](https://developer.tekla.com/topic/en/18/47/e2360d79-0bb9-5a37-c03c-1111f1cf6191)

#### `public bool ImportMeshItems(string path)`

Imports mesh items (*.mexp) from folder to mesh catalog. Import overrides the existing mesh item in the catalog without any warnings.

**Parameters:**
- `path` (System.String) — The path to be used for import.

**Returns:** `Boolean` — true if successful.

[Docs](https://developer.tekla.com/topic/en/18/47/7fa2e012-8246-ec8e-9905-15f25a27102d)

#### `public bool ImportParametricProfileItems(string path)`

Imports parametric profile items (*.uel) from folder to profile catalog. Import overrides the existing profiles in the catalog without any warnings.

**Parameters:**
- `path` (System.String) — The path to be used for import.

**Returns:** `Boolean` — true if successful.

[Docs](https://developer.tekla.com/topic/en/18/47/3f5bc8b2-8d3f-37ee-260c-b0ad223a29e5)

#### `public bool ImportRebarItems(string path)`

Imports rebar items (*.rexp) from folder to reinforcement catalog. Import overrides the existing rebar item in the catalog without any warnings.

**Parameters:**
- `path` (System.String) — The path to be used for import.

**Returns:** `Boolean` — true if successful.

[Docs](https://developer.tekla.com/topic/en/18/47/7004c954-a82e-7a40-1e9d-c5af25142caf)

#### `public bool ImportShapeItems(string path)`

Imports shape geometry items (*.tsc) from specified folder. Import throws exception if model is not open. Import overrides the existing shape items in the catalog without any warnings.

**Parameters:**
- `path` (System.String) — The path to be used for import.

**Returns:** `Boolean` — true if successful.

[Docs](https://developer.tekla.com/topic/en/18/47/148728ab-d5fc-ce70-4dee-f6005e6f7f1f)

#### `public bool ImportShapeItemsFromGeometryFiles(string path)`

Imports shape geometry items from geometry files in specified folder. Import supports following file formats: ".skp", ".dxf", ".dwg", ".step", ".iges", ".stp", ".igs", ".ifc", ".dgn", ".trb" File name is used as shape name. Import throws exception if model is not open. Import overrides the existing shape items in the catalog without any warnings.

**Parameters:**
- `path` (System.String) — The path to be used for import.

**Returns:** `Boolean` — true if successful.

[Docs](https://developer.tekla.com/topic/en/18/47/48449125-2fd4-5422-80f9-62bc46bfdf2a)

#### `public bool SaveProfileDatabase()`

Saves the profile database.

**Returns:** `Boolean` — True on success.

[Docs](https://developer.tekla.com/topic/en/18/47/59394a51-ded4-984c-f616-31fdb8599db7)

## CatalogItemEnumeratorInitializationException (class)

The CatalogItemEnumeratorInitializationException class represents an error that occurred during the catalog item enumerator initialization. This class cannot be inherited.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/17d12340-b3ce-add9-a3e8-10911fbc6b86)

## CatalogObjectTypeEnum (enum)

The catalog object type.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/69665cba-e22d-fad0-277b-377c99a3766a)

### Values
- `PART` = `1` — The part type.
- `STEEL_BEAM` = `2` — The steel beam type.
- `STEEL_COLUMN` = `3` — The steel column type.
- `STEEL_ORTHOGONAL_BEAM` = `4` — The steel orthogonal beam type.
- `STEEL_TWIN_PROFILE_BEAM` = `5` — The steel twin profile beam type.
- `STEEL_CONTOUR_PLATE` = `6` — The steel contour plate type.
- `STEEL_FOLDED_PLATE` = `7` — The steel folded plate type.
- `CONCRETE_BEAM` = `8` — The concrete beam type.
- `CONCRETE_COLUMN` = `9` — The concrete column type.
- `CONCRETE_PAD_FOOTING` = `10` — The concrete pad footing type.
- `CONCRETE_STRIP_FOOTING` = `11` — The concrete strip footing type.
- `CONCRETE_PANEL` = `12` — The concrete panel type.
- `CONCRETE_SLAB` = `13` — The concrete slab type.
- `REINFORCING_BAR` = `14` — The reinforcing bar type.
- `SURFACING` = `15` — The surfacing type.
- `WELD` = `16` — The weld type.
- `BOLT` = `17` — The bolt type.
- `STEEL_ASSEMBLY` = `18` — The steel assembly type.
- `PRECAST_CONCRETE_ASSEMBLY` = `19` — The precast concrete assembly type.
- `INSITU_CONCRETE_ASSEMBLY` = `20` — The in situ concrete assembly type.
- `POUR_OBJECT` = `21` — The pour object type.
- `POUR_BREAK` = `22` — The pour break type.
- `GRID` = `23` — The grid type.
- `PROJECT` = `24` — The project type.
- `PHASE` = `25` — The phase type.
- `TASK` = `26` — The task type.
- `REFERENCE_MODEL` = `27` — The reference model type.
- `REFERENCE_MODEL_OBJECT` = `28` — The reference model object type.
- `SINGLE_PART_DRAWING` = `29` — The single part drawing type.
- `ASSEMBLY_DRAWING` = `30` — The assembly drawing type.
- `GA_DRAWING` = `31` — The general arrangement drawing type.
- `MULTI_DRAWING` = `32` — The multidrawing type.
- `CAST_UNIT_DRAWING` = `33` — The cast unit drawing type.
- `BENT_PLATE` = `34` — The bent plate type.
- `STEEL_BREP_PART` = `35` — The steel brep part type.
- `CONCRETE_BREP_PART` = `36` — The concrete brep part type.
- `CHAMFER_OBJECT` = `37` — The chanfer object type.
- `SURFACE_OBJECT` = `38` — The surface object type.
- `GRID_PLANE` = `39` — The grid plane type.
- `STEEL_SPIRAL_BEAM` = `40` — The steel helix part type.
- `CONCRETE_SPIRAL_BEAM` = `41` — The concrete helix part type.
- `POUR_UNIT` = `42` — The pour unit type.
- `STEEL_LOFTED_PLATE` = `43` — The steel lofted plate part type.
- `CONCRETE_LOFTED_SLAB` = `44` — The concrete lofted slab part type.
- `BUILDING_SPACE` = `45` — The building space object type.
- `BUILDING` = `46` — The building object type.
- `BUILDING_SECTION` = `47` — The building section object type.
- `STOREY` = `48` — The storey object type.

## ComponentItem (class)

The ComponentItem class contains information about the components in the Tekla Structures catalog.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/5f8f7adf-d58e-527e-3ae1-9186855858e2)

### Constructors
- `public ComponentItem()` — Creates a new component item instance.

### Methods
#### `public virtual bool Export(ref string filename)`

Exports the custom component item in *.uel-format to the to given file name. If path is not given custom component is exported to model folder. If filename is empty custom component name is used as filename.

**Parameters:**
- `filename` (System.String.) — The export file name of the component item.

**Returns:** `Boolean` — True on success.

[Docs](https://developer.tekla.com/topic/en/18/47/5a13c8a3-e8c0-e8b3-bab9-cabb1b4920d6)

#### `public bool GetVersion(ref int version)`

Gets the version number of custom component item from the component database.

**Parameters:**
- `version` (System.Int32.) — The version number of the component item.

**Returns:** `Boolean` — Version number on success, int.MinValue if not found.

[Docs](https://developer.tekla.com/topic/en/18/47/d3c4303c-4fa1-634e-27ed-c32be2d98431)

#### `public bool Select(string name, int number)`

Selects the component item from the component database. Uses the type if defined for selection

**Parameters:**
- `name` (System.String) — The name of the component item to select.
- `number` (System.Int32) — The number of the component item to select.

**Returns:** `Boolean` — True on success.

[Docs](https://developer.tekla.com/topic/en/18/47/c9fde42f-cebc-4165-fa52-2411ac2e5931)

#### `public bool Select(string name, int number, ComponentItem.ComponentTypeEnum type)`

Selects the component item from the component database.

**Parameters:**
- `name` (System.String) — The name of the component item to select.
- `number` (System.Int32) — The number of the component item to select.
- `type` (Tekla.Structures.Catalogs.ComponentItem.ComponentTypeEnum) — The type of the component item to select.

**Returns:** `Boolean` — True on success.

[Docs](https://developer.tekla.com/topic/en/18/47/0c351379-98c4-2fe0-ac3e-def043ea1d3f)

### Properties
- `AttributeFileExtension` (String, get) — Gets the attribute file extension of item from the component database.
- `Name` (String, get/set) — The component item's internal name which is used by Tekla Structures in component identification.
- `Number` (Int32, get/set) — The component item's internal number which is used by Tekla Structures in component identification.
- `Type` (ComponentItem.ComponentTypeEnum, get/set) — The component item's type.
- `UIName` (String, get/set) — The component item's name which is visible in the Tekla Structures user interface.

## ComponentItem.ComponentTypeEnum (enum)

Defines the different component types.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/87a65766-7edb-e0cb-d9be-47d432900e45)

### Values
- `UNKNOWN` = `0` — The unknown component type.
- `CONNECTION` = `24` — The component type is connection.
- `COMPONENT` = `25` — The component type is component.
- `SEAM` = `26` — The component type is seam.
- `DETAIL` = `27` — The component type is detail.
- `CUSTOM_PART` = `43` — The component type is custom part object.
- `DRAWING_PLUGIN` = `38` — The component type is drawing plugin.

## ComponentItemEnumerator (class)

The ComponentItemEnumerator class allows to loop through the component catalog items.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/56616cbe-13dc-2aee-eb1e-1438529de663)

### Methods
#### `public int GetSize()`

Returns the total amout of items.

**Returns:** `Int32` — The total amount of items.

[Docs](https://developer.tekla.com/topic/en/18/47/db63047f-fe9b-99cf-200b-c0f83bb909b6)

#### `public bool MoveNext()`

Moves to the next item in the enumerator.

**Returns:** `Boolean` — False on failure.

[Docs](https://developer.tekla.com/topic/en/18/47/62006a3f-6909-42e1-a397-62eaeb85c44f)

#### `public void Reset()`

Resets the enumerator to the beginning.

[Docs](https://developer.tekla.com/topic/en/18/47/458e0c09-e8ec-7988-92a2-b14462fee30b)

### Properties
- `Current` (ComponentItem, get) — Returns a component item instance of the current element.

## CrossSection (class)

The CrossSection class defines a with cross section points

[Vendor docs](https://developer.tekla.com/topic/en/18/47/35f56e00-3ebc-f537-971a-78530f8315dc)

### Constructors
- `public CrossSection(string ProfileString)` — Creates a new cross section point instance.
- `public CrossSection(ProfileName Profile)` — Creates a new cross section point instance.

### Methods
#### `public List<Polycurve> GetInnerContours()`

Gets the inner loop geometries as Polycurve.

**Returns:** `List<Polycurve>` — List

[Docs](https://developer.tekla.com/topic/en/18/47/ce3e7313-3691-0e3b-9ea3-f73187e2761f)

#### `public Polycurve GetOuterContour()`

Gets the outer loop geometry as Polycurve.

**Returns:** `Polycurve` — Polycurve

[Docs](https://developer.tekla.com/topic/en/18/47/403e20e9-8043-3c61-b70b-bbba11e3387a)

#### `public bool Select(bool HighAccuracy, double Location = 0, double Length = 1000)`

Selects the cross section from the profile definition.

**Parameters:**
- `HighAccuracy` (System.Boolean) — The geometric accuracy level.
- `Location` (System.Double) — The location from where cross section is fetched.
- `Length` (System.Double) — The total length of the profile.

**Returns:** `Boolean` — True in success.

[Docs](https://developer.tekla.com/topic/en/18/47/bd26d42e-ce1b-40d8-f039-3c8620dfd204)

#### `public bool Select(double Location = 0, double Length = 1000)`

Selects the cross section from the profile definition.

**Parameters:**
- `Location` (System.Double) — The location from where cross section is fetched.
- `Length` (System.Double) — The total length of the profile.

**Returns:** `Boolean` — True in success.

[Docs](https://developer.tekla.com/topic/en/18/47/899646b5-12f5-a29e-511a-08e4ac3bfb6f)

### Properties
- `HighAccuracy` (Boolean, get) — The contour geometry accuracy.
- `InnerSurfacePoints` (List<List<Point>>, get) — Gets the points for inner surfaces.
- `InnerSurfaces` (List<List<CrossSectionPoint>>, get) — Gets the cross section points for inner surfaces.
- `Length` (Double, get) — The total length
- `Location` (Double, get) — The location in relation to length, 0.0 referring to start of the profile
- `OuterSurface` (List<CrossSectionPoint>, get) — Gets the cross section points for outer surface.
- `OuterSurfacePoints` (List<Point>, get) — Gets the cross section points for outer surface.
- `Profile` (ProfileName, get/set) — The profile where cross section is asked from

## CrossSectionPoint (class)

The CrossSectionPoint class defines a point with possible chamfering information

[Vendor docs](https://developer.tekla.com/topic/en/18/47/86912ba9-9e66-d9a2-21a0-9fb0e808f7ae)

### Constructors
- `public CrossSectionPoint()` — Creates a new cross section point instance.

### Methods
#### `public int CompareTo(Object obj)`

Compares two points. To use binarysearch somekind of sorting should be used.

**Parameters:**
- `obj` (System.Object) — The point to be compared.

**Returns:** `Int32` — 0 if both are equal, -1 if this point is before, 1 if this point is after.

[Docs](https://developer.tekla.com/topic/en/18/47/2f202828-05fd-220f-92b1-5b64372a4f7e)

#### `public override bool Equals(Object obj)`

Returns true if the current object and the given object are equal.

**Parameters:**
- `obj` (System.Object) — The object we wish to check the equality with.

**Returns:** `Boolean` — True if the given object equals the current object.

[Docs](https://developer.tekla.com/topic/en/18/47/dbca17f2-6730-e2c4-583c-90c74c72e0b0)

#### `public override int GetHashCode()`

Returns a hash code for the point. Notice, in extremely rare cases, you might not get the same hash code for two points even though they are considered equal! This should, however, happen only in extremely rare cases!

**Returns:** `Int32` — The hash code for the point.

[Docs](https://developer.tekla.com/topic/en/18/47/9f84f024-170b-5181-770d-5ed2f56332cd)

#### `public override string ToString()`

Formats the point into a string, using the current culture.

**Returns:** `String` — The string that represents the point.

[Docs](https://developer.tekla.com/topic/en/18/47/63db29a6-32f7-6696-31eb-a6084dcb62cd)

#### `public string ToString(CultureInfo cultureInfo)`

Formats the point into a string, using the provided culture.

**Parameters:**
- `cultureInfo` (System.Globalization.CultureInfo)

**Returns:** `String` — The string that represents the point.

[Docs](https://developer.tekla.com/topic/en/18/47/176a7423-000a-5fb0-dab9-659b87bf1505)

#### `public virtual void Translate(double X, double Y, double Z)`

Translates the point using the given vector.

**Parameters:**
- `X` (System.Double) — The X-translation to be used.
- `Y` (System.Double) — The Y-translation to be used.
- `Z` (System.Double) — The Z-translation to be used.

[Docs](https://developer.tekla.com/topic/en/18/47/7ad7ecb9-acf3-7ed2-f2bc-aa52b99ceae6)

#### `public void Zero()`

Zeros all the members of the point.

[Docs](https://developer.tekla.com/topic/en/18/47/9bf63e0c-fe71-40ae-e467-306f0a41f315)

### Properties
- `Chamfer` (Chamfer, get/set) — The chamfer for the cross section point.

## DrawingItem (class)

The DrawingItem class contains information from the drawings in the master drawing catalog.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/9c95f68b-b566-e073-66a1-6eb6016b5da4)

### Constructors
- `public DrawingItem()` — Creates a new drawing item instance.

### Methods
#### `public bool Export(ref string filename)`

Exports the drawing item in the catalog to given file name. If path is not given drawing is exported to model folder. If filename is empty drawing name is used as filename.

**Parameters:**
- `filename` (System.String.) — The export file name of the drawing item.

**Returns:** `Boolean` — True on success.

[Docs](https://developer.tekla.com/topic/en/18/47/e73e32a2-21fa-37e6-2d50-dba9af49e09a)

#### `public bool Select(string Name)`

Selects the drawing item in the drawing catalog.

**Parameters:**
- `Name` (System.String) — The Name of the drawing item to select.

**Returns:** `Boolean` — True on success.

[Docs](https://developer.tekla.com/topic/en/18/47/6a028f0c-b5bb-2e2a-8119-eea7fbea3a34)

### Properties
- `Files` (List<FileInfo>, get/set) — The files belonging to drawing item.
- `Name` (String, get/set) — The drawing item's name.
- `Type` (DrawingTypes, get/set) — The files belonging to drawing item.

## DrawingItemEnumerator (class)

The DrawingItemEnumerator class allows to loop through the catalog drawing items.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/b1173c1b-55e1-315c-d1f0-0f85569c3290)

### Methods
#### `public int GetSize()`

Returns the total amout of items.

**Returns:** `Int32` — The total amount of items.

[Docs](https://developer.tekla.com/topic/en/18/47/8de3ed75-ca8d-b88d-c7b6-0fad5cd4f515)

#### `public bool MoveNext()`

Moves to the next item in the enumerator.

**Returns:** `Boolean` — False on failure.

[Docs](https://developer.tekla.com/topic/en/18/47/fde8aa3f-601b-5d2c-3d3b-15c5b04d34b3)

#### `public void Reset()`

Resets the enumerator to the beginning.

[Docs](https://developer.tekla.com/topic/en/18/47/6cf2a4dd-7afb-ac9e-550a-7ebff355061c)

### Properties
- `Current` (DrawingItem, get) — Returns a drawing item instance of the current element.

## FabricatorProfilesAndMaterials (class)

The FabricatorProfilesAndMaterials class contains methods for converting fabricator profiles and materials to Tekla Structures.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/5041534d-8dc1-d4ac-f2b8-ca0cd7778b26)

### Methods
#### `public static bool ConvertSourceFile(string profilesFilePath, string dimensionsFilePath)`

Converts a fabricator profile catalog source file to an XML file that contains a list of valid Tekla Structures catalog profiles and materials.

**Parameters:**
- `profilesFilePath` (System.String) — The path to the fabricator profiles and materials file.
- `dimensionsFilePath` (System.String) — The path to the fabricator dimensions file.

**Returns:** `Boolean` — True if conversion was successful.

[Docs](https://developer.tekla.com/topic/en/18/47/0ebcca0c-210b-e614-7bb9-ba5dfadceec1)

## FinishItem (struct)

The FinishItem struct contains information about the finish parameters.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/41196302-456a-c390-3cca-e32155fa9743)

### Constructors
- `public FinishItem(string abbreviation, string description)` — Creates a new finish item instance.

### Methods
#### `public override string ToString()`

Returns the fully qualified type name of this instance.

**Returns:** `String` — The fully qualified type name.

[Docs](https://developer.tekla.com/topic/en/18/47/6c0845a0-4621-8c59-f7b1-491241966a2d)

### Properties
- `Abbreviation` (String, get/set) — The finish abbreviation
- `Description` (String, get/set) — The finish description

## LibraryProfileItem (class)

The LibraryProfileItem class contains information from library profiles in the catalog. Library profile items can be enumerated using a profile item enumerator.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/ea4317c4-643d-d275-a818-1354400f42fd)

### Constructors
- `public LibraryProfileItem()` — Creates a new library profile item instance.

### Methods
#### `public bool AddCrossSection(int CrossSectionNumber, double Location, int Type, int SubType)`

Adds the cross section to a profile.

**Parameters:**
- `CrossSectionNumber` (System.Int32) — The cross section number.
- `Location` (System.Double) — The Location value.
- `Type` (System.Int32) — The type number.
- `SubType` (System.Int32) — The subtype number.

**Returns:** `Boolean` — 

[Docs](https://developer.tekla.com/topic/en/18/47/4e6a9522-3637-c6d8-0156-1c0da42950b6)

#### `public bool Copy(string newName)`

Copy the library profile item in the profile database to item with new name.

**Parameters:**
- `newName` (System.String) — The new name of the copied profile item.

**Returns:** `Boolean` — True on success.

[Docs](https://developer.tekla.com/topic/en/18/47/b47e13e3-7cfe-251a-ff4a-268f5e5cf5e4)

#### `public bool Delete()`

Deletes the profile item in the profile database.

**Returns:** `Boolean` — True on success.

[Docs](https://developer.tekla.com/topic/en/18/47/aa4e6c8a-57e6-8a6e-a228-652fcbe977a3)

#### `public bool DeleteCrossSection(int CrossSectionNumber)`

Deletes the cross section of a profile.

**Parameters:**
- `CrossSectionNumber` (System.Int32) — The cross section number.

**Returns:** `Boolean` — 

[Docs](https://developer.tekla.com/topic/en/18/47/1e4bdd8d-a3d7-7aac-9157-4c5f373471e8)

#### `public virtual bool Export(ref string filename)`

Exports the profile item in the profile database to given file name. Currently library profiles, sketch profiles and clb profiles are supported. Library profiles are exported to *.lis format. Sketch profiles are exported to *.uel format. Clb profiles are exported to *.clb format. If path is not given profile is exported to model folder. If filename is empty profile name or prefix is used as filename.

**Parameters:**
- `filename` (System.String.) — The export file name of the profile item.

**Returns:** `Boolean` — True on success.

[Docs](https://developer.tekla.com/topic/en/18/47/24d7de14-d3e6-314a-0e91-e57dab6ab0e7)

#### `public CrossSection GetCrossSection(double RelativeLocation = 0)`

Returns list of cross section points of inner surfaces from the profile item.

**Parameters:**
- `RelativeLocation` (System.Double) — The relative location of the cross section, values between 0 (start of profile) and 1 (end of profile).

**Returns:** `CrossSection` — List of inner cross section point lists on success, null on failure.

[Docs](https://developer.tekla.com/topic/en/18/47/9a9c5248-16b2-338f-062f-bfc58c4738e5)

#### `public CrossSection GetHighAccuracyCrossSection(double RelativeLocation = 0)`

Get cross section with high accuracy.

**Parameters:**
- `RelativeLocation` (System.Double) — The relative location of the cross section, values between 0 (start of profile) and 1 (end of profile).

**Returns:** `CrossSection` — CrossSection object or null on failure.

[Docs](https://developer.tekla.com/topic/en/18/47/285dde88-c699-5213-abc8-fa5f4c951bfe)

#### `public List<ProfileItemSubType> GetProfileItemSubTypes()`

Selects the profile item and updates the sub types in the profile database.

**Returns:** `List<ProfileItemSubType>` — 

[Docs](https://developer.tekla.com/topic/en/18/47/8470624e-f868-9cd5-c674-41da81101565)

#### `public bool IsProfileUserDefined()`

Whether the profile is a fixed user-defined profile.

**Returns:** `Boolean` — Whether the profile is a fixed user-defined profile or not.

[Docs](https://developer.tekla.com/topic/en/18/47/645e4bae-5cca-9fa0-6a29-2cae47b32227)

#### `public bool IsProfileUserParametric()`

Whether the profile is a parametric user-defined profile. If so, the prefix can be asked by type and subtype.

**Returns:** `Boolean` — Whether the profile is a parametric user-defined profile or not.

[Docs](https://developer.tekla.com/topic/en/18/47/ee8579f8-e74e-9ed2-d500-eaae92d78d76)

#### `public bool Modify()`

Modifies the profile item in the profile database.

**Returns:** `Boolean` — True on success.

[Docs](https://developer.tekla.com/topic/en/18/47/17d97070-9bfb-1d2f-fee9-6af19c795839)

#### `public bool ModifyCrossSection(int CrossSectionNumber, double Location, int Type, int SubType)`

Modifies the cross section of a profile.

**Parameters:**
- `CrossSectionNumber` (System.Int32) — The cross section number.
- `Location` (System.Double) — The Location value.
- `Type` (System.Int32) — The type number.
- `SubType` (System.Int32) — The subtype number.

**Returns:** `Boolean` — 

[Docs](https://developer.tekla.com/topic/en/18/47/3770cf41-a91e-a2b7-e61e-45e324e16afe)

#### `public bool ModifyProfileItemAnalysisParameter(ProfileItemParameter value)`

Modify analysis parameter of library profile item. Modify is needed for profile item for updating changes to profile database

**Parameters:**
- `value` (Tekla.Structures.Catalogs.ProfileItemParameter) — The modified parameter value for the profile item.

**Returns:** `Boolean` — True on success.

[Docs](https://developer.tekla.com/topic/en/18/47/8a875a37-33c2-54dd-3563-0ac9668839eb)

#### `public bool ModifyProfileItemParameter(ProfileItemParameter value)`

Modify parameter of library profile item. Modify is needed for profile item for updating changes to profile database

**Parameters:**
- `value` (Tekla.Structures.Catalogs.ProfileItemParameter) — The modified parameter value for the profile item.

**Returns:** `Boolean` — True on success.

[Docs](https://developer.tekla.com/topic/en/18/47/b8f040eb-f414-8c30-cb64-cf68921818db)

#### `public bool ModifyProfileItemUserParameter(ProfileItemParameter value)`

Modify user parameter of library profile item. Modify is needed for profile item for updating changes to profile database

**Parameters:**
- `value` (Tekla.Structures.Catalogs.ProfileItemParameter)

**Returns:** `Boolean` — True on success.

[Docs](https://developer.tekla.com/topic/en/18/47/2bc63067-81ca-5ba6-cf20-efa1c134ea97)

#### `public virtual bool Select()`

Selects the profile item in the profile database.

**Returns:** `Boolean` — True on success.

[Docs](https://developer.tekla.com/topic/en/18/47/90db4650-0ec0-3f08-dcc6-595f25789b73)

#### `public bool Select(string profileName)`

Selects the profile item in the profile database using the given name.

**Parameters:**
- `profileName` (System.String) — The name of the profile to select.

**Returns:** `Boolean` — True on success.

[Docs](https://developer.tekla.com/topic/en/18/47/d75dc6a6-c379-ac2f-a635-5c955166f1b8)

#### `public bool SelectCrossSections()`

Selects the profile item cross section in the profile database.

**Returns:** `Boolean` — True on success.

[Docs](https://developer.tekla.com/topic/en/18/47/f88821da-0793-3a27-6c20-2112029268fb)

#### `public bool SetEquivalentType(int CrossSectionNumber, int EquivalentType)`

Sets the equivalent type for the specified cross section.

**Parameters:**
- `CrossSectionNumber` (System.Int32) — The cross section number.
- `EquivalentType` (System.Int32) — The equivalent type.

**Returns:** `Boolean` — 

[Docs](https://developer.tekla.com/topic/en/18/47/935df01d-74a9-b6f3-5826-9b2d34a5aa52)

### Properties
- `aProfileItemAnalysisParameters` (ArrayList, get) — An array list with the profile item analysis parameters.
- `aProfileItemCrossSections` (ArrayList, get) — An array list with the profile item cross-sections.
- `aProfileItemParameters` (ArrayList, get) — An array list with the profile item parameters.
- `aProfileItemUserParameters` (ArrayList, get) — An array list with the profile item user parameters.
- `IsCLBProfile` (Boolean, get) — Whether the profile is a clb profile.
- `IsHardcodedProfile` (Boolean, get) — Whether the profile is a parametric user-defined hard coded profile.
- `IsMultiCrossSectionUserParametric` (Boolean, get) — Whether the profile is a parametric user-defined multi cross section profile.
- `IsSketchedUserParametric` (Boolean, get) — Whether the profile is a parametric user-defined sketched profile.
- `NumberOfCrossSections` (Int32, get) — The number of cross sections in the profile item.
- `ParameterString` (String, get) — The profile item parameter string.
- `ProfileItemCrossSections` (IList<ProfileItemCrossSection>, get) — The list of profile item cross-sections.
- `ProfileItemSubType` (ProfileItem.ProfileItemSubTypeEnum, get/set) — The profile item subtype.
- `ProfileItemType` (ProfileItem.ProfileItemTypeEnum, get) — The profile item type.
- `ProfileName` (String, get/set) — The profile item name.

## MaterialItem (class)

The MaterialItem class contains information about the materials in the Tekla Structures catalog.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/adea6e3d-a5cf-27f5-aea9-53028a0a7abb)

### Constructors
- `public MaterialItem()` — Creates a new material item instance.

### Methods
#### `public bool Delete()`

Delete this material from material database.

**Returns:** `Boolean` — True on success. False if operation failed. Possible reasons: a material with the name of this does not exist in database.

[Docs](https://developer.tekla.com/topic/en/18/47/c54db24c-0052-6fd7-cc44-feed0faf2d2d)

#### `public virtual bool Export(ref string filename)`

Exports the material item in the material database to given file name. Materials are exported to *.lis format. If path is not given material is exported to model folder. If filename is empty material name is used as filename.

**Parameters:**
- `filename` (System.String.) — The export file name of the material item.

**Returns:** `Boolean` — True on success.

[Docs](https://developer.tekla.com/topic/en/18/47/e25a642c-02df-a2d3-78a7-fb5de8f07c73)

#### `public bool Insert()`

Inserts this material item to material database.

**Returns:** `Boolean` — True on success. False if operation failed. Possible reasons: material name is invalid; a material with the same name already exists in database.

[Docs](https://developer.tekla.com/topic/en/18/47/c11e69a1-d29a-1037-0978-5fd05dd09fd8)

#### `public static int MaterialNameMaxLength()`

Returns the maximum length a material name can be.

**Returns:** `Int32` — The maximum length.

[Docs](https://developer.tekla.com/topic/en/18/47/e969d307-0de6-76ca-1a04-615718bb1a8a)

#### `public bool Modify()`

Modifies the material in material database that has the same name as this instance.

**Returns:** `Boolean` — True on success. False if operation failed. Possible reasons: a material with the name of this does not exist in database.

[Docs](https://developer.tekla.com/topic/en/18/47/8a966994-dbf3-f876-01b4-24a2ce3a609a)

#### `public bool Select()`

Selects the material item from the database based on the material name and alias name in this instance.

**Returns:** `Boolean` — True on success. False if the material was not found.

[Docs](https://developer.tekla.com/topic/en/18/47/0068c882-b486-2d60-27e7-d9f3a15b48d5)

#### `public bool Select(string materialName)`

Selects the material item based on the name from the material database. The material name can also be an alias name.

**Parameters:**
- `materialName` (System.String) — The name or alias name of the material.

**Returns:** `Boolean` — True on success. False if the material was not found.

[Docs](https://developer.tekla.com/topic/en/18/47/24928801-2e7f-f555-08c4-ba2f32d14410)

### Properties
- `AliasName1` (String, get/set) — The material item's alias name 1.
- `AliasName2` (String, get/set) — The material item's alias name 2.
- `AliasName3` (String, get/set) — The material item's alias name 3.
- `DesignCode` (Int32, get/set) — The design code.
- `MaterialName` (String, get/set) — The material item's name.
- `ModulusOfElasticity` (Double, get/set) — Thte modulus of elasticity in unit N/m2.
- `PlateDensity` (Double, get/set) — The plate density in unit kg/m3.
- `PoissonsRatio` (Double, get/set) — The poissons ratio
- `ProfileDensity` (Double, get/set) — The profile density in unit kg/m3.
- `ThermalDilatation` (Double, get/set) — The thermal dilatation in unit 1/K.
- `Type` (MaterialItem.MaterialItemTypeEnum, get/set) — The material item's type.

## MaterialItem.MaterialItemTypeEnum (enum)

Defines the different material item types.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/66c9dde4-2e2e-c54e-8e76-35853038601e)

### Values
- `MATERIAL_UNKNOWN` = `0` — The unknown material item type.
- `MATERIAL_STEEL` = `1` — The steel material type.
- `MATERIAL_CONCRETE` = `2` — The concrete material type.
- `MATERIAL_TIMBER` = `3` — The timber material type.
- `MATERIAL_MISC` = `4` — The miscellaneous material type.
- `MATERIAL_REBAR` = `5` — The rebar material type.
- `MATERIAL_REBAR_MESH` = `6` — The rebar mesh material type.

## MaterialItemEnumerator (class)

The MaterialItemEnumerator class allows to loop through the catalog material items.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/9e82b720-28fa-dceb-f771-59865cd3a5dd)

### Methods
#### `public int GetSize()`

Returns the total amout of items.

**Returns:** `Int32` — The total amount of items.

[Docs](https://developer.tekla.com/topic/en/18/47/11b98a3c-be43-b9dc-2d95-7e671b6c8549)

#### `public bool MoveNext()`

Moves to the next item in the enumerator.

**Returns:** `Boolean` — False on failure.

[Docs](https://developer.tekla.com/topic/en/18/47/c37ba5f4-4b49-a60e-9e70-33ecae0c5251)

#### `public void Reset()`

Resets the enumerator to the beginning.

[Docs](https://developer.tekla.com/topic/en/18/47/88af2290-f975-1fbe-cd9e-7f3dd8d552d2)

### Properties
- `Current` (MaterialItem, get) — Returns a material item instance of the current element.

## MaterialMarketSizesItem (class)

The MaterialMarketSizesItem class contains information about the available market sizes for particular material.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/4b82a70e-35d7-0a4b-08b1-10c3b96f6505)

### Properties
- `MarketSizes` (.Double., get) — Available MarketSizes for the particular material
- `MaterialName` (String, get) — Material name

## MaterialMarketSizesItemEnumerator (class)

The MaterialMarketSizesItemEnumerator class allows to loop through the items defined in XS_PROFDB\marketsizes.dat file.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/b38af553-7529-a4e3-dfb8-56da1faf142b)

### Methods
#### `public bool MoveNext()`

Moves to the next item in the MaterialMarketSizesItemEnumerator.

**Returns:** `Boolean` — False on failure.

[Docs](https://developer.tekla.com/topic/en/18/47/4194e31a-208e-0382-4d8b-f0da8c0c278c)

#### `public void Reset()`

Resets the MaterialMarketSizesItemEnumerator to the beginning.

[Docs](https://developer.tekla.com/topic/en/18/47/137847a1-5995-9fe8-7b7a-9f6ef98b62a1)

### Properties
- `Current` (MaterialMarketSizesItem, get) — Returns a MaterialMarketSizesItem instance with the current index.

## MaterialName (class)

The MaterialName class contains the name of the material item.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/e2e27770-8e50-518f-f6d8-a8cc391c97a2)

### Constructors
- `public MaterialName()` — Initializes a new instance of the MaterialName class

### Properties
- `Name` (String, get/set) — The material item name.

## MeshItem (class)

The MeshItem class contains information from the meshes in the catalog (mesh_database.inp).

[Vendor docs](https://developer.tekla.com/topic/en/18/47/f4d9b53a-3f68-8d8a-92c2-eff42b83b925)

### Constructors
- `public MeshItem()` — Creates a new mesh item instance.

### Methods
#### `public bool Export(ref string filename)`

Exports the rebar item in the catlog to given file name. If path is not given rebar is exported to model folder. If filename is empty rebar name is used as filename.

**Parameters:**
- `filename` (System.String.) — The export file name of the item.

**Returns:** `Boolean` — True on success.

[Docs](https://developer.tekla.com/topic/en/18/47/a6eb55ef-fcd9-90f8-3467-1b69037491ea)

#### `public bool Select(string MeshName, string MeshGrade)`

Selects the mesh item in the mesh database.

**Parameters:**
- `MeshName` (System.String) — The name of the mesh item to select.
- `MeshGrade` (System.String) — The grade of the mesh item to select.

**Returns:** `Boolean` — True on success.

[Docs](https://developer.tekla.com/topic/en/18/47/553fb3e2-fcc9-33bc-79e3-fdacd64003c9)

### Properties
- `DiameterCross` (String, get/set) — The mesh item's cross direction bar size.
- `DiameterLongitudinal` (String, get/set) — The mesh item's longitudinal direction bar size.
- `DistanceCross` (String, get/set) — The mesh item's cross direction distance.
- `DistanceLongitudinal` (String, get/set) — The mesh item's longitudinal direction distance.
- `Grade` (String, get/set) — The mesh item's grade.
- `LeftOverhangCross` (Double, get/set) — The mesh item's cross direction left overhang.
- `LeftOverhangLongitudinal` (Double, get/set) — The mesh item's longitudinal direction left overhang.
- `Length` (Double, get/set) — The mesh item's length.
- `MaximumOverlappingCross` (Double, get/set) — The mesh item's cross direction maximum overlapping.
- `MaximumOverlappingLongitudinal` (Double, get/set) — The mesh item's longitudinal direction maximum overlapping.
- `MinimumOverlappingCross` (Double, get/set) — The mesh item's cross direction minimum overlapping.
- `MinimumOverlappingLongitudinal` (Double, get/set) — The mesh item's longitudinal direction minimum overlapping.
- `Name` (String, get/set) — The mesh item's name.
- `RightOverhangCross` (Double, get/set) — The mesh item's cross direction right overhang.
- `RightOverhangLongitudinal` (Double, get/set) — The mesh item's longitudinal direction right overhang.
- `Width` (Double, get/set) — The mesh item's width.

## MeshItemEnumerator (class)

The MeshItemEnumerator class allows to loop through the catalog mesh items.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/f552ff0e-d4f1-6231-c3ef-0311c4c98d61)

### Methods
#### `public int GetSize()`

Returns the total amout of items.

**Returns:** `Int32` — The total amount of items.

[Docs](https://developer.tekla.com/topic/en/18/47/18b986f2-6f35-4fb8-b4a9-2189fc71872c)

#### `public bool MoveNext()`

Moves to the next item in the enumerator.

**Returns:** `Boolean` — False on failure.

[Docs](https://developer.tekla.com/topic/en/18/47/97bb2529-52f8-4678-1a2e-c0a8ac27b98e)

#### `public void Reset()`

Resets the enumerator to the beginning.

[Docs](https://developer.tekla.com/topic/en/18/47/057e41d3-1955-a5fe-48c7-aca7221b0b5c)

#### `public static MeshItem SelectMeshItem(string Name, string Grade)`

Selects a mesh item in the mesh database with the given name and grade.

**Parameters:**
- `Name` (System.String) — The name of the mesh item to select.
- `Grade` (System.String) — The grade of the mesh item to select.

**Returns:** `MeshItem` — The first MeshItem that matches the conditions.

[Docs](https://developer.tekla.com/topic/en/18/47/a34e89b5-c6a2-ec57-9250-2a9e29ae14bf)

### Properties
- `Current` (MeshItem, get) — Returns a mesh item instance of the current element.

## ParametricProfileItem (class)

The ParametricProfileItem class contains information from parametric profiles in the catalog. Parametric profile items can be enumerated using a profile item enumerator.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/e935ae0d-c280-dd76-a09c-bae369c41a52)

### Constructors
- `public ParametricProfileItem()` — Creates a new parametric profile item instance.

### Methods
#### `public bool AddCrossSection(int CrossSectionNumber, double Location, int Type, int SubType)`

Adds the cross section to a profile.

**Parameters:**
- `CrossSectionNumber` (System.Int32) — The cross section number.
- `Location` (System.Double) — The Location value.
- `Type` (System.Int32) — The type number.
- `SubType` (System.Int32) — The subtype number.

**Returns:** `Boolean` — 

[Docs](https://developer.tekla.com/topic/en/18/47/4e6a9522-3637-c6d8-0156-1c0da42950b6)

#### `public string CreateProfileString()`

Gets the parametric profile item prefix and adds parameter values.

**Returns:** `String` — The parametric profile string with parameter values in millimeters.

[Docs](https://developer.tekla.com/topic/en/18/47/4bc80f28-d1a5-5bef-34e7-07423b892bb8)

#### `public bool DeleteCrossSection(int CrossSectionNumber)`

Deletes the cross section of a profile.

**Parameters:**
- `CrossSectionNumber` (System.Int32) — The cross section number.

**Returns:** `Boolean` — 

[Docs](https://developer.tekla.com/topic/en/18/47/1e4bdd8d-a3d7-7aac-9157-4c5f373471e8)

#### `public virtual bool Export(ref string filename)`

Exports the profile item in the profile database to given file name. Currently library profiles, sketch profiles and clb profiles are supported. Library profiles are exported to *.lis format. Sketch profiles are exported to *.uel format. Clb profiles are exported to *.clb format. If path is not given profile is exported to model folder. If filename is empty profile name or prefix is used as filename.

**Parameters:**
- `filename` (System.String.) — The export file name of the profile item.

**Returns:** `Boolean` — True on success.

[Docs](https://developer.tekla.com/topic/en/18/47/24d7de14-d3e6-314a-0e91-e57dab6ab0e7)

#### `public CrossSection GetCrossSection(double RelativeLocation = 0)`

Returns list of cross section points of inner surfaces from the profile item.

**Parameters:**
- `RelativeLocation` (System.Double) — The relative location of the cross section, values between 0 (start of profile) and 1 (end of profile).

**Returns:** `CrossSection` — List of inner cross section point lists on success, null on failure.

[Docs](https://developer.tekla.com/topic/en/18/47/9a9c5248-16b2-338f-062f-bfc58c4738e5)

#### `public CrossSection GetHighAccuracyCrossSection(double RelativeLocation = 0)`

Get cross section with high accuracy.

**Parameters:**
- `RelativeLocation` (System.Double) — The relative location of the cross section, values between 0 (start of profile) and 1 (end of profile).

**Returns:** `CrossSection` — CrossSection object or null on failure.

[Docs](https://developer.tekla.com/topic/en/18/47/285dde88-c699-5213-abc8-fa5f4c951bfe)

#### `public static string GetParametricProfilePrefix(int Subtype)`

Gets the parametric profile item prefix using the given subtype. This can be used after retrieving a library profile that has a parametric user-defined type.

**Parameters:**
- `Subtype` (System.Int32) — The profile item subtype.

**Returns:** `String` — The parametric profile item prefix.

[Docs](https://developer.tekla.com/topic/en/18/47/6a353980-ac2f-62c2-794d-7e62e80d2a35)

#### `public List<ProfileItemSubType> GetProfileItemSubTypes()`

Selects the profile item and updates the sub types in the profile database.

**Returns:** `List<ProfileItemSubType>` — 

[Docs](https://developer.tekla.com/topic/en/18/47/8470624e-f868-9cd5-c674-41da81101565)

#### `public bool IsProfileUserDefined()`

Whether the profile is a fixed user-defined profile.

**Returns:** `Boolean` — Whether the profile is a fixed user-defined profile or not.

[Docs](https://developer.tekla.com/topic/en/18/47/645e4bae-5cca-9fa0-6a29-2cae47b32227)

#### `public bool IsProfileUserParametric()`

Whether the profile is a parametric user-defined profile. If so, the prefix can be asked by type and subtype.

**Returns:** `Boolean` — Whether the profile is a parametric user-defined profile or not.

[Docs](https://developer.tekla.com/topic/en/18/47/ee8579f8-e74e-9ed2-d500-eaae92d78d76)

#### `public bool ModifyCrossSection(int CrossSectionNumber, double Location, int Type, int SubType)`

Modifies the cross section of a profile.

**Parameters:**
- `CrossSectionNumber` (System.Int32) — The cross section number.
- `Location` (System.Double) — The Location value.
- `Type` (System.Int32) — The type number.
- `SubType` (System.Int32) — The subtype number.

**Returns:** `Boolean` — 

[Docs](https://developer.tekla.com/topic/en/18/47/3770cf41-a91e-a2b7-e61e-45e324e16afe)

#### `public bool ModifyProfileItemAnalysisParameter(ProfileItemParameter value)`

Modify analysis parameter of library profile item. Modify is needed for profile item for updating changes to profile database

**Parameters:**
- `value` (Tekla.Structures.Catalogs.ProfileItemParameter) — The modified parameter value for the profile item.

**Returns:** `Boolean` — True on success.

[Docs](https://developer.tekla.com/topic/en/18/47/8a875a37-33c2-54dd-3563-0ac9668839eb)

#### `public bool ModifyProfileItemParameter(ProfileItemParameter value)`

Modify parameter of library profile item. Modify is needed for profile item for updating changes to profile database

**Parameters:**
- `value` (Tekla.Structures.Catalogs.ProfileItemParameter) — The modified parameter value for the profile item.

**Returns:** `Boolean` — True on success.

[Docs](https://developer.tekla.com/topic/en/18/47/b8f040eb-f414-8c30-cb64-cf68921818db)

#### `public bool ModifyProfileItemUserParameter(ProfileItemParameter value)`

Modify user parameter of library profile item. Modify is needed for profile item for updating changes to profile database

**Parameters:**
- `value` (Tekla.Structures.Catalogs.ProfileItemParameter)

**Returns:** `Boolean` — True on success.

[Docs](https://developer.tekla.com/topic/en/18/47/2bc63067-81ca-5ba6-cf20-efa1c134ea97)

#### `public virtual bool Select()`

Selects the profile item in the profile database.

**Returns:** `Boolean` — True on success.

[Docs](https://developer.tekla.com/topic/en/18/47/90db4650-0ec0-3f08-dcc6-595f25789b73)

#### `public bool Select(string ProfileName)`

Selects the parametric profile item in the profile database using the given name.

**Parameters:**
- `ProfileName` (System.String) — The name of the profile to select.

**Returns:** `Boolean` — True on success.

[Docs](https://developer.tekla.com/topic/en/18/47/33bea54e-b358-cf68-e3eb-0b3de35a8e1a)

#### `public bool SelectByPrefix(string ProfilePrefix)`

Selects the parametric profile item in the profile database using the given prefix.

**Parameters:**
- `ProfilePrefix` (System.String) — The prefix of the profile name to be select.

**Returns:** `Boolean` — True on success.

[Docs](https://developer.tekla.com/topic/en/18/47/e2ac6fd0-46c5-ddee-3d5e-37ce5cfc9fcb)

#### `public bool SelectByProfileName(string ProfileName)`

Selects the parametric profile item in the profile database using the given name.

**Parameters:**
- `ProfileName` (System.String) — The profile name to be select

**Returns:** `Boolean` — True on success.

[Docs](https://developer.tekla.com/topic/en/18/47/161262ed-bbdc-6520-94cc-85293c837c1b)

#### `public bool SelectByTypeAndSubtype(string profileName, ProfileItem.ProfileItemTypeEnum type, ProfileItem.ProfileItemSubTypeEnum subtype)`

Selects the parametric profile item in the profile database using the given type and subtype.

**Parameters:**
- `profileName` (System.String) — The profile name.
- `type` (Tekla.Structures.Catalogs.ProfileItem.ProfileItemTypeEnum) — The profile item type.
- `subtype` (Tekla.Structures.Catalogs.ProfileItem.ProfileItemSubTypeEnum) — The profile item subtype.

**Returns:** `Boolean` — 

[Docs](https://developer.tekla.com/topic/en/18/47/d02e6280-4832-b8ad-6faa-efc77b52de69)

#### `public bool SelectCrossSections()`

Selects the profile item cross section in the profile database.

**Returns:** `Boolean` — True on success.

[Docs](https://developer.tekla.com/topic/en/18/47/f88821da-0793-3a27-6c20-2112029268fb)

#### `public bool SetEquivalentType(int CrossSectionNumber, int EquivalentType)`

Sets the equivalent type for the specified cross section.

**Parameters:**
- `CrossSectionNumber` (System.Int32) — The cross section number.
- `EquivalentType` (System.Int32) — The equivalent type.

**Returns:** `Boolean` — 

[Docs](https://developer.tekla.com/topic/en/18/47/935df01d-74a9-b6f3-5826-9b2d34a5aa52)

### Properties
- `aProfileItemCrossSections` (ArrayList, get) — An array list with the profile item cross-sections.
- `aProfileItemParameters` (ArrayList, get) — An array list with the profile item parameters.
- `IsCLBProfile` (Boolean, get) — Whether the profile is a clb profile.
- `IsHardcodedProfile` (Boolean, get) — Whether the profile is a parametric user-defined hard coded profile.
- `IsMultiCrossSectionUserParametric` (Boolean, get) — Whether the profile is a parametric user-defined multi cross section profile.
- `IsSketchedUserParametric` (Boolean, get) — Whether the profile is a parametric user-defined sketched profile.
- `NumberOfCrossSections` (Int32, get) — The number of cross sections in the profile item.
- `ParameterString` (String, get) — The profile item parameter string.
- `ProfileItemCrossSections` (IList<ProfileItemCrossSection>, get) — The list of profile item cross-sections.
- `ProfileItemSubType` (ProfileItem.ProfileItemSubTypeEnum, get/set) — The profile item subtype.
- `ProfileItemType` (ProfileItem.ProfileItemTypeEnum, get) — The profile item type.
- `ProfilePrefix` (String, get/set) — The parametric profile item prefix.

## PrinterItem (class)

The PrinterItem class contains information about the printers in the Tekla Structures catalog.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/cfcf8cd5-e9f5-e0aa-8773-3f607dba8404)

### Constructors
- `public PrinterItem()` — Creates a new printer item instance.

### Properties
- `Device` (String, get/set) — The actual printer device used in the printing.
- `Extension` (String, get/set) — The printer item's default file extension which is used when printing to a file.
- `Name` (String, get/set) — The printer item's name which Tekla Structures uses to fetch all the item's default properties in the printing process.
- `PrintAreaHeigth` (Double, get/set) — The printer item's print area height.
- `PrintAreaWidth` (Double, get/set) — The printer item's print area width.

## PrinterItemEnumerator (class)

The PrinterItemEnumerator class allows to loop through the catalog printer items.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/132ffaaa-76cc-e3d2-ffac-27b40b486f31)

### Methods
#### `public int GetSize()`

Returns the total amout of items.

**Returns:** `Int32` — The total amount of items.

[Docs](https://developer.tekla.com/topic/en/18/47/9f84ae90-258f-7157-8edf-44306d49b891)

#### `public bool MoveNext()`

Moves to the next item in the enumerator.

**Returns:** `Boolean` — False on failure.

[Docs](https://developer.tekla.com/topic/en/18/47/c8559650-3ec2-6577-f722-d129c6ed86e1)

#### `public void Reset()`

Resets the enumerator to the beginning.

[Docs](https://developer.tekla.com/topic/en/18/47/d7b01b70-0226-b4b8-d384-224406109aa5)

### Properties
- `Current` (PrinterItem, get) — Returns a printer item instance of the current element.

## ProfileItem (class)

The ProfileItem abstract class contains the common information of catalog profiles (parametric and library).

[Vendor docs](https://developer.tekla.com/topic/en/18/47/4f68f2b3-0aff-93e7-d06f-aa8cea335f61)

### Methods
#### `public bool AddCrossSection(int CrossSectionNumber, double Location, int Type, int SubType)`

Adds the cross section to a profile.

**Parameters:**
- `CrossSectionNumber` (System.Int32) — The cross section number.
- `Location` (System.Double) — The Location value.
- `Type` (System.Int32) — The type number.
- `SubType` (System.Int32) — The subtype number.

**Returns:** `Boolean` — 

[Docs](https://developer.tekla.com/topic/en/18/47/4e6a9522-3637-c6d8-0156-1c0da42950b6)

#### `public bool DeleteCrossSection(int CrossSectionNumber)`

Deletes the cross section of a profile.

**Parameters:**
- `CrossSectionNumber` (System.Int32) — The cross section number.

**Returns:** `Boolean` — 

[Docs](https://developer.tekla.com/topic/en/18/47/1e4bdd8d-a3d7-7aac-9157-4c5f373471e8)

#### `public virtual bool Export(ref string filename)`

Exports the profile item in the profile database to given file name. Currently library profiles, sketch profiles and clb profiles are supported. Library profiles are exported to *.lis format. Sketch profiles are exported to *.uel format. Clb profiles are exported to *.clb format. If path is not given profile is exported to model folder. If filename is empty profile name or prefix is used as filename.

**Parameters:**
- `filename` (System.String.) — The export file name of the profile item.

**Returns:** `Boolean` — True on success.

[Docs](https://developer.tekla.com/topic/en/18/47/24d7de14-d3e6-314a-0e91-e57dab6ab0e7)

#### `public CrossSection GetCrossSection(double RelativeLocation = 0)`

Returns list of cross section points of inner surfaces from the profile item.

**Parameters:**
- `RelativeLocation` (System.Double) — The relative location of the cross section, values between 0 (start of profile) and 1 (end of profile).

**Returns:** `CrossSection` — List of inner cross section point lists on success, null on failure.

[Docs](https://developer.tekla.com/topic/en/18/47/9a9c5248-16b2-338f-062f-bfc58c4738e5)

#### `public CrossSection GetHighAccuracyCrossSection(double RelativeLocation = 0)`

Get cross section with high accuracy.

**Parameters:**
- `RelativeLocation` (System.Double) — The relative location of the cross section, values between 0 (start of profile) and 1 (end of profile).

**Returns:** `CrossSection` — CrossSection object or null on failure.

[Docs](https://developer.tekla.com/topic/en/18/47/285dde88-c699-5213-abc8-fa5f4c951bfe)

#### `public List<ProfileItemSubType> GetProfileItemSubTypes()`

Selects the profile item and updates the sub types in the profile database.

**Returns:** `List<ProfileItemSubType>` — 

[Docs](https://developer.tekla.com/topic/en/18/47/8470624e-f868-9cd5-c674-41da81101565)

#### `public static List<ProfileItemSubType> GetProfileItemSubTypes(ProfileItem profileItem, ProfileItem.ProfileItemTypeEnum typeEnum)`



**Parameters:**
- `profileItem` (Tekla.Structures.Catalogs.ProfileItem)
- `typeEnum` (Tekla.Structures.Catalogs.ProfileItem.ProfileItemTypeEnum)

**Returns:** `List<ProfileItemSubType>` — 

[Docs](https://developer.tekla.com/topic/en/18/47/1c3c5a2f-8603-df13-d55d-07ad12763469)

#### `public bool IsProfileUserDefined()`

Whether the profile is a fixed user-defined profile.

**Returns:** `Boolean` — Whether the profile is a fixed user-defined profile or not.

[Docs](https://developer.tekla.com/topic/en/18/47/645e4bae-5cca-9fa0-6a29-2cae47b32227)

#### `public bool IsProfileUserParametric()`

Whether the profile is a parametric user-defined profile. If so, the prefix can be asked by type and subtype.

**Returns:** `Boolean` — Whether the profile is a parametric user-defined profile or not.

[Docs](https://developer.tekla.com/topic/en/18/47/ee8579f8-e74e-9ed2-d500-eaae92d78d76)

#### `public bool ModifyCrossSection(int CrossSectionNumber, double Location, int Type, int SubType)`

Modifies the cross section of a profile.

**Parameters:**
- `CrossSectionNumber` (System.Int32) — The cross section number.
- `Location` (System.Double) — The Location value.
- `Type` (System.Int32) — The type number.
- `SubType` (System.Int32) — The subtype number.

**Returns:** `Boolean` — 

[Docs](https://developer.tekla.com/topic/en/18/47/3770cf41-a91e-a2b7-e61e-45e324e16afe)

#### `public bool ModifyProfileItemAnalysisParameter(ProfileItemParameter value)`

Modify analysis parameter of library profile item. Modify is needed for profile item for updating changes to profile database

**Parameters:**
- `value` (Tekla.Structures.Catalogs.ProfileItemParameter) — The modified parameter value for the profile item.

**Returns:** `Boolean` — True on success.

[Docs](https://developer.tekla.com/topic/en/18/47/8a875a37-33c2-54dd-3563-0ac9668839eb)

#### `public bool ModifyProfileItemParameter(ProfileItemParameter value)`

Modify parameter of library profile item. Modify is needed for profile item for updating changes to profile database

**Parameters:**
- `value` (Tekla.Structures.Catalogs.ProfileItemParameter) — The modified parameter value for the profile item.

**Returns:** `Boolean` — True on success.

[Docs](https://developer.tekla.com/topic/en/18/47/b8f040eb-f414-8c30-cb64-cf68921818db)

#### `public bool ModifyProfileItemUserParameter(ProfileItemParameter value)`

Modify user parameter of library profile item. Modify is needed for profile item for updating changes to profile database

**Parameters:**
- `value` (Tekla.Structures.Catalogs.ProfileItemParameter)

**Returns:** `Boolean` — True on success.

[Docs](https://developer.tekla.com/topic/en/18/47/2bc63067-81ca-5ba6-cf20-efa1c134ea97)

#### `public virtual bool Select()`

Selects the profile item in the profile database.

**Returns:** `Boolean` — True on success.

[Docs](https://developer.tekla.com/topic/en/18/47/90db4650-0ec0-3f08-dcc6-595f25789b73)

#### `public bool SelectCrossSections()`

Selects the profile item cross section in the profile database.

**Returns:** `Boolean` — True on success.

[Docs](https://developer.tekla.com/topic/en/18/47/f88821da-0793-3a27-6c20-2112029268fb)

#### `public bool SetEquivalentType(int CrossSectionNumber, int EquivalentType)`

Sets the equivalent type for the specified cross section.

**Parameters:**
- `CrossSectionNumber` (System.Int32) — The cross section number.
- `EquivalentType` (System.Int32) — The equivalent type.

**Returns:** `Boolean` — 

[Docs](https://developer.tekla.com/topic/en/18/47/935df01d-74a9-b6f3-5826-9b2d34a5aa52)

### Properties
- `aProfileItemCrossSections` (ArrayList, get) — An array list with the profile item cross-sections.
- `aProfileItemParameters` (ArrayList, get) — An array list with the profile item parameters.
- `IsCLBProfile` (Boolean, get) — Whether the profile is a clb profile.
- `IsHardcodedProfile` (Boolean, get) — Whether the profile is a parametric user-defined hard coded profile.
- `IsMultiCrossSectionUserParametric` (Boolean, get) — Whether the profile is a parametric user-defined multi cross section profile.
- `IsSketchedUserParametric` (Boolean, get) — Whether the profile is a parametric user-defined sketched profile.
- `NumberOfCrossSections` (Int32, get) — The number of cross sections in the profile item.
- `ParameterString` (String, get) — The profile item parameter string.
- `ProfileItemCrossSections` (IList<ProfileItemCrossSection>, get) — The list of profile item cross-sections.
- `ProfileItemSubType` (ProfileItem.ProfileItemSubTypeEnum, get/set) — The profile item subtype.
- `ProfileItemType` (ProfileItem.ProfileItemTypeEnum, get) — The profile item type.

## ProfileItem.ProfileItemParametersTypeEnum (enum)

Defines the different profile item parameters types.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/2348c0cd-946a-93da-eef9-dbec35c67fa1)

### Values
- `GENERAL_PARAMETERS` = `0` — The general profile item parameters type.
- `ANALYSIS_PARAMETERS` = `1` — The analysis profile item parameters type.
- `USER_PARAMETERS` = `2` — The user profile item parameters type.

## ProfileItem.ProfileItemSubTypeEnum (enum)

Defines the different profile item subtypes.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/5a60428d-3d51-b5a4-e7b3-2b04e749d1fc)

### Values
- `PROFILE_UNKNOWN_SUBTYPE` = `0` — The unknown subtype profile.
- `PROFILE_I_HOT_ROLLED` = `1001` — The hot rolled I profile.
- `PROFILE_I_WELDED_SYMMETRICAL` = `1002` — The welded symmetrical I profile.
- `PROFILE_I_WELDED_UNSYMMETRICAL` = `1003` — The welded unsymmetrical I profile.
- `PROFILE_I_WELDED_SYMMETRICAL2` = `1004` — The welded symmetrical, altering height, I profile.
- `PROFILE_I_WELDED_UNSYMMETRICAL2` = `1005` — The welded unsymmetrical, altering height, I profile.
- `PROFILE_L_HOT_ROLLED` = `2001` — The hot rolled L profile.
- `PROFILE_L_COLD_ROLLED` = `2002` — The cold rolled L profile.
- `PROFILE_Z_COLD_ROLLED` = `3001` — The cold rolled Z profile.
- `PROFILE_U_HOT_ROLLED` = `4001` — The hot rolled U profile.
- `PROFILE_U_COLD_ROLLED` = `4002` — The cold rolled U profile.
- `PROFILE_PL_DEFAULT` = `5001` — The default plate profile.
- `PROFILE_D_CIRCULAR` = `6001` — The default circular section profile.
- `PROFILE_D_ELLIPTICAL` = `6002` — The elliptical circular section profile.
- `PROFILE_PD_CIRCULAR` = `7001` — The default circular hollow section profile.
- `PROFILE_PD_ELLIPTICAL` = `7002` — The elliptical circular hollow section profile.
- `PROFILE_PD_CIRCULAR_TAPERED` = `7003` — The tapered circular hollow section profile.
- `PROFILE_P_SQUARE` = `8001` — The square hollow section profile.
- `PROFILE_P_RECTANGULAR` = `8002` — The rectangular hollow section profile.
- `PROFILE_P_ALTERING_HEIGHT` = `8003` — The altering height hollow section profile.
- `PROFILE_C_HOT_ROLLED` = `9001` — The hot rolled C profile.
- `PROFILE_C_COLD_ROLLED` = `9002` — The cold rolled C profile.
- `PROFILE_T_HOT_ROLLED` = `10001` — The hot rolled T profile.
- `PROFILE_T_PARAMETRIC` = `10002` — The parametric T profile.
- `PROFILE_HK_SYMMETRICAL` = `11001` — The symmetrical welded box profile.
- `PROFILE_HK_UNSYMMETRICAL` = `11002` — The unsymmetrical welded box profile.
- `PROFILE_HQ_CENTERED` = `13001` — The centered HQ profile.
- `PROFILE_HQ_NOT_CENTERED` = `13002` — The not centered HQ profile.
- `PROFILE_ZZ_SYMMETRICAL` = `15001` — The symmetrical ZZ profile.
- `PROFILE_ZZ_NOT_SYMMETRICAL` = `15002` — The unsymmetrical ZZ profile.
- `PROFILE_CC_SYMMETRICAL` = `16001` — The symmetrical CC profile.
- `PROFILE_CC_NOT_SYMMETRICAL` = `16002` — The unsymmetrical CC profile.
- `PROFILE_CW_SYMMETRICAL` = `17001` — The symmetrical CW profile.
- `PROFILE_CW_UNSYMMETRICAL` = `17002` — The unsymmetrical CW profile.
- `PROFILE_CU_SYMMETRICAL` = `18001` — The symmetrical CU profile.
- `PROFILE_CU_NOT_SYMMETRICAL` = `18002` — The unsymmetrical CU profile.
- `PROFILE_EB_SYMMETRICAL` = `19001` — The symmetrical EB profile.
- `PROFILE_EB_NOT_SYMMETRICAL` = `19002` — The unsymmetrical EB profile.
- `PROFILE_BF_DEFAULT` = `20001` — The default BF profile.
- `PROFILE_SPD_CIRCULAR` = `21001` — The circular SPD profile.
- `PROFILE_SPD_ELLIPTICAL` = `21002` — The elliptical SPD profile.
- `PROFILE_SPD_CIRCULAR_TAPERED` = `21003` — The tapered circular SPD profile.
- `PROFILE_EC_SYMMETRICAL` = `22001` — The symmetrical EC profile.
- `PROFILE_EC_NOT_SYMMETRICAL` = `22002` — The unsymmetrical EC profile.
- `PROFILE_ED_DEFAULT` = `23001` — The default ED profile.
- `PROFILE_EE_DEFAULT` = `24001` — The default EE profile.
- `PROFILE_EF_DEFAULT` = `25001` — The default EF profile.
- `PROFILE_EZ_DEFAULT` = `26001` — The default EZ profile.
- `PROFILE_EW_DEFAULT` = `27001` — The default EW profile.
- `PROFILE_RCDL_SYMMETRICAL` = `102001` — The symmetrical RCDL profile.
- `PROFILE_RCDL_UNSYMMETRICAL` = `102002` — The unsymmetrical RCDL profile.
- `PROFILE_RCXX_DEFAULT` = `103001` — The RCXX default profile.
- `PROFILE_RCL_DEFAULT` = `104001` — The RCL default profile.
- `PROFILE_RCDX_SYMMETRICAL` = `105001` — The symmetrical RCDX profile.
- `PROFILE_RCDX_UNSYMMETRICAL` = `105002` — The unsymmetrical RCDX profile.
- `PROFILE_RCDX_UNSYMMETRICAL2` = `105003` — The unsymmetrical altered height RCDX profile.
- `PROFILE_RCX_DEFAULT` = `106001` — The RCX default profile.

## ProfileItem.ProfileItemTypeEnum (enum)

Defines the different profile item types.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/94572333-9548-b55f-e9ec-167c0efc528e)

### Values
- `ALL_PROFILES` = `-1` — All profiles.
- `PROFILE_UNKNOWN` = `0` — The unknown profile.
- `PROFILE_I` = `1` — The I profile.
- `PROFILE_L` = `2` — The L profile.
- `PROFILE_Z` = `3` — The Z profile.
- `PROFILE_U` = `4` — The U profile.
- `PROFILE_PL` = `5` — The plate profile.
- `PROFILE_D` = `6` — The circular section profile.
- `PROFILE_PD` = `7` — The circular hollow section profile.
- `PROFILE_P` = `8` — The rectangular hollow section profile.
- `PROFILE_C` = `9` — The C profile.
- `PROFILE_T` = `10` — The T profile.
- `PROFILE_HK` = `11` — The welded box profile.
- `PROFILE_HQ` = `13` — The HQ profile
- `PROFILE_ZZ` = `15` — The ZZ profile.
- `PROFILE_CC` = `16` — The CC profile.
- `PROFILE_CW` = `17` — The CW profile.
- `PROFILE_CU` = `18` — The CU profile.
- `PROFILE_EB` = `19` — The EB profile.
- `PROFILE_BF` = `20` — The BF profile.
- `PROFILE_SPD` = `21` — The SPD profile.
- `PROFILE_EC` = `22` — The EC profile.
- `PROFILE_ED` = `23` — The ED profile.
- `PROFILE_EE` = `24` — The EE profile.
- `PROFILE_EF` = `25` — The EF profile.
- `PROFILE_EZ` = `26` — The EZ profile.
- `PROFILE_EW` = `27` — The EW profile.
- `PROFILE_POLYGON_PLATE` = `51` — The polygon plate profile.
- `PROFILE_FPL` = `2` — The FPL profile.
- `PROFILE_SP` = `101` — The SP profile.
- `PROFILE_RCDL` = `102` — The RCDL profile.
- `PROFILE_RCXX` = `103` — The RCXX profile.
- `PROFILE_RCL` = `104` — The RCL profile.
- `PROFILE_RCDX` = `105` — The RCDX profile.
- `PROFILE_RCX` = `106` — The RCX profile.
- `PROFILE_USER_DEFINED` = `998` — The user-defined, fixed profile.
- `PROFILE_USER_PARAMETRIC` = `999` — The user-defined, parametric profile.

## ProfileItemCrossSection (class)

The ProfileItemCrossSection contains the information of the cross section information for multi-cross section profiles.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/89e71628-5e13-2b7e-d0e8-4ebc503cee62)

### Constructors
- `public ProfileItemCrossSection()` — Initializes a new instance of the ProfileItemCrossSection class

### Methods
#### `public void AddProfileItemAnalysisParamter(string property, string alblString, string symbol, Object value, ProfileItemParameter.ParameterUnitTypeEnum ParameterUnitType, int crossSectionNumber)`

adds a new profile item analisis parameter.

**Parameters:**
- `property` (System.String) — The name of the property.
- `alblString` (System.String) — The translation string.
- `symbol` (System.String) — The property symbol.
- `value` (System.Object) — The value of the property.
- `ParameterUnitType` (Tekla.Structures.Catalogs.ProfileItemParameter.ParameterUnitTypeEnum) — The property value unit type.
- `crossSectionNumber` (System.Int32) — The cross-section number.

[Docs](https://developer.tekla.com/topic/en/18/47/1fcd2d06-503e-f7e9-9837-1756ae102de0)

#### `public void AddProfileItemParamter(string property, string alblString, string symbol, Object value, ProfileItemParameter.ParameterUnitTypeEnum ParameterUnitType, int crossSectionNumber)`

adds a new profile item parameter.

**Parameters:**
- `property` (System.String) — The name of the property.
- `alblString` (System.String) — The translation string.
- `symbol` (System.String) — The property symbol.
- `value` (System.Object) — The value of the property.
- `ParameterUnitType` (Tekla.Structures.Catalogs.ProfileItemParameter.ParameterUnitTypeEnum) — The property value unit type.
- `crossSectionNumber` (System.Int32) — The cross-section number.

[Docs](https://developer.tekla.com/topic/en/18/47/0e81ef0a-d9ca-2db1-6937-c833eced17d5)

#### `public void AddProfileItemUserParamter(string property, string alblString, string symbol, Object value, ProfileItemParameter.ParameterUnitTypeEnum ParameterUnitType, int crossSectionNumber)`

adds a new profile item user parameter.

**Parameters:**
- `property` (System.String) — The name of the property.
- `alblString` (System.String) — The translation string.
- `symbol` (System.String) — The property symbol.
- `value` (System.Object) — The value of the property.
- `ParameterUnitType` (Tekla.Structures.Catalogs.ProfileItemParameter.ParameterUnitTypeEnum) — The property value unit type.
- `crossSectionNumber` (System.Int32) — The cross-section number.

[Docs](https://developer.tekla.com/topic/en/18/47/fb403a79-d9d4-f258-5ee6-8ad260b3f729)

### Properties
- `CrossSectionLocation` (Double, get/set) — Gets or sets a value of the property CrossSectionLocation.
- `CrossSectionNumber` (Int32, get/set) — Gets or sets a value of the property CrossSectionNumber.
- `EquivalentType` (Int32, get/set) — Gets or sets a value of the property EquivalentType.
- `Location` (Double, get/set) — Gets or sets a value of the property Location.
- `Number` (Int32, get/set) — Gets or sets a value of the property Number.
- `NumberOfProfileItemAnalysisParameters` (Int32, get/set) — Gets or sets a value of the property NumberOfProfileItemAnalysisParameters.
- `NumberOfProfileItemParameters` (Int32, get/set) — Gets or sets a value of the property NumberOfProfileItemParameters.
- `NumberOfProfileItemUserParameters` (Int32, get/set) — Gets or sets a value of the property NumberOfProfileItemUserParameters.
- `ProfileItemAnalysisParameters` (IList<ProfileItemParameter>, get/set) — Gets or sets a value of the property ProfileItemAnalysisParameters.
- `ProfileItemParameters` (IList<ProfileItemParameter>, get/set) — Gets or sets a value of the property ProfileItemParameters.
- `ProfileItemUserParameters` (IList<ProfileItemParameter>, get/set) — Gets or sets a value of the property ProfileItemUserParameters.
- `ProfileSubType` (Int32, get/set) — Gets or sets a value of the property ProfileSubType.
- `ProfileType` (Int32, get/set) — Gets or sets a value of the property ProfileType.

## ProfileItemEnumerator (class)

The ProfileItemEnumerator class allows to loop through the catalog profile items.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/62285dcc-e4e8-a121-ff28-2be8afdf5cc4)

### Methods
#### `public int GetSize()`

Returns the total amout of items.

**Returns:** `Int32` — The total amount of items.

[Docs](https://developer.tekla.com/topic/en/18/47/089f4ae6-eddb-d928-c1be-39c62ab881aa)

#### `public bool MoveNext()`

Moves to the next item in the enumerator.

**Returns:** `Boolean` — False on failure.

[Docs](https://developer.tekla.com/topic/en/18/47/082a4e9c-78eb-f599-2c31-1cd02d530cab)

#### `public void Reset()`

Resets the enumerator to the beginning.

[Docs](https://developer.tekla.com/topic/en/18/47/8b2a71bb-ede6-eca5-c50b-8b5d5680ef64)

### Properties
- `Current` (ProfileItem, get) — Returns a profile item instance of the current element.
- `SelectInstances` (Boolean, get/set) — Indicates that the instance Select() is called when the 'Current' item is asked from the enumerator. The user can set this to 'false' if no members are ever asked from the instance. This is the case when, for example, asking only for the available profile names or when only certain profiles need to be selected from the model. Without the selection the 'Current' item contains the profile name or the prefix and the profile type. Warning: normally the user should not change this value.

## ProfileItemParameter (class)

The ProfileItemParameter class contains the information of one profile parameter (property name, symbol, unit and unit type). A profile item can contain a maximum of 50 profile parameters.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/1f0dd6fa-80fc-e491-3232-a2ab83cbd1c8)

### Constructors
- `public ProfileItemParameter()` — Initializes a new instance of the ProfileItemParameter class

### Properties
- `AlblString` (String, get) — Gets a translated albl string.
- `CrossSectionNumber` (Int32, get) — The number of the cross section the parameter belongs to.
- `IntegerValue` (Int32, get/set) — The integer value of the profile item parameter.
- `ParameterUnitType` (ProfileItemParameter.ParameterUnitTypeEnum, get) — Defines the parameter unit type.
- `Property` (String, get) — The description of the profile item parameter. Corresponds to the 'Property' in the Tekla Structures profile catalog dialog, and the 'Label in dialog box' in the variable dialog in the Sketch Editor.
- `StringValue` (String, get/set) — The string value of the profile item parameter.
- `Symbol` (String, get) — The symbol of the profile item parameter.
- `Value` (Double, get/set) — The value of the profile item parameter.

## ProfileItemParameter.ParameterUnitTypeEnum (enum)

Defines the different unit types of the profile item parameter.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/80be683f-4f20-cda9-044b-b1d5336794f0)

### Values
- `INPUT_BOOLEAN` = `-3` — The input boolean.
- `INPUT_INTEGER` = `-2` — The input integer.
- `INPUT_STRING` = `-1` — The input string.
- `INPUT_NONE` = `0` — The input none.
- `INPUT_RATIO_UNIT` = `1` — The input ratio unit.
- `INPUT_STRAIN_UNIT` = `2` — The input strain unit.
- `INPUT_ANGLE_UNIT` = `5` — The input angle unit.
- `OUTPUT_ANGLE_UNIT` = `6` — The output angle unit.
- `INPUT_SECTION_ANGLE_UNIT` = `7` — The input section angle unit.
- `INPUT_LENGTH_UNIT` = `10` — The input length unit.
- `OUTPUT_LENGTH_UNIT` = `11` — The output length unit.
- `INPUT_DEFORMATION_UNIT` = `12` — The input deformation unit.
- `OUTPUT_DEFORMATION_UNIT` = `13` — The output deformation unit.
- `INPUT_DIMENSION_UNIT` = `14` — The input dimension unit.
- `INPUT_RADIUSOFINERTIA_UNIT` = `16` — The input radius of inertia unit.
- `INPUT_AREA_UNIT` = `20` — The input area unit.
- `OUTPUT_REINFAREA_UNIT` = `21` — The output reinforced area unit.
- `OUTPUT_TRANSVREINF_UNIT` = `22` — The output transverse reinforcement unit.
- `INPUT_AREAPERLENGTH_UNIT` = `23` — The input area per length unit.
- `OUTPUT_VOLUME_UNIT` = `30` — The output volume unit.
- `INPUT_SECTIONMODULUS_UNIT` = `31` — The input section modulus unit.
- `INPUT_VOLUME_UNIT` = `32` — The input volume unit.
- `INPUT_MOMENTOFINERTIA_UNIT` = `40` — The input moment of inertia unit.
- `INPUT_TORSIONCONSTANT_UNIT` = `41` — The input torsion constant unit.
- `INPUT_WARPINGCONSTANT_UNIT` = `60` — The input warping constant unit.
- `INPUT_FORCE_UNIT` = `100` — The input force unit.
- `OUTPUT_FORCE_UNIT` = `101` — The output force unit.
- `INPUT_WEIGHT_UNIT` = `102` — The input weight unit.
- `OUTPUT_WEIGHT_UNIT` = `103` — The output weight unit.
- `INPUT_DISTRIBLOAD_UNIT` = `110` — The input distributed load unit.
- `OUTPUT_DISTRIBLOAD_UNIT` = `111` — The output distributed load unit.
- `INPUT_SPRINGCONSTANT_UNIT` = `112` — The input spring constant unit.
- `OUTPUT_MASSPERLENGTH_UNIT` = `113` — The output mass per length unit.
- `INPUT_SURFACELOAD_UNIT` = `120` — The input surface load unit.
- `OUTPUT_SURFACELOAD_UNIT` = `121` — The output surface load unit.
- `INPUT_STRENGTH_UNIT` = `122` — The input strength unit.
- `OUTPUT_STRESS_UNIT` = `123` — The output stress unit.
- `INPUT_MODULUS_UNIT` = `124` — The input modulus unit.
- `INPUT_DENSITY_UNIT` = `131` — The input density unit.
- `INPUT_MOMENT_UNIT` = `200` — The input moment unit.
- `OUTPUT_MOMENT_UNIT` = `201` — The output moment unit.
- `INPUT_DISTRIBMOMENT_UNIT` = `205` — The input distributed moment unit.
- `INPUT_ROTSPRINGCONST_UNIT` = `210` — The input rotation spring constant unit.
- `INPUT_TEMPERATURE_UNIT` = `300` — The input temperature unit.
- `OUTPUT_TEMPERATURE_UNIT` = `301` — The output temperature unit.
- `INPUT_THERMDILATCOEFF_UNIT` = `310` — The input thermal dilatation coefficient unit.
- `INPUT_FACTOR_UNIT` = `400` — The input factor unit.
- `INPUT_DATE_UNIT` = `401` — The input date unit.
- `INPUT_DATE_TIME_MIN_UNIT` = `402` — The input date time minutes unit.
- `INPUT_DATE_TIME_SEC_UNIT` = `403` — The input date time seconds unit.
- `INPUT_LENGTH_FRACTIONAL_IMPERIAL` = `1005` — The input length, fractional imperial.
- `INPUT_DEFORMATION_FRACTIONAL_IMPERIAL` = `1305` — The input deformation, fractional imperial.
- `INPUT_DIMENSION_FRACTIONAL_IMPERIAL` = `1405` — The input dimension, fractional imperial.
- `INPUT_RADIUSOFINERTIA_FRACTIONAL_IMPERIAL` = `1605` — The radius of inertia, fractional imperial.
- `OUTPUT_LENGTH_FRACTIONAL_IMPERIAL` = `1105` — The output length, fractional imperial.
- `OUTPUT_DEFORMATION_FRACTIONAL_IMPERIAL` = `1305` — The output deformation, fractional imperial.

## ProfileItemSubType (class)

The ProfileItemSubType class contains the information of one profile item sub type (id, label, parameter string and bitmap name). A profile item can contain a maximum of 10 profile item sub types.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/4261fe17-2e20-9a8a-69e4-343d6e681651)

### Constructors
- `public ProfileItemSubType()` — Initializes a new instance of the ProfileItemSubType class

### Properties
- `BitmapName` (String, get) — Gets a value of the property BitmapName.
- `Label` (String, get) — Gets a value of the property Label.
- `ParameterString` (String, get) — Gets a value of the property ParameterString.
- `SubTypeId` (ProfileItem.ProfileItemSubTypeEnum, get) — Gets a value of the property SubTypeId.

## ProfileName (class)

The ProfileName class contains the name of the profile item.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/f9fe4bfc-f9e6-53a4-a547-c5d1746f4394)

### Constructors
- `public ProfileName()` — Initializes a new instance of the ProfileName class

### Properties
- `Name` (String, get/set) — The profile item name.

## RebarHeaderItem (class)

The RebarHeaderItem class represents an item in the header of the rebar catalog (rebar_database.inp).

[Vendor docs](https://developer.tekla.com/topic/en/18/47/17d6a044-cf93-4155-dd90-aa924948be95)

### Constructors
- `public RebarHeaderItem()` — Initializes a new instance of the RebarHeaderItem class.

### Properties
- `Name` (String, get/set) — Gets or sets the name.
- `PropertyName` (String, get/set) — Gets or sets the property name.
- `Type` (String, get/set) — Gets or sets the type.
- `Units` (String, get/set) — Gets or sets the units.

## RebarItem (class)

The RebarItem class contains information from the rebars in the catalog (rebar_database.inp).

[Vendor docs](https://developer.tekla.com/topic/en/18/47/933c87fb-3e19-dd04-d5c3-92723fca54ca)

### Constructors
- `public RebarItem()` — Creates a new rebar item instance.

### Methods
#### `public bool Delete()`

Deletes the rebar item from the rebar database.

**Returns:** `Boolean` — True on success

[Docs](https://developer.tekla.com/topic/en/18/47/ed0d2530-6424-8337-3f7e-2cee77c6319d)

#### `public bool Export(ref string filename)`

Exports the rebar item in the catlog to given file name. If path is not given rebar is exported to model folder. If filename is empty rebar name is used as filename.

**Parameters:**
- `filename` (System.String.) — The export file name of the item.

**Returns:** `Boolean` — True on success.

[Docs](https://developer.tekla.com/topic/en/18/47/3feb12ed-4c12-9a6c-5dd2-41cbddedc126)

#### `public bool Insert()`

Inserts the rebar item in the rebar database.

**Returns:** `Boolean` — True on success

[Docs](https://developer.tekla.com/topic/en/18/47/304205d9-c7d3-7105-15f3-dce63768ce9a)

#### `public bool Modify(string OriginalGrade, string OriginalSize, string OriginalUsage)`

Modifies the rebar item in the rebar database based on the given parameters.

**Parameters:**
- `OriginalGrade` (System.String) — Original Grade
- `OriginalSize` (System.String) — Original Size
- `OriginalUsage` (System.String) — Original Usage

**Returns:** `Boolean` — True on success

[Docs](https://developer.tekla.com/topic/en/18/47/51fa7a7e-68a3-95d5-d6fe-a8d831f77ad4)

#### `public bool Select(string Grade, double NominalDiameter, bool UseNominalDiameter)`

Selects the rebar item in the rebar database.

**Parameters:**
- `Grade` (System.String) — The grade of the rebar item to select.
- `NominalDiameter` (System.Double) — The diameter of the rebar item to select.
- `UseNominalDiameter` (System.Boolean) — If set to false, the item is selected based on the actual diameter. If true, the item is selected based on the nominal diameter.

**Returns:** `Boolean` — True on success.

[Docs](https://developer.tekla.com/topic/en/18/47/13cbd057-d553-bf4b-4e47-a153a7efdfa9)

#### `public bool Select(string Grade, double Diameter, double BendRadius, bool UseNominalDiameter)`

Selects the rebar item in the rebar database.

**Parameters:**
- `Grade` (System.String) — The grade of the rebar item to select.
- `Diameter` (System.Double) — The diameter of the rebar item to select.
- `BendRadius` (System.Double) — The bending radius of the rebar item to select.
- `UseNominalDiameter` (System.Boolean) — If set to false, the item is selected based on the actual diameter. If true, the item is selected based on the nominal diameter.

**Returns:** `Boolean` — True on success.

[Docs](https://developer.tekla.com/topic/en/18/47/70937de3-c9ba-e60a-55c0-eb149ba46d7d)

#### `public bool Select(string Grade, double Diameter, string Usage, bool UseNominalDiameter)`

Selects the rebar item in the rebar database.

**Parameters:**
- `Grade` (System.String) — The grade of the rebar item to select.
- `Diameter` (System.Double) — The diameter of the rebar item to select.
- `Usage` (System.String) — The usage of the rebar item to select.
- `UseNominalDiameter` (System.Boolean) — If set to false, the item is selected based on the actual diameter. If true, the item is selected based on the nominal diameter.

**Returns:** `Boolean` — True on success.

[Docs](https://developer.tekla.com/topic/en/18/47/0bd0ecb9-f9db-771e-f51e-2f9983b7aafb)

#### `public bool Select(string Grade, string Size)`

Selects the rebar item in the rebar database.

**Parameters:**
- `Grade` (System.String) — The grade of the rebar item to select.
- `Size` (System.String) — The size of the rebar item to select.

**Returns:** `Boolean` — True on success.

[Docs](https://developer.tekla.com/topic/en/18/47/1a861995-0819-f921-aaab-563ae782ba56)

#### `public bool Select(string Grade, string Size, double BendRadius)`

Selects the rebar item in the rebar database.

**Parameters:**
- `Grade` (System.String) — The grade of the rebar item to select.
- `Size` (System.String) — The size of the rebar item to select.
- `BendRadius` (System.Double) — The bending radius of the rebar item to select.

**Returns:** `Boolean` — True on success.

[Docs](https://developer.tekla.com/topic/en/18/47/dd2e488d-3a2a-581d-8ebe-c280fcbef5fb)

#### `public bool Select(string Grade, string Size, string Usage)`

Selects the rebar item in the rebar database.

**Parameters:**
- `Grade` (System.String) — The grade of the rebar item to select.
- `Size` (System.String) — The size of the rebar item to select.
- `Usage` (System.String) — The usage of the rebar item to select.

**Returns:** `Boolean` — True on success.

[Docs](https://developer.tekla.com/topic/en/18/47/a5a6ae29-8271-2eae-e587-fe894af9120e)

### Properties
- `ActualDiameter` (Double, get/set) — The rebar item's actual diameter.
- `BendRadius` (Double, get/set) — The rebar item's bending radius.
- `Code` (String, get/set) — The rebar item's code.
- `CrankedLength` (Double, get/set) — Gets or sets the cranked length.
- `CrankedLengthType` (String, get/set) — Gets or sets the cranked length type.
- `CrankExtraOffset` (Double, get/set) — Gets or sets the crank extra offset.
- `CrankStraightLength` (Double, get/set) — Gets or sets the crank straight length.
- `CrossSectionArea` (Double, get/set) — The rebar item's cross section area.
- `CurveTolerance` (Double, get/set) — Gets or sets the curve tolerance.
- `ExtraPointShortening` (Double, get/set) — Gets or sets the extra point shortening.
- `Grade` (String, get/set) — The rebar item's grade.
- `HookLength135Degrees` (Double, get/set) — The rebar item's hook length for 135 degrees.
- `HookLength180Degrees` (Double, get/set) — The rebar item's hook length for 180 degrees.
- `HookLength90Degrees` (Double, get/set) — The rebar item's hook length for 90 degrees.
- `HookRadius135Degrees` (Double, get/set) — The rebar item's hook radius for 135 degrees.
- `HookRadius180Degrees` (Double, get/set) — The rebar item's hook radius for 180 degrees.
- `HookRadius90Degrees` (Double, get/set) — The rebar item's hook radius for 90 degrees.
- `LapLength` (Double, get/set) — Gets or sets the lap length.
- `MaxRadiusRequiringBending` (Double, get/set) — Gets or sets the max radius requiring bending.
- `NominalDiameter` (Double, get/set) — The rebar item's nominal diameter.
- `Size` (String, get/set) — The rebar item's size.
- `Usage` (String, get/set) — The rebar item's usage.
- `WeightPerLenght` (Double, get/set) — The rebar item's weight per lenght.

## RebarItemEnumerator (class)

The RebarItemEnumerator class allows to loop through the catalog rebar items.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/79244a17-2ec1-6047-bb54-fb7c63ad8d4b)

### Methods
#### `public static bool DeleteRebarItem(RebarItem rebarItem)`

Delete the RebarItem from the list

**Parameters:**
- `rebarItem` (Tekla.Structures.Catalogs.RebarItem) — item to delete

**Returns:** `Boolean` — True on success

[Docs](https://developer.tekla.com/topic/en/18/47/8134716e-7107-ace0-ee37-1762fdc6c6e5)

#### `public static List<RebarHeaderItem> GetRebarHeaderItems()`

Gets the header items from the rebar catalog header.

**Returns:** `List<RebarHeaderItem>` — The rebar header items.

[Docs](https://developer.tekla.com/topic/en/18/47/15c513bc-268d-6adb-2484-6fc63f53586b)

#### `public int GetSize()`

Returns the total amout of items.

**Returns:** `Int32` — The total amount of items.

[Docs](https://developer.tekla.com/topic/en/18/47/143e5791-aab0-dd97-59e4-6dc160365bf8)

#### `public static bool InsertRebarItem(RebarItem rebarItem)`

Insert the RebarItem in to the list

**Parameters:**
- `rebarItem` (Tekla.Structures.Catalogs.RebarItem) — Item to insert

**Returns:** `Boolean` — True on success

[Docs](https://developer.tekla.com/topic/en/18/47/8ccbe8c7-936b-e7f7-5a8b-15bd890c35d7)

#### `public static bool ModifyRebarItem(RebarItem rebarItem, RebarItem originalRebarItem)`

Modify the RebarItem in the list

**Parameters:**
- `rebarItem` (Tekla.Structures.Catalogs.RebarItem) — item to modify
- `originalRebarItem` (Tekla.Structures.Catalogs.RebarItem) — original Rebar Item to match

**Returns:** `Boolean` — True on success

[Docs](https://developer.tekla.com/topic/en/18/47/e12ff711-c46d-beba-9f0a-58c370a56a48)

#### `public bool MoveNext()`

Moves to the next item in the enumerator.

**Returns:** `Boolean` — False on failure.

[Docs](https://developer.tekla.com/topic/en/18/47/16173cf8-253b-7207-0efd-433facfb2a43)

#### `public static bool RebarItemExists(RebarItem rebarItem)`

Check RebarItem is available in database file

**Parameters:**
- `rebarItem` (Tekla.Structures.Catalogs.RebarItem) — rebarItem to check with rebar db

**Returns:** `Boolean` — True on rebar item exist in data base

[Docs](https://developer.tekla.com/topic/en/18/47/91a4841a-1b14-8d2e-a919-f82ac6d98d04)

#### `public void Reset()`

Resets the enumerator to the beginning.

[Docs](https://developer.tekla.com/topic/en/18/47/ad0e7d04-4ef9-97a7-4bea-db64119745ee)

#### `public static RebarItem SelectRebarItem(string Grade, double Diameter, bool UseNominalDiameter)`

Selects a rebar item in the rebar database with the given grade and nominal diameter.

**Parameters:**
- `Grade` (System.String) — The grade of the rebar item to select.
- `Diameter` (System.Double) — The diameter of the rebar item to select.
- `UseNominalDiameter` (System.Boolean) — If set to false, the item is selected based on the actual diameter. If true, the item is selected based on the nominal diameter.

**Returns:** `RebarItem` — The first RebarItem that matches the conditions.

[Docs](https://developer.tekla.com/topic/en/18/47/8070f297-24cb-845f-c6f3-a4c9f4cd18c5)

#### `public static RebarItem SelectRebarItem(string Grade, double Diameter, double BendRadius, bool UseNominalDiameter)`

Selects a rebar item in the rebar database with the given grade, nominal diameter and bending radius.

**Parameters:**
- `Grade` (System.String) — The grade of the rebar item to select.
- `Diameter` (System.Double) — The diameter of the rebar item to select.
- `BendRadius` (System.Double) — The bending radius of the rebar item to select.
- `UseNominalDiameter` (System.Boolean) — If set to false, the item is selected based on the actual diameter. If true, the item is selected based on the nominal diameter.

**Returns:** `RebarItem` — The first RebarItem that matches the conditions.

[Docs](https://developer.tekla.com/topic/en/18/47/57ce99c3-1beb-75bf-b836-a72f15427a21)

#### `public static RebarItem SelectRebarItem(string Grade, double Diameter, string Usage, bool UseNominalDiameter)`

Selects a rebar item in the rebar database with the given grade, nominal diameter and bending radius.

**Parameters:**
- `Grade` (System.String) — The grade of the rebar item to select.
- `Diameter` (System.Double) — The diameter of the rebar item to select.
- `Usage` (System.String) — The usage of the rebar item to select.
- `UseNominalDiameter` (System.Boolean) — If set to false, the item is selected based on the actual diameter. If true, the item is selected based on the nominal diameter.

**Returns:** `RebarItem` — The first RebarItem that matches the conditions.

[Docs](https://developer.tekla.com/topic/en/18/47/711493f8-9bba-e516-550a-e3121156d859)

#### `public static RebarItem SelectRebarItem(string Grade, string Size)`

Selects a rebar item in the rebar database with the given grade and size.

**Parameters:**
- `Grade` (System.String) — The grade of the rebar item to select.
- `Size` (System.String) — The size of the rebar item to select.

**Returns:** `RebarItem` — The first RebarItem that matches the conditions.

[Docs](https://developer.tekla.com/topic/en/18/47/43cc4200-e00f-bc6b-7b54-5fe9aa9eca99)

#### `public static RebarItem SelectRebarItem(string Grade, string Size, double BendRadius)`

Selects a rebar item in the rebar database with the given grade, size and bending radius.

**Parameters:**
- `Grade` (System.String) — The grade of the rebar item to select.
- `Size` (System.String) — The size of the rebar item to select.
- `BendRadius` (System.Double) — The bending radius of the rebar item to select.

**Returns:** `RebarItem` — The first RebarItem that matches the conditions.

[Docs](https://developer.tekla.com/topic/en/18/47/79ebffd5-4708-d0f0-4b97-850d7e31fd8c)

#### `public static RebarItem SelectRebarItem(string Grade, string Size, string Usage)`

Selects a rebar item in the rebar database with the given grade, size and usage.

**Parameters:**
- `Grade` (System.String) — The grade of the rebar item to select.
- `Size` (System.String) — The size of the rebar item to select.
- `Usage` (System.String) — The usage of the rebar item to select.

**Returns:** `RebarItem` — The first RebarItem that matches the conditions.

[Docs](https://developer.tekla.com/topic/en/18/47/a39216d1-61a1-12bb-1df8-440b927be16d)

#### `public static RebarItem SelectRebarItem(string Grade, string Size, string Usage, string Code)`

Selects a rebar item in the rebar database with the given grade, size, usage and code.

**Parameters:**
- `Grade` (System.String) — The grade of the rebar item to select.
- `Size` (System.String) — The size of the rebar item to select.
- `Usage` (System.String) — The usage of the rebar item to select.
- `Code` (System.String) — The code of the rebar item to select.

**Returns:** `RebarItem` — The first RebarItem that matches the conditions.

[Docs](https://developer.tekla.com/topic/en/18/47/7804542c-1ac2-cdff-33bd-19b6a5d4f66b)

### Properties
- `Current` (RebarItem, get) — Returns a rebar item instance of the current element.

## ShapeItem (class)

The Shape class contains information about the shapes in the Tekla Structures shape catalog.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/3b2b9bdd-99b2-1ed9-3e1f-6a79989bfbd8)

### Constructors
- `public ShapeItem()` — Creates a new shape instance.

### Methods
#### `public bool CleanAndModify(ref bool isSolid)`

Cleans and stores the brep into the shape catalog

**Parameters:**
- `isSolid` (System.Boolean.) — Parameter indicating if is solid.

**Returns:** `Boolean` — True if cleaning and storing to catalog was successful

[Docs](https://developer.tekla.com/topic/en/18/47/4edee409-2a8c-032a-7970-1ffafd7de5d2)

#### `public bool Delete()`

Deletes a shape from shape catalog based on the shape name.

**Returns:** `Boolean` — true on success.

[Docs](https://developer.tekla.com/topic/en/18/47/8c1f5ced-f600-2cea-a417-a76c1d7dded0)

#### `public bool DeleteMetadata(string key)`

Delete the Shape item's metadata.

**Parameters:**
- `key` (System.String) — The key of metadata.

**Returns:** `Boolean` — True if success or False if key not found or deletion failed.

[Docs](https://developer.tekla.com/topic/en/18/47/76dde3bd-f5f5-1871-781d-9ed67bc3627f)

#### `public virtual bool Export(ref string filename)`

Exports the shape item in *.tsc-format to the to given file name. If path is not given shape is exported to model folder. If filename is empty shape name is used as filename.

**Parameters:**
- `filename` (System.String.) — The export file name of the shape item.

**Returns:** `Boolean` — True on success.

[Docs](https://developer.tekla.com/topic/en/18/47/c627bccd-30f5-48b6-0ed0-158be2852701)

#### `public List<string> GetAllMetadataKeys()`

Get the Shape item's metadata keys.

**Returns:** `List<String>` — A list of strings containing the keys.

[Docs](https://developer.tekla.com/topic/en/18/47/ff0a336a-d372-dee9-0668-a42a5a154dd6)

#### `public int GetInstanceCount()`

Get the number of instances used in the model of a shape

**Returns:** `Int32` — The number of shape instances

[Docs](https://developer.tekla.com/topic/en/18/47/42331a86-fac0-a1be-b369-7e3274919ce5)

#### `public bool GetMetadata(string key, ref ShapeMetadataTypeEnum type, ref Object value, ref string label, ref string description)`

Get the Shape item's metadata.

**Parameters:**
- `key` (System.String) — The key of metadata.
- `type` (Tekla.Structures.Catalogs.ShapeMetadataTypeEnum.) — The type of metadata.
- `value` (System.Object.) — The value of metadata.
- `label` (System.String.) — The label of metadata.
- `description` (System.String.) — The description of metadata.

**Returns:** `Boolean` — True if success or False

[Docs](https://developer.tekla.com/topic/en/18/47/1b91542e-8190-0ef5-0b9f-b31aa67cf9eb)

#### `public bool Insert()`

Inserts a shape to the shape catalog based on the shape geometry (does not allow duplicate geometry).

**Returns:** `Boolean` — Return true on success.

[Docs](https://developer.tekla.com/topic/en/18/47/16c8423e-6b2e-db2c-d8d7-7f6e267eac0c)

#### `public bool InsertOrGetGuidsOfShapesWithMatchingGeometry(out List<string> existingShapeGuids)`

Inserts a shape to the shape catalog based on the shape geometry (does not allow duplicate geometry). If, based on fingerprint, the geometry already exists it populates a list of Guids of shapes using that same geometry.

**Parameters:**
- `existingShapeGuids` (System.Collections.Generic.List<String>.) — A list of guids of any shapes with matching geometry fingerprints.

**Returns:** `Boolean` — Returns true on success; false if not successful.

[Docs](https://developer.tekla.com/topic/en/18/47/c2fb6b7e-357a-84f5-2d75-5df5689de671)

#### `public bool InsertUsingNormals()`

Inserts a shape to the shape catalog using the shape geometry. Uses vertex normals to determine edge visibility. If the normal vectors of the vertices belonging to one geometrical location are close enough, the edge will be considered smooth and will be marked as hidden.

**Returns:** `Boolean` — Returns true on success.

[Docs](https://developer.tekla.com/topic/en/18/47/11c11135-9e1b-f831-fcd5-5c5c894ed8a2)

#### `public bool InsertUsingNormalsAllowDuplicates()`

Inserts a shape to the shape catalog using the shape geometry allowing duplicates. Uses vertex normals to determine edge visibility. If the normal vectors of the vertices belonging to one geometrical location are close enough, the edge will be considered smooth and will be marked as hidden.

**Returns:** `Boolean` — Returns true on success.

[Docs](https://developer.tekla.com/topic/en/18/47/7aa8d76b-bf72-dc00-4c25-9db20221065e)

#### `public bool Modify()`

Modifies a shape in the shape catalog based on the shape name or if not found, based on shape guid.

**Returns:** `Boolean` — Return true on success.

[Docs](https://developer.tekla.com/topic/en/18/47/e26d3968-b0c4-cbe4-7072-5c594b6e10ee)

#### `public bool Rename(string newName)`

Renames a shape in the shape catalog with the given shape name.

**Parameters:**
- `newName` (System.String) — The new name.

**Returns:** `Boolean` — True on success. False if the new name is empty or duplicated as the current name, or other exceptions.

[Docs](https://developer.tekla.com/topic/en/18/47/e5d7d104-0f1a-2437-122c-c95d57bc1ecd)

#### `public bool Select()`

Selects the shape from the database based on the name given in this instance.

**Returns:** `Boolean` — True on success. False if the shape was not found.

[Docs](https://developer.tekla.com/topic/en/18/47/44b27784-3b82-a5a3-48bd-8b6d31e62693)

#### `public bool Select(string name)`

Selects the shape based on the given name from the database.

**Parameters:**
- `name` (System.String) — The name of the shape.

**Returns:** `Boolean` — True on success. False if the shape was not found.

[Docs](https://developer.tekla.com/topic/en/18/47/78fd7218-d17a-1dd0-fd3c-bda69433da65)

#### `public bool SetHandlePoints(List<Point> HandlePoints)`

Set the Shape item's Handle Points

**Parameters:**
- `HandlePoints` (System.Collections.Generic.List<Point>) — The handle points.

**Returns:** `Boolean` — True if success or False

[Docs](https://developer.tekla.com/topic/en/18/47/32d1759c-965f-dda0-00c4-29c8348115f3)

#### `public bool SetMetadata(string key, ShapeMetadataTypeEnum type, Object value, string label, string description)`

Set the Shape item's metadata. Inserts the metadata if key is not found, otherwise modifies existing data.

**Parameters:**
- `key` (System.String) — The key of metadata.
- `type` (Tekla.Structures.Catalogs.ShapeMetadataTypeEnum) — The type of metadata.
- `value` (System.Object) — The value of metadata.
- `label` (System.String) — The label of metadata.
- `description` (System.String) — The description of metadata.

**Returns:** `Boolean` — True if success or False

[Docs](https://developer.tekla.com/topic/en/18/47/8053f599-8299-bd82-4c5b-85741968a43c)

### Properties
- `BrepType` (BrepType, get/set) — Defines BrepType of shapes. @see enum BrepType
- `Extrema` (AABB, get) — The extrema of the shape as an axis-aligned bounding box, as opposed to object-aligned
- `Fingerprint` (String, get) — A fingerprint value calculated by Tekla Structures to provide quick comparison of geometries to avoid inserting same shape multiple times to the catalog. More rigorous comparison is done only for geometries that result in identical fingerprints.
- `GeometryGuid` (String, get) — A unique identifier of the shape geometry, given initially by BrepStorage. This GUID is used as the body of the filename for the shape geometry information found in the ShapeGeometries directory
- `GeometryHash` (String, get) — This obsolete property can still be used to store and externally provided unique hash value identifying the shape. However, Tekla Structures does not use this value internally for anything. It is strongly recommended to use the newer Fingerprint property for future implementations. That value is automatically calculated by Tekla Structures for all inserted shapes and is used internally for shape identification.
- `Guid` (String, get) — A unique identifier of the actual shape, given initially by the ShapeCatalog. This GUID is used as the body of the filename for the shape information found in the Shape directory
- `HandlePoints` (List<Point>, get/set) — Defines handle points of the shape.
- `IsSolid` (Boolean, get) — Set to true if the shape is detected by TS Core to be a valid solid
- `Name` (String, get/set) — The shape Name
- `ShapeFacetedBrep` (FacetedBrep, get/set) — The data structure containing the geometric information of the shape as a FacetedBRep
- `UpAxis` (ShapeUpAxis, get/set) — The direction defining what is understood as "up" in the shape that the API user is providing. Typically this is the z-axis, if the data is in global coordinates, coming from a system where Z-axis points to global up direction. Possible values are: Undefined = 0, X_Axis = 1, Y_Axis = 2, Z_Axis = 3. It is highly recommendable to orient shapes in the Tekla Structures native way, that is x-axis on the extrusion line and Y-axis pointing up. NOTE: Specifying UpAxis to be ShapeUpAxis.Undefined may throw a ‘System.ArgumentOutOfRangeException’ exception in certain operations, because the system can not proceed without that information.

## ShapeItemEnumerator (class)

The ShapeItemEnumerator class allows to loop through the shape catalog items.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/fb2bc7c4-c0bc-7536-a3e9-9adaa1186d68)

### Methods
#### `public int GetSize()`

Returns the total amount of items.

**Returns:** `Int32` — The total amount of items.

[Docs](https://developer.tekla.com/topic/en/18/47/f53a8688-6506-8c9c-8e75-3c2cdf477af7)

#### `public bool MoveNext()`

Moves to the next item in the enumerator.

**Returns:** `Boolean` — False on failure.

[Docs](https://developer.tekla.com/topic/en/18/47/6b563dd5-5bed-aeb1-4171-dee33e4ea36d)

#### `public void Reset()`

Resets the enumerator to the beginning.

[Docs](https://developer.tekla.com/topic/en/18/47/932e93cd-e0ef-c8a5-48c7-f176dcaff4b5)

### Properties
- `Current` (ShapeItem, get) — Returns a shape item instance of the current element.

## ShapeMetadataTypeEnum (enum)

Represents data type or measurement unit of the specific metadata If this is updated, update ShapeMetadataType_e on the Core side

[Vendor docs](https://developer.tekla.com/topic/en/18/47/c664c68d-0e1f-afdd-716c-9736a174f994)

### Values
- `Text` = `0` — Default value is text
- `DateTime` = `10` — Date
- `Integer` = `11` — Integer
- `Double` = `12` — Double, here precision controlled by TS-> Menu-> Settings-> Options-> Units and decimals -> Modeling -> Factor "XS_UNITDECIMALS_INPUT_FACTOR"
- `Length` = `101` — Unit of length, here length refers to TS-> Menu-> Settings-> Options-> Units and decimals -> Catalogs -> Dimension. "XS_UNIT_INPUT_DIMENSION", "XS_UNITDECIMALS_INPUT_DIMENSION"
- `Area` = `201` — Unit of area, here area refers to TS-> Menu-> Settings-> Options-> Units and decimals -> Catalogs -> Area. "XS_UNIT_INPUT_AREA", "XS_UNITDECIMALS_INPUT_AREA"
- `Volume` = `301` — Unit of volume, here volume refers to TS-> Menu-> Settings-> Options-> Units and decimals -> Catalogs -> Volume. "XS_UNIT_INPUT_VOLUME", "XS_UNITDECIMALS_INPUT_VOLUME"
- `Weight` = `401` — Unit of weight, here weight refers to TS-> Menu-> Settings-> Options-> Units and decimals -> Catalogs -> Weight. "XS_UNIT_INPUT_WEIGHT", "XS_UNITDECIMALS_INPUT_WEIGHT"

## ShapeUpAxis (enum)

Up axis direction

[Vendor docs](https://developer.tekla.com/topic/en/18/47/a182b297-b6de-a62e-79fa-91e7df0e72fc)

### Values
- `Undefined` = `0` — Undefined direction for axis. Should not be used in input.
- `X_Axis` = `1` — X points up.
- `Y_Axis` = `2` — Y points up.
- `Z_Axis` = `3` — Z points up.

## UserPropertyFieldTypeEnum (enum)

The user property field type.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/8dd87f1e-3621-dc2e-f515-2d1678ef0e36)

### Values
- `FIELDTYPE_UNDEFINED` = `0` — The user property field type is undefined.
- `FIELDTYPE_NUMBER` = `1` — The user property field type is number.
- `FIELDTYPE_TEXT` = `2` — The user property field type is text.
- `FIELDTYPE_DISTANCE` = `3` — The user property field type is distance.
- `FIELDTYPE_PROFILE` = `4` — The user property field type is profile.
- `FIELDTYPE_MATERIAL` = `5` — The user property field type is material.
- `FIELDTYPE_TEXT_LIST_DISTANCE` = `6` — The user property field type is distance list.
- `FIELDTYPE_FILE_IN` = `7` — The user property field type is file in.
- `FIELDTYPE_FILE_OUT` = `8` — The user property field type is file out.
- `FIELDTYPE_BOLT_STANDARD` = `9` — The user property field type is bolt standard.
- `FIELDTYPE_BOLT_SIZE` = `10` — The user property field type is bolt size.
- `FIELDTYPE_RATIO` = `11` — The user property field type is ratio.
- `FIELDTYPE_STRAIN` = `12` — The user property field type is strain.
- `FIELDTYPE_ANGLE` = `13` — The user property field type is angle.
- `FIELDTYPE_DEFORMATION` = `14` — The user property field type is deformation.
- `FIELDTYPE_DIMENSION` = `15` — The user property field type is dimension.
- `FIELDTYPE_RADIUSOFINERTIA` = `16` — The user property field type is radius of inertia.
- `FIELDTYPE_AREA` = `17` — The user property field type is area.
- `FIELDTYPE_AREAPERLENGTH` = `18` — The user property field type is area/length.
- `FIELDTYPE_SECTIONMODULUS` = `19` — The user property field type is section modulus.
- `FIELDTYPE_MOMENTOFINERTIA` = `20` — The user property field type is moment of inertia.
- `FIELDTYPE_TORSIONCONSTANT` = `21` — The user property field type is torsion constant.
- `FIELDTYPE_WARPINGCONSTANT` = `22` — The user property field type is warping constant.
- `FIELDTYPE_FORCE` = `23` — The user property field type is force.
- `FIELDTYPE_WEIGHT` = `24` — The user property field type is weight.
- `FIELDTYPE_DISTRIBLOAD` = `25` — The user property field type is distributed load.
- `FIELDTYPE_SPRINGCONSTANT` = `26` — The user property field type is spring constant.
- `FIELDTYPE_SURFACELOAD` = `27` — The user property field type is surface load.
- `FIELDTYPE_STRENGTH` = `28` — The user property field type is strength.
- `FIELDTYPE_MODULUS` = `29` — The user property field type is modulus.
- `FIELDTYPE_DENSITY` = `30` — The user property field type is density.
- `FIELDTYPE_MOMENT` = `31` — The user property field type is moment.
- `FIELDTYPE_DISTRIBMOMENT` = `32` — The user property field type is distributed moment.
- `FIELDTYPE_ROTSPRINGCONST` = `33` — The user property field type is rotational spring constant.
- `FIELDTYPE_TEMPERATURE` = `34` — The user property field type is temperature.
- `FIELDTYPE_THERMDILATCOEFF` = `35` — The user property field type is thermal coefficient.
- `FIELDTYPE_ANALYSIS_RESTRAINT` = `36` — The user property field type is analysis restraint.
- `FIELDTYPE_VOLUME` = `37` — The user property field type is volume.
- `FIELDTYPE_REBAR_MAIN` = `38` — The user property field type is main reinforcement bar.
- `FIELDTYPE_REBAR_STIRRUP` = `39` — The user property field type is stirrup reinforcement bar.
- `FIELDTYPE_DATE` = `40` — The user property field type is date.
- `FIELDTYPE_DATE_TIME_SEC` = `41` — The user property field type is date and time with seconds.
- `FIELDTYPE_DATE_TIME_MIN` = `42` — The user property field type is date and time with minutes.
- `FIELDTYPE_STUD_STANDARD` = `43` — The user property field type is stud standard.
- `FIELDTYPE_STUD_SIZE` = `44` — The user property field type is stud size.
- `FIELDTYPE_STUD_LENGTH` = `45` — The user property field type is stud length.
- `FIELDTYPE_HOLE_TYPE` = `46` — The user property field type is hole type.
- `FIELDTYPE_HOLE_DIRECTION` = `47` — The user property field type is hole direction.
- `FIELDTYPE_WELD_TYPE` = `48` — The user property field type is weld type.
- `FIELDTYPE_CHAMFER_TYPE` = `49` — The user property field type is chamfer type.
- `FIELDTYPE_WELDING_SITE` = `50` — The user property field type is welding site.
- `FIELDTYPE_FACTOR` = `51` — The user property field type is factor.
- `FIELDTYPE_PART_NAME` = `52` — The user property field type is part name.
- `FIELDTYPE_BOLT_TYPE` = `53` — The user property field type is bolt type.
- `FIELDTYPE_COMPONENT_NAME` = `54` — The user property field type is component name.
- `FIELDTYPE_REBAR_MESH` = `55` — The user property field type is rebar mesh.
- `FIELDTYPE_USERDEFINED` = `56` — The user property field type is user defined.
- `FIELDTYPE_YES_NO` = `57` — The user property field type is yes/no.
- `FIELDTYPE_COMPONENT_STANDARD_FILE` = `58` — The user property field type is component standard file.
- `FIELDTYPE_REBAR_GRADE` = `59` — The user property field type is reinforcement bar grade.
- `FIELDTYPE_REBAR_RADIUS` = `60` — The user property field type is reinforcement bar radius.
- `FIELDTYPE_REBAR_SIZE` = `61` — The user property field type is reinforcement bar size.
- `FIELDTYPE_HOOK_SHAPE` = `62` — The user property field type is reinforcement bar hook shape.
- `FIELDTYPE_CROSSBAR_POSITION` = `63` — The user property field type is reinforcement cross bar position.
- `FIELDTYPE_REBAR_SPLIT_TARGET` = `64` — The user property field type is rebar split target.
- `FIELDTYPE_REBAR_STAGGER_TYPE` = `65` — The user property field type is rebar stagger type.
- `FIELDTYPE_REBAR_LAPPING_SIDE` = `66` — The user property field type is rebar lapping side.
- `FIELDTYPE_REBAR_LAPPING_OFFSET_DIRECTION` = `67` — The user property field type is rebar lapping offset direction.
- `FIELDTYPE_REBAR_LAPPING_OFFSET_TYPE` = `68` — The user property field type is rebar lapping offset type.
- `FIELDTYPE_DISTANCE_LIST_TOTAL` = `69` — The user property field type is distance list total.
- `FIELDTYPE_REBAR_LENGTH_ADJUSTMENT_TYPE` = `70` — The user property field type is rebar length adjustment type.
- `FIELDTYPE_SHAPE` = `71` — The user property field type is shape.
- `FIELDTYPE_PLAIN_HOLE_TYPE` = `72` — The user property field type is plain hole type.
- `FIELDTYPE_MASS_FLOW` = `73` — The user property field type is mass flow.
- `FIELDTYPE_VOLUME_FLOW` = `74` — The user property field type is volume flow.
- `FIELDTYPE_HEAT_CAPACITY` = `75` — The user property field type is heat capacity.
- `FIELDTYPE_PRESSURE_LOSS` = `76` — The user property field type is pressure loss.
- `FIELDTYPE_VELOCITY` = `77` — The user property field type is velocity.
- `FIELDTYPE_FLOW_COEFFICIENT` = `78` — The user property field type is flow coefficient.
- `FIELDTYPE_DYNAMIC_VISCOSITY` = `79` — The user property field type is dynamic viscosity.
- `FIELDTYPE_KINEMATIC_VISCOSITY` = `80` — The user property field type is kinematic viscosity.
- `FIELDTYPE_POWER` = `81` — The user property field type is power.

## UserPropertyItem (class)

The UserPropertyItem class contains information about the user properties in the Tekla Structures catalog.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/1b99d080-81f2-5aed-63b2-9408018e79e4)

### Constructors
- `public UserPropertyItem()` — Creates a new user property item instance.

### Methods
#### `public bool AddToObjectType(CatalogObjectTypeEnum objectType)`

Adds the user property item to the given object type.

**Parameters:**
- `objectType` (Tekla.Structures.Catalogs.CatalogObjectTypeEnum) — The object type to add the item to.

**Returns:** `Boolean` — True on success.

[Docs](https://developer.tekla.com/topic/en/18/47/7be988f0-9fd3-7d16-6e38-f65d98b7c422)

#### `public bool Delete()`

Deletes the user property item from the database.

**Returns:** `Boolean` — True on success.

[Docs](https://developer.tekla.com/topic/en/18/47/ae8ea363-5ba4-8dd1-9047-a0ac98d4eb4f)

#### `public bool GetDefaultValue(ref double DefaultValue)`

Gets the default value of a double property.

**Parameters:**
- `DefaultValue` (System.Double.) — The value that was gotten.

**Returns:** `Boolean` — True on success.

[Docs](https://developer.tekla.com/topic/en/18/47/0d6f5378-603f-d5b6-5023-b2117b19ed7a)

#### `public bool GetDefaultValue(ref int DefaultValue)`

Gets the default value of an integer property.

**Parameters:**
- `DefaultValue` (System.Int32.) — The value that was gotten.

**Returns:** `Boolean` — True on success.

[Docs](https://developer.tekla.com/topic/en/18/47/4ce40f2a-3bdd-a3e2-383f-ba34d0890923)

#### `public bool GetDefaultValue(ref string DefaultValue)`

Gets the default value of a string property.

**Parameters:**
- `DefaultValue` (System.String.) — The value that was gotten.

**Returns:** `Boolean` — True on success.

[Docs](https://developer.tekla.com/topic/en/18/47/9aee87ff-94be-8506-3db5-025c3ffee451)

#### `public string GetLabel()`

Gets the label of the user property. Labels are translated when translation is available.

**Returns:** `String` — The label of the user property.

[Docs](https://developer.tekla.com/topic/en/18/47/03fdb5ab-ce53-fbda-e9e7-18f22fde4481)

#### `public string GetLabel(bool translated)`

Gets the label of the user property.

**Parameters:**
- `translated` (System.Boolean) — if set to true translated text is returned.

**Returns:** `String` — The label of the user property.

[Docs](https://developer.tekla.com/topic/en/18/47/18841e46-86a4-0941-eb7d-1bdd69976904)

#### `public bool GetObjectTypes(ref List<CatalogObjectTypeEnum> objectTypes)`

Gets the object types which contain this user property item.

**Parameters:**
- `objectTypes` (System.Collections.Generic.List<CatalogObjectTypeEnum>.) — The object types that were gotten.

**Returns:** `Boolean` — True on success.

[Docs](https://developer.tekla.com/topic/en/18/47/dc091818-71be-e6e4-b292-73185fafeedc)

#### `public bool GetOptions(ref List<KeyValuePair<double, string>> DoubleOptions)`

Gets the value options of a double property. Option strings are translated when translation is available.

**Parameters:**
- `DoubleOptions` (System.Collections.Generic.List<KeyValuePair<Double,String>>.) — The value options.

**Returns:** `Boolean` — True on success.

[Docs](https://developer.tekla.com/topic/en/18/47/508fd0e9-fe08-eb93-fedd-4f3b52e073b2)

#### `public bool GetOptions(ref List<KeyValuePair<double, string>> DoubleOptions, bool translatedLabels)`

Gets the value options of a double property.

**Parameters:**
- `DoubleOptions` (System.Collections.Generic.List<KeyValuePair<Double,String>>.) — The value options.
- `translatedLabels` (System.Boolean) — if set to true labels translated to current language are returned, otherwise the resource key of label is returned.

**Returns:** `Boolean` — True on success.

[Docs](https://developer.tekla.com/topic/en/18/47/4f65183f-06de-d81c-48a9-ec1d346fc50d)

#### `public bool GetOptions(ref List<KeyValuePair<int, string>> IntOptions)`

Gets the value options of an integer property. Option strings are translated when translation is available.

**Parameters:**
- `IntOptions` (System.Collections.Generic.List<KeyValuePair<Int32,String>>.) — The value options.

**Returns:** `Boolean` — True on success.

[Docs](https://developer.tekla.com/topic/en/18/47/00d0baf4-c0ce-d26e-d037-f0c342d92103)

#### `public bool GetOptions(ref List<KeyValuePair<int, string>> IntOptions, bool translatedLabels)`

Gets the value options of an integer property.

**Parameters:**
- `IntOptions` (System.Collections.Generic.List<KeyValuePair<Int32,String>>.) — The value options.
- `translatedLabels` (System.Boolean) — if set to true labels translated to current language are returned, otherwise the resource key of label is returned.

**Returns:** `Boolean` — True on success.

[Docs](https://developer.tekla.com/topic/en/18/47/a9d61eee-15e4-687e-5ddf-1983dcc071ce)

#### `public bool GetOptions(ref List<KeyValuePair<string, string>> StringOptions)`

Gets the value options of a string property. Option strings are translated when translation is available.

**Parameters:**
- `StringOptions` (System.Collections.Generic.List<KeyValuePair<String,String>>.) — The value options.

**Returns:** `Boolean` — True on success.

[Docs](https://developer.tekla.com/topic/en/18/47/a4c8794d-754a-accd-a966-e934fcdc7514)

#### `public bool GetOptions(ref List<KeyValuePair<string, string>> StringOptions, bool translatedLabels)`

Gets the value options of a string property.

**Parameters:**
- `StringOptions` (System.Collections.Generic.List<KeyValuePair<String,String>>.) — The value options.
- `translatedLabels` (System.Boolean) — if set to true labels translated to current language are returned, otherwise the resource key of label is returned.

**Returns:** `Boolean` — True on success.

[Docs](https://developer.tekla.com/topic/en/18/47/c19871a3-e7e9-1974-1174-e27b24e8a5cc)

#### `public bool Insert()`

Inserts the user property item to the database.

**Returns:** `Boolean` — True on success.

[Docs](https://developer.tekla.com/topic/en/18/47/47df9e60-ef7a-23f0-6e08-fd85a21b2be6)

#### `public bool Modify()`

Modifies the user property item.

**Returns:** `Boolean` — True on success.

[Docs](https://developer.tekla.com/topic/en/18/47/44b393b8-36cb-4d17-b620-6e58d7fa3419)

#### `public bool RemoveFromObjectType(CatalogObjectTypeEnum objectType)`

Removes the user property item from the given object type.

**Parameters:**
- `objectType` (Tekla.Structures.Catalogs.CatalogObjectTypeEnum) — The object type to remove the item from.

**Returns:** `Boolean` — True on success.

[Docs](https://developer.tekla.com/topic/en/18/47/fb4c83dc-103a-28f3-5acf-402ef8bd8f92)

#### `public bool Select()`

Selects by name the user property item from the database.

**Returns:** `Boolean` — True on success.

[Docs](https://developer.tekla.com/topic/en/18/47/6f324d5f-053a-8901-67f1-51e882ed9def)

#### `public bool SetDefaultValue(double DefaultValue)`

Sets the default value of a double property.

**Parameters:**
- `DefaultValue` (System.Double) — The new value.

**Returns:** `Boolean` — True on success.

[Docs](https://developer.tekla.com/topic/en/18/47/6270ebb7-1318-06d1-2bbc-efd908962c13)

#### `public bool SetDefaultValue(int DefaultValue)`

Sets the default value of an integer property.

**Parameters:**
- `DefaultValue` (System.Int32) — The new value.

**Returns:** `Boolean` — True on success.

[Docs](https://developer.tekla.com/topic/en/18/47/5165d6f9-44d7-840d-61df-ff0f976b27ea)

#### `public bool SetDefaultValue(string DefaultValue)`

Sets the default value of a string property.

**Parameters:**
- `DefaultValue` (System.String) — The new value.

**Returns:** `Boolean` — True on success.

[Docs](https://developer.tekla.com/topic/en/18/47/758fac4e-2e53-6483-86fc-57ebf8a819ad)

#### `public void SetLabel(string label)`

Sets the label of the user property.

**Parameters:**
- `label` (System.String) — The new label.

[Docs](https://developer.tekla.com/topic/en/18/47/682151bd-ed6f-08dc-c84d-d0f91bb708a7)

#### `public bool SetOptions(List<KeyValuePair<double, string>> DoubleOptions)`

Sets the value options of a double property. The maximum count of options is 400.

**Parameters:**
- `DoubleOptions` (System.Collections.Generic.List<KeyValuePair<Double,String>>) — The value options.

**Returns:** `Boolean` — True on success.

[Docs](https://developer.tekla.com/topic/en/18/47/20d1c30f-74a1-995d-9972-fb0907950eab)

#### `public bool SetOptions(List<KeyValuePair<int, string>> IntOptions)`

Sets the value options of an integer property. The maximum count of options is 400.

**Parameters:**
- `IntOptions` (System.Collections.Generic.List<KeyValuePair<Int32,String>>) — The value options.

**Returns:** `Boolean` — True on success.

[Docs](https://developer.tekla.com/topic/en/18/47/d27524ad-3dad-136a-19b7-fb462072afda)

#### `public bool SetOptions(List<KeyValuePair<string, string>> StringOptions)`

Sets the value options of a string property. The maximum count of options is 400.

**Parameters:**
- `StringOptions` (System.Collections.Generic.List<KeyValuePair<String,String>>) — The new value options.

**Returns:** `Boolean` — True on success.

[Docs](https://developer.tekla.com/topic/en/18/47/3004f766-2f4e-9a7b-3561-2b5030fadb0a)

### Properties
- `AffectsNumbering` (Boolean, get/set) — Indicates whether the property value affects the numbering of objects.
- `FieldType` (UserPropertyFieldTypeEnum, get/set) — The field type of the user property.
- `Level` (UserPropertyLevelEnum, get/set) — The level at which the user property has been defined.
- `Name` (String, get/set) — The name of the user property.
- `Type` (PropertyTypeEnum, get/set) — The type of the user property.
- `Unique` (Boolean, get/set) — Indicates whether the property value is copied when the object is copied.
- `Visibility` (UserPropertyVisibilityEnum, get/set) — Indicates whether the property value is visible/editable.

## UserPropertyItemEnumerator (class)

The UserPropertyItemEnumerator class allows to loop through the user property items.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/ac5773e5-b29e-a74e-ed36-163d2f78cdac)

### Methods
#### `public int GetSize()`

Returns the total amout of items.

**Returns:** `Int32` — The total amount of items.

[Docs](https://developer.tekla.com/topic/en/18/47/08edeb18-b3bc-a616-2b85-b3665fb08417)

#### `public bool MoveNext()`

Moves to the next item in the enumerator.

**Returns:** `Boolean` — False on failure.

[Docs](https://developer.tekla.com/topic/en/18/47/0b52f390-c822-76f2-82d3-0e016b43c0c9)

#### `public void Reset()`

Resets the enumerator to the beginning.

[Docs](https://developer.tekla.com/topic/en/18/47/2f41c308-cf0d-05b7-939b-cc7d38aa8713)

### Properties
- `Current` (UserPropertyItem, get) — Returns a user property item instance of the current element.

## UserPropertyLevelEnum (enum)

The user property level.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/3809973f-fff1-b1d8-28d0-84d1de565582)

### Values
- `LEVEL_MODEL` = `1` — The user property has been defined as a model user property.
- `LEVEL_PROJECT` = `2` — The user property has been defined as a project user property.
- `LEVEL_FIRM` = `3` — The user property has been defined as a firm user property.
- `LEVEL_ENVIRONMENT` = `4` — The user property has been defined as an environment user property.
- `LEVEL_COMMONDEFAULT` = `5` — The user property has been defined as a common default user property.

## UserPropertyName (class)

The UserPropertyName class contains the name of the user property item.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/42e8df58-db5b-2799-58a9-9ba5cc5f9f99)

### Constructors
- `public UserPropertyName()` — Initializes a new instance of the UserPropertyName class

### Properties
- `Name` (String, get/set) — The user property item name.

## UserPropertyOption (class)

The UserPropertyOption class contains the properties of a user property value option.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/86507770-2daf-f78b-4ecc-008b470b1721)

### Constructors
- `public UserPropertyOption()` — Initializes a new instance of the UserPropertyOption class

### Properties
- `DoubleValue` (Double, get/set) — The double value.
- `IntValue` (Int32, get/set) — The integer value.
- `OptionLabel` (String, get/set) — The option name.
- `OptionLabelTranslated` (String, get/set) — The translated option name.
- `StringValue` (String, get/set) — The string value.

## UserPropertyVisibilityEnum (enum)

The visibility of the user property.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/219352c9-b588-0b0b-133a-a94dd92eb15d)

### Values
- `VISIBILITY_NORMAL` = `0` — The user property is visible and the user can modify it.
- `VISIBILITY_READONLY` = `1` — The user property is visible but the user cannot modify it.
- `VISIBILITY_HIDDEN` = `2` — The user property is hidden.

