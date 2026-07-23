// Interact — ID 33
pub const ID_INTERACT: u32 = 33;
use byteorder::{LittleEndian, ReadBytesExt};
use crate::protocol::error::{PResult, PacketError};
use crate::protocol::varint::read_varu64;

pub struct Interact {
    pub action_type: u8,
    pub target_entity_runtime_id: u64,
    pub position: Option<(f32, f32, f32)>,
}

impl Interact {
    pub fn read(buf: &mut &[u8]) -> PResult<Self> {
        let action_type = buf.read_u8()?;
        let target_entity_runtime_id = read_varu64(buf).ok_or(PacketError::Format {
            packet: "Interact",
            detail: "failed to read target_entity_runtime_id".to_string(),
        })?;
        
        if buf.is_empty() {
            return Ok(Self {
                action_type,
                target_entity_runtime_id,
                position: None,
            });
        }
        let has_pos = buf.read_u8()? != 0;
        let position = if has_pos {
            let x = buf.read_f32::<LittleEndian>()?;
            let y = buf.read_f32::<LittleEndian>()?;
            let z = buf.read_f32::<LittleEndian>()?;
            Some((x, y, z))
        } else {
            None
        };
        
        Ok(Self {
            action_type,
            target_entity_runtime_id,
            position,
        })
    }
}
