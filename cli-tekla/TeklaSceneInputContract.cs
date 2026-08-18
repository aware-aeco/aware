using System;
using System.IO;

namespace AwareTekla;

/// <summary>
/// Host-agnostic validation shared by the bridge entry point and the canonical
/// Roslyn materializer. Keeping it here makes the production rules directly testable.
/// </summary>
public static class TeklaSceneInputContract
{
    public static bool SceneUpIsAbsentOrExactZ(bool present, object? value) =>
        !present || value is string text && string.Equals(text, "z", StringComparison.Ordinal);

    public static string CanonicalModelDirectoryPath(string path)
    {
        var fullPath = Path.GetFullPath(path);
        var root = Path.GetPathRoot(fullPath) ?? string.Empty;
        while (fullPath.Length > root.Length
            && (fullPath[fullPath.Length - 1] == Path.DirectorySeparatorChar
                || fullPath[fullPath.Length - 1] == Path.AltDirectorySeparatorChar))
        {
            fullPath = fullPath.Substring(0, fullPath.Length - 1);
        }
        return fullPath;
    }
}
