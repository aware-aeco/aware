# host-utils-get-current-process-info

Lifecycle: single

Returns information about the current process. If Rhino is the top level process,              processName is "Rhino". Otherwise, processName is the name, without extension, of the main              module that is executing. For example, "compute.backend" or "Revit".                           processVersion is the System.Version of the running process. It is the FileVersion              of the executable.
