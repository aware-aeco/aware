"""`exec` -- run author-time `bpy` code headlessly (#692).

Run: blender -b -P exec.py -- @inputs.json
     inputs: {"code": "def run(ctx): ...", "args": {...}, "output-dir": "...",
              "ifc-path" | "blend-path": "...", "preset": "realistic"}

The code was written when the workflow was authored and is pinned in the
approved lock (the runtime refuses a templated `code`), so this wrapper runs it
as given. It:

1. compiles the code as `<aware-exec>`, so a syntax error fails before any
   scene work is paid for;
2. prepares the scene -- `ifc-path`: an empty factory scene, the IFC imported
   with its properties, the `preset` look and default lighting; `blend-path`:
   that file opened as-is; neither: an empty factory scene (no default cube,
   camera or light);
3. runs the module body in a fresh namespace, requires a callable `run`, calls
   `run(ctx)` (see `_exec_ctx`) and frames `{result, outputs, frames}` through
   `_result`.

Failures stay structured: a compile or runtime exception becomes
`{ok: false, code: "exec-error", message, traceback, line}` with line numbers
counted in the authored code, and a result over 256 KiB becomes
`result-too-large`, telling the author to write a file and `ctx.add_output` it.
"""

import json
import linecache
import os
import sys
import traceback

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))

import bpy  # noqa: E402

import _exec_ctx  # noqa: E402
import _ifc_import  # noqa: E402
import _looks  # noqa: E402
import _progress  # noqa: E402
import _result  # noqa: E402
import _stage  # noqa: E402

ERR_EXEC = "exec-error"
ERR_RESULT_TOO_LARGE = "result-too-large"

# The serialized `result` cap. The framed payload is read whole from stdout and
# copied into the run trace, so an unbounded return would cost memory twice and
# make the trace unusable. Anything bigger belongs in a file.
MAX_RESULT_BYTES = 256 * 1024

CODE_FILENAME = "<aware-exec>"


def _reset_to_empty_scene() -> None:
    """Factory settings with no startup objects -- deterministic regardless of the user's startup file."""
    bpy.ops.wm.read_factory_settings(use_empty=True)


def prepare_scene(inputs: dict) -> dict:
    """Stage the scene the code starts from. Returns a receipt of what was done."""
    ifc_path = inputs.get("ifc-path") or None
    blend_path = inputs.get("blend-path") or None
    if ifc_path and blend_path:
        raise _result.AwareBlenderError(
            _result.ERR_INVALID_INPUTS,
            "`ifc-path` and `blend-path` are mutually exclusive; give at most one",
        )

    if blend_path:
        if not os.path.exists(str(blend_path)):
            raise _result.AwareBlenderError(
                _result.ERR_BLEND_UNREADABLE, f".blend not found: {blend_path}"
            )
        _progress.emit("import", message=f"opening {os.path.basename(str(blend_path))}")
        bpy.ops.wm.open_mainfile(filepath=os.path.abspath(str(blend_path)))
        return {"source": "blend-path", "path": os.path.abspath(str(blend_path))}

    if not ifc_path:
        _reset_to_empty_scene()
        return {"source": "empty"}

    if not os.path.exists(str(ifc_path)):
        raise _result.AwareBlenderError(
            _result.ERR_IFC_UNREADABLE, f"IFC not found: {ifc_path}"
        )
    preset = str(inputs.get("preset") or "realistic")
    if preset not in _looks.PRESETS:
        raise _result.AwareBlenderError(
            _result.ERR_INVALID_INPUTS,
            f"unknown preset `{preset}`; expected one of {sorted(_looks.PRESETS)}",
        )
    _reset_to_empty_scene()
    _progress.emit("import", message=f"importing {os.path.basename(str(ifc_path))}")
    imported = _ifc_import.import_ifc(
        str(ifc_path), unit_scale=float(inputs.get("unit-scale", 1.0))
    )
    _progress.emit("look", message=f"applying the {preset} look")
    look = _looks.apply_look(preset)
    environment = _stage.setup_world(_stage.DEFAULT_ENVIRONMENT)
    _stage.setup_key_light()
    return {
        "source": "ifc-path",
        "path": os.path.abspath(str(ifc_path)),
        "imported": imported.get("imported"),
        "skipped-count": imported.get("skipped-count"),
        "preset": preset,
        "assigned": look.get("assigned"),
        "environment": environment,
    }


def _authored_line(tb) -> int | None:
    """The innermost line of the AUTHORED code in a traceback, if any."""
    line = None
    for frame in traceback.extract_tb(tb):
        if frame.filename == CODE_FILENAME:
            line = frame.lineno
    return line


def _exec_error(exc: BaseException, code: str) -> _result.AwareBlenderError:
    """An `exec-error` naming the line in the author's code, with an authored-only traceback."""
    if isinstance(exc, SyntaxError) and exc.filename == CODE_FILENAME:
        line = exc.lineno
        message = f"SyntaxError: {exc.msg} (line {line})"
        text = "".join(traceback.format_exception_only(type(exc), exc))
    else:
        tb = exc.__traceback__
        # Drop this wrapper's own frames: the author reads their code, not ours.
        while tb is not None and tb.tb_frame.f_code.co_filename != CODE_FILENAME:
            tb = tb.tb_next
        if tb is None:
            tb = exc.__traceback__
        line = _authored_line(tb)
        where = f" (line {line})" if line is not None else ""
        message = f"{type(exc).__name__}: {exc}{where}"
        text = "".join(traceback.format_exception(type(exc), exc, tb))
    details = {"traceback": text}
    if line is not None:
        details["line"] = line
    return _result.AwareBlenderError(
        ERR_EXEC,
        message,
        "line numbers count from the first line of `code`",
        **details,
    )


def compile_code(code: str):
    """Compile the authored code -- a syntax error fails before any scene work is paid for."""
    # Registered so tracebacks quote the authored source lines, not "<unknown>".
    linecache.cache[CODE_FILENAME] = (
        len(code),
        None,
        code.splitlines(keepends=True),
        CODE_FILENAME,
    )
    try:
        return compile(code, CODE_FILENAME, "exec")
    except Exception as exc:  # noqa: BLE001 - surfaced as exec-error
        raise _exec_error(exc, code) from exc


def load_run(compiled, code: str):
    """Execute the module body in a fresh namespace and return its `run` callable.

    Runs AFTER the scene is prepared, so top-level statements see the staged
    scene rather than the one the reset is about to discard.
    """
    namespace = {"__name__": "aware_exec", "__file__": CODE_FILENAME, "bpy": bpy}
    try:
        exec(compiled, namespace)  # noqa: S102 - the approved, lock-pinned program
    except Exception as exc:  # noqa: BLE001 - surfaced as exec-error
        raise _exec_error(exc, code) from exc
    run = namespace.get("run")
    if not callable(run):
        raise _result.AwareBlenderError(
            ERR_EXEC,
            "the code defines no callable `run(ctx)`",
            "define `def run(ctx): ...` at the top level of `code`",
        )
    return run


def main(inputs: dict) -> dict:
    code = inputs.get("code")
    if not isinstance(code, str) or not code.strip():
        raise _result.AwareBlenderError(
            _result.ERR_INVALID_INPUTS,
            "required input `code` is missing or empty -- Python source defining `run(ctx)`",
        )
    output_dir = os.path.abspath(str(_result.require(inputs, "output-dir")))
    args = inputs.get("args") or {}
    if not isinstance(args, dict):
        raise _result.AwareBlenderError(
            _result.ERR_INVALID_INPUTS, "`args` must be an object"
        )

    compiled = compile_code(code)
    scene_receipt = prepare_scene(inputs)
    _progress.emit("setup", message="running the workflow's code")
    run = load_run(compiled, code)
    ctx = _exec_ctx.ExecContext(args, output_dir)
    try:
        result = run(ctx)
    except _result.AwareBlenderError:
        raise
    except Exception as exc:  # noqa: BLE001 - surfaced as exec-error
        raise _exec_error(exc, code) from exc

    try:
        serialized = json.dumps(result, sort_keys=True)
    except (TypeError, ValueError) as exc:
        raise _result.AwareBlenderError(
            ERR_EXEC,
            f"run(ctx) returned a value JSON cannot carry: {exc}",
            "return plain dicts, lists, strings, numbers, booleans or None",
        ) from exc
    size = len(serialized.encode("utf-8"))
    if size > MAX_RESULT_BYTES:
        raise _result.AwareBlenderError(
            ERR_RESULT_TOO_LARGE,
            f"run(ctx) returned {size} bytes of JSON; the limit is {MAX_RESULT_BYTES}",
            "write large data to a file in ctx.output_dir and register it with "
            "ctx.add_output(path, 'file'), returning a summary instead",
            outputs=ctx.outputs,
            frames=ctx.frames,
        )

    return {
        "result": result,
        "outputs": ctx.outputs,
        "frames": ctx.frames,
        "scene": scene_receipt,
    }


if __name__ == "__main__":
    _result.run(main)
