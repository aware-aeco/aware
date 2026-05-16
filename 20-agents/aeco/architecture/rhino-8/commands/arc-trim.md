# arc-trim

Lifecycle: single

Sets the arc's angle domain (in radians) to the specified interval, provided it is increasing and its length does not exceed 2.0 * Math.PI (plus a small tolerance).             If the interval length exceeds 2.0 * Math.PI it will be clamped to exactly 2.0 * Math.PI. Returns true if the domain was set successfully, false otherwise.
