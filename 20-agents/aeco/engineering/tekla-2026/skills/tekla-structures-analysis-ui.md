---
name: tekla-tekla-structures-analysis-ui
description: This skill encodes the tekla 2026.0 surface of the Tekla.Structures.Analysis.UI namespace — 3 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: AnalysisObjectPicker, AnalysisObjectSelector, AnalysisObjectPicker.PickObjectType.
---

# Tekla.Structures.Analysis.UI

Auto-generated from vendor docs for tekla 2026.0. 3 types in this namespace.

## AnalysisObjectPicker (class)

The AnalysisObjectPicker class can be used to prompt the user to do manual picks of analysis objects. The methods throw an exception if the user interrupts (cancels) the pick command.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/523a09a5-c6e6-d69d-0d24-26ee5f69f9ac)

### Constructors
- `AnalysisObjectPicker(...)` — Initializes a new instance of the AnalysisObjectPicker class

### Methods
#### `PickObject(...)`

Prompts the user to pick object from the model with the given prompt. Object is either analysis object or model object.

[Docs](https://developer.tekla.com/topic/en/18/47/8dea0aee-506d-06df-c9db-27e86bbdf77e)

#### `PickObjects(...)`

Prompts the user to pick objects from the model with the given prompt.

[Docs](https://developer.tekla.com/topic/en/18/47/a9e9fa8a-ead4-44b2-3838-ce4e26602695)

## AnalysisObjectPicker.PickObjectType (enum)

The possible pick object types.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/13b8b9fe-ca41-81c0-dd39-2a5f269ce8c0)

### Values
- `AnalysisObject` = `0` — Any analysis object.
- `AnalysisPart` = `1` — Analysis part object.
- `AnalysisNode` = `2` — Analysis node object.
- `AnalysisNodeLink` = `3` — Analysis node link object.

## AnalysisObjectSelector (class)

Get or set selected analysis objects in model views.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/e030d77d-a743-dc11-357f-42c32dab23e7)

### Constructors
- `AnalysisObjectSelector(...)` — Initializes a new instance of the AnalysisObjectSelector class.

### Methods
#### `GetSelectedObjects(...)`

Get the analysis objects selected in model views.

[Docs](https://developer.tekla.com/topic/en/18/47/67a2ab44-62cf-ee69-7f6b-c0231934cbeb)

#### `SetSelectedObjects(...)`

Set analysis objects as selected in model views.

[Docs](https://developer.tekla.com/topic/en/18/47/86208bd9-647e-0e3a-5dee-0cce9a4e50a2)

