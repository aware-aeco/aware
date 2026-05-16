# sketchup-app-observer-abstract-on-unload-extension

Lifecycle: single

The #onUnloadExtension method is called when the user turns off a Ruby extension. This is useful for detecting if the user is deactivating some critical set of observers, for example, so you can warn them or cache your extension state.
