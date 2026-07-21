use byteorder::{BigEndian, ReadBytesExt};
use crate::protocol::error::PResult;

pub struct RequestNetworkSettings {
    pub protocol_version: i32,
}

impl RequestNetworkSettings {
    pub fn read(payload: &[u8]) -> PResult<Self> {
        let mut buf = &payload[..];
        let protocol_version = buf.read_i32::<BigEndian>().map_err(|e| {
            log::error!("read RequestNetworkSettings.protocol_version failed: {}", e);
            crate::protocol::error::PacketError::Io { context: "RequestNetworkSettings.protocol_version", source: e }
        })?;
        Ok(Self { protocol_version })
    }
}
