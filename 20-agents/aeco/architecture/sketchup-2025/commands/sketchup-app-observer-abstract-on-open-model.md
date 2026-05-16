# sketchup-app-observer-abstract-on-open-model

Lifecycle: single

If a skp file is loaded via the command line or double-clicking on a skp in explorer (which is also is the command line) then this observer will not be called.  The Ruby interpreter in SketchUp is initialized after command line processing so the observer won't be added in time to get the notification.
