use crate::protocol::error::PResult;

/// Network-level permission flags sent inside StartGame.
pub struct NetworkPermissions {
    pub server_auth_sound_enabled: bool,
}

impl NetworkPermissions {
    pub fn new() -> Self {
        Self {
            server_auth_sound_enabled: true,
        }
    }

    /// Serialize into bytes and append to `buf`.
    pub fn write_into(&self, buf: &mut Vec<u8>) {
        buf.push(if self.server_auth_sound_enabled { 1 } else { 0 });
    }

    pub fn write(&self) -> PResult<Vec<u8>> {
        let mut buf = Vec::new();
        self.write_into(&mut buf);
        Ok(buf)
    }
}
