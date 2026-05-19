// XmlDocLoader — parses a Sandcastle / sndocgen-style .NET XML documentation
// file into a structured lookup keyed by the canonical member id.
//
// XML doc id format (per the C# spec):
//   T:Namespace.TypeName                                 — type
//   M:Namespace.TypeName.MethodName(ParamType1,ParamType2,…)   — method
//   M:Namespace.TypeName.#ctor(ParamType1,…)             — constructor
//   M:Namespace.TypeName.#cctor                          — static ctor
//   M:Namespace.TypeName.op_Equality(LHS,RHS)            — operator overload
//   M:Namespace.TypeName.op_Implicit(SrcType)~DstType    — conversion op
//   P:Namespace.TypeName.PropertyName                    — property
//   P:Namespace.TypeName.Item(IndexType)                 — indexer
//   E:Namespace.TypeName.EventName                       — event
//   F:Namespace.TypeName.FieldName                       — field / enum value
//
// Members carry one or more documentation children:
//   <summary>     — short one-liner; collapsed to a single line in the IR.
//   <remarks>     — paragraphs; preserved as-is (newlines retained).
//   <param name=> — per-parameter documentation; keyed by param name.
//   <returns>     — return-value documentation (only present on methods that return).
//   <exception cref=>  — exception type the method may throw, with description.
//   <value>       — property value documentation (used by some authors instead of <summary>).
//
// This loader yields a `MemberDoc` per `<member>` element. The Extractor pairs
// each MemberDoc against the corresponding metadata reader handle so signatures
// can be reconstructed (XML doc ids don't include parameter names or types in a
// way that's directly usable as a method signature).
//
// AOT: `XDocument.Load` does not use reflection on user types. Fully AOT-clean.
//
// Encoding: XmlReaderSettings.Default — reads the BOM + XML declaration. Autodesk
// emits UTF-8 with BOM; encoding=utf-8 in the XML declaration. No conversion
// required.
//
// Robustness:
// - Malformed XML — XDocument throws XmlException; the caller decides whether to
//   surface that as a hard error or skip the entire doc.
// - Empty member — kept in the dictionary with all-null fields so callers can
//   distinguish "documented with empty content" from "completely undocumented".

using System.Xml.Linq;

namespace AwareSidecar.Ingest.Extractors.Common;

/// <summary>
/// Doc data for one XML doc `<member>` entry. All fields are nullable; absent
/// elements stay null rather than empty string so callers can distinguish
/// "no &lt;remarks&gt; tag" from "&lt;remarks/&gt;".
/// </summary>
public sealed record MemberDoc(
    string Id,
    string? Summary,
    string? Remarks,
    string? Returns,
    string? Value,
    IReadOnlyDictionary<string, string> Params,
    IReadOnlyList<ExceptionDoc> Exceptions);

public sealed record ExceptionDoc(string TypeRef, string Description);

public static class XmlDocLoader
{
    /// <summary>
    /// Loads the entire XML doc into a dictionary keyed by canonical id (T:Foo,
    /// M:Foo.Bar, P:Foo.Baz, E:Foo.Quux, F:Foo.Glorp).
    /// </summary>
    public static Dictionary<string, MemberDoc> LoadFromFile(string xmlPath)
    {
        if (!File.Exists(xmlPath))
            throw new FileNotFoundException($"XML doc not found: {xmlPath}");

        var doc = XDocument.Load(xmlPath, LoadOptions.PreserveWhitespace);
        var members = doc.Descendants("member");
        var dict = new Dictionary<string, MemberDoc>(StringComparer.Ordinal);

        foreach (var m in members)
        {
            var id = (string?)m.Attribute("name");
            if (string.IsNullOrEmpty(id)) continue;

            // Per-param dictionary (allows two <param name="x"> children to collide;
            // the last wins, which matches Sandcastle's tolerant behaviour).
            var paramDict = new Dictionary<string, string>(StringComparer.Ordinal);
            foreach (var p in m.Elements("param"))
            {
                var pname = (string?)p.Attribute("name");
                if (string.IsNullOrEmpty(pname)) continue;
                paramDict[pname] = NormaliseText(p);
            }

            var exceptions = new List<ExceptionDoc>();
            foreach (var ex in m.Elements("exception"))
            {
                var cref = (string?)ex.Attribute("cref") ?? "";
                exceptions.Add(new ExceptionDoc(
                    TypeRef: StripDocPrefix(cref),
                    Description: NormaliseText(ex)));
            }

            dict[id] = new MemberDoc(
                Id: id,
                Summary: CollapseToLine(m.Element("summary")),
                Remarks: NormaliseTextOrNull(m.Element("remarks")),
                Returns: NormaliseTextOrNull(m.Element("returns")),
                Value:   NormaliseTextOrNull(m.Element("value")),
                Params: paramDict,
                Exceptions: exceptions);
        }
        return dict;
    }

    /// <summary>
    /// Returns plain text of the element with inline cref / paramref / typeparamref
    /// resolved to a readable form. Whitespace is collapsed, but newlines are
    /// preserved (callers can flatten further if needed).
    /// </summary>
    static string NormaliseText(XElement element)
    {
        var sb = new System.Text.StringBuilder();
        foreach (var node in element.Nodes())
        {
            switch (node)
            {
                case XText t: sb.Append(t.Value); break;
                case XElement child: sb.Append(RenderInlineChild(child)); break;
            }
        }
        // Collapse runs of spaces but keep line breaks.
        var lines = sb.ToString().Split('\n');
        for (int i = 0; i < lines.Length; i++)
        {
            // Collapse internal whitespace on each line to single spaces.
            lines[i] = string.Join(' ',
                lines[i].Split(new[] { ' ', '\t', '\r' }, StringSplitOptions.RemoveEmptyEntries));
        }
        return string.Join('\n', lines).Trim();
    }

    static string? NormaliseTextOrNull(XElement? element)
    {
        if (element is null) return null;
        var s = NormaliseText(element);
        return string.IsNullOrEmpty(s) ? null : s;
    }

    /// <summary>
    /// Like NormaliseText, but for one-liner contexts (summary). Joins all lines
    /// with single spaces.
    /// </summary>
    static string? CollapseToLine(XElement? element)
    {
        if (element is null) return null;
        var s = NormaliseText(element).Replace('\n', ' ').Replace('\r', ' ');
        s = string.Join(' ',
            s.Split(new[] { ' ', '\t' }, StringSplitOptions.RemoveEmptyEntries));
        return string.IsNullOrEmpty(s) ? null : s;
    }

    /// <summary>
    /// Renders an inline doc child element to text. We don't preserve structure —
    /// downstream consumers only need readable summaries. <see cref="X"> becomes
    /// the short name of X (the part after the last dot).
    /// </summary>
    static string RenderInlineChild(XElement child)
    {
        return child.Name.LocalName switch
        {
            "see" or "seealso" => RenderCref(child),
            "paramref"         => (string?)child.Attribute("name") ?? "",
            "typeparamref"     => (string?)child.Attribute("name") ?? "",
            "c"                => child.Value,
            "code"             => child.Value,
            "para"             => "\n" + NormaliseText(child) + "\n",
            "list"             => RenderList(child),
            "item"             => "- " + NormaliseText(child) + "\n",
            "description"      => NormaliseText(child),
            "term"             => NormaliseText(child),
            _                  => NormaliseText(child),
        };
    }

    static string RenderCref(XElement seeElement)
    {
        var cref = (string?)seeElement.Attribute("cref");
        var langword = (string?)seeElement.Attribute("langword");
        var href = (string?)seeElement.Attribute("href");
        if (!string.IsNullOrEmpty(langword)) return langword!;
        if (!string.IsNullOrEmpty(cref))
        {
            var stripped = StripDocPrefix(cref);
            var inner = seeElement.Value?.Trim();
            return !string.IsNullOrEmpty(inner) ? inner : ShortName(stripped);
        }
        if (!string.IsNullOrEmpty(href)) return href!;
        return seeElement.Value;
    }

    static string RenderList(XElement listElement)
    {
        var sb = new System.Text.StringBuilder();
        foreach (var item in listElement.Elements("item"))
        {
            sb.Append("- ");
            sb.Append(NormaliseText(item));
            sb.Append('\n');
        }
        return sb.ToString();
    }

    /// <summary>
    /// Strips the leading kind prefix from a doc id ("T:Foo" → "Foo", "M:Foo.Bar(X)" → "Foo.Bar(X)").
    /// </summary>
    public static string StripDocPrefix(string id)
    {
        if (id.Length < 3) return id;
        return id[1] == ':' ? id[2..] : id;
    }

    /// <summary>
    /// Returns the short name of a fully-qualified type reference. "System.String" → "String".
    /// Generic suffixes like "`1" and parameter lists in parens are preserved (the caller
    /// renders them as needed).
    /// </summary>
    public static string ShortName(string fqName)
    {
        // Strip namespace, but keep generic suffix and any parameter list.
        var dot = fqName.LastIndexOf('.');
        return dot < 0 ? fqName : fqName[(dot + 1)..];
    }
}
