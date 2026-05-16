# u-i-refresh-toolbars

Lifecycle: single

Tells SketchUp to refresh all floating toolbars. This is useful when you need to manually force a refresh after you've made a change to the document via Ruby. Generally, SketchUp will keep these in sync for you, but occasionally it does not, such as when Sketchup::Model#start_operation has disabled UI updates. This only affects macOS, on Windows the toolbars are always refreshing.
