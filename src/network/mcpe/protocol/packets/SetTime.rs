use crate::protocol::error::PResult;
use crate::protocol::varint::write_vari32;

pub const ID_SET_TIME: u32 = 10;

pub struct SetTime {
    pub time: i32,
}

impl SetTime {
    pub fn write(&self) -> PResult<Vec<u8>> {
        let mut buf = Vec::new();
        write_vari32(&mut buf, self.time);
        Ok(buf)
    }
}
