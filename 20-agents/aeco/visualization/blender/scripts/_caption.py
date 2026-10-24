"""The optional `caption` of `render.still` / `render.turntable`: one line burned into every pixel frame.

It uses Blender's own render stamp ("burn into image") with every stamp field off except the note, so the
picture carries exactly the caller's text and nothing else Blender would otherwise stamp (date, frame, file
name...). The text travels with the image itself: a PNG or an MP4 that is renamed, copied into a document or
cropped from a page still says it.

Without a caption nothing here touches the render settings, so an uncaptioned render is exactly what it was.
"""

import bpy

import _result

CAPTION_MAX_CHARS = 200


def validate_caption(value):
    """The caption as a string, or None when not asked for. A malformed one is the caller's mistake."""
    if value is None:
        return None
    if not isinstance(value, str) or not value.strip() or len(value) > CAPTION_MAX_CHARS:
        raise _result.AwareBlenderError(
            _result.ERR_INVALID_INPUTS,
            f"`caption` must be a non-blank string of at most {CAPTION_MAX_CHARS} characters",
        )
    if any(ord(ch) <= 0x1F or ord(ch) == 0x7F for ch in value):
        raise _result.AwareBlenderError(
            _result.ERR_INVALID_INPUTS, "`caption` must be one line with no control characters"
        )
    return value


def apply_caption(scene, caption, height_pixels: int) -> dict:
    """Burn `caption` into the render; returns the receipt fields (empty when there is no caption)."""
    if caption is None:
        return {}
    render = scene.render
    # Every stamp field off, then only the note on: Blender's defaults stamp the date, time, frame,
    # camera, file name and more, none of which the caller asked to put in their picture.
    for prop in render.bl_rna.properties:
        if prop.identifier.startswith("use_stamp_") and prop.type == "BOOLEAN":
            setattr(render, prop.identifier, False)
    render.use_stamp_note = True
    render.stamp_note_text = caption
    render.use_stamp = True  # burn into the pixels, not only the file's metadata
    # Legible on a 720p video frame as on a 4K still: about 2.8% of the picture's height.
    render.stamp_font_size = max(12, round(height_pixels * 0.028))
    render.stamp_foreground = (1.0, 1.0, 1.0, 1.0)
    render.stamp_background = (0.0, 0.0, 0.0, 0.62)
    return {"caption": caption, "caption-drawn": True}
