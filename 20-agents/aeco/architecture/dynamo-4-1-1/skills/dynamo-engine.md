---
name: dynamo-applications-dynamo-engine
description: API reference for namespace Dynamo.Engine from DynamoCore.dll
---

# Dynamo.Engine

- **AstBuiltEventHandler**
  - This is a delegate used in AstBuilt event.
- **CompilationServices**
  - This class is used to precompile code block node.             Also it's used as helper for resolving code in Input and Output nodes.
- **EngineController**
  - A controller to coordinate the interactions between some DesignScript             sub components like library management, live runner and so on.
- **FunctionDescriptor**
  - Describe a DesignScript function in a imported library
- **FunctionDescriptorParams**
  - Contains parameters for function description.
- **FunctionGroup**
  - A group of overloaded functions
- **FunctionType**
  - The type of a function.
- **IFunctionDescriptor**
  - Describes a function, whether imported or defined in a custom node.
- **ITraceReconciliationProcessor**
- **LibraryServices**
  - LibraryServices is a singleton class which manages builtin libraries                 as well as imported libraries. It is across different sessions.
- **LiveRunnerServices**
  - This is a helper class that can get mirror data from live runner, update graph etc.
- **XmlDocumentationExtensions**
  - Provides extension methods for reading XML documentation from reflected members.
