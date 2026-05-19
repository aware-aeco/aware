---
name: grasshopper-grasshopper-kernel-gdl
description: This skill encodes the grasshopper 8.0 surface of the Grasshopper.Kernel.GDL namespace — 1 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: GH_GDLParser.
---

# Grasshopper.Kernel.GDL

Auto-generated from vendor docs for grasshopper 8.0. 1 types in this namespace.

## GH_GDLParser (class)

Provided functionality for parsing and writing Grasshopper Descriptor Language text.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_Kernel_GDL_GH_GDLParser.htm)

### Methods
#### `public static GH_Document ParseGDL(string[] lines, out string[] messages)`

Convert a GDL text into an actual document.

**Parameters:**
- `lines` (System.String[]) — Lines to parse.
- `messages` (System.String[]) — Messages encountered.

**Returns:** `GH_Document` — A document on success or null on failure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GDL_GH_GDLParser_ParseGDL.htm)

