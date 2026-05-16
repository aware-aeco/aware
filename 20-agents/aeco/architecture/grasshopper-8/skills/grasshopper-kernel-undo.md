---
name: grasshopper-grasshopper-kernel-undo
description: API reference for namespace Grasshopper.Kernel.Undo from Grasshopper.dll
---

# Grasshopper.Kernel.Undo

- **GH_ArchivedUndoAction**
  - Base class implementation for undo actions that require (de)serialization of data.
- **GH_ObjectUndoAction**
  - Utility base class for undo actions that operate on a single IGH_DocumentObject.
- **GH_UndoAction**
  - Base class implementation for undo actions.
- **GH_UndoException**
  - Exception associated with undo events
- **GH_UndoRecord**
  - Represents a single undo/redo record. A single record may contain any number of undo actions.
- **GH_UndoServer**
  - Provides access to a documents undo data.
- **GH_UndoState**
  - State enumeration for undo records.
- **IGH_UndoAction**
  - Base interface for all undo/redo actions
