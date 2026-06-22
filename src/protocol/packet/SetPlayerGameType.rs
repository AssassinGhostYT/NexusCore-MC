use crate::protocol::varint::write_vari32;

pub const ID_SET_PLAYER_GAME_TYPE: u32 = 93;

#[derive(Debug, Clone)]
pub struct SetPlayerGameType {
    pub game_type: i32,
}

impl SetPlayerGameType {
    pub fn write(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        write_vari32(&mut buf, self.game_type);
        buf
    }
}
