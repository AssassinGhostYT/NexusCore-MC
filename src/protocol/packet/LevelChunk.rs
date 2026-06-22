use byteorder::{LittleEndian, WriteBytesExt};
use crate::protocol::varint::{write_varu32, write_vari32};
use crate::block::{Air, Bedrock};

pub const ID_LEVEL_CHUNK: u32 = 58;

#[derive(Debug, Clone)]
pub struct LevelChunk {
    pub chunk_x: i32,
    pub chunk_z: i32,
    pub sub_chunk_count: u32,
    pub payload: Vec<u8>,
}

impl LevelChunk {
    pub fn write(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        write_vari32(&mut buf, self.chunk_x);
        write_vari32(&mut buf, self.chunk_z);
        write_vari32(&mut buf, 0); // Dimension: Overworld (0)
        write_varu32(&mut buf, self.sub_chunk_count);
        buf.push(0); // Cache enabled: false
        
        write_varu32(&mut buf, self.payload.len() as u32);
        buf.extend_from_slice(&self.payload);
        
        buf
    }
}

pub fn pack_block_storage(block_indices: &[u8; 4096], bits_per_block: u8, palette: &[u32]) -> Vec<u8> {
    let mut buf = Vec::new();
    
    // Storage version: (bits_per_block << 1) | 1 (1 indicates network serialization)
    let version_byte = (bits_per_block << 1) | 1;
    buf.push(version_byte);
    
    let blocks_per_word = 32 / bits_per_block as usize;
    let words_count = (4096 + blocks_per_word - 1) / blocks_per_word;
    
    // Write packed words
    for w in 0..words_count {
        let mut word: u32 = 0;
        for b in 0..blocks_per_word {
            let idx = w * blocks_per_word + b;
            if idx < 4096 {
                let palette_idx = block_indices[idx] as u32;
                word |= (palette_idx & ((1 << bits_per_block) - 1)) << (b * bits_per_block as usize);
            }
        }
        buf.write_u32::<LittleEndian>(word).unwrap();
    }
    
    // Write palette size: i32 VarInt
    write_vari32(&mut buf, palette.len() as i32);
    
    // Write palette elements: i32 VarInt runtime ID for each block
    for &runtime_id in palette {
        write_vari32(&mut buf, runtime_id as i32);
    }
    
    buf
}

pub fn pack_single_block_storage(runtime_id: u32) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.push(1); // Palette header: bits_per_block = 0 (1)
    write_vari32(&mut buf, runtime_id as i32); // Runtime ID (zigzag VarInt)
    buf
}

pub fn make_flat_chunk_payload() -> Vec<u8> {
    let mut payload = Vec::new();
    
    // Subchunk 0 (Y = -64..-49) contains Bedrock at the base, and the rest is Air
    // Local Y = 0 (Y = -64): Bedrock
    // Local Y = 1..=15 (Y = -63..-49): Air
    let mut sub0_indices = [0u8; 4096];
    for x in 0..16 {
        for z in 0..16 {
            for y in 0..16 {
                let idx = (x << 8) | (z << 4) | y;
                if y == 0 {
                    sub0_indices[idx] = 1; // Bedrock -> palette index 1
                } else {
                    sub0_indices[idx] = 0; // Air -> palette index 0
                }
            }
        }
    }
    
    let sub0_palette = vec![
        Air::RUNTIME_ID,
        Bedrock::RUNTIME_ID_DEFAULT,
    ];
    let sub0_storage = pack_block_storage(&sub0_indices, 1, &sub0_palette);
    
    // Write Subchunk 0
    payload.push(9); // SubChunk Version: 9
    payload.push(1); // Storage layers count: 1
    payload.push(252); // Y index: -4
    payload.extend_from_slice(&sub0_storage);
    
    // Subchunks 1..23 (Y = -48..319) are all Air (0 storage layers count means empty, defaults to Air)
    for i in 1..24 {
        payload.push(9); // SubChunk Version: 9
        payload.push(0); // Storage layers count: 0
        payload.push((i as i8 - 4) as u8); // Y index
    }
    
    // 24 biome storages (plains biome ID 1)
    let plains_biome_storage = pack_single_block_storage(1);
    for _ in 0..24 {
        payload.extend_from_slice(&plains_biome_storage);
    }
    
    payload.push(0); // Border blocks count: 0
    payload.push(0); // Block entities count: 0
    
    payload
}
