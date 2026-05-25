@echo off
chcp 65001 >nul 2>&1
title Style Sampler - 多风格音频键盘映射播放器

echo ╔══════════════════════════════════════════╗
echo ║   Style Sampler - 多风格音频键盘映射播放器  ║
echo ╚══════════════════════════════════════════╝
echo.

cd /d "%~dp0src-tauri"

if not exist "target\release\style-sampler.exe" (
    echo [1/2] 首次运行，正在编译项目（约需2-5分钟）...
    echo.
    cargo build --release 2>&1
    if errorlevel 1 (
        echo.
        echo [错误] 编译失败！请检查 Rust 环境是否正确安装。
        echo 下载 Rust: https://rustup.rs
        pause
        exit /b 1
    )
    echo.
    echo [完成] 编译成功！
) else (
    echo [跳过] 已检测到编译产物，直接启动...
)

echo.
echo [2/2] 启动应用...
echo.
start "" "target\release\style-sampler.exe"

echo 应用已启动！
echo.
echo 快捷键说明:
echo   A-K 键  → 触发风格 1-11
echo   空格键   → 停止播放
echo   Tab 键   → 自动切换风格
echo.
timeout /t 3 >nul
