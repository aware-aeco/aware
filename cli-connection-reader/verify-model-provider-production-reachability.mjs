import fs from 'node:fs/promises';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const root = path.dirname(fileURLToPath(import.meta.url));
const inventory = JSON.parse(await fs.readFile(path.join(root, 'model-provider-production-inventory.json')));
if (inventory.schemaVersion !== 'aware.model-provider-production-inventory/v1') {
  throw new Error('model provider production inventory schema is unsupported');
}
const imports = /(?:import\s+(?:[^'";]+?\s+from\s+)?|import\s*\()(['"])(\.\/[A-Za-z0-9._/-]+)\1/g;
const reachable = new Set();
async function visit(relative) {
  const normalized = relative.replaceAll('\\', '/');
  if (reachable.has(normalized)) return;
  reachable.add(normalized);
  const source = await fs.readFile(path.join(root, normalized), 'utf8');
  for (const match of source.matchAll(imports)) {
    const target = path.posix.normalize(path.posix.join(path.posix.dirname(normalized), match[2]));
    if (target.endsWith('.mjs')) await visit(target);
  }
}
for (const entrypoint of inventory.entrypoints) await visit(entrypoint);
const unreachable = inventory.runtimeModules.filter((relative) => !reachable.has(relative));
if (unreachable.length) throw new Error(`production-unreachable model provider modules: ${unreachable.join(', ')}`);
for (const relative of [...inventory.runtimeModules, ...inventory.entrypoints]) {
  const source = await fs.readFile(path.join(root, relative), 'utf8');
  for (const forbidden of inventory.forbiddenRuntimeIdentifiers) {
    if (source.includes(forbidden)) throw new Error(`forbidden stale identifier '${forbidden}' in ${relative}`);
  }
}
const reader = await fs.readFile(path.join(root, 'model-reader.mjs'), 'utf8');
for (const command of inventory.requiredCommands) {
  if (!reader.includes(`'${command}'`)) throw new Error(`production reader does not carry command ${command}`);
}
for (const schema of inventory.requiredSchemas) await fs.access(path.join(root, schema));
process.stdout.write(`verified ${inventory.runtimeModules.length} production model-provider modules\n`);
