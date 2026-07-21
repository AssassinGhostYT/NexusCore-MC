use crate::protocol::error::PResult;
use crate::protocol::varint::write_vari32;

pub struct ChunkRadiusUpdated {
    pub radius: i32,
}

impl ChunkRadiusUpdated {
    pub fn write(&self) -> PResult<Vec<u8>> {
        let mut buf = Vec::new();
        write_vari32(&mut buf, self.radius);
        Ok(buf)
    }
}
