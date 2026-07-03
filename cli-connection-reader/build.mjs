// build.mjs — package aware-connection-reader as a standalone Windows exe via Node's built-in SEA.
//
// Why SEA + esbuild: the AWARE runtime spawns `~/.aware/bridges/aware-connection-reader.exe` by
// absolute path (see cli/src/commands/sidecar.rs), so the bridge must ship as ONE real .exe with no
// node_modules beside it. SEA embeds a single bundled script into a copy of node.exe; esbuild first
// inlines index.mjs + web-ifc's JS into that single file. web-ifc's .wasm is read from disk at run
// time (index.mjs SetWasmPath → the exe's dir), so it ships as a sibling file, not inlined.
import { build } from 'esbuild';
import { execFileSync } from 'node:child_process';
import { copyFileSync, mkdirSync, rmSync, writeFileSync } from 'node:fs';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';

const here = dirname(fileURLToPath(import.meta.url));
const dist = join(here, 'dist');
const exeName = 'aware-connection-reader.exe';
const FUSE = 'NODE_SEA_FUSE_fce680ab2cc467b6e072b8b5df1996b2'; // Node's standard SEA sentinel fuse

rmSync(dist, { recursive: true, force: true });
mkdirSync(dist, { recursive: true });

// 1. Bundle ESM entry + web-ifc into one CJS file (SEA embeds a single file; node_modules aren't shipped).
console.log('[build] bundling with esbuild…');
await build({
  entryPoints: [join(here, 'index.mjs')],
  bundle: true,
  platform: 'node',
  format: 'cjs',
  target: 'node20',
  outfile: join(dist, 'bundle.cjs'),
});

// 2. Generate the SEA blob from the bundle.
console.log('[build] generating SEA blob…');
const seaConfig = join(dist, 'sea-config.json');
writeFileSync(
  seaConfig,
  JSON.stringify({
    main: join(dist, 'bundle.cjs'),
    output: join(dist, 'sea-prep.blob'),
    disableExperimentalSEAWarning: true,
  }),
);
execFileSync(process.execPath, ['--experimental-sea-config', seaConfig], { stdio: 'inherit' });

// 3. Copy node.exe and inject the blob with postject.
console.log('[build] injecting blob into exe…');
const exe = join(dist, exeName);
copyFileSync(process.execPath, exe);
execFileSync(
  process.execPath,
  [
    join(here, 'node_modules', 'postject', 'dist', 'cli.js'),
    exe,
    'NODE_SEA_BLOB',
    join(dist, 'sea-prep.blob'),
    '--sentinel-fuse',
    FUSE,
  ],
  { stdio: 'inherit' },
);

// 4. Ship web-ifc's .wasm alongside the exe (read at run time via SetWasmPath).
copyFileSync(
  join(here, 'node_modules', 'web-ifc', 'web-ifc-node.wasm'),
  join(dist, 'web-ifc-node.wasm'),
);

console.log(`[build] done → ${join(dist, exeName)} (+ web-ifc-node.wasm)`);
