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
        self.assertAlmostEqual(mesh_volume(reply['native_boundary']), 1_973_000, delta=1e-8)

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


if __name__ == '__main__':
    unittest.main()
