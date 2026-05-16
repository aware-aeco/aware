---
name: yard-sketchup-pick-helper
description: Sketchup::PickHelper API reference (YARD)
---

# Sketchup::PickHelper API reference

The same Pickhelper instance is being reused by SketchUp. Don't extend, add methods or redefine methods on this object as it can clash with other extensions.

## Methods

- `all_picked` — The all_picked method is used to get an array of entities from the active entities from all the pick paths. Duplicates might occur if there are multiple pick paths for entities that ends in a group or component.
- `best_picked` — The best_picked method is used to retrieve the “best” entity picked (entity that you would have picked if you were using the select tool).
- `boundingbox_pick` — Used to pick a set of entities from a model from a Geom::BoundingBox. The pick criteria can be for completely-contained or partially-contained entities, similar to how the Selection tool works.
- `count` — The count method is used to count the number of entities picked (number of pick records)
- `depth_at` — The depth_at method retrieves the depth of one of the currently picked entities in the list of pick records.
- `do_pick` — The #do_pick method is used to perform the initial pick. This method is generally called before any other methods in the PickHelper class.
- `element_at` — The element_at method is used to retrieve a specific entity in the list of picked elements. This element will be from the active entities.
- `init` — The #init method is used to initialize the PickHelper for testing points.
- `leaf_at` — The leaf_at method retrieves the deepest thing in a pick path.
- `path_at` — The path_at method is used to retrieve the entire path for an entity in the pick list (as an array).
- `pick_segment` — The return value will be a negative index when a segment is picked.
- `picked_edge` — The picked_edge method is used to retrieve the “best” Edge picked.
- `picked_element` — The picked_element method retrieves the best drawing element, that is not an edge or a face, picked.
- `picked_face` — The picked_face method is used to retrieve the best face picked.
- `test_point` — The #test_point method is used to test a point to see if it would be selected using the default or given pick aperture.
- `transformation_at` — The transformation_at method is used to get a transformation at a specific pick path index in the pick helper.
- `view` — The #view method is used to get the view associated with the Sketchup::PickHelper.
- `window_pick` — Prior to SketchUp 2025.0 this method expected physical screen coordinates. As of SketchUp 2025.0 they are expected to be logical screen coordinates.
