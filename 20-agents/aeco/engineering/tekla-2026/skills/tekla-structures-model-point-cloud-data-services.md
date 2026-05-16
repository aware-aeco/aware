---
name: tekla-structures-model-tekla-structures-model-point-cloud-data-services
description: API reference for namespace Tekla.Structures.Model.PointCloudDataServices from Tekla.Structures.Model.dll
---

# Tekla.Structures.Model.PointCloudDataServices

- **PointCloudDataServices**
  - The point cloud data services.                          The following example shows how to use the  class to retrieve point cloud data in Trimble Connect projects              and attach it to a model.                          using System.Threading;             using Tekla.Structures.Model;                          public class Example             {                 public async void Example1()                 {                     var projects = await PointCloudDataServices.GetProjectsWithPointCloudsAsync();                     var pointClouds = projects.First().PointClouds;                     var url = await PointCloudDataServices.GetPointCloudUrlAsync(projects.First(), pointClouds.First());                                          PointCloud pointCloud = new PointCloud                     {                         Name = pointClouds.First().Name,                         Url = url,                         TcProjectId = projects.First().Id,                         TcPointCloudGuid = pointClouds.First().Id,                         LocationBy = Guid.Empty,                         UseAutoCreatedBasePoint = true,                         Scale = 1.0                     };                                          pointCloud.Attach();                 }             }
- **TCPointCloud**
  - Represents a point cloud in Trimble Connect project with metadata such as ID, name, file size, point count, and uploader information.
- **TCProject**
  - Represents a Trimble Connect Project tenancy with point cloud data services.
