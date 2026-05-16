# render-content-serializer-report-deferred-content-and-file

Lifecycle: single

This is called from your implementation of LoadMultiple() to add a 'deferred' content and the             file it will be loaded from when the LoadMultipleFlags.Preload flag is set.             See LoadMultiple() for an explanation of this method's use.             \param c is the deferred content.             \param wszFullPath is the full path to the file that 'c' will be loaded from.             \param flags is reserved for future use; you should pass zero.             \param pReserved is reserved for future use; you should pass nullptr. */
