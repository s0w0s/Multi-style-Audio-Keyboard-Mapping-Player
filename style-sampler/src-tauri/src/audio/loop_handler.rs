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

    pub fn should_loop(&self, _position: f32, at_end: bool) -> bool {
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
