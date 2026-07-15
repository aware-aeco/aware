using System;
using System.IO;
using System.Linq;
using Xunit;

namespace AwareTekla.Tests;

public class ResolveTeklaReferencesTests
{
    [Fact]
    public void DiscoversManagedTeklaStructuresAssembliesAcrossProbePaths()
    {
        var hostInstall = Path.Combine(Path.GetTempPath(), $"aware-tekla-{Guid.NewGuid():N}");
        var bin = Path.Combine(hostInstall, "bin");
        var net48Runtime = Path.Combine(hostInstall, "bin", "Net48Runtime");
        Directory.CreateDirectory(net48Runtime);

        try
        {
            var managedAssembly = typeof(Program).Assembly.Location;
            File.Copy(managedAssembly, Path.Combine(net48Runtime, "Tekla.Structures.dll"));
            File.Copy(managedAssembly, Path.Combine(net48Runtime, "Tekla.Structures.Plugins.dll"));
            File.Copy(managedAssembly, Path.Combine(bin, "Tekla.Structures.dll"));
            File.Copy(managedAssembly, Path.Combine(bin, "Tekla.Structures.Analysis.dll"));
            File.Copy(managedAssembly, Path.Combine(bin, "Tekla.Structures.Catalogs.dll"));
            File.WriteAllText(Path.Combine(bin, "Tekla.Structures.Native.DbvDatabase.dll"), "native");

            var (references, _) = Program.ResolveTeklaReferences(hostInstall);
            var paths = references.Select(reference => reference.Display!).ToArray();
            var fileNames = paths.Select(Path.GetFileName).ToArray();

            Assert.Contains("Tekla.Structures.dll", fileNames);
            Assert.Contains("Tekla.Structures.Analysis.dll", fileNames);
            Assert.Contains("Tekla.Structures.Catalogs.dll", fileNames);
            Assert.Contains("Tekla.Structures.Plugins.dll", fileNames);
            Assert.DoesNotContain("Tekla.Structures.Native.DbvDatabase.dll", fileNames);
            Assert.StartsWith(net48Runtime, paths.Single(path => path.EndsWith("Tekla.Structures.dll")));
        }
        finally
        {
            Directory.Delete(hostInstall, recursive: true);
        }
    }
}
