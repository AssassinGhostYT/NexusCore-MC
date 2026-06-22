use byteorder::{LittleEndian, ReadBytesExt};

pub const ID_MOVE_PLAYER: u32 = 19;

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
