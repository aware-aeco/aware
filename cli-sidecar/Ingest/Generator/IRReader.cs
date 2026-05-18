// IRReader — parse a host-coverage IR JSON file into the typed records in Models.cs.
//
// Uses System.Text.Json with reflection-based deserialization. This is *not* fully
// NativeAOT-compatible: under AOT, `JsonSerializer.Deserialize<T>` falls back to
// reflection metadata that the trimmer may have removed. The main sidecar binary
// is published as NativeAOT, so this code is marked with `[RequiresUnreferencedCode]`
// / `[RequiresDynamicCode]` to surface that constraint to any caller.
//
// For v0.30 A2 the IR reader is only invoked from the (managed, non-AOT) test
// harness, so the AOT warnings don't fire in practice. A6 will wire this into
// the sidecar's `coverage-generate` verb, at which point we either: (i) add a
// JsonSerializerContext source generator, or (ii) ensure the reader runs only
// from a managed entry point. Either choice is local to that task — the API
// here is stable.

using System.Diagnostics.CodeAnalysis;
using System.Text.Json;
using System.Text.Json.Serialization.Metadata;

namespace AwareSidecar.Ingest;

public static class IRReader
{
    // When the host project sets PublishAot=true, the SDK flips the global
    // `JsonSerializerIsReflectionEnabledByDefault` switch off (even in Debug
    // builds), which causes plain `Deserialize<T>` calls to throw at runtime.
    // We explicitly attach a `DefaultJsonTypeInfoResolver` so this reader keeps
    // working both in unit tests (managed) and from the NativeAOT entry point
    // (Program.cs `coverage-generate` op). The trim/AOT warnings remain — see
    // [RequiresUnreferencedCode] / [RequiresDynamicCode] below.
    static readonly JsonSerializerOptions Options = BuildOptions();

    [UnconditionalSuppressMessage("Trimming", "IL2026", Justification = "Reader is annotated; resolver pulled in deliberately.")]
    [UnconditionalSuppressMessage("AOT", "IL3050", Justification = "Reader is annotated; resolver pulled in deliberately.")]
    static JsonSerializerOptions BuildOptions() => new()
    {
        // IR uses snake_case verbatim — record property names already match the
        // wire format (e.g. `host_version`, `extracted_at`), so no naming policy.
        PropertyNamingPolicy = null,
        ReadCommentHandling = JsonCommentHandling.Skip,
        AllowTrailingCommas = true,
        TypeInfoResolver = new DefaultJsonTypeInfoResolver(),
    };

    [RequiresUnreferencedCode("JSON deserialization uses reflection over CoverageIR and its members.")]
    [RequiresDynamicCode("JSON deserialization may construct generic types at runtime.")]
    public static CoverageIR ReadFromFile(string path)
    {
        var json = File.ReadAllText(path);
        var ir = JsonSerializer.Deserialize<CoverageIR>(json, Options)
            ?? throw new InvalidDataException($"failed to parse IR at {path}");
        Validate(ir, path);
        return ir;
    }

    static void Validate(CoverageIR ir, string sourcePath)
    {
        if (string.IsNullOrEmpty(ir.host))
            throw new InvalidDataException($"{sourcePath}: missing host");
        if (string.IsNullOrEmpty(ir.host_version))
            throw new InvalidDataException($"{sourcePath}: missing host_version");
        if (ir.types == null)
            throw new InvalidDataException($"{sourcePath}: missing types array");
        foreach (var t in ir.types)
        {
            if (string.IsNullOrEmpty(t.name))
                throw new InvalidDataException($"{sourcePath}: type missing name");
            if (string.IsNullOrEmpty(t.summary))
                throw new InvalidDataException($"{sourcePath}: type {t.name} missing summary");
            if (string.IsNullOrEmpty(t.doc_url))
                throw new InvalidDataException($"{sourcePath}: type {t.name} missing doc_url");

            // Defensive: ensure all schema-required collection fields are present (non-null).
            // The schema enforces this at validation time but ReadFromFile is sometimes invoked
            // without prior schema validation, so we belt-and-suspenders here.
            if (t.interfaces == null)
                throw new InvalidDataException($"{sourcePath}: type {t.name} has null interfaces (use [])");
            if (t.constructors == null)
                throw new InvalidDataException($"{sourcePath}: type {t.name} has null constructors (use [])");
            if (t.methods == null)
                throw new InvalidDataException($"{sourcePath}: type {t.name} has null methods (use [])");
            if (t.properties == null)
                throw new InvalidDataException($"{sourcePath}: type {t.name} has null properties (use [])");
            if (t.events == null)
                throw new InvalidDataException($"{sourcePath}: type {t.name} has null events (use [])");
            if (t.enum_values == null)
                throw new InvalidDataException($"{sourcePath}: type {t.name} has null enum_values (use [])");

            foreach (var m in t.methods)
            {
                if (m.@params == null)
                    throw new InvalidDataException($"{sourcePath}: method {t.name}.{m.name} has null params (use [])");
                if (m.throws == null)
                    throw new InvalidDataException($"{sourcePath}: method {t.name}.{m.name} has null throws (use [])");
                if (m.events_related == null)
                    throw new InvalidDataException($"{sourcePath}: method {t.name}.{m.name} has null events_related (use [])");
            }
        }
    }
}
