using System;
using System.Collections.Generic;
using Xunit;

namespace AwareTekla.Tests;

/// <summary>
/// #3312: a profile Tekla's catalogue does not carry becomes a profile-less line instead of aborting the
/// batch, and the open model can be asked which profiles it accepts. The Tekla-facing rules live in
/// Roslyn script text with no executable harness here, so (as for the other bake rules) the properties
/// are pinned on the source, and proved end to end against a live Tekla by the PR's verification.
/// </summary>
public sealed class ProfileLineFallbackTests
{
    private static string Bake => BakeSceneScript.Code;

    [Fact]
    public void ProfileCatalogScript_ParsesWithNoSyntaxErrors()
    {
        var tree = Microsoft.CodeAnalysis.CSharp.CSharpSyntaxTree.ParseText(
            ProfileCatalogScript.Code,
            new Microsoft.CodeAnalysis.CSharp.CSharpParseOptions(
                kind: Microsoft.CodeAnalysis.SourceCodeKind.Script));
        var errors = new List<string>();
        foreach (var d in tree.GetDiagnostics())
            if (d.Severity == Microsoft.CodeAnalysis.DiagnosticSeverity.Error)
                errors.Add(d.ToString());
        Assert.True(errors.Count == 0, "ProfileCatalogScript.Code has syntax errors:\n" + string.Join("\n", errors));
    }

    [Fact]
    public void ProfileCatalogScript_IsReadOnly()
    {
        var code = ProfileCatalogScript.Code;
        // It may only ask the catalogue. Any write or commit call here would turn a query into a mutation.
        foreach (var forbidden in new[] { ".Insert(", ".Modify(", ".Delete(", "CommitChanges", "SetUserProperty", ".Save(" })
            Assert.DoesNotContain(forbidden, code);
        Assert.Contains("LibraryProfileItem().Select(", code);
        Assert.Contains("ParametricProfileItem().Select(", code);
    }

    [Fact]
    public void ProfileCatalogVerb_RunsAsScriptOwnedSoNothingIsCommitted()
    {
        var source = System.IO.File.ReadAllText(FindProgramSource());
        Assert.Contains("ExecuteResolvedScript(\"profile-catalog\", ProfileCatalogScript.Code", source);
        Assert.Contains("ScriptCommitPolicy.ScriptOwned", source.Substring(source.IndexOf("ExecuteResolvedScript(\"profile-catalog\"", StringComparison.Ordinal)));
        Assert.Contains("case \"profile-catalog\":", source);
    }

    [Fact]
    public void BakeScript_FallsBackToALineOnlyForVerifiedCatalogueAbsence()
    {
        // The fallback fires when no candidate threw, none was a catalogue name that Tekla still rejected,
        // and the element did not say "native". A real failure (a throw, or a name that IS in the catalogue
        // but whose insert is rejected) therefore keeps its true cause and aborts with a rollback.
        Assert.Contains("profileInCatalogue", Bake);
        Assert.Contains("LibraryProfileItem().Select(name)", Bake);
        Assert.Contains("ParametricProfileItem().Select(name)", Bake);
        Assert.Contains("if(profileInCatalogue(profile))knownRejected=true;", Bake);
        Assert.Contains(
            "if(lineEligible&&!sawError&&!knownRejected&&!String.Equals(profilePolicy,\"native\",StringComparison.Ordinal))return insertProfileLine(",
            Bake);
        // Both failure paths of the candidate loop must mark an error, or an exception would read as "absent".
        Assert.Equal(2, CountOf(Bake, "last=ex;sawError=true;"));
    }

    [Fact]
    public void BakeScript_ProfilePolicyLineGoesStraightToTheLineAndNativeNeverDegrades()
    {
        Assert.Contains("String.Equals(profilePolicy,\"line\",StringComparison.Ordinal))return insertProfileLine(", Bake);
        Assert.Contains("!String.Equals(profilePolicy,\"native\"", Bake);
    }

    [Fact]
    public void BakeScript_LineHasItsOwnReadBackAndProvesTheDesignationBeforeRetirement()
    {
        Assert.Contains("const string lineProfilePlaceholder = \"ROD0\";", Bake);
        // Own read-back path: GUID, placeholder profile and axis — not the solid-vertex / roll verification.
        var start = Bake.IndexOf("insertProfileLine = (", StringComparison.Ordinal);
        var end = Bake.IndexOf("insertBeam = (", StringComparison.Ordinal);
        Assert.True(start >= 0 && end > start, "insertProfileLine must be defined before insertBeam");
        var line = Bake.Substring(start, end - start);
        Assert.Contains("authoritative Beam GUID read-back failed", line);
        Assert.Contains("placeholder line read back as", line);
        Assert.Contains("axis read-back differs", line);
        Assert.DoesNotContain("solidVertices", line);
        // Designation: dedicated property read back, else a Name — and a throw if neither can be proved.
        Assert.Contains("AWARE_REQUESTED_PROFILE", line);
        Assert.Contains("GetUserProperty(\"AWARE_REQUESTED_PROFILE\"", line);
        Assert.Contains("could not be recorded on the placeholder line", line);
        // The ownership fields 1-4 are written by tagAfterInsert only; the line never reuses them.
        Assert.DoesNotContain("USER_FIELD_", line);
    }

    [Fact]
    public void BakeScript_ReportsEachLineInTheReceiptWithoutChangingOrdinaryRows()
    {
        Assert.Contains("profileResolution\"]=\"placeholder-line\"", Bake);
        Assert.Contains("r[\"requestedProfile\"]=lineRequested", Bake);
        Assert.Contains("r[\"nativeProfile\"]=lineProfilePlaceholder", Bake);
        Assert.Contains("profile-not-in-catalogue", Bake);
        Assert.Contains("profile-sent-as-line", Bake);
        Assert.Contains("profileLines=lineProfileById.Count", Bake);
        // The row is added only for a line, so an ordinary receipt stays byte-identical.
        Assert.Contains("if(lineProfileById.TryGetValue(id,out lineRequested))", Bake);
    }

    [Fact]
    public void BakeScript_SingleAngleLineSkipsTheSectionVerification()
    {
        Assert.Contains("if(!lineProfileById.ContainsKey(id)){var rolledX", Bake);
    }

    private static int CountOf(string haystack, string needle)
    {
        int count = 0, at = 0;
        while ((at = haystack.IndexOf(needle, at, StringComparison.Ordinal)) >= 0) { count++; at += needle.Length; }
        return count;
    }

    private static string FindProgramSource()
    {
        var dir = new System.IO.DirectoryInfo(AppContext.BaseDirectory);
        while (dir != null && !System.IO.File.Exists(System.IO.Path.Combine(dir.FullName, "cli-tekla.csproj")))
            dir = dir.Parent;
        Assert.NotNull(dir);
        return System.IO.Path.Combine(dir!.FullName, "Program.cs");
    }
}
