# u-i-refresh-inspectors

Lifecycle: single

Tells SketchUp to refresh all inspectors such as the Component Browser and the Outliner. This is useful when you need to manually force a refresh after you've made a change to the document via Ruby. Generally, SketchUp will keep these in sync for you, but occasionally it does not, such as when model.start_operation has disabled UI updates.
