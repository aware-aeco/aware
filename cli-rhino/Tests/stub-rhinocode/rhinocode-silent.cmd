@echo off
REM Silent stub: simulates a `script` invocation where the script never wrote
REM its result file (e.g. it threw before the write, or never ran). Exits 0 with
REM no stdout and no result file — exactly the infra-failure case Exec must catch.
REM `list` still returns canned JSON so metadata resolution works.

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
  REM Deliberately write nothing.
  exit /b 0
)
echo stub-rhinocode-silent: unknown verb '%~1' 1>&2
exit /b 99
