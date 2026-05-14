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
