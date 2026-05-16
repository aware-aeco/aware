# mesh-require-iterative-cleanup

Lifecycle: single

Analyzes some meshes, and determines if a pass of CreateFromIterativeCleanup would change the array.             All available cleanup steps are used. Currently available cleanup steps are:- mending of single precision coincidence even though double precision vertices differ.- union of nearly identical vertices, irrespectively of their origin.- removal of t-joints along edges.
