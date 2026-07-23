use crate::protocol::error::PResult;
use crate::protocol::varint::read_vari32;

pub struct RequestChunkRadius {
    pub chunk_radius: i32,
}

impl RequestChunkRadius {
    pub fn read(payload: &[u8]) -> PResult<Self> {
        let mut buf = &payload[..];
        let chunk_radius = read_vari32(&mut buf).ok_or_else(|| {
            log::error!("read RequestChunkRadius.chunk_radius varint failed");
            crate::protocol::error::PacketError::VarintOverflow { kind: "RequestChunkRadius.chunk_radius" }
        })? as i32;
        Ok(Self { chunk_radius })
    }
}
