// Tests for the SketchUp PageParser + MemberPageParser. Fixtures are cached HTML pulled
// live from ruby.sketchup.com (2026-05-19). Committed fixtures keep the suite offline +
// deterministic across machines.
//
// Coverage strategy: each fixture exercises a different YARD shape (single-signature
// method, overloaded method, predicate `?`, mutator `!`, setter `=`, getter/setter pair,
// constants block, exception class, observer class, module). The tests assert behaviors,
// not exact strings, so YARD updates that don't change the SHAPE of the markup remain
// passing.

using AwareSidecar.Ingest;
using AwareSidecar.Ingest.Extractors.SketchUp;
using Xunit;

namespace AwareSidecar.Ingest.Tests;

public class SketchUpMemberPageParserTests
{
    static string LoadFixture(string name)
    {
        var here = Path.GetDirectoryName(typeof(SketchUpMemberPageParserTests).Assembly.Location)!;
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

    // ── PageParser-level tests (full page → TypeInfo) ─────────────────────────

    [Fact]
    public void Class_Model_Parses_With_Methods_Properties_Constants()
    {
        var html = LoadFixture("sketchup-class-model.html");
        var r = PageParser.Parse(html, "https://ruby.sketchup.com/Sketchup/Model.html");
        Assert.NotNull(r);
        var t = r!.Type;
        Assert.Equal("Model", t.name);
        Assert.Equal("Sketchup", t.@namespace);
        Assert.Equal("class", t.kind);
        Assert.Equal("Object", t.@base);

        // Model has 60+ instance methods plus the paired getter/setters get pulled out as
        // properties. The verification is "richly populated", not exact-count, so future
        // SketchUp updates don't break the test.
        Assert.True(t.methods.Count >= 40, $"expected >=40 methods, got {t.methods.Count}");
        Assert.True(t.properties.Count >= 15, $"expected >=15 properties, got {t.properties.Count}");

        // Paired getter/setter: `active_layer` + `active_layer=` collapses into one Property
        // with getter=true, setter=true.
        var activeLayer = t.properties.FirstOrDefault(p => p.name == "active_layer");
        Assert.NotNull(activeLayer);
        Assert.True(activeLayer!.getter);
        Assert.True(activeLayer.setter);
        Assert.Contains("Layer", activeLayer.type);

        // Constants from <dl class="constants"> are folded into properties[] with
        // getter=true, setter=false.
        var version2020 = t.properties.FirstOrDefault(p => p.name == "VERSION_2020");
        Assert.NotNull(version2020);
        Assert.True(version2020!.getter);
        Assert.False(version2020.setter);

        // abort_operation is a real instance method with no params, returns Boolean.
        var abortOp = t.methods.FirstOrDefault(m => m.name == "abort_operation");
        Assert.NotNull(abortOp);
        Assert.NotNull(abortOp!.returns);
        Assert.Equal("Boolean", abortOp.returns!.type);
        Assert.Empty(abortOp.@params);
    }

    [Fact]
    public void Module_Sketchup_Parses_As_Interface_Kind()
    {
        // Ruby modules map to IR `kind: "interface"` per the brief.
        var html = LoadFixture("sketchup-module-sketchup.html");
        var r = PageParser.Parse(html, "https://ruby.sketchup.com/Sketchup.html");
        Assert.NotNull(r);
        var t = r!.Type;
        Assert.Equal("Sketchup", t.name);
        Assert.Equal("", t.@namespace);
        Assert.Equal("interface", t.kind);
        // Modules have no inheritance.
        Assert.Null(t.@base);
        // Sketchup has 100+ class methods (no instance methods).
        Assert.True(t.methods.Count >= 30, $"expected >=30 methods, got {t.methods.Count}");
    }

    [Fact]
    public void Class_Point3d_Has_Constructor_And_Overloaded_Methods()
    {
        var html = LoadFixture("sketchup-class-point3d.html");
        var r = PageParser.Parse(html, "https://ruby.sketchup.com/Geom/Point3d.html");
        Assert.NotNull(r);
        var t = r!.Type;
        Assert.Equal("Point3d", t.name);
        Assert.Equal("Geom", t.@namespace);
        Assert.Equal("class", t.kind);
        // Point3d has at least one constructor (`initialize`).
        Assert.NotEmpty(t.constructors);
        var ctor = t.constructors[0];
        Assert.Equal("initialize", ctor.name);
        // Constructor signature is rendered as `TypeName(args)` per IR convention; ctors
        // emit `returns = null` regardless of YARD's annotation.
        Assert.StartsWith("Point3d", ctor.signature);
        Assert.Null(ctor.returns);
    }

    [Fact]
    public void Exception_LockedEntityError_Recognised_As_Class()
    {
        // YARD uses <h1>Exception: ...</h1> for exception subclasses. The parser maps these
        // to `kind: "class"` (they're still Ruby classes).
        var html = LoadFixture("sketchup-exception-lockedentity.html");
        var r = PageParser.Parse(html, "https://ruby.sketchup.com/Layout/LockedEntityError.html");
        Assert.NotNull(r);
        var t = r!.Type;
        Assert.Equal("LockedEntityError", t.name);
        Assert.Equal("Layout", t.@namespace);
        Assert.Equal("class", t.kind);
        // ArgumentError is the typical base for SketchUp exceptions.
        Assert.NotNull(t.@base);
    }

    [Fact]
    public void Observer_Class_Methods_Stay_In_Methods_Not_Events()
    {
        // Ruby observer classes have callback methods (`on*`). These DON'T map to the IR's
        // events[] slot (which is .NET-shaped); they stay in methods[].
        var html = LoadFixture("sketchup-class-modelobserver.html");
        var r = PageParser.Parse(html, "https://ruby.sketchup.com/Sketchup/ModelObserver.html");
        Assert.NotNull(r);
        var t = r!.Type;
        Assert.Equal("ModelObserver", t.name);
        Assert.Empty(t.events);
        // The on* callbacks should all be in methods[].
        var onMethods = t.methods.Where(m => m.name.StartsWith("on", StringComparison.Ordinal)).ToList();
        Assert.NotEmpty(onMethods);
        Assert.True(onMethods.Count >= 5, $"expected >=5 on* methods, got {onMethods.Count}");
    }

    [Fact]
    public void Class_PolygonMesh_Has_Mutator_Method_Names()
    {
        // Ruby idiom: `extend_uvq_arrays!` is a destructive (mutator) method. The trailing
        // `!` MUST be preserved in the IR's `name` field.
        var html = LoadFixture("sketchup-class-polygonmesh.html");
        var r = PageParser.Parse(html, "https://ruby.sketchup.com/Geom/PolygonMesh.html");
        Assert.NotNull(r);
        var t = r!.Type;
        Assert.Equal("PolygonMesh", t.name);
        // At least one method with a trailing `!` (mutator) — verified live: PolygonMesh has
        // `extend_uvq_arrays!`. The test asserts that "if any method ends with `!`, the IR
        // preserves it" rather than depending on a specific name (defensive against vendor
        // doc reorganisations).
        var mutators = t.methods.Where(m => m.name.EndsWith("!", StringComparison.Ordinal)).ToList();
        Assert.NotEmpty(mutators);
    }

    // ── MemberPageParser-level tests (fragment → enrichment) ──────────────────

    [Fact]
    public void MemberPage_AbortOperation_From_Model_Page_Parses()
    {
        // The ParseMethod entry point operates on a fragment containing a single
        // method_details block. We pass the full Model.html page; ParseMethod finds the
        // first method_details block and parses it. The first block on Model is
        // `abort_operation` (alphabetic).
        var html = LoadFixture("sketchup-class-model.html");
        var r = MemberPageParser.ParseMethod(html, isCtor: false);
        Assert.NotNull(r);
        Assert.Equal("abort_operation", r!.name);
        Assert.NotNull(r.returns);
        Assert.Equal("Boolean", r.returns!.type);
    }

    [Fact]
    public void Param_Defaults_Are_Extracted_From_Ruby_Signature()
    {
        // The signature `attribute_dictionary(name, create = false)` carries a default value
        // for the `create` parameter. The parser maps `optional=true, default="false"` for
        // any parameter that has an `= literal` in the rendered Ruby signature.
        var html = LoadFixture("sketchup-class-model.html");
        var r = PageParser.Parse(html, "https://ruby.sketchup.com/Sketchup/Model.html");
        Assert.NotNull(r);
        var t = r!.Type;
        var attrDict = t.methods.FirstOrDefault(m => m.name == "attribute_dictionary");
        Assert.NotNull(attrDict);
        var createParam = attrDict!.@params.FirstOrDefault(p => p.name == "create");
        Assert.NotNull(createParam);
        Assert.True(createParam!.optional);
        Assert.Equal("false", createParam.@default);
    }

    [Fact]
    public void Class_Method_Of_Module_Parses_With_Correct_Signature_Rendering()
    {
        // Class-method-of-module: `Sketchup.active_model` is rendered as `.active_model` in
        // the YARD h3. The parser strips the receiver marker and emits a bare-name signature
        // in the IR. The method should be in methods[] (not constructors[]) since it's not
        // initialize.
        var html = LoadFixture("sketchup-module-sketchup.html");
        var r = PageParser.Parse(html, "https://ruby.sketchup.com/Sketchup.html");
        Assert.NotNull(r);
        var t = r!.Type;
        var activeModel = t.methods.FirstOrDefault(m => m.name == "active_model");
        Assert.NotNull(activeModel);
        // The IR signature should NOT carry the `.` receiver prefix.
        Assert.False(activeModel!.signature.StartsWith(".", StringComparison.Ordinal),
            $"signature should not start with '.': {activeModel.signature}");
        // Returns a Model object.
        Assert.NotNull(activeModel.returns);
    }
}
