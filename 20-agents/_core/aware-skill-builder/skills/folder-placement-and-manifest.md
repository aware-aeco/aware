---
name: aware-skill-builder-folder-placement-and-manifest
description: This skill should be used at Step 5 of the skill-builder pipeline to place a skill file in the correct AWARE folder and update its parent agent's manifest. Encodes the folder convention (`20-agents/<vertical>/<agent>/skills/<short-name>.md`), the manifest `skills:` list update pattern, and the filename-vs-frontmatter-name distinction. Apply after `skill-creator` has produced the skill and AWARE-specific layering (frontmatter, decoupling, runtime-scoping) has been applied.
---

# Folder placement + manifest update

**A skill that lives outside its parent agent's `skills/` folder or that isn't listed in the agent's `manifest.yaml` is invisible to the AWARE runtime. Both placement and manifest update are mandatory at Step 5.**

## Folder convention

```
20-agents/
├── _core/                                  ← meta-agents (aware-agent-builder, aware-skill-builder)
│   └── <agent>/skills/<short-name>.md
└── <vertical>/                             ← e.g., aeco, finance, legal, …
    └── <sub-vertical>/                     ← e.g., engineering, construction, operations
        └── <agent>/                        ← e.g., tekla, trimble-connect, revit
            ├── manifest.yaml
            ├── skills/                     ← skills land HERE
            │   ├── <short-name-1>.md
            │   ├── <short-name-2>.md
            │   └── ...
            └── commands/
                └── ...
```

### The AECO verticals

```
20-agents/aeco/
├── architecture/    ← revit · archicad · rhino · sketchup · allplan
├── engineering/     ← tekla · etabs · staad · idea-statica · robot · civil-3d
├── construction/    ← trimble-connect · procore · acc · navisworks · slack
├── operations/      ← maximo · tandem · planon · servicenow
└── cross-cutting/   ← microsoft-365 · google-workspace · file · email · think-node
```

### Choosing a vertical

When porting or creating, pick by **what the agent's users do**, not by what its vendor is. Examples:

- Tekla → engineering (steel detailers / structural engineers use it)
- Trimble Connect → construction (CDE for project teams)
- Microsoft 365 / Graph → cross-cutting (used across all verticals)
- Excel → cross-cutting (everyone uses it; not AECO-specific)
- AutoCAD → architecture (primarily) but also engineering for civil

If the agent genuinely spans multiple verticals, put it under `cross-cutting/`.

## Filename rules

| Rule | Example |
|---|---|
| Lowercase, kebab-case | `auth-flow.md`, NOT `AuthFlow.md` |
| Short topic, no agent prefix | `auth-flow.md`, NOT `trimble-connect-auth-flow.md` |
| Hyphen-separated multi-word | `event-threading.md`, NOT `event_threading.md` |
| `.md` extension | always |
| No leading numbers | `01-intro.md` discouraged — folder order is for top-level dirs only |

The agent prefix lives in the **frontmatter name** (`name: trimble-connect-auth-flow`), not the filename. The filename is the contributor-facing label; the frontmatter name is the AI-facing identifier.

## Updating the agent's manifest

After placing the skill, append its filename (not the full name) to the agent's `manifest.yaml` `skills:` list:

```yaml
# 20-agents/aeco/construction/trimble-connect/manifest.yaml
agent: trimble-connect
version: 2.4.0
# ...
skills:
  - auth-flow.md
  - idempotency.md
  - rate-limits.md
  - files.md            # ← newly added
```

The `skills:` list is the agent's contract about which skills it ships. The AWARE runtime walks this list when packaging the agent for distribution and when generating host plugins (`~/.claude/plugins/aware-aeco/`).

### Ordering

Order skills in the manifest by **likely-relevance-first**:

1. Foundational / always-relevant skills (e.g., `auth-flow` for any REST agent)
2. Common operation skills (e.g., `upload`, `download`)
3. Edge-case skills (e.g., `rate-limits`, `idempotency`)
4. Reference-only skills (e.g., `error-codes`, `version-history`)

This order doesn't affect runtime behavior but helps contributors reading the manifest understand the agent at a glance.

## Verification before commit

```bash
# 1. File exists at the expected path
test -f "20-agents/aeco/construction/trimble-connect/skills/files.md"

# 2. Manifest references it
grep -F '- files.md' "20-agents/aeco/construction/trimble-connect/manifest.yaml"

# 3. The frontmatter name matches: <agent>-<filename-without-extension>
head -5 "20-agents/aeco/construction/trimble-connect/skills/files.md" \
  | grep -E '^name: trimble-connect-files$'

# 4. The agent loads cleanly (after CLI exists):
# aware agent validate trimble-connect
```

If any of the four checks fail, the placement step isn't complete. Fix before proceeding to Step 6 (eval).

## Common mistakes

| Mistake | Fix |
|---|---|
| File in the wrong vertical | Move it; update any cross-references |
| Filename has the agent prefix duplicated | Strip the prefix from the filename; keep it in `name:` only |
| Manifest not updated | Append; commits without this are invalid |
| Multiple agents share a skill via symlink | Don't symlink — duplicate the file. Symlinks break on Windows clones and confuse contributors. |
| Skill placed under `_core/` when it's vertical-specific | Move to the correct vertical |
