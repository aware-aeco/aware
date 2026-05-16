# igh-param-relink-proxy-sources

Lifecycle: single

Attempt to replace all proxy sources with real sources.   Proxy sources are used during file IO, when actual sources might not be available yet.   Once an IO operation has been completed there should be no more proxy sources.
