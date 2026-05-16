---
name: yard-length
description: Length API reference (YARD)
---

# Length API reference

Prior to SketchUp 2015, Length used to be derived from Float. This is no longer the case.

## Methods

- `<` — The < method is used to see if one length is less than another length.
- `<=` — The <= method is used to see if one length is less than or equal to another length.
- `<=>` — The <=> method is used to see if one length is less than equal or greater than another length. Because we change == for Length to do a test based on a tolerance, we also need to change <=> to also take tolerance into account.
- `==` — The == method is used to see if one length is equal to another length.
- `>` — The > method is used to see if one length is greater than another length.
- `>=` — The >= method is used to see if one length is greater than or equal to another length.
- `inspect` — The inspect method is used to retrieve an unformatted string for the length, which is the length in inches, regardless of the user's model unit settings. See Length.to_s for a way automatically format your Length to the user's model units.
- `to_f` — The to_f method is used to convert a length to a normal float.
- `to_s` — Format a length as a String using the current units formatting settings for the model. (So if the user's model is set to feet, this method will return a nicely formatted length in feet.)
