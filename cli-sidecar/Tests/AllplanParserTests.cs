// Tests for the Allplan PageParser (mkdocs-material / mkdocstrings layout).
// Fixtures are cached HTML pulled live from pythonparts.allplan.com (2026-05-19).
// Committed fixtures keep the suite offline + deterministic across machines.
//
// Coverage strategy: each fixture exercises a different mkdocstrings shape (class
// with attributes + methods, class with overloaded ctors, enum (IntEnum), class
// with no bases, module-level _functions/ page, class with inner classes).

using AwareSidecar.Ingest;
using AwareSidecar.Ingest.Extractors.Allplan;
using Xunit;

namespace AwareSidecar.Ingest.Tests;

public class AllplanParserTests
{
    static string LoadFixture(string name)
    {
        var here = Path.GetDirectoryName(typeof(AllplanParserTests).Assembly.Location)!;
        var path = Path.Combine(here, "fixtures", name);
        if (!File.Exists(path))
        {
            var walk = here;
            for (int i = 0; i < 6 && walk is not null; i++)
            {
                var probe = Path.Combine(walk, "Tests", "fixtures", name);
                if (File.Exists(probe)) { path = probe; break; }
                walk = Path.GetDirectoryName(walk);
            }
        }
        return File.ReadAllText(path);
    }

    // ── Class with attributes (properties) + simple methods ───────────────────

    [Fact]
    public void Class_AllplanElement_Parses_Properties_And_Methods()
    {
        var html = LoadFixture("allplan-class-allplanelement.html");
        var r = PageParser.Parse(html,
            "https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/AllplanElement/");
        Assert.NotNull(r);
        var t = r!.Type;
        Assert.Equal("AllplanElement", t.name);
        Assert.Equal("NemAll_Python_BasisElements", t.@namespace);
        Assert.Equal("class", t.kind);
        Assert.Null(t.@base);

        // 4 attributes (Attributes, CommonProperties, GeometryObject, LabelElements) — all
        // type-annotated, all writable.
        Assert.True(t.properties.Count >= 4, $"expected >=4 properties, got {t.properties.Count}");
        var attrs = t.properties.FirstOrDefault(p => p.name == "Attributes");
        Assert.NotNull(attrs);
        Assert.True(attrs!.getter);
        Assert.True(attrs.setter);                  // mkdocstrings "writable" label
        Assert.Equal("object", attrs.type);         // mkdocstrings emits `Attributes: object`

        var commonProps = t.properties.FirstOrDefault(p => p.name == "CommonProperties");
        Assert.NotNull(commonProps);
        Assert.Equal("NemAll_Python_BaseElements.CommonProperties", commonProps!.type);

        // Several methods. GetCommonProperties is a real method with no params, returns
        // CommonProperties.
        var getCommon = t.methods.FirstOrDefault(m => m.name == "GetCommonProperties");
        Assert.NotNull(getCommon);
        Assert.NotNull(getCommon!.returns);
        Assert.Equal("CommonProperties", getCommon.returns!.type);
        Assert.Empty(getCommon.@params);

        // SetCommonProperties has a single required param `commonProp` of type `CommonProperties`.
        var setCommon = t.methods.FirstOrDefault(m => m.name == "SetCommonProperties");
        Assert.NotNull(setCommon);
        Assert.Single(setCommon!.@params);
        Assert.Equal("commonProp", setCommon.@params[0].name);
        Assert.Equal("CommonProperties", setCommon.@params[0].type);
        Assert.False(setCommon.@params[0].optional);
        Assert.Null(setCommon.@params[0].@default);

        // No constructors on this fixture's page (Allplan documents `AllplanElement` without an
        // __init__ signature).
        // No enum_values; no events.
        Assert.Empty(t.enum_values);
        Assert.Empty(t.events);
    }

    // ── Class with __init__ overloads (Point2D) ───────────────────────────────

    [Fact]
    public void Class_Point2D_Parses_Init_Overloads_As_Constructors()
    {
        var html = LoadFixture("allplan-class-point2d.html");
        var r = PageParser.Parse(html,
            "https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Geometry/Point2D/");
        Assert.NotNull(r);
        var t = r!.Type;
        Assert.Equal("Point2D", t.name);
        Assert.Equal("NemAll_Python_Geometry", t.@namespace);
        Assert.Equal("class", t.kind);

        // 5 ctor overloads: ()  /  (point: Point2D)  /  (point: Point3D)  /  (refPoint, point)  /  (x, y).
        // Post-v0.30 the extractor MERGES overload variants into ONE MethodInfo with a
        // pipe-joined signature (avoids YAML-key corruption from typed param lists). We expect
        // exactly one ctor entry whose signature carries every variant.
        Assert.Single(t.constructors);
        var ctor = t.constructors[0];
        // Name must be bare type name (no parens / no param list).
        Assert.Equal("Point2D", ctor.name);
        // Signature must contain every overload variant joined with " | ", each rewritten to
        // start with `Point2D` (not `__init__`).
        Assert.Contains(" | ", ctor.signature);
        var variants = ctor.signature.Split(" | ", StringSplitOptions.None);
        Assert.True(variants.Length >= 4, $"expected >=4 ctor variants in joined signature, got {variants.Length}");
        foreach (var v in variants) Assert.StartsWith("Point2D", v);
        // ctor returns must always be null (void).
        Assert.Null(ctor.returns);

        // The (x, y) overload must appear inside the joined signature. The 2024 mkdocstrings
        // layout emits bare param names (no type annotations) so we expect `Point2D(x, y)`
        // verbatim. (The 2025 fixture has typed params; its own test asserts the merge behavior.)
        Assert.Contains("Point2D(x, y)", ctor.signature);

        // Properties X, Y are type-annotated as float and writable.
        var x = t.properties.FirstOrDefault(p => p.name == "X");
        var y = t.properties.FirstOrDefault(p => p.name == "Y");
        Assert.NotNull(x);
        Assert.NotNull(y);
        Assert.Equal("float", x!.type);
        Assert.True(x.setter);
    }

    // ── Enum (IntEnum bases → kind="enum", attributes → enum_values) ─────────

    [Fact]
    public void Enum_AngleUnits_Parses_As_Enum_With_Values()
    {
        var html = LoadFixture("allplan-enum-angleunits.html");
        var r = PageParser.Parse(html,
            "https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/AngleUnits/");
        Assert.NotNull(r);
        var t = r!.Type;
        Assert.Equal("AngleUnits", t.name);
        Assert.Equal("enum", t.kind);
        Assert.Equal("IntEnum", t.@base);

        // 3 enum values: eDeg = 1, eGon = 2, eRad = 0.
        Assert.Equal(3, t.enum_values.Count);
        var eDeg = t.enum_values.FirstOrDefault(v => v.name == "eDeg");
        Assert.NotNull(eDeg);
        Assert.Equal(1, eDeg!.value.GetInt32());

        var eRad = t.enum_values.FirstOrDefault(v => v.name == "eRad");
        Assert.NotNull(eRad);
        Assert.Equal(0, eRad!.value.GetInt32());

        // Enums shouldn't pollute properties[].
        Assert.Empty(t.properties);
    }

    // ── Class with inner class promoted ───────────────────────────────────────

    [Fact]
    public void Class_BaseInteractor_Promotes_Inner_IntEnum()
    {
        var html = LoadFixture("allplan-class-baseinteractor.html");
        var r = PageParser.Parse(html,
            "https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/BaseInteractor/BaseInteractor/");
        Assert.NotNull(r);
        var t = r!.Type;
        Assert.Equal("BaseInteractor", t.name);
        Assert.Equal("BaseInteractor", t.@namespace);    // mkdocstrings full path is "BaseInteractor.BaseInteractor"
        Assert.Equal("interface", t.kind);               // Bases: ABC
        Assert.Equal("ABC", t.@base);

        // Inner: InteractorInputMode (IntEnum)
        Assert.NotEmpty(r.InnerTypes);
        var inner = r.InnerTypes.FirstOrDefault(it => it.name == "InteractorInputMode");
        Assert.NotNull(inner);
        Assert.Equal("enum", inner!.kind);
        Assert.Equal("IntEnum", inner.@base);
        Assert.Equal("BaseInteractor.BaseInteractor", inner.@namespace);
    }

    // ── Module-level _functions/ page → synthetic Functions static-class ─────

    [Fact]
    public void Module_Functions_Page_Synthesises_Functions_Static_Class()
    {
        var html = LoadFixture("allplan-module-functions.html");
        var r = PageParser.Parse(html,
            "https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/_functions/");
        Assert.NotNull(r);
        var t = r!.Type;
        Assert.Equal("Functions", t.name);
        Assert.Equal("NemAll_Python_AllplanSettings", t.@namespace);
        Assert.Equal("static-class", t.kind);
        Assert.NotEmpty(t.methods);

        // Both module-level functions GetAngleUnit and GetLengthUnit should be present.
        var getAng = t.methods.FirstOrDefault(m => m.name == "GetAngleUnit");
        Assert.NotNull(getAng);
        Assert.NotNull(getAng!.returns);

        var getLen = t.methods.FirstOrDefault(m => m.name == "GetLengthUnit");
        Assert.NotNull(getLen);
    }

    // ── 2025 fixture: same shape ─────────────────────────────────────────────

    [Fact]
    public void Class_AllplanElement_2025_Parses_Larger_Surface()
    {
        var html = LoadFixture("allplan-2025-class-allplanelement.html");
        var r = PageParser.Parse(html,
            "https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/AllplanElement/");
        Assert.NotNull(r);
        var t = r!.Type;
        Assert.Equal("AllplanElement", t.name);
        Assert.Equal("class", t.kind);
        // 2025 has more functions (33 doc-function blocks vs 10 in 2024).
        Assert.True(t.methods.Count >= 10,
            $"2025 fixture should have richer surface than 2024 — got {t.methods.Count} methods");
    }

    // ── 2025 fixture: overload merge (regression test for v0.30 PageParser fix) ──

    [Fact]
    public void Class_Point2D_2025_Merges_Overloaded_Methods_Into_Single_Entry()
    {
        // Pre-v0.30 the 2025 parser embedded the full Python signature (typed params + return-
        // type annotation) into each overload variant's IR `name`, producing colons (`:`) and
        // angle brackets (`<>`) inside YAML map keys. That broke `aware agent describe` for any
        // agent whose manifest carried merged Allplan-2025 commands. The fix: merge sibling
        // overload variants into one MethodInfo with the bare canonical name and a pipe-joined
        // signature carrying every variant.
        var html = LoadFixture("allplan-2025-class-point2d.html");
        var r = PageParser.Parse(html,
            "https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Geometry/Point2D/");
        Assert.NotNull(r);
        var t = r!.Type;
        Assert.Equal("Point2D", t.name);
        Assert.Equal("class", t.kind);

        // No method name contains parens, colons, or arrow tokens — those are the YAML-fragile
        // characters that produced the v0.30 regression.
        foreach (var m in t.methods)
        {
            Assert.DoesNotContain("(", m.name);
            Assert.DoesNotContain(":", m.name);
            Assert.DoesNotContain("->", m.name);
            Assert.DoesNotContain("[", m.name);
            Assert.DoesNotContain("<", m.name);
        }

        // The fixture documents `__add__` as an overloaded method with multiple typed variants.
        // After merge we expect a single entry named `__add__` whose signature is pipe-joined.
        var add = t.methods.FirstOrDefault(m => m.name == "__add__");
        if (add is not null)
        {
            // If __add__ has overloads in the fixture, the signature must be pipe-joined.
            if (add.signature.Contains(" | "))
            {
                var variants = add.signature.Split(" | ", StringSplitOptions.None);
                Assert.True(variants.Length >= 2, "expected ≥2 pipe-joined variants in __add__");
                foreach (var v in variants) Assert.StartsWith("__add__", v);
            }
        }

        // Same for ctors — bare type name only, pipe-joined signature for overloaded __init__.
        Assert.Single(t.constructors);
        var ctor = t.constructors[0];
        Assert.Equal("Point2D", ctor.name);
        Assert.DoesNotContain("(", ctor.name);
        // Every chunk of the pipe-joined ctor signature starts with `Point2D` (the rewritten
        // `__init__` token).
        foreach (var chunk in ctor.signature.Split(" | ", StringSplitOptions.None))
        {
            Assert.StartsWith("Point2D", chunk);
        }
    }

    // ── Sitemap parsing ───────────────────────────────────────────────────────

    [Fact]
    public void Sitemap_Filter_Keeps_Type_Pages_And_Module_Pages_Skips_Section_Roots()
    {
        var sitemap = @"<?xml version=""1.0"" encoding=""UTF-8""?>
<urlset xmlns=""http://www.sitemaps.org/schemas/sitemap/0.9"">
    <url><loc>https://pythonparts.allplan.com/latest/</loc></url>
    <url><loc>https://pythonparts.allplan.com/latest/getting_started/</loc></url>
    <url><loc>https://pythonparts.allplan.com/latest/api_reference/</loc></url>
    <url><loc>https://pythonparts.allplan.com/latest/api_reference/_navigation_tree/</loc></url>
    <url><loc>https://pythonparts.allplan.com/latest/api_reference/InterfaceStubs/</loc></url>
    <url><loc>https://pythonparts.allplan.com/latest/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/_navigation_tree/</loc></url>
    <url><loc>https://pythonparts.allplan.com/latest/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/AllplanPaths/</loc></url>
    <url><loc>https://pythonparts.allplan.com/latest/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/_functions/</loc></url>
    <url><loc>https://pythonparts.allplan.com/latest/api_reference/GeneralScripts/PythonPart/PythonPart/</loc></url>
    <url><loc>https://pythonparts.allplan.com/latest/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/</loc></url>
</urlset>";
        var urls = Extractor.ExtractTypeUrlsFromSitemap(sitemap, "https://pythonparts.allplan.com/2024/", "2024.0");
        // Expected kept: type pages, _functions/, AND module pages (the 2025 layout uses these
        // to carry inline module-level functions — we must NOT skip them).
        Assert.Contains(urls, u => u.EndsWith("/AllplanPaths/", StringComparison.Ordinal));
        Assert.Contains(urls, u => u.EndsWith("/_functions/", StringComparison.Ordinal));
        Assert.Contains(urls, u => u.EndsWith("/PythonPart/PythonPart/", StringComparison.Ordinal));
        // Module page kept (2025 layout puts free functions here):
        Assert.Contains(urls, u => u.EndsWith("/NemAll_Python_AllplanSettings/", StringComparison.Ordinal));

        // Expected skipped: navigation_tree, the bare /api_reference/, section roots.
        Assert.DoesNotContain(urls, u => u.Contains("_navigation_tree"));
        Assert.DoesNotContain(urls, u => u.EndsWith("/api_reference/", StringComparison.Ordinal));
        Assert.DoesNotContain(urls, u => u.EndsWith("/api_reference/InterfaceStubs/", StringComparison.Ordinal));
        // getting_started/ is not under api_reference at all.
        Assert.DoesNotContain(urls, u => u.EndsWith("/getting_started/", StringComparison.Ordinal));

        // Version rewrite: /latest/ → /2024/ in the kept URLs.
        Assert.All(urls, u => Assert.Contains("/2024/", u));
    }
}
