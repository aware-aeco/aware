
# This suffix is embedded only in the geometry-test-faults executable.
def fault_trace(event, **fields):
    with open(os.environ['AWARE_GEOMETRY_TEST_TRACE'], 'a', encoding='utf-8') as stream:
        print(json.dumps({'event': event, **fields}), file=stream)


def substitute_unchanged(cut, a, _b):
    valid = check_solid(a) and max(map(abs, bounds(a))) <= EXTENT
    check(valid)
    cut_volume = signed_volume(cut)
    wrong_volume = signed_volume(a)
    check(0 < cut_volume < wrong_volume)
    fault_trace('post_cut_swap', cut_volume_mm3=cut_volume, injected_volume_mm3=wrong_volume, valid=valid)
    return a


def substitute_pocket(cut, a, b):
    wrong = cut_once(a, translate(b, (5, 0, 0)))
    check(check_solid(wrong, empty=True) and bounds(cut) == bounds(wrong))
    check(abs(signed_volume(cut) - signed_volume(wrong)) < 1e-8)
    from OCP.BRepClass3d import BRepClass3d_SolidClassifier
    from OCP.TopAbs import TopAbs_IN
    from OCP.gp import gp_Pnt

    box = bounds(a)
    # This injected case is the enclosed-cavity fixture at the origin.
    anchor = [-box[i] for i in range(3)]
    def occupied(shape, member):
        point = gp_Pnt(*(v - anchor[i] for i, v in enumerate(member)))
        return BRepClass3d_SolidClassifier(shape, point, 1e-7).State() == TopAbs_IN

    common = ((70, 75, 10), (250, 200, 10))
    check([[occupied(shape, p) for p in common] for shape in (cut, wrong)] == [[False, True]] * 2)
    check(not occupied(cut, (52, 75, 10)) and occupied(wrong, (52, 75, 10)))
    fault_trace('shifted_pocket', valid=True, volume_mm3=signed_volume(wrong), fixed_witnesses_preserved=True)
    return wrong


def shift_tessellation(mesh):
    mesh['positions_mm'] = [(x + 1, y, z) for x, y, z in mesh['positions_mm']]
    fault_trace('shifted_tessellation')


def native_fault_reply(reply, protocol):
    setting = os.environ.get('AWARE_GEOMETRY_TEST_FAULT', '')
    route, _, fault = setting.partition(':')
    if route != 'native' or '--preflight' in sys.argv:
        write_reply(reply, protocol)
        return
    check(reply.get('status') == 'ok')
    fields = {'route': route, 'fault': fault}
    if fault == 'nonzero':
        write_reply(reply, protocol)
        protocol.flush()
        fault_trace('valid_reply_written', **fields)
        raise SystemExit(7)
    if fault == 'malformed':
        print('{', file=protocol)
    elif fault == 'omitted-field':
        del reply['relation']
        write_reply(reply, protocol)
    elif fault == 'unknown-field':
        reply['extra'] = True
        write_reply(reply, protocol)
    else:
        write_reply(reply, protocol)
        return
    fault_trace('reply_fault', **fields)


if __name__ == '__main__':
    fault = os.environ.get('AWARE_GEOMETRY_TEST_FAULT')
    main({'exact:wrong-solid': substitute_unchanged, 'exact:shifted-pocket': substitute_pocket}.get(fault),
         shift_tessellation if fault == 'exact:shifted-tessellation' else None, native_fault_reply)
