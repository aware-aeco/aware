# Agent Builder Skill · License-aware extraction

**Decompilation is legally gray. The builder respects this: reflection-only by default; decompilation is opt-in AND license-checked. Permissive licenses auto-allow with notice; everything else requires explicit user override.**

## The legal context (not legal advice — read your jurisdiction's rules)

| Activity | Generally allowed | Generally not |
|---|---|---|
| **Reflection** (reading type signatures, method names, parameters via .NET Reflection API) | Yes — this is what every IDE does for IntelliSense. | — |
| **Reading XML doc comments** the vendor published in their NuGet package | Yes — these are documentation, intended to be read. | — |
| **Decompiling** to read method bodies | Permitted under permissive OSS licenses (MIT, Apache, BSD, MPL). | Often prohibited by commercial EULAs. Always prohibited where the license explicitly forbids it. |
| **Redistributing decompiled source** | Sometimes (with attribution under permissive licenses). | Almost never under commercial licenses. |
| **Using decompiled understanding to write descriptions of behavior** | Generally permitted (this is "clean room" / "behavioral description"). | Risky if the descriptions reproduce the original code verbatim. |

The builder is designed to stay on the safe side of each row.

## What the builder does, ordered by risk

| Risk level | Builder action | When it runs |
|---|---|---|
| **Zero risk** | Reflection only — type, method, property signatures | Always |
| **Zero risk** | Read XML docs (vendor-published documentation) | Always when XML files are present |
| **Low risk** | AI-summarize publicly visible signatures into prose | Always (the AI's text is original) |
| **Medium risk** | Decompile to read method bodies | Only if `--decompile` AND license is permissive AND user confirms |
| **High risk** | Redistribute decompiled source | **Never.** Decompiled code is never written to the agent folder. Only AI summaries of behavior are. |

## The permissive license whitelist

Decompile pass auto-runs (with user notice) for these SPDX identifiers:

- MIT
- BSD-2-Clause, BSD-3-Clause
- Apache-2.0
- MPL-2.0
- ISC
- 0BSD, Unlicense

The whitelist is conservative on purpose. Adding to it requires a PR + a brief reasoning note.

## What happens for non-permissive licenses

- **Known proprietary** (Autodesk EULA, Bentley EULA, Trimble Connect SDK terms, etc.) → decompile **disabled** even with `--decompile` flag. Builder emits reflection-only output with a note: *"Refine this skill by hand or by reading the vendor's public docs."*
- **Unknown / unrecognized license** → decompile disabled. Builder asks the user to verify the license and either confirm it's safe (with `--decompile-anyway`) or refine the agent manually.
- **Explicitly forbids reverse engineering** → decompile **hard-blocked** regardless of flags. User must edit the manifest to add `provenance.license-override-acknowledged: true` (which is itself a paper trail).

## License detection

The builder reads, in priority order:

1. NuGet package's `<license>` element (SPDX identifier)
2. NuGet package's `<licenseUrl>` (best-effort match against known URLs)
3. `LICENSE` / `LICENSE.md` / `LICENSE.txt` file inside the package
4. The DLL's `AssemblyCopyrightAttribute` for hints
5. If none found: treat as "unknown" (decompile disabled)

## What "AI summary of behavior" means in practice

When the builder runs the decompile pass under a permissive license, it does this:

1. ILSpy decompiles `Foo.dll` → produces `decompiled/*.cs` files
2. For each public method, the builder chunks the decompiled body into the AI's context
3. The AI is prompted: *"Summarize what this method does in one sentence, focusing on observable behavior and side effects. Do not reproduce code verbatim."*
4. The AI's summary becomes the prose in the skill file
5. **Decompiled source is then deleted** — never written to the agent folder

The agent ships with the AI's behavioral summaries, not the decompiled source. This is the "clean-room" pattern, applied automatically.

## Audit trail

Every decompile pass writes a record to the manifest's `provenance` block:

```yaml
provenance:
  generated-by: aware-agent-builder
  generator-version: 0.1.0
  source:
    type: nuget
    package: Bentley.OpenBridge.Modeler
    license: MIT
    license-source: nuget-metadata
  decompile-pass:
    ran: true
    tool: ilspycmd@8.2.0
    user-confirmed-at: 2026-04-12T14:22:00Z
    user-confirmed-by: pawellisowski
    decompile-output-discarded: true
```

This makes audits and license disputes verifiable. If a vendor objects, the record shows exactly what was extracted, when, under which declared license, and that decompiled source was not retained.

## What contributors should do when in doubt

- **Don't decompile** under proprietary licenses, even if it "seems harmless." Reflection + public docs are usually enough.
- **Cite the source** for every skill that's based on vendor docs.
- **If you wrote a skill from your own experience using the software** (not from decompiling it), say so. That's the cleanest provenance.
- **When the agent breaks at runtime,** the fix is usually a skill update, not a deeper extraction pass.
