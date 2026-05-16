# plug-in-raise-on-plug-in-settings-saved-event

Lifecycle: single

Raise any pending OnPlugInSettingsSaved events, the events are normally             queued while a command is running and fired while Rhino is in an             idle state.  Calling this method will raise any pending changed events             regardless of Rhino's current idle state or if a command is running.
