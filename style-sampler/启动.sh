#!/usr/bin/env bash
set -e

echo "╔══════════════════════════════════════════╗"
echo "║   Style Sampler - 多风格音频键盘映射播放器  ║"
echo "╚══════════════════════════════════════════╝"
echo ""

cd "$(dirname "$0")/src-tauri"

if [ ! -f "target/release/style-sampler" ]; then
    echo "[1/2] 首次运行，正在编译项目..."
    echo ""
    cargo build --release 2>&1
    if [ $? -ne 0 ]; then
        echo ""
        echo "[错误] 编译失败！请检查 Rust 环境是否正确安装。"
        echo "下载 Rust: https://rustup.rs"
        exit 1
    fi
    echo ""
    echo "[完成] 编译成功！"
else
    echo "[跳过] 已检测到编译产物，直接启动..."
fi

echo ""
echo "[2/2] 启动应用..."
echo ""
./target/release/style-sampler &

echo "应用已启动！"
echo ""
echo "快捷键说明:"
echo "  A-K 键  → 触发风格 1-11"
echo "  空格键   → 停止播放"
echo "  Tab 键   → 自动切换风格"
