pub const ID_NETWORK_CHUNK_PUBLISHER_UPDATE: u32 = 121;

use crate::protocol::error::PResult;
use crate::protocol::varint::write_varu32;
use byteorder::{LittleEndian, WriteBytesExt};
use crate::protocol::types::{BlockPos, ChunkPos};

pub struct NetworkChunkPublisherUpdate {
    pub position: BlockPos,
    pub radius: u32,
    pub server_built_chunks: Vec<ChunkPos>,
}

impl NetworkChunkPublisherUpdate {
    pub fn write(&self) -> PResult<Vec<u8>> {
        let mut buf = Vec::new();
        // 1. Position as 3 varint32s (BlockPos)
        self.position.write(&mut buf);
        
        // 2. Radius as varuint32
        write_varu32(&mut buf, self.radius);
        
        // 3. server_built_chunks count as i32 LE (4 bytes)
        buf.write_i32::<LittleEndian>(self.server_built_chunks.len() as i32).unwrap();
        for chunk in &self.server_built_chunks {
            chunk.write(&mut buf);
        }
        
        Ok(buf)
    }
}
