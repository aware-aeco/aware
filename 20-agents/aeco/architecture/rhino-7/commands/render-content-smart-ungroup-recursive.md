# render-content-smart-ungroup-recursive

Lifecycle: single

Remove this content and all its children from any instance groups they may be a member of.             If any content in the same document is left alone in the group, that content is also ungrouped.             Records undo and sends events OnContentChanged and OnContentGroupIdChanged.             \note This method is designed to be called from a content UI and is intended for RDK internal             use but may be used as an expert user override.
