namespace FixtureAssembly.Demo;

/// <summary>A demo class with a few public methods, used by sidecar reflection tests.</summary>
public class Greeter
{
    /// <summary>Returns a greeting for the given name.</summary>
    public string Greet(string name) => $"Hello, {name}";

    /// <summary>Counts the number of times a substring appears.</summary>
    public static int CountOccurrences(string haystack, string needle)
    {
        if (string.IsNullOrEmpty(needle)) return 0;
        int count = 0, idx = 0;
        while ((idx = haystack.IndexOf(needle, idx, StringComparison.Ordinal)) != -1)
        {
            count++;
            idx += needle.Length;
        }
        return count;
    }
}

/// <summary>Marker interface for objects that can be serialized.</summary>
public interface ISerializable
{
    /// <summary>Serializes the object to a string.</summary>
    string Serialize();
}
