# gh-component-param-server-sync

Lifecycle: single

Sync changes from a prerecorded state.   All param references in the sync object which are no longer   present in the current server will be properly deleted.   The sync object will be reset in the process, so you can only call this function   with a specific Sync object once.
