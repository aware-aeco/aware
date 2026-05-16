---
name: yard-sketchup-layers-observer-abstract
description: Sketchup::LayersObserver   Abstract API reference (YARD)
---

# Sketchup::LayersObserver   Abstract API reference

To implement this observer, create a Ruby class of this type, override the desired methods, and add an instance of the observer to the objects of interests.

## Methods

- `onCurrentLayerChanged` — The #onCurrentLayerChanged method is called when the user selects a different active layer.
- `onLayerAdded` — The #onLayerAdded method is called when a new layer is added to a model.
- `onLayerChanged` — The #onLayerChanged method is called when a layer is changed.
- `onLayerFolderAdded` — The #onLayerFolderAdded method is called when a layer folder is added to a model.
- `onLayerFolderChanged` — The #onLayerFolderChanged method is called when a layer folder changes one of its properties.
- `onLayerFolderRemoved` — The #onLayerFolderRemoved method is called when a layer folder is removed from a model.
- `onLayerRemoved` — The #onLayerRemoved method is called when a layer is removed from a model.
- `onParentFolderChanged` — When a folder changes parent #onLayerFolderRemoved and #onLayerFolderAdded triggers.
- `onRemoveAllLayers` — The #onRemoveAllLayers method is called when all layer are removed from a model.
