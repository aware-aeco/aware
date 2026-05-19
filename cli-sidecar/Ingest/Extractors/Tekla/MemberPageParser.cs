// MemberPageParser — parse a single Tekla per-member documentation page
// (constructor/method/property/event) and return real signature + parameter
// + return data to enrich the type-page row that produced it.
//
// Background
// ----------
// Tekla's docs render TWO levels of pages per type:
//
//   • The TYPE PAGE  /doc/tekla-structures/.../<slug>-<id>      (or /topic/...)
//     Lists members in <table class="members"> rows. Each row carries a name
//     + one-line summary + a member link. NO type/param/return info.
//
//   • The MEMBER PAGE  /topic/en/18/47/<guid>
//     One member per page. Carries:
//       - Syntax block <div id="..._code_Div1" class="codeSnippetContainerCode"> <pre> with the
//         full C# signature (keywords + identifiers + LST placeholders).
//       - <h4 class="subHeading">Parameters</h4>     followed by a <dl> of <dt>name</dt><dd>type
//         + (Optional)? + <br>doc</dd> pairs.
//       - <h4 class="subHeading">Return Value</h4>   followed by "Type: <span class="nolink">…</span><br>doc".
//       - <h4 class="subHeading">Property Value</h4> (on property pages) same shape as Return Value.
//       - <h4 class="subHeading">Value</h4>          (on event pages) same shape, gives delegate type.
//       - <span class="collapsibleRegionTitle">Exceptions</span> + <table> of <tr><td>Type</td><td>Condition</td>.
//       - <span class="collapsibleRegionTitle">Remarks</span> + free-text section.
//
// Verified live 2026-05-18 against several Tekla member pages — see EXTRACTION-NOTES.md.
//
// Inline language placeholders
// ----------------------------
// Every dotted type ("System.Object"), every generic angle bracket ("List<T>"),
// every method-dispatch dot, and every namespace separator is emitted by Tekla
// as an empty `<span id="LST...">` followed by a `<script>AddLanguageSpecificTextSet(
// "LST...?cs=X|vb=Y|cpp=Z|nu=W|fs=V")</script>`. A real browser substitutes the
// C# variant `X` into the empty span. We do the substitution ourselves by
// extracting the C# variant from the script body. Tokens we see in the wild:
//   `cs=.`     namespace/type-member dot
//   `cs=<`     generic open
//   `cs=>`     generic close
// Critically these are language-encoded — the script body literally contains
// `&lt;` and `&gt;` HTML entities (since it's all within an HTML document),
// which AngleSharp decodes back to `<` and `>` automatically.
//
// AOT note: pure managed parsing, identical AOT story to PageParser.cs.

using AngleSharp.Dom;
using AngleSharp.Html.Parser;
using System.Text;
using System.Text.RegularExpressions;

namespace AwareSidecar.Ingest.Extractors.Tekla;

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
    /// Parse a method or constructor page. Returns null if the page is not a
    /// recognised member page (its <h1> doesn't end in " Method" / " Constructor").
    /// </summary>
    public static MethodEnrichment? ParseMethod(string html, bool isCtor)
    {
        using var doc = _parser.ParseDocument(html);
        var content = doc.QuerySelector("div.topicContent#TopicContent");
        if (content is null) return null;
        var h1 = content.QuerySelector("h1");
        if (h1 is null) return null;
        var title = CleanInlineText(h1);
        // Title shape: "TypeName.MemberName Method" or "TypeName Constructor" etc.
        // We accept any title ending in " Method" / " Constructor" (singular) — the page-level
        // overview pages end in plural " Methods" / " Constructor"s and are not real signatures.
        // Tekla's title format is verified against developer.tekla.com 2026-05-18.
        // Strip a trailing "(SignatureToken)" overload spec from the title before checking the
        // kind word — Tekla emits "Foo Method", "Foo Method (Bar)", "Foo Constructor",
        // "Foo Constructor (Baz)" etc. The bracketed spec is just an overload disambiguator
        // and is NOT part of the kind word.
        var sansTrailingSig = System.Text.RegularExpressions.Regex.Replace(title, @"\s*\([^)]*\)\s*$", "");
        if (isCtor)
        {
            if (!sansTrailingSig.EndsWith(" Constructor", StringComparison.Ordinal)) return null;
        }
        else
        {
            // Title contains " Method" (singular) but NOT plural " Methods" (overview pages).
            if (!sansTrailingSig.EndsWith(" Method", StringComparison.Ordinal)) return null;
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
    /// Parse a property page. Returns null if the page is not a property page.
    /// </summary>
    public static PropertyEnrichment? ParseProperty(string html)
    {
        using var doc = _parser.ParseDocument(html);
        var content = doc.QuerySelector("div.topicContent#TopicContent");
        if (content is null) return null;
        var h1 = content.QuerySelector("h1");
        if (h1 is null) return null;
        var title = CleanInlineText(h1);
        if (!title.EndsWith(" Property", StringComparison.Ordinal)) return null;

        // Property type comes from the typed-value subHeading section. Tekla's docs
        // are INCONSISTENT here — most property pages use `<h4>Property Value</h4>` (the
        // canonical heading per the docs template), but a significant minority (every
        // property documented as overriding a base property — `IsUpToDate`,
        // `ModificationTime`, etc.) use `<h4>Return Value</h4>` instead. Both encode
        // the same "Type: X" layout, so we fall back from one to the other rather than
        // letting the placeholder `object` leak through. Discovered while regenerating
        // the 2025 catalog after the codex-coverage FAIL fix — the strict-verify on
        // `BaseRebarModifier.IsUpToDate` (Boolean) was flagging it as a placeholder.
        var propType = ExtractTypedSection(content, "Property Value")
                       ?? ExtractTypedSection(content, "Return Value")
                       ?? "object";

        // Getter/setter from the C# syntax block. Look for `{ get; }` vs `{ get; set; }`.
        var sigBlock = ExtractCSharpSyntaxBlock(content) ?? "";
        // Strip whitespace+newlines+tabs so we match `{ get; }` / `{ get; set; }` reliably.
        var sigCompact = Regex.Replace(sigBlock, @"\s+", "");
        // Heuristics: if "set" appears as a keyword inside the property body, setter=true.
        // We're matching against the cleaned signature so the words are literal.
        var hasGetter = Regex.IsMatch(sigCompact, @"\{[^}]*\bget\b", RegexOptions.IgnoreCase);
        var hasSetter = Regex.IsMatch(sigCompact, @"\{[^}]*\bset\b", RegexOptions.IgnoreCase);
        // Fallback: most Tekla properties expose at least a getter; if neither matched
        // (e.g. unusual syntax block), default to (getter=true, setter=false) — a
        // documented read-only property is the more conservative assumption than
        // assuming write access without evidence.
        if (!hasGetter && !hasSetter) hasGetter = true;

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

        // Delegate type comes from "Value" subHeading section (events) — same shape as Return Value.
        var delegateType = ExtractTypedSection(content, "Value");
        if (string.IsNullOrEmpty(delegateType))
        {
            // Fallback: parse the delegate type out of the syntax string. Tekla emits
            // `public event <DelegateType> <EventName>` for almost every event.
            var m = Regex.Match(sig, @"\bevent\s+(\S+)\s+\S+\s*$");
            delegateType = m.Success ? m.Groups[1].Value : "EventHandler";
        }

        var remarks = ExtractRemarks(content);

        return new EventEnrichment(delegateType, sig, remarks);
    }

    // ── C# syntax block extraction ─────────────────────────────────────────

    /// <summary>
    /// Extract the body of the C# syntax block (`<div id="..._code_Div1" class="codeSnippetContainerCode">'s <pre>`).
    /// Returns the inline-cleaned text — keywords, identifiers, parameter names, LST placeholders all
    /// resolved to their C# variants. Returns null if the page has no C# syntax block (some delegate
    /// or interface pages have a different code block layout — caller decides what to do).
    /// </summary>
    static string? ExtractCSharpSyntaxBlock(IElement content)
    {
        // Some member pages have multiple codeSnippetContainerCode elements (Examples sections also use
        // the same class). We want the FIRST one whose id ends in "_code_Div1" — that's the Syntax
        // section. Examples-section code blocks have ids like "ID2EAAABA_code_Div1" but they appear
        // AFTER the Syntax block. Safer to anchor on the "Syntax" collapsibleRegion's child.
        var syntaxRegion = FindCollapsibleSection(content, "Syntax");
        IElement? div = null;
        if (syntaxRegion is not null)
        {
            div = syntaxRegion.QuerySelectorAll("div.codeSnippetContainerCode")
                .FirstOrDefault(d => (d.GetAttribute("id") ?? "").EndsWith("_code_Div1", StringComparison.Ordinal));
        }
        // Fallback: scan from the top of content (some delegate pages don't wrap Syntax in a collapsible).
        div ??= content.QuerySelectorAll("div.codeSnippetContainerCode")
            .FirstOrDefault(d => (d.GetAttribute("id") ?? "").EndsWith("_code_Div1", StringComparison.Ordinal));

        var pre = div?.QuerySelector("pre");
        return pre is null ? null : CleanInlineText(pre);
    }

    // ── parameters table extraction ────────────────────────────────────────

    /// <summary>
    /// Parse the "Parameters" section's &lt;dl&gt; block. Each parameter is a `<dt>name (Optional)?</dt>`
    /// followed by `<dd>Type: <span class="nolink">...</span><br>doc text</dd>`.
    /// </summary>
    static List<ParamInfo> ExtractParameters(IElement content)
    {
        var result = new List<ParamInfo>();
        // The Parameters subHeading is `<h4 class="subHeading">Parameters</h4>` and is followed
        // by a `<dl>` sibling.
        var heading = FindSubHeading(content, "Parameters");
        if (heading is null) return result;

        IElement? dl = heading.NextElementSibling;
        // Some pages have a stray <br> or wrapper between the heading and the dl — walk forward
        // for up to 3 siblings.
        for (int i = 0; i < 3 && dl is not null && dl.LocalName != "dl"; i++) dl = dl.NextElementSibling;
        if (dl is null || dl.LocalName != "dl") return result;

        IElement? dt = dl.FirstElementChild;
        while (dt is not null)
        {
            if (dt.LocalName != "dt") { dt = dt.NextElementSibling; continue; }
            // dt content: <span class="parameter">name</span> (Optional)?
            var nameSpan = dt.QuerySelector("span.parameter");
            var paramName = nameSpan is null ? CleanInlineText(dt).Trim() : CleanInlineText(nameSpan).Trim();
            var dtText = CleanInlineText(dt);
            var isOptional = dtText.Contains("(Optional)", StringComparison.Ordinal);

            // The matching <dd> immediately follows.
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

    /// <summary>
    /// Inside a parameter's `<dd>` cell: "Type: <span class="nolink">…</span><br>doc text".
    /// Splits on the FIRST <br> tag — everything before it (minus the "Type:" literal) is the type;
    /// everything after is the doc text. Uses RenderWithBrSplit so LST language placeholders
    /// embedded directly under <dd> (e.g. the `<` and `>` of `List<String>`) are resolved
    /// correctly even though they are not nested inside a .nolink span.
    /// </summary>
    static (string type, string? doc) ExtractTypeAndDocFromDd(IElement dd)
    {
        var (beforeBr, afterBr) = RenderWithBrSplit(dd);

        // Strip "Type:" prefix (and the U+00A0 non-breaking space Tekla sometimes uses after it).
        var typePart = Regex.Replace(beforeBr, @"^\s*Type\s*:\s*", "").Trim();
        typePart = Regex.Replace(typePart, @"\s+", "");
        var doc = NormaliseWhitespace(afterBr);
        return (typePart, string.IsNullOrEmpty(doc) ? null : doc);
    }

    // ── Return Value / Property Value / Value extraction ───────────────────

    /// <summary>
    /// Parse a typed section ("Return Value", "Property Value", "Value"). These all share the same
    /// shape: a "Type: ...<br>doc" block immediately following the subHeading element. Returns null
    /// if the section is absent (e.g. a void method has no Return Value section).
    /// </summary>
    static ReturnInfo? ExtractReturnValue(IElement content)
    {
        var (type, doc) = ExtractTypedAndDoc(content, "Return Value");
        if (string.IsNullOrEmpty(type)) return null;
        return new ReturnInfo(type, doc ?? "");
    }

    /// <summary>
    /// Same as ExtractReturnValue but returns the type string only — for property/event pages we
    /// only care about the type (the documentation prose lives in the page-level summary).
    /// </summary>
    static string? ExtractTypedSection(IElement content, string headingText)
    {
        var (type, _) = ExtractTypedAndDoc(content, headingText);
        return string.IsNullOrEmpty(type) ? null : type;
    }

    /// <summary>
    /// Walk forward from the named subHeading to collect "Type: …<br>doc" content until the next
    /// section boundary (next h4 or collapsibleAreaRegion or end of parent).
    /// </summary>
    static (string type, string? doc) ExtractTypedAndDoc(IElement content, string headingText)
    {
        var heading = FindSubHeading(content, headingText);
        if (heading is null) return ("", null);

        // The content after the heading is a mix of text nodes, inline spans, LST placeholder pairs,
        // and a <br> that splits the "Type:" line from the doc paragraph. Render the siblings
        // up to the next section boundary using the shared LST-aware renderer.
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

    /// <summary>
    /// Parse the "Exceptions" collapsible region's table: `<th>Exception</th><th>Condition</th>`
    /// header row followed by `<tr><td>Type</td><td>When</td></tr>` rows.
    /// </summary>
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

    /// <summary>
    /// Parse the "Remarks" collapsible region's free-text body. Returns null if the section is
    /// absent.
    /// </summary>
    static string? ExtractRemarks(IElement content)
    {
        var region = FindCollapsibleSection(content, "Remarks");
        if (region is null) return null;
        // Strip code-snippet containers (Examples-style embedded code) before reading text — we
        // want the prose, not the embedded code.
        var clone = (IElement)region.Clone();
        foreach (var code in clone.QuerySelectorAll("div.codeSnippetContainer").ToList())
            code.Remove();
        var text = NormaliseWhitespace(CleanInlineText(clone));
        return string.IsNullOrEmpty(text) ? null : text;
    }

    // ── Section finders ────────────────────────────────────────────────────

    /// <summary>
    /// Find a `<h4 class="subHeading">HEADING TEXT</h4>` element by its exact text content.
    /// </summary>
    static IElement? FindSubHeading(IElement content, string text) =>
        content.QuerySelectorAll("h4.subHeading")
            .FirstOrDefault(h => string.Equals(h.TextContent.Trim(), text, StringComparison.Ordinal));

    /// <summary>
    /// Find a `<div class="collapsibleSection">` whose preceding sibling `<span class="collapsibleRegionTitle">`
    /// has the given title text. Returns the .collapsibleSection div, not the title span.
    /// </summary>
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

    /// <summary>
    /// Render an element's inline text, resolving Tekla's language-specific text placeholders
    /// to their C# variant. The Tekla docs emit patterns like
    /// <c>&lt;span id="LST..."/&gt;&lt;script&gt;AddLanguageSpecificTextSet("LST...?cs=.|vb=.|cpp=::|...");&lt;/script&gt;</c>
    /// where the empty span is replaced by JS with the C# delimiter at render time.
    /// Without JS we read the script body, parse out the <c>cs=X</c> variant, and inject X.
    /// </summary>
    internal static string CleanInlineText(IElement root)
    {
        var sb = new StringBuilder();
        Walk(root, sb);
        return NormaliseWhitespace(sb.ToString());
    }

    static void Walk(IElement node, StringBuilder sb)
    {
        // Defensive: if a caller hands us a <script> element directly, do not emit its JS body.
        if (node.LocalName == "script") return;
        // We track LST placeholder ids that have been emitted so the FOLLOWING <script>
        // can be parsed for the C# variant and substituted in-place. The script appears
        // AS A LATER SIBLING after the placeholder span in the DOM, so we emit the
        // placeholder text WHEN WE SEE THE SCRIPT, not when we see the span — that way
        // the substituted char lands in the right position in the rendered text stream.
        IElement? pendingLstSpan = null;

        foreach (var child in node.ChildNodes)
        {
            if (child.NodeType == NodeType.Text)
            {
                sb.Append(child.TextContent);
                continue;
            }
            if (child is not IElement el) continue;

            // <script> following a pending LST placeholder: parse cs=X, append X.
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
                // Empty placeholder. Buffer it; the next <script> will tell us the C# variant.
                pendingLstSpan = el;
                continue;
            }

            // Any non-placeholder element: descend.
            Walk(el, sb);
        }

        // If a pending LST span had no following <script> (unusual), default to "."
        if (pendingLstSpan is not null) sb.Append('.');
    }

    /// <summary>
    /// Parse the C# variant from a Tekla AddLanguageSpecificTextSet script body. The script body
    /// is e.g. <c>AddLanguageSpecificTextSet("LSTABC_0?cs=.|vb=.|cpp=::|nu=.|fs=.");</c> — we want
    /// the value between <c>cs=</c> and the next <c>|</c> (or the closing quote). The HTML parser
    /// has already decoded entities, so we may see literal <c>&lt;</c> as <c>&lt;</c> still (script
    /// bodies are CDATA but Tekla's are inside a regular &lt;script type="text/javascript"&gt; tag,
    /// which AngleSharp treats as raw-text and does NOT decode entities). Hence we entity-decode
    /// the matched fragment ourselves.
    /// </summary>
    static string ExtractCsVariant(string scriptText, string expectedSpanId)
    {
        // Quick guard: if the script body doesn't carry our LST id, default to ".".
        // (A misaligned LST/script pair would otherwise pick up the wrong variant.)
        if (!string.IsNullOrEmpty(expectedSpanId) && !scriptText.Contains(expectedSpanId, StringComparison.Ordinal))
            return ".";

        var m = Regex.Match(scriptText, @"[?|]cs=([^|""]*)");
        // No `cs=` key at all: the LST is rendering a language-specific marker that has no
        // C# equivalent (e.g. `cpp=%` for an `out`-param indirection, `cpp=^` for a managed
        // pointer marker). The C# rendering is intentionally empty, not "." — emitting "."
        // contaminates the catalog with trailing dots (e.g. `Validation.` instead of `Validation`).
        // See codex-coverage v2 review of tekla-2025/2026 for the 0.015% method impact.
        if (!m.Success) return "";
        var raw = m.Groups[1].Value;
        // The script body is rendered as JS — entities are NOT pre-decoded by the HTML parser
        // (script bodies are RAW-text in HTML). So `&lt;` stays as `&lt;`. Decode manually for the
        // tokens we expect: < > & quotes — that's enough for any signature use.
        return raw
            .Replace("&lt;", "<", StringComparison.Ordinal)
            .Replace("&gt;", ">", StringComparison.Ordinal)
            .Replace("&amp;", "&", StringComparison.Ordinal)
            .Replace("&quot;", "\"", StringComparison.Ordinal);
    }

    /// <summary>
    /// Render the inline contents of <c>parent</c> up to the FIRST <c>&lt;br&gt;</c> child, and
    /// everything after. LST language placeholders that appear as direct children of <c>parent</c>
    /// (not just nested inside .nolink spans) are resolved by reading the next-sibling script's
    /// <c>cs=…</c> variant — this is the case for parameter <c>&lt;dd&gt;</c> cells where generic
    /// angle brackets appear AT the dd level rather than inside a child span.
    /// </summary>
    static (string before, string after) RenderWithBrSplit(IElement parent)
    {
        return RenderSequenceWithBrSplit(parent.ChildNodes.ToList());
    }

    /// <summary>
    /// Same as <see cref="RenderWithBrSplit"/> but renders an arbitrary sequence of sibling nodes.
    /// </summary>
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
                // <br> separates the "Type: ..." line from the doc paragraph. Flip the bucket.
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
            // Any other element: render through Walk (which handles nested LST/script pairs).
            var sb = new StringBuilder();
            Walk(el, sb);
            target.Append(sb);
        }
        return (before.ToString(), after.ToString());
    }

    // ── whitespace + signature shaping ─────────────────────────────────────

    /// <summary>
    /// Normalise inter-line whitespace inside a C# signature: collapse multiple spaces, drop tabs,
    /// drop newlines. Tekla's syntax `<pre>` uses literal newline-and-tab for multi-line signatures
    /// (e.g. four-parameter constructor) which would otherwise survive into the IR.
    /// </summary>
    static string NormaliseSignatureWhitespace(string s)
    {
        s = Regex.Replace(s, @"\s+", " ").Trim();
        // Drop space-before-comma and space-before-close-paren artefacts.
        s = Regex.Replace(s, @"\s*,\s*", ", ");
        s = Regex.Replace(s, @"\(\s+", "(");
        s = Regex.Replace(s, @"\s+\)", ")");
        return s;
    }

    static string NormaliseWhitespace(string s) =>
        Regex.Replace(s, @"\s+", " ").Trim();
}
