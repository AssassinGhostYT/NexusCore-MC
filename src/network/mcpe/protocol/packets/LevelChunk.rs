use crate::protocol::error::PResult;
use crate::protocol::varint::{write_vari32, write_varu32};
use crate::macros::helpers;

/// Sentinel value meaning "unlimited sub-chunks — all data is inline in the payload."
pub const SUB_CHUNK_COUNT_LIMITLESS: u32 = u32::MAX;

pub struct LevelChunk {
    pub chunk_x: i32,
    pub chunk_z: i32,
    /// Number of sub-chunks in the payload.
    /// Use `SUB_CHUNK_COUNT_LIMITLESS` (u32::MAX) when full inline data is present.
    pub sub_chunk_count: u32,
    /// Raw serialized chunk data (sub-chunks + biomes + border blocks).
    pub payload: Vec<u8>,
}

impl LevelChunk {
    pub fn write(&self) -> PResult<Vec<u8>> {
        let mut buf = Vec::new();
        write_vari32(&mut buf, self.chunk_x);   // chunk_x (ZigZag VarI32)
        write_vari32(&mut buf, self.chunk_z);   // chunk_z (ZigZag VarI32)
        write_vari32(&mut buf, 0);              // dimension_id = 0 (Overworld, ZigZag VarI32)
        write_varu32(&mut buf, self.sub_chunk_count); // sub_chunk_count (VarU32)
        buf.push(0);                            // cache_enabled = false (bool byte)
        helpers::write_bytes(&mut buf, &self.payload); // serialized_chunk_data (VarU32 len + raw bytes)
        Ok(buf)
    }
}

/// Build a fully-inline flat chunk payload using sub-chunk format version 9 (Limitless).
///
/// Bedrock overworld covers Y=-64 to Y=320, divided into 24 sub-chunks (index -4..=19).
/// Each sub-chunk is 16×16×16 blocks.
///
/// This generates a completely all-air world (no blocks).
/// Safe for use when `StartGame` sends an empty block palette (count=0)
/// since no non-air block runtime IDs are referenced.
///
/// # Sub-chunk v9 (Limitless) wire layout per sub-chunk:
/// ```
/// [u8: version=9]
/// [u8: layer_count]
/// [i8: sub_chunk_y index]   ← new in v9
/// for each layer:
///   [u8: storage_header = (bits_per_entry << 1) | is_network_palette]
///   if bits_per_entry == 0:
///     [VarI32: single palette entry id]  ← no block array, just one id
///   else:
///     [word_count × u32 LE: block indices packed at bits_per_entry each]
///     [VarI32: palette_count]
///     [palette_count × VarI32: runtime block ids]
/// ```
///
/// # Biomes (one section per sub-chunk):
/// ```
/// [u8: storage_header = (0 << 1) | 1 = 0x01]  ← bits=0 means single-value
/// [VarI32: biome runtime id]                   ← e.g. 1 = plains
/// ```
///
/// # Footer:
/// ```
/// [u8: border_block_count = 0]
/// ```
pub fn make_flat_chunk_payload() -> Vec<u8> {
    let mut data = Vec::new();

    // ── 24 all-air sub-chunks (v9 Limitless, Y indices -4..=19) ─────────────
    for sub_y in -4i8..=19i8 {
        data.push(9);                   // sub-chunk format version = 9 (Limitless)
        data.push(1);                   // 1 storage layer
        data.push(sub_y as u8);         // sub_chunk_y index
        // Single-value storage: bits_per_entry=0, network palette flag=1
        data.push((0u8 << 1) | 1);      // storage_header = 0x01
        write_vari32(&mut data, 0);     // palette entry 0 = air (ZigZag(0) = 0x00)
    }

    // ── Biomes: one single-value section per sub-chunk (24 total) ───────────
    for _ in 0..24 {
        data.push((0u8 << 1) | 1);      // storage_header = 0x01 (bits=0, network palette)
        write_vari32(&mut data, 1);     // biome runtime id = 1 (plains), ZigZag(1) = 0x02
    }

    // ── Footer: border block count = 0 ──────────────────────────────────────
    data.push(0);

    data
}
