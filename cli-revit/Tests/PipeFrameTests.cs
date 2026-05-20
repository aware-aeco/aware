using System.IO;
using System.Text;
using AwareRevit.Shared;
using Xunit;

namespace AwareRevit.Tests;

public class PipeFrameTests
{
    [Fact]
    public async Task Roundtrip_PreservesJsonPayload()
    {
        var ms = new MemoryStream();
        await PipeFrame.WriteAsync(ms, "{\"id\":\"abc\",\"ok\":true}");
        ms.Position = 0;
        var got = await PipeFrame.ReadAsync(ms);
        Assert.Equal("{\"id\":\"abc\",\"ok\":true}", got);
    }

    [Fact]
    public async Task Roundtrip_UnicodePayload()
    {
        var ms = new MemoryStream();
        var payload = "{\"msg\":\"héllo 日本 🔥\"}";
        await PipeFrame.WriteAsync(ms, payload);
        ms.Position = 0;
        var got = await PipeFrame.ReadAsync(ms);
        Assert.Equal(payload, got);
    }

    [Fact]
    public async Task ReadAsync_OnEmptyStream_ReturnsNull()
    {
        var ms = new MemoryStream();
        var got = await PipeFrame.ReadAsync(ms);
        Assert.Null(got);
    }

    [Fact]
    public async Task WriteAsync_WritesLengthPrefixBigEndian()
    {
        var ms = new MemoryStream();
        await PipeFrame.WriteAsync(ms, "AB");  // 2-byte payload after UTF-8 encoding
        var bytes = ms.ToArray();
        Assert.Equal(6, bytes.Length); // 4-byte length + 2-byte payload
        // Big-endian length = 0x00000002
        Assert.Equal(0, bytes[0]);
        Assert.Equal(0, bytes[1]);
        Assert.Equal(0, bytes[2]);
        Assert.Equal(2, bytes[3]);
        Assert.Equal((byte)'A', bytes[4]);
        Assert.Equal((byte)'B', bytes[5]);
    }
}
