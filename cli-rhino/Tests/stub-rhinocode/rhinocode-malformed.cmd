@echo off
REM Malformed stub: simulates a result file that exists and is non-empty but is
REM NOT valid JSON (truncated / garbage write). Exec must surface a "result file
REM JSON parse failed" receipt — and WaitForResultFile's settled-but-unparseable
REM terminal state must kick in so it doesn't poll to the full timeout.
REM `list` still returns canned JSON.

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
  if not "%AWARE_RESULT_PATH%"=="" (
    >"%AWARE_RESULT_PATH%" echo {"ok": true, "result": {not valid json
  )
  exit /b 0
)
echo stub-rhinocode-malformed: unknown verb '%~1' 1>&2
exit /b 99
