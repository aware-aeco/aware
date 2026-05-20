// Length-prefixed JSON framing for the aware-revit pipe protocol. Four bytes
// big-endian length, then UTF-8 JSON payload. One frame per request, one frame
// per response. Streams are otherwise opaque — the caller owns the
// NamedPipe{Server,Client}Stream lifecycle.
using System.Buffers.Binary;
using System.Text;

namespace AwareRevit.Shared;

public static class PipeFrame
{
    /// <summary>Read one frame. Returns null on clean EOF (pipe closed cleanly).</summary>
    public static async Task<string?> ReadAsync(Stream s, CancellationToken ct = default)
    {
        var lenBytes = new byte[4];
        int read = 0;
        while (read < 4)
        {
            var n = await s.ReadAsync(lenBytes.AsMemory(read, 4 - read), ct);
            if (n == 0) return read == 0 ? null : throw new EndOfStreamException("partial length prefix");
            read += n;
        }
        var len = BinaryPrimitives.ReadInt32BigEndian(lenBytes);
        if (len < 0 || len > 64 * 1024 * 1024)
            throw new InvalidDataException($"frame length out of range: {len}");

        var payload = new byte[len];
        read = 0;
        while (read < len)
        {
            var n = await s.ReadAsync(payload.AsMemory(read, len - read), ct);
            if (n == 0) throw new EndOfStreamException("payload truncated");
            read += n;
        }
        return Encoding.UTF8.GetString(payload);
    }

    /// <summary>Write one frame. Flushes the stream before returning.</summary>
    public static async Task WriteAsync(Stream s, string json, CancellationToken ct = default)
    {
        var payload = Encoding.UTF8.GetBytes(json);
        var lenBytes = new byte[4];
        BinaryPrimitives.WriteInt32BigEndian(lenBytes, payload.Length);
        await s.WriteAsync(lenBytes, ct);
        await s.WriteAsync(payload, ct);
        await s.FlushAsync(ct);
    }
}
