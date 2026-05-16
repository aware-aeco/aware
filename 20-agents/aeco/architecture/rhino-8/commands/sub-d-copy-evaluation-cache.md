# sub-d-copy-evaluation-cache

Lifecycle: single

Expert function that copies cached evaluations of component subdivision points and             limit surface information from src to this. Typically this is done for performance             critical situations like control point editing:               - Copy a SubD to be modified (this does not copy the evaluation cache)               - Copy the evaluation cache from the unmodified SubD               - Modify the SubD copy               - Update the surface mesh cache so that only the modified parts are recalculated               - Display, meshing, bounding boxes on the modified SubD are now available
