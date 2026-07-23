use std::fmt;

#[derive(Debug)]
pub enum PacketError {
    Io {
        context: &'static str,
        source: std::io::Error,
    },
    Format {
        packet: &'static str,
        detail: String,
    },
    VarintOverflow {
        kind: &'static str,
    },
    Underflow {
        field: &'static str,
        need: usize,
        have: usize,
    },
    Json {
        context: &'static str,
        source: serde_json::Error,
    },
}

impl fmt::Display for PacketError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PacketError::Io { context, source } => write!(f, "IO error in {}: {}", context, source),
            PacketError::Format { packet, detail } => write!(f, "Format error in {}: {}", packet, detail),
            PacketError::VarintOverflow { kind } => write!(f, "Varint overflow reading {}", kind),
            PacketError::Underflow { field, need, have } => {
                write!(f, "Buffer underflow in {}: needed {} bytes, available {}", field, need, have)
            }
            PacketError::Json { context, source } => write!(f, "JSON error in {}: {}", context, source),
        }
    }
}

impl std::error::Error for PacketError {}

impl From<std::io::Error> for PacketError {
    fn from(source: std::io::Error) -> Self {
        PacketError::Io {
            context: "io",
            source,
        }
    }
}

pub type PResult<T> = Result<T, PacketError>;
