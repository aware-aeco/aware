"""`caption` must be burned into the delivered pixels -- of the still AND of every video frame.

A receipt that says `caption-drawn: true` is a claim; this file checks it against the bytes:

1. **Still.** A captioned and an uncaptioned render of the same model at the same settings differ ONLY in
   a band at the top left of the picture (where Blender draws its stamp), and that difference is about as
   wide as the caption's text. The PNG's own text metadata carries the caption's exact words (Blender
   writes the stamp note there too).
2. **Turntable.** EVERY frame of the MP4 is decoded inside Blender itself (`_decode_frame.py`, through the
   sequencer, so no ffmpeg on PATH is needed). H.264 is lossy, so each frame is compared statistically: the
   change inside the still's caption box must stand far above the coding noise elsewhere. The fixture
   keyframes its own stamp off, so a caption that does not hold through frame evaluation fails here. The
   first and middle frames must differ -- the orbit moved -- so copies of one frame cannot pass.
   `--keep <dir>` saves every picture so a person can read the words.
3. **No caption, no change.** Without `caption` the receipt carries no caption fields, and a malformed
   caption is refused as a caller mistake before anything renders.

Run: python test_caption.py [--blender <path>]
Prints PASS/FAIL and exits 0 only if every check passes; prints SKIP and exits 0 without Blender.
"""

import argparse
import json
import shutil
import subprocess
import sys
import tempfile
from pathlib import Path

HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE))

from run_smoke import (  # noqa: E402
    ChainError,
    _iter_png_chunks,
    decode_png,
    expect_ok,
    resolve_blender,
    run_command,
)

WIDTH, HEIGHT = 320, 180
FRAMES = 8
CAPTION = "Quick model · 3 AI decisions not reviewed"

FAILURES: list[str] = []


def check(label: str, condition: bool, detail=None) -> None:
    if condition:
        print(f"  ok   {label}")
    else:
        print(f"  FAIL {label}")
        if detail is not None:
            print(f"       {detail}")
        FAILURES.append(label)


def pixels(path: Path):
    width, height, bit_depth, _ct, bpp, raw = decode_png(path.read_bytes())
    return width, height, bpp, raw


def diff_box(a: Path, b: Path):
    """Bounding box (x0, y0, x1, y1) of the pixels that differ noticeably between two same-size PNGs."""
    wa, ha, bpa, ra = pixels(a)
    wb, hb, bpb, rb = pixels(b)
    if (wa, ha, bpa) != (wb, hb, bpb):
        raise ChainError(f"{a.name} and {b.name} differ in size or format")
    x0 = y0 = 10**9
    x1 = y1 = -1
    step = bpa
    for y in range(ha):
        row = y * wa * step
        for x in range(wa):
            i = row + x * step
            # 8-bit RGB(A): the first three samples; a 16-bit PNG compares its high bytes.
            if bpa >= 6:
                delta = max(abs(ra[i + c * 2] - rb[i + c * 2]) for c in range(3))
            else:
                delta = max(abs(ra[i + c] - rb[i + c]) for c in range(min(3, step)))
            if delta > 24:
                x0, y0, x1, y1 = min(x0, x), min(y0, y), max(x1, x), max(y1, y)
    return None if x1 < 0 else (x0, y0, x1, y1)


def mean_delta(a: Path, b: Path, box) -> tuple[float, float]:
    """Mean per-pixel colour change inside `box` and outside it -- tolerant of lossy video coding."""
    wa, ha, bpa, ra = pixels(a)
    _wb, _hb, _bpb, rb = pixels(b)
    inside = outside = 0.0
    n_in = n_out = 0
    for y in range(ha):
        row = y * wa * bpa
        for x in range(wa):
            i = row + x * bpa
            delta = sum(abs(ra[i + c] - rb[i + c]) for c in range(3)) / 3
            if box[0] <= x <= box[2] and box[1] <= y <= box[3]:
                inside += delta
                n_in += 1
            else:
                outside += delta
                n_out += 1
    return inside / max(n_in, 1), outside / max(n_out, 1)


def png_text(path: Path) -> dict:
    text = {}
    for ctype, cdata in _iter_png_chunks(path.read_bytes()):
        if ctype == b"tEXt" and b"\x00" in cdata:
            key, value = cdata.split(b"\x00", 1)
            # Blender writes the note as UTF-8 even in a tEXt chunk (nominally Latin-1).
            try:
                text[key.decode("latin-1")] = value.decode("utf-8")
            except UnicodeDecodeError:
                text[key.decode("latin-1")] = value.decode("latin-1")
        elif ctype == b"iTXt" and b"\x00" in cdata:
            key, rest = cdata.split(b"\x00", 1)
            # compression flag, method, language\0, translated keyword\0, text
            parts = rest[2:].split(b"\x00", 2)
            text[key.decode("latin-1")] = parts[2].decode("utf-8") if len(parts) == 3 else ""
    return text


def check_still_caption(label: str, captioned: Path, plain: Path, font_px: int, width: int, height: int,
                        caption: str = CAPTION):
    """The still is lossless, so the difference IS the stamp: a box at the top left, about the text's width,
    inside the picture (never clipped at the right edge).

    Blender draws the stamp note in the picture's top-left corner (wrapping a line too long for the width).
    """
    box = diff_box(captioned, plain)
    check(f"{label}: the caption changed the picture", box is not None)
    if box is None:
        return None
    band = int(height * 0.35) + 2  # the caption's whole budget: a third of the height, plus the box margins
    check(f"{label}: ...only in a band at the top", box[3] < band, f"diff box {box}, band ends at row {band}")
    check(f"{label}: ...starting at the left edge", box[0] <= width // 10, f"diff box {box}")
    longest = max(len(word) for word in caption.split(" "))
    min_width = min(width - box[0] - 1, int(0.4 * font_px * longest))
    check(f"{label}: ...at least as wide as its longest word", box[2] - box[0] + 1 >= min_width,
          f"diff box {box} is {box[2] - box[0] + 1}px wide; expected at least {min_width}px")
    check(f"{label}: ...and not running off the right edge", box[2] <= width - 2, f"diff box {box}, width {width}")
    return box


def check_frame_caption(label: str, captioned: Path, plain: Path, box) -> None:
    """A video frame is H.264-coded, so the comparison is statistical: the change inside the stamp's box
    (taken from the lossless still at the same size) must stand far above the coding noise elsewhere."""
    inside, outside = mean_delta(captioned, plain, box)
    check(f"{label}: the caption's box changed far more than the rest of the frame",
          inside >= 20 and inside >= 5 * max(outside, 1.0),
          f"mean change inside {inside:.1f}, outside {outside:.1f}")


def decode_frames(blender: str, mp4: Path, frames: list[int], out_dir: Path) -> dict[int, Path]:
    out_dir.mkdir(parents=True, exist_ok=True)
    proc = subprocess.run(
        [blender, "-b", "--factory-startup", "-P", str(HERE / "_decode_frame.py"), "--",
         json.dumps({"mp4": str(mp4), "frames": frames, "out-dir": str(out_dir)})],
        capture_output=True, text=True, check=False,
    )
    paths = {frame: out_dir / f"frame-{frame}.png" for frame in frames}
    if "AWARE_DECODED_FRAME" not in proc.stdout or not all(p.exists() for p in paths.values()):
        raise ChainError(f"could not decode the frames of {mp4.name} (MP4 verification outstanding):\n"
                         f"{proc.stdout[-1500:]}\n{proc.stderr[-1500:]}")
    return paths


def make_blend(blender: str, out: Path) -> None:
    """A small staged .blend (three boxes) with the traps `_make_caption_fixture.py` describes: the caption is
    about the pixels, not the IFC import."""
    proc = subprocess.run(
        [blender, "-b", "--factory-startup", "-P", str(HERE / "_make_caption_fixture.py"), "--",
         json.dumps({"out": str(out), "strip-png": str(out.with_name("strip.png"))})],
        capture_output=True, text=True, check=False,
    )
    if "AWARE_FIXTURE_READY" not in proc.stdout or not out.exists():
        raise ChainError(f"could not build the fixture .blend:\n{proc.stdout[-1500:]}\n{proc.stderr[-1500:]}")


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--blender", help="path to the Blender executable")
    parser.add_argument("--aware-bin", help="unused; accepted so every harness here takes the same arguments")
    parser.add_argument("--keep", help="copy the stills and decoded frames here, to look at the words")
    opts = parser.parse_args()

    blender = resolve_blender(opts.blender)
    if blender is None:
        print("SKIP: no Blender available (pass --blender <path> to run this harness)")
        return 0

    work = Path(tempfile.mkdtemp(prefix="aware-caption-"))
    try:
        blend = work / "fixture.blend"
        make_blend(blender, blend)
        common = {"blend-path": str(blend), "quality": "draft", "width-pixels": WIDTH, "height-pixels": HEIGHT}

        # ---- 3. malformed captions are refused before anything renders ------
        for bad in ["", "two\nlines", "x" * 201, 7, "\U0001F600" * 191]:
            payload, _ = run_command(blender, "render_still.py", {
                **common, "output-path": str(work / "bad.png"), "caption": bad})
            check(f"caption {bad!a:.24} is refused", payload.get("ok") is False, payload)
        # A caption that cannot fit the picture even at Blender's smallest stamp is refused, never clipped.
        payload, _ = run_command(blender, "render_still.py", {
            **common, "width-pixels": 64, "height-pixels": 64, "output-path": str(work / "bad.png"), "caption": CAPTION})
        check("a caption too big for a 64x64 picture is refused", payload.get("ok") is False, payload)
        check("...and nothing was rendered for any refusal", not (work / "bad.png").exists())

        # ---- a narrow, tall picture shrinks the one-line caption until it fits ---------------------------
        narrow = {**common, "width-pixels": 160, "height-pixels": 400}
        shrunk = "Quick model " + "W" * 15
        expect_ok("render.still (narrow, no caption)", run_command(blender, "render_still.py", {
            **narrow, "output-path": str(work / "narrow-plain.png")}))
        narrow_cap = expect_ok("render.still (narrow, shrunk)", run_command(blender, "render_still.py", {
            **narrow, "output-path": str(work / "narrow-captioned.png"), "caption": shrunk}))
        check("the narrow picture's caption was shrunk below the preferred size",
              narrow_cap.get("caption-font-pixels", 99) < 12, narrow_cap)
        check_still_caption("narrow", work / "narrow-captioned.png", work / "narrow-plain.png",
                            narrow_cap["caption-font-pixels"], 160, 400, shrunk)
        # One line only: a caption that would need a second line at the smallest size is refused, never wrapped.
        payload, _ = run_command(blender, "render_still.py", {
            **narrow, "output-path": str(work / "narrow-two-lines.png"), "caption": CAPTION})
        check("a caption that needs two lines on a 160 px picture is refused", payload.get("ok") is False, payload)
        # Double-width glyphs are measured, not assumed: CJK at the preferred size would overflow 160 px.
        cjk = "Quick 界界界界界界界界界界界界"
        cjk_cap = expect_ok("render.still (narrow, CJK)", run_command(blender, "render_still.py", {
            **narrow, "output-path": str(work / "narrow-cjk.png"), "caption": cjk}))
        check_still_caption("narrow CJK", work / "narrow-cjk.png", work / "narrow-plain.png",
                            cjk_cap["caption-font-pixels"], 160, 400, cjk)
        # The wrap width is the picture less Blender's 2 px margins (156 px here): 31 W at the 8 px minimum
        # (155 px) fits, 32 W (160 px) cannot fit at any size and is refused, never clipped.
        just_fits = expect_ok("render.still (31 W at the edge)", run_command(blender, "render_still.py", {
            **narrow, "output-path": str(work / "narrow-31w.png"), "caption": "W" * 31}))
        check_still_caption("31 W", work / "narrow-31w.png", work / "narrow-plain.png",
                            just_fits["caption-font-pixels"], 160, 400, "W" * 31)
        payload, _ = run_command(blender, "render_still.py", {
            **narrow, "output-path": str(work / "narrow-32w.png"), "caption": "W" * 32})
        check("32 W, wider than the wrap width at every size, is refused", payload.get("ok") is False, payload)
        check("...and not rendered", not (work / "narrow-32w.png").exists())
        # Whitespace is measured as Blender lays it out: 50 leading spaces push the only glyph off the picture.
        payload, _ = run_command(blender, "render_still.py", {
            **narrow, "output-path": str(work / "narrow-spaces.png"), "caption": " " * 50 + "A"})
        check("a caption whose text would start off the picture is refused", payload.get("ok") is False, payload)
        # Several short words on a tiny picture would wrap into lines taller than the budget: refused.
        payload, _ = run_command(blender, "render_still.py", {
            **common, "width-pixels": 32, "height-pixels": 80, "output-path": str(work / "tiny.png"),
            "caption": "a a a a a a"})
        check("words that would wrap on a 32x80 picture are refused", payload.get("ok") is False, payload)
        # A short one-liner on the same tiny picture fits, inside the budget.
        expect_ok("render.still (tiny, no caption)", run_command(blender, "render_still.py", {
            **common, "width-pixels": 32, "height-pixels": 80, "output-path": str(work / "tiny-plain.png")}))
        tiny_cap = expect_ok("render.still (tiny, one word)", run_command(blender, "render_still.py", {
            **common, "width-pixels": 32, "height-pixels": 80, "output-path": str(work / "tiny-captioned.png"),
            "caption": "AI"}))
        check_still_caption("tiny", work / "tiny-captioned.png", work / "tiny-plain.png",
                            tiny_cap["caption-font-pixels"], 32, 80, "AI")

        # ---- 1. still ---------------------------------------------------------
        plain = expect_ok("render.still (no caption)", run_command(blender, "render_still.py", {
            **common, "output-path": str(work / "plain.png")}))
        captioned = expect_ok("render.still (caption)", run_command(blender, "render_still.py", {
            **common, "output-path": str(work / "captioned.png"), "caption": CAPTION}))
        check("an uncaptioned receipt has no caption fields",
              "caption" not in plain and "caption-drawn" not in plain, plain)
        check("the captioned receipt echoes the caption exactly", captioned.get("caption") == CAPTION, captioned)
        check("...and says it was drawn", captioned.get("caption-drawn") is True, captioned)
        check("the framing is the same with and without a caption", plain["framing"] == captioned["framing"])
        box = check_still_caption("still", work / "captioned.png", work / "plain.png",
                                  captioned["caption-font-pixels"], WIDTH, HEIGHT)
        text = png_text(work / "captioned.png")
        check("the PNG's own metadata carries the caption's words", CAPTION in text.values(), text)
        check("...and nothing else Blender would stamp (no date, no file name)",
              not ({"Date", "File", "Time", "RenderTime"} & set(text)), sorted(text))

        # ---- 2. turntable -----------------------------------------------------
        turn = {**common, "duration-seconds": FRAMES / 8, "fps": 8}
        turn.pop("quality")
        plain_mp4 = expect_ok("render.turntable (no caption)", run_command(blender, "render_turntable.py", {
            **turn, "output-path": str(work / "plain.mp4")}))
        cap_mp4 = expect_ok("render.turntable (caption)", run_command(blender, "render_turntable.py", {
            **turn, "output-path": str(work / "captioned.mp4"), "caption": CAPTION}))
        check("the turntable receipt echoes the caption exactly", cap_mp4.get("caption") == CAPTION, cap_mp4)
        check("...and says it was drawn", cap_mp4.get("caption-drawn") is True, cap_mp4)
        frames = list(range(1, cap_mp4["frames"] + 1))
        plain_frames = decode_frames(blender, Path(plain_mp4["path"]), frames, work / "plain-frames")
        cap_frames = decode_frames(blender, Path(cap_mp4["path"]), frames, work / "captioned-frames")
        if box is not None:
            # Every frame, including those where the fixture's own keyframes switch the stamp off.
            for frame in frames:
                check_frame_caption(f"video frame {frame}", cap_frames[frame], plain_frames[frame], box)
        middle = frames[len(frames) // 2]
        _inside, moved = mean_delta(plain_frames[1], plain_frames[middle], (-1, -1, -1, -1))
        check(f"frames 1 and {middle} are different pictures (the orbit moved)", moved > 1.0,
              f"mean change between them {moved:.2f}")
        if opts.keep:
            keep = Path(opts.keep)
            keep.mkdir(parents=True, exist_ok=True)
            for path in [work / "plain.png", work / "captioned.png"]:
                shutil.copy2(path, keep / path.name)
            for name, decoded in (("plain", plain_frames), ("captioned", cap_frames)):
                for frame, path in decoded.items():
                    shutil.copy2(path, keep / f"{name}-{frame}.png")
            print(f"  kept the pictures for inspection in {keep}")
    except ChainError as exc:
        print(f"\n  FAIL {exc}")
        FAILURES.append(str(exc))
    finally:
        shutil.rmtree(work, ignore_errors=True)

    print("\nCAPTION FAIL" if FAILURES else "\nCAPTION PASS")
    return 1 if FAILURES else 0


if __name__ == "__main__":
    sys.exit(main())
