"""Build paired local BRep and indexed prism meshes from dimensioned fixtures."""

import argparse
import base64
import io
import json
from decimal import Decimal, localcontext
from pathlib import Path

from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeFace, BRepBuilderAPI_MakePolygon
from OCP.BRepPrimAPI import BRepPrimAPI_MakePrism
from OCP.BRepTools import BRepTools
from OCP.TopTools import TopTools_FormatVersion
from OCP.gp import gp_Pnt, gp_Vec


def native_prism(polygon, low, high):
    wire = BRepBuilderAPI_MakePolygon()
    for x, y in polygon:
        wire.Add(gp_Pnt(x, y, low))
    wire.Close()
    if not wire.IsDone():
        raise ValueError('could not build prism wire')
    face = BRepBuilderAPI_MakeFace(wire.Wire(), True)
    if not face.IsDone():
        raise ValueError('could not build prism cap')
    prism = BRepPrimAPI_MakePrism(face.Face(), gp_Vec(0, 0, high - low), True, True)
    if not prism.IsDone():
        raise ValueError('could not build prism')
    stream = io.BytesIO()
    BRepTools.Write_s(prism.Shape(), stream, False, False, TopTools_FormatVersion.TopTools_FormatVersion_CURRENT)
    return base64.standard_b64encode(stream.getvalue()).decode('ascii')


def indexed_prism(polygon, low, high):
    count = len(polygon)
    positions = [[x, y, z] for z in (low, high) for x, y in polygon]
    triangles = []
    for index in range(1, count - 1):
        triangles.extend([[0, index + 1, index], [count, count + index, count + index + 1]])
    for index in range(count):
        following = (index + 1) % count
        triangles.extend([[index, following, following + count], [index, following + count, index + count]])
    return {'positions_mm': positions, 'triangles': triangles}


def operand(identity, xy, z, offset):
    polygon = [(Decimal(x), Decimal(y)) for x, y in xy]
    low, high = map(Decimal, z)
    offset = tuple(map(Decimal, offset))
    if len(polygon) < 3 or high <= low or len(set(polygon)) != len(polygon):
        raise ValueError('expected a nonempty prism with distinct profile vertices')
    with localcontext() as context:
        context.prec = 128
        for index, (x, y) in enumerate(polygon):
            following_x, following_y = polygon[(index + 1) % len(polygon)]
            for j, (px, py) in enumerate(polygon):
                if j not in (index, (index + 1) % len(polygon)):
                    if (following_x - x) * (py - y) <= (following_y - y) * (px - x):
                        raise ValueError('expected a strictly convex counterclockwise polygon')
        profile = {
            'xy_mm': [[format(x + offset[0], 'f'), format(y + offset[1], 'f')] for x, y in polygon],
            'z_mm': [format(level + offset[2], 'f') for level in (low, high)],
        }
    points = [(float(x), float(y)) for x, y in polygon]
    low, high = float(low), float(high)
    return {'id': identity, 'offset_mm': list(map(float, offset)), 'profile': profile,
            'brep_b64': native_prism(points, low, high), 'mesh': indexed_prism(points, low, high)}


def request_for(case):
    fixture = json.loads(Path(__file__).with_name('flush-stress.json').read_bytes())
    plate, clip = fixture['plate_xy_mm'], fixture['clip_xy_mm']
    z = clip_z = fixture['z_mm']
    origin = [0, 0, 0]
    axes = [[1, 0, 0], [0, 1, 0], [0, 0, 1]]
    a_offset = b_offset = ['0', '0', '0']
    if case in ('flush_large_member_offset', 'flush_distinct_operand_origins', 'rotated_frame'):
        a_offset = b_offset = ['5000000', '0', '0']
    if case == 'flush_distinct_operand_origins':
        plate = [(str(Decimal(x) + Decimal('0.125')), y) for x, y in plate]
        b_offset = ['5000000.125', '0', '0']
    if case == 'rotated_frame':
        origin = [60000000, -8000000, 25000]
        axes = [[0, 1, 0], [-1, 0, 0], [0, 0, 1]]
    if case == 'flush_stress':
        a_offset = b_offset = fixture['offset_mm']
    if case == 'far_disjoint':
        a_offset = ['5000000', '0', '0']
        b_offset = ['-5000000', '0', '0']
    if case == 'enclosed_cavity':
        clip = [['50', '50'], ['90', '50'], ['90', '100'], ['50', '100']]
        clip_z = ['6', '14']
    a, b = operand('plate', plate, z, a_offset), operand('clip', clip, clip_z, b_offset)
    if case == 'swapped_operands':
        a, b = b, a
    return {'schema': 'aware.geometry.exec.request/v1', 'op': 'boolean', 'kind': 'difference',
            'frame': {'units': 'mm', 'origin_mm': origin, 'axes': axes},
            'chord_height_mm': 0.01, 'a': a, 'b': b}


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('case', choices=['flush_origin', 'flush_large_member_offset', 'flush_distinct_operand_origins',
                                         'rotated_frame', 'flush_stress', 'far_disjoint', 'swapped_operands',
                                         'enclosed_cavity'])
    print(json.dumps(request_for(parser.parse_args().case), allow_nan=False))
