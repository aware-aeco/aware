---
name: allplan-utils-tabulardatautil
description: This skill encodes the allplan 2024.0 surface of the Utils.TabularDataUtil namespace — 3 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: DataFrame, Functions, Series.
---

# Utils.TabularDataUtil

Auto-generated from vendor docs for allplan 2024.0. 3 types in this namespace.

## DataFrame (class)

Two-dimensional tabular data structure with labeled axes (rows and columns)

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/Utils/TabularDataUtil/DataFrame/)

### Constructors
- `DataFrame(data=None, columns=None, index=None)` — Initialization of class DataFrame

### Methods
#### `__getitem__(key)`

Get items

**Remarks:** Get items

**Parameters:**
- `key` (Union[int, str, slice, Series]) — it can be

**Returns:** `Union[DataFrame, Series]` — item for the index

[Docs](https://pythonparts.allplan.com/2024/api_reference/Utils/TabularDataUtil/DataFrame/#Utils.TabularDataUtil.DataFrame.__getitem__)

#### `__iter__()`

get the data frame iterator

**Remarks:** get the data frame iterator

[Docs](https://pythonparts.allplan.com/2024/api_reference/Utils/TabularDataUtil/DataFrame/#Utils.TabularDataUtil.DataFrame.__iter__)

#### `__len__()`

get the length of the data frame

**Remarks:** get the length of the data frame

**Returns:** `int` — length of the data frame

[Docs](https://pythonparts.allplan.com/2024/api_reference/Utils/TabularDataUtil/DataFrame/#Utils.TabularDataUtil.DataFrame.__len__)

#### `__repr__()`

create a string from the data frame

**Remarks:** create a string from the data frame

**Returns:** `str` — data frame string

[Docs](https://pythonparts.allplan.com/2024/api_reference/Utils/TabularDataUtil/DataFrame/#Utils.TabularDataUtil.DataFrame.__repr__)

#### `get_row(key, value)`

Get a row from data frame by a condition: column_label == value

**Remarks:** Get a row from data frame by a condition: column_label == value

**Parameters:**
- `key` (str) — column label
- `value` (str) — new value

**Returns:** `Optional[Series]` — row value

[Docs](https://pythonparts.allplan.com/2024/api_reference/Utils/TabularDataUtil/DataFrame/#Utils.TabularDataUtil.DataFrame.get_row)

#### `loc(location)`

Get a row from data frame

**Remarks:** Get a row from data frame

**Parameters:**
- `location` (int) — int value of row index, natural row index

**Returns:** `Series` — Series

[Docs](https://pythonparts.allplan.com/2024/api_reference/Utils/TabularDataUtil/DataFrame/#Utils.TabularDataUtil.DataFrame.loc)

#### `short_repr()`

Represent the DataFrame by its row size x column size

**Remarks:** Represent the DataFrame by its row size x column size

**Returns:** `str` — short data string frame

[Docs](https://pythonparts.allplan.com/2024/api_reference/Utils/TabularDataUtil/DataFrame/#Utils.TabularDataUtil.DataFrame.short_repr)

## Functions (static-class)

Module-level functions of Utils.TabularDataUtil

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/Utils/TabularDataUtil/_functions/)

### Methods
#### `concat(data_frames, _ignore_index=False, _keys=None)`

concatenate

**Remarks:** concatenate

**Parameters:**
- `data_frames` (List[DataFrame]) — data frames
- `_ignore_index` (bool) — ignored index state
- `_keys` (Any) — keys

**Returns:** `Optional[DataFrame]` — concatenated data frame

[Docs](https://pythonparts.allplan.com/2024/api_reference/Utils/TabularDataUtil/_functions/#Utils.TabularDataUtil.concat)

#### `eval_noex(value, default=None, value_type=None)`

Evaluate value from Table without throwing exceptions, only used to evaluate value that is string or None

**Remarks:** Evaluate value from Table without throwing exceptions, only used to evaluate value that is string or None

**Parameters:**
- `value` (Any) — new value
- `default` (Any) — has a higher priority than value_type, when both default and value_type are given
- `value_type` (Any) — string that indicates the value type

**Returns:** `Any` — eval result

[Docs](https://pythonparts.allplan.com/2024/api_reference/Utils/TabularDataUtil/_functions/#Utils.TabularDataUtil.eval_noex)

#### `has_duplicates(data_frame, duplicates)`

check if data frame has duplicated column headers and return them if it has

**Remarks:** check if data frame has duplicated column headers and return them if it has

**Parameters:**
- `data_frame` (DataFrame) — data frame
- `duplicates` (List[Union[int, str]]) — duplicates

**Returns:** `bool` — duplicated state

[Docs](https://pythonparts.allplan.com/2024/api_reference/Utils/TabularDataUtil/_functions/#Utils.TabularDataUtil.has_duplicates)

#### `read_csv(csv_path)`

read the data from the csv file

**Remarks:** read the data from the csv file

**Parameters:**
- `csv_path` (str) — path of the csv file

**Returns:** `DataFrame` — data frame

[Docs](https://pythonparts.allplan.com/2024/api_reference/Utils/TabularDataUtil/_functions/#Utils.TabularDataUtil.read_csv)

#### `read_matrix(lists)`

read a matrix

**Remarks:** read a matrix

**Parameters:**
- `lists` (List[List[Any]]) — lists

**Returns:** `DataFrame` — matrix

[Docs](https://pythonparts.allplan.com/2024/api_reference/Utils/TabularDataUtil/_functions/#Utils.TabularDataUtil.read_matrix)

#### `read_xlsx(xlsx_path, sheet=0)`

read an Excel file

**Remarks:** read an Excel file

**Parameters:**
- `xlsx_path` (str) — excel path
- `sheet` (Union[int, str]) — sheet index

**Returns:** `DataFrame` — data frame

[Docs](https://pythonparts.allplan.com/2024/api_reference/Utils/TabularDataUtil/_functions/#Utils.TabularDataUtil.read_xlsx)

#### `read_xml(xml_path, *args)`

Read partial data from xml file and return a DataFrame. Xml file should be those which can be converted into tabular data.

**Remarks:** Read partial data from xml file and return a DataFrame. Xml file should be those which can be converted into tabular data.

**Parameters:**
- `xml_path` (str) — xml file path
- `args` (object) — string, the ElementTree tags, which serve later as column labels of data frame

**Returns:** `Optional[DataFrame]` — data frame

[Docs](https://pythonparts.allplan.com/2024/api_reference/Utils/TabularDataUtil/_functions/#Utils.TabularDataUtil.read_xml)

## Series (class)

One-dimensional array with axis labels

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/Utils/TabularDataUtil/Series/)

### Constructors
- `Series(data=None, axis=None, name=0)` — Initialization of class Series

### Methods
#### `__eq__(other)`

comparison of the series values with an other value

**Remarks:** comparison of the series values with an other value

**Parameters:**
- `other` (Any) — value to compare

**Returns:** `Series` — series with the comparison state

[Docs](https://pythonparts.allplan.com/2024/api_reference/Utils/TabularDataUtil/Series/#Utils.TabularDataUtil.Series.__eq__)

#### `__ge__(other)`

greater equal comparison of the series values with an other value

**Remarks:** greater equal comparison of the series values with an other value

**Parameters:**
- `other` (Any) — value to compare

**Returns:** `Series` — series with the comparison state

[Docs](https://pythonparts.allplan.com/2024/api_reference/Utils/TabularDataUtil/Series/#Utils.TabularDataUtil.Series.__ge__)

#### `__getitem__(key)`

Get items

**Remarks:** Get items

**Parameters:**
- `key` (Union[int, str, slice]) — it can be a column name or a range of row index

**Returns:** `Optional[Union[int, str, Series]]` — item for the index

[Docs](https://pythonparts.allplan.com/2024/api_reference/Utils/TabularDataUtil/Series/#Utils.TabularDataUtil.Series.__getitem__)

#### `__gt__(other)`

greater comparison of the series values with an other value

**Remarks:** greater comparison of the series values with an other value

**Parameters:**
- `other` (Any) — value to compare

**Returns:** `Series` — series with the comparison state

[Docs](https://pythonparts.allplan.com/2024/api_reference/Utils/TabularDataUtil/Series/#Utils.TabularDataUtil.Series.__gt__)

#### `__le__(other)`

less equal comparison of the series values with an other value

**Remarks:** less equal comparison of the series values with an other value

**Parameters:**
- `other` (Any) — value to compare

**Returns:** `Series` — series with the comparison state

[Docs](https://pythonparts.allplan.com/2024/api_reference/Utils/TabularDataUtil/Series/#Utils.TabularDataUtil.Series.__le__)

#### `__lt__(other)`

less comparison of the series values with an other value

**Remarks:** less comparison of the series values with an other value

**Parameters:**
- `other` (Any) — value to compare

**Returns:** `Series` — series with the comparison state

[Docs](https://pythonparts.allplan.com/2024/api_reference/Utils/TabularDataUtil/Series/#Utils.TabularDataUtil.Series.__lt__)

#### `__ne__(other)`

unequal comparison of the series values with an other value

**Remarks:** unequal comparison of the series values with an other value

**Parameters:**
- `other` (Any) — value to compare

**Returns:** `Series` — series with the comparison state

[Docs](https://pythonparts.allplan.com/2024/api_reference/Utils/TabularDataUtil/Series/#Utils.TabularDataUtil.Series.__ne__)

#### `__operation(operator_fct, other)`

execute a comparison operation of the series values with an other value

**Remarks:** execute a comparison operation of the series values with an other value

**Parameters:**
- `operator_fct` (Callable) — operator function
- `other` (Series) — value to compare

**Returns:** `Series` — series

[Docs](https://pythonparts.allplan.com/2024/api_reference/Utils/TabularDataUtil/Series/#Utils.TabularDataUtil.Series.__operation)

#### `__repr__()`

create a string from a series

**Remarks:** create a string from a series

**Returns:** `str` — series data a string

[Docs](https://pythonparts.allplan.com/2024/api_reference/Utils/TabularDataUtil/Series/#Utils.TabularDataUtil.Series.__repr__)

