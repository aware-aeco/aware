<!--
Thanks for contributing to AWARE. Before you fill this out:

  1. Read the decalog — the 5 structural truths AWARE is built on.
     https://github.com/aware-aeco/aware/blob/main/00-vision/decalog.md

  2. If you're adding/modifying a skill, run it through the aware-skill-builder
     pipeline (Anthropic skill-creator + AWARE conventions).
     https://github.com/aware-aeco/aware/blob/main/20-agents/_core/aware-skill-builder/skills/pipeline.md

  3. If you're adding an agent, check the agent-spec.
     https://github.com/aware-aeco/aware/blob/main/10-core/agent-spec.md
-->

## Summary

<!-- 1–3 bullets describing what this PR does. "Why" beats "what." -->

-
-

## Type of change

- [ ] New agent
- [ ] New skill (or skill modification)
- [ ] App (under `30-apps/`)
- [ ] Spec / decalog / manifesto / docs
- [ ] Diagram / visualization
- [ ] Bug fix in substrate
- [ ] Other (specify): ___

## Decalog check

Confirm one of:

- [ ] This change respects all five decalog truths (app=text, AI=runtime, OSS=inherent, no vendor in the loop, AECO=wedge-not-limit).
- [ ] This change deliberately amends a decalog truth. Reasoning in the description below.

## Quality bar (delete what doesn't apply)

### For agents

- [ ] `manifest.yaml` validates (required fields, semver, transport, commands, requires)
- [ ] `provenance` block records how the agent was built (or `hand-written`)
- [ ] Folder layout matches `<vertical>/<agent>/{manifest.yaml,skills/,commands/}`
- [ ] Skill list in manifest matches files in `skills/`
- [ ] Command list in manifest matches files in `commands/`

### For skills

- [ ] Skill was authored / modified via Anthropic's `skill-creator` (mandatory per CLAUDE.md)
- [ ] Frontmatter has `name:` (agent-prefixed) and `description:` (third-person, trigger-rich)
- [ ] No FloLess-runtime-isms (no `group:`, no `Settings > Integrations`, no `inputs[xxxToken]`, no "Smart Node" naming)
- [ ] `.ConfigureAwait(false)` / `System.Text.Json` mandates are scoped as ".NET note" if the agent is Tier B (REST-only)
- [ ] Skill file is listed in the parent agent's `manifest.yaml` `skills:` block

### For apps

- [ ] `app:`, `version:`, `description:`, `nodes:`, `connections:`, `requires:` all present
- [ ] No cycles in `connections:` (DAG only)
- [ ] No secrets in the app file (`{{ secrets.<id> }}` is fine; raw tokens are not)
- [ ] If `exposes-as-agent: true`, the `exposed-commands:` block declares the synthesized agent surface
- [ ] Agent versions are pinned (`<agent>@<minor>.x` recommended)

## Notes for reviewers

<!-- Decisions worth flagging, alternatives you rejected, open questions. -->
