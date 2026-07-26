// Loader for the canonical scene-to-SketchUp materializer.
//
// The Ruby lives beside this file as BakeSceneScript.rb and is compiled into
// the sidecar as an embedded resource, then INJECTED through the bridge's
// existing `exec` request — the same shape cli-tekla uses for its Roslyn
// BakeSceneScript.Code.
//
// Why injected rather than shipped in BridgeAssets/: the bridge in
// BridgeAssets/ is copied into the user's Plugins folder and only reloads when
// SketchUp restarts. A materializer living there would let a freshly updated
// sidecar drive a months-old bake script with no way to notice — the worst
// failure mode for a verb whose job is to erase and replace geometry. Injecting
// it keeps script and sidecar in lockstep by construction. It stays a real .rb
// file (rather than a C# string literal) so it can be read, diffed and
// `ruby -c`-checked as Ruby.

using System.Reflection;

namespace AwareSketchup;

internal static class BakeSceneScript
{
    internal const string ResourceName = "AwareSketchup.BakeSceneScript.rb";

    static readonly Lazy<string> Lazy = new(Load);

    /// <summary>The Ruby materializer, verbatim.</summary>
    internal static string Code => Lazy.Value;

    static string Load()
    {
        var assembly = typeof(BakeSceneScript).Assembly;
        using var stream = assembly.GetManifestResourceStream(ResourceName)
            ?? throw new InvalidOperationException(
                $"embedded bake-scene materializer '{ResourceName}' is missing from {assembly.GetName().Name}; "
                + "check the EmbeddedResource item in cli-sketchup.csproj");
        using var reader = new StreamReader(stream, System.Text.Encoding.UTF8);
        return reader.ReadToEnd();
    }
}
