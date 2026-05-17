---
description: Check if AWARE CLI has a newer version available, show the changelog, and update with consent.
---

Use the `aware-update` skill to perform the version check + update flow against the user's currently-installed `aware` CLI.

The skill detects the install method (npm wrapper / MSI / curl-pipe / PowerShell), compares the installed version against npm + GitHub Releases, fetches release notes for any versions between current and latest, presents a compact summary, and only runs the update command after the user confirms.
