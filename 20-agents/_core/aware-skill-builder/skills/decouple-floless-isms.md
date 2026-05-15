---
name: aware-skill-builder-decouple-floless-isms
description: This skill should be used when porting skills from FloLess source (`src/FloLess/Core/Skills/Resources/`) into AWARE. Lists the seven categories of FloLess-runtime assumptions that must be stripped or rephrased so the skill works for any AWARE agent runtime. Apply at Step 3 of the skill-builder pipeline, after frontmatter conventions are applied and before runtime-scoping. The seven categories are exhaustive — checking against all seven prevents FloLess-runtime leakage into the generic substrate.
---

# Decouple FloLess-runtime assumptions

**FloLess production skills carry seven categories of runtime assumptions that don't apply to the broader AWARE substrate. Strip or rephrase them on every port.**

Why this matters: AWARE agents may be implemented in any language, called from any agentic CLI, and consumed by anyone. A skill that says *"FloLess injects the token via `inputs['trimbleConnectAccessToken']`"* is unintelligible to a contributor writing a Python-based agent — and worse, it primes the AI to generate FloLess-shaped code where AWARE wants language-agnostic guidance.

## The seven categories

| # | FloLess-ism | AWARE equivalent | Severity |
|---|---|---|---|
| 1 | `group: <name>` frontmatter line | Drop entirely | mechanical |
| 2 | *"FloLess injects the token via `inputs[...]`"* | *"AWARE runtime injects the token after `aware connect <agent>`"* | substantive |
| 3 | *"Settings > Integrations > Trimble Connect"* / similar UI references | *"`~/.aware/credentials/<agent>.json`, provisioned via `aware connect`"* | substantive |
| 4 | *"Smart Node"* / *"Think Node"* terminology | *"Agent command"* / *"LLM-backed agent step"* | contextual |
| 5 | `apiBaseUrl` as a Smart Node template variable | *"`apiBaseUrl`, the agent's configured base URL"* — keep the convention, clarify the origin | mechanical |
| 6 | `.ConfigureAwait(false)` mandate | *"For .NET implementations: apply `.ConfigureAwait(false)` to avoid UI-thread capture. Non-.NET runtimes follow their own async idioms."* | scope/clarify |
| 7 | *"System.Text.Json only; Newtonsoft.Json not available"* | Scope to *".NET note"* — AWARE doesn't mandate a JSON library at the substrate level | scope/clarify |

## Category 1 — `group:` frontmatter

The simplest. FloLess uses `group:` to map skills to UI groups in `BuiltInSkills.cs`. AWARE has no UI groups; the parent folder is the implicit group.

```diff
 ---
 name: trimble-connect-files
 description: ...
-group: TrimbleConnect
 ---
```

## Category 2 — Token injection mechanism

FloLess injects tokens via Smart Node inputs. AWARE injects via the agent runtime after credential provisioning.

```diff
-Access tokens are provided via `inputs["trimbleConnectAccessToken"]` — do NOT hardcode tokens.
-**FloLess automatically injects the token** at runtime when Trimble Connect is configured in
-Settings > Integrations > Trimble Connect.
+Authentication is handled by the AWARE runtime. The agent reads
+`~/.aware/credentials/trimble-connect.json` after the user runs
+`aware connect trimble-connect`. The `Authorization: Bearer ...` header is
+injected automatically — do not construct it manually.
```

## Category 3 — UI references

FloLess skills reference its Settings dialog. AWARE skills reference the CLI provisioning command.

```diff
-If the token is missing from inputs, it means the user has NOT configured
-Trimble Connect in the app — return a clear error pointing to
-**Settings > Integrations > Trimble Connect**.
+If the credential file is missing or expired, the agent emits
+`error.auth-expired`. The user provisions or refreshes with
+`aware connect trimble-connect [--refresh]`.
```

## Category 4 — Smart Node / Think Node terminology

FloLess execution model uses "Smart Node" (compiled C# step) and "Think Node" (LLM step). AWARE talks about "agent commands" and (when LLM-backed) "Think Node agent" or "think-node command."

| FloLess term | AWARE term | When to keep vs replace |
|---|---|---|
| "Smart Node code" | "Agent command code" | Replace when describing generic execution |
| "Smart Node" (as a runtime concept) | "Agent step" | Replace when describing topology |
| "Think Node" | "think-node agent command" | Mostly preserved — Think Nodes ARE first-class in AWARE too |
| "Think Node schema" | "think-node agent's input/output schema" | Clarify in context |

## Category 5 — `apiBaseUrl` convention

This is mostly fine in AWARE. FloLess provides it as a template variable inside Smart Nodes. AWARE provides it as part of the agent's runtime config. The variable name and usage are identical; only the origin is different.

```diff
-`apiBaseUrl` = `"https://app.connect.trimble.com/tc/api/2.0"` (FloLess injects this from Settings)
+`apiBaseUrl` = `"https://app.connect.trimble.com/tc/api/2.0"` (the agent's configured base URL — regional, see auth-flow skill)
```

## Category 6 — `.ConfigureAwait(false)` mandate

This is FloLess-WPF-specific (avoids UI-thread capture). For .NET callers it remains best practice; for Python/TypeScript/Go callers it's nonsense. Scope to a .NET note:

```diff
-All `await` calls must use `.ConfigureAwait(false)`
+**For .NET implementations:** apply `.ConfigureAwait(false)` on every `await` (avoids UI-thread capture in WPF / WinForms hosts). For non-.NET runtimes (Python, TypeScript, Go), use the language's idiomatic async patterns; the rule does not apply.
```

## Category 7 — Specific library mandates

`System.Text.Json` is FloLess Smart Node policy (the runtime only ships System.Text.Json). AWARE agents are runtime-agnostic — scope as a .NET note:

```diff
-Use `System.Text.Json` for ALL JSON operations — `Newtonsoft.Json` is NOT available
+**For .NET implementations:** prefer `System.Text.Json` over `Newtonsoft.Json` (less allocation, ships with .NET 6+). For other runtimes, use the standard library JSON tooling (`json` in Python, `JSON.parse` in JS, `encoding/json` in Go).
```

## Verification before Step 4

After applying these, the skill should:

- Not mention "FloLess" anywhere in the body (search `grep -i floless` returns nothing)
- Not say "Smart Node" except when describing genuinely AWARE-equivalent concepts
- Not reference "Settings > Integrations" or similar UI navigation
- Wrap any `.ConfigureAwait(false)` or library-mandate lines in *".NET implementation note"* scope

A grep for `floless\|Smart Node\|Settings >` after Step 3 should return either zero results or only justified survivors (e.g., a historical reference clearly scoped).
