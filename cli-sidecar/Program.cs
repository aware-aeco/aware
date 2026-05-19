using System.Text.Json;
using System.Text.Json.Serialization;
using AwareSidecar.Protocol;

[assembly: System.Runtime.CompilerServices.InternalsVisibleTo("cli-sidecar.Tests")]

namespace AwareSidecar;

internal static class Program
{
    private const string Version = "0.5.1";

    public static int Main(string[] args)
    {
        string? line = null;
        try
        {
            line = Console.In.ReadLine();
            if (string.IsNullOrWhiteSpace(line))
            {
                EmitError(null, "stdin closed with no input");
                return 2;
            }

            var envelope = JsonSerializer.Deserialize(line, SidecarJsonContext.Default.RequestEnvelope);
            if (envelope is null)
            {
                EmitError(null, "could not parse request envelope");
                return 2;
            }

            switch (envelope.Op)
            {
                case "reflect-dlls":
                {
                    var argsObj = envelope.Args.Deserialize(SidecarJsonContext.Default.ReflectDllsArgs);
                    if (argsObj is null) { EmitError(envelope.Op, "missing args"); return 2; }
                    try
                    {
                        var agent = AwareSidecar.Reflection.DllReflector.Reflect(argsObj.Globs, argsObj.AgentId);
                        EmitOk(envelope.Op, new ResponseData { Agent = agent });
                        return 0;
                    }
                    catch (Exception ex)
                    {
                        EmitError(envelope.Op, ex.Message);
                        return 1;
                    }
                }
                case "decompile":
                {
                    var argsObj = envelope.Args.Deserialize(SidecarJsonContext.Default.DecompileArgs);
                    if (argsObj is null) { EmitError(envelope.Op, "missing args"); return 2; }
                    try
                    {
                        var agent = AwareSidecar.Decompile.IlspyShell.Decompile(argsObj.PackagePath, argsObj.Version, argsObj.AgentId, argsObj.AcceptLicense);
                        EmitOk(envelope.Op, new ResponseData { Agent = agent });
                        return 0;
                    }
                    catch (Exception ex)
                    {
                        EmitError(envelope.Op, ex.Message);
                        return 1;
                    }
                }
                case "from-com":
                {
                    if (!OperatingSystem.IsWindows())
                    {
                        EmitError(envelope.Op, "--from-com is Windows-only");
                        return 1;
                    }
                    var argsObj = envelope.Args.Deserialize(SidecarJsonContext.Default.FromComArgs);
                    if (argsObj is null) { EmitError(envelope.Op, "missing args"); return 2; }
                    try
                    {
                        var agent = AwareSidecar.Com.ComReflector.Reflect(argsObj.ProgId, argsObj.AgentId);
                        EmitOk(envelope.Op, new ResponseData { Agent = agent });
                        return 0;
                    }
                    catch (Exception ex) { EmitError(envelope.Op, ex.Message); return 1; }
                }
                case "from-headers":
                {
                    var argsObj = envelope.Args.Deserialize(SidecarJsonContext.Default.FromHeadersArgs);
                    if (argsObj is null) { EmitError(envelope.Op, "missing args"); return 2; }
                    try
                    {
                        var agent = AwareSidecar.Headers.HeaderParser.Parse(argsObj.Files, argsObj.AgentId);
                        EmitOk(envelope.Op, new ResponseData { Agent = agent });
                        return 0;
                    }
                    catch (Exception ex) { EmitError(envelope.Op, ex.Message); return 1; }
                }
                case "coverage-extract":
                {
                    var argsObj = envelope.Args.Deserialize(SidecarJsonContext.Default.CoverageExtractArgs);
                    if (argsObj is null) { EmitError(envelope.Op, "missing args"); return 2; }
                    if (string.IsNullOrEmpty(argsObj.Vendor)
                        || string.IsNullOrEmpty(argsObj.Version)
                        || string.IsNullOrEmpty(argsObj.OutPath))
                    {
                        EmitError(envelope.Op, "coverage-extract requires vendor, version, out_path");
                        return 2;
                    }
                    try
                    {
                        // Synchronous wrapper around the per-vendor async extractor. We block
                        // the Main thread on purpose — the sidecar is one-shot, no other work
                        // is happening on this process.
                        Ingest.CoverageIR ir = argsObj.Vendor.ToLowerInvariant() switch
                        {
                            // The Tekla extractor accepts the destination path so it can
                            // snapshot progress to disk and resume if the process is killed.
                            "tekla" => Ingest.Extractors.Tekla.Extractor.Extract(argsObj.Version, argsObj.OutPath)
                                .GetAwaiter().GetResult(),
                            // Rhino extractor (Phase B B5) — RhinoCommon docs from the McNeel
                            // github mirror at developer.rhino3d.com / mcneel/rhinocommon-api-docs.
                            // Version 7.0 → branch `7`; version 8.0 → branch `gh-pages` (Rhino 8.31).
                            "rhino" => Ingest.Extractors.Rhino.Extractor.Extract(argsObj.Version, argsObj.OutPath)
                                .GetAwaiter().GetResult(),
                            // Grasshopper extractor (Phase B B6) — Grasshopper SDK docs from
                            // mcneel/grasshopper-api-docs. Version 7.0 → branch `7`; version 8.0
                            // → branch `gh-pages` (Grasshopper for Rhino 8.31). Identical Sandcastle
                            // shape as Rhino — shares the parser via Grasshopper.Extractor's
                            // delegation to the Rhino PageParser/MemberPageParser.
                            "grasshopper" => Ingest.Extractors.Grasshopper.Extractor.Extract(argsObj.Version, argsObj.OutPath)
                                .GetAwaiter().GetResult(),
                            // IDEA Statica extractor (Phase B B8) — hybrid REST + .NET SDK docs from
                            // idea-statica/ideastatica-public's tagged release tree. Versions 25.0
                            // (tag 25.1.5.1491) and 26.0 (tag 26.0.1.2450) each carry openapi.yaml
                            // + markdown SDK docs for the Connection API and the RCS API; the
                            // extractor merges both APIs into one IR per version.
                            "idea-statica" => Ingest.Extractors.IdeaStatica.Extractor.Extract(argsObj.Version, argsObj.OutPath)
                                .GetAwaiter().GetResult(),
                            _ => throw new NotSupportedException(
                                $"coverage-extract: vendor '{argsObj.Vendor}' has no extractor yet")
                        };
                        var outDir = Path.GetDirectoryName(argsObj.OutPath);
                        if (!string.IsNullOrEmpty(outDir)) Directory.CreateDirectory(outDir);
                        var json = JsonSerializer.Serialize(ir, Ingest.IrJsonContext.Default.CoverageIR);
                        File.WriteAllText(argsObj.OutPath, json);
                        EmitOk(envelope.Op, new ResponseData
                        {
                            Coverage = new CoverageGenerateResult
                            {
                                Manifest = argsObj.OutPath,            // re-using the field to carry the IR path back
                                SkillCount = ir.types.Count,
                                CatalogCount = ir.metadata.method_count + ir.metadata.event_count + ir.metadata.property_count,
                                Total = ir.metadata.type_count,
                                Skills = Array.Empty<string>(),
                                Catalogs = Array.Empty<string>(),
                            }
                        });
                        return 0;
                    }
                    catch (Exception ex)
                    {
                        Console.Error.WriteLine($"[coverage-extract] EXCEPTION: {ex}");
                        EmitError(envelope.Op, $"{ex.GetType().Name}: {ex.Message}");
                        return 1;
                    }
                }
                case "coverage-generate":
                {
                    var argsObj = envelope.Args.Deserialize(SidecarJsonContext.Default.CoverageGenerateArgs);
                    if (argsObj is null) { EmitError(envelope.Op, "missing args"); return 2; }
                    if (string.IsNullOrEmpty(argsObj.IrPath)
                        || string.IsNullOrEmpty(argsObj.OutDir)
                        || string.IsNullOrEmpty(argsObj.AgentId)
                        || string.IsNullOrEmpty(argsObj.Vendor)
                        || string.IsNullOrEmpty(argsObj.Vertical))
                    {
                        EmitError(envelope.Op, "coverage-generate requires ir_path, out_dir, agent_id, vendor, vertical");
                        return 2;
                    }
                    try
                    {
                        var result = AwareSidecar.Ingest.Generator.Generate(new AwareSidecar.Ingest.GenerateRequest(
                            IrPath:    argsObj.IrPath,
                            OutputDir: argsObj.OutDir,
                            AgentId:   argsObj.AgentId,
                            Vendor:    argsObj.Vendor,
                            Vertical:  argsObj.Vertical));
                        EmitOk(envelope.Op, new ResponseData
                        {
                            Coverage = new CoverageGenerateResult
                            {
                                Manifest = result.ManifestPath,
                                SkillCount = result.SkillPaths.Count,
                                CatalogCount = result.CatalogPaths.Count,
                                Total = result.TotalArtefactsWritten,
                                Skills = result.SkillPaths.ToArray(),
                                Catalogs = result.CatalogPaths.ToArray(),
                            }
                        });
                        return 0;
                    }
                    catch (Exception ex) { EmitError(envelope.Op, ex.Message); return 1; }
                }
                default:
                {
                    EmitError(envelope.Op, $"unknown op: {envelope.Op}");
                    return 2;
                }
            }
        }
        catch (JsonException ex)
        {
            EmitError(null, $"invalid JSON: {ex.Message}");
            return 2;
        }
        catch (Exception ex)
        {
            EmitError(null, $"unhandled: {ex.Message}");
            return 1;
        }
    }

    private static void EmitOk(string op, ResponseData data)
    {
        var resp = new OkResponse { Ok = true, Version = Version, Op = op, Data = data };
        Console.WriteLine(JsonSerializer.Serialize(resp, SidecarJsonContext.Default.OkResponse));
    }

    private static void EmitError(string? op, string message)
    {
        var resp = new ErrorResponse { Ok = false, Version = Version, Op = op, Error = message };
        Console.WriteLine(JsonSerializer.Serialize(resp, SidecarJsonContext.Default.ErrorResponse));
    }
}

[JsonSerializable(typeof(RequestEnvelope))]
[JsonSerializable(typeof(ReflectDllsArgs))]
[JsonSerializable(typeof(DecompileArgs))]
[JsonSerializable(typeof(FromComArgs))]
[JsonSerializable(typeof(FromHeadersArgs))]
[JsonSerializable(typeof(CoverageGenerateArgs))]
[JsonSerializable(typeof(CoverageExtractArgs))]
[JsonSerializable(typeof(OkResponse))]
[JsonSerializable(typeof(ErrorResponse))]
[JsonSerializable(typeof(ResponseData))]
[JsonSerializable(typeof(GeneratedAgent))]
[JsonSerializable(typeof(GeneratedCommand))]
[JsonSerializable(typeof(GeneratedSkill))]
[JsonSerializable(typeof(CoverageGenerateResult))]
internal partial class SidecarJsonContext : JsonSerializerContext { }
