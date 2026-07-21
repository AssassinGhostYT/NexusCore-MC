// NexusCore Plugin System
// Rust is the engine only. Plugins are written exclusively in Python.
// This module loads and manages Python plugins via PyO3.

#[path = "PluginManager.rs"]
pub mod plugin_manager;
pub use plugin_manager::PluginManager;

#[path = "PluginDescription.rs"]
pub mod plugin_description;
pub use plugin_description::PluginDescription;

#[path = "PluginLoader.rs"]
pub mod plugin_loader;
pub use plugin_loader::PluginLoader;
