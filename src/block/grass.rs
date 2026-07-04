use crate::block::Block;
use crate::block::nbt::NbtTag;
use std::collections::HashMap;

pub struct Grass;

impl Grass {
    pub const NAME: &'static str = "minecraft:grass_block";
    pub const HARDNESS: f32 = 0.6;
    pub const BLAST_RESISTANCE: f32 = 0.6;
    pub const SOUND_TYPE: &'static str = "grass";
    pub const TRANSPARENT: bool = false;
    pub const MAP_COLOR: u8 = 1;
}

impl Block for Grass {
    fn name(&self) -> &'static str {
        Self::NAME
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
        (Self::NAME.to_string(), HashMap::new())
    }

    fn is_transparent(&self) -> bool {
        Self::TRANSPARENT
    }

    fn map_color(&self) -> u8 {
        Self::MAP_COLOR
    }
}
