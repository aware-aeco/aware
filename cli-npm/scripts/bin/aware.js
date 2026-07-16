#!/usr/bin/env node
const fs = require('fs');
const path = require('path');
const { spawnSync } = require('child_process');

const PKG_VERSION = require('../../package.json').version;
const { requiredFiles } = require('../payload');
const binaryName = process.platform === 'win32' ? 'aware.exe' : 'aware';
const binariesDir = path.join(__dirname, '..', '..', 'binaries');
let binary = path.join(binariesDir, binaryName);

if (!fs.existsSync(binary)) {
  // Rescue a partial install: postinstall ≤0.90.0 could leave the extracted
  // versioned folder unpromoted (#287) — run the binary from there. Pinned to
  // this package's version so a leftover from another release never runs, and
  // only a COMPLETE payload qualifies — the CLI resolves its sidecar/roslyn
  // companions next to the executable, so a half-extracted folder would run
  // but fail confusingly later.
  const exeSuffix = process.platform === 'win32' ? '.exe' : '';
  const complete = (dir) =>
    requiredFiles(exeSuffix).every((f) => fs.existsSync(path.join(dir, f)));
  const candidate = fs.existsSync(binariesDir)
    ? fs.readdirSync(binariesDir)
        .filter((name) => name.startsWith(`aware-${PKG_VERSION}-`))
        .map((name) => path.join(binariesDir, name))
        .filter(complete)
        .map((dir) => path.join(dir, binaryName))[0]
    : undefined;
  if (candidate) {
    binary = candidate;
  } else {
    // Never fail silently (#287) — say what is missing and how to repair it.
    console.error(`aware: binary not found at ${binary}`);
    console.error('The npm postinstall step did not complete on this machine. To repair, run:');
    console.error(`  node "${path.join(__dirname, '..', 'postinstall.js')}"`);
    console.error('or download a release manually: https://github.com/aware-aeco/aware/releases');
    process.exit(1);
  }
}

const result = spawnSync(binary, process.argv.slice(2), { stdio: 'inherit' });
if (result.error) {
  console.error(`aware: failed to run ${binary}: ${result.error.message}`);
}
process.exit(result.status ?? 1);
