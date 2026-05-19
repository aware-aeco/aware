---
name: grasshopper-grasshopper-documentation
description: This skill encodes the grasshopper 8.0 surface of the Grasshopper.Documentation namespace — 14 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: GH_ContentCollection, GH_GlossaryItem, GH_Format, GH_Link, GH_List, GH_Paragraph, GH_Parser, GH_RuntimeFile, and 6 more types.
---

# Grasshopper.Documentation

Auto-generated from vendor docs for grasshopper 8.0. 14 types in this namespace.

## GH_Audience (enum)

Enumerates the pre-defined target audiences used in the Grasshopper documentation.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_Documentation_GH_Audience.htm)

### Values
- `Beginner` = `0`
- `Intermediate` = `1`
- `Expert` = `2`

## GH_ContentCollection (class)

Represents an ordered collection of IGH_Content.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_Documentation_GH_ContentCollection.htm)

### Constructors
- `public GH_ContentCollection()` — Initializes a new instance of the GH_ContentCollection class

### Methods
#### `public override string ToString()`

(Overrides Object.ToString().)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Documentation_GH_ContentCollection_ToString.htm)

### Properties
- `Content` (List<IGH_Content>, get) — 
- `Count` (Int32, get) — 
- `Item` (IGH_Content, get) — 

## GH_Format (class)

Represents a piece of formatted content.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_Documentation_GH_Format.htm)

### Methods
#### `public static GH_Format Create(IGH_Content content, GH_Style style)`



**Parameters:**
- `content` (Grasshopper.Documentation.IGH_Content)
- `style` (Grasshopper.Documentation.GH_Style)

**Returns:** `GH_Format` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Documentation_GH_Format_Create.htm)

#### `public static GH_Format Create(string content, GH_Style style)`



**Parameters:**
- `content` (System.String)
- `style` (Grasshopper.Documentation.GH_Style)

**Returns:** `GH_Format` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Documentation_GH_Format_Create_1.htm)

#### `public override string ToString()`

(Overrides Object.ToString().)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Documentation_GH_Format_ToString.htm)

### Properties
- `Content` (IGH_Content, get) — 
- `Style` (GH_Style, get) — 

## GH_GlossaryItem (class)

Represents a single glossary entry in the Grasshopper help.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_Documentation_GH_GlossaryItem.htm)

### Methods
#### `public static GH_GlossaryItem ParseFile(string path)`

Parse a *.glossary file and return a glossary entry.

**Parameters:**
- `path` (System.String) — File to parse.

**Returns:** `GH_GlossaryItem` — The glossary entry defined by the file. This function always returns a valid glossary or it throws an exception.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Documentation_GH_GlossaryItem_ParseFile.htm)

### Properties
- `Author` (IGH_Content, get) — Gets the author of this glossary entry. This field is optional and may not be defined.
- `Contact` (IGH_Content, get) — Gets the contact information for the glossary entry author. This field is optional and may not be defined.
- `Descriptions` (List<IGH_Content>, get) — Gets the description (definition) for this glossary entry.
- `Path` (String, get) — Gets the source file for this glossary entry.
- `Pronunciation` (IGH_Content, get) — Gets the pronunciation guide to the word. This field is optional and may not be defined.
- `Synonyms` (ReadOnlyCollection<String>, get) — Gets the synonyms for this glossary entry.
- `Word` (String, get) — Gets the word this glossary entry is about.

## GH_Link (class)

Represents a link to another location in the Help or some external address.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_Documentation_GH_Link.htm)

### Methods
#### `public static GH_Link CreateExternalLink(string text, string url)`



**Parameters:**
- `text` (System.String)
- `url` (System.String)

**Returns:** `GH_Link` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Documentation_GH_Link_CreateExternalLink.htm)

#### `public static GH_Link CreateExternalLink(string text, string url, string tooltip)`



**Parameters:**
- `text` (System.String)
- `url` (System.String)
- `tooltip` (System.String)

**Returns:** `GH_Link` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Documentation_GH_Link_CreateExternalLink_1.htm)

#### `public static GH_Link CreateGlossaryLink(string text)`



**Parameters:**
- `text` (System.String)

**Returns:** `GH_Link` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Documentation_GH_Link_CreateGlossaryLink.htm)

#### `public static GH_Link CreateGlossaryLink(string text, string glossaryEntry)`



**Parameters:**
- `text` (System.String)
- `glossaryEntry` (System.String)

**Returns:** `GH_Link` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Documentation_GH_Link_CreateGlossaryLink_1.htm)

#### `public static GH_Link CreateSharedLink(string linkId, string target)`



**Parameters:**
- `linkId` (System.String)
- `target` (System.String)

**Returns:** `GH_Link` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Documentation_GH_Link_CreateSharedLink.htm)

#### `public static GH_Link CreateSharedLink(string linkId, string target, string tooltip)`



**Parameters:**
- `linkId` (System.String)
- `target` (System.String)
- `tooltip` (System.String)

**Returns:** `GH_Link` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Documentation_GH_Link_CreateSharedLink_1.htm)

#### `public static GH_Link CreateTopicLink(string text, string topicName)`



**Parameters:**
- `text` (System.String)
- `topicName` (System.String)

**Returns:** `GH_Link` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Documentation_GH_Link_CreateTopicLink.htm)

#### `public static GH_Link CreateTopicLink(string text, string topicName, string tooltip)`



**Parameters:**
- `text` (System.String)
- `topicName` (System.String)
- `tooltip` (System.String)

**Returns:** `GH_Link` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Documentation_GH_Link_CreateTopicLink_1.htm)

#### `public override string ToString()`

(Overrides Object.ToString().)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Documentation_GH_Link_ToString.htm)

### Properties
- `Destination` (String, get) — Gets the link target.
- `IsSharedLink` (Boolean, get) — Gets whether this link is a shared link. A shared link should not have text but it should have a non-empty LinkId.
- `LinkId` (String, get) — Gets the link ID. The link ID is only used for shared links.
- `Target` (GH_Target, get) — Gets the link type.
- `Text` (IGH_Content, get) — Gets the list text.
- `Tooltip` (String, get) — Gets the link tooltip text.

## GH_List (class)

Represents a list of items in the Grasshopper help.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_Documentation_GH_List.htm)

### Methods
#### `public static GH_List Create(bool ordered)`



**Parameters:**
- `ordered` (System.Boolean)

**Returns:** `GH_List` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Documentation_GH_List_Create.htm)

#### `public static GH_List Create(bool ordered, IEnumerable<IGH_Content> items)`



**Parameters:**
- `ordered` (System.Boolean)
- `items` (System.Collections.Generic.IEnumerable<IGH_Content>)

**Returns:** `GH_List` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Documentation_GH_List_Create_1.htm)

#### `public override string ToString()`

(Overrides Object.ToString().)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Documentation_GH_List_ToString.htm)

### Properties
- `Items` (List<IGH_Content>, get) — 
- `Ordered` (Boolean, get) — 

## GH_Paragraph (class)

(No description provided in vendor docs for Grasshopper.Documentation.GH_Paragraph.)

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_Documentation_GH_Paragraph.htm)

### Methods
#### `public static GH_Paragraph Create(string content)`



**Parameters:**
- `content` (System.String)

**Returns:** `GH_Paragraph` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Documentation_GH_Paragraph_Create.htm)

#### `public static GH_Paragraph Create(string[] content)`



**Parameters:**
- `content` (System.String[])

**Returns:** `GH_Paragraph` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Documentation_GH_Paragraph_Create_1.htm)

#### `public override string ToString()`

(Overrides Object.ToString().)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Documentation_GH_Paragraph_ToString.htm)

### Properties
- `Content` (IGH_Content, get) — 

## GH_Parser (class)

Provides parsing methods for the Grasshopper Markdown flavour.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_Documentation_GH_Parser.htm)

### Methods
#### `public static bool IsChapterHeaderLine(string line)`

Test whether a line represents a chapter header underline. Chapter header underlines contain at least 4 consecutive equals symbols preceded or followed by any amount of white space.

**Parameters:**
- `line` (System.String) — Line to test.

**Returns:** `Boolean` — True if the line is considered to be a chapter header underline.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Documentation_GH_Parser_IsChapterHeaderLine.htm)

#### `public static bool IsCommentLine(string line)`

Test whether a line is a comment. Commented lines begin with double slashes (//) and are ignored during parsing.

**Parameters:**
- `line` (System.String) — Line to test. If you supply a multi-line string, death's too good for you.

**Returns:** `Boolean` — True if the line is a comment line.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Documentation_GH_Parser_IsCommentLine.htm)

#### `public static bool IsLinkLine(string line)`

Test whether a line represents a referenced link target. Note; this is a quick check. The link might still be invalid, use the IsLinkLine overload to include validity checks.

**Parameters:**
- `line` (System.String) — Line to test.

**Returns:** `Boolean` — True if the line is a referenced link.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Documentation_GH_Parser_IsLinkLine.htm)

#### `public static bool IsLinkLine(string line, out string linkId, out string linkTarget, out string linkTooltip)`

Test whether a line represents a referenced link target.

**Parameters:**
- `line` (System.String) — Line to test.
- `linkId` (System.String)
- `linkTarget` (System.String)
- `linkTooltip` (System.String)

**Returns:** `Boolean` — True if the line is a referenced link.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Documentation_GH_Parser_IsLinkLine_1.htm)

#### `public static bool IsListLine(string line)`

Test whether a line contains a list item.

**Parameters:**
- `line` (System.String) — Line to check.

**Returns:** `Boolean` — True if the line contains a list item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Documentation_GH_Parser_IsListLine.htm)

#### `public static bool IsParagraphHeaderLine(string line)`

Test whether a line represents a paragraph header underline. Paragraph header underlines contain at least 4 consecutive dashes preceded or followed by any amount of white space.

**Parameters:**
- `line` (System.String) — Line to test.

**Returns:** `Boolean` — True if the line is considered to be a paragraph header underline.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Documentation_GH_Parser_IsParagraphHeaderLine.htm)

#### `public static bool IsQuoteLine(string line)`

Test whether a line is a block quote line. Block quotes start with a larger than symbol.

**Parameters:**
- `line` (System.String) — Line to test.

**Returns:** `Boolean` — True if the line is a block quote.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Documentation_GH_Parser_IsQuoteLine.htm)

#### `public static IGH_Content StringToFragment(string text)`

Parse a block of text and return it as an interpreted IGH_Fragment.

**Parameters:**
- `text` (System.String) — Text to parse.

**Returns:** `IGH_Content` — Fragment representing the text.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Documentation_GH_Parser_StringToFragment.htm)

### Properties
- `Whitespace` (Char[], get) — Gets an array of characters that are considered to be whitespace. At the moment, this array contains the space char and the tab char, it does not contain feeds or breaks.

## GH_RuntimeFile (class)

Runtime representation of a Grasshopper help file. A GH_RuntimeFile is nothing more than a collection of keywords with associated content. This class does not interpret a file or check its validity, it merely figures out the different content section based on "Keyword:" entries.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_Documentation_GH_RuntimeFile.htm)

### Methods
#### `public bool ContainsKey(string key)`

Test whether a certain key is present in the File.

**Parameters:**
- `key` (System.String) — Key to search for.

**Returns:** `Boolean` — True if the key is present in the file, false if not.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Documentation_GH_RuntimeFile_ContainsKey.htm)

#### `public static bool IsTag(string text)`

Tests whether a string is a predefined Grasshopper Help keyword.

**Parameters:**
- `text` (System.String) — String to test. Should contain no whitespace, but does not need to properly cased.

**Returns:** `Boolean` — True if text is a predefined keyword, false if not.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Documentation_GH_RuntimeFile_IsTag.htm)

#### `public static bool IsTagLine(string line, out string keyword, out string remainder)`

Tests whether a line of text starts with a Grasshopper Help keyword.

**Parameters:**
- `line` (System.String) — Line of text to test.
- `keyword` (System.String) — This is where the keyword is returned.
- `remainder` (System.String) — This is where the non-keyword part of the line is returned.

**Returns:** `Boolean` — True if the line starts with a keyword.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Documentation_GH_RuntimeFile_IsTagLine.htm)

#### `public static GH_RuntimeFile ParseFile(string path)`

Parse a file and return the runtime representation of said file.

**Parameters:**
- `path` (System.String) — Path to file.

**Returns:** `GH_RuntimeFile` — The runtime representation of the file on the disk.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Documentation_GH_RuntimeFile_ParseFile.htm)

### Properties
- `Content` (List<String[]>, get) — Gets the data associated with a certain key. For every time the key appears in the file, an array of content strings will be returned.
- `Keys` (List<String>, get) — Gets a list of all the keys present in the File.
- `Path` (String, get) — Gets the path of this help file.
- `Tags` (IEnumerable<String>, get) — Gets a collection of all the pre-defined keywords. Use IsKeyword(string) if you want to know whether a string equals a keyword. It's both faster and more reliable than comparing against this collection.
- `TagAuthor` (String, get) — String tag used to identify Author fields in documentation source files.
- `TagAutoLink` (String, get) — String tag used to signal whether automatic linking to a topic is allowed.
- `TagBeginner` (String, get) — String tag used t o identify Beginner Level Help Content in documentation source files.
- `TagCategory` (String, get) — String tag used to identify Category fields in documentation source files.
- `TagComponent` (String, get) — String tag used to identify Component IDs in documentation source files.
- `TagContact` (String, get) — String tag used to identify Author contact details in documentation source files.
- `TagDescription` (String, get) — String tag used t o identify Description fields in glossary source files.
- `TagErranyms` (String, get) — String tag used to identify Do Not Confuse collections in documentation source files.
- `TagExpert` (String, get) — String tag used t o identify Expert Level Help Content in documentation source files.
- `TagInclude` (String, get) — String tag used to signal whether inclusion of a topic is allowed.
- `TagIntermediate` (String, get) — String tag used t o identify Intermediate Level Help Content in documentation source files.
- `TagKeywords` (String, get) — String tag used to identify Keyword collections in documentation source files.
- `TagPronunciation` (String, get) — String tag used to identify Pronunciaion Guide fields (IPA) in glossary source files.
- `TagRhinoCommand` (String, get) — String tag used to identify Similar Rhino Commands in documentation source files.
- `TagSeeAlso` (String, get) — String tag used to identify See Also collections in documentation source files.
- `TagSynonyms` (String, get) — String tag used to identify Synonym collections in glossary source files.
- `TagTitle` (String, get) — String tag used to identify title fields in documentation source files.

## GH_Style (enum)

Enumerates possible formatting for documentation content.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_Documentation_GH_Style.htm)

### Values
- `None` = `0`
- `WeakEmphasis` = `1`
- `StrongEmphasis` = `2`
- `Monospaced` = `3`
- `Boxed` = `4`
- `ChapterHeader` = `5`
- `ParagraphHeader` = `6`

## GH_Target (enum)

Enumerates the possible link targets in Grasshopper documentation.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_Documentation_GH_Target.htm)

### Values
- `Glossary` = `0` — Specifies a link into the Glossary.
- `Topic` = `1` — Specifies a link to another topic.
- `External` = `2` — Specifies a link to some external web-page.

## GH_Text (class)

Represents a single string.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_Documentation_GH_Text.htm)

### Methods
#### `public static GH_Text Create(string text)`

Create a new instance of GH_Text.

**Parameters:**
- `text` (System.String) — Text to wrap.

**Returns:** `GH_Text` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Documentation_GH_Text_Create.htm)

#### `public override string ToString()`

(Overrides Object.ToString().)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Documentation_GH_Text_ToString.htm)

### Properties
- `Text` (String, get) — Gets the text string.

## GH_Topic (class)

Represents a single topic in the Grasshopper help.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_Documentation_GH_Topic.htm)

### Methods
#### `public static GH_Topic ParseFile(string path)`

Parse a *.topic file and return a topic entry.

**Parameters:**
- `path` (System.String) — File to parse.

**Returns:** `GH_Topic` — The topic defined by the file. This function always returns a valid topic or it throws an exception.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Documentation_GH_Topic_ParseFile.htm)

### Properties
- `Description` (GH_ContentCollection, get) — 
- `SafeDescription` (GH_ContentCollection, get) — 

## IGH_Content (interface)

All parts that make up documentation content must implement this interface.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_Documentation_IGH_Content.htm)

