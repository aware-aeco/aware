# layout-sketch-up-model-output-entities

Lifecycle: single

The #output_entities method returns the Group that represents the Layout::SketchUpModel in its exported form. The Group will contain a Image for raster and hybrid-rendered models, and will contain a Group of LayOut entities for vector and hybrid-rendered models. This takes into account the output resolution set in the document's PageInfo, and the render mode override set on the document.
