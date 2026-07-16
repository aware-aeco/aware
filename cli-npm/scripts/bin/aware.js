#!/usr/bin/env node
const fs = require('fs');
const path = require('path');
const { spawnSync } = require('child_process');

const binaryName = process.platform === 'win32' ? 'aware.exe' : 'aware';
const binariesDir = path.join(__dirname, '..', '..', 'binaries');
let binary = path.join(binariesDir, binaryName);

if (!fs.existsSync(binary)) {
  // Rescue a partial install: postinstall ≤0.90.0 could leave the extracted
  // versioned folder unpromoted (#287) — run the binary from there.
  const candidate = fs.existsSync(binariesDir)
    ? fs.readdirSync(binariesDir)
        .filter((name) => name.startsWith('aware-'))
        .map((name) => path.join(binariesDir, name, binaryName))
        .find((p) => fs.existsSync(p))
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
