using System.Reflection;

namespace AwareRhino;

internal static class BakeSceneScript
{
    internal const string ResourceName = "AwareRhino.BakeSceneScript.py";

    static readonly Lazy<string> Source = new(Load);

    internal static string Code => Source.Value;

    static string Load()
    {
        var assembly = typeof(BakeSceneScript).Assembly;
        using var stream = assembly.GetManifestResourceStream(ResourceName)
            ?? throw new InvalidOperationException(
                $"embedded Rhino bake-scene materializer '{ResourceName}' is missing");
        using var reader = new StreamReader(stream);
        // Normalize to LF. Without a .gitattributes rule this file is checked out
        // CRLF on Windows (core.autocrlf=true is the default there) and LF
        // everywhere else, so the embedded resource — and every source assertion
        // pinned against it — would otherwise depend on the contributor's git
        // config. ScriptWrapper normalizes the body again before it reaches
        // Rhino, so this only makes the bytes deterministic, never different.
        return reader.ReadToEnd().Replace("\r\n", "\n").Replace("\r", "\n");
    }
}
