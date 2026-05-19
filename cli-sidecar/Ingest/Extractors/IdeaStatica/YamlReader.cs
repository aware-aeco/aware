// YamlReader — a minimal YAML 1.2 parser for the openapi-generator output subset.
//
// Why a custom parser:
// --------------------
// The IDEA Statica extractor needs to read OpenAPI documents (~7600 lines of YAML
// per API). YamlDotNet is the canonical .NET YAML library, but it relies heavily
// on reflection-based serialization and conditionally generates `RequiresUnreferencedCode`
// warnings under our `<PublishAot>true</PublishAot>` sidecar build. Adopting it
// would push us back into the same AOT-warning suppression patterns we've been
// avoiding (see Tekla extractor's "Why not Playwright" rationale). Worse, YamlDotNet's
// public DOM API surface is large and tightly coupled to .NET runtime reflection,
// which makes correct trimming hard to prove.
//
// The openapi-generator output, however, is a tightly-constrained YAML subset:
//
//   • Block-style only — no flow `{a: 1, b: 2}` or `[1, 2, 3]` notation
//   • 2-space indentation, always; tabs never appear
//   • Scalars are either unquoted (no whitespace-confusing chars), single-quoted,
//     or double-quoted; nothing more exotic
//   • Multi-line double-quoted strings use backslash continuation: `"foo\
//     bar"` becomes `"foobar"` after the parser strips the line break (+ leading
//     whitespace on the continuation line)
//   • Special-cased keys are double-quoted: `"y"` when y is a YAML reserved word
//     and `"200"` for HTTP-status response codes
//   • Sequence items are `- key: value` (with the value possibly being another
//     mapping on the next indent level) or `- scalar`
//   • Comments (`# ...`) do appear sparingly but are stripped from the end-of-line
//
// This parser produces a `YamlNode` tree of three kinds: Scalar (string), Mapping
// (ordered key→YamlNode), and Sequence (list of YamlNode). That's enough surface
// to express the entire OpenAPI document and to walk it with strongly-typed C#
// code in the IDEA Statica extractor. No reflection. No package dependency. AOT-clean.
//
// What this parser does NOT support — intentional gaps we don't need:
//
//   • Anchors / aliases (`&foo` / `*foo`)
//   • Merge keys (`<<: *base`)
//   • Tag types (`!!int`, `!!str`)
//   • Block scalars (`|`, `>` with explicit indentation indicators)
//   • Flow style
//   • Multi-document streams (`---` / `...` separators)
//
// If openapi-generator output ever starts using any of those, this parser will
// emit ParseException at the offending line. That's the right failure mode — better
// than silently producing a wrong tree.

using System.Text;

namespace AwareSidecar.Ingest.Extractors.IdeaStatica;

/// <summary>
/// Three node kinds in the YAML DOM we emit.
/// </summary>
public abstract record YamlNode
{
    public sealed record Scalar(string Value) : YamlNode
    {
        /// <summary>True when the underlying value was the literal scalar `null`.</summary>
        public bool IsNull => string.Equals(Value, "null", StringComparison.Ordinal);
    }

    public sealed record Mapping(Dictionary<string, YamlNode> Entries, List<string> KeyOrder) : YamlNode
    {
        public YamlNode? Get(string key) => Entries.TryGetValue(key, out var v) ? v : null;
        public string? GetScalar(string key) => Get(key) is Scalar s ? s.Value : null;
        public Mapping? GetMapping(string key) => Get(key) as Mapping;
        public Sequence? GetSequence(string key) => Get(key) as Sequence;
    }

    public sealed record Sequence(List<YamlNode> Items) : YamlNode;

    /// <summary>
    /// Convenience: return the string scalar value or empty if not present / not a scalar.
    /// </summary>
    public string AsString() => this is Scalar s ? s.Value : "";
}

/// <summary>
/// Raised when the parser encounters a YAML construct it doesn't support, or a
/// structural error in the input (mixed indentation, unterminated quote, etc.).
/// The message identifies the line + column so a human can locate the issue.
/// </summary>
public sealed class YamlParseException(string message) : Exception(message);

public static class YamlReader
{
    /// <summary>
    /// Parse a full YAML document and return the root node (typically a Mapping).
    /// </summary>
    public static YamlNode Parse(string yaml)
    {
        var lines = SplitLogicalLines(yaml);
        int idx = 0;
        return ParseBlockNode(lines, ref idx, parentIndent: -1)
               ?? new YamlNode.Mapping(new Dictionary<string, YamlNode>(), new List<string>());
    }

    /// <summary>
    /// A logical line carries the original line number (1-indexed, for error messages),
    /// its indentation (count of leading space chars; tabs are forbidden), and the
    /// stripped-of-leading-indent textual content (comments + trailing whitespace
    /// already removed). Blank lines are filtered out at this stage.
    /// </summary>
    sealed record LogicalLine(int LineNo, int Indent, string Content);

    static List<LogicalLine> SplitLogicalLines(string yaml)
    {
        var lines = yaml.Split('\n');
        var result = new List<LogicalLine>();
        for (int i = 0; i < lines.Length; i++)
        {
            // Strip a single trailing CR (Windows line ending).
            var raw = lines[i];
            if (raw.Length > 0 && raw[^1] == '\r') raw = raw[..^1];

            // Skip whitespace-only lines.
            if (string.IsNullOrWhiteSpace(raw)) continue;

            // Tabs are not allowed for indentation in this YAML subset. The openapi-generator
            // never emits them, so a tab is a strong signal that input was hand-edited and
            // we should bail.
            int indent = 0;
            while (indent < raw.Length && raw[indent] == ' ') indent++;
            if (indent < raw.Length && raw[indent] == '\t')
                throw new YamlParseException($"line {i + 1}: tab indentation not supported");

            // Strip trailing whitespace (comments are NOT stripped here — comments inside
            // double-quoted scalars are valid string characters; the scalar parser handles
            // the quoted case. Comments outside quotes are stripped per-line in the
            // mapping-value parser instead). Trailing whitespace removal is purely cosmetic.
            string content = raw.Substring(indent).TrimEnd();
            if (content.Length == 0) continue;

            // Skip pure-comment lines.
            if (content[0] == '#') continue;

            result.Add(new LogicalLine(i + 1, indent, content));
        }
        return result;
    }

    /// <summary>
    /// Parse a block-style node starting at lines[idx]. Consumes one or more lines, advancing
    /// `idx`. Returns null if the next line is not at a valid child indent.
    ///
    /// YAML allows a block sequence to live at the SAME indent as its parent key — i.e.:
    ///
    ///     parameters:
    ///     - name: projectId
    ///       in: path
    ///
    /// Here `parameters:` is at indent 0 and the sequence items also begin at indent 0
    /// but with the `-` indicator. The YAML 1.2 spec calls this the "indent of one" rule:
    /// when the value of a key is a sequence, the items can be flush-left with the key
    /// itself. We accept that here. Mappings as values still require strict indent increase.
    /// </summary>
    static YamlNode? ParseBlockNode(List<LogicalLine> lines, ref int idx, int parentIndent)
    {
        if (idx >= lines.Count) return null;
        var first = lines[idx];

        bool isSequenceMarker = first.Content.StartsWith("- ", StringComparison.Ordinal) || first.Content == "-";
        if (isSequenceMarker)
        {
            // Block-sequence-at-parent's-indent is allowed: the `-` itself counts as one level
            // of "implicit indent". Accept `first.Indent == parentIndent` for sequences.
            if (first.Indent < parentIndent) return null;
            if (first.Indent == parentIndent && parentIndent == -1) return ParseBlockSequence(lines, ref idx, first.Indent);
            return ParseBlockSequence(lines, ref idx, first.Indent);
        }

        // Mappings: strict indent increase.
        if (first.Indent <= parentIndent) return null;
        return ParseBlockMapping(lines, ref idx, first.Indent);
    }

    static YamlNode.Mapping ParseBlockMapping(List<LogicalLine> lines, ref int idx, int indent)
    {
        var entries = new Dictionary<string, YamlNode>();
        var order = new List<string>();
        while (idx < lines.Count)
        {
            var line = lines[idx];
            if (line.Indent < indent) break;
            if (line.Indent > indent)
                throw new YamlParseException(
                    $"line {line.LineNo}: unexpected indentation {line.Indent} (expected {indent})");

            // Sequence start at the same level breaks the mapping.
            if (line.Content.StartsWith("- ", StringComparison.Ordinal) || line.Content == "-") break;

            var (key, valuePart) = SplitKeyValue(line);
            idx++;

            YamlNode value;
            if (string.IsNullOrEmpty(valuePart))
            {
                // No inline value — the value is a nested block on the next line.
                var nested = ParseBlockNode(lines, ref idx, parentIndent: line.Indent);
                value = nested ?? new YamlNode.Scalar("");
            }
            else if (valuePart.StartsWith("\"", StringComparison.Ordinal))
            {
                // Double-quoted scalar — may span multiple lines via `\` continuation.
                value = ParseDoubleQuotedScalar(lines, ref idx, valuePart, line.Indent);
            }
            else if (valuePart.StartsWith("'", StringComparison.Ordinal))
            {
                value = ParseSingleQuotedScalar(valuePart, line.LineNo);
            }
            else
            {
                // Plain unquoted scalar — strip an end-of-line comment if any.
                // Plain scalars MAY span multiple lines using YAML's implicit folding rules:
                // a continuation line is one that's more indented than the current mapping
                // context (`indent`). openapi-generator emits these for long descriptions
                // that DON'T begin with a quote (line 22-23 of the Connection API yaml is the
                // poster child: `description: The unique identifier...` followed by a more-
                // indented `service.` line).
                value = ParsePlainScalar(lines, ref idx, valuePart, indent);
            }

            entries[key] = value;
            order.Add(key);
        }
        return new YamlNode.Mapping(entries, order);
    }

    /// <summary>
    /// Parse a plain (unquoted) scalar that may fold across multiple lines per YAML's
    /// implicit folding rules: any subsequent line indented STRICTLY MORE than
    /// <paramref name="mappingIndent"/> is appended (with a single space separator,
    /// matching YAML's flow-folding semantics). The first line that's at or below
    /// the mapping indent ends the scalar.
    /// </summary>
    static YamlNode.Scalar ParsePlainScalar(List<LogicalLine> lines, ref int idx, string firstChunk, int mappingIndent)
    {
        var sb = new StringBuilder(StripInlineComment(firstChunk).TrimEnd());
        while (idx < lines.Count)
        {
            var next = lines[idx];
            if (next.Indent <= mappingIndent) break;
            // The continuation line must not be a sequence-or-mapping start — those would be
            // legitimate child nodes; we shouldn't fold them into the scalar.
            var c0 = next.Content[0];
            if (c0 == '-' && (next.Content.Length == 1 || next.Content[1] == ' ')) break;
            // A continuation line that looks like "key: value" is also not a scalar-fold target
            // — bail out and let the mapping parser handle it. This matters when the scalar
            // ends and a deeper sibling mapping continues. openapi-generator output doesn't
            // produce this case in practice, but we're defensive.
            if (LooksLikeMappingKey(next.Content)) break;

            sb.Append(' ').Append(StripInlineComment(next.Content).Trim());
            idx++;
        }
        return new YamlNode.Scalar(sb.ToString());
    }

    static bool LooksLikeMappingKey(string content)
    {
        // Heuristic: starts with an identifier-like token, contains ": " or ends with ":",
        // and the colon isn't inside a URL or a generic. openapi-generator's plain-scalar
        // continuation lines never accidentally satisfy this; description prose always
        // contains spaces and rarely "key: pattern".
        int i = 0;
        // Allow optional leading quoted key.
        if (i < content.Length && content[i] == '"') return content.Contains("\": ", StringComparison.Ordinal) || content.EndsWith("\":", StringComparison.Ordinal);
        // Must begin with an identifier char.
        if (i >= content.Length) return false;
        if (!IsIdentChar(content[i])) return false;
        // Scan a short identifier run.
        int j = i;
        while (j < content.Length && IsIdentChar(content[j])) j++;
        // Empty identifier or no immediate colon → not a mapping key.
        if (j == i) return false;
        if (j >= content.Length) return false;
        if (content[j] != ':') return false;
        if (j + 1 == content.Length) return true;     // "foo:"
        char after = content[j + 1];
        return after == ' ' || after == '\t';
    }

    static bool IsIdentChar(char c) => char.IsLetterOrDigit(c) || c == '_' || c == '-' || c == '$' || c == '/' || c == '.';

    static YamlNode.Sequence ParseBlockSequence(List<LogicalLine> lines, ref int idx, int indent)
    {
        var items = new List<YamlNode>();
        while (idx < lines.Count)
        {
            var line = lines[idx];
            if (line.Indent < indent) break;
            if (line.Indent > indent)
                throw new YamlParseException(
                    $"line {line.LineNo}: unexpected indentation in sequence at {line.Indent} (expected {indent})");
            if (!(line.Content.StartsWith("- ", StringComparison.Ordinal) || line.Content == "-")) break;

            string after = line.Content == "-" ? "" : line.Content.Substring(2);
            idx++;

            if (string.IsNullOrEmpty(after))
            {
                // Bare "-" — the item is a block on the next more-indented line.
                var nested = ParseBlockNode(lines, ref idx, parentIndent: line.Indent);
                items.Add(nested ?? new YamlNode.Scalar(""));
            }
            else if (after.Contains(": ", StringComparison.Ordinal) || EndsWithColon(after))
            {
                // The item starts a mapping: "- key: value" or "- key:".
                // Synthesize a logical line at indent + 2 for the first mapping pair, then
                // re-enter ParseBlockMapping. The trick: openapi-generator emits subsequent
                // pairs of the same item indented by indent + 2 (one extra level).
                var synthesized = new LogicalLine(line.LineNo, line.Indent + 2, after);
                lines.Insert(idx, synthesized);
                var mapping = ParseBlockMapping(lines, ref idx, line.Indent + 2);
                items.Add(mapping);
            }
            else
            {
                YamlNode value;
                if (after.StartsWith("\"", StringComparison.Ordinal))
                {
                    value = ParseDoubleQuotedScalar(lines, ref idx, after, line.Indent);
                }
                else if (after.StartsWith("'", StringComparison.Ordinal))
                {
                    value = ParseSingleQuotedScalar(after, line.LineNo);
                }
                else
                {
                    value = new YamlNode.Scalar(StripInlineComment(after).Trim());
                }
                items.Add(value);
            }
        }
        return new YamlNode.Sequence(items);
    }

    /// <summary>
    /// Heuristic: the line ends with a colon and represents a key without an inline value.
    /// Returns true only when the colon is at the end of the visible content (after any
    /// inline comment has been stripped).
    /// </summary>
    static bool EndsWithColon(string s)
    {
        var stripped = StripInlineComment(s).TrimEnd();
        return stripped.EndsWith(":", StringComparison.Ordinal);
    }

    /// <summary>
    /// Split a `key: value` style line into the key (raw, possibly quoted) and the value
    /// portion (everything after the FIRST ": " — may itself be empty if the line is just
    /// "key:" with no inline value).
    /// </summary>
    static (string key, string valuePart) SplitKeyValue(LogicalLine line)
    {
        // The key portion may be double-quoted (e.g. "200": for HTTP status codes, or
        // "y": for the reserved YAML word). Quoted keys may contain ": " inside their
        // quotes, so we must scan past them first.
        string content = line.Content;
        int colonIdx = FindColonAfterKey(content);
        if (colonIdx < 0)
        {
            // Treat the line as "key: " with empty value (caller will look for a nested block).
            if (content.EndsWith(":", StringComparison.Ordinal))
                return (UnquoteKey(content[..^1].TrimEnd(), line.LineNo), "");
            throw new YamlParseException($"line {line.LineNo}: expected ': ' in mapping entry: {content}");
        }
        var rawKey = content.Substring(0, colonIdx).TrimEnd();
        var afterColon = colonIdx + 1 < content.Length ? content.Substring(colonIdx + 1) : "";
        // Drop one leading space if present (the canonical "key: value" delimiter).
        if (afterColon.StartsWith(" ", StringComparison.Ordinal)) afterColon = afterColon.Substring(1);
        return (UnquoteKey(rawKey, line.LineNo), afterColon);
    }

    static int FindColonAfterKey(string s)
    {
        // If the line starts with a quote, scan to the matching close-quote, then look for
        // the colon immediately after.
        int i = 0;
        if (i < s.Length && (s[i] == '"' || s[i] == '\''))
        {
            char q = s[i];
            i++;
            while (i < s.Length)
            {
                if (s[i] == '\\' && q == '"' && i + 1 < s.Length) { i += 2; continue; }
                if (s[i] == q) { i++; break; }
                i++;
            }
            // Look for colon after the close-quote.
            while (i < s.Length && (s[i] == ' ' || s[i] == '\t')) i++;
            if (i < s.Length && s[i] == ':')
            {
                // Either "key: value" or "key:\n" — return the colon position.
                return i;
            }
            return -1;
        }
        // Unquoted key — find the first ": " or trailing ":".
        for (i = 0; i < s.Length; i++)
        {
            if (s[i] == ':')
            {
                if (i + 1 == s.Length) return i;             // "key:"
                char next = s[i + 1];
                if (next == ' ' || next == '\t') return i;   // "key: value"
            }
        }
        return -1;
    }

    static string UnquoteKey(string raw, int lineNo)
    {
        if (raw.Length >= 2 && raw[0] == '"' && raw[^1] == '"')
        {
            return UnescapeDoubleQuoted(raw.Substring(1, raw.Length - 2));
        }
        if (raw.Length >= 2 && raw[0] == '\'' && raw[^1] == '\'')
        {
            return raw.Substring(1, raw.Length - 2).Replace("''", "'");
        }
        return raw;
    }

    /// <summary>
    /// Parse a double-quoted scalar that may span multiple physical lines via the `\`
    /// continuation convention. openapi-generator emits exactly this pattern for any
    /// description longer than ~80 chars.
    /// </summary>
    static YamlNode.Scalar ParseDoubleQuotedScalar(List<LogicalLine> lines, ref int idx, string firstChunk, int indent)
    {
        var sb = new StringBuilder();
        // Strip the leading quote from firstChunk.
        string body = firstChunk.Substring(1);
        while (true)
        {
            // Find the closing quote — unescaped.
            int end = FindUnescapedDoubleQuote(body);
            if (end >= 0)
            {
                sb.Append(body, 0, end);
                return new YamlNode.Scalar(UnescapeDoubleQuoted(sb.ToString()));
            }
            // No closing quote on this line — the line MUST end with a backslash signifying
            // continuation. openapi-generator always uses `\` at end of line + the next line
            // starts with leading whitespace that we trim.
            if (body.EndsWith("\\", StringComparison.Ordinal))
            {
                sb.Append(body, 0, body.Length - 1);
                if (idx >= lines.Count)
                    throw new YamlParseException("EOF in multi-line double-quoted scalar");
                var nextLine = lines[idx];
                idx++;
                body = nextLine.Content.TrimStart();
                // Drop the literal `\ ` continuation prefix emitted by openapi-generator —
                // it appears as `\` followed by exactly one space at the start of the
                // continuation line in the raw YAML. After TrimStart we may have just `\` left.
                if (body.StartsWith("\\ ", StringComparison.Ordinal)) body = body.Substring(2);
                else if (body.StartsWith("\\", StringComparison.Ordinal)) body = body.Substring(1);
            }
            else
            {
                // The line ended without a closing quote and without a backslash. That's malformed
                // for our subset.
                throw new YamlParseException($"unterminated double-quoted scalar near: {body}");
            }
        }
    }

    static int FindUnescapedDoubleQuote(string s)
    {
        int i = 0;
        while (i < s.Length)
        {
            if (s[i] == '\\' && i + 1 < s.Length) { i += 2; continue; }
            if (s[i] == '"') return i;
            i++;
        }
        return -1;
    }

    /// <summary>
    /// Decode YAML double-quoted escapes: \n \t \" \\ \r and unicode-style escapes (\u00XX)
    /// when openapi-generator emits them. Surrogate pairs are passed through verbatim.
    /// </summary>
    static string UnescapeDoubleQuoted(string s)
    {
        var sb = new StringBuilder(s.Length);
        int i = 0;
        while (i < s.Length)
        {
            char c = s[i];
            if (c == '\\' && i + 1 < s.Length)
            {
                char next = s[i + 1];
                switch (next)
                {
                    case 'n': sb.Append('\n'); i += 2; break;
                    case 't': sb.Append('\t'); i += 2; break;
                    case 'r': sb.Append('\r'); i += 2; break;
                    case '"': sb.Append('"'); i += 2; break;
                    case '\\': sb.Append('\\'); i += 2; break;
                    case '/': sb.Append('/'); i += 2; break;
                    case '0': sb.Append('\0'); i += 2; break;
                    case 'u':
                    {
                        if (i + 5 < s.Length)
                        {
                            var hex = s.Substring(i + 2, 4);
                            if (ushort.TryParse(hex, System.Globalization.NumberStyles.HexNumber,
                                                System.Globalization.CultureInfo.InvariantCulture, out var u))
                            {
                                sb.Append((char)u);
                                i += 6;
                                break;
                            }
                        }
                        sb.Append(c); i++; break;
                    }
                    default: sb.Append(c); i++; break;
                }
            }
            else { sb.Append(c); i++; }
        }
        return sb.ToString();
    }

    /// <summary>
    /// Single-quoted YAML scalars: only `''` is an escape (for a literal apostrophe). No backslash
    /// escapes apply. openapi-generator uses this form for things like `'#/components/schemas/Foo'`.
    /// We expect the closing quote on the same line — multi-line single-quoted is allowed by YAML
    /// but doesn't appear in openapi-generator output.
    /// </summary>
    static YamlNode.Scalar ParseSingleQuotedScalar(string chunk, int lineNo)
    {
        string body = chunk.Substring(1);
        int end = body.IndexOf('\'');
        if (end < 0) throw new YamlParseException($"line {lineNo}: unterminated single-quoted scalar");
        return new YamlNode.Scalar(body.Substring(0, end).Replace("''", "'"));
    }

    /// <summary>
    /// Strip an end-of-line comment (a `#` outside of any quoted string). Conservative: the
    /// openapi-generator output rarely contains inline comments; this is purely defensive.
    /// </summary>
    static string StripInlineComment(string s)
    {
        bool inDouble = false, inSingle = false;
        for (int i = 0; i < s.Length; i++)
        {
            char c = s[i];
            if (c == '\\' && i + 1 < s.Length && inDouble) { i++; continue; }
            if (c == '"' && !inSingle) inDouble = !inDouble;
            else if (c == '\'' && !inDouble) inSingle = !inSingle;
            else if (c == '#' && !inDouble && !inSingle)
            {
                // Only strip when preceded by whitespace (inline comments are `... # foo`).
                if (i == 0 || s[i - 1] == ' ' || s[i - 1] == '\t')
                    return s.Substring(0, i);
            }
        }
        return s;
    }
}
