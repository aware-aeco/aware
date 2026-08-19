//! `viewer-3d.render` — render a generic 3D **scene** into a self-contained,
//! interactive HTML document (builtin transport, #201/#215 pattern).
//!
//! Like `ui` and `html-report`, this is an in-process `_core` builtin: there is no host
//! binary to ship. It takes a **domain-agnostic** scene description (members/lines/boxes/
//! nodes + groups + grids + side panels) and returns a single HTML file that renders the
//! scene in 3D (Three.js) — orbit/zoom, click-to-inspect, a legend and side tables. The
//! producer owns all domain meaning (a steel app maps members→elements + a takeoff panel; a
//! data app maps bars→elements + a totals panel); the renderer knows nothing about steel.
//!
//! Output mirrors `ui.render`/`html-report.render`: `{ html, bytes, output-path? }`, with the
//! `output-path` write gated to a real run. The interactive viewer needs scripts enabled, so
//! a client embeds it in a script-enabled surface (or opens it in a browser) — unlike the
//! static `html-report`, it does not render inside a no-scripts sandbox.
//!
//! Display modes: `solid` / `wire` / `xray` / `realistic`. **Realistic** shades each element from
//! its element-level `material` — the same field the IFC writer resolves, so a scene authored for
//! export shades correctly with no producer change (`meta.material` is accepted as a fallback).
//! The value is semantic: a family name like `"concrete"`, or an alloy grade like
//! `"A992"`/`"A240 316"`. It is shaded against a generated
//! image-based light. The grade→family→appearance mapping is the RENDERER's, not the scene's, so
//! the look can improve without re-baking a scene; unknown or plain-carbon values fall back to
//! `painted`, which keeps the element's group colour so the legend stays readable.
//!
//! Determinism: identical `scene` input → identical HTML bytes (no clock, no environment).
//! Three.js loads from a pinned CDN for v1; full-inline (offline) is a planned follow-on.

use crate::error::AwareError;
use crate::json::type_name as json_type;
use crate::render::geom::{
    cross3, distance3, dot3, length3, normalized3, point_in_polygon, point_segment_distance,
    polygon_edges, polygon_is_simple_nonzero,
};
use crate::render::scene_roll::{member_roll, scene_up};
use serde_json::Value;
use std::collections::{HashMap, HashSet};

/// The renderer shell. `__SCENE_JSON__` is replaced with the serialized scene. Every `{`/`}`
/// here is literal (we substitute with `str::replace`, not `format!`). Proven against the
/// floless reference prototype (one renderer drew both a steel frame and a bar chart).
const TEMPLATE: &str = r##"<!doctype html>
<html lang="en">
<head>
<meta charset="utf-8" />
<meta name="viewport" content="width=device-width, initial-scale=1" />
<link rel="icon" href="data:," />
<title>AWARE · viewer-3d</title>
<style>
  :root{--bg:#0a0f1a;--panel:rgba(15,23,42,.82);--border:#1e293b;--border-2:#334155;--text:#e2e8f0;--muted:#94a3b8;--accent:#60a5fa;--accent-2:#38bdf8;
    /* section-editing tokens: a clip plane, a clip box, the armed-pick ghost. Named here so they are
       part of the documented palette rather than hexes buried in the module. */
    --clip-plane:#3b82f6;--clip-box:#93c5fd;--clip-ghost:#bfdbfe;--danger:rgba(127,29,29,.55)}
  *{box-sizing:border-box} html,body{margin:0;height:100%;overflow:hidden;background:var(--bg);color:var(--text);font-family:ui-sans-serif,system-ui,-apple-system,"Segoe UI",Roboto,sans-serif}
  #app{position:fixed;inset:0} canvas{display:block}
  .panel{position:absolute;background:var(--panel);border:1px solid var(--border-2);border-radius:12px;backdrop-filter:blur(10px);box-shadow:0 10px 30px rgba(0,0,0,.45)}
  #topbar{top:16px;left:16px;right:16px;padding:10px 14px;display:flex;align-items:center;gap:12px}
  #topbar .brand{font-weight:700} #topbar .brand b{color:var(--accent)} #topbar .sub{color:var(--muted);font-size:13px;margin-left:auto}
  #side{top:74px;right:16px;width:320px;padding:16px;max-height:calc(100% - 150px);overflow:auto}
  #side h2{margin:0 0 2px;font-size:14px} #side .note{color:var(--muted);font-size:12px;margin:0 0 12px}
  table{width:100%;border-collapse:collapse;font-size:12.5px;margin-bottom:8px}
  th,td{text-align:left;padding:6px 4px;border-bottom:1px solid var(--border)} th{color:var(--muted);font-weight:600}
  td.num,th.num{text-align:right;font-variant-numeric:tabular-nums}
  .swatch{display:inline-block;width:10px;height:10px;border-radius:2px;margin-right:7px;vertical-align:middle}
  /* Clips + legend share ONE bounded bottom-left column. Two independently positioned panels, each
     with its own calc() height cap, drift into each other on a short viewport — flexbox owns the
     budget instead, and the 220px reserve simply moved here from #legend.objects. */
  #bottomLeft{position:absolute;left:16px;bottom:16px;display:flex;flex-direction:column;gap:8px;max-height:calc(100% - 220px)}
  #bottomLeft>.panel{position:relative}
  /* The LEGACY flat legend (a scene with no descriptor) never had a height cap of its own — only
     #legend.objects did — so as a flex child it would run past the column. Give it the same bounded,
     themed scroll rather than letting it grow. */
  #bottomLeft>#legend:not(.objects){min-height:0;overflow-y:auto;scrollbar-width:thin;scrollbar-color:var(--border-2) transparent}
  #bottomLeft>#legend:not(.objects)::-webkit-scrollbar{width:9px}
  #bottomLeft>#legend:not(.objects)::-webkit-scrollbar-track{background:transparent}
  #bottomLeft>#legend:not(.objects)::-webkit-scrollbar-thumb{background:var(--border-2);border-radius:5px;border:2px solid transparent;background-clip:content-box}
  #legend{padding:12px 14px;font-size:12.5px} #legend .row{display:flex;align-items:center;gap:8px;margin:2px 0}
  #legend .legend-hint{color:var(--muted);font-size:11px;margin:0 0 6px}
  #legend .row{cursor:pointer;user-select:none;border-radius:5px;padding:2px 5px} #legend .row:hover{background:rgba(51,65,85,.5)}
  #legend .row.off{opacity:.4} #legend .row.off .swatch{filter:grayscale(1)}
  /* ---- objects panel (scene.legend) ---- bounded and scrollable; the old list ran the full page
     height on a real model (~35 rows). Header (mode toggle, search, Show all, hint) is FIXED and
     only .obody scrolls, so the search box never scrolls away from the rows it filters. */
  /* min-height:0 at EVERY level of the flex chain — a flex item defaults to min-height:auto and
     refuses to shrink below its content, so without it the first long list pushes the whole column
     past the wrapper's cap no matter what the max-height says. The legend keeps a protected floor
     and the clip list does not: the legend is the primary way around the model, while a handful of
     section cuts can degrade to their own scroll first. */
  #legend.objects{width:248px;display:flex;flex-direction:column;padding:10px;flex:1 1 auto;min-height:140px}
  #legend.objects .obody{overflow-y:auto;overflow-x:hidden;min-height:0;margin:-2px -4px 0;padding:2px 4px 0}
  /* ---- clip list ---- same row family as the objects panel, so the two read as one system.
     Hidden outright when there are no clips (the `no-side` precedent: no content ⇒ no panel), which
     also means it contributes nothing to the flex column and the legend reclaims the space. */
  #clips{width:248px;padding:10px;font-size:12.5px;display:none;flex-direction:column;flex:0 1 auto;min-height:0;max-height:180px}
  #clips.show{display:flex}
  #clips .csec{color:#475569;font-size:10px;letter-spacing:.06em;text-transform:uppercase;margin:0 0 4px;flex:none}
  #clips .chint{color:var(--muted);font-size:10px;margin:4px 0 0;flex:none}
  #clips .cbody{overflow-y:auto;overflow-x:hidden;min-height:0;margin:0 -4px;padding:0 4px}
  #clips .cbody{scrollbar-width:thin;scrollbar-color:var(--border-2) transparent}
  #clips .cbody::-webkit-scrollbar{width:9px}
  #clips .cbody::-webkit-scrollbar-track{background:transparent}
  #clips .cbody::-webkit-scrollbar-thumb{background:var(--border-2);border-radius:5px;border:2px solid transparent;background-clip:content-box}
  #clips .cbody::-webkit-scrollbar-thumb:hover{background:#475569;background-clip:content-box}
  #clips .crow{display:flex;align-items:center;gap:4px;border-radius:5px;padding:1px 2px}
  #clips .crow:hover{background:rgba(51,65,85,.5)}
  #clips .crow.sel{box-shadow:inset 2px 0 0 var(--accent)}
  #clips .crow button{background:transparent;border:1px solid transparent;border-radius:5px;color:var(--text);font:12px system-ui;font-family:inherit;cursor:pointer;padding:0}
  #clips .crow button:focus-visible{outline:2px solid var(--accent);outline-offset:1px}
  #clips .cvis{width:24px;height:24px;flex:none;display:flex;align-items:center;justify-content:center}
  #clips .cswatch{width:11px;height:11px;border-radius:2px;background:var(--sw,#94a3b8);box-shadow:inset 0 0 0 1.6px var(--sw,#94a3b8)}
  #clips .cvis[aria-checked=false] .cswatch{background:transparent}
  #clips .cpick{flex:1;min-width:0;text-align:left;padding:3px 2px;overflow:hidden;text-overflow:ellipsis;white-space:nowrap}
  /* Revealed on hover/focus like the objects panel's isolate control. Neither is the ONLY route to
     its action — Del deletes the selection from anywhere, F2 renames the focused row — so this stays
     discoverability, not a keyboard trap. */
  #clips .cren,#clips .cdel{width:24px;height:24px;flex:none;color:var(--muted);opacity:0}
  #clips .crow:hover .cren,#clips .crow:hover .cdel,#clips .cren:focus-visible,#clips .cdel:focus-visible{opacity:1}
  #clips .cdel:hover{background:var(--danger);color:var(--text)}
  #clips .cedit{flex:1;min-width:0;background:rgba(2,8,23,.6);border:1px solid var(--accent);border-radius:5px;color:var(--text);font:12px system-ui;font-family:inherit;padding:2px 4px;outline:none}
  #clips .cedit[aria-invalid=true]{border-color:rgba(127,29,29,.9)}
  #clips .cerr{color:var(--text);background:var(--danger);border-radius:5px;font-size:11px;margin:2px 0 4px;padding:3px 6px}
  /* Theme the scroll container — a native light scrollbar on a dark panel is exactly the leak the
     house rule calls out. Firefox gets the standard properties, WebKit the pseudo-elements. */
  #legend.objects .obody{scrollbar-width:thin;scrollbar-color:var(--border-2) transparent}
  #legend.objects .obody::-webkit-scrollbar{width:9px}
  #legend.objects .obody::-webkit-scrollbar-track{background:transparent}
  #legend.objects .obody::-webkit-scrollbar-thumb{background:var(--border-2);border-radius:5px;border:2px solid transparent;background-clip:content-box}
  #legend.objects .obody::-webkit-scrollbar-thumb:hover{background:#475569;background-clip:content-box}
  #legend.objects .omode{display:flex;border:1px solid var(--border-2);border-radius:6px;overflow:hidden;height:24px;margin-bottom:6px;flex:none}
  #legend.objects .omode button{flex:1;border:0;background:transparent;color:var(--muted);font-size:11px;cursor:pointer;font-family:inherit}
  #legend.objects .omode button.on{background:var(--accent);color:#06121f;font-weight:600}
  #legend.objects .osearch{display:flex;align-items:center;height:26px;margin-bottom:6px;padding:0 8px;background:rgba(2,8,23,.6);border:1px solid var(--border-2);border-radius:6px;flex:none}
  #legend.objects .osearch:focus-within{border-color:var(--accent)}
  #legend.objects .osearch input{flex:1;min-width:0;background:transparent;border:0;outline:none;color:var(--text);font:12px system-ui;font-family:inherit}
  #legend.objects #legClear{display:none;flex:none;margin-bottom:6px;background:rgba(30,41,59,.6);color:var(--text);border:1px solid var(--border-2);border-radius:6px;padding:4px 8px;font-size:11px;cursor:pointer;font-family:inherit}
  #legend.objects #legClear:hover{border-color:var(--accent)}
  #legend.objects .ohint{color:var(--muted);font-size:10px;margin:0 0 6px;flex:none;white-space:normal}
  #legend.objects .osec{color:#475569;font-size:10px;letter-spacing:.06em;text-transform:uppercase;margin:6px 0 2px}
  #legend.objects .ocat{display:flex;align-items:center;gap:6px;width:100%;background:transparent;border:0;border-radius:5px;padding:3px 4px;color:var(--text);font:12px system-ui;font-family:inherit;cursor:pointer;text-align:left}
  #legend.objects .ocat:hover{background:rgba(51,65,85,.5)}
  #legend.objects .ochev{color:var(--muted);width:10px;flex:none}
  #legend.objects .ocatlabel{flex:1} #legend.objects .ocount{color:var(--muted);font-size:10px;font-variant-numeric:tabular-nums}
  #legend.objects .orow{display:flex;align-items:center;gap:4px;border-radius:5px;padding:1px 2px}
  #legend.objects .orow:hover{background:rgba(51,65,85,.5)}
  #legend.objects .orow.typed{padding-left:16px}
  #legend.objects .orow.sel{box-shadow:inset 2px 0 0 var(--accent)}
  /* Every row control is a REAL button with a ≥24px hit area around an 11px mark — the visibility
     control used to be a 10px span, unreachable by keyboard and a poor touch target. */
  #legend.objects .orow button{background:transparent;border:1px solid transparent;border-radius:5px;color:var(--text);font:12px system-ui;font-family:inherit;cursor:pointer;padding:0}
  #legend.objects .orow button:focus-visible{outline:2px solid var(--accent);outline-offset:1px}
  #legend.objects .ovis{width:24px;height:24px;flex:none;display:flex;align-items:center;justify-content:center}
  #legend.objects .oswatch{width:11px;height:11px;border-radius:2px;background:var(--sw,#94a3b8);box-shadow:inset 0 0 0 1.6px var(--sw,#94a3b8)}
  #legend.objects .ovis[aria-checked=false] .oswatch{background:transparent}
  #legend.objects .ovis[aria-checked=mixed] .oswatch{background:linear-gradient(135deg,var(--sw,#94a3b8) 0 50%,transparent 50% 100%)}
  #legend.objects .opick{flex:1;min-width:0;text-align:left;padding:3px 2px;overflow:hidden;text-overflow:ellipsis;white-space:nowrap}
  #legend.objects .oiso{width:24px;height:24px;flex:none;color:var(--muted);opacity:0}
  #legend.objects .orow:hover .oiso,#legend.objects .oiso:focus-visible{opacity:1}
  #legend.objects .oiso:hover{color:var(--accent)}
  #legend.objects .oempty{color:var(--muted);font-size:11px;padding:6px 2px}
  #toolbar{top:74px;left:16px;padding:7px 9px;display:flex;align-items:center;gap:7px;flex-wrap:wrap;max-width:calc(100% - 492px)}  /* clears the side panel AND the ViewCube now sharing the top row */
  #toolbar .tb-grp{display:flex;gap:4px} #toolbar .tb-sep{width:1px;height:20px;background:var(--border-2);margin:0 2px}
  #toolbar button{background:rgba(30,41,59,.6);color:var(--text);border:1px solid var(--border-2);border-radius:7px;padding:5px 9px;font-size:12px;cursor:pointer;line-height:1}
  #toolbar button:hover{background:rgba(51,65,85,.85);border-color:var(--accent)}
  #toolbar button.on{background:var(--accent);color:#06121f;border-color:var(--accent);font-weight:600}
  #toolbar .tb-menu{position:relative}
  #toolbar .tb-menu>.menu{position:absolute;top:calc(100% + 6px);left:0;min-width:172px;background:rgba(15,23,42,.97);border:1px solid var(--border-2);border-radius:8px;padding:5px;display:none;flex-direction:column;gap:2px;box-shadow:0 12px 32px rgba(0,0,0,.55);z-index:7}
  #toolbar .tb-menu.open>.menu{display:flex}
  /* A dropdown hides the active item until opened, so the state has to show twice: the trigger takes
     the current mode's name, and the item itself takes a ✓. Mirrors the steel editor's mode menu. */
  #toolbar #modes button,#toolbar #proj button{text-align:left;padding-right:26px;position:relative}
  #toolbar #modes button.on::after,#toolbar #proj button.on::after{content:'✓';position:absolute;right:9px;color:var(--accent)}
  /* `.menu button` below strips the accent background that `button.on` sets, which would leave the
     active item's near-black text on a dark menu. Menu items get their own readable active state. */
  #toolbar .menu button.on{background:rgba(96,165,250,.16);color:var(--text);border-color:transparent;font-weight:600}
  /* min-width holds the longest label so relabelling the trigger cannot jitter the rest of the bar. */
  #toolbar #modeBtn{min-width:7.6em;text-align:center}
  #toolbar #projBtn{min-width:5.6em;text-align:center}
  #toolbar .menu button{width:100%;text-align:left;background:transparent;border:1px solid transparent;border-radius:6px;padding:6px 9px}
  #toolbar .menu button:hover{background:rgba(51,65,85,.85);border-color:transparent}
  #toolbar .menu button.danger:hover{background:rgba(127,29,29,.55)}
  /* Checkable menu items (work area). The tick is drawn, not a glyph, so it can't shift the row's
     text when it toggles — the label stays put and only the mark changes. */
  #toolbar .menu button.wtog{display:flex;align-items:center;gap:8px}
  #toolbar .menu button.wtog .mck{width:12px;height:12px;flex:none;border:1px solid var(--border-2);border-radius:3px;position:relative}
  #toolbar .menu button.wtog[aria-checked=true] .mck{background:var(--accent);border-color:var(--accent)}
  #toolbar .menu button.wtog[aria-checked=true] .mck::after{content:'';position:absolute;left:3.5px;top:1px;width:3px;height:6px;border:solid #0a0f1a;border-width:0 2px 2px 0;transform:rotate(45deg)}
  #toolbar .menu hr{border:0;border-top:1px solid var(--border);margin:4px 2px}
  /* Themed tooltip — replaces native title= so no OS-default tooltip leaks the dark theme. */
  #tooltip{position:fixed;z-index:50;background:rgba(15,23,42,.97);border:1px solid var(--border-2);border-radius:6px;padding:5px 8px;font-size:11.5px;line-height:1.35;color:var(--text);pointer-events:none;max-width:260px;box-shadow:0 8px 22px rgba(0,0,0,.5);opacity:0;transition:opacity .12s}
  #tooltip.show{opacity:1}
  #readout{bottom:16px;left:50%;transform:translateX(-50%);padding:10px 16px;font-size:13px;color:var(--muted);white-space:nowrap;max-width:60vw;overflow:hidden;text-overflow:ellipsis}
  #readout b{color:var(--text)} #readout .pill{color:var(--accent)}
  #rubber{position:absolute;border:1px solid var(--accent);background:rgba(96,165,250,.16);pointer-events:none;display:none;z-index:6}
  #viewcube{position:absolute;right:352px;top:74px;width:104px;height:104px;cursor:pointer;z-index:5}  /* top-right, left of the side panel (16+320+16) */
  /* A scene that supplies no `panels` has no side panel to render. Hiding it alone is not enough:
     these three measurements are coupled to its 320px column — the ViewCube is offset past it and
     the toolbar reserves its width — so the column has to be reclaimed together or the viewer keeps
     a gap where the panel used to be. 156 = 16 (edge) + 104 (cube) + 16 (gap) + 16 (toolbar left). */
  body.no-side #side{display:none}
  body.no-side #viewcube{right:16px}
  body.no-side #toolbar{max-width:calc(100% - 156px)}
  /* An embedding host that already titles the model can suppress ours — see the presentation
     message below. Never the default: opened standalone, this is the only model identity there is. */
  body.no-title #sceneName{display:none}
  #viewcube canvas{display:block;filter:drop-shadow(0 6px 14px rgba(0,0,0,.5))}
  /* World-axis triad, bottom-right (Tekla-style). Passive readout — pointer-events:none so it can
     never swallow an orbit gesture; orientation CHANGES stay on the ViewCube. */
  #axestriad{position:absolute;right:16px;bottom:16px;width:92px;height:92px;z-index:5;pointer-events:none}
  #axestriad canvas{display:block;filter:drop-shadow(0 6px 14px rgba(0,0,0,.5))}
</style>
</head>
<body>
<div id="app"></div>
<div id="topbar" class="panel"><div class="brand"><b>AWARE</b> · viewer-3d</div><div class="sub" id="sceneName">—</div></div>
<div id="toolbar" class="panel">
  <!-- Camera: projection + fit -->
  <div class="tb-menu" id="projMenu">
    <button id="projBtn" data-tip="Projection — perspective or orthographic">Persp ▾</button>
    <div class="menu" role="menu" id="proj">
      <button data-proj="persp" class="on" data-tip="Perspective view — natural depth">Persp</button>
      <button data-proj="ortho" data-tip="Orthographic — true scale, no perspective">Ortho</button>
    </div>
  </div>
  <button id="fit" data-tip="Fit all to view (Home)">Fit</button>
  <div class="tb-sep"></div>
  <!-- Display mode -->
  <div class="tb-menu" id="modeMenu">
    <button id="modeBtn" data-tip="Display mode — solid, wireframe, see-through (X-ray) or realistic materials">Solid ▾</button>
    <div class="menu" role="menu" id="modes">
      <button data-mode="solid" class="on" data-tip="Solid shaded model">Solid</button>
      <button data-mode="wire" data-tip="Wireframe — edges only">Wire</button>
      <button data-mode="xray" data-tip="See-through — reveal hidden parts">X-ray</button>
      <button data-mode="realistic" data-tip="Realistic — true construction materials (steel, concrete, timber…)">Realistic</button>
      <button data-mode="shadowed" data-tip="Realistic with cast shadows — more depth, at a cost to frame rate">Shadowed</button>
    </div>
  </div>
  <div class="tb-sep"></div>
  <!-- Section: clip planes/boxes + work area -->
  <div class="tb-grp" id="section">
    <div class="tb-menu" id="clipMenu">
      <button id="clip" data-tip="Clip planes and boxes — section to see inside a connection">Clip ▾</button>
      <div class="menu" role="menu">
        <button data-clip="plane" data-tip="Click a model face to cut the view there (Shift+X)">Add clip plane</button>
        <button data-clip="box" data-tip="Section a box around the selection (or whole model) (Shift+B)">Add clip box</button>
      <button data-clip="draw" data-tip="Draw a clip box: click two floor corners, then pull the height (Shift+D)">Draw clip box…</button>
        <hr>
        <button data-clip="clear" class="danger" data-tip="Remove every clip">Clear all clips</button>
      </div>
    </div>
    <div class="tb-menu" id="workMenu">
      <button id="work" data-tip="Working area — bound the view to a box (Tekla-style)">Work area ▾</button>
      <div class="menu" role="menu">
        <button data-wa="all" data-tip="Bound the work area to the whole model">Set to all objects</button>
        <button data-wa="sel" data-tip="Bound the work area to the current selection">Define from selection</button>
        <hr>
        <button id="waOn" class="wtog" role="menuitemcheckbox" aria-checked="false" data-tip="Show or hide the work-area box"><span class="mck" aria-hidden="true"></span>Show work area</button>
        <button id="waWhole" class="wtog" role="menuitemcheckbox" aria-checked="true" style="display:none" data-tip="When ON, any part that touches the work area is shown in full — nothing gets cut. When OFF, the work area slices parts cleanly at its box faces (a section cut)."><span class="mck" aria-hidden="true"></span>Show whole parts</button>
        <hr>
        <button data-wa="clear" class="danger" data-tip="Remove the work area">Clear work area</button>
      </div>
    </div>
  </div>
</div>
<div id="side" class="panel"><h2 id="sideTitle">—</h2><p class="note" id="sideNote"></p><div id="panels"></div></div>
<div id="bottomLeft"><div id="clips" class="panel"></div><div id="legend" class="panel"></div></div>
<div id="readout" class="panel">Left-drag box-select · right-drag orbit · middle-drag pan · scroll zoom · <b>click an element</b> · Home fits · Alt+Z zooms selection</div>
<div id="rubber"></div>
<div id="viewcube" data-tip="Click a face for that view · right-drag to orbit"></div>
<div id="axestriad"></div>
<script>
  // Loading handshake for an embedding client (generic — a client may listen, or ignore it
  // entirely when opened standalone): a failed CDN/module load or a runtime error posts
  // `viewer-error`; the first successful render posts `viewer-ready`. Registered in a classic
  // script so it catches module-load failures too (those throw before the module body runs).
  (function(){ var post=function(t,m){ try{ parent.postMessage({type:t,message:m||''},'*'); }catch(e){} };
    window.addEventListener('error', function(e){ post('viewer-error',(e&&e.message)||'render error'); });
    window.addEventListener('unhandledrejection', function(e){ post('viewer-error', String((e&&e.reason)||'load error')); });
    window.__viewerPost=post;
    setTimeout(function(){ if(!window.__viewerReady) post('viewer-error','timed out loading 3D libraries'); }, 9000);
    // Presentation, from the embedding host. The SAME frozen document is both iframed by a host
    // that already titles the model and opened standalone from disk, so this cannot be decided when
    // the file is baked — only the surface displaying it knows. Default is always "show": a
    // standalone viewer must never end up with no model identity at all.
    //
    // Deliberately narrow: this toggles presentation only. It cannot supply content, run script, or
    // change behaviour, so accepting it from any origin costs nothing — and anything that is not
    // this exact shape is ignored.
    var applyChrome=function(showTitle){
      var go=function(){ document.body.classList.toggle('no-title', showTitle===false); };
      if(document.body) go(); else document.addEventListener('DOMContentLoaded',go);
    };
    window.addEventListener('message', function(e){
      var d=e&&e.data;
      if(!d || d.type!=='viewer-presentation') return;
      if(typeof d.showTitle==='boolean') applyChrome(d.showTitle);
    });
  })();
</script>
<script type="importmap">
{ "imports": { "three": "https://cdn.jsdelivr.net/npm/three@0.160.0/build/three.module.js", "three/addons/": "https://cdn.jsdelivr.net/npm/three@0.160.0/examples/jsm/" } }
</script>
<script type="module">
import * as THREE from 'three';
import { OrbitControls } from 'three/addons/controls/OrbitControls.js';
import { RoomEnvironment } from 'three/addons/environments/RoomEnvironment.js';

const SCENE = __SCENE_JSON__;
const el=(tag,cls,text)=>{const e=document.createElement(tag);if(cls)e.className=cls;if(text!=null)e.textContent=text;return e;};

const scene=new THREE.Scene(); scene.background=new THREE.Color(0x0a0f1a);
// Two cameras share one position/target so the projection can be toggled live.
const perspCam=new THREE.PerspectiveCamera(50, innerWidth/innerHeight, 0.01, 1e7);
const orthoCam=new THREE.OrthographicCamera(-1,1,1,-1,0.01,1e7);
let camera=perspCam;
const renderer=new THREE.WebGLRenderer({antialias:true}); renderer.setPixelRatio(Math.min(devicePixelRatio,2)); renderer.setSize(innerWidth,innerHeight);
renderer.localClippingEnabled=true; // enable clip planes/boxes + the work area (Tekla-style sectioning) — driven via renderer.clippingPlanes (applyClips)
// Shadows compile in unconditionally, but only the Realistic mode's key light actually casts (see
// applyDisplayMode). Toggling shadowMap.enabled at runtime would force every material to recompile
// on each mode switch; leaving it on and toggling castShadow costs nothing when nothing casts.
renderer.shadowMap.enabled=false; renderer.shadowMap.type=THREE.PCFSoftShadowMap;   // switched on only by the `shadowed` mode
document.getElementById('app').appendChild(renderer.domElement);
const controls=new OrbitControls(camera, renderer.domElement);
// CAD feel (parity with floless steel-3d-view): rotate/pan stop dead on release — NO post-release
// inertia/drift; the wheel zooms toward the cursor; the orbit pivot re-centres under the cursor
// on every gesture (repivotToCursor below). zoomSpeed bumped so the wheel reaches a detail faster.
controls.enableDamping=false; controls.zoomSpeed=1.3; controls.zoomToCursor=true;
// CAD-style mouse map (#258): LEFT = box/area multi-select (handled below — NOT orbit),
// RIGHT-drag orbits, MIDDLE-drag pans; wheel (incl. ctrl+wheel) zooms. LEFT:-1 disables
// OrbitControls' left handling so the left button is free for picking + rubber-band select.
controls.mouseButtons={ LEFT:-1, MIDDLE:THREE.MOUSE.PAN, RIGHT:THREE.MOUSE.ROTATE };
// CAD re-pivot: on every orbit/zoom/pan START (OrbitControls fires 'start' for all three, incl. the
// wheel, BEFORE applying the gesture) move the orbit target to the DEPTH of whatever visible element
// is under the cursor, kept ON the view axis so the view never jumps — only the pivot depth changes.
// That makes the wheel converge on a detail in a few ticks and orbit/pan scale to it, instead of
// pivoting around a stale framed centre. Empty space → keep the current pivot (Fit/Home re-centre).
let lastHoverXY=null;
const onWheelHover=e=>{ lastHoverXY=[e.clientX,e.clientY]; }; // capture-phase → fresh cursor before OrbitControls' wheel handler
function repivotToCursor(){
  if(!lastHoverXY) return;
  const ndc=new THREE.Vector2((lastHoverXY[0]/innerWidth)*2-1, -(lastHoverXY[1]/innerHeight)*2+1);
  ray.setFromCamera(ndc,camera);
  const hit=ray.intersectObjects(pickable.filter(m=>m.visible),false)[0]; if(!hit) return;
  const fwd=camera.getWorldDirection(new THREE.Vector3());
  const depth=hit.point.clone().sub(camera.position).dot(fwd);   // hit distance along the view axis
  if(depth>1e-3) controls.target.copy(camera.position).addScaledVector(fwd,depth);
}
controls.addEventListener('start', repivotToCursor);
// The clip draw caches candidate projections; a stale one does not merely cost time, it snaps to the
// wrong place. Camera motion is the obvious invalidator — resize, projection switching and ortho
// reframing are the ones easy to miss, and each is wired at its own site below.
controls.addEventListener('change', invalidateClipProjectionCache);
renderer.domElement.addEventListener('wheel', onWheelHover, {capture:true, passive:true});
renderer.domElement.addEventListener('pointermove', e=>{ lastHoverXY=[e.clientX,e.clientY]; }); // track the cursor for the gesture-start re-pivot on orbit/pan too (not just wheel) — parity with floless
let content=new THREE.Group(); scene.add(content); let pickable=[];
const conv=(P,up)=> up==='z' ? new THREE.Vector3(P[0],P[2],P[1]) : new THREE.Vector3(P[0],P[1],P[2]);

// ---- view state: scene bounds + display/visibility, driven by the toolbar + legend ----
let sceneBox=new THREE.Box3(); let maxDim=1;
const groupHidden=new Set(); let soloGroup=null; let displayMode='solid'; let legendClickT=null;
// ---- objects panel (scene.legend) ------------------------------------------------------------
// Two worlds live here. WITHOUT a descriptor the legacy flat list runs on group-level state
// (groupHidden/soloGroup) and a row click hides — unchanged for every scene already in the wild.
// WITH one, everything is keyed on TARGET IDS, because a descriptor row can be a SUBSET of a group
// (the same profile under Beams and under Columns) and group-level state cannot express that.
let opRenderables=[];                 // rendered weld operations — legend-operable, never canvas-picked
let groupColor={};                    // group key → colour, for the panel's row swatches
let targetOf=new Map();               // targetId → [Object3D,…]
let LEG=null;                         // the validated descriptor (Rust drops an unusable one)
let legModeIx=0, legQuery='', legRows=[];
const legCollapsed=new Set();
let selIds=new Set(), hiddenIds=new Set(), isolatedIds=null, legAnchor=null;
const legActive=()=>!!LEG;

/** Every rendered target, in scene order: elements, then weld operations. */
function buildTargetRegistry(){ targetOf=new Map();
  for(const m of pickable){ const id=m.userData&&m.userData.id; if(id) targetOf.set(id,[m]); }
  for(const w of opRenderables){ const id=w.userData&&w.userData.id; if(id) targetOf.set(id,[w]); }
}
/** Resolve the active mode's rows to concrete target-id sets — once, not per interaction. */
function resolveLegRows(){ legRows=[];
  if(!legActive()) return;
  const mode=LEG.modes[legModeIx]; if(!mode) return;
  const byGroup=new Map();
  for(const [id,objs] of targetOf){ const g=objs[0]&&objs[0].userData&&objs[0].userData.group;
    if(g){ if(!byGroup.has(g)) byGroup.set(g,[]); byGroup.get(g).push(id); } }
  for(const sec of (mode.sections||[])) for(const cat of (sec.categories||[])) for(const row of (cat.rows||[])){
    const ids = Array.isArray(row.targets) && row.targets.length
      ? row.targets.filter(id=>targetOf.has(id))
      : (row.groups||[]).flatMap(g=>byGroup.get(g)||[]);
    if(!ids.length) continue;                                   // a row controlling nothing is not drawn
    legRows.push({ key:row.key, label:row.label, color:row.color||null, ids,
      secKey:sec.key, secLabel:sec.label||null, catKey:cat.key||'', catLabel:cat.label||null });
  }
}
/** The selection as OBJECTS — the single path every existing consumer (Alt+Z, clip box, work area,
 *  readout, highlighting) reads, so a legend-driven selection behaves like a canvas one. */
function selectedObjects(){ const out=[]; for(const id of selIds){ const objs=targetOf.get(id); if(objs) out.push(...objs); } return out; }
function syncSelectionFromIds(){ setSelection(selectedObjects()); refreshLegend(); }   // setSelection re-derives the same ids — no loop
/** A row is on/off/mixed — mixed is real once hiding is per-id, which is why the control is a
 *  tri-state checkbox rather than a switch. */
function rowVis(row){ let shown=0; for(const id of row.ids) if(!hiddenIds.has(id) && (!isolatedIds||isolatedIds.has(id))) shown++;
  return shown===0?'false':(shown===row.ids.length?'true':'mixed'); }

// Recompute the orthographic frustum so its on-screen scale matches the perspective
// camera's at the target plane (keeps zoom continuous across a projection toggle / resize).
function reframeOrtho(){
  const dist=camera.position.distanceTo(controls.target)||maxDim;
  const h=Math.tan(THREE.MathUtils.degToRad(perspCam.fov)*0.5)*dist, aspect=innerWidth/innerHeight||1;
  orthoCam.left=-h*aspect; orthoCam.right=h*aspect; orthoCam.top=h; orthoCam.bottom=-h; orthoCam.updateProjectionMatrix();
  invalidateClipProjectionCache();   // frustum + zoom changed without the camera moving
}
// Frame `box` from direction `dir` (target→camera); `dir` omitted keeps the current view angle.
function frameBox(box, dir){
  if(!box || box.isEmpty()) return;
  const c=box.getCenter(new THREE.Vector3()), sz=box.getSize(new THREE.Vector3());
  const radius=Math.max(sz.length()*0.5, maxDim*0.02)||1;
  let v = dir ? dir.clone() : camera.position.clone().sub(controls.target);
  if(v.lengthSq()<1e-9) v=new THREE.Vector3(1,0.8,1);
  v.normalize();
  const dist=radius/Math.sin(THREE.MathUtils.degToRad(perspCam.fov)*0.5)*1.1;
  controls.target.copy(c);
  const pos=c.clone().add(v.multiplyScalar(dist));
  perspCam.position.copy(pos); orthoCam.position.copy(pos);
  const near=Math.max(dist/1000, maxDim/1000), far=dist*10+radius*6;
  perspCam.near=near; perspCam.far=far; perspCam.updateProjectionMatrix();
  orthoCam.near=near; orthoCam.far=far;
  // A fit resets the ortho dolly-zoom (wheel zoom multiplies orthoCam.zoom, which the
  // distance-based frustum below does not account for) so Fit/Home/views truly re-frame.
  orthoCam.zoom=1;
  if(camera.isOrthographicCamera) reframeOrtho();
  controls.update();
}
// Named views (Top/Bottom/Front/Back/Left/Right/Iso), driven by the toolbar buttons AND the
// interactive ViewCube below (a clicked cube face calls applyView with the matching name).
const VIEWS={ top:[0,1,1e-4], bottom:[0,-1,1e-4], front:[0,0,1], back:[0,0,-1], right:[1,0,0], left:[-1,0,0], iso:[1,0.8,1] };
function applyView(name){ const d=VIEWS[name]; if(d) frameBox(sceneBox, new THREE.Vector3(d[0],d[1],d[2])); }

function setProjection(mode){
  const target=controls.target.clone(), pos=camera.position.clone();
  camera = mode==='ortho' ? orthoCam : perspCam;
  controls.object=camera; camera.up.set(0,1,0); camera.position.copy(pos); camera.lookAt(target);
  if(camera.isOrthographicCamera){ orthoCam.zoom=1; reframeOrtho(); } else camera.updateProjectionMatrix();
  controls.update(); invalidateClipProjectionCache();   // the camera OBJECT was swapped
  activate('#proj button','data-proj',mode);
  const pb=document.getElementById('projBtn');
  if(pb) pb.textContent=(mode==='ortho'?'Ortho':'Persp')+' ▾';
}
function setDisplayMode(m){ displayMode=m; applyDisplayMode(); activate('#modes button','data-mode',m);
  // The trigger carries the label because the menu is closed most of the time. min-width above holds
  // the longest one ("Realistic ▾") so switching modes cannot jitter the rest of the toolbar.
  const b=document.getElementById('modeBtn');
  if(b) b.textContent=(m==='wire'?'Wire':m==='xray'?'X-ray':m==='realistic'?'Realistic':m==='shadowed'?'Shadowed':'Solid')+' ▾'; }
// Realistic metal is nothing but reflections: against the default light rig, with nothing to
// reflect, metalness=1 renders BLACK. So the mode generates an image-based light in the browser
// (RoomEnvironment → PMREM) and switches on filmic tone mapping. Built lazily on first use and
// cached; cleared with the mode so Solid/Wire/X-ray look exactly as they always did. Generated
// from fixed code — no asset fetch, no clock, no randomness — so the DOCUMENT stays byte-identical
// for an identical scene (the determinism guarantee is about the HTML, not the pixels).
// Generated once on first use and cached (measured ~1.9s on a software rasterizer with no GPU,
// well under 300ms on real hardware; every later switch is free). Deliberately NOT pre-generated
// at load: a viewer that never leaves Solid should not pay for it.
let envRT=null, envFailed=false;
let lightHemi=null, lightKey=null, lightFill=null, shadowGround=null;
// Shadows are a VARIANT of Realistic, not an orthogonal toggle: they need its environment and its
// solid surfaces to mean anything, and they cost real frame time — so `shadowed` is its own mode and
// plain `realistic` stays the cheap default. Tracked here only to avoid recompiling every material on
// each mode switch: changing shadowMap.enabled alters the shader defines, so it must be done once,
// when the value actually flips, rather than on every applyDisplayMode pass.
let shadowsEnabled=false;

// Sized from the COMPLETED meshes, not from maxDim: the scene bounds are centrelines, and omit node
// `size` and member section extents, so geometry can extend well past them. A frustum built from the
// centrelines then clips most or all of the shadows — a single node with size 100 in a scene whose
// maxDim is 1 loses them outright.
function sizeShadowRig(){
  if(!lightKey) return;
  const rb=new THREE.Box3();
  for(const m of pickable) rb.expandByObject(m);
  if(rb.isEmpty()) return;
  const c=rb.getCenter(new THREE.Vector3()), sz=rb.getSize(new THREE.Vector3());
  const span=Math.max(sz.x,sz.y,sz.z)||1;
  lightKey.shadow.mapSize.set(2048,2048);
  const sc=lightKey.shadow.camera, r=span*1.2;
  sc.left=-r; sc.right=r; sc.top=r; sc.bottom=-r; sc.near=span*0.02; sc.far=span*10; sc.updateProjectionMatrix();
  // Steel is thin, so acne and peter-panning are both easy to hit; a span-scaled normalBias handles
  // the thin-web case a constant one cannot.
  lightKey.shadow.bias=-0.0004; lightKey.shadow.normalBias=Math.max(span*0.0015,1);
  lightKey.position.copy(c).add(new THREE.Vector3(span,span*1.5,span*0.6));
  lightKey.target.position.copy(c); content.add(lightKey.target);
  if(shadowGround){ content.remove(shadowGround); shadowGround.geometry.dispose(); shadowGround.material.dispose(); }
  // ShadowMaterial catches the shadow while staying invisible, so the background shows through
  // everywhere the shadow is not.
  shadowGround=new THREE.Mesh(new THREE.PlaneGeometry(span*4,span*4), new THREE.ShadowMaterial({opacity:0.42}));
  shadowGround.rotation.x=-Math.PI/2; shadowGround.position.set(c.x, rb.min.y, c.z);
  shadowGround.receiveShadow=true; shadowGround.visible=shadowsEnabled; content.add(shadowGround);
}
function syncShadows(want){
  // Applied every pass, not just on change: setEnvironment used to set these from the PREVIOUS flag,
  // so the first switch into `shadowed` reported shadows on and rendered none. A rebuild also makes
  // fresh lights, which would otherwise keep the default castShadow=false while the flag said true.
  if(lightKey) lightKey.castShadow=want;      // the LIGHT casts; which MESHES cast is decided per-pass below
  if(shadowGround) shadowGround.visible=want;
  if(shadowsEnabled===want) return;
  // Only the recompile is gated: changing shadowMap.enabled alters the shader defines.
  shadowsEnabled=want;
  renderer.shadowMap.enabled=want;
  for(const m of pickable) if(m.material) m.material.needsUpdate=true;
}   // dimmed in Realistic; the environment lights it
// Returns whether the environment is actually LIVE. If PMREM fails (lost context, a driver
// without the float targets it needs) we must NOT go on to apply metalness=1 — with nothing to
// reflect, that renders every metal element BLACK, which is far worse than not switching at all.
// Reporting false makes applyDisplayMode keep the flat shading, so the failure degrades instead
// of producing a broken-looking model.
function setEnvironment(on){
  if(on&&!envRT&&!envFailed){
    try{ const pmrem=new THREE.PMREMGenerator(renderer);
      envRT=pmrem.fromScene(new RoomEnvironment(), 0.04); pmrem.dispose(); }
    catch(err){ envFailed=true;
      if(window.__viewerPost) window.__viewerPost('viewer-error','realistic materials unavailable: '+err); }
  }
  const live = on && !!envRT;
  scene.environment = live ? envRT.texture : null;
  // An environment map IS a light source. The rig above was tuned for a scene with NO environment, so
  // leaving it at full strength while the env is on lights everything about twice over — which is why
  // mid-grey paint clipped to white. The hemisphere goes nearly to zero (the env supplies
  // omnidirectional light, and better, since it has direction and so shows form) and the
  // directionals drop to a shaping role.
  if(lightHemi) lightHemi.intensity = live ? 0.08 : 0.95;
  if(lightKey) lightKey.intensity = live ? 0.55 : 1.3;
  if(lightFill) lightFill.intensity = live ? 0.18 : 0.5;
  renderer.toneMapping = live ? THREE.ACESFilmicToneMapping : THREE.NoToneMapping;
  renderer.toneMappingExposure = live ? 1.15 : 1;
  return live;
}
// solid → honour each material's base opacity; wire → wireframe; xray → translucent, no depth
// write; realistic → per-family PBR (MATERIALS) lit by the generated environment.
function applyDisplayMode(){
  // Gate the PBR pass on the environment being LIVE, not merely on the mode being selected.
  // Both realistic modes get the environment and the material table; only `shadowed` also casts.
  const realistic=setEnvironment(displayMode==='realistic'||displayMode==='shadowed');
  syncShadows(realistic && displayMode==='shadowed');
  for(const mesh of pickable){ const mat=mesh.material; if(!mat) continue;
    const u=mat.userData||{}, base=(u.baseOpacity!=null)?u.baseOpacity:1;
    const real=realistic ? (MATERIALS[u.family]||MATERIALS.painted) : null;
    // Reset the appearance on EVERY pass so a mode switch is symmetric — leaving Realistic must
    // restore the flat shading exactly, not strand the last family's metalness on the mesh.
    mat.metalness = real ? real.metalness : 0.5;
    // A metal has NO diffuse term — the environment IS its brightness — and RoomEnvironment is a dim
    // little box, so an unlifted metalness=1 surface reads darker than the matte paint beside it.
    // Lift the metals; leave dielectrics (concrete, timber, paint) alone or they blow out. Done per
    // material, not via scene.environmentIntensity, which does not exist in the pinned three r160.
    mat.envMapIntensity = real ? (1 + real.metalness * 1.6) : 1;
    // Surface detail only in Realistic — Solid/Wire/X-ray stay flat, which is what keeps them
    // readable as working views. Binding a map changes the shader defines, so needsUpdate matters.
    const tex = real ? surfaceFor(u.family) : null;
    // With a map bound the roughness is ENTIRELY in the texture (absolute values), so the scalar must
    // be 1 or three multiplies it in twice. Without one, fall back to the declared/base scalar.
    mat.roughness = tex ? 1 : (real ? real.roughness : 0.5);
    const wantMap = tex ? tex.map : null;
    if(mat.map !== wantMap){ mat.map = wantMap; mat.roughnessMap = tex ? tex.roughnessMap : null; mat.needsUpdate = true; }
    // `.set` NOT `.setHex`: a group colour arrives from the scene as a CSS string ("#60a5fa")
    // while the family colours here are numeric literals — setHex takes only a number, so a
    // string silently becomes NaN and the element renders black.
    if(u.baseColor!=null) mat.color.set(real&&real.color!=null ? real.color : u.baseColor);
    if(displayMode==='wire'){ mat.wireframe=true; mat.transparent=false; mat.opacity=1; mat.depthWrite=true; }
    else if(displayMode==='xray'){ mat.wireframe=false; mat.transparent=true; mat.opacity=Math.min(base,0.25); mat.depthWrite=false; }
    else { const op = real&&real.opacity!=null ? Math.min(base,real.opacity) : base;
      mat.wireframe=false; mat.opacity=op; mat.transparent=op<1; mat.depthWrite=true; }
    // Casting is decided HERE, not once at build time, because it depends on the opacity this pass
    // just set. three's shadow depth material has no notion of ordinary transparency, so glass and
    // any translucent element would otherwise cast as solidly as steel.
    mesh.castShadow = shadowsEnabled && mat.opacity >= 0.95;
    mat.needsUpdate=true;
  }
  syncClipMirror();   // shadows just turned on or off — the mirror follows
}
function applyGroupVisibility(){
  // In "show whole parts" mode the work area filters by WHOLE meshes: anything whose bounds touch
  // the box is drawn in full, anything outside it is dropped. That is what makes the mode
  // slice-free — the alternative (cut mode) contributes clipping planes in applyClips instead.
  const waWhole = workArea && workArea.enabled && workArea.whole ? workArea.box : null;
  const hit=new THREE.Box3();
  // With a descriptor, visibility is per TARGET; without one it stays per group. Welds are only
  // ever addressable through the descriptor — the legacy list has no row for them.
  const shown=(id,group)=>legActive()
    ? (!hiddenIds.has(id) && (!isolatedIds || isolatedIds.has(id)))
    : (!groupHidden.has(group) && (soloGroup===null || soloGroup===group));
  for(const m of pickable){ const u=m.userData||{};
    let vis = shown(u.id, u.group);
    if(vis && waWhole){ hit.setFromObject(m); vis = hit.intersectsBox(waWhole); }
    m.visible = vis; }
  for(const w of opRenderables){ const u=w.userData||{};
    let vis = legActive() ? shown(u.id, u.group) : true;   // legacy: welds were never hideable
    if(vis && waWhole){ hit.setFromObject(w); vis = hit.intersectsBox(waWhole); }
    w.visible = vis; }
}
function toggleGroup(k){ if(groupHidden.has(k)) groupHidden.delete(k); else groupHidden.add(k); soloGroup=null; applyGroupVisibility(); refreshLegend(); }
function soloToggle(k){ soloGroup = soloGroup===k ? null : k; if(soloGroup) groupHidden.clear(); applyGroupVisibility(); refreshLegend(); }
function refreshLegend(){
  if(legActive()){ refreshObjectsPanel(); return; }
  document.querySelectorAll('#legend .row').forEach(r=>{ const k=r.dataset.key;
  r.classList.toggle('off', groupHidden.has(k) || (soloGroup!==null && soloGroup!==k)); }); }
/** Repaint only the live bits — selection ring, tri-state box, the Show-all/Exit control — so
 *  typing in the search box never rebuilds the node the caret lives in. */
function refreshObjectsPanel(){
  const byKey=new Map(legRows.map(r=>[r.key,r]));
  document.querySelectorAll('#legend .orow').forEach(node=>{
    const row=byKey.get(node.dataset.key); if(!row) return;
    const sel=row.ids.every(id=>selIds.has(id));
    node.classList.toggle('sel',sel);
    const pick=node.querySelector('[data-act=pick]'); if(pick) pick.setAttribute('aria-pressed',sel?'true':'false');
    const box=node.querySelector('[data-act=vis]'); if(box) box.setAttribute('aria-checked',rowVis(row));
  });
  const clear=document.getElementById('legClear');
  if(clear){ const any=isolatedIds||hiddenIds.size;
    clear.style.display=any?'block':'none';
    clear.textContent=isolatedIds?'Exit isolation':'Show all'; }
}
function activate(sel,attr,val){ document.querySelectorAll(sel).forEach(b=>b.classList.toggle('on', b.getAttribute(attr)===val)); }

function clearContent(){ scene.remove(content);
  content.traverse(o=>{ if(o.geometry)o.geometry.dispose(); if(o.material)o.material.dispose(); });
  content=new THREE.Group(); scene.add(content); pickable=[]; opRenderables=[]; targetOf=new Map(); }

function makeLabel(text,pos,maxDim){
  const c=document.createElement('canvas'); c.width=128; c.height=64; const g=c.getContext('2d');
  g.fillStyle='#60a5fa'; g.font='bold 40px ui-sans-serif,system-ui,sans-serif'; g.textAlign='center'; g.textBaseline='middle'; g.fillText(text,64,34);
  const sp=new THREE.Sprite(new THREE.SpriteMaterial({map:new THREE.CanvasTexture(c),transparent:true}));
  sp.scale.set(maxDim*0.09, maxDim*0.045, 1); sp.position.copy(pos); return sp;
}

// ---- structural cross-section profiles (extruded) ----
// Canonical `xsection` wins and uses the same sharp-corner dimensions as IFC.
// Legacy `section` + profile-name inference remains for older producers.
function shapeOf(e){
  const xs=e&&e.xsection, xshape=xs&&String(xs.shape||'').toLowerCase();
  if(xshape==='i') return 'I'; if(xshape==='channel') return 'C'; if(xshape==='angle') return 'L'; if(xshape==='tee') return 'T';
  if(xshape==='double-angle') return 'DOUBLE_ANGLE';
  if(xshape==='rhs'||xshape==='chs') return xshape.toUpperCase(); if(xshape==='rect') return 'BOX';
  const p=((e.section&&e.section.shape)||(e.meta&&e.meta.profile)||'').toString().toUpperCase().trim();
  if(/^(W|M|S|HP|UC|UB|UKC|UKB|IPE|HE)/.test(p)) return 'I';
  if(/^(C|MC|PFC)/.test(p)) return 'C';
  if(/^L/.test(p)) return 'L';
  if(/^(HSS|PIPE|TS|SHS|RHS|CHS|TUBE|HSQ)/.test(p)) return 'TUBE';
  return 'BOX';
}
function sectionSpec(e,w,d){ const xs=e&&e.xsection, shape=xs&&String(xs.shape||'').toLowerCase();
  const pos=(...v)=>v.every(n=>typeof n==='number'&&Number.isFinite(n)&&n>0);
  if(shape==='i'&&pos(xs.d,xs.bf,xs.tw,xs.tf)&&xs.tw<xs.bf&&2*xs.tf<xs.d)return {kind:'I',w:xs.bf,d:xs.d,tw:xs.tw,tf:xs.tf};
  if(shape==='channel'&&pos(xs.d,xs.bf,xs.tw,xs.tf)&&xs.tw<xs.bf&&2*xs.tf<xs.d)return {kind:'C',w:xs.bf,d:xs.d,tw:xs.tw,tf:xs.tf};
  if(shape==='angle'&&pos(xs.d,xs.b,xs.t)&&xs.t<Math.min(xs.d,xs.b))return {kind:'L',w:xs.b,d:xs.d,t:xs.t};
  if(shape==='rhs'&&pos(xs.d,xs.b,xs.t)&&2*xs.t<Math.min(xs.d,xs.b))return {kind:'RHS',w:xs.b,d:xs.d,t:xs.t};
  if(shape==='chs'&&pos(xs.od,xs.t)&&2*xs.t<xs.od)return {kind:'CHS',w:xs.od,d:xs.od,t:xs.t};
  if(shape==='rect'&&pos(xs.w,xs.d))return {kind:'BOX',w:xs.w,d:xs.d};
  if(shape==='tee'&&pos(xs.d,xs.bf,xs.tw,xs.tf)&&xs.tw<xs.bf&&xs.tf<xs.d)return {kind:'T',w:xs.bf,d:xs.d,tw:xs.tw,tf:xs.tf};
  if(shape==='double-angle'&&pos(xs.d,xs.b,xs.t)&&typeof xs.gap==='number'&&Number.isFinite(xs.gap)&&xs.gap>=0&&xs.t<Math.min(xs.d,xs.b)&&(xs.orientation==='llbb'||xs.orientation==='slbb'))return xs.orientation==='llbb'?{kind:'DA_LLBB',w:2*xs.b+xs.gap,d:xs.d,t:xs.t,gap:xs.gap}:{kind:'DA_SLBB',w:2*xs.d+xs.gap,d:xs.b,t:xs.t,gap:xs.gap};
  return {kind:shapeOf(e),w,d}; }
function profileShape(spec){ const kind=spec.kind,w=spec.w,d=spec.d;
  const s=new THREE.Shape(), hw=w/2, hd=d/2;
  if(kind==='I'){ const tf=spec.tf||Math.min(d*0.5,Math.max(d*0.10,6)), tw=spec.tw||Math.min(w*0.5,Math.max(w*0.10,5));
    s.moveTo(-hw,-hd); s.lineTo(hw,-hd); s.lineTo(hw,-hd+tf); s.lineTo(tw/2,-hd+tf);
    s.lineTo(tw/2,hd-tf); s.lineTo(hw,hd-tf); s.lineTo(hw,hd); s.lineTo(-hw,hd);
    s.lineTo(-hw,hd-tf); s.lineTo(-tw/2,hd-tf); s.lineTo(-tw/2,-hd+tf); s.lineTo(-hw,-hd+tf); s.closePath();
  } else if(kind==='C'){ const tf=spec.tf||Math.max(d*0.10,5), tw=spec.tw||Math.max(w*0.12,5);
    s.moveTo(-hw,-hd); s.lineTo(hw,-hd); s.lineTo(hw,-hd+tf); s.lineTo(-hw+tw,-hd+tf);
    s.lineTo(-hw+tw,hd-tf); s.lineTo(hw,hd-tf); s.lineTo(hw,hd); s.lineTo(-hw,hd); s.closePath();
  } else if(kind==='L'){ const t=spec.t||Math.max(Math.min(w,d)*0.18,5);
    s.moveTo(-hw,-hd); s.lineTo(hw,-hd); s.lineTo(hw,-hd+t); s.lineTo(-hw+t,-hd+t); s.lineTo(-hw+t,hd); s.lineTo(-hw,hd); s.closePath();
  } else if(kind==='T'){ const tf=spec.tf||Math.max(d*0.10,5),tw=spec.tw||Math.max(w*0.12,5);
    s.moveTo(-tw/2,-hd); s.lineTo(tw/2,-hd); s.lineTo(tw/2,hd-tf); s.lineTo(hw,hd-tf);
    s.lineTo(hw,hd); s.lineTo(-hw,hd); s.lineTo(-hw,hd-tf); s.lineTo(-tw/2,hd-tf); s.closePath();
  } else if(kind==='DA_LLBB'||kind==='DA_SLBB'){ const t=spec.t,g=spec.gap,a=new THREE.Shape(),b=new THREE.Shape();
    a.moveTo(-g/2-t,-hd); a.lineTo(-g/2,-hd); a.lineTo(-g/2,hd); a.lineTo(-g/2-t,hd); a.lineTo(-g/2-t,-hd+t); a.lineTo(-hw,-hd+t); a.lineTo(-hw,-hd); a.closePath();
    b.moveTo(g/2,-hd); b.lineTo(g/2+t,-hd); b.lineTo(g/2+t,hd); b.lineTo(g/2,hd); b.lineTo(g/2,-hd+t); b.lineTo(hw,-hd+t); b.lineTo(hw,-hd); b.closePath(); return [a,b];
  } else if(kind==='TUBE'||kind==='RHS'){ const t=spec.t||Math.max(Math.min(w,d)*0.12,4);
    s.moveTo(-hw,-hd); s.lineTo(hw,-hd); s.lineTo(hw,hd); s.lineTo(-hw,hd); s.closePath();
    const h=new THREE.Path(); h.moveTo(-hw+t,-hd+t); h.lineTo(hw-t,-hd+t); h.lineTo(hw-t,hd-t); h.lineTo(-hw+t,hd-t); h.closePath(); s.holes.push(h);
  } else if(kind==='CHS'){ const outer=w/2,inner=outer-spec.t;s.absarc(0,0,outer,0,Math.PI*2,false);const h=new THREE.Path();h.absarc(0,0,inner,0,Math.PI*2,true);s.holes.push(h);
  } else { s.moveTo(-hw,-hd); s.lineTo(hw,-hd); s.lineTo(hw,hd); s.lineTo(-hw,hd); s.closePath(); }
  return s;
}
function profileGeom(e,w,d,len){
  const spec=sectionSpec(e,w,d);
  const g=new THREE.ExtrudeGeometry(profileShape(spec), {depth:len, bevelEnabled:false,curveSegments:32});
  g.translate(0,0,-len/2); return g;                                // extruded along +Z, centred
}
function normalizeRoll(d){ let r=((d%360)+360)%360;if(r>=180)r-=360;return Object.is(r,-0)?0:r; }
function memberFrame(e,up){ const from=e.from,to=e.to,d=new THREE.Vector3(to[0]-from[0],to[1]-from[1],to[2]-from[2]),n=d.clone().normalize();
  // Byte-for-byte the branch test in scene_roll::member_frame, which this mirrors exactly. It has
  // to be: the two branches disagree about a section's facing by up to 180°, so one differing bit
  // here means the same member is DRAWN turned around from the way the IFC and Tekla sinks export
  // it. A sum of squares, never the cancelling `1 - du^2` form, which loses its significant digits
  // in precisely the near-vertical band that picks the branch; and taken from the RAW delta as a
  // ratio, never from the normalized axis, because THREE's normalize() multiplies by the
  // reciprocal length while Rust divides by it, and that one-ulp difference is enough to put the
  // same member on opposite sides of the threshold.
  const U=up==='y'?new THREE.Vector3(0,1,0):new THREE.Vector3(0,0,1), du=n.dot(U),
    seeded=d.clone().addScaledVector(U,-d.dot(U)).lengthSq()<=1e-6*d.lengthSq();let x,y;
  if(seeded){ const seed=new THREE.Vector3(1,0,0);x=seed.addScaledVector(n,-seed.dot(n)).normalize();y=n.clone().cross(x).normalize(); }
  else { y=U.clone().addScaledVector(n,-du).normalize();x=y.clone().cross(n).normalize(); }
  const rot=normalizeRoll(typeof e.rot==='number'?e.rot:0),a=rot*Math.PI/180,c=Math.cos(a),s=Math.sin(a);
  const rx=x.clone().multiplyScalar(c).addScaledVector(y,s),ry=y.clone().multiplyScalar(c).addScaledVector(x,-s);
  return {n,x,y,rx,ry,rot}; }
function orientMember(mesh,e,up){ const F=memberFrame(e,up),x=conv(F.rx.toArray(),up).normalize(),y=conv(F.ry.toArray(),up).normalize();
  // Z-up's legacy screen conversion swaps Y/Z and is reflective. Extrusions are centred, so use
  // x×y as the render axis: cross-section orientation stays exact and reversing the centred local Z
  // axis does not move either endpoint.
  const z=x.clone().cross(y).normalize(),M=new THREE.Matrix4().makeBasis(x,y,z);mesh.quaternion.setFromRotationMatrix(M);
  return {scene:{axis:F.n.toArray(),zeroX:F.x.toArray(),zeroY:F.y.toArray(),x:F.rx.toArray(),y:F.ry.toArray(),rot:F.rot},world:{x:x.toArray(),y:y.toArray(),axis:z.toArray()}};
}

function vec3(P){ return Array.isArray(P)&&P.length===3 ? P : null; }
function axisEnds(e){ const a=e&&e.axis;
  if(a&&Array.isArray(a.from)&&Array.isArray(a.to)) return [a.from,a.to];
  if(Array.isArray(a)&&a.length===2&&Array.isArray(a[0])&&Array.isArray(a[1])) return a;
  return Array.isArray(e.from)&&Array.isArray(e.to) ? [e.from,e.to] : null;
}
function frameOf(e,up){ const f=e.frame||{}, o=conv(f.origin,up), u=conv(f.uDir,up).normalize(), v=conv(f.vDir,up).normalize();
  // The legacy Z-up conversion swaps Y/Z and is therefore reflective. A quaternion cannot
  // carry that reflection, so rebuild the render normal from the converted in-plane axes.
  const n=u.clone().cross(v).normalize(); return {o,u,v,n}; }
function applyFrame(mesh,F){ const M=new THREE.Matrix4().makeBasis(F.u,F.v,F.n); mesh.quaternion.setFromRotationMatrix(M); mesh.position.copy(F.o); }
// ---- Realistic materials (the "realistic" display mode) ----
// The scene carries a SEMANTIC material per element (`meta.material` — "A992", "concrete",
// "galvanised"); the family→appearance mapping is deliberately the RENDERER's, so the look can
// improve without re-baking a single scene. `painted` deliberately has no colour of its own: it
// keeps the element's group colour, because most fabricated steel really is painted and it means
// switching to Realistic does not flatten every member to one grey and destroy the profile legend.
const MATERIALS={
  // Painted steel gets a REAL paint colour — shop-primer grey — NOT the element's group colour.
  // Keeping the group hue here was the first design and it was wrong: it put hot-pink beams in a view
  // whose whole purpose is to look real, giving neither a usable legend nor a believable model.
  // Colour-by-group is what Solid is for; Realistic is for showing someone the building.
  painted:   {metalness:0.0, roughness:0.62, color:0x8f949b},
  steel:     {metalness:1.0, roughness:0.45, color:0x8a8f98},  // bare / mill finish
  galvanised:{metalness:0.85,roughness:0.62, color:0xb8bfc6},  // spangled zinc — rougher on purpose
  stainless: {metalness:1.0, roughness:0.18, color:0xc7ccd1},
  weathering:{metalness:0.55,roughness:0.75, color:0x7a4a32},  // COR-TEN
  aluminium: {metalness:1.0, roughness:0.30, color:0xd6d9dc},
  concrete:  {metalness:0.0, roughness:0.92, color:0xa8a49c},
  timber:    {metalness:0.0, roughness:0.70, color:0xb0854a},
  asphalt:   {metalness:0.0, roughness:0.95, color:0x2e3033},
  glass:     {metalness:0.0, roughness:0.05, color:0xcfe3ee, opacity:0.28},
};
// A grade names the ALLOY, not the finish, so it only decides the look where the alloy IS the
// look. Plain carbon (A36/A992/A572/A709/A500/A501) matches nothing here and falls through to
// `painted` — the finish is genuinely unknown, and painted is both the common case on site and
// the one that preserves the legend colour.
const GRADE_FAMILIES=[
  ['stainless',  /STAINLESS|INOX|A240|\b(304|316|321|410)\b/],
  ['weathering', /WEATHERING|COR-?TEN|A588|A847/],
  ['aluminium',  /ALUMINI?UM|\bALUM\b|\b(6061|6063|5052)\b/],
  ['galvanised', /GALVANI[SZ]ED|\bHDG\b|A123|A153|\bG90\b/],
  ['concrete',   /CONCRETE/],
  ['timber',     /TIMBER|WOOD|GLULAM|\bLVL\b/],
  ['asphalt',    /ASPHALT|BITUMEN|TARMAC/],
  ['glass',      /GLASS|GLAZING/],
];
// `material` is an ELEMENT-level field in the shared scene contract — the same one the IFC
// writer resolves (`resolve_material` reads `el.material`), so a scene authored for IFC export
// shades correctly here with no change to the producer. `meta.material` is accepted as a
// fallback only; reading meta alone would silently drop every canonical scene to `painted`.
function familyOf(e){
  // Trim EACH candidate before choosing, matching the IFC writer's treatment of a
  // trimmed-empty material as absent. A bare `||` would pick a whitespace-only canonical
  // value (truthy) and only then trim it to nothing, skipping the fallback entirely.
  const pick=v=>(v==null?'':String(v)).trim();
  const raw=pick(e&&e.material)||pick(e&&e.meta&&e.meta.material);
  if(!raw) return 'painted';
  if(MATERIALS[raw.toLowerCase()]) return raw.toLowerCase();   // an explicit family name wins outright
  const up=raw.toUpperCase();
  for(const [fam,re] of GRADE_FAMILIES) if(re.test(up)) return fam;
  return 'painted';
}
// ---- procedural surface detail (Realistic mode) ----
// Aggregate, grain and spangle are GENERATED on a canvas, never loaded as images. A shared 3D link is
// one self-contained HTML document capped at a few MB, and a single 1K albedo+normal pair is 1-2 MB,
// so ten families of bitmaps could never fit. Generating them also keeps this document byte-identical
// for an identical scene: the CODE is fixed, only the pixels are made at runtime. Every generator
// draws from a SEEDED prng — never Math.random — so a family's surface never changes between runs.
// Kept in step with floless's web/steel-materials.js, which is the reference implementation.
function mulberry32(seed){ let a=seed>>>0; return ()=>{ a=(a+0x6d2b79f5)>>>0;
  let t=Math.imul(a^(a>>>15),1|a); t=(t+Math.imul(t^(t>>>7),61|t))^t; return ((t^(t>>>14))>>>0)/4294967296; }; }
// Separate X and Y lattice counts. Directional finishes (grain, brushing) come from a COARSER lattice
// on one axis, never from resampling a square field with a squashed index: that reads only the first
// few rows and then jumps back to row 0 at the tile edge, which with RepeatWrapping is a hard seam
// every tile. Anisotropy in the lattice stays periodic on both axes by construction.
function noiseField(size,cellsX,cellsY,seed){ const r=mulberry32(seed), g=new Float32Array(cellsX*cellsY);
  for(let i=0;i<g.length;i++) g[i]=r();
  const at=(x,y)=>g[(((y%cellsY)+cellsY)%cellsY)*cellsX+(((x%cellsX)+cellsX)%cellsX)];
  const sm=t=>t*t*(3-2*t), out=new Float32Array(size*size), sx=cellsX/size, sy=cellsY/size;
  for(let y=0;y<size;y++)for(let x=0;x<size;x++){
    const fx=x*sx, fy=y*sy, x0=Math.floor(fx), y0=Math.floor(fy), tx=sm(fx-x0), ty=sm(fy-y0);
    const a=at(x0,y0), b=at(x0+1,y0), c=at(x0,y0+1), d=at(x0+1,y0+1);
    out[y*size+x]=(a+(b-a)*tx)*(1-ty)+(c+(d-c)*tx)*ty; }
  return out; }
// Starts at a FINE lattice deliberately: a coarse first octave makes big soft blobs, and a blob
// stretched over a 3 m column by the triplanar projection reads as a smear or a stain, not material.
function fbm(size,seed,aniso,baseCells,octaves){ const out=new Float32Array(size*size);
  let amp=1, cells=baseCells||16, norm=0;
  // aniso > 1 stretches the pattern along Y by giving that axis proportionally fewer cells.
  for(let o=0;o<(octaves||5);o++){ const n=noiseField(size,cells,Math.max(1,Math.round(cells/(aniso||1))),seed+o*977);
    for(let i=0;i<out.length;i++) out[i]+=n[i]*amp;
    norm+=amp; amp*=0.5; cells*=2; }
  for(let i=0;i<out.length;i++) out[i]/=norm;
  return out; }
// Detail lives mostly in ROUGHNESS, not albedo. That split is the whole trick: varying diffuse colour
// by more than a few percent is exactly what dirt and water staining look like, whereas varying how a
// surface scatters light is what actually separates cast concrete from brushed stainless.
const SURFACES={
  concrete:  (n,r)=>[0.955+n*0.09+(n>0.88?0.04:0), 0.16-n*0.20+(r()<0.003?0.14:0)],
  timber:    n=>[0.90+n*0.19, 0.08-n*0.14],   // grain genuinely IS a colour difference
  galvanised:n=>[0.965+(n>0.55?0.05:0)+n*0.03, 0.20-(n>0.55?0.16:0)],
  stainless: n=>[0.978+n*0.04, 0.09-n*0.11],  // a brushed finish is not a colour change
  aluminium: n=>[0.978+n*0.04, 0.10-n*0.12],
  weathering:n=>[0.88+n*0.22, 0.08-n*0.13],
  // Orange peel. Subtle in ALBEDO — paint is a uniform colour and mottling it reads as dirt — but
  // real variation in ROUGHNESS, so the sheen shifts across the face. Without that a light paint has
  // nothing to catch the environment and blows out to flat white.
  painted:   n=>[0.982+n*0.034, 0.12-n*0.19],
  steel:     n=>[0.968+n*0.055, 0.08-n*0.11],
  asphalt:   (n,r)=>[0.91+n*0.14+(r()<0.015?0.06:0), 0.07-n*0.09],
  glass:     null,
};
// Directional grain / brushing. Keep MILD: the Y cell count is baseCells/aniso, so a large value
// drives it toward 1, and a single cell is constant along Y — the surface degenerates into regular
// vertical ribbing that reads as corrugated sheet, not a brushed finish.
const STRETCH={timber:5, stainless:6, aluminium:6};
// Base lattice per family: how FINE the dominant detail is. The first octave carries full amplitude,
// so this number — not the octave count — decides what a surface actually looks like. Brushing and
// aggregate need to be dense: a 16-cell lattice across a 180 mm tile gives ~11 mm stripes, which at
// member scale reads as ribbing rather than a finish.
const BASE_CELLS={stainless:56, aluminium:56, timber:22, concrete:26, galvanised:22, asphalt:24, painted:34};
const TILE_MM={concrete:600, asphalt:500, timber:900, weathering:700, galvanised:260,
  stainless:180, aluminium:180, painted:420, steel:400};
// Families whose detail runs ALONG the member (wood grain, brushed stainless, extruded aluminium).
// These must project from object space so the pattern rotates with the part; world-space projection
// leaves grain pinned to the global axes, which reads as timber grain running ACROSS a sloped rafter.
// Isotropic families stay world-projected so neighbouring members read as one continuous finish.
const DIRECTIONAL=new Set(['timber','stainless','aluminium']);
const texByFamily=new Map();
function surfaceFor(family){
  if(!family||!SURFACES[family]) return null;
  if(!texByFamily.has(family)){
    const size=256, paint=SURFACES[family];
    // The map encodes ABSOLUTE roughness around this family's declared value, so it needs it here.
    const spec=MATERIALS[family], baseRough=spec?spec.roughness:0.5;
    let seed=0; for(let i=0;i<family.length;i++) seed=(seed*31+family.charCodeAt(i))>>>0;
    const field=fbm(size,seed,STRETCH[family]||1,BASE_CELLS[family]||16), r=mulberry32(seed^0x9e3779b9);
    const ac=document.createElement('canvas'), rc=document.createElement('canvas');
    ac.width=ac.height=rc.width=rc.height=size;
    const ai=ac.getContext('2d').createImageData(size,size), ri=rc.getContext('2d').createImageData(size,size);
    for(let y=0;y<size;y++)for(let x=0;x<size;x++){
      const n=field[y*size+x], v=paint(n,r), i=(y*size+x)*4;
      const L=Math.max(0,Math.min(255,Math.round(v[0]*255)));
      ai.data[i]=ai.data[i+1]=ai.data[i+2]=L; ai.data[i+3]=255;
      // ABSOLUTE roughness, not a delta around 0.5: three MULTIPLIES roughness by this channel, so a
      // map centred on 0.5 silently halves every declared roughness (concrete 0.92 → ~0.46, twice as
      // glossy as specified). applyDisplayMode sets material.roughness = 1 so the multiply is an
      // identity and what is written here is exactly the roughness used. (Read from GREEN.)
      const R=Math.max(0,Math.min(255,Math.round((baseRough+v[1])*255)));
      ri.data[i]=ri.data[i+1]=ri.data[i+2]=R; ri.data[i+3]=255; }
    ac.getContext('2d').putImageData(ai,0,0); rc.getContext('2d').putImageData(ri,0,0);
    const mk=(cv,srgb)=>{ const t=new THREE.CanvasTexture(cv); t.wrapS=t.wrapT=THREE.RepeatWrapping;
      if(srgb) t.colorSpace=THREE.SRGBColorSpace; t.anisotropy=4; return t; };
    // Albedo is colour data (sRGB); roughness is linear — tagging it sRGB washes the detail out.
    texByFamily.set(family,{map:mk(ac,true), roughnessMap:mk(rc,false)});
  }
  return texByFamily.get(family); }
// Project the maps from world XYZ, blended by the normal, instead of through the mesh UVs. An
// extruded member's UVs are millimetre-scale and differ between its cap faces and its side walls, so
// any single repeat value stretches somewhere — badly on a coped end. Triplanar sidesteps UVs, which
// is the only thing that reads correctly at architectural scale.
// Which LOCAL axis a geometry's length runs along — the thing directional sampling must follow.
// The constructors disagree, which is the whole reason this exists: BoxGeometry(w,len,d) puts length
// on Y and CylinderGeometry is Y-axial, while ExtrudeGeometry({depth:len}) runs along Z. Imported
// meshes have no canonical length axis at all, so they keep world projection.
function lengthAxis(e,kind,isMesh){
  if(isMesh||kind==='node') return null;
  if(kind==='rod'||kind==='bolt-shank') return 'y';              // CylinderGeometry: Y-axial
  if(kind==='plate'||kind==='washer'||kind==='nut'||kind==='bolt-head') return 'z'; // extruded on Z (thickness)
  return 'z';                                                    // every member profile now extrudes on local Z
}
function applyTriplanar(material,scaleMm,axis){
  // `axis` null → WORLD projection: isotropic finishes (concrete, asphalt, paint) stay continuous
  // across neighbouring members instead of each carrying its own tile origin.
  //
  // Otherwise project from OBJECT space AND permute so the member's length lands on the sampling
  // space's Z. That permutation is the load-bearing part: STRETCH coarsens the texture's Y cell
  // count, so the grain runs along texture V — and V maps to `z` on the X- and Y-facing side faces
  // (p.yz / p.xz). Object space ALONE therefore only works for geometry whose length is already
  // local Z; on a BoxGeometry member (length Y) the grain would run across the short axis. Swizzling
  // xzy for those makes vertical boxes, horizontal boxes, sloped members and extrusions all correct
  // at once, which no single projection space achieves.
  const swizzle = axis==='y' ? '.xzy' : '';
  const posExpr = axis ? ('transformed'+swizzle) : '(modelMatrix*vec4(transformed,1.0)).xyz';
  // The normal must ride the SAME permutation, or the triplanar blend weights would pick faces in
  // the unpermuted frame and cross-fade the wrong samples at the corners.
  const nrmExpr = axis ? ('objectNormal'+swizzle) : 'mat3(modelMatrix)*objectNormal';
  material.onBeforeCompile=(shader)=>{
    shader.uniforms.uTriScale={value:1/Math.max(1,scaleMm)};
    shader.vertexShader=shader.vertexShader
      .replace('#include <common>','#include <common>\nvarying vec3 vTriPos;\nvarying vec3 vTriNrm;')
      .replace('#include <worldpos_vertex>','#include <worldpos_vertex>\n  vTriPos='+posExpr+';\n  vTriNrm='+nrmExpr+';');
    const helper='\nvarying vec3 vTriPos;\nvarying vec3 vTriNrm;\nuniform float uTriScale;\n'
      +'vec4 triplanar(sampler2D tex, vec3 p, vec3 n){\n'
      +'  vec3 b=pow(abs(normalize(n)),vec3(4.0));\n'   // 4th power keeps faces crisp near corners
      +'  b/=max(b.x+b.y+b.z,1e-4);\n'
      +'  return texture2D(tex,p.yz)*b.x+texture2D(tex,p.xz)*b.y+texture2D(tex,p.xy)*b.z;\n}';
    shader.fragmentShader=shader.fragmentShader
      .replace('#include <common>','#include <common>'+helper)
      .replace('#include <map_fragment>','\n#ifdef USE_MAP\n  diffuseColor*=triplanar(map,vTriPos*uTriScale,vTriNrm);\n#endif')
      .replace('#include <roughnessmap_fragment>','\nfloat roughnessFactor=roughness;\n#ifdef USE_ROUGHNESSMAP\n  roughnessFactor*=triplanar(roughnessMap,vTriPos*uTriScale,vTriNrm).g;\n#endif');
  };
  // Three caches programs by this key; without a distinct one a patched and an unpatched material
  // with otherwise identical parameters would silently share the WRONG compiled program.
  material.customProgramCacheKey=()=>'triplanar:'+scaleMm+':'+(axis||'world');
  material.needsUpdate=true; }

function solidMaterial(e,colorOf,opacityOf,doubleSided,kind){ const col=colorOf[e.group]||0xffffff;
  const op=typeof e.opacity==='number'?e.opacity:(typeof opacityOf[e.group]==='number'?opacityOf[e.group]:1);
  const mat=new THREE.MeshStandardMaterial({color:col,metalness:0.5,roughness:0.5,transparent:op<1, opacity:op,side:doubleSided?THREE.DoubleSide:THREE.FrontSide});
  // Clipping is applied globally via renderer.clippingPlanes, but the shadow pass uses a depth
  // material that ignores it unless the material opts in. Without this, a clip plane, clip box or
  // work area removes geometry from the view while it keeps casting an impossible shadow.
  mat.clipShadows=true;
  const fam=familyOf(e);
  // Patch ONCE at creation rather than toggling with the mode: both replaced chunks are guarded by
  // USE_MAP/USE_ROUGHNESSMAP, so with no maps bound the patched shader matches the stock one — and
  // Solid never pays a program recompile on switch.
  if(TILE_MM[fam]) applyTriplanar(mat,TILE_MM[fam],DIRECTIONAL.has(fam)?lengthAxis(e,kind,doubleSided):null);
  mat.userData={baseOpacity:op, baseColor:col, family:fam}; return mat; }
function cylinderBetween(a,b,r,mat,segments){ const d=b.clone().sub(a), len=d.length();
  const mesh=new THREE.Mesh(new THREE.CylinderGeometry(r,r,len,segments||32,1,false),mat);
  mesh.position.copy(a).add(b).multiplyScalar(0.5); mesh.quaternion.setFromUnitVectors(_YA,d.normalize()); return mesh; }
function plateMesh(e,up,mat){ const outline=e.outline, shape=new THREE.Shape();
  outline.forEach((p,i)=>i?shape.lineTo(p[0],p[1]):shape.moveTo(p[0],p[1])); shape.closePath();
  for(const h of (e.holes||[])){ const c=h.center||h.uv, path=new THREE.Path(); path.absarc(c[0],c[1],h.diameterMm/2,0,Math.PI*2,false); shape.holes.push(path); }
  const g=new THREE.ExtrudeGeometry(shape,{depth:e.thicknessMm,bevelEnabled:false,curveSegments:48}); g.translate(0,0,-e.thicknessMm/2);
  const mesh=new THREE.Mesh(g,mat); applyFrame(mesh,frameOf(e,up)); return mesh; }
function orientedProfileMesh(e,up,mat,shape){
  const g=new THREE.ExtrudeGeometry(shape,{depth:e.thicknessMm,bevelEnabled:false,curveSegments:48}); g.translate(0,0,-e.thicknessMm/2);
  const sourceN=new THREE.Vector3(...e.axis).normalize(), sourceSeed=Math.abs(sourceN.z)<0.9?new THREE.Vector3(0,0,1):new THREE.Vector3(1,0,0);
  const sourceU=sourceSeed.cross(sourceN).normalize(), n=conv(e.axis,up).normalize(), u=conv([sourceU.x,sourceU.y,sourceU.z],up).normalize(), v=n.clone().cross(u).normalize();
  const mesh=new THREE.Mesh(g,mat); applyFrame(mesh,{o:conv(e.center,up),u,v,n}); return mesh; }
function annulusMesh(e,up,mat){ const s=new THREE.Shape(); s.absarc(0,0,e.outerDiameterMm/2,0,Math.PI*2,false);
  const h=new THREE.Path(); h.absarc(0,0,e.innerDiameterMm/2,0,Math.PI*2,true); s.holes.push(h); return orientedProfileMesh(e,up,mat,s); }
function hexMesh(e,up,mat){ const R=e.acrossFlatsMm/Math.sqrt(3), phase=Number(e.phaseRad||0)*(up==='z'?-1:1), s=new THREE.Shape();
  for(let i=0;i<6;i++){ const q=phase+i*Math.PI/3, x=R*Math.cos(q), y=R*Math.sin(q); i?s.lineTo(x,y):s.moveTo(x,y); } s.closePath(); return orientedProfileMesh(e,up,mat,s); }
// ---- structural grids: ONE canonical transform ----------------------------------------------
// A structural grid is authored in PLAN space — X/Y are the two plan axes and the third component is
// the ELEVATION — and that is true regardless of `meta.up`, because the grid contract has its own
// frame (see 10-core: axes carry `direction:'x'|'y'` + `offsetMm`, levels carry an absolute
// `elevationMm`). Elements do NOT: they are scene-space and go through conv(P,up).
//
// So grid geometry must NOT be routed through conv(). It was, and on a `meta.up:'y'` scene conv is
// the identity, which dropped the elevation into world Z and left every LEVEL rendering as a
// VERTICAL plane. `expandSceneBounds` carried the same bug, so Fit and maxDim were wrong there too.
// The mapping below is unconditional, and every consumer — rendering, bounds, labels, and the clip
// draw's snap candidates — reads these exact segments, so a displayed grid and a snappable grid
// cannot drift apart.
const gridToWorld=(gx,gy,elev)=>new THREE.Vector3(gx,elev,gy);
function referenceSystemSegments(R){
  const o=R.origin,b=R.bounds||{}, x0=Number(b.minX),x1=Number(b.maxX),y0=Number(b.minY),y1=Number(b.maxY);
  if(!vec3(o)||![x0,x1,y0,y1].every(Number.isFinite)) return { axes:[], levels:[] };
  const axes=[];
  for(const a of (R.axes||[])){
    if(!a||!Number.isFinite(Number(a.offsetMm)))continue;
    // An axis runs the full bounds of the CROSS direction unless it names its own start/end.
    const start=Number.isFinite(a.startMm)?a.startMm:(a.direction==='x'?y0:x0);
    const end=Number.isFinite(a.endMm)?a.endMm:(a.direction==='x'?y1:x1);
    const A=a.direction==='x'?gridToWorld(o[0]+a.offsetMm,o[1]+start,o[2]):gridToWorld(o[0]+start,o[1]+a.offsetMm,o[2]);
    const B=a.direction==='x'?gridToWorld(o[0]+a.offsetMm,o[1]+end,o[2]):gridToWorld(o[0]+end,o[1]+a.offsetMm,o[2]);
    axes.push({ label:a.label, direction:a.direction, a:A, b:B });
  }
  const levels=[];
  for(const l of (R.levels||[])){
    if(!l||!Number.isFinite(Number(l.elevationMm)))continue;
    const e=l.elevationMm, cx=o[0]+(x0+x1)/2, cy=o[1]+(y0+y1)/2;
    // A level is drawn as a crosshair through the grid centre, not a filled plane.
    levels.push({ label:l.label, y:e, segments:[
      [gridToWorld(o[0]+x0,cy,e), gridToWorld(o[0]+x1,cy,e)],
      [gridToWorld(cx,o[1]+y0,e), gridToWorld(cx,o[1]+y1,e)] ], labelAt:gridToWorld(o[0]+x1,cy,e) });
  }
  return { axes, levels };
}
function expandSceneBounds(box,S,up){ const add=P=>{if(vec3(P))box.expandByPoint(conv(P,up));};
  const addRadius=(P,r)=>{if(!vec3(P)||!Number.isFinite(r))return; for(const dx of [-r,r])for(const dy of [-r,r])for(const dz of [-r,r])add([P[0]+dx,P[1]+dy,P[2]+dz]);};
  for(const e of (S.elements||[])){ if(!e)continue; add(e.from);add(e.to);add(e.at);add(e.center); const A=axisEnds(e);if(A){const r=(e.diameterMm||0)/2;addRadius(A[0],r);addRadius(A[1],r);}
    if(e.kind==='washer')addRadius(e.center,Math.max(e.outerDiameterMm/2,e.thicknessMm/2));
    if(e.kind==='nut'||e.kind==='bolt-head')addRadius(e.center,Math.max(e.acrossFlatsMm/Math.sqrt(3),e.thicknessMm/2));
    if(Array.isArray(e.positions))for(let i=0;i+2<e.positions.length;i+=3)add([e.positions[i],e.positions[i+1],e.positions[i+2]]);
    if(e.kind==='plate'&&e.frame&&Array.isArray(e.outline)){ const F=frameOf(e,up), z=e.thicknessMm/2; for(const p of e.outline)for(const dz of [-z,z])box.expandByPoint(F.o.clone().addScaledVector(F.u,p[0]).addScaledVector(F.v,p[1]).addScaledVector(F.n,dz)); }
  }
  // Grid bounds come from the SAME segments the renderer draws (never conv()'d — see
  // referenceSystemSegments), so Fit and maxDim agree with what is on screen on a y-up scene too.
  for(const R of (S.referenceSystems||[])){ if(!R||R.kind!=='structural-grid')continue;
    const seg=referenceSystemSegments(R);
    for(const a of seg.axes){ box.expandByPoint(a.a); box.expandByPoint(a.b); }
    for(const l of seg.levels) for(const s of l.segments){ box.expandByPoint(s[0]); box.expandByPoint(s[1]); } }
  for(const op of (S.operations||[])){ if(op&&op.kind==='weld'&&Array.isArray(op.path))for(const p of op.path)add(p); }
}
// Grid geometry comes from referenceSystemSegments — already in WORLD space, never conv()'d.
let gridLines=[];
function addReferenceSystems(S){ gridLines=[]; for(const R of (S.referenceSystems||[])){ if(!R||R.kind!=='structural-grid')continue;
  const baseMat=new THREE.LineBasicMaterial({color:0x60a5fa,transparent:true,opacity:0.7});
  const line=(A,B,role)=>{const g=new THREE.BufferGeometry().setFromPoints([A,B]);
    const ln=new THREE.Line(g,baseMat.clone()); ln.userData={gridRole:role, a:A.toArray(), b:B.toArray()};
    content.add(ln); gridLines.push(ln);};
  const seg=referenceSystemSegments(R);
  for(const a of seg.axes){ line(a.a,a.b,'axis'); content.add(makeLabel(a.label,a.b,maxDim)); }
  for(const l of seg.levels){ for(const s of l.segments) line(s[0],s[1],'level'); content.add(makeLabel(l.label,l.labelAt,maxDim)); }
} }
function addOperations(S,up){ for(const op of (S.operations||[])){ if(!op||op.kind!=='weld'||!Array.isArray(op.path))continue;
  const points=op.path.map(p=>conv(p,up)); if(points.length<2)continue;
  const geometry=new THREE.BufferGeometry().setFromPoints(points);
  const material=new THREE.LineBasicMaterial({color:0xf59e0b,transparent:true,opacity:0.95});
  const weld=new THREE.Line(geometry,material); weld.userData=op; content.add(weld);
  // Legend-operable, but deliberately NOT added to `pickable`. Canvas picking would break on it:
  // box-select projects an object's ORIGIN and a weld line's geometry holds world points while the
  // object sits at the origin; clip placement aborts on a hit with no face; and highlighting
  // expects an emissive material where this is a LineBasicMaterial.
  opRenderables.push(weld);
} }

function renderScene(S){
  clearContent();
  const up=(S.meta&&S.meta.up)||'z';
  const colorOf={}, opacityOf={}; (S.groups||[]).forEach(g=>{ colorOf[g.key]=g.color; if(typeof g.opacity==='number') opacityOf[g.key]=g.opacity; });
  groupColor=colorOf;   // kept module-level so a panel row can show its group's colour
  groupHidden.clear(); soloGroup=null;
  const box=new THREE.Box3(); expandSceneBounds(box,S,up);
  if(box.isEmpty()) box.set(new THREE.Vector3(-1,-1,-1), new THREE.Vector3(1,1,1));
  const size=box.getSize(new THREE.Vector3()), center=box.getCenter(new THREE.Vector3());
  maxDim=Math.max(size.x,size.y,size.z)||1; sceneBox=box.clone(); const thick=maxDim*0.006;

  // Kept on module scope so Realistic can dim them: an environment map is itself a light source, and
  // leaving this rig at full strength on top of it lights the scene roughly twice over.
  lightHemi=new THREE.HemisphereLight(0x9fc5ff,0x0a0f1a,0.95); content.add(lightHemi);
  lightKey=new THREE.DirectionalLight(0xffffff,1.3); lightKey.position.copy(center).add(new THREE.Vector3(maxDim,maxDim*1.5,maxDim*0.6)); content.add(lightKey);
  lightFill=new THREE.DirectionalLight(0x88aaff,0.5); lightFill.position.copy(center).add(new THREE.Vector3(-maxDim,maxDim*0.7,-maxDim)); content.add(lightFill);
  const grid=new THREE.GridHelper(maxDim*1.9, 24, 0x1e293b, 0x131c2e); grid.position.set(center.x, box.min.y, center.z); content.add(grid);

  const upY=new THREE.Vector3(0,1,0);
  for(const e of (S.elements||[])){
    if(!e) continue;
    // A tessellated mesh (positions[]+indices[], e.g. an imported connection) has no from/to/at.
    const kind=e.kind||((Array.isArray(e.positions)&&Array.isArray(e.indices))?'mesh':(Array.isArray(e.at)?'node':'member'));
    const isMesh = kind==='mesh';
    if(!['mesh','plate','rod','bolt-shank','washer','nut','bolt-head','node','line','box','member'].includes(kind)) continue;
    const A=axisEnds(e);
    // Opacity: per-element overrides per-group; <1 makes the material translucent so
    // elements embedded in others (e.g. rebar inside concrete) can be revealed (#258).
    // Imported meshes may have inconsistent winding — DoubleSide avoids black back-faces.
    const mat=solidMaterial(e,colorOf,opacityOf,isMesh,kind); let mesh,rollFrame=null;
    if(isMesh){ const g=new THREE.BufferGeometry(), P=e.positions, arr=new Float32Array(P.length);
      for(let i=0;i+2<P.length;i+=3){ const v=conv([P[i],P[i+1],P[i+2]],up); arr[i]=v.x; arr[i+1]=v.y; arr[i+2]=v.z; }
      g.setAttribute('position', new THREE.BufferAttribute(arr,3)); g.setIndex(e.indices); g.computeVertexNormals();
      mesh=new THREE.Mesh(g, mat); }
    else if(kind==='plate') mesh=plateMesh(e,up,mat);
    else if(kind==='rod'||kind==='bolt-shank') mesh=cylinderBetween(conv(A[0],up),conv(A[1],up),e.diameterMm/2,mat,32);
    else if(kind==='washer') mesh=annulusMesh(e,up,mat);
    else if(kind==='nut'||kind==='bolt-head') mesh=hexMesh(e,up,mat);
    else if(kind==='node'){ const r=(e.size||maxDim*0.012); mesh=new THREE.Mesh(new THREE.SphereGeometry(r,20,16), mat); mesh.position.copy(conv(e.at,up)); }
    else { const a=conv(e.from,up), b=conv(e.to,up), dir=b.clone().sub(a), len=dir.length()||thick;
      const w=(e.section&&e.section.w)||thick, d=(e.section&&e.section.d)||thick;
      mesh=new THREE.Mesh(profileGeom(e,w,d,len), mat); mesh.position.copy(a).add(b).multiplyScalar(0.5);
      rollFrame=orientMember(mesh,e,up); }
           mesh.receiveShadow=true;   // steel catches shadow from steel, not just from the ground; casting is decided per-pass in applyDisplayMode
    mesh.userData=e;if(rollFrame)mesh.userData.rollFrame=rollFrame;content.add(mesh); pickable.push(mesh);
  }
  for(const g of (S.grids||[])) if(g&&Array.isArray(g.at)) content.add(makeLabel(g.label, conv(g.at,up), maxDim));
  addReferenceSystems(S);
  addOperations(S,up);
  sizeShadowRig();      // needs the finished meshes: the scene bounds are centrelines, not extents
  applyDisplayMode();   // a rebuild makes new lights and materials — re-apply the current mode

  if(S.camera&&Array.isArray(S.camera.eye)&&Array.isArray(S.camera.target)){
    const eye=conv(S.camera.eye,up), tgt=conv(S.camera.target,up);
    perspCam.position.copy(eye); orthoCam.position.copy(eye); controls.target.copy(tgt);
    const near=maxDim/500, far=maxDim*40;
    perspCam.near=near; perspCam.far=far; perspCam.updateProjectionMatrix();
    orthoCam.near=near; orthoCam.far=far;
    if(camera.isOrthographicCamera) reframeOrtho();
    controls.update();
  } else { frameBox(sceneBox, new THREE.Vector3(1,0.8,1)); }
  // The registry must exist before the panel resolves rows against it. Rust has already validated
  // (or dropped) the descriptor, so a present one is known-good here.
  LEG=(S.legend&&Array.isArray(S.legend.modes)&&S.legend.modes.length)?S.legend:null;
  legModeIx=0; legQuery=''; legCollapsed.clear();
  selIds=new Set(); hiddenIds=new Set(); isolatedIds=null; legAnchor=null;
  buildTargetRegistry(); resolveLegRows();
  applyDisplayMode(); applyGroupVisibility();

  buildSidePanels(S); buildLegend(S); setHint();
  document.getElementById('sceneName').textContent=(S.meta&&S.meta.name)||'';
}

function buildSidePanels(S){
  // No panels ⇒ no side panel. It used to render anyway, titled with the scene name, so a scene
  // that supplies no tables showed an empty box repeating a title the page already carries.
  const hasPanels=Array.isArray(S.panels)&&S.panels.length>0;
  document.body.classList.toggle('no-side',!hasPanels);
  if(!hasPanels){ document.getElementById('panels').replaceChildren(); return; }
  document.getElementById('sideTitle').textContent=(S.panels&&S.panels[0]&&S.panels[0].title)||(S.meta&&S.meta.name)||'';
  document.getElementById('sideNote').textContent=(S.panels&&S.panels[0]&&S.panels[0].note)||'';
  const host=document.getElementById('panels'); host.replaceChildren();
  (S.panels||[]).forEach((p,i)=>{
    if(i>0){ const h=el('h2',null,p.title); h.style.marginTop='10px'; host.append(h); if(p.note) host.append(el('p','note',p.note)); }
    const table=el('table'); const thead=el('thead'); const htr=el('tr');
    (p.columns||[]).forEach((c,ci)=>htr.append(el('th', ci===0?null:'num', c))); thead.append(htr); table.append(thead);
    const tb=el('tbody'); (p.rows||[]).forEach(r=>{ const tr=el('tr'); r.forEach((cell,ci)=>tr.append(el('td', ci===0?null:'num', String(cell)))); tb.append(tr); });
    table.append(tb); host.append(table);
  });
}
// ---- the objects panel (descriptor-driven) ----------------------------------------------------
// Row verbs mirror the floless editor: the LABEL selects, a tri-state box shows/hides, and an
// explicit isolate button gives keyboard and touch a route that is not a double-click.
function legRowMatches(r){ return !legQuery || r.label.toLowerCase().includes(legQuery); }
/** Actually on screen: matches the search AND is not inside a collapsed category. A Shift range
 *  must not reach rows the user cannot see — a search temporarily opens matching categories, which
 *  is why the collapse test is skipped while querying. */
function legRowDisplayed(r){ return legRowMatches(r) && !(r.catLabel && legCollapsed.has(r.catKey) && !legQuery); }
function legApply(){ applyGroupVisibility(); refreshLegend(); }
function legSelectRow(row,additive){
  if(additive){ const all=row.ids.every(id=>selIds.has(id));
    for(const id of row.ids){ if(all) selIds.delete(id); else selIds.add(id); } }
  else { selIds=new Set(row.ids); }
  legAnchor=row.key; syncSelectionFromIds();
}
function legSelectRange(row){                       // Shift: over the rows CURRENTLY displayed
  const shown=legRows.filter(legRowDisplayed);
  const a=shown.findIndex(r=>r.key===legAnchor), b=shown.findIndex(r=>r.key===row.key);
  if(a<0||b<0){ legSelectRow(row,false); return; }
  selIds=new Set(); for(let i=Math.min(a,b);i<=Math.max(a,b);i++) for(const id of shown[i].ids) selIds.add(id);
  syncSelectionFromIds();
}
function legToggleVis(row){ const v=rowVis(row);
  for(const id of row.ids){ if(v==='true') hiddenIds.add(id); else hiddenIds.delete(id); }  // mixed → show all
  legApply();
}
function legIsolate(row){                            // a selected row isolates the whole selection
  const ids = selIds.size && row.ids.some(id=>selIds.has(id)) ? [...selIds] : row.ids;
  const same = isolatedIds && isolatedIds.size===ids.length && ids.every(id=>isolatedIds.has(id));
  isolatedIds = same ? null : new Set(ids);          // isolating the same set again exits
  legApply();
}
function legShowAll(){                               // two DISTINCT transitions, never conflated
  if(isolatedIds) isolatedIds=null;                  // exit isolation, keeping manual hides
  else hiddenIds.clear();                            // then, separately, un-hide
  legApply();
}
/** The producer-authored panel: mode toggle, search, then sections → categories → rows.
 *  Header is fixed; only the row body scrolls. */
function buildObjectsPanel(){
  const host=document.getElementById('legend'); host.replaceChildren(); host.style.display='';
  host.classList.add('objects');
  resolveLegRows();

  if((LEG.modes||[]).length>1){                       // a toggle only when there IS a choice
    const modes=el('div','omode');
    LEG.modes.forEach((m,i)=>{ const b=el('button',i===legModeIx?'on':null,m.label);
      b.type='button'; b.setAttribute('aria-pressed',i===legModeIx?'true':'false');
      b.addEventListener('click',()=>{ if(i===legModeIx) return; legModeIx=i; legAnchor=null; buildObjectsPanel(); });
      modes.append(b); });
    host.append(modes);
  }

  const search=el('div','osearch');
  const input=document.createElement('input');
  input.type='text'; input.placeholder='Search objects…'; input.value=legQuery;
  input.setAttribute('aria-label','Search objects in the list'); input.autocomplete='off';
  input.addEventListener('input',()=>{ legQuery=input.value.trim().toLowerCase(); paintRows(); });
  // Escape clears first, blurs second — and never reaches the window handler, which would
  // otherwise cancel an armed clip.
  input.addEventListener('keydown',e=>{ if(e.key!=='Escape') return; e.stopPropagation();
    if(input.value){ input.value=''; legQuery=''; paintRows(); } else input.blur(); });
  search.append(input); host.append(search);

  const clear=el('button',null,'Show all'); clear.id='legClear'; clear.type='button';
  clear.addEventListener('click',legShowAll); host.append(clear);

  host.append(el('div','ohint','click a row to select · box shows/hides · ⊙ isolates · Ctrl/Shift multi-select'));
  const body=el('div','obody'); body.id='legBody'; host.append(body);
  paintRows();
  refreshObjectsPanel();
}

/** Rebuild just the scrolling row body (search/collapse changes) — the header keeps its state. */
function paintRows(){
  const body=document.getElementById('legBody'); if(!body) return; body.replaceChildren();
  const visible=legRows.filter(legRowMatches);
  let sec=null, cat=null;
  for(const row of visible){
    if(row.secLabel && row.secKey!==sec){ sec=row.secKey; cat=null; body.append(el('div','osec',row.secLabel)); }
    if(row.catKey!==cat){
      cat=row.catKey;
      if(row.catLabel){
        const count=visible.filter(r=>r.catKey===row.catKey&&r.secKey===row.secKey).length;
        const open=!legCollapsed.has(row.catKey)||!!legQuery;   // a search temporarily opens matches
        const h=el('button','ocat'); h.type='button'; h.setAttribute('aria-expanded',open?'true':'false');
        h.append(el('span','ochev',open?'▾':'▸'), el('span','ocatlabel',row.catLabel), el('span','ocount','('+count+')'));
        h.addEventListener('click',()=>{ if(legCollapsed.has(row.catKey)) legCollapsed.delete(row.catKey); else legCollapsed.add(row.catKey); paintRows(); refreshObjectsPanel(); });
        body.append(h);
      }
    }
    if(row.catLabel && legCollapsed.has(row.catKey) && !legQuery) continue;
    body.append(buildRow(row));
  }
  if(!visible.length) body.append(el('div','oempty','No objects match “'+legQuery+'”'));
}

function buildRow(row){
  const node=el('div','orow'+(row.catLabel?' typed':'')); node.dataset.key=row.key;
  // Descriptor colour wins; otherwise inherit the first target's group colour, so the common case
  // needs no colour in the descriptor at all. Weld operations carry no group — they fall through
  // to the neutral default.
  const first=row.ids.length?targetOf.get(row.ids[0]):null;
  const colour=row.color||(first&&groupColor[first[0].userData.group])||'#94a3b8';

  // Visibility — a real tri-state checkbox: once hiding is per-id a row can be PARTLY hidden,
  // which role="switch" cannot express.
  const box=el('button','ovis'); box.type='button'; box.dataset.act='vis';
  box.setAttribute('role','checkbox'); box.setAttribute('aria-checked',rowVis(row));
  box.setAttribute('aria-label','Show or hide '+row.label);
  box.setAttribute('data-tip','Show or hide'); box.style.setProperty('--sw',colour);
  box.append(el('span','oswatch'));
  box.addEventListener('click',e=>{ e.stopPropagation(); legToggleVis(row); });

  const pick=el('button','opick',row.label); pick.type='button'; pick.dataset.act='pick';
  pick.setAttribute('aria-pressed','false');
  pick.setAttribute('data-tip','Select · Ctrl/Shift to multi-select · double-click to isolate');
  // The plain click is DEFERRED so a double-click can cancel it: otherwise the first click of a
  // dblclick replaces a multi-selection and "isolate the selection" silently becomes "isolate this row".
  pick.addEventListener('click',e=>{ e.stopPropagation();
    if(e.shiftKey){ clearTimeout(legendClickT); legSelectRange(row); return; }
    if(e.ctrlKey||e.metaKey){ clearTimeout(legendClickT); legSelectRow(row,true); return; }
    clearTimeout(legendClickT); legendClickT=setTimeout(()=>legSelectRow(row,false),220); });
  pick.addEventListener('dblclick',e=>{ e.preventDefault(); e.stopPropagation(); clearTimeout(legendClickT); legIsolate(row); });

  const iso=el('button','oiso','⊙'); iso.type='button'; iso.dataset.act='iso';
  iso.setAttribute('aria-label','Isolate '+row.label);
  iso.setAttribute('data-tip','Isolate — the keyboard/touch route, no double-click needed');
  iso.addEventListener('click',e=>{ e.stopPropagation(); legIsolate(row); });

  node.append(box,pick,iso); return node;
}

function buildLegend(S){ const host=document.getElementById('legend'); host.replaceChildren();
  if(S.legendError) console.warn('viewer-3d: objects panel ignored — '+S.legendError);
  if(legActive()){ buildObjectsPanel(); return; }
  host.classList.remove('objects');
  // `groups` MUST be bound here. v0.97.0 dropped this line while leaving the forEach below, so every
  // scene without a descriptor threw ReferenceError out of renderScene and killed the whole module —
  // no model, no toolbar, no __viewer3d. And since this legacy list is the documented fallback for a
  // REJECTED descriptor, a producer bug in the panel took the viewer down with it, which is the
  // opposite of the "the panel is chrome, the model is the payload" promise it falls back to honour.
  const groups=(S.groups||[]); if(!groups.length){ host.style.display='none'; return; } host.style.display='';
  host.append(el('div','legend-hint','click: hide/show · dbl-click: isolate'));
  groups.forEach(g=>{ const row=el('div','row'); row.dataset.key=g.key;
    const sw=el('span','swatch'); sw.style.background=g.color;
    row.append(sw, document.createTextNode(g.label));
    row.setAttribute('data-tip','Click to hide/show · double-click to isolate');
    // Defer the single-click toggle so a double-click can cancel it — otherwise the two
    // clicks preceding `dblclick` would clear soloGroup and isolate would never toggle off.
    row.addEventListener('click', ()=>{ clearTimeout(legendClickT); legendClickT=setTimeout(()=>toggleGroup(g.key), 220); });
    row.addEventListener('dblclick', e=>{ e.preventDefault(); clearTimeout(legendClickT); soloToggle(g.key); });
    host.append(row); });
  refreshLegend(); }

const ray=new THREE.Raycaster(), mouse=new THREE.Vector2(); const readout=document.getElementById('readout');
function setHint(){ readout.replaceChildren(document.createTextNode('Left-drag to box-select · right-drag to orbit · '), el('b',null,'click an element'), document.createTextNode(' to inspect')); }

// ---- selection: a left click picks one element; a left-drag rubber-bands a multi-select (#258) ----
let selection=[];
function clearHighlight(){ for(const m of selection){ const mat=m.material; if(mat&&mat.emissive) mat.emissive.setHex(0x000000); } }
function setSelection(meshes){
  clearHighlight(); selection=meshes||[];
  // With a descriptor, selIds is the source of truth — so a CANVAS pick (click, box-select) has to
  // write it too. Without this the panel keeps highlighting the previous selection and isolate acts
  // on it. Deriving from the objects is idempotent, so the panel→canvas path round-trips unchanged.
  if(legActive()){ selIds=new Set(); for(const m of selection){ const id=m.userData&&m.userData.id; if(id) selIds.add(id); } refreshObjectsPanel(); }
  for(const m of selection){ const mat=m.material; if(mat){ mat.emissive=new THREE.Color(0xf59e0b); mat.emissiveIntensity=0.6; } }
  writeSelectionReadout();
}
function writeSelectionReadout(){
  if(selection.length===0){ setHint(); return; }
  if(selection.length===1){ const u=selection[0].userData; const parts=[el('b',null,u.id||'(element)')];
    if(u.group) parts.push(document.createTextNode(' · '), el('span','pill',u.group));
    for(const [k,v] of Object.entries(u.meta||{})) parts.push(document.createTextNode(` · ${k}: ${v}`));
    readout.replaceChildren(...parts); return; }
  readout.replaceChildren(el('b',null,String(selection.length)), document.createTextNode(' elements selected'));
}
// #readout is a fixed, always-visible panel (floless's floats and hides itself), so a drag readout
// left in place would sit there forever. Every gesture terminal calls this.
function refreshReadout(){ if(clipMode){ setClipPrompt(); return; } writeSelectionReadout(); }
// Raycast a single element at a screen point. Only VISIBLE meshes — a legend-hidden / soloed-out
// group in front must not swallow the click for the visible element behind it (raycaster ignores `visible`).
function pickAt(cx,cy){ mouse.x=(cx/innerWidth)*2-1; mouse.y=-(cy/innerHeight)*2+1; ray.setFromCamera(mouse,camera);
  const hit=ray.intersectObjects(pickable.filter(m=>m.visible),false)[0]; setSelection(hit?[hit.object]:[]); }
// Project a mesh centre to screen px (null when behind / clipped beyond the far plane).
function screenOf(obj){ const v=obj.getWorldPosition(new THREE.Vector3()).project(camera);
  return (v.z>1) ? null : { x:(v.x+1)*0.5*innerWidth, y:(-v.y+1)*0.5*innerHeight }; }
// Window select: every visible element whose centre falls inside the drag rectangle.
// ponytail: centre-point hit-test; upgrade to a projected-bbox test if partial members must catch.
function meshesInRect(x0,y0,x1,y1){ const lo={x:Math.min(x0,x1),y:Math.min(y0,y1)}, hi={x:Math.max(x0,x1),y:Math.max(y0,y1)};
  const out=[]; for(const m of pickable){ if(!m.visible) continue; const s=screenOf(m);
    if(s && s.x>=lo.x && s.x<=hi.x && s.y>=lo.y && s.y<=hi.y) out.push(m); } return out; }

// ---- one gesture owner (left button; orbit is on the right, #258) -----------------------------
// Every left-button gesture is decided ONCE on pointerdown and owns the interaction through exactly
// one terminal path. Separate listeners for rubber-band, armed clip placement and handle dragging
// would all observe the same gesture and race; deciding up front is what prevents that.
//   priority: armed clip mode → clip handle → box-select
const rubber=document.getElementById('rubber'); const DRAG_PX=5, DRAG_TOL_PX=4;
let gesture=null;        // null | 'clip-place' | 'clip-handle' | 'box-select'
let gestureToken=0;      // bumped on every start/end — a queued frame carrying a stale token is a no-op
let boxStart=null;       // box-select anchor
let clipDrag=null;       // the live handle drag (source geometry is mutated in place; prePoint/preBox revert it)
let gesturePointerId=null, placeMode=null;   // the owning pointer, and the mode snapshotted at press
let pendingPointer=null, pointerRAF=0;
// Raw pointer events outrun the display. Coalesce to at most one unit of work per frame — otherwise
// every event raycasts, runs clipping tests, re-derives planes and re-points the renderer's array.
function schedulePointerWork(e){ pendingPointer={x:e.clientX,y:e.clientY,token:gestureToken};
  if(pointerRAF) return;
  pointerRAF=requestAnimationFrame(()=>{ pointerRAF=0; const p=pendingPointer; pendingPointer=null;
    if(p&&p.token===gestureToken) doPointerWork(p.x,p.y); }); }
function cancelPointerWork(){ if(pointerRAF){ cancelAnimationFrame(pointerRAF); pointerRAF=0; } pendingPointer=null; }
function flushPointerWork(){ const p=pendingPointer; cancelPointerWork();
  if(p&&p.token===gestureToken) doPointerWork(p.x,p.y); }
function doPointerWork(cx,cy){
  if(gesture==='clip-handle'){ dragClipHandle(cx,cy); return; }
  if(gesture==='box-select'){ const dx=cx-boxStart.x, dy=cy-boxStart.y;
    if(Math.hypot(dx,dy)<DRAG_PX) return;
    rubber.style.display='block'; rubber.style.left=Math.min(cx,boxStart.x)+'px'; rubber.style.top=Math.min(cy,boxStart.y)+'px';
    rubber.style.width=Math.abs(dx)+'px'; rubber.style.height=Math.abs(dy)+'px'; return; }
  if(gesture) return;
  if(clipMode==='plane') clipPlanePreviewAt(cx,cy);   // hover ghost, only while armed
  else if(clipMode==='box') clipBoxPreviewAt(cx,cy);  // live footprint / pulled box + snap reticle
}
function beginClipHandleDrag(h,cx,cy){
  const c=findClip(h.userData.clipId); if(!c) return false;
  const f=h.userData.face;
  const u=h.userData.plane?c.n.clone():new THREE.Vector3(f.axis==='x'?1:0,f.axis==='y'?1:0,f.axis==='z'?1:0);
  const lineP=h.position.clone();
  clipDrag={ clip:c, plane:!!h.userData.plane, face:f, u, lineP, t0:lineClosestT(lineP,u,rayAt(cx,cy)),
    start:[cx,cy], moved:false,
    basePoint:c.point?c.point.clone():null, prePoint:c.point?c.point.clone():null,
    startCoord:f?(f.sign>0?c.box.max[f.axis]:c.box.min[f.axis]):0, preBox:c.box?c.box.clone():null };
  return true; }
function dragClipHandle(cx,cy){ if(!clipDrag) return;
  if(!clipDrag.moved&&Math.hypot(cx-clipDrag.start[0],cy-clipDrag.start[1])<=DRAG_TOL_PX) return;
  clipDrag.moved=true;
  const c=clipDrag.clip, delta=lineClosestT(clipDrag.lineP,clipDrag.u,rayAt(cx,cy))-clipDrag.t0;
  if(clipDrag.plane){ c.point=clipDrag.basePoint.clone().addScaledVector(clipDrag.u,delta); }
  else { const f=clipDrag.face, val=clipDrag.startCoord+delta;
    // Keep min < max: a face dragged past its opposite would invert the box.
    if(f.sign>0) c.box.max[f.axis]=Math.max(val,c.box.min[f.axis]+1);
    else c.box.min[f.axis]=Math.min(val,c.box.max[f.axis]-1); }
  rebuildClipPlanes(c); applyClips(); updateClipGizmoAnchors();
  const amount=clipDrag.plane?delta:(c.box.max[clipDrag.face.axis]-c.box.min[clipDrag.face.axis]);
  readout.replaceChildren(el('b',null,(amount/304.8).toFixed(2)+' ft'),
    document.createTextNode(clipDrag.plane?' ⟂':' '+clipDrag.face.axis.toUpperCase())); }
function revertClipDrag(){ if(!clipDrag||!clipDrag.moved) return;
  const c=clipDrag.clip;
  if(clipDrag.plane) c.point=clipDrag.prePoint; else c.box.copy(clipDrag.preBox);
  rebuildClipPlanes(c); applyClips(); renderClipGizmo(); }
// Idempotent, and it clears ownership FIRST: a normal releasePointerCapture itself fires
// lostpointercapture, so a handler that reverted on capture loss would undo a drag it just committed.
function endGesture(revert){
  if(!gesture) return;
  const wasHandle=gesture==='clip-handle';
  gesture=null; gestureToken++;
  cancelPointerWork();
  rubber.style.display='none';
  if(wasHandle&&revert) revertClipDrag();
  clipDrag=null; boxStart=null; gesturePointerId=null; placeMode=null;
  controls.enabled=true;
  refreshReadout(); }
renderer.domElement.addEventListener('pointerdown', e=>{ if(e.button!==0) return;
  // A second primary press while a gesture is live would re-decide ownership and re-capture
  // prePoint/preBox from ALREADY-MUTATED geometry, so a later Escape would "revert" to the
  // half-applied state — the exact failure the single owner exists to prevent.
  if(gesture) return;
  gestureToken++; gesturePointerId=e.pointerId;
  // Snapshot the armed mode. Reading the live `clipMode` on release let an Escape pressed while the
  // button was still down disarm the tool and then fall through to placing a plane anyway.
  if(clipMode){ gesture='clip-place'; placeMode=clipMode; }
  else { const h=selectedClipIds.size?pickClipHandle(e.clientX,e.clientY):null;
    if(h&&beginClipHandleDrag(h,e.clientX,e.clientY)){ gesture='clip-handle'; controls.enabled=false; }
    else { gesture='box-select'; boxStart={x:e.clientX,y:e.clientY}; } }
  try{ renderer.domElement.setPointerCapture(e.pointerId); }catch{}   // release off-canvas must still reach us
});
renderer.domElement.addEventListener('pointermove', e=>schedulePointerWork(e));
renderer.domElement.addEventListener('pointerup', e=>{ if(e.button!==0||!gesture||e.pointerId!==gesturePointerId) return;
  flushPointerWork();   // the last few millimetres of a drag are part of the result
  const g=gesture;
  if(g==='clip-place'&&placeMode&&clipMode===placeMode){
    if(placeMode==='box') onClipBoxClick(e.clientX,e.clientY);
    // One plane per command; a MISS keeps it armed to retry. (This used to stay armed on success
    // under a comment claiming parity with floless — floless does
    // `if (addClipPlaneAtScreen(...)) setClipMode(null)`, so the comment asserted a parity that did
    // not exist.)
    else if(addClipPlaneAtScreen(e.clientX,e.clientY)){ setClipPlanePreview(null,null); setClipMode(null); }
  } else if(g==='box-select'){
    const dx=e.clientX-boxStart.x, dy=e.clientY-boxStart.y;
    if(Math.hypot(dx,dy)>=DRAG_PX) setSelection(meshesInRect(boxStart.x,boxStart.y,e.clientX,e.clientY));
    else { setSelectedClips([]); pickAt(e.clientX,e.clientY); }   // a canvas pick also drops any clip selection
  }
  endGesture(false); });   // pointerup COMMITS
// pointercancel is the ONLY revert: the drag mutated live geometry and the gesture is genuinely over.
renderer.domElement.addEventListener('pointercancel', e=>{ if(e.pointerId===gesturePointerId) endGesture(true); });
// Capture loss is NOT cancellation. OrbitControls binds capture on the SAME element and pointer id,
// and its pointerup handler has no `enabled` guard — so pressing a second mouse button during a clip
// drag makes it release OUR capture, which as a revert silently discarded the user's live edit.
// Re-acquire instead, and only give up if that fails.
renderer.domElement.addEventListener('lostpointercapture', e=>{
  if(!gesture||e.pointerId!==gesturePointerId) return;
  try{ renderer.domElement.setPointerCapture(e.pointerId); }catch{ endGesture(true); } });
renderer.domElement.addEventListener('pointerleave', ()=>{ if(gesture) return;
  if(clipMode==='plane') setClipPlanePreview(null,null);
  else if(clipMode==='box') clearClipDrawHints(); });   // leaving mid-draw must not strand the reticle + footprint

// ---- clip planes / boxes + work area (Tekla-style sectioning) ----
// Sectioning lives in renderer.clippingPlanes (GLOBAL), so it clips the grid + every element like
// Tekla and survives a re-render. A clip PLANE keeps the camera-far side (1 plane); a clip BOX and
// the work area keep INSIDE (6 inward planes). The ViewCube has its own renderer → never clipped.
// A clip keeps its SOURCE geometry (a plane's normal + point, a box's Box3) and DERIVES `.planes`
// from it. Storing only the derived planes — as this did — makes a clip unmanipulable: there is
// nothing for a drag handle to move, and a box's Box3 was thrown away the moment it was built.
// Every mutation edits the source and re-derives; nothing ever edits `.planes` directly.
//   { id, kind:'plane'|'box', enabled, label, n?:Vector3, point?:Vector3, box?:Box3, planes:[Plane] }
const EMPTY_CLIPS=Object.freeze([]);
let clips=[]; let workArea=null; let clipMode=null; let clipSeq=0;
let selectedClipIds=new Set();   // drives the gizmo only — selection never changes what is cut
const overlayScene=new THREE.Scene(); let workAreaHelper=null; // work-area wireframe → 2nd UNCLIPPED pass
// three.js convention: a material is KEPT where distanceToPoint(p) = normal·p + constant >= 0 (the
// side the normal points toward) and discarded on the negative side. So INWARD normals + these
// constants keep the box interior (e.g. normal -X, constant max.x → keep x<=max.x). Verified live
// (a whole-model box keeps the model visible) — do not "reverse" these signs.
function boxToPlanes(b){ return [
  new THREE.Plane(new THREE.Vector3(-1,0,0), b.max.x), new THREE.Plane(new THREE.Vector3(1,0,0), -b.min.x),
  new THREE.Plane(new THREE.Vector3(0,-1,0), b.max.y), new THREE.Plane(new THREE.Vector3(0,1,0), -b.min.y),
  new THREE.Plane(new THREE.Vector3(0,0,-1), b.max.z), new THREE.Plane(new THREE.Vector3(0,0,1), -b.min.z) ]; }
function applyClips(){ const active=clips.filter(c=>c.enabled).flatMap(c=>c.planes);
  // The work area sections the view too — UNLESS it is in "show whole parts" mode, where parts are
  // hidden or shown whole by applyGroupVisibility and never sliced.
  if(workArea && workArea.enabled && !workArea.whole) active.push(...workArea.planes);
  renderer.clippingPlanes=active.length?active:EMPTY_CLIPS; syncClipMirror(); }
// Re-derive one clip's planes from its source geometry — the ONLY writer of `.planes`.
function rebuildClipPlanes(c){ c.planes=c.kind==='box'?boxToPlanes(c.box):[new THREE.Plane().setFromNormalAndCoplanarPoint(c.n,c.point)]; }
// Materials keep their OWN reference to the clip planes for the shadow pass, since renderer-global
// ones are cleared there. applyClips REPLACES the array rather than mutating it, so that reference
// has to be re-pointed on every clip change: a stale one leaves the model visibly clipped after the
// clip was cleared, and leaves the shadow pass cutting against the previous set after a new one.
function syncClipMirror(){
  const gp=renderer.clippingPlanes;
  const use=(shadowsEnabled && gp && gp.length) ? gp : null;
  for(const m of pickable) if(m.material) m.material.clippingPlanes=use;
}
function meshBox(meshes){ const b=new THREE.Box3(); for(const m of meshes){ if(m.visible) b.expandByObject(m); } return b; } // real mesh bounds incl. section width (sceneBox is centreline-only)
function selBox(pad){ let box=meshBox(selection); if(box.isEmpty()) box=meshBox(pickable); if(box.isEmpty()) return null;
  return box.expandByScalar(pad==null?Math.max(maxDim*0.04,1):pad); }
// A point is clipped when it falls on the cut-away side of ANY active plane (global clipping is a
// union: a fragment outside any plane is removed). THREE.Raycaster does NOT honour
// renderer.clippingPlanes, so without this a pick lands on a face that is visually gone and drops a
// clip plane in empty space. Filtering here is a deliberate improvement, not a port — floless's own
// clipPlaneAtScreen takes the first raw hit.
const _clipPt=new THREE.Vector3();
function isPointClipped(p){ const planes=renderer&&renderer.clippingPlanes;
  if(!planes||!planes.length||!p) return false;
  _clipPt.copy(p);
  for(let i=0;i<planes.length;i++) if(planes[i].distanceToPoint(_clipPt)<0) return true;
  return false; }
// The plane a face-pick WOULD produce at (cx,cy): {n (keep-side normal, pointing away from the
// camera), point} or null on a miss. Shared by the commit and the hover ghost so the preview can
// never disagree with where the click actually lands.
function clipPlaneAtScreen(cx,cy){
  const ndc=new THREE.Vector2((cx/innerWidth)*2-1, -(cy/innerHeight)*2+1); ray.setFromCamera(ndc,camera);
  const hit=ray.intersectObjects(pickable.filter(m=>m.visible),false).find(h=>h.face&&!isPointClipped(h.point));
  if(!hit) return null;
  const n=hit.face.normal.clone().transformDirection(hit.object.matrixWorld).normalize();
  if(n.dot(camera.position.clone().sub(hit.point))>0) n.negate();   // away from the camera → keep the FAR side, revealing the section
  return { n, point:hit.point.clone() }; }
// Push a fully-formed clip, derive its planes, select it, and report the id.
function addClip(c){ rebuildClipPlanes(c); clips.push(c); applyClips(); selectClip(c.id); return c.id; }
// A clip plane from a clicked face (screen px): keep the camera-FAR side so the cut reveals the section.
function addClipPlaneAtScreen(cx,cy){
  const pp=clipPlaneAtScreen(cx,cy); if(!pp) return null;
  return addClip({ id:'clip'+(++clipSeq), kind:'plane', enabled:true, label:'Plane '+clipSeq, n:pp.n.clone(), point:pp.point.clone(), planes:[] }); }
// A clip box around the current selection (or the whole model when nothing is selected).
function addClipBox(pad){ const box=selBox(pad); if(!box) return null; return addClipBoxFromBox(box); }
function addClipBoxFromBox(box){ if(!box||box.isEmpty()) return null;
  return addClip({ id:'clip'+(++clipSeq), kind:'box', enabled:true, label:'Box '+clipSeq, box:box.clone(), planes:[] }); }
function clearClips(){ if(!clips.length) return; clips=[]; selectedClipIds.clear(); applyClips(); renderClipGizmo(); refreshClipList(); }
function clipCount(){ return clips.length; }
// ---- clip lifecycle: mutate the source, re-derive, re-render the gizmo + list ----
function findClip(id){ return clips.find(c=>c.id===id)||null; }
function toggleClip(id,on){ const c=findClip(id); if(!c) return; c.enabled=on===undefined?!c.enabled:!!on; applyClips(); renderClipGizmo(); refreshClipList(); }
// Reject a blank name, or one another clip already uses (case-insensitive) — the caller keeps the old
// name and says why, rather than silently accepting a duplicate that makes two rows indistinguishable.
function renameClip(id,name){ const c=findClip(id); if(!c) return false;
  const nm=String(name||'').trim(); if(!nm) return false;
  if(clips.some(x=>x.id!==id&&x.label.toLowerCase()===nm.toLowerCase())) return false;
  c.label=nm; refreshClipList(); return true; }
function removeClip(id){ if(!findClip(id)) return; clips=clips.filter(c=>c.id!==id); selectedClipIds.delete(id); applyClips(); renderClipGizmo(); refreshClipList(); }
// Selection drives the gizmo ONLY — it never calls applyClips, because selecting a clip must not
// change what is cut. Unknown ids are dropped rather than kept as phantom selection.
function setSelectedClips(ids){ selectedClipIds=new Set((ids||[]).filter(id=>clips.some(c=>c.id===id))); renderClipGizmo(); refreshClipList(); }
function selectClip(id){ setSelectedClips(id?[id]:[]); }
function selectedClips(){ return [...selectedClipIds]; }
function deleteSelectedClips(){ if(!selectedClipIds.size) return; clips=clips.filter(c=>!selectedClipIds.has(c.id)); selectedClipIds.clear(); applyClips(); renderClipGizmo(); refreshClipList(); }
// The read model for the list + the test probe. EXACT values, not rounded: a probe that rounds
// cannot prove a snapped bound or which face a drag moved.
function getClips(){ return clips.map(c=>({ id:c.id, kind:c.kind, enabled:c.enabled, label:c.label,
  selected:selectedClipIds.has(c.id),
  box:c.box?{min:c.box.min.toArray(),max:c.box.max.toArray()}:null,
  plane:c.n?{n:c.n.toArray(),point:c.point.toArray()}:null })); }
// ---- clip visuals: hover ghost + the manipulator ----------------------------------------------
// ALL of it lives in overlayScene, which renders in a 2nd pass with clipping OFF. A gizmo added to
// `scene` would be sectioned by the very planes it exists to move.
// Mirrors of the --clip-* palette tokens (Three.js needs numbers, CSS needs the hex).
const CLIP_PLANE_COLOR=0x3b82f6, CLIP_BOX_COLOR=0x93c5fd, CLIP_PREVIEW_COLOR=0xbfdbfe;
const CLIP_PATCH_R=304.8;   // 1 ft half-size → a 2'×2' marker. Ghost and placed outline share it, so the preview sits exactly where the click will land.
const _UPV=new THREE.Vector3(0,1,0), _XV=new THREE.Vector3(1,0,0), _ZV=new THREE.Vector3(0,0,1);
// The in-plane square of half-size r around hp. The seed vector must not be parallel to the normal
// or the cross product degenerates — and the rendered world is Y-UP, so the test is on n.y. A z-up
// port testing n.z picks the wrong seed for a VERTICAL cut and for nothing else, which is exactly
// the kind of bug that ships looking fine.
function planePatchCorners(hp,n,r){
  const u=(Math.abs(n.y)<0.9?_UPV:_XV).clone().cross(n).normalize();
  const v=n.clone().cross(u).normalize();
  return [ hp.clone().addScaledVector(u, r).addScaledVector(v, r),
           hp.clone().addScaledVector(u,-r).addScaledVector(v, r),
           hp.clone().addScaledVector(u,-r).addScaledVector(v,-r),
           hp.clone().addScaledVector(u, r).addScaledVector(v,-r) ]; }
// The ghost is allocated ONCE and moved; rebuilding geometry every hover frame is pure churn.
let clipGhost=null;
function ensureClipGhost(){ if(clipGhost) return clipGhost;
  const group=new THREE.Group(); group.visible=false;
  const fill=new THREE.Mesh(new THREE.PlaneGeometry(CLIP_PATCH_R*2,CLIP_PATCH_R*2),
    new THREE.MeshBasicMaterial({color:CLIP_PREVIEW_COLOR,transparent:true,opacity:0.3,side:THREE.DoubleSide}));
  fill.material.depthTest=false; fill.renderOrder=995;
  const og=new THREE.BufferGeometry(); og.setAttribute('position',new THREE.BufferAttribute(new Float32Array(12),3));
  const outline=new THREE.LineLoop(og,new THREE.LineBasicMaterial({color:CLIP_PREVIEW_COLOR}));
  outline.material.depthTest=false; outline.renderOrder=996;
  group.add(fill,outline); overlayScene.add(group);
  clipGhost={group,fill,outline}; return clipGhost; }
function setClipPlanePreview(hp,n){ const G=ensureClipGhost();
  if(!hp||!n){ G.group.visible=false; return; }
  G.fill.position.copy(hp); G.fill.quaternion.setFromUnitVectors(_ZV,n);
  const c=planePatchCorners(hp,n,CLIP_PATCH_R), a=G.outline.geometry.attributes.position;
  for(let i=0;i<4;i++) a.setXYZ(i,c[i].x,c[i].y,c[i].z);
  a.needsUpdate=true; G.outline.geometry.computeBoundingSphere(); G.group.visible=true; }
function clipPlanePreviewAt(cx,cy){ const pp=clipPlaneAtScreen(cx,cy); setClipPlanePreview(pp?pp.point:null,pp?pp.n:null); }

// Screen-constant sizing. Projected size depends on CAMERA-SPACE DEPTH, not Euclidean distance to
// the camera: distance oversizes a handle sitting off-axis, and then what is drawn no longer matches
// the fixed-pixel picker below — handles become grabbable where they are not shown.
const _camPt=new THREE.Vector3();
function pxToWorldAt(px,pos){ const h=renderer.domElement.clientHeight||1;
  if(camera.isOrthographicCamera) return px*((camera.top-camera.bottom)/(camera.zoom||1))/h;
  _camPt.copy(pos).applyMatrix4(camera.matrixWorldInverse);
  return px*(2*Math.tan(THREE.MathUtils.degToRad(perspCam.fov)/2)*(Math.abs(_camPt.z)||1e-6))/h; }
function rayAt(cx,cy){ const ndc=new THREE.Vector2((cx/innerWidth)*2-1,-(cy/innerHeight)*2+1);
  ray.setFromCamera(ndc,camera); return ray.ray.clone(); }
// Parameter along the line P + t·u closest to `r`. Degenerate (line ∥ ray) → project onto u.
function lineClosestT(P,u,r){ const w0=P.clone().sub(r.origin), d=r.direction;
  const a=u.dot(u), b=u.dot(d), c=d.dot(d), dd=u.dot(w0), e=d.dot(w0), den=a*c-b*b;
  return Math.abs(den)<1e-6 ? u.dot(w0) : (b*e-c*dd)/den; }

const FACE_AXES=[{axis:'x',sign:1},{axis:'x',sign:-1},{axis:'y',sign:1},{axis:'y',sign:-1},{axis:'z',sign:1},{axis:'z',sign:-1}];
let clipGizmo=null;
function disposeSubtree(o){ o.traverse(c=>{ if(c.geometry)c.geometry.dispose();
  const mm=Array.isArray(c.material)?c.material:(c.material?[c.material]:[]); for(const m of mm) m.dispose(); }); }
// Persistent per SELECTION, not per frame — but a user cycling selections would still leak GPU
// resources without this, so the gizmo is disposed whenever it is replaced or cleared. The ghost and
// (later) the draw preview are session-lived and merely hidden, so they are exempt.
function clearClipGizmo(){ if(!clipGizmo) return; overlayScene.remove(clipGizmo); disposeSubtree(clipGizmo); clipGizmo=null; }
function renderClipGizmo(){ clearClipGizmo();
  if(!selectedClipIds.size) return;
  clipGizmo=new THREE.Group(); overlayScene.add(clipGizmo);
  // One manipulator shape for a plane AND each box face: a translucent disc lying in the plane plus a
  // normal arrow. Grab either and slide along `normal`. The stem is deliberately NOT flagged
  // clipHandle, so it draws but never steals a grab from the disc or the cone.
  const addHandle=(anchor,normal,ud,color)=>{
    const disc=new THREE.Mesh(new THREE.CircleGeometry(1,32),
      new THREE.MeshBasicMaterial({color,transparent:true,opacity:0.32,side:THREE.DoubleSide}));
    disc.material.depthTest=false; disc.renderOrder=998; disc.position.copy(anchor);
    disc.quaternion.setFromUnitVectors(_ZV,normal); disc.userData={clipHandle:true,disc:true,...ud};
    const sg=new THREE.BufferGeometry(); sg.setAttribute('position',new THREE.BufferAttribute(new Float32Array(6),3));
    const stem=new THREE.Line(sg,new THREE.LineBasicMaterial({color}));
    stem.material.depthTest=false; stem.renderOrder=998;
    stem.userData={clipStem:true,baseHp:anchor.clone(),normal:normal.clone()};
    const cone=new THREE.Mesh(new THREE.ConeGeometry(0.5,1.5,18),new THREE.MeshBasicMaterial({color}));
    cone.material.depthTest=false; cone.renderOrder=999;
    cone.quaternion.setFromUnitVectors(_UPV,normal);   // ConeGeometry is Y-axial
    cone.userData={clipHandle:true,arrow:true,baseHp:anchor.clone(),normal:normal.clone(),...ud};
    clipGizmo.add(disc,stem,cone); };
  for(const c of clips){ if(!selectedClipIds.has(c.id)) continue;
    if(c.kind==='box'){
      const helper=new THREE.Box3Helper(c.box,new THREE.Color(CLIP_BOX_COLOR));
      helper.material.depthTest=false; helper.renderOrder=996; clipGizmo.add(helper);
      const ctr=c.box.getCenter(new THREE.Vector3());
      for(const f of FACE_AXES){ const anchor=ctr.clone();
        anchor[f.axis]=f.sign>0?c.box.max[f.axis]:c.box.min[f.axis];
        const nrm=new THREE.Vector3(f.axis==='x'?f.sign:0,f.axis==='y'?f.sign:0,f.axis==='z'?f.sign:0);
        addHandle(anchor,nrm,{face:f,clipId:c.id},CLIP_BOX_COLOR); }
    } else {
      const hp=c.point.clone();
      const outline=new THREE.LineLoop(new THREE.BufferGeometry().setFromPoints(planePatchCorners(hp,c.n,CLIP_PATCH_R)),
        new THREE.LineBasicMaterial({color:CLIP_PLANE_COLOR}));
      outline.material.depthTest=false; outline.renderOrder=996; clipGizmo.add(outline);
      addHandle(hp,c.n.clone(),{plane:true,clipId:c.id},CLIP_PLANE_COLOR); } }
  sizeClipHandles(); }
// Re-anchor the existing handles to the mutated geometry. Only the anchors move during a drag, so
// the full renderClipGizmo() (which disposes and reallocates every geometry and material) is for
// selection changes, not for every frame of a drag.
function updateClipGizmoAnchors(){ if(!clipGizmo) return;
  for(const h of clipGizmo.children){ const ud=h.userData; if(!ud) continue;
    const c=ud.clipId?findClip(ud.clipId):null; if(!c) continue;
    let anchor=null;
    if(ud.plane) anchor=c.point.clone();
    else if(ud.face&&c.box){ const ctr=c.box.getCenter(new THREE.Vector3());
      anchor=ctr.clone(); anchor[ud.face.axis]=ud.face.sign>0?c.box.max[ud.face.axis]:c.box.min[ud.face.axis]; }
    if(!anchor) continue;
    if(ud.disc) h.position.copy(anchor);
    ud.baseHp=anchor; }
  // The stems carry no clipId, so re-anchor them from their sibling handle's normal + the same rule.
  for(const h of clipGizmo.children){ const ud=h.userData; if(!ud||!ud.clipStem) continue;
    const near=clipGizmo.children.find(o=>o.userData&&o.userData.clipHandle&&o.userData.normal&&o.userData.normal.equals(ud.normal));
    if(near&&near.userData.baseHp) ud.baseHp=near.userData.baseHp.clone(); }
  sizeClipHandles(); }
function sizeClipHandles(){ if(!clipGizmo) return;
  for(const h of clipGizmo.children){ const ud=h.userData; if(!ud) continue;
    if(ud.arrow){ const off=pxToWorldAt(34,ud.baseHp);
      h.position.copy(ud.baseHp).addScaledVector(ud.normal,off); h.scale.setScalar(pxToWorldAt(11,h.position)); }
    else if(ud.disc) h.scale.setScalar(pxToWorldAt(14,h.position));
    else if(ud.clipStem){ const off=pxToWorldAt(34,ud.baseHp), a=h.geometry.attributes.position;
      const tip=ud.baseHp.clone().addScaledVector(ud.normal,off);
      a.setXYZ(0,ud.baseHp.x,ud.baseHp.y,ud.baseHp.z); a.setXYZ(1,tip.x,tip.y,tip.z);
      a.needsUpdate=true; h.geometry.computeBoundingSphere(); } } }
// Screen-space nearest, no raycast — a handle buried inside geometry is still grabbable, which is
// consistent with drawing it depthTest:false.
function pickClipHandle(cx,cy){ if(!clipGizmo) return null;
  let best=null,bestD=16;
  for(const h of clipGizmo.children){ if(!h.userData||!h.userData.clipHandle) continue;
    const v=h.position.clone().project(camera); if(v.z>1) continue;
    const sx=(v.x*0.5+0.5)*innerWidth, sy=(-v.y*0.5+0.5)*innerHeight, d=Math.hypot(sx-cx,sy-cy);
    if(d<bestD){ bestD=d; best=h; } }
  return best; }
function clipHandlesScreen(){ const out=[]; if(!clipGizmo) return out;
  for(const h of clipGizmo.children){ const ud=h.userData; if(!ud||!ud.clipHandle) continue;
    const v=h.position.clone().project(camera);
    out.push({ clipId:ud.clipId, plane:!!ud.plane, arrow:!!ud.arrow,
      axis:ud.face?ud.face.axis:null, sign:ud.face?ud.face.sign:0,
      x:(v.x*0.5+0.5)*innerWidth, y:(-v.y*0.5+0.5)*innerHeight, behind:v.z>1 }); }
  return out; }
// ---- snapping (for the clip-box draw) ----------------------------------------------------------
// The rendered world is Y-UP, so the vertical index is 1 and the PLAN axes are X and Z. The floless
// original this is ported from is native Z-up and hardcodes index 2 as "the elevation to preserve";
// copying it verbatim would snap in the wrong plane. Everything below is parameterized on UP_I.
const UP_I=1, PLAN_I=[0,2], SNAP_TOL_PX=10;
const SCENE_UP=(SCENE.meta&&SCENE.meta.up)||'z';
// Endpoint beats intersection beats centre-line beats axis beats grid — CAD convention. Distance is
// primary; this only breaks ties.
const PRECEDENCE={vertex:0,origin:0,intersection:1,'grid-int':1,level:1,midpoint:2,centerline:3,'vertical-axis':4,'grid-line':5};
function closestOnSeg(p,a,b){ const ab=[b[0]-a[0],b[1]-a[1],b[2]-a[2]];
  const len2=ab[0]*ab[0]+ab[1]*ab[1]+ab[2]*ab[2]||1;
  let t=((p[0]-a[0])*ab[0]+(p[1]-a[1])*ab[1]+(p[2]-a[2])*ab[2])/len2;
  t=Math.max(0,Math.min(1,t));
  return [a[0]+ab[0]*t,a[1]+ab[1]*t,a[2]+ab[2]*t]; }
// The point a candidate proposes for `dragged`. A grid or vertical axis steers the PLAN only — it
// must never yank the elevation, which is what makes UP_I load-bearing here.
function candidatePoint(c,dragged){
  if(c.type==='vertex'||c.type==='intersection'||c.type==='midpoint'||c.type==='level'||c.type==='origin') return c.p;
  if(c.type==='vertical-axis'||c.type==='grid-int'){ const q=c.p.slice(); q[UP_I]=dragged[UP_I]; return q; }
  if(c.type==='grid-line'){ const q=closestOnSeg(dragged,c.a,c.b); q[UP_I]=dragged[UP_I]; return q; }
  return closestOnSeg(dragged,c.a,c.b); }
const isFixedCand=c=>c.type==='vertex'||c.type==='intersection'||c.type==='midpoint'||c.type==='level'||c.type==='origin';
function toScreenPt(p){ const v=new THREE.Vector3(p[0],p[1],p[2]).project(camera);
  return { x:(v.x*0.5+0.5)*innerWidth, y:(-v.y*0.5+0.5)*innerHeight }; }
// Snap `dragged` to the nearest candidate within tolPx SCREEN pixels.
// `proj` is the projection cache for THESE candidates, passed explicitly. It used to be read off a
// module global, which silently indexed one array's cache with another array's positions the moment
// a second candidate set (the height levels) was passed — so a level snapped whenever some unrelated
// vertex happened to project near the cursor. Out of range it was worse: clipProj[i*2] is undefined,
// d becomes NaN, `NaN > tolPx` is FALSE so the candidate was never skipped, and it then won the sort.
function snapPoint(dragged,candidates,tolPx,proj){
  const cache=(proj&&proj.length>=candidates.length*2)?proj:null;
  const ds=toScreenPt(dragged), hits=[];
  for(let i=0;i<candidates.length;i++){ const c=candidates[i];
    const p=candidatePoint(c,dragged);
    // Fixed candidates project to a constant screen point, so their projection is cached and only
    // recomputed when the camera changes; the line types depend on `dragged` and cannot be.
    const s=(isFixedCand(c)&&cache)?{x:cache[i*2],y:cache[i*2+1]}:toScreenPt(p);
    const d=Math.hypot(s.x-ds.x,s.y-ds.y);
    if(!(d<=tolPx)) continue;   // NOT `d>tolPx`: that lets a NaN through
    hits.push({c,p,d,rank:PRECEDENCE[c.type]??9}); }
  if(!hits.length) return { snapped:dragged, candidate:null };
  // Distance first, in ~1.5px buckets so a near-tie falls through to precedence.
  hits.sort((A,B)=>(Math.round(A.d/1.5)-Math.round(B.d/1.5))||(A.rank-B.rank)||(A.d-B.d));
  return { snapped:hits[0].p, candidate:hits[0].c }; }
// Projected-position cache for the fixed candidates. Stale projections do not merely cost time —
// they snap to the wrong place — and "the camera moved" is not the only way they go stale: a resize
// changes the projection matrix, setProjection swaps the camera OBJECT, and reframeOrtho changes the
// frustum and zoom without moving anything.
let clipProj=null;
function invalidateClipProjectionCache(){ clipProj=null; }
function buildClipProjection(cands){ clipProj=new Float32Array(cands.length*2);
  for(let i=0;i<cands.length;i++){ if(!isFixedCand(cands[i])) continue;
    const s=toScreenPt(cands[i].p); clipProj[i*2]=s.x; clipProj[i*2+1]=s.y; } }

// ---- the 3-click clip-box draw -----------------------------------------------------------------
// CAD "rectangle then extrude": two floor corners, then pull the height and click to commit.
let clipBoxDraft=null;              // null → {a,b:null} → {a,b} → committed
let clipSnapCands=null, clipLevels=null;   // built ONCE per draw (the scene is immutable after renderScene)
let clipPreview=null, clipReticle=null;
// The draw floor is the bottom of the MODEL. sceneBox is wrong for this — expandSceneBounds folds in
// structural-grid bounds and every level elevation, so its minimum is the lowest grid datum, which
// can sit far below the lowest member. meshBox is wrong too: it filters on m.visible, so hiding the
// lowest member would move the floor under the user mid-session.
function allMeshBounds(){ const b=new THREE.Box3(); for(const m of pickable) b.expandByObject(m); return b; }
function clipBoxFloorY(){ const b=allMeshBounds();
  if(!b.isEmpty()) return b.min.y;
  return sceneBox.isEmpty()?0:sceneBox.min.y; }
function clipBoxFrom(a,b,ylo,ytop){ return new THREE.Box3(
  new THREE.Vector3(Math.min(a[0],b[0]),Math.min(ylo,ytop),Math.min(a[1],b[1])),
  new THREE.Vector3(Math.max(a[0],b[0]),Math.max(ylo,ytop),Math.max(a[1],b[1]))); }
// Cursor ray ∩ the horizontal plane at world Y. Null when the ray is parallel to it.
function rayToFloor(cx,cy,y){ const r=rayAt(cx,cy);
  if(Math.abs(r.direction.y)<1e-9) return null;
  const t=(y-r.origin.y)/r.direction.y; if(!(t>0)) return null;
  return [r.origin.x+r.direction.x*t, y, r.origin.z+r.direction.z*t]; }
// Candidates come from the model itself, not from an authored grid alone — a model with no
// structural grid must still snap to its own steel.
function buildClipCandidates(){
  const out=[], seg=[];
  const push=(type,p)=>out.push({type,p});
  for(const m of pickable){ const e=m.userData; if(!e) continue;
    // axisEnds normalizes every geometry form the viewer supports; raw from/to misses rods,
    // fasteners and node-like records.
    const A=axisEnds(e);
    if(A){ const a=conv(A[0],SCENE_UP), b=conv(A[1],SCENE_UP);
      push('vertex',[a.x,a.y,a.z]); push('vertex',[b.x,b.y,b.z]);
      push('midpoint',[(a.x+b.x)/2,(a.y+b.y)/2,(a.z+b.z)/2]);
      const d=new THREE.Vector3().subVectors(b,a);
      const vertical=Math.abs(d.y)>Math.max(Math.abs(d.x),Math.abs(d.z));
      if(vertical) push('vertical-axis',[a.x,a.y,a.z]);
      else seg.push([[a.x,a.y,a.z],[b.x,b.y,b.z]]);
      out.push({type:'centerline',a:[a.x,a.y,a.z],b:[b.x,b.y,b.z]});
    } else if(vec3(e.at)||vec3(e.center)){ const c=conv(e.at||e.center,SCENE_UP); push('vertex',[c.x,c.y,c.z]); } }
  // Pairwise intersections of non-vertical axes, in the PLAN (x/z). Bounded to both segments, and
  // reported at the draw floor — a corner pick rides the floor plane.
  const fy=clipBoxFloorY(), seen=new Set();
  for(let i=0;i<seg.length;i++) for(let j=i+1;j<seg.length;j++){
    const [p1,p2]=seg[i], [p3,p4]=seg[j];
    const x1=p1[PLAN_I[0]],y1=p1[PLAN_I[1]],x2=p2[PLAN_I[0]],y2=p2[PLAN_I[1]];
    const x3=p3[PLAN_I[0]],y3=p3[PLAN_I[1]],x4=p4[PLAN_I[0]],y4=p4[PLAN_I[1]];
    const den=(x1-x2)*(y3-y4)-(y1-y2)*(x3-x4); if(Math.abs(den)<1e-9) continue;
    const t=((x1-x3)*(y3-y4)-(y1-y3)*(x3-x4))/den, u=((x1-x3)*(y1-y2)-(y1-y3)*(x1-x2))/den;
    if(t<0||t>1||u<0||u>1) continue;
    const ix=x1+t*(x2-x1), iz=y1+t*(y2-y1), key=Math.round(ix)+'|'+Math.round(iz);
    if(seen.has(key)) continue; seen.add(key);
    push('intersection',[ix,fy,iz]); }
  // Grid axes + their cross-direction intersections, from the SAME segments the renderer draws.
  const gridSegs=[];
  for(const R of (SCENE.referenceSystems||[])){ if(!R||R.kind!=='structural-grid') continue;
    const s=referenceSystemSegments(R);
    for(const a of s.axes){ out.push({type:'grid-line',a:[a.a.x,a.a.y,a.a.z],b:[a.b.x,a.b.y,a.b.z],direction:a.direction});
      gridSegs.push({direction:a.direction,a:a.a,b:a.b}); } }
  for(let i=0;i<gridSegs.length;i++) for(let j=i+1;j<gridSegs.length;j++){
    const g=gridSegs[i], h=gridSegs[j]; if(g.direction===h.direction) continue;   // only cross-direction axes meet
    const gx=Math.abs(g.a.x-g.b.x)<1e-6, hx=Math.abs(h.a.x-h.b.x)<1e-6;
    if(gx===hx) continue;
    const vert=gx?g:h, horiz=gx?h:g;
    const x=vert.a.x, z=horiz.a.z;
    // An axis may carry its own startMm/endMm, so two axes of different directions do not necessarily
    // cross where their infinite lines would. Snapping there points at nothing on screen.
    const within=(lo,hi,v)=>v>=Math.min(lo,hi)-1e-6&&v<=Math.max(lo,hi)+1e-6;
    if(!within(vert.a.z,vert.b.z,z)||!within(horiz.a.x,horiz.b.x,x)) continue;
    push('grid-int',[x,fy,z]); }
  return out; }
// Height levels: every distinct element-endpoint elevation, PLUS any authored grid datums. Grid
// levels alone would leave a model with no structural grid unable to snap to its own steel.
function buildClipLevels(){
  const fy=clipBoxFloorY(), ys=new Set();
  for(const m of pickable){ const e=m.userData; if(!e) continue; const A=axisEnds(e); if(!A) continue;
    for(const p of A){ const w=conv(p,SCENE_UP); ys.add(Math.round(w.y*1000)/1000); } }
  for(const R of (SCENE.referenceSystems||[])){ if(!R||R.kind!=='structural-grid') continue;
    for(const l of referenceSystemSegments(R).levels) ys.add(Math.round(l.y*1000)/1000); }
  // A level below the floor would produce a downward box.
  return [...ys].filter(y=>y>=fy+1).sort((a,b)=>a-b); }
function clipDrawCands(){ if(!clipSnapCands){ try{ clipSnapCands=buildClipCandidates(); }
    catch(e){ console.warn('viewer-3d: snap candidates unavailable —',e); clipSnapCands=[]; } clipProj=null; }
  if(!clipProj) buildClipProjection(clipSnapCands);
  return clipSnapCands; }
function clipDrawLevels(){ if(!clipLevels){ try{ clipLevels=buildClipLevels(); }
    catch(e){ console.warn('viewer-3d: height levels unavailable —',e); clipLevels=[]; } } return clipLevels; }
// A footprint corner: the floor point under the cursor, pulled to a snap when one is screen-near.
// Only the PLAN components are kept — the corner rides the floor plane — but `world` carries the
// full snapped point so the reticle can sit at the real target.
function clipBoxFloorPoint(cx,cy){
  const fy=clipBoxFloorY(), g=rayToFloor(cx,cy,fy); if(!g) return null;
  const r=snapPoint(g,clipDrawCands(),SNAP_TOL_PX,clipProj);
  return r.candidate ? { xz:[r.snapped[PLAN_I[0]],r.snapped[PLAN_I[1]]], snap:r.candidate.type, world:r.snapped }
                     : { xz:[g[PLAN_I[0]],g[PLAN_I[1]]], snap:null, world:g }; }
// The box top, pulled along the vertical line through the footprint centre. `usable` is false in an
// axial view, where a vertical axis projects to (almost) a point and no height can be meant.
function clipBoxHeightAt(cx,cy){
  const fy=clipBoxFloorY(), a=clipBoxDraft.a, b=clipBoxDraft.b;
  const mx=(a[0]+b[0])/2, mz=(a[1]+b[1])/2;
  const base=new THREE.Vector3(mx,fy,mz);
  const s0=toScreenPt([mx,fy,mz]), s1=toScreenPt([mx,fy+304.8,mz]);
  if(Math.hypot(s1.x-s0.x,s1.y-s0.y)<4) return { y:fy+1, snap:null, usable:false };
  const raw=fy+Math.max(lineClosestT(base,_UPV,rayAt(cx,cy)),1);
  const levels=clipDrawLevels().map(y=>({type:'level',p:[mx,y,mz]}));
  // Level candidates sit at the CURRENT footprint centre, so they move with the draft and are not
  // cacheable at all — pass no cache rather than a mismatched one.
  const r=snapPoint([mx,raw,mz],levels,SNAP_TOL_PX);
  // Re-clamp AFTER snapping: an accepted level below the floor would invert the box.
  const y=Math.max(r.candidate?r.snapped[UP_I]:raw, fy+1);
  return { y, snap:r.candidate?[mx,y,mz]:null, usable:true }; }

// A camera-facing reticle whose glyph names the snap type — a bare dot vanishes against dark
// background and coloured steel. Screen-constant, and depthTest/depthWrite OFF: the overlay pass
// reuses the main depth buffer, so living in overlayScene is not on its own enough to stay visible.
const RETICLE_PX=44, RETICLE_COLOR='#38bdf8';   // --accent-2, the palette's cyan
const reticleTex={};
function reticleFor(type){
  const key=type||'dot'; if(reticleTex[key]) return reticleTex[key];
  const S=128, cv=document.createElement('canvas'); cv.width=cv.height=S;
  const g=cv.getContext('2d'), c=S/2;
  const stroke=(draw,w,col)=>{ g.lineWidth=w; g.strokeStyle=col; g.beginPath(); draw(); g.stroke(); };
  const both=(draw)=>{ stroke(draw,9,'rgba(2,8,23,.85)'); stroke(draw,4,RETICLE_COLOR); };   // dark halo, then cyan → reads on any background
  both(()=>{ g.moveTo(c-34,c); g.lineTo(c-14,c); g.moveTo(c+14,c); g.lineTo(c+34,c);
             g.moveTo(c,c-34); g.lineTo(c,c-14); g.moveTo(c,c+14); g.lineTo(c,c+34); });
  const glyph={ vertex:()=>{ g.rect(c-11,c-11,22,22); },
    intersection:()=>{ g.moveTo(c-11,c-11); g.lineTo(c+11,c+11); g.moveTo(c+11,c-11); g.lineTo(c-11,c+11); },
    'grid-int':()=>{ g.moveTo(c-11,c-11); g.lineTo(c+11,c+11); g.moveTo(c+11,c-11); g.lineTo(c-11,c+11); },
    midpoint:()=>{ g.moveTo(c-12,c+9); g.lineTo(c+12,c+9); g.lineTo(c,c-11); g.closePath(); },
    centerline:()=>{ g.moveTo(c-11,c-11); g.lineTo(c+11,c-11); g.lineTo(c-11,c+11); g.lineTo(c+11,c+11); g.closePath(); },
    'grid-line':()=>{ g.moveTo(c-11,c-11); g.lineTo(c+11,c-11); g.lineTo(c-11,c+11); g.lineTo(c+11,c+11); g.closePath(); },
    level:()=>{ g.moveTo(c-13,c); g.lineTo(c+13,c); },
    'vertical-axis':()=>{ g.moveTo(c,c-13); g.lineTo(c,c+13); } }[key];
  if(glyph) both(glyph);
  const t=new THREE.CanvasTexture(cv); reticleTex[key]=t; return t; }
function ensureReticle(){ if(clipReticle) return clipReticle;
  clipReticle=new THREE.Sprite(new THREE.SpriteMaterial({map:reticleFor('dot'),transparent:true,depthTest:false,depthWrite:false}));
  clipReticle.renderOrder=999; clipReticle.visible=false; overlayScene.add(clipReticle); return clipReticle; }
function showReticle(p,type){ const s=ensureReticle();
  if(!p){ s.visible=false; return; }
  s.material.map=reticleFor(type); s.material.needsUpdate=true;
  s.position.set(p[0],p[1],p[2]); s.scale.setScalar(pxToWorldAt(RETICLE_PX,s.position)); s.visible=true; }
function ensureClipPreview(){ if(clipPreview) return clipPreview;
  clipPreview=new THREE.Box3Helper(new THREE.Box3(new THREE.Vector3(),new THREE.Vector3()),new THREE.Color(CLIP_BOX_COLOR));
  clipPreview.material.depthTest=false; clipPreview.renderOrder=997; clipPreview.visible=false;
  overlayScene.add(clipPreview); return clipPreview; }
function setClipPreview(box){ const h=ensureClipPreview();
  if(!box||box.isEmpty()){ h.visible=false; return; }
  h.box.copy(box); h.visible=true; }
function clearClipDrawHints(){ setClipPreview(null); showReticle(null); }
// The live preview: reticle + a flat floor rectangle while picking corners, then the pulled box.
function clipBoxPreviewAt(cx,cy){
  if(clipMode!=='box') return;
  const fy=clipBoxFloorY();
  if(!clipBoxDraft||!clipBoxDraft.b){
    const fp=clipBoxFloorPoint(cx,cy);
    showReticle(fp&&fp.snap?fp.world:null, fp&&fp.snap);
    if(!clipBoxDraft||!fp){ setClipPreview(null); return; }
    setClipPreview(clipBoxFrom(clipBoxDraft.a,fp.xz,fy,fy));   // flat footprint on the floor
  } else {
    const h=clipBoxHeightAt(cx,cy);
    showReticle(h.snap,'level');
    setClipPreview(clipBoxFrom(clipBoxDraft.a,clipBoxDraft.b,fy,h.y));
    setClipDrawPrompt(h.usable?'h':'axial');
  } }
function setClipDrawPrompt(stage){
  const say=(b,rest)=>readout.replaceChildren(el('b',null,b),document.createTextNode(rest));
  if(stage==='a') say('Click the first floor corner',' of the clip box · Esc to cancel');
  else if(stage==='b') say('Click the opposite floor corner',' · Esc to step back');
  else if(stage==='axial') say('Orbit off a top view',' to set the height · Esc to step back');
  else say('Move up or down to set the height',' then click · Esc to step back'); }
function onClipBoxClick(cx,cy){
  const fy=clipBoxFloorY();
  if(!clipBoxDraft){
    const fp=clipBoxFloorPoint(cx,cy); if(!fp){ clearClipDrawHints(); return; }
    clipBoxDraft={a:fp.xz,b:null}; setClipDrawPrompt('b'); return; }
  if(!clipBoxDraft.b){
    const fp=clipBoxFloorPoint(cx,cy); if(!fp){ clearClipDrawHints(); return; }
    // BOTH plan extents must be real. floless rejects only when both are under 1 mm, so a perfectly
    // straight line passes here and the third click then disarms and silently adds nothing.
    if(Math.abs(fp.xz[0]-clipBoxDraft.a[0])<1||Math.abs(fp.xz[1]-clipBoxDraft.a[1])<1) return;
    clipBoxDraft.b=fp.xz; setClipDrawPrompt('h'); return; }
  const h=clipBoxHeightAt(cx,cy);
  // In an axial view no height can be meant — refuse rather than commit a 1 mm sliver.
  if(!h.usable){ setClipDrawPrompt('axial'); return; }
  const box=clipBoxFrom(clipBoxDraft.a,clipBoxDraft.b,fy,h.y);
  clipBoxDraft=null; clearClipDrawHints(); setClipMode(null);
  if(box.min.x<box.max.x&&box.min.y<box.max.y&&box.min.z<box.max.z) addClipBoxFromBox(box); }

// ---- the clip list ----------------------------------------------------------------------------
// Swatch = enable/disable (filled cutting, hollow off — the objects panel's own convention), label =
// select, ✎ = rename, × = delete. Selection and enabling sit on DIFFERENT visual channels (row-edge
// bar vs swatch fill) so a disabled-and-selected clip reads unambiguously as both.
const clipsPanel=document.getElementById('clips');
let clipAnchor=null;       // Shift-range anchor
let clipEditingId=null;    // the row currently in inline rename
function clipSelectFromRow(id,e){
  const ids=clips.map(c=>c.id), cur=new Set(selectedClipIds);
  let next;
  if(e.shiftKey&&clipAnchor!=null&&ids.includes(clipAnchor)&&ids.includes(id)){
    const i0=ids.indexOf(clipAnchor), i1=ids.indexOf(id);
    next=ids.slice(Math.min(i0,i1),Math.max(i0,i1)+1);
  } else if(e.ctrlKey||e.metaKey){
    next=new Set(cur); next.has(id)?next.delete(id):next.add(id); next=[...next]; clipAnchor=id;
  } else {
    // A plain click on the only selected clip CLEARS it — the one modifier-free way to dismiss the
    // handles without deleting anything.
    next=(cur.size===1&&cur.has(id))?[]:[id]; clipAnchor=id;
  }
  setSelectedClips(next);
}
function startClipRename(id){
  clipEditingId=id; refreshClipList();
  const inp=clipsPanel.querySelector('.cedit'); if(inp){ inp.focus(); inp.select(); }
}
// Invalid input keeps the row in edit mode rather than reverting: the message stays on screen next
// to the field the user is still holding. A FRESH alert node per attempt — re-using one and only
// changing its text does not reliably re-announce when the same message repeats.
function showClipRenameError(row,inp,msg){
  const old=clipsPanel.querySelector('.cerr'); if(old) old.remove();
  const err=el('div','cerr',msg); err.id='clipErr'; err.setAttribute('role','alert');
  inp.setAttribute('aria-invalid','true'); inp.setAttribute('aria-describedby',err.id);
  row.after(err); inp.focus();
}
// Rebuilding the rows destroys focus, so remember WHICH control on WHICH clip had it and put it
// back. Without this, toggling a clip with Space or finishing an F2 rename throws a keyboard user
// to the top of the tab order — the F2 affordance exists for exactly those users.
function clipFocusKey(){ const a=document.activeElement;
  if(!a||!clipsPanel||!clipsPanel.contains(a)) return null;
  const row=a.closest('.crow'); if(!row) return null;
  const cls=['cvis','cpick','cren','cdel','cedit'].find(k=>a.classList.contains(k));
  return cls?{id:row.dataset.clipId,cls}:null; }
function restoreClipFocus(key){ if(!key||!clipsPanel) return;
  let el2=clipsPanel.querySelector(`.crow[data-clip-id="${key.id}"] .${key.cls}`);
  // The clip is gone (deleted) — keep focus in the panel rather than dropping it on the document.
  if(!el2) el2=clipsPanel.querySelector('.crow .cpick');
  if(el2) el2.focus(); }
function refreshClipList(){
  if(!clipsPanel) return;
  const focusKey=clipFocusKey();
  clipsPanel.classList.toggle('show',clips.length>0);
  if(!clips.length){ clipsPanel.replaceChildren(); clipEditingId=null; return; }
  const head=el('div','csec','Clips');
  const body=el('div','cbody');
  for(const c of clips){
    const row=el('div','crow'+(selectedClipIds.has(c.id)?' sel':''));
    row.dataset.clipId=c.id;
    const kind=c.kind==='plane'?'Plane clip':'Box clip';
    const box=el('button','cvis'); box.type='button';
    box.setAttribute('role','checkbox'); box.setAttribute('aria-checked',c.enabled?'true':'false');
    box.setAttribute('aria-label','Enable or disable — '+c.label);
    box.setAttribute('data-tip','Turn cutting on/off (keeps the clip)');
    const sw=el('span','cswatch'); sw.style.setProperty('--sw',c.kind==='plane'?'var(--clip-plane)':'var(--clip-box)');
    box.append(sw);
    box.addEventListener('click',ev=>{ ev.stopPropagation(); toggleClip(c.id); });
    if(clipEditingId===c.id){
      const inp=document.createElement('input'); inp.className='cedit'; inp.type='text'; inp.value=c.label;
      inp.setAttribute('aria-label','Rename '+c.label);
      const commit=()=>{ const nm=inp.value.trim();
        if(nm===c.label){ clipEditingId=null; refreshClipList(); return; }
        if(!nm){ showClipRenameError(row,inp,'A clip name cannot be blank.'); return; }
        if(!renameClip(c.id,nm)){ showClipRenameError(row,inp,'Another clip is already named “'+nm+'”.'); return; }
        clipEditingId=null; refreshClipList(); };
      const cancel=()=>{ clipEditingId=null; refreshClipList(); };
      inp.addEventListener('keydown',ev=>{ ev.stopPropagation();
        if(ev.key==='Enter'){ ev.preventDefault(); commit(); }
        else if(ev.key==='Escape'){ ev.preventDefault(); cancel(); } });
      inp.addEventListener('blur',()=>{ if(clipEditingId===c.id) cancel(); });   // focus left → treat as cancel, never keep an invalid draft alive
      row.append(box,inp);
    } else {
      const pick=el('button','cpick',c.label); pick.type='button';
      // Kind is carried in text as well as colour — after a rename the default "Plane 1" label is
      // gone and colour would be the only channel left.
      pick.setAttribute('data-tip',kind+' — select · Shift/Ctrl multi-select · click again to clear · F2 renames');
      pick.setAttribute('aria-pressed',selectedClipIds.has(c.id)?'true':'false');
      pick.addEventListener('click',ev=>{ ev.stopPropagation(); clipSelectFromRow(c.id,ev); });
      pick.addEventListener('keydown',ev=>{ if(ev.key==='F2'){ ev.preventDefault(); ev.stopPropagation(); startClipRename(c.id); } });
      const ren=el('button','cren','✎'); ren.type='button';
      ren.setAttribute('aria-label','Rename — '+c.label); ren.setAttribute('data-tip','Rename (F2)');
      ren.addEventListener('click',ev=>{ ev.stopPropagation(); startClipRename(c.id); });
      const del=el('button','cdel','×'); del.type='button';
      del.setAttribute('aria-label','Delete — '+c.label); del.setAttribute('data-tip','Delete this clip');
      del.addEventListener('click',ev=>{ ev.stopPropagation(); removeClip(c.id); });
      row.append(box,pick,ren,del);
    }
    body.append(row);
  }
  clipsPanel.replaceChildren(head,body,el('div','chint','Box shows / hides · F2 renames · Del removes the selected'));
  restoreClipFocus(focusKey);
}
// Arm/disarm the face-pick: 'plane' → next left-click on a face drops a plane; null → back to selecting.
function setClipPrompt(){ if(clipMode==='box'){ setClipDrawPrompt(!clipBoxDraft?'a':(clipBoxDraft.b?'h':'b')); return; }
  readout.replaceChildren(el('b',null,'Click a face'), document.createTextNode(' to cut the view there · Esc to cancel')); }
function setClipMode(m){ clipMode=(m==='plane'||m==='box')?m:null;
  // Drop the per-draw state on every arm/disarm: a stale reticle or draft must never survive into
  // the next command.
  clipBoxDraft=null; clipSnapCands=null; clipLevels=null; clipProj=null;
  clearClipDrawHints();
  if(!clipMode) setClipPlanePreview(null,null);   // disarming must take the ghost with it
  renderer.domElement.style.cursor=clipMode?'crosshair':'default';
  // Armed → the button both lights up AND becomes its own cancel target, so the way out is where the
  // way in was. Matches the floless editor's ✕ affordance.
  const btn=document.getElementById('clip'); if(btn){ btn.classList.toggle('on',!!clipMode); btn.textContent=clipMode?'Clip ✕':'Clip ▾'; }
  if(clipMode) setClipPrompt();
  else writeSelectionReadout();
  return clipMode; }
// Work area: one box that bounds (and sections) the view, shown as an always-visible wireframe.
function renderWorkArea(){ if(workAreaHelper){ overlayScene.remove(workAreaHelper); workAreaHelper.geometry.dispose(); workAreaHelper.material.dispose(); workAreaHelper=null; }
  if(!workArea || !workArea.enabled || workArea.box.isEmpty()) return;   // switched off → no wireframe either
  workAreaHelper=new THREE.Box3Helper(workArea.box, new THREE.Color(0x60a5fa));
  workAreaHelper.material.depthTest=false; workAreaHelper.renderOrder=995; overlayScene.add(workAreaHelper); }
// A work area has two independent switches, matching the floless editor:
//   enabled — is it in force at all (the "Show work area" tick)
//   whole   — ON (default): a part TOUCHING the box is drawn in full and parts outside are hidden
//             outright, so nothing is ever sliced by surprise;
//             OFF: the box sections the view, cutting parts at its faces.
// Only the cut mode contributes clipping planes; whole mode is pure visibility (see applyClips and
// applyGroupVisibility). A re-define keeps whichever mode is current.
function setWorkAreaBox(box){ if(!box||box.isEmpty()) return false;
  const whole = workArea ? workArea.whole : true;
  workArea={ box:box.clone(), planes:boxToPlanes(box), enabled:true, whole };
  applyClips(); renderWorkArea(); applyGroupVisibility(); reflectWorkArea(); return true; }
function workAreaToggle(on){ if(!workArea) return false;
  workArea.enabled = on===undefined ? !workArea.enabled : !!on;
  applyClips(); renderWorkArea(); applyGroupVisibility(); reflectWorkArea(); return workArea.enabled; }
function workAreaSetWhole(on){ if(!workArea) return false;
  workArea.whole = on===undefined ? !workArea.whole : !!on;
  applyClips(); applyGroupVisibility(); reflectWorkArea(); return workArea.whole; }
function workAreaState(){ return workArea ? { on:!!workArea.enabled, whole:!!workArea.whole } : null; }
// Keep the button and its two ticks honest about the live state. "Show whole parts" is meaningless
// without a work area, so it stays hidden until there is one.
function reflectWorkArea(){ const st=workAreaState();
  const btn=document.getElementById('work'); if(btn) btn.classList.toggle('on', !!(st&&st.on));
  const on=document.getElementById('waOn'); if(on) on.setAttribute('aria-checked', st&&st.on?'true':'false');
  const wh=document.getElementById('waWhole'); if(wh){ wh.style.display=st?'flex':'none'; wh.setAttribute('aria-checked', st&&st.whole?'true':'false'); } }
function workAreaSetAll(){ const box=meshBox(pickable); if(box.isEmpty()) return false;
  // Pad before these bounds become clip planes. Bound EXACTLY to the mesh extents, the six planes sit
  // on the model's own outer surfaces and the whole model is clipped away — "set to all objects" made
  // everything vanish (pre-existing: it did this on every release before the whole/cut switch, where
  // cut was the only mode).
  //
  // The pad is the SAME one selBox uses for clip boxes and for "define from selection", which is the
  // empirically proven-good value here: A/B'd in a browser, a 0.6mm pad on this 6 m model still
  // vanishes while this one renders correctly. That threshold is far larger than single-precision
  // error at these magnitudes would predict, so the true mechanism is NOT understood — matching the
  // value that demonstrably works, rather than a derived epsilon that does not.
  box.expandByScalar(Math.max(maxDim*0.04, 1));
  return setWorkAreaBox(box); } // bound the whole model by its rendered mesh bounds (not centrelines)
function workAreaFromSelection(pad){ const box=new THREE.Box3();
  for(const m of selection){ if(m.visible) box.expandByObject(m); }
  if(box.isEmpty()) return false; box.expandByScalar(pad==null?Math.max(maxDim*0.04,1):pad); return setWorkAreaBox(box); }
function clearWorkArea(){ workArea=null; applyClips(); renderWorkArea(); applyGroupVisibility(); reflectWorkArea(); }
function workAreaOn(){ return !!(workArea && workArea.enabled); }   // switched off IS off — clipping, filtering and the helper are all disabled

addEventListener('resize',()=>{
  perspCam.aspect=innerWidth/innerHeight; perspCam.updateProjectionMatrix();
  if(camera.isOrthographicCamera) reframeOrtho();
  renderer.setSize(innerWidth,innerHeight);
  invalidateClipProjectionCache();   // new projection matrix
});
// Single-key view shortcuts mirror the ViewCube faces (lower- or upper-case).
const VIEW_KEYS={ t:'top', f:'front', r:'right', b:'back', l:'left' };
// A single-key shortcut must never fire while the user is typing. Without this guard, typing a
// word containing t/f/r/b/l into ANY text field swings the camera and swallows the character,
// and Home fits the model instead of moving the caret — so no text input in this document can
// work until the guard exists.
function typingInto(t){ if(!t) return false;
  if(t.isContentEditable) return true;
  const tag=(t.tagName||'').toLowerCase();
  return tag==='input'||tag==='textarea'||tag==='select'; }
addEventListener('keydown',e=>{
  // Escape is deliberately still honoured while typing ONLY for the clip-mode cancel below, and
  // a text field that wants Escape for itself stops propagation before this handler sees it.
  if(typingInto(e.target) && e.key!=='Escape') return;
  // A live handle drag mutates geometry as you move, so Escape mid-drag REVERTS — and it wins over
  // every other Escape branch, including the armed-mode cancel below.
  if(e.key==='Escape' && gesture==='clip-handle'){ endGesture(true); e.preventDefault(); return; }
  // Escape STEPS BACK through the draw — height → footprint → armed → disarmed — so a mis-clicked
  // second corner costs one key, not the whole command.
  if(e.key==='Escape' && clipMode==='box' && clipBoxDraft){ e.preventDefault();
    if(clipBoxDraft.b) clipBoxDraft.b=null; else clipBoxDraft=null;
    clearClipDrawHints(); setClipPrompt(); return; }
  if(e.key==='Escape' && clipMode){ setClipMode(null); e.preventDefault(); return; } // cancel an armed clip pick
  if(typingInto(e.target)) return;                                                   // Escape with no armed clip → leave it to the field
  // Del removes the selected clip(s) — only when clips are what is selected, so it can never be
  // mistaken for "delete the selected elements" (which this viewer does not do).
  if((e.key==='Delete'||e.key==='Backspace') && selectedClipIds.size){ deleteSelectedClips(); e.preventDefault(); return; }
  if(e.key==='Home'){ frameBox(sceneBox); e.preventDefault(); }                       // fit all
  // Section shortcuts, matching the floless editor. Shift-qualified so they cannot collide with the
  // bare-letter view keys above, and safe to add only because of the typing guard at the top.
  else if(e.shiftKey && !e.altKey && !e.ctrlKey && !e.metaKey && (e.key==='X'||e.key==='x')){
    setClipMode(clipMode?null:'plane'); e.preventDefault(); }                          // Shift+X → arm / cancel a clip plane
  else if(e.shiftKey && !e.altKey && !e.ctrlKey && !e.metaKey && (e.key==='B'||e.key==='b')){
    addClipBox(); e.preventDefault(); }                                                // Shift+B → clip box around selection/model
  else if(e.shiftKey && !e.altKey && !e.ctrlKey && !e.metaKey && (e.key==='D'||e.key==='d')){
    setClipMode(clipMode==='box'?null:'box'); e.preventDefault(); }                     // Shift+D → draw a clip box by picking points
  else if((e.key==='z'||e.key==='Z') && e.altKey){                                     // zoom the current selection
    if(selection.length){ const b=new THREE.Box3(); for(const m of selection) b.expandByObject(m); frameBox(b); } e.preventDefault(); }
  else if(!e.altKey && !e.ctrlKey && !e.metaKey && VIEW_KEYS[e.key.toLowerCase()]){     // T/F/R/B/L → named views
    applyView(VIEW_KEYS[e.key.toLowerCase()]); e.preventDefault(); }
});

// Toolbar wiring (named views now live on the ViewCube — see below — not duplicate buttons).
document.getElementById('projBtn').addEventListener('click', e=>{ e.stopPropagation(); toggleMenu('projMenu'); });
document.querySelectorAll('#proj [data-proj]').forEach(b=>b.addEventListener('click', e=>{ e.stopPropagation(); closeMenus(); setProjection(b.dataset.proj); }));
document.getElementById('modeBtn').addEventListener('click', e=>{ e.stopPropagation(); toggleMenu('modeMenu'); });
document.querySelectorAll('#modes [data-mode]').forEach(b=>b.addEventListener('click', e=>{ e.stopPropagation(); closeMenus(); setDisplayMode(b.dataset.mode); }));

document.getElementById('fit').addEventListener('click',()=>frameBox(sceneBox));

// ---- Section dropdowns (Clip / Work area) ----
function closeMenus(){ document.querySelectorAll('#toolbar .tb-menu.open').forEach(m=>m.classList.remove('open')); }
function toggleMenu(id){ const m=document.getElementById(id), open=m.classList.contains('open'); closeMenus(); if(!open) m.classList.add('open'); }
document.getElementById('clip').addEventListener('click', e=>{ e.stopPropagation(); if(clipMode){ setClipMode(null); return; } toggleMenu('clipMenu'); });
document.getElementById('work').addEventListener('click', e=>{ e.stopPropagation(); toggleMenu('workMenu'); });
document.querySelectorAll('#clipMenu [data-clip]').forEach(b=>b.addEventListener('click', e=>{ e.stopPropagation(); closeMenus();
  const a=b.dataset.clip; if(a==='plane') setClipMode('plane'); else if(a==='box') addClipBox();
  else if(a==='draw') setClipMode('box'); else if(a==='clear') clearClips(); }));
document.querySelectorAll('#workMenu [data-wa]').forEach(b=>b.addEventListener('click', e=>{ e.stopPropagation(); closeMenus();
  const a=b.dataset.wa; if(a==='all') workAreaSetAll(); else if(a==='sel') workAreaFromSelection(); else if(a==='clear') clearWorkArea(); }));
// The two ticks deliberately do NOT close the menu — the whole point is to see the state flip.
document.getElementById('waOn').addEventListener('click', e=>{ e.stopPropagation(); workAreaToggle(); });
document.getElementById('waWhole').addEventListener('click', e=>{ e.stopPropagation(); workAreaSetWhole(); });
document.addEventListener('pointerdown', e=>{ if(!e.target.closest('#toolbar')) closeMenus(); }, true); // click outside the toolbar closes a menu

// ---- Themed tooltips (replaces native title=): one shared element, shown on data-tip hover ----
const tooltip=document.createElement('div'); tooltip.id='tooltip'; document.body.appendChild(tooltip); let tipT=null;
function showTip(t){ const txt=t.getAttribute('data-tip'); if(!txt) return; tooltip.textContent=txt; tooltip.classList.add('show');
  const r=t.getBoundingClientRect(), tw=tooltip.offsetWidth, th=tooltip.offsetHeight;
  let x=Math.max(6,Math.min(r.left+r.width/2-tw/2, innerWidth-tw-6)), y=r.bottom+6; if(y+th>innerHeight-6) y=r.top-th-6;
  tooltip.style.left=x+'px'; tooltip.style.top=y+'px'; }
function hideTip(){ clearTimeout(tipT); tooltip.classList.remove('show'); }
document.addEventListener('pointerover', e=>{ const t=e.target.closest('[data-tip]'); if(!t) return; clearTimeout(tipT); tipT=setTimeout(()=>showTip(t),400); });
document.addEventListener('pointerout', e=>{ if(e.target.closest('[data-tip]')) hideTip(); });
document.addEventListener('pointerdown', hideTip, true); // a click hides the tip immediately

// ---- ViewCube (#258): a small labelled cube, top-right, mirroring the camera orientation.
// Clicking a face snaps to that ortho view; an edge/corner snaps to an iso view — it fully
// replaces the old named-view buttons (the "cubix with planes" from the QA note). ----
const CUBE_PX=104;
const cubeRenderer=new THREE.WebGLRenderer({antialias:true, alpha:true});
cubeRenderer.setPixelRatio(Math.min(devicePixelRatio,2)); cubeRenderer.setSize(CUBE_PX,CUBE_PX);
document.getElementById('viewcube').appendChild(cubeRenderer.domElement);
const cubeScene=new THREE.Scene();
const cubeCam=new THREE.PerspectiveCamera(40,1,0.1,20); cubeCam.position.set(0,0,5);
function faceTexture(label){ const c=document.createElement('canvas'); c.width=c.height=128; const g=c.getContext('2d');
  g.fillStyle='#eef2f7'; g.fillRect(0,0,128,128); g.strokeStyle='#94a3b8'; g.lineWidth=6; g.strokeRect(4,4,120,120);
  g.fillStyle='#334155'; g.font='bold 20px ui-sans-serif,system-ui,sans-serif'; g.textAlign='center'; g.textBaseline='middle';
  g.fillText(label,64,64); return new THREE.CanvasTexture(c); }
// BoxGeometry material order is +X,-X,+Y,-Y,+Z,-Z. World up=Y, front=+Z, right=+X.
const CUBE_FACES=[ {label:'RIGHT',view:'right'}, {label:'LEFT',view:'left'}, {label:'TOP',view:'top'},
                   {label:'BOTTOM',view:'bottom'}, {label:'FRONT',view:'front'}, {label:'BACK',view:'back'} ];
const cubeMesh=new THREE.Mesh(new THREE.BoxGeometry(1.9,1.9,1.9), CUBE_FACES.map(f=>new THREE.MeshBasicMaterial({map:faceTexture(f.label)})));
cubeMesh.add(new THREE.LineSegments(new THREE.EdgesGeometry(cubeMesh.geometry), new THREE.LineBasicMaterial({color:0x64748b})));
cubeScene.add(cubeMesh);
const cubeRay=new THREE.Raycaster(), cubeMouse=new THREE.Vector2();
cubeRenderer.domElement.addEventListener('pointerdown', e=>{ e.preventDefault();
  const r=cubeRenderer.domElement.getBoundingClientRect();
  cubeMouse.x=((e.clientX-r.left)/r.width)*2-1; cubeMouse.y=-((e.clientY-r.top)/r.height)*2+1;
  cubeRay.setFromCamera(cubeMouse,cubeCam);
  const hit=cubeRay.intersectObject(cubeMesh,false)[0]; if(!hit||!hit.face) return;
  // Snap the local hit point to the nearest face / edge / corner. A centre hit gives that ortho
  // view (the gimbal-safe named direction); an edge/corner hit gives an iso-style view from that
  // world direction — so the cube covers everything the removed named-view buttons did, incl. Iso.
  const p=cubeMesh.worldToLocal(hit.point.clone()), snap=x=>Math.abs(x)>0.6?Math.sign(x):0;
  const d=new THREE.Vector3(snap(p.x),snap(p.y),snap(p.z));
  if(Math.abs(d.x)+Math.abs(d.y)+Math.abs(d.z)<=1){ const f=CUBE_FACES[hit.face.materialIndex]; if(f) applyView(f.view); }
  else frameBox(sceneBox, d); });
// Mirror the scene from the main camera's direction: orient the cube by the inverse of the
// camera's world rotation (front view → the FRONT face turns toward the viewer, and so on).
function syncCube(){ cubeMesh.quaternion.copy(camera.quaternion).invert(); }

// ---- World-axis triad: a passive bottom-right gizmo (Tekla-style) showing where the SCENE's
// X/Y/Z point. The cube OWNS orientation changes; this only reads out (pointer-events:none).
// Axes go through the same up-conversion as the geometry (conv), so the labels mean the
// PRODUCER's axes — a Z-up steel scene shows Z where its Z really points. X red, Y green,
// Z blue is the CAD convention.
const TRIAD_PX=92;
const triadRenderer=new THREE.WebGLRenderer({antialias:true, alpha:true});
triadRenderer.setPixelRatio(Math.min(devicePixelRatio,2)); triadRenderer.setSize(TRIAD_PX,TRIAD_PX);
document.getElementById('axestriad').appendChild(triadRenderer.domElement);
const triadScene=new THREE.Scene();
const triadCam=new THREE.OrthographicCamera(-2.1,2.1,2.1,-2.1,0.1,20); triadCam.position.set(0,0,5);
const triadGroup=new THREE.Group();
function triadTip(label,color,pos){ // free-standing colored letter with a dark halo — legible over any geometry, no disc
  const c=document.createElement('canvas'); c.width=c.height=64; const g=c.getContext('2d');
  g.font='bold 46px ui-sans-serif,system-ui,sans-serif'; g.textAlign='center'; g.textBaseline='middle';
  g.lineWidth=8; g.lineJoin='round'; g.strokeStyle='rgba(2,8,23,.9)'; g.strokeText(label,32,34);
  g.fillStyle=color; g.fillText(label,32,34);
  const s=new THREE.Sprite(new THREE.SpriteMaterial({map:new THREE.CanvasTexture(c)})); s.position.copy(pos); s.scale.setScalar(0.85); return s; }
{ const up=(SCENE.meta&&SCENE.meta.up)||'z', AXIS_Y=new THREE.Vector3(0,1,0);
  for(const [label,color,axis] of [['X','#ef4444',[1,0,0]],['Y','#22c55e',[0,1,0]],['Z','#3b82f6',[0,0,1]]]){
    const d=conv(axis,up).normalize(), mat=new THREE.MeshBasicMaterial({color});
    const shaft=new THREE.Mesh(new THREE.CylinderGeometry(0.06,0.06,1.05,8), mat);
    shaft.quaternion.setFromUnitVectors(AXIS_Y,d); shaft.position.copy(d).multiplyScalar(0.525);
    const head=new THREE.Mesh(new THREE.ConeGeometry(0.16,0.34,12), mat); // arrowhead at the shaft end
    head.quaternion.copy(shaft.quaternion); head.position.copy(d).multiplyScalar(1.22);
    triadGroup.add(shaft, head, triadTip(label,color,d.clone().multiplyScalar(1.62))); }
  triadGroup.add(new THREE.Mesh(new THREE.SphereGeometry(0.1,12,8), new THREE.MeshBasicMaterial({color:0xe2e8f0}))); // origin dot
  triadScene.add(triadGroup); }
function syncTriad(){ triadGroup.quaternion.copy(camera.quaternion).invert(); }

(function loop(){ requestAnimationFrame(loop); controls.update(); sizeClipHandles(); renderer.render(scene,camera);
  // 2nd UNCLIPPED pass: the work-area wireframe must stay visible through any clip (autoClear off so
  // it draws on top of the clipped main pass; clipping planes cleared so it is never sectioned).
  if(overlayScene.children.length){ const saved=renderer.clippingPlanes; renderer.autoClear=false; renderer.clippingPlanes=EMPTY_CLIPS; renderer.render(overlayScene,camera); renderer.clippingPlanes=saved; renderer.autoClear=true; }
  syncCube(); cubeRenderer.render(cubeScene,cubeCam);
  syncTriad(); triadRenderer.render(triadScene,triadCam); })();

renderScene(SCENE);
window.__viewerReady=true; if(window.__viewerPost) window.__viewerPost('viewer-ready');
window.__viewer3d={ count:()=>pickable.length, name:()=>(SCENE.meta&&SCENE.meta.name)||'',
  projection:()=>camera.isOrthographicCamera?'ortho':'persp', mode:()=>displayMode,
  hidden:()=>[...groupHidden], solo:()=>soloGroup, visibleCount:()=>pickable.filter(m=>m.visible).length,
  selectionCount:()=>selection.length, cubeFaces:()=>CUBE_FACES.map(f=>f.view),
  // Realistic-mode state. `envOn` is the load-bearing one: metal with no environment renders
  // black, so "the mode switched" is not evidence on its own — the environment must be live.
  envOn:()=>scene.environment!==null&&renderer.toneMapping===THREE.ACESFilmicToneMapping,
  shadowsOn:()=>shadowsEnabled,
  materialOf:(id)=>{ const m=pickable.find(p=>p.userData&&p.userData.id===id); if(!m) return null;
    return {family:m.material.userData.family, metalness:m.material.metalness,
      roughness:m.material.roughness, color:'#'+m.material.color.getHexString()}; },
  camDir:()=>camera.position.clone().sub(controls.target).normalize().toArray(),
  selectInRect:(x0,y0,x1,y1)=>{ setSelection(meshesInRect(x0,y0,x1,y1)); return selection.length; },
  setView:applyView, setProjection, setDisplayMode, toggleGroup, frameAll:()=>frameBox(sceneBox),
  clipCount, addClipBox, clearClips, setClipMode, addClipPlaneAtScreen,
  workAreaSetAll, workAreaFromSelection, clearWorkArea, workAreaOn,
  workAreaToggle, workAreaSetWhole, workAreaState,
  clipMode:()=>clipMode,
  clipPlanes:()=>(renderer.clippingPlanes||[]).length,
  // Clip editing. `getClips` reports EXACT bounds and plane geometry, not rounded sizes — a probe
  // that rounds cannot prove a snapped bound or which face a drag moved.
  getClips, selectedClips, setSelectedClips, toggleClip, renameClip, removeClip, deleteSelectedClips,
  clipHandlesScreen, addClipBoxFromBox, setClipMode,
  worldToScreen:(p)=>toScreenPt(p),
  gestureState:()=>({ gesture, draft:clipBoxDraft?(clipBoxDraft.b?'height':'footprint'):null, dragging:!!(clipDrag&&clipDrag.moved) }),
  reticleState:()=>({ visible:!!(clipReticle&&clipReticle.visible), at:clipReticle&&clipReticle.visible?clipReticle.position.toArray():null }),
  clipDrawFloorY:()=>clipBoxFloorY(),
  clipBoxFloorAt:(x,y)=>clipBoxFloorPoint(x,y),
  clipHeightAt:(x,y)=>clipBoxDraft&&clipBoxDraft.b?clipBoxHeightAt(x,y):null,
  // The basis guard's silent failure mode is a VERTICAL cut, so a test needs the corners themselves.
  planePatchCorners:(hp,n)=>planePatchCorners(new THREE.Vector3(hp[0],hp[1],hp[2]),new THREE.Vector3(n[0],n[1],n[2]).normalize(),CLIP_PATCH_R).map(v=>v.toArray()),
  clipGhostShown:()=>!!(clipGhost&&clipGhost.group.visible),
  controlsEnabled:()=>controls.enabled,
  memberFrame:(id)=>{const m=pickable.find(o=>o.userData&&o.userData.id===id);return m&&m.userData.rollFrame?JSON.parse(JSON.stringify(m.userData.rollFrame)):null;},
  memberVertices:(id)=>{const m=pickable.find(o=>o.userData&&o.userData.id===id);if(!m||!m.geometry||!m.geometry.attributes.position)return[];m.updateMatrixWorld(true);const p=m.geometry.attributes.position,out=[];for(let i=0;i<p.count;i++){const v=new THREE.Vector3().fromBufferAttribute(p,i).applyMatrix4(m.matrixWorld),q=v.toArray();if(!out.some(a=>Math.hypot(a[0]-q[0],a[1]-q[1],a[2]-q[2])<1e-6))out.push(q);}return out;},
  // What was actually DRAWN, so a test is not limited to interrogating the transform that fed it.
  gridRenderables:()=>gridLines.map(l=>({role:l.userData.gridRole,a:l.userData.a,b:l.userData.b})),
  referenceSystemSegments:()=>(SCENE.referenceSystems||[]).filter(R=>R&&R.kind==='structural-grid').map(R=>{
    const s=referenceSystemSegments(R);
    return { axes:s.axes.map(a=>({label:a.label,direction:a.direction,a:a.a.toArray(),b:a.b.toArray()})),
             levels:s.levels.map(l=>({label:l.label,y:l.y,segments:l.segments.map(g=>[g[0].toArray(),g[1].toArray()])})) }; }),
  pointClipped:(p)=>isPointClipped(new THREE.Vector3(p[0],p[1],p[2])) };
</script>
</body>
</html>
"##;

/// `viewer-3d.render` — render a generic 3D scene into a self-contained interactive HTML page.
/// Mirrors `ui.render`'s contract: `{ html, bytes, output-path? }`, write gated to a real run.
#[derive(Default)]
struct SceneReceipt {
    emitted: Vec<Value>,
    unsupported: Vec<Value>,
    warnings: Vec<Value>,
}

fn scene_error(path: &str, message: &str) -> AwareError {
    AwareError::Validation(format!("viewer-3d render: `{path}` {message}"))
}

fn object_array<'a>(scene: &'a Value, key: &str) -> Result<&'a [Value], AwareError> {
    match scene.get(key) {
        None => Ok(&[]),
        Some(Value::Array(values)) => Ok(values),
        Some(other) => Err(scene_error(
            key,
            &format!("must be an array (got {})", json_type(other)),
        )),
    }
}

fn object_at<'a>(
    value: &'a Value,
    path: &str,
) -> Result<&'a serde_json::Map<String, Value>, AwareError> {
    value
        .as_object()
        .ok_or_else(|| scene_error(path, "must be an object"))
}

fn record_id(
    object: &serde_json::Map<String, Value>,
    path: &str,
    ids: &mut HashSet<String>,
) -> Result<String, AwareError> {
    let raw = object
        .get("id")
        .and_then(Value::as_str)
        .ok_or_else(|| scene_error(&format!("{path}.id"), "must be a non-empty string"))?;
    let id = raw.trim();
    if id.is_empty()
        || id != raw
        || id.len() > 256
        || id
            .chars()
            .any(|character| character <= '\u{1f}' || character == '\u{7f}')
    {
        return Err(scene_error(
            &format!("{path}.id"),
            "must be trimmed, 1-256 UTF-8 bytes, and contain no control characters",
        ));
    }
    if !ids.insert(id.to_owned()) {
        return Err(scene_error(
            &format!("{path}.id"),
            "must be globally unique",
        ));
    }
    Ok(id.to_owned())
}

fn finite_number(value: Option<&Value>, path: &str) -> Result<f64, AwareError> {
    value
        .and_then(Value::as_f64)
        .filter(|number| number.is_finite())
        .ok_or_else(|| scene_error(path, "must be a finite number"))
}

fn positive_number(value: Option<&Value>, path: &str) -> Result<f64, AwareError> {
    let number = finite_number(value, path)?;
    if number <= 0.0 {
        return Err(scene_error(path, "must be greater than zero"));
    }
    Ok(number)
}

fn vector<const N: usize>(value: Option<&Value>, path: &str) -> Result<[f64; N], AwareError> {
    let values = value
        .and_then(Value::as_array)
        .filter(|values| values.len() == N)
        .ok_or_else(|| scene_error(path, &format!("must be an array of {N} numbers")))?;
    let mut result = [0.0; N];
    for (index, value) in values.iter().enumerate() {
        result[index] = finite_number(Some(value), &format!("{path}[{index}]"))?;
    }
    Ok(result)
}

/// Read a JSON array of finite numbers, reporting the offending index rather
/// than panicking on a non-numeric entry.
fn number_array(value: Option<&Value>, path: &str) -> Result<Vec<f64>, AwareError> {
    let values = value
        .and_then(Value::as_array)
        .ok_or_else(|| scene_error(path, "must be an array of numbers"))?;
    values
        .iter()
        .enumerate()
        .map(|(index, value)| finite_number(Some(value), &format!("{path}[{index}]")))
        .collect()
}

fn validate_member_xsection(
    object: &serde_json::Map<String, Value>,
    path: &str,
) -> Result<(), AwareError> {
    let Some(value) = object.get("xsection") else {
        return Ok(());
    };
    let section = value
        .as_object()
        .ok_or_else(|| scene_error(&format!("{path}.xsection"), "must be an object"))?;
    let section_path = format!("{path}.xsection");
    let shape = section
        .get("shape")
        .and_then(Value::as_str)
        .ok_or_else(|| {
            scene_error(
                &format!("{section_path}.shape"),
                "must be a canonical shape",
            )
        })?;
    let dimension =
        |name: &str| positive_number(section.get(name), &format!("{section_path}.{name}"));
    let invalid = |message: &str| Err(scene_error(&section_path, message));
    let envelope = |expected_w: f64, expected_d: f64| {
        let source = object
            .get("section")
            .and_then(Value::as_object)
            .ok_or_else(|| scene_error(&format!("{path}.section"), "must be an object"))?;
        let w = positive_number(source.get("w"), &format!("{path}.section.w"))?;
        let d = positive_number(source.get("d"), &format!("{path}.section.d"))?;
        let tolerance = 1.0e-6_f64.max(1.0e-9 * expected_w.abs().max(expected_d.abs()));
        if (w - expected_w).abs() > tolerance || (d - expected_d).abs() > tolerance {
            return invalid("dimensions must match the member section envelope");
        }
        Ok(())
    };
    match shape {
        "i" | "channel" => {
            let d = dimension("d")?;
            let bf = dimension("bf")?;
            let tw = dimension("tw")?;
            let tf = dimension("tf")?;
            if tw >= bf || 2.0 * tf >= d {
                return invalid("must have tw < bf and 2*tf < d");
            }
        }
        "angle" => {
            let d = dimension("d")?;
            let b = dimension("b")?;
            let t = dimension("t")?;
            if t >= d.min(b) {
                return invalid("must have t < min(d,b)");
            }
        }
        "rhs" => {
            let d = dimension("d")?;
            let b = dimension("b")?;
            let t = dimension("t")?;
            if 2.0 * t >= d.min(b) {
                return invalid("must have 2*t < min(d,b)");
            }
        }
        "chs" => {
            let od = dimension("od")?;
            let t = dimension("t")?;
            if 2.0 * t >= od {
                return invalid("must have 2*t < od");
            }
        }
        "rect" => {
            dimension("w")?;
            dimension("d")?;
        }
        "tee" => {
            let d = dimension("d")?;
            let bf = dimension("bf")?;
            let tw = dimension("tw")?;
            let tf = dimension("tf")?;
            if tw >= bf || tf >= d {
                return invalid("must have tw < bf and tf < d");
            }
            envelope(bf, d)?;
        }
        "double-angle" => {
            let d = dimension("d")?;
            let b = dimension("b")?;
            let t = dimension("t")?;
            let gap = finite_number(section.get("gap"), &format!("{section_path}.gap"))?;
            if gap < 0.0 {
                return invalid("must have gap >= 0");
            }
            if t >= d.min(b) {
                return invalid("must have t < min(d,b)");
            }
            match section.get("orientation").and_then(Value::as_str) {
                Some("llbb") => envelope(2.0 * b + gap, d)?,
                Some("slbb") => envelope(2.0 * d + gap, b)?,
                _ => return invalid("orientation must be exactly llbb or slbb"),
            }
        }
        _ => {
            return invalid(
                "shape must be one of i, channel, angle, rhs, chs, rect, tee, or double-angle",
            );
        }
    }
    Ok(())
}

fn scene_element<'a>(scene: &'a Value, id: &str) -> Option<&'a serde_json::Map<String, Value>> {
    scene
        .get("elements")
        .and_then(Value::as_array)?
        .iter()
        .find(|element| element.get("id").and_then(Value::as_str) == Some(id))
        .and_then(Value::as_object)
}

fn axis(
    object: &serde_json::Map<String, Value>,
    path: &str,
) -> Result<([f64; 3], [f64; 3]), AwareError> {
    let (from, to) = if let Some(axis) = object.get("axis") {
        if let Some(axis) = axis.as_object() {
            (
                vector(axis.get("from"), &format!("{path}.axis.from"))?,
                vector(axis.get("to"), &format!("{path}.axis.to"))?,
            )
        } else if let Some(axis) = axis.as_array().filter(|axis| axis.len() == 2) {
            (
                vector(axis.first(), &format!("{path}.axis[0]"))?,
                vector(axis.get(1), &format!("{path}.axis[1]"))?,
            )
        } else {
            return Err(scene_error(&format!("{path}.axis"), "must be `{from,to}`"));
        }
    } else {
        (
            vector(object.get("from"), &format!("{path}.from"))?,
            vector(object.get("to"), &format!("{path}.to"))?,
        )
    };
    let length_sq = (0..3).map(|i| (to[i] - from[i]).powi(2)).sum::<f64>();
    if length_sq <= 1.0e-18 {
        return Err(scene_error(
            &format!("{path}.axis"),
            "must have nonzero length",
        ));
    }
    Ok((from, to))
}

fn direction(object: &serde_json::Map<String, Value>, path: &str) -> Result<[f64; 3], AwareError> {
    let axis = vector::<3>(object.get("axis"), &format!("{path}.axis"))?;
    if axis.iter().map(|value| value * value).sum::<f64>() <= 1.0e-18 {
        return Err(scene_error(
            &format!("{path}.axis"),
            "must have nonzero length",
        ));
    }
    Ok(axis)
}

fn validate_plate(
    object: &serde_json::Map<String, Value>,
    path: &str,
    parent_id: &str,
    ids: &mut HashSet<String>,
) -> Result<Vec<Value>, AwareError> {
    let frame = object_at(
        object
            .get("frame")
            .ok_or_else(|| scene_error(&format!("{path}.frame"), "is required"))?,
        &format!("{path}.frame"),
    )?;
    vector::<3>(frame.get("origin"), &format!("{path}.frame.origin"))?;
    let u = vector::<3>(frame.get("uDir"), &format!("{path}.frame.uDir"))?;
    let v = vector::<3>(frame.get("vDir"), &format!("{path}.frame.vDir"))?;
    let n = vector::<3>(frame.get("normal"), &format!("{path}.frame.normal"))?;
    let (ul, vl, nl) = (length3(u), length3(v), length3(n));
    if ul <= 1.0e-9 || vl <= 1.0e-9 || nl <= 1.0e-9 {
        return Err(scene_error(
            &format!("{path}.frame"),
            "directions must be nonzero",
        ));
    }
    if (dot3(u, v) / (ul * vl)).abs() > 1.0e-6 {
        return Err(scene_error(
            &format!("{path}.frame"),
            "uDir and vDir must be orthogonal",
        ));
    }
    let cross = cross3(u, v);
    if dot3(cross, n) / (length3(cross) * nl) < 1.0 - 1.0e-6 {
        return Err(scene_error(
            &format!("{path}.frame.normal"),
            "must align with the right-handed uDir cross vDir normal",
        ));
    }
    positive_number(object.get("thicknessMm"), &format!("{path}.thicknessMm"))?;
    let outline = object
        .get("outline")
        .and_then(Value::as_array)
        .filter(|outline| outline.len() >= 3)
        .ok_or_else(|| {
            scene_error(
                &format!("{path}.outline"),
                "must contain at least three points",
            )
        })?;
    let polygon = outline
        .iter()
        .enumerate()
        .map(|(index, point)| vector(Some(point), &format!("{path}.outline[{index}]")))
        .collect::<Result<Vec<[f64; 2]>, _>>()?;
    if !polygon_is_simple_nonzero(&polygon) {
        return Err(scene_error(
            &format!("{path}.outline"),
            "must be a nonzero simple polygon",
        ));
    }
    let holes = match object.get("holes") {
        None | Some(Value::Null) => &[][..],
        Some(Value::Array(holes)) => holes.as_slice(),
        Some(_) => {
            return Err(scene_error(&format!("{path}.holes"), "must be an array"));
        }
    };
    let mut circles = Vec::with_capacity(holes.len());
    let mut rows = Vec::with_capacity(holes.len());
    for (index, hole) in holes.iter().enumerate() {
        let hole_path = format!("{path}.holes[{index}]");
        let hole = object_at(hole, &hole_path)?;
        let id = record_id(hole, &hole_path, ids)?;
        let center = vector(
            hole.get("center").or_else(|| hole.get("uv")),
            &format!("{hole_path}.center"),
        )?;
        let diameter = positive_number(hole.get("diameterMm"), &format!("{hole_path}.diameterMm"))?;
        if !point_in_polygon(center, &polygon)
            || polygon_edges(&polygon)
                .any(|(a, b)| point_segment_distance(center, a, b) + 1.0e-9 < diameter / 2.0)
        {
            return Err(scene_error(
                &hole_path,
                "must lie wholly inside the plate outline",
            ));
        }
        if circles
            .iter()
            .any(|(other, other_diameter): &([f64; 2], f64)| {
                let distance =
                    ((center[0] - other[0]).powi(2) + (center[1] - other[1]).powi(2)).sqrt();
                distance <= (diameter + *other_diameter) / 2.0 + 1.0e-9
            })
        {
            return Err(scene_error(
                &hole_path,
                "must not overlap or touch another hole",
            ));
        }
        circles.push((center, diameter));
        rows.push(serde_json::json!({
            "id": id,
            "status": "emitted",
            "kind": "hole",
            "parentId": parent_id,
            "renderedKind": "plate-hole"
        }));
    }
    Ok(rows)
}

fn classify_scene(scene: &Value) -> Result<SceneReceipt, AwareError> {
    scene_up(scene, true, "viewer-3d render")?;
    if let Some(units) = scene.get("meta").and_then(|meta| meta.get("units"))
        && units.as_str() != Some("mm")
    {
        return Err(scene_error("meta.units", "must be `mm` when present"));
    }
    let mut ids = HashSet::new();
    let mut physical = HashMap::new();
    let mut receipt = SceneReceipt::default();
    for (index, element) in object_array(scene, "elements")?.iter().enumerate() {
        let path = format!("elements[{index}]");
        let object = object_at(element, &path)?;
        let id = record_id(object, &path, &mut ids)?;
        let kind = object
            .get("kind")
            .and_then(Value::as_str)
            .filter(|kind| !kind.is_empty())
            .map(str::to_owned)
            .unwrap_or_else(|| {
                if object.contains_key("positions") || object.contains_key("indices") {
                    "mesh".to_owned()
                } else if object.contains_key("at") {
                    "node".to_owned()
                } else {
                    "member".to_owned()
                }
            });
        let mut nested_rows = Vec::new();
        if matches!(
            kind.as_str(),
            "node" | "mesh" | "plate" | "rod" | "bolt-shank" | "washer" | "nut" | "bolt-head"
        ) && object.contains_key("rot")
        {
            return Err(scene_error(
                &format!("{path}.rot"),
                "is applicable only to physical member, line, and box records",
            ));
        }
        match kind.as_str() {
            "line" | "box" | "member" => {
                let from = vector::<3>(object.get("from"), &format!("{path}.from"))?;
                let to = vector::<3>(object.get("to"), &format!("{path}.to"))?;
                if from == to {
                    return Err(scene_error(&path, "member axis must have nonzero length"));
                }
                member_roll(
                    object.get("rot"),
                    &format!("{path}.rot"),
                    "viewer-3d render",
                )?;
                validate_member_xsection(object, &path)?;
            }
            "node" => {
                vector::<3>(object.get("at"), &format!("{path}.at"))?;
                if object.contains_key("size") {
                    positive_number(object.get("size"), &format!("{path}.size"))?;
                }
            }
            "mesh" => validate_mesh(object, &path)?,
            "plate" => nested_rows = validate_plate(object, &path, &id, &mut ids)?,
            "rod" | "bolt-shank" => {
                axis(object, &path)?;
                positive_number(object.get("diameterMm"), &format!("{path}.diameterMm"))?;
            }
            "washer" => {
                direction(object, &path)?;
                vector::<3>(object.get("center"), &format!("{path}.center"))?;
                let outer = positive_number(
                    object.get("outerDiameterMm"),
                    &format!("{path}.outerDiameterMm"),
                )?;
                let inner = positive_number(
                    object.get("innerDiameterMm"),
                    &format!("{path}.innerDiameterMm"),
                )?;
                if inner >= outer {
                    return Err(scene_error(
                        &path,
                        "innerDiameterMm must be smaller than outerDiameterMm",
                    ));
                }
                positive_number(object.get("thicknessMm"), &format!("{path}.thicknessMm"))?;
            }
            "nut" | "bolt-head" => {
                direction(object, &path)?;
                vector::<3>(object.get("center"), &format!("{path}.center"))?;
                positive_number(
                    object.get("acrossFlatsMm"),
                    &format!("{path}.acrossFlatsMm"),
                )?;
                positive_number(object.get("thicknessMm"), &format!("{path}.thicknessMm"))?;
                if object.contains_key("phaseRad") {
                    finite_number(object.get("phaseRad"), &format!("{path}.phaseRad"))?;
                }
            }
            _ => {
                receipt.unsupported.push(serde_json::json!({
                    "id": id,
                    "status": "unsupported",
                    "kind": &kind,
                    "code": "unsupported-element-kind",
                    "message": format!("element kind '{kind}' is not supported by viewer-3d")
                }));
                continue;
            }
        }
        receipt.emitted.push(serde_json::json!({
            "id": &id,
            "status": "emitted",
            "kind": &kind,
            "renderedKind": &kind
        }));
        receipt.emitted.extend(nested_rows);
        physical.insert(id, kind);
    }
    classify_operations(scene, &physical, &mut ids, &mut receipt)?;
    classify_reference_systems(scene, &mut ids, &mut receipt)?;
    Ok(receipt)
}

fn validate_mesh(object: &serde_json::Map<String, Value>, path: &str) -> Result<(), AwareError> {
    let positions = object
        .get("positions")
        .and_then(Value::as_array)
        .filter(|positions| positions.len() >= 9 && positions.len() % 3 == 0)
        .ok_or_else(|| {
            scene_error(
                &format!("{path}.positions"),
                "must contain complete xyz triples for at least one triangle",
            )
        })?;
    for (index, value) in positions.iter().enumerate() {
        finite_number(Some(value), &format!("{path}.positions[{index}]"))?;
    }
    let indices = object
        .get("indices")
        .and_then(Value::as_array)
        .filter(|indices| !indices.is_empty() && indices.len() % 3 == 0)
        .ok_or_else(|| {
            scene_error(
                &format!("{path}.indices"),
                "must contain complete index triples",
            )
        })?;
    let vertex_count = positions.len() / 3;
    for (index, value) in indices.iter().enumerate() {
        value
            .as_u64()
            .filter(|value| (*value as usize) < vertex_count)
            .ok_or_else(|| {
                scene_error(
                    &format!("{path}.indices[{index}]"),
                    "must reference an existing vertex",
                )
            })?;
    }
    Ok(())
}

fn relation_target<'a>(
    object: &'a serde_json::Map<String, Value>,
    path: &str,
    field: &str,
    physical: &HashMap<String, String>,
    allowed: &[&str],
) -> Result<&'a str, AwareError> {
    let target = object
        .get(field)
        .and_then(Value::as_str)
        .filter(|target| !target.is_empty())
        .ok_or_else(|| scene_error(&format!("{path}.{field}"), "must be an element id"))?;
    match physical.get(target).map(String::as_str) {
        Some(kind) if allowed.contains(&kind) => Ok(target),
        Some(kind) => Err(scene_error(
            &format!("{path}.{field}"),
            &format!("references incompatible element kind `{kind}`"),
        )),
        None => Err(scene_error(
            &format!("{path}.{field}"),
            &format!("references unknown element `{target}`"),
        )),
    }
}

/// The element kinds a bolt can pass through and hole. A bolt's OWN components (`rod`, `bolt-shank`,
/// `nut`, `washer`, `bolt-head`) are deliberately absent — a bolt does not drill itself — which is
/// also what keeps a component from being smuggled in as an extra ply. Shared by the declared-pair
/// check and the per-effect ply check below so the two can never drift apart.
const BOLT_PLY_KINDS: [&str; 4] = ["member", "line", "box", "plate"];

fn classify_operations(
    scene: &Value,
    physical: &HashMap<String, String>,
    ids: &mut HashSet<String>,
    receipt: &mut SceneReceipt,
) -> Result<(), AwareError> {
    let mut claimed_bolt_components = HashSet::new();
    for (index, operation) in object_array(scene, "operations")?.iter().enumerate() {
        let path = format!("operations[{index}]");
        let object = object_at(operation, &path)?;
        let id = record_id(object, &path, ids)?;
        let kind = object
            .get("kind")
            .and_then(Value::as_str)
            .ok_or_else(|| scene_error(&format!("{path}.kind"), "must be a string"))?;

        match kind {
            "bolt-array" => {
                let part_kinds = BOLT_PLY_KINDS;
                let part_to_bolt_to =
                    relation_target(object, &path, "partToBoltTo", physical, &part_kinds)?;
                let part_to_be_bolted =
                    relation_target(object, &path, "partToBeBolted", physical, &part_kinds)?;
                if part_to_bolt_to == part_to_be_bolted {
                    return Err(scene_error(
                        &path,
                        "bolt-array participants must be distinct elements",
                    ));
                }
                let frame = object
                    .get("frame")
                    .and_then(Value::as_object)
                    .ok_or_else(|| scene_error(&format!("{path}.frame"), "must be an object"))?;
                vector::<3>(frame.get("origin"), &format!("{path}.frame.origin"))?;
                let u = vector::<3>(frame.get("uDir"), &format!("{path}.frame.uDir"))?;
                let v = vector::<3>(frame.get("vDir"), &format!("{path}.frame.vDir"))?;
                let normal = vector::<3>(frame.get("normal"), &format!("{path}.frame.normal"))?;
                let (u, v, normal) = (normalized3(u), normalized3(v), normalized3(normal));
                let (Some(u), Some(v), Some(normal)) = (u, v, normal) else {
                    return Err(scene_error(
                        &format!("{path}.frame"),
                        "directions must be nonzero and finite",
                    ));
                };
                let cross = cross3(u, v);
                // A zero-length cross product means `uDir` and `vDir` are
                // parallel, which is precisely the "not orthonormal" case
                // below — report it rather than panicking on the normalize.
                let orthonormal =
                    normalized3(cross).is_some_and(|cross| dot3(cross, normal) >= 1.0 - 1.0e-6);
                if dot3(u, v).abs() > 1.0e-6 || !orthonormal {
                    return Err(scene_error(
                        &format!("{path}.frame"),
                        "must be right-handed and orthonormal",
                    ));
                }
                for field in ["uOffsetsMm", "vOffsetsMm"] {
                    let values = object
                        .get(field)
                        .and_then(Value::as_array)
                        .filter(|values| !values.is_empty())
                        .ok_or_else(|| {
                            scene_error(&format!("{path}.{field}"), "must be a non-empty array")
                        })?;
                    for (offset_index, value) in values.iter().enumerate() {
                        finite_number(Some(value), &format!("{path}.{field}[{offset_index}]"))?;
                    }
                }
                positive_number(object.get("diameterMm"), &format!("{path}.diameterMm"))?;
                if object
                    .get("standard")
                    .and_then(Value::as_str)
                    .is_none_or(|standard| standard.trim().is_empty())
                {
                    return Err(scene_error(
                        &format!("{path}.standard"),
                        "must be a non-empty string",
                    ));
                }
                if object.contains_key("toleranceMm") {
                    let tolerance =
                        finite_number(object.get("toleranceMm"), &format!("{path}.toleranceMm"))?;
                    if tolerance < 0.0 {
                        return Err(scene_error(
                            &format!("{path}.toleranceMm"),
                            "must be nonnegative",
                        ));
                    }
                }
                match object.get("boltType").and_then(Value::as_str) {
                    Some("shop" | "site") => {}
                    _ => {
                        return Err(scene_error(
                            &format!("{path}.boltType"),
                            "must be `shop` or `site`",
                        ));
                    }
                }
                let components = object
                    .get("components")
                    .and_then(Value::as_object)
                    .ok_or_else(|| {
                        scene_error(&format!("{path}.components"), "must be an object")
                    })?;
                for field in ["bolt", "nut", "washer"] {
                    if components
                        .get(field)
                        .is_some_and(|value| !value.is_boolean())
                    {
                        return Err(scene_error(
                            &format!("{path}.components.{field}"),
                            "must be a boolean",
                        ));
                    }
                }
                if components.get("bolt").and_then(Value::as_bool) == Some(false) {
                    return Err(scene_error(
                        &format!("{path}.components.bolt"),
                        "must be true when instances author shankId and headId",
                    ));
                }
                receipt.emitted.push(serde_json::json!({
                    "id": id, "status": "emitted", "kind": kind,
                    "renderedKind": "relationship", "geometryDuplicated": false
                }));
            }
            "weld" => {
                let weld_kinds = ["member", "line", "box", "plate", "rod"];
                relation_target(object, &path, "mainId", physical, &weld_kinds)?;
                relation_target(object, &path, "secondaryId", physical, &weld_kinds)?;
                if object.get("weldType").and_then(Value::as_str) != Some("fillet") {
                    return Err(scene_error(&format!("{path}.weldType"), "must be `fillet`"));
                }
                positive_number(object.get("sizeMm"), &format!("{path}.sizeMm"))?;
                for field in ["around", "shop"] {
                    if !object.get(field).is_some_and(Value::is_boolean) {
                        return Err(scene_error(&format!("{path}.{field}"), "must be a boolean"));
                    }
                }
                let points = object
                    .get("path")
                    .and_then(Value::as_array)
                    .filter(|points| points.len() >= 2)
                    .ok_or_else(|| {
                        scene_error(&format!("{path}.path"), "must have at least two points")
                    })?;
                for (point_index, point) in points.iter().enumerate() {
                    vector::<3>(Some(point), &format!("{path}.path[{point_index}]"))?;
                }
                receipt.emitted.push(serde_json::json!({
                    "id": id, "status": "emitted", "kind": kind,
                    "renderedKind": "weld-path", "geometryDuplicated": false
                }));
            }
            "boolean-cut" => {
                relation_target(
                    object,
                    &path,
                    "targetId",
                    physical,
                    &[
                        "member",
                        "line",
                        "box",
                        "plate",
                        "rod",
                        "bolt-shank",
                        "washer",
                        "nut",
                        "bolt-head",
                        "mesh",
                    ],
                )?;
                let tool = object
                    .get("tool")
                    .and_then(Value::as_object)
                    .ok_or_else(|| scene_error(&format!("{path}.tool"), "must be an object"))?;
                // Accept both finite tool kinds so a box cope passes validation (viewer-3d approximates no
                // Boolean CSG for either, so both stay unsupported below — but neither aborts the scene).
                match tool.get("kind").and_then(Value::as_str) {
                    Some("cylinder") => {
                        let tool_axis =
                            tool.get("axis").and_then(Value::as_object).ok_or_else(|| {
                                scene_error(&format!("{path}.tool.axis"), "must be an object")
                            })?;
                        let from =
                            vector::<3>(tool_axis.get("from"), &format!("{path}.tool.axis.from"))?;
                        let to = vector::<3>(tool_axis.get("to"), &format!("{path}.tool.axis.to"))?;
                        if from == to {
                            return Err(scene_error(
                                &format!("{path}.tool.axis"),
                                "must have nonzero length",
                            ));
                        }
                        positive_number(
                            tool.get("diameterMm"),
                            &format!("{path}.tool.diameterMm"),
                        )?;
                    }
                    Some("box") => {
                        let frame =
                            tool.get("frame")
                                .and_then(Value::as_object)
                                .ok_or_else(|| {
                                    scene_error(&format!("{path}.tool.frame"), "must be an object")
                                })?;
                        for axis in ["origin", "uDir", "vDir", "normal"] {
                            vector::<3>(frame.get(axis), &format!("{path}.tool.frame.{axis}"))?;
                        }
                        let he = tool
                            .get("halfExtents")
                            .and_then(Value::as_array)
                            .filter(|a| a.len() == 3)
                            .ok_or_else(|| {
                                scene_error(
                                    &format!("{path}.tool.halfExtents"),
                                    "must be three numbers",
                                )
                            })?;
                        for (i, h) in he.iter().enumerate() {
                            positive_number(Some(h), &format!("{path}.tool.halfExtents[{i}]"))?;
                        }
                    }
                    _ => {
                        return Err(scene_error(
                            &format!("{path}.tool.kind"),
                            "must be `cylinder` or `box`",
                        ));
                    }
                }
                receipt.unsupported.push(serde_json::json!({
                    "id": id, "status": "unsupported", "kind": kind,
                    "code": "exact-csg-not-available",
                    "message": "viewer-3d does not approximate Boolean CSG"
                }));
            }
            _ => {
                receipt.unsupported.push(serde_json::json!({
                    "id": id, "status": "unsupported", "kind": kind,
                    "code": "unsupported-operation-kind",
                    "message": format!("operation kind '{kind}' is not supported by viewer-3d")
                }));
                continue;
            }
        }

        if kind != "bolt-array" {
            continue;
        }
        let participant_a = object
            .get("partToBoltTo")
            .and_then(Value::as_str)
            .ok_or_else(|| scene_error(&format!("{path}.partToBoltTo"), "must be an element id"))?;
        let participant_b = object
            .get("partToBeBolted")
            .and_then(Value::as_str)
            .ok_or_else(|| {
                scene_error(&format!("{path}.partToBeBolted"), "must be an element id")
            })?;
        let frame = object
            .get("frame")
            .and_then(Value::as_object)
            .ok_or_else(|| scene_error(&format!("{path}.frame"), "must be an object"))?;
        let origin = vector::<3>(frame.get("origin"), &format!("{path}.frame.origin"))?;
        let u = normalized3(vector::<3>(
            frame.get("uDir"),
            &format!("{path}.frame.uDir"),
        )?)
        .ok_or_else(|| scene_error(&format!("{path}.frame.uDir"), "must be nonzero"))?;
        let v = normalized3(vector::<3>(
            frame.get("vDir"),
            &format!("{path}.frame.vDir"),
        )?)
        .ok_or_else(|| scene_error(&format!("{path}.frame.vDir"), "must be nonzero"))?;
        let normal = normalized3(vector::<3>(
            frame.get("normal"),
            &format!("{path}.frame.normal"),
        )?)
        .ok_or_else(|| scene_error(&format!("{path}.frame.normal"), "must be nonzero"))?;
        let u_offsets = number_array(object.get("uOffsetsMm"), &format!("{path}.uOffsetsMm"))?;
        let v_offsets = number_array(object.get("vOffsetsMm"), &format!("{path}.vOffsetsMm"))?;
        for (field, offsets) in [("uOffsetsMm", &u_offsets), ("vOffsetsMm", &v_offsets)] {
            if offsets
                .iter()
                .enumerate()
                .any(|(index, value)| offsets[..index].contains(value))
            {
                return Err(scene_error(
                    &format!("{path}.{field}"),
                    "must contain unique offsets",
                ));
            }
        }
        let diameter = finite_number(object.get("diameterMm"), &format!("{path}.diameterMm"))?;
        let tolerance = object
            .get("toleranceMm")
            .and_then(Value::as_f64)
            .unwrap_or(0.0);
        let mut expected_points = u_offsets
            .iter()
            .flat_map(|u_offset| {
                v_offsets.iter().map(move |v_offset| {
                    [
                        origin[0] + u[0] * u_offset + v[0] * v_offset,
                        origin[1] + u[1] * u_offset + v[1] * v_offset,
                        origin[2] + u[2] * u_offset + v[2] * v_offset,
                    ]
                })
            })
            .collect::<Vec<_>>();
        let instances = object
            .get("instances")
            .and_then(Value::as_array)
            .ok_or_else(|| scene_error(&format!("{path}.instances"), "must be an array"))?;
        let expected = u_offsets.len() * v_offsets.len();
        if instances.len() != expected {
            return Err(scene_error(
                &format!("{path}.instances"),
                "must match the Cartesian offset count",
            ));
        }
        for (instance_index, instance) in instances.iter().enumerate() {
            let instance_path = format!("{path}.instances[{instance_index}]");
            let instance = object_at(instance, &instance_path)?;
            let instance_id = record_id(instance, &instance_path, ids)?;
            let point = vector::<3>(instance.get("point"), &format!("{instance_path}.point"))?;
            let Some(point_index) = expected_points
                .iter()
                .position(|expected| distance3(*expected, point) <= 0.1)
            else {
                return Err(scene_error(
                    &format!("{instance_path}.point"),
                    "must uniquely match an authored Cartesian offset position",
                ));
            };
            expected_points.remove(point_index);
            for (field, expected_kind) in [("shankId", "bolt-shank"), ("headId", "bolt-head")] {
                let child = instance.get(field).and_then(Value::as_str).ok_or_else(|| {
                    scene_error(&format!("{instance_path}.{field}"), "must be an element id")
                })?;
                if physical.get(child).map(String::as_str) != Some(expected_kind) {
                    return Err(scene_error(
                        &format!("{instance_path}.{field}"),
                        "references an incompatible or unknown element",
                    ));
                }
                if !claimed_bolt_components.insert(child) {
                    return Err(scene_error(
                        &format!("{instance_path}.{field}"),
                        "must not reuse a component already claimed by another bolt instance",
                    ));
                }
                let child_element = scene_element(scene, child).ok_or_else(|| {
                    scene_error(
                        &format!("{instance_path}.{field}"),
                        "references an unknown element",
                    )
                })?;
                if field == "shankId" {
                    let (from, to) = axis(child_element, child)?;
                    let direction =
                        normalized3([to[0] - from[0], to[1] - from[1], to[2] - from[2]])
                            .ok_or_else(|| {
                                scene_error(
                                    &format!("{instance_path}.{field}"),
                                    "shank axis must have nonzero length",
                                )
                            })?;
                    let offset = [point[0] - from[0], point[1] - from[1], point[2] - from[2]];
                    let child_diameter = positive_number(
                        child_element.get("diameterMm"),
                        &format!("{instance_path}.{field}.diameterMm"),
                    )?;
                    if (dot3(direction, normal).abs() - 1.0).abs() > 1.0e-6
                        || length3(cross3(offset, direction)) > 0.1
                        || (child_diameter - diameter).abs() > 0.1
                    {
                        return Err(scene_error(
                            &format!("{instance_path}.{field}"),
                            "shank axis and diameter must match the bolt instance",
                        ));
                    }
                } else {
                    let center = vector::<3>(
                        child_element.get("center"),
                        &format!("{instance_path}.{field}.center"),
                    )?;
                    let child_axis =
                        normalized3(direction(child_element, child)?).ok_or_else(|| {
                            scene_error(
                                &format!("{instance_path}.{field}"),
                                "axis must have nonzero length",
                            )
                        })?;
                    let offset = [
                        center[0] - point[0],
                        center[1] - point[1],
                        center[2] - point[2],
                    ];
                    if (dot3(child_axis, normal).abs() - 1.0).abs() > 1.0e-6
                        || length3(cross3(offset, normal)) > 0.1
                    {
                        return Err(scene_error(
                            &format!("{instance_path}.{field}"),
                            "must be centered and aligned on the bolt instance axis",
                        ));
                    }
                }
            }
            for (field, expected_kind) in [("nutIds", "nut"), ("washerIds", "washer")] {
                let children = instance
                    .get(field)
                    .and_then(Value::as_array)
                    .ok_or_else(|| {
                        scene_error(&format!("{instance_path}.{field}"), "must be an array")
                    })?;
                for (child_index, child) in children.iter().enumerate() {
                    let child = child.as_str().ok_or_else(|| {
                        scene_error(
                            &format!("{instance_path}.{field}[{child_index}]"),
                            "must be an element id",
                        )
                    })?;
                    if physical.get(child).map(String::as_str) != Some(expected_kind) {
                        return Err(scene_error(
                            &format!("{instance_path}.{field}[{child_index}]"),
                            "references an incompatible or unknown element",
                        ));
                    }
                    if !claimed_bolt_components.insert(child) {
                        return Err(scene_error(
                            &format!("{instance_path}.{field}[{child_index}]"),
                            "must not reuse a component already claimed by another bolt instance",
                        ));
                    }
                    let child_element = scene_element(scene, child).ok_or_else(|| {
                        scene_error(
                            &format!("{instance_path}.{field}[{child_index}]"),
                            "references an unknown element",
                        )
                    })?;
                    let center = vector::<3>(
                        child_element.get("center"),
                        &format!("{instance_path}.{field}[{child_index}].center"),
                    )?;
                    let child_axis =
                        normalized3(direction(child_element, child)?).ok_or_else(|| {
                            scene_error(
                                &format!("{instance_path}.{field}[{child_index}]"),
                                "axis must have nonzero length",
                            )
                        })?;
                    let offset = [
                        center[0] - point[0],
                        center[1] - point[1],
                        center[2] - point[2],
                    ];
                    if (dot3(child_axis, normal).abs() - 1.0).abs() > 1.0e-6
                        || length3(cross3(offset, normal)) > 0.1
                    {
                        return Err(scene_error(
                            &format!("{instance_path}.{field}[{child_index}]"),
                            "must be centered and aligned on the bolt instance axis",
                        ));
                    }
                }
            }
            receipt.emitted.push(serde_json::json!({
                "id": instance_id, "status": "emitted", "kind": "bolt-instance",
                "realizedBy": id, "renderedKind": "relationship", "geometryDuplicated": false
            }));
            let effects = instance
                .get("holeEffects")
                .and_then(Value::as_array)
                .ok_or_else(|| {
                    scene_error(&format!("{instance_path}.holeEffects"), "must be an array")
                })?;
            // One effect per PLY the bolt passes through — the declared pair at minimum, and more when
            // the joint is genuinely multi-ply. A double-sided gusset bolts plate + angle leg + plate:
            // three plies, three holes, one bolt in double shear. Pinning the count at two refused the
            // whole scene over an ordinary detail. See the matching relaxation in `render/ifc.rs`.
            if effects.len() < 2 {
                return Err(scene_error(
                    &format!("{instance_path}.holeEffects"),
                    "must contain at least one effect per bolt participant",
                ));
            }
            let mut effect_targets = HashSet::new();
            for (effect_index, effect) in effects.iter().enumerate() {
                let effect_path = format!("{instance_path}.holeEffects[{effect_index}]");
                let effect = object_at(effect, &effect_path)?;
                let effect_id = record_id(effect, &effect_path, ids)?;
                let target = effect
                    .get("targetId")
                    .and_then(Value::as_str)
                    .ok_or_else(|| {
                        scene_error(&format!("{effect_path}.targetId"), "must be an element id")
                    })?;
                // A ply beyond the declared pair is held to the same bar the pair is held to — a real
                // element of a kind a bolt can hole. Without that an unknown id would validate here and
                // then be silently dropped downstream, leaving a ply that is fabricated with no hole in
                // it while the receipt still reads clean.
                if ![participant_a, participant_b].contains(&target)
                    && !physical
                        .get(target)
                        .is_some_and(|kind| BOLT_PLY_KINDS.contains(&kind.as_str()))
                {
                    return Err(scene_error(
                        &format!("{effect_path}.targetId"),
                        "must reference a bolt participant or another element the bolt passes through",
                    ));
                }
                if !effect_targets.insert(target) {
                    return Err(scene_error(
                        &format!("{effect_path}.targetId"),
                        "must not hole the same ply twice",
                    ));
                }
                let center = vector::<3>(effect.get("center"), &format!("{effect_path}.center"))?;
                if distance3(center, point) > 0.1 {
                    return Err(scene_error(
                        &format!("{effect_path}.center"),
                        "must match the bolt instance point",
                    ));
                }
                let effect_axis = vector::<3>(effect.get("axis"), &format!("{effect_path}.axis"))?;
                let Some(effect_axis) = normalized3(effect_axis) else {
                    return Err(scene_error(
                        &format!("{effect_path}.axis"),
                        "must be nonzero",
                    ));
                };
                if (dot3(effect_axis, normal).abs() - 1.0).abs() > 1.0e-6 {
                    return Err(scene_error(
                        &format!("{effect_path}.axis"),
                        "must align with the bolt frame normal",
                    ));
                }
                let effect_diameter = positive_number(
                    effect.get("diameterMm"),
                    &format!("{effect_path}.diameterMm"),
                )?;
                if (effect_diameter - (diameter + tolerance)).abs() > 0.1 {
                    return Err(scene_error(
                        &format!("{effect_path}.diameterMm"),
                        "must equal bolt diameter plus tolerance",
                    ));
                }
                receipt.emitted.push(serde_json::json!({
                    "id": effect_id, "status": "emitted", "kind": "hole-effect",
                    "realizedBy": id, "renderedKind": "relationship", "geometryDuplicated": false
                }));
            }
            // Extra plies are allowed; the declared pair is still mandatory. A count test would let a
            // 3-ply stack that MISSES one of the pair pass on arithmetic alone.
            if !effect_targets.contains(participant_a) || !effect_targets.contains(participant_b) {
                return Err(scene_error(
                    &format!("{instance_path}.holeEffects"),
                    "must cover both bolt participants",
                ));
            }
        }
        if !expected_points.is_empty() {
            return Err(scene_error(
                &format!("{path}.instances"),
                "must exhaust the authored Cartesian offset positions",
            ));
        }
    }
    Ok(())
}

fn classify_reference_systems(
    scene: &Value,
    ids: &mut HashSet<String>,
    receipt: &mut SceneReceipt,
) -> Result<(), AwareError> {
    for (index, reference) in object_array(scene, "referenceSystems")?.iter().enumerate() {
        let path = format!("referenceSystems[{index}]");
        let object = object_at(reference, &path)?;
        let id = record_id(object, &path, ids)?;
        let kind = object
            .get("kind")
            .and_then(Value::as_str)
            .ok_or_else(|| scene_error(&format!("{path}.kind"), "must be a string"))?;
        if kind != "structural-grid" {
            receipt.unsupported.push(serde_json::json!({
                "id": id,
                "status": "unsupported",
                "kind": kind,
                "code": "unsupported-reference-system-kind",
                "message": format!("reference-system kind '{kind}' is not supported by viewer-3d")
            }));
            for (collection, child_kind) in [("axes", "grid-axis"), ("levels", "elevation-datum")] {
                if let Some(children) = object.get(collection).and_then(Value::as_array) {
                    for (child_index, child) in children.iter().enumerate() {
                        let child_path = format!("{path}.{collection}[{child_index}]");
                        let child = object_at(child, &child_path)?;
                        let child_id = record_id(child, &child_path, ids)?;
                        receipt.unsupported.push(serde_json::json!({
                            "id": child_id,
                            "status": "unsupported",
                            "kind": child_kind,
                            "code": "unsupported-parent",
                            "parentId": id,
                            "message": format!("parent reference-system kind '{kind}' is not supported")
                        }));
                    }
                }
            }
            continue;
        }
        vector::<3>(object.get("origin"), &format!("{path}.origin"))?;
        validate_grid_bounds(object, &path)?;
        receipt.emitted.push(serde_json::json!({
            "id": id,
            "status": "emitted",
            "kind": kind,
            "renderedKind": "structural-grid"
        }));
        for collection in ["axes", "levels"] {
            let records = object
                .get(collection)
                .and_then(Value::as_array)
                .ok_or_else(|| scene_error(&format!("{path}.{collection}"), "must be an array"))?;
            if collection == "levels" && records.is_empty() {
                return Err(scene_error(
                    &format!("{path}.{collection}"),
                    "must contain at least one elevation datum",
                ));
            }
            for (child_index, child) in records.iter().enumerate() {
                let child_path = format!("{path}.{collection}[{child_index}]");
                let child = object_at(child, &child_path)?;
                let child_id = record_id(child, &child_path, ids)?;
                if collection == "axes" {
                    match child.get("direction").and_then(Value::as_str) {
                        Some("x" | "y") => {}
                        _ => {
                            return Err(scene_error(
                                &format!("{child_path}.direction"),
                                "must be `x` or `y`",
                            ));
                        }
                    }
                    finite_number(child.get("offsetMm"), &format!("{child_path}.offsetMm"))?;
                    for key in ["startMm", "endMm"] {
                        if child.contains_key(key) {
                            finite_number(child.get(key), &format!("{child_path}.{key}"))?;
                        }
                    }
                } else {
                    finite_number(
                        child.get("elevationMm"),
                        &format!("{child_path}.elevationMm"),
                    )?;
                }
                if child.get("label").and_then(Value::as_str).is_none() {
                    return Err(scene_error(
                        &format!("{child_path}.label"),
                        "must be a string",
                    ));
                }
                receipt.emitted.push(serde_json::json!({
                    "id": child_id,
                    "status": "emitted",
                    "kind": if collection == "axes" { "grid-axis" } else { "elevation-datum" },
                    "parentId": id,
                    "renderedKind": if collection == "axes" { "grid-line" } else { "level-crosshair" }
                }));
            }
        }
    }
    Ok(())
}

fn validate_grid_bounds(
    object: &serde_json::Map<String, Value>,
    path: &str,
) -> Result<(), AwareError> {
    let bounds_path = format!("{path}.bounds");
    let bounds = object_at(
        object
            .get("bounds")
            .ok_or_else(|| scene_error(&bounds_path, "is required"))?,
        &bounds_path,
    )?;
    let min_x = finite_number(bounds.get("minX"), &format!("{bounds_path}.minX"))?;
    let max_x = finite_number(bounds.get("maxX"), &format!("{bounds_path}.maxX"))?;
    let min_y = finite_number(bounds.get("minY"), &format!("{bounds_path}.minY"))?;
    let max_y = finite_number(bounds.get("maxY"), &format!("{bounds_path}.maxY"))?;
    if min_x >= max_x || min_y >= max_y {
        return Err(scene_error(
            &bounds_path,
            "must have increasing min/max extents",
        ));
    }
    Ok(())
}

/// The ids a legend row may name as a `target`.
///
/// Deliberately NARROW: a top-level `element`, or a rendered `kind:"weld"` operation — the only
/// two things the viewer draws as independently operable objects. Structural grids, their
/// axes/levels and labels, plate holes, non-rendered operations, lights and helpers are excluded;
/// addressing those would need one-to-many target mappings and buys nothing for this panel.
fn legend_target_ids(scene: &Value, emitted: &HashSet<String>) -> HashSet<String> {
    let mut ids = HashSet::new();
    if let Some(Value::Array(elements)) = scene.get("elements") {
        for e in elements {
            // Only what the renderer EMITS. An unsupported element kind is skipped at render time,
            // so accepting its id here would pass validation and then quietly drop the row later —
            // the opposite of the promised wholesale fallback.
            // `emitted` is necessary but not sufficient: classify_scene can INFER `member` from
            // geometry when `kind` is a truthy non-string, while the browser does
            // `switch(el.kind||'box')` and skips it. Require the renderer's own shape.
            if let Some(id) = e.get("id").and_then(Value::as_str)
                && emitted.contains(id)
                && browser_renders_kind(e)
            {
                ids.insert(id.to_string());
            }
        }
    }
    if let Some(Value::Array(ops)) = scene.get("operations") {
        for op in ops {
            if op.get("kind").and_then(Value::as_str) == Some("weld")
                && let Some(id) = op.get("id").and_then(Value::as_str)
                && emitted.contains(id)
            {
                ids.insert(id.to_string());
            }
        }
    }
    ids
}

/// group key → the target ids in it, so a row naming only `groups` resolves the same way the
/// renderer will. Insertion order is irrelevant here (membership only); the PANEL's order always
/// comes from the descriptor's own arrays.
fn legend_group_members(scene: &Value, emitted: &HashSet<String>) -> HashMap<String, Vec<String>> {
    let mut by_group: HashMap<String, Vec<String>> = HashMap::new();
    if let Some(Value::Array(elements)) = scene.get("elements") {
        for e in elements {
            if let (Some(id), Some(g)) = (
                e.get("id").and_then(Value::as_str),
                e.get("group").and_then(Value::as_str),
            ) && emitted.contains(id)
                && browser_renders_kind(e)
            {
                by_group
                    .entry(g.to_string())
                    .or_default()
                    .push(id.to_string());
            }
        }
    }
    by_group
}

/// The browser resolves an element's kind as `el.kind || 'box'` and switches on it, so a kind
/// that is present but not a string can never match a case — it is skipped no matter what the
/// Rust-side classifier inferred from the geometry.
fn browser_renders_kind(e: &Value) -> bool {
    match e.get("kind") {
        None | Some(Value::Null) => true,
        Some(Value::String(_)) => true,
        Some(Value::Bool(false)) => true,
        Some(Value::Number(n)) => n.as_f64() == Some(0.0),
        Some(_) => false,
    }
}

fn legend_named(v: Option<&Value>) -> bool {
    matches!(v, Some(Value::String(s)) if !s.trim().is_empty())
}

/// Validate `scene.legend` — the producer-authored objects panel — as a whole.
///
/// `None` = absent or wholly valid. `Some(reason)` = it must be IGNORED in favour of the legacy
/// flat list. Ignoring rather than erroring is deliberate: the panel is chrome and the model is the
/// payload, so a producer bug in the panel must never cost the user their model. It is checked
/// ATOMICALLY — a half-valid descriptor renders an ambiguous panel where some rows silently control
/// nothing, which is worse than the honest legacy list.
fn legend_problem(scene: &Value, emitted: &HashSet<String>) -> Option<String> {
    let legend = match scene.get("legend") {
        None | Some(Value::Null) => return None,
        Some(v @ Value::Object(_)) => v,
        Some(other) => return Some(format!("must be an object (got {})", json_type(other))),
    };

    match legend.get("v").and_then(Value::as_u64) {
        Some(1) => {}
        Some(other) => return Some(format!("unknown version {other} (this renderer speaks v1)")),
        None => return Some("`v` is required (expected 1)".into()),
    }
    // Required in v1: the row semantics travel WITH the descriptor the producer authored, and are
    // never inferred from some field happening to be present.
    match legend.get("interaction").and_then(Value::as_str) {
        Some("select") => {}
        Some(other) => return Some(format!("unknown interaction `{other}` (expected `select`)")),
        None => return Some("`interaction` is required in v1 (expected `select`)".into()),
    }

    let modes = match legend.get("modes") {
        Some(Value::Array(m)) if !m.is_empty() => m,
        _ => return Some("`modes` must be a non-empty array".into()),
    };

    let valid_targets = legend_target_ids(scene, emitted);
    let group_members = legend_group_members(scene, emitted);

    for (mi, mode) in modes.iter().enumerate() {
        if !legend_named(mode.get("key")) || !legend_named(mode.get("label")) {
            return Some(format!("modes[{mi}] needs a non-empty `key` and `label`"));
        }
        let sections = match mode.get("sections") {
            Some(Value::Array(s)) if !s.is_empty() => s,
            _ => return Some(format!("modes[{mi}].sections must be a non-empty array")),
        };

        // Row keys are unique per MODE, not merely per parent: a part-group name such as `weld`
        // legitimately recurs under several connection categories, and the Shift-range anchor
        // stores a bare row key — duplicates would make a range ambiguous.
        let mut row_keys: HashSet<&str> = HashSet::new();
        // Category keys share the collapse state and the header identity, so a duplicate makes one
        // header vanish and ties the two categories' collapse together.
        let mut cat_keys: HashSet<&str> = HashSet::new();
        // A target belongs to at most one LEAF row per mode; cross-cutting classifications belong
        // in separate modes. Category headers aggregate descendants without being rows themselves.
        let mut claimed: HashMap<String, String> = HashMap::new();

        for (si, section) in sections.iter().enumerate() {
            if !legend_named(section.get("key")) {
                return Some(format!(
                    "modes[{mi}].sections[{si}].key must be a non-empty string"
                ));
            }
            let categories = match section.get("categories") {
                Some(Value::Array(c)) if !c.is_empty() => c,
                _ => {
                    return Some(format!(
                        "modes[{mi}].sections[{si}].categories must be a non-empty array"
                    ));
                }
            };
            for (ci, category) in categories.iter().enumerate() {
                if let Some(ck) = category.get("key").and_then(Value::as_str)
                    && !ck.is_empty()
                    && !cat_keys.insert(ck)
                {
                    return Some(format!(
                        "modes[{mi}].sections[{si}].categories[{ci}].key `{ck}` recurs within one mode — category keys must be unique per mode"
                    ));
                }
                let rows = match category.get("rows") {
                    Some(Value::Array(r)) if !r.is_empty() => r,
                    _ => {
                        return Some(format!(
                            "modes[{mi}].sections[{si}].categories[{ci}].rows must be a non-empty array"
                        ));
                    }
                };
                for (ri, row) in rows.iter().enumerate() {
                    let at = format!("modes[{mi}].sections[{si}].categories[{ci}].rows[{ri}]");
                    let key = match row.get("key").and_then(Value::as_str) {
                        Some(k) if !k.trim().is_empty() => k,
                        _ => return Some(format!("{at}.key must be a non-empty string")),
                    };
                    if !row_keys.insert(key) {
                        return Some(format!(
                            "{at}.key `{key}` recurs within one mode — row keys must be unique per mode"
                        ));
                    }
                    if !legend_named(row.get("label")) {
                        return Some(format!("{at}.label must be a non-empty string"));
                    }

                    // Resolve exactly as the renderer will: explicit targets win, else the groups.
                    let mut resolved: Vec<String> = Vec::new();
                    match row.get("targets") {
                        Some(Value::Array(t)) if !t.is_empty() => {
                            for v in t {
                                match v.as_str() {
                                    Some(id) if valid_targets.contains(id) => {
                                        resolved.push(id.to_string())
                                    }
                                    Some(id) => {
                                        return Some(format!(
                                            "{at}.targets names `{id}`, which is not a rendered element or weld operation"
                                        ));
                                    }
                                    None => return Some(format!("{at}.targets must be strings")),
                                }
                            }
                        }
                        Some(Value::Array(_)) => {
                            return Some(format!("{at}.targets must not be empty"));
                        }
                        Some(_) => return Some(format!("{at}.targets must be an array")),
                        None => {
                            let groups = match row.get("groups") {
                                Some(Value::Array(g)) if !g.is_empty() => g,
                                _ => return Some(format!("{at} must name `groups` or `targets`")),
                            };
                            for g in groups {
                                let Some(gk) = g.as_str() else {
                                    return Some(format!("{at}.groups must be strings"));
                                };
                                match group_members.get(gk) {
                                    Some(ids) => resolved.extend(ids.iter().cloned()),
                                    None => {
                                        return Some(format!(
                                            "{at}.groups names `{gk}`, which no element belongs to"
                                        ));
                                    }
                                }
                            }
                        }
                    }

                    for id in resolved {
                        if let Some(owner) = claimed.get(&id) {
                            return Some(format!(
                                "{at} claims target `{id}`, already claimed by row `{owner}` in the same mode"
                            ));
                        }
                        claimed.insert(id, key.to_string());
                    }
                }
            }
        }
    }
    None
}

pub fn viewer_3d_render(args: &Value, dry_run: bool) -> Result<Value, AwareError> {
    // The scene is the payload; require an object so the renderer has something to draw.
    let scene = match args.get("scene") {
        Some(v @ Value::Object(_)) => v,
        None | Some(Value::Null) => {
            return Err(AwareError::Validation(
                "viewer-3d render: `scene` is required (an object with `elements`)".into(),
            ));
        }
        Some(other) => {
            return Err(AwareError::Validation(format!(
                "viewer-3d render: `scene` must be an object (got {})",
                json_type(other)
            )));
        }
    };
    // Validate and classify every record before producing any HTML or touching output-path.
    // A malformed supported record therefore fails atomically instead of disappearing in JS.
    let mut receipt = classify_scene(scene)?;

    // Serialize the scene and inject it into the renderer shell as a JS object-literal
    // expression. Neutralize EVERY `<` as a `<` escape that renders back
    // to `<` at runtime) so no HTML-tokenizer-significant sequence can survive in a string
    // value: not just `</script>` (close) but also `<!--` / `<script` (which would push the
    // tokenizer into the script-data-(double-)escaped state and stop the template's own
    // closing `</script>` from closing the element). JSON only contains `<` inside string
    // values, so escaping all of them is safe. Also escape the JS line terminators U+2028/U+2029.
    // An unusable objects-panel descriptor is dropped rather than rendered half-valid, and the
    // reason travels to the producer as `legendError` (the page console-warns it). Cloning only on
    // the failure path keeps the common case allocation-free.
    let emitted_ids: HashSet<String> = receipt
        .emitted
        .iter()
        .filter_map(|e| e.get("id").and_then(Value::as_str).map(str::to_string))
        .collect();
    let repaired;
    let scene = match legend_problem(scene, &emitted_ids) {
        None => scene,
        Some(reason) => {
            // Surface it in the RESULT too. A headless producer must be able to see that its
            // panel was rejected without parsing the generated HTML or opening a browser console.
            receipt.warnings.push(serde_json::json!({
                "status": "warning",
                "code": "legend-ignored",
                "message": format!("scene.legend ignored, falling back to the flat list: {reason}")
            }));
            repaired = {
                let mut s = scene.clone();
                if let Some(obj) = s.as_object_mut() {
                    obj.remove("legend");
                    obj.insert("legendError".into(), Value::String(reason));
                }
                s
            };
            &repaired
        }
    };
    let scene_json = serde_json::to_string(scene)
        .map_err(|e| AwareError::Internal(format!("viewer-3d: serialize scene: {e}")))?
        .replace('<', "\\u003C")
        .replace('\u{2028}', "\\u2028")
        .replace('\u{2029}', "\\u2029");
    let html = TEMPLATE.replace("__SCENE_JSON__", &scene_json);

    let mut out = serde_json::Map::new();
    out.insert("ok".into(), Value::Bool(true));
    out.insert("html".into(), Value::String(html.clone()));
    out.insert("bytes".into(), Value::from(html.len() as u64));
    out.insert("emitted".into(), Value::Array(receipt.emitted));
    out.insert("failed".into(), Value::Array(Vec::new()));
    out.insert("unsupported".into(), Value::Array(receipt.unsupported));
    out.insert("warnings".into(), Value::Array(receipt.warnings));

    if let Some(path) = args
        .get("output-path")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
    {
        // Real run only: a preview (--dry-run / --simulate) returns the HTML and the would-be
        // path but never touches disk (same contract as html-report / ui).
        if !dry_run {
            if let Some(parent) = std::path::Path::new(path).parent()
                && !parent.as_os_str().is_empty()
            {
                std::fs::create_dir_all(parent).map_err(|e| {
                    AwareError::Internal(format!("viewer-3d: create {}: {e}", parent.display()))
                })?;
            }
            std::fs::write(path, html.as_bytes())
                .map_err(|e| AwareError::Internal(format!("viewer-3d: write {path}: {e}")))?;
        }
        out.insert("output-path".into(), Value::String(path.to_string()));
        out.insert("path".into(), Value::String(path.to_string()));
    }

    Ok(Value::Object(out))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn rolled_member(up: &str, rot: Value) -> Value {
        json!({
            "meta": { "name": "roll", "units": "mm", "up": up },
            "elements": [{
                "id": "M", "kind": "member", "from": [0,0,0], "to": [0,0,1000],
                "rot": rot, "section": { "w": 100, "d": 200 },
                "xsection": { "shape": "angle", "d": 200, "b": 100, "t": 10 }
            }]
        })
    }

    #[test]
    fn validates_scene_up_and_member_roll_without_coercion() {
        for up in ["z", "y"] {
            viewer_3d_render(&json!({ "scene": rolled_member(up, json!(82.7)) }), true).unwrap();
        }
        for invalid in [Value::Null, json!("82.7"), json!(true)] {
            let error = viewer_3d_render(&json!({ "scene": rolled_member("z", invalid) }), true)
                .unwrap_err()
                .to_string();
            assert!(error.contains("finite JSON number"), "{error}");
        }
        for invalid_up in [json!("Z"), json!("x"), json!(null), json!(42)] {
            let mut scene = rolled_member("z", json!(0));
            scene["meta"]["up"] = invalid_up;
            assert!(viewer_3d_render(&json!({ "scene": scene }), true).is_err());
        }
        let mut plate = rolled_member("z", json!(0))["elements"][0].clone();
        plate["kind"] = json!("node");
        plate["at"] = json!([0, 0, 0]);
        let error = viewer_3d_render(
            &json!({ "scene": { "meta": { "up": "z" }, "elements": [plate] } }),
            true,
        )
        .unwrap_err()
        .to_string();
        assert!(error.contains("applicable only"), "{error}");
    }

    #[test]
    fn rejects_present_malformed_exact_cross_sections_before_rendering() {
        let malformed = [
            json!({ "shape": "chs", "od": 180 }),
            json!({ "shape": "chs", "od": 180, "t": 90 }),
            json!({ "shape": "i", "d": 200, "bf": 100, "tw": 100, "tf": 10 }),
            json!({ "shape": "angle", "d": 200, "b": 100, "t": 100 }),
            json!({ "shape": "rhs", "d": 200, "b": 100, "t": 50 }),
            json!({ "shape": "rect", "w": 100, "d": 0 }),
            json!({ "shape": "unknown", "w": 100, "d": 100 }),
        ];
        for xsection in malformed {
            let mut scene = rolled_member("z", json!(0));
            scene["elements"][0]["xsection"] = xsection;
            let error = viewer_3d_render(&json!({ "scene": scene }), true)
                .unwrap_err()
                .to_string();
            assert!(error.contains("xsection"), "{error}");
        }
    }

    #[test]
    fn renders_tee_and_both_double_angle_orientations_from_exact_envelopes() {
        for xsection in [
            json!({"shape":"tee","d":300,"bf":200,"tw":10,"tf":20}),
            json!({"shape":"double-angle","d":150,"b":100,"t":10,"gap":12,"orientation":"llbb"}),
            json!({"shape":"double-angle","d":150,"b":100,"t":10,"gap":12,"orientation":"slbb"}),
        ] {
            let (w, d) = match xsection["orientation"].as_str() {
                Some("llbb") => (212, 150),
                Some("slbb") => (312, 100),
                _ => (200, 300),
            };
            let scene = json!({ "meta": { "units": "mm" }, "elements": [{
                "id":"M", "kind":"member", "from":[0,0,0], "to":[1000,0,0],
                "section":{"w":w,"d":d}, "xsection":xsection
            }]});
            let output = viewer_3d_render(&json!({ "scene": scene }), true).unwrap();
            let html = output["html"].as_str().unwrap();
            assert!(html.contains("shape==='tee'"));
            assert!(html.contains("DA_LLBB"));
        }
    }

    #[test]
    fn rejects_double_angle_with_an_incorrect_envelope_or_orientation() {
        let mut scene = rolled_member("z", json!(0));
        scene["elements"][0]["xsection"] = json!({
            "shape":"double-angle","d":150,"b":100,"t":10,"gap":12,"orientation":"llbb"
        });
        scene["elements"][0]["section"] = json!({"w":200,"d":150});
        assert!(viewer_3d_render(&json!({ "scene": scene }), true).is_err());
    }

    #[test]
    fn double_angle_gap_is_exact_and_zero_means_touching_without_overlap() {
        for (orientation, w, d) in [("llbb", 212, 150), ("slbb", 312, 100)] {
            let scene = json!({ "meta": { "units": "mm" }, "elements": [{
                "id":"M", "kind":"member", "from":[0,0,0], "to":[1000,0,0],
                "section":{"w":w,"d":d}, "xsection":{
                    "shape":"double-angle","d":150,"b":100,"t":10,"gap":12,"orientation":orientation
                }
            }]});
            let rendered = viewer_3d_render(&json!({ "scene": scene }), true).unwrap();
            let html = rendered["html"].as_str().unwrap();
            assert!(html.contains("kind==='DA_LLBB'||kind==='DA_SLBB'"));
            assert!(html.contains("a.moveTo(-g/2-t,-hd)"));
            assert!(html.contains("b.moveTo(g/2,-hd)"));
            assert!(!html.contains("else if(kind==='DA_SLBB')"));
        }

        let touching = json!({ "meta": { "units": "mm" }, "elements": [{
            "id":"M", "kind":"member", "from":[0,0,0], "to":[1000,0,0],
            "section":{"w":300,"d":100}, "xsection":{
                "shape":"double-angle","d":150,"b":100,"t":10,"gap":0,"orientation":"slbb"
            }
        }]});
        assert!(viewer_3d_render(&json!({ "scene": touching }), true).is_ok());

        let mut overlap = touching;
        overlap["elements"][0]["xsection"]["gap"] = json!(-1);
        overlap["elements"][0]["section"]["w"] = json!(299);
        assert!(viewer_3d_render(&json!({ "scene": overlap }), true).is_err());
    }

    #[test]
    fn unknown_rotated_element_stays_forward_compatible_and_is_receipted() {
        let scene = json!({
            "meta": { "name": "Future element", "units": "mm", "up": "z" },
            "elements": [{ "id": "FUTURE-1", "kind": "future-native-part", "rot": 82.7 }]
        });
        let output = viewer_3d_render(&json!({ "scene": scene }), true).unwrap();
        assert_eq!(output["unsupported"][0]["id"], "FUTURE-1");
        assert_eq!(output["unsupported"][0]["code"], "unsupported-element-kind");
    }

    #[test]
    fn ships_exact_profiles_roll_frames_and_world_vertex_probes() {
        let output =
            viewer_3d_render(&json!({ "scene": rolled_member("z", json!(82.7)) }), true).unwrap();
        let html = output["html"].as_str().unwrap();
        assert!(html.contains("shape==='angle'"));
        assert!(html.contains("function memberFrame(e,up)"));
        assert!(html.contains("memberFrame:(id)=>"));
        assert!(html.contains("memberVertices:(id)=>"));
    }

    #[test]
    fn the_viewer_selects_the_zero_frame_branch_the_export_sinks_do() {
        // The viewer carries its OWN copy of the member-roll contract, so it can drift from
        // `scene_roll::member_frame` while every Rust test stays green — it did, and Codex
        // caught it on #435. A cancelling `1-du*du` seeds members that `member_frame`, and
        // therefore the IFC and Tekla sinks, project; the two branches disagree about a
        // section's facing by up to 180°, so such a member is DRAWN turned around from how
        // it exports. On the axis pinned by `scene_roll`'s
        // `the_branch_test_does_not_cancel_at_the_threshold` the two differ by 78.5°.
        //
        // A string assertion because nothing else runs this JS in CI: the browser gate
        // (`cli/tests/browser/run.mjs`) is a manual pre-PR step needing Playwright and a CDN.
        // It is the same shape the surrounding template tests already use. Issue #432.
        let output =
            viewer_3d_render(&json!({ "scene": rolled_member("z", json!(82.7)) }), true).unwrap();
        let html = output["html"].as_str().unwrap();
        assert!(
            html.contains(
                "seeded=d.clone().addScaledVector(U,-d.dot(U)).lengthSq()<=1e-6*d.lengthSq()"
            ),
            "viewer memberFrame must branch on the RAW delta as a sum-of-squares ratio"
        );
        assert!(
            !html.contains("1-du*du"),
            "viewer memberFrame must not select the branch on a cancelling `1-du*du`"
        );
        // Normalizing before the branch test is the other way the viewer can disagree with the
        // sinks: THREE's `normalize()` multiplies by the reciprocal length where Rust divides,
        // and an ulp there straddles the threshold. The branch must read `d`, never `n`.
        assert!(
            !html.contains("perp=n.clone()") && !html.contains("n.clone().addScaledVector(U,-du)"),
            "viewer memberFrame must not derive the branch from the normalized axis"
        );
    }

    #[test]
    fn renders_scene_into_self_contained_html() {
        let scene = json!({
            "meta": { "name": "Test frame", "units": "mm", "up": "z" },
            "groups": [ { "key": "column", "label": "Columns", "color": "#60a5fa" } ],
            "elements": [
                { "id": "C1", "group": "column", "kind": "box",
                  "from": [0,0,0], "to": [0,0,3000], "section": { "w": 300, "d": 300 },
                  "meta": { "profile": "UC 305x305x97" } }
            ]
        });
        let out = viewer_3d_render(&json!({ "scene": scene.clone() }), true).unwrap();
        let html = out["html"].as_str().unwrap();
        assert!(html.starts_with("<!doctype html>"));
        assert!(html.contains("import * as THREE from 'three'"));
        assert!(html.contains("\"C1\"")); // the scene was injected
        assert!(html.contains("Test frame"));
        assert!(!html.contains("__SCENE_JSON__")); // placeholder fully substituted
        assert!(out["bytes"].as_u64().unwrap() > 0);
        assert!(out.get("output-path").is_none()); // none given → not written
    }

    #[test]
    fn ships_realistic_material_mode() {
        // The "realistic" display mode shades each element from its semantic `meta.material`.
        // Metal is nothing but reflections, so the mode is only truthful if it ALSO ships an
        // environment to reflect — without one, metalness=1 renders black. Assert the whole
        // chain is wired, not just the button.
        let scene = json!({
            "meta": { "name": "Mixed", "units": "mm", "up": "z" },
            "groups": [ { "key": "W16X26", "label": "Beams", "color": "#60a5fa" } ],
            "elements": [
                // Element-level `material` is the canonical field — the same one the IFC writer
                // resolves. A scene authored for export must shade correctly with no change.
                { "id": "B1", "group": "W16X26", "kind": "box",
                  "from": [0,0,0], "to": [3000,0,0], "section": { "w": 140, "d": 400 },
                  "material": "A992", "meta": { "profile": "W16X26" } }
            ]
        });
        let out = viewer_3d_render(&json!({ "scene": scene }), true).unwrap();
        let html = out["html"].as_str().unwrap();
        // Whitespace-stripped copy so the assertions below survive re-alignment of the source.
        let compact: String = html.chars().filter(|c| !c.is_whitespace()).collect();

        // Resolution order: the canonical element field FIRST, meta only as a fallback. Reading
        // meta alone would silently drop every canonical scene to the `painted` default.
        assert!(
            compact.contains("constraw=pick(e&&e.material)||pick(e&&e.meta&&e.meta.material)"),
            "element-level `material` wins; `meta.material` is only a fallback"
        );
        // Each candidate is trimmed BEFORE the choice, matching the IFC writer's treatment of a
        // trimmed-empty material as absent — otherwise `material: "   "` (truthy) would win and
        // then trim to nothing, silently skipping a perfectly good `meta.material`.
        assert!(
            compact.contains("constpick=v=>(v==null?'':String(v)).trim()"),
            "candidates are trimmed before the fallback decision"
        );

        // The mode itself, and the material carried through into the document.
        assert!(
            html.contains("data-mode=\"realistic\""),
            "realistic mode button"
        );
        assert!(
            html.contains("\"material\""),
            "meta.material rides into the scene"
        );

        // The generated image-based light + filmic tone mapping. RoomEnvironment/PMREM are
        // generated in-browser from fixed code, so no external asset is fetched.
        assert!(
            html.contains("RoomEnvironment") && html.contains("PMREMGenerator"),
            "realistic mode generates an environment to reflect"
        );
        assert!(
            html.contains("ACESFilmicToneMapping") && html.contains("function setEnvironment"),
            "filmic tone mapping is toggled with the mode"
        );

        // Directional finishes must project from OBJECT space so the pattern runs along the member.
        // The floless editor shipped this the other way once: `local` was threaded into the program
        // cache key but never into the shader, so timber grain and brushed metal stayed pinned to
        // the world axes and ran ACROSS sloped members. Nothing errored — it just looked wrong — so
        // assert the branch exists rather than trusting that the parameter is honoured.
        assert!(
            compact.contains("constDIRECTIONAL=newSet(['timber','stainless','aluminium'])"),
            "directional families are declared"
        );
        assert!(
            compact.contains("applyTriplanar(mat,TILE_MM[fam],DIRECTIONAL.has(fam)?lengthAxis(e,kind,doubleSided):null)"),
            "the local flag is actually passed per family"
        );
        assert!(
            compact.contains("constswizzle=axis==='y'?'.xzy':''"),
            "a length-on-Y geometry is permuted so the grain follows the member, not local Z"
        );
        assert!(
            compact.contains("constposExpr=axis?('transformed'+swizzle)")
                && compact.contains("constnrmExpr=axis?('objectNormal'+swizzle)"),
            "the axis reaches the shader (position AND normal) instead of only the cache key"
        );
        // The permutation is only correct if it is keyed off the geometry each element actually
        // gets — rods remain Y-axial, while every member profile now extrudes on local Z.
        assert!(
            compact.contains("if(kind==='rod'||kind==='bolt-shank')return'y'")
                && compact.contains("return'z';//everymemberprofilenowextrudesonlocalZ"),
            "length axis is derived per geometry kind"
        );

        // The per-family appearance table + the grade→family fallback chain.
        assert!(html.contains("const MATERIALS="), "material table");
        // Match the table ENTRY, not the bare word — the family names also occur in prose, so a
        // substring check would pass against a comment while the table itself was empty.
        for family in [
            "painted",
            "steel",
            "galvanised",
            "stainless",
            "weathering",
            "aluminium",
            "concrete",
            "timber",
            "asphalt",
            "glass",
        ] {
            assert!(
                compact.contains(&format!("{family}:{{metalness:")),
                "material family `{family}` has an entry in the table"
            );
        }
        assert!(
            html.contains("const GRADE_FAMILIES=") && html.contains("function familyOf"),
            "a grade names the alloy, so it needs mapping to an appearance family"
        );
        assert!(
            compact.contains("mat.userData={baseOpacity:op,baseColor:col,family:fam}"),
            "the base colour is retained so leaving Realistic restores the group colour"
        );
        // Surface detail: generated procedurally (never fetched) and projected from world space,
        // because an extruded member's UVs are millimetre-scale and inconsistent across its faces.
        assert!(
            html.contains("function applyTriplanar") && html.contains("vec4 triplanar(sampler2D"),
            "maps are projected triplanarly, not sampled through the mesh UVs"
        );
        assert!(
            html.contains("function surfaceFor") && html.contains("const SURFACES="),
            "surface detail is generated in-browser, so it costs the document no bytes"
        );
        assert!(
            // The CALL form, not the bare name — the comment explaining the rule ships in the
            // document too, and would match a looser check.
            !html.contains("Math.random(") && html.contains("function mulberry32"),
            "generated surfaces must be seeded, or the document stops being reproducible"
        );
        // Metal has no diffuse term, so an unlifted metalness=1 surface reads darker than the matte
        // paint beside it. Must be per-material: scene.environmentIntensity is three r163+, and the
        // pinned CDN version here is r160.
        // Per-material on purpose: scene.environmentIntensity is three r163+ and the CDN version is
        // pinned to r160. Asserted positively only — a "must not contain" check here matches the
        // comment that explains the rule, which also ships in the document.
        assert!(
            html.contains("mat.envMapIntensity"),
            "metals are lifted per-material so they don't read darker than the paint beside them"
        );
        // Shadows are opt-in and OFF by default — they cost real frame time on a large frame, and
        // Realistic reads well enough without them. Asserted because a default flipped back to true
        // would be an easy, silent regression to make and a hard one to notice.
        // Shadows are a VARIANT of Realistic (`shadowed`), not an orthogonal toggle: they need its
        // environment and solid surfaces to mean anything. Plain `realistic` stays the cheap default,
        // which is an easy thing to regress silently and a hard one to notice.
        assert!(
            html.contains("data-mode=\"shadowed\""),
            "shadows are their own display mode, not a cross-cutting toggle"
        );
        assert!(
            compact.contains("renderer.shadowMap.enabled=false"),
            "shadow mapping starts disabled, so the default mode costs nothing rather than merely hiding shadows"
        );
        assert!(
            compact.contains("syncShadows(realistic&&displayMode==='shadowed')"),
            "only the shadowed mode casts, and only once the environment is actually live"
        );
        // Regression guard: group colours arrive as CSS strings ("#60a5fa") but the family
        // colours are numeric literals. `setHex` accepts only a number, so restoring a group
        // colour through it yields NaN and renders the element BLACK — which is what shipped
        // in the first cut of this mode and was caught only by driving it in a real browser.
        assert!(
            !html.contains("mat.color.setHex("),
            "restore the base colour with .set (string-or-number), never .setHex"
        );
    }

    #[test]
    fn renders_identical_bytes_for_identical_scene() {
        // The determinism guarantee in this module's header (no clock, no environment) is what
        // lets a rendered document be cached, diffed and hosted. Nothing asserted it until now.
        let scene = json!({
            "meta": { "name": "Determinism", "units": "mm", "up": "z" },
            "groups": [ { "key": "g", "label": "G", "color": "#60a5fa" } ],
            "elements": [
                { "id": "B1", "group": "g", "kind": "box",
                  "from": [0,0,0], "to": [3000,0,0], "section": { "w": 140, "d": 400 },
                  "meta": { "profile": "W16X26", "material": "A992" } }
            ]
        });
        let first = viewer_3d_render(&json!({ "scene": scene.clone() }), true).unwrap();
        let second = viewer_3d_render(&json!({ "scene": scene }), true).unwrap();
        assert_eq!(
            first["html"].as_str().unwrap(),
            second["html"].as_str().unwrap(),
            "identical scene input must produce identical HTML bytes"
        );
        assert_eq!(first["bytes"], second["bytes"]);
    }

    #[test]
    fn renders_a_tessellated_mesh_element() {
        // A kind:"mesh" element (imported connection geometry) rides through as positions+indices,
        // and the renderer ships the BufferGeometry path that draws it.
        let scene = json!({
            "meta": { "name": "Conn", "units": "mm", "up": "z" },
            "elements": [
                { "id": "PL-1", "kind": "mesh", "group": "connection",
                  "positions": [0,0,0, 100,0,0, 100,100,0, 0,100,0],
                  "indices": [0,1,2, 0,2,3] }
            ]
        });
        let out = viewer_3d_render(&json!({ "scene": scene }), true).unwrap();
        let html = out["html"].as_str().unwrap();
        assert!(html.contains("\"PL-1\"")); // the mesh element was injected
        assert!(html.contains("\"positions\"")); // its tessellation rode through
        assert!(html.contains("BufferGeometry")); // the renderer ships mesh support
        assert!(html.contains("setIndex"));
    }

    #[test]
    fn renders_parametric_connection_solids_and_structural_references_with_receipts() {
        let scene = json!({
            "meta": { "name": "Connection and grid", "units": "mm", "up": "z" },
            "elements": [
                { "id": "BEAM-1", "kind": "member", "from": [-1000,0,1000], "to": [1000,0,1000] },
                { "id": "PL-1", "kind": "plate", "frame": {
                    "origin": [0,0,1000], "uDir": [1,0,0], "vDir": [0,1,0], "normal": [0,0,1]
                  }, "outline": [[-200,-150],[200,-150],[200,150],[-200,150]], "thicknessMm": 20,
                  "holes": [{"id":"H-1","center":[0,0],"diameterMm":24}] },
                { "id": "ROD-1", "kind": "rod", "axis": {"from":[0,0,0],"to":[0,0,500]}, "diameterMm": 20 },
                { "id": "SHANK-1", "kind": "bolt-shank", "axis": {"from":[500,0,900],"to":[500,0,1100]}, "diameterMm": 24 },
                { "id": "W-1", "kind": "washer", "center":[500,0,910], "axis":[0,0,1],
                  "outerDiameterMm":44,"innerDiameterMm":22,"thicknessMm":4 },
                { "id": "N-1", "kind": "nut", "center":[500,0,930], "axis":[0,0,1],
                  "acrossFlatsMm":32,"thicknessMm":18,"phaseRad":0 },
                { "id": "HEAD-1", "kind": "bolt-head", "center":[500,0,890], "axis":[0,0,1],
                  "acrossFlatsMm":36,"thicknessMm":14,"phaseRad":0.25 }
            ],
            "operations": [
                {"id":"BA-1","kind":"bolt-array","partToBoltTo":"PL-1","partToBeBolted":"BEAM-1",
                 "frame":{"origin":[500,0,1000],"uDir":[1,0,0],"vDir":[0,1,0],"normal":[0,0,1]},
                 "uOffsetsMm":[0],"vOffsetsMm":[0],"diameterMm":24,"standard":"A325N","toleranceMm":2,
                 "boltType":"shop","components":{},
                 "instances":[{"id":"BI-1","point":[500,0,1000],"shankId":"SHANK-1","headId":"HEAD-1",
                   "nutIds":["N-1"],"washerIds":["W-1"],"holeEffects":[
                     {"id":"HE-1","targetId":"PL-1","center":[500,0,1000],"axis":[0,0,1],"diameterMm":26},
                     {"id":"HE-2","targetId":"BEAM-1","center":[500,0,1000],"axis":[0,0,-1],"diameterMm":26}
                   ]}]},
                {"id":"WELD-1","kind":"weld","mainId":"PL-1","secondaryId":"BEAM-1",
                 "path":[[-200,0,1000],[200,0,1000]],"weldType":"fillet","sizeMm":6,"around":false,"shop":true},
                {"id":"CUT-1","kind":"boolean-cut","targetId":"PL-1",
                 "tool":{"kind":"cylinder","axis":{"from":[0,0,980],"to":[0,0,1020]},"diameterMm":16}}
            ],
            "referenceSystems": [{"id":"GRID-1","kind":"structural-grid","origin":[0,0,0],
                "bounds":{"minX":-1000,"maxX":3000,"minY":-2000,"maxY":2000},
                "axes":[{"id":"GA-X","direction":"x","offsetMm":0,"label":"1"},{"id":"GA-Y","direction":"y","offsetMm":0,"label":"A"}],
                "levels":[{"id":"GL-1","elevationMm":3000,"label":"L1"}] }]
        });
        let out = viewer_3d_render(&json!({ "scene": scene }), true).unwrap();
        let html = out["html"].as_str().unwrap();
        assert!(html.contains("function plateMesh"));
        assert!(
            html.contains("const n=u.clone().cross(v).normalize()"),
            "framed geometry must use a right-handed basis after Z-up conversion"
        );
        assert!(
            html.contains("shape.holes.push(path)"),
            "plate voids use exact Shape holes"
        );
        assert!(html.contains("function annulusMesh"));
        assert!(html.contains("e.acrossFlatsMm/Math.sqrt(3)"));
        assert!(
            html.contains("const sourceU=sourceSeed.cross(sourceN).normalize()"),
            "oriented profiles must transform the deterministic source basis"
        );
        assert!(
            html.contains("phaseRad||0)*(up==='z'?-1:1)"),
            "hex phase must compensate for the reflective Z-up conversion"
        );
        assert!(html.contains("function addOperations"));
        assert!(html.contains("const weld=new THREE.Line(geometry,material)"));
        assert!(
            html.contains("op.kind==='weld'&&Array.isArray(op.path)"),
            "weld paths must contribute to scene bounds as well as rendering"
        );
        assert!(html.contains("function addReferenceSystems"));
        // A structural grid is authored in PLAN space (x/y plan axes + an absolute elevation) and has
        // its own frame, independent of `meta.up`. Routing that through conv(P,up) was a real defect:
        // on a `meta.up:'y'` scene conv is the identity, so the elevation landed in world Z and every
        // LEVEL rendered as a vertical plane. This assertion replaces one that pinned the buggy line
        // ("level elevation is world Z") — in the rendered world, which is always y-up, it is world Y.
        assert!(html.contains("function referenceSystemSegments"));
        assert!(
            html.contains("const gridToWorld=(gx,gy,elev)=>new THREE.Vector3(gx,elev,gy);"),
            "grid elevation maps to world Y, unconditionally — not through conv(P,up)"
        );
        assert!(
            !html.contains("makeLabel(l.label,conv(") && !html.contains("conv(A,up),conv(B,up)"),
            "grid geometry must never be conv()'d again — that is what broke y-up scenes"
        );

        let emitted = out["emitted"].as_array().unwrap();
        for id in [
            "PL-1", "H-1", "ROD-1", "SHANK-1", "W-1", "N-1", "HEAD-1", "BA-1", "BI-1", "HE-1",
            "HE-2", "WELD-1", "GRID-1", "GA-X", "GA-Y", "GL-1",
        ] {
            assert!(
                emitted.iter().any(|row| row["id"] == id),
                "missing receipt for {id}"
            );
        }
        assert!(out["failed"].as_array().unwrap().is_empty());
        assert!(out["warnings"].as_array().unwrap().is_empty());
        assert_eq!(out["unsupported"][0]["id"], "CUT-1");
        assert_eq!(out["unsupported"][0]["code"], "exact-csg-not-available");
        assert_eq!(out["unsupported"][0]["status"], "unsupported");
        assert!(emitted.iter().all(|row| row["status"] == "emitted"));
        assert!(
            emitted
                .iter()
                .filter(|row| row["id"] == "BA-1")
                .all(|row| row["geometryDuplicated"] == false)
        );

        let mut cut_fastener = scene.clone();
        cut_fastener["operations"][2]["targetId"] = json!("SHANK-1");
        let cut_output = viewer_3d_render(&json!({ "scene": cut_fastener }), true).unwrap();
        assert_eq!(cut_output["unsupported"][0]["id"], "CUT-1");

        let mut reused = scene.clone();
        reused["operations"][0]["uOffsetsMm"] = json!([0, 100]);
        let mut second = reused["operations"][0]["instances"][0].clone();
        second["id"] = json!("BI-2");
        second["point"] = json!([600, 0, 1000]);
        reused["operations"][0]["instances"]
            .as_array_mut()
            .unwrap()
            .push(second);
        let error = viewer_3d_render(&json!({ "scene": reused }), true).unwrap_err();
        assert!(
            error
                .to_string()
                .contains("must not reuse a component already claimed")
        );

        let mut reused_across_arrays = scene.clone();
        let mut second_array = reused_across_arrays["operations"][0].clone();
        second_array["id"] = json!("BA-2");
        second_array["instances"][0]["id"] = json!("BI-2");
        second_array["instances"][0]["holeEffects"][0]["id"] = json!("HE-3");
        second_array["instances"][0]["holeEffects"][1]["id"] = json!("HE-4");
        reused_across_arrays["operations"]
            .as_array_mut()
            .unwrap()
            .push(second_array);
        let error = viewer_3d_render(&json!({ "scene": reused_across_arrays }), true).unwrap_err();
        assert!(
            error
                .to_string()
                .contains("must not reuse a component already claimed")
        );

        let mut no_standard = scene.clone();
        no_standard["operations"][0]
            .as_object_mut()
            .unwrap()
            .remove("standard");
        let error = viewer_3d_render(&json!({ "scene": no_standard }), true).unwrap_err();
        assert!(error.to_string().contains("standard"));

        let mut disabled = scene.clone();
        disabled["operations"][0]["components"]["bolt"] = json!(false);
        let error = viewer_3d_render(&json!({ "scene": disabled }), true).unwrap_err();
        assert!(error.to_string().contains("components.bolt"));

        let mut no_grid_label = scene.clone();
        no_grid_label["referenceSystems"][0]["axes"][0]
            .as_object_mut()
            .unwrap()
            .remove("label");
        let error = viewer_3d_render(&json!({ "scene": no_grid_label }), true).unwrap_err();
        assert!(error.to_string().contains("axes[0].label"));

        let mut empty_levels = scene.clone();
        empty_levels["referenceSystems"][0]["levels"] = json!([]);
        let error = viewer_3d_render(&json!({ "scene": empty_levels }), true).unwrap_err();
        assert!(error.to_string().contains("at least one elevation datum"));

        let mut non_boolean = scene;
        non_boolean["operations"][0]["components"]["washer"] = json!("yes");
        let error = viewer_3d_render(&json!({ "scene": non_boolean }), true).unwrap_err();
        assert!(error.to_string().contains("components.washer"));
    }

    /// A double-sided gusset: plate + member + plate on ONE bolt, i.e. double shear. Three plies,
    /// three holes — the commonest multi-ply detail, and the shape the old two-effect cap refused.
    /// Kept in step with `render::ifc`'s `double_shear_scene`.
    fn viewer_double_shear_scene() -> Value {
        let mut scene = json!({
            "meta": { "name": "Double shear", "units": "mm", "up": "z" },
            "elements": [
                {"id":"A","kind":"plate","frame":{"origin":[0,0,10],"uDir":[1,0,0],"vDir":[0,1,0],"normal":[0,0,1]},
                 "outline":[[-40,-40],[40,-40],[40,40],[-40,40]],"thicknessMm":10},
                {"id":"B","kind":"member","from":[-200,0,0],"to":[200,0,0]},
                {"id":"C","kind":"plate","frame":{"origin":[0,0,-10],"uDir":[1,0,0],"vDir":[0,1,0],"normal":[0,0,1]},
                 "outline":[[-40,-40],[40,-40],[40,40],[-40,40]],"thicknessMm":10},
                {"id":"S","kind":"bolt-shank","axis":{"from":[0,0,-30],"to":[0,0,30]},"diameterMm":20},
                {"id":"H","kind":"bolt-head","center":[0,0,34],"axis":[0,0,1],"acrossFlatsMm":30,"thicknessMm":8}
            ],
            "operations":[{
                "id":"BA","kind":"bolt-array","partToBoltTo":"A","partToBeBolted":"B",
                "frame":{"origin":[0,0,0],"uDir":[1,0,0],"vDir":[0,1,0],"normal":[0,0,1]},
                "uOffsetsMm":[0],"vOffsetsMm":[0],"diameterMm":20,"standard":"A325N","toleranceMm":2,
                "boltType":"shop","components":{"bolt":true,"nut":false,"washer":false},
                "instances":[{"id":"I","point":[0,0,0],"shankId":"S","headId":"H",
                  "nutIds":[],"washerIds":[],"holeEffects":[
                    {"id":"HE-A","targetId":"A","center":[0,0,0],"axis":[0,0,1],"diameterMm":22},
                    {"id":"HE-B","targetId":"B","center":[0,0,0],"axis":[0,0,1],"diameterMm":22}
                  ]}]
            }]
        });
        scene["operations"][0]["instances"][0]["holeEffects"]
            .as_array_mut()
            .unwrap()
            .push(
                json!({"id":"HE-C","targetId":"C","center":[0,0,0],"axis":[0,0,1],"diameterMm":22}),
            );
        scene
    }

    /// A bolt-array frame whose `uDir` is finite but enormous used to abort the
    /// process, not fail validation.
    ///
    /// `finite_number` accepts `1e200`, so `vector::<3>` hands it through. But
    /// `normalized3` divides by `length3`, and `1e200 * 1e200` overflows to
    /// `inf`. A degenerate frame is invalid input, so it must come back as a
    /// validation error the caller can report rather than a panic.
    ///
    /// The *message* changed when the two copies of `normalized3` were unified
    /// into `render::geom`: this one lacked the `is_finite` check its `ifc` twin
    /// had, so it answered `Some([0,0,0])` — a "unit" vector of length zero —
    /// and the frame only failed later, on orthonormality, by luck. The shared
    /// version rejects the non-finite length up front, which is both earlier and
    /// truthful about what is wrong with the input.
    #[test]
    fn degenerate_bolt_frame_is_rejected_not_panicked() {
        let mut scene = viewer_double_shear_scene();
        scene["operations"][0]["frame"]["uDir"] = json!([1e200, 0, 0]);
        let error = viewer_3d_render(&json!({ "scene": scene }), true).unwrap_err();
        assert!(
            error
                .to_string()
                .contains("directions must be nonzero and finite"),
            "expected a degenerate-frame validation error, got: {error}"
        );
    }

    #[test]
    fn viewer_accepts_a_bolt_holing_more_plies_than_the_declared_pair() {
        let scene = viewer_double_shear_scene();
        let out = viewer_3d_render(&json!({ "scene": scene }), true).unwrap();
        let emitted = out["emitted"].as_array().unwrap();
        // The far plate's hole must reach the receipt, not merely survive validation.
        assert!(
            emitted.iter().any(|row| row["id"] == "HE-C"),
            "the third ply's hole effect is emitted"
        );
        assert!(out["failed"].as_array().unwrap().is_empty());
    }

    #[test]
    fn viewer_holds_an_extra_ply_to_the_same_bar_as_the_pair() {
        // An unknown id would validate and then be dropped downstream — a ply fabricated with no hole.
        let mut unknown = viewer_double_shear_scene();
        unknown["operations"][0]["instances"][0]["holeEffects"][2]["targetId"] = json!("NOPE");
        let error = viewer_3d_render(&json!({ "scene": unknown }), true).unwrap_err();
        assert!(
            error
                .to_string()
                .contains("another element the bolt passes through"),
            "got: {error}"
        );

        // A bolt does not drill itself.
        let mut component = viewer_double_shear_scene();
        component["operations"][0]["instances"][0]["holeEffects"][2]["targetId"] = json!("S");
        let error = viewer_3d_render(&json!({ "scene": component }), true).unwrap_err();
        assert!(
            error
                .to_string()
                .contains("another element the bolt passes through"),
            "got: {error}"
        );

        // The same ply twice is not two shear planes.
        let mut twice = viewer_double_shear_scene();
        twice["operations"][0]["instances"][0]["holeEffects"][2]["targetId"] = json!("A");
        let error = viewer_3d_render(&json!({ "scene": twice }), true).unwrap_err();
        assert!(
            error
                .to_string()
                .contains("must not hole the same ply twice"),
            "got: {error}"
        );
    }

    #[test]
    fn viewer_extra_plies_do_not_excuse_a_missing_declared_participant() {
        // Two plates and no member: three effects would satisfy any count test, while the bolt misses
        // the very member the array declares it is bolting.
        let mut scene = viewer_double_shear_scene();
        scene["operations"][0]["instances"][0]["holeEffects"]
            .as_array_mut()
            .unwrap()
            .remove(1);
        let error = viewer_3d_render(&json!({ "scene": scene }), true).unwrap_err();
        assert!(
            error
                .to_string()
                .contains("must cover both bolt participants"),
            "got: {error}"
        );
    }

    #[test]
    fn bolt_children_must_match_their_viewer_instance_geometry() {
        let mut scene = json!({
            "elements": [
                {"id":"A","kind":"plate","frame":{"origin":[0,0,0],"uDir":[1,0,0],"vDir":[0,1,0],"normal":[0,0,1]},
                 "outline":[[-10,-10],[10,-10],[10,10],[-10,10]],"thicknessMm":5},
                {"id":"B","kind":"member","from":[-100,0,0],"to":[100,0,0]},
                {"id":"S","kind":"bolt-shank","axis":{"from":[50,0,-10],"to":[50,0,10]},"diameterMm":20},
                {"id":"H","kind":"bolt-head","center":[0,0,12],"axis":[0,0,1],"acrossFlatsMm":30,"thicknessMm":8}
            ],
            "operations":[{
                "id":"BA","kind":"bolt-array","partToBoltTo":"A","partToBeBolted":"B",
                "frame":{"origin":[0,0,0],"uDir":[1,0,0],"vDir":[0,1,0],"normal":[0,0,1]},
                "uOffsetsMm":[0],"vOffsetsMm":[0],"diameterMm":20,"standard":"A325N","toleranceMm":2,
                "boltType":"shop","components":{"bolt":true,"nut":false,"washer":false},
                "instances":[{"id":"I","point":[0,0,0],"shankId":"S","headId":"H",
                    "nutIds":[],"washerIds":[],"holeEffects":[
                        {"id":"HA","targetId":"A","center":[0,0,0],"axis":[0,0,1],"diameterMm":22},
                        {"id":"HB","targetId":"B","center":[0,0,0],"axis":[0,0,1],"diameterMm":22}
                    ]}]
            }]
        });

        let error = viewer_3d_render(&json!({ "scene": scene.clone() }), true).unwrap_err();
        assert!(error.to_string().contains("shank axis and diameter"));

        scene["elements"][2]["axis"]["from"] = json!([0, 0, -10]);
        scene["elements"][2]["axis"]["to"] = json!([0, 0, 10]);
        scene["elements"][3]["center"] = json!([50, 0, 12]);
        let error = viewer_3d_render(&json!({ "scene": scene }), true).unwrap_err();
        assert!(error.to_string().contains("centered and aligned"));
    }

    #[test]
    fn invalid_supported_geometry_fails_before_html_is_produced() {
        let scene = json!({
            "elements": [{ "id": "PL-1", "kind": "plate", "frame": {
                "origin": [0,0,0], "uDir": [1,0,0], "vDir": [0,1,0], "normal": [0,0,1]
              }, "outline": [[-10,-10],[10,-10],[10,10],[-10,10]], "thicknessMm": 5,
              "holes": [{"id":"H-1","center":[9,0],"diameterMm":4}] }]
        });
        let error = viewer_3d_render(&json!({ "scene": scene }), true).unwrap_err();
        assert!(matches!(error, AwareError::Validation(_)));
        assert!(error.to_string().contains("must lie wholly inside"));

        let bowtie = json!({
            "elements": [{ "id": "PL-1", "kind": "plate", "frame": {
                "origin": [0,0,0], "uDir": [1,0,0], "vDir": [0,1,0], "normal": [0,0,1]
              }, "outline": [[-10,-10],[10,10],[-10,10],[10,-10]], "thicknessMm": 5 }]
        });
        let error = viewer_3d_render(&json!({ "scene": bowtie }), true).unwrap_err();
        assert!(error.to_string().contains("nonzero simple polygon"));
    }

    #[test]
    fn explicit_null_scene_collections_are_rejected() {
        for collection in ["elements", "operations", "referenceSystems"] {
            let mut scene = json!({});
            scene[collection] = Value::Null;
            let error = viewer_3d_render(&json!({ "scene": scene }), true).unwrap_err();
            assert!(
                error
                    .to_string()
                    .contains(&format!("`{collection}` must be an array")),
                "{error}"
            );
        }
    }

    #[test]
    fn unknown_records_are_exhaustively_unsupported() {
        let scene = json!({
            "elements": [{"id":"E-X","kind":"future-solid"}],
            "operations": [{"id":"O-X","kind":"future-operation"}],
            "referenceSystems": [{"id":"R-X","kind":"future-reference",
                "axes":[{"id":"RA-X"}], "levels":[{"id":"RL-X"}]}]
        });
        let out = viewer_3d_render(&json!({ "scene": scene }), true).unwrap();
        let unsupported = out["unsupported"].as_array().unwrap();
        assert_eq!(unsupported.len(), 5);
        for id in ["E-X", "O-X", "R-X", "RA-X", "RL-X"] {
            assert!(unsupported.iter().any(|row| row["id"] == id));
        }
        assert!(
            out["html"]
                .as_str()
                .unwrap()
                .contains("if(!['mesh','plate','rod','bolt-shank'"),
            "unsupported records must be skipped by the generated viewer runtime"
        );
        assert_eq!(
            unsupported.iter().find(|row| row["id"] == "RA-X").unwrap()["code"],
            "unsupported-parent"
        );
    }

    #[test]
    fn ships_the_interactive_controls() {
        // #258: the viewer gains a toolbar + interactions. Assert each capability is
        // wired into the rendered document (the renderer is static, so presence of the
        // wiring is the contract a headless browser then exercises).
        let out = viewer_3d_render(
            &json!({ "scene": { "meta": {"name":"x"}, "elements": [] } }),
            true,
        )
        .unwrap();
        let html = out["html"].as_str().unwrap();
        // Orthographic projection toggle (both cameras + the switch).
        assert!(html.contains("OrthographicCamera"), "ortho camera");
        assert!(html.contains("data-proj=\"ortho\"") && html.contains("function setProjection"));
        // Named views live on the ViewCube now; the duplicate toolbar buttons were removed (#258 QA).
        assert!(html.contains("const VIEWS="));
        assert!(
            !html.contains("data-view="),
            "named-view toolbar buttons removed — the ViewCube replaces them"
        );
        // Display modes: solid / wireframe / x-ray / realistic.
        assert!(
            html.contains("data-mode=\"wire\"")
                && html.contains("data-mode=\"xray\"")
                && html.contains("data-mode=\"realistic\""),
            "all four display modes are offered"
        );
        assert!(html.contains("function applyDisplayMode"));
        // Interactive legend (hide / solo) + per-group opacity from the schema.
        assert!(html.contains("function toggleGroup") && html.contains("function soloToggle"));
        assert!(html.contains("opacityOf[e.group]"));
        // CAD navigation: middle-mouse pan + Home/Alt+Z shortcuts.
        assert!(html.contains("MIDDLE:THREE.MOUSE.PAN"));
        assert!(html.contains("'Home'") && html.contains("e.altKey"));
        // Review fixes: a fit resets the ortho dolly-zoom; picking ignores hidden meshes;
        // pick on the primary button only; double-click cancels the deferred single click.
        assert!(html.contains("orthoCam.zoom=1"), "ortho fit resets zoom");
        assert!(
            html.contains("pickable.filter(m=>m.visible)"),
            "picking skips hidden meshes"
        );
        assert!(
            html.contains("e.button!==0) return"),
            "middle-drag pan keeps selection"
        );
        assert!(
            html.contains("clearTimeout(legendClickT)"),
            "dbl-click cancels the single-click toggle"
        );
    }

    /// A scene with two elements sharing a group, plus a weld operation — the shape the split-row
    /// case needs (one group appearing under two categories) and the one welds live in.
    fn legend_scene(legend: Value) -> Value {
        json!({ "scene": {
            "meta": {"name":"x"},
            "groups": [{"key":"W16X26","label":"W16X26","color":"#94a3b8"}],
            "elements": [
                {"id":"b1","kind":"member","group":"W16X26","from":[0,0,0],"to":[1000,0,0],"widthMm":100,"depthMm":200},
                {"id":"b2","kind":"member","group":"W16X26","from":[0,0,500],"to":[1000,0,500],"widthMm":100,"depthMm":200}
            ],
            "operations": [
                {"id":"j1:weld:op","kind":"weld","mainId":"b1","secondaryId":"b2",
                 "path":[[0,0,0],[0,0,500]],"weldType":"fillet","sizeMm":6,"around":false,"shop":true}
            ],
            "legend": legend
        } })
    }

    /// One mode, one section, one category, rows split by explicit targets — the case group
    /// annotation could not express, and the reason the descriptor exists at all.
    fn legend_ok() -> Value {
        json!({ "v":1, "interaction":"select", "modes":[
            {"key":"type","label":"By type","sections":[
                {"key":"members","label":"Members","categories":[
                    {"key":"beam","label":"Beams","rows":[
                        {"key":"beam|W16X26","label":"W16X26","groups":["W16X26"],"targets":["b1"]}]},
                    {"key":"brace","label":"Braces","rows":[
                        {"key":"brace|W16X26","label":"W16X26","groups":["W16X26"],"targets":["b2"]}]}]},
                {"key":"connections","label":"Connections","categories":[
                    {"key":"","label":null,"rows":[
                        {"key":"weld","label":"Welds","targets":["j1:weld:op"]}]}]}]}]})
    }

    #[test]
    fn a_valid_legend_descriptor_survives_and_reaches_the_page() {
        let out = viewer_3d_render(&legend_scene(legend_ok()), true).unwrap();
        let html = out["html"].as_str().unwrap();
        assert!(
            html.contains("\"legend\":{"),
            "the descriptor reaches the page"
        );
        // Matched as the injected JSON KEY: the template itself mentions `legendError` in the
        // console.warn that surfaces it, so a bare word match would pass vacuously.
        assert!(
            !html.contains("\"legendError\":"),
            "a valid descriptor is not flagged"
        );
        // A weld operation is addressable — `groups` alone could never express it, because weld
        // operations carry no group.
        assert!(
            html.contains("j1:weld:op"),
            "weld operations are addressable targets"
        );
    }

    #[test]
    fn an_unusable_legend_falls_back_wholesale_rather_than_half_rendering() {
        // Each of these is a producer bug that would otherwise yield a panel whose rows silently
        // control nothing. Every one must drop the descriptor and say why — never error the render,
        // because the panel is chrome and the model is the payload.
        let cases: Vec<(&str, Value)> = vec![
            (
                "unknown version",
                json!({"v":2,"interaction":"select","modes":[]}),
            ),
            ("missing interaction", json!({"v":1,"modes":[]})),
            (
                "unknown interaction",
                json!({"v":1,"interaction":"hover","modes":[]}),
            ),
            (
                "empty modes",
                json!({"v":1,"interaction":"select","modes":[]}),
            ),
            // A target that is not a rendered element or weld operation — e.g. a structural grid.
            (
                "bad target",
                json!({"v":1,"interaction":"select","modes":[
                {"key":"m","label":"M","sections":[{"key":"s","label":"S","categories":[
                    {"key":"c","label":"C","rows":[{"key":"r","label":"R","targets":["grid-A"]}]}]}]}]}),
            ),
            // The same row key twice in one mode makes the Shift-range anchor ambiguous.
            (
                "duplicate row key in a mode",
                json!({"v":1,"interaction":"select","modes":[
                {"key":"m","label":"M","sections":[{"key":"s","label":"S","categories":[
                    {"key":"c1","label":"C1","rows":[{"key":"dup","label":"R","targets":["b1"]}]},
                    {"key":"c2","label":"C2","rows":[{"key":"dup","label":"R","targets":["b2"]}]}]}]}]}),
            ),
            // Two leaf rows in one mode fighting over the same target.
            (
                "overlapping targets in a mode",
                json!({"v":1,"interaction":"select","modes":[
                {"key":"m","label":"M","sections":[{"key":"s","label":"S","categories":[
                    {"key":"c1","label":"C1","rows":[{"key":"r1","label":"R1","targets":["b1"]}]},
                    {"key":"c2","label":"C2","rows":[{"key":"r2","label":"R2","targets":["b1"]}]}]}]}]}),
            ),
            // A row naming neither groups nor targets controls nothing at all.
            (
                "row addresses nothing",
                json!({"v":1,"interaction":"select","modes":[
                {"key":"m","label":"M","sections":[{"key":"s","label":"S","categories":[
                    {"key":"c","label":"C","rows":[{"key":"r","label":"R"}]}]}]}]}),
            ),
            // A group no element belongs to.
            (
                "unknown group",
                json!({"v":1,"interaction":"select","modes":[
                {"key":"m","label":"M","sections":[{"key":"s","label":"S","categories":[
                    {"key":"c","label":"C","rows":[{"key":"r","label":"R","groups":["NOPE"]}]}]}]}]}),
            ),
        ];
        for (name, legend) in cases {
            let out = viewer_3d_render(&legend_scene(legend), true).unwrap_or_else(|e| {
                panic!("{name}: render must not fail, the model is the payload: {e:?}")
            });
            let html = out["html"].as_str().unwrap();
            assert!(
                html.contains("\"legendError\":"),
                "{name}: must be reported to the producer"
            );
            // The injected scene must carry no descriptor at all. Checked against the JSON shape
            // `"legend":{` — the template's own `#modes` display menu and `#legend` element mean a
            // bare word match would pass vacuously.
            assert!(
                !html.contains("\"legend\":{"),
                "{name}: the descriptor must be dropped wholesale, not partly rendered"
            );
        }
    }

    #[test]
    fn a_target_the_renderer_skips_is_not_a_valid_target() {
        // An unsupported element kind never reaches the scene, so a row aiming at it would pass
        // validation and then be quietly dropped when the panel resolved its rows — the row would
        // vanish instead of the descriptor falling back wholesale, which is exactly the ambiguous
        // half-panel the atomic check exists to prevent.
        let out = viewer_3d_render(
            &json!({ "scene": {
                "meta": {"name":"x"},
                "groups": [{"key":"g","label":"G","color":"#60a5fa"}],
                "elements": [
                    {"id":"ok1","kind":"member","group":"g","from":[0,0,0],"to":[1000,0,0],"widthMm":100,"depthMm":100},
                    {"id":"weird","kind":"tesseract","group":"g"}
                ],
                "legend": {"v":1,"interaction":"select","modes":[
                    {"key":"m","label":"M","sections":[{"key":"s","label":"S","categories":[
                        {"key":"c","label":"C","rows":[
                            {"key":"r","label":"R","targets":["weird"]}]}]}]}]}
            } }),
            true,
        )
        .unwrap();
        let html = out["html"].as_str().unwrap();
        assert!(
            html.contains("\"legendError\":"),
            "a target the renderer skips must invalidate the descriptor"
        );
        assert!(
            !html.contains("\"legend\":{"),
            "and it must be dropped wholesale, not left half-rendered"
        );
    }

    #[test]
    fn legend_fallback_is_visible_to_a_headless_caller() {
        // A producer running without a browser must be able to SEE that its panel was rejected.
        let out = viewer_3d_render(
            &legend_scene(json!({"v":1,"interaction":"select","modes":[]})),
            true,
        )
        .unwrap();
        let warnings = out["warnings"].as_array().unwrap();
        assert!(
            warnings
                .iter()
                .any(|w| w.get("code").and_then(Value::as_str) == Some("legend-ignored")),
            "the fallback reason is reported in the result, not only inside the HTML"
        );
    }

    #[test]
    fn duplicate_category_keys_in_one_mode_are_rejected() {
        // Category keys carry the collapse state and the header identity, so a duplicate would drop
        // one header and tie both categories' collapse together.
        let out = viewer_3d_render(
            &legend_scene(json!({"v":1,"interaction":"select","modes":[
                {"key":"m","label":"M","sections":[{"key":"s","label":"S","categories":[
                    {"key":"dup","label":"C1","rows":[{"key":"r1","label":"R1","targets":["b1"]}]},
                    {"key":"dup","label":"C2","rows":[{"key":"r2","label":"R2","targets":["b2"]}]}]}]}]})),
            true,
        )
        .unwrap();
        assert!(out["html"].as_str().unwrap().contains("\"legendError\":"));
    }

    #[test]
    fn a_truthy_non_string_kind_is_not_a_valid_target() {
        // classify_scene can INFER `member` from the geometry, but the browser does
        // `switch(el.kind||'box')` and skips a numeric kind — so it must not be addressable.
        let out = viewer_3d_render(
            &json!({ "scene": {
                "meta": {"name":"x"},
                "groups": [{"key":"g","label":"G","color":"#60a5fa"}],
                "elements": [
                    {"id":"odd","kind":123,"group":"g","from":[0,0,0],"to":[1000,0,0],"widthMm":100,"depthMm":100}
                ],
                "legend": {"v":1,"interaction":"select","modes":[
                    {"key":"m","label":"M","sections":[{"key":"s","label":"S","categories":[
                        {"key":"c","label":"C","rows":[{"key":"r","label":"R","targets":["odd"]}]}]}]}]}
            } }),
            true,
        )
        .unwrap();
        assert!(
            out["html"].as_str().unwrap().contains("\"legendError\":"),
            "a kind the browser will skip cannot be a legend target"
        );
    }

    #[test]
    fn work_area_and_clip_match_the_editor() {
        // Toolbar parity with the floless steel editor. The work area gains its two switches, and
        // the important one is `whole`: ON (the default) a part touching the box is drawn in FULL
        // and parts outside are dropped, so a freshly-defined work area never slices anything by
        // surprise. Only the cut mode may contribute clipping planes.
        let out = viewer_3d_render(
            &json!({ "scene": { "meta": {"name":"x"}, "elements": [] } }),
            true,
        )
        .unwrap();
        let html = out["html"].as_str().unwrap();

        assert!(html.contains(r#"id="waOn""#), "Show work area tick");
        assert!(html.contains(r#"id="waWhole""#), "Show whole parts tick");
        assert!(
            html.contains(r#"role="menuitemcheckbox""#),
            "the ticks are real checkable menu items, not plain buttons"
        );
        // whole is the default for a new work area, and survives a re-define.
        assert!(
            html.contains("const whole = workArea ? workArea.whole : true;"),
            "a new work area defaults to showing whole parts"
        );
        // Whole mode must NOT clip; cut mode must.
        assert!(
            html.contains("if(workArea && workArea.enabled && !workArea.whole) active.push(...workArea.planes);"),
            "only the cut mode contributes clipping planes"
        );
        // ...and whole mode filters entire meshes instead.
        assert!(
            html.contains("vis = hit.intersectsBox(waWhole)"),
            "whole mode hides parts outside the box rather than slicing them"
        );
        // The box disappears when the work area is switched off.
        assert!(
            html.contains("if(!workArea || !workArea.enabled || workArea.box.isEmpty()) return;"),
            "no wireframe while switched off"
        );

        // Clip: the editor's Shift+X / Shift+B, and an armed button that is its own cancel.
        assert!(
            html.contains("(e.key==='X'||e.key==='x')"),
            "Shift+X arms a clip plane"
        );
        assert!(
            html.contains("(e.key==='B'||e.key==='b')"),
            "Shift+B adds a clip box"
        );
        assert!(
            html.contains("btn.textContent=clipMode?'Clip \u{2715}':'Clip \u{25be}'"),
            "the armed button becomes its own cancel target"
        );
    }

    #[test]
    fn chrome_is_host_controllable_and_typing_safe() {
        // Three independent bits of viewer chrome, asserted together because they all only exist in
        // the rendered document: a scene with no `panels` must reclaim the side-panel column rather
        // than show an empty box; an embedding host must be able to suppress our title (never the
        // default — standalone has no other model identity); and the single-key view shortcuts must
        // not fire while a text field has focus, or no input in this document can ever work.
        let out = viewer_3d_render(
            &json!({ "scene": { "meta": {"name":"x"}, "elements": [] } }),
            true,
        )
        .unwrap();
        let html = out["html"].as_str().unwrap();

        // The side-panel column is reclaimed as a unit — hiding the panel alone would leave the
        // ViewCube offset past a panel that is not there and the toolbar reserving its width.
        assert!(
            html.contains("body.no-side #side{display:none}"),
            "side panel hides"
        );
        assert!(
            html.contains("body.no-side #viewcube{right:16px}"),
            "ViewCube reclaims the column"
        );
        assert!(
            html.contains("body.no-side #toolbar{max-width:calc(100% - 156px)}"),
            "toolbar stops reserving the panel's width"
        );
        assert!(
            html.contains("classList.toggle('no-side',!hasPanels)"),
            "the state is driven by whether the scene supplied panels"
        );

        // Host-controlled title: opt-IN suppression, so standalone keeps its identity.
        assert!(
            html.contains("d.type!=='viewer-presentation'"),
            "listens for the presentation message"
        );
        assert!(
            html.contains("body.no-title #sceneName{display:none}"),
            "title can be suppressed"
        );
        assert!(
            html.contains("classList.toggle('no-title', showTitle===false)"),
            "only an explicit showTitle:false hides it"
        );

        // Typing guard: no view shortcut may fire into a focused text field.
        assert!(
            html.contains("function typingInto(t)"),
            "focus guard exists"
        );
        assert!(
            html.contains("if(typingInto(e.target) && e.key!=='Escape') return;"),
            "the keydown handler consults it before any shortcut"
        );
    }

    #[test]
    fn ships_boxselect_and_viewcube() {
        // #258 rework (qa-rejected): left mouse must area/box-select (not orbit), orbit moves to
        // the right button, and an interactive ViewCube ("cubix with planes") is added. Assert
        // each capability is wired into the rendered document.
        let out = viewer_3d_render(
            &json!({ "scene": { "meta": {"name":"x"}, "elements": [] } }),
            true,
        )
        .unwrap();
        let html = out["html"].as_str().unwrap();
        // Left no longer orbits; right-drag orbits, middle-drag still pans.
        assert!(
            html.contains("LEFT:-1, MIDDLE:THREE.MOUSE.PAN, RIGHT:THREE.MOUSE.ROTATE"),
            "left disabled for orbit; right orbits"
        );
        // Left-drag rubber-band multi-select + left-click single pick.
        assert!(
            html.contains("id=\"rubber\""),
            "rubber-band overlay element"
        );
        assert!(
            html.contains("function meshesInRect") && html.contains("function setSelection"),
            "box-select + selection model"
        );
        assert!(
            html.contains("function pickAt"),
            "left-click still picks one element"
        );
        // Interactive ViewCube with six clickable faces mapped to named views.
        assert!(html.contains("id=\"viewcube\""), "viewcube host element");
        assert!(html.contains("const CUBE_FACES="), "cube faces table");
        assert!(
            html.contains("{label:'FRONT',view:'front'}")
                && html.contains("{label:'RIGHT',view:'right'}"),
            "labelled cube faces map to named views"
        );
        assert!(
            html.contains("function syncCube"),
            "cube mirrors camera orientation"
        );
        assert!(
            html.contains("cubeRenderer.render(cubeScene,cubeCam)"),
            "cube rendered each frame"
        );
        // World-axis triad (bottom-right): a passive readout of the scene's X/Y/Z, rendered and
        // camera-synced each frame like the cube, with the axes run through the up-conversion.
        assert!(html.contains("id=\"axestriad\""), "axes triad host element");
        assert!(
            html.contains("function syncTriad")
                && html.contains("triadRenderer.render(triadScene,triadCam)"),
            "triad mirrors the camera and renders each frame"
        );
        assert!(
            html.contains("conv(axis,up)"),
            "triad axes use the scene up-conversion (labels mean the producer's axes)"
        );
        // The cube fully replaces the named-view buttons: a face press → ortho view, an edge/corner
        // press → iso-style view (so Iso stays reachable without a toolbar button).
        assert!(
            html.contains("cubeMesh.worldToLocal(hit.point.clone())"),
            "cube edge/corner picking yields iso views"
        );
        // Alt+Z now frames the whole selection (not just one element).
        assert!(
            html.contains("for(const m of selection) b.expandByObject(m)"),
            "alt+z zooms the selection"
        );
        // Single-key view shortcuts mirror the cube faces (T/F/R/B/L, any case).
        assert!(
            html.contains("const VIEW_KEYS={ t:'top', f:'front', r:'right', b:'back', l:'left' }"),
            "T/F/R/B/L map to named views"
        );
        assert!(
            html.contains("VIEW_KEYS[e.key.toLowerCase()]"),
            "view keys are case-insensitive and skip when a modifier is held"
        );
    }

    #[test]
    fn cad_camera_zoom_to_cursor_and_repivot() {
        // The viewer's camera matches the floless steel-3d-view CAD feel: no orbit inertia,
        // wheel zooms toward the cursor, and the orbit pivot re-centres under the cursor on
        // every gesture (raycast → on-axis depth, so the view never jumps).
        let out = viewer_3d_render(
            &json!({ "scene": { "meta": {"name":"x"}, "elements": [] } }),
            true,
        )
        .unwrap();
        let html = out["html"].as_str().unwrap();
        assert!(
            html.contains("controls.enableDamping=false"),
            "no post-release inertia"
        );
        assert!(
            html.contains("controls.zoomToCursor=true"),
            "wheel zooms toward the cursor"
        );
        assert!(
            html.contains("function repivotToCursor"),
            "cursor re-pivot present"
        );
        assert!(
            html.contains("controls.addEventListener('start', repivotToCursor)"),
            "re-pivot wired to every gesture start"
        );
    }

    #[test]
    fn ships_clip_planes_boxes_and_work_area() {
        // The viewer can section the model (Tekla-style): clip planes from a clicked face, clip
        // boxes (6 inward planes), and a work-area box — all driven through renderer.clippingPlanes,
        // with the work-area wireframe drawn in a 2nd unclipped pass so a clip never hides it.
        let out = viewer_3d_render(
            &json!({ "scene": { "meta": {"name":"x"}, "elements": [] } }),
            true,
        )
        .unwrap();
        let html = out["html"].as_str().unwrap();
        assert!(
            html.contains("renderer.localClippingEnabled=true"),
            "renderer-level clipping enabled"
        );
        assert!(
            html.contains("function applyClips") && html.contains("renderer.clippingPlanes="),
            "global clip planes driven by applyClips"
        );
        assert!(
            html.contains("function addClipPlaneAtScreen") && html.contains("function boxToPlanes"),
            "clip plane from a face + box → 6 inward planes"
        );
        assert!(
            html.contains("function addClipBox") && html.contains("function clearClips"),
            "add clip box + clear"
        );
        assert!(
            html.contains("function workAreaSetAll") && html.contains("Box3Helper"),
            "work-area box with a wireframe"
        );
        assert!(
            html.contains("renderer.autoClear=false"),
            "2nd unclipped pass keeps the work-area wireframe visible"
        );
    }

    #[test]
    fn clips_are_editable_objects_not_derived_planes() {
        // A clip used to keep ONLY its derived planes — a box threw its Box3 away at creation — so
        // nothing could be dragged, disabled or renamed afterwards. The source geometry is now the
        // record and `.planes` is derived from it.
        let out = viewer_3d_render(
            &json!({ "scene": { "meta": {"name":"x"}, "elements": [] } }),
            true,
        )
        .unwrap();
        let html = out["html"].as_str().unwrap();

        assert!(
            html.contains("function rebuildClipPlanes"),
            "source geometry re-derives the planes"
        );
        assert!(
            html.contains("function applyClips(){ const active=clips.filter(c=>c.enabled).flatMap(c=>c.planes);"),
            "a disabled clip must stop cutting — applyClips used to flatten every clip unconditionally"
        );
        // The shadow pass holds its own reference to the plane array, so every clip change has to
        // re-point it or the model stays visibly clipped after the clip is gone.
        assert!(
            html.contains(
                "renderer.clippingPlanes=active.length?active:EMPTY_CLIPS; syncClipMirror(); }"
            ),
            "applyClips still re-points the shadow-pass mirror"
        );
        for f in [
            "function toggleClip",
            "function renameClip",
            "function removeClip",
            "function setSelectedClips",
            "function deleteSelectedClips",
            "function getClips",
            "function addClipBoxFromBox",
        ] {
            assert!(html.contains(f), "clip lifecycle: {f}");
        }
        // Selection drives the gizmo only. If it also re-applied the clips, selecting a clip would
        // change what is cut.
        assert!(
            html.contains("function setSelectedClips(ids){ selectedClipIds=new Set((ids||[]).filter(id=>clips.some(c=>c.id===id))); renderClipGizmo(); refreshClipList(); }"),
            "selection never calls applyClips, and drops ids that no longer exist"
        );
        // Raycaster ignores renderer.clippingPlanes, so an unfiltered pick drops a plane on a face
        // that is visually cut away.
        assert!(
            html.contains("function isPointClipped")
                && html.contains(".find(h=>h.face&&!isPointClipped(h.point))"),
            "a face hidden behind an active clip must not be pickable"
        );
        // One plane per command, and a miss stays armed to retry.
        assert!(
            html.contains(
                "if(addClipPlaneAtScreen(e.clientX,e.clientY)){ setClipPlanePreview(null,null); setClipMode(null); }"
            ),
            "placing a plane disarms and drops the ghost; missing does neither"
        );
        assert!(
            !html.contains(
                "STAYS armed (crosshair + lit button + Esc/Clip to cancel) — parity with floless"
            ),
            "that comment claimed a parity with floless that did not exist — floless disarms on success"
        );
    }

    #[test]
    fn clip_manipulator_and_one_gesture_owner() {
        let out = viewer_3d_render(
            &json!({ "scene": { "meta": {"name":"x"}, "elements": [] } }),
            true,
        )
        .unwrap();
        let html = out["html"].as_str().unwrap();

        // The patch basis must avoid a degenerate cross product with the plane normal, and the
        // rendered world is Y-UP. A z-up port tests n.z, which picks the wrong seed for a VERTICAL
        // cut and for nothing else — invisible in every other case.
        assert!(
            html.contains("const u=(Math.abs(n.y)<0.9?_UPV:_XV).clone().cross(n).normalize();"),
            "the plane-patch basis guard is on the UP component (y), not z"
        );
        // Projected size depends on camera-space depth. Euclidean distance oversizes off-axis
        // handles, which desynchronises what is drawn from the fixed-pixel picker.
        assert!(
            html.contains("_camPt.copy(pos).applyMatrix4(camera.matrixWorldInverse);"),
            "pxToWorldAt uses camera-space depth, not distanceTo"
        );
        // Scope the negative guard to pxToWorldAt itself. Pinned to a bare identifier it could never
        // fire — that exact string has never existed here — while `camera.position.distanceTo` IS
        // legitimately used by reframeOrtho and must stay.
        let px_fn = &html[html.find("function pxToWorldAt").unwrap()..];
        let px_body = &px_fn[..px_fn.find("function rayAt").unwrap()];
        assert!(
            !px_body.contains("distanceTo"),
            "pxToWorldAt must not go back to Euclidean camera distance"
        );
        for f in [
            "function renderClipGizmo",
            "function sizeClipHandles",
            "function pickClipHandle",
            "function clipHandlesScreen",
            "function setClipPlanePreview",
            "function clipPlanePreviewAt",
            "function planePatchCorners",
            "function lineClosestT",
            "function clearClipGizmo",
        ] {
            assert!(html.contains(f), "clip manipulator: {f}");
        }
        // Handles live in the UNCLIPPED overlay pass — in `scene` they would be sectioned by the
        // very planes they exist to move.
        assert!(
            html.contains("clipGizmo=new THREE.Group(); overlayScene.add(clipGizmo);"),
            "the gizmo is built into overlayScene"
        );
        // Persistent per selection, but a user cycling selections would still leak without disposal.
        assert!(
            html.contains("function clearClipGizmo(){ if(!clipGizmo) return; overlayScene.remove(clipGizmo); disposeSubtree(clipGizmo); clipGizmo=null; }"),
            "a replaced gizmo is disposed, not dropped"
        );
        // A box face dragged past its opposite would invert the box.
        assert!(
            html.contains("if(f.sign>0) c.box.max[f.axis]=Math.max(val,c.box.min[f.axis]+1);"),
            "box faces clamp to a 1 mm minimum extent"
        );

        // One owner decided on pointerdown, not three listeners observing the same gesture.
        assert!(
            html.contains("let gesture=null;") && html.contains("let gestureToken=0;"),
            "a single gesture owner with a token"
        );
        assert!(
            html.contains("function schedulePointerWork")
                && html.contains("requestAnimationFrame(()=>{ pointerRAF=0;"),
            "pointer work is coalesced to one unit per frame"
        );
        // The token is what stops a queued frame applying to a gesture that already ended.
        assert!(
            html.contains("if(p&&p.token===gestureToken) doPointerWork(p.x,p.y); }); }"),
            "a stale queued frame is a no-op"
        );
        // pointerup COMMITS, and must flush the last queued move first.
        assert!(
            html.contains(
                "flushPointerWork();   // the last few millimetres of a drag are part of the result"
            ),
            "pointerup flushes before committing"
        );
        // pointercancel reverts. Capture LOSS must not: OrbitControls binds capture on the same
        // element and pointer id and releases it with no `enabled` guard, so pressing a second mouse
        // button mid-drag made it drop our capture — treated as a revert, that silently discarded a
        // live edit. Re-acquire instead.
        assert!(
            html.contains("renderer.domElement.addEventListener('pointercancel', e=>{ if(e.pointerId===gesturePointerId) endGesture(true); });"),
            "pointercancel reverts the live drag"
        );
        assert!(
            html.contains("try{ renderer.domElement.setPointerCapture(e.pointerId); }catch{ endGesture(true); } });"),
            "capture loss re-acquires rather than cancelling"
        );
        // A second primary press must not re-baseline a live drag's revert state.
        assert!(
            html.contains("if(gesture) return;")
                && html.contains("gestureToken++; gesturePointerId=e.pointerId;"),
            "pointerdown is re-entrancy guarded and records its owning pointer"
        );
        // The armed mode is snapshotted at press, or an Escape while the button is still down
        // disarms the tool and then places a plane anyway on release.
        assert!(
            html.contains("if(g==='clip-place'&&placeMode&&clipMode===placeMode){"),
            "a cancelled arm cannot still commit on release"
        );
        // endGesture clears ownership FIRST: releasePointerCapture itself fires lostpointercapture,
        // so a non-idempotent handler would revert the drag it had just committed.
        assert!(
            html.contains("gesture=null; gestureToken++;"),
            "endGesture clears ownership before doing anything else"
        );
        assert!(
            html.contains("renderer.domElement.setPointerCapture(e.pointerId)"),
            "capture the pointer so a release off-canvas still reaches us"
        );
        // Escape mid-drag beats the armed-mode cancel.
        assert!(
            html.contains("if(e.key==='Escape' && gesture==='clip-handle'){ endGesture(true); e.preventDefault(); return; }"),
            "Escape reverts a live drag before anything else looks at it"
        );
        assert!(
            html.contains("if((e.key==='Delete'||e.key==='Backspace') && selectedClipIds.size){ deleteSelectedClips(); e.preventDefault(); return; }"),
            "Del removes selected clips, and only when clips are selected"
        );
    }

    #[test]
    fn clip_list_is_keyboard_reachable_and_themed() {
        let out = viewer_3d_render(
            &json!({ "scene": { "meta": {"name":"x"}, "elements": [] } }),
            true,
        )
        .unwrap();
        let html = out["html"].as_str().unwrap();

        // Two absolutely-positioned panels with their own calc() caps drift into each other on a
        // short viewport; one flex column owns the budget instead.
        assert!(
            html.contains(r#"<div id="bottomLeft"><div id="clips" class="panel"></div><div id="legend" class="panel"></div></div>"#),
            "clips and legend share one bottom-left column"
        );
        assert!(
            html.contains("#bottomLeft{position:absolute;left:16px;bottom:16px;display:flex;flex-direction:column;gap:8px;max-height:calc(100% - 220px)}"),
            "the 220px reserve moved to the wrapper"
        );
        // A flex item defaults to min-height:auto and refuses to shrink below its content, so
        // without this at every level the first long list overflows the column regardless.
        assert!(
            html.contains("#clips .cbody{overflow-y:auto;overflow-x:hidden;min-height:0;margin:0 -4px;padding:0 4px}")
                && html.contains("flex:1 1 auto;min-height:140px}"),
            "both bodies shrink, and the legend keeps a protected floor"
        );
        // The legacy flat legend never had a cap of its own — as a flex child it would run past the
        // column.
        assert!(
            html.contains("#bottomLeft>#legend:not(.objects){min-height:0;overflow-y:auto;"),
            "the non-descriptor legend is bounded too"
        );
        // Scroll containers are themed — a native light scrollbar on a dark panel is the leak.
        assert!(
            html.contains("#clips .cbody::-webkit-scrollbar-thumb{background:var(--border-2);"),
            "the clip list's scrollbar is themed"
        );
        // Every row control is a real button; the hover-revealed ones each have a keyboard route
        // (Del for delete, F2 for rename), so revealing on hover is discoverability, not a trap.
        assert!(
            html.contains("const ren=el('button','cren','✎'); ren.type='button';")
                && html.contains("const del=el('button','cdel','×'); del.type='button';"),
            "rename and delete are real buttons"
        );
        assert!(
            html.contains("if(ev.key==='F2'){ ev.preventDefault(); ev.stopPropagation(); startClipRename(c.id); }"),
            "F2 renames without needing the hover-revealed pencil"
        );
        assert!(
            html.contains("box.setAttribute('role','checkbox'); box.setAttribute('aria-checked',c.enabled?'true':'false');")
                && html.contains("pick.setAttribute('aria-pressed',selectedClipIds.has(c.id)?'true':'false');"),
            "enable state and selection state are both exposed"
        );
        assert!(
            html.contains("box.setAttribute('aria-label','Enable or disable — '+c.label);"),
            "icon-only controls carry an accessible name"
        );
        // Colour alone must not carry plane-vs-box once a clip is renamed off its default label.
        assert!(
            html.contains("pick.setAttribute('data-tip',kind+' — select"),
            "the clip kind is available as text, not only as a swatch colour"
        );
        // Invalid rename keeps the row in edit mode, and each attempt inserts a FRESH alert node —
        // re-using one and only changing its text does not reliably re-announce a repeated message.
        assert!(
            html.contains("const err=el('div','cerr',msg); err.id='clipErr'; err.setAttribute('role','alert');"),
            "a rename error is announced"
        );
        assert!(
            html.contains("inp.setAttribute('aria-invalid','true'); inp.setAttribute('aria-describedby',err.id);"),
            "the invalid field points at its message"
        );
        // A plain click on the only selected clip clears it — the sole modifier-free way to dismiss
        // the handles.
        assert!(
            html.contains("next=(cur.size===1&&cur.has(id))?[]:[id]; clipAnchor=id;"),
            "plain click toggles off an already-solely-selected clip"
        );
        assert!(
            !html.contains(r#"title=""#),
            "tooltips are data-tip; native title= renders the light OS tooltip"
        );
    }

    #[test]
    fn clip_box_draw_snaps_in_the_rendered_up_frame() {
        let out = viewer_3d_render(
            &json!({ "scene": { "meta": {"name":"x"}, "elements": [] } }),
            true,
        )
        .unwrap();
        let html = out["html"].as_str().unwrap();

        // THE load-bearing line of the whole port. The source is native z-up and hardcodes index 2
        // as "the elevation to preserve"; the rendered world here is y-up, so the vertical index is
        // 1 and the plan axes are x/z. Copied verbatim, snapping happens in the wrong plane.
        assert!(
            html.contains("const UP_I=1, PLAN_I=[0,2], SNAP_TOL_PX=10;"),
            "the snap frame is parameterized on the rendered up axis"
        );
        assert!(
            html.contains("if(c.type==='vertical-axis'||c.type==='grid-int'){ const q=c.p.slice(); q[UP_I]=dragged[UP_I]; return q; }"),
            "a grid or column axis steers the PLAN and never yanks the elevation"
        );
        for f in [
            "function snapPoint",
            "function candidatePoint",
            "function buildClipCandidates",
            "function buildClipLevels",
            "function onClipBoxClick",
            "function clipBoxHeightAt",
            "function clipBoxFloorPoint",
            "function clipBoxPreviewAt",
            "function showReticle",
        ] {
            assert!(html.contains(f), "clip draw: {f}");
        }
        // sceneBox folds in grid bounds and level elevations, so its floor is the lowest DATUM;
        // meshBox filters on visibility, so hiding a member would move the floor mid-session.
        assert!(
            html.contains("function allMeshBounds(){ const b=new THREE.Box3(); for(const m of pickable) b.expandByObject(m); return b; }"),
            "the draw floor is the model's meshes, regardless of visibility"
        );
        // Both plan extents, independently — floless's `&&` lets a straight line through, and the
        // third click then disarms and silently adds nothing.
        assert!(
            html.contains("if(Math.abs(fp.xz[0]-clipBoxDraft.a[0])<1||Math.abs(fp.xz[1]-clipBoxDraft.a[1])<1) return;"),
            "a zero-width footprint is rejected at the second corner"
        );
        // In an axial view a vertical axis projects to a point: no height can be meant, so refuse
        // rather than commit the 1 mm clamp.
        assert!(
            html.contains("if(!h.usable){ setClipDrawPrompt('axial'); return; }"),
            "an axial view refuses the commit and says why"
        );
        assert!(
            html.contains("const y=Math.max(r.candidate?r.snapped[UP_I]:raw, fy+1);"),
            "the floor clamp is re-applied AFTER snapping, or a low level inverts the box"
        );
        // Element candidates come from the viewer's own normalization; raw from/to misses rods,
        // fasteners and node-like records.
        assert!(
            html.contains("const A=axisEnds(e);")
                && html.contains("push('intersection',[ix,fy,iz]);"),
            "candidates use axisEnds, and member-member intersections exist"
        );
        // Levels from the MODEL, not only from authored datums.
        assert!(
            html.contains(
                "for(const p of A){ const w=conv(p,SCENE_UP); ys.add(Math.round(w.y*1000)/1000); }"
            ),
            "height snaps to element elevations, so a model with no grid still snaps"
        );
        // A stale projection cache snaps to the wrong place; camera motion is not the only trigger.
        assert!(
            html.contains("controls.addEventListener('change', invalidateClipProjectionCache);")
                && html.contains("invalidateClipProjectionCache();   // frustum + zoom changed without the camera moving")
                && html.contains("invalidateClipProjectionCache();   // the camera OBJECT was swapped")
                && html.contains("invalidateClipProjectionCache();   // new projection matrix"),
            "the projection cache is invalidated on camera change, ortho reframe, projection swap and resize"
        );
        // Escape steps back through the draw rather than discarding the whole command.
        assert!(
            html.contains("if(clipBoxDraft.b) clipBoxDraft.b=null; else clipBoxDraft=null;"),
            "Escape steps height → footprint → armed"
        );
        assert!(
            html.contains("(e.key==='D'||e.key==='d')") && html.contains(r#"data-clip="draw""#),
            "the draw has a shortcut and a menu item"
        );
        // The reticle must not be occluded: the overlay pass reuses the main depth buffer, so
        // living in overlayScene is not on its own enough.
        assert!(
            html.contains("transparent:true,depthTest:false,depthWrite:false}"),
            "the snap reticle draws over everything"
        );
    }

    #[test]
    fn the_legacy_legend_binds_its_groups() {
        // v0.97.0 dropped `const groups=(S.groups||[])` from buildLegend while leaving the forEach
        // that reads it, so EVERY scene without a `scene.legend` descriptor threw ReferenceError out
        // of renderScene and killed the module — no model, no toolbar, no __viewer3d. The legacy list
        // is also the documented fallback for a REJECTED descriptor, so a producer bug in the panel
        // took the whole viewer down instead of degrading to the flat list it falls back to.
        let out = viewer_3d_render(
            &json!({ "scene": { "meta": {"name":"x"}, "elements": [] } }),
            true,
        )
        .unwrap();
        let html = out["html"].as_str().unwrap();
        assert!(
            html.contains("const groups=(S.groups||[]); if(!groups.length){ host.style.display='none'; return; } host.style.display='';"),
            "buildLegend must bind `groups` before iterating it"
        );
        // Ordering matters as much as presence: the binding has to precede the loop.
        let bind = html.find("const groups=(S.groups||[]);").unwrap();
        let loop_at = html
            .find("groups.forEach(g=>{ const row=el('div','row');")
            .unwrap();
        assert!(bind < loop_at, "`groups` is bound before it is iterated");
    }

    #[test]
    fn ships_grouped_toolbar_and_themed_tooltips() {
        // The toolbar groups into Camera | Display | Section, the Section cluster drives the clip /
        // work-area engine via dropdowns, and every control carries a themed tooltip (data-tip) —
        // no native title= leaking the OS default against the dark theme.
        let out = viewer_3d_render(
            &json!({ "scene": { "meta": {"name":"x"}, "elements": [] } }),
            true,
        )
        .unwrap();
        let html = out["html"].as_str().unwrap();
        assert!(
            html.contains("id=\"clip\"") && html.contains("id=\"work\""),
            "Section cluster: Clip + Work area buttons"
        );
        assert!(
            html.contains("data-clip=\"plane\"") && html.contains("data-clip=\"box\""),
            "clip dropdown items"
        );
        assert!(
            html.contains("data-wa=\"all\"") && html.contains("data-wa=\"sel\""),
            "work-area dropdown items"
        );
        // Themed tooltip element + data-tip on controls; the toolbar's native title= are gone.
        assert!(
            html.contains("#tooltip{")
                && html.contains("tooltip.id='tooltip'")
                && html.contains("data-tip=\"Fit all to view (Home)\""),
            "themed tooltip (CSS + driver) + data-tip on Fit"
        );
        assert!(
            !html.contains("id=\"fit\" title=") && !html.contains("id=\"proj\" title="),
            "native title= replaced by data-tip on the toolbar"
        );
    }

    #[test]
    fn per_element_and_per_group_opacity_round_trip() {
        // A producer can reveal embedded elements (rebar in concrete) by making a group
        // translucent — the schema's `opacity` is carried into the injected scene and the
        // renderer applies it (`transparent:op<1`).
        let scene = json!({
            "meta": { "name": "embedded" },
            "groups": [
                { "key": "concrete", "label": "Concrete", "color": "#94a3b8", "opacity": 0.25 },
                { "key": "rebar", "label": "Rebar", "color": "#ef4444" }
            ],
            "elements": [
                { "id": "C1", "group": "concrete", "kind": "box", "from": [0,0,0], "to": [0,0,3000], "section": {"w":400,"d":400} },
                { "id": "R1", "group": "rebar", "kind": "line", "from": [0,0,0], "to": [0,0,3000], "opacity": 1 }
            ]
        });
        let out = viewer_3d_render(&json!({ "scene": scene }), true).unwrap();
        let html = out["html"].as_str().unwrap();
        // The group opacity is injected into the scene JSON the page renders.
        assert!(html.contains("\"opacity\":0.25"), "group opacity injected");
        // And the renderer wires opacity onto the material.
        assert!(
            html.contains("transparent:op<1, opacity:op"),
            "material honours opacity"
        );
    }

    #[test]
    fn neutralizes_html_breakout_in_string_values() {
        let scene = json!({ "meta": { "name": "x" }, "elements": [],
            "panels": [ { "title": "</script><script>x<!--<script", "columns": [], "rows": [] } ] });
        let out = viewer_3d_render(&json!({ "scene": scene }), true).unwrap();
        let html = out["html"].as_str().unwrap();
        // Every `<` from the payload is escaped, so NO tokenizer-significant sequence survives:
        // not the close tag, not the comment / script-escaped openers (`<!--`, `<script`).
        assert!(!html.contains("</script><script>x"));
        assert!(!html.contains("<!--<script"));
        assert!(html.contains("\\u003C/script>\\u003Cscript>x\\u003C!--\\u003Cscript"));
    }

    #[test]
    fn missing_scene_is_a_validation_error() {
        let err = viewer_3d_render(&json!({}), true).unwrap_err();
        assert!(matches!(err, AwareError::Validation(_)));
    }

    #[test]
    fn non_object_scene_is_a_validation_error() {
        let err = viewer_3d_render(&json!({ "scene": "nope" }), true).unwrap_err();
        assert!(matches!(err, AwareError::Validation(_)));
    }
}
