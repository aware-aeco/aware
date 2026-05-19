using AwareSidecar.Ingest.Extractors.Common;
using Xunit;

namespace AwareSidecar.Ingest.Tests;

public class ChmParserTests
{
    [Fact]
    public void Constructor_Rejects_Null_Path()
    {
        Assert.Throws<ArgumentNullException>(() => new ChmParser(null!));
    }

    [Fact]
    public void Decompile_Yields_Nothing_For_Missing_File_On_Windows()
    {
        // hh.exe -decompile is a quirky Win32 tool: handed a non-existent CHM it
        // silently exits 0 without writing any files. So on Windows we can verify
        // the observable contract — Decompile() returns an empty enumeration —
        // but not that an exception is thrown. On Linux/macOS hh.exe doesn't
        // exist at all, so Process.Start would throw Win32Exception; skip cleanly.
        if (!OperatingSystem.IsWindows())
        {
            return;
        }
        var parser = new ChmParser(Path.Combine(Path.GetTempPath(), "aware-does-not-exist-" + Guid.NewGuid() + ".chm"));
        var files = parser.Decompile().ToList();
        Assert.Empty(files);
    }
}
