---
name: grasshopper-grasshopper-kernel-undo-actions
description: API reference for namespace Grasshopper.Kernel.Undo.Actions from Grasshopper.dll
---

# Grasshopper.Kernel.Undo.Actions

- **GH_AddObjectAction**
  - Records a single object addition.
- **GH_AddStateAction**
  - Record the addition of a single solution state to a document.
- **GH_DataMatchingAction**
  - Records a single Component DataComparison event.
- **GH_DataModificationAction**
  - Records a single Parameter DataModification event.
- **GH_GenericObjectAction**
  - Records a single object change. Object changes are undone/redone by (de)serializing the objects.   Object changes also keep track of all wires feeding into and coming out of the object.
- **GH_HiddenAction**
  - Records a single object Preview event.
- **GH_IconDisplayAction**
  - Records a single icon display mode event.
- **GH_IconOverrideAction**
  - Records a single icon override event.
- **GH_LayoutAction**
  - Records a single object layout event. Useful for tracking object resizes.
- **GH_LockedAction**
  - Records a single object Enabled event.
- **GH_NickNameAction**
  - Records a single object NickName and IconDisplayMode event.
- **GH_PersistentDataAction`1**
  - Records the changes in the persistent data of a parameter.
- **GH_PivotAction**
  - Records a single object pivot event. Useful for tracking object movement.
- **GH_RemoveObjectAction**
  - Records a single object removal.
- **GH_RemoveStateAction**
  - Record the removal of a single solution state from a document.
- **GH_WireAction**
  - Records any change to the input wires of a given parameter.
- **GH_WireDisplayAction**
  - Records a single wire display mode event.
