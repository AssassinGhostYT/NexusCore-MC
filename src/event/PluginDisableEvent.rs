// PluginDisableEvent — fired when a plugin is disabled

use super::event_trait::Event;
use std::any::Any;

pub struct PluginDisableEvent {
    pub plugin_name: String,
}

impl Event for PluginDisableEvent {
    fn event_name(&self) -> &str {
        "PluginDisableEvent"
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}
