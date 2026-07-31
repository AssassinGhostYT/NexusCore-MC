use crate::protocol::error::PResult;
use crate::protocol::varint::{write_varu32, write_vari32};
use crate::macros::helpers;

pub struct LevelChunk {
    pub chunk_x: i32,
    pub chunk_z: i32,
    /// Number of sub-chunks in the payload (use the real count, e.g. 24).
    pub sub_chunk_count: u32,
    /// Raw serialized chunk data (sub-chunks + biomes + border blocks).
    pub payload: Vec<u8>,
}

impl LevelChunk {
    pub fn write(&self) -> PResult<Vec<u8>> {
        let mut buf = Vec::new();
        write_vari32(&mut buf, self.chunk_x);         // chunk_x  (ZigZag VarI32)
        write_vari32(&mut buf, self.chunk_z);         // chunk_z  (ZigZag VarI32)
        write_vari32(&mut buf, 0);                    // dimension_id = 0 (Overworld)
        write_varu32(&mut buf, self.sub_chunk_count); // sub_chunk_count (plain VarU32)
        buf.push(0);                                  // cache_enabled = false
        helpers::write_bytes(&mut buf, &self.payload);// serialized_chunk_data (VarU32 len + bytes)
        Ok(buf)
    }
}

/// Build an all-air chunk payload following the Dragonfly/Bedrock network encoding exactly.
///
/// Sub-chunk wire layout (version 9, Limitless):
///   [u8  version  = 9]
///   [u8  layers   = 1]
///   [u8  sub_y    ]          ← sub-chunk Y index (i8 cast to u8)
///   [u8  storage_header = 0x01]  ← (bits_per_index=0 << 1) | network=1
///   [VarI32 block_runtime_id = 0]  ← air; no count prefix when bits=0; ZigZag(0) = 0x00
///
/// Biome wire layout (24 sections):
///   First section:
///     [u8  storage_header = 0x01]  ← (bits=0 << 1) | network=1
///     [u32 LE biome_id    = 1  ]   ← plains; 4 bytes little-endian (NOT varint!)
///                                    no count prefix when bits=0
///   Sections 2-24:
///     [u8  = 0xFF]  ← "same as previous" marker: (0x7f << 1) | 1 = 0xFF
///
/// Footer:
///   [u8  border_block_count = 0]
pub fn make_flat_chunk_payload() -> Vec<u8> {
    let mut data = Vec::new();

    // ── 24 all-air sub-chunks (Y indices -4..=19) ────────────────────────────
    for sub_y in -4i8..=19i8 {
        data.push(9);               // format version = 9 (Limitless)
        data.push(1);               // 1 storage layer
        data.push(sub_y as u8);     // sub_chunk_y index
        // PalettedStorage header: (bits_per_index << 1) | network_flag
        // For single-value (0 bits): (0 << 1) | 1 = 0x01
        data.push(0x01);
        // Block runtime ID as VarI32 (ZigZag encoded).
        // When bits=0, palette has exactly 1 entry, NO count varint prefix.
        write_vari32(&mut data, 0); // air = runtime ID 0; ZigZag(0) = 0x00
    }

    // ── Biomes: 24 sections ───────────────────────────────────────────────────
    // Biome palette entries use u32 little-endian (NOT varint), per biomePaletteEncoding.
    // When bits=0, no count prefix is written.
    // Sections identical to the previous use the "same as previous" marker: 0xFF = (0x7f<<1)|1.

    // First biome section: plains (ID=1)
    data.push(0x01);                               // storage header: bits=0, network=1
    data.extend_from_slice(&1u32.to_le_bytes());   // biome_id = 1 (plains), u32 LE

    // Sections 2-24: "same as previous" = 0xFF
    for _ in 1..24 {
        data.push(0xFF); // (0x7f << 1) | 1
    }

    // ── Footer ───────────────────────────────────────────────────────────────
    data.push(0); // border_block_count = 0

    data
}
