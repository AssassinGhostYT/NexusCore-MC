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
/// This generates a completely flat world with a single bedrock layer at Y=0
/// (sub-chunk index 0, local Y=0).
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

    // ── 24 Sub-chunks (v9 Limitless format, Y indices -4..=19) ──────────────
    for sub_y in -4i8..=19i8 {
        data.push(9);               // sub-chunk format version = 9 (Limitless)
        data.push(1);               // 1 storage layer
        data.push(sub_y as u8);     // sub_chunk_y index (i8 cast to u8)

        if sub_y == 0 {
            // Sub-chunk 0 (absolute Y=0..15): place bedrock at local Y=0.
            // Use 1 bit per block (palette has 2 entries: air=0, bedrock=1).
            // Storage header: (1 << 1) | 1 = 3  (bits=1, network palette)
            data.push((1u8 << 1) | 1);

            // Block array: 4096 blocks packed at 1 bit each = 128 × u32 words.
            // Block index layout: index = (x << 8) | (z << 4) | y
            // Block at (0,0,0) = index 0 → word 0, bit 0 → palette entry 1 (bedrock)
            // All other blocks = air (palette entry 0)
            data.extend_from_slice(&1u32.to_le_bytes()); // word 0: bit 0 set
            for _ in 0..127 {
                data.extend_from_slice(&0u32.to_le_bytes()); // words 1-127: all air
            }

            // Palette: 2 entries
            write_vari32(&mut data, 2);  // palette_count = 2
            write_vari32(&mut data, 0);  // palette[0] = air (runtime_id 0)
            write_vari32(&mut data, 1);  // palette[1] = bedrock (runtime_id 1)
        } else {
            // All-air sub-chunk: bits_per_entry = 0 → single-value storage.
            // Storage header: (0 << 1) | 1 = 1  (bits=0, network palette)
            data.push((0u8 << 1) | 1);
            // No block array needed when bits=0 (single-value).
            // Just the single palette entry:
            write_vari32(&mut data, 0);  // palette[0] = air (runtime_id 0)
        }
    }

    // ── Biomes: one section per sub-chunk (24 total) ─────────────────────────
    // Single-value biome storage (bits=0, network palette).
    for _ in 0..24 {
        data.push((0u8 << 1) | 1);  // storage_header: bits=0, network palette
        write_vari32(&mut data, 1);  // biome runtime id 1 = plains
    }

    // ── Footer: border block count = 0 ──────────────────────────────────────
    data.push(0);

    data
}
