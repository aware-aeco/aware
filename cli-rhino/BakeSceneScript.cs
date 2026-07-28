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
        return reader.ReadToEnd();
    }
}
