// Test-only. Loads cli/src/render/scene_member_labels.cases.json, the member-label cases the IFC
// writer's own test (cli/src/render/ifc.rs) runs, so each CAD-host sink's tests can prove that it
// accepts and refuses exactly the members `ifc write` does, with the same message (#818).
// Linked into every host's test project; fully qualified so it compiles on net48 and net8/net10.
#nullable disable

/// <summary>One shared case: the member's whole `meta` and what every sink must make of it.</summary>
internal sealed class AwareMemberLabelCase
{
    internal string Name;

    /// <summary>Member B1's full `meta`: `profile` first, then the case's labels, in document order.</summary>
    internal System.Text.Json.Nodes.JsonObject Meta;

    /// <summary>The expected refusal (without `ifc write: `), or null when the member is accepted.</summary>
    internal string Refusal;

    internal string Description;
    internal string PropertySet;
    internal System.Collections.Generic.List<System.Collections.Generic.KeyValuePair<string, string>> Properties =
        new System.Collections.Generic.List<System.Collections.Generic.KeyValuePair<string, string>>();

    public override string ToString() { return Name; }
}

internal static class AwareMemberLabelCases
{
    internal const string MemberId = "B1";
    internal const string Profile = "W16X26";
    const string RelativePath = "cli/src/render/scene_member_labels.cases.json";

    /// <summary>The fixture, found by walking up from the test binaries to the repository root.</summary>
    internal static string FixturePath()
    {
        var dir = new System.IO.DirectoryInfo(System.AppContext.BaseDirectory);
        while (dir != null)
        {
            var candidate = System.IO.Path.Combine(dir.FullName, RelativePath.Replace('/', System.IO.Path.DirectorySeparatorChar));
            if (System.IO.File.Exists(candidate)) return candidate;
            dir = dir.Parent;
        }
        throw new System.IO.FileNotFoundException("the shared member-label cases were not found above " + System.AppContext.BaseDirectory, RelativePath);
    }

    internal static System.Collections.Generic.List<AwareMemberLabelCase> Load()
    {
        var root = System.Text.Json.Nodes.JsonNode.Parse(System.IO.File.ReadAllText(FixturePath())).AsObject();
        if ((string)root["member"] != MemberId) throw new System.InvalidOperationException("the shared cases name another member");
        var cases = new System.Collections.Generic.List<AwareMemberLabelCase>();
        foreach (var node in root["cases"].AsArray())
        {
            var c = node.AsObject();
            var meta = new System.Text.Json.Nodes.JsonObject { ["profile"] = Profile };
            foreach (var field in c["meta"].AsObject())
                meta[field.Key] = field.Value == null ? null : field.Value.DeepClone();
            var item = new AwareMemberLabelCase
            {
                Name = (string)c["name"],
                Meta = meta,
                Refusal = c["refusal"] == null ? null : (string)c["refusal"],
            };
            var labels = c["labels"] as System.Text.Json.Nodes.JsonObject;
            if (labels != null)
            {
                item.Description = labels["description"] == null ? null : (string)labels["description"];
                item.PropertySet = labels["propertySet"] == null ? null : (string)labels["propertySet"];
                var properties = labels["properties"] as System.Text.Json.Nodes.JsonArray;
                if (properties != null)
                    foreach (var pair in properties)
                        item.Properties.Add(new System.Collections.Generic.KeyValuePair<string, string>((string)pair[0], (string)pair[1]));
            }
            cases.Add(item);
        }
        if (cases.Count < 40) throw new System.InvalidOperationException("the shared member-label cases shrank to " + cases.Count);
        return cases;
    }

    /// <summary>Null when a sink's verdict on <paramref name="c"/> is the expected one, else what differs.</summary>
    internal static string Mismatch(
        AwareMemberLabelCase c,
        string refusal,
        string description,
        string propertySet,
        System.Collections.Generic.IList<System.Collections.Generic.KeyValuePair<string, string>> properties)
    {
        if (c.Refusal != null)
        {
            if (refusal != c.Refusal) return c.Name + ": expected the refusal\n  " + c.Refusal + "\nbut got\n  " + (refusal ?? "(accepted)");
            return null;
        }
        if (refusal != null) return c.Name + ": expected acceptance but got the refusal\n  " + refusal;
        if (description != c.Description) return c.Name + ": description `" + description + "`, expected `" + c.Description + "`";
        if (propertySet != c.PropertySet) return c.Name + ": property set `" + propertySet + "`, expected `" + c.PropertySet + "`";
        var got = properties ?? new System.Collections.Generic.List<System.Collections.Generic.KeyValuePair<string, string>>();
        if (got.Count != c.Properties.Count) return c.Name + ": " + got.Count + " properties, expected " + c.Properties.Count;
        for (var i = 0; i < got.Count; i++)
        {
            if (got[i].Key != c.Properties[i].Key || got[i].Value != c.Properties[i].Value)
                return c.Name + ": property " + i + " is `" + got[i].Key + "`, expected `" + c.Properties[i].Key + "` (in key order, with its value)";
        }
        return null;
    }
}
