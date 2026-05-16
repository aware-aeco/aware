# get-base-class-accept-enter-when-done

Lifecycle: single

There are instances of RhinoGet that prompt with "Press Enter when Done." yet do not call AcceptNothing().             On the Mac, these instances need an additional call to AcceptEnterWhenDone() so the GetPointOptions dialog             can correctly enable the Done button.
