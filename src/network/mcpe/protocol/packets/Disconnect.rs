use crate::protocol::error::PResult;
use crate::protocol::varint::{write_vari32, write_varu32};
use crate::macros::helpers::write_string;

pub const ID_DISCONNECT: u32 = 5; // 0x05

/// Disconnect packet (ID = 5 / 0x05).
/// Sent by server to cleanly disconnect a client with a reason and message.
pub struct Disconnect {
    pub reason: i32,
    pub message: Option<String>,
    pub filtered_message: Option<String>,
}

impl Disconnect {
    pub fn new(reason: i32, message: Option<String>) -> Self {
        Self {
            reason,
            message,
            filtered_message: None,
        }
    }

    pub fn write(&self) -> PResult<Vec<u8>> {
        let mut buf = Vec::new();

        write_vari32(&mut buf, self.reason);

        let skip_message = self.message.is_none() && self.filtered_message.is_none();
        write_varu32(&mut buf, if skip_message { 1 } else { 0 });

        if !skip_message {
            write_string(&mut buf, self.message.as_deref().unwrap_or(""));
            write_string(&mut buf, self.filtered_message.as_deref().unwrap_or(""));
        }

        Ok(buf)
    }
}
