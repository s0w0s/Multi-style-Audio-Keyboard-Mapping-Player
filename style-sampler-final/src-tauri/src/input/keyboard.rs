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
            (Key::SemiColon, 9),
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
                log::error!("Failed to start keyboard listener: {:?}", e);
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
