use crate::block::Block;

pub struct Air;

impl Air {
    pub const NAME: &'static str = "minecraft:air";
    pub const NUMERIC_ID: i16 = 413;
    pub const TRANSLATION_KEY: &'static str = "tile.air.name";
    pub const HARDNESS: f32 = 0.0;
    pub const BLAST_RESISTANCE: f32 = 0.0;
    pub const SOUND_TYPE: &'static str = "none";
    pub const RUNTIME_ID: u32 = 12530;
    pub const TRANSPARENT: bool = true;
    pub const MAP_COLOR: u8 = 0; // 0 NONE
}

impl Block for Air {
    fn name(&self) -> &'static str {
        Self::NAME
    }

    fn numeric_id(&self) -> i16 {
        Self::NUMERIC_ID
    }

    fn runtime_id(&self) -> u32 {
        Self::RUNTIME_ID
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

    fn translation_key(&self) -> &'static str {
        Self::TRANSLATION_KEY
    }

    fn transparent(&self) -> bool {
        Self::TRANSPARENT
    }

    fn map_color(&self) -> u8 {
        Self::MAP_COLOR
    }
}
