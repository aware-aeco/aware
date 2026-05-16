#!/usr/bin/env node
const fs = require('fs');
const path = require('path');
const https = require('https');
const { execSync } = require('child_process');

const PKG_VERSION = require('../package.json').version;
const REPO = 'aware-aeco/aware';

const RID_MAP = {
  'win32/x64':    { rid: 'win-x64',    ext: '.exe', archive: 'zip' },
  'linux/x64':    { rid: 'linux-x64',  ext: '',     archive: 'tar.gz' },
  'darwin/arm64': { rid: 'osx-arm64',  ext: '',     archive: 'tar.gz' },
};

// Skip-download escape hatch — for environments without network (corp networks, CI sandboxes)
if (process.env.AWARE_NPM_SKIP_DOWNLOAD === '1') {
  console.log('[aware-npm] AWARE_NPM_SKIP_DOWNLOAD=1 — skipping binary download.');
  process.exit(0);
}

const key = `${process.platform}/${process.arch}`;
const target = RID_MAP[key];
if (!target) {
  console.error(`[aware-npm] unsupported platform: ${key}`);
  console.error(`Supported: ${Object.keys(RID_MAP).join(', ')}`);
  console.error(`Manual install: https://github.com/${REPO}/releases`);
  process.exit(1);
}

const archiveName = `aware-${PKG_VERSION}-${target.rid}.${target.archive}`;
const url = `https://github.com/${REPO}/releases/download/v${PKG_VERSION}/${archiveName}`;
const binariesDir = path.join(__dirname, '..', 'binaries');
const tmpFile = path.join(binariesDir, archiveName);

console.log(`[aware-npm] installing ${PKG_VERSION} for ${target.rid}`);
console.log(`  source: ${url}`);

fs.mkdirSync(binariesDir, { recursive: true });

function download(srcUrl, dest, depth = 0) {
  return new Promise((resolve, reject) => {
    if (depth > 5) return reject(new Error('too many redirects'));
    https.get(srcUrl, { headers: { 'User-Agent': 'aware-npm-installer' } }, (res) => {
      if (res.statusCode >= 300 && res.statusCode < 400 && res.headers.location) {
        return resolve(download(res.headers.location, dest, depth + 1));
      }
      if (res.statusCode !== 200) {
        return reject(new Error(`HTTP ${res.statusCode}: ${srcUrl}`));
      }
      const file = fs.createWriteStream(dest);
      res.pipe(file);
      file.on('finish', () => file.close(() => resolve()));
      file.on('error', reject);
    }).on('error', reject);
  });
}

(async () => {
  try {
    await download(url, tmpFile);

    console.log('[aware-npm] extracting...');
    if (target.archive === 'zip') {
      execSync(`powershell -NoProfile -Command "Expand-Archive -Path '${tmpFile}' -DestinationPath '${binariesDir}' -Force"`, { stdio: 'inherit' });
    } else {
      execSync(`tar -xzf "${tmpFile}" -C "${binariesDir}"`, { stdio: 'inherit' });
    }
    fs.unlinkSync(tmpFile);

    // Promote binaries from <binariesDir>/aware-<version>-<rid>/ to <binariesDir>/
    const extractedDir = path.join(binariesDir, `aware-${PKG_VERSION}-${target.rid}`);
    if (fs.existsSync(extractedDir)) {
      for (const file of fs.readdirSync(extractedDir)) {
        fs.renameSync(path.join(extractedDir, file), path.join(binariesDir, file));
      }
      fs.rmdirSync(extractedDir);
    }

    if (process.platform !== 'win32') {
      fs.chmodSync(path.join(binariesDir, `aware${target.ext}`), 0o755);
      fs.chmodSync(path.join(binariesDir, `aware-sidecar${target.ext}`), 0o755);
    }

    console.log(`[aware-npm] installed to ${binariesDir}`);
  } catch (err) {
    console.error(`[aware-npm] install failed: ${err.message}`);
    console.error(`Set AWARE_NPM_SKIP_DOWNLOAD=1 to skip download and install binaries manually.`);
    console.error(`Manual install: https://github.com/${REPO}/releases`);
    process.exit(1);
  }
})();
