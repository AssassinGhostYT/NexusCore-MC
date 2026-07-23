use crate::protocol::error::PResult;
use crate::protocol::varint::{write_vari32, write_varu32};
use crate::macros::helpers;

pub struct LevelChunk {
    pub chunk_x: i32,
    pub chunk_z: i32,
    /// sub_chunk_count tells the client how many block sub-chunks follow in the payload.
    /// 0 = no sub-chunks (all air — client fills with air automatically).
    pub sub_chunk_count: u32,
    pub payload: Vec<u8>,
}

impl LevelChunk {
    pub fn write(&self) -> PResult<Vec<u8>> {
        let mut buf = Vec::new();
        write_vari32(&mut buf, self.chunk_x);
        write_vari32(&mut buf, self.chunk_z);
        write_vari32(&mut buf, 0); // dimension_id = 0 (overworld)
        write_varu32(&mut buf, self.sub_chunk_count);
        buf.push(0); // cache_enabled = false
        helpers::write_bytes(&mut buf, &self.payload);
        Ok(buf)
    }
}

/// Generate flat chunk data (bedrock sub-chunk format version 8).
/// Constructs 24 sub-chunks (y=-4..19 for world height -64..320) plus biomes.
pub fn make_flat_chunk_payload() -> Vec<u8> {
    let mut data = Vec::new();

    // ── 24 Subchunks (v8 format) ─────────────────────────────────────────
    for sub_y in -4..20i32 {
        data.push(8); // subchunk format version = 8
        data.push(1); // 1 storage layer

        if sub_y == 4 { // Subchunk 4 covers y=64..79 (bedrock platform at y=64)
            // bits_per_block = 1 (bits_per_block << 1 | 1 for network palette)
            data.push((1 << 1) | 1);

            // 4096 blocks packed at 1 bit per block:
            // Block 0 at (x=0,y=0,z=0 within subchunk, which is y=64 absolute) = Bedrock (runtime_id = 1)
            // All other 4095 blocks = Air (runtime_id = 0)
            // Packing: 4096 bits = 128 u32 words = 512 bytes.
            // First u32 word has bit 0 set (1), remaining 127 words are 0.
            data.extend_from_slice(&1u32.to_le_bytes());
            for _ in 0..127 {
                data.extend_from_slice(&0u32.to_le_bytes());
            }

            // Palette count = 2 (vari32 zigzag: 2 = 4)
            write_vari32(&mut data, 2);
            write_vari32(&mut data, 0); // Palette 0: Air (0)
            write_vari32(&mut data, 1); // Palette 1: Bedrock (1)
        } else {
            // All Air subchunk: bits_per_block = 0
            data.push((0 << 1) | 1);
            // Palette 0: Air (vari32 zigzag: 0 = 0)
            write_vari32(&mut data, 0);
        }
    }

    // ── Biomes: 24 sections (overworld -4..19 inclusive) ──────────────────
    for _ in 0..24 {
        data.push(0); // biome storage header: bits=0, runtime=0
        write_vari32(&mut data, 1); // plains biome ID = 1
    }

    // ── Border blocks count = 0 ──────────────────────────────────────────
    data.push(0);

    data
}
