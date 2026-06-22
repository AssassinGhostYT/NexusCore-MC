use byteorder::{BigEndian, ReadBytesExt};

pub const ID_REQUEST_NETWORK_SETTINGS: u32 = 193; // 0xc1

#[derive(Debug, Clone)]
pub struct RequestNetworkSettings {
    pub protocol_version: i32,
}

impl RequestNetworkSettings {
    pub fn read(mut payload: &[u8]) -> Option<Self> {
        let protocol_version = payload.read_i32::<BigEndian>().ok()?;
        Some(RequestNetworkSettings { protocol_version })
    }
}
