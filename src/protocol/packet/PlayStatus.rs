use byteorder::{LittleEndian, WriteBytesExt};
use crate::protocol::error::PResult;

pub struct PlayStatus {
    pub status: i32,
}

impl PlayStatus {
    pub fn write(&self) -> PResult<Vec<u8>> {
        let mut buf = Vec::new();
        buf.write_i32::<LittleEndian>(self.status).unwrap();
        Ok(buf)
    }
}
