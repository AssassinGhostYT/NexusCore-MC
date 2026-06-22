use byteorder::{BigEndian, LittleEndian, ReadBytesExt, WriteBytesExt};

pub const ID_REQUEST_NETWORK_SETTINGS: u32 = 193; // 0xc1
pub const ID_NETWORK_SETTINGS: u32 = 143;         // 0x8f

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

#[derive(Debug, Clone)]
pub struct NetworkSettings {
    pub compression_threshold: u16,
    pub compression_algorithm: u16,
    pub client_throttle: bool,
    pub client_throttle_threshold: u8,
    pub client_throttle_scalar: f32,
}

impl NetworkSettings {
    pub fn write(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.write_u16::<LittleEndian>(self.compression_threshold).unwrap();
        buf.write_u16::<LittleEndian>(self.compression_algorithm).unwrap();
        buf.push(if self.client_throttle { 1 } else { 0 });
        buf.push(self.client_throttle_threshold);
        buf.write_f32::<LittleEndian>(self.client_throttle_scalar).unwrap();
        buf
    }
}
