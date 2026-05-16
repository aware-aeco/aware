---
name: yard-sketchup-app-observer-abstract
description: Sketchup::AppObserver   Abstract API reference (YARD)
---

# Sketchup::AppObserver   Abstract API reference

To implement this observer, create a Ruby class of this type, override the desired methods, and add an instance of the observer to the application class.

## Methods

- `expectsStartupModelNotifications` — Prior to SketchUp 2014, #onNewModel and #onOpenModel were not being called for the startup models. This issue is now fixed but observers still need to express their intent to receive these calls. This is for back-compatibility with existing scripts which worked around these missing calls by other means. For new code, this method should be implemented and should return true.
- `onActivateModel` — The #onActivateModel method is called when an open model is activated. This is relevant on Mac only which supports multiple documents to be opened simultaneously.
- `onExtensionsLoaded` — The #onExtensionsLoaded method is called when SketchUp has finished loading all extensions when the application starts.
- `onNewModel` — The #onNewModel method is called when the application creates a new, empty model.
- `onOpenModel` — If a skp file is loaded via the command line or double-clicking on a skp in explorer (which is also is the command line) then this observer will not be called.  The Ruby interpreter in SketchUp is initialized after command line processing so the observer won't be added in time to get the notification.
- `onQuit` — The #onQuit method is called when SketchUp closes. This is useful if you need to clean up anything or store your application state upon close.
- `onUnloadExtension` — The #onUnloadExtension method is called when the user turns off a Ruby extension. This is useful for detecting if the user is deactivating some critical set of observers, for example, so you can warn them or cache your extension state.
