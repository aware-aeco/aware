# geometry-base-duplicate-shallow

Lifecycle: single

Constructs a light copy of this object. By "light", it is meant that the same             underlying data is used until something is done to attempt to change it. For example,             you could have a shallow copy of a very heavy mesh object and the same underlying             data will be used when doing things like inspecting the number of faces on the mesh.             If you modify the location of one of the mesh vertices, the shallow copy will create             a full duplicate of the underlying mesh data and the shallow copy will become a             deep copy.
