---
name: aps-data-hierarchy
description: This skill should be used when navigating Autodesk cloud storage with the aps-data-management agent — the hub → project → topFolders → folder → item → version model, the tip-version concept, why get-hub-projects only returns the user's projects, and following JSON:API relationships. Encodes the data model behind the navigation endpoints.
---

# APS Data Management hierarchy

The model is a strict tree. Knowing it turns the 41 endpoints into a small set of moves.

```
hub  (b.<account>)                     get-hubs / get-hub
 └─ project  (b.<project>)             get-hub-projects / get-project
     └─ topFolders                     get-project-top-folders   ← entry point into the file tree
         └─ folder                     get-folder / get-folder-contents
             ├─ folder (nested)        (recurse via get-folder-contents)
             └─ item   (a file)        get-item
                 └─ version            get-item-versions / get-version
                     └─ tip            get-item-tip   ← the current version
```

## The moves

- **Enter the tree:** a project's files don't hang off the project directly — call `get-project-top-folders` to get the root folders (e.g. *Project Files*, *Plans*), then recurse with `get-folder-contents`.
- **`item` vs `version`:** an **item** is the file's stable identity (it keeps one id across revisions); a **version** is a specific revision. `get-item-tip` returns the current version — use it for "the latest drawing," not the item itself. `get-item-versions` is the full history.
- **Search within a subtree:** `get-folder-search` lists a folder *and its subfolders'* contents (recursive) — cheaper than hand-recursing `get-folder-contents` when you just need "every IFC under this folder."
- **Relationships / refs:** `*/refs` and `*/relationships/*` expose custom links between resources (e.g. an item referencing another). These are how ACC models design-to-issue and xref links.

## `get-hub-projects` returns only YOUR projects

`GET /project/v1/hubs/{hub}/projects` (`get-hub-projects`) returns only the projects the **token's user is a member of** — *not* the whole account portfolio. For the full list (including projects the user isn't on), use the [acc-account-admin](../../acc-account-admin/) agent's `get-projects` (which also wants the **bare** project GUID, while this API uses `b.<guid>` — see [aps-data-management-auth-and-ids](./aps-data-management-auth-and-ids.md)).

## Everything is JSON:API

Names, types, and child links come from the JSON:API envelope (`data.attributes.displayName`, `relationships`, `links.next`), not flat fields. See [aps-data-management-auth-and-ids](./aps-data-management-auth-and-ids.md).

## See also

- [aps-data-management-auth-and-ids](./aps-data-management-auth-and-ids.md) — the `b.` prefix + JSON:API shape
- [aps-upload-and-download-flow](./aps-upload-and-download-flow.md) — creating items/versions + the upload/download mechanics
