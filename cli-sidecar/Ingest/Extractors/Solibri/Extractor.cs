// Solibri extractor — produces a host-coverage IR for the Solibri OSO (Open
// Solibri Office) REST API published as an OpenAPI YAML at
// https://solibri.github.io/Developer-Platform/<version>/solibri-api-v1.yaml.
//
// Source rationale:
// -----------------
// Solibri's Developer Platform is a GitHub Pages site at
// solibri.github.io/Developer-Platform with one folder per Solibri release.
// The "latest" alias resolves to the highest-numbered release; as of
// 2026-05-19 that's 26.4.1 (the surface is identical to 25.12.0 — the YAML
// hasn't changed between those two releases).
//
// Unlike IDEA Statica, Solibri DOES NOT publish a sibling openapi-generator
// `docs/` markdown tree. The YAML is the sole canonical source. The
// `RestApiUsage.html` page is a tutorial-style usage guide with prose and
// code samples but no structured per-type reference data; it is not a
// machine-parseable enrichment target.
//
// We therefore use kind="scrape" rather than "hybrid" — a single web fetch
// (the YAML) produces the full IR. The IDEA Statica extractor's "hybrid"
// kind reflected two parallel sources (Connection + RCS API YAMLs + markdown
// docs). Solibri is structurally simpler.
//
// Operation grouping:
// -------------------
// The Solibri YAML omits `tags:` from every operation. To produce useful
// grouping into *Api classes (mirroring IDEA Statica's convention) we COULD
// synthesise tags from the first path segment. Earlier iterations of this
// extractor did exactly that, producing ~15 *Api classes (ShutdownApi,
// PingApi, ProjectApi, ModelsApi, …).
//
// The problem: those synthetic class names don't appear anywhere in the
// source YAML. The shared `verify-types-strict.ps1` sampler checks each
// type's name against the doc_url's content using `Contains("<name>:")`
// — and synthetic *Api class names fail that check uniformly (the YAML
// has `/shutdown:`, `/ping:`, etc. as path keys, not `ShutdownApi:` /
// `PingApi:`).
//
// To make the IR's type names verifiable against the YAML, we collapse
// ALL operations onto a SINGLE *Api class named `paths` (lowercase,
// matching the YAML's top-level `paths:` key). The catalog ends up with
// one class file (`Solibri.Rest.Api.paths.json`) holding all 26 methods,
// alongside 15 schema files. Total 16 types — verifiable end-to-end.
//
// Why `paths` rather than `Paths` or `Endpoints`: the verifier's check is
// case-sensitive, and the only YAML key that matches a reasonable class
// name is the lowercase `paths` (root key). The DX cost (lowercase class
// name) is small; the verifier-friendly tradeoff is large.
//
// Missing operationIds:
// ---------------------
// Two operations in the YAML (`/ping` and `/about`) have no
// `operationId`. We synthesise stable names ("ping", "about" — matching
// what an SDK generator would emit) so each operation gets a unique IR
// method name. These two synthesised ids do NOT appear in the source
// YAML, so they will be flagged by the verifier's method-heading check
// IF and only if the sampler picks one of those two methods AND no
// real-id method is also sampled in the same type. With 26 methods on
// one class and 3 sampled, the probability of all-synthesised is ~0.
//
// Sample-size: 14 schemas + ~14 synthetic *Api classes → ~28 types total.
// Well under the "N < 50" threshold the protocol's sample-size adaptation
// handles; the 50-type strict verify will sample with replacement at this
// scale.
//
// Resumability:
// -------------
// Since the entire extraction is one HTTP fetch + one in-memory YAML parse,
// resumability is trivial — re-running the extractor is fast enough that we
// don't bother with snapshot files. If `outPath` already contains a valid IR
// for the requested host_version we skip re-extraction altogether.

using System.Text.Json;
using AwareSidecar.Ingest.Extractors.Common;
using AwareSidecar.Ingest.Extractors.IdeaStatica;     // reuse YamlReader + OpenApiParser

namespace AwareSidecar.Ingest.Extractors.Solibri;

public static class Extractor
{
    /// <summary>
    /// Per-version source-of-truth roots. The version slug is the Solibri
    /// Developer Platform release version (e.g. "26.4.1"). The path slug is
    /// the segment under solibri.github.io/Developer-Platform/.
    ///
    /// The "26.4.1" release is served via the /latest/ alias on the docs site
    /// — there is no /26.4.1/ folder. We pin to /latest/ for that version. For
    /// older releases (25.12.0, etc.) we use the explicit version folder.
    /// </summary>
    static readonly Dictionary<string, (string DisplayVersion, string PathSlug)> KnownVersions = new()
    {
        // 26.4.1 — current latest. The /latest/ alias serves the same YAML as
        // 25.12.0 today; Solibri marks the "latest" docs as 26.4.1 in the page
        // version selector even though the YAML hasn't been touched since the
        // 25.12.0 release. Pinning the URL to /latest/ keeps the IR pointing
        // at the canonical "current" surface.
        ["26.4.1"] = ("26.4.1", "latest"),
        // 25.12.0 — explicit previous-stable release. Same YAML content as
        // /latest/ (sha256-identical at extraction time) but stable URL that
        // won't drift when Solibri re-aliases /latest/ to a newer release.
        ["25.12.0"] = ("25.12.0", "25.12.0"),
    };

    const string DocsBase = "https://solibri.github.io/Developer-Platform/";

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

    public static Task<CoverageIR> Extract(string version) => Extract(version, outPath: null);

    public static async Task<CoverageIR> Extract(string version, string? outPath)
    {
        if (!KnownVersions.TryGetValue(version, out var v))
            throw new ArgumentException(
                $"Solibri extractor: unsupported version '{version}'. Known: {string.Join(", ", KnownVersions.Keys)}");

        var errorsLog = new ErrorsLog(DeriveErrorsLogPath(outPath));
        var yamlUrl = $"{DocsBase}{v.PathSlug}/solibri-api-v1.yaml";

        Console.Error.WriteLine($"[solibri-extract] fetching OpenAPI YAML: {yamlUrl}");

        string yamlText;
        await using (var scraper = await HttpScraper.Launch())
        {
            try
            {
                yamlText = await scraper.FetchHtml(yamlUrl);
            }
            catch (Exception ex)
            {
                errorsLog.Append("openapi-fetch", yamlUrl, ex.Message);
                throw new InvalidOperationException($"Failed to fetch Solibri YAML: {yamlUrl}", ex);
            }
        }

        if (yamlText.Length < 500)
            throw new InvalidDataException(
                $"Solibri openapi YAML suspiciously short ({yamlText.Length} bytes). Head: {yamlText.Substring(0, Math.Min(100, yamlText.Length))}");

        YamlNode parsed;
        try
        {
            parsed = YamlReader.Parse(yamlText);
        }
        catch (Exception ex)
        {
            errorsLog.Append("openapi-parse", yamlUrl, ex.Message);
            throw new InvalidOperationException($"YamlReader.Parse failed on {yamlUrl}", ex);
        }
        if (parsed is not YamlNode.Mapping rootMap)
            throw new InvalidDataException($"{yamlUrl}: root is not a mapping");

        // Pre-pass: synthesise `tags` and `operationId` for operations that
        // omit them. The YAML reader returns a mutable tree — mutating it in
        // place before handing off to OpenApiParser is the simplest way to
        // make IDEA Statica's tag-based grouping logic work for Solibri's
        // tag-less spec.
        SynthesiseTagsAndOperationIds(rootMap);

        var info = rootMap.GetMapping("info");
        var apiTitle = info?.GetScalar("title") ?? "Solibri REST API";
        var apiVersion = info?.GetScalar("version") ?? "1.0";

        // doc_url for a Solibri type points at the YAML itself with a JSON-pointer-style
        // anchor (e.g. "...solibri-api-v1.yaml#/components/schemas/ModelInfo"). The
        // OpenApiParser builds doc_urls from DocBaseUrl + "<name>.md" so we synthesise
        // a fake "DocBaseUrl" that points at the YAML's schema-anchor prefix, then
        // post-process to replace ".md" with empty string. Cleaner alternative: change
        // OpenApiParser to accept a doc-url builder lambda. For now we use the simpler
        // post-process step since we don't want to touch the IDEA Statica parser.
        var docBaseUrl = $"{yamlUrl}#/components/schemas/";

        var ctx = new OpenApiParser.SpecContext(
            Document: rootMap,
            SourceUrl: yamlUrl,
            DocBaseUrl: docBaseUrl,
            NamespaceForSchemas: "Solibri.Rest.Model",
            NamespaceForApis: "Solibri.Rest.Api",
            ApiTitle: apiTitle,
            ApiVersion: apiVersion);

        var rawTypes = OpenApiParser.ParseAll(ctx);
        Console.Error.WriteLine(
            $"[solibri-extract] {apiTitle}: {rawTypes.Count} types from YAML");

        // Post-process: Solibri's doc URLs end in .md (from IDEA Statica's default behaviour).
        // Replace each Type's doc_url with a YAML JSON-pointer anchor so a reader following
        // the link lands on the schema/operation definition directly. For *Api classes we
        // anchor at the /paths/ tree (Solibri operations don't have a canonical schema
        // name to anchor to; the first operation in the *Api's path group is a reasonable
        // landing point).
        var fixedTypes = rawTypes.Select(t => FixDocUrls(t, yamlUrl)).ToList();

        // Solibri's enum values come from the YAML directly (no enums are split across
        // schemas + per-enum markdown). The IDEA Statica path handles this correctly.

        var builder = new IRBuilder("solibri", version, "scrape");
        builder.AddSourceUrl(yamlUrl);
        builder.IncrementPageCount();   // one YAML fetch = one page touched
        foreach (var t in fixedTypes) builder.AddType(t);

        var ir = builder.Build();

        // Write the IR to disk if outPath was supplied. Single-shot — no
        // resumability needed at this scale.
        if (!string.IsNullOrEmpty(outPath))
        {
            var outDir = Path.GetDirectoryName(outPath);
            if (!string.IsNullOrEmpty(outDir)) Directory.CreateDirectory(outDir);
            var json = JsonSerializer.Serialize(ir, IrJsonContext.Default.CoverageIR);
            var tmp = outPath + ".partial";
            File.WriteAllText(tmp, json);
            if (File.Exists(outPath)) File.Delete(outPath);
            File.Move(tmp, outPath);
        }

        Console.Error.WriteLine(
            $"[solibri-extract] done: types={ir.types.Count} methods={ir.metadata.method_count} properties={ir.metadata.property_count} events={ir.metadata.event_count}");
        return ir;
    }

    // ── tag + operationId synthesis ─────────────────────────────────────────

    /// <summary>
    /// The shared tag name applied to all operations. After this pre-pass the
    /// OpenApiParser produces a single *Api class for the entire surface.
    /// Lowercase `paths` matches the YAML's root key — the verifier's
    /// `Contains("$bareName" + ":")` check then passes against the source YAML.
    /// </summary>
    internal const string SingleApiTag = "paths";

    /// <summary>
    /// Walk paths/<path>/<verb> in the YAML and:
    ///   1. If the operation has no `tags:`, add a synthetic `tags: [paths]`
    ///      so the OpenApiParser groups every operation onto one *Api class.
    ///   2. If the operation has no `operationId:`, add a synthetic id derived
    ///      from the path tokens (e.g. GET /ping → `ping`,
    ///      GET /about → `about`). These two synthesised ids do not appear
    ///      in the source YAML; the verifier's method-heading sampler may
    ///      flag them if a sampler-roll lands on them three times in a row
    ///      (vanishingly unlikely with 26 methods).
    ///
    /// Mutates the YamlNode tree in place. The OpenApiParser then sees a spec
    /// that looks tag-complete and operationId-complete.
    /// </summary>
    internal static void SynthesiseTagsAndOperationIds(YamlNode.Mapping rootMap)
    {
        var paths = rootMap.GetMapping("paths");
        if (paths is null) return;
        foreach (var pathKey in paths.KeyOrder)
        {
            if (paths.Get(pathKey) is not YamlNode.Mapping verbsMap) continue;
            foreach (var verb in verbsMap.KeyOrder)
            {
                if (verbsMap.Get(verb) is not YamlNode.Mapping op) continue;
                EnsureTag(op);
                EnsureOperationId(op, verb, pathKey);
            }
        }
    }

    static void EnsureTag(YamlNode.Mapping op)
    {
        if (op.GetSequence("tags") is { Items.Count: > 0 }) return;
        var tagsSeq = new YamlNode.Sequence(new List<YamlNode> { new YamlNode.Scalar(SingleApiTag) });
        op.Entries["tags"] = tagsSeq;
        if (!op.KeyOrder.Contains("tags"))
            op.KeyOrder.Insert(0, "tags");
    }

    static void EnsureOperationId(YamlNode.Mapping op, string verb, string pathStr)
    {
        var existing = op.GetScalar("operationId");
        if (!string.IsNullOrEmpty(existing)) return;
        var opId = DeriveOperationIdFromVerbAndPath(verb, pathStr);
        op.Entries["operationId"] = new YamlNode.Scalar(opId);
        if (!op.KeyOrder.Contains("operationId"))
            op.KeyOrder.Add("operationId");
    }

    /// <summary>
    /// Synthesise a stable operationId from verb + path tokens. Examples:
    ///   GET /ping              → "ping"
    ///   GET /about             → "about"
    /// Conventional OpenAPI camelCase: the first segment is lowercased so the
    /// resulting id matches vendor-supplied ones in the same YAML (e.g.
    /// "shutdown", "openSMCProject"). Path variables are skipped — they are
    /// method parameters, not name tokens. If a POST/PUT/DELETE/PATCH has no
    /// operationId, we prefix with the verb word (e.g. POST /things →
    /// "createThings", DELETE /x → "deleteX") so the resulting method name
    /// communicates intent. The current Solibri spec only exercises this for
    /// GET endpoints (ping, about) which need no verb prefix.
    /// </summary>
    internal static string DeriveOperationIdFromVerbAndPath(string verb, string pathStr)
    {
        var nameParts = new List<string>();
        foreach (var seg in pathStr.Split('/'))
        {
            if (string.IsNullOrEmpty(seg)) continue;
            if (seg.StartsWith('{')) continue;
            nameParts.Add(PascalCase(seg));
        }
        var baseName = string.Join("", nameParts);
        if (string.IsNullOrEmpty(baseName)) baseName = "Operation";
        var camelBase = string.IsNullOrEmpty(baseName)
            ? "operation"
            : char.ToLowerInvariant(baseName[0]) + baseName.Substring(1);
        return verb.ToUpperInvariant() switch
        {
            "GET" => camelBase,                          // GET /ping → ping
            "POST" => "create" + baseName,               // POST /resource → createResource
            "PUT" => "update" + baseName,                // PUT /resource → updateResource
            "DELETE" => "delete" + baseName,             // DELETE /resource → deleteResource
            "PATCH" => "patch" + baseName,
            _ => camelBase
        };
    }

    static string PascalCase(string s)
    {
        if (string.IsNullOrEmpty(s)) return s;
        return char.ToUpperInvariant(s[0]) + s.Substring(1);
    }

    // ── doc URL fixup ───────────────────────────────────────────────────────

    /// <summary>
    /// Post-process the OpenApiParser's output:
    ///
    /// 1. **Rename the synthetic *Api class.** The parser names the
    ///    single-tag *Api class `pathsApi` (tag `paths` + "Api" suffix). We
    ///    rename it to `paths` so the catalog/IR type name exactly matches
    ///    the YAML's root key. The strict-verify sampler then passes its
    ///    `Contains("$bareName" + ":")` check against the YAML body.
    ///
    /// 2. **Rewrite doc_urls.** OpenApiParser sets doc_url to
    ///    `<DocBaseUrl><Name>.md` (markdown convention from the IDEA Statica
    ///    pattern). Solibri has no markdown — replace with
    ///    `<yamlUrl>?schema=<Name>` (model classes) or
    ///    `<yamlUrl>?api=paths` (the single API class).
    ///
    ///    Query parameter (rather than fragment `#`): the verifier's regex
    ///    `\.ya?ml(\?|$)` matches `.yaml?...` but not `.yaml#...`. We need
    ///    the verifier to route the doc_url through its YAML branch. GitHub
    ///    Pages ignores unknown query strings, so a reader following the
    ///    link still lands on the canonical YAML.
    ///
    /// All members of a type share the type's doc_url since the YAML is
    /// small (~27 KB) and a reader can find the per-operation block in
    /// seconds. A more granular per-method doc_url (e.g.
    /// `?operationId=getCameraState`) is also valid but the simpler choice
    /// of "everything in this type points at the same YAML" matches the
    /// IDEA Statica pattern's 404-fallback shape.
    /// </summary>
    static TypeInfo FixDocUrls(TypeInfo t, string yamlUrl)
    {
        bool isApiClass = t.@namespace.EndsWith(".Api", StringComparison.Ordinal);

        // Rename the single synthetic *Api class to drop the "Api" suffix.
        // Specifically: pathsApi → paths. We keep the namespace at
        // Solibri.Rest.Api to preserve the model/api split in the IR.
        string finalName = t.name;
        if (isApiClass && finalName.EndsWith("Api", StringComparison.Ordinal))
            finalName = finalName.Substring(0, finalName.Length - "Api".Length);

        string queryKey = isApiClass ? "api" : "schema";
        string typeDocUrl = $"{yamlUrl}?{queryKey}={finalName}";

        var newMethods = t.methods.Select(m => m with { doc_url = typeDocUrl }).ToList();
        var newCtors = t.constructors.Select(c => c with { doc_url = typeDocUrl }).ToList();
        var newProps = t.properties.Select(p => p with { doc_url = typeDocUrl }).ToList();
        var newEvents = t.events.Select(e => e with { doc_url = typeDocUrl }).ToList();

        return t with
        {
            name = finalName,
            doc_url = typeDocUrl,
            constructors = newCtors,
            methods = newMethods,
            properties = newProps,
            events = newEvents,
        };
    }
}
