---
name: tekla-structures-model-tekla-structures-model-ui
description: API reference for namespace Tekla.Structures.Model.UI from Tekla.Structures.Model.dll
---

# Tekla.Structures.Model.UI

- **ClipPlane**
  - The ClipPlane class defines a clip plane which can be used together with a visible view.
- **ClipPlaneCollection**
  - The ClipPlaneCollection class handles the collection of the clip planes.
- **Color**
  - The Color class represents an RGB color with transparency. The color values must be between 0.0 and 1.0.
- **GraphicPolyLine**
- **GraphicsDrawer**
  - The GraphicsDrawer class draws temporary graphics in the currently active rendered view in Tekla Structures.
- **Mesh**
  - The Mesh class represents a mesh for drawing three-dimensional data.
- **ModelObjectSelector**
  - The ModelObjectSelector class can be used to select objects from the Tekla Structures user interface.             Currently, these selections both select the objects from the database and highlight them visually.
- **ModelObjectVisualization**
  - The class to set and clear temporary visualization (color and transparency) for model objects in view.              Permanent representation will be restored when view is redrawn or temporary visualization is cleared.             Can be used also to fetch current permanent representation of model object.
- **ModelViewEnumerator**
  - The ModelViewEnumerator class is an enumerator class for model views.             The enumerator enables model view items to be looped.
- **PickInput**
  - The PickInput class handles the input of picked objects and positions.
- **Picker**
  - The Picker class can be used to query the user to do manual picks of objects and points from the Tekla Structures model.             The methods throw an exception if the user interrupts (cancels) the pick command.
- **SnapshotSettings**
- **TemporaryTransparency**
  - The possible temporary transparencies. Used to temporarily change transparency             of a model object to visualize some state in the model.
- **View**
  - The View class contains methods related to views.
- **ViewCamera**
  - The ViewCamera class defines a camera which can be used together with a visible view.              Always supply a properly orthogonalized camera up vector when rotating the camera.
- **ViewHandler**
  - The ViewHandler class contains methods for handling views.
- **ViewVisibilitySettings**
  - The View visibility settings class contains object visibility information related to view.
