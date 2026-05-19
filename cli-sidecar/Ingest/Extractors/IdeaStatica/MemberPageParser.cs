// MemberPageParser — enrich OpenAPI-derived TypeInfo records with information from the
// IDEA Statica REST SDK's markdown docs (auto-emitted by openapi-generator alongside
// the C# client).
//
// Source shape:
// -------------
// For each Connection / RCS API the openapi-generator pipeline emits a `docs/` folder
// next to the generated .cs files. Each .md file is one OpenAPI artifact:
//
//   • `docs/ConConnection.md` — one Model schema (object). Format:
//
//       # IdeaStatiCa.ConnectionApi.Model.ConConnection
//       ## Properties
//       Name | Type | Description | Notes
//       ------------ | ------------- | ------------- | -------------
//       **Id** | **int** |  | [optional]
//       **Identifier** | **string** |  | [optional]
//       ...
//
//   • `docs/CalculationApi.md` — one *Api class with all methods. Format:
//
//       # CalculationApi
//       | Method  | Description |
//       |--------|-------------|
//       | [**Calculate**](CalculationApi.md#calculate) | Runs CBFEM calculation...
//
//       <a id="calculate"></a>
//       ## **Calculate**
//       > **List<ConResultSummary> Calculate (Guid projectId, List<int> requestBody)**
//
//       Runs CBFEM calculation and returns the summary of the results.
//
//       ### Parameters
//       | Name | Type | Description | Notes |
//       |------|------|-------------|-------|
//       | **projectId** | **Guid** | The unique identifier of the opened project... |  |
//       | **requestBody** | [**List<int>**](int.md) | List of connection IDs to calculate. |  |
//
//       ### Return type
//       [**List<ConResultSummary>**](ConResultSummary.md)
//
// What this parser does:
// ---------------------
// The OpenAPI YAML already contains every piece of information the SDK docs render —
// the SDK docs ARE the YAML translated through openapi-generator's markdown template.
// The reason we still parse the markdown is twofold:
//
//   1. CROSS-CHECK: enriching descriptions from the markdown lets us catch any divergence
//      from the YAML (the YAML is the source of truth; if the markdown shows different
//      param descriptions, the openapi spec was tampered with by hand — log a remark).
//   2. PROVENANCE: the markdown gives us the per-member doc_url (one anchor per method),
//      which gives Browser-style navigability when a user follows the catalog into the
//      vendor docs.
//
// The enricher is a Pass-2-style overlay: it runs after OpenApiParser produces the
// initial TypeInfo list, walks the markdown for each type/api class, and refines:
//
//   • The type's per-member doc_url to `<sourceBaseUrl>/<RelativeFilename>.md`
//   • Property summaries when the markdown carries a non-empty Description column
//   • Method summaries / param docs when the markdown deviates from the YAML
//
// What this parser does NOT do:
// -----------------------------
// • It doesn't FIND members the YAML didn't already declare. The YAML is canonical.
//   If the markdown lists a method not present in the YAML, that's a bug in the SDK
//   and we don't paper over it.
// • It doesn't parse code blocks or example snippets — too noisy for catalog summary
//   strings, and the YAML's `description` already captures what matters.

using System.Text.RegularExpressions;

namespace AwareSidecar.Ingest.Extractors.IdeaStatica;

public static class MemberPageParser
{
    /// <summary>
    /// Enrichment result for a single Model markdown page (one schema). The keys are
    /// property names exactly as they appear in PascalCase form (matching IR).
    /// </summary>
    public sealed record ModelEnrichment(
        string? TypeSummary,
        IReadOnlyDictionary<string, string> PropertyDescriptions);

    /// <summary>
    /// Enrichment result for an *Api markdown page (one tag's worth of methods).
    /// Keyed by method anchor id which is the lowercased operationId. Property
    /// summaries flow from the per-method Parameters table.
    /// </summary>
    public sealed record ApiEnrichment(
        string? ClassSummary,
        IReadOnlyDictionary<string, MethodPatch> MethodsByOperationId);

    public sealed record MethodPatch(
        string? Summary,                                  // From the prose paragraph below the heading
        IReadOnlyDictionary<string, string> ParamDocs,    // param name → description
        string? ReturnDoc);

    /// <summary>
    /// Parse a Model markdown page (e.g. `ConConnection.md`) into TypeSummary + per-property
    /// description lookups. Returns null if the page doesn't match the openapi-generator
    /// Model template (e.g. README.md or an API page mis-routed here).
    /// </summary>
    public static ModelEnrichment? ParseModel(string markdown)
    {
        if (!markdown.Contains("## Properties", StringComparison.Ordinal))
        {
            // Some openapi-generator Model pages for enum types or oneOf composites have no
            // Properties section. Return an empty enrichment so the caller can still fold in
            // any class-level summary.
            return new ModelEnrichment(ExtractH1Title(markdown), new Dictionary<string, string>(StringComparer.Ordinal));
        }

        var typeSummary = ExtractH1Title(markdown);
        var props = new Dictionary<string, string>(StringComparer.Ordinal);

        // Find the Properties table. Format:
        //   Name | Type | Description | Notes
        //   ------------ | ------------- | ------------- | -------------
        //   **Id** | **int** |  | [optional]
        //   **Identifier** | **string** |  | [optional]
        var lines = markdown.Split('\n');
        int i = 0;
        while (i < lines.Length && !lines[i].StartsWith("## Properties", StringComparison.Ordinal)) i++;
        if (i >= lines.Length) return new ModelEnrichment(typeSummary, props);
        // Skip to the header divider line (------).
        while (i < lines.Length && !IsTableDivider(lines[i])) i++;
        if (i >= lines.Length) return new ModelEnrichment(typeSummary, props);
        i++;
        while (i < lines.Length)
        {
            var line = lines[i].TrimEnd('\r').TrimEnd();
            if (string.IsNullOrEmpty(line)) break;
            if (!line.Contains('|')) break;
            var cells = line.Split('|').Select(c => c.Trim()).ToArray();
            // cells[0] = Name (bolded), cells[1] = Type, cells[2] = Description, cells[3] = Notes.
            // Empty leading/trailing cells creep in when the | starts/ends the line.
            // After Split, an Md `**Id** | **int** | ...` becomes ["**Id**", "**int**", ...].
            // No leading/trailing | on Model tables in this SDK output.
            if (cells.Length < 3) { i++; continue; }
            var name = StripBold(cells[0]);
            var description = cells[2];
            if (!string.IsNullOrEmpty(name) && !string.IsNullOrEmpty(description))
            {
                props[name] = description;
            }
            i++;
        }
        return new ModelEnrichment(typeSummary, props);
    }

    /// <summary>
    /// Parse an Api markdown page (e.g. `CalculationApi.md`) into per-operation patches.
    /// The operation key is the lowercased operationId (the anchor id used by the SDK).
    /// </summary>
    public static ApiEnrichment? ParseApi(string markdown)
    {
        if (!markdown.StartsWith("# ", StringComparison.Ordinal)) return null;
        var classSummary = ExtractApiClassSummary(markdown);
        var patches = new Dictionary<string, MethodPatch>(StringComparer.OrdinalIgnoreCase);

        // Split into method sections. The openapi-generator template emits one section per
        // operation, each starting with `<a id="<opId-lowercase>"></a>` and a `## **OperationName**` heading.
        var anchorRe = new Regex(@"<a id=""([A-Za-z0-9_-]+)""\s*>\s*</a>\s*\n##\s+\*\*([A-Za-z0-9_]+)\*\*", RegexOptions.Compiled);
        var sections = new List<(string opId, string body)>();
        var matches = anchorRe.Matches(markdown);
        for (int i = 0; i < matches.Count; i++)
        {
            var start = matches[i].Index;
            var end = i + 1 < matches.Count ? matches[i + 1].Index : markdown.Length;
            var opId = matches[i].Groups[1].Value;
            sections.Add((opId, markdown.Substring(start, end - start)));
        }

        foreach (var (opId, body) in sections)
        {
            var patch = ParseMethodSection(body);
            if (patch is null) continue;
            patches[opId] = patch;
        }

        return new ApiEnrichment(classSummary, patches);
    }

    static MethodPatch? ParseMethodSection(string body)
    {
        var paramDocs = new Dictionary<string, string>(StringComparer.Ordinal);
        string? summary = null;
        string? returnDoc = null;

        var lines = body.Split('\n');

        // The summary is the first non-empty plain-text paragraph after the signature `>` line.
        int i = 0;
        while (i < lines.Length)
        {
            var line = lines[i].TrimEnd('\r').TrimEnd();
            if (line.StartsWith("> ", StringComparison.Ordinal)) { i++; break; }
            i++;
        }
        var summaryBuilder = new System.Text.StringBuilder();
        while (i < lines.Length)
        {
            var line = lines[i].TrimEnd('\r').Trim();
            if (line.StartsWith("###", StringComparison.Ordinal) || line.StartsWith("[[", StringComparison.Ordinal)) break;
            if (!string.IsNullOrEmpty(line))
            {
                if (summaryBuilder.Length > 0) summaryBuilder.Append(' ');
                summaryBuilder.Append(line);
            }
            else if (summaryBuilder.Length > 0)
            {
                break; // First paragraph break ends the summary.
            }
            i++;
        }
        if (summaryBuilder.Length > 0) summary = OpenApiParser.NormaliseInlineHtml(summaryBuilder.ToString());

        // Parameters table.
        int paramHeaderIdx = -1;
        for (int j = 0; j < lines.Length; j++)
        {
            if (lines[j].TrimEnd('\r').Trim().Equals("### Parameters", StringComparison.Ordinal))
            {
                paramHeaderIdx = j; break;
            }
        }
        if (paramHeaderIdx >= 0)
        {
            int k = paramHeaderIdx + 1;
            while (k < lines.Length && !IsTableDivider(lines[k])) k++;
            if (k < lines.Length)
            {
                k++;
                while (k < lines.Length)
                {
                    var row = lines[k].TrimEnd('\r').TrimEnd();
                    if (string.IsNullOrEmpty(row) || !row.Contains('|')) break;
                    var cells = row.Split('|').Select(c => c.Trim()).ToList();
                    // Markdown tables with leading/trailing pipes produce empty first/last cells.
                    if (cells.Count > 0 && cells[0].Length == 0) cells.RemoveAt(0);
                    if (cells.Count > 0 && cells[^1].Length == 0) cells.RemoveAt(cells.Count - 1);
                    if (cells.Count >= 3)
                    {
                        var paramName = StripBold(cells[0]);
                        var paramDesc = cells[2];
                        if (!string.IsNullOrEmpty(paramName) && !string.IsNullOrEmpty(paramDesc))
                        {
                            paramDocs[paramName] = OpenApiParser.NormaliseInlineHtml(paramDesc);
                        }
                    }
                    k++;
                }
            }
        }

        // Return type block — the line immediately following `### Return type` is the type itself
        // (e.g. `[**List<ConResultSummary>**](ConResultSummary.md)`). The next non-empty paragraph
        // is the description.
        int retHeaderIdx = -1;
        for (int j = 0; j < lines.Length; j++)
        {
            if (lines[j].TrimEnd('\r').Trim().Equals("### Return type", StringComparison.Ordinal))
            {
                retHeaderIdx = j; break;
            }
        }
        if (retHeaderIdx >= 0)
        {
            // The block after the heading is the markdown link; description text would be on the
            // following lines but openapi-generator usually leaves it empty. Pick up whatever
            // non-link prose appears between the type link and the next `###` heading.
            int k = retHeaderIdx + 1;
            var rb = new System.Text.StringBuilder();
            while (k < lines.Length)
            {
                var line = lines[k].TrimEnd('\r').Trim();
                if (line.StartsWith("###", StringComparison.Ordinal) || line.StartsWith("[[", StringComparison.Ordinal)) break;
                if (string.IsNullOrEmpty(line)) { k++; continue; }
                // Skip the very first line if it's the type-link itself.
                if (rb.Length == 0 && line.StartsWith("[**", StringComparison.Ordinal)) { k++; continue; }
                if (rb.Length > 0) rb.Append(' ');
                rb.Append(line);
                k++;
            }
            if (rb.Length > 0) returnDoc = OpenApiParser.NormaliseInlineHtml(rb.ToString());
        }

        return new MethodPatch(summary, paramDocs, returnDoc);
    }

    static string? ExtractH1Title(string markdown)
    {
        // The Model templates emit `# IdeaStatiCa.ConnectionApi.Model.<Name>`. There's no per-class
        // description here — the human-readable summary on Model pages is the type name only.
        // We return null so the YAML-derived summary wins.
        return null;
    }

    static string? ExtractApiClassSummary(string markdown)
    {
        // Api pages have an H1 like `# CalculationApi` and then a method-index table. There is no
        // class-level description prose. Return null so the YAML's class blurb wins.
        return null;
    }

    static string StripBold(string s)
    {
        s = s.Trim();
        if (s.StartsWith("**", StringComparison.Ordinal) && s.EndsWith("**", StringComparison.Ordinal) && s.Length >= 4)
            return s.Substring(2, s.Length - 4).Trim();
        return s;
    }

    static bool IsTableDivider(string line)
    {
        var t = line.Trim();
        if (string.IsNullOrEmpty(t)) return false;
        // A markdown table divider is `---|---|---` or `------- | ------- | -------` etc.
        // Allow optional leading/trailing pipes and any combination of dashes, colons (alignment),
        // and pipes.
        foreach (var c in t)
        {
            if (c != '-' && c != '|' && c != ':' && c != ' ' && c != '\t') return false;
        }
        return t.Contains('-');
    }
}
