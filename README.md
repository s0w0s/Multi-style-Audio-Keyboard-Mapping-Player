<h1 align="center">🎛️ Style Sampler</h1>
<p align="center"><strong>多风格音频键盘映射播放器</strong></p>
<p align="center">将 11 种不同曲风的音频片段映射到电脑键盘，实现实时演奏与无缝风格切换</p>

<p align="center">
  <img src="https://img.shields.io/badge/Tauri-2.0-FFC131?logo=tauri&logoColor=white" alt="Tauri 2.0">
  <img src="https://img.shields.io/badge/Rust-1.70+-DEA584?logo=rust&logoColor=white" alt="Rust">
  <img src="https://img.shields.io/badge/Web%20Audio-API-569A31?logo=webaudio&logoColor=white" alt="Web Audio API">
  <img src="https://img.shields.io/badge/license-MIT-blue" alt="License">
</p>

---

## ✨ 功能特性

- 🎹 **键盘映射演奏** — A~' 共计 11 个键位，分别对应 11 种不同风格，像弹钢琴一样演奏
- 🔀 **无缝风格切换** — 所有风格共享同一条时间线，切换风格时播放头位置保持不变
- 🔁 **灵活循环模式** — 支持始终循环、按住循环、关闭循环三种模式，可自定义循环起点
- 🎚️ **5 种 DSP 效果器** — 低通/高通滤波器、混响、延迟、失真、合唱，实时调节
- 🎵 **Web Audio API 引擎** — 前端内置音频引擎，支持加载真实音频文件或合成器降级播放
- 🎛️ **专业 DJ 风格界面** — 波形可视化、推子电平表、BPM 节拍指示、暗色/亮色主题切换
- ⌨️ **全局键盘监听** — 即使应用在后台也能响应键盘输入（Rust 后端）
- 🎼 **MIDI 输入支持** — 可连接 MIDI 控制器进行演奏
- 🔴 **录音与 WAV 导出** — 内置录制功能，一键导出为 WAV 文件
- 💾 **预设管理** — 保存和加载完整的效果器参数配置

## 🎮 键盘映射

| | | | | | | | | | | |
|---|---|---|---|---|---|---|---|---|---|---|
| **A** | **S** | **D** | **F** | **G** | **H** | **J** | **K** | **L** | **;** | **'** |
| 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 | 10 | 11 |

| 快捷键 | 功能 |
|--------|------|
| 空格 | 停止播放 |
| Tab | 自动切换风格 |
| Esc | 紧急静音 |
| F11 | 全屏切换 |
| Ctrl+O | 打开采样文件夹 |

## 🚀 快速开始

### 前置条件

- [Rust](https://rustup.rs) 1.70+
- [Node.js](https://nodejs.org) 18+（Tauri CLI 需要）

### 安装与运行

```bash
# 克隆仓库
git clone https://github.com/s0w0s/Multi-style-Audio-Keyboard-Mapping-Player.git
cd Multi-style-Audio-Keyboard-Mapping-Player

# 进入项目目录
cd style-sampler-final

# 安装 Tauri CLI
npm install --save-dev @tauri-apps/cli

# 开发模式运行
npx tauri dev

# 构建生产版本
npx tauri build
```

### 一键启动脚本

- **Windows**: 双击 `启动.bat`
- **macOS / Linux**: 运行 `./启动.sh`

## 🎵 音频采样准备

准备 11 个等长的音频文件（推荐 2-8 秒），按顺序命名（仓库里的doppelganger.zip包含了所需要的音频，可以直接使用）：

```
samples/
├── 01.mp3    # 风格 1 → 键 A
├── 02.wav    # 风格 2 → 键 S
├── 03.flac   # 风格 3 → 键 D
├── ...
└── 11.wav    # 风格 11 → 键 '
```

支持的格式：**MP3** · **WAV** · **FLAC** · **OGG** · **AAC** · **M4A**

启动应用后，通过菜单栏 `文件 → 导入` 或点击 `↑ 加载采样` 按钮选择文件夹。

## 🏗️ 技术架构

```
┌──────────────────────────────────────┐
│              Frontend                │
│    HTML / CSS / JavaScript           │
│    Web Audio API                     │
│    Canvas 波形可视化                  │
└──────────────┬───────────────────────┘
               │ Tauri IPC
┌──────────────┴───────────────────────┐
│           Rust Backend               │
│  ┌─────────────────────────────────┐ │
│  │   audio/  cpal + symphonia      │ │
│  │   音频输出 · 采样解码 · 播放头   │ │
│  ├─────────────────────────────────┤ │
│  │   dsp/   滤波 · 混响 · 延迟     │ │
│  │   失真 · 合唱                   │ │
│  ├─────────────────────────────────┤ │
│  │   input/  全局键盘 · MIDI       │ │
│  ├─────────────────────────────────┤ │
│  │   recorder/  WAV 录制导出        │ │
│  ├─────────────────────────────────┤ │
│  │   config/  预设序列化            │ │
│  └─────────────────────────────────┘ │
└──────────────────────────────────────┘
```

| 层级 | 技术 |
|------|------|
| 桌面框架 | Tauri 2.0 |
| 后端语言 | Rust |
| 音频输出 | cpal |
| 音频解码 | symphonia（MP3 / WAV / FLAC） |
| 前端音频 | Web Audio API |
| DSP 算法 | 自定义实现 |
| WAV 导出 | hound |
| 重采样 | rubato |
| 全局按键 | rdev |
| 文件对话框 | rfd |

## 📁 项目结构

```
style-sampler-final/
├── index.html                  # Web 前端界面
├── Cargo.toml                  # 根项目配置
├── 启动.bat / 启动.sh          # 跨平台启动脚本
├── src-tauri/
│   ├── Cargo.toml              # Rust 依赖
│   ├── tauri.conf.json         # Tauri 配置
│   ├── capabilities/           # Tauri 权限声明
│   ├── icons/                  # 应用图标
│   └── src/
│       ├── main.rs             # 应用入口
│       ├── lib.rs              # 库入口
│       ├── audio/              # 音频引擎
│       │   ├── engine.rs       # 音频输出 + FX 链
│       │   ├── sample.rs       # 采样加载管理
│       │   ├── playhead.rs     # 播放头
│       │   ├── switcher.rs     # 风格切换
│       │   └── loop_handler.rs # 循环处理
│       ├── dsp/                # 效果器模块
│       │   ├── filter.rs       # 低通 / 高通
│       │   ├── reverb.rs       # 混响
│       │   ├── delay.rs        # 延迟
│       │   ├── distortion.rs   # 失真
│       │   └── chorus.rs       # 合唱
│       ├── ipc/                # Tauri 命令
│       │   ├── commands.rs     # 19 个 IPC 命令
│       │   └── events.rs       # 事件推送
│       ├── input/              # 输入处理
│       │   ├── keyboard.rs     # 全局键盘
│       │   └── midi.rs         # MIDI 输入
│       ├── recorder/           # 录制导出
│       │   └── exporter.rs     # WAV 导出
│       └── config/             # 配置管理
│           └── settings.rs     # 预设序列化
└── tests/                      # 单元测试
    ├── audio_tests.rs
    └── dsp_tests.rs
```

## 📄 License

MIT © [s0w0s](https://github.com/s0w0s)
