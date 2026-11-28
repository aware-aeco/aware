"""Launch tests for the two staged Windows development artifacts."""

import argparse
import collections
import json
import math
import os
from pathlib import Path
import subprocess
import sys
import tempfile
import time
import unittest
from decimal import Decimal


LIMIT = 32 * 1024 * 1024
RECT = [(0, 0), (100, 0), (100, 100), (0, 100)]


def prism_pair(xy, z, *, left=None):
    request = request_for('flush_origin')
    if left is not None:
        request['a'] = left
    request['b'] = operand('clip', xy, z, [0, 0, 0])
    return request


def mesh_metrics(result):
    points = result['mesh']['positions_mm']
    faces = result['mesh']['triangles']
    assert len(set(map(tuple, points))) == len(points)
    edges = collections.Counter()
    fan = collections.defaultdict(set)
    for i, j, k in faces:
        assert len({i, j, k}) == 3 and min(i, j, k) >= 0 and max(i, j, k) < len(points)
        for a, b in ((i, j), (j, k), (k, i)):
            edges[a, b] += 1
            fan[a].add(b)
            fan[b].add(a)
    assert all(count == 1 and edges[b, a] == 1 for (a, b), count in edges.items())
    pending = set(fan)
    shells = []
    centers = []
    while pending:
        connected = {pending.pop()}
        todo = list(connected)
        while todo:
            for point in fan[todo.pop()] & pending:
                todo.append(point)
                pending.remove(point)
                connected.add(point)
        volumes = []
        moments = [[], [], []]
        for i, j, k in faces:
            if i not in connected:
                continue
            a, b, c = points[i], points[j], points[k]
            tetra = sum(a[q] * (b[(q+1) % 3] * c[(q+2) % 3] - b[(q+2) % 3] * c[(q+1) % 3])
                        for q in range(3)) / 6
            volumes.append(tetra)
            for axis in range(3):
                moments[axis].append(tetra * (a[axis] + b[axis] + c[axis]) / 4)
        shells.append(math.fsum(volumes))
        centers.append([math.fsum(values) for values in moments])
    volume = math.fsum(shells)
    anchor = result['offset_mm']
    member = [[v + anchor[i] for i, v in enumerate(point)] for point in points]
    centroid = [math.fsum(row[i] for row in centers) / volume + anchor[i] for i in range(3)] if volume else None
    return volume, shells, centroid, member


def occupied(result, member):
    """Oriented solid angle at an interior witness, independent of slab checker."""
    query = [x - a for x, a in zip(member, result['offset_mm'])]
    points = [[v - q for v, q in zip(p, query)] for p in result['mesh']['positions_mm']]
    def dot(a, b):
        return sum(x*y for x, y in zip(a, b))
    angles = []
    for i, j, k in result['mesh']['triangles']:
        a, b, c = points[i], points[j], points[k]
        determinant = sum(a[q] * (b[(q+1) % 3] * c[(q+2) % 3] - b[(q+2) % 3] * c[(q+1) % 3])
                          for q in range(3))
        na, nb, nc = math.hypot(*a), math.hypot(*b), math.hypot(*c)
        denominator = na * nb * nc + dot(a, b)*nc + dot(b, c)*na + dot(c, a)*nb
        angles.append(2 * math.atan2(determinant, denominator))
    return abs(math.fsum(angles)) > 2 * math.pi


class StageTests(unittest.TestCase):
    def run_cli(self, request, *, exact=True, fault=None, route=None, file=False):
        binary = self.fault if fault else self.normal
        with tempfile.TemporaryDirectory(prefix='aware-872-exec-') as name:
            scratch = Path(name)
            home = self.runtime if exact else scratch / 'no-runtime'
            environment = {k: v for k, v in os.environ.items()
                           if k.upper() not in {'PATH', 'AWARE_GEOMETRY_TEST_FAULT', 'AWARE_GEOMETRY_TEST_TRACE',
                                               'PYTHONPATH', 'PYTHONHOME'}}
            environment['AWARE_HOME'] = str(home)
            trace = scratch / 'trace.jsonl'
            if fault:
                environment['AWARE_GEOMETRY_TEST_FAULT'] = f'{route or "exact"}:{fault}'
                environment['AWARE_GEOMETRY_TEST_TRACE'] = str(trace)
            payload = json.dumps(request).encode()
            if file:
                request_path = scratch / 'request.json'
                request_path.write_bytes(payload)
                argument = '@' + str(request_path)
            else:
                argument = '-'
            command = [str(binary), 'geometry', 'exec', argument, '--json']
            run = subprocess.run(command, env=environment, input=None if file else payload, capture_output=True, timeout=100)
            self.assertEqual(run.returncode, 0, run.stderr.decode(errors='replace'))
            self.assertEqual(run.stderr, b'')
            self.assertTrue(run.stdout.endswith(b'\n') and len(run.stdout) <= LIMIT)
            envelope = json.loads(run.stdout)
            self.assertTrue(envelope['ok'], envelope)
            data = envelope['data']
            self.assertTrue(data['operand_ids'] == {key: request[key]['id'] for key in ('a', 'b')})
            records = [json.loads(line) for line in trace.read_text().splitlines()] if trace.exists() else []
            return data, records

    def cut(self, data, status, volume, band=1e-8):
        self.assertEqual(data['status'], status, (data['reason_code'], data['warnings']))
        self.assertIsNone(data['reason_code'])
        result = data['result']
        self.assertEqual(bool(result['brep_b64']), status == 'validated_brep' and volume > 0)
        self.assertAlmostEqual(result['volume_mm3'], volume, delta=band)
        metrics = mesh_metrics(result)
        self.assertAlmostEqual(metrics[0], volume, delta=band)
        return result, metrics

    def unavailable(self, data, code):
        self.assertEqual(data['status'], 'unavailable')
        self.assertEqual(data['reason_code'], code)
        self.assertIsNone(data['result'])
        self.assertEqual(set(data), {'schema', 'op', 'kind', 'frame', 'status', 'exact_status',
                                     'warnings', 'result', 'reason_code', 'operand_ids'})

    def flush_cut(self, request, data, status):
        result, (_, shells, center, points) = self.cut(data, status, 1_973_000)
        start = [float(v) for v in (*request['a']['profile']['xy_mm'][0], request['a']['profile']['z_mm'][0])]
        dims = [400, 250, 20]
        retained = 1_973_000
        analytic = [(2_000_000 * a - 27_000 * b) / retained + start[i]
                    for i, (a, b) in enumerate(zip([200, 125, 10], [20, 15, 10]))]
        self.assertEqual(len(shells), 1)
        for axis in range(3):
            self.assertAlmostEqual(center[axis], analytic[axis], delta=1e-7)
            self.assertAlmostEqual(min(p[axis] for p in points), start[axis], delta=1e-7)
            self.assertAlmostEqual(max(p[axis] for p in points), start[axis] + dims[axis], delta=1e-7)
        frame = request['frame']
        def world(point):
            return [frame['origin_mm'][i] + sum(frame['axes'][axis][i] * point[axis] for axis in range(3))
                    for i in range(3)]
        corners = [world([start[0] + x, start[1] + y, start[2] + z])
                   for x in (0, 400) for y in (0, 250) for z in (0, 20)]
        world_points = list(map(world, points))
        for axis in range(3):
            self.assertAlmostEqual(min(p[axis] for p in world_points), min(p[axis] for p in corners), delta=1e-7)
            self.assertAlmostEqual(max(p[axis] for p in world_points), max(p[axis] for p in corners), delta=1e-7)
        for z in (3, 10, 17):
            for xy, kept in [((10, 10), False), ((20, 35), True), ((60, 5), True), ((250, 200), True)]:
                self.assertEqual(occupied(result, [v+s for v, s in zip((*xy, z), start)]), kept)
        return result

    def test_flush_origin(self):
        request = request_for('flush_origin')
        self.flush_cut(request, self.run_cli(request, file=True)[0], 'validated_brep')

    def test_flush_large_member_offset(self):
        request = request_for('flush_large_member_offset')
        self.flush_cut(request, self.run_cli(request)[0], 'validated_brep')

    def test_flush_distinct_operand_origins(self):
        request = request_for('flush_distinct_operand_origins')
        self.assertNotEqual(request['a']['offset_mm'], request['b']['offset_mm'])
        self.assertEqual(request['a']['profile']['xy_mm'][0], request['b']['profile']['xy_mm'][0])
        for exact in (True, False):
            self.flush_cut(request, self.run_cli(request, exact=exact)[0], 'validated_brep' if exact else 'mesh_fallback')

    def test_rotated_frame(self):
        request = request_for('rotated_frame')
        self.flush_cut(request, self.run_cli(request)[0], 'validated_brep')

    def test_no_runtime_flush(self):
        request = request_for('flush_stress')
        data, _ = self.run_cli(request, exact=False)
        self.flush_cut(request, data, 'mesh_fallback')
        self.assertEqual(data['exact_status'], 'unavailable')
        self.assertEqual([w['code'] for w in data['warnings']], ['occt_runtime_missing', 'chord_height_unverified'])

    def test_explicit_null_brep_b64(self):
        request = request_for('flush_stress')
        request['b']['brep_b64'] = None
        data, _ = self.run_cli(request)
        self.flush_cut(request, data, 'mesh_fallback')
        self.assertEqual(data['warnings'][0]['code'], 'brep_input_missing')

    def test_swapped_operands(self):
        request = request_for('swapped_operands')
        for exact in (True, False):
            self.cut(self.run_cli(request, exact=exact)[0], 'validated_brep' if exact else 'mesh_fallback', 0)

    def test_far_disjoint_both_orders(self):
        request = request_for('far_disjoint')
        for volume in (2_000_000, 27_000):
            for exact in (True, False):
                self.cut(self.run_cli(request, exact=exact)[0], 'validated_brep' if exact else 'mesh_fallback', volume)
            request['a'], request['b'] = request['b'], request['a']

    def test_extent_guards(self):
        request = request_for('flush_origin')
        request['b'] = operand('out-of-range', [(16385+x, y) for x, y in RECT], [0, 100], [0, 0, 0])
        for exact in (True, False):
            self.unavailable(self.run_cli(request, exact=exact)[0], 'budget_exceeded')
        # Each operand fits locally but the overlapping union is nearly 64 metres long.
        request['a'] = operand('left', [(-16000, 0), (16000, 0), (16000, 100), (-16000, 100)], [0, 100], [-16000, 0, 0])
        request['b'] = operand('right', [(-16000, 0), (16000, 0), (16000, 100), (-16000, 100)], [0, 100], [15999, 0, 0])
        for exact in (True, False):
            self.unavailable(self.run_cli(request, exact=exact)[0], 'budget_exceeded')

    def test_through_cut_residual_skin_both_paths(self):
        strip = [(180, -20), (200, -20), (200, 270), (180, 270)]
        request = prism_pair(strip, ['0', '19.999995'])
        request['b']['profile']['z_mm'][1] = '20'
        for exact in (True, False):
            data, _ = self.run_cli(request, exact=exact)
            self.unavailable(data, 'coincidence_mismatch')
            self.assertIn('top_cap', data['warnings'][-1]['message'])
            self.assertEqual(data['exact_status'], 'failed' if exact else 'unavailable')
            if exact:
                self.assertEqual(len(data['warnings']), 1)

    def test_enclosed_cavity_both_paths(self):
        request = request_for('enclosed_cavity')
        for exact in (True, False):
            data, _ = self.run_cli(request, exact=exact)
            result, (_, shells, _, _) = self.cut(data, 'validated_brep' if exact else 'mesh_fallback', 1_984_000)
            self.assertEqual(sorted(round(v) for v in shells), [-16_000, 2_000_000])
            self.assertFalse(occupied(result, [70, 75, 10]))
            self.assertTrue(occupied(result, [70, 75, 3]))

    def test_tangent_cavity_both_paths(self):
        request = prism_pair([(0, 100), (30, 70), (60, 100), (30, 130)], [6, 14])
        for exact in (True, False):
            self.unavailable(self.run_cli(request, exact=exact)[0], 'mesh_invalid')

    def threshold_case(self, kind):
        a = operand('plate', RECT, [0, 100], [0, 0, 0])
        def pair(width):
            width = Decimal(str(width))
            offset = 100 - width if kind == 'overlap' else width
            request = request_for('flush_origin')
            request['a'] = a
            request['b'] = operand('clip', RECT, [0, 100], [0, 0, offset])
            return request
        node = self.normal.parent / 'geometry-mesh/node.exe'
        def admission(request):
            script = ("import {readFileSync} from 'node:fs'; "
                      "import {inspectAdmission} from './geometry-mesh-dispatcher.mjs'; "
                      "console.log(JSON.stringify(inspectAdmission(JSON.parse(readFileSync(0)))));")
            reply = subprocess.run([str(node), '--input-type=module', '-e', script],
                                   cwd=node.parent, input=json.dumps(request).encode(), capture_output=True, check=True)
            return json.loads(reply.stdout)
        threshold = admission(pair('0.01'))['threshold_mm'][2]
        for width, admitted in [(1e-8, False), (0.95 * threshold, False), (1.05 * threshold, True)]:
            request = pair(width)
            report = admission(request)
            self.assertAlmostEqual(report['clearance_mm'][2], width, delta=1e-10)
            for path in ('exact', 'mesh'):
                for term in ('input_mm', 'placement_mm', 'construction_mm', 'kernel_mm'):
                    self.assertGreater(min(report['budgets'][path][term]) if term == 'placement_mm'
                                       else report['budgets'][path][term], 0)
            self.assertEqual(report['code'], None if admitted else 'budget_exceeded')
            for exact in (True, False):
                data, _ = self.run_cli(request, exact=exact)
                if not admitted:
                    self.unavailable(data, 'budget_exceeded')
                    _, trace = self.run_cli(request, exact=exact, fault='trace', route='mesh')
                    self.assertTrue(trace)
                    self.assertFalse(any(row['event'] == 'mesh_boolean' or
                                         row['event'] == 'launch' and row['route'] == 'native' and row['mode'] is None
                                         for row in trace))
                else:
                    volume = 10_000 * (100 - width if kind == 'overlap' else width)
                    result, _ = self.cut(data, 'validated_brep' if exact else 'mesh_fallback',
                                         volume, band=1e-8 if exact else 0.08)
                    witness_z = 10 if kind == 'overlap' else width / 2
                    self.assertTrue(occupied(result, [50, 50, witness_z]))

    def test_thin_overlap_threshold_both_paths(self):
        self.threshold_case('overlap')

    def test_thin_residual_threshold_both_paths(self):
        self.threshold_case('residual')

    def test_equal_volume_shifted_pocket(self):
        request = request_for('enclosed_cavity')
        data, trace = self.run_cli(request, fault='shifted-pocket')
        result, _ = self.cut(data, 'mesh_fallback', 1_984_000)
        self.assertEqual(data['warnings'][0]['code'], 'brep_validation_failed')
        row = next(row for row in trace if row['event'] == 'shifted_pocket')
        self.assertTrue(row['valid'] and row['fixed_witnesses_preserved'])
        self.assertAlmostEqual(row['volume_mm3'], 1_984_000, delta=1e-8)
        self.assertTrue(occupied(result, [52, 75, 2]))
        self.assertFalse(occupied(result, [52, 75, 10]))
        self.assertFalse(occupied(result, [70, 75, 10]))

    def test_shifted_exact_tessellation(self):
        request = request_for('flush_stress')
        data, trace = self.run_cli(request, fault='shifted-tessellation')
        self.flush_cut(request, data, 'mesh_fallback')
        self.assertEqual(data['warnings'][0]['code'], 'brep_validation_failed')
        self.assertIn({'event': 'shifted_tessellation'}, trace)

    def forced_cleanup(self, fault, event):
        request = request_for('flush_stress')
        if fault == 'blocked-stdin':
            request['a']['id'] = 'x' * (1024 * 1024)
        start = time.monotonic()
        data, trace = self.run_cli(request, fault=fault)
        self.flush_cut(request, data, 'mesh_fallback')
        self.assertEqual(data['warnings'][0]['code'], 'occt_timeout')
        self.assertLess(time.monotonic() - start, 30)
        self.assertTrue(any(row['event'] == event for row in trace))
        self.assertTrue(any(row['event'] == 'launch' and row['route'] == 'mesh' and row['mode'] is None
                            for row in trace))

    def test_blocked_stdin_fallback(self):
        self.forced_cleanup('blocked-stdin', 'stdin_blocked')

    def test_held_open_pipe_fallback(self):
        self.forced_cleanup('held-open', 'descendant_holds_pipe')

    def output_overflow(self, stream):
        request = request_for('flush_stress')
        data, trace = self.run_cli(request, fault=stream)
        self.flush_cut(request, data, 'mesh_fallback')
        self.assertEqual(data['warnings'][0]['code'], 'budget_exceeded')
        self.assertIn({'event': 'overflow', 'route': 'exact', 'fault': stream}, trace)

    def test_oversized_stdout(self):
        self.output_overflow('stdout')

    def test_oversized_stderr(self):
        self.output_overflow('stderr')

    def test_general_slabs_and_separated_components(self):
        strip = [(180, -20), (200, -20), (200, 270), (180, 270)]
        hole = [(50, 50), (90, 50), (90, 100), (50, 100)]
        for xy, z, volume, count, witness in [
            (strip, [0, 20], 1_900_000, 2, [190, 125, 10]),
            (strip, [6, 14], 1_960_000, 1, [190, 125, 10]),
            (hole, [0, 20], 1_960_000, 1, [70, 75, 10]),
        ]:
            request = prism_pair(xy, z)
            for exact in (True, False):
                data, _ = self.run_cli(request, exact=exact)
                result, (_, shells, _, _) = self.cut(data, 'validated_brep' if exact else 'mesh_fallback', volume)
                self.assertEqual(len(shells), count)
                self.assertTrue(all(v > 0 for v in shells))
                self.assertFalse(occupied(result, witness))
                self.assertTrue(occupied(result, [20, 125, 10]))
                if z[0]:
                    self.assertTrue(occupied(result, [*witness[:2], 3]))

    def test_rejected_after_real_cut(self):
        request = request_for('flush_stress')
        control, _ = self.run_cli(request)
        accepted = self.flush_cut(request, control, 'validated_brep')
        data, trace = self.run_cli(request, fault='wrong-solid')
        mesh = self.flush_cut(request, data, 'mesh_fallback')
        self.assertEqual(data['exact_status'], 'failed')
        self.assertEqual([w['code'] for w in data['warnings']], ['brep_validation_failed', 'chord_height_unverified'])
        self.assertIsNone(mesh['brep_b64'])
        self.assertNotEqual(accepted['brep_b64'], mesh['brep_b64'])
        swap = next(row for row in trace if row['event'] == 'post_cut_swap')
        self.assertAlmostEqual(swap['cut_volume_mm3'], 1_973_000, delta=1e-8)
        self.assertAlmostEqual(swap['injected_volume_mm3'], 2_000_000, delta=1e-8)
        self.assertTrue(swap['valid'])

    def reply_fault(self, fault, event):
        request = request_for('flush_stress')
        for route in ('native', 'exact', 'mesh'):
            with self.subTest(route=route, fault=fault):
                data, trace = self.run_cli(request, exact=route != 'mesh', fault=fault, route=route)
                if route != 'mesh':
                    self.flush_cut(request, data, 'mesh_fallback')
                    self.assertEqual(data['warnings'][0]['code'], 'occt_worker_failed')
                else:
                    self.unavailable(data, 'mesh_worker_failed')
                self.assertIn({'event': event, 'route': route, 'fault': fault}, trace)

    def test_malformed_child_json(self):
        self.reply_fault('malformed', 'reply_fault')

    def test_bad_child_reply_shape(self):
        for fault in ('omitted-field', 'unknown-field'):
            self.reply_fault(fault, 'reply_fault')

    def test_nonzero_child_after_valid_reply(self):
        self.reply_fault('nonzero', 'valid_reply_written')

    def test_oversized_envelope(self):
        request = request_for('flush_stress')
        request['a']['id'] = 'x' * (LIMIT - 250_000)
        for route in ('exact', 'mesh'):
            with self.subTest(route=route):
                data, trace = self.run_cli(request, exact=route == 'exact', fault='envelope', route=route)
                if route == 'exact':
                    self.flush_cut(request, data, 'mesh_fallback')
                    self.assertEqual(data['warnings'][0]['code'], 'budget_exceeded')
                else:
                    self.unavailable(data, 'budget_exceeded')
                row = next(row for row in trace if row['event'] == 'envelope_payload')
                self.assertLessEqual(row['child_bytes'], LIMIT)
                wrapper = next(row for row in trace if row['event'] == 'wrapped_envelope')
                self.assertGreater(wrapper['wrapped_bytes'], LIMIT)


if __name__ == '__main__':
    from generate_difference import operand, request_for

    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--normal', type=Path, required=True)
    parser.add_argument('--fault', type=Path, required=True)
    parser.add_argument('--runtime', type=Path, required=True)
    parser.add_argument('case', nargs='*')
    args = parser.parse_args()
    StageTests.normal, StageTests.fault, StageTests.runtime = args.normal, args.fault, args.runtime
    unittest.main(argv=[sys.argv[0], '-v', *args.case])
