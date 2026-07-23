use crate::protocol::error::PResult;
use crate::macros::helpers::write_string;

pub const ID_STOP_SOUND: u32 = 87; // 0x57

/// StopSound packet (ID = 87 / 0x57).
///
/// Sent by the server to stop playing a specific sound or all active sounds on the client.
pub struct StopSound {
    pub sound_name: String,
    pub stop_all: bool,
    pub stop_legacy_music: bool,
}

impl StopSound {
    pub fn new(sound_name: String, stop_all: bool, stop_legacy_music: bool) -> Self {
        Self {
            sound_name,
            stop_all,
            stop_legacy_music,
        }
    }

    pub fn write(&self) -> PResult<Vec<u8>> {
        let mut buf = Vec::new();
        write_string(&mut buf, &self.sound_name);
        buf.push(if self.stop_all { 1 } else { 0 });
        buf.push(if self.stop_legacy_music { 1 } else { 0 });
        Ok(buf)
    }
}
