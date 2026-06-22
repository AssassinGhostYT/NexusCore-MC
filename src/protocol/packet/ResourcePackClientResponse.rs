use byteorder::{LittleEndian, ReadBytesExt};
use super::helpers::read_string;

pub const ID_RESOURCE_PACK_CLIENT_RESPONSE: u32 = 8;

#[derive(Debug, Clone)]
pub struct ResourcePackClientResponse {
    pub response_status: u8,
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
