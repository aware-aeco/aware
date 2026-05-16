# render-content-get-child-slot-parameter

Lifecycle: single

Extra requirements are a way of specifying extra functionality on parameters in the automatic UI.             Override this function to specify additional functionality for automatic UI sections or the texture summary.             See IAutoUIExtraRequirements.h in the C++ RDK SDK for string definitions for the parameter names.             Call the base class from your override if you do not support the extra requirement parameter.             Please do not call this function. It is only retained for backward compatibility. You should instead             call GetExtraRequirementParameter().
