# post-effect-read-from-document-defaults

Lifecycle: single

Read the state from document defaults. This is implemented by RDK to call ReadState()             so usually a post effect only has to implement ReadState(). However, a post effect can override this             method to take complete control of how the document defaults are read.
