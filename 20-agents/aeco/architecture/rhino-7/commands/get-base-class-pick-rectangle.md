# get-base-class-pick-rectangle

Lifecycle: single

If the get was a GetObjects() and the mouse was used to select the objects,             then the returned rectangle has left < right and top < bottom. This rectangle             is the Windows GDI screen coordinates of the picking rectangle.             RhinoViewport.GetPickXform( pick_rect, pick_xform )             will calculate the picking transformation that was used.             In all other cases, left=right=top=bottom=0;
