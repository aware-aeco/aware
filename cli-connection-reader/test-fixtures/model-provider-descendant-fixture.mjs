import { spawn } from 'node:child_process';

async function main() {
  if (process.argv[2] !== 'describe' || process.argv[3] !== '--json-stdin') process.exit(2);
  for await (const _chunk of process.stdin) { /* drain the bounded request */ }
  const child = spawn(process.env.COMSPEC, ['/d', '/s', '/c', 'ping -n 30 127.0.0.1 >nul'], {
    windowsHide: true, stdio: ['ignore', 'inherit', 'inherit'],
  });
  child.unref();
  process.stdout.write(JSON.stringify({
    protocolVersion: '1', provider: 'descendant-fixture', engine: 'fixture', engineVersion: '1',
    adapterBuildId: 'descendant-v1', formats: ['rvt'], execution: 'local', destination: null,
  }));
}

main().catch(() => { process.exitCode = 1; });
