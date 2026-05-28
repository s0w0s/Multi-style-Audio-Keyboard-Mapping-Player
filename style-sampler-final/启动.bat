@echo off
chcp 936 >nul 2>&1
title Style Sampler

echo ==========================================
echo    Style Sampler - Audio Keyboard Player
echo ==========================================
echo.

if not exist "%~dp0src-tauri" (
    echo [ERROR] Cannot find src-tauri folder!
    echo Please make sure this script is in the project root.
    echo.
    pause
    exit /b 1
)

cd /d "%~dp0src-tauri"
if errorlevel 1 (
    echo [ERROR] Failed to enter src-tauri folder!
    pause
    exit /b 1
)

if not exist "target\release\style-sampler.exe" (
    echo [1/2] Building project (first run, 2-5 min)...
    echo.
    cargo build --release
    if errorlevel 1 (
        echo.
        echo [ERROR] Build failed! Please check Rust installation.
        echo Download Rust: https://rustup.rs
        echo.
        pause
        exit /b 1
    )
    echo.
    echo [DONE] Build successful!
) else (
    echo [SKIP] Build found, starting directly...
)

echo.
echo [2/2] Launching app...
echo.

if not exist "target\release\style-sampler.exe" (
    echo [ERROR] style-sampler.exe not found after build!
    echo.
    pause
    exit /b 1
)

start "" "target\release\style-sampler.exe"

echo App launched!
echo.
echo Keyboard shortcuts:
echo   A-K keys  - Trigger style 1-11
echo   Space     - Stop playback
echo   Tab       - Auto switch style
echo.
echo This window will close in 5 seconds...
timeout /t 5
