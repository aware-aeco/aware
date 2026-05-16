# get-point-enable-no-redraw-on-exit

Lifecycle: single

The default functionality of the GetPoint operation is to perform a redraw on exit.             Calling this function with true turns off automatic redraw at the end of GetPoint.             May be needed in some commands for flicker free feedback.             When set to true, the caller is responsible for cleaning up the screen             after GetPoint.
