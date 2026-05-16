---
name: rhino-common-rhino-input
description: API reference for namespace Rhino.Input from RhinoCommon.dll
---

# Rhino.Input

- **GetBoxMode**
  - Enumerates all Box getter modes.
- **GetResult**
  - Possible results from GetObject.Get(), GetPoint.Get(), etc...
- **RhinoGet**
  - Base class for GetObject, GetPoint, GetSphere, etc.                          You will never directly create a RhinoGet but you will use its member             functions after calling GetObject::GetObjects(), GetPoint::GetPoint(), and so on.                          Provides tools to set command prompt, set command options, and specify             if the "get" can optionally accept numbers, nothing (pressing enter),             and undo.
- **StringParser**
  - Parse strings to numbers, distances and angles
- **StringParserSettings**
  - Parameters for parsing strings
