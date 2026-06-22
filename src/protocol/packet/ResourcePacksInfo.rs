use byteorder::{LittleEndian, WriteBytesExt};
use super::helpers::write_string;

pub const ID_RESOURCE_PACKS_INFO: u32 = 6;

#[derive(Debug, Clone)]
pub struct ResourcePacksInfo {
    pub must_accept: bool,
    pub has_addons: bool,
    pub has_scripts: bool,
}

impl ResourcePacksInfo {
    pub fn write(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.push(if self.must_accept { 1 } else { 0 });
        buf.push(if self.has_addons { 1 } else { 0 });
        buf.push(if self.has_scripts { 1 } else { 0 });
        
        // Force disable vibrant visuals: bool (false)
        buf.push(0);
        
        // World template ID: UUID (16 bytes of 0)
        buf.extend_from_slice(&[0u8; 16]);
        
        // World template version: string (empty)
        write_string(&mut buf, "");
        
        // Texture pack count: u16 little endian (0)
        buf.write_u16::<LittleEndian>(0).unwrap();
        
        buf
    }
}
