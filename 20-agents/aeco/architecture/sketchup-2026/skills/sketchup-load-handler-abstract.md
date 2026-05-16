---
name: yard-sketchup-load-handler-abstract
description: Sketchup::LoadHandler   Abstract API reference (YARD)
---

# Sketchup::LoadHandler   Abstract API reference

Implement the methods described in this class to create a tool. You can not sub-class this class because it is not defined by the API.

## Methods

- `cancelled?` — This method is called when the download is canceled by the user.
- `onFailure` — This method is called when the download unsuccessfully completes
- `onPercentChange` — This method is triggered whenever the percent value updates.
- `onSuccess` — This method is called when the download successfully completes
