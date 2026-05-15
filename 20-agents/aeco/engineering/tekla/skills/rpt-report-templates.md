---
name: tekla-rpt-report-templates
description: Tekla RPT report template file format. Block syntax (template{}, header{}, pageheader{}, row{}, footer{}, valuefield{}, text{}), formula language (GetValue, Sum, format, if/then/else), contenttype/sorttype for object iteration, and GetReportProperty API for programmatic field access. This skill should be used when working with .rpt files, report templates, or GetReportProperty.
---

# Tekla RPT Report Template Format and GetReportProperty API

## RPT File Structure Overview

Tekla report templates (.rpt) use a hierarchical block format. Each block is defined by a keyword, an optional identifier, and curly braces containing `property = value;` pairs and nested blocks.

```
template {
    name = "template_485";
    type = TEXTUAL;
    width = 80;
    maxheight = 57;
    version = 3.6;

    header _tmp_869 {
        name = "Header";
        height = 5;
        text _tmp_949 { string = "ASSEMBLY LIST"; };
        valuefield _tmp_948 { formula = "GetValue(\"DATE\")"; datatype = STRING; };
    };

    pageheader {
        name = "pageheader";
        height = 3;
        text { string = "Mark"; };
        text { string = "Profile"; };
    };

    row {
        name = "assembly";
        height = 2;
        contenttype = "ASSEMBLY";
        sorttype = COMBINE;
        rule = "if GetValue(\"MATERIAL_TYPE\") == \"CONCRETE\" then StepOver() else Output() endif";

        valuefield { formula = "GetValue(\"ASSEMBLY_POS\")"; datatype = STRING; };

        row _tmp_797 {
            name = "part";
            contenttype = "PART";
            sorttype = COMBINE;
            valuefield { formula = "GetValue(\"PART_POS\")"; datatype = STRING; };
            valuefield { formula = "GetValue(\"PROFILE\")"; datatype = STRING; };
        };
    };

    footer _tmp_886 {
        name = "footer";
        height = 4;
        valuefield { formula = "Sum(\"NUMBER_field\")"; datatype = INTEGER; };
        valuefield { formula = "format(Sum(\"Xextended_WEIGHT\"),\"Weight\",\"lbf\", 0)"; };
    };
};
```

### Block Hierarchy

- `template{}` -- Root container for the entire report
  - `header{}` -- Runs once at report start. Title, project info, date/time
  - `pageheader{}` -- Column headers. Repeats at top of each page
  - `row{}` -- **Primary data iterator.** The `contenttype` property determines what objects it iterates (ASSEMBLY, PART, BOLT, etc.)
    - `row{}` -- Nested rows iterate sub-objects (e.g., parts within an assembly)
    - `valuefield{}` -- Dynamic data field (formula-driven)
    - `text{}` -- Static text label
  - `footer{}` -- Runs once at report end. Totals and summaries using `Sum()` aggregation

**Key insight:** The `row{}` block with `contenttype` is the main data driver, not a passive layout container. Nested rows create parent-child iteration (assembly -> parts).

## Block Types and Properties

### template{}

Root container.

| Property | Description |
|----------|-------------|
| `name` | Internal template name (e.g., "template_485") |
| `type` | Output format: TEXTUAL, GRAPHICAL |
| `width` | Report width in character columns |
| `maxheight` | Maximum page height |
| `columns` | Column layout tuple, e.g., `(1, 1)` |
| `gap` | Gap between columns |
| `fillpolicy` | EVEN, etc. |
| `filldirection` | HORIZONTAL, VERTICAL |
| `margins` | Page margins tuple `(left, right, top, bottom)` |
| `gridxspacing` | Horizontal grid spacing |
| `gridyspacing` | Vertical grid spacing |
| `version` | Format version (e.g., 3.6) |
| `created` | Creation timestamp |
| `modified` | Last modified timestamp |

### header{}

Report header. Runs once at the beginning. Contains title, project metadata, date/time fields.

### pageheader{}

Page header. Repeats at the top of every page. Typically contains column labels (Mark, Profile, Length, Weight, etc.).

| Property | Description |
|----------|-------------|
| `name` | Block name |
| `height` | Header height |
| `outputpolicy` | NONE = only on first page, ALWAYS = every page |

### row{} (Data Row)

The primary data iteration block. **Critical properties:**

| Property | Description |
|----------|-------------|
| `name` | Row name (e.g., "assembly", "part") |
| `height` | Row height |
| `contenttype` | **Object type to iterate:** "ASSEMBLY", "PART", "BOLT", "WELD", "REINFORCEMENT", "SURFACE_TREATMENT" |
| `sorttype` | **Grouping:** COMBINE = group identical items (BOM), NONE = list all individually |
| `rule` | **Conditional filter** using if/then/else with StepOver()/Output() |
| `visibility` | TRUE/FALSE |
| `usecolumns` | Whether to use column layout |

**`contenttype` determines the iteration scope.** A row with `contenttype = "ASSEMBLY"` iterates assemblies; a nested row with `contenttype = "PART"` iterates parts within each assembly.

**`sorttype = COMBINE`** groups identical items together (same profile, material, length) and shows quantity — this is what creates BOM-style reports.

**`rule` controls filtering.** Example from real RPT:
```
rule = "if GetValue(\"MATERIAL_TYPE\") == \"CONCRETE\" || GetValue(\"MAINPART.NAME\") == \"JOIST\" then
  StepOver()
else
  Output()
endif";
```
- `StepOver()` = skip this object
- `Output()` = include this object in the report

### valuefield{}

Dynamic data field.

| Property | Description |
|----------|-------------|
| `name` | Field identifier (referenced by Sum() in footers) |
| `formula` | Expression using GetValue(), format(), Sum(), if/then/else |
| `location` | Position `(x, y)` in character coordinates |
| `datatype` | STRING, INTEGER, DOUBLE |
| `class` | Unit class (e.g., "Weight", "Length") for unit-aware formatting |
| `unit` | Display unit (e.g., "lbf", "kg", "mm") |
| `length` | Max display character width |
| `decimals` | Decimal places for numeric display |
| `justify` | LEFT, RIGHT, CENTERED |
| `sortdirection` | ASCENDING, DESCENDING, NONE — controls sort order |
| `oncombine` | **NONE** or **SUM** — when `sorttype = COMBINE`, SUM adds quantities/weights |
| `visibility` | TRUE = displayed, FALSE = hidden (used for sorting/filtering only) |
| `cacheable` | TRUE/FALSE |
| `fontname` | Font family |
| `fontsize` | Font size |
| `fontcolor` | Font color code |
| `fontstyle` | Font style (0 = normal) |

**`oncombine = SUM`** is critical for BOM reports — it tells Tekla to sum this field when combining identical items. Used for quantity (NUMBER) and weight fields.

**`visibility = FALSE`** fields are used for sorting and filtering without displaying.

### text{}

Static text label.

| Property | Description |
|----------|-------------|
| `string` | The literal text to display |
| `x1`, `y1` | Start position |
| `x2`, `y2` | End position |
| `justify` | LEFT, RIGHT, CENTERED |
| `fontname` | Font family |
| `fontsize` | Font size |

### footer{}

Report footer. Runs once at the end. Contains summary/totals using `Sum()` aggregation.

## Formula Language

### GetValue("FIELD_NAME")

Primary data retrieval function. Field names correspond to Tekla report properties.

### Dotted Notation for Nested Properties

Field names use dots to access nested object properties:

| Pattern | Description | Example |
|---------|-------------|---------|
| `FIELD_NAME` | Direct property on current object | `GetValue("PROFILE")` |
| `MAINPART.FIELD` | Main part of assembly | `GetValue("MAINPART.NAME")` |
| `MAINPART.FINISH` | Main part finish | `GetValue("MAINPART.FINISH")` |
| `MAINPART.MATERIAL` | Main part material | `GetValue("MAINPART.MATERIAL")` |
| `USERDEFINED.UDA_NAME` | User-defined attribute on current object | `GetValue("USERDEFINED.BOUGHT_ITEM")` |
| `MAINPART.USERDEFINED.UDA_NAME` | UDA on main part | `GetValue("MAINPART.USERDEFINED.EXISTING")` |
| `PROJECT.NAME` | Project-level property | `GetValue("PROJECT.NAME")` |
| `PROJECT.NUMBER` | Project number | `GetValue("PROJECT.NUMBER")` |
| `ADVANCED_OPTION.XS_IMPERIAL` | Advanced option | `GetValue("ADVANCED_OPTION.XS_IMPERIAL")` |

### Common Field Names

**Identification:**
- `NAME` -- Part name
- `ASSEMBLY_POS` -- Assembly position mark
- `PART_POS` -- Part position mark
- `TOP_LEVEL` -- Top-level assembly mark
- `PHASE` -- Construction phase number
- `MAIN_PART` -- Integer flag: 1 = main part, 0 = secondary

**Geometry:**
- `PROFILE` -- Profile designation (e.g., "W12X26")
- `LENGTH` -- Part length
- `LENGTH_NET` -- Net length after cuts
- `WIDTH` -- Part width
- `HEIGHT` -- Part height
- `AREA` -- Surface area
- `VOLUME` -- Part volume

**Material:**
- `MATERIAL` -- Material grade (e.g., "A992")
- `MATERIAL_TYPE` -- Material classification (STEEL, CONCRETE)
- `FINISH` -- Surface finish
- `GRADE` -- Material grade

**Quantity and Weight:**
- `NUMBER` -- Quantity of identical items (used with `oncombine = SUM`)
- `WEIGHT` -- Single part weight
- `WEIGHT_TOTAL` -- Total weight (weight x quantity)
- `MODEL_TOTAL` -- Model-wide total count

**Section Properties:**
- `FLANGE_THICKNESS` -- Flange thickness
- `WEB_THICKNESS` -- Web thickness

**Report Metadata:**
- `DATE` -- Current date
- `TIME` -- Current time
- `PAGE` -- Page number
- `PRODUCT_NAME` -- Product name

### Conditional Logic (if/then/else/endif)

```
if GetValue("USERDEFINED.BOUGHT_ITEM") == "Yes" then
  GetValue("USERDEFINED.BOUGHT_ITEM_NAME")
else
  GetValue("PROFILE")
endif
```

Conditions support: `==`, `!=`, `||`, `&&`. Can be nested with multiple `endif`.

### String Functions

- `match(string, pattern)` -- Wildcard pattern matching. `match(GetValue("PROFILE"), "[0123456789]*")`
- `mid(string, start)` -- Substring from position
- `find(string, substring)` -- Find position of substring
- `IsSet("FIELD")` -- Check if a field/UDA exists (returns boolean)

### Unit Formatting

```
format(value, "Type", "unit", decimals)
```

Examples:
- `format(GetValue("LENGTH"), "Length", "ft-frac", "1/16")` -- Length in feet-fractions
- `format(GetValue("LENGTH"), "Length", "mm", 0)` -- Length in millimeters
- `format(GetValue("WEIGHT"), "Weight", "lbf", 0)` -- Weight in pounds
- `format(GetValue("WEIGHT"), "Weight", "kg", 1)` -- Weight in kilograms

### Aggregation

```
Sum("field_name")
```

References a `valuefield` by its `name` property and sums all values. Used in `footer{}` blocks for report totals.

Example: `format(Sum("Xextended_WEIGHT"), "Weight", "lbf", 0)` -- Total weight in footer.

### Row Control Flow

Used inside `rule` properties on `row{}` blocks:
- `StepOver()` -- Skip the current object (exclude from report)
- `Output()` -- Include the current object in output

## GetReportProperty API

The critical API for programmatically accessing any field that appears in a Tekla report template. The field names used in `GetValue("FIELD_NAME")` in RPT files map directly to `GetReportProperty` parameter names.

### Method Overloads

```csharp
// String properties
bool GetReportProperty(string name, ref string value)

// Numeric (double) properties
bool GetReportProperty(string name, ref double value)

// Integer properties
bool GetReportProperty(string name, ref int value)
```

All overloads return `true` if the property was found and retrieved, `false` otherwise.

### Important Notes

- `GetReportProperty` works on **any ModelObject** (Part, Beam, Column, ContourPlate, Assembly, BoltGroup, Reinforcement, etc.)
- The `name` parameter uses the **same field names** as `GetValue("FIELD_NAME")` in RPT formulas, including dotted notation
- Always call `Select()` on the ModelObject before calling `GetReportProperty`
- Unlike `GetUserProperty`, report properties are calculated/derived values, not stored UDAs
- Use the correct type overload: string for text fields (PROFILE, MATERIAL, NAME), double for numeric fields (LENGTH, WEIGHT, AREA), int for counts (PHASE)
- For `USERDEFINED.*` fields in RPT formulas, use `GetUserProperty` instead (these are UDAs, not report properties)

### Usage Examples

```csharp
// String properties
string profile = "";
part.GetReportProperty("PROFILE", ref profile);

string material = "";
part.GetReportProperty("MATERIAL", ref material);

string assemblyPos = "";
part.GetReportProperty("ASSEMBLY_POS", ref assemblyPos);

string partPos = "";
part.GetReportProperty("PART_POS", ref partPos);

string name = "";
part.GetReportProperty("NAME", ref name);

string finish = "";
part.GetReportProperty("FINISH", ref finish);

string topLevel = "";
part.GetReportProperty("TOP_LEVEL", ref topLevel);
```

```csharp
// Numeric properties
double weight = 0;
part.GetReportProperty("WEIGHT", ref weight);

double length = 0;
part.GetReportProperty("LENGTH", ref length);

double area = 0;
part.GetReportProperty("AREA", ref area);

double volume = 0;
part.GetReportProperty("VOLUME", ref volume);

double flangeThickness = 0;
part.GetReportProperty("FLANGE_THICKNESS", ref flangeThickness);

double webThickness = 0;
part.GetReportProperty("WEB_THICKNESS", ref webThickness);
```

```csharp
// Integer properties
int quantity = 0;
part.GetReportProperty("QUANTITY", ref quantity);

int phase = 0;
part.GetReportProperty("PHASE", ref phase);
```

### Complete Example: Iterating Model Objects

```csharp
var model = new Model();
var objects = model.GetModelObjectSelector()
    .GetAllObjectsWithType(ModelObject.ModelObjectEnum.BEAM);

while (objects.MoveNext())
{
    var part = objects.Current as Part;
    if (part is null) continue;

    part.Select();

    string profile = "";
    part.GetReportProperty("PROFILE", ref profile);

    string material = "";
    part.GetReportProperty("MATERIAL", ref material);

    double weight = 0;
    part.GetReportProperty("WEIGHT", ref weight);

    double length = 0;
    part.GetReportProperty("LENGTH", ref length);

    string assemblyPos = "";
    part.GetReportProperty("ASSEMBLY_POS", ref assemblyPos);

    // Use retrieved values...
}
```

## Reverse-Engineering an RPT File

When given an .rpt file, follow this pattern to generate equivalent agent command code:

### Step 1: Identify Object Iteration

Find `row{}` blocks with `contenttype`. This determines what to iterate:
- `contenttype = "ASSEMBLY"` -> iterate assemblies
- `contenttype = "PART"` -> iterate parts
- Nested rows mean parent-child iteration (assembly -> parts within it)

Check `sorttype`: if `COMBINE`, the report groups identical items — the generated code should aggregate/group similarly.

### Step 2: Check Filtering Rules

Read the `rule` property on data rows. If present, replicate the filtering logic:
- `GetValue("MATERIAL_TYPE") == "CONCRETE"` with `StepOver()` = skip concrete parts
- `GetValue("MAINPART.NAME") == "JOIST"` with `StepOver()` = skip joists

### Step 3: Extract Field Names

Scan all `valuefield{}` blocks with `visibility = TRUE` and extract field names from `GetValue("FIELD_NAME")` formulas. These are the displayed data columns.

Also note fields with `visibility = FALSE` — these are used for sorting/filtering only.

### Step 4: Map Fields to GetReportProperty Calls

Each `GetValue("FIELD_NAME")` maps to a `GetReportProperty("FIELD_NAME", ref value)` call. Choose the correct type based on `datatype`:
- `datatype = STRING` -> `ref string`
- `datatype = DOUBLE` -> `ref double`
- `datatype = INTEGER` -> `ref int`

For `USERDEFINED.*` fields, use `GetUserProperty` instead of `GetReportProperty`.

### Step 5: Identify Column Headers

Read `pageheader{}` text labels to determine column names for the output. The `text{}` elements in order give the header row.

### Step 6: Check Footer Aggregations

If the RPT has a `footer{}` with `Sum()` formulas, include totals in the generated output.

### Step 7: Generate Code

Create code that:
1. Connects to the Tekla model
2. Iterates the appropriate model objects based on `contenttype`
3. Applies filtering based on `rule` logic
4. Calls `GetReportProperty` for each visible field
5. Groups results if `sorttype = COMBINE`
6. Formats output to match column headers from `pageheader{}`
7. Adds totals if footer aggregations exist
8. Outputs to the desired format (Excel, CSV, text, etc.)

## Difference Between GetReportProperty and GetUserProperty

| Aspect | GetReportProperty | GetUserProperty |
|--------|-------------------|-----------------|
| **Data source** | Calculated/derived values from model | User-defined attributes (UDAs) stored in DB |
| **Field names** | Standard report fields (PROFILE, WEIGHT, etc.) | Custom UDA names defined by user/environment |
| **RPT formula** | `GetValue("PROFILE")` | `GetValue("USERDEFINED.MY_UDA")` |
| **Requires Modify()** | No (read-only) | No (reads/writes directly to DB) |
| **Requires Select()** | Yes | Yes |
| **Writable** | No | Yes (via SetUserProperty) |
| **Use case** | Reading calculated model data for reports/exports | Reading/writing custom metadata |
