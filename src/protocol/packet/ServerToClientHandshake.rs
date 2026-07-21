use crate::protocol::error::PResult;
use crate::macros::helpers;

pub struct ServerToClientHandshake {
    pub jwt: String,
}

impl ServerToClientHandshake {
    pub fn write(&self) -> PResult<Vec<u8>> {
        let mut buf = Vec::new();
        helpers::write_string(&mut buf, &self.jwt);
        Ok(buf)
    }
}
