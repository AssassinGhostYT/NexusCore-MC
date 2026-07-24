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
        write_vari32(&mut buf, self.chunk_x);        // chunk_x  (ZigZag VarI32)
        write_vari32(&mut buf, self.chunk_z);        // chunk_z  (ZigZag VarI32)
        write_vari32(&mut buf, 0);                   // dimension_id = 0 (Overworld)
        write_varu32(&mut buf, self.sub_chunk_count);// sub_chunk_count (plain VarU32)
        buf.push(0);                                 // cache_enabled = false
        helpers::write_bytes(&mut buf, &self.payload);// serialized_chunk_data (VarU32 len + bytes)
        Ok(buf)
    }
}

/// Build an all-air chunk payload (sub-chunk format version 9, Limitless).
///
/// Wire layout per sub-chunk (v9):
///   [u8  version  = 9]
///   [u8  layers   = 1]
///   [u8  sub_y    ]          ← sub-chunk Y index (i8 cast to u8)
///   [u8  storage_header = 0x01]  ← bits_per_entry=0 (single-value), network palette
///   [VarI32 block_runtime_id = 0]  ← air; ZigZag(0) = 0x00
///
/// Wire layout per biome section:
///   [u8  storage_header = 0x01]  ← bits_per_entry=0 (single-value), network palette
///   [VarU32 biome_id    = 1  ]   ← plains; plain VarU32 (NOT zigzag); 1 = 0x01
///
/// Footer:
///   [u8  border_block_count = 0]
pub fn make_flat_chunk_payload() -> Vec<u8> {
    let mut data = Vec::new();

    // ── 24 all-air sub-chunks (Y indices -4..=19) ────────────────────────────
    for sub_y in -4i8..=19i8 {
        data.push(9);                    // format version = 9 (Limitless)
        data.push(1);                    // 1 storage layer
        data.push(sub_y as u8);          // sub_chunk_y index
        data.push(0x01);                 // storage_header: bits=0, network palette
        write_vari32(&mut data, 0);      // block runtime id = 0 (air); ZigZag(0) = 0x00
    }

    // ── Biomes: 24 single-value sections ─────────────────────────────────────
    // Biome palette entries use plain VarU32, NOT ZigZag VarI32.
    for _ in 0..24 {
        data.push(0x01);                 // storage_header: bits=0, network palette
        write_varu32(&mut data, 1);      // biome id = 1 (plains); plain VarU32: 0x01
    }

    // ── Footer ───────────────────────────────────────────────────────────────
    data.push(0); // border_block_count = 0

    data
}
