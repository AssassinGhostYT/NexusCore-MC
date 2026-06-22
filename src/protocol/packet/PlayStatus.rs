use byteorder::{BigEndian, WriteBytesExt};

pub const ID_PLAY_STATUS: u32 = 2;

#[derive(Debug, Clone)]
pub struct PlayStatus {
    pub status: i32,
}

impl PlayStatus {
    pub fn write(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.write_i32::<BigEndian>(self.status).unwrap();
        buf
    }
}
