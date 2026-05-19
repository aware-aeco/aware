---
name: dynamo-dynamo-annotations
description: This skill encodes the dynamo 4.1.1 surface of the Dynamo.Annotations namespace — 1 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: NotifyPropertyChangedInvocatorAttribute.
---

# Dynamo.Annotations

Auto-generated from vendor docs for dynamo 4.1.1. 1 types in this namespace.

## NotifyPropertyChangedInvocatorAttribute (class)

Indicates that the method is contained in a type that implements System.ComponentModel.INotifyPropertyChanged interface and this method is used to notify that some property value changed.

**Remarks:** The method should be non-static and conform to one of the supported signatures: ; NotifyChanged(string); NotifyChanged(params string[]); NotifyChanged{T}(Expression{Func{T}}); NotifyChanged{T,U}(Expression{Func{T,U}}); SetProperty{T}(ref T, T, string)

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Properties/Annotations.cs)

### Constructors
- `void NotifyPropertyChangedInvocatorAttribute()` — NotifyPropertyChangedInvocatorAttribute.NotifyPropertyChangedInvocatorAttribute
- `void NotifyPropertyChangedInvocatorAttribute(string parameterName)` — NotifyPropertyChangedInvocatorAttribute.NotifyPropertyChangedInvocatorAttribute

### Properties
- `ParameterName` (string, get) — NotifyPropertyChangedInvocatorAttribute.ParameterName

