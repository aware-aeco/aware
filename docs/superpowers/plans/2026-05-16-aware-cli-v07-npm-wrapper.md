# AWARE CLI v0.7 (NPM Wrapper Distribution) Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development.

**Goal:** Publish `@aware-aeco/cli` to the public npm registry. After this PR, anyone can run:

```bash
npm install -g @aware-aeco/cli
aware --version
```

…and have a working `aware` + `aware-sidecar` on their machine. Behind the scenes the npm package contains zero Rust/C# code — its `postinstall` script detects the user's platform + arch and downloads the right archive from the GitHub Releases attached to this same repo.

This is the esbuild / swc / turbo pattern — npm registry as a distribution channel for cross-language binaries, not a build target.

**MSI is v0.7.1, NOT in this PR.** Same release artifacts will be consumed by both.

---

## Open design decisions (resolved here)

| Decision | Choice | Why | Alternative |
|---|---|---|---|
| **Package name** | `@aware-aeco/cli` (scoped to the aware-aeco npm org) | Matches the GitHub org; scoped packages avoid name squatting; widely-understood convention | Unscoped `aware-cli` (typo-friendly but anyone could squat it) |
| **NPM org ownership** | User must create `aware-aeco` org on npmjs.com before first publish, OR ship unscoped initially | Operational prerequisite outside this PR | Already-created org (no action needed) |
| **Subdirectory location** | `cli-npm/` at repo root (sibling of `cli/`, `cli-sidecar/`) | Mirrors the existing `cli/` and `cli-sidecar/` siblings; predictable | Under `cli/npm/` or `distribution/npm/` (less discoverable) |
| **Postinstall download source** | GitHub Releases at `aware-aeco/aware` — exact archive names already produced by `release.yml` (`aware-<version>-<rid>.{zip,tar.gz}`) | Single source of truth; the existing release pipeline is the producer | Cloudfront / custom CDN (more infra, no benefit) |
| **Platform detection** | `process.platform` × `process.arch` → RID mapping. `win32/x64 → win-x64`, `linux/x64 → linux-x64`, `darwin/arm64 → osx-arm64`. Anything else: clear error pointing at `curl-pipe` install or building from source | Standard Node convention | Optional fat package (huge; downloads all platforms even if you only need one) |
| **Postinstall failure mode** | Hard error with clear message: which platform we detected, what we tried to download, what to do (set `AWARE_VERSION` env to override, or install via curl-pipe / build from source) | Loud failure beats silent partial install | Best-effort with later runtime error (worse UX) |
| **Bin shim language** | Pure Node JS shim (no TypeScript build step). The shim is `#!/usr/bin/env node` + `child_process.spawn(<binary>, process.argv.slice(2), { stdio: 'inherit' })`. Exit code propagated | One file, zero deps, works on every Node 18+ | Compiled wrapper (overkill for ~30 LOC) |
| **Where binaries live inside the package** | `cli-npm/binaries/` populated at postinstall time; the published tarball does NOT contain the binaries (would balloon the package). `.npmignore` excludes test fixtures, plans, etc. | Smaller published package; binaries fetched at install time | Bundle binaries in package (one-shot install but ~25 MB tarball) |
| **Version sync** | npm package version === CLI binary version (e.g. `0.7.0`). Postinstall downloads `aware-<package.version>-<rid>.{zip,tar.gz}` from the matching GitHub release | Predictable; no version-skew between npm + binary | Pin a separate "binary version" field (more complex) |
| **Caching** | Skip — no per-install cache. Each `npm install -g @aware-aeco/cli` re-downloads. `npm` itself caches the tarball, which is small (~7 MB) | Simple; relies on npm's existing cache layer | Local AWARE_HOME-based cache (extra code for marginal benefit) |
| **CI integration** | Add `publish-npm` job to `.github/workflows/release.yml`. Runs after the build matrix succeeds + after the GitHub Release is created. Uses `NPM_TOKEN` secret. Publishes `cli-npm/` to npm registry | One workflow handles all distribution | Separate workflow (split state; harder to debug) |
| **Code signing / 2FA** | Out of scope for v0.7. Document the recommended `--provenance` flag (npm supports OIDC for unsigned-but-verified) as a v0.7.x follow-up | Skip the auth ceremony for initial publish | Block on signing (delays everything) |

---

## Scope discipline

**v0.7 user-facing surfaces:** none new. Same `aware` CLI; new distribution channel.

**v0.7 internal additions:**
- `cli-npm/package.json` — npm package manifest with `bin`, `postinstall`, etc.
- `cli-npm/scripts/postinstall.js` — platform-detect + download + extract
- `cli-npm/scripts/bin/aware.js` — bin shim that spawns the downloaded binary
- `cli-npm/scripts/uninstall.js` — clean up downloaded binaries on `npm uninstall`
- `cli-npm/README.md` — README for npmjs.com listing
- `cli-npm/.npmignore` — exclude tests, plans, source from published tarball
- Updated `.github/workflows/release.yml` — `publish-npm` job
- Updated `cli/README.md` — npm install instructions

**Out of scope (v0.7.1+):**
- MSI installer (v0.7.1)
- Homebrew formula (v0.7.2)
- winget manifest (v0.7.3)
- Code signing (v0.8)

---

## File Structure

### New files

| Path | Purpose |
|---|---|
| `cli-npm/package.json` | npm package manifest. Fields: `name: @aware-aeco/cli`, `version: 0.7.0`, `bin: { "aware": "scripts/bin/aware.js" }`, `scripts: { postinstall: "node scripts/postinstall.js", uninstall: "node scripts/uninstall.js" }`, `os` array (win32/linux/darwin), `engines: { node: ">=18" }`, `files: ["scripts", "README.md"]`. |
| `cli-npm/scripts/postinstall.js` | Detects platform/arch via `process.platform` + `process.arch`. Maps to RID. Downloads `https://github.com/aware-aeco/aware/releases/download/v<version>/aware-<version>-<rid>.{zip,tar.gz}`. Extracts to `cli-npm/binaries/`. Sets executable bit on Unix. Clear error if anything fails. |
| `cli-npm/scripts/uninstall.js` | Removes `cli-npm/binaries/` directory. |
| `cli-npm/scripts/bin/aware.js` | `#!/usr/bin/env node` + `spawnSync` of the downloaded `aware` binary with argv passthrough and stdio inheritance. Exit code propagation. |
| `cli-npm/README.md` | Npmjs.com listing description. Install instructions + link back to GitHub. |
| `cli-npm/.npmignore` | Excludes `binaries/`, `.github/`, tests, plans. |

### Modified files

| Path | Change |
|---|---|
| `.github/workflows/release.yml` | New `publish-npm` job after `release` job. `actions/setup-node@v4` + `npm publish --provenance --access public`. Uses `NPM_TOKEN` repo secret. |
| `cli/README.md` | Top of Install section: lead with `npm install -g @aware-aeco/cli` as the primary path; curl-pipe + manual remain as secondary. |
| `cli/Cargo.toml` | version `0.6.1` → `0.7.0` |
| `cli-sidecar/cli-sidecar.csproj` | version `0.6.1` → `0.7.0` |
| Two version-asserting Rust tests (`basic.rs`, `doctor.rs`) | update expected version string |

### npm package structure (what gets published)

```
@aware-aeco/cli@0.7.0/
├── package.json
├── README.md
└── scripts/
    ├── postinstall.js
    ├── uninstall.js
    └── bin/
        └── aware.js
```

Total uncompressed: ~10 KB. The binaries (~25 MB total) are fetched at install time.

### Operational note

Before first publish, the user must:
1. Create the `aware-aeco` org on npmjs.com (free for public packages)
2. Generate an NPM_TOKEN with publish scope for that org
3. Add it to the GitHub repo as a secret named `NPM_TOKEN`

This PR makes the CI ready. The actual `npm publish` won't run until `NPM_TOKEN` is set + a tag is pushed.

---

## Tasks

### Task 0: Branch + baseline (done — on `feat/cli-v07-npm-wrapper`)

### Task 1: Scaffold `cli-npm/` with package.json + bin shim

Create the npm package skeleton — `cli-npm/package.json`, `cli-npm/scripts/bin/aware.js`, a stub `cli-npm/scripts/postinstall.js` that prints "TODO" for now (the real download logic is Task 2).

Verify locally: `cd cli-npm && npm install` (should run the postinstall stub without errors).

Commit: `feat(cli-npm): scaffold @aware-aeco/cli npm package`

### Task 2: Real `postinstall.js` — download + extract platform-specific binary

Full implementation. Pseudocode:

```javascript
#!/usr/bin/env node
const fs = require('fs');
const path = require('path');
const https = require('https');
const { execSync } = require('child_process');

const PKG_VERSION = require('../package.json').version;
const REPO = 'aware-aeco/aware';

const RID_MAP = {
  'win32/x64':    { rid: 'win-x64',    ext: 'exe', archive: 'zip' },
  'linux/x64':    { rid: 'linux-x64',  ext: '',    archive: 'tar.gz' },
  'darwin/arm64': { rid: 'osx-arm64',  ext: '',    archive: 'tar.gz' },
};

const key = `${process.platform}/${process.arch}`;
const target = RID_MAP[key];
if (!target) {
  console.error(`[aware] unsupported platform: ${key}`);
  console.error(`Supported: ${Object.keys(RID_MAP).join(', ')}`);
  console.error(`Fall back to manual install: https://github.com/${REPO}/releases`);
  process.exit(1);
}

const archiveName = `aware-${PKG_VERSION}-${target.rid}.${target.archive}`;
const url = `https://github.com/${REPO}/releases/download/v${PKG_VERSION}/${archiveName}`;
const binariesDir = path.join(__dirname, '..', 'binaries');
const tmpFile = path.join(binariesDir, archiveName);

console.log(`[aware] installing ${PKG_VERSION} for ${target.rid}...`);
console.log(`  source: ${url}`);

fs.mkdirSync(binariesDir, { recursive: true });

// Download (follow redirects — GitHub Releases use redirects to S3)
function download(srcUrl, dest, cb, depth = 0) {
  if (depth > 5) return cb(new Error('too many redirects'));
  https.get(srcUrl, (res) => {
    if (res.statusCode >= 300 && res.statusCode < 400 && res.headers.location) {
      return download(res.headers.location, dest, cb, depth + 1);
    }
    if (res.statusCode !== 200) {
      return cb(new Error(`HTTP ${res.statusCode}: ${srcUrl}`));
    }
    const file = fs.createWriteStream(dest);
    res.pipe(file);
    file.on('finish', () => file.close(cb));
  }).on('error', cb);
}

download(url, tmpFile, (err) => {
  if (err) {
    console.error(`[aware] download failed: ${err.message}`);
    console.error(`Set AWARE_NPM_SKIP_DOWNLOAD=1 to skip this step and fall back to manual install.`);
    process.exit(1);
  }

  // Extract
  console.log(`[aware] extracting...`);
  if (target.archive === 'zip') {
    // Windows: use built-in expand-archive via PowerShell
    execSync(`powershell -NoProfile -Command "Expand-Archive -Path '${tmpFile}' -DestinationPath '${binariesDir}' -Force"`, { stdio: 'inherit' });
  } else {
    // Unix: use tar
    execSync(`tar -xzf "${tmpFile}" -C "${binariesDir}"`, { stdio: 'inherit' });
  }
  fs.unlinkSync(tmpFile);

  // Promote binaries from <binariesDir>/aware-<version>-<rid>/ to <binariesDir>/ for easier reference
  const extractedDir = path.join(binariesDir, `aware-${PKG_VERSION}-${target.rid}`);
  if (fs.existsSync(extractedDir)) {
    for (const file of fs.readdirSync(extractedDir)) {
      fs.renameSync(path.join(extractedDir, file), path.join(binariesDir, file));
    }
    fs.rmdirSync(extractedDir);
  }

  // Set executable bit on Unix
  if (process.platform !== 'win32') {
    fs.chmodSync(path.join(binariesDir, `aware${target.ext}`), 0o755);
    fs.chmodSync(path.join(binariesDir, `aware-sidecar${target.ext}`), 0o755);
  }

  console.log(`[aware] ✓ installed to ${binariesDir}`);
});

// Skip-download escape hatch (for environments without network)
if (process.env.AWARE_NPM_SKIP_DOWNLOAD === '1') {
  console.log(`[aware] AWARE_NPM_SKIP_DOWNLOAD=1 — skipping binary download.`);
  process.exit(0);
}
```

Update `bin/aware.js` to point at `binaries/aware{.exe}`:

```javascript
#!/usr/bin/env node
const path = require('path');
const { spawnSync } = require('child_process');
const binaryName = process.platform === 'win32' ? 'aware.exe' : 'aware';
const binary = path.join(__dirname, '..', '..', 'binaries', binaryName);
const result = spawnSync(binary, process.argv.slice(2), { stdio: 'inherit' });
process.exit(result.status ?? 1);
```

Commit: `feat(cli-npm): postinstall downloads + extracts platform binary`

### Task 3: `uninstall.js` cleanup

Remove `binaries/` on `npm uninstall`:

```javascript
const fs = require('fs');
const path = require('path');
const dir = path.join(__dirname, '..', 'binaries');
if (fs.existsSync(dir)) {
  fs.rmSync(dir, { recursive: true, force: true });
  console.log(`[aware] removed ${dir}`);
}
```

Commit: `feat(cli-npm): uninstall removes downloaded binaries`

### Task 4: `.npmignore` + README for npmjs.com

`.npmignore` excludes `binaries/`, `.github/`, tests, plans (anything not needed at install time).

`cli-npm/README.md` is the listing on npmjs.com. Short — "AWARE CLI; npm install -g @aware-aeco/cli; see https://github.com/aware-aeco/aware".

Commit: `docs(cli-npm): .npmignore + README for npmjs.com listing`

### Task 5: Update `release.yml` to publish to npm on tag push

Add a `publish-npm` job after the existing `release` job:

```yaml
publish-npm:
  needs: release
  runs-on: ubuntu-latest
  if: startsWith(github.ref, 'refs/tags/v')
  permissions:
    contents: read
    id-token: write   # for npm --provenance
  steps:
    - uses: actions/checkout@v4
    - uses: actions/setup-node@v4
      with:
        node-version: '20'
        registry-url: 'https://registry.npmjs.org'
    - name: Sync package.json version
      working-directory: cli-npm
      run: |
        version="${GITHUB_REF_NAME#v}"
        # Use jq to update only the version field (preserves all other JSON formatting)
        jq --arg v "$version" '.version = $v' package.json > package.json.tmp
        mv package.json.tmp package.json
        cat package.json
    - name: Publish to npm
      working-directory: cli-npm
      env:
        NODE_AUTH_TOKEN: ${{ secrets.NPM_TOKEN }}
      run: npm publish --access public --provenance
```

Note: `NPM_TOKEN` must be set as a repo secret before this works. The job fails clearly if it's missing.

Commit: `ci: publish-npm job on tag push`

### Task 6: README update — lead with npm install

In `cli/README.md`'s Install section, reorder so `npm install -g @aware-aeco/cli` is the first option (the primary path for developers); curl-pipe + manual + MSI (when available) follow.

Commit: `docs(cli): README leads Install section with npm`

### Task 7: Version bump to 0.7.0 + test updates

```
cli/Cargo.toml:           0.6.1 → 0.7.0
cli-sidecar/cli-sidecar.csproj: 0.6.1 → 0.7.0
cli/tests/basic.rs:       version assertion → 0.7.0
cli/tests/doctor.rs:      version assertion → 0.7.0
cli-npm/package.json:     0.7.0
```

Verify all tests pass.

Commit: `chore: bump to 0.7.0`

### Task 8: Final verification

```powershell
cd cli && cargo test --all-targets && cargo fmt --check && cargo clippy --all-targets -- -D warnings
cd ../cli-sidecar && dotnet test
cd ../cli-npm && node -e "require('./package.json')"
```

All green.

Also verify the package.json is npm-valid:
```bash
cd cli-npm && npm pack --dry-run
```

This shows what would be published without actually publishing.

Commit: (only if anything needed fixing)

---

## Operational follow-up after merge

User must:
1. Create the `aware-aeco` org on npmjs.com
2. Generate an `NPM_TOKEN` with publish scope
3. Add it to the GitHub repo settings as a secret named `NPM_TOKEN`
4. Push a tag `v0.7.0` to trigger the release + npm publish

Once published, anyone can run:
```bash
npm install -g @aware-aeco/cli
aware --version
```

## Self-review

**Spec coverage:** npm wrapper that downloads + spawns the existing Rust + C# binaries. Same release artifacts as curl-pipe.

**Out of scope:** MSI, Homebrew, code signing — all v0.7.1+.

**Risks:**
1. `NPM_TOKEN` operational dependency. The PR makes CI ready; first publish needs the secret.
2. Postinstall network calls. Some CI / corp environments block npm postinstall outbound HTTPS. `AWARE_NPM_SKIP_DOWNLOAD=1` escape hatch documented.
3. NPM org name. If `aware-aeco` is taken on npm, fall back to unscoped `aware-cli` or different scope. User can claim it; this PR assumes it's available.
