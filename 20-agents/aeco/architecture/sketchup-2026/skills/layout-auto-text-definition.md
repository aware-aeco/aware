---
name: yard-layout-auto-text-definition
description: Layout::AutoTextDefinition API reference (YARD)
---

# Layout::AutoTextDefinition API reference

References an auto-text definition. Some auto-text definitions are mandatory. Mandatory auto-text definitions may not be removed, added, or modified.

## Methods

- `==` — The #== method checks to see if the two Layout::AutoTextDefinitions are equal. This checks whether the Ruby Objects are pointing to the same internal object.
- `custom_text` — The #custom_text method returns the custom text of the Layout::AutoTextDefinition::TYPE_CUSTOM_TEXT Layout::AutoTextDefinition.
- `custom_text=` — The #custom_text method sets the custom text of the Layout::AutoTextDefinition::TYPE_CUSTOM_TEXT Layout::AutoTextDefinition.
- `date_format` — The #date_format method returns the date format of a Layout::AutoTextDefinition::TYPE_DATE_* Layout::AutoTextDefinition.
- `date_format=` — The #date_format method sets the date format of a Layout::AutoTextDefinition::TYPE_DATE_* Layout::AutoTextDefinition.
- `display_file_extension=` — The #display_file_extension= method sets whether the Layout::AutoTextDefinition::TYPE_FILE Layout::AutoTextDefinition displays the file extension.
- `display_file_extension?` — The #display_file_extension? method returns whether the Layout::AutoTextDefinition::TYPE_FILE Layout::AutoTextDefinition displays the file extension.
- `display_full_path=` — The #display_full_path= method sets whether the Layout::AutoTextDefinition::TYPE_FILE Layout::AutoTextDefinition displays the full path.
- `display_full_path?` — The #display_full_path? method returns whether the Layout::AutoTextDefinition::TYPE_FILE Layout::AutoTextDefinition displays the full path.
- `end_page` — The #end_page method returns the end page for the Layout::AutoTextDefinition::TYPE_PAGE_COUNT Layout::AutoTextDefinition.
- `end_page=` — The #end_page= method sets the end page for the Layout::AutoTextDefinition::TYPE_PAGE_COUNT Layout::AutoTextDefinition.
- `increment` — The #increment method returns the increment value for Layout::AutoTextDefinition::TYPE_SEQUENCE Layout::AutoTextDefinitions.
- `increment=` — The #increment= method sets the increment value for Layout::AutoTextDefinition::TYPE_SEQUENCE Layout::AutoTextDefinitions.
- `mandatory?` — The #mandatory? method returns whether the Layout::AutoTextDefinition is mandatory or not.
- `name` — The #name method returns the name of the Layout::AutoTextDefinition.
- `name=` — The #name= method sets the name of the Layout::AutoTextDefinition.
- `number_style` — The #number_style method returns the numbering style for Layout::AutoTextDefinition::TYPE_PAGE_NUMBER, Layout::AutoTextDefinition::TYPE_PAGE_COUNT, and Layout::AutoTextDefinition::TYPE_SEQUENCE Layout::AutoTextDefinitions.
- `number_style=` — The #number_style= method sets the numbering style for Layout::AutoTextDefinition::TYPE_PAGE_NUMBER, Layout::AutoTextDefinition::TYPE_PAGE_COUNT, and Layout::AutoTextDefinition::TYPE_SEQUENCE Layout::AutoTextDefinitions.
- `page_number_style` — LayOut 2022.0 This method is deprecated in favor of the more generic #number_style method that also works on Layout::AutoTextDefinition::TYPE_PAGE_COUNT and Layout::AutoTextDefinition::TYPE_SEQUENCE Layout::AutoTextDefinitions.
- `page_number_style=` — LayOut 2022.0 This method is deprecated in favor of the more generic #number_style= method that also works on Layout::AutoTextDefinition::TYPE_PAGE_COUNT and Layout::AutoTextDefinition::TYPE_SEQUENCE Layout::AutoTextDefinitions.
- `renumber` — The #renumber method iterates through all uses of the Layout::AutoTextDefinition::TYPE_SEQUENCE Layout::AutoTextDefinition and eliminates gaps and duplicates in the sequence.
- `sequence_format` — The #sequence_format method returns the sequence format of a Layout::AutoTextDefinition::TYPE_SEQUENCE Layout::AutoTextDefinition.
- `sequence_format=` — The #sequence_format= method sets the sequence format of a Layout::AutoTextDefinition::TYPE_SEQUENCE Layout::AutoTextDefinition.
- `sequence_type` — The #sequence_type method returns how the Layout::AutoTextDefinition::TYPE_SEQUENCE Layout::AutoTextDefinition operates over multiple pages in a document.
- `sequence_type=` — The #sequence_type= method sets how the Layout::AutoTextDefinition::TYPE_SEQUENCE Layout::AutoTextDefinition operates over multiple pages in a document.
- `start_index` — The #start_index method returns the start index for Layout::AutoTextDefinition::TYPE_PAGE_NUMBER, Layout::AutoTextDefinition::TYPE_PAGE_COUNT, and Layout::AutoTextDefinition::TYPE_SEQUENCE Layout::AutoTextDefinitions.
- `start_index=` — The #start_index= method sets the start index for Layout::AutoTextDefinition::TYPE_PAGE_NUMBER, Layout::AutoTextDefinition::TYPE_PAGE_COUNT, and Layout::AutoTextDefinition::TYPE_SEQUENCE Layout::AutoTextDefinitions.
- `start_page` — The #start_page method returns the start page for Layout::AutoTextDefinition::TYPE_PAGE_NUMBER and Layout::AutoTextDefinition::TYPE_PAGE_COUNT Layout::AutoTextDefinitions.
- `start_page=` — The #start_page= method sets the start page for Layout::AutoTextDefinition::TYPE_PAGE_NUMBER and Layout::AutoTextDefinition::TYPE_PAGE_COUNT Layout::AutoTextDefinitions.
- `tag` — The #tag method returns the tag string of the Layout::AutoTextDefinition.
- `type` — The #type method returns the type of the Layout::AutoTextDefinition.
