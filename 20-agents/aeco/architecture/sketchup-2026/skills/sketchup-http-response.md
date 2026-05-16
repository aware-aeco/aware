---
name: yard-sketchup-http-response
description: Sketchup::Http::Response API reference (YARD)
---

# Sketchup::Http::Response API reference

Do not Sketchup::Http::Request#cancel the request in the response callback.

## Methods

- `body` — Gets the HTTP body that was received from the server as a string encoded using the charset provided in the “Content-Type” header of the server response, if no charset is specified, Encoding::ASCII_8BIT will be used.
- `headers` — Returns the HTTP headers that were sent by the server.
- `status_code` — Returns the HTTP response status code as defined in rfc2616.
