pub const ID_SUB_CHUNK: u32 = 175;

use crate::protocol::error::PResult;
use crate::protocol::varint::write_vari32;
use byteorder::{LittleEndian, WriteBytesExt};

pub struct SubChunk;

impl SubChunk {
    pub fn write(&self) -> PResult<Vec<u8>> {
        let mut buf = Vec::new();
        // cache_enabled = false
        buf.write_u8(0).unwrap();
        // dimension = 0
        write_vari32(&mut buf, 0);
        // Position: 3 i32s (0, 0, 0)
        buf.write_i32::<LittleEndian>(0).unwrap();
        buf.write_i32::<LittleEndian>(0).unwrap();
        buf.write_i32::<LittleEndian>(0).unwrap();
        // SubChunkEntries count = 0
        write_vari32(&mut buf, 0);
        Ok(buf)
    }
}
