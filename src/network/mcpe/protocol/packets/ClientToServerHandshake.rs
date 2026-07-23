// ClientToServerHandshake — ID 4
// Sent by the client in response to ServerToClientHandshake.
// Confirms encryption is ready. Payload is empty.

use crate::protocol::error::PResult;

pub struct ClientToServerHandshake;

impl ClientToServerHandshake {
    pub fn read(_payload: &[u8]) -> PResult<Self> {
        // No fields — payload is empty
        Ok(Self)
    }
}
