// CameraAimAssist — ID 316
use byteorder::{LittleEndian, WriteBytesExt};
use crate::protocol::error::PResult;
use crate::macros::helpers;

pub struct CameraAimAssist {
    pub preset: String,
    pub angle: (f32, f32),
    pub distance: f32,
    pub target_mode: u8,
    pub action: u8,
    pub show_debug_render: bool,
}

impl CameraAimAssist {
    pub fn write(&self) -> PResult<Vec<u8>> {
        let mut buf = Vec::new();
        helpers::write_string(&mut buf, &self.preset);
        buf.write_f32::<LittleEndian>(self.angle.0).unwrap();
        buf.write_f32::<LittleEndian>(self.angle.1).unwrap();
        buf.write_f32::<LittleEndian>(self.distance).unwrap();
        buf.write_u8(self.target_mode).unwrap();
        buf.write_u8(self.action).unwrap();
        buf.write_u8(self.show_debug_render as u8).unwrap();
        Ok(buf)
    }
}
