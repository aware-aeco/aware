---
name: tekla-structures-model-flat-buffers
description: API reference for namespace FlatBuffers from Tekla.Structures.Model.dll
---

# FlatBuffers

- **ByteBuffer**
  - Class to mimic Java's ByteBuffer which is used heavily in Flatbuffers.
- **FlatBufferBuilder**
  - Responsible for building up and accessing a FlatBuffer formatted byte             array (via ByteBuffer).
- **FlatBufferConstants**
- **IFlatbufferObject**
  - This is the base for both structs and tables.
- **Offset`1**
  - Offset class for typesafe assignments.
- **StringOffset**
- **Struct**
  - All structs in the generated code derive from this class, and add their own accessors.
- **Table**
  - All tables in the generated code derive from this struct, and add their own accessors.
- **VectorOffset**
