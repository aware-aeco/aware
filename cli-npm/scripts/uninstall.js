#!/usr/bin/env node
const fs = require('fs');
const path = require('path');
const dir = path.join(__dirname, '..', 'binaries');
if (fs.existsSync(dir)) {
  fs.rmSync(dir, { recursive: true, force: true });
  console.log(`[aware-npm] removed ${dir}`);
}
