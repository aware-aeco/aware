# realtime-display-mode-post-construct

Lifecycle: single

Override PostConstruct if you need to initialize where             the underlying RealtimeDisplayMode is available.                          The connection is made right after RealtimeDisplayMode             has been instantiated, but just before PostConstruct is called.                          For instance finding out OpenGL information can be done in             PostConstruct.
