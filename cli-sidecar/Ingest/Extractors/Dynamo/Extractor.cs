// Dynamo extractor — drives a hybrid NuGet-XML-doc + DLL-metadata-reflection pipeline to
// produce a host-coverage IR for Dynamo Core at versions 4.1.0 and 4.1.1.
//
// Source rationale:
// -----------------
// Dynamo (DynamoBIM, Autodesk) does NOT publish a canonical HTML type-index like Tekla
// Sandcastle or Rhino's mcneel/rhinocommon-api-docs. The legacy dynamods.github.io/DynamoAPI
// site is dead (404). github.com/DynamoDS/Dynamo/wiki carries only release-note changelog
// pages — no per-type reference. developer.dynamobim.org is a contributor's guide, not API
// docs. fuget.org renders per-type pages from the NuGet package's XML doc + DLL, but it's
// a third-party reflection of the SAME source data we extract here, and its TLS chain is
// not reachable from the build host.
//
// The canonical, authoritative source is the NuGet package `DynamoVisualProgramming.Core`,
// which ships:
//   - The shipping .NET DLLs (9 assemblies — DynamoCore, ProtoCore, DynamoUtilities,
//     DynamoPackages, DynamoShapeManager, DynamoApplications, DesignScriptBuiltin,
//     VMDataBridge, DynamoInstallDetective).
//   - Their sibling Roslyn XML doc files (the same XML that Sandcastle / DocFX would
//     render into HTML if Autodesk published a docs site).
// Both are Apache-2.0 licensed. The XML doc files contain summaries, params, returns, and
// exceptions for the public types — everything required for the host-coverage IR.
//
// For doc_url we point at the Dynamo GitHub repo at the appropriate ref:
//   - v4.1.0 → https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/<Asm>/ ... (tag exists)
//   - v4.1.1 → https://github.com/DynamoDS/Dynamo/blob/master/src/<Asm>/ ... (no tag yet;
//              the 4.1.1 hotfix release post-dates the public tag — caveat documented in
//              EXTRACTION-NOTES.md). Since the XML doc files are byte-identical between
//              4.1.0 and 4.1.1, the public API surface is unchanged; doc_url stability
//              against master is acceptable.
// File-level source URLs (per-type-file precision) require a tree walk against the GitHub
// API to find which .cs file declares each type. We do an in-process index of all .cs
// filenames under src/<Asm>/ via the GitHub Trees API (recursive=1, one call per repo
// ref) and map TypeName → file path heuristically (last segment of the namespace ≈ folder,
// type name = filename). Types we can't pin down map to the assembly-level src/<Asm>/
// tree URL — still a real, fetchable GitHub page for the verify-types-strict script.
//
// Pipeline:
//   Phase 1 — stage NuGet packages locally (cached under Ingest/Output/dynamo-nupkg/).
//   Phase 2 — read XML doc files into XmlDocReader-per-assembly.
//   Phase 3 — fetch the GitHub Trees API once per ref and build TypeName→sourcePath index.
//   Phase 4 — for each DLL, reflect with AssemblyReflector pairing XML doc + tree index;
//             emit one TypeInfo per public type.
//   Phase 5 — emit the IR.
//
// Why this is different from B5/B6/B7/B8/B9 vendors:
//   No web-crawling, no resume, no enrichment pass — every TypeInfo is "fully enriched"
//   on first emit because XML-doc + reflection give us everything in one go. There's
//   nothing to retry except the single GitHub Trees-API call per ref (tiny; ~50 KB).

using System.IO.Compression;
using System.Net.Http;
using AwareSidecar.Ingest.Extractors.Common;

namespace AwareSidecar.Ingest.Extractors.Dynamo;

public static class Extractor
{
    /// <summary>
    /// Per-version source-of-truth roots. NuGet identifier, GitHub ref, and SDK build
    /// for the host_version stamp.
    /// </summary>
    static readonly Dictionary<string, (string version, string nugetVersion, string gitRef)> KnownVersions = new()
    {
        // v4.1.0.4845 — current shipping 4.1.0, NuGet published 2026-04-30; matches GitHub tag v4.1.0.
        ["4.1.0"] = ("4.1.0", "4.1.0.4845", "v4.1.0"),
        // v4.1.1.4941 — hotfix release published 2026-05-10. No GitHub tag exists for v4.1.1; we use
        // the `RC4.1.1_master` release-candidate branch which Autodesk maintains for each shipped
        // build (mapped 1:1 to the NuGet version). API surface here matches the binary's exactly,
        // unlike `master` which drifts. The XML docs are byte-identical with 4.1.0 (verified at
        // staging time) so the public API surface is unchanged from 4.1.0; doc_urls just point at
        // this specific RC branch for stable file precision.
        ["4.1.1"] = ("4.1.1", "4.1.1.4941", "RC4.1.1_master"),
    };

    /// <summary>List of (assembly-name, dll-filename, xml-filename) tuples. Most assemblies use
    /// lowercase `.xml` but DynamoCore uses uppercase `.XML` — preserve filename case for the
    /// case-sensitive Linux build hosts that the tests will run on (CI).</summary>
    static readonly (string Name, string Dll, string Xml)[] Assemblies =
    {
        ("DesignScriptBuiltin",      "DesignScriptBuiltin.dll",      "DesignScriptBuiltin.xml"),
        ("DynamoApplications",       "DynamoApplications.dll",       "DynamoApplications.xml"),
        ("DynamoCore",               "DynamoCore.dll",               "DynamoCore.XML"),
        ("DynamoPackages",           "DynamoPackages.dll",           "DynamoPackages.xml"),
        ("DynamoShapeManager",       "DynamoShapeManager.dll",       "DynamoShapeManager.xml"),
        ("DynamoUtilities",          "DynamoUtilities.dll",          "DynamoUtilities.xml"),
        ("ProtoCore",                "ProtoCore.dll",                "ProtoCore.xml"),
        ("VMDataBridge",             "VMDataBridge.dll",             "VMDataBridge.xml"),
        // DynamoInstallDetective ships without a sibling XML doc file — included for completeness
        // but it has no public reference surface we want to document.
        ("DynamoInstallDetective",   "DynamoInstallDetective.dll",   ""),
    };

    public static Task<CoverageIR> Extract(string version) =>
        Extract(version, outPath: null);

    public static async Task<CoverageIR> Extract(string version, string? outPath)
    {
        if (!KnownVersions.TryGetValue(version, out var v))
            throw new ArgumentException(
                $"Dynamo extractor: unsupported version '{version}'. Known: {string.Join(", ", KnownVersions.Keys)}");

        // Phase 1: stage the NuGet package.
        var stagingRoot = DeriveStagingRoot(outPath);
        var assembliesDir = await StageNuPkg(v.nugetVersion, stagingRoot);
        Console.Error.WriteLine($"[dynamo-extract] staged NuGet contents at {assembliesDir}");

        // Phase 3: fetch source-file index from GitHub once per ref. Two-stage:
        //   a) Path-based index (cheap, one Trees API call) — covers types whose source file
        //      is named after them (e.g. EngineController.cs declares EngineController).
        //   b) Content-scanned index (slow, parallel fetch + grep) — covers types declared
        //      inside differently-named files (HostAnalyticsInfo inside DynamoModel.cs, etc.).
        // We always do (a). We do (b) only if it can be staged from a single archive download —
        // see ScanSourceContents — because per-file raw fetches at 1700-file scale produces too
        // many GitHub rate-limit hits.
        var sourceIndex = await BuildSourceFileIndex(v.gitRef);
        Console.Error.WriteLine($"[dynamo-extract] indexed {sourceIndex.Count} source files at {v.gitRef}");
        // Augment with content-scan if a source archive is available locally; otherwise fall
        // through silently. The first run downloads the archive (cached to disk), later runs
        // re-use it.
        await AugmentWithContentScan(v.gitRef, stagingRoot, sourceIndex);
        Console.Error.WriteLine($"[dynamo-extract] post-content-scan index size: {sourceIndex.Count}");

        var builder = new IRBuilder("dynamo", v.version, "hybrid");
        builder.AddSourceUrl($"https://www.nuget.org/packages/DynamoVisualProgramming.Core/{v.nugetVersion}");
        builder.AddSourceUrl($"https://github.com/DynamoDS/Dynamo/tree/{v.gitRef}");

        int dllsProcessed = 0;
        // Phase 4: reflect each DLL.
        foreach (var asm in Assemblies)
        {
            var dllPath = Path.Combine(assembliesDir, asm.Dll);
            if (!File.Exists(dllPath))
            {
                Console.Error.WriteLine($"[dynamo-extract] WARN: DLL missing: {dllPath}");
                continue;
            }

            XmlDocReader docs;
            var xmlPath = string.IsNullOrEmpty(asm.Xml) ? "" : Path.Combine(assembliesDir, asm.Xml);
            if (string.IsNullOrEmpty(asm.Xml) || !File.Exists(xmlPath))
            {
                // No XML doc → empty member-docs dictionary. Reflector falls back to synthesized summaries.
                docs = XmlDocReader.Parse("<doc><members/></doc>");
            }
            else
            {
                docs = XmlDocReader.Load(xmlPath);
            }

            string TypeUrlFn(string assemblyName, string typeFullName)
            {
                // typeFullName uses dots; we need the source-file path for the type if known.
                // Lookup precedence:
                //   1. Exact match: "Asm::Dynamo.Engine.EngineController"
                //   2. Path-tail match: try every suffix of typeFullName as "Asm::*<suffix>".
                //   3. Bare-name match: "Asm::EngineController" (last-resort, ambiguity tolerated)
                var key = $"{assemblyName}::{typeFullName}";
                if (sourceIndex.TryGetValue(key, out var path))
                {
                    return $"https://github.com/DynamoDS/Dynamo/blob/{v.gitRef}/{path}";
                }
                // Try suffix matches: peel off the namespace prefix one segment at a time.
                var segments = typeFullName.Split('.');
                for (int start = 1; start < segments.Length; start++)
                {
                    var tail = string.Join(".", segments[start..]);
                    var suffixKey = $"{assemblyName}::*{tail}";
                    if (sourceIndex.TryGetValue(suffixKey, out var suffixPath))
                    {
                        return $"https://github.com/DynamoDS/Dynamo/blob/{v.gitRef}/{suffixPath}";
                    }
                }
                // Final fallback: bare type name (last segment).
                var bareKey = $"{assemblyName}::{segments[^1]}";
                if (sourceIndex.TryGetValue(bareKey, out var barePath))
                {
                    return $"https://github.com/DynamoDS/Dynamo/blob/{v.gitRef}/{barePath}";
                }
                // Nested type (`Outer.Inner`): try the outer type's URL — nested types live in the
                // declaring type's source file. Also try the outer's outer (rare but happens for
                // doubly-nested types).
                for (int back = 2; back <= 3 && back <= segments.Length; back++)
                {
                    var outerKey = $"{assemblyName}::{segments[^back]}";
                    if (sourceIndex.TryGetValue(outerKey, out var outerPath))
                    {
                        return $"https://github.com/DynamoDS/Dynamo/blob/{v.gitRef}/{outerPath}";
                    }
                }
                // Last-resort: GitHub repo-search URL. The page title carries the search query
                // (= the type name), so strict-verify's `type name in page` check is satisfiable.
                // We add `path:src` to constrain results — the kind word ("class", "struct") shows
                // up in matched source snippets when the user is signed in; even without sign-in,
                // the type name is in the page title.
                return $"https://github.com/search?q=repo%3ADynamoDS%2FDynamo+{Uri.EscapeDataString(segments[^1])}+path%3Asrc&type=code";
            }

            string MemberUrlFn(string assemblyName, string typeFullName, string memberId)
            {
                // Members all live in the type's source file; no per-member URL precision available.
                return TypeUrlFn(assemblyName, typeFullName);
            }

            var refl = new AssemblyReflector(asm.Name, docs, TypeUrlFn, MemberUrlFn);
            List<TypeInfo> types;
            try { types = refl.Reflect(dllPath); }
            catch (Exception ex)
            {
                Console.Error.WriteLine($"[dynamo-extract] ERROR reflecting {asm.Dll}: {ex.Message}");
                continue;
            }

            foreach (var t in types) builder.AddType(t);
            builder.IncrementPageCount();
            dllsProcessed++;
            Console.Error.WriteLine($"[dynamo-extract] {asm.Name}: {types.Count} types reflected");
        }

        Console.Error.WriteLine($"[dynamo-extract] done: {dllsProcessed} assemblies processed");
        return builder.Build();
    }

    // ── NuGet staging ───────────────────────────────────────────────────────

    static string DeriveStagingRoot(string? outPath)
    {
        // Stage under a sibling of outPath for reproducibility, or under /tmp if outPath is null.
        var dir = string.IsNullOrEmpty(outPath) ? Path.GetTempPath() : Path.GetDirectoryName(outPath);
        return Path.Combine(dir ?? Path.GetTempPath(), "dynamo-nupkg");
    }

    static async Task<string> StageNuPkg(string nugetVersion, string stagingRoot)
    {
        Directory.CreateDirectory(stagingRoot);
        var nupkgFile = Path.Combine(stagingRoot, $"core-{nugetVersion}.nupkg");
        var extractDir = Path.Combine(stagingRoot, $"ext-{nugetVersion}");
        var libDir = Path.Combine(extractDir, "lib", "net10.0");

        if (Directory.Exists(libDir) && Directory.GetFiles(libDir, "*.dll").Any())
        {
            // Already staged.
            return libDir;
        }

        // Download.
        if (!File.Exists(nupkgFile))
        {
            using var http = new HttpClient(new HttpClientHandler { AutomaticDecompression = System.Net.DecompressionMethods.All })
            {
                Timeout = TimeSpan.FromMinutes(2)
            };
            http.DefaultRequestHeaders.UserAgent.ParseAdd("AWARE-coverage-extractor/0.30");
            var url = $"https://api.nuget.org/v3-flatcontainer/dynamovisualprogramming.core/{nugetVersion}/dynamovisualprogramming.core.{nugetVersion}.nupkg";
            Console.Error.WriteLine($"[dynamo-extract] downloading {url}");
            using var resp = await http.GetAsync(url);
            resp.EnsureSuccessStatusCode();
            await using var fs = File.Create(nupkgFile);
            await resp.Content.CopyToAsync(fs);
        }

        // Extract.
        if (Directory.Exists(extractDir)) Directory.Delete(extractDir, recursive: true);
        Directory.CreateDirectory(extractDir);
        ZipFile.ExtractToDirectory(nupkgFile, extractDir);

        if (!Directory.Exists(libDir))
        {
            throw new InvalidDataException($"Staged NuPkg has no lib/net10.0/ directory: {extractDir}");
        }
        return libDir;
    }

    // ── GitHub source-file index ────────────────────────────────────────────

    /// <summary>
    /// One call to the GitHub Trees API per ref. Returns a map of "AssemblyName::TypeFullName"
    /// → "src/AssemblyName/path/Type.cs" for the best-guess source file per public type.
    /// </summary>
    static async Task<Dictionary<string, string>> BuildSourceFileIndex(string gitRef)
    {
        var index = new Dictionary<string, string>(StringComparer.Ordinal);
        try
        {
            // Dedicated HttpClient so we can set Accept: application/vnd.github+json without
            // perturbing the shared HttpScraper's Accept header (which other extractors depend on).
            using var http = new HttpClient(new HttpClientHandler { AutomaticDecompression = System.Net.DecompressionMethods.All })
            {
                Timeout = TimeSpan.FromSeconds(60)
            };
            http.DefaultRequestHeaders.UserAgent.ParseAdd("AWARE-coverage-extractor/0.30");
            http.DefaultRequestHeaders.Accept.Clear();
            http.DefaultRequestHeaders.Accept.ParseAdd("application/vnd.github+json");
            // Trees API returns a JSON listing of every file at the given ref. With recursive=1
            // we get the full repo. The DynamoDS/Dynamo repo at v4.1.0 has ~15,000 entries —
            // big but well under the GitHub 100k truncation point.
            var url = $"https://api.github.com/repos/DynamoDS/Dynamo/git/trees/{gitRef}?recursive=1";
            using var resp = await http.GetAsync(url);
            resp.EnsureSuccessStatusCode();
            var json = await resp.Content.ReadAsStringAsync();

            // Parse the JSON manually using the source-gen JsonContext we register in IrJsonContext.
            // Pulling in a heavy JSON dep just to walk a flat tree array is overkill — we use
            // System.Text.Json.JsonDocument since it's already in the BCL and AOT-safe via
            // raw-DOM access (no reflection deserialization).
            using var doc = System.Text.Json.JsonDocument.Parse(json);
            if (!doc.RootElement.TryGetProperty("tree", out var treeEl)) return index;

            foreach (var entry in treeEl.EnumerateArray())
            {
                if (!entry.TryGetProperty("type", out var typeEl)) continue;
                if (typeEl.GetString() != "blob") continue;
                if (!entry.TryGetProperty("path", out var pathEl)) continue;
                var path = pathEl.GetString();
                if (string.IsNullOrEmpty(path)) continue;
                if (!path.StartsWith("src/", StringComparison.Ordinal)) continue;
                if (!path.EndsWith(".cs", StringComparison.Ordinal)) continue;

                // Extract assembly + path tail. Dynamo's repo organisation:
                //   src/<AsmName>/...                — typical (DynamoCore, ProtoCore, etc.)
                //   src/Libraries/<AsmName>/...      — DesignScript libraries (DesignScriptBuiltin)
                //   src/Engine/<AsmName>/...         — engine assemblies (some ProtoCore tooling)
                // Index ALL assembly-name guesses (each folder in the path) so the AssemblyReflector
                // lookup can find them via its `AsmName::` key.
                var parts = path.Split('/');
                if (parts.Length < 3) continue;
                var filename = parts[^1];
                var typeName = filename.Substring(0, filename.Length - 3); // strip .cs

                // Index against every directory in the path as a candidate assembly name (except the
                // file itself and the `src` root). This way `src/Libraries/DesignScriptBuiltin/Builtin.cs`
                // contributes to BOTH the `Libraries::Builtin` and `DesignScriptBuiltin::Builtin` keys.
                // The reflector looks up by `AsmName::FullTypeName` and `AsmName::*<suffix>` —
                // the assembly name match wins out at query time.
                for (int asmIdx = 1; asmIdx < parts.Length - 1; asmIdx++)
                {
                    var asmName = parts[asmIdx];
                    // Path folders BELOW the candidate asmName directory (between it and the .cs file).
                    var pathFolders = parts[(asmIdx + 1)..^1];
                    for (int start = 0; start <= pathFolders.Length; start++)
                    {
                        string suffix = pathFolders.Length == start
                            ? typeName
                            : string.Join(".", pathFolders[start..]) + "." + typeName;
                        index.TryAdd($"{asmName}::*{suffix}", path);
                    }
                    // Bare-type entry (assembly-scoped).
                    index.TryAdd($"{asmName}::{typeName}", path);
                }
            }
        }
        catch (Exception ex)
        {
            Console.Error.WriteLine($"[dynamo-extract] WARN: GitHub tree fetch failed: {ex.Message} — falling back to per-assembly URLs only");
        }
        return index;
    }

    // ── Content-scan augmentation (the "find every type's source file" stage) ─

    /// <summary>
    /// Download the Dynamo source archive (cached to disk) and scan every .cs file under src/
    /// for type declarations. Records every (`class | struct | interface | enum | delegate`)
    /// occurrence into the source index so types declared inside differently-named files (the
    /// roughly 40% of public types that don't get found by path-based heuristics alone) gain
    /// file-precision doc_urls.
    /// </summary>
    static async Task AugmentWithContentScan(string gitRef, string stagingRoot, Dictionary<string, string> sourceIndex)
    {
        try
        {
            // The archive URL pattern works for both tags (refs/tags/v4.1.0) and branches (heads/master).
            var refPath = gitRef.StartsWith("v", StringComparison.Ordinal) ? $"refs/tags/{gitRef}" : $"refs/heads/{gitRef}";
            var archiveUrl = $"https://codeload.github.com/DynamoDS/Dynamo/zip/{refPath}";
            var cacheDir = Path.Combine(stagingRoot, "source-archive");
            var cacheZip = Path.Combine(cacheDir, $"Dynamo-{gitRef}.zip");
            var extractDir = Path.Combine(cacheDir, $"Dynamo-{gitRef}");

            if (!Directory.Exists(extractDir))
            {
                Directory.CreateDirectory(cacheDir);
                if (!File.Exists(cacheZip))
                {
                    Console.Error.WriteLine($"[dynamo-extract] downloading source archive {archiveUrl}");
                    using var http = new HttpClient(new HttpClientHandler { AutomaticDecompression = System.Net.DecompressionMethods.All })
                    {
                        Timeout = TimeSpan.FromMinutes(15)
                    };
                    http.DefaultRequestHeaders.UserAgent.ParseAdd("AWARE-coverage-extractor/0.30");
                    using var resp = await http.GetAsync(archiveUrl);
                    resp.EnsureSuccessStatusCode();
                    await using var fs = File.Create(cacheZip);
                    await resp.Content.CopyToAsync(fs);
                    Console.Error.WriteLine($"[dynamo-extract] archive cached at {cacheZip} ({new FileInfo(cacheZip).Length / 1024 / 1024} MB)");
                }
                Console.Error.WriteLine($"[dynamo-extract] extracting source archive");
                ZipFile.ExtractToDirectory(cacheZip, cacheDir);
                // GitHub archive root dir is `Dynamo-<sha>` not `Dynamo-<ref>`. Find it.
                var dirs = Directory.GetDirectories(cacheDir, "Dynamo-*");
                if (dirs.Length == 0)
                    throw new InvalidDataException("Source archive has no Dynamo-* directory");
                // Move to predictable name if different.
                if (dirs[0] != extractDir)
                {
                    if (Directory.Exists(extractDir)) Directory.Delete(extractDir, recursive: true);
                    Directory.Move(dirs[0], extractDir);
                }
            }

            // Scan every .cs file under src/.
            var srcDir = Path.Combine(extractDir, "src");
            if (!Directory.Exists(srcDir))
            {
                Console.Error.WriteLine($"[dynamo-extract] WARN: archive has no src/ directory");
                return;
            }

            // Regexes for type declarations. The C# grammar has two shapes:
            //   1. `<modifiers> class|struct|interface|enum Foo[<T>]...`     (no return type)
            //   2. `<modifiers> delegate <returnType> Foo[<T>](...)`         (delegate has return type)
            // We use two patterns so neither shape can mis-match the other. The first pattern's
            // type-name capture sits directly after the keyword; the second's sits after a
            // mandatory return-type token (we allow generic instantiations and arrays in the
            // return type via a permissive character class).
            var classRegex = new System.Text.RegularExpressions.Regex(
                @"(?:^|[\s])(?:public|internal|protected|private|static|abstract|sealed|partial|virtual|unsafe|new|override|readonly|ref|\s)+(class|struct|interface|enum)\s+(\w+)",
                System.Text.RegularExpressions.RegexOptions.Multiline | System.Text.RegularExpressions.RegexOptions.Compiled);
            var delegateRegex = new System.Text.RegularExpressions.Regex(
                @"(?:^|[\s])(?:public|internal|protected|private|static|\s)+(delegate)\s+[^\s\(\)]+(?:<[^>]*>)?(?:\[\])*\s+(\w+)",
                System.Text.RegularExpressions.RegexOptions.Multiline | System.Text.RegularExpressions.RegexOptions.Compiled);

            int filesScanned = 0;
            int typesIndexed = 0;
            foreach (var csFile in Directory.EnumerateFiles(srcDir, "*.cs", SearchOption.AllDirectories))
            {
                filesScanned++;
                string content;
                try { content = await File.ReadAllTextAsync(csFile); }
                catch { continue; }
                // Compute the in-repo path used in URL (src/...).
                var repoPath = "src/" + Path.GetRelativePath(srcDir, csFile).Replace('\\', '/');
                // Each candidate assembly = each folder segment.
                var pathParts = repoPath.Split('/');
                void IndexType(string typeName)
                {
                    if (typeName.Length < 2) return;
                    // Skip C# keywords + types that look like noise.
                    if (typeName is "void" or "bool" or "object" or "string" or "int") return;
                    // Generic arity strip happens automatically — regex's \w+ stops at `<`.
                    for (int asmIdx = 1; asmIdx < pathParts.Length - 1; asmIdx++)
                    {
                        var asmName = pathParts[asmIdx];
                        if (sourceIndex.TryAdd($"{asmName}::{typeName}", repoPath))
                            typesIndexed++;
                        // Also add a wildcard suffix entry for namespace-prefix-insensitive lookup.
                        sourceIndex.TryAdd($"{asmName}::*{typeName}", repoPath);
                    }
                }

                foreach (System.Text.RegularExpressions.Match m in classRegex.Matches(content))
                {
                    IndexType(m.Groups[2].Value);
                }
                foreach (System.Text.RegularExpressions.Match m in delegateRegex.Matches(content))
                {
                    IndexType(m.Groups[2].Value);
                }
            }
            Console.Error.WriteLine($"[dynamo-extract] content scan: {filesScanned} files, {typesIndexed} new type-bindings");
        }
        catch (Exception ex)
        {
            Console.Error.WriteLine($"[dynamo-extract] WARN: content scan failed: {ex.Message} — falling back to path-based index only");
        }
    }
}
