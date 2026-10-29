# SketchUp and Revit 2027

The `sketchup-2027` and `revit-2027` agent packages extend the existing 2025 and
2026 packages. They retain the curated workflow commands and use newly generated
API references, rather than renaming the 2026 API snapshot.

## Install

```powershell
aware agent install sketchup-2027
aware sidecar install sketchup
& "$HOME/.aware/bridges/aware-sketchup.exe" --install-bridge --target-year 2027

aware agent install revit-2027
aware sidecar install revit
pwsh "$HOME/.aware/bridges/install-addin.ps1" -RevitVersion 2027
```

Restart the host after installing its bridge. Use the same explicit year to
install for 2025 or 2026. Installer defaults remain 2026.

SketchUp reports its host version as `27.0...`; use `version: "27"` to select it
in bridge inputs. Revit uses `version: "2027"`. When multiple models or processes
are open, select the intended instance explicitly.

## API sources and builds

- SketchUp: the unified [Trimble Ruby API documentation](https://ruby.sketchup.com/)
  extracted on October 28, 2026. The snapshot contains 161 types and includes
  2027 additions such as `Sketchup::Entities#add_copy`.
- Revit: RevitAPI and RevitAPIUI 2027.3.0 reference assemblies distributed by
  Nice3point on NuGet. The existing `Autodesk.Revit.SDK` package ends at 2026.
  The new reference covers these two assemblies; it does not claim the additional
  auxiliary assemblies included by the 2026 SDK snapshot.
- The runtime bridge compiles against the **2027.0.0** baseline references so its
  assembly dependency is RevitAPI 27.0, compatible with subsequent 2027 updates.
  The generated API reference describes 2027.3; newer API members require the
  corresponding host update.
- [Revit 2027 uses .NET 10](https://blog.autodesk.io/revit-2027-sdk-net-10-api-changes-and-additions/).
  The release zip contains separate `AwareRevit/2025`, `2026`, and `2027` payloads.
  The installer checks a target-year marker before deploying. Revit's own API
  assemblies are referenced during compilation and are never included in the zip.

Build the 2027 add-in from a checkout with:

```powershell
dotnet build cli-revit/AwareRevit/AwareRevit.csproj -c Release -p:RevitVersion=2027
pwsh cli-revit/install-addin.ps1 -Configuration Release -RevitVersion 2027
```

CI compiles the 2027 add-in and runs the Revit tests, including semantic compilation
of the bake script against the 2027 API. Automated installer tests use isolated
temporary directories.

Live verification passed in SketchUp 27.0.267 (Ruby 3.4.9) and Revit 2027.3
(27.3.0.28, .NET 10.0.9). Each host ran a real, compiled AWARE app using its
2027 agent and development bridge: create a 1,000 × 100 × 200 mm beam, replace
it with a 1,500 mm beam, and verify dimensions, volume, ownership, retirement,
and preservation of unrelated geometry. SketchUp's 2027 `Entities#add_copy`
also passed a native API check. Revit used a separate scratch project and the
documented DirectShape fallback because the test profile family was not loaded;
parametric family placement was not exercised by this check.
