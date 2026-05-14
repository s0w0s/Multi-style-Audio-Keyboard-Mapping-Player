use std::sync::atomic::{AtomicBool, AtomicF32, Ordering};
use std::sync::Arc;

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
            total_duration: Arc::new(AtomicF32::new(8.0)),
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
