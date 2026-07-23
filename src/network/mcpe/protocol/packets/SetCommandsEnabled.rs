use crate::protocol::error::PResult;

pub const ID_SET_COMMANDS_ENABLED: u32 = 59;

pub struct SetCommandsEnabled {
    pub enabled: bool,
}

impl SetCommandsEnabled {
    pub fn write(&self) -> PResult<Vec<u8>> {
        let mut buf = Vec::new();
        buf.push(if self.enabled { 1 } else { 0 });
        Ok(buf)
    }
}
