use super::helpers::write_string;

pub const ID_SERVER_TO_CLIENT_HANDSHAKE: u32 = 3;

#[derive(Debug, Clone)]
pub struct ServerToClientHandshake {
    pub jwt: String,
}

impl ServerToClientHandshake {
    pub fn write(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        write_string(&mut buf, &self.jwt);
        buf
    }
}
