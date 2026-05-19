---
name: navisworks-autodesk-navisworks-api-bim360
description: This skill encodes the navisworks 2026.0 surface of the Autodesk.Navisworks.Api.Bim360 namespace — 3 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: AccountInfo, ResourceInfo, ResourceType.
---

# Autodesk.Navisworks.Api.Bim360

Auto-generated from vendor docs for navisworks 2026.0. 3 types in this namespace.

## AccountInfo (class)

Provides information about a BIM 360 Account.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.Bim360.AccountInfo)

### Constructors
- `AccountInfo()` — Constructs a new AccountInfo.

### Properties
- `AccountGuid` (System.Guid, get/set) — Guid identifier of Account.
- `AccountId` (string, get/set) — Identifier of Account.
- `AuthToken` (string, get/set) — AuthToken related to this Account.
- `Name` (string, get/set) — Name of this Account.
- `UserType` (string, get/set) — User type of this Account.

## ResourceInfo (class)

Provides information about a BIM 360 Resource.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.Bim360.ResourceInfo)

### Constructors
- `ResourceInfo()` — Constructs a new ResourceInfo.

### Properties
- `AccountId` (string, get/set) — Identifier of Account the resource belongs to.
- `DisplayName` (string, get/set) — Display name of the resource.
- `Id` (System.Guid, get/set) — Version independent identifier for the resource.
- `ParentId` (System.Guid?, get/set) — Optional identifier of Parent of the resource.
- `ProjectId` (System.Guid, get/set) — Identifier of Project the resource belongs to.
- `Type` (Autodesk.Navisworks.Api.Bim360.ResourceType, get/set) — Type of the resource.
- `VersionId` (System.Guid, get/set) — Version dependent identifier for the resource.

## ResourceType (enum)

Resource Type.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.Bim360.ResourceType)

### Values
- `Project` = `0` — Project.
- `Model` = `1` — Model.
- `MergedModel` = `2` — Merged Model.

