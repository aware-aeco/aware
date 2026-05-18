// IrJsonContext — source-generated JsonSerializerContext for the host-coverage IR
// model graph.
//
// The sidecar publishes as NativeAOT (`<PublishAot>true</PublishAot>`), which
// globally disables `JsonSerializerIsReflectionEnabledByDefault`. Reflection-
// based `Deserialize<T>` / `Serialize(object)` calls would either throw at
// runtime or have their metadata trimmed away by the ILC. The source generator
// emits typed `JsonTypeInfo<T>` entries for every type registered below, which
// the serializer uses directly — no reflection at runtime, no trim warnings,
// no AOT warnings.
//
// IR fields use snake_case verbatim (record property names already match the
// wire format, e.g. `host_version`, `extracted_at`), so no naming policy is
// applied. `WriteIndented = true` is fine for both reading (whitespace is
// ignored during deserialization) and writing (catalog files want pretty-
// printed output).
//
// Kept separate from `SidecarJsonContext` (which lives in Program.cs and covers
// the stdin/stdout protocol envelopes) because the protocol contexts use
// compact serialization (no indenting) for line-oriented JSON, while the IR
// context wants indented, null-omitting output for on-disk catalog files.

using System.Text.Json;
using System.Text.Json.Serialization;

namespace AwareSidecar.Ingest;

[JsonSourceGenerationOptions(
    PropertyNamingPolicy = JsonKnownNamingPolicy.Unspecified,
    WriteIndented = true,
    DefaultIgnoreCondition = JsonIgnoreCondition.WhenWritingNull,
    ReadCommentHandling = JsonCommentHandling.Skip,
    AllowTrailingCommas = true)]
[JsonSerializable(typeof(CoverageIR))]
[JsonSerializable(typeof(TypeInfo))]
[JsonSerializable(typeof(MethodInfo))]
[JsonSerializable(typeof(PropertyInfo))]
[JsonSerializable(typeof(EventInfo))]
[JsonSerializable(typeof(EnumValueInfo))]
[JsonSerializable(typeof(ParamInfo))]
[JsonSerializable(typeof(ReturnInfo))]
[JsonSerializable(typeof(ThrowsInfo))]
[JsonSerializable(typeof(SourceInfo))]
[JsonSerializable(typeof(MetadataInfo))]
[JsonSerializable(typeof(List<TypeInfo>))]
[JsonSerializable(typeof(List<MethodInfo>))]
[JsonSerializable(typeof(List<PropertyInfo>))]
[JsonSerializable(typeof(List<EventInfo>))]
[JsonSerializable(typeof(List<EnumValueInfo>))]
[JsonSerializable(typeof(List<ParamInfo>))]
[JsonSerializable(typeof(List<ThrowsInfo>))]
[JsonSerializable(typeof(List<string>))]
[JsonSerializable(typeof(JsonElement))]
// Tekla extractor (Phase B B1) emits enum values as either int or string into JsonElement
// via JsonSerializer.SerializeToElement(value, IrJsonContext.Default.<Type>). Registering
// the primitive metadata here keeps that path source-gen / AOT clean — no reflection
// fallback at runtime.
[JsonSerializable(typeof(int))]
[JsonSerializable(typeof(string))]
public partial class IrJsonContext : JsonSerializerContext
{
}
