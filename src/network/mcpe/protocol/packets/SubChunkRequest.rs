pub const ID_SUB_CHUNK_REQUEST: u32 = 175;

use crate::protocol::error::{PResult, PacketError};
use crate::protocol::varint::{read_vari32, read_varu32};
use byteorder::{LittleEndian, ReadBytesExt};

pub struct SubChunkOffset {
    pub x: i8,
    pub y: i8,
    pub z: i8,
}

pub struct SubChunkRequest {
    pub dimension: i32,
    pub offsets: Vec<SubChunkOffset>,
    pub position: (i32, i32, i32),
}

impl SubChunkRequest {
    pub fn read(buf: &mut &[u8]) -> PResult<Self> {
        let dimension = read_vari32(buf).ok_or(PacketError::Format {
            packet: "SubChunkRequest",
            detail: "failed to read dimension".to_string(),
        })?;
        
        let count = read_varu32(buf).ok_or(PacketError::Format {
            packet: "SubChunkRequest",
            detail: "failed to read offsets count".to_string(),
        })? as usize;
        
        let mut offsets = Vec::with_capacity(count);
        for _ in 0..count {
            if buf.len() < 3 {
                break;
            }
            let x = buf.read_i8()?;
            let y = buf.read_i8()?;
            let z = buf.read_i8()?;
            offsets.push(SubChunkOffset { x, y, z });
        }
        
        if buf.len() < 12 {
            return Err(PacketError::Format {
                packet: "SubChunkRequest",
                detail: "buffer too small for position".to_string(),
            });
        }
        let pos_x = buf.read_i32::<LittleEndian>()?;
        let pos_y = buf.read_i32::<LittleEndian>()?;
        let pos_z = buf.read_i32::<LittleEndian>()?;
        
        Ok(Self {
            dimension,
            offsets,
            position: (pos_x, pos_y, pos_z),
        })
    }
}
