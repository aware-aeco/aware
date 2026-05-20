@echo off
REM Error stub: simulates a `script` invocation where the user script RAN but
REM threw — it writes a {"ok":false,"error":...,"stack":...} result file (the
REM most common real non-happy path). `list` still returns canned JSON.

set "STUB_DIR=%~dp0canned-responses"

if "%~1"=="--rhino" (
  shift
  shift
)

if /I "%~1"=="list" (
  type "%STUB_DIR%\list-ok.json"
  exit /b 0
)
if /I "%~1"=="script" (
  if not "%AWARE_RESULT_PATH%"=="" copy /Y "%STUB_DIR%\script-result-err.json" "%AWARE_RESULT_PATH%" >nul
  exit /b 0
)
echo stub-rhinocode-err: unknown verb '%~1' 1>&2
exit /b 99
