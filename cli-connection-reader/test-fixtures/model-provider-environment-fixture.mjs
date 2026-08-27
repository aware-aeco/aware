import fs from 'node:fs/promises';
import path from 'node:path';
import { makeGlbFixture, makeMetadataFixture } from '../model-fixtures.mjs';

const expectedEnvironment = {
  COMSPEC: 'C:\\Windows\\System32\\cmd.exe',
  LANG: 'C',
  LC_ALL: 'C',
  SYSTEMROOT: 'C:\\Windows',
  TEMP: 'C:\\Windows\\Temp',
  TMP: 'C:\\Windows\\Temp',
  TZ: 'UTC',
  WINDIR: 'C:\\Windows',
};
const provenance = {
  protocolVersion: '1', provider: 'fixture-provider', engine: 'fixture-engine',
  engineVersion: '1.2.3', adapterBuildId: 'fixture-environment-build', formats: ['rvt'],
  execution: 'local', destination: null,
};

function assertExactEnvironment() {
  const actual = Object.fromEntries(Object.entries(process.env).sort(([left], [right]) => left.localeCompare(right)));
  const expected = Object.fromEntries(Object.entries(expectedEnvironment).sort(([left], [right]) => left.localeCompare(right)));
  if (JSON.stringify(actual) !== JSON.stringify(expected)) throw new Error('provider environment is not the exact closed canonical map');
}

async function main() {
  assertExactEnvironment();
  const operation = process.argv[2];
  const request = JSON.parse(await new Promise((resolve, reject) => {
    const chunks = [];
    process.stdin.on('data', (chunk) => chunks.push(chunk));
    process.stdin.on('end', () => resolve(Buffer.concat(chunks).toString('utf8')));
    process.stdin.on('error', reject);
  }));
  if (operation === 'describe') {
    process.stdout.write(JSON.stringify(provenance));
  } else if (operation === 'convert') {
    const geometryPath = path.join(request.outputDirectory, 'geometry.glb');
    const metadataPath = path.join(request.outputDirectory, 'metadata.json');
    await fs.writeFile(geometryPath, makeGlbFixture());
    await fs.writeFile(metadataPath, JSON.stringify(makeMetadataFixture()));
    process.stdout.write(JSON.stringify({
      ...provenance, documentKind: 'revit-project', sourceSha256: request.sourceSha256,
      geometryPath, metadataPath,
    }));
  } else {
    process.exitCode = 2;
  }
}

main().catch(() => { process.stderr.write('fixture provider failed its closed-environment assertion\n'); process.exitCode = 1; });
