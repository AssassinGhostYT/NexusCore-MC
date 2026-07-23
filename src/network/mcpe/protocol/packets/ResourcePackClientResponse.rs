use crate::protocol::error::{PacketError, PResult};

pub struct ResourcePackClientResponse {
    pub response_status: u8,
}

impl ResourcePackClientResponse {
    pub fn read(payload: &[u8]) -> PResult<Self> {
        if payload.is_empty() {
            return Err(PacketError::Format { packet: "ResourcePackClientResponse", detail: "empty payload".into() });
        }
        Ok(Self { response_status: payload[0] })
    }
}
