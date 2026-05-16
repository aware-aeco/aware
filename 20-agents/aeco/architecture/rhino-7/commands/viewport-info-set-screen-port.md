# viewport-info-set-screen-port

Lifecycle: single

Location of viewport in pixels.             These are provided so you can set the port you are using             and get the appropriate transformations to and from             screen space.             // For a Windows window             /      int width = width of window client area in pixels;             /      int height = height of window client area in pixels;             /      port_left = 0;             /      port_right = width;             /      port_top = 0;             /      port_bottom = height;             /      port_near = 0;             /      port_far = 1;             /      SetScreenPort( port_left, port_right,              /                     port_bottom, port_top,              /                     port_near, port_far );
