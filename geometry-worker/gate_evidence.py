"""Run and validate Windows M1 evidence from version-controlled case expectations."""

import argparse
import hashlib
import json
import os
from pathlib import Path
import re
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[1]
WORKER = ROOT / 'geometry-worker'
READER = ROOT / 'cli-connection-reader'
LOCK = WORKER / 'win-amd64-cp312.requirements.lock'
ABI = 'windows-x64'
PARTS = ('node', 'dispatcher', 'slabs', 'adapter', 'wasm', 'support')


def cases():
    return json.loads((WORKER / 'gate-cases.json').read_bytes())


def sha(path):
    with path.open('rb') as stream:
        return hashlib.file_digest(stream, 'sha256').hexdigest()


def digest_object(value):
    return hashlib.sha256(json.dumps(value, sort_keys=True, separators=(',', ':')).encode()).hexdigest()


def check(condition, message):
    if not condition:
        raise ValueError(message)


def raw_reference():
    fixture = json.loads((WORKER / 'fixtures/flush-stress.json').read_bytes())
    return {'status': ['NoError'] * 3, 'reference_mm3': fixture['retained_mm3'],
            'member_volume_mm3': fixture['raw_member_volume_mm3'], 'failed_invariant': 'analytic_volume'}


def validate(report, commit, lock):
    check(isinstance(report, dict), 'report must be an object')
    check(report.get('commit') == commit and report.get('wheel_lock') == lock and report.get('abi') == ABI,
          'stale, missing or mismatched commit, wheel lock or ABI')
    check(report.get('raw_baseline') == raw_reference(), 'raw Manifold baseline is missing or changed')
    artifacts = report.get('artifacts', {})
    check(set(artifacts) == {'normal', 'fault', 'release'}, 'expected normal, fault and release artifact identities')
    for role, artifact in artifacts.items():
        fields = {'cli'} if role == 'release' else {'cli', 'worker', 'fault_enabled', *PARTS}
        check(set(artifact) == fields | {'identity'}, f'{role} artifact fields differ')
        for field in fields - {'fault_enabled'} | {'identity'}:
            check(isinstance(artifact[field], str) and re.fullmatch('[0-9a-f]{64}', artifact[field]),
                  f'bad {role} {field} digest')
        if role == 'release':
            check(artifact['identity'] == artifact['cli'], 'release identity does not match binary')
        else:
            check(artifact['fault_enabled'] is (role == 'fault'), f'wrong feature state for {role}')
            check(artifact['identity'] == digest_object({k: artifact[k] for k in fields}), f'wrong {role} identity')
    normal, fault = artifacts['normal'], artifacts['fault']
    check(all(normal[k] == fault[k] for k in PARTS), 'normal and fault helper bytes differ')
    check(all(normal[k] != fault[k] for k in ('cli', 'worker', 'identity')), 'fault artifact must be distinct')
    check(artifacts['release']['cli'] not in {normal['cli'], fault['cli']}, 'release binary must be distinct')
    expected = {(suite, case): (role, outcome) for suite, case, _, role, outcome in cases()}
    check(len(expected) == len(cases()) and expected, 'gate manifest is empty or has duplicate cases')
    observed = report.get('cases', [])
    check(isinstance(observed, list) and len(observed) == len(expected), 'missing, extra or zero cases')
    for row in observed:
        key = row.get('suite'), row.get('case')
        check(key in expected, f'unknown or duplicate case {key}')
        role, outcome = expected.pop(key)
        roles = ('normal', 'fault') if role == 'dual' else (role,)
        check(row.get('artifacts') == {r: artifacts[r]['identity'] for r in roles}, f'wrong artifact for {key}')
        check(row.get('outcome') == outcome and type(row.get('tests')) is int and row['tests'] == 1,
              f'wrong outcome or test count for {key}')


def capture(command, cwd=ROOT, env=None, timeout=450):
    run = subprocess.run([str(arg) for arg in command], cwd=cwd, env=env, capture_output=True,
                         text=True, encoding='utf-8', errors='replace', timeout=timeout)
    check(run.returncode == 0, f'{command[:4]} failed ({run.returncode})\n{run.stdout}\n{run.stderr}')
    return run.stdout, run.stderr


def artifact(stage, role, lock):
    cli = stage / 'aware.exe'
    cli_sha = sha(cli)
    if role == 'release':
        return {'identity': cli_sha, 'cli': cli_sha}
    helper = stage / 'geometry-mesh'
    stdout, stderr = capture([cli, 'geometry', 'artifact-identity', '--json'])
    check(not stderr, f'{role} artifact identity wrote stderr')
    data = json.loads(stdout)['data']
    check(data['abi'] == ABI and data['wheel_lock_sha256'] == lock, f'{role} embedded ABI or lock differs')
    check(data['fault_enabled'] is (role == 'fault'), f'{role} embedded feature set differs')
    names = {'node': 'node.exe', 'dispatcher': 'geometry-mesh-dispatcher.mjs',
             'slabs': 'geometry-slabs.mjs', 'adapter': 'mesh-solid.mjs',
             'wasm': 'node_modules/manifold-3d/manifold.wasm'}
    support = ('snapshot-clash.mjs', 'snapshot-clash-geometry.mjs', 'snapshot-compare.mjs', 'manifold-wasm.mjs',
               'node_modules/manifold-3d/manifold.js', 'node_modules/manifold-3d/package.json')
    fields = {'cli': cli_sha, 'worker': data['worker_sha256'], 'fault_enabled': data['fault_enabled'],
              **{name: sha(helper / path) for name, path in names.items()},
              'support': digest_object({path: sha(helper / path) for path in support})}
    return {'identity': digest_object(fields), **fields}


def rust_test_paths(stage):
    return json.loads((stage / 'rust-tests.json').read_bytes())


def run_one(runner, name, stages, runtime, python):
    node = stages['normal'] / 'geometry-mesh/node.exe'
    cwd = ROOT
    env = None
    if runner in {'adapter', 'dispatcher'}:
        command = [node, '--test', '--test-reporter=tap', f'--test-name-pattern=^{name}$',
                   READER / f'geometry-mesh-{runner}.test.mjs']
        proof = r'(?m)^# tests 1\s*$'
    elif runner == 'python':
        cwd = WORKER / 'fixtures'
        klass = 'ExactWorkerTests' if name in {'native_fuzzy_floor', 'utf8_stdin_under_cp932'} else 'NativeBoundaryTests'
        command = [python, '-m', 'unittest', '-v', f'test_generate_difference.{klass}.test_{name}']
        proof = r'Ran 1 test\b'
    elif runner == 'staged':
        command = [python, WORKER / 'fixtures/staged_geometry.py',
                   '--normal', stages['normal'] / 'aware.exe', '--fault', stages['fault'] / 'aware.exe',
                   '--runtime', runtime, f'StageTests.test_{name}']
        proof = r'Ran 1 test\b'
    else:
        role = 'release' if runner == 'rust-release' else 'normal'
        binary = rust_test_paths(stages[role])['supervision' if runner == 'rust-supervision' else 'wire']
        test = f'geometry::supervision_tests::{name}' if runner == 'rust-supervision' else name
        # The wire tests are inside a Windows-only integration-test module.
        if runner == 'rust-wire':
            test = 'development::' + test
        env = dict(os.environ, AWARE_GEOMETRY_TEST_CLI=str(stages[role] / 'aware.exe'))
        command = [binary, '--exact', test]
        proof = r'test result: ok\. 1 passed;'
    stdout, stderr = capture(command, cwd, env)
    check(re.search(proof, stdout + stderr), f'{name} ran zero or multiple tests\n{stdout}\n{stderr}')
    if runner in {'adapter', 'dispatcher'}:
        check(re.search(r'(?m)^# pass 1\s*$', stdout), f'{name} did not pass\n{stdout}\n{stderr}')
    elif runner in {'python', 'staged'}:
        check(re.search(r'(?m)^Ran 1 test in [^\n]+\n\nOK\s*\Z', stderr),
              f'{name} did not pass\n{stdout}\n{stderr}')


def stages_under(root):
    return {name: root / name for name in ('normal', 'fault', 'release')}


def run_gate(args, lock):
    check(sys.platform == 'win32' and os.environ.get('PROCESSOR_ARCHITECTURE') == 'AMD64', 'requires Windows x64')
    installed = json.loads((args.runtime / 'runtimes/occt/cp312-win_amd64' / lock / 'complete.json').read_bytes())
    check(installed['lock_sha256'] == lock and installed['manifest_sha256'] == sha(WORKER / 'wheel-manifest.json'),
          'test runtime does not match the verified wheel lock and manifest')
    python = Path(installed['path']) / 'Scripts/python.exe'
    stages = stages_under(args.stages)
    identities = {role: artifact(location, role, lock) for role, location in stages.items()}
    node = stages['normal'] / 'geometry-mesh/node.exe'
    output, _ = capture([node, '--input-type=module', '-e',
                         "import {replayFlushBaseline} from './geometry-flush-baseline.mjs';"
                         "console.log(JSON.stringify(await replayFlushBaseline()));"], READER)
    report = {'commit': args.commit, 'wheel_lock': lock, 'abi': ABI, 'artifacts': identities,
              'run_id': int(os.environ.get('GITHUB_RUN_ID', '0')),
              'run_attempt': int(os.environ.get('GITHUB_RUN_ATTEMPT', '0')), 'raw_baseline': json.loads(output), 'cases': []}
    for suite, name, runner, role, outcome in cases():
        run_one(runner, name, stages, args.runtime, python)
        roles = ('normal', 'fault') if role == 'dual' else (role,)
        report['cases'].append({'suite': suite, 'case': name, 'outcome': outcome, 'tests': 1,
                                'artifacts': {r: identities[r]['identity'] for r in roles}})
        print(f'{suite}/{name} {outcome}', flush=True)
    validate(report, args.commit, lock)
    args.report.parent.mkdir(parents=True, exist_ok=True)
    args.report.write_text(json.dumps(report, indent=2) + '\n', encoding='utf-8')


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('operation', choices=['run', 'validate'])
    parser.add_argument('--commit', required=True)
    parser.add_argument('--report', type=Path, required=True)
    parser.add_argument('--stages', type=Path)
    parser.add_argument('--runtime', type=Path)
    args = parser.parse_args()
    commit, _ = capture(['git', 'rev-parse', 'HEAD'])
    check(re.fullmatch('[0-9a-f]{40}', args.commit) and commit.strip() == args.commit,
          'checked-out commit differs from evidence commit')
    lock = sha(LOCK)
    if args.operation == 'run':
        check(args.stages is not None and args.runtime is not None, 'run requires the stages and runtime paths')
        run_gate(args, lock)
    else:
        report = json.loads(args.report.read_bytes())
        validate(report, args.commit, lock)
        for field, env in (('run_id', 'GITHUB_RUN_ID'), ('run_attempt', 'GITHUB_RUN_ATTEMPT')):
            if env in os.environ:
                check(report.get(field) == int(os.environ[env]), f'stale or missing {field}')
        if args.stages is not None:
            for role, stage in stages_under(args.stages).items():
                check(artifact(stage, role, lock) == report['artifacts'][role], f'downloaded {role} artifact differs')


if __name__ == '__main__':
    main()
