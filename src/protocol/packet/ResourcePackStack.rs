use byteorder::{LittleEndian, WriteBytesExt};
use crate::protocol::varint::write_varu32;
use super::helpers::write_string;

pub const ID_RESOURCE_PACK_STACK: u32 = 7;

#[derive(Debug, Clone)]
pub struct ResourcePackStack {
    pub must_accept: bool,
    pub game_version: String,
}

impl ResourcePackStack {
    pub fn write(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.push(if self.must_accept { 1 } else { 0 });
        
        // Resource packs: varu32 length (0)
        write_varu32(&mut buf, 0);
        
        // Game version: string
        write_string(&mut buf, &self.game_version);
        
        // Experiments count: u32 little endian (0)
        buf.write_u32::<LittleEndian>(0).unwrap();
        
        // Experiments previously toggled: bool (false)
        buf.push(0);
        
        // Include editor packs: bool (false)
        buf.push(0);
        
        buf
    }
}
