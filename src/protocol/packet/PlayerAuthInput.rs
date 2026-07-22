use byteorder::{LittleEndian, ReadBytesExt};
use crate::protocol::error::PResult;
use super::move_player::MovePlayerPosition;

pub struct PlayerAuthInput {
    pub pitch: f32,
    pub yaw: f32,
    pub position: MovePlayerPosition,
    pub tick: u64,
}

impl PlayerAuthInput {
    pub fn read(payload: &[u8]) -> PResult<Self> {
        if payload.len() < 20 {
            return Err(crate::protocol::error::PacketError::Underflow {
                field: "PlayerAuthInput",
                need: 20,
                have: payload.len(),
            });
        }
        let mut buf = &payload[..];
        let pitch = buf.read_f32::<LittleEndian>().map_err(|e| {
            crate::protocol::error::PacketError::Io { context: "PlayerAuthInput.pitch", source: e }
        })?;
        let yaw = buf.read_f32::<LittleEndian>().map_err(|e| {
            crate::protocol::error::PacketError::Io { context: "PlayerAuthInput.yaw", source: e }
        })?;
        let x = buf.read_f32::<LittleEndian>().map_err(|e| {
            crate::protocol::error::PacketError::Io { context: "PlayerAuthInput.x", source: e }
        })?;
        let y = buf.read_f32::<LittleEndian>().map_err(|e| {
            crate::protocol::error::PacketError::Io { context: "PlayerAuthInput.y", source: e }
        })?;
        let z = buf.read_f32::<LittleEndian>().map_err(|e| {
            crate::protocol::error::PacketError::Io { context: "PlayerAuthInput.z", source: e }
        })?;

        Ok(Self {
            pitch,
            yaw,
            position: MovePlayerPosition { x, y, z },
            tick: 0,
        })
    }
}
