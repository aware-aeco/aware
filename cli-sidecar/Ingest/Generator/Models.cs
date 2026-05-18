// Data records for the host-coverage IR (Intermediate Representation).
//
// These types mirror the JSON Schema at cli-sidecar/Ingest/Schema/host-coverage.schema.json
// 1:1. Field names use snake_case to match the IR verbatim (no JSON property-naming
// policy is applied during deserialization — see IRReader).
//
// IR fields that are required in the schema map to non-nullable record properties.
// IR fields that are optional / nullable in the schema map to nullable record
// properties. Collection-valued fields are typed as `List<T>` and are expected to
// always be present in well-formed IR (the IR producer is responsible for emitting
// empty arrays rather than omitting them; the schema enforces this).
//
// NOTE: identifiers prefixed with `@` (e.g. `@namespace`, `@params`) are C#
// keywords escaped to use as record property names. The verbatim identifier
// preserves the IR field name (`namespace`, `params`, …) for the default
// reflection-based JSON serializer.

namespace AwareSidecar.Ingest;

public record CoverageIR(
    string host,
    string host_version,
    SourceInfo source,
    List<TypeInfo> types,
    MetadataInfo metadata);

public record SourceInfo(
    string kind,
    List<string> urls,
    DateTime extracted_at);

public record MetadataInfo(
    int page_count,
    int type_count,
    int method_count,
    int event_count,
    int property_count);

public record TypeInfo(
    string name,
    string @namespace,
    string kind,
    string summary,
    string? remarks,
    string? @base,
    List<string> interfaces,
    string doc_url,
    List<MethodInfo> constructors,
    List<MethodInfo> methods,
    List<PropertyInfo> properties,
    List<EventInfo> events,
    List<EnumValueInfo> enum_values,
    string? delegate_signature);

public record MethodInfo(
    string name,
    string signature,
    string summary,
    string? remarks,
    List<ParamInfo> @params,
    ReturnInfo? returns,
    List<ThrowsInfo> throws,
    List<string> events_related,
    string doc_url,
    string? since,
    string? deprecated);

public record PropertyInfo(
    string name,
    string type,
    bool getter,
    bool setter,
    string summary,
    string? remarks,
    string doc_url);

public record EventInfo(
    string name,
    string @delegate,
    string signature,
    string summary,
    string? fires_when,
    string doc_url,
    string handler_thread,
    List<string> interacts_with);

public record EnumValueInfo(string name, object value, string? summary);

public record ParamInfo(
    string name,
    string type,
    string? doc,
    bool optional = false,
    string? @default = null);

public record ReturnInfo(string type, string doc);

public record ThrowsInfo(string type, string when);
