pub mod air;
pub mod bedrock;

pub use air::Air;
pub use bedrock::Bedrock;

pub trait Block {
    fn name(&self) -> &'static str;
    fn numeric_id(&self) -> i16;
    fn runtime_id(&self) -> u32;
    fn hardness(&self) -> f32;
    fn blast_resistance(&self) -> f32;
    fn sound_type(&self) -> &'static str;
    fn translation_key(&self) -> &'static str;
    fn transparent(&self) -> bool;
    fn map_color(&self) -> u8;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlockType {
    Air,
    Bedrock { infiniburn: bool },
}

impl BlockType {
    pub fn name(&self) -> &'static str {
        match self {
            BlockType::Air => Air::NAME,
            BlockType::Bedrock { .. } => Bedrock::NAME,
        }
    }

    pub fn numeric_id(&self) -> i16 {
        match self {
            BlockType::Air => Air::NUMERIC_ID,
            BlockType::Bedrock { .. } => Bedrock::NUMERIC_ID,
        }
    }

    pub fn runtime_id(&self) -> u32 {
        match self {
            BlockType::Air => Air::RUNTIME_ID,
            BlockType::Bedrock { infiniburn } => {
                if *infiniburn {
                    Bedrock::RUNTIME_ID_INFINIBURN
                } else {
                    Bedrock::RUNTIME_ID_DEFAULT
                }
            }
        }
    }

    pub fn hardness(&self) -> f32 {
        match self {
            BlockType::Air => Air::HARDNESS,
            BlockType::Bedrock { .. } => Bedrock::HARDNESS,
        }
    }

    pub fn blast_resistance(&self) -> f32 {
        match self {
            BlockType::Air => Air::BLAST_RESISTANCE,
            BlockType::Bedrock { .. } => Bedrock::BLAST_RESISTANCE,
        }
    }

    pub fn sound_type(&self) -> &'static str {
        match self {
            BlockType::Air => Air::SOUND_TYPE,
            BlockType::Bedrock { .. } => Bedrock::SOUND_TYPE,
        }
    }

    pub fn translation_key(&self) -> &'static str {
        match self {
            BlockType::Air => Air::TRANSLATION_KEY,
            BlockType::Bedrock { .. } => Bedrock::TRANSLATION_KEY,
        }
    }

    pub fn transparent(&self) -> bool {
        match self {
            BlockType::Air => Air::TRANSPARENT,
            BlockType::Bedrock { .. } => Bedrock::TRANSPARENT,
        }
    }

    pub fn map_color(&self) -> u8 {
        match self {
            BlockType::Air => Air::MAP_COLOR,
            BlockType::Bedrock { .. } => Bedrock::MAP_COLOR,
        }
    }
}
