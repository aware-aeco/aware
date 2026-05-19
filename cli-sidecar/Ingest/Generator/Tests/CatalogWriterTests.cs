using System.Text.Json;
using Xunit;

namespace AwareSidecar.Ingest.Tests;

public class CatalogWriterTests
{
    [Fact]
    public void Produces_One_Json_Per_Type()
    {
        var ir = ManifestWriterTests_TestData.IRWithTwoNamespaces();
        var catalogs = CatalogWriter.RenderAll(ir);
        Assert.Equal(2, catalogs.Count);
        Assert.Contains("Tekla.Structures.Model.Model.json", catalogs.Keys);
        Assert.Contains("Tekla.Structures.Drawing.Drawing.json", catalogs.Keys);
    }

    [Fact]
    public void Catalog_Json_Roundtrips_Through_IRReader_Models()
    {
        var ir = ManifestWriterTests_TestData.IRWithTwoNamespaces();
        var catalogs = CatalogWriter.RenderAll(ir);
        var modelCatalog = JsonSerializer.Deserialize<TypeInfo>(catalogs["Tekla.Structures.Model.Model.json"]);
        Assert.NotNull(modelCatalog);
        Assert.Equal("Model", modelCatalog!.name);
        Assert.Single(modelCatalog.methods);
        Assert.Equal("GetConnectionStatus", modelCatalog.methods[0].name);
    }

    [Fact]
    public void Method_With_Null_Returns_Emits_Returns_Key_Explicitly()
    {
        // Defect 3 — codex-coverage FAIL 2026-05-19: System.Text.Json's WhenWritingNull
        // default-ignore-condition was dropping the `returns` key entirely when null.
        // The IR schema marks `returns` as a REQUIRED property whose value may be null
        // (`{"oneOf":[{"type":"null"},{"$ref":"ReturnInfo"}]}`). The key MUST appear in
        // the serialized JSON. Fix: per-property `[JsonIgnore(Never)]` on `MethodInfo.returns`
        // overrides the global WhenWritingNull. This test pins that behaviour.
        var ir = ManifestWriterTests_TestData.IRWithTwoNamespaces();
        var json = CatalogWriter.RenderAll(ir)["Tekla.Structures.Model.Model.json"];
        // Parse the JSON and traverse to types.methods[0]
        using var doc = JsonDocument.Parse(json);
        var method = doc.RootElement.GetProperty("methods")[0];
        // The `returns` key MUST be present (the sample method has returns=null).
        Assert.True(method.TryGetProperty("returns", out var returnsEl),
            "MethodInfo.returns key must always be emitted (schema requires it). Found JSON: " + method.GetRawText());
        Assert.Equal(JsonValueKind.Null, returnsEl.ValueKind);
    }

    [Fact]
    public void Method_With_Non_Null_Returns_Still_Emits_Returns_Object()
    {
        // Sanity: when a method has a real return type, the `returns` key carries the
        // ReturnInfo object as before.
        var ir = new CoverageIR(
            host: "tekla",
            host_version: "2026.0",
            source: new SourceInfo("scrape", new List<string> { "https://example.com" }, DateTime.UtcNow),
            types: new List<TypeInfo>
            {
                new("Foo", "Tekla.Test", "class", "F", null, null,
                    new(), "https://example.com/Foo",
                    new(),
                    new List<MethodInfo>
                    {
                        new("Get", "int Get()", "Returns one.", null, new(),
                            new ReturnInfo("Int32", "the value"),
                            new(), new(), "https://example.com/Foo/Get", null, null)
                    },
                    new(), new(), new(), null)
            },
            metadata: new MetadataInfo(1, 1, 1, 0, 0));
        var json = CatalogWriter.RenderAll(ir)["Tekla.Test.Foo.json"];
        using var doc = JsonDocument.Parse(json);
        var method = doc.RootElement.GetProperty("methods")[0];
        Assert.True(method.TryGetProperty("returns", out var returnsEl));
        Assert.Equal(JsonValueKind.Object, returnsEl.ValueKind);
        Assert.Equal("Int32", returnsEl.GetProperty("type").GetString());
        Assert.Equal("the value", returnsEl.GetProperty("doc").GetString());
    }

    [Fact]
    public void Ctor_With_Null_Returns_Emits_Returns_Key_Explicitly()
    {
        // Constructors always have `returns = null` since they aren't expression-typed.
        // The Method schema $defs is shared between ctors and methods, so the same
        // `returns must be present` rule applies. Pin it for ctors too — that's the
        // largest single source of schema violations from the FAIL report (1058+ ctors).
        var ir = new CoverageIR(
            host: "tekla",
            host_version: "2026.0",
            source: new SourceInfo("scrape", new List<string> { "https://example.com" }, DateTime.UtcNow),
            types: new List<TypeInfo>
            {
                new("Foo", "Tekla.Test", "class", "F", null, null,
                    new(), "https://example.com/Foo",
                    new List<MethodInfo>
                    {
                        new("Foo()", "Foo()", "Default ctor.", null, new(), null,
                            new(), new(), "https://example.com/Foo/ctor", null, null)
                    },
                    new(), new(), new(), new(), null)
            },
            metadata: new MetadataInfo(1, 1, 0, 0, 0));
        var json = CatalogWriter.RenderAll(ir)["Tekla.Test.Foo.json"];
        using var doc = JsonDocument.Parse(json);
        var ctor = doc.RootElement.GetProperty("constructors")[0];
        Assert.True(ctor.TryGetProperty("returns", out var returnsEl),
            "Constructor returns key must always be emitted. Found: " + ctor.GetRawText());
        Assert.Equal(JsonValueKind.Null, returnsEl.ValueKind);
    }

    [Fact]
    public void Generic_Type_Name_Is_Sanitized_In_Filename_But_Preserved_In_Json()
    {
        // Defect-1+2 follow-up: the parser fix surfaced two generic types in the Tekla
        // 2025 IR — `Enum<E>` and `CustomObservableCollection<T>`. Windows forbids
        // `<` and `>` in filenames. The fix in CatalogWriter is to sanitize the
        // FILENAME only; the JSON body must still carry the real type name verbatim
        // so downstream consumers (skill rendering, agent install) see the real name.
        var ir = new CoverageIR(
            host: "tekla",
            host_version: "2026.0",
            source: new SourceInfo("scrape", new List<string> { "https://example.com" }, DateTime.UtcNow),
            types: new List<TypeInfo>
            {
                new("Enum<E>", "Tekla.Structures.Datatype", "class", "Enum wrapper.", null, null,
                    new(), "https://example.com/Enum", new(), new(), new(), new(), new(), null),
                new("Dict<K, V>", "Tekla.Test", "class", "Dict.", null, null,
                    new(), "https://example.com/Dict", new(), new(), new(), new(), new(), null)
            },
            metadata: new MetadataInfo(2, 2, 0, 0, 0));
        var catalogs = CatalogWriter.RenderAll(ir);

        // Filenames are sanitized — no `<` or `>` in any key.
        foreach (var key in catalogs.Keys)
        {
            Assert.DoesNotContain('<', key);
            Assert.DoesNotContain('>', key);
        }
        Assert.Contains("Tekla.Structures.Datatype.Enum_of_E.json", catalogs.Keys);
        Assert.Contains("Tekla.Test.Dict_of_K__V.json", catalogs.Keys);

        // JSON bodies carry the real type name verbatim.
        var enumJson = catalogs["Tekla.Structures.Datatype.Enum_of_E.json"];
        using var enumDoc = JsonDocument.Parse(enumJson);
        Assert.Equal("Enum<E>", enumDoc.RootElement.GetProperty("name").GetString());

        var dictJson = catalogs["Tekla.Test.Dict_of_K__V.json"];
        using var dictDoc = JsonDocument.Parse(dictJson);
        Assert.Equal("Dict<K, V>", dictDoc.RootElement.GetProperty("name").GetString());
    }
}
