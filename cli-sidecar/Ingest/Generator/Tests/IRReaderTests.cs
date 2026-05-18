using System.IO;
using Xunit;

namespace AwareSidecar.Ingest.Tests;

public class IRReaderTests
{
    [Fact]
    public void Reads_Minimal_Valid_IR()
    {
        var tmp = Path.GetTempFileName();
        try
        {
            File.WriteAllText(tmp, """
            {
              "host": "test", "host_version": "1.0",
              "source": { "kind": "scrape", "urls": ["https://example.com"], "extracted_at": "2026-01-01T00:00:00Z" },
              "types": [],
              "metadata": { "page_count": 0, "type_count": 0, "method_count": 0, "event_count": 0, "property_count": 0 }
            }
            """);
            var ir = IRReader.ReadFromFile(tmp);
            Assert.Equal("test", ir.host);
            Assert.Empty(ir.types);
        }
        finally
        {
            File.Delete(tmp);
        }
    }

    [Fact]
    public void Rejects_Type_Without_DocUrl()
    {
        var tmp = Path.GetTempFileName();
        try
        {
            File.WriteAllText(tmp, """
            {
              "host": "test", "host_version": "1.0",
              "source": { "kind": "scrape", "urls": [], "extracted_at": "2026-01-01T00:00:00Z" },
              "types": [{ "name": "Foo", "namespace": "Bar", "kind": "class", "summary": "summary", "doc_url": "" }],
              "metadata": { "page_count": 0, "type_count": 1, "method_count": 0, "event_count": 0, "property_count": 0 }
            }
            """);
            Assert.Throws<InvalidDataException>(() => IRReader.ReadFromFile(tmp));
        }
        finally
        {
            File.Delete(tmp);
        }
    }
}
