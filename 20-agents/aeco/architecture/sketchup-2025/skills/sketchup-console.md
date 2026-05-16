---
name: yard-sketchup-console
description: Sketchup::Console API reference (YARD)
---

# Sketchup::Console API reference

The Console class is used by SketchUp to direct $stdout and $stderr to the Ruby Console. It is a singleton class that only has one instance available. This instance is accessible via the SKETCHUP_CONSOLE constant.

## Methods

- `clear` — Clears the contents of SketchUp's Ruby Console.
- `hide` — Hides the SketchUp Ruby Console.
- `show` — Displays the SketchUp Ruby Console.
- `visible?` — Returns the visibility state of the SketchUp Ruby Console.
