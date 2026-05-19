// ArchiCAD extractor — produces a host-coverage IR by combining:
//   1. Tapir's `command_definitions.js` + `common_schema_definitions.js`
//      (from https://raw.githubusercontent.com/ENZYME-APD/tapir-archicad-automation/main/docs/archicad-addon/)
//   2. Graphisoft's official JSON Interface documentation
//      (from https://archicadapi.graphisoft.com/JSONInterfaceDocumentation/, currently v29.0.0)
//
// IR shape:
//   ArchiCAD is a JSON-RPC API, not a class library. We map:
//     - Tapir command CATEGORIES (e.g. "Application Commands")  →  TypeInfo kind="static-class", namespace "Tapir.AdditionalCommands"
//     - Tapir SHARED schema types (e.g. "ElementId")             →  TypeInfo kind="class"/"enum", namespace "Tapir.Schema"
//     - Graphisoft command GROUPS (e.g. "Attribute Commands")    →  TypeInfo kind="static-class", namespace "Archicad.OfficialCommands"
//     - Each command                                              →  MethodInfo inside its parent TypeInfo
//
// Version handling (CRITICAL):
//   Tapir publishes ONE catalog that targets ArchiCAD 25-29 (the same JSON commands
//   work in every version; the binary builds differ by AC SDK target). So both v28
//   and v29 IRs include the FULL Tapir command set verbatim.
//
//   Graphisoft only publishes ONE JSON Interface site (the latest — currently v29).
//   For AC28 there is NO archived public docs site. We document this in EXTRACTION-NOTES.md
//   and include the Graphisoft v29 surface in BOTH IRs with a note that v28 is a subset
//   (commands new in 29 are still listed; agents using AC28 should consult the per-command
//   compatibility section on the Graphisoft live site for any 28 vs 29 differences).
//
// Pipeline:
//   Phase 1 — fetch Tapir command_definitions.js + common_schema_definitions.js (2 raw.github fetches)
//   Phase 2 — fetch Graphisoft menutree.json (1 fetch)
//   Phase 3 — fan-out fetch every command JSON from Graphisoft (73 fetches, 4-way concurrent)
//   Phase 4 — assemble IR (in-process — all data already in memory)
//   Phase 5 — emit
//
// No snapshotting needed — the entire payload is < 500KB across ~80 HTTP fetches. Even
// a single-threaded run completes in under a minute. The expensive part (HTML page-walk
// for Tekla / Rhino) doesn't apply here.
//
// AOT: zero reflection — JsonDocument is used for parsing, and the final IR is serialised
// via source-gen IrJsonContext. No HttpScraper dispose-after-use issues since the fetches
// are short-lived.

using AwareSidecar.Ingest.Extractors.Common;

namespace AwareSidecar.Ingest.Extractors.ArchiCAD;

public static class Extractor
{
    /// <summary>
    /// Tapir docs source URLs (raw GitHub content, branch `main`). The same files apply to
    /// every ArchiCAD version Tapir supports (25-29) — Tapir builds per-version binaries
    /// against AC's SDK but the JSON command surface is identical across versions.
    /// </summary>
    const string TapirCommandsRawUrl = "https://raw.githubusercontent.com/ENZYME-APD/tapir-archicad-automation/main/docs/archicad-addon/command_definitions.js";
    const string TapirSchemaRawUrl   = "https://raw.githubusercontent.com/ENZYME-APD/tapir-archicad-automation/main/docs/archicad-addon/common_schema_definitions.js";
    const string TapirDocsPageUrl    = "https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/";

    /// <summary>
    /// Graphisoft official docs root. menutree.json lives at content/menutree.json; each
    /// command's schema JSON lives at content/&lt;FileName&gt;.json.
    /// </summary>
    const string GraphisoftMenuUrl = GraphisoftMenuParser.ContentBaseUrl + "menutree.json";

    static readonly HashSet<string> KnownVersions = new(StringComparer.Ordinal) { "28.0", "29.0" };

    const int CommandFetchConcurrency = 4;

    public static Task<CoverageIR> Extract(string version) =>
        Extract(version, outPath: null);

    public static async Task<CoverageIR> Extract(string version, string? outPath)
    {
        if (!KnownVersions.Contains(version))
            throw new ArgumentException(
                $"ArchiCAD extractor: unsupported version '{version}'. Known: {string.Join(", ", KnownVersions)}");

        var errorsLog = new ErrorsLog(DeriveErrorsLogPath(outPath));
        var builder = new IRBuilder("archicad", version, "hybrid");
        builder.AddSourceUrl(TapirCommandsRawUrl);
        builder.AddSourceUrl(TapirSchemaRawUrl);
        builder.AddSourceUrl(TapirDocsPageUrl);
        builder.AddSourceUrl(GraphisoftMenuUrl);

        await using var scraper = await HttpScraper.Launch();
        Console.Error.WriteLine($"[archicad-extract] starting hybrid extraction for v{version}");

        // ── Phase 1: Tapir ─────────────────────────────────────────────────
        Console.Error.WriteLine("[archicad-extract] phase 1: fetch Tapir command_definitions.js + common_schema_definitions.js");
        string tapirCommandsJs;
        string tapirSchemaJs;
        try
        {
            tapirCommandsJs = await scraper.FetchHtml(TapirCommandsRawUrl);
            builder.IncrementPageCount();
        }
        catch (Exception ex)
        {
            errorsLog.Append("tapir-commands-fetch", TapirCommandsRawUrl, ex.Message);
            throw new InvalidOperationException(
                $"ArchiCAD extractor: cannot fetch Tapir commands at {TapirCommandsRawUrl}: {ex.Message}", ex);
        }
        try
        {
            tapirSchemaJs = await scraper.FetchHtml(TapirSchemaRawUrl);
            builder.IncrementPageCount();
        }
        catch (Exception ex)
        {
            errorsLog.Append("tapir-schema-fetch", TapirSchemaRawUrl, ex.Message);
            throw new InvalidOperationException(
                $"ArchiCAD extractor: cannot fetch Tapir schema at {TapirSchemaRawUrl}: {ex.Message}", ex);
        }

        var tapirCategories = TapirCommandParser.Parse(tapirCommandsJs);
        var tapirSchema = TapirSchemaParser.Parse(tapirSchemaJs);
        Console.Error.WriteLine(
            $"[archicad-extract]   Tapir parsed: {tapirCategories.Count} categories, " +
            $"{tapirCategories.Sum(c => c.Commands.Count)} commands, {tapirSchema.Count} shared types");

        // Emit Tapir categories as TypeInfos
        foreach (var cat in tapirCategories)
        {
            var typeInfo = PageParser.BuildTapirCategoryType(cat, TapirDocsPageUrl);
            builder.AddType(typeInfo);
        }
        // Emit Tapir shared schema types as TypeInfos
        foreach (var (name, schemaEl) in tapirSchema.OrderBy(kv => kv.Key, StringComparer.Ordinal))
        {
            var typeInfo = PageParser.BuildSharedSchemaType(name, schemaEl, TapirDocsPageUrl);
            builder.AddType(typeInfo);
        }

        // ── Phase 2: Graphisoft menutree ───────────────────────────────────
        Console.Error.WriteLine("[archicad-extract] phase 2: fetch Graphisoft menutree.json");
        string menuJson;
        try
        {
            menuJson = await scraper.FetchHtml(GraphisoftMenuUrl);
            builder.IncrementPageCount();
        }
        catch (Exception ex)
        {
            errorsLog.Append("graphisoft-menu-fetch", GraphisoftMenuUrl, ex.Message);
            throw new InvalidOperationException(
                $"ArchiCAD extractor: cannot fetch Graphisoft menutree at {GraphisoftMenuUrl}: {ex.Message}", ex);
        }
        var graphisoftCommands = GraphisoftMenuParser.ParseMenuTree(menuJson);
        Console.Error.WriteLine($"[archicad-extract]   Graphisoft menu: {graphisoftCommands.Count} commands across "
            + $"{graphisoftCommands.Select(c => c.GroupName).Distinct().Count()} groups");

        // ── Phase 3: Graphisoft per-command fetch ──────────────────────────
        Console.Error.WriteLine($"[archicad-extract] phase 3: fetch {graphisoftCommands.Count} Graphisoft command schemas (concurrency {CommandFetchConcurrency})");
        var fetched = new Dictionary<string, CommandDefinition>(StringComparer.Ordinal);
        var fetchedLock = new object();
        var fetchErrors = new List<(string url, string err)>();
        var fetchOpts = new ParallelOptions { MaxDegreeOfParallelism = CommandFetchConcurrency };
        await Parallel.ForEachAsync(graphisoftCommands, fetchOpts, async (cmd, ct) =>
        {
            try
            {
                await using var s = await HttpScraper.Launch();
                var body = await s.FetchHtml(cmd.FetchUrl);
                var def = GraphisoftMenuParser.ParseCommandJson(cmd.CommandName, body);
                lock (fetchedLock)
                {
                    fetched[cmd.CommandName] = def;
                    if (fetched.Count % 25 == 0)
                        Console.Error.WriteLine($"[archicad-extract]   fetched {fetched.Count}/{graphisoftCommands.Count}");
                }
            }
            catch (Exception ex)
            {
                lock (fetchedLock) fetchErrors.Add((cmd.FetchUrl, ex.Message));
                errorsLog.Append("graphisoft-command-fetch", cmd.FetchUrl, ex.Message);
            }
        });

        if (fetchErrors.Count > 0)
        {
            Console.Error.WriteLine($"[archicad-extract]   {fetchErrors.Count} hard failures during command fetch; retrying once...");
            foreach (var (failUrl, _) in fetchErrors.ToList())
            {
                var cmdEntry = graphisoftCommands.First(c => c.FetchUrl == failUrl);
                try
                {
                    await Task.Delay(750);
                    await using var s = await HttpScraper.Launch();
                    var body = await s.FetchHtml(failUrl);
                    var def = GraphisoftMenuParser.ParseCommandJson(cmdEntry.CommandName, body);
                    fetched[cmdEntry.CommandName] = def;
                    Console.Error.WriteLine($"[archicad-extract]   RETRY ok: {failUrl}");
                }
                catch (Exception ex)
                {
                    Console.Error.WriteLine($"[archicad-extract]   RETRY still failing: {failUrl}: {ex.Message}");
                    errorsLog.Append("graphisoft-command-retry-final", failUrl, ex.Message);
                }
            }
        }

        builder.IncrementPageCount(); // (count graphisoft as 1 logical page after summing fetches)
        for (int i = 0; i < fetched.Count; i++) builder.IncrementPageCount();

        // ── Phase 4: emit Graphisoft categories ────────────────────────────
        var graphisoftByGroup = graphisoftCommands
            .GroupBy(c => c.GroupName, StringComparer.Ordinal)
            .OrderBy(g => g.Key, StringComparer.Ordinal);
        foreach (var group in graphisoftByGroup)
        {
            var methodList = new List<MethodInfo>();
            foreach (var cmd in group.OrderBy(c => c.CommandName, StringComparer.Ordinal))
            {
                if (!fetched.TryGetValue(cmd.CommandName, out var def))
                {
                    // The fetch failed entirely for this command — synthesize a stub that
                    // still appears in the IR (so the catalog is complete and reviewable)
                    // but with a clearly-marked empty signature.
                    def = new CommandDefinition(cmd.CommandName, "", "(failed to fetch from Graphisoft — see errors.log)", null, null);
                }
                var stub = new CommandDefinition(def.Name, def.Version, def.Description, def.InputScheme, def.OutputScheme);
                var method = PageParser.BuildTapirCommandMethod(stub, GraphisoftMenuParser.DocsBaseUrl);
                // Patch the doc_url to point at the Graphisoft anchor instead of the Tapir page
                method = method with { doc_url = cmd.DocPageUrl };
                methodList.Add(method);
            }

            var typeName = SanitiseGroupName(group.Key);
            var summary = $"Graphisoft official '{group.Key}' command group — {methodList.Count} JSON-RPC commands (Archicad {version}).";
            var typeInfo = new TypeInfo(
                name: typeName,
                @namespace: "Archicad.OfficialCommands",
                kind: "static-class",
                summary: summary,
                remarks: null,
                @base: null,
                interfaces: new List<string>(),
                doc_url: GraphisoftMenuParser.DocsBaseUrl,
                constructors: new List<MethodInfo>(),
                methods: methodList,
                properties: new List<PropertyInfo>(),
                events: new List<EventInfo>(),
                enum_values: new List<EnumValueInfo>(),
                delegate_signature: null);
            builder.AddType(typeInfo);
        }

        // ── Phase 5: finalize ──────────────────────────────────────────────
        Console.Error.WriteLine($"[archicad-extract] done: " +
            $"types={builder.Build().metadata.type_count} methods={builder.Build().metadata.method_count}");

        return builder.Build();
    }

    /// <summary>
    /// Convert "AddOn Commands" → "AddOnCommands", "Issue Management Commands" →
    /// "IssueManagementCommands". Matches the Tapir-side sanitiser logic.
    /// </summary>
    static string SanitiseGroupName(string name)
    {
        var sb = new System.Text.StringBuilder(name.Length);
        foreach (var c in name)
        {
            if (char.IsLetterOrDigit(c) || c == '_') sb.Append(c);
        }
        return sb.ToString();
    }

    // ── error log helpers (modelled on Rhino/Grasshopper) ────────────────

    static string? DeriveErrorsLogPath(string? outPath)
    {
        if (string.IsNullOrEmpty(outPath)) return null;
        var dir = Path.GetDirectoryName(outPath) ?? "";
        var fileName = Path.GetFileName(outPath);
        if (fileName.EndsWith(".ir.json", StringComparison.Ordinal))
            fileName = fileName[..^".ir.json".Length] + "-errors.log";
        else
            fileName += "-errors.log";
        return Path.Combine(dir, fileName);
    }

    sealed class ErrorsLog
    {
        readonly string? _path;
        readonly object _lock = new();
        public ErrorsLog(string? path) { _path = path; }

        public void Append(string phase, string url, string message)
        {
            if (string.IsNullOrEmpty(_path)) return;
            var safeMsg = (message ?? "").Replace('\t', ' ').Replace('\r', ' ').Replace('\n', ' ');
            var safeUrl = (url ?? "").Replace('\t', ' ').Replace('\r', ' ').Replace('\n', ' ');
            var line = $"{DateTime.UtcNow:O}\t{phase}\t{safeUrl}\t{safeMsg}{Environment.NewLine}";
            lock (_lock)
            {
                var dir = Path.GetDirectoryName(_path);
                if (!string.IsNullOrEmpty(dir)) Directory.CreateDirectory(dir);
                File.AppendAllText(_path, line);
            }
        }
    }
}
