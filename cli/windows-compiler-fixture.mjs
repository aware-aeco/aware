// Small authority fixtures for semantic tests; native execution uses actual compiler inputs instead.
import { mkdirSync, mkdtempSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { dirname, join } from 'node:path';
import { CLOSURE_IDS, INPUT_IDS, NONCOMPILER_TOOL_IDS } from './windows-compiler-closure.mjs';
export const COMPILER_FIXTURE_FILES = {
  'compiler-rust-bin': ['cargo.exe', 'rustc.exe', 'rustdoc.exe', 'rustc_driver-fixture.dll', 'std-fixture.dll'],
  'compiler-rust-lib': ['rustlib/x86_64-pc-windows-msvc/lib/libstd-fixture.rlib', 'rustlib/x86_64-pc-windows-msvc/lib/libcore-fixture.rlib'],
  'compiler-msvc-bin': ['cl.exe', 'link.exe', 'lib.exe', 'c1.dll', 'c2.dll', 'msvcp140.dll', 'vcruntime140.dll'],
  'compiler-msvc-include': ['vcruntime.h'], 'compiler-msvc-lib': ['libcmt.lib', 'libvcruntime.lib'],
  'compiler-sdk-include': ['ucrt/stdio.h', 'shared/winerror.h', 'um/windows.h', 'winrt/fixture.h', 'cppwinrt/fixture.h'],
  'compiler-sdk-um-lib': ['kernel32.lib', 'user32.lib'], 'compiler-sdk-ucrt-lib': ['ucrt.lib', 'libucrt.lib'],
  'compiler-sdk-bin': ['rc.exe', 'rcdll.dll'], 'npm-cache': ['cache'], 'cargo-home': ['vendor/probe/source'],
};
export function compilerFixture() {
  const root = mkdtempSync(join(tmpdir(), 'aware-compiler-fixture-'));
  const file = (name, content = name) => { const path = join(root, name); mkdirSync(dirname(path), { recursive: true }); writeFileSync(path, content); return path; };
  const closures = Object.fromEntries(CLOSURE_IDS.map(id => {
    for (const path of COMPILER_FIXTURE_FILES[id]) file(`${id}/${path}`);
    return [id, join(root, id)];
  }));
  return { root, file, input: { schema: 'aware-windows-repro-builder-inputs/v1',
    source: { commit: 'a'.repeat(40), tree: 'b'.repeat(40), bundle: file('source.bundle') },
    inputs: Object.fromEntries(INPUT_IDS.map(id => [id, file(`inputs/${id}`)])),
    tools: Object.fromEntries(NONCOMPILER_TOOL_IDS.map(id => [id, file(`tools/${id}`)])), closures } };
}
