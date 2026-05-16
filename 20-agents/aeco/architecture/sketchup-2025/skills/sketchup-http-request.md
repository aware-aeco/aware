---
name: yard-sketchup-http-request
description: Sketchup::Http::Request API reference (YARD)
---

# Sketchup::Http::Request API reference

Request objects allows you to send HTTP request to HTTP servers.

## Methods

- `initialize` — If no reference is kept to the Sketchup::Http::Request, it can be garbage collected, making the download silently fail. This is especially noticeable for larger downloads that takes longer time.
- `body` — Gets the HTTP body that is going to be used when sending the request.
- `body=` — Sets the HTTP body that is going to be used when sending the request.
- `cancel` — Do not #cancel the request in the response callback.
- `headers` — Returns the HTTP headers that are going to be used when sending the request.
- `headers=` — Sets the HTTP headers that are going to be used when sending the request.
- `method` — Returns the HTTP method that is going to be used when sending the request.
- `method=` — Sets the HTTP method that is going to be used when sending the request.
- `set_download_progress_callback` — total is -1 if the server doesn't specify a file size in the response header.
- `set_upload_progress_callback` — Adds a upload progress callback block that will get called everytime we have uploaded data to the server until the upload finishes.
- `start` — Do not #cancel the request in the response callback.
- `status` — Returns the internal status code. It can be one of the following:
- `url` — Returns a copy of the Request's URL.
