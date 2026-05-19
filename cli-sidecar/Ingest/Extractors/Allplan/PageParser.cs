// PageParser — parse a single mkdocs-material / mkdocstrings page from the Allplan
// PythonParts API documentation (pythonparts.allplan.com) into a complete TypeInfo.
//
// Source: Allplan PythonParts API, mkdocs-material 9.5.19 + mkdocstrings template.
//
// IMPORTANT — TWO LAYOUTS in production (we support both):
//
//   2024 layout ("legacy mkdocstrings" — site `/2024/`):
//     - h1 OUTSIDE the doc-class div, contains the Display Name in Title Case
//       (e.g. "Allplan Element")
//     - <p>Class full path: <code>NemAll_Python_X.Module.Class</code></p>
//     - Member headings use <h3 class="doc-heading"> with <code> containing the
//       full signature (e.g. <code>GetCommonProperties()</code>)
//     - Parameters / Returns / Raises rendered as <table>
//     - Overload groups wrap the variants in <div class="doc doc-function-overload">
//       with inner <div class="doc-contents doc-contents-overloads"> children
//
//   2025 layout ("current mkdocstrings" — site `/2025/`):
//     - h1 INSIDE the doc-class div with id=FQ name, contains <span class="doc-object-name
//       doc-class-name">ClassName</span>
//     - <p class="doc doc-class-full-path">Canonical path: <code>NemAll_Python_X.Module.Class</code></p>
//     - Member headings use <h2 class="doc-heading"> with <span class="doc-object-name
//       doc-function-name|doc-attribute-name">name</span> (NAME ONLY) and the
//       signature lives in a sibling <div class="doc-signature highlight"><pre><code>...</code></pre></div>
//     - Parameters / Returns / Raises rendered as <ul><li class="doc-section-item field-body">
//     - Overload variants are sibling <div class="doc doc-object doc-function doc-function-overload">
//       siblings under the same h2; there is NO outer doc-function-overload wrapper
//
// Both layouts share:
//   - <div class="doc doc-object doc-class"> as the class container
//   - <p class="doc-class-bases">Bases: <code>...</code></p> for inheritance
//   - <div class="doc-children"> wrapping the members
//   - <small class="doc doc-label doc-label-property|writable"> labels for properties
//   - <div class="doc-md-description"><p>...</p></div> for parameter / return descriptions
//
// Detection: presence of `<p class="doc doc-class-full-path">` distinguishes 2025; absent → 2024.
//
// Unlike Sandcastle-based extractors (Tekla/Rhino/Grasshopper) that spread each
// member across per-member pages, mkdocstrings packs the entire class — every
// attribute, every method, every overload — onto ONE HTML page per class. A
// single fetch produces a fully-populated TypeInfo.
//
// IR mapping decisions:
//   • Python class with no `Bases:` → `kind: "class"`.
//   • Python class with `Bases: IntEnum / Enum / Flag / IntFlag / StrEnum` → `kind: "enum"`,
//     each attribute becomes an `enum_values[]` entry (name + integer value).
//   • Python class with `Bases: ABC | ABCMeta` → `kind: "interface"` (Python ABCs are the
//     closest analog).
//   • Module-level free functions (`<Module>/_functions/` pages) → synthetic `kind: "static-class"`
//     named `Functions` under the module namespace. Each function is a `methods[]` entry.
//   • `__init__` → constructors[] (ctor convention; signature renders as `ClassName(args)`).
//   • Properties (mkdocstrings `<div class="doc doc-object doc-attribute">`) → `properties[]`.
//     Each property's getter = always true, setter = true iff the doc-label-writable is present.
//     The type is parsed from the signature/code block (`AttrName: SomeType` → `SomeType`).
//   • Overloaded methods/ctors — both layouts emit ONE entry per overload variant (signature in
//     the IR `name` for disambiguation, matching Tekla/Rhino ctor-overload behavior).
//   • Inner classes (nested under `<h2>Classes</h2>`) — emitted as separate top-level TypeInfo
//     entries with namespace `<parent>.<inner_name>` (matches the .NET-style nested-type
//     convention; the IR has no nested-type slot, so promoting them up is the lossless choice).

using AngleSharp.Dom;
using AngleSharp.Html.Parser;
using System.Text;
using System.Text.Json;
using System.Text.RegularExpressions;

namespace AwareSidecar.Ingest.Extractors.Allplan;

public static class PageParser
{
    static readonly HtmlParser _parser = new();

    /// <summary>
    /// Result of parsing one page. `Type` is the primary class/enum/etc.; `InnerTypes` carries
    /// any nested classes found via &lt;h2&gt;Classes&lt;/h2&gt; sections (promoted to top-level
    /// IR types with parent-prefixed namespaces).
    /// </summary>
    public sealed record ParseResult(TypeInfo Type, List<TypeInfo> InnerTypes);

    /// <summary>
    /// Parse one Allplan PythonParts API page into a complete TypeInfo (plus inner types if
    /// any). Returns null if the page is not a recognised type page (no class container, or
    /// non-API content like getting_started/release_notes).
    /// </summary>
    public static ParseResult? Parse(string html, string sourceUrl)
    {
        using var doc = _parser.ParseDocument(html);
        var article = doc.QuerySelector("article.md-content__inner.md-typeset")
            ?? doc.QuerySelector("article")
            ?? doc.DocumentElement;

        // Locate the primary doc-class container. The article body wraps everything; the first
        // (or only) `<div class="doc doc-object doc-class">` direct child is the class block.
        var primaryClass = article.QuerySelector("div.doc.doc-object.doc-class");

        if (primaryClass is null)
        {
            // No doc-class block on the page. Two cases:
            //   1) A `_functions/` page (2024 layout) with no container — module-level free functions.
            //   2) A `/<Module>/` namespace landing page that uses `<div class="doc doc-object
            //      doc-module">` to wrap module-level free functions inline (2025 layout).
            //   3) Non-API content (getting_started, manual, etc.) — return null.
            var h1 = article.QuerySelector("h1");
            if (h1 is null) return null;

            // 2025 layout: namespace landing page wraps free functions in doc-module. The full
            // module name is the h1's id (e.g. id="NemAll_Python_Geometry") inside doc-module.
            // 2024 layout: `_functions/` page where the doc-module wrapper has no h1; the
            // article-level h1 carries the slugified display name. The FQN sits in the article-
            // level <p>Module full path: <code>FQN</code></p>.
            var moduleDiv = article.QuerySelector("div.doc.doc-object.doc-module");
            if (moduleDiv is not null)
            {
                // Try (1) 2025 layout — h1 inside doc-module with FQN id.
                // Prefer the h1 id (the fully-qualified module path) over the span.doc-module-name
                // (which carries only the LOCAL module name). Two distinct module pages can have
                // the same local name across sections (e.g. `Utils.PythonPart` vs `PythonPart`);
                // the id keeps them disambiguated, the span text would collide.
                var moduleH1 = moduleDiv.QuerySelector("h1.doc-heading");
                var moduleName = "";
                if (moduleH1 is not null)
                {
                    var id = moduleH1.GetAttribute("id") ?? "";
                    moduleName = id;
                    if (string.IsNullOrEmpty(moduleName))
                    {
                        var nameSpan = moduleH1.QuerySelector("span.doc-module-name");
                        if (nameSpan is not null) moduleName = NormaliseWhitespace(nameSpan.TextContent);
                    }
                }
                // Try (2) 2024 layout — article-level <p>Module full path: <code>FQN</code></p>.
                if (string.IsNullOrEmpty(moduleName) || moduleName.Contains(' '))
                {
                    foreach (var p in article.QuerySelectorAll("p"))
                    {
                        var t = p.TextContent ?? "";
                        if (!t.StartsWith("Module full path:", StringComparison.Ordinal)) continue;
                        var code = p.QuerySelector("code");
                        if (code is not null)
                        {
                            moduleName = NormaliseWhitespace(code.TextContent);
                            break;
                        }
                    }
                }
                // Try (3) URL-derived namespace as last resort.
                if (string.IsNullOrEmpty(moduleName))
                {
                    moduleName = TryExtractModuleNamespaceFromUrl(sourceUrl)
                        ?? TryExtractNamespaceFromUrl(sourceUrl)
                        ?? "";
                }
                if (string.IsNullOrEmpty(moduleName)) return null;
                return ParseModuleFunctionsPage(moduleDiv, moduleName, sourceUrl);
            }

            // No doc-module either. 2024 `_functions/` pages technically DO have doc-module (above)
            // but defensive: if a vendor variant doesn't, derive from URL pattern.
            var nsFromUrl = TryExtractModuleNamespaceFromUrl(sourceUrl);
            if (nsFromUrl is null) return null;
            return ParseModuleFunctionsPage(article, nsFromUrl, sourceUrl);
        }

        // Class full path: 2024 = "Class full path:" (under article-level <p>); 2025 =
        // "Canonical path:" (under <p class="doc-class-full-path"> inside the class block).
        var fullPath = ExtractClassFullPath(article, primaryClass);
        if (string.IsNullOrEmpty(fullPath))
        {
            // Some pages have a `<h1 id="...">` with the FQ name as id (a fallback we use for
            // 2025 — its h1's id IS the full path).
            var h1 = primaryClass.QuerySelector("h1.doc-heading") ?? article.QuerySelector("h1");
            fullPath = h1?.GetAttribute("id") ?? "";
            if (string.IsNullOrEmpty(fullPath)) return null;
        }

        var (ns, name) = SplitNamespace(fullPath);

        var (baseType, baseList, kind) = ExtractBasesAndKind(primaryClass);
        var summary = ExtractTypeSummary(primaryClass);
        if (string.IsNullOrEmpty(summary))
            summary = $"(No description provided in vendor docs for {fullPath}.)";

        // Find the doc-children container — methods/attributes/inner classes live here.
        var children = primaryClass.QuerySelector("div.doc.doc-children");

        // Inner classes — promote to top-level IR types with parent-prefixed namespace.
        var innerTypes = new List<TypeInfo>();
        if (children is not null)
        {
            innerTypes.AddRange(ExtractInnerClasses(children, parentFq: fullPath, parentSourceUrl: sourceUrl));
        }

        // Attributes section ↔ Properties / Enum members
        var (properties, enumValues) = ExtractAttributes(children, ownerName: name, ownerFqName: fullPath, sourceUrl: sourceUrl, kind: kind);

        // Functions section ↔ Methods / Constructors
        var (constructors, methods) = ExtractFunctions(children, typeName: name, fqName: fullPath, sourceUrl: sourceUrl);

        var typeInfo = new TypeInfo(
            name: name,
            @namespace: ns,
            kind: kind,
            summary: NormaliseWhitespace(summary),
            remarks: null,
            @base: baseType,
            interfaces: baseList,
            doc_url: sourceUrl,
            constructors: constructors,
            methods: methods,
            properties: properties,
            events: new List<EventInfo>(),
            enum_values: enumValues,
            delegate_signature: null);

        return new ParseResult(typeInfo, innerTypes);
    }

    // ── Class full path ───────────────────────────────────────────────────────

    static string ExtractClassFullPath(IElement article, IElement primaryClass)
    {
        // 2025 layout: <p class="doc doc-class-full-path">Canonical path: <code>FQN</code></p>
        // inside the class block.
        var fullPathP = primaryClass.QuerySelector("p.doc-class-full-path");
        if (fullPathP is not null)
        {
            var code = fullPathP.QuerySelector("code");
            if (code is not null)
            {
                var text = NormaliseWhitespace(code.TextContent);
                if (!string.IsNullOrEmpty(text)) return text;
            }
        }

        // 2024 layout: `<p>Class full path: <code>FQN</code></p>` outside the class block.
        foreach (var p in article.QuerySelectorAll("p"))
        {
            var text = p.TextContent ?? "";
            if (!text.StartsWith("Class full path:", StringComparison.Ordinal)) continue;
            var code = p.QuerySelector("code");
            if (code is null) continue;
            return NormaliseWhitespace(code.TextContent);
        }
        return "";
    }

    /// <summary>
    /// Derive the module namespace from any api_reference page URL. Used as a fallback when
    /// the page's h1 doesn't carry an id (rare in 2025 layout but possible).
    /// </summary>
    static string? TryExtractNamespaceFromUrl(string url)
    {
        // Match `/api_reference/<Section>/<Module>/...` and return <Module>.
        var m = Regex.Match(url, @"/api_reference/[^/]+/([^/]+)/?");
        if (!m.Success) return null;
        return m.Groups[1].Value;
    }

    /// <summary>
    /// Derive the module namespace from a `/_functions/` page URL (mkdocstrings doesn't carry
    /// "Class full path:" on these — the namespace is the module + ".Functions" pseudo-class).
    ///
    /// Examples:
    ///   /api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/_functions/   → "NemAll_Python_AllplanSettings"
    ///   /api_reference/GeneralScripts/PythonPart/_functions/                       → "PythonPart"
    ///   /api_reference/StdReinfShapeBuilder/BarShapePlacementUtil/_functions/      → "BarShapePlacementUtil"
    ///   /api_reference/Utils/LibraryBitmapPreview/_functions/                       → "LibraryBitmapPreview"
    ///
    /// Strategy: the module name is the segment immediately before `_functions/`. Any section
    /// (InterfaceStubs / GeneralScripts / StdReinfShapeBuilder / Utils / etc.) is accepted.
    /// </summary>
    static string? TryExtractModuleNamespaceFromUrl(string url)
    {
        // Match any `<anything>/<Module>/_functions/` where <Module> is the segment we want.
        var m = Regex.Match(url, @"/([^/]+)/_functions/?(\?|$|#)");
        if (!m.Success) return null;
        return m.Groups[1].Value;
    }

    /// <summary>
    /// Split "Foo.Bar.Baz" into namespace "Foo.Bar" + name "Baz".
    /// For "NemAll_Python_Geometry.Point2D" → ns="NemAll_Python_Geometry", name="Point2D".
    /// </summary>
    static (string @namespace, string name) SplitNamespace(string fqName)
    {
        var lastDot = fqName.LastIndexOf('.');
        if (lastDot < 0) return ("", fqName);
        return (fqName.Substring(0, lastDot), fqName.Substring(lastDot + 1));
    }

    // ── Bases / kind ──────────────────────────────────────────────────────────

    /// <summary>
    /// Read the &lt;p class="doc-class-bases"&gt; line if present and return:
    ///   - the first base as the IR `@base`
    ///   - any additional bases as `interfaces[]`
    ///   - the IR `kind` inferred from the bases (IntEnum/Enum/Flag/IntFlag → "enum"; ABC →
    ///     "interface"; otherwise "class").
    ///
    /// Returns (null, [], "class") if no bases line is present.
    ///
    /// IMPORTANT: We must scope the bases lookup to the OUTER class's own doc-contents block,
    /// not the whole subtree. `classDiv.QuerySelector("p.doc-class-bases")` is a descendant
    /// query — when the outer class has NO bases of its own but contains an inner enum/interface
    /// class, the descendant query reaches into the inner class's doc-contents and picks up its
    /// bases. That mis-classifies the outer class as `enum`/`interface`. The 2025 ControlProperties
    /// class (plain class containing an inner IntEnum `ControlType`) was the canonical repro: it
    /// was getting `kind: "enum", base: "IntEnum"` from `ControlType`'s bases paragraph.
    ///
    /// Fix: locate the outer class's own doc-contents (the first direct-child div with class
    /// `doc.doc-contents` or `doc.doc-contents.first`), then look for `p.doc-class-bases` as a
    /// direct child of that doc-contents block. Inner-class bases live one level deeper (inside
    /// doc-children → inner doc-class → inner doc-contents) and are correctly skipped.
    /// </summary>
    static (string? @base, List<string> interfaces, string kind) ExtractBasesAndKind(IElement classDiv)
    {
        // Step 1: find the outer class's OWN doc-contents (direct child of classDiv).
        var ownContents = classDiv.Children.FirstOrDefault(c =>
            c.LocalName == "div"
            && c.ClassList.Contains("doc")
            && c.ClassList.Contains("doc-contents"));

        // Step 2: look for p.doc-class-bases as a direct child of own-contents.
        IElement? basesP = null;
        if (ownContents is not null)
        {
            basesP = ownContents.Children.FirstOrDefault(c =>
                c.LocalName == "p" && c.ClassList.Contains("doc-class-bases")) as IElement;
        }

        if (basesP is null) return (null, new List<string>(), "class");

        // mkdocstrings emits multiple <code> entries for multi-base classes, e.g.
        //   Bases: <code>BaseA</code>, <code>BaseB</code>
        var codes = basesP.QuerySelectorAll("code")
            .Select(c => NormaliseWhitespace(c.TextContent))
            .Where(s => !string.IsNullOrEmpty(s))
            .ToList();

        if (codes.Count == 0) return (null, new List<string>(), "class");

        var firstBase = codes[0];
        var extraInterfaces = codes.Skip(1).ToList();

        // Classify the kind by looking at any of the bases (Python multi-inheritance allows e.g.
        // `class Foo(IntEnum, MyMixin)`).
        string kind = "class";
        foreach (var b in codes)
        {
            // Python's enum module classes: IntEnum / Enum / Flag / IntFlag / StrEnum.
            if (b is "IntEnum" or "Enum" or "Flag" or "IntFlag" or "StrEnum")
            {
                kind = "enum";
                break;
            }
            if (b == "ABC" || b == "ABCMeta")
            {
                kind = "interface";
                break;
            }
        }

        return (firstBase, extraInterfaces, kind);
    }

    // ── Type-level summary ────────────────────────────────────────────────────

    /// <summary>
    /// The class-level summary sits inside &lt;div class="doc doc-contents first"&gt;, as one or
    /// more &lt;p&gt; elements AFTER the optional &lt;p class="doc-class-bases"&gt; (and AFTER
    /// the 2025-style &lt;p class="doc-class-full-path"&gt;). We grab the first non-empty
    /// paragraph that is NOT a meta-line (bases / canonical path / section title).
    /// </summary>
    static string ExtractTypeSummary(IElement classDiv)
    {
        var contents = classDiv.QuerySelector("div.doc.doc-contents");
        if (contents is null) return "";

        foreach (var node in contents.Children)
        {
            if (node.LocalName != "p") continue;
            // Skip meta-lines.
            if (node.ClassList.Contains("doc-class-bases")) continue;
            if (node.ClassList.Contains("doc-class-full-path")) continue;
            // Skip section-title placeholders (Parameters: / Methods: / Attributes: etc. — these
            // are part of the summary list in 2025 layout, not narrative text).
            if (node.QuerySelector("span.doc-section-title") is not null) break;
            var text = NormaliseWhitespace(node.TextContent);
            if (!string.IsNullOrEmpty(text)) return text;
        }
        return "";
    }

    // ── Inner classes (nested under doc-children) ─────────────────────────────

    static List<TypeInfo> ExtractInnerClasses(IElement children, string parentFq, string parentSourceUrl)
    {
        var result = new List<TypeInfo>();
        // Inner classes are direct children of doc-children that have class doc-class.
        // 2024: <div class="doc doc-object doc-class"> with h3 inside.
        // 2025: <div class="doc doc-object doc-class"> with h2 inside (same structural shape).
        foreach (var inner in children.Children
            .Where(c => c.LocalName == "div" && c.ClassList.Contains("doc-class")))
        {
            // The heading sits at h2 (2025) or h3 (2024) — both have .doc-heading.
            var innerHead = inner.QuerySelector("h2.doc-heading, h3.doc-heading");
            var idAttr = innerHead?.GetAttribute("id") ?? "";
            string innerFq;
            if (!string.IsNullOrEmpty(idAttr))
            {
                innerFq = idAttr;
            }
            else
            {
                // Fallback: read the heading text. 2025 puts the name in a span.doc-object-name;
                // 2024 puts it in a code element.
                var nameSpan = innerHead?.QuerySelector("span.doc-object-name");
                var code = innerHead?.QuerySelector("code");
                var innerName = nameSpan is not null
                    ? NormaliseWhitespace(nameSpan.TextContent)
                    : (code is null ? "" : NormaliseWhitespace(code.TextContent));
                if (string.IsNullOrEmpty(innerName)) continue;
                innerFq = $"{parentFq}.{innerName}";
            }

            var (innerNs, innerName2) = SplitNamespace(innerFq);
            var (innerBase, innerInterfaces, innerKind) = ExtractBasesAndKind(inner);
            var innerSummary = ExtractTypeSummary(inner);
            if (string.IsNullOrEmpty(innerSummary))
                innerSummary = $"(No description provided in vendor docs for {innerFq}.)";

            // Inner classes also have their own doc-children — recurse for attributes/functions.
            var innerChildren = inner.QuerySelector("div.doc.doc-children");
            var (innerProps, innerEnumValues) = ExtractAttributes(innerChildren, ownerName: innerName2, ownerFqName: innerFq, sourceUrl: parentSourceUrl, kind: innerKind);
            var (innerCtors, innerMethods) = ExtractFunctions(innerChildren, typeName: innerName2, fqName: innerFq, sourceUrl: parentSourceUrl);

            var doc_url = parentSourceUrl + "#" + (string.IsNullOrEmpty(idAttr) ? innerFq : idAttr);

            result.Add(new TypeInfo(
                name: innerName2,
                @namespace: innerNs,
                kind: innerKind,
                summary: NormaliseWhitespace(innerSummary),
                remarks: null,
                @base: innerBase,
                interfaces: innerInterfaces,
                doc_url: doc_url,
                constructors: innerCtors,
                methods: innerMethods,
                properties: innerProps,
                events: new List<EventInfo>(),
                enum_values: innerEnumValues,
                delegate_signature: null));
        }
        return result;
    }

    // ── Attributes (properties / enum values) ─────────────────────────────────

    /// <summary>
    /// Walk every &lt;div class="doc doc-object doc-attribute"&gt; under doc-children. For enum
    /// kinds, each attribute is an enum value (`name = literal`); for everything else, each
    /// attribute is a property with a type annotation.
    /// </summary>
    static (List<PropertyInfo> properties, List<EnumValueInfo> enumValues)
        ExtractAttributes(IElement? children, string ownerName, string ownerFqName, string sourceUrl, string kind)
    {
        var props = new List<PropertyInfo>();
        var values = new List<EnumValueInfo>();
        if (children is null) return (props, values);

        foreach (var attr in children.QuerySelectorAll("div.doc.doc-object.doc-attribute"))
        {
            // Skip attributes that are inside an inner doc-class subtree.
            if (IsInsideInnerClass(attr, root: children)) continue;

            // The heading sits at h2 (2025) or h3 (2024). Both have .doc-heading.
            var head = attr.QuerySelector("h2.doc-heading, h3.doc-heading");
            if (head is null) continue;

            var idAttr = head.GetAttribute("id") ?? "";
            var (attrName, typeOrValue, isAssignment) = ParseAttributeDeclaration(attr, head);
            if (string.IsNullOrEmpty(attrName)) continue;

            // Description text — first <p> inside .doc-contents.
            var desc = ExtractFirstDescriptionParagraph(attr);

            var labels = ExtractLabels(head);

            var docUrl = string.IsNullOrEmpty(idAttr) ? sourceUrl : $"{sourceUrl}#{idAttr}";

            if (kind == "enum" && isAssignment)
            {
                JsonElement valueEl;
                if (int.TryParse(typeOrValue, System.Globalization.NumberStyles.Integer, System.Globalization.CultureInfo.InvariantCulture, out var intVal))
                {
                    valueEl = JsonSerializer.SerializeToElement(intVal, IrJsonContext.Default.Int32);
                }
                else
                {
                    valueEl = JsonSerializer.SerializeToElement(typeOrValue, IrJsonContext.Default.String);
                }
                values.Add(new EnumValueInfo(
                    name: attrName,
                    value: valueEl,
                    summary: string.IsNullOrEmpty(desc) ? null : desc));
            }
            else
            {
                var setter = labels.Contains("writable");
                string propType;
                if (isAssignment)
                {
                    propType = InferLiteralType(typeOrValue);
                }
                else
                {
                    propType = string.IsNullOrEmpty(typeOrValue) ? "object" : typeOrValue;
                }
                props.Add(new PropertyInfo(
                    name: attrName,
                    type: propType,
                    getter: true,
                    setter: setter,
                    summary: string.IsNullOrEmpty(desc) ? "" : desc,
                    remarks: null,
                    doc_url: docUrl));
            }
        }
        return (props, values);
    }

    /// <summary>
    /// Parse the attribute name + type-or-value from the heading and (optional) signature block.
    ///
    /// 2024 layout:
    ///   <h3><code><span class="n">AttrName</span>: <span class="n">Type</span></code></h3>
    ///   We read the code text and split on `:` or `=`.
    ///
    /// 2025 layout:
    ///   <h2><span class="doc-object-name doc-attribute-name">AttrName</span></h2>
    ///   <div class="doc-signature highlight"><pre><code>AttrName: Type</code></pre></div>
    ///   We read the name from the span and the type/value from the signature code.
    ///
    /// Returns (name, typeOrValue, isAssignment).
    /// </summary>
    static (string name, string typeOrValue, bool isAssignment) ParseAttributeDeclaration(IElement attrBlock, IElement head)
    {
        // 2025-style — name lives in span.doc-object-name; signature in sibling div.doc-signature.
        var nameSpan = head.QuerySelector("span.doc-object-name");
        if (nameSpan is not null)
        {
            var name = NormaliseWhitespace(nameSpan.TextContent);
            // Walk for a sibling div.doc-signature.
            var sig = attrBlock.QuerySelector("div.doc-signature code");
            if (sig is null) return (name, "", false);
            var sigText = NormaliseWhitespace(sig.TextContent);
            return ParseSignatureForAttribute(sigText, fallbackName: name);
        }

        // 2024-style — h3's first <code> child has the full declaration.
        var code = head.QuerySelector("code");
        if (code is null) return ("", "", false);
        var text = NormaliseWhitespace(code.TextContent);
        if (string.IsNullOrEmpty(text)) return ("", "", false);
        return ParseSignatureForAttribute(text, fallbackName: "");
    }

    static (string name, string typeOrValue, bool isAssignment) ParseSignatureForAttribute(string text, string fallbackName)
    {
        // Try `name: Type` first (type annotation).
        var colonIdx = FindFirstAtDepthZero(text, ':');
        var equalsIdx = FindFirstAtDepthZero(text, '=');

        if (colonIdx >= 0 && (equalsIdx < 0 || colonIdx < equalsIdx))
        {
            var name = text.Substring(0, colonIdx).Trim();
            // tail = Type [= default]; strip default if present.
            var tail = text.Substring(colonIdx + 1).Trim();
            var eqIdx = FindFirstAtDepthZero(tail, '=');
            if (eqIdx >= 0) tail = tail.Substring(0, eqIdx).Trim();
            return (string.IsNullOrEmpty(name) ? fallbackName : name, tail, false);
        }
        if (equalsIdx >= 0)
        {
            var name = text.Substring(0, equalsIdx).Trim();
            var value = text.Substring(equalsIdx + 1).Trim();
            return (string.IsNullOrEmpty(name) ? fallbackName : name, value, true);
        }
        return (string.IsNullOrEmpty(text) ? fallbackName : text.Trim(), "", false);
    }

    /// <summary>
    /// Extract the label text for each mkdocstrings &lt;small class="doc-label-*"&gt; under the
    /// element's labels span. Returns just the suffix (e.g. "property", "writable", "overload").
    /// </summary>
    static HashSet<string> ExtractLabels(IElement head)
    {
        var set = new HashSet<string>(StringComparer.Ordinal);
        foreach (var lab in head.QuerySelectorAll("small.doc-label"))
        {
            foreach (var cls in lab.ClassList)
            {
                if (cls.StartsWith("doc-label-", StringComparison.Ordinal))
                {
                    set.Add(cls.Substring("doc-label-".Length));
                }
            }
        }
        return set;
    }

    /// <summary>
    /// Heuristic type inference for class-attribute equals declarations (`name = literal`). Used
    /// for non-enum classes that expose Python constants. Returns "object" if we can't tell.
    /// </summary>
    static string InferLiteralType(string literal)
    {
        if (string.IsNullOrEmpty(literal)) return "object";
        if (literal is "True" or "False") return "bool";
        if (literal == "None") return "None";
        if (Regex.IsMatch(literal, @"^-?\d+$")) return "int";
        if (Regex.IsMatch(literal, @"^-?\d+\.\d+$")) return "float";
        if (literal.StartsWith("\"", StringComparison.Ordinal) || literal.StartsWith("'", StringComparison.Ordinal)) return "str";
        if (literal.StartsWith("[", StringComparison.Ordinal)) return "list";
        if (literal.StartsWith("{", StringComparison.Ordinal)) return "dict";
        if (literal.StartsWith("(", StringComparison.Ordinal)) return "tuple";
        return "object";
    }

    // ── Functions (methods + constructors) ────────────────────────────────────

    /// <summary>
    /// Walk every &lt;div class="doc doc-object doc-function"&gt; (incl. overload variants) under
    /// doc-children. Splits constructors (`__init__`) from regular methods.
    ///
    /// Both layouts produce ONE MethodInfo per canonical name. When @typing.overload variants are
    /// present we merge them: the MethodInfo's `signature` becomes a `" | "` -joined list of every
    /// overload variant's signature (SketchUp YARD convention — see
    /// `cli-sidecar/Ingest/Extractors/SketchUp/MemberPageParser.cs § ExtractNameAndSignature`).
    /// `params`/`returns`/`throws`/`doc_url`/`summary` come from the FIRST variant — downstream
    /// consumers (manifest catalog, skills) only need a representative shape, and the joined
    /// signature carries the full picture.
    ///
    /// Pre-v0.30 the parser disambiguated overloads by embedding the param list into the IR
    /// `name` (`Foo(arg)`, `Foo(other)`). That worked for 2024 (bare-name params) but broke YAML
    /// validation for 2025 layout (typed params with `:` colons + return-type annotations
    /// containing `<>[]`). Merge sidesteps the disambiguator entirely.
    /// </summary>
    static (List<MethodInfo> ctors, List<MethodInfo> methods)
        ExtractFunctions(IElement? children, string typeName, string fqName, string sourceUrl)
    {
        var ctors = new List<MethodInfo>();
        var methods = new List<MethodInfo>();
        if (children is null) return (ctors, methods);

        // Visit-tracking: we walk the DOM via several patterns and need to avoid emitting the
        // same physical block twice.
        var visited = new HashSet<IElement>();

        // 1. 2024-style overload wrappers: <div class="doc-function-overload"> with inner
        //    <div class="doc-contents doc-contents-overloads"> children. Each inner doc-function
        //    is one overload variant; its <code> direct child is the signature.
        foreach (var ov in children.QuerySelectorAll("div.doc.doc-object.doc-function-overload"))
        {
            if (IsInsideInnerClass(ov, root: children)) continue;
            var overloadInner = ov.QuerySelector("div.doc-contents.doc-contents-overloads");
            if (overloadInner is null) continue; // not a 2024-style wrapper

            visited.Add(ov);

            // Heading may be h2 (module-functions page, 2024) or h3 (class-level page, 2024).
            var ovHead = ov.QuerySelector("h2.doc-heading, h3.doc-heading");
            var idAttr = ovHead?.GetAttribute("id") ?? "";
            var canonicalName = ExtractOverloadCanonicalName(ovHead);
            if (string.IsNullOrEmpty(canonicalName)) continue;

            var variants = new List<MethodInfo>();
            foreach (var inner in overloadInner.Children
                .Where(c => c.LocalName == "div" && c.ClassList.Contains("doc-function")))
            {
                visited.Add(inner);
                var enriched = ParseFunctionBlock2024Variant(inner, typeName, fqName, sourceUrl, idAttr);
                if (enriched is not null) variants.Add(enriched);
            }
            if (variants.Count == 0) continue;

            var merged = MergeOverloadVariants(variants, canonicalName);
            if (IsConstructor(merged.name))
                ctors.Add(RenderAsCtor(merged, typeName));
            else
                methods.Add(merged);
        }

        // 2. 2025-style overload sets: <div class="doc-function"> wrapper with h2 + a
        //    <div class="doc-overloads"> child. Inside doc-overloads: ALTERNATING
        //    <div class="doc-signature">...sig...</div>
        //    <div class="doc doc-object doc-function doc-function-overload">...body...</div>
        //    siblings, one pair per overload variant.
        foreach (var fn in children.QuerySelectorAll("div.doc.doc-object.doc-function"))
        {
            if (IsInsideInnerClass(fn, root: children)) continue;
            if (visited.Contains(fn)) continue;
            // Skip 2024-style overload wrappers (already processed above).
            if (fn.ClassList.Contains("doc-function-overload"))
            {
                // 2025-style overload variant bodies will be picked up below as children of a
                // doc-overloads container; standalone (without doc-overloads parent) is rare but
                // we skip them here — they'd be processed as part of a 2025 wrapper if any.
                continue;
            }

            // Look for the 2025-style overload container.
            var overloadContainer = fn.QuerySelector("div.doc-overloads");
            if (overloadContainer is not null)
            {
                visited.Add(fn);
                // Read the canonical name + id from the wrapper's h2.
                var head = fn.QuerySelector("h2.doc-heading, h3.doc-heading");
                var canonicalId = head?.GetAttribute("id") ?? "";
                var canonicalName = ExtractCanonicalNameFromHead(head);
                if (string.IsNullOrEmpty(canonicalName)) continue;

                // Walk the doc-overloads children pair-wise: each doc-signature is followed by a
                // doc-function-overload that contains the body. Accumulate all variants for this
                // overload set, then merge into one MethodInfo (see method-level summary above).
                IElement? pendingSignature = null;
                var variants = new List<MethodInfo>();
                foreach (var child in overloadContainer.Children)
                {
                    if (child.LocalName == "div" && child.ClassList.Contains("doc-signature"))
                    {
                        pendingSignature = child;
                    }
                    else if (child.LocalName == "div" && child.ClassList.Contains("doc-function-overload"))
                    {
                        visited.Add(child);
                        var enriched = ParseFunctionBlock2025Overload(
                            variantBody: child,
                            signatureDiv: pendingSignature,
                            canonicalName: canonicalName,
                            canonicalId: canonicalId,
                            typeName: typeName,
                            sourceUrl: sourceUrl);
                        pendingSignature = null;
                        if (enriched is not null) variants.Add(enriched);
                    }
                }
                if (variants.Count == 0) continue;

                var merged = MergeOverloadVariants(variants, canonicalName);
                if (IsConstructor(merged.name))
                    ctors.Add(RenderAsCtor(merged, typeName));
                else
                    methods.Add(merged);
                continue;
            }

            // 3. Single-signature method (both 2024 and 2025 layout):
            //    The function block has its own h2/h3 with canonical name, and its own
            //    doc-signature (2025) or h3-code (2024).
            visited.Add(fn);
            var singleHead = fn.QuerySelector("h2.doc-heading, h3.doc-heading");
            var singleId = singleHead?.GetAttribute("id") ?? "";
            var singleName = ExtractCanonicalNameFromHead(singleHead);
            if (string.IsNullOrEmpty(singleName)) continue;

            var singleEnriched = ParseFunctionBlock2025OrSingle(fn, singleName, singleId, typeName, fqName, sourceUrl, isOverloadVariant: false);
            if (singleEnriched is null) continue;
            if (IsConstructor(singleEnriched.name))
                ctors.Add(RenderAsCtor(singleEnriched, typeName));
            else
                methods.Add(singleEnriched);
        }

        return (ctors, methods);
    }

    /// <summary>
    /// Collapse a list of overload-variant MethodInfos into a single MethodInfo. The merged
    /// `name` is the bare canonical method name (no parens / no params). The merged `signature`
    /// is `" | "` -joined across every variant — downstream consumers (skills/* markdown,
    /// catalog/* JSON) see all overload shapes; the manifest verb-id sees just the bare name.
    ///
    /// Params / returns / throws / doc_url / summary inherit from the first variant by
    /// construction: every overload of the same method documents the same logical behavior
    /// (the @typing.overload variants only differ in argument shape). When mkdocstrings emits
    /// per-variant docstrings we keep the first one as the representative — losing per-variant
    /// nuance is intentional to keep the IR shape stable.
    /// </summary>
    static MethodInfo MergeOverloadVariants(List<MethodInfo> variants, string canonicalName)
    {
        if (variants.Count == 1)
        {
            // Single variant — re-stamp the name to the bare canonical form (the variant parser
            // emits the bare-canonical name already, but defensive in case a future variant
            // parser embeds the param list).
            return variants[0] with { name = canonicalName };
        }

        var head = variants[0];
        var joinedSignature = string.Join(" | ", variants.Select(v => v.signature));
        return head with { name = canonicalName, signature = joinedSignature };
    }

    /// <summary>
    /// Pull the canonical name from a function/method heading. Handles both 2024 layout (heading
    /// contains a &lt;code&gt; with the full signature → strip the param list) and 2025 layout
    /// (heading contains &lt;span class="doc-object-name doc-function-name"&gt;name&lt;/span&gt;).
    /// </summary>
    static string ExtractCanonicalNameFromHead(IElement? head)
    {
        if (head is null) return "";
        var nameSpan = head.QuerySelector("span.doc-object-name");
        if (nameSpan is not null)
        {
            return NormaliseWhitespace(nameSpan.TextContent);
        }
        var code = head.QuerySelector("code");
        if (code is not null)
        {
            var t = NormaliseWhitespace(code.TextContent);
            var openParen = t.IndexOf('(');
            return openParen < 0 ? t : t.Substring(0, openParen).Trim();
        }
        return "";
    }

    /// <summary>
    /// Parse one 2025-style overload variant. The signature lives in the previous-sibling
    /// &lt;div class="doc-signature"&gt; (passed as parameter); the body (description + tables)
    /// lives inside the variant block's &lt;div class="doc-contents"&gt;.
    ///
    /// Returns a MethodInfo whose `name` is the bare canonical name. The caller
    /// (`MergeOverloadVariants`) is responsible for joining sibling variants into one entry; this
    /// parser does NOT embed the param list into the name (pre-v0.30 it did, which corrupted YAML
    /// verb keys for 2025 layout — typed params contain `:` and return-types contain `<>[]`).
    /// </summary>
    static MethodInfo? ParseFunctionBlock2025Overload(
        IElement variantBody, IElement? signatureDiv, string canonicalName, string canonicalId,
        string typeName, string sourceUrl)
    {
        string signature;
        if (signatureDiv is not null)
        {
            var code = signatureDiv.QuerySelector("code");
            signature = code is not null ? NormaliseWhitespace(code.TextContent) : canonicalName + "()";
        }
        else
        {
            signature = canonicalName + "()";
        }

        var desc = ExtractFirstDescriptionParagraph(variantBody);
        var paramList = ExtractParameterList(variantBody);
        var returns = ExtractReturns(variantBody);
        var throws = ExtractRaises(variantBody);

        var docUrl = string.IsNullOrEmpty(canonicalId) ? sourceUrl : $"{sourceUrl}#{canonicalId}";

        return new MethodInfo(
            name: canonicalName,
            signature: signature,
            summary: string.IsNullOrEmpty(desc) ? "" : desc,
            remarks: string.IsNullOrEmpty(desc) ? null : desc,
            @params: paramList,
            returns: returns,
            throws: throws,
            events_related: new List<string>(),
            doc_url: docUrl,
            since: null,
            deprecated: null);
    }

    /// <summary>
    /// Parse one 2024-style overload-variant div (inner doc-function under a doc-function-overload
    /// wrapper). The signature lives in a bare &lt;code&gt; direct child; description + tables
    /// live under a sibling .doc-contents.
    ///
    /// Returns a MethodInfo whose `name` is the bare canonical name. The caller
    /// (`MergeOverloadVariants`) handles joining sibling variants into one entry.
    /// </summary>
    static MethodInfo? ParseFunctionBlock2024Variant(IElement variant, string typeName, string fqName, string sourceUrl, string idAttr)
    {
        var sigCode = variant.Children.FirstOrDefault(c => c.LocalName == "code");
        if (sigCode is null) return null;

        var signature = NormaliseWhitespace(sigCode.TextContent);
        if (string.IsNullOrEmpty(signature)) return null;

        var openParen = signature.IndexOf('(');
        var name = openParen < 0 ? signature : signature.Substring(0, openParen).Trim();

        var desc = ExtractFirstDescriptionParagraph(variant);
        var paramList = ExtractParameterList(variant);
        var returns = ExtractReturns(variant);
        var throws = ExtractRaises(variant);

        var docUrl = string.IsNullOrEmpty(idAttr) ? sourceUrl : $"{sourceUrl}#{idAttr}";

        return new MethodInfo(
            name: name,
            signature: signature,
            summary: string.IsNullOrEmpty(desc) ? "" : desc,
            remarks: string.IsNullOrEmpty(desc) ? null : desc,
            @params: paramList,
            returns: returns,
            throws: throws,
            events_related: new List<string>(),
            doc_url: docUrl,
            since: null,
            deprecated: null);
    }

    /// <summary>
    /// Parse one 2025-style doc-function block. The signature lives in &lt;div class="doc-signature"&gt;
    /// &lt;pre&gt;&lt;code&gt;...&lt;/code&gt;&lt;/pre&gt;&lt;/div&gt;, the description + sections
    /// in .doc-contents. Also handles 2024-style single-signature blocks where signature lives in
    /// the h3's &lt;code&gt;.
    /// </summary>
    static MethodInfo? ParseFunctionBlock2025OrSingle(
        IElement block, string canonicalName, string canonicalId, string typeName, string fqName, string sourceUrl, bool isOverloadVariant)
    {
        // Signature priority: 2025-style doc-signature first, then 2024-style heading code.
        string? signature = null;
        var sigDiv = block.QuerySelector("div.doc-signature code");
        if (sigDiv is not null)
        {
            signature = NormaliseWhitespace(sigDiv.TextContent);
        }
        else
        {
            // 2024 layout: signature is the heading's <code>. The heading may be h2 (module-
            // level pages) or h3 (class-level pages).
            var hd = block.QuerySelector("h2.doc-heading, h3.doc-heading");
            var code = hd?.QuerySelector("code");
            if (code is not null)
                signature = NormaliseWhitespace(code.TextContent);
        }

        if (string.IsNullOrEmpty(signature))
        {
            // No signature — fall back to canonicalName + "()" to avoid empty IR entries.
            signature = canonicalName + "()";
        }

        // Description + tables.
        var desc = ExtractFirstDescriptionParagraph(block);
        var paramList = ExtractParameterList(block);
        var returns = ExtractReturns(block);
        var throws = ExtractRaises(block);

        var docUrl = string.IsNullOrEmpty(canonicalId) ? sourceUrl : $"{sourceUrl}#{canonicalId}";

        // For overload variants, disambiguate the IR name. For single-signature methods, use the
        // canonical name as-is.
        var openParen = signature.IndexOf('(');
        string irName;
        if (isOverloadVariant)
        {
            irName = openParen >= 0 ? canonicalName + signature.Substring(openParen) : signature;
        }
        else
        {
            irName = canonicalName;
        }

        return new MethodInfo(
            name: irName,
            signature: signature,
            summary: string.IsNullOrEmpty(desc) ? "" : desc,
            remarks: string.IsNullOrEmpty(desc) ? null : desc,
            @params: paramList,
            returns: returns,
            throws: throws,
            events_related: new List<string>(),
            doc_url: docUrl,
            since: null,
            deprecated: null);
    }

    static string ExtractOverloadCanonicalName(IElement? h3)
    {
        if (h3 is null) return "";
        // The h3 text is "<name> overload" with the labels span trailing. Strip the labels.
        var clone = (IElement)h3.Clone();
        foreach (var lab in clone.QuerySelectorAll("span.doc.doc-labels").ToList()) lab.Remove();
        foreach (var lab in clone.QuerySelectorAll("small.doc-label").ToList()) lab.Remove();
        var text = NormaliseWhitespace(clone.TextContent);
        return text;
    }

    static bool IsConstructor(string name) =>
        name == "__init__" || name.StartsWith("__init__(", StringComparison.Ordinal);

    /// <summary>
    /// For constructors, replace the Python `__init__` token with the type name in the IR
    /// signature, drop any return-type arrow, and force `returns = null`. Handles single + merged
    /// (pipe-joined) signatures.
    ///
    /// Examples:
    ///   `__init__()`                                      → `TypeName()`
    ///   `__init__(x: float, y: float)`                    → `TypeName(x: float, y: float)`
    ///   `__init__() | __init__(point: Point2D)`           → `TypeName() | TypeName(point: Point2D)`
    ///   `__init__(arg) -> NoneType`                       → `TypeName(arg)`
    ///
    /// The merged-variant case is the v0.30 change: pre-merge each overload variant was a
    /// separate MethodInfo and `RenderAsCtor` rewrote one signature at a time.
    /// </summary>
    static MethodInfo RenderAsCtor(MethodInfo m, string typeName)
    {
        // Split the merged signature on " | ", rewrite each chunk, rejoin. Single signatures
        // round-trip as a one-element list.
        var chunks = m.signature.Split(" | ", StringSplitOptions.None);
        for (int i = 0; i < chunks.Length; i++)
        {
            var sig = chunks[i];
            var openParen = sig.IndexOf('(');
            var rewritten = openParen < 0 ? $"{typeName}()" : $"{typeName}{sig.Substring(openParen)}";
            // Drop any " -> ReturnType" tail (Python convention; ctors return self/None implicitly).
            var arrowIdx = rewritten.IndexOf("->", StringComparison.Ordinal);
            if (arrowIdx >= 0) rewritten = rewritten.Substring(0, arrowIdx).TrimEnd();
            chunks[i] = rewritten;
        }
        var ctorSig = string.Join(" | ", chunks);

        // The IR `name` for ctors is the bare type name (matches the post-merge convention for
        // methods). Downstream verb-id generation uses just the name; the signature carries every
        // overload's shape.
        return m with { name = typeName, signature = ctorSig, returns = null };
    }

    // ── Module-level _functions/ page handling ────────────────────────────────

    /// <summary>
    /// Module-level functions page (`/InterfaceStubs/<Module>/_functions/`). No `Class full path:`
    /// line; we synthesize a static-class named `Functions` under the module's namespace and put
    /// every doc-function entry there.
    /// </summary>
    static ParseResult ParseModuleFunctionsPage(IElement article, string moduleNamespace, string sourceUrl)
    {
        // The module-functions page has the same shape as a class page MINUS the class wrapper:
        // a series of doc-function divs (sometimes wrapped in doc-function-overload). We can
        // treat the article body as if it were a doc-children container.
        // ExtractFunctions returns (ctors, methods) — module-level page has no ctors so we drop
        // the ctors part.
        var (_, methods) = ExtractFunctions(article, typeName: "Functions", fqName: $"{moduleNamespace}.Functions", sourceUrl: sourceUrl);

        var typeInfo = new TypeInfo(
            name: "Functions",
            @namespace: moduleNamespace,
            kind: "static-class",
            summary: $"Module-level functions of {moduleNamespace}",
            remarks: null,
            @base: null,
            interfaces: new List<string>(),
            doc_url: sourceUrl,
            constructors: new List<MethodInfo>(),
            methods: methods,
            properties: new List<PropertyInfo>(),
            events: new List<EventInfo>(),
            enum_values: new List<EnumValueInfo>(),
            delegate_signature: null);

        return new ParseResult(typeInfo, new List<TypeInfo>());
    }

    // ── Section extraction (Parameters / Returns / Raises) ───────────────────

    /// <summary>
    /// Extract the Parameters section. Supports both 2024 (table) and 2025 (ul) layouts.
    /// </summary>
    static List<ParamInfo> ExtractParameterList(IElement block)
    {
        var result = new List<ParamInfo>();

        // 2024: <table> with Name / Type / Description / Default columns.
        var table = FindSectionTable(block, "Parameters:");
        if (table is not null)
        {
            foreach (var tr in table.QuerySelectorAll("tbody > tr.doc-section-item"))
            {
                var tds = tr.QuerySelectorAll("td").ToList();
                if (tds.Count < 3) continue;
                var name = NormaliseWhitespace(tds[0].TextContent);
                if (string.IsNullOrEmpty(name)) continue;
                name = name.TrimStart('*', '&').Trim();

                var type = NormaliseWhitespace(tds[1].TextContent);
                if (string.IsNullOrEmpty(type)) type = "object";

                var doc = ExtractDescriptionFromCell(tds[2]);

                bool optional = false;
                string? defaultValue = null;
                if (tds.Count >= 4)
                {
                    var defaultCell = tds[3];
                    var em = defaultCell.QuerySelector("em");
                    if (em is not null && NormaliseWhitespace(em.TextContent).Equals("required", StringComparison.OrdinalIgnoreCase))
                    {
                        optional = false;
                    }
                    else
                    {
                        var codeDef = defaultCell.QuerySelector("code");
                        if (codeDef is not null)
                        {
                            defaultValue = NormaliseWhitespace(codeDef.TextContent);
                            optional = true;
                        }
                        else
                        {
                            var rawText = NormaliseWhitespace(defaultCell.TextContent);
                            if (!string.IsNullOrEmpty(rawText) && !rawText.Equals("required", StringComparison.OrdinalIgnoreCase))
                            {
                                defaultValue = rawText;
                                optional = true;
                            }
                        }
                    }
                }

                result.Add(new ParamInfo(
                    name: name,
                    type: type,
                    doc: string.IsNullOrEmpty(doc) ? null : doc,
                    optional: optional,
                    @default: defaultValue));
            }
            return result;
        }

        // 2025: <ul><li class="doc-section-item field-body">
        //   <b><code>name</code></b> (<code>type</code>) – <div class="doc-md-description">...</div>
        // </li></ul>
        var ul = FindSectionUl(block, "Parameters:");
        if (ul is not null)
        {
            foreach (var li in ul.Children.Where(c => c.LocalName == "li" && c.ClassList.Contains("doc-section-item")))
            {
                // The first <b><code> is the name, the first NON-bold <code> is the type.
                var nameCode = li.QuerySelector("b > code");
                if (nameCode is null) continue;
                var name = NormaliseWhitespace(nameCode.TextContent);
                if (string.IsNullOrEmpty(name)) continue;
                name = name.TrimStart('*', '&').Trim();

                // Type: the next <code> that is NOT inside a <b>.
                string type = "object";
                foreach (var code in li.QuerySelectorAll("code"))
                {
                    if (code.Parent is IElement pE && pE.LocalName == "b") continue;
                    if (code.Parent is IElement pE2 && pE2.ClassList.Contains("doc-md-description")) continue;
                    // Skip if inside the description div.
                    var inDescription = false;
                    var cur = code.ParentElement;
                    while (cur is not null && cur != li)
                    {
                        if (cur.LocalName == "div" && cur.ClassList.Contains("doc-md-description")) { inDescription = true; break; }
                        cur = cur.ParentElement;
                    }
                    if (inDescription) continue;
                    type = NormaliseWhitespace(code.TextContent);
                    break;
                }

                var doc = ExtractInlineDocFromLi(li);

                // 2025 ul-list-of-li doesn't have a separate "Default" column. mkdocstrings
                // renders defaults inline in the signature only. Best-effort: leave optional=false
                // unless we can do better. (We could re-parse the signature, but that hasn't been
                // an issue for Allplan in practice.)
                result.Add(new ParamInfo(
                    name: name,
                    type: type,
                    doc: string.IsNullOrEmpty(doc) ? null : doc,
                    optional: false,
                    @default: null));
            }
        }

        return result;
    }

    /// <summary>
    /// Extract the Returns section. Supports both 2024 (table) and 2025 (ul) layouts.
    /// </summary>
    static ReturnInfo? ExtractReturns(IElement block)
    {
        // 2024: <table>
        var table = FindSectionTable(block, "Returns:");
        if (table is not null)
        {
            var firstRow = table.QuerySelector("tbody > tr.doc-section-item");
            if (firstRow is null) return null;
            var tds = firstRow.QuerySelectorAll("td").ToList();
            if (tds.Count < 2) return null;
            var type = NormaliseWhitespace(tds[0].TextContent);
            if (string.IsNullOrEmpty(type)) type = "object";
            var doc = ExtractDescriptionFromCell(tds[1]);
            return new ReturnInfo(type, doc ?? "");
        }

        // 2025: <ul>
        var ul = FindSectionUl(block, "Returns:");
        if (ul is not null)
        {
            var li = ul.Children.FirstOrDefault(c => c.LocalName == "li" && c.ClassList.Contains("doc-section-item"));
            if (li is null) return null;
            // First <code> NOT inside the description.
            string type = "object";
            foreach (var code in li.QuerySelectorAll("code"))
            {
                var inDescription = false;
                var cur = code.ParentElement;
                while (cur is not null && cur != li)
                {
                    if (cur.LocalName == "div" && cur.ClassList.Contains("doc-md-description")) { inDescription = true; break; }
                    cur = cur.ParentElement;
                }
                if (inDescription) continue;
                type = NormaliseWhitespace(code.TextContent);
                break;
            }
            var doc = ExtractInlineDocFromLi(li);
            return new ReturnInfo(type, doc ?? "");
        }

        return null;
    }

    /// <summary>
    /// Extract the Raises section. Both layouts supported.
    /// </summary>
    static List<ThrowsInfo> ExtractRaises(IElement block)
    {
        var result = new List<ThrowsInfo>();

        // 2024: <table>
        var table = FindSectionTable(block, "Raises:");
        if (table is not null)
        {
            foreach (var tr in table.QuerySelectorAll("tbody > tr.doc-section-item"))
            {
                var tds = tr.QuerySelectorAll("td").ToList();
                if (tds.Count < 2) continue;
                var type = NormaliseWhitespace(tds[0].TextContent);
                if (string.IsNullOrEmpty(type)) continue;
                var when = ExtractDescriptionFromCell(tds[1]) ?? "";
                result.Add(new ThrowsInfo(type, when));
            }
            return result;
        }

        // 2025: <ul>
        var ul = FindSectionUl(block, "Raises:");
        if (ul is not null)
        {
            foreach (var li in ul.Children.Where(c => c.LocalName == "li" && c.ClassList.Contains("doc-section-item")))
            {
                string type = "";
                foreach (var code in li.QuerySelectorAll("code"))
                {
                    var inDescription = false;
                    var cur = code.ParentElement;
                    while (cur is not null && cur != li)
                    {
                        if (cur.LocalName == "div" && cur.ClassList.Contains("doc-md-description")) { inDescription = true; break; }
                        cur = cur.ParentElement;
                    }
                    if (inDescription) continue;
                    type = NormaliseWhitespace(code.TextContent);
                    break;
                }
                if (string.IsNullOrEmpty(type)) continue;
                var when = ExtractInlineDocFromLi(li) ?? "";
                result.Add(new ThrowsInfo(type, when));
            }
        }
        return result;
    }

    /// <summary>
    /// Find the &lt;table&gt; following a section-title &lt;p&gt; with the given text. Used for
    /// the 2024 layout.
    /// </summary>
    static IElement? FindSectionTable(IElement block, string title)
    {
        var search = block.QuerySelector("div.doc-contents") ?? block;
        foreach (var titleP in search.QuerySelectorAll("p"))
        {
            var span = titleP.QuerySelector("span.doc-section-title");
            if (span is null) continue;
            if (!NormaliseWhitespace(span.TextContent).Equals(title, StringComparison.Ordinal)) continue;
            var sib = titleP.NextElementSibling;
            while (sib is not null && sib.LocalName != "table") sib = sib.NextElementSibling;
            if (sib is not null && sib.LocalName == "table") return sib;
        }
        return null;
    }

    /// <summary>
    /// Find the &lt;ul&gt; following a section-title &lt;p&gt; with the given text. Used for the
    /// 2025 layout.
    /// </summary>
    static IElement? FindSectionUl(IElement block, string title)
    {
        var search = block.QuerySelector("div.doc-contents") ?? block;
        foreach (var titleP in search.QuerySelectorAll("p"))
        {
            var span = titleP.QuerySelector("span.doc-section-title");
            if (span is null) continue;
            if (!NormaliseWhitespace(span.TextContent).Equals(title, StringComparison.Ordinal)) continue;
            var sib = titleP.NextElementSibling;
            while (sib is not null && sib.LocalName != "ul") sib = sib.NextElementSibling;
            if (sib is not null && sib.LocalName == "ul") return sib;
        }
        return null;
    }

    static string? ExtractDescriptionFromCell(IElement td)
    {
        var div = td.QuerySelector("div.doc-md-description");
        var src = div ?? td;
        var text = NormaliseWhitespace(src.TextContent);
        return string.IsNullOrEmpty(text) ? null : text;
    }

    static string? ExtractInlineDocFromLi(IElement li)
    {
        var div = li.QuerySelector("div.doc-md-description");
        if (div is null) return null;
        var text = NormaliseWhitespace(div.TextContent);
        return string.IsNullOrEmpty(text) ? null : text;
    }

    /// <summary>
    /// Pull the first description &lt;p&gt; from a function or attribute block. We look inside
    /// .doc-contents and concatenate every non-section-title paragraph until we hit a section
    /// title (Parameters: / Returns: / Raises:).
    /// </summary>
    static string ExtractFirstDescriptionParagraph(IElement block)
    {
        var contents = block.QuerySelector("div.doc-contents");
        var search = contents ?? block;
        var sb = new StringBuilder();
        foreach (var node in search.Children)
        {
            if (node.LocalName != "p") continue;
            if (node.QuerySelector("span.doc-section-title") is not null) break;
            var text = NormaliseWhitespace(node.TextContent);
            if (string.IsNullOrEmpty(text)) continue;
            if (sb.Length > 0) sb.Append(' ');
            sb.Append(text);
        }
        return sb.ToString();
    }

    // ── Filtering helpers ─────────────────────────────────────────────────────

    /// <summary>
    /// Returns true if `el` is inside another doc-class element that is itself a descendant of
    /// `root`. Used to skip attributes/functions that belong to an inner (nested) class while
    /// processing the outer class.
    /// </summary>
    static bool IsInsideInnerClass(IElement el, IElement root)
    {
        var p = el.ParentElement;
        while (p is not null && p != root)
        {
            if (p.LocalName == "div" && p.ClassList.Contains("doc-class"))
            {
                return true;
            }
            p = p.ParentElement;
        }
        return false;
    }

    // ── String / parser primitives ────────────────────────────────────────────

    internal static string NormaliseWhitespace(string s) =>
        Regex.Replace(s ?? "", @"\s+", " ").Trim();

    /// <summary>
    /// Find the first occurrence of `ch` at bracket/paren-depth 0 in `s`. Returns -1 if none.
    /// </summary>
    static int FindFirstAtDepthZero(string s, char ch)
    {
        int depth = 0;
        for (int i = 0; i < s.Length; i++)
        {
            var c = s[i];
            if (c is '(' or '[' or '{' or '<') { depth++; continue; }
            if (c is ')' or ']' or '}' or '>') { depth--; continue; }
            if (c == ch && depth == 0) return i;
        }
        return -1;
    }
}
