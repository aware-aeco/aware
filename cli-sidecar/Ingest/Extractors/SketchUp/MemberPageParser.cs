// MemberPageParser — parse a single YARD method/property/event detail block from the
// SketchUp Ruby API documentation.
//
// Background
// ----------
// SketchUp's Ruby API docs are YARD-generated (https://yardoc.org). Unlike Sandcastle
// (Tekla/Rhino/Grasshopper), YARD packs ALL the members of a class/module onto ONE HTML
// page rather than spreading them across per-member pages. There is no separate
// "method-detail page" to fetch; every method, property pair, and observer callback is
// documented inline inside the parent type page.
//
// Layout of one method-detail block (verified live 2026-05-19 against ruby.sketchup.com):
//
//   <div class="method_details">
//     <h3 class="signature" id="MethodName-instance_method">
//       #<strong>method_name</strong>(arg1, arg2 = default)  &#x21d2; <tt>ReturnType</tt>
//     </h3>
//     <div class="docstring">
//       <div class="discussion">
//         <p>Description text...</p>
//       </div>
//     </div>
//     <div class="tags">
//       <div class="examples"> ... </div>
//       <p class="tag_title">Parameters:</p>
//       <ul class="param">
//         <li>
//           <span class='name'>arg1</span>
//           <span class='type'>(<tt>SomeType</tt>)</span>
//           &mdash;
//           <div class='inline'><p>doc</p></div>
//         </li>
//       </ul>
//       <p class="tag_title">Returns:</p>
//       <ul class="return">
//         <li>
//           <span class='type'>(<tt>ReturnType</tt>)</span>
//           &mdash;
//           <div class='inline'><p>return doc</p></div>
//         </li>
//       </ul>
//       <p class="tag_title">Raises:</p>
//       <ul class="raise">
//         <li>
//           <span class='type'>(<tt>ArgumentError</tt>)</span>
//           &mdash;
//           <div class='inline'><p>when condition</p></div>
//         </li>
//       </ul>
//       <p class="tag_title">Version:</p>
//       <ul class="version"><li><div class='inline'><p>SketchUp 6.0</p></div></li></ul>
//     </div>
//   </div>
//
// Method-name suffixes are preserved verbatim (Ruby idiom):
//   • predicate `?`  — `Sketchup.is_pro?`
//   • mutator   `!`  — `Geom::PolygonMesh#extend_uvq_arrays!`
//   • setter    `=`  — `Sketchup::Model#active_layer=`
//
// Class methods use `.method_name` in the signature h3 and end with `-class_method` in the id;
// instance methods use `#method_name` and `-instance_method`. Constructors are `#initialize`
// listed under <h2>Constructor Details</h2> and we treat them as a constructor (the IR type's
// constructors[] slot) rather than a method.

using AngleSharp.Dom;
using AngleSharp.Html.Parser;
using System.Text;
using System.Text.RegularExpressions;

namespace AwareSidecar.Ingest.Extractors.SketchUp;

public static class MemberPageParser
{
    static readonly HtmlParser _parser = new();

    // ── Public enrichment record types (mirror the Rhino/Tekla shape) ─────────

    public sealed record MethodEnrichment(
        string name,
        string signature,
        List<ParamInfo> @params,
        ReturnInfo? returns,
        List<ThrowsInfo> throws,
        string? remarks);

    public sealed record PropertyEnrichment(
        string name,
        string type,
        bool getter,
        bool setter,
        string? remarks);

    /// <summary>
    /// Parse a standalone HTML fragment containing exactly ONE method-detail block. Returns the
    /// enrichment record, or null if the fragment is not a method-detail block.
    ///
    /// This entry point is used by the test fixtures (one method per HTML file). The actual
    /// extractor calls <see cref="ParseMethodFromElement"/> directly with the already-located
    /// &lt;div class="method_details"&gt; element from the parent type page.
    /// </summary>
    public static MethodEnrichment? ParseMethod(string html, bool isCtor)
    {
        using var doc = _parser.ParseDocument(html);
        var block = FindMethodDetailsBlock(doc.DocumentElement);
        return block is null ? null : ParseMethodFromElement(block, isCtor);
    }

    /// <summary>
    /// Parse a standalone HTML fragment containing exactly ONE property pair. The "property" in
    /// the YARD/Ruby sense is a paired getter (#foo) + setter (#foo=) pair, which is two
    /// separate method_details blocks. Tests provide a fragment with BOTH adjacent blocks; this
    /// method parses just the getter (the first block) and reports getter=true,
    /// setter=&lt;does a setter block exist?&gt;.
    /// </summary>
    public static PropertyEnrichment? ParseProperty(string html)
    {
        using var doc = _parser.ParseDocument(html);
        var blocks = doc.QuerySelectorAll("div.method_details").ToList();
        if (blocks.Count == 0) return null;

        // Find the getter (the block whose method name does NOT end in `=`).
        IElement? getterBlock = null;
        IElement? setterBlock = null;
        foreach (var b in blocks)
        {
            var name = ExtractMethodNameFromBlock(b);
            if (name is null) continue;
            if (name.EndsWith("=", StringComparison.Ordinal)) setterBlock = b;
            else getterBlock = b;
        }
        if (getterBlock is null && setterBlock is null) return null;

        // If the fragment has only a setter (no getter), report it as a write-only property.
        var primary = getterBlock ?? setterBlock!;
        var getterName = ExtractMethodNameFromBlock(primary) ?? "";
        var bareName = getterName.TrimEnd('=');

        var enrichment = ParseMethodFromElement(primary, isCtor: false);
        if (enrichment is null) return null;

        string propType;
        if (getterBlock is not null)
        {
            // Type comes from the getter's return annotation.
            propType = enrichment.returns?.type ?? "Object";
        }
        else
        {
            // Setter-only — type comes from the single parameter.
            propType = enrichment.@params.Count > 0 ? enrichment.@params[0].type : "Object";
        }

        var hasGetter = getterBlock is not null;
        var hasSetter = setterBlock is not null;
        // YARD always documents getter on its own — pair detection happens at the page level.
        // For the test fixture-fragment path we cannot inspect the rest of the page; we trust
        // what's in the fragment.

        return new PropertyEnrichment(bareName, propType, hasGetter, hasSetter, enrichment.remarks);
    }

    /// <summary>
    /// Parse a single &lt;div class="method_details"&gt; block into a method enrichment. The
    /// caller is responsible for locating the block on the parent page; we operate purely on
    /// the element subtree.
    ///
    /// `isCtor` controls whether we treat the &lt;h3&gt; signature as a constructor:
    /// constructors emit `returns = null` (void) regardless of the YARD return annotation;
    /// methods emit a `ReturnInfo` parsed from the &lt;ul class="return"&gt; section.
    /// </summary>
    public static MethodEnrichment? ParseMethodFromElement(IElement block, bool isCtor)
    {
        var h3 = block.QuerySelector("h3.signature");
        if (h3 is null) return null;

        var (name, signature) = ExtractNameAndSignature(h3);
        if (string.IsNullOrEmpty(name)) return null;

        var docstring = ExtractDocstring(block);
        var paramList = ExtractParameters(block);
        var returns = isCtor ? null : ExtractReturnValue(block);
        var throws = ExtractRaises(block);

        // Pull defaults out of the signature for parameters that were defaulted in the Ruby
        // declaration (e.g. `def open(path, options = {})`). YARD renders the equals sign in
        // the h3 signature but does NOT separately call them out in the param tag list.
        var defaultsBySigName = ExtractParamDefaultsFromSignature(signature);
        if (defaultsBySigName.Count > 0)
        {
            for (int i = 0; i < paramList.Count; i++)
            {
                var p = paramList[i];
                if (defaultsBySigName.TryGetValue(p.name, out var def))
                {
                    paramList[i] = p with { optional = true, @default = def };
                }
            }
        }

        return new MethodEnrichment(name, signature, paramList, returns, throws, docstring);
    }

    // ── Helpers — exposed internal so PageParser can drive parsing per-block ──

    /// <summary>
    /// Locate the first &lt;div class="method_details"&gt; subtree on a fragment. Returns null
    /// if none exists.
    /// </summary>
    internal static IElement? FindMethodDetailsBlock(IElement? root)
    {
        if (root is null) return null;
        if (root.LocalName == "div" && root.ClassList.Contains("method_details")) return root;
        return root.QuerySelector("div.method_details");
    }

    /// <summary>
    /// Pull the bare method name (without the leading `#` or `.`) from a method_details block's
    /// h3.signature heading. Returns null if no name could be extracted. Trailing `?`, `!`, `=`
    /// are preserved (Ruby method-name idiom).
    /// </summary>
    internal static string? ExtractMethodNameFromBlock(IElement block)
    {
        var h3 = block.QuerySelector("h3.signature");
        if (h3 is null) return null;
        var (name, _) = ExtractNameAndSignature(h3);
        return string.IsNullOrEmpty(name) ? null : name;
    }

    /// <summary>
    /// Pull the bare method name + signature from an h3.signature heading. YARD emits one of
    /// two shapes depending on whether @overload tags are present:
    ///
    ///   No overloads (single signature):
    ///     <h3 class="signature" id="foo-instance_method">
    ///       #<strong>foo</strong>(arg = default)  &#x21d2; <tt>Type</tt>
    ///     </h3>
    ///
    ///   With overloads (multiple signatures):
    ///     <h3 class="signature" id="foo-instance_method">
    ///       <span class="overload">#<strong>foo</strong>(a)  &#x21d2; <tt>Type1</tt></span>
    ///       <span class="overload">#<strong>foo</strong>(a, b)  &#x21d2; <tt>Type2</tt></span>
    ///     </h3>
    ///
    /// Class methods use `.foo` (period prefix); instance methods use `#foo`. We strip the
    /// receiver marker. When overloads are present we concatenate their signatures joined by
    /// " | " so downstream catalog readers see the full overload set without losing any
    /// signature.
    /// </summary>
    internal static (string name, string signature) ExtractNameAndSignature(IElement h3)
    {
        // Overload shape: pick the strong in the FIRST overload span as the canonical name and
        // build the signature from each overload span separately, joining with " | ".
        var overloads = h3.QuerySelectorAll("span.overload").ToList();
        if (overloads.Count > 0)
        {
            var firstStrong = overloads[0].QuerySelector("strong");
            if (firstStrong is null) return ("", "");
            var name = NormaliseWhitespace(firstStrong.TextContent);

            var sigs = new List<string>(overloads.Count);
            foreach (var ov in overloads)
            {
                var sig = BuildSignatureFromContainer(ov, name);
                if (!string.IsNullOrEmpty(sig)) sigs.Add(sig);
            }
            return (name, string.Join(" | ", sigs));
        }

        // No overload spans — the strong sits directly inside the h3.
        var strong = h3.QuerySelector("strong");
        if (strong is null) return ("", "");
        var bareName = NormaliseWhitespace(strong.TextContent);
        var signature = BuildSignatureFromContainer(h3, bareName);
        return (bareName, signature);
    }

    /// <summary>
    /// Walk an h3 or overload-span container and build a `name(args) => ReturnType` style
    /// signature by concatenating the text content AFTER the &lt;strong&gt; tag. We preserve
    /// arguments + return type but drop the receiver marker (`#` / `.`) since the IR's `name`
    /// field already disambiguates instance vs class methods via the doc_url anchor suffix.
    /// </summary>
    static string BuildSignatureFromContainer(IElement container, string bareName)
    {
        var afterStrong = new StringBuilder();
        var sawStrong = false;
        // We DFS-walk the container, tracking when we've passed the FIRST <strong>. Everything
        // after that <strong> (text + descendant elements' text) contributes to the signature
        // tail. We deliberately keep this depth-aware rather than only walking direct children
        // because YARD's tail may include <tt>...</tt> elements containing nested <span><a>...
        // text — the visible "(arg) ⇒ Geom::Vector3d" is constructed from those nested tags.
        void Walk(INode n)
        {
            foreach (var child in n.ChildNodes)
            {
                if (!sawStrong)
                {
                    if (child is IElement el && el.LocalName == "strong")
                    {
                        sawStrong = true;
                        continue;
                    }
                    if (child is IElement childEl) Walk(childEl);
                    // text before <strong> is the receiver marker (`#` / `.`) — drop it.
                    continue;
                }
                if (child.NodeType == NodeType.Text)
                {
                    afterStrong.Append(child.TextContent);
                }
                else if (child is IElement el2)
                {
                    Walk(el2);
                }
            }
        }
        Walk(container);

        var tail = NormaliseWhitespace(afterStrong.ToString());
        // Replace U+21D2 (DOUBLE RIGHTWARDS ARROW) with `=>` for portability across tools that
        // can't render the unicode arrow cleanly.
        tail = tail.Replace("⇒", "=>");
        tail = Regex.Replace(tail, @"\s+", " ").Trim();

        if (string.IsNullOrEmpty(tail)) return bareName;
        // If tail starts with `(` we paste directly; otherwise insert a space so we don't glue
        // the name to the next token.
        return tail.StartsWith("(", StringComparison.Ordinal) ? $"{bareName}{tail}" : $"{bareName} {tail}";
    }

    // ── Docstring extraction ──────────────────────────────────────────────────

    internal static string? ExtractDocstring(IElement block)
    {
        // YARD renders docstring as <div class="docstring"> with <div class="discussion">
        // containing <p>s, <ul>s, and <div class="note">s. The block also has a TOP-LEVEL
        // docstring (the type-page summary) — that one is handled by PageParser. Here we want
        // the docstring INSIDE the method_details block.
        var docstring = block.QuerySelector("div.docstring");
        if (docstring is null) return null;
        var discussion = docstring.QuerySelector("div.discussion");
        if (discussion is null) return null;
        var clone = (IElement)discussion.Clone();
        // Drop code examples — they're rendered as <pre class="example code"> which would
        // produce huge blobs of inline Ruby code if we kept them. The IR stores narrative
        // remarks only; the catalog already carries doc_url for the user who wants the
        // examples.
        foreach (var code in clone.QuerySelectorAll("pre.example, pre.code").ToList()) code.Remove();
        var text = NormaliseWhitespace(clone.TextContent);
        return string.IsNullOrEmpty(text) ? null : text;
    }

    // ── Parameters table extraction ───────────────────────────────────────────

    internal static List<ParamInfo> ExtractParameters(IElement block)
    {
        var result = new List<ParamInfo>();
        var paramUl = block.QuerySelectorAll("ul.param").FirstOrDefault();
        if (paramUl is null) return result;

        foreach (var li in paramUl.Children.Where(c => c.LocalName == "li"))
        {
            var nameSpan = li.QuerySelector("span.name");
            if (nameSpan is null) continue;
            var name = NormaliseWhitespace(nameSpan.TextContent);
            if (string.IsNullOrEmpty(name)) continue;

            var typeText = ExtractTypeFromSpan(li);
            var doc = ExtractInlineDocFromLi(li);

            result.Add(new ParamInfo(
                name: name,
                type: string.IsNullOrEmpty(typeText) ? "Object" : typeText,
                doc: doc,
                optional: false,
                @default: null));
        }
        return result;
    }

    // ── Return / Property type extraction (used by ParseProperty pair logic) ──

    internal static ReturnInfo? ExtractReturnValue(IElement block)
    {
        var returnUl = block.QuerySelectorAll("ul.return").FirstOrDefault();
        if (returnUl is null) return null;
        var li = returnUl.Children.FirstOrDefault(c => c.LocalName == "li");
        if (li is null) return null;

        var typeText = ExtractTypeFromSpan(li);
        if (string.IsNullOrEmpty(typeText)) typeText = "Object";
        var doc = ExtractInlineDocFromLi(li) ?? "";
        return new ReturnInfo(typeText, doc);
    }

    // ── Raises extraction ─────────────────────────────────────────────────────

    internal static List<ThrowsInfo> ExtractRaises(IElement block)
    {
        var result = new List<ThrowsInfo>();
        var raiseUl = block.QuerySelectorAll("ul.raise").FirstOrDefault();
        if (raiseUl is null) return result;

        foreach (var li in raiseUl.Children.Where(c => c.LocalName == "li"))
        {
            var typeText = ExtractTypeFromSpan(li);
            if (string.IsNullOrEmpty(typeText)) continue;
            var when = ExtractInlineDocFromLi(li) ?? "";
            result.Add(new ThrowsInfo(typeText, when));
        }
        return result;
    }

    // ── Common helpers ────────────────────────────────────────────────────────

    /// <summary>
    /// Pull the type annotation out of a YARD param/return/raise &lt;li&gt;. YARD wraps the type
    /// in &lt;span class='type'&gt;(&lt;tt&gt;Type1&lt;/tt&gt;, &lt;tt&gt;Type2&lt;/tt&gt;)&lt;/span&gt;
    /// where the parens are LITERAL text inside the span (not nesting). We strip the parens and
    /// keep the inner text verbatim — multiple alternates remain comma-separated.
    /// </summary>
    internal static string ExtractTypeFromSpan(IElement li)
    {
        var typeSpan = li.QuerySelector("span.type");
        if (typeSpan is null) return "";
        var raw = NormaliseWhitespace(typeSpan.TextContent);
        // Strip the literal outer parens YARD writes.
        if (raw.StartsWith("(", StringComparison.Ordinal) && raw.EndsWith(")", StringComparison.Ordinal))
            raw = raw.Substring(1, raw.Length - 2).Trim();
        // Strip surrounding whitespace + collapse multi-space.
        return Regex.Replace(raw, @"\s+", " ");
    }

    /// <summary>
    /// Pull the &lt;div class='inline'&gt; description from a param/return/raise li. YARD wraps
    /// the inline doc after the &lt;span class='type'&gt; and a &amp;mdash; separator.
    /// </summary>
    internal static string? ExtractInlineDocFromLi(IElement li)
    {
        var inline = li.QuerySelector("div.inline");
        if (inline is null) return null;
        var text = NormaliseWhitespace(inline.TextContent);
        return string.IsNullOrEmpty(text) ? null : text;
    }

    /// <summary>
    /// Parse defaults out of a Ruby-style signature like `open(path, options = {}, mode = :read)`.
    /// Returns a name → default-value-string map. The default value is taken verbatim from the
    /// signature (everything after the `=` and before the next `,` or `)` at depth 0).
    /// </summary>
    internal static Dictionary<string, string> ExtractParamDefaultsFromSignature(string signature)
    {
        var result = new Dictionary<string, string>(StringComparer.Ordinal);
        var openParen = signature.IndexOf('(');
        var closeParen = signature.LastIndexOf(')');
        if (openParen < 0 || closeParen < 0 || closeParen <= openParen) return result;
        var args = signature.Substring(openParen + 1, closeParen - openParen - 1);
        // Split on top-level commas.
        var parts = SplitOnTopLevelCommas(args);
        foreach (var p in parts)
        {
            var eq = FindTopLevelEquals(p);
            if (eq < 0) continue;
            var name = p.Substring(0, eq).Trim();
            // Strip leading splat (`*`, `**`) and Ruby block prefix (`&`) so the map matches the
            // ul.param `<span class='name'>` which carries the bare identifier.
            name = name.TrimStart('*', '&').Trim();
            var def = p.Substring(eq + 1).Trim();
            if (!string.IsNullOrEmpty(name)) result[name] = def;
        }
        return result;
    }

    static List<string> SplitOnTopLevelCommas(string s)
    {
        var parts = new List<string>();
        var sb = new StringBuilder();
        int depth = 0;
        foreach (var ch in s)
        {
            if (ch is '(' or '[' or '{' or '<') { depth++; sb.Append(ch); continue; }
            if (ch is ')' or ']' or '}' or '>') { depth--; sb.Append(ch); continue; }
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

    static int FindTopLevelEquals(string s)
    {
        int depth = 0;
        for (int i = 0; i < s.Length; i++)
        {
            var ch = s[i];
            if (ch is '(' or '[' or '{' or '<') { depth++; continue; }
            if (ch is ')' or ']' or '}' or '>') { depth--; continue; }
            // Ruby's `==` and `===` are NOT defaults — we look only at single `=` at depth 0
            // with no `=` next to it on either side. (`!=`, `<=`, `>=` likewise excluded.)
            if (ch == '=' && depth == 0)
            {
                var prev = i > 0 ? s[i - 1] : ' ';
                var next = i + 1 < s.Length ? s[i + 1] : ' ';
                if (next == '=') return -1;   // `==` — treat as no default
                if (prev is '=' or '<' or '>' or '!') return -1;
                return i;
            }
        }
        return -1;
    }

    internal static string NormaliseWhitespace(string s) =>
        Regex.Replace(s ?? "", @"\s+", " ").Trim();
}
