---
name: tekla-tekla-structures-headless-service
description: This skill encodes GENUINE headless Tekla Structures (tekla 2025.0) via Tekla.Structures.Service.TeklaStructuresService — interacting with a model through the Open API with NO Tekla UI process at all. Distinct from the GUI-bypass launch (Bypass.ini + TeklaStructures.exe). Covers the Service NuGet, the TeklaStructuresService constructor (binDir/language/env-ini/role-ini), Initialize(modelPath, license), Flex-only licensing, the mandatory AssemblyResolve hook, the no-debugger gotcha, and the one-service-per-process dispose rule. NOT in the scraped Open API reference tree — hand-authored from the official HeadlessTeklaStructuresExample repo. Read when the sidecar host must run Tekla without a desktop UI (CI, servers, batch).
---

# Tekla headless service (no UI)

**`Tekla.Structures.Service.TeklaStructuresService` runs the full Open API against a model with NO Tekla Structures UI process. This is real headless — not the GUI-bypass trick. It is the right host for a server/CI/batch sidecar.**

> **Why this doc is hand-authored.** `Tekla.Structures.Service` ships as its own NuGet (`2025.0.0`; net48; x64) and is **not** in the scraped `developer.tekla.com` reference tree. Surface verified against [HeadlessTeklaStructuresExample](https://github.com/TrimbleSolutionsCorporation/HeadlessTeklaStructuresExample) — which ships `2021` + `2026` examples.
>
> ⚠️ **Version caveat — 2025 is expected, not example-verified.** The official example repo has **no 2025 folder** (only 2021 + 2026). However, the `Tekla.Structures.Service` NuGet **does publish `2025.0.0`**, so the headless API is shipped for 2025 and the code below *should* run unchanged on a 2025 install — but **smoke-test it before relying on it**. The 2026 sibling doc is the example-confirmed one.

## Two different "headless" — pick the right one

| Mechanism | What it is | Use when |
|---|---|---|
| **GUI-bypass** ([headless-startup-and-shutdown.md](../../tekla/skills/headless-startup-and-shutdown.md)) | Launch the full `TeklaStructures.exe` non-interactively via `Bypass.ini`, poll ~30s for Open API readiness | You need the UI available, or the license isn't Flex |
| **Service (this doc)** | `TeklaStructuresService` loads the model in-process, **no UI window at all** | Servers, CI, batch — and the AWARE sidecar host. Flex license required |

> **Supersedes a stale claim:** the older GUI-bypass skill states Tekla has "no headless build." That is no longer true — the Service API *is* genuine headless. Keep the GUI-bypass path for non-Flex / UI-needed scenarios.

## Licensing: Flex license type only (hard prerequisite)

**Genuine headless requires a Flex license *type* — not a Named-user subscription.** This is the gating constraint: no Flex → no headless service, fall back to GUI-bypass.

- **Flex** = the floating/network license type, checked out from a license server (`XS_LICENSE_SERVER_HOST=<port>@<host>`) and checked back in on close. Per Trimble it's usable only within the country of purchase, with unlimited reassignment.
- **Named-user subscriptions are NOT supported headless** — they're tied to a Trimble Identity and the service won't run on them.
- The **license *configuration*** is a separate axis: `Initialize(modelPath, "Full")` takes the tier ("Full", or CARBON / GRAPHITE / DIAMOND). Flex is *how* the license is delivered; "Full" is *which* configuration.

Workflow: start Tekla once with the Flex license to activate the checkout, close it, then headless apps can run. The example injects the license by appending to a **copy** of `TeklaStructures.ini` (never edit the install's ini):

```csharp
var overrideIni = Path.Combine(runPath, "TeklaStructures.ini");
File.Copy(Path.Combine(binDir, "TeklaStructures.ini"), overrideIni, overwrite: true);
File.AppendAllText(overrideIni, "\r\nset XS_LICENSE_SERVER_HOST=27001@<host>\r\n");
File.AppendAllText(overrideIni, "set XS_DEFAULT_LICENSE=Full\r\n");
```

## License detection — choose headless vs GUI-bypass automatically

The host must decide *before* committing to headless, because a Named-user machine cannot run the service at all. Detection is a three-tier strategy, cheapest first:

**Tier 1 — read `XS_LICENSE_SERVER_HOST` (cheap, no Tekla load).** This single value is the tell (see [headless-startup-and-shutdown.md](../../tekla/skills/headless-startup-and-shutdown.md) for the magic-string semantics):

| `XS_LICENSE_SERVER_HOST` value | Licensing | Headless? |
|---|---|---|
| `<port>@<host>` (e.g. `27001@server`) | **Flex** — FlexNet floating server | ✅ candidate |
| `https` | Trimble Identity online subscription (**Named**) | ❌ use GUI-bypass |
| empty / unset | Unknown | probe further or default to GUI-bypass |

```csharp
enum TeklaLicensing { FlexFloating, NamedSubscription, Unknown }

static TeklaLicensing DetectFromConfig()
{
    // Prefer the process env var; fall back to parsing `set XS_LICENSE_SERVER_HOST=...`
    // out of the resolved TeklaStructures.ini.
    var host = Environment.GetEnvironmentVariable("XS_LICENSE_SERVER_HOST")
               ?? ReadIniSetting("XS_LICENSE_SERVER_HOST");
    if (string.IsNullOrWhiteSpace(host))                          return TeklaLicensing.Unknown;
    if (host.Equals("https", StringComparison.OrdinalIgnoreCase)) return TeklaLicensing.NamedSubscription;
    if (Regex.IsMatch(host, @"^\d+@"))                            return TeklaLicensing.FlexFloating;
    return TeklaLicensing.Unknown;
}
```

**Tier 2 — confirm a free seat (optional, robust).** A Flex server can still be out of seats or unreachable. If FlexNet tooling is present, query it before launching: `lmutil lmstat -a -c <port>@<host>` and confirm an available Tekla feature line. Treat "server unreachable" or "0 available" as "fall back to GUI-bypass," not a hard error.

**Tier 3 — authoritative backstop (guarded init).** Detection is a prediction; the only certainty is the service itself. Wrap construction/`Initialize` in try/catch and treat a license-related failure as the signal to fall back — but only as a *last* resort, since init is heavy and (per the gotchas below) can't be stepped in a debugger:

```csharp
if (DetectFromConfig() == TeklaLicensing.FlexFloating /* && SeatAvailable() */)
{
    try            { RunHeadless(); }                 // TeklaStructuresService path
    catch (Exception ex) when (IsLicenseFailure(ex)) { RunGuiBypass(); }  // graceful fallback
}
else
{
    RunGuiBypass();   // Named / unknown → the Bypass.ini + TeklaStructures.exe path
}
```

**Rule:** Tier 1 chooses the path, Tier 2 hardens it, Tier 3 guarantees correctness. Never let a license-mode misprediction surface as an opaque crash — always degrade to GUI-bypass.

## The lifecycle

```csharp
using Tekla.Structures.Model;
using Tekla.Structures.Service;

var binDir = $@"C:\Program Files\Tekla Structures\2025.0\bin";

// 1. MANDATORY: resolve Tekla assemblies from binDir (don't copy DLLs into your output).
AppDomain.CurrentDomain.AssemblyResolve += (s, a) =>
{
    var name = new AssemblyName(a.Name).Name + ".dll";
    var path = Path.Combine(binDir, name);
    return File.Exists(path) ? Assembly.LoadFile(path) : null;
};

// 2. Construct the service: binaries, language, environment .ini, role .ini.
using (var service = new TeklaStructuresService(
    new DirectoryInfo(binDir),
    "ENGLISH",
    new FileInfo(@"C:\ProgramData\Trimble\Tekla Structures\2025.0\Environments\default\env_Default_environment.ini"),
    new FileInfo(@"C:\ProgramData\Trimble\Tekla Structures\2025.0\Environments\default\role_Steel_Detailer.ini")))
{
    // 3. Open the model headless. Second arg is the license/configuration ("Full", etc.).
    service.Initialize(new DirectoryInfo(@"C:\TeklaStructuresModels\My model"), "Full");

    // 4. Full Open API now works — no UI:
    Console.WriteLine(new Model().GetInfo().ModelName);
    var beam = new Beam
    {
        StartPoint = new Tekla.Structures.Geometry3d.Point(0, 0, 0),
        EndPoint   = new Tekla.Structures.Geometry3d.Point(3000, 0, 0),
    };
    beam.Profile.ProfileString = "HEA400";
    beam.Insert();
    new ModelHandler().Save(@"C:\TeklaStructuresModels\My model");
}   // 5. Dispose = clean exit.
```

## Three gotchas that will bite

1. **AssemblyResolve is mandatory.** Without the resolver hooking `binDir`, the Tekla DLLs won't load and the service throws on construction. Set it *before* constructing the service.
2. **No debugger across `Initialize`.** Per the example's emphatic comment, you **cannot run this code under a debugger** through `Initialize`. Pattern: launch normally, drop a `Console.ReadKey()` after `Initialize`, then *attach* the debugger to your own code afterward.
3. **One service per process, period.** "Once disposed, it cannot be used again during the process lifetime." Don't try to re-`Initialize` or spin a second service in the same process — start a fresh process.

## NuGet

`Tekla.Structures.Service` `2025.0.0` (confirmed published) + `Tekla.Structures.Model` `2025.x`. `<TargetFrameworks>net48</TargetFrameworks>`, `<PlatformTarget>x64</PlatformTarget>`, `OutputType=Exe`, `App.config` pinning `.NETFramework,Version=v4.8`.

## Where this fits the plugin-forge / sidecar host

This is the cleanest implementation of the **.NET sidecar host**: a net48 process that wraps `TeklaStructuresService`, exposes the local HTTP/SSE API, and runs the live model with no desktop UI — ideal for CI extraction (Phase 0 introspection) and headless compile/deploy. Flex license is the gating constraint. See [[tekla-plugin-forge]].

## Source

- Verified code: [HeadlessTeklaStructuresExample](https://github.com/TrimbleSolutionsCorporation/HeadlessTeklaStructuresExample) — `HeadlessExample.2026/Program.cs`, `.csproj`, `App.config` (repo ships `2021` + `2026` examples; `TeklaStructuresService` API is stable across versions, so this 2025 doc mirrors it).
- The exact `Tekla.Structures.Service` member signatures should be regenerated by reflecting the `Tekla.Structures.Service` 2025.x NuGet.
