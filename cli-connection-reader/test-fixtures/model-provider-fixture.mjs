import fs from 'node:fs/promises';
import path from 'node:path';
import { makeGlbFixture, makeMetadataFixture } from '../model-fixtures.mjs';

const provenance = {
  protocolVersion: '1', provider: 'fixture-provider', engine: 'fixture-engine',
  engineVersion: '1.2.3', adapterBuildId: 'fixture-build', formats: ['rvt'],
  execution: 'local', destination: null,
};

async function main() {
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
    const geometryOptions = request.canonicalRequest?.conversionSettings?.fixtureUnclaimedOffset
      ? {
          scenes: [{ nodes: [0, 1] }],
          nodes: [
            { name: 'part-a', mesh: 0 },
            { name: 'unclaimed-offset', mesh: 0, translation: [10, 0, 0] },
          ],
        }
      : {};
    await fs.writeFile(geometryPath, makeGlbFixture(geometryOptions));
    const metadata = makeMetadataFixture();
    if (request.canonicalRequest?.readerSchemaVersion === 'model-reference-reader/v2') {
      metadata.schemaVersion = '2';
      metadata.parameterGroups[0].id = '1';
      metadata.parameters = [{
        id: '1', name: 'Display Mark', unit: null,
        valueEncoding: 'provider-display', valueType: 'string', value: 'A-1',
      }];
      delete metadata.elements[0].ifcGuid;
    }
    await fs.writeFile(metadataPath, JSON.stringify(metadata));
    process.stdout.write(JSON.stringify({
      ...provenance, documentKind: 'revit-project', sourceSha256: request.sourceSha256,
      geometryPath, metadataPath,
    }));
  } else {
    process.stderr.write('unsupported fixture operation');
    process.exitCode = 2;
  }
}

main().catch((error) => { process.stderr.write(`fixture provider: ${error instanceof Error ? error.message : String(error)}\n`); process.exitCode = 1; });
