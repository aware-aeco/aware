//! Minijinja-based templating for runtime config substitution.
//!
//! Resolves `{{ inputs.X }}`, `{{ <node_id>.<field> }}`, `{{ secrets.<id> }}`,
//! `{{ config.X }}` against a `RenderContext`.
//!
//! Kebab-case node ids (e.g. `tekla-watch`) are auto-translated to underscore
//! form (`tekla_watch`) so they're valid Jinja identifiers; the raw kebab key
//! is also inserted for bracket-syntax access.

use minijinja::{Environment, Value};
use std::collections::BTreeSet;

use serde_json::Map;

use crate::error::AwareError;

#[derive(Default, Debug, Clone)]
pub struct RenderContext {
    pub inputs: serde_json::Value,
    /// Upstream node outputs, keyed by node id (raw kebab and underscore-translated forms both populated).
    pub upstream: Map<String, serde_json::Value>,
    /// Secrets keyed by credential id (e.g. "trimble-connect").
    pub secrets: Map<String, serde_json::Value>,
    /// App-level config kv.
    pub config: Map<String, serde_json::Value>,
    /// Ambient per-run context: `run.id`, `run.date`, `run.operator`. Populated
    /// once at run start (see `context::run_context`) so every node renders the
    /// same values. Always inserted as an object — even when empty — so a
    /// `{{ run.* }}` ref degrades to empty rather than erroring on attribute
    /// access against an undefined `run` (#127).
    pub run: Map<String, serde_json::Value>,
    /// Whether the current `for-each` binding at `upstream["item"]` was drawn
    /// from the `secrets` namespace (`for-each: "{{ secrets.batch }}"`).
    ///
    /// `run_for_each` resolves the collection against the live context and binds
    /// each element under `item`, so a credential reaches a node's params through
    /// `upstream` rather than through `secrets` — where blinding the vault alone
    /// never saw it, and the element went to the trace verbatim (#450, Codex).
    /// Set and restored alongside the `item` binding it describes, so a nested
    /// loop cannot leave it claiming something about the enclosing one.
    pub item_from_vault: bool,
}

/// What a value the credential vault supplied looks like once it is safe to
/// write down. Deliberately readable rather than a fixed-width mask: a dry-run
/// preview is read by a human deciding whether to let the write happen, and
/// `Bearer [redacted]` tells them the header is populated without telling them
/// (or anyone who later reads the trace) what it is populated with.
pub const REDACTED: &str = "[redacted]";

/// Every identifier that appears inside a `{{ … }}` / `{% … %}` block anywhere
/// in `params`.
///
/// Used to decide which KEYS of a credential survive into a trace record. A key
/// the node's own template names is, by construction, already written in the app
/// file — so keeping it reveals nothing the reader could not read there. A key
/// the template does not name is one only the vault knows, which is exactly the
/// case where the key itself can BE the secret.
///
/// A **quoted** span is taken whole, because a bracket lookup names its key
/// verbatim: `secrets.custom['api.key']` addresses the key `api.key`, and
/// splitting it into `api` and `key` would blind the very field the template
/// asked for, previewing it empty (#450, Codex). Outside quotes, a run of
/// `[A-Za-z0-9_-]` is a name, so `secrets['my-api'].access_token` contributes
/// `secrets`, `my-api` and `access_token`.
///
/// Both directions of imprecision are safe, which is why this can stay a scanner
/// rather than a parser: collecting too FEW names only blinds more, and
/// collecting too many only keeps a name the app file already spells out.
pub fn template_identifiers(params: &serde_json::Value) -> BTreeSet<String> {
    /// minijinja's own string-literal escape grammar, mirrored exactly.
    ///
    /// A bracket lookup names its key through a string literal, and minijinja
    /// unescapes that literal before looking the key up: `['api\056key']`
    /// addresses `api.key`. Recording only the source spelling leaves the key
    /// unrecognised as template-named, so it is blinded and the field previews
    /// empty (#450, Codex).
    ///
    /// This mirrors `minijinja::utils::unescape` (2.19) rather than
    /// approximating it — `\" \\ \/ \'` literal, `\b \f \n \r \t`, `\xNN`,
    /// `\uNNNN` with UTF-16 surrogate pairing, and `\NNN` octal up to three
    /// digits, with anything else keeping its backslash. An earlier partial
    /// table read `\0` as NUL where minijinja starts an octal escape, which is
    /// how this went wrong. That module is private so it cannot be called, but
    /// the grammar is finite and the version is pinned by `Cargo.lock`, so
    /// mirroring is bounded work rather than an open-ended chase.
    ///
    /// `None` exactly where minijinja errors. The caller keeps the raw span
    /// alongside any decode, so neither an unknown form nor a decode failure can
    /// widen what gets written down — it can only blind more.
    fn decode_escapes(raw: &str) -> Option<String> {
        // A lone high surrogate awaiting its pair; minijinja refuses any other
        // character while one is outstanding.
        fn push(out: &mut String, pending: u16, c: char) -> Option<()> {
            if pending != 0 {
                return None;
            }
            out.push(c);
            Some(())
        }

        let mut out = String::with_capacity(raw.len());
        let mut pending: u16 = 0;
        let mut chars = raw.chars();
        while let Some(c) = chars.next() {
            if c != '\\' {
                push(&mut out, pending, c)?;
                continue;
            }
            match chars.next()? {
                d @ ('"' | '\\' | '/' | '\'') => push(&mut out, pending, d)?,
                'b' => push(&mut out, pending, '\u{8}')?,
                'f' => push(&mut out, pending, '\u{c}')?,
                'n' => push(&mut out, pending, '\n')?,
                'r' => push(&mut out, pending, '\r')?,
                't' => push(&mut out, pending, '\t')?,
                'u' => {
                    // Short tails are NUL-padded rather than refused, as upstream.
                    let hex: String = chars
                        .by_ref()
                        .chain(std::iter::repeat('\u{0}'))
                        .take(4)
                        .collect();
                    let val = u16::from_str_radix(&hex, 16).ok()?;
                    match (pending, (0xD800..=0xDFFF).contains(&val)) {
                        (0, false) => out.push(char::decode_utf16([val]).next()?.ok()?),
                        (_, false) => return None,
                        (0, true) => pending = val,
                        (prev, true) => {
                            out.push(char::decode_utf16([prev, val]).next()?.ok()?);
                            pending = 0;
                        }
                    }
                }
                'x' => {
                    let hex: String = chars.by_ref().take(2).collect();
                    if hex.len() != 2 {
                        return None;
                    }
                    push(
                        &mut out,
                        pending,
                        u8::from_str_radix(&hex, 16).ok()? as char,
                    )?;
                }
                d @ '0'..='7' => {
                    let mut octal = String::from(d);
                    for _ in 0..2 {
                        match chars.as_str().chars().next() {
                            Some(n) if n.is_digit(8) => {
                                octal.push(n);
                                chars.next();
                            }
                            _ => break,
                        }
                    }
                    push(
                        &mut out,
                        pending,
                        u8::from_str_radix(&octal, 8).ok()? as char,
                    )?;
                }
                other => {
                    push(&mut out, pending, '\\')?;
                    push(&mut out, pending, other)?;
                }
            }
        }
        (pending == 0).then_some(out)
    }

    /// Where an expression ends, skipping any `}}` or `%}` that is INSIDE a
    /// string literal.
    ///
    /// minijinja's lexer treats a delimiter inside a string as data, so
    /// `{{ secrets.custom['api}}key'] }}` is one expression naming the key
    /// `api}}key`. A plain `find` truncated it at the middle `}}`, the key was
    /// never recorded as template-named, and the field previewed empty (#450,
    /// Codex). `normalize_hyphenated_paths` below is quote-aware for the same
    /// reason — this brings the scanner in line with it.
    fn find_terminator(s: &str, close: &str) -> Option<usize> {
        let mut chars = s.char_indices();
        while let Some((i, c)) = chars.next() {
            if c == '\'' || c == '"' {
                // Consume the literal; a backslash escapes the next character,
                // so an escaped quote does not close it.
                while let Some((_, q)) = chars.next() {
                    if q == '\\' {
                        chars.next();
                    } else if q == c {
                        break;
                    }
                }
            } else if s[i..].starts_with(close) {
                return Some(i);
            }
        }
        None
    }

    /// Names inside one expression body, quoted spans kept intact.
    fn collect(expr: &str, out: &mut BTreeSet<String>) {
        let mut bare = String::new();
        let mut chars = expr.chars();
        while let Some(c) = chars.next() {
            match c {
                '\'' | '"' => {
                    if !bare.is_empty() {
                        out.insert(std::mem::take(&mut bare));
                    }
                    // Everything up to the matching quote is one name. A
                    // backslash escapes the next character, so an escaped quote
                    // does not end the span — reading `['it\'s']` as ending at
                    // the middle quote would both miss the key and misread the
                    // rest of the expression.
                    let mut raw = String::new();
                    while let Some(q) = chars.next() {
                        if q == c {
                            break;
                        }
                        raw.push(q);
                        if q == '\\'
                            && let Some(escaped) = chars.next()
                        {
                            raw.push(escaped);
                        }
                    }
                    if !raw.is_empty() {
                        // Both spellings, always: the decode is what minijinja
                        // looks up, and the raw span is the fallback when the
                        // literal is malformed (which minijinja refuses too).
                        if let Some(decoded) = decode_escapes(&raw)
                            && decoded != raw
                        {
                            out.insert(decoded);
                        }
                        out.insert(raw);
                    }
                }
                _ if c.is_alphanumeric() || c == '_' || c == '-' => bare.push(c),
                _ if !bare.is_empty() => {
                    out.insert(std::mem::take(&mut bare));
                }
                _ => {}
            }
        }
        if !bare.is_empty() {
            out.insert(bare);
        }
    }
    fn scan(text: &str, out: &mut BTreeSet<String>) {
        let bytes = text.as_bytes();
        let mut i = 0;
        while i + 1 < bytes.len() {
            // Both delimiters open an expression; `{#` (a comment) has no refs.
            let (close, open_len) = match (bytes[i], bytes[i + 1]) {
                (b'{', b'{') => ("}}", 2),
                (b'{', b'%') => ("%}", 2),
                _ => {
                    i += 1;
                    continue;
                }
            };
            let body_start = i + open_len;
            let Some(rel) = find_terminator(&text[body_start..], close) else {
                return; // unterminated — nothing further is an expression
            };
            collect(&text[body_start..body_start + rel], out);
            i = body_start + rel + close.len();
        }
    }
    fn walk(v: &serde_json::Value, out: &mut BTreeSet<String>) {
        match v {
            serde_json::Value::String(s) => scan(s, out),
            serde_json::Value::Array(items) => items.iter().for_each(|v| walk(v, out)),
            serde_json::Value::Object(fields) => {
                for (k, v) in fields {
                    // A key can carry a template too (`"{{ inputs.h }}": …`).
                    scan(k, out);
                    walk(v, out);
                }
            }
            _ => {}
        }
    }
    let mut out = BTreeSet::new();
    walk(params, &mut out);
    out
}

/// Replace everything a value carries with [`REDACTED`], keeping only the keys
/// named in `keep_keys` and the shape around them.
///
/// EVERY leaf goes — not the subset of field names that hold token material, and
/// not only the strings. A denylist of "sensitive" names would leak the first
/// shape nobody thought to add to it, which is a leak that looks exactly like
/// working code. Over-redacting `token_type` costs a preview the word `Bearer`,
/// which is in the template beside it anyway.
///
/// Numbers and booleans go for the same reason. `access_token` is always a
/// string, so sparing `expires_at` looked free — but `context::load_secret` also
/// takes the raw `<creds_dir>/<id>.json` path, which parses **arbitrary** JSON
/// into the namespace. A hand-written `pin.json` holding `123456` is a
/// credential whose value is a number (#450, Codex).
///
/// `null` is the one exception, and deliberate: it is the *absence* of a value,
/// so there is nothing there to leak, and blinding it would claim the credential
/// carries a hidden `refresh_token` when it carries none.
pub fn blinded(v: &serde_json::Value, keep_keys: &BTreeSet<String>) -> serde_json::Value {
    match v {
        serde_json::Value::Null => serde_json::Value::Null,
        serde_json::Value::Array(items) => {
            serde_json::Value::Array(items.iter().map(|v| blinded(v, keep_keys)).collect())
        }
        serde_json::Value::Object(fields) => serde_json::Value::Object(
            fields
                .iter()
                .map(|(k, v)| {
                    // An unnamed key can itself BE the secret, so it collapses to
                    // one `[redacted]` entry. That costs nothing: a key the
                    // template never names is one no reader was going to use.
                    let key = if keep_keys.contains(k) {
                        k.clone()
                    } else {
                        REDACTED.to_string()
                    };
                    (key, blinded(v, keep_keys))
                })
                .collect(),
        ),
        // String, Number, Bool — every scalar that can carry a value.
        _ => serde_json::Value::String(REDACTED.to_string()),
    }
}

/// The head of the path `expr` reads — the same token [`resolve_value`]
/// dispatches its namespace arm on.
///
/// Deliberately just the head, and deliberately not a full parse. It answers one
/// question: which namespace is this collection drawn from.
pub fn path_head(expr: &str) -> Option<&str> {
    path_segments(expr).first().copied()
}

/// The dotted path `expr` reads, split into segments.
///
/// Bracket syntax collapses into the same list (`upstream['item'].x` yields
/// `upstream`, `item`, `x`). `resolve_value` truncates at `[` and would not
/// resolve that form, so treating it as a path here can only over-taint — which
/// is the safe direction for every caller below.
fn path_segments(expr: &str) -> Vec<&str> {
    let Some(start) = expr.find("{{") else {
        return Vec::new();
    };
    let after = &expr[start + 2..];
    let Some(end) = after.find("}}") else {
        return Vec::new();
    };
    after[..end]
        .trim()
        .split(|c: char| !(c.is_alphanumeric() || c == '_' || c == '-'))
        .filter(|t| !t.is_empty())
        .collect()
}

/// Whether `expr` draws straight from the credential vault.
///
/// `{{ upstream.x }}` where `x` happens to hold a credential is a node's own
/// output, which the trace records in full regardless; claiming to catch that
/// would be a promise this cannot keep.
pub fn reads_secrets_namespace(expr: &str) -> bool {
    path_head(expr) == Some("secrets")
}

/// Whether `expr` draws from the enclosing `for-each` binding.
///
/// A nested `for-each: "{{ item.tokens }}"` inside a loop over the vault selects
/// from a credential the outer loop already lifted out of it, so the inner
/// elements are vault material too — and reading the head alone would call the
/// inner loop clean and write its tokens to the trace (#450, Codex).
pub fn reads_item_binding(expr: &str) -> bool {
    // `upstream.item` too: `render()` exposes the whole upstream map as a
    // top-level object, so that is a supported alias for the same binding, and
    // reading only the head called it a different namespace (#450, Codex).
    matches!(
        path_segments(expr).as_slice(),
        ["item", ..] | ["upstream", "item", ..]
    )
}

impl RenderContext {
    /// Insert a node output, populating both raw kebab and underscore-translated forms.
    pub fn record_output(&mut self, node_id: &str, output: serde_json::Value) {
        self.upstream.insert(node_id.to_string(), output.clone());
        let translated = node_id.replace('-', "_");
        if translated != node_id {
            self.upstream.insert(translated, output);
        }
    }

    /// This context with the `secrets` namespace blinded for a record that will
    /// be written down.
    ///
    /// Rendering a node's params against this yields what the live run would
    /// send, minus anything the credential vault contributed — the only shape of
    /// rendered params that may be persisted. `~/.aware/credentials/` is written
    /// `0600` and `aware credential put` refuses to take a secret from argv
    /// because argv is readable by every process on the machine; a run trace is
    /// a `0644` file that `aware app logs` prints on request, so a rendered
    /// credential reaching one undoes both of those (#448).
    ///
    /// `params` is the node's own **unrendered** template source, and it is here
    /// to decide which credential KEYS survive — see [`template_identifiers`].
    /// Keeping the names the template addresses is what makes the record worth
    /// reading: collapsing a credential to a bare `[redacted]` string closes the
    /// same leak, but attribute access then resolves to nothing and the
    /// documented `Bearer {{ secrets.h.access_token }}` previews as `Bearer `,
    /// which an operator reviewing a proposed write reads as *no credential at
    /// all*. A fixed list of known field names fails the same way on the
    /// repo's own `{{ secrets.teams.coord }}` (#450, Codex).
    ///
    /// The boundary is the vault, and one hop from it. This hides what
    /// `secrets.*` put into the params, and the caller additionally blinds a
    /// `for-each` binding drawn from `secrets`. A credential that reached the
    /// params any further round — computed by a `compare`, or read out of a file
    /// by an upstream node — is that node's *output*, which the trace records in
    /// full regardless of this function and which nothing here can tell from any
    /// other value.
    pub fn with_redacted_secrets(&self, params: &serde_json::Value) -> RenderContext {
        let keep = template_identifiers(params);
        let mut out = RenderContext {
            // The credential ID is a key too, and `load_secret` takes it from the
            // FILE STEM — so a hand-written `sk-live-token.json` puts credential
            // material in the id itself, which a whole-value `{{ secrets }}`
            // writes straight into the record. Same rule as the fields inside:
            // an id the template names is already in the app file (#450, Codex).
            secrets: self
                .secrets
                .iter()
                .map(|(id, v)| {
                    let id = if keep.contains(id) {
                        id.clone()
                    } else {
                        REDACTED.to_string()
                    };
                    (id, blinded(v, &keep))
                })
                .collect(),
            ..self.clone()
        };
        // The one hop: a `for-each` element the runtime itself lifted out of the
        // vault. It sits in `upstream`, not `secrets`, so the walk above cannot
        // reach it — but its provenance is known, which is what separates it
        // from every other upstream value.
        if out.item_from_vault
            && let Some(item) = out.upstream.get("item")
        {
            let safe = blinded(item, &keep);
            out.upstream.insert("item".to_string(), safe);
        }
        out
    }
}

/// Find the byte index just past the next `{% endraw %}` tag at or after `from`,
/// or `None` if the raw block is unterminated. Tolerates whitespace-control
/// dashes (`{%- endraw -%}`).
fn find_endraw(template: &str, from: usize) -> Option<usize> {
    let mut search = from;
    while let Some(open_rel) = template[search..].find("{%") {
        let after_open = search + open_rel + 2;
        let close_rel = template[after_open..].find("%}")?;
        let tag = template[after_open..after_open + close_rel]
            .trim()
            .trim_matches('-')
            .trim();
        let close_end = after_open + close_rel + 2;
        if tag == "endraw" {
            return Some(close_end);
        }
        search = close_end;
    }
    None
}

/// Rewrite hyphenated dot-path segments inside `{{ … }}` / `{% … %}` blocks to
/// bracket form, so kebab-case identifiers work in dot syntax. minijinja lexes
/// `inputs.tc-project-id` as the expression `inputs.tc - project - id`
/// (subtraction over undefined values) and fails; this turns it into
/// `inputs['tc-project-id']`. A hyphenated path BASE — a kebab node id such as
/// `tekla-watch` — is translated to its underscore form (`tekla_watch`), which
/// [`RenderContext::record_output`] also registers, so `tekla-watch.drawing-bytes`
/// becomes `tekla_watch['drawing-bytes']`.
///
/// Only hyphenated dot-paths are touched — exactly the refs that fail today.
/// Hyphen-free paths and the contents of string literals and `{% raw %}` blocks
/// are copied through unchanged, so any template that already renders keeps
/// rendering identically (#117-4).
///
/// Subtraction note: an explicit, spaced `{{ a.b - c }}` is preserved (the space
/// ends the path). A *space-free* `{{ ns.a-b }}` is read as the kebab key `a-b`,
/// matching AWARE's kebab-everywhere convention; write subtraction with spaces.
fn normalize_hyphenated_paths(template: &str) -> String {
    let b = template.as_bytes();
    let n = b.len();
    let mut out = String::with_capacity(n);
    // Start of the pending run of bytes copied through verbatim. Copying by
    // SLICE (not byte-by-byte) keeps multi-byte UTF-8 literal text intact.
    let mut flush_from = 0usize;
    let mut i = 0usize;
    let mut in_expr = false;
    let mut quote: Option<u8> = None;
    let ident = |byte: u8| byte.is_ascii_alphanumeric() || byte == b'_' || byte == b'-';
    while i < n {
        let c = b[i];
        if !in_expr {
            // `{% raw %}` … `{% endraw %}` is emitted literally by minijinja —
            // skip the whole region so its body is never normalized.
            if c == b'{'
                && i + 1 < n
                && b[i + 1] == b'%'
                && let Some(close_rel) = template[i + 2..].find("%}")
                && template[i + 2..i + 2 + close_rel]
                    .trim()
                    .trim_matches('-')
                    .trim()
                    == "raw"
            {
                let body_start = i + 2 + close_rel + 2;
                i = find_endraw(template, body_start).unwrap_or(n);
                continue;
            }
            if c == b'{' && i + 1 < n && (b[i + 1] == b'{' || b[i + 1] == b'%') {
                in_expr = true;
                i += 2;
            } else {
                i += 1;
            }
            continue;
        }
        // Inside a `{{ }}` / `{% %}` block.
        if let Some(q) = quote {
            if c == b'\\' && i + 1 < n {
                i += 2;
            } else {
                if c == q {
                    quote = None;
                }
                i += 1;
            }
            continue;
        }
        if (c == b'}' || c == b'%') && i + 1 < n && b[i + 1] == b'}' {
            in_expr = false;
            i += 2;
            continue;
        }
        if c == b'\'' || c == b'"' {
            quote = Some(c);
            i += 1;
            continue;
        }
        // A potential accessor-chain base: a leading identifier.
        if c.is_ascii_alphabetic() || c == b'_' {
            let base_start = i;
            i += 1;
            while i < n && ident(b[i]) {
                i += 1;
            }
            // Only an accessor chain (base followed by `.` or `[`) is rewritten;
            // a bare token is left alone so `a - b` subtraction is never mangled.
            if i < n && (b[i] == b'.' || b[i] == b'[') {
                // A kebab base node id resolves via its underscore-translated key.
                let mut chain = template[base_start..i].replace('-', "_");
                loop {
                    if i < n && b[i] == b'.' {
                        let seg_start = i + 1;
                        let mut j = seg_start;
                        while j < n && ident(b[j]) {
                            j += 1;
                        }
                        if j == seg_start {
                            break; // `.` not followed by an identifier (e.g. `.*`)
                        }
                        let seg = &template[seg_start..j];
                        if seg.contains('-') {
                            chain.push_str("['");
                            chain.push_str(seg);
                            chain.push_str("']");
                        } else {
                            chain.push('.');
                            chain.push_str(seg);
                        }
                        i = j;
                    } else if i < n && b[i] == b'[' {
                        // Copy a `[ … ]` lookup verbatim (kebab keys already work),
                        // tracking quotes so a `]` inside a string doesn't end it.
                        let br_start = i;
                        i += 1;
                        let mut q2: Option<u8> = None;
                        while i < n {
                            let cc = b[i];
                            if let Some(qq) = q2 {
                                if cc == b'\\' && i + 1 < n {
                                    i += 2;
                                    continue;
                                }
                                if cc == qq {
                                    q2 = None;
                                }
                                i += 1;
                            } else if cc == b'\'' || cc == b'"' {
                                q2 = Some(cc);
                                i += 1;
                            } else if cc == b']' {
                                i += 1;
                                break;
                            } else {
                                i += 1;
                            }
                        }
                        chain.push_str(&template[br_start..i]);
                    } else {
                        break;
                    }
                }
                out.push_str(&template[flush_from..base_start]);
                out.push_str(&chain);
                flush_from = i;
            }
            continue;
        }
        i += 1;
    }
    out.push_str(&template[flush_from..]);
    out
}

pub fn render(template: &str, ctx: &RenderContext) -> Result<String, AwareError> {
    let normalized = normalize_hyphenated_paths(template);
    let mut env = Environment::new();
    env.add_template("t", &normalized)
        .map_err(|e| AwareError::Validation(format!("template parse: {e}")))?;
    let tmpl = env
        .get_template("t")
        .map_err(|e| AwareError::Validation(format!("template load: {e}")))?;

    // Build the flat top-level context.
    let mut top: Map<String, serde_json::Value> = Map::new();
    top.insert("inputs".into(), ctx.inputs.clone());
    top.insert(
        "secrets".into(),
        serde_json::Value::Object(ctx.secrets.clone()),
    );
    top.insert(
        "config".into(),
        serde_json::Value::Object(ctx.config.clone()),
    );
    // Ambient run context. Always an object (possibly empty) so a `{{ run.date }}`
    // ref resolves attribute access against a defined value — an *undefined* `run`
    // errors under minijinja's default behavior, which is the #127 bug.
    top.insert("run".into(), serde_json::Value::Object(ctx.run.clone()));
    // `upstream` object exposes raw kebab keys via bracket syntax:
    //   {{ upstream["tekla-watch"].mark }}
    // Flat top-level underscore-translated keys allow dot notation:
    //   {{ tekla_watch.mark }}
    top.insert(
        "upstream".into(),
        serde_json::Value::Object(ctx.upstream.clone()),
    );
    for (k, v) in &ctx.upstream {
        top.insert(k.clone(), v.clone());
    }

    let value = Value::from_serialize(serde_json::Value::Object(top));
    let rendered = tmpl
        .render(value)
        .map_err(|e| AwareError::Validation(format!("template render: {e}")))?;
    Ok(rendered)
}

/// Resolve a `{{ <head>.<seg>… }}` reference to its structured value (array,
/// object, scalar) rather than a rendered string. The `for-each` runtime needs
/// the underlying [`serde_json::Value`] to iterate — [`render`] would stringify
/// an array into `"[…]"`, which can't be iterated.
///
/// Mirrors the lockfile compiler's reference grammar
/// (`app_lock::collect_refs`): the leading run of identifier / `_` / `-` / `.`
/// characters inside the first `{{ … }}` is taken, split on `.`, and walked.
/// `inputs` / `config` / `secrets` resolve to those namespaces; any other head
/// is an upstream node id (raw kebab or underscore form, matching
/// [`RenderContext::record_output`]). Nested segments are matched verbatim —
/// agent-output fields aren't identifier-translated. A ref that doesn't resolve
/// yields [`serde_json::Value::Null`], which the caller treats as empty.
///
/// Only the dot/kebab path form is resolved — the same subset the compiler
/// validates. A bracket-syntax ref (`{{ src['items'] }}`) stops at `[`, so
/// neither this resolver nor `collect_refs` sees the `items` segment; author
/// `for-each` collections in dot form (`{{ src.items }}`), AWARE's convention.
pub fn resolve_value(expr: &str, ctx: &RenderContext) -> serde_json::Value {
    let null = serde_json::Value::Null;
    let Some(start) = expr.find("{{") else {
        return null;
    };
    let after = &expr[start + 2..];
    let Some(end) = after.find("}}") else {
        return null;
    };
    let inner = after[..end].trim();
    let path_end = inner
        .find(|c: char| !(c.is_alphanumeric() || c == '_' || c == '-' || c == '.'))
        .unwrap_or(inner.len());
    let parts: Vec<&str> = inner[..path_end]
        .split('.')
        .filter(|p| !p.is_empty())
        .collect();
    let Some((head, rest)) = parts.split_first() else {
        return null;
    };

    let mut cur = match *head {
        // Base inputs. (The streaming per-event `inputs` overlay is applied only in
        // the config-rendering context — see `orchestrator::render_config` — so that
        // for-each / compare, which also call this resolver, keep reading app inputs
        // rather than a left-over event overlay. #205 Codex.)
        "inputs" => ctx.inputs.clone(),
        "config" => serde_json::Value::Object(ctx.config.clone()),
        "secrets" => serde_json::Value::Object(ctx.secrets.clone()),
        // Ambient run namespace — but an upstream node literally named `run`
        // takes precedence, mirroring `render()` (where the upstream loop
        // overwrites the `run` key). This keeps a pre-existing `{{ run.* }}`
        // node reference resolving to that node instead of being shadowed.
        "run" => ctx
            .upstream
            .get("run")
            .cloned()
            .unwrap_or_else(|| serde_json::Value::Object(ctx.run.clone())),
        // `upstream` namespace — render() exposes the whole upstream map as a
        // top-level object, so `{{ upstream.<node>.<field> }}` resolves against it. An
        // upstream node literally named `upstream` takes precedence (render()'s upstream
        // loop would overwrite the key), mirroring the `run` arm. Without this a
        // whole-value `{{ upstream.x.y }}` config leaf would treat `upstream` as a
        // missing node id and resolve to null (#205 Codex).
        //
        // Guard on a non-empty `rest`: a bracket ref `{{ upstream['n'].x }}` truncates
        // at `[` to just `upstream` (no rest). Returning the whole map there would make
        // a `for-each` iterate every node; instead let it fall through to the node-id
        // arm (→ null), matching the pre-#205 behavior. (Bracket refs never reach here
        // as config — they aren't bare-path, so `is_whole_value_template` rejects them
        // and they render; only for-each / compare pass brackets to this resolver.)
        "upstream" if !rest.is_empty() => ctx
            .upstream
            .get("upstream")
            .cloned()
            .unwrap_or_else(|| serde_json::Value::Object(ctx.upstream.clone())),
        // Upstream node id — `record_output` stores raw kebab and underscore forms.
        _ => ctx
            .upstream
            .get(*head)
            .or_else(|| ctx.upstream.get(&head.replace('-', "_")))
            .or_else(|| ctx.upstream.get(&head.replace('_', "-")))
            .cloned()
            .unwrap_or(null),
    };
    for seg in rest {
        cur = cur.get(seg).cloned().unwrap_or(serde_json::Value::Null);
    }
    cur
}

/// True when `s` is *exactly* one `{{ <bare-path> }}` expression after trimming —
/// no literal text, no second expression, and the inner content is a bare
/// dot/kebab path (the grammar [`resolve_value`] walks), e.g. `"{{ projects.body }}"`.
/// Such a "whole-value" config leaf resolves to its structured value via
/// [`resolve_value`] rather than being stringified by [`render`] (#205).
///
/// Anything else stays a rendered string: an embedded template (`"file:{{ x }}.pdf"`),
/// a multi-expression string (`"{{ a }}{{ b }}"`), a single expression that carries a
/// **filter, bracket/index, or call** (`"{{ x | f }}"`, `"{{ s[0] }}"`,
/// `"{{ upstream['n'].x }}"`), or a **literal** (`"{{ 123 }}"`, `"{{ true }}"`). Those
/// are excluded because `resolve_value` would silently drop the tail (filters/brackets)
/// or treat a literal as a missing node id (→ null); letting them fall through to
/// `render` preserves minijinja's correct (or loudly-erroring) handling (review).
pub fn is_whole_value_template(s: &str) -> bool {
    let t = s.trim();
    if t.len() < 4
        || !t.starts_with("{{")
        || !t.ends_with("}}")
        || t.matches("{{").count() != 1
        || t.matches("}}").count() != 1
    {
        return false;
    }
    // `strip_prefix`/`strip_suffix` rather than `t[2..t.len() - 2]`: hard-coded
    // byte offsets into a `str` panic when they land mid-character (see
    // `crate::text`). The guard above already proves both delimiters are there,
    // so the strips always succeed — this just removes the arithmetic that would
    // have to be re-audited if the guard ever changed.
    let inner = t
        .strip_prefix("{{")
        .and_then(|rest| rest.strip_suffix("}}"))
        .unwrap_or("")
        .trim();
    // Must be a bare path over exactly the charset `resolve_value` walks …
    if inner.is_empty()
        || !inner
            .chars()
            .all(|c| c.is_alphanumeric() || c == '_' || c == '-' || c == '.')
    {
        return false;
    }
    // … whose head is an identifier — a number literal (`123`, `1.5`) starts with a
    // digit and is not a path — and which isn't a Minijinja literal keyword. Such
    // literals must keep going through `render` so they evaluate, rather than resolve
    // to a null "node id" (#205 Codex).
    let Some(first) = inner.chars().next() else {
        return false;
    };
    (first.is_alphabetic() || first == '_')
        && !matches!(
            inner.to_ascii_lowercase().as_str(),
            "true" | "false" | "none" | "null"
        )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ctx_with_upstream(node_id: &str, output: serde_json::Value) -> RenderContext {
        let mut ctx = RenderContext::default();
        ctx.record_output(node_id, output);
        ctx
    }

    #[test]
    fn substitutes_inputs() {
        let ctx = RenderContext {
            inputs: serde_json::json!({ "tc-project-id": "proj-123" }),
            ..Default::default()
        };
        let out = render("{{ inputs['tc-project-id'] }}", &ctx).unwrap();
        assert_eq!(out, "proj-123");
    }

    #[test]
    fn is_whole_value_template_detects_pure_expression() {
        assert!(is_whole_value_template("{{ projects.body }}"));
        assert!(is_whole_value_template("  {{ x }}  ")); // surrounding whitespace ok
        // Embedded / decorated / multi-expression strings are NOT whole-value:
        assert!(!is_whole_value_template("file:{{ x }}.pdf"));
        assert!(!is_whole_value_template("{{ a }}{{ b }}"));
        assert!(!is_whole_value_template("{{ a }} text"));
        assert!(!is_whole_value_template("text {{ a }}"));
        assert!(!is_whole_value_template("plain string"));
        // A single expression that carries a filter / bracket / index is NOT a bare
        // path — `resolve_value` would silently drop the tail, so it stays a rendered
        // string (review, #205):
        assert!(!is_whole_value_template("{{ x | rowCount }}"));
        assert!(!is_whole_value_template("{{ src.items[0] }}"));
        assert!(!is_whole_value_template("{{ upstream['node-id'].field }}"));
        assert!(!is_whole_value_template("{{}}")); // empty expression
    }

    #[test]
    fn whole_value_template_resolves_structured_array_not_string() {
        let ctx = ctx_with_upstream(
            "projects",
            serde_json::json!({ "body": [ { "id": "p1" }, { "id": "p2" } ] }),
        );
        // resolve_value yields the array structurally (#205) …
        let v = resolve_value("{{ projects.body }}", &ctx);
        assert!(v.is_array() && v.as_array().unwrap().len() == 2);
        // … whereas render would stringify it.
        let s = render("{{ projects.body }}", &ctx).unwrap();
        assert!(s.trim_start().starts_with('[') && s.contains("p1"));
    }

    #[test]
    fn resolve_value_inputs_reads_base_not_streaming_overlay() {
        // resolve_value (used by for-each / compare) reads BASE inputs, NOT the
        // streaming `upstream["inputs"]` overlay — the overlay is applied only in
        // render_config, so a left-over event overlay can't shadow app inputs in a
        // primitive (#205 Codex).
        let mut ctx = RenderContext {
            inputs: serde_json::json!({ "x": "base" }),
            ..Default::default()
        };
        ctx.record_output("inputs", serde_json::json!({ "x": "overlay", "evt": "e" }));
        assert_eq!(
            resolve_value("{{ inputs.x }}", &ctx),
            serde_json::json!("base")
        );
        assert!(resolve_value("{{ inputs.evt }}", &ctx).is_null());
    }

    #[test]
    fn is_whole_value_template_rejects_literals() {
        // Minijinja literals must keep going through `render` (resolve_value would
        // treat them as missing node ids → null). #205 Codex.
        assert!(!is_whole_value_template("{{ true }}"));
        assert!(!is_whole_value_template("{{ false }}"));
        assert!(!is_whole_value_template("{{ none }}"));
        assert!(!is_whole_value_template("{{ null }}"));
        assert!(!is_whole_value_template("{{ 123 }}"));
        assert!(!is_whole_value_template("{{ 1.5 }}"));
        // A bare node id (identifier head) is still whole-value.
        assert!(is_whole_value_template("{{ projects }}"));
        assert!(is_whole_value_template("{{ _internal }}"));
    }

    #[test]
    fn resolve_value_handles_upstream_namespace() {
        // render() exposes `upstream` as a top-level object; resolve_value must too, so
        // a whole-value `{{ upstream.<node>.<field> }}` config leaf resolves rather than
        // treating `upstream` as a missing node id (#205 Codex).
        let ctx = ctx_with_upstream("src", serde_json::json!({ "items": [1, 2, 3] }));
        // Dot form navigates the map …
        assert_eq!(
            resolve_value("{{ upstream.src.items }}", &ctx),
            serde_json::json!([1, 2, 3])
        );
        // … but a bracket ref (the parser truncates at `[` → no rest) must NOT return
        // the whole map — that would make a for-each iterate every node. It resolves to
        // null, the pre-#205 behavior. A bare `{{ upstream }}` is null too (#205 Codex).
        assert!(resolve_value("{{ upstream['src'].items }}", &ctx).is_null());
        assert!(resolve_value("{{ upstream }}", &ctx).is_null());
    }

    #[test]
    fn substitutes_upstream_underscore_translated() {
        let ctx = ctx_with_upstream("tekla-watch", serde_json::json!({ "mark": "A-104" }));
        let out = render("file:{{ tekla_watch.mark }}.pdf", &ctx).unwrap();
        assert_eq!(out, "file:A-104.pdf");
    }

    #[test]
    fn substitutes_upstream_bracket_syntax_raw_kebab() {
        // minijinja bracket syntax for top-level dict keys with hyphens:
        // use the `upstream` object which preserves raw kebab-case keys.
        // `{{ ['tekla-watch'].mark }}` would be parsed as an array literal by
        // minijinja (not a key lookup), so the correct form is shown below.
        let ctx = ctx_with_upstream("tekla-watch", serde_json::json!({ "mark": "A-104" }));
        let out = render("{{ upstream['tekla-watch'].mark }}", &ctx).unwrap();
        assert_eq!(out, "A-104");
    }

    #[test]
    fn substitutes_secrets() {
        let mut ctx = RenderContext::default();
        ctx.secrets.insert(
            "trimble-connect".into(),
            serde_json::json!({ "token": "tk_abc" }),
        );
        let out = render("Bearer {{ secrets['trimble-connect'].token }}", &ctx).unwrap();
        assert_eq!(out, "Bearer tk_abc");
    }

    #[test]
    fn substitutes_config() {
        let mut ctx = RenderContext::default();
        ctx.config
            .insert("pdf-inbox".into(), serde_json::json!("/tmp/inbox"));
        let out = render("path={{ config['pdf-inbox'] }}", &ctx).unwrap();
        assert_eq!(out, "path=/tmp/inbox");
    }

    // ── #127: the documented `{{ run.* }}` namespace renders ────────────────

    #[test]
    fn substitutes_run_namespace() {
        let mut ctx = RenderContext::default();
        ctx.run.insert("id".into(), serde_json::json!("run-abc"));
        ctx.run
            .insert("date".into(), serde_json::json!("2026-05-21"));
        ctx.run
            .insert("operator".into(), serde_json::json!("p.lisowski"));
        assert_eq!(
            render("audit-{{ run.date }}.html", &ctx).unwrap(),
            "audit-2026-05-21.html"
        );
        assert_eq!(render("{{ run.id }}", &ctx).unwrap(), "run-abc");
        assert_eq!(
            render("by {{ run.operator }}", &ctx).unwrap(),
            "by p.lisowski"
        );
    }

    #[test]
    fn run_ref_with_empty_namespace_renders_empty_not_error() {
        // The #127 failure: an *undefined* `run` makes `{{ run.date }}` error on
        // attribute access. With `run` always inserted as an object, a missing
        // sub-key degrades to empty (lenient), exactly like inputs/config/secrets.
        let ctx = RenderContext::default();
        let out = render("audit-{{ run.date }}.html", &ctx);
        assert_eq!(out.unwrap(), "audit-.html");
    }

    #[test]
    fn resolve_value_reads_run_namespace() {
        let mut ctx = RenderContext::default();
        ctx.run.insert("id".into(), serde_json::json!("run-xyz"));
        assert_eq!(
            resolve_value("{{ run.id }}", &ctx),
            serde_json::json!("run-xyz")
        );
    }

    #[test]
    fn upstream_node_named_run_is_not_shadowed_by_ambient_run() {
        // Codex P2: a node `id: run` feeding `for-each: {{ run.items }}` must keep
        // resolving to the node output, not the ambient run namespace (which has
        // no `items` → Null → loop skipped). An upstream `run` wins over ambient.
        let mut ctx = ctx_with_upstream("run", serde_json::json!({ "items": [1, 2] }));
        ctx.run
            .insert("date".into(), serde_json::json!("2026-05-21"));
        assert_eq!(
            resolve_value("{{ run.items }}", &ctx),
            serde_json::json!([1, 2])
        );
        // With no upstream node named `run`, the ambient namespace is used.
        let ambient_only = RenderContext {
            run: ctx.run.clone(),
            ..Default::default()
        };
        assert_eq!(
            resolve_value("{{ run.date }}", &ambient_only),
            serde_json::json!("2026-05-21")
        );
    }

    #[test]
    fn passes_through_literal_strings() {
        let ctx = RenderContext::default();
        let out = render("no templating here", &ctx).unwrap();
        assert_eq!(out, "no templating here");
    }

    #[test]
    fn a_bare_undefined_name_renders_empty_but_a_path_through_one_errors() {
        // minijinja's default lenient mode renders an undefined *name* as the
        // empty string, but attribute access *through* an undefined value is
        // an error — that asymmetry is the whole reason `render` seeds `run`,
        // `inputs`, `secrets`, `config` and `upstream` as objects (#127).
        let ctx = RenderContext::default();
        assert_eq!(render("a={{ nonexistent }}!", &ctx).unwrap(), "a=!");
        let err = render("a={{ nonexistent.deep }}!", &ctx).unwrap_err();
        assert!(
            matches!(&err, AwareError::Validation(m) if m.contains("undefined value")),
            "expected a render error on the undefined path, got {err:?}"
        );
    }

    #[test]
    fn ambient_context_roots_stay_addressable_when_the_run_supplied_nothing() {
        // The #127 regression shape: a node template says `{{ run.date }}` and
        // the run carries no ambient context. Each root must still be a defined
        // (empty) object, so the reference degrades to empty instead of failing
        // the whole run with "undefined value".
        //
        // `run` alone is also covered by `run_ref_with_empty_namespace_renders
        // _empty_not_error` below; the other four roots had no empty-context
        // guard, and each is seeded by its own line in `render`.
        let ctx = RenderContext::default();
        for template in [
            "d={{ run.date }}!",
            "i={{ inputs.missing }}!",
            "s={{ secrets.absent }}!",
            "c={{ config.absent }}!",
            "u={{ upstream[\"no-such-node\"] }}!",
        ] {
            let rendered = render(template, &ctx)
                .unwrap_or_else(|e| panic!("`{template}` must not error, got {e:?}"));
            let (prefix, _) = template.split_once('=').expect("probe template");
            assert_eq!(rendered, format!("{prefix}=!"), "for `{template}`");
        }
    }

    // ── #117-4: hyphenated dot-paths now render via the normalizer ──────────

    // ── #124: resolve_value returns structured values for for-each ──────────

    #[test]
    fn resolve_value_returns_upstream_array() {
        let ctx = ctx_with_upstream("src", serde_json::json!({ "items": [1, 2, 3] }));
        assert_eq!(
            resolve_value("{{ src.items }}", &ctx),
            serde_json::json!([1, 2, 3])
        );
    }

    #[test]
    fn resolve_value_resolves_kebab_node_head() {
        // record_output stores both kebab and underscore forms; a `{{ a_b.x }}`
        // ref must resolve the kebab node `a-b`.
        let ctx = ctx_with_upstream("a-b", serde_json::json!({ "x": [true] }));
        assert_eq!(
            resolve_value("{{ a_b.x }}", &ctx),
            serde_json::json!([true])
        );
    }

    #[test]
    fn resolve_value_missing_ref_is_null() {
        let ctx = RenderContext::default();
        assert!(resolve_value("{{ nope.items }}", &ctx).is_null());
    }

    #[test]
    fn resolve_value_reads_inputs_namespace() {
        let ctx = RenderContext {
            inputs: serde_json::json!({ "ids": ["a", "b"] }),
            ..Default::default()
        };
        assert_eq!(
            resolve_value("{{ inputs.ids }}", &ctx),
            serde_json::json!(["a", "b"])
        );
    }

    #[test]
    fn dot_syntax_hyphenated_input_now_renders() {
        let ctx = RenderContext {
            inputs: serde_json::json!({ "tc-project-id": "proj-123" }),
            ..Default::default()
        };
        let out = render("{{ inputs.tc-project-id }}", &ctx).unwrap();
        assert_eq!(out, "proj-123");
    }

    #[test]
    fn dot_syntax_hyphenated_node_id_and_field_now_renders() {
        let ctx = ctx_with_upstream("tekla-watch", serde_json::json!({ "drawing-bytes": "PDF" }));
        let out = render("{{ tekla-watch.drawing-bytes }}", &ctx).unwrap();
        assert_eq!(out, "PDF");
    }

    #[test]
    fn dot_syntax_hyphenated_config_and_secrets_render() {
        let mut ctx = RenderContext::default();
        ctx.config
            .insert("pdf-inbox".into(), serde_json::json!("/in"));
        ctx.secrets
            .insert("ceng-seal".into(), serde_json::json!("sig"));
        assert_eq!(render("{{ config.pdf-inbox }}", &ctx).unwrap(), "/in");
        assert_eq!(render("{{ secrets.ceng-seal }}", &ctx).unwrap(), "sig");
    }

    #[test]
    fn hyphenated_path_inside_larger_string_renders() {
        let ctx = RenderContext {
            inputs: serde_json::json!({ "report-dir": "/r" }),
            ..Default::default()
        };
        let out = render(
            "{{ inputs.report-dir }}/monday-{{ inputs.report-dir }}.html",
            &ctx,
        )
        .unwrap();
        assert_eq!(out, "/r/monday-/r.html");
    }

    #[test]
    fn normalize_leaves_valid_templates_untouched() {
        // Hyphen-free dot paths, subtraction (no dot), already-bracketed kebab,
        // and literal text outside blocks all pass through byte-identical.
        assert_eq!(
            normalize_hyphenated_paths("{{ inputs.plain }}"),
            "{{ inputs.plain }}"
        );
        assert_eq!(
            normalize_hyphenated_paths("{{ tekla_watch.mark }}"),
            "{{ tekla_watch.mark }}"
        );
        assert_eq!(normalize_hyphenated_paths("{{ a - b }}"), "{{ a - b }}");
        assert_eq!(
            normalize_hyphenated_paths("{{ inputs['a-b'] }}"),
            "{{ inputs['a-b'] }}"
        );
        assert_eq!(
            normalize_hyphenated_paths("a-b.c-d not in a block"),
            "a-b.c-d not in a block"
        );
    }

    #[test]
    fn normalize_rewrites_hyphenated_dot_paths_to_bracket_form() {
        assert_eq!(
            normalize_hyphenated_paths("{{ inputs.tc-project-id }}"),
            "{{ inputs['tc-project-id'] }}"
        );
        assert_eq!(
            normalize_hyphenated_paths("{{ delta-report.output-path }}"),
            "{{ delta_report['output-path'] }}"
        );
        // Path rewritten; the hyphen inside the quoted literal is preserved.
        assert_eq!(
            normalize_hyphenated_paths("{{ x.a-b | replace: \".h-t\" }}"),
            "{{ x['a-b'] | replace: \".h-t\" }}"
        );
    }

    #[test]
    fn normalizer_preserves_non_ascii_literal_text() {
        // Byte-by-byte copy would mojibake multi-byte UTF-8; slice copy keeps it.
        let ctx = RenderContext {
            inputs: serde_json::json!({ "name": "World" }),
            ..Default::default()
        };
        let out = render("Zażółć {{ inputs.name }} gęślą jaźń", &ctx).unwrap();
        assert_eq!(out, "Zażółć World gęślą jaźń");
    }

    #[test]
    fn hyphenated_field_after_bracket_lookup_renders() {
        // `upstream['kebab-id'].kebab-field` — the field follows `]`, not a base.
        let ctx = ctx_with_upstream("tekla-watch", serde_json::json!({ "drawing-bytes": "PDF" }));
        let out = render("{{ upstream['tekla-watch'].drawing-bytes }}", &ctx).unwrap();
        assert_eq!(out, "PDF");
    }

    #[test]
    fn normalize_brackets_field_after_bracket_and_keeps_utf8() {
        assert_eq!(
            normalize_hyphenated_paths("{{ upstream['a-b'].c-d }}"),
            "{{ upstream['a-b']['c-d'] }}"
        );
        // Non-ASCII literal outside the block is byte-preserved.
        assert_eq!(
            normalize_hyphenated_paths("café {{ x.y-z }}"),
            "café {{ x['y-z'] }}"
        );
    }

    #[test]
    fn normalize_skips_raw_blocks() {
        // `{% raw %}` body is emitted literally — no normalization inside.
        assert_eq!(
            normalize_hyphenated_paths("{% raw %}{{ inputs.foo-bar }}{% endraw %}"),
            "{% raw %}{{ inputs.foo-bar }}{% endraw %}"
        );
        // A real ref after the raw block is still normalized.
        assert_eq!(
            normalize_hyphenated_paths("{% raw %}{{ a.b-c }}{% endraw %} {{ x.y-z }}"),
            "{% raw %}{{ a.b-c }}{% endraw %} {{ x['y-z'] }}"
        );
    }

    #[test]
    fn normalize_preserves_spaced_subtraction() {
        // Explicit, spaced subtraction is left intact (the space ends the path).
        assert_eq!(
            normalize_hyphenated_paths("{{ inputs.count - discount }}"),
            "{{ inputs.count - discount }}"
        );
    }

    // ── #448 / #450: the secrets namespace, blinded for anything written down ─

    /// One context holding a credential in each shape the store produces — a
    /// `StoredToken` object, a bare string, a bare number, a custom-field object
    /// — plus a value in every OTHER namespace, to prove the blinding is
    /// confined to `secrets`.
    fn ctx_with_secrets() -> RenderContext {
        let mut ctx = RenderContext::default();
        ctx.secrets.insert(
            "my-api".into(),
            serde_json::json!({
                "access_token": "sk-live-abc",
                "refresh_token": serde_json::Value::Null,
                "token_type": "Bearer",
                "expires_at": 0,
                "obtained_at": 1_700_000_000i64,
                "rotatable": true,
                "nested": { "deep": ["sk-inner"] },
                // An unnamed key where the key IS the secret material.
                "987654": true,
            }),
        );
        // The raw `<creds_dir>/<id>.json` path parses arbitrary JSON into this
        // namespace, so a credential whose value is a NUMBER is reachable —
        // a hand-written PIN, bare or in a field (#450, Codex).
        ctx.secrets.insert("pin".into(), serde_json::json!(123_456));
        // `load_secret` takes the ID from the FILE STEM, so a hand-written
        // `sk-live-….json` puts credential material in the id (#450, Codex).
        ctx.secrets
            .insert("sk-live-in-the-id".into(), serde_json::json!("tok"));
        ctx.secrets
            .insert("pinobj".into(), serde_json::json!({ "token": 987_654 }));
        ctx.secrets
            .insert("legacy".into(), serde_json::json!("sk-bare"));
        // The repo's own documented custom shape: `{{ secrets.teams.coord }}`
        // in `revit-2026/commands/link.reload-all.md`.
        ctx.secrets.insert(
            "teams".into(),
            serde_json::json!({ "coord": "19:abc@thread", "unnamed": "sk-other" }),
        );
        ctx.inputs = serde_json::json!({ "phase": "design" });
        ctx.record_output("src", serde_json::json!({ "id": "p1" }));
        ctx.config.insert("region".into(), serde_json::json!("EU"));
        ctx.run
            .insert("operator".into(), serde_json::json!("pawel"));
        ctx
    }

    /// The params a node would carry. Passed to `with_redacted_secrets` because
    /// the template is what decides which credential KEYS survive.
    fn params() -> serde_json::Value {
        serde_json::json!({
            "authorization": "Bearer {{ secrets['my-api'].access_token }}",
            "channel-id": "{{ secrets.teams.coord }}",
            "whole": "{{ secrets.pin }}",
            "legacy": "{{ secrets.legacy }}",
            "from-field": "{{ secrets.pinobj.token }}",
            "vaulted": "{{ secrets.vault }}",
        })
    }

    #[test]
    fn with_redacted_secrets_blinds_every_value_at_every_depth() {
        let blinded = ctx_with_secrets().with_redacted_secrets(&params());
        let api = &blinded.secrets["my-api"];
        // Token material, obviously …
        assert_eq!(api["access_token"], REDACTED);
        // … and the value that merely describes it. Blinding every value is what
        // makes this safe against a credential shape nobody enumerated; a
        // denylist of "sensitive" names would leak the first one it missed.
        assert_eq!(api[REDACTED], REDACTED);
        // Scalars of every type, because the raw credential-file path parses
        // arbitrary JSON: sparing numbers wrote a hand-written PIN to the trace
        // verbatim, bare and in a field alike (#450, Codex).
        assert_eq!(blinded.secrets["pin"], REDACTED);
        // `token` is named by the params, so the KEY survives and its numeric
        // value is what gets blinded — the two rules acting independently.
        assert_eq!(blinded.secrets["pinobj"]["token"], REDACTED);
        // A bare-string secret is a value, not a map — blinded too, not skipped
        // for having no fields to walk.
        assert_eq!(blinded.secrets["legacy"], REDACTED);
        // `null` alone survives: it is the ABSENCE of a value, so it hides
        // nothing, and blinding it would claim a refresh token this credential
        // does not have.
        assert!(api["refresh_token"].is_null());
    }

    #[test]
    fn with_redacted_secrets_keeps_only_the_keys_the_template_names() {
        // Values alone were not enough: a hand-written credential can put the
        // secret in the KEY, and `{{ secrets.<id> }}` resolves the object
        // structurally into the record (#450, Codex).
        let blinded = ctx_with_secrets().with_redacted_secrets(&params());
        let api = &blinded.secrets["my-api"];
        assert!(
            api.get("987654").is_none(),
            "a key the template never names can BE the secret: {api}"
        );
        assert_eq!(api[REDACTED], REDACTED, "it is blinded, not dropped");
        // Nested objects are walked the same way.
        assert!(
            api["nested"].is_null(),
            "`nested` is not named either: {api}"
        );

        // What the template DOES name survives, and that is the whole point: it
        // is what keeps `{{ secrets.h.access_token }}` resolving, so the header
        // previews as `Bearer [redacted]` rather than `Bearer `, which an
        // operator would read as no credential at all.
        assert!(api.get("access_token").is_some());
        // Including a CUSTOM field name no fixed list would have held — the
        // repo's own `{{ secrets.teams.coord }}`. An allowlist of `StoredToken`
        // fields renamed this one and previewed an empty `channel-id`.
        assert_eq!(blinded.secrets["teams"]["coord"], REDACTED);
        assert!(
            blinded.secrets["teams"].get("unnamed").is_none(),
            "a sibling key the template does not name still goes"
        );
    }

    #[test]
    fn with_redacted_secrets_blinds_a_for_each_binding_drawn_from_the_vault() {
        // `run_for_each` lifts elements out of `secrets` into `upstream["item"]`,
        // so blinding the vault alone never saw them (#450, Codex).
        let mut ctx = ctx_with_secrets();
        ctx.record_output("item", serde_json::json!("sk-batch-1"));
        let loop_params = serde_json::json!({ "body": "{{ item }}" });

        // Not flagged → an ordinary loop element, previewed as itself.
        assert_eq!(
            ctx.with_redacted_secrets(&loop_params).upstream["item"],
            "sk-batch-1"
        );

        ctx.item_from_vault = true;
        let blinded = ctx.with_redacted_secrets(&loop_params);
        assert_eq!(blinded.upstream["item"], REDACTED);
        // Only that binding: every other upstream value is a node output, which
        // the trace records in full regardless.
        assert_eq!(blinded.upstream["src"]["id"], "p1");
    }

    #[test]
    fn reads_secrets_namespace_keys_on_the_path_head() {
        assert!(reads_secrets_namespace("{{ secrets.batch }}"));
        assert!(reads_secrets_namespace("{{ secrets['my-api'].pages }}"));
        // A node output that happens to hold a credential is NOT claimed — the
        // trace records node outputs in full regardless, and pretending
        // otherwise would be a promise this cannot keep.
        assert!(!reads_secrets_namespace("{{ upstream.secrets }}"));
        assert!(!reads_secrets_namespace("{{ inputs.rows }}"));
        assert!(!reads_secrets_namespace("no template here"));
        assert!(!reads_secrets_namespace("{{ unterminated"));
    }

    #[test]
    fn reads_item_binding_spots_a_nested_loop_over_the_enclosing_element() {
        // An inner `for-each` selecting out of the outer element carries the
        // outer loop's provenance with it; keying on this loop's own head alone
        // called a nested vault loop clean (#450, Codex).
        assert!(reads_item_binding("{{ item.tokens }}"));
        assert!(reads_item_binding("{{ item }}"));
        // `upstream.item` is a supported alias for the SAME binding — `render`
        // exposes the whole upstream map at the top level — so reading only the
        // head called it a different namespace and cleared the provenance
        // (#450, Codex).
        assert!(reads_item_binding("{{ upstream.item.tokens }}"));
        assert!(reads_item_binding("{{ upstream['item'].tokens }}"));
        // A sibling collection from anywhere else does NOT inherit it.
        assert!(!reads_item_binding("{{ inputs.rows }}"));
        assert!(!reads_item_binding("{{ upstream.src.items }}"));
        // A node whose id merely starts with the same letters is not the binding.
        assert!(!reads_item_binding("{{ itemized.rows }}"));
        assert!(!reads_item_binding("{{ upstream.items.rows }}"));
    }

    #[test]
    fn decode_escapes_matches_minijinjas_string_literal_grammar() {
        // Mirrored from `minijinja::utils::unescape` (2.19), which is private and
        // so cannot be called. Every form it accepts is pinned here, because an
        // escape this decoder gets WRONG renames the key minijinja looks up, and
        // the field then previews empty (#450, Codex).
        let ids =
            |lit: &str| template_identifiers(&serde_json::json!(format!("{{{{ s['{lit}'] }}}}")));

        // Octal, up to three digits — and its first digit is NOT a NUL escape,
        // which is exactly what the earlier partial table got wrong.
        assert!(ids(r"api\056key").contains("api.key"));
        // Hex, and the simple forms including the two the partial table missed.
        assert!(ids(r"api\x2ekey").contains("api.key"));
        assert!(ids(r"a\tb").contains("a\tb"));
        assert!(ids(r"a\bc").contains("a\u{8}c"));
        assert!(ids(r"a\fc").contains("a\u{c}c"));
        assert!(ids(r"a\/b").contains("a/b"));
        // `\uNNNN`, and a UTF-16 surrogate PAIR — which a decoder handling one
        // code unit at a time rejects outright.
        assert!(ids(r"pa\u0067e").contains("page"));
        assert!(ids(r"e\ud83d\ude00j").contains("e\u{1f600}j"));
        // An unknown escape keeps its backslash, as upstream does.
        assert!(ids(r"a\qb").contains(r"a\qb"));
        // A malformed literal (an unpaired surrogate) decodes to nothing — but
        // the RAW span is still recorded, so a decode failure can only ever
        // blind more, never less.
        assert!(ids(r"lone\ud83dx").contains(r"lone\ud83dx"));
    }

    #[test]
    fn template_identifiers_collects_names_from_every_expression_form() {
        let ids = template_identifiers(&serde_json::json!({
            "a": "Bearer {{ secrets['my-api'].access_token }}",
            "b": ["{% if secrets.teams.coord %}x{% endif %}"],
            // A bracket lookup names its key VERBATIM, punctuation included.
            "c": "{{ secrets.custom['api.key'] }}",
            "d": 7,
        }));
        for name in ["secrets", "my-api", "access_token", "teams", "coord"] {
            assert!(ids.contains(name), "missing {name}: {ids:?}");
        }
        // The whole quoted span, not its pieces: splitting `api.key` on the dot
        // would blind the very field the template asked for, and the preview
        // would show it empty — the missing-credential read this exists to avoid
        // (#450, Codex).
        assert!(ids.contains("api.key"), "dotted bracket key: {ids:?}");

        // An ESCAPED key: minijinja decodes the literal and looks up `api.key`,
        // so the source spelling alone would miss it. Both are recorded, so an
        // escape form this does not know still stays covered by the raw span
        // (#450, Codex).
        let escaped = template_identifiers(&serde_json::json!({
            "a": r"{{ secrets.custom['api\x2ekey'] }}",
            "b": r"{{ secrets.custom['pa\u0067e'] }}",
            // An escaped quote must not end the span early.
            "c": r"{{ secrets.custom['it\'s'] }}",
        }));
        assert!(escaped.contains("api.key"), "\\x escape: {escaped:?}");
        assert!(escaped.contains("page"), "\\u escape: {escaped:?}");
        assert!(escaped.contains("it's"), "escaped quote: {escaped:?}");
        // The raw spelling is kept too — belt and braces for an escape form
        // this decoder does not know.
        assert!(escaped.contains(r"api\x2ekey"));
        // Literal text outside an expression is not a reference, so it cannot
        // keep a credential key alive.
        assert!(!ids.contains("Bearer"));
    }

    #[test]
    fn with_redacted_secrets_blinds_credential_ids_the_template_does_not_name() {
        // A whole-value `{{ secrets }}` puts the entire vault MAP in the record,
        // and its keys are credential ids taken from file stems — so an id can
        // itself be the secret. Same rule as the fields inside (#450, Codex).
        let blinded = ctx_with_secrets().with_redacted_secrets(&params());
        assert!(
            blinded.secrets.get("sk-live-in-the-id").is_none(),
            "an id the template never names must not survive: {:?}",
            blinded.secrets.keys().collect::<Vec<_>>()
        );
        assert!(blinded.secrets.contains_key(REDACTED));
        // The ids the template DOES name stay, or every reference would break.
        for named in ["my-api", "teams", "pin"] {
            assert!(
                blinded.secrets.contains_key(named),
                "{named} is named by the params and must survive"
            );
        }
    }

    #[test]
    fn template_identifiers_finds_the_terminator_outside_quoted_literals() {
        // minijinja's lexer treats `}}` inside a string as data, so this is ONE
        // expression naming the key `api}}key`. A plain `find` truncated it at
        // the middle `}}`, so the key was never recorded and the field previewed
        // empty (#450, Codex).
        let ids = template_identifiers(&serde_json::json!({
            "k": r#"{{ secrets.custom["api}}key"] }}"#,
        }));
        assert!(ids.contains("api}}key"), "{ids:?}");
        // The scan resumes after the REAL terminator, so a later expression is
        // still seen.
        let ids = template_identifiers(&serde_json::json!({
            "k": r#"{{ s["a}}b"] }} then {{ secrets.teams.coord }}"#,
        }));
        assert!(ids.contains("a}}b") && ids.contains("coord"), "{ids:?}");
    }

    #[test]
    fn with_redacted_secrets_touches_no_other_namespace() {
        // The record is still a useful preview: everything the operator needs to
        // judge the write — the inputs, the upstream values, the app config, the
        // run identity — is untouched. Only the vault goes dark.
        let blinded = ctx_with_secrets().with_redacted_secrets(&params());
        assert_eq!(blinded.inputs, serde_json::json!({ "phase": "design" }));
        assert_eq!(blinded.upstream["src"]["id"], "p1");
        assert_eq!(blinded.config["region"], "EU");
        assert_eq!(blinded.run["operator"], "pawel");
    }

    #[test]
    fn rendering_against_a_blinded_context_keeps_the_template_around_the_secret() {
        // What the `would-write` record actually shows. The literal text of the
        // template is the author's, not the vault's, so `Bearer ` survives and
        // the operator can still see the header is populated — they just cannot
        // read, and neither can anyone who opens the trace later, what with.
        let ctx = ctx_with_secrets();
        let blinded = ctx.with_redacted_secrets(&params());
        assert_eq!(
            render("Bearer {{ secrets['my-api'].access_token }}", &ctx).unwrap(),
            "Bearer sk-live-abc",
            "the live render is unchanged — this fix must not reach the wire"
        );
        assert_eq!(
            render("Bearer {{ secrets['my-api'].access_token }}", &blinded).unwrap(),
            format!("Bearer {REDACTED}")
        );
        // The documented `{{ secrets.<credential-id> }}` form (app-spec
        // § Templating) is a WHOLE-VALUE ref: `render_config` resolves it
        // structurally rather than rendering it, so it bypasses the assertion
        // above and has to be blinded in its own right.
        assert_eq!(resolve_value("{{ secrets.legacy }}", &ctx), "sk-bare");
        assert_eq!(resolve_value("{{ secrets.legacy }}", &blinded), REDACTED);
        assert_eq!(
            resolve_value("{{ secrets.teams }}", &blinded)["coord"],
            REDACTED,
            "a whole-credential ref must come back blinded field by field"
        );
    }

    #[test]
    fn an_empty_vault_blinds_to_an_empty_vault() {
        // No secrets loaded is the common case, and it must not invent an entry
        // — a `{{ secrets.x }}` ref renders empty either way, as it did before.
        let blinded = RenderContext::default().with_redacted_secrets(&params());
        assert!(blinded.secrets.is_empty());
    }
}
