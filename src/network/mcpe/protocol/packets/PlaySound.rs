use byteorder::{LittleEndian, WriteBytesExt};
use crate::protocol::error::PResult;
use crate::protocol::varint::{write_vari32, write_varu64};
use crate::macros::helpers::write_string;

pub const ID_PLAY_SOUND: u32 = 86;

/// PlaySound packet (ID = 86 / 0x56).
///
/// Sent by the server to play sound effects on the client.
pub struct PlaySound {
    pub sound_name: String,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub volume: f32,
    pub pitch: f32,
    pub server_sound_handle: Option<u64>,
}

impl PlaySound {
    pub fn new(sound_name: String, x: f32, y: f32, z: f32, volume: f32, pitch: f32) -> Self {
        Self {
            sound_name,
            x,
            y,
            z,
            volume,
            pitch,
            server_sound_handle: None,
        }
    }

    pub fn write(&self) -> PResult<Vec<u8>> {
        let mut buf = Vec::new();

        // 1. Sound Name
        write_string(&mut buf, &self.sound_name);

        // 2. Position (BlockPosition scaled by 8: i32 varints for x, y, z)
        write_vari32(&mut buf, (self.x * 8.0) as i32);
        write_vari32(&mut buf, (self.y * 8.0) as i32);
        write_vari32(&mut buf, (self.z * 8.0) as i32);

        // 3. Volume (f32 LE)
        buf.write_f32::<LittleEndian>(self.volume).unwrap();

        // 4. Pitch (f32 LE)
        buf.write_f32::<LittleEndian>(self.pitch).unwrap();

        // 5. Server Sound Handle (Option<u64 LE>)
        match self.server_sound_handle {
            Some(handle) => {
                buf.push(1);
                buf.write_u64::<LittleEndian>(handle).unwrap();
            }
            None => {
                buf.push(0);
            }
        }

        Ok(buf)
    }
}
