# gh-component-param-server-emit-sync-object

Lifecycle: single

Create a synch object that stores a shallow representation of this server.   Cache this object if you're planning to make sweeping changes to this param-server.   Then, once you are finished call the Sync() function to perform cleanup of   removed parameters. This is a utility function and you can choose to perform   cleanup yourself.
