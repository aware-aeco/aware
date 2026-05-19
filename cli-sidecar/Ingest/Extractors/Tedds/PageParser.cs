// PageParser — parse a single Tekla Tedds developer.tekla.com type-documentation
// page into a TypeInfo plus the URL bundle for each member that needs a follow-up
// per-member fetch.
//
// TSD/Tedds docs site shape (verified 2026-05-19 against developer.tekla.com):
//
//   Tekla Tedds uses a DocFX-style doc generator (different from Tekla Structures'
//   Sandcastle markup). The shape is:
//
//   • The content body lives inside  <article class="content wrap" id="_content" data-uid="<full-name>">.
//   • The page title is <h1 id="..." data-uid="...">Class Foo | Interface IFoo | Enum FooEnum | Struct FooStruct | Delegate FooDelegate</h1>
//     (note: kind word comes FIRST in TSD, opposite of Tekla Structures).
//   • The summary is in  <div class="markdown level0 summary"><p>...</p></div>  directly after the h1.
//   • The Namespace + Assembly lines are <h6> elements:
//       <h6><strong>Namespace</strong>: <a>Tekla</a>.<a>Structural</a>.<a>InteropAssemblies</a>.<a>Tedds</a></h6>
//       <h6><strong>Assembly</strong>: Tedds.TeddsIA.dll</h6>
//   • The C# syntax block:
//       <h5 id="..._syntax">Syntax</h5>
//       <div class="codewrapper"><pre><code class="language-cs">...</code></pre></div>
//   • Inheritance (classes only) is in <div class="inheritance">:
//       <div class="inheritance">
//         <h5>Inheritance</h5>
//         <div class="level0"><a>object</a></div>
//         <div class="level1"><a>Attribute</a></div>
//         <div class="level2"><span class="xref">FooAttribute</span></div>     ← self
//       </div>
//   • Member section headings: <h3 id="constructors">, <h3 id="properties">, <h3 id="methods">,
//     <h3 id="fields"> (enum values), <h3 id="events">.
//   • Each member section is followed by  <table class="table table-bordered table-condensed">
//     with rows like:
//       <tr>
//         <td id="<DocFX_anchor>" data-uid="<canonical>">
//           <a class="xref" href="/topic/en/20/45/<hash>">MemberName(arg, arg)</a>
//         </td>
//         <td class="markdown level1 summary"><p>doc</p></td>
//       </tr>
//   • The xref `/topic/en/...` hash URL 302-redirects to the canonical per-member page
//     at  /doc/tekla-tedds/<ver>/<member-slug>-<id>. We resolve via HttpClient redirect.
//
// Each member-table row carries an <a> pointing to the per-member detail page.
// PageParser captures the member name + URL; MemberPageParser fetches the per-member
// page to get real signatures/types/delegates.
//
// AOT note: pure managed parsing, identical AOT story to Tekla.PageParser.

using AngleSharp.Dom;
using AngleSharp.Html.Parser;
using System.Text;
using System.Text.Json;
using System.Text.RegularExpressions;

namespace AwareSidecar.Ingest.Extractors.Tedds;

public static class PageParser
{
    static readonly HtmlParser _parser = new();

    /// <summary>
    /// URL bundle returned alongside a parsed TypeInfo: each list pairs a member with the
    /// per-member detail page URL. Index alignment is intentional:
    /// result.Type.methods[i] pairs with links.Methods[i].
    /// </summary>
    public sealed record MemberLinks(
        IReadOnlyList<string> Constructors,
        IReadOnlyList<string> Methods,
        IReadOnlyList<string> Properties,
        IReadOnlyList<string> Events);

    public sealed record ParseResult(TypeInfo Type, MemberLinks Links);

    const string BaseHost = "https://developer.tekla.com";

    /// <summary>
    /// Parse one Tekla Tedds type-documentation page. Returns null if the page is not a
    /// recognized type page.
    /// </summary>
    public static ParseResult? Parse(string html, string sourceUrl)
    {
        using var doc = _parser.ParseDocument(html);
        var content = doc.QuerySelector("article.content.wrap#_content");
        if (content is null) return null;

        var h1 = content.QuerySelector("h1");
        if (h1 is null) return null;
        var title = NormaliseWhitespace(CleanInlineText(h1));
        var (name, kind) = ParseTitle(title);
        if (kind is null) return null;

        // data-uid on the h1 carries the full namespace-qualified name. We prefer the
        // <h6><strong>Namespace</strong>: ...</h6> trail for namespace because it's
        // structured and reliable; data-uid is the cross-check.
        var ns = ExtractNamespace(content);
        if (string.IsNullOrEmpty(ns))
        {
            // Fallback to data-uid: "Tekla.Structural.InteropAssemblies.Tedds.IApplication"
            // → "Tekla.Structural.InteropAssemblies.Tedds" (strip last segment).
            var uid = h1.GetAttribute("data-uid") ?? "";
            var lastDot = uid.LastIndexOf('.');
            ns = lastDot > 0 ? uid.Substring(0, lastDot) : "";
        }
        if (string.IsNullOrEmpty(ns)) return null;

        var summary = ExtractSummary(content);
        if (string.IsNullOrEmpty(summary))
            summary = $"(No description provided in vendor docs for {ns}.{name}.)";

        // Type-level remarks — TSD pages may emit them as <div class="markdown level0 conceptual"><p>...</p></div>
        // directly after the summary, or as a Remarks section. Conservative: try the conceptual block.
        var remarks = ExtractTypeRemarks(content);

        // Inheritance: <div class="inheritance">'s level<N-1> entry (the second-to-last) is the
        // immediate base. The level<N> entry is self.
        var (baseTypeName, ifacesFromInheritance) = ExtractInheritanceBlock(content);

        // C# syntax declaration may reveal implemented interfaces for class/struct, e.g.
        //   public sealed class AliasAttribute : Attribute
        // Tekla Tedds rarely emits interface lists (most types implement nothing besides
        // their declared base). We parse anyway for robustness.
        var (baseFromSyntax, ifacesFromSyntax) = ExtractInheritanceFromSyntax(content, kind);
        var finalBase = !string.IsNullOrEmpty(baseTypeName) ? baseTypeName : baseFromSyntax;
        var interfaces = new List<string>(ifacesFromSyntax);
        foreach (var i in ifacesFromInheritance) if (!interfaces.Contains(i)) interfaces.Add(i);

        // Member tables — TSD wraps each section as <h3 id="X">…</h3><table class="table…">.
        var ctorRes = ExtractMethodTable(content, "constructors", name, sourceUrl, isCtor: true);
        var methRes = ExtractMethodTable(content, "methods",      name, sourceUrl, isCtor: false);
        var propRes = ExtractPropertyTable(content, "properties", sourceUrl);
        var evtRes  = ExtractEventTable(content, "events",        sourceUrl);

        // For enums, "Fields" section holds the value rows.
        var enumValues = kind == "enum"
            ? ExtractEnumValues(content)
            : new List<EnumValueInfo>();

        // Delegate signature — TSD doesn't have many delegates but parse the syntax block when present.
        string? delegateSig = kind == "delegate" ? ExtractCSharpSyntax(content) : null;

        var typeInfo = new TypeInfo(
            name: name,
            @namespace: ns,
            kind: kind,
            summary: NormaliseWhitespace(summary),
            remarks: remarks,
            @base: finalBase,
            interfaces: interfaces,
            doc_url: sourceUrl,
            constructors: ctorRes.Members,
            methods: methRes.Members,
            properties: propRes.Members,
            events: evtRes.Members,
            enum_values: enumValues,
            delegate_signature: delegateSig);

        var links = new MemberLinks(
            Constructors: ctorRes.Urls,
            Methods: methRes.Urls,
            Properties: propRes.Urls,
            Events: evtRes.Urls);

        return new ParseResult(typeInfo, links);
    }

    // ── title parsing ────────────────────────────────────────────────────────

    /// <summary>
    /// TSD titles read "Class Foo", "Interface IFoo", "Enum FooEnum", "Struct FooStruct",
    /// "Delegate FooDelegate" — kind keyword comes FIRST, opposite of Tekla Structures'
    /// "Foo Class" trailing-word convention.
    /// </summary>
    static (string name, string? kind) ParseTitle(string title)
    {
        var t = Regex.Replace(title.Trim(), @"\s+", " ");
        if (t.StartsWith("Class ", StringComparison.Ordinal))
            return (t.Substring("Class ".Length).Trim(), "class");
        if (t.StartsWith("Struct ", StringComparison.Ordinal))
            return (t.Substring("Struct ".Length).Trim(), "struct");
        if (t.StartsWith("Interface ", StringComparison.Ordinal))
            return (t.Substring("Interface ".Length).Trim(), "interface");
        if (t.StartsWith("Enum ", StringComparison.Ordinal))
            return (t.Substring("Enum ".Length).Trim(), "enum");
        if (t.StartsWith("Delegate ", StringComparison.Ordinal))
            return (t.Substring("Delegate ".Length).Trim(), "delegate");
        return (t, null);
    }

    // ── summary extraction ─────────────────────────────────────────────────

    /// <summary>
    /// TSD summary is in the first <div class="markdown level0 summary"> within the article.
    /// Empty summary divs are common — caller substitutes a placeholder.
    /// </summary>
    static string ExtractSummary(IElement content)
    {
        var div = content.QuerySelector("div.markdown.level0.summary");
        if (div is null) return "";
        return NormaliseWhitespace(CleanInlineText(div));
    }

    // ── type remarks extraction ─────────────────────────────────────────────

    /// <summary>
    /// TSD type-level remarks live in either <div class="markdown level0 conceptual"> (just
    /// after the summary) or a Remarks section (rare on type pages). Empty conceptual divs
    /// are common; return null in that case so the IR field stays null rather than empty.
    /// </summary>
    static string? ExtractTypeRemarks(IElement content)
    {
        var div = content.QuerySelector("div.markdown.level0.conceptual");
        if (div is null) return null;
        var text = NormaliseWhitespace(CleanInlineText(div));
        return string.IsNullOrEmpty(text) ? null : text;
    }

    // ── namespace extraction ────────────────────────────────────────────────

    /// <summary>
    /// Namespace is rendered as a sequence of <a> tags in an <h6>:
    ///   <h6><strong>Namespace</strong>: <a>Tekla</a>.<a>Structural</a>.<a>InteropAssemblies</a>.<a>Tedds</a></h6>
    /// We collect the anchor text-content joined by dots.
    /// </summary>
    static string ExtractNamespace(IElement content)
    {
        foreach (var h6 in content.QuerySelectorAll("h6"))
        {
            var strong = h6.QuerySelector("strong");
            if (strong is null) continue;
            if (!string.Equals(strong.TextContent.Trim(), "Namespace", StringComparison.Ordinal)) continue;
            var parts = h6.QuerySelectorAll("a").Select(a => a.TextContent.Trim()).Where(s => !string.IsNullOrEmpty(s)).ToList();
            if (parts.Count == 0) continue;
            return string.Join('.', parts);
        }
        return "";
    }

    // ── inheritance block (DocFX-style) ────────────────────────────────────

    /// <summary>
    /// TSD inheritance is rendered as a stack of <div class="levelN"> entries inside
    /// <div class="inheritance">. The LAST level is the type itself; the second-to-last
    /// is the immediate base. Items earlier in the chain are deeper ancestors.
    /// Tekla Tedds doesn't surface interface lists in this block — those (rare) live in
    /// the syntax declaration.
    /// </summary>
    static (string? baseName, List<string> interfaces) ExtractInheritanceBlock(IElement content)
    {
        var inh = content.QuerySelector("div.inheritance");
        if (inh is null) return (null, new List<string>());

        var levels = inh.QuerySelectorAll("div").Where(d =>
        {
            var cls = d.GetAttribute("class") ?? "";
            return cls.StartsWith("level", StringComparison.Ordinal);
        }).ToList();

        if (levels.Count < 2) return (null, new List<string>());

        // Second-to-last is the immediate base.
        var baseLevel = levels[^2];
        var baseAnchor = baseLevel.QuerySelector("a, span");
        var baseName = baseAnchor is null ? null : NormaliseWhitespace(CleanInlineText(baseAnchor));
        return (baseName, new List<string>());
    }

    // ── inheritance from C# syntax block ───────────────────────────────────

    /// <summary>
    /// Parse the C# declaration from the syntax block to recover base + interfaces:
    ///   public sealed class AliasAttribute : Attribute
    ///   public interface IBimProject : IBaseProject, IMore
    ///   public struct Foo : IBar
    /// Returns (null, []) for non-class/interface/struct kinds.
    /// </summary>
    static (string? baseName, List<string> interfaces) ExtractInheritanceFromSyntax(IElement content, string kind)
    {
        if (kind is not ("class" or "struct" or "interface")) return (null, new List<string>());

        var sig = ExtractCSharpSyntax(content);
        if (string.IsNullOrEmpty(sig)) return (null, new List<string>());

        // Drop attribute lines like `[Guid(...)]` or `[AttributeUsage(...)]`.
        var lines = sig.Split('\n').Select(l => l.Trim()).Where(l => l.Length > 0).ToList();
        var idx = lines.FindIndex(l => !l.StartsWith("[", StringComparison.Ordinal));
        if (idx < 0) return (null, new List<string>());
        var declAndAfter = string.Join(" ", lines.Skip(idx));

        var colonIdx = declAndAfter.IndexOf(" : ", StringComparison.Ordinal);
        if (colonIdx < 0) return (null, new List<string>());
        var inheritedRaw = declAndAfter.Substring(colonIdx + 3).Trim();
        var whereIdx = inheritedRaw.IndexOf(" where ", StringComparison.Ordinal);
        if (whereIdx >= 0) inheritedRaw = inheritedRaw.Substring(0, whereIdx).Trim();

        var items = SplitOnTopLevelCommas(inheritedRaw);
        if (items.Count == 0) return (null, new List<string>());

        string? baseName = null;
        var interfaces = new List<string>();
        foreach (var (i, item) in items.Select((x, idx2) => (idx2, x)))
        {
            var s = item.Trim();
            if (string.IsNullOrEmpty(s)) continue;
            var bareName = StripGenericArgs(s);
            var lastDot = bareName.LastIndexOf('.');
            var lastSegment = lastDot >= 0 ? bareName.Substring(lastDot + 1) : bareName;
            bool looksLikeInterface = lastSegment.Length >= 2
                && lastSegment[0] == 'I'
                && char.IsUpper(lastSegment[1]);

            if (kind == "interface")
            {
                interfaces.Add(s);
            }
            else if (i == 0 && !looksLikeInterface)
            {
                baseName = s;
            }
            else
            {
                interfaces.Add(s);
            }
        }
        return (baseName, interfaces);
    }

    static string StripGenericArgs(string name)
    {
        var lt = name.IndexOf('<');
        return lt < 0 ? name : name.Substring(0, lt);
    }

    /// <summary>
    /// Split on commas that are not inside angle-bracket generic parameter lists.
    /// </summary>
    static List<string> SplitOnTopLevelCommas(string s)
    {
        var parts = new List<string>();
        var sb = new StringBuilder();
        int depth = 0;
        foreach (var ch in s)
        {
            if (ch == '<') { depth++; sb.Append(ch); continue; }
            if (ch == '>') { depth--; sb.Append(ch); continue; }
            if (ch == ',' && depth == 0)
            {
                parts.Add(sb.ToString().Trim());
                sb.Clear();
                continue;
            }
            sb.Append(ch);
        }
        if (sb.Length > 0) parts.Add(sb.ToString().Trim());
        return parts;
    }

    /// <summary>
    /// Extract the C# syntax block: <pre><code class="language-cs">...</code></pre>
    /// Returns the text content with surrounding whitespace trimmed.
    /// </summary>
    internal static string? ExtractCSharpSyntax(IElement content)
    {
        var pre = content.QuerySelector("div.codewrapper pre code.language-cs");
        // Some pages emit <code class="language-cs hljs ..."> — querySelector's class match
        // requires the exact list; fall back to looser anchor.
        if (pre is null)
            pre = content.QuerySelectorAll("div.codewrapper pre code")
                .FirstOrDefault(c => (c.GetAttribute("class") ?? "").Contains("language-cs", StringComparison.Ordinal));
        if (pre is null) return null;
        return pre.TextContent;
    }

    // ── member-table parsing ────────────────────────────────────────────────

    sealed record MemberTableResult<T>(List<T> Members, List<string> Urls);

    /// <summary>
    /// Locate the table that follows <h3 id="<sectionId>">. Returns null if the section
    /// is absent (i.e. type has no members of that kind).
    /// </summary>
    static IElement? FindMemberTable(IElement content, string sectionId)
    {
        var h3 = content.QuerySelector($"h3#{sectionId}");
        if (h3 is null) return null;
        // The table is the NEXT sibling with .table class. Walk forward up to 3 nodes
        // to allow for whitespace text nodes.
        var node = h3.NextElementSibling;
        for (int i = 0; i < 3 && node is not null; i++)
        {
            if (node.LocalName == "table") return node;
            node = node.NextElementSibling;
        }
        return null;
    }

    /// <summary>
    /// Parse a member-row table for methods or constructors. Each row is:
    ///   <td id="<docfx_id>" data-uid="<canonical>">
    ///     <a class="xref" href="<topic-hash-url-or-canonical>">MemberName(arg, arg)</a>
    ///   </td>
    ///   <td class="markdown level1 summary"><p>doc</p></td>
    /// We capture the canonical `data-uid` for traceability and resolve the hash href to
    /// an absolute URL — the Extractor will follow it (the server 302-redirects to the
    /// real per-member page).
    /// </summary>
    static MemberTableResult<MethodInfo> ExtractMethodTable(
        IElement content, string sectionId, string typeName, string typeDocUrl, bool isCtor)
    {
        var rows = new List<MethodInfo>();
        var urls = new List<string>();
        var table = FindMemberTable(content, sectionId);
        if (table is null) return new MemberTableResult<MethodInfo>(rows, urls);

        foreach (var tr in table.QuerySelectorAll("tr"))
        {
            if (tr.QuerySelector("th") != null) continue;
            var cells = tr.QuerySelectorAll("td").ToList();
            if (cells.Count < 1) continue;

            // First td has the anchor + data-uid. Second td has the summary.
            var nameCell = cells[0];
            var anchor = nameCell.QuerySelector("a");
            if (anchor is null) continue;

            var memberName = NormaliseWhitespace(anchor.TextContent);
            if (string.IsNullOrEmpty(memberName)) continue;
            var memberHref = anchor.GetAttribute("href") ?? "";
            var memberUrl = AbsoluteUrl(memberHref);

            string summary = "";
            if (cells.Count >= 2) summary = ExtractRowSummary(cells[1]);

            var sig = isCtor
                ? $"{typeName}(...)"
                : $"{memberName}(...)";

            rows.Add(new MethodInfo(
                name: memberName,
                signature: sig,
                summary: summary,
                remarks: null,
                @params: new List<ParamInfo>(),
                returns: null,
                throws: new List<ThrowsInfo>(),
                events_related: new List<string>(),
                doc_url: string.IsNullOrEmpty(memberUrl) ? typeDocUrl : memberUrl,
                since: null,
                deprecated: null));
            urls.Add(string.IsNullOrEmpty(memberUrl) ? "" : memberUrl);
        }
        return new MemberTableResult<MethodInfo>(rows, urls);
    }

    static MemberTableResult<PropertyInfo> ExtractPropertyTable(IElement content, string sectionId, string typeDocUrl)
    {
        var rows = new List<PropertyInfo>();
        var urls = new List<string>();
        var table = FindMemberTable(content, sectionId);
        if (table is null) return new MemberTableResult<PropertyInfo>(rows, urls);

        foreach (var tr in table.QuerySelectorAll("tr"))
        {
            if (tr.QuerySelector("th") != null) continue;
            var cells = tr.QuerySelectorAll("td").ToList();
            if (cells.Count < 1) continue;

            var anchor = cells[0].QuerySelector("a");
            if (anchor is null) continue;
            var name = NormaliseWhitespace(anchor.TextContent);
            if (string.IsNullOrEmpty(name)) continue;
            var href = anchor.GetAttribute("href") ?? "";
            var url = AbsoluteUrl(href);
            string summary = cells.Count >= 2 ? ExtractRowSummary(cells[1]) : "";

            rows.Add(new PropertyInfo(
                name: name,
                type: "object",   // Placeholder; overwritten by MemberPageParser.
                getter: true,     // Placeholder.
                setter: true,     // Placeholder.
                summary: summary,
                remarks: null,
                doc_url: string.IsNullOrEmpty(url) ? typeDocUrl : url));
            urls.Add(string.IsNullOrEmpty(url) ? "" : url);
        }
        return new MemberTableResult<PropertyInfo>(rows, urls);
    }

    static MemberTableResult<EventInfo> ExtractEventTable(IElement content, string sectionId, string typeDocUrl)
    {
        var rows = new List<EventInfo>();
        var urls = new List<string>();
        var table = FindMemberTable(content, sectionId);
        if (table is null) return new MemberTableResult<EventInfo>(rows, urls);

        foreach (var tr in table.QuerySelectorAll("tr"))
        {
            if (tr.QuerySelector("th") != null) continue;
            var cells = tr.QuerySelectorAll("td").ToList();
            if (cells.Count < 1) continue;
            var anchor = cells[0].QuerySelector("a");
            if (anchor is null) continue;
            var name = NormaliseWhitespace(anchor.TextContent);
            if (string.IsNullOrEmpty(name)) continue;
            var href = anchor.GetAttribute("href") ?? "";
            var url = AbsoluteUrl(href);
            string summary = cells.Count >= 2 ? ExtractRowSummary(cells[1]) : "";

            rows.Add(new EventInfo(
                name: name,
                @delegate: "EventHandler",
                signature: $"event EventHandler {name}",
                summary: summary,
                fires_when: null,
                doc_url: string.IsNullOrEmpty(url) ? typeDocUrl : url,
                handler_thread: "unknown",
                interacts_with: new List<string>()));
            urls.Add(string.IsNullOrEmpty(url) ? "" : url);
        }
        return new MemberTableResult<EventInfo>(rows, urls);
    }

    // ── enum value table parsing ────────────────────────────────────────────

    /// <summary>
    /// Enum value rows live under <h3 id="fields">'s table. DocFX 3-column format:
    ///   <thead><tr><th>Name</th><th>Value</th><th>Description</th></tr></thead>
    ///   <tr>
    ///     <td id="...">FieldName</td>                  ← cell 0 (bare name)
    ///     <td>FieldName = -1</td>                       ← cell 1 ("Name = Value")
    ///     <td><p>doc</p></td>                           ← cell 2 (description)
    ///   </tr>
    ///
    /// We parse the integer value out of cell 1's text after "=" when present. Cell 0
    /// has the canonical name; cell 2 is the documentation.
    /// </summary>
    static List<EnumValueInfo> ExtractEnumValues(IElement content)
    {
        var values = new List<EnumValueInfo>();
        var table = FindMemberTable(content, "fields");
        if (table is null) return values;

        foreach (var tr in table.QuerySelectorAll("tr"))
        {
            if (tr.QuerySelector("th") != null) continue;
            var cells = tr.QuerySelectorAll("td").ToList();
            if (cells.Count < 1) continue;

            // Cell 0: bare field name.
            var fieldName = NormaliseWhitespace(cells[0].TextContent);
            if (string.IsNullOrEmpty(fieldName)) continue;

            // Cell 1 (if present): "Name = Value" form. Parse the integer if so.
            // Some TSD enums omit the explicit value column — fall back to storing the
            // field name as the string-value placeholder (schema allows string).
            string? rawValue = null;
            if (cells.Count >= 2)
            {
                var valText = NormaliseWhitespace(cells[1].TextContent);
                var eqIdx = valText.IndexOf('=');
                if (eqIdx >= 0)
                {
                    rawValue = valText.Substring(eqIdx + 1).Trim();
                }
                else if (!string.IsNullOrEmpty(valText) && valText != fieldName)
                {
                    rawValue = valText;
                }
            }

            // Cell 2 (or last cell): description.
            string summary = "";
            if (cells.Count >= 3) summary = ExtractRowSummary(cells[2]);
            else if (cells.Count == 2 && rawValue is null) summary = ExtractRowSummary(cells[1]);

            JsonElement valueElem;
            if (rawValue is not null && int.TryParse(rawValue, out var intVal))
                valueElem = JsonSerializer.SerializeToElement(intVal, IrJsonContext.Default.Int32);
            else if (rawValue is not null)
                valueElem = JsonSerializer.SerializeToElement(rawValue, IrJsonContext.Default.String);
            else
                valueElem = JsonSerializer.SerializeToElement(fieldName, IrJsonContext.Default.String);

            values.Add(new EnumValueInfo(fieldName, valueElem, string.IsNullOrEmpty(summary) ? null : summary));
        }
        return values;
    }

    // ── row summary extraction ─────────────────────────────────────────────

    static string ExtractRowSummary(IElement summaryCell)
    {
        // TSD summary cells: <td class="markdown level1 summary"><p>doc</p></td>
        // Sometimes the cell IS the summary; sometimes the <p> is nested inside.
        return NormaliseWhitespace(CleanInlineText(summaryCell));
    }

    // ── inline text rendering ──────────────────────────────────────────────

    /// <summary>
    /// Render an element's text content. TSD does NOT use Tekla Structures' LST
    /// script-substitution markup — its HTML is plain modern UTF-8 with characters
    /// rendered literally (including `<`, `>` after entity decoding). We just collapse
    /// whitespace; AngleSharp has already decoded entities.
    /// </summary>
    internal static string CleanInlineText(IElement root) => root.TextContent;

    static string NormaliseWhitespace(string s) =>
        Regex.Replace(s ?? "", @"\s+", " ").Trim();

    static string AbsoluteUrl(string href)
    {
        if (string.IsNullOrEmpty(href)) return "";
        if (href.StartsWith("http", StringComparison.Ordinal)) return href;
        if (href.StartsWith("/", StringComparison.Ordinal)) return BaseHost + href;
        return href;
    }
}
