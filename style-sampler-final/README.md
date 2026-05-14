# 多风格音频键盘映射播放器 - 项目说明

## 项目结构
```
style-sampler-final/
├── Cargo.toml                  # 根目录项目配置
├── index.html                  # Web 前端（DJ 台界面）
├── .gitignore
│
├── src-tauri/                  # Tauri Rust 项目
│   ├── Cargo.toml              # Rust 依赖配置
│   ├── tauri.conf.json         # Tauri 应用配置
│   ├── build.rs                # Tauri 构建脚本
│   ├── icons/                  # 应用图标目录
│   │
│   └── src/
│       ├── main.rs             # 应用入口
│       │
│       ├── audio/              # 音频引擎模块
│       │   ├── mod.rs
│       │   ├── engine.rs       # 音频引擎（cpal 输出）
│       │   ├── sample.rs       # 采样管理器（symphonia）
│       │   ├── playhead.rs     # 播放头引擎
│       │   ├── switcher.rs     # 风格切换器
│       │   ├── output.rs       # 音频输出
│       │   └── loop_handler.rs # 循环处理
│       │
│       ├── dsp/                # DSP 效果器
│       │   ├── mod.rs
│       │   ├── filter.rs       # 低通/高通滤波器
│       │   ├── reverb.rs       # 混响
│       │   ├── delay.rs        # 延迟
│       │   ├── distortion.rs   # 失真
│       │   └── chorus.rs       # 合唱
│       │
│       ├── ipc/                # 前后端通信
│       │   ├── mod.rs
│       │   ├── commands.rs     # Tauri 命令
│       │   └── events.rs       # 事件
│       │
│       ├── input/              # 输入处理
│       │   ├── mod.rs
│       │   ├── keyboard.rs     # 全局键盘监听
│       │   └── midi.rs         # MIDI 输入
│       │
│       ├── recorder/           # 录制导出
│       │   ├── mod.rs
│       │   └── exporter.rs     # WAV 导出
│       │
│       └── config/             # 配置管理
│           ├── mod.rs
│           └── settings.rs     # 预设序列化
│
├── tests/                      # 单元测试
│   ├── audio_tests.rs
│   └── dsp_tests.rs
│
└── assets/                     # 资源目录
    └── samples/                # 音频采样目录
```

## 快速开始

### 1. 构建项目
```bash
cd src-tauri
cargo build --release
```

### 2. 使用 Tauri 开发模式（推荐）
```bash
# 在项目根目录
npm init -y
npm install --save-dev @tauri-apps/cli
npx tauri dev
```

### 3. 构建安装包
```bash
npx tauri build
```
构建完成后，安装包位于 `src-tauri/target/release/bundle/`

## 键盘映射

| 键位 | 风格编号 |
|------|----------|
| A | 1 |
| S | 2 |
| D | 3 |
| F | 4 |
| G | 5 |
| H | 6 |
| J | 7 |
| K | 8 |
| L | 9 |
| ; | 10 |
| ' | 11 |

## 功能特性

- ✅ 11 种风格音频采样播放
- ✅ 共享时间线播放
- ✅ 无缝风格切换
- ✅ 循环回跳功能
- ✅ 多种触发模式（门控/触发/循环）
- ✅ 5 种 DSP 效果器
- ✅ 全局键盘监听
- ✅ WAV 录制导出
- ✅ 预设保存和加载

## 音频采样准备

请准备 11 个等长的音频文件（推荐长度 2-8 秒），格式支持：
- MP3
- WAV
- FLAC

将采样文件放在 `assets/samples/` 目录下。

## 技术栈

- **后端音频处理**：Rust + cpal + symphonia
- **桌面应用框架**：Tauri 2.0
- **前端**：HTML/CSS/JavaScript
- **DSP 算法**：自定义音频处理

## 开发

### 项目依赖
```toml
tauri = "2.0"
cpal = "0.15"
symphonia = "0.5"
hound = "3.5"
rdev = "0.5"
```

## License

本项目仅供学习和研究使用。
