// CameraAimAssistPresets — ID 320
use crate::protocol::error::PResult;
use crate::protocol::varint::write_varu32;
use byteorder::WriteBytesExt;

pub struct CameraAimAssistPresets;

impl CameraAimAssistPresets {
    pub fn write(&self) -> PResult<Vec<u8>> {
        let mut buf = Vec::new();
        write_varu32(&mut buf, 0); // Categories count
        write_varu32(&mut buf, 0); // Presets count
        buf.write_u8(0).unwrap();  // Operation
        Ok(buf)
    }
}
