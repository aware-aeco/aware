# render-content-begin-change

Lifecycle: single

Begins a change or batch of changes. It may also make a             copy of the content state allowing  to send an             event with the old and new contents. Calls to this method are counted;             you must call EndChange() once for every call to BeginChange().             Note:               If Changed() was called between the calls to BeginChange() and               EndChange(), the last call to EndChange() may cause the ContentChanged               event to be sent.
