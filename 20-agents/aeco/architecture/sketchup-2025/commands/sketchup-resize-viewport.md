# sketchup-resize-viewport

Lifecycle: single

In SketchUp 2024.0 and later this method doesn't behave correctly in all cases on Windows. The passed values are internally converted to logical pixels, rounded and converted back to physical pixels. This means certain sizes, such as 1000 px at 150% scaling, cannot be accurately set.
