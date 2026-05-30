using System.Collections.Immutable;
using System.Globalization;
using System.Reflection.Metadata;

namespace AwareReader;

/// <summary>
/// One decoded custom attribute: the attribute type's full name plus its positional
/// (fixed) and named arguments. Argument values are rendered as literal strings (never
/// boxed <c>object</c>) so the IR stays AOT/source-gen-safe and deterministic.
/// </summary>
public sealed record AttributeRecord(
    string TypeName,
    IReadOnlyList<AttributeArg> FixedArguments,
    IReadOnlyList<NamedArg> NamedArguments);

/// <summary>A positional attribute argument: its type (xml-doc-id encoding) and literal value.</summary>
public sealed record AttributeArg(string Type, string? Value);

/// <summary>A named attribute argument (property/field set in the attribute usage).</summary>
public sealed record NamedArg(string Name, string Type, string? Value);

/// <summary>
/// Decodes custom attributes from PE metadata via <see cref="MetadataReader"/> — no
/// reflection on user types, so it is safe inside the NativeAOT sidecar. The attribute
/// type name is resolved from the constructor handle (the same constructor-walk
/// <c>DllReflector</c> used for <c>AssemblyDescriptionAttribute</c>); argument values are
/// decoded to literal strings. Failures are isolated per-attribute so one unreadable
/// attribute never aborts a whole type's reflection.
/// </summary>
internal static class AttributeReader
{
    public static IReadOnlyList<AttributeRecord> Read(
        MetadataReader mr,
        CustomAttributeHandleCollection handles,
        SigTypeProvider provider)
    {
        if (handles.Count == 0) return Array.Empty<AttributeRecord>();

        var result = new List<AttributeRecord>();
        foreach (var handle in handles)
        {
            var rec = TryRead(mr, handle, provider);
            if (rec is not null) result.Add(rec);
        }
        // Deterministic order: attributes carry no source ordering guarantee in metadata.
        // List.Sort is not stable, so tie-break duplicate type names (a member can legally
        // carry the same attribute type twice) on a rendered argument key — a total order.
        result.Sort(static (a, b) =>
        {
            var c = string.CompareOrdinal(a.TypeName, b.TypeName);
            return c != 0 ? c : string.CompareOrdinal(ArgKey(a), ArgKey(b));
        });
        return result;
    }

    // A collision-resistant, human-readable key over the rendered arguments. Count prefixes
    // make distinct argument lists impossible to alias by concatenation; remaining ties are
    // between effectively-identical attributes, where ordering is irrelevant.
    private static string ArgKey(AttributeRecord a) =>
        $"{a.FixedArguments.Count}:" + string.Join("|", a.FixedArguments.Select(x => x.Value))
        + $";{a.NamedArguments.Count}:" + string.Join("|", a.NamedArguments.Select(x => x.Name + "=" + x.Value));

    private static AttributeRecord? TryRead(
        MetadataReader mr, CustomAttributeHandle handle, SigTypeProvider provider)
    {
        try
        {
            var ca = mr.GetCustomAttribute(handle);
            var typeName = ResolveAttributeTypeName(mr, ca, provider);
            if (string.IsNullOrEmpty(typeName)) return null;

            var fixedArgs = new List<AttributeArg>();
            var namedArgs = new List<NamedArg>();
            try
            {
                var value = ca.DecodeValue(provider);
                foreach (var fa in value.FixedArguments)
                    fixedArgs.Add(new AttributeArg(fa.Type ?? "", FormatValue(fa.Value)));
                foreach (var na in value.NamedArguments)
                    namedArgs.Add(new NamedArg(na.Name ?? "", na.Type ?? "", FormatValue(na.Value)));
            }
            catch
            {
                // Argument blob references an unsupported shape (or the ctor signature
                // can't be resolved). Record the attribute's presence with no args rather
                // than dropping it or throwing.
            }

            return new AttributeRecord(typeName!, fixedArgs, namedArgs);
        }
        catch
        {
            return null; // unreadable attribute handle — skip
        }
    }

    private static string? ResolveAttributeTypeName(
        MetadataReader mr, CustomAttribute ca, SigTypeProvider provider)
    {
        switch (ca.Constructor.Kind)
        {
            case HandleKind.MemberReference:
            {
                var memberRef = mr.GetMemberReference((MemberReferenceHandle)ca.Constructor);
                return ResolveTypeName(mr, memberRef.Parent, provider);
            }
            case HandleKind.MethodDefinition:
            {
                var methodDef = mr.GetMethodDefinition((MethodDefinitionHandle)ca.Constructor);
                return provider.GetTypeFromDefinition(mr, methodDef.GetDeclaringType(), 0);
            }
            default:
                return null;
        }
    }

    private static string? ResolveTypeName(MetadataReader mr, EntityHandle handle, SigTypeProvider provider)
    {
        if (handle.IsNil) return null;
        return handle.Kind switch
        {
            HandleKind.TypeReference => provider.GetTypeFromReference(mr, (TypeReferenceHandle)handle, 0),
            HandleKind.TypeDefinition => provider.GetTypeFromDefinition(mr, (TypeDefinitionHandle)handle, 0),
            HandleKind.TypeSpecification => mr.GetTypeSpecification((TypeSpecificationHandle)handle)
                .DecodeSignature(provider, new GenericContext(Array.Empty<string>(), Array.Empty<string>())),
            _ => null,
        };
    }

    /// <summary>
    /// Renders a decoded attribute-argument value to a stable literal string. Enums arrive
    /// as their boxed underlying integer; <c>typeof()</c> values arrive as the provider's
    /// serialized type-name string; arrays arrive as an <see cref="ImmutableArray{T}"/> of
    /// typed arguments. Everything is formatted with the invariant culture.
    /// </summary>
    private static string? FormatValue(object? value)
    {
        switch (value)
        {
            case null: return null;
            case string s: return s;
            case bool b: return b ? "true" : "false";
            case char c: return c.ToString();
            case ImmutableArray<CustomAttributeTypedArgument<string>> arr:
                return "[" + string.Join(", ", arr.Select(e => FormatValue(e.Value))) + "]";
            default:
                return Convert.ToString(value, CultureInfo.InvariantCulture);
        }
    }
}
