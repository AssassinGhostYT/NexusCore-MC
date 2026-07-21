// PlayerQuitEvent — fired when a player disconnects

use super::event_trait::Event;
use std::any::Any;

pub struct PlayerQuitEvent {
    pub player_name: String,
}

impl Event for PlayerQuitEvent {
    fn event_name(&self) -> &str {
        "PlayerQuitEvent"
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}
