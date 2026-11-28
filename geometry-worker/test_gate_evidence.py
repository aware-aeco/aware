import copy
import unittest

from gate_evidence import validate, cases, raw_reference


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

