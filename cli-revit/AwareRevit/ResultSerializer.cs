// JSON-shaping serializer for Revit return values. Defensive: primitives are
// emitted as-is, Element gets {id, category, name, level?}, ElementId becomes
// its integer, XYZ becomes {x,y,z}, Parameter becomes {name,type,value}, and
// unknown types fall back to ToString(). IEnumerable / IDictionary recurse.
// Mirrors the behaviour of cli-tekla's SerializeResult but adds the Revit-
// specific shapes the AI orchestrators reach for most often.

using System.Collections;
using System.Text.Json;
using System.Text.Json.Nodes;
using Autodesk.Revit.DB;

namespace AwareRevit.AddIn;

internal static class ResultSerializer
{
    public static JsonNode? ToJson(object? value)
    {
        if (value is null) return null;
        switch (value)
        {
            case bool b:        return JsonValue.Create(b);
            case int i:         return JsonValue.Create(i);
            case long l:        return JsonValue.Create(l);
            case double d:      return JsonValue.Create(d);
            case float f:       return JsonValue.Create((double)f);
            case decimal m:     return JsonValue.Create(m);
            case string s:      return JsonValue.Create(s);
            case Guid g:        return JsonValue.Create(g.ToString());
            case DateTime dt:   return JsonValue.Create(dt.ToString("o"));
            case JsonNode jn:   return jn;
        }

        // Revit-specific shapes.
        if (value is ElementId eid)
            return JsonValue.Create(eid.Value);

        if (value is XYZ xyz)
            return new JsonObject { ["x"] = xyz.X, ["y"] = xyz.Y, ["z"] = xyz.Z };

        if (value is Parameter p)
            return new JsonObject
            {
                ["name"]  = p.Definition?.Name,
                ["type"]  = p.StorageType.ToString(),
                ["value"] = ParameterValue(p),
            };

        if (value is Element e)
            return new JsonObject
            {
                ["id"]       = e.Id?.Value,
                ["category"] = e.Category?.Name,
                ["name"]     = e.Name,
            };

        if (value is IDictionary id)
        {
            var jo = new JsonObject();
            foreach (DictionaryEntry kvp in id)
                jo[kvp.Key?.ToString() ?? ""] = ToJson(kvp.Value);
            return jo;
        }

        if (value is IEnumerable enumerable and not string)
        {
            var ja = new JsonArray();
            foreach (var item in enumerable) ja.Add(ToJson(item));
            return ja;
        }

        // Anonymous types / records / POCOs → System.Text.Json with cycle protection.
        try
        {
            var json = JsonSerializer.Serialize(value, new JsonSerializerOptions
            {
                MaxDepth = 6,
                ReferenceHandler = System.Text.Json.Serialization.ReferenceHandler.IgnoreCycles,
            });
            return JsonNode.Parse(json);
        }
        catch
        {
            return JsonValue.Create($"{value.GetType().FullName}: {value}");
        }
    }

    static JsonNode? ParameterValue(Parameter p)
    {
        return p.StorageType switch
        {
            StorageType.Integer  => JsonValue.Create(p.AsInteger()),
            StorageType.Double   => JsonValue.Create(p.AsDouble()),
            StorageType.String   => JsonValue.Create(p.AsString() ?? ""),
            StorageType.ElementId => JsonValue.Create(p.AsElementId()?.Value),
            _ => JsonValue.Create(p.AsValueString() ?? ""),
        };
    }
}
