// ItemStackRequest — ID 147
// Sent by the client to request item stack operations (crafting, moving items, etc.).
// For now we just read the raw payload for logging purposes.

use crate::protocol::error::PResult;

pub struct ItemStackRequest {
    pub raw: Vec<u8>,
}

impl ItemStackRequest {
    pub fn read(payload: &[u8]) -> PResult<Self> {
        Ok(Self {
            raw: payload.to_vec(),
        })
    }
}
