# gh-fade-animation-evaluate

Lifecycle: single

Reevaluate the fade animation parameters. You should call this   method from within the Render method of your attributes. If an animation   is currently in progress a redraw of the canvas will be scheduled, whether   or not the attributes are actually visible on the canvas. You should therefore   only call this method if your attributes are already drawing themselves.
