"""The optional `caption` of `render.still` / `render.turntable`: one line burned into every pixel frame.

It uses Blender's own render stamp ("burn into image") with every stamp field off except the note, so the
picture carries exactly the caller's text and nothing else Blender would otherwise stamp (date, frame, file
name...). The text travels with the image itself: a PNG or an MP4 that is renamed, copied into a document or
cropped from a page still says it. Blender draws it at the top left and wraps it at spaces.

Without a caption nothing here touches the render settings, so an uncaptioned render is exactly what it was.
"""

import bpy

import _result

CAPTION_MAX_CHARS = 200
# Blender keeps the note in a fixed 768-byte buffer (DNA `stamp_udata`), so a longer UTF-8 caption would be
# cut short in silence. Bounded a little under it, and the stored text is read back to be sure.
CAPTION_MAX_BYTES = 760
# The stamp wraps at spaces only; a single word wider than the picture would run off its edge.
CAPTION_MAX_WORD_CHARS = 40
# Blender accepts a stamp font size of 8..64 px.
STAMP_FONT_MAX_PX = 64


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
    if any(len(word) > CAPTION_MAX_WORD_CHARS for word in value.split(" ")):
        raise _result.AwareBlenderError(
            _result.ERR_INVALID_INPUTS,
            f"`caption` words must be at most {CAPTION_MAX_WORD_CHARS} characters, so the line can wrap inside the picture",
        )
    return value


def _stamp_settings(caption, height_pixels):
    # About 2.8% of the picture's height, within Blender's 8..64 px: legible on a 720p video frame; above
    # ~2,290 px tall the caption stops growing with the picture.
    return {
        "use_stamp_note": True,
        "stamp_note_text": caption,
        "use_stamp": True,  # burn into the pixels, not only the file's metadata
        "stamp_font_size": min(STAMP_FONT_MAX_PX, max(12, round(height_pixels * 0.028))),
        "stamp_foreground": (1.0, 1.0, 1.0, 1.0),
        "stamp_background": (0.0, 0.0, 0.0, 0.62),
    }


def _write(render, settings) -> None:
    # Every stamp field off, then only the note on: Blender's defaults stamp the date, time, frame,
    # camera, file name and more, none of which the caller asked to put in their picture.
    for prop in render.bl_rna.properties:
        if prop.identifier.startswith("use_stamp_") and prop.type == "BOOLEAN" and not prop.is_readonly:
            setattr(render, prop.identifier, False)
    for name, value in settings.items():
        setattr(render, name, value)


def apply_caption(scene, caption, height_pixels: int) -> dict:
    """Burn `caption` into the render; returns the receipt fields (empty when there is no caption)."""
    if caption is None:
        return {}
    render = scene.render
    settings = _stamp_settings(caption, height_pixels)
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
    return {"caption": caption, "caption-drawn": True}
