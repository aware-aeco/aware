# render-content-end-change

Lifecycle: single

Ends a change or batch of changes. Calls to this method are counted;             you must call this method once for every call to .             Note:               If  was called with ChangeContexts.UI,               ChangeContexts.Program, ChangeContexts.Drop or ChangeContexts.UI.Tree               and Changed() was called between the calls to  and               EndChange(), the last call to EndChange() will raise the  event.
