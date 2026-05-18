using AwareSidecar.Ingest;
using AwareSidecar.Ingest.Extractors.Common;
using Xunit;

namespace AwareSidecar.Ingest.Tests;

public class IRBuilderTests
{
    [Fact]
    public void Build_Empty_Returns_Empty_IR()
    {
        var b = new IRBuilder("test", "1.0", "scrape");
        var ir = b.Build();
        Assert.Equal("test", ir.host);
        Assert.Empty(ir.types);
        Assert.Equal(0, ir.metadata.type_count);
        Assert.Equal(0, ir.metadata.method_count);
    }

    [Fact]
    public void AddType_Updates_Metadata_Counts()
    {
        var b = new IRBuilder("test", "1.0", "scrape");
        b.AddSourceUrl("https://example.com");
        b.IncrementPageCount();

        var type = new TypeInfo(
            name: "Foo", @namespace: "Bar", kind: "class",
            summary: "s", remarks: null, @base: null,
            interfaces: new List<string>(), doc_url: "https://example.com/Foo",
            constructors: new List<MethodInfo>(),
            methods: new List<MethodInfo>
            {
                new("M", "void M()", "s", null, new List<ParamInfo>(), null,
                    new List<ThrowsInfo>(), new List<string>(),
                    "https://example.com/M", null, null)
            },
            properties: new List<PropertyInfo>(),
            events: new List<EventInfo>(),
            enum_values: new List<EnumValueInfo>(),
            delegate_signature: null);
        b.AddType(type);
        var ir = b.Build();
        Assert.Single(ir.types);
        Assert.Equal(1, ir.metadata.type_count);
        Assert.Equal(1, ir.metadata.method_count);
        Assert.Equal(1, ir.metadata.page_count);
        Assert.Contains("https://example.com", ir.source.urls);
    }
}
