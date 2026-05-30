// Fake Tekla.Structures.Plugins surface for #180 tests — mirrors the real type names
// (PluginBase, PluginAttribute, StructuresFieldAttribute) and the StructuresData shape
// (public double/int/string fields tagged [StructuresField("attr")]) without depending
// on the proprietary Tekla assemblies. The plugin class that consumes these lives in a
// DIFFERENT assembly (FixtureAssembly) to model the cross-assembly split from the issue.

namespace Tekla.Structures.Plugins
{
    /// <summary>Abstract base class for model plug-ins (fake; mirrors the real PluginBase).</summary>
    public abstract class PluginBase
    {
        /// <summary>The main plug-in entry point.</summary>
        public abstract bool Run(System.Collections.Generic.List<object> input);
    }

    /// <summary>Stores the registered name of the plug-in. Real: [Plugin("Name")].</summary>
    [System.AttributeUsage(System.AttributeTargets.Class)]
    public sealed class PluginAttribute : System.Attribute
    {
        public PluginAttribute(string name) { Name = name; }
        /// <summary>The name of the plug-in in the system.</summary>
        public string Name { get; }
    }

    /// <summary>Maps a database attribute to a plug-in data field. Real: [StructuresField("attr")].</summary>
    [System.AttributeUsage(System.AttributeTargets.Field)]
    public sealed class StructuresFieldAttribute : System.Attribute
    {
        public StructuresFieldAttribute(string attributeName) { AttributeName = attributeName; }
        /// <summary>The name of the attribute.</summary>
        public string AttributeName { get; }
    }
}

namespace FixtureData
{
    /// <summary>
    /// A StructuresData contract: public double/int/string fields tagged with
    /// [StructuresField]. Lives in a different assembly from the plug-in that consumes it,
    /// so resolving it from the plug-in's constructor parameter type requires the
    /// cross-assembly TypeIndex.
    /// </summary>
    public class DemoData
    {
        /// <summary>A double-valued structures field.</summary>
        [Tekla.Structures.Plugins.StructuresField("Length")] public double Length;
        /// <summary>A string-valued structures field.</summary>
        [Tekla.Structures.Plugins.StructuresField("Name")] public string Name = "";
        /// <summary>An int-valued structures field.</summary>
        [Tekla.Structures.Plugins.StructuresField("Count")] public int Count;
    }
}
