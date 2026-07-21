// NexPacker — packs a plugin directory into a .nex archive
// Similar to PMMP's DevTools but built into the server

use std::path::Path;
use super::nex_archive::NexArchive;
use super::nex_error::NexError;

pub struct NexPacker;

impl NexPacker {
    /// Pack a plugin directory into .nex format
    /// The directory must contain a plugin.toml manifest
    pub fn pack(plugin_dir: &Path) -> Result<Vec<u8>, NexError> {
        // Validate manifest exists
        let manifest_path = plugin_dir.join("plugin.toml");
        if !manifest_path.exists() {
            return Err(NexError::MissingManifest);
        }

        // Read all files recursively
        let mut files = Vec::new();
        Self::collect_files(plugin_dir, plugin_dir, &mut files)?;

        // Validate the manifest first
        let archive = NexArchive::pack_files(files.clone())?;
        let nex = NexArchive::read(&archive)?;
        nex.validate()?;

        log::info!("[NexPacker] Packed {} files", files.len());
        Ok(archive)
    }

    /// Pack a plugin directory and write to a .nex file
    pub fn pack_to_file(plugin_dir: &Path, output_path: &Path) -> Result<(), NexError> {
        let data = Self::pack(plugin_dir)?;
        std::fs::write(output_path, &data)
            .map_err(|e| NexError::IoError(e))?;
        log::info!("[NexPacker] Written to {}", output_path.display());
        Ok(())
    }

    /// Collect all files in a directory recursively (excluding hidden files and build artifacts)
    fn collect_files(base: &Path, current: &Path, files: &mut Vec<(String, Vec<u8>)>) -> Result<(), NexError> {
        for entry in std::fs::read_dir(current).map_err(NexError::IoError)? {
            let entry = entry.map_err(NexError::IoError)?;
            let path = entry.path();
            let name = path.file_name().unwrap().to_string_lossy().to_string();

            // Skip hidden files, __pycache__, .pyc, .nex
            if name.starts_with('.') || name == "__pycache__" || name.ends_with(".pyc") || name.ends_with(".nex") {
                continue;
            }

            if path.is_dir() {
                Self::collect_files(base, &path, files)?;
            } else {
                let data = std::fs::read(&path).map_err(NexError::IoError)?;
                if data.len() as u32 > super::MAX_ENTRY_SIZE {
                    return Err(NexError::EntryTooLarge {
                        name: name.clone(),
                        size: data.len() as u32,
                    });
                }

                // Get relative path from base directory
                let rel_path = path.strip_prefix(base).unwrap_or(&path);
                let rel_str = rel_path.to_string_lossy().to_string();

                files.push((rel_str, data));
            }
        }

        Ok(())
    }
}
