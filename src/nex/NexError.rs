// NexError — error types for .nex archive operations

use std::fmt;

#[derive(Debug)]
pub enum NexError {
    InvalidMagic,
    CorruptedArchive(String),
    IoError(std::io::Error),
    CompressionError(String),
    EntryTooLarge { name: String, size: u32 },
    TooManyEntries { count: u32 },
    MissingManifest,
    InvalidManifest(String),
}

impl fmt::Display for NexError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidMagic => write!(f, "Not a valid .nex file (bad magic header)"),
            Self::CorruptedArchive(msg) => write!(f, "Corrupted .nex archive: {}", msg),
            Self::IoError(e) => write!(f, "IO error: {}", e),
            Self::CompressionError(e) => write!(f, "Compression error: {}", e),
            Self::EntryTooLarge { name, size } => {
                write!(f, "Entry '{}' too large: {} bytes", name, size)
            }
            Self::TooManyEntries { count } => {
                write!(f, "Too many entries: {}", count)
            }
            Self::MissingManifest => write!(f, "Missing plugin.toml in .nex archive"),
            Self::InvalidManifest(msg) => write!(f, "Invalid plugin.toml: {}", msg),
        }
    }
}

impl From<std::io::Error> for NexError {
    fn from(e: std::io::Error) -> Self {
        Self::IoError(e)
    }
}
