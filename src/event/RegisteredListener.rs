// RegisteredListener — wraps a handler closure with priority and owning plugin

use super::event_priority::EventPriority;

pub struct RegisteredListener {
    /// The event name this listener handles
    event_name: String,
    /// Priority of execution
    priority: EventPriority,
    /// Name of the plugin that owns this listener
    plugin_name: String,
    /// If true, skip this handler when event is cancelled
    ignore_cancelled: bool,
}

impl RegisteredListener {
    pub fn new(
        event_name: String,
        priority: EventPriority,
        plugin_name: String,
        ignore_cancelled: bool,
    ) -> Self {
        Self { event_name, priority, plugin_name, ignore_cancelled }
    }

    pub fn event_name(&self) -> &str {
        &self.event_name
    }

    pub fn priority(&self) -> EventPriority {
        self.priority
    }

    pub fn plugin_name(&self) -> &str {
        &self.plugin_name
    }

    pub fn ignore_cancelled(&self) -> bool {
        self.ignore_cancelled
    }
}
