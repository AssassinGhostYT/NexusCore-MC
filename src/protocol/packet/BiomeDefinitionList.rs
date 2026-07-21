// BiomeDefinitionList — ID 122
// Sent by the server after chunk radius is set.
// Contains biome definitions in NBT format.
// We send an empty list for now.

/// BiomeDefinitionList — ID 122
/// In v1001, send empty NBT compound (client uses built-in biomes).
use crate::protocol::varint::write_varu32;
use crate::protocol::error::PResult;

pub struct BiomeDefinitionList;

impl BiomeDefinitionList {
    pub fn empty() -> Self { Self }

    pub fn write(&self) -> PResult<Vec<u8>> {
        let mut buf = Vec::new();
        // biomes: Vec<BiomeEntry> -> count = 0
        write_varu32(&mut buf, 0);
        // strings: Vec<String> -> count = 0
        write_varu32(&mut buf, 0);
        Ok(buf)
    }
}
