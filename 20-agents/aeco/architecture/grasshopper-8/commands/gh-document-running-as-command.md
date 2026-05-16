# gh-document-running-as-command

Lifecycle: single

When the document is being solved inside the context of an executing  Rhino command, this function will return the command instance. Under  normal operation of Grasshopper in the traditional sense, this function  will return null. The document executes as a command when inside of the  grasshopper player or as a compiled grasshopper player based command.  The way to run a GH_Document as a command is to call the  GH_RhinoScriptInterface.RunAsCommand function
