#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ChunkPos {
    pub x: i32,
    pub z: i32,
}

impl ChunkPos {
    pub fn write(&self, buf: &mut Vec<u8>) {
        use crate::protocol::varint::write_vari32;
        write_vari32(buf, self.x);
        write_vari32(buf, self.z);
    }
}
