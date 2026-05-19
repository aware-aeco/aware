// XmlDocReader — parses Roslyn-style XML documentation files (the kind emitted by
// `csc /doc:`) and exposes per-member docs by their canonical XML-doc ID.
//
// XML-doc ID format:
//   T:Namespace.Type                       — types
//   M:Namespace.Type.Method                — parameterless method or ctor with no args
//   M:Namespace.Type.Method(P1,P2,…)       — method with parameters (canonical types,
//                                             dotted, e.g. System.String)
//   M:Namespace.Type.#ctor                 — parameterless constructor
//   M:Namespace.Type.#ctor(P1,P2)          — parameterized constructor
//   P:Namespace.Type.PropertyName          — property
//   F:Namespace.Type.FieldName             — field (incl. enum members)
//   E:Namespace.Type.EventName             — event
//
// Generic types use backtick-arity in the ID: `Dictionary``2.Add(``0,``1)` represents
// `Dictionary<TKey,TValue>.Add(TKey, TValue)`. Method-level generics use double-backtick.
//
// We intentionally use System.Xml.XmlReader (forward-only, no DOM) so the AOT publish
// stays trim-clean — XDocument's LINQ-to-XML pulls in a wider surface.
//
// What we extract per member:
//   - summary (paragraph-joined plain text)
//   - remarks (paragraph-joined plain text)
//   - param[name] → doc text          (per-parameter doc)
//   - returns → doc text
//   - exception[cref] → doc text       (mapped to ThrowsInfo)
//
// Inline element handling:
//   - <see cref="..."/> → "Foo.Bar" or fully-qualified depending on cref form
//   - <paramref name="x"/> → "x"
//   - <typeparamref name="T"/> → "T"
//   - <c>code</c> → bare "code" text
//   - <para> paragraphs → "\n\n" separator
//   - <list> → flatten to bullet-style "- item; - item"
//
// Why this matters:
//   The IR consumer (codex-coverage, the generator) renders these as natural-language
//   docs in skills and catalog files. Keeping inline elements out of the rendered text
//   prevents `<see cref="T:Foo.Bar"/>` showing up verbatim inside markdown.

using System.Xml;

namespace AwareSidecar.Ingest.Extractors.Dynamo;

public sealed class XmlDocReader
{
    public sealed record MemberDoc(
        string? Summary,
        string? Remarks,
        Dictionary<string, string> Params,
        string? ReturnsDoc,
        List<(string Cref, string Doc)> Exceptions);

    /// <summary>
    /// Map of XML-doc-id → MemberDoc, e.g. `M:Dynamo.Engine.EngineController.RunExpression(System.String)`.
    /// </summary>
    public IReadOnlyDictionary<string, MemberDoc> Members => _members;

    readonly Dictionary<string, MemberDoc> _members = new(StringComparer.Ordinal);

    public static XmlDocReader Load(string path)
    {
        var reader = new XmlDocReader();
        reader.LoadFrom(path);
        return reader;
    }

    public static XmlDocReader Parse(string xml)
    {
        var reader = new XmlDocReader();
        using var sr = new System.IO.StringReader(xml);
        using var xr = XmlReader.Create(sr, new XmlReaderSettings
        {
            DtdProcessing = DtdProcessing.Prohibit,
            IgnoreWhitespace = false,
            CloseInput = false
        });
        reader.ReadDocument(xr);
        return reader;
    }

    void LoadFrom(string path)
    {
        using var fs = File.OpenRead(path);
        using var xr = XmlReader.Create(fs, new XmlReaderSettings
        {
            DtdProcessing = DtdProcessing.Prohibit,
            IgnoreWhitespace = false
        });
        ReadDocument(xr);
    }

    void ReadDocument(XmlReader xr)
    {
        // Stream: walk to <members>, then iterate <member name="..."> children, read inner
        // content per element kind. This is forward-only — we never seek back.
        while (xr.Read())
        {
            if (xr.NodeType != XmlNodeType.Element) continue;
            if (xr.Name != "member") continue;

            var memberId = xr.GetAttribute("name");
            if (string.IsNullOrEmpty(memberId)) { xr.Skip(); continue; }

            string? summary = null;
            string? remarks = null;
            string? returnsDoc = null;
            var paramDocs = new Dictionary<string, string>(StringComparer.Ordinal);
            var exceptions = new List<(string, string)>();

            // Read all <member> children. We pre-read the full member contents into an XML
            // fragment so we can use a straightforward two-pass approach: first build a tiny
            // DOM-like structure, then extract each known child element. This avoids the
            // XmlReader.ReadInnerXml+Read state-machine ambiguity (where Read advances past
            // the next sibling after ReadInnerXml consumed the prior element's end-tag).
            string memberInnerXml;
            try { memberInnerXml = xr.ReadInnerXml(); }
            catch
            {
                // Malformed member XML — skip the member, advance to the next.
                continue;
            }
            ParseMemberChildren(memberInnerXml, out summary, out remarks, out returnsDoc, paramDocs, exceptions);

            _members[memberId] = new MemberDoc(summary, remarks, paramDocs, returnsDoc, exceptions);
        }
    }

    /// <summary>
    /// Parse the inner XML of a <member> element and extract its known children. Uses a DOM-style
    /// pre-load (XmlDocument) instead of streaming so we don't have to track the XmlReader's
    /// post-ReadInnerXml state. The fragment is small (a single XML doc comment, typically &lt;1KB)
    /// so the DOM overhead is negligible.
    /// </summary>
    static void ParseMemberChildren(string innerXml, out string? summary, out string? remarks,
        out string? returnsDoc, Dictionary<string, string> paramDocs, List<(string, string)> exceptions)
    {
        summary = null; remarks = null; returnsDoc = null;
        if (string.IsNullOrWhiteSpace(innerXml)) return;
        var wrapped = "<m>" + innerXml + "</m>";
        try
        {
            var doc = new System.Xml.XmlDocument();
            doc.LoadXml(wrapped);
            if (doc.DocumentElement is null) return;
            foreach (System.Xml.XmlNode child in doc.DocumentElement.ChildNodes)
            {
                if (child.NodeType != XmlNodeType.Element) continue;
                var elementName = child.LocalName;
                var inner = child.InnerXml;
                switch (elementName)
                {
                    case "summary":
                        summary = NormalizeInline(inner);
                        break;
                    case "remarks":
                        remarks = NormalizeInline(inner);
                        break;
                    case "returns":
                        returnsDoc = NormalizeInline(inner);
                        break;
                    case "param":
                    {
                        var name = child.Attributes?["name"]?.Value ?? "";
                        var text = NormalizeInline(inner);
                        if (!string.IsNullOrEmpty(name)) paramDocs[name] = text;
                        break;
                    }
                    case "exception":
                    {
                        var cref = child.Attributes?["cref"]?.Value ?? "";
                        var text = NormalizeInline(inner);
                        var crefClean = cref.StartsWith("T:", StringComparison.Ordinal) ? cref[2..] : cref;
                        exceptions.Add((crefClean, text));
                        break;
                    }
                }
            }
        }
        catch
        {
            // Malformed XML — leave whatever was already extracted.
        }
    }

    /// <summary>
    /// Strip the inline doc tags (<see>, <paramref>, <c>, <para>, …) and collapse whitespace.
    /// The input is an XML fragment, so we re-parse it to walk text nodes — this is the simplest
    /// way to handle nested tags without writing a tokenizer.
    /// </summary>
    internal static string NormalizeInline(string xmlFragment)
    {
        if (string.IsNullOrWhiteSpace(xmlFragment)) return "";

        // Wrap so the fragment is parseable as a single rooted document.
        var wrapped = "<root>" + xmlFragment + "</root>";

        var sb = new System.Text.StringBuilder();
        try
        {
            using var sr = new System.IO.StringReader(wrapped);
            using var xr = XmlReader.Create(sr, new XmlReaderSettings
            {
                DtdProcessing = DtdProcessing.Prohibit,
                IgnoreWhitespace = false
            });
            while (xr.Read())
            {
                switch (xr.NodeType)
                {
                    case XmlNodeType.Text:
                    case XmlNodeType.CDATA:
                    case XmlNodeType.SignificantWhitespace:
                        sb.Append(xr.Value);
                        break;
                    case XmlNodeType.Whitespace:
                        sb.Append(' ');
                        break;
                    case XmlNodeType.Element:
                        switch (xr.Name)
                        {
                            case "see":
                            case "seealso":
                                // Render cref / langword as plain text.
                                var cref = xr.GetAttribute("cref");
                                var langword = xr.GetAttribute("langword");
                                var href = xr.GetAttribute("href");
                                if (!string.IsNullOrEmpty(cref))
                                {
                                    sb.Append(StripCrefPrefix(cref));
                                }
                                else if (!string.IsNullOrEmpty(langword))
                                {
                                    sb.Append(langword);
                                }
                                else if (!string.IsNullOrEmpty(href))
                                {
                                    sb.Append(href);
                                }
                                if (xr.IsEmptyElement) continue;
                                // Otherwise inner text will be picked up by the text walker.
                                break;
                            case "paramref":
                            case "typeparamref":
                                var name = xr.GetAttribute("name");
                                if (!string.IsNullOrEmpty(name)) sb.Append(name);
                                if (xr.IsEmptyElement) continue;
                                break;
                            case "para":
                                if (sb.Length > 0 && sb[sb.Length - 1] != ' ' && sb[sb.Length - 1] != '\n')
                                    sb.Append(' ');
                                break;
                            case "c":
                            case "code":
                                // Inline code: keep the inner text but no markers.
                                break;
                            case "list":
                            case "item":
                            case "description":
                            case "term":
                                // Flatten — we just emit text content with a separator.
                                if (xr.Name == "item") sb.Append("; ");
                                break;
                            default:
                                break;
                        }
                        break;
                    case XmlNodeType.EndElement:
                        if (xr.Name == "para") sb.Append(' ');
                        break;
                }
            }
        }
        catch
        {
            // If a docstring is malformed XML, fall back to a regex strip.
            return CollapseWhitespace(System.Text.RegularExpressions.Regex.Replace(xmlFragment, @"<[^>]+>", ""));
        }
        return CollapseWhitespace(sb.ToString());
    }

    static string StripCrefPrefix(string cref)
    {
        // cref shapes: "T:Foo.Bar", "M:Foo.Bar.Baz(...)", "P:Foo.Bar.X", etc. Strip the prefix.
        if (cref.Length >= 2 && cref[1] == ':') return cref[2..];
        return cref;
    }

    static string CollapseWhitespace(string s)
    {
        if (string.IsNullOrEmpty(s)) return "";
        var sb = new System.Text.StringBuilder(s.Length);
        bool lastWasSpace = false;
        foreach (var c in s)
        {
            if (char.IsWhiteSpace(c))
            {
                if (!lastWasSpace) sb.Append(' ');
                lastWasSpace = true;
            }
            else
            {
                sb.Append(c);
                lastWasSpace = false;
            }
        }
        return sb.ToString().Trim();
    }
}
