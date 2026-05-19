// PageParser — parse a single Tekla developer.tekla.com type-documentation page
// into a TypeInfo plus the URL bundle for each member that needs a follow-up
// per-member fetch.
//
// Tekla docs site shape (verified 2026-05-18 against developer.tekla.com):
//
//   • Two URL forms serve identical content for any given page:
//       - /doc/tekla-structures/<version>/<slug>-<id>       (canonical)
//       - /topic/en/18/47/<guid>                            (legacy)
//   • The page body lives inside  <div class="topicContent" id="TopicContent">.
//   • The title is <h1>Foo Class | Foo Enumeration | Foo Interface | Foo Structure | Foo Delegate</h1>.
//   • The first <div class="summary"> directly after the title contains the type summary.
//   • The namespace is shown via:  <strong>Namespace:</strong> <a ...>Tekla.Structures.Foo</a>
//   • The first inheritance line (before Syntax) is the SECTION "Inheritance Hierarchy" — we
//     parse `base` from the LAST <span class="nolink"> entry (its parent before SelfLink).
//   • The C# syntax block lives in <div id="..._code_Div1" class="codeSnippetContainerCode">'s <pre>.
//     We parse the class declaration here to recover IMPLEMENTED INTERFACES — the
//     Inheritance Hierarchy section only carries base types.
//   • Each member-table section is a <span class="collapsibleRegionTitle"> labelled with the
//     section name (Constructors / Properties / Methods / Events) IMMEDIATELY followed by a
//     <table class="members" id="constructorList|propertyList|methodList|eventList"> in the
//     same parent <div class="collapsibleSection">.
//   • For enum types, the values table is <table id="enumMemberList" class="members"> with
//     rows of  <td>(icon)</td><td>(name)</td><td>(value)</td><td>(summary)</td>.
//
// Each member-table row carries an <a href="/topic/.../<guid>"> pointing to the member-detail
// page. The TYPE page lists only name + one-line summary — the full signature, parameter types,
// return value, exception list, and remarks all live on the per-member page (see
// MemberPageParser). PageParser captures the member URL for each row and returns it alongside
// the partial TypeInfo so the Extractor can drive a follow-up enrichment pass that overwrites
// the placeholders with real data.

using AngleSharp.Dom;
using AngleSharp.Html.Parser;
using System.Text;
using System.Text.Json;
using System.Text.RegularExpressions;

namespace AwareSidecar.Ingest.Extractors.Tekla;

public static class PageParser
{
    static readonly HtmlParser _parser = new();

    /// <summary>
    /// URL bundle returned alongside a parsed TypeInfo: each list pairs a member NAME with the
    /// member-detail page URL the type-page row pointed to. The Extractor walks these URLs to
    /// enrich each member with real signature + param + return data.
    ///
    /// Constructors are keyed by the row name (the type name in single-ctor cases, or the
    /// type-name + overload-signature for multi-ctor). Methods and properties and events by their
    /// row names. Index alignment is intentional: result.methods[i] pairs with links.Methods[i].
    /// </summary>
    public sealed record MemberLinks(
        IReadOnlyList<string> Constructors,
        IReadOnlyList<string> Methods,
        IReadOnlyList<string> Properties,
        IReadOnlyList<string> Events);

    public sealed record ParseResult(TypeInfo Type, MemberLinks Links);

    /// <summary>
    /// Parse one Tekla type-documentation page into a TypeInfo + member URL bundle. Returns null
    /// if the page is not a recognized type page (e.g. a "FooBar Methods" overview index page).
    /// </summary>
    public static ParseResult? Parse(string html, string sourceUrl)
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

        // The IR schema requires a non-empty `summary` for every type. Some Tekla doc pages
        // ship with an empty <div class="summary">. We supply a precise placeholder — visible
        // to anyone reading the catalog — rather than dropping the type entirely.
        if (string.IsNullOrEmpty(summary))
            summary = $"(No description provided in vendor docs for {ns}.{name}.)";

        var (baseTypeName, ifacesFromHierarchy) = ExtractInheritanceFromHierarchy(content);

        // The Inheritance Hierarchy section gives us the LINEAR ancestor chain (base). The C#
        // syntax block, however, is the only place that lists IMPLEMENTED INTERFACES, e.g.
        //   public abstract class ModelObject : Object, IComparable, IEquatable<ModelObject>
        // We parse the declaration line of the syntax pre to recover interfaces and to refine
        // `base` when the hierarchy block is absent.
        var (baseFromSyntax, ifacesFromSyntax) = ExtractInheritanceFromSyntax(content, kind);
        var finalBase = !string.IsNullOrEmpty(baseTypeName) ? baseTypeName : baseFromSyntax;
        // Merge interfaces from both sources, preserving syntax-block order which is the
        // declaration order Tekla emits.
        var interfaces = new List<string>(ifacesFromSyntax);
        foreach (var i in ifacesFromHierarchy) if (!interfaces.Contains(i)) interfaces.Add(i);

        var ctorRes = ExtractMethodTable(content, "constructorList", name, sourceUrl, isCtor: true);
        var methRes = ExtractMethodTable(content, "methodList",      name, sourceUrl, isCtor: false);
        var propRes = ExtractPropertyTable(content, "propertyList",  sourceUrl);
        var evtRes  = ExtractEventTable(content, "eventList",        sourceUrl);

        // Enum value table — only present on enum pages.
        var enumValues = kind == "enum"
            ? ExtractEnumValues(content)
            : new List<EnumValueInfo>();

        // Delegate signature — for delegates the syntax block carries the full method-style
        // signature including parameter types.
        string? delegateSig = kind == "delegate" ? ExtractCSharpSyntax(content) : null;

        // Type-level Remarks — type pages can have a top-level Remarks collapsible region.
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

    static (string name, string? kind) ParseTitle(string title)
    {
        var t = Regex.Replace(title.Trim(), @"\s+", " ");
        if (t.EndsWith(" Class", StringComparison.Ordinal))
            return (t[..^" Class".Length], "class");
        if (t.EndsWith(" Structure", StringComparison.Ordinal))
            return (t[..^" Structure".Length], "struct");
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
            if (child.LocalName == "table") continue; // titleTable carries the H1 only.
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
        // Type-level Remarks live in a top-level "Remarks" collapsibleRegion (NOT nested inside any
        // member table). The same title text appears on member pages too — but on type pages it's
        // a direct child of topicContent.
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

    /// <summary>
    /// Parse the C# class declaration from the syntax block to recover base type + interface list.
    /// Tekla emits e.g.
    ///   public abstract class ModelObject : Object,
    ///     IComparable, IEquatable&lt;ModelObject&gt;
    /// where the type-name span ends just before " : " and everything after the colon and before
    /// the end of declaration line (i.e. before any "where T :" generic constraint, before the
    /// opening brace which never appears in Tekla, or before EOF) is the list of inherited types.
    /// Returns (null, []) for non-class/interface/struct kinds (enums and delegates don't expose
    /// interfaces this way).
    /// </summary>
    static (string? baseName, List<string> interfaces) ExtractInheritanceFromSyntax(IElement content, string kind)
    {
        if (kind is not ("class" or "struct" or "interface")) return (null, new List<string>());

        var pre = FindSyntaxPre(content);
        if (pre is null) return (null, new List<string>());
        var sig = CleanInlineText(pre);

        // The signature can be multi-line and start with one or more attribute lines like
        // `[SerializableAttribute]` or `[Flags()]`. Drop those.
        var lines = sig.Split('\n').Select(l => l.Trim()).Where(l => l.Length > 0).ToList();
        var idx = lines.FindIndex(l => !l.StartsWith("[", StringComparison.Ordinal));
        if (idx < 0) return (null, new List<string>());
        // Re-join from the declaration line onward — Tekla wraps long inheritance lists across
        // multiple lines.
        var declAndAfter = string.Join(" ", lines.Skip(idx));

        // Find the type-name token. We look for the kind keyword followed by the type identifier.
        // Examples:
        //   public abstract class ModelObject : Object, IComparable, IEquatable<ModelObject>
        //   public interface ICoreLibLayoutHandler : IFoo, IBar
        //   public struct DistanceList : IBar
        // We split on " : " — anything before is the declaration; anything after is the inherited list.
        var colonIdx = declAndAfter.IndexOf(" : ", StringComparison.Ordinal);
        if (colonIdx < 0) return (null, new List<string>());
        var inheritedRaw = declAndAfter.Substring(colonIdx + 3).Trim();
        // Drop any trailing "where T : …" generic constraint clauses — Tekla rarely uses them but
        // some types do.
        var whereIdx = inheritedRaw.IndexOf(" where ", StringComparison.Ordinal);
        if (whereIdx >= 0) inheritedRaw = inheritedRaw.Substring(0, whereIdx).Trim();

        // Split on commas — but commas inside angle brackets must not split, e.g.
        // `IEquatable<Foo, Bar>` is one element, not two.
        var items = SplitOnTopLevelCommas(inheritedRaw);
        if (items.Count == 0) return (null, new List<string>());

        // For class/struct: convention says the first item is the base TYPE if it does not start
        // with "I" + uppercase (interface convention) or contain ".I" + uppercase (qualified
        // interface). It's not airtight but Tekla follows convention reliably. For interfaces, ALL
        // items are inherited interfaces, no base.
        string? baseName = null;
        var interfaces = new List<string>();
        foreach (var (i, item) in items.Select((x, idx2) => (idx2, x)))
        {
            var s = item.Trim();
            if (string.IsNullOrEmpty(s)) continue;
            // Strip a generic-arg suffix when checking the "I" prefix convention — we want to
            // detect `IEquatable<ModelObject>` as an interface.
            var bareName = StripGenericArgs(s);
            // Extract the unqualified last segment for the "starts with I + capital" check.
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
    /// Split on commas that are not inside angle-bracket generic parameter lists. Handles nested
    /// generics correctly (e.g. <c>List&lt;Dictionary&lt;int, string&gt;&gt;</c>).
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

    static IElement? FindSyntaxPre(IElement content)
    {
        // Look in the "Syntax" collapsible region's first C# code block. Tekla pages for types
        // always have a Syntax section.
        var titleSpan = content.QuerySelectorAll("span.collapsibleRegionTitle")
            .FirstOrDefault(s => string.Equals(NormaliseWhitespace(s.TextContent), "Syntax", StringComparison.Ordinal));
        var section = titleSpan?.ParentElement?.NextElementSibling;
        if (section is null) section = content;  // fallback
        var div = section.QuerySelectorAll("div.codeSnippetContainerCode")
            .FirstOrDefault(d => (d.GetAttribute("id") ?? "").EndsWith("_code_Div1", StringComparison.Ordinal));
        return div?.QuerySelector("pre");
    }

    // ── member-table parsing ────────────────────────────────────────────────

    sealed record MemberTableResult<T>(List<T> Members, List<string> Urls);

    static MemberTableResult<MethodInfo> ExtractMethodTable(IElement content, string tableId, string typeName, string typeDocUrl, bool isCtor)
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

            // The row's anchor text is the FULL disambiguated member name as the vendor
            // renders it — including overload args. Examples (LST placeholders already
            // resolved to their C# variant by CleanInlineText):
            //   • method  → "ToString(IEnumerable<Angle>)"
            //   • method  → "GetSolid()"
            //   • ctor    → "Foo()"  (the default constructor's empty-parens overload)
            //   • ctor    → "Foo(String)"
            //   • ctor    → "ReinforcementBase.ReinforcementGroupAttributes()"  (nested type)
            // We keep this text verbatim for both ctors and methods so the catalog
            // preserves overload disambiguation. Previously ctors collapsed to the bare
            // type name and lost their overload key — multiple ctors aliased to one row.
            var memberName = CleanInlineText(nameAnchor);
            var memberHref = nameAnchor.GetAttribute("href") ?? "";
            var memberUrl = AbsoluteUrl(memberHref);

            var summary = ExtractRowSummary(cells[2]);

            // Placeholder signature — overwritten by MemberPageParser during enrichment.
            // For ctors we anchor on the bare type name (Foo(...)) so the placeholder check
            // in Extractor.IsMethodPlaceholder still picks up these rows for enrichment;
            // for methods we anchor on the full member text.
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

    static MemberTableResult<PropertyInfo> ExtractPropertyTable(IElement content, string tableId, string typeDocUrl)
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
            var url = AbsoluteUrl(href);

            props.Add(new PropertyInfo(
                name: name,
                type: "object",          // Placeholder; overwritten by MemberPageParser.
                getter: true,             // Placeholder; overwritten by MemberPageParser.
                setter: true,             // Placeholder; overwritten by MemberPageParser.
                summary: ExtractRowSummary(cells[2]),
                remarks: null,
                doc_url: string.IsNullOrEmpty(url) ? typeDocUrl : url));
            urls.Add(string.IsNullOrEmpty(url) ? "" : url);
        }
        return new MemberTableResult<PropertyInfo>(props, urls);
    }

    static MemberTableResult<EventInfo> ExtractEventTable(IElement content, string tableId, string typeDocUrl)
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
            var url = AbsoluteUrl(href);

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
    /// Render inline text with Tekla's language-specific text placeholders resolved to their C#
    /// variant. Delegates to MemberPageParser.CleanInlineText so both parsers share the same
    /// LST-script substitution logic.
    /// </summary>
    static string CleanInlineText(IElement root) => MemberPageParser.CleanInlineText(root);

    static string ExtractRowSummary(IElement summaryCell)
    {
        var s = summaryCell.QuerySelector("div.summary");
        return CleanInlineText(s ?? summaryCell);
    }

    static string NormaliseWhitespace(string s) =>
        Regex.Replace(s, @"\s+", " ").Trim();

    static string AbsoluteUrl(string href)
    {
        if (string.IsNullOrEmpty(href)) return "";
        if (href.StartsWith("http", StringComparison.Ordinal)) return href;
        if (href.StartsWith("/", StringComparison.Ordinal)) return "https://developer.tekla.com" + href;
        return href;
    }
}
