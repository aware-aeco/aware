#if NET10_0_OR_GREATER
using AwareRevit.AddIn;
using Xunit;

namespace AwareRevit.Tests;

public class RevitCompatibilityTests
{
    [Fact]
    public void Revit2027Addin_ReferencesTheInitialReleaseApi()
    {
        // Revit loads its own API assemblies. A bridge requiring 27.3 cannot
        // load beside an already-loaded 27.0 API in an earlier 2027 host.
        var references = typeof(AwarePendingRevitCommits).Assembly.GetReferencedAssemblies();
        foreach (var name in new[] { "RevitAPI", "RevitAPIUI" })
        {
            var reference = Assert.Single(references, assembly => assembly.Name == name);
            Assert.Equal(new Version(27, 0, 0, 0), reference.Version);
        }
    }
}
#endif
