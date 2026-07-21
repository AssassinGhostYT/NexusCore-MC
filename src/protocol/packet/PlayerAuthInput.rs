use byteorder::{LittleEndian, ReadBytesExt};
use crate::protocol::error::PResult;
use crate::protocol::varint::{read_varu64, read_vari32};
use super::move_player::MovePlayerPosition;

pub struct PlayerAuthInput {
    pub pitch: f32,
    pub yaw: f32,
    pub position: MovePlayerPosition,
    pub tick: u64,
}

impl PlayerAuthInput {
    pub fn read(payload: &[u8]) -> PResult<Self> {
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
        // Skip movement fields (head_x, head_y, head_z)
        buf.read_f32::<LittleEndian>().map_err(|e| {
            crate::protocol::error::PacketError::Io { context: "PlayerAuthInput.head_x", source: e }
        })?;
        buf.read_f32::<LittleEndian>().map_err(|e| {
            crate::protocol::error::PacketError::Io { context: "PlayerAuthInput.head_y", source: e }
        })?;
        buf.read_f32::<LittleEndian>().map_err(|e| {
            crate::protocol::error::PacketError::Io { context: "PlayerAuthInput.head_z", source: e }
        })?;
        let _input_data = read_varu64(&mut buf).ok_or_else(|| {
            crate::protocol::error::PacketError::VarintOverflow { kind: "PlayerAuthInput.input_data" }
        })?;
        let _input_mode = buf.read_u8().map_err(|e| {
            crate::protocol::error::PacketError::Io { context: "PlayerAuthInput.input_mode", source: e }
        })?;
        let _play_mode = buf.read_u32::<LittleEndian>().map_err(|e| {
            crate::protocol::error::PacketError::Io { context: "PlayerAuthInput.play_mode", source: e }
        })?;
        let _new_interaction_model = read_vari32(&mut buf).ok_or_else(|| {
            crate::protocol::error::PacketError::VarintOverflow { kind: "PlayerAuthInput.new_interaction_model" }
        })?;
        let tick = read_varu64(&mut buf).ok_or_else(|| {
            crate::protocol::error::PacketError::VarintOverflow { kind: "PlayerAuthInput.tick" }
        })?;
        // Skip input_auth (3 floats)
        buf.read_f32::<LittleEndian>().map_err(|e| {
            crate::protocol::error::PacketError::Io { context: "PlayerAuthInput.input_auth_0", source: e }
        })?;
        buf.read_f32::<LittleEndian>().map_err(|e| {
            crate::protocol::error::PacketError::Io { context: "PlayerAuthInput.input_auth_1", source: e }
        })?;
        buf.read_f32::<LittleEndian>().map_err(|e| {
            crate::protocol::error::PacketError::Io { context: "PlayerAuthInput.input_auth_2", source: e }
        })?;
        Ok(Self { pitch, yaw, position: MovePlayerPosition { x, y, z }, tick })
    }
}
