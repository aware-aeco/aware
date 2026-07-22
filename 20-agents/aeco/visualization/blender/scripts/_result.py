"""Result and error protocol shared by every aware-blender command script.

Blender writes progress, warnings and render statistics to stdout, so a bare
`print(json.dumps(...))` is not recoverable by the caller. Every script instead
frames its payload between sentinels; the transport slices between them.

Exit codes: 0 on success, 1 on a named failure. A named failure still emits a
framed payload, so the caller always gets structured data.
"""

import json
import sys
import traceback

RESULT_BEGIN = "<<<AWARE_RESULT>>>"
RESULT_END = "<<<AWARE_RESULT_END>>>"

# The named error taxonomy. The transport surfaces `code` verbatim, so these
# strings are contract: renaming one is a breaking change.
ERR_IFCOPENSHELL_MISSING = "ifcopenshell-missing"
ERR_IFC_UNREADABLE = "ifc-unreadable"
ERR_IFC_EMPTY = "ifc-empty"
ERR_BLEND_UNREADABLE = "blend-unreadable"
ERR_INVALID_INPUTS = "invalid-inputs"
ERR_RENDER_FAILED = "render-failed"
ERR_UNEXPECTED = "unexpected-error"

# Install hint reused by the ifcopenshell guard; the design doc requires the
# exact one-liner to travel with the error.
IFCOPENSHELL_HINT = (
    "Install it into Blender's bundled Python:\n"
    '  blender -b --python-expr "import sys,subprocess; '
    "subprocess.check_call([sys.executable,'-m','pip','install','ifcopenshell'])\""
)


class AwareBlenderError(Exception):
    """A failure with a stable machine-readable code and an actionable hint."""

    def __init__(self, code: str, message: str, hint: str = "", **details):
        super().__init__(message)
        self.code = code
        self.message = message
        self.hint = hint
        self.details = details

    def payload(self) -> dict:
        out = {"ok": False, "code": self.code, "message": self.message}
        if self.hint:
            out["hint"] = self.hint
        out.update(self.details)
        return out


def emit(payload: dict) -> None:
    """Frame a payload on stdout so the transport can recover it."""
    sys.stdout.flush()
    print(RESULT_BEGIN)
    print(json.dumps(payload, indent=2, sort_keys=True))
    print(RESULT_END)
    sys.stdout.flush()


def parse_args(argv: list[str]) -> dict:
    """Read the JSON blob that follows `--` in a `blender -b -P script.py -- {...}` call.

    Accepts either inline JSON or `@path/to/file.json` for payloads that would
    otherwise hit the command-line length limit.
    """
    if "--" not in argv:
        raise AwareBlenderError(
            ERR_INVALID_INPUTS,
            "no `--` separator found; expected `blender -b -P script.py -- <json>`",
        )
    rest = argv[argv.index("--") + 1:]
    if not rest:
        raise AwareBlenderError(ERR_INVALID_INPUTS, "no JSON payload after `--`")

    raw = rest[0]
    if raw.startswith("@"):
        try:
            with open(raw[1:], encoding="utf-8") as handle:
                raw = handle.read()
        except OSError as exc:
            raise AwareBlenderError(
                ERR_INVALID_INPUTS, f"cannot read inputs file {raw[1:]}: {exc}"
            ) from exc
    try:
        parsed = json.loads(raw)
    except json.JSONDecodeError as exc:
        raise AwareBlenderError(
            ERR_INVALID_INPUTS, f"inputs are not valid JSON: {exc}"
        ) from exc
    if not isinstance(parsed, dict):
        raise AwareBlenderError(ERR_INVALID_INPUTS, "inputs must be a JSON object")
    return parsed


def require(inputs: dict, key: str) -> object:
    """Fetch a required input or raise the named invalid-inputs error."""
    if key not in inputs or inputs[key] in (None, ""):
        raise AwareBlenderError(
            ERR_INVALID_INPUTS, f"required input `{key}` is missing"
        )
    return inputs[key]


def run(main_fn) -> None:
    """Entry-point wrapper: run `main_fn(inputs) -> dict`, frame whatever happens.

    Every command script ends with `_result.run(main)`. Unexpected exceptions are
    caught and framed too, so the caller never has to scrape a Python traceback
    out of Blender's log to find out what went wrong.
    """
    try:
        inputs = parse_args(sys.argv)
        payload = main_fn(inputs)
        payload.setdefault("ok", True)
        emit(payload)
        sys.exit(0)
    except AwareBlenderError as exc:
        emit(exc.payload())
        sys.exit(1)
    except Exception as exc:  # noqa: BLE001 - last resort, must stay structured
        emit(
            {
                "ok": False,
                "code": ERR_UNEXPECTED,
                "message": f"{type(exc).__name__}: {exc}",
                "traceback": traceback.format_exc(),
            }
        )
        sys.exit(1)
