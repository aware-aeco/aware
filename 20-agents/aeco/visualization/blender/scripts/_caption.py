"""The optional `caption` of `render.still` / `render.turntable`: one line burned into every pixel frame.

The caption is drawn on a single line, shrunk (from ~2.8% of the height down to 8 px) until it fits the picture's
width as Blender itself measures it; one that fits at no size is refused rather than clipped or wrapped.

It uses Blender's own render stamp ("burn into image") with every stamp field off except the note, so the
picture carries exactly the caller's text and nothing else Blender would otherwise stamp (date, frame, file
name...). The text travels with the image itself: a PNG or an MP4 that is renamed, copied into a document or
cropped from a page still says it. Blender draws it at the top left.

Without a caption nothing here touches the render settings, so an uncaptioned render is exactly what it was.
"""

import os
import struct

import blf
import bpy

import _result

CAPTION_MAX_CHARS = 200
# Blender keeps the note in a fixed 768-byte buffer (DNA `stamp_udata`), so a longer UTF-8 caption would be
# cut short in silence. Bounded a little under it, and the stored text is read back to be sure.
CAPTION_MAX_BYTES = 760
# Blender accepts a stamp font size of 8..64 px.
STAMP_FONT_MIN_PX = 8
STAMP_FONT_MAX_PX = 64
# Blender's stamp wraps its text to the picture's width less a 2 px margin each side (BUFF_MARGIN_X in
# `BKE_image_stamp_buf`), and pads its background box 1 px above and below each block (BUFF_MARGIN_Y).
STAMP_MARGIN_X = 2
STAMP_MARGIN_Y = 1
# A stamp line's cell is the font's maximum line height (`BLF_height_max`), a little over one em for the stamp font;
# budgeted at 1.5 em so one line is always judged with room to spare.
STAMP_LINE_CELL_EM = 1.5
# The caption may take at most this share of the picture's height, so it never buries the model.
STAMP_MAX_HEIGHT_SHARE = 0.35
# The font the stamp is drawn in: Blender's monospaced UI font (with its CJK and symbol fallbacks).
STAMP_FONT_FILE = "DejaVuSansMono.woff2"


def validate_caption(value):
    """The caption as a string, or None when not asked for. A malformed one is the caller's mistake."""
    if value is None:
        return None
    if not isinstance(value, str) or not value.strip() or len(value) > CAPTION_MAX_CHARS:
        raise _result.AwareBlenderError(
            _result.ERR_INVALID_INPUTS,
            f"`caption` must be a non-blank string of at most {CAPTION_MAX_CHARS} characters",
        )
    if len(value.encode("utf-8")) > CAPTION_MAX_BYTES:
        raise _result.AwareBlenderError(
            _result.ERR_INVALID_INPUTS,
            f"`caption` must be at most {CAPTION_MAX_BYTES} bytes as UTF-8 (Blender's stamp keeps no more)",
        )
    if any(ord(ch) <= 0x1F or ord(ch) == 0x7F for ch in value):
        raise _result.AwareBlenderError(
            _result.ERR_INVALID_INPUTS, "`caption` must be one line with no control characters"
        )
    return value


def _stamp_font_id() -> int:
    """The id of the font the stamp is drawn in. Loading Blender's bundled monospaced font hands back the id it
    already has (the stamp's own, fallbacks included), so what is measured here is what the stamp draws."""
    path = os.path.join(bpy.utils.system_resource("DATAFILES", path="fonts"), STAMP_FONT_FILE)
    font_id = blf.load(path) if os.path.exists(path) else -1
    if font_id < 0:
        raise _result.AwareBlenderError(
            _result.ERR_RENDER_FAILED,
            f"cannot measure the caption: Blender's stamp font {STAMP_FONT_FILE} was not found",
        )
    return font_id


def fit_font(caption: str, width_pixels: int, height_pixels: int):
    """The largest stamp font (8..64 px, preferring ~2.8% of the height) at which the caption fits on ONE line, or
    None when it cannot at Blender's smallest size.

    One line, on purpose (the owner's call after three review rounds on multi-line fitting): the stamp wraps a line
    once the next glyph's advance reaches its wrap width, so a caption measured by Blender's own engine, in the
    stamp's own font and whitespace included, as narrower than that width is drawn unwrapped -- and one line's cell
    (at most ~1.25 em) sits inside the height budget at every size allowed here. Nothing is estimated."""
    font_id = _stamp_font_id()
    wrap_width = width_pixels - 2 * STAMP_MARGIN_X
    preferred = min(STAMP_FONT_MAX_PX, max(12, round(height_pixels * 0.028)))
    for font_px in range(preferred, STAMP_FONT_MIN_PX - 1, -1):
        blf.size(font_id, font_px)
        line_width = blf.dimensions(font_id, caption)[0]
        if line_width < wrap_width and font_px * STAMP_LINE_CELL_EM + 2 * STAMP_MARGIN_Y <= height_pixels * STAMP_MAX_HEIGHT_SHARE:
            return font_px
    return None


def _stamp_settings(caption, font_px):
    settings = {
        "use_stamp_note": True,
        "stamp_note_text": caption,
        "use_stamp": True,  # burn into the pixels, not only the file's metadata
        "stamp_font_size": font_px,
        "stamp_foreground": (1.0, 1.0, 1.0, 1.0),
        "stamp_background": (0.0, 0.0, 0.0, 0.62),
    }
    # The scene's own note, never a sequencer strip's. Defensive: Blender reads strip metadata (an empty note
    # included) in place of `stamp_note_text` only when the picture IS the sequencer output, which a model render
    # never is; the test fixture keeps that trap so a change in Blender's behaviour would show.
    if hasattr(bpy.types.RenderSettings, "bl_rna") and "metadata_input" in bpy.types.RenderSettings.bl_rna.properties:
        settings["metadata_input"] = "SCENE"
    return settings


def _write(render, settings) -> None:
    # Every stamp field off, then only the note on: Blender's defaults stamp the date, time, frame,
    # camera, file name and more, none of which the caller asked to put in their picture.
    for prop in render.bl_rna.properties:
        if prop.identifier.startswith("use_stamp_") and prop.type == "BOOLEAN" and not prop.is_readonly:
            setattr(render, prop.identifier, False)
    for name, value in settings.items():
        setattr(render, name, value)


def _f32(value: float) -> float:
    return struct.unpack("f", struct.pack("f", value))[0]


def _cropped_span(border_min: float, border_max: float, full_pixels: int) -> int:
    """The size of a border-cropped render along one axis, computed as Blender does: each border endpoint is
    scaled in single precision and truncated to an integer (`disprect.xmin = border.xmin * winx`), then the two
    are subtracted. Rounding the span instead can come out one pixel wider than the picture Blender delivers."""
    low = int(_f32(_f32(border_min) * _f32(full_pixels)))
    high = int(_f32(_f32(border_max) * _f32(full_pixels)))
    return max(1, high - low)


def apply_caption(scene, caption) -> dict:
    """Burn `caption` into the render, fitted to the picture its settings deliver; returns the receipt fields (empty
    when there is no caption). Call it after the resolution is set."""
    if caption is None:
        return {}
    # Fitted to the picture Blender actually delivers, which is what it stamps: a staged .blend may crop its
    # render to a border, and then the result is that border's size, not the requested resolution.
    render = scene.render
    # The resolution Blender STORED, not the one requested: it clamps each axis (to 65,536 px), and the picture it
    # delivers is the stored resolution scaled by its percentage (`winx = xsch * size / 100`).
    width_pixels = max(1, render.resolution_x * render.resolution_percentage // 100)
    height_pixels = max(1, render.resolution_y * render.resolution_percentage // 100)
    if render.use_border and render.use_crop_to_border:
        width_pixels = _cropped_span(render.border_min_x, render.border_max_x, width_pixels)
        height_pixels = _cropped_span(render.border_min_y, render.border_max_y, height_pixels)
    # A caption of only invisible characters (a zero-width space, say) passes every text check yet draws nothing:
    # it is refused rather than reported as drawn.
    font_id = _stamp_font_id()
    blf.size(font_id, STAMP_FONT_MAX_PX)
    if min(blf.dimensions(font_id, caption)) <= 0:
        raise _result.AwareBlenderError(
            _result.ERR_INVALID_INPUTS, "`caption` has nothing visible to draw"
        )
    font_px = fit_font(caption, width_pixels, height_pixels)
    if font_px is None:
        raise _result.AwareBlenderError(
            _result.ERR_INVALID_INPUTS,
            f"`caption` cannot fit inside a {width_pixels}x{height_pixels} picture even at the smallest stamp "
            "size on one line; shorten it or render larger",
        )
    settings = _stamp_settings(caption, font_px)
    _write(render, settings)
    if render.stamp_note_text != caption:
        raise _result.AwareBlenderError(
            _result.ERR_RENDER_FAILED,
            "Blender did not keep the caption exactly as given, so it cannot be drawn faithfully",
        )

    # A staged .blend may animate or drive its own render settings, and frame evaluation would then put
    # them back (`use_stamp` off) on every frame of a turntable. Re-assert them after each frame change --
    # after animation and drivers have been evaluated, before that frame is rendered.
    def _hold(evaluated_scene, *_args):
        if evaluated_scene == scene:
            _write(evaluated_scene.render, settings)

    bpy.app.handlers.frame_change_post.append(_hold)
    return {"caption": caption, "caption-drawn": True, "caption-font-pixels": font_px}
