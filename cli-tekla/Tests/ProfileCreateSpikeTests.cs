using System;
using System.IO;
using System.Linq;
using Microsoft.CodeAnalysis;
using Xunit;

namespace AwareTekla.Tests;

/// <summary>
/// A2 spike (aware#858): the experimental scripts under cli-tekla/spike/profile-create/ are not part of the shipped bridge,
/// but they are the only code that will write a person's Tekla catalogue, so their guards are pinned here. The scripts
/// parse as exec scripts, and each guard that keeps a write inside a throwaway model and an isolated, backed-up catalogue
/// is present in the source. The Tekla-facing behaviour is proved by the spike itself in a live Tekla.
/// </summary>
public sealed class ProfileCreateSpikeTests
{
    private static string SpikeDir()
    {
        var dir = new DirectoryInfo(AppContext.BaseDirectory);
        while (dir != null && !File.Exists(Path.Combine(dir.FullName, "cli-tekla", "cli-tekla.csproj"))) dir = dir.Parent;
        Assert.NotNull(dir);
        return Path.Combine(dir!.FullName, "cli-tekla", "spike", "profile-create");
    }

    private static string Read(string name) => File.ReadAllText(Path.Combine(SpikeDir(), name));

    [Theory]
    [InlineData("probe.csx")]
    [InlineData("export.csx")]
    [InlineData("import.csx")]
    public void SpikeScripts_ParseWithNoSyntaxErrors(string file)
    {
        var tree = Microsoft.CodeAnalysis.CSharp.CSharpSyntaxTree.ParseText(
            Read(file), new Microsoft.CodeAnalysis.CSharp.CSharpParseOptions(kind: SourceCodeKind.Script));
        var errors = tree.GetDiagnostics().Where(d => d.Severity == DiagnosticSeverity.Error).Select(d => d.ToString()).ToList();
        Assert.True(errors.Count == 0, file + " has syntax errors:\n" + string.Join("\n", errors));
    }

    [Theory]
    [InlineData("probe.csx")]
    [InlineData("export.csx")]
    [InlineData("import.csx")]
    public void EverySpikeScript_RefusesAModelThatIsNotAThrowawayA2SpikeModel(string file)
    {
        var text = Read(file);
        Assert.Contains("StartsWith(\"A2SPIKE\"", text);
        Assert.Contains("throw new Exception(\"Refusing:", text);
    }

    [Theory]
    [InlineData("probe.csx")]
    [InlineData("export.csx")]
    public void ReadOnlyScripts_NeverCallACatalogueWrite(string file)
    {
        var text = Read(file);
        foreach (var write in new[] { ".Import", ".Modify(", ".Delete(", ".Copy(", ".Insert(", "SaveProfileDatabase" })
            Assert.DoesNotContain(write, text);
    }

    [Fact]
    public void ExportScript_KeepsItsOutputOutsideTheModelFolderAndInsideTheScratchRoot()
    {
        var text = Read("export.csx");
        Assert.Contains("out_dir is outside scratch_root", text);
        Assert.Contains("out_dir must not be inside the model folder", text);
    }

    [Fact]
    public void ImportScript_HoldsEveryWriteBehindTheFiveGuards()
    {
        var text = Read("import.csx");
        // 2. an isolated catalogue folder, or the master's written OK naming this model.
        Assert.Contains("shared_catalog_ok", text);
        Assert.Contains("\"MASTER-OK:\" + leaf", text);
        // 3. a backup proven against the files as they are now.
        Assert.Contains("changed since its backup hash was taken", text);
        Assert.Contains("no backup hash recorded", text);
        // 4. no silent overwrite: Tekla's Import overwrites without a warning.
        Assert.Contains("overwrite it silently", text);
        Assert.Contains("expect_absent", text);
        // 5. only names that start with AWZ.
        Assert.Contains("StartsWith(\"AWZ\"", text);
        // The guards run before the first catalogue call.
        var firstWrite = new[] { "ImportParametricProfileItems(importDir)", "SaveProfileDatabase()", "source.Copy(", "doomed.Delete(" }
            .Select(w => text.IndexOf(w, StringComparison.Ordinal)).Where(i => i >= 0).Min();
        foreach (var guard in new[] { "shared_catalog_ok", "changed since its backup hash", "expect_absent", "StartsWith(\"AWZ\"" })
            Assert.True(text.IndexOf(guard, StringComparison.Ordinal) < firstWrite, guard + " must come before the first catalogue write");
    }

    [Fact]
    public void RunSpike_RefusesAnImportWithoutWriteAndABackup()
    {
        var text = Read("run-spike.ps1");
        Assert.Contains("pass -Write to say so", text);
        Assert.Contains("import needs -BackupManifest", text);
    }
}
