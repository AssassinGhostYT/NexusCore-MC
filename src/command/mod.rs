// NexusCore Command System — inspired by PMMP
// Each file is a separate component

#[path = "CommandSender.rs"]
pub mod command_sender;
pub use command_sender::CommandSender;

#[path = "ConsoleCommandSender.rs"]
pub mod console_command_sender;
pub use console_command_sender::ConsoleCommandSender;

#[path = "CommandBase.rs"]
pub mod command_base;
pub use command_base::{Command, PluginCommand};

#[path = "CommandMap.rs"]
pub mod command_map;
pub use command_map::CommandMap;
