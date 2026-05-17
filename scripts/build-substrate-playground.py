"""
Build the AWARE substrate force-graph playground.

Walks 20-agents/, extracts each agent's metadata + command-name set,
computes pairwise "shared command names" edge weights, and emits a
self-contained HTML file with the data inlined + a vanilla-JS
force-directed simulation.

Run from repo root:

  python scripts/build-substrate-playground.py
"""

import json
from pathlib import Path

REPO = Path(__file__).resolve().parent.parent
AGENTS_ROOT = REPO / "20-agents"
OUTPUT = REPO / "40-diagrams" / "substrate-playground.html"

VENDOR_COLORS = {
    "trimble": "#0067a6",
    "autodesk": "#2e8b57",
    "mcneel": "#dc143c",
    "nemetschek": "#ff8c00",
    "idea-statica": "#9333ea",
    "creoox": "#06b6d4",
    "mrdoob": "#a855f7",
    "thatopen": "#22d3ee",
    "speckle": "#3b82f6",
    "bentley": "#ea580c",
    "computers-and-structures": "#84cc16",
    "slack": "#4a154b",
    "aware-aeco": "#0f172a",
    "unknown": "#6b7280",
}


def vertical_from_keywords(keywords):
    """Same heuristic as the Rust diagram regen."""
    for kw, v in [
        ("engineering", "Engineering"),
        ("architecture", "Architecture"),
        ("construction", "Construction"),
        ("visualization", "Visualization"),
        ("cross-cutting", "Cross-cutting"),
        ("operations", "Operations"),
    ]:
        if kw in keywords:
            return v
    if "meta" in keywords or "meta-primitive" in keywords:
        return "Meta"
    return "Other"


def parse_manifest(path):
    """Best-effort YAML parse pulling out only the fields we need."""
    text = path.read_text(encoding="utf-8")
    agent_id = None
    display_name = None
    vendor = None
    keywords = []
    commands = []
    in_commands_block = False
    in_keywords = False
    for line in text.splitlines():
        s = line.rstrip()
        ls = s.strip()
        if not in_commands_block:
            if s.startswith("agent:"):
                agent_id = s.split(":", 1)[1].strip()
            elif s.startswith("display-name:"):
                display_name = s.split(":", 1)[1].strip()
            elif s.startswith("vendor:"):
                vendor = s.split(":", 1)[1].strip()
            elif s.startswith("keywords:"):
                rest = s.split(":", 1)[1].strip()
                if rest.startswith("[") and rest.endswith("]"):
                    keywords = [
                        k.strip().strip(",").strip('"').strip("'")
                        for k in rest[1:-1].split(",")
                        if k.strip()
                    ]
                in_keywords = False
            elif s.startswith("commands:"):
                in_commands_block = True
        else:
            # We're under `commands:`. Each top-level command key starts at
            # column 2: `  cmdname:`. We only want the name.
            if s.startswith("  ") and not s.startswith("    ") and s.endswith(":"):
                name = s.strip().rstrip(":")
                if name and not name.startswith("#"):
                    commands.append(name)
            elif s and not s.startswith(" ") and not s.startswith("#"):
                # Left `commands:` block (back to top-level key).
                in_commands_block = False
    return {
        "id": agent_id,
        "displayName": display_name or agent_id,
        "vendor": (vendor or "unknown").lower(),
        "vertical": vertical_from_keywords(keywords),
        "commands": commands,
    }


def stem(cmd_name):
    """Reduce a kebab command name to its 'last verb' stem so that e.g.
    'viewer-load-model' shares with 'scene-load-model' on 'load-model'.
    Heuristic: take the last 2 hyphen-separated tokens.
    """
    parts = cmd_name.split("-")
    if len(parts) >= 2:
        return "-".join(parts[-2:])
    return cmd_name


def compute_edges(agents):
    """Pairwise shared-stem counts. Returns list of {source, target, weight}."""
    stem_sets = {a["id"]: set(stem(c) for c in a["commands"]) for a in agents}
    edges = []
    ids = sorted(stem_sets.keys())
    for i in range(len(ids)):
        for j in range(i + 1, len(ids)):
            a, b = ids[i], ids[j]
            common = stem_sets[a] & stem_sets[b]
            w = len(common)
            if w >= 3:  # cull low-signal edges; raise threshold to keep file small
                edges.append({"source": a, "target": b, "weight": w})
    return edges


def main():
    agents = []
    for manifest_path in AGENTS_ROOT.rglob("manifest.yaml"):
        parsed = parse_manifest(manifest_path)
        if not parsed["id"]:
            continue
        parsed["skills"] = sum(1 for _ in (manifest_path.parent / "skills").glob("*.md")) if (manifest_path.parent / "skills").is_dir() else 0
        parsed["commandCount"] = len(parsed["commands"])
        parsed["color"] = VENDOR_COLORS.get(parsed["vendor"], "#6b7280")
        agents.append(parsed)

    print(f"discovered {len(agents)} agents")
    edges = compute_edges(agents)
    print(f"computed {len(edges)} edges (weight >= 3)")

    # Strip commands list from payload — we only kept it for edge computation
    payload_agents = [
        {
            "id": a["id"],
            "displayName": a["displayName"],
            "vendor": a["vendor"],
            "vertical": a["vertical"],
            "skills": a["skills"],
            "commands": a["commandCount"],
            "color": a["color"],
        }
        for a in agents
    ]

    html = HTML_TEMPLATE.replace(
        "__AGENTS__", json.dumps(payload_agents)
    ).replace(
        "__EDGES__", json.dumps(edges)
    )
    OUTPUT.parent.mkdir(parents=True, exist_ok=True)
    OUTPUT.write_text(html, encoding="utf-8")
    print(f"wrote {OUTPUT} ({OUTPUT.stat().st_size / 1024:.1f} KB)")


HTML_TEMPLATE = r"""<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="utf-8" />
<title>AWARE substrate — force-directed graph playground</title>
<style>
:root {
  --bg: #0a0a0a;
  --panel: #141414;
  --border: #262626;
  --text: #e5e5e5;
  --muted: #71717a;
  --accent: #4ade80;
  --link: #2563eb;
}
* { box-sizing: border-box; }
html, body { margin: 0; padding: 0; height: 100%; background: var(--bg); color: var(--text); font: 13px/1.4 -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif; }
.app { display: grid; grid-template-columns: 280px 1fr; grid-template-rows: 1fr 200px; height: 100vh; gap: 1px; background: var(--border); }
.sidebar { background: var(--panel); padding: 16px; overflow-y: auto; grid-row: 1 / 3; }
.canvas-wrap { background: var(--bg); position: relative; overflow: hidden; }
canvas { display: block; }
.prompt-pane { background: var(--panel); padding: 12px 16px; grid-column: 2; overflow: auto; border-top: 1px solid var(--border); }
h1 { font-size: 14px; font-weight: 600; margin: 0 0 12px; letter-spacing: 0.02em; }
h2 { font-size: 11px; text-transform: uppercase; letter-spacing: 0.08em; color: var(--muted); margin: 16px 0 6px; }
label { display: block; font-size: 12px; margin: 8px 0 4px; color: var(--muted); }
input[type=range] { width: 100%; }
.row { display: flex; align-items: center; gap: 8px; margin-bottom: 4px; font-size: 12px; }
.row input[type=checkbox] { margin: 0; }
.row .swatch { width: 10px; height: 10px; border-radius: 50%; flex-shrink: 0; }
.row .label { flex: 1; }
.row .count { color: var(--muted); font-variant-numeric: tabular-nums; }
.presets { display: flex; flex-wrap: wrap; gap: 4px; margin-top: 6px; }
.presets button { background: var(--border); color: var(--text); border: 0; padding: 4px 8px; font-size: 11px; border-radius: 3px; cursor: pointer; }
.presets button:hover { background: #404040; }
.value { color: var(--accent); font-variant-numeric: tabular-nums; }
.tooltip { position: absolute; background: var(--panel); border: 1px solid var(--border); padding: 8px 10px; border-radius: 4px; font-size: 12px; pointer-events: none; display: none; max-width: 280px; box-shadow: 0 4px 12px rgba(0,0,0,0.4); }
.tooltip .t-title { font-weight: 600; color: var(--accent); margin-bottom: 4px; }
.tooltip .t-meta { color: var(--muted); font-size: 11px; }
.prompt-text { font-family: monospace; font-size: 12px; background: var(--bg); border: 1px solid var(--border); border-radius: 4px; padding: 10px; white-space: pre-wrap; word-break: break-word; min-height: 80px; }
.copy-btn { margin-top: 8px; background: var(--link); color: white; border: 0; padding: 6px 14px; font-size: 12px; border-radius: 3px; cursor: pointer; }
.copy-btn:hover { opacity: 0.9; }
.copy-btn.copied { background: var(--accent); color: #0a0a0a; }
.legend { position: absolute; bottom: 12px; left: 12px; background: rgba(20,20,20,0.9); border: 1px solid var(--border); border-radius: 4px; padding: 8px 10px; font-size: 11px; }
.stats { color: var(--muted); margin: 8px 0; font-size: 11px; }
</style>
</head>
<body>
<div class="app">
  <aside class="sidebar">
    <h1>AWARE substrate · force graph</h1>
    <div class="stats" id="stats"></div>

    <h2>Presets</h2>
    <div class="presets">
      <button data-preset="all">All</button>
      <button data-preset="bim">BIM stack</button>
      <button data-preset="cad">CAD stack</button>
      <button data-preset="viz">Visualization</button>
      <button data-preset="structural">Structural</button>
    </div>

    <h2>Edge threshold</h2>
    <label>min shared method stems: <span class="value" id="thr-v">5</span></label>
    <input id="thr" type="range" min="3" max="50" value="5" />

    <h2>Forces</h2>
    <label>repulsion: <span class="value" id="rep-v">1000</span></label>
    <input id="rep" type="range" min="100" max="4000" value="1000" />
    <label>edge stiffness: <span class="value" id="link-v">0.01</span></label>
    <input id="link" type="range" min="1" max="50" value="10" />

    <h2>Verticals</h2>
    <div id="verticals"></div>

    <h2>Vendors</h2>
    <div id="vendors"></div>
  </aside>

  <div class="canvas-wrap">
    <canvas id="cv"></canvas>
    <div id="tip" class="tooltip"></div>
    <div class="legend"><b>Node size</b> = skill count<br/><b>Edge weight</b> = shared command stems</div>
  </div>

  <div class="prompt-pane">
    <h2>Prompt — copy back to Claude</h2>
    <div class="prompt-text" id="prompt"></div>
    <button class="copy-btn" id="copy">Copy</button>
  </div>
</div>

<script>
const RAW_AGENTS = __AGENTS__;
const RAW_EDGES = __EDGES__;

// ---- Layout / state ----
const state = {
  threshold: 5,
  repulsion: 1000,
  linkStiffness: 0.01,
  verticals: {},
  vendors: {},
};

// Initialize filter state from data
new Set(RAW_AGENTS.map(a => a.vertical)).forEach(v => state.verticals[v] = true);
new Set(RAW_AGENTS.map(a => a.vendor)).forEach(v => state.vendors[v] = true);

const canvas = document.getElementById('cv');
const ctx = canvas.getContext('2d');
function fit() {
  const w = canvas.parentElement.clientWidth;
  const h = canvas.parentElement.clientHeight;
  canvas.width = w * devicePixelRatio;
  canvas.height = h * devicePixelRatio;
  canvas.style.width = w + 'px';
  canvas.style.height = h + 'px';
  ctx.setTransform(devicePixelRatio, 0, 0, devicePixelRatio, 0, 0);
}
fit();
window.addEventListener('resize', fit);

// Position store
const positions = new Map();
function W() { return canvas.width / devicePixelRatio; }
function H() { return canvas.height / devicePixelRatio; }
RAW_AGENTS.forEach((a, i) => {
  const angle = (i / RAW_AGENTS.length) * Math.PI * 2;
  positions.set(a.id, {
    x: W()/2 + Math.cos(angle) * 200,
    y: H()/2 + Math.sin(angle) * 200,
    vx: 0, vy: 0,
  });
});

// ---- Filtering ----
function visibleAgents() {
  return RAW_AGENTS.filter(a => state.verticals[a.vertical] && state.vendors[a.vendor]);
}
function visibleEdges(agents) {
  const ids = new Set(agents.map(a => a.id));
  return RAW_EDGES.filter(e => ids.has(e.source) && ids.has(e.target) && e.weight >= state.threshold);
}

// ---- Force simulation ----
function step() {
  const agents = visibleAgents();
  const edges = visibleEdges(agents);
  const cx = W()/2, cy = H()/2;

  // Repulsion (only between visible nodes — O(n^2) but n is small)
  for (let i = 0; i < agents.length; i++) {
    const a = positions.get(agents[i].id);
    for (let j = i+1; j < agents.length; j++) {
      const b = positions.get(agents[j].id);
      let dx = a.x - b.x, dy = a.y - b.y;
      const d2 = Math.max(100, dx*dx + dy*dy);
      const force = state.repulsion / d2;
      const d = Math.sqrt(d2);
      const fx = (dx/d) * force, fy = (dy/d) * force;
      a.vx += fx; a.vy += fy;
      b.vx -= fx; b.vy -= fy;
    }
    // Centering force
    a.vx += (cx - a.x) * 0.001;
    a.vy += (cy - a.y) * 0.001;
  }
  // Edge attraction
  for (const e of edges) {
    const a = positions.get(e.source);
    const b = positions.get(e.target);
    if (!a || !b) continue;
    const dx = b.x - a.x, dy = b.y - a.y;
    const d = Math.sqrt(dx*dx + dy*dy) || 1;
    const target = 120 + Math.max(0, 80 - e.weight);
    const force = state.linkStiffness * (d - target) * Math.log(1 + e.weight);
    const fx = (dx/d) * force, fy = (dy/d) * force;
    a.vx += fx; a.vy += fy;
    b.vx -= fx; b.vy -= fy;
  }
  // Integrate + damp
  for (const a of agents) {
    const p = positions.get(a.id);
    if (p.dragging) continue;
    p.vx *= 0.85; p.vy *= 0.85;
    p.x += p.vx; p.y += p.vy;
    // Keep within bounds
    p.x = Math.max(40, Math.min(W()-40, p.x));
    p.y = Math.max(40, Math.min(H()-40, p.y));
  }
}

function nodeRadius(a) {
  return 5 + Math.sqrt(a.skills || 1) * 1.2;
}

function draw() {
  ctx.clearRect(0, 0, W(), H());
  const agents = visibleAgents();
  const edges = visibleEdges(agents);

  // Edges
  for (const e of edges) {
    const a = positions.get(e.source);
    const b = positions.get(e.target);
    if (!a || !b) continue;
    const opacity = Math.min(0.7, 0.1 + e.weight / 100);
    ctx.strokeStyle = `rgba(96,165,250,${opacity})`;
    ctx.lineWidth = Math.min(3, 0.3 + Math.log(1 + e.weight) / 2);
    ctx.beginPath();
    ctx.moveTo(a.x, a.y);
    ctx.lineTo(b.x, b.y);
    ctx.stroke();
  }
  // Nodes
  for (const a of agents) {
    const p = positions.get(a.id);
    const r = nodeRadius(a);
    ctx.beginPath();
    ctx.arc(p.x, p.y, r, 0, Math.PI*2);
    ctx.fillStyle = a.color;
    ctx.fill();
    ctx.strokeStyle = '#0a0a0a';
    ctx.lineWidth = 2;
    ctx.stroke();
    // Label
    ctx.fillStyle = '#e5e5e5';
    ctx.font = '11px monospace';
    ctx.textAlign = 'center';
    ctx.textBaseline = 'top';
    ctx.fillText(a.id, p.x, p.y + r + 4);
  }
}

let animFrame;
function loop() {
  step();
  draw();
  animFrame = requestAnimationFrame(loop);
}
loop();

// ---- Interaction ----
const tip = document.getElementById('tip');
canvas.addEventListener('mousemove', (e) => {
  const rect = canvas.getBoundingClientRect();
  const mx = e.clientX - rect.left;
  const my = e.clientY - rect.top;
  let hit = null;
  for (const a of visibleAgents()) {
    const p = positions.get(a.id);
    const dx = p.x - mx, dy = p.y - my;
    if (dx*dx + dy*dy <= nodeRadius(a) * nodeRadius(a)) { hit = a; break; }
  }
  if (drag.id) {
    const p = positions.get(drag.id);
    p.x = mx; p.y = my; p.vx = 0; p.vy = 0;
    p.dragging = true;
  }
  if (hit) {
    tip.style.display = 'block';
    tip.style.left = (mx + 16) + 'px';
    tip.style.top = (my + 16) + 'px';
    tip.innerHTML = `<div class="t-title">${hit.displayName}</div>
      <div class="t-meta">${hit.id} · ${hit.vendor} · ${hit.vertical}</div>
      <div class="t-meta">${hit.skills} skills · ${hit.commands} cmds</div>`;
  } else {
    tip.style.display = 'none';
  }
});
const drag = { id: null };
canvas.addEventListener('mousedown', (e) => {
  const rect = canvas.getBoundingClientRect();
  const mx = e.clientX - rect.left;
  const my = e.clientY - rect.top;
  for (const a of visibleAgents()) {
    const p = positions.get(a.id);
    const dx = p.x - mx, dy = p.y - my;
    if (dx*dx + dy*dy <= nodeRadius(a) * nodeRadius(a)) {
      drag.id = a.id;
      break;
    }
  }
});
canvas.addEventListener('mouseup', () => {
  if (drag.id) {
    const p = positions.get(drag.id);
    p.dragging = false;
  }
  drag.id = null;
});

// ---- Controls ----
function updateRange(id, key, fmt) {
  const el = document.getElementById(id);
  const out = document.getElementById(id + '-v');
  el.addEventListener('input', () => {
    state[key] = parseFloat(el.value) * (key === 'linkStiffness' ? 0.001 : 1);
    out.textContent = fmt ? fmt(state[key]) : el.value;
    updatePrompt();
  });
}
updateRange('thr', 'threshold');
updateRange('rep', 'repulsion');
updateRange('link', 'linkStiffness', v => v.toFixed(3));

function buildFilterList(containerId, items, stateKey) {
  const c = document.getElementById(containerId);
  c.innerHTML = '';
  items.forEach(({key, label, color, count}) => {
    const row = document.createElement('label');
    row.className = 'row';
    row.innerHTML = `<input type="checkbox" ${state[stateKey][key] ? 'checked' : ''}>
      ${color ? `<span class="swatch" style="background:${color}"></span>` : ''}
      <span class="label">${label}</span>
      <span class="count">${count}</span>`;
    row.querySelector('input').addEventListener('change', (e) => {
      state[stateKey][key] = e.target.checked;
      updatePrompt();
    });
    c.appendChild(row);
  });
}

function rebuildFilters() {
  const vertCounts = {};
  const vendorCounts = {};
  RAW_AGENTS.forEach(a => {
    vertCounts[a.vertical] = (vertCounts[a.vertical] || 0) + 1;
    vendorCounts[a.vendor] = (vendorCounts[a.vendor] || 0) + 1;
  });
  buildFilterList('verticals',
    Object.entries(vertCounts).sort().map(([k, c]) => ({key: k, label: k, count: c})),
    'verticals');
  buildFilterList('vendors',
    Object.entries(vendorCounts).sort().map(([k, c]) => {
      const a = RAW_AGENTS.find(a => a.vendor === k);
      return {key: k, label: k, color: a?.color, count: c};
    }),
    'vendors');
}
rebuildFilters();

// ---- Presets ----
function applyPreset(name) {
  const allVerts = Object.keys(state.verticals);
  const allVendors = Object.keys(state.vendors);
  allVerts.forEach(v => state.verticals[v] = true);
  allVendors.forEach(v => state.vendors[v] = true);

  if (name === 'bim') {
    state.threshold = 10;
    document.getElementById('thr').value = 10;
    document.getElementById('thr-v').textContent = 10;
  } else if (name === 'cad') {
    Object.keys(state.verticals).forEach(v => state.verticals[v] = (v === 'Architecture'));
  } else if (name === 'viz') {
    Object.keys(state.verticals).forEach(v => state.verticals[v] = (v === 'Visualization'));
  } else if (name === 'structural') {
    Object.keys(state.verticals).forEach(v => state.verticals[v] = (v === 'Engineering'));
  }
  rebuildFilters();
  updatePrompt();
}
document.querySelectorAll('[data-preset]').forEach(btn => {
  btn.addEventListener('click', () => applyPreset(btn.dataset.preset));
});

// ---- Stats ----
function updateStats() {
  const agents = visibleAgents();
  const edges = visibleEdges(agents);
  const totalCmds = agents.reduce((a, b) => a + b.commands, 0);
  document.getElementById('stats').innerHTML =
    `${agents.length}/${RAW_AGENTS.length} agents · ${edges.length} edges · ${totalCmds.toLocaleString()} cmds in view`;
}

// ---- Prompt output ----
function updatePrompt() {
  updateStats();
  const visibleVerts = Object.entries(state.verticals).filter(([_, v]) => v).map(([k]) => k);
  const visibleVendors = Object.entries(state.vendors).filter(([_, v]) => v).map(([k]) => k);
  const allVerts = visibleVerts.length === Object.keys(state.verticals).length;
  const allVendors = visibleVendors.length === Object.keys(state.vendors).length;
  const parts = [];
  if (!allVerts) parts.push(`only the ${visibleVerts.join(' + ')} vertical${visibleVerts.length > 1 ? 's' : ''}`);
  if (!allVendors) parts.push(`only vendor${visibleVendors.length > 1 ? 's' : ''} ${visibleVendors.join(' + ')}`);
  if (state.threshold !== 5) parts.push(`edge threshold ${state.threshold} shared method-stems`);

  let text;
  if (parts.length === 0) {
    text = `Show me how the AWARE substrate's 39 agents connect across all 5 verticals. Default threshold (5 shared command-name stems).`;
  } else {
    text = `Show me the AWARE substrate force-graph view restricted to ${parts.join('; ')}. Highlight which agents are tightly coupled by shared API surface — those are the candidates for cross-vendor composition in an aware app.`;
  }
  document.getElementById('prompt').textContent = text;
}
updatePrompt();

// ---- Copy ----
document.getElementById('copy').addEventListener('click', async () => {
  const btn = document.getElementById('copy');
  await navigator.clipboard.writeText(document.getElementById('prompt').textContent);
  btn.textContent = 'Copied!';
  btn.classList.add('copied');
  setTimeout(() => { btn.textContent = 'Copy'; btn.classList.remove('copied'); }, 1500);
});
</script>
</body>
</html>
"""


if __name__ == "__main__":
    main()
