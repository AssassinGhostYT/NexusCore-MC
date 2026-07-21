// PlayerMoveEvent — fired when a player moves

use super::event_trait::Event;
use super::cancellable::Cancellable;
use std::any::Any;

pub struct PlayerMoveEvent {
    pub player_name: String,
    pub from: (f32, f32, f32),
    pub to: (f32, f32, f32),
    pub cancelled: bool,
}

impl Event for PlayerMoveEvent {
    fn event_name(&self) -> &str {
        "PlayerMoveEvent"
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Cancellable for PlayerMoveEvent {
    fn is_cancelled(&self) -> bool {
        self.cancelled
    }
    fn set_cancelled(&mut self, cancelled: bool) {
        self.cancelled = cancelled;
    }
}
