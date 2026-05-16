# render-content-manager-restore-render-content

Lifecycle: single

Unpacks the default render contents from the from the application and places them in the User's folder.             This will restore the default versions if the version of Rhino that is running is newer than the contents             of the last Rhino that wrote the version.txt file.  If the version.txt file is not present, it will             automatically restore the default contents.  This does not overwrite files that the user has changed.
