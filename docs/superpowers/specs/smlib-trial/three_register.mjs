// Lets Node run FloLess' own vendored three.js + three-bvh-csg (the exact files the browser loads) by mapping the
// bare specifiers of web/'s import map. Read-only use of web/vendor; nothing is written there.
import { register } from 'node:module';
import { pathToFileURL, fileURLToPath } from 'node:url';
import path from 'node:path';

const here = path.dirname(fileURLToPath(import.meta.url));
const vendor = path.resolve(here, '../../../../web/vendor');
globalThis.__VENDOR__ = vendor;
const hook = `
export async function resolve(spec, ctx, next) {
  const v = ${JSON.stringify(pathToFileURL(vendor).href)};
  if (spec === 'three') return { url: v + '/three.module.js', shortCircuit: true };
  if (spec === 'three-bvh-csg') return { url: v + '/three-bvh-csg.module.js', shortCircuit: true };
  return next(spec, ctx);
}`;
register('data:text/javascript,' + encodeURIComponent(hook));
