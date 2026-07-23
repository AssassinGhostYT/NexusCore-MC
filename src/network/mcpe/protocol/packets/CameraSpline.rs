// CameraSpline — ID 338
use crate::protocol::error::PResult;
use crate::protocol::varint::write_varu32;

pub struct CameraSpline;

impl CameraSpline {
    pub fn write(&self) -> PResult<Vec<u8>> {
        let mut buf = Vec::new();
        write_varu32(&mut buf, 0); // count = 0
        Ok(buf)
    }
}
