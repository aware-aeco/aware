---
name: tekla-structures-model-tekla-structures-model-internal-bim-model-data-product
description: API reference for namespace Tekla.Structures.ModelInternal.BimModelDataProduct from Tekla.Structures.Model.dll
---

# Tekla.Structures.ModelInternal.BimModelDataProduct

- **AccessControlDto**
  - Represents an entry in the access control list for a BIM model, associating a principal with an access level.             Note: Model ID and creation info have been removed from this DTO as they are not needed for the UI.
- **AccessControlListEntry**
  - Represents an entry in the access control list for a BIM model, associating a principal with an access level.             Note: Model ID and creation info have been removed from this DTO as they are not needed for the UI.
- **BimModelDataProductClient**
  - Client for interacting with the BimModelDataProduct API to manage BIM models.
- **BimModelDataProductMetadata**
  - Represents information about the support and version recommendations for a product.
- **CompatibilityCheckResult**
  - The results of TS-Bim Model data product compatibility check.
- **CompatibilityStatus**
  - Compatibility status between Tekla Structures and Bim Model data product.
- **CurieParser**
  - Parses a CURIE (Compact Uniform Resource Identifier) string into its components.
- **CurieResult**
  - Represents a result of a CURIE (Compact Uniform Resource Identifier) resolution.
- **ICloudModelInfo**
  - An interface for model information in the cloud.
- **ICloudModelVersionInfo**
  - An interface for a specific model version information in the cloud.
- **ListModelOptions**
  - Contains optional filters for listing and searching for models.             Options can be used to filter models by access level, name, schema version, state, tags, type, view, and organization ID.             Corresponds to the GetModelsParams in Structural Data Product API.
- **ModelUpdate**
  - Represents an update to be applied to a model in the Structural Data Product.             All properties are optional; only the provided properties will be updated.             To restore deleted models, leave all properties null.
- **ModelVersionInit**
  - Represents the payload to create a new BIM model to be used by the application.
- **UnsupportedTsVersionException**
  - Custom exception thrown when the TS version in use is not supported by the structural-data-product.
