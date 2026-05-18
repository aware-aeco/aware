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
}
