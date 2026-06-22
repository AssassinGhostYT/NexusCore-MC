use crate::protocol::varint::write_vari32;

pub const ID_CHUNK_RADIUS_UPDATED: u32 = 70;

#[derive(Debug, Clone)]
pub struct ChunkRadiusUpdated {
    pub radius: i32,
}

impl ChunkRadiusUpdated {
    pub fn write(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        write_vari32(&mut buf, self.radius);
        buf
    }
}
