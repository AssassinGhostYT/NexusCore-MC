use crate::block::Block;

pub struct Bedrock {
    pub infiniburn: bool,
}

impl Bedrock {
    pub const NAME: &'static str = "minecraft:bedrock";
    pub const NUMERIC_ID: i16 = 7;
    pub const TRANSLATION_KEY: &'static str = "tile.bedrock.name";
    pub const HARDNESS: f32 = -1.0;
    pub const BLAST_RESISTANCE: f32 = 3600000.0;
    pub const SOUND_TYPE: &'static str = "stone";
    pub const RUNTIME_ID_DEFAULT: u32 = 13079;
    pub const RUNTIME_ID_INFINIBURN: u32 = 13080;
    pub const TRANSPARENT: bool = false;
    pub const MAP_COLOR: u8 = 11; // 11 STONE
}

impl Block for Bedrock {
    fn name(&self) -> &'static str {
        Self::NAME
    }

    fn numeric_id(&self) -> i16 {
        Self::NUMERIC_ID
    }

    fn runtime_id(&self) -> u32 {
        if self.infiniburn {
            Self::RUNTIME_ID_INFINIBURN
        } else {
            Self::RUNTIME_ID_DEFAULT
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
