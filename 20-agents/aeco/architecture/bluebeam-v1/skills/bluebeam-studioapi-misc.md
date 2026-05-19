---
name: bluebeam-bluebeam-studioapi-misc
description: This skill encodes the bluebeam v1 surface of the Bluebeam.StudioApi.Misc namespace — 1 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: MiscApi.
---

# Bluebeam.StudioApi.Misc

Auto-generated from vendor docs for bluebeam v1. 1 types in this namespace.

## MiscApi (class)

Bluebeam Studio API — Misc endpoints. Auto-generated REST surface; one method per HTTP operation.

**Remarks:** REST root: /publicapi/v1/. Source: https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json

[Vendor docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

### Methods
#### `Task GetUserMe()`

Get info about currently authenticated user

**Remarks:** HTTP GET `/publicapi/v1/users/me`. Vendor docs page: https://support.bluebeam.com/developer/

[Docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

#### `Task ListHealthcheck()`

Healthcheck

**Remarks:** HTTP GET `/publicapi/v1/healthcheck`. Vendor docs page: https://support.bluebeam.com/developer/

[Docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

#### `Task ListVersion()`

Get Current Version

**Remarks:** HTTP GET `/publicapi/api/version`. Vendor docs page: https://support.bluebeam.com/developer/

[Docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

#### `Task UpdateUserMe(UpdateUserMeRequest request)`

Get info about currently authenticated user

**Remarks:** HTTP PUT `/publicapi/v1/users/me`. Vendor docs page: https://support.bluebeam.com/developer/

**Parameters:**
- `request` (UpdateUserMeRequest) — Request body. JSON object with fields: DisplayName.

[Docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

