use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use crate::protocol::varint::{write_varu32};
use super::helpers::{read_string, write_string};

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

pub const ID_RESOURCE_PACK_CLIENT_RESPONSE: u32 = 8;

#[derive(Debug, Clone)]
pub struct ResourcePackClientResponse {
    pub response_status: u8,
    #[allow(dead_code)]
    pub pack_ids: Vec<(String, String)>,
}

impl ResourcePackClientResponse {
    pub fn read(mut payload: &[u8]) -> Option<Self> {
        let response_status = payload.read_u8().ok()?;
        let mut pack_ids = Vec::new();
        if !payload.is_empty() {
            let pack_ids_len = payload.read_u16::<LittleEndian>().ok()? as usize;
            for _ in 0..pack_ids_len {
                let pack_id = read_string(&mut payload)?;
                let pack_version = read_string(&mut payload)?;
                pack_ids.push((pack_id, pack_version));
            }
        }
        Some(ResourcePackClientResponse { response_status, pack_ids })
    }
}
