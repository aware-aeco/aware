---
name: sketchup-sketchup-http
description: This skill encodes the sketchup 2025.0 surface of the Sketchup::Http namespace — 2 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: Request, Response.
---

# Sketchup::Http

Auto-generated from vendor docs for sketchup 2025.0. 2 types in this namespace.

## Request (class)

Request objects allows you to send HTTP request to HTTP servers.

[Vendor docs](https://ruby.sketchup.com/Sketchup/Http/Request.html)

### Constructors
- `Request(url, method = Sketchup::Http::GET)` — Note: If no reference is kept to the Sketchup::Http::Request, it can be garbage collected, making the download silently fail. This is especially noticeable for larger downloads that takes longer time. Note: Do not #cancel the request in the response callback. The default port is 80, to use a different port define it in the URL when creating a new Sketchup::Http::Request. The #method parameter accepts any custom HTTP method or one of the following: GET POST PUT DELETE HEAD OPTIONS

### Methods
#### `cancel => true`

Note: Do not #cancel the request in the response callback.

**Remarks:** Note: Do not #cancel the request in the response callback. Cancels the request.

**Returns:** `true` — 

[Docs](https://ruby.sketchup.com/Sketchup/Http/Request.html#cancel-instance_method)

#### `set_download_progress_callback {|current, total| ... } => Boolean`

Note: total is -1 if the server doesn't specify a file size in the response header.

**Remarks:** Note: total is -1 if the server doesn't specify a file size in the response header. Adds a download progress callback block that will get called everytime we have received data from the server until the download finishes.

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Sketchup/Http/Request.html#set_download_progress_callback-instance_method)

#### `set_upload_progress_callback {|current, total| ... } => Boolean`

Adds a upload progress callback block that will get called everytime we have uploaded data to the server until the upload finishes.

**Remarks:** Adds a upload progress callback block that will get called everytime we have uploaded data to the server until the upload finishes.

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Sketchup/Http/Request.html#set_upload_progress_callback-instance_method)

#### `start {|request, response| ... } => Boolean`

Note: Do not #cancel the request in the response callback.

**Remarks:** Note: Do not #cancel the request in the response callback. Starts the request with optionally a response callback block.

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/Sketchup/Http/Request.html#start-instance_method)

#### `status => Integer`

Returns the internal status code.

**Remarks:** Returns the internal status code. It can be one of the following: Sketchup::Http::STATUS_UNKNOWN Sketchup::Http::STATUS_SUCCESS Sketchup::Http::STATUS_PENDING Sketchup::Http::STATUS_CANCELED Sketchup::Http::STATUS_FAILED

**Returns:** `Integer` — 

[Docs](https://ruby.sketchup.com/Sketchup/Http/Request.html#status-instance_method)

#### `url => String`

Returns a copy of the Request's URL.

**Remarks:** Returns a copy of the Request's URL.

**Returns:** `String` — 

[Docs](https://ruby.sketchup.com/Sketchup/Http/Request.html#url-instance_method)

### Properties
- `body` (String, get/set) — Gets the HTTP body that is going to be used when sending the request.
- `headers` (Hash, get/set) — Returns the HTTP headers that are going to be used when sending the request.
- `method` (String, get/set) — Returns the HTTP method that is going to be used when sending the request.

## Response (class)

Note: Do not Sketchup::Http::Request#cancel the request in the response callback. Response objects allows you to get the response information from the server, you can only receive the Response if you have attached a callback block when calling start from the Request object.

[Vendor docs](https://ruby.sketchup.com/Sketchup/Http/Response.html)

### Methods
#### `body => String`

Gets the HTTP body that was received from the server as a string encoded using the charset provided in the “Content-Type” header of the server response, if no charset is specified, Encoding::ASCII_8BIT will be used.

**Remarks:** Gets the HTTP body that was received from the server as a string encoded using the charset provided in the “Content-Type” header of the server response, if no charset is specified, Encoding::ASCII_8BIT will be used.

**Returns:** `String` — 

[Docs](https://ruby.sketchup.com/Sketchup/Http/Response.html#body-instance_method)

#### `headers => Hash`

Returns the HTTP headers that were sent by the server.

**Remarks:** Returns the HTTP headers that were sent by the server.

**Returns:** `Hash` — 

[Docs](https://ruby.sketchup.com/Sketchup/Http/Response.html#headers-instance_method)

#### `status_code => Integer`

Returns the HTTP response status code as defined in rfc2616.

**Remarks:** Returns the HTTP response status code as defined in rfc2616.

**Returns:** `Integer` — 

[Docs](https://ruby.sketchup.com/Sketchup/Http/Response.html#status_code-instance_method)

