// DocId — small helpers for working with .NET XML-documentation member ids.
//
// Extracted into AwareReader so the metadata reader (and, later, the synthesizer)
// can shorten fully-qualified names without depending on the sidecar's XmlDocLoader
// (which stays in cli-sidecar with the vendor-extractor pipeline). The bodies match
// XmlDocLoader.ShortName / XmlDocLoader.StripDocPrefix exactly; both are pure string
// ops with no reflection, so they are AOT/trim-clean.

namespace AwareReader;

public static class DocId
{
    /// <summary>
    /// Returns the short name of a fully-qualified type reference. "System.String" → "String".
    /// Generic suffixes like "`1" and parameter lists in parens are preserved (the caller
    /// renders them as needed).
    /// </summary>
    public static string ShortName(string fqName)
    {
        var dot = fqName.LastIndexOf('.');
        return dot < 0 ? fqName : fqName[(dot + 1)..];
    }

    /// <summary>
    /// Strips the leading kind prefix from a doc id ("T:Foo" → "Foo", "M:Foo.Bar(X)" → "Foo.Bar(X)").
    /// </summary>
    public static string StripDocPrefix(string id)
    {
        if (id.Length < 3) return id;
        return id[1] == ':' ? id[2..] : id;
    }
}
