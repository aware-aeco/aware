# viewer-3d — the scene schema

`viewer-3d.render` takes one input, `scene`: a **domain-agnostic** description of a 3D scene.
The renderer knows nothing about any vertical — a producer (a steel app, a data app, …) maps
its data into this shape.

```jsonc
{
  "meta":   { "name": "string", "units": "mm|u|…", "up": "z|y" },

  // legend + per-element colour, keyed by `group`
  "groups": [ { "key": "column", "label": "Columns", "color": "#60a5fa" } ],

  "elements": [
    {
      "id":   "COL-A1",                 // unique; shown on click
      "group":"column",                 // → colour + legend (optional)
      "kind": "line | box | node",
      "from": [x,y,z], "to": [x,y,z],   // line/box: the member axis a→b
      "at":   [x,y,z],                  // node: a point
      "section": { "w": 310, "d": 310 },// optional cross-section (units of meta.units)
      "size": 120,                      // optional node radius
      "meta": { "profile": "UC 305x305x97", "length": "6.00 m" }  // arbitrary; shown on click
    }
  ],

  "grids":  [ { "label": "A", "at": [x,y,z] } ],          // optional ground labels
  "panels": [ { "title": "Takeoff", "note": "…",          // optional generic side tables
                "columns": ["Section","No.","Length","Weight"],
                "rows": [ ["UC 305x305x97", 10, "60.0 m", "5.81 t"] ] } ],
  "camera": { "eye": [x,y,z], "target": [x,y,z] }         // optional; else auto-fit
}
```

## Rules

- **No domain knowledge in the renderer.** It draws `elements`, colours by `group`, lists
  `groups` as a legend, renders `panels` as side tables, labels `grids`, and shows `meta` on
  click. Domain smarts (e.g. steel profiles → tonnage) are computed by the **producer** and
  passed as generic `groups` + `panels`.
- **Coordinates are native** (`meta.units` is informational). `up:"z"` maps a structural Z to
  screen-up; `up:"y"` is passthrough. The camera/grid/lights **auto-fit** the element bounding
  box, so a 20 m building and a unitless bar chart both frame correctly with no tuning.
- **`kind`:** `line`/`box` = an oriented bar `from`→`to` (optional `section` for thickness,
  else a hairline relative to scene size); `node` = a sphere at `at` (optional `size`).
- **`camera`** is honoured when present, else auto-fit.

## Output

`{ html, bytes, output-path? }`. `html` is a self-contained interactive document (Three.js from
a pinned CDN for v1; full-inline offline is a planned follow-on). It needs scripts enabled, so a
client embeds it in a script-enabled surface or opens it in a browser — it is **not** a static
no-scripts report.
