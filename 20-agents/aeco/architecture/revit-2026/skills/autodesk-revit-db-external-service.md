---
name: ad-windows-autodesk-revit-db-external-service
description: API reference for namespace Autodesk.Revit.DB.ExternalService from RevitAPI.dll
---

# Autodesk.Revit.DB.ExternalService

- **DisparityResponse**
  - An enumerated value to return from OnServerDiparity indicating    what the service wants Revit to do as the post-action of the call.
- **ExecutionPolicy**
  - Controls how servers of multi-server external services are executed.
- **ExternalService**
  - This base class represents an external service inside Revit application.
- **ExternalServiceId**
  - Unique identifier of an external service.
- **ExternalServiceOptions**
  - Various options affecting the behavior of an External Service
- **ExternalServiceRegistry**
  - This class gives access to external services.    Use it to register external services with Revit and execute them.    Only the application that registers a service is allowed to execute it.
- **ExternalServiceResult**
  - An enumerated value representing a result from executing an external service.
- **ExternalServices**
  - Provides a container of all Revit built-in ExternalServiceId instances.
- **IExternalData**
  - The base interface for data classes used when executing servers of external services.
- **IExternalServer**
  - The base interface for all external servers.
- **IExternalService**
  - The base interface class for all external services.
- **IMultiServerService**
  - The base interface class for all multi-server services.
- **ISingleServerService**
  - The base interface class for all single-server services.
- **MultiServerService**
  - This class represents a multi-server service inside Revit application.    It is created when an instance of IMultiServerService is registered with Revit.
- **ServerChangeCause**
  - Indicates the cause for the active server to be changed
- **SingleServerService**
  - This class represents a single-server service inside Revit application.    It is created when an instance of ISingleServerService is registered with Revit.
