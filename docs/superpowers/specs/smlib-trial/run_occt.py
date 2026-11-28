"""Run an unchanged SMLib-trial script on Open Cascade.
Usage:  python -I run_occt.py [--fuzzy MM] script.py [script args]
 e.g.   python -I run_occt.py run_b.py 100        (writes out/occt_kernel_b.json instead of kernel_b.json)
        python -I run_occt.py --fuzzy 1e-4 run_b.py 100   (writes out/occt_f1e-04_kernel_b.json)
It installs occt_ops as `usd_brep`, redirects the result file name so the SMLib results are never overwritten, and
shrinks the JSON that is written to a summary (+ failing rows) so out/ stays small."""
import builtins
import json
import os
import runpy
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
import occt_ops  # noqa: E402

args = sys.argv[1:]
tag = 'occt_'
if args and args[0] == '--adaptive':
    occt_ops.ADAPTIVE = True
    tag = 'occt_adaptive_'
    args = args[1:]
if args and args[0] == '--fuzzy':
    occt_ops.FUZZY = float(args[1])
    tag = f'occt_f{occt_ops.FUZZY:.0e}_'
    args = args[2:]
script, rest = args[0], args[1:]
sys.modules['usd_brep'] = occt_ops

_open = builtins.open


def _redirect(path):
    p = str(path).replace('\\', '/')
    d, f = os.path.split(p)
    if d.endswith('/out') and f.startswith('kernel_') and f.endswith('.json'):
        return os.path.join(d, tag + f)
    return path


def _open2(path, mode='r', *a, **k):
    if 'w' in mode:
        path = _redirect(path)
    return _open(path, mode, *a, **k)


builtins.open = _open2
_dump = json.dump


def _med(x):
    x = sorted(x)
    return x[len(x) // 2] if x else None


def _compact(obj, fp, **k):
    if isinstance(obj, list):  # run_kernel's list of rows
        bad = [r for r in obj if not r.get('ok', True)]
        ms = sorted(r['boolean_ms'] for r in obj if 'boolean_ms' in r)
        obj = dict(n=len(obj), ok=sum(1 for r in obj if r.get('ok')), failed_rows=[dict(id=r['id'], rel_err=r.get('rel_err'), error=r.get('error'), manifold=r.get('manifold_solid'), mesh_nonmanifold=(r.get('mesh') or {}).get('nonmanifold_edges'), mesh_coincident=(r.get('mesh') or {}).get('coincident_opposite_planes')) for r in bad[:6]], silent_wrong_volume=sum(1 for r in obj if abs(r.get('rel_err', 0)) > 1e-6 and r.get('manifold_solid')),
                   rel_err_max=max((abs(r['rel_err']) for r in obj if 'rel_err' in r), default=None),
                   boolean_ms_median=ms[len(ms) // 2] if ms else None, boolean_ms_p95=ms[int(len(ms) * .95)] if ms else None,
                   tess_ms_median=_med([r['tess_ms'] for r in obj if 'tess_ms' in r]), tris_median=_med([r['mesh']['tris'] for r in obj if 'mesh' in r]),
                   mesh_vol_err_median=_med([abs(r['mesh_volume_rel_err']) for r in obj if 'mesh_volume_rel_err' in r]),
                   mesh_vol_err_max=max((abs(r['mesh_volume_rel_err']) for r in obj if 'mesh_volume_rel_err' in r), default=None),
                   not_manifold=sum(1 for r in obj if r.get('manifold_solid') is False), errors=sum(1 for r in obj if 'error' in r),
                   mesh_defect_rows=sum(1 for r in obj if 'mesh' in r and (r['mesh']['open_edges'] or r['mesh']['nonmanifold_edges'] or r['mesh']['zero_area_tris'] or r['mesh']['coincident_opposite_planes'])))
    elif isinstance(obj, dict) and 'rows' in obj and 'summary' in obj:
        obj = dict(summary=obj['summary'])
    elif isinstance(obj, dict) and isinstance(obj.get('rows'), dict):  # run_f
        obj = {k: v for k, v in obj.items() if k != 'rows'}
    return _dump(obj, fp, **k)


json.dump = _compact
sys.argv = [script] + rest
try:
    runpy.run_path(os.path.join(HERE, script), run_name='__main__')
finally:
    if occt_ops.BOOL_LOG:
        import statistics as st
        ms = sorted(m for _, m in occt_ops.BOOL_LOG)
        print(f'[occt] {len(ms)} booleans, median {st.median(ms):.2f} ms, p95 {ms[int(len(ms) * .95)]:.2f} ms, max {ms[-1]:.1f} ms')
