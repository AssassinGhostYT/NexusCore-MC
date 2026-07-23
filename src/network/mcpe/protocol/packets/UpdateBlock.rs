use crate::protocol::error::PResult;
use crate::protocol::varint::{write_vari32, write_varu32};

pub const ID_UPDATE_BLOCK: u32 = 21; // 0x15

pub const FLAG_NETWORK: u32 = 0b0010;
pub const DATA_LAYER_NORMAL: u32 = 0;

/// UpdateBlock packet (ID = 21 / 0x15).
/// Sent by server when a single block in the world changes.
pub struct UpdateBlock {
    pub position: (i32, i32, i32),
    pub block_runtime_id: u32,
    pub flags: u32,
    pub data_layer_id: u32,
}

impl UpdateBlock {
    pub fn new(position: (i32, i32, i32), block_runtime_id: u32) -> Self {
        Self {
            position,
            block_runtime_id,
            flags: FLAG_NETWORK,
            data_layer_id: DATA_LAYER_NORMAL,
        }
    }

    pub fn write(&self) -> PResult<Vec<u8>> {
        let mut buf = Vec::new();

        write_vari32(&mut buf, self.position.0);
        write_vari32(&mut buf, self.position.1);
        write_vari32(&mut buf, self.position.2);

        write_varu32(&mut buf, self.block_runtime_id);
        write_varu32(&mut buf, self.flags);
        write_varu32(&mut buf, self.dataLayerId_to_varu32());

        Ok(buf)
    }

    fn dataLayerId_to_varu32(&self) -> u32 {
        self.data_layer_id
    }
}
