use crate::protocol::error::PResult;
use crate::protocol::varint::write_varu32;

pub const ID_SET_DIFFICULTY: u32 = 60;

pub struct SetDifficulty {
    pub difficulty: u32,
}

impl SetDifficulty {
    pub fn write(&self) -> PResult<Vec<u8>> {
        let mut buf = Vec::new();
        write_varu32(&mut buf, self.difficulty);
        Ok(buf)
    }
}
