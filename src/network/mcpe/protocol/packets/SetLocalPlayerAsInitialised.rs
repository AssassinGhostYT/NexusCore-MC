use crate::protocol::error::PResult;
use crate::protocol::varint::read_varu64;

pub struct SetLocalPlayerAsInitialized {
    pub entity_runtime_id: u64,
}

impl SetLocalPlayerAsInitialized {
    pub fn read(payload: &[u8]) -> PResult<Self> {
        let mut buf = &payload[..];
        let entity_runtime_id = read_varu64(&mut buf).ok_or_else(|| {
            log::error!("read SetLocalPlayerAsInitialised.entity_runtime_id failed");
            crate::protocol::error::PacketError::VarintOverflow { kind: "SetLocalPlayerAsInitialised.entity_runtime_id" }
        })?;
        Ok(Self { entity_runtime_id })
    }
}
