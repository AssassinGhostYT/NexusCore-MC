use crate::protocol::error::PResult;
use crate::protocol::varint::{write_vari32, write_varu32};
use crate::macros::helpers;
use byteorder::{LittleEndian, WriteBytesExt};

pub struct LevelChunk {
    pub chunk_x: i32,
    pub chunk_z: i32,
    pub sub_chunk_count: u32,
    pub payload: Vec<u8>,
}

impl LevelChunk {
    pub fn write(&self) -> PResult<Vec<u8>> {
        let mut buf = Vec::new();
        write_vari32(&mut buf, self.chunk_x);
        write_vari32(&mut buf, self.chunk_z);
        write_vari32(&mut buf, 0); // dimension_id = 0
        write_varu32(&mut buf, self.sub_chunk_count);
        buf.push(0); // cache_enabled = false
        helpers::write_bytes(&mut buf, &self.payload);
        Ok(buf)
    }
}

/// Generate flat world chunk data: Bedrock/Dirt/Grass at y=-64 (subchunk index 0)
/// and air for the rest of the 23 subchunks.
pub fn make_flat_chunk_payload() -> Vec<u8> {
    let mut data = Vec::new();
    
    let air_id: i32 = -604749536;
    let bedrock_id: i32 = -173245189;
    let dirt_id: i32 = -2108756090;
    let grass_id: i32 = -567203660;

    log::info!("Chunk runtime hashes: air={}, bedrock={}, dirt={}, grass={}", 
               air_id, bedrock_id, dirt_id, grass_id);
    
    // --- Sub-chunk 0 (y = -4, index 0): contains ground ---
    data.push(9); // version = 9 (Limitless)
    data.push(2); // storage_count = 2 layers
    data.push(252); // sub_chunk_y = -4 (0xfc)
    
    // Layer 0: Bedrock + Dirt + Grass + Air
    data.push(5); // bits_per_block = 2, flag = 1 (5)
    
    let mut words = vec![0u32; 256];
    for x in 0..16 {
        for z in 0..16 {
            for y in 0..16 {
                let index = ((x as usize) << 8) | ((z as usize) << 4) | (y as usize);
                let palette_idx = if y == 0 {
                    0 // Bedrock
                } else if y == 1 || y == 2 {
                    1 // Dirt
                } else if y == 3 {
                    2 // Grass
                } else {
                    3 // Air
                };
                
                let word_idx = index / 16;
                let bit_offset = (index % 16) * 2;
                words[word_idx] |= (palette_idx as u32) << bit_offset;
            }
        }
    }
    
    for w in words {
        data.write_u32::<LittleEndian>(w).unwrap();
    }
    
    write_vari32(&mut data, 4); // palette count = 4
    write_vari32(&mut data, bedrock_id as i32);
    write_vari32(&mut data, dirt_id as i32);
    write_vari32(&mut data, grass_id as i32);
    write_vari32(&mut data, air_id as i32);
    
    // Layer 1: all air (single valued palette)
    data.push(1);
    write_vari32(&mut data, air_id as i32);
    
    // --- Sub-chunks 1..23 (y = -3..19): all air ---
    for y_idx in 1..24 {
        let y_val = (y_idx as i8 - 4) as u8;
        data.push(9);
        data.push(2);
        data.push(y_val);
        
        // Layer 0: all air
        data.push(1);
        write_vari32(&mut data, air_id as i32);
        
        // Layer 1: all air
        data.push(1);
        write_vari32(&mut data, air_id as i32);
    }
    
    // --- Biomes for all 24 subchunks ---
    for _ in 0..24 {
        // Biome: all Plains (ID 0, single valued palette)
        data.push(1);
        write_vari32(&mut data, 0); // Plains biome (0)
    }
    
    // Border blocks count (0)
    data.push(0);
    
    data
}
