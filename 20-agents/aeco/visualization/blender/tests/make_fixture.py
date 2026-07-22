"""Regenerate the reference IFC from the committed scene JSON via the aware CLI.

The IFC is never committed: `ifc.write` is deterministic, so regenerating it at
test time gives a stable asset without a binary in git.
"""

import argparse
import json
import subprocess
import sys
import tempfile
from pathlib import Path

HERE = Path(__file__).resolve().parent


def build_fixture(aware_bin: str, out_ifc: Path) -> dict:
    """Run `ifc.write` over the committed scene; return the CLI's receipt."""
    scene = json.loads((HERE / "fixture-scene.json").read_text(encoding="utf-8"))
    payload = {"scene": scene, "output-path": str(out_ifc)}

    with tempfile.NamedTemporaryFile(
        "w", suffix=".json", delete=False, encoding="utf-8"
    ) as handle:
        json.dump(payload, handle)
        args_path = Path(handle.name)

    try:
        proc = subprocess.run(
            [aware_bin, "agent", "invoke", "ifc", "write", "--inputs", f"@{args_path}"],
            capture_output=True,
            text=True,
            check=False,
        )
    finally:
        args_path.unlink(missing_ok=True)

    if proc.returncode != 0:
        raise RuntimeError(f"ifc.write failed ({proc.returncode}):\n{proc.stderr}")
    if not out_ifc.exists():
        raise RuntimeError(f"ifc.write reported success but {out_ifc} is missing")
    return json.loads(proc.stdout[proc.stdout.index("{"):])


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--aware-bin", default="aware", help="path to the aware binary")
    parser.add_argument("--out", required=True, type=Path, help="IFC output path")
    opts = parser.parse_args()

    receipt = build_fixture(opts.aware_bin, opts.out)
    print(f"fixture: {opts.out} ({len(receipt.get('emitted', []))} elements)")
    return 0


if __name__ == "__main__":
    sys.exit(main())
