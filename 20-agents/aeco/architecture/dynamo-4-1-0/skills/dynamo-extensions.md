---
name: dynamo-applications-dynamo-extensions
description: API reference for namespace Dynamo.Extensions from DynamoCore.dll
---

# Dynamo.Extensions

- **EnumDescriptionAttribute**
  - Provides description for enum member.
- **ExtensionData**
- **ExtensionDefinition**
- **ExtensionLibraryLoader**
  - Provides functionality for loading node's DLLs
- **ExtensionLoader**
  - Provides functionality for loading Dynamo's extensions.             This class loads formatted XMLs which contain information about             *Extension.dll and type name of IExtension inheritor.                          Example:             ..\ExtensionName.dllDynamo.ExtensionName.ExtensionTypeName
- **ExtensionManager**
  - This class handles registration, lookup, and disposal of extensions.
- **ICommandExecutive**
  - Interface to Dynamo's recordable command framework
- **IExtension**
  - An extension to the model layer of Dynamo
- **IExtensionLoader**
  - Handles loading extensions given an extension definition files path
- **IExtensionManager**
  - This class handles registration, lookup, and disposal of extensions.  There should only              be one of these per application instance.
- **IExtensionSource**
  - An object which may request extensions to be loaded and added to the extensionsManager.
- **IExtensionStorageAccess**
- **IServiceManager**
  - Defines a mechanism for registering and retrieving a service object;              that is, an object that provides custom support to other objects.
- **LinterExtensionBase**
  - Base class for all LinterExtensions
- **ReadyParams**
  - Application-level handles provided to an extension when              Dynamo has started and is ready for interaction
- **StartupParams**
  - Application-level handles provided to an extension when             Dynamo is starting up and is not yet ready for interaction.
