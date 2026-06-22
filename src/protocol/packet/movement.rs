use byteorder::{LittleEndian, ReadBytesExt};
use crate::protocol::varint::{read_varu32};

pub const ID_MOVE_PLAYER: u32 = 19;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct MovePlayer {
    pub runtime_entity_id: u64,
    pub position: (f32, f32, f32),
    pub pitch: f32,
    pub yaw: f32,
    pub head_yaw: f32,
    pub mode: u8,
    pub on_ground: bool,
}

impl MovePlayer {
    pub fn read(mut payload: &[u8]) -> Option<Self> {
        let runtime_entity_id = crate::protocol::varint::read_varu64(&mut payload)?;
        let pos_x = payload.read_f32::<LittleEndian>().ok()?;
        let pos_y = payload.read_f32::<LittleEndian>().ok()?;
        let pos_z = payload.read_f32::<LittleEndian>().ok()?;
        let pitch = payload.read_f32::<LittleEndian>().ok()?;
        let yaw = payload.read_f32::<LittleEndian>().ok()?;
        let head_yaw = payload.read_f32::<LittleEndian>().ok()?;
        let mode = payload.read_u8().ok()?;
        let on_ground = payload.read_u8().ok()? != 0;
        Some(MovePlayer {
            runtime_entity_id,
            position: (pos_x, pos_y, pos_z),
            pitch,
            yaw,
            head_yaw,
            mode,
            on_ground,
        })
    }
}

pub const ID_PLAYER_AUTH_INPUT: u32 = 144;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct PlayerAuthInput {
    pub pitch: f32,
    pub yaw: f32,
    pub position: (f32, f32, f32),
    pub move_vector: (f32, f32),
    pub head_yaw: f32,
    pub input_flags: u64,
    pub input_mode: u32,
    pub play_mode: u32,
}

impl PlayerAuthInput {
    pub fn read(mut payload: &[u8]) -> Option<Self> {
        let pitch = payload.read_f32::<LittleEndian>().ok()?;
        let yaw = payload.read_f32::<LittleEndian>().ok()?;
        let pos_x = payload.read_f32::<LittleEndian>().ok()?;
        let pos_y = payload.read_f32::<LittleEndian>().ok()?;
        let pos_z = payload.read_f32::<LittleEndian>().ok()?;
        let move_x = payload.read_f32::<LittleEndian>().ok()?;
        let move_z = payload.read_f32::<LittleEndian>().ok()?;
        let head_yaw = payload.read_f32::<LittleEndian>().ok()?;
        let input_flags = crate::protocol::varint::read_varu64(&mut payload)?;
        let input_mode = read_varu32(&mut payload)?;
        let play_mode = read_varu32(&mut payload)?;
        Some(PlayerAuthInput {
            pitch,
            yaw,
            position: (pos_x, pos_y, pos_z),
            move_vector: (move_x, move_z),
            head_yaw,
            input_flags,
            input_mode,
            play_mode,
        })
    }
}
