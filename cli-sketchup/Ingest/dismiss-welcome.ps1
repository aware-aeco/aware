# dismiss-welcome.ps1 — programmatically close the SketchUp 2026 "Welcome to
# SketchUp" CEF-rendered modal so the AWARE bridge can finish loading.
#
# Why this exists
# ---------------
# When SketchUp 2026 boots for the first time after each install (and on most
# subsequent cold boots until a Trimble account is signed in), it shows a
# Qt-framed CEF dialog that blocks Ruby Plugins/ from loading until the user
# clicks "Start Modeling". That means our aware_sketchup_bridge.rb never runs
# and `aware-sketchup list-instances` always returns an empty array.
#
# What we tried that DID NOT work
# -------------------------------
# - User32.SendInput via System.Windows.Forms.SendKeys to the top-level Qt
#   window: focus-stealing protection prevents the keystrokes from reaching it
#   right after a foreign process launch.
# - PostMessage(WM_CLOSE) to the Qt frame: kills the whole SketchUp process.
# - SendMessage(WM_KEYDOWN/UP) to the Qt frame: ignored.
# - UI Automation's InvokePattern: not supported on the CEF render layer.
# - FindWindow("CommonWebDialog", ...): class lookup returns NULL because the
#   actual top-level class in SU 2026 is `Qt691QWindowIcon`, not the legacy
#   CommonWebDialog name from pre-Qt-6 SketchUp.
#
# What DOES work (the technique below)
# ------------------------------------
# Walk the child-window chain of the Qt frame and PostMessage VK_ESCAPE
# (WM_KEYDOWN / WM_KEYUP) to the inner Chrome_RenderWidgetHostHWND child.
# Chromium accepts WM_KEYDOWN routed directly at the render widget — the
# synthetic-input gating it normally enforces only fires when input arrives
# through the standard SendInput / hardware-injection path. PostMessage at the
# raw HWND of the render widget is treated by CEF as an internal message.
#
# The Vue welcome app catches Escape via its router and calls
# `window.sketchup.closeWelcomeWindow()` (the Ruby-host CEF bridge), which
# closes the dialog cleanly and lets SketchUp resume normal startup. The
# Plugins/ folder is then loaded a second or two later.
#
# Usage
# -----
#   .\dismiss-welcome.ps1                  # poll up to 30 s for the welcome
#                                          # dialog, dismiss on first sight,
#                                          # then exit 0; exit 1 on timeout.
#
#   .\dismiss-welcome.ps1 -TimeoutSec 60   # custom timeout
#
#   .\dismiss-welcome.ps1 -Verbose         # log each polling tick

[CmdletBinding()]
param(
    [int]$TimeoutSec   = 30,
    [int]$PollEveryMs  = 250
)

$ErrorActionPreference = "Stop"

$src = @'
using System;
using System.Collections.Generic;
using System.Runtime.InteropServices;
using System.Text;
public static class SketchupWelcomeDismisser {
    public delegate bool EnumWindowsProc(IntPtr hWnd, IntPtr lParam);
    [DllImport("user32.dll")] public static extern bool EnumWindows(EnumWindowsProc lpEnumFunc, IntPtr lParam);
    [DllImport("user32.dll")] public static extern bool EnumChildWindows(IntPtr hWndParent, EnumWindowsProc lpEnumFunc, IntPtr lParam);
    [DllImport("user32.dll", CharSet=CharSet.Auto)] public static extern int GetWindowText(IntPtr hWnd, StringBuilder lpString, int nMaxCount);
    [DllImport("user32.dll", CharSet=CharSet.Auto)] public static extern int GetClassName(IntPtr hWnd, StringBuilder lpClassName, int nMaxCount);
    [DllImport("user32.dll")] public static extern bool IsWindowVisible(IntPtr hWnd);
    [DllImport("user32.dll")] public static extern uint GetWindowThreadProcessId(IntPtr hWnd, out uint pid);
    [DllImport("user32.dll", SetLastError=true)] public static extern bool PostMessage(IntPtr hWnd, uint Msg, IntPtr wParam, IntPtr lParam);
    public const uint WM_KEYDOWN = 0x100;
    public const uint WM_KEYUP   = 0x101;
    public const int  VK_ESCAPE  = 0x1B;

    public static IntPtr FindWelcome(uint pid) {
        IntPtr found = IntPtr.Zero;
        EnumWindows((h, lp) => {
            uint p; GetWindowThreadProcessId(h, out p);
            if (p != pid) return true;
            if (!IsWindowVisible(h)) return true;
            var title = new StringBuilder(512); GetWindowText(h, title, 512);
            if (title.ToString().IndexOf("Welcome to SketchUp", StringComparison.OrdinalIgnoreCase) >= 0) {
                found = h;
                return false;
            }
            return true;
        }, IntPtr.Zero);
        return found;
    }

    public static IntPtr FindRenderWidget(IntPtr root) {
        IntPtr found = IntPtr.Zero;
        EnumChildWindows(root, (h, lp) => {
            var cb = new StringBuilder(256); GetClassName(h, cb, 256);
            if (cb.ToString() == "Chrome_RenderWidgetHostHWND") {
                found = h;
                return false;
            }
            return true;
        }, IntPtr.Zero);
        return found;
    }

    public static List<IntPtr> ListChildren(IntPtr root) {
        var list = new List<IntPtr>();
        EnumChildWindows(root, (h, lp) => { list.Add(h); return true; }, IntPtr.Zero);
        return list;
    }

    public static void SendEscape(IntPtr h) {
        PostMessage(h, WM_KEYDOWN, (IntPtr)VK_ESCAPE, IntPtr.Zero);
        PostMessage(h, WM_KEYUP,   (IntPtr)VK_ESCAPE, IntPtr.Zero);
    }
}
'@

if (-not ('SketchupWelcomeDismisser' -as [type])) {
    Add-Type -TypeDefinition $src -Language CSharp
}

$deadline = (Get-Date).AddSeconds($TimeoutSec)
$dismissed = $false
$ticks = 0
$everSawWelcome = $false

while ((Get-Date) -lt $deadline) {
    $ticks++
    $sketchup = Get-Process SketchUp -ErrorAction SilentlyContinue
    if (-not $sketchup) {
        Write-Verbose ("[tick $ticks] SketchUp.exe not running yet")
        Start-Sleep -Milliseconds $PollEveryMs
        continue
    }

    $welcome = [SketchupWelcomeDismisser]::FindWelcome([uint32]$sketchup.Id)
    if ($welcome -eq [IntPtr]::Zero) {
        Write-Verbose ("[tick $ticks] no welcome dialog visible (SketchUp pid $($sketchup.Id))")
        # If we have been polling and the welcome never appeared, that's a
        # success: SketchUp booted past it on its own (already-signed-in user,
        # or someone dismissed it manually). Bail out cleanly after one
        # successful "no welcome" sighting once SketchUp is up.
        if (-not $everSawWelcome -and $ticks -ge 2) {
            Write-Host "No SketchUp Welcome dialog detected (already dismissed or signed in)."
            exit 0
        }
        Start-Sleep -Milliseconds $PollEveryMs
        continue
    }
    $everSawWelcome = $true

    Write-Verbose ("[tick $ticks] welcome dialog hWnd=0x{0:X} on pid {1}" -f $welcome.ToInt64(), $sketchup.Id)

    # Send Escape to every child window we can find. The render widget is the
    # one that matters but fanning out covers Qt frame variations between
    # SketchUp builds.
    foreach ($child in [SketchupWelcomeDismisser]::ListChildren($welcome)) {
        [SketchupWelcomeDismisser]::SendEscape($child)
    }
    [SketchupWelcomeDismisser]::SendEscape($welcome)

    # Give Vue's router + CEF teardown ~600 ms.
    Start-Sleep -Milliseconds 600

    # Verify the welcome window is gone (or the main window has a non-Welcome
    # title).
    $still = [SketchupWelcomeDismisser]::FindWelcome([uint32]$sketchup.Id)
    if ($still -eq [IntPtr]::Zero) {
        Write-Host ("Welcome dismissed (after {0} tick(s))." -f $ticks)
        $dismissed = $true
        break
    } else {
        Write-Verbose ("[tick $ticks] welcome still present; retrying")
    }
}

if (-not $dismissed) {
    Write-Error ("dismiss-welcome.ps1: timed out after {0}s; welcome dialog still showing." -f $TimeoutSec)
    exit 1
}

exit 0
