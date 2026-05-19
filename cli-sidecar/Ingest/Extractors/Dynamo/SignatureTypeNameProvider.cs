// SignatureTypeNameProvider — turns System.Reflection.Metadata signature blobs into
// C#-style type-name strings.
//
// Why this exists:
// ----------------
// PEReader+MetadataReader give us metadata tokens (TypeDefinitionHandle,
// TypeReferenceHandle, TypeSpecificationHandle) but the binary signature blobs for
// methods, properties, and fields are encoded sequences of ELEMENT_TYPE_* opcodes
// — they need a "type provider" to decode each opcode into a presentation form.
//
// We implement ISignatureTypeProvider<string, GenericContext> producing C#-style
// strings: "System.Int32", "System.Collections.Generic.List<System.String>",
// "Dynamo.Graph.Workspaces.WorkspaceModel[]", etc. This matches the form the
// host-coverage IR expects in ParamInfo.type / PropertyInfo.type / ReturnInfo.type.
//
// Generic context (TGenericContext = GenericContext) carries the open type's
// generic parameter list so a parameter blob referring to `!T` resolves to the
// type's `T` (rather than a numeric placeholder). For methods that themselves
// introduce a generic parameter, `!!U` resolves to the method's `U`. Both are
// kept as `T`/`U` strings — open generics stay symbolic and the consumer can
// substitute when it instantiates.
//
// Primitive opcodes use the canonical short C# alias (`int`, `string`, `bool`, …)
// where present; non-aliased primitives keep the .NET name (`IntPtr`, `UIntPtr`,
// `TypedReference`).
//
// Array shapes:
//   - SZ-arrays (single-dim, zero-based — the most common) → `T[]`
//   - Multi-dim arrays → `T[,]` etc. (we render rank as commas)
//
// Pointer / pinned / by-ref:
//   - T* → `T*`
//   - ref T → `ref T`
//   - pinned T → `pinned T`     (rare; only in IL bodies — kept for completeness)
//
// NOTE — System.Reflection.Metadata's GetSZArrayType / GetArrayType are SEPARATE
// hooks; the decoder dispatches between them on the wire format. Implement both.

using System.Collections.Immutable;
using System.Reflection.Metadata;

namespace AwareSidecar.Ingest.Extractors.Dynamo;

/// <summary>
/// Generic-context bag used while decoding a method/field/property signature blob.
/// Type-level generic params live in <see cref="TypeGenericParameters"/>; method-
/// level generic params (only valid inside a MethodDefinition signature) live in
/// <see cref="MethodGenericParameters"/>. Either may be empty when the surrounding
/// type/method is non-generic.
/// </summary>
public readonly struct GenericContext
{
    public ImmutableArray<string> TypeGenericParameters { get; }
    public ImmutableArray<string> MethodGenericParameters { get; }

    public GenericContext(ImmutableArray<string> typeParams, ImmutableArray<string> methodParams)
    {
        TypeGenericParameters = typeParams.IsDefault ? ImmutableArray<string>.Empty : typeParams;
        MethodGenericParameters = methodParams.IsDefault ? ImmutableArray<string>.Empty : methodParams;
    }

    public static GenericContext Empty => new(ImmutableArray<string>.Empty, ImmutableArray<string>.Empty);
}

internal sealed class SignatureTypeNameProvider : ISignatureTypeProvider<string, GenericContext>
{
    public static SignatureTypeNameProvider Instance { get; } = new();

    public string GetPrimitiveType(PrimitiveTypeCode typeCode) => typeCode switch
    {
        PrimitiveTypeCode.Boolean       => "bool",
        PrimitiveTypeCode.Byte          => "byte",
        PrimitiveTypeCode.Char          => "char",
        PrimitiveTypeCode.Double        => "double",
        PrimitiveTypeCode.Int16         => "short",
        PrimitiveTypeCode.Int32         => "int",
        PrimitiveTypeCode.Int64         => "long",
        PrimitiveTypeCode.IntPtr        => "IntPtr",
        PrimitiveTypeCode.Object        => "object",
        PrimitiveTypeCode.SByte         => "sbyte",
        PrimitiveTypeCode.Single        => "float",
        PrimitiveTypeCode.String        => "string",
        PrimitiveTypeCode.TypedReference=> "TypedReference",
        PrimitiveTypeCode.UInt16        => "ushort",
        PrimitiveTypeCode.UInt32        => "uint",
        PrimitiveTypeCode.UInt64        => "ulong",
        PrimitiveTypeCode.UIntPtr       => "UIntPtr",
        PrimitiveTypeCode.Void          => "void",
        _ => "object"
    };

    public string GetTypeFromDefinition(MetadataReader reader, TypeDefinitionHandle handle, byte rawTypeKind)
    {
        var td = reader.GetTypeDefinition(handle);
        var name = reader.GetString(td.Name);
        var ns = td.Namespace.IsNil ? "" : reader.GetString(td.Namespace);
        if (td.IsNested)
        {
            // Nested types: emit `Outer.Inner`. Recurse up the chain.
            var parent = GetTypeFromDefinition(reader, td.GetDeclaringType(), rawTypeKind);
            return parent + "." + CleanGenericArity(name);
        }
        return string.IsNullOrEmpty(ns) ? CleanGenericArity(name) : ns + "." + CleanGenericArity(name);
    }

    public string GetTypeFromReference(MetadataReader reader, TypeReferenceHandle handle, byte rawTypeKind)
    {
        var tr = reader.GetTypeReference(handle);
        var name = reader.GetString(tr.Name);
        var ns = tr.Namespace.IsNil ? "" : reader.GetString(tr.Namespace);
        // Resolution scope for nested type references comes through as another TypeReference;
        // for non-nested types the scope is an AssemblyReference (ignored here — type-presentation
        // strings don't carry assembly identity, matching how DocFX / Sandcastle render them).
        if (tr.ResolutionScope.Kind == HandleKind.TypeReference)
        {
            var parent = GetTypeFromReference(reader, (TypeReferenceHandle)tr.ResolutionScope, rawTypeKind);
            return parent + "." + CleanGenericArity(name);
        }
        return string.IsNullOrEmpty(ns) ? CleanGenericArity(name) : ns + "." + CleanGenericArity(name);
    }

    public string GetTypeFromSpecification(MetadataReader reader, GenericContext genericContext, TypeSpecificationHandle handle, byte rawTypeKind)
    {
        var ts = reader.GetTypeSpecification(handle);
        return ts.DecodeSignature(this, genericContext);
    }

    public string GetSZArrayType(string elementType) => elementType + "[]";

    public string GetPointerType(string elementType) => elementType + "*";

    public string GetByReferenceType(string elementType) => "ref " + elementType;

    public string GetGenericMethodParameter(GenericContext genericContext, int index)
    {
        if (index >= 0 && index < genericContext.MethodGenericParameters.Length)
            return genericContext.MethodGenericParameters[index];
        return "!!" + index;
    }

    public string GetGenericTypeParameter(GenericContext genericContext, int index)
    {
        if (index >= 0 && index < genericContext.TypeGenericParameters.Length)
            return genericContext.TypeGenericParameters[index];
        return "!" + index;
    }

    public string GetGenericInstantiation(string genericType, ImmutableArray<string> typeArguments)
    {
        // genericType arrived already cleaned of arity (`List` not `List`1`). Concat C#-style.
        return $"{genericType}<{string.Join(", ", typeArguments)}>";
    }

    public string GetArrayType(string elementType, ArrayShape shape)
    {
        // Multi-dim array: `T[,]`, `T[,,]`, …. We don't render lower-bounds because
        // they're rare and would clutter type strings disproportionately.
        if (shape.Rank == 1) return elementType + "[]";
        return elementType + "[" + new string(',', shape.Rank - 1) + "]";
    }

    public string GetModifiedType(string modifier, string unmodifiedType, bool isRequired) =>
        // Modifiers like `[modreq(System.Runtime.CompilerServices.IsExternalInit)]` aren't useful in IR.
        // Drop them; keep the underlying type.
        unmodifiedType;

    public string GetPinnedType(string elementType) => "pinned " + elementType;

    public string GetFunctionPointerType(MethodSignature<string> signature) =>
        // Function pointers are exceedingly rare in vendor public APIs. Render generically.
        $"delegate*<{string.Join(", ", signature.ParameterTypes)}, {signature.ReturnType}>";

    public string GetTypeFromSerializedName(string name)
    {
        // Used for custom-attribute decoding paths only — we don't enter those for member signatures.
        return name;
    }

    public PrimitiveTypeCode GetUnderlyingEnumType(string type) => PrimitiveTypeCode.Int32;

    public bool IsSystemType(string type) => type == "System.Type";

    /// <summary>
    /// Convert a metadata type name like "Dictionary`2" into "Dictionary". The arity is recovered
    /// later through GetGenericInstantiation when the type argument blob is decoded.
    /// </summary>
    static string CleanGenericArity(string name)
    {
        var tick = name.IndexOf('`');
        return tick < 0 ? name : name.Substring(0, tick);
    }
}
