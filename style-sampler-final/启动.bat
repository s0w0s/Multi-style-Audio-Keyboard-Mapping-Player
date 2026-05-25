@echo off
chcp 936 >nul 2>&1
title Style Sampler

echo ==========================================
echo    Style Sampler - Audio Keyboard Player
echo ==========================================
echo.

cd /d "%~dp0src-tauri"

if not exist "target\release\style-sampler.exe" (
    echo [1/2] Building project (first run, 2-5 min)...
    echo.
    cargo build --release 2>&1
    if errorlevel 1 (
        echo.
        echo [ERROR] Build failed! Please check Rust installation.
        echo Download Rust: https://rustup.rs
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
start "" "target\release\style-sampler.exe"

echo App launched!
echo.
echo Keyboard shortcuts:
echo   A-K keys  - Trigger style 1-11
echo   Space     - Stop playback
echo   Tab       - Auto switch style
echo.
timeout /t 3 >nul
