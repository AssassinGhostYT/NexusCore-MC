use crate::protocol::varint::read_vari32;

pub const ID_REQUEST_CHUNK_RADIUS: u32 = 69;

#[derive(Debug, Clone)]
pub struct RequestChunkRadius {
    pub radius: i32,
}

impl RequestChunkRadius {
    pub fn read(mut payload: &[u8]) -> Option<Self> {
        let radius = read_vari32(&mut payload)?;
        Some(RequestChunkRadius { radius })
    }
}
