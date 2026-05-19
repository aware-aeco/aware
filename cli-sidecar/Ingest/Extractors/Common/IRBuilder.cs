// IRBuilder — in-memory IR construction helper used by every vendor extractor.
//
// Vendor extractors typically walk a doc site (or a DLL, or a NuGet package, or
// an OpenAPI doc) and emit one TypeInfo at a time. Rather than have each extractor
// hand-roll the running counts and final assembly into a CoverageIR, this builder
// owns that bookkeeping: it tracks page count, type count, method count, event
// count, property count, and source URLs; on Build() it stamps `extracted_at`
// with the current UTC timestamp and emits the final IR doc ready to be written
// to disk.
//
// Method count rule (matches the IR schema): both constructors and instance/static
// methods count toward `metadata.method_count`. Operator overloads — if a vendor
// extractor ever emits them — would also live under `methods` and thus be counted.
//
// Concurrency: not safe for parallel mutation. Extractors that fan out work over
// multiple tasks should either build per-task IRBuilders and merge their type
// lists, or serialize the AddType calls through a single owner.

namespace AwareSidecar.Ingest.Extractors.Common;

public sealed class IRBuilder
{
    readonly string _host;
    readonly string _hostVersion;
    readonly string _sourceKind;
    readonly List<string> _urls = new();
    readonly List<TypeInfo> _types = new();
    int _methodCount;
    int _eventCount;
    int _propertyCount;
    int _pageCount;

    public IRBuilder(string host, string hostVersion, string sourceKind)
    {
        _host = host;
        _hostVersion = hostVersion;
        _sourceKind = sourceKind;
    }

    public void AddSourceUrl(string url) => _urls.Add(url);

    public void IncrementPageCount() => _pageCount++;

    public void AddType(TypeInfo t)
    {
        _types.Add(t);
        _methodCount += t.methods.Count + t.constructors.Count;
        _eventCount += t.events.Count;
        _propertyCount += t.properties.Count;
    }

    public CoverageIR Build() => new(
        _host,
        _hostVersion,
        new SourceInfo(_sourceKind, _urls, DateTime.UtcNow),
        _types,
        new MetadataInfo(_pageCount, _types.Count, _methodCount, _eventCount, _propertyCount));
}
