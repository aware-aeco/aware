---
name: sketchup-layout
description: This skill encodes the sketchup 2025.0 surface of the Layout namespace — 32 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: AngularDimension, AutoTextDefinition, AutoTextDefinitions, ConnectionPoint, Dictionary, Document, Ellipse, Entities, and 24 more types.
---

# Layout

Auto-generated from vendor docs for sketchup 2025.0. 32 types in this namespace.

## AngularDimension (class)

References an angular dimension entity. An AngularDimension is composed of the following sub-entities: two 'extension lines' that extend from the Entity being dimensioned. a 'dimension line' connecting the ends of the leaders. This may be represented by one or two Paths depending on appearance. an optional 'leader line' that is used for small AngularDimensions. a 'dimension text' that displays the AngularDimension's text. There are seven points that may be modified for an AngularDimension: two 'connection points' that define the start and end of the AngularDimension. two 'extent points' that define the start and end of the dimension line and are the ends of the two extension lines. two 'offset points' that define the starting points of the extension lines. one 'arc center point' that defines the center of the AngularDimension, where the extension lines intersect.

[Vendor docs](https://ruby.sketchup.com/Layout/AngularDimension.html)

### Constructors
- `AngularDimension(start_point, end_point, start_extent_point, end_extent_point, inner_angle)` — The #initialize method creates a new disconnected Layout::AngularDimension. inner_angle)

### Methods
#### `angle => Numeric`

The #angle method returns the Layout::AngularDimension's angle.

**Remarks:** The #angle method returns the Layout::AngularDimension's angle. The angle is represented in radians. inner_angle) angle = dim.angle

**Returns:** `Numeric` — 

[Docs](https://ruby.sketchup.com/Layout/AngularDimension.html#angle-instance_method)

#### `arc_center_point => Geom::Point2d`

The #arc_center_point method returns the paper space location for the dimension arc center point.

**Remarks:** The #arc_center_point method returns the paper space location for the dimension arc center point. inner_angle) center = dim.arc_center_point

**Returns:** `Geom::Point2d` — 

[Docs](https://ruby.sketchup.com/Layout/AngularDimension.html#arc_center_point-instance_method)

#### `custom_text? => Boolean`

The #custom_text? method returns whether the Layout::AngularDimension uses custom text.

**Remarks:** The #custom_text? method returns whether the Layout::AngularDimension uses custom text. When true, the text will display a custom string that doesn't change. When false, the text will display the length measurement and will update automatically. inner_angle) # returns false uses_custom_text = dim.custom_text?

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Layout/AngularDimension.html#custom_text?-instance_method)

#### `end_offset_point => Geom::Point2d`

The #end_offset_point method returns the paper space location for the end of the first extension line.

**Remarks:** The #end_offset_point method returns the paper space location for the end of the first extension line. The first extension line runs from this offset point to the end extent point. inner_angle) end_offset = dim.end_offset_point

**Returns:** `Geom::Point2d` — end_offset

[Docs](https://ruby.sketchup.com/Layout/AngularDimension.html#end_offset_point-instance_method)

#### `entities => Layout::Entities`

The #entities method returns the Entities that represent the Layout::AngularDimension in its exploded form.

**Remarks:** The #entities method returns the Entities that represent the Layout::AngularDimension in its exploded form. Depending on the appearance of the Layout::AngularDimension being exploded, this may return anywhere from four to six Entitys: start extension line, end extension line, one or two dimension lines, dimension text, and optionally the leader line. inner_angle) entites = dim.entities

**Returns:** `Layout::Entities` — 

[Docs](https://ruby.sketchup.com/Layout/AngularDimension.html#entities-instance_method)

#### `start_offset_point => Geom::Point2d`

The #start_offset_point method returns the paper space location for the start of the first extension line.

**Remarks:** The #start_offset_point method returns the paper space location for the start of the first extension line. The first extension line runs from this offset point to the start extent point. inner_angle) start_offset = dim.start_offset_point

**Returns:** `Geom::Point2d` — start_offset

[Docs](https://ruby.sketchup.com/Layout/AngularDimension.html#start_offset_point-instance_method)

### Properties
- `custom_text` (Boolean, set) — The #custom_text= sets whether or not the Layout::AngularDimension uses custom text.
- `end_connection_point` (Geom::Point2d, get/set) — The #end_connection_point method returns the paper space location for the second connection.
- `end_extent_point` (Geom::Point2d, get/set) — The #end_extent_point method returns the paper space location for the end of the dimension line.
- `end_offset_length` (Numeric, set) — The #end_offset_length= method sets the length of the offset from the second connection point to the start of the second extension line.
- `leader_line_type` (Integer, get/set) — The #leader_line_type method returns the type of leader line the Layout::AngularDimension is using.
- `radius` (Numeric, get/set) — The #radius method returns the Layout::AngularDimension's radius.
- `start_connection_point` (Geom::Point2d, get/set) — The #start_connection_point method returns the paper space location for the first connection.
- `start_extent_point` (Geom::Point2d, get/set) — The #start_extent_point method returns the paper space location for the start of the dimension line.
- `start_offset_length` (Numeric, set) — The #start_offset_length= method sets the length of the offset from the first connection point to the start of the first extension line.
- `text` (Layout::FormattedText, get/set) — Note: With the addition of auto-text in dimensions for LayOut 2019.
- `LEADER_LINE_TYPE_SINGLE_SEGMENT` (Object, get) — 
- `LEADER_LINE_TYPE_TWO_SEGMENT` (Object, get) — 
- `LEADER_LINE_TYPE_BEZIER` (Object, get) — 
- `LEADER_LINE_TYPE_HIDDEN` (Object, get) — 

## AutoTextDefinition (class)

References an auto-text definition. Some auto-text definitions are mandatory. Mandatory auto-text definitions may not be removed, added, or modified. A mandatory AutoTextDefinition is one of the following types: Layout::AutoTextDefinition::TYPE_MODEL_GROUP_NAME Layout::AutoTextDefinition::TYPE_MODEL_COMPONENT_INSTANCE_NAME Layout::AutoTextDefinition::TYPE_MODEL_COMPONENT_DEFINITION_NAME Layout::AutoTextDefinition::TYPE_MODEL_COMPONENT_DESCRIPTION Layout::AutoTextDefinition::TYPE_MODEL_VOLUME Layout::AutoTextDefinition::TYPE_MODEL_FACE_AREA Layout::AutoTextDefinition::TYPE_MODEL_EDGE_LENGTH Layout::AutoTextDefinition::TYPE_MODEL_COORDINATES Layout::AutoTextDefinition::TYPE_MODEL_DYNAMIC_COMPONENT_ATTRIBUTE Layout::AutoTextDefinition::TYPE_MODEL_CLASSIFIER_ATTRIBUTE Layout::AutoTextDefinition::TYPE_MODEL_SCENE_NAME Layout::AutoTextDefinition::TYPE_MODEL_SCENE_DESCRIPTION Layout::AutoTextDefinition::TYPE_MODEL_SCALE Layout::AutoTextDefinition::TYPE_MODEL_SECTION_NAME Layout::AutoTextDefinition::TYPE_MODEL_SECTION_SYMBOL

[Vendor docs](https://ruby.sketchup.com/Layout/AutoTextDefinition.html)

### Methods
#### `display_file_extension? => Boolean`

The #display_file_extension? method returns whether the Layout::AutoTextDefinition::TYPE_FILE Layout::AutoTextDefinition displays the file extension.

**Remarks:** The #display_file_extension? method returns whether the Layout::AutoTextDefinition::TYPE_FILE Layout::AutoTextDefinition displays the file extension.

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Layout/AutoTextDefinition.html#display_file_extension?-instance_method)

#### `display_full_path? => Boolean`

The #display_full_path? method returns whether the Layout::AutoTextDefinition::TYPE_FILE Layout::AutoTextDefinition displays the full path.

**Remarks:** The #display_full_path? method returns whether the Layout::AutoTextDefinition::TYPE_FILE Layout::AutoTextDefinition displays the full path.

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Layout/AutoTextDefinition.html#display_full_path?-instance_method)

#### `mandatory? => Boolean`

The #mandatory? method returns whether the Layout::AutoTextDefinition is mandatory or not.

**Remarks:** The #mandatory? method returns whether the Layout::AutoTextDefinition is mandatory or not. A mandatory Layout::AutoTextDefinition is one of the following types: Layout::AutoTextDefinition::TYPE_MODEL_GROUP_NAME Layout::AutoTextDefinition::TYPE_MODEL_COMPONENT_INSTANCE_NAME Layout::AutoTextDefinition::TYPE_MODEL_COMPONENT_DEFINITION_NAME Layout::AutoTextDefinition::TYPE_MODEL_COMPONENT_DESCRIPTION Layout::AutoTextDefinition::TYPE_MODEL_VOLUME Layout::AutoTextDefinition::TYPE_MODEL_FACE_AREA Layout::AutoTextDefinition::TYPE_MODEL_EDGE_LENGTH Layout::AutoTextDefinition::TYPE_MODEL_COORDINATES Layout::AutoTextDefinition::TYPE_MODEL_DYNAMIC_COMPONENT_ATTRIBUTE Layout::AutoTextDefinition::TYPE_MODEL_CLASSIFIER_ATTRIBUTE Layout::AutoTextDefinition::TYPE_MODEL_SCENE_NAME Layout::AutoTextDefinition::TYPE_MODEL_SCENE_DESCRIPTION Layout::AutoTextDefinition::TYPE_MODEL_SCALE Layout::AutoTextDefinition::TYPE_MODEL_SECTION_NAME Layout::AutoTextDefinition::TYPE_MODEL_SECTION_SYMBOL

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Layout/AutoTextDefinition.html#mandatory?-instance_method)

#### `renumber => Object`

The #renumber method iterates through all uses of the Layout::AutoTextDefinition::TYPE_SEQUENCE Layout::AutoTextDefinition and eliminates gaps and duplicates in the sequence.

**Remarks:** The #renumber method iterates through all uses of the Layout::AutoTextDefinition::TYPE_SEQUENCE Layout::AutoTextDefinition and eliminates gaps and duplicates in the sequence.

[Docs](https://ruby.sketchup.com/Layout/AutoTextDefinition.html#renumber-instance_method)

#### `tag => String`

The #tag method returns the tag string of the Layout::AutoTextDefinition.

**Remarks:** The #tag method returns the tag string of the Layout::AutoTextDefinition.

**Returns:** `String` — 

[Docs](https://ruby.sketchup.com/Layout/AutoTextDefinition.html#tag-instance_method)

#### `type => Integer`

The #type method returns the type of the Layout::AutoTextDefinition.

**Remarks:** The #type method returns the type of the Layout::AutoTextDefinition. The type can be one of the following values: Layout::AutoTextDefinition::TYPE_FILE Layout::AutoTextDefinition::TYPE_PAGE_NAME Layout::AutoTextDefinition::TYPE_PAGE_NUMBER Layout::AutoTextDefinition::TYPE_CUSTOM_TEXT Layout::AutoTextDefinition::TYPE_DATE_CURRENT Layout::AutoTextDefinition::TYPE_DATE_CREATED Layout::AutoTextDefinition::TYPE_DATE_MODIFIED Layout::AutoTextDefinition::TYPE_DATE_PUBLISHED Layout::AutoTextDefinition::TYPE_MODEL_GROUP_NAME Layout::AutoTextDefinition::TYPE_MODEL_COMPONENT_INSTANCE_NAME Layout::AutoTextDefinition::TYPE_MODEL_COMPONENT_DEFINITION_NAME Layout::AutoTextDefinition::TYPE_MODEL_COMPONENT_DESCRIPTION Layout::AutoTextDefinition::TYPE_MODEL_VOLUME Layout::AutoTextDefinition::TYPE_MODEL_FACE_AREA Layout::AutoTextDefinition::TYPE_MODEL_EDGE_LENGTH Layout::AutoTextDefinition::TYPE_MODEL_COORDINATES Layout::AutoTextDefinition::TYPE_MODEL_DYNAMIC_COMPONENT_ATTRIBUTE Layout::AutoTextDefinition::TYPE_MODEL_CLASSIFIER_ATTRIBUTE Layout::AutoTextDefinition::TYPE_MODEL_SCENE_NAME Layout::AutoTextDefinition::TYPE_MODEL_SCENE_DESCRIPTION Layout::AutoTextDefinition::TYPE_MODEL_SCALE Layout::AutoTextDefinition::TYPE_MODEL_SECTION_NAME Layout::AutoTextDefinition::TYPE_MODEL_SECTION_SYMBOL Layout::AutoTextDefinition::TYPE_PAGE_COUNT Layout::AutoTextDefinition::TYPE_SEQUENCE

**Returns:** `Integer` — 

[Docs](https://ruby.sketchup.com/Layout/AutoTextDefinition.html#type-instance_method)

### Properties
- `=` (Layout::AutoTextDefinition, set) — The #== method checks to see if the two Layout::AutoTextDefinitions are equal.
- `custom_text` (String, get/set) — The #custom_text method returns the custom text of the Layout::AutoTextDefinition::TYPE_CUSTOM_TEXT Layout::AutoTextDefinition.
- `date_format` (String, get/set) — The #date_format method returns the date format of a Layout::AutoTextDefinition::TYPE_DATE_* Layout::AutoTextDefinition.
- `display_file_extension` (Boolean, set) — The #display_file_extension= method sets whether the Layout::AutoTextDefinition::TYPE_FILE Layout::AutoTextDefinition displays the file extension.
- `display_full_path` (Boolean, set) — The #display_full_path= method sets whether the Layout::AutoTextDefinition::TYPE_FILE Layout::AutoTextDefinition displays the full path.
- `end_page` (Layout::Page, nil, get/set) — The #end_page method returns the end page for the Layout::AutoTextDefinition::TYPE_PAGE_COUNT Layout::AutoTextDefinition.
- `increment` (Integer, get/set) — The #increment method returns the increment value for Layout::AutoTextDefinition::TYPE_SEQUENCE Layout::AutoTextDefinitions.
- `name` (String, get/set) — The #name method returns the name of the Layout::AutoTextDefinition.
- `number_style` (Integer, get/set) — The #number_style method returns the numbering style for Layout::AutoTextDefinition::TYPE_PAGE_NUMBER, Layout::AutoTextDefinition::TYPE_PAGE_COUNT, and Layout::AutoTextDefinition::TYPE_SEQUENCE Layout::AutoTextDefinitions.
- `page_number_style` (Integer, get/set) — Deprecated.
- `sequence_format` (String, get/set) — The #sequence_format method returns the sequence format of a Layout::AutoTextDefinition::TYPE_SEQUENCE Layout::AutoTextDefinition.
- `sequence_type` (Integer, get/set) — The #sequence_type method returns how the Layout::AutoTextDefinition::TYPE_SEQUENCE Layout::AutoTextDefinition operates over multiple pages in a document.
- `start_index` (Integer, get/set) — The #start_index method returns the start index for Layout::AutoTextDefinition::TYPE_PAGE_NUMBER, Layout::AutoTextDefinition::TYPE_PAGE_COUNT, and Layout::AutoTextDefinition::TYPE_SEQUENCE Layout::AutoTextDefinitions.
- `start_page` (Layout::Page, get/set) — The #start_page method returns the start page for Layout::AutoTextDefinition::TYPE_PAGE_NUMBER and Layout::AutoTextDefinition::TYPE_PAGE_COUNT Layout::AutoTextDefinitions.
- `TYPE_FILE` (Object, get) — 
- `TYPE_PAGE_NAME` (Object, get) — 
- `TYPE_PAGE_NUMBER` (Object, get) — 
- `TYPE_CUSTOM_TEXT` (Object, get) — 
- `TYPE_DATE_CURRENT` (Object, get) — 
- `TYPE_DATE_CREATED` (Object, get) — 
- `TYPE_DATE_MODIFIED` (Object, get) — 
- `TYPE_DATE_PUBLISHED` (Object, get) — 
- `TYPE_MODEL_GROUP_NAME` (Object, get) — 
- `TYPE_MODEL_COMPONENT_INSTANCE_NAME` (Object, get) — 
- `TYPE_MODEL_COMPONENT_DEFINITION_NAME` (Object, get) — 
- `TYPE_MODEL_COMPONENT_DESCRIPTION` (Object, get) — 
- `TYPE_MODEL_VOLUME` (Object, get) — 
- `TYPE_MODEL_FACE_AREA` (Object, get) — 
- `TYPE_MODEL_EDGE_LENGTH` (Object, get) — 
- `TYPE_MODEL_COORDINATES` (Object, get) — 
- `TYPE_MODEL_DYNAMIC_COMPONENT_ATTRIBUTE` (Object, get) — 
- `TYPE_MODEL_CLASSIFIER_ATTRIBUTE` (Object, get) — 
- `TYPE_MODEL_COMPONENT_INSTANCE_ATTRIBUTE` (Object, get) — 
- `TYPE_MODEL_COMPONENT_DEFINITION_ATTRIBUTE` (Object, get) — 
- `TYPE_MODEL_SCENE_NAME` (Object, get) — 
- `TYPE_MODEL_SCENE_DESCRIPTION` (Object, get) — 
- `TYPE_MODEL_SCALE` (Object, get) — 
- `TYPE_MODEL_SECTION_NAME` (Object, get) — 
- `TYPE_MODEL_SECTION_SYMBOL` (Object, get) — 
- `TYPE_PAGE_COUNT` (Object, get) — 
- `TYPE_SEQUENCE` (Object, get) — 
- `TYPE_MODEL_RATIO` (Object, get) — 
- `NUMBER_STYLE_ARABIC` (Object, get) — 
- `NUMBER_STYLE_ARABIC_PADDED` (Object, get) — 
- `NUMBER_STYLE_UC_ALPHA` (Object, get) — 
- `NUMBER_STYLE_LC_ALPHA` (Object, get) — 
- `NUMBER_STYLE_UC_ROMAN` (Object, get) — 
- `NUMBER_STYLE_LC_ROMAN` (Object, get) — 
- `SEQUENCE_TYPE_PER_DOCUMENT` (Object, get) — 
- `SEQUENCE_TYPE_PER_PAGE` (Object, get) — 

## AutoTextDefinitions (class)

The AutoTextDefinitions class is a container class for all AutoTextDefinitions in a Document.

[Vendor docs](https://ruby.sketchup.com/Layout/AutoTextDefinitions.html)

### Methods
#### `[](index) => Layout::AutoTextDefinition | [](name) => Layout::AutoTextDefinition`

The #[] method returns a value from the array of Layout::AutoTextDefinitions.

**Remarks:** The #[] method returns a value from the array of Layout::AutoTextDefinitions.

**Parameters:**
- `index` (Integer) — The index of the Layout::AutoTextDefinition to return

**Returns:** `Layout::AutoTextDefinition` — 

[Docs](https://ruby.sketchup.com/Layout/AutoTextDefinitions.html#[]-instance_method)

#### `add(name, type) => Layout::AutoTextDefinition`

The #add method adds an Layout::AutoTextDefinition to the Document.

**Remarks:** The #add method adds an Layout::AutoTextDefinition to the Document. The type can be one of the following values: Layout::AutoTextDefinition::TYPE_FILE Layout::AutoTextDefinition::TYPE_PAGE_NAME Layout::AutoTextDefinition::TYPE_PAGE_NUMBER Layout::AutoTextDefinition::TYPE_CUSTOM_TEXT Layout::AutoTextDefinition::TYPE_DATE_CURRENT Layout::AutoTextDefinition::TYPE_DATE_CREATED Layout::AutoTextDefinition::TYPE_DATE_MODIFIED Layout::AutoTextDefinition::TYPE_DATE_PUBLISHED Layout::AutoTextDefinition::TYPE_PAGE_COUNT Layout::AutoTextDefinition::TYPE_SEQUENCE

**Parameters:**
- `name` (String)
- `type` (Integer)

**Returns:** `Layout::AutoTextDefinition` — 

[Docs](https://ruby.sketchup.com/Layout/AutoTextDefinitions.html#add-instance_method)

#### `each {|auto_text| ... } => Object`

Note: Don't remove content from this collection while iterating over it with #each.

**Remarks:** Note: Don't remove content from this collection while iterating over it with #each. This would change the size of the collection and cause elements to be skipped as the indices change. Instead copy the current collection to an array using to_a and then use each on the array, when removing content. The #each method iterates through all of the Layout::AutoTextDefinitions.

[Docs](https://ruby.sketchup.com/Layout/AutoTextDefinitions.html#each-instance_method)

#### `index(auto_text) => Integer? | index(name) => Integer?`

The #index method returns the index of the Layout::AutoTextDefinition, or nil if it doesn't exist in the Document.

**Remarks:** The #index method returns the index of the Layout::AutoTextDefinition, or nil if it doesn't exist in the Document.

**Parameters:**
- `auto_text` (Layout::AutoTextDefinition)

**Returns:** `Integer, nil` — 

[Docs](https://ruby.sketchup.com/Layout/AutoTextDefinitions.html#index-instance_method)

#### `length => Integer Also known as: size`

The #length method returns the number of Layout::AutoTextDefinitions.

**Remarks:** The #length method returns the number of Layout::AutoTextDefinitions.

**Returns:** `Integer` — 

[Docs](https://ruby.sketchup.com/Layout/AutoTextDefinitions.html#length-instance_method)

#### `remove(definition, convert_tags_to_text = true) => Object | remove(name, convert_tags_to_text = true) => Object | remove(index, convert_tags_to_text = true) => Object`

The #remove method removes an Layout::AutoTextDefinition from the Document.

**Remarks:** The #remove method removes an Layout::AutoTextDefinition from the Document. The Layout::AutoTextDefinition must be one of the following types: Layout::AutoTextDefinition::TYPE_FILE Layout::AutoTextDefinition::TYPE_PAGE_NAME Layout::AutoTextDefinition::TYPE_PAGE_NUMBER Layout::AutoTextDefinition::TYPE_CUSTOM_TEXT Layout::AutoTextDefinition::TYPE_DATE_CURRENT Layout::AutoTextDefinition::TYPE_DATE_CREATED Layout::AutoTextDefinition::TYPE_DATE_MODIFIED Layout::AutoTextDefinition::TYPE_DATE_PUBLISHED Layout::AutoTextDefinition::TYPE_PAGE_COUNT Layout::AutoTextDefinition::TYPE_SEQUENCE

**Parameters:**
- `definition` (Layout::AutoTextDefinition)
- `convert_tags_to_text` (Boolean)

[Docs](https://ruby.sketchup.com/Layout/AutoTextDefinitions.html#remove-instance_method)

## ConnectionPoint (class)

This is the interface to a LayOut Connection Point. A ConnectionPoint defines a target point to which a Label or LinearDimension can connect.

[Vendor docs](https://ruby.sketchup.com/Layout/ConnectionPoint.html)

### Constructors
- `ConnectionPoint(entity, point, aperture = 0.0001)` — The #initialize method creates a new Layout::ConnectionPoint.

## Dictionary (class)

This is the interface to a LayOut dictionary. A Dictionary wraps key/value pairs.

[Vendor docs](https://ruby.sketchup.com/Layout/Dictionary.html)

### Constructors
- `Dictionary(dict)` — The #initialize method creates a new Layout::Dictionary.

### Methods
#### `delete_key(key) => String, ...`

The #delete_key method deletes a key/value pair from the dictionary.

**Remarks:** The #delete_key method deletes a key/value pair from the dictionary.

**Parameters:**
- `key` (String) — The key to be deleted.

**Returns:** `String, Boolean, Integer, Float, Layout::Dictionary, nil` — 

[Docs](https://ruby.sketchup.com/Layout/Dictionary.html#delete_key-instance_method)

#### `each {|key, value| ... } => Object`

The #each_pair method is an alias for #each.

**Remarks:** The #each_pair method is an alias for #each.

[Docs](https://ruby.sketchup.com/Layout/Dictionary.html#each-instance_method)

#### `each_key {|key| ... } => Object`

The #each_key method iterates through all of the dictionary keys.

**Remarks:** The #each_key method iterates through all of the dictionary keys.

[Docs](https://ruby.sketchup.com/Layout/Dictionary.html#each_key-instance_method)

#### `each_pair {|key, value| ... } => Object`

The #each_pair method is an alias for #each.

**Remarks:** The #each_pair method is an alias for #each.

[Docs](https://ruby.sketchup.com/Layout/Dictionary.html#each_pair-instance_method)

#### `empty? => Boolean`

The #empty? method checks if the dictionary is empty.

**Remarks:** The #empty? method checks if the dictionary is empty.

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Layout/Dictionary.html#empty?-instance_method)

#### `keys => Array<String>`

The #keys method retrieves an array with all of the dictionary keys.

**Remarks:** The #keys method retrieves an array with all of the dictionary keys.

**Returns:** `Array<String>` — an array of keys within the dictionary if successful

[Docs](https://ruby.sketchup.com/Layout/Dictionary.html#keys-instance_method)

#### `length => Integer`

The #length method retrieves the size (number of elements) of a dictionary.

**Remarks:** The #length method retrieves the size (number of elements) of a dictionary.

**Returns:** `Integer` — 

[Docs](https://ruby.sketchup.com/Layout/Dictionary.html#length-instance_method)

#### `size => Integer`

The #length method retrieves the size (number of elements) of a dictionary.

**Remarks:** The #length method retrieves the size (number of elements) of a dictionary.

**Returns:** `Integer` — 

[Docs](https://ruby.sketchup.com/Layout/Dictionary.html#size-instance_method)

#### `values => Array<Object>`

The #values method retrieves an array with all of the dictionary values.

**Remarks:** The #values method retrieves an array with all of the dictionary values.

**Returns:** `Array<Object>` — an array of values within the dictionary if successful

[Docs](https://ruby.sketchup.com/Layout/Dictionary.html#values-instance_method)

### Properties
- `[]` (String, Boolean, Integer, Float, Layout::Dictionary, nil, get/set) — The #[] method retrieves the value for a given key.

## Document (class)

This is the interface to a LayOut document. A Document is the 2D drawing that the user is working with, and it serves as the “entry point” for most Ruby API interactions. The Document.open method gives you a handle to a Document, and from there you can use the document-level methods to start getting information and making changes.

[Vendor docs](https://ruby.sketchup.com/Layout/Document.html)

### Constructors
- `Document(template_path)` — The #initialize method creates a new Layout::Document. Passing a path to an existing Layout::Document will use that file as a template. The new Layout::Document won't have a path until it is saved for the first time.

### Methods
#### `add_entity(entity, layer, page) => Layout::Entity | add_entity(entity, layer) => Layout::Entity`

The #add_entity method adds an Entity to the Layout::Document and places it on the given Layer and Page.

**Remarks:** The #add_entity method adds an Entity to the Layout::Document and places it on the given Layer and Page. If layer is a shared Layer then page may be ommitted. The Entity must not already belong to a Layout::Document. If the Entity is a Group, then the Group along with all of its children will be added to the Layout::Document.

**Parameters:**
- `entity` (Layout::Entity) — The Entity to be added
- `layer` (Layout::Layer) — The Layer to add the Entity to
- `page` (Layout::Page) — The Page to add the Entity to

**Returns:** `Layout::Entity` — The Entity that was added to the Layout::Document.

[Docs](https://ruby.sketchup.com/Layout/Document.html#add_entity-instance_method)

#### `attribute_dictionary(name) => Layout::Dictionary?`

The #attribute_dictionary method returns a copy of the document's attribute dictionary with the given name.

**Remarks:** The #attribute_dictionary method returns a copy of the document's attribute dictionary with the given name. is no attribute dictionary

**Parameters:**
- `name` (String)

**Returns:** `Layout::Dictionary, nil` — A copy of the document's attribute dictionary, or nil if there

[Docs](https://ruby.sketchup.com/Layout/Document.html#attribute_dictionary-instance_method)

#### `auto_text_definitions => Layout::AutoTextDefinitions`

The #auto_text_definitions method returns an array of AutoTextDefinition's in the Layout::Document.

**Remarks:** The #auto_text_definitions method returns an array of AutoTextDefinition's in the Layout::Document.

**Returns:** `Layout::AutoTextDefinitions` — 

[Docs](https://ruby.sketchup.com/Layout/Document.html#auto_text_definitions-instance_method)

#### `delete_attribute(dictionary_name) => Boolean | delete_attribute(dictionary_name, key) => Boolean`

The #delete_attribute method is used to delete an attribute from a document.

**Remarks:** The #delete_attribute method is used to delete an attribute from a document.

**Parameters:**
- `dictionary_name` (String) — The name of an attribute dictionary.

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Layout/Document.html#delete_attribute-instance_method)

#### `export(file_path, options = nil) => Object`

The #export method exports the Layout::Document to a given file format.

**Remarks:** The #export method exports the Layout::Document to a given file format. It knows which format to export based on the file extension you place on the file name. For example, a filename of “thing.pdf” will export a PDF file, whereas “thing.png” will export a set of PNG images. For LayOut version 2020.1, valid extensions include .pdf, .jpg, and .png.

**Parameters:**
- `file_path` (String) — The file or image set to create. The directory path must already exist. The path must include the file extension.
- `options` (Hash, nil) — An optional hash of settings for the export.

[Docs](https://ruby.sketchup.com/Layout/Document.html#export-instance_method)

#### `get_attribute(name, key, default_value = nil) => String, ...`

The #get_attribute method is used to retrieve the value of an attribute in the document's attribute dictionary.

**Remarks:** The #get_attribute method is used to retrieve the value of an attribute in the document's attribute dictionary. If the third parameter, default_value, is not passed and there is no attribute that matches the given name, it returns nil. If default_value is provided and there is no matching attribute it returns the given value. It does not create an attribute with that name though.

**Parameters:**
- `name` (String) — The name of an attribute dictionary.
- `key` (String) — An attribute key.
- `default_value` (String, Boolean, Integer, Float, Hash, Layout::Dictionary, nil) — A default value to return if no attribute is found.

**Returns:** `String, Boolean, Integer, Float, Layout::Dictionary, nil` — the retrieved value.

[Docs](https://ruby.sketchup.com/Layout/Document.html#get_attribute-instance_method)

#### `grid => Layout::Grid`

The #grid method returns the Grid for a Layout::Document.

**Remarks:** The #grid method returns the Grid for a Layout::Document.

**Returns:** `Layout::Grid` — 

[Docs](https://ruby.sketchup.com/Layout/Document.html#grid-instance_method)

#### `grid_snap_enabled? => Boolean`

The #grid_snap_enabled? method returns whether or not grid snap is enabled in the Layout::Document.

**Remarks:** The #grid_snap_enabled? method returns whether or not grid snap is enabled in the Layout::Document.

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Layout/Document.html#grid_snap_enabled?-instance_method)

#### `layers => Layout::Layers`

The #layers method returns the Layers of the Layout::Document.

**Remarks:** The #layers method returns the Layers of the Layout::Document.

**Returns:** `Layout::Layers` — 

[Docs](https://ruby.sketchup.com/Layout/Document.html#layers-instance_method)

#### `object_snap_enabled? => Boolean`

The #object_snap_enabled? method returns whether or not inference is enabled in the Layout::Document.

**Remarks:** The #object_snap_enabled? method returns whether or not inference is enabled in the Layout::Document.

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Layout/Document.html#object_snap_enabled?-instance_method)

#### `open(path) => Layout::Document`

The open method creates a new Layout::Document by loading an existing .

**Remarks:** The open method creates a new Layout::Document by loading an existing .layout file.

**Parameters:**
- `path` (String) — The path to the .layout file on disk.

**Returns:** `Layout::Document` — The Layout::Document created from the .layout file.

[Docs](https://ruby.sketchup.com/Layout/Document.html#open-class_method)

#### `page_info => Layout::PageInfo`

The #page_info method returns a reference to the PageInfo settings of the Layout::Document.

**Remarks:** The #page_info method returns a reference to the PageInfo settings of the Layout::Document.

**Returns:** `Layout::PageInfo` — 

[Docs](https://ruby.sketchup.com/Layout/Document.html#page_info-instance_method)

#### `pages => Layout::Pages`

The #pages method returns the Pages of the Layout::Document.

**Remarks:** The #pages method returns the Pages of the Layout::Document. @example:

**Returns:** `Layout::Pages` — The Pages for the Layout::Document.

[Docs](https://ruby.sketchup.com/Layout/Document.html#pages-instance_method)

#### `path => String`

The #path method returns the full path of the Layout::Document file.

**Remarks:** The #path method returns the full path of the Layout::Document file. An empty string is returned for a new Layout::Document (one which has not been saved and opened).

**Returns:** `String` — 

[Docs](https://ruby.sketchup.com/Layout/Document.html#path-instance_method)

#### `remove_entity(entity) => Object`

The #remove_entity method removes an Entity from the Layout::Document.

**Remarks:** The #remove_entity method removes an Entity from the Layout::Document. If entity is a Group, then the Group and all of its children will be removed from the Layout::Document.

**Parameters:**
- `entity` (Layout::Entity) — The Entity to be removed

[Docs](https://ruby.sketchup.com/Layout/Document.html#remove_entity-instance_method)

#### `save => Object | save(path, version = Layout::Document::VERSION_CURRENT) => Object`

The #save method saves the Layout::Document to a file at the given path.

**Remarks:** The #save method saves the Layout::Document to a file at the given path. Passing an empty path string will save the Layout::Document at its current path.

**Parameters:**
- `path` (String) — The path to the .layout file on disk.
- `version` (Integer) — LayOut file format to save.

[Docs](https://ruby.sketchup.com/Layout/Document.html#save-instance_method)

#### `set_attribute(name, key, value) => Object`

The #set_attribute method adds an attribute to the document's attribute dictionary.

**Remarks:** The #set_attribute method adds an attribute to the document's attribute dictionary.

**Parameters:**
- `name` (String) — The name of an attribute dictionary. @param [String] key An attribute key. @param [String, Boolean, Integer, Float, Hash, Layout::Dictionary, nil] value The value for the attribute.

[Docs](https://ruby.sketchup.com/Layout/Document.html#set_attribute-instance_method)

#### `shared_entities => Layout::Entities`

The #shared_entities method returns the Entities that exist on shared Layers in the Layout::Document.

**Remarks:** The #shared_entities method returns the Entities that exist on shared Layers in the Layout::Document.

**Returns:** `Layout::Entities` — 

[Docs](https://ruby.sketchup.com/Layout/Document.html#shared_entities-instance_method)

#### `time_created => Time`

The #time_created method returns the time when the Layout::Document was created.

**Remarks:** The #time_created method returns the time when the Layout::Document was created.

**Returns:** `Time` — time when the Layout::Document was created

[Docs](https://ruby.sketchup.com/Layout/Document.html#time_created-instance_method)

#### `time_modified => Time`

The #time_modified method returns the last time the Layout::Document was modified.

**Remarks:** The #time_modified method returns the last time the Layout::Document was modified.

**Returns:** `Time` — time when the Layout::Document was last modified

[Docs](https://ruby.sketchup.com/Layout/Document.html#time_modified-instance_method)

#### `time_published => Time`

The #time_published method returns the time when the Layout::Document was published.

**Remarks:** The #time_published method returns the time when the Layout::Document was published.

**Returns:** `Time` — time when the Layout::Document was published

[Docs](https://ruby.sketchup.com/Layout/Document.html#time_published-instance_method)

### Properties
- `=` (Layout::Document, set) — The #== method checks to see if the two Layout::Documents are equal.
- `grid_snap_enabled` (Boolean, set) — The #grid_snap_enabled= method sets whether or not grid snap is enabled in the Layout::Document.
- `object_snap_enabled` (Boolean, set) — The #object_snap_enabled= method enables or disables inference in the Layout::Document.
- `precision` (Float, get/set) — The #precision method returns the precision for the Layout::Document.
- `render_mode_override` (Integer, get/set) — The #render_mode_override method returns the override setting for output render modes of SketchUpModels in the Layout::Document.
- `units` (Integer, get/set) — The #units method returns the units for the Layout::Document.
- `VERSION_1` (Object, get) — 
- `VERSION_2` (Object, get) — 
- `VERSION_3` (Object, get) — 
- `VERSION_2013` (Object, get) — 
- `VERSION_2014` (Object, get) — 
- `VERSION_2015` (Object, get) — 
- `VERSION_2016` (Object, get) — 
- `VERSION_2017` (Object, get) — 
- `VERSION_2018` (Object, get) — 
- `VERSION_2019` (Object, get) — 
- `VERSION_2020` (Object, get) — 
- `VERSION_2021` (Object, get) — 
- `VERSION_2022` (Object, get) — 
- `VERSION_2023` (Object, get) — 
- `VERSION_CURRENT` (Object, get) — 
- `FRACTIONAL_INCHES` (Object, get) — 
- `DECIMAL_INCHES` (Object, get) — 
- `DECIMAL_FEET` (Object, get) — 
- `DECIMAL_MILLIMETERS` (Object, get) — 
- `DECIMAL_CENTIMETERS` (Object, get) — 
- `DECIMAL_METERS` (Object, get) — 
- `DECIMAL_POINTS` (Object, get) — 

## Ellipse (class)

A simple elliptical shape entity.

[Vendor docs](https://ruby.sketchup.com/Layout/Ellipse.html)

### Constructors
- `Ellipse(bounds)` — The #initialize method creates a new Layout::Ellipse.

## Entities (class)

The Entities class is a container class for Entitys. A Entities object is different from a SketchUp::Entities object in that it is read-only. Adding or removing Entitys from a Document happens with the Document#add_entity and Document#remove_entity methods. The Entities from AngularDimension#entities, Label#entities, LinearDimension#entities, or Table#entities contains the Entitys that represent the Entity in its exploded form. The Entities from Group#entities contains all the Entitys that belong to the Group. The Entities from Page#entities contains all of the Entitys on both shared and non-shared Layers. This class is used to iterate through the Entitys in draw order or pick order (reverse draw order) using the #each and #reverse_each methods. The Entities from Document#shared_entities contains all of the Entitys that belong on all shared Layers. The Entities from Page#nonshared_entities contains all of the Entitys that belong to that Page. The Entities from LayerInstance#entities contains all of the Entitys that belong on that LayerInstance.

[Vendor docs](https://ruby.sketchup.com/Layout/Entities.html)

### Methods
#### `[](index) => Layout::Entity`

The #[] method returns the Layout::Entity at the given index.

**Remarks:** The #[] method returns the Layout::Entity at the given index. This method is not valid for use when the Layout::Entities object came from a Page.

**Parameters:**
- `index` (Integer)

**Returns:** `Layout::Entity` — 

[Docs](https://ruby.sketchup.com/Layout/Entities.html#[]-instance_method)

#### `each(flags = {}) {|entity| ... } => Object`

Note: Don't remove content from this collection while iterating over it with #each.

**Remarks:** Note: Don't remove content from this collection while iterating over it with #each. This would change the size of the collection and cause elements to be skipped as the indices change. Instead copy the current collection to an array using to_a and then use each on the array, when removing content. The #each method iterates through all of the Layout::Entitys. When iterating over a LayerInstance's Layout::Entities, it is not necessary to provide a flags Hash. When iterating over a Page's Layout::Entities, the flags Hash is optional; providing no Hash will result in iterating over all Layout::Entitys, including those on hidden or locked Layers. Valid symbols for the Hash are :skip_hidden and :skip_locked.

**Parameters:**
- `flags` (Hash{Symbol => Boolean}) — A hash that allows skipping of Layout::Entitys on hidden or locked Layers

[Docs](https://ruby.sketchup.com/Layout/Entities.html#each-instance_method)

#### `length => Integer Also known as: size`

The #length method returns the number of Layout::Entitys.

**Remarks:** The #length method returns the number of Layout::Entitys.

**Returns:** `Integer` — 

[Docs](https://ruby.sketchup.com/Layout/Entities.html#length-instance_method)

#### `reverse_each {|entity| ... } => Object | reverse_each(flags) {|entity| ... } => Object`

The #reverse_each method iterates through all of the Layout::Entitys in reverse order.

**Remarks:** The #reverse_each method iterates through all of the Layout::Entitys in reverse order. When iterating over a LayerInstance's Layout::Entities, it is not necessary to provide a flags Hash. When iterating over a Page's Layout::Entities, the flags Hash is optional; providing no Hash will result in iterating over all Layout::Entitys, including those on hidden or locked Layers. Valid symbols for the Hash are :skip_hidden and :skip_locked.

**Parameters:**
- `flags` (Hash{Symbol => Boolean}) — A hash that allows skipping of Layout::Entitys on hidden or locked Layers.

[Docs](https://ruby.sketchup.com/Layout/Entities.html#reverse_each-instance_method)

## Entity (class)

An entity is an object shown on a page of a LayOut document.

[Vendor docs](https://ruby.sketchup.com/Layout/Entity.html)

### Methods
#### `attribute_dictionary(name) => Layout::Dictionary?`

The #attribute_dictionary method returns a copy of the entity's attribute dictionary with the given name.

**Remarks:** The #attribute_dictionary method returns a copy of the entity's attribute dictionary with the given name. no attribute dictionary

**Parameters:**
- `name` (String)

**Returns:** `Layout::Dictionary, nil` — A copy of the entity's attribute dictionary, or nil if there is

[Docs](https://ruby.sketchup.com/Layout/Entity.html#attribute_dictionary-instance_method)

#### `bounds => Geom::Bounds2d`

The #bounds method returns the 2D rectangular bounds of the Layout::Entity.

**Remarks:** The #bounds method returns the 2D rectangular bounds of the Layout::Entity.

**Returns:** `Geom::Bounds2d` — 

[Docs](https://ruby.sketchup.com/Layout/Entity.html#bounds-instance_method)

#### `delete_attribute(dictionary_name) => Boolean | delete_attribute(dictionary_name, key) => Boolean`

The #delete_attribute method is used to delete an attribute from an entity.

**Remarks:** The #delete_attribute method is used to delete an attribute from an entity.

**Parameters:**
- `dictionary_name` (String) — The name of an attribute dictionary.

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Layout/Entity.html#delete_attribute-instance_method)

#### `document => Layout::Document?`

The #document method returns the Document that the Layout::Entity belongs to, or nil if it is not in a Document.

**Remarks:** The #document method returns the Document that the Layout::Entity belongs to, or nil if it is not in a Document.

**Returns:** `Layout::Document, nil` — 

[Docs](https://ruby.sketchup.com/Layout/Entity.html#document-instance_method)

#### `drawing_bounds => Geom::OrientedBounds2d`

The #drawing_bounds method returns the 2D rectangular drawing bounds of the Layout::Entity.

**Remarks:** The #drawing_bounds method returns the 2D rectangular drawing bounds of the Layout::Entity.

**Returns:** `Geom::OrientedBounds2d` — 

[Docs](https://ruby.sketchup.com/Layout/Entity.html#drawing_bounds-instance_method)

#### `get_attribute(name, key, default_value = nil) => String, ...`

The #get_attribute method is used to retrieve the value of an attribute in the entity's attribute dictionary.

**Remarks:** The #get_attribute method is used to retrieve the value of an attribute in the entity's attribute dictionary. If the third parameter, default_value, is not passed and there is no attribute that matches the given name, it returns nil. If default_value is provided and there is no matching attribute it returns the given value. It does not create an attribute with that name though.

**Parameters:**
- `name` (String) — The name of an attribute dictionary.
- `key` (String) — An attribute key.
- `default_value` (String, Boolean, Integer, Float, Hash, Layout::Dictionary, nil) — A default value to return if no attribute is found.

**Returns:** `String, Boolean, Integer, Float, Layout::Dictionary, nil` — the retrieved value.

[Docs](https://ruby.sketchup.com/Layout/Entity.html#get_attribute-instance_method)

#### `group => Layout::Group?`

The #group method returns the Group the Layout::Entity belongs to, or nil if it is not in a Group.

**Remarks:** The #group method returns the Group the Layout::Entity belongs to, or nil if it is not in a Group.

**Returns:** `Layout::Group, nil` — 

[Docs](https://ruby.sketchup.com/Layout/Entity.html#group-instance_method)

#### `layer_instance => Layout::LayerInstance?`

Note: Groups are never associated with a LayerInstance.

**Remarks:** Note: Groups are never associated with a LayerInstance. The #layer_instance method returns the LayerInstance that the Layout::Entity is on, or nil if it is not associated with a LayerInstance.

**Returns:** `Layout::LayerInstance, nil` — 

[Docs](https://ruby.sketchup.com/Layout/Entity.html#layer_instance-instance_method)

#### `locked? => Boolean`

The #locked? method returns whether the Layout::Entity is locked or unlocked.

**Remarks:** The #locked? method returns whether the Layout::Entity is locked or unlocked.

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Layout/Entity.html#locked?-instance_method)

#### `move_to_group(group) => Object`

The #move_to_group method moves the Layout::Entity into a Group.

**Remarks:** The #move_to_group method moves the Layout::Entity into a Group. If the Layout::Entity is already in a Group, it will be removed from that Group prior to being added to the new one. If this action results in the old Group containing only one Layout::Entity, the old Group will be collapsed and the remaining Layout::Entity will be moved to the old Group's parent.

**Parameters:**
- `group` (Layout::Group)

[Docs](https://ruby.sketchup.com/Layout/Entity.html#move_to_group-instance_method)

#### `move_to_layer(layer) => Object | move_to_layer(layer, pages) => Object`

The #move_to_layer method moves the Layout::Entity to the given Layer.

**Remarks:** The #move_to_layer method moves the Layout::Entity to the given Layer. If the Layer is non-shared and the Layout::Entity is currently on a shared Layer, an array of Pages must be provided to move the Layout::Entity to. In all other cases, passing in an array of Pages is not necessary. The Layout::Entity must belong to the same Document as the the Layer and the Pages.

**Parameters:**
- `layer` (Layout::Layer)

[Docs](https://ruby.sketchup.com/Layout/Entity.html#move_to_layer-instance_method)

#### `on_shared_layer? => Boolean`

The #on_shared_layer? method returns whether or not the Layout::Entity is on a shared Layer.

**Remarks:** The #on_shared_layer? method returns whether or not the Layout::Entity is on a shared Layer. This function works for all Layout::Entity types, including Group. Groups do not belong to a specific Layer, but their children are all on either a shared or non-shared Layer.

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Layout/Entity.html#on_shared_layer?-instance_method)

#### `page => Layout::Page?`

The #page method returns the Page that the Layout::Entity belongs to, or nil if it is on a shared Layer or not in a Document.

**Remarks:** The #page method returns the Page that the Layout::Entity belongs to, or nil if it is on a shared Layer or not in a Document.

**Returns:** `Layout::Page, nil` — 

[Docs](https://ruby.sketchup.com/Layout/Entity.html#page-instance_method)

#### `set_attribute(name, key, value) => Object`

The #set_attribute method adds an attribute to the entity's attribute dictionary.

**Remarks:** The #set_attribute method adds an attribute to the entity's attribute dictionary.

**Parameters:**
- `name` (String) — The name of an attribute dictionary. @param [String] key An attribute key. @param [String, Boolean, Integer, Float, Hash, Layout::Dictionary, nil] value The value for the attribute.

[Docs](https://ruby.sketchup.com/Layout/Entity.html#set_attribute-instance_method)

#### `transform!(transformation) => Object`

Note: Since LayOut 2026.

**Remarks:** Note: Since LayOut 2026.1, passing a non-invertible transformation raises an `ArgumentError`. The #transform! method transforms the Layout::Entity with a given Geom::Transformation2d.

**Parameters:**
- `transformation` (Geom::Transformation2d)

[Docs](https://ruby.sketchup.com/Layout/Entity.html#transform!-instance_method)

#### `transformation => Geom::Transformation2d?`

The #transformation method returns the explicit Geom::Transformation2d.

**Remarks:** The #transformation method returns the explicit Geom::Transformation2d.

**Returns:** `Geom::Transformation2d, nil` — 

[Docs](https://ruby.sketchup.com/Layout/Entity.html#transformation-instance_method)

### Properties
- `=` (Layout::Entity, set) — The #== method checks to see if the two Layout::Entitys are equal.
- `locked` (Boolean, set) — The #locked= method sets the Layout::Entity as locked or unlocked.
- `style` (Layout::Style, nil, get/set) — The #style method returns the Style of the Layout::Entity.
- `untransformed_bounds` (Geom::Bounds2d, get/set) — The #untransformed_bounds method returns the untransformed bounds of the Layout::Entity.

## FormattedText (class)

A formatted text entity.

[Vendor docs](https://ruby.sketchup.com/Layout/FormattedText.html)

### Constructors
- `FormattedText(text, bounds)` — The #initialize method creates a new Layout::FormattedText. The anchor type can be one of the following values: ANCHOR_TYPE_TOP_LEFT ANCHOR_TYPE_CENTER_LEFT ANCHOR_TYPE_BOTTOM_LEFT ANCHOR_TYPE_TOP_RIGHT ANCHOR_TYPE_CENTER_RIGHT ANCHOR_TYPE_BOTTOM_RIGHT ANCHOR_TYPE_TOP_CENTER ANCHOR_TYPE_CENTER_CENTER ANCHOR_TYPE_BOTTOM_CENTER

### Methods
#### `append_plain_text(plain_text, style) => Object`

Note: This method does not support more than two different style runs in a single text string.

**Remarks:** Note: This method does not support more than two different style runs in a single text string. The #append_plain_text method appends new text with a given style to the end of the existing plain text of the Layout::FormattedText.

**Parameters:**
- `plain_text` (String)
- `style` (Layout::Style)

[Docs](https://ruby.sketchup.com/Layout/FormattedText.html#append_plain_text-instance_method)

#### `apply_style(style, index = 0, length = length_to_end_of_text) => Object`

The #apply_style method sets the Style for the text starting at the given character index, and running for the given number of characters.

**Remarks:** The #apply_style method sets the Style for the text starting at the given character index, and running for the given number of characters.

**Parameters:**
- `style` (Layout::Style)
- `index` (Integer)
- `length` (Integer)

[Docs](https://ruby.sketchup.com/Layout/FormattedText.html#apply_style-instance_method)

#### `display_text(page = nil) => String`

Note: Passing an invalid Page will prevent an auto-text tag from being substituted with its display representation.

**Remarks:** Note: Passing an invalid Page will prevent an auto-text tag from being substituted with its display representation. The #display_text method returns the display text representation of the Layout::FormattedText. Layout::FormattedText::ANCHOR_TYPE_TOP_LEFT) doc.add_entity(text, doc.layers.first, doc.pages.first) text = text.display_text(doc.pages.first)

**Parameters:**
- `page` (Layout::Page) — The Page to use to convert an auto-text tag to display text

**Returns:** `String` — 

[Docs](https://ruby.sketchup.com/Layout/FormattedText.html#display_text-instance_method)

#### `new_from_file(path, bounds) => Layout::FormattedText | new_from_file(path, anchor_point, anchor_type) => Layout::FormattedText`

The new_from_file method creates a new Layout::FormattedText from a text file.

**Remarks:** The new_from_file method creates a new Layout::FormattedText from a text file. The anchor type can be one of the following values: ANCHOR_TYPE_TOP_LEFT ANCHOR_TYPE_CENTER_LEFT ANCHOR_TYPE_BOTTOM_LEFT ANCHOR_TYPE_TOP_RIGHT ANCHOR_TYPE_CENTER_RIGHT ANCHOR_TYPE_BOTTOM_RIGHT ANCHOR_TYPE_TOP_CENTER ANCHOR_TYPE_CENTER_CENTER ANCHOR_TYPE_BOTTOM_CENTER Layout::FormattedText::ANCHOR_TYPE_TOP_LEFT)

**Parameters:**
- `path` (String)
- `bounds` (Geom::Bounds2d)

**Returns:** `Layout::FormattedText` — 

[Docs](https://ruby.sketchup.com/Layout/FormattedText.html#new_from_file-class_method)

#### `style(index = 0, length = 1) => Layout::Style`

The #style method returns a Style for the text starting at the given character index, and running for the given length.

**Remarks:** The #style method returns a Style for the text starting at the given character index, and running for the given length.

**Parameters:**
- `index` (Integer)
- `length` (Integer)

**Returns:** `Layout::Style` — 

[Docs](https://ruby.sketchup.com/Layout/FormattedText.html#style-instance_method)

### Properties
- `grow_mode` (Integer, get/set) — The #grow_mode method returns the mode for how the Layout::FormattedText sizes itself.
- `plain_text` (String, get/set) — The #plain_text method returns the plain text representation of the Layout::FormattedText.
- `rtf` (String, get/set) — Note: Passing an invalid Page will prevent an auto-text tag from being substituted with its display representation.
- `GROW_MODE_UNBOUNDED` (Object, get) — 
- `GROW_MODE_BOUNDED` (Object, get) — 
- `ANCHOR_TYPE_TOP_LEFT` (Object, get) — 
- `ANCHOR_TYPE_CENTER_LEFT` (Object, get) — 
- `ANCHOR_TYPE_BOTTOM_LEFT` (Object, get) — 
- `ANCHOR_TYPE_TOP_RIGHT` (Object, get) — 
- `ANCHOR_TYPE_CENTER_RIGHT` (Object, get) — 
- `ANCHOR_TYPE_BOTTOM_RIGHT` (Object, get) — 
- `ANCHOR_TYPE_TOP_CENTER` (Object, get) — 
- `ANCHOR_TYPE_CENTER_CENTER` (Object, get) — 
- `ANCHOR_TYPE_BOTTOM_CENTER` (Object, get) — 

## Grid (class)

Class that references a Document's grid settings.

[Vendor docs](https://ruby.sketchup.com/Layout/Grid.html)

### Methods
#### `clip_to_margins? => Boolean`

The #clip_to_margins? method returns whether or not the grid is clipped to the page margins.

**Remarks:** The #clip_to_margins? method returns whether or not the grid is clipped to the page margins.

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Layout/Grid.html#clip_to_margins?-instance_method)

#### `in_front? => Boolean`

The #in_front? method returns whether or not the grid is drawn on top of entities.

**Remarks:** The #in_front? method returns whether or not the grid is drawn on top of entities.

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Layout/Grid.html#in_front?-instance_method)

#### `print? => Boolean`

The #print? method returns whether or not the Layout::Grid is printed.

**Remarks:** The #print? method returns whether or not the Layout::Grid is printed.

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Layout/Grid.html#print?-instance_method)

#### `show? => Boolean`

The #show? method returns whether or not the Layout::Grid is visible.

**Remarks:** The #show? method returns whether or not the Layout::Grid is visible.

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Layout/Grid.html#show?-instance_method)

#### `show_major? => Boolean`

The #show_major? method returns whether or not the major grid lines are visible.

**Remarks:** The #show_major? method returns whether or not the major grid lines are visible.

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Layout/Grid.html#show_major?-instance_method)

#### `show_minor? => Boolean`

The #show_minor? method returns whether or not the minor grid lines are visible.

**Remarks:** The #show_minor? method returns whether or not the minor grid lines are visible.

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Layout/Grid.html#show_minor?-instance_method)

### Properties
- `clip_to_margins` (Boolean, set) — The #clip_to_margins= method sets whether or not the grid is clipped to the page margins.
- `in_front` (Boolean, set) — The #in_front= method sets whether or not the grid is drawn on top of entities.
- `major_color` (Sketchup::Color, get/set) — The #major_color method returns the Sketchup::Color for the major grid lines.
- `major_spacing` (Numeric, get/set) — The #major_spacing method returns the major space size of the Layout::Grid.
- `minor_color` (Sketchup::Color, get/set) — The #minor_color method returns the Sketchup::Color for the minor grid lines.
- `minor_divisions` (Integer, get/set) — The #minor_divisions method returns the number of minor divisions of the Layout::Grid.
- `print` (Boolean, set) — The #print= method sets whether or not the Layout::Grid is printed.
- `show` (Boolean, set) — The #show= method sets whether or not the Layout::Grid is visible.
- `show_major` (Boolean, set) — The #show_major= method sets whether or not the major grid lines are visible.
- `show_minor` (Boolean, set) — The #show_minor= method sets whether or not the minor grid lines are visible.

## Group (class)

A group is a special type of Entity that does not belong to a Layer and contains other Entitys as children. A Group's children may include other Groups, allowing for a hierarchical tree structure of Entitys. A Group must contain at least one child and will be automatically collapsed if an operation is performed that results in the Group being empty.

[Vendor docs](https://ruby.sketchup.com/Layout/Group.html)

### Constructors
- `Group(entities)` — The #initialize method creates a new Layout::Group.

### Methods
#### `entities => Layout::Entities`

The #entities method returns the Entities that belong to the Layout::Group.

**Remarks:** The #entities method returns the Entities that belong to the Layout::Group.

**Returns:** `Layout::Entities` — 

[Docs](https://ruby.sketchup.com/Layout/Group.html#entities-instance_method)

#### `remove_scale_factor(resize_behavior) => Object`

The #remove_scale_factor method removes the scale factor from the Layout::Group.

**Remarks:** The #remove_scale_factor method removes the scale factor from the Layout::Group. The resize behavior can be one of the following values: Layout::Group::RESIZE_BEHAVIOR_NONE Layout::Group::RESIZE_BEHAVIOR_BOUNDS Layout::Group::RESIZE_BEHAVIOR_BOUNDS_AND_FONTS

**Parameters:**
- `resize_behavior` (Integer)

[Docs](https://ruby.sketchup.com/Layout/Group.html#remove_scale_factor-instance_method)

#### `scale_factor => Float?`

The #scale_factor method returns the scale factor associated with the Layout::Group.

**Remarks:** The #scale_factor method returns the scale factor associated with the Layout::Group.

**Returns:** `Float, nil` — 

[Docs](https://ruby.sketchup.com/Layout/Group.html#scale_factor-instance_method)

#### `set_scale_factor(scale_factor, units_format, resize_behavior) => Object`

The #set_scale_factor method sets the scale factor for the Layout::Group.

**Remarks:** The #set_scale_factor method sets the scale factor for the Layout::Group. The units format can be any of the following values: Layout::Document::FRACTIONAL_INCHES Layout::Document::DECIMAL_INCHES Layout::Document::DECIMAL_FEET Layout::Document::DECIMAL_MILLIMETERS Layout::Document::DECIMAL_CENTIMETERS Layout::Document::DECIMAL_METERS Layout::Document::DECIMAL_POINTS The resize behavior can be one of the following values: Layout::Group::RESIZE_BEHAVIOR_NONE Layout::Group::RESIZE_BEHAVIOR_BOUNDS Layout::Group::RESIZE_BEHAVIOR_BOUNDS_AND_FONTS

**Parameters:**
- `scale_factor` (Float)
- `units_format` (Integer)
- `resize_behavior` (Integer)

[Docs](https://ruby.sketchup.com/Layout/Group.html#set_scale_factor-instance_method)

#### `ungroup => Boolean`

The #ungroup method removes all Entitys from the Layout::Group and deletes the Layout::Group.

**Remarks:** The #ungroup method removes all Entitys from the Layout::Group and deletes the Layout::Group.

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Layout/Group.html#ungroup-instance_method)

### Properties
- `scale_precision` (Float, get/set) — The #scale_precision method returns the precision used for the scale of the Layout::Group.
- `scale_units` (Integer, get/set) — The #scale_units method returns the units format used in the scale for the Layout::Group.
- `RESIZE_BEHAVIOR_NONE` (Object, get) — 
- `RESIZE_BEHAVIOR_BOUNDS` (Object, get) — 
- `RESIZE_BEHAVIOR_BOUNDS_AND_FONTS` (Object, get) — 

## Image (class)

A raster image entity.

[Vendor docs](https://ruby.sketchup.com/Layout/Image.html)

### Constructors
- `Image(path, bounds)` — The #initialize method creates a new Layout::Image from a given image file.

### Properties
- `clip_mask` (Layout::Entity, get/set) — The #clip_mask method returns the clip mask of the Layout::Image, or nil if it does not have a clip mask.

## Label (class)

This is an interface to a label entity. A Label consists of a FormattedText and the label leader Path. A Label may be connected to another Entity via a ConnectionPoint.

[Vendor docs](https://ruby.sketchup.com/Layout/Label.html)

### Constructors
- `Label(text, leader_type, target_point, bounds)` — The #initialize method creates a new disconnected Layout::Label.

### Methods
#### `connect(connection_point) => Object`

The #connect method connects the Layout::Label to the given ConnectionPoint.

**Remarks:** The #connect method connects the Layout::Label to the given ConnectionPoint. The leader line will be adjusted to point at the ConnectionPoint. The Layout::Label must be in the same Document as the ConnectionPoint. If both the Layout::Label and the ConnectionPoint's Entity are on non-shared Layout::Layers, they must be on the same Page.

**Parameters:**
- `connection_point` (Layout::ConnectionPoint)

[Docs](https://ruby.sketchup.com/Layout/Label.html#connect-instance_method)

#### `disconnect => Object`

The #disconnect method disconnects the Layout::Label from its ConnectionPoint.

**Remarks:** The #disconnect method disconnects the Layout::Label from its ConnectionPoint. The leader line will not be adjusted by disconnecting from a ConnectionPoint.

[Docs](https://ruby.sketchup.com/Layout/Label.html#disconnect-instance_method)

#### `entities => Layout::Entities | entities(page) => Layout::Entities`

The #entities method returns the Entities that represent the Layout::Label in its exploded form.

**Remarks:** The #entities method returns the Entities that represent the Layout::Label in its exploded form.

**Parameters:**
- `page` (Layout::Page) — The Page to use to convert an auto-text tag to display text

**Returns:** `Layout::Entities` — 

[Docs](https://ruby.sketchup.com/Layout/Label.html#entities-instance_method)

### Properties
- `connection_type` (Integer, get/set) — The #connection_type method returns the type of the text connection for the Layout::Label.
- `leader_line` (Layout::Path, get/set) — The #leader_line method returns a copy of the leader line.
- `leader_line_type` (Integer, get/set) — The #leader_line_type method returns the type of the leader line for the Layout::Label.
- `text` (Layout::FormattedText, get/set) — The #text method returns a copy of the FormattedText of the Layout::Label.
- `CONNECTION_TYPE_NONE` (Object, get) — 
- `CONNECTION_TYPE_AUTO` (Object, get) — 
- `CONNECTION_TYPE_REVERSE_AUTO` (Object, get) — 
- `CONNECTION_TYPE_TOP_LEFT` (Object, get) — 
- `CONNECTION_TYPE_CENTER_LEFT` (Object, get) — 
- `CONNECTION_TYPE_BOTTOM_LEFT` (Object, get) — 
- `CONNECTION_TYPE_TOP_RIGHT` (Object, get) — 
- `CONNECTION_TYPE_CENTER_RIGHT` (Object, get) — 
- `CONNECTION_TYPE_BOTTOM_RIGHT` (Object, get) — 
- `LEADER_LINE_TYPE_SINGLE_SEGMENT` (Object, get) — 
- `LEADER_LINE_TYPE_TWO_SEGMENT` (Object, get) — 
- `LEADER_LINE_TYPE_BEZIER` (Object, get) — 
- `LEADER_LINE_TYPE_UNKNOWN` (Object, get) — 

## Layer (class)

This is the interface to a LayOut Layer Definition. A layer definition specifies the document-wide information about a layer. To access the entities on a layer for a given page, use LayerInstance.

[Vendor docs](https://ruby.sketchup.com/Layout/Layer.html)

### Methods
#### `document => Layout::Document`

The #document method returns the Document that the Layout::Layer belongs to.

**Remarks:** The #document method returns the Document that the Layout::Layer belongs to.

**Returns:** `Layout::Document` — 

[Docs](https://ruby.sketchup.com/Layout/Layer.html#document-instance_method)

#### `layer_instance => Layout::LayerInstance | layer_instance(page) => Layout::LayerInstance`

The #layer_instance method returns a Layout::LayerInstance from the Layout::Layer.

**Remarks:** The #layer_instance method returns a Layout::LayerInstance from the Layout::Layer. If the Layout::Layer is shared, a Page does not have to be provided.

**Parameters:**
- `page` (Layout::Page)

**Returns:** `Layout::LayerInstance` — 

[Docs](https://ruby.sketchup.com/Layout/Layer.html#layer_instance-instance_method)

#### `locked? => Boolean`

The #locked? method returns whether the Layout::Layer is locked.

**Remarks:** The #locked? method returns whether the Layout::Layer is locked.

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Layout/Layer.html#locked?-instance_method)

#### `set_nonshared(page, unshare_action) => Object`

The #set_nonshared method sets the Layout::Layer to non-shared.

**Remarks:** The #set_nonshared method sets the Layout::Layer to non-shared. The unshare action can be one of the following values: Layout::Layer::UNSHARELAYERACTION_CLEAR Layout::Layer::UNSHARELAYERACTION_COPYTOONEPAGE Layout::Layer::UNSHARELAYERACTION_COPYTOALLPAGES

**Parameters:**
- `page` (Layout::Page) — The Page with the Layout::Layer to be non-shared.
- `unshare_action` (Integer) — The share action that specifies what to do with Entitys after the Layout::Layer is non-shared.

[Docs](https://ruby.sketchup.com/Layout/Layer.html#set_nonshared-instance_method)

#### `set_shared(page, share_action) => Object`

The #set_shared method sets the Layout::Layer to shared.

**Remarks:** The #set_shared method sets the Layout::Layer to shared. The share action can be one of the following values: Layout::Layer::SHARELAYERACTION_CLEAR Layout::Layer::SHARELAYERACTION_KEEPONEPAGE Layout::Layer::SHARELAYERACTION_MERGEALLPAGES

**Parameters:**
- `page` (Layout::Page) — The Page with the Layout::Layer to be shared.
- `share_action` (Integer) — The share action that specifies what to do with Entitys after the Layout::Layer is shared.

[Docs](https://ruby.sketchup.com/Layout/Layer.html#set_shared-instance_method)

#### `shared? => Boolean`

The #shared? method returns whether the Layout::Layer is shared.

**Remarks:** The #shared? method returns whether the Layout::Layer is shared.

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Layout/Layer.html#shared?-instance_method)

### Properties
- `=` (Layout::Layer, set) — The #== method checks to see if the two Layout::Layers are equal.
- `locked` (Boolean, set) — The #locked= method sets whether the Layout::Layer is locked.
- `name` (String, get/set) — The #name method returns the name of the Layout::Layer.
- `UNSHARELAYERACTION_CLEAR` (Object, get) — 
- `UNSHARELAYERACTION_COPYTOONEPAGE` (Object, get) — 
- `UNSHARELAYERACTION_COPYTOALLPAGES` (Object, get) — 
- `SHARELAYERACTION_CLEAR` (Object, get) — 
- `SHARELAYERACTION_KEEPONEPAGE` (Object, get) — 
- `SHARELAYERACTION_MERGEALLPAGES` (Object, get) — 

## LayerInstance (class)

References an instance of a Layer. A LayerInstance provides access to the Entitys that are on it, as well as their draw order. Groups are not included in the list of Entitys associated with a LayerInstance, since the group hierarchy has no effect on entity draw order. Each Page has one LayerInstance for each Layer in the Document. Non-shared LayerInstances are unique per Page, while shared LayerInstances are shared across all Pages of the Document.

[Vendor docs](https://ruby.sketchup.com/Layout/LayerInstance.html)

### Methods
#### `definition => Layout::Layer`

The #definition method returns the Layout::Layer of the Layout::LayerInstance.

**Remarks:** The #definition method returns the Layout::Layer of the Layout::LayerInstance.

**Returns:** `Layout::Layer` — 

[Docs](https://ruby.sketchup.com/Layout/LayerInstance.html#definition-instance_method)

#### `entities => Layout::Entities`

The #entities method returns the Entities on the Layout::LayerInstance.

**Remarks:** The #entities method returns the Entities on the Layout::LayerInstance.

**Returns:** `Layout::Entities` — 

[Docs](https://ruby.sketchup.com/Layout/LayerInstance.html#entities-instance_method)

#### `entity_index(entity) => Integer`

The #entity_index method returns the index of the Entity on the Layout::LayerInstance.

**Remarks:** The #entity_index method returns the index of the Entity on the Layout::LayerInstance.

**Parameters:**
- `entity` (Layout::Entity) — The entity to get the index of.

**Returns:** `Integer` — 

[Docs](https://ruby.sketchup.com/Layout/LayerInstance.html#entity_index-instance_method)

#### `reorder_entity(entity, index) => Object`

The #reorder_entity method moves the Entity to the specified index.

**Remarks:** The #reorder_entity method moves the Entity to the specified index.

**Parameters:**
- `entity` (Layout::Entity) — The Entity to reorder
- `index` (Integer) — The desired index of the Entity

[Docs](https://ruby.sketchup.com/Layout/LayerInstance.html#reorder_entity-instance_method)

### Properties
- `=` (Layout::LayerInstance, set) — The #== method checks to see if the two Layout::LayerInstances are equal.

## Layers (class)

The Layers class is a container class for all layers in a Document.

[Vendor docs](https://ruby.sketchup.com/Layout/Layers.html)

### Methods
#### `[](index) => Layout::Layer`

The #[] method returns a value from the array of Layout::Layers.

**Remarks:** The #[] method returns a value from the array of Layout::Layers.

**Parameters:**
- `index` (Integer) — The index of the Layout::Layer to return.

**Returns:** `Layout::Layer` — 

[Docs](https://ruby.sketchup.com/Layout/Layers.html#[]-instance_method)

#### `add(shared = false) => Layout::Layer | add(name, shared = false) => Layout::Layer`

The #add method adds a new Layout::Layer to the Document.

**Remarks:** The #add method adds a new Layout::Layer to the Document. The newly added Layout::Layer will be the last one in the Document.

**Parameters:**
- `shared` (Boolean) — true to make the Layout::Layer shared, false for non-shared.

**Returns:** `Layout::Layer` — The newly added Layout::Layer.

[Docs](https://ruby.sketchup.com/Layout/Layers.html#add-instance_method)

#### `each {|layer| ... } => Object`

Note: Don't remove content from this collection while iterating over it with #each.

**Remarks:** Note: Don't remove content from this collection while iterating over it with #each. This would change the size of the collection and cause elements to be skipped as the indices change. Instead copy the current collection to an array using to_a and then use each on the array, when removing content. The #each method iterates through all of the Layout::Layers.

[Docs](https://ruby.sketchup.com/Layout/Layers.html#each-instance_method)

#### `index(layer) => Integer?`

The #index method returns the index of the Layout::Layer, or nil if it doesn't exist in the Document.

**Remarks:** The #index method returns the index of the Layout::Layer, or nil if it doesn't exist in the Document.

**Parameters:**
- `layer` (Layout::Layer)

**Returns:** `Integer, nil` — 

[Docs](https://ruby.sketchup.com/Layout/Layers.html#index-instance_method)

#### `length => Integer Also known as: size`

The #length method returns the number of Layout::Layers.

**Remarks:** The #length method returns the number of Layout::Layers.

**Returns:** `Integer` — 

[Docs](https://ruby.sketchup.com/Layout/Layers.html#length-instance_method)

#### `remove(layer, delete_entities = false) => Object | remove(index, delete_entities = false) => Object`

The #remove method deletes the given Layout::Layer from the Document.

**Remarks:** The #remove method deletes the given Layout::Layer from the Document.

**Parameters:**
- `layer` (Layout::Layer) — The Layout::Layer to be removed
- `delete_entities` (Boolean) — Whether the Entitys on the deleted Layout::Layer should be deleted as well

[Docs](https://ruby.sketchup.com/Layout/Layers.html#remove-instance_method)

#### `reorder(layer, new_index) => Object | reorder(index, new_index) => Object`

The #reorder method moves a Layout::Layer to a different index within the Document's list of layers.

**Remarks:** The #reorder method moves a Layout::Layer to a different index within the Document's list of layers. This will move the Layout::Layer such that its new index becomes new_index.

**Parameters:**
- `layer` (Layout::Layer) — The Layout::Layer to be reordered
- `new_index` (Integer) — The index to put the Layout::Layer at

[Docs](https://ruby.sketchup.com/Layout/Layers.html#reorder-instance_method)

### Properties
- `active` (Layout::Layer, get/set) — The #active method returns the active Layout::Layer in the Document.

## LinearDimension (class)

References a linear dimension entity. A LinearDimension is composed of the following sub-entities: two 'extension lines' that connect to the Entity being dimensioned. a 'dimension line' connecting the ends of the leaders. This may be represented by one or two Paths depending on appearance. an optional 'leader line' that is used for small LinearDimensions. a 'dimension text' that displays the LinearDimension's text. There are six points that may be modified for a LinearDimension: two 'connection points' that define the start and end of the LinearDimension. two 'extent points' that define the start and end of the dimension line and are the ends of the two extension lines. two 'offset points' that define the starting points of the extension lines.

[Vendor docs](https://ruby.sketchup.com/Layout/LinearDimension.html)

### Constructors
- `LinearDimension(start_point, end_point, height)` — The #initialize method creates a new disconnected Layout::LinearDimension.

### Methods
#### `auto_scale? => Boolean`

The #auto_scale? method returns whether the scale for the Layout::LinearDimension is set automatically.

**Remarks:** The #auto_scale? method returns whether the scale for the Layout::LinearDimension is set automatically.

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Layout/LinearDimension.html#auto_scale?-instance_method)

#### `connect(start_connection, end_connection) => Object`

The #connect method connects the Layout::LinearDimension to one or two Entitys using the provided ConnectionPoints.

**Remarks:** The #connect method connects the Layout::LinearDimension to one or two Entitys using the provided ConnectionPoints. The Layout::LinearDimension must be in the same Document as the Entitys, and on the same Page if the Entitys are on non-shared Layout::Layers.

**Parameters:**
- `start_connection` (Layout::ConnectionPoint)
- `end_connection` (Layout::ConnectionPoint)

[Docs](https://ruby.sketchup.com/Layout/LinearDimension.html#connect-instance_method)

#### `custom_text? => Boolean`

The #custom_text? method returns whether the Layout::LinearDimension uses custom text.

**Remarks:** The #custom_text? method returns whether the Layout::LinearDimension uses custom text. When true, the Layout::LinearDimension will display a custom string that doesn't change. When false, it will display the length measurement and will update automatically.

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Layout/LinearDimension.html#custom_text?-instance_method)

#### `disconnect => Object`

The #disconnect method disconnects the Layout::LinearDimension from its ConnectionPoints.

**Remarks:** The #disconnect method disconnects the Layout::LinearDimension from its ConnectionPoints. The dimension will not be adjusted by disconnecting from its ConnectionPoints.

[Docs](https://ruby.sketchup.com/Layout/LinearDimension.html#disconnect-instance_method)

#### `end_offset_point => Geom::Point2d`

The #end_offset_point method returns the paper space location for the end of the first extension line.

**Remarks:** The #end_offset_point method returns the paper space location for the end of the first extension line. The first extension line runs from this offset point to the end extent point.

**Returns:** `Geom::Point2d` — end_offset

[Docs](https://ruby.sketchup.com/Layout/LinearDimension.html#end_offset_point-instance_method)

#### `entities => Layout::Entities`

The #entities method returns the Entities that represent the Layout::LinearDimension in its exploded form.

**Remarks:** The #entities method returns the Entities that represent the Layout::LinearDimension in its exploded form. Depending on the appearance of the Layout::LinearDimension, this may return anywhere from four to six Entitys: start extension line, end extension line, one or two dimension lines, dimension text, and optionally the leader line.

**Returns:** `Layout::Entities` — 

[Docs](https://ruby.sketchup.com/Layout/LinearDimension.html#entities-instance_method)

#### `leader_line_visible? => Boolean`

The #leader_line_visible? method returns whether the leader line is currently visible.

**Remarks:** The #leader_line_visible? method returns whether the leader line is currently visible.

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Layout/LinearDimension.html#leader_line_visible?-instance_method)

#### `start_offset_point => Geom::Point2d`

The #start_offset_point method returns the paper space location for the start of the first extension line.

**Remarks:** The #start_offset_point method returns the paper space location for the start of the first extension line. The first extension line runs from this offset point to the start extent point.

**Returns:** `Geom::Point2d` — start_offset

[Docs](https://ruby.sketchup.com/Layout/LinearDimension.html#start_offset_point-instance_method)

### Properties
- `auto_scale` (Boolean, set) — The #auto_scale= method sets whether the scale for the Layout::LinearDimension is set automatically.
- `custom_text` (Boolean, set) — The #custom_text= method sets whether the Layout::LinearDimension uses custom text.
- `end_connection_point` (Geom::Point2d, get/set) — The #end_connection_point method returns the paper space location for the second connection.
- `end_extent_point` (Geom::Point2d, get/set) — The #end_extent_point method returns the paper space location for the end of the dimension line.
- `end_offset_length` (Numeric, set) — The #end_offset_length= method sets the length of the offset from the second ConnectionPoint to the start of the second extension line.
- `leader_line_type` (Integer, get/set) — The #leader_line_type method returns the type of leader line the Layout::LinearDimension is using.
- `scale` (Numeric, get/set) — The #scale method returns the scale being used for the Layout::LinearDimension.
- `start_connection_point` (Geom::Point2d, get/set) — The #start_connection_point method returns the paper space location for the first connection.
- `start_extent_point` (Geom::Point2d, get/set) — The #start_extent_point method returns the paper space location for the start of the dimension line.
- `start_offset_length` (Numeric, set) — The #start_offset_length= method sets the length of the offset from the first ConnectionPoint to the start of the first extension line.
- `text` (Layout::FormattedText, get/set) — Note: With the addition of auto-text in dimensions for LayOut 2019.
- `LEADER_LINE_TYPE_SINGLE_SEGMENT` (Object, get) — 
- `LEADER_LINE_TYPE_TWO_SEGMENT` (Object, get) — 
- `LEADER_LINE_TYPE_BEZIER` (Object, get) — 
- `LEADER_LINE_TYPE_HIDDEN` (Object, get) — 
- `DIMENSION_LINE_ALIGNED` (Object, get) — 
- `DIMENSION_LINE_VERTICAL` (Object, get) — 
- `DIMENSION_LINE_HORIZONTAL` (Object, get) — 

## LockedEntityError (class)

This is raised whenever a method attempts to modify any Entity that is individually locked.

[Vendor docs](https://ruby.sketchup.com/Layout/LockedEntityError.html)

## LockedLayerError (class)

This is raised whenever a method attempts to modify any Entity that resides on a locked Layer, or when attempting to change the shared attribute of a locked Layer.

[Vendor docs](https://ruby.sketchup.com/Layout/LockedLayerError.html)

## Page (class)

Class for a single page in a LayOut document.

[Vendor docs](https://ruby.sketchup.com/Layout/Page.html)

### Methods
#### `attribute_dictionary(name) => Layout::Dictionary?`

The #attribute_dictionary method returns a copy of the page's attribute dictionary with the given name.

**Remarks:** The #attribute_dictionary method returns a copy of the page's attribute dictionary with the given name.

**Parameters:**
- `name` (String)

**Returns:** `Layout::Dictionary, nil` — A copy of the page's attribute dictionary, or nil if there is no attribute dictionary

[Docs](https://ruby.sketchup.com/Layout/Page.html#attribute_dictionary-instance_method)

#### `delete_attribute(dictionary_name) => Boolean | delete_attribute(dictionary_name, key) => Boolean`

The #delete_attribute method is used to delete an attribute from a page.

**Remarks:** The #delete_attribute method is used to delete an attribute from a page.

**Parameters:**
- `dictionary_name` (String) — The name of an attribute dictionary.

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Layout/Page.html#delete_attribute-instance_method)

#### `document => Layout::Document`

The #document method returns the Document that the Layout::Page belongs to.

**Remarks:** The #document method returns the Document that the Layout::Page belongs to.

**Returns:** `Layout::Document` — 

[Docs](https://ruby.sketchup.com/Layout/Page.html#document-instance_method)

#### `entities => Layout::Entities`

The #entities method returns all Entitys that are on the Layout::Page.

**Remarks:** The #entities method returns all Entitys that are on the Layout::Page. This is the equivalent of iterating over all LayerInstances and using LayerInstance.entities.

**Returns:** `Layout::Entities` — 

[Docs](https://ruby.sketchup.com/Layout/Page.html#entities-instance_method)

#### `get_attribute(name, key, default_value = nil) => String, ...`

The #get_attribute method is used to retrieve the value of an attribute in the page's attribute dictionary.

**Remarks:** The #get_attribute method is used to retrieve the value of an attribute in the page's attribute dictionary. If the third parameter, default_value, is not passed and there is no attribute that matches the given name, it returns nil. If default_value is provided and there is no matching attribute it returns the given value. It does not create an attribute with that name though.

**Parameters:**
- `name` (String) — The name of an attribute dictionary.
- `key` (String) — An attribute key.
- `default_value` (String, Boolean, Integer, Float, Hash, Layout::Dictionary, nil) — A default value to return if no attribute is found.

**Returns:** `String, Boolean, Integer, Float, Layout::Dictionary, nil` — the retrieved value.

[Docs](https://ruby.sketchup.com/Layout/Page.html#get_attribute-instance_method)

#### `in_presentation? => Boolean`

The #in_presentation? method returns whether the Layout::Page is included in presentations.

**Remarks:** The #in_presentation? method returns whether the Layout::Page is included in presentations.

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Layout/Page.html#in_presentation?-instance_method)

#### `layer_instances => Array<Layout::LayerInstance>`

The #layer_instances method returns an array of the LayerInstances for the Layout::Page.

**Remarks:** The #layer_instances method returns an array of the LayerInstances for the Layout::Page.

**Returns:** `Array<Layout::LayerInstance>` — 

[Docs](https://ruby.sketchup.com/Layout/Page.html#layer_instances-instance_method)

#### `layer_visible?(layer) => Boolean`

The #layer_visible? method returns whether a Layer is visible on the Layout::Page.

**Remarks:** The #layer_visible? method returns whether a Layer is visible on the Layout::Page.

**Parameters:**
- `layer` (Layout::Layer)

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Layout/Page.html#layer_visible?-instance_method)

#### `nonshared_entities => Layout::Entities`

The #nonshared_entities method returns the Entities unique to the Layout::Page.

**Remarks:** The #nonshared_entities method returns the Entities unique to the Layout::Page.

**Returns:** `Layout::Entities` — 

[Docs](https://ruby.sketchup.com/Layout/Page.html#nonshared_entities-instance_method)

#### `set_attribute(name, key, value) => Object`

The #set_attribute method adds an attribute to the page's attribute dictionary.

**Remarks:** The #set_attribute method adds an attribute to the page's attribute dictionary.

**Parameters:**
- `name` (String) — The name of an attribute dictionary. @param [String] key An attribute key. @param [String, Boolean, Integer, Float, Hash, Layout::Dictionary, nil] value The value for the attribute.

[Docs](https://ruby.sketchup.com/Layout/Page.html#set_attribute-instance_method)

#### `set_layer_visibility(layer, visible) => Boolean`

The #set_layer_visibility method sets whether a Layer is visible on the Layout::Page.

**Remarks:** The #set_layer_visibility method sets whether a Layer is visible on the Layout::Page.

**Parameters:**
- `layer` (Layout::Layer)
- `visible` (Boolean)

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Layout/Page.html#set_layer_visibility-instance_method)

### Properties
- `=` (Layout::Page, set) — The #== method checks to see if the two Layout::Pages are equal.
- `in_presentation` (Boolean, set) — The #in_presentation= method sets whether the Layout::Page is included in presentations.
- `name` (String, get/set) — The #name method returns the name of the Layout::Page.

## PageInfo (class)

This is the interface to a Document's paper space information. The paper size and margins, display resolution, and colors can all be accessed and set through this class.

[Vendor docs](https://ruby.sketchup.com/Layout/PageInfo.html)

### Methods
#### `print_margins? => Boolean`

The #print_margins? method returns whether to print the paper's margins.

**Remarks:** The #print_margins? method returns whether to print the paper's margins.

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Layout/PageInfo.html#print_margins?-instance_method)

#### `print_paper_color? => Boolean`

The #print_paper_color? method returns whether or not the page color should be printed.

**Remarks:** The #print_paper_color? method returns whether or not the page color should be printed.

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Layout/PageInfo.html#print_paper_color?-instance_method)

#### `show_margins? => Boolean`

The #show_margins? method returns whether the paper's margins are visible.

**Remarks:** The #show_margins? method returns whether the paper's margins are visible.

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Layout/PageInfo.html#show_margins?-instance_method)

### Properties
- `bottom_margin` (Numeric, get/set) — The #bottom_margin method returns the paper's bottom margin in inches.
- `color` (Sketchup::Color, get/set) — The #color method returns the paper's color.
- `display_resolution` (Integer, get/set) — The #display_resolution method returns the on screen rendering resolution quality.
- `height` (Numeric, get/set) — The #height method returns the paper height in inches.
- `image_display_resolution` (Integer, get/set) — The #image_display_resolution method returns the on screen image quality.
- `image_output_resolution` (Integer, get/set) — The #image_output_resolution method returns the output image quality.
- `left_margin` (Numeric, get/set) — The #left_margin method returns the paper's left margin in inches.
- `margin_color` (Sketchup::Color, get/set) — The #margin_color method returns the color of the paper's margin.
- `output_resolution` (Integer, get/set) — The #output_resolution method returns the output rendering resolution quality.
- `print_margins` (Boolean, set) — The #print_margins= method sets whether to print the paper's margins.
- `print_paper_color` (Boolean, set) — The #print_paper_color= method sets whether or not the page color should be printed.
- `right_margin` (Numeric, get/set) — The #right_margin method returns the paper's right margin in inches.
- `show_margins` (Boolean, set) — The #show_margins= method sets whether the paper's margins are visible.
- `top_margin` (Numeric, get/set) — The #top_margin method returns the paper's top margin in inches.
- `width` (Numeric, get/set) — The #width method returns the paper width in inches.
- `RESOLUTION_LOW` (Object, get) — 
- `RESOLUTION_MEDIUM` (Object, get) — 
- `RESOLUTION_HIGH` (Object, get) — 

## Pages (class)

The Pages class is a container class for all pages in a Document.

[Vendor docs](https://ruby.sketchup.com/Layout/Pages.html)

### Methods
#### `[](index) => Layout::Page`

The #[] method returns a value from the array of Layout::Pages.

**Remarks:** The #[] method returns a value from the array of Layout::Pages.

**Parameters:**
- `index` (Integer) — The index of the Layout::Page to return.

**Returns:** `Layout::Page` — 

[Docs](https://ruby.sketchup.com/Layout/Pages.html#[]-instance_method)

#### `add(name = nil) => Layout::Page`

The #add method adds a new Layout::Page to the Document.

**Remarks:** The #add method adds a new Layout::Page to the Document. The newly added Layout::Page will be the last one in the Document.

**Parameters:**
- `name` (String) — The name for the new page.

**Returns:** `Layout::Page` — The newly added Layout::Page.

[Docs](https://ruby.sketchup.com/Layout/Pages.html#add-instance_method)

#### `each {|page| ... } => Object`

Note: Don't remove content from this collection while iterating over it with #each.

**Remarks:** Note: Don't remove content from this collection while iterating over it with #each. This would change the size of the collection and cause elements to be skipped as the indices change. Instead copy the current collection to an array using to_a and then use each on the array, when removing content. The #each method iterates through all of the Layout::Pages.

[Docs](https://ruby.sketchup.com/Layout/Pages.html#each-instance_method)

#### `index(page) => Integer?`

The #index method returns the index of the Layout::Page, or nil if it doesn't exist in the Document.

**Remarks:** The #index method returns the index of the Layout::Page, or nil if it doesn't exist in the Document.

**Parameters:**
- `page` (Layout::Page)

**Returns:** `Integer, nil` — 

[Docs](https://ruby.sketchup.com/Layout/Pages.html#index-instance_method)

#### `length => Integer Also known as: size`

The #length method returns the number of Layout::Pages.

**Remarks:** The #length method returns the number of Layout::Pages.

**Returns:** `Integer` — 

[Docs](https://ruby.sketchup.com/Layout/Pages.html#length-instance_method)

#### `remove(page) => Object | remove(index) => Object`

The #remove method deletes the given Layout::Page from the Document.

**Remarks:** The #remove method deletes the given Layout::Page from the Document.

**Parameters:**
- `page` (Layout::Page) — The Layout::Page to be removed

[Docs](https://ruby.sketchup.com/Layout/Pages.html#remove-instance_method)

#### `reorder(page, new_index) => Object | reorder(index, new_index) => Object`

The #reorder method moves a Layout::Page to a different index within the Document's list of pages.

**Remarks:** The #reorder method moves a Layout::Page to a different index within the Document's list of pages. This will move the Layout::Page such that its new index becomes new_index.

**Parameters:**
- `page` (Layout::Page) — The Layout::Page to be reordered
- `new_index` (Integer) — The index to put the Layout::Page at

[Docs](https://ruby.sketchup.com/Layout/Pages.html#reorder-instance_method)

### Properties
- `initial` (Layout::Page, get/set) — The #initial method returns the initial Layout::Page that will be displayed the next time the Document is opened.

## Path (class)

A path entity represents a continuous, multi-segment polyline or bezier curve.

[Vendor docs](https://ruby.sketchup.com/Layout/Path.html)

### Constructors
- `Path(start_point, end_point)` — The #initialize method creates a new Layout::Path between a start point and an end point, or from a provided Rectangle or Ellipse.

### Methods
#### `append_point(point) => Layout::Path | append_point(control_point1, control_point2, point) => Layout::Path`

The #append_point method appends a Geom::Point2d to the end of the Layout::Path.

**Remarks:** The #append_point method appends a Geom::Point2d to the end of the Layout::Path.

**Parameters:**
- `point` (Geom::Point2d)

**Returns:** `Layout::Path` — 

[Docs](https://ruby.sketchup.com/Layout/Path.html#append_point-instance_method)

#### `arc => Array(Geom::Point2d, Float, Float, Float)?`

The #arc method returns the parameters of an arc from the Layout::Path, or nil if path is not an arc.

**Remarks:** The #arc method returns the parameters of an arc from the Layout::Path, or nil if path is not an arc.

**Returns:** `Array(Geom::Point2d, Float, Float, Float), nil` — The center point, radius, start angle, and end angle

[Docs](https://ruby.sketchup.com/Layout/Path.html#arc-instance_method)

#### `circle => Array(Geom::Point2d, Float)?`

The #circle method returns the parameters of a circle from the Layout::Path, or nil if path is not a circle.

**Remarks:** The #circle method returns the parameters of a circle from the Layout::Path, or nil if path is not a circle.

**Returns:** `Array(Geom::Point2d, Float), nil` — The center point and the radius

[Docs](https://ruby.sketchup.com/Layout/Path.html#circle-instance_method)

#### `close => Object`

The #close method closes the Layout::Path.

**Remarks:** The #close method closes the Layout::Path.

[Docs](https://ruby.sketchup.com/Layout/Path.html#close-instance_method)

#### `closed? => Boolean`

The #closed? method returns whether the Layout::Path is closed.

**Remarks:** The #closed? method returns whether the Layout::Path is closed.

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Layout/Path.html#closed?-instance_method)

#### `end_arrow => Layout::Path?`

The #end_arrow method creates a new Layout::Path from an end arrow.

**Remarks:** The #end_arrow method creates a new Layout::Path from an end arrow.

**Returns:** `Layout::Path, nil` — The end arrow

[Docs](https://ruby.sketchup.com/Layout/Path.html#end_arrow-instance_method)

#### `end_point => Geom::Point2d`

The #end_point method returns the end point of the Layout::Path.

**Remarks:** The #end_point method returns the end point of the Layout::Path.

**Returns:** `Geom::Point2d` — 

[Docs](https://ruby.sketchup.com/Layout/Path.html#end_point-instance_method)

#### `new_arc(center_point, radius, start_angle, end_angle) => Layout::Path`

The new_arc method creates a new arc-shaped Layout::Path.

**Remarks:** The new_arc method creates a new arc-shaped Layout::Path.

**Parameters:**
- `center_point` (Geom::Point2d)
- `radius` (Float)
- `start_angle` (Float) — in radians
- `end_angle` (Float) — in radians

**Returns:** `Layout::Path` — an arc path

[Docs](https://ruby.sketchup.com/Layout/Path.html#new_arc-class_method)

#### `parametric_length => Float`

The #parametric_length method returns the parametric length for the Layout::Path.

**Remarks:** The #parametric_length method returns the parametric length for the Layout::Path. The parametric length is the length with respect to the curve of the Layout::Path.

**Returns:** `Float` — 

[Docs](https://ruby.sketchup.com/Layout/Path.html#parametric_length-instance_method)

#### `point_at(parametric_value) => Geom::Point2d`

The #point_at method returns the Geom::Point2d at a given parametric value.

**Remarks:** The #point_at method returns the Geom::Point2d at a given parametric value.

**Parameters:**
- `parametric_value` (Float)

**Returns:** `Geom::Point2d` — 

[Docs](https://ruby.sketchup.com/Layout/Path.html#point_at-instance_method)

#### `point_types => Array<Integer>`

The #point_types method returns an array of point types corresponding to the Geom::Point2ds in the Layout::Path.

**Remarks:** The #point_types method returns an array of point types corresponding to the Geom::Point2ds in the Layout::Path. A point type can be one of the following values: POINT_TYPE_MOVE_TO POINT_TYPE_LINE_TO POINT_TYPE_BEZIER_TO POINT_TYPE_ARC_CENTER POINT_TYPE_BEZIER_CONTROL POINT_TYPE_CLOSE

**Returns:** `Array<Integer>` — An array of integers that correspond with point types.

[Docs](https://ruby.sketchup.com/Layout/Path.html#point_types-instance_method)

#### `points => Array<Geom::Point2d>`

The #points method returns an array of Geom::Point2ds in the Layout::Path.

**Remarks:** The #points method returns an array of Geom::Point2ds in the Layout::Path.

**Returns:** `Array<Geom::Point2d>` — 

[Docs](https://ruby.sketchup.com/Layout/Path.html#points-instance_method)

#### `start_arrow => Layout::Path?`

The #start_arrow method creates a new Layout::Path from a start arrow.

**Remarks:** The #start_arrow method creates a new Layout::Path from a start arrow.

**Returns:** `Layout::Path, nil` — The start arrow

[Docs](https://ruby.sketchup.com/Layout/Path.html#start_arrow-instance_method)

#### `start_point => Geom::Point2d`

The #start_point method returns the start point of the Layout::Path.

**Remarks:** The #start_point method returns the start point of the Layout::Path.

**Returns:** `Geom::Point2d` — 

[Docs](https://ruby.sketchup.com/Layout/Path.html#start_point-instance_method)

#### `tangent_at(parametric_value) => Geom::Vector2d`

The #tangent_at method returns the tangent Geom::Vector2d at the given parametric value.

**Remarks:** The #tangent_at method returns the tangent Geom::Vector2d at the given parametric value.

**Parameters:**
- `parametric_value` (Float)

**Returns:** `Geom::Vector2d` — 

[Docs](https://ruby.sketchup.com/Layout/Path.html#tangent_at-instance_method)

#### `winding => Integer`

The #winding method returns the winding type of the Layout::Path.

**Remarks:** The #winding method returns the winding type of the Layout::Path. A point type can be one of the following values: PATH_WINDING_NONE PATH_WINDING_CLOCKWISE PATH_WINDING_COUNTER_CLOCKWISE

**Returns:** `Integer` — 

[Docs](https://ruby.sketchup.com/Layout/Path.html#winding-instance_method)

### Properties
- `POINT_TYPE_MOVE_TO` (Object, get) — 
- `POINT_TYPE_LINE_TO` (Object, get) — 
- `POINT_TYPE_BEZIER_TO` (Object, get) — 
- `POINT_TYPE_ARC_CENTER` (Object, get) — 
- `POINT_TYPE_BEZIER_CONTROL` (Object, get) — 
- `POINT_TYPE_CLOSE` (Object, get) — 
- `PATH_WINDING_NONE` (Object, get) — 
- `PATH_WINDING_CLOCKWISE` (Object, get) — 
- `PATH_WINDING_COUNTER_CLOCKWISE` (Object, get) — 

## Rectangle (class)

A simple rectangular shape entity.

[Vendor docs](https://ruby.sketchup.com/Layout/Rectangle.html)

### Constructors
- `Rectangle(bounds)` — The #initialize method creates a new normal, lozenge, bulged or rounded Layout::Rectangle, depending on the type passed in. The rectangle type can be one of the following values: Layout::Rectangle::TYPE_NORMAL Layout::Rectangle::TYPE_ROUNDED Layout::Rectangle::TYPE_LOZENGE Layout::Rectangle::TYPE_BULGED

### Properties
- `radius` (Float, nil, get/set) — The #radius method returns the radius of the Layout::Rectangle, or nil if the Layout::Rectangle is not of type Layout::Rectangle::TYPE_BULGED or Layout::Rectangle::TYPE_ROUNDED
- `type` (Integer, get/set) — The #type method returns the type of the Layout::Rectangle.
- `TYPE_NORMAL` (Object, get) — 
- `TYPE_ROUNDED` (Object, get) — 
- `TYPE_LOZENGE` (Object, get) — 
- `TYPE_BULGED` (Object, get) — 

## ReferenceEntity (class)

An entity that represents the data inserted from an external file.

[Vendor docs](https://ruby.sketchup.com/Layout/ReferenceEntity.html)

### Methods
#### `entities => Layout::Group`

The #entities method returns the Group that represents the Layout::ReferenceEntity in its exploded form.

**Remarks:** The #entities method returns the Group that represents the Layout::ReferenceEntity in its exploded form.

**Returns:** `Layout::Group` — 

[Docs](https://ruby.sketchup.com/Layout/ReferenceEntity.html#entities-instance_method)

### Properties
- `clip_mask` (Layout::Entity, get/set) — The #clip_mask method returns the clip mask of the Layout::ReferenceEntity, or nil if it does not have a clip mask.

## SketchUpModel (class)

A SketchUp Model entity. This is an instance of a SketchUp Model that is inserted into a .layout file. You can change the render mode, line weight, and set the current scene for the SketchUp Model with this interface.

[Vendor docs](https://ruby.sketchup.com/Layout/SketchUpModel.html)

### Constructors
- `SketchUpModel(path, bounds)` — The #initialize method creates a new Layout::SketchUpModel.

### Methods
#### `camera_modified? => Boolean`

The #camera_modified? method returns whether the camera of the Layout::SketchUpModel has been modified.

**Remarks:** The #camera_modified? method returns whether the camera of the Layout::SketchUpModel has been modified.

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Layout/SketchUpModel.html#camera_modified?-instance_method)

#### `current_scene_modified? => Boolean`

The #current_scene_modified? method returns whether the most recently selected scene of the Layout::SketchUpModel has been modified.

**Remarks:** The #current_scene_modified? method returns whether the most recently selected scene of the Layout::SketchUpModel has been modified.

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Layout/SketchUpModel.html#current_scene_modified?-instance_method)

#### `display_background? => Boolean`

The #display_background? method returns whether the background is displayed for the Layout::SketchUpModel.

**Remarks:** The #display_background? method returns whether the background is displayed for the Layout::SketchUpModel.

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Layout/SketchUpModel.html#display_background?-instance_method)

#### `effects_modified? => Boolean`

The #effects_modified? method returns whether the shadow or fog settings of the Layout::SketchUpModel have been modified.

**Remarks:** The #effects_modified? method returns whether the shadow or fog settings of the Layout::SketchUpModel have been modified.

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Layout/SketchUpModel.html#effects_modified?-instance_method)

#### `entities => Layout::Entity`

The #entities method returns the Group that represents the Layout::SketchUpModel in its exploded form.

**Remarks:** The #entities method returns the Group that represents the Layout::SketchUpModel in its exploded form. The Group will contain a Image for raster and hybrid-rendered models, and will contain a Group of LayOut entities for vector and hybrid-rendered models.

**Returns:** `Layout::Entity` — 

[Docs](https://ruby.sketchup.com/Layout/SketchUpModel.html#entities-instance_method)

#### `layers_modified? => Boolean`

Note: In SketchUp 2020, SketchUp “layers” were renamed to “tags”.

**Remarks:** Note: In SketchUp 2020, SketchUp “layers” were renamed to “tags”. For consistency with the SketchUp API, this will continue to refer to “tags” as “layers”. The #layers_modified? method returns whether the layers of the Layout::SketchUpModel has been modified.

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Layout/SketchUpModel.html#layers_modified?-instance_method)

#### `model_to_paper_point(model_point) => Geom::Point2d`

The #model_to_paper_point method converts the Geom::Point3d in the Layout::SketchUpModel to a Geom::Point2d in paper space.

**Remarks:** The #model_to_paper_point method converts the Geom::Point3d in the Layout::SketchUpModel to a Geom::Point2d in paper space.

**Parameters:**
- `model_point` (Geom::Point3d)

**Returns:** `Geom::Point2d` — 

[Docs](https://ruby.sketchup.com/Layout/SketchUpModel.html#model_to_paper_point-instance_method)

#### `output_entities => Layout::Entity`

The #output_entities method returns the Group that represents the Layout::SketchUpModel in its exported form.

**Remarks:** The #output_entities method returns the Group that represents the Layout::SketchUpModel in its exported form. The Group will contain a Image for raster and hybrid-rendered models, and will contain a Group of LayOut entities for vector and hybrid-rendered models. This takes into account the output resolution set in the document's PageInfo, and the render mode override set on the document.

**Returns:** `Layout::Entity` — 

[Docs](https://ruby.sketchup.com/Layout/SketchUpModel.html#output_entities-instance_method)

#### `perspective? => Boolean`

The #perspective? method returns whether the Layout::SketchUpModel's view is perspective or orthographic.

**Remarks:** The #perspective? method returns whether the Layout::SketchUpModel's view is perspective or orthographic.

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Layout/SketchUpModel.html#perspective?-instance_method)

#### `preserve_scale_on_resize? => Boolean`

The #preserve_scale_on_resize? method returns whether the scale is preserved when the Layout::SketchUpModel is resized.

**Remarks:** The #preserve_scale_on_resize? method returns whether the scale is preserved when the Layout::SketchUpModel is resized.

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Layout/SketchUpModel.html#preserve_scale_on_resize?-instance_method)

#### `render => Object`

The #render method renders the Layout::SketchUpModel.

**Remarks:** The #render method renders the Layout::SketchUpModel. If the model belongs to a Document, then the render will be performed at the quality set in document.page_info (see Document and PageInfo). Otherwise, the render will be performed at Low quality.

[Docs](https://ruby.sketchup.com/Layout/SketchUpModel.html#render-instance_method)

#### `render_needed? => Boolean`

The #render_needed? method returns whether the Layout::SketchUpModel needs to be rendered.

**Remarks:** The #render_needed? method returns whether the Layout::SketchUpModel needs to be rendered.

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Layout/SketchUpModel.html#render_needed?-instance_method)

#### `reset_camera => Object`

The #reset_camera method resets the Layout::SketchUpModel's camera to the scene's setting.

**Remarks:** The #reset_camera method resets the Layout::SketchUpModel's camera to the scene's setting.

[Docs](https://ruby.sketchup.com/Layout/SketchUpModel.html#reset_camera-instance_method)

#### `reset_effects => Object`

The #reset_effects method resets the Layout::SketchUpModel's shadow and fog settings to the scene's settings.

**Remarks:** The #reset_effects method resets the Layout::SketchUpModel's shadow and fog settings to the scene's settings.

[Docs](https://ruby.sketchup.com/Layout/SketchUpModel.html#reset_effects-instance_method)

#### `reset_layers => Object`

Note: In SketchUp 2020, SketchUp “layers” were renamed to “tags”.

**Remarks:** Note: In SketchUp 2020, SketchUp “layers” were renamed to “tags”. For consistency with the SketchUp API, this will continue to refer to “tags” as “layers”. The #reset_layers method resets the Layout::SketchUpModel's layers to the scene's setting.

[Docs](https://ruby.sketchup.com/Layout/SketchUpModel.html#reset_layers-instance_method)

#### `reset_style => Object`

The #reset_style method resets the Layout::SketchUpModel's style to the scene's setting.

**Remarks:** The #reset_style method resets the Layout::SketchUpModel's style to the scene's setting.

[Docs](https://ruby.sketchup.com/Layout/SketchUpModel.html#reset_style-instance_method)

#### `scenes => Array<String>`

The #scenes method returns an array of scene names that are available for the Layout::SketchUpModel.

**Remarks:** The #scenes method returns an array of scene names that are available for the Layout::SketchUpModel. The first scene will always be the default scene, called “Last saved SketchUp View”.

**Returns:** `Array<String>` — 

[Docs](https://ruby.sketchup.com/Layout/SketchUpModel.html#scenes-instance_method)

#### `style_modified? => Boolean`

The #style_modified? method returns whether the style of the Layout::SketchUpModel has been modified.

**Remarks:** The #style_modified? method returns whether the style of the Layout::SketchUpModel has been modified.

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Layout/SketchUpModel.html#style_modified?-instance_method)

### Properties
- `clip_mask` (Layout::Entity, get/set) — The #clip_mask method returns the clip mask entity for the Layout::SketchUpModel, or nil if it does not have one.
- `current_scene` (Integer, get/set) — The #current_scene method returns the index of the most recently selected scene of the Layout::SketchUpModel.
- `dash_scale` (Float, get/set) — The #dash_scale method returns the dash scale for the Layout::SketchUpModel.
- `display_background` (Boolean, set) — The #display_background= method sets whether the background is displayed for the Layout::SketchUpModel.
- `line_weight` (Float, get/set) — The #line_weight method returns the line weight for the Layout::SketchUpModel.
- `perspective` (Boolean, set) — The #perspective= method sets whether the Layout::SketchUpModel's view is perspective or orthographic.
- `preserve_scale_on_resize` (Boolean, set) — The #preserve_scale_on_resize= method sets whether the scale is preserved when the Layout::SketchUpModel is resized.
- `render_mode` (Integer, get/set) — The #render_mode method returns the render mode of the Layout::SketchUpModel.
- `scale` (Float, get/set) — The #scale method returns the scale of the Layout::SketchUpModel.
- `view` (Integer, get/set) — The #view method returns the standard view of the Layout::SketchUpModel.
- `NO_OVERRIDE` (Object, get) — 
- `RASTER_RENDER` (Object, get) — 
- `HYBRID_RENDER` (Object, get) — 
- `VECTOR_RENDER` (Object, get) — 
- `CUSTOM_VIEW` (Object, get) — 
- `TOP_VIEW` (Object, get) — 
- `TOP_RELATIVE_VIEW` (Object, get) — 
- `BOTTOM_VIEW` (Object, get) — 
- `BOTTOM_RELATIVE_VIEW` (Object, get) — 
- `FRONT_VIEW` (Object, get) — 
- `BACK_VIEW` (Object, get) — 
- `LEFT_VIEW` (Object, get) — 
- `RIGHT_VIEW` (Object, get) — 
- `ISO_VIEW` (Object, get) — 

## Style (class)

References a collection of style attributes that determine the visual appearance of Entitys. Style attributes are those attributes which the user can manipulate in LayOut's inspector windows. For example, shape style attributes that define stroke and fill, or text style attributes that define the font for FormattedText. The Document maintains a default style for various types of Entitys, and it is possible to apply the style of one entity to another. Style objects are transient and do not belong to a Document.

[Vendor docs](https://ruby.sketchup.com/Layout/Style.html)

### Constructors
- `Style()` — The #initialize method creates a new Layout::Style.

### Methods
#### `arrow_type_filled?(arrow_type) => Boolean`

The arrow_type_filled? method returns whether the specified arrow type is filled or not.

**Remarks:** The arrow_type_filled? method returns whether the specified arrow type is filled or not. The arrow type can be one of the following values: Layout::Style::ARROW_NONE Layout::Style::ARROW_FILLED_TRIANGLE Layout::Style::ARROW_OPEN_TRIANGLE Layout::Style::ARROW_FILLED_SKINNY_TRIANGLE Layout::Style::ARROW_OPEN_SKINNY_TRIANGLE Layout::Style::ARROW_OPEN_ARROW_90 Layout::Style::ARROW_OPEN_ARROW_120 Layout::Style::ARROW_FILLED_CIRCLE Layout::Style::ARROW_OPEN_CIRCLE Layout::Style::ARROW_FILLED_SQUARE Layout::Style::ARROW_OPEN_SQUARE Layout::Style::ARROW_FILLED_DIAMOND Layout::Style::ARROW_OPEN_DIAMOND Layout::Style::ARROW_STAR Layout::Style::ARROW_T Layout::Style::ARROW_SLASH_RIGHT Layout::Style::ARROW_SLASH_LEFT Layout::Style::ARROW_UNDERRUN Layout::Style::ARROW_OVERRUN

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Layout/Style.html#arrow_type_filled?-class_method)

#### `dimension_units => Array(Integer, Float)?`

The #dimension_units method returns the unit format and precision for dimensions, or nil if the Layout::Style does not have a value for that setting.

**Remarks:** The #dimension_units method returns the unit format and precision for dimensions, or nil if the Layout::Style does not have a value for that setting. Units may be for either LinearDimensions or AngularDimensions, but not both. The units can be one of the following values: Layout::Style::FRACTIONAL_INCHES Layout::Style::ARCHITECTURAL_INCHES Layout::Style::ENGINEERING_FEET Layout::Style::DECIMAL_INCHES Layout::Style::DECIMAL_FEET Layout::Style::DECIMAL_MILLIMETERS Layout::Style::DECIMAL_CENTIMETERS Layout::Style::DECIMAL_METERS Layout::Style::DECIMAL_POINTS Layout::Style::DEGREES Layout::Style::RADIANS

**Returns:** `Array(Integer, Float), nil` — 

[Docs](https://ruby.sketchup.com/Layout/Style.html#dimension_units-instance_method)

#### `get_sub_style(type) => Layout::Style`

The #get_sub_style method returns the Layout::Style for a sub-entity from the Layout::Style.

**Remarks:** The #get_sub_style method returns the Layout::Style for a sub-entity from the Layout::Style. This would be used to get the current style of a LinearDimension's text, for example.

**Parameters:**
- `type` (Integer)

**Returns:** `Layout::Style` — 

[Docs](https://ruby.sketchup.com/Layout/Style.html#get_sub_style-instance_method)

#### `set_dimension_units(units, precision) => Object`

The #set_dimension_units method sets the unit format and precision for dimensions.

**Remarks:** The #set_dimension_units method sets the unit format and precision for dimensions. Units may be for either LinearDimensions or AngularDimensions, but not both. The units can be one of the following values: Layout::Style::FRACTIONAL_INCHES Layout::Style::ARCHITECTURAL_INCHES Layout::Style::ENGINEERING_FEET Layout::Style::DECIMAL_INCHES Layout::Style::DECIMAL_FEET Layout::Style::DECIMAL_MILLIMETERS Layout::Style::DECIMAL_CENTIMETERS Layout::Style::DECIMAL_METERS Layout::Style::DECIMAL_POINTS Layout::Style::DEGREES Layout::Style::RADIANS

**Parameters:**
- `units` (Integer)
- `precision` (Float)

[Docs](https://ruby.sketchup.com/Layout/Style.html#set_dimension_units-instance_method)

#### `set_sub_style(type, sub_style) => Object`

The #set_sub_style method adds a Layout::Style to apply to a Entity's sub-entity.

**Remarks:** The #set_sub_style method adds a Layout::Style to apply to a Entity's sub-entity. This would be used to set the arrow type for extension lines of a LinearDimension, for example.

**Parameters:**
- `type` (Integer)
- `sub_style` (Layout::Style)

[Docs](https://ruby.sketchup.com/Layout/Style.html#set_sub_style-instance_method)

### Properties
- `dimension_rotation_alignment` (Integer, nil, get/set) — The #dimension_rotation_alignment method returns the rotational text alignment for LinearDimension text, or nil if the Layout::Style does not have a value for that setting.
- `dimension_vertical_alignment` (Integer, nil, get/set) — The #dimension_vertical_alignment method returns the vertical text alignment for LinearDimension text, or nil if the Layout::Style does not have a value for that setting.
- `end_arrow_size` (Float, nil, get/set) — The #end_arrow_size method returns the size of the end arrow, or nil if the Layout::Style does not have a value for that setting.
- `end_arrow_type` (Integer, nil, get/set) — The #end_arrow_type method returns the type of end arrow, or nil if the Layout::Style does not have a value for that setting.
- `fill_color` (Sketchup::Color, nil, get/set) — The #fill_color method returns the solid file color, or nil if the Layout::Style does not have a value for that setting.
- `font_family` (String, nil, get/set) — The #font_family method returns the text font name, or nil if the Layout::Style does not have a value for that setting.
- `font_size` (Float, nil, get/set) — The #font_size method returns the font size, or nil if the Layout::Style does not have a value for that setting.
- `pattern_fill_origin` (Geom::Point2d, nil, get/set) — The #pattern_fill_origin method returns the starting piont for the pattern fill, or nil if the Layout::Style does not have a value for that setting.
- `pattern_fill_path` (String, nil, get/set) — The #pattern_fill_path method returns the file path to the pattern fill image, or nil if the Layout::Style does not have a value for that setting.
- `pattern_fill_rotation` (Float, nil, get/set) — The #pattern_fill_rotation method returns the rotation of the pattern fill image in degrees, or nil if the Layout::Style does not have a value for that setting.
- `pattern_fill_scale` (Float, nil, get/set) — The #pattern_fill_scale method returns the pattern fill scale, or nil if the Layout::Style does not have a value for that setting.
- `pattern_filled` (Boolean, nil, get/set) — The #pattern_filled method returns whether the Layout::Style has a pattern fill, or nil if the Layout::Style does not have a value for that setting.
- `solid_filled` (Boolean, nil, get/set) — The #solid_filled method returns whether the Layout::Style has a solid fill, or nil if the Layout::Style does not have a value for that setting.
- `start_arrow_size` (Float, nil, get/set) — The #start_arrow_size method returns the size of the start arrow, or nil if the Layout::Style does not have a value for that setting.
- `start_arrow_type` (Integer, nil, get/set) — The #start_arrow_type method returns the type of start arrow, or nil if the Layout::Style does not have a value for that setting.
- `stroke_cap_style` (Integer, nil, get/set) — The #stroke_cap_style method returns the stroke cap style, or nil if the Layout::Style does not have a value for that setting.
- `stroke_color` (Sketchup::Color, nil, get/set) — The #stroke_color method returns the stroke color, or nil if the Layout::Style does not have a value for that setting.
- `stroke_join_style` (Integer, nil, get/set) — The #stroke_join_style method returns the stroke join style, or nil if the Layout::Style does not have a value for that setting.
- `stroke_pattern` (Integer, nil, get/set) — The #stroke_pattern method returns the stroke pattern, or nil if the Layout::Style does not have a value for that setting.
- `stroke_pattern_scale` (Float, nil, get/set) — The #stroke_pattern_scale method returns the stroke pattern scale, or nil if the Layout::Style does not have a value for that setting.
- `stroke_width` (Float, nil, get/set) — The #stroke_width method returns the stroke width, or nil if the Layout::Style does not have a value for that setting.
- `stroked` (Boolean, nil, get/set) — The #stroked method returns whether the Layout::Style has a stroke, or nil if the Layout::Style does not have a value for that setting.
- `suppress_dimension_units` (Boolean, nil, get/set) — The #suppress_dimension_units method returns whether the units for dimensions are suppressed, or nil if the Layout::Style does not have a value for that setting.
- `text_alignment` (Integer, nil, get/set) — The #text_alignment method returns the text alignment, or nil if the Layout::Style does not have a value for that setting.
- `text_anchor` (Integer, nil, get/set) — The #text_anchor method returns the text anchor type, or nil if the Layout::Style does not have a value for that setting.
- `text_bold` (Boolean, nil, get/set) — The #text_bold method returns whether text is bold, or nil if the Layout::Style does not have a value for that setting.
- `text_color` (Sketchup::Color, nil, get/set) — The #text_color method returns the text color, or nil if the Layout::Style does not have a value for that setting.
- `text_elevation` (Integer, nil, get/set) — The #text_elevation method returns the text elevation, or nil if the Layout::Style does not have a value for that setting.
- `text_italic` (Boolean, nil, get/set) — The #text_italic method returns whether text is italic, or nil if the Layout::Style does not have a value for that setting.
- `text_strikethrough` (Integer, nil, get/set) — The #text_strikethrough method returns the text strike through type, or nil if the Layout::Style does not have a value for that setting.
- `text_underline` (Integer, nil, get/set) — The #text_underline method returns the text underline type, or nil if the Layout::Style does not have a value for that setting.
- `JOIN_STYLE_MITER` (Object, get) — 
- `JOIN_STYLE_ROUND` (Object, get) — 
- `JOIN_STYLE_BEVEL` (Object, get) — 
- `CAP_STYLE_FLAT` (Object, get) — 
- `CAP_STYLE_ROUND` (Object, get) — 
- `CAP_STYLE_SQUARE` (Object, get) — 
- `STROKE_PATTERN_SOLID` (Object, get) — 
- `STROKE_PATTERN_DASH` (Object, get) — 
- `STROKE_PATTERN_DOT` (Object, get) — 
- `STROKE_PATTERN_DASH_DOT` (Object, get) — 
- `STROKE_PATTERN_DASH_DOT_DOT` (Object, get) — 
- `STROKE_PATTERN_DASH_SPACE` (Object, get) — 
- `STROKE_PATTERN_DASH_DOT_DOT_DOT` (Object, get) — 
- `STROKE_PATTERN_DASH_DASH_DOT` (Object, get) — 
- `STROKE_PATTERN_DASH_DASH_DOT_DOT` (Object, get) — 
- `STROKE_PATTERN_DASH_DASH_DOT_DOT_DOT` (Object, get) — 
- `STROKE_PATTERN_CENTER` (Object, get) — 
- `STROKE_PATTERN_PHANTOM` (Object, get) — 
- `STROKE_PATTERN_SHORT_DASH` (Object, get) — 
- `UNDERLINE_NONE` (Object, get) — 
- `UNDERLINE_SINGLE` (Object, get) — 
- `UNDERLINE_DOUBLE` (Object, get) — 
- `STRIKETHROUGH_NONE` (Object, get) — 
- `STRIKETHROUGH_SINGLE` (Object, get) — 
- `NORMAL_SCRIPT` (Object, get) — 
- `SUPER_SCRIPT` (Object, get) — 
- `SUB_SCRIPT` (Object, get) — 
- `ALIGN_LEFT` (Object, get) — 
- `ALIGN_RIGHT` (Object, get) — 
- `ALIGN_CENTER` (Object, get) — 
- `ANCHOR_TOP` (Object, get) — 
- `ANCHOR_CENTER` (Object, get) — 
- `ANCHOR_BOTTOM` (Object, get) — 
- `DIMENSION_TEXT_ABOVE` (Object, get) — 
- `DIMENSION_TEXT_CENTER` (Object, get) — 
- `DIMENSION_TEXT_BELOW` (Object, get) — 
- `DIMENSION_TEXT_OFFSET` (Object, get) — 
- `DIMENSION_TEXT_HORIZONTAL` (Object, get) — 
- `DIMENSION_TEXT_VERTICAL` (Object, get) — 
- `DIMENSION_TEXT_PARALLEL` (Object, get) — 
- `DIMENSION_TEXT_PERPENDICULAR` (Object, get) — 
- `FRACTIONAL_INCHES` (Object, get) — 
- `ARCHITECTURAL_INCHES` (Object, get) — 
- `ENGINEERING_FEET` (Object, get) — 
- `DECIMAL_INCHES` (Object, get) — 
- `DECIMAL_FEET` (Object, get) — 
- `DECIMAL_MILLIMETERS` (Object, get) — 
- `DECIMAL_CENTIMETERS` (Object, get) — 
- `DECIMAL_METERS` (Object, get) — 
- `DECIMAL_POINTS` (Object, get) — 
- `DEGREES` (Object, get) — 
- `RADIANS` (Object, get) — 
- `ARROW_NONE` (Object, get) — 
- `ARROW_FILLED_TRIANGLE` (Object, get) — 
- `ARROW_OPEN_TRIANGLE` (Object, get) — 
- `ARROW_FILLED_SKINNY_TRIANGLE` (Object, get) — 
- `ARROW_OPEN_SKINNY_TRIANGLE` (Object, get) — 
- `ARROW_OPEN_ARROW_90` (Object, get) — 
- `ARROW_OPEN_ARROW_120` (Object, get) — 
- `ARROW_FILLED_CIRCLE` (Object, get) — 
- `ARROW_OPEN_CIRCLE` (Object, get) — 
- `ARROW_FILLED_SQUARE` (Object, get) — 
- `ARROW_OPEN_SQUARE` (Object, get) — 
- `ARROW_FILLED_DIAMOND` (Object, get) — 
- `ARROW_OPEN_DIAMOND` (Object, get) — 
- `ARROW_STAR` (Object, get) — 
- `ARROW_T` (Object, get) — 
- `ARROW_SLASH_RIGHT` (Object, get) — 
- `ARROW_SLASH_LEFT` (Object, get) — 
- `ARROW_UNDERRUN` (Object, get) — 
- `ARROW_OVERRUN` (Object, get) — 
- `LABEL_LEADER_LINE` (Object, get) — 
- `LABEL_TEXT` (Object, get) — 
- `DIMENSION_START_EXTENSION_LINE` (Object, get) — 
- `DIMENSION_END_EXTENSION_LINE` (Object, get) — 
- `DIMENSION_LINE` (Object, get) — 
- `DIMENSION_LEADER_LINE` (Object, get) — 
- `DIMENSION_TEXT` (Object, get) — 

## Table (class)

A Table is a series of rows and columns that holds data.

[Vendor docs](https://ruby.sketchup.com/Layout/Table.html)

### Constructors
- `Table(bounds, rows, columns)` — The #initialize method creates a Layout::Table with a specified size, and a specified number of rows and columns.

### Methods
#### `[](row_index, column_index) => Layout::TableCell`

The #[] method returns the Layout::TableCell at the specified row and column.

**Remarks:** The #[] method returns the Layout::TableCell at the specified row and column.

**Parameters:**
- `row_index` (Integer)
- `column_index` (Integer)

**Returns:** `Layout::TableCell` — 

[Docs](https://ruby.sketchup.com/Layout/Table.html#[]-instance_method)

#### `dimensions => Array(Integer, Integer)`

The #dimensions method returns the number of rows and columns in a Layout::Table.

**Remarks:** The #dimensions method returns the number of rows and columns in a Layout::Table.

**Returns:** `Array(Integer, Integer)` — The first value is the number of rows; the second, the number of columns.

[Docs](https://ruby.sketchup.com/Layout/Table.html#dimensions-instance_method)

#### `each {|cell| ... } => Object`

The #each method iterates in column major order through all of the cells in the Layout::Table.

**Remarks:** The #each method iterates in column major order through all of the cells in the Layout::Table.

[Docs](https://ruby.sketchup.com/Layout/Table.html#each-instance_method)

#### `entities => Layout::Entities`

The #entities method creates and returns the Entities that represent the Layout::Table in its exploded form.

**Remarks:** The #entities method creates and returns the Entities that represent the Layout::Table in its exploded form.

**Returns:** `Layout::Entities` — 

[Docs](https://ruby.sketchup.com/Layout/Table.html#entities-instance_method)

#### `get_column(index) => Layout::TableColumn`

The #get_column method returns the Layout::TableColumn at the specified index.

**Remarks:** The #get_column method returns the Layout::TableColumn at the specified index.

**Parameters:**
- `index` (Integer)

**Returns:** `Layout::TableColumn` — 

[Docs](https://ruby.sketchup.com/Layout/Table.html#get_column-instance_method)

#### `get_row(index) => Layout::TableRow`

The #get_row method returns the Layout::TableRow at the specified index.

**Remarks:** The #get_row method returns the Layout::TableRow at the specified index.

**Parameters:**
- `index` (Integer)

**Returns:** `Layout::TableRow` — 

[Docs](https://ruby.sketchup.com/Layout/Table.html#get_row-instance_method)

#### `insert_column(index) => Object`

The #insert_column method inserts a new column at the specified index.

**Remarks:** The #insert_column method inserts a new column at the specified index.

**Parameters:**
- `index` (Integer)

[Docs](https://ruby.sketchup.com/Layout/Table.html#insert_column-instance_method)

#### `insert_row(index) => Object`

The #insert_row method inserts a new row at the specified index.

**Remarks:** The #insert_row method inserts a new row at the specified index.

**Parameters:**
- `index` (Integer)

[Docs](https://ruby.sketchup.com/Layout/Table.html#insert_row-instance_method)

#### `merge(start_row, start_column, end_row, end_column) => Object`

The #merge method merges a range of cells within a Layout::Table.

**Remarks:** The #merge method merges a range of cells within a Layout::Table. Only cells which are not already merged can be merged.

**Parameters:**
- `start_row` (Integer)
- `start_column` (Integer)
- `end_row` (Integer)
- `end_column` (Integer)

[Docs](https://ruby.sketchup.com/Layout/Table.html#merge-instance_method)

#### `remove_column(index) => Object`

The #remove_column method removes the column at the specified index.

**Remarks:** The #remove_column method removes the column at the specified index.

**Parameters:**
- `index` (Integer)

[Docs](https://ruby.sketchup.com/Layout/Table.html#remove_column-instance_method)

#### `remove_row(index) => Object`

The #remove_row method removes the row at the specified index.

**Remarks:** The #remove_row method removes the row at the specified index.

**Parameters:**
- `index` (Integer)

[Docs](https://ruby.sketchup.com/Layout/Table.html#remove_row-instance_method)

## TableCell (class)

A TableCell is a single cell from a table that contains data.

[Vendor docs](https://ruby.sketchup.com/Layout/TableCell.html)

### Methods
#### `span => Array(Integer, Integer)`

The #span method returns the row and column span of a Layout::TableCell.

**Remarks:** The #span method returns the row and column span of a Layout::TableCell. If the values returned are both 1, then it is a normal, non-merged cell. If either of the values are greater than 1, then it is a merged cell. If the values are both 0, then it is an unused cell that resides within the inner portion of another merged cell.

**Returns:** `Array(Integer, Integer)` — Row span and column span.

[Docs](https://ruby.sketchup.com/Layout/TableCell.html#span-instance_method)

### Properties
- `data` (Layout::Entity, get/set) — The #data method creates a copy of the FormattedText for the Layout::TableCell.
- `rotation` (Integer, get/set) — The #rotation method returns the rotation of a Layout::TableCell.
- `ROTATION_0` (Object, get) — 
- `ROTATION_90` (Object, get) — 
- `ROTATION_180` (Object, get) — 
- `ROTATION_270` (Object, get) — 

## TableColumn (class)

A TableColumn is a single column from a table.

[Vendor docs](https://ruby.sketchup.com/Layout/TableColumn.html)

### Properties
- `left_edge_style` (Layout::Style, get/set) — The #left_edge_style method returns the Style of a Layout::TableColumn's left edge.
- `right_edge_style` (Layout::Style, get/set) — The #right_edge_style method returns the Style of a Layout::TableColumn's right edge.
- `width` (Float, get/set) — The #width method returns the width of the Layout::TableColumn.

## TableRow (class)

A TableColumn is a single row from a table.

[Vendor docs](https://ruby.sketchup.com/Layout/TableRow.html)

### Properties
- `bottom_edge_style` (Layout::Style, get/set) — The #bottom_edge_style method returns the Style of a Layout::TableRow's bottom edge.
- `height` (Float, get/set) — The #height method returns the height of the Layout::TableRow.
- `top_edge_style` (Layout::Style, get/set) — The #top_edge_style method returns the Style of a Layout::TableRow's top edge.

