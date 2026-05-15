---
name: aware-skill-builder-runtime-scoping
description: This skill should be used when deciding which language conventions (C#, Python, TypeScript, etc.) belong inside an AWARE skill. Encodes the rule "C# only when the target API mandates .NET — otherwise pick whatever's best for AWARE." Lists the AECO software tiers (mandate-.NET vs runtime-agnostic) and shows how to scope C# code examples in runtime-agnostic agents. Apply at Step 4 of the skill-builder pipeline, after FloLess-isms have been decoupled.
---

# Runtime scoping for AWARE skills

**C# is mandated only when the target API requires .NET. For REST / cloud / language-agnostic targets, the skill stays runtime-agnostic and C# examples are scoped as .NET notes.**

This rule prevents a Python-using contributor from being lectured about `.ConfigureAwait(false)` when wiring a Slack agent, while still preserving .NET-specific guidance where it actually matters (Tekla, Revit).

## The two tiers

### Tier A — agents that MUST use C#

Their host API is .NET-only:

| Agent | API | Why C# is mandated |
|---|---|---|
| `tekla` | Tekla Open API | Ships as .NET assemblies; no other binding |
| `revit` | Revit API | .NET assemblies + Revit add-in model |
| `autocad` | AutoCAD .NET API / COM | ObjectARX has C# wrappers; COM is .NET-friendly |
| `civil-3d` | Civil 3D .NET | Same as AutoCAD |
| `bentley-openroads` | Bentley .NET SDK | .NET assemblies |
| `microstation` | MicroStation .NET | .NET only |
| `office-com` (Excel/Word/PowerPoint via Interop) | Office COM Interop | .NET is the only sane host |

For these, **all code examples in the skill are C#**, `.ConfigureAwait(false)` is just convention, and library mandates (e.g., System.Text.Json) are the agent's actual rules.

### Tier B — agents that are runtime-agnostic

Their host API is REST, gRPC, or platform-neutral:

| Agent | API | Why runtime is open |
|---|---|---|
| `trimble-connect` | REST | Any language can call REST |
| `procore` | REST | Same |
| `autodesk-aps` | REST (Forge / Design Automation) | Same |
| `slack` | REST | Same |
| `microsoft-365` | Microsoft Graph REST | Same |
| `google-workspace` | Google APIs REST | Same |
| `file` | OS filesystem | Any language |
| `email` | IMAP/SMTP | Any language |
| `github` | REST + GraphQL | Any language |
| `bcf` | BCF-API REST | Any language |

For these, **code examples may be C#, Python, TypeScript, Go, or pseudocode** — whichever best illustrates the gotcha. Language-specific conventions (`.ConfigureAwait(false)`, `System.Text.Json`, etc.) are scoped to *".NET implementation notes."*

### Tier C — mixed (rare)

A few agents straddle both. For example, `tekla-bridge` exposes Tekla's .NET API as a localhost REST endpoint so non-.NET callers can use it. Skills for `tekla-bridge` are runtime-agnostic (REST), even though the bridge itself is C#.

## How to scope C# in Tier B skills

When porting a FloLess skill (which is always C# code) into a Tier B agent, **keep the C# examples** but mark them clearly:

```markdown
## Upload file (3-step package upload)

> **For .NET callers** (these examples are C#; equivalent patterns in Python, TypeScript, etc. follow the same 3-step shape):

```csharp
var initiateUrl = $"{apiBaseUrl}/files/fs/upload?parentId={parentFolderId}&parentType=FOLDER";
// ...
```

The pattern: any non-.NET runtime needs to:
1. POST the metadata to the initiate endpoint with parentId/parentType as query params (NOT body)
2. PUT bytes to the returned pre-signed URL (different domain, no auth header)
3. GET the complete endpoint with wait=true
```

The structural rule (3 steps, parent-as-query-param, no-auth-on-S3) is language-agnostic. The implementation lines are .NET-specific and scoped.

## How to scope C# in Tier A skills

Keep everything C#. No scoping needed. The skill applies only when generating .NET code, which is the only kind of code the underlying API accepts.

A Tier A skill that suddenly tries to be language-agnostic is wasted effort — there's no other language to be agnostic toward.

## Decision tree

```
Is the target API .NET-only?
├── Yes → Tier A → keep all C# examples as-is, no scoping
└── No → Tier B → keep useful C# examples but mark as .NET notes;
                  the structural rules (gotchas, patterns) stay language-neutral
```

## What "best for AWARE" means in Tier B

When given a choice and a clean slate (no existing FloLess skill to port), pick the language that gives the **clearest demonstration of the structural rule** while remaining readable to the broadest audience.

In practice:

- **Python** for ML / data-shape skills
- **TypeScript** for typed REST / web tooling
- **Curl + JSON** for the most language-neutral REST examples
- **C#** when porting from FloLess and the code is already there

Rule of thumb: the AI doesn't care which language the example is in — it can translate at will. The contributor cares because they read the file. Pick the language a contributor in this domain is most likely to recognize.
