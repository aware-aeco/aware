# sketchup-animation-abstract-stop

Lifecycle: single

Do not call View#animation= from this method. This will cause a recursive loop and crash SketchUp 2017 and earlier versions. As of SketchUp 2018 this will raise a RunTimeError.
