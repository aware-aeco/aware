# decal-try-get-color

Lifecycle: single

Blend a color with the decal color at a given point. Please note that this function is very slow             so you should only use it if you're not overly concerned about performance.             Also please note that this function is actually a 'blend' operation, not a true 'get' operation             and is sensitive to the value passed into the colInOut parameter. The alpha value of colInOut             is used as the blend amount where 0 means 'all decal RGB' and 1 means 'all colInOut RGB'.             If you want to simply GET the decal color, you must pass 0 for colInOut alpha.
