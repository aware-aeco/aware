# brep-builder-removed-some-faces

Lifecycle: single

Returns 'true' if BRepBuilder removed some problematic faces from the output geometry, 'false' if not.    If allowRemovalOfProblematicFaces was not called to enable removal of problematic faces, this function    will return 'false'. Note that if some faces were removed, the output geometry's type will be OpenShell    regardless of the expected type that was specified when the BRepBuilder was created.
