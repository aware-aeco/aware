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
// Filename sanitization: vendor type names can contain characters that Windows
// (and POSIX, in NTFS-on-Linux scenarios) rejects in filenames — most notably
// the generic-parameter angle brackets in names like `Enum<E>` and
// `CustomObservableCollection<T>`. We rewrite `<` → `_of_` and `>` → `` so
// the filename is portable on every filesystem. The JSON body inside the file
// preserves the real type name verbatim — only the filename is sanitized.
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
            var safeName = SanitizeForFilename(t.name);
            var safeNamespace = SanitizeNamespaceForFilename(t.@namespace);
            // Top-level types (Ruby's root namespace, e.g. `Array`, `LanguageHandler`) come
            // through with empty namespace — emit the bare type name to avoid a leading dot.
            var filename = string.IsNullOrEmpty(safeNamespace)
                ? $"{safeName}.json"
                : $"{safeNamespace}.{safeName}.json";
            result[filename] = JsonSerializer.Serialize(t, IrJsonContext.Default.TypeInfo);
        }
        return result;
    }

    /// <summary>
    /// Replace Ruby's `::` namespace separator with `.` so the filename layout is identical to
    /// what .NET vendor agents produce. The catalog JSON inside the file keeps the original
    /// namespace verbatim (with `::`), so downstream tools see the true vendor name. Only the
    /// filename is normalised.
    /// </summary>
    static string SanitizeNamespaceForFilename(string @namespace) =>
        (@namespace ?? "").Replace("::", ".", StringComparison.Ordinal);

    /// <summary>
    /// Rewrite a vendor type name into a filesystem-safe variant for use as a catalog
    /// filename. Replaces the Windows-forbidden + POSIX-fragile characters that can
    /// appear in generic-parameter type names (`Foo&lt;T&gt;`) and a handful of
    /// other shapes vendor docs may emit. The JSON body inside the catalog still
    /// carries the real type name; only the filename is sanitized.
    ///
    /// Substitutions (deliberate, readable — not URL-encoded):
    ///   `&lt;` → `_of_`   so `Enum&lt;E&gt;` becomes `Enum_of_E`
    ///   `&gt;` → ``       (the close-bracket terminator is dropped — `Enum_of_E`
    ///                     is unambiguous; trailing markers add noise)
    ///   `,`  → `__`       multi-arg generics become `Dict_of_K__V`
    ///   ` `  → `_`        (defensive — type names normally have no spaces)
    /// </summary>
    static string SanitizeForFilename(string name) =>
        name.Replace("<", "_of_", StringComparison.Ordinal)
            .Replace(">", "", StringComparison.Ordinal)
            .Replace(", ", "__", StringComparison.Ordinal)
            .Replace(",", "__", StringComparison.Ordinal)
            .Replace(" ", "_", StringComparison.Ordinal);
}
