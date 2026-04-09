@echo off
echo ============================================
echo   HWID Apply Tool
echo ============================================
echo.

REM Check for admin
net session >nul 2>&1
if %errorlevel% neq 0 (
    echo ERROR: This must be run as Administrator.
    echo Right-click this file and select "Run as administrator"
    pause
    exit /b 1
)

set /p FILENAME="Enter config filename (e.g. target_hwids): "

if "%FILENAME%"=="" (
    echo ERROR: No filename entered.
    pause
    exit /b 1
)

REM Add .json if they didn't type it
if not exist "%FILENAME%" (
    if exist "%FILENAME%.json" (
        set "FILENAME=%FILENAME%.json"
    )
)

if not exist "%FILENAME%" (
    echo ERROR: File "%FILENAME%" not found.
    echo Make sure the file is in this folder.
    pause
    exit /b 1
)

echo.
echo Config file: %FILENAME%
echo.
echo WARNING: This will modify your system hardware IDs.
echo Make sure you have a backup (run backup_hwids.bat first).
echo.
set /p CONFIRM="Type YES to continue: "

if /i not "%CONFIRM%"=="YES" (
    echo Cancelled.
    pause
    exit /b 0
)

echo.
echo Applying hardware IDs from %FILENAME% ...
echo.

hwspoof.exe "%FILENAME%"

echo.
echo ============================================
echo   Done. Reboot for changes to take effect.
echo ============================================
echo.
pause
