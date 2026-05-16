# sketchup-tools-observer-abstract-on-active-tool-changed

Lifecycle: single

In SketchUp 6 and SketchUp 7.0, tool names on the Mac have some of their first characters truncated. For instance, on Windows, a tool is “CameraOrbit”. On the Mac, is comes across as “raOrbit”.  Therefore, use the tool_id to keep track of which tool you need to watch for, or use logic that corrects for the error. There is an example method of one way to do this shown below. (This example is not a comprehensive list of the tool names.)
