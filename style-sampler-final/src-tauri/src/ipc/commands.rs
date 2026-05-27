use crate::audio::{AudioEngine, TriggerMode, LoopMode};
use crate::recorder::exporter::Exporter;
use std::sync::Mutex;
use tauri::State;

pub struct AppState {
    pub engine: Mutex<AudioEngine>,
    pub recorder: Mutex<Option<Exporter>>,
}

#[tauri::command]
pub fn pick_folder() -> Result<Option<String>, String> {
    let folder = rfd::FileDialog::new()
        .set_title("选择音频采样文件夹")
        .pick_folder();
    match folder {
        Some(f) => Ok(Some(f.to_string_lossy().to_string())),
        None => Ok(None),
    }
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
        "while_pressed" | "hold" => LoopMode::WhilePressed,
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
        "loop_start": *engine.playhead.loop_start.read(),
        "volume": *engine.master_volume.read(),
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
