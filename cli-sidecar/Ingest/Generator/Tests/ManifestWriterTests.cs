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

    // ── Defensive sanitisation (regression test for v0.30 A4) ────────────────

    static CoverageIR IRWithFragileVerbNames() => new(
        host: "allplan",
        host_version: "2025.0",
        source: new SourceInfo("scrape", new List<string> { "https://pythonparts.allplan.com" }, DateTime.UtcNow),
        types: new List<TypeInfo>
        {
            new(
                name: "FontProvider", @namespace: "NemAll_Python_AllplanSettings", kind: "class",
                summary: "Bad data — name contains YAML-fragile chars.", remarks: null,
                @base: null, interfaces: new List<string>(),
                doc_url: "https://example.com",
                constructors: new List<MethodInfo>(),
                // Three methods whose IR `name` carries the param list (a regression scenario);
                // sanitisation MUST keep the manifest a valid YAML document. The first two have
                // the SAME canonical name with different param names — sanitisation collapses
                // them to the same base verb-id, so the second must get a `-2` suffix.
                methods: new List<MethodInfo>
                {
                    new(
                        name: "GetCharsetList(fontName: str) -> tuple[bool, list[int]]",
                        signature: "GetCharsetList(fontName: str) -> tuple[bool, list[int]]",
                        summary: "Returns the list of available charsets for the given font.",
                        remarks: null, @params: new List<ParamInfo>(), returns: null,
                        throws: new List<ThrowsInfo>(), events_related: new List<string>(),
                        doc_url: "https://example.com/A", since: null, deprecated: null),
                    new(
                        // Same canonical name as the first method — must collide after
                        // sanitisation and get a numeric suffix.
                        name: "GetCharsetList(fontName: str) -> tuple[bool, list[int]]",
                        signature: "GetCharsetList(fontName: str) -> tuple[bool, list[int]]",
                        summary: "Duplicate canonical name — collides under sanitisation.",
                        remarks: null, @params: new List<ParamInfo>(), returns: null,
                        throws: new List<ThrowsInfo>(), events_related: new List<string>(),
                        doc_url: "https://example.com/B", since: null, deprecated: null),
                    new(
                        name: "GetCharsetList(nFontID: int) -> tuple[bool, list[int]]",
                        signature: "GetCharsetList(nFontID: int) -> tuple[bool, list[int]]",
                        summary: "Different params — distinct sanitised verb-id (no suffix).",
                        remarks: null, @params: new List<ParamInfo>(), returns: null,
                        throws: new List<ThrowsInfo>(), events_related: new List<string>(),
                        doc_url: "https://example.com/C", since: null, deprecated: null)
                },
                properties: new List<PropertyInfo>(),
                events: new List<EventInfo>(),
                enum_values: new List<EnumValueInfo>(),
                delegate_signature: null)
        },
        metadata: new MetadataInfo(1, 3, 0, 0, 0));

    [Fact]
    public void QuoteYamlScalar_Quotes_Values_Starting_With_AtSign()
    {
        // Description values that begin with `@` (Doxygen `@brief` etc.) are valid in vendor docs
        // but `@` is a YAML 1.2 reserved indicator — emitting bare causes a scan-time error.
        // Build a one-shot IR with a description carrying `@brief` and assert it round-trips
        // through `serde_yaml`-compatible quoting.
        var ir = new CoverageIR(
            host: "allplan", host_version: "2025.0",
            source: new SourceInfo("scrape", new List<string> { "https://example.com" }, DateTime.UtcNow),
            types: new List<TypeInfo>
            {
                new(
                    name: "Functions", @namespace: "Module", kind: "static-class",
                    summary: "", remarks: null, @base: null, interfaces: new List<string>(),
                    doc_url: "https://example.com",
                    constructors: new List<MethodInfo>(),
                    methods: new List<MethodInfo>
                    {
                        new("CreateElementplan", "CreateElementplan(doc, catOffset, elements)",
                            "@brief Creates the elementplan @param doc DocumentAdapter for ptrArrrayData",
                            null, new List<ParamInfo>(), null,
                            new List<ThrowsInfo>(), new List<string>(), "https://example.com/x",
                            null, null)
                    },
                    properties: new List<PropertyInfo>(),
                    events: new List<EventInfo>(),
                    enum_values: new List<EnumValueInfo>(),
                    delegate_signature: null)
            },
            metadata: new MetadataInfo(1, 1, 0, 0, 0));
        var output = ManifestWriter.Render(ir, "test", "vendor", "vertical");
        // The `@brief` substring must appear inside a double-quoted scalar.
        Assert.Contains("\"@brief Creates the elementplan", output);
        Assert.DoesNotContain("description: @brief", output);
    }

    [Fact]
    public void Sanitises_YamlFragile_VerbNames()
    {
        var output = ManifestWriter.Render(IRWithFragileVerbNames(), "allplan-2025", "allplan", "architecture");

        // Sanitiser MUST strip parens, colons, angle brackets, spaces — none of those may appear
        // before the lifecycle/description block of any verb.
        var lines = output.Split('\n');
        foreach (var line in lines)
        {
            // Locate verb-id lines: "  <verb-id>:" — 2-space indent, ends in colon, no inner colon.
            if (!line.StartsWith("  ") || line.StartsWith("    ")) continue;
            if (!line.TrimEnd().EndsWith(":")) continue;
            var verbLine = line.Trim().TrimEnd(':');
            // Skip non-verb sections (e.g. "cli:" doesn't appear here, but defensive).
            if (verbLine == "cli" || verbLine == "source" || verbLine == "binary") continue;
            Assert.DoesNotContain("(", verbLine);
            Assert.DoesNotContain(")", verbLine);
            Assert.DoesNotContain(":", verbLine);
            Assert.DoesNotContain(" ", verbLine);
            Assert.DoesNotContain("<", verbLine);
            Assert.DoesNotContain(">", verbLine);
            Assert.DoesNotContain("[", verbLine);
            Assert.DoesNotContain("]", verbLine);
        }

        // Both methods sanitise to the same base verb-id; the second must get a numeric suffix
        // so the manifest stays a unique-keyed map.
        Assert.Contains("font-provider-get-charset-list", output);
        Assert.Matches(@"font-provider-get-charset-list[-a-z0-9]*-2", output);
    }
}
