---
name: grasshopper-grasshopper-documentation
description: API reference for namespace Grasshopper.Documentation from Grasshopper.dll
---

# Grasshopper.Documentation

- **GH_Audience**
  - Enumerates the pre-defined target audiences used in the Grasshopper documentation.
- **GH_ContentCollection**
  - Represents an ordered collection of IGH_Content.
- **GH_Format**
  - Represents a piece of formatted content.
- **GH_GlossaryItem**
  - Represents a single glossary entry in the Grasshopper help.
- **GH_Link**
  - Represents a link to another location in the Help or some external address.
- **GH_List**
  - Represents a list of items in the Grasshopper help.
- **GH_Paragraph**
- **GH_Parser**
  - Provides parsing methods for the Grasshopper Markdown flavour.
- **GH_RuntimeFile**
  - Runtime representation of a Grasshopper help file.   A GH_RuntimeFile is nothing more than a collection of keywords with associated content.   This class does not interpret a file or check its validity, it merely   figures out the different content section based on "Keyword:" entries.
- **GH_Style**
  - Enumerates possible formatting for documentation content.
- **GH_Target**
  - Enumerates the possible link targets in Grasshopper documentation.
- **GH_Text**
  - Represents a single string.
- **GH_Topic**
  - Represents a single topic in the Grasshopper help.
- **IGH_Content**
  - All parts that make up documentation content must implement this interface.
