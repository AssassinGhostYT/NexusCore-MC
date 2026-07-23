use byteorder::{LittleEndian, WriteBytesExt};
use crate::protocol::error::PResult;

pub struct SetPlayerGameType {
    pub game_type: i32,
}

impl SetPlayerGameType {
    pub fn write(&self) -> PResult<Vec<u8>> {
        let mut buf = Vec::new();
        buf.write_i32::<LittleEndian>(self.game_type).unwrap();
        Ok(buf)
    }
}
