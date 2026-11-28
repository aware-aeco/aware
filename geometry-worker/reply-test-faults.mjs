// Runs inside the same supervised Node process as the real dispatcher.
const { appendFileSync, existsSync } = await import('node:fs');
const { pathToFileURL } = await import('node:url');
const [dispatcher, route, fault, trace] = process.argv.slice(2);
const record = (event, fields = {}) => appendFileSync(trace, JSON.stringify({event, route, fault, ...fields}) + '\n');

if (fault === 'blocked-stdin') {
  record('stdin_blocked');
  await new Promise(() => setInterval(() => {}, 1000));
}
if (fault === 'held-open') {
  // Isolate the inherited output pipes from the blocked-stdin case.
  for await (const _chunk of process.stdin) { /* drain request */ }
  const { spawn } = await import('node:child_process');
  const ready = trace + '.ready';
  // Detached skips libuv's parent-owned job while keeping the supervisor's job.
  const child = spawn(process.execPath, [
    '-e',
    "process.stdout.write('held', () => require('node:fs').writeFileSync(process.argv[1], 'ready')); setInterval(() => {}, 1000)",
    ready,
  ], { detached: true, windowsHide: true, stdio: ['ignore', 'inherit', 'inherit'] });
  child.on('error', () => process.exit(6));
  setInterval(() => {
    if (existsSync(ready)) {
      record('descendant_holds_pipe', { pid: child.pid });
      process.exit(0);
    }
  }, 10);
  await new Promise(() => {});
}

const { cutMeshRequest, verifyExact } = await import(pathToFileURL(dispatcher));
const chunks = [];
for await (const chunk of process.stdin) chunks.push(chunk);
const input = JSON.parse(Buffer.concat(chunks).toString('utf8'));
const reply = route === 'exact' ? verifyExact(input) : await cutMeshRequest(input, () => record('mesh_boolean'));
if (fault === 'trace') {
  process.stdout.write(JSON.stringify(reply) + '\n');
  process.exitCode = 0;
} else {
  if (reply.status !== 'ok') throw new Error('fault requires an independently verified child result');
  writeFault();
}

function subdivide(mesh) {
  const points = mesh.positions_mm;
  const seen = new Map();
  const midpoint = (i, j) => {
    const key = i < j ? i + ',' + j : j + ',' + i;
    if (!seen.has(key)) {
      seen.set(key, points.length);
      points.push(points[i].map((value, axis) => (value + points[j][axis]) / 2));
    }
    return seen.get(key);
  };
  mesh.triangles = mesh.triangles.flatMap(([a, b, c]) => {
    const ab = midpoint(a, b), bc = midpoint(b, c), ca = midpoint(c, a);
    return [[a, ab, ca], [ab, b, bc], [ca, bc, c], [ab, bc, ca]];
  });
}

function writeFault() {
  if (fault === 'malformed') {
    record('reply_fault');
    process.stdout.write('{');
  } else if (fault === 'nonzero') {
    process.stdout.write(JSON.stringify(reply) + '\n', () => {
      record('valid_reply_written');
      process.exit(7);
    });
  } else if (fault === 'stdout' || fault === 'stderr') {
    record('overflow');
    const size = (fault === 'stdout' ? 32 * 1024 * 1024 : 64 * 1024) + 1;
    process[fault].write(Buffer.alloc(size, 32));
  } else {
    if (fault === 'unknown-field') reply.extra = true;
    if (fault === 'omitted-field') delete reply.relation;
    if (fault === 'envelope') {
      for (let i = 0; i < 5; i += 1) subdivide(reply.result.mesh);
    }
    const body = JSON.stringify(reply) + '\n';
    if (fault === 'envelope') record('envelope_payload', { child_bytes: Buffer.byteLength(body) });
    else record('reply_fault');
    process.stdout.write(body);
  }
}
