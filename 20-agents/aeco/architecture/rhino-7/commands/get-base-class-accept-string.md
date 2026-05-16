# get-base-class-accept-string

Lifecycle: single

If you want to allow the user to be able to type in a string during GetPoint.Get(),             GetObject::GetObjects(), etc., then call AcceptString(true) before calling             GetPoint()/GetObject(). If the user chooses to type in a string, then the result code             GetResult.String is returned and you can use RhinoGet.String() to get the value of the string.
