// CatalogWriter — serialize each TypeInfo in a CoverageIR into a single JSON file.
//
// One JSON file per type, keyed as "<namespace>.<typename>.json". The structured
// catalog is the queryable counterpart to the narrative skill markdown — it's what
// downstream tooling (the CLI, agent runners, evaluators) reads to discover the
// surface area of a host without parsing prose.
//
// Nullable fields with a null value are OMITTED from the output to keep the
// catalog compact; the IR's reflection-based deserializer treats absent fields
// as null by default, so this is a lossless space optimization.
//
// AOT note: like IRReader, this writer uses reflection-based serialization and is
// annotated with `[RequiresUnreferencedCode]` / `[RequiresDynamicCode]` to surface
// that constraint to callers. The sidecar's NativeAOT entry point in A6 will
// either swap in a JsonSerializerContext source generator or invoke this from a
// managed (non-AOT) host. Either choice is local to that task — the API here is
// stable.
//
// This writer is pure / stateless / synchronous; the caller is responsible
// for any I/O.

using System.Diagnostics.CodeAnalysis;
using System.Text.Json;
using System.Text.Json.Serialization.Metadata;

namespace AwareSidecar.Ingest;

public static class CatalogWriter
{
    // See IRReader for the rationale on the explicit TypeInfoResolver: the
    // sidecar csproj's PublishAot=true disables reflection-based JSON globally,
    // so we re-enable it for this writer's options object. Trim/AOT warnings
    // are accepted at the [Requires...] attributes below.
    static readonly JsonSerializerOptions Pretty = BuildOptions();

    [UnconditionalSuppressMessage("Trimming", "IL2026", Justification = "Writer is annotated; resolver pulled in deliberately.")]
    [UnconditionalSuppressMessage("AOT", "IL3050", Justification = "Writer is annotated; resolver pulled in deliberately.")]
    static JsonSerializerOptions BuildOptions() => new()
    {
        WriteIndented = true,
        PropertyNamingPolicy = null,
        DefaultIgnoreCondition = System.Text.Json.Serialization.JsonIgnoreCondition.WhenWritingNull,
        TypeInfoResolver = new DefaultJsonTypeInfoResolver(),
    };

    [RequiresUnreferencedCode("JSON serialization uses reflection over TypeInfo and its members.")]
    [RequiresDynamicCode("JSON serialization may construct generic types at runtime.")]
    public static Dictionary<string, string> RenderAll(CoverageIR ir)
    {
        var result = new Dictionary<string, string>();
        foreach (var t in ir.types)
        {
            var filename = $"{t.@namespace}.{t.name}.json";
            result[filename] = JsonSerializer.Serialize(t, Pretty);
        }
        return result;
    }
}
