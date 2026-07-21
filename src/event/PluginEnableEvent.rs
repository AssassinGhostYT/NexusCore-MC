// PluginEnableEvent — fired when a plugin is enabled

use super::event_trait::Event;
use std::any::Any;

pub struct PluginEnableEvent {
    pub plugin_name: String,
}

impl Event for PluginEnableEvent {
    fn event_name(&self) -> &str {
        "PluginEnableEvent"
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}
