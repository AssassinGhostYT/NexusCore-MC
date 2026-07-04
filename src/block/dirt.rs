use crate::block::Block;
use crate::block::nbt::NbtTag;
use std::collections::HashMap;

pub struct Dirt {
    pub coarse: bool,
}

impl Dirt {
    pub const NAME_DIRT: &'static str = "minecraft:dirt";
    pub const NAME_COARSE: &'static str = "minecraft:coarse_dirt";
    pub const HARDNESS: f32 = 0.5;
    pub const BLAST_RESISTANCE: f32 = 0.5;
    pub const SOUND_TYPE: &'static str = "gravel";
    pub const TRANSPARENT: bool = false;
    pub const MAP_COLOR: u8 = 2; // 2 CLAY/DIRT
}

impl Block for Dirt {
    fn name(&self) -> &'static str {
        if self.coarse {
            Self::NAME_COARSE
        } else {
            Self::NAME_DIRT
        }
    }

    fn hardness(&self) -> f32 {
        Self::HARDNESS
    }

    fn blast_resistance(&self) -> f32 {
        Self::BLAST_RESISTANCE
    }

    fn sound_type(&self) -> &'static str {
        Self::SOUND_TYPE
    }

    fn encode_block(&self) -> (String, HashMap<String, NbtTag>) {
        (self.name().to_string(), HashMap::new())
    }

    fn is_transparent(&self) -> bool {
        Self::TRANSPARENT
    }

    fn map_color(&self) -> u8 {
        Self::MAP_COLOR
    }
}
