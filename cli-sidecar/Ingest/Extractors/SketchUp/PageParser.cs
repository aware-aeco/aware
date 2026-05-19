// PageParser — parse a single YARD class/module documentation page from the SketchUp Ruby
// API site (https://ruby.sketchup.com/) into a complete TypeInfo.
//
// Source: SketchUp Ruby API docs, YARD-generated (https://yardoc.org).
//
// Unlike Sandcastle-based extractors (Tekla/Rhino/Grasshopper) which spread each member onto
// its own page and require a two-pass crawl (type-page → per-member-page), YARD packs every
// method, attribute pair, constant, and class-method onto ONE PAGE per type. This means a
// single fetch + single parse produces a fully-populated TypeInfo. No member-URL bundle is
// needed; no enrichment pass is needed.
//
// Page layout (verified live 2026-05-19):
//
//   <h1>Class: Sketchup::Model</h1>    or    <h1>Module: Sketchup</h1>
//   <div class="box_info">                  ← "Inherits:" and other type-level metadata
//     <dl><dt>Inherits:</dt><dd><span class="inheritName">Object</span> ...</dd></dl>
//   </div>
//   <h2>Overview</h2>
//   <div class="docstring"><div class="discussion"><p>summary text...</p></div></div>
//
//   <h2>Defined Under Namespace</h2>        ← only on module pages; we ignore
//   <p class="children">
//     <strong class="modules">Modules:</strong> ...
//     <strong class="classes">Classes:</strong> ...
//   </p>
//
//   <h2>Constants for X</h2>                ← one h2 per constant group (or single h2
//                                              "Constant Summary" for unnamed groups)
//   <dl class="constants">
//     <dt id="VALUE_NAME-constant">Path::To::VALUE_NAME = literal_value
//       <a ...>#</a>
//     </dt>
//     <dd>summary text</dd>
//   </dl>
//
//   <h2>Class Method Summary</h2>          ← summary list of class methods
//   <ul class="summary">
//     <li class="public">.<strong>name</strong>(args) ⇒ <tt>Type</tt> <span class="summary_desc">...</span></li>
//   </ul>
//
//   <h2>Instance Method Summary</h2>       ← summary list of instance methods
//   <ul class="summary"> ... </ul>
//
//   <h2>Constructor Details</h2>
//   <div id="constructor_details" class="method_details_list">
//     <div class="method_details first"> ... </div>     ← single #initialize block
//   </div>
//
//   <h2>Class Method Details</h2>
//   <div id="class_method_details" class="method_details_list">
//     <div class="method_details"> ... </div>           ← one per class method
//     <div class="method_details"> ... </div>
//   </div>
//
//   <h2>Instance Method Details</h2>
//   <div id="instance_method_details" class="method_details_list">
//     <div class="method_details"> ... </div>           ← one per instance method
//     ...
//   </div>
//
// Ruby idioms we preserve in the IR `name` field:
//   • predicate methods end in `?`   (`is_pro?`, `valid?`)
//   • mutator methods end in `!`     (`extend_uvq_arrays!`)
//   • setter methods end in `=`      (`active_layer=`) — these are paired with the getter
//                                      (`active_layer`) to form an IR `property` with
//                                      getter/setter = true/true.
//
// IR mapping decisions:
//   • Ruby modules  → `kind: "interface"` (closest IR analog per the brief; Ruby modules don't
//                     map to .NET interfaces but the IR schema's allowed enum is class | struct |
//                     interface | enum | delegate | static-class).
//   • Ruby classes  → `kind: "class"`.
//   • #initialize   → `constructors[]`. The IR's `returns` is null (void) by IR convention even
//                     though YARD documents initialize as returning the type — that's the Ruby
//                     idiom (`new` returns the instance, `initialize` itself returns nil).
//   • Class methods → `methods[]`. The IR has no separate "static" slot; .NET reflection puts
//                     them in `methods[]` too, so this is consistent.
//   • Instance methods → `methods[]`. The IR mixes both kinds.
//   • Constants → folded into `properties[]` with `getter=true, setter=false`. Ruby constants
//                 are public read-only by language convention. SketchUp uses constants as enum
//                 surrogates (e.g. `Sketchup::Model::VERSION_2020`) so this gives downstream
//                 tooling something to discover.
//   • Observer "callback" methods (on Observer classes, names starting `on...`) — left in
//                                 `methods[]`. The IR's `events[]` schema doesn't fit Ruby
//                                 callbacks (no delegate type, no .NET threading model). We
//                                 don't try to force them into the wrong slot.
//   • Properties (getter/setter pairs) — detected when both `#foo` (no args, returns X) and
//                                 `#foo=` (one arg of type X, no return) appear on the same page.
//                                 The pair is collapsed into ONE PropertyInfo with both getters.
//                                 The standalone getter without a setter remains a method (it's
//                                 truer to the Ruby semantic — Ruby has no `attr_*` concept at
//                                 the API surface; users call methods that happen to be named
//                                 like getters).
//                                 NOTE: SketchUp consistently uses paired getter/setter for
//                                       state ("active_layer" + "active_layer=") and bare methods
//                                       for actions ("close_active") — the pair-detection rule
//                                       correctly catalogs state as `properties[]` and actions
//                                       as `methods[]`.

using AngleSharp.Dom;
using AngleSharp.Html.Parser;
using System.Text.Json;
using System.Text.RegularExpressions;

namespace AwareSidecar.Ingest.Extractors.SketchUp;

public static class PageParser
{
    static readonly HtmlParser _parser = new();

    public sealed record ParseResult(TypeInfo Type);

    /// <summary>
    /// Parse one SketchUp YARD class/module page into a complete TypeInfo. Returns null if the
    /// page is not a recognised type page (e.g. the alphabetic index, a file.* doc page, or any
    /// page whose &lt;h1&gt; does not start with "Class: " or "Module: ").
    /// </summary>
    public static ParseResult? Parse(string html, string sourceUrl)
    {
        using var doc = _parser.ParseDocument(html);
        var content = doc.QuerySelector("div#content") ?? doc.DocumentElement;
        var h1 = content.QuerySelector("h1");
        if (h1 is null) return null;
        // Compute the title from h1 but DROP YARD's note spans (`<span class="abstract note
        // title">Abstract</span>`, `<span class="deprecated note title">Deprecated</span>`, etc.)
        // — those are visual labels appended to the title, not part of the type name. Without
        // this filter we'd produce names like "PagesObserver Abstract" instead of
        // "PagesObserver".
        var h1Clone = (IElement)h1.Clone();
        foreach (var note in h1Clone.QuerySelectorAll("span.note.title").ToList()) note.Remove();
        var titleText = MemberPageParser.NormaliseWhitespace(h1Clone.TextContent);

        string? kind = null;
        string? fqName = null;
        if (titleText.StartsWith("Class: ", StringComparison.Ordinal))
        {
            kind = "class";
            fqName = titleText.Substring("Class: ".Length).Trim();
        }
        else if (titleText.StartsWith("Exception: ", StringComparison.Ordinal))
        {
            // YARD distinguishes Exception subclasses with a separate prefix. They're still
            // Ruby classes — we catalog them with `kind: "class"` and rely on the `@base`
            // chain (typically StandardError / RuntimeError / ArgumentError / etc.) to
            // signal exception-hood downstream. The IR schema has no separate exception
            // type, and folding them into `class` matches how the rest of the corpus
            // would consume them.
            kind = "class";
            fqName = titleText.Substring("Exception: ".Length).Trim();
        }
        else if (titleText.StartsWith("Module: ", StringComparison.Ordinal))
        {
            // Ruby modules don't map to .NET interfaces semantically but the IR schema's
            // allowed enum values are class | struct | interface | enum | delegate | static-class.
            // The brief calls out "interface" as the agreed analog for Ruby modules.
            kind = "interface";
            fqName = titleText.Substring("Module: ".Length).Trim();
        }
        else if (titleText.StartsWith("Top Level Namespace", StringComparison.Ordinal))
        {
            // Ruby's root namespace — we don't catalog it.
            return null;
        }
        else
        {
            return null;
        }
        if (string.IsNullOrEmpty(fqName)) return null;

        // Split Foo::Bar::Baz into namespace "Foo::Bar" + name "Baz".
        var (ns, name) = SplitNamespace(fqName);

        var summary = ExtractTopLevelSummary(content);
        if (string.IsNullOrEmpty(summary))
            summary = $"(No description provided in vendor docs for {fqName}.)";

        var (baseType, interfaces) = ExtractInheritance(content);

        var typeRemarks = ExtractTypeLevelRemarks(content);

        // Constructors: under <h2>Constructor Details</h2> / <div id="constructor_details">.
        var constructors = ExtractConstructorDetails(content, name);

        // Class methods: under <h2>Class Method Details</h2> / <div id="class_method_details">.
        // Instance methods: under <h2>Instance Method Details</h2> / <div id="instance_method_details">.
        // We collect all of them as raw methods, then pull pair-detected properties OUT of the
        // instance-method list before final assembly.
        var classMethods = ExtractMethodBlocks(content, "#class_method_details");
        var instanceMethods = ExtractMethodBlocks(content, "#instance_method_details");

        var (remainingInstanceMethods, properties) = ExtractPropertiesFromGetterSetterPairs(instanceMethods, sourceUrl);

        // Constants: each <dl class="constants"> on the page → properties[] with getter only.
        var constantProps = ExtractConstantsAsProperties(content, sourceUrl);
        // Constants are conceptually different from getter/setter properties; put them at the
        // end of the properties[] list to keep the natural getter/setter pairs first.
        var allProps = new List<PropertyInfo>(properties.Count + constantProps.Count);
        allProps.AddRange(properties);
        allProps.AddRange(constantProps);

        // Class methods are added verbatim. Each carries a "ClassName.method" signature shape via
        // doc_url anchor; we already stored them with the bare name and the rendered signature
        // from the h3.
        var allMethods = new List<MethodInfo>();
        allMethods.AddRange(classMethods);
        allMethods.AddRange(remainingInstanceMethods);

        var typeInfo = new TypeInfo(
            name: name,
            @namespace: ns,
            kind: kind,
            summary: MemberPageParser.NormaliseWhitespace(summary),
            remarks: typeRemarks,
            @base: baseType,
            interfaces: interfaces,
            doc_url: sourceUrl,
            constructors: constructors,
            methods: allMethods,
            properties: allProps,
            events: new List<EventInfo>(),
            enum_values: new List<EnumValueInfo>(),
            delegate_signature: null);

        return new ParseResult(typeInfo);
    }

    // ── Title splitting ───────────────────────────────────────────────────────

    /// <summary>
    /// Split "Foo::Bar::Baz" into namespace "Foo::Bar" + name "Baz". For top-level types (e.g.
    /// "Array", "LanguageHandler") the namespace is empty string "".
    /// </summary>
    static (string @namespace, string name) SplitNamespace(string fqName)
    {
        var lastSep = fqName.LastIndexOf("::", StringComparison.Ordinal);
        if (lastSep < 0) return ("", fqName);
        return (fqName.Substring(0, lastSep), fqName.Substring(lastSep + 2));
    }

    // ── Top-level summary (Overview section) ─────────────────────────────────

    static string ExtractTopLevelSummary(IElement content)
    {
        // The Overview section appears immediately after <h1>. YARD emits <h2>Overview</h2>
        // followed by <div class="docstring"><div class="discussion"><p>...</p></div></div>.
        // We grab the FIRST top-level docstring after h1; if none, fall back to empty.
        foreach (var docstring in content.QuerySelectorAll("div.docstring"))
        {
            // Skip docstrings nested inside method_details blocks.
            var p = docstring.ParentElement;
            bool inMember = false;
            while (p is not null && p != content)
            {
                if (p.LocalName == "div" && p.ClassList.Contains("method_details"))
                {
                    inMember = true;
                    break;
                }
                p = p.ParentElement;
            }
            if (inMember) continue;
            var discussion = docstring.QuerySelector("div.discussion");
            if (discussion is null) continue;
            var clone = (IElement)discussion.Clone();
            // Drop code examples — they'd otherwise produce huge blobs of inline Ruby.
            foreach (var code in clone.QuerySelectorAll("pre.example, pre.code").ToList()) code.Remove();
            var text = MemberPageParser.NormaliseWhitespace(clone.TextContent);
            if (!string.IsNullOrEmpty(text)) return text;
        }
        return "";
    }

    // ── Type-level remarks (Notes / known bugs in YARD) ───────────────────────

    static string? ExtractTypeLevelRemarks(IElement content)
    {
        // YARD renders type-level notes inside <p class="tag_title">Note:</p> + <div class='inline'>
        // and known bugs inside <p class="tag_title">Known Bugs:</p> + <ul class="bug">. The
        // overview's tags section sits directly after the <div class="docstring"> at type level
        // (before any "Defined Under Namespace" or "Constant Summary" headers).
        //
        // We try a coarse-grained capture: find the first <div class="tags"> child of <div
        // class="docstring">'s parent, after the Overview h2. If it contains anything that
        // looks like type-level remarks (note / known bug / version), render it as a single
        // line.
        //
        // For SketchUp's actual data, type-level remarks are rare — most class summaries are
        // self-contained. We keep the implementation simple: look for the first <div class="tags">
        // that is NOT inside a method_details block AND not inside the docstring itself.

        foreach (var tags in content.QuerySelectorAll("div.tags"))
        {
            var p = tags.ParentElement;
            bool inMember = false;
            while (p is not null && p != content)
            {
                if (p.LocalName == "div" && p.ClassList.Contains("method_details")) { inMember = true; break; }
                p = p.ParentElement;
            }
            if (inMember) continue;

            // Skip the tags block if it's empty / examples-only (we keep examples out of remarks).
            var clone = (IElement)tags.Clone();
            foreach (var code in clone.QuerySelectorAll("div.examples, pre.example, pre.code").ToList()) code.Remove();
            // Strip the "Version:" footer at type level — it's metadata, not narrative.
            foreach (var ul in clone.QuerySelectorAll("ul.version").ToList())
            {
                // Walk back to the preceding <p class="tag_title">Version:</p>.
                var prev = ul.PreviousElementSibling;
                if (prev is not null && prev.LocalName == "p" && prev.ClassList.Contains("tag_title"))
                    prev.Remove();
                ul.Remove();
            }
            var text = MemberPageParser.NormaliseWhitespace(clone.TextContent);
            return string.IsNullOrEmpty(text) ? null : text;
        }
        return null;
    }

    // ── Inheritance ───────────────────────────────────────────────────────────

    /// <summary>
    /// Pull (base, interfaces[]) from the &lt;div class="box_info"&gt; block at the top of a
    /// YARD class page. SketchUp's Ruby classes are single-inheritance + don't include modules
    /// in the inheritance dl; we report just the base class. Modules ("Module: X") have no base.
    /// </summary>
    static (string? @base, List<string> interfaces) ExtractInheritance(IElement content)
    {
        var interfaces = new List<string>();
        var boxInfo = content.QuerySelector("div.box_info");
        if (boxInfo is null) return (null, interfaces);

        var inheritDt = boxInfo.QuerySelectorAll("dt").FirstOrDefault(dt =>
            string.Equals(MemberPageParser.NormaliseWhitespace(dt.TextContent), "Inherits:", StringComparison.Ordinal));
        if (inheritDt is null) return (null, interfaces);

        var dd = inheritDt.NextElementSibling;
        while (dd is not null && dd.LocalName != "dd") dd = dd.NextElementSibling;
        if (dd is null) return (null, interfaces);

        // Direct parent: <span class="inheritName">Object</span> at the top of the dd.
        var nameSpan = dd.QuerySelector("span.inheritName");
        var baseType = nameSpan is null ? null : MemberPageParser.NormaliseWhitespace(nameSpan.TextContent);
        return (string.IsNullOrEmpty(baseType) ? null : baseType, interfaces);
    }

    // ── Constructor blocks ────────────────────────────────────────────────────

    static List<MethodInfo> ExtractConstructorDetails(IElement content, string typeName)
    {
        var result = new List<MethodInfo>();
        var details = content.QuerySelector("div#constructor_details");
        if (details is null) return result;

        foreach (var block in details.QuerySelectorAll("div.method_details"))
        {
            var enriched = MemberPageParser.ParseMethodFromElement(block, isCtor: true);
            if (enriched is null) continue;

            // The IR ctor signature: render as `TypeName(args)` rather than `initialize(args)`.
            // This is the convention from Tekla / Rhino and matches what downstream tooling
            // expects (constructors carry the type name).
            var openParen = enriched.signature.IndexOf('(');
            var ctorSig = openParen < 0
                ? $"{typeName}()"
                : $"{typeName}{enriched.signature.Substring(openParen)}";
            // Drop any " => ReturnType" tail — ctors always return null in IR.
            var arrowIdx = ctorSig.IndexOf(" =>", StringComparison.Ordinal);
            if (arrowIdx >= 0) ctorSig = ctorSig.Substring(0, arrowIdx);

            // Name keeps the Ruby idiom: `initialize` (the brief calls out preserving `?`/`!`).
            // We use `initialize` rather than the type name so the catalog name stays the bare
            // Ruby method name a user would call.
            var docUrl = ExtractDocUrlForBlock(block, content);

            result.Add(new MethodInfo(
                name: enriched.name,
                signature: ctorSig.Trim(),
                summary: enriched.remarks ?? "",
                remarks: enriched.remarks,
                @params: enriched.@params,
                returns: null,
                throws: enriched.throws,
                events_related: new List<string>(),
                doc_url: docUrl,
                since: null,
                deprecated: null));
        }
        return result;
    }

    // ── Method blocks (class or instance) ─────────────────────────────────────

    static List<MethodInfo> ExtractMethodBlocks(IElement content, string detailsListSelector)
    {
        var result = new List<MethodInfo>();
        var details = content.QuerySelector("div" + detailsListSelector);
        if (details is null) return result;

        foreach (var block in details.QuerySelectorAll("div.method_details"))
        {
            var enriched = MemberPageParser.ParseMethodFromElement(block, isCtor: false);
            if (enriched is null) continue;
            var docUrl = ExtractDocUrlForBlock(block, content);

            // YARD's method summary uses the description's first paragraph; the IR `summary`
            // should be a concise one-liner. We mirror that here by taking the remarks (full
            // discussion) and ALSO keeping it as the `remarks` — readers can normalise later.
            var summary = TrimToFirstSentence(enriched.remarks);

            result.Add(new MethodInfo(
                name: enriched.name,
                signature: enriched.signature,
                summary: summary,
                remarks: enriched.remarks,
                @params: enriched.@params,
                returns: enriched.returns,
                throws: enriched.throws,
                events_related: new List<string>(),
                doc_url: docUrl,
                since: null,
                deprecated: null));
        }
        return result;
    }

    /// <summary>
    /// Compose the per-member doc URL from the block's h3 id + the type page URL. YARD page-
    /// internal anchor format is `#methodname-instance_method` / `#methodname-class_method`.
    /// </summary>
    static string ExtractDocUrlForBlock(IElement block, IElement content)
    {
        var h3 = block.QuerySelector("h3.signature");
        var id = h3?.GetAttribute("id") ?? "";
        // Fall back to the page <meta name="url"> if the block has no id. We bound to "" so
        // schema validation (`type: "string"`) doesn't trip.
        if (string.IsNullOrEmpty(id))
        {
            return "";
        }
        // Locate the page url. content is the inner content div; the meta tag is on the document.
        var doc = block.Owner;
        var meta = doc?.QuerySelector("meta[name='url']");
        var pageUrl = meta?.GetAttribute("content") ?? "";
        return string.IsNullOrEmpty(pageUrl) ? "" : pageUrl + "#" + id;
    }

    static string TrimToFirstSentence(string? remarks)
    {
        if (string.IsNullOrEmpty(remarks)) return "";
        // Take up to first period or newline that isn't preceded by an abbreviation. We accept
        // false positives — the IR is downstream-readable either way.
        var idx = remarks.IndexOf('.');
        if (idx < 0) return MemberPageParser.NormaliseWhitespace(remarks);
        return MemberPageParser.NormaliseWhitespace(remarks.Substring(0, idx + 1));
    }

    // ── Getter/setter pair detection ─────────────────────────────────────────

    /// <summary>
    /// Walk the parsed instance methods and pair each setter (`foo=`) with its companion getter
    /// (`foo`). The pair collapses into a single PropertyInfo with both getter + setter true;
    /// the bare getter remains a method ONLY if no setter exists. The standalone setter (if a
    /// matching getter is missing) becomes a property with getter=false / setter=true.
    ///
    /// Rationale: Ruby has no first-class `attr_*` distinction at the API surface — users just
    /// call methods. SketchUp uses paired methods consistently for state (e.g. `active_layer`
    /// + `active_layer=`) and bare methods for behavior (`close_active`). Pair detection
    /// correctly catalogs the state methods as properties without losing the behavior ones.
    /// </summary>
    static (List<MethodInfo> remainingMethods, List<PropertyInfo> properties)
        ExtractPropertiesFromGetterSetterPairs(IReadOnlyList<MethodInfo> methods, string sourceUrl)
    {
        var byName = methods.GroupBy(m => m.name).ToDictionary(g => g.Key, g => g.First());
        var consumed = new HashSet<string>(StringComparer.Ordinal);
        var properties = new List<PropertyInfo>();

        foreach (var m in methods)
        {
            if (consumed.Contains(m.name)) continue;
            // Setter: name ends with `=`. Bare name (without =) MAY have a matching getter.
            if (m.name.EndsWith("=", StringComparison.Ordinal))
            {
                var bare = m.name.Substring(0, m.name.Length - 1);
                if (byName.TryGetValue(bare, out var getter) && !consumed.Contains(bare))
                {
                    // Pair: getter + setter.
                    var propType = getter.returns?.type ?? (m.@params.Count > 0 ? m.@params[0].type : "Object");
                    properties.Add(new PropertyInfo(
                        name: bare,
                        type: propType,
                        getter: true,
                        setter: true,
                        summary: getter.summary,
                        remarks: getter.remarks ?? m.remarks,
                        doc_url: !string.IsNullOrEmpty(getter.doc_url) ? getter.doc_url : m.doc_url));
                    consumed.Add(bare);
                    consumed.Add(m.name);
                    continue;
                }
                // Setter-only — no companion getter. Catalog as write-only property.
                var setterType = m.@params.Count > 0 ? m.@params[0].type : "Object";
                properties.Add(new PropertyInfo(
                    name: bare,
                    type: setterType,
                    getter: false,
                    setter: true,
                    summary: m.summary,
                    remarks: m.remarks,
                    doc_url: m.doc_url));
                consumed.Add(m.name);
            }
        }

        var remaining = methods.Where(m => !consumed.Contains(m.name)).ToList();
        return (remaining, properties);
    }

    // ── Constants ────────────────────────────────────────────────────────────

    /// <summary>
    /// Walk every &lt;dl class="constants"&gt; on the page. Each &lt;dt id="X-constant"&gt; carries
    /// the fully-qualified constant name; the optional &lt;dd&gt; sibling carries a description.
    /// We map each constant to a PropertyInfo with getter=true, setter=false, type derived from
    /// the literal-value if present (best-effort), else "Object".
    /// </summary>
    static List<PropertyInfo> ExtractConstantsAsProperties(IElement content, string sourceUrl)
    {
        var props = new List<PropertyInfo>();
        foreach (var dl in content.QuerySelectorAll("dl.constants"))
        {
            foreach (var dt in dl.Children.Where(c => c.LocalName == "dt"))
            {
                var id = dt.GetAttribute("id") ?? "";
                // id format: "VALUE_NAME-constant" — the bare local name comes from the suffix.
                var bareName = id.EndsWith("-constant", StringComparison.Ordinal)
                    ? id.Substring(0, id.Length - "-constant".Length)
                    : id;
                if (string.IsNullOrEmpty(bareName)) continue;

                // dt full text is "Path::To::NAME = value". Best-effort type inference from the
                // value literal (just for the IR — the catalog will downstream-render the actual
                // value as part of the doc_url's anchor anyway).
                var dtText = MemberPageParser.NormaliseWhitespace(dt.TextContent);
                var equalsIdx = dtText.IndexOf('=');
                var literal = equalsIdx >= 0 ? dtText.Substring(equalsIdx + 1).Trim() : "";
                var type = InferConstantType(literal);

                // dd is the optional sibling description.
                string? summary = null;
                var dd = dt.NextElementSibling;
                while (dd is not null && dd.LocalName != "dd" && dd.LocalName != "dt") dd = dd.NextElementSibling;
                if (dd is not null && dd.LocalName == "dd")
                {
                    var txt = MemberPageParser.NormaliseWhitespace(dd.TextContent);
                    if (!string.IsNullOrEmpty(txt)) summary = txt;
                }

                props.Add(new PropertyInfo(
                    name: bareName,
                    type: type,
                    getter: true,
                    setter: false,
                    summary: summary ?? "",
                    remarks: null,
                    doc_url: $"{sourceUrl}#{id}"));
            }
        }
        return props;
    }

    static string InferConstantType(string literal)
    {
        if (string.IsNullOrEmpty(literal)) return "Object";
        if (literal == "true" || literal == "false") return "Boolean";
        if (literal == "nil") return "NilClass";
        if (Regex.IsMatch(literal, @"^-?\d+$")) return "Integer";
        if (Regex.IsMatch(literal, @"^-?\d+\.\d+$")) return "Float";
        if (literal.StartsWith("\"", StringComparison.Ordinal) || literal.StartsWith("'", StringComparison.Ordinal)) return "String";
        if (literal.StartsWith(":", StringComparison.Ordinal)) return "Symbol";
        if (literal.StartsWith("[", StringComparison.Ordinal)) return "Array";
        if (literal.StartsWith("{", StringComparison.Ordinal)) return "Hash";
        return "Object";
    }
}
