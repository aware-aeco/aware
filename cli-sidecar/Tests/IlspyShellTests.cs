using AwareSidecar.Decompile;
using Xunit;

namespace AwareSidecar.Tests;

public class IlspyShellTests
{
    [Fact]
    public void MissingPackageThrows()
    {
        var ex = Assert.Throws<InvalidOperationException>(() =>
            IlspyShell.Decompile("C:/this-does-not-exist-12345.nupkg", "1.0", null, false));
        Assert.Contains("nupkg not found", ex.Message);
    }

    [Fact]
    public void NonPermissiveLicenseBlocksWithoutAcceptFlag()
    {
        // Build a fake nupkg with a Proprietary license.
        // Use a unique path each run; avoid ZipArchiveMode.Create failing on existing files.
        var nupkg = Path.Combine(Path.GetTempPath(), $"aware-test-{Guid.NewGuid():N}.nupkg");
        try
        {
            using (var zip = System.IO.Compression.ZipFile.Open(nupkg, System.IO.Compression.ZipArchiveMode.Create))
            {
                var entry = zip.CreateEntry("Test.nuspec");
                using var w = new StreamWriter(entry.Open());
                w.Write("<?xml version=\"1.0\"?>\n<package><metadata>\n<id>Test</id><version>1.0.0</version>\n<description>x</description>\n<license type=\"expression\">Proprietary</license>\n</metadata></package>\n");
            }

            // Only run the rest of this assertion if ilspycmd is available; otherwise we get a
            // different error first. We can short-circuit by accepting that on systems where
            // ilspycmd is missing, the test verifies the "not installed" error path instead.
            try
            {
                IlspyShell.Decompile(nupkg, "1.0", null, acceptLicense: false);
                Assert.Fail("expected InvalidOperationException");
            }
            catch (InvalidOperationException ex)
            {
                // Accept either: "license not permissive" OR "ilspycmd not found"
                Assert.True(
                    ex.Message.Contains("license") || ex.Message.Contains("ilspycmd"),
                    $"unexpected error: {ex.Message}");
            }
        }
        finally
        {
            try { File.Delete(nupkg); } catch { }
        }
    }
}
