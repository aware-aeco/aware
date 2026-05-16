# custom-render-mesh-provider-register-providers

Lifecycle: single

Call this method once from your plug-ins OnLoad override for each             assembly containing a custom mesh provider.  Only publicly exported             classes derived from CustomRenderMeshProvider with a public constructor             that has no parameters will get registered.
