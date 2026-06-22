use byteorder::{LittleEndian, WriteBytesExt};
use crate::protocol::varint::{write_varu32, write_vari32};
use super::helpers::write_string;
use serde::{Deserialize, Serialize};

pub const ID_ITEM_REGISTRY: u32 = 162;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemEntry {
    pub name: String,
    pub runtime_id: i16,
}

#[derive(Debug, Clone)]
pub struct ItemRegistry {
    pub items: Vec<ItemEntry>,
}

impl ItemRegistry {
    pub fn new() -> Self {
        let json_str = include_str!("../../../items.json");
        let items: Vec<ItemEntry> = serde_json::from_str(json_str).unwrap_or_else(|e| {
            log::error!("Failed to parse items.json: {:?}", e);
            Vec::new()
        });
        ItemRegistry { items }
    }

    pub fn write(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        write_varu32(&mut buf, self.items.len() as u32);
        for item in &self.items {
            write_string(&mut buf, &item.name);
            buf.write_i16::<LittleEndian>(item.runtime_id).unwrap();
            buf.push(0); // ComponentBased
            write_vari32(&mut buf, 0); // Version
            buf.extend_from_slice(&[0x0a, 0x00, 0x00]); // Empty NBT Compound
        }
        buf
    }
}
