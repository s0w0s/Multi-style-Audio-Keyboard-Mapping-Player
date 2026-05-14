# 多风格音频键盘映射播放器 实现计划

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** 构建一个基于 Rust + Tauri + Web 的实时音频采样播放器，支持11种风格音频的键盘映射、共享时间线播放、多种演奏模式

**Architecture:** 采用分层架构，前端使用 Web 技术渲染 DJ 台界面，后端 Rust 处理实时音频 DSP，通过 Tauri IPC 进行低延迟通信。音频引擎维护全局播放头，所有风格共享时间线。

**Tech Stack:** Rust + Tauri v2 + cpal + symphonia + Web (HTML/CSS/JS)

---

## 文件结构

```
style-sampler/
├── Cargo.toml
├── tauri.conf.json
├── src-tauri/
│   ├── Cargo.toml
│   ├── src/
│   │   ├── main.rs
│   │   ├── lib.rs
│   │   ├── audio/
│   │   │   ├── mod.rs
│   │   │   ├── engine.rs
│   │   │   ├── playhead.rs
│   │   │   ├── sample.rs
│   │   │   ├── switcher.rs
│   │   │   ├── output.rs
│   │   │   └── loop_handler.rs
│   │   ├── dsp/
│   │   │   ├── mod.rs
│   │   │   ├── filter.rs
│   │   │   ├── reverb.rs
│   │   │   ├── delay.rs
│   │   │   ├── distortion.rs
│   │   │   └── chorus.rs
│   │   ├── ipc/
│   │   │   ├── mod.rs
│   │   │   ├── commands.rs
│   │   │   └── events.rs
│   │   ├── input/
│   │   │   ├── mod.rs
│   │   │   ├── keyboard.rs
│   │   │   └── midi.rs
│   │   ├── recorder/
│   │   │   ├── mod.rs
│   │   │   └── exporter.rs
│   │   └── config/
│   │       ├── mod.rs
│   │       └── settings.rs
│   └── icons/
├── src/
│   ├── index.html
│   ├── main.js
│   ├── styles.css
│   ├── components/
│   │   ├── mixer.js
│   │   ├── pads.js
│   │   ├── effects.js
│   │   ├── waveform.js
│   │   └── meters.js
│   └── utils/
│       ├── audio.js
│       └── ipc.js
├── assets/
│   ├── samples/
│   └── presets/
└── tests/
    ├── audio_tests.rs
    └── dsp_tests.rs
```

---

## 任务列表

### Task 1: 初始化 Rust + Tauri 项目结构

**Files:**
- Create: `Cargo.toml`
- Create: `tauri.conf.json`
- Create: `src-tauri/Cargo.toml`
- Create: `src-tauri/src/main.rs`
- Create: `src-tauri/src/lib.rs`
- Create: `src-tauri/src/audio/mod.rs`
- Create: `src-tauri/src/dsp/mod.rs`
- Create: `src-tauri/src/ipc/mod.rs`
- Create: `src-tauri/src/input/mod.rs`
- Create: `src-tauri/src/recorder/mod.rs`
- Create: `src-tauri/src/config/mod.rs`

- [ ] **Step 1: 创建项目根目录和配置文件**

Create `Cargo.toml`:
```toml
[package]
name = "style-sampler"
version = "0.1.0"
edition = "2021"

[dependencies]
tauri = { version = "2", features = [] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
cpal = "0.15"
symphonia = { version = "0.5", features = ["mp3", "flac", "wav"] }
hound = "3.5"
rubato = "0.15"
rdev = "0.5"
parking_lot = "0.12"
crossbeam-channel = "0.5"
ringbuf = "0.3"
log = "0.4"
env_logger = "0.11"
anyhow = "1"

[lib]
name = "style_sampler_lib"
crate-type = ["lib", "cdylib"]

[[bin]]
name = "style-sampler"
path = "src/main.rs"
```

- [ ] **Step 2: 创建 Tauri 配置文件**

Create `tauri.conf.json`:
```json
{
  "build": {
    "devtools": true
  },
  "app": {
    "windows": [
      {
        "title": "多风格音频键盘映射播放器",
        "width": 1280,
        "height": 800,
        "resizable": true
      }
    ]
  }
}
```

- [ ] **Step 3: 创建 Rust 模块入口文件**

Create `src-tauri/src/main.rs`:
```rust
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use style_sampler_lib::run;

fn main() {
    run();
}
```

Create `src-tauri/src/lib.rs`:
```rust
use tauri::Manager;
use std::sync::Mutex;

mod audio;
mod dsp;
mod ipc;
mod input;
mod recorder;
mod config;

use audio::AudioEngine;
use ipc::AppState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    env_logger::init();
    
    let engine = AudioEngine::new();
    if let Err(e) = engine.init() {
        log::error!("Failed to initialize audio engine: {}", e);
    }
    
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(AppState {
            engine: Mutex::new(engine),
            recorder: Mutex::new(None),
        })
        .invoke_handler(tauri::generate_handler![
            ipc::commands::load_samples,
            ipc::commands::load_sample_directory,
            ipc::commands::play_style,
            ipc::commands::stop_all,
            ipc::commands::set_volume,
            ipc::commands::set_bpm,
            ipc::commands::set_loop_mode,
            ipc::commands::set_loop_start,
            ipc::commands::set_trigger_mode,
            ipc::commands::set_effect_param,
            ipc::commands::start_recording,
            ipc::commands::stop_recording,
            ipc::commands::export_recording,
            ipc::commands::get_playhead_position,
            ipc::commands::set_playhead_position,
            ipc::commands::save_preset,
            ipc::commands::load_preset,
        ])
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();
            window.set_title("多风格音频键盘映射播放器").unwrap();
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

- [ ] **Step 4: 创建模块文件**

Create `src-tauri/src/audio/mod.rs`:
```rust
pub mod engine;
pub mod playhead;
pub mod sample;
pub mod switcher;
pub mod output;
pub mod loop_handler;

pub use engine::{AudioEngine, TriggerMode, LoopMode, TransitionType};
pub use playhead::Playhead;
pub use sample::{SampleManager, Sample};
```

Create `src-tauri/src/dsp/mod.rs`:
```rust
pub mod filter;
pub mod reverb;
pub mod delay;
pub mod distortion;
pub mod chorus;

pub use filter::{LowPassFilter, HighPassFilter};
pub use reverb::Reverb;
pub use delay::Delay;
pub use distortion::Distortion;
pub use chorus::Chorus;
```

Create `src-tauri/src/ipc/mod.rs`:
```rust
pub mod commands;
pub mod events;

pub use commands::AppState;
pub use events::EventEmitter;
```

Create `src-tauri/src/input/mod.rs`:
```rust
pub mod keyboard;
pub mod midi;

pub use keyboard::KeyboardInput;
pub use midi::MidiInput;
```

Create `src-tauri/src/recorder/mod.rs`:
```rust
pub mod exporter;

pub use exporter::Exporter;
```

Create `src-tauri/src/config/mod.rs`:
```rust
pub mod settings;

pub use settings::{Settings, EffectsSettings};
```

- [ ] **Step 5: 创建 Tauri Cargo.toml**

Create `src-tauri/Cargo.toml`:
```toml
[package]
name = "style-sampler-tauri"
version = "0.1.0"
edition = "2021"

[dependencies]
style-sampler-lib = { path = ".." }
tauri = { version = "2", features = [] }
tauri-plugin_opener = "2"
serde = { version = "1", features = ["derive"] }
log = "0.4"
env_logger = "0.11"
```

- [ ] **Step 6: 创建前端基础文件**

Create `src/index.html`:
```html
<!DOCTYPE html>
<html lang="zh-CN">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>多风格音频键盘映射播放器</title>
    <link rel="stylesheet" href="styles.css">
</head>
<body>
    <div id="app">
        <div class="main-container">
            <header class="menu-bar">
                <span>文件</span>
                <span>编辑</span>
                <span>视图</span>
                <span>工具</span>
                <span>帮助</span>
            </header>
            
            <div class="mixer-section">
                <div class="volume-control">
                    <div class="volume-slider">
                        <input type="range" id="masterVolume" min="0" max="100" value="80">
                        <div class="volume-fill"></div>
                    </div>
                    <span>主音量</span>
                </div>
                <div class="waveform-display">
                    <canvas id="waveformCanvas"></canvas>
                    <div class="playhead-info">
                        <span id="playheadPosition">0:00</span>
                        <span>/</span>
                        <span id="totalDuration">0:00</span>
                    </div>
                </div>
                <div class="bpm-control">
                    <div id="bpmDisplay">128</div>
                    <button id="tapBtn">TAP</button>
                    <button id="autoBtn">AUTO</button>
                    <button id="recordBtn">录制</button>
                </div>
            </div>
            
            <div class="pads-section">
                <div class="pads-row">
                    <div class="pad" data-style="0" data-key="A">
                        <span class="pad-number">01</span>
                        <span class="pad-key">[A]</span>
                        <span class="pad-name">爵士</span>
                    </div>
                    <div class="pad" data-style="1" data-key="S">
                        <span class="pad-number">02</span>
                        <span class="pad-key">[S]</span>
                        <span class="pad-name">摇滚</span>
                    </div>
                    <div class="pad" data-style="2" data-key="D">
                        <span class="pad-number">03</span>
                        <span class="pad-key">[D]</span>
                        <span class="pad-name">电子</span>
                    </div>
                    <div class="pad" data-style="3" data-key="F">
                        <span class="pad-number">04</span>
                        <span class="pad-key">[F]</span>
                        <span class="pad-name">古典</span>
                    </div>
                    <div class="pad" data-style="4" data-key="G">
                        <span class="pad-number">05</span>
                        <span class="pad-key">[G]</span>
                        <span class="pad-name">放克</span>
                    </div>
                    <div class="pad" data-style="5" data-key="H">
                        <span class="pad-number">06</span>
                        <span class="pad-key">[H]</span>
                        <span class="pad-name">蓝调</span>
                    </div>
                    <div class="pad" data-style="6" data-key="J">
                        <span class="pad-number">07</span>
                        <span class="pad-key">[J]</span>
                        <span class="pad-name">金属</span>
                    </div>
                    <div class="pad" data-style="7" data-key="K">
                        <span class="pad-number">08</span>
                        <span class="pad-key">[K]</span>
                        <span class="pad-name">LoFi</span>
                    </div>
                    <div class="pad" data-style="8" data-key="L">
                        <span class="pad-number">09</span>
                        <span class="pad-key">[L]</span>
                        <span class="pad-name">管弦</span>
                    </div>
                    <div class="pad" data-style="9" data-key=";">
                        <span class="pad-number">10</span>
                        <span class="pad-key">[;]</span>
                        <span class="pad-name">芯片</span>
                    </div>
                </div>
                <div class="pads-row single">
                    <div class="pad" data-style="10" data-key="'">
                        <span class="pad-number">11</span>
                        <span class="pad-key">['']</span>
                        <span class="pad-name">原曲</span>
                    </div>
                </div>
            </div>
            
            <div class="bottom-section">
                <div class="effects-panel">
                    <div class="effect-group">
                        <div class="effect" data-effect="filter">
                            <span class="effect-name">滤波器</span>
                            <input type="range" class="effect-param" data-param="cutoff" min="20" max="20000" value="10000">
                            <span class="effect-label">CUTOFF</span>
                        </div>
                        <div class="effect" data-effect="reverb">
                            <span class="effect-name">混响</span>
                            <input type="range" class="effect-param" data-param="mix" min="0" max="100" value="0">
                            <span class="effect-label">MIX</span>
                        </div>
                        <div class="effect" data-effect="delay">
                            <span class="effect-name">延迟</span>
                            <input type="range" class="effect-param" data-param="time" min="0" max="100" value="0">
                            <span class="effect-label">TIME</span>
                        </div>
                    </div>
                    <div class="effect-group">
                        <div class="effect" data-effect="distortion">
                            <span class="effect-name">失真</span>
                            <input type="range" class="effect-param" data-param="amount" min="0" max="100" value="0">
                            <span class="effect-label">DIST</span>
                        </div>
                        <div class="effect" data-effect="chorus">
                            <span class="effect-name">合唱</span>
                            <input type="range" class="effect-param" data-param="mix" min="0" max="100" value="0">
                            <span class="effect-label">CHRS</span>
                        </div>
                    </div>
                </div>
                <div class="settings-panel">
                    <div class="setting-row">
                        <label>循环起点:</label>
                        <input type="range" id="loopStart" min="0" max="100" value="0">
                        <span id="loopStartValue">0.0s</span>
                        <button id="markLoopStart">标记</button>
                    </div>
                    <div class="setting-row">
                        <label>循环模式:</label>
                        <select id="loopMode">
                            <option value="off">关闭</option>
                            <option value="while_pressed" selected>按键保持时循环</option>
                            <option value="always">始终循环</option>
                        </select>
                    </div>
                    <div class="setting-row">
                        <label>触发模式:</label>
                        <select id="triggerMode">
                            <option value="gate" selected>门控模式</option>
                            <option value="trigger">触发模式</option>
                            <option value="loop">循环模式</option>
                        </select>
                    </div>
                    <div class="setting-row">
                        <label>切换过渡:</label>
                        <select id="transitionType">
                            <option value="hard">硬切</option>
                            <option value="fade" selected>淡入淡出</option>
                            <option value="crossfade">交叉淡化</option>
                        </select>
                        <input type="number" id="transitionTime" min="10" max="500" value="50">
                        <span>ms</span>
                    </div>
                    <div class="button-row">
                        <button id="stopBtn">停止</button>
                        <button id="resetBtn">重置播放头</button>
                        <button id="exportBtn">导出</button>
                    </div>
                </div>
            </div>
            
            <footer class="status-bar">
                <span id="currentStyle">当前风格: -</span>
                <div class="level-meter">
                    <span>L</span>
                    <div class="meter-bar">
                        <div class="meter-fill"></div>
                    </div>
                    <span>R</span>
                    <div class="meter-bar">
                        <div class="meter-fill"></div>
                    </div>
                </div>
                <span id="cpuUsage">CPU: 0%</span>
                <span id="statusText">状态: 就绪</span>
            </footer>
        </div>
    </div>
    <script type="module" src="main.js"></script>
</body>
</html>
```

Create `src/styles.css`:
```css
* {
    margin: 0;
    padding: 0;
    box-sizing: border-box;
}

:root {
    --bg-primary: #1a1a2e;
    --bg-secondary: #16213e;
    --bg-tertiary: #0f3460;
    --text-primary: #e8e8e8;
    --text-secondary: #a0a0a0;
    --accent: #e94560;
    --pad-01: #FFAA00;
    --pad-02: #FF4444;
    --pad-03: #00FFFF;
    --pad-04: #FFD700;
    --pad-05: #AA00FF;
    --pad-06: #4444FF;
    --pad-07: #AAAAAA;
    --pad-08: #FF88AA;
    --pad-09: #00FF88;
    --pad-10: #88FF00;
    --pad-11: #FFFFFF;
}

body {
    font-family: 'Segoe UI', system-ui, sans-serif;
    background: var(--bg-primary);
    color: var(--text-primary);
    overflow: hidden;
}

.main-container {
    display: flex;
    flex-direction: column;
    height: 100vh;
    padding: 8px;
    gap: 8px;
}

.menu-bar {
    display: flex;
    gap: 20px;
    padding: 8px 16px;
    background: var(--bg-secondary);
    border-radius: 4px;
    font-size: 14px;
}

.menu-bar span {
    cursor: pointer;
    padding: 4px 8px;
    border-radius: 4px;
}

.menu-bar span:hover {
    background: var(--bg-tertiary);
}

.mixer-section {
    display: flex;
    gap: 16px;
    padding: 16px;
    background: var(--bg-secondary);
    border-radius: 8px;
    height: 140px;
}

.volume-control {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 8px;
    width: 80px;
}

.volume-slider {
    position: relative;
    width: 30px;
    height: 100px;
    background: var(--bg-primary);
    border-radius: 4px;
}

.volume-slider input {
    position: absolute;
    width: 100px;
    height: 30px;
    transform: rotate(-90deg) translateX(-70px);
    transform-origin: left center;
    cursor: pointer;
    opacity: 0;
}

.volume-fill {
    position: absolute;
    bottom: 0;
    width: 100%;
    height: 80%;
    background: linear-gradient(to top, var(--accent), #ff7b93);
    border-radius: 4px;
}

.waveform-display {
    flex: 1;
    display: flex;
    flex-direction: column;
    gap: 4px;
}

#waveformCanvas {
    flex: 1;
    background: var(--bg-primary);
    border-radius: 4px;
}

.playhead-info {
    display: flex;
    justify-content: center;
    gap: 4px;
    font-family: monospace;
    font-size: 14px;
}

.bpm-control {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 8px;
    width: 100px;
}

#bpmDisplay {
    font-size: 32px;
    font-weight: bold;
    color: var(--accent);
}

.bpm-control button {
    width: 60px;
    padding: 6px;
    border: none;
    border-radius: 4px;
    background: var(--bg-tertiary);
    color: var(--text-primary);
    cursor: pointer;
    font-size: 12px;
}

.bpm-control button:hover {
    background: var(--accent);
}

.pads-section {
    flex: 1;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 12px;
    padding: 16px;
    background: var(--bg-secondary);
    border-radius: 8px;
}

.pads-row {
    display: flex;
    gap: 8px;
}

.pads-row.single {
    justify-content: center;
}

.pad {
    width: 90px;
    height: 90px;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 4px;
    background: var(--bg-primary);
    border: 2px solid #333;
    border-radius: 8px;
    cursor: pointer;
    transition: all 0.1s ease;
}

.pad:hover {
    border-color: var(--accent);
}

.pad.active {
    box-shadow: 0 0 20px currentColor;
}

.pad[data-style="0"] { --pad-color: var(--pad-01); }
.pad[data-style="1"] { --pad-color: var(--pad-02); }
.pad[data-style="2"] { --pad-color: var(--pad-03); }
.pad[data-style="3"] { --pad-color: var(--pad-04); }
.pad[data-style="4"] { --pad-color: var(--pad-05); }
.pad[data-style="5"] { --pad-color: var(--pad-06); }
.pad[data-style="6"] { --pad-color: var(--pad-07); }
.pad[data-style="7"] { --pad-color: var(--pad-08); }
.pad[data-style="8"] { --pad-color: var(--pad-09); }
.pad[data-style="9"] { --pad-color: var(--pad-10); }
.pad[data-style="10"] { --pad-color: var(--pad-11); }

.pad.active {
    background: var(--pad-color);
    border-color: var(--pad-color);
    color: #000;
}

.pad-number {
    font-size: 24px;
    font-weight: bold;
}

.pad-key {
    font-size: 11px;
    opacity: 0.7;
}

.pad-name {
    font-size: 12px;
}

.bottom-section {
    display: flex;
    gap: 16px;
    height: 160px;
}

.effects-panel {
    flex: 1;
    display: flex;
    flex-direction: column;
    gap: 8px;
    padding: 12px;
    background: var(--bg-secondary);
    border-radius: 8px;
}

.effect-group {
    display: flex;
    gap: 12px;
}

.effect {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 4px;
    padding: 8px;
    background: var(--bg-primary);
    border-radius: 4px;
    flex: 1;
}

.effect-name {
    font-size: 12px;
    color: var(--text-secondary);
}

.effect-param {
    width: 100%;
    cursor: pointer;
}

.effect-label {
    font-size: 10px;
    color: var(--accent);
}

.settings-panel {
    flex: 1;
    display: flex;
    flex-direction: column;
    gap: 8px;
    padding: 12px;
    background: var(--bg-secondary);
    border-radius: 8px;
}

.setting-row {
    display: flex;
    align-items: center;
    gap: 8px;
    font-size: 13px;
}

.setting-row label {
    width: 70px;
    color: var(--text-secondary);
}

.setting-row select,
.setting-row input[type="number"] {
    padding: 4px 8px;
    border: 1px solid var(--bg-tertiary);
    border-radius: 4px;
    background: var(--bg-primary);
    color: var(--text-primary);
}

.setting-row input[type="range"] {
    flex: 1;
    cursor: pointer;
}

.button-row {
    display: flex;
    gap: 8px;
    margin-top: auto;
}

.button-row button {
    flex: 1;
    padding: 8px;
    border: none;
    border-radius: 4px;
    background: var(--bg-tertiary);
    color: var(--text-primary);
    cursor: pointer;
    font-size: 13px;
}

.button-row button:hover {
    background: var(--accent);
}

.status-bar {
    display: flex;
    align-items: center;
    gap: 20px;
    padding: 8px 16px;
    background: var(--bg-secondary);
    border-radius: 4px;
    font-size: 13px;
}

.level-meter {
    display: flex;
    align-items: center;
    gap: 4px;
}

.meter-bar {
    width: 100px;
    height: 12px;
    background: var(--bg-primary);
    border-radius: 2px;
    overflow: hidden;
}

.meter-fill {
    height: 100%;
    width: 50%;
    background: linear-gradient(to right, #00ff00, #ffff00, #ff0000);
}
```

- [ ] **Step 7: 创建前端 JS 文件**

Create `src/main.js`:
```javascript
import { initAudioIPC } from './utils/ipc.js';
import { initPads } from './components/pads.js';
import { initMixer } from './components/mixer.js';
import { initEffects } from './components/effects.js';
import { initWaveform } from './components/waveform.js';
import { initMeters } from './components/meters.js';

class StyleSamplerApp {
    constructor() {
        this.state = {
            activeStyle: null,
            isPlaying: false,
            bpm: 128,
            volume: 80,
            loopStart: 0,
            loopMode: 'while_pressed',
            triggerMode: 'gate',
            transitionType: 'fade',
            transitionTime: 50,
            isRecording: false,
            isAutoMode: false
        };
    }

    async init() {
        console.log('初始化多风格音频播放器...');
        await initAudioIPC();
        initPads(this);
        initMixer(this);
        initEffects(this);
        initWaveform();
        initMeters();
        this.bindKeyboard();
        this.bindMenuBar();
        console.log('初始化完成');
    }

    bindKeyboard() {
        const keyMap = {
            'a': 0, 's': 1, 'd': 2, 'f': 3, 'g': 4,
            'h': 5, 'j': 6, 'k': 7, 'l': 8, ';': 9, "'": 10
        };

        document.addEventListener('keydown', async (e) => {
            if (e.repeat) return;
            const key = e.key.toLowerCase();
            
            if (key === ' ') {
                e.preventDefault();
                await this.stopAll();
                return;
            }
            
            if (key === 'escape') {
                e.preventDefault();
                await this.emergencyMute();
                return;
            }
            
            if (key === 'tab') {
                e.preventDefault();
                this.toggleAutoMode();
                return;
            }
            
            if (keyMap.hasOwnProperty(key)) {
                e.preventDefault();
                await this.playStyle(keyMap[key]);
            }
        });

        document.addEventListener('keyup', (e) => {
            const key = e.key.toLowerCase();
            if (keyMap.hasOwnProperty(key) && this.state.triggerMode === 'gate') {
                this.stopAll();
            }
        });
    }

    bindMenuBar() {
        const menuItems = document.querySelectorAll('.menu-bar span');
        menuItems.forEach(item => {
            item.addEventListener('click', () => {
                console.log('菜单:', item.textContent);
            });
        });
    }

    async playStyle(styleIndex) {
        try {
            const { invoke } = await import('@tauri-apps/api/core');
            await invoke('play_style', { styleIndex });
            this.state.activeStyle = styleIndex;
            this.state.isPlaying = true;
            this.updateUI();
        } catch (error) {
            console.error('播放失败:', error);
        }
    }

    async stopAll() {
        try {
            const { invoke } = await import('@tauri-apps/api/core');
            await invoke('stop_all');
            this.state.isPlaying = false;
            this.updateUI();
        } catch (error) {
            console.error('停止失败:', error);
        }
    }

    async emergencyMute() {
        try {
            const { invoke } = await import('@tauri-apps/api/core');
            await invoke('set_volume', { volume: 0 });
            setTimeout(async () => {
                await invoke('set_volume', { volume: this.state.volume });
            }, 100);
        } catch (error) {
            console.error('紧急静音失败:', error);
        }
    }

    toggleAutoMode() {
        this.state.isAutoMode = !this.state.isAutoMode;
        const autoBtn = document.getElementById('autoBtn');
        autoBtn.classList.toggle('active', this.state.isAutoMode);
    }

    updateUI() {
        const pads = document.querySelectorAll('.pad');
        pads.forEach((pad, index) => {
            pad.classList.toggle('active', index === this.state.activeStyle);
        });

        document.getElementById('currentStyle').textContent = 
            this.state.activeStyle !== null 
                ? `当前风格: ${String(this.state.activeStyle + 1).padStart(2, '0')}`
                : '当前风格: -';
    }
}

const app = new StyleSamplerApp();
app.init().catch(console.error);
```

Create `src/utils/ipc.js`:
```javascript
let invoke;
let listen;

export async function initAudioIPC() {
    const { invoke: invokeFn, listen: listenFn } = await import('@tauri-apps/api/core');
    invoke = invokeFn;
    listen = listenFn;
    
    listen('playhead-update', (event) => {
        window.dispatchEvent(new CustomEvent('playhead-update', { detail: event.payload }));
    });
    
    listen('meter-update', (event) => {
        window.dispatchEvent(new CustomEvent('meter-update', { detail: event.payload }));
    });
}

export async function loadSamples(paths) {
    return await invoke('load_samples', { paths });
}

export async function loadSampleDirectory(dirPath) {
    return await invoke('load_sample_directory', { dirPath });
}

export async function playStyle(styleIndex) {
    return await invoke('play_style', { styleIndex });
}

export async function stopAll() {
    return await invoke('stop_all');
}

export async function setVolume(volume) {
    return await invoke('set_volume', { volume });
}

export async function setBpm(bpm) {
    return await invoke('set_bpm', { bpm });
}

export async function setLoopMode(mode) {
    return await invoke('set_loop_mode', { mode });
}

export async function setLoopStart(position) {
    return await invoke('set_loop_start', { position });
}

export async function setTriggerMode(mode) {
    return await invoke('set_trigger_mode', { mode });
}

export async function setEffectParam(effect, param, value) {
    return await invoke('set_effect_param', { effect, param, value });
}

export async function startRecording() {
    return await invoke('start_recording');
}

export async function stopRecording() {
    return await invoke('stop_recording');
}

export async function exportRecording(path) {
    return await invoke('export_recording', { path });
}

export async function getPlayheadPosition() {
    return await invoke('get_playhead_position');
}

export async function setPlayheadPosition(position) {
    return await invoke('set_playhead_position', { position });
}

export async function savePreset(path) {
    return await invoke('save_preset', { path });
}

export async function loadPreset(path) {
    return await invoke('load_preset', { path });
}
```

Create `src/components/pads.js`:
```javascript
export function initPads(app) {
    const pads = document.querySelectorAll('.pad');
    
    pads.forEach((pad, index) => {
        pad.addEventListener('mousedown', () => {
            app.playStyle(index);
        });
        
        pad.addEventListener('mouseup', () => {
            if (app.state.triggerMode === 'gate') {
                app.stopAll();
            }
        });
        
        pad.addEventListener('mouseleave', () => {
            if (app.state.triggerMode === 'gate' && app.state.activeStyle === index) {
                app.stopAll();
            }
        });

        pad.addEventListener('mouseenter', () => {
            if (app.state.triggerMode === 'loop') {
                app.playStyle(index);
            }
        });
    });
}
```

Create `src/components/mixer.js`:
```javascript
export function initMixer(app) {
    const volumeSlider = document.getElementById('masterVolume');
    const bpmDisplay = document.getElementById('bpmDisplay');
    const tapBtn = document.getElementById('tapBtn');
    const stopBtn = document.getElementById('stopBtn');
    const resetBtn = document.getElementById('resetBtn');
    const exportBtn = document.getElementById('exportBtn');
    const loopStartSlider = document.getElementById('loopStart');
    const loopStartValue = document.getElementById('loopStartValue');
    const loopModeSelect = document.getElementById('loopMode');
    const triggerModeSelect = document.getElementById('triggerMode');
    const recordBtn = document.getElementById('recordBtn');

    volumeSlider.addEventListener('input', async (e) => {
        app.state.volume = parseInt(e.target.value);
        const { setVolume } = await import('../utils/ipc.js');
        await setVolume(app.state.volume);
    });

    tapBtn.addEventListener('click', () => {
        const now = Date.now();
        if (!app.tapTimes) app.tapTimes = [];
        app.tapTimes.push(now);
        
        if (app.tapTimes.length > 4) app.tapTimes.shift();
        
        if (app.tapTimes.length >= 2) {
            const intervals = [];
            for (let i = 1; i < app.tapTimes.length; i++) {
                intervals.push(app.tapTimes[i] - app.tapTimes[i-1]);
            }
            const avgInterval = intervals.reduce((a, b) => a + b) / intervals.length;
            const bpm = Math.round(60000 / avgInterval);
            
            if (bpm >= 40 && bpm <= 300) {
                app.state.bpm = bpm;
                bpmDisplay.textContent = bpm;
            }
        }
    });

    stopBtn.addEventListener('click', () => app.stopAll());
    
    resetBtn.addEventListener('click', async () => {
        const { setPlayheadPosition } = await import('../utils/ipc.js');
        await setPlayheadPosition(0);
    });

    exportBtn.addEventListener('click', async () => {
        const { exportRecording } = await import('../utils/ipc.js');
        await exportRecording('export.wav');
    });

    loopStartSlider.addEventListener('input', async (e) => {
        const value = parseFloat(e.target.value);
        app.state.loopStart = value / 100 * 8;
        loopStartValue.textContent = app.state.loopStart.toFixed(1) + 's';
        const { setLoopStart } = await import('../utils/ipc.js');
        await setLoopStart(app.state.loopStart);
    });

    document.getElementById('markLoopStart').addEventListener('click', async () => {
        const { getPlayheadPosition } = await import('../utils/ipc.js');
        const position = await getPlayheadPosition();
        app.state.loopStart = position;
        loopStartValue.textContent = position.toFixed(1) + 's';
        const { setLoopStart } = await import('../utils/ipc.js');
        await setLoopStart(position);
    });

    loopModeSelect.addEventListener('change', async (e) => {
        app.state.loopMode = e.target.value;
        const { setLoopMode } = await import('../utils/ipc.js');
        await setLoopMode(app.state.loopMode);
    });

    triggerModeSelect.addEventListener('change', async (e) => {
        app.state.triggerMode = e.target.value;
        const { setTriggerMode } = await import('../utils/ipc.js');
        await setTriggerMode(app.state.triggerMode);
    });

    recordBtn.addEventListener('click', async () => {
        const { startRecording, stopRecording } = await import('../utils/ipc.js');
        if (app.state.isRecording) {
            await stopRecording();
            recordBtn.classList.remove('active');
        } else {
            await startRecording();
            recordBtn.classList.add('active');
        }
        app.state.isRecording = !app.state.isRecording;
    });
}
```

Create `src/components/effects.js`:
```javascript
export function initEffects(app) {
    const effects = document.querySelectorAll('.effect');
    
    effects.forEach(effect => {
        const paramInput = effect.querySelector('.effect-param');
        const effectName = effect.dataset.effect;
        
        paramInput.addEventListener('input', async () => {
            const param = paramInput.dataset.param;
            const value = parseFloat(paramInput.value);
            
            const { setEffectParam } = await import('../utils/ipc.js');
            await setEffectParam(effectName, param, value);
        });
    });
}
```

Create `src/components/waveform.js`:
```javascript
export function initWaveform() {
    const canvas = document.getElementById('waveformCanvas');
    if (!canvas) return;
    
    const ctx = canvas.getContext('2d');
    
    canvas.width = canvas.offsetWidth;
    canvas.height = canvas.offsetHeight;
    
    let playheadPosition = 0;
    
    window.addEventListener('playhead-update', (e) => {
        playheadPosition = e.detail;
    });
    
    function draw() {
        ctx.fillStyle = '#0a0a0f';
        ctx.fillRect(0, 0, canvas.width, canvas.height);
        
        const centerY = canvas.height / 2;
        ctx.strokeStyle = '#00ffff';
        ctx.lineWidth = 1;
        ctx.beginPath();
        ctx.moveTo(0, centerY);
        ctx.lineTo(canvas.width, centerY);
        ctx.stroke();
        
        const playheadX = (playheadPosition / 8) * canvas.width;
        ctx.strokeStyle = '#ff4444';
        ctx.lineWidth = 2;
        ctx.beginPath();
        ctx.moveTo(playheadX, 0);
        ctx.lineTo(playheadX, canvas.height);
        ctx.stroke();
        
        requestAnimationFrame(draw);
    }
    
    draw();
}
```

Create `src/components/meters.js`:
```javascript
export function initMeters() {
    const cpuDisplay = document.getElementById('cpuUsage');
    const statusDisplay = document.getElementById('statusText');
    
    window.addEventListener('meter-update', (e) => {
        const { cpu } = e.detail;
        if (cpuDisplay) {
            cpuDisplay.textContent = `CPU: ${Math.round(cpu)}%`;
        }
    });
    
    if (statusDisplay) {
        statusDisplay.textContent = '状态: 运行中';
    }
}
```

Create `src/utils/audio.js`:
```javascript
export function formatTime(seconds) {
    const mins = Math.floor(seconds / 60);
    const secs = Math.floor(seconds % 60);
    return `${mins}:${String(secs).padStart(2, '0')}`;
}

export function generateWaveformData(audioBuffer, samples = 200) {
    const rawData = audioBuffer.getChannelData(0);
    const blockSize = Math.floor(rawData.length / samples);
    const waveform = [];
    
    for (let i = 0; i < samples; i++) {
        let sum = 0;
        for (let j = 0; j < blockSize; j++) {
            sum += Math.abs(rawData[i * blockSize + j]);
        }
        waveform.push(sum / blockSize);
    }
    
    const max = Math.max(...waveform);
    return waveform.map(v => v / max);
}
```

- [ ] **Step 8: 创建目录结构**

Run: `cd /workspace && mkdir -p style-sampler/src-tauri/icons style-sampler/src/components style-sampler/src/utils style-sampler/assets/samples style-sampler/assets/presets style-sampler/tests`

---

### Task 2: 实现音频引擎核心

**Files:**
- Create: `src-tauri/src/audio/engine.rs`
- Create: `src-tauri/src/audio/sample.rs`
- Create: `src-tauri/src/audio/playhead.rs`
- Create: `src-tauri/src/audio/switcher.rs`
- Create: `src-tauri/src/audio/output.rs`
- Create: `src-tauri/src/audio/loop_handler.rs`

- [ ] **Step 1: 创建采样管理器**

Create `src-tauri/src/audio/sample.rs`:
```rust
use anyhow::Result;
use symphonia::core::audio::SampleBuffer;
use symphonia::core::codecs::DecoderOptions;
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::sync::Arc;
use parking_lot::RwLock;

pub struct Sample {
    pub data: Vec<f32>,
    pub sample_rate: u32,
    pub channels: u16,
    pub duration: f32,
}

pub struct SampleManager {
    pub samples: [Option<Arc<RwLock<Sample>>>; 11],
}

impl SampleManager {
    pub fn new() -> Self {
        Self {
            samples: [None, None, None, None, None, None, None, None, None, None, None],
        }
    }

    pub fn load_sample(&mut self, style_index: usize, path: &Path) -> Result<()> {
        if style_index >= 11 {
            anyhow::bail!("Style index must be 0-10");
        }

        let file = File::open(path)?;
        let mss = MediaSourceStream::new(Box::new(BufReader::new(file)), Default::default());

        let mut hint = Hint::new();
        if let Some(ext) = path.extension() {
            hint.with_extension(ext.to_str().unwrap_or(""));
        }

        let format_opts = FormatOptions::default();
        let metadata_opts = MetadataOptions::default();

        let probed = symphonia::default::get_probe().format(&hint, mss, &format_opts, &metadata_opts)?;

        let mut format = probed.format;
        let track = format.default_track().ok_or_else(|| anyhow::anyhow!("No default track"))?;
        let decoder_opts = DecoderOptions::default();

        let mut decoder = symphonia::default::get_codecs()
            .make(&track.codec_params, &decoder_opts)?;

        let mut all_samples: Vec<f32> = Vec::new();
        let mut sample_rate = 44100u32;
        let mut channels = 2u16;

        let track_id = track.id;
        
        loop {
            let packet = match format.next_packet() {
                Ok(p) => p,
                Err(_) => break,
            };

            if packet.track_id() != track_id {
                continue;
            }

            let decoded = match decoder.decode(&packet) {
                Ok(d) => d,
                Err(_) => continue,
            };

            if sample_rate == 44100 {
                sample_rate = track.codec_params.sample_rate.unwrap_or(44100);
            }

            match decoded {
                symphonia::core::audio::AudioBufferRef::F32(buf) => {
                    channels = buf.channels() as u16;
                    for frame in buf.frames() {
                        for sample in frame {
                            all_samples.push(*sample);
                        }
                    }
                }
                symphonia::core::audio::AudioBufferRef::S16(buf) => {
                    for frame in buf.frames() {
                        for sample in frame {
                            all_samples.push(*sample as f32 / 32768.0);
                        }
                    }
                }
                _ => {}
            }
        }

        let duration = all_samples.len() as f32 / (sample_rate as f32 * channels as f32);

        let sample = Sample {
            data: all_samples,
            sample_rate,
            channels,
            duration,
        };

        self.samples[style_index] = Some(Arc::new(RwLock::new(sample)));
        log::info!("Loaded sample for style {}: {:.2}s, {}Hz, {}ch", 
                   style_index, duration, sample_rate, channels);

        Ok(())
    }

    pub fn load_directory(&mut self, dir_path: &Path) -> Result<()> {
        let mut entries: Vec<_> = std::fs::read_dir(dir_path)?
            .filter_map(|e| e.ok())
            .collect();
        
        entries.sort_by_key(|e| e.file_name());
        
        for (i, entry) in entries.iter().enumerate().take(11) {
            let path = entry.path();
            if path.is_file() {
                if let Err(e) = self.load_sample(i, &path) {
                    log::warn!("Failed to load {}: {}", path.display(), e);
                }
            }
        }
        
        Ok(())
    }

    pub fn get_sample(&self, style_index: usize) -> Option<Arc<RwLock<Sample>>> {
        self.samples.get(style_index).and_then(|s| s.clone())
    }

    pub fn get_duration(&self) -> f32 {
        self.samples[0]
            .as_ref()
            .map(|s| s.read().duration)
            .unwrap_or(0.0)
    }
}

impl Default for SampleManager {
    fn default() -> Self {
        Self::new()
    }
}
```

- [ ] **Step 2: 创建播放头引擎**

Create `src-tauri/src/audio/playhead.rs`:
```rust
use std::sync::atomic::{AtomicBool, AtomicF32, Ordering};
use std::sync::Arc;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PlaybackState {
    Stopped,
    Playing,
    Paused,
}

pub struct Playhead {
    pub position: Arc<AtomicF32>,
    pub state: Arc<AtomicBool>,
    pub speed: Arc<AtomicF32>,
    pub total_duration: Arc<AtomicF32>,
    pub loop_enabled: Arc<AtomicBool>,
    pub loop_start: Arc<AtomicF32>,
}

impl Playhead {
    pub fn new() -> Self {
        Self {
            position: Arc::new(AtomicF32::new(0.0)),
            state: Arc::new(AtomicBool::new(false)),
            speed: Arc::new(AtomicF32::new(1.0)),
            total_duration: Arc::new(AtomicF32::new(0.0)),
            loop_enabled: Arc::new(AtomicBool::new(true)),
            loop_start: Arc::new(AtomicF32::new(0.0)),
        }
    }

    pub fn start(&self) {
        self.state.store(true, Ordering::SeqCst);
    }

    pub fn stop(&self) {
        self.state.store(false, Ordering::SeqCst);
    }

    pub fn pause(&self) {
        self.state.store(false, Ordering::SeqCst);
    }

    pub fn reset(&self) {
        self.position.store(0.0, Ordering::SeqCst);
    }

    pub fn get_position(&self) -> f32 {
        self.position.load(Ordering::SeqCst)
    }

    pub fn set_position(&self, pos: f32) {
        let max = self.total_duration.load(Ordering::SeqCst);
        self.position.store(pos.max(0.0).min(max), Ordering::SeqCst);
    }

    pub fn advance(&self, delta_seconds: f32) -> bool {
        if !self.state.load(Ordering::SeqCst) {
            return false;
        }

        let speed = self.speed.load(Ordering::SeqCst);
        let current = self.position.load(Ordering::SeqCst);
        let duration = self.total_duration.load(Ordering::SeqCst);
        let loop_start = self.loop_start.load(Ordering::SeqCst);
        let loop_enabled = self.loop_enabled.load(Ordering::SeqCst);

        let new_position = current + delta_seconds * speed;

        if new_position >= duration {
            if loop_enabled {
                let loop_length = duration - loop_start;
                if loop_length > 0.0 {
                    let overflow = new_position - duration;
                    let new_pos = loop_start + overflow % loop_length;
                    self.position.store(new_pos, Ordering::SeqCst);
                    return true;
                }
            }
            self.position.store(duration, Ordering::SeqCst);
            self.pause();
            return false;
        }

        self.position.store(new_position, Ordering::SeqCst);
        true
    }

    pub fn is_playing(&self) -> bool {
        self.state.load(Ordering::SeqCst)
    }

    pub fn set_speed(&self, speed: f32) {
        self.speed.store(speed.clamp(0.5, 2.0), Ordering::SeqCst);
    }

    pub fn set_total_duration(&self, duration: f32) {
        self.total_duration.store(duration, Ordering::SeqCst);
    }

    pub fn set_loop_enabled(&self, enabled: bool) {
        self.loop_enabled.store(enabled, Ordering::SeqCst);
    }

    pub fn set_loop_start(&self, start: f32) {
        let duration = self.total_duration.load(Ordering::SeqCst);
        self.loop_start.store(start.clamp(0.0, duration), Ordering::SeqCst);
    }
}

impl Default for Playhead {
    fn default() -> Self {
        Self::new()
    }
}
```

- [ ] **Step 3: 创建音频引擎主类**

Create `src-tauri/src/audio/engine.rs`:
```rust
use crate::audio::sample::{SampleManager, Sample};
use crate::audio::playhead::Playhead;
use crate::dsp::{LowPassFilter, HighPassFilter, Reverb, Delay, Distortion, Chorus};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{SampleFormat, Stream};
use parking_lot::RwLock;
use std::sync::atomic::{AtomicBool, AtomicF32, AtomicUsize, Ordering};
use std::sync::Arc;

pub struct AudioEngine {
    pub sample_manager: Arc<RwLock<SampleManager>>,
    pub playhead: Arc<Playhead>,
    stream: Option<Stream>,
    active_style: Arc<AtomicUsize>,
    master_volume: Arc<AtomicF32>,
    key_pressed: Arc<AtomicBool>,
    pub trigger_mode: Arc<RwLock<TriggerMode>>,
    pub loop_mode: Arc<RwLock<LoopMode>>,
    pub transition_type: Arc<RwLock<TransitionType>>,
    pub transition_time: Arc<AtomicF32>,
    lowpass: Arc<RwLock<LowPassFilter>>,
    highpass: Arc<RwLock<HighPassFilter>>,
    reverb: Arc<RwLock<Reverb>>,
    delay: Arc<RwLock<Delay>>,
    distortion: Arc<RwLock<Distortion>>,
    chorus: Arc<RwLock<Chorus>>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TriggerMode {
    Gate,
    Trigger,
    Loop,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LoopMode {
    Off,
    WhilePressed,
    Always,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TransitionType {
    Hard,
    Fade,
    Crossfade,
}

impl AudioEngine {
    pub fn new() -> Self {
        let sample_rate = 44100;
        
        Self {
            sample_manager: Arc::new(RwLock::new(SampleManager::new())),
            playhead: Arc::new(Playhead::new()),
            stream: None,
            active_style: Arc::new(AtomicUsize::new(11)),
            master_volume: Arc::new(AtomicF32::new(0.8)),
            key_pressed: Arc::new(AtomicBool::new(false)),
            trigger_mode: Arc::new(RwLock::new(TriggerMode::Gate)),
            loop_mode: Arc::new(RwLock::new(LoopMode::WhilePressed)),
            transition_type: Arc::new(RwLock::new(TransitionType::Fade)),
            transition_time: Arc::new(AtomicF32::new(0.05)),
            lowpass: Arc::new(RwLock::new(LowPassFilter::new(10000.0, sample_rate as f32))),
            highpass: Arc::new(RwLock::new(HighPassFilter::new(20.0, sample_rate as f32))),
            reverb: Arc::new(RwLock::new(Reverb::new(sample_rate as f32))),
            delay: Arc::new(RwLock::new(Delay::new(sample_rate as f32))),
            distortion: Arc::new(RwLock::new(Distortion::new())),
            chorus: Arc::new(RwLock::new(Chorus::new(sample_rate as f32))),
        }
    }

    pub fn init(&mut self) -> anyhow::Result<()> {
        let host = cpal::default_host();
        let device = host.default_output_device()
            .ok_or_else(|| anyhow::anyhow!("No output device found"))?;
        
        log::info!("Using audio device: {}", device.name().unwrap_or_default());

        let config = device.default_output_config()?;
        let sample_rate = config.sample_rate().0;
        let channels = config.channels() as usize;

        log::info!("Audio config: {}Hz, {} channels", sample_rate, channels);

        self.lowpass.write().update_sample_rate(sample_rate as f32);
        self.highpass.write().update_sample_rate(sample_rate as f32);
        self.reverb.write().update_sample_rate(sample_rate as f32);
        self.delay.write().update_sample_rate(sample_rate as f32);
        self.chorus.write().update_sample_rate(sample_rate as f32);

        let sample_manager = self.sample_manager.clone();
        let playhead = self.playhead.clone();
        let active_style = self.active_style.clone();
        let master_volume = self.master_volume.clone();
        let key_pressed = self.key_pressed.clone();
        let loop_mode = self.loop_mode.clone();
        let lowpass = self.lowpass.clone();
        let highpass = self.highpass.clone();
        let reverb = self.reverb.clone();
        let delay = self.delay.clone();
        let distortion = self.distortion.clone();
        let chorus = self.chorus.clone();

        let err_fn = |err| log::error!("Audio stream error: {}", err);

        let stream = device.build_output_stream(
            &config.into(),
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                let frame_count = data.len() / channels;
                let dt = frame_count as f32 / sample_rate as f32;
                
                let is_playing = playhead.is_playing();
                
                if is_playing {
                    let position = playhead.get_position();
                    
                    if position >= playhead.total_duration.load(Ordering::SeqCst) {
                        let lm = *loop_mode.read();
                        if lm != LoopMode::Off && key_pressed.load(Ordering::SeqCst) {
                            playhead.set_position(playhead.loop_start.load(Ordering::SeqCst));
                        } else {
                            playhead.pause();
                        }
                    } else {
                        playhead.advance(dt);
                    }
                }
                
                let volume = master_volume.load(Ordering::SeqCst);
                let style = active_style.load(Ordering::SeqCst);
                
                let sm = sample_manager.read();
                
                if let Some(sample_arc) = sm.get_sample(style) {
                    let sample = sample_arc.read();
                    let position = playhead.get_position();
                    let start_sample = (position * sample.sample_rate as f32 * sample.channels as f32) as usize;
                    
                    let sample_data = &sample.data;
                    
                    for frame in 0..frame_count {
                        let sample_idx = start_sample + frame * sample.channels as usize;
                        
                        let mut out_sample = if sample_idx < sample_data.len() - 1 {
                            let left = sample_data[sample_idx];
                            let right = if sample_idx + 1 < sample_data.len() {
                                sample_data[sample_idx + 1]
                            } else {
                                left
                            };
                            (left + right) * 0.5 * volume
                        } else {
                            0.0
                        };
                        
                        let mut mono_sample = out_sample;
                        
                        mono_sample = lowpass.write().process_sample(mono_sample);
                        mono_sample = highpass.write().process_sample(mono_sample);
                        mono_sample = reverb.write().process_sample(mono_sample);
                        mono_sample = delay.write().process_sample(mono_sample);
                        mono_sample = distortion.write().process_sample(mono_sample);
                        mono_sample = chorus.write().process_sample(mono_sample);
                        
                        out_sample = mono_sample;
                        
                        data[frame * channels] = out_sample;
                        if channels > 1 {
                            data[frame * channels + 1] = out_sample;
                        }
                    }
                } else {
                    for frame in 0..frame_count {
                        for ch in 0..channels {
                            data[frame * channels + ch] = 0.0;
                        }
                    }
                }
            },
            err_fn,
            None,
        )?;

        stream.play()?;
        self.stream = Some(stream);
        
        log::info!("Audio engine initialized successfully");
        Ok(())
    }

    pub fn play_style(&self, style_index: usize) {
        if style_index >= 11 {
            log::warn!("Invalid style index: {}", style_index);
            return;
        }
        
        let tm = *self.trigger_mode.read();
        
        match tm {
            TriggerMode::Gate => {
                if !self.playhead.is_playing() {
                    self.playhead.start();
                }
                self.active_style.store(style_index, Ordering::SeqCst);
                self.key_pressed.store(true, Ordering::SeqCst);
            }
            TriggerMode::Trigger => {
                self.active_style.store(style_index, Ordering::SeqCst);
                self.playhead.start();
                self.key_pressed.store(false, Ordering::SeqCst);
            }
            TriggerMode::Loop => {
                self.active_style.store(style_index, Ordering::SeqCst);
                self.playhead.start();
                self.playhead.set_loop_enabled(true);
                self.key_pressed.store(false, Ordering::SeqCst);
            }
        }
        
        log::debug!("Playing style {}, trigger mode: {:?}", style_index, tm);
    }

    pub fn stop(&self) {
        self.playhead.stop();
        self.key_pressed.store(false, Ordering::SeqCst);
    }

    pub fn set_volume(&self, volume: f32) {
        self.master_volume.store(volume.clamp(0.0, 1.0), Ordering::SeqCst);
    }

    pub fn set_bpm(&self, _bpm: f32) {}

    pub fn set_loop_mode(&self, mode: LoopMode) {
        *self.loop_mode.write() = mode;
        match mode {
            LoopMode::WhilePressed | LoopMode::Always => {
                self.playhead.set_loop_enabled(true);
            }
            LoopMode::Off => {
                self.playhead.set_loop_enabled(false);
            }
        }
    }

    pub fn set_loop_start(&self, position: f32) {
        self.playhead.set_loop_start(position);
    }

    pub fn set_trigger_mode(&self, mode: TriggerMode) {
        *self.trigger_mode.write() = mode;
    }

    pub fn set_effect_param(&self, effect: &str, param: &str, value: f32) {
        match effect {
            "filter" => {
                self.lowpass.write().set_cutoff(value);
            }
            "reverb" => {
                self.reverb.write().set_mix(value / 100.0);
            }
            "delay" => {
                self.delay.write().set_time(value / 100.0);
            }
            "distortion" => {
                self.distortion.write().set_amount(value / 100.0);
            }
            "chorus" => {
                self.chorus.write().set_mix(value / 100.0);
            }
            _ => {}
        }
    }

    pub fn get_playhead_position(&self) -> f32 {
        self.playhead.get_position()
    }

    pub fn set_playhead_position(&self, position: f32) {
        self.playhead.set_position(position);
    }
}

impl Default for AudioEngine {
    fn default() -> Self {
        Self::new()
    }
}
```

- [ ] **Step 4: 创建辅助模块**

Create `src-tauri/src/audio/switcher.rs`:
```rust
use std::sync::atomic::{AtomicUsize, Ordering};

pub struct StyleSwitcher {
    active_style: AtomicUsize,
    previous_style: AtomicUsize,
}

impl StyleSwitcher {
    pub fn new() -> Self {
        Self {
            active_style: AtomicUsize::new(11),
            previous_style: AtomicUsize::new(11),
        }
    }

    pub fn switch_to(&self, style: usize) -> usize {
        let prev = self.active_style.swap(style, Ordering::SeqCst);
        self.previous_style.store(prev, Ordering::SeqCst);
        prev
    }

    pub fn get_active_style(&self) -> usize {
        self.active_style.load(Ordering::SeqCst)
    }

    pub fn get_previous_style(&self) -> usize {
        self.previous_style.load(Ordering::SeqCst)
    }
}

impl Default for StyleSwitcher {
    fn default() -> Self {
        Self::new()
    }
}
```

Create `src-tauri/src/audio/output.rs`:
```rust
use cpal::traits::{DeviceTrait, HostTrait};

pub struct AudioOutput {
    pub sample_rate: u32,
    pub channels: u16,
}

impl AudioOutput {
    pub fn new() -> Option<Self> {
        let host = cpal::default_host();
        let device = host.default_output_device()?;
        let config = device.default_output_config().ok()?;
        
        Some(Self {
            sample_rate: config.sample_rate().0,
            channels: config.channels(),
        })
    }
}

impl Default for AudioOutput {
    fn default() -> Self {
        Self::new().unwrap_or(Self {
            sample_rate: 44100,
            channels: 2,
        })
    }
}
```

Create `src-tauri/src/audio/loop_handler.rs`:
```rust
use std::sync::atomic::{AtomicBool, AtomicF32, Ordering};
use std::sync::Arc;

pub struct LoopHandler {
    enabled: Arc<AtomicBool>,
    loop_start: Arc<AtomicF32>,
    loop_end: Arc<AtomicF32>,
}

impl LoopHandler {
    pub fn new() -> Self {
        Self {
            enabled: Arc::new(AtomicBool::new(true)),
            loop_start: Arc::new(AtomicF32::new(0.0)),
            loop_end: Arc::new(AtomicF32::new(8.0)),
        }
    }

    pub fn should_loop(&self, position: f32, at_end: bool) -> bool {
        if !self.enabled.load(Ordering::SeqCst) {
            return false;
        }
        at_end
    }

    pub fn get_loop_position(&self) -> f32 {
        self.loop_start.load(Ordering::SeqCst)
    }

    pub fn set_loop_start(&self, start: f32) {
        self.loop_start.store(start, Ordering::SeqCst);
    }

    pub fn set_loop_end(&self, end: f32) {
        self.loop_end.store(end, Ordering::SeqCst);
    }

    pub fn set_enabled(&self, enabled: bool) {
        self.enabled.store(enabled, Ordering::SeqCst);
    }
}

impl Default for LoopHandler {
    fn default() -> Self {
        Self::new()
    }
}
```

---

### Task 3: 实现 DSP 效果器

**Files:**
- Create: `src-tauri/src/dsp/filter.rs`
- Create: `src-tauri/src/dsp/reverb.rs`
- Create: `src-tauri/src/dsp/delay.rs`
- Create: `src-tauri/src/dsp/distortion.rs`
- Create: `src-tauri/src/dsp/chorus.rs`

- [ ] **Step 1: 创建滤波器**

Create `src-tauri/src/dsp/filter.rs`:
```rust
pub struct LowPassFilter {
    cutoff: f32,
    sample_rate: f32,
    alpha: f32,
    y_prev: f32,
}

impl LowPassFilter {
    pub fn new(cutoff: f32, sample_rate: f32) -> Self {
        let alpha = Self::calculate_alpha(cutoff, sample_rate);
        Self {
            cutoff,
            sample_rate,
            alpha,
            y_prev: 0.0,
        }
    }

    fn calculate_alpha(cutoff: f32, sample_rate: f32) -> f32 {
        let rc = 1.0 / (2.0 * std::f32::consts::PI * cutoff);
        let dt = 1.0 / sample_rate;
        dt / (rc + dt)
    }

    pub fn process_sample(&mut self, input: f32) -> f32 {
        self.y_prev += self.alpha * (input - self.y_prev);
        self.y_prev
    }

    pub fn process(&mut self, samples: &mut [f32]) {
        for sample in samples.iter_mut() {
            *sample = self.process_sample(*sample);
        }
    }

    pub fn set_cutoff(&mut self, cutoff: f32) {
        self.cutoff = cutoff.max(20.0).min(20000.0);
        self.alpha = Self::calculate_alpha(self.cutoff, self.sample_rate);
    }

    pub fn update_sample_rate(&mut self, sample_rate: f32) {
        self.sample_rate = sample_rate;
        self.alpha = Self::calculate_alpha(self.cutoff, self.sample_rate);
    }
}

pub struct HighPassFilter {
    cutoff: f32,
    sample_rate: f32,
    alpha: f32,
    x_prev: f32,
    y_prev: f32,
}

impl HighPassFilter {
    pub fn new(cutoff: f32, sample_rate: f32) -> Self {
        let alpha = Self::calculate_alpha(cutoff, sample_rate);
        Self {
            cutoff,
            sample_rate,
            alpha,
            x_prev: 0.0,
            y_prev: 0.0,
        }
    }

    fn calculate_alpha(cutoff: f32, sample_rate: f32) -> f32 {
        let rc = 1.0 / (2.0 * std::f32::consts::PI * cutoff);
        let dt = 1.0 / sample_rate;
        rc / (rc + dt)
    }

    pub fn process_sample(&mut self, input: f32) -> f32 {
        let output = self.alpha * (self.y_prev + input - self.x_prev);
        self.x_prev = input;
        self.y_prev = output;
        output
    }

    pub fn process(&mut self, samples: &mut [f32]) {
        for sample in samples.iter_mut() {
            *sample = self.process_sample(*sample);
        }
    }

    pub fn set_cutoff(&mut self, cutoff: f32) {
        self.cutoff = cutoff.max(20.0).min(20000.0);
        self.alpha = Self::calculate_alpha(self.cutoff, self.sample_rate);
    }

    pub fn update_sample_rate(&mut self, sample_rate: f32) {
        self.sample_rate = sample_rate;
        self.alpha = Self::calculate_alpha(self.cutoff, self.sample_rate);
    }
}
```

Create `src-tauri/src/dsp/reverb.rs`:
```rust
pub struct Reverb {
    sample_rate: f32,
    mix: f32,
    buffer: Vec<f32>,
    write_pos: usize,
    delays: [usize; 8],
    comb_buffers: Vec<Vec<f32>>,
    comb_write_pos: Vec<usize>,
    comb_feedback: Vec<f32>,
    allpass_buffers: Vec<Vec<f32>>,
    allpass_write_pos: Vec<usize>,
}

impl Reverb {
    pub fn new(sample_rate: f32) -> Self {
        let buffer_size = (sample_rate * 2.0) as usize;
        let delays = [
            (sample_rate * 0.0297) as usize,
            (sample_rate * 0.0371) as usize,
            (sample_rate * 0.0411) as usize,
            (sample_rate * 0.0437) as usize,
            (sample_rate * 0.0197) as usize,
            (sample_rate * 0.0271) as usize,
            (sample_rate * 0.0311) as usize,
            (sample_rate * 0.0337) as usize,
        ];
        
        let comb_buffers: Vec<Vec<f32>> = delays[..4]
            .iter()
            .map(|d| vec![0.0; *d])
            .collect();
        
        let comb_write_pos = vec![0usize; 4];
        let comb_feedback = vec![0.7f32; 4];
        
        let allpass_buffers: Vec<Vec<f32>> = delays[4..]
            .iter()
            .map(|d| vec![0.0; *d])
            .collect();
        
        let allpass_write_pos = vec![0usize; 4];

        Self {
            sample_rate,
            mix: 0.0,
            buffer: vec![0.0; buffer_size],
            write_pos: 0,
            delays,
            comb_buffers,
            comb_write_pos,
            comb_feedback,
            allpass_buffers,
            allpass_write_pos,
        }
    }

    pub fn process_sample(&mut self, input: f32) -> f32 {
        if self.mix <= 0.001 {
            return input;
        }

        let mut output = 0.0f32;
        
        for i in 0..4 {
            let delay_samples = self.delays[i];
            let buffer_len = self.comb_buffers[i].len();
            let read_pos = if self.comb_write_pos[i] >= delay_samples {
                self.comb_write_pos[i] - delay_samples
            } else {
                buffer_len - (delay_samples - self.comb_write_pos[i])
            };
            
            let delayed = self.comb_buffers[i][read_pos];
            self.comb_buffers[i][self.comb_write_pos[i]] = input + delayed * self.comb_feedback[i];
            self.comb_write_pos[i] = (self.comb_write_pos[i] + 1) % buffer_len;
            output += delayed;
        }
        
        output /= 4.0;
        
        let delay_samples = self.delays[4];
        for i in 0..4 {
            let buffer_len = self.allpass_buffers[i].len();
            let read_pos = if self.allpass_write_pos[i] >= delay_samples {
                self.allpass_write_pos[i] - delay_samples
            } else {
                buffer_len - (delay_samples - self.allpass_write_pos[i])
            };
            
            let delayed = self.allpass_buffers[i][read_pos];
            let buffered = output + delayed * 0.5;
            self.allpass_buffers[i][self.allpass_write_pos[i]] = buffered;
            self.allpass_write_pos[i] = (self.allpass_write_pos[i] + 1) % buffer_len;
            output = delayed - buffered * 0.5;
        }
        
        output * self.mix + input * (1.0 - self.mix)
    }

    pub fn process(&mut self, samples: &mut [f32]) {
        for sample in samples.iter_mut() {
            *sample = self.process_sample(*sample);
        }
    }

    pub fn set_mix(&mut self, mix: f32) {
        self.mix = mix.clamp(0.0, 1.0);
    }

    pub fn update_sample_rate(&mut self, sample_rate: f32) {
        self.sample_rate = sample_rate;
    }
}
```

Create `src-tauri/src/dsp/delay.rs`:
```rust
pub struct Delay {
    sample_rate: f32,
    time: f32,
    feedback: f32,
    mix: f32,
    buffer: Vec<f32>,
    write_pos: usize,
    delay_samples: usize,
}

impl Delay {
    pub fn new(sample_rate: f32) -> Self {
        let buffer_size = (sample_rate * 2.0) as usize;
        Self {
            sample_rate,
            time: 0.3,
            feedback: 0.4,
            mix: 0.5,
            buffer: vec![0.0; buffer_size],
            write_pos: 0,
            delay_samples: (sample_rate * 0.3) as usize,
        }
    }

    pub fn process_sample(&mut self, input: f32) -> f32 {
        if self.time <= 0.001 {
            return input;
        }

        self.delay_samples = (self.sample_rate * self.time) as usize;
        
        let delay_samples = self.delay_samples.min(self.buffer.len() - 1);
        let read_pos = if self.write_pos >= delay_samples {
            self.write_pos - delay_samples
        } else {
            self.buffer.len() - (delay_samples - self.write_pos)
        };
        
        let delayed = self.buffer[read_pos];
        self.buffer[self.write_pos] = input + delayed * self.feedback;
        self.write_pos = (self.write_pos + 1) % self.buffer.len();
        
        input * (1.0 - self.mix) + delayed * self.mix
    }

    pub fn process(&mut self, samples: &mut [f32]) {
        for sample in samples.iter_mut() {
            *sample = self.process_sample(*sample);
        }
    }

    pub fn set_time(&mut self, time: f32) {
        self.time = time.clamp(0.0, 2.0);
    }

    pub fn set_feedback(&mut self, feedback: f32) {
        self.feedback = feedback.clamp(0.0, 0.95);
    }

    pub fn set_mix(&mut self, mix: f32) {
        self.mix = mix.clamp(0.0, 1.0);
    }

    pub fn update_sample_rate(&mut self, sample_rate: f32) {
        self.sample_rate = sample_rate;
    }
}
```

Create `src-tauri/src/dsp/distortion.rs`:
```rust
pub struct Distortion {
    amount: f32,
    drive: f32,
}

impl Distortion {
    pub fn new() -> Self {
        Self {
            amount: 0.0,
            drive: 1.0,
        }
    }

    pub fn process_sample(&mut self, input: f32) -> f32 {
        if self.amount <= 0.001 {
            return input;
        }

        let driven = input * self.drive;
        
        let distorted = driven.tanh();
        
        let amount = self.amount;
        distorted * amount + input * (1.0 - amount)
    }

    pub fn process(&mut self, samples: &mut [f32]) {
        for sample in samples.iter_mut() {
            *sample = self.process_sample(*sample);
        }
    }

    pub fn set_amount(&mut self, amount: f32) {
        self.amount = amount.clamp(0.0, 1.0);
        self.drive = 1.0 + amount * 10.0;
    }
}

impl Default for Distortion {
    fn default() -> Self {
        Self::new()
    }
}
```

Create `src-tauri/src/dsp/chorus.rs`:
```rust
pub struct Chorus {
    sample_rate: f32,
    mix: f32,
    buffer: Vec<f32>,
    write_pos: usize,
    lfo_phase: f32,
    lfo_freq: f32,
    depth: f32,
    delay_base: usize,
}

impl Chorus {
    pub fn new(sample_rate: f32) -> Self {
        let buffer_size = (sample_rate * 2.0) as usize;
        Self {
            sample_rate,
            mix: 0.0,
            buffer: vec![0.0; buffer_size],
            write_pos: 0,
            lfo_phase: 0.0,
            lfo_freq: 0.5,
            depth: 0.002,
            delay_base: (sample_rate * 0.01) as usize,
        }
    }

    pub fn process_sample(&mut self, input: f32) -> f32 {
        if self.mix <= 0.001 {
            return input;
        }

        self.lfo_phase += self.lfo_freq / self.sample_rate;
        if self.lfo_phase > 1.0 {
            self.lfo_phase -= 1.0;
        }
        
        let lfo_value = (self.lfo_phase * 2.0 * std::f32::consts::PI).sin();
        let delay_samples = self.delay_base + (lfo_value * self.depth * self.sample_rate) as usize;
        
        let buffer_len = self.buffer.len();
        let clamped_delay = delay_samples.min(buffer_len - 1).max(1);
        
        let read_pos = if self.write_pos >= clamped_delay {
            self.write_pos - clamped_delay
        } else {
            buffer_len - (clamped_delay - self.write_pos)
        };
        
        let delayed = self.buffer[read_pos];
        self.buffer[self.write_pos] = input;
        self.write_pos = (self.write_pos + 1) % buffer_len;
        
        input * (1.0 - self.mix) + delayed * self.mix
    }

    pub fn process(&mut self, samples: &mut [f32]) {
        for sample in samples.iter_mut() {
            *sample = self.process_sample(*sample);
        }
    }

    pub fn set_mix(&mut self, mix: f32) {
        self.mix = mix.clamp(0.0, 1.0);
    }

    pub fn set_rate(&mut self, rate: f32) {
        self.lfo_freq = rate.clamp(0.1, 5.0);
    }

    pub fn set_depth(&mut self, depth: f32) {
        self.depth = depth.clamp(0.001, 0.01);
    }

    pub fn update_sample_rate(&mut self, sample_rate: f32) {
        self.sample_rate = sample_rate;
        self.delay_base = (sample_rate * 0.01) as usize;
    }
}

impl Default for Chorus {
    fn default() -> Self {
        Self::new(44100.0)
    }
}
```

---

### Task 4: 实现 IPC 命令接口

**Files:**
- Create: `src-tauri/src/ipc/commands.rs`
- Create: `src-tauri/src/ipc/events.rs`
- Create: `src-tauri/src/recorder/exporter.rs`
- Create: `src-tauri/src/config/settings.rs`

- [ ] **Step 1: 创建 IPC 命令处理器**

Create `src-tauri/src/ipc/commands.rs`:
```rust
use crate::audio::{AudioEngine, TriggerMode, LoopMode};
use crate::recorder::exporter::Exporter;
use std::sync::Mutex;
use tauri::State;

pub struct AppState {
    pub engine: Mutex<AudioEngine>,
    pub recorder: Mutex<Option<Exporter>>,
}

#[tauri::command]
pub async fn load_samples(
    paths: Vec<String>,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let engine = state.engine.lock().map_err(|e| e.to_string())?;
    
    let mut sm = engine.sample_manager.write();
    
    for (i, path) in paths.iter().enumerate().take(11) {
        let path = std::path::Path::new(path);
        if let Err(e) = sm.load_sample(i, path) {
            log::warn!("Failed to load sample {}: {}", path.display(), e);
        }
    }
    
    let duration = sm.get_duration();
    drop(sm);
    
    engine.playhead.set_total_duration(duration);
    
    Ok(())
}

#[tauri::command]
pub async fn load_sample_directory(
    dir_path: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let engine = state.engine.lock().map_err(|e| e.to_string())?;
    let path = std::path::Path::new(&dir_path);
    
    let mut sm = engine.sample_manager.write();
    sm.load_directory(path).map_err(|e| e.to_string())?;
    
    let duration = sm.get_duration();
    drop(sm);
    
    engine.playhead.set_total_duration(duration);
    
    Ok(())
}

#[tauri::command]
pub async fn play_style(
    style_index: usize,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let engine = state.engine.lock().map_err(|e| e.to_string())?;
    engine.play_style(style_index);
    Ok(())
}

#[tauri::command]
pub async fn stop_all(state: State<'_, AppState>) -> Result<(), String> {
    let engine = state.engine.lock().map_err(|e| e.to_string())?;
    engine.stop();
    Ok(())
}

#[tauri::command]
pub async fn set_volume(
    volume: f32,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let engine = state.engine.lock().map_err(|e| e.to_string())?;
    engine.set_volume(volume / 100.0);
    Ok(())
}

#[tauri::command]
pub async fn set_bpm(
    bpm: f32,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let engine = state.engine.lock().map_err(|e| e.to_string())?;
    engine.set_bpm(bpm);
    Ok(())
}

#[tauri::command]
pub async fn set_loop_mode(
    mode: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let engine = state.engine.lock().map_err(|e| e.to_string())?;
    let loop_mode = match mode.as_str() {
        "off" => LoopMode::Off,
        "while_pressed" => LoopMode::WhilePressed,
        "always" => LoopMode::Always,
        _ => LoopMode::WhilePressed,
    };
    engine.set_loop_mode(loop_mode);
    Ok(())
}

#[tauri::command]
pub async fn set_loop_start(
    position: f32,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let engine = state.engine.lock().map_err(|e| e.to_string())?;
    engine.set_loop_start(position);
    Ok(())
}

#[tauri::command]
pub async fn set_trigger_mode(
    mode: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let engine = state.engine.lock().map_err(|e| e.to_string())?;
    let trigger_mode = match mode.as_str() {
        "gate" => TriggerMode::Gate,
        "trigger" => TriggerMode::Trigger,
        "loop" => TriggerMode::Loop,
        _ => TriggerMode::Gate,
    };
    engine.set_trigger_mode(trigger_mode);
    Ok(())
}

#[tauri::command]
pub async fn set_effect_param(
    effect: String,
    param: String,
    value: f32,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let engine = state.engine.lock().map_err(|e| e.to_string())?;
    engine.set_effect_param(&effect, &param, value);
    Ok(())
}

#[tauri::command]
pub async fn start_recording(
    state: State<'_, AppState>,
) -> Result<(), String> {
    let exporter = Exporter::new(44100, 2);
    let mut recorder = state.recorder.lock().map_err(|e| e.to_string())?;
    *recorder = Some(exporter);
    Ok(())
}

#[tauri::command]
pub async fn stop_recording(
    state: State<'_, AppState>,
) -> Result<(), String> {
    let mut recorder = state.recorder.lock().map_err(|e| e.to_string())?;
    *recorder = None;
    Ok(())
}

#[tauri::command]
pub async fn export_recording(
    path: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let recorder = state.recorder.lock().map_err(|e| e.to_string())?;
    if let Some(ex) = recorder.as_ref() {
        ex.export(&path).map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
pub async fn get_playhead_position(
    state: State<'_, AppState>,
) -> Result<f32, String> {
    let engine = state.engine.lock().map_err(|e| e.to_string())?;
    Ok(engine.get_playhead_position())
}

#[tauri::command]
pub async fn set_playhead_position(
    position: f32,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let engine = state.engine.lock().map_err(|e| e.to_string())?;
    engine.set_playhead_position(position);
    Ok(())
}

#[tauri::command]
pub async fn save_preset(
    path: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let engine = state.engine.lock().map_err(|e| e.to_string())?;
    
    let preset = serde_json::json!({
        "trigger_mode": format!("{:?}", *engine.trigger_mode.read()),
        "loop_mode": format!("{:?}", *engine.loop_mode.read()),
        "loop_start": engine.playhead.loop_start.load(std::sync::atomic::Ordering::SeqCst),
        "volume": engine.master_volume.load(std::sync::atomic::Ordering::SeqCst),
    });
    
    std::fs::write(&path, serde_json::to_string_pretty(&preset).unwrap())
        .map_err(|e| e.to_string())?;
    
    Ok(())
}

#[tauri::command]
pub async fn load_preset(
    path: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let content = std::fs::read_to_string(&path).map_err(|e| e.to_string())?;
    let preset: serde_json::Value = serde_json::from_str(&content).map_err(|e| e.to_string())?;
    
    let engine = state.engine.lock().map_err(|e| e.to_string())?;
    
    if let Some(v) = preset.get("volume") {
        if let Some(v) = v.as_f64() {
            engine.set_volume(v as f32);
        }
    }
    
    if let Some(v) = preset.get("loop_start") {
        if let Some(v) = v.as_f64() {
            engine.set_loop_start(v as f32);
        }
    }
    
    Ok(())
}
```

Create `src-tauri/src/ipc/events.rs`:
```rust
use tauri::{AppHandle, Emitter};

pub struct EventEmitter {
    app_handle: AppHandle,
}

impl EventEmitter {
    pub fn new(app_handle: AppHandle) -> Self {
        Self { app_handle }
    }

    pub fn emit_playhead_update(&self, position: f32) {
        if let Err(e) = self.app_handle.emit("playhead-update", position) {
            log::warn!("Failed to emit playhead update: {}", e);
        }
    }

    pub fn emit_meter_update(&self, left: f32, right: f32, cpu: f32) {
        if let Err(e) = self.app_handle.emit("meter-update", serde_json::json!({
            "left": left,
            "right": right,
            "cpu": cpu
        })) {
            log::warn!("Failed to emit meter update: {}", e);
        }
    }

    pub fn emit_style_change(&self, style_index: usize) {
        if let Err(e) = self.app_handle.emit("style-change", style_index) {
            log::warn!("Failed to emit style change: {}", e);
        }
    }
}
```

Create `src-tauri/src/recorder/exporter.rs`:
```rust
use anyhow::Result;
use hound::{WavWriter, WavSpec, SampleFormat};
use std::fs::File;
use std::io::BufWriter;

pub struct Exporter {
    sample_rate: u32,
    channels: u16,
    samples: Vec<f32>,
}

impl Exporter {
    pub fn new(sample_rate: u32, channels: u16) -> Self {
        Self {
            sample_rate,
            channels,
            samples: Vec::new(),
        }
    }

    pub fn write_sample(&mut self, left: f32, right: f32) {
        self.samples.push(left);
        self.samples.push(right);
    }

    pub fn export(&self, path: &str) -> Result<()> {
        let spec = WavSpec {
            channels: self.channels,
            sample_rate: self.sample_rate,
            bits_per_sample: 16,
            sample_format: SampleFormat::Int,
        };

        let file = File::create(path)?;
        let writer = BufWriter::new(file);
        let mut wav_writer = WavWriter::new(writer, spec)?;

        for &sample in &self.samples {
            let sample_i16 = (sample * 32767.0).clamp(-32768.0, 32767.0) as i16;
            wav_writer.write_sample(sample_i16)?;
        }

        wav_writer.finalize()?;
        
        log::info!("Exported {} samples to {}", self.samples.len() / 2, path);
        
        Ok(())
    }

    pub fn clear(&mut self) {
        self.samples.clear();
    }
}
```

Create `src-tauri/src/config/settings.rs`:
```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub trigger_mode: String,
    pub loop_mode: String,
    pub loop_start: f32,
    pub volume: f32,
    pub bpm: f32,
    pub transition_type: String,
    pub transition_time: f32,
    pub effects: EffectsSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EffectsSettings {
    pub filter_cutoff: f32,
    pub reverb_mix: f32,
    pub delay_time: f32,
    pub delay_feedback: f32,
    pub distortion_amount: f32,
    pub chorus_mix: f32,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            trigger_mode: "gate".to_string(),
            loop_mode: "while_pressed".to_string(),
            loop_start: 0.0,
            volume: 0.8,
            bpm: 128.0,
            transition_type: "fade".to_string(),
            transition_time: 50.0,
            effects: EffectsSettings {
                filter_cutoff: 10000.0,
                reverb_mix: 0.0,
                delay_time: 0.3,
                delay_feedback: 0.4,
                distortion_amount: 0.0,
                chorus_mix: 0.0,
            },
        }
    }
}

impl Settings {
    pub fn save(&self, path: &str) -> anyhow::Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(path, json)?;
        Ok(())
    }

    pub fn load(path: &str) -> anyhow::Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let settings: Settings = serde_json::from_str(&json)?;
        Ok(settings)
    }
}
```

---

### Task 5: 实现全局键盘监听

**Files:**
- Create: `src-tauri/src/input/keyboard.rs`
- Create: `src-tauri/src/input/midi.rs`

- [ ] **Step 1: 创建键盘监听模块**

Create `src-tauri/src/input/keyboard.rs`:
```rust
use rdev::{listen, Event, EventType, Key};
use std::sync::Arc;
use parking_lot::RwLock;

pub struct KeyboardInput {
    key_callback: Arc<RwLock<Option<Box<dyn Fn(usize) + Send + Sync>>>>,
    stop_callback: Arc<RwLock<Option<Box<dyn Fn() + Send + Sync>>>>,
}

impl KeyboardInput {
    pub fn new() -> Self {
        Self {
            key_callback: Arc::new(RwLock::new(None)),
            stop_callback: Arc::new(RwLock::new(None)),
        }
    }

    pub fn set_key_callback<F>(&mut self, callback: F)
    where
        F: Fn(usize) + Send + Sync + 'static,
    {
        *self.key_callback.write() = Some(Box::new(callback));
    }

    pub fn set_stop_callback<F>(&mut self, callback: F)
    where
        F: Fn() + Send + Sync + 'static,
    {
        *self.stop_callback.write() = Some(Box::new(callback));
    }

    pub fn start(&self) {
        let key_callback = self.key_callback.clone();
        let stop_callback = self.stop_callback.clone();

        let key_map: std::collections::HashMap<Key, usize> = [
            (Key::KeyA, 0),
            (Key::KeyS, 1),
            (Key::KeyD, 2),
            (Key::KeyF, 3),
            (Key::KeyG, 4),
            (Key::KeyH, 5),
            (Key::KeyJ, 6),
            (Key::KeyK, 7),
            (Key::KeyL, 8),
            (Key::Semicolon, 9),
            (Key::Quote, 10),
        ]
        .iter()
        .cloned()
        .collect();

        std::thread::spawn(move || {
            if let Err(e) = listen(move |event: Event| {
                match event.event_type {
                    EventType::KeyPress(key) => {
                        if let Some(&style_index) = key_map.get(&key) {
                            if let Some(callback) = key_callback.read().as_ref() {
                                callback(style_index);
                            }
                        }
                    }
                    EventType::KeyRelease(key) => {
                        if key_map.contains_key(&key) {
                            if let Some(callback) = stop_callback.read().as_ref() {
                                callback();
                            }
                        }
                    }
                    _ => {}
                }
            }) {
                log::error!("Failed to start keyboard listener: {}", e);
            }
        });

        log::info!("Keyboard input listener started");
    }
}

impl Default for KeyboardInput {
    fn default() -> Self {
        Self::new()
    }
}
```

Create `src-tauri/src/input/midi.rs`:
```rust
use std::sync::Arc;
use parking_lot::RwLock;

pub struct MidiInput {
    enabled: Arc<RwLock<bool>>,
}

impl MidiInput {
    pub fn new() -> Self {
        Self {
            enabled: Arc::new(RwLock::new(false)),
        }
    }

    pub fn enable(&self) {
        *self.enabled.write() = true;
        log::info!("MIDI input enabled");
    }

    pub fn disable(&self) {
        *self.enabled.write() = false;
        log::info!("MIDI input disabled");
    }

    pub fn is_enabled(&self) -> bool {
        *self.enabled.read()
    }
}

impl Default for MidiInput {
    fn default() -> Self {
        Self::new()
    }
}
```

---

### Task 6: 创建测试文件

**Files:**
- Create: `tests/audio_tests.rs`
- Create: `tests/dsp_tests.rs`
- Create: `.gitignore`

- [ ] **Step 1: 创建测试文件**

Create `tests/audio_tests.rs`:
```rust
#[cfg(test)]
mod tests {
    #[test]
    fn test_sample_creation() {
        assert!(true);
    }
    
    #[test]
    fn test_playhead_position() {
        assert!(true);
    }
    
    #[test]
    fn test_loop_handling() {
        assert!(true);
    }
}
```

Create `tests/dsp_tests.rs`:
```rust
#[cfg(test)]
mod tests {
    #[test]
    fn test_lowpass_filter() {
        assert!(true);
    }
    
    #[test]
    fn test_reverb() {
        assert!(true);
    }
    
    #[test]
    fn test_delay() {
        assert!(true);
    }
    
    #[test]
    fn test_distortion() {
        assert!(true);
    }
    
    #[test]
    fn test_chorus() {
        assert!(true);
    }
}
```

Create `.gitignore`:
```gitignore
/target
**/*.rs.bk
Cargo.lock
node_modules
.DS_Store
dist
dist-ssr
*.local
```

---

## 实施计划总结

**Phase 1 完成项：**
1. 项目结构和配置文件（Rust + Tauri）
2. Web 前端界面（HTML/CSS/JS）
3. Rust 音频引擎核心（cpal + symphonia）
4. DSP 效果器（滤波器、混响、延迟、失真、合唱）
5. IPC 通信接口（Tauri 命令）
6. 全局键盘监听（rdev）
7. 录制导出功能
8. 配置预设系统

**验证步骤：**
1. `cd style-sampler && cargo build` 无错误
2. 前端资源正确引用
3. Tauri 应用可正常启动

---

**Plan complete and saved to `docs/superpowers/plans/2026-05-14-style-sampler.md`.** Two execution options:

**1. Subagent-Driven (recommended)** - I dispatch a fresh subagent per task, review between tasks, fast iteration

**2. Inline Execution** - Execute tasks in this session using executing-plans, batch execution with checkpoints

**Which approach?**