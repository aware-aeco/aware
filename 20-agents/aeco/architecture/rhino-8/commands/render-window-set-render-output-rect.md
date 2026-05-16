# render-window-set-render-output-rect

Lifecycle: single

Specify a sub-rectangle of the render window to indicate where the relevant render output has             been written. For example, if a renderer chooses to render at half resolution, it could write             its data to the top left quadrant of the render window, which can be indicated by setting the             render output rectangle.             This is set to the full render window size by default. When the render window size is modified,             the render output rectangle will be set to reflect the new size.
