"""Progress records for a long render (#405 parity on the builtin transport, #692).

A render can run for minutes or hours, and from the outside a 40-minute animation
looks exactly like a hang. So while a command works it appends one JSON line per
event to the file named by `AWARE_PROGRESS_FILE`:

    {"$aware-progress": {"phase": "render", "done": 3, "total": 8, "message": "pass 1 of ?"}}

The runtime tails that file and mirrors each line into the run trace as a
`node-progress` event. The channel is advisory: with the variable unset every
call here is a no-op, and a write that fails is dropped rather than raised --
progress must never be the reason a render fails.

Frames are counted on `bpy.app.handlers.render_write` and nothing else. Measured
on Blender 5.2.0 (EEVEE and Cycles, a 5-frame FFMPEG animation): `render_write`
fires exactly once per frame written into the output; `render_post` fires before
the write; `frame_change_post` fires an extra time after the job (6 for 5) and
would claim a frame that was never written. Counted progress is enabled only on
versions where that was measured (`COUNTED_VERSIONS`); anywhere else a render
reports its phase with no `done`/`total`, so a consumer shows honest elapsed time
instead of a bar that might lie. A version joins the set only by running the
frame-count test (`tests/test_exec.py`) under it.

Imports no `bpy` at module level, so the record writer is testable in plain
Python; `FrameCounter` imports it when used.
"""

from __future__ import annotations

import json
import math
import os

ENV = "AWARE_PROGRESS_FILE"
RECORD_KEY = "$aware-progress"

# Mirrors `cli/src/runtime/progress.rs`: an over-long record is DROPPED by the
# runtime, so it is kept under the cap here rather than sent and lost.
MAX_RECORD_BYTES = 8 * 1024
MAX_PHASE_CHARS = 64
MAX_MESSAGE_CHARS = 512

# The runtime stops mirroring after 10,000 records from one invocation. Above
# this many frames a pass reports every ceil(total / COALESCE_ABOVE)-th frame
# plus the last, so the cap is never why progress stops.
COALESCE_ABOVE = 5_000

# (major, minor) versions on which `render_write` was measured to fire once per
# written frame. See the module docstring before adding one.
COUNTED_VERSIONS = {(5, 2)}

# The phases a consumer can expect, in the order a command passes through them.
PHASES = ("import", "look", "setup", "render")


def enabled() -> bool:
    """Whether anything is listening -- i.e. the runtime handed us a channel."""
    return bool(os.environ.get(ENV))


def _truncate(text: str, limit: int) -> str:
    return text if len(text) <= limit else text[: max(0, limit - 1)] + "…"


def record(phase, done=None, total=None, message=None) -> dict:
    """The record body for one event, already fitted to the runtime's limits."""
    body = {"phase": _truncate(str(phase), MAX_PHASE_CHARS) or "progress"}
    if done is not None:
        body["done"] = int(done)
    if total is not None:
        body["total"] = int(total)
    if message is not None:
        body["message"] = _truncate(str(message), MAX_MESSAGE_CHARS)
    return body


def line(body: dict) -> str:
    """One channel line for `body`, guaranteed under `MAX_RECORD_BYTES`."""
    text = json.dumps({RECORD_KEY: body}, ensure_ascii=False)
    while len(text.encode("utf-8")) > MAX_RECORD_BYTES and body.get("message"):
        body = dict(body, message=_truncate(body["message"], len(body["message"]) // 2))
        text = json.dumps({RECORD_KEY: body}, ensure_ascii=False)
    return text


def emit(phase, done=None, total=None, message=None) -> None:
    """Append one record to the channel. A no-op without `AWARE_PROGRESS_FILE`."""
    path = os.environ.get(ENV)
    if not path:
        return
    text = line(record(phase, done, total, message))
    try:
        # Open-append-close per record: each line is flushed and on disk before the
        # call returns, so the runtime's next poll sees it.
        with open(path, "a", encoding="utf-8") as handle:
            handle.write(text + "\n")
    except OSError:
        pass  # advisory channel: never fail a render over a progress write


def counting_supported(version=None) -> bool:
    """Whether frame counts are trustworthy on this Blender (see `COUNTED_VERSIONS`)."""
    if version is None:
        import bpy

        version = bpy.app.version
    return tuple(version[:2]) in COUNTED_VERSIONS


def stride_for(total: int) -> int:
    """Report every `stride`-th frame, so one pass never exceeds ~COALESCE_ABOVE records."""
    return math.ceil(total / COALESCE_ABOVE) if total > COALESCE_ABOVE else 1


class FrameCounter:
    """One progress SEGMENT: a single `bpy.ops.render.render` call.

    Each render call gets its own `done`/`total`, starting from 0, with a message
    naming the pass. A cumulative count would make the bar jump backwards the
    moment a second pass added its frames to the total; a new segment is a new
    bar instead.

    Use as a context manager around the render. Handlers are registered on entry
    and always removed on exit, even when the render raises.
    """

    def __init__(self, total: int, label: str, counted: bool | None = None):
        self.total = max(0, int(total))
        self.label = label
        self.counted = counting_supported() if counted is None else counted
        self.stride = stride_for(self.total)
        self.done = 0
        self.completed = False
        self.cancelled = False
        self._handlers = []

    def _on_write(self, *_args):
        self.done += 1
        if self.counted and (self.done % self.stride == 0 or self.done == self.total):
            emit("render", self.done, self.total, self.label)

    def _on_complete(self, *_args):
        self.completed = True

    def _on_cancel(self, *_args):
        self.cancelled = True

    def __enter__(self):
        import bpy

        # Plain closures, not bound methods: Blender keeps handler references in a
        # C-level list and these must be removable by identity on exit.
        def on_write(*args):
            self._on_write(*args)

        def on_complete(*args):
            self._on_complete(*args)

        def on_cancel(*args):
            self._on_cancel(*args)

        self._handlers = [
            (bpy.app.handlers.render_write, on_write),
            (bpy.app.handlers.render_complete, on_complete),
            (bpy.app.handlers.render_cancel, on_cancel),
        ]
        for handler_list, handler in self._handlers:
            handler_list.append(handler)
        # Counted: one record per written frame and nothing else, so `render`
        # records map 1:1 onto frames. Uncounted: one phase-only record, so the
        # consumer knows rendering began without being handed a number.
        if not self.counted:
            emit("render", message=self.label)
        return self

    def __exit__(self, *_exc):
        for handler_list, handler in self._handlers:
            if handler in handler_list:
                handler_list.remove(handler)
        self._handlers = []
        return False
