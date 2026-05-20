using Xunit;

namespace AwareTekla.Tests;

public class ExtractVersionFromPathTests
{
    [Theory]
    [InlineData(@"C:\Program Files\Tekla Structures\2026.0\bin\TeklaStructures.exe", "2026.0")]
    [InlineData(@"C:\Program Files\Tekla Structures\2025.0\bin\TeklaStructures.exe", "2025.0")]
    [InlineData("C:/Program Files/Tekla Structures/2024.0/bin/TeklaStructures.exe", "2024.0")]
    public void ExtractsVersion_FromStandardInstallPath(string path, string expected)
    {
        Assert.Equal(expected, Program.ExtractVersionFromPath(path));
    }

    [Fact]
    public void ReturnsNull_WhenNoVersionSegment()
    {
        Assert.Null(Program.ExtractVersionFromPath(@"C:\Some\Other\App\app.exe"));
    }

    [Fact]
    public void ReturnsNull_WhenSegmentIsNotVersionShaped()
    {
        // "Tekla Structures" present but next segment isn't YYYY.N.
        Assert.Null(Program.ExtractVersionFromPath(@"C:\Program Files\Tekla Structures\beta\bin\TeklaStructures.exe"));
    }
}
