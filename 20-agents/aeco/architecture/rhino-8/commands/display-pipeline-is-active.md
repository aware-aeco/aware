# display-pipeline-is-active

Lifecycle: single

Determines if an object can be visible in this viewport based on it's object type and display attributes.              This test does not check for visibility based on location of the object.              NOTE: Use CRhinoDisplayPipeline::IsVisible() to perform "visibility"                    tests based on location (is some part of the object in the view frustum).                    Use CRhinoDisplayPipeline::IsActive() to perform "visibility"                    tests based on object type.
