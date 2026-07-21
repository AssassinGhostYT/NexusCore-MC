// PlayerJoinEvent — fired when a player joins the server

use super::event_trait::Event;
use std::any::Any;

pub struct PlayerJoinEvent {
    pub player_name: String,
    pub player_uuid: String,
}

impl Event for PlayerJoinEvent {
    fn event_name(&self) -> &str {
        "PlayerJoinEvent"
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}
