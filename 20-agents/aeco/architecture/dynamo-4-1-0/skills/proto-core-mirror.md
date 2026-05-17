---
name: dynamo-applications-proto-core-mirror
description: API reference for namespace ProtoCore.Mirror from ProtoCore.dll
---

# ProtoCore.Mirror

- **ClassMirror**
  - A ClassMirror object reflects upon the type of a single designscript variable              The information here is populated during the code generation phase
- **LibraryMirror**
  - The LibraryMirror reflects upon an assembly or DS file to return assembly specific information             such as imported classes, global methods, etc.
- **MethodMirror**
  - Reflects upon a Function to retrieve its arguments
- **MirrorData**
  - An object that performs marshalling of all relevant data associated with this object
- **MirrorObject**
  - An abstract MirrorObject that represents a generic DesignScript object that can reflected             Reflection on this object can be done at compiletime or runtime
- **PropertyMirror**
- **Reflection**
- **RuntimeMirror**
  - A RuntimeMirror object is used to reflect on the runtime status of a single designsript variable
- **StaticMirror**
  - StaticMirror is a base class representing all Mirror classes that              perform static (build time) reflection on types             Static reflection can be done without executing the code
