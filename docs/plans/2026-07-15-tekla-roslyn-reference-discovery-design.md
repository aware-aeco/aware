# Tekla Roslyn reference discovery

## Context

The `aware-tekla` bridge compiled scripts against a fixed list of five Tekla assemblies. That list
covered the original model and drawing probes but drifted from the public Open API surface: a script
using `Tekla.Structures.Catalogs` failed even though the matching DLL was installed. Extending the
list one assembly at a time would preserve the same maintenance failure for Analysis, Plugins,
Dialog, Direct Manipulation, and future Tekla releases.

## Decision

Resolve Roslyn references from the selected Tekla installation instead of maintaining an assembly
whitelist. The bridge scans the manifest-defined probe locations in order—`bin/Net48Runtime` before
`bin`—for top-level files matching `Tekla.Structures*.dll`. Candidates are sorted by filename for
deterministic compiler inputs. The first valid managed assembly with a given filename wins, which
keeps the .NET Framework build preferred when Tekla ships the same assembly in both locations.

Each candidate is checked with `AssemblyName.GetAssemblyName` before becoming a Roslyn metadata
reference. This excludes native helpers such as `Tekla.Structures.Native.DbvDatabase.dll` and skips
invalid, unsupported, raced, or unreadable files without preventing otherwise valid scripts from
compiling. Runtime dependency loading remains unchanged: the existing `AssemblyResolve` handler
uses the same ordered probe locations.

The scope is intentionally `Tekla.Structures*.dll`, matching the Open API assembly naming family
and the user's script-facing type surface. Unrelated implementation dependencies are still loaded
transitively at runtime when required; they are not automatically exposed as direct scripting
references.

## Verification

A filesystem-isolated unit test supplies arbitrary managed Analysis, Catalogs, Plugins, and core
assemblies plus a fake native DLL. It proves discovery is open-ended, probe precedence is retained,
and native binaries are excluded. The real end-to-end gate installs the Tekla agent and bridge into
a temporary `AWARE_HOME`, runs a read-only app through `aware app run`, and compiles types from
multiple public assemblies against the running Tekla host.
