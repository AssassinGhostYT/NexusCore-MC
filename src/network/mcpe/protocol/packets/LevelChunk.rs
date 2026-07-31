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

/// Build an all-air chunk payload following Chorus/bedrock-rs network encoding exactly.
///
/// Reference: /root/Chorus/src/level/sub_chunk.rs  (Palette::serialize + SubChunk::serialize_network)
///            /root/Chorus/src/level/chunk.rs       (Chunk::serialize)
///
/// Per sub-chunk (version 9 Limitless, 2 block layers):
///   [u8  version  = 9]
///   [u8  layers   = 2]          ← TWO layers: block + waterlog layer
///   [u8  sub_y    ]             ← sub-chunk Y index (i8 as u8)
///   -- Layer 1 (blocks) --
///   [u8  header   = 0x01]       ← (bits=0 << 1) | 1 = single-value, network palette
///   [VarI32 air_id = 0]         ← ZigZag(0) = 0x00  (no count when bits=0)
///   -- Layer 2 (waterlog/liquid) --
///   [u8  header   = 0x01]
///   [VarI32 air_id = 0]         ← 0x00
///
/// Biomes (one entry per sub-chunk, written AFTER all sub-chunks):
///   [u8  header   = 0x01]       ← single-value, network palette
///   [VarI32 biome_id = 1]       ← ZigZag(1) = 0x02  (plains)
///
/// Footer:
///   [u8  = 0]                   ← border_block_count
pub fn make_flat_chunk_payload() -> Vec<u8> {
    let mut data = Vec::new();

    // ── 24 all-air sub-chunks (Y indices -4..=19) ────────────────────────────
    // Chorus uses 2 block layers per sub-chunk.
    for sub_y in -4i8..=19i8 {
        data.push(9);               // format version = 9 (Limitless)
        data.push(2);               // 2 storage layers (block + waterlog)
        data.push(sub_y as u8);     // sub_chunk_y index

        // Layer 1: blocks (all air, runtime_id = 0)
        data.push(0x01);            // header: (0 bits << 1) | 1 = 0x01
        write_vari32(&mut data, 0); // ZigZag(0) = 0x00; no count when bits=0

        // Layer 2: waterlog (all air)
        data.push(0x01);            // header
        write_vari32(&mut data, 0); // ZigZag(0) = 0x00
    }

    // ── Biomes: one PalettedStorage per sub-chunk ─────────────────────────────
    // Written AFTER all sub-chunks, one entry per sub-chunk (24 total).
    // Biome IDs use VarI32 (ZigZag), same as block IDs.
    // Plains biome = 1 → ZigZag(1) = 2 = 0x02.
    for _ in 0..24 {
        data.push(0x01);            // header: single-value, network palette
        write_vari32(&mut data, 1); // biome_id = 1 (plains); ZigZag(1) = 0x02
    }

    // ── Footer ───────────────────────────────────────────────────────────────
    data.push(0); // border_block_count = 0

    data
}
