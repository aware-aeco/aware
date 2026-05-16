# gh-number-slider-harvest-range

Lifecycle: single

Parse an init code for validity. Valid init codes are A<B, A<B<C, A..B, A..B..C   Where A, B and C can all be evaluated to doubles or integers. This method does not validate whether the  actual values of A, B and C are non-decremental.   Note that sliders can also be initiated using init codes that evaluate to single numbers. This method   does not take those into account. This is purely for validating range inits.
