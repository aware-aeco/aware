---
name: tekla-tekla-structures-fusion-ui
description: This skill encodes TeklaFusion — Trimble's WPF UI framework that Tekla Structures itself is built on (tekla 2025.0 / TeklaFusion 4.1.10 line). It is the native look-and-feel layer for Tekla extension UIs and plugin dialogs: an MVVM app host (Fusion.App, ViewModel/WindowViewModel), the `[CommandHandler]` command pattern, Host.UI dialog services, and the `http://schemas.trimble.com/fusion` XAML namespace (Theme styles, boolean→visibility converters, watermark attached properties). NOT in the scraped Open API reference tree — hand-authored from the official FusionExample repo. Read when building a WPF dialog/tool that must look native inside Tekla, or when generating the UI layer of a plugin.
---

# TeklaFusion (native WPF UI)

**TeklaFusion is the WPF framework Tekla Structures is built on. Use it so your extension or plugin dialog looks and behaves native — themed window chrome, Tekla controls, MVVM with attribute-bound commands — instead of a bolted-on WinForms/plain-WPF dialog.**

> **Why this doc is hand-authored.** `TeklaFusion` ships as its own NuGet (the `4.1.10` line for tekla 2025; targets net48 + net6/8/9; depends on Newtonsoft.Json) and is **not** in the scraped `developer.tekla.com` reference tree. Surface verified against [FusionExample](https://github.com/TrimbleSolutionsCorporation/FusionExample). The `Tekla.Structures.Plugins.DirectManipulation` NuGet depends on `TeklaFusion`, so pulling DM pulls Fusion too.

## The three pieces

| Piece | Type / hook | Role |
|---|---|---|
| **App host** | `Fusion.App` → `App.Start(new MyApp())` | Entry point; owns lifetime, key handling, the `Host` services |
| **View model** | `ViewModel` / `WindowViewModel` base; `SetValue(ref field, value)` | MVVM base with change notification; no manual `INotifyPropertyChanged` |
| **View binding** | `xmlns:ui="http://schemas.trimble.com/fusion"` | Theme styles, converters, markup extensions, attached properties |

## App + published views

```csharp
public class ExampleApp : App
{
    [STAThread]
    public static void Main() => App.Start(new ExampleApp());

    [PublishedView("App.MainWindow")]               // registers a view by name
    public ViewModel CreateMainWindow(object parameter) => new MainWindowViewModel();

    protected override void OnKeyDown(KeyEventArgs e)
    {
        base.OnKeyDown(e);
        if (e.Match(Key.F9)) { e.Handled = true; this.Host.UI.ShowDiagnosticWindow(); }
    }
}
```

`[PublishedView("name")]` is how Fusion discovers a window's view model. `Host.UI` is the service surface (`ShowDiagnosticWindow()`, `ShowMessageDialog(...)`).

## View model — commands without ICommand

`WindowViewModel` gives you change-notifying properties via `SetValue`, and `[CommandHandler]` turns a plain method into a bindable command — no `ICommand`/`RelayCommand` boilerplate:

```csharp
public class MainWindowViewModel : WindowViewModel
{
    private bool isConnected;
    public bool IsConnected { get => isConnected; private set => SetValue(ref isConnected, value); }

    [CommandHandler]
    public void ShowMessage()
    {
        // Rich-text markup: [nl/] newline, [i]..[/i] italic, [b]..[/b] bold; named icons.
        this.Host.UI.ShowMessageDialog("A message.[nl/]With [i]simple[/i] [b]formatting[/b].",
            "Message", icon: "Geometry.Cloud");
    }

    protected override async void Initialize()        // called when the view model is created
    {
        base.Initialize();
        this.IsConnected = await Task.Run(() => Tekla.Structures.TeklaStructures.Connect());
        if (this.IsConnected) this.ModelName = new Model().GetInfo().ModelName;
    }
}
```

## XAML — the `ui:` namespace

```xml
<Window xmlns:ui="http://schemas.trimble.com/fusion"
        Style="{DynamicResource Theme.Window.Style}">          <!-- native themed chrome -->
  <Button Command="{ui:CommandHandler ShowMessage}">Click me</Button>   <!-- binds [CommandHandler] -->
  <TextBox  ui:Extensions.Watermark="Try typing here" />               <!-- attached-property watermark -->
  <TextBlock Visibility="{Binding IsConnected, Converter={ui:ConvertBooleanToVisibility}}" />
  <TextBlock Visibility="{Binding IsConnected, Converter={ui:ConvertBooleanToInverseVisibility}}" />
</Window>
```

Verified building blocks: `Theme.Window.Style` (and sibling theme resources), `{ui:CommandHandler Method}`, `{ui:ConvertBooleanToVisibility}` / `…Inverse…`, `ui:Extensions.Watermark`. Apply the theme `Style` to inherit Tekla's window chrome, fonts, and dark/light theme automatically.

## Fusion vs the plugin dialog base

- **Standalone extension / tool** (its own `.exe`, connects via `TeklaStructures.Connect()`): use the full `Fusion.App` host as above. This is what FusionExample is.
- **Plugin dialog** (the properties window for a `PluginBase`): the dialog derives from `PluginWindowBase` (see [tekla-structures-dialog.md](./tekla-structures-dialog.md)); pull in Fusion **theme resources + the `ui:` controls/converters** for native styling and bind to the plugin's `StructuresData` fields. Don't confuse the two hosts — Fusion's `App` is not the plugin dialog base.

## Where this fits the plugin-forge

Phase 3 (generate) emits the dialog as a Fusion-themed WPF window bound to the inferred `StructuresData` params: `WindowViewModel` per parameter group, `[CommandHandler]` for actions, `Theme.Window.Style` + `ui:` controls so the generated dialog is indistinguishable from a native Tekla component dialog. See [[tekla-plugin-forge]] and [tekla-structures-direct-manipulation.md](./tekla-structures-direct-manipulation.md) (the in-model counterpart).

## Source

- Verified code: [FusionExample](https://github.com/TrimbleSolutionsCorporation/FusionExample) — `ExampleApp.cs`, `MainWindowViewModel.cs`, `MainWindow.xaml`.
- Package: [TeklaFusion](https://www.nuget.org/packages/TeklaFusion) (4.1.10 line for tekla 2025; net48 + net6/8/9; Trimble; copyright 1992–2026).
- **Reflected reference** for the dialog-tier Fusion surface (signatures-only — Fusion ships no XML docs) is the sibling agent **`tekla-plugin-sdk-2025`**, generated from the `TeklaFusion` 4.1.10 NuGet. This doc is the conceptual narrative; that agent enumerates the dialog-tier types (App / UI / CommandHandler / Formatting / Data / Features.Panels / Models).
