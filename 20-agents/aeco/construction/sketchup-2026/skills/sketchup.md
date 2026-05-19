---
name: sketchup-sketchup
description: This skill encodes the sketchup 2026.0 surface of the Sketchup namespace — 93 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: Animation, AppObserver, ArcCurve, AttributeDictionaries, Axes, AttributeDictionary, Behavior, Camera, and 85 more types.
---

# Sketchup

Auto-generated from vendor docs for sketchup 2026.0. 93 types in this namespace.

## Animation (class)

This class is abstract. Implement the methods described in this class to create a an animation. You can not sub-class this class because it is not defined by the API. The Animation interface is implemented to create animations inside SketchUp. At any given time, a single animation can be active on a View. To make your own, build a Ruby class that contains the methods described below: Animation objects are activated by using the View#animation= method on a View object. To stop an animation set the view's animation object to nil, like so: Managing Multiple Animations: While only one animation object can be active on a View at any given time, you can create a composite animation class to manage multiple animations simultaneously. This approach allows you to animate different elements, such as objects and the camera, within a single animation framework. Example: Combining Animations

[Vendor docs](https://ruby.sketchup.com/Sketchup/Animation.html)

### Methods
#### `nextFrame(view) => Boolean`

The #nextFrame method is invoked by SketchUp to tell the animation to display its next frame.

**Remarks:** The #nextFrame method is invoked by SketchUp to tell the animation to display its next frame. This method should set up the camera and then call View#show_frame. The #nextFrame method is the only required method of the Sketchup::Animation interface that you must implement.

**Parameters:**
- `view` (Sketchup::View) — The view for the animation.

**Returns:** `Boolean` — continue - true if you want the animation to continue on to the next frame, false if you want the animation to stop after this frame.

[Docs](https://ruby.sketchup.com/Sketchup/Animation.html#nextFrame-instance_method)

#### `pause => nil`

Note: The user interface for pausing and resuming animations isn't integrated with the Ruby API in the current version, so this method is probably not useful to you.

**Remarks:** Note: The user interface for pausing and resuming animations isn't integrated with the Ruby API in the current version, so this method is probably not useful to you. The #pause method is invoked by SketchUp when the animation is paused. This method is optional (you do not need to implement this method unless you want to perform some specialized function when the animation is paused). You cannot call this method in your code explicitly and expect an animation to pause, only certain SketchUp events cause the method to be called.

**Returns:** `nil` — 

[Docs](https://ruby.sketchup.com/Sketchup/Animation.html#pause-instance_method)

#### `resume => nil`

Note: The user interface for pausing and resuming animations isn't integrated with the Ruby API in the current version, so this method is probably not useful to you.

**Remarks:** Note: The user interface for pausing and resuming animations isn't integrated with the Ruby API in the current version, so this method is probably not useful to you. The #resume method is invoked by SketchUp when the animation is resumed after being paused. This method is optional (you do not need to implement this method unless you want to perform some specialized function when the animation is resumed). You cannot call this method in your code explicitly and expect an animation to stop, only certain SketchUp events cause the method to be called.

**Returns:** `nil` — 

[Docs](https://ruby.sketchup.com/Sketchup/Animation.html#resume-instance_method)

#### `stop => nil`

Note: Do not call View#animation= from this method.

**Remarks:** Note: Do not call View#animation= from this method. This will cause a recursive loop and crash SketchUp 2017 and earlier versions. As of SketchUp 2018 this will raise a RunTimeError. The #stop method is invoked by SketchUp when the animation is stopped. This method is optional (you do not need to implement this method unless you want to perform some specialized function when the animation is stopped). You cannot call this method in your code explicitly and expect an animation to stop, only certain SketchUp events cause the method to be called. Perhaps the most common way for this method to be called is when your Ruby code sets View#animation= to nil. See the class comments for an example of this.

**Returns:** `nil` — 

[Docs](https://ruby.sketchup.com/Sketchup/Animation.html#stop-instance_method)

## AppObserver (class)

This class is abstract. To implement this observer, create a Ruby class of this type, override the desired methods, and add an instance of the observer to the application class. This observer interface is implemented to react to application events. This interface is often used to attach other observers to models as they are opened or started. This ensures that your observers are watching all open models. For example, when one attaches a SelectionObserver, it is only attached to the Selection collection of a given model. If a 2nd model is opened, the new model's selection changes will not fire selection callbacks unless you've attached a SelectionObserver to the new model as well. By watching for #onNewModel, you can be sure to do so.

[Vendor docs](https://ruby.sketchup.com/Sketchup/AppObserver.html)

### Methods
#### `expectsStartupModelNotifications => Boolean`

Note: Prior to SketchUp 2014, #onNewModel and #onOpenModel were not being called for the startup models.

**Remarks:** Note: Prior to SketchUp 2014, #onNewModel and #onOpenModel were not being called for the startup models. This issue is now fixed but observers still need to express their intent to receive these calls. This is for back-compatibility with existing scripts which worked around these missing calls by other means. For new code, this method should be implemented and should return true. The #expectsStartupModelNotifications method is called to determine if the observer expects to receive #onNewModel and #onOpenModel calls for the models that are created or opened at SketchUp startup. This includes the empty initial model, a model opened via command line arguments, or auto-restored models on Mac OS X.

**Returns:** `Boolean` — true to receive #onNewModel and #onOpenModel calls for startup models. Return false or simply not implement the method in order to not receive these calls (which was the behavior prior to SketchUp 2014).

[Docs](https://ruby.sketchup.com/Sketchup/AppObserver.html#expectsStartupModelNotifications-instance_method)

#### `onActivateModel(model) => nil`

The #onActivateModel method is called when an open model is activated.

**Remarks:** The #onActivateModel method is called when an open model is activated. This is relevant on Mac only which supports multiple documents to be opened simultaneously.

**Parameters:**
- `model` (Sketchup::Model) — The newly-activated model object.

**Returns:** `nil` — 

[Docs](https://ruby.sketchup.com/Sketchup/AppObserver.html#onActivateModel-instance_method)

#### `onExtensionsLoaded => Object`

The #onExtensionsLoaded method is called when SketchUp has finished loading all extensions when the application starts.

**Remarks:** The #onExtensionsLoaded method is called when SketchUp has finished loading all extensions when the application starts.

[Docs](https://ruby.sketchup.com/Sketchup/AppObserver.html#onExtensionsLoaded-instance_method)

#### `onNewModel(model) => nil`

The #onNewModel method is called when the application creates a new, empty model.

**Remarks:** The #onNewModel method is called when the application creates a new, empty model.

**Parameters:**
- `model` (Sketchup::Model) — The active model object.

**Returns:** `nil` — 

[Docs](https://ruby.sketchup.com/Sketchup/AppObserver.html#onNewModel-instance_method)

#### `onOpenModel(model) => nil`

Note: If a skp file is loaded via the command line or double-clicking on a skp in explorer (which is also is the command line) then this observer will not be called.

**Remarks:** Note: If a skp file is loaded via the command line or double-clicking on a skp in explorer (which is also is the command line) then this observer will not be called. The Ruby interpreter in SketchUp is initialized after command line processing so the observer won't be added in time to get the notification. The #onOpenModel method is called when the application opens an existing model.

**Parameters:**
- `model` (Sketchup::Model) — The active model object.

**Returns:** `nil` — 

[Docs](https://ruby.sketchup.com/Sketchup/AppObserver.html#onOpenModel-instance_method)

#### `onQuit => nil`

The #onQuit method is called when SketchUp closes.

**Remarks:** The #onQuit method is called when SketchUp closes. This is useful if you need to clean up anything or store your application state upon close.

**Returns:** `nil` — 

[Docs](https://ruby.sketchup.com/Sketchup/AppObserver.html#onQuit-instance_method)

#### `onUnloadExtension(extension_name) => nil`

The #onUnloadExtension method is called when the user turns off a Ruby extension.

**Remarks:** The #onUnloadExtension method is called when the user turns off a Ruby extension. This is useful for detecting if the user is deactivating some critical set of observers, for example, so you can warn them or cache your extension state.

**Parameters:**
- `extension_name` (String) — The name of the extension just unloaded.

**Returns:** `nil` — 

[Docs](https://ruby.sketchup.com/Sketchup/AppObserver.html#onUnloadExtension-instance_method)

## ArcCurve (class)

An ArcCurve is a Curve that makes up part of a circle. This is the underlying class for circles as well. You can think of ArcCurves as entities that were created with SketchUp's Arc or Circle drawing tools and Curves as entities that were created with the Freehand drawing tool. However, keep in mind that all Curves in SketchUp are really edges with some extra data attached to them. When you use the API to draw a Curve or ArcCurve, you are really drawing edges. ArcCurve is a subclass of Curve, so all of the methods that are available to Curves are also available to ArcCurves.

[Vendor docs](https://ruby.sketchup.com/Sketchup/ArcCurve.html)

### Methods
#### `center => Object`

The center method is used to retrieve the Point3d that is at the center of the circular arc.

**Remarks:** The center method is used to retrieve the Point3d that is at the center of the circular arc.

**Returns:** `Object` — point - a Point3d at the center of the arc if successful

[Docs](https://ruby.sketchup.com/Sketchup/ArcCurve.html#center-instance_method)

#### `circular? => Boolean`

Checks if the ArcCurve is a circle.

**Remarks:** Checks if the ArcCurve is a circle.

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Sketchup/ArcCurve.html#circular?-instance_method)

#### `end_angle => Float`

Note: A bug in SketchUp 2017 and older will report the end-angle for some circles as more than 360 degrees.

**Remarks:** Note: A bug in SketchUp 2017 and older will report the end-angle for some circles as more than 360 degrees. In such case, subtract 2 * PI from the end angle value. The #end_angle method is used to retrieve the angle of the end of the arc measured from the X axis in radians.

**Returns:** `Float` — 

[Docs](https://ruby.sketchup.com/Sketchup/ArcCurve.html#end_angle-instance_method)

#### `normal => Object`

The normal method retrieves a Vector3d that is perpendicular to the plane of the arc.

**Remarks:** The normal method retrieves a Vector3d that is perpendicular to the plane of the arc.

**Returns:** `Object` — vector - a Vector3d object if successful

[Docs](https://ruby.sketchup.com/Sketchup/ArcCurve.html#normal-instance_method)

#### `plane => Object`

The plane method is used to retrieve the plane of the arc.

**Remarks:** The plane method is used to retrieve the plane of the arc. Refer to the Geom module for instructions to create a plane.

**Returns:** `Object` — plane - the plane of the arc if successful

[Docs](https://ruby.sketchup.com/Sketchup/ArcCurve.html#plane-instance_method)

#### `radius => Object`

The radius method is used to retrieve the radius of the arc.

**Remarks:** The radius method is used to retrieve the radius of the arc.

**Returns:** `Object` — radius - the radius of the arc if successful

[Docs](https://ruby.sketchup.com/Sketchup/ArcCurve.html#radius-instance_method)

#### `start_angle => Object`

The start_angle method is used to retrieve the angle of the start of the arc, measured from the X axis in radians.

**Remarks:** The start_angle method is used to retrieve the angle of the start of the arc, measured from the X axis in radians.

**Returns:** `Object` — angle - the angle of the start of the arc if successful

[Docs](https://ruby.sketchup.com/Sketchup/ArcCurve.html#start_angle-instance_method)

#### `xaxis => Object`

The xaxis method is used to retrieve the X axis of the coordinate system for the curve.

**Remarks:** The xaxis method is used to retrieve the X axis of the coordinate system for the curve. Note that the length of the returned vector is equal to the radius of the underlying curve.

**Returns:** `Object` — vector - a Vector3d object if successful

[Docs](https://ruby.sketchup.com/Sketchup/ArcCurve.html#xaxis-instance_method)

#### `yaxis => Object`

The yaxis method is used to retrieve the Y axis of the coordinate system for the curve.

**Remarks:** The yaxis method is used to retrieve the Y axis of the coordinate system for the curve. Note that the length of the returned vector is equal to the radius of the underlying curve.

**Returns:** `Object` — vector - a Vector3d object if successful

[Docs](https://ruby.sketchup.com/Sketchup/ArcCurve.html#yaxis-instance_method)

## AttributeDictionaries (class)

The AttributeDictionaries class is a collection of all of the AttributeDictionary objects that are attached to a given Entity object. The Entity class is a popular parent class in SketchUp, meaning you can attach AttributeDictionaries to almost anything, from geometric items like edges and faces and components to more conceptual things like pages or materials. You access this class not by performing an AttributeDictionaries.new but by grabbing a handle from an existing entity.

[Vendor docs](https://ruby.sketchup.com/Sketchup/AttributeDictionaries.html)

### Methods
#### `[](key) => Sketchup::AttributeDictionary`

Get an AttributeDictionary by name.

**Remarks:** Get an AttributeDictionary by name. Returns nil if there is none with the given name.

**Parameters:**
- `key` (String) — The name of the attribute dictionary.

**Returns:** `Sketchup::AttributeDictionary` — the dictionary

[Docs](https://ruby.sketchup.com/Sketchup/AttributeDictionaries.html#[]-instance_method)

#### `count => Integer`

The count method is inherited from the Enumerable mix-in module.

**Remarks:** The count method is inherited from the Enumerable mix-in module.

**Returns:** `Integer` — 

[Docs](https://ruby.sketchup.com/Sketchup/AttributeDictionaries.html#count-instance_method)

#### `delete(key_or_dict) => Sketchup::AttributeDictionaries`

The delete method destroys a given AttributeDictionary.

**Remarks:** The delete method destroys a given AttributeDictionary. This AttributeDictionary can be passed directly or identified by its string name. In SketchUp 2018, special attribute dictionaries have been added. The name of these dictionaries are “SU_InstanceSet” and “SU_DefinitionSet”. The dictionaries cannot be deleted via ruby and an ArgumentError will be raised. The key/value pairs in the dictionary can be deleted safely. object

**Parameters:**
- `key_or_dict` (String, Sketchup::AttributeDictionary) — The name of the attribute dictionary to delete, or the dictionary object itself.

**Returns:** `Sketchup::AttributeDictionaries` — the modified AttributeDictionaries

[Docs](https://ruby.sketchup.com/Sketchup/AttributeDictionaries.html#delete-instance_method)

#### `each {|dictionary| ... } => Object`

The #each method is used to iterate through all of the attributes dictionaries.

**Remarks:** The #each method is used to iterate through all of the attributes dictionaries.

**Returns:** `Object` — nil

[Docs](https://ruby.sketchup.com/Sketchup/AttributeDictionaries.html#each-instance_method)

#### `length => Integer`

The #length method returns the number of attribute dictionary objects in the collection.

**Remarks:** The #length method returns the number of attribute dictionary objects in the collection.

**Returns:** `Integer` — the number of attribute dictionary objects in the collection.

[Docs](https://ruby.sketchup.com/Sketchup/AttributeDictionaries.html#length-instance_method)

#### `size => Integer`

The #size method is an alias of #length.

**Remarks:** The #size method is an alias of #length.

**Returns:** `Integer` — 

[Docs](https://ruby.sketchup.com/Sketchup/AttributeDictionaries.html#size-instance_method)

## AttributeDictionary (class)

The AttributeDictionary class allows you to attach arbitrary collections of attributes to a SketchUp entity. The attributes are defined by key/value pairs where the keys are strings. An Entity or Model object can have any number of AttributeDictionary objects (see the AttributeDictionaries class). The Entity class is a popular parent class in SketchUp, meaning you can attach attribute dictionaries to almost anything, from geometric items like edges and faces and components to more conceptual things like pages or materials.

[Vendor docs](https://ruby.sketchup.com/Sketchup/AttributeDictionary.html)

### Methods
#### `count => Integer`

The count method is inherited from the Enumerable mix-in module.

**Remarks:** The count method is inherited from the Enumerable mix-in module.

**Returns:** `Integer` — 

[Docs](https://ruby.sketchup.com/Sketchup/AttributeDictionary.html#count-instance_method)

#### `delete_key(key) => Object?`

The delete_key method is used to delete an attribute with a given key.

**Remarks:** The delete_key method is used to delete an attribute with a given key.

**Parameters:**
- `key` (String) — The key to be deleted.

**Returns:** `Object, nil` — the value of the key

[Docs](https://ruby.sketchup.com/Sketchup/AttributeDictionary.html#delete_key-instance_method)

#### `each {|key, value| ... } => Object`

Note: Don't remove content from this collection while iterating over it with #each.

**Remarks:** Note: Don't remove content from this collection while iterating over it with #each. This would change the size of the collection and cause elements to be skipped as the indices change. Instead copy the current collection to an array using to_a and then use each on the array, when removing content. The #each method iterate through all of the attributes.

[Docs](https://ruby.sketchup.com/Sketchup/AttributeDictionary.html#each-instance_method)

#### `each_key {|key| ... } => nil`

The #each_key method is used to iterate through all of the attribute keys.

**Remarks:** The #each_key method is used to iterate through all of the attribute keys.

**Returns:** `nil` — 

[Docs](https://ruby.sketchup.com/Sketchup/AttributeDictionary.html#each_key-instance_method)

#### `each_pair {|key, value| ... } => Object`

The #each_pair method is an alias for #each.

**Remarks:** The #each_pair method is an alias for #each.

[Docs](https://ruby.sketchup.com/Sketchup/AttributeDictionary.html#each_pair-instance_method)

#### `empty? => Boolean`

The #empty? method is used to check if the attribute dictionary is empty.

**Remarks:** The #empty? method is used to check if the attribute dictionary is empty.

**Returns:** `Boolean` — true if the attribute dictionary is empty, false otherwise

[Docs](https://ruby.sketchup.com/Sketchup/AttributeDictionary.html#empty?-instance_method)

#### `keys => Array<String>`

The keys method is used to retrieve an array with all of the attribute keys.

**Remarks:** The keys method is used to retrieve an array with all of the attribute keys.

**Returns:** `Array<String>` — an array of keys within the attribute dictionary if successful

[Docs](https://ruby.sketchup.com/Sketchup/AttributeDictionary.html#keys-instance_method)

#### `length => Integer`

The #length method is used to retrieve the size (number of elements) of an attribute dictionary.

**Remarks:** The #length method is used to retrieve the size (number of elements) of an attribute dictionary.

**Returns:** `Integer` — 

[Docs](https://ruby.sketchup.com/Sketchup/AttributeDictionary.html#length-instance_method)

#### `name => String`

The name method is used to retrieve the name of an attribute dictionary.

**Remarks:** The name method is used to retrieve the name of an attribute dictionary.

**Returns:** `String` — the name of the attribute dictionary if successful

[Docs](https://ruby.sketchup.com/Sketchup/AttributeDictionary.html#name-instance_method)

#### `size => Integer`

The #size method is an alias of #length.

**Remarks:** The #size method is an alias of #length.

**Returns:** `Integer` — 

[Docs](https://ruby.sketchup.com/Sketchup/AttributeDictionary.html#size-instance_method)

#### `values => Array<Object>`

The values method is used to retrieve an array with all of the attribute values.

**Remarks:** The values method is used to retrieve an array with all of the attribute values.

**Returns:** `Array<Object>` — an array of values within the attribute dictionary if successful

[Docs](https://ruby.sketchup.com/Sketchup/AttributeDictionary.html#values-instance_method)

### Properties
- `[]` (Object, nil, get/set) — The [] method is used to retrieve the attribute with a given key.

## Axes (class)

SketchUp's drawing axes consist of three colored lines (red, green, blue), usually perpendicular to each other, displayed in the drawing area. The exception is when the user open an instance with a non-orthogonal transformation. The root model transformation is always orthogonal. The drawing axes are used by drawing tools to align the geometry it creates as well as affecting the inference engine. The plane where the red and green axes lines lie is called the ground plane. The term origin, is used to define the place where all of axes lines start or originate.

[Vendor docs](https://ruby.sketchup.com/Sketchup/Axes.html)

### Methods
#### `axes => Object`

The axes method returns the vectors representing the directions of the axes.

**Remarks:** The axes method returns the vectors representing the directions of the axes.

**Returns:** `Object` — Array - an array of three vectors.

[Docs](https://ruby.sketchup.com/Sketchup/Axes.html#axes-instance_method)

#### `origin => Geom::Point3d`

The #origin method returns the origin of the axes.

**Remarks:** The #origin method returns the origin of the axes.

**Returns:** `Geom::Point3d` — 

[Docs](https://ruby.sketchup.com/Sketchup/Axes.html#origin-instance_method)

#### `set(origin, xaxis, yaxis, zaxis) => Sketchup::Axes`

The #set method allows the axes to be manipulated.

**Remarks:** The #set method allows the axes to be manipulated. The axes must always be orthogonal, otherwise an error is thrown.

**Parameters:**
- `origin` (Object) — Point3d - The origin to set.
- `xaxis` (Object) — Vector3d - The x axis to set.
- `yaxis` (Object) — Vector3d - The y axis to set.
- `zaxis` (Object) — Vector3d - The z axis to set.

**Returns:** `Sketchup::Axes` — the axes object being set.

[Docs](https://ruby.sketchup.com/Sketchup/Axes.html#set-instance_method)

#### `sketch_plane => Object`

The sketch_plane method returns a plane representing the ground plane of the axes.

**Remarks:** The sketch_plane method returns a plane representing the ground plane of the axes.

**Returns:** `Object` — Array - of 4 numbers that give the coefficients of a plane equation.

[Docs](https://ruby.sketchup.com/Sketchup/Axes.html#sketch_plane-instance_method)

#### `to_a => Object`

The axes method returns the origin and vectors representing the axes.

**Remarks:** The axes method returns the origin and vectors representing the axes.

**Returns:** `Object` — Array - an array of a point and three vectors.

[Docs](https://ruby.sketchup.com/Sketchup/Axes.html#to_a-instance_method)

#### `transformation => Geom::Transformation`

The #transformation method returns the transformation of the axes.

**Remarks:** The #transformation method returns the transformation of the axes. This is useful when creating tools that respect the model's drawing axes.

**Returns:** `Geom::Transformation` — the transformation for the axes.

[Docs](https://ruby.sketchup.com/Sketchup/Axes.html#transformation-instance_method)

#### `xaxis => Geom::Vector3d`

The #xaxis method returns the x axis of the axes.

**Remarks:** The #xaxis method returns the x axis of the axes.

**Returns:** `Geom::Vector3d` — the x axis for the axes.

[Docs](https://ruby.sketchup.com/Sketchup/Axes.html#xaxis-instance_method)

#### `yaxis => Geom::Vector3d`

The #yaxis method returns the y axis of the axes.

**Remarks:** The #yaxis method returns the y axis of the axes.

**Returns:** `Geom::Vector3d` — the y axis for the axes.

[Docs](https://ruby.sketchup.com/Sketchup/Axes.html#yaxis-instance_method)

#### `zaxis => Geom::Vector3d`

The #zaxis method returns the z axis of the axes.

**Remarks:** The #zaxis method returns the z axis of the axes.

**Returns:** `Geom::Vector3d` — the z axis for the axes.

[Docs](https://ruby.sketchup.com/Sketchup/Axes.html#zaxis-instance_method)

## Behavior (class)

The Behavior class is used to control the “behavior” of components, which roughly correlates to the series of options that you see in the Components dialog under the “edit” tab, such as whether it casts shadows, glues to walls, etc. A Behavior object is accessed from a ComponentDefinition object, not created with a Behavior.new call.

[Vendor docs](https://ruby.sketchup.com/Sketchup/Behavior.html)

### Methods
#### `always_face_camera? => Boolean`

The always_face_camera? method is used to retrieve the always_face_camera behavior for a component.

**Remarks:** The always_face_camera? method is used to retrieve the always_face_camera behavior for a component. If the always_face_camera behavior is true, then a component will always try to orient itself so that the -Y axis of the component is facing the camera.

**Returns:** `Boolean` — behavior - true if the component is set to always face the camera, false if the component is not set to always face camera.

[Docs](https://ruby.sketchup.com/Sketchup/Behavior.html#always_face_camera?-instance_method)

#### `cuts_opening? => Boolean`

The cuts_opening? method is used to get the status of a component's cut opening behavior.

**Remarks:** The cuts_opening? method is used to get the status of a component's cut opening behavior.

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Sketchup/Behavior.html#cuts_opening?-instance_method)

#### `is2d? => Boolean`

The #is2d? method is used to get whether the component can glue to other entities or not.

**Remarks:** The #is2d? method is used to get whether the component can glue to other entities or not.

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Sketchup/Behavior.html#is2d?-instance_method)

#### `no_scale_mask? => Boolean`

The no_scale_mask? method returns an integer that is a bit-by-bit description of which scale tool handles are hidden when the user selects this single component with the scale tool.

**Remarks:** The no_scale_mask? method returns an integer that is a bit-by-bit description of which scale tool handles are hidden when the user selects this single component with the scale tool. See the no_scale_mask= method for details on the bit encodings used.

**Returns:** `Boolean` — scale_mask - an integer describing which scale tool handles are hidden.

[Docs](https://ruby.sketchup.com/Sketchup/Behavior.html#no_scale_mask?-instance_method)

#### `shadows_face_sun? => Boolean`

The shadows_face_sun? method is used to determine whether the component's shadow is being cast from the component's current position (as though the component were facing the sun).

**Remarks:** The shadows_face_sun? method is used to determine whether the component's shadow is being cast from the component's current position (as though the component were facing the sun). See the Component entity within the SketchUp User's guide for more information on this feature.

**Returns:** `Boolean` — status - true if the component's is to be cast from the component's current position as though the component were facing the sun. False to cause the shadow to be cast from the component's current position.

[Docs](https://ruby.sketchup.com/Sketchup/Behavior.html#shadows_face_sun?-instance_method)

### Properties
- `always_face_camera` (Object, set) — The always_face_camera= method is used to set the always_face_camera behavior for a component.
- `cuts_opening` (Object, set) — Note: To enable cut opening, also set #is2d= to true.
- `is2d` (Boolean, set) — The #is2d= method is used to set whether the component can glue to other entities or not.
- `no_scale_mask` (Object, set) — Sets an integer that is really a bit-by-bit description of which scale tool handles are hidden on a given component.
- `shadows_face_sun` (Object, set) — The shadows_face_sun= method is used to identify whether the component's shadow will be cast from the component's current position as though the component were facing the sun.
- `snapto` (Integer, get/set) — The #snapto method is used to see how a component can glue to other entities.

## Camera (class)

The Camera class contains methods for creating and manipulating a camera. The camera in SketchUp is the “point of view” from which you look at the model.

[Vendor docs](https://ruby.sketchup.com/Sketchup/Camera.html)

### Constructors
- `Camera(eye, target, up, perspective = true, fov = 30.0)` — Returns a new camera with eye (where the camera is) and targets (where the camera is looking).

### Methods
#### `center_2d => Geom::Point3d`

The #center_2d method returns a point with the x and y offset of the camera when it's in two-point perspective or math photo mode.

**Remarks:** The #center_2d method returns a point with the x and y offset of the camera when it's in two-point perspective or math photo mode. When the camera is in two-point perspective and the user pans around, the x and y values will change. These values are in normalized device coordinates, so for instance, the range [-1.0, 1.0] spans the full width or height of the screen. The z value is unused and it is always zero.

**Returns:** `Geom::Point3d` — 

[Docs](https://ruby.sketchup.com/Sketchup/Camera.html#center_2d-instance_method)

#### `direction => Geom::Vector3d`

The #direction method is used to retrieve a Vector3d object in the direction that the Camera is pointing.

**Remarks:** The #direction method is used to retrieve a Vector3d object in the direction that the Camera is pointing.

**Returns:** `Geom::Vector3d` — 

[Docs](https://ruby.sketchup.com/Sketchup/Camera.html#direction-instance_method)

#### `eye => Geom::Point3d`

The #eye method is used to retrieve the eye Point3d object for the Camera.

**Remarks:** The #eye method is used to retrieve the eye Point3d object for the Camera.

**Returns:** `Geom::Point3d` — 

[Docs](https://ruby.sketchup.com/Sketchup/Camera.html#eye-instance_method)

#### `fov_is_height? => Boolean`

The #fov_is_height? method indicates whether the field of view is measured vertically, as opposed horizontally.

**Remarks:** The #fov_is_height? method indicates whether the field of view is measured vertically, as opposed horizontally.

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Sketchup/Camera.html#fov_is_height?-instance_method)

#### `is_2d? => Boolean`

The #is_2d? method indicates whether the camera mode is two-point perspective or match photo mode, as opposed to a normal perspective or parallel projection camera.

**Remarks:** The #is_2d? method indicates whether the camera mode is two-point perspective or match photo mode, as opposed to a normal perspective or parallel projection camera.

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Sketchup/Camera.html#is_2d?-instance_method)

#### `perspective? => Boolean`

The #perspective? method is used to determine whether a camera is a perspective or orthographic camera.

**Remarks:** The #perspective? method is used to determine whether a camera is a perspective or orthographic camera.

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Sketchup/Camera.html#perspective?-instance_method)

#### `scale_2d => Float`

The #scale_2d method returns a float indicating the scaling factor of 2 point perspective cameras.

**Remarks:** The #scale_2d method returns a float indicating the scaling factor of 2 point perspective cameras. When the camera is in two-point perspective and the user uses the zoom tools, this value will change. Zooming out will produce a value greater than 1.0.

**Returns:** `Float` — 

[Docs](https://ruby.sketchup.com/Sketchup/Camera.html#scale_2d-instance_method)

#### `set(eye, target, up) => Sketchup::Camera`

The #set method sets the camera orientation.

**Remarks:** The #set method sets the camera orientation. You have to set the camera eye, target and up parameters at the same time to make sure that you have a valid camera definition.

**Parameters:**
- `eye` (Geom::Point3d) — See #eye.
- `target` (Geom::Point3d) — See #target.
- `up` (Geom::Vector3d) — See #up.

**Returns:** `Sketchup::Camera` — 

[Docs](https://ruby.sketchup.com/Sketchup/Camera.html#set-instance_method)

#### `target => Geom::Point3d`

The #target method retrieves Point3d that the camera is pointing at.

**Remarks:** The #target method retrieves Point3d that the camera is pointing at.

**Returns:** `Geom::Point3d` — 

[Docs](https://ruby.sketchup.com/Sketchup/Camera.html#target-instance_method)

#### `up => Geom::Vector3d`

The #up method is used to retrieve the up vector for the camera.

**Remarks:** The #up method is used to retrieve the up vector for the camera. This is

**Returns:** `Geom::Vector3d` — 

[Docs](https://ruby.sketchup.com/Sketchup/Camera.html#up-instance_method)

#### `xaxis => Geom::Vector3d`

The #xaxis method is used to retrieve the x axis of the camera coordinate system.

**Remarks:** The #xaxis method is used to retrieve the x axis of the camera coordinate system. This value is computed from the cross product between the camera direction and the up vector.

**Returns:** `Geom::Vector3d` — 

[Docs](https://ruby.sketchup.com/Sketchup/Camera.html#xaxis-instance_method)

#### `yaxis => Geom::Vector3d`

The #yaxis method retrieves the y axis of the camera coordinate system.

**Remarks:** The #yaxis method retrieves the y axis of the camera coordinate system. This value is computed to be perpendicular the camera x and z axes. It is equivalent to the up direction, but is computed to make sure that it is perpendicular to the direction.

**Returns:** `Geom::Vector3d` — 

[Docs](https://ruby.sketchup.com/Sketchup/Camera.html#yaxis-instance_method)

#### `zaxis => Geom::Vector3d`

The #zaxis method retrieves the z axis of the camera coordinate system.

**Remarks:** The #zaxis method retrieves the z axis of the camera coordinate system. This value is computed. It is the same as Camera.direction

**Returns:** `Geom::Vector3d` — 

[Docs](https://ruby.sketchup.com/Sketchup/Camera.html#zaxis-instance_method)

### Properties
- `aspect_ratio` (Float, get/set) — The #aspect_ratio method is used to retrieve the aspect ratio of the Camera.
- `description` (String, get/set) — The #description method is used to retrieve the description for a Camera.
- `focal_length` (Float, get/set) — The focal_length method is used to get the focal length in millimeters of perspective Camera.
- `fov` (Float, get/set) — The #fov method retrieves the field of view of the Camera.
- `height` (Float, get/set) — The #height method retrieves the height of a Camera.
- `image_width` (Float, get/set) — The #image_width method returns the width of the image, as used to calculate the #focal_length.
- `perspective` (Boolean, set) — The #perspective= method is used to set whether or not this is a perspective camera or an orthographic camera.

## ClassificationSchema (class)

The ClassificationSchema class represent schemas loaded in the model.

[Vendor docs](https://ruby.sketchup.com/Sketchup/ClassificationSchema.html)

### Methods
#### `<=>(schema2) => Object`

The <=> method is used to compare two ClassificationSchema objects for sorting.

**Remarks:** The <=> method is used to compare two ClassificationSchema objects for sorting. The comparison is done based on the schema name.

**Parameters:**
- `schema2` (Object) — The second schema in the comparison.

**Returns:** `Object` — Integer - -1 if schema1 is less then schema2. 1 if schema1 is greater than schema2, 0 if the schemas are equal.

[Docs](https://ruby.sketchup.com/Sketchup/ClassificationSchema.html#<=>-instance_method)

#### `name => Object`

The name method returns the name of the schema.

**Remarks:** The name method returns the name of the schema.

**Returns:** `Object` — String - containing the schema name.

[Docs](https://ruby.sketchup.com/Sketchup/ClassificationSchema.html#name-instance_method)

#### `namespace => Object`

The namespace method returns the namespace of the schema.

**Remarks:** The namespace method returns the namespace of the schema.

**Returns:** `Object` — String - containing the schema namespace.

[Docs](https://ruby.sketchup.com/Sketchup/ClassificationSchema.html#namespace-instance_method)

## Classifications (class)

The Classifications class is a container/manager for all classifications in a model.

[Vendor docs](https://ruby.sketchup.com/Sketchup/Classifications.html)

### Methods
#### `[](index_or_name) => Object`

The [] method is used to get a classification schema by name or index.

**Remarks:** The [] method is used to get a classification schema by name or index.

**Parameters:**
- `index_or_name` (Object) — The index or name of the ClassificationSchema object.

**Returns:** `Object` — schema - a ClassificationSchema object if successful, otherwise nil.

[Docs](https://ruby.sketchup.com/Sketchup/Classifications.html#[]-instance_method)

#### `each {|schema| ... } => nil`

The #each method is used to iterate through loaded classification schemas.

**Remarks:** The #each method is used to iterate through loaded classification schemas.

**Returns:** `nil` — 

[Docs](https://ruby.sketchup.com/Sketchup/Classifications.html#each-instance_method)

#### `keys => Object`

The keys method is used to get a list of keys in the Classifications class, which are the same as the names of the schemas.

**Remarks:** The keys method is used to get a list of keys in the Classifications class, which are the same as the names of the schemas.

**Returns:** `Object` — keys - Array of string keys

[Docs](https://ruby.sketchup.com/Sketchup/Classifications.html#keys-instance_method)

#### `length => Integer`

The #length method returns the number of loaded classification schemas.

**Remarks:** The #length method returns the number of loaded classification schemas.

**Returns:** `Integer` — 

[Docs](https://ruby.sketchup.com/Sketchup/Classifications.html#length-instance_method)

#### `load_schema(file) => Object`

The load_schema method is used to load a classification schema into a model.

**Remarks:** The load_schema method is used to load a classification schema into a model.

**Parameters:**
- `file` (Object) — Full path to the schema file

**Returns:** `Object` — True if successful.

[Docs](https://ruby.sketchup.com/Sketchup/Classifications.html#load_schema-instance_method)

#### `size => Integer`

The #size method returns the number of loaded classification schemas.

**Remarks:** The #size method returns the number of loaded classification schemas.

**Returns:** `Integer` — 

[Docs](https://ruby.sketchup.com/Sketchup/Classifications.html#size-instance_method)

#### `unload_schema(schema_name) => Object`

The unload_schema method is used to unload a classification schema that was previously loaded into a model.

**Remarks:** The unload_schema method is used to unload a classification schema that was previously loaded into a model.

**Parameters:**
- `schema_name` (Object) — Name of the schema to unload

**Returns:** `Object` — True if successful.

[Docs](https://ruby.sketchup.com/Sketchup/Classifications.html#unload_schema-instance_method)

## Color (class)

The Color class is used to create and manipulate colors within SketchUp Models. The class can also be used the same way with LayOut documents. For methods that accept a Color object, such as the face.material method, you can pass in an actual Color object, or an object that can be converted to a Color. For example: SketchUp ships with several built in colors in the Materials Browser. These colors are listed in the following table. NameRGB ValuesSwatch AliceBlue240,248,255 AntiqueWhite250,235,215 Aqua0,255,255 Aquamarine127,255,212 Azure240,255,255 Beige245,245,220 Bisque255,228,196 Black0,0,0 BlanchedAlmond255,235,205 Blue0,0,255 BlueViolet138,43,226 Brown165,42,42 BurlyWood222,184,135 CadetBlue95,158,160 Chartreuse127,255,0 Chocolate210,105,30 Coral255,127,80 CornflowerBlue100,149,237 Cornsilk255,248,220 Crimson220,20,60 Cyan0,255,255 DarkBlue0,0,139 DarkCyan0,139,139 DarkGoldenrod184,134,11 DarkGray169,169,169 DarkGreen0,100,0 DarkKhaki189,183,107 DarkMagenta139,0,139 DarkOliveGreen85,107,47 DarkOrange255,140,0 DarkOrchid153,50,204 DarkRed139,0,0 DarkSalmon233,150,122 DarkSeaGreen143,188,143 DarkSlateBlue72,61,139 DarkSlateGray47,79,79 DarkTurquoise0,206,209 DarkViolet148,0,211 DeepPink255,20,147 DeepSkyBlue0,191,255 DimGray105,105,105 DodgerBlue30,144,255 FireBrick178,34,34 FloralWhite255,250,240 ForestGreen34,139,34 Fuchsia255,0,255 Gainsboro220,220,220 GhostWhite248,248,255 Gold255,215,0 Goldenrod218,165,32 Gray128,128,128 Green0,128,0 GreenYellow173,255,47 Honeydew240,255,240 HotPink255,105,180 IndianRed205,92,92 Indigo75,0,130 Ivory255,255,240 Khaki240,230,140 Lavender230,230,250 LavenderBlush255,240,245 LawnGreen124,252,0 LemonChiffon255,250,205 LightBlue173,216,230 LightCoral240,128,128 LightCyan224,255,255 LightGoldenrodYellow250,250,210 LightGreen144,238,144 LightGrey211,211,211 LightPink255,182,193 LightSalmon255,160,122 LightSeaGreen32,178,170 LightSkyBlue135,206,250 LightSlateGray119,136,153 LightSteelBlue176,196,222 LightYellow255,255,224 Lime0,255,0 LimeGreen50,205,50 Linen250,240,230 Magenta255,0,255 Maroon128,0,0 MediumAquamarine102,205,170 MediumBlue0,0,205 MediumOrchid186,85,211 MediumPurple147,112,219 MediumSeaGreen60,179,113 MediumSlateBlue123,104,238 MediumSpringGreen0,250,154 MediumTurquoise72,209,204 MediumVioletRed199,21,133 MidnightBlue25,25,112 MintCream245,255,250 MistyRose255,228,225 Moccasin255,228,181 NavajoWhite255,222,173 Navy0,0,128 OldLace253,245,230 Olive128,128,0 OliveDrab107,142,35 Orange255,165,0 OrangeRed255,69,0 Orchid218,112,214 PaleGoldenrod238,232,170 PaleGreen152,251,152 PaleTurquoise175,238,238 PaleVioletRed219,112,147 PapayaWhip255,239,213 PeachPuff255,218,185 Peru205,133,63 Pink255,192,203 Plum221,160,221 PowderBlue176,224,230 Purple128,0,128 Red255,0,0 RosyBrown188,143,143 RoyalBlue65,105,225 SaddleBrown139,69,19 Salmon250,128,114 SandyBrown244,164,96 SeaGreen46,139,87 Seashell255,245,238 Sienna160,82,45 Silver192,192,192 SkyBlue135,206,235 SlateBlue106,90,205 SlateGray112,128,144 Snow255,250,250 SpringGreen0,255,127 SteelBlue70,130,180 Tan210,180,140 Teal0,128,128 Thistle216,191,216 Tomato255,99,71 Turquoise64,224,208 Violet238,130,238 Wheat245,222,179 White255,255,255 WhiteSmoke245,245,245 Yellow255,255,0 YellowGreen154,205,50

[Vendor docs](https://ruby.sketchup.com/Sketchup/Color.html)

### Constructors
- `Color(red, green, blue, alpha = 255)` — The new method is used to create a new Color object.

### Methods
#### `blend(color2, weight) => Sketchup::Color`

The #blend method is used to blend two colors.

**Remarks:** The #blend method is used to blend two colors. The blended color will be the result of taking (1 - weight) * color1 + weight * color2. If weight = 0, you will get color2. If weight = 1 you will get color1.

**Parameters:**
- `color2` (Sketchup::Color) — The second color to be blended (with this color).
- `weight` (Float) — A Float between 0.0 and 1.0

**Returns:** `Sketchup::Color` — 

[Docs](https://ruby.sketchup.com/Sketchup/Color.html#blend-instance_method)

#### `names => Array`

The names method is used to retrieve an array of all color names recognized by SketchUp.

**Remarks:** The names method is used to retrieve an array of all color names recognized by SketchUp. In general, whenever a method wants a color, you can pass in a String with one of these names.

**Returns:** `Array` — 

[Docs](https://ruby.sketchup.com/Sketchup/Color.html#names-class_method)

#### `to_a => Array`

The #to_a method is used to convert a Color object to an Array object.

**Remarks:** The #to_a method is used to convert a Color object to an Array object. The returned array will contain 4 integer values (RGBA) between 0 and 255.

**Returns:** `Array` — 

[Docs](https://ruby.sketchup.com/Sketchup/Color.html#to_a-instance_method)

#### `to_i => Integer`

The #to_i method is used to convert a Color object to an 32 bit integer.

**Remarks:** The #to_i method is used to convert a Color object to an 32 bit integer.

**Returns:** `Integer` — 

[Docs](https://ruby.sketchup.com/Sketchup/Color.html#to_i-instance_method)

#### `to_s => String`

The #to_s method returns a string representation of the Sketchup::Color object, in the form of “Color(255, 255, 255, 255)”.

**Remarks:** The #to_s method returns a string representation of the Sketchup::Color object, in the form of “Color(255, 255, 255, 255)”.

**Returns:** `String` — 

[Docs](https://ruby.sketchup.com/Sketchup/Color.html#to_s-instance_method)

### Properties
- `=` (Object, set) — The #== method checks to see if the two Sketchup::Colors are equal.
- `alpha` (Integer, get/set) — The #alpha method is used to retrieve the opacity of the color.
- `blue` (Integer, get/set) — The #blue method is used to retrieve the blue value of a color.
- `green` (Integer, get/set) — The #green method is used to retrieve the green value of a color.
- `red` (Integer, get/set) — The #red method is used to retrieve the red component of a RGB Color.

## ComponentDefinition (class)

The ComponentDefinition class is used to define the contents for a SketchUp component. Components are a collection of entities that can be instanced and reused multiple times throughout a model. For example, you could draw a chair once, turn it into a component, and then use 6 instances of it to surround a table. Edits to the original “definition” will then propagate across all of its instances. The ComponentDefinition class contains the global entities and settings for each definition. See the ComponentInstance class for how each copy is defined. Starting from SketchUp 2018+, the ComponentDefinition class contains a new default attribute dictionary named “SU_DefinitionSet” with default keys named named “Price”, “Size”, “Url”. See the Help article for more information. The dictionary cannot be deleted via ruby and an ArgumentError will be raised. The key/value pairs in the dictionary can be deleted safely.

[Vendor docs](https://ruby.sketchup.com/Sketchup/ComponentDefinition.html)

### Methods
#### `<=>(compdef2) => Integer`

The <=> method is used to compare two ComponentDefinition objects for sorting.

**Remarks:** The <=> method is used to compare two ComponentDefinition objects for sorting. The comparison is done based on the component name.

**Parameters:**
- `compdef2` (Sketchup::ComponentDefinition) — The second component definition in the comparison.

**Returns:** `Integer` — a -1 if component1 is less then component2. A 1 if component1 greater than component2

[Docs](https://ruby.sketchup.com/Sketchup/ComponentDefinition.html#<=>-instance_method)

#### `add_classification(schema_name, schema_type) => Boolean`

The add_classification method is used to add a given classification to the component.

**Remarks:** The add_classification method is used to add a given classification to the component. Note that you cannot classify image definitions.

**Parameters:**
- `schema_name` (String) — a String - Schema name to add
- `schema_type` (String) — a String - Schema type to add

**Returns:** `Boolean` — true if the classification succeeds. Otherwise false.

[Docs](https://ruby.sketchup.com/Sketchup/ComponentDefinition.html#add_classification-instance_method)

#### `add_observer(observer) => Boolean`

The add_observer method is used to add an observer to the current object.

**Remarks:** The add_observer method is used to add an observer to the current object.

**Parameters:**
- `observer` (Object) — An observer.

**Returns:** `Boolean` — true if successful, false if unsuccessful.

[Docs](https://ruby.sketchup.com/Sketchup/ComponentDefinition.html#add_observer-instance_method)

#### `behavior => Sketchup::Behavior`

The behavior method is used to retrieve the Behavior object associated with a component definition.

**Remarks:** The behavior method is used to retrieve the Behavior object associated with a component definition.

**Returns:** `Sketchup::Behavior` — a Behavior object if successful

[Docs](https://ruby.sketchup.com/Sketchup/ComponentDefinition.html#behavior-instance_method)

#### `count_instances => Integer`

The count_instances method is used to count the number of unique component instances in a model using this component definition.

**Remarks:** The count_instances method is used to count the number of unique component instances in a model using this component definition. This does not represent the total number of instances placed in the model as it doesn't take into account instances inside unused definitions.

**Returns:** `Integer` — the number of component instances of this component definition (if successful)

[Docs](https://ruby.sketchup.com/Sketchup/ComponentDefinition.html#count_instances-instance_method)

#### `count_used_instances => Integer`

The count_used_instances method is used to count the total number of component instances in a model using this component definition.

**Remarks:** The count_used_instances method is used to count the total number of component instances in a model using this component definition. This method takes into account the full hierarchy of the model.

**Returns:** `Integer` — the number of component instances of this component definition (if successful)

[Docs](https://ruby.sketchup.com/Sketchup/ComponentDefinition.html#count_used_instances-instance_method)

#### `entities => Sketchup::Entities`

The entities method retrieves a collection of all the entities in the component definition

**Remarks:** The entities method retrieves a collection of all the entities in the component definition

**Returns:** `Sketchup::Entities` — an Entities object if successful

[Docs](https://ruby.sketchup.com/Sketchup/ComponentDefinition.html#entities-instance_method)

#### `get_classification_value(path) => Object?`

The get_classification_value method is used to retrieve the value from a classification attribute given a key path.

**Remarks:** The get_classification_value method is used to retrieve the value from a classification attribute given a key path.

**Parameters:**
- `path` (Array<String>) — An array composed of the key path to the value.

**Returns:** `Object, nil` — a Ruby object if successful, nil otherwise.

[Docs](https://ruby.sketchup.com/Sketchup/ComponentDefinition.html#get_classification_value-instance_method)

#### `group? => Boolean`

The group? method is used to determine if this component definition is used to hold the elements of a group.

**Remarks:** The group? method is used to determine if this component definition is used to hold the elements of a group.

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Sketchup/ComponentDefinition.html#group?-instance_method)

#### `guid => String`

The guid method is used to retrieve the unique identifier of this component definition.

**Remarks:** The guid method is used to retrieve the unique identifier of this component definition. The guid changes after the component definition is modified and the component edit is exited.

**Returns:** `String` — a string guid if successful

[Docs](https://ruby.sketchup.com/Sketchup/ComponentDefinition.html#guid-instance_method)

#### `hidden? => Boolean`

The #hidden? method is used to determine if this component definition is hidden in the component browser.

**Remarks:** The #hidden? method is used to determine if this component definition is hidden in the component browser. This is based on how its instances are placed in the model hierarchy. For more details, see this article. In addition, component definitions used by Groups and Images are always hidden in the Component Browser. See #group? and #image?.

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Sketchup/ComponentDefinition.html#hidden?-instance_method)

#### `image? => Boolean`

The image? method is used to determine if this component definition is used to define an image.

**Remarks:** The image? method is used to determine if this component definition is used to define an image.

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Sketchup/ComponentDefinition.html#image?-instance_method)

#### `instances => Array<Sketchup::ComponentInstance>`

The instances method is used to return any array of ComponentInstancesfor this ComponentDefinition.

**Remarks:** The instances method is used to return any array of ComponentInstancesfor this ComponentDefinition.

**Returns:** `Array<Sketchup::ComponentInstance>` — an array of ComponentInstances (if successful)

[Docs](https://ruby.sketchup.com/Sketchup/ComponentDefinition.html#instances-instance_method)

#### `internal? => Boolean`

The internal? method is used to determine if the component definition is internal to the Component Browser

**Remarks:** The internal? method is used to determine if the component definition is internal to the Component Browser

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Sketchup/ComponentDefinition.html#internal?-instance_method)

#### `invalidate_bounds => Boolean`

Invalidates the bounding box of your definition.

**Remarks:** Invalidates the bounding box of your definition. This command forces the update of the bounding box of definition while inside an operation. See Model.start_operation for how to start an operation. This method is useful if you make changes to your geometry using the Ruby API and then need to know your bounding box size. This method forces SketchUp to recalculate the definition's bounding box when you choose.

**Returns:** `Boolean` — true if successful

[Docs](https://ruby.sketchup.com/Sketchup/ComponentDefinition.html#invalidate_bounds-instance_method)

#### `live_component? => Boolean`

Note: These components are parametrically generated and API users should not modify them.

**Remarks:** Note: These components are parametrically generated and API users should not modify them. The #live_component? method is used to identify Live Components and sub-definitions of Live Components.

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Sketchup/ComponentDefinition.html#live_component?-instance_method)

#### `load_time => Time`

The #load_time method gets the load time of the component definition.

**Remarks:** The #load_time method gets the load time of the component definition. For an internal component definition, this is the time that it was created. For an external component definition, this is the time that it was added to the model.

**Returns:** `Time` — 

[Docs](https://ruby.sketchup.com/Sketchup/ComponentDefinition.html#load_time-instance_method)

#### `path => String`

The path method is used to retrieve the path where the component was loaded.

**Remarks:** The path method is used to retrieve the path where the component was loaded.

**Returns:** `String` — Returns empty string if it is an internal component.

[Docs](https://ruby.sketchup.com/Sketchup/ComponentDefinition.html#path-instance_method)

#### `refresh_thumbnail => nil`

The refresh_thumbnail method is used to force SketchUp to regenerate the thumbnail image that appears in the component browser.

**Remarks:** The refresh_thumbnail method is used to force SketchUp to regenerate the thumbnail image that appears in the component browser. This is useful if you've used the API to change the geometry of your component and would like the thumbnail to match.

**Returns:** `nil` — 

[Docs](https://ruby.sketchup.com/Sketchup/ComponentDefinition.html#refresh_thumbnail-instance_method)

#### `remove_classification(schema_name, schema_type) => Boolean`

The remove_classification method is used to remove a given classification from the component.

**Remarks:** The remove_classification method is used to remove a given classification from the component. Note that you cannot classify image definitions.

**Parameters:**
- `schema_name` (String) — Schema name to remove
- `schema_type` (String) — Schema type to remove. If not provided or an empty string, the currently applied schema type for the given schema name will be removed.

**Returns:** `Boolean` — true if the removal succeeds. Otherwise false.

[Docs](https://ruby.sketchup.com/Sketchup/ComponentDefinition.html#remove_classification-instance_method)

#### `remove_observer(observer) => Boolean`

The remove_observer method is used to remove an observer from the current object.

**Remarks:** The remove_observer method is used to remove an observer from the current object.

**Parameters:**
- `observer` (Object) — An observer.

**Returns:** `Boolean` — true if successful, false if unsuccessful.

[Docs](https://ruby.sketchup.com/Sketchup/ComponentDefinition.html#remove_observer-instance_method)

#### `save_as(file_path) => Boolean | save_as(file_path, version) => Boolean`

The #save_as method is used to save your definition as a SketchUp file at the specified file destination.

**Remarks:** The #save_as method is used to save your definition as a SketchUp file at the specified file destination. Use this method when the user has chosen a path. If you want to “silently” save out the definition, without changing the path it is associated with, use #save_copy instead.

**Parameters:**
- `file_path` (String)

**Returns:** `Boolean` — true if successful, false otherwise

[Docs](https://ruby.sketchup.com/Sketchup/ComponentDefinition.html#save_as-instance_method)

#### `save_copy(file_path) => Boolean | save_copy(file_path, version) => Boolean`

The #save_copy method is used to save your definition as a SketchUp file without changing the file path it is already associated with.

**Remarks:** The #save_copy method is used to save your definition as a SketchUp file without changing the file path it is already associated with. This can be used to save out to a temporary file used by some other process, without having the temporary path permanentely written to the SketchUp model.

**Parameters:**
- `file_path` (String)

**Returns:** `Boolean` — true if successful

[Docs](https://ruby.sketchup.com/Sketchup/ComponentDefinition.html#save_copy-instance_method)

#### `save_thumbnail(filename) => Boolean`

Saves a component thumbnail image.

**Remarks:** Saves a component thumbnail image. The image format is specified by the file extension of filePath. Supported image formats are bmp, jpg, png, tif, pct, and gif.

**Parameters:**
- `filename` (String)

**Returns:** `Boolean` — true if successful, false otherwise.

[Docs](https://ruby.sketchup.com/Sketchup/ComponentDefinition.html#save_thumbnail-instance_method)

#### `set_classification_value(path, value) => Boolean`

The set_classification_value method is used to set the value of a classification attribute given a key path.

**Remarks:** The set_classification_value method is used to set the value of a classification attribute given a key path.

**Parameters:**
- `path` (Array<String>) — An array composed of the key path to the value.
- `value` (Object) — A value valid for that specific attribute.

**Returns:** `Boolean` — true if the path was valid, false otherwise.

[Docs](https://ruby.sketchup.com/Sketchup/ComponentDefinition.html#set_classification_value-instance_method)

### Properties
- `=` (Sketchup::ComponentDefinition, set) — The == method is used to test if two ComponentDefinition objects are the same (based on their address in memory).
- `description` (String, get/set) — The description method is used to retrieve the description of the component definition.
- `insertion_point` (Geom::Point3d, get/set) — Deprecated.
- `name` (String, get/set) — The name method retrieves the name of the component definition.
- `thumbnail_camera` (Sketchup::Camera, get/set) — The #thumbnail_camera method is used to retrieve a camera representing the thumbnail associated with the component definition.

## ComponentInstance (class)

The ComponentInstance class is used to represent component instances of a component definition or components that have been dragged from the Component Browser and placed (thus, instanced) within the Model. Therefore, the ComponentInstance class contains a reference to a corresponding ComponentDefinition object and a Transformation object (which contains the location of the component in the Drawing Window). Starting from SketchUp 2018+, the ComponentInstance class contains default attributes when created or imported. The attributes are: “Owner”, “Status”. See the Help article for more information. The dictionary cannot be deleted via ruby and an @raise ArgumentError will be raised. The key/value pairs in the dictionary can be deleted safely.

[Vendor docs](https://ruby.sketchup.com/Sketchup/ComponentInstance.html)

### Methods
#### `add_observer(observer) => Boolean`

The add_observer method is used to add an observer to the current object.

**Remarks:** The add_observer method is used to add an observer to the current object.

**Parameters:**
- `observer` (Object) — An observer.

**Returns:** `Boolean` — true if successful, false if unsuccessful.

[Docs](https://ruby.sketchup.com/Sketchup/ComponentInstance.html#add_observer-instance_method)

#### `equals?(instance) => Boolean`

The equals? method is used to determine if a component instance is geometrically equivalent to another instance.

**Remarks:** The equals? method is used to determine if a component instance is geometrically equivalent to another instance.

**Parameters:**
- `instance` (Sketchup::ComponentInstance) — The instance to compare this instance with.

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Sketchup/ComponentInstance.html#equals?-instance_method)

#### `explode => Array<Sketchup::Entity>, false`

The explode method is used to explode the component instance into separate entities.

**Remarks:** The explode method is used to explode the component instance into separate entities.

**Returns:** `Array<Sketchup::Entity>, false` — An array of entity objects if successful, false if unsuccessful

[Docs](https://ruby.sketchup.com/Sketchup/ComponentInstance.html#explode-instance_method)

#### `guid => String`

The guid method is used to get the base 64 encoded unique id for this SketchUp object.

**Remarks:** The guid method is used to get the base 64 encoded unique id for this SketchUp object.

**Returns:** `String` — a unique 22 character string

[Docs](https://ruby.sketchup.com/Sketchup/ComponentInstance.html#guid-instance_method)

#### `intersect(instance) => Sketchup::Group?`

Note: This method is not available in SketchUp Make.

**Remarks:** Note: This method is not available in SketchUp Make. The intersect method is used to compute the boolean intersection of two instances representing manifold solid volumes (this - arg). If the specified objects (this and arg) do not represent manifold volumes, this method fails.

**Parameters:**
- `instance` (Sketchup::ComponentInstance) — The instance to intersect this instance with.

**Returns:** `Sketchup::Group, nil` — The resultant group if the two objects (this and arg) represent manifold solids and the operation succeeds otherwise nil is returned.

[Docs](https://ruby.sketchup.com/Sketchup/ComponentInstance.html#intersect-instance_method)

#### `locked? => Boolean`

The locked? method is used to determine if a component instance is locked.

**Remarks:** The locked? method is used to determine if a component instance is locked.

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Sketchup/ComponentInstance.html#locked?-instance_method)

#### `make_unique => Sketchup::ComponentInstance`

The #make_unique method is used to create a component definition for this instance that is not used by any other instances.

**Remarks:** The #make_unique method is used to create a component definition for this instance that is not used by any other instances. If the component is already unique in the model, nothing happens.

**Returns:** `Sketchup::ComponentInstance` — returns itself

[Docs](https://ruby.sketchup.com/Sketchup/ComponentInstance.html#make_unique-instance_method)

#### `manifold? => Boolean`

The manifold? method is used to determine if an instance is manifold.

**Remarks:** The manifold? method is used to determine if an instance is manifold.

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Sketchup/ComponentInstance.html#manifold?-instance_method)

#### `move!(transformation) => Boolean`

Note: Despite the name being similar to #transform!, this method closer corresponds to #transformation=.

**Remarks:** Note: Despite the name being similar to #transform!, this method closer corresponds to #transformation=. The #move! method is used to set the transformation of this component instance, similarly to #transformation= but without recording to the undo stack. This method is useful for moving entities inside of an animation or page transition.

**Parameters:**
- `transformation` (Geom::Transformation)

**Returns:** `Boolean` — true if successful, false if unsuccessful

[Docs](https://ruby.sketchup.com/Sketchup/ComponentInstance.html#move!-instance_method)

#### `outer_shell(instance) => Sketchup::Group?`

The outer_shell method is used to compute the outer shell of the two instances representing manifold solid volumes (this || arg).

**Remarks:** The outer_shell method is used to compute the outer shell of the two instances representing manifold solid volumes (this || arg). If the specified objects (this and arg) do not represent manifold volumes, this method fails.

**Parameters:**
- `instance` (Sketchup::ComponentInstance) — The instance to outer shell this instance with.

**Returns:** `Sketchup::Group, nil` — The resultant group if the two objects (this and arg) represent manifold solids and the operation succeeds otherwise nil is returned.

[Docs](https://ruby.sketchup.com/Sketchup/ComponentInstance.html#outer_shell-instance_method)

#### `remove_observer(observer) => Boolean`

The remove_observer method is used to remove an observer from the current object.

**Remarks:** The remove_observer method is used to remove an observer from the current object.

**Parameters:**
- `observer` (Object) — An observer.

**Returns:** `Boolean` — true if successful, false if unsuccessful.

[Docs](https://ruby.sketchup.com/Sketchup/ComponentInstance.html#remove_observer-instance_method)

#### `show_differences(instance, verbose) => Boolean`

The show_differences method is used to determine if a component instance is geometrically equivalent to another instance and in addition move the non- matching and matching geometry to new layers.

**Remarks:** The show_differences method is used to determine if a component instance is geometrically equivalent to another instance and in addition move the non- matching and matching geometry to new layers. This method will move both instances to Layer0. Geometry that is the same in both components will be moved to a new layer named def_name + “_same”. Geometry that is not the same will be moved to a layer named def_name + “_diff”. If verbose is true, a list of all the geometry that is different from one component to the other is displayed texturally in the Ruby Console.

**Parameters:**
- `instance` (Sketchup::ComponentInstance) — The instance to be compared with.
- `verbose` (Boolean) — A boolean flag indicating whether to display a textural report of the found differences to the Ruby console.

**Returns:** `Boolean` — true if the instances are geometrically equivalent, otherwise false.

[Docs](https://ruby.sketchup.com/Sketchup/ComponentInstance.html#show_differences-instance_method)

#### `split(instance) => Array(Sketchup::Group, Sketchup::Group, Sketchup::Group)`

Note: This method is not available in SketchUp Make.

**Remarks:** Note: This method is not available in SketchUp Make. The split method is used to compute the boolean split (map overlay)of the two instances representing manifold solid volumes (this - arg). If the specified objects (this and arg) do not represent manifold volumes, this method fails. resultant groups if the two objects (this and arg) represent manifold solids and the operation succeeds otherwise nil is returned. The 3 groups are as follows: The intersection of volume 1 & volume 2, the difference of volume 1 minus volume 2, and the reverse difference of volume 2 minus volume 1.

**Parameters:**
- `instance` (Sketchup::ComponentInstance, nil) — The instance to split this instance with.

**Returns:** `Array(Sketchup::Group, Sketchup::Group, Sketchup::Group)` — A vector (array) of the three

[Docs](https://ruby.sketchup.com/Sketchup/ComponentInstance.html#split-instance_method)

#### `subtract(instance) => Sketchup::Group?`

Note: This method is not available in SketchUp Make.

**Remarks:** Note: This method is not available in SketchUp Make. The subtract method is used to compute the boolean difference of the two instances representing manifold solid volumes (this - arg). If the specified objects (this and arg) do not represent manifold volumes, this method fails.

**Parameters:**
- `instance` (Sketchup::ComponentInstance) — The instance to subtract this instance from.

**Returns:** `Sketchup::Group, nil` — The resultant group if the two objects (this and arg) represent manifold solids and the operation succeeds otherwise nil is returned.

[Docs](https://ruby.sketchup.com/Sketchup/ComponentInstance.html#subtract-instance_method)

#### `transform!(transform) => Boolean`

Apply a Geom::Transformation to a component instance.

**Remarks:** Apply a Geom::Transformation to a component instance.

**Parameters:**
- `transform` (Geom::Transformation) — The transformation object to apply to the component instance.

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Sketchup/ComponentInstance.html#transform!-instance_method)

#### `trim(instance) => Sketchup::Group?`

Note: Trimming object instance2 using instance1 results in a new trimmed version of instance2.

**Remarks:** Note: Trimming object instance2 using instance1 results in a new trimmed version of instance2. If the trim is successful the original instance2 is erased and a newly trimmed version is created. This new version, derived from the trimming operation, will possess a new GUID and will be returned. Note: This method is not available in SketchUp Make. The #trim method is used to compute the (non-destructive) boolean difference of the two instances representing manifold solid volumes (this - arg). If the specified objects (this and arg) do not represent manifold volumes, this method fails.

**Parameters:**
- `instance` (Sketchup::ComponentInstance) — The instance to trim this instance against.

**Returns:** `Sketchup::Group, nil` — The resultant group if the two objects (this and arg) represent manifold solids and the operation succeeds otherwise nil is returned.

[Docs](https://ruby.sketchup.com/Sketchup/ComponentInstance.html#trim-instance_method)

#### `union(instance) => Sketchup::Group?`

Note: This method is not available in SketchUp Make.

**Remarks:** Note: This method is not available in SketchUp Make. The union method is used to compute the boolean union of the two instances representing manifold solid volumes (this | arg). If the specified objects (this and arg) do not represent manifold volumes, this method fails.

**Parameters:**
- `instance` (Sketchup::ComponentInstance) — The instance to union this instance with.

**Returns:** `Sketchup::Group, nil` — The resultant group if the two objects (this and arg) represent manifold solids and the operation succeeds otherwise nil is returned.

[Docs](https://ruby.sketchup.com/Sketchup/ComponentInstance.html#union-instance_method)

#### `volume => Float`

The volume method is used to compute the volume of this instance if and only if this instance is manifold.

**Remarks:** The volume method is used to compute the volume of this instance if and only if this instance is manifold.

**Returns:** `Float` — If the instance represents a manifold volume, volume will be a positive value. If volume is negative, the instance is not manifold.

[Docs](https://ruby.sketchup.com/Sketchup/ComponentInstance.html#volume-instance_method)

### Properties
- `definition` (Sketchup::ComponentDefinition, get/set) — The definition method is used to retrieve the component definition for this component instance.
- `glued_to` (Sketchup::Face, Sketchup::Group, Sketchup::ComponentInstance, Sketchup::Image, nil, get/set) — The #glued_to method is used to retrieve the entity that this instance is glued to.
- `locked` (Boolean, set) — The locked= method is used to lock a component instance.
- `name` (String, get/set) — The name method is used to get the name of this instance.
- `transformation` (Geom::Transformation, get/set) — The transformation method is used to retrieve the transformation of this instance.

## Console (class)

The Console class is used by SketchUp to direct $stdout and $stderr to the Ruby Console. It is a singleton class that only has one instance available. This instance is accessible via the SKETCHUP_CONSOLE constant. In SketchUp 2014 methods were added to allow developers to control the visibility of the Ruby Console. Also note that in SketchUp 2014, writing to the console does not work from within Ruby threads other than the main thread. So the following code will not produce any output.

[Vendor docs](https://ruby.sketchup.com/Sketchup/Console.html)

### Methods
#### `clear => Object`

Clears the contents of SketchUp's Ruby Console.

**Remarks:** Clears the contents of SketchUp's Ruby Console.

**Returns:** `Object` — true.

[Docs](https://ruby.sketchup.com/Sketchup/Console.html#clear-instance_method)

#### `hide => Object`

Hides the SketchUp Ruby Console.

**Remarks:** Hides the SketchUp Ruby Console.

**Returns:** `Object` — true.

[Docs](https://ruby.sketchup.com/Sketchup/Console.html#hide-instance_method)

#### `show => Object`

Displays the SketchUp Ruby Console.

**Remarks:** Displays the SketchUp Ruby Console.

**Returns:** `Object` — true.

[Docs](https://ruby.sketchup.com/Sketchup/Console.html#show-instance_method)

#### `visible? => Boolean`

Returns the visibility state of the SketchUp Ruby Console.

**Remarks:** Returns the visibility state of the SketchUp Ruby Console.

**Returns:** `Boolean` — true if visible, false if not.

[Docs](https://ruby.sketchup.com/Sketchup/Console.html#visible?-instance_method)

## ConstructionLine (class)

The ConstructionLine class contains methods for modifying construction lines. Construction lines can be infinite in length, semi-infinite (i.e. infinite in one direction) or finite.

[Vendor docs](https://ruby.sketchup.com/Sketchup/ConstructionLine.html)

### Methods
#### `reverse! => Object`

The reverse! method is used to reverse the direction of the construction line.

**Remarks:** The reverse! method is used to reverse the direction of the construction line.

**Returns:** `Object` — status

[Docs](https://ruby.sketchup.com/Sketchup/ConstructionLine.html#reverse!-instance_method)

### Properties
- `direction` (Object, get/set) — The direction method retrieves a 3D vector in the direction of the construction line.
- `end` (Object, get/set) — The end method retrieves the end point of a construction line in the form of a 3D point.
- `position` (Object, get/set) — The position method is used to retrieve a 3D point used to create a construction line on an infinite construction line.
- `start` (Object, get/set) — The start method is used to retrieve the starting point of a construction line.
- `stipple` (Integer, get/set) — The #stipple method is used to retrieve the stipple pattern used to display the construction line.

## ConstructionPoint (class)

A construction point represents a point in the model that can be used to aid in other modeling operations. For example, you may put a construction point at the center of a circle to make it easier to locate that point for performing other operations.

[Vendor docs](https://ruby.sketchup.com/Sketchup/ConstructionPoint.html)

### Methods
#### `position => Object`

The position method is used to retrieve a Point3d used to create a construction point.

**Remarks:** The position method is used to retrieve a Point3d used to create a construction point.

**Returns:** `Object` — point - the Point3d object used to create the ConstructionPoint

[Docs](https://ruby.sketchup.com/Sketchup/ConstructionPoint.html#position-instance_method)

## Curve (class)

The Curve class is used by SketchUp to unite a series of Edge objects into one conceptual entity. Since SketchUp is a surface modeler, all circles, arcs, and arbitrary curves are really just edges that are bound together in sequence. There is a subclass of Curve called ArcCurve, which is any curve that makes up part of a circle. You can think of ArcCurves as entities that were created with SketchUp's Arc or Circle drawing tools and Curves as entities that were created with the Freehand drawing tool.

[Vendor docs](https://ruby.sketchup.com/Sketchup/Curve.html)

### Methods
#### `count_edges => Object`

The count_edges method is used to retrieve the number of Edge objects that make up the Curve.

**Remarks:** The count_edges method is used to retrieve the number of Edge objects that make up the Curve.

**Returns:** `Object` — num_edges - the number of edges in the curve

[Docs](https://ruby.sketchup.com/Sketchup/Curve.html#count_edges-instance_method)

#### `each_edge => Object`

The each_edge method is used to iterate through all of the Edge objects in the curve.

**Remarks:** The each_edge method is used to iterate through all of the Edge objects in the curve.

**Returns:** `Object` — edge - a variable that will hold each Edge object as they are found.

[Docs](https://ruby.sketchup.com/Sketchup/Curve.html#each_edge-instance_method)

#### `edges => Object`

The edges method is used to retrieve an array of Edge objects that make up the Curve.

**Remarks:** The edges method is used to retrieve an array of Edge objects that make up the Curve.

**Returns:** `Object` — edges - an array of Edge objects if successful

[Docs](https://ruby.sketchup.com/Sketchup/Curve.html#edges-instance_method)

#### `first_edge => Object`

The first_edge method is used to retrieve the first edge of the curve.

**Remarks:** The first_edge method is used to retrieve the first edge of the curve.

**Returns:** `Object` — edge - the first Edge object in the curve if successful

[Docs](https://ruby.sketchup.com/Sketchup/Curve.html#first_edge-instance_method)

#### `is_polygon? => Boolean`

Returns True if this edge was originally created by the polygon tool, otherwise false.

**Remarks:** Returns True if this edge was originally created by the polygon tool, otherwise false.

**Returns:** `Boolean` — True if this edge was originally created by the polygon tool, otherwise false.

[Docs](https://ruby.sketchup.com/Sketchup/Curve.html#is_polygon?-instance_method)

#### `last_edge => Object`

The last_edge method is used to retrieve the last edge of the curve.

**Remarks:** The last_edge method is used to retrieve the last edge of the curve.

**Returns:** `Object` — edge - the last Edge object in the curve if successful

[Docs](https://ruby.sketchup.com/Sketchup/Curve.html#last_edge-instance_method)

#### `length => Length`

The length method retrieves the length of the curve.

**Remarks:** The length method retrieves the length of the curve.

**Returns:** `Length` — 

[Docs](https://ruby.sketchup.com/Sketchup/Curve.html#length-instance_method)

#### `move_vertices(point_array) => Boolean`

The #move_vertices method moves the vertices in the curve to points.

**Remarks:** The #move_vertices method moves the vertices in the curve to points.

**Parameters:**
- `point_array` (Array<Geom::Point3d>)

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Sketchup/Curve.html#move_vertices-instance_method)

#### `vertices => Object`

The vertices method retrieves a collection of all vertices in a curve.

**Remarks:** The vertices method retrieves a collection of all vertices in a curve.

**Returns:** `Object` — vertices - a collection of the vertices

[Docs](https://ruby.sketchup.com/Sketchup/Curve.html#vertices-instance_method)

## DefinitionList (class)

A DefinitionList object holds a list of all of the ComponentDefinition objects in a model. This class contains methods for adding and retrieving definitions from the list.

[Vendor docs](https://ruby.sketchup.com/Sketchup/DefinitionList.html)

### Methods
#### `[](index) => Sketchup::ComponentDefinition? | [](name) => Sketchup::ComponentDefinition? | [](guid) => Sketchup::ComponentDefinition?`

The [] method is used to retrieve a component definition from the list.

**Remarks:** The [] method is used to retrieve a component definition from the list. You can give an integer index in the range 0 to length, a string which represents the GUID (a unique internal identifier), or a string that is the name of the definition.

**Parameters:**
- `index` (Integer) — The index for a specific component definition.

**Returns:** `Sketchup::ComponentDefinition, nil` — 

[Docs](https://ruby.sketchup.com/Sketchup/DefinitionList.html#[]-instance_method)

#### `[](index) => Sketchup::ComponentDefinition? | [](name) => Sketchup::ComponentDefinition? | [](guid) => Sketchup::ComponentDefinition?`

The [] method is used to retrieve a component definition from the list.

**Remarks:** The [] method is used to retrieve a component definition from the list. You can give an integer index in the range 0 to length, a string which represents the GUID (a unique internal identifier), or a string that is the name of the definition.

**Parameters:**
- `index` (Integer) — The index for a specific component definition.

**Returns:** `Sketchup::ComponentDefinition, nil` — 

[Docs](https://ruby.sketchup.com/Sketchup/DefinitionList.html#at-instance_method)

#### `add(def_name) => Sketchup::ComponentDefinition`

The add method is used to add a new component definition to the definition list with the given name.

**Remarks:** The add method is used to add a new component definition to the definition list with the given name.

**Parameters:**
- `def_name` (String) — The new component definition to add to the definition list.

**Returns:** `Sketchup::ComponentDefinition` — the ComponentDefinition object that was added (if successful)

[Docs](https://ruby.sketchup.com/Sketchup/DefinitionList.html#add-instance_method)

#### `add_observer(observer) => Boolean`

The add_observer method is used to add an observer to the current object.

**Remarks:** The add_observer method is used to add an observer to the current object.

**Parameters:**
- `observer` (Object) — An observer.

**Returns:** `Boolean` — true if successful, false if unsuccessful.

[Docs](https://ruby.sketchup.com/Sketchup/DefinitionList.html#add_observer-instance_method)

#### `count => Integer`

Note: Since SketchUp 2014 the count method is inherited from Ruby's Enumerable mix-in module.

**Remarks:** Note: Since SketchUp 2014 the count method is inherited from Ruby's Enumerable mix-in module. Prior to that the #count method is an alias for #length.

**Returns:** `Integer` — 

[Docs](https://ruby.sketchup.com/Sketchup/DefinitionList.html#count-instance_method)

#### `each {|definition| ... } => Sketchup::ComponentDefinition`

Note: Don't remove content from this collection while iterating over it with #each.

**Remarks:** Note: Don't remove content from this collection while iterating over it with #each. This would change the size of the collection and cause elements to be skipped as the indices change. Instead copy the current collection to an array using to_a and then use each on the array, when removing content. The #each method is used to iterate through all of the component definitions in the definition list.

**Returns:** `Sketchup::ComponentDefinition` — 

[Docs](https://ruby.sketchup.com/Sketchup/DefinitionList.html#each-instance_method)

#### `import(path, options = {}) => Sketchup::ComponentDefinition`

The #import method is used to import a (non-SketchUp) 3d model file as a definition.

**Remarks:** The #import method is used to import a (non-SketchUp) 3d model file as a definition. Importers using the C API SketchUpModelImporterInterface interface are supported (those in the Importers/ directory). See the Importer Options file for information on creating a valid hash for the various importers. For SketchUp models, instead use #load.

**Parameters:**
- `path` (String)
- `options` (Hash)

**Returns:** `Sketchup::ComponentDefinition` — 

[Docs](https://ruby.sketchup.com/Sketchup/DefinitionList.html#import-instance_method)

#### `length => Integer`

The #length method is used to retrieve number of component definitions in the list.

**Remarks:** The #length method is used to retrieve number of component definitions in the list.

**Returns:** `Integer` — 

[Docs](https://ruby.sketchup.com/Sketchup/DefinitionList.html#length-instance_method)

#### `load(path) => Sketchup::ComponentDefinition | load(path, allow_newer: false) => Sketchup::ComponentDefinition`

The #load method is used to load a component from a file.

**Remarks:** The #load method is used to load a component from a file.

**Parameters:**
- `path` (String) — The path where the component definition file is located.

**Returns:** `Sketchup::ComponentDefinition` — the loaded ComponentDefinition

[Docs](https://ruby.sketchup.com/Sketchup/DefinitionList.html#load-instance_method)

#### `load_from_url(url) => Sketchup::ComponentDefinition | load_from_url(url, load_handler) => Sketchup::ComponentDefinition`

The #load_from_url method loads a component from a location specified by string url.

**Remarks:** The #load_from_url method loads a component from a location specified by string url. This method throws an exception if an url string is not given, or an error occurs during retrieval from URL and a load_handler was not given.

**Parameters:**
- `url` (String) — URL to load a .skp file from.

**Returns:** `Sketchup::ComponentDefinition` — 

[Docs](https://ruby.sketchup.com/Sketchup/DefinitionList.html#load_from_url-instance_method)

#### `purge_unused => Sketchup::DefinitionList`

The purge_unused method is used to remove the unused component definitions.

**Remarks:** The purge_unused method is used to remove the unused component definitions.

**Returns:** `Sketchup::DefinitionList` — 

[Docs](https://ruby.sketchup.com/Sketchup/DefinitionList.html#purge_unused-instance_method)

#### `remove(definition) => Boolean`

The #remove method is used to remove a component definition from the definition list with the given component definition.

**Remarks:** The #remove method is used to remove a component definition from the definition list with the given component definition. This will remove all instances of the definition.

**Parameters:**
- `definition` (Sketchup::ComponentDefinition)

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Sketchup/DefinitionList.html#remove-instance_method)

#### `remove_observer(observer) => Boolean`

The remove_observer method is used to remove an observer from the current object.

**Remarks:** The remove_observer method is used to remove an observer from the current object.

**Parameters:**
- `observer` (Object) — An observer.

**Returns:** `Boolean` — true if successful, false if unsuccessful.

[Docs](https://ruby.sketchup.com/Sketchup/DefinitionList.html#remove_observer-instance_method)

#### `size => Integer`

The #size method is an alias for #length.

**Remarks:** The #size method is an alias for #length.

**Returns:** `Integer` — 

[Docs](https://ruby.sketchup.com/Sketchup/DefinitionList.html#size-instance_method)

#### `unique_name(base_name) => String`

The unique_name is used to generate a unique name for a definition based on a base_name string.

**Remarks:** The unique_name is used to generate a unique name for a definition based on a base_name string. For example, a base_name of “Joe” might return “Joe #2”

**Parameters:**
- `base_name` (String)

**Returns:** `String` — the unique name.

[Docs](https://ruby.sketchup.com/Sketchup/DefinitionList.html#unique_name-instance_method)

## DefinitionObserver (class)

This class is abstract. To implement this observer, create a Ruby class of this type, override the desired methods, and add an instance of the observer to the definitions of interests. This observer interface is implemented to react to component definition events.

[Vendor docs](https://ruby.sketchup.com/Sketchup/DefinitionObserver.html)

### Methods
#### `onComponentInstanceAdded(definition, instance) => nil`

The #onComponentInstanceAdded method is called when a new component instance is added to a model.

**Remarks:** The #onComponentInstanceAdded method is called when a new component instance is added to a model.

**Parameters:**
- `definition` (Sketchup::ComponentDefinition) — The definition of the added instance
- `instance` (Sketchup::ComponentInstance) — The added instance

**Returns:** `nil` — 

[Docs](https://ruby.sketchup.com/Sketchup/DefinitionObserver.html#onComponentInstanceAdded-instance_method)

#### `onComponentInstanceRemoved(definition, instance) => nil`

Note: Due to the underlying way that the SketchUp Move Tool is implemented, this method is fired on a Move + Copy operation even though no ComponentInstance is apparently removed.

**Remarks:** Note: Due to the underlying way that the SketchUp Move Tool is implemented, this method is fired on a Move + Copy operation even though no ComponentInstance is apparently removed. The #onComponentInstanceRemoved method is called when a component instance is removed from a model.

**Parameters:**
- `definition` (Sketchup::ComponentDefinition) — The definition of the instance removed
- `instance` (Sketchup::ComponentInstance) — The removed instance

**Returns:** `nil` — 

[Docs](https://ruby.sketchup.com/Sketchup/DefinitionObserver.html#onComponentInstanceRemoved-instance_method)

## DefinitionsObserver (class)

This class is abstract. To implement this observer, create a Ruby class of this type, override the desired methods, and add an instance of the observer to the collection of interest. This observer interface is implemented to react to events on a definitions collection.

[Vendor docs](https://ruby.sketchup.com/Sketchup/DefinitionsObserver.html)

### Methods
#### `onComponentAdded(definitions, definition) => nil`

The #onComponentAdded method is called whenever a definition is added to the definitions collection.

**Remarks:** The #onComponentAdded method is called whenever a definition is added to the definitions collection.

**Parameters:**
- `definitions` (Sketchup::DefinitionList)
- `definition` (Sketchup::ComponentDefinition)

**Returns:** `nil` — 

[Docs](https://ruby.sketchup.com/Sketchup/DefinitionsObserver.html#onComponentAdded-instance_method)

#### `onComponentPropertiesChanged(definitions, definition) => nil`

The #onComponentPropertiesChanged method is called whenever a definition's name or description are changed.

**Remarks:** The #onComponentPropertiesChanged method is called whenever a definition's name or description are changed. This does not fire when “Glue To”, “Cuts Opening”, or “Face Camera” settings are changed.

**Parameters:**
- `definitions` (Sketchup::DefinitionList)
- `definition` (Sketchup::ComponentDefinition)

**Returns:** `nil` — 

[Docs](https://ruby.sketchup.com/Sketchup/DefinitionsObserver.html#onComponentPropertiesChanged-instance_method)

#### `onComponentRemoved(definitions, definition) => nil`

Note: This methods fires twice for each Component/Group erased.

**Remarks:** Note: This methods fires twice for each Component/Group erased. The #onComponentAdded method is called whenever a definition is removed from the definitions collection.

**Parameters:**
- `definitions` (Sketchup::DefinitionList)
- `definition` (Sketchup::ComponentDefinition)

**Returns:** `nil` — 

[Docs](https://ruby.sketchup.com/Sketchup/DefinitionsObserver.html#onComponentRemoved-instance_method)

#### `onComponentTypeChanged(definitions, definition) => nil`

The #onComponentTypeChanged event is fired when a component is converted to a group or vice versa.

**Remarks:** The #onComponentTypeChanged event is fired when a component is converted to a group or vice versa. (In the underlying implementation, Groups are just a special kind of definition that is allowed to only have a single instance.)

**Parameters:**
- `definitions` (Sketchup::DefinitionList)
- `definition` (Sketchup::ComponentDefinition)

**Returns:** `nil` — 

[Docs](https://ruby.sketchup.com/Sketchup/DefinitionsObserver.html#onComponentTypeChanged-instance_method)

## Dimension (class)

The Dimension class provides base functionality for classes DimensionLinear and DimensionRadial. It's not instantiable.

[Vendor docs](https://ruby.sketchup.com/Sketchup/Dimension.html)

### Methods
#### `add_observer(observer) => Object`

Note: If the given observer responds to onTextChanged, it will be added as a Sketchup::DimensionObserver.

**Remarks:** Note: If the given observer responds to onTextChanged, it will be added as a Sketchup::DimensionObserver. If not, the base Entity#add_observer will be called. The add_observer method is used to add a DimensionObserver to the dimension.

**Parameters:**
- `observer` (Object) — A DimensionObserver.

**Returns:** `Object` — true if successful, false if unsuccessful.

[Docs](https://ruby.sketchup.com/Sketchup/Dimension.html#add_observer-instance_method)

#### `has_aligned_text? => Boolean`

The has_aligned_text method is used to determine whether the dimension's text is aligned to the dimension or to the screen.

**Remarks:** The has_aligned_text method is used to determine whether the dimension's text is aligned to the dimension or to the screen.

**Returns:** `Boolean` — status - true if text is aligned to the dimension. False if dimension text is aligned to the screen.

[Docs](https://ruby.sketchup.com/Sketchup/Dimension.html#has_aligned_text?-instance_method)

#### `plane => Object`

The plane method is used to retrieve the plane of the dimension.

**Remarks:** The plane method is used to retrieve the plane of the dimension. Refer to the Geom module for information on how planes are represented.

**Returns:** `Object` — the plane of the dimension

[Docs](https://ruby.sketchup.com/Sketchup/Dimension.html#plane-instance_method)

#### `remove_observer(observer) => Object`

The remove_observer method is used to remove a DimensionObserver from the dimension.

**Remarks:** The remove_observer method is used to remove a DimensionObserver from the dimension. Note that, if the given observer responds to 'onTextChanged', it will be removed as a DimensionObserver. If not, the base Entity.remove_observer will be called.

**Parameters:**
- `observer` (Object) — A DimensionObserver.

**Returns:** `Object` — true if successful, false if unsuccessful.

[Docs](https://ruby.sketchup.com/Sketchup/Dimension.html#remove_observer-instance_method)

### Properties
- `arrow_type` (Object, get/set) — The #arrow_type method retrieves the current arrow type of the dimension.
- `had_aligned_text` (Boolean, set) — The has_aligned_text= method accepts true or false indicating whether the dimension's text is aligned to the dimension or to the screen.
- `text` (Object, get/set) — The text method is used to retrieve the dimension text.
- `ARROW_NONE` (Object, get) — 
- `ARROW_SLASH` (Object, get) — 
- `ARROW_DOT` (Object, get) — 
- `ARROW_CLOSED` (Object, get) — 
- `ARROW_OPEN` (Object, get) — 

## DimensionLinear (class)

The DimensionLinear class represents linear dimensions.

[Vendor docs](https://ruby.sketchup.com/Sketchup/DimensionLinear.html)

### Properties
- `aligned_text_position` (Object, get/set) — The #aligned_text_position method returns the text position for dimensions with aligned text (i.
- `end` (Object, get/set) — The end method returns the point or entity the dimension is referencing at its end.
- `end_attached_to` (Array(Sketchup::InstancePath, Geom::Point3d), nil, get/set) — The #end_attached_to method will return the attached end point via an array containing the InstancePath and Geom::Point3d.
- `offset_vector` (Object, get/set) — The offset_vector method returns the parallel offset vector from the reference line to the dimension line measured from the 'start' reference point.
- `start` (Object, get/set) — The start method returns the point or entity the dimension is referencing at its start.
- `start_attached_to` (Array(Sketchup::InstancePath, Geom::Point3d), nil, get/set) — The #start_attached_to method will return the attached start point via an array containing the InstancePath and Geom::Point3d.
- `text_position` (Object, get/set) — The #text_position method returns the position of the text along the dimension line.
- `TEXT_OUTSIDE_START` (Object, get) — 
- `TEXT_CENTERED` (Object, get) — 
- `TEXT_OUTSIDE_END` (Object, get) — 
- `ALIGNED_TEXT_ABOVE` (Object, get) — 
- `ALIGNED_TEXT_CENTER` (Object, get) — 
- `ALIGNED_TEXT_OUTSIDE` (Object, get) — 

## DimensionObserver (class)

This class is abstract. To implement this observer, create a Ruby class of this type, override the desired methods, and add an instance of the observer to the dimensions of interest. This observer interface is implemented to react to changes in dimension text.

[Vendor docs](https://ruby.sketchup.com/Sketchup/DimensionObserver.html)

### Methods
#### `onTextChanged(dimension) => Object`

The #onTextChanged method is invoked when your entity is erased.

**Remarks:** The #onTextChanged method is invoked when your entity is erased.

**Parameters:**
- `dimension` (Sketchup::Dimension) — The dimension object whose text has been changed.

[Docs](https://ruby.sketchup.com/Sketchup/DimensionObserver.html#onTextChanged-instance_method)

## DimensionRadial (class)

The DimensionRadial class represents radius and diameter dimensions on arcs and circles.

[Vendor docs](https://ruby.sketchup.com/Sketchup/DimensionRadial.html)

### Methods
#### `leader_points => Object`

The leader_points method returns the 3 significant points along the dimension line in world coordinates.

**Remarks:** The leader_points method returns the 3 significant points along the dimension line in world coordinates.

**Returns:** `Object` — Array of 3 Point3d objects. Point 0: leader break point, where the text extension attaches. Point 1: attach point, where leader touches the arc/circle. Point 2: opposite point, where the diameter leader touches the circle on the opposite side.

[Docs](https://ruby.sketchup.com/Sketchup/DimensionRadial.html#leader_points-instance_method)

### Properties
- `arc_curve` (Object, get/set) — The arc_curve method returns the ArcCurve object to which this dimension is attached.
- `leader_break_point` (Object, get/set) — The #leader_break_point method returns the break point on the leader where the dimension text is attached.

## Drawingelement (class)

Drawingelement is a base class for an item in the model that can be displayed. These items include edges, construction points, construction lines, and images. Arc curves and arcs are not included because they are not drawing elements by themselves, but are a composition of edges.

[Vendor docs](https://ruby.sketchup.com/Sketchup/Drawingelement.html)

### Methods
#### `bounds => Geom::BoundingBox`

The #bounds method is used to retrieve the Geom::BoundingBox bounding a Sketchup::Drawingelement.

**Remarks:** The #bounds method is used to retrieve the Geom::BoundingBox bounding a Sketchup::Drawingelement. For a Edge, ComponentInstance and most other Sketchup::Drawingelements, the boundingbox follows the coordinate system the drawing element is placed in. For ComponentDefinition, the box bounds the contents of the component and follows the component's own internal coordinate system.

**Returns:** `Geom::BoundingBox` — 

[Docs](https://ruby.sketchup.com/Sketchup/Drawingelement.html#bounds-instance_method)

#### `casts_shadows? => Boolean`

The casts_shadows? method is used to determine if the Drawingelement is casting shadows.

**Remarks:** The casts_shadows? method is used to determine if the Drawingelement is casting shadows.

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Sketchup/Drawingelement.html#casts_shadows?-instance_method)

#### `erase! => nil`

Note: When erasing multiple elements, it's faster to use Entities#erase_entities and erase in bulk than to iterate individual drawing elements calling #erase!.

**Remarks:** Note: When erasing multiple elements, it's faster to use Entities#erase_entities and erase in bulk than to iterate individual drawing elements calling #erase!. The #erase! method is used to erase an element from the model. Erasing an Edge also erases all of the Face objects that use the Edge.

**Returns:** `nil` — 

[Docs](https://ruby.sketchup.com/Sketchup/Drawingelement.html#erase!-instance_method)

#### `hidden? => Boolean`

The hidden? method is used to determine if the element is hidden.

**Remarks:** The hidden? method is used to determine if the element is hidden. Hidden elements are still in the model, but they are not displayed.

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Sketchup/Drawingelement.html#hidden?-instance_method)

#### `receives_shadows? => Boolean`

The receive_shadows? method is used to determine if the Drawingelement is receiving shadows.

**Remarks:** The receive_shadows? method is used to determine if the Drawingelement is receiving shadows.

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Sketchup/Drawingelement.html#receives_shadows?-instance_method)

#### `visible? => Boolean`

The #visible? method checks if a Drawingelement object is not explicitly hidden (i.

**Remarks:** The #visible? method checks if a Drawingelement object is not explicitly hidden (i.e. its hidden property is false). However, this method's return value alone does not guarantee that the element is visible in the model view. Its tag or parent elements can also be hidden. Some element types can also be hidden by rendering options (Styles).

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Sketchup/Drawingelement.html#visible?-instance_method)

### Properties
- `casts_shadows` (Boolean, set) — The casts_shadows= method is used to set the Drawingelement to cast shadows.
- `hidden` (Boolean, set) — The hidden= method is used to set the hidden status for an element.
- `layer` (Sketchup::Layer, get/set) — The layer method is used to retrieve the Layer object of the drawing element.
- `material` (Sketchup::Material, get/set) — The material method is used to retrieve the material for the drawing element.
- `receives_shadows` (Boolean, set) — The receive_shadows= method is used to set the Drawingelement to receive shadows.
- `visible` (Boolean, set) — The #visible= method is used to set the visible status for an element.

## Edge (class)

The Edge class contains methods modifying and extracting information for edges.

[Vendor docs](https://ruby.sketchup.com/Sketchup/Edge.html)

### Methods
#### `all_connected => Array<Sketchup::Entity>`

The all_connected method retrieves all of the entities connected to an edge, including the edge itself.

**Remarks:** The all_connected method retrieves all of the entities connected to an edge, including the edge itself.

**Returns:** `Array<Sketchup::Entity>` — the edge and entities connected to that edge

[Docs](https://ruby.sketchup.com/Sketchup/Edge.html#all_connected-instance_method)

#### `common_face(edge2) => Sketchup::Face?`

The common_face method is used to identify a face that is common to two edges.

**Remarks:** The common_face method is used to identify a face that is common to two edges.

**Parameters:**
- `edge2` (Sketchup::Edge) — The face whose edge you are checking for commonality.

**Returns:** `Sketchup::Face, nil` — the Face object that is common to the two edges if successful

[Docs](https://ruby.sketchup.com/Sketchup/Edge.html#common_face-instance_method)

#### `curve => Sketchup::Curve?`

The curve method is used to get the Curve object that this edge belongs to, if any.

**Remarks:** The curve method is used to get the Curve object that this edge belongs to, if any. Note that if the edge is part of an arc instead of a random curve, then this method will return an ArcCurve object.

**Returns:** `Sketchup::Curve, nil` — returns a Curve object if it is a curve, nil if it is not a curve

[Docs](https://ruby.sketchup.com/Sketchup/Edge.html#curve-instance_method)

#### `end => Sketchup::Vertex`

The end method is used to retrieve the Vertex object at the end of the edge.

**Remarks:** The end method is used to retrieve the Vertex object at the end of the edge.

**Returns:** `Sketchup::Vertex` — a Vertex object if successful

[Docs](https://ruby.sketchup.com/Sketchup/Edge.html#end-instance_method)

#### `explode_curve => Sketchup::Edge`

The explode_curve method is used to explode the curve that the given edge is a part of.

**Remarks:** The explode_curve method is used to explode the curve that the given edge is a part of.

**Returns:** `Sketchup::Edge` — an exploded edge object if successful

[Docs](https://ruby.sketchup.com/Sketchup/Edge.html#explode_curve-instance_method)

#### `faces => Array<Sketchup::Face>`

The #faces method is used to retrieve all of the faces common to the edge.

**Remarks:** The #faces method is used to retrieve all of the faces common to the edge.

**Returns:** `Array<Sketchup::Face>` — 

[Docs](https://ruby.sketchup.com/Sketchup/Edge.html#faces-instance_method)

#### `find_faces => Integer`

The find_faces method is used to create all of the Faces that can be created with this edge.

**Remarks:** The find_faces method is used to create all of the Faces that can be created with this edge. For example, if you use the API to draw three edges that form a triangle, the face between them will not show up because you've only drawn the edges, but if you call find_faces on one of the edges, the triangle will be filled in.

**Returns:** `Integer` — the number of faces found

[Docs](https://ruby.sketchup.com/Sketchup/Edge.html#find_faces-instance_method)

#### `length => Length | length(transform) => Length`

The #length method is used to retrieve the length of an edge in current units.

**Remarks:** The #length method is used to retrieve the length of an edge in current units. You can pass in an optional Geom::Transformation (or an array that can represent a transformation), to correct for a parent group's transformation. For example, if an edge is inside of a group that is scaled to 200%, the length method will return the unscaled length of the edge. So by passing a 200% transformation object to this method, you can account for that to get the “visual” length of the edge.

**Parameters:**
- `transform` (Geom::Transformation) — A Transformation object or array that can be interpreted as a Transformation object.

**Returns:** `Length` — the length of the edge

[Docs](https://ruby.sketchup.com/Sketchup/Edge.html#length-instance_method)

#### `line => Array(Geom::Point3d, Geom::Vector3d)`

The line method is used to retrieve the line defined by the edge.

**Remarks:** The line method is used to retrieve the line defined by the edge. Lines in SketchUp aren't visible entities but geometric constructs represented by an Array with a Point3d and a Vector3d. See the Geom module and the Array class for more information on lines.

**Returns:** `Array(Geom::Point3d, Geom::Vector3d)` — an array with a Point3d object and a Vector3d object.

[Docs](https://ruby.sketchup.com/Sketchup/Edge.html#line-instance_method)

#### `other_vertex(vertex1) => Sketchup::Vertex`

The other_vertex method is used to find the opposite vertex given one vertex of the edge.

**Remarks:** The other_vertex method is used to find the opposite vertex given one vertex of the edge.

**Parameters:**
- `vertex1` (Sketchup::Vertex) — One of the Vertex objects associated with the edge.

**Returns:** `Sketchup::Vertex` — the other Vertex object associated with the edge

[Docs](https://ruby.sketchup.com/Sketchup/Edge.html#other_vertex-instance_method)

#### `reversed_in?(face) => Boolean`

The #reversed_in? method is used to determine if the edge is reversed in a face's bounding loop.

**Remarks:** The #reversed_in? method is used to determine if the edge is reversed in a face's bounding loop.

**Parameters:**
- `face` (Sketchup::Face) — The face that is bounded by the edge.

**Returns:** `Boolean` — true if the edge is reversed, false if it is not reversed. nil if the edge and face is not connected.

[Docs](https://ruby.sketchup.com/Sketchup/Edge.html#reversed_in?-instance_method)

#### `smooth? => Boolean`

The #smooth? method is used to retrieve the current smooth setting for an edge.

**Remarks:** The #smooth? method is used to retrieve the current smooth setting for an edge. A smooth edge will cause the shading between connected faces to blend to a smooth transition. The edge will still be visible.

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Sketchup/Edge.html#smooth?-instance_method)

#### `soft? => Boolean`

The #soft? method is used to retrieve the current soft setting for an edge.

**Remarks:** The #soft? method is used to retrieve the current soft setting for an edge. A soft edge will cause the connected faces to be treated as a surface. This means that if you have Hidden Geometry off and select one face it will also select all faces connected with soft edges. A soft edge will also appear hidden.

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Sketchup/Edge.html#soft?-instance_method)

#### `split(position) => Sketchup::Edge`

The #split method is used to split an edge into two or more distinct edges.

**Remarks:** The #split method is used to split an edge into two or more distinct edges. If a Point3d is given, it must be a point that is on the Edge. If a Float is given, it is a number between 0 and 1 that gives the relative position along the edge at which to split it. For example, edge.split(0.5) will split the Edge at its midpoint. This split position is measured from the Edge.start. Returns the new Edge that was created as a result of splitting this one.

**Parameters:**
- `position` (Geom::Point3d) — A Point3d object whose location is along the edge, or a Float between 0.0 and 1.0 defining how far along the edge to split.

**Returns:** `Sketchup::Edge` — the new Edge object that was split off the old one if successful

[Docs](https://ruby.sketchup.com/Sketchup/Edge.html#split-instance_method)

#### `start => Sketchup::Vertex`

The start method is used to retrieve the Vertex object at the start of the edge.

**Remarks:** The start method is used to retrieve the Vertex object at the start of the edge.

**Returns:** `Sketchup::Vertex` — a Vertex object if successful

[Docs](https://ruby.sketchup.com/Sketchup/Edge.html#start-instance_method)

#### `used_by?(element) => Boolean`

The used_by? method is used to see if an edge is used by a given Face or Vertex.

**Remarks:** The used_by? method is used to see if an edge is used by a given Face or Vertex.

**Parameters:**
- `element` (Sketchup::Vertex, Sketchup::Face) — A Vertex or Face object.

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Sketchup/Edge.html#used_by?-instance_method)

#### `vertices => Array<Sketchup::Vertex>`

The vertices method is used to retrieve the vertices on the edge.

**Remarks:** The vertices method is used to retrieve the vertices on the edge.

**Returns:** `Array<Sketchup::Vertex>` — an array of Vertex objects

[Docs](https://ruby.sketchup.com/Sketchup/Edge.html#vertices-instance_method)

### Properties
- `smooth` (Boolean, set) — Note: The soft and smooth properties are normally set in pairs.
- `soft` (Boolean, set) — Note: The soft and smooth properties are normally set in pairs.

## EdgeUse (class)

The EdgeUse class defines how an Edge is used in the definition of a Face.

[Vendor docs](https://ruby.sketchup.com/Sketchup/EdgeUse.html)

### Methods
#### `edge => Sketchup::Edge`

The edge method is used to retrieve the edge for the edge use.

**Remarks:** The edge method is used to retrieve the edge for the edge use.

**Returns:** `Sketchup::Edge` — an Edge object used by this edge use

[Docs](https://ruby.sketchup.com/Sketchup/EdgeUse.html#edge-instance_method)

#### `end_vertex_normal => Geom::Vector3d`

The end_vertex_normal method is used to retrieve the vertex normal for the end point of this edgeuse.

**Remarks:** The end_vertex_normal method is used to retrieve the vertex normal for the end point of this edgeuse.

**Returns:** `Geom::Vector3d` — a vector3d object if successful.

[Docs](https://ruby.sketchup.com/Sketchup/EdgeUse.html#end_vertex_normal-instance_method)

#### `face => Sketchup::Face`

The face method is used to retrieve the face used by this edge use.

**Remarks:** The face method is used to retrieve the face used by this edge use.

**Returns:** `Sketchup::Face` — a Face object used by this edge use

[Docs](https://ruby.sketchup.com/Sketchup/EdgeUse.html#face-instance_method)

#### `loop => Sketchup::Loop`

The loop method is used to retrieve the loop for this edge use.

**Remarks:** The loop method is used to retrieve the loop for this edge use.

**Returns:** `Sketchup::Loop` — a Loop object that contains this edge use.

[Docs](https://ruby.sketchup.com/Sketchup/EdgeUse.html#loop-instance_method)

#### `next => Sketchup::EdgeUse`

The next method is used to retrieve the next edge use in a loop.

**Remarks:** The next method is used to retrieve the next edge use in a loop.

**Returns:** `Sketchup::EdgeUse` — the next EdgeUse object in a loop

[Docs](https://ruby.sketchup.com/Sketchup/EdgeUse.html#next-instance_method)

#### `partners => Array<Sketchup::EdgeUse>`

The partners method is used to retrieve all of the partner edge uses that uses the same edge.

**Remarks:** The partners method is used to retrieve all of the partner edge uses that uses the same edge.

**Returns:** `Array<Sketchup::EdgeUse>` — an array of partner Edge Use objects.

[Docs](https://ruby.sketchup.com/Sketchup/EdgeUse.html#partners-instance_method)

#### `previous => Sketchup::EdgeUse`

The previous method is used to retrieve the previous edge use in a loop.

**Remarks:** The previous method is used to retrieve the previous edge use in a loop.

**Returns:** `Sketchup::EdgeUse` — the previous Edge Use object in the loop

[Docs](https://ruby.sketchup.com/Sketchup/EdgeUse.html#previous-instance_method)

#### `reversed? => Boolean`

The reversed? method is used to determine if the edge direction is opposite of the edge use direction.

**Remarks:** The reversed? method is used to determine if the edge direction is opposite of the edge use direction. The edge use direction is the same as the loop it belongs to.

**Returns:** `Boolean` — true if reversed, false if not reversed.

[Docs](https://ruby.sketchup.com/Sketchup/EdgeUse.html#reversed?-instance_method)

#### `start_vertex_normal => Geom::Vector3d`

The start_vertex_normal method is used to retrieve the vertex normal for the start point of this edgeuse.

**Remarks:** The start_vertex_normal method is used to retrieve the vertex normal for the start point of this edgeuse.

**Returns:** `Geom::Vector3d` — a vector3d object if successful.

[Docs](https://ruby.sketchup.com/Sketchup/EdgeUse.html#start_vertex_normal-instance_method)

## Entities (class)

The Entities class is a collection of Entity objects, either in a ComponentDefinition or directly in the Model. A Entities object corresponds to a drawing context in the GUI.

[Vendor docs](https://ruby.sketchup.com/Sketchup/Entities.html)

### Methods
#### `[](entity_index) => Sketchup::Entity?`

The #[] method is used to retrieve an entity by its index in an array of entities.

**Remarks:** The #[] method is used to retrieve an entity by its index in an array of entities. The index is a number between 0 and entities.length - 1. In general, it is preferable to use the #each method to iterate though all of the entities in the collection as it will be much more efficient.

**Parameters:**
- `entity_index` (Integer) — The index for a specific entity.

**Returns:** `Sketchup::Entity, nil` — an Sketchup::Entity object if successful, nil if not found

[Docs](https://ruby.sketchup.com/Sketchup/Entities.html#[]-instance_method)

#### `add_3d_text(string, alignment, font, is_bold = false, is_italic = false, letter_height = 1.0, tolerance = 0.0, z = 0.0, is_filled = true, extrusion = 0.0) => Boolean`

The #add_3d_text method is used to create 3D text.

**Remarks:** The #add_3d_text method is used to create 3D text. It will be added as edges and faces drawn at the origin.

**Parameters:**
- `string` (String) — The text to create.
- `alignment` (Integer) — Number that defines the alignment. There are constants called TextAlignLeft, TextAlignRight, and TextAlignCenter that can be passed.
- `font` (String) — font name.
- `is_bold` (Boolean) — true for bold.
- `is_italic` (Boolean) — true for italic.
- `letter_height` (Length) — Height of the text
- `tolerance` (Numeric) — Tolerance of the curve creation. Defaults to 0.0, which creates the highest possible curve quality.
- `z` (Length) — z position of the text
- `is_filled` (Boolean) — true for filled, which will put a face between the edges of the letters.
- `extrusion` (Length) — Extrusion depth

**Returns:** `Boolean` — true if successful

[Docs](https://ruby.sketchup.com/Sketchup/Entities.html#add_3d_text-instance_method)

#### `add_arc(center, xaxis, normal, radius, start_angle, end_angle) => Array<Sketchup::Edge> | add_arc(center, xaxis, normal, radius, start_angle, end_angle, num_segments) => Array<Sketchup::Edge>`

The add_arc method is used to create an arc curve segment.

**Remarks:** The add_arc method is used to create an arc curve segment.

**Parameters:**
- `center` (Geom::Point3d) — A Point3d object representing the center .
- `xaxis` (Geom::Vector3d) — A Vector3d object representing xaxis for the arc.
- `normal` (Geom::Vector3d) — A Vector3d object representing normal for the arc.
- `radius` (Numeric) — The radius of the arc.
- `start_angle` (Numeric) — Start angle for the arc, in radians.
- `end_angle` (Numeric) — End angle for the arc, in radians.

**Returns:** `Array<Sketchup::Edge>` — an array of Edge objects that define the arc.

[Docs](https://ruby.sketchup.com/Sketchup/Entities.html#add_arc-instance_method)

#### `add_circle(center, normal, radius, numsegs = 24) => Array<Sketchup::Edge>`

The add_circle method is used to create a circle.

**Remarks:** The add_circle method is used to create a circle.

**Parameters:**
- `center` (Geom::Point3d) — A Point3d object representing the center.
- `normal` (Geom::Vector3d) — A Vector3d object representing normal for the arc.
- `radius` (Numeric) — The radius of the arc.
- `numsegs` (Integer) — The number of segments.

**Returns:** `Array<Sketchup::Edge>` — an Array object containing edges if successful

[Docs](https://ruby.sketchup.com/Sketchup/Entities.html#add_circle-instance_method)

#### `add_cline(start_point, end_point, stipple = '-') => Sketchup::ConstructionLine | add_cline(point, vector, stipple = '-') => Sketchup::ConstructionLine`

The #add_cline method is used to create a construction line.

**Remarks:** The #add_cline method is used to create a construction line. This can be finite or infinite.

**Parameters:**
- `start_point` (Geom::Point3d)
- `end_point` (Geom::Point3d)
- `stipple` (String) — See ConstructionLine#stipple= for acceptable patterns.

**Returns:** `Sketchup::ConstructionLine` — 

[Docs](https://ruby.sketchup.com/Sketchup/Entities.html#add_cline-instance_method)

#### `add_cpoint(point) => Sketchup::ConstructionPoint`

The add_cpoint method is used to create a construction point.

**Remarks:** The add_cpoint method is used to create a construction point.

**Parameters:**
- `point` (Geom::Point3d) — A Point3d object.

**Returns:** `Sketchup::ConstructionPoint` — a ConstructionPoint object if successful

[Docs](https://ruby.sketchup.com/Sketchup/Entities.html#add_cpoint-instance_method)

#### `add_curve(points) => Array<Sketchup::Edge> | add_curve(*points) => Array<Sketchup::Edge>`

The add_curve method is used to create a curve from a collection of edges.

**Remarks:** The add_curve method is used to create a curve from a collection of edges. The arguments are either Points or an Array of Points. At least 2 points are required.

**Parameters:**
- `points` (Array<Geom::Point3d>)

**Returns:** `Array<Sketchup::Edge>` — 

[Docs](https://ruby.sketchup.com/Sketchup/Entities.html#add_curve-instance_method)

#### `add_dimension_linear(start_pt_or_entity, end_pt_or_entity, offset_vector) => Sketchup::DimensionLinear | add_dimension_linear(start_instance_path, end_instance_path, offset_vector) => Sketchup::DimensionLinear | add_dimension_linear(start_array, end_array, offset_vector) => Sketchup::DimensionLinear`

The #add_dimension_linear method adds a linear dimension to the entities.

**Remarks:** The #add_dimension_linear method adds a linear dimension to the entities.

**Parameters:**
- `start_pt_or_entity` (Geom::Point3d, Sketchup::Entity) — the reference point and/or entity at the start of the dimension. This parameter can take several forms:
- `end_pt_or_entity` (Geom::Point3d, Sketchup::Entity) — the reference point and/or entity at the end of the dimension. This parameter takes the exact same forms as 'start_pt_or_entity'.
- `offset_vector` (Geom::Vector3d) — the parallel offset vector from the reference line to the dimension line measured from the 'start' reference point.

**Returns:** `Sketchup::DimensionLinear` — the created dimension

[Docs](https://ruby.sketchup.com/Sketchup/Entities.html#add_dimension_linear-instance_method)

#### `add_dimension_radial(arc_curve, leader_break_pt) => Sketchup::DimensionRadial`

The add_dimension_radial method adds a radial dimension (i.

**Remarks:** The add_dimension_radial method adds a radial dimension (i.e arc/circle radius/diameter dimension) to the entities.

**Parameters:**
- `arc_curve` (Sketchup::ArcCurve) — an ArcCurve object to which the dimension is to be attached.
- `leader_break_pt` (Geom::Point3d) — a Point3d for the break point on the leader where the dimension text is attached.

**Returns:** `Sketchup::DimensionRadial` — the created dimension

[Docs](https://ruby.sketchup.com/Sketchup/Entities.html#add_dimension_radial-instance_method)

#### `add_edges(points) => Array<Sketchup::Edge> | add_edges(*points) => Array<Sketchup::Edge>`

Note: If the points form a closed loop, the first and last vertex will not merge.

**Remarks:** Note: If the points form a closed loop, the first and last vertex will not merge. If you intend to create a face from the edges, use #add_face directly. The #add_edges method is used to add a set of connected edges to the Sketchup::Entities collection.

**Parameters:**
- `points` (Array<Geom::Point3d>) — At least two points required.

**Returns:** `Array<Sketchup::Edge>` — 

[Docs](https://ruby.sketchup.com/Sketchup/Entities.html#add_edges-instance_method)

#### `add_face(entities) => Sketchup::Face? | add_face(*entities) => Sketchup::Face?`

Note: A special case exists for any face created on the ground plane, in which case the vertex order is ignored and the face is always facing down.

**Remarks:** Note: A special case exists for any face created on the ground plane, in which case the vertex order is ignored and the face is always facing down. The add_face method is used to create a face. You can call this method a number of ways: For the last form that takes a Curve, the curve must be closed - like a circle.

**Parameters:**
- `entities` (Array<Sketchup::Edge>, Array<Geom::Point3d>, Sketchup::Curve)

**Returns:** `Sketchup::Face, nil` — 

[Docs](https://ruby.sketchup.com/Sketchup/Entities.html#add_face-instance_method)

#### `add_faces_from_mesh(polygon_mesh, smooth_flags = Geom::PolygonMesh::AUTO_SOFTEN|Geom::PolygonMesh::SMOOTH_SOFT_EDGES, f_material = nil, b_material = nil) => Integer`

The #add_faces_from_mesh method is used to add Face entities to the collection of entities from a Geom::PolygonMesh.

**Remarks:** The #add_faces_from_mesh method is used to add Face entities to the collection of entities from a Geom::PolygonMesh. The smooth_flags parameter can contain any of the following values if passed. The constants were added in SketchUp 2014. For previous versions, numeric values have to be specified instead of the Ruby constants. 0: Geom::PolygonMesh::NO_SMOOTH_OR_HIDE 1: Geom::PolygonMesh::HIDE_BASED_ON_INDEX (Negative point index will hide the edge.) 2: Geom::PolygonMesh::SOFTEN_BASED_ON_INDEX (Negative point index will soften the edge.) 4: Geom::PolygonMesh::AUTO_SOFTEN (Interior edges are softened.) 8: Geom::PolygonMesh::SMOOTH_SOFT_EDGES (All soft edges will also be smooth.) The 3rd and 4th parameters will accept a Material object or a string name of a material currently in the model.

**Parameters:**
- `polygon_mesh` (Geom::PolygonMesh)
- `smooth_flags` (Integer) — flags for softening and smoothing of edges.
- `f_material` (Sketchup::Material, String) — material to paint front faces with.
- `b_material` (Sketchup::Material, String) — material to paint back faces with.

**Returns:** `Integer` — Number of faces created

[Docs](https://ruby.sketchup.com/Sketchup/Entities.html#add_faces_from_mesh-instance_method)

#### `add_group(entities) => Sketchup::Group | add_group(*entities) => Sketchup::Group`

Note: Calling add_group with entities in its parameters has been known to crash SketchUp before version 8.

**Remarks:** Note: Calling add_group with entities in its parameters has been known to crash SketchUp before version 8.0. It is preferable to create an empty group and then add things to its Entities collection. The #add_group method is used to create a new group.

**Parameters:**
- `entities` (Array<Sketchup::Entity>) — an entities collection to add to the group.

**Returns:** `Sketchup::Group` — 

[Docs](https://ruby.sketchup.com/Sketchup/Entities.html#add_group-instance_method)

#### `add_image(path, point, width, height = 0.0) => Sketchup::Image?`

The add_image method is used to add an image to the collection of entities.

**Remarks:** The add_image method is used to add an image to the collection of entities. The width and height are measured in model units (i.e. inches). If the height is not given, then it is computed from the width to preserve the aspect ratio of the image.

**Parameters:**
- `path` (String) — A path for the image file.
- `point` (Geom::Point3d) — A Point3d object representing the insertion point of the image.
- `width` (Numeric) — Width for the image.
- `height` (Numeric) — height for the image if you want to control width and height independently. Leave as default 0.0 when you want it to be relative to the aspect ratio.

**Returns:** `Sketchup::Image, nil` — an Image object if successful.

[Docs](https://ruby.sketchup.com/Sketchup/Entities.html#add_image-instance_method)

#### `add_instance(definition, transform) => Sketchup::ComponentInstance`

The #add_instance method adds a group or component instance to the collection of entities using an existent definition.

**Remarks:** The #add_instance method adds a group or component instance to the collection of entities using an existent definition.

**Parameters:**
- `definition` (Sketchup::ComponentDefinition) — A ComponentDefinition object.
- `transform` (Geom::Transformation) — A Transformation object.

**Returns:** `Sketchup::ComponentInstance` — a ComponentInstance object if successful

[Docs](https://ruby.sketchup.com/Sketchup/Entities.html#add_instance-instance_method)

#### `add_line(point1, point2) => Sketchup::Edge`

The add_line method is used to add an edge to the collection of entities.

**Remarks:** The add_line method is used to add an edge to the collection of entities. This is not to be confused with the concept of a “line” from a geometric sense, which is an invisible object represented by an Array of a point and a vector. (See the Array class for more information on geometric lines in SketchUp.) This method is the same as add_edges method, but returns a single edge.

**Parameters:**
- `point1` (Geom::Point3d) — Point3d object representing the edge's starting point.
- `point2` (Geom::Point3d) — Point3d object representing the edge's ending point.

**Returns:** `Sketchup::Edge` — a Edge object if successful

[Docs](https://ruby.sketchup.com/Sketchup/Entities.html#add_line-instance_method)

#### `add_ngon(center, normal, radius, numsides = 24) => Array<Sketchup::Edge>`

The add_ngon method is used to create a multi-sided polygon.

**Remarks:** The add_ngon method is used to create a multi-sided polygon.

**Parameters:**
- `center` (Geom::Point3d) — A Point3d object representing the center of the polygon.
- `normal` (Geom::Vector3d) — A Vector3d object.
- `radius` (Numeric) — A radius.
- `numsides` (Integer) — The number of sides for the polygon.

**Returns:** `Array<Sketchup::Edge>` — an array of Edges that make up the polygon if successful

[Docs](https://ruby.sketchup.com/Sketchup/Entities.html#add_ngon-instance_method)

#### `add_observer(observer) => Boolean`

The add_observer method is used to add an observer to the current object.

**Remarks:** The add_observer method is used to add an observer to the current object.

**Parameters:**
- `observer` (Object) — An observer.

**Returns:** `Boolean` — true if successful, false if unsuccessful.

[Docs](https://ruby.sketchup.com/Sketchup/Entities.html#add_observer-instance_method)

#### `add_section_plane(point, vector) => Sketchup::SectionPlane? | add_section_plane(plane) => Sketchup::SectionPlane? | add_section_plane(plane) => Sketchup::SectionPlane?`

Adds a section plane object to the entities.

**Remarks:** Adds a section plane object to the entities. Refer to the Geom module for information on how planes are represented.

**Parameters:**
- `point` (Geom::Point3d)
- `vector` (Geom::Vector3d)

**Returns:** `Sketchup::SectionPlane, nil` — 

[Docs](https://ruby.sketchup.com/Sketchup/Entities.html#add_section_plane-instance_method)

#### `add_snap(position, direction) => Sketchup::Snap | add_snap(position, direction, up) => Sketchup::Snap`

The #add_snap method is used to create a new Snap.

**Remarks:** The #add_snap method is used to create a new Snap.

**Parameters:**
- `position` (Geom::Point3d)
- `direction` (Geom::Vector3d)

**Returns:** `Sketchup::Snap` — 

[Docs](https://ruby.sketchup.com/Sketchup/Entities.html#add_snap-instance_method)

#### `add_text(text, point, vector) => Sketchup::Text | add_text(text, instance_path_and_pt, vector) => Sketchup::Text | add_text(text, instance_array_and_pt, vector) => Sketchup::Text`

The #add_text method adds a note or label text entity to the entities.

**Remarks:** The #add_text method adds a note or label text entity to the entities.

**Parameters:**
- `text` (String) — The text string to add.
- `point` (Geom::Point3d, Sketchup::Vertex, Sketchup::InputPoint) — A Point3d object representing the insertion point.
- `vector` (Geom::Vector3d) — The Vector representing an arrow leader.

**Returns:** `Sketchup::Text` — a Text object if successful

[Docs](https://ruby.sketchup.com/Sketchup/Entities.html#add_text-instance_method)

#### `at(entity_index) => Sketchup::Entity?`

The #at method is an alias for #[].

**Remarks:** The #at method is an alias for #[].

**Parameters:**
- `entity_index` (Integer) — The index for a specific entity.

**Returns:** `Sketchup::Entity, nil` — 

[Docs](https://ruby.sketchup.com/Sketchup/Entities.html#at-instance_method)

#### `build {|builder| ... } => nil`

Note: While using #build it is important to not add or remove vertices by other means than the builder.

**Remarks:** Note: While using #build it is important to not add or remove vertices by other means than the builder. Also don't modify the position of the vertices in the Sketchup::Entities collection. Doing so can break the vertex-cache that de-duplicates the vertices. Creates an Sketchup::EntitiesBuilder that can be used to generate bulk geometry with performance in mind. This is particularly useful for importers where the geometry is already well defined and one wants to recreate it without further processing. The call to #build starts an implicit operation, even if no other model changes are made within the block. This is not the same as Model#start_operation, so it's still recommended to wrap all model changes, including #build with Model#start_operation and Model#commit_operation. Refer to the documentation of Sketchup::EntitiesBuilder for more details.

**Returns:** `nil` — 

[Docs](https://ruby.sketchup.com/Sketchup/Entities.html#build-instance_method)

#### `clear! => Boolean`

The clear! method is used to remove all entities from the collection of entities.

**Remarks:** The clear! method is used to remove all entities from the collection of entities.

**Returns:** `Boolean` — true if successful, false if unsuccessful

[Docs](https://ruby.sketchup.com/Sketchup/Entities.html#clear!-instance_method)

#### `count => Integer`

Note: Since SketchUp 2014 the count method is inherited from Ruby's Enumerable mix-in module.

**Remarks:** Note: Since SketchUp 2014 the count method is inherited from Ruby's Enumerable mix-in module. Prior to that the #count method is an alias for #length.

**Returns:** `Integer` — 

[Docs](https://ruby.sketchup.com/Sketchup/Entities.html#count-instance_method)

#### `each {|entity| ... } => nil`

Note: Don't remove content from this collection while iterating over it with #each.

**Remarks:** Note: Don't remove content from this collection while iterating over it with #each. This would change the size of the collection and cause elements to be skipped as the indices change. Instead copy the current collection to an array using to_a and then use each on the array, when removing content. The #each method is used to iterate through the entities in the collection of entities.

**Returns:** `nil` — 

[Docs](https://ruby.sketchup.com/Sketchup/Entities.html#each-instance_method)

#### `erase_entities(entities) => nil | erase_entities(*entities) => nil`

Note: It's faster to use this method and erase in bulk than to iterate individual drawing elements calling Drawingelement#erase!.

**Remarks:** Note: It's faster to use this method and erase in bulk than to iterate individual drawing elements calling Drawingelement#erase!. The #erase_entities method is used to erase one or more entities from the model.

**Parameters:**
- `entities` (Array<Sketchup::Entity>)

**Returns:** `nil` — 

[Docs](https://ruby.sketchup.com/Sketchup/Entities.html#erase_entities-instance_method)

#### `fill_from_mesh(polygon_mesh, weld_vertices = true, smooth_flags = Geom::PolygonMesh::AUTO_SOFTEN|Geom::PolygonMesh::SMOOTH_SOFT_EDGES, f_material = nil, b_material = nil) => Boolean`

The #fill_from_mesh method is used to add faces and edges to the collection of entities from a Geom::PolygonMesh.

**Remarks:** The #fill_from_mesh method is used to add faces and edges to the collection of entities from a Geom::PolygonMesh. It requires that the entities collection to be filled is empty. It has higher performance than #add_faces_from_mesh, but does less error checking as it builds the geometry. The smooth_flags parameter can contain any of the following values if passed. The constants were added in SketchUp 2014. For previous versions, numeric values have to be specified instead of the Ruby constants: 0: Geom::PolygonMesh::NO_SMOOTH_OR_HIDE 1: Geom::PolygonMesh::HIDE_BASED_ON_INDEX (Negative point index will hide the edge.) 2: Geom::PolygonMesh::SOFTEN_BASED_ON_INDEX (Negative point index will soften the edge.) 4: Geom::PolygonMesh::AUTO_SOFTEN (Interior edges are softened.) 8: Geom::PolygonMesh::SMOOTH_SOFT_EDGES (All soft edges will also be smooth.) The 4rd and 5th parameters will accept a Material object or a string name of a material currently in the model.

**Parameters:**
- `polygon_mesh` (Geom::PolygonMesh)
- `weld_vertices` (Boolean) — This argument has no effect and is kept for compatibility reasons. Points are always merged.
- `smooth_flags` (Integer) — flags for softening and smoothing of edges.
- `f_material` (Sketchup::Material, String) — material to paint front faces with.
- `b_material` (Sketchup::Material, String) — material to paint back faces with.

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Sketchup/Entities.html#fill_from_mesh-instance_method)

#### `intersect_with(recurse, transform1, entities1, transform2, hidden, entities2) => Array<Sketchup::Edge>`

The #intersect_with method is used to intersect a Sketchup::Entities, Sketchup::Component, or Sketchup::Group object with a entities object.

**Remarks:** The #intersect_with method is used to intersect a Sketchup::Entities, Sketchup::Component, or Sketchup::Group object with a entities object. empty if no intersection(s) were found.

**Parameters:**
- `recurse` (Boolean) — true if you want this entities object to be recursed (intersection lines will be put inside of groups and components within this entities object).
- `transform1` (Geom::Transformation) — The transformation for this entities object.
- `entities1` (Sketchup::Entities) — The entities object where you want the intersection lines to appear.
- `transform2` (Geom::Transformation) — The transformation for entities1.
- `hidden` (Boolean) — true if you want hidden geometry in this entities object to be used in the intersection.
- `entities2` (Sketchup::Entity, Array<Sketchup::Entity>) — A single entity, or an array of entities.

**Returns:** `Array<Sketchup::Edge>` — The intersecting edges created. This array may be

[Docs](https://ruby.sketchup.com/Sketchup/Entities.html#intersect_with-instance_method)

#### `length => Integer`

The #length method is used to retrieve the number of entities in the collection of entities.

**Remarks:** The #length method is used to retrieve the number of entities in the collection of entities.

**Returns:** `Integer` — 

[Docs](https://ruby.sketchup.com/Sketchup/Entities.html#length-instance_method)

#### `model => Sketchup::Model`

The model method is used to retrieve the model that contains the collection of entities.

**Remarks:** The model method is used to retrieve the model that contains the collection of entities.

**Returns:** `Sketchup::Model` — the model that contains the collection of entities if successful.

[Docs](https://ruby.sketchup.com/Sketchup/Entities.html#model-instance_method)

#### `parent => Sketchup::ComponentDefinition, Sketchup::Model`

The parent method is used to retrieve the parent or object that contains the collection of entities.

**Remarks:** The parent method is used to retrieve the parent or object that contains the collection of entities. A parent can be either a Model or ComponentDefinition object.

**Returns:** `Sketchup::ComponentDefinition, Sketchup::Model` — the object that contains the collection of entities if successful

[Docs](https://ruby.sketchup.com/Sketchup/Entities.html#parent-instance_method)

#### `remove_observer(observer) => Boolean`

The remove_observer method is used to remove an observer from the current object.

**Remarks:** The remove_observer method is used to remove an observer from the current object.

**Parameters:**
- `observer` (Object) — An observer.

**Returns:** `Boolean` — true if successful, false if unsuccessful.

[Docs](https://ruby.sketchup.com/Sketchup/Entities.html#remove_observer-instance_method)

#### `size => Integer`

The #size method is an alias for the #length method.

**Remarks:** The #size method is an alias for the #length method.

**Returns:** `Integer` — 

[Docs](https://ruby.sketchup.com/Sketchup/Entities.html#size-instance_method)

#### `transform_by_vectors(sub_entities, vectors) => Sketchup::Entities`

The transform_by_vectors method is used to apply several vectors to several sub-entities all at once.

**Remarks:** The transform_by_vectors method is used to apply several vectors to several sub-entities all at once.

**Parameters:**
- `sub_entities` (Array<Sketchup::Entity>) — An array of entities to transform.
- `vectors` (Array<Geom::Vector3d>) — An array of vectors to apply.

**Returns:** `Sketchup::Entities` — 

[Docs](https://ruby.sketchup.com/Sketchup/Entities.html#transform_by_vectors-instance_method)

#### `transform_entities(transform, entities) => Boolean`

The transform_entities method is used to apply a transform to several sub-entities all at once.

**Remarks:** The transform_entities method is used to apply a transform to several sub-entities all at once. If you are transforming entities in the active drawing context or any of its parent drawing contexts, the transformation will be interpreted as relative to the global coordinate system. Otherwise the transformation will be interpreted as being on the local coordinate system.

**Parameters:**
- `transform` (Geom::Transformation) — The Transformation to apply.
- `entities` (Array<Sketchup::Entity>) — An array or series of entities to transform.

**Returns:** `Boolean` — false if the entities array was empty.

[Docs](https://ruby.sketchup.com/Sketchup/Entities.html#transform_entities-instance_method)

#### `weld(edges) => Array<Sketchup::Curve>`

The #weld method takes a set of edges and find all possible chains of edges and connect them with a Curve.

**Remarks:** The #weld method takes a set of edges and find all possible chains of edges and connect them with a Curve. A curve will not cross another curve. They will split where multiple curves meet.

**Parameters:**
- `edges` (Array<Sketchup::Edge>)

**Returns:** `Array<Sketchup::Curve>` — 

[Docs](https://ruby.sketchup.com/Sketchup/Entities.html#weld-instance_method)

### Properties
- `active_section_plane` (Sketchup::SectionPlane, nil, get/set) — The active_section_plane method is used to access the currently active section plane in the Entities object.

## EntitiesBuilder (class)

Note: Like Geom::PolygonMesh there is minimal validation checks made on the input to the creation of the geometry. Vertices are de-duplicated and edges sharing the same vertices will be de-duplicated. But no intersection of overlapping entities is made. It leaves a higher responsibility on the API user to produce valid geometry. Note: While using Sketchup::Entities#build it is important to not add or remove vertices by other means of the builder. Also don't modify the position of the vertices in the Entities container geometry is added to. Doing so can break the vertex-cache that de-duplicates the vertices. The EntitiesBuilder is an interface to generate bulk geometry with performance in mind. This is particularly useful for importers where the geometry is already well defined and one wants to recreate it without further processing. Before the Entities Builder was introduced there were two ways of adding geometry; the add_* methods of Entities and Geom::PolygonMesh. The former is slow as the methods perform intersection, splitting and merging of overlapping geometry. This is useful when creating tools similar to the Line and Rectangle tool. Geom::PolygonMesh is fast, but it doesn't provide granular control per face or edge added. Entities Builder is similar to Geom::PolygonMesh in speed, but with the flexibility of Entities's add_* methods. (See “Creating a triangulated grid” example)

**Remarks:** See Also: Guide on Generating Geometry Sketchup::Entities#build Entities Geom::PolygonMesh

[Vendor docs](https://ruby.sketchup.com/Sketchup/EntitiesBuilder.html)

### Methods
#### `add_edge(point1, point2) => Sketchup::Edge | add_edge(points) => Sketchup::Edge`

Note: Does not split intersecting faces or edges.

**Remarks:** Note: Does not split intersecting faces or edges. Adds a Sketchup::Edge to the #entities collection.

**Parameters:**
- `point1` (Geom::Point3d)
- `point2` (Geom::Point3d)

**Returns:** `Sketchup::Edge` — 

[Docs](https://ruby.sketchup.com/Sketchup/EntitiesBuilder.html#add_edge-instance_method)

#### `add_edges(points) => Array<Sketchup::Edge, nil> | add_edges(*points) => Array<Sketchup::Edge, nil>`

Note: Does not split intersecting faces or edges.

**Remarks:** Note: Does not split intersecting faces or edges. Adds a continuous set of Sketchup::Edge's to the #entities collection.

**Parameters:**
- `points` (Array<Geom::Point3d>)

**Returns:** `Array<Sketchup::Edge, nil>` — In the array, for each pair in points an edge is returned. If two point are so close they are considered identical then nil is returned.

[Docs](https://ruby.sketchup.com/Sketchup/EntitiesBuilder.html#add_edges-instance_method)

#### `add_face(outer_loop) => Sketchup::Face | add_face(*outer_loop) => Sketchup::Face | add_face(outer_loop, holes: inner_loops) => Sketchup::Face | add_face(*outer_loop, holes: inner_loops) => Sketchup::Face`

Note: Does not split intersecting faces or edges.

**Remarks:** Note: Does not split intersecting faces or edges. Adds a Face to the #entities collection.

**Parameters:**
- `outer_loop` (Array<Geom::Point3d>)

**Returns:** `Sketchup::Face` — 

[Docs](https://ruby.sketchup.com/Sketchup/EntitiesBuilder.html#add_face-instance_method)

#### `entities => Sketchup::Entities`

The Sketchup::Entities collection the Sketchup::EntitiesBuilder will add the geometry to.

**Remarks:** The Sketchup::Entities collection the Sketchup::EntitiesBuilder will add the geometry to.

**Returns:** `Sketchup::Entities` — 

[Docs](https://ruby.sketchup.com/Sketchup/EntitiesBuilder.html#entities-instance_method)

#### `valid? => Boolean`

Indicates whether the builder object is valid and can be used.

**Remarks:** Indicates whether the builder object is valid and can be used. A builder object is only valid within the scope of the block given to Sketchup::Entities#build. When this return false, calling any other method on the builder will raise an error.

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Sketchup/EntitiesBuilder.html#valid?-instance_method)

#### `vertex_at(position) => Sketchup::Vertex?`

Finds an existing Vertex for the given position, otherwise returns nil.

**Remarks:** Finds an existing Vertex for the given position, otherwise returns nil.

**Returns:** `Sketchup::Vertex, nil` — 

[Docs](https://ruby.sketchup.com/Sketchup/EntitiesBuilder.html#vertex_at-instance_method)

## EntitiesObserver (class)

This class is abstract. To implement this observer, create a Ruby class of this type, override the desired methods, and add an instance of the observer to the objects of interests. Note: The methods of this observer fire in such a way that making changes to the model while inside of them is dangerous. If you experience sudden crashes, it could be because of this observer. A potential workaround is to use a ToolsObserver to watch what the user is doing instead. This observer interface is implemented to react to Entities collection events.

[Vendor docs](https://ruby.sketchup.com/Sketchup/EntitiesObserver.html)

### Methods
#### `onActiveSectionPlaneChanged(entities) => nil`

The #onActiveSectionPlaneChanged method is invoked when a section plane within this entities is activated or the active one is deactivated.

**Remarks:** The #onActiveSectionPlaneChanged method is invoked when a section plane within this entities is activated or the active one is deactivated.

**Parameters:**
- `entities` (Sketchup::Entities)

**Returns:** `nil` — 

[Docs](https://ruby.sketchup.com/Sketchup/EntitiesObserver.html#onActiveSectionPlaneChanged-instance_method)

#### `onElementAdded(entities, entity) => nil`

The onElementAdded method is invoked when a single element is added to the Sketchup::Entities collection.

**Remarks:** The onElementAdded method is invoked when a single element is added to the Sketchup::Entities collection.

**Parameters:**
- `entities` (Sketchup::Entities)
- `entity` (Sketchup::Entity)

**Returns:** `nil` — 

[Docs](https://ruby.sketchup.com/Sketchup/EntitiesObserver.html#onElementAdded-instance_method)

#### `onElementModified(entities, entity) => nil`

The #onElementModified method is invoked whenever one or more elements in the collection are modified.

**Remarks:** The #onElementModified method is invoked whenever one or more elements in the collection are modified.

**Parameters:**
- `entities` (Sketchup::Entities)
- `entity` (Sketchup::Entity)

**Returns:** `nil` — 

[Docs](https://ruby.sketchup.com/Sketchup/EntitiesObserver.html#onElementModified-instance_method)

#### `onElementRemoved(entities, entity_id) => nil`

The #onElementRemoved method is invoked when a single element is removed from the Sketchup::Entities collection.

**Remarks:** The #onElementRemoved method is invoked when a single element is removed from the Sketchup::Entities collection. Note that the entity has been deleted and should not be used in anyway except to know that the entity has been deleted.

**Parameters:**
- `entities` (Sketchup::Entities)
- `entity_id` (Sketchup::Entity) — The id of the entity that was deleted/removed.

**Returns:** `nil` — 

[Docs](https://ruby.sketchup.com/Sketchup/EntitiesObserver.html#onElementRemoved-instance_method)

#### `onEraseEntities(entities) => nil`

The #onEraseEntities method is invoked when one or more entities are erased.

**Remarks:** The #onEraseEntities method is invoked when one or more entities are erased.

**Parameters:**
- `entities` (Sketchup::Entities)

**Returns:** `nil` — 

[Docs](https://ruby.sketchup.com/Sketchup/EntitiesObserver.html#onEraseEntities-instance_method)

## Entity (class)

This is the base class for all SketchUp entities. Entities are basically anything that can be contained in a model, including Drawingelements such as Edges, SectionPlanes, Groups, etc. and entities that relate to those Drawingelements, such as Loops, Layers, etc. Keep in mind that the methods below are available on all subclasses. For example, an Edge's parent class is Drawingelement, and a Drawingelement's parent class is Entity. Therefore an Edge has all of the methods defined in Drawingelement and Entity. The Object.is_a? method is the common way of determining what sort of Entity you're dealing with.

[Vendor docs](https://ruby.sketchup.com/Sketchup/Entity.html)

### Methods
#### `add_observer(observer) => Boolean`

The add_observer method is used to add an observer to the current object.

**Remarks:** The add_observer method is used to add an observer to the current object.

**Parameters:**
- `observer` (Object) — An observer.

**Returns:** `Boolean` — true if successful, false if unsuccessful.

[Docs](https://ruby.sketchup.com/Sketchup/Entity.html#add_observer-instance_method)

#### `attribute_dictionaries => Sketchup::AttributeDictionaries?`

Note: The return value may be either nil or an empty AttributeDictionaries collection if this entity has no AttributeDictionarys.

**Remarks:** Note: The return value may be either nil or an empty AttributeDictionaries collection if this entity has no AttributeDictionarys. The attribute_dictionaries method is used to retrieve the AttributeDictionaries collection attached to the entity.

**Returns:** `Sketchup::AttributeDictionaries, nil` — 

[Docs](https://ruby.sketchup.com/Sketchup/Entity.html#attribute_dictionaries-instance_method)

#### `attribute_dictionary(name, create = false) => Sketchup::AttributeDictionary?`

The attribute_dictionary method is used to retrieve an attribute dictionary with a given name that is attached to an Entity.

**Remarks:** The attribute_dictionary method is used to retrieve an attribute dictionary with a given name that is attached to an Entity.

**Parameters:**
- `name` (String) — The name of the attribute dictionary.
- `create` (Boolean) — boolean, if set to true then the attribute dictionary will be created if it does not exist.

**Returns:** `Sketchup::AttributeDictionary, nil` — an AttributeDictionary object if successful, or nil if there is no attribute dictionary

[Docs](https://ruby.sketchup.com/Sketchup/Entity.html#attribute_dictionary-instance_method)

#### `delete_attribute(dictionary_name) => Boolean | delete_attribute(dictionary_name, key) => Boolean`

Note: In SketchUp 2018, special attribute dictionaries have been added.

**Remarks:** Note: In SketchUp 2018, special attribute dictionaries have been added. The name of these dictionaries are “SU_InstanceSet” and “SU_DefinitionSet”. The dictionaries cannot be deleted via ruby and an ArgumentError will be raised. The key/value pairs in the dictionary can be deleted safely. The #delete_attribute method is used to delete an attribute from an entity. If only the dictionary_name is given, then it deletes the entire AttributeDictionary. Otherwise, #delete_attribute deletes the attribute with the given key from the given dictionary.

**Parameters:**
- `dictionary_name` (String) — The name of an attribute dictionary.

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Sketchup/Entity.html#delete_attribute-instance_method)

#### `deleted? => Boolean`

The deleted? method is used to determine if your entity is still valid (not deleted by another script, for example.

**Remarks:** The deleted? method is used to determine if your entity is still valid (not deleted by another script, for example.)

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Sketchup/Entity.html#deleted?-instance_method)

#### `entityID => Integer`

The entityID method is used to retrieve a unique ID assigned to an entity.

**Remarks:** The entityID method is used to retrieve a unique ID assigned to an entity. The entityID is not persistent between sessions.

**Returns:** `Integer` — the id for the Entity object

[Docs](https://ruby.sketchup.com/Sketchup/Entity.html#entityID-instance_method)

#### `get_attribute(dict_name, key, default_value = nil) => Object`

The #get_attribute method is used to retrieve the value of an attribute in the entity's attribute dictionary.

**Remarks:** The #get_attribute method is used to retrieve the value of an attribute in the entity's attribute dictionary. If the third parameter, default_value, is not passed and there is no attribute that matches the given name, it returns nil. If default_value is provided and there is no matching attribute it returns the given value. It does not create an attribute with that name though.

**Parameters:**
- `dict_name` (String) — The name of an attribute dictionary.
- `key` (String) — An attribute key.
- `default_value` (Object) — A default value to return if no attribute is found.

**Returns:** `Object` — the retrieved value

[Docs](https://ruby.sketchup.com/Sketchup/Entity.html#get_attribute-instance_method)

#### `inspect => String`

The #inspect method is used to retrieve the string representation of the entity.

**Remarks:** The #inspect method is used to retrieve the string representation of the entity.

**Returns:** `String` — 

[Docs](https://ruby.sketchup.com/Sketchup/Entity.html#inspect-instance_method)

#### `model => Sketchup::Model`

The model method is used to retrieve the model for the entity.

**Remarks:** The model method is used to retrieve the model for the entity.

**Returns:** `Sketchup::Model` — the model that contains the Entity object

[Docs](https://ruby.sketchup.com/Sketchup/Entity.html#model-instance_method)

#### `parent => Sketchup::ComponentDefinition, Sketchup::Model`

The parent method is used to retrieve the parent of the entity.

**Remarks:** The parent method is used to retrieve the parent of the entity. The parent will be a ComponentDefinition, a Group, a Model, or whatever the entity is contained within.

**Returns:** `Sketchup::ComponentDefinition, Sketchup::Model` — a Entity object representing the parent of this entity

[Docs](https://ruby.sketchup.com/Sketchup/Entity.html#parent-instance_method)

#### `persistent_id => Integer`

Note: Only a subset of entity types support PIDs.

**Remarks:** Note: Only a subset of entity types support PIDs. Refer to the table below for which and when support was added. In general it is entities that you can iterate over in a Sketchup::Entities collection. The #persistent_id method is used to retrieve a unique persistent id assigned to an entity. The persistent id persistent between sessions. SketchUp 2020.1 ComponentDefinition Material Style SketchUp 2020.0 Layer LineStyle SketchUp 2018 Page SketchUp 2017 ComponentInstance ConstructionLine ConstructionPoint Curve Dimension Sketchup::Edge Face Group Image SectionPlane Text Vertex Polyline3d entities exposed only as Drawingelement Use #typename to determine if a Drawingelement is a “Polyline3d”.

**Returns:** `Integer` — the id for the Sketchup::Entity object

[Docs](https://ruby.sketchup.com/Sketchup/Entity.html#persistent_id-instance_method)

#### `remove_observer(observer) => Boolean`

The remove_observer method is used to remove an observer from the current object.

**Remarks:** The remove_observer method is used to remove an observer from the current object.

**Parameters:**
- `observer` (Object) — An observer.

**Returns:** `Boolean` — true if successful, false if unsuccessful.

[Docs](https://ruby.sketchup.com/Sketchup/Entity.html#remove_observer-instance_method)

#### `set_attribute(dict_name, key, value) => Object`

The set attribute is used to set the value of an attribute in an attribute dictionary with the given name.

**Remarks:** The set attribute is used to set the value of an attribute in an attribute dictionary with the given name. This method will create a new AttributeDictionary if none exists. Note, a bug prior to SketchUp 2015 would corrupt the model if the key is an empty string. This also includes values that will evaluate to empty strings, such as nil.

**Parameters:**
- `dict_name` (String) — The name of an attribute dictionary.
- `key` (String) — An attribute key.
- `value` (Object) — The value for the attribute.

**Returns:** `Object` — the newly set value if successful

[Docs](https://ruby.sketchup.com/Sketchup/Entity.html#set_attribute-instance_method)

#### `to_s => String`

The #to_s method is used to retrieve the string representation of the entity.

**Remarks:** The #to_s method is used to retrieve the string representation of the entity.

**Returns:** `String` — 

[Docs](https://ruby.sketchup.com/Sketchup/Entity.html#to_s-instance_method)

#### `typename => String`

Note: Prefer is_a? over typename when possible as it is faster.

**Remarks:** Note: Prefer is_a? over typename when possible as it is faster. The typename method retrieves the type of the entity, which will be a string such as “Face”, “Edge”, or “Group”.

**Returns:** `String` — the type of the entity

[Docs](https://ruby.sketchup.com/Sketchup/Entity.html#typename-instance_method)

#### `valid? => Boolean`

The #valid? method is used to determine if your entity is still valid (not deleted by another script, for example).

**Remarks:** The #valid? method is used to determine if your entity is still valid (not deleted by another script, for example). This method is functionally the inverse to the #deleted? method.

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Sketchup/Entity.html#valid?-instance_method)

## EntityObserver (class)

This class is abstract. To implement this observer, create a Ruby class of this type, override the desired methods, and add an instance of the observer to the entity of interests. Note: The methods of this observer fire in such a way that making changes to the model while inside of them is dangerous. If you experience sudden crashes, it could be because of this observer. A potential workaround is to use a ToolsObserver to watch what the user is doing instead. This observer interface is implemented to react to entity events.

[Vendor docs](https://ruby.sketchup.com/Sketchup/EntityObserver.html)

### Methods
#### `onChangeEntity(entity) => nil`

The #onChangeEntity method is invoked when your entity is modified.

**Remarks:** The #onChangeEntity method is invoked when your entity is modified.

**Parameters:**
- `entity` (Sketchup::Entity)

**Returns:** `nil` — 

[Docs](https://ruby.sketchup.com/Sketchup/EntityObserver.html#onChangeEntity-instance_method)

#### `onEraseEntity(entity) => nil`

The #onEraseEntity method is invoked when your entity is erased.

**Remarks:** The #onEraseEntity method is invoked when your entity is erased.

**Parameters:**
- `entity` (Sketchup::Entity)

**Returns:** `nil` — 

[Docs](https://ruby.sketchup.com/Sketchup/EntityObserver.html#onEraseEntity-instance_method)

## Environment (class)

An Environment object represents an environment in the model. Environments are used to control the background and lighting of the model. Environments can be used as skydomes, for reflections, and to link the sun to the environment.

[Vendor docs](https://ruby.sketchup.com/Sketchup/Environment.html)

### Methods
#### `linked_sun? => Boolean`

The #linked_sun? method is used to determine if the Sketchup::Environment is linked to the sun.

**Remarks:** The #linked_sun? method is used to determine if the Sketchup::Environment is linked to the sun. This function returns a boolean value indicating whether the shadow light feature is currently enabled in the environment. Shadow lighting is used to create realistic shadows in the scene, enhancing the visual quality.

**Returns:** `Boolean` — true if the environment is linked to the sun, false otherwise

[Docs](https://ruby.sketchup.com/Sketchup/Environment.html#linked_sun?-instance_method)

#### `path => String`

The #path method is used to get the file name of the image or SKE file used for the Sketchup::Environment.

**Remarks:** The #path method is used to get the file name of the image or SKE file used for the Sketchup::Environment.

**Returns:** `String` — the file name of the image or SKE file used for the environment

[Docs](https://ruby.sketchup.com/Sketchup/Environment.html#path-instance_method)

#### `thumbnail => Sketchup::ImageRep`

The #thumbnail method is used to get the thumbnail image of the Sketchup::Environment.

**Remarks:** The #thumbnail method is used to get the thumbnail image of the Sketchup::Environment.

**Returns:** `Sketchup::ImageRep` — 

[Docs](https://ruby.sketchup.com/Sketchup/Environment.html#thumbnail-instance_method)

#### `use_as_skydome? => Boolean`

The #use_as_skydome? method is used to determine if the Sketchup::Environment is used as a skydome.

**Remarks:** The #use_as_skydome? method is used to determine if the Sketchup::Environment is used as a skydome.

**Returns:** `Boolean` — true

[Docs](https://ruby.sketchup.com/Sketchup/Environment.html#use_as_skydome?-instance_method)

#### `use_for_reflections? => Boolean`

The #use_for_reflections? method is used to determine if the Sketchup::Environment is used for reflections.

**Remarks:** The #use_for_reflections? method is used to determine if the Sketchup::Environment is used for reflections.

**Returns:** `Boolean` — true if the environment is used for reflections, false otherwise

[Docs](https://ruby.sketchup.com/Sketchup/Environment.html#use_for_reflections?-instance_method)

#### `write_hdr(path) => String`

The #write_hdr method writes the HDR, EXR or SKE image of the environment to a file in its original file type.

**Remarks:** The #write_hdr method writes the HDR, EXR or SKE image of the environment to a file in its original file type.

**Parameters:**
- `path` (String) — the directory where the image should be written

**Returns:** `String` — the full path of the written file

[Docs](https://ruby.sketchup.com/Sketchup/Environment.html#write_hdr-instance_method)

### Properties
- `description` (String, get/set) — The #description method gets the description for an Sketchup::Environment.
- `linked_sun` (Boolean, set) — The #linked_sun= method is used to set if the Sketchup::Environment is linked to the sun.
- `linked_sun_position` (Geom::Point3d, get/set) — The #linked_sun_position method is used to get the position of the sun linked to the Sketchup::Environment.
- `name` (String, get/set) — The #name method retrieves the name of the Sketchup::Environment.
- `reflection_exposure` (Float, get/set) — Note: Reflection exposure is a value between 0.
- `rotation` (Float, get/set) — The #rotation method is used to get the vertical rotation angle in degrees to apply to the Sketchup::Environment.
- `skydome_exposure` (Float, get/set) — Note: Skydome exposure is a value between 0.
- `use_as_skydome` (Boolean, set) — The #use_as_skydome= method is used to set if the Sketchup::Environment is used as a skydome.
- `use_for_reflections` (Boolean, set) — The #use_for_reflections= method is used to set if the Sketchup::Environment is used for reflections.

## Environments (class)

An Environments object is a collection of Environment objects. It is used to manage the environments in a model. An Environment object represents an environment in the model. Environments are used to control the background and lighting of the model. Environments can be used as skyboxes, for reflections, and to link the sun to the environment.

[Vendor docs](https://ruby.sketchup.com/Sketchup/Environments.html)

### Methods
#### `[](name) => Sketchup::Environment?`

The #[] method is used to retrieve an Sketchup::Environment by name.

**Remarks:** The #[] method is used to retrieve an Sketchup::Environment by name.

**Parameters:**
- `name` (String) — The name of the Sketchup::Environment.

**Returns:** `Sketchup::Environment, nil` — 

[Docs](https://ruby.sketchup.com/Sketchup/Environments.html#[]-instance_method)

#### `add(name, path) => Sketchup::Environment | add(path) => Sketchup::Environment`

Note: The supported file formats are HDR, EXR and SKE.

**Remarks:** Note: The supported file formats are HDR, EXR and SKE. The #add method adds an Sketchup::Environment to the Sketchup::Environments.

**Parameters:**
- `name` (String) — the name of the environment.
- `path` (String) — the path to the image or SKE file used for the environment

**Returns:** `Sketchup::Environment` — the newly created environment

[Docs](https://ruby.sketchup.com/Sketchup/Environments.html#add-instance_method)

#### `add_observer(arg) => Boolean`

The #add_observer method is used to add an observer to the environments collection.

**Remarks:** The #add_observer method is used to add an observer to the environments collection.

**Returns:** `Boolean` — true if successful, false if unsuccessful.

[Docs](https://ruby.sketchup.com/Sketchup/Environments.html#add_observer-instance_method)

#### `each {|environment| ... } => Sketchup::Environments`

The #each method is used to iterate over all the environments in the Sketchup::Environments.

**Remarks:** The #each method is used to iterate over all the environments in the Sketchup::Environments.

**Returns:** `Sketchup::Environments` — 

[Docs](https://ruby.sketchup.com/Sketchup/Environments.html#each-instance_method)

#### `purge_unused => Sketchup::Environments`

The #purge_unused method is used to remove unused environments.

**Remarks:** The #purge_unused method is used to remove unused environments.

**Returns:** `Sketchup::Environments` — 

[Docs](https://ruby.sketchup.com/Sketchup/Environments.html#purge_unused-instance_method)

#### `remove(environment) => Boolean`

The #remove method removes an Sketchup::Environment from the Sketchup::Environments.

**Remarks:** The #remove method removes an Sketchup::Environment from the Sketchup::Environments.

**Parameters:**
- `environment` (Sketchup::Environment) — the environment to remove

**Returns:** `Boolean` — true if the environment was removed, false if it was not found

[Docs](https://ruby.sketchup.com/Sketchup/Environments.html#remove-instance_method)

#### `remove_observer(arg) => Boolean`

The #remove_observer method is used to remove an observer from the current object.

**Remarks:** The #remove_observer method is used to remove an observer from the current object.

**Returns:** `Boolean` — true if successful, false if unsuccessful.

[Docs](https://ruby.sketchup.com/Sketchup/Environments.html#remove_observer-instance_method)

#### `size => Integer Also known as: length`

The #size method retrieves the number of environments in the Sketchup::Environments.

**Remarks:** The #size method retrieves the number of environments in the Sketchup::Environments.

**Returns:** `Integer` — the number of environments

[Docs](https://ruby.sketchup.com/Sketchup/Environments.html#size-instance_method)

### Properties
- `current` (Sketchup::Environment, nil, get/set) — The #current method is used to get the current environment in the Sketchup::Environments.

## EnvironmentsObserver (class)

This class is abstract. To implement this observer, create a Ruby class of this type, override the desired methods, and add an instance of the observer to the Environment object. This observer interface is implemented to react to Environment events.

**Remarks:** See Also: Sketchup::Environments#add_observer

[Vendor docs](https://ruby.sketchup.com/Sketchup/EnvironmentsObserver.html)

### Methods
#### `onEnvironmentAdd(environments, environment) => nil`

The #onEnvironmentAdd method is called whenever an environment is added to the Sketchup::Environments.

**Remarks:** The #onEnvironmentAdd method is called whenever an environment is added to the Sketchup::Environments.

**Parameters:**
- `environments` (Sketchup::Environments)
- `environment` (Sketchup::Environment)

**Returns:** `nil` — 

[Docs](https://ruby.sketchup.com/Sketchup/EnvironmentsObserver.html#onEnvironmentAdd-instance_method)

#### `onEnvironmentChange(environments, environment) => nil`

The #onEnvironmentChange method is called whenever the environment properties are changed.

**Remarks:** The #onEnvironmentChange method is called whenever the environment properties are changed.

**Parameters:**
- `environments` (Sketchup::Environments)
- `environment` (Sketchup::Environment)

**Returns:** `nil` — 

[Docs](https://ruby.sketchup.com/Sketchup/EnvironmentsObserver.html#onEnvironmentChange-instance_method)

#### `onEnvironmentRemove(environments, environment) => nil`

The #onEnvironmentRemove method is called whenever an environment is removed from the Sketchup::Environments.

**Remarks:** The #onEnvironmentRemove method is called whenever an environment is removed from the Sketchup::Environments.

**Parameters:**
- `environments` (Sketchup::Environments)
- `environment` (Sketchup::Environment)

**Returns:** `nil` — 

[Docs](https://ruby.sketchup.com/Sketchup/EnvironmentsObserver.html#onEnvironmentRemove-instance_method)

#### `onEnvironmentSetCurrent(environments, environment) => nil`

The #onEnvironmentSetCurrent method is called whenever the current environment is changed.

**Remarks:** The #onEnvironmentSetCurrent method is called whenever the current environment is changed.

**Parameters:**
- `environments` (Sketchup::Environments)
- `environment` (Sketchup::Environment)

**Returns:** `nil` — 

[Docs](https://ruby.sketchup.com/Sketchup/EnvironmentsObserver.html#onEnvironmentSetCurrent-instance_method)

## ExtensionsManager (class)

The ExtensionsManager class provides a way of accessing the SketchupExtensions that have been registered via the Sketchup.register_extension method. There is only one ExtensionsManager available. You access it via the Sketchup.extensions method.

[Vendor docs](https://ruby.sketchup.com/Sketchup/ExtensionsManager.html)

### Methods
#### `[](index_or_name) => SketchupExtension?`

The [] method is used to get an extension by name, index or ID.

**Remarks:** The [] method is used to get an extension by name, index or ID.

**Parameters:**
- `index_or_name` (Integer, String) — The index, name or ID of the SketchupExtension object.

**Returns:** `SketchupExtension, nil` — 

[Docs](https://ruby.sketchup.com/Sketchup/ExtensionsManager.html#[]-instance_method)

#### `count => Integer`

Note: Since SketchUp 2014 the count method is inherited from Ruby's Enumerable mix-in module.

**Remarks:** Note: Since SketchUp 2014 the count method is inherited from Ruby's Enumerable mix-in module. Prior to that the #count method is an alias for #length.

**Returns:** `Integer` — 

[Docs](https://ruby.sketchup.com/Sketchup/ExtensionsManager.html#count-instance_method)

#### `each {|extension| ... } => nil`

The #each method is used to iterate through extensions.

**Remarks:** The #each method is used to iterate through extensions.

**Returns:** `nil` — 

[Docs](https://ruby.sketchup.com/Sketchup/ExtensionsManager.html#each-instance_method)

#### `keys => Object`

The keys method is used to get a list of keys in the ExtensionsManager, which are the same as the names of the extensions.

**Remarks:** The keys method is used to get a list of keys in the ExtensionsManager, which are the same as the names of the extensions.

**Returns:** `Object` — keys - Array of string keys

[Docs](https://ruby.sketchup.com/Sketchup/ExtensionsManager.html#keys-instance_method)

#### `length => Integer`

The #length method returns the number of SketchupExtension objects inside this ExtensionsManager.

**Remarks:** The #length method returns the number of SketchupExtension objects inside this ExtensionsManager.

**Returns:** `Integer` — 

[Docs](https://ruby.sketchup.com/Sketchup/ExtensionsManager.html#length-instance_method)

#### `size => Integer`

The #size method is an alias of #length.

**Remarks:** The #size method is an alias of #length.

**Returns:** `Integer` — 

[Docs](https://ruby.sketchup.com/Sketchup/ExtensionsManager.html#size-instance_method)

## Face (class)

Faces in SketchUp are flat, 2-sided polygons with 3 or more sides.

[Vendor docs](https://ruby.sketchup.com/Sketchup/Face.html)

### Methods
#### `all_connected => Array<Sketchup::Entity>`

The all_connected method retrieves all of the entities connected to a face.

**Remarks:** The all_connected method retrieves all of the entities connected to a face.

**Returns:** `Array<Sketchup::Entity>` — the entities connected to the face

[Docs](https://ruby.sketchup.com/Sketchup/Face.html#all_connected-instance_method)

#### `area => Float | area(transform) => Float`

The area method is used to retrieve the area of a face.

**Remarks:** The area method is used to retrieve the area of a face. You can pass in an optional Transformation (or an array that can represent a transformation), to correct for a parent group's transformation. For example, if a face is inside of a group that is scaled to 200%, the area method will return the unscaled area of the face. So by passing a 200% transformation object to this method, you can account for that to get the “visual” area of the face.

**Parameters:**
- `transform` (Geom::Transformation) — A Transformation object or array that can be interpreted as a Transformation object.

**Returns:** `Float` — the area of the face in square inches.

[Docs](https://ruby.sketchup.com/Sketchup/Face.html#area-instance_method)

#### `classify_point(point) => Integer`

The classify_point method is used to determine if a given Point3d is on the referenced Face.

**Remarks:** The classify_point method is used to determine if a given Point3d is on the referenced Face. It is important that return value comparisons be made against the symbolic constants (i.e. PointUnknown, PointInside, PointOnVertex, etc.) rather than the absolute integer values as these values may change from one release to the next.

**Parameters:**
- `point` (Geom::Point3d) — A Point3d.

**Returns:** `Integer` — an integer describing where a Point3d is in relation to the referenced Face.

[Docs](https://ruby.sketchup.com/Sketchup/Face.html#classify_point-instance_method)

#### `clear_texture_position(front) => Object`

The #clear_texture_position method is used to remove any explicit texture positioning for a face and have SketchUp display it with the default texture positioning.

**Remarks:** The #clear_texture_position method is used to remove any explicit texture positioning for a face and have SketchUp display it with the default texture positioning.

**Parameters:**
- `front` (Boolean) — true Clears on the front side of the face, false the back side.

[Docs](https://ruby.sketchup.com/Sketchup/Face.html#clear_texture_position-instance_method)

#### `clear_texture_projection(frontside) => Object`

The #clear_texture_projection method is used to clear the texture projection.

**Remarks:** The #clear_texture_projection method is used to clear the texture projection. This is similar to toggling off Projection from the Position Texture tool in the UI.

**Parameters:**
- `frontside` (Boolean) — true for front side, false for back side.

[Docs](https://ruby.sketchup.com/Sketchup/Face.html#clear_texture_projection-instance_method)

#### `coplanar_with?(other_face) => Boolean`

The #coplanar_with? method is used determine whether a face is coplanar with `other_face`.

**Remarks:** The #coplanar_with? method is used determine whether a face is coplanar with `other_face`.

**Parameters:**
- `other_face` (Sketchup::Face) — The face to compare with.

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Sketchup/Face.html#coplanar_with?-instance_method)

#### `edges => Array<Sketchup::Edge>`

The edges method is used to get an array of edges that bound the face.

**Remarks:** The edges method is used to get an array of edges that bound the face.

**Returns:** `Array<Sketchup::Edge>` — an array of Edge objects (if successful)

[Docs](https://ruby.sketchup.com/Sketchup/Face.html#edges-instance_method)

#### `followme(edges) => Boolean | followme(edge) => Boolean`

The #followme method is used to create a shape by making the face follow along an array of edges.

**Remarks:** The #followme method is used to create a shape by making the face follow along an array of edges.

**Parameters:**
- `edges` (Array<Sketchup::Edge>) — An array of edge objects to follow.

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Sketchup/Face.html#followme-instance_method)

#### `get_UVHelper(front = true, back = true) => Sketchup::UVHelper | get_UVHelper(front = true, back = true, texturewriter) => Sketchup::UVHelper`

The get_UVHelper object is used to retrieve a UVHelper object for use in texture manipulation on a face.

**Remarks:** The get_UVHelper object is used to retrieve a UVHelper object for use in texture manipulation on a face.

**Parameters:**
- `front` (Boolean) — True if you want the texture coordinates for the front face, false if not.
- `back` (Boolean) — True if you want the texture coordinates for the back face, false if not.

**Returns:** `Sketchup::UVHelper` — 

[Docs](https://ruby.sketchup.com/Sketchup/Face.html#get_UVHelper-instance_method)

#### `get_glued_instances => Array<Sketchup::ComponentInstance, Sketchup::Group, Sketchup::Image>`

The get_glued_instances method returns an Array any ComponentInstances that are glued to the face.

**Remarks:** The get_glued_instances method returns an Array any ComponentInstances that are glued to the face.

**Returns:** `Array<Sketchup::ComponentInstance, Sketchup::Group, Sketchup::Image>` — An array of ComponentInstance objects that are currently glued to the face.

[Docs](https://ruby.sketchup.com/Sketchup/Face.html#get_glued_instances-instance_method)

#### `get_texture_projection(frontside) => Geom::Vector3d?`

The #get_texture_projection method will return a vector representing the projection for either the front or back side of the face.

**Remarks:** The #get_texture_projection method will return a vector representing the projection for either the front or back side of the face.

**Parameters:**
- `frontside` (Boolean) — true for front side, false for back side.

**Returns:** `Geom::Vector3d, nil` — a vector on success, nil if face is not textured with a projected texture mapping.

[Docs](https://ruby.sketchup.com/Sketchup/Face.html#get_texture_projection-instance_method)

#### `loops => Array<Sketchup::Loop>`

The loops method is used to get an array of all of the loops that bound the face.

**Remarks:** The loops method is used to get an array of all of the loops that bound the face.

**Returns:** `Array<Sketchup::Loop>` — an array of Loop objects if successful

[Docs](https://ruby.sketchup.com/Sketchup/Face.html#loops-instance_method)

#### `mesh(flags = 0) => Geom::PolygonMesh`

The mesh method creates a polygon mesh that represents the face.

**Remarks:** The mesh method creates a polygon mesh that represents the face. See the Geom::PolygonMesh class for more information. Valid flags are: 0: Include PolygonMeshPoints, 1: Include PolygonMeshUVQFront, 2: Include PolygonMeshUVQBack, 4: Include PolygonMeshNormals. Use bitwise OR to combine flags. A value of 7 will include all flags, for example.

**Parameters:**
- `flags` (Integer) — One or more flags used to generate a mesh.

**Returns:** `Geom::PolygonMesh` — 

[Docs](https://ruby.sketchup.com/Sketchup/Face.html#mesh-instance_method)

#### `normal => Geom::Vector3d`

The normal method is used to retrieve the 3D vector normal to the face in the front direction.

**Remarks:** The normal method is used to retrieve the 3D vector normal to the face in the front direction.

**Returns:** `Geom::Vector3d` — a Vector3d object if successful

[Docs](https://ruby.sketchup.com/Sketchup/Face.html#normal-instance_method)

#### `outer_loop => Sketchup::Loop`

This method is used to retrieve the outer loop that bounds the face.

**Remarks:** This method is used to retrieve the outer loop that bounds the face.

**Returns:** `Sketchup::Loop` — a Loop object representing the outer loop (if successful)

[Docs](https://ruby.sketchup.com/Sketchup/Face.html#outer_loop-instance_method)

#### `plane => Array(Float, Float, Float, Float)`

The plane method is used to retrieve the plane of the face.

**Remarks:** The plane method is used to retrieve the plane of the face. See the Array class for information on how planes are stored.

**Returns:** `Array(Float, Float, Float, Float)` — a plane that contains the face (if successful)

[Docs](https://ruby.sketchup.com/Sketchup/Face.html#plane-instance_method)

#### `position_material(material, points, on_front) => Sketchup::Face, false | position_material(material, points, on_front, projection) => Sketchup::Face, false`

The #position_material method is used to position a material on a face.

**Remarks:** The #position_material method is used to position a material on a face. The points argument must contain 2, 4, 6 or 8 points. The points are used in pairs to describe where a point in the texture image is positioned on the Face. The first point in each pair is a 3D point in the model. The second point in each pair of points is a 2D point that gives the (u,v) coordinates of a point in the image to match up with the 3D point.

**Parameters:**
- `material` (Sketchup::Material)
- `points` (Array<Geom::Point3d>) — An array of Point3d objects used to position the material. The points should be on the plane of the face. If they are not they will be projected to the face's plane.
- `on_front` (Boolean) — true to position the texture on the front of the Face or false to position it on the back of the Face.

**Returns:** `Sketchup::Face, false` — the face upon success, false if material or texture if not valid.

[Docs](https://ruby.sketchup.com/Sketchup/Face.html#position_material-instance_method)

#### `pushpull(distance, copy = false) => nil`

The pushpull method is used to perform a push/pull on a face.

**Remarks:** The pushpull method is used to perform a push/pull on a face. The distance is measured in the direction that the face normal is pointing.

**Parameters:**
- `distance` (Length) — The distance to push/pull the face.
- `copy` (Boolean) — Create a new push/pull starting face if true (equivalent of pressing CTRL while in SketchUp), do not create a push/pull starting face if false.

**Returns:** `nil` — 

[Docs](https://ruby.sketchup.com/Sketchup/Face.html#pushpull-instance_method)

#### `reverse! => Sketchup::Face, false`

The reverse! method is used to reverse the face's orientation, meaning the front becomes the back.

**Remarks:** The reverse! method is used to reverse the face's orientation, meaning the front becomes the back.

**Returns:** `Sketchup::Face, false` — the reversed Face object if successful, false if unsuccessful

[Docs](https://ruby.sketchup.com/Sketchup/Face.html#reverse!-instance_method)

#### `set_texture_projection(vector, frontside) => Boolean`

Deprecated.

**Remarks:** Deprecated. This function never worked correctly. It's not capable of controlling the position and orientation of the texture. In some cases it produced an invalid model. As of SketchUp 2021.1 the method simply raises NotImplementedError. The #set_texture_projection method is used to set the texture projection direction.

**Parameters:**
- `vector` (Geom::Vector3d) — representing the direction of the projection. Use nil to remove the projection.
- `frontside` (Boolean) — true for front side, false for back side.

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Sketchup/Face.html#set_texture_projection-instance_method)

#### `texture_positioned?(front) => Boolean`

The #texture_positioned? method is used to check if the face has a texture that is positioned.

**Remarks:** The #texture_positioned? method is used to check if the face has a texture that is positioned. A texture is positioned when it's not using the default texture coordinates. When a user uses the Paint Bucket Tool to apply a material sampled from the Material Browser it will use default texture coordinates and not be positioned. It will be positioned if the user uses the Position Texture Tool. When an API user uses Drawinglement#material= the texture is not positioned. It will be positioned when the API user uses #position_material. It it also positioned of the face was crafted via Geom::PolygonMesh.

**Parameters:**
- `front` (Boolean) — true Checks the front side of the face, false the back side.

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Sketchup/Face.html#texture_positioned?-instance_method)

#### `texture_projected?(front) => Boolean`

The #texture_projected? method is used to check if the face has a texture that is projected.

**Remarks:** The #texture_projected? method is used to check if the face has a texture that is projected. A texture is projected when the user enables this property via the Position Texture Tool. It is also projected when the API user passes a projection vector to #position_material.

**Parameters:**
- `front` (Boolean) — true Checks the front side of the face, false the back side.

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Sketchup/Face.html#texture_projected?-instance_method)

#### `uv_tile_at(position, front) => Array<Geom::Point3d>?`

The #uv_tile_at method is used to get the corner positions (model and UV) of a UV tile.

**Remarks:** The #uv_tile_at method is used to get the corner positions (model and UV) of a UV tile. The UV tile bounds the given reference point on the plane of the face. If the reference isn't on the plane of the face it will be projected onto it. The world coordinates are on the plane of the face unless the texture is projected. When the texture is projected the the world points are on an arbitrary plane that is perpendicular to the projection direction. The returned coordinates are arranged to be compatible with #position_material. Getting the bounds of the UV tile under the cursor: The red quadrilateral represents the model points returned.

**Parameters:**
- `position` (Geom::Point3d) — Model position on the face's plane that will be bounded by the UV tile.
- `front` (Boolean) — true Checks the front side of the face, false the back side.

**Returns:** `Array<Geom::Point3d>, nil` — A set of 8 points. Each stride of two is first a model space point, the second a UV coordinate. nil if the face doesn't have a texture on the given side.

[Docs](https://ruby.sketchup.com/Sketchup/Face.html#uv_tile_at-instance_method)

#### `vertices => Array<Sketchup::Vertex>`

The vertices method is used to get an array of all of the vertices that bound the face.

**Remarks:** The vertices method is used to get an array of all of the vertices that bound the face.

**Returns:** `Array<Sketchup::Vertex>` — an array of Vertex objects if successful

[Docs](https://ruby.sketchup.com/Sketchup/Face.html#vertices-instance_method)

### Properties
- `back_material` (Sketchup::Material, nil, get/set) — The back_material method is used to retrieve the material assigned to the back side of the face.
- `material` (Sketchup::Material, nil, get/set) — The material method is used to retrieve the material assigned to the front of the face.
- `PointUnknown` (Object, get) — 
- `PointInside` (Object, get) — 
- `PointOnVertex` (Object, get) — 
- `PointOnEdge` (Object, get) — 
- `PointOnFace` (Object, get) — 
- `PointOutside` (Object, get) — 
- `PointNotOnPlane` (Object, get) — 

## FrameChangeObserver (class)

This class is abstract. Implement the methods described in this class to create a frame change observer. You can not sub-class this class because it is not defined by the API. This observer interface is implemented to react to changes in camera position (a frame) between one scene page and another. This observer's callback method is called when the user manually makes a scene change, or the internal animation feature runs. This abstract observer is any object that implements a callback method frameChange with 3 arguments: from_scene (the scene that you transition from), to_scene (the scene that you transition toward) and a percent_done between 0.0 and 1.0 (that indicates the percentage of transition between the two scenes.) The observer is attached using the Pages.add_frame_change_observer class method, which returns an integer id that can be stored and later used to detach the observer. Later, detaching this observer is done by passing this id reference to the Pages.remove_frame_change_observer class method.

[Vendor docs](https://ruby.sketchup.com/Sketchup/FrameChangeObserver.html)

### Methods
#### `frameChange(from_scene, to_scene, percent_done) => Object`

Note: The from_scene argument into this callback does not appear to be populated on the PC.

**Remarks:** Note: The from_scene argument into this callback does not appear to be populated on the PC. You can store a variable that keeps track of the to_scene and then use that on a subsequent Scene selection to determine the last Page that the user was on. This callback method is called during a slide show or creation of an animation after the camera has been set up, but before the frame is displayed. It gives you a chance to perform your own actions during the animation. The arguments for frameChange method are the scene page that you transition from (from_scene), the scene page that you transition to (to_scene), and a percent_done between 0.0 and 1.0 that tell you the percentage of the transition between the two scene pages. By watching for percent_done >= 1.0, you can activate Ruby code that executes as soon as the user's camera has finished animating.

**Parameters:**
- `from_scene` (Sketchup::Page, nil) — The previous scene page the view is transitioning from.
- `to_scene` (Sketchup::Page) — The selected scene page the view is transitioning towards.
- `percent_done` (Float) — The percentage of transition between the two scene pages.

[Docs](https://ruby.sketchup.com/Sketchup/FrameChangeObserver.html#frameChange-instance_method)

## Group (class)

A Group class contains methods for manipulating groups of entities. Groups in SketchUp are very similar to components, but can from a user point of view be thought of as unique objects. Groups can be instanced when copied but are silently made unique when edited through the GUI. To honor this behavior, make sure to call #make_unique before modifying a group through the API.

[Vendor docs](https://ruby.sketchup.com/Sketchup/Group.html)

### Methods
#### `add_observer(observer) => Boolean`

The add_observer method is used to add a ComponentInstance observer to the group.

**Remarks:** The add_observer method is used to add a ComponentInstance observer to the group.

**Parameters:**
- `observer` (Object) — An observer.

**Returns:** `Boolean` — true if successful, false if unsuccessful.

[Docs](https://ruby.sketchup.com/Sketchup/Group.html#add_observer-instance_method)

#### `copy => Sketchup::Group`

The copy method is used to create a new Group object that is a copy of the group.

**Remarks:** The copy method is used to create a new Group object that is a copy of the group.

**Returns:** `Sketchup::Group` — a new Group object

[Docs](https://ruby.sketchup.com/Sketchup/Group.html#copy-instance_method)

#### `definition => Sketchup::ComponentDefinition`

The definition method is used to retrieve the component definition for this group.

**Remarks:** The definition method is used to retrieve the component definition for this group.

**Returns:** `Sketchup::ComponentDefinition` — a ComponentDefinition object if successful

[Docs](https://ruby.sketchup.com/Sketchup/Group.html#definition-instance_method)

#### `entities => Sketchup::Entities`

The entities method is used to retrieve a collection of entities in the group.

**Remarks:** The entities method is used to retrieve a collection of entities in the group.

**Returns:** `Sketchup::Entities` — an Entities object if successful

[Docs](https://ruby.sketchup.com/Sketchup/Group.html#entities-instance_method)

#### `equals?(group) => Boolean`

The equals? method is used to determine if a group is geometrically equivalent to another group.

**Remarks:** The equals? method is used to determine if a group is geometrically equivalent to another group.

**Parameters:**
- `group` (Sketchup::Group) — The group to compare this group with.

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Sketchup/Group.html#equals?-instance_method)

#### `explode => Array<Sketchup::Drawingelement>`

The explode method is used to explode the group into individual entities.

**Remarks:** The explode method is used to explode the group into individual entities.

**Returns:** `Array<Sketchup::Drawingelement>` — An array of entity objects if successful, false if unsuccessful.

[Docs](https://ruby.sketchup.com/Sketchup/Group.html#explode-instance_method)

#### `guid => String`

The guid method is used to get the base 64 encoded unique id for this SketchUp object.

**Remarks:** The guid method is used to get the base 64 encoded unique id for this SketchUp object.

**Returns:** `String` — a unique 22 character string

[Docs](https://ruby.sketchup.com/Sketchup/Group.html#guid-instance_method)

#### `intersect(group) => Sketchup::Group?`

Note: This method is not available in SketchUp Make.

**Remarks:** Note: This method is not available in SketchUp Make. The intersect method is used to compute the boolean intersection of two groups representing manifold solid volumes (this & arg). If the specified objects (this and arg) do not represent manifold volumes, this method fails.

**Parameters:**
- `group` (Sketchup::Group, Sketchup::ComponentInstance) — The group to intersect this group with.

**Returns:** `Sketchup::Group, nil` — The resultant group if the two objects (this and arg) represent manifold solids and the operation succeeds. Otherwise nil is returned.

[Docs](https://ruby.sketchup.com/Sketchup/Group.html#intersect-instance_method)

#### `local_bounds => Geom::BoundingBox`

Deprecated.

**Remarks:** Deprecated. In favor of `group.definition.bounds`. The #local_bounds method is used to retrieve the Geom::BoundingBox bounding the contents of a Sketchup::Group, in the group's own internal coordinate system.

**Returns:** `Geom::BoundingBox` — 

[Docs](https://ruby.sketchup.com/Sketchup/Group.html#local_bounds-instance_method)

#### `locked? => Boolean`

The locked? method is used to determine if a group is locked.

**Remarks:** The locked? method is used to determine if a group is locked.

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Sketchup/Group.html#locked?-instance_method)

#### `make_unique => Sketchup::Group`

The #make_unique method is used to force a group to have a unique definition.

**Remarks:** The #make_unique method is used to force a group to have a unique definition. If the group is already unique in the model, nothing happens. Copying a group in SketchUp will create a group that shares the same definition. SketchUp implicitly makes group unique when edited from the GUI, and from a user point of view groups could be thought of as always being unique. To honor this behavior, call this method before editing a group through the API.

**Returns:** `Sketchup::Group` — the unique group

[Docs](https://ruby.sketchup.com/Sketchup/Group.html#make_unique-instance_method)

#### `manifold? => Boolean`

The manifold? method is used to determine if a group is manifold.

**Remarks:** The manifold? method is used to determine if a group is manifold.

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Sketchup/Group.html#manifold?-instance_method)

#### `move!(transformation) => Sketchup::Group`

Note: Despite the name being similar to #transform!, this method closer corresponds to #transformation=.

**Remarks:** Note: Despite the name being similar to #transform!, this method closer corresponds to #transformation=. The #move! method is used to set the transformation of this group instance, similarly to #transformation= but without recording to the undo stack. This method is useful for moving entities inside of an animation or page transition.

**Parameters:**
- `transformation` (Geom::Transformation)

**Returns:** `Sketchup::Group` — the transformed Group object if successful

[Docs](https://ruby.sketchup.com/Sketchup/Group.html#move!-instance_method)

#### `outer_shell(group) => Sketchup::Group?`

The outer_shell method is used to compute the outer shell of the two groups representing manifold solid volumes (this || arg).

**Remarks:** The outer_shell method is used to compute the outer shell of the two groups representing manifold solid volumes (this || arg). If the specified objects (this and arg) do not represent manifold volumes, this method fails.

**Parameters:**
- `group` (Sketchup::Group, Sketchup::ComponentInstance) — The group to outer shell this group with.

**Returns:** `Sketchup::Group, nil` — The resultant group if the two objects (this and arg) represent manifold solids and the operation succeeds otherwise nil is returned.

[Docs](https://ruby.sketchup.com/Sketchup/Group.html#outer_shell-instance_method)

#### `remove_observer(observer) => Boolean`

The remove_observer method is used to remove a ComponentInstance observer from the group.

**Remarks:** The remove_observer method is used to remove a ComponentInstance observer from the group.

**Parameters:**
- `observer` (Object) — An observer.

**Returns:** `Boolean` — true if successful, false if unsuccessful.

[Docs](https://ruby.sketchup.com/Sketchup/Group.html#remove_observer-instance_method)

#### `show_differences(group, verbose) => Boolean`

The show_differences method is used to determine if a group is geometrically equivalent to another group and in addition move the non- matching and matching geometry to new layers.

**Remarks:** The show_differences method is used to determine if a group is geometrically equivalent to another group and in addition move the non- matching and matching geometry to new layers. This method will move both groups to Layer0. Geometry that is the same in both groups will be moved to a new layer named group_name + “_same”. Geometry that is not the same will be moved to a layer named group_name + “_diff”. If verbose is true, a list of all the geometry that is different from one group to the other is displayed texturally in the Ruby Console.

**Parameters:**
- `group` (Sketchup::Group, Sketchup::ComponentInstance) — The group to be compared with.
- `verbose` (Boolean) — A boolean flag indicating whether to display a textural report of the found differences to the Ruby console.

**Returns:** `Boolean` — true if the groups are geometrically equivalent. Otherwise false.

[Docs](https://ruby.sketchup.com/Sketchup/Group.html#show_differences-instance_method)

#### `split(group) => Array(Sketchup::Group, Sketchup::Group, Sketchup::Group)?`

Note: This method is not available in SketchUp Make.

**Remarks:** Note: This method is not available in SketchUp Make. The split method is used to compute the boolean split (map overlay) of the two groups representing manifold solid volumes (this ^ arg). If the specified objects (this and arg) do not represent manifold volumes, this method fails.

**Parameters:**
- `group` (Sketchup::Group, Sketchup::ComponentInstance) — The group to split this group with.

**Returns:** `Array(Sketchup::Group, Sketchup::Group, Sketchup::Group), nil` — A vector (array) of the three resultant groups If the two objects (this and arg) represent manifold solids and the operation succeeds otherwise nil is returned. The 3 groups are as follows: The intersection of volume 1 & volume 2, the difference of volume 1 minus volume 2, and the reverse difference of volume 2 minus volume 1.

[Docs](https://ruby.sketchup.com/Sketchup/Group.html#split-instance_method)

#### `subtract(group) => Sketchup::Group?`

Note: This method is not available in SketchUp Make.

**Remarks:** Note: This method is not available in SketchUp Make. The subtract method is used to compute the boolean difference of the two groups representing manifold solid volumes (this - arg). If the specified objects (this and arg) do not represent manifold volumes, this method fails.

**Parameters:**
- `group` (Sketchup::Group, Sketchup::ComponentInstance) — The group to subtract this group from.

**Returns:** `Sketchup::Group, nil` — The resultant group if the two objects (this and arg) represent manifold solids and the operation succeeds. Otherwise nil is returned.

[Docs](https://ruby.sketchup.com/Sketchup/Group.html#subtract-instance_method)

#### `to_component => Sketchup::ComponentInstance`

The to_component method is used to convert the group to a component instance.

**Remarks:** The to_component method is used to convert the group to a component instance.

**Returns:** `Sketchup::ComponentInstance` — the new ComponentInstance object

[Docs](https://ruby.sketchup.com/Sketchup/Group.html#to_component-instance_method)

#### `transform!(transform) => Sketchup::Group`

The transform! method is used to apply a transformation to a group.

**Remarks:** The transform! method is used to apply a transformation to a group.

**Parameters:**
- `transform` (Geom::Transformation) — A Transformation object.

**Returns:** `Sketchup::Group` — a transformed group object if successful

[Docs](https://ruby.sketchup.com/Sketchup/Group.html#transform!-instance_method)

#### `trim(group) => Sketchup::Group?`

Note: Trimming object group2 using group1 results in a new trimmed version of group2.

**Remarks:** Note: Trimming object group2 using group1 results in a new trimmed version of group2. If the trim is successful the original group2 is erased and a newly trimmed version is created. This new version, derived from the trimming operation, will possess a new GUID and will be returned. Note: This method is not available in SketchUp Make. The #trim method is used to compute the (non-destructive) boolean difference of the two groups representing manifold solid volumes (this - arg). If the specified objects (this and arg) do not represent manifold volumes, this method fails.

**Parameters:**
- `group` (Sketchup::Group, Sketchup::ComponentInstance) — The group to trim this group against.

**Returns:** `Sketchup::Group, nil` — The resultant group if the two objects (this and arg) represent manifold solids and the operation succeeds otherwise nil is returned.

[Docs](https://ruby.sketchup.com/Sketchup/Group.html#trim-instance_method)

#### `union(group) => Sketchup::Group?`

Note: This method is not available in SketchUp Make.

**Remarks:** Note: This method is not available in SketchUp Make. The union method is used to compute the boolean union of the two groups representing manifold solid volumes (this | arg). If the specified objects (this and arg) do not represent manifold volumes, this method fails.

**Parameters:**
- `group` (Sketchup::Group, Sketchup::ComponentInstance) — The group to union this group with.

**Returns:** `Sketchup::Group, nil` — The resultant group if the two objects (this and arg) represent manifold solids and the operation succeeds. Otherwise nil is returned.

[Docs](https://ruby.sketchup.com/Sketchup/Group.html#union-instance_method)

#### `volume => Float`

The volume method is used to compute the volume of this group if and only if this group is manifold.

**Remarks:** The volume method is used to compute the volume of this group if and only if this group is manifold.

**Returns:** `Float` — If the group represents a manifold volume, volume will be a positive value. If volume is negative, the group is not manifold.

[Docs](https://ruby.sketchup.com/Sketchup/Group.html#volume-instance_method)

### Properties
- `description` (String, get/set) — The description method is used to retrieve the description for the group.
- `glued_to` (Sketchup::Face, Sketchup::Group, Sketchup::ComponentInstance, Sketchup::Image, nil, get/set) — The #glued_to method is used to retrieve the entity that this group is glued to.
- `locked` (Boolean, set) — The locked= method is used to lock a group.
- `name` (String, get/set) — The name method is used to retrieve the name of the group.
- `transformation` (Geom::Transformation, get/set) — The transformation method is used to retrieve the transformation for the group.

## Http (interface)

The Http module provides interfaces to create asynchronous HTTP requests. This is an alternative to the Net::Http module that comes with Ruby StdLib - which is known to have issues within SketchUp.

[Vendor docs](https://ruby.sketchup.com/Sketchup/Http.html)

### Properties
- `GET` (Object, get) — 
- `POST` (Object, get) — 
- `PUT` (Object, get) — 
- `DELETE` (Object, get) — 
- `HEAD` (Object, get) — 
- `OPTIONS` (Object, get) — 
- `PATCH` (Object, get) — 
- `STATUS_UNKNOWN` (Object, get) — 
- `STATUS_SUCCESS` (Object, get) — 
- `STATUS_PENDING` (Object, get) — 
- `STATUS_CANCELED` (Object, get) — 
- `STATUS_FAILED` (Object, get) — 

## Image (class)

An Image object represents a raster image placed in the Model.

[Vendor docs](https://ruby.sketchup.com/Sketchup/Image.html)

### Methods
#### `explode => Object`

The explode method is used to explode an image into a face with a texture on it.

**Remarks:** The explode method is used to explode an image into a face with a texture on it. Note that current versions of SketchUp will return an empty array here. To work around this limitation you can iterate over your entities collection to determine which new entities were created. Versions prior to SketchUp 2015 returned an empty array due to a bug.

**Returns:** `Object` — entitiesarray - an Array object of entities if successful

[Docs](https://ruby.sketchup.com/Sketchup/Image.html#explode-instance_method)

#### `image_rep => Sketchup::ImageRep`

The #image_rep method returns a copy of a Sketchup::ImageRep object representing the pixel data.

**Remarks:** The #image_rep method returns a copy of a Sketchup::ImageRep object representing the pixel data.

**Returns:** `Sketchup::ImageRep` — 

[Docs](https://ruby.sketchup.com/Sketchup/Image.html#image_rep-instance_method)

#### `normal => Object`

The normal method is used to retrieve the 3D Vector that is perpendicular to the plane of the image.

**Remarks:** The normal method is used to retrieve the 3D Vector that is perpendicular to the plane of the image.

**Returns:** `Object` — vector - a Vector3d object if successful

[Docs](https://ruby.sketchup.com/Sketchup/Image.html#normal-instance_method)

#### `path => Object`

The path method is used to retrieve the path of the file defining the image.

**Remarks:** The path method is used to retrieve the path of the file defining the image.

**Returns:** `Object` — path - the path for the image file if successful

[Docs](https://ruby.sketchup.com/Sketchup/Image.html#path-instance_method)

#### `pixelheight => Object`

The pixelheight method is used to retrieve the height of the image file in pixels.

**Remarks:** The pixelheight method is used to retrieve the height of the image file in pixels.

**Returns:** `Object` — height - the height of the image in pixels if successful

[Docs](https://ruby.sketchup.com/Sketchup/Image.html#pixelheight-instance_method)

#### `pixelwidth => Object`

The pixelwidth method is used to retrieve the width of the image file in pixels.

**Remarks:** The pixelwidth method is used to retrieve the width of the image file in pixels.

**Returns:** `Object` — width - the width of the image in pixels if successful

[Docs](https://ruby.sketchup.com/Sketchup/Image.html#pixelwidth-instance_method)

#### `transform!(transform) => Object`

The transform! method is used to apply a transformation to the image.

**Remarks:** The transform! method is used to apply a transformation to the image.

**Parameters:**
- `transform` (Object) — A Transformation object.

**Returns:** `Object` — image - the transformed Image object if successful

[Docs](https://ruby.sketchup.com/Sketchup/Image.html#transform!-instance_method)

#### `zrotation => Object`

The zrotation method is used to get the angle that the image is rotated about the normal vector from an arbitrary X axis.

**Remarks:** The zrotation method is used to get the angle that the image is rotated about the normal vector from an arbitrary X axis.

**Returns:** `Object` — vector - a Vector3d object if successful

[Docs](https://ruby.sketchup.com/Sketchup/Image.html#zrotation-instance_method)

### Properties
- `glued_to` (Sketchup::Face, Sketchup::Group, Sketchup::ComponentInstance, Sketchup::Image, nil, get/set) — The #glued_to method is used to retrieve the entity that this image is glued to.
- `height` (Object, get/set) — The height method is used to retrieve the height of the image.
- `origin` (Object, get/set) — The origin method is used to retrieve the 3D point that defines the origin of the image.
- `size` (Object, set) — The size= method is used to set the width and height of the image, in inches.
- `transformation` (Object, get/set) — The transformation method is used to retrieve the transformation for the image.
- `width` (Object, get/set) — The width method is used to retrieve the width of the image.

## ImageRep (class)

References an image representation object.

[Vendor docs](https://ruby.sketchup.com/Sketchup/ImageRep.html)

### Constructors
- `ImageRep(filepath)` — The #initialize method creates a new image object. The image object will have no data if a path to the image is not provided.

### Methods
#### `bits_per_pixel => Integer`

The #bits_per_pixel method gets the number of bits per pixel in the image.

**Remarks:** The #bits_per_pixel method gets the number of bits per pixel in the image.

**Returns:** `Integer` — 

[Docs](https://ruby.sketchup.com/Sketchup/ImageRep.html#bits_per_pixel-instance_method)

#### `color_at_uv(u, v, bilinear = false) => Sketchup::Color?`

The #color_at_uv method returns a color corresponding to the UV texture coordinates.

**Remarks:** The #color_at_uv method returns a color corresponding to the UV texture coordinates. 0.0, 0.0 maps to the bottom left and 1.0, 1.0 to the top right of the image.

**Parameters:**
- `u` (Float) — The U texture coordinate.
- `v` (Float) — The V texture coordinate.
- `bilinear` (Boolean) — Use bilinear texture filtering. This interpolates the colors instead of picking the nearest neighbor.

**Returns:** `Sketchup::Color, nil` — 

[Docs](https://ruby.sketchup.com/Sketchup/ImageRep.html#color_at_uv-instance_method)

#### `colors => Array<Sketchup::Color>?`

The #colors method returns an array of Color for each pixel in the image.

**Remarks:** The #colors method returns an array of Color for each pixel in the image.

**Returns:** `Array<Sketchup::Color>, nil` — 

[Docs](https://ruby.sketchup.com/Sketchup/ImageRep.html#colors-instance_method)

#### `data => String?`

Note: The byte order of the pixels are RGB(A) on macOS and BGR(A) on Windows.

**Remarks:** Note: The byte order of the pixels are RGB(A) on macOS and BGR(A) on Windows. The #data method gets the pixel data for an image in a string of bytes.

**Returns:** `String, nil` — 

[Docs](https://ruby.sketchup.com/Sketchup/ImageRep.html#data-instance_method)

#### `height => Integer`

The #height method returns the height of an image.

**Remarks:** The #height method returns the height of an image.

**Returns:** `Integer` — 

[Docs](https://ruby.sketchup.com/Sketchup/ImageRep.html#height-instance_method)

#### `load_file(filepath) => Object`

The #load_file method loads image data from the specified file.

**Remarks:** The #load_file method loads image data from the specified file.

**Parameters:**
- `filepath` (String)

[Docs](https://ruby.sketchup.com/Sketchup/ImageRep.html#load_file-instance_method)

#### `row_padding => Integer`

The #row_padding method returns the size of the row padding of an image in bytes.

**Remarks:** The #row_padding method returns the size of the row padding of an image in bytes.

**Returns:** `Integer` — 

[Docs](https://ruby.sketchup.com/Sketchup/ImageRep.html#row_padding-instance_method)

#### `save_file(filepath) => Object`

The #save_file method saves an image data object to an image file specified by a path.

**Remarks:** The #save_file method saves an image data object to an image file specified by a path.

**Parameters:**
- `filepath` (String)

[Docs](https://ruby.sketchup.com/Sketchup/ImageRep.html#save_file-instance_method)

#### `set_data(width, height, bits_per_pixel, row_padding, pixel_data) => Sketchup::ImageRep`

Note: The byte order of the pixels are RGB(A) on macOS and BGR(A) on Windows.

**Remarks:** Note: The byte order of the pixels are RGB(A) on macOS and BGR(A) on Windows. Note: The encoding of the pixel_data String parameter should be ASCII-8BIT. Any other encoding could corrupt the binary data. Using Array#pack(“C*”) gives correct encoding. The #set_data method discards any existing data and sets new pixel data for the Sketchup::ImageRep.

**Parameters:**
- `width` (Integer) — The width of the pixel data. Must be greater than 0.
- `height` (Integer) — The height of the pixel data. Must be greater than 0.
- `bits_per_pixel` (Integer) — The bits per pixel for the pixel data. Must be either 8/24/32.
- `row_padding` (Integer) — The row padding for the pixel data which is sized in bytes. Row padding is used to pad each row with zeros so that each scanline on the pixel data will end on the data-type boundary.
- `pixel_data` (String) — The binary string containing the pixel data representing the new image.

**Returns:** `Sketchup::ImageRep` — 

[Docs](https://ruby.sketchup.com/Sketchup/ImageRep.html#set_data-instance_method)

#### `size => Integer`

The #size method gets the total size of the image data in bytes.

**Remarks:** The #size method gets the total size of the image data in bytes.

**Returns:** `Integer` — 

[Docs](https://ruby.sketchup.com/Sketchup/ImageRep.html#size-instance_method)

#### `width => Integer`

The #width method returns the width of an image.

**Remarks:** The #width method returns the width of an image.

**Returns:** `Integer` — 

[Docs](https://ruby.sketchup.com/Sketchup/ImageRep.html#width-instance_method)

## Importer (class)

The Importer interface lets you build your own importers for SketchUp. To use this, you create a subclass of Importer and implement all of the methods described below. This will make your importer appear in the list that users see under File > Import, and you can use Ruby to do all of the work of opening the file and creating whatever you need inside SketchUp. Here is an example of a complete script that imports a .txt file and displays its contents in a messagebox.

[Vendor docs](https://ruby.sketchup.com/Sketchup/Importer.html)

### Methods
#### `description => Object`

This method is called by SketchUp to determine the description that appears in the File > Import dialog's pulldown list of valid importers.

**Remarks:** This method is called by SketchUp to determine the description that appears in the File > Import dialog's pulldown list of valid importers. Though it is common for the description to include the file extension supported by the importer (such as “Text Importer (.txt)”), the actual extension is defined in the file_extension method.

**Returns:** `Object` — description - a brief string description

[Docs](https://ruby.sketchup.com/Sketchup/Importer.html#description-instance_method)

#### `do_options => Object`

This method is called by SketchUp when the user clicks on the “Options” button inside the File > Import dialog.

**Remarks:** This method is called by SketchUp when the user clicks on the “Options” button inside the File > Import dialog. You can use it to gather and store settings for your importer. Only applicable if the importer supports options, meaning its supports_options method returns true.

**Returns:** `Object` — id - an id string

[Docs](https://ruby.sketchup.com/Sketchup/Importer.html#do_options-instance_method)

#### `file_extension => Object`

This method is called by SketchUp to determine a single file extension is associated with your importer.

**Remarks:** This method is called by SketchUp to determine a single file extension is associated with your importer. Only files that match this extension will be shown to the user as they browse their harddrive for things to import. Ruby importers are only allowed to support a single extension.

**Returns:** `Object` — extension - typically a 3-letter string

[Docs](https://ruby.sketchup.com/Sketchup/Importer.html#file_extension-instance_method)

#### `id => Object`

This method is called by SketchUp to determine a unique identifier for your importer, typically something like “com.

**Remarks:** This method is called by SketchUp to determine a unique identifier for your importer, typically something like “com.sketchup.importers.dxf”.

**Returns:** `Object` — id - an id string

[Docs](https://ruby.sketchup.com/Sketchup/Importer.html#id-instance_method)

#### `load_file(file_path, status) => Object`

This method is called by SketchUp after the user has selected a file to import.

**Remarks:** This method is called by SketchUp after the user has selected a file to import. This is where you do the real work by opening the file via Ruby's File object and processing it in whatever way you need. You must return an integer success code to SketchUp when you are done. These are the codes that SketchUp understands:

**Parameters:**
- `file_path` (Object) — Absolute path to the file the user selected
- `status` (Object) — The status of the import so far. Contains true.

**Returns:** `Object` — success - an integer status code. See above.

[Docs](https://ruby.sketchup.com/Sketchup/Importer.html#load_file-instance_method)

#### `supports_options? => Boolean`

This method is called by SketchUp to determine if the “Options” button inside the File > Import dialog should be enabled while your importer is selected.

**Remarks:** This method is called by SketchUp to determine if the “Options” button inside the File > Import dialog should be enabled while your importer is selected.

**Returns:** `Boolean` — supports_options - a boolean

[Docs](https://ruby.sketchup.com/Sketchup/Importer.html#supports_options?-instance_method)

### Properties
- `ImportSuccess` (Object, get) — 
- `ImportFail` (Object, get) — 
- `ImportCanceled` (Object, get) — 
- `ImportFileNotFound` (Object, get) — 

## InputPoint (class)

The InputPoint class is used to pick 3d points and/or entities that reside under the current cursor location, similar to native Line tool and other drawing tools. Unlike PickHelper, InputPoint uses inference, i.e. “snaps” to vertices and other entities when the cursor is close to them. Only Tools react to cursor location and most of these methods are only useful in the context of a tool. For example, if you want to determine the 3d point you just moved the cursor over, you would use #pick from within your Tool#onMouseMove method. InputPoints are best picked from mouse move, as you want them to draw them to the view. For an example, see Tool Example. To lock inference similar to native SketchUp tools, see View#lock_inference.

[Vendor docs](https://ruby.sketchup.com/Sketchup/InputPoint.html)

### Constructors
- `InputPoint(pt_or_vertex)` — Note: Prior to SketchUp 2019 it was not possible to sub-class Sketchup::InputPoint due to a bug in how SketchUp initialized the class. The new method is used to create a new InputPoint object.

### Methods
#### `clear => Object`

The clear method is used to clear the input point.

**Remarks:** The clear method is used to clear the input point. This sets it to an empty state. After calling this, valid? will return false.

**Returns:** `Object` — inputpoint - the cleared (empty) input point if this successful

[Docs](https://ruby.sketchup.com/Sketchup/InputPoint.html#clear-instance_method)

#### `copy!(inputpoint) => Object`

The copy! method is used to copy the data from a second input point into this input point.

**Remarks:** The copy! method is used to copy the data from a second input point into this input point.

**Parameters:**
- `inputpoint` (Object) — The second input point.

**Returns:** `Object` — inputpoint - the new input point that received the copy if successful

[Docs](https://ruby.sketchup.com/Sketchup/InputPoint.html#copy!-instance_method)

#### `degrees_of_freedom => Object`

The degrees_of_freedom method retrieves the number of degrees of freedom there are for an input point.

**Remarks:** The degrees_of_freedom method retrieves the number of degrees of freedom there are for an input point. If you are just getting a point in space, then the degrees_of_freedom will be 3 - meaning that there is nothing about the point that would constrain its position. If you are on a face, then the degrees_of_freedom will be 2 meaning that you can only move on the plane of the face. If you are on an Edge or an axis, then the degrees_of_freedom will be 1 meaning that you can only move in the direction of the edge or axis. If you get an end point of an Edge, or an intersection point, then the degrees_of_freedom will be 0.

**Returns:** `Object` — degrees_of_freedom - see comments.

[Docs](https://ruby.sketchup.com/Sketchup/InputPoint.html#degrees_of_freedom-instance_method)

#### `depth => Object`

The depth method retrieves the depth of an inference if it is coming from a component.

**Remarks:** The depth method retrieves the depth of an inference if it is coming from a component. If the InputPoint is not getting a position from inside a component, this method will return 0. Otherwise it returns the depth of the entity in a nested component that is providing the position.

**Returns:** `Object` — depth - a number representing the depth of the inputpoint (inside groups and components) if successful

[Docs](https://ruby.sketchup.com/Sketchup/InputPoint.html#depth-instance_method)

#### `display? => Boolean`

The display? method is used to determine if the input point has anything to draw.

**Remarks:** The display? method is used to determine if the input point has anything to draw. If the method returns true, then the draw method will draw something.

**Returns:** `Boolean` — status - true if the draw method will draw something, false if the draw method has nothing to draw

[Docs](https://ruby.sketchup.com/Sketchup/InputPoint.html#display?-instance_method)

#### `draw(view) => Object`

The draw method is used to draw the input point.

**Remarks:** The draw method is used to draw the input point. This is useful for showing an InputPoint from within the draw method of a tool that you have implemented in Ruby. Additional examples are available in the Plugins/examples directory.

**Parameters:**
- `view` (Object) — The current view.

**Returns:** `Object` — view

[Docs](https://ruby.sketchup.com/Sketchup/InputPoint.html#draw-instance_method)

#### `edge => Object`

The edge method is used to retrieve the edge if the input point is getting its position from a point on an Edge.

**Remarks:** The edge method is used to retrieve the edge if the input point is getting its position from a point on an Edge.

**Returns:** `Object` — edge - an Edge object if successful, or nil if unsuccessful

[Docs](https://ruby.sketchup.com/Sketchup/InputPoint.html#edge-instance_method)

#### `face => Sketchup::Face?`

Note: The InputPoint doesn't necessarily lie on the face, but can be e.

**Remarks:** Note: The InputPoint doesn't necessarily lie on the face, but can be e.g. on an edge in front of the face. The face method retrieves the face at or behind the input point. This can be used to determine a plane, similar to what native Rotate tool does.

**Returns:** `Sketchup::Face, nil` — 

[Docs](https://ruby.sketchup.com/Sketchup/InputPoint.html#face-instance_method)

#### `instance_path => Sketchup::InstancePath?`

The #instance_path method retrieves the instance path for the picked point.

**Remarks:** The #instance_path method retrieves the instance path for the picked point. The returned instance_path is a copy of what the input point is holding on to at the moment you access it. Your copy will not update if you make new picks with the input point. If there has been no valid pick it will return `nil`.

**Returns:** `Sketchup::InstancePath, nil` — 

[Docs](https://ruby.sketchup.com/Sketchup/InputPoint.html#instance_path-instance_method)

#### `pick(view, x, y) => Boolean | pick(view, x, y, inputpoint) => Boolean | pick(view, x, y) => Boolean | pick(view, x, y, inputpoint) => Boolean`

The #pick method is used to get a input point at a specific screen position.

**Remarks:** The #pick method is used to get a input point at a specific screen position.

**Parameters:**
- `view` (Sketchup::View)
- `x` (Integer) — Screen coordinate in physical pixels.
- `y` (Integer) — Screen coordinate in physical pixels.

**Returns:** `Boolean` — true if a valid input point was picked and it is different than it was before.

[Docs](https://ruby.sketchup.com/Sketchup/InputPoint.html#pick-instance_method)

#### `position => Object`

The position method is used to get the 3D point from the input point.

**Remarks:** The position method is used to get the 3D point from the input point.

**Returns:** `Object` — point - a Point3d object position for the input point if successful

[Docs](https://ruby.sketchup.com/Sketchup/InputPoint.html#position-instance_method)

#### `tooltip => Object`

The tooltip method is used to retrieve the string that is the tool tip to display for the input point.

**Remarks:** The tooltip method is used to retrieve the string that is the tool tip to display for the input point.

**Returns:** `Object` — tip - a string tooltip or an empty string (if the input point doesn't provide a tooltip).

[Docs](https://ruby.sketchup.com/Sketchup/InputPoint.html#tooltip-instance_method)

#### `transformation => Object`

The transformation method retrieves the Transformation object for the input point.

**Remarks:** The transformation method retrieves the Transformation object for the input point. If the InputPoint object is getting its position from something inside of a component instance, this method returns the Transformation of the component instance. Otherwise it returns the identity Transformation. Note that the position method on a input point always returns a point that is transformed into model space. If you are using the edge, face or vertex method on the InputPoint though, you will probably need to use the transformation method to transform the data that you get back from the selected entity.

**Returns:** `Object` — transformation - the Transformation for the input point if successful

[Docs](https://ruby.sketchup.com/Sketchup/InputPoint.html#transformation-instance_method)

#### `valid? => Boolean`

The valid? method is used to determine if an input point has valid data.

**Remarks:** The valid? method is used to determine if an input point has valid data. You must have called the pick method to set the data before it is valid.

**Returns:** `Boolean` — status - true if the input point has valid data, false if it does not have valid data.

[Docs](https://ruby.sketchup.com/Sketchup/InputPoint.html#valid?-instance_method)

#### `vertex => Object`

The vertex method returns a Vertex associated with the InputPoint.

**Remarks:** The vertex method returns a Vertex associated with the InputPoint. If the InputPoint is on the end of a line, then it will return the Vertex. If the InputPoint does not select any vertices this method returns nil.

**Returns:** `Object` — vertex - The associated vertex

[Docs](https://ruby.sketchup.com/Sketchup/InputPoint.html#vertex-instance_method)

### Properties
- `=` (Object, set) — The == method is used to determine if two input points are the same.

## InstanceObserver (class)

This class is abstract. To implement this observer, create a Ruby class of this type, override the desired methods, and add an instance of the observer to the objects of interests. This observer interface is implemented to react to component instance events. Note that you may also attach InstanceObservers to Groups.

[Vendor docs](https://ruby.sketchup.com/Sketchup/InstanceObserver.html)

### Methods
#### `onClose(instance) => nil`

The #onClose method is called when an instance is “closed,” meaning an end user was editing a component's geometry and then exited back into the parent's editing space.

**Remarks:** The #onClose method is called when an instance is “closed,” meaning an end user was editing a component's geometry and then exited back into the parent's editing space.

**Parameters:**
- `instance` (Sketchup::ComponentInstance) — The instance that was just closed

**Returns:** `nil` — 

[Docs](https://ruby.sketchup.com/Sketchup/InstanceObserver.html#onClose-instance_method)

#### `onOpen(instance) => nil`

The #onOpen method is called when an instance is “opened,” meaning an end user has double clicked on it to edit its geometry.

**Remarks:** The #onOpen method is called when an instance is “opened,” meaning an end user has double clicked on it to edit its geometry. This is particularly useful if your plugin is dynamically drawing geometry or performing transformations in global space, since when in “edit component” mode all transformations and positions are returned in relation to the current component's origin. This method will tell you when a user has entered edit mode, and you can then use Model#active_path and Model#edit_transform methods to determine any corrections you need to make to your transformations.

**Parameters:**
- `instance` (Sketchup::ComponentInstance) — The instance that was opened

**Returns:** `nil` — 

[Docs](https://ruby.sketchup.com/Sketchup/InstanceObserver.html#onOpen-instance_method)

## InstancePath (class)

The InstancePath class represent the instance path to a given entity within the model hierarchy.

[Vendor docs](https://ruby.sketchup.com/Sketchup/InstancePath.html)

### Constructors
- `InstancePath(path)` — 

### Methods
#### `[](index) => Sketchup::Entity`

Note: This method does not accept negative indices.

**Remarks:** Note: This method does not accept negative indices. For the exact behavior of an array, use {#to_a}. The elements of an instance path can be accessed similarly to an array.

**Parameters:**
- `index` (Integer)

**Returns:** `Sketchup::Entity` — 

[Docs](https://ruby.sketchup.com/Sketchup/InstancePath.html#[]-instance_method)

#### `each {|entity| ... } => nil`

The yielded entities will start with the root and end with the leaf.

**Remarks:** The yielded entities will start with the root and end with the leaf.

**Returns:** `nil` — 

[Docs](https://ruby.sketchup.com/Sketchup/InstancePath.html#each-instance_method)

#### `empty? => Boolean`



**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Sketchup/InstancePath.html#empty?-instance_method)

#### `include?(object) => Boolean`

Returns `true` if the instance path contain the given object.

**Remarks:** Returns `true` if the instance path contain the given object.

**Parameters:**
- `object` (Object)

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Sketchup/InstancePath.html#include?-instance_method)

#### `leaf => Sketchup::Entity?`

The leaf of an instance path is the last element which can be any entity that can be represented in the model.

**Remarks:** The leaf of an instance path is the last element which can be any entity that can be represented in the model. This is normally a Drawingelement, but could be a Vertex. An instance can also be a leaf.

**Returns:** `Sketchup::Entity, nil` — Nil if the last item of the instance path is a group or component, otherwise Entity.

[Docs](https://ruby.sketchup.com/Sketchup/InstancePath.html#leaf-instance_method)

#### `length => Integer`

#length is an alias of #size.

**Remarks:** #length is an alias of #size.

**Returns:** `Integer` — 

[Docs](https://ruby.sketchup.com/Sketchup/InstancePath.html#length-instance_method)

#### `persistent_id_path => String`

The serialized version of an instance path is the persistent ids of its entities concatenated with a period.

**Remarks:** The serialized version of an instance path is the persistent ids of its entities concatenated with a period.

**Returns:** `String` — 

[Docs](https://ruby.sketchup.com/Sketchup/InstancePath.html#persistent_id_path-instance_method)

#### `root => Sketchup::Group, ...`

The root of an instance path is the element located closest to the model root.

**Remarks:** The root of an instance path is the element located closest to the model root. This will be a group or component instance. If you have a non-instance as a leaf with no other parent component this will return `nil`.

**Returns:** `Sketchup::Group, Sketchup::ComponentInstance, Sketchup::Image, nil` — 

[Docs](https://ruby.sketchup.com/Sketchup/InstancePath.html#root-instance_method)

#### `size => Integer`



**Returns:** `Integer` — 

[Docs](https://ruby.sketchup.com/Sketchup/InstancePath.html#size-instance_method)

#### `to_a => Array`

Returns an array representing the instance path.

**Remarks:** Returns an array representing the instance path.

**Returns:** `Array` — an array representing the instance path.

[Docs](https://ruby.sketchup.com/Sketchup/InstancePath.html#to_a-instance_method)

#### `transformation => Geom::Transformation | transformation(index) => Geom::Transformation`



**Parameters:**
- `index` (Integer)

**Returns:** `Geom::Transformation` — the combined transformation up to the the leaf entity.

[Docs](https://ruby.sketchup.com/Sketchup/InstancePath.html#transformation-instance_method)

#### `valid? => Boolean`

An instance path is valid if it has at least one element and consist of groups and instances with exception of the leaf which can be any entity.

**Remarks:** An instance path is valid if it has at least one element and consist of groups and instances with exception of the leaf which can be any entity. This method doesn't check if the path can actually be looked up in the model.

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Sketchup/InstancePath.html#valid?-instance_method)

### Properties
- `=` (Object, set) — Returns `true` if the instances paths represent the same set of entities.

## Layer (class)

Note: As of SketchUp 2020 “Layers” were renamed to “Tags” in the UI. The API retains the use of “Layer” for compatibility and is synonymous with “Tag”. The Layer class contains methods modifying and extracting information for a layer. By default, a SketchUp model has one layer, Layer 0 (Named “Untagged” in the UI since SketchUp 2020), which is the base layer. You can't delete or rename Layer 0. Unlike certain other CAD software packages, entities associated with different layers in SketchUp still intersect with each other. (If you want collections of entities to not intersect, place them in Groups instead.) Layers are commonly used to organize your model and control the visibility of related groups and components. For example, you could make all of your wall and roof entities different groups, associate layers with those groups, and then hide those layers so as to display just the floor plan in the model.

[Vendor docs](https://ruby.sketchup.com/Sketchup/Layer.html)

### Methods
#### `<=>(layer2) => Integer`

The #<=> method is used to compare two layers based on their names.

**Remarks:** The #<=> method is used to compare two layers based on their names. This enables the Ruby Array#sort method to sort SketchUp layers.

**Parameters:**
- `layer2` (Sketchup::Layer)

**Returns:** `Integer` — -1 if layer1 is less than layer2. 1 if layer2 is less than layer1. 0 if layer1 and layer2 are equal.

[Docs](https://ruby.sketchup.com/Sketchup/Layer.html#<=>-instance_method)

#### `display_name => String`

Note: The display name and internal name of layers should share the same value except for the Layer0.

**Remarks:** Note: The display name and internal name of layers should share the same value except for the Layer0. From version 2020.0 onwards the display name of Layer0 is “Untagged” and it is localized. The #display_name method is used to retrieve the display name of the layer. This is the name shown to the user in the Sketchup UI.

**Returns:** `String` — 

[Docs](https://ruby.sketchup.com/Sketchup/Layer.html#display_name-instance_method)

#### `visible? => Boolean`

The #visible? method is used to determine if the layer is visible.

**Remarks:** The #visible? method is used to determine if the layer is visible.

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Sketchup/Layer.html#visible?-instance_method)

### Properties
- `=` (Object, set) — The #== method is used to determine if two layers are the same.
- `color` (Sketchup::Color, get/set) — The #color method is used to retrieve the color of the layer.
- `folder` (Sketchup::LayerFolder, nil, get/set) — The #folder method is used to return the parent layer folder of a layer.
- `line_style` (Sketchup::LineStyle, nil, get/set) — The #line_style method retrieves the line style on this layer.
- `name` (String, get/set) — Note: The internal layer and display name of layers should share the same value except for the Layer0.
- `page_behavior` (Integer, get/set) — The #page_behavior method is used to retrieve the visibility behavior of the layer for new pages and existing pages.
- `visible` (Boolean, set) — The #visible= method is used to set if the layer is visible.

## LayerFolder (class)

Note: As of SketchUp 2020 “Layers” were renamed to “Tags” in the UI. The API retains the use of “Layer” for compatibility and is synonymous with “Tag”. Allows layers to be organized in folders. Folders may have duplicate names.

[Vendor docs](https://ruby.sketchup.com/Sketchup/LayerFolder.html)

### Methods
#### `<=>(other) => Integer?`

The #<=> method is used to compare two layer folders based on their names.

**Remarks:** The #<=> method is used to compare two layer folders based on their names. This enables the Ruby Array#sort method to sort SketchUp layer folders.

**Parameters:**
- `other` (Object)

**Returns:** `Integer, nil` — -1 if the receiver should appear before other. 1 if the receiver should appear after other. 0 if the receiver and other are equal. nil if other is not comparable with the receiver.

[Docs](https://ruby.sketchup.com/Sketchup/LayerFolder.html#<=>-instance_method)

#### `add_folder(name) => Sketchup::LayerFolder | add_folder(folder) => Sketchup::LayerFolder`

The #add_folder method adds or moves a layer folder.

**Remarks:** The #add_folder method adds or moves a layer folder.

**Parameters:**
- `name` (String)

**Returns:** `Sketchup::LayerFolder` — 

[Docs](https://ruby.sketchup.com/Sketchup/LayerFolder.html#add_folder-instance_method)

#### `add_layer(layer) => nil`

The #add_layer method adds a layer to a folder.

**Remarks:** The #add_layer method adds a layer to a folder.

**Parameters:**
- `layer` (Sketchup::Layer)

**Returns:** `nil` — 

[Docs](https://ruby.sketchup.com/Sketchup/LayerFolder.html#add_layer-instance_method)

#### `count_folders => Integer`

The #count_folders method retrieves the number of child folders in the folder.

**Remarks:** The #count_folders method retrieves the number of child folders in the folder.

**Returns:** `Integer` — 

[Docs](https://ruby.sketchup.com/Sketchup/LayerFolder.html#count_folders-instance_method)

#### `count_layers => Integer Also known as: length, size`

The #count_layers method retrieves the number of layers in the folder.

**Remarks:** The #count_layers method retrieves the number of layers in the folder.

**Returns:** `Integer` — 

[Docs](https://ruby.sketchup.com/Sketchup/LayerFolder.html#count_layers-instance_method)

#### `each_folder {|folder| ... } => Object`

The #each_folder method is used to iterate through the folders that are direct children to the folder.

**Remarks:** The #each_folder method is used to iterate through the folders that are direct children to the folder.

[Docs](https://ruby.sketchup.com/Sketchup/LayerFolder.html#each_folder-instance_method)

#### `each_layer {|layer| ... } => Object Also known as: each`

The #each_layer method is used to iterate through the layers that are direct children to the folder.

**Remarks:** The #each_layer method is used to iterate through the layers that are direct children to the folder.

[Docs](https://ruby.sketchup.com/Sketchup/LayerFolder.html#each_layer-instance_method)

#### `folders => Array<Sketchup::LayerFolder>`

The #folders returns the direct child-folders of the folder.

**Remarks:** The #folders returns the direct child-folders of the folder.

**Returns:** `Array<Sketchup::LayerFolder>` — 

[Docs](https://ruby.sketchup.com/Sketchup/LayerFolder.html#folders-instance_method)

#### `layers => Array<Sketchup::Layer>`

The #layers method retrieves the child layers of a folder.

**Remarks:** The #layers method retrieves the child layers of a folder.

**Returns:** `Array<Sketchup::Layer>` — 

[Docs](https://ruby.sketchup.com/Sketchup/LayerFolder.html#layers-instance_method)

#### `remove_folder(folder) => nil`

The #remove_folder method removes the folder from the model.

**Remarks:** The #remove_folder method removes the folder from the model. All children are preserved, but move up one level.

**Parameters:**
- `folder` (Sketchup::LayerFolder)

**Returns:** `nil` — 

[Docs](https://ruby.sketchup.com/Sketchup/LayerFolder.html#remove_folder-instance_method)

#### `remove_layer(layer) => nil`

The #remove_layer method removes a layer from a folder.

**Remarks:** The #remove_layer method removes a layer from a folder. The layer will be parent to the layer manager.

**Parameters:**
- `layer` (Sketchup::Layer)

**Returns:** `nil` — 

[Docs](https://ruby.sketchup.com/Sketchup/LayerFolder.html#remove_layer-instance_method)

#### `visible? => Boolean`

The #visible? method is used to determine if the layer folder is visible.

**Remarks:** The #visible? method is used to determine if the layer folder is visible.

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Sketchup/LayerFolder.html#visible?-instance_method)

#### `visible_on_new_pages? => Boolean`

The #visible_on_new_pages? method is used to determine if the layer folder is by default visible on new pages.

**Remarks:** The #visible_on_new_pages? method is used to determine if the layer folder is by default visible on new pages.

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Sketchup/LayerFolder.html#visible_on_new_pages?-instance_method)

### Properties
- `=` (Object, set) — The #== method is used to determine if two layer folders are the same.
- `folder` (Sketchup::LayerFolder, nil, get/set) — The #folder method is used to return the parent layer folder of a layer folder.
- `name` (String, get/set) — The #name method gets the name of the folder.
- `visible` (Boolean, set) — The #visible= method is used to set if the layer folder is visible.
- `visible_on_new_pages` (Boolean, set) — The #visible_on_new_pages= method is used to set if the layer folder is by default visible on new pages.

## Layers (class)

Note: As of SketchUp 2020 “Layers” were renamed to “Tags” in the UI. The API retains the use of “Layer” for compatibility and is synonymous with “Tag”. The Layers collection allows you to see and manage all of the layers in a model. You get a pointer to the Layers object from within the Model.

[Vendor docs](https://ruby.sketchup.com/Sketchup/Layers.html)

### Methods
#### `[](index_or_name) => Sketchup::Layer?`

The #[] method is used to retrieve a layer by index or name.

**Remarks:** The #[] method is used to retrieve a layer by index or name.

**Parameters:**
- `index_or_name` (Integer, String) — A number representing the layer's index in an array of Layer objects, or the name of the layer.

**Returns:** `Sketchup::Layer, nil` — 

[Docs](https://ruby.sketchup.com/Sketchup/Layers.html#[]-instance_method)

#### `add(layer_name) => Sketchup::Layer Also known as: add_layer`

The #add method is used to add a new layer.

**Remarks:** The #add method is used to add a new layer. If you give the name of a Layer that is already defined, it will return the existing Layer rather than adding a new one.

**Parameters:**
- `layer_name` (String) — The name of the added layer.

**Returns:** `Sketchup::Layer` — 

[Docs](https://ruby.sketchup.com/Sketchup/Layers.html#add-instance_method)

#### `add_folder(name) => Sketchup::LayerFolder | add_folder(folder) => Sketchup::LayerFolder`

The #add_folder method adds or moves a layer folder.

**Remarks:** The #add_folder method adds or moves a layer folder.

**Parameters:**
- `name` (String)

**Returns:** `Sketchup::LayerFolder` — 

[Docs](https://ruby.sketchup.com/Sketchup/Layers.html#add_folder-instance_method)

#### `add_observer(observer) => Boolean`

The #add_observer method is used to add an observer to the layers collection.

**Remarks:** The #add_observer method is used to add an observer to the layers collection.

**Parameters:**
- `observer` (Sketchup::LayersObserver)

**Returns:** `Boolean` — true if successful, false if unsuccessful.

[Docs](https://ruby.sketchup.com/Sketchup/Layers.html#add_observer-instance_method)

#### `at(index_or_name) => Sketchup::Layer?`

The #at method is an alias for #[].

**Remarks:** The #at method is an alias for #[].

**Returns:** `Sketchup::Layer, nil` — 

[Docs](https://ruby.sketchup.com/Sketchup/Layers.html#at-instance_method)

#### `count => Object`

Note: Since SketchUp 2014 the count method is inherited from Ruby's Enumerable mix-in module.

**Remarks:** Note: Since SketchUp 2014 the count method is inherited from Ruby's Enumerable mix-in module. Prior to that the #count method is an alias for #length. Returns integer - the number of layers in the collection.

**Returns:** `Object` — integer - the number of layers in the collection

[Docs](https://ruby.sketchup.com/Sketchup/Layers.html#count-instance_method)

#### `count_folders => Integer`

The #count_folders method counts the number of folders which are direct children of the layer manager.

**Remarks:** The #count_folders method counts the number of folders which are direct children of the layer manager.

**Returns:** `Integer` — 

[Docs](https://ruby.sketchup.com/Sketchup/Layers.html#count_folders-instance_method)

#### `count_layers => Integer`

The #count_layers method retrieves the number of layers not in a folder.

**Remarks:** The #count_layers method retrieves the number of layers not in a folder.

**Returns:** `Integer` — 

[Docs](https://ruby.sketchup.com/Sketchup/Layers.html#count_layers-instance_method)

#### `each {|layer| ... } => Object`

Note: Don't remove content from this collection while iterating over it with #each.

**Remarks:** Note: Don't remove content from this collection while iterating over it with #each. This would change the size of the collection and cause elements to be skipped as the indices change. Instead copy the current collection to an array using to_a and then use each on the array, when removing content. The #each method is used to iterate through all of the layers in the model. This include layers that are nested inside folders.

[Docs](https://ruby.sketchup.com/Sketchup/Layers.html#each-instance_method)

#### `each_folder {|folder| ... } => Object`

The #each_folder method is used to iterate through the folders that are direct children to the layer manager.

**Remarks:** The #each_folder method is used to iterate through the folders that are direct children to the layer manager.

[Docs](https://ruby.sketchup.com/Sketchup/Layers.html#each_folder-instance_method)

#### `each_layer {|layer| ... } => Object`

The #each_layer method is used to iterate through the layers that are not inside a layer folder.

**Remarks:** The #each_layer method is used to iterate through the layers that are not inside a layer folder.

[Docs](https://ruby.sketchup.com/Sketchup/Layers.html#each_layer-instance_method)

#### `folders => Array<Sketchup::LayerFolder>`

Note: This does not return all the folders in the model, only those that are direct children of the layer manager.

**Remarks:** Note: This does not return all the folders in the model, only those that are direct children of the layer manager. The #folders method returns the folders of the layer manager.

**Returns:** `Array<Sketchup::LayerFolder>` — 

[Docs](https://ruby.sketchup.com/Sketchup/Layers.html#folders-instance_method)

#### `layers => Array<Sketchup::Layer>`

The #layers method retrieves the layers not in a folder.

**Remarks:** The #layers method retrieves the layers not in a folder.

**Returns:** `Array<Sketchup::Layer>` — 

[Docs](https://ruby.sketchup.com/Sketchup/Layers.html#layers-instance_method)

#### `length => Integer`

The #length method retrieves the number of layers.

**Remarks:** The #length method retrieves the number of layers.

**Returns:** `Integer` — 

[Docs](https://ruby.sketchup.com/Sketchup/Layers.html#length-instance_method)

#### `purge_unused => Integer Also known as: purge_unused_layers`

The #purge_unused method is used to remove unused layers.

**Remarks:** The #purge_unused method is used to remove unused layers.

**Returns:** `Integer` — Number of unused layers removed

[Docs](https://ruby.sketchup.com/Sketchup/Layers.html#purge_unused-instance_method)

#### `purge_unused_folders {|folder| ... } => Object`

The #purge_unused_folders method is used to remove all layer folder with no children.

**Remarks:** The #purge_unused_folders method is used to remove all layer folder with no children.

[Docs](https://ruby.sketchup.com/Sketchup/Layers.html#purge_unused_folders-instance_method)

#### `remove(layer, remove_geometry = false) => Boolean Also known as: remove_layer`

Remove the given layer from the model, optionally removing the geometry.

**Remarks:** Remove the given layer from the model, optionally removing the geometry.

**Parameters:**
- `layer` (Sketchup::Layer, Integer, String)
- `remove_geometry` (Boolean) — If true, geometry in the removed layer will be removed as well. If false (which is the default), this geometry will be placed on Layer 0.

**Returns:** `Boolean` — true if successful, false if unsuccessful.

[Docs](https://ruby.sketchup.com/Sketchup/Layers.html#remove-instance_method)

#### `remove_folder(folder) => nil`

The #remove_folder method removes the folder from the model.

**Remarks:** The #remove_folder method removes the folder from the model. All children are preserved, but moved up one level.

**Parameters:**
- `folder` (Sketchup::LayerFolder)

**Returns:** `nil` — 

[Docs](https://ruby.sketchup.com/Sketchup/Layers.html#remove_folder-instance_method)

#### `remove_observer(observer) => Boolean`

The #remove_observer method is used to remove an observer from the current object.

**Remarks:** The #remove_observer method is used to remove an observer from the current object.

**Parameters:**
- `observer` (Sketchup::LayersObserver)

**Returns:** `Boolean` — true if successful, false if unsuccessful.

[Docs](https://ruby.sketchup.com/Sketchup/Layers.html#remove_observer-instance_method)

#### `size => Integer`

The #size method is an alias of #length.

**Remarks:** The #size method is an alias of #length.

**Returns:** `Integer` — 

[Docs](https://ruby.sketchup.com/Sketchup/Layers.html#size-instance_method)

#### `unique_name => String | unique_name(base_name) => String`

The #unique_name method can be used to get a string that will be a unique layer name inside this collection.

**Remarks:** The #unique_name method can be used to get a string that will be a unique layer name inside this collection.

**Parameters:**
- `base_name` (String) — The base name to build the unique name from.

**Returns:** `String` — Will default to using “Layer” (SketchUp2019 and older) or “Tag” as basename for a unique name.

[Docs](https://ruby.sketchup.com/Sketchup/Layers.html#unique_name-instance_method)

## LayersObserver (class)

This class is abstract. To implement this observer, create a Ruby class of this type, override the desired methods, and add an instance of the observer to the objects of interests. This observer interface is implemented to react to layers events.

[Vendor docs](https://ruby.sketchup.com/Sketchup/LayersObserver.html)

### Methods
#### `onCurrentLayerChanged(layers, layer) => nil`

The #onCurrentLayerChanged method is called when the user selects a different active layer.

**Remarks:** The #onCurrentLayerChanged method is called when the user selects a different active layer.

**Parameters:**
- `layers` (Sketchup::Layers)
- `layer` (Sketchup::Layer)

**Returns:** `nil` — 

[Docs](https://ruby.sketchup.com/Sketchup/LayersObserver.html#onCurrentLayerChanged-instance_method)

#### `onLayerAdded(layers, layer) => nil`

The #onLayerAdded method is called when a new layer is added to a model.

**Remarks:** The #onLayerAdded method is called when a new layer is added to a model.

**Parameters:**
- `layers` (Sketchup::Layers)
- `layer` (Sketchup::Layer)

**Returns:** `nil` — 

[Docs](https://ruby.sketchup.com/Sketchup/LayersObserver.html#onLayerAdded-instance_method)

#### `onLayerChanged(layers, layer) => nil`

The #onLayerChanged method is called when a layer is changed.

**Remarks:** The #onLayerChanged method is called when a layer is changed.

**Parameters:**
- `layers` (Sketchup::Layers)
- `layer` (Sketchup::Layer)

**Returns:** `nil` — 

[Docs](https://ruby.sketchup.com/Sketchup/LayersObserver.html#onLayerChanged-instance_method)

#### `onLayerFolderAdded(layers, layer_folder) => nil`

The #onLayerFolderAdded method is called when a layer folder is added to a model.

**Remarks:** The #onLayerFolderAdded method is called when a layer folder is added to a model.

**Parameters:**
- `layers` (Sketchup::Layers)
- `layer_folder` (Sketchup::LayerFolder)

**Returns:** `nil` — 

[Docs](https://ruby.sketchup.com/Sketchup/LayersObserver.html#onLayerFolderAdded-instance_method)

#### `onLayerFolderChanged(layers, layer_folder) => nil`

The #onLayerFolderChanged method is called when a layer folder changes one of its properties.

**Remarks:** The #onLayerFolderChanged method is called when a layer folder changes one of its properties.

**Parameters:**
- `layers` (Sketchup::Layers)
- `layer_folder` (Sketchup::LayerFolder)

**Returns:** `nil` — 

[Docs](https://ruby.sketchup.com/Sketchup/LayersObserver.html#onLayerFolderChanged-instance_method)

#### `onLayerFolderRemoved(layers, layer_folder) => nil`

The #onLayerFolderRemoved method is called when a layer folder is removed from a model.

**Remarks:** The #onLayerFolderRemoved method is called when a layer folder is removed from a model.

**Parameters:**
- `layers` (Sketchup::Layers)
- `layer_folder` (Sketchup::LayerFolder)

**Returns:** `nil` — 

[Docs](https://ruby.sketchup.com/Sketchup/LayersObserver.html#onLayerFolderRemoved-instance_method)

#### `onLayerRemoved(layers, layer) => nil`

The #onLayerRemoved method is called when a layer is removed from a model.

**Remarks:** The #onLayerRemoved method is called when a layer is removed from a model.

**Parameters:**
- `layers` (Sketchup::Layers)
- `layer` (Sketchup::Layer)

**Returns:** `nil` — 

[Docs](https://ruby.sketchup.com/Sketchup/LayersObserver.html#onLayerRemoved-instance_method)

#### `onParentFolderChanged(layers, layer) => nil`

Note: When a folder changes parent #onLayerFolderRemoved and #onLayerFolderAdded triggers.

**Remarks:** Note: When a folder changes parent #onLayerFolderRemoved and #onLayerFolderAdded triggers. The #onParentFolderChanged method is called when a layer changes parent folder.

**Parameters:**
- `layers` (Sketchup::Layers)
- `layer` (Sketchup::Layer)

**Returns:** `nil` — 

[Docs](https://ruby.sketchup.com/Sketchup/LayersObserver.html#onParentFolderChanged-instance_method)

#### `onRemoveAllLayers(layers) => nil`

The #onRemoveAllLayers method is called when all layer are removed from a model.

**Remarks:** The #onRemoveAllLayers method is called when all layer are removed from a model.

**Parameters:**
- `layers` (Sketchup::Layers)

**Returns:** `nil` — 

[Docs](https://ruby.sketchup.com/Sketchup/LayersObserver.html#onRemoveAllLayers-instance_method)

## Licensing (interface)

The Sketchup::Licensing module contains methods for extensions purchased from Extension Warehouse to check their licensing status. It is advised to place these calls inside encrypted Ruby (.rbe) files. This API is also exposed via the SketchUp C API. Extensions that already use native code should prefer using the native API, which should be more secure than their Ruby API counterparts. For more details, see Licensing Example

[Vendor docs](https://ruby.sketchup.com/Sketchup/Licensing.html)

### Methods
#### `get_extension_license(extension_id) => Sketchup::Licensing::ExtensionLicense`

Gets a license for a given extension.

**Remarks:** Gets a license for a given extension. Starting in SketchUp 2025.0, this method automatically tries to fetch a license from Extension Warehouse if the extension doesn't have a license on the current device. This only works if the user is signed in. In earlier SketchUp versions, the user has to go to Extension Manager, expand the extension in question and press Update License if the license is missing. (For performance reasons this automatic fetching is skipped during SU startup. Make sure to do a license check when the user interacts with the extension).

**Parameters:**
- `extension_id` (String) — The Extension Warehouse UUID for the desired extension.

**Returns:** `Sketchup::Licensing::ExtensionLicense` — An object representing licensing state for the extension. Do not store this object; retrieve it again when needed since licensing state may have changed.

[Docs](https://ruby.sketchup.com/Sketchup/Licensing.html#get_extension_license-class_method)

### Properties
- `LICENSED` (Object, get) — 
- `EXPIRED` (Object, get) — 
- `TRIAL` (Object, get) — 
- `TRIAL_EXPIRED` (Object, get) — 
- `NOT_LICENSED` (Object, get) — 

## LineStyle (class)

This provides a way for SketchUp to customize a line style and be set on a layer.

[Vendor docs](https://ruby.sketchup.com/Sketchup/LineStyle.html)

### Methods
#### `name => String`

The #name method retrieves the name of the line style object.

**Remarks:** The #name method retrieves the name of the line style object.

**Returns:** `String` — The name of the line style.

[Docs](https://ruby.sketchup.com/Sketchup/LineStyle.html#name-instance_method)

## LineStyles (class)

Provides access to the different line style objects in the model.

[Vendor docs](https://ruby.sketchup.com/Sketchup/LineStyles.html)

### Methods
#### `[](name) => Sketchup::LineStyle? | [](index) => Sketchup::LineStyle?`

The #[] method lets you retrieve a line style by name or index.

**Remarks:** The #[] method lets you retrieve a line style by name or index. The #at method is an alias of #[].

**Parameters:**
- `name` (String) — The name of the line style to retrieve.

**Returns:** `Sketchup::LineStyle, nil` — The line style retrieved. nil if the name couldn't be retrieved.

[Docs](https://ruby.sketchup.com/Sketchup/LineStyles.html#[]-instance_method)

#### `[](name) => Sketchup::LineStyle? | [](index) => Sketchup::LineStyle?`

The #[] method lets you retrieve a line style by name or index.

**Remarks:** The #[] method lets you retrieve a line style by name or index. The #at method is an alias of #[].

**Parameters:**
- `name` (String) — The name of the line style to retrieve.

**Returns:** `Sketchup::LineStyle, nil` — The line style retrieved. nil if the name couldn't be retrieved.

[Docs](https://ruby.sketchup.com/Sketchup/LineStyles.html#at-instance_method)

#### `each {|linestyle| ... } => nil`

The #each method is used to iterate through all the line styles.

**Remarks:** The #each method is used to iterate through all the line styles.

**Returns:** `nil` — 

[Docs](https://ruby.sketchup.com/Sketchup/LineStyles.html#each-instance_method)

#### `names => Array<String>`

The #names method return the support line styles as strings.

**Remarks:** The #names method return the support line styles as strings.

**Returns:** `Array<String>` — 

[Docs](https://ruby.sketchup.com/Sketchup/LineStyles.html#names-instance_method)

#### `size => Integer Also known as: length`

The #size method returns the number of line styles that SketchUp supports.

**Remarks:** The #size method returns the number of line styles that SketchUp supports.

**Returns:** `Integer` — The count of line styles.

[Docs](https://ruby.sketchup.com/Sketchup/LineStyles.html#size-instance_method)

#### `to_a => Array`

The #to_a method returns an array of all the line styles.

**Remarks:** The #to_a method returns an array of all the line styles.

**Returns:** `Array` — Containing all the line style objects.

[Docs](https://ruby.sketchup.com/Sketchup/LineStyles.html#to_a-instance_method)

## LoadHandler (class)

This class is abstract. Implement the methods described in this class to create a tool. You can not sub-class this class because it is not defined by the API. The main purpose of the LoadHandler interface is to be used as an optional second parameter of the DefinitionList#load_from_url method. Its methods that require implementation handle the process of downloading and managing the state of the load operation, including progress updates and error handling.

[Vendor docs](https://ruby.sketchup.com/Sketchup/LoadHandler.html)

### Methods
#### `cancelled? => Boolean`

This method is called when the download is canceled by the user.

**Remarks:** This method is called when the download is canceled by the user.

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Sketchup/LoadHandler.html#cancelled?-instance_method)

#### `onFailure(message) => Boolean`

This method is called when the download unsuccessfully completes

**Remarks:** This method is called when the download unsuccessfully completes

**Parameters:**
- `message` (String)

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Sketchup/LoadHandler.html#onFailure-instance_method)

#### `onPercentChange(percent) => nil`

This method is triggered whenever the percent value updates.

**Remarks:** This method is triggered whenever the percent value updates.

**Parameters:**
- `percent` (Float)

**Returns:** `nil` — 

[Docs](https://ruby.sketchup.com/Sketchup/LoadHandler.html#onPercentChange-instance_method)

#### `onSuccess => nil`

This method is called when the download successfully completes

**Remarks:** This method is called when the download successfully completes

**Returns:** `nil` — 

[Docs](https://ruby.sketchup.com/Sketchup/LoadHandler.html#onSuccess-instance_method)

## Loop (class)

Loop is a low level topology class that will not need to be used often. A Loop is a chain of Edges that bound a Face.

[Vendor docs](https://ruby.sketchup.com/Sketchup/Loop.html)

### Methods
#### `convex? => Boolean`

Determine if the loop is convex.

**Remarks:** Determine if the loop is convex.

**Returns:** `Boolean` — status - true if convex, false if not convex.

[Docs](https://ruby.sketchup.com/Sketchup/Loop.html#convex?-instance_method)

#### `edges => Object`

Get an array of the edges that define the loop in an ordered sequence.

**Remarks:** Get an array of the edges that define the loop in an ordered sequence.

**Returns:** `Object` — edges - an array of Edge objects if successful.

[Docs](https://ruby.sketchup.com/Sketchup/Loop.html#edges-instance_method)

#### `edgeuses => Object`

Get an array of the EdgeUse objects that define this loop in an ordered sequence.

**Remarks:** Get an array of the EdgeUse objects that define this loop in an ordered sequence.

**Returns:** `Object` — edgeuses - an array of EdgeUse objects if successful.

[Docs](https://ruby.sketchup.com/Sketchup/Loop.html#edgeuses-instance_method)

#### `face => Object`

Get the Face object that is bounded by this loop.

**Remarks:** Get the Face object that is bounded by this loop.

**Returns:** `Object` — face - a Face object if successful

[Docs](https://ruby.sketchup.com/Sketchup/Loop.html#face-instance_method)

#### `outer? => Boolean`

Determine if this is an outer loop.

**Remarks:** Determine if this is an outer loop. Each face has one outer loop, and will have one loop for each hole.

**Returns:** `Boolean` — status - true if the loop is an outer loop, false if it is not an outer loop.

[Docs](https://ruby.sketchup.com/Sketchup/Loop.html#outer?-instance_method)

#### `vertices => Object`

Get an array of the vertices that define the loop in an ordered sequence.

**Remarks:** Get an array of the vertices that define the loop in an ordered sequence.

**Returns:** `Object` — vertices - an array of Vertex objects if successful.

[Docs](https://ruby.sketchup.com/Sketchup/Loop.html#vertices-instance_method)

## Material (class)

The Material class represents a Texture or Color that can be applied to Drawingelements. It is most often applied to Faces. You can pass any object that can be used as a material to a method that requires a material. Objects include actual Material, Color, and classes that can be converted to a color.

[Vendor docs](https://ruby.sketchup.com/Sketchup/Material.html)

### Methods
#### `<=>(material) => Integer`

The #<=> method is used to compare two materials based on ##display_name.

**Remarks:** The #<=> method is used to compare two materials based on ##display_name.

**Parameters:**
- `material` (Sketchup::Material)

**Returns:** `Integer` — 0 if they are equal, 1 if material1 > material2, -1 if material1 < material2

[Docs](https://ruby.sketchup.com/Sketchup/Material.html#<=>-instance_method)

#### `ao_enabled? => Boolean PBR Metallic Roughness Workflow`

Note: There is no setter for this property.

**Remarks:** Note: There is no setter for this property. Instead it's dictated whether a #ao_texture is set.

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Sketchup/Material.html#ao_enabled?-instance_method)

#### `colorize_deltas => Array(Float, Float, Float)`

The #colorize_deltas method retrieves the HLS delta for colorized materials.

**Remarks:** The #colorize_deltas method retrieves the HLS delta for colorized materials.

**Returns:** `Array(Float, Float, Float)` — An array of floats representing the HLS delta.

[Docs](https://ruby.sketchup.com/Sketchup/Material.html#colorize_deltas-instance_method)

#### `display_name => String`

The #display_name method retrieves the name that is displayed within SketchUp for the material.

**Remarks:** The #display_name method retrieves the name that is displayed within SketchUp for the material. This should be used when presenting the name in the UI, but the returned name cannot be used as a key in Sketchup::Model#materials.

**Returns:** `String` — 

[Docs](https://ruby.sketchup.com/Sketchup/Material.html#display_name-instance_method)

#### `materialType => Integer`

The #materialType method retrieves the type of the material.

**Remarks:** The #materialType method retrieves the type of the material. Material Types: 0 = solid (MATERIAL_SOLID) 1 = textured (MATERIAL_TEXTURED) 2 = colorized textured (MATERIAL_COLORIZED_TEXTURED) The constants were added in SketchUp 2015.

**Returns:** `Integer` — One of Sketchup::Material::MATERIAL_* values.

[Docs](https://ruby.sketchup.com/Sketchup/Material.html#materialType-instance_method)

#### `metalness_enabled? => Boolean PBR Metallic Roughness Workflow`



**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Sketchup/Material.html#metalness_enabled?-instance_method)

#### `normal_enabled? => Boolean PBR Metallic Roughness Workflow`



**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Sketchup/Material.html#normal_enabled?-instance_method)

#### `owner_type => Integer`

The #owner_type method is used to determine what owns the material.

**Remarks:** The #owner_type method is used to determine what owns the material. Material Owner Types: OWNER_MANAGER OWNER_IMAGE OWNER_LAYER

**Returns:** `Integer` — 

[Docs](https://ruby.sketchup.com/Sketchup/Material.html#owner_type-instance_method)

#### `roughness_enabled? => Boolean PBR Metallic Roughness Workflow`



**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Sketchup/Material.html#roughness_enabled?-instance_method)

#### `save_as(filename) => Boolean`

Note: You must remember to append “.

**Remarks:** Note: You must remember to append “.skm” to the filename as this will not be done automatically. The #save_as method is used to write a material to a SKM file.

**Parameters:**
- `filename` (String) — the path to the SKM file to load.

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Sketchup/Material.html#save_as-instance_method)

#### `use_alpha? => Boolean`

Note: that this is not affected by the alpha value of the #color object.

**Remarks:** Note: that this is not affected by the alpha value of the #color object. Only the #alpha value will make this method return true. The #use_alpha? method tells if the material uses transparency. It uses some tolerance checking to account for floating point precision noise.

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Sketchup/Material.html#use_alpha?-instance_method)

#### `workflow => Integer PBR Metallic Roughness Workflow`

Material Workflows: WORKFLOW_CLASSIC WORKFLOW_PBR_METALLIC_ROUGHNESS When the workflow returns WORKFLOW_PBR_METALLIC_ROUGHNESS the properties listed under PBR Metallic Roughness Workflow are relevant.

**Remarks:** Material Workflows: WORKFLOW_CLASSIC WORKFLOW_PBR_METALLIC_ROUGHNESS When the workflow returns WORKFLOW_PBR_METALLIC_ROUGHNESS the properties listed under PBR Metallic Roughness Workflow are relevant.

**Returns:** `Integer` — One of Sketchup::Material::WORKFLOW_* values.

[Docs](https://ruby.sketchup.com/Sketchup/Material.html#workflow-instance_method)

#### `write_thumbnail(path, max_size) => Boolean?`

The #write_thumbnail method writes a bitmap thumbnail to the given file name.

**Remarks:** The #write_thumbnail method writes a bitmap thumbnail to the given file name.

**Parameters:**
- `path` (String) — The file path for the thumbnail.
- `max_size` (Integer) — The maximum width or height of the generated image.

**Returns:** `Boolean, nil` — true if successful, false if unsuccessful. nil if arguments are invalid.

[Docs](https://ruby.sketchup.com/Sketchup/Material.html#write_thumbnail-instance_method)

### Properties
- `=` (Sketchup::Material, set) — The #== method is used to test if two materials are the same.
- `alpha` (Float, get/set) — The #alpha method is used to get the opacity of the material.
- `ao_enabled` (Object, set) — 
- `ao_strength` (Float, get/set) — Returns A value between 0.
- `ao_texture` (Sketchup::Texture, get/set) — 
- `color` (Sketchup::Color, get/set) — Note: The alpha value of the Color object is not used for the material's transparency.
- `colorize_type` (Integer, get/set) — Note: This value is only relevant when the #materialType is set to MATERIAL_COLORIZED_TEXTURED.
- `metallic_factor` (Float, get/set) — Returns A value between 0.
- `metallic_texture` (Sketchup::Texture, get/set) — 
- `metalness_enabled` (Boolean, set) — 
- `name` (String, get/set) — The #name method retrieves the name of the material.
- `normal_enabled` (Boolean, set) — 
- `normal_scale` (Float, get/set) — Returns A value larger than or equal to 0.
- `normal_style` (Integer, get/set) — Material Normal Styles: NORMAL_STYLE_OPENGL NORMAL_STYLE_DIRECTX
- `normal_texture` (Sketchup::Texture, get/set) — 
- `roughness_enabled` (Boolean, set) — 
- `roughness_factor` (Float, get/set) — Returns A value between 0.
- `roughness_texture` (Sketchup::Texture, get/set) — 
- `texture` (Sketchup::Texture, nil, get/set) — The #texture method retrieves the texture of the material.
- `WORKFLOW_CLASSIC` (Object, get) — 
- `WORKFLOW_PBR_METALLIC_ROUGHNESS` (Object, get) — 
- `NORMAL_STYLE_OPENGL` (Object, get) — 
- `NORMAL_STYLE_DIRECTX` (Object, get) — 
- `MATERIAL_SOLID` (Object, get) — 
- `MATERIAL_TEXTURED` (Object, get) — 
- `MATERIAL_COLORIZED_TEXTURED` (Object, get) — 
- `COLORIZE_SHIFT` (Object, get) — 
- `COLORIZE_TINT` (Object, get) — 
- `OWNER_MANAGER` (Object, get) — 
- `OWNER_IMAGE` (Object, get) — 
- `OWNER_LAYER` (Object, get) — 

## Materials (class)

A collection of Materials objects. Each model contains a Materials collection that can be accessed via Model.materials.

[Vendor docs](https://ruby.sketchup.com/Sketchup/Materials.html)

### Methods
#### `[](index) => Sketchup::Material? | [](name) => Sketchup::Material?`

The #[] method is used to retrieve a material by index or name.

**Remarks:** The #[] method is used to retrieve a material by index or name. The #at method is an alias of #[]

**Parameters:**
- `index` (Integer) — A number representing the material's index in an array of Material objects.

**Returns:** `Sketchup::Material, nil` — a Material object on success, Nil on failure

[Docs](https://ruby.sketchup.com/Sketchup/Materials.html#[]-instance_method)

#### `[](index) => Sketchup::Material? | [](name) => Sketchup::Material?`

The #[] method is used to retrieve a material by index or name.

**Remarks:** The #[] method is used to retrieve a material by index or name. The #at method is an alias of #[]

**Parameters:**
- `index` (Integer) — A number representing the material's index in an array of Material objects.

**Returns:** `Sketchup::Material, nil` — a Material object on success, Nil on failure

[Docs](https://ruby.sketchup.com/Sketchup/Materials.html#at-instance_method)

#### `add(name) => Sketchup::Material`

Add a new Material.

**Remarks:** Add a new Material. When called with no arguments, this will generate a new unique name for the new Material. If a name is given, it will check to see if there is already a material with that name. If there is already a material with the given name, then a new unique name is generated using the given name as a base.

**Parameters:**
- `name` (String) — The name of the new material.

**Returns:** `Sketchup::Material` — a Material object

[Docs](https://ruby.sketchup.com/Sketchup/Materials.html#add-instance_method)

#### `add_observer(observer) => Boolean`

The add_observer method is used to add an observer to the materials collection.

**Remarks:** The add_observer method is used to add an observer to the materials collection.

**Parameters:**
- `observer` (Object) — An observer.

**Returns:** `Boolean` — true if successful, false if unsuccessful.

[Docs](https://ruby.sketchup.com/Sketchup/Materials.html#add_observer-instance_method)

#### `count => Integer`

Note: Since SketchUp 2014 the count method is inherited from Ruby's Enumerable mix-in module.

**Remarks:** Note: Since SketchUp 2014 the count method is inherited from Ruby's Enumerable mix-in module. Prior to that the #count method is an alias for #length.

**Returns:** `Integer` — 

[Docs](https://ruby.sketchup.com/Sketchup/Materials.html#count-instance_method)

#### `each {|material| ... } => Sketchup::Materials`

Note: Don't remove content from this collection while iterating over it with #each.

**Remarks:** Note: Don't remove content from this collection while iterating over it with #each. This would change the size of the collection and cause elements to be skipped as the indices change. Instead copy the current collection to an array using to_a and then use each on the array, when removing content. The #each method is used to iterate through all of the materials.

**Returns:** `Sketchup::Materials` — 

[Docs](https://ruby.sketchup.com/Sketchup/Materials.html#each-instance_method)

#### `length => Integer`

Note: The returned number includes Image materials as well.

**Remarks:** Note: The returned number includes Image materials as well. It will not reflect the number of materials yielded by #each. To get the number of non-image materials use #count or materials.to_a.size. The number of materials in the collection.

**Returns:** `Integer` — 

[Docs](https://ruby.sketchup.com/Sketchup/Materials.html#length-instance_method)

#### `load(filename) => Sketchup::Material`

The #load method is used to load a material from file into the model.

**Remarks:** The #load method is used to load a material from file into the model. If a matching material exist in the model it will be returned instead.

**Parameters:**
- `filename` (String) — the path to the SKM file to load.

**Returns:** `Sketchup::Material` — the new loaded material, or existing material.

[Docs](https://ruby.sketchup.com/Sketchup/Materials.html#load-instance_method)

#### `purge_unused => Sketchup::Materials`

The purge_unused method is used to remove unused materials.

**Remarks:** The purge_unused method is used to remove unused materials.

**Returns:** `Sketchup::Materials` — The Materials object.

[Docs](https://ruby.sketchup.com/Sketchup/Materials.html#purge_unused-instance_method)

#### `remove(material) => Boolean`

Remove a given material.

**Remarks:** Remove a given material.

**Parameters:**
- `material` (Sketchup::Material) — The material to remove.

**Returns:** `Boolean` — true if successful, false if unsuccessful.

[Docs](https://ruby.sketchup.com/Sketchup/Materials.html#remove-instance_method)

#### `remove_observer(observer) => Boolean`

The remove_observer method is used to remove an observer from the materials collection.

**Remarks:** The remove_observer method is used to remove an observer from the materials collection.

**Parameters:**
- `observer` (Object) — An observer.

**Returns:** `Boolean` — true if successful, false if unsuccessful.

[Docs](https://ruby.sketchup.com/Sketchup/Materials.html#remove_observer-instance_method)

#### `size => Integer`

Note: The returned number includes Image materials as well.

**Remarks:** Note: The returned number includes Image materials as well. It will not reflect the number of materials yielded by #each. To get the number of non-image materials use #count or materials.to_a.size. The number of materials in the collection. The #size method is an alias for #length.

**Returns:** `Integer` — 

[Docs](https://ruby.sketchup.com/Sketchup/Materials.html#size-instance_method)

#### `unique_name(name) => String`

The #unique_name method is used to retrieve a unique name from the materials collection that is based on the provided one.

**Remarks:** The #unique_name method is used to retrieve a unique name from the materials collection that is based on the provided one. If provided name is unique it will be returned, otherwise any trailing indices will be replaced by a new index.

**Parameters:**
- `name` (String) — the suggested name.

**Returns:** `String` — a unique name.

[Docs](https://ruby.sketchup.com/Sketchup/Materials.html#unique_name-instance_method)

### Properties
- `current` (Sketchup::Material, get/set) — The current method is used to get the current material, i.

## MaterialsObserver (class)

This class is abstract. To implement this observer, create a Ruby class of this type, override the desired methods, and add an instance of the observer to the objects of interests. Note: The callback onMaterialRemoveAll has been deprecated, we recommend using #onMaterialRemove instead. This observer interface is implemented to react to materials events.

[Vendor docs](https://ruby.sketchup.com/Sketchup/MaterialsObserver.html)

### Methods
#### `onMaterialAdd(materials, material) => nil`

The #onMaterialAdd method is invoked whenever a new material is added.

**Remarks:** The #onMaterialAdd method is invoked whenever a new material is added.

**Parameters:**
- `materials` (Sketchup::Materials)
- `material` (Sketchup::Material)

**Returns:** `nil` — 

[Docs](https://ruby.sketchup.com/Sketchup/MaterialsObserver.html#onMaterialAdd-instance_method)

#### `onMaterialChange(materials, material) => nil`

The #onMaterialChange method is invoked whenever a material's texture image is altered.

**Remarks:** The #onMaterialChange method is invoked whenever a material's texture image is altered.

**Parameters:**
- `materials` (Sketchup::Materials)
- `material` (Sketchup::Material)

**Returns:** `nil` — 

[Docs](https://ruby.sketchup.com/Sketchup/MaterialsObserver.html#onMaterialChange-instance_method)

#### `onMaterialRefChange(materials, material) => nil`

The #onMaterialRefChange method is invoked whenever the number of entities that a material is painted on changes.

**Remarks:** The #onMaterialRefChange method is invoked whenever the number of entities that a material is painted on changes. This could be due to the user manually painting something, but it could also be when faces are split, pasted, push-pulled, deleted, etc.

**Parameters:**
- `materials` (Sketchup::Materials)
- `material` (Sketchup::Material)

**Returns:** `nil` — 

[Docs](https://ruby.sketchup.com/Sketchup/MaterialsObserver.html#onMaterialRefChange-instance_method)

#### `onMaterialRemove(materials, material) => nil`

The #onMaterialRemove method is invoked whenever a material is deleted.

**Remarks:** The #onMaterialRemove method is invoked whenever a material is deleted.

**Parameters:**
- `materials` (Sketchup::Materials)
- `material` (Sketchup::Material)

**Returns:** `nil` — 

[Docs](https://ruby.sketchup.com/Sketchup/MaterialsObserver.html#onMaterialRemove-instance_method)

#### `onMaterialSetCurrent(materials, material) => nil`

The #onMaterialSetCurrent method is invoked whenever a different material is selected in the Materials dialog.

**Remarks:** The #onMaterialSetCurrent method is invoked whenever a different material is selected in the Materials dialog. The materials parameter might be Nil when the material is picked from the materials libraries and not yet added to the model.

**Parameters:**
- `materials` (Sketchup::Materials)
- `material` (Sketchup::Material)

**Returns:** `nil` — 

[Docs](https://ruby.sketchup.com/Sketchup/MaterialsObserver.html#onMaterialSetCurrent-instance_method)

#### `onMaterialUndoRedo(materials, material) => nil`

Note: Due to a bug, this callback does not fire in SU6 or SU7.

**Remarks:** Note: Due to a bug, this callback does not fire in SU6 or SU7. You can use the Sketchup::ModelObserver#onTransactionStart to capture all undo events. The #onMaterialUndoRedo method is invoked whenever a material is altered and then those changes are undone or redone.

**Parameters:**
- `materials` (Sketchup::Materials)
- `material` (Sketchup::Material)

**Returns:** `nil` — 

[Docs](https://ruby.sketchup.com/Sketchup/MaterialsObserver.html#onMaterialUndoRedo-instance_method)

## Menu (class)

An interface to a menu.

[Vendor docs](https://ruby.sketchup.com/Sketchup/Menu.html)

### Methods
#### `add_item(title) { ... } => Integer | add_item(command) => Integer`

The #add_item method is used to add a menu item to the specified menu.

**Remarks:** The #add_item method is used to add a menu item to the specified menu. This method takes a block that defines the action to perform when the menu item is selected. The item id that is returned can be used when adding an optional validation procedure for the menu item.

**Parameters:**
- `title` (String)

**Returns:** `Integer` — A unique integer id for the added menu item.

[Docs](https://ruby.sketchup.com/Sketchup/Menu.html#add_item-instance_method)

#### `add_separator => nil`

The #add_separator method is used to add a menu separator to a menu.

**Remarks:** The #add_separator method is used to add a menu separator to a menu.

**Returns:** `nil` — 

[Docs](https://ruby.sketchup.com/Sketchup/Menu.html#add_separator-instance_method)

#### `add_submenu(title) => Sketchup::Menu`

The #add_submenu method is used to add a sub-menu to a menu.

**Remarks:** The #add_submenu method is used to add a sub-menu to a menu.

**Parameters:**
- `title` (String) — The title of the sub menu.

**Returns:** `Sketchup::Menu` — a Menu object

[Docs](https://ruby.sketchup.com/Sketchup/Menu.html#add_submenu-instance_method)

#### `set_validation_proc(item) { ... } => Object`

The #set_validation_proc method is used to specify the menu validation procedure.

**Remarks:** The #set_validation_proc method is used to specify the menu validation procedure. Your procedure should return either MF_ENABLED, MF_DISABLED, MF_CHECKED, MF_UNCHECKED, or MF_GRAYED.

**Parameters:**
- `item` (Integer) — The numerical identifier for the menu item.

[Docs](https://ruby.sketchup.com/Sketchup/Menu.html#set_validation_proc-instance_method)

## Model (class)

This is the interface to a SketchUp model. The model is the 3D drawing that the user is working with, and it serves as the “entry point” for most Ruby API interactions. The Sketchup.active_model method gives you a handle to the current model, and from there you can use the model-level methods to start getting information and making changes.

**Remarks:** Known Bugs: Prior to SketchUp 2019.0 this class would yield TypeError for all method calls if #singleton_class was called on the model object.

[Vendor docs](https://ruby.sketchup.com/Sketchup/Model.html)

### Methods
#### `abort_operation => Boolean`

Note: Never abort a transparent operation.

**Remarks:** Note: Never abort a transparent operation. Doing so would abort the operation it chains to. Instead, try to clean up and simply commit in order to make sure the operation is closed. The #abort_operation method aborts the current operation started with the start_operation method. The #abort_operation method is normally called from inside of a rescue clause to cancel an operation if something goes wrong.

**Returns:** `Boolean` — true if successful, false if unsuccessful

[Docs](https://ruby.sketchup.com/Sketchup/Model.html#abort_operation-instance_method)

#### `active_entities => Sketchup::Entities`

Returns an Entities object which contains the entities in the open group or component instance.

**Remarks:** Returns an Entities object which contains the entities in the open group or component instance. If no group or component is open for editing then this will be the same as #entities. To perform actions upon the current set of entities the user is working with then this is the method to use. Entities selected by the user will be a subset of the active entities.

**Returns:** `Sketchup::Entities` — 

[Docs](https://ruby.sketchup.com/Sketchup/Model.html#active_entities-instance_method)

#### `active_section_planes => Array<Sketchup::SectionPlane>`

The #active_section_planes method returns all of the active section planes in the model.

**Remarks:** The #active_section_planes method returns all of the active section planes in the model.

**Returns:** `Array<Sketchup::SectionPlane>` — 

[Docs](https://ruby.sketchup.com/Sketchup/Model.html#active_section_planes-instance_method)

#### `active_view => Sketchup::View`

The #active_view method returns the active View object for this model.

**Remarks:** The #active_view method returns the active View object for this model.

**Returns:** `Sketchup::View` — 

[Docs](https://ruby.sketchup.com/Sketchup/Model.html#active_view-instance_method)

#### `add_note(note, x, y) => Sketchup::Text`

Add a text note to the Model.

**Remarks:** Add a text note to the Model. The position of the note is given as relative window positions between 0 and 1. For example, the following command would create a note that start 1/10 of the ways down the screen from the upper left corner of the window.

**Parameters:**
- `note` (String) — A string note.
- `x` (Numeric) — A distance along the x axis between 0 and 1.
- `y` (Numeric) — A distance along the y axis between 0 and 1.

**Returns:** `Sketchup::Text` — a note object or an exception if it is unsuccessful.

[Docs](https://ruby.sketchup.com/Sketchup/Model.html#add_note-instance_method)

#### `add_observer(observer) => Boolean`

The add_observer method is used to add an observer to the current object.

**Remarks:** The add_observer method is used to add an observer to the current object.

**Parameters:**
- `observer` (Object) — An observer.

**Returns:** `Boolean` — true if successful, false if unsuccessful.

[Docs](https://ruby.sketchup.com/Sketchup/Model.html#add_observer-instance_method)

#### `attribute_dictionaries => Sketchup::AttributeDictionaries`

The #attribute_dictionaries method retrieves the AttributeDictionaries object that is associated with the Model.

**Remarks:** The #attribute_dictionaries method retrieves the AttributeDictionaries object that is associated with the Model.

**Returns:** `Sketchup::AttributeDictionaries` — the AttributeDictionaries object associated with the entity, or nil if there are no attribute_dictionary objects associated with the model. Care must be taken if nil is returned, for example: invoking attribute_dictionaries.length will throw a NoMethodError exception, not return 0.

[Docs](https://ruby.sketchup.com/Sketchup/Model.html#attribute_dictionaries-instance_method)

#### `attribute_dictionary(name, create = false) => Sketchup::AttributeDictionary`

Returns the Sketchup::AttributeDictionary object that is specified by name.

**Remarks:** Returns the Sketchup::AttributeDictionary object that is specified by name. If the model does not have an attribute dictionary that corresponds to name, returns either nil, or a creates an attribute dictionary. If the optional second argument is true, and there is no attribute dictionary that corresponds to name, a new attribute dictionary is created.

**Parameters:**
- `name` (String) — The name of the dictionary you are attempting to retrieve.
- `create` (Boolean) — if set to true an attribute dictionary of the given “name” will be created if not found.

**Returns:** `Sketchup::AttributeDictionary` — an attribute dictionary object if successful, nil if unsuccessful

[Docs](https://ruby.sketchup.com/Sketchup/Model.html#attribute_dictionary-instance_method)

#### `axes => Sketchup::Axes`

The #axes method returns the drawing axes for the model.

**Remarks:** The #axes method returns the drawing axes for the model.

**Returns:** `Sketchup::Axes` — the axes for the model.

[Docs](https://ruby.sketchup.com/Sketchup/Model.html#axes-instance_method)

#### `behavior => Sketchup::Behavior`

The behavior method retrieves the behavior of the model.

**Remarks:** The behavior method retrieves the behavior of the model.

**Returns:** `Sketchup::Behavior` — behavior object for the model if successful

[Docs](https://ruby.sketchup.com/Sketchup/Model.html#behavior-instance_method)

#### `bounds => Geom::BoundingBox`

The #bounds method is used to retrieve the Geom::BoundingBox bounding the contents of a Sketchup::Model.

**Remarks:** The #bounds method is used to retrieve the Geom::BoundingBox bounding the contents of a Sketchup::Model.

**Returns:** `Geom::BoundingBox` — 

[Docs](https://ruby.sketchup.com/Sketchup/Model.html#bounds-instance_method)

#### `classifications => Sketchup::Classifications`

The #classifications method is used to retrieve the Classifications object for this model.

**Remarks:** The #classifications method is used to retrieve the Classifications object for this model.

**Returns:** `Sketchup::Classifications` — a Classifications object.

[Docs](https://ruby.sketchup.com/Sketchup/Model.html#classifications-instance_method)

#### `close(ignore_changes = false) => nil`

Note: As of SketchUp 2024.

**Remarks:** Note: As of SketchUp 2024.0 this method will ensure the next model window gets focus if there is one. Before that `Sketchup.active_model` might return `nil` after calling this method even though more models where open. The #close method is used to close this model. On Mac OS, only the active model can be closed. On Windows, since there can be only one document open, this method will perform a File/New operation.

**Parameters:**
- `ignore_changes` (Boolean) — If `true`, model changes will be ignored and save prompts will be suppressed. If `false`, changes will not be ignored and save prompts will be displayed normally.

**Returns:** `nil` — 

[Docs](https://ruby.sketchup.com/Sketchup/Model.html#close-instance_method)

#### `close_active => Boolean`

Note: Before SketchUp 2014 this method had a bug where it didn't create an undo operation and that could lead to corrupted geometry when undo/redo was used after invoking this method.

**Remarks:** Note: Before SketchUp 2014 this method had a bug where it didn't create an undo operation and that could lead to corrupted geometry when undo/redo was used after invoking this method. The #close_active method is used to close the currently active (open) group or component.

**Returns:** `Boolean` — true if successful, false if unsuccessful.

[Docs](https://ruby.sketchup.com/Sketchup/Model.html#close_active-instance_method)

#### `commit_operation => Boolean`

The commit_operation method commits an operation for undo.

**Remarks:** The commit_operation method commits an operation for undo. The commit_operation method is normally called at the end of a method to commit the operation that the method performs.

**Returns:** `Boolean` — true if successful, false if unsuccessful

[Docs](https://ruby.sketchup.com/Sketchup/Model.html#commit_operation-instance_method)

#### `definitions => Sketchup::DefinitionList`

The #definitions method retrieves a definition list containing all of the component definitions in the model.

**Remarks:** The #definitions method retrieves a definition list containing all of the component definitions in the model.

**Returns:** `Sketchup::DefinitionList` — 

[Docs](https://ruby.sketchup.com/Sketchup/Model.html#definitions-instance_method)

#### `drawing_element_visible?(instance_path) => Boolean`

The #drawing_element_visible? method reports whether the given drawing element in an instance path is visible given the current model options.

**Remarks:** The #drawing_element_visible? method reports whether the given drawing element in an instance path is visible given the current model options.

**Parameters:**
- `instance_path` (Sketchup::InstancePath, Array<Sketchup::Drawingelement>)

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Sketchup/Model.html#drawing_element_visible?-instance_method)

#### `edit_transform => Geom::Transformation`

Returns the transformation of the current component edit session.

**Remarks:** Returns the transformation of the current component edit session. If a user has double-clicked to edit a component's geometry, this will return the transformation of that component, relative to its parent's origin. This allows one to correctly calculate “local” transformations of a given entity regardless of whether the user is in edit mode.

**Returns:** `Geom::Transformation` — the current edit Transformation

[Docs](https://ruby.sketchup.com/Sketchup/Model.html#edit_transform-instance_method)

#### `entities => Sketchup::Entities`

Note: This does not return a collection of all the entities in the model, only the top level node of the model hierarchy.

**Remarks:** Note: This does not return a collection of all the entities in the model, only the top level node of the model hierarchy. To get to all entities in a model you must recursivly traverse the model. The #entities method returns an Entities object containing the entities in the root of model.

**Returns:** `Sketchup::Entities` — an Entities object if successful

[Docs](https://ruby.sketchup.com/Sketchup/Model.html#entities-instance_method)

#### `environments => Sketchup::Environments`

The #environments method is used to retrieve the Environments object for this model.

**Remarks:** The #environments method is used to retrieve the Environments object for this model.

**Returns:** `Sketchup::Environments` — 

[Docs](https://ruby.sketchup.com/Sketchup/Model.html#environments-instance_method)

#### `export(path, show_summary = false) => Boolean | export(path, options) => Boolean`

The export method is used to export a given file format.

**Remarks:** The export method is used to export a given file format. It knows which format to export based on the file extension you place on the file name. For example, a filename of “thing.obj” will export an OBJ file, whereas “thing.dae” will export a COLLADA file. For SketchUp Pro 7.1+, valid extensions include dae, kmz, 3ds, dwg, dxf, fbx, obj, wrl, and xsi. SketchUp Free only supports dae and kmz. Format Support Changes: SketchUp 7.1 added COLLADA (.dae) export capability. SketchUp Pro 2015+ added IFC export capability. SketchUp Pro 2016+ added PDF export capability. SketchUp Pro 2018+ added options for all 3D exporters. SketchUp 2024+ added glb and usdz exporters. SketchUp 2026+ the IFC exporter has been replaced with a new implementation that provides specific export options. See the Exporter Options file for information on creating a valid hash for the various exporters.

**Parameters:**
- `path` (String) — The name of the file to export.
- `show_summary` (Boolean) — Boolean to show summary dialog.

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Sketchup/Model.html#export-instance_method)

#### `find_entity_by_id(ids_or_array) => Array<Sketchup::Entity, nil>`

Finds and returns entities by their entityID or GUID.

**Remarks:** Finds and returns entities by their entityID or GUID. GUIDs looked up are only relevant to Group and ComponentInstance as these GUIDs are persistent. ComponentDefinition and Model GUIDs are not persistent and are not looked up. When given an array of IDs, an array is returned with a 1:1 mapping to the input arguments. This array may contain nil values if some ids were not found. You cannot look up a mix of entityIDs and GUIDs in the same call.

**Parameters:**
- `ids_or_array` (Array<Integer, String>) — Pass either a series of ids or a single array containing ids. Ids must either be entityID Integers or GUID Strings.

**Returns:** `Array<Sketchup::Entity, nil>` — Returns an array with Entity objects for each id found and nil otherwise. Single Entity or nil when called with a single id.

[Docs](https://ruby.sketchup.com/Sketchup/Model.html#find_entity_by_id-instance_method)

#### `find_entity_by_persistent_id(ids_or_array) => Sketchup::Entity, ... | find_entity_by_persistent_id(ids_or_array, **scope) => Sketchup::Entity, ...`

Finds and returns entities by their persistent id.

**Remarks:** Finds and returns entities by their persistent id. When given an array of IDs, an array is returned with a 1:1 mapping to the input arguments. This array may contain `nil` values if some ids were not found.

**Parameters:**
- `ids_or_array` (Array<Integer>) — Pass either a series of ids or a single array containing persistent ids.

**Returns:** `Sketchup::Entity, Array<Sketchup::Entity>, nil` — Returns an Entity if a single PID was given or an array of Entity objects for each id found or nil if nothing was found.

[Docs](https://ruby.sketchup.com/Sketchup/Model.html#find_entity_by_persistent_id-instance_method)

#### `georeferenced? => Boolean`

This methods determines if the model is georeferenced.

**Remarks:** This methods determines if the model is georeferenced.

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Sketchup/Model.html#georeferenced?-instance_method)

#### `get_attribute(dictname, key, defaultvalue = nil) => Object?`

The get_attribute method gets the value of an attribute that in the AttributeDictionary with the given name.

**Remarks:** The get_attribute method gets the value of an attribute that in the AttributeDictionary with the given name. If no value is associated with key, or if the model does not have an attribute dictionary specified by name, the optional third parameter will be returned.

**Parameters:**
- `dictname` (String) — The name of the dictionary containing the value.
- `key` (String) — The key containing the value.
- `defaultvalue` (Object) — default value that will be returned if a value does not exist.

**Returns:** `Object, nil` — the value for a given key in the given dictionary if a value exists; the default value if a defaultvalue is provided and the value does not exist; nil if the value does not exist and no defaultvalue is provided.

[Docs](https://ruby.sketchup.com/Sketchup/Model.html#get_attribute-instance_method)

#### `get_datum => String`

the get_datum method retrieves the datum, in the form of a string, used in UTM conversions.

**Remarks:** the get_datum method retrieves the datum, in the form of a string, used in UTM conversions.

**Returns:** `String` — a datum represented as a string if successful.

[Docs](https://ruby.sketchup.com/Sketchup/Model.html#get_datum-instance_method)

#### `get_product_family => Integer`

Returns a value which indicates the product family of the installed SketchUp application.

**Remarks:** Returns a value which indicates the product family of the installed SketchUp application. The constants for possible return values since SketchUp 2016: ProTrial ProLicensed MakeTrial Make In earlier SketchUp versions there were no defined constants and additional values could be returned. As of SketchUp 2013, the return values were: 0 = Unknown 1 = Pro Trial 2 = Pro 3 = Pro Expired 4 = Make Trial 5 = Make Expired 6 = Make 7 = Pro License Unavailable

**Returns:** `Integer` — the product family number.

[Docs](https://ruby.sketchup.com/Sketchup/Model.html#get_product_family-instance_method)

#### `guid => String`

The guid method retrieves the globally unique identifier, in the form of a string, for the Model.

**Remarks:** The guid method retrieves the globally unique identifier, in the form of a string, for the Model. The guid will change after the model is modified and saved. The Model guid is stored with the SketchUp file; it will not change if the file is moved to another computer.

**Returns:** `String` — a globally unique identifier, in the form of a string, for the model

[Docs](https://ruby.sketchup.com/Sketchup/Model.html#guid-instance_method)

#### `import(path, options) => Boolean | import(path, show_summary = false) => Boolean`

The import method is used to load a file by recognizing the file extension and calling appropriate importer.

**Remarks:** The import method is used to load a file by recognizing the file extension and calling appropriate importer. See DefinitionList#import for importing a 3d model file as a component definition, without activating the UI for placing an instance. See the Importer Options file for information on creating a valid hash for the various importers.

**Parameters:**
- `path` (String) — The input file path.
- `options` (Hash) — The options.

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Sketchup/Model.html#import-instance_method)

#### `instance_path_from_pid_path(pid_path) => Sketchup::InstancePath`

The #instance_path_from_pid_path method returns a instance path given a string with persistent ids representing the path to the entity.

**Remarks:** The #instance_path_from_pid_path method returns a instance path given a string with persistent ids representing the path to the entity.

**Parameters:**
- `pid_path` (String) — a string with persistent ids delimited by period.

**Returns:** `Sketchup::InstancePath` — 

[Docs](https://ruby.sketchup.com/Sketchup/Model.html#instance_path_from_pid_path-instance_method)

#### `latlong_to_point(lnglat_array) => Geom::Point3d`

The latlong_to_point method converts a latitude and longitude to a Point3d object in the model.

**Remarks:** The latlong_to_point method converts a latitude and longitude to a Point3d object in the model. It does not actually work with a LatLong object, but operates on a 2-element array. The returned point will always be on the ground (z=0).

**Parameters:**
- `lnglat_array` (Array(Numeric, Numeric)) — A 2-element array containing first the longitude then the latitude.

**Returns:** `Geom::Point3d` — A Point3d in model coordinates.

[Docs](https://ruby.sketchup.com/Sketchup/Model.html#latlong_to_point-instance_method)

#### `layers => Sketchup::Layers`

The #layers method retrieves a collection of all Layers objects in the model.

**Remarks:** The #layers method retrieves a collection of all Layers objects in the model.

**Returns:** `Sketchup::Layers` — a Layers object containing a collection of layers in the model

[Docs](https://ruby.sketchup.com/Sketchup/Model.html#layers-instance_method)

#### `line_styles => Sketchup::LineStyles`

The #line_styles method returns the line styles manager.

**Remarks:** The #line_styles method returns the line styles manager.

**Returns:** `Sketchup::LineStyles` — The line styles manager.

[Docs](https://ruby.sketchup.com/Sketchup/Model.html#line_styles-instance_method)

#### `list_datums => Array<String>`

This method retrieves an Array of all of the datums recognized by SketchUp.

**Remarks:** This method retrieves an Array of all of the datums recognized by SketchUp.

**Returns:** `Array<String>` — An Array object containing the datums supported by SketchUp

[Docs](https://ruby.sketchup.com/Sketchup/Model.html#list_datums-instance_method)

#### `materials => Sketchup::Materials`

The #materials method returns a collection of all of the materials in the model.

**Remarks:** The #materials method returns a collection of all of the materials in the model.

**Returns:** `Sketchup::Materials` — 

[Docs](https://ruby.sketchup.com/Sketchup/Model.html#materials-instance_method)

#### `mipmapping? => Boolean`

This method can be used to find out if mipmapping is on or off.

**Remarks:** This method can be used to find out if mipmapping is on or off.

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Sketchup/Model.html#mipmapping?-instance_method)

#### `modified? => Boolean`

The modified? method determines if the Model has been modified since the last save.

**Remarks:** The modified? method determines if the Model has been modified since the last save.

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Sketchup/Model.html#modified?-instance_method)

#### `number_faces => Integer`

Returns the number faces in a model.

**Remarks:** Returns the number faces in a model.

**Returns:** `Integer` — 

[Docs](https://ruby.sketchup.com/Sketchup/Model.html#number_faces-instance_method)

#### `options => Sketchup::OptionsManager`

The #options method retrieves the options manager that defines the options settings for the model.

**Remarks:** The #options method retrieves the options manager that defines the options settings for the model. Use the string keys instead of numerical indicies when accessing the options as the indicies are not consistent between SketchUp versions.

**Returns:** `Sketchup::OptionsManager` — 

[Docs](https://ruby.sketchup.com/Sketchup/Model.html#options-instance_method)

#### `overlays => Sketchup::OverlaysManager`



**Returns:** `Sketchup::OverlaysManager` — 

[Docs](https://ruby.sketchup.com/Sketchup/Model.html#overlays-instance_method)

#### `pages => Sketchup::Pages`

The #pages method retrieves a Pages object containing all of the pages in the model.

**Remarks:** The #pages method retrieves a Pages object containing all of the pages in the model.

**Returns:** `Sketchup::Pages` — 

[Docs](https://ruby.sketchup.com/Sketchup/Model.html#pages-instance_method)

#### `path => String`

The path method retrieves the path of the file from which the model was opened.

**Remarks:** The path method retrieves the path of the file from which the model was opened. An empty string is returned for a model that has not been saved.

**Returns:** `String` — 

[Docs](https://ruby.sketchup.com/Sketchup/Model.html#path-instance_method)

#### `place_component(componentdef, repeat = false) => Sketchup::Model?`

The place_component method places a new component in the Model using the component placement tool.

**Remarks:** The place_component method places a new component in the Model using the component placement tool.

**Parameters:**
- `componentdef` (Sketchup::ComponentDefinition) — A component definition object containing the definition (blueprint) for the component.
- `repeat` (Boolean) — If set to true, stay in the component placement tool and place multiple components.

**Returns:** `Sketchup::Model, nil` — The model object on success or Nil

[Docs](https://ruby.sketchup.com/Sketchup/Model.html#place_component-instance_method)

#### `point_to_latlong(point) => Geom::Point3d, Geom::LatLong`

The point_to_latlong method converts a point in the model to a LatLong so that you can get its latitude and longitude.

**Remarks:** The point_to_latlong method converts a point in the model to a LatLong so that you can get its latitude and longitude. This method uses the location information set in ShadowInfo. NOTE: SketchUp 6.0 and higher has a change where this method returns a Point3d instead of a LatLong, where the x and y values contain the LatLong coordinates.

**Parameters:**
- `point` (Geom::Point3d) — A Point3d object.

**Returns:** `Geom::Point3d` — Point3d[longitude_deg, latitude_deg, altitude_m]

[Docs](https://ruby.sketchup.com/Sketchup/Model.html#point_to_latlong-instance_method)

#### `point_to_utm(point) => Geom::UTM`

This method converts a Point3d object in the Model to UTM coordinates.

**Remarks:** This method converts a Point3d object in the Model to UTM coordinates. This method uses the location information set in ShadowInfo. See also UTM.

**Parameters:**
- `point` (Geom::Point3d) — A Point3d object.

**Returns:** `Geom::UTM` — a UTM object

[Docs](https://ruby.sketchup.com/Sketchup/Model.html#point_to_utm-instance_method)

#### `raytest(ray, wysiwyg_flag = true) => Array(Geom::Point3d, Array<Sketchup::Drawingelement>)?`

The #raytest method is used to cast a ray (line) through the model and return the first thing that the ray hits.

**Remarks:** The #raytest method is used to cast a ray (line) through the model and return the first thing that the ray hits. A ray is a two element array containing a point and a vector [Geom::Point3d, Geom::Vector3d]. The point defines the start point of the ray and the vector defines the direction. If direction can not be normalized (e.g. direction = [0, 0, 0]), direction is taken as a point the ray intersects.

**Parameters:**
- `ray` (Array(Geom::Point3d, Geom::Vector3d)) — A two element array containing a point and a vector.
- `wysiwyg_flag` (Boolean) — An optional boolean, added in SU8 M1, indicating whether or not to consider hidden geometry in intersect computations. If this flag is not specified, it defaults to true (WYSIWYG) - i.e. hidden geometry is not intersected against.

**Returns:** `Array(Geom::Point3d, Array<Sketchup::Drawingelement>), nil` — The first value is a Geom::Point3d where the item that the ray passed through exists. The second element is the instance path array of the entity that the ray hit. For example, if the ray hits a face that is contained by a component instance the instance path would be [Component1]. If the ray hit a face that is contained by a component instance, which is contained by another component instance and so on, the instance path would be [Component1, Component2, Component3…].

[Docs](https://ruby.sketchup.com/Sketchup/Model.html#raytest-instance_method)

#### `remove_observer(observer) => Boolean`

The remove_observer method is used to remove an observer from the current object.

**Remarks:** The remove_observer method is used to remove an observer from the current object.

**Parameters:**
- `observer` (Object) — An observer.

**Returns:** `Boolean` — true if successful, false if unsuccessful.

[Docs](https://ruby.sketchup.com/Sketchup/Model.html#remove_observer-instance_method)

#### `rendering_options => Sketchup::RenderingOptions`

The #rendering_options method retrieves the RenderingOptions object for this model.

**Remarks:** The #rendering_options method retrieves the RenderingOptions object for this model.

**Returns:** `Sketchup::RenderingOptions` — 

[Docs](https://ruby.sketchup.com/Sketchup/Model.html#rendering_options-instance_method)

#### `save => Boolean | save(path) => Boolean | save(path, version) => Boolean`

Note: A bug in SketchUp 2016 and older caused the .

**Remarks:** Note: A bug in SketchUp 2016 and older caused the .skb backup file written during save to be empty. The .skp file was however valid. Note: Starting with SketchUp 2021, SketchUp is using a the same file format across versions. For instance, SketchUp 2021 can open a file made in SketchUp 2022. This method is used to save the model to a file.

**Parameters:**
- `path` (String) — If empty, the model saves to the path it is already associated with.

**Returns:** `Boolean` — true if successful, false if unsuccessful

[Docs](https://ruby.sketchup.com/Sketchup/Model.html#save-instance_method)

#### `save_copy(path) => Boolean | save_copy(path, version) => Boolean`

This method is used to save the copy of the current model to a file.

**Remarks:** This method is used to save the copy of the current model to a file.

**Parameters:**
- `path` (String) — The path of the file to save the model copy to.

**Returns:** `Boolean` — true if successful, false if unsuccessful

[Docs](https://ruby.sketchup.com/Sketchup/Model.html#save_copy-instance_method)

#### `save_thumbnail(filename) => Boolean`

The save_thumbnail method is used to save a thumbnail image to a file.

**Remarks:** The save_thumbnail method is used to save a thumbnail image to a file. The image format is specified by the file extension of filename. Supported formats are bmp, jpg, png, tif, pct, and gif.

**Parameters:**
- `filename` (String) — The name of the file, with extension, to save the thumbnail as.

**Returns:** `Boolean` — true if successful, false if unsuccessful

[Docs](https://ruby.sketchup.com/Sketchup/Model.html#save_thumbnail-instance_method)

#### `select_tool(tool) => Sketchup::Model`

Note: If you call select_tool from within the initialize method of a custom tool, the call will be ignored.

**Remarks:** Note: If you call select_tool from within the initialize method of a custom tool, the call will be ignored. The #select_tool method is used to select a SketchUp Tool object as the active tool. To activate the native Select tool, pass nil as the argument. Before calling this method, you need to implement the SketchUp Tool interface and create a tool object. Then, pass the tool object to this method to activate it. the native Select tool.

**Parameters:**
- `tool` (Object) — A tool instance object to activate, or nil to activate

**Returns:** `Sketchup::Model` — The Model object.

[Docs](https://ruby.sketchup.com/Sketchup/Model.html#select_tool-instance_method)

#### `selection => Sketchup::Selection`

This method retrieves a Selection object for the model, containing the currently selected entities.

**Remarks:** This method retrieves a Selection object for the model, containing the currently selected entities. The entries in the selection list are not necessarily in the same order in which the user selected them.

**Returns:** `Sketchup::Selection` — A Selection object with 0 or more entities that are currently selected.

[Docs](https://ruby.sketchup.com/Sketchup/Model.html#selection-instance_method)

#### `set_attribute(attrdictname, key, value) => Object`

This method is used to set the value of an attribute in an attribute dictionary with the given name.

**Remarks:** This method is used to set the value of an attribute in an attribute dictionary with the given name. This method can be used create a new AttributeDictionary object, if needed.

**Parameters:**
- `attrdictname` (String) — The name of the attribute dictionary whose attribute you wish to set.
- `key` (String) — The attribute name.
- `value` (Object) — The value to set.

**Returns:** `Object` — the value that was set

[Docs](https://ruby.sketchup.com/Sketchup/Model.html#set_attribute-instance_method)

#### `set_datum(datum) => nil`

This method sets the datum used in conversions between the internal coordinate system and UTM.

**Remarks:** This method sets the datum used in conversions between the internal coordinate system and UTM. The default datum is WGS84. You can use the method list_datums to get a list of all of the datums supported in SketchUp. If you pass an invalid datum to set_datum, set_datum returns the default datum.

**Parameters:**
- `datum` (String)

**Returns:** `nil` — 

[Docs](https://ruby.sketchup.com/Sketchup/Model.html#set_datum-instance_method)

#### `shadow_info => Sketchup::ShadowInfo`

This method is used to retrieve the shadow information for the Model.

**Remarks:** This method is used to retrieve the shadow information for the Model.

**Returns:** `Sketchup::ShadowInfo` — 

[Docs](https://ruby.sketchup.com/Sketchup/Model.html#shadow_info-instance_method)

#### `start_operation(op_name, disable_ui = false, next_transparent = false, transparent = false) => Boolean`

Note: Operations in SketchUp are sequential and cannot be nested.

**Remarks:** Note: Operations in SketchUp are sequential and cannot be nested. If you start a new Ruby operation while another is still open, you will implicitly close the first one. The #start_operation method is used to notify SketchUp that a new operation (which can be undone) is starting. The op_name argument is a description for the operation that is displayed adjacent to the Edit > Undo menu item. Make sure to provide a user friendly name for your operation. Starting with SketchUp 7.0, there are three additional booleans that one can pass in when starting an operation. It is recommended to always set disable_ui to true. It's left to false for default for compatibility reasons.

**Parameters:**
- `op_name` (String) — name of the operation visible in the UI
- `disable_ui` (Boolean) — if set to true, then SketchUp's tendency to update the user interface after each geometry change will be suppressed. This can result in much faster Ruby code execution if the operation involves updating the model in any way.
- `next_transparent` (Boolean) — Deprecated! if set to true, then whatever operation comes after this one will be appended into one combined operation, allowing the user the undo both actions with a single undo command. This flag is a highly difficult one, since there are so many ways that a SketchUp user can interrupt a given operation with one of their own. Use extreme caution and test thoroughly when setting this to true.
- `transparent` (Boolean) — if set to true, then this operation will append to the previous operation. This is particularly useful for creating observers that react to user actions without littering the undo stack with extra steps that Ruby is performing.

**Returns:** `Boolean` — true if successful, false if unsuccessful

[Docs](https://ruby.sketchup.com/Sketchup/Model.html#start_operation-instance_method)

#### `styles => Sketchup::Styles`

The #styles method retrieves the styles associated with the model.

**Remarks:** The #styles method retrieves the styles associated with the model.

**Returns:** `Sketchup::Styles` — 

[Docs](https://ruby.sketchup.com/Sketchup/Model.html#styles-instance_method)

#### `title => String`

The #title method retrieves the name of the model.

**Remarks:** The #title method retrieves the name of the model. If the model is saved on disk, returns the file name without extension. Otherwise returns an empty string.

**Returns:** `String` — the title of the model or an empty string (if the title is not set)

[Docs](https://ruby.sketchup.com/Sketchup/Model.html#title-instance_method)

#### `tools => Sketchup::Tools`

The #tools method is used to retrieve the current Tools object.

**Remarks:** The #tools method is used to retrieve the current Tools object.

**Returns:** `Sketchup::Tools` — a Tools object.

[Docs](https://ruby.sketchup.com/Sketchup/Model.html#tools-instance_method)

#### `utm_to_point(utm) => Geom::Point3d`

The utm_to_point method converts a position given in UTM coordinates to a Point3d in the Model.

**Remarks:** The utm_to_point method converts a position given in UTM coordinates to a Point3d in the Model.

**Parameters:**
- `utm` (Geom::UTM) — A UTM object.

**Returns:** `Geom::Point3d` — A Point3d object.

[Docs](https://ruby.sketchup.com/Sketchup/Model.html#utm_to_point-instance_method)

#### `valid? => Boolean`

Determine if a model is a valid Sketchup::Model object.

**Remarks:** Determine if a model is a valid Sketchup::Model object. Returns false if the model has been closed. This is useful on the mac where one can have multiple models open at the same time. In such a case, this method can tell you if the user has closed the model before you perform operations on it.

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Sketchup/Model.html#valid?-instance_method)

### Properties
- `active_layer` (Sketchup::Layer, get/set) — The #active_layer method retrieves the active Layer.
- `active_path` (Array<Sketchup::Drawingelement>, nil, get/set) — Returns an array containing the sequence of entities the user has double-clicked on for editing.
- `description` (String, get/set) — The description method retrieves a description of the model as found in the Model Info > Files panel.
- `mipmapping` (Boolean, set) — This method can be used to turn mipmapping on or off.
- `name` (String, get/set) — The #name method retrieves the name of the model.
- `tags` (String, get/set) — The tags method retrieves the string tags of the model.
- `VERSION_3` (Object, get) — 
- `VERSION_4` (Object, get) — 
- `VERSION_5` (Object, get) — 
- `VERSION_6` (Object, get) — 
- `VERSION_7` (Object, get) — 
- `VERSION_8` (Object, get) — 
- `VERSION_2013` (Object, get) — 
- `VERSION_2014` (Object, get) — 
- `VERSION_2015` (Object, get) — 
- `VERSION_2016` (Object, get) — 
- `VERSION_2017` (Object, get) — 
- `VERSION_2018` (Object, get) — 
- `VERSION_2019` (Object, get) — 
- `VERSION_2020` (Object, get) — 
- `VERSION_2021` (Object, get) — 
- `ProTrial` (Object, get) — 
- `ProLicensed` (Object, get) — 
- `MakeTrial` (Object, get) — 
- `Make` (Object, get) — 
- `LOAD_STATUS_SUCCESS` (Object, get) — 
- `LOAD_STATUS_SUCCESS_MORE_RECENT` (Object, get) — 

## ModelObserver (class)

This class is abstract. To implement this observer, create a Ruby class of this type, override the desired methods, and add an instance of the observer to the model. This observer interface is implemented to react to model events. Note that the observers related to transactions (aka undoable operations) are primarily for reporting and debugging. Performing any edit operations of your own (such as modifying the model) inside the observer callback should be avoided, as it could cause crashes or model corruption. The most common use for these callbacks is to help debug problems where your Ruby script's Sketchup::Model#start_operation and Sketchup::Model#commit_operation calls are somehow conflicting with SketchUp's native undo operations. You can set up an observer set to watch precisely what is going on.

[Vendor docs](https://ruby.sketchup.com/Sketchup/ModelObserver.html)

### Methods
#### `onActivePathChanged(model) => nil`

The #onActivePathChanged method is invoked when the user opens or closes a ComponentInstance or Group for editing.

**Remarks:** The #onActivePathChanged method is invoked when the user opens or closes a ComponentInstance or Group for editing. When the user opens an instance for editing the positions and transformations of the entities in the opened instance will be relative to global world coordinates instead of the local coordinates relative to their parent. See Sketchup::Model#active_path and Sketchup::Model#edit_transform for methods that report the current edit origin vs. the global origin, etc. By using this observer callback, you can keep track of the various nested transformations as your users double click to drill into and out of component edits.

**Parameters:**
- `model` (Sketchup::Model)

**Returns:** `nil` — 

[Docs](https://ruby.sketchup.com/Sketchup/ModelObserver.html#onActivePathChanged-instance_method)

#### `onAfterComponentSaveAs(model) => nil`

The #onAfterComponentSaveAs method is invoked when the user context-clicks > Save As on a component instance.

**Remarks:** The #onAfterComponentSaveAs method is invoked when the user context-clicks > Save As on a component instance. It is called just after the component is written to disk, so you can restore the component to some state before returning control to the user.

**Parameters:**
- `model` (Sketchup::Model)

**Returns:** `nil` — 

[Docs](https://ruby.sketchup.com/Sketchup/ModelObserver.html#onAfterComponentSaveAs-instance_method)

#### `onBeforeComponentSaveAs(model) => nil`

The #onBeforeComponentSaveAs method is invoked when the user context-clicks > Save As on a component instance.

**Remarks:** The #onBeforeComponentSaveAs method is invoked when the user context-clicks > Save As on a component instance. It is called just before the component is written to disk, so you can make changes within the handler and it will make it into the save. For example, you may decide that you want to add some attribute to every component that is saved out, but you do not want that attribute sticking around inside the current model. Within #onBeforeComponentSaveAs you could add the attribute, and within #onAfterComponentSaveAs you could delete that attribute. The callback is not sent the component that is to be saved, but the model's selection will contain it.

**Parameters:**
- `model` (Sketchup::Model)

**Returns:** `nil` — 

[Docs](https://ruby.sketchup.com/Sketchup/ModelObserver.html#onBeforeComponentSaveAs-instance_method)

#### `onDeleteModel(model) => nil`

The #onDeleteModel method is invoked when a model is deleted.

**Remarks:** The #onDeleteModel method is invoked when a model is deleted.

**Parameters:**
- `model` (Sketchup::Model)

**Returns:** `nil` — 

[Docs](https://ruby.sketchup.com/Sketchup/ModelObserver.html#onDeleteModel-instance_method)

#### `onEraseAll(model) => nil`

The #onEraseAll method is invoked when everything in a model is erased.

**Remarks:** The #onEraseAll method is invoked when everything in a model is erased.

**Parameters:**
- `model` (Sketchup::Model)

**Returns:** `nil` — 

[Docs](https://ruby.sketchup.com/Sketchup/ModelObserver.html#onEraseAll-instance_method)

#### `onExplode(model) => nil`

The method is invoked whenever a component anywhere in this model is exploded.

**Remarks:** The method is invoked whenever a component anywhere in this model is exploded. This is an easier way to watch explode events vs. attaching an InstanceObserver to every instance in the model. Since the callback does not return what was exploded, one solution is to place a selection observer that keeps track of which entities whose explosion you are interested in are in the selection. Since SketchUp's user interface only provides a means of exploding the selection, this method is a reliable way to know what was just exploded. Another method would be to watch ComponentDefinition.count_instances to determine what just changed, as any components that were exploded will now be less an instance.

**Parameters:**
- `model` (Sketchup::Model)

**Returns:** `nil` — 

[Docs](https://ruby.sketchup.com/Sketchup/ModelObserver.html#onExplode-instance_method)

#### `onPidChanged(model, old_pid, new_pid) => nil`

Note: This callback is useful for tracking changes to entities that result in new PIDs, such as grouping or other modifications that result in new entities.

**Remarks:** Note: This callback is useful for tracking changes to entities that result in new PIDs, such as grouping or other modifications that result in new entities. The #onPidChanged method is invoked when a persistent id of an entity changes within the model.

**Parameters:**
- `model` (Sketchup::Model)
- `old_pid` (Integer) — The old persistent ID of the entity.
- `new_pid` (Integer) — The new persistent ID of the entity.

**Returns:** `nil` — 

[Docs](https://ruby.sketchup.com/Sketchup/ModelObserver.html#onPidChanged-instance_method)

#### `onPlaceComponent(model) => nil`

The #onPlaceComponent method is invoked when a component is “placed” into the model, meaning it is dragged from the Component Browser.

**Remarks:** The #onPlaceComponent method is invoked when a component is “placed” into the model, meaning it is dragged from the Component Browser.

**Parameters:**
- `model` (Sketchup::Model)

**Returns:** `nil` — 

[Docs](https://ruby.sketchup.com/Sketchup/ModelObserver.html#onPlaceComponent-instance_method)

#### `onPostSaveModel(model) => nil`

The #onPostSaveModel method is invoked after a model has been saved to disk.

**Remarks:** The #onPostSaveModel method is invoked after a model has been saved to disk.

**Parameters:**
- `model` (Sketchup::Model)

**Returns:** `nil` — 

[Docs](https://ruby.sketchup.com/Sketchup/ModelObserver.html#onPostSaveModel-instance_method)

#### `onPreSaveModel(model) => nil`

The #onPreSaveModel method is invoked before a model is saved to disk.

**Remarks:** The #onPreSaveModel method is invoked before a model is saved to disk.

**Parameters:**
- `model` (Sketchup::Model)

**Returns:** `nil` — 

[Docs](https://ruby.sketchup.com/Sketchup/ModelObserver.html#onPreSaveModel-instance_method)

#### `onSaveModel(model) => nil`

The #onSaveModel method is invoked after a model has been saved to disk.

**Remarks:** The #onSaveModel method is invoked after a model has been saved to disk.

**Parameters:**
- `model` (Sketchup::Model)

**Returns:** `nil` — 

[Docs](https://ruby.sketchup.com/Sketchup/ModelObserver.html#onSaveModel-instance_method)

#### `onTransactionAbort(model) => nil`

The #onTransactionAbort method is invoked when a transaction is aborted.

**Remarks:** The #onTransactionAbort method is invoked when a transaction is aborted.

**Parameters:**
- `model` (Sketchup::Model)

**Returns:** `nil` — 

[Docs](https://ruby.sketchup.com/Sketchup/ModelObserver.html#onTransactionAbort-instance_method)

#### `onTransactionCommit(model) => nil`

The #onTransactionCommit method is invoked when a transaction is completed.

**Remarks:** The #onTransactionCommit method is invoked when a transaction is completed.

**Parameters:**
- `model` (Sketchup::Model)

**Returns:** `nil` — 

[Docs](https://ruby.sketchup.com/Sketchup/ModelObserver.html#onTransactionCommit-instance_method)

#### `onTransactionEmpty(model) => nil`

The #onTransactionEmpty method is invoked when a transaction (aka an undoable operation) starts and then is committed without anything being altered in between.

**Remarks:** The #onTransactionEmpty method is invoked when a transaction (aka an undoable operation) starts and then is committed without anything being altered in between.

**Parameters:**
- `model` (Sketchup::Model)

**Returns:** `nil` — 

[Docs](https://ruby.sketchup.com/Sketchup/ModelObserver.html#onTransactionEmpty-instance_method)

#### `onTransactionRedo(model) => nil`

The #onTransactionRedo method is invoked when the user “redoes” a transaction (aka undo operation.

**Remarks:** The #onTransactionRedo method is invoked when the user “redoes” a transaction (aka undo operation.) You can programmatically fire a redo by calling Sketchup.sendAction(“editRedo”).

**Parameters:**
- `model` (Sketchup::Model)

**Returns:** `nil` — 

[Docs](https://ruby.sketchup.com/Sketchup/ModelObserver.html#onTransactionRedo-instance_method)

#### `onTransactionStart(model) => nil`

The #onTransactionStart method is invoked when a transaction (aka an undoable operation) starts.

**Remarks:** The #onTransactionStart method is invoked when a transaction (aka an undoable operation) starts.

**Parameters:**
- `model` (Sketchup::Model)

**Returns:** `nil` — 

[Docs](https://ruby.sketchup.com/Sketchup/ModelObserver.html#onTransactionStart-instance_method)

#### `onTransactionUndo(model) => nil`

The method is invoked when the user “undoes” a transaction (aka undo operation.

**Remarks:** The method is invoked when the user “undoes” a transaction (aka undo operation.) You can programmatically fire an undo by calling Sketchup.sendAction(“editUndo”).

**Parameters:**
- `model` (Sketchup::Model)

**Returns:** `nil` — 

[Docs](https://ruby.sketchup.com/Sketchup/ModelObserver.html#onTransactionUndo-instance_method)

## OptionsManager (class)

The OptionsManager class manages various kinds of OptionsProviders on a Model. To get the OptionsProvider key list, run the following code in the ruby console:

[Vendor docs](https://ruby.sketchup.com/Sketchup/OptionsManager.html)

### Methods
#### `[](index) => Object | [](name) => Object`

The [] method is used to get an option provider by name or index.

**Remarks:** The [] method is used to get an option provider by name or index. For example, to get the UnitsOptions on the Model, you could use the command:

**Parameters:**
- `index` (Object) — The index of the OptionsProvider object.

**Returns:** `Object` — optionprovider - an OptionsProvider object if successful, otherwise nil.

[Docs](https://ruby.sketchup.com/Sketchup/OptionsManager.html#[]-instance_method)

#### `count => Object`

Note: Since SketchUp 2014 the count method is inherited from Ruby's Enumerable mix-in module.

**Remarks:** Note: Since SketchUp 2014 the count method is inherited from Ruby's Enumerable mix-in module. Prior to that the #count method is an alias for #length. Returns integer - number of OptionsProvider objects if successful.

**Returns:** `Object` — integer - number of OptionsProvider objects if successful

[Docs](https://ruby.sketchup.com/Sketchup/OptionsManager.html#count-instance_method)

#### `each {|provider| ... } => nil`

The #each method is used to iterate through options providers.

**Remarks:** The #each method is used to iterate through options providers.

**Returns:** `nil` — 

[Docs](https://ruby.sketchup.com/Sketchup/OptionsManager.html#each-instance_method)

#### `keys => Object`

The keys method is used to get a list of keys in the OptionsManager.

**Remarks:** The keys method is used to get a list of keys in the OptionsManager.

**Returns:** `Object` — keys - Array of string keys

[Docs](https://ruby.sketchup.com/Sketchup/OptionsManager.html#keys-instance_method)

#### `length => Integer`

The #length method is an alias of #size.

**Remarks:** The #length method is an alias of #size.

**Returns:** `Integer` — 

[Docs](https://ruby.sketchup.com/Sketchup/OptionsManager.html#length-instance_method)

#### `size => Integer`

The #size method returns the number of Sketchup::OptionsProvider objects inside this Sketchup::OptionsManager.

**Remarks:** The #size method returns the number of Sketchup::OptionsProvider objects inside this Sketchup::OptionsManager.

**Returns:** `Integer` — 

[Docs](https://ruby.sketchup.com/Sketchup/OptionsManager.html#size-instance_method)

## OptionsProvider (class)

An OptionsProvider class provides various kinds of options on a Model. You get an OptionsProvider from the OptionsManager. The options are given as name/value pairs. List of keys added in different SketchUp versions: UnitsOptions AreaUnit (SketchUp 2019.2) VolumeUnit (SketchUp 2019.2) AreaPrecision (SketchUp 2020.0) VolumePrecision (SketchUp 2020.0) The AreaUnit and VolumeUnit options in UnitsOptions only applies if the UnitFormat is Length::Decimal.

[Vendor docs](https://ruby.sketchup.com/Sketchup/OptionsProvider.html)

### Methods
#### `add_observer(observer) => Object`

The add_observer method is used to add an observer to the current object.

**Remarks:** The add_observer method is used to add an observer to the current object.

**Parameters:**
- `observer` (Object) — An observer.

**Returns:** `Object` — true if successful, false if unsuccessful.

[Docs](https://ruby.sketchup.com/Sketchup/OptionsProvider.html#add_observer-instance_method)

#### `count => Integer`

Note: Since SketchUp 2014 the count method is inherited from Ruby's Enumerable mix-in module.

**Remarks:** Note: Since SketchUp 2014 the count method is inherited from Ruby's Enumerable mix-in module. Prior to that the #count method is an alias for #length.

**Returns:** `Integer` — 

[Docs](https://ruby.sketchup.com/Sketchup/OptionsProvider.html#count-instance_method)

#### `each {|key, value| ... } => nil`

The #each method is used to iterate through all of the options.

**Remarks:** The #each method is used to iterate through all of the options.

**Returns:** `nil` — 

[Docs](https://ruby.sketchup.com/Sketchup/OptionsProvider.html#each-instance_method)

#### `each_key {|key| ... } => nil`

The #each_key method is used to iterate through all of the attribute keys.

**Remarks:** The #each_key method is used to iterate through all of the attribute keys.

**Returns:** `nil` — 

[Docs](https://ruby.sketchup.com/Sketchup/OptionsProvider.html#each_key-instance_method)

#### `each_pair {|key, value| ... } => nil`

The #each method is used to iterate through all of the options.

**Remarks:** The #each method is used to iterate through all of the options.

**Returns:** `nil` — 

[Docs](https://ruby.sketchup.com/Sketchup/OptionsProvider.html#each_pair-instance_method)

#### `each_value {|value| ... } => nil`

The #each_value method is used to iterate through all of the attribute values.

**Remarks:** The #each_value method is used to iterate through all of the attribute values.

**Returns:** `nil` — 

[Docs](https://ruby.sketchup.com/Sketchup/OptionsProvider.html#each_value-instance_method)

#### `has_key?(name) => Boolean`

The #has_key? method is an alias for #key?.

**Remarks:** The #has_key? method is an alias for #key?.

**Parameters:**
- `name` (String) — The name of the key you are looking for.

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Sketchup/OptionsProvider.html#has_key?-instance_method)

#### `key?(name) => Boolean`

The #key? method is used to determine if the options provider has a specific key.

**Remarks:** The #key? method is used to determine if the options provider has a specific key.

**Parameters:**
- `name` (String) — The name of the key you are looking for.

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Sketchup/OptionsProvider.html#key?-instance_method)

#### `keys => Object`

The #keys method is used to retrieve an array with all of the attribute keys.

**Remarks:** The #keys method is used to retrieve an array with all of the attribute keys.

**Returns:** `Object` — keys - an array of keys within the options provider if successful

[Docs](https://ruby.sketchup.com/Sketchup/OptionsProvider.html#keys-instance_method)

#### `length => Integer`

The #length method is an alias of #size.

**Remarks:** The #length method is an alias of #size.

**Returns:** `Integer` — 

[Docs](https://ruby.sketchup.com/Sketchup/OptionsProvider.html#length-instance_method)

#### `name => Object`

The name method is used to retrieve the name of an options provider.

**Remarks:** The name method is used to retrieve the name of an options provider.

**Returns:** `Object` — name - the name of the options provider if successful

[Docs](https://ruby.sketchup.com/Sketchup/OptionsProvider.html#name-instance_method)

#### `remove_observer(observer) => Object`

The remove_observer method is used to remove an observer from the current object.

**Remarks:** The remove_observer method is used to remove an observer from the current object.

**Parameters:**
- `observer` (Object) — An observer.

**Returns:** `Object` — true if successful, false if unsuccessful.

[Docs](https://ruby.sketchup.com/Sketchup/OptionsProvider.html#remove_observer-instance_method)

#### `size => Integer`

The #size method is used to retrieve the size (number of elements) of an options provider.

**Remarks:** The #size method is used to retrieve the size (number of elements) of an options provider.

**Returns:** `Integer` — 

[Docs](https://ruby.sketchup.com/Sketchup/OptionsProvider.html#size-instance_method)

### Properties
- `[]` (Object, get/set) — The [] method is used to get a value by name or index of the key.

## OptionsProviderObserver (class)

This class is abstract. To implement this observer, create a Ruby class of this type, override the desired methods, and add an instance of the observer to the objects of interests. This observer interface is implemented to react to operations provider events. What this means is that you can watch as the user changes SketchUp options and react to them. The following OptionsProviders are defined in the API: "UnitsOptions" "PrintOptions" "PageOptions" "SlideshowOptions" "NamedOptions" Each of these has a list of specific options that the user can set. See the OptionsManager and OptionsProvider classes for more details.

[Vendor docs](https://ruby.sketchup.com/Sketchup/OptionsProviderObserver.html)

### Methods
#### `onOptionsProviderChanged(provider, name) => nil`

The #onOptionsProviderChanged method is invoked when a property of an Sketchup::OptionsProvider changes.

**Remarks:** The #onOptionsProviderChanged method is invoked when a property of an Sketchup::OptionsProvider changes.

**Parameters:**
- `provider` (Sketchup::OptionsProvider)
- `name` (String) — The name of the specific option that was changed.

**Returns:** `nil` — 

[Docs](https://ruby.sketchup.com/Sketchup/OptionsProviderObserver.html#onOptionsProviderChanged-instance_method)

## Overlay (class)

An Overlay provides contextual model information directly in the viewport. This can be presented in 2D and 3D. Examples can be annotations or analytical model information such as geometry analysis, energy analysis, etc. The overlay feature is not intended as a mechanism to provide custom entities to SketchUp. Whatever overlays draw is not pickable nor exportable. It is also not allowed to perform model changes from overlay events. Doing so will result in a RuntimeError being thrown.

[Vendor docs](https://ruby.sketchup.com/Sketchup/Overlay.html)

### Constructors
- `Overlay(id, name, description: "")` — 

### Methods
#### `draw(view) => Object`

This method is abstract.

**Remarks:** This method is abstract. It is called whenever the view updates. Note: This is called very often. Perform minimal amount of computation in this event. Cache the data needed to draw what the overlay needs whenever possible. Note: If you draw outside the model bounds you need to implement #getExtents which return a bounding box large enough to include the points you draw. Otherwise your drawing will be clipped.

**Parameters:**
- `view` (Sketchup::View) — A View object where the method was invoked.

[Docs](https://ruby.sketchup.com/Sketchup/Overlay.html#draw-instance_method)

#### `enabled? => Boolean`



**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Sketchup/Overlay.html#enabled?-instance_method)

#### `getExtents => Geom::BoundingBox`

This method is abstract.

**Remarks:** This method is abstract. The method should be implementing sub-classes ensure what is drawn in 3D space doesn't appear clipped. If the overlay only draws in 2D this isn't needed. Note: This is called very often. Perform minimal amount of computation in this event. Cache the data needed to compute the bounds of what the overlay draws whenever possible. In order to accurately draw things, SketchUp needs to know the extents of what it is drawing. If the overlay is doing its own drawing, it may need to implement this method to tell SketchUp the extents of what it will be drawing. If you don't implement this method, you may find that part of what the overlay is drawing gets clipped to the extents of the rest of the model. This must return a Geom::BoundingBox. In a typical implementation, you will create a new Geom::BoundingBox, add points to set the extents of the drawing that the overlay will do and then return it.

**Returns:** `Geom::BoundingBox` — 

[Docs](https://ruby.sketchup.com/Sketchup/Overlay.html#getExtents-instance_method)

#### `name => String`

This is a user facing display name that will appear in the UI.

**Remarks:** This is a user facing display name that will appear in the UI.

**Returns:** `String` — 

[Docs](https://ruby.sketchup.com/Sketchup/Overlay.html#name-instance_method)

#### `onMouseEnter(flags, x, y, view) => Object`

This method is abstract.

**Remarks:** This method is abstract. It can be used by implementing sub-classes to react to mouse movement in the viewport. The #onMouseEnter method is called by SketchUp when the mouse enters the viewport.

**Parameters:**
- `flags` (Integer) — A bit mask that tells the state of the modifier keys and other mouse buttons at the time.
- `x` (Integer) — The X coordinate on the screen where the event occurred.
- `y` (Integer) — The Y coordinate on the screen where the event occurred.
- `view` (Sketchup::View)

[Docs](https://ruby.sketchup.com/Sketchup/Overlay.html#onMouseEnter-instance_method)

#### `onMouseLeave(view) => Object`

This method is abstract.

**Remarks:** This method is abstract. It can be used by implementing sub-classes to react to mouse movement in the viewport. The #onMouseLeave method is called by SketchUp when the mouse enters the viewport.

**Parameters:**
- `view` (Sketchup::View)

[Docs](https://ruby.sketchup.com/Sketchup/Overlay.html#onMouseLeave-instance_method)

#### `onMouseMove(flags, x, y, view) => Object | onMouseMove(flags, x, y, view) => Object`

This method is abstract.

**Remarks:** This method is abstract. It can be used by implementing sub-classes to react to mouse movement in the viewport. Try to make this method as efficient as possible because this method is called often.

**Parameters:**
- `flags` (Integer) — A bit mask that tells the state of the modifier keys and other mouse buttons at the time.
- `x` (Integer) — Screen coordinate in physical pixels.
- `y` (Integer) — Screen coordinate in physical pixels.
- `view` (Sketchup::View)

[Docs](https://ruby.sketchup.com/Sketchup/Overlay.html#onMouseMove-instance_method)

#### `overlay_id => String`



**Returns:** `String` — 

[Docs](https://ruby.sketchup.com/Sketchup/Overlay.html#overlay_id-instance_method)

#### `source => String`

Describes the source associated with the overlay.

**Remarks:** Describes the source associated with the overlay. This is automatically inferred when the overlay instance is initialized.

**Returns:** `String` — 

[Docs](https://ruby.sketchup.com/Sketchup/Overlay.html#source-instance_method)

#### `start => Object`

This method is abstract.

**Remarks:** This method is abstract. It can be used by implementing sub-classes to react when the overlay becomes active, for instance when the user turns it on.

[Docs](https://ruby.sketchup.com/Sketchup/Overlay.html#start-instance_method)

#### `stop => Object`

This method is abstract.

**Remarks:** This method is abstract. It can be used by implementing sub-classes to react when the overlay becomes inactive, for instance when the user turns it off.

[Docs](https://ruby.sketchup.com/Sketchup/Overlay.html#stop-instance_method)

#### `valid? => Boolean`

Indicates whether the overlay is valid.

**Remarks:** Indicates whether the overlay is valid. An overlay becomes invalid after being removed from the model and cannot be reused.

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Sketchup/Overlay.html#valid?-instance_method)

### Properties
- `description` (String, get/set) — This is a short user facing description of the overlay that will appear in the UI.
- `enabled` (Boolean, set) — Note: In most cases, extensions doesn't need to expose any new UI for enabling them.

## OverlaysManager (class)

An overlay added to a model is invalidated once it's removed from the model. It cannot be re-added or added to another model. Create another unique instance for that. Contains the registered overlays for a model along with methods to manage them. A model can not have multiple overlays with the same id.

**Remarks:** See Also: Documentation for overlays for more details.

[Vendor docs](https://ruby.sketchup.com/Sketchup/OverlaysManager.html)

### Methods
#### `[](index) => Sketchup::Overlay Also known as: at`



**Returns:** `Sketchup::Overlay` — 

[Docs](https://ruby.sketchup.com/Sketchup/OverlaysManager.html#[]-instance_method)

#### `add(service) => Boolean`

Returns false if an overlay with the same id already exists.

**Remarks:** Returns false if an overlay with the same id already exists.

**Returns:** `Boolean` — false if an overlay with the same id already exists.

[Docs](https://ruby.sketchup.com/Sketchup/OverlaysManager.html#add-instance_method)

#### `each {|overlay| ... } => nil`



**Returns:** `nil` — 

[Docs](https://ruby.sketchup.com/Sketchup/OverlaysManager.html#each-instance_method)

#### `remove(service) => Boolean`



**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Sketchup/OverlaysManager.html#remove-instance_method)

#### `size => Integer Also known as: length`



**Returns:** `Integer` — 

[Docs](https://ruby.sketchup.com/Sketchup/OverlaysManager.html#size-instance_method)

## Page (class)

The Page class contains methods to extract information and modify the properties of an individual page. Note that inside the SketchUp user interface pages are called “Scenes”. Since SketchUp 2026.0, modifying the Axes, Camera, RenderingOptions, and ShadowInfo properties of a page is an undoable operation and should be wrapped between Model#start_operation and Model#commit_operation. Example:

[Vendor docs](https://ruby.sketchup.com/Sketchup/Page.html)

### Methods
#### `active_section_planes => Array<Sketchup::SectionPlane>?`

The #active_section_planes method is used to retrieve the active section plane for the Sketchup::Page.

**Remarks:** The #active_section_planes method is used to retrieve the active section plane for the Sketchup::Page.

**Returns:** `Array<Sketchup::SectionPlane>, nil` — Returns nil if the page does not use section planes.

[Docs](https://ruby.sketchup.com/Sketchup/Page.html#active_section_planes-instance_method)

#### `axes => Sketchup::Axes`

The axes method returns the drawing axes for the page.

**Remarks:** The axes method returns the drawing axes for the page. Since SketchUp 2026.0, modifying the axes of a scene is an undoable operation.

**Returns:** `Sketchup::Axes` — 

[Docs](https://ruby.sketchup.com/Sketchup/Page.html#axes-instance_method)

#### `camera => Sketchup::Camera`

The #camera method retrieves the camera for a particular page.

**Remarks:** The #camera method retrieves the camera for a particular page. Since SketchUp 2026.0, modifying the camera properties of a scene is an undoable operation.

**Returns:** `Sketchup::Camera` — 

[Docs](https://ruby.sketchup.com/Sketchup/Page.html#camera-instance_method)

#### `get_drawingelement_visibility(element) => Boolean`

The #get_drawingelement_visibility method is used to get the visibility of a drawing element on a particular page.

**Remarks:** The #get_drawingelement_visibility method is used to get the visibility of a drawing element on a particular page.

**Parameters:**
- `element` (Sketchup::Drawingelement)

**Returns:** `Boolean` — true if visible, false if not.

[Docs](https://ruby.sketchup.com/Sketchup/Page.html#get_drawingelement_visibility-instance_method)

#### `hidden_entities => Array<Sketchup::Drawingelement>`

The hidden_entities method retrieves all hidden entities within a page.

**Remarks:** The hidden_entities method retrieves all hidden entities within a page.

**Returns:** `Array<Sketchup::Drawingelement>` — an array of drawing elements that are hidden on the page.

[Docs](https://ruby.sketchup.com/Sketchup/Page.html#hidden_entities-instance_method)

#### `include_in_animation? => Boolean`

The #include_in_animation? method determines whether the page should be included when exporting an animation from the model.

**Remarks:** The #include_in_animation? method determines whether the page should be included when exporting an animation from the model.

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Sketchup/Page.html#include_in_animation?-instance_method)

#### `label => Object`

The label method retrieves the label for a page from the page tab.

**Remarks:** The label method retrieves the label for a page from the page tab.

**Returns:** `Object` — label - a string label for the page tab

[Docs](https://ruby.sketchup.com/Sketchup/Page.html#label-instance_method)

#### `layer_folders => Array<Sketchup::LayerFolder>?`

The #layer_folders method retrieves the hidden layer folders associated with a page.

**Remarks:** The #layer_folders method retrieves the hidden layer folders associated with a page.

**Returns:** `Array<Sketchup::LayerFolder>, nil` — Returns nil if #use_hidden_layers? returns false

[Docs](https://ruby.sketchup.com/Sketchup/Page.html#layer_folders-instance_method)

#### `layers => Array<Sketchup::Layer>?`

The #layers method retrieves layers that don't use their default visibility on this page.

**Remarks:** The #layers method retrieves layers that don't use their default visibility on this page.

**Returns:** `Array<Sketchup::Layer>, nil` — Returns nil if #use_hidden_layers? returns false

[Docs](https://ruby.sketchup.com/Sketchup/Page.html#layers-instance_method)

#### `rendering_options => Sketchup::RenderingOptions`

Note: Most rendering options of a scene are also present in Style and are governed by the selected style.

**Remarks:** Note: Most rendering options of a scene are also present in Style and are governed by the selected style. Those options should not be changed from the scene. The ones not related to Style like fog (DisplayFog, FogColor) are safe to be changed from the scene. The rendering_options method retrieves a RenderingOptions object for the page. Since SketchUp 2026.0, modifying rendering_options of a scene is an undoable operation.

**Returns:** `Sketchup::RenderingOptions` — 

[Docs](https://ruby.sketchup.com/Sketchup/Page.html#rendering_options-instance_method)

#### `set_drawingelement_visibility(element, visibility) => Boolean`

The #set_drawingelement_visibility method is used to change the visibility of a drawing element on a particular page.

**Remarks:** The #set_drawingelement_visibility method is used to change the visibility of a drawing element on a particular page. Only drawing elements on the root of the model, as well as nested instances of components, groups, and images are controlled by Page visibility.

**Parameters:**
- `element` (Sketchup::Drawingelement)
- `visibility` (Boolean)

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Sketchup/Page.html#set_drawingelement_visibility-instance_method)

#### `set_visibility(layer, visible_for_page) => Sketchup::Page | set_visibility(layer_folder, visible_for_page) => Sketchup::Page`

The #set_visibility method sets the visibility for a layer or layer folder on a page.

**Remarks:** The #set_visibility method sets the visibility for a layer or layer folder on a page.

**Parameters:**
- `layer` (Sketchup::Layer)
- `visible_for_page` (Boolean)

**Returns:** `Sketchup::Page` — the page whose visibility was set.

[Docs](https://ruby.sketchup.com/Sketchup/Page.html#set_visibility-instance_method)

#### `shadow_info => Sketchup::ShadowInfo`

Note: While certain shadow settings, such as those available in the Shadows panel, can be controlled on a per-page basis, global settings like north angle, latitude, and longitude are managed at the model level and are not page-specific.

**Remarks:** Note: While certain shadow settings, such as those available in the Shadows panel, can be controlled on a per-page basis, global settings like north angle, latitude, and longitude are managed at the model level and are not page-specific. The #shadow_info method retrieves the ShadowInfo object for the page. Since SketchUp 2026.0, modifying shadow_info of a scene is an undoable operation.

**Returns:** `Sketchup::ShadowInfo` — 

[Docs](https://ruby.sketchup.com/Sketchup/Page.html#shadow_info-instance_method)

#### `style => Object`

The style method retrieves the style associated with the page.

**Remarks:** The style method retrieves the style associated with the page.

**Returns:** `Object` — style - the Style object if successful

[Docs](https://ruby.sketchup.com/Sketchup/Page.html#style-instance_method)

#### `update(flags) => Boolean`

The #update method performs an update on the page properties based on the current view that the user has.

**Remarks:** The #update method performs an update on the page properties based on the current view that the user has. What properties of the Page get updated are controlled via an integer whose bits corresponds to different properties. These flags can be used individually or combined using bitwise OR.

**Parameters:**
- `flags` (Integer) — The bitwise OR of the bit flags.

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Sketchup/Page.html#update-instance_method)

#### `use_axes? => Boolean`

The use_axes? method determines whether you are storing the axes property with the page.

**Remarks:** The use_axes? method determines whether you are storing the axes property with the page.

**Returns:** `Boolean` — status - true if you are storing the this property with the page, false if you are not storing this property with the page.

[Docs](https://ruby.sketchup.com/Sketchup/Page.html#use_axes?-instance_method)

#### `use_camera? => Boolean`

The use_camera? method determines whether you are storing the camera property with the page.

**Remarks:** The use_camera? method determines whether you are storing the camera property with the page.

**Returns:** `Boolean` — status - true if you are storing the this property with the page, false if you are not storing this property with the page.

[Docs](https://ruby.sketchup.com/Sketchup/Page.html#use_camera?-instance_method)

#### `use_environment? => Boolean`

The #use_environment? method is used to determine if the Environment settings are used in the scene.

**Remarks:** The #use_environment? method is used to determine if the Environment settings are used in the scene.

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Sketchup/Page.html#use_environment?-instance_method)

#### `use_hidden? => Boolean`

Deprecated.

**Remarks:** Deprecated. The functionality is replaced by #use_hidden_geometry? and #use_hidden_objects? in SketchUp 2020.1. The use_hidden? method determines whether you are storing the hidden property with the page.

**Returns:** `Boolean` — status - true if you are storing the this property with the page, false if you are not storing this property with the page.

[Docs](https://ruby.sketchup.com/Sketchup/Page.html#use_hidden?-instance_method)

#### `use_hidden_geometry? => Boolean`

Returns the use hidden geometry property from the page.

**Remarks:** Returns the use hidden geometry property from the page.

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Sketchup/Page.html#use_hidden_geometry?-instance_method)

#### `use_hidden_layers? => Boolean`

The use_hidden_layers? method determines whether you are storing the hidden layers property with the page.

**Remarks:** The use_hidden_layers? method determines whether you are storing the hidden layers property with the page.

**Returns:** `Boolean` — status - true if you are storing the this property with the page, false if you are not storing this property with the page.

[Docs](https://ruby.sketchup.com/Sketchup/Page.html#use_hidden_layers?-instance_method)

#### `use_hidden_objects? => Boolean`

Returns the use hidden objects property from the page.

**Remarks:** Returns the use hidden objects property from the page.

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Sketchup/Page.html#use_hidden_objects?-instance_method)

#### `use_rendering_options? => Boolean`

The use_rendering_options? method determines whether you are storing the rendering options property with the page.

**Remarks:** The use_rendering_options? method determines whether you are storing the rendering options property with the page.

**Returns:** `Boolean` — status - true if you are storing the this property with the page, false if you are not storing this property with the page.

[Docs](https://ruby.sketchup.com/Sketchup/Page.html#use_rendering_options?-instance_method)

#### `use_section_planes? => Boolean`

The use_section_planes? method determines whether you are storing the section planes property with the page.

**Remarks:** The use_section_planes? method determines whether you are storing the section planes property with the page.

**Returns:** `Boolean` — status - true if you are storing the this property with the page, false if you are not storing this property with the page.

[Docs](https://ruby.sketchup.com/Sketchup/Page.html#use_section_planes?-instance_method)

#### `use_shadow_info? => Boolean`

The use_shadow_info? method determines whether you are storing the shadow info property with the page.

**Remarks:** The use_shadow_info? method determines whether you are storing the shadow info property with the page.

**Returns:** `Boolean` — status - true if you are storing the this property with the page, false if you are not storing this property with the page.

[Docs](https://ruby.sketchup.com/Sketchup/Page.html#use_shadow_info?-instance_method)

#### `use_style? => Boolean`

The use_style? method determines whether storing a style with the page.

**Remarks:** The use_style? method determines whether storing a style with the page.

**Returns:** `Boolean` — status - true if you are storing the this property with the page, false if you are not storing this property with the page.

[Docs](https://ruby.sketchup.com/Sketchup/Page.html#use_style?-instance_method)

### Properties
- `delay_time` (Object, get/set) — The delay_time method retrieves the amount of time, in seconds, that a page will be displayed before transition to another page during a tour.
- `description` (Object, get/set) — The description method retrieves the description for a page as found in the Scenes manager dialog.
- `environment` (Sketchup::Environment, get/set) — The #environment method is used to retrieve the Environment for the scene.
- `include_in_animation` (Boolean, set) — The #include_in_animation= method controls whether the page should be included when exporting an animation from the model.
- `name` (Object, get/set) — The name method retrieves the name for a page from the page tab.
- `transition_time` (Object, get/set) — Get the amount of time that it takes to transition to this page during a slideshow or animation export.
- `use_axes` (Object, set) — The use_axes= method sets the page's axes property.
- `use_camera` (Object, set) — The use_camera= method sets the page's camera property.
- `use_environment` (Boolean, set) — The #use_environment= method is used to set if the Environment settings are used in the scene.
- `use_hidden` (Object, set) — Deprecated.
- `use_hidden_geometry` (Boolean, set) — Sets the page's use hidden geometry property.
- `use_hidden_layers` (Object, set) — The use_hidden_layers= method sets the page's hidden layers property.
- `use_hidden_objects` (Boolean, set) — Sets the page's use hidden objects property.
- `use_rendering_options` (Object, set) — The use_rendering_optoins= method sets the page's display settings property.
- `use_section_planes` (Object, set) — The use_section_planes= method sets the page's section planes property.
- `use_shadow_info` (Object, set) — The use_shadow_info= method sets the page's shadow info property.
- `use_style` (Object, set) — The use_style= method sets the style to be used by the page.

## Pages (class)

The Pages class contains methods for manipulating a collection of Pages (Named “scenes” in the UI.) in a model. You get a handle to this collection by calling Model.pages.

[Vendor docs](https://ruby.sketchup.com/Sketchup/Pages.html)

### Methods
#### `[](index_or_name) => Object`

The [] method retrieves a page by either name or index.

**Remarks:** The [] method retrieves a page by either name or index.

**Parameters:**
- `index_or_name` (Object) — The index or the string name of the specific page.

**Returns:** `Object` — page - a Page object if successful

[Docs](https://ruby.sketchup.com/Sketchup/Pages.html#[]-instance_method)

#### `add(name = nil, flags = PAGE_USE_ALL, index = self.size) => Sketchup::Page`

The #add method is used to add a new Page object to the collection.

**Remarks:** The #add method is used to add a new Page object to the collection. If no name is given, then a new name is generated using the default name for new Pages. If a name is given, then a new Page with that name is added. If the name is already used by another page, a unique name is created. If the flags parameter is given, it controls which properties are saved with the Page. See the Sketchup::Page#update method for a description of the flags that can be set. If index is given, it specifies the position in the page list that the new page is added. Otherwise the new page is added to the end.

**Parameters:**
- `name` (String) — The name of the specific page.
- `flags` (Integer) — Bit flags in integer form.
- `index` (Integer) — Index of where to inset.

**Returns:** `Sketchup::Page` — 

[Docs](https://ruby.sketchup.com/Sketchup/Pages.html#add-instance_method)

#### `add_frame_change_observer(object) => Integer`

The add_frame_change_observer method is used to add a new frame change observer that is called with each frame of an animation, meaning the end user has clicked on a Scene tab (aka Page) inside SketchUp and the camera is animating to that scene.

**Remarks:** The add_frame_change_observer method is used to add a new frame change observer that is called with each frame of an animation, meaning the end user has clicked on a Scene tab (aka Page) inside SketchUp and the camera is animating to that scene. The method returns an integer id that can be stored and later used to remove the observer with the remove_frame_change_observer method.

**Parameters:**
- `object` (#frameChange) — An object that implements the FrameChangeObserver#frameChange method.

**Returns:** `Integer` — A unique id of the observer

[Docs](https://ruby.sketchup.com/Sketchup/Pages.html#add_frame_change_observer-class_method)

#### `add_matchphoto_page(image_filename, camera = nil, page_name = nil) => Sketchup::Page`

The #add_matchphoto_page method is used to add a photomatch page to the model.

**Remarks:** The #add_matchphoto_page method is used to add a photomatch page to the model. This is an advanced feature that was added to support internal SketchUp work, so it is unlikely to be useful to you.

**Parameters:**
- `image_filename` (String) — The image file to use as the background.
- `camera` (Sketchup::Camera)
- `page_name` (String)

**Returns:** `Sketchup::Page` — 

[Docs](https://ruby.sketchup.com/Sketchup/Pages.html#add_matchphoto_page-instance_method)

#### `add_observer(observer) => Object`

The add_observer method is used to add an observer to the Pages object.

**Remarks:** The add_observer method is used to add an observer to the Pages object. See the PagesObserver interface for more details.

**Parameters:**
- `observer` (Object) — An observer.

**Returns:** `Object` — true if successful, false if unsuccessful.

[Docs](https://ruby.sketchup.com/Sketchup/Pages.html#add_observer-instance_method)

#### `count => Integer`

Note: Since SketchUp 2014 the count method is inherited from Ruby's Enumerable mix-in module.

**Remarks:** Note: Since SketchUp 2014 the count method is inherited from Ruby's Enumerable mix-in module. Prior to that the #count method is an alias for #length.

**Returns:** `Integer` — 

[Docs](https://ruby.sketchup.com/Sketchup/Pages.html#count-instance_method)

#### `each {|page| ... } => nil`

The #each method is used to iterate through pages in the model.

**Remarks:** The #each method is used to iterate through pages in the model.

**Returns:** `nil` — 

[Docs](https://ruby.sketchup.com/Sketchup/Pages.html#each-instance_method)

#### `erase(page) => Boolean`

The #erase method is used to remove a page from the collection.

**Remarks:** The #erase method is used to remove a page from the collection.

**Parameters:**
- `page` (Sketchup::Page)

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Sketchup/Pages.html#erase-instance_method)

#### `length => Integer`

The #length method is an alias for #size.

**Remarks:** The #length method is an alias for #size.

**Returns:** `Integer` — 

[Docs](https://ruby.sketchup.com/Sketchup/Pages.html#length-instance_method)

#### `parent => Object`

The parent method is used to determine the model for the Pages collection.

**Remarks:** The parent method is used to determine the model for the Pages collection.

**Returns:** `Object` — model - the model that contains the pages if successful

[Docs](https://ruby.sketchup.com/Sketchup/Pages.html#parent-instance_method)

#### `remove_frame_change_observer(observer_id) => Boolean`

The remove_frame_change_observer method is used to remove a frame change observer

**Remarks:** The remove_frame_change_observer method is used to remove a frame change observer

**Parameters:**
- `observer_id` (Integer) — The unique id returned by add_frame_change_observer

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Sketchup/Pages.html#remove_frame_change_observer-class_method)

#### `remove_observer(observer) => Object`

The remove_observer method is used to remove an observer from the current object.

**Remarks:** The remove_observer method is used to remove an observer from the current object. See the PagesObserver interface for more details.

**Parameters:**
- `observer` (Object) — An observer.

**Returns:** `Object` — true if successful, false if unsuccessful.

[Docs](https://ruby.sketchup.com/Sketchup/Pages.html#remove_observer-instance_method)

#### `reorder(page, new_index) => Object`

The #reorder method is used to reorder an existing Sketchup::Page object inside collection.

**Remarks:** The #reorder method is used to reorder an existing Sketchup::Page object inside collection. new_index specifies the new position of the page. It should be a value between 0 and the max index of the Sketchup::Pages collection. Negative indices can be used to specify an index from the end of the list.

**Parameters:**
- `page` (Sketchup::Page) — The page to be reordered.
- `new_index` (Integer) — Index of where to replace the page.

**Returns:** `Object` — nil

[Docs](https://ruby.sketchup.com/Sketchup/Pages.html#reorder-instance_method)

#### `show_frame_at(seconds) => Array(Sketchup::Page, Float)?`

Note: In versions prior to SketchUp 2019 this method will crash if called when there are no pages in the model.

**Remarks:** Note: In versions prior to SketchUp 2019 this method will crash if called when there are no pages in the model. The #show_frame_at method is used to show a frame in animation (of the slide show) at a given time in seconds.

**Parameters:**
- `seconds` (Float) — The time in seconds.

**Returns:** `Array(Sketchup::Page, Float), nil` — Upon success it returns the page that is displayed and a parameter between 0.0 and 1.0 that is tells how far along the transition to next page you are at.

[Docs](https://ruby.sketchup.com/Sketchup/Pages.html#show_frame_at-instance_method)

#### `size => Integer`

The #size method is used to retrieve the number of pages.

**Remarks:** The #size method is used to retrieve the number of pages.

**Returns:** `Integer` — 

[Docs](https://ruby.sketchup.com/Sketchup/Pages.html#size-instance_method)

#### `slideshow_time => Object`

The slideshow_time method is used to get the amount of time that a slideshow of all of the pages will take.

**Remarks:** The slideshow_time method is used to get the amount of time that a slideshow of all of the pages will take. This takes into account the transition time for each Page and the amount of time that each Page is displayed.

**Returns:** `Object` — status - true if successful

[Docs](https://ruby.sketchup.com/Sketchup/Pages.html#slideshow_time-instance_method)

#### `unique_name(base_name) => String`

The #unique_name method is used to generate a unique name for the page based on a base_name string.

**Remarks:** The #unique_name method is used to generate a unique name for the page based on a base_name string. For example, a base name of “Joe” might return “Joe 2” if “Joe” already exists.

**Parameters:**
- `base_name` (String) — The base name to use for the unique name.

**Returns:** `String` — The unique name.

[Docs](https://ruby.sketchup.com/Sketchup/Pages.html#unique_name-instance_method)

### Properties
- `selected_page` (Object, get/set) — The selected_page method is used to retrieve the currently selected page.

## PagesObserver (class)

This class is abstract. To implement this observer, create a Ruby class of this type, override the desired methods, and add an instance of the observer to the objects of interests. This observer interface is implemented to react to pages events.

[Vendor docs](https://ruby.sketchup.com/Sketchup/PagesObserver.html)

### Methods
#### `onContentsModified(pages) => nil`

The #onContentsModified method is invoked whenever the pages change.

**Remarks:** The #onContentsModified method is invoked whenever the pages change.

**Parameters:**
- `pages` (Sketchup::Pages)

**Returns:** `nil` — 

[Docs](https://ruby.sketchup.com/Sketchup/PagesObserver.html#onContentsModified-instance_method)

#### `onElementAdded(pages, page) => nil`

The #onElementAdded method is invoked when an element is added to a Sketchup::Pages object.

**Remarks:** The #onElementAdded method is invoked when an element is added to a Sketchup::Pages object.

**Parameters:**
- `pages` (Sketchup::Pages)
- `page` (Sketchup::Page)

**Returns:** `nil` — 

[Docs](https://ruby.sketchup.com/Sketchup/PagesObserver.html#onElementAdded-instance_method)

#### `onElementRemoved(pages, page) => nil`

The #onElementRemoved method is invoked when an element is removed from a Sketchup::Pages object.

**Remarks:** The #onElementRemoved method is invoked when an element is removed from a Sketchup::Pages object.

**Parameters:**
- `pages` (Sketchup::Pages)
- `page` (Sketchup::Page)

**Returns:** `nil` — 

[Docs](https://ruby.sketchup.com/Sketchup/PagesObserver.html#onElementRemoved-instance_method)

## PickHelper (class)

Note: The same Pickhelper instance is being reused by SketchUp. Don't extend, add methods or redefine methods on this object as it can clash with other extensions. The PickHelper class is used to pick entities that reside under the current cursor location, similar to native Select tool. Unlike InputPoint, PickHelper uses no inference. Only Tools react to cursor location and most of these methods are only useful in the context of a tool. For example, if you want to determine the entity you just clicked, you would use #do_pick from within your Tool#onLButtonDown method. You can retrieve a PickHelper object using the View#pick_helper method. Entities that are picked (found under the cursor when a mouse or keyboard event occurs), are called Pick Records and are placed in an indexed list.

[Vendor docs](https://ruby.sketchup.com/Sketchup/PickHelper.html)

### Methods
#### `all_picked => Object`

The all_picked method is used to get an array of entities from the active entities from all the pick paths.

**Remarks:** The all_picked method is used to get an array of entities from the active entities from all the pick paths. Duplicates might occur if there are multiple pick paths for entities that ends in a group or component. For example, if the pick hits at the border of an edge and face inside a group there will be two pick paths - one for the face and one for the edge. Since this method returns entities from the active entities it would return an array with the group two times.

**Returns:** `Object` — elements - the array of elements in the pick.

[Docs](https://ruby.sketchup.com/Sketchup/PickHelper.html#all_picked-instance_method)

#### `best_picked => Object`

The best_picked method is used to retrieve the “best” entity picked (entity that you would have picked if you were using the select tool).

**Remarks:** The best_picked method is used to retrieve the “best” entity picked (entity that you would have picked if you were using the select tool). Returns nil if nothing was picked. You must have called do_pick before calling this method.

**Returns:** `Object` — entity - the best picked entity

[Docs](https://ruby.sketchup.com/Sketchup/PickHelper.html#best_picked-instance_method)

#### `boundingbox_pick(bounding_box, pick_type, transformation = IDENTITY) => Integer`

Used to pick a set of entities from a model from a Geom::BoundingBox.

**Remarks:** Used to pick a set of entities from a model from a Geom::BoundingBox. The pick criteria can be for completely-contained or partially-contained entities, similar to how the Selection tool works.

**Parameters:**
- `bounding_box` (Geom::BoundingBox) — Bounding box defining the volume to use for picking.
- `pick_type` (Integer) — PICK_INSIDE to select entities completely contained or PICK_CROSSING to select entities partially contained.
- `transformation` (Geom::Transformation) — Transformation that will be applied to the volume defined by the bounding_box that allows for a rotation.

**Returns:** `Integer` — The number of entities picked

[Docs](https://ruby.sketchup.com/Sketchup/PickHelper.html#boundingbox_pick-instance_method)

#### `count => Object`

The count method is used to count the number of entities picked (number of pick records)

**Remarks:** The count method is used to count the number of entities picked (number of pick records)

**Returns:** `Object` — number - the number of entities picked

[Docs](https://ruby.sketchup.com/Sketchup/PickHelper.html#count-instance_method)

#### `depth_at(index) => Object`

The depth_at method retrieves the depth of one of the currently picked entities in the list of pick records.

**Remarks:** The depth_at method retrieves the depth of one of the currently picked entities in the list of pick records.

**Parameters:**
- `index` (Object) — A number from 0 to number of items picked minus one.

**Returns:** `Object` — integer - the depth of the entity if successful

[Docs](https://ruby.sketchup.com/Sketchup/PickHelper.html#depth_at-instance_method)

#### `do_pick(x, y, aperture = 0) => Integer | do_pick(x, y, aperture = 0.0) => Integer`

The #do_pick method is used to perform the initial pick.

**Remarks:** The #do_pick method is used to perform the initial pick. This method is generally called before any other methods in the PickHelper class.

**Parameters:**
- `x` (Integer) — Screen coordinate in physical pixels.
- `y` (Integer) — Screen coordinate in physical pixels.
- `aperture` (Integer) — The size of the aperture in physical pixels.

**Returns:** `Integer` — Number of entities picked.

[Docs](https://ruby.sketchup.com/Sketchup/PickHelper.html#do_pick-instance_method)

#### `element_at(index) => Object`

The element_at method is used to retrieve a specific entity in the list of picked elements.

**Remarks:** The element_at method is used to retrieve a specific entity in the list of picked elements. This element will be from the active entities. Use count() to get the number of possible pick paths.

**Parameters:**
- `index` (Object) — A number from 0 to number of items picked minus one.

**Returns:** `Object` — entity - the entity at the index position in the list of picked entities.

[Docs](https://ruby.sketchup.com/Sketchup/PickHelper.html#element_at-instance_method)

#### `init(x, y, aperture = 0) => Sketchup::PickHelper | init(x, y, aperture = 0.0) => Sketchup::PickHelper`

The #init method is used to initialize the PickHelper for testing points.

**Remarks:** The #init method is used to initialize the PickHelper for testing points. You do not normally need to call this method, but you can use this if you want to call #test_point or #pick_segment on a lot of points.

**Parameters:**
- `x` (Integer) — Screen coordinate in physical pixels.
- `y` (Integer) — Screen coordinate in physical pixels.
- `aperture` (Integer) — The size of the aperture in physical pixels. This is the width and height of the square picking aperture.

**Returns:** `Sketchup::PickHelper` — 

[Docs](https://ruby.sketchup.com/Sketchup/PickHelper.html#init-instance_method)

#### `leaf_at(index) => Object`

The leaf_at method retrieves the deepest thing in a pick path.

**Remarks:** The leaf_at method retrieves the deepest thing in a pick path. For example, if you have a face that is within a component that is within another component, leaf_at returns the face. Use count() to get the number of possible pick paths.

**Parameters:**
- `index` (Object) — A number from 0 to number of items picked minus one.

**Returns:** `Object` — entity - the leaf entity

[Docs](https://ruby.sketchup.com/Sketchup/PickHelper.html#leaf_at-instance_method)

#### `path_at(index) => Object`

The path_at method is used to retrieve the entire path for an entity in the pick list (as an array).

**Remarks:** The path_at method is used to retrieve the entire path for an entity in the pick list (as an array). If one of the pick paths end in a face nested in a group nested in a component this method will return an array of these entities. The Group will be first and the face will be last. The first item in the array will be from the active entities and the last item will be a drawing element that is not a group, component or image.

**Parameters:**
- `index` (Object) — A number from 0 to number of items picked minus one.

**Returns:** `Object` — array - an array of entities including the leaf

[Docs](https://ruby.sketchup.com/Sketchup/PickHelper.html#path_at-instance_method)

#### `pick_segment(points) => Integer, false | pick_segment(points, x, y, aperture = 0) => Integer, false`

Note: The return value will be a negative index when a segment is picked.

**Remarks:** Note: The return value will be a negative index when a segment is picked. The #pick_segment method is used to pick a segment of a polyline curve that is defined by an array of points. If you click on a point in a polyline curve, the index of the point in the curve is returned (starting at 0). If you click on a segment in the polyline curve, the index of the segment is returned. Segments are returned by negative indicies and start at index -1 (for the segment connecting the first two points) and increase by a factor of -1 (for example, the segment connecting second and third point is -2). There is no need to invoke #do_pick for this and the results are unrelated.

**Parameters:**
- `points` (Array<Geom::Point3d>) — A series of points in the polyline as a list of parameters or an array containing Point3d objects.

**Returns:** `Integer, false` — an index on success, false on failure

[Docs](https://ruby.sketchup.com/Sketchup/PickHelper.html#pick_segment-instance_method)

#### `picked_edge => Object`

The picked_edge method is used to retrieve the “best” Edge picked.

**Remarks:** The picked_edge method is used to retrieve the “best” Edge picked. Returns nil if there were no edges picked. You must have called do_pick before calling this method.

**Returns:** `Object` — edge - an Edge object if successful

[Docs](https://ruby.sketchup.com/Sketchup/PickHelper.html#picked_edge-instance_method)

#### `picked_element => Sketchup::Drawingelement? | picked_element(index) => Sketchup::Drawingelement?`

The picked_element method retrieves the best drawing element, that is not an edge or a face, picked.

**Remarks:** The picked_element method retrieves the best drawing element, that is not an edge or a face, picked. Returns nil if nothing was picked. You must have called do_pick before calling this method.

**Parameters:**
- `index` (Integer)

**Returns:** `Sketchup::Drawingelement, nil` — a drawing element that is not an edge or face

[Docs](https://ruby.sketchup.com/Sketchup/PickHelper.html#picked_element-instance_method)

#### `picked_face => Object`

The picked_face method is used to retrieve the best face picked.

**Remarks:** The picked_face method is used to retrieve the best face picked. Returns nil if there were no faces picked. You must have called do_pick before calling this method.

**Returns:** `Object` — face - a Face object if successful

[Docs](https://ruby.sketchup.com/Sketchup/PickHelper.html#picked_face-instance_method)

#### `test_point(point) => Boolean | test_point(point, x, y, aperture = 0) => Boolean`

The #test_point method is used to test a point to see if it would be selected using the default or given pick aperture.

**Remarks:** The #test_point method is used to test a point to see if it would be selected using the default or given pick aperture. There is no need to invoke #do_pick for this and the results are unrelated.

**Parameters:**
- `point` (Geom::Point3d) — Model coordinate.

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Sketchup/PickHelper.html#test_point-instance_method)

#### `transformation_at(index) => Object`

The transformation_at method is used to get a transformation at a specific pick path index in the pick helper.

**Remarks:** The transformation_at method is used to get a transformation at a specific pick path index in the pick helper. The transformation combines the transformation of all groups, components and images in the pick path. This transformation can be used to transform the coordinates of the leaf entity into the coordinates of the active entities.

**Parameters:**
- `index` (Object) — The index where the transformation should be retrieved.

**Returns:** `Object` — transformation - the transformation found

[Docs](https://ruby.sketchup.com/Sketchup/PickHelper.html#transformation_at-instance_method)

#### `view => Sketchup::View`

The #view method is used to get the view associated with the Sketchup::PickHelper.

**Remarks:** The #view method is used to get the view associated with the Sketchup::PickHelper.

**Returns:** `Sketchup::View` — 

[Docs](https://ruby.sketchup.com/Sketchup/PickHelper.html#view-instance_method)

#### `window_pick(start_point, end_point, pick_type) => Integer`

Note: Prior to SketchUp 2025.

**Remarks:** Note: Prior to SketchUp 2025.0 this method expected physical screen coordinates. As of SketchUp 2025.0 they are expected to be logical screen coordinates. Used to pick a set of entities from a model based on a screen coordinate rectangular area defined by two points. The pick criteria can be for completely-contained or partially-contained entities, similar to how the Selection tool works. The z value of the points passed in are ignored.

**Parameters:**
- `start_point` (Geom::Point3d) — First screen coordinate point.
- `end_point` (Geom::Point3d) — Second screen coordinate point.
- `pick_type` (Integer) — PICK_INSIDE to select entities completely contained or PICK_CROSSING to select entities partially contained.

**Returns:** `Integer` — The number of Drawingelement objects picked.

[Docs](https://ruby.sketchup.com/Sketchup/PickHelper.html#window_pick-instance_method)

### Properties
- `PICK_INSIDE` (Object, get) — 
- `PICK_CROSSING` (Object, get) — 

## RegionalSettings (interface)

The RegionalSettings module contains methods getting information about the user's locale settings. Note that when you convert between units and strings you don't need to parse it yourself and you can instead use String#to_l and Length#to_s.

[Vendor docs](https://ruby.sketchup.com/Sketchup/RegionalSettings.html)

### Methods
#### `decimal_separator => String`

Returns the decimal character for the current user's locale.

**Remarks:** Returns the decimal character for the current user's locale.

**Returns:** `String` — the decimal separator character

[Docs](https://ruby.sketchup.com/Sketchup/RegionalSettings.html#decimal_separator-class_method)

#### `list_separator => String`

Returns the list separator character for the current user's locale.

**Remarks:** Returns the list separator character for the current user's locale.

**Returns:** `String` — the list separator character

[Docs](https://ruby.sketchup.com/Sketchup/RegionalSettings.html#list_separator-class_method)

## RenderingOptions (class)

The RenderingOptions class contains method to extract the rendering information for a model. The majority of the rendering information returned exists in the Styles dialog. The following rendering information keys are maintained in SketchUp: BackgroundColor BandColor ConstructionColor DepthQueWidth DisplayColorByLayer DisplayFog DisplayInstanceAxes DisplayWatermarks DrawDepthQue DrawGround DrawHidden DrawHorizon DrawLineEnds DrawProfilesOnly DrawSilhouettes DrawUnderground EdgeColorMode EdgeDisplayMode EdgeType Accepted values (Integer): 0: Standard edges 1: Sketchy edges ExtendLines FaceBackColor FaceFrontColor FogColor FogEndDist FogStartDist FogUseBkColor ForegroundColor GroundColor GroundTransparency HideConstructionGeometry HighlightColor HorizonColor InactiveHidden InstanceHidden JitterEdges LineEndWidth LineExtension LockedColor MaterialTransparency ModelTransparency RenderMode SectionActiveColor SectionCutWidth SectionDefaultCutColor SectionInactiveColor ShowViewName SilhouetteWidth SkyColor Texture TransparencySort SketchUp 2017 treats Medium transparency as Faster. Added in SketchUp 7: DisplayDims DisplaySketchAxes DisplayText Added in SketchUp 8: InactiveFade InstanceFade Added in SketchUp 2014: DisplaySectionPlanes Added in SketchUp 2015: DisplaySectionCuts DrawBackEdges SectionCutDrawEdges Added in SketchUp 2018: SectionCutFilled SectionDefaultFillColor Removed in SketchUp 2019.1 FaceColorMode This option was previously included but it was ineffective. Added in SketchUp 2020.0: ROPDrawHiddenGeometry ROPDrawHiddenObjects Added in SketchUp 2024.0: AmbientOcclusion AmbientOcclusionDistance AmbientOcclusionIntensity Added in SketchUp 2026.0: AmbientOcclusionColor AmbientOcclusionMultiplier

[Vendor docs](https://ruby.sketchup.com/Sketchup/RenderingOptions.html)

### Methods
#### `add_observer(observer) => Boolean`

The add_observer method is used to add an observer to the current object.

**Remarks:** The add_observer method is used to add an observer to the current object.

**Parameters:**
- `observer` (Object) — An observer.

**Returns:** `Boolean` — true if successful, false if unsuccessful.

[Docs](https://ruby.sketchup.com/Sketchup/RenderingOptions.html#add_observer-instance_method)

#### `count => Integer`

The #count method is inherited from the Enumerable mix-in module.

**Remarks:** The #count method is inherited from the Enumerable mix-in module.

**Returns:** `Integer` — 

[Docs](https://ruby.sketchup.com/Sketchup/RenderingOptions.html#count-instance_method)

#### `each {|key, value| ... } => nil`

The #each method iterates through all of the rendering options key/value pairs.

**Remarks:** The #each method iterates through all of the rendering options key/value pairs.

**Returns:** `nil` — 

[Docs](https://ruby.sketchup.com/Sketchup/RenderingOptions.html#each-instance_method)

#### `each_key {|key| ... } => nil`

The #each_key method iterates through all of the rendering options keys.

**Remarks:** The #each_key method iterates through all of the rendering options keys.

**Returns:** `nil` — 

[Docs](https://ruby.sketchup.com/Sketchup/RenderingOptions.html#each_key-class_method)

#### `each_key {|key| ... } => nil`

The #each_key method iterates through all of the rendering options keys.

**Remarks:** The #each_key method iterates through all of the rendering options keys.

**Returns:** `nil` — 

[Docs](https://ruby.sketchup.com/Sketchup/RenderingOptions.html#each_key-instance_method)

#### `each_pair => nil`

The #each_pair method is an alias for #each.

**Remarks:** The #each_pair method is an alias for #each.

**Returns:** `nil` — 

[Docs](https://ruby.sketchup.com/Sketchup/RenderingOptions.html#each_pair-instance_method)

#### `keys => Array<String>`

The keys method returns an array with all of the attribute keys.

**Remarks:** The keys method returns an array with all of the attribute keys.

**Returns:** `Array<String>` — an array of keys

[Docs](https://ruby.sketchup.com/Sketchup/RenderingOptions.html#keys-class_method)

#### `keys => Array<String>`

The keys method returns an array with all of the attribute keys.

**Remarks:** The keys method returns an array with all of the attribute keys.

**Returns:** `Array<String>` — an array of keys

[Docs](https://ruby.sketchup.com/Sketchup/RenderingOptions.html#keys-instance_method)

#### `length => Integer`

The #length method returns the number of options in the rendering options collection.

**Remarks:** The #length method returns the number of options in the rendering options collection.

**Returns:** `Integer` — 

[Docs](https://ruby.sketchup.com/Sketchup/RenderingOptions.html#length-instance_method)

#### `remove_observer(observer) => Boolean`

The remove_observer method is used to remove an observer from the current object.

**Remarks:** The remove_observer method is used to remove an observer from the current object.

**Parameters:**
- `observer` (Object) — An observer.

**Returns:** `Boolean` — true if successful, false if unsuccessful.

[Docs](https://ruby.sketchup.com/Sketchup/RenderingOptions.html#remove_observer-instance_method)

#### `size => Integer`

The #size method is an alias for #length.

**Remarks:** The #size method is an alias for #length.

**Returns:** `Integer` — 

[Docs](https://ruby.sketchup.com/Sketchup/RenderingOptions.html#size-instance_method)

### Properties
- `[]` (Object, nil, get/set) — The #[] method is used to get the value of a rendering option.
- `ROPAssign` (Object, get) — 
- `ROPDrawHidden` (Object, get) — 
- `ROPDrawHiddenGeometry` (Object, get) — 
- `ROPDrawHiddenObjects` (Object, get) — 
- `ROPEditComponent` (Object, get) — 
- `ROPSectionDisplayTurnedOff` (Object, get) — 
- `ROPSetBackgroundColor` (Object, get) — 
- `ROPSetConstructionColor` (Object, get) — 
- `ROPSetDepthQueEdges` (Object, get) — 
- `ROPSetDepthQueWidth` (Object, get) — 
- `ROPSetDisplayColorByLayer` (Object, get) — 
- `ROPSetDisplayDims` (Object, get) — 
- `ROPSetDisplayFog` (Object, get) — 
- `ROPSetDisplayInstanceAxes` (Object, get) — 
- `ROPSetDisplaySketchAxes` (Object, get) — 
- `ROPSetDisplayText` (Object, get) — 
- `ROPSetDisplayWatermarks` (Object, get) — 
- `ROPSetDrawBackEdges` (Object, get) — 
- `ROPSetDrawGround` (Object, get) — 
- `ROPSetDrawHorizon` (Object, get) — 
- `ROPSetDrawUnderground` (Object, get) — 
- `ROPSetEdgeColorMode` (Object, get) — 
- `ROPSetEdgeDisplayMode` (Object, get) — 
- `ROPSetEdgeType` (Object, get) — 
- `ROPSetExtendEdges` (Object, get) — 
- `ROPSetExtendLines` (Object, get) — 
- `ROPSetFaceColor` (Object, get) — 
- `ROPSetFogColor` (Object, get) — 
- `ROPSetFogDist` (Object, get) — 
- `ROPSetFogHint` (Object, get) — 
- `ROPSetFogUseBkColor` (Object, get) — 
- `ROPSetForegroundColor` (Object, get) — 
- `ROPSetGroundColor` (Object, get) — 
- `ROPSetGroundTransparency` (Object, get) — 
- `ROPSetHorizonColor` (Object, get) — 
- `ROPSetHideConstructionGeometry` (Object, get) — 
- `ROPSetHighlightColor` (Object, get) — 
- `ROPSetJitterEdges` (Object, get) — 
- `ROPSetLineEndEdges` (Object, get) — 
- `ROPSetLineEndWidth` (Object, get) — 
- `ROPSetLineExtension` (Object, get) — 
- `ROPSetLineStyleEdges` (Object, get) — 
- `ROPSetLockedColor` (Object, get) — 
- `ROPSetMaterialTransparency` (Object, get) — 
- `ROPSetModelTransparency` (Object, get) — 
- `ROPSetPhotomatchBackgroundOpacity` (Object, get) — 
- `ROPSetPhotomatchDrawBackground` (Object, get) — 
- `ROPSetPhotomatchDrawOverlay` (Object, get) — 
- `ROPSetPhotomatchOverlayOpacity` (Object, get) — 
- `ROPSetProfileEdges` (Object, get) — 
- `ROPSetProfilesOnlyEdges` (Object, get) — 
- `ROPSetProfileWidth` (Object, get) — 
- `ROPSetRenderMode` (Object, get) — 
- `ROPSetSectionActiveColor` (Object, get) — 
- `ROPSetSectionCutFilled` (Object, get) — 
- `ROPSetSectionCutWidth` (Object, get) — 
- `ROPSetSectionDefaultCutColor` (Object, get) — 
- `ROPSetSectionDefaultFillColor` (Object, get) — 
- `ROPSetSectionDisplayMode` (Object, get) — 
- `ROPSetSectionInactiveColor` (Object, get) — 
- `ROPSetSkyColor` (Object, get) — 
- `ROPSetTexture` (Object, get) — 
- `ROPSetTransparencyObsolete` (Object, get) — 
- `ROPSetXRayOpacity` (Object, get) — 
- `ROPTransparencySortMethod` (Object, get) — 
- `ROPSetAOEnabled` (Object, get) — 
- `ROPSetAODistance` (Object, get) — 
- `ROPSetAOIntensity` (Object, get) — 
- `ROPSetModelingGrid` (Object, get) — 
- `ROPSetHideSpaces` (Object, get) — 
- `ROPSetAOColorEnabled` (Object, get) — 
- `ROPSetAOColor` (Object, get) — 
- `ROPSetAOMultiplier` (Object, get) — 
- `ROPSetFaceColorMode` (Object, get) — 

## RenderingOptionsObserver (class)

This class is abstract. To implement this observer, create a Ruby class of this type, implement the desired methods, and add an instance of the observer to the objects of interests. This observer interface is implemented to react to rendering options events.

[Vendor docs](https://ruby.sketchup.com/Sketchup/RenderingOptionsObserver.html)

### Methods
#### `onRenderingOptionsChanged(rendering_options, type) => Object`

The onRenderingOptionsChanged method is invoked whenever the user changes their rendering options.

**Remarks:** The onRenderingOptionsChanged method is invoked whenever the user changes their rendering options.

**Parameters:**
- `rendering_options` (Sketchup::RenderingOptions)
- `type` (Integer) — A number indicating which option was changed represented by one of the constants defined in Sketchup::RenderingOptions.

[Docs](https://ruby.sketchup.com/Sketchup/RenderingOptionsObserver.html#onRenderingOptionsChanged-instance_method)

## SectionPlane (class)

The SectionPlane class represents a section plane in a SketchUp model. Note that prior to SketchUp 2014 there was no way to create a SectionPlane object using Ruby. For older versions of SketchUp, you must manually create a section plane with the Section Plane Tool in SketchUp and then query the entities array to find the SectionPlane object.

[Vendor docs](https://ruby.sketchup.com/Sketchup/SectionPlane.html)

### Methods
#### `activate => Object`

The activate method is used to make the section plane the active one of its parent component/group.

**Remarks:** The activate method is used to make the section plane the active one of its parent component/group.

**Returns:** `Object` — self if successful

[Docs](https://ruby.sketchup.com/Sketchup/SectionPlane.html#activate-instance_method)

#### `active? => Boolean`

The active? method indicates whether the section plane is active or not.

**Remarks:** The active? method indicates whether the section plane is active or not.

**Returns:** `Boolean` — boolean - true if active

[Docs](https://ruby.sketchup.com/Sketchup/SectionPlane.html#active?-instance_method)

#### `get_plane => Object`

The get_plane method is used to retrieve the plane that the section plane is on.

**Remarks:** The get_plane method is used to retrieve the plane that the section plane is on.

**Returns:** `Object` — plane - a plane. See the Geom module and Array class for further information on planes.

[Docs](https://ruby.sketchup.com/Sketchup/SectionPlane.html#get_plane-instance_method)

#### `set_plane(plane) => Object`

The set_plane method is used to set the plane that the section plane is on.

**Remarks:** The set_plane method is used to set the plane that the section plane is on.

**Parameters:**
- `plane` (Object) — An array representing the new plane,

**Returns:** `Object` — section_plane - the updated SectionPlane.

[Docs](https://ruby.sketchup.com/Sketchup/SectionPlane.html#set_plane-instance_method)

### Properties
- `name` (String, get/set) — The #name method is used to retrieve the name of the section plane.
- `symbol` (String, get/set) — The #symbol method is used to retrieve the symbol of the section plane.

## Selection (class)

A set of the currently selected drawing elements. Use the Model.selection method to get a Selection object. Note that the order of drawing elements (selection[0], selection[1] and so on) in the set is in no particular order and should not be assumed to be in the same order as the user selected the drawing elements.

[Vendor docs](https://ruby.sketchup.com/Sketchup/Selection.html)

### Methods
#### `[](index) => Sketchup::Drawingelement?`

The #[] method is used to retrieve a Drawingelement from the selection by index.

**Remarks:** The #[] method is used to retrieve a Drawingelement from the selection by index. Index 0 is the first Drawingelement in the selection. This method is not very efficient. If you need to look at every entity in the selection, consider using #each instead of using this method to manually grab each one.

**Parameters:**
- `index` (Integer) — The index of the Drawingelement object to retrieve.

**Returns:** `Sketchup::Drawingelement, nil` — 

[Docs](https://ruby.sketchup.com/Sketchup/Selection.html#[]-instance_method)

#### `add(drawing_elements) => Integer | add(*drawing_elements) => Integer`

The #add method is used to add Drawingelement to the selection.

**Remarks:** The #add method is used to add Drawingelement to the selection. Drawingelements that are added to the Selection are visually indicated by the selection bounding box. You can pass it individual Drawingelements or an Array of Drawingelements.

**Parameters:**
- `drawing_elements` (Array<Sketchup::Drawingelement>)

**Returns:** `Integer` — the number of Drawingelement objects added

[Docs](https://ruby.sketchup.com/Sketchup/Selection.html#add-instance_method)

#### `add_observer(observer) => Boolean`

The add_observer method is used to add an observer to the selection object.

**Remarks:** The add_observer method is used to add an observer to the selection object.

**Parameters:**
- `observer` (Object) — An observer.

**Returns:** `Boolean` — true if successful, false if unsuccessful.

[Docs](https://ruby.sketchup.com/Sketchup/Selection.html#add_observer-instance_method)

#### `at(index) => Sketchup::Drawingelement?`

The #at method is an alias for #[].

**Remarks:** The #at method is an alias for #[].

**Parameters:**
- `index` (Integer) — The index of the Drawingelement object to retrieve.

**Returns:** `Sketchup::Drawingelement, nil` — 

[Docs](https://ruby.sketchup.com/Sketchup/Selection.html#at-instance_method)

#### `clear => nil`

The clear method is used to clear the selection.

**Remarks:** The clear method is used to clear the selection.

**Returns:** `nil` — 

[Docs](https://ruby.sketchup.com/Sketchup/Selection.html#clear-instance_method)

#### `contains?(drawing_element) => Boolean`

The #contains? method is and alias of #include?.

**Remarks:** The #contains? method is and alias of #include?.

**Parameters:**
- `drawing_element` (Sketchup::Drawingelement)

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Sketchup/Selection.html#contains?-instance_method)

#### `count => Integer`

Note: Since SketchUp 2014 the count method is inherited from Ruby's Enumerable mix-in module.

**Remarks:** Note: Since SketchUp 2014 the count method is inherited from Ruby's Enumerable mix-in module. Prior to that the #count method is an alias for #length.

**Returns:** `Integer` — 

[Docs](https://ruby.sketchup.com/Sketchup/Selection.html#count-instance_method)

#### `each {|drawing_element| ... } => nil`

Note: Don't remove content from this collection while iterating over it with #each.

**Remarks:** Note: Don't remove content from this collection while iterating over it with #each. This would change the size of the collection and cause elemnts to be skipped as the indices change. Instead copy the current collection to an array using to_a and then use each on the array, when removing content. The #each method is used to iterate through all of the selected Drawingelements. If you want to do something with all of the selected Drawingelements, this is more efficient than using #[].

**Returns:** `nil` — 

[Docs](https://ruby.sketchup.com/Sketchup/Selection.html#each-instance_method)

#### `empty? => Boolean`

The #empty? method is used to determine if there are drawing elements in the selection.

**Remarks:** The #empty? method is used to determine if there are drawing elements in the selection.

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Sketchup/Selection.html#empty?-instance_method)

#### `first => Sketchup::Drawingelement`

The #first method is used to retrieve the first selected Drawingelement Returns nil if nothing is selected.

**Remarks:** The #first method is used to retrieve the first selected Drawingelement Returns nil if nothing is selected. This method is useful when you know that only a single Drawingelement is selected, or you are only interested in the first selected Drawingelement.

**Returns:** `Sketchup::Drawingelement` — the first selected Drawingelement object if successful

[Docs](https://ruby.sketchup.com/Sketchup/Selection.html#first-instance_method)

#### `include?(drawing_element) => Boolean`

The #include? method is used to determine if a given Drawingelement is in the selection.

**Remarks:** The #include? method is used to determine if a given Drawingelement is in the selection.

**Parameters:**
- `drawing_element` (Sketchup::Drawingelement)

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Sketchup/Selection.html#include?-instance_method)

#### `invert => nil`

The #invert method is used to invert the selection.

**Remarks:** The #invert method is used to invert the selection.

**Returns:** `nil` — 

[Docs](https://ruby.sketchup.com/Sketchup/Selection.html#invert-instance_method)

#### `is_curve? => Boolean`

The #is_curve? method is used to determine if the selection contains all edges that belong to a single curve.

**Remarks:** The #is_curve? method is used to determine if the selection contains all edges that belong to a single curve.

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Sketchup/Selection.html#is_curve?-instance_method)

#### `is_surface? => Boolean`

The #is_surface? method is used to determine if the selection contains only all of the faces that are part of a single curved surface.

**Remarks:** The #is_surface? method is used to determine if the selection contains only all of the faces that are part of a single curved surface.

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Sketchup/Selection.html#is_surface?-instance_method)

#### `length => Integer`

The #length method is used to retrieve the number of selected drawing elements.

**Remarks:** The #length method is used to retrieve the number of selected drawing elements.

**Returns:** `Integer` — 

[Docs](https://ruby.sketchup.com/Sketchup/Selection.html#length-instance_method)

#### `model => Sketchup::Model`

The #model method retrieves the model for the selection.

**Remarks:** The #model method retrieves the model for the selection.

**Returns:** `Sketchup::Model` — the model that includes the selection if successful

[Docs](https://ruby.sketchup.com/Sketchup/Selection.html#model-instance_method)

#### `nitems => Integer`

The #nitems method is an alias for #length.

**Remarks:** The #nitems method is an alias for #length.

**Returns:** `Integer` — 

[Docs](https://ruby.sketchup.com/Sketchup/Selection.html#nitems-instance_method)

#### `remove(drawing_elements) => Integer | remove(*drawing_elements) => Integer`

The #remove method is used to remove Drawingelements from the selection.

**Remarks:** The #remove method is used to remove Drawingelements from the selection. You can pass it individual Drawingelements or an Array of Drawingelements.

**Parameters:**
- `drawing_elements` (Array<Sketchup::Drawingelement>)

**Returns:** `Integer` — the number of Drawingelement objects removed

[Docs](https://ruby.sketchup.com/Sketchup/Selection.html#remove-instance_method)

#### `remove_observer(observer) => Boolean`

The remove_observer method is used to remove an observer from the selection object.

**Remarks:** The remove_observer method is used to remove an observer from the selection object.

**Parameters:**
- `observer` (Object) — An observer.

**Returns:** `Boolean` — true if successful, false if unsuccessful.

[Docs](https://ruby.sketchup.com/Sketchup/Selection.html#remove_observer-instance_method)

#### `shift => Sketchup::Drawingelement`

The #shift method is used to remove the first Drawingelement from the selection and returns it.

**Remarks:** The #shift method is used to remove the first Drawingelement from the selection and returns it.

**Returns:** `Sketchup::Drawingelement` — the first Drawingelement object in the selection set if successful

[Docs](https://ruby.sketchup.com/Sketchup/Selection.html#shift-instance_method)

#### `single_object? => Boolean`

The #single_object? method is used to determine if the selection contains a single object.

**Remarks:** The #single_object? method is used to determine if the selection contains a single object. It can either be a single DrawingElement or a group of DrawingElements for which is_curve? or is_surface? will return true.

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Sketchup/Selection.html#single_object?-instance_method)

#### `size => Integer`

The #size method is an alias for #length.

**Remarks:** The #size method is an alias for #length.

**Returns:** `Integer` — 

[Docs](https://ruby.sketchup.com/Sketchup/Selection.html#size-instance_method)

#### `toggle(drawings_elements) => Integer | toggle(*drawing_elements) => Integer`

The #toggle method is used to change whether a Drawingelement is part of the selection.

**Remarks:** The #toggle method is used to change whether a Drawingelement is part of the selection. Drawingelements that are not already selected are added. Drawingelements that are already selected are removed. You can pass it individual Drawingelements or an Array of Drawingelements.

**Parameters:**
- `drawing_elements` (Array<Sketchup::Drawingelement>)

**Returns:** `Integer` — the number of Drawingelement objects changed

[Docs](https://ruby.sketchup.com/Sketchup/Selection.html#toggle-instance_method)

## SelectionObserver (class)

This class is abstract. To implement this observer, create a Ruby class of this type, override the desired methods, and add an instance of the observer to the objects of interests. This observer interface is implemented to react to selection events.

[Vendor docs](https://ruby.sketchup.com/Sketchup/SelectionObserver.html)

### Methods
#### `onSelectionAdded(selection, entity) => nil`

Note: This event might not trigger even if a single element is selected.

**Remarks:** Note: This event might not trigger even if a single element is selected. For instance the Selection tool will trigger #onSelectionBulkChange regardless.

**Parameters:**
- `selection` (Sketchup::Selection)
- `entity` (Sketchup::Entity)

**Returns:** `nil` — 

[Docs](https://ruby.sketchup.com/Sketchup/SelectionObserver.html#onSelectionAdded-instance_method)

#### `onSelectionBulkChange(selection) => nil`

The #onSelectionBulkChange method is called whenever items are added or removed from the selection set.

**Remarks:** The #onSelectionBulkChange method is called whenever items are added or removed from the selection set. The #onSelectionBulkChange callback will not trigger if the selection is cleared by clicking on empty model space. Use the #onSelectionCleared method to catch this case.

**Parameters:**
- `selection` (Sketchup::Selection)

**Returns:** `nil` — 

[Docs](https://ruby.sketchup.com/Sketchup/SelectionObserver.html#onSelectionBulkChange-instance_method)

#### `onSelectionCleared(selection) => nil`

The #onSelectionCleared method is called when the selection is completely emptied.

**Remarks:** The #onSelectionCleared method is called when the selection is completely emptied.

**Parameters:**
- `selection` (Sketchup::Selection)

**Returns:** `nil` — 

[Docs](https://ruby.sketchup.com/Sketchup/SelectionObserver.html#onSelectionCleared-instance_method)

#### `onSelectionRemoved(selection, entity) => nil`

Note: This event might not trigger even if a single element is deselected.

**Remarks:** Note: This event might not trigger even if a single element is deselected. For instance the Selection tool will trigger #onSelectionBulkChange regardless.

**Parameters:**
- `selection` (Sketchup::Selection)
- `entity` (Sketchup::Entity)

**Returns:** `nil` — 

[Docs](https://ruby.sketchup.com/Sketchup/SelectionObserver.html#onSelectionRemoved-instance_method)

## Set (class)

Deprecated. In SketchUp 2014 this class was changed from Set to Sketchup::Set in order to avoid conflict with the Ruby Standard Library. The Sketchup::Set class is deprecated and new extensions should make use of Ruby's Set class unless they need backward compatibility. The set class represents a collection of unique objects. This class is useful for keeping track of a group of related entities, kind of like a selection set that stays around for as long as you need it to. To make a set of your own, create an empty one using Sketchup::Set.new, and then add items to it.

[Vendor docs](https://ruby.sketchup.com/Sketchup/Set.html)

### Methods
#### `clear => Object`

The clear method is used to clear all objects out of the set.

**Remarks:** The clear method is used to clear all objects out of the set.

**Returns:** `Object` — set - an empty Set object

[Docs](https://ruby.sketchup.com/Sketchup/Set.html#clear-instance_method)

#### `contains?(entity) => Boolean`

The #contains? method is an alias for #include?.

**Remarks:** The #contains? method is an alias for #include?.

**Parameters:**
- `entity` (Sketchup::Entity)

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Sketchup/Set.html#contains?-instance_method)

#### `delete(object) => Object`

The delete object is used to delete or remove an object from the set.

**Remarks:** The delete object is used to delete or remove an object from the set.

**Parameters:**
- `object` (Object) — The object to be deleted.

**Returns:** `Object` — object - the object that was deleted.

[Docs](https://ruby.sketchup.com/Sketchup/Set.html#delete-instance_method)

#### `each {|item| ... } => Object`

The each method is used to iterate through all of the objects in the set.

**Remarks:** The each method is used to iterate through all of the objects in the set.

[Docs](https://ruby.sketchup.com/Sketchup/Set.html#each-instance_method)

#### `empty? => Boolean`

The empty? method is used to determine whether the set is empty.

**Remarks:** The empty? method is used to determine whether the set is empty.

**Returns:** `Boolean` — status - true if the set is empty, false if it is not empty.

[Docs](https://ruby.sketchup.com/Sketchup/Set.html#empty?-instance_method)

#### `include?(entity) => Boolean`

The #include? method is used to determine if the set includes a particular object.

**Remarks:** The #include? method is used to determine if the set includes a particular object.

**Parameters:**
- `entity` (Sketchup::Entity)

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Sketchup/Set.html#include?-instance_method)

#### `insert(object) => Object`

The insert method is used to insert an object into the set.

**Remarks:** The insert method is used to insert an object into the set.

**Parameters:**
- `object` (Object) — The object to be inserted into the set.

**Returns:** `Object` — size - the number of objects in the set

[Docs](https://ruby.sketchup.com/Sketchup/Set.html#insert-instance_method)

#### `length => Integer`

The #length method is an alias for #size.

**Remarks:** The #length method is an alias for #size.

**Returns:** `Integer` — 

[Docs](https://ruby.sketchup.com/Sketchup/Set.html#length-instance_method)

#### `size => Integer`

The #size method is used to determine the number of objects in the set.

**Remarks:** The #size method is used to determine the number of objects in the set.

**Returns:** `Integer` — 

[Docs](https://ruby.sketchup.com/Sketchup/Set.html#size-instance_method)

#### `to_a => Object`

The to_a method is used to get an Array of the entities in your Set.

**Remarks:** The to_a method is used to get an Array of the entities in your Set.

**Returns:** `Object` — array - The Array of the entities in the Set.

[Docs](https://ruby.sketchup.com/Sketchup/Set.html#to_a-instance_method)

## ShadowInfo (class)

The ShadowInfo class contains method to extract the shadow information for a model. The majority of the shadow information returned exists in the Model Info > Location and Model Info > Shadows dialogs inside SketchUp. The following shadow information keys are maintained in SketchUp: City (in Model Info > Geo-location > Set Manual Location…) Note that 'City' is called 'Location' in the UI Country (in Model Info > Geo-location > Set Manual Location…) Dark (in Window > Shadows) DayOfYear DaylightSavings DisplayNorth DisplayOnAllFaces (in Window > Shadows) DisplayOnGroundPlane (in Window > Shadows) DisplayShadows (in Window > Shadows) EdgesCastShadows (in Window > Shadows) Latitude (in Model Info > Geo-location > Set Manual Location…) Light (in Window > Shadows) Longitude (in Model Info > Geo-location > Set Manual Location…) NorthAngle ShadowTime (in Window > Shadows) ShadowTime_time_t (ShadowTime in Epoch time) SunDirection (Generated based on ShadowTime) SunRise (Generated based on ShadowTime) SunRise_time_t (SunRise in Epoch time) SunSet (Generated based on ShadowTime) SunSet_time_t (SunSet in Epoch time) TZOffset (in Window > Shadows) UseSunForAllShading (in Window > Shadows) You access the ShadowInfo object by calling Model#shadow_info:

[Vendor docs](https://ruby.sketchup.com/Sketchup/ShadowInfo.html)

### Methods
#### `add_observer(observer) => Boolean`

The add_observer method is used to add an observer to the current object.

**Remarks:** The add_observer method is used to add an observer to the current object.

**Parameters:**
- `observer` (Object) — An observer.

**Returns:** `Boolean` — true if successful, false if unsuccessful.

[Docs](https://ruby.sketchup.com/Sketchup/ShadowInfo.html#add_observer-instance_method)

#### `count => Integer`

The count method is inherited from the Enumerable mix-in module.

**Remarks:** The count method is inherited from the Enumerable mix-in module.

**Returns:** `Integer` — 

[Docs](https://ruby.sketchup.com/Sketchup/ShadowInfo.html#count-instance_method)

#### `each {|key, value| ... } => nil`

The #each method iterates through all of the shadow information key/value pairs.

**Remarks:** The #each method iterates through all of the shadow information key/value pairs.

**Returns:** `nil` — 

[Docs](https://ruby.sketchup.com/Sketchup/ShadowInfo.html#each-instance_method)

#### `each_key {|key| ... } => nil`

The #each_key method iterates through all of the shadow information keys.

**Remarks:** The #each_key method iterates through all of the shadow information keys.

**Returns:** `nil` — 

[Docs](https://ruby.sketchup.com/Sketchup/ShadowInfo.html#each_key-class_method)

#### `each_key {|key| ... } => nil`

The #each_key method iterates through all of the shadow information keys.

**Remarks:** The #each_key method iterates through all of the shadow information keys.

**Returns:** `nil` — 

[Docs](https://ruby.sketchup.com/Sketchup/ShadowInfo.html#each_key-instance_method)

#### `each_pair {|key, value| ... } => nil`

The #each_pair method is an alias for #each.

**Remarks:** The #each_pair method is an alias for #each.

**Returns:** `nil` — 

[Docs](https://ruby.sketchup.com/Sketchup/ShadowInfo.html#each_pair-instance_method)

#### `keys => Array<String>`

The keys method is a class method that returns an array with all of the attribute keys

**Remarks:** The keys method is a class method that returns an array with all of the attribute keys

**Returns:** `Array<String>` — an array of keys

[Docs](https://ruby.sketchup.com/Sketchup/ShadowInfo.html#keys-class_method)

#### `keys => Array<String>`

The keys method is a class method that returns an array with all of the attribute keys

**Remarks:** The keys method is a class method that returns an array with all of the attribute keys

**Returns:** `Array<String>` — an array of keys

[Docs](https://ruby.sketchup.com/Sketchup/ShadowInfo.html#keys-instance_method)

#### `length => Integer`

The #length method returns the number of options in the shadow options collection

**Remarks:** The #length method returns the number of options in the shadow options collection

**Returns:** `Integer` — 

[Docs](https://ruby.sketchup.com/Sketchup/ShadowInfo.html#length-instance_method)

#### `remove_observer(observer) => Boolean`

The remove_observer method is used to remove an observer from the current object.

**Remarks:** The remove_observer method is used to remove an observer from the current object.

**Parameters:**
- `observer` (Object) — An observer.

**Returns:** `Boolean` — true if successful, false if unsuccessful.

[Docs](https://ruby.sketchup.com/Sketchup/ShadowInfo.html#remove_observer-instance_method)

#### `size => Integer`

The #size method is an alias for #length.

**Remarks:** The #size method is an alias for #length.

**Returns:** `Integer` — 

[Docs](https://ruby.sketchup.com/Sketchup/ShadowInfo.html#size-instance_method)

### Properties
- `[]` (Object, nil, get/set) — The [] method retrieves a value from the array of keys

## ShadowInfoObserver (class)

This class is abstract. To implement this observer, create a Ruby class of this type, override the desired methods, and add an instance of the observer to the ShadowInfo object. This observer interface is implemented to react to changes to the shadow settings.

[Vendor docs](https://ruby.sketchup.com/Sketchup/ShadowInfoObserver.html)

### Methods
#### `onShadowInfoChanged(shadow_info, type) => nil`

The #onShadowInfoChanged method is invoked whenever the user alters a setting inside the Shadows and Model Info dialogs.

**Remarks:** The #onShadowInfoChanged method is invoked whenever the user alters a setting inside the Shadows and Model Info dialogs. The type parameter contains a number identifying which option was altered. Here are the types to expect: 0 = Time/Date sliders 1 = Display Shadows checkbox 2 = Light/Dark sliders 3 = Geographic Location (in Model Info > Location) 4 = Solar Orientation (in Model Info > Location) 7 = Use Sun for Shading checkbox 8 = Display from Edges checkbox 9 = Display on Ground checkbox 10 = Display on Faces checkbox

**Parameters:**
- `shadow_info` (Sketchup::ShadowInfo)
- `type` (Integer) — A number identifying which setting was changed.

**Returns:** `nil` — 

[Docs](https://ruby.sketchup.com/Sketchup/ShadowInfoObserver.html#onShadowInfoChanged-instance_method)

## Skp (interface)

The Skp module is used to read metadata from external SketchUp files without loading the whole file.

[Vendor docs](https://ruby.sketchup.com/Sketchup/Skp.html)

### Methods
#### `read_guid(filepath) => String`

The read_guid method is used to read the GUID, globally unique identifier, for an external model.

**Remarks:** The read_guid method is used to read the GUID, globally unique identifier, for an external model. In SketchUp, GUIDs are used to test if ComponentDefinitions and Models match (a component being an embedded model). When you insert a component to a model, its GUID is compared to existing component definitions in that model, and if there is a match the existing component definition is re-used rather than a duplicate being added. When a component definition is modified or a model is saved, their GUIDs are renewed.

**Parameters:**
- `filepath` (String)

**Returns:** `String` — 

[Docs](https://ruby.sketchup.com/Sketchup/Skp.html#read_guid-class_method)

## Snap (class)

A Snap is a custom grip used by SketchUp's Move tool. Snaps can be added at strategic places such as connectors to help assembling objects. #direction is the direction a snap is “pointing”. This can be thought of as the normal direction of the snap. It can also be thought of as the direction you move an object when plugging it into another object, e.g. inserting a power coord. #up controls the rotation around the Snap's axis. When two objects are snapped together, the Snaps have opposite #direction vectors but matching #up vectors.

[Vendor docs](https://ruby.sketchup.com/Sketchup/Snap.html)

### Methods
#### `direction => Geom::Vector3d`

The #direction method is used to get the direction this Snap is “pointing”.

**Remarks:** The #direction method is used to get the direction this Snap is “pointing”. When two Snaps are snapped into each other, they have the opposite #direction.

**Returns:** `Geom::Vector3d` — 

[Docs](https://ruby.sketchup.com/Sketchup/Snap.html#direction-instance_method)

#### `position => Geom::Point3d`

The #position method is used to get the position of this Snap.

**Remarks:** The #position method is used to get the position of this Snap.

**Returns:** `Geom::Point3d` — 

[Docs](https://ruby.sketchup.com/Sketchup/Snap.html#position-instance_method)

#### `set(position) => Sketchup::Snap | set(position, direction) => Sketchup::Snap | set(position, direction, up) => Sketchup::Snap`

The #set method is used to move and/or reorient a Snap.

**Remarks:** The #set method is used to move and/or reorient a Snap.

**Parameters:**
- `position` (Geom::Point3d)

**Returns:** `Sketchup::Snap` — self

[Docs](https://ruby.sketchup.com/Sketchup/Snap.html#set-instance_method)

#### `up => Geom::Vector3d`

The #up method is used to get a vector representing the rotation of this Snap along its axis.

**Remarks:** The #up method is used to get a vector representing the rotation of this Snap along its axis. When two Snaps are snapped into each other, they have the same aligned #up direction.

**Returns:** `Geom::Vector3d` — 

[Docs](https://ruby.sketchup.com/Sketchup/Snap.html#up-instance_method)

## Style (class)

The Style class contains methods for modifying information about a specific style. Styles are a collection of display settings that tell SketchUp how to draw the model. In SketchUp, there are two important style objects in a model: The Sketchup::Styles#selected_style is the style currently selected in the Styles Browser. The Sketchup::Styles#active_style is a temporary copy of the selected style that allows editing without committing changes. Changes to the active style are not saved unless you call Sketchup::Styles#update_selected_style.

[Vendor docs](https://ruby.sketchup.com/Sketchup/Style.html)

### Methods
#### `path => String`

The #path method gets the file path the Sketchup::Style was loaded from.

**Remarks:** The #path method gets the file path the Sketchup::Style was loaded from.

**Returns:** `String` — path The file path the style was loaded from.

[Docs](https://ruby.sketchup.com/Sketchup/Style.html#path-instance_method)

### Properties
- `description` (String, get/set) — The #description method gets the description for a Sketchup::Style.
- `name` (String, get/set) — The #name method gets the name for a Sketchup::Style.

## Styles (class)

The Styles class contains methods for manipulating a collection of styles in a model. Typically, you will access this via the active_model: There are two objects of this class that play important roles: the #selected_style and the #active_style. The latter is a temporary copy made from the #selected_style that allows the user to edit the style without committing to save anything. To save the changes, one should use #update_selected_style.

[Vendor docs](https://ruby.sketchup.com/Sketchup/Styles.html)

### Methods
#### `[](name) => Sketchup::Style? | [](index) => Sketchup::Style?`

The #[] method is used to retrieves a style by either name or index.

**Remarks:** The #[] method is used to retrieves a style by either name or index.

**Parameters:**
- `name` (String) — The name of the style to retrieve.

**Returns:** `Sketchup::Style, nil` — 

[Docs](https://ruby.sketchup.com/Sketchup/Styles.html#[]-instance_method)

#### `active_style => Sketchup::Style`

The #active_style method is used to retrieve the active style.

**Remarks:** The #active_style method is used to retrieve the active style. While #selected_style is the style being selected in the Style Browser, the ##active_style is a different object also including any unsaved style changes. These changes are silently dropped once a new style is selected. To save these changes to the selected style, call ##update_selected_style.

**Returns:** `Sketchup::Style` — 

[Docs](https://ruby.sketchup.com/Sketchup/Styles.html#active_style-instance_method)

#### `active_style_changed => Boolean`

The #active_style_changed method tells you if the active style has been edited by the user since it was last saved.

**Remarks:** The #active_style_changed method tells you if the active style has been edited by the user since it was last saved.

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Sketchup/Styles.html#active_style_changed-instance_method)

#### `add_style(filename, select) => Boolean | add_style(filename, select = false) => Sketchup::Style?`

The #add_style method is used to create and load a style from the given file.

**Remarks:** The #add_style method is used to create and load a style from the given file.

**Parameters:**
- `filename` (String) — The file path to the style file.
- `select` (Boolean) — true if you want to set the style to be the active style.

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Sketchup/Styles.html#add_style-instance_method)

#### `count => Integer`

Note: Since SketchUp 2014 the count method is inherited from Ruby's Enumerable mix-in module.

**Remarks:** Note: Since SketchUp 2014 the count method is inherited from Ruby's Enumerable mix-in module. Prior to that the #count method is an alias for #length.

**Returns:** `Integer` — 

[Docs](https://ruby.sketchup.com/Sketchup/Styles.html#count-instance_method)

#### `each {|style| ... } => nil`

The #each method is used to iterate through styles.

**Remarks:** The #each method is used to iterate through styles.

**Returns:** `nil` — 

[Docs](https://ruby.sketchup.com/Sketchup/Styles.html#each-instance_method)

#### `length => Integer`

The #length method is an alias of #size.

**Remarks:** The #length method is an alias of #size.

**Returns:** `Integer` — 

[Docs](https://ruby.sketchup.com/Sketchup/Styles.html#length-instance_method)

#### `parent => Sketchup::Model`

The #parent method is used to return the model for the styles.

**Remarks:** The #parent method is used to return the model for the styles.

**Returns:** `Sketchup::Model` — 

[Docs](https://ruby.sketchup.com/Sketchup/Styles.html#parent-instance_method)

#### `purge_unused => nil`

The #purge_unused method is used to remove unused styles from the model.

**Remarks:** The #purge_unused method is used to remove unused styles from the model.

**Returns:** `nil` — 

[Docs](https://ruby.sketchup.com/Sketchup/Styles.html#purge_unused-instance_method)

#### `remove_style(style) => nil`

The #remove_style method is used to remove a Sketchup::Style from the Sketchup::Styles.

**Remarks:** The #remove_style method is used to remove a Sketchup::Style from the Sketchup::Styles.

**Parameters:**
- `style` (Sketchup::Style)

**Returns:** `nil` — 

[Docs](https://ruby.sketchup.com/Sketchup/Styles.html#remove_style-instance_method)

#### `size => Integer`

The #size method is used to retrieve the number of styles in the collection.

**Remarks:** The #size method is used to retrieve the number of styles in the collection.

**Returns:** `Integer` — 

[Docs](https://ruby.sketchup.com/Sketchup/Styles.html#size-instance_method)

#### `update_selected_style => nil`

The #update_selected_style method commits the current style settings to the style selected in the Style Browser.

**Remarks:** The #update_selected_style method commits the current style settings to the style selected in the Style Browser.

**Returns:** `nil` — 

[Docs](https://ruby.sketchup.com/Sketchup/Styles.html#update_selected_style-instance_method)

### Properties
- `selected_style` (Sketchup::Style, get/set) — The #selected_style method is used to retrieve the style currently selected in the Styles Browser.

## Text (class)

The Text class contains method to manipulate a Text entity object.

[Vendor docs](https://ruby.sketchup.com/Sketchup/Text.html)

### Methods
#### `display_leader? => Boolean`

The display_leader? method returns the status of the leader.

**Remarks:** The display_leader? method returns the status of the leader.

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Sketchup/Text.html#display_leader?-instance_method)

#### `has_leader? => Boolean`

The has_leader method is used to determine if the Text object has a leader.

**Remarks:** The has_leader method is used to determine if the Text object has a leader.

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Sketchup/Text.html#has_leader?-instance_method)

#### `set_text(textstring) => Sketchup::Text`

The set_text method is used to set the text within a Text object without recording an Undo operation.

**Remarks:** The set_text method is used to set the text within a Text object without recording an Undo operation.

**Parameters:**
- `textstring` (String) — The string to be set within the Text object.

**Returns:** `Sketchup::Text` — the Text object

[Docs](https://ruby.sketchup.com/Sketchup/Text.html#set_text-instance_method)

### Properties
- `arrow_type` (Integer, get/set) — The arrow_type method retrieves the current arrow type used for the leader text.
- `attached_to` (Array(Sketchup::InstancePath, Geom::Point3d), nil, get/set) — The #attached_to method returns an array of the attached InstancePath object and the Geom::Point3d.
- `display_leader` (Boolean, set) — The display_leader= method accepts true or false for whether to display the leader.
- `leader_type` (Integer, get/set) — The #leader_type method retrieves the currently set leader type.
- `line_weight` (Integer, get/set) — The line_weight method returns a line weight in number of pixels.
- `point` (Geom::Point3d, get/set) — The point method is used to get the point associated with the text.
- `text` (String, get/set) — The text method is used to retrieve the string version of a Text object.
- `vector` (Geom::Vector3d, get/set) — The vector method is used to get the vector associated with the text.

## Texture (class)

The Texture class contains methods for obtaining information about textures that are part of your materials in your model (within the In-Model section of the Materials Browser). Remember, textures are repeatable images that “tile” when painted on a surface.

[Vendor docs](https://ruby.sketchup.com/Sketchup/Texture.html)

### Methods
#### `average_color => Sketchup::Color?`

The average_color method retrieves a color object with the average color found in the texture.

**Remarks:** The average_color method retrieves a color object with the average color found in the texture.

**Returns:** `Sketchup::Color, nil` — a color object (if successful), nil if unsuccessful.

[Docs](https://ruby.sketchup.com/Sketchup/Texture.html#average_color-instance_method)

#### `filename => String`

Note: Since SketchUp 2021.

**Remarks:** Note: Since SketchUp 2021.0 this method will append a file extension matching the image format if the file extension is missing from stored filepath. The #filename method retrieves the full path, if available, for a texture object. Textures for materials shipping with SketchUp might only have a filename, since the path would be invalid on the end user's machine. Textures dynamically created from ImageRep objects may have an empty path unless saved with ImageRep#save_file or until the texture is saved with #write. If you need only the filename of the texture use filename = File.basename(texture.filename).

**Returns:** `String` — a string representation of the path and filename used for the texture.

[Docs](https://ruby.sketchup.com/Sketchup/Texture.html#filename-instance_method)

#### `height => Integer`

The height method is used to get the height of a repeatable texture image, in inches.

**Remarks:** The height method is used to get the height of a repeatable texture image, in inches.

**Returns:** `Integer` — the height, in inches, of the texture pattern

[Docs](https://ruby.sketchup.com/Sketchup/Texture.html#height-instance_method)

#### `image_height => Integer`

The image_height method retrieves the height of the repeatable texture image, in pixels.

**Remarks:** The image_height method retrieves the height of the repeatable texture image, in pixels.

**Returns:** `Integer` — the height, in pixels, of the texture pattern

[Docs](https://ruby.sketchup.com/Sketchup/Texture.html#image_height-instance_method)

#### `image_rep(colorized = false) => Sketchup::ImageRep`

The #image_rep method returns a copy of a ImageRep object representing the texture pixel data.

**Remarks:** The #image_rep method returns a copy of a ImageRep object representing the texture pixel data.

**Parameters:**
- `colorized` (Boolean) — Set to true to obtain the colorized version.

**Returns:** `Sketchup::ImageRep` — 

[Docs](https://ruby.sketchup.com/Sketchup/Texture.html#image_rep-instance_method)

#### `image_width => Integer`

The image_width method retrieves the width of the repeatable texture image, in pixels.

**Remarks:** The image_width method retrieves the width of the repeatable texture image, in pixels.

**Returns:** `Integer` — the width, in pixels, of the texture pattern

[Docs](https://ruby.sketchup.com/Sketchup/Texture.html#image_width-instance_method)

#### `valid? => Boolean`

The valid? method ensures that a texture is valid.

**Remarks:** The valid? method ensures that a texture is valid.

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Sketchup/Texture.html#valid?-instance_method)

#### `width => Integer`

The width method is used to get the width of a repeatable texture image, in inches.

**Remarks:** The width method is used to get the width of a repeatable texture image, in inches.

**Returns:** `Integer` — the width, in inches, of the texture pattern

[Docs](https://ruby.sketchup.com/Sketchup/Texture.html#width-instance_method)

#### `write(path, colorize = false) => Boolean`

Writes the texture to file with option to preserve the color adjustments made by the material.

**Remarks:** Writes the texture to file with option to preserve the color adjustments made by the material.

**Parameters:**
- `path` (String) — The file path to write the texture to.
- `colorize` (Boolean) — Boolean - Allows for the texture to be exported with the color adjustments.

**Returns:** `Boolean` — true if the method succeeded

[Docs](https://ruby.sketchup.com/Sketchup/Texture.html#write-instance_method)

### Properties
- `size` (Integer, Array(Integer, Integer), set) — The size= method allows you to set the size of the repeatable texture image, in inches,

## TextureWriter (class)

The TextureWriter class is used primarily for writing the textures used in a SketchUp model out to files as part of an export for use in another application. These methods are usually invoked in this order: #load - load one or more textures from a model into the TextureWriter. #write_all or #write - write the texture(s) to file.

[Vendor docs](https://ruby.sketchup.com/Sketchup/TextureWriter.html)

### Methods
#### `count => Integer`

The #length method is used to determine the number of textures loaded into the texture writer.

**Remarks:** The #length method is used to determine the number of textures loaded into the texture writer. The #count method is an alias for #length.

**Returns:** `Integer` — length - the number of textures loaded in the texture writer

[Docs](https://ruby.sketchup.com/Sketchup/TextureWriter.html#count-instance_method)

#### `filename(handle) => String`

The filename method is used to retrieve the original filename for a particular texture.

**Remarks:** The filename method is used to retrieve the original filename for a particular texture.

**Parameters:**
- `handle` (Integer) — The index or handle of the texture in the texture writer.

**Returns:** `String` — filename - the filename of the texture on the file system. Textures can be generated without filenames, so if the texture doesn't have a name write_all will save the texture to an image file named “i.png”. Where i is the first integer greater than zero which was not already used for a png file name.

[Docs](https://ruby.sketchup.com/Sketchup/TextureWriter.html#filename-instance_method)

#### `handle(entity) => Integer | handle(face, side) => Integer`

The handle method is used to retrieve a handle or index for a specific texture in the texture writer.

**Remarks:** The handle method is used to retrieve a handle or index for a specific texture in the texture writer.

**Parameters:**
- `entity` (Sketchup::Entity) — A image, component instance, group, or layer.

**Returns:** `Integer` — the index for the entity in the texture writer.

[Docs](https://ruby.sketchup.com/Sketchup/TextureWriter.html#handle-instance_method)

#### `length => Integer`

The #length method is used to determine the number of textures loaded into the texture writer.

**Remarks:** The #length method is used to determine the number of textures loaded into the texture writer. The #count method is an alias for #length.

**Returns:** `Integer` — length - the number of textures loaded in the texture writer

[Docs](https://ruby.sketchup.com/Sketchup/TextureWriter.html#length-instance_method)

#### `load(entity) => Integer | load(face, side) => Integer`

Note: If you are passing a face in as the entity argument when loading a texture you will have to specify the second boolean argument, side.

**Remarks:** Note: If you are passing a face in as the entity argument when loading a texture you will have to specify the second boolean argument, side. The argument side specifies which side of the face the texture will be loaded from. The load method is used to load one or more textures into the texture writer for writing out to a file.

**Parameters:**
- `entity` (Sketchup::Entity) — Image, component instance, group, or layer to load.

**Returns:** `Integer` — handle - the index or handle of the entity that was loaded

[Docs](https://ruby.sketchup.com/Sketchup/TextureWriter.html#load-instance_method)

#### `write(entity, filename) => Integer | write(entity, side, filename) => Integer`

Note: If you are passing a face in as the entity argument when writing a texture you will have to specify the boolean argument, side.

**Remarks:** Note: If you are passing a face in as the entity argument when writing a texture you will have to specify the boolean argument, side. The argument side controls the side of the face from which the texture will be sampled before writing it. The write method is used to write an individual textures, within the texture writer, to a file. An entity's texture must have been loaded into the texture writer before this method can be used to write it's texture. This method will return one of the following status messages. (These are constants that are defined by the API.) 0 = FILE_WRITE_OK 1 = FILE_WRITE_FAILED_INVALID_TYPE 2 = FILE_WRITE_FAILED_UNKNOWN

**Parameters:**
- `entity` (Sketchup::Entity) — An image, component instance, group, or layer to write.
- `filename` (String) — The name of the file to contain the texture.

**Returns:** `Integer` — status - one of three status messages (see comments.)

[Docs](https://ruby.sketchup.com/Sketchup/TextureWriter.html#write-instance_method)

#### `write_all(dirname, filename_format) => Integer`

The write_all method is used to write all of the textures within the texture writer to files.

**Remarks:** The write_all method is used to write all of the textures within the texture writer to files. It will return one of three status numbers: 0 = FILE_WRITE_OK 1 = FILE_WRITE_FAILED_INVALID_TYPE 2 = FILE_WRITE_FAILED_UNKNOWN

**Parameters:**
- `dirname` (String) — The directory to write to.
- `filename_format` (Boolean) — true to use 8.3 DOS name format.

**Returns:** `Integer` — status - one of three status messages (see comments.)

[Docs](https://ruby.sketchup.com/Sketchup/TextureWriter.html#write_all-instance_method)

## Tool (class)

This class is abstract. Implement the methods described in this class to create a tool. You can not sub-class this class because it is not defined by the API. Tool is the interface that you implement to create a SketchUp tool. See our code example for how to create a custom tool in Ruby. To create a new tool in Ruby, you must define a new class that implements the methods for the events that you want to respond to. You do not have to implement methods for every possible event that a Tool can respond to. Once you have defined a tool class, you select that tool by creating an instance of it and passing it to Model#select_tool. For example: The following table contains several constants you can use when check for certain key presses inside the keyboard handling callbacks: CONSTRAIN_MODIFIER_KEY = Shift Key CONSTRAIN_MODIFIER_MASK = Shift Key COPY_MODIFIER_KEY = Alt/Option on Mac, Ctrl on PC COPY_MODIFIER_MASK = Alt/Option on Mac, Ctrl on PC ALT_MODIFIER_KEY = Command on Mac, Alt on PC ALT_MODIFIER_MASK = Command on Mac, Alt on PC

[Vendor docs](https://ruby.sketchup.com/Sketchup/Tool.html)

### Methods
#### `activate => Object`

The #activate method is called by SketchUp when the tool is selected.

**Remarks:** The #activate method is called by SketchUp when the tool is selected. It is a good place to put most of your initialization, such as instance variables to track the state of the tool.

[Docs](https://ruby.sketchup.com/Sketchup/Tool.html#activate-instance_method)

#### `deactivate(view) => Object`

The #deactivate method is called when the tool is deactivated because a different tool was selected.

**Remarks:** The #deactivate method is called when the tool is deactivated because a different tool was selected.

**Parameters:**
- `view` (Sketchup::View)

[Docs](https://ruby.sketchup.com/Sketchup/Tool.html#deactivate-instance_method)

#### `draw(view) => Object`

Note: If you draw outside the model bounds you need to implement #getExtents which return a bounding box large enough to include the points you draw.

**Remarks:** Note: If you draw outside the model bounds you need to implement #getExtents which return a bounding box large enough to include the points you draw. Otherwise your drawing will be clipped. The #draw method is called by SketchUp whenever the view is refreshed to allow the tool to do its own drawing. If the tool has some temporary graphics that it wants displayed while it is active, it should implement this method and draw to the View.

**Parameters:**
- `view` (Sketchup::View) — A View object where the method was invoked.

[Docs](https://ruby.sketchup.com/Sketchup/Tool.html#draw-instance_method)

#### `enableVCB? => Boolean`

The #enableVCB? method is used to tell SketchUp whether to allow the user to enter text into the VCB (value control box, aka the “measurements” panel).

**Remarks:** The #enableVCB? method is used to tell SketchUp whether to allow the user to enter text into the VCB (value control box, aka the “measurements” panel). If you do not implement this method, then the vcb is disabled by default.

**Returns:** `Boolean` — Return true if you want the VCB enabled

[Docs](https://ruby.sketchup.com/Sketchup/Tool.html#enableVCB?-instance_method)

#### `getExtents => Geom::BoundingBox`

In order to accurately draw things, SketchUp needs to know the extents of what it is drawing.

**Remarks:** In order to accurately draw things, SketchUp needs to know the extents of what it is drawing. If the tool is doing its own drawing, it may need to implement this method to tell SketchUp the extents of what it will be drawing. If you don't implement this method, you may find that part of what the tool is drawing gets clipped to the extents of the rest of the model.

**Returns:** `Geom::BoundingBox` — 

[Docs](https://ruby.sketchup.com/Sketchup/Tool.html#getExtents-instance_method)

#### `getInstructorContentDirectory => String`

Note: Prior to SketchUp 2014 this method would assume the path was relative to the SketchUp resource folder.

**Remarks:** Note: Prior to SketchUp 2014 this method would assume the path was relative to the SketchUp resource folder. From 2014 and onwards you can specify the absolute path to an HTML file or the absolute path to a directory containing an index.html file. The #getInstructorContentDirectory method is used to tell SketchUp the directory containing your Tool's instructor content. To use this, create a custom instructor directory, put an index.html file inside of it, and then return that path via this method. If the SketchUp user has the Instructor window open when they activate your tool, they will see your html file.

**Returns:** `String` — the directory path where the Instructor content exists.

[Docs](https://ruby.sketchup.com/Sketchup/Tool.html#getInstructorContentDirectory-instance_method)

#### `getMenu(menu) => nil | getMenu(menu, flags, x, y, view) => nil | getMenu(menu, flags, x, y, view) => nil`

Note: In SketchUp 2015 the flags, x, y and view parameters were added.

**Remarks:** Note: In SketchUp 2015 the flags, x, y and view parameters were added. They are needed if you need to pick the entities under the mouse position. The new parameters are optional, but if you need to use one you must include them all. The #getMenu method is called by SketchUp to let the tool provide its own context menu. Most tools will not want to implement this method and, instead, use the normal context menu found on all entities. If you do implement this method, the argument is a Menu. You should use the Menu#add_item method to build the context menu. Your tool will use a standard context menu by default if you do not implement this method. Implement this method if you want a context-click to display something other than this default context menu.

**Parameters:**
- `menu` (Sketchup::Menu)

**Returns:** `nil` — 

[Docs](https://ruby.sketchup.com/Sketchup/Tool.html#getMenu-instance_method)

#### `onCancel(reason, view) => Object`

Note: When something is undone #onCancel is called before the undo is actually executed.

**Remarks:** Note: When something is undone #onCancel is called before the undo is actually executed. If you need to do something with the model after an undo use ModelObserver#onTransactionUndo. Note: When #onKeyDown is implemented and returns true, pressing Esc doesn't trigger #onCancel. The #onCancel method is called by SketchUp to cancel the current operation of the tool. The typical response will be to reset the tool to its initial state. The reason identifies the action that triggered the call. The reason can be one of the following values: 0: the user canceled the current operation by hitting the escape key. 1: the user re-selected the same tool from the toolbar or menu. 2: the user did an undo while the tool was active.

**Parameters:**
- `reason` (Integer) — A reason value (see comments).
- `view` (Sketchup::View)

[Docs](https://ruby.sketchup.com/Sketchup/Tool.html#onCancel-instance_method)

#### `onKeyDown(key, repeat, flags, view) => Boolean`

The #onKeyDown method is called by SketchUp when the user presses a key on the keyboard.

**Remarks:** The #onKeyDown method is called by SketchUp when the user presses a key on the keyboard. If you want to get input from the VCB, you should implement onUserText rather than this method. This method is can be used for special keys such as the Shift key, Ctrl key, and so on, or for just determining which key a user pressed. This method is actually called for all keys that are pressed. There are several “virtual keys” defined as constants you can use. Their use is cross platform. They are: VK_ALT VK_COMMAND VK_CONTROL VK_DELETE VK_DOWN VK_END VK_HOME VK_INSERT VK_LEFT VK_MENU VK_NEXT VK_PRIOR VK_RIGHT VK_SHIFT VK_SPACE VK_UP V6: There is a bug on Windows where the typematic effect does not work. Typematic effects work fine on a Mac.

**Parameters:**
- `key` (Integer) — The key that was pressed.
- `repeat` (Integer) — A value of 1 for a single press of a key. A value of 2 if the user has pressed a key and is holding it down.
- `flags` (Integer) — A bit mask that tells the state of the modifier keys at the time of the event.
- `view` (Sketchup::View)

**Returns:** `Boolean` — Return true to prevent SketchUp from processing the event.

[Docs](https://ruby.sketchup.com/Sketchup/Tool.html#onKeyDown-instance_method)

#### `onKeyUp(key, repeat, flags, view) => Boolean`

The #onKeyUp method is called by SketchUp when the user releases a key on the keyboard.

**Remarks:** The #onKeyUp method is called by SketchUp when the user releases a key on the keyboard.

**Parameters:**
- `key` (Integer) — The key that was pressed.
- `repeat` (Integer) — A value of 1 for a single press of a key. A value of 2 if the user has pressed a key and is holding it down.
- `flags` (Integer) — A bit mask that tells the state of the modifier keys at the time of the event.
- `view` (Sketchup::View)

**Returns:** `Boolean` — Return true to prevent SketchUp from processing the event.

[Docs](https://ruby.sketchup.com/Sketchup/Tool.html#onKeyUp-instance_method)

#### `onLButtonDoubleClick(flags, x, y, view) => Object | onLButtonDoubleClick(flags, x, y, view) => Object`

The #onLButtonDoubleClick is called by SketchUp when the user double clicks with the left mouse button.

**Remarks:** The #onLButtonDoubleClick is called by SketchUp when the user double clicks with the left mouse button.

**Parameters:**
- `flags` (Integer) — A bit mask that tells the state of the modifier keys and other mouse buttons at the time.
- `x` (Integer) — Screen coordinate in physical pixels.
- `y` (Integer) — Screen coordinate in physical pixels.
- `view` (Sketchup::View)

[Docs](https://ruby.sketchup.com/Sketchup/Tool.html#onLButtonDoubleClick-instance_method)

#### `onLButtonDown(flags, x, y, view) => Object | onLButtonDown(flags, x, y, view) => Object`

The #onLButtonDown method is called by SketchUp when the left mouse button is pressed.

**Remarks:** The #onLButtonDown method is called by SketchUp when the left mouse button is pressed. Most tools will implement this method.

**Parameters:**
- `flags` (Integer) — A bit mask that tells the state of the modifier keys and other mouse buttons at the time.
- `x` (Integer) — Screen coordinate in physical pixels.
- `y` (Integer) — Screen coordinate in physical pixels.
- `view` (Sketchup::View)

[Docs](https://ruby.sketchup.com/Sketchup/Tool.html#onLButtonDown-instance_method)

#### `onLButtonUp(flags, x, y, view) => Object | onLButtonUp(flags, x, y, view) => Object`

The #onLButtonUp method is called by SketchUp when the left mouse button is released.

**Remarks:** The #onLButtonUp method is called by SketchUp when the left mouse button is released.

**Parameters:**
- `flags` (Integer) — A bit mask that tells the state of the modifier keys and other mouse buttons at the time.
- `x` (Integer) — Screen coordinate in physical pixels.
- `y` (Integer) — Screen coordinate in physical pixels.
- `view` (Sketchup::View)

[Docs](https://ruby.sketchup.com/Sketchup/Tool.html#onLButtonUp-instance_method)

#### `onMButtonDoubleClick(flags, x, y, view) => Object | onMButtonDoubleClick(flags, x, y, view) => Object`

Note: Though this method has been documented in the Ruby API for many years, it has never worked properly.

**Remarks:** Note: Though this method has been documented in the Ruby API for many years, it has never worked properly. We are leaving this documentation in place for now in the hopes of fixing the implementation, but you won't have any luck trying to use it in SU7 and earlier. The #onMButtonDoubleClick method is called by SketchUp when the middle mouse button (on a three button mouse) is double-clicked. Only implement this method if you want SketchUp to react to a middle mouse button being double-clicked.

**Parameters:**
- `flags` (Integer) — A bit mask that tells the state of the modifier keys and other mouse buttons at the time.
- `x` (Integer) — Screen coordinate in physical pixels.
- `y` (Integer) — Screen coordinate in physical pixels.
- `view` (Sketchup::View)

[Docs](https://ruby.sketchup.com/Sketchup/Tool.html#onMButtonDoubleClick-instance_method)

#### `onMButtonDown(flags, x, y, view) => Object | onMButtonDown(flags, x, y, view) => Object`

The #onMButtonDown method is called by SketchUp when the middle mouse button (on a three button mouse) is down.

**Remarks:** The #onMButtonDown method is called by SketchUp when the middle mouse button (on a three button mouse) is down. The Orbit tool is activated by default when the middle mouse button is down. Implement this method if you want a middle mouse button to do something other than invoke the Orbit tool.

**Parameters:**
- `flags` (Integer) — A bit mask that tells the state of the modifier keys and other mouse buttons at the time.
- `x` (Integer) — Screen coordinate in physical pixels.
- `y` (Integer) — Screen coordinate in physical pixels.
- `view` (Sketchup::View)

[Docs](https://ruby.sketchup.com/Sketchup/Tool.html#onMButtonDown-instance_method)

#### `onMButtonUp(flags, x, y, view) => Object | onMButtonUp(flags, x, y, view) => Object`

The #onMButtonUp method is called by SketchUp when the middle mouse button (on a three button mouse) is released.

**Remarks:** The #onMButtonUp method is called by SketchUp when the middle mouse button (on a three button mouse) is released. SketchUp returns to the previous tool from the Orbit tool when the middle mouse button is released. Implement this method if you want a middle mouse button to do something other than return to the previous tool when in the Orbit tool.

**Parameters:**
- `flags` (Integer) — A bit mask that tells the state of the modifier keys and other mouse buttons at the time.
- `x` (Integer) — Screen coordinate in physical pixels.
- `y` (Integer) — Screen coordinate in physical pixels.
- `view` (Sketchup::View)

[Docs](https://ruby.sketchup.com/Sketchup/Tool.html#onMButtonUp-instance_method)

#### `onMouseEnter(view) => Object`

The #onMouseEnter method is called by SketchUp when the mouse enters the viewport.

**Remarks:** The #onMouseEnter method is called by SketchUp when the mouse enters the viewport.

**Parameters:**
- `view` (Sketchup::View)

[Docs](https://ruby.sketchup.com/Sketchup/Tool.html#onMouseEnter-instance_method)

#### `onMouseLeave(view) => Object`

The #onMouseLeave method is called by SketchUp when the mouse leaves the viewport.

**Remarks:** The #onMouseLeave method is called by SketchUp when the mouse leaves the viewport.

**Parameters:**
- `view` (Sketchup::View)

[Docs](https://ruby.sketchup.com/Sketchup/Tool.html#onMouseLeave-instance_method)

#### `onMouseMove(flags, x, y, view) => Object | onMouseMove(flags, x, y, view) => Object`

The #onMouseMove method is called by SketchUp whenever the mouse is moved.

**Remarks:** The #onMouseMove method is called by SketchUp whenever the mouse is moved. You will often want to implement this method. Try to make this method as efficient as possible because this method is called often.

**Parameters:**
- `flags` (Integer) — A bit mask that tells the state of the modifier keys and other mouse buttons at the time.
- `x` (Integer) — Screen coordinate in physical pixels.
- `y` (Integer) — Screen coordinate in physical pixels.
- `view` (Sketchup::View)

[Docs](https://ruby.sketchup.com/Sketchup/Tool.html#onMouseMove-instance_method)

#### `onMouseWheel(flags, delta, x, y, view) => Boolean | onMouseWheel(flags, delta, x, y, view) => Boolean`

The #onMouseWheel method is called by SketchUp when the mouse scroll wheel is used.

**Remarks:** The #onMouseWheel method is called by SketchUp when the mouse scroll wheel is used.

**Parameters:**
- `flags` (Integer) — A bit mask that tells the state of the modifier keys and other mouse buttons at the time.
- `delta` (Integer) — Either 1 or -1 depending on which direction the mouse wheel scrolled.
- `x` (Integer) — Screen coordinate in physical pixels.
- `y` (Integer) — Screen coordinate in physical pixels.
- `view` (Sketchup::View)

**Returns:** `Boolean` — Return true to prevent SketchUp from performing default zoom action.

[Docs](https://ruby.sketchup.com/Sketchup/Tool.html#onMouseWheel-instance_method)

#### `onRButtonDoubleClick(flags, x, y, view) => Object | onRButtonDoubleClick(flags, x, y, view) => Object`

The #onRButtonDoubleClick is called by SketchUp when the user double clicks with the right mouse button.

**Remarks:** The #onRButtonDoubleClick is called by SketchUp when the user double clicks with the right mouse button.

**Parameters:**
- `flags` (Integer) — A bit mask that tells the state of the modifier keys and other mouse buttons at the time.
- `x` (Integer) — Screen coordinate in physical pixels.
- `y` (Integer) — Screen coordinate in physical pixels.
- `view` (Sketchup::View)

[Docs](https://ruby.sketchup.com/Sketchup/Tool.html#onRButtonDoubleClick-instance_method)

#### `onRButtonDown(flags, x, y, view) => Object | onRButtonDown(flags, x, y, view) => Object`

The #onRButtonDown method is called by SketchUp when the user presses the right mouse button.

**Remarks:** The #onRButtonDown method is called by SketchUp when the user presses the right mouse button. Implement this method, along with the tool.getMenu method, when you want your tool to do something other than display the default context menu when the right mouse button is clicked.

**Parameters:**
- `flags` (Integer) — A bit mask that tells the state of the modifier keys and other mouse buttons at the time.
- `x` (Integer) — Screen coordinate in physical pixels.
- `y` (Integer) — Screen coordinate in physical pixels.
- `view` (Sketchup::View)

[Docs](https://ruby.sketchup.com/Sketchup/Tool.html#onRButtonDown-instance_method)

#### `onRButtonUp(flags, x, y, view) => Object | onRButtonUp(flags, x, y, view) => Object`

The #onRButtonUp method is called by SketchUp when the user releases the right mouse button.

**Remarks:** The #onRButtonUp method is called by SketchUp when the user releases the right mouse button.

**Parameters:**
- `flags` (Integer) — A bit mask that tells the state of the modifier keys and other mouse buttons at the time.
- `x` (Integer) — Screen coordinate in physical pixels.
- `y` (Integer) — Screen coordinate in physical pixels.
- `view` (Sketchup::View)

[Docs](https://ruby.sketchup.com/Sketchup/Tool.html#onRButtonUp-instance_method)

#### `onReturn(view) => nil`

The #onReturn method is called by SketchUp when the user hit the Return key to complete an operation in the tool.

**Remarks:** The #onReturn method is called by SketchUp when the user hit the Return key to complete an operation in the tool. This method will rarely need to be implemented.

**Parameters:**
- `view` (Sketchup::View)

**Returns:** `nil` — 

[Docs](https://ruby.sketchup.com/Sketchup/Tool.html#onReturn-instance_method)

#### `onSetCursor => Boolean`

The #onSetCursor method is called by SketchUp when the tool wants to set the cursor.

**Remarks:** The #onSetCursor method is called by SketchUp when the tool wants to set the cursor.

**Returns:** `Boolean` — Return true to prevent SketchUp using the default cursor.

[Docs](https://ruby.sketchup.com/Sketchup/Tool.html#onSetCursor-instance_method)

#### `onUserText(text, view) => Object`

The #onUserText method is called by SketchUp when the user has typed text into the VCB and hit return.

**Remarks:** The #onUserText method is called by SketchUp when the user has typed text into the VCB and hit return.

**Parameters:**
- `text` (String) — The text string that was typed into the VCB.
- `view` (Sketchup::View) — A view object where the method was invoked.

[Docs](https://ruby.sketchup.com/Sketchup/Tool.html#onUserText-instance_method)

#### `resume(view) => Object`

The #resume method is called by SketchUp when the tool becomes active again after being suspended.

**Remarks:** The #resume method is called by SketchUp when the tool becomes active again after being suspended.

**Parameters:**
- `view` (Sketchup::View)

[Docs](https://ruby.sketchup.com/Sketchup/Tool.html#resume-instance_method)

#### `suspend(view) => Object`

The #suspend method is called by SketchUp when the tool temporarily becomes inactive because another tool has been activated.

**Remarks:** The #suspend method is called by SketchUp when the tool temporarily becomes inactive because another tool has been activated. This typically happens when a viewing tool is activated, such as when orbit is active due to the middle mouse button.

**Parameters:**
- `view` (Sketchup::View)

[Docs](https://ruby.sketchup.com/Sketchup/Tool.html#suspend-instance_method)

## Tools (class)

The Tools class contains methods to manipulate a collection of SketchUp tools. You access this collection by calling the Model.tools method.

[Vendor docs](https://ruby.sketchup.com/Sketchup/Tools.html)

### Methods
#### `active_tool => Object?`

The #active_tool method is used to obtain the active Ruby tool.

**Remarks:** The #active_tool method is used to obtain the active Ruby tool.

**Returns:** `Object, nil` — Returns the active Ruby tool, or `nil` otherwise.

[Docs](https://ruby.sketchup.com/Sketchup/Tools.html#active_tool-instance_method)

#### `active_tool_id => Object`

The active_tool_id method is used to retrieve the active tool's id.

**Remarks:** The active_tool_id method is used to retrieve the active tool's id.

**Returns:** `Object` — id - the active tool's id.

[Docs](https://ruby.sketchup.com/Sketchup/Tools.html#active_tool_id-instance_method)

#### `active_tool_name => Object`

The active_tool_name method is used to retrieve the active tool's name.

**Remarks:** The active_tool_name method is used to retrieve the active tool's name.

**Returns:** `Object` — name = the active tool's name.

[Docs](https://ruby.sketchup.com/Sketchup/Tools.html#active_tool_name-instance_method)

#### `add_observer(observer) => Object`

The add_observer method is used to add an observer to the current object.

**Remarks:** The add_observer method is used to add an observer to the current object.

**Parameters:**
- `observer` (Object) — An observer.

**Returns:** `Object` — status - true if successful, false if unsuccessful.

[Docs](https://ruby.sketchup.com/Sketchup/Tools.html#add_observer-instance_method)

#### `model => Object`

The model method is used to get the model associated with this tools object.

**Remarks:** The model method is used to get the model associated with this tools object.

**Returns:** `Object` — model - the Model object associated with this tools collection.

[Docs](https://ruby.sketchup.com/Sketchup/Tools.html#model-instance_method)

#### `pop_tool => Object`

The pop_tool method is used to pop the last pushed tool on the tool stack.

**Remarks:** The pop_tool method is used to pop the last pushed tool on the tool stack.

**Returns:** `Object` — the last pushed Tool object, if it is a Ruby tool. If a native tool is active, selects the default native tool and returns nil.

[Docs](https://ruby.sketchup.com/Sketchup/Tools.html#pop_tool-instance_method)

#### `push_tool(tool) => Object`

The push_tool method is used to push (aka activate) a user-defined tool.

**Remarks:** The push_tool method is used to push (aka activate) a user-defined tool. See the Tool interface for details on creating your own SketchUp tool.

**Parameters:**
- `tool` (Object) — A user.

**Returns:** `Object` — status - true if successful, false if unsuccessful.

[Docs](https://ruby.sketchup.com/Sketchup/Tools.html#push_tool-instance_method)

#### `remove_observer(observer) => Object`

The remove_observer method is used to remove an observer from the current object.

**Remarks:** The remove_observer method is used to remove an observer from the current object.

**Parameters:**
- `observer` (Object) — An observer.

**Returns:** `Object` — true if successful, false if unsuccessful.

[Docs](https://ruby.sketchup.com/Sketchup/Tools.html#remove_observer-instance_method)

## ToolsObserver (class)

This class is abstract. To implement this observer, create a Ruby class of this type, override the desired methods, and add an instance of the observer to the Tools object. This observer interface is implemented to react to tool events. Some of the code below mentions tool_names and tool_ids. Here is a list of the common tool IDs and names: 21013 = 3DTextTool 21065 = ArcTool 10523 = CameraDollyTool 10508 = CameraOrbitTool 10525 = CameraPanTool 21169 = PositionCameraTool 10520 = CameraWalkTool 10509 = CameraZoomTool 10526 = CameraZoomWindowTool 21096 = CircleTool 21013 = ComponentTool 21126 = ComponentCSTool 21410 = DimensionTool 21019 = EraseTool 21031 = FreehandTool 21525 = ExtrudeTool 21126 = SketchCSTool 21048 = MoveTool 21024 = MeasureTool 21100 = OffsetTool 21074 = PaintTool 21013 = PasteTool 21095 = PolyTool 21515 = PositionTextureTool 21041 = PushPullTool 21057 = ProtractorTool 21094 = RectangleTool 21129 = RotateTool 21236 = ScaleTool 21022 = SelectionTool 21337 = SectionPlaneTool 21020 = SketchTool 21405 = TextTool

[Vendor docs](https://ruby.sketchup.com/Sketchup/ToolsObserver.html)

### Methods
#### `onActiveToolChanged(tools, tool_name, tool_id) => nil`

Note: In SketchUp 6 and SketchUp 7.

**Remarks:** Note: In SketchUp 6 and SketchUp 7.0, tool names on the Mac have some of their first characters truncated. For instance, on Windows, a tool is “CameraOrbit”. On the Mac, is comes across as “raOrbit”. Therefore, use the tool_id to keep track of which tool you need to watch for, or use logic that corrects for the error. There is an example method of one way to do this shown below. (This example is not a comprehensive list of the tool names.) Once you subclass Sketchup::ToolsObserver with your unique class, you can override the #onActiveToolChanged method to receive tool change notifications.

**Parameters:**
- `tools` (Sketchup::Tools) — A Tools object.
- `tool_name` (String) — The name of the tool.
- `tool_id` (Integer) — The ID of the tool. This is a predefined number unique to a given tool. For example, the Materials Browser is tool_id 21074.

**Returns:** `nil` — 

[Docs](https://ruby.sketchup.com/Sketchup/ToolsObserver.html#onActiveToolChanged-instance_method)

#### `onToolStateChanged(tools, tool_name, tool_id, tool_state) => nil`

Note: In SketchUp 6 and SketchUp 7, tool names on the Mac have their first few characters truncated.

**Remarks:** Note: In SketchUp 6 and SketchUp 7, tool names on the Mac have their first few characters truncated. For instance, on Windows, a tool is “CameraOrbit”. On the Mac, is comes across as “raOrbit”. Therefore, use the tool_id to keep track of which tool you need to watch for, or use logic that corrects for the error. This bug was fixed in SketchUp 8.0. The #onToolStateChanged method is called each time the user performs an action with a tool. The actual state that is returned is an internal number that varies tool to tool. If you want to watch existing tools for every interaction, you will need to experiment with the tool state to determine which states you care about. There is little consistency tool to tool.

**Parameters:**
- `tools` (Sketchup::Tools) — A Tools object.
- `tool_name` (String) — The name of the tool.
- `tool_id` (Integer) — The ID of the tool. This is a predefined number unique to a given tool. For example, the Materials Browser is tool_id 21074.
- `tool_state` (Integer) — A number identifying the state the tool just entered.

**Returns:** `nil` — 

[Docs](https://ruby.sketchup.com/Sketchup/ToolsObserver.html#onToolStateChanged-instance_method)

## UVHelper (class)

The UV Helper class contains methods allowing you to determine the location (UV coordinates) of a texture on a face. This class is particularly useful in determining how textures that have been manipulated using the Texture Tweaker should appear when exported to another file type Use the Face.get_UVHelper method to create a UVHelper for a given face. See the TextureWriter class as well.

[Vendor docs](https://ruby.sketchup.com/Sketchup/UVHelper.html)

### Methods
#### `get_back_UVQ(point) => Geom::Point3d`

Note: To convert UVQ coordinates to UV, divide U and V by Q.

**Remarks:** Note: To convert UVQ coordinates to UV, divide U and V by Q. The #get_back_UVQ method is used to get the UV texture coordinates on the back of a face.

**Parameters:**
- `point` (Geom::Point3d) — A point on the face.

**Returns:** `Geom::Point3d` — Point where X represents U, Y represents V and Z represents Q.

[Docs](https://ruby.sketchup.com/Sketchup/UVHelper.html#get_back_UVQ-instance_method)

#### `get_front_UVQ(point) => Geom::Point3d`

Note: To convert UVQ coordinates to UV, divide U and V by Q.

**Remarks:** Note: To convert UVQ coordinates to UV, divide U and V by Q. The #get_front_UVQ method is used to get the UV texture coordinates on the front of a face.

**Parameters:**
- `point` (Geom::Point3d) — A point on the face.

**Returns:** `Geom::Point3d` — Point where X represents U, Y represents V and Z represents Q.

[Docs](https://ruby.sketchup.com/Sketchup/UVHelper.html#get_front_UVQ-instance_method)

## Vertex (class)

A Vertex. A Vertex represents the end of an Edge or a point inside a Face.

[Vendor docs](https://ruby.sketchup.com/Sketchup/Vertex.html)

### Methods
#### `common_edge(vertex2) => Sketchup::Edge?`

The common_edge method is used to find a common edge that is defined by this vertex and another vertex

**Remarks:** The common_edge method is used to find a common edge that is defined by this vertex and another vertex

**Parameters:**
- `vertex2` (Sketchup::Vertex) — A Vertex object.

**Returns:** `Sketchup::Edge, nil` — an Edge object common to both vertices if successful. Returns nil if there is no edge between the two vertices.

[Docs](https://ruby.sketchup.com/Sketchup/Vertex.html#common_edge-instance_method)

#### `curve_interior? => Sketchup::ArcCurve?`

Note: This method doesn't actually return a boolean as the question mark post-fix would normally indicate.

**Remarks:** Note: This method doesn't actually return a boolean as the question mark post-fix would normally indicate. But the result still evaluates to truthy or falsy. The #curve_interior? method is used to determine if this vertex is on the interior of a Curve.

**Returns:** `Sketchup::ArcCurve, nil` — 

[Docs](https://ruby.sketchup.com/Sketchup/Vertex.html#curve_interior?-instance_method)

#### `edges => Array<Sketchup::Edge>`

The edges method is used to retrieve an Array of edges that use the Vertex.

**Remarks:** The edges method is used to retrieve an Array of edges that use the Vertex.

**Returns:** `Array<Sketchup::Edge>` — an Array of edge objects if successful

[Docs](https://ruby.sketchup.com/Sketchup/Vertex.html#edges-instance_method)

#### `faces => Array<Sketchup::Face>`

The faces method is used to retrieve an Array of faces that use the vertex.

**Remarks:** The faces method is used to retrieve an Array of faces that use the vertex.

**Returns:** `Array<Sketchup::Face>` — an Array of faces that use the vertex if successful

[Docs](https://ruby.sketchup.com/Sketchup/Vertex.html#faces-instance_method)

#### `loops => Array<Sketchup::Loop>`

The loops method is used to retrieve an Array of loops that use the vertex.

**Remarks:** The loops method is used to retrieve an Array of loops that use the vertex.

**Returns:** `Array<Sketchup::Loop>` — an Array of loops that use the vertex if successful

[Docs](https://ruby.sketchup.com/Sketchup/Vertex.html#loops-instance_method)

#### `position => Geom::Point3d`

The position method is used to retrieve the Point3d position of a vertex.

**Remarks:** The position method is used to retrieve the Point3d position of a vertex.

**Returns:** `Geom::Point3d` — a Point3d object representing the position of the vertex if successful

[Docs](https://ruby.sketchup.com/Sketchup/Vertex.html#position-instance_method)

#### `used_by?(face_or_edge) => Boolean`

The used_by? method is used to determine if the Vertex is used by a given Edge or Face.

**Remarks:** The used_by? method is used to determine if the Vertex is used by a given Edge or Face.

**Parameters:**
- `face_or_edge` (Sketchup::Edge, Sketchup::Face) — A Face or Edge ot test against.

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Sketchup/Vertex.html#used_by?-instance_method)

## View (class)

This class contains methods to manipulate the current point of view of the model. The drawing methods here (#draw_line, #draw_polyline, etc) are meant to be invoked within a tool's Tool#draw method. Calling them outside Tool#draw will have no effect. You access the View by calling the Model#active_view method.

[Vendor docs](https://ruby.sketchup.com/Sketchup/View.html)

### Methods
#### `add_observer(observer) => Boolean`

The add_observer method is used to add an observer to the current object.

**Remarks:** The add_observer method is used to add an observer to the current object.

**Parameters:**
- `observer` (Object) — An observer.

**Returns:** `Boolean` — true if successful, false if unsuccessful.

[Docs](https://ruby.sketchup.com/Sketchup/View.html#add_observer-instance_method)

#### `average_refresh_time => Float`

The average_refresh_time is used to set the average time used to refresh the current model in the view.

**Remarks:** The average_refresh_time is used to set the average time used to refresh the current model in the view. This can be used to estimate the frame rate for an animation.

**Returns:** `Float` — the time in seconds

[Docs](https://ruby.sketchup.com/Sketchup/View.html#average_refresh_time-instance_method)

#### `center => Array(Integer, Integer) | center => Array(Float, Float)`

The #center method is used to retrieve the coordinates of the center of the view in pixels.

**Remarks:** The #center method is used to retrieve the coordinates of the center of the view in pixels.

**Returns:** `Array(Integer, Integer)` — Physical pixels.

[Docs](https://ruby.sketchup.com/Sketchup/View.html#center-instance_method)

#### `corner(index) => Array(Integer, Integer) | corner(index) => Array(Float, Float)`

Note: If the view uses a Camera with a fixed aspect ratio, then the corners are the corners of the viewing are of the camera which might be different than the actual corners of the view itself.

**Remarks:** Note: If the view uses a Camera with a fixed aspect ratio, then the corners are the corners of the viewing are of the camera which might be different than the actual corners of the view itself. The #corner method is used to retrieve the coordinates of one of the corners of the view. The argument is an index between 0 and 3 that identifies which corner you want. This method returns an array with two integers which are the coordinates of the corner of the view in the view space. The indices are as follows: 0: CORNER_TOP_LEFT 1: CORNER_TOP_RIGHT 2: CORNER_BOTTOM_LEFT 3: CORNER_BOTTOM_RIGHT

**Parameters:**
- `index` (Integer) — A value between (or including) 0 and 3 identifying the corner whose coordinate you want to retrieve.

**Returns:** `Array(Integer, Integer)` — a 2D array [x, y] representing the screen point in physical pixels.

[Docs](https://ruby.sketchup.com/Sketchup/View.html#corner-instance_method)

#### `device_height => Integer`

The #device_height method is used to retrieve the height of the viewport for the view in physical pixels.

**Remarks:** The #device_height method is used to retrieve the height of the viewport for the view in physical pixels.

**Returns:** `Integer` — the height of the viewport in physical pixels.

[Docs](https://ruby.sketchup.com/Sketchup/View.html#device_height-instance_method)

#### `device_width => Integer`

The #device_width method is used to retrieve the width of the viewport for the view in physical pixels.

**Remarks:** The #device_width method is used to retrieve the width of the viewport for the view in physical pixels.

**Returns:** `Integer` — the width of the viewport in physical pixels.

[Docs](https://ruby.sketchup.com/Sketchup/View.html#device_width-instance_method)

#### `draw(openglenum, points) => Sketchup::View | draw(openglenum, *points) => Sketchup::View | draw(openglenum, points, **options) => Sketchup::View | draw(openglenum, *points, **options) => Sketchup::View`

Note: If you draw outside the model bounds you need to implement Tool#getExtents which returns a bounding box large enough to include the points you draw.

**Remarks:** Note: If you draw outside the model bounds you need to implement Tool#getExtents which returns a bounding box large enough to include the points you draw. Otherwise your drawing will be clipped. The #draw method is used to do basic drawing. This method can only be called from within the Tool#draw method of a tool that you implement in Ruby. The following constants are all OpenGL terms and have been externalized to Ruby. Here is a summary of their meanings: GL_POINTS Treats each vertex as a single point. Vertex n defines point n. N points are drawn. GL_LINES Treats each pair of vertices as an independent line segment. Vertices 2n-1 and 2n define line n. N/2 lines are drawn. GL_LINE_STRIP Draws a connected group of line segments from the first vertex to the last. Vertices n and n+1 define line n. N-1 lines are drawn. GL_LINE_LOOP Draws a connected group of line segments from the first vertex to the last, then back to the first. Vertices n and n+1 define line n. The last line, however, is defined by vertices N and 1. N lines are drawn. GL_TRIANGLES Treats each triplet of vertices as an independent triangle. Vertices 3n-2, 3n-1, and 3n define triangle n. N/3 triangles are drawn. GL_TRIANGLE_STRIP Draws a connected group of triangles. One triangle is defined for each vertex presented after the first two vertices. For odd n, vertices n, n+1, and n+2 define triangle n. For even n, vertices n+1, n, and n+2 define triangle n. N-2 triangles are drawn. GL_TRIANGLE_FAN Draws a connected group of triangles. One triangle is defined for each vertex presented after the first two vertices. Vertices 1, n+1, and n+2 define triangle n. N-2 triangles are drawn. GL_QUADS Treats each group of four vertices as an independent quadrilateral. Vertices 4n-3, 4n-2, 4n-1, and 4n define quadrilateral n. N/4 quadrilaterals are drawn. GL_QUAD_STRIP Draws a connected group of quadrilaterals. One quadrilateral is defined for each pair of vertices presented after the first pair. Vertices 2n-1, 2n, 2n+2, and 2n+1 define quadrilateral n. N/2-1 quadrilaterals are drawn. Note that the order in which vertices are used to construct a quadrilateral from strip data is different from that used with independent data. GL_POLYGON Draws a single, convex polygon. Vertices 1 through N define this polygon.

**Parameters:**
- `openglenum` (Integer) — The item you are going to draw, one of the constants from the comments, such as GL_LINES.
- `points` (Array<Geom::Point3d>)

**Returns:** `Sketchup::View` — 

[Docs](https://ruby.sketchup.com/Sketchup/View.html#draw-instance_method)

#### `draw2d(openglenum, points) => Sketchup::View | draw2d(openglenum, *points) => Sketchup::View | draw2d(openglenum, points, **options) => Sketchup::View | draw2d(openglenum, *points, **options) => Sketchup::View`

Note: Prior to SketchUp 2025.

**Remarks:** Note: Prior to SketchUp 2025.0 this method accepted the points as physical screen coordinates. As of SketchUp 2025.0 the points are expected to be in logical screen coordinates. Older versions need to apply the scaling factor from UI.scale_factor to the points before passing them to this method. The #draw2d method is used to draw in screen space (using 2D screen coordinates) instead of 3D space. The second parameter is an Array of Geom::Point3d objects (or several individual Geom::Point3d objects). These Geom::Point3d objects are in screen space, not 3D space. The X value corresponds to the number of pixels from the left edge of the drawing area. The Y value corresponds to the number of pixels down from the top of the drawing area. The Z value is not used.

**Parameters:**
- `openglenum` (Integer) — The item you are going to draw, one of the constants from the comments, such as GL_LINES.
- `points` (Array<Geom::Point3d>)

**Returns:** `Sketchup::View` — 

[Docs](https://ruby.sketchup.com/Sketchup/View.html#draw2d-instance_method)

#### `draw_lines(points, ...) => Sketchup::View | draw_lines(points) => Sketchup::View`

The draw_lines method is used to draw disconnected lines.

**Remarks:** The draw_lines method is used to draw disconnected lines. You must have an even number of points. This method is usually invoked within the draw method of a tool.

**Parameters:**
- `points` (Array<Geom::Point3d>) — An even number of Point3d objects.

**Returns:** `Sketchup::View` — 

[Docs](https://ruby.sketchup.com/Sketchup/View.html#draw_line-instance_method)

#### `draw_lines(points, ...) => Sketchup::View | draw_lines(points) => Sketchup::View`

The draw_lines method is used to draw disconnected lines.

**Remarks:** The draw_lines method is used to draw disconnected lines. You must have an even number of points. This method is usually invoked within the draw method of a tool.

**Parameters:**
- `points` (Array<Geom::Point3d>) — An even number of Point3d objects.

**Returns:** `Sketchup::View` — 

[Docs](https://ruby.sketchup.com/Sketchup/View.html#draw_lines-instance_method)

#### `draw_points(points, size = 6, style = 3, color = 'black') => Sketchup::View`

Note: Prior to SketchUp 2025.

**Remarks:** Note: Prior to SketchUp 2025.0 this method accepted the size as physical pixels. As of SketchUp 2025.0 the points are expected to be in logical pixels. Older versions need to apply the scaling factor from UI.scale_factor to the size before passing them to this method. This method is used to draw points in model space.

**Parameters:**
- `points` (Array<Geom::Point3d>) — Model coordinates.
- `size` (Integer) — Size of the point in pixels.
- `style` (Integer) — 1 = open square 2 = filled square 3 = plus shape “+” 4 = cross shape “X” 5 = star shape “*” 6 = open triangle 7 = filled triangle
- `color` (Sketchup::Color)

**Returns:** `Sketchup::View` — a View object

[Docs](https://ruby.sketchup.com/Sketchup/View.html#draw_points-instance_method)

#### `draw_polyline(points, ...) => Sketchup::View | draw_polyline(points) => Sketchup::View`

The draw_polyline method is used to draw a series of connected line segments from pt1 to pt2 to pt3, and so on.

**Remarks:** The draw_polyline method is used to draw a series of connected line segments from pt1 to pt2 to pt3, and so on. This method is usually invoked within the draw method of a tool.

**Parameters:**
- `points` (Array<Geom::Point3d>) — An even number of Point3d objects.

**Returns:** `Sketchup::View` — 

[Docs](https://ruby.sketchup.com/Sketchup/View.html#draw_polyline-instance_method)

#### `draw_text(point, text) => Sketchup::View | draw_text(point, text, options = {}) => Sketchup::View`

Note: Under Windows the font name must be less than 32 characters - due to system limitations.

**Remarks:** Note: Under Windows the font name must be less than 32 characters - due to system limitations. Note: As of SU2017 this will automatically scale the font-size by the same factor as UI.scale_factor. Note: The font size is platform dependent. On Windows the method expects points, where on Mac it's pixels. See “Cross Platform Font Size” example for details. As of SketchUp 2025.0 you can use the :pixel_size or :point_size options to specify the size in pixels or points respectively. This method is used to draw text on the screen and is usually invoked within the draw method of a tool. The TextVerticalAlignCenter option will align the text to the center of the height of the first line, not the whole boundingbox of the text. To align around the full bounds of the text, use #text_bounds to compute the desired alignment. The vertical alignment can vary between fonts and platforms. It's recommended to test different fonts and find one that fits well across both platforms for your purposes. Example of different vertical alignment and text bounds:

**Parameters:**
- `point` (Geom::Point3d) — A Point3d object representing a 2D coordinate in view space.
- `text` (String) — The text string to draw.

**Returns:** `Sketchup::View` — 

[Docs](https://ruby.sketchup.com/Sketchup/View.html#draw_text-instance_method)

#### `graphics_engine => Symbol`

The #graphics_engine method is used query the type of the graphics engine that's currently used by this view.

**Remarks:** The #graphics_engine method is used query the type of the graphics engine that's currently used by this view.

**Returns:** `Symbol` — Type of the graphics engine. :graphics_engine_classic or :graphics_engine_2024

[Docs](https://ruby.sketchup.com/Sketchup/View.html#graphics_engine-instance_method)

#### `guess_target => Geom::Point3d | guess_target(screen_point) => Geom::Point3d`

The guess_target method is used to guess at what the user is looking at when you have a perspective view.

**Remarks:** The guess_target method is used to guess at what the user is looking at when you have a perspective view.

**Parameters:**
- `screen_point` (Geom::Point3d)

**Returns:** `Geom::Point3d` — a Point3d object representing the point in the model that the user is likely interested in.

[Docs](https://ruby.sketchup.com/Sketchup/View.html#guess_target-instance_method)

#### `inference_locked? => Boolean`

The inference_locked? method is used to determine if inference locking is on for the view.

**Remarks:** The inference_locked? method is used to determine if inference locking is on for the view.

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Sketchup/View.html#inference_locked?-instance_method)

#### `inputpoint(x, y) => Sketchup::InputPoint | inputpoint(x, y, inputpoint1) => Sketchup::InputPoint | inputpoint(x, y) => Sketchup::InputPoint | inputpoint(x, y, inputpoint1) => Sketchup::InputPoint`

The #inputpoint method is used to retrieve an InputPoint.

**Remarks:** The #inputpoint method is used to retrieve an InputPoint. This will normally be used inside one of the mouse event handling methods in a tool. Usually, it is preferable to create the InputPoint first and then use the pick method on it.

**Parameters:**
- `x` (Integer) — Screen coordinate in physical pixels.
- `y` (Integer) — Screen coordinate in physical pixels.

**Returns:** `Sketchup::InputPoint` — 

[Docs](https://ruby.sketchup.com/Sketchup/View.html#inputpoint-instance_method)

#### `invalidate => Sketchup::View`

Note: This is the preferred method to update the viewport.

**Remarks:** Note: This is the preferred method to update the viewport. Use this before trying to use #refresh. The invalidate method is used mark the view as in need of a redraw.

**Returns:** `Sketchup::View` — the invalidated View object

[Docs](https://ruby.sketchup.com/Sketchup/View.html#invalidate-instance_method)

#### `last_refresh_time => Float | last_refresh_time(full) => Float`

The last_refresh_time method is used to retrieve the time for the last full view refresh.

**Remarks:** The last_refresh_time method is used to retrieve the time for the last full view refresh.

**Parameters:**
- `full` (Boolean)

**Returns:** `Float` — time in milliseconds

[Docs](https://ruby.sketchup.com/Sketchup/View.html#last_refresh_time-instance_method)

#### `load_texture(image_rep) => Integer`

Note: Avoid loading and releasing textures within the Tool#draw event as that is not efficient.

**Remarks:** Note: Avoid loading and releasing textures within the Tool#draw event as that is not efficient. Note: SketchUp 2020.0-2022.0: To conserve resources on the user's machine, textures can be loaded only when there is a Ruby tool on the tool stack. Make sure to release the texture when it's no longer needed. Any textures not already released when the last Ruby tool on the tool stack is removed will be automatically released by SketchUp. As of SketchUp 2023.0 this automatic cleanup was removed to allow Overlays to draw textures. Loads a texture to be drawn with #draw or #draw2d.

**Parameters:**
- `image_rep` (Sketchup::ImageRep)

**Returns:** `Integer` — A resource ID referring to the image loaded.

[Docs](https://ruby.sketchup.com/Sketchup/View.html#load_texture-instance_method)

#### `lock_inference => Sketchup::View | lock_inference(inputpoint) => Sketchup::View | lock_inference(inputpoint, inputpoint2) => Sketchup::View`

The #lock_inference method is used to lock or unlock an inference.

**Remarks:** The #lock_inference method is used to lock or unlock an inference. This method will typically be called from inside a tool class when the user presses the shift key or arrow keys. With no arguments it unlocks all inferences. With one argument it locks inference based on that passed InputPoint's entities, e.g. along a Edge's line or a Face's plane. With two arguments, it locks inference along an axis.

**Parameters:**
- `inputpoint` (Sketchup::InputPoint)

**Returns:** `Sketchup::View` — a View object

[Docs](https://ruby.sketchup.com/Sketchup/View.html#lock_inference-instance_method)

#### `model => Sketchup::Model`

The model method is used to retrieve the model for the current view.

**Remarks:** The model method is used to retrieve the model for the current view.

**Returns:** `Sketchup::Model` — the model for this view

[Docs](https://ruby.sketchup.com/Sketchup/View.html#model-instance_method)

#### `pick_helper => Sketchup::PickHelper | pick_helper(x, y, aperture = 0) => Sketchup::PickHelper | pick_helper(x, y, aperture = 0.0) => Sketchup::PickHelper`

The #pick_helper method is used to retrieve a pick helper for the view.

**Remarks:** The #pick_helper method is used to retrieve a pick helper for the view.

**Parameters:**
- `x` (Integer) — Screen coordinate in physical pixels.
- `y` (Integer) — Screen coordinate in physical pixels.
- `aperture` (Integer) — The size of the aperture in physical pixels.

**Returns:** `Sketchup::PickHelper` — 

[Docs](https://ruby.sketchup.com/Sketchup/View.html#pick_helper-instance_method)

#### `pickray(screen_point) => Array(Geom::Point3d, Geom::Vector3d) | pickray(x, y) => Array(Geom::Point3d, Geom::Vector3d) | pickray(screen_point) => Array(Geom::Point3d, Geom::Vector3d) | pickray(x, y) => Array(Geom::Point3d, Geom::Vector3d)`

The #pickray method is used to retrieve a ray passing through a given screen position in the viewing direction.

**Remarks:** The #pickray method is used to retrieve a ray passing through a given screen position in the viewing direction.

**Parameters:**
- `screen_point` (Array(Integer, Integer)) — Screen coordinates in physical pixels.

**Returns:** `Array(Geom::Point3d, Geom::Vector3d)` — a ray

[Docs](https://ruby.sketchup.com/Sketchup/View.html#pickray-instance_method)

#### `pixels_to_model(pixels, point) => Float`

Note: As of SU2017 this will automatically scale the pixel-size by the same factor as UI.

**Remarks:** Note: As of SU2017 this will automatically scale the pixel-size by the same factor as UI.scale_factor. The #pixels_to_model method is used to compute a model size from a pixel size at a given point. This method is useful for deciding how big to draw something based on a desired size in pixels.

**Parameters:**
- `pixels` (Numeric) — Logical pixels since SketchUp 2017.
- `point` (Geom::Point3d) — A model point where the size will be calculated from.

**Returns:** `Float` — the model size

[Docs](https://ruby.sketchup.com/Sketchup/View.html#pixels_to_model-instance_method)

#### `refresh => Sketchup::View`

Note: This method might impact performance and if used incorrectly cause instability or crashes.

**Remarks:** Note: This method might impact performance and if used incorrectly cause instability or crashes. Don't use this unless you have verified that you cannot use #invalidate instead. The refresh method is used to immediately force a redraw of the view.

**Returns:** `Sketchup::View` — the refreshed View object

[Docs](https://ruby.sketchup.com/Sketchup/View.html#refresh-instance_method)

#### `release_texture(texture_id) => Boolean`

Releases a texture loaded via #load_texture, freeing up it's memory.

**Remarks:** Releases a texture loaded via #load_texture, freeing up it's memory. It's good practice to do so whenever there is no longer any need for the resource. For example, when your tool deactivates you probably want to release your resources as you don't know if your tool will be used again.

**Parameters:**
- `texture_id` (Integer)

**Returns:** `Boolean` — true if texture was released. false otherwise.

[Docs](https://ruby.sketchup.com/Sketchup/View.html#release_texture-instance_method)

#### `remove_observer(observer) => Boolean`

The remove_observer method is used to remove an observer from the current object.

**Remarks:** The remove_observer method is used to remove an observer from the current object.

**Parameters:**
- `observer` (Object) — An observer.

**Returns:** `Boolean` — true if successful, false if unsuccessful.

[Docs](https://ruby.sketchup.com/Sketchup/View.html#remove_observer-instance_method)

#### `screen_coords(model_point) => Geom::Point3d | screen_coords(model_point) => Geom::Point3d`

Note: Prior to SketchUp 2025.

**Remarks:** Note: Prior to SketchUp 2025.0 this method returned the points as physical screen coordinates. As of SketchUp 2025.0 the points are returned in logical screen coordinates. The #screen_coords method is used to retrieve the screen coordinates of the given point on the screen. The x and y values returned correspond to the x and y screen coordinates. Ignore the z values. If the referenced point is not in the current viewport, the x and/or y value may be negative.

**Parameters:**
- `model_point` (Geom::Point3d) — Model coordinate.

**Returns:** `Geom::Point3d` — Screen coordinate in pixels (physical prior to SketchUp 2025.0, logical from 2025.0).

[Docs](https://ruby.sketchup.com/Sketchup/View.html#screen_coords-instance_method)

#### `set_color_from_line(point1, point2) => Sketchup::View`

Set the drawing color for the view based on the direction of a line that you want to draw.

**Remarks:** Set the drawing color for the view based on the direction of a line that you want to draw. These colors will match the axes colors in the SketchUp model (typically blue for straight up and down, etc.) This method is usually invoked within the draw method of a tool.

**Parameters:**
- `point1` (Geom::Point3d) — Point3d object representing first point in the line.
- `point2` (Geom::Point3d) — Point3d object representing the second point in the line.

**Returns:** `Sketchup::View` — a View object

[Docs](https://ruby.sketchup.com/Sketchup/View.html#set_color_from_line-instance_method)

#### `show_frame(delay) => Sketchup::View`

The show_frame method is used to show a frame of an Animation object in the current view.

**Remarks:** The show_frame method is used to show a frame of an Animation object in the current view. You can supply an optional delay in seconds to wait before showing the next frame. This can be useful to control the speed at which the animation runs.

**Parameters:**
- `delay` (Numeric) — An optional delay in seconds.

**Returns:** `Sketchup::View` — 

[Docs](https://ruby.sketchup.com/Sketchup/View.html#show_frame-instance_method)

#### `text_bounds(point, text, options = {}) => Geom::Bounds2d`

Note: Under Windows the font name must be less than 32 characters - due to system limitations.

**Remarks:** Note: Under Windows the font name must be less than 32 characters - due to system limitations. This method is used to compute the bounds of the text when using #draw_text. The bounds are not a tight fit around the top and bottom as they include varying amount of line spacing depending on the font used. The TextVerticalAlignCenter option will align the text to the center of the height of the first line, not the whole boundingbox of the text. Example of different vertical alignment and text bounds:

**Parameters:**
- `point` (Geom::Point3d) — A Point3d object representing a 2D coordinate in view space.
- `text` (String) — The text string to draw.
- `options` (Hash) — The text can be customized by providing a hash or named arguments of options.

**Returns:** `Geom::Bounds2d` — 

[Docs](https://ruby.sketchup.com/Sketchup/View.html#text_bounds-instance_method)

#### `vpheight => Integer | vpheight => Float`

Note: Prior to SketchUp 2025.

**Remarks:** Note: Prior to SketchUp 2025.0 this method returned the size as physical screen coordinates. As of SketchUp 2025.0 the size are returned in logical screen coordinates. The #vpheight method is used to retrieve the height of the viewport for the view.

**Returns:** `Integer` — the height of the viewport in physical pixels.

[Docs](https://ruby.sketchup.com/Sketchup/View.html#vpheight-instance_method)

#### `vpwidth => Integer | vpwidth => Float`

Note: Prior to SketchUp 2025.

**Remarks:** Note: Prior to SketchUp 2025.0 this method returned the size as physical screen coordinates. As of SketchUp 2025.0 the size are returned in logical screen coordinates. The #vpwidth method is used to retrieve the width of the viewport for the view.

**Returns:** `Integer` — the width of the viewport in physical pixels.

[Docs](https://ruby.sketchup.com/Sketchup/View.html#vpwidth-instance_method)

#### `write_image(filename, width = view.vpwidth, height = view.vpheight, antialias = false, compression = 1.0) => Boolean | write_image(options) => Boolean | write_image(options) => Boolean`

The #write_image method is used to write the current view to an image file.

**Remarks:** The #write_image method is used to write the current view to an image file. Supported file types are .png, .jpg, .jpeg, gif, .bmp, .tif. For other file formats available from the GUI in File > Export > 2D Graphics, .e.g .pdf, use Model#export.

**Parameters:**
- `filename` (String) — The filename for the saved image
- `width` (Integer) — Width in pixels, defaults to the current viewport width #vpwidth.
- `height` (Integer) — Height in pixels, defaults to the current viewport height #vpheight.
- `antialias` (Boolean)
- `compression` (Float) — Compression factor for JPEG images, between 0.0 and 1.0.

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Sketchup/View.html#write_image-instance_method)

#### `zoom(zoom_or_ents) => Sketchup::View`

The zoom method is used to zoom in or out by some zoom factor.

**Remarks:** The zoom method is used to zoom in or out by some zoom factor.

**Parameters:**
- `zoom_or_ents` (Numeric, Sketchup::Selection, Sketchup::Entity, Array<Sketchup::Entity>) — A Float zoom factor from 1.0 or larger or an Array or collection of entities to “zoom extents” around.

**Returns:** `Sketchup::View` — the zoomed View object

[Docs](https://ruby.sketchup.com/Sketchup/View.html#zoom-instance_method)

#### `zoom_extents => Sketchup::View`

The zoom_extents method is used to zoom to the extents about the entire model, as if the user has selected the zoom extents command from the menu.

**Remarks:** The zoom_extents method is used to zoom to the extents about the entire model, as if the user has selected the zoom extents command from the menu.

**Returns:** `Sketchup::View` — the zoomed View object

[Docs](https://ruby.sketchup.com/Sketchup/View.html#zoom_extents-instance_method)

### Properties
- `animation` (#nextFrame, set) — The #animation= method is used to set an animation that is displayed for a view.
- `camera` (Sketchup::Camera, get/set) — The camera method is used to retrieve the camera for the view.
- `drawing_color` (Sketchup::Color, String, set) — The drawing_color method is used to set the color that is used for drawing to the view.
- `dynamic` (Boolean, set) — Deprecated.
- `field_of_view` (Float, get/set) — The field_of_view method is used get the view's field of view setting, in degrees.
- `line_stipple` (String, set) — The line_stipple= method is used to set the line pattern to use for drawing.
- `line_width` (Numeric, set) — Note: As of SU2017 this will automatically scale the line width by the same factor as UI.
- `tooltip` (String, set) — Set a tooltip to display in the view.
- `CORNER_TOP_LEFT` (Object, get) — 
- `CORNER_TOP_RIGHT` (Object, get) — 
- `CORNER_BOTTOM_LEFT` (Object, get) — 
- `CORNER_BOTTOM_RIGHT` (Object, get) — 

## ViewObserver (class)

This class is abstract. To implement this observer, create a Ruby class of this type, override the desired methods, and add an instance of the observer to the View object. This observer interface is implemented to react to View events.

**Remarks:** See Also: Sketchup::View#add_observer

[Vendor docs](https://ruby.sketchup.com/Sketchup/ViewObserver.html)

### Methods
#### `onScaleFactorChange(view) => nil`

The #onScaleFactorChange method is called whenever the view DPI of the view changes.

**Remarks:** The #onScaleFactorChange method is called whenever the view DPI of the view changes. This can be the SketchUp window being moved to another monitor with a different DPI or the user changing the DPI settings of the monitor.

**Parameters:**
- `view` (Sketchup::View)

**Returns:** `nil` — 

[Docs](https://ruby.sketchup.com/Sketchup/ViewObserver.html#onScaleFactorChange-instance_method)

#### `onViewChanged(view) => nil`

The #onViewChanged method is called whenever the view is altered, such as when the user uses the Pan, Orbit, or Zoom tools.

**Remarks:** The #onViewChanged method is called whenever the view is altered, such as when the user uses the Pan, Orbit, or Zoom tools.

**Parameters:**
- `view` (Sketchup::View)

**Returns:** `nil` — 

[Docs](https://ruby.sketchup.com/Sketchup/ViewObserver.html#onViewChanged-instance_method)

