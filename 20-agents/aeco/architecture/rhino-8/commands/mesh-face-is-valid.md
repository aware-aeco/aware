# mesh-face-is-valid

Lifecycle: single

Gets a value indicating whether or not this mesh face              is considered to be valid. Note that even valid mesh faces              could potentially be invalid in the context of a specific Mesh,              if one or more of the corner indices exceeds the number of              vertices on the mesh. If you want to perform a complete              validity check, use IsValid(int) instead.
