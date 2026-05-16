---
name: yard-language-handler
description: LanguageHandler API reference (YARD)
---

# LanguageHandler API reference

The LanguageHandler class contains methods used to help make SketchUp extensions easier to localize across different languages. It looks for translated resources within the Resources folder in the extension's directory structure. All translated resources should be located within the appropriate language folder and encoded in UTF-8. The strings file should include “key”=“value” string pairs in the following format:

## Methods

- `initialize` — The new method is used to create a new LanguageHandler object.
- `[]` — Looks up and returns the localized version of a given string, based on the language SketchUp is currently running in, and the available translations in the Resources folder.
- `resource_path` — Returns a string containing the path to the given filename if it can be found in the Resources folder.
- `strings` — Returns a Hash object containing the localization dictionary.
