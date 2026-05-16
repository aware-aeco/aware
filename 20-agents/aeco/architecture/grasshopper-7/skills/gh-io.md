---
name: grasshopper-gh-io
description: API reference for namespace GH_IO from GH_IO.dll
---

# GH_IO

- **GH_ISerializable**
  - Every object which needs to (de)serialize itself using GH_IO methodology             must implement this interface.
- **VersionNumber**
  - Represents a product version number that encodes major, minor, time and build branch information.             as major.minor.yyddd.hhmmb where:             yy = year of build - 2000             ddd = year day of build (1-366)             hh = hour of build             mm = minute of build             b = branch of build
