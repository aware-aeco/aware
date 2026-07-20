#!/usr/bin/env node
// Browser gate for viewer-3d's clip editing. ONE entry point: it parse-checks the template, renders
// fixtures through the WORKTREE build, serves them, drives Playwright, cleans up, and exits nonzero
// on any failed assertion — so "the gate passed" is distinguishable from clicking around by hand.
//
// This is a pre-PR gate run on demand, NOT CI: this repo has no Rust CI job at all, declares no
// Playwright dependency, and the template loads Three.js from a CDN (so the run needs network).
//
// Usage:  node cli/tests/browser/run.mjs

import { execFileSync, spawnSync } from 'node:child_process';
import { createServer } from 'node:http';
import { mkdtempSync, rmSync, writeFileSync, readFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join, dirname, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

const HERE = dirname(fileURLToPath(import.meta.url));
const CLI = resolve(HERE, '../..');           // cli/
const ROOT = resolve(CLI, '..');              // repo root
const MANIFEST = join(CLI, 'Cargo.toml');

let failures = 0;
const ok = (name, cond, detail = '') => {
  if (cond) { console.log(`  PASS  ${name}`); return; }
  failures++; console.log(`  FAIL  ${name}${detail ? ` — ${detail}` : ''}`);
};
const near = (a, b, tol = 0.5) => Math.abs(a - b) <= tol;

// ---- step 0: the template must be syntactically valid JS -------------------------------------
// The Rust tests are all string-contains, so a syntax error in the embedded module would still
// "render" and pass every one of them. This catches it in milliseconds, with no browser.
function parseCheckTemplate() {
  const src = readFileSync(join(CLI, 'src/render/viewer_3d.rs'), 'utf8');
  const t0 = src.indexOf('const TEMPLATE'), t1 = src.indexOf('"##;', t0);
  const tpl = src.slice(t0, t1);
  const a = tpl.indexOf('<script type="module">'), b = tpl.indexOf('</script>', a);
  const js = tpl.slice(a + '<script type="module">'.length, b)
    .replace(/^\s*import[^;]+;\s*$/gm, '')
    .replace('__SCENE_JSON__', '{}');
  // `new Function` here COMPILES but never runs the body, and the input is this repo's own source
  // file — anyone able to change it already has code execution. It is a parse check, not an eval of
  // anything external; the alternative is adding a JS parser dependency to a Rust repo.
  try { new Function(js); ok('template module script parses', true); }
  catch (e) { ok('template module script parses', false, e.message); }
}

// ---- fixtures: rendered by the WORKTREE build, never a globally installed release --------------
// `aware agent invoke …` off PATH resolves whatever CLI and agent happen to be installed, which
// would report green while never executing the change under test.
function renderFixture(scene) {
  const home = mkdtempSync(join(tmpdir(), 'aware-browser-'));
  try {
    const env = { ...process.env, AWARE_HOME: home };
    execFileSync('cargo', ['run', '--quiet', '--manifest-path', MANIFEST, '--',
      'agent', 'install', join(ROOT, '20-agents/_core/viewer-3d')], { env, stdio: 'pipe' });
    const inputs = join(home, 'inputs.json');
    writeFileSync(inputs, JSON.stringify({ scene }));
    const out = execFileSync('cargo', ['run', '--quiet', '--manifest-path', MANIFEST, '--',
      'agent', 'invoke', 'viewer-3d', 'render', '--inputs', `@${inputs}`, '--json'],
      { env, stdio: 'pipe', maxBuffer: 64 * 1024 * 1024 }).toString();
    const parsed = JSON.parse(out);
    if (!parsed.data || !parsed.data.html) throw new Error('render produced no html');
    return parsed.data.html;
  } finally { rmSync(home, { recursive: true, force: true }); }
}

// A frame with a grid, authored so the SAME model can be emitted z-up or y-up. Reference systems are
// plan-space either way (that is the contract), which is exactly what the y-up case checks.
const frameScene = (up) => {
  const P = (x, y, z) => (up === 'z' ? [x, z, y] : [x, y, z]);   // author in world terms, emit in scene terms
  return {
    meta: { name: `frame-${up}`, units: 'mm', up },
    groups: [{ key: 'g', label: 'G', color: '#60a5fa' }],
    elements: [
      { id: 'C1', kind: 'member', group: 'g', from: P(0, 0, 0), to: P(0, 4000, 0), section: { w: 200, d: 200 }, meta: { profile: 'HSS' } },
      { id: 'C2', kind: 'member', group: 'g', from: P(6000, 0, 0), to: P(6000, 4000, 0), section: { w: 200, d: 200 }, meta: { profile: 'HSS' } },
      { id: 'B1', kind: 'member', group: 'g', from: P(0, 4000, 0), to: P(6000, 4000, 0), section: { w: 150, d: 400 }, meta: { profile: 'W' } },
      { id: 'B2', kind: 'member', group: 'g', from: P(3000, 4000, -2000), to: P(3000, 4000, 2000), section: { w: 150, d: 300 }, meta: { profile: 'W' } },
    ],
    referenceSystems: [{
      id: 'GRID-1', kind: 'structural-grid', name: 'G', origin: [0, 0, 0],
      bounds: { minX: -500, maxX: 6500, minY: -2500, maxY: 2500 },
      axes: [{ id: 'GA-A', direction: 'x', offsetMm: 0, label: 'A' },
             { id: 'GA-B', direction: 'x', offsetMm: 6000, label: 'B' },
             { id: 'GA-1', direction: 'y', offsetMm: 0, label: '1' }],
      levels: [{ id: 'GL-1', elevationMm: 4000, label: 'ROOF' }],
    }],
  };
};

function serve(pages) {
  const server = createServer((req, res) => {
    const body = pages[req.url];
    if (!body) { res.writeHead(404); res.end('no'); return; }
    res.writeHead(200, { 'content-type': 'text/html; charset=utf-8' }); res.end(body);
  });
  return new Promise((r) => server.listen(0, '127.0.0.1', () => r(server)));
}

// ---- Playwright, acquired explicitly ----------------------------------------------------------
// The repo declares no Playwright dependency, so without this the gate dies before asserting
// anything — on the one machine where that matters. Fail with the install command, not a stack.
const PW_VERSION = '1.49.1';
async function loadPlaywright() {
  try { return (await import('playwright')).chromium; }
  catch { /* fall through to the pinned local install */ }
  const probe = spawnSync('npx', ['--yes', `playwright@${PW_VERSION}`, '--version'],
    { stdio: 'pipe', shell: process.platform === 'win32' });
  if (probe.status !== 0) {
    console.error(`\nSETUP: Playwright is not available.\n  npm i -D playwright@${PW_VERSION} && npx playwright@${PW_VERSION} install chromium\n`);
    process.exit(2);
  }
  try { return (await import('playwright')).chromium; }
  catch (e) {
    console.error(`\nSETUP: Playwright resolved but could not be imported: ${e.message}\n  npm i -D playwright@${PW_VERSION}\n`);
    process.exit(2);
  }
}

(async () => {
  console.log('viewer-3d clip browser gate\n');
  console.log('template:');
  parseCheckTemplate();

  console.log('\nfixtures (rendered by the worktree build in a temp AWARE_HOME):');
  const pages = { '/zup.html': renderFixture(frameScene('z')), '/yup.html': renderFixture(frameScene('y')) };
  ok('z-up fixture rendered', pages['/zup.html'].length > 10000);
  ok('y-up fixture rendered', pages['/yup.html'].length > 10000);

  const chromium = await loadPlaywright();
  const server = await serve(pages);
  const base = `http://127.0.0.1:${server.address().port}`;
  const browser = await chromium.launch();

  const open = async (path) => {
    const page = await browser.newPage({ viewport: { width: 1280, height: 860 } });
    const errors = [];
    page.on('console', (m) => { if (m.type() === 'error') errors.push(m.text()); });
    page.on('pageerror', (e) => errors.push(String(e)));
    await page.goto(`${base}${path}`);
    await page.waitForFunction('window.__viewer3d && window.__viewer3d.count() > 0', null, { timeout: 20000 });
    return { page, errors };
  };

  try {
    // ---- the coordinate invariant, in both directions ------------------------------------------
    for (const up of ['z', 'y']) {
      console.log(`\n${up}-up scene:`);
      const { page, errors } = await open(`/${up}up.html`);
      const grids = await page.evaluate('window.__viewer3d.referenceSystemSegments()');
      const level = grids[0].levels[0];
      const ys = level.segments.flat().map((p) => p[1]);
      ok(`${up}-up: a grid level is horizontal`, ys.every((y) => near(y, ys[0])), `world Y = ${ys.join(', ')}`);
      ok(`${up}-up: the level sits at its elevation`, near(level.y, 4000) && near(ys[0], 4000), `y=${level.y}`);
      // A level that is horizontal must span the two PLAN axes and not the vertical one.
      const spans = level.segments.map((s) => [s[1][0] - s[0][0], s[1][1] - s[0][1], s[1][2] - s[0][2]]);
      ok(`${up}-up: level spans plan axes only`, spans.every((d) => near(d[1], 0)), JSON.stringify(spans));
      ok(`${up}-up: console clean`, errors.length === 0, errors.join(' | '));
      await page.close();
    }

    // ---- clip editing --------------------------------------------------------------------------
    console.log('\nclip editing (z-up):');
    const { page, errors } = await open('/zup.html');
    const V = (js) => page.evaluate(js);

    await V('window.__viewer3d.addClipBox(0)');
    let clips = await V('window.__viewer3d.getClips()');
    ok('a box clip keeps its source Box3', clips.length === 1 && clips[0].box !== null);
    ok('a new clip is auto-selected', clips[0].selected === true);
    const before = clips[0].box.max[1];

    // Disabling stops it cutting without deleting it.
    await V(`window.__viewer3d.toggleClip('${clips[0].id}', false)`);
    ok('disabled clip contributes no planes', (await V('window.__viewer3d.clipPlanes()')) === 0);
    ok('disabled clip still exists', (await V('window.__viewer3d.getClips()')).length === 1);
    await V(`window.__viewer3d.toggleClip('${clips[0].id}', true)`);
    ok('re-enabled clip cuts again', (await V('window.__viewer3d.clipPlanes()')) === 6);

    // Rename rejects blank and case-insensitive duplicates.
    ok('rename accepts a new name', (await V(`window.__viewer3d.renameClip('${clips[0].id}','Roof cut')`)) === true);
    ok('rename rejects blank', (await V(`window.__viewer3d.renameClip('${clips[0].id}','   ')`)) === false);
    await V('window.__viewer3d.addClipBox(0)');
    const two = await V('window.__viewer3d.getClips()');
    ok('rename rejects a case-insensitive duplicate',
      (await V(`window.__viewer3d.renameClip('${two[1].id}','ROOF CUT')`)) === false);

    // The panel renders a row per clip, and every control is a real button.
    ok('the clip panel is visible', await page.isVisible('#clips.show'));
    ok('one row per clip', (await page.locator('#clips .crow').count()) === 2);
    ok('row controls are buttons',
      (await page.locator('#clips .crow button.cvis').count()) === 2 &&
      (await page.locator('#clips .crow button.cren').count()) === 2 &&
      (await page.locator('#clips .crow button.cdel').count()) === 2);
    ok('the enable control exposes checked state',
      (await page.locator('#clips .crow .cvis').first().getAttribute('aria-checked')) !== null);

    // Dragging a face handle moves that face and only that face.
    await V(`window.__viewer3d.setSelectedClips(['${two[0].id}'])`);
    const handles = await V('window.__viewer3d.clipHandlesScreen()');
    const topArrow = handles.find((h) => h.axis === 'y' && h.sign === 1 && h.arrow && !h.behind);
    ok('a +Y face handle is projected on screen', !!topArrow);
    if (topArrow) {
      await page.mouse.move(topArrow.x, topArrow.y);
      await page.mouse.down();
      await page.mouse.move(topArrow.x, topArrow.y - 120, { steps: 12 });
      await page.mouse.up();
      const after = (await V('window.__viewer3d.getClips()'))[0].box;
      ok('the dragged face moved', !near(after.max[1], before, 5), `max.y ${before} → ${after.max[1]}`);
      ok('the opposite face did not', near(after.min[1], clips[0].box.min[1], 1));
    }

    // Delete removes only the selected clip.
    await V('window.__viewer3d.deleteSelectedClips()');
    ok('delete removes the selected clip', (await V('window.__viewer3d.getClips()')).length === 1);
    await V('window.__viewer3d.clearClips()');

    // ---- the plane-patch basis guard -----------------------------------------------------------
    // Its failure mode is a VERTICAL cut and nothing else, so both orientations must be exercised.
    console.log('\nplane-patch basis (the y-up guard):');
    for (const [label, n] of [['horizontal cut (normal +Y)', [0, 1, 0]], ['vertical cut (normal +X)', [1, 0, 0]],
                              ['vertical cut (normal +Z)', [0, 0, 1]], ['oblique', [0.6, 0.2, -0.77]]]) {
      const c = await V(`window.__viewer3d.planePatchCorners([0,0,0],[${n.join(',')}])`);
      const finite = c.flat().every(Number.isFinite);
      // Every corner must lie IN the plane (dot with the normal ≈ 0) and the patch must be square.
      const inPlane = c.every((p) => near(p[0] * n[0] + p[1] * n[1] + p[2] * n[2], 0, 1e-6));
      const d = (a, b) => Math.hypot(a[0] - b[0], a[1] - b[1], a[2] - b[2]);
      const sides = [d(c[0], c[1]), d(c[1], c[2]), d(c[2], c[3]), d(c[3], c[0])];
      ok(`${label}: corners finite`, finite);
      ok(`${label}: corners coplanar with the normal`, inPlane);
      ok(`${label}: patch is square, 2×CLIP_PATCH_R`, sides.every((s) => near(s, 609.6, 0.5)), sides.map((s) => s.toFixed(1)).join(','));
    }

    // ---- the 3-click draw ----------------------------------------------------------------------
    console.log('\nclip-box draw:');
    await V("window.__viewer3d.setClipMode('box')");
    ok('the draw arms', (await V('window.__viewer3d.clipMode()')) === 'box');
    const floorY = await V('window.__viewer3d.clipDrawFloorY()');
    ok('the draw floor is the model floor, not a grid datum', near(floorY, 0, 1), `floorY=${floorY}`);
    // Two corners far apart in plan, then a height. Screen positions come from the viewer's own
    // projection so the test does not re-derive it.
    const s1 = await V('window.__viewer3d.worldToScreen([-1000,0,-1000])');
    const s2 = await V('window.__viewer3d.worldToScreen([7000,0,1000])');
    await page.mouse.click(s1.x, s1.y);
    ok('first corner accepted', (await V('window.__viewer3d.gestureState()')).draft === 'footprint');
    // A second corner sharing a plan coordinate is a zero-width footprint and must be refused.
    const sameRow = await V('window.__viewer3d.worldToScreen([-1000,0,-1000])');
    await page.mouse.click(sameRow.x, sameRow.y);
    ok('a zero-extent second corner is refused', (await V('window.__viewer3d.gestureState()')).draft === 'footprint');
    await page.mouse.click(s2.x, s2.y);
    ok('second corner accepted → height stage', (await V('window.__viewer3d.gestureState()')).draft === 'height');
    const top = await V('window.__viewer3d.worldToScreen([3000,5000,0])');
    await page.mouse.move(top.x, top.y);
    await page.mouse.click(top.x, top.y);
    const drawn = await V('window.__viewer3d.getClips()');
    ok('the draw committed a box', drawn.length === 1 && drawn[0].box !== null);
    if (drawn.length === 1 && drawn[0].box) {
      ok('the box starts at the floor', near(drawn[0].box.min[1], floorY, 2), `min.y=${drawn[0].box.min[1]}`);
      ok('the box has real extent on all three axes',
        drawn[0].box.max[0] - drawn[0].box.min[0] > 1 &&
        drawn[0].box.max[1] - drawn[0].box.min[1] > 1 &&
        drawn[0].box.max[2] - drawn[0].box.min[2] > 1);
    }
    ok('committing disarms the draw', (await V('window.__viewer3d.clipMode()')) === null);

    ok('console clean through clip editing', errors.length === 0, errors.join(' | '));
    await page.close();

    // ---- regression: a scene with NO descriptor must still render -------------------------------
    // v0.97.0 dropped the `groups` binding in buildLegend, so the legacy list threw out of
    // renderScene and killed the whole module. That path is also the documented fallback for a
    // REJECTED descriptor, so a producer bug in the panel took the viewer down with it.
    console.log('\nlegacy legend (no descriptor):');
    const legacy = await open('/zup.html');
    ok('the viewer initialises without a legend descriptor', (await legacy.page.evaluate('window.__viewer3d.count()')) > 0);
    ok('the legacy legend renders its group rows', (await legacy.page.locator('#legend .row').count()) > 0);
    ok('no console errors on the legacy path', legacy.errors.length === 0, legacy.errors.join(' | '));
    await legacy.page.close();
  } finally {
    await browser.close();
    server.close();
  }

  console.log(`\n${failures === 0 ? 'GATE PASSED' : `GATE FAILED — ${failures} assertion(s)`}`);
  process.exit(failures === 0 ? 0 : 1);
})().catch((e) => { console.error('\nGATE ERROR:', e); process.exit(1); });
