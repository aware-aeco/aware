---
name: ui-inspector-host-adapter-seam
description: This skill should be used when implementing or extending the ui-inspector agent's sidecar — adding a new OS/host adapter (Win32, UIAutomation, macOS AX, Linux AT-SPI), or wiring a command through the host-agnostic surface. Explains why the inspector binds to no single host .exe and how the per-platform adapter seam keeps the five commands identical across hosts.
---

# UI-inspector host-adapter seam

The ui-inspector is **host-agnostic**. The vendor agents (`tekla`, `revit`,
`sketchup`, `rhino`) each bind to one host `.exe` and one managed API. The
inspector binds to **none** — its target is whatever window you hand it, and the
native machinery to read that window sits behind a per-platform *adapter seam*.

This is decalog #5 (AECO is the wedge, not the limit) made concrete: the
field-to-setting mapping problem is identical in a Tekla connection dialog, a
Revit family-parameter dialog, a SketchUp preferences pane, and a non-AECO
desktop app. Build the inspector once; everyone composes it (decalog #6).

## The seam

```
        ┌─────────────────────────────────────────────┐
        │  host-agnostic command surface (5 verbs)     │
        │  enumerate-controls · capture-window ·       │
        │  walk-tabs · map-fields · build-overlay      │
        └───────────────────────┬─────────────────────┘
                                │  IHostAdapter trait
        ┌───────────┬───────────┼───────────┬──────────┐
        │  Win32    │UIAutomation│  macOS AX │ AT-SPI   │
        │ (today)   │ (future)   │ (future)  │ (future) │
        └───────────┴───────────┴───────────┴──────────┘
```

Every command is written ONCE against the adapter interface. Adding a platform
means implementing the interface, not touching the command surface or the agent
manifest. The five commands, their inputs, and their output shapes are frozen by
the manifest — an adapter only has to satisfy them.

## The adapter interface

An adapter resolves a window reference and answers four primitives; the five
commands are composed from these:

| Primitive | Win32 realization | Used by |
|---|---|---|
| `resolve_window(ref) -> handle` | `FindWindow` / enumerate top-level + substring match, or parse `0x` HWND | all |
| `enumerate(handle) -> control[]` | `EnumChildWindows` + per-control class/text/rect | enumerate-controls, walk-tabs |
| `tabs(handle) -> tab[]` + `select_tab(i)` | `TCM_GETITEMW` cross-process read; `PSM_SETCURSEL` / synthesized click (NOT `TCM_SETCURSEL` alone — see win32 skill) + settle | walk-tabs |
| `capture(handle, rect) -> png` | `PrintWindow` PW_CLIENTONLY \| PW_RENDERFULLCONTENT | capture-window, walk-tabs |

`map-fields` and `build-overlay` are **adapter-independent** — they compose
`enumerate` + a preset round-trip (see [sentinel-field-mapping](sentinel-field-mapping.md))
and pure HTML generation respectively. Keep them above the seam so every future
adapter gets them for free.

## Why net9 (not net48)

The host runtime agents must target `net48` because they load the host's
own managed Open API DLLs, which are Framework-only (see the tekla
runtime-bridge skill). The inspector is different: it talks to windows through
**Win32 user32/gdi32 P/Invoke from a separate process** and never loads a host's
managed API. So the sidecar targets `net9.0` and can publish NativeAOT as a
single static binary — consistent with AWARE's "binaries don't decay" stance
(decalog #4).

## Invariants an adapter must hold

1. **Coordinates are client-area, not screen.** Every `rect` an adapter returns
   and every screenshot it captures use the window's client origin, so overlay
   hot-zones line up. Mixing screen and client coordinates shifts the overlay.
2. **Settle before reading a freshly-switched tab** — see
   [win32-control-enumeration](win32-control-enumeration.md). The adapter owns
   the settle, not the caller.
3. **Read-only by default.** Only `map-fields` writes, and only into a *scratch
   copy* of a preset — never the live deliverable. An adapter must not mutate
   host/model state.
4. **Degrade, don't crash.** A control with no label returns `label: ""`, not an
   error. A window that can't be resolved is the one hard failure (exit 1).
