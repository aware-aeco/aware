# render-content-set-child

Lifecycle: single

Set another content as a child of this content. This content may or may             not be attached to a document.  If this content already has a child             with the specified child slot name, that child will be deleted.  If             this content is not attached to a document, the child will be added             without sending any events.  If this content is attached to a document,             the necessary events will be sent to update the UI.             Note:               Do not call this method to add children in your constructor. If you               want to add default children, you should override Initialize() and add               them there.
