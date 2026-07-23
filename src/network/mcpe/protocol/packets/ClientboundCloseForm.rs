// ClientboundCloseForm — ID 310 (0x136)
// Sent by the server to close a form on the client.
// Has no fields in v1001.

use crate::protocol::error::PResult;

pub struct ClientboundCloseForm;

impl ClientboundCloseForm {
    pub fn new() -> Self {
        Self
    }

    pub fn write(&self) -> PResult<Vec<u8>> {
        Ok(Vec::new())
    }
}
