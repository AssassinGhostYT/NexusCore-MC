use crate::protocol::error::PResult;
use crate::protocol::varint::{write_varu32, write_vari32};
use crate::macros::helpers;

/// SubChunkRequestModeLimited: client sends SubChunkRequest packets.
/// With HighestSubChunk=0, client knows all sub-chunks are air and doesn't request them.
/// This is what Dragonfly uses for Bedrock 1.18+.
pub const SUB_CHUNK_REQUEST_MODE_LIMITED: u32 = u32::MAX - 1;   // 0xFFFFFFFE

pub struct LevelChunk {
    pub chunk_x: i32,
    pub chunk_z: i32,
    /// Set to SUB_CHUNK_REQUEST_MODE_LIMITED for the LIMITED mode (Dragonfly default for 1.18+).
    /// Set to actual count (e.g. 24) for the legacy inline mode.
    pub sub_chunk_count: u32,
    /// Only used when sub_chunk_count == SUB_CHUNK_REQUEST_MODE_LIMITED.
    /// HighestSubChunk=0 means no filled sub-chunks (all air).
    pub highest_sub_chunk: u16,
    /// Raw serialized chunk data.
    /// In LIMITED mode: biomes + border block byte (26 bytes for all-plains).
    /// In legacy mode:  sub-chunks + biomes + border block bytes.
    pub payload: Vec<u8>,
}

impl LevelChunk {
    pub fn write(&self) -> PResult<Vec<u8>> {
        let mut buf = Vec::new();
        write_vari32(&mut buf, self.chunk_x);         // chunk_x  (ZigZag VarI32)
        write_vari32(&mut buf, self.chunk_z);         // chunk_z  (ZigZag VarI32)
        write_vari32(&mut buf, 0);                    // dimension_id = 0 (Overworld)
        write_varu32(&mut buf, self.sub_chunk_count); // sub_chunk_count (VarU32)

        // Only in LIMITED mode: write HighestSubChunk as u16 LE
        if self.sub_chunk_count == SUB_CHUNK_REQUEST_MODE_LIMITED {
            buf.extend_from_slice(&self.highest_sub_chunk.to_le_bytes());
        }

        buf.push(0);                                  // cache_enabled = false
        helpers::write_bytes(&mut buf, &self.payload);// serialized_chunk_data (VarU32 len + bytes)
        Ok(buf)
    }
}

/// Build a LIMITED-mode chunk payload (biomes only, no inline sub-chunks).
///
/// Reference: Dragonfly server/session/chunk.go sendNetworkChunk (subChunkRequests=true):
///   s.writePacket(&packet.LevelChunk{
///       SubChunkCount:   protocol.SubChunkRequestModeLimited,
///       HighestSubChunk: c.HighestFilledSubChunk(),  // 0 for all-air
///       RawPayload:      append(chunk.EncodeBiomes(c, chunk.NetworkEncoding), 0),
///   })
///
/// Biome wire layout (Dragonfly networkEncoding, 24 sections):
///   First section:
///     [u8 header = 0x01]       ← (bits=0 << 1) | network=1 = single-value palette
///     [VarI32 biome_id = 1]    ← ZigZag(1) = 2 = 0x02  (plains)
///   Sections 2-24 (same as previous):
///     [u8 = 0xFF]              ← (0x7F << 1) | 1 = "same as previous" marker
///   Footer:
///     [u8 = 0]                 ← border_block_count
///
/// Total: 2 + 23 + 1 = 26 bytes.
pub fn make_limited_chunk_payload() -> Vec<u8> {
    let mut data = Vec::new();

    // First biome section: plains (biome_id = 1)
    // ZigZag(1) = 2 → 0x02 (1 byte VarI32)
    data.push(0x01);            // header: (0 bits << 1) | network=1
    write_vari32(&mut data, 1); // ZigZag(1) = 0x02

    // Sections 2-24: "same as previous" = (0x7F << 1) | 1 = 0xFF
    for _ in 1..24 {
        data.push(0xFF);
    }

    // Border block count
    data.push(0);

    data
}
