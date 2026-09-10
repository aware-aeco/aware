#!/usr/bin/env node
// Fail the build when JavaScript the CLI *emits* is not syntactically valid.
//
// CLAUDE.md §Engineering rules — "Verify before answering", "No corner-cutting".
// A green check is the claim that the tests passed; for the ~148 KB of
// JavaScript this crate ships inside its HTML output that claim was never
// evaluated. `aware agent invoke viewer-3d render`, `aware report` and the two
// OAuth callback pages all bake inline `<script>` blocks into a document handed
// to a browser, and every Rust assertion on them is `String::contains` — which a
// syntax error passes. Measured, not assumed: with `const input = =` planted in
// `commands/report.rs`'s `SCRIPT` and `var var post=` planted in
// `render/viewer_3d.rs`'s bootstrap script, all 51 `render::viewer_3d` unit
// tests and all 17 `commands::report` unit tests still reported ok.
//
// The one check that would have caught it is step 0 of `tests/browser/run.mjs`,
// and two things were wrong with it. It never ran — the file is a manual pre-PR
// gate, and its header explains that CI cannot host it because "this repo has no
// Rust CI job at all", which stopped being true when `ci.yml` landed (#298). And
// it reaches exactly ONE of the five blocks below: it slices `const TEMPLATE`
// out of `viewer_3d.rs` and parse-checks the `type="module"` script inside it,
// so the viewer's classic bootstrap script — the handshake that decides whether
// an embedding client is told the render failed — and `report.rs`'s search
// script were covered by nothing at all, in CI or out of it.
//
// Parsing needs no browser, no Playwright and no network, so it belongs in CI
// whatever happens to the Playwright half. This is that half, widened from one
// block to every block, and discovering them rather than naming them: a sixth
// script added later would otherwise arrive unchecked with nothing saying so —
// the lesson `tests/dotnet_suites_gate.rs` records from #433.
//
// ## What counts as a script this crate emits
//
// Only a `<script>` inside a Rust *string literal*, outside `#[cfg(test)]`.
// Both halves are load-bearing and both were found by measurement:
//
//   * String literals only, because a first pass over raw file text matched
//     `<scripts-dir>` in a `//!` doc comment in `render/blender.rs` and the text
//     `</script` inside a `//` comment in `viewer_3d.rs`. Neither is a script and
//     neither is reachable by a user; a gate that reports them teaches people to
//     ignore it. [`rustStringLiterals`] tokenizes the source instead — comments,
//     char literals and lifetimes are skipped, so only text that can actually
//     reach a browser is scanned.
//
//   * Outside `#[cfg(test)]`, because test modules carry deliberately malformed
//     script fixtures: `render/html_report.rs` asserts that
//     `<script>alert("x")</script>` is escaped, and that fragment does not parse
//     (Rust's own `\"` escapes are still in it). Flagging a fixture whose whole
//     purpose is to be invalid is a false positive, and it is the exact shape
//     that gets a gate deleted. [`stripTestItems`] removes `#[cfg(test)]` items
//     by brace matching inside the same tokenizer, so it does not depend on unit
//     tests being last in the file.
//
// Parsing is `node --check` on a temp file — `.mjs` for `type="module"`,
// `.js` otherwise — which is the real parser under the real goal symbol, rather
// than `new Function` (sloppy-mode script semantics, which accept constructs a
// module rejects). `type="importmap"` and any `application/json` block is
// checked as JSON, since that is what a browser does with it. A `<script src=…>`
// with an empty body loads its code elsewhere and is skipped.
//
// Two known limits, stated rather than hidden. A script assembled by `format!`
// from several literals would be checked per-literal and would not parse; nothing
// in this crate does that today, and the execution floor below is what would
// notice if one arrived. And the reported line is exact for a raw string but can
// sit a line or two early for a plain one, because unescaping a `\<newline>`
// continuation collapses source lines that the offset was counted against — the
// file and the parser's own message are what a maintainer navigates by.
//
// ## Usage
//
//     node scripts/parse-check-embedded-js.mjs           # check (run from cli/)
//     node scripts/parse-check-embedded-js.mjs --self-test
//
// `--self-test` is the negative control: it drives the extractor and the checker
// over fixtures with known answers, including a planted syntax error that must be
// reported and the same broken text inside `#[cfg(test)]` that must not be. A
// scanner that has quietly stopped matching anything fails there loudly instead
// of reporting a clean crate.

import { readFileSync, writeFileSync, mkdtempSync, rmSync, readdirSync, statSync } from 'node:fs';
import { execFileSync } from 'node:child_process';
import { tmpdir } from 'node:os';
import { join, relative } from 'node:path';
import { pathToFileURL } from 'node:url';
import vm from 'node:vm';

// The crate ships five inline scripts today. The floor is what keeps this from
// reporting a clean crate after a refactor moves the templates somewhere the
// walk no longer reaches — the same role `--self-test` plays for the classifier
// and `js.length < 5000` plays in `tests/browser/run.mjs`.
const MIN_BLOCKS = 5;
const MIN_FILES = 3;

// ---------------------------------------------------------------------------
// Rust source → string literals
// ---------------------------------------------------------------------------

/**
 * Every string literal in `src`, with the 1-based line each starts on.
 *
 * Walks tokens rather than pattern-matching so that `"` inside a comment or a
 * char literal cannot open a literal. Handles raw strings (`r"…"`, `r#"…"#`, any
 * hash count), byte strings (`b"…"`, `br#"…"#`), plain strings with escapes,
 * nested block comments, and the `'a` lifetime / `'x'` char-literal ambiguity.
 */
export function rustStringLiterals(src) {
  const out = [];
  let i = 0;
  let line = 1;
  const isIdent = (c) => c !== undefined && /[A-Za-z0-9_]/.test(c);
  const advance = (to) => {
    for (let k = i; k < to; k++) if (src[k] === '\n') line++;
    i = to;
  };
  while (i < src.length) {
    const c = src[i];
    if (c === '\n') { line++; i++; continue; }
    // line comment
    if (c === '/' && src[i + 1] === '/') {
      const nl = src.indexOf('\n', i);
      advance(nl === -1 ? src.length : nl);
      continue;
    }
    // block comment (Rust nests them)
    if (c === '/' && src[i + 1] === '*') {
      let depth = 1;
      let k = i + 2;
      while (k < src.length && depth > 0) {
        if (src[k] === '/' && src[k + 1] === '*') { depth++; k += 2; }
        else if (src[k] === '*' && src[k + 1] === '/') { depth--; k += 2; }
        else k++;
      }
      advance(k);
      continue;
    }
    // char literal vs lifetime: `'x'` / `'\n'` are literals, `'static` is not.
    if (c === "'") {
      let k = i + 1;
      if (src[k] === '\\') {
        k++;
        while (k < src.length && src[k] !== "'" && src[k] !== '\n') k++;
      } else {
        k++;
      }
      if (src[k] === "'") { advance(k + 1); continue; }
      i++;                                   // a lifetime — consume just the tick
      continue;
    }
    // raw string, possibly byte-prefixed: r"…", r#"…"#, br##"…"##
    let p = i;
    if (src[p] === 'b' && (src[p + 1] === 'r' || src[p + 1] === '"')) p++;
    if (src[p] === 'r' && !isIdent(src[i - 1])) {
      let h = p + 1;
      while (src[h] === '#') h++;
      if (src[h] === '"') {
        const hashes = '#'.repeat(h - p - 1);
        const startLine = line;
        const close = src.indexOf(`"${hashes}`, h + 1);
        const end = close === -1 ? src.length : close;
        out.push({ text: src.slice(h + 1, end), line: startLine, raw: true });
        advance(close === -1 ? src.length : close + 1 + hashes.length);
        continue;
      }
    }
    // plain string (possibly byte-prefixed)
    if (src[p] === '"' && (p === i || src[i] === 'b') && !isIdent(src[i - 1])) {
      const startLine = line;
      let k = p + 1;
      let buf = '';
      while (k < src.length && src[k] !== '"') {
        if (src[k] === '\\') { buf += src[k] + src[k + 1]; k += 2; }
        else { buf += src[k]; k++; }
      }
      out.push({ text: unescapeRust(buf), line: startLine, raw: false });
      advance(k + 1);
      continue;
    }
    i++;
  }
  return out;
}

/** Rust string escapes, including the `\<newline>` continuation that eats indent. */
export function unescapeRust(s) {
  let out = '';
  for (let i = 0; i < s.length; i++) {
    if (s[i] !== '\\') { out += s[i]; continue; }
    const n = s[++i];
    if (n === '\n') { while (i + 1 < s.length && /\s/.test(s[i + 1])) i++; continue; }
    if (n === 'n') { out += '\n'; continue; }
    if (n === 't') { out += '\t'; continue; }
    if (n === 'r') { out += '\r'; continue; }
    if (n === '0') { out += '\0'; continue; }
    if (n === 'x') { out += String.fromCharCode(parseInt(s.slice(i + 1, i + 3), 16)); i += 2; continue; }
    if (n === 'u' && s[i + 1] === '{') {
      const end = s.indexOf('}', i);
      out += String.fromCodePoint(parseInt(s.slice(i + 2, end), 16));
      i = end;
      continue;
    }
    out += n;                                 // \\ \" \'
  }
  return out;
}

/**
 * `src` with every `#[cfg(test)]` item blanked out, newlines preserved so line
 * numbers still line up.
 *
 * Brace matching runs through the same tokenizer as above, so a `{` inside a
 * string or comment in the test module cannot end the item early.
 */
export function stripTestItems(src) {
  const chars = [...src];
  let at = 0;
  for (;;) {
    const start = src.indexOf('#[cfg(test)]', at);
    if (start === -1) break;
    const end = itemEnd(src, start + '#[cfg(test)]'.length);
    for (let k = start; k < end; k++) if (chars[k] !== '\n') chars[k] = ' ';
    at = end;
  }
  return chars.join('');
}

/** End offset of the item beginning at `from`: its `{…}` body, or its `;`. */
function itemEnd(src, from) {
  let i = from;
  let depth = 0;
  let seenBrace = false;
  while (i < src.length) {
    const c = src[i];
    if (c === '/' && src[i + 1] === '/') { const nl = src.indexOf('\n', i); i = nl === -1 ? src.length : nl; continue; }
    if (c === '/' && src[i + 1] === '*') {
      let d = 1; let k = i + 2;
      while (k < src.length && d > 0) {
        if (src[k] === '/' && src[k + 1] === '*') { d++; k += 2; }
        else if (src[k] === '*' && src[k + 1] === '/') { d--; k += 2; }
        else k++;
      }
      i = k; continue;
    }
    if (c === 'r' || c === 'b' || c === '"' || c === "'") {
      const span = firstLiteralSpan(src, i);
      if (span !== null) { i = span; continue; }
    }
    if (c === '{') { depth++; seenBrace = true; i++; continue; }
    if (c === '}') { depth--; i++; if (seenBrace && depth === 0) return i; continue; }
    if (c === ';' && !seenBrace && depth === 0) return i + 1;
    i++;
  }
  return src.length;
}

/** End offset of a string/char literal starting exactly at `i`, else null. */
function firstLiteralSpan(src, i) {
  const isIdent = (c) => c !== undefined && /[A-Za-z0-9_]/.test(c);
  let p = i;
  if (src[p] === 'b' && (src[p + 1] === 'r' || src[p + 1] === '"')) p++;
  if (src[p] === 'r' && !isIdent(src[i - 1])) {
    let h = p + 1;
    while (src[h] === '#') h++;
    if (src[h] === '"') {
      const hashes = '#'.repeat(h - p - 1);
      const close = src.indexOf(`"${hashes}`, h + 1);
      return close === -1 ? src.length : close + 1 + hashes.length;
    }
  }
  if (src[p] === '"' && !isIdent(src[i - 1])) {
    let k = p + 1;
    while (k < src.length && src[k] !== '"') k += src[k] === '\\' ? 2 : 1;
    return k + 1;
  }
  if (src[i] === "'") {
    let k = i + 1;
    if (src[k] === '\\') { k++; while (k < src.length && src[k] !== "'" && src[k] !== '\n') k++; }
    else k++;
    if (src[k] === "'") return k + 1;
  }
  return null;
}

// ---------------------------------------------------------------------------
// String literal → script blocks
// ---------------------------------------------------------------------------

/** Every `<script …>…</script>` in an HTML fragment, with its type and body. */
export function scriptBlocks(html) {
  const out = [];
  const re = /<script(\s[^>]*)?>/gi;
  let m;
  while ((m = re.exec(html)) !== null) {
    const attrs = m[1] || '';
    const bodyStart = m.index + m[0].length;
    const close = html.toLowerCase().indexOf('</script>', bodyStart);
    if (close === -1) continue;               // not a complete block in this literal
    const typeMatch = attrs.match(/\btype\s*=\s*["']?([^"'\s>]*)/i);
    out.push({
      type: (typeMatch ? typeMatch[1] : '').toLowerCase(),
      hasSrc: /\bsrc\s*=/i.test(attrs),
      body: html.slice(bodyStart, close),
      line: html.slice(0, m.index).split('\n').length - 1,   // 0-based, added to the literal's line
    });
    re.lastIndex = close;
  }
  return out;
}

/** How a browser would treat this block: 'json', 'module', 'classic', or 'skip'. */
export function classify(block) {
  if (block.type === 'importmap' || /json/.test(block.type)) return 'json';
  if (block.hasSrc && block.body.trim() === '') return 'skip';
  if (block.type === 'module') return 'module';
  if (block.type === '' || block.type === 'text/javascript' || block.type === 'application/javascript') {
    return 'classic';
  }
  return 'skip';                              // a data block, not script a browser runs
}

// ---------------------------------------------------------------------------
// Checking
// ---------------------------------------------------------------------------

/** Placeholders the crate substitutes at render time, given parseable stand-ins. */
function substitutePlaceholders(js) {
  return js.replace(/__[A-Z0-9_]+__/g, 'null');
}

/**
 * null when `js` parses under `goal`, else the parser's message.
 *
 * The goal symbol is the whole point, so each is parsed the way a browser parses
 * it. A classic script gets `vm.Script`, which is the Script grammar — it
 * rejects `import`, top-level `await` and top-level `return` exactly as a
 * browser does. A temp `.js` file would NOT: since Node 22, module syntax in a
 * `.js` file is detected and the file is re-parsed as a module, so
 * `<script>import x from "y"</script>` — broken in every browser — passed.
 * A module gets `node --check` on a `.mjs`, which is the Module grammar;
 * `vm.SourceTextModule` would need `--experimental-vm-modules` on the runner.
 */
export function parseError(js, goal, scratch) {
  if (goal === 'json') {
    try { JSON.parse(js); return null; } catch (e) { return e.message; }
  }
  const src = substitutePlaceholders(js);
  if (goal === 'classic') {
    try { new vm.Script(src); return null; } catch (e) { return e.message; }
  }
  const file = join(scratch, `probe-${Math.random().toString(36).slice(2)}.mjs`);
  try {
    writeFileSync(file, src);
    execFileSync(process.execPath, ['--check', file], { stdio: 'pipe' });
    return null;
  } catch (e) {
    const text = `${e.stderr || ''}${e.stdout || ''}` || e.message;
    const hit = text.split('\n').find((l) => /Error/.test(l));
    return (hit || text).trim();
  } finally {
    rmSync(file, { force: true });
  }
}

function rustFiles(dir) {
  const out = [];
  for (const name of readdirSync(dir)) {
    const full = join(dir, name);
    if (statSync(full).isDirectory()) out.push(...rustFiles(full));
    else if (name.endsWith('.rs')) out.push(full);
  }
  return out;
}

/** Every script block this crate emits, discovered from `src/`. */
export function collect(root) {
  const found = [];
  for (const file of rustFiles(join(root, 'src')).sort()) {
    const shipped = stripTestItems(readFileSync(file, 'utf8'));
    for (const lit of rustStringLiterals(shipped)) {
      if (!/<script/i.test(lit.text)) continue;
      for (const block of scriptBlocks(lit.text)) {
        const goal = classify(block);
        if (goal === 'skip') continue;
        found.push({ file: relative(root, file), line: lit.line + block.line, goal, body: block.body });
      }
    }
  }
  return found;
}

// ---------------------------------------------------------------------------
// Self-test
// ---------------------------------------------------------------------------

const FIXTURES = [
  {
    name: 'a valid classic script is found and accepted',
    rust: 'const P: &str = r#"<script>var a = 1;</script>"#;',
    expect: [{ goal: 'classic', ok: true }],
  },
  {
    name: 'a syntax error in a shipped classic script is reported',
    rust: 'const P: &str = r#"<script>var a = = 1;</script>"#;',
    expect: [{ goal: 'classic', ok: false }],
  },
  {
    name: 'the same broken script inside #[cfg(test)] is ignored',
    rust: '#[cfg(test)]\nmod tests {\n  const P: &str = r#"<script>var a = = 1;</script>"#;\n}\n',
    expect: [],
  },
  {
    name: 'a module script is parsed under module semantics',
    rust: 'const P: &str = r#"<script type="module">import x from "y"; export const z = x;</script>"#;',
    expect: [{ goal: 'module', ok: true }],
  },
  {
    name: 'an import statement is a syntax error in a classic script',
    rust: 'const P: &str = r#"<script>import x from "y";</script>"#;',
    expect: [{ goal: 'classic', ok: false }],
  },
  {
    name: 'a doc comment mentioning <scripts-dir> is not a script',
    rust: '//! pass a <scripts-dir> here\nfn f() {}\n',
    expect: [],
  },
  {
    name: 'a line comment mentioning </script> is not a script',
    rust: '// the template\'s own closing </script> tag\nfn f() {}\n',
    expect: [],
  },
  {
    name: 'a plain string literal is unescaped before parsing',
    rust: 'let h = "<script>var s = \\"hi\\";</script>";',
    expect: [{ goal: 'classic', ok: true }],
  },
  {
    name: 'a Rust line continuation inside a plain string is honoured',
    rust: 'let h = "<script>var s = 1;\\\n                 var t = 2;</script>";',
    expect: [{ goal: 'classic', ok: true }],
  },
  {
    name: 'an importmap is checked as JSON, not as script',
    rust: 'const P: &str = r#"<script type="importmap">{ "imports": {} }</script>"#;',
    expect: [{ goal: 'json', ok: true }],
  },
  {
    name: 'a malformed importmap is reported',
    rust: 'const P: &str = r#"<script type="importmap">{ "imports": }</script>"#;',
    expect: [{ goal: 'json', ok: false }],
  },
  {
    name: 'a src-only script has no body to check',
    rust: 'const P: &str = r#"<script src="/x.js"></script>"#;',
    expect: [],
  },
  {
    name: 'a render-time placeholder does not read as a syntax error',
    rust: 'const P: &str = r#"<script>const SCENE = __SCENE_JSON__;</script>"#;',
    expect: [{ goal: 'classic', ok: true }],
  },
  {
    name: 'a char literal holding a quote does not open a string',
    rust: "let q = '\"';\nlet h = r#\"<script>var a = 1;</script>\"#;",
    expect: [{ goal: 'classic', ok: true }],
  },
  {
    name: 'a lifetime is not read as a char literal',
    rust: "fn f<'a>(s: &'a str) -> &'a str { s }\nlet h = r#\"<script>var a = 1;</script>\"#;",
    expect: [{ goal: 'classic', ok: true }],
  },
];

function selfTest(scratch) {
  let bad = 0;
  for (const fx of FIXTURES) {
    const shipped = stripTestItems(fx.rust);
    const got = [];
    for (const lit of rustStringLiterals(shipped)) {
      if (!/<script/i.test(lit.text)) continue;
      for (const block of scriptBlocks(lit.text)) {
        const goal = classify(block);
        if (goal === 'skip') continue;
        got.push({ goal, ok: parseError(block.body, goal, scratch) === null });
      }
    }
    const same =
      got.length === fx.expect.length &&
      got.every((g, i) => g.goal === fx.expect[i].goal && g.ok === fx.expect[i].ok);
    if (!same) {
      bad++;
      console.log(`  FAIL  ${fx.name}`);
      console.log(`        expected ${JSON.stringify(fx.expect)}`);
      console.log(`        got      ${JSON.stringify(got)}`);
    }
  }
  if (bad > 0) {
    console.error(`self-test: ${bad} of ${FIXTURES.length} cases failed`);
    return 1;
  }
  console.log(`self-test ok (${FIXTURES.length} cases)`);
  return 0;
}

// ---------------------------------------------------------------------------

function main() {
  const scratch = mkdtempSync(join(tmpdir(), 'aware-jsparse-'));
  try {
    if (process.argv.includes('--self-test')) return selfTest(scratch);

    const blocks = collect(process.cwd());
    if (process.argv.includes('--list')) {
      for (const b of blocks) console.log(`${b.file}:${b.line}  ${b.goal}  ${b.body.length} chars`);
    }
    if (blocks.length < MIN_BLOCKS || new Set(blocks.map((b) => b.file)).size < MIN_FILES) {
      console.error(
        `error: found only ${blocks.length} inline script(s) in ` +
          `${new Set(blocks.map((b) => b.file)).size} file(s) — expected at least ` +
          `${MIN_BLOCKS} in ${MIN_FILES}. The templates moved somewhere this scan ` +
          `no longer reaches; fix the walk rather than lowering the floor.`,
      );
      return 1;
    }
    let bad = 0;
    for (const b of blocks) {
      const err = parseError(b.body, b.goal, scratch);
      if (err === null) continue;
      bad++;
      console.error(`error: ${b.file}:${b.line}: shipped ${b.goal} script does not parse — ${err}`);
    }
    if (bad > 0) {
      console.error(
        `\n${bad} of ${blocks.length} inline script(s) this crate emits do not parse. ` +
          `They are handed to a browser as-is, and every Rust assertion on them is ` +
          `String::contains, which a syntax error passes.`,
      );
      return 1;
    }
    console.log(`ok: ${blocks.length} inline script(s) this crate emits all parse`);
    return 0;
  } finally {
    rmSync(scratch, { recursive: true, force: true });
  }
}

// Only when run as a program. `tests/browser/run.mjs` imports [`collect`] and
// [`parseError`] from here so the two never drift into separate parse checks.
if (process.argv[1] && import.meta.url === pathToFileURL(process.argv[1]).href) {
  process.exit(main());
}
