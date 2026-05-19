// Tests for the Dynamo extractor's XmlDocReader, SignatureTypeNameProvider, and AssemblyReflector.
//
// Fixtures live in cli-sidecar/Tests/fixtures/dynamo-*.xml and are excerpted from the
// real NuGet `DynamoVisualProgramming.Core@4.1.0.4845` package's XML doc files (small
// assemblies: DynamoApplications + VMDataBridge — the rest are large enough that
// committing them as fixtures would bloat the repo).
//
// Coverage targets:
//   - XmlDocReader.Load round-trips real Dynamo XML doc entries
//   - XmlDocReader inline-tag stripping (see, paramref, c, para)
//   - SignatureTypeNameProvider primitive aliases (string, int, bool, void)
//   - SignatureTypeNameProvider generic instantiations and arrays
//   - GenericContext type-parameter resolution
//
// We do NOT exercise AssemblyReflector end-to-end here — that's exercised live in the
// extractor run (see cli-sidecar/Ingest/Output/dynamo-4.1.0.ir.json after extraction).
// Unit-testing it would require a managed DLL fixture, which adds toolchain weight.
// The integration test that exercises AssemblyReflector lives at
// `cli/tests/coverage_e2e.rs::dynamo_*` once we wire one up; for now the strict-verify
// `verify-types-strict.ps1` script's 50/50 PASS run on the real IR is the integration
// proof.

using System.Collections.Immutable;
using AwareSidecar.Ingest.Extractors.Dynamo;
using Xunit;

namespace AwareSidecar.Ingest.Tests;

public class DynamoParserTests
{
    static string LoadFixturePath(string name)
    {
        var here = Path.GetDirectoryName(typeof(DynamoParserTests).Assembly.Location)!;
        var path = Path.Combine(here, "fixtures", name);
        if (File.Exists(path)) return path;
        var walk = here;
        for (int i = 0; i < 6 && walk is not null; i++)
        {
            var probe = Path.Combine(walk, "Tests", "fixtures", name);
            if (File.Exists(probe)) return probe;
            walk = Path.GetDirectoryName(walk);
        }
        throw new FileNotFoundException($"fixture not found: {name}");
    }

    // ── XmlDocReader ────────────────────────────────────────────────────────

    [Fact]
    public void XmlDocReader_Loads_DynamoApplications_With_Real_Member_Keys()
    {
        var reader = XmlDocReader.Load(LoadFixturePath("dynamo-applications.xml"));
        // The fixture has 21 documented members spanning T:, P:, F:, M:, E: prefixes.
        Assert.True(reader.Members.Count >= 20, $"expected >=20 docs, got {reader.Members.Count}");
        // Spot-check a method with parameters.
        var key = "M:Dynamo.Applications.StartupUtils.MakeCLIModel(System.String,System.String,System.String,Dynamo.Models.HostAnalyticsInfo)";
        Assert.True(reader.Members.ContainsKey(key), $"expected key not found: {key}");
        var doc = reader.Members[key];
        Assert.NotNull(doc.Summary);
        Assert.Contains("DynamoModel", doc.Summary!);
        // The param docs should map by parameter NAME (asmPath, userDataFolder, …).
        Assert.True(doc.Params.ContainsKey("asmPath"));
        Assert.Contains("geometry library", doc.Params["asmPath"]);
    }

    [Fact]
    public void XmlDocReader_Captures_Type_Property_Field_Event_Member_Kinds()
    {
        var reader = XmlDocReader.Load(LoadFixturePath("dynamo-applications.xml"));
        Assert.True(reader.Members.ContainsKey("T:DynamoApplications.Properties.Resources"));
        Assert.True(reader.Members.ContainsKey("P:DynamoApplications.Properties.Resources.ResourceManager"));
        Assert.True(reader.Members.ContainsKey("F:Dynamo.Applications.StartupUtils.assemblyNamesToIgnore"));
        Assert.True(reader.Members.ContainsKey("E:Dynamo.Applications.StartupUtils.ASMPreloadFailure"));
    }

    [Fact]
    public void XmlDocReader_NormalizeInline_Strips_See_Cref_Tags()
    {
        const string xml = @"<summary>This is <see cref=""T:Foo.Bar""/> and <paramref name=""arg""/>.</summary>";
        var reader = XmlDocReader.Parse("<doc><members><member name=\"T:X\">" + xml + "</member></members></doc>");
        var member = reader.Members["T:X"];
        Assert.NotNull(member.Summary);
        Assert.Contains("Foo.Bar", member.Summary!);
        Assert.Contains("arg", member.Summary!);
        Assert.DoesNotContain("<see", member.Summary!);
        Assert.DoesNotContain("cref=", member.Summary!);
    }

    [Fact]
    public void XmlDocReader_NormalizeInline_Strips_Paragraph_Tags_And_Code()
    {
        const string xml = @"<summary><para>First paragraph.</para><para>Second <c>code</c>.</para></summary>";
        var reader = XmlDocReader.Parse("<doc><members><member name=\"T:X\">" + xml + "</member></members></doc>");
        var member = reader.Members["T:X"];
        Assert.NotNull(member.Summary);
        Assert.Contains("First paragraph", member.Summary!);
        Assert.Contains("Second", member.Summary!);
        Assert.Contains("code", member.Summary!);
        Assert.DoesNotContain("<para>", member.Summary!);
        Assert.DoesNotContain("<c>", member.Summary!);
    }

    [Fact]
    public void XmlDocReader_Captures_Exceptions_With_Stripped_Cref()
    {
        const string xml = @"<summary>x</summary><exception cref=""T:System.ArgumentNullException"">when arg is null</exception>";
        var reader = XmlDocReader.Parse("<doc><members><member name=\"M:Foo.Bar\">" + xml + "</member></members></doc>");
        var member = reader.Members["M:Foo.Bar"];
        Assert.Single(member.Exceptions);
        Assert.Equal("System.ArgumentNullException", member.Exceptions[0].Cref);
        Assert.Contains("null", member.Exceptions[0].Doc);
    }

    [Fact]
    public void XmlDocReader_Captures_Returns_Doc()
    {
        const string xml = @"<summary>x</summary><returns>a list of strings</returns>";
        // Probe NormalizeInline directly to see if it would return non-null for the inner text.
        var normalized = XmlDocReader.NormalizeInline("a list of strings");
        Assert.True(normalized.Contains("list of strings"), $"NormalizeInline result: '{normalized}'");

        var probe = "<doc><members><member name=\"M:Foo.Bar\">" + xml + "</member></members></doc>";
        var reader = XmlDocReader.Parse(probe);
        var member = reader.Members["M:Foo.Bar"];
        Assert.NotNull(member.ReturnsDoc);
        Assert.Contains("list of strings", member.ReturnsDoc!);
    }

    [Fact]
    public void XmlDocReader_Malformed_Xml_Survives_Without_Throwing()
    {
        // Mis-nested attribute (no equals sign) — XmlReader should fail on the inner content,
        // but the OUTER reader (over <doc><members><member></members></doc>) must not throw
        // and must surface a usable Members map (possibly empty). The test just guards against
        // an unhandled exception escaping XmlDocReader.Parse.
        const string xml = @"<summary>Before <unclosed Tag> after</summary>";
        var reader = XmlDocReader.Parse("<doc><members><member name=\"T:Z\">" + xml + "</member></members></doc>");
        Assert.NotNull(reader);
        // Whether T:Z is present in the dictionary is implementation-defined; we just require
        // that Parse does not throw and yields some Members map.
        Assert.NotNull(reader.Members);
    }

    // ── SignatureTypeNameProvider ───────────────────────────────────────────

    [Fact]
    public void SignatureTypeNameProvider_Maps_Primitives_To_CSharp_Aliases()
    {
        var p = SignatureTypeNameProvider.Instance;
        Assert.Equal("bool", p.GetPrimitiveType(System.Reflection.Metadata.PrimitiveTypeCode.Boolean));
        Assert.Equal("int", p.GetPrimitiveType(System.Reflection.Metadata.PrimitiveTypeCode.Int32));
        Assert.Equal("string", p.GetPrimitiveType(System.Reflection.Metadata.PrimitiveTypeCode.String));
        Assert.Equal("void", p.GetPrimitiveType(System.Reflection.Metadata.PrimitiveTypeCode.Void));
        Assert.Equal("object", p.GetPrimitiveType(System.Reflection.Metadata.PrimitiveTypeCode.Object));
    }

    [Fact]
    public void SignatureTypeNameProvider_SZArray_Renders_With_Brackets()
    {
        Assert.Equal("string[]", SignatureTypeNameProvider.Instance.GetSZArrayType("string"));
        Assert.Equal("Foo.Bar[]", SignatureTypeNameProvider.Instance.GetSZArrayType("Foo.Bar"));
    }

    [Fact]
    public void SignatureTypeNameProvider_GenericInstantiation_Renders_With_Angles()
    {
        var args = ImmutableArray.Create("string", "int");
        var inst = SignatureTypeNameProvider.Instance.GetGenericInstantiation(
            "System.Collections.Generic.Dictionary", args);
        Assert.Equal("System.Collections.Generic.Dictionary<string, int>", inst);
    }

    [Fact]
    public void SignatureTypeNameProvider_ByRefAndPointer_Renderable()
    {
        Assert.Equal("ref int", SignatureTypeNameProvider.Instance.GetByReferenceType("int"));
        Assert.Equal("int*", SignatureTypeNameProvider.Instance.GetPointerType("int"));
    }

    [Fact]
    public void SignatureTypeNameProvider_TypeGenericParameter_Uses_Name_From_Context()
    {
        var ctx = new GenericContext(
            typeParams: ImmutableArray.Create("TKey", "TValue"),
            methodParams: ImmutableArray<string>.Empty);
        Assert.Equal("TKey", SignatureTypeNameProvider.Instance.GetGenericTypeParameter(ctx, 0));
        Assert.Equal("TValue", SignatureTypeNameProvider.Instance.GetGenericTypeParameter(ctx, 1));
    }

    [Fact]
    public void SignatureTypeNameProvider_MethodGenericParameter_Uses_Method_Names()
    {
        var ctx = new GenericContext(
            typeParams: ImmutableArray<string>.Empty,
            methodParams: ImmutableArray.Create("U"));
        Assert.Equal("U", SignatureTypeNameProvider.Instance.GetGenericMethodParameter(ctx, 0));
        // Out-of-range falls back to placeholder.
        Assert.Equal("!!1", SignatureTypeNameProvider.Instance.GetGenericMethodParameter(ctx, 1));
    }

    [Fact]
    public void SignatureTypeNameProvider_MultiDimArray_Renders_With_Commas()
    {
        var shape = new System.Reflection.Metadata.ArrayShape(
            rank: 3,
            sizes: ImmutableArray<int>.Empty,
            lowerBounds: ImmutableArray<int>.Empty);
        Assert.Equal("int[,,]", SignatureTypeNameProvider.Instance.GetArrayType("int", shape));
    }
}
