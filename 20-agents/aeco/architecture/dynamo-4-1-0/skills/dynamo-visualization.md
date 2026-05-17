---
name: dynamo-applications-dynamo-visualization
description: API reference for namespace Dynamo.Visualization from DynamoCore.dll
---

# Dynamo.Visualization

- **DefaultRenderPackage**
  - Example implementation of IRenderPackage.             DefaultRenderPackage can be used as package for your visualization.
- **DefaultRenderPackageFactory**
  - Example implementation of IRenderPackageFactory.             DefaultRenderPackageFactory produces DefaultRenderPackages.
- **IRenderPackageFactory**
  - IRenderPackageFactory is used to create IRenderPackage objects suitable              for a specific rendering pipeline. IRenderPackages generated from IRenderPackageFactory              classes contain tessellated geometry for rendering, which may be stored              in different forms depending on the rendering pipeline being used.
- **IRenderPackageSource`1**
  - This interface has events that is fired when the render packages are changed.
- **RenderPackageCache**
  - This class controls the collection and distribution of render packages
