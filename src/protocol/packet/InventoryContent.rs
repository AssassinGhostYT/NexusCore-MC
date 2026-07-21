use byteorder::{LittleEndian, WriteBytesExt};
use crate::protocol::error::PResult;
use crate::protocol::varint::{write_varu32, write_vari32};

pub struct InventoryContent {
    pub window_id: u32,
    pub slots: Vec<(u32, u32, u32)>,
}

impl InventoryContent {
    pub fn write(&self) -> PResult<Vec<u8>> {
        let mut buf = Vec::new();
        write_varu32(&mut buf, self.window_id);
        write_varu32(&mut buf, self.slots.len() as u32);
        for (net_id, count, meta) in &self.slots {
            if *net_id == 0 && *count == 0 && *meta == 0 {
                write_vari32(&mut buf, 0);
            } else {
                write_vari32(&mut buf, *net_id as i32);
                buf.write_u16::<LittleEndian>(*count as u16).unwrap();
                write_varu32(&mut buf, *meta);
            }
        }
        Ok(buf)
    }
}
