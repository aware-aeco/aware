// PageParser — parse a single RhinoCommon Sandcastle type-documentation page into a
// TypeInfo plus the URL bundle for each member that needs a follow-up per-member fetch.
//
// Source: mcneel/rhinocommon-api-docs (Sandcastle-generated docs, two-version layout).
//
// Sandcastle docs layout (verified 2026-05-19 against the McNeel raw github content):
//
//   • Each version has its own branch in the github repo. Raw URLs:
//       - Rhino 7: https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/<page>
//       - Rhino 8: https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/<page>
//     The gh-pages branch is the latest 8.31; branch 7 is the last 7.19 release (LTS).
//   • The page body lives inside <div class="topicContent" id="TopicContent">.
//   • The title is <h1>Foo Class | Foo Struct | Foo Interface | Foo Enumeration | Foo Delegate</h1>
//     (note "Struct" not "Structure"; Sandcastle's RhinoCommon variant differs slightly from Tekla).
//   • Direct-child `<div class="summary">` carries the type-level summary (identical to Tekla).
//   • Namespace shown via `<strong>Namespace:</strong>` followed by an `<a>Path.To.Namespace</a>`.
//   • Inheritance Hierarchy section identical to Tekla (`span.collapsibleRegionTitle` containing
//     "Inheritance Hierarchy" → following `.collapsibleSection` → last `span.nolink` is base).
//   • C# syntax block at `div.codeSnippetContainerCode[id$="_code_Div1"] > pre` (identical to Tekla).
//   • Member tables — same `<table class="members" id="...">` shape but with three extra IDs vs Tekla:
//       - constructorList
//       - methodList
//       - propertyList
//       - eventList
//       - enumMemberList
//       - operatorList  (NEW vs Tekla — Sandcastle splits operators out)
//       - fieldList     (NEW vs Tekla — Sandcastle exposes public fields as a separate table)
//     Operators are rendered as static methods at the per-member detail page level (their title
//     ends in " Operator" rather than " Method"). Fields have no enrichment surface beyond what
//     the type-page row already shows (name + type + summary) — they're folded into the
//     properties[] list with `getter=true, setter=false` since the IR schema has no `fields[]`.
//   • Member detail pages have h1 titles ending in:
//       - " Method"         → method
//       - " Constructor"    → constructor
//       - " Property"       → property
//       - " Event"          → event
//       - " Operator"       → operator (parsed like a method)
//       - " Field"          → field (parsed like a property)
//
// Each member-table row's second <td> carries an <a href="(M_|P_|E_|F_|Overload_)..."> link to
// the detail page. We capture that URL for the Pass 2 enrichment.

using AngleSharp.Dom;
using AngleSharp.Html.Parser;
using System.Text;
using System.Text.Json;
using System.Text.RegularExpressions;

namespace AwareSidecar.Ingest.Extractors.Rhino;

public static class PageParser
{
    static readonly HtmlParser _parser = new();

    /// <summary>
    /// URL bundle returned alongside a parsed TypeInfo: each list pairs a member NAME with the
    /// member-detail page URL the type-page row pointed to. The Extractor walks these URLs to
    /// enrich each member with real signature + param + return data.
    /// </summary>
    public sealed record MemberLinks(
        IReadOnlyList<string> Constructors,
        IReadOnlyList<string> Methods,
        IReadOnlyList<string> Properties,
        IReadOnlyList<string> Events);

    public sealed record ParseResult(TypeInfo Type, MemberLinks Links);

    /// <summary>
    /// Parse one RhinoCommon type-documentation page into a TypeInfo + member URL bundle. Returns
    /// null if the page is not a recognized type page (e.g. a "FooBar Methods" overview index page).
    /// </summary>
    /// <param name="baseUrl">Absolute URL the relative href values should resolve against.</param>
    public static ParseResult? Parse(string html, string sourceUrl, string baseUrl)
    {
        using var doc = _parser.ParseDocument(html);
        var content = doc.QuerySelector("div.topicContent#TopicContent");
        if (content is null) return null;

        var h1 = content.QuerySelector("h1");
        var title = h1 is null ? "" : CleanInlineText(h1);
        if (string.IsNullOrEmpty(title)) return null;

        var (name, kind) = ParseTitle(title);
        if (kind is null) return null;

        var summary = ExtractDirectChildSummary(content);
        var ns = ExtractNamespace(content);
        if (string.IsNullOrEmpty(ns)) return null;

        if (string.IsNullOrEmpty(summary))
            summary = $"(No description provided in vendor docs for {ns}.{name}.)";

        var (baseTypeName, ifacesFromHierarchy) = ExtractInheritanceFromHierarchy(content);

        var (baseFromSyntax, ifacesFromSyntax) = ExtractInheritanceFromSyntax(content, kind);
        var finalBase = !string.IsNullOrEmpty(baseTypeName) ? baseTypeName : baseFromSyntax;
        var interfaces = new List<string>(ifacesFromSyntax);
        foreach (var i in ifacesFromHierarchy) if (!interfaces.Contains(i)) interfaces.Add(i);

        // Methods table — RhinoCommon splits operators into their own table. The Extractor will
        // run the enrichment against BOTH. We merge them under `methods[]` since the IR schema
        // has no separate operators slot. Operators preserve their `(Args)` overload signature
        // verbatim from the row name so they remain disambiguated.
        var ctorRes = ExtractMethodTable(content, "constructorList", name, sourceUrl, baseUrl, isCtor: true);
        var methRes = ExtractMethodTable(content, "methodList",      name, sourceUrl, baseUrl, isCtor: false);
        var opRes   = ExtractMethodTable(content, "operatorList",    name, sourceUrl, baseUrl, isCtor: false);
        var propRes = ExtractPropertyTable(content, "propertyList",  sourceUrl, baseUrl);
        // Fields are folded into properties[] — `getter=true, setter=false` (public read-only) is
        // the conventional .NET reflection mapping. They have a type and a value so the property
        // schema's required fields (`type`, `getter`, `setter`) fit naturally.
        var fieldRes = ExtractFieldTable(content, "fieldList", sourceUrl, baseUrl);
        var evtRes  = ExtractEventTable(content, "eventList",        sourceUrl, baseUrl);

        var allMethods = methRes.Members.Concat(opRes.Members).ToList();
        var allMethodUrls = methRes.Urls.Concat(opRes.Urls).ToList();
        var allProps = propRes.Members.Concat(fieldRes.Members).ToList();
        var allPropUrls = propRes.Urls.Concat(fieldRes.Urls).ToList();

        // Enum value table — only present on enum pages.
        var enumValues = kind == "enum"
            ? ExtractEnumValues(content)
            : new List<EnumValueInfo>();

        string? delegateSig = kind == "delegate" ? ExtractCSharpSyntax(content) : null;

        var typeRemarks = ExtractTypeRemarks(content);

        var typeInfo = new TypeInfo(
            name: name,
            @namespace: ns,
            kind: kind,
            summary: NormaliseWhitespace(summary),
            remarks: typeRemarks,
            @base: finalBase,
            interfaces: interfaces,
            doc_url: sourceUrl,
            constructors: ctorRes.Members,
            methods: allMethods,
            properties: allProps,
            events: evtRes.Members,
            enum_values: enumValues,
            delegate_signature: delegateSig);

        var links = new MemberLinks(
            Constructors: ctorRes.Urls,
            Methods: allMethodUrls,
            Properties: allPropUrls,
            Events: evtRes.Urls);

        return new ParseResult(typeInfo, links);
    }

    // ── title parsing ────────────────────────────────────────────────────────

    static (string name, string? kind) ParseTitle(string title)
    {
        var t = Regex.Replace(title.Trim(), @"\s+", " ");
        if (t.EndsWith(" Class", StringComparison.Ordinal))
            return (t[..^" Class".Length], "class");
        if (t.EndsWith(" Structure", StringComparison.Ordinal))
            return (t[..^" Structure".Length], "struct");
        if (t.EndsWith(" Struct", StringComparison.Ordinal))
            return (t[..^" Struct".Length], "struct");
        if (t.EndsWith(" Interface", StringComparison.Ordinal))
            return (t[..^" Interface".Length], "interface");
        if (t.EndsWith(" Enumeration", StringComparison.Ordinal))
            return (t[..^" Enumeration".Length], "enum");
        if (t.EndsWith(" Delegate", StringComparison.Ordinal))
            return (t[..^" Delegate".Length], "delegate");
        return (t, null);
    }

    // ── direct-child summary extraction ─────────────────────────────────────

    static string ExtractDirectChildSummary(IElement content)
    {
        foreach (var child in content.Children)
        {
            if (child.LocalName == "div" && child.ClassList.Contains("summary"))
                return CleanInlineText(child);
            if (child.LocalName == "table") continue;
        }
        foreach (var s in content.QuerySelectorAll("div.summary"))
        {
            var p = s.ParentElement;
            var inMembersTable = false;
            while (p is not null && p != content)
            {
                if (p.LocalName == "table" && p.ClassList.Contains("members")) { inMembersTable = true; break; }
                p = p.ParentElement;
            }
            if (!inMembersTable) return CleanInlineText(s);
        }
        return "";
    }

    // ── namespace extraction ────────────────────────────────────────────────

    static string ExtractNamespace(IElement content)
    {
        foreach (var strong in content.QuerySelectorAll("strong"))
        {
            if (string.Equals(strong.TextContent.Trim(), "Namespace:", StringComparison.Ordinal))
            {
                var next = strong.NextElementSibling;
                while (next != null)
                {
                    if (next.LocalName == "a") return CleanInlineText(next).Replace(" ", "");
                    next = next.NextElementSibling;
                }
            }
        }
        return "";
    }

    // ── type-level remarks extraction ───────────────────────────────────────

    static string? ExtractTypeRemarks(IElement content)
    {
        foreach (var titleSpan in content.QuerySelectorAll("span.collapsibleRegionTitle"))
        {
            var t = NormaliseWhitespace(titleSpan.TextContent);
            if (!string.Equals(t, "Remarks", StringComparison.Ordinal)) continue;
            var section = titleSpan.ParentElement?.NextElementSibling;
            if (section is null || !section.ClassList.Contains("collapsibleSection")) continue;
            var clone = (IElement)section.Clone();
            foreach (var code in clone.QuerySelectorAll("div.codeSnippetContainer").ToList())
                code.Remove();
            var text = NormaliseWhitespace(CleanInlineText(clone));
            return string.IsNullOrEmpty(text) ? null : text;
        }
        return null;
    }

    // ── inheritance hierarchy: linear ancestor chain ────────────────────────

    static (string? baseName, List<string> interfaces) ExtractInheritanceFromHierarchy(IElement content)
    {
        var titleSpan = content.QuerySelectorAll("span.collapsibleRegionTitle")
            .FirstOrDefault(s => s.TextContent.Contains("Inheritance Hierarchy", StringComparison.Ordinal));
        if (titleSpan is null) return (null, new List<string>());

        var section = titleSpan.ParentElement?.NextElementSibling;
        if (section is null || !section.ClassList.Contains("collapsibleSection"))
            return (null, new List<string>());

        var ancestors = section.QuerySelectorAll("span.nolink").ToList();
        if (ancestors.Count == 0) return (null, new List<string>());

        var lastAncestor = ancestors[^1];
        var baseText = CleanInlineText(lastAncestor).Replace(" ", "");
        return (baseText, new List<string>());
    }

    // ── inheritance from C# syntax block: base + interfaces ────────────────

    static (string? baseName, List<string> interfaces) ExtractInheritanceFromSyntax(IElement content, string kind)
    {
        if (kind is not ("class" or "struct" or "interface")) return (null, new List<string>());

        var pre = FindSyntaxPre(content);
        if (pre is null) return (null, new List<string>());
        var sig = CleanInlineText(pre);

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

    static IElement? FindSyntaxPre(IElement content)
    {
        var titleSpan = content.QuerySelectorAll("span.collapsibleRegionTitle")
            .FirstOrDefault(s => string.Equals(NormaliseWhitespace(s.TextContent), "Syntax", StringComparison.Ordinal));
        var section = titleSpan?.ParentElement?.NextElementSibling;
        if (section is null) section = content;
        var div = section.QuerySelectorAll("div.codeSnippetContainerCode")
            .FirstOrDefault(d => (d.GetAttribute("id") ?? "").EndsWith("_code_Div1", StringComparison.Ordinal));
        return div?.QuerySelector("pre");
    }

    // ── member-table parsing ────────────────────────────────────────────────

    sealed record MemberTableResult<T>(List<T> Members, List<string> Urls);

    static MemberTableResult<MethodInfo> ExtractMethodTable(IElement content, string tableId, string typeName, string typeDocUrl, string baseUrl, bool isCtor)
    {
        var rows = new List<MethodInfo>();
        var urls = new List<string>();
        var table = content.QuerySelector($"table.members#{tableId}");
        if (table is null) return new MemberTableResult<MethodInfo>(rows, urls);

        foreach (var tr in table.QuerySelectorAll("tr"))
        {
            if (tr.QuerySelector("th") != null) continue;
            var cells = tr.QuerySelectorAll("td").ToList();
            if (cells.Count < 3) continue;

            var nameAnchor = cells[1].QuerySelector("a");
            if (nameAnchor is null) continue;

            var memberName = CleanInlineText(nameAnchor);
            var memberHref = nameAnchor.GetAttribute("href") ?? "";
            var memberUrl = AbsoluteUrl(memberHref, baseUrl);

            // Sandcastle's RhinoCommon type pages list INHERITED members (Equals / ToString /
            // GetType / etc.) with hrefs pointing to docs.microsoft.com / learn.microsoft.com —
            // NOT to a McNeel-hosted member page. Those system members are not part of the
            // McNeel API surface; they're standard .NET object/value-type inheritance. We skip
            // them entirely at type-page parse time so:
            //   (a) they don't pollute the IR's methods[] count with system-inherited members
            //   (b) they don't generate spurious enrichment errors on docs.microsoft.com (which
            //       returns 301 redirects and serves non-Sandcastle pages anyway)
            //
            // Detection: any href whose host is not the same as baseUrl's host. The baseUrl
            // we receive points at the raw.githubusercontent.com/mcneel/... tree, so this
            // cleanly excludes microsoft.com / docs.microsoft.com / learn.microsoft.com etc.
            if (IsExternalUrl(memberUrl, baseUrl)) continue;

            var summary = ExtractRowSummary(cells[2]);

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

    /// <summary>
    /// True when <paramref name="memberUrl"/>'s host differs from <paramref name="baseUrl"/>'s
    /// host — meaning the member is inherited from a non-McNeel-documented system type and links
    /// to a different docs site (typically docs.microsoft.com / learn.microsoft.com).
    /// </summary>
    static bool IsExternalUrl(string memberUrl, string baseUrl)
    {
        if (string.IsNullOrEmpty(memberUrl)) return false;
        try
        {
            var m = new Uri(memberUrl);
            var b = new Uri(baseUrl);
            return !string.Equals(m.Host, b.Host, StringComparison.OrdinalIgnoreCase);
        }
        catch
        {
            return false;
        }
    }

    static MemberTableResult<PropertyInfo> ExtractPropertyTable(IElement content, string tableId, string typeDocUrl, string baseUrl)
    {
        var props = new List<PropertyInfo>();
        var urls = new List<string>();
        var table = content.QuerySelector($"table.members#{tableId}");
        if (table is null) return new MemberTableResult<PropertyInfo>(props, urls);

        foreach (var tr in table.QuerySelectorAll("tr"))
        {
            if (tr.QuerySelector("th") != null) continue;
            var cells = tr.QuerySelectorAll("td").ToList();
            if (cells.Count < 3) continue;
            var anchor = cells[1].QuerySelector("a");
            if (anchor is null) continue;

            var name = CleanInlineText(anchor);
            var href = anchor.GetAttribute("href") ?? "";
            var url = AbsoluteUrl(href, baseUrl);

            // Skip system-inherited properties whose href points outside the McNeel docs.
            if (IsExternalUrl(url, baseUrl)) continue;

            props.Add(new PropertyInfo(
                name: name,
                type: "object",
                getter: true,
                setter: true,
                summary: ExtractRowSummary(cells[2]),
                remarks: null,
                doc_url: string.IsNullOrEmpty(url) ? typeDocUrl : url));
            urls.Add(string.IsNullOrEmpty(url) ? "" : url);
        }
        return new MemberTableResult<PropertyInfo>(props, urls);
    }

    /// <summary>
    /// Fields share the property table shape but are read-only by definition (Sandcastle
    /// doesn't render writable fields as a separate slot). We fold them into properties[] with
    /// `getter=true, setter=false` so they go through the same enrichment path. The Pass 2
    /// enrichment recognises field detail pages (h1 ending in " Field") and produces a
    /// PropertyEnrichment shape with `setter=false`.
    /// </summary>
    static MemberTableResult<PropertyInfo> ExtractFieldTable(IElement content, string tableId, string typeDocUrl, string baseUrl)
    {
        var props = new List<PropertyInfo>();
        var urls = new List<string>();
        var table = content.QuerySelector($"table.members#{tableId}");
        if (table is null) return new MemberTableResult<PropertyInfo>(props, urls);

        foreach (var tr in table.QuerySelectorAll("tr"))
        {
            if (tr.QuerySelector("th") != null) continue;
            var cells = tr.QuerySelectorAll("td").ToList();
            if (cells.Count < 3) continue;
            var anchor = cells[1].QuerySelector("a");
            if (anchor is null) continue;

            var name = CleanInlineText(anchor);
            var href = anchor.GetAttribute("href") ?? "";
            var url = AbsoluteUrl(href, baseUrl);

            // Skip system-inherited fields whose href points outside the McNeel docs.
            if (IsExternalUrl(url, baseUrl)) continue;

            props.Add(new PropertyInfo(
                name: name,
                type: "object",
                getter: true,
                setter: false,  // fields are conceptually read-only in C# reflection-mapping
                summary: ExtractRowSummary(cells[2]),
                remarks: null,
                doc_url: string.IsNullOrEmpty(url) ? typeDocUrl : url));
            urls.Add(string.IsNullOrEmpty(url) ? "" : url);
        }
        return new MemberTableResult<PropertyInfo>(props, urls);
    }

    static MemberTableResult<EventInfo> ExtractEventTable(IElement content, string tableId, string typeDocUrl, string baseUrl)
    {
        var events = new List<EventInfo>();
        var urls = new List<string>();
        var table = content.QuerySelector($"table.members#{tableId}");
        if (table is null) return new MemberTableResult<EventInfo>(events, urls);

        foreach (var tr in table.QuerySelectorAll("tr"))
        {
            if (tr.QuerySelector("th") != null) continue;
            var cells = tr.QuerySelectorAll("td").ToList();
            if (cells.Count < 3) continue;
            var anchor = cells[1].QuerySelector("a");
            if (anchor is null) continue;

            var name = CleanInlineText(anchor);
            var href = anchor.GetAttribute("href") ?? "";
            var url = AbsoluteUrl(href, baseUrl);

            // Skip system-inherited events whose href points outside the McNeel docs.
            if (IsExternalUrl(url, baseUrl)) continue;

            events.Add(new EventInfo(
                name: name,
                @delegate: "EventHandler",
                signature: $"event EventHandler {name}",
                summary: ExtractRowSummary(cells[2]),
                fires_when: null,
                doc_url: string.IsNullOrEmpty(url) ? typeDocUrl : url,
                handler_thread: "unknown",
                interacts_with: new List<string>()));
            urls.Add(string.IsNullOrEmpty(url) ? "" : url);
        }
        return new MemberTableResult<EventInfo>(events, urls);
    }

    // ── enum value table parsing ────────────────────────────────────────────

    static List<EnumValueInfo> ExtractEnumValues(IElement content)
    {
        var table = content.QuerySelector("table.members#enumMemberList");
        if (table is null) return new List<EnumValueInfo>();
        var values = new List<EnumValueInfo>();
        foreach (var tr in table.QuerySelectorAll("tr"))
        {
            if (tr.QuerySelector("th") != null) continue;
            var cells = tr.QuerySelectorAll("td").ToList();
            if (cells.Count < 4) continue;
            var nameCell = cells[1];
            var nameText = nameCell.QuerySelector("span.selflink")?.TextContent.Trim()
                          ?? nameCell.TextContent.Trim();
            var valueText = cells[2].TextContent.Trim();
            var summary = cells[3].TextContent.Trim();

            JsonElement valueElem;
            if (int.TryParse(valueText, out var intVal))
                valueElem = JsonSerializer.SerializeToElement(intVal, IrJsonContext.Default.Int32);
            else
                valueElem = JsonSerializer.SerializeToElement(valueText, IrJsonContext.Default.String);

            values.Add(new EnumValueInfo(nameText, valueElem, string.IsNullOrEmpty(summary) ? null : NormaliseWhitespace(summary)));
        }
        return values;
    }

    // ── syntax block extraction (for delegates) ─────────────────────────────

    static string? ExtractCSharpSyntax(IElement content)
    {
        var pre = FindSyntaxPre(content);
        return pre is null ? null : CleanInlineText(pre);
    }

    // ── inline text rendering ──────────────────────────────────────────────

    /// <summary>
    /// Render inline text with Sandcastle's language-specific text placeholders resolved to their
    /// C# variant. Delegates to MemberPageParser.CleanInlineText so both parsers share the same
    /// LST-script substitution logic (including the `nu=` fallback when no `cs=` key exists).
    /// </summary>
    static string CleanInlineText(IElement root) => MemberPageParser.CleanInlineText(root);

    static string ExtractRowSummary(IElement summaryCell)
    {
        var s = summaryCell.QuerySelector("div.summary");
        return CleanInlineText(s ?? summaryCell);
    }

    static string NormaliseWhitespace(string s) =>
        Regex.Replace(s, @"\s+", " ").Trim();

    /// <summary>
    /// Resolve a Sandcastle relative href against the base URL of the doc tree. Sandcastle emits
    /// both bare hrefs ("N_Rhino_Geometry.htm") and parent-walk hrefs ("../html/N_Rhino_Geometry.htm").
    /// Uri math handles both forms correctly.
    /// </summary>
    static string AbsoluteUrl(string href, string baseUrl)
    {
        if (string.IsNullOrEmpty(href)) return "";
        if (href.StartsWith("http", StringComparison.Ordinal)) return href;
        try
        {
            var uri = new Uri(new Uri(baseUrl), href);
            return uri.ToString();
        }
        catch
        {
            if (baseUrl.EndsWith("/", StringComparison.Ordinal)) return baseUrl + href;
            var slash = baseUrl.LastIndexOf('/');
            return slash < 0 ? href : baseUrl.Substring(0, slash + 1) + href;
        }
    }
}
