use crate::block::{Bedrock, Dirt, Grass, Block};

pub struct CreativeItem {
    pub name: &'static str,
    pub network_id: i32,
    pub block_runtime_id: i32,
    pub category: i32, // 1 = Construction, 2 = Nature, etc.
}

pub fn items() -> Vec<CreativeItem> {
    vec![
        // 1. Bedrock -> Construction (Category 1)
        CreativeItem {
            name: Bedrock::NAME,
            network_id: 58,
            block_runtime_id: Bedrock { infiniburn: false }.runtime_id() as i32,
            category: 1,
        },
        // 2. Dirt -> Nature (Category 2)
        CreativeItem {
            name: Dirt::NAME_DIRT,
            network_id: 28,
            block_runtime_id: Dirt { coarse: false }.runtime_id() as i32,
            category: 2,
        },
        // 3. Coarse Dirt -> Nature (Category 2)
        CreativeItem {
            name: Dirt::NAME_COARSE,
            network_id: 29,
            block_runtime_id: Dirt { coarse: true }.runtime_id() as i32,
            category: 2,
        },
        // 4. Grass Block -> Nature (Category 2)
        CreativeItem {
            name: Grass::NAME,
            network_id: 27,
            block_runtime_id: Grass.runtime_id() as i32,
            category: 2,
        },
    ]
}
