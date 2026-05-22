// ScriptServerHelper — activates the Rhino Script Server on an already-running
// Rhino instance by sending `_StartScriptServer` + Enter to its command bar via
// SendInput (KEYEVENTF_UNICODE). Called automatically by the `exec` and
// `list-instances` verbs when `rhinocode list` returns an empty array despite
// Rhino.exe processes being present, removing the need for the user to manually
// type `StartScriptServer` when they open Rhino outside of `aware-rhino launch`.

using System.Diagnostics;
using System.Runtime.InteropServices;
using System.Text.Json.Nodes;

namespace AwareRhino;

internal static class ScriptServerHelper
{
    // ── P/Invoke ────────────────────────────────────────────────────────────

    [DllImport("user32.dll")] static extern bool SetForegroundWindow(IntPtr hWnd);

    [DllImport("user32.dll", SetLastError = true)]
    static extern uint SendInput(uint nInputs, INPUT[] pInputs, int cbSize);

    const uint INPUT_KEYBOARD    = 1;
    const uint KEYEVENTF_KEYUP   = 0x0002;
    const uint KEYEVENTF_UNICODE = 0x0004;
    const ushort VK_RETURN       = 0x0D;

    // Use [FieldOffset] union so the compiler can compute the correct size
    // (MOUSEINPUT is the largest member at 28 bytes on x64) without any
    // manually-managed padding fields that would trigger CS0169.
    [StructLayout(LayoutKind.Sequential)]
    struct INPUT
    {
        public uint      type;
        public InputUnion u;
    }

    [StructLayout(LayoutKind.Explicit)]
    struct InputUnion
    {
        // MOUSEINPUT is the largest member of the Windows INPUT union (28 bytes
        // on x64); keeping it here ensures Marshal.SizeOf<INPUT>() matches the
        // Windows ABI. KEYBDINPUT (20 bytes) is what we actually populate.
        [FieldOffset(0)] public KEYBDINPUT ki;
        [FieldOffset(0)] public MOUSEINPUT mi;
    }

    [StructLayout(LayoutKind.Sequential)]
    struct KEYBDINPUT
    {
        public ushort wVk;
        public ushort wScan;
        public uint   dwFlags;
        public uint   time;
        public IntPtr dwExtraInfo;
    }

    // Included solely to make InputUnion the right size via the [FieldOffset] union.
    [StructLayout(LayoutKind.Sequential)]
    struct MOUSEINPUT
    {
        public int    dx;
        public int    dy;
        public uint   mouseData;
        public uint   dwFlags;
        public uint   time;
        public IntPtr dwExtraInfo;
    }

    // ── Public API ──────────────────────────────────────────────────────────

    /// <summary>
    /// If <c>rhinocode list</c> returns an empty array but a <c>Rhino.exe</c>
    /// process is running, activates the Rhino window and sends
    /// <c>_StartScriptServer</c> + Enter to the command bar via SendInput,
    /// then polls <c>rhinocode list</c> until an instance appears or
    /// <paramref name="timeoutMs"/> elapses.
    /// </summary>
    /// <returns>
    /// <c>true</c> if at least one instance is now visible in
    /// <c>rhinocode list</c>; <c>false</c> otherwise.
    /// </returns>
    public static bool TryEnsureScriptServer(RhinocodeClient client, int timeoutMs = 6_000)
    {
        var rhinoProcs = Process.GetProcessesByName("Rhino");
        if (rhinoProcs.Length == 0) return false;

        foreach (var proc in rhinoProcs)
        {
            var hwnd = proc.MainWindowHandle;
            if (hwnd == IntPtr.Zero) continue;

            // Bring the Rhino window to the foreground so SendInput lands in
            // its command bar rather than whatever was previously focused.
            SetForegroundWindow(hwnd);
            Thread.Sleep(350);

            // Type "_StartScriptServer" using Unicode key events (layout-
            // independent: the scan code carries the character, wVk is 0).
            TypeText("_StartScriptServer");

            // Send Enter to confirm the command.
            SendRawInput(new[]
            {
                MakeKey(VK_RETURN, 0, 0),
                MakeKey(VK_RETURN, 0, KEYEVENTF_KEYUP),
            });

            // Poll rhinocode list until an instance appears.
            var deadline = DateTime.UtcNow.AddMilliseconds(timeoutMs);
            while (DateTime.UtcNow < deadline)
            {
                Thread.Sleep(500);
                var (exit, stdout, _) = client.RunListJson();
                if (exit != 0) continue;
                try
                {
                    var arr = JsonNode.Parse(stdout) as JsonArray;
                    if (arr?.Count > 0) return true;
                }
                catch { /* ignore parse errors during startup */ }
            }
        }

        return false;
    }

    // ── Helpers ─────────────────────────────────────────────────────────────

    static void TypeText(string text)
    {
        var inputs = new List<INPUT>(text.Length * 2);
        foreach (char c in text)
        {
            inputs.Add(MakeUnicodeKey(c, 0));
            inputs.Add(MakeUnicodeKey(c, KEYEVENTF_KEYUP));
        }
        SendRawInput(inputs.ToArray());
    }

    static void SendRawInput(INPUT[] inputs) =>
        SendInput((uint)inputs.Length, inputs, Marshal.SizeOf<INPUT>());

    static INPUT MakeUnicodeKey(char c, uint flags) => new INPUT
    {
        type = INPUT_KEYBOARD,
        u    = new InputUnion { ki = new KEYBDINPUT { wVk = 0, wScan = (ushort)c, dwFlags = KEYEVENTF_UNICODE | flags } },
    };

    static INPUT MakeKey(ushort vk, ushort scan, uint flags) => new INPUT
    {
        type = INPUT_KEYBOARD,
        u    = new InputUnion { ki = new KEYBDINPUT { wVk = vk, wScan = scan, dwFlags = flags } },
    };
}
