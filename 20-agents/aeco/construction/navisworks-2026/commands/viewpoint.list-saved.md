# viewpoint.list-saved

List every saved viewpoint in the federation (name, containing folder, GUID). Read-only.

## Inputs

| Field | Type | Required | Description |
|---|---|---|---|
| `document-id` | string | yes | The open federation. |

## Output

```yaml
viewpoints:
  - name:   "Level 3 — coordination"
    folder: "Reviews/2026-05"
    guid:   "a83f…"
  - name:   "Atrium overview"
    folder: ""
    guid:   "11b2…"
```

## Implementation (Navisworks .NET API)

`Application.ActiveDocument.SavedViewpoints` is a `DocumentSavedViewpoints` part holding a **tree** (not a flat list). Each node is a `SavedItem`: a `SavedViewpoint` (a leaf, derives from `GroupItem`), a `FolderItem` (a folder that nests sub-folders), or a `SavedViewpointAnimation`. Recurse the tree, and for each `SavedViewpoint` read `DisplayName` and `Guid`; build `folder` from the ancestor `FolderItem` names.

```csharp
void Walk(SavedItem item, string folder) {
    if (item is GroupItem g)            // folder or animation container
        foreach (var child in g.Children) Walk(child, Join(folder, item.DisplayName));
    else
        emit(item.DisplayName, folder, item.Guid);   // a SavedViewpoint leaf
}
Walk(Application.ActiveDocument.SavedViewpoints.RootItem, "");
```

## Gotchas

- **It's a folder tree, not a flat list** — `FolderItem`s nest; recurse rather than assuming one level. (The exact traversal members — `RootItem` / `Value` / `Children` — are partly unverified against the SDK reference; the *concept* of a `SavedItem` tree with `FolderItem`s and `SavedViewpoint` leaves is confirmed.)
- **`SavedViewpoint` derives from `GroupItem`** — a viewpoint can itself act as a container in some structures; test the node kind before treating it as a leaf.
- **`CurrentSavedViewpoint` goes null** after any manual camera change ("anonymous" view) — don't rely on it to identify "the current saved viewpoint" mid-run.
- **`GetCamera()` can throw `AccessViolationException`** in certain event contexts (community reports decorating with `[HandleProcessCorruptedStateExceptions]`) — listing name/folder/guid avoids touching the camera.

Sources: [Updating saved viewpoints — `SavedViewpoints`/`CurrentSavedViewpoint` (Autodesk dev blog)](https://blog.autodesk.io/updating-saved-viewpoints-through-navisworks-net-api/) · [Navigate to saved viewpoint — `SavedViewpoint : GroupItem`, `FolderItem`, `CopyFrom` (Autodesk forum)](https://forums.autodesk.com/t5/navisworks-api-forum/navigate-to-saved-viewpoint-in-model/td-p/9376077) · [Viewpoint part 1 — `SavedViewpoints` tree (twentytwo)](https://twentytwo.space/2020/12/06/navisworks-api-viewpoint-part-1/)

## See also

- [viewpoint.capture-current](./viewpoint.capture-current.md) · [viewpoint.from-clash](./viewpoint.from-clash.md)
- [viewpoints-and-markup](../skills/viewpoints-and-markup.md)
