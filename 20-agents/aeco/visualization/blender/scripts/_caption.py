"""The optional `caption` of `render.still` / `render.turntable`: one line burned into every pixel frame.

It uses Blender's own render stamp ("burn into image") with every stamp field off except the note, so the
picture carries exactly the caller's text and nothing else Blender would otherwise stamp (date, frame, file
name...). The text travels with the image itself: a PNG or an MP4 that is renamed, copied into a document or
cropped from a page still says it. Blender draws it at the top left and wraps it at spaces.

Without a caption nothing here touches the render settings, so an uncaptioned render is exactly what it was.
"""

import math

import bpy

import _result

CAPTION_MAX_CHARS = 200
# Blender keeps the note in a fixed 768-byte buffer (DNA `stamp_udata`), so a longer UTF-8 caption would be
# cut short in silence. Bounded a little under it, and the stored text is read back to be sure.
CAPTION_MAX_BYTES = 760
# Blender accepts a stamp font size of 8..64 px.
STAMP_FONT_MIN_PX = 8
STAMP_FONT_MAX_PX = 64
# The stamp is drawn in Blender's monospaced UI font: every glyph advances about 0.6 em (a 40-character
# word at 30 px measures 720 px). Laid out a little wide, so a caption judged to fit does fit.
STAMP_ADVANCE_EM = 0.62
STAMP_LINE_EM = 1.5
# The caption may take at most this share of the picture's height, so it never buries the model.
STAMP_MAX_HEIGHT_SHARE = 0.35


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


def _lines_at(caption: str, font_px: int, width_pixels: int):
    """How many lines the caption wraps to at `font_px` (wrapping at spaces), or None if a word cannot fit."""
    per_line = math.floor(width_pixels / (STAMP_ADVANCE_EM * font_px))
    if per_line < 1:
        return None
    lines, used = 1, 0
    for word in caption.split(" "):
        if len(word) > per_line:
            return None
        need = len(word) if used == 0 else used + 1 + len(word)
        if need <= per_line:
            used = need
        else:
            lines, used = lines + 1, len(word)
    return lines


def fit_font(caption: str, width_pixels: int, height_pixels: int):
    """The largest stamp font (8..64 px, at most ~2.8% of the height) at which the whole caption fits inside the
    picture, or None when it cannot fit even at Blender's smallest size."""
    preferred = min(STAMP_FONT_MAX_PX, max(12, round(height_pixels * 0.028)))
    for font_px in range(preferred, STAMP_FONT_MIN_PX - 1, -1):
        lines = _lines_at(caption, font_px, width_pixels)
        if lines is not None and lines * font_px * STAMP_LINE_EM <= height_pixels * STAMP_MAX_HEIGHT_SHARE:
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


def apply_caption(scene, caption, width_pixels: int, height_pixels: int) -> dict:
    """Burn `caption` into the render; returns the receipt fields (empty when there is no caption)."""
    if caption is None:
        return {}
    font_px = fit_font(caption, width_pixels, height_pixels)
    if font_px is None:
        raise _result.AwareBlenderError(
            _result.ERR_INVALID_INPUTS,
            f"`caption` cannot fit inside a {width_pixels}x{height_pixels} picture even at the smallest stamp "
            "size; shorten it, break up its longest word, or render larger",
        )
    render = scene.render
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
