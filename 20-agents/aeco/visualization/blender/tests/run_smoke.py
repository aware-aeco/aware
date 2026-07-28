"""Headless smoke test for the blender agent -- the Phase 1 gate.

Chain: fixture scene JSON -> `ifc.write` (the real `aware` CLI) -> IFC ->
`scene.import` -> `scene.info` -> `scene.apply-look` -> `render.still` ->
`render.turntable`, asserting the design doc's verification criteria
(`docs/superpowers/specs/2026-07-22-blender-visualization-agent-design.md`)
at each hop against a real Blender, not a mock.

Not run in CI -- CI has no Blender. Run it by hand on a Blender-equipped
machine:

    python tests/run_smoke.py \
        --blender "/c/Program Files/Blender Foundation/Blender 5.2/blender.exe" \
        --aware-bin ./cli/target/debug/aware.exe

Exit codes: 0 = SMOKE PASS. 1 = a step failed or asserted wrong (SMOKE FAIL,
detail on stderr). 77 = SKIP -- no usable Blender was found; this is the
conventional "test could not run" code (autotools, and pytest's
`--report=skip`-style consumers honour it too) precisely so a CI wrapper or
a human glancing at `$?` does not mistake "nothing to check here" for a red
gate.

Design note -- the flat-colour check runs in-process, not through Blender.
`render.still`'s own verification criterion is "PNG exists and is not a
single flat colour" (the design doc's literal wording). The obvious way to
check that is to load the PNG through Blender and inspect `image.pixels`,
but that costs an entire extra Blender launch (Blender's startup alone is a
meaningful fraction of this harness's total wall time) just to answer a
question a ~120-line pure-stdlib PNG reader (zlib + struct, no Pillow)
answers just as correctly. Blender's own PNG writer only ever produces the
subset this decoder supports (8-bit, non-interlaced, colour type 2/6 in
practice) -- confirmed against a real render while writing this harness, see
`decode_png`'s docstring. Because a hand-rolled decoder that has never been
proven to work is worse than no check at all, `self_test_flat_detector()`
below manufactures a known-flat and a known-not-flat PNG from raw bytes (no
Blender, no real render needed) and asserts the decoder calls both
correctly -- and it runs at the top of every single invocation of this
harness, not just once during development.
"""

import argparse
import json
import math
import re
import shutil
import struct
import subprocess
import sys
import tempfile
import time
import traceback
import zlib
from pathlib import Path

HERE = Path(__file__).resolve().parent
SCRIPTS = HERE.parent / "scripts"

sys.path.insert(0, str(SCRIPTS))
sys.path.insert(0, str(HERE))

import _result  # noqa: E402 -- no bpy import, safe for plain Python (see scripts/_result.py)
import make_fixture  # noqa: E402

# The fixture's known inventory -- verified by actually running the chain
# against this repo's current scripts, not copied from the design draft.
# See scene_import.py / scene_info.py receipts captured while writing this
# harness: by-class {IfcBeam: 4, IfcColumn: 4, IfcMember: 1}, by-material
# {A36: 1, A500-GR.B: 4, A992: 4}, by-storey {Storey: 9} -- all steel, so
# scene.apply-look's `assigned` collapses to a single family regardless of
# which preset is requested.
EXPECTED_CLASSES = {"IfcBeam": 4, "IfcColumn": 4, "IfcMember": 1}
EXPECTED_MATERIALS = {"A36": 1, "A500-GR.B": 4, "A992": 4}
EXPECTED_STOREYS = {"Storey": 9}
EXPECTED_ELEMENT_COUNT = 9

# Small but not degenerate -- big enough that a genuinely broken render
# (nothing drawn, wrong camera, a shader error) still shows real variety,
# small enough that this harness stays fast to re-run.
STILL_WIDTH, STILL_HEIGHT = 640, 360
TURNTABLE_WIDTH, TURNTABLE_HEIGHT = 480, 270
TURNTABLE_DURATION_SECONDS = 1
TURNTABLE_FPS = 12
MIN_MP4_BYTES = 10_000

# Hand-measured against the committed 9-element fixture at STILL_WIDTH x
# STILL_HEIGHT. The near web of beam B1 runs diagonally through this sample
# line. A clean web is a smooth luminance ramp; the Blender 5.2 world-sun
# shadow alias from #314 raises this high-frequency residual above 5.5.
MAX_WEB_BANDING_SCORE = 2.0
WEB_BANDING_REGRESSION_VERSIONS = {(5, 2)}

# Verified location from the Task 1 toolchain probe; used only when neither
# --blender nor PATH resolves anything.
_KNOWN_BLENDER_WINDOWS = r"C:\Program Files\Blender Foundation\Blender 5.2\blender.exe"


class ChainError(RuntimeError):
    """A step in the command chain failed in a way this harness understood.

    Deliberately a RuntimeError subclass so `make_fixture.build_fixture`'s
    own RuntimeErrors (the aware CLI failing, or succeeding but not writing
    its output file) are caught by the same handler in `main()` without
    needing a second except clause.
    """


# --------------------------------------------------------------------------
# Minimal PNG decode -- just enough to answer "what size is this, and is it
# a single flat colour". See the module docstring for why this runs
# in-process instead of through Blender.
# --------------------------------------------------------------------------


def _iter_png_chunks(data: bytes):
    """Yield (chunk_type, chunk_data) for every chunk in a PNG byte string."""
    if data[:8] != b"\x89PNG\r\n\x1a\n":
        raise ValueError("not a PNG file (bad signature)")
    pos = 8
    n = len(data)
    while pos + 8 <= n:
        (length,) = struct.unpack_from(">I", data, pos)
        ctype = data[pos + 4 : pos + 8]
        start = pos + 8
        cdata = data[start : start + length]
        pos = start + length + 4  # skip the trailing CRC32
        yield ctype, cdata
        if ctype == b"IEND":
            return


def _paeth_predictor(a: int, b: int, c: int) -> int:
    """The PNG spec's Paeth predictor (filter type 4), verbatim."""
    p = a + b - c
    pa, pb, pc = abs(p - a), abs(p - b), abs(p - c)
    if pa <= pb and pa <= pc:
        return a
    if pb <= pc:
        return b
    return c


def decode_png(data: bytes) -> tuple[int, int, int, int, int, bytes]:
    """Decode just enough of a PNG to check "is this a single flat colour".

    Supports the subset Blender's own PNG writer actually produces: colour
    types 0/2/3/4/6 (grey / RGB / palette / grey+alpha / RGBA), bit depth 8
    or 16, no interlacing -- confirmed against a real `render.still` output
    while writing this harness (8-bit RGBA, colour type 6, filter method 0,
    interlace 0). Anything outside that refuses loudly with a clear
    ValueError rather than guessing: a silently-wrong decode would make the
    "not a single flat colour" check meaningless, which is worse than no
    check running at all.

    Returns (width, height, bit_depth, color_type, bytes_per_pixel, raw)
    where `raw` is the fully de-filtered pixel data --
    `height * width * bytes_per_pixel` bytes, row-major, top-to-bottom.
    Each `bytes_per_pixel`-byte slice is one pixel's exact on-disk sample;
    byte-equality between two slices is colour-equality. The explicit sample
    metadata also lets visual assertions decode 8- and 16-bit channels
    without guessing from bytes-per-pixel alone.
    """
    width = height = bit_depth = color_type = None
    idat = bytearray()
    palette = None

    for ctype, cdata in _iter_png_chunks(data):
        if ctype == b"IHDR":
            (width, height, bit_depth, color_type, compression, filt, interlace) = (
                struct.unpack(">IIBBBBB", cdata)
            )
            if compression != 0 or filt != 0:
                raise ValueError("unsupported PNG compression/filter method")
            if interlace != 0:
                raise ValueError("interlaced PNG not supported by this minimal decoder")
            if bit_depth not in (8, 16):
                raise ValueError(f"unsupported PNG bit depth {bit_depth}")
        elif ctype == b"PLTE":
            palette = cdata
        elif ctype == b"IDAT":
            idat += cdata
        elif ctype == b"IEND":
            break

    if width is None:
        raise ValueError("PNG has no IHDR chunk")

    channels = {0: 1, 2: 3, 3: 1, 4: 2, 6: 4}.get(color_type)
    if channels is None:
        raise ValueError(f"unsupported PNG colour type {color_type}")
    if color_type == 3 and bit_depth != 8:
        raise ValueError("unsupported palette PNG bit depth (only 8 handled)")

    bpp = channels * (bit_depth // 8)  # bytes per whole pixel, incl. all channels
    stride = width * bpp

    decompressed = zlib.decompress(bytes(idat))
    if len(decompressed) < height * (stride + 1):
        raise ValueError("PNG pixel data is shorter than IHDR promises")

    out = bytearray(height * stride)
    prev = bytearray(stride)  # "row above" for row 0 is defined as all zeros
    pos = 0
    for y in range(height):
        ftype = decompressed[pos]
        pos += 1
        line = decompressed[pos : pos + stride]
        pos += stride
        cur = bytearray(stride)
        if ftype == 0:  # None
            cur[:] = line
        elif ftype == 1:  # Sub
            for x in range(stride):
                a = cur[x - bpp] if x >= bpp else 0
                cur[x] = (line[x] + a) & 0xFF
        elif ftype == 2:  # Up
            for x in range(stride):
                cur[x] = (line[x] + prev[x]) & 0xFF
        elif ftype == 3:  # Average
            for x in range(stride):
                a = cur[x - bpp] if x >= bpp else 0
                b = prev[x]
                cur[x] = (line[x] + ((a + b) >> 1)) & 0xFF
        elif ftype == 4:  # Paeth
            for x in range(stride):
                a = cur[x - bpp] if x >= bpp else 0
                b = prev[x]
                c = prev[x - bpp] if x >= bpp else 0
                cur[x] = (line[x] + _paeth_predictor(a, b, c)) & 0xFF
        else:
            raise ValueError(f"unsupported PNG filter type {ftype} on row {y}")
        out[y * stride : (y + 1) * stride] = cur
        prev = cur

    if color_type == 3:
        if palette is None:
            raise ValueError("indexed PNG has no PLTE chunk")
        expanded = bytearray(height * width * 3)
        for i in range(height * width):
            idx = out[i]
            expanded[i * 3 : i * 3 + 3] = palette[idx * 3 : idx * 3 + 3]
        return width, height, 8, 2, 3, bytes(expanded)

    return width, height, bit_depth, color_type, bpp, bytes(out)


def count_distinct_pixels(bpp: int, raw: bytes, cap: int = 64) -> int:
    """Count distinct pixel byte-patterns in `raw`, stopping at `cap`.

    The caller only ever needs to know "0, 1, or clearly several" -- an
    early exit keeps this cheap even on a full-resolution production render.
    """
    seen = set()
    for i in range(0, len(raw), bpp):
        seen.add(raw[i : i + bpp])
        if len(seen) >= cap:
            break
    return len(seen)


def inspect_png(path: Path) -> dict:
    """Decode a PNG and report exactly what this smoke test checks."""
    width, height, _bit_depth, _color_type, bpp, raw = decode_png(path.read_bytes())
    distinct = count_distinct_pixels(bpp, raw)
    return {
        "width": width,
        "height": height,
        "distinct-colours-sampled": distinct,
        "is-flat": distinct <= 1,
    }


def _rgb_channels_8bit_scale(
    raw: bytes, index: int, bit_depth: int, color_type: int, bpp: int
) -> tuple[float, float, float]:
    """Read one RGB/RGBA pixel, normalizing 16-bit samples to the 8-bit scale."""
    if color_type not in (2, 6):
        raise ValueError(
            "fixture web banding score requires RGB or RGBA data, "
            f"got PNG colour type {color_type}"
        )
    sample_bytes = bit_depth // 8
    expected_bpp = (3 if color_type == 2 else 4) * sample_bytes
    if bpp != expected_bpp:
        raise ValueError(
            "fixture web banding score got inconsistent PNG sample metadata: "
            f"bit depth {bit_depth}, colour type {color_type}, {bpp} Bpp"
        )
    if bit_depth == 8:
        return tuple(raw[index + channel] for channel in range(3))
    return tuple(
        int.from_bytes(raw[index + channel * 2 : index + channel * 2 + 2], "big") / 257
        for channel in range(3)
    )


def fixture_web_banding_score(path: Path) -> float:
    """High-frequency luminance residual along the fixture's near beam web.

    This is intentionally a visual-fixture assertion, not a source/property
    check: removing the shadow filter must fail on the rendered symptom. The
    diagonal and threshold are hand-derived from `fixture-scene.json` rendered
    at the smoke test's fixed 640x360 framing. A 15-pixel moving mean removes
    the legitimate gradual HDRI reflection; the remaining standard deviation
    measures the regular light/dark dashes from #314.
    """
    width, height, bit_depth, color_type, bpp, raw = decode_png(path.read_bytes())
    if (width, height) != (STILL_WIDTH, STILL_HEIGHT):
        raise ValueError(
            "fixture web banding score requires the smoke test's fixed "
            f"{STILL_WIDTH}x{STILL_HEIGHT} render, got {width}x{height}"
        )
    luminances = []
    for x in range(270, 455):
        y = round(89 + 0.127 * (x - 260) + 3)
        index = (y * width + x) * bpp
        red, green, blue = _rgb_channels_8bit_scale(
            raw, index, bit_depth, color_type, bpp
        )
        luminances.append(0.2126 * red + 0.7152 * green + 0.0722 * blue)

    residuals = []
    for index in range(7, len(luminances) - 7):
        local_mean = sum(luminances[index - 7 : index + 8]) / 15
        residuals.append(luminances[index] - local_mean)
    mean = sum(residuals) / len(residuals)
    return math.sqrt(sum((value - mean) ** 2 for value in residuals) / len(residuals))


# --------------------------------------------------------------------------
# A tiny PNG *encoder* -- test-fixture use only. Builds the known-flat and
# known-not-flat images `self_test_flat_detector` decodes to prove
# `decode_png` actually works, without needing Blender or any real render.
# --------------------------------------------------------------------------


def _png_chunk(ctype: bytes, cdata: bytes) -> bytes:
    return (
        struct.pack(">I", len(cdata))
        + ctype
        + cdata
        + struct.pack(">I", zlib.crc32(ctype + cdata) & 0xFFFFFFFF)
    )


def _encode_rgb8_png(rows: list) -> bytes:
    """Hand-rolled RGB8/filter-0 PNG encoder. `rows` is rows of (r,g,b)."""
    height = len(rows)
    width = len(rows[0])
    raw = bytearray()
    for row in rows:
        raw.append(0)  # filter type 0 (None) for every scanline
        for r, g, b in row:
            raw += bytes((r & 0xFF, g & 0xFF, b & 0xFF))
    ihdr = struct.pack(">IIBBBBB", width, height, 8, 2, 0, 0, 0)
    idat = zlib.compress(bytes(raw), 9)
    return (
        b"\x89PNG\r\n\x1a\n"
        + _png_chunk(b"IHDR", ihdr)
        + _png_chunk(b"IDAT", idat)
        + _png_chunk(b"IEND", b"")
    )


def _encode_rgb16_png(rows: list) -> bytes:
    """Hand-rolled RGB16/filter-0 PNG encoder. `rows` is rows of (r,g,b)."""
    height = len(rows)
    width = len(rows[0])
    raw = bytearray()
    for row in rows:
        raw.append(0)
        for red, green, blue in row:
            raw += struct.pack(">HHH", red, green, blue)
    ihdr = struct.pack(">IIBBBBB", width, height, 16, 2, 0, 0, 0)
    idat = zlib.compress(bytes(raw), 9)
    return (
        b"\x89PNG\r\n\x1a\n"
        + _png_chunk(b"IHDR", ihdr)
        + _png_chunk(b"IDAT", idat)
        + _png_chunk(b"IEND", b"")
    )


def self_test_flat_detector(workdir: Path) -> None:
    """Prove the flat-colour detector distinguishes flat from not-flat.

    "A check that has never failed is not evidence" applies to a hand-rolled
    PNG decoder more than almost anything else in this file. This builds a
    solid-colour PNG and a two-colour PNG entirely from raw bytes -- no
    Blender, no real render -- and asserts `decode_png` + `inspect_png` call
    them correctly, every time this harness runs, before trusting either to
    judge a real render.
    """
    flat_bytes = _encode_rgb8_png([[(40, 40, 40)] * 5 for _ in range(5)])
    striped_bytes = _encode_rgb8_png(
        [[(40, 40, 40) if x < 2 else (200, 40, 40) for x in range(5)] for _ in range(5)]
    )

    flat_path = workdir / "_selftest_flat.png"
    striped_path = workdir / "_selftest_striped.png"
    rgb16_path = workdir / "_selftest_rgb16.png"
    flat_path.write_bytes(flat_bytes)
    striped_path.write_bytes(striped_bytes)
    rgb16_path.write_bytes(_encode_rgb16_png([[(0x1234, 0xABCD, 0xFFFF)]]))

    flat_info = inspect_png(flat_path)
    striped_info = inspect_png(striped_path)
    width, height, bit_depth, color_type, bpp, raw = decode_png(rgb16_path.read_bytes())

    if not (flat_info["is-flat"] and flat_info["width"] == 5 and flat_info["height"] == 5):
        raise AssertionError(
            "flat-image detector self-test FAILED on a deliberately flat "
            f"5x5 PNG -- decoder is not trustworthy: {flat_info}"
        )
    if striped_info["is-flat"] or striped_info["distinct-colours-sampled"] < 2:
        raise AssertionError(
            "flat-image detector self-test FAILED on a deliberately "
            f"two-colour 5x5 PNG -- decoder is not trustworthy: {striped_info}"
        )
    if (width, height, bit_depth, color_type, bpp) != (1, 1, 16, 2, 6):
        raise AssertionError(
            "PNG decoder self-test FAILED to surface RGB16 sample metadata: "
            f"{(width, height, bit_depth, color_type, bpp)}"
        )
    if raw != struct.pack(">HHH", 0x1234, 0xABCD, 0xFFFF):
        raise AssertionError("PNG decoder self-test FAILED to preserve RGB16 samples")
    decoded_rgb16 = _rgb_channels_8bit_scale(raw, 0, bit_depth, color_type, bpp)
    expected_rgb16 = tuple(value / 257 for value in (0x1234, 0xABCD, 0xFFFF))
    if any(abs(actual - expected) > 1e-9 for actual, expected in zip(decoded_rgb16, expected_rgb16)):
        raise AssertionError(
            "RGB16 visual sample self-test FAILED: "
            f"expected {expected_rgb16}, got {decoded_rgb16}"
        )
    print(
        "  ok  flat-image detector self-test "
        f"(flat -> {flat_info['distinct-colours-sampled']} colour, "
        f"striped -> {striped_info['distinct-colours-sampled']}+ colours)"
    )


# --------------------------------------------------------------------------
# Blender process plumbing
# --------------------------------------------------------------------------


def resolve_blender(explicit: str | None) -> str | None:
    """Find a usable Blender executable, or None if nothing resolves.

    `explicit` may be a filesystem path or a bare command name on PATH --
    whatever was passed to `--blender`. With nothing passed, falls back to
    `blender` on PATH, then the one location this project's development
    machine is known to have it installed (Task 1 toolchain probe).
    """
    candidates = (explicit,) if explicit else ("blender", _KNOWN_BLENDER_WINDOWS)
    for candidate in candidates:
        if not candidate:
            continue
        if Path(candidate).exists():
            return candidate
        found = shutil.which(candidate)
        if found:
            return found
    return None


def detect_blender_version(blender: str) -> tuple[int, int] | None:
    """Return Blender's major/minor version without loading a scene."""
    proc = subprocess.run(
        [blender, "--version"],
        capture_output=True,
        text=True,
        check=False,
    )
    match = re.search(r"\bBlender\s+(\d+)\.(\d+)", proc.stdout)
    if proc.returncode != 0 or match is None:
        return None
    return int(match.group(1)), int(match.group(2))


def run_command(blender: str, script: str, inputs: dict) -> tuple[dict, float]:
    """Run one agent command headless; return (payload, elapsed_seconds).

    Recovers the payload by slicing between `_result.RESULT_BEGIN` /
    `RESULT_END`, exactly as the real transport will. A missing sentinel
    pair -- a hard crash before the script could even frame an error -- is
    reported with both stdout and stderr tails: the sentinel protocol exists
    precisely to cut through Blender's log noise, so losing that signal here
    would be the one failure mode this harness cannot afford to hide.
    """
    script_path = SCRIPTS / script
    started = time.monotonic()
    proc = subprocess.run(
        [blender, "-b", "-P", str(script_path), "--", json.dumps(inputs)],
        capture_output=True,
        text=True,
        check=False,
    )
    elapsed = time.monotonic() - started

    out = proc.stdout
    if _result.RESULT_BEGIN not in out or _result.RESULT_END not in out:
        raise ChainError(
            f"{script}: no framed result found in Blender's output "
            f"(exit code {proc.returncode}, {elapsed:.1f}s).\n"
            f"--- stdout tail (last 2000 chars) ---\n{out[-2000:]}\n"
            f"--- stderr tail (last 2000 chars) ---\n{proc.stderr[-2000:]}"
        )
    body = out.split(_result.RESULT_BEGIN, 1)[1].split(_result.RESULT_END, 1)[0]
    try:
        payload = json.loads(body)
    except json.JSONDecodeError as exc:
        raise ChainError(
            f"{script}: framed result was not valid JSON: {exc}\n--- body ---\n{body}"
        ) from exc
    return payload, elapsed


def expect_ok(label: str, result: tuple[dict, float]) -> dict:
    """Assert a command's payload reports success; print a pass line."""
    payload, elapsed = result
    if not payload.get("ok"):
        raise ChainError(
            f"{label} reported failure ({elapsed:.1f}s):\n{json.dumps(payload, indent=2, sort_keys=True)}"
        )
    print(f"  ok  {label} ({elapsed:.1f}s)")
    return payload


# --------------------------------------------------------------------------
# The chain
# --------------------------------------------------------------------------


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--blender",
        default=None,
        help="path to the blender executable, or a bare command on PATH; "
        "omit to auto-discover (PATH, then the known dev-machine install)",
    )
    parser.add_argument("--aware-bin", default="aware", help="path to the aware CLI binary")
    parser.add_argument(
        "--keep", action="store_true", help="keep the temp work directory instead of deleting it"
    )
    opts = parser.parse_args()

    blender = resolve_blender(opts.blender)
    if blender is None:
        tried = opts.blender or f"blender (PATH), {_KNOWN_BLENDER_WINDOWS}"
        print(f"SKIP: no usable Blender found (tried: {tried})", file=sys.stderr)
        print("      pass --blender <path-to-blender-executable> to run this harness", file=sys.stderr)
        return 77  # conventional "could not run" exit code -- not a failure

    start = time.monotonic()
    work = Path(tempfile.mkdtemp(prefix="aware-blender-smoke-"))
    blender_version = detect_blender_version(blender)
    print(f"blender:  {blender}")
    print(
        "version:  "
        + (
            f"{blender_version[0]}.{blender_version[1]}"
            if blender_version is not None
            else "unknown"
        )
    )
    print(f"aware:    {opts.aware_bin}")
    print(f"work dir: {work}\n")

    try:
        self_test_flat_detector(work)

        # 1. fixture IFC through the real CLI
        ifc = work / "fixture.ifc"
        fixture_started = time.monotonic()
        fixture_receipt = make_fixture.build_fixture(opts.aware_bin, ifc)
        fixture_elapsed = time.monotonic() - fixture_started
        emitted = fixture_receipt.get("emitted", [])
        if len(emitted) != EXPECTED_ELEMENT_COUNT:
            raise ChainError(
                f"ifc.write: expected {EXPECTED_ELEMENT_COUNT} emitted elements, "
                f"got {len(emitted)}\n{json.dumps(fixture_receipt, indent=2)[:2000]}"
            )
        print(f"  ok  ifc.write ({len(emitted)} elements, {fixture_elapsed:.1f}s)")

        # 2. scene.import
        imported = expect_ok(
            "scene.import",
            run_command(
                blender,
                "scene_import.py",
                {"ifc-path": str(ifc), "blend-path": str(work / "fixture.blend")},
            ),
        )
        assert imported["imported"] == EXPECTED_ELEMENT_COUNT, (
            f"scene.import: expected imported={EXPECTED_ELEMENT_COUNT}, "
            f"got {imported.get('imported')}"
        )
        assert imported["by-class"] == EXPECTED_CLASSES, (
            f"scene.import: by-class mismatch: expected {EXPECTED_CLASSES}, "
            f"got {imported['by-class']}"
        )
        assert imported["by-material"] == EXPECTED_MATERIALS, (
            f"scene.import: by-material mismatch: expected {EXPECTED_MATERIALS}, "
            f"got {imported['by-material']}"
        )
        assert imported["by-storey"] == EXPECTED_STOREYS, (
            f"scene.import: by-storey mismatch: expected {EXPECTED_STOREYS}, "
            f"got {imported['by-storey']}"
        )
        assert imported["skipped-count"] == 0, (
            f"scene.import: expected skipped-count=0, got "
            f"{imported['skipped-count']} ({imported.get('skipped')})"
        )
        blend = Path(imported["blend-path"])  # chain via the receipt, not an assumed path
        print(f"      by-class={imported['by-class']}  by-material={imported['by-material']}")

        # 3. scene.info -- the design doc's look-mapping verification instrument
        info = expect_ok(
            "scene.info", run_command(blender, "scene_info.py", {"blend-path": str(blend)})
        )
        assert info["source"] == "blend-path", (
            f"scene.info: expected source=blend-path, got {info.get('source')!r}"
        )
        assert info["count"] == EXPECTED_ELEMENT_COUNT, (
            f"scene.info: expected count={EXPECTED_ELEMENT_COUNT}, got {info.get('count')}"
        )
        assert info["by-class"] == EXPECTED_CLASSES, (
            f"scene.info: by-class mismatch: expected {EXPECTED_CLASSES}, got {info['by-class']}"
        )
        assert info["by-material"] == EXPECTED_MATERIALS, (
            f"scene.info: by-material mismatch: expected {EXPECTED_MATERIALS}, "
            f"got {info['by-material']}"
        )
        assert info["by-storey"] == EXPECTED_STOREYS, (
            f"scene.info: by-storey mismatch: expected {EXPECTED_STOREYS}, got {info['by-storey']}"
        )
        assert "skipped-count" not in info and "skipped" not in info, (
            "scene.info: the blend-path branch must omit skipped/skipped-count "
            f"(a staged .blend carries no import record); got keys {sorted(info.keys())}"
        )
        print(f"      inventory matches the fixture exactly: {info['by-class']} / {info['by-material']}")

        # 4. scene.apply-look
        look = expect_ok(
            "scene.apply-look",
            run_command(
                blender,
                "scene_apply_look.py",
                {"blend-path": str(blend), "preset": "realistic"},
            ),
        )
        assert look["assigned"] == {"steel": 9}, (
            f"scene.apply-look: expected assigned={{'steel': 9}}, got {look['assigned']}"
        )
        blend = Path(look["blend-path"])
        print(f"      assigned={look['assigned']}")

        # 5. render.still -- PNG must exist, be the right size, and not be flat
        png_request = work / "still.png"
        still = expect_ok(
            "render.still",
            run_command(
                blender,
                "render_still.py",
                {
                    "blend-path": str(blend),
                    "output-path": str(png_request),
                    "quality": "draft",
                    "width-pixels": STILL_WIDTH,
                    "height-pixels": STILL_HEIGHT,
                },
            ),
        )
        rendered_png = Path(still["path"])
        assert rendered_png.exists(), f"render.still: reported path does not exist: {rendered_png}"
        png_info = inspect_png(rendered_png)
        assert png_info["width"] == STILL_WIDTH and png_info["height"] == STILL_HEIGHT, (
            f"render.still: expected {STILL_WIDTH}x{STILL_HEIGHT}, PNG is "
            f"{png_info['width']}x{png_info['height']}"
        )
        assert not png_info["is-flat"], (
            f"render.still: {rendered_png} is a single flat colour "
            f"({png_info['distinct-colours-sampled']} distinct colour(s) sampled) -- "
            "the render shows nothing"
        )
        banding_score = None
        if blender_version in WEB_BANDING_REGRESSION_VERSIONS:
            banding_score = fixture_web_banding_score(rendered_png)
            assert banding_score <= MAX_WEB_BANDING_SCORE, (
                "render.still: EEVEE + HDRI bands the fixture's I-section web "
                f"(high-frequency score {banding_score:.3f}, expected <= "
                f"{MAX_WEB_BANDING_SCORE:.3f})"
            )
        else:
            shown_version = (
                f"{blender_version[0]}.{blender_version[1]}"
                if blender_version is not None
                else "unknown"
            )
            print(
                "      web banding regression check skipped: calibrated for "
                f"Blender 5.2, running {shown_version}"
            )
        # The staging receipts. A degraded environment still renders a perfectly
        # good-looking picture, so these are the only place the chain can tell
        # "lit by the HDRI it asked for" from "quietly fell back to the
        # gradient" -- same reasoning as skipped/excluded on the import receipt.
        assert still["environment"]["source"] == "blender-studiolight", (
            "render.still: the default environment should resolve to one of "
            f"Blender's bundled HDRIs, got {still['environment']}"
        )
        assert still["environment"]["fallback-reason"] is None, (
            "render.still: the default environment degraded on a machine that "
            f"has the bundled HDRIs: {still['environment']}"
        )
        assert still["ground"]["present"], (
            f"render.still: expected a ground plane by default, got {still['ground']}"
        )
        print(
            f"      environment={still['environment']['resolved']} "
            f"({still['environment']['source']}), "
            f"ground radius={still['ground']['radius']}"
        )
        render_summary = (
            f"      {rendered_png.name}: {png_info['width']}x{png_info['height']}, "
            f"{png_info['distinct-colours-sampled']}+ distinct colours sampled, "
            f"{rendered_png.stat().st_size:,} bytes"
        )
        if banding_score is not None:
            render_summary += f", web banding score={banding_score:.3f}"
        print(render_summary)

        # 6. render.turntable -- MP4 must exist, be a real container, non-trivial size
        mp4_request = work / "turn.mp4"
        turntable = expect_ok(
            "render.turntable",
            run_command(
                blender,
                "render_turntable.py",
                {
                    "blend-path": str(blend),
                    "output-path": str(mp4_request),
                    "duration-seconds": TURNTABLE_DURATION_SECONDS,
                    "fps": TURNTABLE_FPS,
                    "width-pixels": TURNTABLE_WIDTH,
                    "height-pixels": TURNTABLE_HEIGHT,
                },
            ),
        )
        rendered_mp4 = Path(turntable["path"])
        assert rendered_mp4.exists(), (
            f"render.turntable: reported path does not exist: {rendered_mp4}"
        )
        size = rendered_mp4.stat().st_size
        assert size > MIN_MP4_BYTES, (
            f"render.turntable: {rendered_mp4} is only {size} bytes, "
            f"expected > {MIN_MP4_BYTES} for a non-trivial MP4"
        )
        header = rendered_mp4.read_bytes()[:12]
        assert header[4:8] == b"ftyp", (
            f"render.turntable: {rendered_mp4} does not look like an MP4 "
            f"container (first 12 bytes: {header!r})"
        )
        print(
            f"      {rendered_mp4.name}: {size:,} bytes, "
            f"{turntable.get('frames')} frames @ {turntable.get('fps')}fps"
        )

        elapsed = time.monotonic() - start
        print(f"\nSMOKE PASS ({elapsed:.1f}s)")
        return 0

    except (RuntimeError, AssertionError) as exc:
        # RuntimeError, not ChainError: ChainError IS a RuntimeError, so this
        # catches it too, but catching only ChainError here would miss the
        # plain RuntimeErrors make_fixture.build_fixture raises directly
        # (the aware CLI failing, or succeeding but not writing its output)
        # -- those would otherwise fall through to the generic handler below
        # and print as an "unexpected exception" traceback dump instead of
        # this handler's clean one-line summary.
        elapsed = time.monotonic() - start
        print(f"\nSMOKE FAIL ({elapsed:.1f}s)", file=sys.stderr)
        print(str(exc), file=sys.stderr)
        return 1
    except Exception:  # noqa: BLE001 - last resort: fail loudly, with a traceback
        elapsed = time.monotonic() - start
        print(f"\nSMOKE FAIL ({elapsed:.1f}s) -- unexpected exception", file=sys.stderr)
        traceback.print_exc()
        return 1
    finally:
        if opts.keep:
            print(f"\n--keep: work dir retained at {work}")
        else:
            shutil.rmtree(work, ignore_errors=True)


if __name__ == "__main__":
    sys.exit(main())
