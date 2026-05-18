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

namespace AwareSidecar.Ingest;

public static class IRReader
{
    static readonly JsonSerializerOptions Options = new()
    {
        // IR uses snake_case verbatim — record property names already match the
        // wire format (e.g. `host_version`, `extracted_at`), so no naming policy.
        PropertyNamingPolicy = null,
        ReadCommentHandling = JsonCommentHandling.Skip,
        AllowTrailingCommas = true,
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
        }
    }
}
