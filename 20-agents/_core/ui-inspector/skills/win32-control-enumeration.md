---
name: ui-inspector-win32-control-enumeration
description: This skill should be used when implementing the ui-inspector's Win32 adapter — enumerating dialog controls, reading tab labels cross-process, capturing coordinate-aligned screenshots, or debugging missing/under-counted controls. Encodes the battle-tested Win32 technique (EnumChildWindows, TCM_GETITEMW via VirtualAllocEx, PrintWindow PW_CLIENTONLY, tab-switch settle timing) proven on real host dialogs.
---

# Win32 control enumeration

The Win32 adapter for the ui-inspector. Every rule below was learned the hard
way against real host dialogs; ignore one and the overlay is wrong or empty.

## 1. Enumerate the REAL tree with `EnumChildWindows`

High-level accessibility trees (UIAutomation) collapse custom-drawn dialogs into
a handful of opaque "pane" elements — on one real connection dialog, UIA showed
~38 nodes where `EnumChildWindows` revealed ~760 actual child controls. Always
walk the native child-window tree:

```csharp
[DllImport("user32.dll")]
static extern bool EnumChildWindows(IntPtr hWnd, EnumWindowsProc cb, IntPtr p);
delegate bool EnumWindowsProc(IntPtr hWnd, IntPtr lParam);

var controls = new List<IntPtr>();
EnumChildWindows(dialog, (h, _) => { controls.Add(h); return true; }, IntPtr.Zero);
```

For each child read class (`GetClassName`), text (`WM_GETTEXT`, not
`GetWindowText` — the latter fails cross-process for some controls), and rect
(`GetWindowRect` then `MapWindowPoints`/`ScreenToClient` into the dialog's client
space). Normalize the Win32 class to a `type` (`Button`→button, `Edit`→edit,
`ComboBox`→combo, `SysTabControl32`→tab, `Static`→static, …).

**Label association:** a control's own text is its label for buttons/checks;
for edits/combos the label is the nearest preceding `Static` to the left or
above. Match by smallest center-to-center distance among statics on the same row/column.

## 2. Read tab labels CROSS-PROCESS with `TCM_GETITEMW`

`TCM_GETITEM` passes a `TCITEM` struct *by pointer*. The dialog lives in another
process, so a pointer into your address space is meaningless there. You must
allocate the struct **in the target process** with `VirtualAllocEx`, write to it
with `WriteProcessMemory`, send the message, then read it back with
`ReadProcessMemory`:

```csharp
uint pid; GetWindowThreadProcessId(tabCtrl, out pid);
var proc = OpenProcess(PROCESS_VM_OPERATION|PROCESS_VM_READ|PROCESS_VM_WRITE, false, pid);
IntPtr remote = VirtualAllocEx(proc, IntPtr.Zero, size, MEM_COMMIT, PAGE_READWRITE);
// write a TCITEM whose pszText points at remote+sizeof(TCITEM); send TCM_GETITEMW;
// ReadProcessMemory the text buffer back. Free with VirtualFreeEx, CloseHandle.
```

Use the **W** (`TCM_GETITEMW`) variant and a UTF-16 buffer. The ANSI variant
mangles non-ASCII tab names.

## 3. Settle >= 300 ms after switching a tab

After `TCM_SETCURSEL` (or clicking a tab), the dialog re-lays-out its controls
asynchronously. Reading too soon under-counts: at **80 ms a property sheet
under-counted its controls by ~50%**; **>=300 ms** is the floor that counts them
all. Settle in the adapter, not the caller:

```csharp
SendMessage(tabCtrl, TCM_SETCURSEL, idx, 0);
Thread.Sleep(300);   // floor; bump for heavy dialogs
```

## 4. Capture with `PrintWindow` + `PW_CLIENTONLY`

`PrintWindow` renders a window to a DC even when occluded. **`PW_CLIENTONLY` is
not optional** — without it the bitmap includes the title bar / border, which
shifts every pixel down-and-right and makes the overlay rects miss their
controls. Pair it with `PW_RENDERFULLCONTENT` so DirectComposition/owner-drawn
surfaces actually paint:

```csharp
const uint PW_CLIENTONLY = 0x1, PW_RENDERFULLCONTENT = 0x2;
GetClientRect(dialog, out RECT rc);
using var bmp = new Bitmap(rc.right, rc.bottom);
using var g = Graphics.FromImage(bmp);
IntPtr hdc = g.GetHdc();
PrintWindow(dialog, hdc, PW_CLIENTONLY | PW_RENDERFULLCONTENT);
g.ReleaseHdc(hdc);
```

The bitmap's `(0,0)` now equals the client origin — the same origin
`enumerate-controls` reports rects against, so overlay hot-zones align exactly.

## Checklist before shipping a capture

- [ ] `PW_CLIENTONLY` set (no chrome in the image)
- [ ] control rects mapped to **client** coords (not screen)
- [ ] tab read via `TCM_GETITEMW` cross-process (not a local pointer)
- [ ] >=300 ms settle after each tab switch
- [ ] every `VirtualAllocEx` paired with `VirtualFreeEx`; every `OpenProcess` handle closed
