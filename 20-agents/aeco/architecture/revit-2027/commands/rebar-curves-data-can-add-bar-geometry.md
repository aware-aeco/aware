# rebar-curves-data-can-add-bar-geometry

Lifecycle: single

If the layout rule is Single or FixedNumber or NumberWithSpacing this function will return true if getNumberOfBarGeometry() is less getBarsNumber(), false otherwise.If the layout rule is CustomSpacing, it will return true if GetNumberOfBarGeometry() is less the number of bars (specified by the formula), false otherwise.For the other layout types, will return true.
