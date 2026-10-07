# Changelog

## 0.8.1 — 2026-10-07

- Managed protocol-2 provider launches can receive one independent host-configured authority directory through `AWARE_MODEL_PROVIDER_AUTHORITY_STORE_DIR`. When supplied, it must be a bounded canonical absolute path exactly matching the requested authority store before either describe or convert launches. Conflicting Windows key aliases fail explicitly.
- Existing minimal environments remain unchanged for protocol 1 and package/v3 providers. No credentials, licence decisions or provider-specific environment overrides are added to AWARE; providers still authenticate independently.
- Ships with connection-reader bridge 0.5.4. Snapshot packager identity reports reader 0.8.1 and bridge `aware-connection-reader@0.5.4`.
