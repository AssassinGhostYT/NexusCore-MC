pub const ID_CLIENT_TO_SERVER_HANDSHAKE: u32 = 4;

#[derive(Debug, Clone)]
pub struct ClientToServerHandshake;

impl ClientToServerHandshake {
    pub fn read(_payload: &[u8]) -> Option<Self> {
        Some(ClientToServerHandshake)
    }
}
