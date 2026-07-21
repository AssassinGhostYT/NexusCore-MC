// PluginLoader — loads Python plugins from directories OR .nex archives
// Rejects Rust plugins with a clear error

use std::path::Path;
use super::plugin_description::PluginDescription;
use crate::nex::NexArchive;

pub struct PluginLoader;

impl PluginLoader {
    /// Load a plugin from a directory
    pub fn load_from_dir(plugin_dir: &Path) -> Result<PluginDescription, String> {
        let manifest = plugin_dir.join("plugin.toml");
        if !manifest.exists() {
            return Err(format!("No plugin.toml found in {}", plugin_dir.display()));
        }

        let content = std::fs::read_to_string(&manifest)
            .map_err(|e| format!("Error reading plugin.toml: {}", e))?;
        let desc = PluginDescription::from_toml(&content)?;

        // Main class must be a .py file
        let main_path = plugin_dir.join(desc.main());
        if !main_path.exists() {
            return Err(format!(
                "Plugin '{}': main file '{}' not found. Plugins must be Python (.py only).",
                desc.name(), desc.main()
            ));
        }

        if !main_path.to_string_lossy().ends_with(".py") {
            return Err(format!(
                "Plugin '{}': main file must be .py (Python only). Got: '{}'",
                desc.name(), desc.main()
            ));
        }

        // Reject Rust plugins
        let src_dir = plugin_dir.join("src");
        if src_dir.exists() {
            if let Ok(entries) = std::fs::read_dir(&src_dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.extension().and_then(|e| e.to_str()) == Some("rs") {
                        return Err(format!(
                            "Plugin '{}': .rs files are not allowed. Python only.",
                            desc.name()
                        ));
                    }
                }
            }
        }

        log::info!("[PluginLoader] Loaded directory plugin: {} v{}", desc.name(), desc.version());
        Ok(desc)
    }

    /// Load a plugin from a .nex archive file
    pub fn load_from_nex(nex_path: &Path) -> Result<PluginDescription, String> {
        let data = std::fs::read(nex_path)
            .map_err(|e| format!("Error reading .nex file: {}", e))?;

        let archive = NexArchive::read(&data)
            .map_err(|e| format!("Invalid .nex file '{}': {}", nex_path.display(), e))?;

        // Validate it has a proper plugin.toml
        let toml_str = archive.validate()
            .map_err(|e| format!("Plugin validation failed: {}", e))?;

        let desc = PluginDescription::from_toml(&toml_str)?;

        // Verify main file is Python
        if !desc.main().ends_with(".py") {
            return Err(format!(
                "Plugin '{}': main file must be .py (Python only). Got: '{}'",
                desc.name(), desc.main()
            ));
        }

        // Check no .rs files inside
        for filename in archive.list_files() {
            if filename.ends_with(".rs") {
                return Err(format!(
                    "Plugin '{}': .rs files not allowed in .nex archive.",
                    desc.name()
                ));
            }
        }

        log::info!("[PluginLoader] Loaded .nex plugin: {} v{}", desc.name(), desc.version());
        Ok(desc)
    }

    /// Scan a plugins directory — handles both directories and .nex files
    pub fn scan_directory(plugins_dir: &Path) -> Vec<Result<PluginDescription, String>> {
        let mut results = Vec::new();

        if !plugins_dir.exists() {
            return results;
        }

        if let Ok(entries) = std::fs::read_dir(plugins_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                let name = path.file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("unknown")
                    .to_string();

                let result = if path.is_dir() {
                    // Directory plugin (development mode)
                    Self::load_from_dir(&path)
                } else if path.extension().and_then(|e| e.to_str()) == Some("nex") {
                    // .nex archive plugin (production)
                    Self::load_from_nex(&path)
                } else {
                    // Skip unknown files (old .phar, .zip, etc.)
                    continue;
                };

                match result {
                    Ok(desc) => results.push(Ok(desc)),
                    Err(e) => {
                        log::warn!("[PluginLoader] Skipping '{}': {}", name, e);
                        results.push(Err(e));
                    }
                }
            }
        }

        results
    }
}
