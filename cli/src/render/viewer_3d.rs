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
//! Determinism: identical `scene` input → identical HTML bytes (no clock, no environment).
//! Three.js loads from a pinned CDN for v1; full-inline (offline) is a planned follow-on.

use crate::error::AwareError;
use serde_json::Value;

/// The renderer shell. `__SCENE_JSON__` is replaced with the serialized scene. Every `{`/`}`
/// here is literal (we substitute with `str::replace`, not `format!`). Proven against the
/// floless reference prototype (one renderer drew both a steel frame and a bar chart).
const TEMPLATE: &str = r##"<!doctype html>
<html lang="en">
<head>
<meta charset="utf-8" />
<meta name="viewport" content="width=device-width, initial-scale=1" />
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
  #axestriad{position:absolute;right:16px;bottom:16px;width:72px;height:72px;z-index:5;pointer-events:none}
  #axestriad canvas{display:block;filter:drop-shadow(0 6px 14px rgba(0,0,0,.5))}
</style>
</head>
<body>
<div id="app"></div>
<div id="topbar" class="panel"><div class="brand"><b>AWARE</b> · viewer-3d</div><div class="sub" id="sceneName">—</div></div>
<div id="toolbar" class="panel">
  <!-- Camera: projection + fit -->
  <div class="tb-grp" id="proj">
    <button data-proj="persp" class="on" data-tip="Perspective view — natural depth">Persp</button><button data-proj="ortho" data-tip="Orthographic — true scale, no perspective">Ortho</button>
  </div>
  <button id="fit" data-tip="Fit all to view (Home)">Fit</button>
  <div class="tb-sep"></div>
  <!-- Display mode -->
  <div class="tb-grp" id="modes">
    <button data-mode="solid" class="on" data-tip="Solid shaded model">Solid</button><button data-mode="wire" data-tip="Wireframe — edges only">Wire</button><button data-mode="xray" data-tip="See-through — reveal hidden parts">X-ray</button>
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

const SCENE = __SCENE_JSON__;
const el=(tag,cls,text)=>{const e=document.createElement(tag);if(cls)e.className=cls;if(text!=null)e.textContent=text;return e;};

const scene=new THREE.Scene(); scene.background=new THREE.Color(0x0a0f1a);
// Two cameras share one position/target so the projection can be toggled live.
const perspCam=new THREE.PerspectiveCamera(50, innerWidth/innerHeight, 0.01, 1e7);
const orthoCam=new THREE.OrthographicCamera(-1,1,1,-1,0.01,1e7);
let camera=perspCam;
const renderer=new THREE.WebGLRenderer({antialias:true}); renderer.setPixelRatio(Math.min(devicePixelRatio,2)); renderer.setSize(innerWidth,innerHeight);
renderer.localClippingEnabled=true; // enable clip planes/boxes + the work area (Tekla-style sectioning) — driven via renderer.clippingPlanes (applyClips)
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
}
function setDisplayMode(m){ displayMode=m; applyDisplayMode(); activate('#modes button','data-mode',m); }
// solid → honour each material's base opacity; wire → wireframe; xray → translucent, no depth write.
function applyDisplayMode(){
  for(const mesh of pickable){ const mat=mesh.material; if(!mat) continue;
    const base=(mat.userData&&mat.userData.baseOpacity!=null)?mat.userData.baseOpacity:1;
    if(displayMode==='wire'){ mat.wireframe=true; mat.transparent=false; mat.opacity=1; mat.depthWrite=true; }
    else if(displayMode==='xray'){ mat.wireframe=false; mat.transparent=true; mat.opacity=Math.min(base,0.25); mat.depthWrite=false; }
    else { mat.wireframe=false; mat.opacity=base; mat.transparent=base<1; mat.depthWrite=true; }
    mat.needsUpdate=true;
  }
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

function renderScene(S){
  clearContent();
  const up=(S.meta&&S.meta.up)||'z';
  const colorOf={}, opacityOf={}; (S.groups||[]).forEach(g=>{ colorOf[g.key]=g.color; if(typeof g.opacity==='number') opacityOf[g.key]=g.opacity; });
  groupHidden.clear(); soloGroup=null;
  const box=new THREE.Box3();
  for(const e of (S.elements||[])){ if(Array.isArray(e.from))box.expandByPoint(conv(e.from,up)); if(Array.isArray(e.to))box.expandByPoint(conv(e.to,up)); if(Array.isArray(e.at))box.expandByPoint(conv(e.at,up)); }
  if(box.isEmpty()) box.set(new THREE.Vector3(-1,-1,-1), new THREE.Vector3(1,1,1));
  const size=box.getSize(new THREE.Vector3()), center=box.getCenter(new THREE.Vector3());
  maxDim=Math.max(size.x,size.y,size.z)||1; sceneBox=box.clone(); const thick=maxDim*0.006;

  content.add(new THREE.HemisphereLight(0x9fc5ff,0x0a0f1a,0.95));
  const key=new THREE.DirectionalLight(0xffffff,1.3); key.position.copy(center).add(new THREE.Vector3(maxDim,maxDim*1.5,maxDim*0.6)); content.add(key);
  const fill=new THREE.DirectionalLight(0x88aaff,0.5); fill.position.copy(center).add(new THREE.Vector3(-maxDim,maxDim*0.7,-maxDim)); content.add(fill);
  const grid=new THREE.GridHelper(maxDim*1.9, 24, 0x1e293b, 0x131c2e); grid.position.set(center.x, box.min.y, center.z); content.add(grid);

  const upY=new THREE.Vector3(0,1,0);
  for(const e of (S.elements||[])){
    if(!e || (e.kind==='node' ? !Array.isArray(e.at) : (!Array.isArray(e.from)||!Array.isArray(e.to)))) continue;
    const col=colorOf[e.group] || 0xffffff;
    // Opacity: per-element overrides per-group; <1 makes the material translucent so
    // elements embedded in others (e.g. rebar inside concrete) can be revealed (#258).
    const op = typeof e.opacity==='number' ? e.opacity : (typeof opacityOf[e.group]==='number' ? opacityOf[e.group] : 1);
    const mat=new THREE.MeshStandardMaterial({color:col, metalness:0.5, roughness:0.5, transparent:op<1, opacity:op});
    mat.userData={baseOpacity:op}; let mesh;
    if(e.kind==='node'){ const r=(e.size||maxDim*0.012); mesh=new THREE.Mesh(new THREE.SphereGeometry(r,20,16), mat); mesh.position.copy(conv(e.at,up)); }
    else { const a=conv(e.from,up), b=conv(e.to,up), dir=b.clone().sub(a), len=dir.length()||thick;
      const w=(e.section&&e.section.w)||thick, d=(e.section&&e.section.d)||thick;
      mesh=new THREE.Mesh(profileGeom(e,w,d,len), mat); mesh.position.copy(a).add(b).multiplyScalar(0.5);
      orientMember(mesh, dir); }
    mesh.userData=e; content.add(mesh); pickable.push(mesh);
  }
  for(const g of (S.grids||[])) if(g&&Array.isArray(g.at)) content.add(makeLabel(g.label, conv(g.at,up), maxDim));

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
  renderer.clippingPlanes=active.length?active:EMPTY_CLIPS; }
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
document.querySelectorAll('#proj button').forEach(b=>b.addEventListener('click',()=>setProjection(b.dataset.proj)));
document.querySelectorAll('#modes button').forEach(b=>b.addEventListener('click',()=>setDisplayMode(b.dataset.mode)));
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
const TRIAD_PX=72;
const triadRenderer=new THREE.WebGLRenderer({antialias:true, alpha:true});
triadRenderer.setPixelRatio(Math.min(devicePixelRatio,2)); triadRenderer.setSize(TRIAD_PX,TRIAD_PX);
document.getElementById('axestriad').appendChild(triadRenderer.domElement);
const triadScene=new THREE.Scene();
const triadCam=new THREE.OrthographicCamera(-1.85,1.85,1.85,-1.85,0.1,20); triadCam.position.set(0,0,5);
const triadGroup=new THREE.Group();
function triadTip(label,color,pos){ // letter on a colored disc — legible over any geometry behind it
  const c=document.createElement('canvas'); c.width=c.height=64; const g=c.getContext('2d');
  g.fillStyle=color; g.beginPath(); g.arc(32,32,30,0,Math.PI*2); g.fill();
  g.fillStyle='#fff'; g.font='bold 42px ui-sans-serif,system-ui,sans-serif'; g.textAlign='center'; g.textBaseline='middle'; g.fillText(label,32,34);
  const s=new THREE.Sprite(new THREE.SpriteMaterial({map:new THREE.CanvasTexture(c)})); s.position.copy(pos); s.scale.setScalar(0.95); return s; }
{ const up=(SCENE.meta&&SCENE.meta.up)||'z', AXIS_Y=new THREE.Vector3(0,1,0);
  for(const [label,color,axis] of [['X','#ef4444',[1,0,0]],['Y','#22c55e',[0,1,0]],['Z','#3b82f6',[0,0,1]]]){
    const d=conv(axis,up).normalize();
    const shaft=new THREE.Mesh(new THREE.CylinderGeometry(0.055,0.055,1.05,8), new THREE.MeshBasicMaterial({color}));
    shaft.quaternion.setFromUnitVectors(AXIS_Y,d); shaft.position.copy(d).multiplyScalar(0.525);
    triadGroup.add(shaft, triadTip(label,color,d.clone().multiplyScalar(1.38))); }
  triadGroup.add(new THREE.Mesh(new THREE.SphereGeometry(0.12,12,8), new THREE.MeshBasicMaterial({color:0xe2e8f0}))); // origin ball
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
    out.insert("html".into(), Value::String(html.clone()));
    out.insert("bytes".into(), Value::from(html.len() as u64));

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
        let out = viewer_3d_render(&json!({ "scene": scene }), true).unwrap();
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
        // Display modes: solid / wireframe / x-ray.
        assert!(html.contains("data-mode=\"wire\"") && html.contains("data-mode=\"xray\""));
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
