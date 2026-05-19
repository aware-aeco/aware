---
name: sketchup-sketchup-licensing
description: This skill encodes the sketchup 2026.0 surface of the Sketchup::Licensing namespace — 1 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: ExtensionLicense.
---

# Sketchup::Licensing

Auto-generated from vendor docs for sketchup 2026.0. 1 types in this namespace.

## ExtensionLicense (class)

The Sketchup::Licensing::ExtensionLicense class is used to store extension license information. An instance of this class is returned by Sketchup::Licensing.get_extension_license. Note that this is a temporary object representing the extension licensing state for the moment it was returned. It will not reflect any licensing state changes that may occur.

[Vendor docs](https://ruby.sketchup.com/Sketchup/Licensing/ExtensionLicense.html)

### Methods
#### `days_remaining => Object`

The days_remaining method is used to get the number of days remaining until license expiration.

**Remarks:** The days_remaining method is used to get the number of days remaining until license expiration.

**Returns:** `Object` — Integer - Number of days until license expiration. Zero if a permanent license or not licensed.

[Docs](https://ruby.sketchup.com/Sketchup/Licensing/ExtensionLicense.html#days_remaining-instance_method)

#### `error_description => Object`

The error_description method is used to obtain error information in case of failure to acquire a license.

**Remarks:** The error_description method is used to obtain error information in case of failure to acquire a license. This is meant to aid in debugging only. Extensions should not rely on any exact error description.

**Returns:** `Object` — String - error description.

[Docs](https://ruby.sketchup.com/Sketchup/Licensing/ExtensionLicense.html#error_description-instance_method)

#### `licensed? => Boolean`

The licensed? method is used to decide whether the extension is licensed to run or not.

**Remarks:** The licensed? method is used to decide whether the extension is licensed to run or not.

**Returns:** `Boolean` — Boolean - true if the extension is allowed to run, false if it is not licensed and should quit.

[Docs](https://ruby.sketchup.com/Sketchup/Licensing/ExtensionLicense.html#licensed?-instance_method)

#### `state => Object`

The state method returns a constant indicating the specific licensing state.

**Remarks:** The state method returns a constant indicating the specific licensing state. These should be used for informational purposes only and not to decide if the extension is licensed to run. For that, please use the licensed? method.

**Returns:** `Object` — Integer - One of Sketchup::Licensing::LICENSED, Sketchup::Licensing::EXPIRED, Sketchup::Licensing::TRIAL, Sketchup::Licensing::TRIAL_EXPIRED, Sketchup::Licensing::NOT_LICENSED

[Docs](https://ruby.sketchup.com/Sketchup/Licensing/ExtensionLicense.html#state-instance_method)

