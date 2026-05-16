# post-effect-write-to-document-defaults

Lifecycle: single

Write the state to document defaults. This is implemented by RDK to call WriteState()             so usually a post effect only has to implement WriteState(). However, a post effect can override this             method to take complete control of how the document defaults are written.
