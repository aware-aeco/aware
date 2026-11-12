// build.mjs — package aware-connection-reader as a standalone Windows exe via Node's built-in SEA.
//
// Why SEA + esbuild: the AWARE runtime spawns `~/.aware/bridges/aware-connection-reader.exe` by
// absolute path (see cli/src/commands/sidecar.rs), so the bridge must ship as ONE real .exe with no
// node_modules beside it. SEA embeds a single bundled script into a copy of node.exe; esbuild first
// inlines index.mjs + web-ifc's JS into that single file. web-ifc's .wasm is read from disk at run
// time (index.mjs SetWasmPath → the exe's dir), so it ships as a sibling file, not inlined.
import { build } from 'esbuild';
import { execFileSync } from 'node:child_process';
import { copyFileSync, mkdirSync, readFileSync, rmSync, writeFileSync } from 'node:fs';
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
//
// manifold-3d (the clash narrow phase) ships a ~540 KB .wasm and finds it through `import.meta.url`. Neither survives a
// CJS bundle, so: `manifold-wasm.mjs` is swapped for a module carrying the same bytes inline (the exe stays one file and
// the release payload gains no second sibling), and inside manifold.js ALONE `import.meta.url` becomes the exe's own file
// URL (it only builds a `require` from it; the wasm bytes are handed over explicitly). It is not defined globally: the
// rest of the bridge relies on `import.meta` being inert in the bundle (see the entry guards in index.mjs).
const manifoldForBundle = {
  name: 'manifold-for-bundle',
  setup(b) {
    b.onLoad({ filter: /manifold-wasm\.mjs$/ }, () => {
      const bytes = readFileSync(join(here, 'node_modules', 'manifold-3d', 'manifold.wasm'));
      return { contents: `export function manifoldWasmBytes() { return Buffer.from(${JSON.stringify(bytes.toString('base64'))}, 'base64'); }`, loader: 'js' };
    });
    b.onLoad({ filter: /node_modules.manifold-3d.manifold\.js$/ }, () => {
      const source = readFileSync(join(here, 'node_modules', 'manifold-3d', 'manifold.js'), 'utf8');
      if (!source.includes('import.meta.url')) throw new Error('manifold.js no longer uses import.meta.url; revisit the bundle step in build.mjs');
      return {
        contents: `import { pathToFileURL as __awareToUrl } from 'node:url';\nconst __awareImportMetaUrl = __awareToUrl(process.execPath).href;\n${source.replaceAll('import.meta.url', '__awareImportMetaUrl')}`,
        loader: 'js',
      };
    });
  },
};
await build({
  entryPoints: [join(here, 'model-dispatcher.mjs')],
  bundle: true,
  platform: 'node',
  format: 'cjs',
  target: 'node20',
  outfile: join(dist, 'bundle.cjs'),
  plugins: [manifoldForBundle],
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

// 3. Copy node.exe, strip Node's own Authenticode signature, and inject the blob with postject.
//
// The strip is what makes the exe signable at all. node.exe keeps its signature in a certificate
// table at the end of the file, found through the PE security directory. postject appends its
// section over that spot but leaves the directory pointing there, so the "certificate table" lands
// inside section data: Windows reads the exe as malformed and SignTool fails it with 0x800700C1
// (bad exe format) — which is how release.yml's first real signing run died. Node's SEA docs
// prescribe removing the signature before injecting; this does what `signtool remove /s` does,
// without needing the Windows SDK, and checks the result after injection.
console.log('[build] injecting blob into exe…');
const exe = join(dist, exeName);
copyFileSync(process.execPath, exe);
stripAuthenticodeSignature(exe);
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
const after = readSecurityDirectory(readFileSync(exe));
if (after.size !== 0) {
  throw new Error(
    `${exeName} still has a PE security directory (offset ${after.offset}, size ${after.size}) after injection; ` +
      'release.yml could not Authenticode-sign it',
  );
}

// 4. Ship web-ifc's .wasm alongside the exe (read at run time via SetWasmPath).
copyFileSync(
  join(here, 'node_modules', 'web-ifc', 'web-ifc-node.wasm'),
  join(dist, 'web-ifc-node.wasm'),
);

console.log(`[build] done → ${join(dist, exeName)} (+ web-ifc-node.wasm)`);

// Where the PE security directory (data directory 4: the Authenticode certificate table) sits in
// the header, and what it says. Its "address" is a plain file offset, unlike the other directories.
function readSecurityDirectory(bytes) {
  const pe = bytes.readUInt32LE(0x3c);
  if (bytes.toString('latin1', pe, pe + 4) !== 'PE\0\0') throw new Error('not a PE image');
  const optional = pe + 24;
  const magic = bytes.readUInt16LE(optional);
  const directories = optional + (magic === 0x20b ? 112 : 96); // PE32+ : PE32
  const entry = directories + 4 * 8;
  return { entry, offset: bytes.readUInt32LE(entry), size: bytes.readUInt32LE(entry + 4) };
}

// Remove the certificate table and clear the directory that points at it, leaving an unsigned
// image release.yml can sign. The table must be the file's tail, which is where the PE format puts
// it; anything else is refused rather than guessed at.
function stripAuthenticodeSignature(path) {
  const bytes = readFileSync(path);
  const { entry, offset, size } = readSecurityDirectory(bytes);
  if (size === 0) return;
  if (offset + size !== bytes.length) {
    throw new Error(`${path}: certificate table (offset ${offset}, size ${size}) is not the file's tail`);
  }
  const stripped = Buffer.from(bytes.subarray(0, offset));
  stripped.writeUInt32LE(0, entry);
  stripped.writeUInt32LE(0, entry + 4);
  writeFileSync(path, stripped);
  console.log(`[build] stripped node.exe's signature (${size} bytes) so the exe can be re-signed`);
}
