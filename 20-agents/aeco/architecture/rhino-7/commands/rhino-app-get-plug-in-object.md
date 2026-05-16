# rhino-app-get-plug-in-object

Lifecycle: single

Gets the object that is returned by PlugIn.GetPlugInObject for a given             plug-in. This function attempts to find and load a plug-in with a given Id.             When a plug-in is found, it's GetPlugInObject function is called and the             result is returned here.             Note the plug-in must have already been installed in Rhino or the plug-in manager             will not know where to look for a plug-in with a matching id.
