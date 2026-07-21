// MovePlayer — ID 19
// Sent by the client to update its position, and by the server to update other entities' positions.

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use crate::protocol::error::PResult;
use crate::protocol::varint::{write_varu64, read_varu64};

pub struct MovePlayerPosition {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub struct MovePlayer {
    pub runtime_entity_id: u64,
    pub position: MovePlayerPosition,
    pub pitch: f32,
    pub yaw: f32,
    pub head_yaw: f32,
    pub mode: u8,
    pub on_ground: bool,
    pub ridden_entity_runtime_id: u64,
    pub teleport_cause: u8,
    pub source_actor_type: u8,
    pub tick: u64,
}

impl MovePlayer {
    pub fn read(payload: &[u8]) -> PResult<Self> {
        let mut buf = &payload[..];
        let runtime_entity_id = read_varu64(&mut buf).ok_or_else(|| {
            crate::protocol::error::PacketError::VarintOverflow { kind: "MovePlayer.runtime_entity_id" }
        })?;
        let x = buf.read_f32::<LittleEndian>().map_err(|e| {
            crate::protocol::error::PacketError::Io { context: "MovePlayer.x", source: e }
        })?;
        let y = buf.read_f32::<LittleEndian>().map_err(|e| {
            crate::protocol::error::PacketError::Io { context: "MovePlayer.y", source: e }
        })?;
        let z = buf.read_f32::<LittleEndian>().map_err(|e| {
            crate::protocol::error::PacketError::Io { context: "MovePlayer.z", source: e }
        })?;
        Ok(Self {
            runtime_entity_id,
            position: MovePlayerPosition { x, y, z },
            pitch: 0.0, yaw: 0.0, head_yaw: 0.0,
            mode: 0, on_ground: false,
            ridden_entity_runtime_id: 0,
            teleport_cause: 0, source_actor_type: 0, tick: 0,
        })
    }

    pub fn write(&self) -> PResult<Vec<u8>> {
        let mut buf = Vec::new();
        write_varu64(&mut buf, self.runtime_entity_id);
        buf.write_f32::<LittleEndian>(self.position.x).unwrap();
        buf.write_f32::<LittleEndian>(self.position.y).unwrap();
        buf.write_f32::<LittleEndian>(self.position.z).unwrap();
        buf.write_f32::<LittleEndian>(self.pitch).unwrap();
        buf.write_f32::<LittleEndian>(self.yaw).unwrap();
        buf.write_f32::<LittleEndian>(self.head_yaw).unwrap();
        buf.push(self.mode);
        buf.push(if self.on_ground { 1 } else { 0 });
        write_varu64(&mut buf, self.ridden_entity_runtime_id);
        
        if self.mode == 2 {
            buf.write_i32::<LittleEndian>(self.teleport_cause as i32).unwrap();
            buf.write_i32::<LittleEndian>(self.source_actor_type as i32).unwrap();
        }
        
        write_varu64(&mut buf, self.tick);
        Ok(buf)
    }
}
