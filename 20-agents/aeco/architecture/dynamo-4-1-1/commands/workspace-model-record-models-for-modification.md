# workspace-model-record-models-for-modification

Lifecycle: single

TODO(Ben): This method is exposed this way for external codes (e.g.             the DragCanvas) to record models before they are modified. This is             by no means ideal. The ideal case of course is for ALL codes that             end up modifying models to be folded back into WorkspaceViewModel in             the form of commands. These commands then internally record those             affected models before updating them. We need this method to be gone             sooner than later.
