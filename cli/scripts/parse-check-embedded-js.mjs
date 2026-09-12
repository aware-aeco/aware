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
// it reaches exactly ONE of the six blocks below: it slices `const TEMPLATE`
// out of `viewer_3d.rs` and parse-checks the `type="module"` script inside it,
// so the viewer's classic bootstrap script — the handshake that decides whether
// an embedding client is told the render failed — and `report.rs`'s search
// script were covered by nothing at all, in CI or out of it.
//
// Parsing needs no browser, no Playwright and no network, so it belongs in CI
// whatever happens to the Playwright half. This is that half, widened from one
// block to every block, and discovering them rather than naming them: a seventh
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
// Each block is parsed under the goal symbol a browser would use — `vm.Script`
// for a classic script, `node --check` on a `.mjs` for a module, `JSON.parse`
// for an importmap or any `application/json` block — rather than `new Function`
// for everything, which is sloppy-mode Script semantics and accepts constructs a
// module rejects. See [`parseError`] for why a temp `.js` file is not the
// substitute for `vm.Script` it looks like. A `<script src=…>` with an empty body
// loads its code elsewhere and is skipped.
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

// The inventory the crate ships today, PER FILE. This is what keeps the scan
// from reporting a clean crate after a refactor moves a template somewhere the
// walk no longer reaches — the same role `--self-test` plays for the classifier.
//
// Per file, and exact, because a total was not enough: at "at least 5 blocks in
// at least 3 files" against a real inventory of 6 in 4, losing an OAuth page's
// script — or the viewer module itself — still cleared the floor, and the
// omitted script's syntax then went unchecked with both this gate and the
// browser gate green (Codex review, PR #518). A count that sits below what is
// actually there is not a floor.
//
// Adding a script means adding it here, deliberately and in review. That is the
// point: a new inline script should not be able to arrive unchecked, and a line
// in this table is the cheapest possible way to say "I meant to add one".
const EXPECTED = {
  'src/auth/paste.rs': 1,
  'src/auth/pkce.rs': 1,
  'src/commands/report.rs': 1,
  'src/render/viewer_3d.rs': 3,
};

// ---------------------------------------------------------------------------
// Rust source → string literals
// ---------------------------------------------------------------------------

/**
 * One pass over Rust source, returning both things the rest of this file needs:
 *
 *   `literals` — every string literal, with the 1-based line each starts on.
 *   `code`     — the same source with every comment and every literal (delimiters
 *                included) replaced by spaces, newlines kept so offsets and line
 *                numbers still agree with the original.
 *
 * `code` exists because searching raw source for a Rust *token* is unsound, and
 * that was a real bug rather than a hypothetical one (Codex review, PR #518).
 * [`stripTestItems`] located `#[cfg(test)]` with a plain `indexOf`, so the prose
 * `// behavior under #[cfg(test)] differs` sitting above a production constant
 * made the scanner blank through that constant's semicolon and drop its
 * `<script>` from the inventory entirely — a gate reporting clean over code it
 * could no longer see. Against `code` the same search cannot match inside a
 * comment or a string, because there is nothing there to match.
 *
 * Walking tokens is likewise what stops a `"` inside a comment or a char literal
 * from opening a literal. Handles raw strings (`r"…"`, `r#"…"#`, any hash count),
 * byte strings (`b"…"`, `br#"…"#`), plain strings with escapes, nested block
 * comments, and the `'a` lifetime / `'x'` char-literal ambiguity.
 */
export function walkRust(src) {
  const literals = [];
  // `split('')`, NOT `[...src]`. Spread iterates CODE POINTS, while every index
  // here — `i`, `k`, `indexOf`, `src.length` — is a UTF-16 code UNIT offset. One
  // astral character makes the array shorter than the string it is supposed to
  // mirror, and from there every write lands on the wrong element: a
  // `#[cfg(test)] mod` whose body holds three emoji lost its closing brace in the
  // mask, so `itemEnd` ran to EOF and the next production script was blanked out
  // of the inventory (Codex review, PR #518 — the same silent false negative as
  // the raw `indexOf`, arriving by a different road). `split('')` yields code
  // units, so offsets stay aligned and `join('')` reconstructs the string.
  const code = src.split('');
  let i = 0;
  let line = 1;
  const isIdent = (c) => c !== undefined && /[A-Za-z0-9_]/.test(c);
  /** Consume `src[i..to)`, tracking lines and blanking it out of `code`. */
  const consume = (to) => {
    for (let k = i; k < to; k++) {
      if (src[k] === '\n') line++;
      else code[k] = ' ';
    }
    i = to;
  };
  while (i < src.length) {
    const c = src[i];
    if (c === '\n') { line++; i++; continue; }
    // line comment
    if (c === '/' && src[i + 1] === '/') {
      const nl = src.indexOf('\n', i);
      consume(nl === -1 ? src.length : nl);
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
      consume(k);
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
      if (src[k] === "'") { consume(k + 1); continue; }
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
        literals.push({ text: src.slice(h + 1, end), line: startLine, raw: true });
        consume(close === -1 ? src.length : close + 1 + hashes.length);
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
      literals.push({ text: unescapeRust(buf), line: startLine, raw: false });
      consume(k + 1);
      continue;
    }
    i++;
  }
  return { literals, code: code.join('') };
}

/** Every string literal in `src`, with the 1-based line each starts on. */
export function rustStringLiterals(src) {
  return walkRust(src).literals;
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
 * Both the search and the brace matching run against [`walkRust`]'s `code` mask
 * rather than raw source, which is what makes them token-aware for free: in the
 * mask a comment or a string is spaces, so `#[cfg(test)]` written in prose
 * cannot be mistaken for the attribute and a `{`, `}` or `;` inside a string
 * cannot end an item early. The first of those was a live false negative before
 * this (Codex review, PR #518) — see [`walkRust`].
 */
export function stripTestItems(src) {
  const { code } = walkRust(src);
  const chars = src.split(''); // code units, for the reason [`walkRust`] records
  let at = 0;
  for (;;) {
    const start = code.indexOf('#[cfg(test)]', at);
    if (start === -1) break;
    const end = itemEnd(code, start + '#[cfg(test)]'.length);
    for (let k = start; k < end; k++) if (chars[k] !== '\n') chars[k] = ' ';
    at = end;
  }
  return chars.join('');
}

/**
 * End offset of the item beginning at `from`: its `{…}` body, or its `;`.
 *
 * `code` must be a [`walkRust`] mask, so every brace and semicolon it still
 * contains is real syntax.
 */
function itemEnd(code, from) {
  let i = from;
  let depth = 0;
  let seenBrace = false;
  while (i < code.length) {
    const c = code[i];
    if (c === '{') { depth++; seenBrace = true; i++; continue; }
    if (c === '}') { depth--; i++; if (seenBrace && depth === 0) return i; continue; }
    if (c === ';' && !seenBrace && depth === 0) return i + 1;
    i++;
  }
  return code.length;
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

/**
 * The whole check, in one call: discover the blocks, apply the inventory floor,
 * and parse what survives. Returns `{ blocks, short, failures }`; callers format.
 *
 * This exists because splitting the step across its callers kept going wrong.
 * `tests/browser/run.mjs` imported [`collect`] and [`parseError`] but kept its
 * own `blocks.length < 5` floor after this file moved to [`EXPECTED`], so it
 * could pass over a missing script; and the Rust gate written to prevent that
 * checked only that `shortfall` was *imported*, which deleting the call left
 * true (Codex reviews, PR #518 — twice on the same seam). Two callers reassembling
 * three primitives is three chances each to leave one out. One function is none:
 * there is no order to get wrong and no part to skip, and the floor cannot be
 * reached around because it is inside.
 */
export function checkEmbeddedScripts(root) {
  const blocks = collect(root);
  const short = shortfall(blocks);
  if (short.length > 0) return { blocks, short, failures: [] };
  const scratch = mkdtempSync(join(tmpdir(), 'aware-jsparse-'));
  try {
    const failures = [];
    for (const b of blocks) {
      const err = parseError(b.body, b.goal, scratch);
      if (err !== null) failures.push({ ...b, err });
    }
    return { blocks, short, failures };
  } finally {
    rmSync(scratch, { recursive: true, force: true });
  }
}

/**
 * One message per file whose block count fell below [`EXPECTED`], empty when the
 * inventory is intact. A file with MORE blocks than expected is not a shortfall
 * — the new one is checked like every other, and the count here is a floor.
 */
export function shortfall(blocks) {
  const seen = new Map();
  for (const b of blocks) seen.set(b.file, (seen.get(b.file) || 0) + 1);
  const out = [];
  for (const [file, want] of Object.entries(EXPECTED)) {
    const got = seen.get(file) || 0;
    if (got < want) out.push(`${file}: found ${got} inline script(s), expected at least ${want}`);
  }
  return out;
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
  // The next three are the Codex #518 finding: `#[cfg(test)]` was located with a
  // raw `indexOf`, so the attribute written in prose or in a string swallowed the
  // production item that followed and its script vanished from the inventory.
  {
    name: 'a line comment naming #[cfg(test)] does not hide the constant below it',
    rust: '// behavior under #[cfg(test)] differs\nconst P: &str = r#"<script>var a = 1;</script>"#;',
    expect: [{ goal: 'classic', ok: true }],
  },
  {
    name: 'a block comment naming #[cfg(test)] does not hide the constant below it',
    rust: '/* see #[cfg(test)] below */\nconst P: &str = r#"<script>var a = 1;</script>"#;',
    expect: [{ goal: 'classic', ok: true }],
  },
  {
    name: 'a string containing #[cfg(test)] does not hide the constant below it',
    rust: 'const N: &str = "#[cfg(test)]";\nconst P: &str = r#"<script>var a = 1;</script>"#;',
    expect: [{ goal: 'classic', ok: true }],
  },
  {
    name: 'a real #[cfg(test)] fn is still stripped, not only a mod',
    rust: '#[cfg(test)]\nfn probe() {\n  let p = r#"<script>var a = = 1;</script>"#;\n}\n',
    expect: [],
  },
  {
    name: 'a #[cfg(test)] use statement ends at its semicolon, not at the next item',
    rust: '#[cfg(test)]\nuse std::fmt;\nconst P: &str = r#"<script>var a = 1;</script>"#;',
    expect: [{ goal: 'classic', ok: true }],
  },
  // Astral characters: the mask must be indexed in UTF-16 code units, or a
  // surrogate pair shifts every later offset and the mask stops describing the
  // source it mirrors (Codex review, PR #518).
  {
    name: 'astral characters in a test module do not shift the mask',
    rust: '#[cfg(test)]\nmod tests {\n  const E: &str = "🏗🏗🏗";\n}\nconst P: &str = r#"<script>var a = 1;</script>"#;',
    expect: [{ goal: 'classic', ok: true }],
  },
  {
    name: 'astral characters in a comment do not shift the mask',
    rust: '// 🏗🏗🏗 under construction\nconst P: &str = r#"<script>var a = 1;</script>"#;',
    expect: [{ goal: 'classic', ok: true }],
  },
  {
    name: 'a script containing an astral character is still extracted whole',
    rust: 'const P: &str = r#"<script>var s = "🏗"; var t = 1;</script>"#;',
    expect: [{ goal: 'classic', ok: true }],
  },
];

/** Inventory-floor cases: what [`shortfall`] must and must not report. */
const FLOOR_FIXTURES = [
  {
    name: 'a full inventory is not a shortfall',
    blocks: Object.entries(EXPECTED).flatMap(([file, n]) =>
      Array.from({ length: n }, () => ({ file })),
    ),
    expectFiles: [],
  },
  {
    name: 'losing one OAuth page is a shortfall even though the total stays high',
    blocks: [
      { file: 'src/auth/paste.rs' },
      { file: 'src/commands/report.rs' },
      { file: 'src/render/viewer_3d.rs' },
      { file: 'src/render/viewer_3d.rs' },
      { file: 'src/render/viewer_3d.rs' },
      { file: 'src/render/viewer_3d.rs' },
    ],
    expectFiles: ['src/auth/pkce.rs'],
  },
  {
    name: 'losing the viewer module alone is a shortfall',
    blocks: [
      { file: 'src/auth/paste.rs' },
      { file: 'src/auth/pkce.rs' },
      { file: 'src/commands/report.rs' },
      { file: 'src/render/viewer_3d.rs' },
      { file: 'src/render/viewer_3d.rs' },
    ],
    expectFiles: ['src/render/viewer_3d.rs'],
  },
  {
    name: 'an extra script beyond the inventory is not a shortfall',
    blocks: [
      ...Object.entries(EXPECTED).flatMap(([file, n]) =>
        Array.from({ length: n }, () => ({ file })),
      ),
      { file: 'src/render/ui.rs' },
    ],
    expectFiles: [],
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
  for (const fx of FLOOR_FIXTURES) {
    const got = shortfall(fx.blocks)
      .map((line) => line.split(':')[0])
      .sort();
    const want = [...fx.expectFiles].sort();
    if (got.join('|') !== want.join('|')) {
      bad++;
      console.log(`  FAIL  ${fx.name}`);
      console.log(`        expected shortfall in ${JSON.stringify(want)}`);
      console.log(`        got               ${JSON.stringify(got)}`);
    }
  }
  const total = FIXTURES.length + FLOOR_FIXTURES.length;
  if (bad > 0) {
    console.error(`self-test: ${bad} of ${total} cases failed`);
    return 1;
  }
  console.log(`self-test ok (${total} cases)`);
  return 0;
}

// ---------------------------------------------------------------------------

function main() {
  const scratch = mkdtempSync(join(tmpdir(), 'aware-jsparse-'));
  try {
    if (process.argv.includes('--self-test')) return selfTest(scratch);

    // Through [`checkEmbeddedScripts`] like every other caller — this entry point
    // formats the result, it does not re-decide what the check is.
    const { blocks, short, failures } = checkEmbeddedScripts(process.cwd());
    if (process.argv.includes('--list')) {
      for (const b of blocks) console.log(`${b.file}:${b.line}  ${b.goal}  ${b.body.length} chars`);
    }
    if (short.length > 0) {
      for (const line of short) console.error(`error: ${line}`);
      console.error(
        '\nA script this crate emits is no longer reachable by the scan, so its ' +
          'syntax is checked by nothing. Fix the walk, or — if the template really ' +
          'moved or went away — update EXPECTED in this file to say so deliberately. ' +
          'Do not lower it to whatever the scan happens to find.',
      );
      return 1;
    }
    for (const f of failures) {
      console.error(
        `error: ${f.file}:${f.line}: shipped ${f.goal} script does not parse — ${f.err}`,
      );
    }
    if (failures.length > 0) {
      console.error(
        `\n${failures.length} of ${blocks.length} inline script(s) this crate emits do not ` +
          `parse. They are handed to a browser as-is, and every Rust assertion on them is ` +
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

// Only when run as a program. `tests/browser/run.mjs` imports
// [`checkEmbeddedScripts`] from here — the WHOLE step, not its pieces — so the two
// cannot drift into separate checks or into separate ideas of the inventory floor.
if (process.argv[1] && import.meta.url === pathToFileURL(process.argv[1]).href) {
  process.exit(main());
}
