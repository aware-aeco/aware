---
name: allplan-stdreinfshapebuilder-generalreinfshapebuilder
description: This skill encodes the allplan 2025.0 surface of the StdReinfShapeBuilder.GeneralReinfShapeBuilder namespace — 1 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: Functions.
---

# StdReinfShapeBuilder.GeneralReinfShapeBuilder

Auto-generated from vendor docs for allplan 2025.0. 1 types in this namespace.

## Functions (static-class)

Module-level functions of StdReinfShapeBuilder.GeneralReinfShapeBuilder

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/StdReinfShapeBuilder/GeneralReinfShapeBuilder/)

### Methods
#### `create_circle_stirrup_with_user_hooks( radius: float, model_angles: RotationAngles, shape_props: ReinforcementShapeProperties, concrete_cover: float, overlap: float, hook_length_start: float, hook_angle_start: float, hook_length_end: float, hook_angle_end: float, ) -> BendingShape`

Create a circle stirrup shape. If rotation angles all set to 0, the shape is created in local XY coordinate system. The center of the reference circle is the (0,0) point.

**Remarks:** Create a circle stirrup shape. If rotation angles all set to 0, the shape is created in local XY coordinate system. The center of the reference circle is the (0,0) point.

**Parameters:**
- `radius` (float) — Radius of the reference circle.
- `model_angles` (RotationAngles) — Angles to rotate the shape from local to global coordinate system.
- `shape_props` (ReinforcementShapeProperties) — Shape properties.
- `concrete_cover` (float) — Concrete cover.
- `overlap` (float) — Overlap length.
- `hook_length_start` (float) — Length of the hook at the start. When set to 0, the length will be calculated automatically.
- `hook_angle_start` (float) — Hook angle at the start. When set to 0, it will be calculated automatically.
- `hook_length_end` (float) — Length of the hook at the end. When set to 0, the length will be calculated automatically.
- `hook_angle_end` (float) — Hook angle at the end. When set to 0, it will be calculated automatically.

**Returns:** `BendingShape` — Bar shape in world coordinates

[Docs](https://pythonparts.allplan.com/2025/api_reference/StdReinfShapeBuilder/GeneralReinfShapeBuilder/#StdReinfShapeBuilder.GeneralReinfShapeBuilder.create_circle_stirrup_with_user_hooks)

#### `create_freeform_shape_with_hooks( points: Point2DList | Point3DList | list[Point2D] | list[Point3D], model_angles: RotationAngles, shape_props: ReinforcementShapeProperties, concrete_cover: float, start_hook: float = 0, end_hook: float = 0, ) -> BendingShape`

Create a free form shape based on points. Optionally with standard 90° hooks at start and/or end of the shape.

**Remarks:** Create a free form shape based on points. Optionally with standard 90° hooks at start and/or end of the shape.

**Parameters:**
- `points` (Point2DList | Point3DList | list[Point2D] | list[Point3D]) — Points of the geometry side (min. 2 points)
- `model_angles` (RotationAngles) — Angles to rotate the shape from local to global coordinate system.
- `shape_props` (ReinforcementShapeProperties) — Shape properties.
- `concrete_cover` (float) — Concrete cover. Will be applied on all sides.
- `start_hook` (float) — Length of the hook at the start. Default value (0) calculates the length automatically. If set to -1: no hook.
- `end_hook` (float) — Length of the hook at the end. Default value (0) calculates the length automatically. If set to -1: no hook.

[Docs](https://pythonparts.allplan.com/2025/api_reference/StdReinfShapeBuilder/GeneralReinfShapeBuilder/#StdReinfShapeBuilder.GeneralReinfShapeBuilder.create_freeform_shape_with_hooks)

#### `create_hook_stirrup( length: float, model_angles: RotationAngles, shape_props: ReinforcementShapeProperties, concrete_cover_props: ConcreteCoverProperties, hook_length: float = 0, hook_start_angle: float = 180.0, hook_end_angle: float = 180.0, ) -> BendingShape`

Create a hook stirrup with user defined hooks at both ends. The shape is created in local XY coordinate system, along the X-axis.

**Remarks:** Create a hook stirrup with user defined hooks at both ends. The shape is created in local XY coordinate system, along the X-axis.

**Parameters:**
- `length` (float) — Length of the reference line
- `model_angles` (RotationAngles) — Angles to rotate the shape from local to global coordinate system.
- `shape_props` (ReinforcementShapeProperties) — Shape properties.
- `concrete_cover_props` (ConcreteCoverProperties) — Concrete covers. Relevant are: left and right cover.
- `hook_length` (float) — Length of both hooks. Default value (0) calculates the length automatically.
- `hook_start_angle` (float) — Angle of the left hook.
- `hook_end_angle` (float) — Angle of the right hook.

**Returns:** `BendingShape` — Bar shape in world coordinates

[Docs](https://pythonparts.allplan.com/2025/api_reference/StdReinfShapeBuilder/GeneralReinfShapeBuilder/#StdReinfShapeBuilder.GeneralReinfShapeBuilder.create_hook_stirrup)

#### `create_l_shape_with_hooks( length: float, width: float, model_angles: RotationAngles, shape_props: ReinforcementShapeProperties, concrete_cover_props: ConcreteCoverProperties, start_hook: float = 0, end_hook: float = 0, ) -> BendingShape`

Create an L-shape, optionally with standard 90° hooks at start and/or end. The shape is created in local XY coordinate system like this: ⅃

**Remarks:** Create an L-shape, optionally with standard 90° hooks at start and/or end. The shape is created in local XY coordinate system like this: ⅃

**Parameters:**
- `length` (float) — Length of the reference line parallel to X axis
- `width` (float) — Length of the reference line parallel to Y axis
- `model_angles` (RotationAngles) — Angles to rotate the shape from local to global coordinate system
- `shape_props` (ReinforcementShapeProperties) — Shape properties
- `concrete_cover_props` (ConcreteCoverProperties) — Concrete cover properties. Relevant are all: bottom, left, right and top.
- `start_hook` (float) — Hook length at the start. Default value (0) calculates the length automatically. If set to -1: no hook.
- `end_hook` (float) — Hook length at the end. Default value (0) calculates the length automatically. If set to -1: no hook.

**Returns:** `BendingShape` — Bar shape in world coordinates

[Docs](https://pythonparts.allplan.com/2025/api_reference/StdReinfShapeBuilder/GeneralReinfShapeBuilder/#StdReinfShapeBuilder.GeneralReinfShapeBuilder.create_l_shape_with_hooks)

#### `create_longitudinal_shape_with_anchorage( from_point: Point2D | Point3D, to_point: Point2D | Point3D, shape_props: ReinforcementShapeProperties, concrete_cover_props: ConcreteCoverProperties, start_anchorage: float = 0, end_anchorage: float = 0, ) -> BendingShape`

Create a straight bar shape, optionally with anchorage. The shape is created in the local XY coordinate system, along X axis.

**Remarks:** Create a straight bar shape, optionally with anchorage. The shape is created in the local XY coordinate system, along X axis.

**Parameters:**
- `from_point` (Point2D | Point3D) — Start point of the reference line.
- `to_point` (Point2D | Point3D) — End point of the reference line.
- `shape_props` (ReinforcementShapeProperties) — Shape properties.
- `concrete_cover_props` (ConcreteCoverProperties) — Concrete covers. Relevant are: bottom cover. If the anchorage length at start or end is set to 0, then also left or right cover respectively are considered.
- `start_anchorage` (float) — Anchorage length at the start.
- `end_anchorage` (float) — Anchorage length at the end.

**Returns:** `BendingShape` — Bar shape in world coordinates

[Docs](https://pythonparts.allplan.com/2025/api_reference/StdReinfShapeBuilder/GeneralReinfShapeBuilder/#StdReinfShapeBuilder.GeneralReinfShapeBuilder.create_longitudinal_shape_with_anchorage)

#### `create_longitudinal_shape_with_hooks( length: float, model_angles: RotationAngles, shape_props: ReinforcementShapeProperties, concrete_cover_props: ConcreteCoverProperties, start_hook: float = 0, end_hook: float = 0, ) -> BendingShape`

Create a straight bar shape, optionally with standard 90° hooks at start and/or end. The shape is created in the XY coordinate system, along X axis.

**Remarks:** Create a straight bar shape, optionally with standard 90° hooks at start and/or end. The shape is created in the XY coordinate system, along X axis.

**Parameters:**
- `length` (float) — Length of the reference line.
- `model_angles` (RotationAngles) — Angles to rotate the shape from local to global coordinate system
- `shape_props` (ReinforcementShapeProperties) — Shape properties
- `concrete_cover_props` (ConcreteCoverProperties) — Concrete covers. Relevant are: left, right and bottom cover
- `start_hook` (float) — Hook length at the start. Default value (0) calculates the length automatically. If set to -1: no hook.
- `end_hook` (float) — Hook length at the end. Default value (0) calculates the length automatically. If set to -1: no hook.

**Returns:** `BendingShape` — Bar shape in world coordinates

[Docs](https://pythonparts.allplan.com/2025/api_reference/StdReinfShapeBuilder/GeneralReinfShapeBuilder/#StdReinfShapeBuilder.GeneralReinfShapeBuilder.create_longitudinal_shape_with_hooks)

#### `create_longitudinal_shape_with_user_hooks( length: float, model_angles: RotationAngles, shape_props: ReinforcementShapeProperties, concrete_cover_props: ConcreteCoverProperties, start_hook: float = 0.0, end_hook: float = 0.0, start_hook_angle: float = 90.0, end_hook_angle: float = 90.0, hook_type_start: HookType | int = -1, hook_type_end: HookType | int = -1, ) -> BendingShape`

Create a straight bar shape, optionally with user defined hooks at start and/or end. The shape is created in the local XY coordinate system, along X axis.

**Remarks:** Create a straight bar shape, optionally with user defined hooks at start and/or end. The shape is created in the local XY coordinate system, along X axis.

**Parameters:**
- `length` (float) — Length of the reference line.
- `model_angles` (RotationAngles) — Angles to rotate the shape from local to global coordinate system
- `shape_props` (ReinforcementShapeProperties) — Shape properties
- `concrete_cover_props` (ConcreteCoverProperties) — Concrete covers. Relevant are: left, right and bottom cover
- `start_hook` (float) — Hook length at the start. Default value (0) calculates the length automatically. If set to -1: no hook.
- `end_hook` (float) — Hook length at the end. Default value (0) calculates the length automatically. If set to -1: no hook.
- `start_hook_angle` (float) — Hook angle at start. Value between [-180, 180]
- `end_hook_angle` (float) — Hook angle at end. Value between [-180, 180]
- `hook_type_start` (HookType | int) — Type of the hook at the start. By default determined based on angle.
- `hook_type_end` (HookType | int) — Type of the hook at the end. By default determined based on angle.

**Returns:** `BendingShape` — Bar shape in world coordinates

[Docs](https://pythonparts.allplan.com/2025/api_reference/StdReinfShapeBuilder/GeneralReinfShapeBuilder/#StdReinfShapeBuilder.GeneralReinfShapeBuilder.create_longitudinal_shape_with_user_hooks)

#### `create_open_stirrup( length: float, width: float, model_angles: RotationAngles, shape_props: ReinforcementShapeProperties, concrete_cover_props: ConcreteCoverProperties, start_hook: float = 0, end_hook: float = 0, start_hook_angle: float = 90.0, end_hook_angle: float = 90.0, hook_type: HookType | int = -1, ) -> BendingShape`

Create a rectangular stirrup shape, open on the top side. The shape is created in local XY coordinate system.

**Remarks:** Create a rectangular stirrup shape, open on the top side. The shape is created in local XY coordinate system.

**Parameters:**
- `length` (float) — Length of the reference rectangle (X-direction).
- `width` (float) — Width of the reference rectangle (Y-direction).
- `model_angles` (RotationAngles) — Angles to rotate the shape from local to global coordinate system.
- `shape_props` (ReinforcementShapeProperties) — Shape properties.
- `concrete_cover_props` (ConcreteCoverProperties) — Concrete covers: bottom, left, right and top.
- `start_hook` (float) — Length of the left hook. Default value (0) calculates the length automatically. If set to -1: no hook.
- `end_hook` (float) — Length of the right hook. Default value (0) calculates the length automatically. If set to -1: no hook.
- `start_hook_angle` (float) — Angle of the left hook [-180, 180]
- `end_hook_angle` (float) — Angle of the right hook [-180, 180]
- `hook_type` (HookType | int) — Type of the hook at the end. By default determined based on angle.

**Returns:** `BendingShape` — Bar shape in world coordinates

[Docs](https://pythonparts.allplan.com/2025/api_reference/StdReinfShapeBuilder/GeneralReinfShapeBuilder/#StdReinfShapeBuilder.GeneralReinfShapeBuilder.create_open_stirrup)

#### `create_s_hook( length: float, model_angles: RotationAngles, shape_props: ReinforcementShapeProperties, concrete_cover_props: ConcreteCoverProperties, hook_length: float = 0, ) -> BendingShape`

Create an S-hook shape with 180° hooks at both ends. The shape is created in local XY coordinate system, along the X-axis.

**Remarks:** Create an S-hook shape with 180° hooks at both ends. The shape is created in local XY coordinate system, along the X-axis.

**Parameters:**
- `length` (float) — Length of the reference line
- `model_angles` (RotationAngles) — Angles to rotate the shape from local to global coordinate system.
- `shape_props` (ReinforcementShapeProperties) — Shape properties.
- `concrete_cover_props` (ConcreteCoverProperties) — Concrete covers. Relevant are: left and right cover.
- `hook_length` (float) — Length of both hooks. Default value (0) calculates the length automatically.

**Returns:** `BendingShape` — Bar shape in world coordinates

[Docs](https://pythonparts.allplan.com/2025/api_reference/StdReinfShapeBuilder/GeneralReinfShapeBuilder/#StdReinfShapeBuilder.GeneralReinfShapeBuilder.create_s_hook)

#### `create_spacer( length: float, width: float, height: float, model_angles: RotationAngles, shape_props: ReinforcementShapeProperties, ) -> BendingShape`

Create a spacer shape. Created shape is 3-dimensional with total 5 legs. Concrete covers are not considered. Legs nr:

**Remarks:** Create a spacer shape. Created shape is 3-dimensional with total 5 legs. Concrete covers are not considered. Legs nr:

**Parameters:**
- `length` (float) — Length of the box bounding the shape (X-direction).
- `width` (float) — Width of the box bounding the shape (Y-direction).
- `height` (float) — Height of the box bounding the shape (Z-direction).
- `model_angles` (RotationAngles) — Angles to rotate the shape from local to global coordinate system.
- `shape_props` (ReinforcementShapeProperties) — Shape properties.

**Returns:** `BendingShape` — Bar shape in world coordinates

[Docs](https://pythonparts.allplan.com/2025/api_reference/StdReinfShapeBuilder/GeneralReinfShapeBuilder/#StdReinfShapeBuilder.GeneralReinfShapeBuilder.create_spacer)

#### `create_stirrup( length: float, width: float, model_angles: RotationAngles, shape_props: ReinforcementShapeProperties, concrete_cover_props: ConcreteCoverProperties, stirrup_type: StirrupType = Normal, hook_length: float = 0, ) -> BendingShape`

Create a rectangular stirrup shape with default hooks. The shape is created in local XY coordinate system.

**Remarks:** Create a rectangular stirrup shape with default hooks. The shape is created in local XY coordinate system.

**Parameters:**
- `length` (float) — Length of the reference rectangle (X-direction).
- `width` (float) — Width of the reference rectangle (Y-direction).
- `model_angles` (RotationAngles) — Angles to rotate the shape from local to global coordinate system.
- `shape_props` (ReinforcementShapeProperties) — Shape properties.
- `concrete_cover_props` (ConcreteCoverProperties) — Concrete covers: bottom, left, right and top.
- `stirrup_type` (StirrupType) — Type of the stirrup.
- `hook_length` (float) — Hook length. Default value (0) calculates the length automatically.

**Returns:** `BendingShape` — Bar shape in world coordinates

[Docs](https://pythonparts.allplan.com/2025/api_reference/StdReinfShapeBuilder/GeneralReinfShapeBuilder/#StdReinfShapeBuilder.GeneralReinfShapeBuilder.create_stirrup)

#### `create_stirrup_with_user_hooks( length: float, width: float, model_angles: RotationAngles, shape_props: ReinforcementShapeProperties, concrete_cover_props: ConcreteCoverProperties, stirrup_type: StirrupType, hook_length_start: float, hook_angle_start: float, hook_length_end: float, hook_angle_end: float, ) -> BendingShape`

Create a rectangular stirrup shape with user defined hooks. The shape is created in local XY coordinate system.

**Remarks:** Create a rectangular stirrup shape with user defined hooks. The shape is created in local XY coordinate system.

**Parameters:**
- `length` (float) — Length of the reference rectangle (X-direction).
- `width` (float) — Width of the reference rectangle (Y-direction).
- `model_angles` (RotationAngles) — Angles to rotate the shape from local to global coordinate system.
- `shape_props` (ReinforcementShapeProperties) — Shape properties.
- `concrete_cover_props` (ConcreteCoverProperties) — Concrete covers: bottom, left, right and top.
- `stirrup_type` (StirrupType) — Type of the stirrup.
- `hook_length_start` (float) — Hook length at start. When set to 0, the length is calculated automatically.
- `hook_angle_start` (float) — Hook angle at start. When set to 0, the angle is calculated automatically based on stirrup type.
- `hook_length_end` (float) — Hook length at end. When set to 0, the length is calculated automatically.
- `hook_angle_end` (float) — Hook angle at end. When set to 0, the angle is calculated automatically based on stirrup type.

**Returns:** `BendingShape` — Bar shape in world coordinates

[Docs](https://pythonparts.allplan.com/2025/api_reference/StdReinfShapeBuilder/GeneralReinfShapeBuilder/#StdReinfShapeBuilder.GeneralReinfShapeBuilder.create_stirrup_with_user_hooks)

#### `create_u_link( width: float, side_length: float, model_angles: RotationAngles, shape_props: ReinforcementShapeProperties, concrete_cover_props: ConcreteCoverProperties, hook_length: float, ) -> BendingShape`

Create a U-bar with both left and right legs of the same length. Optionally with hooks at both ends. The shape is created in local XY coordinate system, with its open side pointing upwards (in Y+ direction) like in the letter U.

**Remarks:** Create a U-bar with both left and right legs of the same length. Optionally with hooks at both ends. The shape is created in local XY coordinate system, with its open side pointing upwards (in Y+ direction) like in the letter U.

**Parameters:**
- `width` (float) — Width of the U-bar base line.
- `side_length` (float) — Length of both left and right leg.
- `model_angles` (RotationAngles) — Angles to rotate the shape from local to global coordinate system.
- `shape_props` (ReinforcementShapeProperties) — Shape properties.
- `concrete_cover_props` (ConcreteCoverProperties) — Concrete covers. Relevant are: left, right and bottom cover.
- `hook_length` (float) — Length of both hooks. When set to 0, it's calculated automatically. When set to -1, no hooks are created.

**Returns:** `BendingShape` — Bar shape of the u link shape in world coordinates

[Docs](https://pythonparts.allplan.com/2025/api_reference/StdReinfShapeBuilder/GeneralReinfShapeBuilder/#StdReinfShapeBuilder.GeneralReinfShapeBuilder.create_u_link)

#### `create_u_link_variable( width: float, side_length_start: float, side_length_end: float, model_angles: RotationAngles, shape_props: ReinforcementShapeProperties, concrete_cover_props: ConcreteCoverProperties, start_hook: float = 0.0, end_hook: float = 0.0, start_hook_angle: float = 90.0, end_hook_angle: float = 90.0, hook_type_start: HookType | int = -1, hook_type_end: HookType | int = -1, ) -> BendingShape`

Create an u-bar with different lengths of the left and right leg. Optionally with hooks at the end of right and/or left leg. The shape is created in local XY coordinate system, with its open side pointing upwards (in Y+ direction) like in the letter U.

**Remarks:** Create an u-bar with different lengths of the left and right leg. Optionally with hooks at the end of right and/or left leg. The shape is created in local XY coordinate system, with its open side pointing upwards (in Y+ direction) like in the letter U.

**Parameters:**
- `width` (float) — Width of the U-bar base line.
- `side_length_start` (float) — Length of the left leg.
- `side_length_end` (float) — Length of the right leg.
- `model_angles` (RotationAngles) — Angles to rotate the shape from local to global coordinate system.
- `shape_props` (ReinforcementShapeProperties) — Shape properties.
- `concrete_cover_props` (ConcreteCoverProperties) — Concrete covers. Relevant are: left, right and bottom cover.
- `start_hook` (float) — Length of the hook at the end of the left leg. When set to 0, it's calculated automatically. When set to -1, hook is not created.
- `end_hook` (float) — Length of the hook at the end of the right leg. When set to 0, it's calculated automatically. When set to -1, hook is not created.
- `start_hook_angle` (float) — Angle of the hook of the left leg [-180, 180].
- `end_hook_angle` (float) — Angle of the hook of the right leg [-180, 180].
- `hook_type_start` (HookType | int) — Type of the hook of the left leg. By default determined based on angle.
- `hook_type_end` (HookType | int) — Type of the hook of the right leg. By default determined based on angle.

**Returns:** `BendingShape` — Bar shape in world coordinates

[Docs](https://pythonparts.allplan.com/2025/api_reference/StdReinfShapeBuilder/GeneralReinfShapeBuilder/#StdReinfShapeBuilder.GeneralReinfShapeBuilder.create_u_link_variable)

#### `get_hook_type_from_angle( hook_type: HookType | int, hook_angle: float ) -> HookType`

Determines the hook type based on the given angle

**Remarks:** Determines the hook type based on the given angle

**Parameters:**
- `hook_type` (HookType | int) — Type of the hook. If set to -1, hook type is determined based on given angle. Otherwise, given hook type is returned.
- `hook_angle` (float) — Hook angle

**Returns:** `HookType` — Determined hook type

[Docs](https://pythonparts.allplan.com/2025/api_reference/StdReinfShapeBuilder/GeneralReinfShapeBuilder/#StdReinfShapeBuilder.GeneralReinfShapeBuilder.get_hook_type_from_angle)

