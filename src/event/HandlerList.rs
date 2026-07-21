// HandlerList — sorted list of RegisteredListeners for one event type

use super::registered_listener::RegisteredListener;

pub struct HandlerList {
    slots: [Vec<RegisteredListener>; 6],
}

impl HandlerList {
    pub fn new() -> Self {
        Self {
            slots: Default::default(),
        }
    }

    pub fn register(&mut self, listener: RegisteredListener) {
        let idx = listener.priority().value() as usize;
        if idx < 6 {
            self.slots[idx].push(listener);
        }
    }

    /// Get listeners in execution order (Lowest first, Monitor last)
    pub fn get_listeners(&self) -> Vec<&RegisteredListener> {
        let mut result = Vec::new();
        for idx in (0..6).rev() {
            for listener in &self.slots[idx] {
                result.push(listener);
            }
        }
        result
    }

    pub fn count(&self) -> usize {
        self.slots.iter().map(|s| s.len()).sum()
    }

    pub fn clear(&mut self) {
        for slot in &mut self.slots {
            slot.clear();
        }
    }
}

impl Default for HandlerList {
    fn default() -> Self {
        Self::new()
    }
}
