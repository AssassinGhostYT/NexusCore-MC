use byteorder::{LittleEndian, WriteBytesExt};
use crate::protocol::error::PResult;
use crate::protocol::varint::write_vari32;

pub struct LevelEvent {
    pub event_type: i32,
    pub position: (f32, f32, f32),
    pub data: i32,
}

impl LevelEvent {
    pub fn write(&self) -> PResult<Vec<u8>> {
        let mut buf = Vec::new();
        write_vari32(&mut buf, self.event_type);
        buf.write_f32::<LittleEndian>(self.position.0).unwrap();
        buf.write_f32::<LittleEndian>(self.position.1).unwrap();
        buf.write_f32::<LittleEndian>(self.position.2).unwrap();
        write_vari32(&mut buf, self.data);
        Ok(buf)
    }
}
