# Trimble Connect Skill · Idempotency

**Always include a stable key (typically the drawing `mark`) as a custom property when uploading. The agent uses this to replace-not-duplicate on re-uploads.**

## The problem without idempotency

A long-running app fires `tekla.watch → tc.upload` many times across a session. Without idempotency:

- The same drawing gets uploaded 10 times (10 file IDs)
- The fab team sees A-104, A-104 (v2), A-104 (v3), … in the folder
- Disk and bandwidth wasted
- Re-runs produce drift, not stability

## The pattern

Pass the stable identity (usually `mark`) as a **custom property** AND as a header:

```yaml
- id: upload
  agent: trimble-connect
  command: upload
  config:
    project-id: "..."
    folder-id: "..."
    filename: "A-104.pdf"
    bytes:    "{{ tekla-watch.drawing-bytes }}"
    properties:
      mark:   "{{ tekla-watch.mark }}"           # custom property; survives re-uploads
      source: "aware-fab-pipeline"
```

The agent translates that into:

```http
POST /projects/{pid}/folders/{fid}/files
Content-Type: multipart/form-data
X-AWARE-Idempotency-Key: A-104                   ← derived from properties.mark
Authorization: Bearer ****

file=@A-104.pdf
properties={"mark":"A-104","source":"aware-fab-pipeline"}
```

Before the upload, the agent does a quick lookup: *"Is there already a file in this folder with `properties.mark == 'A-104'`?"*

- If **yes**: PUT the new bytes to the existing file ID (Trimble preserves the URL + comments + history)
- If **no**: POST as new and record the mark

The caller sees `replaced: true` in the response when an existing file was updated.

## Why `mark`, specifically

In AECO, the drawing mark is the stable, human-readable identifier that survives revisions. Filename can change (`A-104.pdf` → `A-104-rev-B.pdf`), but the mark is constant. Use the mark as the idempotency key; use the filename as the *display* value.

## Idempotency vs. revision history

Don't conflate them. Idempotency means "same mark = same file slot." Revision history is **part of** that single file slot — Trimble keeps every previous version under "Revisions" on the file, and your replacement just adds another revision. You get both: a stable URL and a full history.

If you actually want to fork — *"this is a new drawing that happens to share the same mark"* — use a different idempotency key (e.g. `mark + revision-letter`).

## What goes wrong without this

A team I know ran a Tekla → TC pipeline for 4 months. Their fab folder ended up with 19,000 duplicate drawings. They had to manually script the cleanup. **Set the property on the first upload; never look back.**

## Source

Trimble Connect API allows custom properties on file uploads (`properties` field in the multipart form). The X-AWARE-Idempotency-Key header is AWARE's convention, not Trimble's — it triggers the agent's internal de-dup before the actual REST call.
