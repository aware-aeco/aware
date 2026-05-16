---
name: rhino-common-rhino-render-custom-render-meshes
description: API reference for namespace Rhino.Render.CustomRenderMeshes from RhinoCommon.dll
---

# Rhino.Render.CustomRenderMeshes

- **CustomRenderMeshProviderAttribute**
  - Attributes for RenderMeshProvider
- **Instance**
  - The core of the custom render primitive delivery system - and instance defines a single mesh.             Each instance has a shared mesh and a transform, along with a material, mapping channels (at the mesh and instance level).
- **MeshProviderIds**
  - Built in primitive provider ids.
- **RenderMeshProvider**
  - A RenderMeshProvider delivers custom render primitives (in the form of RenderMeshes).  Derive a public class from this with a public constructor,             and this primitive provider will be added to the RDK forming a collection of providers.             Each RenderMeshProvider::HasCustomRenderMeshes will be called and if it returns true, a call to RenderMeshes will be made. It is up to the provider             to cache its own primitives - the IRenderMeshes::ProviderTracking class is provided for that.             A provider may optionally return a collection of non-object Ids that it will provide custom render primitives for.  An example of this is Grasshopper, which             which will typically return a collection of the Ids of each CustomPreview component.             Override this class if you are a plug-in developer intending to supply a custom set of primitives for a given object, or objectId.  Examples of IRenderMeshProviders             are CurvePiping, EdgeSoftening, Displacement, Shutlining, Grasshopper's CustomPreview component.
- **RenderMeshProviderProgress**
  - Expresses the progress of a custom render mesh provider.
- **RenderMeshes**
  - RenderMeshes is a collection of geometry instances for a given ObjectId typically returned by the Custom Render primitive system.             As each set of primitives is returned by the RenderMeshProviders in turn, the running hash is updated with new information             about the primitive modifications that have been made along the way.
