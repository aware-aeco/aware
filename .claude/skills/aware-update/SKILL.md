---
name: aware-update
description: Use when the user asks about updating AWARE, checking for a new version of the `aware` CLI, "what's new", "is there an update", or invokes /aware-update. Checks installed version against npm + GitHub Releases, shows the changelog, asks before applying.
---

# aware-update

Check the user's installed AWARE CLI version against the latest published release, surface what's changed, and (with user consent) update in place using whichever install method they're on.

## When to invoke

- User asks: "is AWARE up to date?", "what's new in aware?", "should I update?", "update aware"
- User invokes `/aware-update`
- During other AWARE workflows, if a behavior the user is hitting was fixed in a newer version, surface this skill as the path to upgrade

Do NOT invoke this skill speculatively when the user is doing unrelated work — only when they ask, or when a version mismatch is directly relevant to their current task.

## What the skill does

1. **Detect installed version.** Run `aware --version`. Capture the version string (format: `aware 0.8.1`).

2. **Detect install method.** Determine how `aware` was installed so the update command matches:
   - `which aware` → look at the resolved path
   - If path is under a `node_modules` / `npm` / `pnpm` / `yarn` / `bun` global dir → **npm wrapper** (`@aware-aeco/cli`)
   - If path is `C:\Program Files\AWARE\aware.exe` → **MSI** (Windows)
   - If path is `/usr/local/bin/aware`, `/opt/aware/...`, `~/.aware/bin/...` → **curl-pipe / PowerShell** installer
   - Otherwise: ambiguous, ask the user

3. **Fetch latest version.** Try in order:
   - `npm view @aware-aeco/cli version` (fastest, no auth)
   - Fallback: `gh release view --repo aware-aeco/aware --json tagName,name,body,publishedAt`

4. **Compare versions** using semver. If installed ≥ latest, tell the user they're up to date and stop. If older, continue.

5. **Fetch changelog** from the GitHub Release body for the new version:
   ```
   gh release view <new-tag> --repo aware-aeco/aware --json name,body,publishedAt
   ```
   Also show what versions are being skipped (e.g., installed 0.8.0 → latest 0.8.2 means show release notes for 0.8.1 + 0.8.2). For each release: `gh release list --repo aware-aeco/aware --limit 20 --json tagName,name,publishedAt`.

6. **Present to the user**:
   - Current version → latest version
   - Compact summary of what changed (top of release body, or first paragraph of each release if multiple)
   - Detected install method (so they know what'll run)
   - Ask: *"Apply update?"*

7. **On confirmation**, run the right update command:
   - **npm**: `npm install -g @aware-aeco/cli@latest`
   - **pnpm**: `pnpm add -g @aware-aeco/cli@latest`
   - **yarn**: `yarn global add @aware-aeco/cli@latest`
   - **bun**: `bun install -g @aware-aeco/cli@latest`
   - **MSI** (Windows): download `aware-<latest>-win-x64.msi` from the GitHub Release; tell the user to run it (MSI install needs elevation, so we don't silent-install on their behalf):
     ```pwsh
     # User runs this themselves (UAC prompt)
     msiexec /i aware-<latest>-win-x64.msi
     ```
   - **curl-pipe (Linux/macOS)**:
     ```bash
     curl -fsSL https://raw.githubusercontent.com/aware-aeco/aware/main/scripts/install.sh | bash -s -- --version <latest>
     ```
   - **PowerShell (Windows)**:
     ```pwsh
     $env:AWARE_VERSION = "<latest>"; iex (irm https://raw.githubusercontent.com/aware-aeco/aware/main/scripts/install.ps1)
     ```

8. **Verify**: after the update command completes, run `aware --version` again. Confirm it matches the latest version. If not, flag the mismatch — the user may need to restart their shell (PATH cache) or check for a permissions issue.

## Edge cases

- **Installed binary not found.** `aware --version` errors or `which aware` returns nothing. Tell the user AWARE isn't installed and point at <https://github.com/aware-aeco/aware> for first install.
- **Pre-release versions** (e.g., `0.9.0-beta.1`). Treat as newer than the corresponding stable; skip when comparing unless user explicitly asked about pre-releases.
- **No network**. `npm view` and `gh` will fail. Tell the user the check can't run offline.
- **Multiple install methods present** (e.g., both npm-global and an MSI install). `which aware` resolves to one — usually the one earlier in PATH. Update only that one, but tell the user there's a duplicate so they can decide if they want to uninstall the other.
- **User has uncommitted aware app/agent state** — irrelevant. AWARE state lives under `~/.aware/` and is preserved across binary updates. No backup ritual needed.

## Don't do

- Don't run the update command without user consent.
- Don't show every release note verbatim if there are many — summarize.
- Don't silently downgrade if the user passes a specific version pin; just do what they asked.
- Don't try to update a system-package-manager install (apt, dnf, brew) by running its package-manager command unless you've confirmed the package exists there — currently AWARE isn't published to apt/dnf/brew, so this is moot, but if it changes, the detection logic needs updating.

## Output format

A clean, single-pass exchange. Example:

```
You're on aware 0.8.0 — latest is 0.8.2.

What changed:
  0.8.2 — patch: ServiceNow agent + Operations vertical placeholder
  0.8.1 — patch: XML doc summaries enrich all NuGet-sourced agents

Install method: npm wrapper (@aware-aeco/cli)
Update command: npm install -g @aware-aeco/cli@latest

Apply update? (y/n)
```

When they say yes:
```
$ npm install -g @aware-aeco/cli@latest
… (output)
$ aware --version
aware 0.8.2

✓ Updated.
```

Brief. Useful. Not noisy.
