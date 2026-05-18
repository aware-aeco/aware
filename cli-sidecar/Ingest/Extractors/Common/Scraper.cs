// Scraper — shared Playwright wrapper used by every vendor extractor that needs
// to crawl an online API-reference site.
//
// Each extractor creates one Scraper, launches a headless Chromium context, then
// asks it for HTML pages (`FetchHtml`) or link lists (`ExtractLinks`). The scraper
// is disposable so the browser shuts down deterministically when the extractor is
// done.
//
// Why Playwright over a raw HttpClient? Most vendor doc sites render their type
// index / member tables client-side via JavaScript (Doxygen Awesome, DocFx, custom
// React shells). A bare HTTP fetch returns the empty SPA shell. Playwright runs
// a real browser, so `WaitUntilState.NetworkIdle` gives us the post-JS DOM that
// matches what a human reader sees.
//
// Runtime requirement (out of scope for A7): the user/CI must have run
// `playwright install chromium` once before invoking a real extraction. The
// Microsoft.Playwright NuGet only ships the .NET driver bindings; the actual
// browser binaries are downloaded on first install. This is documented in the
// Phase B vendor-extractor onboarding notes.
//
// NativeAOT note: `Playwright.CreateAsync()` internally launches a Node-based
// driver via Process.Start and communicates over stdio. There is no managed
// reflection on user types, so AOT publish works without per-namespace
// suppressions beyond the existing IL2026/IL3050 already in cli-sidecar.csproj.

using Microsoft.Playwright;

namespace AwareSidecar.Ingest.Extractors.Common;

public sealed class Scraper : IAsyncDisposable
{
    readonly IPlaywright _playwright;
    readonly IBrowser _browser;
    readonly IBrowserContext _context;

    public static async Task<Scraper> Launch(string userAgent = "AWARE-coverage-extractor/0.30")
    {
        var pw = await Playwright.CreateAsync();
        var browser = await pw.Chromium.LaunchAsync(new BrowserTypeLaunchOptions { Headless = true });
        var context = await browser.NewContextAsync(new BrowserNewContextOptions { UserAgent = userAgent });
        return new Scraper(pw, browser, context);
    }

    Scraper(IPlaywright playwright, IBrowser browser, IBrowserContext context)
    {
        _playwright = playwright;
        _browser = browser;
        _context = context;
    }

    /// <summary>
    /// Fetches a single page, waits for network idle, and returns the rendered HTML.
    /// </summary>
    public async Task<string> FetchHtml(string url, int timeoutMs = 30_000)
    {
        var page = await _context.NewPageAsync();
        try
        {
            await page.GotoAsync(url, new PageGotoOptions
            {
                Timeout = timeoutMs,
                WaitUntil = WaitUntilState.NetworkIdle
            });
            return await page.ContentAsync();
        }
        finally
        {
            await page.CloseAsync();
        }
    }

    /// <summary>
    /// Loads a page and returns the resolved `href` value of every element matching the
    /// CSS selector. Useful for harvesting the type-index of an API-reference site.
    /// </summary>
    public async Task<List<string>> ExtractLinks(string url, string cssSelector)
    {
        var page = await _context.NewPageAsync();
        try
        {
            await page.GotoAsync(url);
            var links = await page.EvalOnSelectorAllAsync<List<string>>(
                cssSelector,
                "elements => elements.map(e => e.href)");
            return links;
        }
        finally
        {
            await page.CloseAsync();
        }
    }

    public async ValueTask DisposeAsync()
    {
        await _context.CloseAsync();
        await _browser.CloseAsync();
        _playwright.Dispose();
    }
}
