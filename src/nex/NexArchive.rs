// NexArchive — reader for .nex plugin archives
// Reads the binary format: magic + entries + compressed data

use std::collections::HashMap;
use std::io::{Read};
use flate2::read::ZlibDecoder;
use flate2::Compression;
use flate2::write::ZlibEncoder;
use std::io::Write;
use super::nex_error::NexError;
use super::nex_format::*;

/// A single file entry inside a .nex archive
#[derive(Debug, Clone)]
pub struct NexEntry {
    pub filename: String,
    pub data: Vec<u8>,
}

/// A parsed .nex archive
pub struct NexArchive {
    entries: HashMap<String, NexEntry>,
}

impl NexArchive {
    /// Create an empty archive
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }

    /// Read a .nex archive from bytes
    pub fn read(data: &[u8]) -> Result<Self, NexError> {
        if data.len() < 8 {
            return Err(NexError::CorruptedArchive("File too small".into()));
        }

        // Verify magic
        if &data[0..4] != NEX_MAGIC {
            return Err(NexError::InvalidMagic);
        }

        let entry_count = u32::from_le_bytes([data[4], data[5], data[6], data[7]]);
        if entry_count > MAX_ENTRIES {
            return Err(NexError::TooManyEntries { count: entry_count });
        }

        let mut cursor = 8;
        let mut entries = HashMap::new();

        for _ in 0..entry_count {
            if cursor + 2 > data.len() {
                return Err(NexError::CorruptedArchive("Unexpected end of file".into()));
            }

            // Read filename length
            let name_len = u16::from_le_bytes([data[cursor], data[cursor + 1]]) as usize;
            cursor += 2;

            if cursor + name_len > data.len() {
                return Err(NexError::CorruptedArchive("Truncated filename".into()));
            }

            // Read filename
            let filename = String::from_utf8(data[cursor..cursor + name_len].to_vec())
                .map_err(|_| NexError::CorruptedArchive("Invalid UTF-8 in filename".into()))?;
            cursor += name_len;

            if cursor + 8 > data.len() {
                return Err(NexError::CorruptedArchive("Truncated entry header".into()));
            }

            // Read sizes and checksum
            let compressed_size = u32::from_le_bytes([
                data[cursor], data[cursor + 1], data[cursor + 2], data[cursor + 3],
            ]);
            let _uncompressed_size = u32::from_le_bytes([
                data[cursor + 4], data[cursor + 5], data[cursor + 6], data[cursor + 7],
            ]);
            let _checksum = u32::from_le_bytes([
                data[cursor + 8], data[cursor + 9], data[cursor + 10], data[cursor + 11],
            ]);
            cursor += 12;

            if compressed_size > MAX_ENTRY_SIZE {
                return Err(NexError::EntryTooLarge {
                    name: filename.clone(),
                    size: compressed_size,
                });
            }

            if cursor + compressed_size as usize > data.len() {
                return Err(NexError::CorruptedArchive("Truncated compressed data".into()));
            }

            // Decompress
            let compressed_data = &data[cursor..cursor + compressed_size as usize];
            let decompressed = decompress_zlib(compressed_data)
                .map_err(|e| NexError::CompressionError(format!("{}: {}", filename, e)))?;

            cursor += compressed_size as usize;

            entries.insert(filename.clone(), NexEntry {
                filename,
                data: decompressed,
            });
        }

        Ok(Self { entries })
    }

    /// Pack a directory into .nex binary format
    pub fn pack_files(files: Vec<(String, Vec<u8>)>) -> Result<Vec<u8>, NexError> {
        if files.len() as u32 > MAX_ENTRIES {
            return Err(NexError::TooManyEntries { count: files.len() as u32 });
        }

        let mut output = Vec::new();

        // Write magic
        output.extend_from_slice(NEX_MAGIC);

        // Write entry count
        output.extend_from_slice(&(files.len() as u32).to_le_bytes());

        for (filename, data) in &files {
            let name_bytes = filename.as_bytes();
            if name_bytes.len() > u16::MAX as usize {
                return Err(NexError::CorruptedArchive(format!(
                    "Filename too long: {}", filename
                )));
            }

            // Compress data
            let compressed = compress_zlib(data)
                .map_err(|e| NexError::CompressionError(format!("{}: {}", filename, e)))?;

            // Calculate CRC32 checksum
            let checksum = crc32fast::hash(data);

            // Write filename
            output.extend_from_slice(&(name_bytes.len() as u16).to_le_bytes());
            output.extend_from_slice(name_bytes);

            // Write sizes and checksum
            output.extend_from_slice(&(compressed.len() as u32).to_le_bytes());
            output.extend_from_slice(&(data.len() as u32).to_le_bytes());
            output.extend_from_slice(&checksum.to_le_bytes());

            // Write compressed data
            output.extend_from_slice(&compressed);
        }

        Ok(output)
    }

    /// Get a file from the archive
    pub fn get(&self, path: &str) -> Option<&[u8]> {
        self.entries.get(path).map(|e| e.data.as_slice())
    }

    /// Check if a file exists
    pub fn has(&self, path: &str) -> bool {
        self.entries.contains_key(path)
    }

    /// List all files in the archive
    pub fn list_files(&self) -> Vec<&str> {
        self.entries.keys().map(|s| s.as_str()).collect()
    }

    /// Get all entries
    pub fn entries(&self) -> &HashMap<String, NexEntry> {
        &self.entries
    }

    /// Validate that the archive has a valid plugin.toml
    pub fn validate(&self) -> Result<String, NexError> {
        let toml_content = self.get("plugin.toml")
            .ok_or(NexError::MissingManifest)?;

        let toml_str = std::str::from_utf8(toml_content)
            .map_err(|_| NexError::InvalidManifest("Not valid UTF-8".into()))?;

        // Validate TOML parses correctly
        crate::plugin::PluginDescription::from_toml(toml_str)
            .map_err(|e| NexError::InvalidManifest(e))?;

        Ok(toml_str.to_string())
    }
}

fn compress_zlib(data: &[u8]) -> Result<Vec<u8>, String> {
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::fast());
    encoder.write_all(data).map_err(|e| e.to_string())?;
    encoder.finish().map_err(|e| e.to_string())
}

fn decompress_zlib(data: &[u8]) -> Result<Vec<u8>, String> {
    let mut decoder = ZlibDecoder::new(data);
    let mut decompressed = Vec::new();
    decoder.read_to_end(&mut decompressed).map_err(|e| e.to_string())?;
    Ok(decompressed)
}
