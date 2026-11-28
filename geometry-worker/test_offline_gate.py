import json
import os
import runpy
import sys
import unittest

from gate_evidence import WORKER
from process_trace import trace_processes


offline = runpy.run_path(str(WORKER / 'offline-gate.py'))
validate_guest_report = offline['validate_guest_report']


class OfflineGateTests(unittest.TestCase):
    def test_stale_base_report_requires_process_evidence(self):
        expected = json.loads((WORKER / 'offline-cases.json').read_bytes())
        bound = {'commit': 'a' * 40}
        legacy = {**bound, 'network_adapters_up': 0, 'default_routes': 0, 'cases': expected}
        with self.assertRaisesRegex(ValueError, 'process'):
            validate_guest_report(legacy, bound, expected)

    def test_stale_base_report_rejects_python_or_missing_mesh_launch(self):
        expected = json.loads((WORKER / 'offline-cases.json').read_bytes())
        bound = {'commit': 'a' * 40}
        images = offline['EXPECTED_CHILD_IMAGES']
        report = {**bound, 'network_adapters_up': 0, 'default_routes': 0, 'cases': expected, 'child_images': images}
        validate_guest_report(report, bound, expected)
        for case in ('base_python_missing', 'base_python_replaced'):
            for names in ([], ['node.exe', 'python.exe'], ['node.exe', 'other.exe']):
                with self.subTest(case=case, names=names), self.assertRaisesRegex(ValueError, 'process'):
                    validate_guest_report({**report, 'child_images': {**images, case: names}}, bound, expected)

    @unittest.skipUnless(sys.platform == 'win32', 'requires Windows process events')
    def test_observer_sees_short_lived_python_descendant(self):
        source = "import subprocess, sys; subprocess.run([sys.executable, '-c', 'pass'], check=True)"
        result, images = trace_processes([sys.executable, '-c', source], cwd=WORKER,
                                         env=os.environ, payload=b'', timeout=15)
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertEqual(images, ['python.exe'])


if __name__ == '__main__':
    unittest.main()
