# notification-execute-assembly-protected-code

Lifecycle: single

If a  object is only allowed to be modified by certain             assemblies, then any code that interacts with it must be wrapped around this method,             or a  will be thrown. For performance reasons,             the code wrapped by this method should be kept as simple as possible.
