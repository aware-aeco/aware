# updater-data-is-change-triggered

Lifecycle: single

Allows updater to check if specific change has happened to an element.    Compares input type to the types that caused Updater::execute() to be triggered.    If input type was not registered as a trigger for the associated Updater, this    method will always return false for that ChangeType.    For example, if the only trigger registered for UpdaterX is ChangeTypeAny for Element A,    then passing in ChangeTypeGeometry will return false even if the geometry of A changed because    the registered trigger was ChangeTypeAny. However, passing in ChangeTypeAny will return true.
