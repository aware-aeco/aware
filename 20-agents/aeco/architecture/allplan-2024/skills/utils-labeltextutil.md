---
name: allplan-utils-labeltextutil
description: This skill encodes the allplan 2024.0 surface of the Utils.LabelTextUtil namespace — 4 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: LabelTextDimensionUnit, LabelTextFormat, LabelTextFrame, LabelTextUtil.
---

# Utils.LabelTextUtil

Auto-generated from vendor docs for allplan 2024.0. 4 types in this namespace.

## LabelTextDimensionUnit (enum)

definition the label text dimension unit

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/Utils/LabelTextUtil/LabelTextDimensionUnit/)

### Values
- `CM` = `1`
- `CM2` = `5`
- `CM3` = `9`
- `DEG` = `12`
- `DM` = `2`
- `DM2` = `6`
- `DM3` = `10`
- `M` = `3`
- `M2` = `7`
- `M3` = `11`
- `MM` = `0`
- `MM2` = `4`
- `MM3` = `8`
- `NO` = `13`

## LabelTextFormat (enum)

definition the label text format unit

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/Utils/LabelTextUtil/LabelTextFormat/)

### Values
- `CHARACTER` = `0`
- `FLOAT` = `1`
- `INTEGER` = `2`

## LabelTextFrame (enum)

definition of the label text frame types

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/Utils/LabelTextUtil/LabelTextFrame/)

### Values
- `CIRCLE` = `4`
- `FILLET_RECTANGLE` = `3`
- `NO` = `0`
- `OCTAGON` = `5`
- `RECTANGLE` = `2`
- `UNDERLINE` = `1`

## LabelTextUtil (class)

Implementation of the utility for the label text creation

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/Utils/LabelTextUtil/LabelTextUtil/)

### Constructors
- `LabelTextUtil()` — initialize

### Methods
#### `add_attribute(doc, attribute_id, value='')`

add an attribute to the label text

**Remarks:** add an attribute to the label text

**Parameters:**
- `doc` (DocumentAdapter) — document of the Allplan drawing files
- `attribute_id` (int) — attribute ID
- `value` (Any) — value

[Docs](https://pythonparts.allplan.com/2024/api_reference/Utils/LabelTextUtil/LabelTextUtil/#Utils.LabelTextUtil.LabelTextUtil.add_attribute)

#### `add_parameter(build_ele, parameter_name)`

add a parameter to the label text

**Remarks:** add a parameter to the label text

**Parameters:**
- `build_ele` (BuildingElement) — building element with the parameter properties
- `parameter_name` (str) — parameter name

[Docs](https://pythonparts.allplan.com/2024/api_reference/Utils/LabelTextUtil/LabelTextUtil/#Utils.LabelTextUtil.LabelTextUtil.add_parameter)

#### `add_parameter_name(parameter_name, value)`

add a parameter name and value to the label text

**Remarks:** add a parameter name and value to the label text

**Parameters:**
- `parameter_name` (str) — parameter name
- `value` (Any) — value

[Docs](https://pythonparts.allplan.com/2024/api_reference/Utils/LabelTextUtil/LabelTextUtil/#Utils.LabelTextUtil.LabelTextUtil.add_parameter_name)

#### `create_label_default_text()`

create the text

**Remarks:** create the text

**Returns:** `str` — default label text

[Docs](https://pythonparts.allplan.com/2024/api_reference/Utils/LabelTextUtil/LabelTextUtil/#Utils.LabelTextUtil.LabelTextUtil.create_label_default_text)

#### `create_label_text()`

create the label text

**Remarks:** create the label text

**Returns:** `str` — label text

[Docs](https://pythonparts.allplan.com/2024/api_reference/Utils/LabelTextUtil/LabelTextUtil/#Utils.LabelTextUtil.LabelTextUtil.create_label_text)

#### `set_dimension_unit(dimension_unit)`

set the dimension

**Remarks:** set the dimension

**Parameters:**
- `dimension_unit` (LabelTextDimensionUnit) — dimension unit

[Docs](https://pythonparts.allplan.com/2024/api_reference/Utils/LabelTextUtil/LabelTextUtil/#Utils.LabelTextUtil.LabelTextUtil.set_dimension_unit)

#### `set_dimension_unit_by_value_type(value_type)`

set the dimension unit by the value type

**Remarks:** set the dimension unit by the value type

**Parameters:**
- `value_type` (ParameterPropertyValueType) — Value type

[Docs](https://pythonparts.allplan.com/2024/api_reference/Utils/LabelTextUtil/LabelTextUtil/#Utils.LabelTextUtil.LabelTextUtil.set_dimension_unit_by_value_type)

#### `set_format(text_format, digits, decimals=0)`

set the format of the label text

**Remarks:** set the format of the label text

**Parameters:**
- `text_format` (LabelTextFormat) — text format
- `digits` (int) — digits
- `decimals` (int) — decimals

[Docs](https://pythonparts.allplan.com/2024/api_reference/Utils/LabelTextUtil/LabelTextUtil/#Utils.LabelTextUtil.LabelTextUtil.set_format)

#### `set_format_by_value_type(value_type)`

set the format by the value type

**Remarks:** set the format by the value type

**Parameters:**
- `value_type` (ParameterPropertyValueType) — Value type

[Docs](https://pythonparts.allplan.com/2024/api_reference/Utils/LabelTextUtil/LabelTextUtil/#Utils.LabelTextUtil.LabelTextUtil.set_format_by_value_type)

#### `set_pre_text(pre_text)`

set the pre text

**Remarks:** set the pre text

**Parameters:**
- `pre_text` (str) — prefix text

[Docs](https://pythonparts.allplan.com/2024/api_reference/Utils/LabelTextUtil/LabelTextUtil/#Utils.LabelTextUtil.LabelTextUtil.set_pre_text)

#### `set_text_frame(text_frame)`

set the text frame of the label

**Remarks:** set the text frame of the label

**Parameters:**
- `text_frame` (LabelTextFrame) — text frame

[Docs](https://pythonparts.allplan.com/2024/api_reference/Utils/LabelTextUtil/LabelTextUtil/#Utils.LabelTextUtil.LabelTextUtil.set_text_frame)

