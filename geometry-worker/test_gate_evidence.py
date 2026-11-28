import copy
import json
import re
from pathlib import Path
import runpy
import subprocess
import sys
import tempfile
import unittest
from unittest.mock import patch

from gate_evidence import ROOT, WORKER, validate, cases, main, raw_reference, run_one, sha


class GateEvidenceTests(unittest.TestCase):
    def setUp(self):
        def artifact(c, w, fault):
            shared = {name: '3' * 64 for name in ('node', 'dispatcher', 'slabs', 'adapter', 'wasm', 'support')}
            fields = {'cli': c * 64, 'worker': w * 64, 'fault_enabled': fault, **shared}
            from gate_evidence import digest_object
            return {'identity': digest_object(fields), **fields}
        self.artifacts = {'normal': artifact('1', '4', False), 'fault': artifact('2', '5', True),
                          'release': {'cli': '6' * 64, 'identity': '6' * 64}}
        self.report = {'commit': 'a' * 40, 'wheel_lock': 'b' * 64, 'abi': 'windows-x64',
                       'artifacts': self.artifacts, 'raw_baseline': raw_reference(), 'cases': [
                           {'suite': suite, 'case': case, 'outcome': outcome, 'tests': 1,
                            'artifacts': {role: self.artifacts[role]['identity']
                                          for role in (('normal', 'fault') if role == 'dual' else (role,))}}
                           for suite, case, _, role, outcome in cases()
                       ]}

    def check(self, report):
        validate(report, 'a' * 40, 'b' * 64)

    def test_complete(self):
        self.check(self.report)

    def test_geometry_digests_match_lf_and_crlf_checkouts(self):
        inputs = [
            'geometry-worker/win-amd64-cp312.requirements.lock',
            'geometry-worker/wheel-manifest.json',
            *(path.relative_to(ROOT).as_posix() for path in sorted((WORKER / 'licenses').glob('*.txt'))),
            'geometry-worker/occt-worker.py',
            'geometry-worker/occt-test-faults.py',
            'geometry-worker/reply-test-faults.mjs',
            *(f'cli-connection-reader/{name}.mjs' for name in (
                'geometry-mesh-dispatcher', 'geometry-slabs', 'mesh-solid', 'snapshot-clash',
                'snapshot-clash-geometry', 'snapshot-compare', 'manifold-wasm')),
        ]
        with tempfile.TemporaryDirectory() as directory:
            scratch = Path(directory)
            repo = scratch / 'repo'
            repo.mkdir()

            def git(*args):
                subprocess.run(['git', *args], cwd=repo, check=True, capture_output=True)

            git('init', '--quiet')
            (repo / 'checkout-control.txt').write_bytes(b'line endings\n')
            for name in [*inputs, '.gitattributes']:
                source = ROOT / name
                if source.exists():
                    target = repo / name
                    target.parent.mkdir(parents=True, exist_ok=True)
                    target.write_bytes(source.read_bytes())
            git('-c', 'core.autocrlf=input', 'add', '.')
            for ending, autocrlf in [('lf', 'input'), ('crlf', 'true')]:
                git('-c', f'core.autocrlf={autocrlf}', 'checkout-index', '--all',
                    f'--prefix={scratch.as_posix()}/{ending}/')
            self.assertEqual((scratch / 'lf/checkout-control.txt').read_bytes(), b'line endings\n')
            self.assertEqual((scratch / 'crlf/checkout-control.txt').read_bytes(), b'line endings\r\n')
            for name in inputs:
                with self.subTest(name=name):
                    self.assertEqual(sha(scratch / 'lf' / name), sha(scratch / 'crlf' / name))

    def test_gate_uses_the_evidence_job_attempt_on_gate_only_rerun(self):
        workflow = (ROOT / '.github/workflows/geometry-m1.yml').read_text(encoding='utf-8')
        producer, gate = workflow.split('\n  geometry-m1-gate:\n')
        producer_attempt, gate_attempt = '1', '2'
        output = 'needs.windows-evidence.outputs.attempt'
        attempts = {'github.run_attempt': gate_attempt, output: producer_attempt}
        for artifact, expression in re.findall(r'name: (geometry-m1-\w+)-\$\{\{ ([\w.-]+) \}\}', gate):
            with self.subTest(artifact=artifact):
                self.assertEqual(f'{artifact}-{attempts[expression]}', f'{artifact}-{producer_attempt}')
        self.assertEqual(len(re.findall(r'name: geometry-m1-\w+-\$\{\{', gate)), 4)
        self.assertIn('attempt: ${{ steps.evidence.outputs.attempt }}', producer)
        self.assertIn('"attempt=$env:GITHUB_RUN_ATTEMPT" >> $env:GITHUB_OUTPUT', producer)
        self.assertIn(f'EVIDENCE_ATTEMPT: ${{{{ {output} }}}}', gate)
        self.assertIn('--run-attempt $env:EVIDENCE_ATTEMPT', gate)

    def test_validator_checks_producer_attempt_during_gate_rerun(self):
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / 'report.json'
            path.write_text(json.dumps({**self.report, 'run_id': 20, 'run_attempt': 1}), encoding='utf-8')
            command = ['gate_evidence.py', 'validate', '--commit', self.report['commit'], '--report', str(path)]
            with (patch.dict('os.environ', GITHUB_RUN_ID='20', GITHUB_RUN_ATTEMPT='2'),
                  patch('gate_evidence.capture', return_value=(self.report['commit'], '')),
                  patch('gate_evidence.sha', return_value=self.report['wheel_lock'])):
                with patch.object(sys, 'argv', command + ['--run-attempt', '1']):
                    main()
                with patch.object(sys, 'argv', command), self.assertRaisesRegex(ValueError, 'stale or missing run_attempt'):
                    main()
                with (patch.dict('os.environ', GITHUB_RUN_ID='21'),
                      patch.object(sys, 'argv', command + ['--run-attempt', '1']),
                      self.assertRaisesRegex(ValueError, 'stale or missing run_id')):
                    main()

    def test_neither_python_runner_accepts_a_skip(self):
        staged = """import sys, unittest
class StageTests(unittest.TestCase):
    @unittest.skip('deliberately skipped')
    def test_skip(self):
        pass
unittest.main(argv=[sys.argv[0], '-v', sys.argv[-1]])
"""
        worker = staged[:staged.index('unittest.main')].replace('StageTests', 'NativeBoundaryTests')
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            fixtures = root / 'fixtures'
            fixtures.mkdir()
            (fixtures / 'staged_geometry.py').write_text(staged, encoding='utf-8')
            (fixtures / 'test_generate_difference.py').write_text(worker, encoding='utf-8')
            with patch('gate_evidence.WORKER', root):
                for runner in ('python', 'staged'):
                    with self.subTest(runner=runner), self.assertRaisesRegex(ValueError, 'did not pass'):
                        run_one(runner, 'skip', {'normal': root, 'fault': root}, root, sys.executable)

    def test_offline_gate_is_dispatchable_on_hosted_windows(self):
        path = ROOT / '.github/workflows/geometry-offline.yml'
        self.assertTrue(path.is_file(), 'missing geometry-offline workflow')
        workflow = path.read_text(encoding='utf-8')
        for required in ('workflow_dispatch:', 'candidate_sha:', 'windows_run_id:', 'guest_report:',
                         'runs-on: windows-2025', 'offline-gate.py validate', 'actions/upload-artifact@'):
            with self.subTest(required=required):
                self.assertIn(required, workflow)
        self.assertNotIn('self-hosted', workflow)

    def test_offline_guest_report_is_bound_to_the_passing_attempt(self):
        offline = runpy.run_path(str(WORKER / 'offline-gate.py'))
        validate_guest = offline['validate_guest_report']
        expected = json.loads((WORKER / 'offline-cases.json').read_bytes())
        bound = {k: self.report[k] for k in ('commit', 'wheel_lock', 'abi')}
        bound.update(windows_run_id=20, windows_run_attempt=2, normal_artifact=self.artifacts['normal']['identity'])
        report = {**bound, 'cases': expected, 'network_adapters_up': 0, 'default_routes': 0,
                  'child_images': offline['EXPECTED_CHILD_IMAGES']}
        validate_guest(report, bound, expected)
        for field, value in {
            'commit': 'c' * 40, 'wheel_lock': 'c' * 64, 'abi': 'linux-x64', 'normal_artifact': '0' * 64,
            'windows_run_id': 21, 'windows_run_attempt': 1, 'default_routes': 1, 'network_adapters_up': False,
            'cases': expected[:-1],
        }.items():
            with self.subTest(field=field), self.assertRaisesRegex(ValueError, 'missing, stale or failed'):
                validate_guest({**report, field: value}, bound, expected)

    def test_empty_incomplete_stale_or_wrong(self):
        variants = [{}]
        for change in (
            lambda r: r.update(cases=[]),
            lambda r: r['cases'].pop(),
            lambda r: r['cases'].append(r['cases'][0]),
            lambda r: r['cases'][0].update(tests=0),
            lambda r: r['cases'][0].update(outcome='skipped'),
            lambda r: r['cases'][0].update(artifacts={'normal': '0' * 64}),
            lambda r: r.update(commit='c' * 40),
            lambda r: r.update(wheel_lock='c' * 64),
            lambda r: r.update(abi='linux-x64'),
            lambda r: r['artifacts'].update(fault=r['artifacts']['normal']),
        ):
            report = copy.deepcopy(self.report)
            change(report)
            variants.append(report)
        for index, report in enumerate(variants):
            with self.subTest(index=index), self.assertRaises(ValueError):
                self.check(report)


if __name__ == '__main__':
    unittest.main()

