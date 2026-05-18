// CatalogWriter — serialize each TypeInfo in a CoverageIR into a single JSON file.
//
// One JSON file per type, keyed as "<namespace>.<typename>.json". The structured
// catalog is the queryable counterpart to the narrative skill markdown — it's what
// downstream tooling (the CLI, agent runners, evaluators) reads to discover the
// surface area of a host without parsing prose.
//
// Nullable fields with a null value are OMITTED from the output to keep the
// catalog compact (driven by JsonIgnoreCondition.WhenWritingNull on the source-
// generated IrJsonContext). The IR's deserializer treats absent fields as null
// by default, so this is a lossless space optimization.
//
// Uses the source-generated IrJsonContext: zero reflection, no trimmer hazard,
// fully NativeAOT-compatible. No `[Requires*]` attributes, no runtime
// `DefaultJsonTypeInfoResolver` — the serializer reads typed `JsonTypeInfo<T>`
// entries emitted at compile time.
//
// This writer is pure / stateless / synchronous; the caller is responsible
// for any I/O.

using System.Text.Json;

namespace AwareSidecar.Ingest;

public static class CatalogWriter
{
    public static Dictionary<string, string> RenderAll(CoverageIR ir)
    {
        var result = new Dictionary<string, string>();
        foreach (var t in ir.types)
        {
            var filename = $"{t.@namespace}.{t.name}.json";
            result[filename] = JsonSerializer.Serialize(t, IrJsonContext.Default.TypeInfo);
        }
        return result;
    }
}
