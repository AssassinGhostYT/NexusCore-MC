// CameraShake — ID 159
use byteorder::{LittleEndian, WriteBytesExt};
use crate::protocol::error::PResult;

pub struct CameraShake {
    pub intensity: f32,
    pub duration: f32,
    pub shake_type: u8,
    pub action: u8,
}

impl CameraShake {
    pub fn write(&self) -> PResult<Vec<u8>> {
        let mut buf = Vec::new();
        buf.write_f32::<LittleEndian>(self.intensity).unwrap();
        buf.write_f32::<LittleEndian>(self.duration).unwrap();
        buf.write_u8(self.shake_type).unwrap();
        buf.write_u8(self.action).unwrap();
        Ok(buf)
    }
}
