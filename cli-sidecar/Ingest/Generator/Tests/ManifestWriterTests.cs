using Xunit;

namespace AwareSidecar.Ingest.Tests;

public class ManifestWriterTests
{
    static CoverageIR SampleIR() => new(
        host: "tekla",
        host_version: "2026.0",
        source: new SourceInfo("scrape", new List<string> { "https://developer.tekla.com" }, DateTime.UtcNow),
        types: new List<TypeInfo>
        {
            new(
                name: "Model", @namespace: "Tekla.Structures.Model", kind: "class",
                summary: "Represents the Tekla model.", remarks: null, @base: null,
                interfaces: new List<string>(), doc_url: "https://example.com",
                constructors: new List<MethodInfo>(),
                methods: new List<MethodInfo>
                {
                    new("GetConnectionStatus", "bool GetConnectionStatus()",
                        "Returns true once connected.", null, new List<ParamInfo>(), null,
                        new List<ThrowsInfo>(), new List<string>(), "https://example.com/GetConnectionStatus",
                        null, null)
                },
                properties: new List<PropertyInfo>(),
                events: new List<EventInfo>(),
                enum_values: new List<EnumValueInfo>(),
                delegate_signature: null)
        },
        metadata: new MetadataInfo(1, 1, 1, 0, 0));

    [Fact]
    public void Renders_Agent_Name_And_Version()
    {
        var output = ManifestWriter.Render(SampleIR(), "tekla-2026", "trimble", "engineering");
        Assert.Contains("agent:        tekla-2026", output);
        Assert.Contains("version:      0.30.0", output);
        Assert.Contains("sdk-target:   2026.0", output);
    }

    [Fact]
    public void Renders_Methods_As_Verbs()
    {
        var output = ManifestWriter.Render(SampleIR(), "tekla-2026", "trimble", "engineering");
        Assert.Contains("model-get-connection-status:", output);
        Assert.Contains("Returns true once connected.", output);
    }

    [Fact]
    public void Lists_Each_Namespace_As_Skill()
    {
        var output = ManifestWriter.Render(SampleIR(), "tekla-2026", "trimble", "engineering");
        Assert.Contains("- tekla-structures-model.md", output);
    }
}
