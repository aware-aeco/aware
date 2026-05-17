---
name: dynamo-applications-proto-ffi
description: API reference for namespace ProtoFFI from ProtoCore.dll
---

# ProtoFFI

- **CLRDLLModule**
  - Implements DLLModule for CLR types and FFI. This class supports .NET             module import to DesignScript and provides FFIFunctionPointer &             FFIObjectMarshler.
- **CLRModuleType**
  - This class creates ClassDeclNode for a given type and caches all the             imported types. This class also keeps the list of FFIFunctionPointer             for the given type.
- **CSModuleHelper**
  - Helper class to load CLR modules.
- **DLLFFIHandler**
- **DLLModule**
- **FFIClassAttributes**
  - It keeps FFI class's attributes.
- **FFIFunctionPointer**
- **FFIHandler**
- **FFILanguage**
- **FFIMethodAttributes**
  - It keeps FFI method's attributes.
- **FFIObjectMarshaler**
  - This class is responsible for marshaling of FFI objects to DS world and              vice-versa.
- **FFIParamAttributes**
  - A parameter's attributes.
- **ImportModuleHandler**
- **ModuleHelper**
- **PInvokeDLLModule**
- **PInvokeFunctionPointer**
- **PInvokeModuleHelper**
- **PointerValueComparer**
  - This class compares two Pointer type StackValue objects.
- **ReferenceEqualityComparer**
  - This class compares two CLR objects using reference equality. It is used in CLRObjectMap             to map CLR object instances to their marshaled StackValue representations. Uses              to get the object's identity hash code, which             ensures well-distributed hash codes even when objects have value-based hash codes that             collide (e.g., Point objects with identical coordinates).             Note: The hash code computation is intentionally aligned with the reference             equality behavior used by . This ensures consistent             semantics between equality comparison and hash code generation, which is a requirement             for proper  implementation.
