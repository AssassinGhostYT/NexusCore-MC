// PluginManager — manages Python plugin lifecycle
// Loads, enables, disables Python plugins via PyO3

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::RwLock;
use super::plugin_description::PluginDescription;

/// Represents a loaded Python plugin
pub struct PythonPlugin {
    pub description: PluginDescription,
    pub enabled: bool,
    pub data_folder: PathBuf,
}

impl PythonPlugin {
    pub fn name(&self) -> &str {
        self.description.name()
    }
    pub fn version(&self) -> &str {
        self.description.version()
    }
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}

pub struct PluginManager {
    plugins: RwLock<HashMap<String, PythonPlugin>>,
    plugins_dir: PathBuf,
    data_dir: PathBuf,
}

impl PluginManager {
    pub fn new(plugins_dir: PathBuf, data_dir: PathBuf) -> Self {
        Self {
            plugins: RwLock::new(HashMap::new()),
            plugins_dir,
            data_dir,
        }
    }

    /// Scan plugins directory and load all valid Python plugins
    pub fn load_plugins(&self) {
        use super::plugin_loader::PluginLoader;

        if !self.plugins_dir.exists() {
            log::warn!("Plugins directory does not exist: {}", self.plugins_dir.display());
            let _ = std::fs::create_dir_all(&self.plugins_dir);
            return;
        }

        let results = PluginLoader::scan_directory(&self.plugins_dir);
        let mut plugins = self.plugins.write().unwrap();

        for result in results {
            match result {
                Ok(desc) => {
                    let data_folder = self.data_dir.join(desc.name());
                    let _ = std::fs::create_dir_all(&data_folder);

                    log::info!("[PluginManager] Loading plugin: {} v{}", desc.name(), desc.version());

                    let plugin = PythonPlugin {
                        description: desc,
                        enabled: false,
                        data_folder,
                    };
                    plugins.insert(plugin.name().to_string(), plugin);
                }
                Err(e) => {
                    log::error!("[PluginManager] {}", e);
                }
            }
        }

        log::info!("[PluginManager] {} plugin(s) loaded", plugins.len());
    }

    /// Enable a plugin by name
    pub fn enable_plugin(&self, name: &str) -> bool {
        let mut plugins = self.plugins.write().unwrap();
        if let Some(plugin) = plugins.get_mut(name) {
            if !plugin.enabled {
                plugin.enabled = true;
                log::info!("[PluginManager] {} enabled", name);
                // TODO: call Python onEnable() via PyO3
                return true;
            }
        }
        false
    }

    /// Disable a plugin by name
    pub fn disable_plugin(&self, name: &str) -> bool {
        let mut plugins = self.plugins.write().unwrap();
        if let Some(plugin) = plugins.get_mut(name) {
            if plugin.enabled {
                plugin.enabled = false;
                log::info!("[PluginManager] {} disabled", name);
                // TODO: call Python onDisable() via PyO3
                return true;
            }
        }
        false
    }

    /// Disable all plugins
    pub fn disable_all(&self) {
        let mut plugins = self.plugins.write().unwrap();
        for (name, plugin) in plugins.iter_mut() {
            if plugin.enabled {
                plugin.enabled = false;
                log::info!("[PluginManager] {} disabled", name);
            }
        }
    }

    /// Get plugin by name
    pub fn get_plugin(&self, name: &str) -> Option<String> {
        let plugins = self.plugins.read().unwrap();
        plugins.get(name).map(|p| p.name().to_string())
    }

    /// Count of loaded plugins
    pub fn count(&self) -> usize {
        let plugins = self.plugins.read().unwrap();
        plugins.len()
    }

    /// Count of enabled plugins
    pub fn enabled_count(&self) -> usize {
        let plugins = self.plugins.read().unwrap();
        plugins.values().filter(|p| p.enabled).count()
    }
}
