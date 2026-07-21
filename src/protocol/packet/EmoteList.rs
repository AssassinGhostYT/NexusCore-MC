// EmoteList — ID 152
pub const ID_EMOTE_LIST: u32 = 152;
use crate::protocol::error::{PResult, PacketError};
use crate::protocol::varint::{read_varu64, read_varu32};

pub struct EmoteList {
    pub player_runtime_id: u64,
    pub emotes: Vec<[u8; 16]>,
}

impl EmoteList {
    pub fn read(buf: &mut &[u8]) -> PResult<Self> {
        let player_runtime_id = read_varu64(buf).ok_or(PacketError::Format {
            packet: "EmoteList",
            detail: "failed to read player_runtime_id".to_string(),
        })?;
        
        if buf.is_empty() {
            return Ok(Self {
                player_runtime_id,
                emotes: Vec::new(),
            });
        }
        let count = read_varu32(buf).ok_or(PacketError::Format {
            packet: "EmoteList",
            detail: "failed to read emotes count".to_string(),
        })? as usize;
        let mut emotes = Vec::with_capacity(count);
        for _ in 0..count {
            if buf.len() < 16 {
                break;
            }
            let mut uuid = [0u8; 16];
            uuid.copy_from_slice(&buf[..16]);
            *buf = &buf[16..];
            emotes.push(uuid);
        }
        
        Ok(Self {
            player_runtime_id,
            emotes,
        })
    }
}
