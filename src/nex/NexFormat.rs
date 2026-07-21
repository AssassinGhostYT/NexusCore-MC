// NexFormat — format constants for .nex archives

/// Magic bytes identifying a .nex file
pub const NEX_MAGIC: &[u8; 4] = b"NEX1";

/// Maximum file size allowed inside a .nex archive (50 MB)
pub const MAX_ENTRY_SIZE: u32 = 50 * 1024 * 1024;

/// Maximum number of files in a single .nex archive
pub const MAX_ENTRIES: u32 = 1000;
