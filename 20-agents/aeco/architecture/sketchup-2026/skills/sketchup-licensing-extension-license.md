---
name: yard-sketchup-licensing-extension-license
description: Sketchup::Licensing::ExtensionLicense API reference (YARD)
---

# Sketchup::Licensing::ExtensionLicense API reference

The Sketchup::Licensing::ExtensionLicense class is used to store extension license information. An instance of this class is returned by Sketchup::Licensing.get_extension_license. Note that this is a temporary object representing the extension licensing state for the moment it was returned. It will not reflect any licensing state changes that may occur.

## Methods

- `days_remaining` — The days_remaining method is used to get the number of days remaining until license expiration.
- `error_description` — The error_description method is used to obtain error information in case of failure to acquire a license. This is meant to aid in debugging only. Extensions should not rely on any exact error description.
- `licensed?` — The licensed? method is used to decide whether the extension is licensed to run or not.
- `state` — The state method returns a constant indicating the specific licensing state. These should be used for informational purposes only and not to decide if the extension is licensed to run. For that, please use the licensed? method.
