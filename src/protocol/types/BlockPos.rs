use crate::protocol::varint::write_vari32;

#[derive(Clone, Debug)]
pub struct BlockPos {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl BlockPos {
    pub fn write(&self, buf: &mut Vec<u8>) {
        write_vari32(buf, self.x);
        write_vari32(buf, self.y);
        write_vari32(buf, self.z);
    }
}
