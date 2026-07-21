// Event — base trait for all events
// Inspired by PMMP's Event.php

use std::any::Any;

pub trait Event: Any + Send + Sync {
    fn event_name(&self) -> &str;
    fn as_any(&self) -> &dyn Any;
}
