// HandlerListManager — singleton managing HandlerLists per event type

use std::collections::HashMap;
use std::sync::RwLock;
use super::handler_list::HandlerList;
use super::registered_listener::RegisteredListener;

pub struct HandlerListManager {
    lists: RwLock<HashMap<String, HandlerList>>,
}

impl HandlerListManager {
    pub fn new() -> Self {
        Self {
            lists: RwLock::new(HashMap::new()),
        }
    }

    /// Register a listener in the appropriate HandlerList
    pub fn register_listener(&self, event_name: &str, listener: RegisteredListener) {
        let mut lists = self.lists.write().unwrap();
        let list = lists
            .entry(event_name.to_string())
            .or_insert_with(HandlerList::new);
        list.register(listener);
    }

    /// Clear all HandlerLists
    pub fn clear(&self) {
        let mut lists = self.lists.write().unwrap();
        lists.clear();
    }
}

impl Default for HandlerListManager {
    fn default() -> Self {
        Self::new()
    }
}
