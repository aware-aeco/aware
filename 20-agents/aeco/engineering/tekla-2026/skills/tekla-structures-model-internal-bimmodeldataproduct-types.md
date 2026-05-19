---
name: tekla-tekla-structures-model-internal-bimmodeldataproduct-types
description: This skill encodes the tekla 2026.0 surface of the Tekla.Structures.Model.Internal.BimModelDataProduct.Types namespace — 1 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: UploadProgress.
---

# Tekla.Structures.Model.Internal.BimModelDataProduct.Types

Auto-generated from vendor docs for tekla 2026.0. 1 types in this namespace.

## UploadProgress (class)

Represents the progress for a file upload operation.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/07880a21-4805-702d-ed86-5cd5a37726ca)

### Constructors
- `public UploadProgress(long? totalBytes, long bytesUploaded)` — Initializes a new instance of the UploadProgress class with known total size.

### Properties
- `PercentComplete` (Nullable<Int32>, get) — Gets the upload progress as a percentage (0-100). If total is unknown, returns null. If null, it indicates we can't determine the completed percentage as we don't know the total size. UI should handle this case appropriately (e.g don't show percentage of completion but bytes upload).
- `TotalBytes` (Nullable<Int64>, get) — Gets the total number of bytes to upload, 0 if total is unknown.
- `UploadedBytes` (Int64, get) — Gets the number of bytes uploaded so far.

