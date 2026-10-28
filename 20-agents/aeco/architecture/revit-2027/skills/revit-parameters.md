---
name: revit-parameters
description: This skill should be used when composing snippets that read or write Revit element parameters — built-in, project, shared, or family parameters; type vs instance parameters; the StorageType conversion gotchas; and the `Element.get_Parameter(BuiltInParameter)` / `LookupParameter(string)` / `GetParameters(string)` accessor differences. Encodes the parameter type hierarchy, the AsDouble/AsInteger/AsString/AsElementId selectors that depend on StorageType, the bulk-write pattern used by `parameter.bulk-write`, and the gotcha that the same parameter name can exist multiple times on one element if it's defined both as a built-in AND as a project parameter.
---

# Revit parameters

Revit parameters are the carrier for almost every piece of business data on an element — height, mark, fire rating, manufacturer, anything. The API has several accessors and several storage types; using the wrong combination silently returns wrong values.

## The four parameter kinds

| Kind | Where defined | Where applied | Identity |
|---|---|---|---|
| **Built-in** | Hardcoded in Revit | Always present on every category that has it | `BuiltInParameter` enum |
| **Project** | Created via Manage > Project Parameters | Bound to one or more categories project-wide | Name (within the project) |
| **Shared** | Defined in an external .txt file; bound as Project Parameter | Same as Project + cross-project consistency | GUID (stable across projects) |
| **Family** | Defined in the .rfa family | Type-parameter OR instance-parameter per FamilyInstance/Symbol | Name (within the family) |

## Type vs instance

EVERY parameter is EITHER:
- **Type parameter** — lives on the `ElementType` (WallType, FamilySymbol). Changing it affects ALL instances.
- **Instance parameter** — lives on the placed `Element` (Wall, FamilyInstance). Changing it affects only that one.

```csharp
var wall = doc.GetElement(wallId) as Wall;

// Instance parameter
var height = wall.LookupParameter("Height");  // varies per wall

// Type parameter (via the WallType)
var wallType = doc.GetElement(wall.GetTypeId()) as WallType;
var thickness = wallType.LookupParameter("Width");  // shared across all walls of this type
```

## The accessor zoo

```csharp
// 1. By BuiltInParameter (preferred — stable, type-safe)
var markParam = element.get_Parameter(BuiltInParameter.ALL_MODEL_MARK);

// 2. By name (fragile — can collide if name re-used as project param)
var customParam = element.LookupParameter("My Custom Param");

// 3. By GUID (shared parameters only)
var sharedGuid = new Guid("...");
var sharedParam = element.get_Parameter(sharedGuid);

// 4. By name, ALL matches (handles the duplicate-name case)
var allMatches = element.GetParameters("Width");  // IList<Parameter>

// 5. Via Parameters collection (iterate everything — useful for discovery)
foreach (Parameter p in element.Parameters) { ... }
```

**Always prefer `BuiltInParameter` over name lookup.** Names are locale-dependent (a French Revit returns "Hauteur" for what en-US calls "Height"); BuiltInParameter is locale-independent.

## StorageType determines the AsXxx selector

```csharp
public enum StorageType { None, Integer, Double, String, ElementId }
```

The right `AsXxx` selector depends on the parameter's `StorageType`:

```csharp
var p = element.get_Parameter(BuiltInParameter.HOST_AREA_COMPUTED);
switch (p.StorageType)
{
    case StorageType.Double:    var area = p.AsDouble(); break;       // raw internal units (sq feet)
    case StorageType.Integer:   var i    = p.AsInteger(); break;
    case StorageType.String:    var s    = p.AsString(); break;
    case StorageType.ElementId: var eid  = p.AsElementId(); break;
    case StorageType.None:      break;
}
```

Calling `AsString()` on a Double parameter returns `null`, NOT an exception. Be defensive:

```csharp
string display = p.AsValueString() ?? p.AsString() ?? "";
```

`AsValueString()` returns the formatted display string (with units and locale formatting). For numeric params, this is usually what you want for human-readable output. For machine-readable round-trip, use `AsDouble() / AsInteger() / AsElementId()`.

## Internal units (the AsDouble gotcha)

Revit stores ALL numeric values in **internal units** (feet for length, square feet for area, etc.) regardless of the project's display units. Always convert when reading or writing:

```csharp
// Read length in mm
double lengthInternal = lengthParam.AsDouble();   // FEET
double lengthMm = UnitUtils.ConvertFromInternalUnits(lengthInternal, UnitTypeId.Millimeters);

// Write length given mm
double newLengthInternal = UnitUtils.ConvertToInternalUnits(2500.0, UnitTypeId.Millimeters);
heightParam.Set(newLengthInternal);
```

`UnitTypeId.<X>` is the Revit 2021+ replacement for the deprecated `DisplayUnitType.DUT_<X>`. The forum literature still references the old enums; use `UnitTypeId` in new code.

## Writing parameters

```csharp
using (var t = new Transaction(doc, "AWARE: write-parameter"))
{
    t.Start();
    var p = element.LookupParameter("Mark");
    p.Set("W-1.A.001");      // String parameters: .Set(string)
    // p.Set(42);             // Integer parameters
    // p.Set(2500.0);          // Double — REMEMBER internal units
    // p.Set(otherElementId);  // ElementId parameters
    t.Commit();
}
```

`Set` returns `true` on success, `false` if the parameter rejects the value (e.g. trying to set a read-only param like an area). Always check:

```csharp
if (!p.Set(value))
    return new { ok = false, error = $"parameter '{p.Definition.Name}' refused the value" };
```

## Read-only parameters

Some built-in parameters are read-only:
- Computed parameters (Area, Volume, Length) — derived from geometry
- System-managed (Family/Symbol/Type — pointers Revit maintains)
- Calculated formulas

Attempting to `Set` these returns `false` (not an exception). Check `param.IsReadOnly` before writing.

## The duplicate-name case

When a project parameter has the same name as a built-in parameter, OR two project parameters share a name across category-bindings, `LookupParameter("X")` returns the FIRST match — which may not be what you want. Use `GetParameters("X")` to list all matches and pick by category / scope.

## Standard bulk-write pattern (the curated `parameter.bulk-write` shape)

```csharp
var doc = uiapp.ActiveUIDocument.Document;
var entries = args["entries"] as IList<object>;    // [{ key:"Mark", value:"W-1" }, ...]

int written = 0;
using (var t = new Transaction(doc, "AWARE: parameter.bulk-write"))
{
    t.Start();
    foreach (var obj in doc.GetSelectedElementIds().Select(id => doc.GetElement(id)))
    {
        foreach (IDictionary<string, object> entry in entries)
        {
            var key   = entry["key"].ToString();
            var value = entry["value"];
            var p = obj.LookupParameter(key);
            if (p == null || p.IsReadOnly) continue;
            switch (p.StorageType)
            {
                case StorageType.String:  if (p.Set(value.ToString())) written++; break;
                case StorageType.Integer: if (p.Set(Convert.ToInt32(value))) written++; break;
                case StorageType.Double:  if (p.Set(Convert.ToDouble(value))) written++; break;
            }
        }
    }
    t.Commit();
}
return new { written = written };
```

## See also

- [revit-element-collector](./revit-element-collector.md) — get the elements first
- [revit-transactions](./revit-transactions.md) — wrapping the write
- [`parameter.bulk-write` curated verb](../commands/parameter.bulk-write.md)
