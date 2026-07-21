use crate::protocol::error::PResult;
use crate::protocol::varint::{write_vari32};

/// Player movement synchronisation settings, sent inside StartGame.
/// In v1001 (v818 variant): only rewind_history_size + server_authoritative_block_breaking
/// (authority_mode was removed — client uses built-in default)
pub struct SyncedPlayerMovementSettings {
    pub rewind_history_size: i32,
    pub server_authoritative_block_breaking: bool,
}

impl SyncedPlayerMovementSettings {
    pub fn new() -> Self {
        Self {
            rewind_history_size: 0,
            server_authoritative_block_breaking: false,
        }
    }

    /// Serialize into bytes and append to `buf`.
    pub fn write_into(&self, buf: &mut Vec<u8>) {
        write_vari32(buf, self.rewind_history_size);
        buf.push(if self.server_authoritative_block_breaking { 1 } else { 0 });
    }

    pub fn write(&self) -> PResult<Vec<u8>> {
        let mut buf = Vec::new();
        self.write_into(&mut buf);
        Ok(buf)
    }
}
