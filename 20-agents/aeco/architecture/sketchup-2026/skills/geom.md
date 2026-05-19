---
name: sketchup-geom
description: This skill encodes the sketchup 2026.0 surface of the Geom namespace — 12 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: BoundingBox, Bounds2d, LatLong, OrientedBounds2d, Point2d, Transformation2d, PolygonMesh, Transformation, and 4 more types.
---

# Geom

Auto-generated from vendor docs for sketchup 2026.0. 12 types in this namespace.

## BoundingBox (class)

Note: that the bounding box returned for face-me components is the center of its entire range of motion. This behavior changed in SketchUp 7.1. In 7.0 and earlier, the .bounds method would return the bounds around the face-me component's current, visible center. Bounding boxes are three-dimensional boxes (eight corners), aligned with the axes, that surround entities within your model. There is a default bounding box for any new model that will surround all entities, including all groups and components. Additionally, there are bounding boxes for Drawingelement objects, including components and groups. Bounding boxes are only large enough to exactly bound the entities within your model, group, or component. You can also create arbitrary BoundingBox objects by calling BoundingBox.new.

[Vendor docs](https://ruby.sketchup.com/Geom/BoundingBox.html)

### Constructors
- `BoundingBox()` — The new method is used to create a new, empty, bounding box.

### Methods
#### `add(point_or_bb) => Geom::BoundingBox | add(points_or_bb) => Geom::BoundingBox`

The add method is used to add a point, vertex, or other bounding boxes to the bounding box.

**Remarks:** The add method is used to add a point, vertex, or other bounding boxes to the bounding box. The size of the bounding box will increase as necessary to accommodate the new items. Adding one point to an empty bounding box does not increase the size of the bounding box. You must add at least two points before methods such as BoundingBox.diagonal will return a size greater than zero.

**Parameters:**
- `point_or_bb` (Geom::Point3d, Geom::BoundingBox, Sketchup::Vertex)

**Returns:** `Geom::BoundingBox` — 

[Docs](https://ruby.sketchup.com/Geom/BoundingBox.html#add-instance_method)

#### `center => Geom::Point3d`

The center method is used to retrieve the Point3d object at the center of the bounding box.

**Remarks:** The center method is used to retrieve the Point3d object at the center of the bounding box.

**Returns:** `Geom::Point3d` — the Point3d at the center of the bounding box if successful

[Docs](https://ruby.sketchup.com/Geom/BoundingBox.html#center-instance_method)

#### `clear => Geom::BoundingBox`

The clear method is used to clear a bounding box.

**Remarks:** The clear method is used to clear a bounding box. A cleared BoundingBox does not have a size greater than zero until you add at least two points or another bounding box.

**Returns:** `Geom::BoundingBox` — the BoundingBox object which was cleared

[Docs](https://ruby.sketchup.com/Geom/BoundingBox.html#clear-instance_method)

#### `contains?(point_or_bb) => Boolean`

This method is used to determine if a bounding box contains a specific Point3d or BoundingBox object.

**Remarks:** This method is used to determine if a bounding box contains a specific Point3d or BoundingBox object.

**Parameters:**
- `point_or_bb` (Geom::Point3d, Geom::BoundingBox)

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Geom/BoundingBox.html#contains?-instance_method)

#### `corner(corner_index) => Geom::Point3d`

The corner method is used to retrieve a point object at a specified corner of the bounding box.

**Remarks:** The corner method is used to retrieve a point object at a specified corner of the bounding box. There are 8 corners to a bounding box, identified by the numbers 0 through 7. Points are returned in the currently set units (inches, by default). These are which index refers to which corner:

**Parameters:**
- `corner_index` (Integer) — A number (from 0 to 7) representing point at the corner you want to retrieve.

**Returns:** `Geom::Point3d` — a Point3d object if successful

[Docs](https://ruby.sketchup.com/Geom/BoundingBox.html#corner-instance_method)

#### `depth => Length`

Note: In SketchUp's coordinate system, this corresponds to the height.

**Remarks:** Note: In SketchUp's coordinate system, this corresponds to the height. The #depth method is used to retrieve the Z extents of the bounding box.

**Returns:** `Length` — 

[Docs](https://ruby.sketchup.com/Geom/BoundingBox.html#depth-instance_method)

#### `diagonal => Length`

The #diagonal method is used to get the length of the diagonal of the bounding box.

**Remarks:** The #diagonal method is used to get the length of the diagonal of the bounding box.

**Returns:** `Length` — the size of the diagonal for the bounding box

[Docs](https://ruby.sketchup.com/Geom/BoundingBox.html#diagonal-instance_method)

#### `empty? => Boolean`

The empty? method is used to determine if a bounding box is empty (such as if the bounds have not been set.

**Remarks:** The empty? method is used to determine if a bounding box is empty (such as if the bounds have not been set.) This returns the opposite of the valid? method.

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Geom/BoundingBox.html#empty?-instance_method)

#### `height => Length`

Note: In SketchUp's coordinate system, this corersponds to the depth.

**Remarks:** Note: In SketchUp's coordinate system, this corersponds to the depth. The #height method is used to retrieve the Y extent of the bounding box.

**Returns:** `Length` — 

[Docs](https://ruby.sketchup.com/Geom/BoundingBox.html#height-instance_method)

#### `intersect(boundingbox) => Geom::BoundingBox`

Note: Prior to SU2015 this method would return incorrect result in some cases.

**Remarks:** Note: Prior to SU2015 this method would return incorrect result in some cases. For correct result in these versions you must first check if the boundingboxes actually overlap - then call this to get the resulting boundingbox. The intersect method is used to retrieve a bounding box that is the result of intersecting one bounding box with another.

**Parameters:**
- `boundingbox` (Geom::BoundingBox) — A second boundbox which might intersect boundingbox1.

**Returns:** `Geom::BoundingBox` — the resulting BoundingBox object if successful, an empty BoundingBox object if unsuccessful.

[Docs](https://ruby.sketchup.com/Geom/BoundingBox.html#intersect-instance_method)

#### `max => Geom::Point3d`

The max method is used to retrieve the Point3d object where x, y and z are maximum in the bounding box.

**Remarks:** The max method is used to retrieve the Point3d object where x, y and z are maximum in the bounding box. If you attempt to call the max method on an empty bounding box, you will receive a very large negative number.

**Returns:** `Geom::Point3d` — a Point3d object representing the point where x, y, and z are the maximum in the bounding box.

[Docs](https://ruby.sketchup.com/Geom/BoundingBox.html#max-instance_method)

#### `min => Geom::Point3d`

The min method is used to retrieve the Point3d where x, y and z are minimum in the bounding box.

**Remarks:** The min method is used to retrieve the Point3d where x, y and z are minimum in the bounding box.

**Returns:** `Geom::Point3d` — a Point3d object representing the point where x, y, and z are the maximum in the bounding box.

[Docs](https://ruby.sketchup.com/Geom/BoundingBox.html#min-instance_method)

#### `valid? => Boolean`

The valid method is used to determine if a bounding box is valid (contains points).

**Remarks:** The valid method is used to determine if a bounding box is valid (contains points).

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Geom/BoundingBox.html#valid?-instance_method)

#### `width => Length`

The #width method is used to retrieve the X extent of the bounding box.

**Remarks:** The #width method is used to retrieve the X extent of the bounding box.

**Returns:** `Length` — 

[Docs](https://ruby.sketchup.com/Geom/BoundingBox.html#width-instance_method)

## Bounds2d (class)

The bounds2d class represents an axis aligned bounding box represented by two Point2d objects, upper left and lower right positions. The units utilized in the creation and modification Bounds2d are inches.

[Vendor docs](https://ruby.sketchup.com/Geom/Bounds2d.html)

### Constructors
- `Bounds2d(other_bounds)` — The #initialize method creates a new Geom::Bounds2d.

### Methods
#### `height => Float`

The #height method returns the height of the Geom::Bounds2d.

**Remarks:** The #height method returns the height of the Geom::Bounds2d.

**Returns:** `Float` — 

[Docs](https://ruby.sketchup.com/Geom/Bounds2d.html#height-instance_method)

#### `lower_right => Geom::Point2d`

The #lower_right method returns the Point2d of the lower right corner of the Geom::Bounds2d.

**Remarks:** The #lower_right method returns the Point2d of the lower right corner of the Geom::Bounds2d.

**Returns:** `Geom::Point2d` — 

[Docs](https://ruby.sketchup.com/Geom/Bounds2d.html#lower_right-instance_method)

#### `set!(other_bounds) => Geom::Bounds2d | set!(upper_left, lower_right) => Geom::Bounds2d | set!(x, y, width, height) => Geom::Bounds2d | set!(point_array) => Geom::Bounds2d | set!(float_array) => Geom::Bounds2d`

The #set! method is used to update the dimensions and position of a Geom::Bounds2d object so that it matches the specified bounds.

**Remarks:** The #set! method is used to update the dimensions and position of a Geom::Bounds2d object so that it matches the specified bounds. The argument is anything that can be converted into a Geom::Bounds2d.

**Parameters:**
- `other_bounds` (Geom::Bounds2d)

**Returns:** `Geom::Bounds2d` — 

[Docs](https://ruby.sketchup.com/Geom/Bounds2d.html#set!-instance_method)

#### `to_a => Array(Geom::Point2d, Geom::Point2d)`

The #to_a method returns an array which contains the Point2ds that define the Geom::Bounds2d.

**Remarks:** The #to_a method returns an array which contains the Point2ds that define the Geom::Bounds2d.

**Returns:** `Array(Geom::Point2d, Geom::Point2d)` — 

[Docs](https://ruby.sketchup.com/Geom/Bounds2d.html#to_a-instance_method)

#### `upper_left => Geom::Point2d`

The #upper_left method returns the Point2d of the upper left corner of the Geom::Bounds2d.

**Remarks:** The #upper_left method returns the Point2d of the upper left corner of the Geom::Bounds2d.

**Returns:** `Geom::Point2d` — 

[Docs](https://ruby.sketchup.com/Geom/Bounds2d.html#upper_left-instance_method)

#### `width => Float`

The #width method returns the width of the Geom::Bounds2d.

**Remarks:** The #width method returns the width of the Geom::Bounds2d.

**Returns:** `Float` — 

[Docs](https://ruby.sketchup.com/Geom/Bounds2d.html#width-instance_method)

### Properties
- `=` (Geom::Bounds2d, set) — The #== method checks to see if the two Geom::Bounds2ds are equal.

## LatLong (class)

The LatLong class contains various methods for creating and manipulating latitude and longitude coordinates.

[Vendor docs](https://ruby.sketchup.com/Geom/LatLong.html)

### Constructors
- `LatLong(latlong)` — The new method creates a LatLong object.

### Methods
#### `latitude => Float`

The Latitude method retrieves the latitude coordinate from a LatLong object.

**Remarks:** The Latitude method retrieves the latitude coordinate from a LatLong object.

**Returns:** `Float` — a latitude coordinate value

[Docs](https://ruby.sketchup.com/Geom/LatLong.html#latitude-instance_method)

#### `longitude => Float`

The Latitude method retrieves the longitude coordinate from a LatLong object.

**Remarks:** The Latitude method retrieves the longitude coordinate from a LatLong object.

**Returns:** `Float` — a latitude coordinate value

[Docs](https://ruby.sketchup.com/Geom/LatLong.html#longitude-instance_method)

#### `to_a => Array(Float, Float)`

The #to_a method converts a LatLong object to an array of two values.

**Remarks:** The #to_a method converts a LatLong object to an array of two values.

**Returns:** `Array(Float, Float)` — an array of latitude and longitude

[Docs](https://ruby.sketchup.com/Geom/LatLong.html#to_a-instance_method)

#### `to_s => String`

The #to_s method converts a LatLong object to a String.

**Remarks:** The #to_s method converts a LatLong object to a String.

**Returns:** `String` — 

[Docs](https://ruby.sketchup.com/Geom/LatLong.html#to_s-instance_method)

#### `to_utm => Geom::UTM`

The to_utm method converts a LatLong object to a UTM object.

**Remarks:** The to_utm method converts a LatLong object to a UTM object.

**Returns:** `Geom::UTM` — 

[Docs](https://ruby.sketchup.com/Geom/LatLong.html#to_utm-instance_method)

## OrientedBounds2d (class)

The OrientedBounds2d class is a bounding box represented by four Point2d objects, upper left, upper right, lower left and lower right positions.

[Vendor docs](https://ruby.sketchup.com/Geom/OrientedBounds2d.html)

### Methods
#### `lower_left => Geom::Point2d`

The #lower_left method returns the Point2d of the lower left corner of the Geom::OrientedBounds2d.

**Remarks:** The #lower_left method returns the Point2d of the lower left corner of the Geom::OrientedBounds2d.

**Returns:** `Geom::Point2d` — 

[Docs](https://ruby.sketchup.com/Geom/OrientedBounds2d.html#lower_left-instance_method)

#### `lower_right => Geom::Point2d`

The #lower_right method returns the Point2d of the lower right corner of the Geom::OrientedBounds2d.

**Remarks:** The #lower_right method returns the Point2d of the lower right corner of the Geom::OrientedBounds2d.

**Returns:** `Geom::Point2d` — 

[Docs](https://ruby.sketchup.com/Geom/OrientedBounds2d.html#lower_right-instance_method)

#### `to_a => Array(Geom::Point2d, Geom::Point2d, Geom::Point2d, Geom::Point2d)`

The #to_a method returns an array which contains the Point2d that define the Geom::OrientedBounds2d.

**Remarks:** The #to_a method returns an array which contains the Point2d that define the Geom::OrientedBounds2d.

**Returns:** `Array(Geom::Point2d, Geom::Point2d, Geom::Point2d, Geom::Point2d)` — 

[Docs](https://ruby.sketchup.com/Geom/OrientedBounds2d.html#to_a-instance_method)

#### `upper_left => Geom::Point2d`

The #upper_left method returns the Point2d of the upper left corner of the Geom::OrientedBounds2d.

**Remarks:** The #upper_left method returns the Point2d of the upper left corner of the Geom::OrientedBounds2d.

**Returns:** `Geom::Point2d` — 

[Docs](https://ruby.sketchup.com/Geom/OrientedBounds2d.html#upper_left-instance_method)

#### `upper_right => Geom::Point2d`

The #upper_right method returns the Point2d of the upper right corner of the Geom::OrientedBounds2d.

**Remarks:** The #upper_right method returns the Point2d of the upper right corner of the Geom::OrientedBounds2d.

**Returns:** `Geom::Point2d` — 

[Docs](https://ruby.sketchup.com/Geom/OrientedBounds2d.html#upper_right-instance_method)

### Properties
- `=` (Geom::OrientedBounds2d, set) — The #== method checks to see if the two Geom::OrientedBounds2ds are equal.

## Point2d (class)

The Point2d class allows you to work with a point in 2D space. Point2d is a series of values representing x and y coordinates. The values are specified as [x, y]. For example [1, 1]. To create a point call Geom::Point2d.new, where the creation method can take a variety of arguments:

[Vendor docs](https://ruby.sketchup.com/Geom/Point2d.html)

### Constructors
- `Point2d(x, y)` — The new method creates a new Geom::Point2d.

### Methods
#### `+(vector) => Geom::Point2d`

The #+ operator is a simple way to add to the current x and y values of the Geom::Point2d, or to set the values of the Geom::Point2d by adding a Vector2d to the Geom::Point2d.

**Remarks:** The #+ operator is a simple way to add to the current x and y values of the Geom::Point2d, or to set the values of the Geom::Point2d by adding a Vector2d to the Geom::Point2d.

**Parameters:**
- `vector` (Geom::Vector2d, Array(Numeric, Numeric))

**Returns:** `Geom::Point2d` — 

[Docs](https://ruby.sketchup.com/Geom/Point2d.html#+-instance_method)

#### `-(array2d) => Geom::Vector2d | -(point2d) => Geom::Vector2d | -(vector2d) => Geom::Point2d`

The #- operator is a simple way to subtract from the current x and y values of the Geom::Point2d.

**Remarks:** The #- operator is a simple way to subtract from the current x and y values of the Geom::Point2d.

**Parameters:**
- `array2d` (Array(Numeric, Numeric))

**Returns:** `Geom::Vector2d` — 

[Docs](https://ruby.sketchup.com/Geom/Point2d.html#--instance_method)

#### `clone => Geom::Point2d`

The #clone method creates another point identical to the Geom::Point2d being cloned.

**Remarks:** The #clone method creates another point identical to the Geom::Point2d being cloned.

**Returns:** `Geom::Point2d` — the cloned Geom::Point2d object

[Docs](https://ruby.sketchup.com/Geom/Point2d.html#clone-instance_method)

#### `distance(point) => Numeric`

The #distance method computes the distance from the Geom::Point2d to another Geom::Point2d.

**Remarks:** The #distance method computes the distance from the Geom::Point2d to another Geom::Point2d.

**Parameters:**
- `point` (Geom::Point2d, Array(Numeric, Numeric))

**Returns:** `Numeric` — the distance between the points in the current units

[Docs](https://ruby.sketchup.com/Geom/Point2d.html#distance-instance_method)

#### `inspect => String`

The #inspect method formats the Geom::Point2d as a string.

**Remarks:** The #inspect method formats the Geom::Point2d as a string.

**Returns:** `String` — 

[Docs](https://ruby.sketchup.com/Geom/Point2d.html#inspect-instance_method)

#### `offset(vector) => Geom::Point2d | offset(vector, distance) => Geom::Point2d`

The #offset method offsets the Geom::Point2d by a Vector2d and returns a new Geom::Point2d.

**Remarks:** The #offset method offsets the Geom::Point2d by a Vector2d and returns a new Geom::Point2d. If distance is provided, it must be non-zero.

**Parameters:**
- `vector` (Geom::Vector2d, Array(Numeric, Numeric))

**Returns:** `Geom::Point2d` — 

[Docs](https://ruby.sketchup.com/Geom/Point2d.html#offset-instance_method)

#### `offset!(vector) => Geom::Point2d | offset!(vector, distance) => Geom::Point2d`

The #offset! method offsets the Geom::Point2d by a Vector2d.

**Remarks:** The #offset! method offsets the Geom::Point2d by a Vector2d. The Geom::Point2d itself is modified. The length of the vector must not be zero.

**Parameters:**
- `vector` (Geom::Vector2d, Array(Numeric, Numeric))

**Returns:** `Geom::Point2d` — 

[Docs](https://ruby.sketchup.com/Geom/Point2d.html#offset!-instance_method)

#### `set!(point) => Geom::Point2d | set!(x, y) => Geom::Point2d`

The #set! method sets the values of the Geom::Point2d.

**Remarks:** The #set! method sets the values of the Geom::Point2d.

**Parameters:**
- `point` (Geom::Point2d, Array(Numeric, Numeric))

**Returns:** `Geom::Point2d` — 

[Docs](https://ruby.sketchup.com/Geom/Point2d.html#set!-instance_method)

#### `to_a => Array(Numeric, Numeric)`

The #to_a method converts the Geom::Point2d to an array of 2 numbers.

**Remarks:** The #to_a method converts the Geom::Point2d to an array of 2 numbers.

**Returns:** `Array(Numeric, Numeric)` — an array of two numbers representing x, y of the Geom::Point2d

[Docs](https://ruby.sketchup.com/Geom/Point2d.html#to_a-instance_method)

#### `to_s => String`

The #to_s method returns a string representation of the Geom::Point2d.

**Remarks:** The #to_s method returns a string representation of the Geom::Point2d.

**Returns:** `String` — 

[Docs](https://ruby.sketchup.com/Geom/Point2d.html#to_s-instance_method)

#### `transform(transform) => Geom::Point2d`

The #transform method applies a transformation to a point, returning a new point.

**Remarks:** The #transform method applies a transformation to a point, returning a new point. The original point is unchanged by this method.

**Parameters:**
- `transform` (Geom::Transformation2d) — A Transformation object to apply to the point.

**Returns:** `Geom::Point2d` — the transformed point

[Docs](https://ruby.sketchup.com/Geom/Point2d.html#transform-instance_method)

#### `transform!(transform) => Geom::Point2d`

The #transform! method applies a transformation to a point.

**Remarks:** The #transform! method applies a transformation to a point. The point itself is modified.

**Parameters:**
- `transform` (Geom::Transformation2d) — A Transformation object to apply to the point.

**Returns:** `Geom::Point2d` — the transformed point

[Docs](https://ruby.sketchup.com/Geom/Point2d.html#transform!-instance_method)

#### `vector_to(point) => Geom::Vector2d`

The #vector_to method returns the vector between points.

**Remarks:** The #vector_to method returns the vector between points.

**Parameters:**
- `point` (Geom::Point2d, Array(Numeric, Numeric))

**Returns:** `Geom::Vector2d` — 

[Docs](https://ruby.sketchup.com/Geom/Point2d.html#vector_to-instance_method)

### Properties
- `=` (Geom::Point2d, Array(Numeric, Numeric), set) — The #== method compares two points for equality.
- `[]` (Length, get/set) — The #[] method returns the value of the Geom::Point2d at the specified index.
- `x` (Length, get/set) — The #x method returns the x value of the Geom::Point2d.
- `y` (Length, get/set) — The #y method returns the y value of the Geom::Point2d.

## Point3d (class)

The Point3d class allows you to work with a point in 3D space. The point is basically just a series of values representing x, y and z coordinates. The values are specified as [x,y,z]. For example [100,200,300]. To create a point call Geom::Point3d.new, where the creation method can take a variety of arguments: In addition to the methods below, there are a series of geometry related methods that are on the Array class, since Point3d objects can also be represented as a 3-element Array. These Array-level methods are for operations such as determining if a point is on a line, on a plane, etc. See the Array class for details.

[Vendor docs](https://ruby.sketchup.com/Geom/Point3d.html)

### Constructors
- `Point3d(x, y, z = 0.0)` — The new method is used to create a new 3D point.

### Methods
#### `+(vector) => Geom::Point3d`

The #+ operator is a fast way to add to the current x, y and z values of a vector.

**Remarks:** The #+ operator is a fast way to add to the current x, y and z values of a vector.

**Parameters:**
- `vector` (Geom::Vector3d, Array(Numeric, Numeric, Numeric))

**Returns:** `Geom::Point3d` — 

[Docs](https://ruby.sketchup.com/Geom/Point3d.html#+-instance_method)

#### `-(array3d) => Geom::Vector3d | -(point3d) => Geom::Vector3d | -(vector3d) => Geom::Point3d`

The '-' operator is a fast way to subtract from the current x, y and z values of a point.

**Remarks:** The '-' operator is a fast way to subtract from the current x, y and z values of a point.

**Parameters:**
- `array3d` (Array(Numeric, Numeric, Numeric))

**Returns:** `Geom::Vector3d` — 

[Docs](https://ruby.sketchup.com/Geom/Point3d.html#--instance_method)

#### `<(point2) => Boolean`

Note: The comparison is performed in the order x, y and z coordinates.

**Remarks:** Note: The comparison is performed in the order x, y and z coordinates. The #< compare method is used to compare two points to determine if the left-hand point is less than the right-hand point.

**Parameters:**
- `point2` (Geom::Point3d, Array(Numeric, Numeric, Numeric)) — A Point3d object.

**Returns:** `Boolean` — true if the point1 is smaller than point2

[Docs](https://ruby.sketchup.com/Geom/Point3d.html#<-instance_method)

#### `clone => Geom::Point3d`

The clone method is used to create another point identical to the point being cloned.

**Remarks:** The clone method is used to create another point identical to the point being cloned.

**Returns:** `Geom::Point3d` — the cloned Point3d object

[Docs](https://ruby.sketchup.com/Geom/Point3d.html#clone-instance_method)

#### `distance(point2) => Length`

The distance method is used to compute the distance from a point to another point.

**Remarks:** The distance method is used to compute the distance from a point to another point.

**Parameters:**
- `point2` (Geom::Point3d) — The Point3d object to compute the distance to.

**Returns:** `Length` — 

[Docs](https://ruby.sketchup.com/Geom/Point3d.html#distance-instance_method)

#### `distance_to_line(line) => Float`

Note: This function returns a `Float` value, not a `Length`.

**Remarks:** Note: This function returns a `Float` value, not a `Length`. The distance_to_line method is used to compute the distance from a point to a line. See Geom module for how to specify a line.

**Parameters:**
- `line` (Object) — A line (see Geom for information on creating lines).

**Returns:** `Float` — the distance between a point and line in internal units if successful

[Docs](https://ruby.sketchup.com/Geom/Point3d.html#distance_to_line-instance_method)

#### `distance_to_plane(plane) => Float`

Note: This function returns a `Float` value, not a `Length`.

**Remarks:** Note: This function returns a `Float` value, not a `Length`. The distance_to_plane method is used to compute the distance from the point to a plane. See module Geom for how to specify a plane.

**Parameters:**
- `plane` (Object) — A plane (see Geom for how to create a plane).

**Returns:** `Float` — a distance between a point and a plane in internal units if successful

[Docs](https://ruby.sketchup.com/Geom/Point3d.html#distance_to_plane-instance_method)

#### `inspect => String`

The inspect method is used to format a 3D point as a string.

**Remarks:** The inspect method is used to format a 3D point as a string. You will not often use these function directly. Instead, they are called automatically when an object is output using a print command like 'puts', which writes to the Ruby console.

**Returns:** `String` — a string point representation

[Docs](https://ruby.sketchup.com/Geom/Point3d.html#inspect-instance_method)

#### `linear_combination(weight1, point1, weight2, point2) => Geom::Point3d`

The linear_combination method is used to create a new point as a linear combination of two points.

**Remarks:** The linear_combination method is used to create a new point as a linear combination of two points. This method is generally used to get a point at some percentage along a line connecting the two points. A linear combination is a standard term for vector math. It is defined as point = weight1 * point1 + weight2 * point2.

**Parameters:**
- `weight1` (Float)
- `point1` (Geom::Point3d)
- `weight2` (Float)
- `point2` (Geom::Point3d)

**Returns:** `Geom::Point3d` — 

[Docs](https://ruby.sketchup.com/Geom/Point3d.html#linear_combination-class_method)

#### `offset(vector, length = vector.length) => Geom::Point3d`

The offset method is used to offset a point by a vector and return a new point.

**Remarks:** The offset method is used to offset a point by a vector and return a new point. The length of the vector must not be zero.

**Parameters:**
- `vector` (Geom::Vector3d) — A Vector3d object to offset the point by.
- `length` (Numeric) — the distance to offset. If not provided, the offset is my a distance equal to the vector length.

**Returns:** `Geom::Point3d` — a new Point3d object

[Docs](https://ruby.sketchup.com/Geom/Point3d.html#offset-instance_method)

#### `offset!(vector, length = vector.length) => Geom::Point3d`

The offset! method is used to offset a point by a vector.

**Remarks:** The offset! method is used to offset a point by a vector. The point itself is modified. Unlike offset, the point itself is modified.

**Parameters:**
- `vector` (Geom::Vector3d) — A Vector3d object to offset the point by.
- `length` (Numeric) — the distance to offset. If not provided, the offset is my a distance equal to the vector length.

**Returns:** `Geom::Point3d` — a new Point3d object

[Docs](https://ruby.sketchup.com/Geom/Point3d.html#offset!-instance_method)

#### `on_line?(line) => Boolean`

The on_line? method is used to determine if the point is on a line.

**Remarks:** The on_line? method is used to determine if the point is on a line. See module Geom for the various ways to specify a line.

**Parameters:**
- `line` (Object) — A line (see Geom for how to create a line).

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Geom/Point3d.html#on_line?-instance_method)

#### `on_plane?(plane) => Boolean`

The on_plane? method is used to determine if the point is on a plane.

**Remarks:** The on_plane? method is used to determine if the point is on a plane. See module Geom for the various ways to specify a plane.

**Parameters:**
- `plane` (Object)

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Geom/Point3d.html#on_plane?-instance_method)

#### `project_to_line(line) => Geom::Point3d`

The project_to_line method is used to retrieve the point on a line that is closest to this point.

**Remarks:** The project_to_line method is used to retrieve the point on a line that is closest to this point. The line may be defined by either a point and a vector or by two points.

**Parameters:**
- `line` (Object) — see Geom for how to specify a line

**Returns:** `Geom::Point3d` — the Point3d that is on a line closest to the point

[Docs](https://ruby.sketchup.com/Geom/Point3d.html#project_to_line-instance_method)

#### `project_to_plane(plane) => Geom::Point3d`

The project_to_plane method is used to retrieve the point on a plane that is closest to the point.

**Remarks:** The project_to_plane method is used to retrieve the point on a plane that is closest to the point. The plane may be defined by either a point on the plane and a vector perpendicular to the plane or by the coeficients to the plane equation AX + BY + CZ + D = 0. See Geom for details.

**Parameters:**
- `plane` (Object) — A plane (see Geom for how to create a plane).

**Returns:** `Geom::Point3d` — the Point3d that is on a plane closest to the point

[Docs](https://ruby.sketchup.com/Geom/Point3d.html#project_to_plane-instance_method)

#### `set!(x, y, z) => Geom::Point3d | set!(array3d) => Geom::Point3d | set!(point3d) => Geom::Point3d`

The #set! method is used to set the values of the Point3d.

**Remarks:** The #set! method is used to set the values of the Point3d.

**Parameters:**
- `x` (Numeric) — The x value for the point.
- `y` (Numeric) — The y value for the point.
- `z` (Numeric) — The z value for the point.

**Returns:** `Geom::Point3d` — The newly set Point3d object

[Docs](https://ruby.sketchup.com/Geom/Point3d.html#set!-instance_method)

#### `to_a => Array(Length, Length, Length)`

The to_a method is used to convert the point to an array of 3 numbers

**Remarks:** The to_a method is used to convert the point to an array of 3 numbers

**Returns:** `Array(Length, Length, Length)` — an array of three numbers representing x, y, z of the Point3d

[Docs](https://ruby.sketchup.com/Geom/Point3d.html#to_a-instance_method)

#### `to_s => String`

The to_s method is used to retrieve a string representation of a point.

**Remarks:** The to_s method is used to retrieve a string representation of a point.

**Returns:** `String` — the string representation of the Point3d

[Docs](https://ruby.sketchup.com/Geom/Point3d.html#to_s-instance_method)

#### `transform(transform) => Geom::Point3d`

Apply a Transformation to a point, returning a new point.

**Remarks:** Apply a Transformation to a point, returning a new point. The original vector is unchanged by this method.

**Parameters:**
- `transform` (Geom::Transformation) — A Transformation object.

**Returns:** `Geom::Point3d` — the newly transformed point

[Docs](https://ruby.sketchup.com/Geom/Point3d.html#transform-instance_method)

#### `transform!(transform) => Geom::Point3d`

Apply a Transformation to a point.

**Remarks:** Apply a Transformation to a point. The point itself is modified.

**Parameters:**
- `transform` (Geom::Transformation) — A Transformation object.

**Returns:** `Geom::Point3d` — the transformed point

[Docs](https://ruby.sketchup.com/Geom/Point3d.html#transform!-instance_method)

#### `vector_to(point3d) => Geom::Vector3d`

The vector_to team method retrieves the vector between points.

**Remarks:** The vector_to team method retrieves the vector between points.

**Parameters:**
- `point` (Geom::Point3d, Array(Numeric, Numeric, Numeric)) — A Point3d object.

**Returns:** `Geom::Vector3d` — a Vector object

[Docs](https://ruby.sketchup.com/Geom/Point3d.html#vector_to-instance_method)

### Properties
- `=` (Geom::Point3d, Array(Numeric, Numeric, Numeric), set) — The == method is used to compare two points for equality.
- `[]` (Length, get/set) — The [] method is used to retrieve the value of the point at the specified index.
- `x` (Length, get/set) — The #x method retrieves the x value of the 3D point.
- `y` (Length, get/set) — The #y method retrieves the y value of the 3D point.
- `z` (Length, get/set) — The #z method retrieves the z value of the 3D point.

## PolygonMesh (class)

Note: As of SketchUp 2022.0 the new Sketchup::EntitiesBuilder interface can be used to generate bulk geometry. It has similar performance as PolygonMesh, but with similar degree of per-entity control as Sketchup::Entities. The PolygonMesh class contains methods to create polygon mesh structures. This is useful if you need to write a custom importer/exporter in Ruby that works at the level of triangulated polygons. For example, you can determine the triangles that make up a 15-sided Sketchup::Face by using this class, or write a Sketchup::Importer that reads a data file, creates a mesh from it, and draws faces based on the mesh. You can construct a mesh manually using the methods of this class, or you can get a mesh from a face by calling the Sketchup::Face#mesh method. See Sketchup::Entities#add_faces_from_mesh for an easy way to convert a mesh back into faces.

**Remarks:** See Also: Guide on Generating Geometry

[Vendor docs](https://ruby.sketchup.com/Geom/PolygonMesh.html)

### Constructors
- `PolygonMesh(number_points)` — Note: When creating a mesh with normals and/or UVQ data it's critical that the number of points estimated is equal to or higher than the final number of points added. If fewer points are estimated the normals and UVQ data might end up out of sync. Create a new empty polygon mesh. The number of points and polygons are optional and are used as a hint to decide how much space to pre-allocate to speed up adding points and polygons. As of SketchUp 2021.1 the performance of looking up and inserting points is significantly better provided the mesh was initialized with roughly the correct number of total points.

### Methods
#### `add_point(point) => Integer`

Note: In SketchUp 2021.

**Remarks:** Note: In SketchUp 2021.1 this method was improved to be faster. See #initialize for details. The #add_point method is used to add a point to the mesh. The returned index can be used for creating polygons.

**Parameters:**
- `point` (Geom::Point3d)

**Returns:** `Integer` — the index in the mesh for the point

[Docs](https://ruby.sketchup.com/Geom/PolygonMesh.html#add_point-instance_method)

#### `add_polygon(index, index, index, ...) => Integer | add_polygon(index_array) => Integer | add_polygon(point3d, point3d, point3d, ...) => Integer | add_polygon(point3d_array) => Integer`

Note: In SketchUp 2021.

**Remarks:** Note: In SketchUp 2021.1 this method was improved to be faster. See #initialize for details. The #add_polygon method is used for adding a polygon to a Geom::PolygonMesh. All variations of this method require at least 3 elements to define a polygon, although more may be given.

**Parameters:**
- `index` (Integer) — An index of a vertex in the mesh. Remember that mesh indices start at 1.
- `...` (Integer) — Additional indices (optional)

**Returns:** `Integer` — The 1-based index of the polygon in the mesh. 0 is returned if the method failed to create a polygon.

[Docs](https://ruby.sketchup.com/Geom/PolygonMesh.html#add_polygon-instance_method)

#### `count_points => Integer`

The #count_points method is used to count the number of points in a mesh.

**Remarks:** The #count_points method is used to count the number of points in a mesh.

**Returns:** `Integer` — the number of points in a mesh

[Docs](https://ruby.sketchup.com/Geom/PolygonMesh.html#count_points-instance_method)

#### `count_polygons => Integer`

The #count_polygons count the number of polygons in the mesh.

**Remarks:** The #count_polygons count the number of polygons in the mesh.

**Returns:** `Integer` — the number of polygons in the mesh

[Docs](https://ruby.sketchup.com/Geom/PolygonMesh.html#count_polygons-instance_method)

#### `normal_at(index) => Geom::Vector3d?`

Note: Index starts at 1.

**Remarks:** Note: Index starts at 1. The #normal_at method is used to determine the vertex normal at a particular index in the mesh. This only works for meshes retrieved from Sketchup::Face#mesh with the PolygonMeshNormals flag.

**Parameters:**
- `index` (Integer) — The index in the mesh for the vertex normal to be retrieved

**Returns:** `Geom::Vector3d, nil` — 

[Docs](https://ruby.sketchup.com/Geom/PolygonMesh.html#normal_at-instance_method)

#### `point_at(index) => Geom::Point3d?`

Note: Index starts at 1.

**Remarks:** Note: Index starts at 1. The #point_at method is used to retrieve the point at a specific index in the mesh.

**Parameters:**
- `index` (Integer) — The index in the mesh for the point to be retrieved

**Returns:** `Geom::Point3d, nil` — 

[Docs](https://ruby.sketchup.com/Geom/PolygonMesh.html#point_at-instance_method)

#### `point_index(point) => Integer`

Note: Returns 0 if point is not found.

**Remarks:** Note: Returns 0 if point is not found. The #point_index method is used to retrieve the index of a point in the mesh.

**Parameters:**
- `point` (Geom::Point3d)

**Returns:** `Integer` — the index in the mesh for the Geom::Point3d object

[Docs](https://ruby.sketchup.com/Geom/PolygonMesh.html#point_index-instance_method)

#### `points => Array<Geom::Point3d>`

The #points method is used to retrieve an array of points (vertices) in the mesh

**Remarks:** The #points method is used to retrieve an array of points (vertices) in the mesh

**Returns:** `Array<Geom::Point3d>` — 

[Docs](https://ruby.sketchup.com/Geom/PolygonMesh.html#points-instance_method)

#### `polygon_at(index) => Array<Integer>?`

Note: Index starts at 1.

**Remarks:** Note: Index starts at 1. Note: The returned array can contain negative values with the sign indicating a hidden edge. For example, a return value of [-1, 2, 3] indicates that the edge from 1 to 2 is hidden. The negative values should not be used as an index for #point_at, take the absolute value of the index value in the polygon array. So if you get [-1, 2,3] use 1 as the argument to #point_at. The #polygon_at method is used to retrieve an array of vertex index values for a polygon at a specific index.

**Parameters:**
- `index` (Integer) — The index of the desired polygon.

**Returns:** `Array<Integer>, nil` — 

[Docs](https://ruby.sketchup.com/Geom/PolygonMesh.html#polygon_at-instance_method)

#### `polygon_points_at(index) => Array<Geom::Point3d>?`

Note: Index starts at 1.

**Remarks:** Note: Index starts at 1. The #polygon_points_at method is used to retrieve the points for a polygon that is at a specific index in the mesh.

**Parameters:**
- `index` (Integer) — An index for a polygon in the mesh.

**Returns:** `Array<Geom::Point3d>, nil` — 

[Docs](https://ruby.sketchup.com/Geom/PolygonMesh.html#polygon_points_at-instance_method)

#### `polygons => Array<Array<Integer>>`

The #polygons method is used to retrieve an array of all polygons in the mesh.

**Remarks:** The #polygons method is used to retrieve an array of all polygons in the mesh. The returned array contains an array that can have a negative value with the sign indicating a hidden edge. For example, a return value of [-1, 2, 3] indicates that the edge from 1 to 2 is hidden.

**Returns:** `Array<Array<Integer>>` — 

[Docs](https://ruby.sketchup.com/Geom/PolygonMesh.html#polygons-instance_method)

#### `set_point(index, point) => Geom::PolygonMesh`

Note: Index starts at 1.

**Remarks:** Note: Index starts at 1. The #set_point method is used to set the point at a specific index in the mesh.

**Parameters:**
- `index` (Integer) — The index where the point will be set.
- `point` (Geom::Point3d) — A Point3d object to set at the index.

**Returns:** `Geom::PolygonMesh` — 

[Docs](https://ruby.sketchup.com/Geom/PolygonMesh.html#set_point-instance_method)

#### `set_uv(index, point, front) => nil`

Note: If you don't specify how many points you will be adding to the mesh when you initiate it you may risk the UV data becoming out of sync.

**Remarks:** Note: If you don't specify how many points you will be adding to the mesh when you initiate it you may risk the UV data becoming out of sync. Note: Index starts at 1. The #set_uv method is used to define UV mapping coordinates to points in the mesh. Beware that the polygons connected to the point will share UV coordiates so UV mapping coordinates needs to be continuous across the polygon mesh. When setting the UV for a point one need to make sure to have the correct index for the point. It's therefore best to add the points using #add_point and use the index it returns for following calls to set_uv and #add_polygon. If you are not able to calculate how many points there will be in your mesh make sure to not specify an index in #set_uv higher than the number of times you have called #set_uv.

**Parameters:**
- `index` (Integer) — An Integer representing the UV index.
- `point` (Geom::Point3d) — A Point3d object representing UV coordinates.
- `front` (Boolean) — A boolean representing the front or back.

**Returns:** `nil` — 

[Docs](https://ruby.sketchup.com/Geom/PolygonMesh.html#set_uv-instance_method)

#### `transform!(transformation) => Geom::PolygonMesh`

The #transform! method is used to apply a transformation to a mesh.

**Remarks:** The #transform! method is used to apply a transformation to a mesh.

**Parameters:**
- `transformation` (Geom::Transformation)

**Returns:** `Geom::PolygonMesh` — 

[Docs](https://ruby.sketchup.com/Geom/PolygonMesh.html#transform!-instance_method)

#### `uv_at(index, front) => Geom::Point3d?`

Note: Index starts at 1.

**Remarks:** Note: Index starts at 1. The #uv_at method is used to access a uv (texture coordinates) at a specific index. “UVs” is a way of referring to the u,v texture coordinates (as opposed to the X, Y, and Z axis that you construct your meshes on), which are points defining 1-by-1 positions within an image. These coordinates connect to points in your 3D model, to position an image texture onto it's surface (similar to virtual “thumb tacks”) These coordinates pin an exact spot on an image that you wish to use to texture your model to a specific point on an object's surface. Between these points, your software will stretch the image smoothly. This is what is referred to as UV mapping.

**Parameters:**
- `index` (Integer) — The index for the texture coordinate.
- `front` (Boolean) — Set to true to get the UV for the front size, false for the back side.

**Returns:** `Geom::Point3d, nil` — a Point3d object where the x equals the u value and the y equals the v value. Returns nil on failure.

[Docs](https://ruby.sketchup.com/Geom/PolygonMesh.html#uv_at-instance_method)

#### `uvs(front) => Array<Geom::Point3d>`

The #uvs method is used to retrieve an array of uv coordinates in the mesh.

**Remarks:** The #uvs method is used to retrieve an array of uv coordinates in the mesh.

**Parameters:**
- `front` (Boolean)

**Returns:** `Array<Geom::Point3d>` — 

[Docs](https://ruby.sketchup.com/Geom/PolygonMesh.html#uvs-instance_method)

### Properties
- `NO_SMOOTH_OR_HIDE` (Object, get) — 
- `HIDE_BASED_ON_INDEX` (Object, get) — 
- `SOFTEN_BASED_ON_INDEX` (Object, get) — 
- `AUTO_SOFTEN` (Object, get) — 
- `SMOOTH_SOFT_EDGES` (Object, get) — 
- `MESH_POINTS` (Object, get) — 
- `MESH_UVQ_FRONT` (Object, get) — 
- `MESH_UVQ_BACK` (Object, get) — 
- `MESH_NORMALS` (Object, get) — 

## Transformation (class)

Transformations are a standard construct in the 3D world for representing the position, rotation, and sizing of a given entity. In the SketchUp world, Sketchup::ComponentInstance and Sketchup::Group have a .transformation method that reports their current state and various methods (.move!, transformation=, etc.) that allow them to be manipulated. Use of the transformation class requires a knowledge of geometrical transformations in 3 dimensions which is covered extensively on the internet.

[Vendor docs](https://ruby.sketchup.com/Geom/Transformation.html)

### Constructors
- `Transformation(point)` — You can use this method or one of the more specific methods for creating specific kinds of Transformations.

### Methods
#### `*(point) => Geom::Point3d | *(vector) => Geom::Vector3d | *(transformation) => Geom::Transformation | *(point) => Array(Float, Float, Float) | *(plane) => Array(Float, Float, Float, Float) | *(plane) => Array(Float, Float, Float, Float)`

The #* method is used to do matrix multiplication using the transform.

**Remarks:** The #* method is used to do matrix multiplication using the transform.

**Parameters:**
- `point` (Geom::Point3d)

**Returns:** `Geom::Point3d` — 

[Docs](https://ruby.sketchup.com/Geom/Transformation.html#*-instance_method)

#### `axes(origin, xaxis, yaxis, zaxis) => Geom::Transformation | axes(origin, xaxis, yaxis) => Geom::Transformation`

The axes method creates a transformation that goes from world coordinates to an arbitrary coordinate system defined by an origin and three axis vectors.

**Remarks:** The axes method creates a transformation that goes from world coordinates to an arbitrary coordinate system defined by an origin and three axis vectors.

**Parameters:**
- `origin` (Geom::Point3d)
- `xaxis` (Geom::Vector3d)
- `yaxis` (Geom::Vector3d)
- `zaxis` (Geom::Vector3d)

**Returns:** `Geom::Transformation` — 

[Docs](https://ruby.sketchup.com/Geom/Transformation.html#axes-class_method)

#### `clone => Geom::Transformation`

The #clone method is used to create a copy of a transformation.

**Remarks:** The #clone method is used to create a copy of a transformation.

**Returns:** `Geom::Transformation` — 

[Docs](https://ruby.sketchup.com/Geom/Transformation.html#clone-instance_method)

#### `identity? => Boolean`

Note: As of SketchUp 2018, this now looks at the data to determine if the transformation is identity.

**Remarks:** Note: As of SketchUp 2018, this now looks at the data to determine if the transformation is identity. Prior to SU2018, this only looks at the flag to see if the transform has not been modified. If the transform has been changed, this will return false even if it is really the identity. The #identity? method is used to determine if a transformation is the IDENTITY transform.

**Returns:** `Boolean` — true if the transformation is the identity

[Docs](https://ruby.sketchup.com/Geom/Transformation.html#identity?-instance_method)

#### `interpolate(transform1, transform2, weight) => Geom::Transformation`

The interpolate method is used to create a new transformation that is the result of interpolating between two other transformations.

**Remarks:** The interpolate method is used to create a new transformation that is the result of interpolating between two other transformations. Parameter weight is a value (between 0.0 and 1.0) that represents the percentage given to `transformation1` and `transformation2`.

**Parameters:**
- `transformation1` (Geom::Transformation)
- `transformation2` (Geom::Transformation)
- `weight` (Float) — A value between 0.0 and 1.0.

**Returns:** `Geom::Transformation` — 

[Docs](https://ruby.sketchup.com/Geom/Transformation.html#interpolate-class_method)

#### `inverse => Geom::Transformation`

Note: As of SketchUp 2026, this will raise an error if the Geom::Transformation is not invertible.

**Remarks:** Note: As of SketchUp 2026, this will raise an error if the Geom::Transformation is not invertible. Prior to 2026 this would silently attempt to invert the transformation possibly returning in an invalid transformation. The #inverse method is used to retrieve the inverse of a transformation.

**Returns:** `Geom::Transformation` — 

[Docs](https://ruby.sketchup.com/Geom/Transformation.html#inverse-instance_method)

#### `invert! => Geom::Transformation`

Note: As of SketchUp 2026, this will raise an error if the Geom::Transformation is not invertible.

**Remarks:** Note: As of SketchUp 2026, this will raise an error if the Geom::Transformation is not invertible. Prior to 2026 this would silently attempt to invert the transformation possibly creating in an invalid transformation. The #invert! method sets the transformation to its inverse.

**Returns:** `Geom::Transformation` — 

[Docs](https://ruby.sketchup.com/Geom/Transformation.html#invert!-instance_method)

#### `origin => Geom::Point3d`

The #origin method retrieves the origin of a rigid transformation.

**Remarks:** The #origin method retrieves the origin of a rigid transformation.

**Returns:** `Geom::Point3d` — the origin of the transformation.

[Docs](https://ruby.sketchup.com/Geom/Transformation.html#origin-instance_method)

#### `rotation(point, vector, angle) => Geom::Transformation`

The rotation method is used to create a transformation that does rotation about an axis.

**Remarks:** The rotation method is used to create a transformation that does rotation about an axis. The axis is defined by a point and a vector. The angle is given in radians.

**Parameters:**
- `point` (Geom::Point3d)
- `vector` (Geom::Vector3d)
- `angle` (Float) — The angle in radians.

**Returns:** `Geom::Transformation` — 

[Docs](https://ruby.sketchup.com/Geom/Transformation.html#rotation-class_method)

#### `scaling(scale) => Geom::Transformation | scaling(xscale, yscale, zscale) => Geom::Transformation | scaling(point, scale) => Geom::Transformation | scaling(point, xscale, yscale, zscale) => Geom::Transformation`

The scaling method is used to create a transformation that does scaling.

**Remarks:** The scaling method is used to create a transformation that does scaling.

**Parameters:**
- `scale` (Float) — The global scale factor for the transform.

**Returns:** `Geom::Transformation` — 

[Docs](https://ruby.sketchup.com/Geom/Transformation.html#scaling-class_method)

#### `set!(transformation) => Geom::Transformation | set!(point) => Geom::Transformation | set!(vector) => Geom::Transformation | set!(matrix) => Geom::Transformation | set!(scale) => Geom::Transformation`

The #set! method is used to set this transformation to match another one.

**Remarks:** The #set! method is used to set this transformation to match another one. The argument is anything that can be converted into a transformation.

**Parameters:**
- `transformation` (Geom::Transformation)

**Returns:** `Geom::Transformation` — 

[Docs](https://ruby.sketchup.com/Geom/Transformation.html#set!-instance_method)

#### `to_a => Array<Float>`

The #to_a method retrieves a 16 element array which contains the values that define the transformation.

**Remarks:** The #to_a method retrieves a 16 element array which contains the values that define the transformation.

**Returns:** `Array<Float>` — 

[Docs](https://ruby.sketchup.com/Geom/Transformation.html#to_a-instance_method)

#### `translation(vector) => Geom::Transformation | translation(point) => Geom::Transformation`

The translation method is used to create a transformation that does translation.

**Remarks:** The translation method is used to create a transformation that does translation.

**Parameters:**
- `vector` (Geom::Vector3d)

**Returns:** `Geom::Transformation` — 

[Docs](https://ruby.sketchup.com/Geom/Transformation.html#translation-class_method)

#### `xaxis => Geom::Vector3d`

The #xaxis method retrieves the x axis of a rigid transformation.

**Remarks:** The #xaxis method retrieves the x axis of a rigid transformation.

**Returns:** `Geom::Vector3d` — 

[Docs](https://ruby.sketchup.com/Geom/Transformation.html#xaxis-instance_method)

#### `yaxis => Geom::Vector3d`

The #yaxis method retrieves the y axis of a rigid transformation.

**Remarks:** The #yaxis method retrieves the y axis of a rigid transformation.

**Returns:** `Geom::Vector3d` — 

[Docs](https://ruby.sketchup.com/Geom/Transformation.html#yaxis-instance_method)

#### `zaxis => Geom::Vector3d`

The #zaxis method retrieves the z axis of a rigid transformation.

**Remarks:** The #zaxis method retrieves the z axis of a rigid transformation.

**Returns:** `Geom::Vector3d` — 

[Docs](https://ruby.sketchup.com/Geom/Transformation.html#zaxis-instance_method)

## Transformation2d (class)

(No description provided in vendor docs for Geom::Transformation2d.)

[Vendor docs](https://ruby.sketchup.com/Geom/Transformation2d.html)

### Constructors
- `Transformation2d(transformation)` — The #initialize method creates a new Geom::Transformation2d. You can use this method or one of the more specific methods for creating specific kinds of Geom::Transformation2d.

### Methods
#### `*(point) => Geom::Point2d | *(vector) => Geom::Vector2d | *(transformation) => Geom::Transformation2d | *(point) => Array(Float, Float)`

The #* method is used to do matrix multiplication using the transform.

**Remarks:** The #* method is used to do matrix multiplication using the transform.

**Parameters:**
- `point` (Geom::Point2d)

**Returns:** `Geom::Point2d` — 

[Docs](https://ruby.sketchup.com/Geom/Transformation2d.html#*-instance_method)

#### `clone => Geom::Transformation2d`

The #clone method creates a copy of the Geom::Transformation2d.

**Remarks:** The #clone method creates a copy of the Geom::Transformation2d.

**Returns:** `Geom::Transformation2d` — 

[Docs](https://ruby.sketchup.com/Geom/Transformation2d.html#clone-instance_method)

#### `identity? => Boolean`

The #identity? method determines if the Geom::Transformation2d is the IDENTITY_2D transform.

**Remarks:** The #identity? method determines if the Geom::Transformation2d is the IDENTITY_2D transform.

**Returns:** `Boolean` — true if the transform is the identity

[Docs](https://ruby.sketchup.com/Geom/Transformation2d.html#identity?-instance_method)

#### `inverse => Geom::Transformation2d`

The #inverse method is used to retrieve the inverse of a transformation.

**Remarks:** The #inverse method is used to retrieve the inverse of a transformation.

**Returns:** `Geom::Transformation2d` — 

[Docs](https://ruby.sketchup.com/Geom/Transformation2d.html#inverse-instance_method)

#### `invert! => Geom::Transformation2d`

The #invert! method sets the transformation to its inverse.

**Remarks:** The #invert! method sets the transformation to its inverse.

**Returns:** `Geom::Transformation2d` — 

[Docs](https://ruby.sketchup.com/Geom/Transformation2d.html#invert!-instance_method)

#### `rotation(point, angle) => Geom::Transformation2d`

The rotation method is used to create a transformation that does rotation about a point.

**Remarks:** The rotation method is used to create a transformation that does rotation about a point.

**Parameters:**
- `point` (Geom::Point2d)
- `angle` (Float) — The angle in radians.

**Returns:** `Geom::Transformation2d` — 

[Docs](https://ruby.sketchup.com/Geom/Transformation2d.html#rotation-class_method)

#### `scaling(scale) => Geom::Transformation2d | scaling(xscale, yscale) => Geom::Transformation2d | scaling(point, scale) => Geom::Transformation2d | scaling(point, xscale, yscale) => Geom::Transformation2d`

The scaling method is used to create a transformation that does scaling.

**Remarks:** The scaling method is used to create a transformation that does scaling.

**Parameters:**
- `scale` (Float) — The global scale factor for the transform.

**Returns:** `Geom::Transformation2d` — 

[Docs](https://ruby.sketchup.com/Geom/Transformation2d.html#scaling-class_method)

#### `set!(transformation) => Geom::Transformation2d | set!(matrix) => Geom::Transformation2d`

The #set! method sets the Geom::Transformation2d to match another one.

**Remarks:** The #set! method sets the Geom::Transformation2d to match another one. The argument is anything that can be converted into a Geom::Transformation2d.

**Parameters:**
- `transformation` (Geom::Transformation2d)

**Returns:** `Geom::Transformation2d` — 

[Docs](https://ruby.sketchup.com/Geom/Transformation2d.html#set!-instance_method)

#### `to_a => Array<Float>`

The #to_a method returns a 6 element array which contains the values that define the Transformation2d.

**Remarks:** The #to_a method returns a 6 element array which contains the values that define the Transformation2d.

**Returns:** `Array<Float>` — an array of 6 elements

[Docs](https://ruby.sketchup.com/Geom/Transformation2d.html#to_a-instance_method)

#### `translation(vector) => Geom::Transformation2d | translation(point) => Geom::Transformation2d`

The translation method is used to create a transformation that does translation.

**Remarks:** The translation method is used to create a transformation that does translation.

**Parameters:**
- `vector` (Geom::Vector2d)

**Returns:** `Geom::Transformation2d` — 

[Docs](https://ruby.sketchup.com/Geom/Transformation2d.html#translation-class_method)

### Properties
- `=` (Geom::Transformation2d, set) — The #== method checks to see if the two Geom::Transformation2ds are equal.

## UTM (class)

Note: Valid ranges for #zone_number and #zone_letter are 1-60 and C-X (omitting I and O). Valid ranges for #x and #y are 100000-899999. The UTM class lets you work with UTM map coordinates.

[Vendor docs](https://ruby.sketchup.com/Geom/UTM.html)

### Constructors
- `UTM(zone_number, zone_letter, x, y)` — The #initialize method is used to create a new UTM coordinate. You will often create UTM objects by calling the method Sketchup::Model#point_to_utm instead of calling this method.

### Methods
#### `to_a => Array(Integer, String, Float, Float)`

The #to_a method returns a UTM coordinate as a 4 element array.

**Remarks:** The #to_a method returns a UTM coordinate as a 4 element array. The Array elements are the zone number, the zone letter, the x coordinate and the y coordinate.

**Returns:** `Array(Integer, String, Float, Float)` — 

[Docs](https://ruby.sketchup.com/Geom/UTM.html#to_a-instance_method)

#### `to_latlong => Geom::LatLong`

The #to_latlong method is used to convert UTM coordinates to latitude and longitude.

**Remarks:** The #to_latlong method is used to convert UTM coordinates to latitude and longitude. See the LatLong class for more information.

**Returns:** `Geom::LatLong` — 

[Docs](https://ruby.sketchup.com/Geom/UTM.html#to_latlong-instance_method)

#### `to_s => String`

The #to_s method is used to retrieve a string representation of a UTM.

**Remarks:** The #to_s method is used to retrieve a string representation of a UTM.

**Returns:** `String` — 

[Docs](https://ruby.sketchup.com/Geom/UTM.html#to_s-instance_method)

#### `x => Float`

The #x method returns the UTM x coordinate.

**Remarks:** The #x method returns the UTM x coordinate.

**Returns:** `Float` — 

[Docs](https://ruby.sketchup.com/Geom/UTM.html#x-instance_method)

#### `y => Float`

The #y method returns the UTM y coordinate.

**Remarks:** The #y method returns the UTM y coordinate.

**Returns:** `Float` — 

[Docs](https://ruby.sketchup.com/Geom/UTM.html#y-instance_method)

#### `zone_letter => String`

The #zone_letter method returns the UTM zone letter.

**Remarks:** The #zone_letter method returns the UTM zone letter.

**Returns:** `String` — 

[Docs](https://ruby.sketchup.com/Geom/UTM.html#zone_letter-instance_method)

#### `zone_number => Integer`

The #zone_number method returns the UTM zone number.

**Remarks:** The #zone_number method returns the UTM zone number.

**Returns:** `Integer` — 

[Docs](https://ruby.sketchup.com/Geom/UTM.html#zone_number-instance_method)

## Vector2d (class)

The Vector2d class represents vectors in a 2 dimensional space. There are numerous tutorials on 2D vectors available on the internet.

[Vendor docs](https://ruby.sketchup.com/Geom/Vector2d.html)

### Constructors
- `Vector2d(x, y)` — The new method creates a new Geom::Vector2d.

### Methods
#### `%(vector) => Float`

The #% method is used to compute the dot product between two vectors.

**Remarks:** The #% method is used to compute the dot product between two vectors. This is an alias of the #dot method.

**Parameters:**
- `vector` (Geom::Vector2d, Array(Float, Float))

**Returns:** `Float` — 

[Docs](https://ruby.sketchup.com/Geom/Vector2d.html#%-instance_method)

#### `*(vector) => Float`

The #* method returns the cross product between two Geom::Vector2d.

**Remarks:** The #* method returns the cross product between two Geom::Vector2d. This is an alias of the cross method.

**Parameters:**
- `vector` (Geom::Vector2d, Array(Float, Float))

**Returns:** `Float` — 

[Docs](https://ruby.sketchup.com/Geom/Vector2d.html#*-instance_method)

#### `+(vector) => Geom::Vector2d`

The #+ method adds a Geom::Vector2d to this one.

**Remarks:** The #+ method adds a Geom::Vector2d to this one.

**Parameters:**
- `vector` (Geom::Vector2d, Array(Float, Float))

**Returns:** `Geom::Vector2d` — 

[Docs](https://ruby.sketchup.com/Geom/Vector2d.html#+-instance_method)

#### `-(vector) => Geom::Vector2d`

The #- method subtracts a Geom::Vector2d from this one.

**Remarks:** The #- method subtracts a Geom::Vector2d from this one.

**Parameters:**
- `vector` (Geom::Vector2d, Array(Float, Float))

**Returns:** `Geom::Vector2d` — 

[Docs](https://ruby.sketchup.com/Geom/Vector2d.html#--instance_method)

#### `angle_between(vector) => Float`

The #angle_between method computes the angle in radians between the Geom::Vector2d and another Geom::Vector2d.

**Remarks:** The #angle_between method computes the angle in radians between the Geom::Vector2d and another Geom::Vector2d.

**Parameters:**
- `vector` (Geom::Vector2d, Array(Float, Float))

**Returns:** `Float` — The angle (in radians)

[Docs](https://ruby.sketchup.com/Geom/Vector2d.html#angle_between-instance_method)

#### `clone => Geom::Vector2d`

The #clone method makes a copy of the Geom::Vector2d.

**Remarks:** The #clone method makes a copy of the Geom::Vector2d. This method is equivalent to vec2 = Geom::Vector2d.new(vec).

**Returns:** `Geom::Vector2d` — 

[Docs](https://ruby.sketchup.com/Geom/Vector2d.html#clone-instance_method)

#### `cross(vector) => Float`

The #cross method returns the cross product between two Geom::Vector2ds.

**Remarks:** The #cross method returns the cross product between two Geom::Vector2ds. The cross product, also called the vector product, is an operation on two vectors. The cross product of two vectors produces a third vector which is perpendicular to the plane in which the first two lie.

**Parameters:**
- `vector` (Geom::Vector2d, Array(Float, Float))

**Returns:** `Float` — 

[Docs](https://ruby.sketchup.com/Geom/Vector2d.html#cross-instance_method)

#### `dot(vector) => Float`

The #dot method is used to compute the dot product between two vectors.

**Remarks:** The #dot method is used to compute the dot product between two vectors.

**Parameters:**
- `vector` (Geom::Vector2d, Array(Float, Float))

**Returns:** `Float` — 

[Docs](https://ruby.sketchup.com/Geom/Vector2d.html#dot-instance_method)

#### `inspect => String`

The #inspect method formats the Geom::Vector2d as a string.

**Remarks:** The #inspect method formats the Geom::Vector2d as a string.

**Returns:** `String` — the string representation of the Geom::Vector2d

[Docs](https://ruby.sketchup.com/Geom/Vector2d.html#inspect-instance_method)

#### `normalize => Geom::Vector2d`

The #normalize method returns a Geom::Vector2d that is a unit vector of the Geom::Vector2d.

**Remarks:** The #normalize method returns a Geom::Vector2d that is a unit vector of the Geom::Vector2d.

**Returns:** `Geom::Vector2d` — 

[Docs](https://ruby.sketchup.com/Geom/Vector2d.html#normalize-instance_method)

#### `normalize! => Object`

The #normalize! method converts a Geom::Vector2d vector into a unit vector.

**Remarks:** The #normalize! method converts a Geom::Vector2d vector into a unit vector. Another way to do this is vector.length = 1.0

[Docs](https://ruby.sketchup.com/Geom/Vector2d.html#normalize!-instance_method)

#### `parallel?(vector) => Boolean`

The #parallel? method determines if two Geom::Vector2ds are parallel within a tolerance.

**Remarks:** The #parallel? method determines if two Geom::Vector2ds are parallel within a tolerance. Two vectors are parallel if there exists a scalar multiple between them.

**Parameters:**
- `vector` (Geom::Vector2d, Array(Float, Float))

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Geom/Vector2d.html#parallel?-instance_method)

#### `perpendicular?(vector) => Boolean`

The #perpendicular? method determines if two Geom::Vector2ds are perpendicular within a tolerance.

**Remarks:** The #perpendicular? method determines if two Geom::Vector2ds are perpendicular within a tolerance. Two vectors are considered perpendicular if their dot product is zero.

**Parameters:**
- `vector` (Geom::Vector2d, Array(Float, Float))

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Geom/Vector2d.html#perpendicular?-instance_method)

#### `reverse => Geom::Vector2d`

The #reverse method returns a new Geom::Vector2d that is the reverse of the Geom::Vector2d, leaving the original unchanged.

**Remarks:** The #reverse method returns a new Geom::Vector2d that is the reverse of the Geom::Vector2d, leaving the original unchanged.

**Returns:** `Geom::Vector2d` — 

[Docs](https://ruby.sketchup.com/Geom/Vector2d.html#reverse-instance_method)

#### `reverse! => Object`

The #reverse! method reverses the Geom::Vector2d in place.

**Remarks:** The #reverse! method reverses the Geom::Vector2d in place.

[Docs](https://ruby.sketchup.com/Geom/Vector2d.html#reverse!-instance_method)

#### `same_direction?(vector) => Boolean`

The #same_direction? method determines if the Geom::Vector2d is parallel to and in the same direction as another Geom::Vector2d within tolerance.

**Remarks:** The #same_direction? method determines if the Geom::Vector2d is parallel to and in the same direction as another Geom::Vector2d within tolerance.

**Parameters:**
- `vector` (Geom::Vector2d, Array(Float, Float))

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Geom/Vector2d.html#same_direction?-instance_method)

#### `set!(vector) => Geom::Vector2d | set!(x, y) => Geom::Vector2d`

The #set! method sets the values of the Geom::Vector2d.

**Remarks:** The #set! method sets the values of the Geom::Vector2d.

**Parameters:**
- `vector` (Geom::Vector2d, Array(Float, Float))

**Returns:** `Geom::Vector2d` — 

[Docs](https://ruby.sketchup.com/Geom/Vector2d.html#set!-instance_method)

#### `to_a => Array(Float, Float)`

The #to_a method retrieves the coordinates of the Geom::Vector2d in an Array.

**Remarks:** The #to_a method retrieves the coordinates of the Geom::Vector2d in an Array.

**Returns:** `Array(Float, Float)` — 

[Docs](https://ruby.sketchup.com/Geom/Vector2d.html#to_a-instance_method)

#### `to_s => String`

The #to_s method returns a string representation of the Geom::Vector2d.

**Remarks:** The #to_s method returns a string representation of the Geom::Vector2d.

**Returns:** `String` — the string representation of the Geom::Vector2d

[Docs](https://ruby.sketchup.com/Geom/Vector2d.html#to_s-instance_method)

#### `transform(transform) => Geom::Vector2d`

The #transform method applies a transformation to a vector, returning a new vector.

**Remarks:** The #transform method applies a transformation to a vector, returning a new vector. The original vector is unchanged by this method.

**Parameters:**
- `transform` (Geom::Transformation2d) — A transformation object to apply to the vector.

**Returns:** `Geom::Vector2d` — the newly transformed vector

[Docs](https://ruby.sketchup.com/Geom/Vector2d.html#transform-instance_method)

#### `transform!(transform) => Geom::Vector2d`

The #transform! method applies a transformation to a vector.

**Remarks:** The #transform! method applies a transformation to a vector. The vector itself is modified.

**Parameters:**
- `transform` (Geom::Transformation2d) — A Transformation object to apply to the vector.

**Returns:** `Geom::Vector2d` — the transformed vector

[Docs](https://ruby.sketchup.com/Geom/Vector2d.html#transform!-instance_method)

#### `unit_vector? => Boolean`

The #unit_vector? method returns whether the Geom::Vector2d is a unit vector.

**Remarks:** The #unit_vector? method returns whether the Geom::Vector2d is a unit vector. This is equivalent to vector.length == 1.0

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Geom/Vector2d.html#unit_vector?-instance_method)

#### `valid? => Boolean`

The #valid? method verifies if a Geom::Vector2d is valid.

**Remarks:** The #valid? method verifies if a Geom::Vector2d is valid. A Geom::Vector2d is valid if its length is not zero.

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Geom/Vector2d.html#valid?-instance_method)

### Properties
- `=` (Geom::Vector2d, Array(Float, Float), set) — The #== method returns whether two Geom::Vector2d are equal within tolerance.
- `[]` (Float, get/set) — The #[] method returns the value of the Geom::Vector2d at the specified index.
- `length` (Length, get/set) — The #length method returns the length of the Geom::Vector2d.
- `x` (Float, get/set) — The #x method retrieves the x value of the Geom::Vector2d.
- `y` (Float, get/set) — The #y method retrieves the y value of the Geom::Vector2d.

## Vector3d (class)

The Vector3d class is used to represent vectors in a 3 dimensional space. Vectors in SketchUp have a direction and a length, but not a starting point. There are numerous tutorials on 3D vectors available on the internet.

[Vendor docs](https://ruby.sketchup.com/Geom/Vector3d.html)

### Constructors
- `Vector3d(x, y, z)` — The new method is used to create a new vector.

### Methods
#### `%(vector3d) => Float`

The #% method is used to compute the dot product between two vectors.

**Remarks:** The #% method is used to compute the dot product between two vectors. This is an alias of the #dot method.

**Parameters:**
- `vector3d` (Geom::Vector3d, Array(Float, Float, Float))

**Returns:** `Float` — 

[Docs](https://ruby.sketchup.com/Geom/Vector3d.html#%-instance_method)

#### `*(vector3d) => Geom::Vector3d`

The #* method is used to compute the cross product between two vectors.

**Remarks:** The #* method is used to compute the cross product between two vectors. The cross product, also called the vector product, is an operation on two vectors. The cross product of two vectors produces a third vector which is perpendicular to the plane in which the first two lie. This is an alias of the #cross method.

**Parameters:**
- `vector3d` (Geom::Vector3d, Array(Float, Float, Float))

**Returns:** `Geom::Vector3d` — 

[Docs](https://ruby.sketchup.com/Geom/Vector3d.html#*-instance_method)

#### `+(vector3d) => Geom::Vector3d`

The #+ method is used to add a vector to this one.

**Remarks:** The #+ method is used to add a vector to this one.

**Parameters:**
- `vector3d` (Geom::Vector3d, Array(Float, Float, Float))

**Returns:** `Geom::Vector3d` — the new vector.

[Docs](https://ruby.sketchup.com/Geom/Vector3d.html#+-instance_method)

#### `-(vector3d) => Geom::Vector3d`

The #- method is used to subtract a vector from this one.

**Remarks:** The #- method is used to subtract a vector from this one.

**Parameters:**
- `vector3d` (Geom::Vector3d, Array(Float, Float, Float))

**Returns:** `Geom::Vector3d` — the new vector.

[Docs](https://ruby.sketchup.com/Geom/Vector3d.html#--instance_method)

#### `<(vector3d) => Boolean`

The #< compare method is used to compare two vectors to determine if the left-hand vector is less than the right-hand vector.

**Remarks:** The #< compare method is used to compare two vectors to determine if the left-hand vector is less than the right-hand vector.

**Parameters:**
- `vector3d` (Geom::Vector3d, Array(Float, Float, Float))

**Returns:** `Boolean` — true if the vector1 is closer to origin than vector2

[Docs](https://ruby.sketchup.com/Geom/Vector3d.html#<-instance_method)

#### `angle_between(vector3d) => Float`

The #angle_between method is used to compute the angle (in radians) between this vector and another vector.

**Remarks:** The #angle_between method is used to compute the angle (in radians) between this vector and another vector.

**Parameters:**
- `vector3d` (Geom::Vector3d, Array(Float, Float, Float))

**Returns:** `Float` — an angle (in radians)

[Docs](https://ruby.sketchup.com/Geom/Vector3d.html#angle_between-instance_method)

#### `axes => Array(Geom::Vector3d, Geom::Vector3d, Geom::Vector3d)`

The #axes method is used to compute an arbitrary set of axes with the given vector as the z-axis direction.

**Remarks:** The #axes method is used to compute an arbitrary set of axes with the given vector as the z-axis direction. Returns an Array of three vectors [xaxis, yaxis, zaxis] Vector3d objects

**Returns:** `Array(Geom::Vector3d, Geom::Vector3d, Geom::Vector3d)` — an Array object containing three

[Docs](https://ruby.sketchup.com/Geom/Vector3d.html#axes-instance_method)

#### `clone => Geom::Vector3d`

The #clone method is used to make a copy of a vector.

**Remarks:** The #clone method is used to make a copy of a vector.

**Returns:** `Geom::Vector3d` — a Vector3d object which is the clone of vector

[Docs](https://ruby.sketchup.com/Geom/Vector3d.html#clone-instance_method)

#### `cross(vector3d) => Geom::Vector3d`

The #cross method is used to compute the cross product between two vectors.

**Remarks:** The #cross method is used to compute the cross product between two vectors. The cross product, also called the vector product, is an operation on two vectors. The cross product of two vectors produces a third vector which is perpendicular to the plane in which the first two lie.

**Parameters:**
- `vector3d` (Geom::Vector3d, Array(Float, Float, Float))

**Returns:** `Geom::Vector3d` — 

[Docs](https://ruby.sketchup.com/Geom/Vector3d.html#cross-instance_method)

#### `dot(vector3d) => Float`

The #dot method is used to compute the dot product between two vectors.

**Remarks:** The #dot method is used to compute the dot product between two vectors.

**Parameters:**
- `vector3d` (Geom::Vector3d, Array(Float, Float, Float))

**Returns:** `Float` — 

[Docs](https://ruby.sketchup.com/Geom/Vector3d.html#dot-instance_method)

#### `inspect => Geom::Vector3d`

The #inspect method is used to inspect the contents of a vector as a friendly string.

**Remarks:** The #inspect method is used to inspect the contents of a vector as a friendly string.

**Returns:** `Geom::Vector3d` — the Vector3d object

[Docs](https://ruby.sketchup.com/Geom/Vector3d.html#inspect-instance_method)

#### `linear_combination(weight1, vector1, weight2, vector2) => Geom::Vector3d | linear_combination(x, xaxis, y, yaxis, z, zaxis) => Geom::Vector3d`

The Geom#linear_combination method is used to create a new vector as a linear combination of other vectors.

**Remarks:** The Geom#linear_combination method is used to create a new vector as a linear combination of other vectors. This method is generally used to get a vector at some percentage between two vectors. A linear combination is a standard term for vector math. It is defined as vector = weight1 * vector1 + weight2 * vector2

**Parameters:**
- `weight1` (Float) — weights
- `vector1` (Geom::Vector3d) — The first vector.
- `weight2` (Float) — weights
- `vector2` (Geom::Vector3d) — The second vector.

**Returns:** `Geom::Vector3d` — 

[Docs](https://ruby.sketchup.com/Geom/Vector3d.html#linear_combination-class_method)

#### `normalize => Geom::Vector3d`

The #normalize method is used to return a vector that is a unit vector of another.

**Remarks:** The #normalize method is used to return a vector that is a unit vector of another.

**Returns:** `Geom::Vector3d` — a new normalized Vector3d object

[Docs](https://ruby.sketchup.com/Geom/Vector3d.html#normalize-instance_method)

#### `normalize! => Geom::Vector3d`

The #normalize! method is used to convert a vector into a unit vector, in place.

**Remarks:** The #normalize! method is used to convert a vector into a unit vector, in place. Another way to do this is vector.length = 1.0

**Returns:** `Geom::Vector3d` — a normalized Vector3d object

[Docs](https://ruby.sketchup.com/Geom/Vector3d.html#normalize!-instance_method)

#### `parallel?(vector3d) => Boolean`

The #parallel? method determines if two Geom::Vector3ds are parallel within a tolerance.

**Remarks:** The #parallel? method determines if two Geom::Vector3ds are parallel within a tolerance. Two vectors are parallel if there exists a scalar multiple between them.

**Parameters:**
- `vector3d` (Geom::Vector3d, Array(Float, Float, Float))

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Geom/Vector3d.html#parallel?-instance_method)

#### `perpendicular?(vector3d) => Boolean`

The #perpendicular? method determines if two Geom::Vector3ds are perpendicular within a tolerance.

**Remarks:** The #perpendicular? method determines if two Geom::Vector3ds are perpendicular within a tolerance. Two vectors are considered perpendicular if their dot product is zero.

**Parameters:**
- `vector3d` (Geom::Vector3d, Array(Float, Float, Float))

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Geom/Vector3d.html#perpendicular?-instance_method)

#### `reverse => Geom::Vector3d`

The #reverse method is used to return a new vector that is the reverse of this vector, while leaving the original unchanged.

**Remarks:** The #reverse method is used to return a new vector that is the reverse of this vector, while leaving the original unchanged.

**Returns:** `Geom::Vector3d` — a reverse Vector3d object

[Docs](https://ruby.sketchup.com/Geom/Vector3d.html#reverse-instance_method)

#### `reverse! => Geom::Vector3d`

The #reverse! method is used to reverse the vector in place.

**Remarks:** The #reverse! method is used to reverse the vector in place.

**Returns:** `Geom::Vector3d` — a reverse Vector3d object

[Docs](https://ruby.sketchup.com/Geom/Vector3d.html#reverse!-instance_method)

#### `samedirection?(vector3d) => Boolean`

The #samedirection? method is used to determine if this vector is parallel to and in the same direction as another vector to within tolerance.

**Remarks:** The #samedirection? method is used to determine if this vector is parallel to and in the same direction as another vector to within tolerance.

**Parameters:**
- `vector3d` (Geom::Vector3d, Array(Float, Float, Float))

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Geom/Vector3d.html#samedirection?-instance_method)

#### `set!(vector) => Geom::Vector3d | set!(x, y, z) => Geom::Vector3d | set!(array3d) => Geom::Vector3d`

The #set! method is used to set the coordinates of the vector.

**Remarks:** The #set! method is used to set the coordinates of the vector.

**Parameters:**
- `vector` (Geom::Vector3d)

**Returns:** `Geom::Vector3d` — 

[Docs](https://ruby.sketchup.com/Geom/Vector3d.html#set!-instance_method)

#### `to_a => Array(Float, Float, Float)`

The #to_a method retrieves the coordinates of the vector in an Array[x, y, z].

**Remarks:** The #to_a method retrieves the coordinates of the vector in an Array[x, y, z].

**Returns:** `Array(Float, Float, Float)` — the coordinates of the vector in an array

[Docs](https://ruby.sketchup.com/Geom/Vector3d.html#to_a-instance_method)

#### `to_s => String`

The #to_s method is used to format the vector as a String.

**Remarks:** The #to_s method is used to format the vector as a String.

**Returns:** `String` — a string representation of vector

[Docs](https://ruby.sketchup.com/Geom/Vector3d.html#to_s-instance_method)

#### `transform(transform) => Geom::Vector3d`

The #transform method applies a Transformation to a vector, returning a new vector.

**Remarks:** The #transform method applies a Transformation to a vector, returning a new vector. The original vector is unchanged by this method.

**Parameters:**
- `transform` (Geom::Transformation) — A Transformation object to apply to the vector.

**Returns:** `Geom::Vector3d` — the newly transformed vector

[Docs](https://ruby.sketchup.com/Geom/Vector3d.html#transform-instance_method)

#### `transform!(transform) => Geom::Vector3d`

The #transform! method applies a Transformation to a vector.

**Remarks:** The #transform! method applies a Transformation to a vector. The vector itself is modified.

**Parameters:**
- `transform` (Geom::Transformation) — A Transformation object to apply to the vector.

**Returns:** `Geom::Vector3d` — the transformed vector

[Docs](https://ruby.sketchup.com/Geom/Vector3d.html#transform!-instance_method)

#### `unitvector? => Boolean`

The #unitvector? method is used to see if the vector is a unit vector.

**Remarks:** The #unitvector? method is used to see if the vector is a unit vector. This is equivalent to vector.length == 1.0

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Geom/Vector3d.html#unitvector?-instance_method)

#### `valid? => Boolean`

The #valid? method is used to verify if a vector is valid.

**Remarks:** The #valid? method is used to verify if a vector is valid. A vector is valid if its length is not zero.

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Geom/Vector3d.html#valid?-instance_method)

### Properties
- `=` (Geom::Vector3d, Array(Float, Float, Float), set) — The #== method is used to determine if two vectors are equal to within tolerance.
- `[]` (Length, get/set) — The #[] method is used to access the coordinates of a vector as if it was an Array.
- `length` (Length, get/set) — The #length method is used to retrieve the length of the vector.
- `x` (Float, get/set) — The #x method is used to retrieve the x coordinate of the vector.
- `y` (Float, get/set) — The #y method is used to retrieve the y coordinate of the vector.
- `z` (Float, get/set) — Get the #z coordinate of the vector.

