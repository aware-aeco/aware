// HttpScraper — HttpClient-based scraper for vendor doc sites that are fully
// server-rendered (no JS-driven content).
//
// Phase A shipped a Playwright-based Scraper for vendor sites that hydrate
// content client-side. For sites where the relevant DOM is in the initial HTML
// response (Tekla's developer.tekla.com is the proof-of-pattern), the heavyweight
// browser launch is wasted cost AND triggers a real obstacle: Playwright's .NET
// transport uses reflection-based JSON serialization internally, which is
// disabled by our NativeAOT / single-file publish (`PublishAot=true`). Forcing
// reflection JSON back on globally would defeat AOT determinism and bloat the
// binary; running a real browser to fetch HTML that arrives pre-rendered is
// architecturally backwards.
//
// HttpScraper is what those server-rendered sites should use. It mirrors the
// Scraper surface (`FetchHtml`, `ExtractLinks`) so an extractor can swap one
// for the other without changing the surrounding crawl loop, but it goes
// through HttpClient with a polite-bot User-Agent and short per-request
// timeouts.
//
// Concurrency: HttpClient is thread-safe; we expose `FetchHtml` only, with
// linear in-thread use today. If a vendor needs parallel fetches the caller
// fan-outs `Task.WhenAll` over multiple FetchHtml calls — the shared HttpClient
// inside HttpScraper handles that.
//
// AOT: pure managed code, no reflection on user types, no serialization at all
// — the only serialized payload is the raw HTML string. Fully NativeAOT-clean.

using System.Net;
using System.Net.Http;

namespace AwareSidecar.Ingest.Extractors.Common;

public sealed class HttpScraper : IAsyncDisposable
{
    readonly HttpClient _http;
    readonly HttpClientHandler _handler;

    public HttpScraper(string userAgent = "AWARE-coverage-extractor/0.30")
    {
        _handler = new HttpClientHandler
        {
            // Vendor doc sites uniformly gzip their HTML responses (Tekla measured at ~5x
            // compression). Without explicit decompression the GetStringAsync result is a
            // raw gzip byte stream interpreted as UTF-8 — short and garbled. AutomaticDecompression
            // lets the handler decode based on the response's Content-Encoding header.
            AutomaticDecompression = DecompressionMethods.All
        };
        _http = new HttpClient(_handler, disposeHandler: true)
        {
            Timeout = TimeSpan.FromSeconds(30)
        };
        _http.DefaultRequestHeaders.UserAgent.ParseAdd(userAgent);
        _http.DefaultRequestHeaders.Accept.ParseAdd("text/html,application/xhtml+xml");
    }

    public static Task<HttpScraper> Launch(string userAgent = "AWARE-coverage-extractor/0.30") =>
        Task.FromResult(new HttpScraper(userAgent));

    /// <summary>
    /// Fetch the page and return the raw HTML body. Throws if the response is non-2xx.
    /// </summary>
    public async Task<string> FetchHtml(string url, int timeoutMs = 30_000)
    {
        using var cts = new System.Threading.CancellationTokenSource(timeoutMs);
        var response = await _http.GetAsync(url, cts.Token);
        response.EnsureSuccessStatusCode();
        return await response.Content.ReadAsStringAsync(cts.Token);
    }

    public ValueTask DisposeAsync()
    {
        _http.Dispose();
        return ValueTask.CompletedTask;
    }
}
