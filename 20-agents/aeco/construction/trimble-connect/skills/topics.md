---
name: trimble-connect-topics
description: Trimble Connect BCF Topics API — create, query, filter, update, delete topics, issues, comments, viewpoints, document references, BCF 2.1, clash detection results, design review, issue tracking, BIM coordination. This skill should be used when working with Trimble Connect issues, topics, comments, or BIM coordination workflows.
---

# Trimble Connect — BCF Topics API Reference (BCF 2.1)

All BCF Topic Service endpoints use the regional base URL (see trimble-connect-auth skill).
Example base: `https://open11.connect.trimble.com/bcf/2.1`

---

## Topics

### List Topics

```
GET /bcf/2.1/projects/{projectId}/topics
```

**Response:** Array of Topic objects

```csharp
var url = $"{bcfBaseUrl}/projects/{projectId}/topics";
var response = await client.GetAsync(url).ConfigureAwait(false);
var json = await response.Content.ReadAsStringAsync().ConfigureAwait(false);
using var doc = JsonDocument.Parse(json);

foreach (var topic in doc.RootElement.EnumerateArray())
{
    var guid = topic.GetProperty("guid").GetString();
    var title = topic.GetProperty("title").GetString();
    var status = topic.TryGetProperty("topic_status", out var s) ? s.GetString() : null;
    var type = topic.TryGetProperty("topic_type", out var t) ? t.GetString() : null;
    var priority = topic.TryGetProperty("priority", out var p) ? p.GetString() : null;
    var assignedTo = topic.TryGetProperty("assigned_to", out var a) ? a.GetString() : null;
}
```

### Get Single Topic

```
GET /bcf/2.1/projects/{projectId}/topics/{topicId}
```

**Response:** Single Topic object

### Create Topic

```
POST /bcf/2.1/projects/{projectId}/topics
Content-Type: application/json
```

**Request Body:**
```json
{
    "title": "Clash detected at grid B-3",
    "description": "Steel beam intersects with duct",
    "topic_type": "Clash",
    "topic_status": "Open",
    "priority": "High",
    "assigned_to": "engineer@company.com",
    "due_date": "2025-06-15T00:00:00Z",
    "stage": "Design",
    "labels": ["structural", "mep"],
    "reference_links": ["https://example.com/clash-report"]
}
```

**Code Example:**
```csharp
var url = $"{bcfBaseUrl}/projects/{projectId}/topics";
var payload = new Dictionary<string, object>
{
    ["title"] = title,
    ["topic_type"] = "Issue",
    ["topic_status"] = "Open",
    ["priority"] = "Normal"
};

// Add optional fields only if provided
if (string.IsNullOrEmpty(description) is false)
    payload["description"] = description;
if (string.IsNullOrEmpty(assignedTo) is false)
    payload["assigned_to"] = assignedTo;
if (dueDate.HasValue)
    payload["due_date"] = dueDate.Value.ToString("o");
if (labels?.Length > 0)
    payload["labels"] = labels;

var jsonContent = JsonSerializer.Serialize(payload);
var content = new StringContent(jsonContent, System.Text.Encoding.UTF8, "application/json");
var response = await client.PostAsync(url, content).ConfigureAwait(false);
```

**Response:** Created Topic object with `guid` assigned by server

### Update Topic

```
PUT /bcf/2.1/projects/{projectId}/topics/{topicId}
Content-Type: application/json
```

**Request Body:** Full or partial Topic object. Only include fields you want to change.

```csharp
var url = $"{bcfBaseUrl}/projects/{projectId}/topics/{topicId}";
var payload = new Dictionary<string, object>
{
    ["topic_status"] = "Closed",
    ["priority"] = "Low"
};

var jsonContent = JsonSerializer.Serialize(payload);
var content = new StringContent(jsonContent, System.Text.Encoding.UTF8, "application/json");

var request = new HttpRequestMessage(HttpMethod.Put, url) { Content = content };
var response = await client.SendAsync(request).ConfigureAwait(false);
```

### Delete Topic

```
DELETE /bcf/2.1/projects/{projectId}/topics/{topicId}
```

```csharp
var url = $"{bcfBaseUrl}/projects/{projectId}/topics/{topicId}";
var response = await client.DeleteAsync(url).ConfigureAwait(false);
```

---

## Topic Object Schema

| Property | Type | Description |
|----------|------|-------------|
| `guid` | string | Server-assigned unique identifier (read-only) |
| `server_assigned_id` | string | Alternative server-assigned ID (read-only) |
| `title` | string | Topic title/subject (required for create) |
| `description` | string | Detailed description text |
| `topic_type` | string | Category: `RFI`, `Issue`, `Clash`, `Comment`, `Request`, `Solution` |
| `topic_status` | string | Status: `Open`, `In Progress`, `Closed`, `Resolved` |
| `priority` | string | Priority: `Critical`, `High`, `Normal`, `Low` |
| `assigned_to` | string | Email address of assignee |
| `due_date` | string (ISO 8601) | Deadline date |
| `stage` | string | Project phase: `Design`, `Construction`, `Fabrication`, `Installation` |
| `labels` | string[] | Tags/categories for the topic |
| `reference_links` | string[] | Related URLs |
| `creation_date` | string (ISO 8601) | When created (read-only) |
| `creation_author` | string | Email of creator (read-only) |
| `modified_date` | string (ISO 8601) | When last modified (read-only) |
| `modified_author` | string | Email of last modifier (read-only) |

### Valid Topic Types

Values from project extensions. Common defaults:
- `RFI` — Request for Information
- `Issue` — General issue
- `Clash` — Model clash detection
- `Comment` — General comment
- `Request` — Change request
- `Solution` — Proposed solution

### Valid Topic Statuses

- `Open` — New/unresolved
- `In Progress` — Being worked on
- `Closed` — Resolved and closed
- `Resolved` — Fixed, pending verification

### Valid Priorities

- `Critical` — Immediate attention required
- `High` — Important, address soon
- `Normal` — Standard priority
- `Low` — Address when convenient

### Valid Stages

- `Design` — Design phase
- `Construction` — Construction phase
- `Fabrication` — Fabrication phase
- `Installation` — Installation phase

---

## Comments

### List Comments

```
GET /bcf/2.1/projects/{projectId}/topics/{topicId}/comments
```

**Response:** Array of Comment objects

```csharp
var url = $"{bcfBaseUrl}/projects/{projectId}/topics/{topicId}/comments";
var response = await client.GetAsync(url).ConfigureAwait(false);
var json = await response.Content.ReadAsStringAsync().ConfigureAwait(false);
using var doc = JsonDocument.Parse(json);

foreach (var comment in doc.RootElement.EnumerateArray())
{
    var id = comment.GetProperty("guid").GetString();
    var text = comment.GetProperty("comment").GetString();
    var author = comment.TryGetProperty("modified_author", out var a) ? a.GetString() : null;
    var date = comment.TryGetProperty("modified_date", out var d) ? d.GetString() : null;
}
```

### Create Comment

```
POST /bcf/2.1/projects/{projectId}/topics/{topicId}/comments
Content-Type: application/json
```

**Request Body:**
```json
{
    "comment": "The clash has been resolved by moving the duct 200mm north."
}
```

```csharp
var url = $"{bcfBaseUrl}/projects/{projectId}/topics/{topicId}/comments";
var payload = new Dictionary<string, string> { ["comment"] = commentText };
var content = new StringContent(
    JsonSerializer.Serialize(payload),
    System.Text.Encoding.UTF8,
    "application/json");
var response = await client.PostAsync(url, content).ConfigureAwait(false);
```

### Delete Comment

```
DELETE /bcf/2.1/projects/{projectId}/topics/{topicId}/comments/{commentId}
```

### Update Comment

```
PUT /bcf/2.1/projects/{projectId}/topics/{topicId}/comments/{commentId}
Content-Type: application/json
```

**Request Body:**
```json
{
    "comment": "Updated comment text with corrected information."
}
```

```csharp
var url = $"{bcfBaseUrl}/projects/{projectId}/topics/{topicId}/comments/{commentId}";
var payload = new Dictionary<string, string> { ["comment"] = updatedText };
var jsonContent = JsonSerializer.Serialize(payload);
var content = new StringContent(jsonContent, System.Text.Encoding.UTF8, "application/json");

var request = new HttpRequestMessage(HttpMethod.Put, url) { Content = content };
var response = await client.SendAsync(request).ConfigureAwait(false);
```

### Comment Object Schema

| Property | Type | Description |
|----------|------|-------------|
| `guid` | string | Unique comment ID (read-only) |
| `comment` | string | Comment text content |
| `modified_author` | string | Email of author/last editor (read-only) |
| `modified_date` | string (ISO 8601) | Timestamp (read-only) |
| `viewpoint_guid` | string | Optional linked viewpoint ID |
| `reply_to_comment_guid` | string | GUID of parent comment (for threaded replies) |

---

## Viewpoints

### List Viewpoints

```
GET /bcf/2.1/projects/{projectId}/topics/{topicId}/viewpoints
```

**Response:** Array of Viewpoint objects

### Create Viewpoint

```
POST /bcf/2.1/projects/{projectId}/topics/{topicId}/viewpoints
Content-Type: application/json
```

**Request Body:**
```json
{
    "index": 1
}
```

### Get Viewpoint Snapshot

```
GET /bcf/2.1/projects/{projectId}/topics/{topicId}/viewpoints/{viewpointId}/snapshot
```

### Viewpoint Object Schema

| Property | Type | Description |
|----------|------|-------------|
| `guid` | string | Unique viewpoint ID (read-only) |
| `index` | int | Viewpoint order/index |
| `orthogonal_camera` | object | Orthogonal camera settings |
| `perspective_camera` | object | Perspective camera settings |
| `lines` | object | Markup line elements |
| `clipping_planes` | object[] | Section planes |
| `snapshot` | object | Associated image snapshot |

---

## Document References

### List Document References

```
GET /bcf/2.1/projects/{projectId}/topics/{topicId}/document_references
```

### Create Document Reference

```
POST /bcf/2.1/projects/{projectId}/topics/{topicId}/document_references
Content-Type: application/json
```

**Request Body:**
```json
{
    "url": "https://example.com/report.pdf",
    "description": "Clash detection report"
}
```

Or link to a Trimble Connect document:
```json
{
    "document_guid": "abc-123-def",
    "description": "Referenced IFC model"
}
```

### Update Document Reference

```
PUT /bcf/2.1/projects/{projectId}/topics/{topicId}/document_references/{referenceId}
Content-Type: application/json
```

```csharp
var url = $"{bcfBaseUrl}/projects/{projectId}/topics/{topicId}/document_references/{referenceId}";
var payload = new Dictionary<string, string>
{
    ["description"] = "Updated description",
    ["url"] = "https://example.com/updated-report.pdf"
};
var jsonContent = JsonSerializer.Serialize(payload);
var content = new StringContent(jsonContent, System.Text.Encoding.UTF8, "application/json");

var request = new HttpRequestMessage(HttpMethod.Put, url) { Content = content };
var response = await client.SendAsync(request).ConfigureAwait(false);
```

### Document Reference Object Schema

| Property | Type | Description |
|----------|------|-------------|
| `guid` | string | Unique reference ID (read-only) |
| `url` | string | External document URL |
| `document_guid` | string | Trimble Connect document GUID |
| `description` | string | Description of the reference |

---

## Related Topics

### List Related Topics

```
GET /bcf/2.1/projects/{projectId}/topics/{topicId}/related_topics
```

### Add Related Topic

```
POST /bcf/2.1/projects/{projectId}/topics/{topicId}/related_topics
Content-Type: application/json
```

**Request Body:**
```json
{
    "related_topic_guid": "guid-of-related-topic"
}
```

**Note:** Related topics support may vary by SDK version. The AWARE SDK wrapper currently has limited support for this endpoint.

---

## Project Extensions

### Get Project Extensions

```
GET /bcf/2.1/projects/{projectId}/extensions
```

Returns the valid values for topic fields in this project:

```csharp
var url = $"{bcfBaseUrl}/projects/{projectId}/extensions";
var response = await client.GetAsync(url).ConfigureAwait(false);
var json = await response.Content.ReadAsStringAsync().ConfigureAwait(false);
using var doc = JsonDocument.Parse(json);

// Read available topic types
if (doc.RootElement.TryGetProperty("topic_type", out var types))
{
    foreach (var t in types.EnumerateArray())
    {
        var typeName = t.GetString(); // "RFI", "Issue", "Clash", etc.
    }
}

// Read available statuses
if (doc.RootElement.TryGetProperty("topic_status", out var statuses))
{
    foreach (var s in statuses.EnumerateArray())
    {
        var statusName = s.GetString(); // "Open", "In Progress", etc.
    }
}

// Also available: "priority", "topic_label", "stage"
```

### Extensions Object Schema

| Property | Type | Description |
|----------|------|-------------|
| `topic_type` | string[] | Valid topic types for this project |
| `topic_status` | string[] | Valid status values |
| `priority` | string[] | Valid priority values |
| `topic_label` | string[] | Available labels/tags |
| `stage` | string[] | Valid stage values |

---

## Filtering Topics Client-Side

The BCF API does not natively support query parameters for filtering. Apply filters after fetching:

```csharp
// Fetch all topics
var url = $"{bcfBaseUrl}/projects/{projectId}/topics";
var response = await client.GetAsync(url).ConfigureAwait(false);
var json = await response.Content.ReadAsStringAsync().ConfigureAwait(false);
using var doc = JsonDocument.Parse(json);

var results = new List<Dictionary<string, object>>();

foreach (var topic in doc.RootElement.EnumerateArray())
{
    // Filter by status
    if (filterStatus is not null)
    {
        var status = topic.TryGetProperty("topic_status", out var s) ? s.GetString() : null;
        if (string.Equals(status, filterStatus, StringComparison.OrdinalIgnoreCase) is false)
            continue;
    }

    // Filter by type
    if (filterType is not null)
    {
        var type = topic.TryGetProperty("topic_type", out var t) ? t.GetString() : null;
        if (string.Equals(type, filterType, StringComparison.OrdinalIgnoreCase) is false)
            continue;
    }

    // Filter by priority
    if (filterPriority is not null)
    {
        var priority = topic.TryGetProperty("priority", out var p) ? p.GetString() : null;
        if (string.Equals(priority, filterPriority, StringComparison.OrdinalIgnoreCase) is false)
            continue;
    }

    // Filter by assignee
    if (filterAssignedTo is not null)
    {
        var assigned = topic.TryGetProperty("assigned_to", out var a) ? a.GetString() : null;
        if (string.Equals(assigned, filterAssignedTo, StringComparison.OrdinalIgnoreCase) is false)
            continue;
    }

    results.Add(/* build result dict */);
}
```

## BCF Events (Change Tracking)

### Topic Events

```
GET /bcf/2.1/projects/{projectId}/topics_events
```

Returns a list of topic change events for the project. Useful for syncing or detecting recent changes.

```csharp
var url = $"{bcfBaseUrl}/projects/{projectId}/topics_events";
var response = await client.GetAsync(url).ConfigureAwait(false);
var json = await response.Content.ReadAsStringAsync().ConfigureAwait(false);
using var doc = JsonDocument.Parse(json);

foreach (var evt in doc.RootElement.EnumerateArray())
{
    var topicGuid = evt.GetProperty("topic_guid").GetString();
    var date = evt.GetProperty("date").GetString();
    var author = evt.GetProperty("author").GetString();

    // Actions: "add", "update", "delete"
    if (evt.TryGetProperty("actions", out var actions))
    {
        foreach (var action in actions.EnumerateArray())
        {
            var actionType = action.GetString();
        }
    }
}
```

### Comment Events

```
GET /bcf/2.1/projects/{projectId}/comments_events
```

Returns a list of comment change events. Same structure as topic events:

```csharp
var url = $"{bcfBaseUrl}/projects/{projectId}/comments_events";
var response = await client.GetAsync(url).ConfigureAwait(false);
var json = await response.Content.ReadAsStringAsync().ConfigureAwait(false);
using var doc = JsonDocument.Parse(json);

foreach (var evt in doc.RootElement.EnumerateArray())
{
    var commentGuid = evt.GetProperty("comment_guid").GetString();
    var topicGuid = evt.GetProperty("topic_guid").GetString();
    var date = evt.GetProperty("date").GetString();
    var author = evt.GetProperty("author").GetString();
}
```

### Event Object Schema

| Property | Type | Description |
|----------|------|-------------|
| `topic_guid` | string | Related topic GUID |
| `comment_guid` | string | Comment GUID (comment events only) |
| `date` | string (ISO 8601) | Event timestamp |
| `author` | string | Email of the user who made the change |
| `actions` | string[] | Change types: `"add"`, `"update"`, `"delete"` |

---

## Topic Model — SDK C# Property Mapping

When using the Trimble Connect .NET SDK (`Trimble.Connect.Topic.Client`), the `Topic` class maps to BCF JSON properties:

| C# Property | Type | JSON Property | Description |
|-------------|------|---------------|-------------|
| `Id` | `string` | `guid` | Server-assigned GUID |
| `ServerAssignedId` | `string` | `server_assigned_id` | Alternative server ID |
| `Title` | `string` | `title` | Topic title (required) |
| `Description` | `string` | `description` | Detailed description |
| `TopicType` | `string` | `topic_type` | Category (from extensions) |
| `TopicStatus` | `string` | `topic_status` | Status (from extensions) |
| `Priority` | `string` | `priority` | Priority level |
| `AssignedTo` | `string` | `assigned_to` | Assignee email |
| `DueDate` | `DateTime?` | `due_date` | Deadline |
| `Stage` | `string` | `stage` | Project phase |
| `Labels` | `IList<string>` | `labels` | Tags/categories |
| `ReferenceLinks` | `IList<string>` | `reference_links` | Related URLs |
| `CreationDate` | `DateTime` | `creation_date` | Created timestamp (read-only) |
| `CreationAuthor` | `string` | `creation_author` | Creator email (read-only) |
| `ModifiedDate` | `DateTime` | `modified_date` | Modified timestamp (read-only) |
| `ModifiedAuthor` | `string` | `modified_author` | Modifier email (read-only) |
| `BimSnippet` | `BimSnippet` | `bim_snippet` | Attached BIM snippet metadata |

### Comment Model — SDK C# Property Mapping

| C# Property | Type | JSON Property | Description |
|-------------|------|---------------|-------------|
| `Id` | `string` | `guid` | Server-assigned GUID |
| `Comment` | `string` | `comment` | Comment text |
| `ViewpointId` | `string` | `viewpoint_guid` | Linked viewpoint |
| `ReplyToCommentId` | `string` | `reply_to_comment_guid` | Parent comment for threading |
| `ModifiedDate` | `DateTime` | `modified_date` | Timestamp (read-only) |
| `ModifiedAuthor` | `string` | `modified_author` | Author email (read-only) |

---

## Web Links

Generate direct links to topics in the Trimble Connect web portal:
```csharp
var topicUrl = $"{webBaseUrl}/projects/{projectId}/topics/{topicId}";
// Example: https://web.connect.trimble.com/projects/proj-123/topics/topic-456
```
