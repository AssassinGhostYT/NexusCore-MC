use super::helpers::{read_string, write_string};

pub const ID_SERVER_TO_CLIENT_HANDSHAKE: u32 = 3;
pub const ID_CLIENT_TO_SERVER_HANDSHAKE: u32 = 4;

#[derive(Debug, Clone)]
pub struct ServerToClientHandshake {
    pub jwt: String,
}

impl ServerToClientHandshake {
    pub fn read(mut payload: &[u8]) -> Option<Self> {
        let jwt = read_string(&mut payload)?;
        Some(ServerToClientHandshake { jwt })
    }

    pub fn write(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        write_string(&mut buf, &self.jwt);
        buf
    }
}
