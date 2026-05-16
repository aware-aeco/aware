# `aware-sidecar`

A C# NativeAOT companion binary for the AWARE CLI. Handles .NET-heavy operations
(DLL reflection via `MetadataLoadContext`, decompile via `ilspycmd`) that are
awkward to do in pure Rust.

The Rust CLI (`aware`) spawns this binary as a subprocess, talks to it over
stdin/stdout JSON, and serializes the result into a generated AWARE agent.

## Build

```bash
# Debug build (fast, requires .NET 9 runtime to run)
dotnet build

# Release build with NativeAOT (single-file binary, no runtime needed)
dotnet publish -c Release -r win-x64 -p:PublishAot=true
# → bin/Release/net9.0/win-x64/publish/aware-sidecar.exe
```

## IPC protocol

One JSON request on stdin, one JSON response on stdout, process exits.

Request shape (one JSON object per invocation):

```json
{ "op": "reflect-dlls", "args": { "globs": ["C:/foo/*.dll"], "agent_id": "tekla" } }
```

Response shape:

```json
{ "ok": true, "version": "0.5.1", "op": "reflect-dlls", "data": { ... } }
```

```json
{ "ok": false, "version": "0.5.1", "error": "..." }
```

Operations:
- `reflect-dlls` — load .NET assemblies via `MetadataLoadContext`; emit an AWARE agent
- `decompile` — shell out to `ilspycmd`; emit an AWARE agent

## License

Apache-2.0 (same as the parent repo).
