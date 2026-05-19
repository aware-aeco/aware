---
name: navisworks-autodesk-navisworks-api-interop-plugins
description: This skill encodes the navisworks 2026.0 surface of the Autodesk.Navisworks.Api.Interop.Plugins namespace — 4 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: LegacyManager, LegacyPlugin, ModelDataPlugin, ModelDataPluginRecord.
---

# Autodesk.Navisworks.Api.Interop.Plugins

Auto-generated from vendor docs for navisworks 2026.0. 4 types in this namespace.

## LegacyManager (class)

LegacyManager class in Autodesk.Navisworks.Api.Interop.Plugins.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.Interop.Plugins.LegacyManager)

### Methods
#### `static int LoadPlugins(System.Attribute attribLoading)`

LoadPlugins method of LegacyManager.

**Parameters:**
- `attribLoading` (System.Attribute)

**Returns:** `int` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Interop.Plugins.LegacyManager.LoadPlugins%28System.Attribute%29)

### Properties
- `Plugins` (System.Collections.ObjectModel.Collection<Autodesk.Navisworks.Api.Interop.Plugins.LegacyPlugin>, get) — Plugins property of LegacyManager.

## LegacyPlugin (class)

LegacyPlugin class in Autodesk.Navisworks.Api.Interop.Plugins.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.Interop.Plugins.LegacyPlugin)

### Constructors
- `LegacyPlugin(System.Reflection.Assembly assembly, Autodesk.Navisworks.Api.Interop.IPlugin iPlugin, string DataDir)` — Constructs a new LegacyPlugin.

### Properties
- `AssemblyPrefix` (string, get) — AssemblyPrefix property of LegacyPlugin.
- `DataDir` (string, get) — DataDir property of LegacyPlugin.
- `PluginInterface` (Autodesk.Navisworks.Api.Interop.IPlugin, get) — PluginInterface property of LegacyPlugin.

## ModelDataPlugin (class)

ModelDataPlugin class in Autodesk.Navisworks.Api.Interop.Plugins.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.Interop.Plugins.ModelDataPlugin)

### Methods
#### `void ImportData(Autodesk.Navisworks.Api.Interop.LcOpModel m, string filename)`

ImportData method of ModelDataPlugin.

**Parameters:**
- `m` (Autodesk.Navisworks.Api.Interop.LcOpModel)
- `filename` (string)

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Interop.Plugins.ModelDataPlugin.ImportData%28Autodesk.Navisworks.Api.Interop.LcOpModel%2CSystem.String%29)

## ModelDataPluginRecord (class)

ModelDataPluginRecord class in Autodesk.Navisworks.Api.Interop.Plugins.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.Interop.Plugins.ModelDataPluginRecord)

