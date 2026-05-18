using Xunit;

namespace AwareSidecar.Ingest.Tests;

public class SkillWriterTests
{
    [Fact]
    public void Produces_One_Skill_Per_Namespace()
    {
        var ir = ManifestWriterTests_TestData.IRWithTwoNamespaces();
        var skills = SkillWriter.RenderAll(ir);
        Assert.Equal(2, skills.Count);
        Assert.Contains("tekla-structures-model.md", skills.Keys);
        Assert.Contains("tekla-structures-drawing.md", skills.Keys);
    }

    [Fact]
    public void Skill_Has_Frontmatter()
    {
        var ir = ManifestWriterTests_TestData.IRWithTwoNamespaces();
        var skill = SkillWriter.RenderAll(ir)["tekla-structures-model.md"];
        Assert.StartsWith("---", skill);
        Assert.Contains("name: tekla-tekla-structures-model", skill);
    }

    [Fact]
    public void Skill_Includes_All_Methods_With_DocUrl()
    {
        var ir = ManifestWriterTests_TestData.IRWithTwoNamespaces();
        var skill = SkillWriter.RenderAll(ir)["tekla-structures-model.md"];
        Assert.Contains("`bool GetConnectionStatus()`", skill);
        Assert.Contains("[Docs](https://example.com/GetConnectionStatus)", skill);
    }
}

// Shared test fixture used by SkillWriter + (future) CatalogWriter tests.
// Note: A3's ManifestWriterTests has its own private SampleIR() — different name, no collision.
internal static class ManifestWriterTests_TestData
{
    public static CoverageIR IRWithTwoNamespaces() => new(
        host: "tekla",
        host_version: "2026.0",
        source: new SourceInfo("scrape", new List<string> { "https://example.com" }, DateTime.UtcNow),
        types: new List<TypeInfo>
        {
            new("Model", "Tekla.Structures.Model", "class", "Represents the Tekla model.", null, null,
                new(), "https://example.com/Model",
                new(),
                new List<MethodInfo>
                {
                    new("GetConnectionStatus", "bool GetConnectionStatus()", "Returns true once connected.", null,
                        new(), null, new(), new(), "https://example.com/GetConnectionStatus", null, null)
                },
                new(), new(), new(), null),
            new("Drawing", "Tekla.Structures.Drawing", "class", "A drawing.", null, null,
                new(), "https://example.com/Drawing", new(), new(), new(), new(), new(), null)
        },
        metadata: new MetadataInfo(2, 2, 1, 0, 0));
}
