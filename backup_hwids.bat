@echo off
echo ============================================
echo   HWID Backup Tool
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

set /p FILENAME="Enter backup filename (e.g. my_backup): "

if "%FILENAME%"=="" (
    echo ERROR: No filename entered.
    pause
    exit /b 1
)

echo.
echo Dumping current hardware IDs to %FILENAME%.json ...
echo.

hwspoof.exe --dump > "%FILENAME%.json"

if %errorlevel% equ 0 (
    echo.
    echo ============================================
    echo   Backup saved to: %FILENAME%.json
    echo   Keep this file safe for restoring later.
    echo ============================================
) else (
    echo.
    echo ERROR: Dump failed. Make sure hwspoof.exe is in this folder.
)

echo.
pause
