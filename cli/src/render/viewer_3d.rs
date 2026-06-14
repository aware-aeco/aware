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
  #legend{bottom:16px;left:16px;padding:12px 14px;font-size:12.5px} #legend .row{display:flex;align-items:center;gap:8px;margin:4px 0}
  #readout{bottom:16px;left:50%;transform:translateX(-50%);padding:10px 16px;font-size:13px;color:var(--muted);white-space:nowrap;max-width:60vw;overflow:hidden;text-overflow:ellipsis}
  #readout b{color:var(--text)} #readout .pill{color:var(--accent)}
</style>
</head>
<body>
<div id="app"></div>
<div id="topbar" class="panel"><div class="brand"><b>AWARE</b> · viewer-3d</div><div class="sub" id="sceneName">—</div></div>
<div id="side" class="panel"><h2 id="sideTitle">—</h2><p class="note" id="sideNote"></p><div id="panels"></div></div>
<div id="legend" class="panel"></div>
<div id="readout" class="panel">Drag to orbit · scroll to zoom · <b>click an element</b> to inspect</div>
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
const camera=new THREE.PerspectiveCamera(50, innerWidth/innerHeight, 0.01, 1e7);
const renderer=new THREE.WebGLRenderer({antialias:true}); renderer.setPixelRatio(Math.min(devicePixelRatio,2)); renderer.setSize(innerWidth,innerHeight);
document.getElementById('app').appendChild(renderer.domElement);
const controls=new OrbitControls(camera, renderer.domElement); controls.enableDamping=true; controls.dampingFactor=0.08;
let content=new THREE.Group(); scene.add(content); let pickable=[];
const conv=(P,up)=> up==='z' ? new THREE.Vector3(P[0],P[2],P[1]) : new THREE.Vector3(P[0],P[1],P[2]);

function clearContent(){ scene.remove(content);
  content.traverse(o=>{ if(o.geometry)o.geometry.dispose(); if(o.material)o.material.dispose(); });
  content=new THREE.Group(); scene.add(content); pickable=[]; }

function makeLabel(text,pos,maxDim){
  const c=document.createElement('canvas'); c.width=128; c.height=64; const g=c.getContext('2d');
  g.fillStyle='#60a5fa'; g.font='bold 40px ui-sans-serif,system-ui,sans-serif'; g.textAlign='center'; g.textBaseline='middle'; g.fillText(text,64,34);
  const sp=new THREE.Sprite(new THREE.SpriteMaterial({map:new THREE.CanvasTexture(c),transparent:true}));
  sp.scale.set(maxDim*0.09, maxDim*0.045, 1); sp.position.copy(pos); return sp;
}

function renderScene(S){
  clearContent();
  const up=(S.meta&&S.meta.up)||'z';
  const colorOf={}; (S.groups||[]).forEach(g=>colorOf[g.key]=g.color);
  const box=new THREE.Box3();
  for(const e of (S.elements||[])){ if(e.from)box.expandByPoint(conv(e.from,up)); if(e.to)box.expandByPoint(conv(e.to,up)); if(e.at)box.expandByPoint(conv(e.at,up)); }
  if(box.isEmpty()) box.set(new THREE.Vector3(-1,-1,-1), new THREE.Vector3(1,1,1));
  const size=box.getSize(new THREE.Vector3()), center=box.getCenter(new THREE.Vector3());
  const maxDim=Math.max(size.x,size.y,size.z)||1; const thick=maxDim*0.006;

  content.add(new THREE.HemisphereLight(0x9fc5ff,0x0a0f1a,0.95));
  const key=new THREE.DirectionalLight(0xffffff,1.3); key.position.copy(center).add(new THREE.Vector3(maxDim,maxDim*1.5,maxDim*0.6)); content.add(key);
  const fill=new THREE.DirectionalLight(0x88aaff,0.5); fill.position.copy(center).add(new THREE.Vector3(-maxDim,maxDim*0.7,-maxDim)); content.add(fill);
  const grid=new THREE.GridHelper(maxDim*1.9, 24, 0x1e293b, 0x131c2e); grid.position.set(center.x, box.min.y, center.z); content.add(grid);

  const upY=new THREE.Vector3(0,1,0);
  for(const e of (S.elements||[])){
    const col=colorOf[e.group] || 0xffffff;
    const mat=new THREE.MeshStandardMaterial({color:col, metalness:0.5, roughness:0.5}); let mesh;
    if(e.kind==='node'){ const r=(e.size||maxDim*0.012); mesh=new THREE.Mesh(new THREE.SphereGeometry(r,20,16), mat); mesh.position.copy(conv(e.at,up)); }
    else { const a=conv(e.from,up), b=conv(e.to,up), dir=b.clone().sub(a), len=dir.length()||thick;
      const w=(e.section&&e.section.w)||thick, d=(e.section&&e.section.d)||thick;
      mesh=new THREE.Mesh(new THREE.BoxGeometry(w,len,d), mat); mesh.position.copy(a).add(b).multiplyScalar(0.5);
      mesh.quaternion.setFromUnitVectors(upY, dir.normalize()); }
    mesh.userData=e; content.add(mesh); pickable.push(mesh);
  }
  for(const g of (S.grids||[])) content.add(makeLabel(g.label, conv(g.at,up), maxDim));

  if(S.camera&&S.camera.eye&&S.camera.target){ camera.position.set(...S.camera.eye); controls.target.set(...S.camera.target); }
  else { const dist=maxDim*1.7; camera.position.copy(center).add(new THREE.Vector3(dist,dist*0.8,dist)); controls.target.copy(center); }
  camera.near=maxDim/500; camera.far=maxDim*40; camera.updateProjectionMatrix();

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
  (S.groups||[]).forEach(g=>{ const row=el('div','row'); const sw=el('span','swatch'); sw.style.background=g.color; row.append(sw, document.createTextNode(g.label)); host.append(row); }); }

const ray=new THREE.Raycaster(), mouse=new THREE.Vector2(); let selected=null; const readout=document.getElementById('readout');
function setHint(){ readout.replaceChildren(document.createTextNode('Drag to orbit · scroll to zoom · '), el('b',null,'click an element'), document.createTextNode(' to inspect')); }
renderer.domElement.addEventListener('pointerdown', e=>{
  mouse.x=(e.clientX/innerWidth)*2-1; mouse.y=-(e.clientY/innerHeight)*2+1; ray.setFromCamera(mouse,camera);
  const hit=ray.intersectObjects(pickable,false)[0];
  if(selected){ selected.material.emissive.setHex(0x000000); selected=null; }
  if(hit){ selected=hit.object; selected.material.emissive=new THREE.Color(0xf59e0b); selected.material.emissiveIntensity=0.6;
    const u=selected.userData; const parts=[el('b',null,u.id||'(element)')]; if(u.group) parts.push(document.createTextNode(' · '), el('span','pill',u.group));
    for(const [k,v] of Object.entries(u.meta||{})) parts.push(document.createTextNode(` · ${k}: ${v}`));
    readout.replaceChildren(...parts);
  } else setHint();
});

addEventListener('resize',()=>{ camera.aspect=innerWidth/innerHeight; camera.updateProjectionMatrix(); renderer.setSize(innerWidth,innerHeight); });
(function loop(){ requestAnimationFrame(loop); controls.update(); renderer.render(scene,camera); })();

renderScene(SCENE);
window.__viewerReady=true; if(window.__viewerPost) window.__viewerPost('viewer-ready');
window.__viewer3d={ count:()=>pickable.length, name:()=>(SCENE.meta&&SCENE.meta.name)||'' };
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

    // Serialize the scene and inject it into the renderer shell. Escape `</` so a string value
    // containing `</script>` can't close the embedding <script> element (the only XSS-shaped
    // break-out for JSON embedded in a script context).
    let scene_json = serde_json::to_string(scene)
        .map_err(|e| AwareError::Internal(format!("viewer-3d: serialize scene: {e}")))?
        .replace("</", "<\\/");
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
    fn escapes_script_close_in_string_values() {
        let scene = json!({ "meta": { "name": "x" }, "elements": [],
            "panels": [ { "title": "</script><script>alert(1)", "columns": [], "rows": [] } ] });
        let out = viewer_3d_render(&json!({ "scene": scene }), true).unwrap();
        let html = out["html"].as_str().unwrap();
        // the literal closing tag must not survive inside the embedded data
        assert!(!html.contains("</script><script>alert(1)"));
        assert!(html.contains("<\\/script>"));
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
