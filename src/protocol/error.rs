// Centralized error types for all protocol packet operations.
// Every packet read/write goes through Result<T, PacketError> — no silent failures.
use std::fmt;

/// All possible errors that can occur during packet read/write operations.
#[derive(Debug)]
pub enum PacketError {
    /// Not enough bytes left in the buffer to read a field.
    Underflow { field: &'static str, need: usize, have: usize },
    /// Invalid UTF-8 string in packet data.
    InvalidUtf8 { field: &'static str, source: std::string::FromUtf8Error },
    /// JSON deserialization failed.
    Json { context: &'static str, source: serde_json::Error },
    /// Base64 decoding failed.
    Base64 { context: &'static str, source: base64::DecodeError },
    /// Deflate/zlib decompression failed.
    Decompress { source: std::io::Error },
    /// Deflate/zlib compression failed.
    Compress { source: std::io::Error },
    /// General I/O error during read/write.
    Io { context: &'static str, source: std::io::Error },
    /// Varint is malformed or too long.
    VarintOverflow { kind: &'static str },
    /// Packet data doesn't match expected format.
    Format { packet: &'static str, detail: String },
}

impl fmt::Display for PacketError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PacketError::Underflow { field, need, have } =>
                write!(f, "underflow reading '{}': need {} bytes, have {}", field, need, have),
            PacketError::InvalidUtf8 { field, source } =>
                write!(f, "invalid UTF-8 in '{}': {}", field, source),
            PacketError::Json { context, source } =>
                write!(f, "JSON error in {}: {}", context, source),
            PacketError::Base64 { context, source } =>
                write!(f, "base64 error in {}: {}", context, source),
            PacketError::Decompress { source } =>
                write!(f, "decompress failed: {}", source),
            PacketError::Compress { source } =>
                write!(f, "compress failed: {}", source),
            PacketError::Io { context, source } =>
                write!(f, "IO error in {}: {}", context, source),
            PacketError::VarintOverflow { kind } =>
                write!(f, "varint overflow: {}", kind),
            PacketError::Format { packet, detail } =>
                write!(f, "bad format in {}: {}", packet, detail),
        }
    }
}

impl std::error::Error for PacketError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            PacketError::InvalidUtf8 { source, .. } => Some(source),
            PacketError::Json { source, .. } => Some(source),
            PacketError::Base64 { source, .. } => Some(source),
            PacketError::Decompress { source } => Some(source),
            PacketError::Compress { source } => Some(source),
            PacketError::Io { source, .. } => Some(source),
            _ => None,
        }
    }
}

// Auto-convert from std::io::Error
impl From<std::io::Error> for PacketError {
    fn from(e: std::io::Error) -> Self {
        PacketError::Io { context: "unknown", source: e }
    }
}

// Auto-convert from serde_json::Error
impl From<serde_json::Error> for PacketError {
    fn from(e: serde_json::Error) -> Self {
        PacketError::Json { context: "unknown", source: e }
    }
}

// Auto-convert from base64::DecodeError
impl From<base64::DecodeError> for PacketError {
    fn from(e: base64::DecodeError) -> Self {
        PacketError::Base64 { context: "unknown", source: e }
    }
}

/// Convenience alias used everywhere in the protocol layer.
pub type PResult<T> = Result<T, PacketError>;
