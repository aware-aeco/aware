# gh-component-server-external-files

Lifecycle: single

Get lists of all the external files (GHA, GHUSER and GHCLUSTER) that will be loaded by Grasshopper.   Note: this does not equal the files that may already have been loaded, just the files that we'll attempt   to load if we were asked to load all plugins now. The list contains no duplicate file paths, but it may contain   identical files which won't be filtered until actual loading.
