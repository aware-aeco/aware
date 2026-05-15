---
parent: tekla-structures-catalogs
description: Full API reference for Tekla.Structures.Catalogs classes, profile items, material items, bolt items, rebar items, mesh items, enumerators.
---

## CatalogHandler (Central Entry Point)

### CatalogHandler
> The CatalogHandler class is a class from which the user can query catalog instances.

**Constructors:** `CatalogHandler()`

**Methods - Get Enumerators:**
- `BoltItemEnumerator GetBoltItems()` - get all bolt catalog items
- `ComponentItemEnumerator GetComponentItems()` - get all component catalog items
- `DrawingItemEnumerator GetDrawingItems()` - get all master drawing catalog items
- `MaterialItemEnumerator GetMaterialItems()` - get all material catalog items
- `MaterialMarketSizesItemEnumerator GetMaterialMarketSizes()` - get market sizes for materials
- `MeshItemEnumerator GetMeshItems()` - get all mesh catalog items
- `PrinterItemEnumerator GetPrinterItems()` - get all printer catalog items
- `ProfileItemEnumerator GetProfileItems()` - get all profile catalog items
- `ProfileItemEnumerator GetLibraryProfileItems()` - get library profile items only
- `ProfileItemEnumerator GetParametricProfileItems()` - get parametric profile items only
- `RebarItemEnumerator GetRebarItems()` - get all rebar catalog items
- `ShapeItemEnumerator GetShapeItems()` - get all shape catalog items
- `UserPropertyItemEnumerator GetUserPropertyItems()` - get all user property items
- `UserPropertyItemEnumerator GetUserPropertyItems(CatalogObjectTypeEnum objectType)` - get user property items for a specific object type

**Methods - Import:**
- `bool ImportBoltItems(string path)` - import bolt items from file
- `bool ImportCustomComponentItems(string path)` - import custom component items from file
- `bool ImportDrawingItems(string path)` - import drawing items from file
- `bool ImportLibraryProfileItems(string path)` - import library profile items from file
- `bool ImportMaterialItems(string path)` - import material items from file
- `bool ImportMeshItems(string path)` - import mesh items from file
- `bool ImportParametricProfileItems(string path)` - import parametric profile items from file
- `bool ImportRebarItems(string path)` - import rebar items from file
- `bool ImportShapeItems(string path)` - import shape items from file
- `bool ImportShapeItemsFromGeometryFiles(string path)` - import shape items from geometry files

**Methods - Export/Other:**
- `bool ExportProfileItems(IList<string> profileNames, string path, string fileName)` - export specific profiles
- `bool SaveProfileDatabase()` - save profile database
- `bool GetConnectionStatus()` - check if catalog connection is active
- `static IEnumerable<FinishItem> GetSteelFinishItems()` - get steel finish items

---

## Bolt Catalog

### BoltItem
> The BoltItem class contains information about the bolts in the Tekla Structures bolt catalog.

**Constructors:** `BoltItem()`

**Properties:**
- `List<double> Lengths { get; }` - the bolt item's length values
- `double Size { get; set; }` - the bolt item's size
- `string Standard { get; set; }` - the bolt item's grade
- `BoltItemTypeEnum Type { get; set; }` - the bolt item's type

**Methods:**
- `bool ExportBoltStandard(ref string filename)` - export bolt standard to file

### BoltItem.BoltItemTypeEnum
> Defines the different bolt item types.

| Value | Int | Description |
|-------|-----|-------------|
| BOLT_UNKNOWN | 0 | The unknown bolt item type |
| BOLT | 1 | The bolt type |
| STUD | 2 | The stud type |

### BoltItemEnumerator (sealed)
> The BoltItemEnumerator class allows to loop through the bolt catalog items.

**Properties:**
- `BoltItem Current { get; }` - returns a bolt item instance of the current element

**Methods:**
- `int GetSize()` - total number of items
- `bool MoveNext()` - advance to next item
- `void Reset()` - reset to beginning

### BoltName (sealed)
> The BoltName class contains the name of the bolt item.

**Constructors:** `BoltName()`

**Properties:**
- `string Name { get; set; }` - the bolt item name

---

## Profile Catalog

### ProfileItem (abstract)
> The ProfileItem abstract class contains the common information of catalog profiles (parametric and library).

**Properties:**
- `ArrayList aProfileItemCrossSections { get; }` - array list with the profile item cross-sections
- `ArrayList aProfileItemParameters { get; }` - array list with the profile item parameters
- `bool IsCLBProfile { get; }` - whether profile is a CLB profile
- `bool IsHardcodedProfile { get; }` - whether profile is hardcoded
- `bool IsMultiCrossSectionUserParametric { get; }` - whether the profile is a parametric user-defined multi cross section profile
- `bool IsSketchedUserParametric { get; }` - whether the profile is a parametric user-defined sketched profile
- `int NumberOfCrossSections { get; }` - the number of cross sections in the profile item
- `string ParameterString { get; }` - the profile item parameter string
- `IList<ProfileItemCrossSection> ProfileItemCrossSections { get; }` - the list of profile item cross-sections
- `ProfileItemSubTypeEnum ProfileItemSubType { get; set; }` - the profile item subtype
- `ProfileItemTypeEnum ProfileItemType { get; }` - the profile item type

**Methods:**
- `bool AddCrossSection(int CrossSectionNumber, double Location, int Type, int SubType)` - add a cross section
- `bool DeleteCrossSection(int CrossSectionNumber)` - delete a cross section
- `bool Export(ref string filename)` - export profile to file
- `CrossSection GetCrossSection(double RelativeLocation)` - get cross section at relative location
- `CrossSection GetHighAccuracyCrossSection(double RelativeLocation)` - get high accuracy cross section
- `List<ProfileItemSubType> GetProfileItemSubTypes()` - get available subtypes
- `bool IsProfileUserDefined()` - whether profile is user-defined
- `bool IsProfileUserParametric()` - whether profile is user-parametric
- `bool ModifyCrossSection(int CrossSectionNumber, double Location, int Type, int SubType)` - modify a cross section
- `bool ModifyProfileItemAnalysisParameter(ProfileItemParameter value)` - modify analysis parameter
- `bool ModifyProfileItemParameter(ProfileItemParameter value)` - modify parameter
- `bool ModifyProfileItemUserParameter(ProfileItemParameter value)` - modify user parameter
- `bool Select()` - select/load the profile data
- `bool SelectCrossSections()` - select/load cross section data
- `bool SetEquivalentType(int CrossSectionNumber, int EquivalentType)` - set equivalent type
- `static List<ProfileItemSubType> GetProfileItemSubTypes(ProfileItem profileItem, ProfileItemTypeEnum typeEnum)` - get subtypes for a profile/type combo

### LibraryProfileItem : ProfileItem (sealed)
> The LibraryProfileItem class contains information from library profiles in the catalog. Library profile items can be enumerated using a profile item enumerator.

**Constructors:** `LibraryProfileItem()`

**Properties:**
- `ArrayList aProfileItemAnalysisParameters { get; }` - array list with the profile item analysis parameters
- `ArrayList aProfileItemUserParameters { get; }` - array list with the profile item user parameters
- `string ProfileName { get; set; }` - the profile item name

**Methods:**
- `bool Copy(string newName)` - copy profile with new name
- `bool Delete()` - delete the profile
- `bool Modify()` - modify the profile
- `bool Select(string profileName)` - select a library profile by name

### ParametricProfileItem : ProfileItem (sealed)
> The ParametricProfileItem class contains information from parametric profiles in the catalog. Parametric profile items can be enumerated using a profile item enumerator.

**Constructors:** `ParametricProfileItem()`

**Properties:**
- `string ProfilePrefix { get; set; }` - the parametric profile item prefix

**Methods:**
- `string CreateProfileString()` - create the profile string representation
- `bool Select(string ProfileName)` - select by profile name
- `bool SelectByPrefix(string ProfilePrefix)` - select by prefix
- `bool SelectByProfileName(string ProfileName)` - select by profile name
- `bool SelectByTypeAndSubtype(string profileName, ProfileItemTypeEnum type, ProfileItemSubTypeEnum subtype)` - select by type and subtype
- `static string GetParametricProfilePrefix(int Subtype)` - get prefix string for a subtype

### ProfileItemEnumerator (sealed)
> The ProfileItemEnumerator class allows to loop through the catalog profile items.

**Properties:**
- `ProfileItem Current { get; }` - returns a profile item instance of the current element
- `bool SelectInstances { get; set; }` - indicates that Select() is called when Current is accessed. Set to false if no members are needed (e.g., only reading profile names). Default: true.

**Methods:**
- `int GetSize()` - total number of items
- `bool MoveNext()` - advance to next item
- `void Reset()` - reset to beginning

### ProfileItemCrossSection (sealed)
> Profile item cross section data.

**Constructors:** `ProfileItemCrossSection()`

**Properties:**
- `double CrossSectionLocation { get; set; }` - cross section location
- `int CrossSectionNumber { get; set; }` - cross section number
- `int EquivalentType { get; set; }` - equivalent type
- `double Location { get; set; }` - location
- `int Number { get; set; }` - number
- `int NumberOfProfileItemAnalysisParameters { get; set; }` - count of analysis parameters
- `int NumberOfProfileItemParameters { get; set; }` - count of parameters
- `int NumberOfProfileItemUserParameters { get; set; }` - count of user parameters
- `IList<ProfileItemParameter> ProfileItemAnalysisParameters { get; set; }` - analysis parameters
- `IList<ProfileItemParameter> ProfileItemParameters { get; set; }` - parameters
- `IList<ProfileItemParameter> ProfileItemUserParameters { get; set; }` - user parameters
- `int ProfileSubType { get; set; }` - profile subtype
- `int ProfileType { get; set; }` - profile type

**Methods:**
- `void AddProfileItemAnalysisParamter(string property, string alblString, string symbol, object value, ParameterUnitTypeEnum ParameterUnitType, int crossSectionNumber)`
- `void AddProfileItemParamter(string property, string alblString, string symbol, object value, ParameterUnitTypeEnum ParameterUnitType, int crossSectionNumber)`
- `void AddProfileItemUserParamter(string property, string alblString, string symbol, object value, ParameterUnitTypeEnum ParameterUnitType, int crossSectionNumber)`

### ProfileItemParameter (sealed)
> The ProfileItemParameter class contains the information of one profile parameter (property name, symbol, unit and unit type). A profile item can contain a maximum of 50 profile parameters.

**Constructors:** `ProfileItemParameter()`

**Properties:**
- `string AlblString { get; }` - gets a translated albl string
- `int CrossSectionNumber { get; }` - the number of the cross section the parameter belongs to
- `int IntegerValue { get; set; }` - the integer value of the profile item parameter
- `ParameterUnitTypeEnum ParameterUnitType { get; }` - defines the parameter unit type
- `string Property { get; }` - the description of the profile item parameter (corresponds to 'Property' in the profile catalog dialog)
- `string StringValue { get; set; }` - the string value of the profile item parameter
- `string Symbol { get; }` - the symbol of the profile item parameter
- `double Value { get; set; }` - the value of the profile item parameter

### ProfileItemSubType (sealed)
> Profile item subtype definition.

**Constructors:** `ProfileItemSubType()`

**Properties:**
- `string BitmapName { get; set; }` - bitmap name for the subtype
- `string Label { get; set; }` - label for the subtype
- `string ParameterString { get; set; }` - parameter string
- `ProfileItemSubTypeEnum SubTypeId { get; set; }` - subtype identifier

### ProfileName (sealed)
> The ProfileName class contains the name of the profile item.

**Constructors:** `ProfileName()`

**Properties:**
- `string Name { get; set; }` - the profile item name

### CrossSection (sealed)
> The CrossSection class defines a cross section with points.

**Constructors:** `CrossSection(ProfileName Profile)`, `CrossSection(string ProfileString)`

**Properties:**
- `bool HighAccuracy { get; set; }` - the contour geometry accuracy
- `List<List<Point>> InnerSurfacePoints { get; }` - points for inner surfaces
- `List<List<CrossSectionPoint>> InnerSurfaces { get; }` - cross section points for inner surfaces
- `double Length { get; set; }` - the total length
- `double Location { get; set; }` - the location in relation to length (0.0 = start)
- `List<CrossSectionPoint> OuterSurface { get; }` - cross section points for outer surface
- `List<Point> OuterSurfacePoints { get; }` - points for outer surface
- `ProfileName Profile { get; set; }` - the profile to query

**Methods:**
- `List<Polycurve> GetInnerContours()` - get inner contour polycurves
- `Polycurve GetOuterContour()` - get outer contour polycurve
- `bool Select(double Location, double Length)` - select cross section at location
- `bool Select(bool HighAccuracy, double Location, double Length)` - select with accuracy control

### CrossSectionPoint : Point (sealed)
> The CrossSectionPoint class defines a point with possible chamfering information.

**Constructors:** `CrossSectionPoint()`

**Properties:**
- `Chamfer Chamfer { get; set; }` - the chamfer for the cross section point

### ProfileItem.ProfileItemTypeEnum
> Defines the different profile item types.

| Value | Int | Description |
|-------|-----|-------------|
| PROFILE_UNKNOWN | 0 | The unknown profile |
| PROFILE_I | 1 | The I profile |
| PROFILE_L | 2 | The L profile |
| PROFILE_FPL | 2 | The FPL profile (same as L) |
| PROFILE_Z | 3 | The Z profile |
| PROFILE_U | 4 | The U profile |
| PROFILE_PL | 5 | The plate profile |
| PROFILE_D | 6 | The circular section profile |
| PROFILE_PD | 7 | The circular hollow section profile |
| PROFILE_P | 8 | The rectangular hollow section profile |
| PROFILE_C | 9 | The C profile |
| PROFILE_T | 10 | The T profile |
| PROFILE_HK | 11 | The welded box profile |
| PROFILE_HQ | 13 | The HQ profile |
| PROFILE_ZZ | 15 | The ZZ profile |
| PROFILE_CC | 16 | The CC profile |
| PROFILE_CW | 17 | The CW profile |
| PROFILE_CU | 18 | The CU profile |
| PROFILE_EB | 19 | The EB profile |
| PROFILE_BF | 20 | The BF profile |
| PROFILE_SPD | 21 | The SPD profile |
| PROFILE_EC | 22 | The EC profile |
| PROFILE_ED | 23 | The ED profile |
| PROFILE_EE | 24 | The EE profile |
| PROFILE_EF | 25 | The EF profile |
| PROFILE_EZ | 26 | The EZ profile |
| PROFILE_EW | 27 | The EW profile |
| PROFILE_POLYGON_PLATE | 51 | The polygon plate profile |
| PROFILE_SP | 101 | The SP profile |
| PROFILE_RCDL | 102 | The RCDL profile |
| PROFILE_RCXX | 103 | The RCXX profile |
| PROFILE_RCL | 104 | The RCL profile |
| PROFILE_RCDX | 105 | The RCDX profile |
| PROFILE_RCX | 106 | The RCX profile |
| PROFILE_USER_DEFINED | 998 | The user-defined, fixed profile |
| PROFILE_USER_PARAMETRIC | 999 | The user-defined, parametric profile |
| ALL_PROFILES | -1 | All profiles |

### ProfileItem.ProfileItemSubTypeEnum
> Defines the different profile item subtypes.

| Value | Int | Description |
|-------|-----|-------------|
| PROFILE_UNKNOWN_SUBTYPE | 0 | The unknown subtype profile |
| PROFILE_I_HOT_ROLLED | 1001 | The hot rolled I profile |
| PROFILE_I_WELDED_SYMMETRICAL | 1002 | The welded symmetrical I profile |
| PROFILE_I_WELDED_UNSYMMETRICAL | 1003 | The welded unsymmetrical I profile |
| PROFILE_I_WELDED_SYMMETRICAL2 | 1004 | The welded symmetrical, altering height, I profile |
| PROFILE_I_WELDED_UNSYMMETRICAL2 | 1005 | The welded unsymmetrical, altering height, I profile |
| PROFILE_L_HOT_ROLLED | 2001 | The hot rolled L profile |
| PROFILE_L_COLD_ROLLED | 2002 | The cold rolled L profile |
| PROFILE_Z_COLD_ROLLED | 3001 | The cold rolled Z profile |
| PROFILE_U_HOT_ROLLED | 4001 | The hot rolled U profile |
| PROFILE_U_COLD_ROLLED | 4002 | The cold rolled U profile |
| PROFILE_PL_DEFAULT | 5001 | The default plate profile |
| PROFILE_D_CIRCULAR | 6001 | The default circular section profile |
| PROFILE_D_ELLIPTICAL | 6002 | The elliptical circular section profile |
| PROFILE_PD_CIRCULAR | 7001 | The default circular hollow section profile |
| PROFILE_PD_ELLIPTICAL | 7002 | The elliptical circular hollow section profile |
| PROFILE_PD_CIRCULAR_TAPERED | 7003 | The tapered circular hollow section profile |
| PROFILE_P_SQUARE | 8001 | The square hollow section profile |
| PROFILE_P_RECTANGULAR | 8002 | The rectangular hollow section profile |
| PROFILE_P_ALTERING_HEIGHT | 8003 | The altering height hollow section profile |
| PROFILE_C_HOT_ROLLED | 9001 | The hot rolled C profile |
| PROFILE_C_COLD_ROLLED | 9002 | The cold rolled C profile |
| PROFILE_T_HOT_ROLLED | 10001 | The hot rolled T profile |
| PROFILE_T_PARAMETRIC | 10002 | The parametric T profile |
| PROFILE_HK_SYMMETRICAL | 11001 | The symmetrical welded box profile |
| PROFILE_HK_UNSYMMETRICAL | 11002 | The unsymmetrical welded box profile |
| PROFILE_HQ_CENTERED | 13001 | The centered HQ profile |
| PROFILE_HQ_NOT_CENTERED | 13002 | The not centered HQ profile |
| PROFILE_ZZ_SYMMETRICAL | 15001 | The symmetrical ZZ profile |
| PROFILE_ZZ_NOT_SYMMETRICAL | 15002 | The unsymmetrical ZZ profile |
| PROFILE_CC_SYMMETRICAL | 16001 | The symmetrical CC profile |
| PROFILE_CC_NOT_SYMMETRICAL | 16002 | The unsymmetrical CC profile |
| PROFILE_CW_SYMMETRICAL | 17001 | The symmetrical CW profile |
| PROFILE_CW_UNSYMMETRICAL | 17002 | The unsymmetrical CW profile |
| PROFILE_CU_SYMMETRICAL | 18001 | The symmetrical CU profile |
| PROFILE_CU_NOT_SYMMETRICAL | 18002 | The unsymmetrical CU profile |
| PROFILE_EB_SYMMETRICAL | 19001 | The symmetrical EB profile |
| PROFILE_EB_NOT_SYMMETRICAL | 19002 | The unsymmetrical EB profile |
| PROFILE_BF_DEFAULT | 20001 | The default BF profile |
| PROFILE_SPD_CIRCULAR | 21001 | The circular SPD profile |
| PROFILE_SPD_ELLIPTICAL | 21002 | The elliptical SPD profile |
| PROFILE_SPD_CIRCULAR_TAPERED | 21003 | The tapered circular SPD profile |
| PROFILE_EC_SYMMETRICAL | 22001 | The symmetrical EC profile |
| PROFILE_EC_NOT_SYMMETRICAL | 22002 | The unsymmetrical EC profile |
| PROFILE_ED_DEFAULT | 23001 | The default ED profile |
| PROFILE_EE_DEFAULT | 24001 | The default EE profile |
| PROFILE_EF_DEFAULT | 25001 | The default EF profile |
| PROFILE_EZ_DEFAULT | 26001 | The default EZ profile |
| PROFILE_EW_DEFAULT | 27001 | The default EW profile |
| PROFILE_RCDL_SYMMETRICAL | 102001 | The symmetrical RCDL profile |
| PROFILE_RCDL_UNSYMMETRICAL | 102002 | The unsymmetrical RCDL profile |
| PROFILE_RCXX_DEFAULT | 103001 | The RCXX default profile |
| PROFILE_RCL_DEFAULT | 104001 | The RCL default profile |
| PROFILE_RCDX_SYMMETRICAL | 105001 | The symmetrical RCDX profile |
| PROFILE_RCDX_UNSYMMETRICAL | 105002 | The unsymmetrical RCDX profile |
| PROFILE_RCDX_UNSYMMETRICAL2 | 105003 | The unsymmetrical altered height RCDX profile |
| PROFILE_RCX_DEFAULT | 106001 | The RCX default profile |

### ProfileItem.ProfileItemParametersTypeEnum

| Value | Int | Description |
|-------|-----|-------------|
| GENERAL_PARAMETERS | 0 | General parameters |
| ANALYSIS_PARAMETERS | 1 | Analysis parameters |
| USER_PARAMETERS | 2 | User parameters |

### ProfileItemParameter.ParameterUnitTypeEnum
> Defines the different unit types of the profile item parameter.

| Value | Int | Description |
|-------|-----|-------------|
| INPUT_NONE | 0 | No unit |
| INPUT_RATIO_UNIT | 1 | Ratio unit |
| INPUT_STRAIN_UNIT | 2 | Strain unit |
| INPUT_ANGLE_UNIT | 5 | Angle unit (input) |
| OUTPUT_ANGLE_UNIT | 6 | Angle unit (output) |
| INPUT_SECTION_ANGLE_UNIT | 7 | Section angle unit |
| INPUT_LENGTH_UNIT | 10 | Length unit (input) |
| OUTPUT_LENGTH_UNIT | 11 | Length unit (output) |
| INPUT_DEFORMATION_UNIT | 12 | Deformation unit (input) |
| OUTPUT_DEFORMATION_UNIT | 13 | Deformation unit (output) |
| INPUT_DIMENSION_UNIT | 14 | Dimension unit |
| INPUT_RADIUSOFINERTIA_UNIT | 16 | Radius of inertia unit |
| INPUT_AREA_UNIT | 20 | Area unit (input) |
| OUTPUT_REINFAREA_UNIT | 21 | Reinforced area unit (output) |
| OUTPUT_TRANSVREINF_UNIT | 22 | Transverse reinforcement unit (output) |
| INPUT_AREAPERLENGTH_UNIT | 23 | Area per length unit |
| OUTPUT_VOLUME_UNIT | 30 | Volume unit (output) |
| INPUT_SECTIONMODULUS_UNIT | 31 | Section modulus unit |
| INPUT_VOLUME_UNIT | 32 | Volume unit (input) |
| INPUT_MOMENTOFINERTIA_UNIT | 40 | Moment of inertia unit |
| INPUT_TORSIONCONSTANT_UNIT | 41 | Torsion constant unit |
| INPUT_WARPINGCONSTANT_UNIT | 60 | Warping constant unit |
| INPUT_FORCE_UNIT | 100 | Force unit (input) |
| OUTPUT_FORCE_UNIT | 101 | Force unit (output) |
| INPUT_WEIGHT_UNIT | 102 | Weight unit (input) |
| OUTPUT_WEIGHT_UNIT | 103 | Weight unit (output) |
| INPUT_DISTRIBLOAD_UNIT | 110 | Distributed load unit (input) |
| OUTPUT_DISTRIBLOAD_UNIT | 111 | Distributed load unit (output) |
| INPUT_SPRINGCONSTANT_UNIT | 112 | Spring constant unit |
| OUTPUT_MASSPERLENGTH_UNIT | 113 | Mass per length unit (output) |
| INPUT_SURFACELOAD_UNIT | 120 | Surface load unit (input) |
| OUTPUT_SURFACELOAD_UNIT | 121 | Surface load unit (output) |
| INPUT_STRENGTH_UNIT | 122 | Strength unit |
| OUTPUT_STRESS_UNIT | 123 | Stress unit (output) |
| INPUT_MODULUS_UNIT | 124 | Modulus unit |
| INPUT_DENSITY_UNIT | 131 | Density unit |
| INPUT_MOMENT_UNIT | 200 | Moment unit (input) |
| OUTPUT_MOMENT_UNIT | 201 | Moment unit (output) |
| INPUT_DISTRIBMOMENT_UNIT | 205 | Distributed moment unit |
| INPUT_ROTSPRINGCONST_UNIT | 210 | Rotation spring constant unit |
| INPUT_TEMPERATURE_UNIT | 300 | Temperature unit (input) |
| OUTPUT_TEMPERATURE_UNIT | 301 | Temperature unit (output) |
| INPUT_THERMDILATCOEFF_UNIT | 310 | Thermal dilatation coefficient unit |
| INPUT_FACTOR_UNIT | 400 | Factor unit |
| INPUT_DATE_UNIT | 401 | Date unit |
| INPUT_DATE_TIME_MIN_UNIT | 402 | Date time minutes unit |
| INPUT_DATE_TIME_SEC_UNIT | 403 | Date time seconds unit |
| INPUT_LENGTH_FRACTIONAL_IMPERIAL | 1005 | Length, fractional imperial (input) |
| OUTPUT_LENGTH_FRACTIONAL_IMPERIAL | 1105 | Length, fractional imperial (output) |
| INPUT_DEFORMATION_FRACTIONAL_IMPERIAL | 1305 | Deformation, fractional imperial (input) |
| OUTPUT_DEFORMATION_FRACTIONAL_IMPERIAL | 1305 | Deformation, fractional imperial (output) |
| INPUT_DIMENSION_FRACTIONAL_IMPERIAL | 1405 | Dimension, fractional imperial |
| INPUT_RADIUSOFINERTIA_FRACTIONAL_IMPERIAL | 1605 | Radius of inertia, fractional imperial |
| INPUT_BOOLEAN | -3 | Boolean input |
| INPUT_INTEGER | -2 | Integer input |
| INPUT_STRING | -1 | String input |

---

## Material Catalog

### MaterialItem
> The MaterialItem class contains information about the materials in the Tekla Structures catalog.

**Constructors:** `MaterialItem()`

**Properties:**
- `string AliasName1 { get; set; }` - alias name 1
- `string AliasName2 { get; set; }` - alias name 2
- `string AliasName3 { get; set; }` - alias name 3
- `int DesignCode { get; set; }` - the design code
- `string MaterialName { get; set; }` - the material item's name
- `double ModulusOfElasticity { get; set; }` - modulus of elasticity in N/m2
- `double PlateDensity { get; set; }` - plate density in kg/m3
- `double PoissonsRatio { get; set; }` - Poisson's ratio
- `double ProfileDensity { get; set; }` - profile density in kg/m3
- `double ThermalDilatation { get; set; }` - thermal dilatation in 1/K
- `MaterialItemTypeEnum Type { get; set; }` - the material item's type

**Methods:**
- `bool Delete()` - delete material from catalog
- `bool Export(ref string filename)` - export material to file
- `bool Insert()` - insert new material into catalog
- `bool Modify()` - modify existing material
- `bool Select(string materialName)` - select material by name
- `bool Select()` - select using current property values
- `static int MaterialNameMaxLength()` - get maximum material name length

### MaterialItem.MaterialItemTypeEnum
> Defines the different material item types.

| Value | Int | Description |
|-------|-----|-------------|
| MATERIAL_UNKNOWN | 0 | The unknown material item type |
| MATERIAL_STEEL | 1 | The steel material type |
| MATERIAL_CONCRETE | 2 | The concrete material type |
| MATERIAL_TIMBER | 3 | The timber material type |
| MATERIAL_MISC | 4 | The miscellaneous material type |
| MATERIAL_REBAR | 5 | The rebar material type |
| MATERIAL_REBAR_MESH | 6 | The rebar mesh material type |

### MaterialItemEnumerator (sealed)
> The MaterialItemEnumerator class allows to loop through the catalog material items.

**Properties:**
- `MaterialItem Current { get; }` - returns a material item instance of the current element

**Methods:**
- `int GetSize()` - total number of items
- `bool MoveNext()` - advance to next item
- `void Reset()` - reset to beginning

### MaterialMarketSizesItem (sealed)
> The MaterialMarketSizesItem class contains information about the available market sizes for particular material.

**Properties:**
- `double[] MarketSizes { get; set; }` - available market sizes for the material
- `string MaterialName { get; set; }` - material name

### MaterialMarketSizesItemEnumerator (sealed)
> The MaterialMarketSizesItemEnumerator class allows to loop through the items defined in XS_PROFDB\marketsizes.dat file.

**Properties:**
- `MaterialMarketSizesItem Current { get; }` - returns a MaterialMarketSizesItem instance with the current index

**Methods:**
- `bool MoveNext()` - advance to next item
- `void Reset()` - reset to beginning

### MaterialName (sealed)
> The MaterialName class contains the name of the material item.

**Constructors:** `MaterialName()`

**Properties:**
- `string Name { get; set; }` - the material item name

### FinishItem (sealed)
> The FinishItem struct contains information about the finish parameters.

**Constructors:** `FinishItem(string abbreviation, string description)`

**Properties:**
- `string Abbreviation { get; set; }` - the finish abbreviation
- `string Description { get; set; }` - the finish description

---

## Rebar Catalog

### RebarItem (sealed)
> The RebarItem class contains information from the rebars in the catalog (rebar_database.inp).

**Constructors:** `RebarItem()`

**Properties:**
- `double ActualDiameter { get; set; }` - actual diameter
- `double BendRadius { get; set; }` - bending radius
- `string Code { get; set; }` - rebar code
- `double CrankedLength { get; set; }` - cranked length
- `string CrankedLengthType { get; set; }` - cranked length type
- `double CrankExtraOffset { get; set; }` - crank extra offset
- `double CrankStraightLength { get; set; }` - crank straight length
- `double CrossSectionArea { get; set; }` - cross section area
- `double CurveTolerance { get; set; }` - curve tolerance
- `double ExtraPointShortening { get; set; }` - extra point shortening
- `string Grade { get; set; }` - grade
- `double HookLength135Degrees { get; set; }` - hook length for 135 degrees
- `double HookLength180Degrees { get; set; }` - hook length for 180 degrees
- `double HookLength90Degrees { get; set; }` - hook length for 90 degrees
- `double HookRadius135Degrees { get; set; }` - hook radius for 135 degrees
- `double HookRadius180Degrees { get; set; }` - hook radius for 180 degrees
- `double HookRadius90Degrees { get; set; }` - hook radius for 90 degrees
- `double LapLength { get; set; }` - lap length
- `double MaxRadiusRequiringBending { get; set; }` - max radius requiring bending
- `double NominalDiameter { get; set; }` - nominal diameter
- `string Size { get; set; }` - size
- `string Usage { get; set; }` - usage
- `double WeightPerLenght { get; set; }` - weight per length (note: Tekla API typo "Lenght")

**Methods:**
- `bool Delete()` - delete rebar item
- `bool Export(ref string filename)` - export to file
- `bool Insert()` - insert new rebar item
- `bool Modify(string OriginalGrade, string OriginalSize, string OriginalUsage)` - modify using original identity
- `bool Select(string Grade, string Size)` - select by grade and size
- `bool Select(string Grade, string Size, double BendRadius)` - select by grade, size, bend radius
- `bool Select(string Grade, string Size, string Usage)` - select by grade, size, usage
- `bool Select(string Grade, double NominalDiameter, bool UseNominalDiameter)` - select by grade and nominal diameter
- `bool Select(string Grade, double Diameter, double BendRadius, bool UseNominalDiameter)` - select by grade, diameter, bend radius
- `bool Select(string Grade, double Diameter, string Usage, bool UseNominalDiameter)` - select by grade, diameter, usage

### RebarItemEnumerator (sealed)
> The RebarItemEnumerator class allows to loop through the catalog rebar items.

**Properties:**
- `RebarItem Current { get; }` - returns a rebar item instance of the current element

**Methods:**
- `int GetSize()` - total number of items
- `bool MoveNext()` - advance to next item
- `void Reset()` - reset to beginning

**Static Methods:**
- `static bool DeleteRebarItem(RebarItem rebarItem)` - delete a rebar item
- `static List<RebarHeaderItem> GetRebarHeaderItems()` - get rebar header items
- `static bool InsertRebarItem(RebarItem rebarItem)` - insert a rebar item
- `static bool ModifyRebarItem(RebarItem rebarItem, RebarItem originalRebarItem)` - modify a rebar item
- `static bool RebarItemExists(RebarItem rebarItem)` - check if rebar item exists
- `static RebarItem SelectRebarItem(string Grade, string Size)` - select by grade and size
- `static RebarItem SelectRebarItem(string Grade, string Size, double BendRadius)` - select by grade, size, bend radius
- `static RebarItem SelectRebarItem(string Grade, string Size, string Usage)` - select by grade, size, usage
- `static RebarItem SelectRebarItem(string Grade, string Size, string Usage, string Code)` - select by grade, size, usage, code
- `static RebarItem SelectRebarItem(string Grade, double Diameter, bool UseNominalDiameter)` - select by grade and diameter
- `static RebarItem SelectRebarItem(string Grade, double Diameter, string Usage, bool UseNominalDiameter)` - select by grade, diameter, usage
- `static RebarItem SelectRebarItem(string Grade, double Diameter, double BendRadius, bool UseNominalDiameter)` - select by grade, diameter, bend radius

### RebarHeaderItem
> The RebarHeaderItem class represents an item in the header of the rebar catalog (rebar_database.inp).

**Constructors:** `RebarHeaderItem()`

**Properties:**
- `string Name { get; set; }` - the name
- `string PropertyName { get; set; }` - the property name
- `string Type { get; set; }` - the type
- `string Units { get; set; }` - the units

---

## Mesh Catalog

### MeshItem (sealed)
> The MeshItem class contains information from the meshes in the catalog (mesh_database.inp).

**Constructors:** `MeshItem()`

**Properties:**
- `string DiameterCross { get; set; }` - cross direction bar size
- `string DiameterLongitudinal { get; set; }` - longitudinal direction bar size
- `string DistanceCross { get; set; }` - cross direction distance
- `string DistanceLongitudinal { get; set; }` - longitudinal direction distance
- `string Grade { get; set; }` - grade
- `double LeftOverhangCross { get; set; }` - cross direction left overhang
- `double LeftOverhangLongitudinal { get; set; }` - longitudinal direction left overhang
- `double Length { get; set; }` - length
- `double MaximumOverlappingCross { get; set; }` - cross direction maximum overlapping
- `double MaximumOverlappingLongitudinal { get; set; }` - longitudinal direction maximum overlapping
- `double MinimumOverlappingCross { get; set; }` - cross direction minimum overlapping
- `double MinimumOverlappingLongitudinal { get; set; }` - longitudinal direction minimum overlapping
- `string Name { get; set; }` - name
- `double RightOverhangCross { get; set; }` - cross direction right overhang
- `double RightOverhangLongitudinal { get; set; }` - longitudinal direction right overhang
- `double Width { get; set; }` - width

**Methods:**
- `bool Export(ref string filename)` - export to file
- `bool Select(string MeshName, string MeshGrade)` - select by name and grade

### MeshItemEnumerator (sealed)
> The MeshItemEnumerator class allows to loop through the catalog mesh items.

**Properties:**
- `MeshItem Current { get; }` - returns a mesh item instance of the current element

**Methods:**
- `int GetSize()` - total number of items
- `bool MoveNext()` - advance to next item
- `void Reset()` - reset to beginning
- `static MeshItem SelectMeshItem(string Name, string Grade)` - select a specific mesh item

---

## Shape Catalog

### ShapeItem
> The Shape class contains information about the shapes in the Tekla Structures shape catalog.

**Constructors:** `ShapeItem()`

**Properties:**
- `BrepType BrepType { get; set; }` - defines BrepType (BuildingProduct or StructuralShape)
- `AABB Extrema { get; set; }` - axis-aligned bounding box extrema
- `string Fingerprint { get; set; }` - fingerprint value for quick geometry comparison
- `string GeometryGuid { get; set; }` - unique identifier of the shape geometry (filename in ShapeGeometries directory)
- `string GeometryHash { get; set; }` - (obsolete) externally provided hash value; use Fingerprint instead
- `string Guid { get; set; }` - unique identifier of the shape (filename in Shape directory)
- `List<Point> HandlePoints { get; set; }` - handle points of the shape
- `bool IsSolid { get; set; }` - true if detected by TS Core to be a valid solid
- `string Name { get; set; }` - the shape name
- `FacetedBrep ShapeFacetedBrep { get; set; }` - geometric information as a FacetedBRep
- `ShapeUpAxis UpAxis { get; set; }` - the "up" direction (Undefined, X_Axis, Y_Axis, Z_Axis). Native Tekla orientation: x-axis on extrusion line, Y-axis pointing up.

**Methods:**
- `bool CleanAndModify(ref bool isSolid)` - clean and modify the shape
- `bool Delete()` - delete shape from catalog
- `bool DeleteMetadata(string key)` - delete metadata by key
- `bool Export(ref string filename)` - export shape to file
- `List<string> GetAllMetadataKeys()` - get all metadata keys
- `int GetInstanceCount()` - get number of instances using this shape
- `bool GetMetadata(string key, ref ShapeMetadataTypeEnum type, ref object value, ref string label, ref string description)` - get metadata by key
- `bool Insert()` - insert shape into catalog
- `bool InsertOrGetGuidsOfShapesWithMatchingGeometry(out List<string> existingShapeGuids)` - insert or find matching shapes
- `bool InsertUsingNormals()` - insert using normals
- `bool InsertUsingNormalsAllowDuplicates()` - insert using normals, allowing duplicates
- `bool Modify()` - modify existing shape
- `bool Rename(string newName)` - rename the shape
- `bool Select()` - select using current Guid
- `bool Select(string name)` - select shape by name
- `bool SetHandlePoints(List<Point> HandlePoints)` - set handle points
- `bool SetMetadata(string key, ShapeMetadataTypeEnum type, object value, string label, string description)` - set metadata

### ShapeItemEnumerator (sealed)
> The ShapeItemEnumerator class allows to loop through the shape catalog items.

**Properties:**
- `ShapeItem Current { get; }` - returns a shape item instance of the current element

**Methods:**
- `int GetSize()` - total number of items
- `bool MoveNext()` - advance to next item
- `void Reset()` - reset to beginning

### BrepType (enum)
> Distinguishes shape types in the shape catalog.

| Value | Int | Description |
|-------|-----|-------------|
| BuildingProduct | 0 | Default. Shapes imported from e.g. Tekla Warehouse, provided by manufacturers |
| StructuralShape | 1 | Shapes created by "Convert part to item" or geometry editing |

### ShapeUpAxis (enum)
> Up axis direction for shapes.

| Value | Int | Description |
|-------|-----|-------------|
| Undefined | 0 | Undefined direction. Should not be used as input. |
| X_Axis | 1 | X points up |
| Y_Axis | 2 | Y points up |
| Z_Axis | 3 | Z points up |

### ShapeMetadataTypeEnum (enum)
> Represents data type or measurement unit of specific metadata.

| Value | Int | Description |
|-------|-----|-------------|
| Text | 0 | Default text value |
| DateTime | 10 | Date |
| Integer | 11 | Integer |
| Double | 12 | Double (precision controlled by XS_UNITDECIMALS_INPUT_FACTOR) |
| Length | 101 | Unit of length (XS_UNIT_INPUT_DIMENSION) |
| Area | 201 | Unit of area (XS_UNIT_INPUT_AREA) |
| Volume | 301 | Unit of volume (XS_UNIT_INPUT_VOLUME) |
| Weight | 401 | Unit of weight (XS_UNIT_INPUT_WEIGHT) |

---

## Component Catalog

### ComponentItem
> The ComponentItem class contains information about the components in the Tekla Structures catalog.

**Constructors:** `ComponentItem()`

**Properties:**
- `string AttributeFileExtension { get; }` - attribute file extension from the component database
- `string Name { get; set; }` - internal name used by Tekla Structures for component identification
- `int Number { get; set; }` - internal number used by Tekla Structures for component identification
- `ComponentTypeEnum Type { get; set; }` - the component item's type
- `string UIName { get; set; }` - name visible in the Tekla Structures user interface

**Methods:**
- `bool Export(ref string filename)` - export component to file
- `bool GetVersion(ref int version)` - get component version
- `bool Select(string name, int number)` - select by name and number
- `bool Select(string name, int number, ComponentTypeEnum type)` - select by name, number, and type

### ComponentItem.ComponentTypeEnum
> Defines the different component types.

| Value | Int | Description |
|-------|-----|-------------|
| UNKNOWN | 0 | The unknown component type |
| CONNECTION | 24 | Connection |
| COMPONENT | 25 | Component |
| SEAM | 26 | Seam |
| DETAIL | 27 | Detail |
| DRAWING_PLUGIN | 38 | Drawing plugin |
| CUSTOM_PART | 43 | Custom part object |

### ComponentItemEnumerator (sealed)
> The ComponentItemEnumerator class allows to loop through the component catalog items.

**Properties:**
- `ComponentItem Current { get; }` - returns a component item instance of the current element

**Methods:**
- `int GetSize()` - total number of items
- `bool MoveNext()` - advance to next item
- `void Reset()` - reset to beginning

---

## Drawing Catalog

### DrawingItem (sealed)
> The DrawingItem class contains information from the drawings in the master drawing catalog.

**Constructors:** `DrawingItem()`

**Properties:**
- `List<FileInfo> Files { get; set; }` - files belonging to drawing item
- `string Name { get; set; }` - drawing item's name
- `DrawingTypes Type { get; set; }` - drawing item's type

**Methods:**
- `bool Export(ref string filename)` - export drawing item to file
- `bool Select(string Name)` - select by name

### DrawingItemEnumerator (sealed)
> The DrawingItemEnumerator class allows to loop through the catalog drawing items.

**Properties:**
- `DrawingItem Current { get; }` - returns a drawing item instance of the current element

**Methods:**
- `int GetSize()` - total number of items
- `bool MoveNext()` - advance to next item
- `void Reset()` - reset to beginning

---

## Printer Catalog

### PrinterItem
> The PrinterItem class contains information about the printers in the Tekla Structures catalog.

**Constructors:** `PrinterItem()`

**Properties:**
- `string Device { get; set; }` - the actual printer device used in the printing
- `string Extension { get; set; }` - default file extension when printing to a file
- `string Name { get; set; }` - name used by Tekla Structures to fetch default properties
- `double PrintAreaHeigth { get; set; }` - print area height (note: Tekla API typo "Heigth")
- `double PrintAreaWidth { get; set; }` - print area width

### PrinterItemEnumerator (sealed)
> The PrinterItemEnumerator class allows to loop through the catalog printer items.

**Properties:**
- `PrinterItem Current { get; }` - returns a printer item instance of the current element

**Methods:**
- `int GetSize()` - total number of items
- `bool MoveNext()` - advance to next item
- `void Reset()` - reset to beginning

---

## User Properties

### UserPropertyItem
> The UserPropertyItem class contains information about the user properties in the Tekla Structures catalog.

**Constructors:** `UserPropertyItem()`

**Properties:**
- `bool AffectsNumbering { get; set; }` - whether the property value affects object numbering
- `UserPropertyFieldTypeEnum FieldType { get; set; }` - field type of the user property
- `UserPropertyLevelEnum Level { get; set; }` - level at which the user property has been defined
- `string Name { get; set; }` - name of the user property
- `PropertyTypeEnum Type { get; set; }` - type of the user property
- `bool Unique { get; set; }` - whether the property value is copied when the object is copied
- `UserPropertyVisibilityEnum Visibility { get; set; }` - whether the property is visible/editable

**Methods:**
- `bool AddToObjectType(CatalogObjectTypeEnum objectType)` - add user property to an object type
- `bool Delete()` - delete user property
- `bool GetDefaultValue(ref string DefaultValue)` - get string default
- `bool GetDefaultValue(ref int DefaultValue)` - get int default
- `bool GetDefaultValue(ref double DefaultValue)` - get double default
- `string GetLabel()` - get the label
- `string GetLabel(bool translated)` - get the label, optionally translated
- `bool GetObjectTypes(ref List<CatalogObjectTypeEnum> objectTypes)` - get object types this property applies to
- `bool GetOptions(ref List<KeyValuePair<string, string>> StringOptions)` - get string options
- `bool GetOptions(ref List<KeyValuePair<string, string>> StringOptions, bool translatedLabels)` - get string options with translation
- `bool GetOptions(ref List<KeyValuePair<double, string>> DoubleOptions)` - get double options
- `bool GetOptions(ref List<KeyValuePair<double, string>> DoubleOptions, bool translatedLabels)` - get double options with translation
- `bool GetOptions(ref List<KeyValuePair<int, string>> IntOptions)` - get int options
- `bool GetOptions(ref List<KeyValuePair<int, string>> IntOptions, bool translatedLabels)` - get int options with translation
- `bool Insert()` - insert new user property
- `bool Modify()` - modify existing user property
- `bool RemoveFromObjectType(CatalogObjectTypeEnum objectType)` - remove user property from an object type
- `bool Select()` - select/load user property data
- `bool SetDefaultValue(string DefaultValue)` - set string default
- `bool SetDefaultValue(double DefaultValue)` - set double default
- `bool SetDefaultValue(int DefaultValue)` - set int default
- `void SetLabel(string label)` - set the label
- `bool SetOptions(List<KeyValuePair<string, string>> StringOptions)` - set string options
- `bool SetOptions(List<KeyValuePair<double, string>> DoubleOptions)` - set double options
- `bool SetOptions(List<KeyValuePair<int, string>> IntOptions)` - set int options

### UserPropertyItemEnumerator (sealed)
> The UserPropertyItemEnumerator class allows to loop through the user property items.

**Properties:**
- `UserPropertyItem Current { get; }` - returns a user property item instance of the current element

**Methods:**
- `int GetSize()` - total number of items
- `bool MoveNext()` - advance to next item
- `void Reset()` - reset to beginning

### UserPropertyName (sealed)
> The UserPropertyName class contains the name of the user property item.

**Constructors:** `UserPropertyName()`

**Properties:**
- `string Name { get; set; }` - the user property item name

### UserPropertyOption (sealed)
> The UserPropertyOption class contains the properties of a user property value option.

**Constructors:** `UserPropertyOption()`

**Properties:**
- `double DoubleValue { get; set; }` - the double value
- `int IntValue { get; set; }` - the integer value
- `string OptionLabel { get; set; }` - the option name
- `string OptionLabelTranslated { get; set; }` - the translated option name
- `string StringValue { get; set; }` - the string value

---

## Utility Classes

### AttributeConfiguration (static)
> Attribute configuration utilities.

### FabricatorProfilesAndMaterials (static)
> The FabricatorProfilesAndMaterials class contains methods for converting fabricator profiles and materials to Tekla Structures.

**Static Methods:**
- `static bool ConvertSourceFile(string profilesFilePath, string dimensionsFilePath)` - convert fabricator source files

### CatalogItemEnumeratorInitializationException : Exception (sealed)
> The CatalogItemEnumeratorInitializationException class represents an error that occurred during the catalog item enumerator initialization. This class cannot be inherited.

---

## Shared Enumerations

### AttributeConfiguration.DrawingTypes

| Value | Int | Description |
|-------|-----|-------------|
| UNKNOWN | 0 | Unknown drawing type |
| GA | 1 | General arrangement drawing |
| Assembly | 2 | Assembly drawing |
| CastUnit | 3 | Cast unit drawing |
| SinglePart | 4 | Single part drawing |
| Wizard | 5 | Wizard drawing |

### CatalogObjectTypeEnum
> The catalog object type. Used with `GetUserPropertyItems()` and `UserPropertyItem.AddToObjectType()`.

| Value | Int | Description |
|-------|-----|-------------|
| PART | 1 | Part |
| STEEL_BEAM | 2 | Steel beam |
| STEEL_COLUMN | 3 | Steel column |
| STEEL_ORTHOGONAL_BEAM | 4 | Steel orthogonal beam |
| STEEL_TWIN_PROFILE_BEAM | 5 | Steel twin profile beam |
| STEEL_CONTOUR_PLATE | 6 | Steel contour plate |
| STEEL_FOLDED_PLATE | 7 | Steel folded plate |
| CONCRETE_BEAM | 8 | Concrete beam |
| CONCRETE_COLUMN | 9 | Concrete column |
| CONCRETE_PAD_FOOTING | 10 | Concrete pad footing |
| CONCRETE_STRIP_FOOTING | 11 | Concrete strip footing |
| CONCRETE_PANEL | 12 | Concrete panel |
| CONCRETE_SLAB | 13 | Concrete slab |
| REINFORCING_BAR | 14 | Reinforcing bar |
| SURFACING | 15 | Surfacing |
| WELD | 16 | Weld |
| BOLT | 17 | Bolt |
| STEEL_ASSEMBLY | 18 | Steel assembly |
| PRECAST_CONCRETE_ASSEMBLY | 19 | Precast concrete assembly |
| INSITU_CONCRETE_ASSEMBLY | 20 | In situ concrete assembly |
| POUR_OBJECT | 21 | Pour object |
| POUR_BREAK | 22 | Pour break |
| GRID | 23 | Grid |
| PROJECT | 24 | Project |
| PHASE | 25 | Phase |
| TASK | 26 | Task |
| REFERENCE_MODEL | 27 | Reference model |
| REFERENCE_MODEL_OBJECT | 28 | Reference model object |
| SINGLE_PART_DRAWING | 29 | Single part drawing |
| ASSEMBLY_DRAWING | 30 | Assembly drawing |
| GA_DRAWING | 31 | General arrangement drawing |
| MULTI_DRAWING | 32 | Multidrawing |
| CAST_UNIT_DRAWING | 33 | Cast unit drawing |
| BENT_PLATE | 34 | Bent plate |
| STEEL_BREP_PART | 35 | Steel brep part |
| CONCRETE_BREP_PART | 36 | Concrete brep part |
| CHAMFER_OBJECT | 37 | Chamfer object |
| SURFACE_OBJECT | 38 | Surface object |
| GRID_PLANE | 39 | Grid plane |
| STEEL_SPIRAL_BEAM | 40 | Steel helix part |
| CONCRETE_SPIRAL_BEAM | 41 | Concrete helix part |
| POUR_UNIT | 42 | Pour unit |
| STEEL_LOFTED_PLATE | 43 | Steel lofted plate |
| CONCRETE_LOFTED_SLAB | 44 | Concrete lofted slab |
| BUILDING_SPACE | 45 | Building space object |

### UserPropertyFieldTypeEnum
> The user property field type.

| Value | Int | Description |
|-------|-----|-------------|
| FIELDTYPE_UNDEFINED | 0 | Undefined |
| FIELDTYPE_NUMBER | 1 | Number |
| FIELDTYPE_TEXT | 2 | Text |
| FIELDTYPE_DISTANCE | 3 | Distance |
| FIELDTYPE_PROFILE | 4 | Profile |
| FIELDTYPE_MATERIAL | 5 | Material |
| FIELDTYPE_TEXT_LIST_DISTANCE | 6 | Distance list |
| FIELDTYPE_FILE_IN | 7 | File in |
| FIELDTYPE_FILE_OUT | 8 | File out |
| FIELDTYPE_BOLT_STANDARD | 9 | Bolt standard |
| FIELDTYPE_BOLT_SIZE | 10 | Bolt size |
| FIELDTYPE_RATIO | 11 | Ratio |
| FIELDTYPE_STRAIN | 12 | Strain |
| FIELDTYPE_ANGLE | 13 | Angle |
| FIELDTYPE_DEFORMATION | 14 | Deformation |
| FIELDTYPE_DIMENSION | 15 | Dimension |
| FIELDTYPE_RADIUSOFINERTIA | 16 | Radius of inertia |
| FIELDTYPE_AREA | 17 | Area |
| FIELDTYPE_AREAPERLENGTH | 18 | Area/length |
| FIELDTYPE_SECTIONMODULUS | 19 | Section modulus |
| FIELDTYPE_MOMENTOFINERTIA | 20 | Moment of inertia |
| FIELDTYPE_TORSIONCONSTANT | 21 | Torsion constant |
| FIELDTYPE_WARPINGCONSTANT | 22 | Warping constant |
| FIELDTYPE_FORCE | 23 | Force |
| FIELDTYPE_WEIGHT | 24 | Weight |
| FIELDTYPE_DISTRIBLOAD | 25 | Distributed load |
| FIELDTYPE_SPRINGCONSTANT | 26 | Spring constant |
| FIELDTYPE_SURFACELOAD | 27 | Surface load |
| FIELDTYPE_STRENGTH | 28 | Strength |
| FIELDTYPE_MODULUS | 29 | Modulus |
| FIELDTYPE_DENSITY | 30 | Density |
| FIELDTYPE_MOMENT | 31 | Moment |
| FIELDTYPE_DISTRIBMOMENT | 32 | Distributed moment |
| FIELDTYPE_ROTSPRINGCONST | 33 | Rotational spring constant |
| FIELDTYPE_TEMPERATURE | 34 | Temperature |
| FIELDTYPE_THERMDILATCOEFF | 35 | Thermal coefficient |
| FIELDTYPE_ANALYSIS_RESTRAINT | 36 | Analysis restraint |
| FIELDTYPE_VOLUME | 37 | Volume |
| FIELDTYPE_REBAR_MAIN | 38 | Main reinforcement bar |
| FIELDTYPE_REBAR_STIRRUP | 39 | Stirrup reinforcement bar |
| FIELDTYPE_DATE | 40 | Date |
| FIELDTYPE_DATE_TIME_SEC | 41 | Date and time with seconds |
| FIELDTYPE_DATE_TIME_MIN | 42 | Date and time with minutes |
| FIELDTYPE_STUD_STANDARD | 43 | Stud standard |
| FIELDTYPE_STUD_SIZE | 44 | Stud size |
| FIELDTYPE_STUD_LENGTH | 45 | Stud length |
| FIELDTYPE_HOLE_TYPE | 46 | Hole type |
| FIELDTYPE_HOLE_DIRECTION | 47 | Hole direction |
| FIELDTYPE_WELD_TYPE | 48 | Weld type |
| FIELDTYPE_CHAMFER_TYPE | 49 | Chamfer type |
| FIELDTYPE_WELDING_SITE | 50 | Welding site |
| FIELDTYPE_FACTOR | 51 | Factor |
| FIELDTYPE_PART_NAME | 52 | Part name |
| FIELDTYPE_BOLT_TYPE | 53 | Bolt type |
| FIELDTYPE_COMPONENT_NAME | 54 | Component name |
| FIELDTYPE_REBAR_MESH | 55 | Rebar mesh |
| FIELDTYPE_USERDEFINED | 56 | User defined |
| FIELDTYPE_YES_NO | 57 | Yes/no |
| FIELDTYPE_COMPONENT_STANDARD_FILE | 58 | Component standard file |
| FIELDTYPE_REBAR_GRADE | 59 | Reinforcement bar grade |
| FIELDTYPE_REBAR_RADIUS | 60 | Reinforcement bar radius |
| FIELDTYPE_REBAR_SIZE | 61 | Reinforcement bar size |
| FIELDTYPE_HOOK_SHAPE | 62 | Reinforcement bar hook shape |
| FIELDTYPE_CROSSBAR_POSITION | 63 | Reinforcement cross bar position |
| FIELDTYPE_REBAR_SPLIT_TARGET | 64 | Rebar split target |
| FIELDTYPE_REBAR_STAGGER_TYPE | 65 | Rebar stagger type |
| FIELDTYPE_REBAR_LAPPING_SIDE | 66 | Rebar lapping side |
| FIELDTYPE_REBAR_LAPPING_OFFSET_DIRECTION | 67 | Rebar lapping offset direction |
| FIELDTYPE_REBAR_LAPPING_OFFSET_TYPE | 68 | Rebar lapping offset type |
| FIELDTYPE_DISTANCE_LIST_TOTAL | 69 | Distance list total |
| FIELDTYPE_REBAR_LENGTH_ADJUSTMENT_TYPE | 70 | Rebar length adjustment type |
| FIELDTYPE_SHAPE | 71 | Shape |
| FIELDTYPE_PLAIN_HOLE_TYPE | 72 | Plain hole type |
| FIELDTYPE_MASS_FLOW | 73 | Mass flow |
| FIELDTYPE_VOLUME_FLOW | 74 | Volume flow |
| FIELDTYPE_HEAT_CAPACITY | 75 | Heat capacity |
| FIELDTYPE_PRESSURE_LOSS | 76 | Pressure loss |
| FIELDTYPE_VELOCITY | 77 | Velocity |
| FIELDTYPE_FLOW_COEFFICIENT | 78 | Flow coefficient |
| FIELDTYPE_DYNAMIC_VISCOSITY | 79 | Dynamic viscosity |
| FIELDTYPE_KINEMATIC_VISCOSITY | 80 | Kinematic viscosity |
| FIELDTYPE_POWER | 81 | Power |

### UserPropertyLevelEnum
> The user property level.

| Value | Int | Description |
|-------|-----|-------------|
| LEVEL_MODEL | 1 | Defined as a model user property |
| LEVEL_PROJECT | 2 | Defined as a project user property |
| LEVEL_FIRM | 3 | Defined as a firm user property |
| LEVEL_ENVIRONMENT | 4 | Defined as an environment user property |
| LEVEL_COMMONDEFAULT | 5 | Defined as a common default user property |

### UserPropertyVisibilityEnum
> The visibility of the user property.

| Value | Int | Description |
|-------|-----|-------------|
| VISIBILITY_NORMAL | 0 | Visible and user can modify |
| VISIBILITY_READONLY | 1 | Visible but user cannot modify |
| VISIBILITY_HIDDEN | 2 | Hidden |
