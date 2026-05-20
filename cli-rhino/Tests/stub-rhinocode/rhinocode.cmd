@echo off
REM Stub rhinocode for AwareRhino integration tests. Faithfully SIMULATES real
REM rhinocode behaviour:
REM   - `list`  -> prints canned list JSON to stdout (real `list` does return stdout).
REM   - `script` -> writes a canned {"ok":true,"result":{...}} JSON to the path in
REM                 %AWARE_RESULT_PATH% and prints NOTHING to stdout. Real rhinocode
REM                 runs the script inside the live Rhino process and returns no
REM                 stdout; the script itself writes the result file. The stub
REM                 stands in for "the script ran and wrote its result".
REM Honors `--rhino <id>` (global flag, comes BEFORE the subcommand) by recording
REM it to %AWARE_STUB_DUMP_DIR%\last-rhino-id.txt for verification.
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
REM Record the script path the caller asked us to run, for verification.
if not "%~2"=="" echo %~2>"%AWARE_STUB_DUMP_DIR%\last-script-path.txt"
REM Real rhinocode prints nothing on stdout for `script`. Simulate the script
REM writing its result file to %AWARE_RESULT_PATH% (set by RhinocodeClient).
if not "%AWARE_RESULT_PATH%"=="" copy /Y "%STUB_DIR%\script-result-ok.json" "%AWARE_RESULT_PATH%" >nul
exit /b 0
