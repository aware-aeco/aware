// Sidecar-side mirror of AwareRevit/ResultSerializer.cs covering the
// non-Revit code paths (primitives, IDictionary, IEnumerable, anonymous
// types). Keeps the shape under test without dragging RevitAPI into the
// test project. The add-in adds Revit-specific shapes on top.

using System.Collections;
using System.Text.Json;
using System.Text.Json.Nodes;

namespace AwareRevit.Sidecar;

internal static class ResultSerializerCore
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
}
