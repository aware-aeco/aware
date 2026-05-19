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

        // 5 ctor overloads: ()  /  (point: Point2D)  /  (point: Point3D)  /  (refPoint, point)  /  (x, y)
        Assert.True(t.constructors.Count >= 4, $"expected >=4 ctors, got {t.constructors.Count}");
        // All constructors must render their signature as "Point2D(...)" not "__init__(...)".
        foreach (var c in t.constructors)
        {
            Assert.StartsWith("Point2D", c.signature);
            // ctor returns must always be null (void).
            Assert.Null(c.returns);
        }

        // The (x, y) overload has two params: x: float, y: float.
        var xyCtor = t.constructors.FirstOrDefault(c =>
            c.@params.Count == 2 && c.@params[0].name == "x" && c.@params[1].name == "y");
        Assert.NotNull(xyCtor);
        Assert.Equal("float", xyCtor!.@params[0].type);
        Assert.Equal("float", xyCtor.@params[1].type);

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
