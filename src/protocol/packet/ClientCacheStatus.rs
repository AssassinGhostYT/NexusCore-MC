// ClientCacheStatus — ID 129
// Sent by the client to tell the server whether it supports the client-side blob cache.
// If supported is true, the client will use the blob cache for chunks.

use crate::protocol::error::{PacketError, PResult};

pub struct ClientCacheStatus {
    pub supported: bool,
}

impl ClientCacheStatus {
    pub fn read(payload: &[u8]) -> PResult<Self> {
        if payload.is_empty() {
            return Err(PacketError::Format {
                packet: "ClientCacheStatus",
                detail: "empty payload".into(),
            });
        }
        Ok(Self {
            supported: payload[0] != 0,
        })
    }
}
