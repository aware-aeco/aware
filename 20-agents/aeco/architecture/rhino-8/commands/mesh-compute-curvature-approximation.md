# mesh-compute-curvature-approximation

Lifecycle: single

Compute an approximation of the discrete curvatures of the mesh vertices, according to which             type of curvature information is requested.             This method will not yield meaningful values on nonmanifold vertices, and nan on naked vertices.             For now, this method operates solely on the mesh topology (the fully welded mesh),             but distribtes the result across all the ordinary vertices.             This method is a const and thread-safe method and will leave the m_K array untouched.                          An integer indicating which curvature is desired.             gaussian_curvature = 1, mean_curvature = 2, minimum unsigned radius of curvature = 3, maximum unsigned radius of curvature = 4                          Resulting curvature array on success, null on failure. On success, the length of the array is the number of vertices.
