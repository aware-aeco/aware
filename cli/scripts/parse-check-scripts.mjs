#!/usr/bin/env node
// Parse-checks script bodies that `cli/src/emitted_js_gate.rs` extracted from HTML the crate
// actually rendered. It decides nothing about HTML: the Rust side tokenizes the page with
// `lol_html` and writes one file per executable script, named for the goal a browser would parse
// it under. This file only answers "does it parse?" for each one.
//
//   <n>.js    classic script  -> `vm.Script`, the Script grammar
//   <n>.mjs   module script   -> `node --check`, the Module grammar
//   <n>.json  import map      -> `JSON.parse`
//
// A classic script is NOT checked with `node --check` on a `.js` file: since Node 22 that
// detects module syntax and re-parses the file as a module, so `<script>import x from "y"</script>`
// — broken in every browser — would pass. `vm.SourceTextModule` would need
// `--experimental-vm-modules`, hence `--check` for modules. `vm.Script` compiles without running.
//
// Usage:  node parse-check-scripts.mjs <dir>
// Prints one JSON array to stdout: [{ "file": "<name>", "error": null | "<message>" }], one entry
// per recognised file, and exits 0. Exit 2 means it could not do its job (bad usage, unreadable
// directory, an unrecognised file) — the caller must treat that as a failure, never as "clean".

import { execFileSync } from 'node:child_process';
import { readdirSync, readFileSync } from 'node:fs';
import { join, extname } from 'node:path';
import vm from 'node:vm';

function parseError(path) {
  const ext = extname(path);
  if (ext === '.json') {
    try { JSON.parse(readFileSync(path, 'utf8')); return null; } catch (e) { return e.message; }
  }
  if (ext === '.js') {
    try { new vm.Script(readFileSync(path, 'utf8'), { filename: path }); return null; }
    catch (e) { return e.message; }
  }
  try {
    execFileSync(process.execPath, ['--check', path], { stdio: 'pipe' });
    return null;
  } catch (e) {
    const text = `${e.stderr || ''}${e.stdout || ''}` || e.message;
    const hit = text.split('\n').find((l) => /Error/.test(l));
    return (hit || text).trim();
  }
}

const dir = process.argv[2];
if (!dir || process.argv.length !== 3) {
  console.error('usage: node parse-check-scripts.mjs <dir>');
  process.exit(2);
}
const results = [];
for (const name of readdirSync(dir).sort()) {
  if (!['.js', '.mjs', '.json'].includes(extname(name))) {
    console.error(`unrecognised file ${name}: expected .js, .mjs or .json`);
    process.exit(2);
  }
  results.push({ file: name, error: parseError(join(dir, name)) });
}
process.stdout.write(JSON.stringify(results));
