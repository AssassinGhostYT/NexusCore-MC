use crate::protocol::error::PResult;
use crate::protocol::varint::write_varu32;
use byteorder::{LittleEndian, WriteBytesExt};

pub struct ResourcePacksInfo {
    pub must_accept: bool,
    pub has_addons: bool,
    pub has_scripts: bool,
}

impl ResourcePacksInfo {
    pub fn write(&self) -> PResult<Vec<u8>> {
        let mut buf = Vec::new();
        buf.push(if self.must_accept { 1 } else { 0 });
        buf.push(if self.has_addons { 1 } else { 0 });
        buf.push(if self.has_scripts { 1 } else { 0 });
        buf.push(0); // force_disable_vibrant_visuals
        buf.extend_from_slice(&[0u8; 16]); // world_template_uuid (16 zero bytes)
        write_varu32(&mut buf, 0); // world_template_version (empty string)
        buf.write_u16::<LittleEndian>(0).unwrap(); // resource_packs count (u16 LE)
        Ok(buf)
    }
}
