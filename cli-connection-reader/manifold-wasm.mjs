// The bytes of manifold-3d's WASM module.
//
// In development (and in the tests) they are read from node_modules. `build.mjs` swaps THIS FILE for a module that
// carries the same bytes inline when it makes the single-file bridge, so the shipped exe stays self-contained and the
// release payload does not grow a second sibling file (web-ifc's .wasm is the only one it has today).
import { readFileSync } from 'node:fs';
import { createRequire } from 'node:module';

export function manifoldWasmBytes() {
  const require = createRequire(import.meta.url);
  return readFileSync(require.resolve('manifold-3d/manifold.wasm'));
}
