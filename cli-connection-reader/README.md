# Connection-reader managed authority configuration

Reader 0.8.1 / bridge 0.5.4 supports the optional process configuration
`AWARE_MODEL_PROVIDER_AUTHORITY_STORE_DIR` for managed protocol-2 providers. The operator's host
sets it independently before launching AWARE. It is not derived from workflow input or provider stdin.

The reader requires a nonempty string of at most 4096 code units, with no NUL, that is absolute and
already equals the platform's `path.resolve` result. It must exactly equal the requested canonical
`authority-store-path`. Case, separators and alias spellings are not repaired. Windows environment
key aliases with identical values collapse to the canonical key; conflicting aliases are rejected.
Invalid or mismatched configuration fails before any provider launch.

Only this directory joins the existing minimal environment during protocol-2 describe and convert.
Credentials, proxy variables, caller PATH and application-specific storage overrides remain excluded.
Absent configuration preserves existing provider defaults. Protocol 1 and package/v3 launch
environments are unchanged. This chooses where a provider can look for authority; it grants no
authentication, entitlement or approval, and the provider must still enforce its own exact store binding.
