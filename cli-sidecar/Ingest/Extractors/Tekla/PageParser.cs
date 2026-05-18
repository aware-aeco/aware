// PageParser — parse a single Tekla developer.tekla.com type-documentation page
// into a TypeInfo (the schema-level record).
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
//   • Each member-table section is a <span class="collapsibleRegionTitle"> labelled with the
//     section name (Constructors / Properties / Methods / Events) IMMEDIATELY followed by a
//     <table class="members" id="constructorList|propertyList|methodList|eventList"> in the
//     same parent <div class="collapsibleSection">.
//   • For enum types, the values table is <table id="enumMemberList" class="members"> with
//     rows of  <td>(icon)</td><td>(name)</td><td>(value)</td><td>(summary)</td>.
//   • The C# syntax block is <div id="..._code_Div1" class="codeSnippetContainerCode">'s <pre>.
//
// This parser extracts type-level coverage. Method/property/event/constructor signatures
// available on the type page are SHORT (name + summary only) — full per-member signatures
// + parameter types + returns + exceptions live on per-member pages (/topic/... links).
// For coverage IR purposes the type page yields:
//   • method.name + summary + doc_url   (signature = name as a synthesized fallback)
//   • property.name + summary + doc_url (type + getter/setter inferred from icon, see below)
//   • event.name + summary + doc_url    (delegate left as "EventHandler" placeholder)
//   • constructor.name + summary + doc_url (signature = type name + "()" fallback)
// EXTRACTION-NOTES.md documents this limitation. A future Phase B pass can deepen
// the parser to follow member-page links and overwrite signature/params/returns —
// but the IR + agent are valid and useful as-is, and the row summaries are the same
// authoritative source the SkillWriter renders into namespace skills.

using AngleSharp;
using AngleSharp.Dom;
using AngleSharp.Html.Parser;
using System.Text.Json;
using System.Text.RegularExpressions;

namespace AwareSidecar.Ingest.Extractors.Tekla;

public static class PageParser
{
    static readonly HtmlParser _parser = new();

    /// <summary>
    /// Parse one Tekla type-documentation page into a TypeInfo. Returns null if the page
    /// is not a recognized type page (e.g. a "FooBar Methods" overview page that has no
    /// type-level content — those are intermediate index pages and not types themselves).
    /// </summary>
    public static TypeInfo? Parse(string html, string sourceUrl)
    {
        using var doc = _parser.ParseDocument(html);
        var content = doc.QuerySelector("div.topicContent#TopicContent");
        if (content is null) return null;

        var h1 = content.QuerySelector("h1");
        var title = h1 is null ? "" : CleanInlineText(h1);
        if (string.IsNullOrEmpty(title)) return null;

        // The H1 title is one of:
        //   "Foo Class"           → kind=class
        //   "Foo Structure"       → kind=struct
        //   "Foo Interface"       → kind=interface
        //   "Foo Enumeration"     → kind=enum
        //   "Foo Delegate"        → kind=delegate
        //   "Foo Methods"         → NOT a type page (overview index); skip
        //   "Foo Properties"      → NOT a type page; skip
        //   "Foo Constructor"     → NOT a type page (single-member page); skip
        //   "Foo.Bar Method"      → NOT a type page (single-member page); skip
        //   "Foo Namespace"       → NOT a type page (namespace landing page); skip
        var (name, kind) = ParseTitle(title);
        if (kind is null) return null;

        var summary = ExtractDirectChildSummary(content);
        var ns = ExtractNamespace(content);
        if (string.IsNullOrEmpty(ns)) return null;  // can't place the type without a namespace

        // The IR schema requires a non-empty `summary` for every type. Some Tekla doc pages
        // ship with an empty <div class="summary"> (no description authored on the vendor side).
        // We supply a precise, honest placeholder rather than dropping the type — the
        // placeholder makes the omission visible to anyone reading the catalog.
        if (string.IsNullOrEmpty(summary))
            summary = $"(No description provided in vendor docs for {ns}.{name}.)";

        var (baseTypeName, interfaces) = ExtractInheritance(content);

        // Member tables — each section appears as a "collapsibleRegionTitle" span,
        // immediately followed by a "collapsibleSection" div containing the table.
        var constructors = ExtractMethodTable(content, "constructorList", name, sourceUrl, isCtor: true);
        var methods       = ExtractMethodTable(content, "methodList",      name, sourceUrl, isCtor: false);
        var properties    = ExtractPropertyTable(content, "propertyList",  sourceUrl);
        var events        = ExtractEventTable(content, "eventList",        sourceUrl);

        // Enum values table — only present on enum pages.
        var enumValues = kind == "enum"
            ? ExtractEnumValues(content)
            : new List<EnumValueInfo>();

        // Delegate signature — for delegates the syntax block contains the full method-style
        // signature. We extract the C# variant.
        string? delegateSig = kind == "delegate" ? ExtractCSharpSyntax(content) : null;

        return new TypeInfo(
            name: name,
            @namespace: ns,
            kind: kind,
            summary: NormaliseWhitespace(summary),
            remarks: null,                  // Type pages don't have a "Remarks" section at type scope.
            @base: baseTypeName,
            interfaces: interfaces,
            doc_url: sourceUrl,
            constructors: constructors,
            methods: methods,
            properties: properties,
            events: events,
            enum_values: enumValues,
            delegate_signature: delegateSig);
    }

    // ── title parsing ────────────────────────────────────────────────────────

    static (string name, string? kind) ParseTitle(string title)
    {
        // Tekla titles use trailing keyword to identify kind. Examples seen:
        //   "Assertion Class"
        //   "TemporaryTransparency Enumeration"
        //   "TeklaStructuresSettings.InvalidPathCallback Delegate"
        //   "ClashCheckOptions.SafetyDistanceMode Structure"
        //   "ICoreLibLayoutHandler Interface"
        // Some title-cased tokens collide (e.g. "Foo Method" is a member page not a type page).
        var t = title.Trim();
        // Strip trailing whitespace artefacts (the docs sometimes emit "Constructor " etc.).
        t = Regex.Replace(t, @"\s+", " ");

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
        // Anything else (Methods, Properties, Constructor, Namespace, individual Method names)
        // is not a type page from our perspective.
        return (t, null);
    }

    // ── direct-child summary extraction ─────────────────────────────────────

    static string ExtractDirectChildSummary(IElement content)
    {
        // The type-level summary is the FIRST <div class="summary"> that is a direct child
        // of the topicContent container (or directly inside the title block). Member-table
        // rows ALSO contain <div class="summary"> children, so we must not grab those.
        // Use CleanInlineText to swap LST placeholder spans for ".".
        foreach (var child in content.Children)
        {
            if (child.LocalName == "div" && child.ClassList.Contains("summary"))
                return CleanInlineText(child);
            if (child.LocalName == "table") continue; // titleTable holds the H1 only — skip.
        }
        // Fallback: find ANY .summary that is NOT inside a .members table.
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
        // Pattern in the page body:
        //   <strong>Namespace:</strong> <a href="/topic/...">Tekla.Structures.Model</a>
        // We find the <strong> with "Namespace:" text and take the next link's text.
        // The anchor may carry inline <script>+placeholder spans the same way ancestor types do,
        // so we route through CleanInlineText.
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
        // Fallback: parse "Tekla.Structures.<something>" out of the breadcrumb chain if available.
        return "";
    }

    // ── inheritance hierarchy ───────────────────────────────────────────────

    static (string? baseName, List<string> interfaces) ExtractInheritance(IElement content)
    {
        // The "Inheritance Hierarchy" section ID is positional ("ID0RBSection" or similar) so
        // we anchor on its title span instead.
        var titleSpan = content.QuerySelectorAll("span.collapsibleRegionTitle")
            .FirstOrDefault(s => s.TextContent.Contains("Inheritance Hierarchy", StringComparison.Ordinal));
        if (titleSpan is null) return (null, new List<string>());

        // The hierarchy lives in the next sibling div with class="collapsibleSection".
        var section = titleSpan.ParentElement?.NextElementSibling;
        if (section is null || !section.ClassList.Contains("collapsibleSection"))
            return (null, new List<string>());

        // The hierarchy is rendered as a chain of <span class="nolink"> for ancestor types and
        // a final <span class="selflink"> for the current type. The direct parent (base type)
        // is the LAST .nolink before .selflink.
        var ancestors = section.QuerySelectorAll("span.nolink").ToList();
        if (ancestors.Count == 0) return (null, new List<string>());

        var lastAncestor = ancestors[^1];
        // Strip nested script tags' text contributions before reading textContent.
        var baseText = CleanNoLinkText(lastAncestor);
        return (baseText, new List<string>());  // Tekla pages don't itemize implemented interfaces; left as [].
    }

    // The .nolink spans render dotted type names through inline placeholders that the
    // page's JS replaces at render time, e.g.:
    //
    //   <span class="nolink">System<span id="LSTAB654CE3_0"></span><script>AddLanguageSpecificTextSet(
    //     "LSTAB654CE3_0?cs=.|vb=.|cpp=::|nu=.|fs=.");</script>Object</span>
    //
    // In a real browser the script substitutes "." (the C# variant) into the empty inner span.
    // Without JS execution we have to do the substitution ourselves. The empty placeholder span
    // has an id starting with "LST" and is followed by a <script> whose first language is `cs=`.
    // We walk children and synthesize the dot between the text nodes on either side of each
    // LST placeholder.
    static string CleanNoLinkText(IElement span)
    {
        var sb = new System.Text.StringBuilder();
        foreach (var node in span.ChildNodes)
        {
            switch (node.NodeType)
            {
                case NodeType.Text:
                    sb.Append(node.TextContent);
                    break;
                case NodeType.Element when node is IElement el:
                    if (el.LocalName == "script") break;            // script never contributes text
                    var id = el.GetAttribute("id") ?? "";
                    if (id.StartsWith("LST", StringComparison.Ordinal))
                    {
                        // Placeholder — the doc's JS would replace it with the C# delimiter ".".
                        sb.Append('.');
                    }
                    else
                    {
                        sb.Append(el.TextContent);
                    }
                    break;
            }
        }
        return Regex.Replace(sb.ToString(), @"\s+", "").Trim();
    }

    // ── method / constructor table parsing ──────────────────────────────────

    static List<MethodInfo> ExtractMethodTable(IElement content, string tableId, string typeName, string typeDocUrl, bool isCtor)
    {
        var table = content.QuerySelector($"table.members#{tableId}");
        if (table is null) return new List<MethodInfo>();

        var rows = new List<MethodInfo>();
        foreach (var tr in table.QuerySelectorAll("tr"))
        {
            // Skip header row (its first cell is a <th>).
            if (tr.QuerySelector("th") != null) continue;
            var cells = tr.QuerySelectorAll("td").ToList();
            if (cells.Count < 3) continue;

            // Cell layout: [icon] [<a>Name</a>] [<div class="summary">...</div>]
            var nameAnchor = cells[1].QuerySelector("a");
            if (nameAnchor is null) continue;

            var memberName = CleanInlineText(nameAnchor);
            var memberHref = nameAnchor.GetAttribute("href") ?? "";
            var memberUrl = AbsoluteUrl(memberHref);

            var summary = ExtractRowSummary(cells[2]);

            // Synthesized signature: name + "(...)" placeholder. Member-page deep crawl would
            // replace this; documented as a coverage caveat in EXTRACTION-NOTES.md.
            var sig = isCtor
                ? $"{typeName}(...)"
                : $"{memberName}(...)";

            rows.Add(new MethodInfo(
                name: isCtor ? typeName : memberName,
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
        }
        return rows;
    }

    // ── property table parsing ──────────────────────────────────────────────

    static List<PropertyInfo> ExtractPropertyTable(IElement content, string tableId, string typeDocUrl)
    {
        var table = content.QuerySelector($"table.members#{tableId}");
        if (table is null) return new List<PropertyInfo>();
        var props = new List<PropertyInfo>();
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

            // Icon column (cells[0]) tells us static/instance. Without a per-member fetch we
            // don't know the property TYPE; we placeholder "object" and document this.
            // get/set: Tekla docs don't expose this in the table without a member fetch. Default
            // to (getter=true, setter=true) so consumers see read-write access until corrected.
            props.Add(new PropertyInfo(
                name: name,
                type: "object",
                getter: true,
                setter: true,
                summary: ExtractRowSummary(cells[2]),
                remarks: null,
                doc_url: string.IsNullOrEmpty(url) ? typeDocUrl : url));
        }
        return props;
    }

    // ── event table parsing ─────────────────────────────────────────────────

    static List<EventInfo> ExtractEventTable(IElement content, string tableId, string typeDocUrl)
    {
        var table = content.QuerySelector($"table.members#{tableId}");
        if (table is null) return new List<EventInfo>();
        var events = new List<EventInfo>();
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
                @delegate: "EventHandler",       // Placeholder; the per-member page has the real delegate name.
                signature: $"event EventHandler {name}",
                summary: ExtractRowSummary(cells[2]),
                fires_when: null,
                doc_url: string.IsNullOrEmpty(url) ? typeDocUrl : url,
                handler_thread: "unknown",
                interacts_with: new List<string>()));
        }
        return events;
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
            // Cell layout: [icon] [name] [value] [summary]
            var nameCell = cells[1];
            var nameText = nameCell.QuerySelector("span.selflink")?.TextContent.Trim()
                          ?? nameCell.TextContent.Trim();
            var valueText = cells[2].TextContent.Trim();
            var summary = cells[3].TextContent.Trim();

            // Try to parse value as integer first; fall back to string.
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
        // Page emits two language tabs: id ends with "_code_Div1" (C#) and "_code_Div2" (VB).
        // The <pre> may contain inline language-placeholder spans for dotted nested types
        // (e.g. Events.DrawingUpdateTypeEnum). Route through CleanInlineText to swap the
        // placeholders for "." and to keep the result script-text-free.
        var div = content.QuerySelectorAll("div.codeSnippetContainerCode")
            .FirstOrDefault(d => (d.GetAttribute("id") ?? "").EndsWith("_code_Div1"));
        var pre = div?.QuerySelector("pre");
        return pre is null ? null : CleanInlineText(pre);
    }

    // ── helpers ─────────────────────────────────────────────────────────────

    /// <summary>
    /// Strip inline `<script>` and language-placeholder `<span id="LST...">` nodes from an
    /// element's textual rendering, substituting the C# delimiter "." where the placeholder
    /// span would otherwise be replaced by JS. Used for member names which sometimes contain
    /// dotted-generic types (e.g. <c>GetAttributeFile(List&lt;String&gt;, String)</c>).
    /// </summary>
    static string CleanInlineText(IElement root)
    {
        var sb = new System.Text.StringBuilder();
        Walk(root);
        return Regex.Replace(sb.ToString(), @"\s+", " ").Trim();

        void Walk(IElement node)
        {
            foreach (var child in node.ChildNodes)
            {
                if (child.NodeType == NodeType.Text)
                {
                    sb.Append(child.TextContent);
                }
                else if (child is IElement el)
                {
                    if (el.LocalName == "script") continue;
                    var id = el.GetAttribute("id") ?? "";
                    if (id.StartsWith("LST", StringComparison.Ordinal))
                    {
                        // Placeholder span — the page's JS would inject the C# delimiter ".".
                        sb.Append('.');
                        continue;
                    }
                    Walk(el);
                }
            }
        }
    }

    static string ExtractRowSummary(IElement summaryCell)
    {
        // Cell can be either a <div class="summary">…</div> wrapper, or raw text containing
        // an "(Overrides …)" suffix. We pick the first .summary div content if present,
        // else the whole cell's text. Use CleanInlineText to strip inline <script> and
        // language-placeholder spans (e.g. (Overrides Object.Equals(Object).) renders with
        // an LST language placeholder for the dot).
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
