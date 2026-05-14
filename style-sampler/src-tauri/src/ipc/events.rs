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
