# rebar-free-form-accessor-disconnect-from-server

Lifecycle: single

Sets the GUID of the API server to invalid value and removes all the server related data from the Rebar (ex. the current constraints and the handle tags are removed).    Calling this method will result in a Rebar that will not react to host changes anymore, however it will still have all the properties that it used to have.
