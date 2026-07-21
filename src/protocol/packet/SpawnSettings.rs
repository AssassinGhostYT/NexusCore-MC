use byteorder::{LittleEndian, WriteBytesExt};
use crate::protocol::error::PResult;
use crate::protocol::varint::{write_vari32};
use crate::macros::helpers;

/// Spawn biome/dimension settings sent as part of LevelSettings.
pub struct SpawnSettings {
    /// 0 = default, 1 = user_defined (i16 LE in v1001 — NOT varint!)
    pub spawn_type: i16,
    pub user_defined_biome_name: String,
    /// 0 = overworld, 1 = nether, 2 = end
    pub dimension: i32,
}

impl SpawnSettings {
    pub fn new() -> Self {
        Self {
            spawn_type: 0,
            user_defined_biome_name: "RandomBiome".to_string(),
            dimension: 0,
        }
    }

    /// Serialize into bytes and append to `buf`.
    pub fn write_into(&self, buf: &mut Vec<u8>) {
        log::debug!("[SpawnSettings] spawn_type={} biome='{}' dim={}", self.spawn_type, self.user_defined_biome_name, self.dimension);
        buf.write_i16::<LittleEndian>(self.spawn_type).unwrap();
        helpers::write_string(buf, &self.user_defined_biome_name);
        write_vari32(buf, self.dimension);
    }

    pub fn write(&self) -> PResult<Vec<u8>> {
        let mut buf = Vec::new();
        self.write_into(&mut buf);
        Ok(buf)
    }
}
