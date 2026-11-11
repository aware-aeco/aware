using System;
using System.Collections.Generic;
using System.Globalization;
using System.IO;
using System.Linq;
using System.Text.Json.Nodes;
using Microsoft.CodeAnalysis;
using Xunit;

namespace AwareTekla.Tests;

/// <summary>
/// `profile-project` (FloLess #3382 A0p). The plan — validation, the mapping onto Tekla's built-in shapes and the
/// exact name text — is pure and executed here; the Tekla half (aliases, the library check and the read-back) is
/// pinned on the script source and proved against a live Tekla by the PR's verification.
/// </summary>
public sealed class ProfileProjectTests
{
    // ── sizes in a name ────────────────────────────────────────────────────────

    [Theory]
    [InlineData(20.0, "20")]
    [InlineData(203.2, "203.2")]
    [InlineData(6.35, "6.35")]
    [InlineData(1234.5678, "1234.5678")]
    [InlineData(9.525, "9.525")]
    [InlineData(0.000001, "0.000001")]
    [InlineData(0.0000123, "0.0000123")]
    [InlineData(1e9, "1000000000")]
    [InlineData(123456789.25, "123456789.25")]
    public void FormatSize_WritesPlainInvariantDigits(double value, string expected)
    {
        Assert.Equal(expected, TeklaProfileProjection.FormatSize(value));
    }

    [Theory]
    [InlineData(1.0 / 3.0)]
    [InlineData(0.1 + 0.2)]
    [InlineData(25.4 * 7 / 16)]
    [InlineData(0.000001)]
    [InlineData(0.0000017320508075688772)]
    [InlineData(999999999.99999988)]
    [InlineData(1e9)]
    [InlineData(168.275)]
    public void FormatSize_RoundTripsBitForBitWithNoExponentOrComma(double value)
    {
        var text = TeklaProfileProjection.FormatSize(value);
        Assert.DoesNotContain("E", text, StringComparison.OrdinalIgnoreCase);
        Assert.DoesNotContain(",", text);
        Assert.Equal(BitConverter.DoubleToInt64Bits(value),
            BitConverter.DoubleToInt64Bits(double.Parse(text, NumberStyles.Float, CultureInfo.InvariantCulture)));
    }

    [Fact]
    public void FormatSize_IgnoresACommaDecimalCulture()
    {
        // The bridge's own culture on the machine the spike ran on is Polish (7,5): the name must not follow it.
        var saved = CultureInfo.CurrentCulture;
        try
        {
            CultureInfo.CurrentCulture = new CultureInfo("pl-PL");
            Assert.Equal("7.5", TeklaProfileProjection.FormatSize(7.5));
        }
        finally { CultureInfo.CurrentCulture = saved; }
    }

    [Theory]
    [InlineData(0.0)]
    [InlineData(-20.0)]
    [InlineData(0.0000009)]
    [InlineData(1.0000001e9)]
    [InlineData(double.NaN)]
    [InlineData(double.PositiveInfinity)]
    public void FormatSize_RefusesSizesOutsideTheDomain(double value)
    {
        Assert.Throws<ArgumentOutOfRangeException>(() => TeklaProfileProjection.FormatSize(value));
    }

    [Theory]
    [InlineData("1E-06", "0.000001")]
    [InlineData("1.5E-05", "0.000015")]
    [InlineData("1.2345E+3", "1234.5")]
    [InlineData("1E+9", "1000000000")]
    [InlineData("-2.5E-1", "-0.25")]
    [InlineData("12.5", "12.5")]
    public void ExpandExponent_ShiftsTheDecimalPoint(string text, string expected)
    {
        Assert.Equal(expected, TeklaProfileProjection.ExpandExponent(text));
    }

    // ── names ──────────────────────────────────────────────────────────────────

    [Fact]
    public void ComposeName_FollowsTheParameterStringBySymbolNotByPosition()
    {
        var values = new Dictionary<string, double> { ["b"] = 200, ["t"] = 15, ["h"] = 400, ["s"] = 10 };
        Assert.Equal("HI400-10-15*200", TeklaProfileProjection.ComposeName("HI", "h-s-t*b", values));
        Assert.Equal("T200-10-15-100", TeklaProfileProjection.ComposeName("T", "h-s-t-b",
            new Dictionary<string, double> { ["h"] = 200, ["s"] = 10, ["t"] = 15, ["b"] = 100 }));
    }

    [Fact]
    public void ComposeName_RefusesAMissingSymbol()
    {
        var error = Assert.Throws<ArgumentException>(() => TeklaProfileProjection.ComposeName("HI", "h-s-t*b",
            new Dictionary<string, double> { ["h"] = 400, ["s"] = 10, ["t"] = 15 }));
        Assert.Contains("'b'", error.Message);
    }

    [Fact]
    public void SymbolsOf_ReadsTheNameOrder()
    {
        Assert.Equal(new[] { "h", "s", "t", "b" }, TeklaProfileProjection.SymbolsOf("h-s-t*b"));
        Assert.Equal(new[] { "h1", "h2", "s", "t", "b" }, TeklaProfileProjection.SymbolsOf("h1-h2-s-t*b"));
        Assert.Equal(new[] { "d" }, TeklaProfileProjection.SymbolsOf("d"));
    }

    // ── the plan ───────────────────────────────────────────────────────────────

    static TeklaProfileProjection.Plan Plan(string json) => TeklaProfileProjection.PlanSection(JsonNode.Parse(json));

    static string Named(TeklaProfileProjection.Plan plan, string prefix) =>
        TeklaProfileProjection.ComposeName(prefix, plan.Target!.ParameterString, plan.Values.ToDictionary(v => v.Symbol, v => v.Value));

    [Theory]
    [InlineData("""{"shape":"rect","w":20,"d":200}""", "PROFILE_PL", "PROFILE_PL_DEFAULT", "PL", "PL200*20", false)]
    [InlineData("""{"shape":"rect","w":200,"d":20}""", "PROFILE_PL", "PROFILE_PL_DEFAULT", "PL", "PL200*20", true)]
    [InlineData("""{"shape":"rect","w":50,"d":50}""", "PROFILE_PL", "PROFILE_PL_DEFAULT", "PL", "PL50*50", false)]
    [InlineData("""{"shape":"i","d":400,"bf":200,"tw":10,"tf":15}""", "PROFILE_I", "PROFILE_I_WELDED_SYMMETRICAL", "HI", "HI400-10-15*200", false)]
    [InlineData("""{"shape":"tee","d":200,"bf":100,"tw":10,"tf":15}""", "PROFILE_T", "PROFILE_T_PARAMETRIC", "T", "T200-10-15-100", false)]
    [InlineData("""{"shape":"angle","d":100,"b":80,"t":8}""", "PROFILE_FPL", "PROFILE_L_COLD_ROLLED", "L", "L100*80*8", false)]
    [InlineData("""{"shape":"channel","d":200,"bf":100,"tw":5,"tf":5}""", "PROFILE_U", "PROFILE_U_COLD_ROLLED", "U", "U200*100*5", false)]
    [InlineData("""{"shape":"rhs","d":200,"b":100,"t":5}""", "PROFILE_P", "PROFILE_P_RECTANGULAR", "P", "P200*100*5", false)]
    [InlineData("""{"shape":"rhs","d":200,"b":200,"t":5}""", "PROFILE_P", "PROFILE_P_RECTANGULAR", "P", "P200*200*5", false)]
    [InlineData("""{"shape":"chs","od":168.3,"t":6}""", "PROFILE_PD", "PROFILE_PD_CIRCULAR", "PD", "PD168.3*6", false)]
    [InlineData("""{"shape":"solid-circle","diameter":25.4}""", "PROFILE_D", "PROFILE_D_CIRCULAR", "D", "D25.4", false)]
    public void PlanSection_MapsEachCanonicalShapeOntoItsTeklaShape(string canonical, string type, string subtype, string prefix, string name, bool quarterTurn)
    {
        var plan = Plan(canonical);
        Assert.Equal("planned", plan.Status);
        Assert.Equal(type, plan.Target!.Type);
        Assert.Equal(subtype, plan.Target.SubType);
        Assert.Equal(prefix, plan.Target.DefaultPrefix);
        Assert.Equal(name, Named(plan, prefix));
        Assert.Equal(quarterTurn, plan.QuarterTurn);
        Assert.Equal(TeklaProfileProjection.SymbolsOf(plan.Target.ParameterString), plan.Values.Select(v => v.Symbol));
    }

    [Fact]
    public void PlanSection_RecordsWhichCanonicalSizeEachSymbolCameFrom()
    {
        var plan = Plan("""{"shape":"i","d":400,"bf":200,"tw":10,"tf":15}""");
        Assert.Equal(new[] { "h<-d", "s<-tw", "t<-tf", "b<-bf" }, plan.Values.Select(v => v.Symbol + "<-" + v.From));
        var wide = Plan("""{"shape":"rect","w":200,"d":20}""");
        Assert.Equal(new[] { "h<-w", "b<-d" }, wide.Values.Select(v => v.Symbol + "<-" + v.From));
    }

    [Theory]
    // Each bound at, inside and outside. "planned" inside, "invalid" at and outside.
    [InlineData("""{"shape":"i","d":30,"bf":200,"tw":10,"tf":15}""", "invalid")]
    [InlineData("""{"shape":"i","d":30.000001,"bf":200,"tw":10,"tf":15}""", "planned")]
    [InlineData("""{"shape":"i","d":400,"bf":10,"tw":10,"tf":15}""", "invalid")]
    [InlineData("""{"shape":"i","d":400,"bf":10.000001,"tw":10,"tf":15}""", "planned")]
    [InlineData("""{"shape":"tee","d":15,"bf":100,"tw":10,"tf":15}""", "invalid")]
    [InlineData("""{"shape":"tee","d":15.000001,"bf":100,"tw":10,"tf":15}""", "planned")]
    [InlineData("""{"shape":"tee","d":200,"bf":10,"tw":10,"tf":15}""", "invalid")]
    [InlineData("""{"shape":"angle","d":100,"b":8,"t":8}""", "invalid")]
    [InlineData("""{"shape":"angle","d":100,"b":8.000001,"t":8}""", "planned")]
    [InlineData("""{"shape":"rhs","d":200,"b":10,"t":5}""", "invalid")]
    [InlineData("""{"shape":"rhs","d":200,"b":10.000001,"t":5}""", "planned")]
    [InlineData("""{"shape":"chs","od":12,"t":6}""", "invalid")]
    [InlineData("""{"shape":"chs","od":12.000001,"t":6}""", "planned")]
    [InlineData("""{"shape":"channel","d":200,"bf":75,"tw":6,"tf":9}""", "no-shape")]
    [InlineData("""{"shape":"double-angle","d":100,"b":100,"t":10,"gap":10,"orientation":"llbb"}""", "no-shape")]
    [InlineData("""{"shape":"rect","w":-20,"d":200}""", "invalid")]
    [InlineData("""{"shape":"rect","w":0,"d":200}""", "invalid")]
    [InlineData("""{"shape":"rect","w":20}""", "invalid")]
    [InlineData("""{"shape":"rect","w":"20","d":200}""", "invalid")]
    [InlineData("""{"shape":"solid-circle","diameter":1e10}""", "invalid")]
    [InlineData("""{"shape":"solid-circle","d":25}""", "invalid")]
    [InlineData("""{"shape":"z","d":100}""", "invalid")]
    [InlineData("""{"d":100}""", "invalid")]
    [InlineData("""[1,2]""", "invalid")]
    public void PlanSection_ChecksEveryBoundAndNamesNothingItCannot(string canonical, string status)
    {
        var plan = Plan(canonical);
        Assert.Equal(status, plan.Status);
        if (status != "planned")
        {
            Assert.Null(plan.Target);
            Assert.False(string.IsNullOrWhiteSpace(plan.Message));
        }
    }

    [Fact]
    public void Targets_DeclareTheHiddenRadiusOnlyWhereTeklaDerivesOne()
    {
        // Measured: L/U r = 1.5 t, P r = 2 t, HI/T r = 0.7 s, none on PL, PD or D (#3382 A0p spike).
        foreach (var shape in new[] { "i", "tee", "angle", "channel", "rhs" })
            Assert.Equal(new[] { "r" }, TeklaProfileProjection.Targets[shape].Hidden);
        foreach (var shape in new[] { "rect", "chs", "solid-circle" })
            Assert.Empty(TeklaProfileProjection.Targets[shape].Hidden);
    }

    // ── the request ────────────────────────────────────────────────────────────

    [Theory]
    [InlineData("""[]""", "JSON object")]
    [InlineData("""{}""", "\"sections\" must be a list")]
    [InlineData("""{"sections":[]}""", "is empty")]
    [InlineData("""{"sections":[1]}""", "sections[0] must be an object")]
    [InlineData("""{"sections":[{"canonical":{}}]}""", "sections[0].id")]
    [InlineData("""{"sections":[{"id":"  ","canonical":{}}]}""", "sections[0].id")]
    [InlineData("""{"sections":[{"id":"a","canonical":{}},{"id":"a","canonical":{}}]}""", "used twice")]
    [InlineData("""{"sections":[{"id":"a"}]}""", "no \"canonical\"")]
    [InlineData("""{"sections":[{"id":"a","canonical":{}}],"prefixes":[]}""", "\"prefixes\" must be an object")]
    [InlineData("""{"sections":[{"id":"a","canonical":{}}],"prefixes":{"z":"Z"}}""", "names 'z'")]
    [InlineData("""{"sections":[{"id":"a","canonical":{}}],"prefixes":{"i":3}}""", "\"prefixes.i\"")]
    public void ValidateRequest_RefusesAMalformedRequestBeforeTeklaIsTouched(string json, string reason)
    {
        var refusal = TeklaProfileProjection.ValidateRequest(JsonNode.Parse(json));
        Assert.NotNull(refusal);
        Assert.Contains(reason, refusal);
    }

    [Fact]
    public void ValidateRequest_RefusesMoreSectionsThanTheCap()
    {
        var sections = new JsonArray();
        for (int i = 0; i <= TeklaProfileProjection.MaxSections; i++)
            sections.Add(new JsonObject { ["id"] = "s" + i, ["canonical"] = new JsonObject() });
        Assert.Contains("at most", TeklaProfileProjection.ValidateRequest(new JsonObject { ["sections"] = sections }));
    }

    [Fact]
    public void ValidateRequest_AcceptsAWellFormedRequest()
    {
        Assert.Null(TeklaProfileProjection.ValidateRequest(JsonNode.Parse(
            """{"sections":[{"id":"a","canonical":{"shape":"rect","w":1,"d":2}}],"prefixes":{"i":"WI"}}""")));
    }

    [Fact]
    public void PlanToArgs_EchoesTheNeutralPayloadAndCarriesTheExactValues()
    {
        var section = (JsonObject)JsonNode.Parse(
            """{"id":"m1","canonical":{"shape":"rect","w":0.1,"d":0.2},"definition":{"id":"x"},"sizes":{"t":1},"faces":{"start":{}},"reference_point":[0,0],"other":1}""")!;
        var plan = TeklaProfileProjection.PlanSection(section["canonical"]);
        var args = TeklaProfileProjection.PlanToArgs(section, plan, "FLT");
        foreach (var echo in new[] { "canonical", "definition", "sizes", "faces", "reference_point" })
            Assert.True(JsonNode.DeepEquals(section[echo], args[echo]), echo);
        Assert.Null(args["other"]);
        Assert.Equal("FLT", args["requested_prefix"]!.GetValue<string>());
        var values = Program.JsonNodeToObject(args["values"]) as List<object?>;
        var first = (Dictionary<string, object?>)values![0]!;
        Assert.Equal(BitConverter.DoubleToInt64Bits(0.2), BitConverter.DoubleToInt64Bits((double)first["value"]!));
    }

    // ── the Tekla half, on its source ──────────────────────────────────────────

    [Fact]
    public void Script_ParsesAndOnlyAsksTheCatalogue()
    {
        var code = ProfileProjectScript.Code;
        var tree = Microsoft.CodeAnalysis.CSharp.CSharpSyntaxTree.ParseText(
            code, new Microsoft.CodeAnalysis.CSharp.CSharpParseOptions(kind: SourceCodeKind.Script));
        Assert.DoesNotContain(tree.GetDiagnostics(), d => d.Severity == DiagnosticSeverity.Error);
        foreach (var forbidden in new[] { ".Insert(", ".Modify(", ".Delete(", "CommitChanges", "SetUserProperty", ".Save(", "CreateProfileString" })
            Assert.DoesNotContain(forbidden, code);
        // The library is asked first (Tekla resolves it before a parametric name), then the parametric read-back.
        Assert.True(code.IndexOf("new LibraryProfileItem().Select(name)", StringComparison.Ordinal)
                    < code.IndexOf("back.Select(name)", StringComparison.Ordinal));
        Assert.Contains("\"library-collision\"", code);
        Assert.Contains("DoubleToInt64Bits(p.Value) != BitConverter.DoubleToInt64Bits(intended[p.Symbol])", code);
        Assert.Contains("readSubtype != subtype", code);
        Assert.Contains("an unexpected parameter", code);
        Assert.Contains("if (named.Count > 0) result[\"measure\"]", code);
        Assert.Contains("!m.GetConnectionStatus()", code);
    }

    [Fact]
    public void Verb_IsDispatchedAndRunsScriptOwnedSoNothingIsCommitted()
    {
        var source = File.ReadAllText(FindProgramSource());
        Assert.Contains("case \"profile-project\":", source);
        var call = source.Substring(source.IndexOf("ExecuteResolvedScript(\"profile-project\"", StringComparison.Ordinal));
        Assert.StartsWith("ExecuteResolvedScript(\"profile-project\", ProfileProjectScript.Code", call);
        Assert.Contains("ScriptCommitPolicy.ScriptOwned", call.Substring(0, call.IndexOf(';')));
        // Refused before Tekla: validation precedes the pid lookup and the script.
        var handler = source.Substring(source.IndexOf("static int ProfileProject(", StringComparison.Ordinal));
        Assert.True(handler.IndexOf("ValidateRequest(input)", StringComparison.Ordinal) < handler.IndexOf("TryReadPid(", StringComparison.Ordinal));
    }

    static string FindProgramSource()
    {
        var dir = new DirectoryInfo(AppContext.BaseDirectory);
        while (dir != null && !File.Exists(Path.Combine(dir.FullName, "cli-tekla.csproj")))
            dir = dir.Parent;
        Assert.NotNull(dir);
        return Path.Combine(dir!.FullName, "Program.cs");
    }
}
