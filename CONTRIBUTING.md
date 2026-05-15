# Contributing to AWARE

If you can write a paragraph, you can contribute. **There is no build system to learn, no boilerplate generator, no CI pipeline that needs your time.** Most contributions are markdown PRs.

Before you start: read [the decalog](./00-vision/decalog.md). It's five short sections. Every contribution gets evaluated against it.

---

## Three ways to contribute

### 1. Write or extract an agent

Wrap a piece of software as an AWARE agent so AECO professionals can compose against it.

- **Bootstrap:** run `aware agent build --from-<source>` against the software's API surface (DLLs, NuGet package, OpenAPI spec, COM type library, CLI `--help`, C++ headers, Python module). The builder emits a complete agent folder.
- **Refine:** the generated skills are a starting point. Add gotchas, judgment, and version-specific quirks. The Tekla agent's `drawing-identity.md` skill is a good example.
- **Validate:** `aware agent validate ./my-agent/` runs manifest + skill + command checks.
- **Submit:** open a PR adding the folder under `20-agents/<vertical>/<name>/`.

See [`10-core/agent-spec.md`](./10-core/agent-spec.md) for the full contract.

### 2. Publish an app

Compose existing agents into something useful. If others might want to use your composition, mark it `exposes-as-agent: true` and publish.

- **Compose:** write the app file directly, or have Claude compose it for you in your terminal.
- **Validate:** `aware app validate <path>` checks structure, version pins, capabilities, and cycle-freedom.
- **Publish:** for the registry, open a PR under `30-apps/<your-handle>/<app-name>/`. For private use, share the file directly — `.flo`/`.app` files are plain text.

See [`10-core/app-spec.md`](./10-core/app-spec.md) for the format.

### 3. Improve a skill

This is the **lowest-friction, highest-leverage** way to contribute. Skills are plain markdown files inside agent folders. Find a gotcha that bit you, add a paragraph to the relevant skill file, open a PR.

- A skill is *good* when an AI reading it composes better code than an AI without it.
- A skill is *bad* when it duplicates what the code already says, or when it's a feature list instead of judgment.
- The fastest review path is *"here's the gotcha, here's the fix, here's the source I verified it against."*

---

## Repo conventions

- **Numbered top-level folders** (`00-vision/`, `10-core/`, `20-agents/`, …) so the structure stays stable as the project grows. Open the repo in Obsidian and it works as a vault.
- **Agent folders** under `20-agents/<vertical>/<name>/` (e.g., `20-agents/aeco/engineering/tekla/`). Core meta-agents (like `aware-agent-builder`) live under `20-agents/_core/`.
- **App folders** under `30-apps/<author-handle>/<app-name>/` for community apps; `30-apps/_examples/` for reference apps the project itself ships.
- **Plain markdown for all human-readable content.** No HTML files, no JSX, no `.docx`. If it's not openable in Notepad, it doesn't belong in this repo.
- **YAML for manifests.** JSON is also accepted (the substrate parses both), but YAML is canonical for human-edited files.
- **No secrets in PRs.** The repo has zero `*.credentials.*` / `*.env` / `*.pem` files. Credentials live in `~/.aware/credentials/` on the user's machine, never in git.

---

## Quality bar

Before submitting:

- [ ] My change respects the [decalog](./00-vision/decalog.md). If it doesn't, I have a written justification for which truth I'm proposing to amend.
- [ ] Manifests validate (`aware agent validate` or `aware app validate`).
- [ ] Skills lead with the rule, then explain. They encode judgment, not just facts.
- [ ] No vendor secrets, license-encumbered code, or proprietary IP committed.
- [ ] If I'm adding a new agent, I cited the source of its API surface in the manifest's `provenance` block.
- [ ] If I changed an existing agent's commands or schemas, I bumped the version (semver) and added a one-line note to `CHANGELOG.md` in that agent's folder.

---

## Filing issues

- **Reporting a gotcha** that should be in a skill: open a PR adding the paragraph directly. Faster than a discussion thread.
- **Suggesting a new agent**: open an issue tagged `agent-request` describing the software, its API surface (DLL/REST/CLI?), and one concrete app you'd build with it.
- **Reporting a bug**: include the agent name + version, the exact CLI command, and the error envelope returned by the runtime.

---

## Code of conduct

Be kind. Disagree about ideas, not people. Assume good faith. Don't bring vendor politics into the repo — AWARE is a substrate, not a competitor to specific products.

If a contribution debate stalls, the decalog is the tiebreaker.

---

## Who's behind this

AWARE is maintained by BIMstudio and a growing list of AECO contributors. The substrate is Apache 2.0 forever. The commercial apps built on top (including [FloLess](https://floless.io)) are separate projects under their own licenses.
