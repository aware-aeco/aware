---
name: solibri-rule-sets-and-checks
description: This skill should be used when composing snippets that target Solibri's central feature — running rule sets against loaded IFC models to detect coordination issues (clashes, missing data, code-compliance violations, naming-convention errors). Encodes the rule-set hierarchy, the difference between rules / rule parameters / severity, the run-checking lifecycle, the result categories (issue / accepted / undefined), and the gotcha that rules are .ruleset / .csruleset files that must be loaded into the project before running.
---

# Solibri rule sets and checks

Solibri's value is not viewing IFC — it's running RULES against IFC. Every coordinator's Monday morning is "load the latest federation, run the rule set, export BCF". Get this right and you've automated 80% of the BIM-manager's clash-detection day.

## The hierarchy

| Concept | What |
|---|---|
| Rule Set (`.ruleset` / `.csruleset`) | Container of related rules (e.g. "Architectural Pre-Checking") |
| Rule | One specific check (e.g. "Doors must have a hardware set") |
| Rule Parameter | Configurable threshold per rule (e.g. tolerance, IFC class filter) |
| Severity | `Critical` / `Moderate` / `Low` (drives BCF export priority) |
| Result | One per (rule × affected component): `Issue` / `Accepted` / `Undefined` |

## Loading a rule set

Solibri ships with default rule sets per discipline. Custom ones live as `.csruleset` files exchanged between teams. To load:

```
POST /solibri/v1/rulesets/load
Content-Type: application/json
{ "filePath": "C:\\Shared\\our-firm.csruleset" }
```

After load, the rule set is available by name. Running it doesn't reload — Solibri caches in memory.

## Running checks

```
POST /solibri/v1/checking/run
Content-Type: application/json
{ "rulesetName": "Architectural Pre-Checking" }

→ 202 Accepted with task ID; poll /checking/status/{id} until COMPLETED
```

The check is asynchronous. For large federations (1000+ MB IFC), it can take minutes. Always poll, never assume synchronous return.

## Result categories

After a check completes, each affected component lands in ONE of three buckets:

| Category | Meaning |
|---|---|
| **Issue** | The rule failed for this component. Needs attention. |
| **Accepted** | The rule failed, but a previous reviewer marked it as accepted (intentional deviation). |
| **Undefined** | The rule couldn't evaluate (e.g. missing data — counts neither as pass nor fail). |

For a "find me all real problems" query, filter to Issue + Undefined. For an audit report, include Accepted to show what reviewers explicitly OK'd.

## The check result schema

```json
{
  "rulesetName": "Architectural Pre-Checking",
  "rules": [
    {
      "name": "Doors must have hardware set",
      "severity": "Moderate",
      "issueCount": 47,
      "issues": [
        {
          "guid": "3hjK4...A2",
          "title": "Door D-1.01 missing hardware set",
          "componentGuids": ["3hjK4...A2"]
        }
      ]
    }
  ]
}
```

The structure mirrors the curated `check.run-rule-set` verb's output shape exactly.

## BCF export

After checking, the issues become a BCF topic stream:

```
GET /solibri/v1/bcf/export?severityFilter=Critical,Moderate&version=2.1
→ application/octet-stream (.bcfzip)
```

Versions 2.1 (broader compatibility) and 3.0 (richer markup) are both supported. Default to 2.1 for cross-vendor coordination with Revit / Navisworks / Tekla.

## Common gotchas

- **Rule sets are session-scoped after load.** Reloading a project resets the loaded-ruleset list — re-load before each run.
- **Components without IFC GUID are unrunable.** Solibri identifies everything by IFC GUID; non-IFC-imported components show up as Undefined for every rule.
- **`checking/status/{id}` polling** has a hard 5-minute TTL on the task ID. If polling takes longer, the task ID becomes invalid even though checking continues internally — retrieve by ruleset name instead.
- **The "Accepted" state is per-Solibri-project, not per-IFC.** Re-importing the same IFC into a new project loses Accepted marks. Save the project (.smcproject) to preserve them.

## Standard pattern (via `aware-solibri check.run-rule-set`)

```python
# Pseudocode for the eventual aware-solibri sidecar dispatching this verb
import requests
import time

base = "http://localhost:10876"   # Solibri Anywhere default
rs = args["ruleset-name"]
r = requests.post(f"{base}/checking/run", json={"rulesetName": rs}).json()
task_id = r["taskId"]
while True:
    s = requests.get(f"{base}/checking/status/{task_id}").json()
    if s["state"] == "COMPLETED": break
    time.sleep(2)
results = requests.get(f"{base}/checking/results?rulesetName={rs}").json()
return {"ruleset_name": rs, "rules": results["rules"]}
```

## See also

- [`check.run-rule-set` curated verb](../commands/check.run-rule-set.md) (forthcoming)
- [`bcf.export-checking-results` curated verb](../commands/bcf.export-checking-results.md) (forthcoming)
- `solibri-rest-api.md` — REST contract overview
