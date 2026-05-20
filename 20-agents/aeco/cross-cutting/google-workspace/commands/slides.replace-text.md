# `google-workspace.slides.replace-text`

Bulk-replace `{{ token }}` placeholders in a Slides deck.

## When to use

The "project-prefix template filler" verb — substitute `{{ project-name }}`, `{{ run-date }}` etc. into a standing client-deck template before exporting it with `slides.export-pdf`.

**WRITE-mode**.

## Inputs

| Field | Type | Required | Description |
|---|---|---|---|
| `presentation-id` | string | yes | |
| `replacements` | object | yes | Map of `{{ token }}` → replacement string. |

## Output

```yaml
replaced-count: number
```

## Worked example

```yaml
- id: fill-template
  agent: google-workspace
  command: slides.replace-text
  inputs:
    presentation-id: "{{ secrets.client-deck-id }}"
    replacements:
      "{{ project-name }}": "{{ project.name }}"
      "{{ run-date }}": "{{ run.date }}"
  safety:
    transaction-group: publish
    snapshot: true
```

## Implementation note

Calls `POST presentations.batchUpdate` (Slides API) with one `replaceAllText` request per entry in `replacements`. Write scope: `presentations`. Back off on HTTP 429 / 403 `rateLimitExceeded`.

## See also

- [`slides.export-pdf`](./slides.export-pdf.md)
- [`drive.share`](./drive.share.md)
- [`read-sheet`](./read-sheet.md)
