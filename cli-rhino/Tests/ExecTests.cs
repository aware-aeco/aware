using AwareRhino.Verbs;
using Xunit;

namespace AwareRhino.Tests;

public class ExecTests
{
    [Fact]
    public void ExtractResultJson_HappyPath()
    {
        var stdout = "noise\n__AWARE_RESULT_BEGIN__\n{\"x\":1}\n__AWARE_RESULT_END__\nmore noise";
        var (json, rest) = Exec.ExtractResultJson(stdout);
        Assert.Equal("{\"x\":1}", json);
        Assert.Contains("noise", rest);
        Assert.Contains("more noise", rest);
        Assert.DoesNotContain("__AWARE_RESULT_", rest);
    }

    [Fact]
    public void ExtractResultJson_MultiLineJson()
    {
        var stdout = "__AWARE_RESULT_BEGIN__\n{\n  \"x\": 1,\n  \"y\": 2\n}\n__AWARE_RESULT_END__";
        var (json, _) = Exec.ExtractResultJson(stdout);
        Assert.NotNull(json);
        Assert.Contains("\"x\": 1", json!);
        Assert.Contains("\"y\": 2", json!);
    }

    [Fact]
    public void ExtractResultJson_NoSentinels_ReturnsNullAndAllStdout()
    {
        var stdout = "boom this script threw\nat line 5";
        var (json, rest) = Exec.ExtractResultJson(stdout);
        Assert.Null(json);
        Assert.Contains("boom this script threw", rest);
    }
}
