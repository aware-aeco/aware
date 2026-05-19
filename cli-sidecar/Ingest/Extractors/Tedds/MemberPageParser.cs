// MemberPageParser — parse a single Tekla Tedds per-member documentation page
// (constructor/method/property/event) and return real signature + parameter + return data
// to enrich the type-page row that produced it.
//
// Background
// ----------
// TSD member pages share the per-member-page convention. Each page typically holds ONE
// member, but constructor pages bundle ALL overloads on a single page (with multiple
// <h4 data-uid="...#ctor(...)"> blocks inside).
//
// Sections seen on member pages (verified 2026-05-19 against developer.tekla.com):
//
//   • <h1 ...>Method GetOutput</h1>       ← title carries the KIND keyword as first word
//                                            (Method | Constructor | Property | Event)
//                                            and the bare member name (NOT signature)
//   • <h4 data-uid="...">GetOutput(OutputFormat)</h4>   ← per-overload h4 with the
//                                                          disambiguating signature in parens
//   • <div class="markdown level1 summary">  ← per-overload summary
//   • <h5 class="declaration">Declaration</h5>
//     <div class="codewrapper"><pre><code class="language-cs">FULL C# SIGNATURE</code></pre></div>
//   • <h5 class="parameters">Parameters</h5>
//     <table>: 3 columns Type|Name|Description, one <tr> per param.
//   • <h5 class="returns">Returns</h5>      ← absent on void methods
//     <table>: 2 columns Type|Description, single <tr>.
//   • <h5 class="propertyValue">Property Value</h5>   ← on property pages
//     <table>: 2 columns Type|Description.
//   • <h5 class="exceptions">Exceptions</h5> ← on methods that throw documented exceptions
//     <table>: 2 columns Type|Condition.
//   • <h5 id="..._remarks">Remarks</h5>
//     <div class="markdown level1 remarks"><p>…</p></div>
//
// Format differences from Tekla Structures (Sandcastle):
//   - No <span id="LST..."/> + <script>AddLanguageSpecificTextSet(...)</script> placeholder
//     pairs — TSD uses plain UTF-8 with C-style angle brackets rendered as literal
//     `&lt;` and `&gt;` HTML entities that AngleSharp decodes for us.
//   - Section markers are <h5 class="declaration|parameters|returns|propertyValue|exceptions">
//     instead of <h4 class="subHeading">SectionName</h4>.
//   - Multiple overloads on one ctor page (handled by collecting EACH h4's declaration block).
//
// AOT note: pure managed parsing.

using AngleSharp.Dom;
using AngleSharp.Html.Parser;
using System.Text;
using System.Text.RegularExpressions;

namespace AwareSidecar.Ingest.Extractors.Tedds;

public static class MemberPageParser
{
    static readonly HtmlParser _parser = new();

    // ── Public surface ─────────────────────────────────────────────────────

    public sealed record MethodEnrichment(
        string signature,
        List<ParamInfo> @params,
        ReturnInfo? returns,
        List<ThrowsInfo> throws,
        string? remarks);

    public sealed record PropertyEnrichment(
        string type,
        bool getter,
        bool setter,
        string? remarks);

    public sealed record EventEnrichment(
        string @delegate,
        string signature,
        string? remarks);

    /// <summary>
    /// Parse a method or constructor page. The TYPE page row carries the disambiguated
    /// member-with-args text (e.g. "Move(int, int)") which is used to select WHICH overload
    /// inside a multi-overload member page is the target. We pass that hint via
    /// <paramref name="memberRowText"/>; for single-overload pages (most cases) the hint
    /// is ignored and the only overload is returned.
    /// </summary>
    public static MethodEnrichment? ParseMethod(string html, bool isCtor, string? memberRowText = null)
    {
        using var doc = _parser.ParseDocument(html);
        var content = doc.QuerySelector("article.content.wrap#_content");
        if (content is null) return null;
        var h1 = content.QuerySelector("h1");
        if (h1 is null) return null;
        var title = NormaliseWhitespace(h1.TextContent);
        // Title: "Method Move" or "Constructor AliasAttribute" — check the first word.
        if (isCtor)
        {
            if (!title.StartsWith("Constructor ", StringComparison.Ordinal)) return null;
        }
        else
        {
            if (!title.StartsWith("Method ", StringComparison.Ordinal)) return null;
        }

        // Find the H4 overload block that matches the member-row text. If no hint, take the first.
        var (overloadHeader, overloadEnd) = FindOverloadBlock(content, memberRowText);
        if (overloadHeader is null) return null;

        var sig = ExtractDeclarationSignature(content, overloadHeader, overloadEnd);
        if (string.IsNullOrEmpty(sig)) return null;
        sig = NormaliseSignatureWhitespace(sig);

        var paramList = ExtractParameters(content, overloadHeader, overloadEnd);
        var returns = ExtractReturn(content, overloadHeader, overloadEnd);
        var throws = ExtractExceptions(content, overloadHeader, overloadEnd);
        var remarks = ExtractRemarks(content, overloadHeader, overloadEnd);

        return new MethodEnrichment(sig, paramList, returns, throws, remarks);
    }

    /// <summary>
    /// Parse a property page. Properties don't have overloads — first/only h4 is used.
    /// </summary>
    public static PropertyEnrichment? ParseProperty(string html)
    {
        using var doc = _parser.ParseDocument(html);
        var content = doc.QuerySelector("article.content.wrap#_content");
        if (content is null) return null;
        var h1 = content.QuerySelector("h1");
        if (h1 is null) return null;
        var title = NormaliseWhitespace(h1.TextContent);
        if (!title.StartsWith("Property ", StringComparison.Ordinal)) return null;

        var (overloadHeader, overloadEnd) = FindOverloadBlock(content, null);
        if (overloadHeader is null) return null;

        var sig = ExtractDeclarationSignature(content, overloadHeader, overloadEnd) ?? "";

        // Property type — prefer the dedicated Property Value section's first table row.
        // Fall back to Return if the doc author used the wrong heading (rare in TSD but
        // possible on overridden properties).
        var propType = ExtractTypedSectionType(content, overloadHeader, overloadEnd, "propertyValue")
                     ?? ExtractTypedSectionType(content, overloadHeader, overloadEnd, "returns")
                     ?? "object";

        // Getter/setter from the syntax block: look for `{ get; }` vs `{ get; set; }`.
        var sigCompact = Regex.Replace(sig, @"\s+", "");
        var hasGetter = Regex.IsMatch(sigCompact, @"\{[^}]*\bget\b", RegexOptions.IgnoreCase);
        var hasSetter = Regex.IsMatch(sigCompact, @"\{[^}]*\bset\b", RegexOptions.IgnoreCase);
        if (!hasGetter && !hasSetter) hasGetter = true;  // conservative: assume read-only.

        var remarks = ExtractRemarks(content, overloadHeader, overloadEnd);
        return new PropertyEnrichment(propType, hasGetter, hasSetter, remarks);
    }

    /// <summary>
    /// Parse an event page (rare in TSD — Tekla Tedds uses COM-style event interfaces with
    /// methods, not first-class events. Provided for completeness in case future versions
    /// add real .NET events).
    /// </summary>
    public static EventEnrichment? ParseEvent(string html)
    {
        using var doc = _parser.ParseDocument(html);
        var content = doc.QuerySelector("article.content.wrap#_content");
        if (content is null) return null;
        var h1 = content.QuerySelector("h1");
        if (h1 is null) return null;
        var title = NormaliseWhitespace(h1.TextContent);
        if (!title.StartsWith("Event ", StringComparison.Ordinal)) return null;

        var (overloadHeader, overloadEnd) = FindOverloadBlock(content, null);
        if (overloadHeader is null) return null;
        var sig = NormaliseSignatureWhitespace(ExtractDeclarationSignature(content, overloadHeader, overloadEnd) ?? "");

        // Delegate type from the typed-section (DocFX uses "eventType" or "returns" depending
        // on template version). Fall back to parsing it out of the signature.
        var delegateType = ExtractTypedSectionType(content, overloadHeader, overloadEnd, "eventType")
                        ?? ExtractTypedSectionType(content, overloadHeader, overloadEnd, "returns")
                        ?? ExtractDelegateFromSig(sig)
                        ?? "EventHandler";

        var remarks = ExtractRemarks(content, overloadHeader, overloadEnd);
        return new EventEnrichment(delegateType, sig, remarks);
    }

    static string? ExtractDelegateFromSig(string sig)
    {
        var m = Regex.Match(sig, @"\bevent\s+(\S+)\s+\S+\s*$");
        return m.Success ? m.Groups[1].Value : null;
    }

    // ── overload block discovery ───────────────────────────────────────────

    /// <summary>
    /// Find the per-overload &lt;h4&gt; that matches the row-text hint. Multi-overload pages
    /// have e.g. <c>&lt;h4&gt;Foo(string)&lt;/h4&gt;&lt;h5 class="declaration"&gt;...&lt;/h5&gt;</c>
    /// followed by <c>&lt;h4&gt;Foo(string, bool)&lt;/h4&gt;&lt;h5...</c>. We return the h4
    /// element + the NEXT h4 (the boundary at which sections for this overload end).
    /// Returns (null, null) if there are no h4 elements (the page has just the top-level h1).
    /// </summary>
    static (IElement? header, IElement? next) FindOverloadBlock(IElement content, string? memberRowText)
    {
        var h4s = content.QuerySelectorAll("h4").Where(h =>
        {
            // Skip h4s outside the content article (defensive — querySelector should already scope us).
            return true;
        }).ToList();
        if (h4s.Count == 0) return (null, null);

        IElement? picked = null;
        if (!string.IsNullOrEmpty(memberRowText))
        {
            // Match by the disambiguating text in the row.
            var rowNorm = NormaliseWhitespace(memberRowText);
            picked = h4s.FirstOrDefault(h => NormaliseWhitespace(h.TextContent) == rowNorm);
        }
        if (picked is null) picked = h4s[0];
        var idx = h4s.IndexOf(picked);
        IElement? next = idx + 1 < h4s.Count ? h4s[idx + 1] : null;
        return (picked, next);
    }

    /// <summary>
    /// Walk forward from <paramref name="from"/> (an element inside the content article),
    /// yielding every following sibling/descendant element UNTIL we reach <paramref name="endHeader"/>
    /// (the next overload's h4) or the end of content. Used to scope section searches to a
    /// single overload's block within a multi-overload page.
    /// </summary>
    static IEnumerable<IElement> WalkUntil(IElement from, IElement? endHeader)
    {
        // Iterate forward through following siblings of `from` until we hit endHeader.
        var node = from.NextElementSibling;
        while (node is not null && node != endHeader)
        {
            yield return node;
            node = node.NextElementSibling;
        }
    }

    // ── declaration signature extraction ───────────────────────────────────

    /// <summary>
    /// Find the &lt;h5 class="declaration"&gt; within the overload block and return its
    /// codewrapper's &lt;pre&gt;&lt;code&gt; text content.
    /// </summary>
    static string? ExtractDeclarationSignature(IElement content, IElement overloadHeader, IElement? overloadEnd)
    {
        IElement? h5 = null;
        foreach (var el in WalkUntil(overloadHeader, overloadEnd))
        {
            if (el.LocalName == "h5" && (el.GetAttribute("class") ?? "").Contains("declaration", StringComparison.Ordinal))
            {
                h5 = el;
                break;
            }
        }
        if (h5 is null) return null;
        // The codewrapper follows the h5 (immediate sibling or a couple of nodes later).
        var node = h5.NextElementSibling;
        for (int i = 0; i < 3 && node is not null; i++)
        {
            if (node.LocalName == "div" && (node.GetAttribute("class") ?? "").Contains("codewrapper", StringComparison.Ordinal))
            {
                var code = node.QuerySelector("pre code");
                return code?.TextContent;
            }
            node = node.NextElementSibling;
        }
        return null;
    }

    // ── parameters extraction ──────────────────────────────────────────────

    /// <summary>
    /// Find the &lt;h5 class="parameters"&gt; section within the overload block and parse
    /// its 3-column table:
    ///   <thead><tr><th>Type</th><th>Name</th><th>Description</th></tr></thead>
    ///   <tr>
    ///     <td><a class="xref">TypeName</a></td>
    ///     <td><span class="parametername">name</span></td>
    ///     <td><p>doc</p></td>
    ///   </tr>
    /// </summary>
    static List<ParamInfo> ExtractParameters(IElement content, IElement overloadHeader, IElement? overloadEnd)
    {
        var result = new List<ParamInfo>();
        var table = FindSectionTable(content, overloadHeader, overloadEnd, "parameters");
        if (table is null) return result;

        foreach (var tr in table.QuerySelectorAll("tr"))
        {
            if (tr.QuerySelector("th") != null) continue;
            var cells = tr.QuerySelectorAll("td").ToList();
            if (cells.Count < 2) continue;

            var paramType = NormaliseWhitespace(cells[0].TextContent);
            if (string.IsNullOrEmpty(paramType)) paramType = "object";

            var nameSpan = cells[1].QuerySelector("span.parametername");
            var paramName = NormaliseWhitespace(nameSpan?.TextContent ?? cells[1].TextContent);
            if (string.IsNullOrEmpty(paramName)) continue;

            var docText = cells.Count >= 3 ? NormaliseWhitespace(cells[2].TextContent) : "";

            result.Add(new ParamInfo(
                name: paramName,
                type: paramType,
                doc: string.IsNullOrEmpty(docText) ? null : docText,
                optional: false,   // TSD doesn't expose Optional separately; default-value parameters
                                   // appear in the syntax block (e.g. `format = OutputFormat.Rtf`).
                @default: null));
        }
        return result;
    }

    // ── return value extraction ────────────────────────────────────────────

    /// <summary>
    /// Find the &lt;h5 class="returns"&gt; section and parse its single-row 2-column table:
    /// Type | Description. Returns null if absent (void method).
    /// </summary>
    static ReturnInfo? ExtractReturn(IElement content, IElement overloadHeader, IElement? overloadEnd)
    {
        var table = FindSectionTable(content, overloadHeader, overloadEnd, "returns");
        if (table is null) return null;
        foreach (var tr in table.QuerySelectorAll("tr"))
        {
            if (tr.QuerySelector("th") != null) continue;
            var cells = tr.QuerySelectorAll("td").ToList();
            if (cells.Count < 1) continue;
            var t = NormaliseWhitespace(cells[0].TextContent);
            var d = cells.Count >= 2 ? NormaliseWhitespace(cells[1].TextContent) : "";
            if (string.IsNullOrEmpty(t)) continue;
            return new ReturnInfo(t, d);
        }
        return null;
    }

    /// <summary>
    /// Same as ExtractReturn but used for typed sections like Property Value or Event Type —
    /// returns just the Type string.
    /// </summary>
    static string? ExtractTypedSectionType(IElement content, IElement overloadHeader, IElement? overloadEnd, string sectionClass)
    {
        var table = FindSectionTable(content, overloadHeader, overloadEnd, sectionClass);
        if (table is null) return null;
        foreach (var tr in table.QuerySelectorAll("tr"))
        {
            if (tr.QuerySelector("th") != null) continue;
            var cells = tr.QuerySelectorAll("td").ToList();
            if (cells.Count < 1) continue;
            var t = NormaliseWhitespace(cells[0].TextContent);
            if (!string.IsNullOrEmpty(t)) return t;
        }
        return null;
    }

    // ── exceptions extraction ──────────────────────────────────────────────

    static List<ThrowsInfo> ExtractExceptions(IElement content, IElement overloadHeader, IElement? overloadEnd)
    {
        var result = new List<ThrowsInfo>();
        var table = FindSectionTable(content, overloadHeader, overloadEnd, "exceptions");
        if (table is null) return result;
        foreach (var tr in table.QuerySelectorAll("tr"))
        {
            if (tr.QuerySelector("th") != null) continue;
            var cells = tr.QuerySelectorAll("td").ToList();
            if (cells.Count < 1) continue;
            var exType = NormaliseWhitespace(cells[0].TextContent);
            var when = cells.Count >= 2 ? NormaliseWhitespace(cells[1].TextContent) : "";
            if (string.IsNullOrEmpty(exType)) continue;
            result.Add(new ThrowsInfo(exType, when));
        }
        return result;
    }

    // ── remarks extraction ─────────────────────────────────────────────────

    /// <summary>
    /// Find &lt;h5 id="..._remarks"&gt; or &lt;h5 class="remarks"&gt; within the overload
    /// block and read the following &lt;div class="markdown level1 remarks"&gt; (or sibling div).
    /// </summary>
    static string? ExtractRemarks(IElement content, IElement overloadHeader, IElement? overloadEnd)
    {
        IElement? h5 = null;
        foreach (var el in WalkUntil(overloadHeader, overloadEnd))
        {
            if (el.LocalName != "h5") continue;
            var id = el.GetAttribute("id") ?? "";
            var cls = el.GetAttribute("class") ?? "";
            if (id.EndsWith("_remarks", StringComparison.Ordinal) || cls.Contains("remarks", StringComparison.Ordinal))
            {
                h5 = el;
                break;
            }
            // Some pages use the text 'Remarks' directly as the h5 content.
            if (string.Equals(NormaliseWhitespace(el.TextContent), "Remarks", StringComparison.Ordinal))
            {
                h5 = el;
                break;
            }
        }
        if (h5 is null) return null;
        // Read the following markdown div (or any non-h5 element until the next h4/h5).
        var node = h5.NextElementSibling;
        var sb = new StringBuilder();
        while (node is not null && node != overloadEnd)
        {
            if (node.LocalName == "h4") break;
            if (node.LocalName == "h5") break;
            // Strip embedded code blocks so we report prose, not embedded examples.
            var clone = (IElement)node.Clone();
            foreach (var code in clone.QuerySelectorAll("div.codewrapper, pre, example").ToList())
                code.Remove();
            sb.Append(' ').Append(clone.TextContent);
            node = node.NextElementSibling;
        }
        var text = NormaliseWhitespace(sb.ToString());
        return string.IsNullOrEmpty(text) ? null : text;
    }

    // ── section helpers ────────────────────────────────────────────────────

    /// <summary>
    /// Find the table that follows an &lt;h5&gt; section header whose class contains
    /// <paramref name="sectionClass"/> (e.g. "parameters", "returns"), scoped to the
    /// overload block bounded by <paramref name="overloadEnd"/>.
    /// </summary>
    static IElement? FindSectionTable(IElement content, IElement overloadHeader, IElement? overloadEnd, string sectionClass)
    {
        IElement? h5 = null;
        foreach (var el in WalkUntil(overloadHeader, overloadEnd))
        {
            if (el.LocalName == "h5" && (el.GetAttribute("class") ?? "").Contains(sectionClass, StringComparison.Ordinal))
            {
                h5 = el;
                break;
            }
        }
        if (h5 is null) return null;
        var node = h5.NextElementSibling;
        for (int i = 0; i < 3 && node is not null; i++)
        {
            if (node.LocalName == "table") return node;
            node = node.NextElementSibling;
        }
        return null;
    }

    // ── signature shaping ──────────────────────────────────────────────────

    static string NormaliseSignatureWhitespace(string s)
    {
        s = Regex.Replace(s, @"\s+", " ").Trim();
        s = Regex.Replace(s, @"\s*,\s*", ", ");
        s = Regex.Replace(s, @"\(\s+", "(");
        s = Regex.Replace(s, @"\s+\)", ")");
        return s;
    }

    static string NormaliseWhitespace(string s) =>
        Regex.Replace(s ?? "", @"\s+", " ").Trim();
}
