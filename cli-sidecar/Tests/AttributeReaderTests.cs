using AwareReader;
using Xunit;

namespace AwareSidecar.Tests;

// #180 PR-A: custom-attribute extraction in the shared reader IR. Reflects the
// FixtureAssembly.Attributed.* types (built next to the test assembly) and asserts
// that type/method/field custom attributes decode with positional + named args,
// enum + typeof() values, and cross-assembly (BCL) attribute constructors.
public class AttributeReaderTests
{
    private static string FixturePath()
    {
        var here = Path.GetDirectoryName(typeof(AttributeReaderTests).Assembly.Location)!;
        return Path.Combine(here, "FixtureAssembly.dll");
    }

    private static TypeRecord Type(string fullName)
    {
        var asm = MetadataReflector.Reflect(FixturePath());
        var t = asm.Types.FirstOrDefault(t => t.FullName == fullName);
        Assert.True(t is not null, $"type {fullName} not reflected");
        return t!;
    }

    [Fact]
    public void TypeAttributesDecodedWithPositionalAndNamedArgs()
    {
        var t = Type("FixtureAssembly.Attributed.MarkedWidget");
        var marker = t.Attributes.FirstOrDefault(
            a => a.TypeName == "FixtureAssembly.Attributed.DemoMarkerAttribute");
        Assert.True(marker is not null, "DemoMarkerAttribute not decoded on MarkedWidget");
        Assert.Equal(new[] { "widget" }, marker!.FixedArguments.Select(a => a.Value));
        var order = marker.NamedArguments.FirstOrDefault(n => n.Name == "Order");
        Assert.True(order is not null, "Order named arg not decoded");
        Assert.Equal("3", order!.Value);
    }

    [Fact]
    public void FieldAttributesDecoded()
    {
        var t = Type("FixtureAssembly.Attributed.MarkedWidget");
        var size = t.Fields.First(f => f.Name == "Size");
        Assert.Contains(size.Attributes, a =>
            a.TypeName.EndsWith("DemoMarkerAttribute", StringComparison.Ordinal)
            && a.FixedArguments.Any(x => x.Value == "field-marker"));
    }

    [Fact]
    public void MethodAttributesDecodedIncludingEnumNamedArg()
    {
        var t = Type("FixtureAssembly.Attributed.MarkedWidget");
        var m = t.Methods.First(x => x.Name == "Configure");
        var marker = m.Attributes.First(
            a => a.TypeName.EndsWith("DemoMarkerAttribute", StringComparison.Ordinal));
        Assert.Equal("method-marker", marker.FixedArguments.Single().Value);
        Assert.Equal("7", marker.NamedArguments.First(n => n.Name == "Order").Value);
        // DemoKind.Beta == 1 — enum args decode to their integer literal.
        Assert.Equal("1", marker.NamedArguments.First(n => n.Name == "Kind").Value);
    }

    [Fact]
    public void ByteBackedEnumNamedArgDecodes()
    {
        // Regression guard: a byte-backed enum arg must resolve its real underlying type
        // (Byte), not a hardcoded Int32 — otherwise DecodeValue mis-reads the blob, throws,
        // and silently drops EVERY argument on the attribute. DemoByteKind.Big == 200.
        var t = Type("FixtureAssembly.Attributed.ByteMarked");
        var marker = t.Attributes.First(
            a => a.TypeName.EndsWith("DemoMarkerAttribute", StringComparison.Ordinal));
        Assert.Equal("bytey", marker.FixedArguments.Single().Value);
        Assert.Equal("200", marker.NamedArguments.First(n => n.Name == "ByteKind").Value);
    }

    [Fact]
    public void TypeofNamedArgDecodesToTypeName()
    {
        var t = Type("FixtureAssembly.Attributed.TypedMarker");
        var marker = t.Attributes.First(
            a => a.TypeName.EndsWith("DemoMarkerAttribute", StringComparison.Ordinal));
        var target = marker.NamedArguments.First(n => n.Name == "Target");
        Assert.Contains("MarkedWidget", target.Value ?? "");
    }

    [Fact]
    public void CrossAssemblyAttributeConstructorResolves()
    {
        // [System.ComponentModel.Description] — its ctor is a MemberReference pointing at
        // a BCL TypeReference, exercising the cross-assembly type-name resolution path.
        var t = Type("FixtureAssembly.Attributed.MarkedWidget");
        Assert.Contains(t.Attributes, a => a.TypeName == "System.ComponentModel.DescriptionAttribute");
    }

    [Fact]
    public void AttributesAreSortedByTypeNameForDeterminism()
    {
        var t = Type("FixtureAssembly.Attributed.MarkedWidget");
        var names = t.Attributes.Select(a => a.TypeName).ToList();
        Assert.Equal(names.OrderBy(n => n, StringComparer.Ordinal).ToList(), names);
    }
}
