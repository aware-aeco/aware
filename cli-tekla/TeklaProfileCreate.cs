using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Security.Cryptography;
using System.Text;
using System.Text.Json;
using System.Text.Json.Nodes;
using System.Text.RegularExpressions;

namespace AwareTekla;

/// <summary>
/// `profile-create` (aware#858, FloLess #3382 slice A2), ALIAS route only. It adds one row to the environment's
/// `profitab.inp` (the folder named by XS_PROFDB) that registers a new prefix as an alias of a section Tekla already
/// loads, e.g. `AWTRI ! USER ! 0 ! ! 2 ! 2 !TEKLAINC.TRIANGLE !h*b`. Proven live on Tekla 2026.0: the alias registers,
/// measures and honours sizes. It never writes a `.clb` or any other Tekla-owned library (owner ruling 2026-11-19), and
/// a shape Tekla lacks is the Item route (aware#859), not this verb.
///
/// The work is plain file I/O, so it is made here and needs no running Tekla. Tekla reads `profitab.inp` only at
/// start, so a write is never live: the result says "takes effect after Tekla restarts" and `profile-verify` reads the
/// outcome back once Tekla has been restarted. The file is shared by every model on that environment (and possibly the
/// firm), so every write keeps a SHA-256 verified backup, a receipt and an exact undo.
/// </summary>
internal static class TeklaProfileCreate
{
    internal const int MaxPrefixLength = 8;
    internal const string ProfileTableName = "profitab.inp";

    static readonly Encoding Latin1 = Encoding.GetEncoding("ISO-8859-1");
    static readonly Regex PrefixShape = new(@"\A[A-Z][A-Z0-9_-]{0,7}\z", RegexOptions.CultureInvariant);
    static readonly Regex LibrarySection = new(@"\A[A-Za-z0-9_+\-]+\.[A-Za-z0-9_+\-]+\z", RegexOptions.CultureInvariant);

    internal static readonly HashSet<string> OkStatuses = new(StringComparer.Ordinal) { "planned", "created", "reused-identical", "undone" };

    /// <summary>One data row of profitab.inp, split on `!` (comment lines and short lines are not rows).</summary>
    internal sealed class Row
    {
        public Row(string line, string prefix, string type, string target, string parameters)
        { Line = line; Prefix = prefix; Type = type; Target = target; Parameters = parameters; }
        public string Line { get; }
        public string Prefix { get; }
        public string Type { get; }
        public string Target { get; }
        public string Parameters { get; }
    }

    // ── pure helpers ─────────────────────────────────────────────────────────

    /// <summary>Why this prefix cannot be written, or null. Upper-case letters, digits, `-`, `_`; 1 to 8 characters; starts with a letter; does not end in a digit.</summary>
    internal static string? ValidatePrefix(string? prefix)
    {
        if (prefix is null || prefix.Length == 0) return "prefix is required";
        if (prefix.Length > MaxPrefixLength)
            return $"prefix '{prefix}' is {prefix.Length} characters; at most {MaxPrefixLength} work (a longer prefix registers a broken entry Tekla cannot select)";
        if (!PrefixShape.IsMatch(prefix))
            return $"prefix '{prefix}' must be upper-case letters, digits, '-' or '_', start with a letter, and be at most {MaxPrefixLength} characters";
        if (char.IsDigit(prefix[prefix.Length - 1]))
            return $"prefix '{prefix}' ends in a digit, which Tekla would read as the first size of a name like '{prefix}400*200'";
        return null;
    }

    internal static string? ValidateAliasOf(string? aliasOf)
        => string.IsNullOrEmpty(aliasOf) ? "alias_of is required (LIBRARY.SECTION, e.g. TEKLAINC.TRIANGLE)"
         : LibrarySection.IsMatch(aliasOf) ? null
         : $"alias_of '{aliasOf}' must look like LIBRARY.SECTION";

    internal static string DetectTerminator(string text)
    {
        int crlf = 0, bare = 0;
        for (int i = 0; i < text.Length; i++)
            if (text[i] == '\n') { if (i > 0 && text[i - 1] == '\r') crlf++; else bare++; }
        return crlf == 0 && bare > 0 ? "\n" : "\r\n";
    }

    internal static List<Row> ParseRows(string text)
    {
        var rows = new List<Row>();
        foreach (var raw in text.Split('\n'))
        {
            var line = raw.TrimEnd('\r');
            var trimmed = line.TrimStart();
            if (trimmed.Length == 0 || trimmed.StartsWith("/*", StringComparison.Ordinal)) continue;
            var f = line.Split('!').Select(s => s.Trim()).ToArray();
            if (f.Length < 8 || f[0].Length == 0) continue;
            rows.Add(new Row(line, f[0], f[1], f[6], f[7]));
        }
        return rows;
    }

    /// <summary>The new row: the template row's text verbatim (type, assortment, limits, target, parameter string) behind the new prefix.</summary>
    internal static string FormatRow(string prefix, Row template)
    {
        int bang = template.Line.IndexOf('!');
        return prefix.PadRight(9) + template.Line.Substring(bang);
    }

    /// <summary>The bytes after adding `row`: every original byte kept, the row appended in the file's own line ending.</summary>
    internal static byte[] AppendRow(byte[] original, string row)
    {
        var text = Latin1.GetString(original);
        var term = DetectTerminator(text);
        var sb = new StringBuilder();
        if (text.Length > 0 && !text.EndsWith("\n", StringComparison.Ordinal)) sb.Append(term);
        sb.Append(row).Append(term);
        var tail = Latin1.GetBytes(sb.ToString());
        var result = new byte[original.Length + tail.Length];
        Buffer.BlockCopy(original, 0, result, 0, original.Length);
        Buffer.BlockCopy(tail, 0, result, original.Length, tail.Length);
        return result;
    }

    internal static string Sha256(byte[] bytes)
    {
        using var sha = SHA256.Create();
        return string.Concat(sha.ComputeHash(bytes).Select(b => b.ToString("x2")));
    }

    static bool SameDefinition(Row a, string target, string parameters)
        => string.Equals(a.Type, "USER", StringComparison.OrdinalIgnoreCase)
        && string.Equals(a.Target, target, StringComparison.OrdinalIgnoreCase)
        && string.Equals(a.Parameters, parameters, StringComparison.Ordinal);

    // ── the verb ─────────────────────────────────────────────────────────────

    /// <param name="input">The request JSON.</param>
    /// <param name="teklaRunning">Whether a Tekla is running now (only changes the wording of the result).</param>
    /// <param name="fault">Test seam: called with a stage name; a throw there simulates a failure at that stage.</param>
    internal static (JsonObject Result, int Exit) Run(JsonNode? input, Func<bool>? teklaRunning = null, Action<string>? fault = null)
    {
        if (input is not JsonObject)
            return Refuse("invalid-request", "plan", "the request must be a JSON object");
        var mode = Str(input, "mode") ?? "plan";
        try
        {
            return mode switch
            {
                "plan" => Create(input, teklaRunning, fault, write: false),
                "create" => Locked(LockKeyForCreate(input), () => Create(input, teklaRunning, fault, write: true)),
                "undo" => Locked(LockKeyForUndo(input), () => Undo(input, teklaRunning)),
                _ => Refuse("invalid-request", mode, $"mode '{mode}' must be plan, create or undo"),
            };
        }
        catch (Exception e)
        {
            return (new JsonObject { ["status"] = "failed", ["mode"] = mode, ["message"] = e.Message }, 1);
        }
    }

    // One writer at a time per catalogue file, across processes: the read, the checks, the replace, the read-back and
    // any rollback all sit inside the lock, so two creates (or a create and an undo) cannot each start from the same
    // bytes and silently drop the other's row.
    static string? LockKeyForCreate(JsonNode? input)
        => Str(input, "profile_database") is { Length: > 0 } f ? SafeFull(Path.Combine(f, ProfileTableName)) : null;

    static string? LockKeyForUndo(JsonNode? input)
    {
        try
        {
            if (Str(input, "receipt") is { Length: > 0 } r && File.Exists(r)
                && JsonNode.Parse(File.ReadAllText(r)) is JsonObject o && Str(o, "file") is { Length: > 0 } f) return SafeFull(f);
        }
        catch { /* an unreadable receipt is reported by Undo itself */ }
        return null;
    }

    static string? SafeFull(string path) { try { return Path.GetFullPath(path); } catch { return null; } }

    static (JsonObject, int) Locked(string? file, Func<(JsonObject, int)> body)
    {
        if (file is null || !Directory.Exists(Path.GetDirectoryName(file))) return body();   // nothing to lock; the body names the missing folder
        var path = file + LockSuffix;
        FileStream? handle = null;
        var deadline = DateTime.UtcNow + LockWait;
        string? error = null;
        while (handle is null)
        {
            try { handle = new FileStream(path, FileMode.OpenOrCreate, FileAccess.ReadWrite, FileShare.None, 1, FileOptions.DeleteOnClose); }
            // A holder that is closing leaves the file delete-pending for a moment: Windows answers that with a sharing
            // violation or an access-denied, so both are retried until the deadline.
            catch (Exception e) when ((e is IOException || e is UnauthorizedAccessException) && DateTime.UtcNow < deadline)
            { error = e.Message; System.Threading.Thread.Sleep(25 + (Environment.TickCount & 31)); }
            catch (Exception e) { error = e.Message; break; }
        }
        if (handle is null)
            return (new JsonObject
            {
                ["status"] = "failed",
                ["message"] = $"Could not take the write lock {path} ({error}). Another profile-create may be changing this file, or the folder is read-only. Nothing was written.",
                ["written"] = false,
            }, 1);
        using (handle) { return body(); }
    }

    // The lock is a file next to the catalogue, held open with FileShare.None and deleted when closed. Unlike a
    // machine-local mutex it also serialises two workstations writing the same firm-shared folder (SMB honours the
    // share mode), and a crashed writer cannot leave it behind: the handle closes with the process.
    internal const string LockSuffix = ".aware-lock";
    internal static TimeSpan LockWait = TimeSpan.FromSeconds(30);

    static string? Str(JsonNode? n, string key) => n?[key] is JsonValue v && v.TryGetValue<string>(out var s) ? s : null;

    static (JsonObject, int) Refuse(string status, string mode, string message, JsonObject? extra = null)
    {
        var o = new JsonObject { ["status"] = status, ["mode"] = mode, ["message"] = message };
        if (extra != null) foreach (var kv in extra) o[kv.Key] = kv.Value?.DeepClone();
        return (o, 2);
    }

    const string SharedNote = "This file is the environment's profile database: every model on this environment (and the firm, if the folder is shared) sees the new prefix after Tekla restarts.";

    static (JsonObject, int) Create(JsonNode? input, Func<bool>? teklaRunning, Action<string>? fault, bool write)
    {
        string mode = write ? "create" : "plan";
        var folder = Str(input, "profile_database");
        var prefix = Str(input, "prefix");
        var aliasOf = Str(input, "alias_of");
        var wantedParams = Str(input, "parameter_string");

        if (string.IsNullOrWhiteSpace(folder)) return Refuse("invalid-request", mode, "profile_database is required: the XS_PROFDB folder `section-measure` reports under environment.profile_database");
        if (ValidatePrefix(prefix) is { } pe) return Refuse("invalid-request", mode, pe);
        if (ValidateAliasOf(aliasOf) is { } ae) return Refuse("invalid-request", mode, ae);
        folder = Path.GetFullPath(folder!);
        var file = Path.Combine(folder, ProfileTableName);
        if (!File.Exists(file)) return Refuse("invalid-request", mode, $"{file} does not exist; point profile_database at the environment's XS_PROFDB folder");

        var original = File.ReadAllBytes(file);
        var before = Sha256(original);
        var rows = ParseRows(Latin1.GetString(original));

        var template = rows.FirstOrDefault(r =>
            string.Equals(r.Type, "USER", StringComparison.OrdinalIgnoreCase)
            && string.Equals(r.Target, aliasOf, StringComparison.OrdinalIgnoreCase)
            && (wantedParams == null || string.Equals(r.Parameters, wantedParams, StringComparison.Ordinal)));
        if (template == null)
            return Refuse("target-unknown", mode,
                $"{ProfileTableName} has no USER row for {aliasOf}, so Tekla in this environment is not known to load that section. Alias a section that an existing row already names.");

        var identity = new JsonObject
        {
            ["profile_database"] = folder,
            ["file"] = file,
            ["prefix"] = prefix,
            ["alias_of"] = template.Target,
            ["parameter_string"] = template.Parameters,
            ["template_prefix"] = template.Prefix,
            ["shared_note"] = SharedNote,
        };

        var clash = rows.Where(r => string.Equals(r.Prefix, prefix, StringComparison.OrdinalIgnoreCase)).ToList();
        if (clash.Count > 0)
        {
            if (clash.Any(r => SameDefinition(r, template.Target, template.Parameters)))
            {
                return (new JsonObject
                {
                    ["status"] = "reused-identical",
                    ["mode"] = mode,
                    ["message"] = $"{prefix} already aliases {template.Target} with the same sizes; nothing written.",
                    ["written"] = false,
                    ["profile"] = identity,
                    ["verify"] = VerifyHint(prefix!, template, input?["samples"]),
                }, 0);
            }
            var c = clash[0];
            return Refuse("conflict", mode,
                $"{prefix} already exists as '{c.Target}' ({c.Type}, parameters '{c.Parameters}'), not as {template.Target}. Nothing written; pick another prefix.",
                new JsonObject { ["profile"] = identity, ["existing_row"] = c.Line });
        }

        var row = FormatRow(prefix!, template);
        identity["row"] = row;
        bool running = teklaRunning?.Invoke() ?? false;
        var after = AppendRow(original, row);
        var afterSha = Sha256(after);

        if (!write)
        {
            return (new JsonObject
            {
                ["status"] = "planned",
                ["mode"] = mode,
                ["message"] = $"Would add one row to {file}. Nothing written.",
                ["written"] = false,
                ["tekla_running"] = running,
                ["takes_effect"] = "after Tekla restarts",
                ["profile"] = identity,
                ["file_change"] = new JsonObject { ["path"] = file, ["sha256_before"] = before, ["sha256_after_expected"] = afterSha, ["bytes_before"] = original.Length, ["bytes_after"] = after.Length },
            }, 0);
        }

        // Write: backup (verified) -> receipt -> temp file -> atomic replace -> read-back; restore on any failure after the replace.
        var runId = DateTime.UtcNow.ToString("yyyyMMddTHHmmssfff") + "-" + Guid.NewGuid().ToString("N").Substring(0, 6);
        var root = Str(input, "backup_dir") is { Length: > 0 } bd ? Path.GetFullPath(bd) : DefaultBackupRoot(folder);
        var runDir = Path.Combine(root, runId);
        var backup = Path.Combine(runDir, ProfileTableName + ".bak");
        var receiptPath = Path.Combine(runDir, "receipt.json");

        try
        {
            Directory.CreateDirectory(runDir);
            File.WriteAllBytes(backup, original);
            fault?.Invoke("after-backup");
            if (Sha256(File.ReadAllBytes(backup)) != before) throw new IOException("the backup copy does not match the original");
        }
        catch (Exception e)
        {
            return (new JsonObject { ["status"] = "backup-failed", ["mode"] = mode, ["message"] = "No backup could be verified, so nothing was written: " + e.Message, ["written"] = false, ["profile"] = identity }, 1);
        }

        var receipt = new JsonObject
        {
            ["run_id"] = runId,
            ["written_at"] = DateTime.UtcNow.ToString("o"),
            ["state"] = "pending",
            ["file"] = file,
            ["prefix"] = prefix,
            ["alias_of"] = template.Target,
            ["row"] = row,
            ["backup"] = backup,
            ["sha256_before"] = before,
            ["sha256_after"] = afterSha,
        };
        var temp = file + ".aware-" + runId + ".tmp";
        bool replaced = false;
        try
        {
            WriteReceipt(receiptPath, receipt);
            fault?.Invoke("after-receipt");
            if (Sha256(File.ReadAllBytes(file)) != before) throw new IOException(ProfileTableName + " changed while the backup was made; nothing was written");
            File.WriteAllBytes(temp, after);
            fault?.Invoke("after-temp");
            File.Replace(temp, file, null);
            replaced = true;
            fault?.Invoke("after-replace");
            if (Sha256(File.ReadAllBytes(file)) != afterSha) throw new IOException("read-back of the written file does not match what was intended");
        }
        catch (Exception e)
        {
            TryDelete(temp);
            string restored = "not-needed";
            // File.Replace can fail after removing the target (the original gone, the replacement still under its temp
            // name), so what happened is decided by the bytes on disk, not by whether the call returned.
            if (!replaced && (!File.Exists(file) || SameBytes(file, afterSha))) replaced = true;   // a file that is someone else's content is left alone
            if (replaced)
            {
                try { ReplaceWithBytes(file, original, runId); restored = Sha256(File.ReadAllBytes(file)) == before ? "restored" : "restore-mismatch"; }
                catch (Exception re) { restored = "restore-failed: " + re.Message; }
            }
            receipt["state"] = replaced ? "failed-" + restored : "failed-not-written";
            var staleReceipt = TryWriteReceipt(receiptPath, receipt);
            return (new JsonObject
            {
                ["status"] = replaced ? "partial" : "failed",
                ["mode"] = mode,
                ["message"] = e.Message + (replaced ? $" ({ProfileTableName}: {restored})" : $" ({ProfileTableName} untouched)"),
                ["written"] = replaced && restored != "restored",
                ["restore"] = restored,
                ["profile"] = identity,
                ["receipt"] = receiptPath,
                ["receipt_warning"] = staleReceipt is null ? null : "The receipt could not be updated and may still say 'pending': " + staleReceipt,
                ["backup"] = backup,
                ["undo"] = UndoHint(receiptPath, backup, file),
            }, 1);
        }

        receipt["state"] = "written";
        var receiptWarning = TryWriteReceipt(receiptPath, receipt);
        return (new JsonObject
        {
            ["status"] = "created",
            ["mode"] = mode,
            ["message"] = $"Added {prefix} to {file}. It takes effect after Tekla restarts" + (running ? " (a Tekla is running now and has NOT seen it)." : "."),
            ["written"] = true,
            ["tekla_running"] = running,
            ["takes_effect"] = "after Tekla restarts",
            ["profile"] = identity,
            ["file_change"] = new JsonObject { ["path"] = file, ["sha256_before"] = before, ["sha256_after"] = afterSha, ["backup"] = backup, ["bytes_before"] = original.Length, ["bytes_after"] = after.Length },
            ["receipt"] = receiptPath,
            ["receipt_warning"] = receiptWarning is null ? null : "The receipt could not be updated and may still say 'pending' (the backup and hashes are right): " + receiptWarning,
            ["undo"] = UndoHint(receiptPath, backup, file),
            ["verify"] = VerifyHint(prefix!, template, input?["samples"]),
        }, 0);
    }

    static JsonObject UndoHint(string receipt, string backup, string file) => new()
    {
        ["request"] = new JsonObject { ["mode"] = "undo", ["receipt"] = receipt },
        ["instruction"] = $"Run profile-create with mode undo and this receipt, or copy {backup} over {file}. Either puts back the exact bytes from before the write; Tekla must be restarted for the removal to show.",
    };

    static JsonObject VerifyHint(string prefix, Row template, JsonNode? samples)
    {
        var request = new JsonObject { ["prefix"] = prefix, ["reference"] = template.Prefix };
        bool complete = samples is JsonArray a && a.Count > 0;
        if (complete) request["samples"] = samples!.DeepClone();
        return new JsonObject
        {
            ["verb"] = "profile-verify",
            ["after"] = "Restart Tekla on this environment, then run profile-verify with this request."
                + (complete ? "" : " It is missing `samples`: add 1 to 8 size strings, e.g. [\"400*200\"] (pass `samples` to profile-create to have them filled in)."),
            ["request"] = request,
            ["request_complete"] = complete,
        };
    }

    static string DefaultBackupRoot(string folder)
    {
        var key = Sha256(Encoding.UTF8.GetBytes(Path.GetFullPath(folder).ToLowerInvariant())).Substring(0, 12);
        return Path.Combine(Environment.GetFolderPath(Environment.SpecialFolder.LocalApplicationData), "aware", "tekla-profile-create", key);
    }

    static void WriteReceipt(string path, JsonObject receipt)
        => File.WriteAllText(path, receipt.ToJsonString(new JsonSerializerOptions { WriteIndented = true }), new UTF8Encoding(false));

    /// <summary>Writes the receipt; returns null, or why it could not (the result then says the receipt is stale).</summary>
    static string? TryWriteReceipt(string path, JsonObject receipt) { try { WriteReceipt(path, receipt); return null; } catch (Exception e) { return e.Message; } }

    static bool SameBytes(string file, string sha)
    {
        try { return Sha256(File.ReadAllBytes(file)) == sha; }
        catch { return false; }
    }

    static bool DiskDiffers(string file, string sha)
    {
        try { return !File.Exists(file) || Sha256(File.ReadAllBytes(file)) != sha; }
        catch { return true; }
    }

    static void TryDelete(string path) { try { if (File.Exists(path)) File.Delete(path); } catch { } }

    static void ReplaceWithBytes(string file, byte[] bytes, string runId)
    {
        var temp = file + ".aware-" + runId + ".restore";
        try { File.WriteAllBytes(temp, bytes); if (File.Exists(file)) File.Replace(temp, file, null); else File.Move(temp, file); }
        finally { TryDelete(temp); }
    }

    // ── undo ─────────────────────────────────────────────────────────────────

    static (JsonObject, int) Undo(JsonNode? input, Func<bool>? teklaRunning)
    {
        var receiptPath = Str(input, "receipt") is { Length: > 0 } rp ? Path.GetFullPath(rp) : null;
        if (receiptPath is null || !File.Exists(receiptPath))
            return Refuse("invalid-request", "undo", "receipt is required: the receipt path a create returned");
        bool force = input?["force"] is JsonValue fv && fv.TryGetValue<bool>(out var fb) && fb;
        var receipt = JsonNode.Parse(File.ReadAllText(receiptPath)) as JsonObject
            ?? throw new InvalidDataException("receipt is not a JSON object");
        var file = Str(receipt, "file") ?? "";
        var backup = Str(receipt, "backup") ?? "";
        var sha0 = Str(receipt, "sha256_before") ?? "";
        var sha1 = Str(receipt, "sha256_after") ?? "";
        if (string.IsNullOrEmpty(file) || string.IsNullOrEmpty(backup) || string.IsNullOrEmpty(sha0) || string.IsNullOrEmpty(sha1))
            return Refuse("invalid-request", "undo", "the receipt is missing file, backup, sha256_before or sha256_after; it is not one profile-create wrote");
        if (Str(receipt, "state") == "undone")
            return (new JsonObject { ["status"] = "undone", ["mode"] = "undo", ["message"] = "Already undone; nothing to do.", ["written"] = false, ["file"] = file }, 0);
        if (!File.Exists(backup)) return Refuse("backup-failed", "undo", $"backup {backup} is missing; nothing changed");
        var saved = File.ReadAllBytes(backup);
        if (Sha256(saved) != sha0) return Refuse("backup-failed", "undo", "the backup no longer matches its recorded SHA-256; nothing changed");
        if (!File.Exists(file)) return Refuse("invalid-request", "undo", $"{file} is missing");
        var current = Sha256(File.ReadAllBytes(file));
        if (current == sha0)
            return (new JsonObject { ["status"] = "undone", ["mode"] = "undo", ["message"] = "The file already holds the pre-write bytes; nothing to do.", ["written"] = false, ["file"] = file }, 0);
        if (current != sha1 && !force)
            return Refuse("changed-since", "undo",
                $"{ProfileTableName} was changed after this write (SHA-256 {current}, expected {sha1}). Restoring would also drop that later change. Send force:true to restore anyway.",
                new JsonObject { ["file"] = file });
        try { ReplaceWithBytes(file, saved, Str(receipt, "run_id") ?? "undo"); }
        catch (Exception e)
        {
            return (new JsonObject { ["status"] = "failed", ["mode"] = "undo", ["message"] = "The restore failed: " + e.Message + (DiskDiffers(file, sha0) ? $" {ProfileTableName} may not hold the pre-write bytes; copy {backup} over {file}." : ""), ["written"] = DiskDiffers(file, current), ["file"] = file, ["backup"] = backup }, 1);
        }
        if (DiskDiffers(file, sha0))
            return (new JsonObject { ["status"] = "failed", ["mode"] = "undo", ["message"] = $"The restored file does not match the backup; copy {backup} over {file}.", ["written"] = true, ["file"] = file, ["backup"] = backup }, 1);
        receipt["state"] = "undone";
        receipt["undone_at"] = DateTime.UtcNow.ToString("o");
        var undoReceiptWarning = TryWriteReceipt(receiptPath, receipt);
        bool running = teklaRunning?.Invoke() ?? false;
        return (new JsonObject
        {
            ["status"] = "undone",
            ["mode"] = "undo",
            ["message"] = $"Restored {file} to its exact pre-write bytes (SHA-256 {sha0})." + (current != sha1 ? " Rows added after this write went with it." : "") + (running ? " A running Tekla still has the alias until it restarts." : " Takes effect after Tekla restarts."),
            ["written"] = true,
            ["tekla_running"] = running,
            ["file"] = file,
            ["sha256"] = sha0,
            ["receipt_warning"] = undoReceiptWarning is null ? null : "The receipt could not be marked undone: " + undoReceiptWarning,
        }, 0);
    }
}
