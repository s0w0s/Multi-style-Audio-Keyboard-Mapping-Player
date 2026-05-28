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
