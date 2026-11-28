"""Observe the process tree of a Windows CLI invocation using debug events."""

from concurrent.futures import ThreadPoolExecutor
import ctypes as c
from ctypes import wintypes as w
from pathlib import Path
import subprocess
import tempfile
import time


class ExceptionRecord(c.Structure):
    _fields_ = [('code', w.DWORD), ('flags', w.DWORD), ('record', w.LPVOID),
                ('address', w.LPVOID), ('parameters', w.DWORD), ('info', c.c_size_t * 15)]


class ExceptionInfo(c.Structure):
    _fields_ = [('record', ExceptionRecord), ('first_chance', w.DWORD)]


class ProcessInfo(c.Structure):
    # Only the leading fields of CREATE_PROCESS_DEBUG_INFO are used.
    _fields_ = [('file', w.HANDLE), ('process', w.HANDLE)]


class EventInfo(c.Union):
    _fields_ = [('exception', ExceptionInfo), ('create_process', ProcessInfo), ('dll_file', w.HANDLE)]


class DebugEvent(c.Structure):
    _fields_ = [('code', w.DWORD), ('pid', w.DWORD), ('tid', w.DWORD), ('info', EventInfo)]


def trace_processes(command, *, cwd, env, payload, timeout=100):
    # The launching thread must consume the events. On failure, its exit
    # lets Windows terminate any debuggees still waiting on an event.
    with ThreadPoolExecutor(max_workers=1) as executor:
        return executor.submit(_trace, command, cwd, env, payload, timeout).result()


def _trace(command, cwd, env, payload, timeout):
    kernel = c.WinDLL('kernel32', use_last_error=True)
    kernel.WaitForDebugEvent.argtypes = [c.POINTER(DebugEvent), w.DWORD]
    kernel.ContinueDebugEvent.argtypes = [w.DWORD, w.DWORD, w.DWORD]
    kernel.QueryFullProcessImageNameW.argtypes = [w.HANDLE, w.DWORD, w.LPWSTR, c.POINTER(w.DWORD)]
    kernel.CloseHandle.argtypes = [w.HANDLE]

    debug_process = 1
    exception, create, exit_process, load_dll = 1, 3, 5, 6
    continue_event, exception_not_handled = 0x00010002, 0x80010001
    breakpoint = 0x80000003
    with tempfile.TemporaryFile() as stdin, tempfile.TemporaryFile() as stdout, tempfile.TemporaryFile() as stderr:
        stdin.write(payload)
        stdin.seek(0)
        process = subprocess.Popen(command, cwd=cwd, env=env, stdin=stdin, stdout=stdout, stderr=stderr,
                                   creationflags=debug_process)
        deadline = time.monotonic() + timeout
        pending = {process.pid}
        images = []
        while pending:
            remaining = deadline - time.monotonic()
            if remaining <= 0:
                raise subprocess.TimeoutExpired(command, timeout)
            event = DebugEvent()
            if not kernel.WaitForDebugEvent(c.byref(event), max(1, int(remaining * 1000))):
                if c.get_last_error() == 121:  # ERROR_SEM_TIMEOUT
                    raise subprocess.TimeoutExpired(command, timeout)
                raise c.WinError(c.get_last_error())
            status = continue_event
            if event.code == create:
                pending.add(event.pid)
                info = event.info.create_process
                if info.file:
                    kernel.CloseHandle(info.file)
                name = c.create_unicode_buffer(32768)
                size = w.DWORD(len(name))
                if not kernel.QueryFullProcessImageNameW(info.process, 0, name, c.byref(size)):
                    raise c.WinError(c.get_last_error())
                if event.pid != process.pid:
                    images.append(Path(name.value).name.lower())
            elif event.code == exit_process:
                pending.remove(event.pid)
            elif event.code == load_dll and event.info.dll_file:
                kernel.CloseHandle(event.info.dll_file)
            elif event.code == exception and event.info.exception.record.code != breakpoint:
                status = exception_not_handled
            if not kernel.ContinueDebugEvent(event.pid, event.tid, status):
                raise c.WinError(c.get_last_error())
        code = process.wait(timeout=max(0, deadline - time.monotonic()))
        stdout.seek(0)
        stderr.seek(0)
        return subprocess.CompletedProcess(command, code, stdout.read(), stderr.read()), sorted(set(images))
