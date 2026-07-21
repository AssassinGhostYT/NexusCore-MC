// NexusCore Event System — inspired by PMMP
// Pub-sub with priorities, event inheritance, and Cancellable support
// Each event is in its own file

#[path = "EventTrait.rs"]
pub mod event_trait;
pub use event_trait::Event;

#[path = "Cancellable.rs"]
pub mod cancellable;
pub use cancellable::Cancellable;

#[path = "EventPriority.rs"]
pub mod event_priority;
pub use event_priority::EventPriority;

#[path = "RegisteredListener.rs"]
pub mod registered_listener;
pub use registered_listener::RegisteredListener;

#[path = "HandlerList.rs"]
pub mod handler_list;
pub use handler_list::HandlerList;

#[path = "HandlerListManager.rs"]
pub mod handler_list_manager;
pub use handler_list_manager::HandlerListManager;

#[path = "PlayerJoinEvent.rs"]
pub mod player_join_event;
pub use player_join_event::PlayerJoinEvent;

#[path = "PlayerQuitEvent.rs"]
pub mod player_quit_event;
pub use player_quit_event::PlayerQuitEvent;

#[path = "PlayerMoveEvent.rs"]
pub mod player_move_event;
pub use player_move_event::PlayerMoveEvent;

#[path = "PlayerChatEvent.rs"]
pub mod player_chat_event;
pub use player_chat_event::PlayerChatEvent;

#[path = "BlockBreakEvent.rs"]
pub mod block_break_event;
pub use block_break_event::BlockBreakEvent;

#[path = "BlockPlaceEvent.rs"]
pub mod block_place_event;
pub use block_place_event::BlockPlaceEvent;

#[path = "PluginEnableEvent.rs"]
pub mod plugin_enable_event;
pub use plugin_enable_event::PluginEnableEvent;

#[path = "PluginDisableEvent.rs"]
pub mod plugin_disable_event;
pub use plugin_disable_event::PluginDisableEvent;
