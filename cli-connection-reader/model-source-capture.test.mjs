import assert from 'node:assert/strict';
import fs from 'node:fs/promises';
import os from 'node:os';
import path from 'node:path';
import test from 'node:test';

import { canonicalJsonBytes, sha256 } from './model-contract.mjs';
import { captureSourceNamespaces } from './model-source-capture.mjs';

async function temporary(t) {
  const root = await fs.mkdtemp(path.join(await fs.realpath(os.tmpdir()), 'aware-source-capture-'));
  t.after(() => fs.rm(root, { recursive: true, force: true }));
  return root;
}

test('capture produces deterministic provider-visible receipts without original paths', async (t) => {
  const root = await temporary(t);
  const model = path.join(root, 'original-model'); const catalogue = path.join(root, 'original-catalogue');
  await fs.mkdir(path.join(model, 'nested'), { recursive: true }); await fs.mkdir(catalogue);
  await fs.writeFile(path.join(model, 'z.db2'), 'db2');
  await fs.writeFile(path.join(model, 'nested', 'a.db1'), 'db1');
  await fs.writeFile(path.join(catalogue, 'profiles.bin'), 'profiles');
  const output = await captureSourceNamespaces([
    { id: 'model', root: model }, { id: 'catalogue', root: catalogue },
  ], path.join(root, 'stage'));
  assert.deepEqual(output.manifest, {
    schemaVersion: 'aware.model-source-capture/v1',
    namespaces: [
      { namespaceId: 'model', files: [
        { path: 'nested/a.db1', bytes: 3, sha256: sha256(Buffer.from('db1')) },
        { path: 'z.db2', bytes: 3, sha256: sha256(Buffer.from('db2')) },
      ] },
      { namespaceId: 'catalogue', files: [
        { path: 'profiles.bin', bytes: 8, sha256: sha256(Buffer.from('profiles')) },
      ] },
    ],
  });
  assert.equal(output.manifestSha256, sha256(canonicalJsonBytes(output.manifest)));
  assert.equal(JSON.stringify(output.manifest).includes(root), false);
  assert.equal(await fs.readFile(path.join(root, 'stage', 'namespaces', 'model', 'nested', 'a.db1'), 'utf8'), 'db1');
  const repeated = await captureSourceNamespaces([
    { id: 'model', root: model }, { id: 'catalogue', root: catalogue },
  ], path.join(root, 'stage-repeated'));
  assert.deepEqual(repeated.manifest, output.manifest);
  assert.equal(repeated.manifestSha256, output.manifestSha256);
});

test('capture rejects links and removes its incomplete staging directory', async (t) => {
  const root = await temporary(t); const source = path.join(root, 'source');
  const outside = path.join(root, 'outside');
  await fs.mkdir(source); await fs.mkdir(outside); await fs.writeFile(path.join(outside, 'model.db1'), 'outside');
  const link = path.join(source, 'linked');
  await fs.symlink(outside, link, process.platform === 'win32' ? 'junction' : 'dir');
  const stage = path.join(root, 'stage');
  await assert.rejects(
    () => captureSourceNamespaces([{ id: 'model', root: source }], stage),
    (error) => error.code === 'reference-source-link',
  );
  await assert.rejects(() => fs.access(stage));
});

test('capture rejects namespace IDs that are unsafe as portable directory names', async (t) => {
  const root = await temporary(t); const source = path.join(root, 'source');
  await fs.mkdir(source); await fs.writeFile(path.join(source, 'model.db1'), 'db1');
  for (const id of ['CON', 'com1.txt', '.', '..', 'model.']) {
    await assert.rejects(
      () => captureSourceNamespaces([{ id, root: source }], path.join(root, `stage-${id.replaceAll('.', '_')}`)),
      (error) => error.code === 'reference-source-namespace-invalid',
    );
  }
});

test('capture rejects a directory swapped for a link during enumeration', async (t) => {
  const root = await temporary(t); const source = path.join(root, 'source');
  const original = path.join(root, 'source-original'); const outside = path.join(root, 'outside');
  await fs.mkdir(source); await fs.mkdir(outside);
  await fs.writeFile(path.join(source, 'inside.db1'), 'inside');
  await fs.writeFile(path.join(outside, 'outside.db1'), 'outside');
  let swapped = false;
  const readDirectory = async (directory, options) => {
    if (!swapped) {
      swapped = true;
      await fs.rename(source, original);
      await fs.symlink(outside, source, process.platform === 'win32' ? 'junction' : 'dir');
    }
    return fs.readdir(directory, options);
  };
  const stage = path.join(root, 'stage');
  await assert.rejects(
    () => captureSourceNamespaces([{ id: 'model', root: source }], stage, { readDirectory }),
    (error) => error.code === 'reference-source-changed',
  );
  await assert.rejects(() => fs.access(stage));
});

test('capture rejects case-colliding directory paths', { skip: process.platform === 'win32' }, async (t) => {
  const root = await temporary(t); const source = path.join(root, 'source');
  await fs.mkdir(path.join(source, 'Foo'), { recursive: true });
  await fs.mkdir(path.join(source, 'foo'), { recursive: true });
  await fs.writeFile(path.join(source, 'Foo', 'a.db1'), 'a');
  await fs.writeFile(path.join(source, 'foo', 'b.db1'), 'b');
  const stage = path.join(root, 'stage');
  await assert.rejects(
    () => captureSourceNamespaces([{ id: 'model', root: source }], stage),
    (error) => error.code === 'reference-source-path-collision',
  );
  await assert.rejects(() => fs.access(stage));
});

test('capture bounds directories even when they contain no files', async (t) => {
  const root = await temporary(t); const source = path.join(root, 'source');
  await fs.mkdir(path.join(source, 'a', 'b'), { recursive: true });
  const stage = path.join(root, 'stage');
  await assert.rejects(
    () => captureSourceNamespaces([{ id: 'model', root: source }], stage, { limits: { maxDirectories: 2 } }),
    (error) => error.code === 'reference-source-package-too-large',
  );
  await assert.rejects(() => fs.access(stage));
});

test('capture rejects staging paths that overlap a source namespace', async (t) => {
  const root = await temporary(t); const source = path.join(root, 'source');
  await fs.mkdir(source); await fs.writeFile(path.join(source, 'model.db1'), 'db1');
  for (const stage of [source, path.join(source, 'stage'), root]) {
    await assert.rejects(
      () => captureSourceNamespaces([{ id: 'model', root: source }], stage),
      (error) => error.code === 'reference-source-staging-invalid',
    );
  }
});

test('capture schema declares the portable path format and rejects unsafe lexical paths', async () => {
  const schema = JSON.parse(await fs.readFile(new URL('./model-source-capture-v1.schema.json', import.meta.url), 'utf8'));
  assert.equal(schema.$defs.path.format, 'aware-portable-relative-path-v1');
  const pattern = new RegExp(schema.$defs.path.pattern, 'u');
  for (const value of ['CON/file', 'dir//file', 'dir/file.', '../file', `${'a'.repeat(256)}/file`]) {
    assert.equal(pattern.test(value), false, value);
  }
  assert.equal(pattern.test('katalog/żółć.db1'), true);
});

test('capture enforces file, aggregate and path bounds before publishing a manifest', async (t) => {
  const root = await temporary(t); const source = path.join(root, 'source');
  await fs.mkdir(source); await fs.writeFile(path.join(source, 'a.db1'), '1234');
  await fs.writeFile(path.join(source, 'b.db1'), '5678');
  for (const [limits, code] of [
    [{ maxFileBytes: 3 }, 'reference-source-file-too-large'],
    [{ maxAggregateBytes: 3 }, 'reference-source-package-too-large'],
    [{ maxFiles: 1 }, 'reference-source-package-too-large'],
    [{ maxPathBytes: 4 }, 'reference-source-path-invalid'],
  ]) {
    const stage = path.join(root, `stage-${code}-${Object.keys(limits)[0]}`);
    await assert.rejects(
      () => captureSourceNamespaces([{ id: 'model', root: source }], stage, { limits }),
      (error) => error.code === code,
    );
    await assert.rejects(() => fs.access(stage));
  }
});

test('capture detects a same-size mutation between copy and the second inventory', async (t) => {
  const root = await temporary(t); const source = path.join(root, 'source');
  await fs.mkdir(source); const sourceFile = path.join(source, 'model.db1');
  await fs.writeFile(sourceFile, 'before');
  let opens = 0;
  const openSource = async (pathname, flags) => {
    opens += 1;
    if (opens === 2) await fs.writeFile(pathname, 'after!');
    return fs.open(pathname, flags);
  };
  const stage = path.join(root, 'stage');
  await assert.rejects(
    () => captureSourceNamespaces([{ id: 'model', root: source }], stage, { openSource }),
    (error) => error.code === 'reference-source-changed',
  );
  await assert.rejects(() => fs.access(stage));
});

test('capture verifies the opened file identity before reading bytes', async (t) => {
  const root = await temporary(t); const source = path.join(root, 'source');
  await fs.mkdir(source); await fs.writeFile(path.join(source, 'model.db1'), 'inside');
  const outside = path.join(root, 'outside.db1'); await fs.writeFile(outside, 'outside');
  let substituted = false;
  const openSource = async (pathname, flags) => {
    if (!substituted) {
      substituted = true;
      return fs.open(outside, flags);
    }
    return fs.open(pathname, flags);
  };
  const stage = path.join(root, 'stage');
  await assert.rejects(
    () => captureSourceNamespaces([{ id: 'model', root: source }], stage, { openSource }),
    (error) => error.code === 'reference-source-changed',
  );
  await assert.rejects(() => fs.access(stage));
});

test('capture respects cancellation before reading source bytes', async (t) => {
  const root = await temporary(t); const source = path.join(root, 'source');
  await fs.mkdir(source); await fs.writeFile(path.join(source, 'model.db1'), 'db1');
  const controller = new AbortController(); controller.abort();
  const stage = path.join(root, 'stage');
  await assert.rejects(
    () => captureSourceNamespaces([{ id: 'model', root: source }], stage, { signal: controller.signal }),
    (error) => error.code === 'reference-source-cancelled',
  );
  await assert.rejects(() => fs.access(stage));
});
