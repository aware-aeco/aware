# get-base-class-accept-color

Lifecycle: single

If you want to allow the user to be able to type in a color r,g,b or name             during GetPoint.Get(), GetObject::GetObjects(), etc., then call AcceptColor(true)             before calling GetPoint()/GetObject(). If the user chooses to type in a color,             then the result code GetResult.Color is returned and you can use RhinoGet.Color()             to get the value of the color.  If the get accepts points, then the user will not             be able to type in r,g,b colors but will be able to type color names.
