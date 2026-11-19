using System;
using System.IO;
using System.Linq;
using System.Text;
using System.Text.Json.Nodes;
using Microsoft.CodeAnalysis;
using Xunit;

namespace AwareTekla.Tests;

/// <summary>
/// `profile-create` (aware#858, alias route). The whole write path is plain file I/O, so it runs here against a throwaway
/// `profitab.inp`: row formatting, prefix rules, conflict and reuse, byte-exact encoding and line-ending preservation, the
/// verified backup, the receipt, the exact undo, and every stage of a write failing part-way. The Tekla half
/// (`profile-verify`) is pinned on its source and proved against a live Tekla by the PR's verification.
/// </summary>
public sealed class ProfileCreateTests : IDisposable
{
    static readonly Encoding Latin1 = Encoding.GetEncoding("ISO-8859-1");
    readonly string _dir = Path.Combine(Path.GetTempPath(), "aware-pc-" + Guid.NewGuid().ToString("N").Substring(0, 8));

    public ProfileCreateTests() { Directory.CreateDirectory(_dir); }
    public void Dispose() { try { Directory.Delete(_dir, true); } catch { } }

    // A trimmed profitab.inp: comments (one with a non-ASCII byte), real USER rows, a non-USER row and a short line.
    const string Table =
        "/*-------+------+----+---+----+----+------------------+------------------+\r\n" +
        "/* Prefix! Type ! SO ! Z ! MI ! MA ! G3-NAME          ! Z3-NAME          !\r\n" +
        "/* Bäsic steel shapes\r\n" +
        "HI       ! I    !    !   !  2 !  2 !                  !\r\n" +
        "TRI_D    ! USER ! 0  !   !  2 !  2 !TEKLAINC.TRIANGLE !h*b\r\n" +
        "TRI_E    ! USER ! 0  !   !  4 !  4 !TEKLAINC.PCTRI    !b*h*h2*b2\r\n" +
        "OBLVAR_C ! USER ! 0  !   !  8 !  8 !TEKLAINC.OBLVAR   !h1-h2-h3\r\n" +
        "garbage line\r\n";

    string Env(string text = Table, bool latin1 = true)
    {
        var folder = Path.Combine(_dir, "Profil");
        Directory.CreateDirectory(folder);
        File.WriteAllBytes(Path.Combine(folder, "profitab.inp"), (latin1 ? Latin1 : Encoding.UTF8).GetBytes(text));
        return folder;
    }

    static string Table_(string folder) => Path.Combine(folder, "profitab.inp");

    JsonObject Request(string folder, string mode, string prefix = "AWTRI", string aliasOf = "TEKLAINC.TRIANGLE", string? parameters = null)
    {
        var o = new JsonObject
        {
            ["mode"] = mode,
            ["profile_database"] = folder,
            ["prefix"] = prefix,
            ["alias_of"] = aliasOf,
            ["backup_dir"] = Path.Combine(_dir, "backups"),
        };
        if (parameters != null) o["parameter_string"] = parameters;
        return o;
    }

    static string Status(JsonObject r) => r["status"]!.GetValue<string>();

    // ── prefix and target rules ───────────────────────────────────────────────

    [Theory]
    [InlineData("AWTRI")]
    [InlineData("AW-TRI")]
    [InlineData("AW_TRI")]
    [InlineData("A1B2C3D")]
    [InlineData("ABCDEFGH")]
    [InlineData("A")]
    public void ValidatePrefix_AcceptsLettersDigitsDashUnderscoreUpToEight(string prefix)
        => Assert.Null(TeklaProfileCreate.ValidatePrefix(prefix));

    [Theory]
    [InlineData(null, "required")]
    [InlineData("", "required")]
    [InlineData("ABCDEFGHI", "9 characters")]          // 9 registers a broken PROFILE_UNKNOWN entry
    [InlineData("AWZ5LONGPREFIXNAME", "18 characters")]
    [InlineData("awtri", "upper-case")]
    [InlineData("1AW", "start with a letter")]
    [InlineData("AW TRI", "upper-case letters")]
    [InlineData("AW!TRI", "upper-case letters")]       // the profitab column separator
    [InlineData("AW*", "upper-case letters")]
    [InlineData("AWZ12", "ends in a digit")]
    public void ValidatePrefix_RefusesWithTheReason(string? prefix, string reason)
    {
        var message = TeklaProfileCreate.ValidatePrefix(prefix);
        Assert.NotNull(message);
        Assert.Contains(reason, message);
    }

    [Theory]
    [InlineData("TEKLAINC.TRIANGLE", true)]
    [InlineData("MET_C+.C+", true)]
    [InlineData("TRIANGLE", false)]
    [InlineData("A.B.C", false)]
    [InlineData("TEKLAINC.TRI ANGLE", false)]
    [InlineData("", false)]
    public void ValidateAliasOf_NeedsLibraryDotSection(string value, bool ok)
        => Assert.Equal(ok, TeklaProfileCreate.ValidateAliasOf(value) == null);

    [Theory]
    [InlineData("AWTRI\n")]      // .NET's `$` would accept this and split the appended row in two
    [InlineData("AWTRI\r\n")]
    [InlineData("AW1\n")]
    public void ValidatePrefix_RefusesATrailingNewline(string prefix)
        => Assert.NotNull(TeklaProfileCreate.ValidatePrefix(prefix));

    [Fact]
    public void ValidateAliasOf_RefusesATrailingNewline()
        => Assert.NotNull(TeklaProfileCreate.ValidateAliasOf("TEKLAINC.TRIANGLE\n"));

    [Fact]
    public void Create_ATrailingNewlinePrefixWritesNothing()
    {
        var folder = Env();
        var before = File.ReadAllBytes(Table_(folder));
        var (r, exit) = TeklaProfileCreate.Run(Request(folder, "create", prefix: "AWTRI\n"));
        Assert.Equal((2, "invalid-request"), (exit, Status(r)));
        Assert.True(File.ReadAllBytes(Table_(folder)).SequenceEqual(before));
    }

    // ── paths and concurrency ─────────────────────────────────────────────────

    [Fact]
    public void RelativePathsAreStoredAbsoluteSoUndoWorksFromAnotherDirectory()
    {
        var folder = Env();
        var before = File.ReadAllBytes(Table_(folder));
        var saved = Directory.GetCurrentDirectory();
        try
        {
            Directory.SetCurrentDirectory(_dir);
            var request = Request("Profil", "create");
            request["backup_dir"] = "rel-backups";
            var (created, _) = TeklaProfileCreate.Run(request);
            Assert.Equal("created", Status(created));
            var receipt = created["receipt"]!.GetValue<string>();
            Assert.True(Path.IsPathRooted(receipt));
            Assert.True(Path.IsPathRooted(created["file_change"]!["backup"]!.GetValue<string>()));
            Assert.True(Path.IsPathRooted(created["profile"]!["file"]!.GetValue<string>()));
            Directory.SetCurrentDirectory(Path.GetTempPath());
            Assert.Equal("undone", Status(TeklaProfileCreate.Run(new JsonObject { ["mode"] = "undo", ["receipt"] = receipt }).Result));
            Assert.True(File.ReadAllBytes(Table_(folder)).SequenceEqual(before));
        }
        finally { Directory.SetCurrentDirectory(saved); }
    }

    [Fact]
    public void AHeldWriteLockStopsACreateWithoutTouchingTheTable_AndIsReleasedAfterwards()
    {
        var folder = Env();
        var before = File.ReadAllBytes(Table_(folder));
        var saved = TeklaProfileCreate.LockWait;
        TeklaProfileCreate.LockWait = TimeSpan.FromMilliseconds(300);
        try
        {
            var lockPath = Table_(folder) + TeklaProfileCreate.LockSuffix;
            using (new FileStream(lockPath, FileMode.OpenOrCreate, FileAccess.ReadWrite, FileShare.None))   // another writer, here or on another machine
            {
                var (r, exit) = TeklaProfileCreate.Run(Request(folder, "create"));
                Assert.Equal((1, "failed"), (exit, Status(r)));
                Assert.Contains("write lock", r["message"]!.GetValue<string>());
                Assert.True(File.ReadAllBytes(Table_(folder)).SequenceEqual(before));
            }
            File.Delete(lockPath);
            Assert.Equal("created", Status(TeklaProfileCreate.Run(Request(folder, "create")).Result));
            Assert.False(File.Exists(lockPath));   // released and removed
        }
        finally { TeklaProfileCreate.LockWait = saved; }
    }

    [Fact]
    public void ConcurrentCreatesOnOneFileKeepEveryRow()
    {
        var folder = Env();
        var prefixes = new[] { "AWA", "AWB", "AWC", "AWD", "AWE", "AWF" };
        var statuses = new string[prefixes.Length];
        var threads = prefixes.Select((p, i) => new System.Threading.Thread(() =>
            statuses[i] = Status(TeklaProfileCreate.Run(Request(folder, "create", prefix: p)).Result))).ToList();
        threads.ForEach(t => t.Start());
        threads.ForEach(t => t.Join());
        Assert.All(statuses, s => Assert.Equal("created", s));
        var rows = TeklaProfileCreate.ParseRows(Latin1.GetString(File.ReadAllBytes(Table_(folder))));
        foreach (var p in prefixes) Assert.Single(rows, r => r.Prefix == p);
    }

    // ── row formatting ────────────────────────────────────────────────────────

    [Fact]
    public void FormatRow_KeepsTheTemplatesColumnsAndSwapsThePrefix()
    {
        var rows = TeklaProfileCreate.ParseRows(Table);
        var tri = rows.Single(r => r.Prefix == "TRI_D");
        Assert.Equal("AWTRI    ! USER ! 0  !   !  2 !  2 !TEKLAINC.TRIANGLE !h*b", TeklaProfileCreate.FormatRow("AWTRI", tri));
        var parsed = TeklaProfileCreate.ParseRows(TeklaProfileCreate.FormatRow("AWTRI", tri)).Single();
        Assert.Equal(("AWTRI", "USER", "TEKLAINC.TRIANGLE", "h*b"), (parsed.Prefix, parsed.Type, parsed.Target, parsed.Parameters));
    }

    [Fact]
    public void ParseRows_SkipsCommentsAndShortLines()
    {
        var rows = TeklaProfileCreate.ParseRows(Table);
        Assert.Equal(new[] { "TRI_D", "TRI_E", "OBLVAR_C" }, rows.Where(r => r.Type == "USER").Select(r => r.Prefix));
        Assert.DoesNotContain(rows, r => r.Prefix == "garbage line");
    }

    // ── bytes: encoding and line endings ──────────────────────────────────────

    [Fact]
    public void AppendRow_KeepsEveryOriginalByteAndTheCrlf()
    {
        var original = Latin1.GetBytes(Table);
        var after = TeklaProfileCreate.AppendRow(original, "AWTRI    ! USER ! 0  !   !  2 !  2 !TEKLAINC.TRIANGLE !h*b");
        Assert.True(after.Take(original.Length).SequenceEqual(original));
        var tail = Latin1.GetString(after, original.Length, after.Length - original.Length);
        Assert.Equal("AWTRI    ! USER ! 0  !   !  2 !  2 !TEKLAINC.TRIANGLE !h*b\r\n", tail);
        Assert.Contains((byte)0xE4, original);           // the ISO-8859-1 byte survived, not re-encoded as UTF-8
        Assert.DoesNotContain((byte)0xC3, after);
    }

    [Fact]
    public void AppendRow_FollowsAnLfFileAndAddsMissingFinalTerminator()
    {
        var lf = Latin1.GetBytes("A ! USER ! 0 ! ! 1 ! 1 !L.S !a\nB ! USER ! 0 ! ! 1 ! 1 !L.T !a");
        var after = Latin1.GetString(TeklaProfileCreate.AppendRow(lf, "NEW"));
        Assert.Equal("A ! USER ! 0 ! ! 1 ! 1 !L.S !a\nB ! USER ! 0 ! ! 1 ! 1 !L.T !a\nNEW\n", after);
    }

    [Fact]
    public void Create_PreservesLatin1BytesAndCrlfOnDisk()
    {
        var folder = Env();
        var before = File.ReadAllBytes(Table_(folder));
        var (r, exit) = TeklaProfileCreate.Run(Request(folder, "create"));
        Assert.Equal(0, exit);
        Assert.Equal("created", Status(r));
        var after = File.ReadAllBytes(Table_(folder));
        Assert.True(after.Take(before.Length).SequenceEqual(before));
        Assert.Equal(1, after.Skip(before.Length).Count(b => b == (byte)'\n'));
        Assert.Contains((byte)0xE4, after);
        Assert.Equal(before.Length + Latin1.GetBytes("AWTRI    ! USER ! 0  !   !  2 !  2 !TEKLAINC.TRIANGLE !h*b\r\n").Length, after.Length);
    }

    // ── plan, create, reuse, conflict ─────────────────────────────────────────

    [Fact]
    public void Plan_WritesNothingAndNamesTheFileAndTheRestart()
    {
        var folder = Env();
        var before = File.ReadAllBytes(Table_(folder));
        var (r, exit) = TeklaProfileCreate.Run(Request(folder, "plan"), () => true);
        Assert.Equal(0, exit);
        Assert.Equal("planned", Status(r));
        Assert.False(r["written"]!.GetValue<bool>());
        Assert.Equal("after Tekla restarts", r["takes_effect"]!.GetValue<string>());
        Assert.Equal(Table_(folder), r["profile"]!["file"]!.GetValue<string>());
        Assert.Contains("every model", r["profile"]!["shared_note"]!.GetValue<string>());
        Assert.Equal(TeklaProfileCreate.Sha256(before), r["file_change"]!["sha256_before"]!.GetValue<string>());
        Assert.True(File.ReadAllBytes(Table_(folder)).SequenceEqual(before));
        Assert.False(Directory.Exists(Path.Combine(_dir, "backups")));
    }

    [Fact]
    public void Create_WritesBackupAndReceiptAndTheHashesAgree()
    {
        var folder = Env();
        var before = File.ReadAllBytes(Table_(folder));
        var (r, _) = TeklaProfileCreate.Run(Request(folder, "create"), () => true);
        Assert.Equal("created", Status(r));
        Assert.True(r["written"]!.GetValue<bool>());
        Assert.True(r["tekla_running"]!.GetValue<bool>());
        Assert.Contains("NOT seen", r["message"]!.GetValue<string>());
        var fc = r["file_change"]!;
        Assert.Equal(TeklaProfileCreate.Sha256(before), fc["sha256_before"]!.GetValue<string>());
        Assert.Equal(TeklaProfileCreate.Sha256(File.ReadAllBytes(Table_(folder))), fc["sha256_after"]!.GetValue<string>());
        Assert.True(File.ReadAllBytes(fc["backup"]!.GetValue<string>()).SequenceEqual(before));
        var receipt = JsonNode.Parse(File.ReadAllText(r["receipt"]!.GetValue<string>()))!;
        Assert.Equal("written", receipt["state"]!.GetValue<string>());
        Assert.Equal(fc["sha256_before"]!.GetValue<string>(), receipt["sha256_before"]!.GetValue<string>());
        Assert.Equal("profile-verify", r["verify"]!["verb"]!.GetValue<string>());
        Assert.Equal("undo", r["undo"]!["request"]!["mode"]!.GetValue<string>());
        Assert.Empty(Directory.GetFiles(folder).Where(f => !f.EndsWith("profitab.inp")));   // no temp file left behind
    }

    [Fact]
    public void Create_TheSameAliasTwiceIsReusedIdenticalAndWritesNothing()
    {
        var folder = Env();
        TeklaProfileCreate.Run(Request(folder, "create"));
        var written = File.ReadAllBytes(Table_(folder));
        var (r, exit) = TeklaProfileCreate.Run(Request(folder, "create"));
        Assert.Equal(0, exit);
        Assert.Equal("reused-identical", Status(r));
        Assert.False(r["written"]!.GetValue<bool>());
        Assert.True(File.ReadAllBytes(Table_(folder)).SequenceEqual(written));
        Assert.Single(Directory.GetDirectories(Path.Combine(_dir, "backups")));     // no second backup run
    }

    [Fact]
    public void Create_ADifferentDefinitionUnderAnExistingPrefixIsAConflictNeverOverwritten()
    {
        var folder = Env();
        TeklaProfileCreate.Run(Request(folder, "create"));
        var written = File.ReadAllBytes(Table_(folder));
        var (r, exit) = TeklaProfileCreate.Run(Request(folder, "create", aliasOf: "TEKLAINC.PCTRI"));
        Assert.Equal(2, exit);
        Assert.Equal("conflict", Status(r));
        Assert.Contains("TEKLAINC.TRIANGLE", r["message"]!.GetValue<string>());
        Assert.True(File.ReadAllBytes(Table_(folder)).SequenceEqual(written));
    }

    [Fact]
    public void Create_AnExistingTeklaPrefixIsAConflictEvenInAnotherCase()
    {
        var folder = Env();
        var (r, exit) = TeklaProfileCreate.Run(Request(folder, "create", prefix: "TRI_D", aliasOf: "TEKLAINC.PCTRI"));
        Assert.Equal((2, "conflict"), (exit, Status(r)));
        var (same, _) = TeklaProfileCreate.Run(Request(folder, "create", prefix: "TRI_D"));
        Assert.Equal("reused-identical", Status(same));
    }

    [Fact]
    public void Create_ASectionNoRowNamesIsTargetUnknownAndNothingIsWritten()
    {
        var folder = Env();
        var before = File.ReadAllBytes(Table_(folder));
        var (r, exit) = TeklaProfileCreate.Run(Request(folder, "create", aliasOf: "NOSUCH.SECTION"));
        Assert.Equal((2, "target-unknown"), (exit, Status(r)));
        Assert.True(File.ReadAllBytes(Table_(folder)).SequenceEqual(before));
        // The Type column matters: HI is not a USER row, so it cannot be aliased by library name.
        var (viaParameters, _) = TeklaProfileCreate.Run(Request(folder, "plan", parameters: "x*y"));
        Assert.Equal("target-unknown", Status(viaParameters));
    }

    [Theory]
    [InlineData("create", "ABCDEFGHI")]
    [InlineData("plan", "awtri")]
    [InlineData("create", "AWZ1")]
    public void InvalidPrefixIsRefusedBeforeAnyFileIsTouched(string mode, string prefix)
    {
        var folder = Env();
        var before = File.ReadAllBytes(Table_(folder));
        var (r, exit) = TeklaProfileCreate.Run(Request(folder, mode, prefix: prefix));
        Assert.Equal((2, "invalid-request"), (exit, Status(r)));
        Assert.True(File.ReadAllBytes(Table_(folder)).SequenceEqual(before));
        Assert.False(Directory.Exists(Path.Combine(_dir, "backups")));
    }

    [Fact]
    public void MissingFolderOrTableIsInvalidRequest()
    {
        Assert.Equal("invalid-request", Status(TeklaProfileCreate.Run(new JsonObject { ["mode"] = "plan", ["prefix"] = "AWTRI", ["alias_of"] = "TEKLAINC.TRIANGLE" }).Result));
        var (r, exit) = TeklaProfileCreate.Run(Request(Path.Combine(_dir, "nowhere"), "plan"));
        Assert.Equal((2, "invalid-request"), (exit, Status(r)));
        Assert.Equal("invalid-request", Status(TeklaProfileCreate.Run(new JsonObject { ["mode"] = "wipe" }).Result));
    }

    [Fact]
    public void ParameterStringPicksTheMatchingRowWhenSeveralAliasTheSameSection()
    {
        var folder = Env(Table + "TRI_X    ! USER ! 0  !   !  2 !  2 !TEKLAINC.TRIANGLE !b*h\r\n");
        var (r, _) = TeklaProfileCreate.Run(Request(folder, "plan", parameters: "b*h"));
        Assert.Equal("b*h", r["profile"]!["parameter_string"]!.GetValue<string>());
        Assert.Equal("TRI_X", r["profile"]!["template_prefix"]!.GetValue<string>());
    }

    // ── a write that fails part-way ───────────────────────────────────────────

    [Theory]
    [InlineData("after-backup")]
    [InlineData("after-receipt")]
    [InlineData("after-temp")]
    public void AFailureBeforeTheReplaceLeavesTheTableUntouchedAndNoTempFile(string stage)
    {
        var folder = Env();
        var before = File.ReadAllBytes(Table_(folder));
        var (r, exit) = TeklaProfileCreate.Run(Request(folder, "create"), null, s => { if (s == stage) throw new IOException("injected at " + stage); });
        Assert.Equal(1, exit);
        Assert.Contains(Status(r), new[] { "failed", "backup-failed" });
        Assert.Contains("injected", r["message"]!.GetValue<string>());
        Assert.True(File.ReadAllBytes(Table_(folder)).SequenceEqual(before));
        Assert.Empty(Directory.GetFiles(folder).Where(f => !f.EndsWith("profitab.inp")));
    }

    [Fact]
    public void AFailureAfterTheReplaceRestoresTheBackupAndSaysSo()
    {
        var folder = Env();
        var before = File.ReadAllBytes(Table_(folder));
        var (r, exit) = TeklaProfileCreate.Run(Request(folder, "create"), null, s => { if (s == "after-replace") throw new IOException("injected after replace"); });
        Assert.Equal((1, "partial"), (exit, Status(r)));
        Assert.Equal("restored", r["restore"]!.GetValue<string>());
        Assert.False(r["written"]!.GetValue<bool>());
        Assert.True(File.ReadAllBytes(Table_(folder)).SequenceEqual(before));
        var receipt = JsonNode.Parse(File.ReadAllText(r["receipt"]!.GetValue<string>()))!;
        Assert.Equal("failed-restored", receipt["state"]!.GetValue<string>());
        Assert.True(File.ReadAllBytes(r["backup"]!.GetValue<string>()).SequenceEqual(before));
    }

    [Fact]
    public void ABackupThatCannotBeWrittenStopsBeforeTheTableIsTouched()
    {
        var folder = Env();
        var before = File.ReadAllBytes(Table_(folder));
        var blocker = Path.Combine(_dir, "blocker");
        File.WriteAllText(blocker, "a file where a folder is needed");
        var request = Request(folder, "create");
        request["backup_dir"] = blocker;
        var (r, exit) = TeklaProfileCreate.Run(request);
        Assert.Equal((1, "backup-failed"), (exit, Status(r)));
        Assert.False(r["written"]!.GetValue<bool>());
        Assert.True(File.ReadAllBytes(Table_(folder)).SequenceEqual(before));
    }

    [Fact]
    public void ATableChangedBetweenReadAndWriteIsNotOverwritten()
    {
        var folder = Env();
        var changed = Latin1.GetBytes(Table + "OTHER    ! USER ! 0  !   !  2 !  2 !TEKLAINC.TRIANGLE !h*b\r\n");
        var (r, exit) = TeklaProfileCreate.Run(Request(folder, "create"), null, s => { if (s == "after-receipt") File.WriteAllBytes(Table_(folder), changed); });
        Assert.Equal((1, "failed"), (exit, Status(r)));
        Assert.Contains("changed", r["message"]!.GetValue<string>());
        Assert.True(File.ReadAllBytes(Table_(folder)).SequenceEqual(changed));
    }

    // ── undo ──────────────────────────────────────────────────────────────────

    [Fact]
    public void Undo_RestoresTheExactBytesAndIsRepeatable()
    {
        var folder = Env();
        var before = File.ReadAllBytes(Table_(folder));
        var (created, _) = TeklaProfileCreate.Run(Request(folder, "create"));
        var receipt = created["receipt"]!.GetValue<string>();
        var (u, exit) = TeklaProfileCreate.Run(new JsonObject { ["mode"] = "undo", ["receipt"] = receipt });
        Assert.Equal((0, "undone"), (exit, Status(u)));
        Assert.True(File.ReadAllBytes(Table_(folder)).SequenceEqual(before));
        Assert.Equal("undone", JsonNode.Parse(File.ReadAllText(receipt))!["state"]!.GetValue<string>());
        Assert.Equal("undone", Status(TeklaProfileCreate.Run(new JsonObject { ["mode"] = "undo", ["receipt"] = receipt }).Result));
        // and the alias can be created again afterwards
        Assert.Equal("created", Status(TeklaProfileCreate.Run(Request(folder, "create")).Result));
    }

    [Fact]
    public void Undo_RefusesWhenTheTableChangedSinceUnlessForced()
    {
        var folder = Env();
        var before = File.ReadAllBytes(Table_(folder));
        var (created, _) = TeklaProfileCreate.Run(Request(folder, "create"));
        var receipt = created["receipt"]!.GetValue<string>();
        File.AppendAllText(Table_(folder), "LATER    ! USER ! 0  !   !  2 !  2 !TEKLAINC.PCTRI !h*b\r\n", Latin1);
        var later = File.ReadAllBytes(Table_(folder));
        var (refused, exit) = TeklaProfileCreate.Run(new JsonObject { ["mode"] = "undo", ["receipt"] = receipt });
        Assert.Equal((2, "changed-since"), (exit, Status(refused)));
        Assert.True(File.ReadAllBytes(Table_(folder)).SequenceEqual(later));
        var (forced, _) = TeklaProfileCreate.Run(new JsonObject { ["mode"] = "undo", ["receipt"] = receipt, ["force"] = true });
        Assert.Equal("undone", Status(forced));
        Assert.True(File.ReadAllBytes(Table_(folder)).SequenceEqual(before));
    }

    [Fact]
    public void Undo_RefusesADamagedBackup()
    {
        var folder = Env();
        var (created, _) = TeklaProfileCreate.Run(Request(folder, "create"));
        var written = File.ReadAllBytes(Table_(folder));
        File.WriteAllText(created["file_change"]!["backup"]!.GetValue<string>(), "tampered");
        var (r, exit) = TeklaProfileCreate.Run(new JsonObject { ["mode"] = "undo", ["receipt"] = created["receipt"]!.GetValue<string>() });
        Assert.Equal((2, "backup-failed"), (exit, Status(r)));
        Assert.True(File.ReadAllBytes(Table_(folder)).SequenceEqual(written));
    }

    [Fact]
    public void Undo_NeedsAReceipt()
    {
        var (r, exit) = TeklaProfileCreate.Run(new JsonObject { ["mode"] = "undo" });
        Assert.Equal((2, "invalid-request"), (exit, Status(r)));
    }

    // ── the Tekla half is pinned on its source ────────────────────────────────

    [Fact]
    public void VerifyScript_ParsesAndNeverWrites()
    {
        var tree = Microsoft.CodeAnalysis.CSharp.CSharpSyntaxTree.ParseText(
            ProfileVerifyScript.Code, new Microsoft.CodeAnalysis.CSharp.CSharpParseOptions(kind: SourceCodeKind.Script));
        var errors = tree.GetDiagnostics().Where(d => d.Severity == DiagnosticSeverity.Error).Select(d => d.ToString()).ToList();
        Assert.True(errors.Count == 0, string.Join("\n", errors));
        foreach (var write in new[] { ".Import", ".Modify(", ".Delete(", ".Insert(", "SaveProfileDatabase", "CommitChanges" })
            Assert.DoesNotContain(write, ProfileVerifyScript.Code);
        Assert.Contains("GetParametricProfileItems", ProfileVerifyScript.Code);   // enumerates: SelectByPrefix cannot see a broken entry
        Assert.Contains("SelectByPrefix", ProfileVerifyScript.Code);
        Assert.Contains("near_miss_entries", ProfileVerifyScript.Code);
    }
}
