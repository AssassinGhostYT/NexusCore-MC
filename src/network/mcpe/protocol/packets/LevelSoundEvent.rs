use byteorder::{LittleEndian, WriteBytesExt};
use crate::protocol::error::PResult;
use crate::protocol::varint::write_vari32;
use crate::macros::helpers::write_string;

pub const ID_LEVEL_SOUND_EVENT: u32 = 24; // 0x18

/// LevelSoundEvent packet (ID = 24 / 0x18).
///
/// Sent by server or client to trigger preset world/entity sound events
/// (e.g. chest open/close, portal sound, rain, item use, entity hurt/death).
pub struct LevelSoundEvent {
    pub sound_identifier: String,
    pub position: (f32, f32, f32),
    pub extra_data: i32,
    pub entity_type: String,
    pub is_baby_mob: bool,
    pub disable_relative_volume: bool,
    pub actor_unique_id: i64,
    pub fire_position: Option<(f32, f32, f32)>,
}

impl LevelSoundEvent {
    pub fn new(sound_identifier: String, position: (f32, f32, f32)) -> Self {
        Self {
            sound_identifier,
            position,
            extra_data: -1,
            entity_type: ":".to_string(),
            is_baby_mob: false,
            disable_relative_volume: false,
            actor_unique_id: -1,
            fire_position: None,
        }
    }

    pub fn write(&self) -> PResult<Vec<u8>> {
        let mut buf = Vec::new();

        // 1. Sound Identifier
        write_string(&mut buf, &self.sound_identifier);

        // 2. Position (f32 LE x, y, z)
        buf.write_f32::<LittleEndian>(self.position.0).unwrap();
        buf.write_f32::<LittleEndian>(self.position.1).unwrap();
        buf.write_f32::<LittleEndian>(self.position.2).unwrap();

        // 3. Extra Data (VarI32)
        write_vari32(&mut buf, self.extra_data);

        // 4. Entity Type String
        write_string(&mut buf, &self.entity_type);

        // 5. Flags
        buf.push(if self.is_baby_mob { 1 } else { 0 });
        buf.push(if self.disable_relative_volume { 1 } else { 0 });

        // 6. Actor Unique ID (i64 LE)
        buf.write_i64::<LittleEndian>(self.actor_unique_id).unwrap();

        // 7. Fire Position (Option<(f32, f32, f32)>)
        match self.fire_position {
            Some(pos) => {
                buf.push(1);
                buf.write_f32::<LittleEndian>(pos.0).unwrap();
                buf.write_f32::<LittleEndian>(pos.1).unwrap();
                buf.write_f32::<LittleEndian>(pos.2).unwrap();
            }
            None => {
                buf.push(0);
            }
        }

        Ok(buf)
    }
}
