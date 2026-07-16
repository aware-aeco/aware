#!/usr/bin/env node
const fs = require('fs');
const path = require('path');
const https = require('https');
const { execFileSync } = require('child_process');

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
    // bsdtar handles both .zip and .tar.gz, exits non-zero on real failures, and
    // (unlike PS 5.1 Expand-Archive) never half-extracts silently (#287). Use the
    // System32 copy on Windows — a GNU tar earlier on PATH (Git Bash) can't read zip.
    const winTar = path.join(process.env.WINDIR || 'C:\\Windows', 'System32', 'tar.exe');
    if (process.platform === 'win32' && !fs.existsSync(winTar)) {
      // Server 2016 / Win10 <1803 ship no inbox bsdtar — fall back to
      // Expand-Archive. Its silent half-extraction mode is defused by the
      // shim-target check below, which turns it into a loud failure (#287).
      // $ErrorActionPreference='Stop' promotes Expand-Archive's non-terminating
      // extraction errors to a non-zero exit, so a partial payload throws here
      // instead of surfacing later.
      execFileSync('powershell', [
        '-NoProfile', '-Command',
        `$ErrorActionPreference='Stop'; Expand-Archive -LiteralPath '${tmpFile}' -DestinationPath '${binariesDir}' -Force`,
      ], { stdio: 'inherit' });
    } else {
      const tar = process.platform === 'win32' ? winTar : 'tar';
      execFileSync(tar, ['-xf', tmpFile, '-C', binariesDir], { stdio: 'inherit' });
    }
    // Cleanup is cosmetic — an AV scan holding the fresh archive must not abort
    // an install that already extracted successfully (#287).
    try { fs.unlinkSync(tmpFile); } catch { /* locked archive; leave it */ }

    // Promote binaries from <binariesDir>/aware-<version>-<rid>/ to <binariesDir>/
    const extractedDir = path.join(binariesDir, `aware-${PKG_VERSION}-${target.rid}`);
    if (fs.existsSync(extractedDir)) {
      for (const file of fs.readdirSync(extractedDir)) {
        const src = path.join(extractedDir, file);
        const dest = path.join(binariesDir, file);
        // renameSync replaces existing FILES atomically (MOVEFILE_REPLACE_EXISTING
        // on Windows); only a directory dest (aware-roslyn) must be moved aside
        // first. Keep it until the replacement lands so an interrupted repair
        // can't destroy a working install (#287 review).
        let backup = null;
        if (fs.existsSync(dest) && fs.statSync(dest).isDirectory()) {
          backup = `${dest}.old-${process.pid}`;
          fs.rmSync(backup, { recursive: true, force: true });
          fs.renameSync(dest, backup);
        }
        try {
          fs.renameSync(src, dest);
        } catch (err) {
          if (backup) fs.renameSync(backup, dest); // restore the old payload
          throw err;
        }
        // The replacement already landed — a locked obsolete backup (AV handle)
        // must not abort the remaining promotions.
        if (backup) {
          try { fs.rmSync(backup, { recursive: true, force: true }); } catch { /* best-effort */ }
        }
      }
      fs.rmdirSync(extractedDir);
    }

    // The shim resolves <binariesDir>/aware(.exe), and the CLI needs its
    // sidecar + roslyn companions next to it. Anything missing is a broken
    // install — fail loudly here instead of letting every later `aware` call
    // exit 1 in silence (#287). Also catches Expand-Archive's silent
    // half-extraction on the no-inbox-tar fallback path.
    const required = [
      `aware${target.ext}`,
      `aware-sidecar${target.ext}`,
      // The roslyn HOST executable, not just its folder — Expand-Archive can
      // create the folder and then fail on its contents.
      path.join('aware-roslyn', `aware-roslyn${target.ext}`),
    ];
    const missing = required.filter((f) => !fs.existsSync(path.join(binariesDir, f)));
    if (missing.length) {
      throw new Error(`extraction finished but ${missing.join(', ')} missing from ${binariesDir}`);
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
