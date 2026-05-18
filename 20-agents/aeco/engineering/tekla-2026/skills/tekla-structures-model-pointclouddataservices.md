---
name: tekla-tekla-structures-model-pointclouddataservices
description: This skill encodes the tekla 2026.0 surface of the Tekla.Structures.Model.PointCloudDataServices namespace — 3 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: PointCloudDataServices, TCProject, TCPointCloud.
---

# Tekla.Structures.Model.PointCloudDataServices

Auto-generated from vendor docs for tekla 2026.0. 3 types in this namespace.

## PointCloudDataServices (class)

The point cloud data services. Examples The following example shows how to use the PointCloudDataServices class to retrieve point cloud data in Trimble Connect projects and attach it to a model. Copyusing System.Threading; using Tekla.Structures.Model; public class Example { public async void Example1() { var projects = await PointCloudDataServices.GetProjectsWithPointCloudsAsync(); var pointClouds = projects.First().PointClouds; var url = await PointCloudDataServices.GetPointCloudUrlAsync(projects.First(), pointClouds.First()); PointCloud pointCloud = new PointCloud { Name = pointClouds.First().Name, Url = url, TcProjectId = projects.First().Id, TcPointCloudGuid = pointClouds.First().Id, LocationBy = Guid.Empty, UseAutoCreatedBasePoint = true, Scale = 1.0 }; pointCloud.Attach(); } }

[Vendor docs](https://developer.tekla.com/topic/en/18/47/acb39e25-f698-8918-8981-ab466ed1d0d7)

### Methods
#### `public static Task<string> GetPointCloudUrlAsync(TCProject project, TCPointCloud pointCloud)`

Retrieves the URL for a point cloud.

**Parameters:**
- `project` (Tekla.Structures.Model.PointCloudDataServices.TCProject) — The project for which to get the URL.
- `pointCloud` (Tekla.Structures.Model.PointCloudDataServices.TCPointCloud) — The point cloud for which to get the URL.

**Returns:** `Task<String>` — A task that represents the asynchronous operation. The task result contains a URL.

[Docs](https://developer.tekla.com/topic/en/18/47/6be42c9c-b8bd-4539-55bb-b40a3311dfc7)

#### `public static Task<IEnumerable<TCProject>> GetProjectsWithPointCloudsAsync()`

Retrieves the list of Trimble Connect projects with point clouds.

**Returns:** `Task<IEnumerable<TCProject>>` — A task that represents the asynchronous operation. The task result contains a list of projects that have point clouds.

[Docs](https://developer.tekla.com/topic/en/18/47/653fec87-3c1e-7c9f-23ed-c8a1cfcd9849)

## TCPointCloud (struct)

Represents a point cloud in Trimble Connect project with metadata such as ID, name, file size, point count, and uploader information.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/23881e7d-cebf-6a60-a990-6468e8e1b877)

## TCProject (struct)

Represents a Trimble Connect Project tenancy with point cloud data services.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/0c2dfb0d-0a8c-46a9-d291-a338eeaa2fdb)

