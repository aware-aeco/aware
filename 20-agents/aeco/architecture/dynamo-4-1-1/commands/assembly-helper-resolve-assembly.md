# assembly-helper-resolve-assembly

Lifecycle: single

Handler to the ApplicationDomain's AssemblyResolve event.             If an assembly's location cannot be resolved, an exception is             thrown. Failure to resolve an assembly will leave Dynamo in              a bad state, so we should throw an exception here which gets caught              by our unhandled exception handler and presents the crash dialogue.
