# get-base-class-accept-number

Lifecycle: single

If you want to allow the user to be able to type in a number during GetPoint.Get(),             GetObject::GetObjects(), etc., then call AcceptNumber() beforehand.             If the user chooses to type in a number, then the result code GetResult.Number is             returned and you can use RhinoGet.Number() to get the value of the number. If you             are using GetPoint and you want "0" to return (0,0,0) instead of the number zero,              then set acceptZero = false.
