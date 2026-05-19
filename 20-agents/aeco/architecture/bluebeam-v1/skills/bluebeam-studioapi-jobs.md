---
name: bluebeam-bluebeam-studioapi-jobs
description: This skill encodes the bluebeam v1 surface of the Bluebeam.StudioApi.Jobs namespace — 2 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: JobsApi, ProjectFileJobsApi.
---

# Bluebeam.StudioApi.Jobs

Auto-generated from vendor docs for bluebeam v1. 2 types in this namespace.

## JobsApi (class)

Bluebeam Studio API — Jobs endpoints. Auto-generated REST surface; one method per HTTP operation.

**Remarks:** REST root: /publicapi/v1/. Source: https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json

[Vendor docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

### Methods
#### `Task CancelJob(Guid id)`

Cancel a job

**Remarks:** HTTP PUT `/publicapi/v1/jobs/{id}/cancel`. Vendor docs page: https://support.bluebeam.com/developer/

**Parameters:**
- `id` (Guid) — Path parameter `id` from `/publicapi/v1/jobs/{id}/cancel`.

[Docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

#### `Task GetJob(Guid id)`

Get a single job metadata

**Remarks:** HTTP GET `/publicapi/v1/jobs/{id}`. Vendor docs page: https://support.bluebeam.com/developer/

**Parameters:**
- `id` (Guid) — Path parameter `id` from `/publicapi/v1/jobs/{id}`.

[Docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

#### `Task GetResult(Guid id, int resultId)`

Get a single job result metadata

**Remarks:** HTTP GET `/publicapi/v1/jobs/{id}/results/{resultId}`. Vendor docs page: https://support.bluebeam.com/developer/

**Parameters:**
- `id` (Guid) — Path parameter `id` from `/publicapi/v1/jobs/{id}/results/{resultId}`.
- `resultId` (int) — Path parameter `resultId` from `/publicapi/v1/jobs/{id}/results/{resultId}`.

[Docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

#### `Task ListJobResults(Guid id)`

Get a list of all job results metadata

**Remarks:** HTTP GET `/publicapi/v1/jobs/{id}/results`. Vendor docs page: https://support.bluebeam.com/developer/

**Parameters:**
- `id` (Guid) — Path parameter `id` from `/publicapi/v1/jobs/{id}/results`.

[Docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

#### `Task ListJobs()`

Get a list of jobs

**Remarks:** HTTP GET `/publicapi/v1/jobs`. Vendor docs page: https://support.bluebeam.com/developer/

[Docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

## ProjectFileJobsApi (class)

Bluebeam Studio API — Jobs / Project File Jobs endpoints. Auto-generated REST surface; one method per HTTP operation.

**Remarks:** REST root: /publicapi/v1/. Source: https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json

[Vendor docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

### Methods
#### `Task CreateJobBalancepages(Guid projectId, int fileId, CreateJobBalancepagesRequest request)`

Inserts blank pages into the Pdf file to balance the total number of pages to an odd, even, specific count, or specific page division

**Remarks:** HTTP POST `/publicapi/v1/projects/{projectId}/files/{fileId}/jobs/balancepages`. Vendor docs page: https://support.bluebeam.com/developer/

**Parameters:**
- `projectId` (Guid) — Path parameter `projectId` from `/publicapi/v1/projects/{projectId}/files/{fileId}/jobs/balancepages`.
- `fileId` (int) — Path parameter `fileId` from `/publicapi/v1/projects/{projectId}/files/{fileId}/jobs/balancepages`.
- `request` (CreateJobBalancepagesRequest) — Request body. JSON object with fields: CurrentPassword, OutputFileName, OutputPath, Priority.

[Docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

#### `Task CreateJobCombine(Guid projectId, int fileId, CreateJobCombineRequest request)`

Combines the Pdf file with a list of other Pdf files

**Remarks:** HTTP POST `/publicapi/v1/projects/{projectId}/files/{fileId}/jobs/combine`. Vendor docs page: https://support.bluebeam.com/developer/

**Parameters:**
- `projectId` (Guid) — Path parameter `projectId` from `/publicapi/v1/projects/{projectId}/files/{fileId}/jobs/combine`.
- `fileId` (int) — Path parameter `fileId` from `/publicapi/v1/projects/{projectId}/files/{fileId}/jobs/combine`.
- `request` (CreateJobCombineRequest) — Request body. JSON object with fields: OutputFileName, OutputPath, Priority.

[Docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

#### `Task CreateJobCreatepdfareport(Guid projectId, int fileId, CreateJobCreatepdfareportRequest request)`

Generate a text report on whether the PDF file is PDF/A-1b compliant. Results will always be appended to the output report file

**Remarks:** HTTP POST `/publicapi/v1/projects/{projectId}/files/{fileId}/jobs/createpdfareport`. Vendor docs page: https://support.bluebeam.com/developer/

**Parameters:**
- `projectId` (Guid) — Path parameter `projectId` from `/publicapi/v1/projects/{projectId}/files/{fileId}/jobs/createpdfareport`.
- `fileId` (int) — Path parameter `fileId` from `/publicapi/v1/projects/{projectId}/files/{fileId}/jobs/createpdfareport`.
- `request` (CreateJobCreatepdfareportRequest) — Request body. JSON object with fields: CurrentPassword, OutputFileName, OutputPath, Priority.

[Docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

#### `Task CreateJobDeletepages(Guid projectId, int fileId, CreateJobDeletepagesRequest request)`

Deletes pages from a Pdf file

**Remarks:** HTTP POST `/publicapi/v1/projects/{projectId}/files/{fileId}/jobs/deletepages`. Vendor docs page: https://support.bluebeam.com/developer/

**Parameters:**
- `projectId` (Guid) — Path parameter `projectId` from `/publicapi/v1/projects/{projectId}/files/{fileId}/jobs/deletepages`.
- `fileId` (int) — Path parameter `fileId` from `/publicapi/v1/projects/{projectId}/files/{fileId}/jobs/deletepages`.
- `request` (CreateJobDeletepagesRequest) — Request body. JSON object with fields: CurrentPassword, OutputFileName, OutputPath, Priority.

[Docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

#### `Task CreateJobDwg2pdf(Guid projectId, int fileId, CreateJobDwg2pdfRequest request)`

Converts a DWG file to Pdf

**Remarks:** HTTP POST `/publicapi/v1/projects/{projectId}/files/{fileId}/jobs/dwg2pdf`. Vendor docs page: https://support.bluebeam.com/developer/

**Parameters:**
- `projectId` (Guid) — Path parameter `projectId` from `/publicapi/v1/projects/{projectId}/files/{fileId}/jobs/dwg2pdf`.
- `fileId` (int) — Path parameter `fileId` from `/publicapi/v1/projects/{projectId}/files/{fileId}/jobs/dwg2pdf`.
- `request` (CreateJobDwg2pdfRequest) — Request body. JSON object with fields: OutputFileName, OutputPath, Priority.

[Docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

#### `Task CreateJobExcel2pdf(Guid projectId, int fileId, CreateJobExcel2pdfRequest request)`

Converts an Excel file to Pdf

**Remarks:** HTTP POST `/publicapi/v1/projects/{projectId}/files/{fileId}/jobs/excel2pdf`. Vendor docs page: https://support.bluebeam.com/developer/

**Parameters:**
- `projectId` (Guid) — Path parameter `projectId` from `/publicapi/v1/projects/{projectId}/files/{fileId}/jobs/excel2pdf`.
- `fileId` (int) — Path parameter `fileId` from `/publicapi/v1/projects/{projectId}/files/{fileId}/jobs/excel2pdf`.
- `request` (CreateJobExcel2pdfRequest) — Request body. JSON object with fields: OutputFileName, OutputPath, Priority.

[Docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

#### `Task CreateJobExportcustomcolumns(Guid projectId, int fileId, CreateJobExportcustomcolumnsRequest request)`

Exports the Custom Column definition of the Pdf file to an .xml file

**Remarks:** HTTP POST `/publicapi/v1/projects/{projectId}/files/{fileId}/jobs/exportcustomcolumns`. Vendor docs page: https://support.bluebeam.com/developer/

**Parameters:**
- `projectId` (Guid) — Path parameter `projectId` from `/publicapi/v1/projects/{projectId}/files/{fileId}/jobs/exportcustomcolumns`.
- `fileId` (int) — Path parameter `fileId` from `/publicapi/v1/projects/{projectId}/files/{fileId}/jobs/exportcustomcolumns`.
- `request` (CreateJobExportcustomcolumnsRequest) — Request body. JSON object with fields: CurrentPassword, OutputFileName, OutputPath, Priority.

[Docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

#### `Task CreateJobExportformdata(Guid projectId, int fileId, CreateJobExportformdataRequest request)`

Exports the form data from the Pdf file to a .xml, .csv, or .fdf file

**Remarks:** HTTP POST `/publicapi/v1/projects/{projectId}/files/{fileId}/jobs/exportformdata`. Vendor docs page: https://support.bluebeam.com/developer/

**Parameters:**
- `projectId` (Guid) — Path parameter `projectId` from `/publicapi/v1/projects/{projectId}/files/{fileId}/jobs/exportformdata`.
- `fileId` (int) — Path parameter `fileId` from `/publicapi/v1/projects/{projectId}/files/{fileId}/jobs/exportformdata`.
- `request` (CreateJobExportformdataRequest) — Request body. JSON object with fields: CurrentPassword, OutputFileName, OutputPath, Priority.

[Docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

#### `Task CreateJobExportmarkups(Guid projectId, int fileId, CreateJobExportmarkupsRequest request)`

Exports the Pdf file markups to an output file

**Remarks:** HTTP POST `/publicapi/v1/projects/{projectId}/files/{fileId}/jobs/exportmarkups`. Vendor docs page: https://support.bluebeam.com/developer/

**Parameters:**
- `projectId` (Guid) — Path parameter `projectId` from `/publicapi/v1/projects/{projectId}/files/{fileId}/jobs/exportmarkups`.
- `fileId` (int) — Path parameter `fileId` from `/publicapi/v1/projects/{projectId}/files/{fileId}/jobs/exportmarkups`.
- `request` (CreateJobExportmarkupsRequest) — Request body. JSON object with fields: CurrentPassword, OutputFileName, OutputPath, Priority.

[Docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

#### `Task CreateJobExtractpages(Guid projectId, int fileId, CreateJobExtractpagesRequest request)`

Extracts pages from the Pdf file

**Remarks:** HTTP POST `/publicapi/v1/projects/{projectId}/files/{fileId}/jobs/extractpages`. Vendor docs page: https://support.bluebeam.com/developer/

**Parameters:**
- `projectId` (Guid) — Path parameter `projectId` from `/publicapi/v1/projects/{projectId}/files/{fileId}/jobs/extractpages`.
- `fileId` (int) — Path parameter `fileId` from `/publicapi/v1/projects/{projectId}/files/{fileId}/jobs/extractpages`.
- `request` (CreateJobExtractpagesRequest) — Request body. JSON object with fields: CurrentPassword, OutputFileName, OutputPath, Priority.

[Docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

#### `Task CreateJobFileproperties(Guid projectId, int fileId, CreateJobFilepropertiesRequest request)`

Creates a job that will return the Pdf file properties

**Remarks:** HTTP POST `/publicapi/v1/projects/{projectId}/files/{fileId}/jobs/fileproperties`. Vendor docs page: https://support.bluebeam.com/developer/

**Parameters:**
- `projectId` (Guid) — Path parameter `projectId` from `/publicapi/v1/projects/{projectId}/files/{fileId}/jobs/fileproperties`.
- `fileId` (int) — Path parameter `fileId` from `/publicapi/v1/projects/{projectId}/files/{fileId}/jobs/fileproperties`.
- `request` (CreateJobFilepropertiesRequest) — Request body. JSON object with fields: CurrentPassword, Priority.

[Docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

#### `Task CreateJobFileproperty(Guid projectId, int fileId, CreateJobFilepropertyRequest request)`

Sets the value for the specified property in the Pdf file

**Remarks:** HTTP POST `/publicapi/v1/projects/{projectId}/files/{fileId}/jobs/fileproperty`. Vendor docs page: https://support.bluebeam.com/developer/

**Parameters:**
- `projectId` (Guid) — Path parameter `projectId` from `/publicapi/v1/projects/{projectId}/files/{fileId}/jobs/fileproperty`.
- `fileId` (int) — Path parameter `fileId` from `/publicapi/v1/projects/{projectId}/files/{fileId}/jobs/fileproperty`.
- `request` (CreateJobFilepropertyRequest) — Request body. JSON object with fields: CurrentPassword, Name, Priority, Value.

[Docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

#### `Task CreateJobFilepropertyvalue(Guid projectId, int fileId, CreateJobFilepropertyvalueRequest request)`

Creates a job that will return the value of a specified file property

**Remarks:** HTTP POST `/publicapi/v1/projects/{projectId}/files/{fileId}/jobs/filepropertyvalue`. Vendor docs page: https://support.bluebeam.com/developer/

**Parameters:**
- `projectId` (Guid) — Path parameter `projectId` from `/publicapi/v1/projects/{projectId}/files/{fileId}/jobs/filepropertyvalue`.
- `fileId` (int) — Path parameter `fileId` from `/publicapi/v1/projects/{projectId}/files/{fileId}/jobs/filepropertyvalue`.
- `request` (CreateJobFilepropertyvalueRequest) — Request body. JSON object with fields: CurrentPassword, Name, Priority.

[Docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

#### `Task CreateJobFlatten(Guid projectId, int fileId, CreateJobFlattenRequest request)`

Flattens the annotation to be part of the page content, for a given Pdf file

**Remarks:** HTTP POST `/publicapi/v1/projects/{projectId}/files/{fileId}/jobs/flatten`. Vendor docs page: https://support.bluebeam.com/developer/

**Parameters:**
- `projectId` (Guid) — Path parameter `projectId` from `/publicapi/v1/projects/{projectId}/files/{fileId}/jobs/flatten`.
- `fileId` (int) — Path parameter `fileId` from `/publicapi/v1/projects/{projectId}/files/{fileId}/jobs/flatten`.
- `request` (CreateJobFlattenRequest) — Request body. JSON object with fields: CurrentPassword, LayerName, OutputFileName, OutputPath, PageRange, Priority, Recoverable.

[Docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

#### `Task CreateJobHeaderandfooter(Guid projectId, int fileId, CreateJobHeaderandfooterRequest request)`

Applies headers and footers to the Pdf file

**Remarks:** HTTP POST `/publicapi/v1/projects/{projectId}/files/{fileId}/jobs/headerandfooter`. Vendor docs page: https://support.bluebeam.com/developer/

**Parameters:**
- `projectId` (Guid) — Path parameter `projectId` from `/publicapi/v1/projects/{projectId}/files/{fileId}/jobs/headerandfooter`.
- `fileId` (int) — Path parameter `fileId` from `/publicapi/v1/projects/{projectId}/files/{fileId}/jobs/headerandfooter`.
- `request` (CreateJobHeaderandfooterRequest) — Request body. JSON object with fields: CurrentPassword, FitToContent, OutputFileName, OutputPath, PageRange, Priority.

[Docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

#### `Task CreateJobImage2pdf(Guid projectId, int fileId, CreateJobImage2pdfRequest request)`

Converts an Image file to Pdf

**Remarks:** HTTP POST `/publicapi/v1/projects/{projectId}/files/{fileId}/jobs/image2pdf`. Vendor docs page: https://support.bluebeam.com/developer/

**Parameters:**
- `projectId` (Guid) — Path parameter `projectId` from `/publicapi/v1/projects/{projectId}/files/{fileId}/jobs/image2pdf`.
- `fileId` (int) — Path parameter `fileId` from `/publicapi/v1/projects/{projectId}/files/{fileId}/jobs/image2pdf`.
- `request` (CreateJobImage2pdfRequest) — Request body. JSON object with fields: OutputFileName, OutputPath, Priority.

[Docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

#### `Task CreateJobImportcustomcolumns(Guid projectId, int fileId, CreateJobImportcustomcolumnsRequest request)`

Imports a Custom Column definition .xml file into the Pdf file , overwriting any existing Custom Columns.               An .xml file can be generated by either the method 'exportcustomcolumns', or from within Bluebeam Revu

**Remarks:** HTTP POST `/publicapi/v1/projects/{projectId}/files/{fileId}/jobs/importcustomcolumns`. Vendor docs page: https://support.bluebeam.com/developer/

**Parameters:**
- `projectId` (Guid) — Path parameter `projectId` from `/publicapi/v1/projects/{projectId}/files/{fileId}/jobs/importcustomcolumns`.
- `fileId` (int) — Path parameter `fileId` from `/publicapi/v1/projects/{projectId}/files/{fileId}/jobs/importcustomcolumns`.
- `request` (CreateJobImportcustomcolumnsRequest) — Request body. JSON object with fields: CurrentPassword, CustomColumnsFileID, OutputFileName, OutputPath, Priority.

[Docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

#### `Task CreateJobImportmarkups(Guid projectId, int fileId, CreateJobImportmarkupsRequest request)`

Imports the markups from a list of files into the input Pdf file

**Remarks:** HTTP POST `/publicapi/v1/projects/{projectId}/files/{fileId}/jobs/importmarkups`. Vendor docs page: https://support.bluebeam.com/developer/

**Parameters:**
- `projectId` (Guid) — Path parameter `projectId` from `/publicapi/v1/projects/{projectId}/files/{fileId}/jobs/importmarkups`.
- `fileId` (int) — Path parameter `fileId` from `/publicapi/v1/projects/{projectId}/files/{fileId}/jobs/importmarkups`.
- `request` (CreateJobImportmarkupsRequest) — Request body. JSON object with fields: CurrentPassword, OutputFileName, OutputPath, Priority.

[Docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

#### `Task CreateJobInsertblankpages(Guid projectId, int fileId, CreateJobInsertblankpagesRequest request)`

Inserts new blank pages into the Pdf file using the specified parameters for width, height, count and style

**Remarks:** HTTP POST `/publicapi/v1/projects/{projectId}/files/{fileId}/jobs/insertblankpages`. Vendor docs page: https://support.bluebeam.com/developer/

**Parameters:**
- `projectId` (Guid) — Path parameter `projectId` from `/publicapi/v1/projects/{projectId}/files/{fileId}/jobs/insertblankpages`.
- `fileId` (int) — Path parameter `fileId` from `/publicapi/v1/projects/{projectId}/files/{fileId}/jobs/insertblankpages`.
- `request` (CreateJobInsertblankpagesRequest) — Request body. JSON object with fields: CurrentPassword, OutputFileName, OutputPath, Priority.

[Docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

#### `Task CreateJobInsertpages(Guid projectId, int fileId, CreateJobInsertpagesRequest request)`

Inserts a Pdf file into the active Pdf file using the specified parameters to control what additional data to be additionally imported such as bookmarks, file attachments, and file properties

**Remarks:** HTTP POST `/publicapi/v1/projects/{projectId}/files/{fileId}/jobs/insertpages`. Vendor docs page: https://support.bluebeam.com/developer/

**Parameters:**
- `projectId` (Guid) — Path parameter `projectId` from `/publicapi/v1/projects/{projectId}/files/{fileId}/jobs/insertpages`.
- `fileId` (int) — Path parameter `fileId` from `/publicapi/v1/projects/{projectId}/files/{fileId}/jobs/insertpages`.
- `request` (CreateJobInsertpagesRequest) — Request body. JSON object with fields: CurrentPassword, OutputFileName, OutputPath, Priority, SourceFileId.

[Docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

#### `Task CreateJobMarkuplist(Guid projectId, int fileId, CreateJobMarkuplistRequest request)`

Creates a job that will generate the list of the markups from the Pdf file

**Remarks:** HTTP POST `/publicapi/v1/projects/{projectId}/files/{fileId}/jobs/markuplist`. Vendor docs page: https://support.bluebeam.com/developer/

**Parameters:**
- `projectId` (Guid) — Path parameter `projectId` from `/publicapi/v1/projects/{projectId}/files/{fileId}/jobs/markuplist`.
- `fileId` (int) — Path parameter `fileId` from `/publicapi/v1/projects/{projectId}/files/{fileId}/jobs/markuplist`.
- `request` (CreateJobMarkuplistRequest) — Request body. JSON object with fields: CurrentPassword, PageRange, Priority.

[Docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

#### `Task CreateJobOpenpassword(Guid projectId, int fileId, CreateJobOpenpasswordRequest request)`

Sets open password for the Pdf file

**Remarks:** HTTP POST `/publicapi/v1/projects/{projectId}/files/{fileId}/jobs/openpassword`. Vendor docs page: https://support.bluebeam.com/developer/

**Parameters:**
- `projectId` (Guid) — Path parameter `projectId` from `/publicapi/v1/projects/{projectId}/files/{fileId}/jobs/openpassword`.
- `fileId` (int) — Path parameter `fileId` from `/publicapi/v1/projects/{projectId}/files/{fileId}/jobs/openpassword`.
- `request` (CreateJobOpenpasswordRequest) — Request body. JSON object with fields: CurrentPassword, OutputFileName, OutputPath, Password, Priority.

[Docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

#### `Task CreateJobPagecount(Guid projectId, int fileId, CreateJobPagecountRequest request)`

Creates a job that will return the number of pages in the Pdf file

**Remarks:** HTTP POST `/publicapi/v1/projects/{projectId}/files/{fileId}/jobs/pagecount`. Vendor docs page: https://support.bluebeam.com/developer/

**Parameters:**
- `projectId` (Guid) — Path parameter `projectId` from `/publicapi/v1/projects/{projectId}/files/{fileId}/jobs/pagecount`.
- `fileId` (int) — Path parameter `fileId` from `/publicapi/v1/projects/{projectId}/files/{fileId}/jobs/pagecount`.
- `request` (CreateJobPagecountRequest) — Request body. JSON object with fields: CurrentPassword, Priority.

[Docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

#### `Task CreateJobPagerotation(Guid projectId, int fileId, CreateJobPagerotationRequest request)`

Creates a job that will return the page rotation of a specified page in the Pdf file

**Remarks:** HTTP POST `/publicapi/v1/projects/{projectId}/files/{fileId}/jobs/pagerotation`. Vendor docs page: https://support.bluebeam.com/developer/

**Parameters:**
- `projectId` (Guid) — Path parameter `projectId` from `/publicapi/v1/projects/{projectId}/files/{fileId}/jobs/pagerotation`.
- `fileId` (int) — Path parameter `fileId` from `/publicapi/v1/projects/{projectId}/files/{fileId}/jobs/pagerotation`.
- `request` (CreateJobPagerotationRequest) — Request body. JSON object with fields: CurrentPassword, PageIndex, Priority.

[Docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

#### `Task CreateJobPagesize(Guid projectId, int fileId, CreateJobPagesizeRequest request)`

Creates a job that will return the page size of a specified page in the Pdf file

**Remarks:** HTTP POST `/publicapi/v1/projects/{projectId}/files/{fileId}/jobs/pagesize`. Vendor docs page: https://support.bluebeam.com/developer/

**Parameters:**
- `projectId` (Guid) — Path parameter `projectId` from `/publicapi/v1/projects/{projectId}/files/{fileId}/jobs/pagesize`.
- `fileId` (int) — Path parameter `fileId` from `/publicapi/v1/projects/{projectId}/files/{fileId}/jobs/pagesize`.
- `request` (CreateJobPagesizeRequest) — Request body. JSON object with fields: CurrentPassword, PageIndex, Priority.

[Docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

#### `Task CreateJobPdfsecurity(Guid projectId, int fileId, CreateJobPdfsecurityRequest request)`

Applies security permissions to the Pdf file

**Remarks:** HTTP POST `/publicapi/v1/projects/{projectId}/files/{fileId}/jobs/pdfsecurity`. Vendor docs page: https://support.bluebeam.com/developer/

**Parameters:**
- `projectId` (Guid) — Path parameter `projectId` from `/publicapi/v1/projects/{projectId}/files/{fileId}/jobs/pdfsecurity`.
- `fileId` (int) — Path parameter `fileId` from `/publicapi/v1/projects/{projectId}/files/{fileId}/jobs/pdfsecurity`.
- `request` (CreateJobPdfsecurityRequest) — Request body. JSON object with fields: CurrentPassword, OutputFileName, OutputPath, Priority.

[Docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

#### `Task CreateJobProcesscolors(Guid projectId, int fileId, CreateJobProcesscolorsRequest request)`

Converts page content colors to a color or gray scale

**Remarks:** HTTP POST `/publicapi/v1/projects/{projectId}/files/{fileId}/jobs/processcolors`. Vendor docs page: https://support.bluebeam.com/developer/

**Parameters:**
- `projectId` (Guid) — Path parameter `projectId` from `/publicapi/v1/projects/{projectId}/files/{fileId}/jobs/processcolors`.
- `fileId` (int) — Path parameter `fileId` from `/publicapi/v1/projects/{projectId}/files/{fileId}/jobs/processcolors`.
- `request` (CreateJobProcesscolorsRequest) — Request body. JSON object with fields: CurrentPassword, OutputFileName, OutputPath, Priority.

[Docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

#### `Task CreateJobPs2pdf(Guid projectId, int fileId, CreateJobPs2pdfRequest request)`

Converts a PostScript file to Pdf

**Remarks:** HTTP POST `/publicapi/v1/projects/{projectId}/files/{fileId}/jobs/ps2pdf`. Vendor docs page: https://support.bluebeam.com/developer/

**Parameters:**
- `projectId` (Guid) — Path parameter `projectId` from `/publicapi/v1/projects/{projectId}/files/{fileId}/jobs/ps2pdf`.
- `fileId` (int) — Path parameter `fileId` from `/publicapi/v1/projects/{projectId}/files/{fileId}/jobs/ps2pdf`.
- `request` (CreateJobPs2pdfRequest) — Request body. JSON object with fields: OutputFileName, OutputPath, Priority.

[Docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

#### `Task CreateJobReducefilesize(Guid projectId, int fileId, CreateJobReducefilesizeRequest request)`

Reduces the file size of the Pdf file

**Remarks:** HTTP POST `/publicapi/v1/projects/{projectId}/files/{fileId}/jobs/reducefilesize`. Vendor docs page: https://support.bluebeam.com/developer/

**Parameters:**
- `projectId` (Guid) — Path parameter `projectId` from `/publicapi/v1/projects/{projectId}/files/{fileId}/jobs/reducefilesize`.
- `fileId` (int) — Path parameter `fileId` from `/publicapi/v1/projects/{projectId}/files/{fileId}/jobs/reducefilesize`.
- `request` (CreateJobReducefilesizeRequest) — Request body. JSON object with fields: CurrentPassword, OutputFileName, OutputPath, Priority.

[Docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

#### `Task CreateJobRepair(Guid projectId, int fileId, CreateJobRepairRequest request)`

Runs a repair process on the Pdf file using the specified options

**Remarks:** HTTP POST `/publicapi/v1/projects/{projectId}/files/{fileId}/jobs/repair`. Vendor docs page: https://support.bluebeam.com/developer/

**Parameters:**
- `projectId` (Guid) — Path parameter `projectId` from `/publicapi/v1/projects/{projectId}/files/{fileId}/jobs/repair`.
- `fileId` (int) — Path parameter `fileId` from `/publicapi/v1/projects/{projectId}/files/{fileId}/jobs/repair`.
- `request` (CreateJobRepairRequest) — Request body. JSON object with fields: CurrentPassword, OutputFileName, OutputPath, Priority.

[Docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

#### `Task CreateJobReplacepages(Guid projectId, int fileId, CreateJobReplacepagesRequest request)`

Replaces pages in the current Pdf file with pages from the source Pdf file

**Remarks:** HTTP POST `/publicapi/v1/projects/{projectId}/files/{fileId}/jobs/replacepages`. Vendor docs page: https://support.bluebeam.com/developer/

**Parameters:**
- `projectId` (Guid) — Path parameter `projectId` from `/publicapi/v1/projects/{projectId}/files/{fileId}/jobs/replacepages`.
- `fileId` (int) — Path parameter `fileId` from `/publicapi/v1/projects/{projectId}/files/{fileId}/jobs/replacepages`.
- `request` (CreateJobReplacepagesRequest) — Request body. JSON object with fields: CurrentPassword, OutputFileName, OutputPath, Priority, SourceFileId.

[Docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

#### `Task CreateJobReversepages(Guid projectId, int fileId, CreateJobReversepagesRequest request)`

Reverses all pages in the Pdf file

**Remarks:** HTTP POST `/publicapi/v1/projects/{projectId}/files/{fileId}/jobs/reversepages`. Vendor docs page: https://support.bluebeam.com/developer/

**Parameters:**
- `projectId` (Guid) — Path parameter `projectId` from `/publicapi/v1/projects/{projectId}/files/{fileId}/jobs/reversepages`.
- `fileId` (int) — Path parameter `fileId` from `/publicapi/v1/projects/{projectId}/files/{fileId}/jobs/reversepages`.
- `request` (CreateJobReversepagesRequest) — Request body. JSON object with fields: CurrentPassword, OutputFileName, OutputPath, Priority.

[Docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

#### `Task CreateJobRotatepage(Guid projectId, int fileId, CreateJobRotatepageRequest request)`

Rotates the pages from a Pdf file by 90 degree increments

**Remarks:** HTTP POST `/publicapi/v1/projects/{projectId}/files/{fileId}/jobs/rotatepage`. Vendor docs page: https://support.bluebeam.com/developer/

**Parameters:**
- `projectId` (Guid) — Path parameter `projectId` from `/publicapi/v1/projects/{projectId}/files/{fileId}/jobs/rotatepage`.
- `fileId` (int) — Path parameter `fileId` from `/publicapi/v1/projects/{projectId}/files/{fileId}/jobs/rotatepage`.
- `request` (CreateJobRotatepageRequest) — Request body. JSON object with fields: CurrentPassword, OutputFileName, OutputPath, Priority.

[Docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

#### `Task CreateJobSaveaspdfa(Guid projectId, int fileId, CreateJobSaveaspdfaRequest request)`

Converts the current PDF document into a PDF/A-1b document

**Remarks:** HTTP POST `/publicapi/v1/projects/{projectId}/files/{fileId}/jobs/saveaspdfa`. Vendor docs page: https://support.bluebeam.com/developer/

**Parameters:**
- `projectId` (Guid) — Path parameter `projectId` from `/publicapi/v1/projects/{projectId}/files/{fileId}/jobs/saveaspdfa`.
- `fileId` (int) — Path parameter `fileId` from `/publicapi/v1/projects/{projectId}/files/{fileId}/jobs/saveaspdfa`.
- `request` (CreateJobSaveaspdfaRequest) — Request body. JSON object with fields: CurrentPassword, OutputFileName, OutputPath, Priority.

[Docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

#### `Task CreateJobSplitpages(Guid projectId, int fileId, CreateJobSplitpagesRequest request)`

Extracts all pages inside the Pdf file to individual Pdf files

**Remarks:** HTTP POST `/publicapi/v1/projects/{projectId}/files/{fileId}/jobs/splitpages`. Vendor docs page: https://support.bluebeam.com/developer/

**Parameters:**
- `projectId` (Guid) — Path parameter `projectId` from `/publicapi/v1/projects/{projectId}/files/{fileId}/jobs/splitpages`.
- `fileId` (int) — Path parameter `fileId` from `/publicapi/v1/projects/{projectId}/files/{fileId}/jobs/splitpages`.
- `request` (CreateJobSplitpagesRequest) — Request body. JSON object with fields: CurrentPassword, OutputPath, Priority.

[Docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

#### `Task CreateJobStamp(Guid projectId, int fileId, CreateJobStampRequest request)`

Stamps a Pdf file

**Remarks:** HTTP POST `/publicapi/v1/projects/{projectId}/files/{fileId}/jobs/stamp`. Vendor docs page: https://support.bluebeam.com/developer/

**Parameters:**
- `projectId` (Guid) — Path parameter `projectId` from `/publicapi/v1/projects/{projectId}/files/{fileId}/jobs/stamp`.
- `fileId` (int) — Path parameter `fileId` from `/publicapi/v1/projects/{projectId}/files/{fileId}/jobs/stamp`.
- `request` (CreateJobStampRequest) — Request body. JSON object with fields: CurrentPassword, OutputFileName, OutputPath, Priority, StampFileId.

[Docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

#### `Task CreateJobUnflatten(Guid projectId, int fileId, CreateJobUnflattenRequest request)`

Reverses the flattening process on a Pdf file

**Remarks:** HTTP POST `/publicapi/v1/projects/{projectId}/files/{fileId}/jobs/unflatten`. Vendor docs page: https://support.bluebeam.com/developer/

**Parameters:**
- `projectId` (Guid) — Path parameter `projectId` from `/publicapi/v1/projects/{projectId}/files/{fileId}/jobs/unflatten`.
- `fileId` (int) — Path parameter `fileId` from `/publicapi/v1/projects/{projectId}/files/{fileId}/jobs/unflatten`.
- `request` (CreateJobUnflattenRequest) — Request body. JSON object with fields: CurrentPassword, OutputFileName, OutputPath, PageRange, Priority.

[Docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

#### `Task CreateJobWord2pdf(Guid projectId, int fileId, CreateJobWord2pdfRequest request)`

Converts a Word file to Pdf

**Remarks:** HTTP POST `/publicapi/v1/projects/{projectId}/files/{fileId}/jobs/word2pdf`. Vendor docs page: https://support.bluebeam.com/developer/

**Parameters:**
- `projectId` (Guid) — Path parameter `projectId` from `/publicapi/v1/projects/{projectId}/files/{fileId}/jobs/word2pdf`.
- `fileId` (int) — Path parameter `fileId` from `/publicapi/v1/projects/{projectId}/files/{fileId}/jobs/word2pdf`.
- `request` (CreateJobWord2pdfRequest) — Request body. JSON object with fields: OutputFileName, OutputPath, Priority.

[Docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

#### `Task GetFileproperties(Guid projectId, int fileId, Guid jobId)`

Retrieves the file properties list generated by the 'fileproperties' job

**Remarks:** HTTP GET `/publicapi/v1/projects/{projectId}/files/{fileId}/jobs/fileproperties/{jobId}`. Vendor docs page: https://support.bluebeam.com/developer/

**Parameters:**
- `projectId` (Guid) — Path parameter `projectId` from `/publicapi/v1/projects/{projectId}/files/{fileId}/jobs/fileproperties/{jobId}`.
- `fileId` (int) — Path parameter `fileId` from `/publicapi/v1/projects/{projectId}/files/{fileId}/jobs/fileproperties/{jobId}`.
- `jobId` (Guid) — Path parameter `jobId` from `/publicapi/v1/projects/{projectId}/files/{fileId}/jobs/fileproperties/{jobId}`.

[Docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

#### `Task GetFilepropertyvalue(Guid projectId, int fileId, Guid jobId)`

Retrieves the value generated by the 'filepropertyvalue' job

**Remarks:** HTTP GET `/publicapi/v1/projects/{projectId}/files/{fileId}/jobs/filepropertyvalue/{jobId}`. Vendor docs page: https://support.bluebeam.com/developer/

**Parameters:**
- `projectId` (Guid) — Path parameter `projectId` from `/publicapi/v1/projects/{projectId}/files/{fileId}/jobs/filepropertyvalue/{jobId}`.
- `fileId` (int) — Path parameter `fileId` from `/publicapi/v1/projects/{projectId}/files/{fileId}/jobs/filepropertyvalue/{jobId}`.
- `jobId` (Guid) — Path parameter `jobId` from `/publicapi/v1/projects/{projectId}/files/{fileId}/jobs/filepropertyvalue/{jobId}`.

[Docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

#### `Task GetMarkuplist(Guid projectId, int fileId, Guid jobId)`

Retrieves the markup list generated by the 'markuplist' job

**Remarks:** HTTP GET `/publicapi/v1/projects/{projectId}/files/{fileId}/jobs/markuplist/{jobId}`. Vendor docs page: https://support.bluebeam.com/developer/

**Parameters:**
- `projectId` (Guid) — Path parameter `projectId` from `/publicapi/v1/projects/{projectId}/files/{fileId}/jobs/markuplist/{jobId}`.
- `fileId` (int) — Path parameter `fileId` from `/publicapi/v1/projects/{projectId}/files/{fileId}/jobs/markuplist/{jobId}`.
- `jobId` (Guid) — Path parameter `jobId` from `/publicapi/v1/projects/{projectId}/files/{fileId}/jobs/markuplist/{jobId}`.

[Docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

#### `Task GetPagecount(Guid projectId, int fileId, Guid jobId)`

Retrieves the 'PageCount' value generated by the 'pagecount' job

**Remarks:** HTTP GET `/publicapi/v1/projects/{projectId}/files/{fileId}/jobs/pagecount/{jobId}`. Vendor docs page: https://support.bluebeam.com/developer/

**Parameters:**
- `projectId` (Guid) — Path parameter `projectId` from `/publicapi/v1/projects/{projectId}/files/{fileId}/jobs/pagecount/{jobId}`.
- `fileId` (int) — Path parameter `fileId` from `/publicapi/v1/projects/{projectId}/files/{fileId}/jobs/pagecount/{jobId}`.
- `jobId` (Guid) — Path parameter `jobId` from `/publicapi/v1/projects/{projectId}/files/{fileId}/jobs/pagecount/{jobId}`.

[Docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

#### `Task GetPagerotation(Guid projectId, int fileId, Guid jobId)`

Retrieves the 'PageRotation' value generated by the 'pagerotation' job

**Remarks:** HTTP GET `/publicapi/v1/projects/{projectId}/files/{fileId}/jobs/pagerotation/{jobId}`. Vendor docs page: https://support.bluebeam.com/developer/

**Parameters:**
- `projectId` (Guid) — Path parameter `projectId` from `/publicapi/v1/projects/{projectId}/files/{fileId}/jobs/pagerotation/{jobId}`.
- `fileId` (int) — Path parameter `fileId` from `/publicapi/v1/projects/{projectId}/files/{fileId}/jobs/pagerotation/{jobId}`.
- `jobId` (Guid) — Path parameter `jobId` from `/publicapi/v1/projects/{projectId}/files/{fileId}/jobs/pagerotation/{jobId}`.

[Docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

#### `Task GetPagesize(Guid projectId, int fileId, Guid jobId)`

Retrieves the 'PageSize' value generated by the 'pagesize' job

**Remarks:** HTTP GET `/publicapi/v1/projects/{projectId}/files/{fileId}/jobs/pagesize/{jobId}`. Vendor docs page: https://support.bluebeam.com/developer/

**Parameters:**
- `projectId` (Guid) — Path parameter `projectId` from `/publicapi/v1/projects/{projectId}/files/{fileId}/jobs/pagesize/{jobId}`.
- `fileId` (int) — Path parameter `fileId` from `/publicapi/v1/projects/{projectId}/files/{fileId}/jobs/pagesize/{jobId}`.
- `jobId` (Guid) — Path parameter `jobId` from `/publicapi/v1/projects/{projectId}/files/{fileId}/jobs/pagesize/{jobId}`.

[Docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

