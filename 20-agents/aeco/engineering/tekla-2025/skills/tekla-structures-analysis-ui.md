---
name: tekla-tekla-structures-analysis-ui
description: This skill encodes the tekla 2025.0 surface of the Tekla.Structures.Analysis.UI namespace — 3 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: AnalysisObjectSelector, AnalysisObjectPicker, AnalysisObjectPicker.PickObjectType.
---

# Tekla.Structures.Analysis.UI

Auto-generated from vendor docs for tekla 2025.0. 3 types in this namespace.

## AnalysisObjectPicker (class)

The AnalysisObjectPicker class can be used to prompt the user to do manual picks of analysis objects. The methods throw an exception if the user interrupts (cancels) the pick command.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/523a09a5-c6e6-d69d-0d24-26ee5f69f9ac)

### Constructors
- `public AnalysisObjectPicker()` — Initializes a new instance of the AnalysisObjectPicker class

### Methods
#### `public AnalysisObject PickObject(string analysisModelName, AnalysisObjectPicker.PickObjectType type, string prompt)`

Prompts the user to pick object from the model with the given prompt. Object is either analysis object or model object.

**Parameters:**
- `analysisModelName` (System.String) — The analysis model name.
- `type` (Tekla.Structures.Analysis.UI.AnalysisObjectPicker.PickObjectType) — The type of object to pick.
- `prompt` (System.String) — The string to display as user guidance.

**Returns:** `AnalysisObject` — The analysis object instance.

[Docs](https://developer.tekla.com/topic/en/18/43/8dea0aee-506d-06df-c9db-27e86bbdf77e)

#### `public List<AnalysisObject> PickObjects(string analysisModelName, AnalysisObjectPicker.PickObjectType type, string prompt)`

Prompts the user to pick objects from the model with the given prompt.

**Parameters:**
- `analysisModelName` (System.String) — The analysis model name.
- `type` (Tekla.Structures.Analysis.UI.AnalysisObjectPicker.PickObjectType) — The type of objects to pick.
- `prompt` (System.String) — The string to display as user guidance.

**Returns:** `List<AnalysisObject>` — An enumerator of analysis object instances.

[Docs](https://developer.tekla.com/topic/en/18/43/a9e9fa8a-ead4-44b2-3838-ce4e26602695)

## AnalysisObjectPicker.PickObjectType (enum)

The possible pick object types.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/13b8b9fe-ca41-81c0-dd39-2a5f269ce8c0)

### Values
- `AnalysisObject` = `0` — Any analysis object.
- `AnalysisPart` = `1` — Analysis part object.
- `AnalysisNode` = `2` — Analysis node object.
- `AnalysisNodeLink` = `3` — Analysis node link object.

## AnalysisObjectSelector (class)

Get or set selected analysis objects in model views.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/e030d77d-a743-dc11-357f-42c32dab23e7)

### Constructors
- `public AnalysisObjectSelector()` — Initializes a new instance of the AnalysisObjectSelector class.

### Methods
#### `public AnalysisObjectEnumerator GetSelectedObjects(string analysisModelName)`

Get the analysis objects selected in model views.

**Parameters:**
- `analysisModelName` (System.String) — The analysis model name.

**Returns:** `AnalysisObjectEnumerator` — The analysis object enumerator of analysis objects selected in model views.

[Docs](https://developer.tekla.com/topic/en/18/43/67a2ab44-62cf-ee69-7f6b-c0231934cbeb)

#### `public int SetSelectedObjects(List<AnalysisObject> analysisObjects, bool append)`

Set analysis objects as selected in model views.

**Parameters:**
- `analysisObjects` (System.Collections.Generic.List<AnalysisObject>) — The analysis objects.
- `append` (System.Boolean) — If true, objects are added to currently selected. If false, current selection is cleared.

**Returns:** `Int32` — The count of objects selected.

[Docs](https://developer.tekla.com/topic/en/18/43/86208bd9-647e-0e3a-5dee-0cce9a4e50a2)

