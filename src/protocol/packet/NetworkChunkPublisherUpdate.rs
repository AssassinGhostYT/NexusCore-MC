use crate::protocol::varint::{write_vari32, write_varu32};
use byteorder::{LittleEndian, WriteBytesExt};

pub const ID_NETWORK_CHUNK_PUBLISHER_UPDATE: u32 = 121;

#[derive(Debug, Clone)]
pub struct NetworkChunkPublisherUpdate {
    pub position: (i32, i32, i32),
    pub radius: u32,
}

impl NetworkChunkPublisherUpdate {
    pub fn write(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        write_vari32(&mut buf, self.position.0);
        write_vari32(&mut buf, self.position.1);
        write_vari32(&mut buf, self.position.2);
        write_varu32(&mut buf, self.radius);
        buf.write_u32::<LittleEndian>(0).unwrap();
        buf
    }
}
