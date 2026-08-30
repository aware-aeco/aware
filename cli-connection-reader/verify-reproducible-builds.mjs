#!/usr/bin/env node
import { createHash } from 'node:crypto';
import { lstatSync, readFileSync, readdirSync, realpathSync } from 'node:fs';
import { join, relative, resolve, sep } from 'node:path';
import { fileURLToPath } from 'node:url';

const portable = (path) => path.split(sep).join('/');
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');

function files(root, path = root) {
  const stat = lstatSync(path);
  if (stat.isSymbolicLink()) throw new Error(`reproducibility output contains symbolic link: ${path}`);
  if (stat.isFile()) return [path];
  if (!stat.isDirectory()) throw new Error(`unsupported reproducibility output entry: ${path}`);
  return readdirSync(path, { withFileTypes: true })
    .flatMap((entry) => files(root, join(path, entry.name)));
}

export function inventory(root) {
  const absolute = resolve(root);
  return files(absolute).map((path) => ({
    path: portable(relative(absolute, path)),
    size: lstatSync(path).size,
    sha256: sha256(readFileSync(path)),
  })).sort((a, b) => Buffer.compare(Buffer.from(a.path), Buffer.from(b.path)));
}

export function forbiddenEncodings(root) {
  const variants = new Set();
  const add = (value) => {
    if (!value) return;
    variants.add(value); variants.add(value.toLowerCase());
    variants.add(value.replaceAll('\\', '/')); variants.add(value.replaceAll('\\', '/').toLowerCase());
    variants.add(JSON.stringify(value).slice(1, -1));
    try { variants.add(new URL(`file:///${value.replaceAll('\\', '/')}`).href); } catch { /* invalid path is still checked raw */ }
  };
  add(resolve(root));
  return [...variants].flatMap((value) => [
    { label: value, bytes: Buffer.from(value, 'utf8') },
    { label: `${value} (UTF-16LE)`, bytes: Buffer.from(value, 'utf16le') },
  ]).filter((entry) => entry.bytes.length);
}

export function scanForbiddenRoots(outputRoot, forbiddenRoots) {
  const needles = forbiddenRoots.flatMap(forbiddenEncodings);
  const hits = [];
  for (const path of files(resolve(outputRoot))) {
    const bytes = readFileSync(path);
    const lowerUtf8 = Buffer.from(bytes.toString('utf8').toLowerCase(), 'utf8');
    const lowerUtf16 = Buffer.from(bytes.toString('utf16le').toLowerCase(), 'utf16le');
    for (const needle of needles) {
      if (bytes.includes(needle.bytes) || lowerUtf8.includes(needle.bytes) || lowerUtf16.includes(needle.bytes)) {
        hits.push({ path: portable(relative(resolve(outputRoot), path)), rootEncoding: needle.label });
      }
    }
  }
  return hits;
}

export function verifyReproducibleOutputs({ left, right, forbiddenRoots = [] }) {
  const leftInventory = inventory(left); const rightInventory = inventory(right);
  if (JSON.stringify(leftInventory) !== JSON.stringify(rightInventory)) {
    throw new Error(`builder outputs differ:\nleft=${JSON.stringify(leftInventory, null, 2)}\nright=${JSON.stringify(rightInventory, null, 2)}`);
  }
  const hits = [...scanForbiddenRoots(left, forbiddenRoots), ...scanForbiddenRoots(right, forbiddenRoots)];
  if (hits.length) throw new Error(`builder root leaked into compared artifact: ${JSON.stringify(hits)}`);
  return leftInventory;
}

function parseArgs(argv) {
  const out = { forbiddenRoots: [] };
  for (let index = 0; index < argv.length; index += 1) {
    const arg = argv[index]; const value = argv[index + 1];
    if (arg === '--forbid-root' && value != null) { out.forbiddenRoots.push(value); index += 1; continue; }
    if ((arg === '--left' || arg === '--right') && value != null) { out[arg.slice(2)] = value; index += 1; continue; }
    throw new Error('arguments are --left DIR --right DIR [--forbid-root DIR ...]');
  }
  if (!out.left || !out.right) throw new Error('arguments are --left DIR --right DIR [--forbid-root DIR ...]');
  return out;
}

if (realpathSync(fileURLToPath(import.meta.url)) === realpathSync(process.argv[1])) {
  const result = verifyReproducibleOutputs(parseArgs(process.argv.slice(2)));
  console.log(`reproducible output proof passed: ${result.length} byte-identical files`);
}
