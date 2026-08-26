using Xunit;

namespace AwareTekla.Tests;

public sealed class ModelFreeExecTests
{
    [Theory]
    [InlineData("return new { ok = true, title = args[\"title\"] }; ")]
    [InlineData("return 1 + 2;")]
    [InlineData("// model is deliberately unused\nreturn \"model\";")]
    [InlineData("return System.Reflection.Assembly.Load(args[\"assembly\"].ToString()).FullName;")]
    public void BclAndArgsOnlyScriptsUseTheFastPath(string code)
    {
        Assert.True(Program.CanExecuteWithoutTekla(code));
    }

    [Theory]
    [InlineData("return model;")]
    [InlineData("var model = 1; return model;")]
    [InlineData("return new Tekla.Structures.Model.Beam();")]
    [InlineData("return new Beam();")]
    [InlineData("#load \"script-that-may-use-model.csx\"")]
    [InlineData("#r \"Tekla.Structures.Model.dll\"\nreturn 1;")]
    public void HostOrTeklaTypeReferencesKeepTheConnectedPath(string code)
    {
        Assert.False(Program.CanExecuteWithoutTekla(code));
    }
}
