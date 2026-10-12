"""`ctx` -- what `exec` hands the author's `run(ctx)` (#692).

The author writes `bpy` code once, when the workflow is authored; AWARE runs it
unchanged on every run. `ctx` is the stable surface that code leans on, so it
does not have to re-derive the agent's own staging (IFC properties, camera fit,
looks, lighting) or the bookkeeping a long render needs:

  ctx.args, ctx.output_dir, ctx.scene
  ctx.elements()                 -> [(obj, {guid, name, class, material, storey})]
  ctx.frame(direction)           camera fitted to the model (`_framing`)
  ctx.apply_look(preset)         IFC-driven materials (`_looks`)
  ctx.environment(name)          HDRI lighting + key light (`_stage`)
  ctx.ground(on)                 the fading floor (`_stage`)
  ctx.progress(phase, done, total, message)
  ctx.plan_passes(n)             say how many render calls are coming
  ctx.render_still(path, quality)
  ctx.render_animation(path, quality)
  ctx.add_output(path, kind)

Outputs are an EXPLICIT receipt, never a directory diff. Only what a `render_*`
call wrote or `add_output` registered is listed; each entry is verified (the file
exists, is a regular non-empty file, and its real path is inside `output_dir`).
An entry that fails is still listed, with `verified: false` and the reason --
never silently dropped, and never passed off as good.
"""

from __future__ import annotations

import os
import shutil
import tempfile

import bpy

import _framing
import _ifc_import
import _looks
import _progress
import _result
import _stage

KINDS = ("video", "image", "blend", "file")
QUALITIES = ("draft", "production")


def _same_or_inside(path: str, root: str) -> bool:
    """Whether real path `path` is `root` or lies beneath it (case-folded on Windows)."""
    path_n, root_n = os.path.normcase(path), os.path.normcase(root)
    try:
        return os.path.commonpath([path_n, root_n]) == root_n
    except ValueError:  # different drives on Windows
        return False


def _identity(path: str) -> tuple[int, int]:
    """What says a file is still the one that was written: its size and mtime."""
    stat = os.stat(path)
    return (stat.st_size, stat.st_mtime_ns)


def eevee_engine() -> str:
    """EEVEE's identifier on this Blender (renamed BLENDER_EEVEE_NEXT in 4.2, back in 5.x)."""
    identifiers = {
        item.identifier
        for item in bpy.types.RenderSettings.bl_rna.properties["engine"].enum_items
    }
    for candidate in ("BLENDER_EEVEE_NEXT", "BLENDER_EEVEE"):
        if candidate in identifiers:
            return candidate
    raise _result.AwareBlenderError(
        _result.ERR_RENDER_FAILED,
        f"no EEVEE engine in this Blender; available: {sorted(identifiers)}",
    )


class ExecContext:
    """The object passed to `run(ctx)`."""

    def __init__(self, args: dict, output_dir: str):
        self.args = dict(args or {})
        os.makedirs(output_dir, exist_ok=True)
        # The REAL path, so the inside-check below compares like with like even
        # when output-dir itself is reached through a link.
        self.output_dir = os.path.realpath(output_dir)
        self.outputs: list[dict] = []
        self.frames = 0
        self._planned_passes: int | None = None
        self._pass = 0

    # -- the scene -----------------------------------------------------------

    @property
    def scene(self):
        return bpy.context.scene

    def elements(self) -> list:
        """Every imported IFC element with the properties staged on it.

        Staging helpers (the ground plane) are not elements and are skipped.
        """
        found = []
        for obj in bpy.data.objects:
            if obj.get(_ifc_import.PROP_HELPER):
                continue
            guid = obj.get(_ifc_import.PROP_GUID)
            ifc_class = obj.get(_ifc_import.PROP_CLASS)
            if not guid and not ifc_class:
                continue
            found.append(
                (
                    obj,
                    {
                        "guid": str(guid or ""),
                        "name": str(obj.get(_ifc_import.PROP_NAME, "") or ""),
                        "class": str(ifc_class or ""),
                        "material": str(obj.get(_ifc_import.PROP_MATERIAL, "") or ""),
                        "storey": str(obj.get(_ifc_import.PROP_STOREY, "") or ""),
                    },
                )
            )
        return found

    def frame(self, direction: str = "hero") -> dict:
        """Fit the scene camera (created if missing) to the model; returns the receipt."""
        if direction not in _framing.DIRECTIONS:
            raise ValueError(
                f"unknown direction `{direction}`; expected one of {sorted(_framing.DIRECTIONS)}"
            )
        camera = _framing.ensure_camera()
        receipt = _framing.frame_camera(camera, direction)
        bpy.context.view_layer.update()
        return receipt

    def apply_look(self, preset: str = "realistic") -> dict:
        return _looks.apply_look(preset)

    def environment(self, name: str = _stage.DEFAULT_ENVIRONMENT) -> dict:
        """Light the scene with a bundled HDRI (or a path, or `gradient`) plus a key light."""
        receipt = _stage.setup_world(name)
        _stage.setup_key_light()
        return receipt

    def ground(self, on: bool = True) -> dict:
        return _stage.setup_ground(bool(on))

    # -- progress ------------------------------------------------------------

    def progress(self, phase, done=None, total=None, message=None) -> None:
        _progress.emit(phase, done, total, message)

    def plan_passes(self, count: int) -> None:
        """Declare how many `render_*` calls follow, so each is labelled `pass i of n`."""
        self._planned_passes = max(1, int(count))

    def _next_pass_label(self) -> str:
        self._pass += 1
        planned = self._planned_passes if self._planned_passes else "?"
        return f"pass {self._pass} of {planned}"

    # -- outputs ---------------------------------------------------------------

    def _resolve(self, path: str) -> str:
        path = os.fspath(path)
        if not os.path.isabs(path):
            path = os.path.join(self.output_dir, path)
        return os.path.abspath(path)

    def _verified(self, path: str, kind: str, reason: str | None = None, **extra) -> dict:
        """Check one file and append its receipt entry.

        `reason` lets a render call record a failure it already knows about
        (no output file was found, or the file changed after it was written)
        in the same entry shape.
        """
        entry = {"path": os.path.abspath(path), "kind": kind}
        entry.update(extra)
        if reason is None:
            reason = self._problem_with(path, kind, entry)
        entry["verified"] = reason is None
        if reason is not None:
            entry["reason"] = reason
        self.outputs.append(entry)
        return entry

    def _problem_with(self, path: str, kind: str, entry: dict) -> str | None:
        """Why `path` is not a verified output (None when it is); fills in its real path and size."""
        reason = None
        if kind not in KINDS:
            reason = f"unknown kind `{kind}`; expected one of {list(KINDS)}"
        elif os.path.islink(path):
            reason = "is a symbolic link, not a file the code wrote"
        elif not os.path.exists(path):
            reason = "was not written"
        elif not os.path.isfile(path):
            reason = "is not a regular file"
        else:
            real = os.path.realpath(path)
            entry["path"] = real
            size = os.path.getsize(real)
            entry["size-bytes"] = size
            if not _same_or_inside(real, self.output_dir):
                reason = f"is outside output-dir ({self.output_dir})"
            elif size <= 0:
                reason = "is empty"
        return reason

    def add_output(self, path: str, kind: str = "file") -> dict:
        """Register a file the code wrote itself; verified exactly like a render."""
        return self._verified(self._resolve(path), kind)

    # -- rendering -------------------------------------------------------------

    def _engine(self, quality: str, samples) -> None:
        if quality not in QUALITIES:
            raise ValueError(f"unknown quality `{quality}`; expected draft or production")
        scene = self.scene
        if quality == "production":
            scene.render.engine = "CYCLES"
            if samples:
                scene.cycles.samples = int(samples)
        else:
            scene.render.engine = eevee_engine()
            if samples and hasattr(scene, "eevee") and hasattr(scene.eevee, "taa_render_samples"):
                scene.eevee.taa_render_samples = int(samples)
            _stage.tune_eevee(scene)

    def _ensure_camera(self) -> None:
        if self.scene.camera is None:
            self.frame("hero")

    def _render_to(self, target: str, kind: str, render, **extra) -> dict:
        """Run `render` into a scratch directory only this call uses, then land its
        one output on `target`.

        The scratch directory sits beside `target` (same volume, so the final move
        is a rename) and is removed whatever happens. Blender's filepath points
        inside it with no extension, so it is free to name the file -- `render.png`,
        or `render0001-0240.mp4` with the frame range a movie gets. Afterwards the
        ONLY candidates are the files in that directory whose extension is the one
        the format being rendered writes (Blender's own `frame_path`, read before
        the render starts); nothing else writing in `output_dir` (an earlier
        run, a concurrent one), and no side file of another type a render hook
        drops into the scratch directory, can be taken for this render's output.
        Rule when there are several: the largest wins -- a movie is one file, so
        several means strays, and the payload is the biggest. The winner is moved
        onto `target` with `os.replace`, which overwrites an existing file there:
        that is what asking for that path means. Its size and modification time
        are recorded before the move and checked again after it, so a file another
        writer put at `target` in between is never passed off as the render.
        """
        directory = os.path.dirname(target)
        os.makedirs(directory, exist_ok=True)
        scratch = tempfile.mkdtemp(prefix=".aware-render-", dir=directory)
        try:
            scene = self.scene
            scene.render.filepath = os.path.join(scratch, "render")
            # The name Blender will give this format's output (`.png`, `.mp4`);
            # `render.file_extension` lags behind a switch to a movie format.
            extension = os.path.splitext(
                scene.render.frame_path(frame=scene.frame_start)
            )[1].lower()
            render()
            produced = [
                os.path.join(scratch, name)
                for name in os.listdir(scratch)
                if os.path.splitext(name)[1].lower() == extension
                and os.path.isfile(os.path.join(scratch, name))
            ]
            if not produced:
                return self._verified(
                    target, kind, reason="no render output file was found", **extra
                )
            chosen = max(produced, key=os.path.getsize)
            written = _identity(chosen)
            os.replace(chosen, target)
        finally:
            shutil.rmtree(scratch, ignore_errors=True)
        try:
            landed = _identity(target)
        except OSError:
            landed = None
        if landed != written:
            return self._verified(
                target,
                kind,
                reason=f"the file at {target} changed after the render wrote it",
                **extra,
            )
        return self._verified(target, kind, **extra)

    def render_still(self, path: str, quality: str = "draft", samples=None) -> dict:
        """Render the current frame to a PNG at `path` (relative paths land in output_dir)."""
        target = self._resolve(path)
        self._engine(quality, samples)
        self._ensure_camera()
        scene = self.scene
        if hasattr(scene.render.image_settings, "media_type"):
            scene.render.image_settings.media_type = "IMAGE"
        scene.render.image_settings.file_format = "PNG"

        def render():
            with _progress.FrameCounter(1, self._next_pass_label()):
                bpy.ops.render.render(write_still=True)

        return self._render_to(target, "image", render)

    def render_animation(self, path: str, quality: str = "draft", samples=None) -> dict:
        """Render the scene's frame range to an H.264 MP4 at `path`.

        Progress: one `render` record per frame actually written (counted on
        `render_write`), as its own segment labelled `pass i of n`.
        """
        target = self._resolve(path)
        self._engine(quality, samples)
        self._ensure_camera()
        scene = self.scene
        # Blender 5.x gates FFMPEG behind `media_type = VIDEO`; assigning it
        # without the switch raises. Older builds have no such property.
        if hasattr(scene.render.image_settings, "media_type"):
            scene.render.image_settings.media_type = "VIDEO"
        scene.render.image_settings.file_format = "FFMPEG"
        scene.render.ffmpeg.format = "MPEG4"
        scene.render.ffmpeg.codec = "H264"
        scene.render.ffmpeg.constant_rate_factor = "MEDIUM"
        scene.render.ffmpeg.ffmpeg_preset = "GOOD"

        step = max(1, int(scene.frame_step))
        total = max(0, (scene.frame_end - scene.frame_start) // step + 1)
        counted = {"done": 0}

        def render():
            with _progress.FrameCounter(total, self._next_pass_label()) as counter:
                try:
                    bpy.ops.render.render(animation=True)
                finally:
                    counted["done"] = counter.done

        try:
            entry = self._render_to(target, "video", render)
        finally:
            self.frames += counted["done"]
        # The entry is the one already in the receipt; it gains the frame count.
        entry["frames"] = counted["done"]
        return entry
