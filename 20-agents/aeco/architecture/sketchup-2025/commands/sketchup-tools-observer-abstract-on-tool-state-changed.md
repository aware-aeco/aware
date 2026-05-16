# sketchup-tools-observer-abstract-on-tool-state-changed

Lifecycle: single

In SketchUp 6 and SketchUp 7, tool names on the Mac have their first few characters truncated. For instance, on Windows, a tool is “CameraOrbit”. On the Mac, is comes across as “raOrbit”.  Therefore, use the tool_id to keep track of which tool you need to watch for, or use logic that corrects for the error. This bug was fixed in SketchUp 8.0.
