// MemberPageParser — parse a single RhinoCommon Sandcastle per-member documentation page
// (constructor/method/property/event/operator/field) and return real signature + parameter
// + return data to enrich the type-page row that produced it.
//
// Background
// ----------
// RhinoCommon docs are Sandcastle-generated (same template family as Tekla but visually different).
// Two pages per member:
//
//   • The TYPE PAGE  /html/T_<Namespace>_<TypeName>.htm
//     Lists members in <table class="members"> rows with one-line summaries. NO param/return data.
//
//   • The MEMBER PAGE  /html/(M_|P_|E_|F_|Overload_)<Namespace>_<TypeName>_<Member>.htm
//     One member per page. Layout matches Tekla:
//       - Syntax block <div id="..._code_Div1" class="codeSnippetContainerCode"> <pre> with the
//         full C# signature.
//       - <h4 class="subHeading">Parameters</h4>     followed by <dl> of <dt>name</dt><dd>type</dd>.
//       - <h4 class="subHeading">Return Value</h4>   followed by "Type: ...<br>doc".
//       - <h4 class="subHeading">Property Value</h4> on property pages.
//       - <h4 class="subHeading">Field Value</h4>    on field pages (same shape as Property Value).
//       - <h4 class="subHeading">Value</h4>          on event pages (delegate type).
//       - <span class="collapsibleRegionTitle">Exceptions</span> + table.
//       - <span class="collapsibleRegionTitle">Remarks</span> + free text.
//
// Verified live 2026-05-19 against several RhinoCommon member pages on branches 7 and gh-pages —
// see EXTRACTION-NOTES.md.
//
// Differences vs Tekla:
// ---------------------
//   1. Title kinds: RhinoCommon adds " Operator" (operator overloads) and " Field" (public fields).
//      Operators are parsed as methods, fields are parsed as properties with `setter=false`.
//   2. LST language-specific text placeholders use the same `cs=X|vb=Y|cpp=Z|nu=W` pattern as Tekla,
//      but RhinoCommon emits operator headers like `LSTxxx_0?cpp=::|nu=.` — only `cpp=` and `nu=`,
//      no `cs=`. The neutral fallback `nu=` is semantically equivalent to the C# rendering for
//      separator-style placeholders, so we use it when `cs=` is absent. Without this fallback the
//      type names in operator pages would collapse: `Point3d.Addition` would render as `Point3dAddition`.
//      The Tekla implementation defaulted to "" for missing `cs=`, which is correct for the rare
//      Tekla case (only `cpp=%` for `out`-param markers) but wrong for Sandcastle's cs-omitted
//      separator pattern. The order is: prefer cs=, fall back to nu=, then "" — this preserves
//      Tekla's correct empty-string behaviour for `cpp=%`-only cases while also rendering
//      `?cpp=::|nu=.` as `.`.

using AngleSharp.Dom;
using AngleSharp.Html.Parser;
using System.Text;
using System.Text.RegularExpressions;

namespace AwareSidecar.Ingest.Extractors.Rhino;

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
    /// Parse a method, constructor, or operator page. Returns null if the page is not a
    /// recognised member page (its h1 doesn't end in " Method" / " Constructor" / " Operator").
    /// </summary>
    public static MethodEnrichment? ParseMethod(string html, bool isCtor)
    {
        using var doc = _parser.ParseDocument(html);
        var content = doc.QuerySelector("div.topicContent#TopicContent");
        if (content is null) return null;
        var h1 = content.QuerySelector("h1");
        if (h1 is null) return null;
        var title = CleanInlineText(h1);

        var sansTrailingSig = Regex.Replace(title, @"\s*\([^)]*\)\s*$", "");
        if (isCtor)
        {
            if (!sansTrailingSig.EndsWith(" Constructor", StringComparison.Ordinal)) return null;
        }
        else
        {
            // Methods + operators + implicit/explicit conversions all come through this path. We
            // exclude " Methods" (plural — overview index pages); the accepted singular forms are:
            //   • " Method"     — regular instance/static methods
            //   • " Operator"   — operator overloads (Addition, Equality, etc.)
            //   • " Conversion" — implicit/explicit user-defined conversions. Sandcastle renders
            //     these with a leading LST tag whose `nu=` reads "Explicit" / "Implicit" — the
            //     trailing kind word is always " Conversion (FromType to ToType)" regardless.
            var endsWithMethod = sansTrailingSig.EndsWith(" Method", StringComparison.Ordinal);
            var endsWithOperator = sansTrailingSig.EndsWith(" Operator", StringComparison.Ordinal);
            var endsWithConversion = sansTrailingSig.EndsWith(" Conversion", StringComparison.Ordinal);
            if (!endsWithMethod && !endsWithOperator && !endsWithConversion) return null;
        }

        var sigBlock = ExtractCSharpSyntaxBlock(content);
        var sig = sigBlock is null ? "" : NormaliseSignatureWhitespace(sigBlock);

        var paramList = ExtractParameters(content);
        var returns = ExtractReturnValue(content);
        var throws = ExtractExceptions(content);
        var remarks = ExtractRemarks(content);

        return new MethodEnrichment(sig, paramList, returns, throws, remarks);
    }

    /// <summary>
    /// Parse a property OR field page. Both produce a typed read-only/read-write value. Fields
    /// from RhinoCommon are conceptually read-only (Sandcastle pages don't show field setters)
    /// so we report `setter=false` whenever the page is a field.
    /// </summary>
    public static PropertyEnrichment? ParseProperty(string html)
    {
        using var doc = _parser.ParseDocument(html);
        var content = doc.QuerySelector("div.topicContent#TopicContent");
        if (content is null) return null;
        var h1 = content.QuerySelector("h1");
        if (h1 is null) return null;
        var title = CleanInlineText(h1);
        var sansTrailingSig = Regex.Replace(title, @"\s*\([^)]*\)\s*$", "");
        var isProperty = sansTrailingSig.EndsWith(" Property", StringComparison.Ordinal);
        var isField = sansTrailingSig.EndsWith(" Field", StringComparison.Ordinal);
        if (!isProperty && !isField) return null;

        // Type heading varies per kind. Both encode "Type: X" under the heading body.
        // For fields RhinoCommon uses "Field Value"; properties use "Property Value" or
        // (for inherited/override properties) "Return Value".
        string? propType;
        if (isField)
        {
            propType = ExtractTypedSection(content, "Field Value")
                       ?? ExtractTypedSection(content, "Property Value")
                       ?? ExtractTypedSection(content, "Return Value")
                       ?? "object";
        }
        else
        {
            propType = ExtractTypedSection(content, "Property Value")
                       ?? ExtractTypedSection(content, "Return Value")
                       ?? "object";
        }

        var sigBlock = ExtractCSharpSyntaxBlock(content) ?? "";
        var sigCompact = Regex.Replace(sigBlock, @"\s+", "");
        var hasGetter = Regex.IsMatch(sigCompact, @"\{[^}]*\bget\b", RegexOptions.IgnoreCase);
        var hasSetter = Regex.IsMatch(sigCompact, @"\{[^}]*\bset\b", RegexOptions.IgnoreCase);
        if (isField)
        {
            // Fields are read-only at the API surface (you can read the value, you can't
            // hot-swap it from outside the type). Always emit getter=true, setter=false.
            hasGetter = true;
            hasSetter = false;
        }
        else if (!hasGetter && !hasSetter)
        {
            hasGetter = true;
        }

        var remarks = ExtractRemarks(content);

        return new PropertyEnrichment(propType, hasGetter, hasSetter, remarks);
    }

    /// <summary>
    /// Parse an event page. Returns null if the page is not an event page.
    /// </summary>
    public static EventEnrichment? ParseEvent(string html)
    {
        using var doc = _parser.ParseDocument(html);
        var content = doc.QuerySelector("div.topicContent#TopicContent");
        if (content is null) return null;
        var h1 = content.QuerySelector("h1");
        if (h1 is null) return null;
        var title = CleanInlineText(h1);
        if (!title.EndsWith(" Event", StringComparison.Ordinal)) return null;

        var sigBlock = ExtractCSharpSyntaxBlock(content) ?? "";
        var sig = NormaliseSignatureWhitespace(sigBlock);

        var delegateType = ExtractTypedSection(content, "Value");
        if (string.IsNullOrEmpty(delegateType))
        {
            var m = Regex.Match(sig, @"\bevent\s+(\S+)\s+\S+\s*$");
            delegateType = m.Success ? m.Groups[1].Value : "EventHandler";
        }

        var remarks = ExtractRemarks(content);

        return new EventEnrichment(delegateType, sig, remarks);
    }

    // ── C# syntax block extraction ─────────────────────────────────────────

    static string? ExtractCSharpSyntaxBlock(IElement content)
    {
        var syntaxRegion = FindCollapsibleSection(content, "Syntax");
        IElement? div = null;
        if (syntaxRegion is not null)
        {
            div = syntaxRegion.QuerySelectorAll("div.codeSnippetContainerCode")
                .FirstOrDefault(d => (d.GetAttribute("id") ?? "").EndsWith("_code_Div1", StringComparison.Ordinal));
        }
        div ??= content.QuerySelectorAll("div.codeSnippetContainerCode")
            .FirstOrDefault(d => (d.GetAttribute("id") ?? "").EndsWith("_code_Div1", StringComparison.Ordinal));

        var pre = div?.QuerySelector("pre");
        return pre is null ? null : CleanInlineText(pre);
    }

    // ── parameters table extraction ────────────────────────────────────────

    static List<ParamInfo> ExtractParameters(IElement content)
    {
        var result = new List<ParamInfo>();
        var heading = FindSubHeading(content, "Parameters");
        if (heading is null) return result;

        IElement? dl = heading.NextElementSibling;
        for (int i = 0; i < 3 && dl is not null && dl.LocalName != "dl"; i++) dl = dl.NextElementSibling;
        if (dl is null || dl.LocalName != "dl") return result;

        IElement? dt = dl.FirstElementChild;
        while (dt is not null)
        {
            if (dt.LocalName != "dt") { dt = dt.NextElementSibling; continue; }
            var nameSpan = dt.QuerySelector("span.parameter");
            var paramName = nameSpan is null ? CleanInlineText(dt).Trim() : CleanInlineText(nameSpan).Trim();
            var dtText = CleanInlineText(dt);
            var isOptional = dtText.Contains("(Optional)", StringComparison.Ordinal);

            var dd = dt.NextElementSibling;
            while (dd is not null && dd.LocalName != "dd") dd = dd.NextElementSibling;
            string typeText = "";
            string? docText = null;
            if (dd is not null)
            {
                var (t, d) = ExtractTypeAndDocFromDd(dd);
                typeText = t;
                docText = d;
            }

            result.Add(new ParamInfo(
                name: paramName,
                type: string.IsNullOrEmpty(typeText) ? "object" : typeText,
                doc: docText,
                optional: isOptional,
                @default: null));

            dt = dd?.NextElementSibling ?? dt.NextElementSibling;
        }
        return result;
    }

    static (string type, string? doc) ExtractTypeAndDocFromDd(IElement dd)
    {
        var (beforeBr, afterBr) = RenderWithBrSplit(dd);
        var typePart = Regex.Replace(beforeBr, @"^\s*Type\s*:\s*", "").Trim();
        typePart = Regex.Replace(typePart, @"\s+", "");
        var doc = NormaliseWhitespace(afterBr);
        return (typePart, string.IsNullOrEmpty(doc) ? null : doc);
    }

    // ── Return Value / Property Value / Field Value / Value extraction ─────

    static ReturnInfo? ExtractReturnValue(IElement content)
    {
        var (type, doc) = ExtractTypedAndDoc(content, "Return Value");
        if (string.IsNullOrEmpty(type)) return null;
        return new ReturnInfo(type, doc ?? "");
    }

    static string? ExtractTypedSection(IElement content, string headingText)
    {
        var (type, _) = ExtractTypedAndDoc(content, headingText);
        return string.IsNullOrEmpty(type) ? null : type;
    }

    static (string type, string? doc) ExtractTypedAndDoc(IElement content, string headingText)
    {
        var heading = FindSubHeading(content, headingText);
        if (heading is null) return ("", null);

        var siblings = new List<INode>();
        var node = heading.NextSibling;
        while (node is not null)
        {
            if (node is IElement el)
            {
                if (el.LocalName == "h4" && el.ClassList.Contains("subHeading")) break;
                if (el.LocalName == "div" && el.ClassList.Contains("collapsibleAreaRegion")) break;
                if (el.LocalName == "div" && (el.GetAttribute("id") ?? "").EndsWith("Section", StringComparison.Ordinal)
                    && el.ClassList.Contains("collapsibleSection")) break;
            }
            siblings.Add(node);
            node = node.NextSibling;
        }

        var (beforeBr, afterBr) = RenderSequenceWithBrSplit(siblings);
        var typePart = Regex.Replace(beforeBr, @"^\s*Type\s*:\s*", "");
        typePart = Regex.Replace(typePart, @"\s+", "").Trim();
        var doc = NormaliseWhitespace(afterBr);
        return (typePart, string.IsNullOrEmpty(doc) ? null : doc);
    }

    // ── Exceptions extraction ──────────────────────────────────────────────

    static List<ThrowsInfo> ExtractExceptions(IElement content)
    {
        var result = new List<ThrowsInfo>();
        var region = FindCollapsibleSection(content, "Exceptions");
        if (region is null) return result;
        var table = region.QuerySelector("table");
        if (table is null) return result;
        foreach (var tr in table.QuerySelectorAll("tr"))
        {
            if (tr.QuerySelector("th") != null) continue;
            var cells = tr.QuerySelectorAll("td").ToList();
            if (cells.Count < 2) continue;
            var exType = NormaliseWhitespace(CleanInlineText(cells[0]));
            var when = NormaliseWhitespace(CleanInlineText(cells[1]));
            if (string.IsNullOrEmpty(exType)) continue;
            result.Add(new ThrowsInfo(exType, when));
        }
        return result;
    }

    // ── Remarks extraction ─────────────────────────────────────────────────

    static string? ExtractRemarks(IElement content)
    {
        var region = FindCollapsibleSection(content, "Remarks");
        if (region is null) return null;
        var clone = (IElement)region.Clone();
        foreach (var code in clone.QuerySelectorAll("div.codeSnippetContainer").ToList())
            code.Remove();
        var text = NormaliseWhitespace(CleanInlineText(clone));
        return string.IsNullOrEmpty(text) ? null : text;
    }

    // ── Section finders ────────────────────────────────────────────────────

    static IElement? FindSubHeading(IElement content, string text) =>
        content.QuerySelectorAll("h4.subHeading")
            .FirstOrDefault(h => string.Equals(h.TextContent.Trim(), text, StringComparison.Ordinal));

    static IElement? FindCollapsibleSection(IElement content, string title)
    {
        foreach (var titleSpan in content.QuerySelectorAll("span.collapsibleRegionTitle"))
        {
            var t = NormaliseWhitespace(titleSpan.TextContent);
            if (string.Equals(t, title, StringComparison.Ordinal))
            {
                var section = titleSpan.ParentElement?.NextElementSibling;
                if (section is not null && section.ClassList.Contains("collapsibleSection"))
                    return section;
            }
        }
        return null;
    }

    // ── inline text rendering with LST placeholder resolution ──────────────

    internal static string CleanInlineText(IElement root)
    {
        var sb = new StringBuilder();
        Walk(root, sb);
        return NormaliseWhitespace(sb.ToString());
    }

    static void Walk(IElement node, StringBuilder sb)
    {
        if (node.LocalName == "script") return;
        IElement? pendingLstSpan = null;

        foreach (var child in node.ChildNodes)
        {
            if (child.NodeType == NodeType.Text)
            {
                sb.Append(child.TextContent);
                continue;
            }
            if (child is not IElement el) continue;

            if (el.LocalName == "script")
            {
                if (pendingLstSpan is not null)
                {
                    var cs = ExtractCsVariant(el.TextContent, pendingLstSpan.GetAttribute("id") ?? "");
                    sb.Append(cs);
                    pendingLstSpan = null;
                }
                continue;
            }

            var id = el.GetAttribute("id") ?? "";
            if (id.StartsWith("LST", StringComparison.Ordinal) && el.ChildNodes.Length == 0)
            {
                pendingLstSpan = el;
                continue;
            }

            Walk(el, sb);
        }

        if (pendingLstSpan is not null) sb.Append('.');
    }

    /// <summary>
    /// Parse the C# variant from a Sandcastle AddLanguageSpecificTextSet script body. The script
    /// body is e.g. <c>AddLanguageSpecificTextSet("LSTABC_0?cs=.|vb=.|cpp=::|nu=.|fs=.");</c>.
    ///
    /// Resolution order:
    ///   1. `cs=` (C# rendering) — preferred when present
    ///   2. `nu=` (neutral rendering) — fallback for separator-style placeholders that
    ///      Sandcastle's RhinoCommon emits as `cpp=::|nu=.` on operator pages
    ///   3. empty string — for language-only markers that have no C# / neutral equivalent
    ///      (e.g. `cpp=%` for an `out`-param indirection marker)
    /// </summary>
    static string ExtractCsVariant(string scriptText, string expectedSpanId)
    {
        if (!string.IsNullOrEmpty(expectedSpanId) && !scriptText.Contains(expectedSpanId, StringComparison.Ordinal))
            return ".";

        var m = Regex.Match(scriptText, @"[?|]cs=([^|""]*)");
        if (!m.Success)
        {
            // No cs= key — try the neutral rendering as a fallback. This is the dominant
            // pattern on RhinoCommon operator pages where the operator name separator only
            // has `cpp=::` and `nu=.` keys. The neutral rendering is the same as what cs=
            // would emit for these separator-only placeholders.
            var mNu = Regex.Match(scriptText, @"[?|]nu=([^|""]*)");
            if (!mNu.Success) return "";
            return DecodeEntities(mNu.Groups[1].Value);
        }
        return DecodeEntities(m.Groups[1].Value);
    }

    static string DecodeEntities(string raw) =>
        raw.Replace("&lt;", "<", StringComparison.Ordinal)
           .Replace("&gt;", ">", StringComparison.Ordinal)
           .Replace("&amp;", "&", StringComparison.Ordinal)
           .Replace("&quot;", "\"", StringComparison.Ordinal);

    static (string before, string after) RenderWithBrSplit(IElement parent)
    {
        return RenderSequenceWithBrSplit(parent.ChildNodes.ToList());
    }

    static (string before, string after) RenderSequenceWithBrSplit(IReadOnlyList<INode> nodes)
    {
        var before = new StringBuilder();
        var after = new StringBuilder();
        bool past = false;
        IElement? pendingLstSpan = null;

        foreach (var n in nodes)
        {
            var target = past ? after : before;
            if (n.NodeType == NodeType.Text)
            {
                target.Append(n.TextContent);
                continue;
            }
            if (n is not IElement el) continue;

            if (el.LocalName == "br")
            {
                past = true;
                pendingLstSpan = null;
                continue;
            }
            if (el.LocalName == "script")
            {
                if (pendingLstSpan is not null)
                {
                    var cs = ExtractCsVariant(el.TextContent, pendingLstSpan.GetAttribute("id") ?? "");
                    target.Append(cs);
                    pendingLstSpan = null;
                }
                continue;
            }
            var id = el.GetAttribute("id") ?? "";
            if (id.StartsWith("LST", StringComparison.Ordinal) && el.ChildNodes.Length == 0)
            {
                pendingLstSpan = el;
                continue;
            }
            var sb = new StringBuilder();
            Walk(el, sb);
            target.Append(sb);
        }
        return (before.ToString(), after.ToString());
    }

    // ── whitespace + signature shaping ─────────────────────────────────────

    static string NormaliseSignatureWhitespace(string s)
    {
        s = Regex.Replace(s, @"\s+", " ").Trim();
        s = Regex.Replace(s, @"\s*,\s*", ", ");
        s = Regex.Replace(s, @"\(\s+", "(");
        s = Regex.Replace(s, @"\s+\)", ")");
        return s;
    }

    static string NormaliseWhitespace(string s) =>
        Regex.Replace(s, @"\s+", " ").Trim();
}
