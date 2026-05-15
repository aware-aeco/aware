---
name: trimble-connect-viewpoints
description: Trimble Connect BCF Viewpoints API — camera position, orthogonal view, perspective view, snapshots, screenshots, selected components, colored components, visibility, clipping planes, BIM snippets, 3D view, model coordination. This skill should be used when working with Trimble Connect viewpoints, camera positions, snapshots, or 3D view manipulation.
---

# Trimble Connect — BCF Viewpoints API Reference (BCF 2.1)

All Viewpoint endpoints use the regional BCF base URL (see trimble-connect-auth skill).
Example base: `https://open11.connect.trimble.com/bcf/2.1`

All viewpoints belong to a topic: `/bcf/2.1/projects/{projectId}/topics/{topicId}/viewpoints`

---

## Viewpoints

### List Viewpoints

```
GET /bcf/2.1/projects/{projectId}/topics/{topicId}/viewpoints
```

**Response:** Array of Viewpoint objects

```csharp
var url = $"{bcfBaseUrl}/projects/{projectId}/topics/{topicId}/viewpoints";
var response = await client.GetAsync(url).ConfigureAwait(false);
var json = await response.Content.ReadAsStringAsync().ConfigureAwait(false);
using var doc = JsonDocument.Parse(json);

foreach (var vp in doc.RootElement.EnumerateArray())
{
    var guid = vp.GetProperty("guid").GetString();
    var index = vp.TryGetProperty("index", out var idx) ? idx.GetInt32() : 0;

    // Check which camera type is present
    var hasPerspective = vp.TryGetProperty("perspective_camera", out _);
    var hasOrthogonal = vp.TryGetProperty("orthogonal_camera", out _);
}
```

### Get Single Viewpoint

```
GET /bcf/2.1/projects/{projectId}/topics/{topicId}/viewpoints/{viewpointId}
```

**Response:** Single Viewpoint object with full camera and component data

### Create Viewpoint

```
POST /bcf/2.1/projects/{projectId}/topics/{topicId}/viewpoints
Content-Type: application/json
```

**Request Body (perspective camera):**
```json
{
    "index": 1,
    "perspective_camera": {
        "camera_view_point": { "x": 10.5, "y": 20.3, "z": 5.0 },
        "camera_direction": { "x": -0.5, "y": -0.7, "z": -0.1 },
        "camera_up_vector": { "x": 0.0, "y": 0.0, "z": 1.0 },
        "field_of_view": 60.0
    }
}
```

**Request Body (orthogonal camera):**
```json
{
    "index": 1,
    "orthogonal_camera": {
        "camera_view_point": { "x": 10.5, "y": 20.3, "z": 5.0 },
        "camera_direction": { "x": -0.5, "y": -0.7, "z": -0.1 },
        "camera_up_vector": { "x": 0.0, "y": 0.0, "z": 1.0 },
        "view_to_world_scale": 0.01
    }
}
```

**Code Example:**
```csharp
var url = $"{bcfBaseUrl}/projects/{projectId}/topics/{topicId}/viewpoints";
var payload = new Dictionary<string, object>
{
    ["index"] = 1,
    ["perspective_camera"] = new Dictionary<string, object>
    {
        ["camera_view_point"] = new Dictionary<string, double>
        {
            ["x"] = cameraX, ["y"] = cameraY, ["z"] = cameraZ
        },
        ["camera_direction"] = new Dictionary<string, double>
        {
            ["x"] = dirX, ["y"] = dirY, ["z"] = dirZ
        },
        ["camera_up_vector"] = new Dictionary<string, double>
        {
            ["x"] = 0.0, ["y"] = 0.0, ["z"] = 1.0
        },
        ["field_of_view"] = 60.0
    }
};

var jsonContent = JsonSerializer.Serialize(payload);
var content = new StringContent(jsonContent, System.Text.Encoding.UTF8, "application/json");
var response = await client.PostAsync(url, content).ConfigureAwait(false);
```

**Response:** Created Viewpoint object with `guid` assigned by server

---

## Viewpoint Object Schema

| Property | Type | Description |
|----------|------|-------------|
| `guid` | string | Server-assigned unique identifier (read-only) |
| `index` | int | Viewpoint order/index within topic |
| `perspective_camera` | object | Perspective camera settings (mutually exclusive with orthogonal) |
| `orthogonal_camera` | object | Orthogonal camera settings (mutually exclusive with perspective) |
| `lines` | object | Markup line elements drawn on the view |
| `clipping_planes` | object[] | Section/clipping planes |
| `snapshot` | object | Associated image snapshot metadata |
| `components` | object | Component selection, coloring, and visibility |

### Perspective Camera Schema

| Property | Type | Description |
|----------|------|-------------|
| `camera_view_point` | object | Camera position in world coordinates `{x, y, z}` |
| `camera_direction` | object | Camera look-at direction vector `{x, y, z}` |
| `camera_up_vector` | object | Camera up direction vector `{x, y, z}` |
| `field_of_view` | double | Vertical field of view in degrees (0-180) |

### Orthogonal Camera Schema

| Property | Type | Description |
|----------|------|-------------|
| `camera_view_point` | object | Camera position in world coordinates `{x, y, z}` |
| `camera_direction` | object | Camera look-at direction vector `{x, y, z}` |
| `camera_up_vector` | object | Camera up direction vector `{x, y, z}` |
| `view_to_world_scale` | double | Scale factor from view to world units |

### Point/Vector Schema (x, y, z)

Used by `camera_view_point`, `camera_direction`, `camera_up_vector`:

| Property | Type | Description |
|----------|------|-------------|
| `x` | double | X coordinate (meters in world space) |
| `y` | double | Y coordinate (meters in world space) |
| `z` | double | Z coordinate (meters in world space) |

### Clipping Plane Schema

| Property | Type | Description |
|----------|------|-------------|
| `location` | object | Point on the plane `{x, y, z}` |
| `direction` | object | Normal vector of the plane `{x, y, z}` |

---

## Snapshots

### Get Viewpoint Snapshot

```
GET /bcf/2.1/projects/{projectId}/topics/{topicId}/viewpoints/{viewpointId}/snapshot
```

**Response:** Image bytes (PNG or JPEG). Content-Type header indicates format.

```csharp
var url = $"{bcfBaseUrl}/projects/{projectId}/topics/{topicId}/viewpoints/{viewpointId}/snapshot";
var response = await client.GetAsync(url).ConfigureAwait(false);

if (response.IsSuccessStatusCode)
{
    var contentType = response.Content.Headers.ContentType?.MediaType; // "image/png" or "image/jpeg"
    var imageBytes = await response.Content.ReadAsByteArrayAsync().ConfigureAwait(false);

    // Save to file
    var extension = contentType == "image/jpeg" ? ".jpg" : ".png";
    var filePath = System.IO.Path.Combine(outputDir, $"snapshot_{viewpointId}{extension}");
    System.IO.File.WriteAllBytes(filePath, imageBytes);
}
```

### Get Viewpoint Bitmap

```
GET /bcf/2.1/projects/{projectId}/topics/{topicId}/viewpoints/{viewpointId}/bitmap
```

**Response:** Bitmap overlay image bytes. Same pattern as snapshot retrieval.

---

## Selected Components

### Get Selected Components

```
GET /bcf/2.1/projects/{projectId}/topics/{topicId}/viewpoints/{viewpointId}/selection
```

**Response:** Object with `selection` array of component references

```csharp
var url = $"{bcfBaseUrl}/projects/{projectId}/topics/{topicId}/viewpoints/{viewpointId}/selection";
var response = await client.GetAsync(url).ConfigureAwait(false);
var json = await response.Content.ReadAsStringAsync().ConfigureAwait(false);
using var doc = JsonDocument.Parse(json);

if (doc.RootElement.TryGetProperty("selection", out var selection))
{
    foreach (var component in selection.EnumerateArray())
    {
        var ifcGuid = component.TryGetProperty("ifc_guid", out var g) ? g.GetString() : null;
        var originatingSystem = component.TryGetProperty("originating_system", out var os) ? os.GetString() : null;
        var authoringToolId = component.TryGetProperty("authoring_tool_id", out var at) ? at.GetString() : null;
    }
}
```

---

## Colored Components

### Get Colored Components

```
GET /bcf/2.1/projects/{projectId}/topics/{topicId}/viewpoints/{viewpointId}/coloring
```

**Response:** Object with `coloring` array of color groups, each containing a color and list of components

```csharp
var url = $"{bcfBaseUrl}/projects/{projectId}/topics/{topicId}/viewpoints/{viewpointId}/coloring";
var response = await client.GetAsync(url).ConfigureAwait(false);
var json = await response.Content.ReadAsStringAsync().ConfigureAwait(false);
using var doc = JsonDocument.Parse(json);

if (doc.RootElement.TryGetProperty("coloring", out var coloring))
{
    foreach (var colorGroup in coloring.EnumerateArray())
    {
        // Color in hex ARGB format: "FF00FF00" = opaque green
        var color = colorGroup.GetProperty("color").GetString();

        foreach (var component in colorGroup.GetProperty("components").EnumerateArray())
        {
            var ifcGuid = component.TryGetProperty("ifc_guid", out var g) ? g.GetString() : null;
        }
    }
}
```

### Coloring Schema

| Property | Type | Description |
|----------|------|-------------|
| `coloring` | array | Array of color group objects |

**Color Group:**

| Property | Type | Description |
|----------|------|-------------|
| `color` | string | Hex ARGB color (e.g., `"FFFF0000"` = opaque red) |
| `components` | array | Array of component references to apply this color |

---

## Visibility

### Get Visibility Settings

```
GET /bcf/2.1/projects/{projectId}/topics/{topicId}/viewpoints/{viewpointId}/visibility
```

**Response:** Object with visibility configuration

```csharp
var url = $"{bcfBaseUrl}/projects/{projectId}/topics/{topicId}/viewpoints/{viewpointId}/visibility";
var response = await client.GetAsync(url).ConfigureAwait(false);
var json = await response.Content.ReadAsStringAsync().ConfigureAwait(false);
using var doc = JsonDocument.Parse(json);

if (doc.RootElement.TryGetProperty("visibility", out var visibility))
{
    // default_visibility: true = everything visible except exceptions
    //                     false = everything hidden except exceptions
    var defaultVis = visibility.TryGetProperty("default_visibility", out var dv) && dv.GetBoolean();

    // Exception list: components that differ from default
    if (visibility.TryGetProperty("exceptions", out var exceptions))
    {
        foreach (var component in exceptions.EnumerateArray())
        {
            var ifcGuid = component.TryGetProperty("ifc_guid", out var g) ? g.GetString() : null;
        }
    }
}
```

### Visibility Schema

| Property | Type | Description |
|----------|------|-------------|
| `visibility` | object | Visibility configuration |

**Visibility Object:**

| Property | Type | Description |
|----------|------|-------------|
| `default_visibility` | bool | `true` = all visible by default, `false` = all hidden by default |
| `exceptions` | array | Array of component references that are exceptions to the default |
| `view_setup_hints` | object | Optional hints for view configuration |

**View Setup Hints:**

| Property | Type | Description |
|----------|------|-------------|
| `spaces_visible` | bool | Whether spaces (rooms) should be visible |
| `space_boundaries_visible` | bool | Whether space boundaries should be visible |
| `openings_visible` | bool | Whether openings (doors/windows cutouts) should be visible |

---

## Component Reference Schema

Used across selection, coloring, and visibility:

| Property | Type | Description |
|----------|------|-------------|
| `ifc_guid` | string | IFC GlobalId of the element |
| `originating_system` | string | Source system identifier (e.g., "Tekla Structures") |
| `authoring_tool_id` | string | Element ID in the authoring tool |

**Note:** At least one property must be provided. `ifc_guid` is the most commonly used identifier for cross-system element referencing.

---

## BIM Snippets

### Get BIM Snippet

```
GET /bcf/2.1/projects/{projectId}/topics/{topicId}/bim_snippet
```

**Response:** Binary file content. Check `Content-Type` header for format.

```csharp
var url = $"{bcfBaseUrl}/projects/{projectId}/topics/{topicId}/bim_snippet";
var response = await client.GetAsync(url).ConfigureAwait(false);

if (response.IsSuccessStatusCode)
{
    var contentType = response.Content.Headers.ContentType?.MediaType;
    var bytes = await response.Content.ReadAsByteArrayAsync().ConfigureAwait(false);
}
```

### Upload BIM Snippet

```
PUT /bcf/2.1/projects/{projectId}/topics/{topicId}/bim_snippet
Content-Type: application/octet-stream
```

```csharp
var url = $"{bcfBaseUrl}/projects/{projectId}/topics/{topicId}/bim_snippet";
var fileBytes = System.IO.File.ReadAllBytes(snippetFilePath);
var content = new ByteArrayContent(fileBytes);
content.Headers.ContentType = new System.Net.Http.Headers.MediaTypeHeaderValue("application/octet-stream");

var request = new HttpRequestMessage(HttpMethod.Put, url) { Content = content };
var response = await client.SendAsync(request).ConfigureAwait(false);
```

### BIM Snippet Metadata (in Topic object)

When a BIM snippet is attached, the topic object includes:

| Property | Type | Description |
|----------|------|-------------|
| `bim_snippet.snippet_type` | string | Type identifier (e.g., `"ifcZip"`, `"ifc"`, `"json"`) |
| `bim_snippet.is_external` | bool | Whether snippet is hosted externally |
| `bim_snippet.reference` | string | URL if external, filename if internal |
| `bim_snippet.reference_schema` | string | Schema URL for the snippet format |

---

## Creating a Complete Viewpoint (Full Example)

Create a viewpoint with camera, selection, coloring, and visibility in one request:

```csharp
var url = $"{bcfBaseUrl}/projects/{projectId}/topics/{topicId}/viewpoints";
var payload = new Dictionary<string, object>
{
    ["index"] = 1,
    ["perspective_camera"] = new Dictionary<string, object>
    {
        ["camera_view_point"] = new Dictionary<string, double>
        {
            ["x"] = 15.0, ["y"] = 25.0, ["z"] = 10.0
        },
        ["camera_direction"] = new Dictionary<string, double>
        {
            ["x"] = -0.6, ["y"] = -0.8, ["z"] = -0.2
        },
        ["camera_up_vector"] = new Dictionary<string, double>
        {
            ["x"] = 0.0, ["y"] = 0.0, ["z"] = 1.0
        },
        ["field_of_view"] = 60.0
    },
    ["components"] = new Dictionary<string, object>
    {
        ["selection"] = new[]
        {
            new Dictionary<string, string>
            {
                ["ifc_guid"] = "2MFv0RB6X7Jw8MzUP3bar0",
                ["originating_system"] = "Tekla Structures"
            }
        },
        ["coloring"] = new[]
        {
            new Dictionary<string, object>
            {
                ["color"] = "FFFF0000",
                ["components"] = new[]
                {
                    new Dictionary<string, string>
                    {
                        ["ifc_guid"] = "3kQ9Z1RBv0ow9MzUP2xyz1"
                    }
                }
            }
        },
        ["visibility"] = new Dictionary<string, object>
        {
            ["default_visibility"] = true,
            ["exceptions"] = new[]
            {
                new Dictionary<string, string>
                {
                    ["ifc_guid"] = "1bC3D4EfG5HiJ6KlM7NoP8"
                }
            },
            ["view_setup_hints"] = new Dictionary<string, bool>
            {
                ["spaces_visible"] = false,
                ["space_boundaries_visible"] = false,
                ["openings_visible"] = true
            }
        }
    }
};

var jsonContent = JsonSerializer.Serialize(payload);
var content = new StringContent(jsonContent, System.Text.Encoding.UTF8, "application/json");
var response = await client.PostAsync(url, content).ConfigureAwait(false);
```

---

## Common Patterns

### Link Comment to Viewpoint

When creating a comment, reference a viewpoint:
```csharp
var commentPayload = new Dictionary<string, string>
{
    ["comment"] = "See the highlighted beam in the attached view",
    ["viewpoint_guid"] = viewpointGuid
};
```

### Download All Viewpoint Snapshots for a Topic

```csharp
// 1. List viewpoints
var vpUrl = $"{bcfBaseUrl}/projects/{projectId}/topics/{topicId}/viewpoints";
var vpResponse = await client.GetAsync(vpUrl).ConfigureAwait(false);
var vpJson = await vpResponse.Content.ReadAsStringAsync().ConfigureAwait(false);
using var vpDoc = JsonDocument.Parse(vpJson);

foreach (var vp in vpDoc.RootElement.EnumerateArray())
{
    var vpId = vp.GetProperty("guid").GetString();

    // 2. Download snapshot for each viewpoint
    var snapUrl = $"{bcfBaseUrl}/projects/{projectId}/topics/{topicId}/viewpoints/{vpId}/snapshot";
    var snapResponse = await client.GetAsync(snapUrl).ConfigureAwait(false);

    if (snapResponse.IsSuccessStatusCode)
    {
        var imageBytes = await snapResponse.Content.ReadAsByteArrayAsync().ConfigureAwait(false);
        var filePath = System.IO.Path.Combine(outputDir, $"snapshot_{vpId}.png");
        System.IO.File.WriteAllBytes(filePath, imageBytes);
    }
}
```

### Extract IFC GUIDs from Viewpoint

```csharp
var guids = new List<string>();

// From selection
var selUrl = $"{bcfBaseUrl}/projects/{projectId}/topics/{topicId}/viewpoints/{viewpointId}/selection";
var selResponse = await client.GetAsync(selUrl).ConfigureAwait(false);
var selJson = await selResponse.Content.ReadAsStringAsync().ConfigureAwait(false);
using var selDoc = JsonDocument.Parse(selJson);

if (selDoc.RootElement.TryGetProperty("selection", out var sel))
{
    foreach (var comp in sel.EnumerateArray())
    {
        if (comp.TryGetProperty("ifc_guid", out var g))
            guids.Add(g.GetString());
    }
}
```
