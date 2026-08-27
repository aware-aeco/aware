using Xunit;

namespace AwareTekla.Tests;

public sealed class ModelFreeExecTests
{
    [Theory]
    [InlineData("return new { ok = true, title = args[\"title\"] }; ")]
    [InlineData("return 1 + 2;")]
    [InlineData("// model is deliberately unused\nreturn \"model\";")]
    public void BclAndArgsOnlyScriptsUseTheFastPath(string code)
    {
        Assert.True(Program.TryCreateModelFreeScript(code, out var script));
        Assert.NotNull(script);
    }

    [Theory]
    [InlineData("return model;")]
    [InlineData("var model = 1; return model;")]
    [InlineData("return new Tekla.Structures.Model.Beam();")]
    [InlineData("return new Beam();")]
    [InlineData("#load \"script-that-may-use-model.csx\"")]
    [InlineData("#r \"Tekla.Structures.Model.dll\"\nreturn 1;")]
    [InlineData("return System.Reflection.Assembly.Load(args[\"assembly\"].ToString()).FullName;")]
    [InlineData("dynamic loader = args[\"loader\"]; return loader.Load();")]
    public void HostOrTeklaTypeReferencesKeepTheConnectedPath(string code)
    {
        Assert.False(Program.TryCreateModelFreeScript(code, out var script));
        Assert.Null(script);
    }
}
