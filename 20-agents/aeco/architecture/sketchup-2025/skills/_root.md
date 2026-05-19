---
name: sketchup-
description: This skill encodes the sketchup 2025.0 surface of the  namespace — 10 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: Array, Geom, LanguageHandler, Layout, Length, Numeric, Sketchup, SketchupExtension, and 2 more types.
---

# 

Auto-generated from vendor docs for sketchup 2025.0. 10 types in this namespace.

## Array (class)

The SketchUp Array class adds additional methods to the standard Ruby Array class. Specifically, it contains methods allowing an array to behave just as a Geom::Vector3d or Geom::Point3d object (which can be thought of as arrays of 3 coordinate values). Therefore, you can use the Array class in place of a Geom::Point3d or Geom::Vector3d as a way to pass coordinate values.

[Vendor docs](https://ruby.sketchup.com/Array.html)

### Methods
#### `cross(vector) => Geom::Vector3d | cross(vector) => Geom::Vector2d`

The #cross method is used to compute the cross product between two vectors.

**Remarks:** The #cross method is used to compute the cross product between two vectors.

**Parameters:**
- `vector` (Geom::Vector3d)

**Returns:** `Geom::Vector3d` — 

[Docs](https://ruby.sketchup.com/Array.html#cross-instance_method)

#### `distance(point) => Length | distance(point) => Length`

The #distance method is used to compute the distance between two points.

**Remarks:** The #distance method is used to compute the distance between two points.

**Parameters:**
- `point` (Geom::Point3d)

**Returns:** `Length` — 

[Docs](https://ruby.sketchup.com/Array.html#distance-instance_method)

#### `distance_to_line(point, vector) => Length | distance_to_line(point1, point2) => Length`

The #distance_to_line method is used to compute the distance from a Geom::Point3d object to a line.

**Remarks:** The #distance_to_line method is used to compute the distance from a Geom::Point3d object to a line.

**Parameters:**
- `point` (Geom::Point3d)
- `vector` (Geom::Vector3d)

**Returns:** `Length` — 

[Docs](https://ruby.sketchup.com/Array.html#distance_to_line-instance_method)

#### `distance_to_plane(point, vector) => Length | distance_to_plane(point1, point2, point3) => Length | distance_to_plane(float1, float2, float3, float4) => Length | distance_to_plane(array) => Length | distance_to_plane(array) => Length | distance_to_plane(array) => Length`

The #distance_to_plane method is used to compute the distance from a Geom::Point3d object to a plane.

**Remarks:** The #distance_to_plane method is used to compute the distance from a Geom::Point3d object to a plane.

**Parameters:**
- `point` (Geom::Point3d)
- `vector` (Geom::Vector3d)

**Returns:** `Length` — The distance between the two points.

[Docs](https://ruby.sketchup.com/Array.html#distance_to_plane-instance_method)

#### `dot(vector) => Float | dot(vector) => Float`

The #dot method is used to compute the dot product between two vectors.

**Remarks:** The #dot method is used to compute the dot product between two vectors.

**Parameters:**
- `vector` (Geom::Vector3d)

**Returns:** `Float` — 

[Docs](https://ruby.sketchup.com/Array.html#dot-instance_method)

#### `normalize => Array(Float, Float, Float) | normalize => Array(Float, Float)`

Note: The arguments and return value will be converted to a floating point value.

**Remarks:** Note: The arguments and return value will be converted to a floating point value. (Unlike in the Geom::Vector3d#normalize! method.) The #normalize method is used to normalize a vector (setting its length to 1). It returns a new array rather than changing the original in place.

**Returns:** `Array(Float, Float, Float)` — an array object representing a vector

[Docs](https://ruby.sketchup.com/Array.html#normalize-instance_method)

#### `normalize! => Array(Float, Float, Float) | normalize! => Array(Float, Float)`

The #normalize! method is used to normalize a vector in place (setting its length to 1).

**Remarks:** The #normalize! method is used to normalize a vector in place (setting its length to 1).

**Returns:** `Array(Float, Float, Float)` — an array object representing a vector

[Docs](https://ruby.sketchup.com/Array.html#normalize!-instance_method)

#### `offset(vector) => Array(Length, Length, Length) | offset(vector) => Array(Length, Length) | offset(vector, length) => Array(Length, Length, Length) | offset(vector, length) => Array(Length, Length)`

The #offset method is used to offset a point by a vector.

**Remarks:** The #offset method is used to offset a point by a vector. it returns a new array rather than modifying the original in place.

**Parameters:**
- `vector` (Geom::Vector3d) — A Vector3d object used to offset the point.

**Returns:** `Array(Length, Length, Length)` — The newly offset array representing a point or vector.

[Docs](https://ruby.sketchup.com/Array.html#offset-instance_method)

#### `offset!(vector) => Array(Length, Length, Length) | offset!(vector) => Array(Length, Length) | offset!(vector, length) => Array(Length, Length, Length) | offset!(vector, length) => Array(Length, Length)`

The #offset! method is used to offset a point by a vector.

**Remarks:** The #offset! method is used to offset a point by a vector. The array is modified in place.

**Parameters:**
- `vector` (Geom::Vector3d) — A Vector3d object used to offset the point.

**Returns:** `Array(Length, Length, Length)` — The newly offset array representing a point or vector.

[Docs](https://ruby.sketchup.com/Array.html#offset!-instance_method)

#### `on_line?(point, vector) => Boolean | on_line?(point1, point2) => Boolean`

The #on_line? method is used to determine if a Geom::Point3d object is on a line.

**Remarks:** The #on_line? method is used to determine if a Geom::Point3d object is on a line.

**Parameters:**
- `point` (Geom::Point3d)
- `vector` (Geom::Vector3d)

**Returns:** `Boolean` — true if the point is on the line, false if the point is not on the line.

[Docs](https://ruby.sketchup.com/Array.html#on_line?-instance_method)

#### `on_plane?(point, vector) => Boolean | on_plane?(point1, point2, point3) => Boolean | on_plane?(float1, float2, float3, float4) => Boolean | on_plane?(array) => Boolean | on_plane?(array) => Boolean | on_plane?(array) => Boolean`

The #on_plane? method is used to determine if a Geom::Point3d object is on a plane (to within SketchUp's standard floating point tolerance).

**Remarks:** The #on_plane? method is used to determine if a Geom::Point3d object is on a plane (to within SketchUp's standard floating point tolerance).

**Parameters:**
- `point` (Geom::Point3d)
- `vector` (Geom::Vector3d)

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Array.html#on_plane?-instance_method)

#### `project_to_line(point, vector) => Array(Length, Length, Length) | project_to_line(point1, point2) => Array(Length, Length, Length)`

The #project_to_line method is used to retrieve the projection of a Geom::Point3d object onto a line.

**Remarks:** The #project_to_line method is used to retrieve the projection of a Geom::Point3d object onto a line.

**Parameters:**
- `point` (Geom::Point3d)
- `vector` (Geom::Vector3d)

**Returns:** `Array(Length, Length, Length)` — A new point on the line that is closest to this point

[Docs](https://ruby.sketchup.com/Array.html#project_to_line-instance_method)

#### `project_to_plane(point, vector) => Array(Length, Length, Length) | project_to_plane(point1, point2, point3) => Array(Length, Length, Length) | project_to_plane(float1, float2, float3, float4) => Array(Length, Length, Length) | project_to_plane(array) => Array(Length, Length, Length) | project_to_plane(array) => Array(Length, Length, Length) | project_to_plane(array) => Array(Length, Length, Length)`

The #project_to_plane method retrieves the projection of a Geom::Point3d onto a plane.

**Remarks:** The #project_to_plane method retrieves the projection of a Geom::Point3d onto a plane.

**Parameters:**
- `point` (Geom::Point3d)
- `vector` (Geom::Vector3d)

**Returns:** `Array(Length, Length, Length)` — 

[Docs](https://ruby.sketchup.com/Array.html#project_to_plane-instance_method)

#### `transform(transform) => Array<Length, Length> | transform(transform) => Array<Length, Length, Length>`

The #transform method is used to apply a Geom::Transformation or Geom::Transformation2d object to a Geom::Point3d or Geom::Point2d object defined by an Array object.

**Remarks:** The #transform method is used to apply a Geom::Transformation or Geom::Transformation2d object to a Geom::Point3d or Geom::Point2d object defined by an Array object. This method returns a new Array object instead of modifying the original.

**Parameters:**
- `transform` (Geom::Transformation2d)

**Returns:** `Array<Length, Length>` — The newly transformed point.

[Docs](https://ruby.sketchup.com/Array.html#transform-instance_method)

#### `transform!(transform) => Array | transform!(transform) => Array`

Note: This method modifies the original.

**Remarks:** Note: This method modifies the original. The #transform! method is used to apply a Geom::Transformation object to a Geom::Point3d object defined by an Array object.

**Parameters:**
- `transform` (Geom::Transformation2d)

**Returns:** `Array` — The newly transformed point.

[Docs](https://ruby.sketchup.com/Array.html#transform!-instance_method)

#### `vector_to(point) => Geom::Vector3d | vector_to(point) => Geom::Vector2d`

The #vector_to method is used to create an array as a vector from one point to a second point.

**Remarks:** The #vector_to method is used to create an array as a vector from one point to a second point.

**Parameters:**
- `point` (Geom::Point3d)

**Returns:** `Geom::Vector3d` — 

[Docs](https://ruby.sketchup.com/Array.html#vector_to-instance_method)

### Properties
- `x` (Object, nil, get/set) — The #x method retrieves the x coordinate.
- `y` (Object, nil, get/set) — The #y method retrieves the y coordinate.
- `z` (Object, nil, get/set) — The #z method retrieves the z coordinate.

## Geom (interface)

Note: Lines and Planes are infinite. The Geom module defines a number of Module methods that let you perform different geometric operations. The methods in this module take lines and planes as arguments. There is no special class for representing lines or planes. Arrays are used for both. A line can be represented as either an Array of a point and a vector, or as an Array of two points. A plane can be represented as either an Array of a point and a vector, or as an Array of 4 numbers that give the coefficients of a plane equation. There are several good books on 3D math if you are new to the concepts of a line, plane, and vector.

[Vendor docs](https://ruby.sketchup.com/Geom.html)

### Methods
#### `closest_points(line1, line2) => Array(Geom::Point3d, Geom::Point3d)`

The closest_points method is used to compute the closest points on two lines.

**Remarks:** The closest_points method is used to compute the closest points on two lines. line.

**Parameters:**
- `line1` (Array(Geom::Point3d, Geom::Vector3d)) — The first line to intersect
- `line2` (Array(Geom::Point3d, Geom::Vector3d)) — The second line to intersect

**Returns:** `Array(Geom::Point3d, Geom::Point3d)` — An array of two points. The first point is on the first line and the second point is on the second

[Docs](https://ruby.sketchup.com/Geom.html#closest_points-class_method)

#### `fit_plane_to_points(point1, point2, point3, ...) => Array(Float, Float, Float, Float) | fit_plane_to_points(points) => Array(Float, Float, Float, Float)`

The fit_plane_to_points method is used to compute a plane that is a best fit to an array of points.

**Remarks:** The fit_plane_to_points method is used to compute a plane that is a best fit to an array of points. If more than three points are given some of the points may not be on the plane. The plane is returned as an Array of 4 numbers which are the coefficients of the plane equation Ax + By + Cz + D = 0.

**Parameters:**
- `point1` (Geom::Point3d)
- `point2` (Geom::Point3d)
- `point3` (Geom::Point3d)

**Returns:** `Array(Float, Float, Float, Float)` — A plane

[Docs](https://ruby.sketchup.com/Geom.html#fit_plane_to_points-class_method)

#### `intersect_line_line(line1, line2) => Geom::Point3d?`

The intersect_line_line computes the intersection of two lines.

**Remarks:** The intersect_line_line computes the intersection of two lines.

**Parameters:**
- `line1` (Array(Geom::Point3d, Geom::Vector3d)) — The first line to intersect.
- `line2` (Array(Geom::Point3d, Geom::Vector3d)) — The second line to intersect.

**Returns:** `Geom::Point3d, nil` — The intersection point. Returns nil if they do not intersect.

[Docs](https://ruby.sketchup.com/Geom.html#intersect_line_line-class_method)

#### `intersect_line_plane(line, plane) => Geom::Point3d?`

The intersect_line_plane method is used to compute the intersection of a line and a plane.

**Remarks:** The intersect_line_plane method is used to compute the intersection of a line and a plane.

**Parameters:**
- `line` (Array(Geom::Point3d, Geom::Vector3d))
- `plane` (Array(Geom::Point3d, Geom::Vector3d))

**Returns:** `Geom::Point3d, nil` — A Point3d object. Returns nil if they do not intersect.

[Docs](https://ruby.sketchup.com/Geom.html#intersect_line_plane-class_method)

#### `intersect_plane_plane(plane1, plane2) => Array(Geom::Point3d, Geom::Vector3d)`

The intersect_plane_plane method is used to compute the intersection of two planes.

**Remarks:** The intersect_plane_plane method is used to compute the intersection of two planes.

**Parameters:**
- `plane1` (Array(Geom::Point3d, Geom::Vector3d)) — The first plane to intersect
- `plane2` (Array(Geom::Point3d, Geom::Vector3d)) — The second plane to intersect

**Returns:** `Array(Geom::Point3d, Geom::Vector3d)` — A line where the planes intersect if successful. Returns nil if the planes do not intersect.

[Docs](https://ruby.sketchup.com/Geom.html#intersect_plane_plane-class_method)

#### `linear_combination(weight1, point1, weight2, point2) => Geom::Point3d | linear_combination(weight1, vector1, weight2, vector2) => Geom::Vector3d`

The linear_combination method is used to compute the linear combination of points or vectors.

**Remarks:** The linear_combination method is used to compute the linear combination of points or vectors. A linear combination is a standard term for vector math. It is defined as vector = weight1 * vector1 + weight2 * vector2.

**Parameters:**
- `weight1` (Float)
- `point1` (Geom::Point3d)
- `weight2` (Float)
- `point2` (Geom::Point3d)

**Returns:** `Geom::Point3d` — 

[Docs](https://ruby.sketchup.com/Geom.html#linear_combination-class_method)

#### `point_in_polygon_2D(point, polygon, check_border) => Boolean`

The point_in_polygon_2D method is used to determine whether a point is inside a polygon.

**Remarks:** The point_in_polygon_2D method is used to determine whether a point is inside a polygon. The z component of both the point you're checking and the points in the polygon are ignored, effectively making it a 2-d check.

**Parameters:**
- `point` (Geom::Point3d)
- `polygon` (Array<Geom::Point3d>) — An array of points that represent the corners of the polygon you are checking against.
- `check_border` (Boolean) — Pass true if a point on the border should be counted as inside the polygon.

**Returns:** `Boolean` — true if the point is inside the polygon.

[Docs](https://ruby.sketchup.com/Geom.html#point_in_polygon_2D-class_method)

#### `tesselate(polygon_loop_points, *inner_loop_points) => Array<Geom::Point3d>`

Note: The winding order of the polygons matter.

**Remarks:** Note: The winding order of the polygons matter. The outer loop should be in counter-clockwise order. To cut an empty hole the inner loops should be in clockwise order, otherwise they will create a loop filled with triangles. Note: The tesselation is using the same logic as SketchUp its rendering pipeline. But the exact result is an implementation detail which may change between versions. In particularly the results of degenerate polygons and non-planar polygons is undefined as part of the API contract. Such polygons are for example polygons where all points are colinear, polygons with duplicate points, non-planar polygons. Note: If you want the triangles from an existing Sketchup::Face it's better to use Sketchup::Face#mesh. Tessellates a polygon, represented as a collection of 3D points. Can include holes by providing collections of points describing the inner loops. This is intended to be used for planar polygons. Useful to draw concave polygons using Sketchup::View#draw or Sketchup::View#draw2d. It can also be useful for importers where the input format describes only the loops for a polygon and you want to work with a collection of triangles. Polygon with two holes, one empty and one filled: (See “Drawing a polygon with holes from a custom tool” example)

**Parameters:**
- `polygon_loop_points` (Array<Geom::Point3d>)
- `inner_loop_points` (Array<Array<Geom::Point3d>>)

**Returns:** `Array<Geom::Point3d>` — an array of points with a stride of three representing a set of triangles.

[Docs](https://ruby.sketchup.com/Geom.html#tesselate-class_method)

## LanguageHandler (class)

The LanguageHandler class contains methods used to help make SketchUp extensions easier to localize across different languages. It looks for translated resources within the Resources folder in the extension's directory structure. All translated resources should be located within the appropriate language folder and encoded in UTF-8. The strings file should include “key”=“value” string pairs in the following format: language code gocorp_swiveldriver/Resources/fr/swiveldriver.strings

[Vendor docs](https://ruby.sketchup.com/LanguageHandler.html)

### Constructors
- `LanguageHandler(filename)` — The new method is used to create a new LanguageHandler object.

### Methods
#### `[](key) => String`

Looks up and returns the localized version of a given string, based on the language SketchUp is currently running in, and the available translations in the Resources folder.

**Remarks:** Looks up and returns the localized version of a given string, based on the language SketchUp is currently running in, and the available translations in the Resources folder.

**Parameters:**
- `key` (String) — The key for the string to be retrieved.

**Returns:** `String` — the localized string.

[Docs](https://ruby.sketchup.com/LanguageHandler.html#[]-instance_method)

#### `resource_path => String`

Returns a string containing the path to the given filename if it can be found in the Resources folder.

**Remarks:** Returns a string containing the path to the given filename if it can be found in the Resources folder.

**Returns:** `String` — the location of the file in the Resources folder.

[Docs](https://ruby.sketchup.com/LanguageHandler.html#resource_path-instance_method)

#### `strings => Hash`

Returns a Hash object containing the localization dictionary.

**Remarks:** Returns a Hash object containing the localization dictionary.

**Returns:** `Hash` — the localization dictionary.

[Docs](https://ruby.sketchup.com/LanguageHandler.html#strings-instance_method)

## Layout (interface)

The LayOut module is the root of the LayOut Ruby API. Many of the classes in the API are implemented beneath this module.

[Vendor docs](https://ruby.sketchup.com/Layout.html)

## Length (class)

Note: Prior to SketchUp 2015, Length used to be derived from Float. This is no longer the case. Note: When serializing a Length object to a string to save for later use, e.g. in a config file, first convert them to Float objects. The string representation of a Length is rounded and uses the local decimal separator which can lead to data loss and portability issues. The string representation is intended for humans, not computers. Because length units are used so often in SketchUp, a special class has been added to make it easier to work with length values. You can use a Length object any place that you would use a Float. Internally, all lengths in SketchUp are stored in inches. The Length class stores values in inches as well. A number of methods have been added to the Ruby Numeric class to do units conversions. Find more info about units and lengths in this article. The setting for the Length Format and Length Unit can be retrieved from the Sketchup::Model#options by querying the “UnitsOptions” Sketchup::OptionsProvider for “LengthFormat” and “LengthUnit” respectively. Constants: Length Format Length::Decimal Length::Architectural Length::Engineering Length::Fractional Length Unit Length::Inches Length::Feet Length::Millimeter Length::Centimeter Length::Meter (Added in SketchUp 2020.0) Length::Yard Area Unit (Added in SketchUp 2019.2) Length::SquareInches Length::SquareFeet Length::SquareMillimeter Length::SquareCentimeter Length::SquareMeter (Added in SketchUp 2020.0) Length::SquareYard Volume Unit (Added in SketchUp 2019.2) Length::CubicInches Length::CubicFeet Length::CubicMillimeter Length::CubicCentimeter Length::CubicMeter (Added in SketchUp 2020.0) Length::CubicYard Length::Liter Length::USGallon

[Vendor docs](https://ruby.sketchup.com/Length.html)

### Methods
#### `<=>(length2) => Integer`

The <=> method is used to see if one length is less than equal or greater than another length.

**Remarks:** The <=> method is used to see if one length is less than equal or greater than another length. Because we change == for Length to do a test based on a tolerance, we also need to change <=> to also take tolerance into account.

**Parameters:**
- `length2` (Length) — A length value.

**Returns:** `Integer` — the result of the comparison

[Docs](https://ruby.sketchup.com/Length.html#<=>-instance_method)

#### `inspect => String`

The inspect method is used to retrieve an unformatted string for the length, which is the length in inches, regardless of the user's model unit settings.

**Remarks:** The inspect method is used to retrieve an unformatted string for the length, which is the length in inches, regardless of the user's model unit settings. See Length.to_s for a way automatically format your Length to the user's model units.

**Returns:** `String` — an unformatted length string

[Docs](https://ruby.sketchup.com/Length.html#inspect-instance_method)

#### `to_f => Float`

The to_f method is used to convert a length to a normal float.

**Remarks:** The to_f method is used to convert a length to a normal float.

**Returns:** `Float` — the float length value

[Docs](https://ruby.sketchup.com/Length.html#to_f-instance_method)

#### `to_s => String`

Format a length as a String using the current units formatting settings for the model.

**Remarks:** Format a length as a String using the current units formatting settings for the model. (So if the user's model is set to feet, this method will return a nicely formatted length in feet.)

**Returns:** `String` — the float length value

[Docs](https://ruby.sketchup.com/Length.html#to_s-instance_method)

### Properties
- `<` (Boolean, get/set) — The < method is used to see if one length is less than another length.
- `=` (Length, set) — The == method is used to see if one length is equal to another length.
- `>` (Boolean, get/set) — The > method is used to see if one length is greater than another length.
- `Decimal` (Object, get) — 
- `Architectural` (Object, get) — 
- `Engineering` (Object, get) — 
- `Fractional` (Object, get) — 
- `Inches` (Object, get) — 
- `Feet` (Object, get) — 
- `Millimeter` (Object, get) — 
- `Centimeter` (Object, get) — 
- `Meter` (Object, get) — 
- `Yard` (Object, get) — 
- `SquareInches` (Object, get) — 
- `SquareFeet` (Object, get) — 
- `SquareMillimeter` (Object, get) — 
- `SquareCentimeter` (Object, get) — 
- `SquareMeter` (Object, get) — 
- `SquareYard` (Object, get) — 
- `CubicInches` (Object, get) — 
- `CubicFeet` (Object, get) — 
- `CubicMillimeter` (Object, get) — 
- `CubicCentimeter` (Object, get) — 
- `CubicMeter` (Object, get) — 
- `CubicYard` (Object, get) — 
- `Liter` (Object, get) — 
- `USGallon` (Object, get) — 

## Numeric (class)

A number of methods have been added to the Ruby Numeric class to do units conversions.

[Vendor docs](https://ruby.sketchup.com/Numeric.html)

### Methods
#### `cm => Length`

The cm method is used to convert from centimeters to inches.

**Remarks:** The cm method is used to convert from centimeters to inches.

**Returns:** `Length` — a value in inches if successful

[Docs](https://ruby.sketchup.com/Numeric.html#cm-instance_method)

#### `degrees => Float`

The degrees method is used to convert from degrees to radians.

**Remarks:** The degrees method is used to convert from degrees to radians. For example 90.degrees would return 1.5707963267949

**Returns:** `Float` — a value in radians if successful

[Docs](https://ruby.sketchup.com/Numeric.html#degrees-instance_method)

#### `feet => Length`

The feet method is used to convert from feet to inches.

**Remarks:** The feet method is used to convert from feet to inches.

**Returns:** `Length` — a value in inches if successful

[Docs](https://ruby.sketchup.com/Numeric.html#feet-instance_method)

#### `inch => Length`

The to_l is used to convert from a number to a length.

**Remarks:** The to_l is used to convert from a number to a length.

**Returns:** `Length` — a Length object if successful

[Docs](https://ruby.sketchup.com/Numeric.html#inch-instance_method)

#### `km => Length`

The km method is used to convert from kilometers to inches.

**Remarks:** The km method is used to convert from kilometers to inches.

**Returns:** `Length` — a value in inches if successful

[Docs](https://ruby.sketchup.com/Numeric.html#km-instance_method)

#### `m => Length`

The m method is used to convert from meters to inches.

**Remarks:** The m method is used to convert from meters to inches.

**Returns:** `Length` — a value in inches if successful

[Docs](https://ruby.sketchup.com/Numeric.html#m-instance_method)

#### `mile => Length`

The mile method is used to convert from miles to inches.

**Remarks:** The mile method is used to convert from miles to inches.

**Returns:** `Length` — a value in inches if successful

[Docs](https://ruby.sketchup.com/Numeric.html#mile-instance_method)

#### `mm => Length`

The mm method is used to convert from millimeters to inches.

**Remarks:** The mm method is used to convert from millimeters to inches. It returns a Length value which is the same length as the given value. For example len = 25.4.mm returns 1 inch.

**Returns:** `Length` — a value in millimeters if successful

[Docs](https://ruby.sketchup.com/Numeric.html#mm-instance_method)

#### `radians => Float`

The radians method is used to convert from radians to degrees.

**Remarks:** The radians method is used to convert from radians to degrees. For example, 1.5707963267949.radians would return 90.0

**Returns:** `Float` — a value in degrees if successful

[Docs](https://ruby.sketchup.com/Numeric.html#radians-instance_method)

#### `to_cm => Float`

The to_cm method is used to convert from inches to centimeters.

**Remarks:** The to_cm method is used to convert from inches to centimeters.

**Returns:** `Float` — a value in centimeters if successful

[Docs](https://ruby.sketchup.com/Numeric.html#to_cm-instance_method)

#### `to_feet => Float`

The to_feet method is used to convert from inches to feet.

**Remarks:** The to_feet method is used to convert from inches to feet.

**Returns:** `Float` — a value in feet if successful

[Docs](https://ruby.sketchup.com/Numeric.html#to_feet-instance_method)

#### `to_inch => Float`

The to_inch method converts from inches to inches.

**Remarks:** The to_inch method converts from inches to inches. This does not change the value.

**Returns:** `Float` — a value in inches if successful

[Docs](https://ruby.sketchup.com/Numeric.html#to_inch-instance_method)

#### `to_km => Float`

The to_km method is used to convert from inches to kilometers.

**Remarks:** The to_km method is used to convert from inches to kilometers.

**Returns:** `Float` — a value in kilometers if successful

[Docs](https://ruby.sketchup.com/Numeric.html#to_km-instance_method)

#### `to_l => Length`

The to_l is used to convert from a number to a length.

**Remarks:** The to_l is used to convert from a number to a length.

**Returns:** `Length` — a Length object if successful

[Docs](https://ruby.sketchup.com/Numeric.html#to_l-instance_method)

#### `to_m => Float`

The to_m method is used to convert from inches to meters.

**Remarks:** The to_m method is used to convert from inches to meters.

**Returns:** `Float` — a value in meters if successful

[Docs](https://ruby.sketchup.com/Numeric.html#to_m-instance_method)

#### `to_mile => Float`

The to_mile method is used to convert from inches to miles.

**Remarks:** The to_mile method is used to convert from inches to miles.

**Returns:** `Float` — a value in miles if successful

[Docs](https://ruby.sketchup.com/Numeric.html#to_mile-instance_method)

#### `to_mm => Float`

The to_mm method is used to convert from inches to millimeters.

**Remarks:** The to_mm method is used to convert from inches to millimeters.

**Returns:** `Float` — a value in millimeters if successful

[Docs](https://ruby.sketchup.com/Numeric.html#to_mm-instance_method)

#### `to_yard => Float`

The to_yard method is used to convert from inches to yards.

**Remarks:** The to_yard method is used to convert from inches to yards.

**Returns:** `Float` — a value in yards if successful

[Docs](https://ruby.sketchup.com/Numeric.html#to_yard-instance_method)

#### `yard => Length`

The yard method is used to convert from yards to inches.

**Remarks:** The yard method is used to convert from yards to inches.

**Returns:** `Length` — a value in inches if successful

[Docs](https://ruby.sketchup.com/Numeric.html#yard-instance_method)

## Sketchup (interface)

The Sketchup module contains a number of important utility methods for use in your Ruby scripts. Many of the classes in the API are implemented beneath this module. You can think of the Sketchup module as the “root” of the application tree. Most ruby calls start from the currently active model, and this is accessed via the Sketchup.active_model method.

[Vendor docs](https://ruby.sketchup.com/Sketchup.html)

### Methods
#### `active_model => Sketchup::Model`

The active_model method returns the currently active SketchUp model.

**Remarks:** The active_model method returns the currently active SketchUp model. On the PC, this is the only model that one can have access to via the API, but Macintosh versions of SketchUp can have multiple models open at once, in which case the method will return the model that the user currently has focused.

**Returns:** `Sketchup::Model` — active model object if successful, false if unsuccessful

[Docs](https://ruby.sketchup.com/Sketchup.html#active_model-class_method)

#### `add_observer(observer) => Boolean`

The add_observer method is used to add an observer to the current object.

**Remarks:** The add_observer method is used to add an observer to the current object.

**Parameters:**
- `observer` (Object) — An observer.

**Returns:** `Boolean` — true if successful, false if unsuccessful.

[Docs](https://ruby.sketchup.com/Sketchup.html#add_observer-class_method)

#### `app_name => String`

The app_name method is used to retrieve the current application name.

**Remarks:** The app_name method is used to retrieve the current application name.

**Returns:** `String` — the name of the application, either “SketchUp Pro” or “SketchUp”. Note: For versions earlier than SketchUp8 M4 (Mac 8.0.15157 and Windows 8.0.15158) this function will return “Google SketchUp Pro” or “Google SketchUp”.

[Docs](https://ruby.sketchup.com/Sketchup.html#app_name-class_method)

#### `break_edges=(enabled) => Boolean`

The break_edges= method can be used to disable or enable the break edges feature.

**Remarks:** The break_edges= method can be used to disable or enable the break edges feature. Break edges is the SketchUp 7 feature that automatically splits edges that the user draws which cross over one another. This feature is always on by default and cannot be disabled by the user via the user interface, but you can call this method to disable it. Be cautious in doing so, however, as the resulting model could then be altered when the user later draws lines into it with the break edges feature reactivated.

**Parameters:**
- `enabled` (Boolean) — If true, break edges will be turned on. If false, it will be deactivated.

**Returns:** `Boolean` — true if break edges was turned on.

[Docs](https://ruby.sketchup.com/Sketchup.html#break_edges=-class_method)

#### `break_edges? => Boolean`

The break_edges? method indicates whether the break edges feature is currently turned on.

**Remarks:** The break_edges? method indicates whether the break edges feature is currently turned on. Break edges is the SketchUp 7 feature that automatically splits edges that the user draws which cross over one another. This feature is always on by default and cannot be disabled by the user via the user interface.

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Sketchup.html#break_edges?-class_method)

#### `create_texture_writer => Sketchup::TextureWriter`

The create_texture_writer method is used to create a TextureWriter object.

**Remarks:** The create_texture_writer method is used to create a TextureWriter object.

**Returns:** `Sketchup::TextureWriter` — a texturewriter object if successful.

[Docs](https://ruby.sketchup.com/Sketchup.html#create_texture_writer-class_method)

#### `debug_mode=(enabled) => Boolean`

Note: Changing this value within your extension can cause problems for other extension developers who rely on the debug information for their own work.

**Remarks:** Note: Changing this value within your extension can cause problems for other extension developers who rely on the debug information for their own work. Only use this locally; never change this value in an extension you publish. The debug_mode= method lets you controls whether SketchUp will output warnings to the console when it detects incorrect usage of the API. The setting takes effect right away, no need to restart SketchUp.

**Parameters:**
- `enabled` (Boolean) — If true, SketchUp will produce debug warnings.

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Sketchup.html#debug_mode=-class_method)

#### `debug_mode? => Boolean`

The debug_mode? controls whether SketchUp will output warnings to the console when it detects incorrect usage of the API.

**Remarks:** The debug_mode? controls whether SketchUp will output warnings to the console when it detects incorrect usage of the API.

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Sketchup.html#debug_mode?-class_method)

#### `display_name_from_action(action_name) => String`

Note: This method has been non-functional on Mac since SketchUp 8.

**Remarks:** Note: This method has been non-functional on Mac since SketchUp 8. The display_name_from_action method is used to gets a user-friendly name from an action string. See Sketchup.send_action for a list of valid action strings.

**Parameters:**
- `action_name` (String) — An action string.

**Returns:** `String` — a friendly name.

[Docs](https://ruby.sketchup.com/Sketchup.html#display_name_from_action-class_method)

#### `extensions => Sketchup::ExtensionsManager`

Returns the ExtensionsManager where you can find all registered SketchupExtension objects.

**Remarks:** Returns the ExtensionsManager where you can find all registered SketchupExtension objects. #Sketchup.extensionextension.loaded?“

**Returns:** `Sketchup::ExtensionsManager` — an ExtensionsManager object.

[Docs](https://ruby.sketchup.com/Sketchup.html#extensions-class_method)

#### `file_new => Module`

The file_new method is used to create a new file.

**Remarks:** The file_new method is used to create a new file.

**Returns:** `Module` — The Sketchup module.

[Docs](https://ruby.sketchup.com/Sketchup.html#file_new-class_method)

#### `find_support_file(filename, directory) => String`

The find_support_files method is used to retrieve the relative path and name of a file within the SketchUp installation directory.

**Remarks:** The find_support_files method is used to retrieve the relative path and name of a file within the SketchUp installation directory. Forward slashes must be used to delimit between directory names.

**Parameters:**
- `filename` (String) — Name of the filename you want to find.
- `directory` (String) — directory relative to the SketchUp installation directory.

**Returns:** `String` — the entire path if successful. If unsuccessful, the method returns false.

[Docs](https://ruby.sketchup.com/Sketchup.html#find_support_file-class_method)

#### `find_support_files(filename, directory) => Array<String>`

The find_support_files method is used to retrieve the path and name of all matching files within the SketchUp installation directory.

**Remarks:** The find_support_files method is used to retrieve the path and name of all matching files within the SketchUp installation directory. Forward slashes must be used to delimit between directory names.

**Parameters:**
- `filename` (String) — Extension of the files to be found.
- `directory` (String) — directory relative to the SketchUp installation directory. Without this the result will be empty.

**Returns:** `Array<String>` — an array of files. If unsuccessful, the method returns false.

[Docs](https://ruby.sketchup.com/Sketchup.html#find_support_files-class_method)

#### `fix_shadow_strings=(enabled) => Boolean`

The fix_shadow_strings= method lets you control whether shadow rendering attempts to fix an artifact commonly referred to as “strings”.

**Remarks:** The fix_shadow_strings= method lets you control whether shadow rendering attempts to fix an artifact commonly referred to as “strings”. The fix is actually very model dependent and not controllable from the UI, so this method can be used to control it.

**Parameters:**
- `enabled` (Boolean) — If true, shadow strings fix will be turned on. If false, it will be deactivated.

**Returns:** `Boolean` — true if shadow strings fix was turned on.

[Docs](https://ruby.sketchup.com/Sketchup.html#fix_shadow_strings=-class_method)

#### `fix_shadow_strings? => Boolean`

The fix_shadow_strings? method indicates whether the a fix for a shadow rendering artifact commonly referred to as “strings” is enabled.

**Remarks:** The fix_shadow_strings? method indicates whether the a fix for a shadow rendering artifact commonly referred to as “strings” is enabled. The fix is actually very model dependent and not controllable from the UI, so this method can be used to test it.

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Sketchup.html#fix_shadow_strings?-class_method)

#### `focus => Object`

The focus method is used to focus the active model window.

**Remarks:** The focus method is used to focus the active model window.

[Docs](https://ruby.sketchup.com/Sketchup.html#focus-class_method)

#### `format_angle(number) => String`

The format_angle method takes a number as an angle in radians and formats it into degrees.

**Remarks:** The format_angle method takes a number as an angle in radians and formats it into degrees. For example, format_angle(Math::PI) will return 180.0.

**Parameters:**
- `number` (Numeric) — A number to be formatted.

**Returns:** `String` — an angle in degrees if successful, false if unsuccessful

[Docs](https://ruby.sketchup.com/Sketchup.html#format_angle-class_method)

#### `format_area(number) => String`

The format_area method formats a number as an area using the current units settings.

**Remarks:** The format_area method formats a number as an area using the current units settings. The number must be in square inches.

**Parameters:**
- `number` (Numeric) — A number to be formatted.

**Returns:** `String` — 

[Docs](https://ruby.sketchup.com/Sketchup.html#format_area-class_method)

#### `format_degrees(number) => String`

The format_degrees method formats a number as an angle given in degrees.

**Remarks:** The format_degrees method formats a number as an angle given in degrees. For example, 10 becomes 10.0. This is the equivalent to a to_f call.

**Parameters:**
- `number` (Numeric) — A number to be formatted.

**Returns:** `String` — degrees if successful, false if unsuccessful.

[Docs](https://ruby.sketchup.com/Sketchup.html#format_degrees-class_method)

#### `format_length(number) => String | format_length(number, precision) => String`

The format_length method formats a number as a length using the current units settings.

**Remarks:** The format_length method formats a number as a length using the current units settings. The default unit setting is inches. For example, 10 becomes 10“.

**Parameters:**
- `number` (Numeric) — A number to be formatted.

**Returns:** `String` — 

[Docs](https://ruby.sketchup.com/Sketchup.html#format_length-class_method)

#### `format_volume(number) => String`

The format_volume method formats a number as a volume using the current units settings.

**Remarks:** The format_volume method formats a number as a volume using the current units settings. The number must be in cubic inches.

**Parameters:**
- `number` (Numeric) — A number to be formatted.

**Returns:** `String` — 

[Docs](https://ruby.sketchup.com/Sketchup.html#format_volume-class_method)

#### `get_datfile_info(key, default_value) => String`

The get_datfile_info method is used to retrieve the value for the given key from Sketchup.

**Remarks:** The get_datfile_info method is used to retrieve the value for the given key from Sketchup.dat. If the key is not found, default_value is returned.

**Parameters:**
- `key` (String) — The key whose value you want to retrieve.
- `default_value` (String) — The default value you want returned if key is not available.

**Returns:** `String` — a string value if successful.

[Docs](https://ruby.sketchup.com/Sketchup.html#get_datfile_info-class_method)

#### `get_i18n_datfile_info(key, default_value) => String`

The get_i18n_datfile_info method is used to retrieve the value for the given key from the internationalization file that SketchUp uses to work in multiple languages.

**Remarks:** The get_i18n_datfile_info method is used to retrieve the value for the given key from the internationalization file that SketchUp uses to work in multiple languages. If the key is not found, default_value is returned.

**Parameters:**
- `key` (String) — The key whose value you want to retrieve.
- `default_value` (String) — The default value you want returned if key is not available.

**Returns:** `String` — 

[Docs](https://ruby.sketchup.com/Sketchup.html#get_i18n_datfile_info-class_method)

#### `get_locale => String`

The os_language method returns the language code for the language SketchUp is running in.

**Remarks:** The os_language method returns the language code for the language SketchUp is running in. This is an alias for the get_locale method. Examples of return values are: en-US, fr, it, de, es, ja, ko, zh-CN, zh-TW, pt-BR, nl, ru and sv. For an up to date list os supported languages, see the SketchUp download page.

**Returns:** `String` — a code representing the language SketchUp is displaying.

[Docs](https://ruby.sketchup.com/Sketchup.html#get_locale-class_method)

#### `get_resource_path(filename) => String`

The get_resource_path is used to retrieve the directory where “resource” files are stored by SketchUp.

**Remarks:** The get_resource_path is used to retrieve the directory where “resource” files are stored by SketchUp. Resource files include things like language localization files.

**Parameters:**
- `filename` (String) — The filename of a resource file in the resource directory hierarchy.

**Returns:** `String` — the directory path to the resources folder.

[Docs](https://ruby.sketchup.com/Sketchup.html#get_resource_path-class_method)

#### `get_shortcuts => Array<String>`

The get_shortcuts method retrieves an array of all keyboard shortcuts currently registered with SketchUp.

**Remarks:** The get_shortcuts method retrieves an array of all keyboard shortcuts currently registered with SketchUp. Each shortcut is returned as a string with the shortcut and the command separated by a tab, similar to “Ctrl+A\tEdit/Select All”

**Returns:** `Array<String>` — an array of shortcut strings.

[Docs](https://ruby.sketchup.com/Sketchup.html#get_shortcuts-class_method)

#### `install_from_archive(filepath, show_warning = true) => Boolean`

Installs the contents of a ZIP archive file into SketchUp's Plugins folder.

**Remarks:** Installs the contents of a ZIP archive file into SketchUp's Plugins folder. If the ZIP contains subfolders, these will be preserved. This allows for a Ruby API plugin or Extension developer to distribute their plugin as a single file regardless of how many asset files must be included. The user will be shown a warning message that they must agree to before the install proceeds. If they do not agree, an Interrupt error will be raised. If the user does agree but there is a problem with the unzip process, an Exception will be raised. You can capture these states via a begin/rescue. See the example below. If the install is successful, any Ruby files that have been added to the Plugins folder will immediately be executed, saving the user a restart. To create an archive file, use your favorite tool (7zip, Winzip, etc.) to zip up any files and folders in your plugins directory. If the archive contains a SketchupExtension that you would like users to be able to install from the Preferences > Extensions panel, rename your file to have a .rbz file extension.

**Parameters:**
- `filepath` (String) — The path to the RBZ or ZIP file to install.
- `show_warning` (Boolean) — Whether to warn the user not to install untrusted extensions. In certain cases the warning can be confusing and redundant, e.g. when automatically updating a trusted extension. When the user has selected the archive themselves, it is best to warn about the possible risks.

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Sketchup.html#install_from_archive-class_method)

#### `is_64bit? => Boolean`

This methods indicates whether the host SketchUp application is 64bit.

**Remarks:** This methods indicates whether the host SketchUp application is 64bit. Useful for extensions that ship with binaries and need to determine which versions to load.

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Sketchup.html#is_64bit?-class_method)

#### `is_online => Boolean`

The is_online method is used to verify a connection to the Internet.

**Remarks:** The is_online method is used to verify a connection to the Internet. This method can take some time to execute, so be careful not to call it more often than you need.

**Returns:** `Boolean` — true if successful, false if unsuccessful.

[Docs](https://ruby.sketchup.com/Sketchup.html#is_online-class_method)

#### `is_pro? => Boolean`

Note: In SketchUp Make this method will return true during the Pro trial period and revert to false when the trial period is over.

**Remarks:** Note: In SketchUp Make this method will return true during the Pro trial period and revert to false when the trial period is over. Returns a boolean flag indicating whether the application is SketchUp Pro.

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Sketchup.html#is_pro?-class_method)

#### `is_valid_filename?(filename) => Boolean`

The is_valid_filename? method is used to determine whether a filename contains illegal characters.

**Remarks:** The is_valid_filename? method is used to determine whether a filename contains illegal characters.

**Parameters:**
- `filename` (String) — A filename string.

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Sketchup.html#is_valid_filename?-class_method)

#### `load(path) => Boolean`

The load method is used to load Ruby files.

**Remarks:** The load method is used to load Ruby files. Unlike Ruby's own load method it also supports SketchUp's encrypted .rbe files. You do not need to include the file extension in the path. This method will look for .rb first (unencrypted) and then .rbe (encrypted) and finally .rbs (the deprecated scrambled format) files. See the “Distributing your Plugin” article for details.

**Parameters:**
- `path` (String) — The path, including the filename, to the file you want to require.

**Returns:** `Boolean` — True if the file is included. False if the file is not included.

[Docs](https://ruby.sketchup.com/Sketchup.html#load-class_method)

#### `open_file(filename) => Boolean | open_file(filename, with_status: true) => Integer, false | open_file(filename, with_status: true, show_version_warning_dialog: true) => Integer, false`

The open_file method is used to open a SketchUp model.

**Remarks:** The open_file method is used to open a SketchUp model.

**Parameters:**
- `filename` (String) — The model file to open.

**Returns:** `Boolean` — true if opening the file succeeded, false otherwise.

[Docs](https://ruby.sketchup.com/Sketchup.html#open_file-class_method)

#### `os_language => String`

The os_language method returns the language code for the language SketchUp is running in.

**Remarks:** The os_language method returns the language code for the language SketchUp is running in. This is an alias for the get_locale method. Examples of return values are: en-US, fr, it, de, es, ja, ko, zh-CN, zh-TW, pt-BR, nl, ru and sv. For an up to date list os supported languages, see the SketchUp download page.

**Returns:** `String` — a code representing the language SketchUp is displaying.

[Docs](https://ruby.sketchup.com/Sketchup.html#os_language-class_method)

#### `parse_length(string) => Float`

The parse_length method parses a string as a length.

**Remarks:** The parse_length method parses a string as a length. For example, “200” becomes 200.0.

**Parameters:**
- `string` (String) — The string to be parsed as a number.

**Returns:** `Float` — the numerical representation of the string if successful, or nil if unsuccessful.

[Docs](https://ruby.sketchup.com/Sketchup.html#parse_length-class_method)

#### `platform => Symbol`

This methods returns a symbol indicating the current platform.

**Remarks:** This methods returns a symbol indicating the current platform. It should be used over RUBY_PLATFORM as this returns a different value for Windows since SketchUp 2014. Older SketchUp versions still need to check RUBY_PLATFORM.include?('mswin') or RUBY_PLATFORM.include?('darwin'). Possible return values: :platform_win :platform_osx

**Returns:** `Symbol` — Current OS platform.

[Docs](https://ruby.sketchup.com/Sketchup.html#platform-class_method)

#### `plugins_disabled=(enabled) => Boolean`

The plugins_disabled= method lets you control whether SketchUp will load Ruby scripts from the plugins directory at startup time.

**Remarks:** The plugins_disabled= method lets you control whether SketchUp will load Ruby scripts from the plugins directory at startup time. This is primarily a trouble-shooting method. If you are having strange behavior in SketchUp that you suspect is from a bad script, you can type Sketchup.plugins_disabled=true into the Ruby console and restart SketchUp to see if the problem is fixed.

**Parameters:**
- `enabled` (Boolean) — If true, the plugins directory will not load.

**Returns:** `Boolean` — true if plugins were disabled.

[Docs](https://ruby.sketchup.com/Sketchup.html#plugins_disabled=-class_method)

#### `plugins_disabled? => Boolean`

The plugins_disabled? method indicates whether Ruby scripts in the plugins directory will be loaded at startup time.

**Remarks:** The plugins_disabled? method indicates whether Ruby scripts in the plugins directory will be loaded at startup time.

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Sketchup.html#plugins_disabled?-class_method)

#### `quit => Object`

The quit method is used to terminate the application.

**Remarks:** The quit method is used to terminate the application. This will pop-up the usual model save prompts if there are unsaved models open. User can cancel the model save, in which case the application will not terminate.

**Returns:** `Object` — self

[Docs](https://ruby.sketchup.com/Sketchup.html#quit-class_method)

#### `read_default(section, variable, default = nil) => Object?`

Note: Be aware that the method is not capable of handling Length objects.

**Remarks:** Note: Be aware that the method is not capable of handling Length objects. You can convert the value to a Float before writing and convert back to Length when reading the value. Don't store the value as a String as this rounds the value and formats it in a way that can't be read if the system setting for decimal separator changes. The #read_default method is used to retrieve the string associated with a value within the specified sub-section section of a .INI file or registry (within the Software > SketchUp > SketchUp [Version] section).

**Parameters:**
- `section` (String) — A section in an .INI or registry.
- `variable` (String) — A variable within the section.
- `default` (Object) — A default value if the value is not found.

**Returns:** `Object, nil` — if unsuccessful, the value of the default if successful.

[Docs](https://ruby.sketchup.com/Sketchup.html#read_default-class_method)

#### `redo => nil`

The redo method is used redo the last transaction on the redo stack.

**Remarks:** The redo method is used redo the last transaction on the redo stack.

**Returns:** `nil` — 

[Docs](https://ruby.sketchup.com/Sketchup.html#redo-class_method)

#### `register_extension(extension, load_on_start = false) => Boolean`

Note: It is recommended to set load_on_start to true unless you have a very good reason not to.

**Remarks:** Note: It is recommended to set load_on_start to true unless you have a very good reason not to. The #register_extension method is used to register an extension with SketchUp's Extension Manager.

**Parameters:**
- `extension` (SketchupExtension) — A SketchupExtension object.
- `load_on_start` (Boolean) — Passing true into this will load the extension immediately and set it so that it will load automatically whenever SketchUp restarts.

**Returns:** `Boolean` — true if extension registered properly

[Docs](https://ruby.sketchup.com/Sketchup.html#register_extension-class_method)

#### `register_importer(importer) => Boolean`

The register_importer method is used to register an importer with SketchUp.

**Remarks:** The register_importer method is used to register an importer with SketchUp.

**Parameters:**
- `importer` (Sketchup::Importer) — An Importer object representing the importer.

**Returns:** `Boolean` — true if successful, false if unsuccessful.

[Docs](https://ruby.sketchup.com/Sketchup.html#register_importer-class_method)

#### `remove_observer(observer) => Boolean`

The remove_observer method is used to remove an observer from the current object.

**Remarks:** The remove_observer method is used to remove an observer from the current object.

**Parameters:**
- `observer` (Sketchup::AppObserver) — An observer.

**Returns:** `Boolean` — true if successful, false if unsuccessful.

[Docs](https://ruby.sketchup.com/Sketchup.html#remove_observer-class_method)

#### `require(path) => Boolean`

The require method is used to load Ruby files once.

**Remarks:** The require method is used to load Ruby files once. Unlike Ruby's own require method it also supports SketchUp's encrypted .rbe files. You do not need to include the file extension in the path. This method will look for .rbe first (encrypted) and then .rbs (the deprecated scrambled format) and finally .rb (unencrypted) files. The loading order was changed in SketchUp 2016 when the new .rbe encryption was introduced. Prior to SketchUp 2016 the loading order was first .rb then .rbs.

**Parameters:**
- `path` (String) — The path, including the filename, to the file you want to require.

**Returns:** `Boolean` — True if the file is included. False if the file is not included.

[Docs](https://ruby.sketchup.com/Sketchup.html#require-class_method)

#### `resize_viewport(model, width, height) => Boolean | resize_viewport(model, width, height, logical_pixels: false) => Boolean`

Note: In SketchUp 2024.

**Remarks:** Note: In SketchUp 2024.0 and later this method doesn't behave correctly in all cases on Windows. The passed values are internally converted to logical pixels, rounded and converted back to physical pixels. This means certain sizes, such as 1000 px at 150% scaling, cannot be accurately set. As a workaround in you can use sizes that are evenly divisible with common scale factors, if you depend on pixel perfect sizes. The resize_viewport method changes the pixel size of the viewport and SketchUp window. This can be useful for producing a consistent behavior in automatic testing, regardless of the display resolution.

**Parameters:**
- `model` (Sketchup::Model)
- `width` (Integer) — Width in physical pixels
- `height` (Integer) — Height in physical pixels

**Returns:** `Boolean` — true on success. false if the window couldn't reach the desired size, e.g. because it wouldn't fit the screen.

[Docs](https://ruby.sketchup.com/Sketchup.html#resize_viewport-class_method)

#### `save_thumbnail(skp_filename, img_filename) => Boolean`

The save_thumbnail method is used to generate a thumbnail for any SKP file - not necessarily the loaded model.

**Remarks:** The save_thumbnail method is used to generate a thumbnail for any SKP file - not necessarily the loaded model.

**Parameters:**
- `skp_filename` (String) — The name of the SketchUp file whose model you want represented in the thumbnail.
- `img_filename` (String) — The name of the file where the thumbnail will be saved.

**Returns:** `Boolean` — true if successful, false if unsuccessful.

[Docs](https://ruby.sketchup.com/Sketchup.html#save_thumbnail-class_method)

#### `send_action(action) => Boolean`

The send_action method sends a message to the message queue to perform some action asynchronously.

**Remarks:** The send_action method sends a message to the message queue to perform some action asynchronously. Valid actions are: showRubyPanel: viewBack: viewBottom: viewFront: viewIso: viewLeft: viewRight: viewTop: viewPerspective: viewShowAxes: viewShowHidden: viewZoomExtents: viewZoomToSelection: viewUndo: selectOrbitTool: selectPositionCameraTool: selectDollyTool: selectTurnTool: selectWalkTool: selectZoomTool: selectFieldOfViewTool: selectZoomWindowTool: pageAdd: pageDelete: pageUpdate: pageNext: pagePrevious: renderWireframe: renderHiddenLine: renderMonochrome: renderShaded: renderTextures: selectArcTool: selectArc3PointTool: selectArc3PointPieTool: selectAxisTool: selectCircleTool: selectEraseTool: selectFreehandTool: selectLineTool: selectMeasureTool: selectMoveTool: selectOffsetTool: selectPaintTool: selectPolygonTool: selectProtractorTool: selectPushPullTool: selectRectangleTool: selectRectangle3PointTool: selectRotateTool: selectScaleTool: selectSectionPlaneTool: selectTextTool: selectDimensionTool: selectExtrudeTool: selectSelectionTool: editUndo: editRedo: editHide: editUnhide: fixNonPlanarFaces: Added in SketchUp 8.0+: addBuilding: getPhotoTexture: selectImageIglooTool: selectNorthTool: Added in SketchUp 2013+: showExtensionStore: Removed in SketchUp 2013+: addBuilding: On the PC only, you can also send these numeric values. (Note that these are officially “unsupported” and are not guaranteed to work in current or future versions of the API.) 10501: set view to Top 10502: set view to Front 10503: set view to Rear 10504: set view to Left 10505: set view to Right 10506: set view to Bottom 10507: set view to Axonometric 10510: set render mode to Wire 10511: set render mode to Hidden lines removal 10512: set render mode to Surfaces Shading 10513: set render mode to Transparency 10519: set camera to ortho (removes perspective) 10520: walk tool 10521: display the System Preferences dialog box (Files tab) 10522: removes axes display 10523: pan tool 10525: set the interactive eye height feature 10526: zoom window 10527: zoom extents 10529: zoom out 2 10531: toggle the Standard toolbar 10532: toggle the Camera toolbar 10533: display the Shadows Settings dialog box 10537: toggle the Views toolbar 10538: display the System Preferences dialog box (Display tab) 10545: toggle Shadows 10546: toggle Shadows toolbar 10551: toogle Large icons 10576: toggle Render Mode toolbar 21019: select the Eraser tool 21020: show Status bar and VCB 21022: hide Status bar and VCB 21023: place 3d text box 21024: select the Measure tool 21031: select the Freehand Draw tool 21041: select the PushPull tool 21048: select the Move tool 21052: hide selected objects 21056: create face with selected edges closed loop 21057: select the Protractor tool 21060: display Components Window 21061: toggle Draw toolbar 21063: toggle Model Bounding Box display 21065: select the Arc tool 21067: creat a new Page 21069: select the Arc 3 Point tool 21070: select the Arc 3 Point Pie tool 21074: show the Materials Browser Window 21076: display the Preferences dialog box (Text activated) 21077: display the Tip of the day Window 21078: select the Paint Bucket tool 21080: display the Page Manager Window 21082: display the Macros Dialog Box 21086: display the Components Browser Window 21094: select the Rectangle tool 21095: select the Polygon tool 21096: select the Circle tool 21098: open the Open Window 21100: select the Offset tool 21101: select all objects 21107: invert selection 21112: open the Import Window 21124: launch the validity check tool 21126: select the Axes tool 21029: select the Rotate tool 21032: toggle Layer toolbar 21036: display the Save as Window 21046: spin the model a full 360&deg; and display report 21047: fast Pick Time report 21049: open the Export model Window 21169: select the Position Camera tool 21170: display the Preferences, Tour Guide activated 21180: create a new Page just right of selected page 21200: display the Insert Image Window 21233: display Area of selected face 21234: display Area of all faces with selected material 21236: select the Scale tool 21237: display the Export 2D Graphics Window 21245: display a Polygon Offset Factors dialog box 21276: reverse selected face(s) 21287: select the Divide feature 21337: select the Section Plane Placement tool 21354: open the Layer Window 21386: open the Export Animation Window 21405: select the Text tool 21406: display Fog dialog box 21410: select the Dim tool 21433: toggle Edit toolbar 21442: select the FollowMe tool 21448: select the Axes tool 21453: select all objects 21460: display Licence 21462: display Authorization dialog box 21463: display un-authorizing message 21464: display Open Licence files (Network) Window 21466: display Quick reference Card in Adobe Reader 21467: display Licences in use dialog box 21469: zoom extents to selected objects 21476: perform a non-planar check on selected objects 21477: list accelerators in window 21485: erase selected objects 21487: display Edit current material dialog box 21485: erase all new created pages 21488: display Entity Info Window 21490: display Soften Edges Window 21491: display Profiles 21492: display Extended Edges 21493: display Jitter Lines 21494: select Field of view tool 21513: display the outliner 21520: override Tile Rendering Size dialog box 21525: select the FollowMe tool 21542: display the Insert Image Window 21560 and up: causes a runtime Error

**Parameters:**
- `action` (String, Integer) — The action to be performed.

**Returns:** `Boolean` — true if successful, false if unsuccessful

[Docs](https://ruby.sketchup.com/Sketchup.html#send_action-class_method)

#### `send_to_layout(file) => Boolean`

The send_to_layout method is used to open a file in LayOut.

**Remarks:** The send_to_layout method is used to open a file in LayOut.

**Parameters:**
- `file` (String) — The path and filename to open, either .skp or .layout.

**Returns:** `Boolean` — true if opening the file succeeded, false otherwise. If LayOut is not installed or the file is not present this function will return false.

[Docs](https://ruby.sketchup.com/Sketchup.html#send_to_layout-class_method)

#### `set_status_text => nil | set_status_text(status_text = '', position = SB_PROMPT) => nil`

The set_status_text method is used to set the text appearing on the status bar within the drawing window.

**Remarks:** The set_status_text method is used to set the text appearing on the status bar within the drawing window. If no arguments are passed, the status bar content is cleared. Valid positions are: SB_PROMPT - the text will appear at the left-side of the status bar SB_VCB_LABEL - the text will appear in place of the VCB label SB_VCB_VALUE - the text will appear in the VCB

**Parameters:**
- `status` (String) — text the status text that will appear.
- `position` (Integer) — the position where the text will appear.

**Returns:** `nil` — 

[Docs](https://ruby.sketchup.com/Sketchup.html#set_status_text-class_method)

#### `status_text=(status_text) => String`

The status_text= method is used to set the text appearing on the status bar within the drawing window.

**Remarks:** The status_text= method is used to set the text appearing on the status bar within the drawing window. This is the same as calling set_status_text with a 2nd parameter of SB_PROMPT.

**Parameters:**
- `status_text` (String) — The status text that will appear.

**Returns:** `String` — 

[Docs](https://ruby.sketchup.com/Sketchup.html#status_text=-class_method)

#### `temp_dir => String`

The temp_dir method is used to retrieve the OS temporary directory for the current user.

**Remarks:** The temp_dir method is used to retrieve the OS temporary directory for the current user. You can use this directory to write temporary files that are not required to persist between SketchUp sessions.

**Returns:** `String` — a string containing the full temporary directory path

[Docs](https://ruby.sketchup.com/Sketchup.html#temp_dir-class_method)

#### `template => String`

The template method is used to get the file name of the current template.

**Remarks:** The template method is used to get the file name of the current template. Templates are the .skp files that are loaded when the user select File > New.

**Returns:** `String` — the current template

[Docs](https://ruby.sketchup.com/Sketchup.html#template-class_method)

#### `template=(filename) => String`

The template= method is used to set the file name of the current template.

**Remarks:** The template= method is used to set the file name of the current template. Templates are the .skp files that are loaded when the user select File > New.

**Parameters:**
- `filename` (String) — The name of the template to set.

**Returns:** `String` — true if successful, false if unsuccessful.

[Docs](https://ruby.sketchup.com/Sketchup.html#template=-class_method)

#### `template_dir => String`

The template_dir is used to retrieve the directory where templates are stored by the SketchUp install.

**Remarks:** The template_dir is used to retrieve the directory where templates are stored by the SketchUp install. Templates are the .skp files that are loaded when the user select File > New.

**Returns:** `String` — containing the full template directory path

[Docs](https://ruby.sketchup.com/Sketchup.html#template_dir-class_method)

#### `undo => nil`

The undo method is used undo the last transaction on the undo stack.

**Remarks:** The undo method is used undo the last transaction on the undo stack.

**Returns:** `nil` — 

[Docs](https://ruby.sketchup.com/Sketchup.html#undo-class_method)

#### `vcb_label=(label_text) => String`

The vcb_label= method is used to set the label that appears on the vcb, or the “value control box”, which is another word for the “measurements” text entry box that appears at the bottom on the SketchUp window.

**Remarks:** The vcb_label= method is used to set the label that appears on the vcb, or the “value control box”, which is another word for the “measurements” text entry box that appears at the bottom on the SketchUp window. This is the same as calling set_status_text with a 2nd parameter of SB_VCB_LABEL.

**Parameters:**
- `label_text` (String) — The label text that will appear.

**Returns:** `String` — 

[Docs](https://ruby.sketchup.com/Sketchup.html#vcb_label=-class_method)

#### `vcb_value=(value) => String`

The vcb_value= method is used to set the value that appears on the vcb, or the “value control box”, which is another word for the “measurements” text entry box that appears at the bottom on the SketchUp window.

**Remarks:** The vcb_value= method is used to set the value that appears on the vcb, or the “value control box”, which is another word for the “measurements” text entry box that appears at the bottom on the SketchUp window. This is the same as calling set_status_text with a 2nd parameter of SB_VCB_VALUE.

**Parameters:**
- `value` (String) — The text that will appear as the vcb's value.

**Returns:** `String` — 

[Docs](https://ruby.sketchup.com/Sketchup.html#vcb_value=-class_method)

#### `version => String`

Gets the current version of sketchup in decimal form.

**Remarks:** Gets the current version of sketchup in decimal form.

**Returns:** `String` — the decimal form of the version

[Docs](https://ruby.sketchup.com/Sketchup.html#version-class_method)

#### `version_number => Integer`

Get the current version of sketchup as a whole number for comparisons.

**Remarks:** Get the current version of sketchup as a whole number for comparisons. The number returned has the major, minor, and build values packed into an integer value as follows: Major version = X Minor version = Y Build number = Z SketchUp 6.0 - SketchUp 2015 XXYYYZZZ SketchUp 2016+ XXYZZZZZZZ

**Returns:** `Integer` — the whole number form of the version

[Docs](https://ruby.sketchup.com/Sketchup.html#version_number-class_method)

#### `write_default(section, key, value) => Boolean`

Note: Be aware that the method is not capable of handling Length objects.

**Remarks:** Note: Be aware that the method is not capable of handling Length objects. You can convert the value to a Float before writing and convert back to Length when reading the value. Don't store the value as a String as this rounds the value and formats it in a way that can't be read if the system setting for decimal separator changes. The #write_default method is used to set the string associated with a variable within the specified sub-section of a .plist file on the Mac or the registry on Windows (within the Software > SketchUp > SketchUp [Version] section).

**Parameters:**
- `section` (String) — A section in a .plist file (Mac) or the registry (Windows).
- `key` (String) — A key within the section.
- `value` (Object) — The value to store.

**Returns:** `Boolean` — True if successful, false if unsuccessful.

[Docs](https://ruby.sketchup.com/Sketchup.html#write_default-class_method)

## SketchupExtension (class)

The SketchupExtension class contains methods allowing you to create and manipulate SketchUp extensions. Extensions are Ruby scripts that can be loaded and unloaded using the Extension manager (Extensions panel of the Extension Manager dialog box). Generally you should register your ruby scripts as an extension to give SketchUp users the ability to disable it through the user interface. The idea here is to take the ruby script that actually creates your functionality and place it in a folder somewhere outside of the /Plugins folder, most commonly a subdirectory like /Plugins/MyExtension. Then you create a new ruby script inside the /Plugins directory that will set up the extension entry and load your original script if the user has your extension turned on. Here is an example extension loading script. For this example, the following code would be saved in /Plugins/StairTools.rb, and the actual plugin itself would live in /Plugins/StairTools/core.rb. You can find two example extensions that ship with SketchUp, su_dynamiccomponents.rb and su_sandboxtools.rb, under the /Plugins/ folder.

[Vendor docs](https://ruby.sketchup.com/SketchupExtension.html)

### Constructors
- `SketchupExtension(title, path)` — Note: It is recommended to omit the file extension provided in the path argument. SketchUp will resolve the file extension to .rbe, .rbs or .rb. The new method is used to create a new SketchupExtension object. Note that once the extension object is created, it will not appear in the Extension Manager dialog until your register it with the Sketchup.register_extension method.

### Methods
#### `check => Boolean`

Loads the extension, meaning the underlying ruby script is immediately interpreted.

**Remarks:** Loads the extension, meaning the underlying ruby script is immediately interpreted. This is the equivalent of checking the extension's checkbox in the Extension Manager.

**Returns:** `Boolean` — whether the load succeeded

[Docs](https://ruby.sketchup.com/SketchupExtension.html#check-instance_method)

#### `extension_path => String`

The extension_path method returns the file system path to the extension's outer rb file.

**Remarks:** The extension_path method returns the file system path to the extension's outer rb file.

**Returns:** `String` — the file system path to the extension

[Docs](https://ruby.sketchup.com/SketchupExtension.html#extension_path-instance_method)

#### `id => String`

The id method returns the Extension Warehouse ID string.

**Remarks:** The id method returns the Extension Warehouse ID string.

**Returns:** `String` — the Extension Warehouse ID

[Docs](https://ruby.sketchup.com/SketchupExtension.html#id-instance_method)

#### `load_on_start? => Boolean`

Returns whether the extension is set to load when SketchUp starts up.

**Remarks:** Returns whether the extension is set to load when SketchUp starts up.

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/SketchupExtension.html#load_on_start?-instance_method)

#### `loaded? => Boolean`

Returns whether the extension is currently loaded, meaning the actual ruby script that implements the extension has been evaluated.

**Remarks:** Returns whether the extension is currently loaded, meaning the actual ruby script that implements the extension has been evaluated.

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/SketchupExtension.html#loaded?-instance_method)

#### `registered? => Boolean`

Returns whether the extension has been registered via Sketchup.

**Remarks:** Returns whether the extension has been registered via Sketchup.register_extension.

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/SketchupExtension.html#registered?-instance_method)

#### `uncheck => Boolean`

Unloads the extension.

**Remarks:** Unloads the extension. This is the equivalent of unchecking the extension's checkbox in the Extension Manager > Extensions list. Note that technically the extension is not “unloaded” in the sense that it stops running during the current SketchUp session, but the next time the user restarts SketchUp, the extension will not be active.

**Returns:** `Boolean` — whether the unload succeeded

[Docs](https://ruby.sketchup.com/SketchupExtension.html#uncheck-instance_method)

#### `version_id => String`

The version_id method returns the Extension Warehouse Version ID string.

**Remarks:** The version_id method returns the Extension Warehouse Version ID string.

**Returns:** `String` — the Extension Warehouse Version ID string

[Docs](https://ruby.sketchup.com/SketchupExtension.html#version_id-instance_method)

### Properties
- `copyright` (String, get/set) — The copyright method returns the copyright string which appears beneath an extension inside the Extensions Manager dialog.
- `creator` (String, get/set) — The creator method returns the creator string which appears beneath an extension inside the Extensions Manager dialog.
- `description` (String, get/set) — The description method returns the long description which appears beneath an extension inside the Extensions Manager dialog.
- `name` (String, get/set) — The name method returns the name which appears for an extension inside the Extensions Manager dialog.
- `version` (String, get/set) — The version method returns the version which appears beneath an extension inside the Extensions Manager dialog.

## String (class)

The String class contains a method used to parse a string as a length value. All string arguments in Ruby API expect utf-8 strings.

[Vendor docs](https://ruby.sketchup.com/String.html)

### Methods
#### `to_l => Length`

The to_l method is used to convert a string to a length.

**Remarks:** The to_l method is used to convert a string to a length. The returned length is expressed in the Model units.

**Returns:** `Length` — the length value

[Docs](https://ruby.sketchup.com/String.html#to_l-instance_method)

## UI (interface)

The UI module contains a number of methods for creating simple UI elements from a SketchUp Ruby script.

[Vendor docs](https://ruby.sketchup.com/UI.html)

### Methods
#### `add_context_menu_handler {|menu| ... } => Integer`

The add_context_menu_handler method is used to register a block of code with SketchUp that will be called when a context menu is to be displayed.

**Remarks:** The add_context_menu_handler method is used to register a block of code with SketchUp that will be called when a context menu is to be displayed. The context menu handler can then display the context menu with the items that you have added. Be careful with what you do in a context menu handler. If you perform an operation takes take a long time, such as traversing the model or selection in a large model it will delay the menu. See the contextmenu.rb script in the Plugins/examples directory for an example.

**Returns:** `Integer` — the number of context handlers that are registered

[Docs](https://ruby.sketchup.com/UI.html#add_context_menu_handler-class_method)

#### `beep => nil`

The beep method plays a system beep sound.

**Remarks:** The beep method plays a system beep sound. The beep method does not accept any arguments nor return any values.

**Returns:** `nil` — 

[Docs](https://ruby.sketchup.com/UI.html#beep-class_method)

#### `create_cursor(path, hot_x, hot_y) => Integer`

Note: Since SketchUp 2016 it is possible to provide vector images for the cursors.

**Remarks:** Note: Since SketchUp 2016 it is possible to provide vector images for the cursors. SVG format for Windows and PDF format for macOS. This is the recommended format to use since it will scale well with different DPI scaling. Note: Because the image is never released, avoid creating new cursors for the same image for any given tool. Cache it and reuse it. The create_cursor method is used to create a cursor from an image file at the specified location. This must be called from within a custom tool. See the Sketchup::Tool documentation for a complete example. The size of the cursor images should be 32x32 pixels. Any size different from that will be scaled using the shrink-to-fit policy. The coordinates for the cursor's hotspot are based from it's top left corner, starting at (0, 0). For example, a value of (hot_x, hot_y) = (5, 10) would identify the hotpoint of the cursor at 5 pixels from the left edge of your cursor image and 10 pixels from the top edge of your cursor image.

**Parameters:**
- `path` (String) — File path to an image.
- `hot_x` (Integer) — The x-coordinate of the “hotpoint” of the cursor, computed from the left edge of your cursor image.
- `hot_y` (Integer) — The y-coordinate of the “hotpoint” of the cursor, computed from the top edge of your cursor image.

**Returns:** `Integer` — Id associated with the cursor. Use this with set_cursor in Sketchup::Tool#onSetCursor.

[Docs](https://ruby.sketchup.com/UI.html#create_cursor-class_method)

#### `get_clipboard_data => String?`

Returns the plain text available on the clipboard, if there is any.

**Remarks:** Returns the plain text available on the clipboard, if there is any.

**Returns:** `String, nil` — 

[Docs](https://ruby.sketchup.com/UI.html#get_clipboard_data-class_method)

#### `inputbox(prompts, defaults, title) => Array<String>, false | inputbox(prompts, defaults, list, title) => Array<String>, false`

Note: The method intelligently handles various types for default values and lists, automatically attempting to convert provided inputs to match the type of the default value.

**Remarks:** Note: The method intelligently handles various types for default values and lists, automatically attempting to convert provided inputs to match the type of the default value. This ensures consistency in data types throughout the operation. Creates a dialog box for inputting user information. The dialog box contains input fields with static text prompts, optional default values, optional drop down selections, and optional title. You can also use this method to display drop down lists of options, by passing an optional param.

**Parameters:**
- `prompts` (Array<String>) — An array of prompt names appearing in the input box adjacent to input fields.
- `defaults` (Array<String>) — An array of default values for the input fields.
- `title` (String) — The title for the input box.

**Returns:** `Array<String>, false` — An array of returned values if the user did not cancel the dialog. If the user canceled the dialog, false is returned. The returned values in the array will be in the same order as the input fields.

[Docs](https://ruby.sketchup.com/UI.html#inputbox-class_method)

#### `inspector_names => Array<String>`

The inspector_names method is used to returns the names of all the inspectors.

**Remarks:** The inspector_names method is used to returns the names of all the inspectors. Inspectors are another name for the various floating dialog windows that you can activate from withing SketchUp, such as the Materials window.

**Returns:** `Array<String>` — an array of strings containing the names of inspectors.

[Docs](https://ruby.sketchup.com/UI.html#inspector_names-class_method)

#### `menu(menu_name = "Extensions") => Sketchup::Menu`

Note: The 'Extensions' menu was named 'Plugins' prior to SketchUp 2015.

**Remarks:** Note: The 'Extensions' menu was named 'Plugins' prior to SketchUp 2015. For backward compatibility 'Plugins' still works. Note: 'Developer' menu was added with SketchUp 2021.1. The menu method retrieves a SketchUp's menu object with a given name. This is the first step toward adding your own custom items to the bottom of SketchUp's menus. Valid menu names are: 'File', 'Edit', 'View', 'Camera', 'Draw', 'Tools', 'Window', 'Extensions', 'Help' and 'Developer'.

**Parameters:**
- `menu_name` (String) — The name of a supported menu.

**Returns:** `Sketchup::Menu` — 

[Docs](https://ruby.sketchup.com/UI.html#menu-class_method)

#### `messagebox(message, type = MB_OK) => Integer`

Creates a dialog box containing static text with a series of buttons for the user to choose from.

**Remarks:** Creates a dialog box containing static text with a series of buttons for the user to choose from. Valid message box types are: MB_OK - Contains an OK button. MB_OKCANCEL - Contains OK and Cancel buttons. MB_ABORTRETRYIGNORE - Contains Abort, Retry, and Ignore buttons. MB_YESNOCANCEL - Contains Yes, No, and Cancel buttons. MB_YESNO - Contains Yes and No buttons. MB_RETRYCANCEL - Contains Retry and Cancel buttons. MB_MULTILINE - Contains and OK button. Return values can be any of following: IDOK IDCANCEL IDABORT IDRETRY IDIGNORE IDYES IDNO In an MB_MULTILINE message box, the message is displayed as a multi-line message with scrollbars (as needed). MB_MULTILNE also allows a third string argument that will be used as the title for the messagebox.

**Parameters:**
- `message` (String) — The message that you want to display.
- `type` (Integer) — The message box type, which will be a constant from the list in the method comments.

**Returns:** `Integer` — A number corresponding to what the user selected.

[Docs](https://ruby.sketchup.com/UI.html#messagebox-class_method)

#### `model_info_pages => Array<String>`

The model_info_pages method is used to returns the names of all the available model info pages.

**Remarks:** The model_info_pages method is used to returns the names of all the available model info pages. These include UI windows such as Components, Credits, and Units.

**Returns:** `Array<String>` — an array of strings containing the names of model info pages.

[Docs](https://ruby.sketchup.com/UI.html#model_info_pages-class_method)

#### `openURL(url) => Boolean`

The openURL method is used to open the default browser to a URL.

**Remarks:** The openURL method is used to open the default browser to a URL.

**Parameters:**
- `url` (String)

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/UI.html#openURL-class_method)

#### `openpanel(title, directory, filename) => String`

The openpanel method is used to display the Open dialog box.

**Remarks:** The openpanel method is used to display the Open dialog box. The path that is returned can then be used inside code to open a text or image file. See the standard Ruby class File for examples of reading and writing from disk.

**Parameters:**
- `title` (String) — The title to apply to the open dialog box.
- `directory` (String) — The default directory for the open panel.
- `filename` (String) — The default filename for the open panel. On Windows, you can alternatively pass a wildcard filter using this format: UIname|wildcard||. Additional filter dropdown list items can be added by adding additional pairs of filter name and filter like this: UIname1|wildcard1|UIname2|wildcard2||. Also multiple wildcard filters can be combined into a single line using a semicolon-separated list in the filter field: ui_name|wildcard1;wildcard2||.

**Returns:** `String` — the full path and name of the file selected, or nil if the dialog was canceled.

[Docs](https://ruby.sketchup.com/UI.html#openpanel-class_method)

#### `play_sound(filename) => nil`

The play_sound method is used to play a sound file.

**Remarks:** The play_sound method is used to play a sound file. Valid sound files include .wav and .mp3 files on the Mac and .wav files on the PC.

**Parameters:**
- `filename` (String) — the relative path to the filename from the SketchUp install directory, or an absolute path to the file. (See Sketchup.find_support_file for a way to search for a specific file.)

**Returns:** `nil` — 

[Docs](https://ruby.sketchup.com/UI.html#play_sound-class_method)

#### `preferences_pages => Array<String>`

The preferences_pages method is used to returns the names of all the preferences pages.

**Remarks:** The preferences_pages method is used to returns the names of all the preferences pages. These include windows like Templates. SketchUp 2017 "Extensions" page was removed.

**Returns:** `Array<String>` — an array of strings containing the names of preference pages.

[Docs](https://ruby.sketchup.com/UI.html#preferences_pages-class_method)

#### `refresh_inspectors => nil`

Tells SketchUp to refresh all inspectors such as the Component Browser and the Outliner.

**Remarks:** Tells SketchUp to refresh all inspectors such as the Component Browser and the Outliner. This is useful when you need to manually force a refresh after you've made a change to the document via Ruby. Generally, SketchUp will keep these in sync for you, but occasionally it does not, such as when model.start_operation has disabled UI updates.

**Returns:** `nil` — 

[Docs](https://ruby.sketchup.com/UI.html#refresh_inspectors-class_method)

#### `refresh_toolbars => nil`

Tells SketchUp to refresh all floating toolbars.

**Remarks:** Tells SketchUp to refresh all floating toolbars. This is useful when you need to manually force a refresh after you've made a change to the document via Ruby. Generally, SketchUp will keep these in sync for you, but occasionally it does not, such as when Sketchup::Model#start_operation has disabled UI updates. This only affects macOS, on Windows the toolbars are always refreshing.

**Returns:** `nil` — 

[Docs](https://ruby.sketchup.com/UI.html#refresh_toolbars-class_method)

#### `savepanel(title, directory, filename) => String`

The savepanel method is used to display the Save dialog box.

**Remarks:** The savepanel method is used to display the Save dialog box. The path that is returned can then be used inside code to save out a text or image file. See the standard Ruby class File for examples of reading and writing from disk.

**Parameters:**
- `title` (String) — The title to apply to the save dialog box.
- `directory` (String) — The default directory for the save panel.
- `filename` (String) — The default filename for the save panel. On Windows, you can alternatively pass a mask, like “*.txt”, to have all the .txt files display. If you want multiple file types to display, you can supply multiple masks for the filename and separate them with a semicolon, like this: “.txt;.doc”.

**Returns:** `String` — the full path and name of the file selected or nil if the dialog was canceled.

[Docs](https://ruby.sketchup.com/UI.html#savepanel-class_method)

#### `scale_factor => Float | scale_factor(view) => Float`

Note: SU2017M0 will automatically scale up line width and text size, but will not scale up the points provided to Sketchup::View#draw2d.

**Remarks:** Note: SU2017M0 will automatically scale up line width and text size, but will not scale up the points provided to Sketchup::View#draw2d. Returns the scaling factor SketchUp uses on high DPI monitors. Useful for things like Sketchup::View#draw2d.

**Parameters:**
- `view` (Sketchup::View)

**Returns:** `Float` — 

[Docs](https://ruby.sketchup.com/UI.html#scale_factor-class_method)

#### `select_directory(options = {}) => String, ...`

The select_directory method is used to display the OS dialog for selecting one or several directories from the file system.

**Remarks:** The select_directory method is used to display the OS dialog for selecting one or several directories from the file system.

**Parameters:**
- `options` (Hash) — The dialog can be customized by providing a hash or named arguments of options.

**Returns:** `String, Array<String>, nil` — A string with the full path of the directory selected when :select_multiple option is set to false otherwise an array of strings or nil if the user cancelled.

[Docs](https://ruby.sketchup.com/UI.html#select_directory-class_method)

#### `set_clipboard_data(data) => Boolean`

Copies plain text to the clipboard.

**Remarks:** Copies plain text to the clipboard.

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/UI.html#set_clipboard_data-class_method)

#### `set_cursor(cursor_id) => Boolean`

The set_cursor method is used to change the cursor to a new cursor with a given cursor id.

**Remarks:** The set_cursor method is used to change the cursor to a new cursor with a given cursor id. See UI.create_cursor and the Tool class for details on creating your own tools with arbitrary cursors. If you call this while a standard SketchUp tool is active, you will not see your custom cursor, as these tools are constantly setting their own cursors to indicate SketchUp's state.

**Parameters:**
- `cursor_id` (Integer) — The id of the cursor you want to display.

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/UI.html#set_cursor-class_method)

#### `set_toolbar_visible(name, visible) => Boolean`

The set_toolbar_visible method is used to set whether a given toolbar is visible.

**Remarks:** The set_toolbar_visible method is used to set whether a given toolbar is visible. Note that the toolbars and their names are different on the Mac vs. PC, so be careful and be sure to test when using this method in a cross-platform script.

**Parameters:**
- `name` (String) — The name of a Ruby toolbar.
- `visible` (Boolean) — True to make the toolbar visible, false to hide it.

**Returns:** `Boolean` — true if successful, false if not.

[Docs](https://ruby.sketchup.com/UI.html#set_toolbar_visible-class_method)

#### `show_extension_manager => nil`

The show_extension_manager method is used to display the Extension Manager dialog.

**Remarks:** The show_extension_manager method is used to display the Extension Manager dialog.

**Returns:** `nil` — 

[Docs](https://ruby.sketchup.com/UI.html#show_extension_manager-class_method)

#### `show_extension_warehouse(extension_id = nil) => nil`

Show the Extension Warehouse dialog inside SketchUp.

**Remarks:** Show the Extension Warehouse dialog inside SketchUp. If an extension UUID is provided the dialog navigates directly to that extension's page. If no UUID is provided it opens the Extension Warehouse home page.

**Parameters:**
- `extension_id` (String, nil) — Optional Extension Warehouse UUID. If nil or omitted the EW home page is shown.

**Returns:** `nil` — 

[Docs](https://ruby.sketchup.com/UI.html#show_extension_warehouse-class_method)

#### `show_inspector(name) => Boolean`

The show_inspector method is used to display the inspector with the given name.

**Remarks:** The show_inspector method is used to display the inspector with the given name. You can get the list of valid inspectors with UI.inspector_names.

**Parameters:**
- `name` (String) — The name of inspector that you want to display.

**Returns:** `Boolean` — true if successful, false if unsuccessful

[Docs](https://ruby.sketchup.com/UI.html#show_inspector-class_method)

#### `show_model_info(page_name) => Boolean`

The show_model_info method is used to display the model info dialog for a specific page.

**Remarks:** The show_model_info method is used to display the model info dialog for a specific page. You can get the list of valid page names with model_info_pages. SketchUp 2014 "Classifications" page was added.

**Parameters:**
- `page_name` (String) — The name of the model info dialog you want to display.

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/UI.html#show_model_info-class_method)

#### `show_preferences(page_name) => Boolean`

The show_preferences method is used to display a SketchUp preferences dialog.

**Remarks:** The show_preferences method is used to display a SketchUp preferences dialog. You can get the list of valid dialogs with preferences_pages.

**Parameters:**
- `page_name` (String) — The name of the preferences dialog you want to display.

**Returns:** `Boolean` — true

[Docs](https://ruby.sketchup.com/UI.html#show_preferences-class_method)

#### `start_timer(seconds, repeat = false) {|procedure| ... } => Integer`

The start_timer method is used to start a timer.

**Remarks:** The start_timer method is used to start a timer. This is an effective method to create a repeating snippet of code for arbitrary animation. See this blog post for an detailed example of custom animation using timers: sketchupapi.blogspot.com/2008/10/animate-yo-cheese.html Note that there is a bug that if you open a modal window in a non-repeating timer the timer will repeat until the window is closed.

**Parameters:**
- `seconds` (Numeric) — The time in seconds before your code should be called.
- `repeat` (Boolean) — true if you want the timer to repeat, false (or omit) if you do not want it to repeat.

**Returns:** `Integer` — a timer ID

[Docs](https://ruby.sketchup.com/UI.html#start_timer-class_method)

#### `stop_timer(id) => nil`

The stop_timer method is used to stop a timer based on its id.

**Remarks:** The stop_timer method is used to stop a timer based on its id.

**Parameters:**
- `id` (Integer) — The timer id for the timer that you want to stop.

**Returns:** `nil` — 

[Docs](https://ruby.sketchup.com/UI.html#stop_timer-class_method)

#### `toolbar(name) => UI::Toolbar`

The toolbar method is used to get a Ruby toolbar by name.

**Remarks:** The toolbar method is used to get a Ruby toolbar by name. If the toolbar doesn't exist a new one will be created.

**Parameters:**
- `name` (String) — The name of the Ruby toolbar.

**Returns:** `UI::Toolbar` — a Toolbar object

[Docs](https://ruby.sketchup.com/UI.html#toolbar-class_method)

#### `toolbar_names => Array<String>`

The toolbar_names method is used to return the name of all the available native toolbars (this differs between PC and Mac).

**Remarks:** The toolbar_names method is used to return the name of all the available native toolbars (this differs between PC and Mac). These toolbar names do not include Ruby toolbars.

**Returns:** `Array<String>` — Array of strings representing toolbar names.

[Docs](https://ruby.sketchup.com/UI.html#toolbar_names-class_method)

#### `toolbar_visible?(name) => Boolean`

The toolbar_visible? method is used to determine whether a given toolbar is visible.

**Remarks:** The toolbar_visible? method is used to determine whether a given toolbar is visible. Note that the toolbars and their names are different on the Mac vs. PC, so be careful and be sure to test when using this method in a cross-platform script.

**Parameters:**
- `name` (String) — The name of a native toolbar.

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/UI.html#toolbar_visible?-class_method)

