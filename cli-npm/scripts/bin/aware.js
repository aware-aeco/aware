#!/usr/bin/env node
const path = require('path');
const { spawnSync } = require('child_process');

const binaryName = process.platform === 'win32' ? 'aware.exe' : 'aware';
const binary = path.join(__dirname, '..', '..', 'binaries', binaryName);

const result = spawnSync(binary, process.argv.slice(2), { stdio: 'inherit' });
process.exit(result.status ?? 1);
