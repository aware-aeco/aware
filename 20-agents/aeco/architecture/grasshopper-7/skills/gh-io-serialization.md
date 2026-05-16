---
name: grasshopper-gh-io-serialization
description: API reference for namespace GH_IO.Serialization from GH_IO.dll
---

# GH_IO.Serialization

- **EncodedStringWriter**
  - This class is needed to override the UTF-16 encoding property of the default DotNET StringWriter.
- **GH_Archive**
  - This is the base archive class which takes care of all (de)serialization and messaging.
- **GH_Chunk**
  - Full implementation of GH_IChunk, GH_IReader, GH_IWriter, GH_IBinarySupport and GH_IXmlSupport.              Instances of this class are usually disguised as one of the interfaces it implements.
- **GH_Compression**
  - Provides static methods for compression of byte-arrays.
- **GH_IBinarySupport**
  - Interface which declares all methods required for objects that              can be (de)serialized to and from a binary archive.
- **GH_IChunk**
  - Base interface for all Archive Chunks.
- **GH_IReader**
  - Provides access to a subset of GH_Chunk methods used for reading archives.
- **GH_IWriter**
  - Provides access to a subset of GH_Chunk methods used for writing archives.
- **GH_IXmlSupport**
  - Interface which declares all methods required for objects that              can be (de)serialized to and from an Xml archive.
- **GH_LooseChunk**
  - A utility class for creating partial archives.
- **GH_Message**
  - Represents an archive log message.              Messages are collected during read/write operations.
- **GH_Message_Type**
  - Message type flag.
- **ID**
  - An ID is used to uniquely identify a specific item.
