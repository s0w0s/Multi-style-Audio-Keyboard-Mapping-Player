use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use parking_lot::RwLock;

pub struct Playhead {
    pub position: Arc<RwLock<f32>>,
    pub state: Arc<AtomicBool>,
    pub speed: Arc<RwLock<f32>>,
    pub total_duration: Arc<RwLock<f32>>,
    pub loop_enabled: Arc<AtomicBool>,
    pub loop_start: Arc<RwLock<f32>>,
}

impl Playhead {
    pub fn new() -> Self {
        Self {
            position: Arc::new(RwLock::new(0.0)),
            state: Arc::new(AtomicBool::new(false)),
            speed: Arc::new(RwLock::new(1.0)),
            total_duration: Arc::new(RwLock::new(8.0)),
            loop_enabled: Arc::new(AtomicBool::new(true)),
            loop_start: Arc::new(RwLock::new(0.0)),
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
        *self.position.write() = 0.0;
    }

    pub fn get_position(&self) -> f32 {
        *self.position.read()
    }

    pub fn set_position(&self, pos: f32) {
        let max = *self.total_duration.read();
        *self.position.write() = pos.max(0.0).min(max);
    }

    pub fn advance(&self, delta_seconds: f32) -> bool {
        if !self.state.load(Ordering::SeqCst) {
            return false;
        }

        let speed = *self.speed.read();
        let current = *self.position.read();
        let duration = *self.total_duration.read();
        let loop_start = *self.loop_start.read();
        let loop_enabled = self.loop_enabled.load(Ordering::SeqCst);

        let new_position = current + delta_seconds * speed;

        if new_position >= duration {
            if loop_enabled {
                let loop_length = duration - loop_start;
                if loop_length > 0.0 {
                    let overflow = new_position - duration;
                    let new_pos = loop_start + overflow % loop_length;
                    *self.position.write() = new_pos;
                    return true;
                }
            }
            *self.position.write() = duration;
            self.pause();
            return false;
        }

        *self.position.write() = new_position;
        true
    }

    pub fn is_playing(&self) -> bool {
        self.state.load(Ordering::SeqCst)
    }

    pub fn set_speed(&self, speed: f32) {
        *self.speed.write() = speed.clamp(0.5, 2.0);
    }

    pub fn set_total_duration(&self, duration: f32) {
        *self.total_duration.write() = duration;
    }

    pub fn set_loop_enabled(&self, enabled: bool) {
        self.loop_enabled.store(enabled, Ordering::SeqCst);
    }

    pub fn set_loop_start(&self, start: f32) {
        let duration = *self.total_duration.read();
        *self.loop_start.write() = start.clamp(0.0, duration);
    }
}

impl Default for Playhead {
    fn default() -> Self {
        Self::new()
    }
}
