# home-workspace-model-run

Lifecycle: single

This method is typically called from the main application thread (as              a result of user actions such as button click or node UI changes) to             schedule an update of the graph. This call may or may not represent              an actual update. In the event that the user action does not result              in actual graph update (e.g. moving of node on UI), the update task              will not be scheduled for execution.
