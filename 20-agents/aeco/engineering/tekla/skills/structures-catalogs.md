---
name: tekla-structures-catalogs
description: Tekla Open API - Tekla.Structures.Catalogs namespace. Profile catalog, material catalog, bolt catalog, rebar catalog, mesh catalog, profile list, material grades, steel grades, concrete grades, bolt standards, rebar sizes, catalog search, available profiles, available materials. This skill should be used when querying, listing, or looking up Tekla catalog data, profiles, materials, or bolt standards.
---

# Tekla.Structures.Catalogs API Reference (v2025)

## Overview

Access Tekla Structures catalogs: profiles, materials, bolts, reinforcing bars, meshes, shapes, components, drawings, printers, and user properties. Query items, enumerate catalog contents, import/export entries.

The central entry point is `CatalogHandler`, which provides enumerators and import/export methods for every catalog area. Each catalog area follows a consistent pattern: an item class (e.g., `BoltItem`), an enumerator class (e.g., `BoltItemEnumerator`) with `Current`, `MoveNext()`, `Reset()`, `GetSize()`, and often a name helper class (e.g., `BoltName`).

> Detailed API reference (all catalog classes, enumerators, item types) is available in the supporting file `tekla-structures-catalogs-api-reference.md`.

