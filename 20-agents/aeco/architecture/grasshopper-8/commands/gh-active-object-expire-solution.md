# gh-active-object-expire-solution

Lifecycle: single

Informs the document that owns this object that the solution has expired.  The current object will be set to BLANK as a result. This method is recursive,   it will also expire any and all objects which depend on this object.   If you want a less destructive expiration, consider using ClearData().   If this object is already Blank, you should consider not expiring it.
