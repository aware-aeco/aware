namespace FixtureAssembly.Demo
{

/// <summary>A demo class with a few public methods, used by sidecar reflection tests.</summary>
public class Greeter
{
    /// <summary>Returns a greeting for the given name.</summary>
    public string Greet(string name) => $"Hello, {name}";

    /// <summary>Counts the number of times a substring appears.</summary>
    public static int CountOccurrences(string haystack, string needle)
    {
        if (string.IsNullOrEmpty(needle)) return 0;
        int count = 0, idx = 0;
        while ((idx = haystack.IndexOf(needle, idx, StringComparison.Ordinal)) != -1)
        {
            count++;
            idx += needle.Length;
        }
        return count;
    }
}

/// <summary>Marker interface for objects that can be serialized.</summary>
public interface ISerializable
{
    /// <summary>Serializes the object to a string.</summary>
    string Serialize();
}

} // namespace FixtureAssembly.Demo

// ── Fixtures for the Navisworks-style coverage extractor ──────────────────────
//
// These types exist solely to exercise the MetadataReflector + XmlDocLoader +
// Navisworks PageParser / MemberPageParser pipeline. They mirror the Sandcastle
// XML doc patterns the Autodesk.Navisworks.Api ships, without depending on that
// (proprietary) assembly at test-time.

namespace FixtureAssembly.NavisworksLike
{
    /// <summary>Demo enumeration with integer-backed values and per-value docs.</summary>
    public enum AnimationBehavior
    {
        /// <summary>The animation duration is scaled to be same as task duration.</summary>
        Scale = 0,
        /// <summary>The animation starts when the task starts.</summary>
        Start = 1,
        /// <summary>The animation starts so that it ends when the task ends.</summary>
        End = 2,
    }

    /// <summary>The singleton object that represents the application.</summary>
    public class Application
    {
        private Application() { }

        /// <summary>Gets a reference to the current Document being displayed in a View.</summary>
        public string ActiveDocument { get; private set; } = "(none)";

        /// <summary>
        /// Provides an instance of the <see cref="DemoProgress"/> class which starts
        /// reporting of progress of an operation.
        /// </summary>
        /// <returns>A new progress bar instance.</returns>
        public DemoProgress BeginProgress() => new DemoProgress();

        /// <summary>
        /// Provides an instance of the <see cref="DemoProgress"/> class with a title.
        /// </summary>
        /// <param name="title">Title for the overall progress operation.</param>
        /// <returns>A new progress bar instance.</returns>
        public DemoProgress BeginProgress(string title) => new DemoProgress();

        /// <summary>
        /// Provides an instance of the <see cref="DemoProgress"/> class with a title and message.
        /// </summary>
        /// <param name="title">Title for the overall progress operation.</param>
        /// <param name="message">Message describing the current operation.</param>
        /// <returns>A new progress bar instance.</returns>
        /// <exception cref="System.ArgumentNullException">title is null.</exception>
        public DemoProgress BeginProgress(string title, string message) => new DemoProgress();

        /// <summary>Occurs when the active document has changed.</summary>
#pragma warning disable CS0067  // Event used by reflection tests, not raised in fixture.
        public event System.EventHandler? ActiveDocumentChanged;
#pragma warning restore CS0067
    }

    /// <summary>Reports progress of an operation.</summary>
    public class DemoProgress : System.IDisposable
    {
        /// <summary>Initializes a new DemoProgress with no title.</summary>
        public DemoProgress() { }

        /// <summary>Initializes a new DemoProgress with the given title.</summary>
        /// <param name="title">Title shown to the user.</param>
        public DemoProgress(string title) { }

        /// <summary>Releases any resources held by the progress bar.</summary>
        public void Dispose() { }
    }
}

// ── Fixtures for #180 custom-attribute decoding ───────────────────────────────
//
// Exercises type / method / field custom-attribute extraction: positional + named
// arguments, enum- and typeof()-valued arguments, and a cross-assembly (BCL)
// attribute whose constructor is a MemberReference rather than a MethodDefinition.

namespace FixtureAssembly.Attributed
{
    /// <summary>Demo enum used as a named attribute-argument value.</summary>
    public enum DemoKind { Alpha = 0, Beta = 1 }

    /// <summary>Byte-backed enum — exercises non-Int32 underlying-type resolution (#180).</summary>
    public enum DemoByteKind : byte { Small = 1, Big = 200 }

    /// <summary>Marker attribute used to verify positional + named argument decoding.</summary>
    [System.AttributeUsage(System.AttributeTargets.All)]
    public sealed class DemoMarkerAttribute : System.Attribute
    {
        /// <summary>Creates a marker with the given positional name.</summary>
        public DemoMarkerAttribute(string name) { Name = name; }
        /// <summary>The positional name.</summary>
        public string Name { get; }
        /// <summary>Named integer argument.</summary>
        public int Order { get; set; }
        /// <summary>Named enum argument.</summary>
        public DemoKind Kind { get; set; }
        /// <summary>Named byte-backed enum argument.</summary>
        public DemoByteKind ByteKind { get; set; }
        /// <summary>Named typeof() argument.</summary>
        public System.Type? Target { get; set; }
    }

    /// <summary>A demo type whose type/method/field carry custom attributes.</summary>
    [DemoMarker("widget", Order = 3)]
    [System.ComponentModel.Description("a marked widget")]
    public class MarkedWidget
    {
        /// <summary>A field carrying a field-level attribute.</summary>
        [DemoMarker("field-marker")] public double Size;

        /// <summary>A method carrying a method-level attribute with an enum named arg.</summary>
        [DemoMarker("method-marker", Order = 7, Kind = DemoKind.Beta)]
        public void Configure(int value) { }
    }

    /// <summary>Exercises a typeof()-valued named attribute argument.</summary>
    [DemoMarker("typed", Target = typeof(MarkedWidget))]
    public class TypedMarker { }

    /// <summary>Exercises a byte-backed enum named attribute argument (#180 review).</summary>
    [DemoMarker("bytey", ByteKind = DemoByteKind.Big)]
    public class ByteMarked { }
}

// ── Tekla-style plug-in fixture for #180 (multi-assembly + recipe) ─────────────
//
// Models the issue's cross-assembly split: this plug-in derives from PluginBase and
// takes its StructuresData contract (DemoData) as a constructor parameter — but both
// PluginBase and DemoData live in a DIFFERENT assembly (FixtureDataAssembly). Resolving
// the data type therefore requires the cross-assembly TypeIndex from MetadataReflector.ReflectSet.

namespace FixtureAssembly.TeklaLike
{
    /// <summary>A model plug-in registered as "Demo Plugin"; its data contract is DemoData.</summary>
    [Tekla.Structures.Plugins.Plugin("Demo Plugin")]
    public class DemoPlugin : Tekla.Structures.Plugins.PluginBase
    {
        /// <summary>The classic Tekla pattern: the plug-in takes its StructuresData via ctor.</summary>
        public DemoPlugin(FixtureData.DemoData data) { Data = data; }

        /// <summary>The bound data contract.</summary>
        public FixtureData.DemoData Data { get; }

        /// <summary>Framework callback — not a useful end-user command on its own.</summary>
        public override bool Run(System.Collections.Generic.List<object> input) => true;
    }

    /// <summary>Derives from PluginBase but lacks [Plugin] — must NOT be recognized as a plug-in.</summary>
    public class DerivedNoAttr : Tekla.Structures.Plugins.PluginBase
    {
        public override bool Run(System.Collections.Generic.List<object> input) => true;
    }

    /// <summary>Carries [Plugin] but does NOT derive from PluginBase — must NOT be recognized.</summary>
    [Tekla.Structures.Plugins.Plugin("Orphan")]
    public class OrphanPlugin
    {
        public void DoThing() { }
    }

    /// <summary>An in-assembly intermediate base, to exercise the base-chain walk via the index.</summary>
    public abstract class MidBase : Tekla.Structures.Plugins.PluginBase { }

    /// <summary>Reaches PluginBase through MidBase — recognized via the base-chain walk.</summary>
    [Tekla.Structures.Plugins.Plugin("Intermediate Plugin")]
    public class IntermediatePlugin : MidBase
    {
        public override bool Run(System.Collections.Generic.List<object> input) => true;
    }
}
