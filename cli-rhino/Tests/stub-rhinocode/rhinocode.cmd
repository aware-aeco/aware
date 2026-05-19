@echo off
REM Stub rhinocode for AwareRhino integration tests. Echoes a canned response
REM based on the verb argument: `list` -> list-ok.json, `script` -> script-ok.txt.
REM Honors `--rhino <id>` (global flag, comes BEFORE the subcommand) by
REM recording it to %AWARE_STUB_DUMP_DIR%\last-rhino-id.txt for verification.
REM AWARE_STUB_DUMP_DIR is set by the test fixture so both sides agree on path.

set "STUB_DIR=%~dp0canned-responses"
if "%AWARE_STUB_DUMP_DIR%"=="" set "AWARE_STUB_DUMP_DIR=%TEMP%"

REM Handle leading --rhino <id> global flag.
if "%~1"=="--rhino" goto handle_rhino_flag
goto dispatch_verb

:handle_rhino_flag
echo %~2>"%AWARE_STUB_DUMP_DIR%\last-rhino-id.txt"
shift
shift
goto dispatch_verb

:dispatch_verb
if /I "%~1"=="list" goto do_list
if /I "%~1"=="script" goto do_script
echo stub-rhinocode: unknown verb '%~1' 1>&2
exit /b 99

:do_list
type "%STUB_DIR%\list-ok.json"
exit /b 0

:do_script
if not "%~2"=="" echo %~2>"%AWARE_STUB_DUMP_DIR%\last-script-path.txt"
type "%STUB_DIR%\script-ok.txt"
exit /b 0
