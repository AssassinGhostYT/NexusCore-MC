// BlockPlaceEvent — fired when a player places a block

use super::event_trait::Event;
use super::cancellable::Cancellable;
use std::any::Any;

pub struct BlockPlaceEvent {
    pub player_name: String,
    pub position: (i32, i32, i32),
    pub block_id: u32,
    pub cancelled: bool,
}

impl Event for BlockPlaceEvent {
    fn event_name(&self) -> &str {
        "BlockPlaceEvent"
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Cancellable for BlockPlaceEvent {
    fn is_cancelled(&self) -> bool {
        self.cancelled
    }
    fn set_cancelled(&mut self, cancelled: bool) {
        self.cancelled = cancelled;
    }
}
