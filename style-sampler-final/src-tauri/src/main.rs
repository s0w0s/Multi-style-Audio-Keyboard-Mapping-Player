#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

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

fn main() {
    env_logger::init();
    
    let mut engine = AudioEngine::new();
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
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.set_title("多风格音频键盘映射播放器");
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
