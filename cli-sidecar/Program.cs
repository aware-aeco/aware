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
[JsonSerializable(typeof(OkResponse))]
[JsonSerializable(typeof(ErrorResponse))]
[JsonSerializable(typeof(ResponseData))]
[JsonSerializable(typeof(GeneratedAgent))]
[JsonSerializable(typeof(GeneratedCommand))]
[JsonSerializable(typeof(GeneratedSkill))]
[JsonSerializable(typeof(CoverageGenerateResult))]
internal partial class SidecarJsonContext : JsonSerializerContext { }
