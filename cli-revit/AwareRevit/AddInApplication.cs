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

    public Result OnStartup(UIControlledApplication app)
    {
        try
        {
            // ExternalEvent.Create wires Revit's main-thread dispatch mechanism;
            // the handler's Execute(UIApplication) receives the live UIApplication
            // from Revit itself when the event fires, so we don't need to capture
            // it ourselves.
            _handler = new ExecuteEventHandler();
            _event = ExternalEvent.Create(_handler);
            _server = new PipeServer(_handler, _event);
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
        try { _server?.Stop(); }
        catch { /* best effort */ }
        return Result.Succeeded;
    }
}
