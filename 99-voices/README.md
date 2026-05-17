# Sample voice packs

Three first-party voice packs ship with AWARE as reference implementations of the v0.25 voice-pack distribution primitive. Install any of them locally with:

```bash
aware voice install 99-voices/aware-aeco/structural-engineer/2026 --scope aware-aeco
aware voice install 99-voices/aware-aeco/steel-detailer/2026       --scope aware-aeco
aware voice install 99-voices/aware-aeco/building-physics/2026     --scope aware-aeco
```

Once installed, reference them from a panel block:

```yaml
- id: bump-revisions
  agent: revit-2026
  command: revision.bump
  panel:
    require: unanimous
    voices:
      - id: structural-engineer
        model: claude-opus-4-7
        voice-pack: '@aware-aeco/structural-engineer@2026'
      - id: detailer
        model: gemini-2-5-pro
        voice-pack: '@aware-aeco/steel-detailer@2026'
      - id: building-physics
        model: llama-4-405b
        voice-pack: '@aware-aeco/building-physics@2026'
```

## Authoring your own

A voice pack is a folder with:

```
manifest.yaml          # id, version, scope, citation
system-prompt.md       # the LLM's domain-hat instructions
references/            # optional — cited code excerpts (BS, AISC, etc.)
examples/              # optional — sample dissents + rationales
```

The system prompt should instruct the LLM to **dissent loudly when the
proposed write violates the voice's domain rules**, with specific code
citations. The panel's value is exactly the dissents — if every voice
just approves everything, the gate is theatre.

## Publishing

When the voice-pack registry ships (v0.25.x), publish via:

```bash
aware voice publish ./my-pack --scope @my-firm
```

Until then, voice packs install from local paths only.
