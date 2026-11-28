import base64
import io
import json
import pathlib
import subprocess
import sys
import unittest

from OCP.BRep import BRep_Builder
from OCP.BRepGProp import BRepGProp
from OCP.BRepTools import BRepTools
from OCP.GProp import GProp_GProps
from OCP.TopoDS import TopoDS_Shape


class ProducerTests(unittest.TestCase):
    def test_paired_flush_and_member_placement(self):
        script = pathlib.Path(__file__).with_name('generate_difference.py')
        for case, start, offset in [('flush_origin', '0', 0.0),
                                    ('flush_large_member_offset', '5000000', 5_000_000.0),
                                    ('flush_distinct_operand_origins', '5000000.125', 5_000_000.0)]:
            with self.subTest(case=case):
                run = subprocess.run([sys.executable, '-I', str(script), case], capture_output=True, check=True)
                request = json.loads(run.stdout)
                self.assertFalse(run.stderr)
                self.assertEqual(request['a']['offset_mm'][0], offset)
                for part, expected, triangles in [('a', 2_000_000, 12), ('b', 27_000, 8)]:
                    op = request[part]
                    self.assertEqual(op['profile']['xy_mm'][0][0], start)
                    self.assertEqual(len(op['mesh']['triangles']), triangles)
                    self.assertAlmostEqual(mesh_volume(op['mesh']), expected, delta=1e-8)
                    shape = TopoDS_Shape()
                    BRepTools.Read_s(shape, io.BytesIO(base64.standard_b64decode(op['brep_b64'])), BRep_Builder())
                    props = GProp_GProps()
                    BRepGProp.VolumeProperties_s(shape, props, 1e-12, True, False)
                    self.assertAlmostEqual(props.Mass(), expected, delta=1e-8)
                if case == 'flush_distinct_operand_origins':
                    self.assertEqual(request['b']['offset_mm'][0], offset + 0.125)
                    self.assertEqual(request['a']['mesh']['positions_mm'][0][0], 0.125)
                    self.assertEqual(request['b']['mesh']['positions_mm'][0][0], 0)


class ExactWorkerTests(unittest.TestCase):
    def test_native_fuzzy_floor(self):
        script = pathlib.Path(__file__).resolve().parents[1] / 'occt-worker.py'
        run = subprocess.run([sys.executable, '-I', str(script), '--native-probe'], capture_output=True, check=True)
        self.assertEqual(json.loads(run.stdout), {'requested_fuzz_mm': 0, 'native_fuzzy_floor_mm': 1e-7,
                                                'parallel': False, 'non_destructive': True})

    def test_flush_origin_native_cut(self):
        fixture = pathlib.Path(__file__).with_name('generate_difference.py')
        script = fixture.resolve().parents[1] / 'occt-worker.py'
        request = subprocess.check_output([sys.executable, '-I', str(fixture), 'flush_origin'])
        run = subprocess.run([sys.executable, '-I', str(script)], input=request, capture_output=True, check=True)
        reply = json.loads(run.stdout)
        self.assertFalse(run.stderr)
        self.assertEqual(reply['status'], 'ok', reply)
        cut = reply['result']
        self.assertTrue(cut['brep_b64'])
        self.assertAlmostEqual(cut['volume_mm3'], 1_973_000, delta=1e-8)
        self.assertAlmostEqual(mesh_volume(cut['mesh']), 1_973_000, delta=1e-8)
        self.assertTrue(reply['native_boundary'])

    def test_enclosed_cavity_native(self):
        fixture = pathlib.Path(__file__).with_name('generate_difference.py')
        script = fixture.resolve().parents[1] / 'occt-worker.py'
        request = subprocess.check_output([sys.executable, '-I', str(fixture), 'enclosed_cavity'])
        run = subprocess.run([sys.executable, '-I', str(script)], input=request, capture_output=True, check=True)
        reply = json.loads(run.stdout)
        self.assertEqual(reply['status'], 'ok', reply)
        self.assertAlmostEqual(reply['result']['volume_mm3'], 1_984_000, delta=1e-8)
        self.assertAlmostEqual(mesh_volume(reply['result']['mesh']), 1_984_000, delta=1e-8)


def mesh_volume(mesh):
    vertices = mesh['positions_mm']
    volume = 0
    for i, j, k in mesh['triangles']:
        a, b, c = vertices[i], vertices[j], vertices[k]
        volume += sum(a[q] * (b[(q + 1) % 3] * c[(q + 2) % 3] - b[(q + 2) % 3] * c[(q + 1) % 3])
                      for q in range(3)) / 6
    return volume

# Load the actual worker so test seams can observe the native call in-process.
def load_worker():
    import importlib.util
    path = pathlib.Path(__file__).resolve().parents[1] / 'occt-worker.py'
    spec = importlib.util.spec_from_file_location('occt_worker', path)
    worker = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(worker)
    return worker


class NativeBoundaryTests(unittest.TestCase):
    def test_tolerance_weld_before_closed_topology(self):
        worker = load_worker()
        faces = [
            ([(0., 0., 0.), (1., 0., 0.), (0., 1., 0.)], [(0, 2, 1)], [0, 1, 2]),
            ([(3e-8, 0., 0.), (1., 0., 0.), (0., 0., 1.)], [(0, 1, 2)], [0, 1, 3]),
            ([(0., 0., 0.), (0., 0., 1.), (0., 1., 0.)], [(0, 1, 2)], [0, 3, 2]),
            ([(1., 0., 0.), (0., 1., 0.), (0., 0., 1.)], [(0, 1, 2)], [1, 2, 3]),
        ]
        mesh = worker.indexed(faces, weld=1e-7)
        self.assertEqual(len(mesh['positions_mm']), 4)
        self.assertAlmostEqual(mesh_volume(mesh), 1 / 6)
        faces[1][0][0] = (2e-7, 0., 0.)
        with self.assertRaises(worker.Reject):
            worker.indexed(faces, weld=1e-7)

    def test_cut_operand_centring(self):
        from unittest.mock import patch
        from generate_difference import request_for

        worker = load_worker()
        request = request_for('flush_origin')
        for operand in (request['a'], request['b']):
            operand['offset_mm'][0] = 5_000_000
            for point in operand['profile']['xy_mm']:
                point[0] = str(int(point[0]) + 5_000_000)
        source_a, *_, box_a = worker.read_operand(request['a'])
        source_b, *_, box_b = worker.read_operand(request['b'])
        real_read = worker.read_operand
        real_cut = worker.configured_cut
        calls = []

        def read(operand):
            source = source_a if operand['id'] == 'plate' else source_b
            return (source, *real_read(operand)[1:])

        def at_build(a, b):
            calls.append(True)
            self.assertFalse(a.IsPartner(source_a))
            self.assertFalse(b.IsPartner(source_b))
            aa, bb = worker.bounds(a), worker.bounds(b)
            for axis in range(3):
                self.assertAlmostEqual((min(aa[axis], bb[axis]) + max(aa[axis+3], bb[axis+3])) / 2, 0, delta=1e-5)
            for actual, expected in [(aa, (-200, -125, -10, 200, 125, 10)),
                                     (bb, (-200, -125, -10, -140, -80, 10))]:
                for x, y in zip(actual, expected):
                    self.assertAlmostEqual(x, y, delta=1e-5)

        with patch.object(worker, 'read_operand', read), patch.object(worker, 'configured_cut', wraps=real_cut) as configure:
            a, b, _ = worker.prepare(request)
            worker.cut_once(a, b, before_build=at_build)
            self.assertEqual(configure.call_count, 1)
            a, b, _ = worker.prepare(request, recenter=False)
            with self.assertRaises(AssertionError):
                worker.cut_once(a, b, before_build=at_build)
        self.assertEqual(len(calls), 2)
        self.assertEqual(worker.bounds(source_a), box_a)
        self.assertEqual(worker.bounds(source_b), box_b)


if __name__ == '__main__':
    unittest.main()
