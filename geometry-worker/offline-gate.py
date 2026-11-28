"""Verify the candidate and collect offline Windows provisioning evidence."""

import argparse
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile

from gate_evidence import ABI, LOCK, WORKER, artifact, capture, check, sha, validate


def binding(args):
    windows = json.loads(args.windows_report.read_bytes())
    lock = sha(LOCK)
    commit, _ = capture(['git', 'rev-parse', 'HEAD'])
    check(commit.strip() == args.commit, 'offline checkout is not the candidate SHA')
    check(args.run_id > 0 and windows.get('run_id') == args.run_id, 'Windows build run does not match')
    validate(windows, args.commit, lock)
    candidate = artifact(args.normal, 'normal', lock)
    check(candidate == windows['artifacts']['normal'], 'normal artifact differs from Windows gate')
    return {'commit': args.commit, 'wheel_lock': lock, 'abi': ABI,
            'windows_run_id': args.run_id, 'normal_artifact': candidate['identity']}


def guest(args, bound, expected):
    check(sys.platform == 'win32' and os.environ.get('PROCESSOR_ARCHITECTURE') == 'AMD64', 'requires Windows x64')
    adapters, _ = capture(['powershell.exe', '-NoProfile', '-Command',
                           '@(Get-NetAdapter -ErrorAction Stop | Where-Object Status -eq Up).Count'])
    check(adapters.strip() == '0', 'guest has an enabled network adapter')
    with tempfile.TemporaryDirectory(prefix='g872-', dir=Path(os.environ['SystemDrive'] + ':/')) as name:
        scratch = Path(name)
        base = scratch / 'py'
        shutil.copytree(args.python.parent, base)
        python = base / args.python.name
        cli = args.normal / 'aware.exe'

        def install(home, wheels):
            return subprocess.run([cli, 'geometry', 'install', '--occt', '--json',
                                   '--python', python, '--wheelhouse', wheels], capture_output=True,
                                  env=dict(os.environ, AWARE_HOME=str(home)), timeout=240)

        home = scratch / 'home'
        done = install(home, args.wheels)
        check(done.returncode == 0 and not done.stderr, f'offline installation failed: {done.stderr!r}')
        data = json.loads(done.stdout)['data']
        installed = Path(data['path'])
        check(data['lock_sha256'] == bound['wheel_lock'] and data['manifest_sha256'] == sha(WORKER / 'wheel-manifest.json'),
              'installed wheel identities differ')
        check(data['probe']['native_fuzzy_floor_mm'] == 1e-7, 'native import or fuzzy floor differs')
        for notice in ('OCP-LICENSE.txt', 'VTK-LICENSE.txt', 'LICENSE_LGPL_21.txt', 'OCCT_LGPL_EXCEPTION.txt'):
            check(sha(installed / 'licenses' / notice) == sha(WORKER / 'licenses' / notice), f'notice differs: {notice}')
        generated = installed / 'Scripts/python.exe'
        result, _ = capture([generated, '-I', WORKER / 'fixtures/generate_difference.py', 'flush_stress'])
        request_path = scratch / 'request.json'
        request_path.write_text(result, encoding='utf-8')

        script = (
            "import json, pathlib, sys; from staged_geometry import StageTests; "
            "c=StageTests(); c.normal=pathlib.Path(sys.argv[1]); c.runtime=pathlib.Path(sys.argv[2]); "
            "r=json.loads(pathlib.Path(sys.argv[3]).read_text()); d,_=c.run_cli(r); c.flush_cut(r, d, sys.argv[4]); "
            "assert sys.argv[4]=='validated_brep' or d['warnings'][0]['code']=='occt_runtime_missing'"
        )

        def cut(quality):
            capture([args.python, '-c', script, cli, home, request_path, quality], WORKER / 'fixtures')

        cut('validated_brep')
        observed = [['offline_lock_install', 'verified_offline_install_and_exact_cut']]

        bad = scratch / 'bad-wheels'
        shutil.copytree(args.wheels, bad)
        first = next(bad.glob('cadquery_ocp-*.whl'))
        with first.open('r+b') as stream:
            initial = stream.read(1)
            stream.seek(0)
            stream.write(bytes([initial[0] ^ 1]))
        bad_home = scratch / 'bad-home'
        rejected = install(bad_home, bad)
        check(rejected.returncode != 0 and 'wheel digest mismatch' in json.loads(rejected.stdout)['error']['message'],
              'offline installer did not reject the modified wheel')
        check(not list(bad_home.rglob('complete.json')), 'modified wheel published a completion marker')
        observed.append(['offline_bad_hash', 'modified_wheel_refused_no_marker'])

        python.rename(base / 'original-python.exe')
        cut('mesh_fallback')
        observed.append(['base_python_missing', 'occt_runtime_missing_checked_mesh'])
        shutil.copyfile(base / 'pythonw.exe', python)
        cut('mesh_fallback')
        observed.append(['base_python_replaced', 'occt_runtime_missing_checked_mesh'])
    check(observed == expected, 'offline suite differs from its manifest')
    args.report.write_text(json.dumps({**bound, 'network_adapters_up': 0, 'cases': observed}, indent=2) + '\n',
                           encoding='utf-8')


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('operation', choices=['precheck', 'guest', 'validate'])
    parser.add_argument('--commit', required=True)
    parser.add_argument('--run-id', required=True, type=int)
    for option in ('normal', 'windows-report'):
        parser.add_argument('--' + option, type=Path, required=True)
    for option in ('wheels', 'python', 'report'):
        parser.add_argument('--' + option, type=Path)
    args = parser.parse_args()
    bound = binding(args)
    expected = json.loads((WORKER / 'offline-cases.json').read_bytes())
    check(len(expected) == 4 and len({row[0] for row in expected}) == 4, 'bad offline suite manifest')
    if args.operation == 'guest':
        check(args.wheels and args.python and args.report, 'guest requires wheels, base Python and report')
        guest(args, bound, expected)
    elif args.operation == 'validate':
        check(args.report, 'validation requires the guest report')
        check(json.loads(args.report.read_bytes()) == {**bound, 'network_adapters_up': 0, 'cases': expected},
              'offline report is missing, stale or failed')
    print(json.dumps(bound))


if __name__ == '__main__':
    main()
