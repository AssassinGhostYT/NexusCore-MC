use byteorder::{LittleEndian, WriteBytesExt};
use crate::protocol::error::PResult;

pub struct NetworkSettings {
    pub compression_threshold: u16,
    pub compression_algorithm: u16,
    pub client_throttle: bool,
    pub client_throttle_threshold: i8,
    pub client_throttle_scalar: f32,
}

impl NetworkSettings {
    pub fn write(&self) -> PResult<Vec<u8>> {
        let mut buf = Vec::new();
        buf.write_u16::<LittleEndian>(self.compression_threshold).unwrap();
        buf.write_u16::<LittleEndian>(self.compression_algorithm).unwrap();
        buf.push(if self.client_throttle { 1 } else { 0 });
        buf.write_i8(self.client_throttle_threshold).unwrap();
        buf.write_f32::<LittleEndian>(self.client_throttle_scalar).unwrap();
        Ok(buf)
    }
}
