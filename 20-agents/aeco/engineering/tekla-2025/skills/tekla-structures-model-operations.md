---
name: tekla-structures-model-tekla-structures-model-operations
description: API reference for namespace Tekla.Structures.Model.Operations from Tekla.Structures.Model.dll
---

# Tekla.Structures.Model.Operations

- **GuidConversion**
  - Conversion of old TS GUIDs to current GUIDs.             GUIDs are changed in TS save as operation, this class can be used to convert old GUIDs to current GUIDs.             To recognize the need for GUID conversion, application needs to save project GUID and compare to the current project GUID.             Note: With big models the instance uses a lot of memory.
- **Operation**
  - The Operation class implements Tekla Structures level operations.
