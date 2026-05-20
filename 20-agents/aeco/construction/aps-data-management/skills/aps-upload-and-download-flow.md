---
name: aps-upload-and-download-flow
description: This skill should be used when uploading or downloading file bytes with the aps-data-management agent — the create-storage → OSS signed-S3 upload → create-version sequence (and the gap that the byte-upload step is a separate OSS API not in this catalog), the deprecation of the old direct upload, and the async download-job flow. Encodes how files actually move in and out of ACC/BIM 360 storage.
---

# APS upload & download flow

Creating and fetching file *bytes* is a multi-step dance, and one critical step lives in a different API than this agent reflects.

## Upload: create-storage → S3 upload → create-version

To land a new file (or a new revision):

1. **`create-storage`** — creates an OSS object location; returns a storage URN with a `bucketKey` + `objectKey`.
2. **Upload the bytes via OSS v2 signed-S3 upload** — three calls: `GET /oss/v2/buckets/{bucketKey}/objects/{objectKey}/signeds3upload` (returns signed S3 URL(s); up to 25 parts for big files) → `PUT` the bytes straight to the returned S3 URL(s) → `POST .../signeds3upload` to complete.
3. **`create-version`** (new revision of an existing item) or **`create-item`** (brand-new file) — references the storage URN from step 1.

> **Gap (read this):** step 2 is the **OSS API**, which is **not in this agent's catalog** — this agent only reflects the Data Management spec (steps 1 and 3). So `aps-data-management` **cannot upload bytes end-to-end as shipped.** Either regenerate an OSS agent from the OSS OpenAPI spec, or hand-author a small shim for the 3-call signed-S3 sequence. `create-storage` alone does not upload anything — it just reserves the location.

**The old way is gone.** The legacy direct upload (`PUT object` / `PUT resumable` to `developer.api.autodesk.com`) was **removed on 2022-12-31**; OSS **v1** endpoints sunset **2025-10-31**. Only the signed-S3 (v2) flow works now — don't document or attempt the old PUT-to-storage upload.

## Download: it's asynchronous

Downloading isn't a single GET — Autodesk may transcode to the requested format first:

1. `get-version-downloads` / `get-version-download-formats` — what formats this version can be delivered as.
2. **`create-download`** — request a download in a format → returns a **job**.
3. **`get-download-job`** — poll until the job completes (poll, don't sleep blindly).
4. **`get-download`** — returns the signed S3 URL; `GET` the bytes from it.

For the *current* file, resolve the tip first (`get-item-tip`; see [aps-data-hierarchy](./aps-data-hierarchy.md)).

## `execute-command`

`execute-command` (`POST /data/v1/projects/{project_id}/commands`) runs named batch operations (e.g. publish a Revit C4R model, list refs in bulk). It's the escape hatch for operations that don't have a dedicated endpoint — read the APS "Commands" reference for the command payloads.

## AECO pattern — daily Tekla drawing push

`create-storage` → (OSS signed-S3 upload of the exported PDF, via the shim) → `create-version` into the right ACC folder (resolved with [aps-data-hierarchy](./aps-data-hierarchy.md)). Because the byte step is external, many AECO pipelines instead let the authoring tool's own ACC connector do the upload and use this agent only for the *navigation* + version bookkeeping.

## See also

- [aps-data-hierarchy](./aps-data-hierarchy.md) — resolve the target folder / tip version
- [aps-data-management-auth-and-ids](./aps-data-management-auth-and-ids.md) — `b.` IDs, JSON:API, rate limits
- [acc-docs](../../acc-docs/) — higher-level sheet/transmittal verbs over the same storage
