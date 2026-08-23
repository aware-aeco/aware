import { appendFileSync, readFileSync } from 'node:fs';
import { isSea } from 'node:sea';
import { pathToFileURL } from 'node:url';
import { lowerableLimits, ModelReaderError, safeErrorEnvelope } from './model-contract.mjs';
import { runModelCommand } from './model-reader.mjs';

function route(command, args) {
  const hasIfc = ['ifc-path', 'ifcPath', 'base-ifc-path', 'revised-ifc-path'].some((key) => args[key] !== undefined);
  const hasRvt = ['rvt-path', 'rvtPath', 'model-path'].some((key) => args[key] !== undefined);
  if (hasIfc && hasRvt) throw new ModelReaderError('reference-mixed-model-paths', 'request', false, 'IFC and RVT paths cannot be mixed in one command.');
  if (command === 'preflight') return 'rvt';
  if ((command === 'probe' || command === 'read-model') && hasRvt) return 'rvt';
  return 'ifc';
}

export function serializeModelResult(result, limits = undefined) {
  const text = JSON.stringify(result);
  if (Buffer.byteLength(text) > lowerableLimits(limits).maxCommandResponseBytes) {
    throw new ModelReaderError('reference-response-too-large', 'response', false, 'The bounded model-reader response exceeds its byte limit.');
  }
  return text;
}

export async function main(command = process.argv[2], stdinText = undefined, dependencies = undefined) {
  const raw = stdinText ?? readFileSync(0, 'utf8');
  let args;
  try { args = JSON.parse(raw || '{}'); }
  catch (error) { throw new ModelReaderError('reference-request-invalid', 'request', false, 'Command input is not valid JSON.', error); }
  if (!args || typeof args !== 'object' || Array.isArray(args)) throw new ModelReaderError('reference-request-invalid', 'request', false, 'Command input must be a JSON object.');
  if (route(command, args) === 'ifc') {
    const ifc = await import('./index.mjs');
    return await ifc.main(command, raw);
  }
  const runtimeDependencies = dependencies ?? {
    progress: process.env.AWARE_PROGRESS_FILE ? (record) => {
      appendFileSync(process.env.AWARE_PROGRESS_FILE, `${JSON.stringify({ '$aware-progress': record })}\n`, { encoding: 'utf8' });
    } : undefined,
  };
  const result = await runModelCommand(command, args, runtimeDependencies);
  process.stdout.write(serializeModelResult(result, dependencies?.limits));
}

const invokedDirectly = isSea() || (!!process.argv[1] && import.meta.url === pathToFileURL(process.argv[1]).href);
if (invokedDirectly) {
  main().catch((error) => {
    if (error instanceof ModelReaderError) process.stderr.write(`${JSON.stringify(safeErrorEnvelope(error))}\n`);
    else process.stderr.write(`connection-reader: ${error instanceof Error ? error.message : String(error)}\n`);
    process.exitCode = 1;
  });
}
