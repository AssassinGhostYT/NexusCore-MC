// PlayerChatEvent — fired when a player sends a chat message

use super::event_trait::Event;
use super::cancellable::Cancellable;
use std::any::Any;

pub struct PlayerChatEvent {
    pub player_name: String,
    pub message: String,
    pub cancelled: bool,
}

impl Event for PlayerChatEvent {
    fn event_name(&self) -> &str {
        "PlayerChatEvent"
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Cancellable for PlayerChatEvent {
    fn is_cancelled(&self) -> bool {
        self.cancelled
    }
    fn set_cancelled(&mut self, cancelled: bool) {
        self.cancelled = cancelled;
    }
}
