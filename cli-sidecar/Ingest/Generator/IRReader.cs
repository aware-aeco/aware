// IRReader — parse a host-coverage IR JSON file into the typed records in Models.cs.
//
// Uses the source-generated IrJsonContext: zero reflection, no trimmer hazard,
// fully NativeAOT-compatible. The serializer reads its metadata from typed
// `JsonTypeInfo<T>` entries emitted at compile time, so neither `[Requires*]`
// attributes nor a runtime `DefaultJsonTypeInfoResolver` are needed.

using System.Text.Json;

namespace AwareSidecar.Ingest;

public static class IRReader
{
    public static CoverageIR ReadFromFile(string path)
    {
        var json = File.ReadAllText(path);
        var ir = JsonSerializer.Deserialize(json, IrJsonContext.Default.CoverageIR)
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
