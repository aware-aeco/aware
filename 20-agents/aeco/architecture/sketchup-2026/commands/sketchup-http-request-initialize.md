# sketchup-http-request-initialize

Lifecycle: single

If no reference is kept to the Sketchup::Http::Request, it can be garbage collected, making the download silently fail. This is especially noticeable for larger downloads that takes longer time.
