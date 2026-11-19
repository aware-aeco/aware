using System;
using System.IO;
using System.Linq;
using Microsoft.CodeAnalysis;
using Xunit;

namespace AwareTekla.Tests;

/// <summary>
/// A5 spike (aware#859): the experimental scripts under cli-tekla/spike/item-create/ are not part of the shipped bridge,
/// but they are the only code that will write a person's Tekla shape catalogue and insert Items, so their guards are
/// pinned here. The scripts parse as exec scripts, and every guard that keeps a write inside a throwaway model, inside
/// the scratch root and behind a proven backup is present in the source, and runs before the first catalogue call. The
/// Tekla-facing behaviour is proved by the spike itself in a live Tekla; compiling the scripts against the Tekla 2026.0
/// assemblies is a manual step recorded in the plan (it needs the install folder, which CI does not have).
/// </summary>
public sealed class ItemCreateSpikeTests
{
    private static string SpikeDir()
    {
        var dir = new DirectoryInfo(AppContext.BaseDirectory);
        while (dir != null && !File.Exists(Path.Combine(dir.FullName, "cli-tekla", "cli-tekla.csproj"))) dir = dir.Parent;
        Assert.NotNull(dir);
        return Path.Combine(dir!.FullName, "cli-tekla", "spike", "item-create");
    }

    private static string Read(string name) => File.ReadAllText(Path.Combine(SpikeDir(), name));

    /// <summary>The script without its comment lines, so a name mentioned in the header is not mistaken for a call.</summary>
    private static string Code(string name) =>
        string.Join("\n", Read(name).Split('\n').Where(line => !line.TrimStart().StartsWith("//", StringComparison.Ordinal)));

    private static int First(string text, string needle)
    {
        var at = text.IndexOf(needle, StringComparison.Ordinal);
        Assert.True(at >= 0, "expected to find: " + needle);
        return at;
    }

    [Theory]
    [InlineData("probe.csx")]
    [InlineData("readback.csx")]
    [InlineData("shape-create.csx")]
    [InlineData("item-place.csx")]
    public void SpikeScripts_ParseWithNoSyntaxErrors(string file)
    {
        var tree = Microsoft.CodeAnalysis.CSharp.CSharpSyntaxTree.ParseText(
            Read(file), new Microsoft.CodeAnalysis.CSharp.CSharpParseOptions(kind: SourceCodeKind.Script));
        var errors = tree.GetDiagnostics().Where(d => d.Severity == DiagnosticSeverity.Error).Select(d => d.ToString()).ToList();
        Assert.True(errors.Count == 0, file + " has syntax errors:\n" + string.Join("\n", errors));
    }

    [Theory]
    [InlineData("probe.csx")]
    [InlineData("readback.csx")]
    [InlineData("shape-create.csx")]
    [InlineData("item-place.csx")]
    public void EverySpikeScript_RefusesAModelThatIsNotAThrowawayA5SpikeModel(string file)
    {
        var text = Code(file);
        Assert.Contains("StartsWith(\"A5SPIKE\"", text);
        Assert.Contains("throw new Exception(\"Refusing:", text);
        // The A2 spike's name must not be accepted by a copy-paste slip.
        Assert.DoesNotContain("A2SPIKE", text);
    }

    [Theory]
    [InlineData("probe.csx")]
    [InlineData("readback.csx")]
    public void ReadOnlyScripts_NeverCallACatalogueOrModelWrite(string file)
    {
        var text = Code(file);
        foreach (var write in new[] { ".Insert(", ".Modify(", ".Delete(", ".Rename(", "SetMetadata(", "DeleteMetadata(", "ImportShape", ".Export(", ".Copy(", "SaveProfileDatabase", "CommitChanges", "InsertUsingNormals", "InsertOrGet" })
            Assert.DoesNotContain(write, text);
    }

    [Fact]
    public void ShapeCreate_HoldsEveryWriteBehindTheFiveGuards()
    {
        var text = Code("shape-create.csx");
        // 2. the model folder and XS_PROFDB are inside the scratch root, or the master's written OK names this model.
        Assert.Contains("shared_catalog_ok", text);
        Assert.Contains("\"MASTER-OK:\" + leaf", text);
        // 3. a backup was proven against the files as they are now, and no file is missing from it.
        Assert.Contains("args.backup_id is empty", text);
        Assert.Contains("no backup hash recorded for", text);
        Assert.Contains("changed since its backup hash was taken", text);
        // 4. no silent overwrite: Tekla's import overrides without a warning, and a same-name insert is unproven.
        Assert.Contains("would overwrite it silently", text);
        Assert.Contains("expect_absent", text);
        Assert.Contains("expect_present", text);
        // 5. only spike names, within Tekla's documented length.
        Assert.Contains("StartsWith(\"AWI\"", text);
        Assert.Contains("MaxShapeNameLength = 1023", text);
        // The guards run before the first catalogue call.
        var firstWrite = new[]
            {
                "shape.Insert()", "shape.InsertUsingNormalsAllowDuplicates()", "InsertOrGetGuidsOfShapesWithMatchingGeometry(", "SetMetadata(",
                "s.Rename(", "s.Delete()", "s.Export(", "ImportShapeItems(", "ImportShapeItemsFromGeometryFiles(",
            }
            .Select(w => text.IndexOf(w, StringComparison.Ordinal)).Where(i => i >= 0).Min();
        foreach (var guard in new[]
        {
            "StartsWith(\"A5SPIKE\"", "shared_catalog_ok", "args.backup_id is empty", "changed since its backup hash",
            "would overwrite it silently", "does not start with AWI", "must name every shape this call touches",
        })
            Assert.True(First(text, guard) < firstWrite, guard + " must come before the first catalogue write");
    }

    [Fact]
    public void ShapeCreate_KeepsExportAndImportFoldersInsideTheScratchRootAndOutsideTheModelFolder()
    {
        var text = Code("shape-create.csx");
        Assert.Contains("out_dir is outside scratch_root", text);
        Assert.Contains("out_dir must not be inside the model folder", text);
        Assert.Contains("import_dir is outside scratch_root", text);
        Assert.Contains("import_dir must not be inside the model folder", text);
    }

    [Fact]
    public void ShapeCreate_ChecksTheGeometryWithTeklasOwnValidatorBeforeOfferingItToTheCatalogue()
    {
        var text = Code("shape-create.csx");
        Assert.True(First(text, "Polymesh.Validate(") < First(text, "shape.Insert()"));
        Assert.Contains("PolymeshCheckerFlags.All", text);
        // Caps face outwards: the x = 0 cap runs clockwise, the x = L cap counter-clockwise, and the hole winding is tried both ways.
        Assert.Contains("faces.Add(ringIds(0, 0).Reverse().ToArray())", text);
        Assert.Contains("innerWinding", text);
    }

    [Fact]
    public void ItemPlace_NeverWritesTheShapeCatalogueAndRestoresTheWorkPlane()
    {
        var text = Code("item-place.csx");
        foreach (var write in new[] { "shape.Insert", "SetMetadata(", "DeleteMetadata(", ".Delete(", ".Rename(", ".Modify(", "ImportShape", ".Export(", "SaveProfileDatabase" })
            Assert.DoesNotContain(write, text);
        Assert.Contains("Refusing: the model folder is not under the scratch root", text);
        Assert.Contains("Refusing: shape must start with AWI", text);
        Assert.Contains("Refusing: item_name must start with AWI-ITEM", text);
        Assert.Contains("Refusing: no shape named", text);
        // The one write it does make is the item, and only after the guards.
        Assert.True(First(text, "Refusing: no shape named") < First(text, "item.Insert()"));
        // The user's work plane is put back even when the read-back throws.
        Assert.True(First(text, "finally") < First(text, "SetCurrentTransformationPlane(previousPlane)"));
    }

    [Fact]
    public void RunSpike_RefusesAWriteWithoutWriteAndAShapeCreateWithoutABackup()
    {
        var text = Read("run-spike.ps1");
        Assert.Contains("pass -Write to say so", text);
        Assert.Contains("shape-create needs -BackupManifest", text);
        Assert.Contains("C:\\A5SPIKE\\", text);
        Assert.Contains("backup_id", text);
        Assert.Contains("'probe', 'readback', 'shape-create', 'item-place'", text);
    }

    [Fact]
    public void SnapshotScript_CoversTheModelsShapeCatalogueAndTheProfileCatalogue()
    {
        var text = Read("snapshot-shapes.ps1");
        Assert.Contains("'Shapes', 'ShapeGeometries'", text);
        Assert.Contains("'profdb.bin', 'profitab.inp'", text);
        Assert.Contains("VERIFIED:", text);
        Assert.Contains("NOT IDENTICAL", text);
    }

    [Fact]
    public void TheSpikeScriptsAddNoVerbOfTheirOwn()
    {
        // The spike scripts run through the existing exec verb; they are not dispatched. The one real verb that grew out
        // of them is `item-create` (aware#859 step 2), which is a compiled script with its own tests (ItemCreateTests).
        var program = File.ReadAllText(Path.Combine(SpikeDir(), "..", "..", "Program.cs"));
        Assert.DoesNotContain("shape-create", program);
        Assert.DoesNotContain("item-place", program);
        Assert.Equal(1, System.Text.RegularExpressions.Regex.Matches(program, "case \"item-create\":").Count);
    }
}
