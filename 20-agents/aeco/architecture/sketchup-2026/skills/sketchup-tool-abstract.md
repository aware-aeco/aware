---
name: yard-sketchup-tool-abstract
description: Sketchup::Tool   Abstract API reference (YARD)
---

# Sketchup::Tool   Abstract API reference

Implement the methods described in this class to create a tool. You can not sub-class this class because it is not defined by the API.

## Methods

- `activate` — The #activate method is called by SketchUp when the tool is selected. It is a good place to put most of your initialization, such as instance variables to track the state of the tool.
- `deactivate` — The #deactivate method is called when the tool is deactivated because a different tool was selected.
- `draw` — If you draw outside the model bounds you need to implement #getExtents which return a bounding box large enough to include the points you draw. Otherwise your drawing will be clipped.
- `enableVCB?` — The #enableVCB? method is used to tell SketchUp whether to allow the user to enter text into the VCB (value control box, aka the “measurements” panel). If you do not implement this method, then the vcb is disabled by default.
- `getExtents` — In order to accurately draw things, SketchUp needs to know the extents of what it is drawing. If the tool is doing its own drawing, it may need to implement this method to tell SketchUp the extents of what it will be drawing. If you don't implement this method, you may find that part of what the tool is drawing gets clipped to the extents of the rest of the model.
- `getInstructorContentDirectory` — Prior to SketchUp 2014 this method would assume the path was relative to the SketchUp resource folder. From 2014 and onwards you can specify the absolute path to an HTML file or the absolute path to a directory containing an index.html file.
- `getMenu` — In SketchUp 2015 the flags, x, y and view parameters were added. They are needed if you need to pick the entities under the mouse position. The new parameters are optional, but if you need to use one you must include them all.
- `onCancel` — When something is undone #onCancel is called before the undo is actually executed. If you need to do something with the model after an undo use ModelObserver#onTransactionUndo.
- `onKeyDown` — The #onKeyDown method is called by SketchUp when the user presses a key on the keyboard. If you want to get input from the VCB, you should implement onUserText rather than this method.
- `onKeyUp` — The #onKeyUp method is called by SketchUp when the user releases a key on the keyboard.
- `onLButtonDoubleClick` — The #onLButtonDoubleClick is called by SketchUp when the user double clicks with the left mouse button.
- `onLButtonDown` — The #onLButtonDown method is called by SketchUp when the left mouse button is pressed. Most tools will implement this method.
- `onLButtonUp` — The #onLButtonUp method is called by SketchUp when the left mouse button is released.
- `onMButtonDoubleClick` — Though this method has been documented in the Ruby API for many years, it has never worked properly. We are leaving this documentation in place for now in the hopes of fixing the implementation, but you won't have any luck trying to use it in SU7 and earlier.
- `onMButtonDown` — The #onMButtonDown method is called by SketchUp when the middle mouse button (on a three button mouse) is down.
- `onMButtonUp` — The #onMButtonUp method is called by SketchUp when the middle mouse button (on a three button mouse) is released.
- `onMouseEnter` — The #onMouseEnter method is called by SketchUp when the mouse enters the viewport.
- `onMouseLeave` — The #onMouseLeave method is called by SketchUp when the mouse leaves the viewport.
- `onMouseMove` — The #onMouseMove method is called by SketchUp whenever the mouse is moved. You will often want to implement this method.
- `onMouseWheel` — The #onMouseWheel method is called by SketchUp when the mouse scroll wheel is used.
- `onRButtonDoubleClick` — The #onRButtonDoubleClick is called by SketchUp when the user double clicks with the right mouse button.
- `onRButtonDown` — The #onRButtonDown method is called by SketchUp when the user presses the right mouse button. Implement this method, along with the tool.getMenu method, when you want your tool to do something other than display the default context menu when the right mouse button is clicked.
- `onRButtonUp` — The #onRButtonUp method is called by SketchUp when the user releases the right mouse button.
- `onReturn` — The #onReturn method is called by SketchUp when the user hit the Return key to complete an operation in the tool. This method will rarely need to be implemented.
- `onSetCursor` — The #onSetCursor method is called by SketchUp when the tool wants to set the cursor.
- `onUserText` — The #onUserText method is called by SketchUp when the user has typed text into the VCB and hit return.
- `resume` — The #resume method is called by SketchUp when the tool becomes active again after being suspended.
- `suspend` — The #suspend method is called by SketchUp when the tool temporarily becomes inactive because another tool has been activated. This typically happens when a viewing tool is activated, such as when orbit is active due to the middle mouse button.
