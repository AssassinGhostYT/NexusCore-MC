pub mod air;
pub mod bedrock;
pub mod dirt;
pub mod grass;
pub mod cube;
pub mod customblock;
pub mod model;
pub mod action;
pub mod nbt;
pub mod registry;

pub use air::Air;
pub use bedrock::Bedrock;
pub use dirt::Dirt;
pub use grass::Grass;
pub use model::Model;
pub use action::BlockAction;

pub trait Block: Send + Sync + 'static {
    fn name(&self) -> &'static str;
    fn hardness(&self) -> f32;
    fn blast_resistance(&self) -> f32;
    fn sound_type(&self) -> &'static str;
    fn encode_block(&self) -> (String, std::collections::HashMap<String, nbt::NbtTag>);
    
    // Retorna el TypeId concreto de la estructura que implementa el bloque
    fn type_id(&self) -> std::any::TypeId {
        std::any::TypeId::of::<Self>()
    }

    // Generación del hash totalmente automática
    fn hash(&self) -> (u64, u64) {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        // 1. Hash base basado en el TypeId (único por cada struct/tipo de bloque)
        let mut base_hasher = DefaultHasher::new();
        self.type_id().hash(&mut base_hasher);
        let base_hash = base_hasher.finish();

        // 2. Hash de estado basado en sus propiedades NBT ordenadas
        let (_, states) = self.encode_block();
        let mut props: Vec<(String, nbt::NbtTag)> = states.into_iter().collect();
        props.sort_by(|a, b| a.0.cmp(&b.0));

        let mut state_hasher = DefaultHasher::new();
        props.hash(&mut state_hasher);
        let state_hash = state_hasher.finish();

        (base_hash, state_hash)
    }

    fn runtime_id(&self) -> u32 {
        let (name, states) = self.encode_block();
        registry::get_runtime_id(&name, &states).unwrap_or(0)
    }
    fn is_transparent(&self) -> bool;
    fn map_color(&self) -> u8;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlockType {
    Air,
    Bedrock { infiniburn: bool },
    Dirt { coarse: bool },
    Grass,
}

impl BlockType {
    pub fn name(&self) -> &'static str {
        match self {
            BlockType::Air => Air::NAME,
            BlockType::Bedrock { .. } => Bedrock::NAME,
            BlockType::Dirt { coarse } => {
                if *coarse {
                    Dirt::NAME_COARSE
                } else {
                    Dirt::NAME_DIRT
                }
            }
            BlockType::Grass => Grass::NAME,
        }
    }

    pub fn numeric_id(&self) -> i16 {
        match self {
            BlockType::Air => 413,
            BlockType::Bedrock { .. } => 7,
            BlockType::Dirt { coarse } => {
                if *coarse {
                    243 // coarse_dirt numeric ID
                } else {
                    3 // dirt numeric ID
                }
            }
            BlockType::Grass => 2, // grass_block numeric ID
        }
    }

    pub fn runtime_id(&self) -> u32 {
        match self {
            BlockType::Air => Air.runtime_id(),
            BlockType::Bedrock { infiniburn } => {
                Bedrock { infiniburn: *infiniburn }.runtime_id()
            }
            BlockType::Dirt { coarse } => {
                Dirt { coarse: *coarse }.runtime_id()
            }
            BlockType::Grass => Grass.runtime_id(),
        }
    }

    pub fn hardness(&self) -> f32 {
        match self {
            BlockType::Air => Air::HARDNESS,
            BlockType::Bedrock { .. } => Bedrock::HARDNESS,
            BlockType::Dirt { .. } => Dirt::HARDNESS,
            BlockType::Grass => Grass::HARDNESS,
        }
    }

    pub fn blast_resistance(&self) -> f32 {
        match self {
            BlockType::Air => Air::BLAST_RESISTANCE,
            BlockType::Bedrock { .. } => Bedrock::BLAST_RESISTANCE,
            BlockType::Dirt { .. } => Dirt::BLAST_RESISTANCE,
            BlockType::Grass => Grass::BLAST_RESISTANCE,
        }
    }

    pub fn sound_type(&self) -> &'static str {
        match self {
            BlockType::Air => Air::SOUND_TYPE,
            BlockType::Bedrock { .. } => Bedrock::SOUND_TYPE,
            BlockType::Dirt { .. } => Dirt::SOUND_TYPE,
            BlockType::Grass => Grass::SOUND_TYPE,
        }
    }

    pub fn translation_key(&self) -> &'static str {
        match self {
            BlockType::Air => "tile.air.name",
            BlockType::Bedrock { .. } => "tile.bedrock.name",
            BlockType::Dirt { coarse } => {
                if *coarse {
                    "tile.coarse_dirt.name"
                } else {
                    "tile.dirt.name"
                }
            }
            BlockType::Grass => "tile.grass.name",
        }
    }

    pub fn transparent(&self) -> bool {
        match self {
            BlockType::Air => Air::TRANSPARENT,
            BlockType::Bedrock { .. } => Bedrock::TRANSPARENT,
            BlockType::Dirt { .. } => Dirt::TRANSPARENT,
            BlockType::Grass => Grass::TRANSPARENT,
        }
    }

    pub fn map_color(&self) -> u8 {
        match self {
            BlockType::Air => Air::MAP_COLOR,
            BlockType::Bedrock { .. } => Bedrock::MAP_COLOR,
            BlockType::Dirt { .. } => Dirt::MAP_COLOR,
            BlockType::Grass => Grass::MAP_COLOR,
        }
    }
}
