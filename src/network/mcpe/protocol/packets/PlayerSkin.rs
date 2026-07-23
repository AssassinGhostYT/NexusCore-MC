// PlayerSkin — ID 93
pub const ID_PLAYER_SKIN: u32 = 93;
use crate::protocol::error::PResult;

pub struct PlayerSkin {
    pub uuid: [u8; 16],
}

impl PlayerSkin {
    pub fn read(buf: &mut &[u8]) -> PResult<Self> {
        let mut uuid = [0u8; 16];
        if buf.len() >= 16 {
            uuid.copy_from_slice(&buf[..16]);
            *buf = &buf[16..];
        }
        Ok(Self { uuid })
    }
}
