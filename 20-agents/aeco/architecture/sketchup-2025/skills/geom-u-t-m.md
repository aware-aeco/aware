---
name: yard-geom-u-t-m
description: Geom::UTM API reference (YARD)
---

# Geom::UTM API reference

Valid ranges for #zone_number and #zone_letter are 1-60 and C-X (omitting I and O). Valid ranges for #x and #y are 100000-899999.

## Methods

- `initialize` — The #initialize method is used to create a new UTM coordinate. You will often create UTM objects by calling the method Sketchup::Model#point_to_utm instead of calling this method.
- `to_a` — The #to_a method returns a UTM coordinate as a 4 element array. The Array elements are the zone number, the zone letter, the x coordinate and the y coordinate.
- `to_latlong` — The #to_latlong method is used to convert UTM coordinates to latitude and longitude. See the LatLong class for more information.
- `to_s` — The #to_s method is used to retrieve a string representation of a UTM.
- `x` — The #x method returns the UTM x coordinate.
- `y` — The #y method returns the UTM y coordinate.
- `zone_letter` — The #zone_letter method returns the UTM zone letter.
- `zone_number` — The #zone_number method returns the UTM zone number.
