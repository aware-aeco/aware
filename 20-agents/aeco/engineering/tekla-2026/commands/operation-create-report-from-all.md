# operation-create-report-from-all

Lifecycle: single

Creates a report from all the objects using the given template name and filename.              A template with the name given in the TemplateName parameter must exist in model folder or in the folder defined                    with the advanced option XS_TEMPLATE_DIRECTORY.If a path is not given in the filename, the file is created to the folder defined with the advanced option XS_REPORT_OUTPUT_DIRECTORY.If the given folder does not exist, the report creation fails.Internally, this method is asynchronous, and because of that the output file cannot be immediately available.See Tekla Structures Help for more information about reports.
