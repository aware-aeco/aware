// AwareRevit — Revit 2026 add-in for the AWARE runtime. Loaded by Revit at
// startup via AwareRevit.addin (placed in %APPDATA%\Autodesk\Revit\Addins\2026\).
// OnStartup boots a NamedPipeServerStream listener on "aware-revit-<PID>" and
// registers a single IExternalEventHandler for marshaling user code onto
// Revit's main thread.

using Autodesk.Revit.UI;

namespace AwareRevit.AddIn;

public sealed class AddInApplication : IExternalApplication
{
    PipeServer? _server;
    ExecuteEventHandler? _handler;
    ExternalEvent? _event;
    readonly UIApplicationProvider _uiProvider = new();

    public Result OnStartup(UIControlledApplication app)
    {
        try
        {
            // Capture the live UIApplication on the first Idling event — that's
            // when Revit guarantees the object is constructed and ready.
            app.Idling += OnIdling;

            _handler = new ExecuteEventHandler();
            _event = ExternalEvent.Create(_handler);
            _server = new PipeServer(_handler, _event, _uiProvider);
            _server.Start();
            return Result.Succeeded;
        }
        catch (Exception ex)
        {
            TaskDialog.Show("AWARE", $"OnStartup failed: {ex.Message}\n\n{ex.StackTrace}");
            return Result.Failed;
        }
    }

    public Result OnShutdown(UIControlledApplication app)
    {
        try
        {
            app.Idling -= OnIdling;
            _server?.Stop();
        }
        catch { /* best effort */ }
        return Result.Succeeded;
    }

    void OnIdling(object? sender, Autodesk.Revit.UI.Events.IdlingEventArgs e)
    {
        if (sender is UIApplication ui) _uiProvider.Set(ui);
    }
}

/// <summary>Thread-safe holder: the pipe server captures this at boot,
/// and the Idling event populates it later (once Revit has a real UIApplication).</summary>
internal sealed class UIApplicationProvider
{
    UIApplication? _ui;
    readonly object _lock = new();
    public void Set(UIApplication ui)
    {
        lock (_lock) { _ui ??= ui; }
    }
    public UIApplication? Get()
    {
        lock (_lock) { return _ui; }
    }
}
