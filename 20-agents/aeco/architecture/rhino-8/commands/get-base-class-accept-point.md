# get-base-class-accept-point

Lifecycle: single

If you want to allow the user to be able to type in a point then call AcceptPoint(true)             before calling GetPoint()/GetObject(). If the user chooses to type in a number, then             the result code GetResult.Point is returned and you can use RhinoGet.Point()             to get the value of the point.
