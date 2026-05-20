# view.publish-set

Publish a Publisher Set to its configured output — PDF sheet pack, DWG batch, BIMx package, IFC export — whatever format the set is configured for. The architect's "issue the sheet pack" primitive in Archicad.

## Inputs

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `publisher-set-name` | string | yes | — | Display name of the Publisher Set as shown in Navigator. |
| `output-path` | string | no | `""` | Override the set's configured output dir. Empty = use the set's default. |
| `navigator-item-ids` | string[] | no | `[]` | Optional — publish only these items inside the set instead of the whole set. |

## Implementation (Tapir / Graphisoft JSON-RPC)

Single call. Archicad's non-interactive output (PDF / DWG / BIMx / IFC) goes through the **Publisher**, never through "Print View" alone — the Publisher Set is the unit of issuance because it carries the per-item format, scale, layer combination, view-settings, output-path, and file-naming rules. Tapir exposes the trigger as `NavigatorCommands.PublishPublisherSet`.

### Whole-set publish (most common)

```http
POST http://127.0.0.1:19723/
Content-Type: application/json

{
  "command": "API.ExecuteAddOnCommand",
  "parameters": {
    "addOnCommandId": {
      "commandNamespace": "TapirCommand",
      "commandName": "PublishPublisherSet"
    },
    "addOnCommandParameters": {
      "publisherSetName": "Issue – Stage 4 PDF"
    }
  }
}
```

### Override output path

```http
POST http://127.0.0.1:19723/
Content-Type: application/json

{
  "command": "API.ExecuteAddOnCommand",
  "parameters": {
    "addOnCommandId": {
      "commandNamespace": "TapirCommand",
      "commandName": "PublishPublisherSet"
    },
    "addOnCommandParameters": {
      "publisherSetName": "Issue – Stage 4 PDF",
      "outputPath":       "C:/Issues/2026-05-20-Stage4-PDF"
    }
  }
}
```

### Publish only specific items inside the set

First, walk the Navigator tree to resolve item GUIDs:

```http
POST http://127.0.0.1:19723/
Content-Type: application/json

{
  "command": "API.GetNavigatorItemTree",
  "parameters": {
    "navigatorTreeId": { "type": "PublisherSets" }
  }
}
```

Then call PublishPublisherSet with the selected GUIDs:

```http
POST http://127.0.0.1:19723/
Content-Type: application/json

{
  "command": "API.ExecuteAddOnCommand",
  "parameters": {
    "addOnCommandId": {
      "commandNamespace": "TapirCommand",
      "commandName": "PublishPublisherSet"
    },
    "addOnCommandParameters": {
      "publisherSetName": "Issue – Stage 4 PDF",
      "selectedNavigatorItemIds": [
        { "guid": "AAAAAAAA-BBBB-CCCC-DDDD-EEEEEEEEEEEE" }
      ]
    }
  }
}
```

## Gotchas

- **The Publisher Set must already exist.** This verb does NOT create one. Setting up a Publisher Set is a one-time human-driven Navigator action — the AI orchestrator triggers an existing recipe; it does not author the recipe.
- **The set's output format determines what you get.** A "Save as PDF" set yields PDFs; a "Save as DWG" set yields DWGs. Inspect the set in Navigator → Publisher to confirm before automating.
- **`PublishPublisherSet` returns nothing.** No success object, no file list. Confirmation must come from filesystem-watching the output dir (count expected files) or from a follow-up `GetCurrentWindowType` probe to confirm Archicad is responsive again.
- **Publishing is synchronous in Archicad's main thread.** A 200-sheet PDF publish blocks Archicad for minutes — JSON-RPC clients see the response only when publishing completes. Set HTTP client timeouts to 5+ minutes for large sets.
- **Layout drawings cached against stale model data publish stale.** Pair with `NavigatorCommands.UpdateDrawings` if the model changed since the layouts were last opened.
- **Teamwork projects need a `TeamworkReceive` first** to ensure layouts are at the latest server revision; otherwise sheet revisions on the issued PDFs lag behind the team's checked-in changes.

## See also

- [archicad-scripting-context](../skills/archicad-scripting-context.md) — Tapir + JSON-RPC transport
- [archicad-layouts-and-issue](../skills/archicad-layouts-and-issue.md) — Layout Book, Publisher Sets, sheet revisioning
- [archicad-hotlinks-and-teamwork](../skills/archicad-hotlinks-and-teamwork.md) — TeamworkSend/Receive coordination
