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
  :root{--bg:#0a0f1a;--panel:rgba(15,23,42,.82);--border:#1e293b;--border-2:#334155;--text:#e2e8f0;--muted:#94a3b8;--accent:#60a5fa;--accent-2:#38bdf8}
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
  #legend{bottom:16px;left:16px;padding:12px 14px;font-size:12.5px} #legend .row{display:flex;align-items:center;gap:8px;margin:2px 0}
  #legend .legend-hint{color:var(--muted);font-size:11px;margin:0 0 6px}
  #legend .row{cursor:pointer;user-select:none;border-radius:5px;padding:2px 5px} #legend .row:hover{background:rgba(51,65,85,.5)}
  #legend .row.off{opacity:.4} #legend .row.off .swatch{filter:grayscale(1)}
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
  #toolbar .menu hr{border:0;border-top:1px solid var(--border);margin:4px 2px}
  /* Themed tooltip — replaces native title= so no OS-default tooltip leaks the dark theme. */
  #tooltip{position:fixed;z-index:50;background:rgba(15,23,42,.97);border:1px solid var(--border-2);border-radius:6px;padding:5px 8px;font-size:11.5px;line-height:1.35;color:var(--text);pointer-events:none;max-width:260px;box-shadow:0 8px 22px rgba(0,0,0,.5);opacity:0;transition:opacity .12s}
  #tooltip.show{opacity:1}
  #readout{bottom:16px;left:50%;transform:translateX(-50%);padding:10px 16px;font-size:13px;color:var(--muted);white-space:nowrap;max-width:60vw;overflow:hidden;text-overflow:ellipsis}
  #readout b{color:var(--text)} #readout .pill{color:var(--accent)}
  #rubber{position:absolute;border:1px solid var(--accent);background:rgba(96,165,250,.16);pointer-events:none;display:none;z-index:6}
  #viewcube{position:absolute;right:352px;top:74px;width:104px;height:104px;cursor:pointer;z-index:5}  /* top-right, left of the side panel (16+320+16) */
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
        <button data-clip="plane" data-tip="Click a model face to cut the view there">Add clip plane</button>
        <button data-clip="box" data-tip="Section a box around the selection (or whole model)">Add clip box</button>
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
        <button data-wa="clear" class="danger" data-tip="Remove the work area">Clear work area</button>
      </div>
    </div>
  </div>
</div>
<div id="side" class="panel"><h2 id="sideTitle">—</h2><p class="note" id="sideNote"></p><div id="panels"></div></div>
<div id="legend" class="panel"></div>
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
renderer.domElement.addEventListener('wheel', onWheelHover, {capture:true, passive:true});
renderer.domElement.addEventListener('pointermove', e=>{ lastHoverXY=[e.clientX,e.clientY]; }); // track the cursor for the gesture-start re-pivot on orbit/pan too (not just wheel) — parity with floless
let content=new THREE.Group(); scene.add(content); let pickable=[];
const conv=(P,up)=> up==='z' ? new THREE.Vector3(P[0],P[2],P[1]) : new THREE.Vector3(P[0],P[1],P[2]);

// ---- view state: scene bounds + display/visibility, driven by the toolbar + legend ----
let sceneBox=new THREE.Box3(); let maxDim=1;
const groupHidden=new Set(); let soloGroup=null; let displayMode='solid'; let legendClickT=null;

// Recompute the orthographic frustum so its on-screen scale matches the perspective
// camera's at the target plane (keeps zoom continuous across a projection toggle / resize).
function reframeOrtho(){
  const dist=camera.position.distanceTo(controls.target)||maxDim;
  const h=Math.tan(THREE.MathUtils.degToRad(perspCam.fov)*0.5)*dist, aspect=innerWidth/innerHeight||1;
  orthoCam.left=-h*aspect; orthoCam.right=h*aspect; orthoCam.top=h; orthoCam.bottom=-h; orthoCam.updateProjectionMatrix();
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
  controls.update(); activate('#proj button','data-proj',mode);
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
  for(const m of pickable){ const k=m.userData&&m.userData.group;
    m.visible = !groupHidden.has(k) && (soloGroup===null || soloGroup===k); }
}
function toggleGroup(k){ if(groupHidden.has(k)) groupHidden.delete(k); else groupHidden.add(k); soloGroup=null; applyGroupVisibility(); refreshLegend(); }
function soloToggle(k){ soloGroup = soloGroup===k ? null : k; if(soloGroup) groupHidden.clear(); applyGroupVisibility(); refreshLegend(); }
function refreshLegend(){ document.querySelectorAll('#legend .row').forEach(r=>{ const k=r.dataset.key;
  r.classList.toggle('off', groupHidden.has(k) || (soloGroup!==null && soloGroup!==k)); }); }
function activate(sel,attr,val){ document.querySelectorAll(sel).forEach(b=>b.classList.toggle('on', b.getAttribute(attr)===val)); }

function clearContent(){ scene.remove(content);
  content.traverse(o=>{ if(o.geometry)o.geometry.dispose(); if(o.material)o.material.dispose(); });
  content=new THREE.Group(); scene.add(content); pickable=[]; }

function makeLabel(text,pos,maxDim){
  const c=document.createElement('canvas'); c.width=128; c.height=64; const g=c.getContext('2d');
  g.fillStyle='#60a5fa'; g.font='bold 40px ui-sans-serif,system-ui,sans-serif'; g.textAlign='center'; g.textBaseline='middle'; g.fillText(text,64,34);
  const sp=new THREE.Sprite(new THREE.SpriteMaterial({map:new THREE.CanvasTexture(c),transparent:true}));
  sp.scale.set(maxDim*0.09, maxDim*0.045, 1); sp.position.copy(pos); return sp;
}

// ---- structural cross-section profiles (extruded), derived from the member's profile name ----
// section.w = flange width / overall width, section.d = section depth. An optional
// section.shape ("I"|"C"|"L"|"TUBE"|"BOX") overrides the name-based guess.
function shapeOf(e){
  const p=((e.section&&e.section.shape)||(e.meta&&e.meta.profile)||'').toString().toUpperCase().trim();
  if(/^(W|M|S|HP|UC|UB|UKC|UKB|IPE|HE)/.test(p)) return 'I';
  if(/^(C|MC|PFC)/.test(p)) return 'C';
  if(/^L/.test(p)) return 'L';
  if(/^(HSS|PIPE|TS|SHS|RHS|CHS|TUBE|HSQ)/.test(p)) return 'TUBE';
  return 'BOX';
}
function profileShape(kind,w,d){
  const s=new THREE.Shape(), hw=w/2, hd=d/2;
  if(kind==='I'){ const tf=Math.min(d*0.5,Math.max(d*0.10,6)), tw=Math.min(w*0.5,Math.max(w*0.10,5));
    s.moveTo(-hw,-hd); s.lineTo(hw,-hd); s.lineTo(hw,-hd+tf); s.lineTo(tw/2,-hd+tf);
    s.lineTo(tw/2,hd-tf); s.lineTo(hw,hd-tf); s.lineTo(hw,hd); s.lineTo(-hw,hd);
    s.lineTo(-hw,hd-tf); s.lineTo(-tw/2,hd-tf); s.lineTo(-tw/2,-hd+tf); s.lineTo(-hw,-hd+tf); s.closePath();
  } else if(kind==='C'){ const tf=Math.max(d*0.10,5), tw=Math.max(w*0.12,5);
    s.moveTo(-hw,-hd); s.lineTo(hw,-hd); s.lineTo(hw,-hd+tf); s.lineTo(-hw+tw,-hd+tf);
    s.lineTo(-hw+tw,hd-tf); s.lineTo(hw,hd-tf); s.lineTo(hw,hd); s.lineTo(-hw,hd); s.closePath();
  } else if(kind==='L'){ const t=Math.max(Math.min(w,d)*0.18,5);
    s.moveTo(-hw,-hd); s.lineTo(hw,-hd); s.lineTo(hw,-hd+t); s.lineTo(-hw+t,-hd+t); s.lineTo(-hw+t,hd); s.lineTo(-hw,hd); s.closePath();
  } else if(kind==='TUBE'){ const t=Math.max(Math.min(w,d)*0.12,4);
    s.moveTo(-hw,-hd); s.lineTo(hw,-hd); s.lineTo(hw,hd); s.lineTo(-hw,hd); s.closePath();
    const h=new THREE.Path(); h.moveTo(-hw+t,-hd+t); h.lineTo(hw-t,-hd+t); h.lineTo(hw-t,hd-t); h.lineTo(-hw+t,hd-t); h.closePath(); s.holes.push(h);
  } else { s.moveTo(-hw,-hd); s.lineTo(hw,-hd); s.lineTo(hw,hd); s.lineTo(-hw,hd); s.closePath(); }
  return s;
}
function profileGeom(e,w,d,len){
  const kind=shapeOf(e);
  if(kind==='BOX') return new THREE.BoxGeometry(w,len,d);          // fallback: length on local Y
  const g=new THREE.ExtrudeGeometry(profileShape(kind,w,d), {depth:len, bevelEnabled:false});
  g.translate(0,0,-len/2); return g;                                // extruded along +Z, centred
}
const _ZA=new THREE.Vector3(0,0,1), _YA=new THREE.Vector3(0,1,0);
function orientMember(mesh, dir){
  const m=dir.clone().normalize();
  if(mesh.geometry.type==='BoxGeometry'){ mesh.quaternion.setFromUnitVectors(_YA,m); return; }
  const q=new THREE.Quaternion().setFromUnitVectors(_ZA,m);         // member axis = extrude (Z)
  const proj=_YA.clone().sub(m.clone().multiplyScalar(_YA.dot(m))); // world-up perpendicular to member
  if(proj.lengthSq()>1e-6){ proj.normalize();
    const ly=_YA.clone().applyQuaternion(q);                        // where section-depth currently points
    const ang=Math.atan2(ly.clone().cross(proj).dot(m), ly.dot(proj));
    q.premultiply(new THREE.Quaternion().setFromAxisAngle(m, ang)); // roll so the web stands vertical
  }
  mesh.quaternion.copy(q);
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
function applyTriplanar(material,scaleMm,local){
  // `local` projects from OBJECT space so a directional finish follows the member's own axes; the
  // default projects from world space so isotropic finishes stay continuous across neighbours.
  const posExpr=local?'transformed':'(modelMatrix*vec4(transformed,1.0)).xyz';
  const nrmExpr=local?'objectNormal':'mat3(modelMatrix)*objectNormal';
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
  material.customProgramCacheKey=()=>'triplanar:'+scaleMm+':'+(local?'local':'world');
  material.needsUpdate=true; }

function solidMaterial(e,colorOf,opacityOf,doubleSided){ const col=colorOf[e.group]||0xffffff;
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
  if(TILE_MM[fam]) applyTriplanar(mat,TILE_MM[fam],DIRECTIONAL.has(fam));
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
function expandSceneBounds(box,S,up){ const add=P=>{if(vec3(P))box.expandByPoint(conv(P,up));};
  const addRadius=(P,r)=>{if(!vec3(P)||!Number.isFinite(r))return; for(const dx of [-r,r])for(const dy of [-r,r])for(const dz of [-r,r])add([P[0]+dx,P[1]+dy,P[2]+dz]);};
  for(const e of (S.elements||[])){ if(!e)continue; add(e.from);add(e.to);add(e.at);add(e.center); const A=axisEnds(e);if(A){const r=(e.diameterMm||0)/2;addRadius(A[0],r);addRadius(A[1],r);}
    if(e.kind==='washer')addRadius(e.center,Math.max(e.outerDiameterMm/2,e.thicknessMm/2));
    if(e.kind==='nut'||e.kind==='bolt-head')addRadius(e.center,Math.max(e.acrossFlatsMm/Math.sqrt(3),e.thicknessMm/2));
    if(Array.isArray(e.positions))for(let i=0;i+2<e.positions.length;i+=3)add([e.positions[i],e.positions[i+1],e.positions[i+2]]);
    if(e.kind==='plate'&&e.frame&&Array.isArray(e.outline)){ const F=frameOf(e,up), z=e.thicknessMm/2; for(const p of e.outline)for(const dz of [-z,z])box.expandByPoint(F.o.clone().addScaledVector(F.u,p[0]).addScaledVector(F.v,p[1]).addScaledVector(F.n,dz)); }
  }
  for(const R of (S.referenceSystems||[])){ if(!R||R.kind!=='structural-grid')continue; const o=R.origin,b=R.bounds||{}, x0=Number(b.minX),x1=Number(b.maxX),y0=Number(b.minY),y1=Number(b.maxY); if(vec3(o)&&[x0,x1,y0,y1].every(Number.isFinite)){add([o[0]+x0,o[1]+y0,o[2]]);add([o[0]+x1,o[1]+y1,o[2]]); for(const l of (R.levels||[])){add([o[0]+x0,o[1]+y0,l.elevationMm]);add([o[0]+x1,o[1]+y1,l.elevationMm]);}} }
  for(const op of (S.operations||[])){ if(op&&op.kind==='weld'&&Array.isArray(op.path))for(const p of op.path)add(p); }
}
function addReferenceSystems(S,up){ for(const R of (S.referenceSystems||[])){ if(!R||R.kind!=='structural-grid')continue;
  const o=R.origin,b=R.bounds||{}, x0=b.minX,x1=b.maxX,y0=b.minY,y1=b.maxY, baseMat=new THREE.LineBasicMaterial({color:0x60a5fa,transparent:true,opacity:0.7});
  const line=(A,B)=>{const g=new THREE.BufferGeometry().setFromPoints([conv(A,up),conv(B,up)]);content.add(new THREE.Line(g,baseMat.clone()));};
  for(const a of (R.axes||[])){ const start=Number.isFinite(a.startMm)?a.startMm:(a.direction==='x'?y0:x0), end=Number.isFinite(a.endMm)?a.endMm:(a.direction==='x'?y1:x1);
    const A=a.direction==='x'?[o[0]+a.offsetMm,o[1]+start,o[2]]:[o[0]+start,o[1]+a.offsetMm,o[2]], B=a.direction==='x'?[o[0]+a.offsetMm,o[1]+end,o[2]]:[o[0]+end,o[1]+a.offsetMm,o[2]]; line(A,B); content.add(makeLabel(a.label,conv(B,up),maxDim)); }
  for(const l of (R.levels||[])){ const z=l.elevationMm,c=[o[0]+(x0+x1)/2,o[1]+(y0+y1)/2,z]; line([o[0]+x0,c[1],z],[o[0]+x1,c[1],z]); line([c[0],o[1]+y0,z],[c[0],o[1]+y1,z]); content.add(makeLabel(l.label,conv([o[0]+x1,c[1],z],up),maxDim)); }
} }
function addOperations(S,up){ for(const op of (S.operations||[])){ if(!op||op.kind!=='weld'||!Array.isArray(op.path))continue;
  const points=op.path.map(p=>conv(p,up)); if(points.length<2)continue;
  const geometry=new THREE.BufferGeometry().setFromPoints(points);
  const material=new THREE.LineBasicMaterial({color:0xf59e0b,transparent:true,opacity:0.95});
  const weld=new THREE.Line(geometry,material); weld.userData=op; content.add(weld);
} }

function renderScene(S){
  clearContent();
  const up=(S.meta&&S.meta.up)||'z';
  const colorOf={}, opacityOf={}; (S.groups||[]).forEach(g=>{ colorOf[g.key]=g.color; if(typeof g.opacity==='number') opacityOf[g.key]=g.opacity; });
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
    const mat=solidMaterial(e,colorOf,opacityOf,isMesh); let mesh;
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
      orientMember(mesh, dir); }
           mesh.receiveShadow=true;   // steel catches shadow from steel, not just from the ground; casting is decided per-pass in applyDisplayMode
    mesh.userData=e; content.add(mesh); pickable.push(mesh);
  }
  for(const g of (S.grids||[])) if(g&&Array.isArray(g.at)) content.add(makeLabel(g.label, conv(g.at,up), maxDim));
  addReferenceSystems(S,up);
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
  applyDisplayMode(); applyGroupVisibility();

  buildSidePanels(S); buildLegend(S); setHint();
  document.getElementById('sceneName').textContent=(S.meta&&S.meta.name)||'';
}

function buildSidePanels(S){
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
function buildLegend(S){ const host=document.getElementById('legend'); host.replaceChildren();
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
  for(const m of selection){ const mat=m.material; if(mat){ mat.emissive=new THREE.Color(0xf59e0b); mat.emissiveIntensity=0.6; } }
  if(selection.length===0){ setHint(); return; }
  if(selection.length===1){ const u=selection[0].userData; const parts=[el('b',null,u.id||'(element)')];
    if(u.group) parts.push(document.createTextNode(' · '), el('span','pill',u.group));
    for(const [k,v] of Object.entries(u.meta||{})) parts.push(document.createTextNode(` · ${k}: ${v}`));
    readout.replaceChildren(...parts); return; }
  readout.replaceChildren(el('b',null,String(selection.length)), document.createTextNode(' elements selected'));
}
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

// Left button drives selection (orbit moved to the right button, #258): a drag rubber-bands a
// multi-select; a click (movement under DRAG_PX) picks the single element under the cursor.
const rubber=document.getElementById('rubber'); let boxStart=null; const DRAG_PX=5;
renderer.domElement.addEventListener('pointerdown', e=>{ if(e.button!==0) return; boxStart={x:e.clientX,y:e.clientY}; });
renderer.domElement.addEventListener('pointermove', e=>{ if(!boxStart) return;
  const dx=e.clientX-boxStart.x, dy=e.clientY-boxStart.y; if(Math.hypot(dx,dy)<DRAG_PX) return;
  rubber.style.display='block'; rubber.style.left=Math.min(e.clientX,boxStart.x)+'px'; rubber.style.top=Math.min(e.clientY,boxStart.y)+'px';
  rubber.style.width=Math.abs(dx)+'px'; rubber.style.height=Math.abs(dy)+'px'; });
renderer.domElement.addEventListener('pointerup', e=>{ if(e.button!==0||!boxStart) return;
  const dx=e.clientX-boxStart.x, dy=e.clientY-boxStart.y; rubber.style.display='none';
  if(Math.hypot(dx,dy)>=DRAG_PX) setSelection(meshesInRect(boxStart.x,boxStart.y,e.clientX,e.clientY));
  else if(clipMode==='plane') addClipPlaneAtScreen(e.clientX,e.clientY); // armed → a click drops a clip plane on the picked face; STAYS armed (crosshair + lit button + Esc/Clip to cancel) — parity with floless
  else pickAt(e.clientX,e.clientY);
  boxStart=null; });

// ---- clip planes / boxes + work area (Tekla-style sectioning) ----
// Sectioning lives in renderer.clippingPlanes (GLOBAL), so it clips the grid + every element like
// Tekla and survives a re-render. A clip PLANE keeps the camera-far side (1 plane); a clip BOX and
// the work area keep INSIDE (6 inward planes). The ViewCube has its own renderer → never clipped.
const EMPTY_CLIPS=Object.freeze([]);
let clips=[]; let workArea=null; let clipMode=null; let clipSeq=0;
const overlayScene=new THREE.Scene(); let workAreaHelper=null; // work-area wireframe → 2nd UNCLIPPED pass
// three.js convention: a material is KEPT where distanceToPoint(p) = normal·p + constant >= 0 (the
// side the normal points toward) and discarded on the negative side. So INWARD normals + these
// constants keep the box interior (e.g. normal -X, constant max.x → keep x<=max.x). Verified live
// (a whole-model box keeps the model visible) — do not "reverse" these signs.
function boxToPlanes(b){ return [
  new THREE.Plane(new THREE.Vector3(-1,0,0), b.max.x), new THREE.Plane(new THREE.Vector3(1,0,0), -b.min.x),
  new THREE.Plane(new THREE.Vector3(0,-1,0), b.max.y), new THREE.Plane(new THREE.Vector3(0,1,0), -b.min.y),
  new THREE.Plane(new THREE.Vector3(0,0,-1), b.max.z), new THREE.Plane(new THREE.Vector3(0,0,1), -b.min.z) ]; }
function applyClips(){ const active=clips.flatMap(c=>c.planes); if(workArea) active.push(...workArea.planes);
  renderer.clippingPlanes=active.length?active:EMPTY_CLIPS; syncClipMirror(); }
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
// A clip plane from a clicked face (screen px): keep the camera-FAR side so the cut reveals the section.
function addClipPlaneAtScreen(cx,cy){
  const ndc=new THREE.Vector2((cx/innerWidth)*2-1, -(cy/innerHeight)*2+1); ray.setFromCamera(ndc,camera);
  const hit=ray.intersectObjects(pickable.filter(m=>m.visible),false)[0]; if(!hit||!hit.face) return null;
  const n=hit.face.normal.clone().transformDirection(hit.object.matrixWorld).normalize();
  if(n.dot(camera.position.clone().sub(hit.point))>0) n.negate();
  clips.push({ id:'clip'+(++clipSeq), kind:'plane', planes:[new THREE.Plane().setFromNormalAndCoplanarPoint(n,hit.point)] });
  applyClips(); return clips[clips.length-1].id; }
// A clip box around the current selection (or the whole model when nothing is selected).
function addClipBox(pad){ const box=selBox(pad); if(!box) return null;
  clips.push({ id:'clip'+(++clipSeq), kind:'box', planes:boxToPlanes(box) }); applyClips(); return clips[clips.length-1].id; }
function clearClips(){ clips=[]; applyClips(); }
function clipCount(){ return clips.length; }
// Arm/disarm the face-pick: 'plane' → next left-click on a face drops a plane; null → back to selecting.
function setClipMode(m){ clipMode=m==='plane'?'plane':null;
  renderer.domElement.style.cursor=clipMode?'crosshair':'default';
  const btn=document.getElementById('clip'); if(btn) btn.classList.toggle('on',!!clipMode);
  if(clipMode) readout.replaceChildren(el('b',null,'Click a face'), document.createTextNode(' to cut the view there · Esc to cancel'));
  else setHint();
  return clipMode; }
// Work area: one box that bounds (and sections) the view, shown as an always-visible wireframe.
function renderWorkArea(){ if(workAreaHelper){ overlayScene.remove(workAreaHelper); workAreaHelper.geometry.dispose(); workAreaHelper.material.dispose(); workAreaHelper=null; }
  if(!workArea || workArea.box.isEmpty()) return;
  workAreaHelper=new THREE.Box3Helper(workArea.box, new THREE.Color(0x60a5fa));
  workAreaHelper.material.depthTest=false; workAreaHelper.renderOrder=995; overlayScene.add(workAreaHelper); }
function setWorkAreaBox(box){ if(!box||box.isEmpty()) return false; workArea={ box:box.clone(), planes:boxToPlanes(box) }; applyClips(); renderWorkArea(); return true; }
function workAreaSetAll(){ const box=meshBox(pickable); return box.isEmpty() ? false : setWorkAreaBox(box); } // bound the whole model by its rendered mesh bounds (not centrelines)
function workAreaFromSelection(pad){ const box=new THREE.Box3();
  for(const m of selection){ if(m.visible) box.expandByObject(m); }
  if(box.isEmpty()) return false; box.expandByScalar(pad==null?Math.max(maxDim*0.04,1):pad); return setWorkAreaBox(box); }
function clearWorkArea(){ workArea=null; applyClips(); renderWorkArea(); }
function workAreaOn(){ return !!workArea; }

addEventListener('resize',()=>{
  perspCam.aspect=innerWidth/innerHeight; perspCam.updateProjectionMatrix();
  if(camera.isOrthographicCamera) reframeOrtho();
  renderer.setSize(innerWidth,innerHeight);
});
// Single-key view shortcuts mirror the ViewCube faces (lower- or upper-case).
const VIEW_KEYS={ t:'top', f:'front', r:'right', b:'back', l:'left' };
addEventListener('keydown',e=>{
  if(e.key==='Escape' && clipMode){ setClipMode(null); e.preventDefault(); return; } // cancel an armed clip-plane pick
  if(e.key==='Home'){ frameBox(sceneBox); e.preventDefault(); }                       // fit all
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
  const a=b.dataset.clip; if(a==='plane') setClipMode('plane'); else if(a==='box') addClipBox(); else if(a==='clear') clearClips(); }));
document.querySelectorAll('#workMenu [data-wa]').forEach(b=>b.addEventListener('click', e=>{ e.stopPropagation(); closeMenus();
  const a=b.dataset.wa; if(a==='all') workAreaSetAll(); else if(a==='sel') workAreaFromSelection(); else if(a==='clear') clearWorkArea(); }));
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

(function loop(){ requestAnimationFrame(loop); controls.update(); renderer.render(scene,camera);
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
  clipPlanes:()=>(renderer.clippingPlanes||[]).length };
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

fn length3(value: [f64; 3]) -> f64 {
    value
        .iter()
        .map(|component| component * component)
        .sum::<f64>()
        .sqrt()
}

fn dot3(a: [f64; 3], b: [f64; 3]) -> f64 {
    (0..3).map(|index| a[index] * b[index]).sum()
}

fn normalized3(value: [f64; 3]) -> Option<[f64; 3]> {
    let length = length3(value);
    (length > 1.0e-9).then(|| value.map(|component| component / length))
}

fn distance3(a: [f64; 3], b: [f64; 3]) -> f64 {
    length3([a[0] - b[0], a[1] - b[1], a[2] - b[2]])
}

fn cross3(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
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

fn polygon_edges(polygon: &[[f64; 2]]) -> impl Iterator<Item = ([f64; 2], [f64; 2])> + '_ {
    polygon
        .iter()
        .copied()
        .zip(polygon.iter().copied().cycle().skip(1))
        .take(polygon.len())
}

fn point_in_polygon(point: [f64; 2], polygon: &[[f64; 2]]) -> bool {
    let mut inside = false;
    for (a, b) in polygon_edges(polygon) {
        if (a[1] > point[1]) != (b[1] > point[1])
            && point[0] < (b[0] - a[0]) * (point[1] - a[1]) / (b[1] - a[1]) + a[0]
        {
            inside = !inside;
        }
    }
    inside
}

fn point_segment_distance(point: [f64; 2], a: [f64; 2], b: [f64; 2]) -> f64 {
    let delta = [b[0] - a[0], b[1] - a[1]];
    let length_sq = delta[0] * delta[0] + delta[1] * delta[1];
    if length_sq <= 1.0e-18 {
        return ((point[0] - a[0]).powi(2) + (point[1] - a[1]).powi(2)).sqrt();
    }
    let t =
        (((point[0] - a[0]) * delta[0] + (point[1] - a[1]) * delta[1]) / length_sq).clamp(0.0, 1.0);
    let nearest = [a[0] + t * delta[0], a[1] + t * delta[1]];
    ((point[0] - nearest[0]).powi(2) + (point[1] - nearest[1]).powi(2)).sqrt()
}

fn cross2(a: [f64; 2], b: [f64; 2], c: [f64; 2]) -> f64 {
    (b[0] - a[0]) * (c[1] - a[1]) - (b[1] - a[1]) * (c[0] - a[0])
}

fn segments_intersect(a: [f64; 2], b: [f64; 2], c: [f64; 2], d: [f64; 2]) -> bool {
    const EPS: f64 = 1.0e-9;
    let on_segment = |p: [f64; 2], q: [f64; 2], r: [f64; 2]| {
        cross2(p, q, r).abs() <= EPS
            && r[0] >= p[0].min(q[0]) - EPS
            && r[0] <= p[0].max(q[0]) + EPS
            && r[1] >= p[1].min(q[1]) - EPS
            && r[1] <= p[1].max(q[1]) + EPS
    };
    let (ab_c, ab_d, cd_a, cd_b) = (
        cross2(a, b, c),
        cross2(a, b, d),
        cross2(c, d, a),
        cross2(c, d, b),
    );
    (ab_c.signum() != ab_d.signum() && cd_a.signum() != cd_b.signum())
        || on_segment(a, b, c)
        || on_segment(a, b, d)
        || on_segment(c, d, a)
        || on_segment(c, d, b)
}

fn polygon_is_simple_nonzero(polygon: &[[f64; 2]]) -> bool {
    const EPS: f64 = 1.0e-9;
    if polygon.len() < 3
        || polygon_edges(polygon)
            .any(|(a, b)| ((a[0] - b[0]).powi(2) + (a[1] - b[1]).powi(2)).sqrt() <= EPS)
    {
        return false;
    }
    let twice_area = polygon_edges(polygon)
        .map(|(a, b)| a[0] * b[1] - b[0] * a[1])
        .sum::<f64>();
    if twice_area.abs() <= EPS {
        return false;
    }
    let n = polygon.len();
    for i in 0..n {
        for j in (i + 1)..n {
            if j == (i + 1) % n || (j + 1) % n == i {
                continue;
            }
            if segments_intersect(
                polygon[i],
                polygon[(i + 1) % n],
                polygon[j],
                polygon[(j + 1) % n],
            ) {
                return false;
            }
        }
    }
    true
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
    let length = |a: [f64; 3]| a.iter().map(|x| x * x).sum::<f64>().sqrt();
    let (ul, vl, nl) = (length(u), length(v), length(n));
    if ul <= 1.0e-9 || vl <= 1.0e-9 || nl <= 1.0e-9 {
        return Err(scene_error(
            &format!("{path}.frame"),
            "directions must be nonzero",
        ));
    }
    let dot = |a: [f64; 3], b: [f64; 3]| (0..3).map(|i| a[i] * b[i]).sum::<f64>();
    if (dot(u, v) / (ul * vl)).abs() > 1.0e-6 {
        return Err(scene_error(
            &format!("{path}.frame"),
            "uDir and vDir must be orthogonal",
        ));
    }
    let cross = [
        u[1] * v[2] - u[2] * v[1],
        u[2] * v[0] - u[0] * v[2],
        u[0] * v[1] - u[1] * v[0],
    ];
    if dot(cross, n) / (length(cross) * nl) < 1.0 - 1.0e-6 {
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
        match kind.as_str() {
            "line" | "box" | "member" => {
                let from = vector::<3>(object.get("from"), &format!("{path}.from"))?;
                let to = vector::<3>(object.get("to"), &format!("{path}.to"))?;
                if from == to {
                    return Err(scene_error(&path, "member axis must have nonzero length"));
                }
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
                let part_kinds = ["member", "line", "box", "plate"];
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
                        "directions must be nonzero",
                    ));
                };
                let cross = [
                    u[1] * v[2] - u[2] * v[1],
                    u[2] * v[0] - u[0] * v[2],
                    u[0] * v[1] - u[1] * v[0],
                ];
                if dot3(u, v).abs() > 1.0e-6
                    || dot3(normalized3(cross).unwrap(), normal) < 1.0 - 1.0e-6
                {
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
                        let tool_axis = tool.get("axis").and_then(Value::as_object).ok_or_else(|| {
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
                        positive_number(tool.get("diameterMm"), &format!("{path}.tool.diameterMm"))?;
                    }
                    Some("box") => {
                        let frame = tool.get("frame").and_then(Value::as_object).ok_or_else(|| {
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
                                scene_error(&format!("{path}.tool.halfExtents"), "must be three numbers")
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
        let participant_a = object["partToBoltTo"].as_str().unwrap();
        let participant_b = object["partToBeBolted"].as_str().unwrap();
        let frame = object["frame"].as_object().unwrap();
        let origin = vector::<3>(frame.get("origin"), &format!("{path}.frame.origin"))?;
        let u = normalized3(vector::<3>(
            frame.get("uDir"),
            &format!("{path}.frame.uDir"),
        )?)
        .unwrap();
        let v = normalized3(vector::<3>(
            frame.get("vDir"),
            &format!("{path}.frame.vDir"),
        )?)
        .unwrap();
        let normal = normalized3(vector::<3>(
            frame.get("normal"),
            &format!("{path}.frame.normal"),
        )?)
        .unwrap();
        let u_offsets = object["uOffsetsMm"]
            .as_array()
            .unwrap()
            .iter()
            .map(|value| value.as_f64().unwrap())
            .collect::<Vec<_>>();
        let v_offsets = object["vOffsetsMm"]
            .as_array()
            .unwrap()
            .iter()
            .map(|value| value.as_f64().unwrap())
            .collect::<Vec<_>>();
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
        let diameter = object["diameterMm"].as_f64().unwrap();
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
        let expected = object["uOffsetsMm"].as_array().unwrap().len()
            * object["vOffsetsMm"].as_array().unwrap().len();
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
                let child_element = scene_element(scene, child).unwrap();
                if field == "shankId" {
                    let (from, to) = axis(child_element, child)?;
                    let direction =
                        normalized3([to[0] - from[0], to[1] - from[1], to[2] - from[2]]).unwrap();
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
                    let child_axis = normalized3(direction(child_element, child)?).unwrap();
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
                    let child_element = scene_element(scene, child).unwrap();
                    let center = vector::<3>(
                        child_element.get("center"),
                        &format!("{instance_path}.{field}[{child_index}].center"),
                    )?;
                    let child_axis = normalized3(direction(child_element, child)?).unwrap();
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
            if effects.len() != 2 {
                return Err(scene_error(
                    &format!("{instance_path}.holeEffects"),
                    "must contain exactly one effect per bolt participant",
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
                if ![participant_a, participant_b].contains(&target)
                    || !effect_targets.insert(target)
                {
                    return Err(scene_error(
                        &format!("{effect_path}.targetId"),
                        "must uniquely reference a bolt participant",
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
            if effect_targets.len() != 2 {
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
    let receipt = classify_scene(scene)?;

    // Serialize the scene and inject it into the renderer shell as a JS object-literal
    // expression. Neutralize EVERY `<` as a `<` escape that renders back
    // to `<` at runtime) so no HTML-tokenizer-significant sequence can survive in a string
    // value: not just `</script>` (close) but also `<!--` / `<script` (which would push the
    // tokenizer into the script-data-(double-)escaped state and stop the template's own
    // closing `</script>` from closing the element). JSON only contains `<` inside string
    // values, so escaping all of them is safe. Also escape the JS line terminators U+2028/U+2029.
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

/// JSON type name for clear validation errors.
fn json_type(v: &Value) -> &'static str {
    match v {
        Value::Null => "null",
        Value::Bool(_) => "boolean",
        Value::Number(_) => "number",
        Value::String(_) => "string",
        Value::Array(_) => "array",
        Value::Object(_) => "object",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

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
            compact.contains("applyTriplanar(mat,TILE_MM[fam],DIRECTIONAL.has(fam))"),
            "the local flag is actually passed per family"
        );
        assert!(
            compact.contains("constposExpr=local?'transformed'")
                && compact.contains("constnrmExpr=local?'objectNormal'"),
            "local projection reaches the shader instead of only the cache key"
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
        assert!(
            html.contains("const z=l.elevationMm"),
            "level elevation is world Z"
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
